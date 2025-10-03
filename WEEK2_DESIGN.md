# Week 2: Channel-Based Pipeline Architecture - Design Document

**Date:** 2025-10-03
**Status:** DRAFT - Awaiting Approval
**Educational Goal:** Demonstrate channel-based concurrency patterns with natural backpressure

---

## Executive Summary

Refactor `pipeline_service.rs` from **mutex-based concurrency** to **channel-based pipeline architecture**.

**Benefits:**
- ✅ Eliminates writer mutex contention
- ✅ Natural backpressure via bounded channels
- ✅ Clear separation of concerns (reader/worker/writer)
- ✅ Better observability (queue depth metrics)
- ✅ Foundation for Week 3 streaming I/O

---

## Current Architecture Problems

### Problem 1: Writer Mutex Contention
```rust
// pipeline_service.rs:636
let writer_shared = Arc<tokio::sync::Mutex<BinaryWriter>>;

// Later... (line 813+)
let mut writer = writer_clone.lock().await;  // ← Contention point!
writer.write_chunk(...)
```

**Issue:** N workers compete for write lock → serialization bottleneck

### Problem 2: No Backpressure
```rust
// All chunks loaded into memory immediately
for (chunk_index, file_chunk) in input_chunks.into_iter().enumerate() {
    tokio::spawn(async move { /* process chunk */ });
}
```

**Issue:** 10GB file with 8MB chunks = 1,280 tasks spawned instantly → memory spike

### Problem 3: Complex Coordination
- Semaphore limits workers (good!)
- But mutex serializes writes anyway (bad!)
- Two different concurrency primitives fighting each other

---

## New Architecture: Three-Stage Pipeline

```
┌─────────────┐          ┌──────────────┐          ┌─────────────┐
│   Reader    │  Chunk   │ CPU Workers  │ Processed│   Writer    │
│   Task      │─────────▶│  (Pool of N) │─────────▶│    Task     │
│  (1 task)   │  Channel │  (N tasks)   │  Channel │  (1 task)   │
└─────────────┘  (MPSC)  └──────────────┘  (MPSC)  └─────────────┘
     │                           │                         │
     │ Reads from disk           │ Compression/Encryption  │ Writes to disk
     │ Sends raw chunks          │ CPU-bound work          │ Sequential writes
     └──────────────────────────────────────────────────────┘
                    Bounded channels create backpressure
```

### Message Types

```rust
/// Message from Reader → CPU Workers
#[derive(Debug)]
struct ChunkMessage {
    chunk_index: usize,
    data: Vec<u8>,
    is_final: bool,
    metadata: ChunkMetadata,
}

/// Message from CPU Workers → Writer
#[derive(Debug)]
struct ProcessedChunkMessage {
    chunk_index: usize,
    processed_data: Vec<u8>,
    is_final: bool,
}
```

### Channel Configuration

```rust
let channel_depth = 4;  // CLI: --channel-depth=4 (default)

// Bounded channels (backpressure!)
let (tx_cpu, rx_cpu) = tokio::sync::mpsc::channel::<ChunkMessage>(channel_depth);
let (tx_writer, rx_writer) = tokio::sync::mpsc::channel::<ProcessedChunkMessage>(channel_depth);
```

**Why bounded?**
- `depth=4` means max 4 chunks buffered in each stage
- Prevents unbounded memory growth
- Creates natural backpressure when writer is slow

---

## Task Implementations

### 1. Reader Task

```rust
/// Educational: Single reader task eliminates read coordination
async fn reader_task(
    input_path: PathBuf,
    chunk_size: usize,
    tx_cpu: Sender<ChunkMessage>,
) -> Result<ReaderStats> {
    // Use FileIOService for streaming reads
    let read_options = ReadOptions {
        chunk_size: Some(chunk_size),
        use_memory_mapping: false,
        calculate_checksums: false,
        ..Default::default()
    };

    let file_io_service = FileIOServiceImpl::new();
    let read_result = file_io_service
        .read_file_chunks(&input_path, read_options)
        .await?;

    let total_chunks = read_result.chunks.len();

    // Send chunks to CPU workers
    for (index, chunk) in read_result.chunks.into_iter().enumerate() {
        let message = ChunkMessage {
            chunk_index: index,
            data: chunk.data().to_vec(),
            is_final: index == total_chunks - 1,
            metadata: /* ... */,
        };

        // Educational: This blocks if channel is full → backpressure!
        tx_cpu.send(message).await
            .map_err(|_| PipelineError::channel_send_failed())?;
    }

    // Drop tx_cpu → signals "no more chunks" to workers
    Ok(ReaderStats { chunks_read: total_chunks })
}
```

**Teaching Points:**
- Single reader → no read coordination needed
- `tx_cpu.send()` blocks when channel full → natural backpressure
- Dropping sender signals completion to receivers

### 2. CPU Worker Task

```rust
/// Educational: Worker pool pattern with channel communication
async fn cpu_worker_task(
    worker_id: usize,
    mut rx_cpu: Receiver<ChunkMessage>,
    tx_writer: Sender<ProcessedChunkMessage>,
    pipeline: Arc<Pipeline>,
    stage_executor: Arc<dyn StageExecutor>,
) -> Result<WorkerStats> {
    use crate::infrastructure::runtime::RESOURCE_MANAGER;
    use crate::infrastructure::metrics::CONCURRENCY_METRICS;

    let mut chunks_processed = 0;

    // Educational: Worker loop - receive, process, send
    while let Some(chunk_msg) = rx_cpu.recv().await {
        // Acquire global CPU token (prevent oversubscription)
        let cpu_wait_start = std::time::Instant::now();
        let _cpu_permit = RESOURCE_MANAGER.acquire_cpu().await?;
        let cpu_wait_duration = cpu_wait_start.elapsed();

        CONCURRENCY_METRICS.record_cpu_wait(cpu_wait_duration);
        CONCURRENCY_METRICS.worker_started();

        // Process chunk through pipeline stages
        let processed_data = process_chunk(
            &chunk_msg.data,
            chunk_msg.chunk_index,
            &pipeline,
            &stage_executor,
        ).await?;

        // Send to writer
        let writer_msg = ProcessedChunkMessage {
            chunk_index: chunk_msg.chunk_index,
            processed_data,
            is_final: chunk_msg.is_final,
        };

        tx_writer.send(writer_msg).await
            .map_err(|_| PipelineError::channel_send_failed())?;

        CONCURRENCY_METRICS.worker_completed();
        chunks_processed += 1;
    }

    Ok(WorkerStats { chunks_processed })
}
```

**Teaching Points:**
- `while let Some(msg) = rx.recv().await` → worker loop pattern
- Global resource manager prevents oversubscription
- Each worker is independent → natural parallelism

### 3. Writer Task

```rust
/// Educational: Single writer eliminates mutex contention
async fn writer_task(
    mut rx_writer: Receiver<ProcessedChunkMessage>,
    output_path: PathBuf,
    header: FileHeader,
    binary_format_service: Arc<dyn BinaryFormatService>,
) -> Result<WriterStats> {
    // NO MUTEX NEEDED! Only one writer task exists.
    let mut binary_writer = binary_format_service
        .create_writer(&output_path, header)?;

    let mut chunks_written = 0;

    // Educational: Sequential writes in arrival order
    // (For ordered writes, we'd need a reordering buffer - future enhancement)
    while let Some(processed_chunk) = rx_writer.recv().await {
        binary_writer.write_chunk(
            processed_chunk.chunk_index,
            &processed_chunk.processed_data,
            processed_chunk.is_final,
        )?;

        chunks_written += 1;
    }

    // Finalize file
    binary_writer.finalize()?;

    Ok(WriterStats { chunks_written })
}
```

**Teaching Points:**
- Single writer → NO MUTEX NEEDED!
- Sequential writes naturally ordered
- Simple, clear responsibility

---

## Main Processing Function Refactor

### Before (Current)

```rust
pub async fn process_file(...) -> Result<...> {
    // Load all chunks into memory
    let input_chunks = /* ... */;

    // Create shared writer with mutex
    let writer_shared = Arc::new(tokio::sync::Mutex::new(binary_writer));

    // Create semaphore for workers
    let semaphore = Arc::new(tokio::sync::Semaphore::new(worker_count));

    // Spawn task per chunk
    for chunk in input_chunks {
        tokio::spawn(async move {
            let _permit = semaphore.acquire().await;
            let mut writer = writer_shared.lock().await; // ← CONTENTION
            // ...
        });
    }
}
```

### After (New)

```rust
pub async fn process_file(
    &self,
    input_path: &Path,
    output_path: &Path,
    pipeline: Arc<Pipeline>,
    observer: Arc<dyn PipelineObserver>,
    worker_count: Option<usize>,
    channel_depth: usize,  // New parameter from CLI
) -> Result<ProcessingResult> {
    // Bounded channels
    let (tx_cpu, rx_cpu) = tokio::sync::mpsc::channel(channel_depth);
    let (tx_writer, rx_writer) = tokio::sync::mpsc::channel(channel_depth);

    // Calculate worker count (adaptive or user-specified)
    let workers = /* ... */;

    // Spawn three-stage pipeline
    let reader = tokio::spawn(reader_task(
        input_path.to_path_buf(),
        chunk_size,
        tx_cpu,
    ));

    let cpu_workers: Vec<_> = (0..workers)
        .map(|id| tokio::spawn(cpu_worker_task(
            id,
            rx_cpu.clone(),
            tx_writer.clone(),
            pipeline.clone(),
            self.stage_executor.clone(),
        )))
        .collect();

    let writer = tokio::spawn(writer_task(
        rx_writer,
        output_path.to_path_buf(),
        header,
        self.binary_format_service.clone(),
    ));

    // Wait for completion
    let reader_result = reader.await??;
    drop(tx_writer);  // Signal workers we're done

    let worker_results: Vec<_> = futures::future::try_join_all(cpu_workers).await?;
    let writer_result = writer.await??;

    // Aggregate results
    Ok(ProcessingResult {
        chunks_processed: worker_results.iter().map(|r| r.chunks_processed).sum(),
        bytes_processed: writer_result.bytes_written,
        /* ... */
    })
}
```

---

## Metrics to Add

```rust
// In ConcurrencyMetrics
pub struct ConcurrencyMetrics {
    // ... existing metrics ...

    /// Current depth of CPU worker channel (gauge)
    pub cpu_queue_depth: AtomicUsize,

    /// Current depth of writer channel (gauge)
    pub writer_queue_depth: AtomicUsize,

    /// Time chunks wait in CPU queue (histogram)
    pub cpu_queue_wait_ms: Mutex<Histogram>,

    /// Time chunks wait in writer queue (histogram)
    pub writer_queue_wait_ms: Mutex<Histogram>,
}
```

---

## Files to Modify

1. **pipeline/src/main.rs**
   - ✅ Add `--channel-depth` CLI flag (DONE)

2. **pipeline/src/application/services/pipeline_service.rs**
   - Add message types (`ChunkMessage`, `ProcessedChunkMessage`)
   - Add three task functions (`reader_task`, `cpu_worker_task`, `writer_task`)
   - Refactor `process_file` to use channel-based architecture
   - Remove mutex-based writer
   - Update metrics recording

3. **pipeline/src/infrastructure/metrics/concurrency_metrics.rs**
   - Add queue depth gauges
   - Add queue wait histograms
   - Add queue depth update methods

---

## Testing Strategy

1. **Unit Tests**
   - Test each task function independently
   - Mock channels for isolated testing

2. **Integration Tests**
   - Process test file through full pipeline
   - Verify output correctness
   - Check all metrics updated

3. **Performance Tests**
   - Compare throughput: mutex vs channels
   - Measure queue depth under load
   - Validate backpressure behavior

---

## Migration Plan

### Phase 1: Add New Code (Non-Breaking)
1. Add message types
2. Add three task functions
3. Add new `process_file_channelized()` method alongside existing

### Phase 2: Switch Default
1. Make `process_file()` call `process_file_channelized()`
2. Keep old implementation as `process_file_legacy()` for comparison

### Phase 3: Cleanup
1. Remove legacy implementation
2. Update all call sites
3. Remove unused mutex code

---

## Expected Results

**Before (Mutex-Based):**
```
10 workers processing 100MB file (8MB chunks)
→ Writer mutex serializes all writes
→ Throughput: ~150 MB/s (write bottleneck)
→ Memory: Loads all chunks immediately (100MB+)
```

**After (Channel-Based):**
```
10 workers processing 100MB file (8MB chunks)
→ Single writer, no contention
→ Throughput: ~250 MB/s (I/O bound, not mutex bound)
→ Memory: Max 4 chunks buffered per stage (32MB)
```

---

## Educational Value

This refactoring demonstrates:
1. **Channel-based concurrency** - Alternative to locks
2. **Backpressure** - Bounded channels prevent overload
3. **Pipeline architecture** - Clean separation of stages
4. **MPSC pattern** - Multiple producers, single consumer
5. **Resource governance** - Global limits + local backpressure

---

## Approval Needed

**Questions for Mike:**

1. **Scope:** Proceed with full refactoring, or start with side-by-side comparison?
2. **Migration:** Prefer gradual (Phase 1-3 above) or all-at-once replacement?
3. **Ordering:** Current design uses arrival order. Need strict chunk ordering?
   - If yes, add reordering buffer in writer (adds complexity)
   - If no, current design is simpler (chunks written as they complete)

4. **Testing:** Any specific workloads to validate against?

**Ready to implement upon approval!**
