# Pipeline Optimization Benchmark Report

Generated: 2025-10-08 01:58:26 UTC

## File Size: 1024 MB

**Adaptive Configuration:**
- Chunk Size: 64 MB
- Worker Count: 10
- Throughput: 659.66 MB/s
- Duration: 1.71 seconds

**Best Configuration:**
- Chunk Size: 64 MB
- Worker Count: 5
- Throughput: 821.82 MB/s
- Duration: 1.25 seconds
- Configuration Type: Worker Variation

**Performance Improvement:** 24.6% faster than adaptive

### Detailed Results

| Chunk Size (MB) | Workers | Throughput (MB/s) | Duration (s) | Config Type |
|-----------------|---------|-------------------|--------------|-------------|
| 64 | 5 | 821.82 | 1.25 | Worker Variation |
| 64 | 8 | 818.94 | 1.25 | Worker Variation |
| 64 | 4 | 817.73 | 1.25 | Worker Variation |
| 64 | 7 | 817.54 | 1.25 | Worker Variation |
| 64 | 6 | 816.10 | 1.25 | Worker Variation |
| 64 | 11 | 815.82 | 1.26 | Worker Variation |
| 64 | 12 | 815.72 | 1.26 | Worker Variation |
| 64 | 9 | 815.30 | 1.26 | Worker Variation |
| 64 | 3 | 813.66 | 1.26 | Worker Variation |
| 64 | 16 | 804.61 | 1.27 | Worker Variation |
| 64 | 14 | 803.34 | 1.27 | Worker Variation |
| 64 | 13 | 799.16 | 1.28 | Worker Variation |
| 64 | 2 | 784.84 | 1.30 | Worker Variation |
| 64 | 15 | 768.51 | 1.34 | Worker Variation |
| 32 | 10 | 708.76 | 1.54 | Chunk Variation |
| 64 | 10 | 659.66 | 1.71 | Adaptive |
| 128 | 10 | 616.42 | 1.73 | Chunk Variation |
| 64 | 1 | 582.68 | 1.89 | Worker Variation |
| 16 | 10 | 547.32 | 1.93 | Chunk Variation |
| 4 | 10 | 497.57 | 2.06 | Chunk Variation |
| 8 | 10 | 488.82 | 2.10 | Chunk Variation |
| 2 | 10 | 461.82 | 2.22 | Chunk Variation |
| 1 | 10 | 434.91 | 2.37 | Chunk Variation |

## Summary Recommendations

- **1024 MB files**: 64 MB chunks, 5 workers (821.82 MB/s)
