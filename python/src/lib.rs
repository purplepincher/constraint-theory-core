// constraint-theory-python — PyO3 bindings for constraint-theory-core
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use constraint_theory_core::kdtree::KDTree as CoreKDTree;
use constraint_theory_core::manifold::{PythagoreanManifold as CoreManifold, PythagoreanTriple as CoreTriple};

// ─── PythagoreanTriple ────────────────────────────────────────────

#[pyclass(name = "PythagoreanTriple", module = "constraint_theory_python")]
#[derive(Clone)]
struct PyTriple {
    inner: CoreTriple,
}

#[pymethods]
impl PyTriple {
    #[new]
    fn new(a: f32, b: f32, c: f32) -> Self {
        PyTriple { inner: CoreTriple::new(a, b, c) }
    }

    #[getter]
    fn a(&self) -> f32 { self.inner.a }

    #[getter]
    fn b(&self) -> f32 { self.inner.b }

    #[getter]
    fn c(&self) -> f32 { self.inner.c }

    fn is_valid(&self) -> bool { self.inner.is_valid() }

    fn to_vector(&self) -> [f32; 2] { self.inner.to_vector() }

    fn __repr__(&self) -> String {
        format!("PythagoreanTriple({}, {}, {})", self.inner.a, self.inner.b, self.inner.c)
    }
}

// ─── PythagoreanManifold ──────────────────────────────────────────

#[pyclass(name = "PythagoreanManifold", module = "constraint_theory_python")]
struct PyManifold {
    inner: CoreManifold,
}

#[pymethods]
impl PyManifold {
    #[new]
    fn new(density: usize) -> Self {
        PyManifold {
            inner: CoreManifold::new(density),
        }
    }

    fn state_count(&self) -> usize {
        self.inner.state_count()
    }

    fn states(&self) -> Vec<[f32; 2]> {
        self.inner.states().to_vec()
    }

    fn validate_input(&self, vector: [f32; 2]) -> PyResult<bool> {
        self.inner.validate_input(vector)
            .map(|_| true)
            .map_err(|e| PyValueError::new_err(e))
    }

    fn snap(&self, vector: [f32; 2]) -> ([f32; 2], f32) {
        constraint_theory_core::manifold::snap(&self.inner, vector)
    }

    fn max_angular_error(&self) -> f32 {
        self.inner.max_angular_error()
    }

    fn __repr__(&self) -> String {
        format!("PythagoreanManifold(states={})", self.inner.state_count())
    }
}

// ─── KDTree ───────────────────────────────────────────────────────

#[pyclass(name = "KDTree", module = "constraint_theory_python")]
struct PyKDTree {
    inner: CoreKDTree,
}

#[pymethods]
impl PyKDTree {
    #[staticmethod]
    fn build(points: Vec<[f32; 2]>) -> Self {
        PyKDTree {
            inner: CoreKDTree::build(&points),
        }
    }

    fn nearest(&self, query: [f32; 2]) -> PyResult<([f32; 2], usize, f32)> {
        self.inner.nearest(&query)
            .map(|(pt, idx, dist)| (pt, idx, dist))
            .ok_or_else(|| PyValueError::new_err("No points in KD-tree"))
    }

    fn nearest_k(&self, query: [f32; 2], k: usize) -> Vec<([f32; 2], usize, f32)> {
        self.inner.nearest_k(&query, k)
            .into_iter()
            .map(|(pt, idx, dist)| (pt, idx, dist))
            .collect()
    }

    fn size(&self) -> usize { self.inner.size() }

    fn __repr__(&self) -> String {
        format!("KDTree(size={})", self.inner.size())
    }
}

// ─── Free functions ───────────────────────────────────────────────

#[pyfunction]
fn snap(manifold: &PyManifold, vector: [f32; 2]) -> ([f32; 2], f32) {
    constraint_theory_core::manifold::snap(&manifold.inner, vector)
}

#[pyfunction]
fn ricci_flow_step(curvature: f32, alpha: f32, target: f32) -> f32 {
    constraint_theory_core::curvature::ricci_flow_step(curvature, alpha, target)
}

// ─── Module ───────────────────────────────────────────────────────

#[pymodule]
fn constraint_theory_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTriple>()?;
    m.add_class::<PyManifold>()?;
    m.add_class::<PyKDTree>()?;
    m.add_function(wrap_pyfunction!(snap, m)?)?;
    m.add_function(wrap_pyfunction!(ricci_flow_step, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
