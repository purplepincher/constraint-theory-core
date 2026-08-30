# Constraint Theory Core

[![CI](https://github.com/purplepincher/constraint-theory-core/actions/workflows/ci.yml/badge.svg)](https://github.com/purplepincher/constraint-theory-core/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/constraint-theory-core.svg)](https://crates.io/crates/constraint-theory-core)
[![docs.rs](https://docs.rs/constraint-theory-core/badge.svg)](https://docs.rs/constraint-theory-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Deterministic 2D vector snapping to exact Pythagorean triples.

## What this is

Floating-point unit vectors are not exact: in `f32`, normalizing `(1, 1)`
yields `(0.70710677, 0.70710677)`, whose squared magnitude is `0.99999994`,
not `1.0`. This crate sidesteps that drift by mapping any 2D direction to the
nearest rational point `(a/c, b/c)` where `a² + b² = c²` — a Pythagorean
triple. Output is drawn from a fixed lattice of exact unit directions, snapped
deterministically and backed by a KD-tree for `O(log N)` lookup, so the same
input produces the same output on every platform (scalar path).

It is a **focused geometric primitive**, not a general constraint solver and
not an ML framework. See [Scope and limitations](#scope-and-limitations).

## Status

- ✅ Core snapping: deterministic 2D Pythagorean snapping via KD-tree, tested
  (262 passing tests, 2 ignored on the current revision).
- ✅ Published on [crates.io](https://crates.io/crates/constraint-theory-core)
  as `2.2.0` (zero runtime dependencies, pure Rust, `#![deny(missing_docs)]`).
- ⚠️ The published `2.2.0` still advertises the old `SuperInstance` repository
  URL on crates.io (immutable once published). Source lives at
  `purplepincher/constraint-theory-core`; see
  [`docs/CRATES_IO_METADATA_GAP.md`](./docs/CRATES_IO_METADATA_GAP.md).
- ⚠️ The SIMD batch path (`snap_batch_simd`) is a brute-force scan over every
  state and is **slower** than the scalar KD-tree path in practice; prefer
  `snap_batch`. See [Performance](#performance).
- 🔮 Higher-dimensional snapping, a competitive SIMD path, and ML/CSP
  comparisons are unproven research directions, not shipping features.

## Install

The crate is published, so the standard command works:

```bash
cargo add constraint-theory-core
```

Zero runtime dependencies, pure Rust, MSRV 1.75. The public API is safe.

## Quick start

```rust
use constraint_theory_core::{PythagoreanManifold, snap};

fn main() {
    let manifold = PythagoreanManifold::new(200); // 40,384 exact states
    let (exact, noise) = snap(&manifold, [3.0, 4.0]);

    println!("Snapped to: [{}, {}]", exact[0], exact[1]); // [0.6, 0.8]  (the 3-4-5 triple)
    let mag_sq = exact[0] * exact[0] + exact[1] * exact[1];
    println!("|v|² = {}", mag_sq); // 1
    println!("noise = {}", noise);  // 0 (input was already an exact triple)
}
```

(Verified to compile and run; the output comments are literal.) A non-triple
input snaps to the nearest exact direction the same way:
`snap(&manifold, [5.0, 12.0])` returns `[0.3846154, 0.9230769]` (the 5-12-13
triple), also with `|v|² = 1` and `noise = 0`.

Run the bundled examples:

```bash
cargo run --example basic          # core snapping walkthrough
cargo run --release --example bench_comparison   # KD-tree vs brute force
```

## API surface

| Type / Function | What it does |
| --------------- | ------------ |
| `PythagoreanManifold::new(density)` | Pre-compute Pythagorean states and a KD-tree. `density` is the max `m` in Euclid's formula; state count grows ~quadratically (200 → 40,384). |
| `PythagoreanManifold::snap(&self, [f32; 2])` | Snap a single vector to the nearest exact state. O(log N). |
| `snap(&manifold, vector)` | Convenience wrapper for the above. |
| `PythagoreanManifold::snap_batch()` | Scalar batch snap via KD-tree. **Recommended** for production/consensus. |
| `PythagoreanManifold::snap_batch_simd()` | AVX2 batch snap (x86_64). ⚠️ Brute-force over all states — **slower** than `snap_batch` at realistic sizes; results may differ from scalar on ties. |
| `PythagoreanQuantizer` | Quantize vectors while preserving unit-norm constraints (Polar/Turbo/Ternary/Hybrid). |
| `HolonomyChecker` / `compute_holonomy` | Check consistency of transformations around cycles. |
| `FastPercolation` | Laman-rigidity style rigidity checks for constraint graphs. |
| `hidden_dim_count(epsilon)` | Compute `k = ⌈log₂(1/ε)⌉` for precision encoding. |

See [docs.rs](https://docs.rs/constraint-theory-core) for full API documentation.

## Performance

Headline (measured 2026-07-09, release build; full numbers and methodology in
[`docs/BENCHMARKS.md`](./docs/BENCHMARKS.md) and
[`docs/PERFORMANCE.md`](./docs/PERFORMANCE.md)):

- ✅ KD-tree vs brute force: **65×–660× faster** as density grows (100→500).
- ✅ Scalar `snap`/`snap_batch` (KD-tree): ~170–182 ns per vector at density 200.
- ⚠️ SIMD `snap_batch_simd`: ~8,000 ns per vector at density 200 — **~44× slower**
  than scalar, because it brute-force scans all states instead of using the
  KD-tree. Use `snap_batch` instead.

Reproduce on your machine:

```bash
cargo run --release --example bench_comparison
cargo run --release --example simd
```

## Scope and limitations

- **2D only.** Pythagorean triples are inherently planar. Higher-dimensional
  support is theoretical/unproven.
- **Finite lattice.** A density of 200 yields 40,384 discrete states. The
  lattice is non-uniform (denser near some angles), so snapping introduces
  quantization noise; check the returned `noise` (it is `0` when the input is
  already an exact triple).
- **Not a general constraint solver.** For scheduling, routing, or large CSPs,
  use OR-Tools, Gecode, or MiniZinc.
- **Not an AI/LLM framework.** "Deterministic" here means every snapped output
  satisfies `a² + b² = c²`; it makes no claims about ML outputs.
- **SIMD path is non-competitive.** `snap_batch_simd` is a brute-force scan and
  is slower than the scalar KD-tree path; it may also break ties differently
  across platforms. Use `snap_batch` for consensus-critical code.
- **Performance claims are scoped.** Timings are measured on specific hardware
  for this nearest-neighbour task, not for arbitrary workloads, and the crate
  has not been battle-tested at production scale.

## License

MIT. See [LICENSE](./LICENSE).
