//! constraint-theory-core WASM — high-level JS wrapper
//!
//! Usage:
//!   import init, { PythagoreanManifold, KDTree, ricciFlowStep } from './index.js';
//!   await init();
//!   const m = new PythagoreanManifold(200);
//!   const { snapped, noise } = m.snap(0.6, 0.8);

import wasm from "./pkg/constraint_theory_core_wasm.js";

let initialized = false;
let mod = null;

/** Load and cache the WASM module. Idempotent. */
export async function init(moduleOrPath) {
  if (initialized) return;
  if (moduleOrPath) {
    mod = await wasm(moduleOrPath);
  } else {
    mod = await wasm();
  }
  initialized = true;
}

/** Ensure the module is loaded; throw if not. */
function requireMod() {
  if (!initialized) throw new Error(
    "constraint-theory-core WASM not initialized — call await init() first",
  );
  return mod;
}

// ── Re-export everything from the raw wasm-bindgen module ──

export {
  PythagoreanManifold,
  PythagoreanTriple,
  KDTree,
  RicciFlow,
} from "./pkg/constraint_theory_core_wasm.js";

// ── Wrapped functions ──

/**
 * Snapping convenience wrapper.
 * @param {PythagoreanManifold} manifold
 * @param {[number, number]} vector
 * @returns {{ snapped: Float32Array, noise: number }}
 */
export function snap(manifold, [x, y]) {
  return manifold.snap(x, y);
}

/**
 * ricci_flow_step exposed with camelCase name.
 * @param {number} curvature
 * @param {number} alpha
 * @param {number} target
 * @returns {number}
 */
export function ricciFlowStep(curvature, alpha, target) {
  return requireMod().ricci_flow_step(curvature, alpha, target);
}

/**
 * hidden_dimensions exposed with camelCase name.
 * @param {number} epsilon
 * @returns {number}
 */
export function hiddenDimensions(epsilon) {
  return requireMod().hidden_dimensions(epsilon);
}

/**
 * max_angular_error_for_states exposed with camelCase name.
 * @param {number} stateCount
 * @returns {number}
 */
export function maxAngularErrorForStates(stateCount) {
  return requireMod().max_angular_error_for_states(stateCount);
}

/** @returns {string} */
export function getVersion() {
  return requireMod().version();
}

export default init;
