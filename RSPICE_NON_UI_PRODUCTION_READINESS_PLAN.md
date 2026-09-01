# RSpice non-UI production-readiness implementation plan

Plan date: 2026-09-01

Scope: the RSpice simulation engine, netlist and import frontends, deck
orchestration, numerical analyses, result contracts, CLI, Python API, WebAssembly
API, persistence, and non-UI export paths.

Explicitly out of scope for this plan:

- all EGUI and other UI design or implementation work;
- licensing, packaging, distribution, and platform-qualification programs;
- Verilog-A, Verilog-AMS, event-driven digital simulation, and digital UI/output
  work currently owned by the other implementation effort; and
- cloud-service policy and account behavior unrelated to simulator correctness.

The plan retains coordination points for excluded work so that shared result,
checkpoint, and analysis interfaces do not make that work harder to integrate.

## Objective

Eliminate every non-UI, non-digital defect and incomplete capability identified
by the two production-readiness audits, and replace the duplicated execution
paths that caused several of them with one target-neutral execution contract.

The immediate goal is not merely to make the current tests pass. The completed
product must:

1. never publish a successful but scientifically incorrect analysis;
2. never silently omit an authored output or substitute zero for missing data;
3. never panic because of valid authored input, parameter order, topology, or
   output selection;
4. execute the same deck semantics in the CLI, Python, and WebAssembly surfaces;
5. preserve typed result data through compression, serialization, and export;
6. stop promptly and consistently on cancellation or timeout;
7. reject unsupported work before starting, with a precise diagnostic and no
   partial output artifact; and
8. qualify numerical work against independent analytical or simulator oracles,
   not against newly generated RSpice output alone.

## Audit finding register

The IDs below are the source of truth for scope and traceability. “Observed”
means reproduced during the audits. “Inspected” means established directly from
an implementation boundary or explicit rejection in the current source. Phase
0 must turn every still-open item into an executable regression before its fix
is started.

| ID | Priority | Finding | Audit status |
|---|---:|---|---|
| AR-01 | P0 | `rspice-wasm` does not compile because `OutputDirectiveKind::Fft` is not matched | Reproduced again on 2026-09-01 |
| AR-02 | P0 | CLI `.DISTO` validates `f2/f1` backwards, invokes ordinary AC, and exports zero/linear data instead of Volterra products | Observed |
| AR-03 | P0 | Conditional `.STEP` topology can omit signals or panic depending on coordinate order because only analysis kind, not result schema, is preflighted | Observed |
| AR-04 | P0 | CLI silently discards `.SAVE @device[param]`, even though the parsed `SaveSet` supports it | Observed |
| AR-05 | P1 | `.SP ... donoise` rejects the documented keyword form and the numeric form does not export its computed port-noise result | Observed |
| AR-06 | P1 | `.FOUR` silently drops current outputs and launches an independently chosen transient instead of consuming the authored transient result | Observed |
| AR-07 | P1 | `.TEMP` is treated as a separate DC operating-point sweep instead of a run axis wrapping authored analyses; the Python deck runner has the same semantic error | Observed |
| AR-08 | P1 | Typed transient `.FFT` results are computed in core but are absent from CLI/WASM, inaccessible in Python, and discarded by Python pickle/compressed-pickle paths | Inspected |
| AR-09 | P1 | CLI timeout and Ctrl-C do not reach several Monte Carlo, PSS, HB, Fourier, temperature, STB, noise, pole-zero, sensitivity, and checkpoint paths | Inspected |
| AR-10 | P1 | The browser/WASM API only executes OP, AC, and TRAN, exposes an incomplete result set, and lacks a production cancellation contract | Inspected |
| AR-11 | P1 | Compressed transient results lose complete signal families and metadata, fabricate zero step sizes on expansion, and do not reliably compose with checkpoint/restart and authored output scheduling | Observed/inspected; partially improved in the current tree |
| AR-12 | P1 | A checkpoint file can be successfully saved even when its captured blockers guarantee that resume will reject it | Inspected |
| AR-13 | P2 | Exact HB/PAC/PNoise/PSS/envelope/PZ coverage is incomplete for native analog devices, dynamic state, colored noise, AC-NQS, and lossy/distributed elements | Inspected |
| AR-14 | P2 | Native compact/passive model coverage has explicit gaps: HICUM/MEXTRAM routing, LTRA `G != 0`, and turns/geometry-based inductor synthesis | Inspected; HICUM/Verilog-A ownership requires coordination |
| AR-15 | P2 | SPEF reduced parasitics and inductance are rejected; Spectre statistics and native analog statement/model lowering are incomplete | Inspected |
| AR-16 | P2 | Transient integration-order support is restricted to orders 1 and 2 | Resolved as an intentional Xyce-compatible boundary; see `TRANSIENT_INTEGRATION_ORDER_DECISION.md` |
| AR-17 | P1 | CLI and Python outputs are written directly to final paths and can leave truncated or partial files | Inspected |
| AR-18 | P1 | Internal/public result access can panic and aggregation can zero-fill missing currents, converting schema errors into plausible data | Observed/inspected |
| AR-19 | P1 | CLI `.STEP` supports only a subset of analyses, rejects repeated analysis kinds, cannot compose with `.ALTER`/textual `.DATA`, and lacks per-coordinate checkpoint/output namespaces | Inspected |
| AR-20 | P2 | Analog netlist compatibility gaps include parenthesized subcircuit actual nodes, differential `.IC`/`.NODESET`, and per-record continuous `FAILVALUE`; legacy Xyce Y-device work must be split into analog and excluded digital families | Inspected |
| AR-21 | P1 | The first audit found the `event-listener` security advisory in the dependency graph | Previously observed; the package is absent from the current locked graph and needs a permanent gate, not another upgrade |
| AR-22 | P2 | Panic-prone convenience use, warning debt, and incomplete lint/API hardening remain incompatible with a strict production error contract | Inspected |

## Non-negotiable engineering rules

These rules apply to every work package.

1. **No silent fallback.** An unsupported device, signal, analysis, import form,
   or result projection returns a typed error. It must not become AC, OP, zero,
   an empty table, or a nominal-temperature run.
2. **No authored-input panic.** All input-derived indices, names, schemas, and
   shapes use checked access. `expect` is permitted only for a locally proven
   invariant with a test that constructs the boundary case.
3. **No unqualified golden update.** A mismatch is investigated against an
   analytical result or independent simulator. Existing oracle data is not
   replaced merely to make a change pass.
4. **Typed missingness.** A signal that does not exist at a coordinate is
   represented as absent with provenance, never as `0.0`. Formats without a
   missing-value representation use separate coordinate artifacts or fail
   before writing.
5. **One semantic implementation.** CLI, Python, WASM, and the engine adapter
   may translate inputs and serialize outputs, but may not independently decide
   what `.TEMP`, `.STEP`, `.FOUR`, `.FFT`, `.DISTO`, or `.SAVE` means.
6. **Abort is an execution dependency.** Long-running core entrypoints receive
   an abort source; a frontend may not call a non-abort wrapper.
7. **Results are immutable evidence.** Compression and serialization may use a
   declared numerical approximation for waveforms, but may not drop channels,
   result kinds, units, names, validity masks, provenance, or solver metadata.
8. **Output is transactional.** A failed or cancelled run leaves either the old
   complete artifact or no artifact, never a partially written replacement.
9. **Limits are preflighted.** Result shape, run cardinality, allocation size,
   output namespaces, and checkpoint capability are checked before solver work.
10. **Feature capability is explicit.** Native analog capability interfaces
    must leave extension points for the separate Verilog/AMS/digital effort,
    without that effort becoming a dependency of this plan.

## Target architecture

The primary structural change is a shared, target-neutral execution pipeline in
`rspice-core`. It owns deck semantics but not CLI formatting, Python objects, or
JavaScript serialization.

```text
parsed Netlist
     |
     v
DeckPlan -- axes (.STEP/.TEMP/.ALTER/.DATA), analysis ordinals, dependencies
     |
     v
MaterializedRun -- immutable coordinate, topology identity, output namespace
     |
     v
AnalysisExecutor -- abort source, resource budget, checkpoint policy
     |
     v
TypedAnalysisResult -- schema + values + units + provenance + validity
     |
     +--> Post-processing: .MEASURE / .FOUR / .FFT
     |
     v
SignalProjection -- .SAVE/.PRINT/.PROBE selection, never format-specific
     |
     +--> CLI exporter
     +--> Python adapter
     +--> WASM adapter
     +--> engine/cloud adapter
```

Create `crates/rspice-core/src/execution/` with these initial concepts:

- `DeckPlan`: the ordered analysis instances and Cartesian run axes;
- `AnalysisInstanceId`: analysis kind plus authored ordinal, so repeated cards
  cannot collide;
- `RunCoordinate`: typed axis values, stable coordinate ID, and display label;
- `MaterializedRun`: one elaborated netlist, topology fingerprint, output
  namespace, and checkpoint namespace;
- `AnalysisRequest`: typed analysis configuration plus post-process requests;
- `AnalysisResult`: an enum covering every core result type without reducing
  it to voltage tables;
- `SignalDescriptor` and `SignalSchema`: stable name, kind, unit, owner,
  coordinate availability, and value shape;
- `ProjectedResult`: selected typed signals plus explicit validity; and
- `ExecutionEvent`: bounded progress and cancellation checkpoints that do not
  encode presentation behavior.

Do not repurpose `rspice-engine-adapter` as this layer. That crate owns a cloud
worker protocol and should consume the shared execution layer like every other
adapter.

## Dependency order

| Phase | Purpose | Findings | Depends on |
|---|---|---|---|
| 0 | Freeze regressions and reconcile current state | All | None |
| 1 | Restore buildability and enforce safe result contracts | AR-01, AR-18, AR-21, AR-22 | 0 |
| 2 | Implement shared deck planning and run orchestration | AR-03, AR-07, AR-19 | 1 |
| 3 | Correct analysis/output semantics and projection | AR-02, AR-04, AR-05, AR-06, AR-08 | 2 |
| 4 | Make cancellation universal | AR-09, part of AR-10 | 1; integrate with 2-3 |
| 5 | Make transient compression and checkpointing trustworthy | AR-11, AR-12 | 1-4 |
| 6 | Make every native output transactional | AR-17 | 3 and 5 |
| 7 | Complete cross-surface CLI/Python/WASM result parity | AR-08, AR-10 | 2-6 |
| 8 | Close analog grammar/import/statistics gaps | AR-15, AR-20 | 2; can run beside 4-7 |
| 9 | Expand transient integration methods if qualified | AR-16 | 5 |
| 10 | Expand native periodic/RF and analog model coverage | AR-13, AR-14 | 1, 4, 5; can begin after capability contracts stabilize |
| 11 | Run production release gates and remove temporary compatibility code | All | 1-10 |

Phases 0-7 are the correctness-critical release blocker. Phases 8-10 are larger
compatibility and capability programs. They may be developed in parallel after
their shared contracts stabilize, but none may bypass the Phase 11 gate.

## Phase 0: freeze every audit reproduction

### 0.1 Add durable regression decks

Add small, deterministic fixtures under
`crates/rspice-cli/tests/fixtures/audit_regressions/` and integration tests in a
new `crates/rspice-cli/tests/audit_regressions.rs` for:

- valid one-tone and two-tone `.DISTO`, including `f2/f1=0.9`;
- conditional `.STEP` topology in forward and reverse coordinate order;
- `.SAVE @D1[id]` with `.PRINT` as a comparison;
- `.SP ... donoise` in keyword, numeric true, numeric false, and malformed form;
- `.FOUR 1k V(out) I(V1)` attached to an authored `.TRAN`; and
- `.TEMP` with `.TRAN` and at least one frequency-domain analysis.

Each test asserts exit status, analysis identity, signal schema, nonempty typed
payload, and physically meaningful values. A CSV header-only or time-only file
must fail the test even when the process exits zero.

### 0.2 Add lower-level contract regressions

Add core tests for:

- `SignalSchema` equality, union, and explicit missingness;
- topology fingerprints that change only when elaborated topology changes;
- all output-symbol kinds, including device parameters and differential probes;
- a compressed/uncompressed result inventory comparison;
- checkpoint preflight with each resume-blocker family; and
- every long-running analysis responding to a deterministic test abort source.

### 0.3 Capture a machine-readable baseline

Record, without blessing incorrect numbers:

- toolchain and feature set;
- supported analysis/result matrix for core, CLI, Python, and WASM;
- existing ngspice/Xyce/analytical oracle pass counts;
- deterministic hashes for selected typed result documents; and
- warnings, dependency advisories, and panic sites reachable from public input.

The earlier `event-listener` advisory is currently absent from
`cargo tree --locked`. Close AR-21 only after CI proves `cargo deny check
advisories` (or the repository's equivalent security command) runs against
`Cargo.lock` on every change and fails on unacknowledged advisories.

### Phase 0 acceptance

- Every open finding has a red automated test or an explicit source-level
  capability test.
- Previously fixed/partially fixed behavior is represented by a green regression
  and marked accordingly; it is not reimplemented.
- Fixtures are minimal enough to diagnose a single behavior and run in normal
  CI, not only a manual audit script.

## Phase 1: buildability, schema safety, and error contracts

### 1.1 Fix the WASM exhaustive match

Add the explicit `OutputDirectiveKind::Fft => "fft"` mapping in
`crates/rspice-wasm/src/lib.rs`, then add a test that serializes every enum
variant. Avoid a wildcard arm: future output directives must produce another
compile failure until their public mapping is intentionally added.

### 1.2 Remove input-reachable panic paths

Use existing checked accessors such as `SimulationResult::try_voltage` and
`TransientResult::try_voltage_at` throughout deck execution, aggregation, and
export. Add checked branch and device-observable accessors where equivalent
APIs do not exist.

Replace assumptions in `crates/rspice-cli/src/commands/run_signals.rs` with a
schema validation result. In particular:

- never infer all later coordinate shapes from the first result;
- never call `SimulationResult::voltage` with an unvalidated coordinate-local
  node ID; and
- remove `unwrap_or_default()` for missing branch currents.

Keep explicitly documented panic convenience methods only if they cannot be
reached by authored input through first-party surfaces. Internally, deny their
use in execution/projection modules.

### 1.3 Establish a typed error taxonomy

Extend `SimulationError`/surface mappings to distinguish:

- invalid authored input;
- unsupported but well-formed capability;
- materialization/topology mismatch;
- requested signal unavailable;
- result-schema mismatch;
- resource limit;
- cancellation/timeout;
- persistence incompatibility; and
- output commit failure.

CLI exit codes, Python exceptions, WASM error objects, and engine-adapter wire
errors must preserve the category, analysis ID, coordinate, and source span
where available.

### 1.4 Make lint and dependency health ratchets

- Run Clippy with warnings denied for changed non-UI crates and then for the
  full in-scope workspace.
- Inventory `panic!`, `unwrap`, `expect`, `todo!`, and `unimplemented!` in
  input-reachable production modules. Eliminate them or document/test the local
  invariant.
- Keep the security advisory gate locked to the committed dependency graph.
- Do not suppress warnings at crate scope to reach green.

### Phase 1 acceptance

- Native and `wasm32-unknown-unknown` checks for `rspice-wasm` pass.
- Both conditional-topology orderings return a typed result or typed error and
  cannot panic.
- No schema mismatch becomes zero or an omitted column.
- In-scope Clippy and dependency security gates are green.

## Phase 2: one deck planner for all surfaces

### 2.1 Normalize meta-analyses into run axes

`DeckPlan` must treat `.STEP`, `.TEMP`, `.ALTER`, and `.DATA` as axes or deck
variants, not as unrelated analyses. Define the order explicitly:

1. expand textual `.ALTER` variants;
2. resolve referenced `.DATA` tables;
3. build `.STEP` and `.TEMP` Cartesian coordinates;
4. evaluate coordinate-dependent conditionals and parameters;
5. materialize topology and output requests; and
6. execute each authored physical analysis instance in authored order.

If a deck has axes but no physical analysis, preserve the existing implicit OP
behavior only as an explicit `AnalysisRequest::ImplicitOp` with provenance. Do
not let `.TEMP` create extra OP analyses when `.TRAN`, `.AC`, or another
physical analysis is present.

### 2.2 Support repeated analyses without collisions

Replace kind-only output naming with `AnalysisInstanceId { kind, ordinal }`.
For example, two authored `.AC` cards become `ac-001` and `ac-002`, and every
coordinate adds its stable run ID. Apply the same IDs to result lookup,
measurements, logs, filenames, HDF5 groups, JSON documents, checkpoints, and
errors.

### 2.3 Support conditional topology deliberately

Preflight and retain each coordinate's complete `SignalSchema` and topology
fingerprint, not only the child-analysis signature.

For stable topology, wide aggregation is allowed after exact descriptor
equality. For changing topology:

- the typed result retains coordinate-local schemas;
- structured formats use a union schema plus a validity bitmap;
- flat formats write one coordinate artifact or leave absent cells blank with
  an explicit validity companion, according to a documented format policy;
- no format silently chooses the first coordinate's schema; and
- reversing coordinate order produces semantically identical named data.

### 2.4 Expand `.STEP` composition

After ordinal namespaces exist, permit `.STEP` to wrap every physical analysis
that the core can execute, including noise, SP, distortion, pole-zero,
sensitivity, PSS/PAC/PNoise, HB, Monte Carlo where nesting semantics are
unambiguous, and post-processing directives attached to their parent result.

Define and test:

- `.STEP` plus `.ALTER`/textual `.DATA`;
- `.STEP DATA=<table>` without double expansion;
- multiple step dimensions and temperature axes;
- per-coordinate checkpoint names;
- per-coordinate resource accounting and aggregate result budgets; and
- cancellation between and within coordinates.

Do not enable mathematically ambiguous nesting by accident. For example, an
outer authored Monte Carlo and an inner step require an explicit deterministic
seed derivation contract before support is declared.

### 2.5 Migrate every surface

- CLI `commands/run.rs` becomes an adapter over `DeckPlan`.
- Python `engine/directives.rs` uses the same plan; remove its independent
  `.STEP` and `.TEMP` OP behavior.
- WASM and `rspice-engine-adapter` consume the plan instead of maintaining
  analysis whitelists.

### Phase 2 acceptance

- The same deck produces the same ordered analysis/coordinate IDs and typed
  schemas in core, CLI, Python, WASM, and engine-adapter contract tests.
- `.TEMP` wraps authored TRAN/AC/etc. without extra nominal or OP runs.
- Repeated analysis kinds and `.STEP` plus `.ALTER`/`.DATA` cannot collide.
- Conditional topology is order-independent and contains no fabricated data.

## Phase 3: correct analysis and output semantics

### 3.1 Replace CLI `.DISTO` AC substitution

In `crates/rspice-cli/src/commands/run/frequency.rs`:

- remove the linear AC implementation and the inverted ratio validation;
- validate one-tone omission or `0 < f2/f1 < 1` consistently with core;
- call the core distortion/Volterra entrypoint;
- project requested node, branch, and device observables from each distortion
  product; and
- export typed product identity (`2f1`, `3f1`, `f1+f2`, `f1-f2`, `2f1-f2` as
  applicable), complex value, magnitude/phase, and normalization provenance.

Add analytical weakly nonlinear diode tests and compare selected points to an
independent oracle within declared magnitude and phase tolerances.

### 3.2 Complete `.SAVE` projection

Make `SignalProjection` the only implementation of `.SAVE`, `.PRINT`, `.PROBE`,
and `.PLOT` selection. Materialize `@device[param]` through the core's device
observable registry for OP, DC, AC, TRAN, distortion, and other analyses where
the observable is defined.

If a requested observable cannot be supplied for that analysis/device, return
`RequestedSignalUnavailable` with the exact authored symbol. A successful
time-only file is not an acceptable response.

### 3.3 Implement `.SP donoise` end to end

- Parse case-insensitive `donoise` as documented.
- Retain numeric `0`/`1` only if it is a deliberate dialect-compatibility form;
  malformed trailing tokens must fail.
- Invoke the existing port-noise computation when enabled.
- Expose noise figure/parameters, correlation data, reference temperature, and
  normalization alongside S-parameters in every capable result format.
- Assert `donoise=0` does not perform or claim noise work.

### 3.4 Attach `.FOUR` to authored transient data

Represent `.FOUR` as a transient post-process request, not a separate
independently configured simulation. Parse its outputs through the common
output-symbol resolver so voltage differences, branch currents, and supported
device observables are all valid inputs.

Validate that the authored transient interval contains enough settled periods
and samples for the requested fundamental. If not, fail with the exact required
time/window information; do not silently invent a transient stop or step.

### 3.5 Publish `.FFT` everywhere

Represent `.FFT` as a typed transient post-process result with:

- source signal descriptor;
- window, detrending, sample interval, bin frequencies, and normalization;
- complex bins plus derived magnitude/phase where requested; and
- the parent analysis/coordinate ID.

Add CLI JSON/CSV/HDF5/raw policy, Python accessors and versioned pickle state,
WASM typed-array DTOs, and engine-adapter wire representation. Compressed and
uncompressed parent transients must retain identical FFT results because FFT is
computed from the qualified source trajectory, not reconstructed from an
insufficient compressed subset.

### Phase 3 acceptance

- All six Phase 0 CLI reproductions are green with numerically meaningful data.
- Every authored output is present or produces a typed error.
- `.FOUR` and `.FFT` identify and consume the authored transient instance.
- Cross-format round trips retain analysis/product/signal identities and units.

## Phase 4: universal cancellation and timeout

### 4.1 Require abort-aware core execution

Add or expose `_with_abort` entrypoints for every analysis and long
post-processing operation. Where a non-abort convenience wrapper remains for
third-party embedding, first-party surfaces must not call it.

Instrument bounded checks in:

- nonlinear/Newton and frequency loops;
- Monte Carlo sample and inner-analysis loops;
- PSS shooting, HB/APFT/Krylov, PAC, and PNoise loops;
- noise, SP noise, distortion, pole-zero, sensitivity, and STB sweeps;
- Fourier/FFT transforms and large result projections;
- `.STEP`/`.TEMP`/`.ALTER` coordinate materialization;
- checkpoint serialization and large output preparation; and
- compression/resampling.

### 4.2 Standardize surface behavior

- CLI: one cancellation source combines Ctrl-C and timeout, maps to stable exit
  semantics, stops progress workers, and does not commit output.
- Python: expose a cancellation token, release the GIL for long work, and map
  cancellation to one documented exception without poisoning the engine.
- WASM: expose an abort handle suitable for Web Worker execution. Use a shared
  atomic/cooperative path when available and a documented worker-termination
  fallback; never imply that an event on a blocked main thread can cancel a
  synchronous solver.
- Engine adapter: propagate caller cancellation into the same core token.

### 4.3 Define a latency budget

Create deterministic stress fixtures and require cancellation to be observed
within a bounded number of solver iterations/points. Wall-clock thresholds may
be generous in shared CI, but an iteration-bound assertion must be exact.

### Phase 4 acceptance

- Every analysis has a test that aborts after a deterministic work count.
- CLI timeout/Ctrl-C, Python token cancellation, and WASM worker cancellation
  all return the same error category and leave no committed artifact.
- An engine remains usable after cancellation unless the backend explicitly
  returns a fatal-state error.

## Phase 5: trustworthy transient compression and checkpointing

### 5.1 Redesign the compressed result as a complete result container

Replace the voltage-only shape with a descriptor-indexed set of compressed
series. It must preserve:

- time and real accepted step sizes;
- node voltages and branch currents;
- analog device observables and stored real traces;
- event/digital trace containers as opaque typed channels for integration with
  the separate digital effort, without implementing digital behavior here;
- node/branch/device names, units, directions, and sign conventions;
- validity and nonfinite policy;
- FFT/Fourier/measurement post-results;
- compression configuration, input-point count, error statistics, and version;
  and
- parent analysis/coordinate/topology identity.

Do not fabricate `vec![0.0; len]` step sizes in
`From<TransientResultCompressed> for TransientResult`. Either store the actual
series or make expansion return a typed error when exact metadata is absent.

### 5.2 Qualify compression error per signal

Compression is allowed to approximate waveform samples only within its stated
absolute/relative tolerance. Test each signal kind for:

- extrema and narrow pulses;
- discontinuities and source breakpoints;
- oscillatory/ringing waveforms;
- current sign and near-zero crossings;
- nonuniform timesteps; and
- interpolation at authored output points.

Report worst observed error and the signal/time at which it occurs. Never use
compression ratio alone as a success criterion.

### 5.3 Decouple solver state, output schedule, and stored waveform

The transient executor should feed three independent consumers:

1. exact accepted solver state for continuation/checkpoint;
2. authored output-schedule projection; and
3. optional compressed result storage.

This allows compression to compose with `.TRAN` output intervals,
`--checkpoint`, `--resume`, and authored restart without changing numerical
integration or losing exact resume state.

### 5.4 Version Python and wire persistence

Add versioned pickle state for full and compressed transient results. Round-trip
all signal families, `store_traces`, FFT results, metadata, and validity. Reject
malformed or unsupported future states precisely. Provide an explicit migration
path for any already persisted old state; do not silently construct missing
fields as zeros.

### 5.5 Refuse unusable checkpoints before writing

Split checkpoint creation into:

- `capture_resumable`, which validates all line, device, nonlinear, junction,
  integration, and extension-state blockers before serialization; and
- an optional clearly different diagnostic snapshot type that is never accepted
  by resume.

When the user requests a resume checkpoint, preflight capability immediately
after elaboration. If a later dynamic blocker can still arise, fail the
checkpoint operation and leave no checkpoint file. A file accepted as a
checkpoint must successfully pass structural resume validation for the exact
captured netlist/configuration.

### Phase 5 acceptance

- Compressed versus uncompressed inventory tests show identical descriptors and
  post-results, with waveform differences inside declared tolerance.
- Checkpoint/resume plus compression and output scheduling passes segmented
  versus monolithic equivalence tests.
- Python pickle and engine wire round trips lose no typed fields.
- No successful checkpoint-save operation produces a known-unresumable file.

## Phase 6: transactional output and persistence

### 6.1 Add one native atomic-artifact helper

Create a small workspace crate, `rspice-output`, used by CLI and Python native
exports. It should:

- create a uniquely named temporary file in the destination directory;
- preserve the existing destination until the new artifact is complete;
- write through a closure or streaming writer;
- flush and synchronize file data when requested by the product durability
  policy;
- atomically replace/rename using platform-correct semantics;
- synchronize the parent directory where supported;
- clean up temporary files on error/cancel; and
- report preparation, write, flush, and commit failures distinctly.

The helper is an implementation utility, not a packaging/distribution project.

### 6.2 Migrate every direct writer

Migrate at minimum:

- `crates/rspice-cli/src/commands/export_table.rs`;
- direct writers in `commands/run/basic.rs`, `frequency.rs`, `advanced.rs`, and
  `report.rs`;
- HDF5 output by writing a complete temporary HDF5 artifact before commit; and
- `crates/rspice-python/src/export.rs`.

Multi-file outputs use a manifest/transaction directory so a cancellation
cannot publish half of a coordinate set as a complete run.

### 6.3 Add fault-injection tests

Inject failure after headers, midway through values, during flush, and before
rename. Test both absent and pre-existing destinations. The old file must remain
byte-identical until successful commit.

### Phase 6 acceptance

- No in-scope production exporter opens the final path with truncation before
  successful completion.
- Failure/cancellation tests leave the old complete file or no file.
- Temporary artifacts are recoverable/diagnosable during a process crash and
  cleaned on the next controlled run according to policy.

## Phase 7: CLI, Python, and WASM surface parity

### 7.1 Define a capability matrix generated from code

Generate a checked-in/readable matrix from registered `AnalysisResult` and
`SignalDescriptor` adapters. It must cover OP, DC, AC, TRAN, noise, SP and port
noise, distortion, TF, STB, sensitivity, pole-zero, Fourier, FFT, Monte Carlo,
PSS, PAC, PNoise, and HB, plus stepped/temperature variants.

The matrix is a test input: adding a core result variant without a surface
mapping must fail compilation or CI, as AR-01 did for FFT.

### 7.2 Complete Python projection

- Make `Engine.run()` consume `DeckPlan` and return ordered typed run results.
- Add missing FFT and compressed-result accessors.
- Preserve analysis ordinals, coordinate IDs, units, complex values, branch
  currents, device observables, and missingness.
- Keep direct convenience methods as thin constructors of the same
  `AnalysisRequest`; do not allow them to drift semantically.

### 7.3 Complete WASM execution and results

Replace the OP/AC/TRAN switch in `crates/rspice-wasm/src/lib.rs` with adapters
over all target-safe core result variants. Use typed arrays for large real and
complex series and a small metadata object for descriptors/provenance.

Expose:

- branch currents and device observables;
- all analysis and post-process result kinds in the capability matrix;
- coordinate/ordinal IDs and explicit missingness;
- resource-limit and cancellation controls; and
- incremental/bounded result transfer so a large run does not require multiple
  full JavaScript copies.

This phase covers the simulator's browser API, not browser UI work.

### 7.4 Align engine-adapter documents

Remove any voltage-only or analysis-subset assumptions in
`rspice-engine-adapter`. Version the wire document when adding result variants
and test forward-version rejection and same-version round trips.

### Phase 7 acceptance

- The generated matrix has no unimplemented in-scope surface cell.
- A shared deck corpus produces semantically equal typed documents from CLI
  JSON, Python, WASM, and engine adapter after representation normalization.
- Large WASM results stay within configured allocation/copy budgets and can be
  cancelled.

## Phase 8: analog grammar, import, and statistics completeness

### 8.1 Parenthesized subcircuit actual nodes

Extend the native SPICE grammar to accept the applicable dialect's
parenthesized actual-node list on subcircuit instances without confusing it
with parameter expressions. Preserve source spans and strict arity checks.
Replace the current intentional rejection test with positive, ambiguous, and
malformed cases across nested subcircuits and continuation lines.

### 8.2 Differential `.IC` and `.NODESET`

Represent `V(a,b)=value` as a differential constraint, not as an arbitrarily
rewritten pair of single-ended values. Define its behavior for:

- ordinary operating-point initial guesses;
- `.TRAN UIC` initial conditions;
- multiple consistent and inconsistent constraints;
- one terminal at ground; and
- subcircuit-qualified nodes.

Use a small constraint solver or explicit MNA startup constraint as required;
detect rank deficiency/conflict with a source-located error.

### 8.3 Continuous-measure `FAILVALUE`

Implement per-record verification semantics for continuous DC/AC/TRAN measure
modes. Each record retains raw value, threshold, pass/fail, and coordinate; the
analysis aggregate reports failure if the documented policy is met. Add
CSV/JSON/JUnit/TAP coverage without reducing a stream to one ambiguous scalar.

### 8.4 Split legacy Xyce Y-device scope

Inventory every currently rejected Y keyword in the conformance corpus.
Classify each as:

- analog device that belongs in this plan;
- digital/code-model device owned by the separate digital effort; or
- deprecated alias of an existing native element.

Implement analog families through typed native element lowering and device
tests. Keep digital families explicitly rejected here until the other effort's
capability registration is integrated. Never parse an unknown Y keyword as a
transmission line.

### 8.5 Complete SPEF analog lowering

- Parse and materialize reduced `*R_NET`/`*C_NET` forms with conservation and
  unit validation.
- Parse `*INDUC` into inductors/mutual coupling or the appropriate reduced
  network representation.
- Preserve hierarchical names and source provenance.
- Compare extracted RC/RLC network impedance/admittance against analytical
  fixtures over frequency and transient step response.

### 8.6 Complete Spectre statistical semantics

Implement executable lowering for:

- process versus mismatch scope;
- independent and correlated variation;
- Gaussian, uniform, and lognormal distributions;
- absolute and percent-relative variation; and
- deterministic seed/substream derivation across `.STEP`, `.TEMP`, Monte Carlo,
  and parallel execution.

Validate correlation matrices for symmetry and positive semidefiniteness. Add
statistical property tests for mean, variance, correlation, and reproducibility,
using generous probabilistic tolerances plus exact seeded sequences.

### 8.7 Broaden native Spectre analog statements/models

Build a machine-readable inventory from the existing Spectre corpus and every
explicit `unsupported native Spectre` branch in `spectre_adapter.rs`. Implement
canonical lowering for in-scope analog passives, independent/dependent and
behavioral sources, semiconductor instances/model cards, subcircuits,
parameters, includes/sections, sweeps, analyses, saves, and statistics.

`ahdl_include` and Verilog-A model execution remain with the separate effort;
this plan owns only the boundary contract and must preserve those statements
for that backend rather than discard them.

### Phase 8 acceptance

- All newly supported grammar has positive, malformed, and ambiguity tests.
- In-scope analog Xyce/Spectre/SPEF corpus entries parse, lower, and simulate;
  exclusions are explicitly classified, not silently skipped.
- Statistical results satisfy seeded reproducibility and distribution tests.

## Phase 9: integration-order qualification

AR-16 must begin with a compatibility decision backed by authoritative dialect
behavior and actual product requirements. If orders above two are not valid for
the selected OneStep/Gear12 compatibility mode, keep the explicit rejection and
close the finding as an intentional, tested boundary. If higher-order Gear/BDF
is a product requirement, implement it as follows.

**Decision (2026-09-01):** Current authoritative Xyce documentation explicitly
defines variable-order trapezoidal and Gear methods over orders 1 and 2. RSpice
therefore retains its fail-closed 1/2 boundary; higher-order work in 9.1 through
9.3 is not required by the current compatibility contract. The evidence,
existing enforcement, and conditions for reopening the decision are recorded
in `TRANSIENT_INTEGRATION_ORDER_DECISION.md`.

### 9.1 Generalize integration history

Replace order-1/order-2 fixed fields with a bounded, versioned history capable
of the required maximum order. Implement variable-step BDF coefficients,
predictors, LTE estimates, startup order growth, rejection rollback, breakpoint
restart, and order selection without allocating in the accepted-step hot path.

### 9.2 Update every dynamic device and persisted state

Generalize capacitor, inductor, charge-based semiconductor, behavioral
integral/derivative, transmission-line, and other native analog histories.
Extend checkpoint identity/serialization and PSS continuation state. Coordinate
Verilog/digital history changes through their separate owner rather than editing
those implementations in this phase.

### 9.3 Apply numerical stability policy

Higher-order BDF is not A-stable above order two. Define order ceilings and
automatic demotion for oscillatory/stability-sensitive cases. Preserve exact
breakpoint landings and never carry invalid high-order history across a source
discontinuity or resume boundary.

### Phase 9 acceptance

- Polynomial-order and manufactured-solution tests demonstrate expected
  convergence order on uniform and nonuniform grids.
- Stiff, oscillatory, switching, and breakpoint corpora meet stability/error
  limits and do not regress Gear1/Gear2 behavior.
- Checkpoint/resume and monolithic trajectories agree within the integrator's
  declared tolerance for every supported order.

## Phase 10: native periodic/RF and analog model expansion

This is an XL capability program and should be delivered in independently
qualified slices. First introduce capability descriptors; then add device
families from simpler stamps to stateful distributed models. Do not add a
device to an allowlist before its residual, Jacobian, charge/state, and noise
contracts are complete for that analysis.

### 10.1 Add shared periodic-analysis device capabilities

Define native traits/descriptors for:

- periodic residual and exact Jacobian;
- charge and dynamic-state contribution;
- small-signal/PAC descriptor contribution;
- stationary and cyclostationary noise sources/correlations;
- PSS period-map state capture/restore; and
- envelope initialization/continuation.

The interface must support future Verilog-A capability metadata without making
runtime Verilog-A part of this plan.

### 10.2 Expand exact HB/PAC device coverage in tiers

1. Advanced diode equations, classic MOS/JFET variants, voltage/current/generic
   switches, behavioral sources, and solution-dependent capacitance.
2. Native BJT/VBIC, BSIM3, BSIM4, B3SOI, EKV/VDMOS and other already-native
   semiconductor families.
3. Memristors, hysteretic/nonlinear magnetic devices, mutual structures, and
   lossy/distributed transmission lines.

For each device tier, test:

- zero-harmonic agreement with DC;
- small-perturbation agreement with AC/PAC;
- analytic versus finite-difference Jacobians over a bounded physical domain;
- HB waveform reconstruction versus long-settled transient; and
- conservation, passivity, and power balance where applicable.

### 10.3 Complete PSS and envelope state

Replace the ordinary-capacitor/inductor allowlist with capability-driven capture
of every native analog dynamic state. Include semiconductor charge states,
transmission-line history, hysteretic magnetic state, memristive state,
behavioral integrals/delays, and switch memory as their exact implementations
become available.

Envelope initialization must use the same complete state descriptor and reject
stale/incomplete artifacts by identity, not by a manually maintained device
list.

### 10.4 Implement cyclostationary colored PNoise

Generalize resistor, diode, MOS, and JFET flicker-noise treatment from DC-bias
substitution to periodically modulated source/correlation spectra with sideband
folding. Retain contributor identity and cross-correlation. Validate stationary
limits against ordinary noise and modulated cases against analytical mixers or
an independent periodic-noise oracle.

### 10.5 Implement AC-NQS pole-zero states

Augment PZ descriptor extraction with hidden charge-deficit states for BSIM3
and BSIM4 `ACNQSMOD=1`. Verify poles/zeros against direct complex-frequency
evaluation and independent small-signal references.

### 10.6 Generalize EKV3 noise

Remove the exact-fixture gate only after implementing general topology,
frequency, bias, temperature, and source/output handling for EKV3 noise. Share
the implementation between ordinary noise and SP noise. Preserve the checked
Xyce fixture as one oracle rather than the executable definition of support.

### 10.7 Close native passive/model gaps

- Implement LTRA shunt conductance `G != 0` in DC, AC, transient, noise, and
  periodic analyses, including stable convolution/history and passivity tests.
- Implement Xyce-compatible inductor synthesis from `NT`, `CSECT`, and `LENGTH`
  with units, geometry validation, temperature behavior, and analytical
  qualification.
- Resolve HICUM/MEXTRAM ownership before coding. If these are delivered by the
  separate Verilog-A/model effort, this plan adds only native parser/routing,
  result, and analysis-capability integration tests. If native implementations
  are required, create separate model qualification plans with equation-source,
  DC/AC/TRAN/noise, temperature, geometry, convergence, and oracle gates. A
  name-only model route does not close AR-14.

### Phase 10 acceptance

- No supported native analog device reaches an advanced-analysis allowlist
  without capability and oracle tests.
- PSS/HB/PAC/PNoise/PZ fail only for explicitly excluded capability, never due
  to stale family lists.
- Energy/conservation, Jacobian, stationary-limit, and transient-comparison
  gates pass for each delivered slice.

## Phase 11: production qualification and release gate

### 11.1 Required automated gates

Run at least:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --locked -p rspice-matrix
cargo test --locked -p rspice-core
cargo test --locked -p rspice-cli
cargo test --locked -p rspice-engine-adapter
cargo test --locked -p rspice-python --lib
pytest crates/rspice-python/tests
cargo test --locked -p rspice-wasm
cargo check --locked -p rspice-wasm --target wasm32-unknown-unknown
cargo test --locked -p rspice-conformance
cargo deny check
```

If full `--all-features` conflicts with mutually exclusive production backends,
replace it with an explicit checked feature matrix; do not omit a feature path.
UI-only and separately owned Verilog/digital suites may be reported by their
owners, but shared-contract changes must not knowingly break them.

### 11.2 Numerical qualification

For every changed analysis/device:

- analytical unit fixtures;
- ngspice/Xyce or other independent simulator comparison where compatible;
- convergence-order or residual/Jacobian checks;
- temperature and parameter boundary sweeps;
- determinism across repeat runs and parallel scheduling;
- resource-limit and allocation-failure behavior; and
- nonfinite/invalid-parameter fail-closed tests.

Tolerance must be declared per quantity. A single broad relative tolerance is
not acceptable near zero; use combined absolute/relative and phase-aware
criteria.

### 11.3 Robustness qualification

- Fuzz parsers, import adapters, checkpoint/pickle/wire decoders, and output
  symbol resolution.
- Property-test run-axis cardinality, coordinate IDs, schema unions, and
  serialization round trips.
- Fault-inject allocation, I/O, cancellation, malformed data, and incompatible
  persistence versions.
- Run panic detection over the netlist corpus; a process panic is a gate
  failure even if an outer harness catches it.

### 11.4 Performance and resource qualification

Capture before/after baselines for OP/DC/AC/TRAN/noise/HB/PSS and large output
projection. Correctness fixes may add required work, but unexplained regressions
must be investigated. Gate:

- solver time and iteration counts;
- peak resident memory and WASM linear memory;
- result-copy counts and serialized size;
- cancellation observation latency;
- compression time/error/ratio; and
- checkpoint/output throughput.

Do not accept a performance optimization that changes numerical results outside
the qualified tolerance or weakens an error check.

### 11.5 Documentation generated from contracts

Generate the public analysis/signal/surface support matrix, error categories,
checkpoint/pickle/wire versions, and intentional unsupported boundaries from
registered capabilities. Documentation is not a substitute for implementation,
but generated documentation prevents claims from drifting away from code.

## Traceability and closure evidence

| Finding | Primary work packages | Minimum closure evidence |
|---|---|---|
| AR-01 | 1.1, 7.1 | Native and wasm-target compile; all output enum variants serialize |
| AR-02 | 3.1 | Valid ratio accepted, invalid rejected, nonzero qualified Volterra products exported |
| AR-03 | 1.2, 2.3 | Forward/reverse conditional-topology runs are equivalent and panic-free |
| AR-04 | 3.2 | `.SAVE @device[param]` appears with qualified values or precise unsupported error |
| AR-05 | 3.3 | Keyword/numeric parse tests and complete port-noise exports |
| AR-06 | 3.4 | Current and voltage Fourier outputs use the authored transient ID/data |
| AR-07 | 2.1, 2.5 | TEMP wraps each authored physical analysis identically in CLI/Python/WASM |
| AR-08 | 3.5, 5.4, 7 | FFT survives every surface, compression, pickle, and wire round trip |
| AR-09 | 4 | Deterministic abort tests for every long-running path |
| AR-10 | 4.2, 7.3 | Generated matrix complete; currents/advanced results/cancellation available |
| AR-11 | 5.1-5.4 | Inventory equality, error bounds, schedule and segmented-run equivalence |
| AR-12 | 5.5, 6 | Known blocker refuses save and commits no checkpoint artifact |
| AR-13 | 10.1-10.6 | Per-capability device/oracle matrix and stationary/transient limits |
| AR-14 | 10.7 | LTRA/inductor oracle tests; explicit HICUM/MEXTRAM ownership and full qualification |
| AR-15 | 8.5-8.7 | SPEF electrical equivalence and Spectre corpus/statistics gates |
| AR-16 | 9 | Evidence-backed intentional boundary or fully qualified higher orders |
| AR-17 | 6 | Fault injection proves old-or-complete atomic behavior |
| AR-18 | 1.2-1.3, 2.3 | No authored-input panic and no zero-filled missing signal |
| AR-19 | 2.2-2.4 | Repeated/mixed analyses, ALTER/DATA composition, unique output/checkpoints |
| AR-20 | 8.1-8.4 | Positive/malformed grammar tests and analog/digital Y classification |
| AR-21 | 0.3, 1.4 | Locked advisory gate green; no undocumented ignore |
| AR-22 | 1.2-1.4, 11 | Warning-denied builds and audited public panic/error surface |

## Delivery policy

1. Land Phase 0 regressions before their fixes whenever the current tree can
   express the test.
2. Keep architectural refactors behavior-preserving and separate from numerical
   changes. Do not combine a new deck planner, model equation, and output format
   in one unreviewable change.
3. Deliver one independently reviewable work package at a time with tests and
   compatibility notes in the same change.
4. Do not remove a fail-closed rejection until the replacement path is fully
   qualified.
5. Version every persisted or wire-visible schema change and test old-state
   handling before merging.
6. At the end of each phase, update the finding register with commit IDs, test
   names, oracle provenance, remaining exclusions, and measured regressions.
7. Do not declare production readiness while any P0/P1 finding is open, while a
   surface can silently drop data, or while an in-scope advertised capability
   lacks independent numerical qualification.

## Recommended first implementation sequence

The first changes should be small enough to land quickly but chosen to establish
the contracts needed by later work:

1. add the Phase 0 six-deck CLI regression suite;
2. fix AR-01 with the exhaustive enum-mapping test;
3. replace zero-filling/panicking aggregation with checked `SignalSchema` use;
4. introduce `AnalysisInstanceId`, `RunCoordinate`, and `MaterializedRun`;
5. migrate `.STEP` and `.TEMP` to the shared planner;
6. migrate output selection to `SignalProjection` and fix `.SAVE` device
   parameters;
7. replace CLI `.DISTO`, then attach `.FOUR`/`.FFT` to parent transient results;
8. complete `.SP donoise` parsing/export;
9. make all first-party analysis calls abort-aware;
10. redesign transient compression/checkpoint contracts;
11. migrate output writers to atomic commit; and
12. complete Python/WASM/engine-adapter parity before beginning broad
    parser/model/RF expansion.

This sequence closes the known wrong-answer and crash risks first and leaves a
stable execution/result foundation for the larger analog compatibility and
numerical programs.
