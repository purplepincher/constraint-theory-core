# constraint-theory-python

Python bindings for [constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core) — deterministic manifold snapping with O(log n) KD-tree indexing.

## Quick Start

```python
from constraint_theory_python import snap, PythagoreanManifold, KDTree

manifold = PythagoreanManifold(limit=10)
snapped, error = snap(manifold, [0.5, 0.3])
print(f"Original [0.5, 0.3] → snapped {snapped}, error {error:.6f}")

kdtree = KDTree.build([[1.0, 0.0], [0.0, 1.0], [0.707, 0.707]])
nearest, idx, dist = kdtree.nearest([0.6, 0.8])
print(f"Nearest: {nearest}, index: {idx}, dist²: {dist}")
```

## API

| Function | Description |
|----------|-------------|
| `snap(manifold, vector)` | Snap a 2D vector to nearest Pythagorean triple |
| `PythagoreanManifold(limit)` | Manifold of Pythagorean triples up to `limit` |
| `PythagoreanTriple(a, b, c)` | A primitive Pythagorean triple |
| `KDTree.build(points)` | Build a KD-tree from 2D points |
| `KDTree.nearest(query)` | Find nearest point (returns point, index, distance²) |
| `ricci_flow_step(curvature, alpha, target)` | Single Ricci flow step |
