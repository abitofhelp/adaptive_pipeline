# Pipeline Optimization Benchmark Report

Generated: 2025-10-08 02:22:48 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 0.87 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 14
- Throughput: 2.83 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 225.9% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 14 | 2.83 | 0.00 | Worker Variation |
| 1 | 7 | 1.49 | 0.00 | Worker Variation |
| 1 | 16 | 1.44 | 0.00 | Worker Variation |
| 1 | 12 | 1.41 | 0.00 | Worker Variation |
| 1 | 6 | 1.40 | 0.00 | Worker Variation |
| 1 | 15 | 1.40 | 0.00 | Worker Variation |
| 1 | 10 | 1.40 | 0.00 | Worker Variation |
| 1 | 8 | 1.39 | 0.00 | Worker Variation |
| 1 | 1 | 1.39 | 0.00 | Worker Variation |
| 1 | 13 | 1.37 | 0.00 | Worker Variation |
| 1 | 11 | 1.28 | 0.00 | Worker Variation |
| 4 | 2 | 1.28 | 0.00 | Chunk Variation |
| 1 | 3 | 1.27 | 0.00 | Worker Variation |
| 32 | 2 | 1.26 | 0.00 | Chunk Variation |
| 16 | 2 | 1.26 | 0.00 | Chunk Variation |
| 1 | 2 | 1.25 | 0.00 | Chunk Variation |
| 64 | 2 | 1.19 | 0.00 | Chunk Variation |
| 1 | 9 | 1.11 | 0.00 | Worker Variation |
| 128 | 2 | 0.99 | 0.00 | Chunk Variation |
| 1 | 4 | 0.99 | 0.00 | Worker Variation |
| 1 | 5 | 0.90 | 0.00 | Worker Variation |
| 1 | 2 | 0.87 | 0.00 | Adaptive |
| 2 | 2 | 0.72 | 0.00 | Chunk Variation |
| 8 | 2 | 0.24 | 0.01 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 14 workers (2.83 MB/s)
