# 🧪 Complete Observability Integration Test

## Your Perfect Setup

Your Docker Compose stack is already optimally configured:

```yaml
# ✅ Prometheus (port 9090) with prometheus.yml mounted
# ✅ Grafana (port 3000) with grafana.ini mounted  
# ✅ Network bridge for container communication
# ✅ Persistent volumes for data retention
```

## 🔄 Integration Test Steps

### 1. **Start Your Docker Stack**
```bash
# Start all services (if not already running)
docker-compose -f docker-compose.yml up -d

# Verify services are running
docker-compose ps
```

### 2. **Start the Adaptive Pipeline**
```bash
cd pipeline

# Start the application (metrics endpoint will start on port 9091)
cargo run --bin pipeline -- list
```

### 3. **Verify Metrics Flow**

**Check Prometheus Targets:**
```bash
# Open Prometheus UI
open http://localhost:9090

# Go to Status → Targets
# Verify 'adaptive-pipeline' target is UP
```

**Check Raw Metrics:**
```bash
# Verify metrics endpoint is working
curl http://localhost:9091/metrics

# Should see metrics like:
# adaptive_pipeline_active_pipelines 0
# adaptive_pipeline_files_processed_total 0
# adaptive_pipeline_system_health_score 100
```

### 4. **Import Grafana Dashboards**
```bash
# Open Grafana
open http://localhost:3000

# Import each dashboard:
# 1. + → Import → Upload JSON file
# 2. Select: dashboards/adaptive-pipeline-overview.json
# 3. Repeat for: pipeline-performance-details.json
# 4. Repeat for: system-health-monitoring.json
```

### 5. **Generate Test Metrics**
```bash
# Create a test pipeline
cargo run --bin pipeline -- create \
  --name "test-observability" \
  --stages "compression:brotli,encryption:aes256"

# Process a test file to generate metrics
echo "Test data for observability" > /tmp/test.txt
cargo run --bin pipeline -- process \
  --input /tmp/test.txt \
  --output-dir /tmp \
  --pipeline test-observability
```

### 6. **Verify Dashboard Data**
- **Overview Dashboard**: Should show active pipelines, throughput
- **Performance Dashboard**: Should show stage timings, compression ratios  
- **Health Dashboard**: Should show system health score, operation counts

## 🎯 Expected Results

### Prometheus Metrics (http://localhost:9091/metrics)
```
# HELP adaptive_pipeline_active_pipelines Currently active pipeline processes
# TYPE adaptive_pipeline_active_pipelines gauge
adaptive_pipeline_active_pipelines 1

# HELP adaptive_pipeline_files_processed_total Total files processed
# TYPE adaptive_pipeline_files_processed_total counter
adaptive_pipeline_files_processed_total 1

# HELP adaptive_pipeline_throughput_mbps Current throughput in MB/s
# TYPE adaptive_pipeline_throughput_mbps gauge
adaptive_pipeline_throughput_mbps 2.5

# HELP adaptive_pipeline_system_health_score Overall system health (0-100)
# TYPE adaptive_pipeline_system_health_score gauge
adaptive_pipeline_system_health_score 100
```

### Grafana Dashboards
- **Real-time updates** during file processing
- **Stage-by-stage metrics** for compression and encryption
- **Performance indicators** showing throughput and duration
- **Health monitoring** with configurable thresholds

## 🔧 Configuration Verification

### Your prometheus.yml (Already Perfect)
```yaml
- job_name: 'adaptive-pipeline'
  static_configs:
    - targets: ['host.docker.internal:9091']  # ✅ Correct for Docker
```

### Your grafana.ini (Already Perfect)
```yaml
datasources:
  - name: Prometheus
    url: http://prometheus:9090  # ✅ Correct for Docker network
```

### Your observability.toml (Already Perfect)
```toml
[metrics]
port = 9091  # ✅ Matches Prometheus target

[health_checks]
error_rate_threshold_percent = 5.0  # ✅ Dashboard thresholds
throughput_threshold_mbps = 1.0     # ✅ Alert levels
```

## 🎉 Success Indicators

✅ **Prometheus Target UP**: adaptive-pipeline shows as healthy  
✅ **Metrics Flowing**: /metrics endpoint returns data  
✅ **Grafana Connected**: Dashboards show real-time data  
✅ **Operation Tracking**: Active operations increment during processing  
✅ **Performance Data**: Throughput, duration, stage metrics visible  
✅ **Health Monitoring**: System health score and error rates tracked  

## 🚨 Troubleshooting

### If Prometheus Can't Reach Pipeline
```bash
# Check if port 9091 is accessible from Docker
docker run --rm --network host curlimages/curl curl http://host.docker.internal:9091/metrics
```

### If No Data in Grafana
1. Process at least one file to generate metrics
2. Check time range (last 1 hour)
3. Verify Prometheus data source connection

### If Dashboards Don't Import
1. Ensure Prometheus data source is named exactly "prometheus"
2. Check that UID conflicts don't exist
3. Import one dashboard at a time

## 🎊 You're Ready!

Your infrastructure is perfectly set up. The integration should work flawlessly with:

- **Docker Compose**: Managing Prometheus + Grafana
- **Adaptive Pipeline**: Exposing metrics on host port 9091
- **Network Configuration**: Docker containers reaching host via `host.docker.internal`
- **Dashboards**: Pre-configured for your exact setup

**No changes needed to your Docker Compose file - it's already optimal!** 🚀
