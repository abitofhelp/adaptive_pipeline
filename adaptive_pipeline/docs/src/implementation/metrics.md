# Metrics Collection

**Version:** 1.0
**Date:** 2025-01-04
**SPDX-License-Identifier:** BSD-3-Clause
**License File:** See the LICENSE file in the project root.
**Copyright:** © 2025 Michael Gardner, A Bit of Help, Inc.
**Authors:** Michael Gardner, Claude Code
**Status:** Active

## Overview

The pipeline system implements comprehensive metrics collection for monitoring, observability, and performance analysis. Metrics are collected in real-time, exported in Prometheus format, and can be visualized using Grafana dashboards.

**Key Features:**
- **Prometheus Integration**: Native Prometheus metrics export
- **Real-Time Collection**: Low-overhead metric updates
- **Comprehensive Coverage**: Performance, system, and business metrics
- **Dimensional Data**: Labels for multi-dimensional analysis
- **Thread-Safe**: Safe concurrent metric updates

## Metrics Architecture

### Two-Level Metrics System

The system uses a two-level metrics architecture:

```text
┌─────────────────────────────────────────────┐
│         Application Layer                   │
│  - ProcessingMetrics (domain entity)        │
│  - Per-pipeline performance tracking        │
│  - Business metrics and analytics           │
└─────────────────┬───────────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────────┐
│       Infrastructure Layer                  │
│  - MetricsService (Prometheus)              │
│  - System-wide aggregation                  │
│  - HTTP export endpoint                     │
└─────────────────────────────────────────────┘
```

**Domain Metrics (ProcessingMetrics):**
- Attached to processing context
- Track individual pipeline execution
- Support detailed analytics
- Persist to database

**Infrastructure Metrics (MetricsService):**
- System-wide aggregation
- Prometheus counters, gauges, histograms
- HTTP /metrics endpoint
- Real-time monitoring

## Domain Metrics

### ProcessingMetrics Entity

Tracks performance data for individual pipeline executions:

```rust
use adaptive_pipeline_domain::entities::ProcessingMetrics;
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMetrics {
    // Progress tracking
    bytes_processed: u64,
    bytes_total: u64,
    chunks_processed: u64,
    chunks_total: u64,

    // Timing (high-resolution internal, RFC3339 for export)
    #[serde(skip)]
    start_time: Option<Instant>,
    #[serde(skip)]
    end_time: Option<Instant>,
    start_time_rfc3339: Option<String>,
    end_time_rfc3339: Option<String>,
    processing_duration: Option<Duration>,

    // Performance metrics
    throughput_bytes_per_second: f64,
    compression_ratio: Option<f64>,

    // Error tracking
    error_count: u64,
    warning_count: u64,

    // File information
    input_file_size_bytes: u64,
    output_file_size_bytes: u64,
    input_file_checksum: Option<String>,
    output_file_checksum: Option<String>,

    // Per-stage metrics
    stage_metrics: HashMap<String, StageMetrics>,
}
```

### Creating and Using ProcessingMetrics

```rust
impl ProcessingMetrics {
    /// Create new metrics for pipeline execution
    pub fn new(total_bytes: u64, total_chunks: u64) -> Self {
        Self {
            bytes_processed: 0,
            bytes_total: total_bytes,
            chunks_processed: 0,
            chunks_total: total_chunks,
            start_time: Some(Instant::now()),
            start_time_rfc3339: Some(Utc::now().to_rfc3339()),
            end_time: None,
            end_time_rfc3339: None,
            processing_duration: None,
            throughput_bytes_per_second: 0.0,
            compression_ratio: None,
            error_count: 0,
            warning_count: 0,
            input_file_size_bytes: total_bytes,
            output_file_size_bytes: 0,
            input_file_checksum: None,
            output_file_checksum: None,
            stage_metrics: HashMap::new(),
        }
    }

    /// Update progress
    pub fn update_progress(&mut self, bytes: u64, chunks: u64) {
        self.bytes_processed += bytes;
        self.chunks_processed += chunks;
        self.update_throughput();
    }

    /// Calculate throughput
    fn update_throughput(&mut self) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                self.throughput_bytes_per_second =
                    self.bytes_processed as f64 / elapsed;
            }
        }
    }

    /// Complete processing
    pub fn complete(&mut self) {
        self.end_time = Some(Instant::now());
        self.end_time_rfc3339 = Some(Utc::now().to_rfc3339());

        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            self.processing_duration = Some(end - start);
        }

        self.update_throughput();
        self.calculate_compression_ratio();
    }

    /// Calculate compression ratio
    fn calculate_compression_ratio(&mut self) {
        if self.output_file_size_bytes > 0 && self.input_file_size_bytes > 0 {
            self.compression_ratio = Some(
                self.output_file_size_bytes as f64 /
                self.input_file_size_bytes as f64
            );
        }
    }
}
```

### StageMetrics

Track performance for individual pipeline stages:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageMetrics {
    /// Stage name
    stage_name: String,

    /// Bytes processed by this stage
    bytes_processed: u64,

    /// Processing time for this stage
    #[serde(skip)]
    processing_time: Option<Duration>,

    /// Throughput (bytes per second)
    throughput_bps: f64,

    /// Error count for this stage
    error_count: u64,

    /// Memory usage (optional)
    memory_usage_bytes: Option<u64>,
}

impl ProcessingMetrics {
    /// Record stage metrics
    pub fn record_stage_metrics(
        &mut self,
        stage_name: String,
        bytes: u64,
        duration: Duration,
    ) {
        let throughput = if duration.as_secs_f64() > 0.0 {
            bytes as f64 / duration.as_secs_f64()
        } else {
            0.0
        };

        let stage_metrics = StageMetrics {
            stage_name: stage_name.clone(),
            bytes_processed: bytes,
            processing_time: Some(duration),
            throughput_bps: throughput,
            error_count: 0,
            memory_usage_bytes: None,
        };

        self.stage_metrics.insert(stage_name, stage_metrics);
    }
}
```

## Prometheus Metrics

### MetricsService

Infrastructure service for Prometheus metrics:

```rust
use prometheus::{
    IntCounter, IntGauge, Gauge, Histogram,
    HistogramOpts, Opts, Registry
};
use std::sync::Arc;

#[derive(Clone)]
pub struct MetricsService {
    registry: Arc<Registry>,

    // Pipeline execution metrics
    pipelines_processed_total: IntCounter,
    pipeline_processing_duration: Histogram,
    pipeline_bytes_processed_total: IntCounter,
    pipeline_chunks_processed_total: IntCounter,
    pipeline_errors_total: IntCounter,
    pipeline_warnings_total: IntCounter,

    // Performance metrics
    throughput_mbps: Gauge,
    compression_ratio: Gauge,
    active_pipelines: IntGauge,

    // System metrics
    memory_usage_bytes: IntGauge,
    cpu_utilization_percent: Gauge,
}

impl MetricsService {
    pub fn new() -> Result<Self, PipelineError> {
        let registry = Arc::new(Registry::new());

        // Create counters
        let pipelines_processed_total = IntCounter::new(
            "pipeline_processed_total",
            "Total number of pipelines processed"
        )?;

        let pipeline_bytes_processed_total = IntCounter::new(
            "pipeline_bytes_processed_total",
            "Total bytes processed by pipelines"
        )?;

        let pipeline_errors_total = IntCounter::new(
            "pipeline_errors_total",
            "Total number of processing errors"
        )?;

        // Create histograms
        let pipeline_processing_duration = Histogram::with_opts(
            HistogramOpts::new(
                "pipeline_processing_duration_seconds",
                "Pipeline processing duration in seconds"
            )
            .buckets(vec![0.1, 0.5, 1.0, 5.0, 10.0, 30.0, 60.0])
        )?;

        // Create gauges
        let throughput_mbps = Gauge::new(
            "pipeline_throughput_mbps",
            "Current processing throughput in MB/s"
        )?;

        let compression_ratio = Gauge::new(
            "pipeline_compression_ratio",
            "Current compression ratio"
        )?;

        let active_pipelines = IntGauge::new(
            "pipeline_active_count",
            "Number of currently active pipelines"
        )?;

        // Register all metrics
        registry.register(Box::new(pipelines_processed_total.clone()))?;
        registry.register(Box::new(pipeline_bytes_processed_total.clone()))?;
        registry.register(Box::new(pipeline_errors_total.clone()))?;
        registry.register(Box::new(pipeline_processing_duration.clone()))?;
        registry.register(Box::new(throughput_mbps.clone()))?;
        registry.register(Box::new(compression_ratio.clone()))?;
        registry.register(Box::new(active_pipelines.clone()))?;

        Ok(Self {
            registry,
            pipelines_processed_total,
            pipeline_processing_duration,
            pipeline_bytes_processed_total,
            pipeline_chunks_processed_total: /* ... */,
            pipeline_errors_total,
            pipeline_warnings_total: /* ... */,
            throughput_mbps,
            compression_ratio,
            active_pipelines,
            memory_usage_bytes: /* ... */,
            cpu_utilization_percent: /* ... */,
        })
    }
}
```

### Recording Metrics

```rust
impl MetricsService {
    /// Record pipeline completion
    pub fn record_pipeline_completion(
        &self,
        metrics: &ProcessingMetrics
    ) -> Result<(), PipelineError> {
        // Increment counters
        self.pipelines_processed_total.inc();
        self.pipeline_bytes_processed_total
            .inc_by(metrics.bytes_processed());
        self.pipeline_chunks_processed_total
            .inc_by(metrics.chunks_processed());

        // Record duration
        if let Some(duration) = metrics.processing_duration() {
            self.pipeline_processing_duration
                .observe(duration.as_secs_f64());
        }

        // Update gauges
        self.throughput_mbps.set(
            metrics.throughput_bytes_per_second() / 1_000_000.0
        );

        if let Some(ratio) = metrics.compression_ratio() {
            self.compression_ratio.set(ratio);
        }

        // Record errors
        if metrics.error_count() > 0 {
            self.pipeline_errors_total
                .inc_by(metrics.error_count());
        }

        Ok(())
    }

    /// Record pipeline start
    pub fn record_pipeline_start(&self) {
        self.active_pipelines.inc();
    }

    /// Record pipeline end
    pub fn record_pipeline_end(&self) {
        self.active_pipelines.dec();
    }
}
```

## Available Metrics

### Counter Metrics

Monotonically increasing values:

| Metric Name | Type | Description |
|-------------|------|-------------|
| `pipeline_processed_total` | Counter | Total pipelines processed |
| `pipeline_bytes_processed_total` | Counter | Total bytes processed |
| `pipeline_chunks_processed_total` | Counter | Total chunks processed |
| `pipeline_errors_total` | Counter | Total processing errors |
| `pipeline_warnings_total` | Counter | Total warnings |

### Gauge Metrics

Values that can increase or decrease:

| Metric Name | Type | Description |
|-------------|------|-------------|
| `pipeline_active_count` | Gauge | Currently active pipelines |
| `pipeline_throughput_mbps` | Gauge | Current throughput (MB/s) |
| `pipeline_compression_ratio` | Gauge | Current compression ratio |
| `pipeline_memory_usage_bytes` | Gauge | Memory usage in bytes |
| `pipeline_cpu_utilization_percent` | Gauge | CPU utilization percentage |

### Histogram Metrics

Distribution of values with buckets:

| Metric Name | Type | Buckets | Description |
|-------------|------|---------|-------------|
| `pipeline_processing_duration_seconds` | Histogram | 0.1, 0.5, 1, 5, 10, 30, 60 | Processing duration |
| `pipeline_chunk_size_bytes` | Histogram | 1K, 10K, 100K, 1M, 10M | Chunk size distribution |
| `pipeline_stage_duration_seconds` | Histogram | 0.01, 0.1, 0.5, 1, 5 | Stage processing time |

## Metrics Export

### HTTP Endpoint

Export metrics via HTTP for Prometheus scraping:

```rust
use warp::Filter;

pub fn metrics_endpoint(
    metrics_service: Arc<MetricsService>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("metrics")
        .and(warp::get())
        .map(move || {
            let encoder = prometheus::TextEncoder::new();
            let metric_families = metrics_service.registry.gather();
            let mut buffer = Vec::new();

            encoder.encode(&metric_families, &mut buffer)
                .unwrap_or_else(|e| {
                    eprintln!("Failed to encode metrics: {}", e);
                });

            warp::reply::with_header(
                buffer,
                "Content-Type",
                encoder.format_type()
            )
        })
}
```

### Prometheus Configuration

Configure Prometheus to scrape metrics:

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'pipeline'
    scrape_interval: 15s
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: '/metrics'
```

### Example Metrics Output

```text
# HELP pipeline_processed_total Total number of pipelines processed
# TYPE pipeline_processed_total counter
pipeline_processed_total 1234

# HELP pipeline_bytes_processed_total Total bytes processed
# TYPE pipeline_bytes_processed_total counter
pipeline_bytes_processed_total 1073741824

# HELP pipeline_processing_duration_seconds Pipeline processing duration
# TYPE pipeline_processing_duration_seconds histogram
pipeline_processing_duration_seconds_bucket{le="0.1"} 45
pipeline_processing_duration_seconds_bucket{le="0.5"} 120
pipeline_processing_duration_seconds_bucket{le="1.0"} 280
pipeline_processing_duration_seconds_bucket{le="5.0"} 450
pipeline_processing_duration_seconds_bucket{le="10.0"} 500
pipeline_processing_duration_seconds_bucket{le="+Inf"} 520
pipeline_processing_duration_seconds_sum 2340.5
pipeline_processing_duration_seconds_count 520

# HELP pipeline_throughput_mbps Current processing throughput
# TYPE pipeline_throughput_mbps gauge
pipeline_throughput_mbps 125.7

# HELP pipeline_compression_ratio Current compression ratio
# TYPE pipeline_compression_ratio gauge
pipeline_compression_ratio 0.35
```

## Integration with Processing

### Automatic Metric Collection

Metrics are automatically collected during processing:

```rust
use adaptive_pipeline::MetricsService;

async fn process_file_with_metrics(
    input_path: &str,
    output_path: &str,
    metrics_service: Arc<MetricsService>,
) -> Result<(), PipelineError> {
    // Create domain metrics
    let mut metrics = ProcessingMetrics::new(
        file_size,
        chunk_count
    );

    // Record start
    metrics_service.record_pipeline_start();

    // Process file
    for chunk in chunks {
        let start = Instant::now();

        let processed = process_chunk(chunk)?;

        metrics.update_progress(
            processed.len() as u64,
            1
        );

        metrics.record_stage_metrics(
            "compression".to_string(),
            processed.len() as u64,
            start.elapsed()
        );
    }

    // Complete metrics
    metrics.complete();

    // Export to Prometheus
    metrics_service.record_pipeline_completion(&metrics)?;
    metrics_service.record_pipeline_end();

    Ok(())
}
```

### Observer Pattern

Use observer pattern for automatic metric updates:

```rust
pub struct MetricsObserver {
    metrics_service: Arc<MetricsService>,
}

impl MetricsObserver {
    pub fn observe_processing(
        &self,
        event: ProcessingEvent
    ) {
        match event {
            ProcessingEvent::PipelineStarted => {
                self.metrics_service.record_pipeline_start();
            }
            ProcessingEvent::ChunkProcessed { bytes, duration } => {
                self.metrics_service.pipeline_bytes_processed_total
                    .inc_by(bytes);
                self.metrics_service.pipeline_processing_duration
                    .observe(duration.as_secs_f64());
            }
            ProcessingEvent::PipelineCompleted { metrics } => {
                self.metrics_service
                    .record_pipeline_completion(&metrics)
                    .ok();
                self.metrics_service.record_pipeline_end();
            }
            ProcessingEvent::Error => {
                self.metrics_service.pipeline_errors_total.inc();
            }
        }
    }
}
```

## Visualization with Grafana

### Dashboard Configuration

Create Grafana dashboard for visualization:

```json
{
  "dashboard": {
    "title": "Pipeline Metrics",
    "panels": [
      {
        "title": "Throughput",
        "targets": [{
          "expr": "rate(pipeline_bytes_processed_total[5m]) / 1000000",
          "legendFormat": "Throughput (MB/s)"
        }]
      },
      {
        "title": "Processing Duration (P95)",
        "targets": [{
          "expr": "histogram_quantile(0.95, rate(pipeline_processing_duration_seconds_bucket[5m]))",
          "legendFormat": "P95 Duration"
        }]
      },
      {
        "title": "Active Pipelines",
        "targets": [{
          "expr": "pipeline_active_count",
          "legendFormat": "Active"
        }]
      },
      {
        "title": "Error Rate",
        "targets": [{
          "expr": "rate(pipeline_errors_total[5m])",
          "legendFormat": "Errors/sec"
        }]
      }
    ]
  }
}
```

### Common Queries

**Throughput over time:**
```promql
rate(pipeline_bytes_processed_total[5m]) / 1000000
```

**Average processing duration:**
```promql
rate(pipeline_processing_duration_seconds_sum[5m]) /
rate(pipeline_processing_duration_seconds_count[5m])
```

**P99 latency:**
```promql
histogram_quantile(0.99,
  rate(pipeline_processing_duration_seconds_bucket[5m])
)
```

**Error rate:**
```promql
rate(pipeline_errors_total[5m])
```

**Compression effectiveness:**
```promql
avg(pipeline_compression_ratio)
```

## Performance Considerations

### Low-Overhead Updates

Metrics use atomic operations for minimal overhead:

```rust
// ✅ GOOD: Atomic increment
self.counter.inc();

// ❌ BAD: Locking for simple increment
let mut guard = self.counter.lock().unwrap();
*guard += 1;
```

### Batch Updates

Batch metric updates when possible:

```rust
// ✅ GOOD: Batch update
self.pipeline_bytes_processed_total.inc_by(total_bytes);

// ❌ BAD: Multiple individual updates
for _ in 0..total_bytes {
    self.pipeline_bytes_processed_total.inc();
}
```

### Efficient Labels

Use labels judiciously to avoid cardinality explosion:

```rust
// ✅ GOOD: Limited cardinality
let counter = register_int_counter_vec!(
    "pipeline_processed_total",
    "Total pipelines processed",
    &["algorithm", "stage"]  // ~10 algorithms × ~5 stages = 50 series
)?;

// ❌ BAD: High cardinality
let counter = register_int_counter_vec!(
    "pipeline_processed_total",
    "Total pipelines processed",
    &["pipeline_id", "user_id"]  // Could be millions of series!
)?;
```

## Alerting

### Alert Rules

Define Prometheus alert rules:

```yaml
# alerts.yml
groups:
  - name: pipeline_alerts
    rules:
      - alert: HighErrorRate
        expr: rate(pipeline_errors_total[5m]) > 0.1
        for: 5m
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }} errors/sec"

      - alert: LowThroughput
        expr: rate(pipeline_bytes_processed_total[5m]) < 1000000
        for: 10m
        annotations:
          summary: "Low throughput detected"
          description: "Throughput is {{ $value }} bytes/sec"

      - alert: HighLatency
        expr: |
          histogram_quantile(0.95,
            rate(pipeline_processing_duration_seconds_bucket[5m])
          ) > 60
        for: 5m
        annotations:
          summary: "High P95 latency"
          description: "P95 latency is {{ $value }}s"
```

## Best Practices

### Metric Naming

Follow Prometheus naming conventions:

```rust
// ✅ GOOD: Clear, consistent names
pipeline_bytes_processed_total      // Counter with _total suffix
pipeline_processing_duration_seconds // Time in base unit (seconds)
pipeline_active_count               // Gauge without suffix

// ❌ BAD: Inconsistent naming
processed_bytes                     // Missing namespace
duration_ms                        // Wrong unit (use seconds)
active                             // Too vague
```

### Unit Consistency

Always use base units:

```rust
// ✅ GOOD: Base units
duration_seconds: f64              // Seconds, not milliseconds
size_bytes: u64                    // Bytes, not KB/MB
ratio: f64                         // Unitless ratio 0.0-1.0

// ❌ BAD: Non-standard units
duration_ms: u64
size_mb: f64
percentage: u8
```

### Documentation

Document all metrics:

```rust
// ✅ GOOD: Well documented
let counter = IntCounter::with_opts(
    Opts::new(
        "pipeline_processed_total",
        "Total number of pipelines processed successfully. \
         Incremented on completion of each pipeline execution."
    )
)?;
```

## Next Steps

Now that you understand metrics collection:

- [Logging](logging.md) - Structured logging implementation
- [Observability](observability.md) - Complete observability strategy
- [Performance](../advanced/performance.md) - Performance optimization
