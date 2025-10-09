# Pipeline Optimization Benchmark Report

Generated: 2025-10-09 03:04:33 UTC

## File Size: 1 MB

**Adaptive Configuration:**
- Chunk Size: 1 MB
- Worker Count: 2
- Throughput: 1.11 MB/s
- Duration: 0.00 seconds

**Best Configuration:**
- Chunk Size: 1 MB
- Worker Count: 9
- Throughput: 4.21 MB/s
- Duration: 0.00 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 277.9% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 1 | 9 | 4.21 | 0.00 | Worker Variation |
| 1 | 14 | 1.44 | 0.00 | Worker Variation |
| 1 | 11 | 1.40 | 0.00 | Worker Variation |
| 1 | 12 | 1.35 | 0.00 | Worker Variation |
| 1 | 6 | 1.22 | 0.00 | Worker Variation |
| 1 | 7 | 1.20 | 0.00 | Worker Variation |
| 1 | 2 | 1.18 | 0.00 | Chunk Variation |
| 1 | 8 | 1.15 | 0.00 | Worker Variation |
| 32 | 2 | 1.15 | 0.00 | Chunk Variation |
| 1 | 2 | 1.11 | 0.00 | Adaptive |
| 1 | 13 | 1.08 | 0.00 | Worker Variation |
| 1 | 16 | 1.07 | 0.00 | Worker Variation |
| 1 | 15 | 1.02 | 0.00 | Worker Variation |
| 2 | 2 | 1.02 | 0.00 | Chunk Variation |
| 16 | 2 | 1.00 | 0.00 | Chunk Variation |
| 1 | 3 | 0.98 | 0.00 | Worker Variation |
| 128 | 2 | 0.97 | 0.00 | Chunk Variation |
| 8 | 2 | 0.92 | 0.00 | Chunk Variation |
| 1 | 4 | 0.92 | 0.00 | Worker Variation |
| 1 | 5 | 0.88 | 0.00 | Worker Variation |
| 64 | 2 | 0.88 | 0.00 | Chunk Variation |
| 1 | 10 | 0.85 | 0.00 | Worker Variation |
| 1 | 1 | 0.83 | 0.00 | Worker Variation |
| 4 | 2 | 0.26 | 0.01 | Chunk Variation |

## Summary Recommendations

- **1 MB files**: 1 MB chunks, 9 workers (4.21 MB/s)
