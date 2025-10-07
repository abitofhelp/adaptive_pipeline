# Pipeline Optimization Benchmark Report

Generated: 2025-10-07 03:27:54 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 1.05 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 15
- Throughput: 5.78 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 452.4% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 15 | 5.78 | 0.00 | Worker Variation |
| 1 | 4 | 1.45 | 0.00 | Worker Variation |
| 1 | 8 | 1.42 | 0.00 | Worker Variation |
| 1 | 10 | 1.42 | 0.00 | Worker Variation |
| 32 | 2 | 1.41 | 0.00 | Chunk Variation |
| 1 | 6 | 1.40 | 0.00 | Worker Variation |
| 1 | 9 | 1.39 | 0.00 | Worker Variation |
| 1 | 11 | 1.38 | 0.00 | Worker Variation |
| 1 | 13 | 1.38 | 0.00 | Worker Variation |
| 1 | 14 | 1.37 | 0.00 | Worker Variation |
| 1 | 16 | 1.37 | 0.00 | Worker Variation |
| 1 | 3 | 1.30 | 0.00 | Worker Variation |
| 1 | 1 | 1.29 | 0.00 | Worker Variation |
| 2 | 2 | 1.25 | 0.00 | Chunk Variation |
| 1 | 2 | 1.25 | 0.00 | Chunk Variation |
| 16 | 2 | 1.18 | 0.00 | Chunk Variation |
| 64 | 2 | 1.05 | 0.00 | Chunk Variation |
| 1 | 2 | 1.05 | 0.00 | Adaptive |
| 1 | 7 | 1.04 | 0.00 | Worker Variation |
| 1 | 12 | 1.03 | 0.00 | Worker Variation |
| 128 | 2 | 0.98 | 0.00 | Chunk Variation |
| 1 | 5 | 0.77 | 0.00 | Worker Variation |
| 4 | 2 | 0.45 | 0.00 | Chunk Variation |
| 8 | 2 | 0.25 | 0.01 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 15 workers (5.78 MB/s)
