# Pipeline Optimization Benchmark Report

Generated: 2025-10-06 23:08:26 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 1.15 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 7
- Throughput: 6.90 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 499.7% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 7 | 6.90 | 0.00 | Worker Variation |
| 1 | 16 | 6.40 | 0.00 | Worker Variation |
| 1 | 14 | 1.44 | 0.00 | Worker Variation |
| 1 | 1 | 1.43 | 0.00 | Worker Variation |
| 1 | 11 | 1.43 | 0.00 | Worker Variation |
| 1 | 6 | 1.42 | 0.00 | Worker Variation |
| 1 | 8 | 1.42 | 0.00 | Worker Variation |
| 1 | 15 | 1.39 | 0.00 | Worker Variation |
| 1 | 5 | 1.36 | 0.00 | Worker Variation |
| 1 | 13 | 1.35 | 0.00 | Worker Variation |
| 16 | 2 | 1.35 | 0.00 | Chunk Variation |
| 32 | 2 | 1.27 | 0.00 | Chunk Variation |
| 2 | 2 | 1.24 | 0.00 | Chunk Variation |
| 64 | 2 | 1.20 | 0.00 | Chunk Variation |
| 1 | 2 | 1.19 | 0.00 | Chunk Variation |
| 1 | 2 | 1.15 | 0.00 | Adaptive |
| 4 | 2 | 1.11 | 0.00 | Chunk Variation |
| 1 | 10 | 1.08 | 0.00 | Worker Variation |
| 1 | 12 | 1.08 | 0.00 | Worker Variation |
| 128 | 2 | 1.06 | 0.00 | Chunk Variation |
| 1 | 9 | 1.02 | 0.00 | Worker Variation |
| 1 | 3 | 0.92 | 0.00 | Worker Variation |
| 1 | 4 | 0.87 | 0.00 | Worker Variation |
| 8 | 2 | 0.26 | 0.01 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 7 workers (6.90 MB/s)
