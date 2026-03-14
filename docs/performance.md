# Performance and Benchmarking

Vox-Hash is designed for speed, utilizing Rust's zero-cost abstractions and safe concurrency.

## Benchmarking Command

You can measure the hashing speed on your machine using the `benchmark` command:

```bash
vox-hash benchmark --algo sha1 --iterations 10000000
```

This will run the hashing algorithm in a tight loop and output the average hashes per second.

## Key Performance Factors

### 1. Algorithm Selection
- MD5 is generally faster than SHA1.
- Choosing the correct algorithm is crucial for maximizing throughput.

### 2. Concurrency (`--conc`)
- Vox-Hash uses `rayon` to parallelize work across all available CPU cores.
- By default, it uses high concurrency (20 threads), which can be adjusted.
- For brute-force, more cores = more candidates tried per second.

### 3. Strategy Ordering
The tool follows a specific order to minimize work:
1. **Rainbow Table**: Instant lookup for precomputed results.
2. **Common Patterns**: High-probability matches (e.g., "password").
3. **Wordlist**: Checks common dictionaries.
4. **Brute Force**: Exhaustive search (slowest).

### 4. Search Space reduction
Unnecessarily large charsets or lengths exponentially increase the search space. Use `--pattern` or `--length` to narrow down the search when possible.

## Typical Performance

*Note: These are estimates and vary highly based on hardware.*

| Algorithm | Speed (Hashes/sec) | Search Space (6-char alphanumeric) | Time to Exhaust |
|-----------|--------------------|-----------------------------------|-----------------|
| MD5       | ~5-10 Million     | 2.1 Billion                       | ~4-8 Minutes    |
| SHA1      | ~3-7 Million      | 2.1 Billion                       | ~6-12 Minutes   |
