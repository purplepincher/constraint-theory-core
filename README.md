# Constraint Theory Core

[![CI](https://github.com/purplepincher/constraint-theory-core/actions/workflows/ci.yml/badge.svg)](https://github.com/purplepincher/constraint-theory-core/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/constraint-theory-core.svg)](https://crates.io/crates/constraint-theory-core)
[![docs.rs](https://docs.rs/constraint-theory-core/badge.svg)](https://docs.rs/constraint-theory-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Deterministic 2D vector snapping to exact Pythagorean triples.

`0.6² + 0.8² = 1.0000000000000002` in IEEE-754, but this crate makes it exactly `1.0`
by mapping any continuous 2D direction to the nearest rational point
`(a/c, b/c)` where `a² + b² = c²`. The snap is deterministic, platform-independent,
and backed by a KD-tree for `O(log N)` lookup.

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
    let manifold = PythagoreanManifold::new(200); // ~1000 exact states
    let (exact, noise) = snap(&manifold, [0.577, 0.816]);

    println!("Snapped to: [{}, {}]", exact[0], exact[1]); // [0.6, 0.8]
    let mag_sq = exact[0] * exact[0] + exact[1] * exact[1];
    println!("|v|² = {}", mag_sq); // 1.0 exactly
    println!("noise = {}", noise); // distance from the input
}
```

Run the bundled example:

```bash
cargo run --example basic
```

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
- **Finite lattice.** A density of ~200 yields ~1000 discrete states and an
  angular resolution of roughly 0.36°. Snapping always introduces some
  quantization noise; check the returned `noise` value.
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
