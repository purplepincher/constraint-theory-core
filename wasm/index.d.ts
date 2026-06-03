// Type definitions for constraint-theory-core WASM bindings v2.2.0
// Project: https://github.com/SuperInstance/constraint-theory-core

/* ─────────────── Module initialisation ─────────────── */

/**
 * Load and cache the WASM module. Safe to call multiple times.
 * @param moduleOrPath - Optional WASM module / path override.
 */
export function init(moduleOrPath?: unknown): Promise<void>;

/* ─────────────── PythagoreanTriple ─────────────── */

export class PythagoreanTriple {
  /** Construct (a, b, c). */
  constructor(a: number, b: number, c: number);

  a(): number;
  b(): number;
  c(): number;

  /** True when a² + b² ≈ c² (within 1e-6). */
  is_valid(): boolean;

  /** Normalised 2-D vector [a/c, b/c]. */
  to_vector(): Float32Array;
}

/* ─────────────── PythagoreanManifold ─────────────── */

export interface SnapResult {
  /** The nearest Pythagorean coordinate. */
  snapped: Float32Array; // length 2
  /** 1 − resonance (dot product). 0 = perfect match. */
  noise: number;
}

export interface SnapError {
  /** Human-readable error message. */
  error: string;
}

/**
 * Pre-computed Pythagorean manifold.
 *
 * ```ts
 * const m = new PythagoreanManifold(200);
 * const { snapped, noise } = m.snap(0.6, 0.8);
 * ```
 */
export class PythagoreanManifold {
  /**
   * Build a new manifold.
   * @param density - Number of generator states (50–500 recommended).
   */
  constructor(density: number);

  /** Total valid states stored in the manifold. */
  state_count(): number;

  /**
   * Snap a 2-D vector to the nearest Pythagorean triple.
   * Returns `SnapResult`. Noise == 1.0 signals invalid input (NaN / Inf).
   */
  snap(x: number, y: number): SnapResult;

  /**
   * Consensus-safe snap with explicit error reporting.
   * On success returns `SnapResult`.
   * On failure returns `SnapError`.
   */
  snap_checked(x: number, y: number): SnapResult | SnapError;

  /** Worst-case angular deviation in radians for this density. */
  max_angular_error(): number;

  /**
   * Validate an input vector. Returns `null` on success,
   * or a string explaining why the input would produce
   * non-deterministic results.
   */
  validate_input(x: number, y: number): string | null;

  /**
   * Recommended noise threshold for common use-cases.
   * - "animation"   → 0.02
   * - "game"        → 0.05
   * - "robotics"    → 0.01
   * - "ml"          → 0.03
   * - "consensus"   → 0.10
   */
  static recommended_noise_threshold(useCase: string): number;
}

/* ─────────────── KDTree ─────────────── */

export interface NearestResult {
  /** Nearest 2-D point. */
  point: Float32Array;
  /** Original index in the build data. */
  index: number;
  /** Squared Euclidean distance. */
  distanceSq: number;
}

/**
 * 2-D KD-tree for O(log N) nearest-neighbour queries.
 *
 * ```ts
 * const tree = KDTree.build(new Float32Array([1,0, 0,1, 0.6,0.8]));
 * const hit = tree.nearest(0.59, 0.81);
 * // hit → { point: Float32Array[0.6, 0.8], index: 2, distanceSq: 0.0002 }
 * ```
 */
export class KDTree {
  /**
   * Build from a flat Float32Array of interleaved [x, y, x, y, …].
   */
  static build(data: Float32Array): KDTree;

  /**
   * Find the single nearest neighbour.
   * Returns `NearestResult` or `null` when the tree is empty.
   */
  nearest(x: number, y: number): NearestResult | null;

  /**
   * Find up to `k` nearest neighbours, sorted by distance.
   */
  nearest_k(x: number, y: number, k: number): NearestResult[];

  /** Number of indexed points. */
  size(): number;

  /** True when the tree contains no points. */
  is_empty(): boolean;
}

/* ─────────────── RicciFlow ─────────────── */

/**
 * Ricci-flow state machine — evolves curvatures toward a target.
 */
export class RicciFlow {
  /**
   * @param alpha          - Learning rate (0.0 – 1.0).
   * @param targetCurvature - Target curvature to converge toward.
   */
  constructor(alpha: number, targetCurvature: number);

  /** Create with defaults: alpha=0.1, targetCurvature=0.0. */
  static with_defaults(): RicciFlow;

  /**
   * Evolve curvatures in-place for `steps` iterations.
   * Mutates the supplied Float32Array.
   */
  evolve(curvatures: Float32Array, steps: number): void;
}

/* ─────────────── Free functions ─────────────── */

/**
 * Single Ricci-flow evolution step:
 * `curvature + alpha * (target - curvature)`.
 */
export function ricci_flow_step(
  curvature: number,
  alpha: number,
  target: number,
): number;

/** CamelCase alias of `ricci_flow_step`. */
export function ricciFlowStep(
  curvature: number,
  alpha: number,
  target: number,
): number;

/**
 * Snapping convenience wrapper.
 * Accepts a destructureable 2-element array instead of separate x, y args.
 */
export function snap(
  manifold: PythagoreanManifold,
  vector: [number, number],
): SnapResult;

/**
 * Hidden dimensions required for target precision:
 * k = ⌈log₂(1/ε)⌉.
 */
export function hidden_dimensions(epsilon: number): number;

/** CamelCase alias of hidden_dimensions. */
export function hiddenDimensions(epsilon: number): number;

/**
 * Maximum angular error (radians) for a given number of valid states.
 */
export function max_angular_error_for_states(stateCount: number): number;

/** CamelCase alias of max_angular_error_for_states. */
export function maxAngularErrorForStates(stateCount: number): number;

/** Library version string (e.g. "2.2.0"). */
export function version(): string;

/** CamelCase alias of version. */
export function getVersion(): string;

export default init;
