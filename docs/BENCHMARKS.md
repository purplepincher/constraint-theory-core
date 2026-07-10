# Constraint Theory Core — Benchmarks

**Last measured:** 2026-07-09
**Crate version:** 2.2.0
**How to reproduce:** `cargo run --release --example bench_comparison` (KD-tree vs brute force) and `cargo run --release --example simd` (scalar vs SIMD batch).

> ⚠️ Every number below was measured on **one development machine** in a release
> build, single-threaded. Absolute timings are CPU/cache dependent — treat them
> as order-of-magnitude and **re-run the examples on your own hardware** before
> trusting them. The things that are *not* machine-dependent (state counts,
> asymptotic complexity, and the fact that the SIMD batch path is slower than
> the scalar KD-tree path) are called out explicitly.

---

## 1. Manifold state counts (deterministic, not machine-dependent)

`PythagoreanManifold::new(density)` enumerates primitive Pythagorean triples
via Euclid's formula and stores 5 sign/swap variants per triple plus the 4
cardinal axes. The count is exact and reproducible:

| Density | Valid states | Memory for `Vec<[f32;2]>` |
|---------|-------------:|--------------------------:|
| 50      |        2,494 |                   ~20 KB  |
| 100     |       10,004 |                   ~78 KB  |
| 200     |       40,384 |                  ~315 KB  |
| 500     |      252,829 |                 ~1.9 MB   |

State count grows roughly quadratically with `density` (≈ density² / π). This is
the single most important correction in this document: earlier revisions
claimed "density 200 ≈ 1000 states", which is off by ~40× and invalidated the
build-time, query-time, and memory tables below.

---

## 2. KD-tree vs brute force (single-vector nearest neighbour)

Measured with `cargo run --release --example bench_comparison` (2026-07-09).
The example builds a fresh KD-tree and queries 1000 unit vectors × 10
iterations, with the result sink `black_box`-ed so the optimiser cannot elide
the work.

| Density | States   | Brute force (ns/op) | KD-tree (ns/op) | Speedup |
|---------|---------:|--------------------:|----------------:|--------:|
| 100     |  10,004  |              12,041 |             184 |   65×   |
| 200     |  40,384  |              52,218 |             245 |  213×   |
| 500     | 252,829  |             335,678 |             509 |  660×   |

✅ This is the headline result: the KD-tree gives the expected near-logarithmic
scaling and is **two to three orders of magnitude faster than brute force** as
the manifold grows. (The hand-rolled KD-tree in the example is independent of
the crate's `KDTree`; both are O(log n).)

---

## 3. Scalar (`snap_batch`) vs SIMD (`snap_batch_simd`) batch snapping

Measured with `cargo run --release --example simd`, 1000 vectors per density
(2026-07-09).

| Density | States   | Scalar KD-tree (ns/vec) | SIMD batch (ns/vec) | SIMD / scalar |
|---------|---------:|------------------------:|--------------------:|--------------:|
| 50      |   2,494  |                    106  |                526  |     0.20×     |
| 100     |  10,004  |                    131  |              2,198  |     0.06×     |
| 200     |  40,384  |                    182  |              8,018  |     0.02×     |
| 500     | 252,829  |                  1,172  |             56,057  |     0.02×     |

⚠️ **The SIMD batch path is not faster — it is much slower.** `snap_batch_simd`
is a **brute-force scan over every manifold state** (it does not use the
KD-tree), so it is O(batch × states) while the scalar `snap_batch` is
O(batch × log states). Earlier revisions of this document claimed an "8–9× SIMD
speedup"; that does **not** reproduce and should not be relied on.

**Recommendation:** use `snap_batch()` (scalar, KD-tree) for production batch
snapping. Treat `snap_batch_simd()` as experimental until it is restructured to
use an indexed lookup. 🔮

---

## 4. Complexity summary

| Operation                         | Complexity        | Notes                          |
|-----------------------------------|-------------------|--------------------------------|
| `PythagoreanManifold::new(d)`     | O(d²)             | Enumerate primitive triples    |
| `snap` (single vector)            | O(log N)          | KD-tree nearest neighbour      |
| `snap_batch` (scalar)             | O(m log N)        | KD-tree per vector; recommended|
| `snap_batch_simd`                 | O(m × N)          | Brute-force; slower for N ≥ ~50|
| Memory                            | O(N)              | ~8 N bytes for the state vector|

---

## 5. How to reproduce on your machine

```bash
git clone https://github.com/purplepincher/constraint-theory-core
cd constraint-theory-core

# KD-tree vs brute force (the meaningful comparison)
cargo run --release --example bench_comparison

# Scalar vs SIMD batch (shows the SIMD path is currently slower)
cargo run --release --example simd

# Criterion micro-benchmarks (slower; many groups)
cargo bench
```

`cargo bench` runs the Criterion harness in `benches/core_benchmarks.rs`
(manifold snap/batch/construction, quantizer, hidden dims, holonomy). Note that
the `manifold_batch` SIMD group will likewise show SIMD slower than scalar.

---

## 6. What these benchmarks do NOT measure

⚠️ Be explicit about the gaps before citing this crate for a workload:

- **No ML benchmarking.** Nothing is measured against classification,
  clustering, retrieval, or embedding workloads.
- **No comparison with general CSP solvers** (OR-Tools, Gecode, MiniZinc). This
  crate is a geometric snapper, not a general constraint solver.
- **No concurrent/production load testing.** All numbers are single-threaded on
  synthetic unit vectors.
- **Synthetic inputs only.** Query vectors are generated by sweeping an angle,
  not drawn from a real application.
- **The "~100 ns snap" figure quoted in some older material** is in the right
  order of magnitude for the scalar KD-tree path at small densities (measured
  ~170–245 ns here), but it is **not** a guarantee and was never reproducible as
  a blanket statement.

---

## 7. Industry context (references, not measured here)

For context only — these figures come from each project's own documentation, not
from measurements taken for this crate:

| Tool | Typical NN latency (claimed) | Source |
|------|------------------------------|--------|
| FLANN | ~50–200 ns | flann-lib/flann README |
| FAISS | ~10–100 ns | facebookresearch/faiss docs |
| scikit-learn KDTree | ~1–5 µs | scikit-learn docs |

This crate's scalar KD-tree (~180–500 ns in the table above) is in a similar
range for this specialised 2-D Pythagorean task, but it has **not** been
benchmarked head-to-head against any of those libraries.

---

**Document version:** 2.2.0-bench
**Next review:** when SIMD is restructured to use an indexed lookup, or on the
next release — whichever comes first.
