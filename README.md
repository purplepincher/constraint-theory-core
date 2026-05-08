# constraint-theory-core

**Exact constraint satisfaction on integer coordinates. 184 tests. Production use.**

This crate does one thing: maintain exact constraints across distributed systems using integer arithmetic instead of floating-point approximations. Every constraint value snaps to a discrete lattice point. Every comparison is deterministic. Every machine produces exactly the same result for the same input.

## The Problem

Distributed constraint propagation has a floating-point problem. Two agents run the same computation on the same data. Different FPU rounding modes, different compiler optimization levels, different hardware — they get different answers. The fleet agrees on something that was never true. This is the silent failure that multi-agent systems die from.

## The Solution

Encode every constraint as an integer lattice coordinate. The snapping operation projects a continuous value to the nearest lattice point. Once snapped, all operations — comparison, composition, propagation — are exact integer arithmetic. Holonomy verification around any cycle returns exactly zero if and only if all participants computed consistently.

## What's In the Crate

- **Constraint lattice** — discrete coordinate system for constraint values
- **Snapping** — project continuous values to nearest exact lattice point
- **Comparison** — exact integer comparison, no epsilon needed
- **Propagation** — lattice-preserving constraint propagation
- **Holonomy verification** — cycle detection for distributed consistency

## Quick Start

```rust
use constraint_theory_core::{Constraint, Lattice};

// Create a constraint at a lattice point
let c = Constraint::new(3, 4);
assert_eq!(c.norm_squared(), 25); // exact

// Snap a continuous value to the lattice
let snapped = Lattice::snap(0.6, 0.8);
assert!(snapped.is_exact());

// Verify holonomy around a cycle
let consistent = c.verify_cycle(&neighbors);
assert!(consistent);
```

## Status

- **184 tests passing** — unit, property, integration
- **Published to crates.io** — `cargo add constraint-theory-core`
- **`no_std` compatible** — no allocator needed for core types
- **Zero unsafe** — no `unsafe` in the constraint propagation core

## What This Is Not

Constraint theory doesn't solve P vs NP. It doesn't replace SAT solvers. It doesn't eliminate the need for careful distributed systems design. What it does is eliminate an entire class of silent failures — the ones caused by floating-point disagreement across machines. That's a narrow claim, but it's a true one.

## License

MIT OR Apache-2.0

## Eisenstein Ecosystem

| Project | What It Does |
|---------|-------------|
| **[eisenstein](https://github.com/SuperInstance/eisenstein)** | Core Eisenstein integer arithmetic |
| **[constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core)** | Constraint propagation on integer lattices |
| **[flux-lucid](https://github.com/SuperInstance/flux-lucid)** | Intent vectors, alignment, tolerance navigation |
| **[holonomy-consensus](https://github.com/SuperInstance/holonomy-consensus)** | Topological consensus without quorum |
| **[fleet-coordinate](https://github.com/SuperInstance/fleet-coordinate)** | Multi-agent spatial coordination |
