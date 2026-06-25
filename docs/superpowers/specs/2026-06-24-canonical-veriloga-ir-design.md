# Canonical Verilog-A IR Design

> **Status update (2026-06-25):** Canonical IR has moved from design into the
> `rspice-veriloga` implementation, and a first Rust backend now consumes it for
> feature-gated generated built-ins. The non-goals below describe the original
> IR slice, not the current repository state.

## Purpose

RSpice needs a single canonical compiler target for Verilog-A device support. The current `rspice-veriloga` pipeline lowers analyzed modules into `DeviceIR`, then immediately emits bytecode-oriented `CompiledModel` programs with an optional Cranelift JIT layered over that bytecode. That structure has served as an enabling path, but it is not the right long-term commercial contract for generated Rust devices, an optimized custom x64/ARM JIT, reproducible model artifacts, or supportable diagnostics.

This design introduces a new multi-level canonical IR pipeline. The new IR becomes the semantic source of truth for Verilog-A compilation. Existing bytecode and Cranelift paths are legacy migration backends only; new architecture work targets the canonical IR.

The first implementation slice is intentionally scoped to the IR system itself: data model, lowering boundaries, validation, serialization, diagnostics, optimization invariants, and backend contracts. Verilog-A to Rust transpilation and the custom JIT are downstream workstreams that consume the scheduled optimized IR once this contract is stable.

## Requirements

- Provide one canonical compiler target for all future Verilog-A execution paths: generated Rust, custom native JIT, diagnostic dumps, and temporary legacy adapters.
- Preserve Verilog-A semantics explicitly before optimization: modules, ports, disciplines, parameters, ranges, aliases, internal nodes, branches, analog operators, contributions, events, noise, source spans, and system functions.
- Represent device equations in a backend-independent form suitable for residual evaluation, analytic Jacobian evaluation, reactive charge/flux stamping, AC small-signal behavior, noise analysis, operating-point reporting, and transient timestep control.
- Make optimization deliberate and verifiable. Each lowering or optimization phase must have a verifier that checks structural and semantic invariants.
- Separate instance-static, temperature-static, timestep-static, operating-point-static, and Newton-iteration values so hot device evaluation avoids repeated work.
- Support stable serialization for cache artifacts, reproducible builds, diagnostics, and future offline compilation.
- Keep names and source spans for diagnostics while using dense typed IDs and arenas for compiler internals and hot backend lowering.
- Reject unsupported or ambiguous semantics with hard diagnostics. Do not silently fall back to finite differences, ignored model physics, or backend-specific approximations.
- Preserve numerical limiting and discontinuity semantics. Optimizations must not move stateful analog operators, event operators, `$limit`, `limexp`, or table boundary behavior across invalid boundaries.

## Non-Goals

- This design does not implement the generated Rust backend.
- This design does not implement the custom x64/ARM JIT.
- This design does not remove the current bytecode runtime immediately.
- This design does not claim CMC model support. CMC coverage requires generated Rust or another canonical-IR backend plus oracle-backed engine tests.
- This design does not redesign the SPICE solver, matrix infrastructure, egui UI, or model-library UI.

## Architecture

The new compiler flow is:

```text
Verilog-A source
  -> preprocessor / lexer / parser
  -> semantic analysis
  -> HIR: Verilog-A semantic model
  -> MIR: normalized device-equation model
  -> OptIR: typed optimization and scheduling model
  -> backend lowering: generated Rust, custom JIT, diagnostics, legacy adapter
```

The important split is semantic preservation first and execution optimization second. HIR preserves what the Verilog-A source means. MIR converts that meaning into explicit simulator equations. OptIR is the performance layer consumed by backends.

### HIR

HIR is the high-level semantic IR produced after the existing semantic analyzer. It should be close enough to the source to give excellent diagnostics, but normalized enough that later passes do not need to inspect raw AST details.

HIR contains:

- Module identity, source package metadata, source digest, compiler version, and source spans.
- Ports, disciplines, internal nodes, named branches, variables, arrays, parameters, localparams, aliases, ranges, exclusions, attributes, and declaration order.
- Analog statements, contribution statements, control-flow regions, event controls, analog functions after inlining decisions, and system function calls.
- Verilog-A concepts that must remain visible for diagnostics: `$param_given`, `$port_connected`, `$temperature`, `$vt`, `$abstime`, `$bound_step`, `$discontinuity`, `$limit`, `ddt`, `idt`, `idtmod`, `ddx`, noise functions, filters, delay, transition, slew, crossing, and timer behavior.

HIR validation checks source-level legality. It rejects unsupported constructs before they become lower-level compiler problems.

### MIR

MIR is the normalized device-equation IR. It removes source syntax as a concern and expresses the model as explicit simulator semantics.

MIR contains:

- Dense IDs for ports, nodes, internal unknowns, branch-current unknowns, parameters, state slots, equations, contributions, noise sources, expressions, and regions.
- Explicit current and potential contributions with row/column participation, sign, activation guard, analysis-domain behavior, and branch-current coupling.
- Internal nodes and named branches as first-class equation participants.
- State slots for `ddt`, `idt`, `idtmod`, filters, delays, slew/transition, crossing detectors, timers, `$limit`, and other stateful analog operators.
- Reactive operands and derivative targets for transient and AC behavior.
- Noise injection points with current/potential polarity, PSD expressions, flicker exponent expressions, noise-table data, labels, and activation linkage.
- Parameter default programs, alias resolution, range metadata, `$param_given` behavior, and static structure guards.
- Analytic derivative requests and derivative graphs. Unsupported derivatives are compile errors, not runtime surprises.

MIR validation checks equation completeness, derivative coverage, state-slot consistency, branch-current structural stamps, analysis guard consistency, and source-span coverage for diagnostics.

### OptIR

OptIR is the backend-independent optimization IR. It should use typed value graphs and scheduled regions instead of recursive expression trees.

OptIR contains:

- Typed `ValueId` graphs with explicit pure and effectful operations.
- Structured control regions and bounded loops.
- Side-effect tokens for stateful operators so scheduling cannot reorder them incorrectly.
- Precompute regions partitioned by invalidation class: instance-static, temperature-static, timestep-static, operating-point-static, Newton-iteration, AC frequency, noise frequency, and operating-point report.
- Fused residual and Jacobian value graphs so common subexpressions and derivative intermediates are shared.
- Backend-ready memory layouts for parameters, variables, states, ports, internal nodes, branch currents, table data, and report values.
- Scheduled evaluation plans for setup, temperature update, transient state update, Newton residual/Jacobian, AC/reactive stamping, noise, bound-step/discontinuity, and operating-point reporting.

OptIR validation checks type correctness, dominance, effect ordering, schedule completeness, invalidation boundaries, derivative consistency, and backend contract requirements.

## Data Model

The canonical IR should use typed IDs and arenas rather than string lookups or recursive ownership in hot compiler paths. Human-readable names live in side tables.

Recommended ID families:

- `ModuleId`
- `SourceId`
- `SymbolId`
- `PortId`
- `DisciplineId`
- `ParamId`
- `VariableId`
- `ArrayId`
- `NodeId`
- `BranchId`
- `BranchUnknownId`
- `StateId`
- `EquationId`
- `ContributionId`
- `NoiseSourceId`
- `RegionId`
- `ExprId`
- `ValueId`
- `ScheduleId`

Core metadata tables:

- Source package and license/provenance identity.
- Source digest and include dependency digests.
- Compiler schema version and feature flags.
- Names, source spans, diagnostics labels, attributes, units, and discipline metadata.
- Port and parameter order as declared by the source model.
- Stable public model ABI metadata for generated artifacts.

The serialized artifact should include schema version, source package identity, source digest, compiler version, target feature requirements, pass pipeline version, HIR/MIR/OptIR checksums, diagnostics metadata, and optional textual dumps. Binary serialization should be deterministic. Textual dumps should be stable enough for snapshot tests and support investigations.

## Optimization Pipeline

Optimization runs in declared stages. Every stage produces verifiable IR.

### Stage 1: HIR Validation

Validate source semantics: declarations, contribution legality, analog operator placement, parameter defaults, ranges, aliases, units or discipline compatibility where available, unsupported constructs, and source spans.

### Stage 2: MIR Normalization

Convert HIR statements into explicit equations, state slots, analysis guards, branch-current unknowns, noise sources, reactive operands, and derivative targets. Normalize contribution polarity and structural stamps here, before backend lowering.

### Stage 3: Static Specialization

Fold constants, resolve parameter defaults, peel instance-static conditions, prune inactive structure when parameters prove it unreachable, and split setup/precompute code from hot Newton evaluation.

### Stage 4: Algebraic Optimization

Apply canonical common subexpression elimination, copy propagation, dead value elimination, strength reduction, derivative graph reuse, safe algebraic simplification, branch-equation simplification, lookup-table canonicalization, and reassociation only when the numerical contract allows it.

### Stage 5: Numerical Safety

Preserve model-intended limiting, discontinuity, state, and table behavior. Do not move or duplicate effectful operations unless the operation explicitly permits it. This pass checks that earlier optimizations did not break analog semantics.

### Stage 6: Scheduling And Layout

Produce backend-ready schedules and memory layouts. The scheduler should optimize residual and Jacobian evaluation together, because compact-model performance depends heavily on sharing intermediate values across current, charge, and derivative paths.

### Stage 7: Backend Lowering

Backends consume scheduled OptIR. Backend-specific passes may perform register allocation, instruction selection, helper inlining, vectorization, calling convention lowering, and target-specific peepholes. Backends may not reinterpret Verilog-A semantics.

## Backend Contracts

### Generated Rust Backend

The generated Rust backend is the release-quality native path for CMC devices. It consumes scheduled OptIR and emits deterministic, readable Rust modules with provenance metadata, model ABI metadata, setup/precompute functions, hot residual/Jacobian functions, noise functions, AC/reactive functions, and operating-point reporting hooks.

Generated Rust must pass the same IR conformance tests as every other backend.

### Custom JIT Backend

The custom JIT backend consumes scheduled OptIR after generated Rust has proven the semantic contract. It emits x64 and ARM machine code through a RSpice-owned backend. The JIT is a performance backend, not a semantic discovery layer.

The JIT must share generated Rust's conformance tests and must support deterministic fallback diagnostics when a target feature is unavailable.

### Diagnostic Backend

The diagnostic backend emits stable HIR, MIR, OptIR, schedule, derivative, and optimization-report dumps. It must identify rejected optimizations and explain verifier failures with source spans whenever possible.

### Legacy Adapter

A temporary adapter may lower scheduled OptIR into the old runtime shape to ease migration. It is not a design center, and it must not constrain canonical IR semantics.

## Runtime Integration

`rspice-core` should continue to own simulator-facing device contracts: node mapping, matrix/RHS stamping, analysis state, temperature, timestep, noise frequency, and result reporting. `rspice-veriloga` should own Verilog-A source semantics, canonical IR construction, optimization, serialization, and backend artifact generation.

The first IR slice should not force a wholesale `VerilogADevice` rewrite. Instead, it should introduce the canonical IR alongside the existing compiler path, then add controlled adapters and equivalence tests. Replacement happens only when tests prove that the new path matches or deliberately improves behavior.

## Error Handling

Compiler diagnostics should be typed, source-spanned, and phase-aware. A diagnostic should say whether the failure occurred during semantic lowering, HIR validation, MIR normalization, derivative generation, optimization, scheduling, serialization, or backend lowering.

Runtime model errors should remain explicit. Out-of-range dynamic array indexes, invalid table data, unsupported analog operator placement, missing derivative support, non-finite required constants, and invalid state configurations should not silently skip work.

Numerical non-finites inside a model must be handled according to the simulator's existing policy for device evaluation. The IR should make it possible to identify the exact contribution, value, derivative, and source span that produced a non-finite result.

## Validation

The IR implementation should land behind focused tests before changing production behavior.

Required gates:

- Unit tests for every IR invariant and verifier error.
- Snapshot tests for HIR, MIR, OptIR, schedules, and diagnostics.
- Lowering tests from small Verilog-A fixtures covering parameters, aliases, internal nodes, current contributions, potential contributions, indirect contributions, `ddt`, `idt`, `$limit`, `ddx`, noise, event controls, arrays, and runtime loops.
- Equivalence tests comparing old `DeviceIR`/bytecode and canonical IR reference evaluation on controlled fixtures during migration.
- Backend-independent reference evaluator tests for OptIR schedules.
- Property-style tests for safe algebraic simplifications where practical.
- CMC compile-frontier tests against packages under `models/veriloga/cmc/`, initially as compile/lowering coverage rather than support claims.
- Performance scoreboards for compile time, optimization time, serialized artifact size, cache hit behavior, residual evaluation throughput, Jacobian evaluation throughput, and generated-code size once backends exist.

The validation policy is evidence-first: no backend replaces existing behavior until it proves equivalence or a reviewed, documented correction.

## Rollout

1. Add canonical IR modules, ID types, arenas, serialization metadata, diagnostics, validators, and dump support.
2. Lower analyzed modules into HIR and prove source semantic preservation with tests.
3. Lower HIR into MIR and prove explicit equation, state, derivative, reactive, and noise semantics.
4. Add OptIR, optimization passes, schedule generation, and the reference evaluator.
5. Add legacy comparison harnesses against existing bytecode fixtures.
6. Add generated Rust backend after the IR contract is stable.
7. Add the custom x64/ARM JIT after generated Rust proves the semantic contract.

## Commercial Quality Criteria

- The compiler has one canonical semantic contract.
- Every transformation is validated by verifiers and tests.
- Serialized artifacts are deterministic and versioned.
- Diagnostics are source-spanned and phase-aware.
- Performance work is measured by checked-in scoreboards.
- Unsupported semantics fail closed.
- Backends share conformance tests.
- Generated artifacts preserve upstream source identity and license/notice provenance.
- The old bytecode and Cranelift backend do not shape new compiler architecture.
