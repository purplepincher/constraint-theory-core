//! WASM bindings for constraint-theory-core.
//!
//! Exposes PythagoreanManifold, KDTree, RicciFlow, and utility
//! functions to JavaScript / TypeScript via wasm-bindgen.

use wasm_bindgen::prelude::*;

use constraint_theory_core::kdtree::KDTree as NativeKDTree;
use constraint_theory_core::manifold::PythagoreanManifold as NativeManifold;
use constraint_theory_core::manifold::PythagoreanTriple as NativeTriple;
use constraint_theory_core::curvature::RicciFlow as NativeRicciFlow;
use constraint_theory_core::CTErr;

// ──────────────────────────────────────────────
//  PythagoreanTriple
// ──────────────────────────────────────────────

/// A Pythagorean triple (a, b, c) where a² + b² = c².
#[wasm_bindgen]
pub struct PythagoreanTriple {
    inner: NativeTriple,
}

#[wasm_bindgen]
impl PythagoreanTriple {
    /// Create a new Pythagorean triple.
    #[wasm_bindgen(constructor)]
    pub fn new(a: f32, b: f32, c: f32) -> PythagoreanTriple {
        PythagoreanTriple {
            inner: NativeTriple::new(a, b, c),
        }
    }

    pub fn a(&self) -> f32 {
        self.inner.a
    }
    pub fn b(&self) -> f32 {
        self.inner.b
    }
    pub fn c(&self) -> f32 {
        self.inner.c
    }

    /// Returns true when a² + b² ≈ c² (within 1e-6).
    pub fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    /// Returns the normalized 2D vector [a/c, b/c].
    pub fn to_vector(&self) -> js_sys::Float32Array {
        let v = self.inner.to_vector();
        float32_array(&v)
    }
}

// ──────────────────────────────────────────────
//  PythagoreanManifold
// ──────────────────────────────────────────────

/// Pre-computes all valid Pythagorean triples up to a given density and
/// provides O(log N) snapping via internal KD-tree lookup.
#[wasm_bindgen]
pub struct PythagoreanManifold {
    inner: NativeManifold,
}

#[wasm_bindgen]
impl PythagoreanManifold {
    /// Build a new manifold. `density` controls the number of generated
    /// states (typically 50–500). Higher density = finer resolution.
    #[wasm_bindgen(constructor)]
    pub fn new(density: usize) -> PythagoreanManifold {
        PythagoreanManifold {
            inner: NativeManifold::new(density),
        }
    }

    /// Number of valid states stored in the manifold.
    pub fn state_count(&self) -> usize {
        self.inner.state_count()
    }

    /// Snap a 2D vector to the nearest Pythagorean triple.
    ///
    /// Returns `{ snapped: Float32Array[2], noise: f32 }`.
    ///
    /// `noise` = 1 − resonance (dot product with nearest).
    /// Noise near zero means an almost-exact match.
    pub fn snap(&self, x: f32, y: f32) -> JsValue {
        let (snapped, noise) = self.inner.snap([x, y]);
        js_snap_result(&snapped, noise)
    }

    /// Like `snap()` but returns an error result for invalid inputs (NaN,
    /// Infinity, zero vector).
    ///
    /// On success: `{ snapped: Float32Array[2], noise: f32 }`
    /// On failure: `{ error: string }`
    pub fn snap_checked(&self, x: f32, y: f32) -> JsValue {
        match self.inner.snap_checked([x, y]) {
            Ok((snapped, noise)) => js_snap_result(&snapped, noise),
            Err(e) => js_err(e),
        }
    }

    /// Worst-case angular deviation (radians) for this manifold density.
    pub fn max_angular_error(&self) -> f32 {
        self.inner.max_angular_error()
    }

    /// Validate a 2D vector before snapping (consensus-critical use).
    /// Returns `null` on success, or a string describing the issue.
    pub fn validate_input(&self, x: f32, y: f32) -> Option<String> {
        self.inner.validate_input([x, y]).err().map(|s| s.to_string())
    }

    /// Suggested noise threshold for a given use-case string:
    /// "animation" (0.02), "game" (0.05), "robotics" (0.01),
    /// "ml" (0.03), "consensus" (0.1).
    pub fn recommended_noise_threshold(use_case: &str) -> f32 {
        NativeManifold::recommended_noise_threshold(use_case)
    }
}

// ──────────────────────────────────────────────
//  KDTree
// ──────────────────────────────────────────────

/// 2D KD-tree for fast O(log N) nearest-neighbour queries.
#[wasm_bindgen]
pub struct KDTree {
    inner: NativeKDTree,
}

#[wasm_bindgen]
impl KDTree {
    /// Build a KD-tree from a flat `Float32Array` of interleaved
    /// `[x, y, x, y, …]` values.
    #[wasm_bindgen]
    pub fn build(data: &[f32]) -> KDTree {
        let points: Vec<[f32; 2]> = data
            .chunks_exact(2)
            .map(|c| [c[0], c[1]])
            .collect();
        KDTree {
            inner: NativeKDTree::build(&points),
        }
    }

    /// Returns the nearest point as
    /// `{ point: Float32Array[2], index: number, distanceSq: number }`,
    /// or `null` when the tree is empty.
    pub fn nearest(&self, x: f32, y: f32) -> JsValue {
        match self.inner.nearest(&[x, y]) {
            Some((point, idx, dist_sq)) => {
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"point".into(), &float32_array(&point)).ok();
                js_sys::Reflect::set(&obj, &"index".into(), &JsValue::from(idx as u32)).ok();
                js_sys::Reflect::set(&obj, &"distanceSq".into(), &JsValue::from(dist_sq)).ok();
                obj.into()
            }
            None => JsValue::NULL,
        }
    }

    /// Returns up to `k` nearest neighbours as an array of:
    /// `{ point: Float32Array[2], index: number, distanceSq: number }`.
    pub fn nearest_k(&self, x: f32, y: f32, k: usize) -> JsValue {
        let items = self.inner.nearest_k(&[x, y], k);
        let arr = js_sys::Array::new();
        for (point, idx, dist_sq) in items {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"point".into(), &float32_array(&point)).ok();
            js_sys::Reflect::set(&obj, &"index".into(), &JsValue::from(idx as u32)).ok();
            js_sys::Reflect::set(&obj, &"distanceSq".into(), &JsValue::from(dist_sq)).ok();
            arr.push(&obj);
        }
        arr.into()
    }

    /// Number of points indexed by the tree.
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    /// True when the tree contains no points.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

// ──────────────────────────────────────────────
//  RicciFlow / ricci_flow_step
// ──────────────────────────────────────────────

/// Ricci flow evolution state machine.
#[wasm_bindgen]
pub struct RicciFlow {
    inner: NativeRicciFlow,
}

#[wasm_bindgen]
impl RicciFlow {
    /// Create a new Ricci flow with the given `alpha` (learning rate,
    /// 0.0–1.0) and `targetCurvature`.
    #[wasm_bindgen(constructor)]
    pub fn new(alpha: f32, target_curvature: f32) -> RicciFlow {
        RicciFlow {
            inner: NativeRicciFlow::new(alpha, target_curvature),
        }
    }

    /// Convenience constructor with alpha=0.1, targetCurvature=0.0.
    pub fn with_defaults() -> RicciFlow {
        RicciFlow {
            inner: NativeRicciFlow::with_defaults(),
        }
    }

    /// Evolve an array of curvature values in-place for the given number
    /// of steps.  Mutates the input Float32Array.
    pub fn evolve(&mut self, curvatures: &mut [f32], steps: usize) {
        self.inner.evolve(curvatures, steps);
    }
}

/// Single step of Ricci flow: returns `curvature + alpha * (target - curvature)`.
#[wasm_bindgen]
pub fn ricci_flow_step(curvature: f32, alpha: f32, target: f32) -> f32 {
    constraint_theory_core::curvature::ricci_flow_step(curvature, alpha, target)
}

// ──────────────────────────────────────────────
//  Standalone helpers
// ──────────────────────────────────────────────

/// Free-standing snap convenience: returns
/// `{ snapped: Float32Array[2], noise: f32 }`.
#[wasm_bindgen]
pub fn snap(manifold: &PythagoreanManifold, x: f32, y: f32) -> JsValue {
    manifold.snap(x, y)
}

/// Hidden dimensions required for target precision.
/// Returns k = ⌈log₂(1/ε)⌉.
#[wasm_bindgen]
pub fn hidden_dimensions(epsilon: f32) -> usize {
    constraint_theory_core::hidden_dimensions(epsilon)
}

/// Maximum angular error (radians) for a given number of valid states.
#[wasm_bindgen]
pub fn max_angular_error_for_states(state_count: usize) -> f32 {
    constraint_theory_core::max_angular_error_for_states(state_count)
}

/// Library version string (e.g. "2.2.0").
#[wasm_bindgen]
pub fn version() -> String {
    constraint_theory_core::VERSION.to_string()
}

// ──────────────────────────────────────────────
//  Internal helpers
// ──────────────────────────────────────────────

fn float32_array(v: &[f32; 2]) -> js_sys::Float32Array {
    let buf = js_sys::Float32Array::new_with_length(2);
    buf.set_index(0, v[0]);
    buf.set_index(1, v[1]);
    buf
}

fn js_snap_result(snapped: &[f32; 2], noise: f32) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"snapped".into(), &float32_array(snapped)).ok();
    js_sys::Reflect::set(&obj, &"noise".into(), &JsValue::from(noise)).ok();
    obj.into()
}

fn js_err(e: CTErr) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"error".into(), &JsValue::from(e.to_string())).ok();
    obj.into()
}
