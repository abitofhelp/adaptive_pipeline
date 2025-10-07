# Pipeline Optimization Benchmark Report

Generated: 2025-10-07 00:57:46 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 0.99 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 9
- Throughput: 1.45 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 46.8% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 9 | 1.45 | 0.00 | Worker Variation |
| 1 | 14 | 1.43 | 0.00 | Worker Variation |
| 1 | 7 | 1.41 | 0.00 | Worker Variation |
| 32 | 2 | 1.40 | 0.00 | Chunk Variation |
| 1 | 15 | 1.40 | 0.00 | Worker Variation |
| 16 | 2 | 1.37 | 0.00 | Chunk Variation |
| 1 | 13 | 1.37 | 0.00 | Worker Variation |
| 1 | 11 | 1.36 | 0.00 | Worker Variation |
| 1 | 12 | 1.36 | 0.00 | Worker Variation |
| 1 | 4 | 1.34 | 0.00 | Worker Variation |
| 1 | 3 | 1.27 | 0.00 | Worker Variation |
| 1 | 5 | 1.26 | 0.00 | Worker Variation |
| 8 | 2 | 1.23 | 0.00 | Chunk Variation |
| 1 | 16 | 1.19 | 0.00 | Worker Variation |
| 64 | 2 | 1.16 | 0.00 | Chunk Variation |
| 1 | 6 | 1.11 | 0.00 | Worker Variation |
| 1 | 8 | 1.07 | 0.00 | Worker Variation |
| 1 | 10 | 1.04 | 0.00 | Worker Variation |
| 128 | 2 | 1.01 | 0.00 | Chunk Variation |
| 1 | 2 | 1.00 | 0.00 | Chunk Variation |
| 1 | 2 | 0.99 | 0.00 | Adaptive |
| 1 | 1 | 0.98 | 0.00 | Worker Variation |
| 2 | 2 | 0.72 | 0.00 | Chunk Variation |
| 4 | 2 | 0.38 | 0.01 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 9 workers (1.45 MB/s)
