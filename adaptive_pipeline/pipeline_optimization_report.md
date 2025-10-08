# Pipeline Optimization Benchmark Report

Generated: 2025-10-08 00:39:52 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 0.82 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 11
- Throughput: 1.45 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 76.5% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 11 | 1.45 | 0.00 | Worker Variation |
| 1 | 15 | 1.40 | 0.00 | Worker Variation |
| 1 | 10 | 1.39 | 0.00 | Worker Variation |
| 1 | 14 | 1.38 | 0.00 | Worker Variation |
| 1 | 3 | 1.33 | 0.00 | Worker Variation |
| 32 | 2 | 1.32 | 0.00 | Chunk Variation |
| 1 | 12 | 1.29 | 0.00 | Worker Variation |
| 1 | 1 | 1.29 | 0.00 | Worker Variation |
| 1 | 4 | 1.27 | 0.00 | Worker Variation |
| 2 | 2 | 1.25 | 0.00 | Chunk Variation |
| 8 | 2 | 1.21 | 0.00 | Chunk Variation |
| 64 | 2 | 1.20 | 0.00 | Chunk Variation |
| 1 | 2 | 1.15 | 0.00 | Chunk Variation |
| 1 | 13 | 1.08 | 0.00 | Worker Variation |
| 16 | 2 | 1.07 | 0.00 | Chunk Variation |
| 128 | 2 | 1.06 | 0.00 | Chunk Variation |
| 1 | 5 | 1.02 | 0.00 | Worker Variation |
| 1 | 16 | 1.01 | 0.00 | Worker Variation |
| 1 | 9 | 1.00 | 0.00 | Worker Variation |
| 1 | 8 | 0.99 | 0.00 | Worker Variation |
| 1 | 7 | 0.97 | 0.00 | Worker Variation |
| 1 | 6 | 0.87 | 0.00 | Worker Variation |
| 1 | 2 | 0.82 | 0.00 | Adaptive |
| 4 | 2 | 0.36 | 0.01 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 11 workers (1.45 MB/s)
