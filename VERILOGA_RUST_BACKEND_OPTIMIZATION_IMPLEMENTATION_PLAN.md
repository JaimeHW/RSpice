# Verilog-A Rust Backend Optimization Implementation Plan

Plan date: 2026-08-08

Scope: `rspice-veriloga`, `rspice-veriloga-runtime`, the generated
`rspice-veriloga-models` catalog, and the generated-model qualification tools.

## Objective

Reduce generated Rust source size and release compile time as far as practical
without weakening numerical correctness, steady-state stamp throughput,
diagnostics, model identity, portability, or the direct-Rust execution model.

The work is complete only when every accepted optimization is regenerated over
all 42 shipped models and is supported by correctness, performance, resource,
formatting, warning, and cross-target evidence. A promising prototype is not a
completed phase. Any candidate that fails its acceptance gate is removed rather
than hidden behind an undocumented fallback.

## Current measured baseline

The first optimization pass is the starting point, not work to repeat.

| Measurement | Current result |
|---|---:|
| Generated model Rust files | 210 |
| Generated model Rust bytes | 26,797,561 |
| `stamp.rs` bytes | 16,387,127 |
| `noise.rs` bytes | 6,991,041 |
| `state.rs` bytes | 3,389,693 |
| Other model Rust bytes | 29,700 |
| Largest generated model | 1,976,243 bytes |
| HiSIM-HV cold release package compile | 291.3 seconds |
| BSIM-BULK profiled rustc compile | 57.5 seconds |
| BSIM-BULK LLVM/codegen share | 55.5 seconds |
| BSIM-BULK stamp-function LLVM optimization | 46.1 seconds |
| BSIM-BULK LLVM loop-pass time | 42.6 seconds |

The compiler frontend is no longer the compile-time bottleneck. In the profiled
BSIM-BULK build, parsing, expansion, name resolution, type checking, borrow
checking, metadata generation, and LLVM-IR generation together took roughly
1.8 seconds. The plan therefore separates source-only cleanup from changes that
materially reduce LLVM work.

The existing implementation already provides:

- native Rust Boolean CFG values;
- conservative constant-literal inlining;
- a 32-node expanded-expression cap;
- packed derivative values through `Lanes<N>`;
- empty-diamond-arm elimination;
- emission-time state-slot resolution;
- shortest exact real literals;
- generated-module `rustfmt` boundaries;
- source-size and structural regression checks; and
- independent packages for the generated leaf models.

These are baseline invariants. None should be reopened without new measured
evidence.

## Non-negotiable invariants

Every phase must preserve all of the following.

1. Generated stamps remain direct Rust. No interpreter, bytecode dispatcher,
   JIT dependency, dynamic expression graph, or per-operation virtual dispatch
   may enter a generated package.
2. All values, residuals, Jacobians, charges, noise magnitudes, state
   transitions, parameter defaults, and parameter errors retain their current
   semantics. Numerical qualification is performed before golden fixtures may
   be updated.
3. The hot Newton path may not allocate, lock, hash, dynamically dispatch, or
   add data-dependent loops that are absent from the current path.
4. A runtime-performance regression is not traded for a smaller source tree or
   a faster build.
5. Desktop, `wasm32`, and supported mobile targets use the same semantic
   backend. Architecture-specific SIMD may be an optional later enhancement,
   but cannot be the correctness implementation.
6. Generated files remain reproducible, authenticated by `manifest.txt`, and
   outside ordinary workspace `rustfmt` traversal.
7. Model names, terminal counts, parameter names and aliases, checkpoint model
   identities, persistent-state validation, noise descriptors, and feature
   names remain stable unless an intentional migration is separately approved.
8. Unsupported conditions fail explicitly. No optimization may introduce a
   silent slow path or silently emit a different representation.

## Delivery and experiment policy

Implement each work package separately. Do not stack two unqualified
optimizations in one measurement, because a win in one can conceal a regression
in the other.

For every candidate:

1. capture a same-host baseline from the current parent revision;
2. implement the candidate in isolation;
3. regenerate only a subset while iterating;
4. run exact and derivative checks on the subset;
5. run paired compile and runtime measurements;
6. accept or remove the candidate;
7. regenerate the complete catalog only after acceptance; and
8. commit the implementation, generated output, tests, and evidence together.

Use alternating baseline/candidate runtime runs rather than running all
baseline samples first. Record toolchain, target triple, CPU, power mode,
profile, enabled features, model digest, and generated bundle digest with every
result.

The acceptance policy for runtime measurements is:

- the candidate median must not be slower than the paired baseline median;
- a result within one percent but on the slower side is inconclusive and must
  be rerun with more samples;
- a result more than one percent slower is rejected; and
- p95, checksum, linked stamp count, and same-run handwritten-reference ratio
  must remain valid even when the median passes.

Source and compile-time targets in this plan are minimum useful wins, not
permission to accept a runtime regression.

## Dependency order and primary code ownership

The phases are intentionally ordered by measurement dependency, not merely by
estimated source savings. Phase 0 is required infrastructure. Phase 1 changes
the dominant LLVM workload and therefore establishes the compile-time baseline
for every later phase. Phases 2, 3, and 4 can then be delivered independently.
Phase 5 depends on the stabilized lane and metadata representations, and Phase
6 should be judged after the cache and family boundaries are final. Phase 7 is
the catalog-wide integration gate.

| Phase | Primary implementation files | Primary qualification files |
|---|---|---|
| 0 | `crates/rspice-bench/src/main.rs`, new `generated_compile.rs`, `generated_rust.rs`, benchmark report/provenance modules | `rspice-bench` unit tests and `benchmarks/README.md` |
| 1 | `crates/rspice-veriloga-runtime/src/lib.rs`, `rust_backend/emit.rs`, `rust_backend/canonical.rs` | `cfg_runtime.rs`, `generated_output_audit.rs`, `cfg_derivatives.rs` |
| 2 | runtime cache helper, `rust_backend/canonical.rs`, `rust_backend/stamp_plan.rs` | canonical-device, invalidation, generated-output, and engine cache tests |
| 3 | `rust_backend/canonical.rs`, `rust_backend/emit.rs`, `rust_backend/expr.rs` | structured-CFG fixtures, `canonical_device.rs`, derivative audit |
| 4 | `rust_backend/state_file.rs`, runtime parameter helpers, generated manifest/resource reporting | parameter corpus, state/checkpoint, generated-output, and rustfmt-boundary tests |
| 5 | `rust_backend/builtins.rs`, `manifest.rs`, `files.rs`, `registry.rs`, generated catalog workspace/package emitters | family equivalence, checkpoint identity, feature isolation, compile and stamp benchmarks |
| 6 | `rust_backend/noise.rs`, `canonical.rs`, `emit.rs`, runtime stage helpers | noise golden and call-order-independence integration tests |
| 7 | generator budgets, CI workflows, benchmark baselines, release evidence | complete catalog and cross-target matrix |

Each source/compile target below is measured against that phase's accepted
parent. The targets are not blindly additive because family sharing and noise
factoring can remove some of the same physical source. Final reporting must
show both per-phase deltas and the net delta from the baseline in this document.

## Phase 0: make compile-time evidence repeatable

### Purpose

Turn the ad hoc package-only measurements and LLVM traces into a maintained,
reproducible qualification workflow before changing LLVM shape.

### Implementation

1. Add a `generated-compile` command to `rspice-bench` rather than a separate
   script.
2. Measure package-only release compilation after dependencies are prepared.
   Force a leaf rebuild with a unique metadata value while keeping the same
   dependency artifacts.
3. Support at least these model selections:
   `rspice-veriloga-model-bsimbulk`,
   `rspice-veriloga-model-bsimcmg-va`, and
   `rspice-veriloga-model-hisimhv-va`.
4. Support default and `veriloga-builtins-noise` builds independently.
5. Run at least three samples and report median, minimum, maximum, and raw
   samples. Do not mix cold dependency build time with leaf compile time.
6. Emit a provenance-rich JSON report beside the existing `generated-rust`
   and `generated-stamp` reports.
7. Keep stable-toolchain wall time as the release gate. Permit
   `RUSTC_BOOTSTRAP=1 -Ztime-passes -Zllvm-time-trace` only in an explicitly
   diagnostic mode; nightly-only output is evidence, not a shipping
   dependency.
8. Add structural counters to `generated-rust` for packed-lane operations,
   emitted source loops, cache-scatter mappings, capture declarations, and
   generic-lane fallback use.

### Acceptance

- Repeated unchanged runs identify the same packages and bundle digest.
- Package-only timings exclude dependency compilation.
- JSON reports are deterministic except for timing and host fields.
- Invalid package names, failed builds, and host/toolchain mismatches fail
  visibly.
- Existing `generated-rust` and `generated-stamp` commands remain compatible.

## Phase 1: replace inline generic lane loops with fixed-width unrolled lanes

### Why this is first

`Lanes<N>` currently implements `Add`, `Sub`, scalar `Mul`, and scalar `Div`
with an `#[inline(always)]` `while` loop. A wide generated stamp contains
thousands of these operations. LLVM inlines the loops into the stamp, performs
loop analysis and unrolling thousands of times, and spends most of the leaf
compile there.

BSIM-BULK has only four source-level stamp loops, but its LLVM trace recorded
approximately 5,678 loop-rotation invocations. Removing those synthetic loops
is the highest-confidence route to a large compile-time improvement while
retaining direct scalar arithmetic.

### Runtime representation

1. Add fixed-width public, hidden-API lane newtypes in
   `rspice-veriloga-runtime`: `L2`, `L3`, and so on through `L32`.
2. Generate their definitions with one runtime macro, but make every generated
   operator body explicitly construct the output array from indexed scalar
   expressions. The expanded implementation must contain no loop.
3. Implement `Add`, `Sub`, scalar `Mul`, scalar `Div`, and `Index<usize>` for
   every fixed width.
4. Retain the generic `Lanes<N>` type as a correctness fallback for widths
   above 32. The shipped corpus must use the fallback zero times.
5. Preserve `#[repr(transparent)]`, `Copy`, and `Clone` where valid, and retain
   the same array layout.
6. Do not introduce fused arithmetic, reassociation, reduction operations, or
   cross-lane dependencies. Each output lane performs the same IEEE operation
   in the same order as today.

### Emitter changes

1. Add one width-to-type function in `rust_backend/emit.rs`.
2. Emit `f64` for one lane, `L2` through `L32` for supported packed widths, and
   `Lanes<N>` only above the fixed-width ceiling.
3. Apply the selected constructor consistently to splats, widening,
   conditional zero initialization, captures, and ordinary packed values.
4. Import exactly the fixed-width types used by each generated file, or import
   the bounded set under the existing generated unused-import policy.
5. Keep the standalone `RUNTIME_PRELUDE` behavior in lockstep with the shared
   runtime implementation. Prefer one source template or generated definition
   over two handwritten copies.
6. Extend generator metrics with a width histogram and generic-fallback count.

### Tests

1. Unit-test every fixed type's four arithmetic operations against the current
   generic implementation.
2. Include `0.0`, `-0.0`, subnormals, infinities, and several NaN payloads.
   Compare result bits wherever IEEE arithmetic defines a stable result.
3. Exercise widths 2, 3, 4, 8, 16, 22, and 32 directly.
4. Add emitter tests for splat, widen, extract, captures, and mixed scalar/packed
   expressions.
5. Add a generated-output audit requiring zero generic-lane fallback uses in
   the shipped corpus.
6. Run the complete golden and complex-step derivative gates.

### Performance gate

Measure BSIM-BULK and HiSIM-HV before and after.

- Required: at least 15% lower median BSIM-BULK package compile time.
- Required: at least 10% lower median HiSIM-HV package compile time.
- Target: remove most of the LLVM loop-pass time shown by the current trace.
- Required: no generated-stamp median regression under the experiment policy.
- Required: no new loop remains in fixed-width lane operator IR.

If fixed-width types do not produce the expected compile win, inspect the LLVM
trace before trying another representation. Do not compensate by disabling
optimizer passes.

## Phase 2: table-drive canonical-stage cache installation

### Current cost

The generated stamps contain 16,053 statements assigning `produced[index]` or
shared model values into `canonical_staged[slot]`. They occupy 818,627 source
bytes and execute only when a model, instance, or temperature stage is
installed.

### Design

1. Add a shared runtime helper that copies a value slice into destination slots
   described by an authenticated static mapping.
2. Use `u32` destination slots in generated tables and checked conversion to
   `usize`; do not assume every future model fits in `u16`.
3. Keep the helper non-generic and out of line so every model does not inline
   and re-optimize the installation loop.
4. Emit one mapping table per invalidation stage.
5. Preserve export order and Boolean-to-`f64` conversion at the stage boundary.
6. Optionally encode contiguous destination runs separately and use
   `copy_from_slice` where measurement shows that this improves cold-stage
   installation without increasing source.
7. Continue storing model-stage values in the existing shared `Arc`; the table
   changes installation, not cache identity or ownership.

### Tests and acceptance

- Unit-test empty, singleton, sparse, contiguous, and invalid mappings.
- Prove every exported value is installed once and every mapping slot is in
  range during generation.
- Test parameter, instance, temperature, and timestep invalidation separately.
- Test model-cache hits and misses across multiple instances.
- Require at least 500 KB total generated-source reduction.
- Require no steady-state generated-stamp regression.
- Measure first-stamp and invalidation-stage cost separately; reject a material
  cold-path regression rather than assuming it is irrelevant.

## Phase 3: emit eligible multi-output diamonds as tuple expressions

### Current cost

The generated stamp and noise files contain approximately 8,943 mutable capture
declarations and 12,899 capture assignments, totaling about 472,826 bytes.

### Eligibility analysis

Add a dedicated structured-CFG analysis rather than changing all captures.
A diamond is eligible only when:

1. it is acyclic and contains no back edge;
2. both arms have one lexical owner and rejoin at the same block;
3. every returned value has an explicit value for both outcomes, including the
   current zero/default behavior where applicable;
4. no returned value is the "last iteration" result of a loop;
5. tuple arity is below a measured cap, initially eight;
6. emitted tuple form is smaller under the existing source-cost model; and
7. total expanded expression complexity remains under a bounded cap.

Stateful calls inside an eligible arm may be retained only if tuple emission
preserves their exact control dependence and single evaluation. Otherwise the
diamond remains in capture form.

### Emission

Emit:

```rust
let (a, b) = if condition {
    // ordinary arm statements
    (true_a, true_b)
} else {
    // ordinary arm statements
    (false_a, false_b)
};
```

Do not build nested tuples merely to bypass the arity cap.

### Acceptance

- Add focused nested-diamond, partial-definition, stateful-call, and loop
  fixtures.
- Require exact golden and derivative agreement.
- Require a measurable source reduction; target 250 KB or more.
- Reprofile BSIM-BULK SROA, control-flow, and total LLVM time.
- Reject the tuple form if any representative runtime median regresses or LLVM
  time increases materially.

This phase is deliberately after fixed-width lanes so tuple behavior is judged
against the new LLVM bottleneck rather than the one being removed.

## Phase 4: compact cold state and parameter metadata

Treat this as several independently reviewable changes.

### 4A. Generated indentation

Use tab indentation throughout generated `state.rs`, just as the CFG emitter
does. The stable parent-module `rustfmt` boundary remains mandatory. Add a
regression test that ordinary workspace formatting reports no generated paths.

Expected saving: roughly 250-300 KB.

### 4B. Parameter-bound interning

1. Build a per-model pool of unique `(value, exact_label)` bounds.
2. Store compact bound-pool indices in the per-parameter min/max tables rather
   than repeating `ParameterBound { value, label }`.
3. Use an explicit sentinel or `Option<NonZeroU32>` representation whose layout
   is tested; do not depend on an undocumented enum size.
4. Pool exclusions through the same representation where profitable.
5. Preserve exact diagnostic labels and exclusive/inclusive flags.

### 4C. Default-alias operations

1. Identify simple dependency-ordered defaults of the form
   `params[dst] = params[src]`.
2. Encode them as static operations and execute them through a shared runtime
   helper in original dependency order.
3. Keep general default expressions in emitted Rust.
4. Preserve finite, integer, range, and exclusion validation after every
   dynamic default.

### 4D. Name and alias lookup

1. Keep lookup deterministic and at least logarithmic for large models.
2. Use compact integer indices and one shared lookup implementation.
3. Do not move metadata to an opaque binary blob in the first implementation.
   Consider `include_bytes!` only if typed-table work fails to deliver a useful
   source reduction and a versioned binary format is justified separately.

### Acceptance

- Enumerate every parameter name and alias for all 42 models and compare lookup
  results before and after.
- Compare defaults, `param_given`, model/instance scope, min/max, exclusivity,
  exclusions, and error text.
- Test chained and forward-invalid default aliases.
- Require 700 KB total state-source reduction across 4A-4D.
- Keep all generated packages warning-free.
- Confirm no steady-state stamp change; separately measure construction and
  parameter-validation cost.

## Phase 5: share compiled kernels across near-identical model variants

### Candidate families

Start with families whose upstream sources differ only in small topology or
preprocessor sections:

- HiSIM-HV 6-port, N4, and N5 variants;
- HiSIM-SOI base, N4, and N5 variants; and
- only then investigate other related variants.

L-UTSOI and L-UTSOI-NQS differ by one preprocessor definition but intentionally
select different physics. They are not automatically one kernel.

### Discovery before architecture

1. Add a canonical structural fingerprint for CFG regions after normalization
   of value IDs, block IDs, source expression IDs, module names, and terminal
   numbering.
2. Fingerprint value type, operator, operand topology, control-flow structure,
   derivative lane shape, invalidation class, and stateful-operator identity.
3. Report exact common-region coverage between family members.
4. Do not share regions based on textual similarity or source-file names.
5. Proceed only when at least 85% of the expensive Newton kernel is exactly
   common after explicit topology normalization.

### Package architecture

Generate a family-kernel package under a stable content-derived identity. Thin
model packages depend on it, so Cargo compiles the expensive common code once.

The common Newton kernel must be non-generic. A generic helper would be
monomorphized again in every leaf and would not solve the problem.

Preferred design:

1. one family-owned instance/core state representation large enough for all
   supported variants;
2. a small variant enum fixed at construction;
3. variant-specific prologues that normalize external/internal terminal
   potentials and branch indices;
4. one non-generic common compute kernel;
5. variant-specific topology metadata or epilogues only where stamps differ;
6. one outer variant decision per entry point, not branches distributed through
   every arithmetic region; and
7. family-owned shared model preprocessing and noise metadata where structural
   fingerprints prove identity.

If the shared function requires a second hot-path call, benchmark it. Prefer
catalog routing directly to the family implementation over a wrapper that adds
another layer of dynamic dispatch.

### Identity and state requirements

- Each public model retains its own model name, feature, terminal count,
  descriptor table, and checkpoint identity.
- Model-cache keys include the variant or exact kernel fingerprint whenever
  variant-dependent preprocessing exists.
- A checkpoint captured from one variant must be rejected by another variant.
- Parameters and aliases remain variant-correct even when metadata is shared.
- Enabling one variant pulls the family kernel but not unrelated public model
  wrappers. Enabling several variants still compiles the common kernel once.

### Manifest and resource reporting

Extend the generated manifest to authenticate shared kernel files and attribute
their bytes separately from thin public-model wrappers. `generated-rust` must
report both physical bundle bytes and attributed logical model bytes without
double-counting the shared kernel.

### Acceptance

- Begin with HiSIM-HV only.
- Require at least 3 MB physical generated-source reduction for the accepted
  family-sharing phase.
- Measure one-variant and all-variant release builds.
- Require a substantial all-variant compile-time reduction.
- Require exact golden, derivative, noise, state, parameter, and checkpoint
  agreement for every variant.
- Require no stamp-throughput regression under the experiment policy.
- If the unified hot kernel regresses runtime, retain only proven cold/state
  sharing or reject the phase; do not hide the regression behind the aggregate
  catalog median.

## Phase 6: reduce the independent noise path without assuming stamp call order

### Correctness boundary

`evaluate_noise_sources` receives its own operating-point context. It is valid
on a fresh instance and cannot assume that `stamp` ran at the same solution.
Reading `canonical_staged` implicitly would make results depend on call order
and is forbidden.

### Safe design

1. Refactor model, instance, and temperature preprocessing into generated pure
   helper functions with explicit inputs and outputs.
2. The stamp path invokes those helpers only when its existing caches are
   invalid and installs their results.
3. The noise path invokes the same helpers into a local prepared-noise stage
   buffer, independent of stamp history.
4. Keep helpers non-generic and out of line. They are invalidation/noise setup
   work, not steady-state Newton work.
5. Apply ordinary liveness to the prepared noise stages so noise computes only
   stage outputs its magnitudes need.
6. Continue emitting the Newton-dependent primal noise slice from current node
   potentials; it cannot be reused from a previous stamp.
7. Combine this with family-kernel sharing where exact family fingerprints
   prove the noise preprocessing or magnitude body is common.

An alternative mutable `evaluate_noise_sources(&mut self)` API may be studied,
but it is not the preferred first design: mutation alone does not prove the
cached context matches the supplied operating point.

### Tests

For every representative noise model, compare:

1. noise on a fresh instance;
2. noise after a stamp at the same bias;
3. noise after a stamp at a different bias;
4. noise after a parameter change;
5. noise after a temperature change;
6. repeated noise calls; and
7. noise before and after checkpoint restore.

All cases with the same explicit noise context must produce identical source
descriptors and bit-identical magnitudes.

### Acceptance

- Require a meaningful physical noise-source reduction; target at least 15%.
- Require exact noise golden agreement and call-order-independence tests.
- Measure noise preparation/evaluation separately from stamp throughput.
- Require no steady-state stamp regression.
- Do not lower the noise crate's optimization level as a substitute for a
  better representation.

## Phase 7: full integration and release qualification

After all individually accepted phases:

1. regenerate all 42 packages with the production generator profile;
2. run `check-builtins` and authenticate the manifest;
3. update source budgets only downward from measured accepted results, leaving
   documented model-growth headroom;
4. compile the complete catalog with noise and require zero warnings;
5. run all compiler, canonical backend, generated-output, state, parameter,
   checkpoint, and engine integration tests;
6. run full-corpus golden verification;
7. run the ignored whole-corpus complex-step derivative audit;
8. run full-corpus generated-stamp benchmarks with the same-run handwritten
   reference;
9. run package-only compile benchmarks for BSIM-BULK, BSIM-CMG, and HiSIM-HV;
10. build/check representative generated packages for desktop, `wasm32`, and
    supported mobile target triples;
11. verify ordinary `cargo fmt --all -- --check` reports no generated paths and
    no rustfmt stack overflow;
12. inspect the final diff for generated-only churn, stale files, feature drift,
    or unrelated workspace changes; and
13. record final physical bytes, compile timings, runtime timings, toolchain,
    host, and bundle digest in the release evidence.

Core commands include:

```text
cargo test -p rspice-veriloga --lib
cargo test -p rspice-veriloga --test canonical_device
cargo test -p rspice-veriloga --test generated_output_audit
cargo test -p rspice-veriloga --test cfg_derivatives \
  the_whole_corpus_matches_complex_step_at_drawn_bias_points -- --ignored --nocapture

cargo run -p rspice-veriloga --profile generator \
  --bin rspice-veriloga-gen -- regenerate-builtins --jobs 2
cargo run -p rspice-veriloga --bin rspice-veriloga-gen -- check-builtins

cargo check -p rspice-veriloga-models --all-features -j 2

cargo run --locked --release -p rspice-conformance \
  --features veriloga-builtins-models \
  --bin rspice-veriloga-golden -- verify

cargo run --locked --release -p rspice-bench \
  --features generated-stamp -- generated-stamp \
  --out benchmarks/results/generated-stamp.json

cargo run --locked -p rspice-bench -- generated-rust \
  --out benchmarks/results/generated-rust.json
```

The final qualification must use the maintained `generated-compile` command
added in Phase 0 rather than copied shell timing snippets.

## Explicit non-solutions

Do not pursue these without new evidence that changes the tradeoff:

- macros whose only effect is shorter checked-in text; rustc expands them back
  into the same or a larger workload;
- global constant interning or aggressive one-use predicate inlining, which
  already increased compile time;
- lowering release optimization levels for hot generated stamps;
- disabling LLVM loop, vectorization, or scalar optimization passes;
- arbitrary `#[inline(never)]` splitting of the Newton body, which adds call and
  materialization overhead and has already regressed runtime;
- generic shared family kernels, which are monomorphized per leaf;
- implicit noise-cache reuse based on assumed call order;
- unchecked indexing merely to remove bounds checks that LLVM already proves
  away; or
- an opaque binary metadata format before typed static-table compaction has
  been exhausted.

## Completion matrix

| Requirement | Authoritative evidence |
|---|---|
| Direct Rust remains the execution model | Generated-output audit; dependency audit; source scan |
| Numerical behavior is unchanged | Full golden verification plus independent complex-step audit |
| Stamp throughput does not regress | Paired `generated-stamp` reports and handwritten-reference ratios |
| Compile time improves | Provenance-rich `generated-compile` reports for three wide models |
| Source shrinks | Authenticated `generated-rust` report and manifest digest |
| Noise is call-order independent | Fresh/stale/repeated noise-context integration tests |
| State and identity remain correct | Checkpoint, rollback, persistent-state, and cross-variant refusal tests |
| Parameters remain correct | Full name/alias/default/range corpus audit |
| Desktop/browser/mobile remain supported | Target-specific generated-package builds and product CI |
| Formatting cannot rewrite generated bodies | Static boundary test plus workspace rustfmt diagnostic |
| Catalog remains production-clean | All-feature build with zero warnings and current manifest |

No phase is complete when any row relevant to that phase has missing,
inconclusive, or contradictory evidence.
