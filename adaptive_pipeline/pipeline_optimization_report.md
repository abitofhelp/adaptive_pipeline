# Pipeline Optimization Benchmark Report

Generated: 2025-10-07 23:26:54 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 0.84 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 6
- Throughput: 1.47 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 74.3% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 6 | 1.47 | 0.00 | Worker Variation |
| 1 | 15 | 1.42 | 0.00 | Worker Variation |
| 1 | 11 | 1.41 | 0.00 | Worker Variation |
| 1 | 12 | 1.41 | 0.00 | Worker Variation |
| 1 | 10 | 1.41 | 0.00 | Worker Variation |
| 1 | 14 | 1.40 | 0.00 | Worker Variation |
| 1 | 16 | 1.40 | 0.00 | Worker Variation |
| 1 | 13 | 1.38 | 0.00 | Worker Variation |
| 1 | 3 | 1.38 | 0.00 | Worker Variation |
| 1 | 1 | 1.37 | 0.00 | Worker Variation |
| 16 | 2 | 1.34 | 0.00 | Chunk Variation |
| 32 | 2 | 1.24 | 0.00 | Chunk Variation |
| 8 | 2 | 1.10 | 0.00 | Chunk Variation |
| 64 | 2 | 1.09 | 0.00 | Chunk Variation |
| 128 | 2 | 1.06 | 0.00 | Chunk Variation |
| 1 | 7 | 1.05 | 0.00 | Worker Variation |
| 1 | 2 | 1.05 | 0.00 | Chunk Variation |
| 1 | 9 | 1.01 | 0.00 | Worker Variation |
| 1 | 4 | 1.01 | 0.00 | Worker Variation |
| 1 | 5 | 0.99 | 0.00 | Worker Variation |
| 1 | 8 | 0.99 | 0.00 | Worker Variation |
| 2 | 2 | 0.87 | 0.00 | Chunk Variation |
| 1 | 2 | 0.84 | 0.00 | Adaptive |
| 4 | 2 | 0.42 | 0.00 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 6 workers (1.47 MB/s)
