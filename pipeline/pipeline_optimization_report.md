# Pipeline Optimization Benchmark Report

Generated: 2025-10-07 04:06:41 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 0.72 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 16
- Throughput: 1.50 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 110.0% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 16 | 1.50 | 0.00 | Worker Variation |
| 1 | 13 | 1.43 | 0.00 | Worker Variation |
| 1 | 10 | 1.42 | 0.00 | Worker Variation |
| 1 | 12 | 1.39 | 0.00 | Worker Variation |
| 1 | 1 | 1.39 | 0.00 | Worker Variation |
| 1 | 8 | 1.38 | 0.00 | Worker Variation |
| 1 | 9 | 1.38 | 0.00 | Worker Variation |
| 1 | 15 | 1.38 | 0.00 | Worker Variation |
| 1 | 11 | 1.36 | 0.00 | Worker Variation |
| 2 | 2 | 1.27 | 0.00 | Chunk Variation |
| 1 | 2 | 1.27 | 0.00 | Chunk Variation |
| 32 | 2 | 1.27 | 0.00 | Chunk Variation |
| 1 | 5 | 1.26 | 0.00 | Worker Variation |
| 1 | 4 | 1.23 | 0.00 | Worker Variation |
| 4 | 2 | 1.19 | 0.00 | Chunk Variation |
| 64 | 2 | 1.17 | 0.00 | Chunk Variation |
| 128 | 2 | 1.09 | 0.00 | Chunk Variation |
| 1 | 14 | 1.06 | 0.00 | Worker Variation |
| 16 | 2 | 1.04 | 0.00 | Chunk Variation |
| 1 | 7 | 0.98 | 0.00 | Worker Variation |
| 1 | 6 | 0.96 | 0.00 | Worker Variation |
| 8 | 2 | 0.95 | 0.00 | Chunk Variation |
| 1 | 3 | 0.95 | 0.00 | Worker Variation |
| 1 | 2 | 0.72 | 0.00 | Adaptive |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 16 workers (1.50 MB/s)
