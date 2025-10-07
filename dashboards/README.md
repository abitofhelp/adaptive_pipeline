# Adaptive Pipeline Grafana Dashboards

This directory contains pre-configured Grafana dashboards for monitoring the Adaptive Pipeline Rust application.

## Dashboard Overview

### 1. **Adaptive Pipeline - Overview** (`adaptive-pipeline-overview.json`)
**Primary dashboard for high-level monitoring**
- Pipeline activity and throughput
- Error rate gauge with thresholds
- Stage execution distribution
- Processing duration percentiles
- Real-time performance metrics

### 2. **Pipeline Performance Details** (`pipeline-performance-details.json`)
**Detailed performance analysis**
- Stage-by-stage processing times (compression, encryption, checksum)
- Compression efficiency tracking
- Data volume and throughput trends
- Error and warning analysis
- Processing configuration monitoring

### 3. **System Health Monitoring** (`system-health-monitoring.json`)
**System health and operational status**
- Service availability status
- System health score gauge
- Operation load monitoring
- Error rate and throughput thresholds
- Active alerts dashboard

## Prerequisites

1. **Prometheus Server** running on `localhost:9090`
2. **Grafana Server** running on `localhost:3000`
3. **Adaptive Pipeline Application** exposing metrics on port `9091`

## Setup Instructions

### 1. Configure Prometheus Data Source in Grafana

1. Open Grafana at `http://localhost:3000`
2. Go to **Configuration** → **Data Sources**
3. Add **Prometheus** data source:
   - **Name**: `prometheus`
   - **URL**: `http://localhost:9090`
   - **Access**: `Server (default)`
4. Click **Save & Test**

### 2. Import Dashboards

For each dashboard JSON file:

1. In Grafana, go to **+** → **Import**
2. Click **Upload JSON file** or copy/paste the JSON content
3. Configure the dashboard:
   - **Name**: Keep the default or customize
   - **Folder**: Choose appropriate folder
   - **UID**: Keep the default unique identifier
4. Click **Import**

### 3. Prometheus Configuration

Ensure your `prometheus.yml` includes the adaptive pipeline target:

```yaml
scrape_configs:
  - job_name: 'adaptive-pipeline'
    static_configs:
      - targets: ['localhost:9091']
    scrape_interval: 30s
    metrics_path: '/metrics'
```

## Key Metrics Monitored

### Pipeline Metrics
- `adaptive_pipeline_active_pipelines` - Currently active pipeline processes
- `adaptive_pipeline_files_processed_total` - Total files processed counter
- `adaptive_pipeline_processing_errors_total` - Total processing errors
- `adaptive_pipeline_processing_warnings_total` - Total warnings
- `adaptive_pipeline_throughput_mbps` - Current throughput in MB/s
- `adaptive_pipeline_peak_throughput_mbps` - Peak throughput achieved

### Stage Metrics
- `adaptive_pipeline_stage_executions_total{stage_type}` - Executions per stage type
- `adaptive_pipeline_stage_duration_seconds{stage_type}` - Processing time per stage
- `adaptive_pipeline_compression_ratio` - Compression efficiency percentage

### System Health Metrics
- `adaptive_pipeline_system_health_score` - Overall system health (0-100)
- `adaptive_pipeline_error_rate_percent` - Current error rate percentage
- `adaptive_pipeline_active_operations` - Currently active operations
- `adaptive_pipeline_total_operations` - Total operations counter

### Performance Metrics
- `adaptive_pipeline_processing_duration_seconds_bucket` - Processing time histogram
- `adaptive_pipeline_bytes_processed_total` - Total bytes processed
- `adaptive_pipeline_chunk_size_bytes` - Current chunk size configuration
- `adaptive_pipeline_worker_count` - Number of worker threads

## Alert Thresholds

The dashboards include visual thresholds based on the `observability.toml` configuration:

- **Error Rate**: Warning at 5%, Critical at 10%
- **Throughput**: Warning below 1.0 MB/s
- **Health Score**: Warning below 70%, Critical below 50%
- **Processing Duration**: Warning above 60s, Critical above 300s

## Customization

### Adding Custom Panels

1. Edit any dashboard in Grafana
2. Click **Add Panel**
3. Use PromQL queries with available metrics
4. Configure visualization type and thresholds

### Example Custom Queries

```promql
# Average processing time by pipeline
avg(adaptive_pipeline_processing_duration_seconds) by (pipeline_name)

# Error rate over time
rate(adaptive_pipeline_processing_errors_total[5m]) / rate(adaptive_pipeline_files_processed_total[5m]) * 100

# Compression efficiency trend
avg_over_time(adaptive_pipeline_compression_ratio[1h])

# Peak vs current throughput comparison
adaptive_pipeline_peak_throughput_mbps - adaptive_pipeline_throughput_mbps
```

## Troubleshooting

### No Data Appearing
1. Verify Prometheus is scraping the adaptive pipeline endpoint
2. Check that the application is running and exposing metrics on port 9091
3. Confirm Grafana data source configuration

### Missing Metrics
1. Ensure the application has processed at least one file to generate metrics
2. Check the `/metrics` endpoint directly: `curl http://localhost:9091/metrics`
3. Verify metric names match those in the dashboard queries

### Performance Issues
1. Adjust dashboard refresh rate (default: 30s)
2. Reduce time range for heavy queries
3. Use recording rules in Prometheus for complex calculations

## Dashboard Maintenance

- **Refresh Rate**: 30 seconds (configurable per dashboard)
- **Time Range**: Default 1 hour (adjustable)
- **Auto-refresh**: Enabled for real-time monitoring
- **Variables**: Can be added for filtering by pipeline name, stage type, etc.

## Integration with Alerting

These dashboards work with Grafana alerting rules. Consider setting up alerts for:
- High error rates
- Low throughput
- System health degradation
- Processing timeouts

For production environments, integrate with notification channels (Slack, email, PagerDuty) for critical alerts.
