# Constraint Theory Core

[![CI](https://github.com/purplepincher/constraint-theory-core/actions/workflows/ci.yml/badge.svg)](https://github.com/purplepincher/constraint-theory-core/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/constraint-theory-core.svg)](https://crates.io/crates/constraint-theory-core)
[![docs.rs](https://docs.rs/constraint-theory-core/badge.svg)](https://docs.rs/constraint-theory-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Deterministic 2D vector snapping to exact Pythagorean triples.

Floating-point unit vectors are not exact: in f32, normalizing `(1, 1)`
yields `(0.70710677, 0.70710677)`, whose squared magnitude is `0.99999994`,
not `1.0`. This crate sidesteps that drift by mapping any 2D direction to
the nearest rational point `(a/c, b/c)` where `a² + b² = c²` — a
Pythagorean triple. The result is drawn from a fixed lattice of exact unit
directions, snapped deterministically and backed by a KD-tree for
`O(log N)` lookup, so the same input produces the same output on every
platform (scalar path; see the SIMD note in [Limitations](#honest-limitations)).

## Install

```bash
cargo add constraint-theory-core
```

Zero runtime dependencies. Pure Rust. The public API is safe; SIMD batch paths use
architecture-specific intrinsics internally and are wrapped in safe interfaces.

## Quick Start

```rust
use constraint_theory_core::{PythagoreanManifold, snap};

fn main() {
    let manifold = PythagoreanManifold::new(200); // ~40,000 exact states
    let (exact, noise) = snap(&manifold, [3.0, 4.0]);

    println!("Snapped to: [{}, {}]", exact[0], exact[1]); // [0.6, 0.8]  (the 3-4-5 triple)
    let mag_sq = exact[0] * exact[0] + exact[1] * exact[1];
    println!("|v|² = {}", mag_sq); // 1
    println!("noise = {}", noise);  // 0 (input was already an exact triple)
}
```

Run the bundled example:

```bash
cargo run --example basic
```

A non-triple input snaps to the nearest exact direction just the same —
`snap(&manifold, [5.0, 12.0])` returns `[0.3846154, 0.9230769]` (the 5-12-13
triple), also with `|v|² = 1` and `noise = 0`.

## API Surface

| Type / Function | What it does |
| --------------- | ------------ |
| `PythagoreanManifold::new(density)` | Pre-compute Pythagorean states and a KD-tree. |
| `PythagoreanManifold::snap(&self, [f32; 2])` | Snap a single vector to the nearest exact state. |
| `snap(&manifold, vector)` | Convenience wrapper for the above. |
| `PythagoreanManifold::snap_batch()` | Deterministic scalar batch snap (recommended for consensus). |
| `PythagoreanManifold::snap_batch_simd()` | AVX2 batch snap (faster, but platform-specific tie-breaking). |
| `PythagoreanQuantizer` | Quantize vectors while preserving unit-norm constraints (Polar/Turbo/Ternary/Hybrid). |
| `HolonomyChecker` / `compute_holonomy` | Check consistency of transformations around cycles. |
| `FastPercolation` | Laman-rigidity style rigidity checks for constraint graphs. |
| `hidden_dim_count(epsilon)` | Compute `k = ⌈log₂(1/ε)⌉` for precision encoding. |

See [docs.rs](https://docs.rs/constraint-theory-core) for full API documentation.

## Honest Limitations

This crate does one thing well: deterministic 2D geometric snapping.

- **2D only.** Pythagorean triples are inherently planar. Higher-dimensional
  support is experimental/theoretical.
- **Finite lattice.** A density of 200 yields 40,384 discrete states. The
  lattice is non-uniform (denser near some angles than others), so snapping
  always introduces some quantization noise; check the returned `noise`
  value, which is `0` when the input is already an exact triple.
- **Not a general constraint solver.** For scheduling, routing, or large CSPs,
  use established tools such as OR-Tools or Gecode.
- **Not an AI/LLM framework.** The term “zero hallucination” applies only in the
  narrow geometric sense that every snapped output satisfies `a² + b² = c²`;
  it does not make claims about machine-learning outputs.
- **Performance claims are scoped.** The ~100 ns snap time and any speedup
  figures are relative to a simple NumPy brute-force baseline on this specific
  nearest-neighbor task, not to every possible workload.
- **Research release.** The code is tested and deterministic, but it has not yet
  been battle-tested in large-scale production deployments.
- **SIMD path.** `snap_batch_simd` may produce slightly different results across
  platforms due to parallel reduction ordering; use `snap_batch` for
  consensus-critical code.

## Benchmarks

See [`docs/BENCHMARKS.md`](./docs/BENCHMARKS.md) for methodology and measured
numbers. Run the included benchmark example with:

```bash
cargo run --release --example bench
```

## License

MIT. See [LICENSE](./LICENSE).
