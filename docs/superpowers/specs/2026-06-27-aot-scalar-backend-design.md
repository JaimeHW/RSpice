# AOT Scalar Backend Design

## Goal

Generate Rust for Verilog-A compact models that runs as close as practical to a hand-written native semiconductor model. The primary metric is wall-clock time a user waits for simulation. Compile time, generated code size, and implementation complexity are secondary guardrails only.

This design is for the AOT generated Rust path. The current Cranelift JIT remains in place and is intentionally out of scope for this phase.

## Performance Contract

The generated Newton hot path must converge on these invariants:

- No `AdValue` construction or generic derivative containers for operations that the scalar backend supports.
- No text/string rewriting pass as the primary optimizer.
- Direct scalar SSA-style value graph for primal values.
- Separate sparse derivative graph keyed by circuit unknown lanes.
- Fused residual and Jacobian schedules when they share subexpressions.
- Hoisted schedules for values invariant across instance setup, temperature, timestep, operating point, Newton iteration, AC frequency, noise frequency, and operating-point reporting.
- Direct scalar stores into RHS, charge/current accumulators, and Jacobian stamp slots.
- Conservative IEEE-preserving arithmetic by default; no unsafe fast-math rewrites unless a model or backend mode explicitly opts in.

The existing backend can remain as fallback during migration, but fallback must be visible in tests and metrics. Unsupported constructs should not silently fall back in a way that hides hot-path coverage gaps.

## Pipeline

The target pipeline is:

```text
Verilog-A AST
  -> semantic analysis
  -> HIR
  -> MIR
  -> OptGraph builder
  -> automatic differentiation lowering
  -> scalar optimizer
  -> invalidation scheduler
  -> Rust scalar emitter
```

The important change is that optimization moves from emitted Rust text into typed compiler IR. Generated Rust becomes a mechanical lowering of an already-optimized scalar schedule.

## Core IR

The scalar backend owns a backend-ready OptIR layer:

- `ScalarValueId`: dense index into scalar values.
- `DerivativeLane`: one circuit unknown lane, usually a node or branch axis.
- `ScalarType`: real, integer, boolean, and event/control values needed by supported Verilog-A constructs.
- `ScalarOp`: constants, parameters, state reads, terminal potentials, branch flows, unary math, binary math, comparisons, selects, limited functions, table reads, and analysis/time probes.
- `DerivativeOp`: sparse derivative operation associated with one primal value and one derivative lane.
- `StampTarget`: direct RHS, matrix, charge, flux, noise, or operating-point report target.
- `ScheduleRegion`: setup, temperature, timestep, operating-point static, Newton, AC frequency, noise frequency, and report regions.
- `ScalarSchedule`: topologically ordered value and derivative operations plus direct stamp stores for one region.
- `EmissionPlan`: final Rust function partitioning, scratch layout, and generated helper requirements.

All value ids are dense and topologically ordered before emission. All operands must refer to earlier ids unless an operation is explicitly marked as a phi/control merge. Derivative lanes are sparse and sorted.

## Optimization Strategy

The first-class optimization target is fewer executed floating-point operations and fewer memory touches in the Newton loop. Required optimizations include:

- Constant folding and literal canonicalization.
- Copy propagation and dead value elimination.
- Common subexpression elimination across residual and derivative expressions.
- Algebraic normalization for safe identities such as `x + 0`, `x * 1`, `x - 0`, and `x / 1`.
- Strength reductions that preserve IEEE behavior for normal model domains.
- Direct derivative formulas for supported unary and binary operations.
- Derivative-lane pruning when operands are independent of an unknown.
- Branch/guard splitting so invariant code does not execute inside per-iteration paths.
- Region hoisting driven by dependency analysis rather than by emitted syntax.
- Direct stamp slot aggregation to avoid temporary `AdValue` residual objects.
- Helper specialization only when it reduces generated code and does not put generic containers back in the hot path.

The optimizer must prefer explicit scalar code over abstraction in the generated Newton path. The Rust compiler should see plain `f64` locals, simple branches, and direct calls to `libm`/intrinsics for elementary functions.

## Rust Emission

The scalar emitter generates Rust from `ScalarSchedule` and `EmissionPlan`.

Generated code should:

- Use `f64` locals for scalar values.
- Use compact scratch arrays only for values that cross schedule boundaries or are too large to keep as locals.
- Emit derivative lanes as named scalar locals when profitable and dense slices only when the lane count justifies it.
- Emit direct Jacobian/RHS updates with known indices.
- Partition very large models into deterministic block functions to keep Rust compile time and LLVM optimization stable.
- Avoid helper calls in the Newton hot path unless the helper is `#[inline(always)]`, scalar, and measurably equivalent to inline code.

The existing `AdValue` support remains available for migration and correctness fallback, but it is not the desired output for supported compact-model operations.

## Verification

Every transformation that changes algebraic form needs tests. The minimum verification set is:

- OptIR verifier tests for dense ids, topological order, valid operands, valid derivative lanes, and schedule validity.
- Reference evaluator tests that compare scalar graph evaluation with a straightforward interpreter.
- AD lowering tests for each supported operation and derivative formula.
- Backend equivalence tests against current generated-model fixtures.
- Generated runtime tests for representative diode, BJT, MOS, and large compact models.
- Performance scoreboards that track Newton hot-path operation counts, generated code coverage, and benchmark timing.

Correctness gates are non-negotiable. A faster generated model that changes simulator answers is a regression unless the old answer is proven wrong and the fixture is updated with evidence.

## Migration Plan

The migration should proceed in slices:

1. Extend OptIR with scalar graph data structures, verifier, and tests while preserving current behavior.
2. Add a scalar graph reference evaluator.
3. Lower a small MIR expression subset into scalar OptIR and verify AD formulas.
4. Add a disabled-by-default scalar Rust emitter for simple expression and contribution cases.
5. Route selected generated tests through the scalar backend behind an explicit option.
6. Grow operation coverage and region scheduling until common compact models stop using `AdValue` in the Newton path.
7. Replace text compaction with IR optimizations and make fallback coverage visible in CI.
8. Regenerate devices once scalar coverage is broad enough to produce stable performance wins.

The final target is that high-volume compact models emit scalar Rust that looks structurally similar to a manually optimized native model.
