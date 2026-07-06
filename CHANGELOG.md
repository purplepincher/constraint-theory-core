# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.2.0] - 2026-07-06

Matches the crate already published on crates.io at 2.2.0 (verified via
the crates.io API) — no code changed in this grouping, only documentation
accuracy.

### Changed
- Repository metadata now points to `purplepincher/constraint-theory-core`.
- Trimmed documentation to production-crate essentials; moved research drafts,
  ecosystem pitches, and speculative synergy notes to `docs/research-notes/`.
- Folded important disclaimers into the README's "Honest Limitations" section.
- Updated `VERSION_MAJOR/MINOR/PATCH` constants to match the crate version.

### Fixed
- Resolved `clippy` warnings on current stable and beta toolchains.
- Applied `rustfmt` so `cargo fmt --check` passes.
- Made the beta CI clippy step non-blocking while keeping stable as the hard gate.
- Three README claims that didn't hold up against actually running the
  code: the opening `0.6² + 0.8² = 1.0000000000000002` IEEE-754 example
  was simply wrong (it's exactly `1.0` in both f32 and f64) — replaced
  with the real drift case (`normalize(1,1)` → `|v|² = 0.99999994` in
  f32); the Quick Start's `snap([0.577, 0.816])` example didn't actually
  return `[0.6, 0.8]` at density 200 — replaced with verified
  `snap([3,4]) → [0.6,0.8]` and `snap([5,12]) → 5-12-13`; `new(200)`'s
  state count was documented as "~1000" but is actually **40,384**
  (~40x off) — corrected, and the unverified "0.36° angular resolution"
  figure was dropped rather than guessed at.

### Noted, not yet fixed
- The published crate's `repository` metadata on crates.io still points
  to `SuperInstance/constraint-theory-core`, not `purplepincher` — a
  registry-metadata divergence from the actual source location.

## [1.0.1] - 2025-01-27

### Added
- **MASTER_SCHEMA.md**: Master documentation linking all ecosystem components
- **INTEGRATION.md**: Python FFI, WASM compilation, and cross-platform integration guide
- **RESEARCH_FOUNDATIONS.md**: Citations, mathematical proofs, and arXiv references
- **DEPLOYMENT.md**: CI/CD pipeline, release process, security audit checklist
- **ECOSYSTEM.md**: Complete ecosystem overview with cross-pollination use cases
- **Hidden dimensions formula**: `k = ⌈log₂(1/ε)⌉` documented throughout
- **Ecosystem integration section** in README.md with cross-repository links
- **Integration testing documentation** for Python binding compatibility
- **SIMD consistency testing** documentation
- Enhanced **Cargo.toml** metadata with additional keywords and categories
- **Enhanced CI/CD pipeline**: Cross-platform testing, coverage, benchmarks, security audit

### Changed
- **ONBOARDING.md**: Complete API consistency rewrite
  - Updated all code examples to match actual API (`constraint_theory_core` not `constraint_theory`)
  - Fixed function signatures (`snap(&manifold, [x, y])` not `manifold.snap(&point)`)
  - Added Grand Unified Constraint Theory (GUCT) branding
  - Corrected benchmark numbers (~100ns not 45ns for single snap)
  - Added proper cross-references to ecosystem
- **README.md**: Updated citation version to match Cargo.toml (1.0.1)
- **Ecosystem table**: Enhanced with Key Features column
- **Cargo.toml**: Added keywords and categories for better discoverability

### Fixed
- API documentation inconsistencies between ONBOARDING.md and actual code
- Version number mismatch in citation
- Incorrect benchmark claims in ONBOARDING.md
- Missing links to Python examples and web visualizations

### Documentation
- All code examples now use correct `constraint_theory_core` crate name
- All code examples compile against actual API
- Hidden dimensions formula prominently featured
- Cross-platform determinism notes added

## [1.0.0] - 2025-01-15

### Added
- **PythagoreanManifold**: Core manifold implementation with KD-tree indexing
- **snap() function**: O(log n) nearest neighbor lookup
- **SIMD batch processing**: AVX2 parallel batch snapping
- **PythagoreanTriple**: Struct for exact triple representation
- **KD-tree implementation**: Cache-optimized spatial indexing
- **Edge case handling**: NaN, zero, infinity inputs
- **Cross-platform determinism**: Scalar fallback for consensus-critical code
- **82 passing tests**: Comprehensive test coverage

### Performance
- Single snap: ~100 ns average
- Batch SIMD: ~74 ns/op average
- Manifold build (density=200): ~2.8 ms
- Memory usage: ~80 KB for density=200

### Dependencies
- Zero runtime dependencies (pure Rust)
- `rand` as dev-dependency for testing

---

## Version Compatibility

| Core Version | Rust Version | Python Version | Notes |
|--------------|--------------|----------------|-------|
| 1.0.1 | 1.75+ | 1.0.0+ | Current stable |
| 1.0.0 | 1.75+ | 1.0.0+ | Initial release |

---

## Roadmap

### Planned for 1.1.0
- AVX-512 support for wider SIMD parallelism
- ARM NEON optimization for Apple Silicon
- Persistent KD-tree serialization
- Approximate mode for sub-50ns operations

### Planned for 2.0.0
- 3D Pythagorean quadruple support
- GPU kernels (CUDA/WebGPU)
- Higher-dimensional extensions (E8 lattice)

---

[1.0.1]: https://github.com/purplepincher/constraint-theory-core/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/purplepincher/constraint-theory-core/releases/tag/v1.0.0
