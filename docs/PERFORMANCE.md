# Performance Characteristics

**Crate version:** 2.2.0
**Last reviewed:** 2026-07-09 (numbers re-measured; see `docs/BENCHMARKS.md`)

---

## Overview

This document describes the performance characteristics of
`constraint-theory-core`. All timings are illustrative and were measured on one
development machine in a release build, single-threaded — re-run the bundled
examples on your own hardware before relying on them. The structural facts
(complexity, state counts, and that the SIMD batch path is slower than the
scalar KD-tree path) are not machine-dependent.

---

## Complexity Analysis

### Core operations

| Operation                          | Time             | Space   | Notes                            |
|------------------------------------|------------------|---------|----------------------------------|
| `PythagoreanManifold::new(density)`| O(density²)      | O(N)    | Enumerate primitive triples      |
| `snap()` (single vector)           | O(log N)         | O(1)    | KD-tree lookup; recommended      |
| `snap_batch()` (scalar)            | O(m log N)       | O(m)    | KD-tree per vector; recommended  |
| `snap_batch_simd()` (AVX2)         | O(m × N)         | O(m)    | Brute-force; **slower for N ≥ ~50** |

`N` is the number of valid states; `m` is the batch size.

### KD-tree operations

| Operation        | Average   | Worst case | Notes                |
|------------------|-----------|------------|----------------------|
| Build            | O(N log N)| O(N log N) | Deterministic        |
| Nearest neighbour| O(log N)  | O(N)       | Degenerate input rare|

---

## State counts (deterministic)

`density` is the max `m` in Euclid's formula; the count is exact (see
`docs/BENCHMARKS.md` §1):

| Density | States   |
|---------|---------:|
| 50      |    2,494 |
| 100     |   10,004 |
| 200     |   40,384 |
| 500     |  252,829 |

> Earlier revisions of this document listed "density 200 ≈ 1000 states". That
> was incorrect by ~40×.

---

## Memory usage

The dominant, exactly-knowable term is the state vector
`Vec<[f32; 2]>` = **8 · N bytes**:

| Density | States   | `valid_states` vector |
|---------|---------:|----------------------:|
| 50      |   2,494  |            ~20 KB     |
| 100     |  10,004  |            ~78 KB     |
| 200     |  40,384  |           ~315 KB     |
| 500     | 252,829  |           ~1.9 MB     |

On top of that, the KD-tree holds its own data: leaf nodes store a copy of
their points (`[f32; 2]`) plus a `Vec<usize>` of indices, and internal nodes
carry split metadata and two boxed children. ⚠️ This roughly **doubles to
triples** the per-state footprint versus the raw `8 · N` figure above, so the
true resident size at density 200 is on the order of **~0.7–1.0 MB** (not the
"~48 KB" claimed in earlier revisions). Memory still scales linearly with `N`.

### Allocation pattern

- Manifold creation: one `Vec` for states + a single KD-tree build (recursive).
- `snap()` hot path: no heap allocation.
- Batch paths: the results `Vec` is allocated by the caller (`*_into`) or by the
  convenience method.

---

## SIMD: measured reality

⚠️ The SIMD batch path (`snap_batch_simd`, AVX2 on x86_64) does a **brute-force
scan over every state** — it does not use the KD-tree. Measured
(`cargo run --release --example simd`, 2026-07-09):

| Density | Scalar `snap_batch` | SIMD `snap_batch_simd` | SIMD / scalar |
|---------|--------------------:|-----------------------:|--------------:|
| 50      |          106 ns/vec |              526 ns/vec|     0.20×     |
| 100     |          131 ns/vec |            2,198 ns/vec|     0.06×     |
| 200     |          182 ns/vec |            8,018 ns/vec|     0.02×     |
| 500     |        1,172 ns/vec |           56,057 ns/vec|     0.02×     |

So SIMD is **not** faster; it is one to two orders of magnitude slower than the
scalar KD-tree path at these sizes. 🔮 Making the SIMD path competitive would
require restructuring it to use an indexed/tree lookup (or delegating to the
scalar path). Until then, prefer `snap_batch()`.

There is **no** AVX-512 or NEON path today (the earlier "AVX-512 / NEON
planned/implemented" table was aspirational, not real). 🔮

---

## Throughput (scalar, KD-tree path)

Measured single-threaded, release build (2026-07-09):

| Operation               | Latency        | Throughput     |
|-------------------------|----------------|----------------|
| Single `snap()` (d=200) | ~170 ns        | ~5.9 M ops/sec |
| `snap_batch` (d=200)    | ~182 ns/vec    | ~5.5 M ops/sec |
| `snap_batch` (d=500)    | ~1172 ns/vec   | ~0.85 M ops/sec|

(The "~100 ns / ~10 M ops/sec" headline quoted in older material is the right
order of magnitude for small densities but is not a guarantee.)

---

## Multi-threading

The manifold is immutable after construction, so the recommended pattern is to
share one `Arc<PythagoreanManifold>` across threads and call `snap`/`snap_batch`
without locking:

```rust
let manifold = Arc::new(PythagoreanManifold::new(200));
// each thread: manifold.clone() then call snap()/snap_batch() — lock-free
```

⚠️ No multi-threaded scaling measurements have been taken for this crate. The
"1→2→4→8 thread ≈ linear scaling" table in earlier revisions was projected, not
measured. 🔮

---

## Latency distribution

⚠️ No percentile (P50/P95/P99) measurements have been taken. Earlier revisions
listed specific P50/P95/P99 figures; those were not reproducible from the
codebase and have been removed. The mean single-`snap` latency is ~170 ns at
density 200 (above).

---

## Choosing a density

| Use case            | Density | States   | `valid_states` size |
|---------------------|---------|---------:|--------------------:|
| Low memory / fast   | 50–100  | 2.5K–10K | ~20–78 KB           |
| Balanced (default)  | 200     | ~40K     | ~315 KB             |
| High precision      | 500     | ~253K    | ~1.9 MB             |

Higher density → finer angular resolution but more memory and slightly slower
queries (logarithmic).

---

## Platform considerations

- **x86_64**: scalar KD-tree path is the fast path; AVX2 SIMD batch path
  available but slower (see above).
- **Other architectures (ARM, WebAssembly)**: the SIMD path falls back to a
  scalar brute-force scan — use `snap_batch()` instead. A `wasm/` target is
  present in the repo.

```bash
# Build with native CPU features enabled
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

---

**Document version:** 2.2.0-perf
**Next review:** when the SIMD path is restructured, or on the next release.
