# Concurrency Implementation - Final Plan
## Integrated from GPT-5 Feedback + Mike's Constraints

**Date:** 2025-10-03
**Status:** Ready to Implement
**Educational Focus:** DDD/Clean/Hexagonal Architecture with Enterprise Concurrency Patterns

---

## Executive Summary

GPT-5 has validated our Phase 1 scope and provided specific guidance on all open questions. Mike has clarified complexity constraints. This document integrates all feedback into a concrete, actionable plan.

**Key Decisions:**
- ✅ Phase 1 scope confirmed (Global resources, channels, streaming, atomics)
- ✅ Start with CPU + I/O tokens; memory as gauge only
- ✅ Move metrics to END of Phase 1 for better integration
- ✅ Prioritize educational clarity over micro-optimizations
- ✅ Use CLI flags for pattern comparison where pedagogically valuable

---

## Phase 1: Core Patterns (Weeks 1-3)

### Week 1: Global Resource Manager

#### Implementation Scope
```rust
// pipeline/src/infrastructure/runtime/resource_manager.rs
pub struct GlobalResourceManager {
    cpu_tokens: Arc<Semaphore>,      // = cores - 1
    io_tokens: Arc<Semaphore>,       // = NVMe: 24, SSD: 12, HDD: 4
    memory_used: Arc<AtomicUsize>,   // Gauge only (no cap yet)
}

// Educational: Show both acquisition styles
impl GlobalResourceManager {
    // Explicit (pedagogical)
    pub async fn acquire_cpu(&self) -> Result<SemaphorePermit> {
        self.cpu_tokens.acquire().await
    }

    // RAII wrapper (idiomatic)
    pub async fn cpu_lease(&self) -> Result<ResourceLease> {
        Ok(ResourceLease::new(self.cpu_tokens.acquire().await?))
    }
}
```

**Teaching Points:**
- Two-level governance (global + local)
- RAII for automatic cleanup
- Backpressure through semaphores

**Metrics to Add:**
- `cpu_permits_available` (gauge)
- `io_permits_available` (gauge)
- `cpu_permit_wait_ms` (counter)
- `io_permit_wait_ms` (counter)

**CLI Flags:**
```bash
--cpu-tokens=N        # Override default (cores-1)
--io-tokens=N         # Override device default
--io-qdepth=N         # Alias for io-tokens
```

**Files to Modify:**
- Create: `pipeline/src/infrastructure/runtime/resource_manager.rs`
- Create: `pipeline/src/infrastructure/runtime/mod.rs`
- Modify: `pipeline/src/application/services/pipeline_service.rs` (acquire tokens)
- Modify: `pipeline/src/infrastructure/adapters/async_*_adapter.rs` (acquire CPU tokens)

---

### Week 2: Channel-Based Pipeline Architecture

#### Implementation Scope
```rust
// Educational: Three-stage pipeline
async fn process_file_channelized(
    path: PathBuf,
    worker_count: usize,
) -> Result<ProcessingResult> {
    // Bounded channels (default depth=4)
    let (tx_cpu, rx_cpu) = bounded(channel_depth);     // MPSC: one reader, many workers
    let (tx_writer, rx_writer) = bounded(channel_depth); // SPSC: many workers, one writer

    // Stage 1: Reader task
    let reader = tokio::spawn(reader_task(path.clone(), tx_cpu));

    // Stage 2: CPU worker pool
    let cpu_workers: Vec<_> = (0..worker_count)
        .map(|id| tokio::spawn(cpu_worker_task(id, rx_cpu.clone(), tx_writer.clone())))
        .collect();

    // Stage 3: Writer task
    let writer = tokio::spawn(writer_task(rx_writer, output_path));

    // Wait for completion
    let ((reader_result, _), writer_result) = tokio::join!(
        futures::future::try_join_all(std::iter::once(reader).chain(cpu_workers)),
        writer
    );

    // ... error handling and result aggregation
}
```

**Teaching Points:**
- MPSC vs SPSC patterns side-by-side
- Bounded channels create backpressure
- Natural ordering barrier (single writer)
- Error propagation across task boundaries

**Metrics to Add:**
- `cpu_queue_depth` (gauge) - instantaneous
- `writer_queue_depth` (gauge) - instantaneous
- `cpu_queue_wait_ms` (histogram) - time waiting in queue
- `writer_queue_wait_ms` (histogram)

**CLI Flags:**
```bash
--channel-depth=N     # Default: 4 (teaching: show impact of 4 vs 8)
```

**Files to Modify:**
- Major refactor: `pipeline/src/application/services/pipeline_service.rs`
- Remove: `tokio::Mutex` around writer (~L813)
- Add: Three separate task functions (reader, cpu_worker, writer)

---

### Week 3: Streaming I/O with Buffer Reuse

#### Implementation Scope

**Decision (per Mike's constraint):**
- Default: Use existing `FileIOService` (no flag needed)
- Optional: Add minimal reader example in documentation/examples (not main code path)
- Focus: Verify we're using streaming path, not full-file read

**Current Code Audit:**
```rust
// CURRENT (pipeline_service.rs:462-467)
let (input_metadata, input_data) = tokio::join!(
    tokio::fs::metadata(input_path),
    tokio::fs::read(input_path)  // ← THIS NEEDS TO CHANGE
);
```

**Change To:**
```rust
// NEW: Use FileIOService streaming path
let read_options = ReadOptions {
    chunk_size: Some(chunk_size),
    use_memory_mapping: false,  // Start with streaming
    calculate_checksums: false,
    ..Default::default()
};

let read_result = self.file_io_service
    .read_file_chunks(input_path, read_options)
    .await?;

// Send chunks to tx_cpu instead of processing inline
for chunk in read_result.chunks {
    tx_cpu.send(chunk).await?;
}
```

**Buffer Reuse Pattern:**
```rust
// Simple buffer pool (educational clarity)
pub struct BufferPool {
    buffers: Mutex<Vec<Vec<u8>>>,
    capacity: usize,
    chunk_size: usize,
}

impl BufferPool {
    pub async fn acquire(&self) -> Vec<u8> {
        let mut buffers = self.buffers.lock().await;
        buffers.pop().unwrap_or_else(|| {
            vec![0; self.chunk_size]
        })
    }

    pub async fn release(&self, mut buffer: Vec<u8>) {
        buffer.clear();
        let mut buffers = self.buffers.lock().await;
        if buffers.len() < self.capacity {
            buffers.push(buffer);
        }
    }
}
```

**Teaching Points:**
- Streaming reduces memory footprint
- Buffer reuse eliminates allocations
- Ownership transfer via channels
- Compare: batch vs stream memory usage

**Files to Modify:**
- Modify: `pipeline/src/application/services/pipeline_service.rs` (replace full-file read)
- Add: `pipeline/src/infrastructure/runtime/buffer_pool.rs` (simple pool)
- Verify: `FileIOService` is using streaming internally

---

### Week 3 (End): Atomic Ordering + Initial Metrics

#### Atomic Ordering
```rust
// BEFORE
self.bytes_processed.fetch_add(n, Ordering::SeqCst);

// AFTER (with educational comments)
/// Relaxed ordering is sufficient for simple counters because:
/// 1. We only need atomicity (prevent torn reads/writes)
/// 2. We don't coordinate with other atomic variables
/// 3. Final aggregation uses acquire/release for synchronization
///
/// Educational note: When would SeqCst be needed?
/// - Coordinating multiple atomics with total ordering guarantees
/// - Example: Dekker's algorithm, Peterson's algorithm
/// - Our use case: independent counter = Relaxed is optimal
self.bytes_processed.fetch_add(n, Ordering::Relaxed);
```

**Microbenchmark:**
```rust
// benches/atomic_ordering.rs
#[bench]
fn bench_relaxed_counter(b: &mut Bencher) {
    let counter = AtomicU64::new(0);
    b.iter(|| {
        counter.fetch_add(1, Ordering::Relaxed)
    });
}

#[bench]
fn bench_seqcst_counter(b: &mut Bencher) {
    let counter = AtomicU64::new(0);
    b.iter(|| {
        counter.fetch_add(1, Ordering::SeqCst)
    });
}
```

**Teaching Points:**
- Ordering hierarchy: Relaxed < Acquire/Release < SeqCst
- Performance impact of memory fences
- When to use each ordering level

**Files to Modify:**
- Modify: All atomic counters in `pipeline_service.rs`
- Add: Benchmark in `benches/atomic_ordering.rs`
- Add: Documentation examples showing each ordering

#### Initial Metrics (moved from Phase 2)

**Minimal metrics to enable Phase 2 validation:**
```rust
pub struct PipelineMetrics {
    // Counters
    pub files_processed: AtomicU64,
    pub bytes_processed: AtomicU64,
    pub chunks_processed: AtomicU64,

    // Gauges (instant state)
    pub cpu_permits_available: AtomicUsize,
    pub io_permits_available: AtomicUsize,
    pub cpu_queue_depth: AtomicUsize,
    pub writer_queue_depth: AtomicUsize,
    pub active_workers: AtomicUsize,
    pub memory_used_bytes: AtomicUsize,

    // Histograms (latency distributions)
    pub chunk_cpu_time_ms: Mutex<Histogram>,
    pub cpu_queue_wait_ms: Mutex<Histogram>,
    pub writer_queue_wait_ms: Mutex<Histogram>,
    pub cpu_permit_wait_ms: Mutex<Histogram>,
    pub io_permit_wait_ms: Mutex<Histogram>,
}
```

**Files to Modify:**
- Create: `pipeline/src/infrastructure/metrics/concurrency_metrics.rs`
- Modify: Instrument all critical paths with metric collection

---

## Phase 2: Observability & Tuning (Weeks 4-5)

### Week 4: Metrics Dashboard & Visualization

**CLI Output Format:**
```
=== PIPELINE METRICS (1.2s elapsed) ===

Throughput:
  Files:  145 files/s
  Bytes:  2.4 GB/s
  Chunks: 19,234 chunks/s

CPU Resources:
  Permits Available: 3/8
  Permit Wait: P50=0.2ms P95=1.8ms P99=4.3ms
  Queue Depth: avg=2.1, max=4

I/O Resources:
  Permits Available: 18/24
  Permit Wait: P50=0.1ms P95=0.8ms P99=2.1ms
  Queue Depth: avg=1.3, max=4

Latency (chunk processing):
  CPU Time:  P50=1.2ms P95=3.4ms P99=5.8ms
  Queue Wait: P50=0.3ms P95=1.1ms P99=2.9ms

Memory:
  RSS: 342 MB
  Buffer Pool: 89% reuse rate
```

**Optional Prometheus Export:**
- Use existing `metrics_endpoint.rs`
- Add new metrics from Phase 1
- Keep simple (optional, not required)

**Teaching Points:**
- P50/P95/P99 for understanding tail latency
- Queue depth shows backpressure
- Correlation between metrics reveals bottlenecks

**Files to Modify:**
- Create: `pipeline/src/infrastructure/metrics/dashboard.rs`
- Modify: `pipeline/src/infrastructure/metrics/metrics_endpoint.rs`

---

### Week 5: Device-Aware I/O Queue Depth

**Implementation:**
```rust
impl GlobalResourceManager {
    pub fn new(config: ResourceConfig) -> Self {
        let io_tokens = match config.storage_type {
            StorageType::NVMe => 24,
            StorageType::SSD => 12,
            StorageType::HDD => 4,
            StorageType::Auto => Self::detect_optimal_qd(),
            StorageType::Custom(n) => n,
        };

        Self {
            io_tokens: Arc::new(Semaphore::new(io_tokens)),
            // ...
        }
    }

    // Educational: Simple detection
    fn detect_optimal_qd() -> usize {
        // Check /sys/block/*/queue/nr_requests on Linux
        // Or use sysctl on macOS
        // Default to conservative: 12
        12
    }
}
```

**CLI Flags:**
```bash
--storage-type=nvme|ssd|hdd|auto   # Default: auto
--io-qdepth=N                       # Override detection
```

**Teaching Points:**
- Different devices need different queue depths
- NVMe benefits from higher QD (parallel channels)
- HDD suffers from high QD (seek thrashing)
- Measurement is essential

**Validation:**
Create benchmark showing throughput vs QD:
```
QD=2:  150 MB/s (HDD optimal)
QD=4:  180 MB/s
QD=8:  190 MB/s
QD=12: 195 MB/s (SSD optimal)
QD=24: 210 MB/s (NVMe optimal)
QD=48: 205 MB/s (oversubscription)
```

**Files to Modify:**
- Modify: `resource_manager.rs` (add device detection)
- Add: Benchmark script in `benches/io_queue_depth.sh`

---

## Phase 3: Advanced Patterns (Future - Prioritized)

### Task List for Future Implementation

**HIGH PRIORITY (Near Future):**

1. **Spawn Style Comparison** (Educational Value: ⭐⭐⭐⭐)
   - **When:** After Phase 1-2 complete
   - **Why:** Shows different concurrency patterns
   - **Complexity:** Medium (if we have time/resources)
   - **Implementation:**
     ```bash
     --spawn-style=tasks|stream   # DEFAULT: stream

     # tasks: Current explicit tokio::spawn approach
     # stream: futures::stream::iter().buffer_unordered(N)
     ```
   - **Decision:** If too much work, just use stream without flag
   - **Files:** `pipeline_service.rs` - add stream-based alternative

2. **Hill-Climb Chunk Tuning** (Educational Value: ⭐⭐⭐)
   - **When:** After Phase 2 metrics are stable
   - **Why:** Demonstrates adaptive optimization
   - **Complexity:** Medium
   - **Implementation:**
     ```bash
     --chunk-method=filesize|hillclimb   # DEFAULT: hillclimb

     # Algorithm: Simple one-step adjustment
     # If mean chunk CPU time > 5ms for N chunks: decrease size
     # If mean chunk CPU time < 0.5ms for N chunks: increase size
     # Clamp to [128KB, 4MB] aligned to 128KB
     ```
   - **Can stub now:** Add interface, implement fixed sizes initially
   - **Files:** Create `chunk_tuner.rs`

3. **Small-File Fast Path** (Educational Value: ⭐⭐⭐)
   - **When:** After Phase 2
   - **Why:** Shows when to avoid parallelism overhead
   - **Complexity:** Low-Medium
   - **Implementation:**
     ```rust
     const SMALL_FILE_THRESHOLD: u64 = 256 * 1024; // 256KB

     if file_size < SMALL_FILE_THRESHOLD {
         process_inline(path).await  // No chunking
     } else {
         process_channelized(path).await
     }
     ```
   - **Teaching:** Compare syscalls, latency for small vs large
   - **Files:** `pipeline_service.rs` - add dispatch logic

**MEDIUM PRIORITY (Evaluate After Phase 1-2):**

4. **Fair Scheduling (Small/Large Files)** (Educational Value: ⭐⭐⭐)
   - **When:** If profiling shows head-of-line blocking
   - **Why:** Demonstrates scheduling fairness
   - **Complexity:** Medium-High
   - **Implementation:**
     ```rust
     struct FileScheduler {
         small_queue: VecDeque<PathBuf>,  // <10MB
         large_queue: VecDeque<PathBuf>,  // >=10MB
     }

     // Pick ratio: 3 small : 1 large
     ```
   - **Validation:** Create test with 1×10GB + many×10MB
   - **Files:** Create `scheduler.rs`

5. **Reader Implementation Toggle** (Educational Value: ⭐⭐⭐)
   - **When:** Only if we have time and resources
   - **Why:** Shows abstraction comparison
   - **Complexity:** Medium
   - **Implementation:**
     ```bash
     --reader=service|minimal   # DEFAULT: service
     ```
   - **Decision:** Skip if too much work, just use FileIOService
   - **Files:** Create `examples/minimal_reader.rs` (not main code)

**LOW PRIORITY (Nice to Have):**

6. **Batch Small Files** (Educational Value: ⭐⭐)
   - **When:** Much later, if ever
   - **Why:** Advanced optimization pattern
   - **Complexity:** High
   - **Note:** Mention in docs, don't implement unless needed

7. **Advanced Time-Based Tuning** (Educational Value: ⭐⭐)
   - **When:** Only if simple hill-climb insufficient
   - **Why:** Sophisticated but may obscure learning
   - **Complexity:** High
   - **Note:** Simple hill-climb (#2 above) should be sufficient

---

## Validation Scenarios (GPT-5's Recommendations)

After each phase, validate with these test cases:

### 1. CPU-Bound Dataset
**Setup:** Small files, fast NVMe
**Validate:**
- CPU tokens cap run-queue ≈ cores
- CPU utilization 75-90%
- P95 latency stable

### 2. I/O-Bound Dataset
**Setup:** Large sequential I/O
**Validate:**
- Vary I/O QD (4, 8, 12, 24)
- Find throughput "knee"
- Show device saturation point

### 3. Mixed Small + Large Files
**Setup:** 1×10GB + 100×10MB files
**Validate:**
- Small file P95 without fair scheduling
- Small file P95 with fair scheduling (Phase 3)
- Show head-of-line blocking effect

### 4. Memory Stress Test
**Setup:** Many large files concurrently
**Validate:**
- RSS with full-file read (baseline)
- RSS with streaming + buffer reuse
- Show memory savings

### 5. Cancellation Test
**Setup:** Kill mid-processing
**Validate:**
- All tasks drain cleanly
- All permits returned
- No resource leaks

---

## Implementation Timeline

### Phase 1 (Weeks 1-3): ✅ COMMIT TO THIS
- Week 1: Global Resource Manager + Metrics Infrastructure
- Week 2: Channel-Based Pipeline
- Week 3: Streaming I/O + Atomic Ordering + Metrics Integration
- **Deliverable:** Core patterns working, validated, documented

### Phase 2 (Weeks 4-5): ✅ COMMIT TO THIS
- Week 4: CLI Dashboard + Metrics Visualization
- Week 5: Device-Aware I/O Queue Depth
- **Deliverable:** Observable, tunable system

### Phase 3 (Weeks 6+): 🤔 EVALUATE BASED ON RESULTS
- Prioritized task list (see above)
- Implement based on:
  - Available time/resources
  - Profiling results from Phase 1-2
  - Educational value vs complexity

---

## CLI Flags Summary (Final)

**Phase 1:**
```bash
--cpu-tokens=N              # Default: cores-1
--io-tokens=N               # Default: device-specific (24/12/4)
--io-qdepth=N              # Alias for io-tokens
--channel-depth=N          # Default: 4
```

**Phase 2:**
```bash
--storage-type=TYPE        # nvme|ssd|hdd|auto (default: auto)
--metrics-interval=MS      # Dashboard update interval (default: 200ms)
```

**Phase 3 (Future):**
```bash
--chunk-method=METHOD      # filesize|hillclimb (default: hillclimb)
--spawn-style=STYLE        # tasks|stream (default: stream)
--reader=TYPE              # service|minimal (default: service)
--scheduler=TYPE           # simple|fair (default: simple)
```

**Educational Optimization Modes:**
```bash
--optimize=MODE            # throughput|latency|resources (default: throughput)

# Translates to:
# throughput: cpu=cores, io=high, mem=aggressive, chunk=large
# latency:    cpu=cores/2, io=moderate, chunk=small, fairness=ON
# resources:  cpu=cores-2, io=low, mem=conservative, chunk=medium
```

---

## Files to Create/Modify

### Phase 1 (New Files):
- `pipeline/src/infrastructure/runtime/resource_manager.rs`
- `pipeline/src/infrastructure/runtime/buffer_pool.rs`
- `pipeline/src/infrastructure/runtime/mod.rs`
- `pipeline/src/infrastructure/metrics/concurrency_metrics.rs`
- `benches/atomic_ordering.rs`

### Phase 1 (Major Modifications):
- `pipeline/src/application/services/pipeline_service.rs` (major refactor)
- `pipeline/src/infrastructure/adapters/async_*_adapter.rs` (acquire CPU tokens)

### Phase 2 (New Files):
- `pipeline/src/infrastructure/metrics/dashboard.rs`
- `benches/io_queue_depth.sh`

### Phase 3 (Future):
- `pipeline/src/application/services/chunk_tuner.rs`
- `pipeline/src/application/services/scheduler.rs`
- `examples/minimal_reader.rs`
- `examples/spawn_comparison.rs`

---

## Success Criteria

### Phase 1 Complete When:
- ✅ Global resource limits prevent oversubscription
- ✅ Channel-based pipeline eliminates writer mutex
- ✅ Streaming I/O reduces memory footprint (measured)
- ✅ All patterns documented with teaching comments
- ✅ Validation scenarios 1, 4, 5 passing
- ✅ Metrics infrastructure in place

### Phase 2 Complete When:
- ✅ Metrics clearly show bottlenecks
- ✅ CLI dashboard provides real-time visibility
- ✅ I/O QD tuning shows measurable impact
- ✅ Validation scenarios 2, 3 passing
- ✅ Students can debug system from observability

### Overall Educational Success When:
- ✅ Students understand when/why to use each pattern
- ✅ Code is clear enough to teach from
- ✅ Patterns are reusable in other projects
- ✅ Performance improvements validate architectural choices
- ✅ Documentation supports self-guided learning

---

## Next Steps

1. **Get final approval** from Mike on this plan
2. **Begin Week 1** (Global Resource Manager)
3. **Create tracking TODO list** for Phase 1 implementation
4. **Set up validation infrastructure** for testing

Ready to start implementation immediately upon approval!
