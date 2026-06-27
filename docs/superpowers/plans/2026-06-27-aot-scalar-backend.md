# AOT Scalar Backend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the current helper-heavy AOT generated Rust path with a typed scalar OptIR and scalar Rust emitter that minimizes Newton hot-path wall time.

**Architecture:** Extend canonical OptIR into a backend-ready scalar graph, lower MIR/HIR expressions into scalar primal and derivative schedules, optimize those schedules in IR, and emit direct scalar Rust. Keep the current `AdValue` backend as explicit fallback while coverage grows.

**Tech Stack:** Rust, serde canonical IR, existing `rspice-veriloga` compiler crate, existing generated-device integration in `rspice-core`, existing canonical IR tests and generated runtime tests.

---

## Phase 1: Scalar OptIR Foundation

- [x] Add scalar graph types to `crates/rspice-veriloga/src/canonical_ir/opt.rs`.
  - Types: `OptValueKind`, `OptUnaryOp`, `OptBinaryOp`, `DerivativeLane`, `DerivativeLaneKind`, `OptDerivative`, `OptScheduleOp`.
  - Keep existing `OptOp::EvaluateEquation` compatibility until the Rust backend migrates.
  - Preserve serde compatibility for newly generated artifacts.

- [x] Add OptIR verifier checks.
  - Dense `ValueId` ordering.
  - Valid operand ids.
  - Topological operand order.
  - Valid parameter/node/branch/equation references.
  - Sorted and unique derivative lanes per value.
  - Schedule operations reference valid values and equations.

- [x] Add canonical IR tests in `crates/rspice-veriloga/tests/canonical_ir.rs`.
  - Accepts a simple scalar graph.
  - Rejects non-dense scalar ids.
  - Rejects out-of-range operands.
  - Rejects forward operand references.
  - Rejects duplicate derivative lanes.
  - Rejects schedule references to missing values.

## Phase 2: Scalar Reference Evaluator

- [x] Add a reference evaluator module under `crates/rspice-veriloga/src/canonical_ir/opt_eval.rs` or equivalent.
  - Evaluate scalar values from constants, parameters, branch potentials, unary ops, binary ops, comparisons, and selects.
  - Evaluate derivative lanes from explicit derivative graph operations.
  - Keep evaluator straightforward and unoptimized; it is a correctness oracle, not the production runtime.

- [x] Add tests comparing evaluator output against direct arithmetic for simple Verilog-A expression fixtures.
  - Resistor current: `V(p,n) / r`.
  - Diode-like expression: `is * (limexp(vd / vt) - 1)`.
  - Conditional expression with branch split.

## Phase 3: MIR/HIR to Scalar Graph Lowering

- [ ] Build scalar graph lowering for a conservative expression subset.
  - Numbers, real parameters, identifiers, branch potentials, unary `+`/`-`, binary `+`, `-`, `*`, `/`, comparisons, ternary conditional, selected math calls.
  - Emit unsupported constructs as explicit fallback markers.

- [ ] Add AD lowering for supported operations.
  - Direct sparse derivative formulas.
  - Lane pruning for independent operands.
  - Reuse primal subexpressions for derivative formulas where profitable.

- [ ] Add lowering tests from Verilog-A fixtures.
  - Verify expected primal graph shape.
  - Verify expected derivative lanes for terminal voltages.
  - Verify fallback markers for unsupported constructs.

## Phase 4: Scalar Optimizer

- [ ] Implement IR-level cleanup passes.
  - Constant folding.
  - Copy propagation.
  - Dead value elimination.
  - Safe algebraic identities.
  - CSE with commutative normalization.

- [ ] Implement derivative-specific passes.
  - Zero-lane pruning.
  - Common derivative subexpression elimination.
  - Fused residual/Jacobian value reuse.

- [ ] Add pass tests.
  - Each rewrite needs a positive test and a no-rewrite test for unsafe cases.

## Phase 5: Invalidation Scheduling

- [ ] Add dependency classification for scalar values.
  - Instance-static, temperature-static, timestep-static, operating-point-static, Newton, AC frequency, noise frequency, report.

- [ ] Build `ScalarSchedule` partitioning from dependency classes.
  - Hoist constants and parameters out of Newton.
  - Keep terminal voltage/current probes in Newton.
  - Preserve schedule order and explicit data movement across regions.

- [ ] Add tests proving invariant expressions leave the Newton region.

## Phase 6: Scalar Rust Emitter

- [ ] Add a new Rust backend emitter beside the current backend.
  - Emit `f64` locals for primal values.
  - Emit sparse derivative locals for Jacobian lanes.
  - Emit direct RHS and matrix stamp updates.
  - Partition large schedules deterministically.

- [ ] Add an explicit backend selection option.
  - Default remains current backend until scalar coverage is broad enough.
  - Tests can force scalar backend for supported fixtures.

- [ ] Add generated Rust compile tests for scalar-backed simple models.

## Phase 7: Model Migration and Performance Gates

- [ ] Migrate generated model classes incrementally.
  - Start with resistor/diode-like fixtures.
  - Move to compact MOS/BJT fixtures once AD and scheduling are mature.
  - Avoid `vbic_spice_legacy`; another agent owns that work.

- [ ] Add performance reporting.
  - Count generated `AdValue` hot-path uses.
  - Count scalar operations, derivative lanes, helper calls, RHS stores, and matrix stores.
  - Benchmark representative generated devices before and after scalar backend routing.

- [ ] Regenerate devices after coverage reaches clear wins.
  - Commit generated source separately from compiler changes.
  - Do not stage unrelated dirty main-checkout changes.

## Phase 8: Make Scalar Backend Primary

- [ ] Promote scalar backend to default for covered models.
- [ ] Fail loudly or report coverage when fallback is used in performance-sensitive regions.
- [ ] Remove reliance on `compact_generated_stamp_surface` for optimization once equivalent IR passes exist.
- [ ] Keep legacy backend only for unsupported constructs until parity is complete.

## Checkpoints

- [ ] `cargo test -p rspice-veriloga canonical_ir --test canonical_ir`
- [ ] `cargo test -p rspice-veriloga`
- [ ] Generated runtime tests for scalar-enabled fixtures.
- [ ] Full relevant workspace tests before large generated-device commits.
- [ ] Push each granular checkpoint branch commit.
