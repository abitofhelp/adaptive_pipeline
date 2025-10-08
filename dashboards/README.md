<!--
Adaptive Pipeline
Copyright (c) 2025 Michael Gardner, A Bit of Help, Inc.
SPDX-License-Identifier: BSD-3-Clause
See LICENSE file in the project root.
-->

# Adaptive Pipeline Grafana Dashboard

This directory contains a production-ready Grafana dashboard for monitoring the Adaptive Pipeline Rust application.

## Dashboard: Adaptive Pipeline Monitoring

**File:** `adaptive-pipeline-monitoring.json`

A comprehensive monitoring dashboard organized into four key sections:

### 1. **Overview - Key Metrics**
High-level system status at a glance:
- **Active Pipelines** - Currently running pipeline processes (gauge with thresholds)
- **Total Pipelines Processed** - Cumulative count of completed pipelines
- **Current Throughput** - Real-time MB/s processing speed
- **Total Errors** - Cumulative error count (with warning/critical thresholds)

### 2. **Performance Metrics**
Detailed performance analysis:
- **Processing Duration (Percentiles)** - P50/P95/P99 latency tracking
  - Green line (P50): Median performance
  - Yellow line (P95): 95th percentile - catches most outliers
  - Red line (P99): Tail latencies - shows worst-case performance
- **Throughput Over Time** - MB/s trend graph with mean/max statistics
- **Compression Ratio** - Gauge showing compression efficiency (0.0-1.0)
- **Pipeline Processing Rate** - Pipelines completed per second

### 3. **Data Volume**
Track data processing volume:
- **Total Bytes Processed** - Cumulative bytes (displayed in human-readable units)
- **Total Chunks Processed** - Cumulative chunk count
- **Byte Processing Rate** - Bytes/second throughput graph

### 4. **Error Tracking**
Monitor reliability and error rates:
- **Total Errors** - Cumulative error count
- **Total Warnings** - Cumulative warning count
- **Error Rate Percentage** - Calculated: `(errors / chunks) * 100`
  - Thresholds: Green < 5%, Yellow < 10%, Red >= 10%

## Prerequisites

1. **Prometheus Server** running on `localhost:9090`
2. **Grafana Server** running on `localhost:3000`
3. **Adaptive Pipeline Application** exposing metrics on port `9091`

## Quick Setup

### 1. Start Prometheus

Create or update `prometheus.yml`:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'adaptive-pipeline'
    static_configs:
      - targets: ['localhost:9091']
    scrape_interval: 10s
    metrics_path: '/metrics'
```

Start Prometheus:
```bash
prometheus --config.file=prometheus.yml
```

### 2. Configure Grafana

1. Open Grafana at `http://localhost:3000` (default: admin/admin)
2. Go to **Configuration** → **Data Sources** → **Add data source**
3. Select **Prometheus**
4. Configure:
   - **Name**: `prometheus`
   - **URL**: `http://localhost:9090`
   - **Access**: `Server (default)`
5. Click **Save & Test** (should show "Data source is working")

### 3. Import Dashboard

1. In Grafana, click **+** → **Import**
2. Click **Upload JSON file**
3. Select `adaptive-pipeline-monitoring.json`
4. Select the `prometheus` data source from the dropdown
5. Click **Import**

### 4. Run the Pipeline

The dashboard will populate as soon as the pipeline starts processing:

```bash
# Process a file to generate metrics
adaptive_pipeline process \
  --input test.txt \
  --output test.adapipe \
  --pipeline your_pipeline_name
```

Visit the Prometheus metrics endpoint to verify:
```bash
curl http://localhost:9091/metrics | grep adaptive_pipeline
```

## Available Metrics

The dashboard monitors these Prometheus metrics exported by the pipeline:

| Metric | Type | Description |
|--------|------|-------------|
| `adaptive_pipeline_pipeline_active_count` | Gauge | Currently active pipeline processes |
| `adaptive_pipeline_pipeline_processed_total` | Counter | Total pipelines completed |
| `adaptive_pipeline_pipeline_processing_duration_seconds` | Histogram | Processing time distribution (buckets: 0.1s to 300s) |
| `adaptive_pipeline_pipeline_bytes_processed_total` | Counter | Total bytes processed across all pipelines |
| `adaptive_pipeline_pipeline_chunks_processed_total` | Counter | Total chunks processed |
| `adaptive_pipeline_pipeline_errors_total` | Counter | Total processing errors encountered |
| `adaptive_pipeline_pipeline_warnings_total` | Counter | Total warnings generated |
| `adaptive_pipeline_pipeline_throughput_mbps` | Gauge | Current throughput in MB/s |
| `adaptive_pipeline_pipeline_compression_ratio` | Gauge | Current compression ratio (0.0-1.0) |

## Dashboard Features

### Auto-Refresh
- Default: 30 seconds
- Configurable via dashboard settings (top-right)

### Time Range
- Default: Last 1 hour
- Adjustable via time picker (top-right)

### Thresholds & Colors
- **Active Pipelines**: Yellow at 1, Red at 5
- **Errors**: Yellow at 1, Red at 10
- **Throughput**: Context-dependent (100+ MB/s = Yellow, 500+ = Red on gauge)
- **Error Rate %**: Yellow at 5%, Red at 10%

### Queries & Calculations

The dashboard uses PromQL for advanced calculations:

```promql
# P95 latency (95% of requests faster than this)
histogram_quantile(0.95, rate(adaptive_pipeline_pipeline_processing_duration_seconds_bucket[5m]))

# Error rate percentage
(adaptive_pipeline_pipeline_errors_total / (adaptive_pipeline_pipeline_chunks_processed_total + 1)) * 100

# Pipeline processing rate (pipelines/sec)
rate(adaptive_pipeline_pipeline_processed_total[5m])

# Byte processing rate (bytes/sec)
rate(adaptive_pipeline_pipeline_bytes_processed_total[1m])
```

## Customization

### Adding Custom Panels

1. Click **Add panel** in the dashboard
2. Write a PromQL query using available metrics
3. Choose visualization type (Time series, Stat, Gauge, etc.)
4. Configure thresholds and display options

### Example Custom Queries

```promql
# Average processing time over 15 minutes
avg_over_time(adaptive_pipeline_pipeline_processing_duration_seconds[15m])

# Chunks per second rate
rate(adaptive_pipeline_pipeline_chunks_processed_total[30s])

# Warning-to-error ratio
adaptive_pipeline_pipeline_warnings_total / (adaptive_pipeline_pipeline_errors_total + 1)
```

## Troubleshooting

### Dashboard Shows "No Data"

1. **Verify Prometheus is scraping**:
   ```bash
   # Check Prometheus targets
   curl http://localhost:9090/api/v1/targets | jq
   ```

2. **Check pipeline metrics endpoint**:
   ```bash
   curl http://localhost:9091/metrics
   ```

3. **Ensure pipeline has processed at least one file** - metrics are created on first use

4. **Check Grafana data source** - Test connection in Data Sources settings

### Metrics Not Updating

1. **Check scrape interval** - Default is 10-15 seconds
2. **Verify time range** - Dashboard shows last 1 hour by default
3. **Refresh dashboard** - Click refresh icon or wait for auto-refresh (30s)

### High Error Rates

If error rate is high:
1. Check pipeline logs for details
2. Verify input file format and integrity
3. Review pipeline configuration
4. Check system resources (memory, disk space)

## Production Considerations

### Alerting

Set up Grafana alerts for critical conditions:

1. Go to a panel → **Alert** tab
2. Create alert rules, for example:
   - Error rate > 10% for 5 minutes
   - Throughput < 10 MB/s for 2 minutes
   - Active pipelines > 10 (potential bottleneck)

3. Configure notification channels:
   - Email
   - Slack
   - PagerDuty
   - Webhook

### Data Retention

Configure Prometheus retention:

```yaml
# prometheus.yml
global:
  # Keep metrics for 30 days
  retention: 30d
```

### Performance

For high-volume deployments:

1. **Adjust scrape intervals** - Balance freshness vs. load
2. **Use recording rules** - Pre-calculate expensive queries
3. **Increase dashboard refresh** - 1-5 minutes for non-critical views

### Security

Production checklist:
- Enable Grafana authentication
- Use HTTPS for Grafana and Prometheus
- Restrict network access to metrics endpoint
- Consider Prometheus authentication if exposed

## Learn More

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Dashboard Best Practices](https://grafana.com/docs/grafana/latest/dashboards/build-dashboards/best-practices/)
- [PromQL Basics](https://prometheus.io/docs/prometheus/latest/querying/basics/)
- Adaptive Pipeline Documentation: https://abitofhelp.github.io/adaptive_pipeline/

## Support

- Report issues: [GitHub Issues](https://github.com/abitofhelp/adaptive_pipeline/issues)
- Documentation: [User Guide](https://abitofhelp.github.io/adaptive_pipeline/)
- Advanced topics: [Developer Guide](https://abitofhelp.github.io/adaptive_pipeline/developer/)
