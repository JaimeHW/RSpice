# RSpice core remaining implementation plan

Status: implementation in progress; no package is closed yet. Prepared October 5, 2026 against revision `20fd0d6350596c0cfc814ee729f4e2286dd30d2b`; initial execution baseline is `f24e2ea28f2322e78086c14837ea53bc2d4c18c3`.

Complete the unfinished work identified by the core audit: missing analysis and model support, incomplete control execution, IMD measurements, architecture and duplication problems, and core qualification. This plan covers `rspice-core`, its public Rust interfaces, and the dependency contracts needed by its solvers. Application UI, packaging, deployment, and product-level platform certification are outside this plan.

The previous repair batch fixed the reproduced transient, transfer-function, stability, and measurement defects. It did not complete the capability gaps below. An unsupported-capability error protects callers while implementation proceeds; adding that error, hiding an option, or changing documentation does not complete a missing implementation.

## Completed repairs to preserve

| Area | Completed behavior | Commit |
|---|---|---|
| Transfer functions | Suppress independent excitations throughout hierarchy; share the operating point and AC factorization; support hierarchical sources and collision-free probes | `b0066a72f` |
| Transient events | Reserve the future Xyce integration floor before a physical event | `1b8829268` |
| Restart state | Keep LTE references aligned with the accepted solution after a projected restart | `590dfff1b` |
| Stability | Validate sweep samples and unwrap phase continuously before extracting margins | `2f2305b30` |
| AC measurements | Include the interval adjacent to the peak when finding the lower cutoff; clear stale characteristics; correct phase interpolation and bounded group delay | `762e31f25` |
| Post-processing | Use checked measurement APIs and shared Fourier integration; remove fabricated carrier, spur, and invalid-power results | `f6ce6fe48` |
| Transfer probe edge cases | Apply branch-impedance conventions and handle ground-only probes | `707353f1d` |
| Qualification baseline | Reconcile the added oracle deck and expanded checkpoint schema without increasing tolerances | `942e8cc87` |
| Static analysis | Remove stale lint expectations and correct iterator and conditional warnings | `23236d5ee` |
| Pole-zero admission | Reject irrational transmission delays from finite rational descriptors while preserving eligible periodic analyses; integrated from concurrent core work | `0815435d6` |

The repair batch recorded 4,843 passing core unit tests, three ignored tests, 126 passing selected integration tests, strict library Clippy, and successful core/dependent build checks. These are historical validation results, not a certificate for every model or analysis. Re-establish the baseline in C00 before further implementation.

## Completion rules

1. Exercise each delivered capability through a public core entry point. Private test admission paths and individual model stamps alone do not establish working analysis support.
2. Trace every remaining finding to a work package, implementation, permanent regression, and qualification record. Update the capability declarations only after the corresponding implementation passes.
3. Keep invalid input, unavailable evidence, unsupported mathematical formulations, nonconvergence, cancellation, and resource exhaustion distinguishable. Do not substitute zeros, fabricated finite floors, or a successful empty result.
4. Preserve accepted state across rejected trials and failed operations. Publish a result or checkpoint only after its state and numerical certificates are valid.
5. Use independent analytical or simulator evidence where applicable. Agreement between two routes that share the same implementation is a consistency check, not an independent accuracy oracle.
6. Keep mathematically inapplicable contracts explicit. For example, an exact transport delay has no finite rational `G + sC` representation. Implement its valid delay-aware analyses; any finite approximation needs an explicit model, order, error controls, and separate qualification.
7. Split work into focused commits, validate each batch, and push to remote `main` as requested. Preserve unrelated changes. A package stays open when any required behavior or acceptance test is missing.

## Capability coverage

The current [capability table](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/engine/periodic_capability.rs) declares 38 device families across six contracts: periodic residual/Jacobian, finite dynamic-state descriptor, periodic small-signal stamp, noise, shooting PSS state, and envelope continuation. Its instance predicates further restrict support. C00 must enumerate both the declarations and those predicates; the table below groups the remaining work without treating all restrictions as equivalent defects.

| Family or restriction | Remaining implementation or qualification | Package |
|---|---|---|
| GP BJT excess phase | Public transient admission, exact-delay and ngspice Weil semantics, error control, restart, and applicable periodic continuation | C03, C06 |
| GP thermal extensions | Actual thermal equations, accepted thermal state, periodic residual and continuation where the authored model requires them | C06, C07 |
| B3SOI DD, FD, PD | Exact periodic residuals, periodic noise, PSS charge history, and envelope state | C06, C08 |
| EKV 2.6, EKV3, VDMOS | Periodic device equations/stamps, noise, PSS state, and envelope continuation | C06, C08 |
| Diodes | Periodic high-injection, recombination, sidewall, tunneling, overlap, and other authored terms excluded by current predicates | C06 |
| JFETs | Remove implementation restrictions on multiplicity, area, junction parameters, temperature, and applicable charge-history variants | C06 |
| Classic MOS and behavioral sources | Verify each restriction against supported instances; implement genuinely missing equations, dynamic operators, or forcing treatment | C06, C07 |
| Capacitors | Applicable live-frequency/dynamic constitutive equations beyond a constant or explicit integral-coordinate stamp | C07 |
| Resistors | Accepted thermal-state evolution in PSS/envelope; flicker modulation beyond currently qualified exponent forms | C07, C08 |
| Native memristors | State-variable linearization, periodic residual, state restoration, applicable noise, and envelope evolution | C07, C08 |
| Voltage, current, and generic switches | Exact controlling equations, hysteresis state, event treatment, PSS closure, and envelope support | C07 |
| Scalar transmission lines | Lossy/convolution history in PSS/envelope; qualify every supported frequency-domain model and preserve exact delay semantics | C07 |
| Coupled transmission lines | Applicable AC/periodic stamps, full convolution history, shooting state, and envelope continuation | C07 |
| Mutual inductors and transformers | Independent shooting coordinates for singular flux constraints; complete multi-winding history and envelope support | C07 |
| Jiles–Atherton and shared Xyce cores | Nonlinear periodic magnetic equations, hysteretic state, and continuation | C07 |
| XSPICE code models | Per-model electrical equations, accepted and event state, eligible small-signal/periodic analyses, and continuation | C09, C10, C11 |
| Runtime Verilog-A | Core consumption of real residual, charge, noise, and state capabilities; explicit treatment of hidden-state and delay operators | C09 |
| Generated Verilog-A | Periodic stamps, declared noise, complete integration state, and envelope support through actual generated evaluators | C09 |
| Mixed Verilog-AMS | Settled equilibrium, dependent linear analyses, complete accepted state, and event-aware periodic/RF/envelope formulations | C10, C11 |
| Cyclostationary noise | Periodically modulated thermal, shot, flicker, and correlated sources for newly supported families | C08 |

Families currently marked complete remain in the regression matrix. An `Inapplicable` declaration must agree with the chosen numerical formulation; it must not conceal missing device equations.

## Work packages

### C00 Establish the executable scope and baseline

**Dependencies:** none. **Deliverable:** a checked-in core requirements ledger and reproducible baseline.

- Inventory every `AnalysisCommand` variant, public engine route, device-family contract, instance restriction, temporary admission guard, and relevant ignored test. Record implemented, incomplete, and mathematically inapplicable cases separately, with source references and owning package.
- Include all six periodic contracts for all 38 current families and tests that require a disposition when a family or analysis is added. Inspect actual dispatch and model contributions, rather than inferring support from the parser or a declaration.
- Capture the exact source revision, Cargo feature set, configuration, simulator/source versions, input decks, and tolerance rationale for each baseline result. Use a clean committed checkout for qualification; distinguish it from development tests containing unrelated local changes.
- Preserve the repair regressions and reconcile outstanding source-derivative work with the existing project audit plan. Do not overwrite or independently redo that work.
- Record numerical accuracy, phase/event timing, conservation error, allocations, peak memory, cancellation latency, and representative solve times. Establish benchmark tolerances from repeatability and engineering requirements.

**Acceptance:** every unfinished item in this plan has a ledger entry and a test or explicit test-development task. The baseline distinguishes compile checks, executed tests, independent comparisons, and unresolved evidence.

### C01 Define shared numerical and state contracts

**Dependencies:** C00. **Deliverable:** internal contracts that new models and analyses can share without duplicating equations or mutable state.

- Define the authoritative static residual, charge/flux, derivatives, noise contributions, accepted state, trial workspace, and observation interfaces. Extend existing abstractions where they already serve these purposes.
- Specify units, unknown ordering, branch-current signs, hierarchy identity, temperature and multiplicity handling, event sides, and derivative conventions. Keep delay histories and finite descriptor coordinates distinct.
- Define prepare, trial, reject, accept, capture, and restore responsibilities. Require state restoration to validate the full image before mutation, including history clocks and model identity.
- Make analysis capability checks depend on usable evaluator/state contracts and instance parameters. A provider registration or support flag alone must not authorize an unstamped device.
- Keep transient, shooting, HB, QPSS, and envelope storage ownership explicit. Share physical equations; retain separate algorithms where their state representations differ.

**Acceptance:** conformance tests expose omitted residual contributions, incorrect units, missing state, failed rollback, and invalid capability advertisements. The abstraction adds no unmeasured allocation or dispatch cost to existing hot paths.

### C02 Decompose transient execution without changing its behavior

**Dependencies:** C01. **Deliverable:** a readable transient driver with explicit acceptance boundaries.

Primary code: [transient driver](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/engine/transient.rs), its existing `startup`, `breakpoints`, `state_commit`, `state_recovery`, `restart`, and `checkpoint` modules.

- Extract setup and preflight, step proposal/event fitting, trial solve, error estimation/rejection, accepted-state commit, observation, and checkpoint publication in that order.
- Introduce focused structures for accepted integration state, disposable trial state, scheduling, and retained output. Avoid a replacement context object containing every former local variable.
- Route ordinary acceptance, supported recovery acceptance, event-side transitions, and restart through common invariant checks. Keep their genuinely different numerical policies explicit.
- Remove duplicated history snapshots and bookkeeping only after ownership is established. Keep allocations reusable and snapshot only state that a trial can modify.
- Separate purely structural commits from numerical fixes. Do not change dialect defaults, event clocks, tolerances, or operation ordering as an incidental refactor.

**Acceptance:** unchanged event/acceptance decisions and checkpoint continuation for structural changes; bitwise equality where arithmetic order is preserved. Existing nonlinear, physical-event, locked-grid, restart, abort, and resource regressions pass. Any intentional numerical change has separate before/after evidence.

### C03 Qualify and enable GP transient excess phase

**Dependencies:** C01 and the relevant C02 state/scheduling extractions. **Deliverable:** working public GP PTF transient support.

Primary code: [BJT phase admission and state](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/engine/transient/bjt.rs), its delay/Weil implementations, and [public admission tests](C:/Users/James/Desktop/RSpice/crates/rspice-core/tests/bjt_excess_phase_transient.rs).

- Specify `ExactDelay` and `NgspiceWeil` behavior independently, including dialect defaults, initialization, parameter validity, and the zero-delay limit. Compare the implemented equations with the matching local ngspice/Xyce source paths.
- Integrate forward transport and its derivatives with the correct delayed history and private BJT nodes. Qualify internal resistances, charge coupling, cutoff, polarity, temperature, and multiplicity.
- Complete incoming/outgoing event values, scheduled arrivals, local error control, rejected-step rollback, and history retention. A step reduction must not erase the physical delay or consume an arrival early.
- Persist and restore all delay/Weil history and pending events through public checkpoint APIs; verify exact and scheduled restart, compression, UIC, operating-point startup, and horizon extension.
- Replace temporary refusal tests with public numerical regressions only when the corresponding path is qualified. Retain precise errors for invalid or noncausal parameter combinations.

**Acceptance:** nonzero PTF produces the expected response; refinement improves error against the appropriate independent reference; accepted history survives retries and restart; public APIs pass all supported dialect/model combinations. Remove `ensure_bjt_transient_phase_support` restrictions only for combinations meeting these tests.

### C04 Complete core control execution

**Dependencies:** C00; incorporate C01 contracts where needed. **Deliverable:** one typed analysis execution path available to direct callers and the core control host.

Primary code: [control host](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/engine/control.rs), [analysis commands](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/netlist/ast.rs), and [control execution tests](C:/Users/James/Desktop/RSpice/crates/rspice-core/tests/control_execution.rs).

- Expand `ControlAnalysisResult`, dataset identities, vector/scalar access, units, and retained-value accounting. Reuse established core result types; do not convert every analysis into an AC or transient-shaped placeholder.
- First add DC and nested DC sweeps, AC data tables, TF, noise and noise tables, pole-zero, sensitivity, STB, SP, DISTO, and DC mismatch through existing engine runners.
- Add FOUR and other supported result-dependent operations with explicit input-dataset selection. Report absent or incompatible producer data before starting work.
- Add HB, PSS, PAC, PXF, PNOISE, PSTB, envelope, QPSS, QPAC, QPXF, and QPNOISE with authenticated producer configuration/state. Preserve source sidebands, correlations, and scalar results.
- Define STEP, TEMP, and Monte Carlo as bounded orchestration of child analyses, with deterministic seeds, nested identities, and immutable completed datasets. Do not execute these cards as unrelated flat results.
- Share semantic request parsing and option resolution. Cover `run`, explicit analysis commands, `alter`, settings, reset/reuse, cancellation, failure propagation, and circuit changes that invalidate prepared state.

**Acceptance:** a representative command for every analysis variant has the same core numerical result, identity, units, and error behavior as its direct API route. Mixed scripts, producer/consumer mismatches, nested sweeps, cancellation, and cumulative limits have permanent tests.

### C05 Implement complete IMD measurements

**Dependencies:** C00; C04 for control exposure. **Deliverable:** checked IMD2/IMD3 and intercept-point APIs.

Primary code: [post-processing](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/analysis/post_processing.rs), [existing distortion results](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/analysis/distortion.rs), and [existing Volterra engine](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/engine/distortion.rs).

- Reuse `.DISTO` results for small-signal distortion and the existing Fourier/HB/QPSS infrastructure for measured or large-signal spectra. The missing `ImdResult` constructors are not evidence that the entire distortion solver is absent.
- Define tone and product identities for both fundamentals, second-order sum/difference and harmonics, and both third-order intermodulation sidebands. Handle negative-frequency conjugation and coincident product frequencies explicitly.
- Preserve absolute and relative levels, units, amplitude conventions, observation bandwidth/window, and input/output references. Require an appropriate impedance or power measurement before converting voltage/current amplitudes to dBm.
- Compute IIP2/OIP2 and IIP3/OIP3 only from adequate evidence. Validate the equal-tone weak-nonlinearity assumptions for a single-point extrapolation; support drive sweeps that verify slopes and expose compression. Do not apply equal-tone formulas silently to unequal drives.
- Return explicit unavailable/invalid results for unresolved tones, absent fundamentals/products, insufficient frequency resolution, conflicting products, or invalid extrapolation. Replace mandatory numeric placeholder fields as needed.

**Acceptance:** independent polynomial two-tone cases recover known products and intercepts; exchanged/unequal tones, noncoherent windows, dynamic range extremes, noise, and compression are covered. Tests distinguish Volterra validity from large-signal measurement behavior.

### C06 Complete native semiconductor periodic support

**Dependencies:** C01; C03 for GP phase-dependent cases. **Deliverable:** qualified native semiconductor contributions across applicable HB, QPSS, small-signal, PSS, and envelope paths.

- Implement B3SOI DD/FD/PD, EKV 2.6, EKV3, and VDMOS in separate family batches. Preserve internal electrical and thermal coordinates, body effects, charges, parasitics, model selectors, instance scaling, and temperature behavior.
- Reuse authoritative device equations for static and dynamic residuals and all required derivatives. Complete periodic small-signal conversion and accepted charge/thermal histories alongside the nonlinear contribution.
- Extend diode and JFET periodic equations beyond their current reduced subsets. Audit classic MOS restrictions against actual model levels/parameters before deciding whether each requires code or a more precise capability predicate.
- Carry GP thermal and qualified excess-phase states into applicable periodic and envelope formulations. Preserve the rational/nonrational distinction from C01.
- Test each implemented family first in DC-equilibrium reduction, then driven periodic operation, perturbation/conversion, shooting continuation, and envelope evolution. Hand completed noise interfaces to C08.

**Acceptance:** charge/current conservation, qualified Jacobians, high-precision or simulator comparisons, tone/grid/time refinement, and public analysis execution for each family. No device may disappear from a matrix while its analysis reports success.

### C07 Complete dynamic, hysteretic, and distributed element support

**Dependencies:** C01 and relevant C02 state boundaries. **Deliverable:** correct device equations and retained state for the non-semiconductor gaps in the capability table.

- Add thermal-resistor and memristor state equations, state bounds, derivatives, and continuation. Handle state-boundary events and distinguish a genuine equilibrium from a frozen state.
- Complete voltage/current/expression-controlled switches, including control-branch spectra, hysteresis memory, threshold events, and sensitivities of moving event times. Diagnose grazing, ambiguous, and nonunique transitions.
- Add lossy scalar and coupled-line convolution histories to shooting and envelope paths; finish applicable frequency-domain stamps. Preserve causal history windows, pre-origin conditions, delayed events, and tail/error controls.
- Complete multi-winding and nonlinear magnetic-core equations and histories. Derive independent shooting coordinates for singular mutual-inductance constraints rather than perturbing them into a different circuit.
- Resolve applicable capacitor and behavioral live-frequency/hidden-state restrictions with explicit equations or the appropriate operator formulation. Arbitrary nonperiodic forcing must not be silently projected into a periodic model.

**Acceptance:** analytic RC/RL/transformer limits, line delay/attenuation and reflection cases, closed hysteresis cycles, stateful continuation, event refinement, and conservation tests. Public PSS/envelope results agree with independently refined transient results within declared tolerances.

### C08 Complete periodic noise and correlation support

**Dependencies:** completed residual/state slices from C06, C07, and C09. **Deliverable:** valid periodic noise for each implemented noisy device family.

- Evaluate bias-dependent noise along the periodic orbit, including thermal/shot sources, flicker modulation for the supported exponent range, and cross-correlations. Do not replace cyclostationary behavior with DC-bias noise.
- Preserve source identity, physical units, normalization, conjugate symmetry, correlation structure, and positive semidefiniteness through sideband folding and output projection.
- Share source assembly and conversion contracts across periodic and quasiperiodic noise routes; retain the distinct formulations needed by sampled and autonomous analyses.
- Handle singular/zero-frequency behavior and finite integration bands explicitly. Account for truncation and sideband convergence without introducing a fabricated noise floor.

**Acceptance:** constant-bias reduction to ordinary noise; independent modulated-noise cases; known correlated-source cancellation/addition; sideband refinement; and integrated-power checks for semiconductor, thermal, memristive, and external-model sources as applicable.

### C09 Wire external model capabilities into core analyses

**Dependencies:** C01 and evaluator/state contracts supplied by the existing Verilog-A/runtime workstreams. **Deliverable:** actual core execution for eligible runtime, generated, and XSPICE models.

- Bind capability metadata to concrete evaluator, derivative, noise, and accepted-state interfaces. Replace family-wide optimism with instance/operator-aware admission where necessary.
- Implement core adapters for periodic residuals, small-signal stamps, noise, shooting state, and envelope continuation. Exercise real models, not only stub capability providers.
- Make `ddt`, integrators, filters, sampled operators, delays, and conditional state ownership explicit. Export finite states where valid; use delay/operator-aware paths where a finite descriptor is not exact.
- Require authenticated model/ABI/state identity, bounded allocation, cancellation, and atomic restoration. Ensure failed analyses and separate engine instances do not leak registration or mutable model state into later runs.
- For XSPICE, classify and implement supported code-model classes and their event/accepted-step histories rather than treating all models as one stateless family.

**Acceptance:** representative real runtime and generated models match their qualified native/analytic counterparts through public core analyses and continuation. Metadata-only support declarations cannot pass the qualification suite.

Compiler/language implementation remains owned by the existing [Verilog-AMS plan](C:/Users/James/Desktop/RSpice/VERILOG_AMS_IMPLEMENTATION_PLAN.md). Missing dependency interfaces keep the affected core package open; they are not replaced with dummy evaluators.

### C10 Complete mixed accepted state and equilibrium analyses

**Dependencies:** C01, relevant C02 boundaries, and the core/runtime state interfaces from C09. **Deliverable:** a complete core mixed-state contract and valid settled-state analyses.

- Include process resumption state, event queues, drivers, resolved digital/real values, analog/digital bridge histories, timers, random state, and analog device/integration state in accepted-state capture and restore.
- Make trial-time cross-domain changes and side effects rejectable until analog acceptance. Preserve deterministic ordering for simultaneous events and prevent duplicate effects after retries or resume.
- Implement coupled equilibrium and DC behavior beyond the state left by initial blocks. Define convergence, discrete fixed points, cycles, ambiguity, and initialization diagnostics.
- Define eligibility for AC, noise, transfer, and sensitivity around a settled discrete state. Distinguish differentiable perturbations from threshold-crossing behavior that requires a hybrid formulation.

**Acceptance:** settled mixed loops, DAC/ADC feedback, initialization-order cases, same-time events, failed trials, cancellation, and persisted continuation reproduce their qualified reference behavior. Unsupported nonunique equilibria produce specific diagnostics.

This is the core portion of MS03, MS08, and MS10 in the [mixed simulation plan](C:/Users/James/Desktop/RSpice/MIXED_SIMULATION_IMPLEMENTATION_PLAN.md), not a second scheduler implementation.

### C11 Implement event-aware mixed periodic and RF analyses

**Dependencies:** C10 and the device/operator providers required by each test circuit. **Deliverable:** applicable mixed PSS, periodic linear analyses, noise, and envelope continuation.

- Define eligible driven, autonomous, subharmonic, multiperiod, and quasiperiodic designs. Require closure of discrete state, pending events, clock phase, and delay/conversion history as well as analog values.
- Extend shooting and continuation with event-time sensitivities and state-jump derivatives. Do not freeze a switching schedule whose times move under the perturbation.
- Implement the applicable harmonic/hybrid residual, PAC/PXF/PSTB, noise folding, and envelope equations. Retain sideband identities and the periodic producer's complete accepted state.
- Diagnose nonperiodic behavior, changing event topology, grazing events, and ambiguous periods. Use a formulation appropriate to each class rather than routing every mixed design through a smooth analog HB solver.

**Acceptance:** switched-capacitor/filter, divider-controlled analog loop, sampled feedback, and suitable oscillator cases pass period closure, conversion, phase, noise, and transient-continuation comparisons. Completion covers the core portion of MS09 in the existing mixed simulation plan.

### C12 Close core execution and result integration

**Dependencies:** follows each capability package; final completion after C03 through C11. **Deliverable:** complete parser-to-result and analysis-to-analysis behavior inside core.

- Verify hierarchy, model binding, aliases, multiplicity, options, temperature, and statistical identity through every public analysis path. Reuse elaboration and operating-point preparation where the physical state is genuinely shared.
- Ensure direct calls, parsed commands, and control sessions share validation, capability checks, cancellation, resource limits, errors, and typed results. Validate all outputs before publishing them.
- Preserve dependencies between steady-state producers and small-signal/noise/envelope consumers. Reject stale model/configuration/state identities and incompatible source or sideband requests.
- Exercise ordinary, checkpointed, resumed, compressed, and selected-output transient paths against the same physical observations, including discontinuities and current impulses.
- Update public documentation and migration guidance for changed result contracts, including the checked post-processing API and explicit measurement availability.

**Acceptance:** the C00 route matrix has end-to-end tests for each required public core path. No parsed option is silently dropped, no analysis result is lost through control execution, and no failed solve produces a successful result.

### C13 Finish core organization and performance work

**Dependencies:** C02 and stable implementations from the relevant capability packages. **Deliverable:** simpler ownership and measured performance without changing qualified physics.

- Audit oversized builder, netlist, measurement, checkpoint, and analysis modules for actual responsibility boundaries. Extract model-family construction, schema handling, measurement resolution, and orchestration where cohesion improves; avoid a line-count-driven rewrite.
- Consolidate remaining duplicated sweep generation, phase/spectral processing, probe resolution, result sizing, and analysis preparation. Keep dialect differences and mathematically different measurement definitions explicit.
- Profile elaboration, bias solve, sparse assembly/factorization, nonlinear iterations, history retention, periodic transforms, and result capture separately. Reduce measured allocation/copying costs and reuse stable topology/factorizations where valid.
- Preserve sparse scaling and bounded memory; test deterministic output ordering and independent engine state under concurrency. Avoid global mutable model caches or premature generic abstractions in hot loops.
- Remove obsolete compatibility paths, unused helpers, and duplicated caches only after all callers and persisted compatibility contracts are accounted for.

**Acceptance:** documented ownership boundaries, no duplicate implementation of the same numerical contract, and repeatable performance/memory comparisons on C00 circuits. Every optimization retains the numerical and cancellation gates.

### C14 Qualify and close the core plan

**Dependencies:** all prior packages. **Deliverable:** an auditable core completion record on the exact committed source.

- Run the full core unit, integration, documentation, and relevant feature-configuration suites in development and optimized profiles. Run strict static checks on the core targets covered by the project policy.
- Execute the applicable ngspice/Xyce and analytical corpora, including currently ignored live-oracle tests when their reference binaries are supplied. Separate diagnostic probes from required correctness tests and give each omission a recorded disposition.
- Add fuzz/property tests at the actual changed boundaries: netlist/control inputs, model parameter combinations, numerical extreme values, capability dispatch, checkpoint decoding/restoration, and result projections.
- Run public-API cancellation and resource-failure tests at elaboration, nonlinear iteration, frequency/period sweeps, event processing, result construction, and restore. Verify no partial committed state survives a failed operation.
- Check the crate's intended Cargo feature/target combinations, including the core WebAssembly build, without turning this into application UI or deployment qualification.
- Publish the exact revision, fixtures, reference versions, numerical tolerances, test outcomes, and performance evidence. Generate the final support matrix from the implemented contracts and tests.

**Acceptance:** every required ledger row is implemented and independently qualified where an oracle exists; remaining mathematical exclusions are justified and tested; no temporary implementation guard is counted as completion; no unresolved core correctness failure remains. A green default test suite alone cannot close this package.

## Implementation order

| Stage | Packages | Required result before advancing |
|---|---|---|
| 1 | C00, C01 | Reproducible baseline, complete ledger, and agreed equation/state contracts |
| 2 | C02, C03 | Explicit transient boundaries and public GP excess-phase support |
| 3 | C04, C05 | Complete control dispatch and checked IMD measurements using existing solvers |
| 4 | C06, C07, C09 | Qualified device families and external-model adapters delivered in small vertical slices |
| 5 | C08, C10, C11 | Qualified periodic noise and mixed equilibrium/state/periodic execution |
| 6 | C12, C13, C14 | All core routes connected, duplication/performance work verified, completion evidence recorded |

C04 and C05 can proceed after C00 without waiting for every transient or device package. C08 follows each completed noisy-device slice rather than waiting for every model. C12 accompanies every package; stage 6 is its final coverage review.

Within each model family, deliver residual and derivative tests, transient/state ownership, periodic and small-signal integration, noise, and public-route qualification as reviewable commits. Do not combine a mechanical refactor, new equations, changed tolerances, and baseline updates into one commit.

## Reference evidence and collaboration boundaries

- Local ngspice source: [ngspice 46](C:/Users/James/Desktop/ngspice-46-release/ngspice-46). Local Xyce source: [Xyce 7.10](C:/Users/James/Desktop/Xyce-7.10.0/Xyce-7.10). Record the exact source/build identity used for each comparison. Compare like model/dialect semantics; one simulator is not an oracle for a different model formulation.
- Use the existing [core integration tests](C:/Users/James/Desktop/RSpice/crates/rspice-core/tests), checked-in conformance fixtures, and permanent analytical cases. Keep temporary development evidence separate from release gates.
- Coordinate source-derivative changes with [project audit IP05](C:/Users/James/Desktop/RSpice/PROJECT_AUDIT_IMPLEMENTATION_PLAN.md). Coordinate compiler/runtime metadata and mixed-state contracts with the existing Verilog-AMS and mixed simulation plans. Those dependencies do not broaden this plan into the product-level tasks in those documents.
- Preserve current unrelated edits. Validate and commit only package-owned changes. If remote `main` advances, integrate its committed changes without including unrelated local files, then rerun checks affected by the integration.

## Initial execution batch

Start with C00 and the smallest C01/C02 state-boundary extraction: create the requirements ledger, capture a clean baseline, document accepted versus trial ownership, and extract physical-event step fitting with its existing regressions. Establish state invariants before opening GP PTF admission. Prepare the C04 result/dispatch design and C05 measurement contract alongside that work; neither requires rewriting the remaining device families first.

Update this plan after each delivered package with implementation commits, permanent tests, evidence, and unresolved rows. All packages remain open.

## Execution record

- C00 has an executable requirements ledger under `crates/rspice-core/tests/testdata/qualification/`: all 29 analysis command variants, representative public routes, control-host disposition, all 38 families across six periodic contracts, the full declared restrictions, package ownership, and remaining qualification tasks. Tests check the parsed command inventory, actual control-host outcomes, and declaration changes. These checks do not certify missing numerical implementations.
- The clean committed baseline at `f24e2ea28` passed 4,843 unit tests. The two live ngspice-46 BSIM oracle tests also passed all 22,586 comparisons. Toolchain, reference-binary identity, commands, observed errors, and unresolved evidence are recorded in `core-baseline-windows-20261005.json`.
- C00 remains open for complete per-instance predicate fixtures, auxiliary public-route inventory, broader configurations, and reproducible numerical/performance/resource measurements.
- C00's initial ledger and baseline evidence were committed as `2c4ce4d6d` and pushed to `main`.
- C01/C02 now separate disposable physical-event proposals and immutable step bounds from accepted state. Canonical endpoint selection and physical-event approach fitting live in `engine/transient/step_proposal.rs`; the driver retains event ownership, trial acceptance, and state mutation. Arithmetic, dialect floors, and event clocks are preserved.
- This extraction passed 542 transient unit tests (one existing diagnostic ignored), 65 integration tests covering checkpoint encoding/continuation, source events, Xyce restart planning, failure atomicity, and the existing qualification baseline, plus strict library Clippy. The remaining transient phases and shared equation/state contracts are still open.
