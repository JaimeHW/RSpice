# Native JIT Backend Design

## Purpose

RSpice needs to replace the existing Cranelift-based Verilog-A native path with
a RSpice-owned native JIT. The new backend must target x64 first, keep a clean
path to AArch64, and optimize for runtime machine-code throughput above every
other backend metric.

The approved long-term direction is a canonical-IR native JIT, not a bytecode
JIT. The backend consumes scheduled canonical OptIR, lowers it into a compact
RSpice JIT IR, runs backend-local optimization, and emits target machine code.
Cranelift is removed from source, Cargo manifests, lockfiles, documentation, and
runtime behavior.

This design also changes the runtime contract: there is no interpreter fallback
when native JIT is requested. If a model, canonical IR operation, target
feature, relocation, executable-memory operation, or calling-convention detail
cannot be compiled correctly, native compilation returns a hard typed error and
model construction fails. Unsupported JIT coverage is a backend defect to fix,
not a condition the simulator hides by running interpreted code.

## Requirements

- Remove all Cranelift dependencies and the `cranelift_jit` module.
- Build a native x64 backend owned by RSpice, with architecture boundaries that
  allow an AArch64 backend to be added without rewriting the front end.
- Make runtime performance of emitted machine code the primary optimization
  target. Compile time, code size, and implementation convenience are secondary
  unless they threaten usability or correctness.
- Consume canonical OptIR, not bytecode `CompiledModel` programs, as the
  semantic source of truth for native compilation.
- Compile every required residual, Jacobian, assignment, state, noise, AC, and
  reporting path selected by the canonical schedules for the model being loaded.
- Fail closed on unsupported canonical IR operations, target features,
  executable-memory failures, non-relocatable symbols, verifier failures, or
  ABI mismatches.
- Keep the bytecode interpreter available only for explicit non-native builds
  and test-oracle comparisons. It must not be an automatic substitute for a
  failed native JIT compile.
- Preserve deterministic, phase-aware diagnostics so native compile failures
  identify the canonical schedule, operation, source span, and backend phase
  whenever that information exists.
- Keep all raw pointer, executable memory, cache lifetime, Send/Sync, and ABI
  safety reasoning inside the native module with tests that cover the boundary.
- Make unsupported targets explicit. For targets without a supported JIT memory
  and ABI implementation, the native-JIT feature is unavailable or returns a
  target-not-supported error rather than silently using the interpreter.

## Non-Goals

- This design does not implement AOT generated Rust. The AOT backend remains a
  separate consumer of canonical IR.
- This design does not redesign the SPICE solver, matrix package, UI, or model
  library metadata.
- This design does not keep the bytecode-to-native Cranelift path as a migration
  bridge.
- This design does not provide a best-effort hybrid native/interpreted execution
  plan. A model is fully native for the requested schedules or it does not load
  through the native path.
- This design does not promise browser WebAssembly JIT execution. Browser and
  mobile product profiles must choose an explicitly supported backend; native
  JIT remains a target-gated backend with hard errors when unavailable.

## Current State

The current `rspice-veriloga` native module compiles bytecode programs through
Cranelift. `native::try_compile_native` returns `Option<NativeModel>` and its
documentation tells callers to fall back to bytecode interpretation. It also
catches compiler panics and returns `None`, again causing interpreted runtime
execution.

`NativeModel` currently stores a hybrid assignment plan. `PlanStep::Chunk`
executes compiled code, while `PlanStep::Interpret` and loop conditions run on
the bytecode VM. Stamp values and Jacobian entries are stored as `Option`
function pointers, so individual failed programs naturally fall back to the
interpreter.

The canonical IR module already exists and states the intended direction:
bytecode and Cranelift are legacy runtime paths, while new architecture work
targets canonical IR. The implemented OptIR is still skeletal, with schedules
and `OptOp::EvaluateEquation` rather than a backend-ready value graph. The JIT
work therefore has two coupled tracks: replace the native backend and expand
OptIR into a complete backend contract.

## Architecture

The native execution flow is:

```text
Verilog-A source
  -> parser and semantic analysis
  -> canonical HIR
  -> canonical MIR
  -> scheduled canonical OptIR
  -> JIT IR lowering
  -> target-independent JIT optimization
  -> target backend selection
  -> x64 machine code emission
  -> relocation, finalization, executable code cache
  -> simulator calls compiled entry points
```

The native backend is split into these crates or modules under
`rspice-veriloga::native`:

- `contract`: typed compile inputs, entry-point descriptions, memory layout
  descriptors, target feature declarations, and typed errors.
- `lower`: canonical OptIR to JIT IR lowering.
- `jir`: compact SSA-like JIT IR with explicit effects, typed values, blocks,
  calls, memory operations, fast-math policy markers, and source-span metadata.
- `opt`: JIT-local optimization passes that are legal after canonical scheduling.
- `isel`: target-independent instruction-selection interfaces.
- `x64`: x64 instruction selection, register allocation integration, encoder,
  relocation generation, ABI lowering, and target feature policy.
- `aarch64`: reserved module boundary for the later AArch64 implementation, not
  enabled until complete.
- `runtime`: executable memory, cache ownership, function pointer publication,
  helper symbol registry, unwind policy, and safety contracts.
- `verify`: structural verifiers for JIT IR, machine IR, stack maps, relocations,
  and entry-point ABI layouts.

Backends implement a narrow trait shaped around complete model compilation:

```rust
trait TargetBackend {
    fn target(&self) -> TargetSpec;
    fn compile_model(&mut self, model: &NativeCompileUnit) -> Result<NativeImage, JitError>;
}
```

The trait does not expose partial compile success. A backend may internally
compile many functions and schedules, but it publishes a `NativeImage` only
after every required entry point verifies and finalizes.

## Canonical IR Contract

The JIT consumes scheduled OptIR, so OptIR must become rich enough to describe
all executable semantics without asking the bytecode layer for help. Required
OptIR additions are:

- Typed value graph operations for arithmetic, comparisons, selects, boolean
  control, casts, loads, stores, table lookup, simulator intrinsics, and helper
  calls.
- Explicit residual, Jacobian, state update, AC, noise, bound-step,
  discontinuity, and operating-point report entry points.
- Canonical memory layout tables for parameters, instance variables, scratch
  values, state slots, branch unknowns, node voltages, internal-node voltages,
  lookup tables, and simulator analysis metadata.
- Effect tokens for stateful analog operators and helper calls whose ordering
  cannot be changed.
- Structured control regions for supported loops and branches with verifier
  rules for dominance, side effects, and bounded temporary storage.
- Derivative value sharing across residual and Jacobian expressions, so compact
  model hot paths avoid duplicated transcendental and limiting work.
- Invalidation schedules that separate instance-static, temperature-static,
  timestep-static, operating-point-static, Newton-iteration, AC-frequency,
  noise-frequency, and reporting work.

The OptIR verifier must reject missing derivatives, unscheduled effectful
operations, ambiguous state updates, unsupported dynamic array indexing,
untyped helper calls, and schedule gaps before target lowering starts.

## JIT IR

JIT IR is not another semantic layer. It is a backend-owned compiler IR designed
for fast lowering into high-quality machine code. It keeps enough structure for
optimization while being close enough to machine code that the x64 backend is
predictable and inspectable.

JIT IR properties:

- Dense IDs for functions, blocks, values, constants, stack slots, virtual
  registers, symbols, and relocations.
- Scalar types required by Verilog-A execution: `f64`, `i64`, `i32`, `u8`,
  pointer-sized integers, booleans, and target pointers.
- Explicit floating-point policy on each function or op group. The default is
  IEEE-preserving behavior suitable for circuit simulation. Any relaxation must
  be reviewed, named, tested, and proven not to change model semantics.
- Operations for arithmetic, fused multiply-add only where allowed, comparisons,
  selects, branches, memory load/store, constant materialization, helper calls,
  and simulator ABI loads.
- Side-effect and alias metadata for simulator state, model parameters,
  scratch buffers, and helper calls.
- Source-span and canonical-operation provenance that can survive into
  diagnostics, disassembly dumps, and failing verifier messages.

JIT IR passes include constant folding, CSE, copy propagation, dead-code
elimination, block simplification, local value numbering, algebraic rewrites
approved by the canonical numerical contract, branch folding, select formation,
loop-invariant code motion only across verified pure operations, and instruction
combining for target patterns.

## x64 Backend

The initial backend targets x86-64 System V and Windows x64 ABIs, with Windows
x64 prioritized because the current development environment is Windows. The
backend must avoid platform assumptions leaking into canonical IR or generic JIT
IR.

The x64 pipeline is:

1. Lower JIT IR to x64 machine IR with virtual registers.
2. Select SSE2/AVX scalar `f64` instructions according to target feature policy.
3. Allocate registers with a production register allocator suitable for hot
   compact-model kernels.
4. Insert ABI prologues, epilogues, call sequences, stack alignment, shadow
   space on Windows, spill slots, constants, and helper-call clobber handling.
5. Encode machine instructions into a relocatable text buffer.
6. Apply relocations, publish executable memory, and expose typed entry points.

Baseline x64 codegen uses SSE2 scalar floating point because it is universal on
x86-64. The backend may add AVX and FMA variants after target-feature dispatch
and correctness gates exist. Vectorization is an optimization layer, not a
semantic dependency.

Runtime performance priorities:

- Keep hot residual/Jacobian entry points straight-line where canonical control
  flow allows it.
- Fuse residual and Jacobian common subexpressions before target lowering.
- Keep frequently used simulator pointers and model bases in registers.
- Avoid helper calls for operations that can be emitted inline without semantic
  loss.
- Use branchless selects for common limiting and guard patterns when they match
  the numerical contract.
- Place constants and cold paths to protect instruction-cache locality.
- Specialize entry points by schedule and target features instead of carrying
  dynamic checks through the hot path.
- Measure generated machine-code throughput directly with model-shaped kernels,
  not only end-to-end simulation wall time.

## Register Allocation

Register allocation is part of the performance core. The first implementation
should use a simple, correct allocator only if it is isolated behind a trait and
replaced before claiming production performance.

The production allocator requirements are:

- Support x64 integer and floating-point register classes.
- Model Windows x64 and System V caller/callee-save rules accurately.
- Handle helper-call clobbers and stack alignment.
- Prefer allocation decisions that reduce hot spills in compact-model
  residual/Jacobian kernels.
- Produce deterministic output for reproducible tests and disassembly diffs.
- Provide verifier checks for live ranges, use-before-def, clobbers, stack
  slots, and ABI-preserved registers.

The implementation can start with linear scan plus splitting, then move to a
backtracking allocator if benchmark data shows meaningful spill pressure. The
trait boundary must allow that change without rewriting lowering or encoding.

## Runtime And Memory Safety

The native runtime owns executable memory and function pointer lifetimes. It
must provide:

- Write-then-execute memory transitions with W^X policy where the platform
  permits it.
- Windows `VirtualAlloc`/`VirtualProtect` and Unix `mmap`/`mprotect`
  implementations behind one interface.
- Instruction-cache flush hooks where required by the target.
- A symbol registry for approved math and simulator helper functions.
- Typed function pointer publication only after relocation and verifier success.
- `NativeModel` ownership that keeps code memory alive longer than every exposed
  function pointer.
- Explicit Send/Sync safety comments and tests for immutable compiled images.
- No panic-to-interpreter downgrade. Recoverable compile failures return
  `JitError`; internal compiler bugs fail tests and are fixed.

The native ABI should pass one pointer to immutable model data and one pointer
to mutable evaluation state where possible, rather than exposing many raw
pointers. The exact layout should be generated from canonical memory descriptors
and checked with offset tests.

## Error Handling

`try_compile_native` is replaced with a result-returning API:

```rust
pub fn compile_native(model: &CanonicalModelArtifact) -> Result<NativeModel, JitError>;
```

The non-native build may expose no-op compile functions only where required for
conditional compilation, but simulator code must not treat native compile
failure as permission to run interpreted code. Native model construction either
stores a complete `Arc<NativeModel>` or returns an error to the caller.

`JitError` categories:

- `UnsupportedTarget`
- `UnsupportedCanonicalOp`
- `InvalidCanonicalIr`
- `Lowering`
- `Verifier`
- `RegisterAllocation`
- `Encoding`
- `Relocation`
- `ExecutableMemory`
- `AbiMismatch`
- `MissingEntryPoint`
- `InternalCompilerError`

Every error includes the model name, backend phase, target triple or target
spec, canonical schedule or entry point when known, source span when available,
and a stable diagnostic code. Errors that represent missing JIT coverage are
tracked as implementation work, not accepted runtime behavior.

## Interpreter Policy

The interpreter has exactly two allowed roles:

- It can be built for explicit non-native product profiles where no native JIT
  was requested.
- It can be used by tests and verification tools as an oracle to compare native
  results against independently executed semantics.

The interpreter is not allowed as:

- An automatic fallback after native compile failure.
- A per-entry-point substitute for missing stamp, Jacobian, loop, assignment,
  noise, AC, or report code.
- A panic recovery path.
- A hidden path in benchmarks that claim native execution.
- A runtime path under `veriloga-native` when native JIT compilation was
  requested for a model.

Tests must include guards that make partial native compilation impossible to
miss. Examples include asserting that native models contain no optional entry
points, no `PlanStep::Interpret` equivalent, no `force_interpreter` production
escape hatch, and no `None` native result that device construction can ignore.

## Target Portability

AArch64 support is designed at the boundary level now and implemented only after
x64 has complete correctness and performance gates. The target abstraction must
separate:

- Calling convention.
- Register classes.
- Instruction selection.
- Encoded instruction format.
- Relocation kinds.
- Constant materialization.
- Stack frame layout.
- Executable-memory cache flush requirements.
- Feature detection and dispatch.

The generic JIT IR, canonical memory descriptors, diagnostics, and model compile
contract stay target independent. Target-specific code owns only ABI lowering,
machine instruction selection, register allocation constraints, encoding, and
relocations.

## Testing And Verification

Required verification gates:

- Filtered searches for `cranelift`, `Cranelift`, and `cranelift_jit` are clean
  across active source, Cargo manifests, lockfiles, legal notices, and product
  documentation after the replacement lands. Historical design records may still
  describe why the old backend was removed.
- Cargo manifests and `Cargo.lock` contain no Cranelift packages.
- Native compile APIs return `Result`, and device construction fails on native
  errors instead of caching `None`.
- Tests prove there is no automatic interpreter fallback in native mode.
- Canonical OptIR verifier tests cover every operation and schedule consumed by
  the native JIT.
- JIT IR verifier tests cover dominance, types, effects, side-effect ordering,
  function signatures, stack slots, and provenance metadata.
- x64 encoder tests compare exact bytes for representative instructions,
  addressing modes, constants, calls, branches, prologues, epilogues, and
  relocations.
- Executable-memory tests allocate, write, protect, call, and release small
  functions on each supported desktop platform.
- ABI tests call generated functions with sentinel register and stack values to
  detect clobber, alignment, shadow-space, and return-value mistakes.
- Native/interpreter equivalence tests remain, but the interpreter is launched
  explicitly by the test harness as an oracle.
- Disassembly snapshot tests cover generated kernels for representative compact
  model patterns.
- Benchmark tests report runtime throughput for generated x64 code against the
  interpreter and against generated Rust where applicable.
- Wasm/browser builds prove native-JIT code and executable-memory dependencies
  do not leak into unsupported targets.

## Rollout

1. Add this design spec on an isolated branch for review.
2. Write an implementation plan that decomposes the work into small, verifiable
   steps.
3. Replace native API semantics with result-returning hard-fail behavior and add
   tests that prevent interpreter fallback in native mode.
4. Remove the Cranelift dependency surface from manifests, lockfile, modules,
   docs, and tests.
5. Expand canonical OptIR until it can describe the required executable device
   schedules without bytecode assistance.
6. Add JIT IR, verifiers, and lowering tests from canonical OptIR.
7. Add x64 executable-memory and encoder foundation tests.
8. Add the initial x64 backend with enough operation coverage to compile the
   current native test fixtures fully.
9. Replace device integration with complete native entry-point tables and hard
   failure on missing entries.
10. Add performance scoreboards and optimize register allocation, instruction
    selection, helper inlining, and schedule fusion based on measured kernels.
11. Add target abstraction tests that keep AArch64 support practical without
    enabling an incomplete AArch64 backend.

## Commercial Quality Criteria

- The native JIT is sourced from canonical IR, not bytecode.
- Cranelift is absent from active source, dependencies, lockfiles, and product
  documentation.
- A requested native JIT model either compiles every required entry point or
  fails with a typed diagnostic.
- No runtime interpreter fallback exists in native mode.
- Generated x64 machine code is verified, disassembled, benchmarked, and
  reproducible.
- Backend interfaces isolate target-specific code so AArch64 can be added
  cleanly.
- Safety-sensitive code has narrow ownership boundaries, explicit invariants,
  and tests.
- Numerical behavior is governed by canonical IR and verifier rules, not by
  target backend convenience.
- Performance claims are backed by checked-in benchmarks and generated-code
  inspection.
