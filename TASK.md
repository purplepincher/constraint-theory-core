# Task: polish constraint-theory-core to production grade

## Context

This is a fresh fork of `SuperInstance/constraint-theory-core` into
`purplepincher` — a mission-wide research effort found it to be the
single strongest fork candidate discovered across a survey of ~600
previously-unexamined SuperInstance repos: 12.7K LOC Rust, **zero
runtime dependencies**, and 262 passing tests confirmed via real CI
logs (not a masked `|| true` badge). It implements a broad geometric/
constraint-satisfaction toolkit: Pythagorean-triple snapping, KD-tree
spatial indexing, CSP/AC-3/backtracking/CDCL solvers, holonomy
checking, sheaf cohomology, Ricci flow, Laman rigidity, and quantizers.

A separate strategic review (by a large planning model, treat its
conclusion as settled, not open for re-litigation) explicitly
recommended: **fork this crate as-is, and resist the urge to trim the
eclectic math modules on day one — they're tested and inert, and
pruning should follow actual usage, not a first-pass cleanup
instinct.** So: this task is about documentation honesty and CI health,
NOT about deciding which modules "belong." Don't delete working,
tested code because a module seems unrelated to the others — that
judgment call has already been made deliberately.

## What actually needs fixing

### 1. CI is currently broken, for a real (if minor) reason

The CI job runs the same steps against both `stable` and `beta` Rust
toolchains in one matrix. A prior investigation found the **latest CI
run is marked failure because the `beta`-toolchain leg's `cargo
clippy` produces warnings that don't appear on `stable`** — this is
extremely common (beta Rust adds new lints ahead of stable). The
**tests themselves pass on stable** (262 passed, 0 failed, 2 ignored)
— this is not a code-correctness problem, it's a CI-configuration
brittleness problem. Investigate the actual clippy output on beta,
and choose the right fix:
- If the new lints are trivial style nits, fix them (they'll very
  likely also improve the stable build).
- If a genuinely new beta-only lint is overly strict/not yet stable
  policy, it's reasonable to make clippy `continue-on-error: true` (or
  similarly scoped) specifically for the `beta` leg only, while keeping
  it a hard gate on `stable` — document why in a CI comment. Do NOT
  disable clippy entirely or make it non-blocking on stable.
- Whatever you choose, the fix must be verifiable: after your change,
  a fresh clone should show clippy passing (or the beta leg not
  blocking merge) with the reasoning visible in the CI file itself.

### 2. Documentation is bloated with paper-draft/pitch material that doesn't belong in a production crate

The repo currently has, alongside a reasonable `README.md` (234 lines):
`SYNERGY-ANALYSIS.md` (183 lines), `docs/CONVERGENCE-PAPER-DRAFT.md`
(96 lines), `docs/PAPER-V2-ADDITIONS.md` (86 lines), `ONBOARDING.md`
(628 lines!), `DOCKSIDE-EXAM.md`, `docs/DISCLAIMERS.md` (202 lines).
This is a strong instance of the pattern this whole mission has found
repeatedly across SuperInstance: research-in-progress prose living
alongside the actual shipped code, inflating what a visitor has to
read to trust the crate. Read every one of these files, then:
- Keep exactly what's needed for a production crate: a tight README
  (Overview, Install, Quick Start with a real runnable example,
  API surface summary, License), a real `LICENSE`, a `CHANGELOG.md` if
  it's accurate.
- `docs/DISCLAIMERS.md` is worth reading closely — if it contains
  honest, load-bearing caveats about what's proven vs. not (this
  crate's math heritage includes some claims that don't fully hold up
  per the mission's own research — check `SYNERGY-ANALYSIS.md` and the
  paper-draft docs for anything similar to the "Intent-Holonomy
  duality: converse open, 30% confidence" pattern found in a sibling
  repo), fold the *real* caveats into the README's own honesty section
  rather than deleting them — don't lose true information, just stop
  presenting it as a separate paper submission.
- Anything that reads as a paper draft, an ecosystem-integration pitch,
  or speculative synergy analysis rather than documentation of what
  this crate actually does and how to use it: either delete it or move
  it to a clearly-labeled `docs/research-notes/` directory that the
  main README does not require reading. State in your final report
  which you did and why.
- `AGENT.md` and any `agent_context.json` — check whether these are
  the same ensign-scaffold boilerplate seen in other SuperInstance
  forks (a fleet-identity/duty-log template unrelated to this crate's
  actual purpose); if so, remove them the same way other forks in
  this org have.

### 3. General adoption-bar check

- Verify the zero-runtime-dependency claim is still true
  (`cargo tree` or reading `Cargo.toml` — only `rand`/`criterion` as
  dev-deps is what was previously confirmed; re-confirm).
- `Cargo.toml`'s `repository` field, if present, should point to
  `github.com/purplepincher/constraint-theory-core`, not
  `SuperInstance/...`.
- Confirm the Quick Start example in the README actually compiles/runs
  as written (`cargo run --example basic` or similar — there's an
  `examples/` directory).

## Constraints

- Do not delete or gut the actual `src/` implementation modules — see
  the explicit "resist the urge to trim" guidance above. This task is
  docs + CI, not a module-scope debate.
- Commit as you go with clear messages. Leave the worktree on branch
  `polish/production-grade`, fully committed. Do not merge or push
  yourself — that happens after independent verification.
- This worktree only has `origin` configured (the purplepincher fork).
  Do not add or push to any other remote.
