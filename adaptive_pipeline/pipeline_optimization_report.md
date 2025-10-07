# Pipeline Optimization Benchmark Report

Generated: 2025-10-07 23:55:22 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 1.12 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 64 MB
- Worker Count: 2
- Throughput: 3.20 MB/s
- Duration: 0.00 seconds
- Configuration Type: Chunk Variation

**Performance Improvement:** 185.0% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 64 | 2 | 3.20 | 0.00 | Chunk Variation |
| 1 | 15 | 1.40 | 0.00 | Worker Variation |
| 1 | 10 | 1.40 | 0.00 | Worker Variation |
| 1 | 11 | 1.38 | 0.00 | Worker Variation |
| 1 | 13 | 1.38 | 0.00 | Worker Variation |
| 1 | 1 | 1.37 | 0.00 | Worker Variation |
| 1 | 16 | 1.36 | 0.00 | Worker Variation |
| 1 | 4 | 1.36 | 0.00 | Worker Variation |
| 1 | 6 | 1.36 | 0.00 | Worker Variation |
| 1 | 12 | 1.35 | 0.00 | Worker Variation |
| 1 | 14 | 1.34 | 0.00 | Worker Variation |
| 16 | 2 | 1.30 | 0.00 | Chunk Variation |
| 32 | 2 | 1.20 | 0.00 | Chunk Variation |
| 1 | 2 | 1.12 | 0.00 | Adaptive |
| 128 | 2 | 1.10 | 0.00 | Chunk Variation |
| 4 | 2 | 1.07 | 0.00 | Chunk Variation |
| 1 | 2 | 1.07 | 0.00 | Chunk Variation |
| 1 | 9 | 1.06 | 0.00 | Worker Variation |
| 1 | 5 | 1.04 | 0.00 | Worker Variation |
| 1 | 8 | 1.03 | 0.00 | Worker Variation |
| 1 | 3 | 1.02 | 0.00 | Worker Variation |
| 2 | 2 | 1.00 | 0.00 | Chunk Variation |
| 1 | 7 | 0.97 | 0.00 | Worker Variation |
| 8 | 2 | 0.28 | 0.01 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 64 MB chunks, 2 workers (3.20 MB/s)
