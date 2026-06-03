# constraint-theory-core WASM bindings

WASM/JS/TS bindings for [constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core) —
a deterministic Pythagorean manifold snapping engine with O(log N) KD-tree indexing,
Ricci flow, and constraint validation.

## Quick start

```bash
npm install @superinstance/constraint-theory-core-wasm
```

```ts
import init, { PythagoreanManifold, ricciFlowStep } from "@superinstance/constraint-theory-core-wasm";

await init();

// --- Pythagorean manifold snapping ---
const manifold = new PythagoreanManifold(200);
const { snapped, noise } = manifold.snap(0.6, 0.8);
// snapped → Float32Array [0.6, 0.8]   (the 3-4-5 triple)
// noise   → ~0                        (exact match)

// --- KD-tree spatial indexing ---
import { KDTree } from "@superinstance/constraint-theory-core-wasm";

const tree = KDTree.build(new Float32Array([1, 0, 0, 1, 0.6, 0.8]));
const hit = tree.nearest(0.59, 0.81);
// hit.point      → Float32Array [0.6, 0.8]
// hit.index      → 2
// hit.distanceSq → ~0.0002

// --- Ricci flow ---
const c = ricciFlowStep(1.0, 0.1, 0.0);
// c → 0.9
```

## API

| Class / Function | Description |
|------------------|-------------|
| `PythagoreanManifold` | Pre-computes valid Pythagorean triples, snaps vectors |
| `PythagoreanTriple`   | (a, b, c) where a² + b² = c² |
| `KDTree`              | 2-D spatial index with nearest / nearest-k queries |
| `RicciFlow`           | Curvature evolution via Ricci flow |
| `ricciFlowStep`       | Single evolution step (`c + α(t−c)`) |
| `hiddenDimensions`    | k = ⌈log₂(1/ε)⌉ |
| `maxAngularErrorForStates` | Worst‑case angular deviation |

## Build from source

```bash
cd constraint-theory-core/wasm
wasm-pack build --target web
```

The output lands in `pkg/`.

## Browser / bundler support

These bindings target the `--target web` output of `wasm-pack`.
Works in modern Chrome, Firefox, Safari, and Edge.

For Node.js use `--target nodejs` (not currently tested).
