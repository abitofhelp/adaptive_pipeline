# Pipeline Optimization Benchmark Report

Generated: 2025-10-07 02:37:33 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 0.99 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 12
- Throughput: 2.57 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 160.3% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 12 | 2.57 | 0.00 | Worker Variation |
| 1 | 16 | 1.48 | 0.00 | Worker Variation |
| 1 | 10 | 1.46 | 0.00 | Worker Variation |
| 1 | 14 | 1.40 | 0.00 | Worker Variation |
| 1 | 8 | 1.38 | 0.00 | Worker Variation |
| 1 | 11 | 1.38 | 0.00 | Worker Variation |
| 1 | 15 | 1.36 | 0.00 | Worker Variation |
| 1 | 9 | 1.33 | 0.00 | Worker Variation |
| 1 | 5 | 1.32 | 0.00 | Worker Variation |
| 1 | 2 | 1.31 | 0.00 | Chunk Variation |
| 1 | 4 | 1.27 | 0.00 | Worker Variation |
| 32 | 2 | 1.25 | 0.00 | Chunk Variation |
| 64 | 2 | 1.18 | 0.00 | Chunk Variation |
| 2 | 2 | 1.15 | 0.00 | Chunk Variation |
| 16 | 2 | 1.15 | 0.00 | Chunk Variation |
| 1 | 7 | 1.07 | 0.00 | Worker Variation |
| 8 | 2 | 1.07 | 0.00 | Chunk Variation |
| 1 | 13 | 1.05 | 0.00 | Worker Variation |
| 128 | 2 | 1.03 | 0.00 | Chunk Variation |
| 1 | 2 | 0.99 | 0.00 | Adaptive |
| 1 | 3 | 0.96 | 0.00 | Worker Variation |
| 1 | 6 | 0.92 | 0.00 | Worker Variation |
| 1 | 1 | 0.92 | 0.00 | Worker Variation |
| 4 | 2 | 0.80 | 0.00 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 12 workers (2.57 MB/s)
