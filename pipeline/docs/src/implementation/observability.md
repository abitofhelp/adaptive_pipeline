# Observability Overview

**Version**: 1.0
**Date**: 2025-10-04
**License**: BSD-3-Clause
**Copyright**: (c) 2025 Michael Gardner, A Bit of Help, Inc.
**Authors**: Michael Gardner
**Status**: Active

---

## Overview

Observability is the ability to understand the internal state of a system by examining its external outputs. The Optimized Adaptive Pipeline implements a comprehensive observability strategy that combines **metrics**, **logging**, and **health monitoring** to provide complete system visibility.

### Key Principles

- **Three Pillars**: Metrics, Logs, and Traces (health monitoring)
- **Comprehensive Coverage**: Monitor all aspects of system operation
- **Real-Time Insights**: Live performance tracking and alerting
- **Low Overhead**: Minimal performance impact on pipeline processing
- **Integration Ready**: Compatible with external monitoring systems (Prometheus, Grafana)
- **Actionable**: Designed to support debugging, optimization, and operations

---

## The Three Pillars

### 1. Metrics - Quantitative Measurements

**What**: Numerical measurements aggregated over time

**Purpose**: Track system performance, identify trends, detect anomalies

**Implementation**:
- Domain layer: `ProcessingMetrics` entity
- Infrastructure layer: `MetricsService` with Prometheus integration
- HTTP `/metrics` endpoint for scraping

**Key Metrics**:
- **Counters**: Total pipelines processed, bytes processed, errors
- **Gauges**: Active pipelines, current throughput, memory usage
- **Histograms**: Processing duration, latency distribution

**See**: [Metrics Collection](metrics.md)

### 2. Logging - Contextual Events

**What**: Timestamped records of discrete events with structured context

**Purpose**: Understand what happened, when, and why

**Implementation**:
- Bootstrap phase: `BootstrapLogger` trait
- Application phase: `tracing` crate with structured logging
- Multiple log levels: ERROR, WARN, INFO, DEBUG, TRACE

**Key Features**:
- Structured fields for filtering and analysis
- Correlation IDs for request tracing
- Integration with ObservabilityService for alerts

**See**: [Logging Implementation](logging.md)

### 3. Health Monitoring - System Status

**What**: Aggregated health scores and status indicators

**Purpose**: Quickly assess system health and detect degradation

**Implementation**:
- `ObservabilityService` with real-time health scoring
- `SystemHealth` status reporting
- Alert generation for threshold violations

**Key Components**:
- Performance health (throughput, latency)
- Error health (error rates, failure patterns)
- Resource health (CPU, memory, I/O)
- Overall health score (weighted composite)

---

## Architecture

### Layered Observability

```text
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │           ObservabilityService                      │   │
│  │  (Orchestrates monitoring, alerting, health)        │   │
│  └──────────┬────────────────┬──────────────┬──────────┘   │
│             │                │              │               │
│             ▼                ▼              ▼               │
│  ┌──────────────┐  ┌─────────────┐  ┌─────────────┐       │
│  │ Performance  │  │   Alert     │  │   Health    │       │
│  │   Tracker    │  │  Manager    │  │  Monitor    │       │
│  └──────────────┘  └─────────────┘  └─────────────┘       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ Uses
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                  Infrastructure Layer                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────┐              ┌──────────────────┐    │
│  │ MetricsService   │              │ Logging (tracing)│    │
│  │ (Prometheus)     │              │ (Structured logs)│    │
│  └──────────────────┘              └──────────────────┘    │
│           │                                 │               │
│           │                                 │               │
│           ▼                                 ▼               │
│  ┌──────────────────┐              ┌──────────────────┐    │
│  │ /metrics HTTP    │              │ Log Subscribers  │    │
│  │ endpoint         │              │ (console, file)  │    │
│  └──────────────────┘              └──────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
                           │
                           │ Exposes
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                    External Systems                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐ │
│  │  Prometheus  │    │    Grafana   │    │ Log Analysis │ │
│  │   (Scraper)  │    │ (Dashboards) │    │    Tools     │ │
│  └──────────────┘    └──────────────┘    └──────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Component Integration

The observability components are tightly integrated:

1. **ObservabilityService** orchestrates monitoring
2. **MetricsService** records quantitative data
3. **Logging** records contextual events
4. **PerformanceTracker** maintains real-time state
5. **AlertManager** checks thresholds and generates alerts
6. **HealthMonitor** computes system health scores

---

## ObservabilityService

### Core Responsibilities

The `ObservabilityService` is the central orchestrator for monitoring:

```rust
pub struct ObservabilityService {
    metrics_service: Arc<MetricsService>,
    performance_tracker: Arc<RwLock<PerformanceTracker>>,
    alert_thresholds: AlertThresholds,
}
```

**Key Methods**:
- `start_operation()` - Begin tracking an operation
- `complete_operation()` - End tracking with metrics
- `get_system_health()` - Get current health status
- `record_processing_metrics()` - Record pipeline metrics
- `check_alerts()` - Evaluate alert conditions

### PerformanceTracker

Maintains real-time performance state:

```rust
pub struct PerformanceTracker {
    pub active_operations: u32,
    pub total_operations: u64,
    pub average_throughput_mbps: f64,
    pub peak_throughput_mbps: f64,
    pub error_rate_percent: f64,
    pub system_health_score: f64,
    pub last_update: Instant,
}
```

**Tracked Metrics**:
- Active operation count
- Total operation count
- Average and peak throughput
- Error rate percentage
- Overall health score
- Last update timestamp

### OperationTracker

Automatic operation lifecycle tracking:

```rust
pub struct OperationTracker {
    operation_name: String,
    start_time: Instant,
    observability_service: ObservabilityService,
    completed: AtomicBool,
}
```

**Lifecycle**:
1. Created via `start_operation()`
2. Increments active operation count
3. Logs operation start
4. On completion: Records duration, throughput, success/failure
5. On drop (if not completed): Marks as failed

**Drop Safety**: If the tracker is dropped without explicit completion (e.g., due to panic), it automatically marks the operation as failed.

---

## Health Monitoring

### SystemHealth Structure

```rust
pub struct SystemHealth {
    pub status: HealthStatus,
    pub score: f64,
    pub active_operations: u32,
    pub throughput_mbps: f64,
    pub error_rate_percent: f64,
    pub uptime_seconds: u64,
    pub alerts: Vec<Alert>,
}

pub enum HealthStatus {
    Healthy,   // Score >= 90.0
    Warning,   // Score >= 70.0 && < 90.0
    Critical,  // Score < 70.0
    Unknown,   // Unable to determine health
}
```

### Health Score Calculation

The health score starts at 100 and deductions are applied:

```rust
let mut score = 100.0;

// Deduct for high error rate
if error_rate_percent > max_error_rate_percent {
    score -= 30.0;  // Error rate is critical
}

// Deduct for low throughput
if average_throughput_mbps < min_throughput_mbps {
    score -= 20.0;  // Performance degradation
}

// Additional deductions for other factors...
```

**Health Score Ranges**:
- **100-90**: Healthy - System operating normally
- **89-70**: Warning - Degraded performance, investigation needed
- **69-0**: Critical - System in distress, immediate action required

### Alert Structure

```rust
pub struct Alert {
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: String,
    pub metric_name: String,
    pub current_value: f64,
    pub threshold: f64,
}

pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}
```

---

## Alert Thresholds

### Configuration

```rust
pub struct AlertThresholds {
    pub max_error_rate_percent: f64,
    pub min_throughput_mbps: f64,
    pub max_processing_duration_seconds: f64,
    pub max_memory_usage_mb: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            max_error_rate_percent: 5.0,
            min_throughput_mbps: 1.0,
            max_processing_duration_seconds: 300.0,
            max_memory_usage_mb: 1024.0,
        }
    }
}
```

### Alert Generation

Alerts are generated when thresholds are violated:

```rust
async fn check_alerts(&self, tracker: &PerformanceTracker) {
    // High error rate alert
    if tracker.error_rate_percent > self.alert_thresholds.max_error_rate_percent {
        warn!(
            "🚨 Alert: High error rate {:.1}% (threshold: {:.1}%)",
            tracker.error_rate_percent,
            self.alert_thresholds.max_error_rate_percent
        );
    }

    // Low throughput alert
    if tracker.average_throughput_mbps < self.alert_thresholds.min_throughput_mbps {
        warn!(
            "🚨 Alert: Low throughput {:.2} MB/s (threshold: {:.2} MB/s)",
            tracker.average_throughput_mbps,
            self.alert_thresholds.min_throughput_mbps
        );
    }

    // High concurrent operations alert
    if tracker.active_operations > 10 {
        warn!("🚨 Alert: High concurrent operations: {}", tracker.active_operations);
    }
}
```

---

## Usage Patterns

### Basic Operation Tracking

```rust
// Start operation tracking
let tracker = observability_service
    .start_operation("file_processing")
    .await;

// Do work
let result = process_file(&input_path).await?;

// Complete tracking with success/failure
tracker.complete(true, result.bytes_processed).await;
```

### Automatic Tracking with Drop Safety

```rust
async fn process_pipeline(id: &PipelineId) -> Result<()> {
    // Tracker automatically handles failure if function panics or returns Err
    let tracker = observability_service
        .start_operation("pipeline_execution")
        .await;

    // If this fails, tracker is dropped and marks operation as failed
    let result = execute_stages(id).await?;

    // Explicit success
    tracker.complete(true, result.bytes_processed).await;
    Ok(())
}
```

### Recording Pipeline Metrics

```rust
// After pipeline completion
let metrics = pipeline.processing_metrics();

// Record to both Prometheus and performance tracker
observability_service
    .record_processing_metrics(&metrics)
    .await;

// This automatically:
// - Updates Prometheus counters/gauges/histograms
// - Updates PerformanceTracker state
// - Checks alert thresholds
// - Logs completion with metrics
```

### Health Check Endpoint

```rust
async fn health_check() -> Result<SystemHealth> {
    let health = observability_service.get_system_health().await;

    match health.status {
        HealthStatus::Healthy => {
            info!("System health: HEALTHY (score: {:.1})", health.score);
        }
        HealthStatus::Warning => {
            warn!(
                "System health: WARNING (score: {:.1}, {} alerts)",
                health.score,
                health.alerts.len()
            );
        }
        HealthStatus::Critical => {
            error!(
                "System health: CRITICAL (score: {:.1}, {} alerts)",
                health.score,
                health.alerts.len()
            );
        }
        HealthStatus::Unknown => {
            warn!("System health: UNKNOWN");
        }
    }

    Ok(health)
}
```

### Performance Summary

```rust
// Get human-readable performance summary
let summary = observability_service
    .get_performance_summary()
    .await;

println!("{}", summary);
```

**Output**:
```text
📊 Performance Summary:
Active Operations: 3
Total Operations: 1247
Average Throughput: 45.67 MB/s
Peak Throughput: 89.23 MB/s
Error Rate: 2.1%
System Health: 88.5/100 (Warning)
Alerts: 1
```

---

## Integration with External Systems

### Prometheus Integration

The system exposes metrics via HTTP endpoint:

```rust
// HTTP /metrics endpoint
use axum::{routing::get, Router};

let app = Router::new()
    .route("/metrics", get(metrics_handler));

async fn metrics_handler() -> String {
    metrics_service.get_metrics()
        .unwrap_or_else(|_| "# Error generating metrics\n".to_string())
}
```

**Prometheus Configuration**:
```yaml
scrape_configs:
  - job_name: 'pipeline'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 15s
    scrape_timeout: 10s
```

### Grafana Dashboards

Create dashboards to visualize:
- **Pipeline Throughput**: Line graph of MB/s over time
- **Active Operations**: Gauge of current active count
- **Error Rate**: Line graph of error percentage
- **Processing Duration**: Histogram of completion times
- **System Health**: Gauge with color thresholds

**Example PromQL Queries**:
```promql
# Average throughput over 5 minutes
rate(pipeline_bytes_processed_total[5m]) / 1024 / 1024

# Error rate percentage
100 * (
  rate(pipeline_errors_total[5m]) /
  rate(pipeline_processed_total[5m])
)

# P99 processing duration
histogram_quantile(0.99, pipeline_processing_duration_seconds_bucket)
```

### Log Aggregation

Send logs to external systems:

```rust
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use tracing_appender::{non_blocking, rolling};

// JSON logs for shipping to ELK/Splunk
let file_appender = rolling::daily("./logs", "pipeline.json");
let (non_blocking_appender, _guard) = non_blocking(file_appender);

let file_layer = fmt::layer()
    .with_writer(non_blocking_appender)
    .json()
    .with_target(true)
    .with_thread_ids(true);

let subscriber = Registry::default()
    .with(EnvFilter::new("info"))
    .with(file_layer);

tracing::subscriber::set_global_default(subscriber)?;
```

---

## Performance Considerations

### Low Overhead Design

**Atomic Operations**: Metrics use atomic types to avoid locks:
```rust
pub struct MetricsService {
    pipelines_processed: Arc<AtomicU64>,
    bytes_processed: Arc<AtomicU64>,
    // ...
}
```

**Async RwLock**: PerformanceTracker uses async RwLock for concurrent reads:
```rust
performance_tracker: Arc<RwLock<PerformanceTracker>>
```

**Lazy Evaluation**: Expensive calculations only performed when health is queried

**Compile-Time Filtering**: Debug/trace logs have zero overhead in release builds

### Benchmark Results

Observability overhead on Intel i7-10700K @ 3.8 GHz:

| Operation | Time | Overhead |
|-----------|------|----------|
| `start_operation()` | ~500 ns | Negligible |
| `complete_operation()` | ~1.2 μs | Minimal |
| `record_processing_metrics()` | ~2.5 μs | Low |
| `get_system_health()` | ~8 μs | Moderate (infrequent) |
| `info!()` log | ~80 ns | Negligible |
| `debug!()` log (disabled) | ~0 ns | Zero |

**Total overhead**: < 0.1% of pipeline processing time

---

## Best Practices

### ✅ DO

**Track all significant operations**
```rust
let tracker = observability.start_operation("file_compression").await;
let result = compress_file(&path).await?;
tracker.complete(true, result.compressed_size).await;
```

**Use structured logging**
```rust
info!(
    pipeline_id = %id,
    bytes = total_bytes,
    duration_ms = elapsed.as_millis(),
    "Pipeline completed"
);
```

**Record domain metrics**
```rust
observability.record_processing_metrics(&pipeline.metrics()).await;
```

**Check health regularly**
```rust
// In health check endpoint
let health = observability.get_system_health().await;
```

**Configure thresholds appropriately**
```rust
let observability = ObservabilityService::new_with_config(metrics_service).await;
```

### ❌ DON'T

**Don't track trivial operations**
```rust
// BAD: Too fine-grained
let tracker = observability.start_operation("allocate_vec").await;
let vec = Vec::with_capacity(100);
tracker.complete(true, 0).await; // Overhead > value
```

**Don't log in hot loops without rate limiting**
```rust
// BAD: Excessive logging
for chunk in chunks {
    debug!("Processing chunk {}", chunk.id); // Called millions of times!
}

// GOOD: Log summary
debug!(chunk_count = chunks.len(), "Processing chunks");
info!(chunks_processed = chunks.len(), "Chunk processing complete");
```

**Don't forget to complete trackers**
```rust
// BAD: Leaks active operation count
let tracker = observability.start_operation("process").await;
process().await?;
// Forgot to call tracker.complete()!

// GOOD: Explicit completion
let tracker = observability.start_operation("process").await;
let result = process().await?;
tracker.complete(true, result.bytes).await;
```

**Don't block on observability operations**
```rust
// BAD: Blocking in async context
tokio::task::block_in_place(|| {
    observability.get_system_health().await // Won't compile anyway!
});

// GOOD: Await directly
let health = observability.get_system_health().await;
```

---

## Testing Strategies

### Unit Testing ObservabilityService

```rust
#[tokio::test]
async fn test_operation_tracking() {
    let metrics_service = Arc::new(MetricsService::new().unwrap());
    let observability = ObservabilityService::new(metrics_service);

    // Start operation
    let tracker = observability.start_operation("test").await;

    // Check active count increased
    let health = observability.get_system_health().await;
    assert_eq!(health.active_operations, 1);

    // Complete operation
    tracker.complete(true, 1000).await;

    // Check active count decreased
    let health = observability.get_system_health().await;
    assert_eq!(health.active_operations, 0);
}
```

### Testing Alert Generation

```rust
#[tokio::test]
async fn test_high_error_rate_alert() {
    let metrics_service = Arc::new(MetricsService::new().unwrap());
    let mut observability = ObservabilityService::new(metrics_service);

    // Set low threshold
    observability.alert_thresholds.max_error_rate_percent = 1.0;

    // Simulate high error rate
    for _ in 0..10 {
        let tracker = observability.start_operation("test").await;
        tracker.complete(false, 0).await; // All failures
    }

    // Check health has alerts
    let health = observability.get_system_health().await;
    assert!(!health.alerts.is_empty());
    assert_eq!(health.status, HealthStatus::Critical);
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_end_to_end_observability() {
    // Setup
    let metrics_service = Arc::new(MetricsService::new().unwrap());
    let observability = Arc::new(ObservabilityService::new(metrics_service.clone()));

    // Run pipeline with tracking
    let tracker = observability.start_operation("pipeline").await;
    let result = run_test_pipeline().await.unwrap();
    tracker.complete(true, result.bytes_processed).await;

    // Verify metrics recorded
    let metrics_output = metrics_service.get_metrics().unwrap();
    assert!(metrics_output.contains("pipeline_processed_total"));

    // Verify health is good
    let health = observability.get_system_health().await;
    assert_eq!(health.status, HealthStatus::Healthy);
}
```

---

## Common Issues and Solutions

### Issue: Active operations count stuck

**Symptom**: `active_operations` never decreases

**Cause**: `OperationTracker` not completed or dropped

**Solution**:
```rust
// Ensure tracker is completed in all code paths
let tracker = observability.start_operation("op").await;
let result = match dangerous_operation().await {
    Ok(r) => {
        tracker.complete(true, r.bytes).await;
        Ok(r)
    }
    Err(e) => {
        tracker.complete(false, 0).await;
        Err(e)
    }
};
```

### Issue: Health score always 100

**Symptom**: Health never degrades despite errors

**Cause**: Metrics not being recorded

**Solution**:
```rust
// Always record processing metrics
observability.record_processing_metrics(&metrics).await;
```

### Issue: Alerts not firing

**Symptom**: Thresholds violated but no alerts logged

**Cause**: Log level filtering out WARN messages

**Solution**:
```bash
# Enable WARN level
export RUST_LOG=warn

# Or per-module
export RUST_LOG=pipeline::infrastructure::logging=warn
```

---

## Next Steps

- **[Metrics Collection](metrics.md)**: Deep dive into Prometheus metrics
- **[Logging Implementation](logging.md)**: Structured logging with tracing
- **[Configuration](configuration.md)**: Configure alert thresholds and settings
- **[Testing](../testing/integration-tests.md)**: Integration testing strategies

---

## References

- Source: `pipeline/src/infrastructure/logging/observability_service.rs` (lines 1-716)
- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Dashboards](https://grafana.com/docs/grafana/latest/dashboards/)
- [The Three Pillars of Observability](https://www.oreilly.com/library/view/distributed-systems-observability/9781492033431/ch04.html)
- [Site Reliability Engineering](https://sre.google/sre-book/monitoring-distributed-systems/)
