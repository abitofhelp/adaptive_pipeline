# Understanding Execution vs Processing Pipelines

**Author:** Claude + Mike Gardner
**Date:** 2025-10-03
**Purpose:** Educational documentation on dual pipeline architecture
**Audience:** Students learning concurrent systems design

---

## Executive Summary

This system uses **two distinct types of pipelines** that work together but serve different purposes:

1. **Execution Pipeline** - The concurrency architecture (HOW work flows)
2. **Processing Pipeline** - The business logic (WHAT work gets done)

Understanding this distinction is critical to understanding the system's architecture.

---

## The Two Pipelines

### 1. Execution Pipeline (Concurrency Architecture)

**Purpose:** Controls HOW chunks flow through the system concurrently

**Structure:** Fixed three-stage architecture
```
┌─────────────┐   Channel    ┌──────────────┐   Channel    ┌─────────────┐
│   Reader    │─────────────▶│ CPU Workers  │─────────────▶│   Writer    │
│   Task      │   (MPSC)     │  (Pool of N) │   (MPSC)     │    Task     │
└─────────────┘              └──────────────┘              └─────────────┘
     │                              │                            │
  Reads chunks              Processes chunks                Writes chunks
  from disk                 in parallel                     to disk
```

**Key Characteristics:**
- **Fixed structure** - Always 3 stages (Reader → Workers → Writer)
- **Concurrency control** - Manages parallelism and backpressure
- **Resource management** - Enforces global resource limits
- **Infrastructure concern** - Part of the application layer orchestration

**Educational Focus:**
- Demonstrates channel-based concurrency patterns
- Shows natural backpressure with bounded channels
- Illustrates MPSC (Multiple Producer, Single Consumer) pattern
- Eliminates lock contention through architecture

---

### 2. Processing Pipeline (Business Logic)

**Purpose:** Defines WHAT processing happens to data

**Structure:** User-configurable sequence of stages
```rust
Pipeline {
    name: "compress-encrypt-archive",
    stages: [
        Stage::Compression(Algorithm::Brotli, level: 6),
        Stage::Encryption(Algorithm::AES256, key: Key::from_file("key.bin")),
        Stage::Checksum(Algorithm::SHA256),
    ]
}
```

**Key Characteristics:**
- **Configurable** - User defines stages and their order
- **Composable** - Any combination of stages
- **Domain logic** - Defines business operations
- **Declarative** - Specified in configuration/code

**Educational Focus:**
- Demonstrates Domain-Driven Design (Pipeline as domain entity)
- Shows Strategy pattern (different algorithms)
- Illustrates composition over inheritance
- Clean separation of concerns

---

## How They Work Together

### The Integration Point

The processing pipeline runs **INSIDE** the CPU worker tasks:

```rust
/// CPU Worker Task - Where execution meets processing
async fn cpu_worker_task(
    worker_id: usize,
    mut rx_cpu: Receiver<ChunkMessage>,
    tx_writer: Sender<ProcessedChunkMessage>,
    pipeline: Arc<Pipeline>,              // ← Processing pipeline config
    stage_executor: Arc<dyn StageExecutor>, // ← Executes processing stages
) -> Result<WorkerStats> {

    while let Some(chunk_msg) = rx_cpu.recv().await {
        // ===================================================
        // EXECUTION PIPELINE: Concurrency management
        // ===================================================

        // Acquire global CPU resource token (prevent oversubscription)
        let _cpu_permit = RESOURCE_MANAGER.acquire_cpu().await?;

        // ===================================================
        // PROCESSING PIPELINE: Business logic execution
        // ===================================================

        let mut data = chunk_msg.data;

        // Execute EACH configured stage sequentially:
        for stage in pipeline.stages() {
            data = stage_executor.execute_stage(stage, data).await?;
            // Stage could be: Compression, Encryption, Checksum, PassThrough, etc.
        }

        // ===================================================
        // EXECUTION PIPELINE: Send result to next stage
        // ===================================================

        // Release CPU token (automatic via RAII)
        // Send processed data to writer channel
        tx_writer.send(ProcessedChunkMessage {
            chunk_index: chunk_msg.chunk_index,
            processed_data: data,
            is_final: chunk_msg.is_final,
        }).await?;
    }

    Ok(WorkerStats { chunks_processed })
}
```

---

## Complete Example: Processing a Single Chunk

### Scenario
User creates pipeline: **Compression → Encryption → Checksum**
Processing a 10MB file with 8MB chunks
Using 4 CPU workers

### Flow for Chunk #2

```
EXECUTION PIPELINE STAGE 1: Reader Task
┌─────────────────────────────────────────────┐
│ Reader Task                                 │
│                                             │
│ 1. Read chunk #2 from disk (8MB)           │
│ 2. Create ChunkMessage:                     │
│    - index: 2                               │
│    - data: [8MB raw bytes]                  │
│    - is_final: false                        │
│ 3. Send to CPU channel                      │
│    (blocks if channel full → backpressure!) │
└────────────────┬────────────────────────────┘
                 │
                 │ ChunkMessage
                 ▼
        [CPU Channel: depth=4]
                 │
                 ▼
EXECUTION PIPELINE STAGE 2: CPU Worker Pool
┌─────────────────────────────────────────────┐
│ CPU Worker #3 (picked up chunk #2)         │
│                                             │
│ EXECUTION: Resource acquisition             │
│ ├─ Acquire global CPU token                │
│ │  (waits if all tokens in use)            │
│ └─ Record CPU wait time metric             │
│                                             │
│ PROCESSING: Execute configured stages       │
│ ┌─────────────────────────────────────────┐ │
│ │ STAGE 1: Compression (Brotli, level=6) │ │ ← Processing Pipeline
│ │ Input:  8MB raw bytes                   │ │
│ │ Output: 2MB compressed bytes            │ │
│ └─────────────┬───────────────────────────┘ │
│               ▼                             │
│ ┌─────────────────────────────────────────┐ │
│ │ STAGE 2: Encryption (AES256)            │ │ ← Processing Pipeline
│ │ Input:  2MB compressed bytes            │ │
│ │ Output: 2MB encrypted bytes             │ │
│ └─────────────┬───────────────────────────┘ │
│               ▼                             │
│ ┌─────────────────────────────────────────┐ │
│ │ STAGE 3: Checksum (SHA256)              │ │ ← Processing Pipeline
│ │ Input:  2MB encrypted bytes             │ │
│ │ Output: 2MB + 32 bytes (with checksum)  │ │
│ └─────────────┬───────────────────────────┘ │
│               ▼                             │
│ EXECUTION: Send result                      │
│ ├─ Create ProcessedChunkMessage            │
│ ├─ Send to writer channel                  │
│ └─ Release CPU token (automatic)           │
└────────────────┬────────────────────────────┘
                 │
                 │ ProcessedChunkMessage
                 ▼
       [Writer Channel: depth=4]
                 │
                 ▼
EXECUTION PIPELINE STAGE 3: Writer Task
┌─────────────────────────────────────────────┐
│ Writer Task                                 │
│                                             │
│ 1. Receive ProcessedChunkMessage            │
│ 2. Write chunk #2 to output file            │
│    (sequential write, NO MUTEX!)           │
│ 3. Update progress metrics                  │
└─────────────────────────────────────────────┘
```

### Parallel Processing Visualization

**At the same time, other workers are processing different chunks:**

```
Worker #1: Chunk #0 → [Compress → Encrypt → Checksum] → Writer
Worker #2: Chunk #1 → [Compress → Encrypt → Checksum] → Writer
Worker #3: Chunk #2 → [Compress → Encrypt → Checksum] → Writer (shown above)
Worker #4: Chunk #3 → [Compress → Encrypt → Checksum] → Writer
```

**Key insight:** Parallelism happens at the **chunk level**, not the stage level.
Each worker executes ALL processing stages sequentially for ONE chunk.

---

## Different Processing Pipeline Configurations

The **same execution pipeline** works with **different processing configurations**:

### Configuration A: Compression Only
```rust
Pipeline {
    stages: [
        Compression(Brotli, level=6)
    ]
}

// CPU worker executes:
data = compress(data);  // ← Only one processing stage
```

### Configuration B: Encryption Only
```rust
Pipeline {
    stages: [
        Encryption(AES256, key=...)
    ]
}

// CPU worker executes:
data = encrypt(data);  // ← Only one processing stage
```

### Configuration C: Full Processing
```rust
Pipeline {
    stages: [
        Compression(Brotli, level=6),
        Encryption(ChaCha20, key=...),
        Checksum(SHA256)
    ]
}

// CPU worker executes:
data = compress(data);
data = encrypt(data);
data = checksum(data);  // ← Three processing stages
```

### Configuration D: Pass-Through (No Processing)
```rust
Pipeline {
    stages: [
        PassThrough
    ]
}

// CPU worker executes:
data = data;  // ← Just validates, no transformation
```

### Configuration E: Checksum Only (Validation)
```rust
Pipeline {
    stages: [
        Checksum(SHA256)
    ]
}

// CPU worker executes:
data = checksum(data);  // ← Only calculate checksum
```

**All configurations use the same 3-stage execution pipeline!**

---

## Architectural Benefits

### Separation of Concerns

**Execution Pipeline responsibilities:**
- Concurrency control
- Backpressure management
- Resource coordination
- Task scheduling
- Channel communication

**Processing Pipeline responsibilities:**
- Data transformation
- Algorithm selection
- Business rules
- Domain logic
- Stage composition

### Independent Evolution

**You can change:**
- Execution strategy (channels → streams → actors)
- Processing stages (add new algorithms)
- Without affecting the other!

### Educational Value

**Students learn:**
1. **Two types of composition:**
   - Horizontal: Execution stages (Reader → Workers → Writer)
   - Vertical: Processing stages (Compress → Encrypt → Checksum)

2. **Separation of concerns:**
   - Infrastructure vs Domain
   - Concurrency vs Business Logic

3. **Architectural patterns:**
   - Pipeline pattern (both types!)
   - Strategy pattern (stage algorithms)
   - MPSC communication
   - Bounded buffering

---

## Common Misconceptions

### ❌ Misconception 1: "The stages are pipeline stages"
**Correct:** The processing stages (Compression, Encryption) run INSIDE the CPU workers.
The execution pipeline stages are Reader/Workers/Writer.

### ❌ Misconception 2: "Each stage is a separate task"
**Correct:** Each WORKER is a separate task. Within each worker, stages execute sequentially.

### ❌ Misconception 3: "Parallelism happens across stages"
**Correct:** Parallelism happens across CHUNKS. Each chunk goes through all stages sequentially,
but multiple chunks are processed in parallel by different workers.

### ❌ Misconception 4: "We have a pipeline within a pipeline"
**Correct:** Sort of! But they serve different purposes:
- Execution pipeline = infrastructure (concurrency)
- Processing pipeline = domain (business logic)

---

## Code Location Reference

### Execution Pipeline Code

**Reader Task:**
- Location: `pipeline/src/application/services/pipeline_service.rs::reader_task()`
- Purpose: Reads chunks from disk, sends to CPU workers

**CPU Worker Task:**
- Location: `pipeline/src/application/services/pipeline_service.rs::cpu_worker_task()`
- Purpose: Receives chunks, executes processing stages, sends to writer

**Writer Task:**
- Location: `pipeline/src/application/services/pipeline_service.rs::writer_task()`
- Purpose: Receives processed chunks, writes to output file

### Processing Pipeline Code

**Pipeline Domain Entity:**
- Location: `pipeline_domain/src/entities/pipeline.rs`
- Purpose: Defines configurable processing stages

**Stage Executor:**
- Location: `pipeline/src/infrastructure/repositories/stage_executor.rs`
- Purpose: Executes individual processing stages

**Stage Implementations:**
- Compression: `pipeline/src/infrastructure/adapters/async_compression_adapter.rs`
- Encryption: `pipeline/src/infrastructure/adapters/async_encryption_adapter.rs`
- Checksum: `pipeline/src/infrastructure/adapters/async_checksum_adapter.rs`

---

## Summary

| Aspect | Execution Pipeline | Processing Pipeline |
|--------|-------------------|-------------------|
| **Purpose** | Concurrency orchestration | Business logic |
| **Structure** | Fixed (Reader → Workers → Writer) | Configurable (user-defined stages) |
| **Layer** | Infrastructure/Application | Domain |
| **Parallelism** | Across chunks | Sequential within chunk |
| **Pattern** | Channel-based communication | Stage composition |
| **Changes** | Rarely (architectural) | Often (new features) |
| **Educational** | Concurrent systems patterns | DDD, Strategy pattern |

**Key Takeaway:** The execution pipeline provides the concurrency infrastructure,
while the processing pipeline provides the business logic. They're complementary,
not conflicting. This separation enables independent evolution and clear
architectural boundaries.

---

## Next Steps for Students

After understanding this distinction:

1. **Trace a chunk through both pipelines** using a debugger
2. **Create a custom processing pipeline** with different stages
3. **Measure the impact** of different channel depths (execution) vs stage counts (processing)
4. **Modify one pipeline** without touching the other to see independence
5. **Add metrics** to observe both pipelines separately

---

**This document should be included in:**
- Architecture documentation (high-level overview)
- Developer onboarding materials
- Educational courseware
- API documentation (as context)
