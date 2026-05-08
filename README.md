# Constraint Theory Core

`0.6² + 0.8² = 1.0000000000000002` — and you've been debugging this for years.

[![GitHub stars](https://img.shields.io/github/stars/SuperInstance/constraint-theory-core?style=social)](https://github.com/SuperInstance/constraint-theory-core)
[![CI](https://github.com/SuperInstance/constraint-theory-core/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/constraint-theory-core/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/constraint-theory-core.svg)](https://crates.io/crates/constraint-theory-core)
[![docs.rs](https://docs.rs/constraint-theory-core/badge.svg)](https://docs.rs/constraint-theory-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**`cargo add constraint-theory-core`** · [Live Demos](https://constraint-theory-web.pages.dev) · [Docs](https://docs.rs/constraint-theory-core)

---

## What This Crate Does

Constraint Theory replaces floating-point approximation with exact rational arithmetic. It maps any continuous 2D vector to the nearest Pythagorean rational point — `(3/5, 4/5)`, `(5/13, 12/13)`, and so on — where a² + b² = c² holds in integers, not floats.

The result: **0.6² + 0.8² = 1.0 exactly. Not 1.0000000000000002.**

This isn't arbitrary precision. It's a finite lattice of exact points, indexed by a KD-tree, with O(log N) lookup. You snap once, and the result is reproducible on every machine, every platform, every compile, forever.

### The mechanism in one paragraph

Pythagorean triples are integer solutions to a² + b² = c². Euclid's formula generates all of them: (m²−n², 2mn, m²+n²). Normalize to (a/c, b/c) and you get exact rational points on the unit circle. There are infinitely many, but only finitely many within any precision bound. Precompute those, build a KD-tree, and snap any input vector to the nearest exact neighbor. The snap is deterministic, platform-independent, and exact.

## Install

```bash
cargo add constraint-theory-core
```

Zero dependencies. Pure Rust. `#![forbid(unsafe_code)]` on the public API.

## Verify It Works

```rust
use constraint_theory_core::{PythagoreanManifold, snap};

fn main() {
    let manifold = PythagoreanManifold::new(200);        // ~1000 exact states, ~80KB
    let (exact, noise) = snap(&manifold, [0.577, 0.816]); // ~100ns

    println!("Snapped to: [{}, {}]", exact[0], exact[1]); // [0.6, 0.8]

    let mag_sq = exact[0] * exact[0] + exact[1] * exact[1];
    println!("|v|² = {}", mag_sq); // 1.0. Not 1.0000000000000002.
}
```

```bash
cargo test    # 184 tests pass
```

## What It's Good For

**Deterministic multiplayer** — Same joystick input produces the same direction on every client, every frame, no floating-point divergence.

```rust
fn process_input(&mut self, joystick: [f32; 2]) {
    let (direction, noise) = self.manifold.snap(joystick);
    self.velocity = [direction[0] * SPEED, direction[1] * SPEED];
    // direction is identical on every machine. Always.
}
```

**Robotics** — Repeatability. The same target direction produces the same motion, down to the bit, across power cycles.

```rust
fn move_arm(&mut self, target: [f32; 2]) {
    let (direction, noise) = self.manifold.snap(target);
    if noise > 0.01 {
        log::warn!("High quantization noise — target is between lattice points");
    }
}
```

**ML direction quantization** — Reproducible training. Same embedding, same snap, same result, every run.

```rust
let (quantized, _) = manifold.snap(project_to_2d(&embedding));
// Integer arithmetic from here. Deterministic across runs.
```

## Capabilities

| Feature | What It Does |
|---|---|
| **Exact snapping** | Map any 2D vector to nearest Pythagorean rational. O(log N). |
| **KD-tree spatial index** | O(N log N) build, O(log N) query. Deterministic tie-breaking. |
| **SIMD batch** | 8× f32 parallelism on AVX2. Auto-detected at runtime. |
| **Holonomy verification** | Parallel transport around cycles. Zero holonomy = consistent. |
| **Sheaf cohomology** | H₀ (components) and H₁ (cycles) in O(1). Emergence detection. |
| **Ricci flow** | Curvature evolution. Flatten manifolds. Convergence guaranteed. |
| **Laman rigidity** | Check if a constraint graph is rigid. O(V²) per check. |
| **Quantization** | TurboQuant, BitNet (ternary), PolarQuant, Hybrid auto-select. |
| **Hidden dimensions** | Lift to Rⁿ⁺ᵏ for exact constraint encoding. k = ⌈log₂(1/ε)⌉. |

## Benchmarks

| Operation | Time |
|---|---|
| Single snap (density 200) | ~100 ns |
| SIMD batch (1000 vectors) | ~74 ns/op |
| Manifold build (density 200) | ~2.8 ms (one-time) |
| Ternary quantize (128D) | ~50 ns |
| Holonomy check (cycle length 16) | ~300 ns |

| Density | Exact States | Memory |
|---|---|---|
| 50 | ~250 | ~20 KB |
| 200 | ~1000 | ~80 KB |
| 500 | ~2500 | ~200 KB |

Full benchmarks: [docs/BENCHMARKS.md](./docs/BENCHMARKS.md)

## Core Types

| Type | What It Is |
|---|---|
| `PythagoreanManifold` | Precomputed exact points + KD-tree. The main entry point. ~80KB at density 200. |
| `Tile` | 384-byte fundamental unit (compile-time verified). Origin, payload, constraints. Cache-line aligned. |
| `ConstraintBlock` | 192 bytes of holonomy matrix, Ricci curvature, percolation probability, gluing map. |
| `PythagoreanQuantizer` | Unified quantizer: Ternary {-1,0,1}, Polar (exact unit norm), Turbo (near-optimal), Hybrid (auto). |
| `HolonomyChecker` | Incremental cycle verification. `apply()`, `check_partial()`, `check_closed()`. |
| `FastPercolation` | Union-find with path compression for Laman rigidity percolation. |
| `FastCohomology` | H₀ and H¹ via Euler characteristic. O(1). |

## Architecture

```
src/
├── manifold.rs         PythagoreanManifold — triple generation, snapping
├── kdtree.rs           KD-tree — O(log N) spatial index
├── tile.rs             Tile (384B), Origin (64B), ConstraintBlock (192B)
├── holonomy.rs         Holonomy verification, HolonomyChecker
├── cohomology.rs       H₀, H¹ via Euler characteristic
├── curvature.rs        RicciFlow — curvature evolution
├── percolation.rs      FastPercolation — Laman rigidity
├── gauge.rs            GaugeConnection — parallel transport
├── quantizer.rs        PythagoreanQuantizer — 4 modes
├── hidden_dimensions.rs  Precision encoding, k = ⌈log₂(1/ε)⌉
├── cache.rs            CachedLattice — thread-safe global cache
├── simd.rs             AVX2 batch snapping (x86_64, runtime-detected)
└── dcs.rs              Physical constants (Laman threshold, Ricci multiplier)
```

## The Floating-Point Problem

```rust
// The bug you've shipped:
let x = 0.6_f64;
let y = 0.8_f64;
let mag = (x * x + y * y).sqrt();  // 1.0000000000000002

if mag == 1.0 { /* this never runs */ }
```

0.6 and 0.8 can't be represented exactly in IEEE 754. They're approximations. The error is small — one ULP — but it compounds. After a few matrix multiplications, you're comparing against 1.0000003. After a few hundred, you're in territory where `==` is meaningless.

Constraint Theory's answer: 0.6 and 0.8 aren't floats. They're the rational numbers 3/5 and 4/5. Stored as floats for display, but the underlying triple (3, 4, 5) guarantees a² + b² = c² = 25 in exact integer arithmetic. The float representation is a convenience. The integer triple is the truth.

## Limitations (Honest Ones)

| Limitation | Why | Impact |
|---|---|---|
| **2D only** | Pythagorean triples are inherently 2D | Not suitable for 3D directly |
| **~1000 discrete states** (density 200) | Finite lattice, not continuous | ~0.36° angular resolution |
| **Quantization noise** | Snapping introduces distance from input | Check the returned `noise` value |
| **SIMD platform variance** | AVX2 results may differ from scalar | Use scalar path for consensus-critical code |

## Quality

| Metric | Value |
|---|---|
| Tests | 184 passing |
| Dependencies | Zero |
| unsafe | Only in SIMD intrinsics, behind safe wrappers |
| CI | Linux, macOS, Windows |
| Fuzzing | Property-based tests via proptest |

## Research

- [arXiv:2503.15847](https://arxiv.org/abs/2503.15847) — Constraint Theory: Deterministic Manifold Snapping via Pythagorean Geometry
- [Mathematical Foundations (45 pages)](https://github.com/SuperInstance/constraint-theory-research/blob/main/MATHEMATICAL_FOUNDATIONS_DEEP_DIVE.md)
- [Theoretical Guarantees](https://github.com/SuperInstance/constraint-theory-research/blob/main/guides/THEORETICAL_GUARANTEES.md)
- [Proofs and errata → constraint-theory-math](https://github.com/SuperInstance/constraint-theory-math)

## The Ecosystem

| Repo | What It Is |
|---|---|
| **[constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core)** | This crate. Rust, zero deps, 184 tests. |
| **[constraint-theory-python](https://github.com/SuperInstance/constraint-theory-python)** | Python bindings. NumPy + PyTorch. |
| **[constraint-theory-web](https://github.com/SuperInstance/constraint-theory-web)** | 50 interactive demos. |
| **[constraint-theory-math](https://github.com/SuperInstance/constraint-theory-math)** | Proofs, sheaf cohomology, errata. |
| **[holonomy-consensus](https://github.com/SuperInstance/holonomy-consensus)** | Zero-holonomy consensus for distributed systems. |
| **[fleet-coordinate](https://github.com/SuperInstance/fleet-coordinate)** | Fleet coordination using Eisenstein spatial hashing. |
| **[constraint-theory-research](https://github.com/SuperInstance/constraint-theory-research)** | Papers, open problems, formal proofs. |

## Contributing

[Good First Issues](https://github.com/SuperInstance/constraint-theory-core/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) · [CONTRIBUTING.md](CONTRIBUTING.md)

```bash
rustup component add clippy rustfmt
cargo fmt && cargo clippy -- -D warnings && cargo test
```

## Citation

```bibtex
@software{constraint_theory,
  title={Constraint Theory: Deterministic Manifold Snapping via Pythagorean Geometry},
  author={SuperInstance},
  year={2025},
  url={https://github.com/SuperInstance/constraint-theory-core},
  version={2.2.0}
}
```

## License

MIT
