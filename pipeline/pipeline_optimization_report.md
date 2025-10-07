# Pipeline Optimization Benchmark Report

Generated: 2025-10-07 00:11:11 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 1.00 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 8
- Throughput: 1.51 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 50.9% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 8 | 1.51 | 0.00 | Worker Variation |
| 1 | 7 | 1.47 | 0.00 | Worker Variation |
| 1 | 16 | 1.43 | 0.00 | Worker Variation |
| 1 | 10 | 1.42 | 0.00 | Worker Variation |
| 1 | 13 | 1.40 | 0.00 | Worker Variation |
| 1 | 15 | 1.39 | 0.00 | Worker Variation |
| 1 | 3 | 1.38 | 0.00 | Worker Variation |
| 1 | 6 | 1.37 | 0.00 | Worker Variation |
| 1 | 11 | 1.33 | 0.00 | Worker Variation |
| 1 | 12 | 1.33 | 0.00 | Worker Variation |
| 1 | 2 | 1.30 | 0.00 | Chunk Variation |
| 1 | 14 | 1.30 | 0.00 | Worker Variation |
| 64 | 2 | 1.27 | 0.00 | Chunk Variation |
| 32 | 2 | 1.25 | 0.00 | Chunk Variation |
| 1 | 5 | 1.25 | 0.00 | Worker Variation |
| 2 | 2 | 1.24 | 0.00 | Chunk Variation |
| 16 | 2 | 1.21 | 0.00 | Chunk Variation |
| 1 | 9 | 1.09 | 0.00 | Worker Variation |
| 128 | 2 | 1.08 | 0.00 | Chunk Variation |
| 8 | 2 | 1.06 | 0.00 | Chunk Variation |
| 1 | 2 | 1.00 | 0.00 | Adaptive |
| 4 | 2 | 0.93 | 0.00 | Chunk Variation |
| 1 | 4 | 0.89 | 0.00 | Worker Variation |
| 1 | 1 | 0.88 | 0.00 | Worker Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 8 workers (1.51 MB/s)
