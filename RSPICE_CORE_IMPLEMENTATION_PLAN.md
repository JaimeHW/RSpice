# RSpice core remaining implementation plan

Status: implementation in progress; all 15 packages remain open. Remaining work reviewed October 6, 2026 against revision `2bb484dbd317ac6988b702ac15b98596463f49ea`, including startup repair `4ec035c19`, continuous-event rate repair `87ff9cbf9`, physical-event approach repair `0d6ceb788`, smooth prescribed behavioral event providers `03f270c65`, native voltage-controlled event providers `5619108f7`, native CCCS events/startup/current observations `8fb9d66ad`, shared descriptor closure `79e24649f`, the event voltage-seed sign repair `e584ecaef`, the test-gated distributional transition kernel `90247985e`, typed current impulse derivative observations `d3d15ca70`, analytic FFT taper derivatives `bcaefd8a3`, current integral boundary state `ba2f8cfd4`, DC control execution `1dd0e1146`, input-noise normalization `fa7fc79c0`, transient identity preparation `91bb08b05`, and ordinary NOISE control execution `ad6230cde`. Originally prepared October 5 against `20fd0d6350596c0cfc814ee729f4e2286dd30d2b`; initial execution baseline is `f24e2ea28f2322e78086c14837ea53bc2d4c18c3`.

Remaining-work reconciliation includes fallible BJT history initialization and reseeding at `6b32f5bb330993d7f6b3e150a364d42d4334a17b`, in addition to the native GP transport-memory implementation at `4006fa60a1fa5e0e0aa47433578d59e4b6f01468`. C03b record-ceiling diagnostics and their adapter contracts are delivered in `d01e555b8` and `948fd90ff`. Recorded tests qualify their stated paths and configurations; they do not close whole packages.

Complete the unfinished work identified by the core audit: missing analysis and model support, incomplete control execution, IMD measurements, architecture and duplication problems, and core qualification. This plan covers `rspice-core`, its public Rust interfaces, and the dependency contracts needed by its solvers. Application UI, packaging, deployment, and product-level platform certification are outside this plan.

The previous repair batch fixed the reproduced transient, transfer-function, stability, and measurement defects. It did not complete the capability gaps below. An unsupported-capability error protects callers while implementation proceeds; adding that error, hiding an option, or changing documentation does not complete a missing implementation.

## Current planning baseline

- The executable ledger has 15 packages: C00–C04 are in progress and C05–C14 are open. The execution record below identifies delivered portions; none establishes completion of its whole package.
- Of 29 analysis-command variants, the core control host currently executes OP, DC (including nested sweeps), AC, NOISE, AC DATA, NOISE DATA, TF, and transient. The remaining 21 need control execution or producer/orchestration semantics. Their direct solver routes already exist; this is not a claim that 21 solvers are absent. Table and TF documents and CLI/WASM publication are implemented, including bounded projection and cancellation. Broader C04 binding, configuration and performance qualification remains open.
- The periodic declaration snapshot contains 228 rows: 38 families across six contracts. There are 77 complete, 27 restricted, 64 absent, and 60 inapplicable declarations. The 91 restricted/absent rows identify work to resolve against actual instance predicates. They are not 91 independent missing solvers, and the 77 complete declarations still require numerical qualification.
- `ImdResult` remains a measurement container without checked estimators. The existing `.DISTO` product contract includes `2*f1-f2` but not `2*f2-f1`; preserve the existing solver while completing the product and measurement contracts.
- Bias-dependent GP forward/reverse charge oracles are delivered in `da18bfa88`. The following private-node probes exposed a small-charge event-row defect, repaired in `50523d8fb`, and two further convergence failures repaired by the subsequent work below. Startup repair `4ec035c19` makes the constant-bias regression active at unchanged accuracy limits. Continuous-event repair `87ff9cbf9` and event-approach repair `0d6ceb788` make the driven private-node regression active too. Both named failures are repaired; the remaining feedback/Weil, parameter, continuation, resource and configuration evidence still leaves C03c open.
- Smooth prescribed behavioral voltage/current sources now have physical-event equations in `03f270c65`. The original sinusoidal-source reproducer passes independent numerical, startup-impulse and checkpoint tests. Native VCVS/VCCS event equations, current observations and independent feedback/step/restart oracles are delivered in `5619108f7`. Native CCCS weighted charge/current equations and finite/impulsive observations are delivered in `8fb9d66ad`. CCVS, circuit-dependent/switched/stateful behavioral sources and other family/instance providers remain C03d work; this bounded slice does not establish general GP circuit support.

Use `core-requirements-v1.json` and `core-periodic-requirements-v1.json` under `crates/rspice-core/tests/testdata/qualification/` as the executable inventory. Record newly reproduced defects there with a package, regression, and acceptance gate. The next-batch tables below provide the delivery order; the package definitions remain the full completion contract.

## Priorities for the remaining work

| Priority | Deliverable | Completion gate |
|---|---|---|
| 1 — incomplete event integration | Implement remaining physical-event equations for devices accompanying exact-delay GP (C03d), continuing with CCVS and remaining behavioral operators | Preserve the delivered prescribed-source analytical event/continuation tests; each additional family has real equations and complete state before admission |
| 2 — remaining numerical qualification | Complete private-node/feedback and parameter cases (C03c.2), preserving the two repaired regressions | Independent Weil feedback evidence, remaining charge/parameter boundaries and rejection/restart/OP/UIC cases; existing accuracy, conservation and refinement gates remain active |
| 3 — existing solver accessibility and measurements | Complete the 21 remaining control-host dispositions (C04) and checked IMD/intercept measurements (C05) | Direct/control equivalence, correct result units and identities, producer dependencies, independent two-tone evidence, and bounded orchestration |
| 4 — model and analysis completeness | Deliver native, dynamic, and external providers (C06/C07/C09), then each provider's noise and mixed-analysis contracts (C08/C10/C11) | A per-family matrix of public numerical and continuation tests; declarations alone cannot close a row |
| Throughout — architecture and integration | Finish the ledger, shared equation/state ownership, transient decomposition, and public-route integration (C00/C01/C02/C12) | One authoritative physical implementation per contract, atomic acceptance/restore, and matching parser/direct/control behavior |
| Final — measured performance and core qualification | Finish C03 resource/configuration gates and C13/C14 | Reproducible performance and memory limits, cancellation during expensive work, all required regressions active, and qualification on the exact committed source |

These are implementation priorities, not a requirement to stall independent work. C04/C05 can advance while a specific C03 numerical issue is investigated. Deliver provider support in small complete slices instead of waiting to implement every family before testing its public routes.

Architecture decision: retain the existing engines and authoritative device equations, then improve the numerical/state boundaries incrementally. The evidence supports targeted ownership and duplication repairs; it does not establish that the current architecture is optimal or justify a wholesale rewrite. Use C01 conformance tests and C13 measurements to decide each extraction or optimization.

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

The repair batch recorded 4,843 passing core unit tests, three ignored tests, 126 passing selected integration tests, strict library Clippy, and successful core/dependent build checks. These are historical validation results, not a certificate for every model or analysis. C00 has since established the committed baseline recorded below; preserve it and complete the outstanding coverage rather than repeating its delivered setup.

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
| GP BJT excess phase | Finish remaining transient qualification; implement applicable periodic continuation. Public phase execution, restart, fixed-state initialization, typed record-ceiling diagnostics, and aggregate transport-record budgets are delivered | C03, C06 |
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
- Complete fallible allocation and aggregate accounting for retained GP delay records and checkpoint copies. The shared delay buffer's per-site sample ceiling and time-window pruning do not establish an analysis-wide memory bound. Qualify resource failure during initialization, accepted-history growth, capture, and restoration without partially published state.
- Reserve fixed per-device lanes and any owned UIC seed storage in `initialize_bjt_history` fallibly through typed errors. Preserve arithmetic and accepted-state initialization across transient and periodic callers. Keep this fixed-state responsibility distinct from the delivered exact-delay record-byte quota.
- Preserve a typed per-site record-ceiling refusal through delay validation, event preparation, and public execution. Include requested/allowed counts and the affected instance when available; do not classify failures by matching message text. Malformed checkpoint data remains an input/state-validation error.
- Replace temporary refusal tests with public numerical regressions only when the corresponding path is qualified. Retain precise errors for invalid or noncausal parameter combinations.
- Complete C03d's physical-event device integration: inventory the families excluded by `PreparedEventCircuit::admitted_family`, then supply the appropriate residual, charge, sided time derivatives, impulse constraints, and accepted state. Preserve the delivered smooth behavioral-source reproducer and complete the remaining expression and controlled-source providers. Reuse authoritative provider equations under C01; do not remove the guard before those contributions exist. Coordinate stateful and external providers with C07/C09 while keeping the public exact-delay integration requirement open in C03.

**Acceptance:** nonzero PTF produces the expected response; refinement improves error against the appropriate independent reference; accepted history survives retries and restart; public APIs pass all supported dialect/model combinations. Remove `ensure_bjt_transient_phase_support` restrictions only for combinations meeting these tests.

### C04 Complete core control execution

**Dependencies:** C00; incorporate C01 contracts where needed. **Deliverable:** one typed analysis execution path available to direct callers and the core control host.

Primary code: [control host](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/engine/control.rs), [analysis commands](C:/Users/James/Desktop/RSpice/crates/rspice-core/src/netlist/ast.rs), and [control execution tests](C:/Users/James/Desktop/RSpice/crates/rspice-core/tests/control_execution.rs).

- Expand `ControlAnalysisResult`, dataset identities, vector/scalar access, units, and retained-value accounting. Reuse established core result types; do not convert every analysis into an AC or transient-shaped placeholder.
- DC and nested DC sweeps are delivered in `1dd0e1146`; ordinary NOISE is delivered in `ad6230cde`. Core AC/noise table dispatch and presentation now retain physical row coordinates; finish typed document/adapter publication, then add TF, pole-zero, sensitivity, STB, SP, DISTO, and DC mismatch through existing engine runners.
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
- Resolve the recorded Windows compiler and WASM-runner reproducibility incidents. The provisional-parameter evidence also records a Rust 1.94 compiler access violation while rebuilding core at `f8d45a153`; the unchanged retry passed, but neither its cause nor the broader toolchain gate is closed.

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

## Original execution batch

The original starting batch was C00 and the smallest C01/C02 state-boundary extraction: create the requirements ledger, capture a clean baseline, document accepted versus trial ownership, and extract physical-event step fitting with its existing regressions. The execution record below identifies what has been delivered. Continue from the next implementation batches rather than recreating this baseline or reopening delivered GP admission work.

Update this plan after each delivered package with implementation commits, permanent tests, evidence, and unresolved rows. All packages remain open.

## Execution record

- C00 has an executable requirements ledger under `crates/rspice-core/tests/testdata/qualification/`: all 29 analysis command variants, representative public routes, control-host disposition, all 38 families across six periodic contracts, the full declared restrictions, package ownership, and remaining qualification tasks. Tests check the parsed command inventory, actual control-host outcomes, and declaration changes. These checks do not certify missing numerical implementations.
- The clean committed baseline at `f24e2ea28` passed 4,843 unit tests. The two live ngspice-46 BSIM oracle tests also passed all 22,586 comparisons. Toolchain, reference-binary identity, commands, observed errors, and unresolved evidence are recorded in `core-baseline-windows-20261005.json`.
- C00 remains open for complete per-instance predicate fixtures, auxiliary public-route inventory, broader configurations, and reproducible numerical/performance/resource measurements.
- C00's initial ledger and baseline evidence were committed as `2c4ce4d6d` and pushed to `main`.
- C01/C02 now separate disposable physical-event proposals and immutable step bounds from accepted state. Canonical endpoint selection and physical-event approach fitting live in `engine/transient/step_proposal.rs`; the driver retains event ownership, trial acceptance, and state mutation. Arithmetic, dialect floors, and event clocks are preserved.
- This extraction passed 542 transient unit tests (one existing diagnostic ignored), 65 integration tests covering checkpoint encoding/continuation, source events, Xyce restart planning, failure atomicity, and the existing qualification baseline, plus strict library Clippy. The remaining transient phases and shared equation/state contracts are still open.
- C03 qualification found and repaired two numerical defects in `85f7f0b8e`, pushed to `main`: exact-history runs could leave a rounding-sized final interval that failed to converge, and hybrid Gear2 intervals could leave a stale OneStep static residual that caused current ringing when trapezoidal integration resumed. Stop-step fitting preserves existing step bounds and event clocks; both acceptance paths refresh a retained static residual. Checkpoint configuration identity 96 prevents continuation with the former numerical policy.
- C03 delivery `263f992d7`, pushed to `main`, replaces the public nonzero-PTF refusal with finite, nonnegative delay validation. Its independent physical-event/oracle cases now use public transient and resume APIs. Coverage includes both phase operators, all three dialects, OP/UIC startup, NPN/PNP, private resistances, temperature, area/multiplicity, interpolation refinement, cancellation, packed/unpacked checkpoints, compression, and continuation. Periodic GP capability remains owned by C06.
- Development validation of that combined C03 tree passed 4,851 unit tests (three existing ignored tests), 85 selected integration tests, strict library/changed-test Clippy, and all 10 GP integration tests without default Cargo features. The exact source, commands, fixture coverage, log hashes, and remaining evidence are recorded in `core-gp-transient-windows-20261005.json`. These are executed correctness results, not final C14 qualification or solver benchmarks.
- C03 storage work in `a9c9e2d63` and `0a3e1eb3e`, both pushed to `main`, makes native delay-buffer construction, record growth, copying, and restoration fallible. Ordinary steps, physical startup, and periodic boundaries reserve transport storage before accepted-state mutation. GP capture/restore copies propagate allocation failures; restart normalization consumes an owned checkpoint instead of copying its complete state again.
- That storage delivery passed 136 runtime tests, including four allocation-failure regressions; 544 transient and 201 HB/PSS unit tests; 50 public phase/checkpoint integration tests; strict runtime/core Clippy; and the core build with `veriloga`. This does not close aggregate working-memory limits or qualification of every model/provider's allocation behavior.

- C03 checkpoint accounting in `0a41d35a6`, pushed to `main`, charges retained exact-delay samples, event sides, and derivative-order records to the result-value budget. Its public regression refuses an oversized retained checkpoint while allowing the same waveform without checkpoint retention. That committed revision passed 245 checkpoint/driver/sensitivity unit tests, 59 selected integration tests, and strict core Clippy. Scoped storage and accounting evidence, including subsequent concurrent-main integration, is recorded in `core-gp-storage-windows-20261005.json`; aggregate live-history and peak capture-memory limits were still open at that revision.

- C03 allocation diagnostics in `3fb1e860d` preserve `SimulationError::Allocation`, its allocator source, and the stable `allocation_failed` code through native GP history growth and checkpoint capture/restore. Policy refusals and allocator refusals share the resource category but keep distinct metadata. Later fault tests explicitly inject into final capture after startup copies and verify unchanged input checkpoints and successful subsequent runs. Fixed BJT initialization and per-site record-ceiling diagnostics were still open at that revision; initialization is delivered below.
- Shared storage sizing in `5dffb50d3` makes delay-buffer growth requests predictable, includes spare capacity and event records, and exposes compact-copy sizing. Runtime validation passed 194 tests. This is the dependency contract used by core accounting, not qualification of runtime VM acceptance.
- C03 transport accounting in `4006fa60a`, pushed to `main`, bounds aggregate native GP exact-delay backing records with `ResourceLimits::max_transport_history_bytes` (256 MiB default). Initialization, acceptance, startup copies, scheduled/final checkpoint capture, and restoration preflight engine-owned storage. Retained output checkpoints stay charged during later growth/capture. Caller-owned input checkpoints, fixed device lanes, other providers, and solver workspaces are outside this specific quota.
- The memory implementation passed 758 selected core unit tests (one existing diagnostic ignored), 15 subsequent focused tests including the added startup-refusal regression, 75 integration tests, and strict core Clippy. The unit sets overlap and must not be added as a unique-test total. Tests cover policy refusal before copy allocation, aggregate multi-device growth, compressed output, OP/UIC, scheduled copies, stricter resume budgets, and preservation of accepted state. Configuration-specific qualification and exact source/log provenance are recorded in `core-gp-memory-windows-20261005.json`.
- On the committed `4006fa60a` implementation, the 10 public GP phase tests and four transport-resource tests pass both without default features and in the optimized default-feature profile. Three ledger checks also pass for ownership, parsed analysis/control dispositions, and periodic declarations. These selected runs do not replace the complete feature/target and performance/cancellation gates.
- Integration repair `e0bb0477f` shares checked materialization-coordinate access and avoids cloning a coordinate for borrowed netlist materialization. Its 11 unit tests are included in the focused 15-test run. The overlapping remote fix was reconciled in `c6b35c657` before the final default integration checks.

- C03a implementation `6b32f5bb3` makes all 22 fixed BJT history lanes and owned UIC solution copies fallible. Restart prepares replacement history before changing accepted capacitor/inductor/BJT state or taking phase owners. Shooting initialization reserves before mutation; periodic reconstruction reserves all three history generations before installing BJT bias. Resume uses restored BJT history without constructing a discarded replacement. Device equations, event clocks, tolerances, and checkpoint encoding are unchanged.
- Five new initialization and atomicity regressions, together with the existing copy-failure cases, pass in a focused seven-test run. The broader selected unit run passes 759 tests with one existing diagnostic ignored; the focused tests overlap that run. Final successful outcomes across 11 default integration targets total 163 tests. Another 31 public phase/resource/UIC tests pass in each of the no-default-feature and optimized profiles on committed `6b32f5bb3`; strict library and changed-fixture Clippy also pass. Exact commands, revisions, log hashes, and the resolved failed attempt are recorded in `core-bjt-initialization-windows-20261005.json`.
- Separate repair `fb68d3d24` replaces an obsolete classic-MOS envelope refusal assertion with positive state creation/continuation coverage. Native MOS support was already delivered in `c5beb38c2`. The corrected target and existing independent MOS envelope regression pass 12 tests; those are included in the integration total above. No admission restriction or numerical tolerance was relaxed. Unsupported GP periodic phase remains explicitly refused and belongs to C06.
- C03a closes this fixed-history allocation boundary only. Other model/provider lanes, trial and snapshot caches, periodic model clones, and solver workspaces remain C01/C09/C14 work. Fixed device state remains outside the transport-record byte quota. All C00–C14 packages remain open.

- C03b runtime/core contract `d01e555b8` introduces `DelayAcceptanceError` and preserves per-site record-limit counts as `SimulationError::DeviceResourceLimit` with the elaborated instance and `ResourceKind::TransportHistoryRecords`. Native ordinary, startup, and physical-event preparation propagate the same classification. The retention check counts the complete event before refusing it; known-order records no longer lose their contribution behind an earlier ordinary-sample refusal. The existing ceiling, byte quota, numerical equations, tolerances, event clocks, and checkpoint encoding are unchanged.
- Runtime validation passes 194 tests. The broad core selection passed 564 tests with one diagnostic ignored and one old message assertion failing; its typed replacement and all three new core regressions pass the final four-test run. Thus 565 selected cases have successful final outcomes, without adding overlapping focused runs. All 62 selected public integration tests pass, as do 17 adapter tests, dependent compilation, and strict library Clippy for core/runtime and simulation/contract. Exact outcomes, the resolved failed attempt, and source provenance are in `core-delay-errors-windows-20261005.json`.
- Adapter repair `948fd90ff` also resolves the prior missing `Allocation` match in `rspice-simulation`. Service/worker conversions preserve device identity, requested/allowed counts, allocation object/detail, and stable resource codes. Optimization treats both as fatal after its first failing evaluation. This is a required core error-contract integration repair; C12's broader route coverage remains open.
- Merge `f25a3646c` integrates concurrent quasiperiodic producer/result-document and RAW-table repairs. Its 201 selected core execution, requirements, and resource-regression tests pass, as does compilation of simulation, CLI, and engine-adapter consumers. The selection overlaps the preceding checks; merge provenance is recorded separately in the same evidence file.
- C03b closes the native GP diagnostic requirement. VM acceptance, external-model error contracts, other state/cache allocations, and the complete target and performance/cancellation matrix remain with C09, C01, C12, and C14 as specified above. C03c continues below; no C00–C14 package is closed.
- C03c initial delivery `d4c2a1412` bounds cancellation requested after accepted GP work across both phase operators, all three dialects, and OP/UIC startup. Cancellation before subsequent acceptance preserves engine reuse; cancelled continuation preserves the input and final resumed checkpoint. `bf65aa27a` adds an opt-in GP command to the existing benchmark tool, with complete workload/provenance records, immutable reports and explicit timing/storage gates.
- At merged revision `e4a4dd93f`, 72 core integration tests pass, and the 16 selected GP integration tests pass with/without default features in both test and release profiles. The benchmark's 24 tests and affected strict Clippy checks pass. The core's `wasm32-unknown-unknown` configuration with `wasm,veriloga,veriloga-wasm-jit` compiles; that is not executed target qualification.
- Two clean release benchmark launches each exercise eight cases and five measured full runs/cancellations per case. They reproduce waveform bits, work counts and charged peak transport storage, and pass the predeclared workload gates. The quota boundary measures live transport-record capacity, temporary copies and retained checkpoints; it excludes total heap/process memory. Cancellation is requested after an accepted point beyond one physical delay; it does not measure a request arriving inside factorization. Commands, source/artifact hashes, portable counts and limitations are recorded in `core-gp-measurements-windows-20261005.json`; raw machine timings remain local artifacts.
- C03c nonlinear qualification `722363fd0` adds independent saturation, reverse-active and high-injection transport/charge oracles for both polarities, all three dialects, and both phase laws. Exact delay is evaluated analytically; Weil uses a separate recurrence checked against the local ngspice-46 equations. The 54 public runs per configuration include exact-delay refinement, instantaneous reverse transport and terminal-current conservation. These prescribed-terminal cases do not complete nonlinear feedback/private-node or bias-dependent transit-time coverage.
- Those oracles exposed a rounding-sized interval before an exact phase arrival, which amplified charge subtraction error into terminal-current ringing. `722363fd0` balances the remaining gap before floor fitting, preserving event clocks, both step ceilings and established floor policies; checkpoint configuration identity advances to v97. At that committed revision, 4,871 library tests pass with three existing ignores, strict Clippy passes, the new nonlinear cases pass in the native minimal test configuration and default release, and eight Verilog-A timestep tests pass. The 83 selected public integration cases, including the default-configuration nonlinear tests, have successful final development outcomes. `3d5bf34e7` corrects the control inventory's producer requirement, and `3021ee04f` reconciles four existing quasiperiodic capability declarations without changing numerical/resource gates. Exact commands, source hashes, errors and scope are in `core-gp-regimes-windows-20261005.json`.
- C03c executed WebAssembly delivery `12ba3605b` registers the same 18 public GP integration tests with the workspace's WASM harness; native and WASM executions share all cases, oracles and tolerances. All 18 pass in Node and a headless Chrome window with minimal features, and in an optimized Chrome dedicated worker with `wasm,veriloga-wasm-jit`. Native registration and strict WASM Clippy also pass. The 32-bit executions cover exact/Weil accuracy, nonlinear regimes, temperature/scaling, restart/compression, callback cancellation and transport-record budgets; enabling the Verilog-A feature does not establish external-model/JIT evaluation.
- CI commit `b0878b1d0` adds the optimized worker selection to the existing Firefox job. Workflow lint and 64 CI configuration checks pass. A clean repeat at that committed revision passes all 18 tests in Chrome; hosted Firefox was still queued when observed. Source/runtime versions, commands, logs, eight WASM artifact hashes and scope are recorded in `core-gp-wasm-windows-20261006.json`. Remaining browser/runtime and delivered-profile coverage stays open in C14, alongside C03's remaining numerical/resource work.
- C03c remains open for the remaining parameter/topology corpus, executed target/feature cases, whole-run memory/allocation evidence, cancellation during expensive inner work, and broader performance limits. No C00–C14 package is closed by this delivery.

- C03c.1 delivery `da18bfa88`, pushed to `main`, adds independent bias-dependent forward and reverse transit-charge oracles (`XTF/VTF/ITF/TR`). The added test covers 72 public runs: both phase laws, three dialects, both polarities, three operating regimes and two step sizes. Native development/release and optimized WebAssembly worker selections each pass the three prescribed-terminal regime tests. The new charge cases have a conditioning-based floating-point conservation allowance; their physical-current accuracy/refinement gates and the earlier conservation gates are unchanged.
- Numerical repair `50523d8fb`, pushed through merge `743ca9097`, retains weak peripheral charge modes by selecting the highest-incidence node as each floating charge component's algebraic representative. A three-node star with capacitances `1e-9` and `1e-30` passes for all six node orderings without removing storage. Checkpoint configuration identity is v98. The development tree passed 4,872 library tests (three existing ignores), 73 selected integration tests and strict Clippy; subsequent merged checks passed all 34 charge-event unit tests and four regime tests. These runs have different source/fixture scopes and are not a full-suite certificate for the latest merge.
- Required reproductions `4fb1105d8`, pushed through `36a5606f8`, preserve the two positive private-node tests as explicitly unresolved ignores, plus the behavioral-source event-admission probe and ledger ownership. Explicit execution at that revision failed both cases; the later startup repair and remaining failure are recorded below. Strict library/fixture Clippy, all three requirements checks and 17 RAW I/O tests pass at the preceding development source. These passing checks do not resolve the numerical failures or supply the missing event equations.
- On committed compiled source `36a5606f8`, the four optimized core WebAssembly integration targets pass 20 tests in a Chrome dedicated worker with `wasm,veriloga-wasm-jit`; the two required private-node regressions remain ignored. The charge/event delivery, historical and current source scopes, log/artifact hashes, and unresolved failures are recorded in `core-gp-charge-events-windows-20261006.json`. This is core numerical execution evidence, not external-model or application qualification.

- C03c.2 startup repair `4ec035c19`, pushed through `92cd7cf66`, preserves an authenticated unconstrained operating point when forming physical startup rates. Per-source side identity and conditioning checks exclude real jumps, changed gmin, released ICs and uncertified startup; line-history startup is conservatively excluded. Constant authored forcing extends its stationary prehistory without a synthetic time-zero transport event. Tiny capacitances, differentiated source constraints and the original KCL/flux tolerances remain intact. Checkpoint configuration identity is v99. Six focused regressions guard these contracts; the constant private-bias test is active for all three dialects and both polarities. Resource tests cover both startup-history forms at exact and one-byte-insufficient limits. At that revision the driven high-injection test still failed and remained required.

- Qualification for `4ec035c19`: 74 distinct selected native integration tests pass, along with strict library/GP-fixture Clippy and requirements checks. A broad transient run passed 565 cases and exposed one new source-jump fixture error; correcting that fixture and rerunning all three contract tests establishes 566 distinct successful selected cases, with one existing diagnostic ignore. On committed compiled source `92cd7cf66`, all 21 selected optimized WebAssembly worker tests pass with one required ignore. Explicit execution of that driven high-injection regression failed after 284 iterations at that revision. `core-gp-operating-point-windows-20261006.json` records per-run source scopes, failures and corrections, hashes, and remaining gates.

- C03c.2 continuous-event repair `87ff9cbf9` uses retained integration current only as a predictor checked against physical equation budgets. Original storage rows and nonzero weak-mode rates remain; individual forcing-side identity excludes real tiny jumps, and floating components share the omitted KCL row's remaining budget. Storage-rate audit normalization retains static contribution scales rather than only cancelled net F. Six new unit cases cover weak currents, source slopes/jumps, shared KCL and capacitor/mutual-inductor companions. Checkpoint identity is v100.
- Physical-event approach repair `0d6ceb788` balances short tails across two unsolved intervals while preserving existing bounds and exact clocks. It repairs the separate 1.4e-22 s tail exposed by the additional 0.4 ps BestAvailable high-injection run. Two new scheduling tests cover all three dialects, replay, other exact events and current bounds. Checkpoint identity is v101. The driven private-node regression is active with 42 public runs and unchanged physical accuracy/refinement limits; the finer-grid protocol is documented below.
- Qualification for the combined repairs: 574 selected transient unit tests pass with one existing diagnostic ignore; all 75 selected native integration tests pass with zero ignores, along with the three requirements tests and strict library/GP-fixture Clippy. On committed compiled source `63d47419a`, all 22 selected optimized WebAssembly tests pass in a Chrome dedicated worker with zero ignores. `core-gp-driven-events-windows-20261006.json` records exact commands, source scopes, log/artifact hashes, development failures and their corrections. Both named private-node regressions are repaired; all 15 packages remain open.

- C03d.1 delivery `03f270c65`, pushed to `main`, adds structurally smooth prescribed behavioral voltage/current sources to physical events. The provider reuses analytic expression derivatives and the existing charge/impulse solver, preserves typed cancellation/resource failures, and requires structural constancy for stationary startup. Checkpoint identity is v102. Circuit-dependent, switched and stateful sources remain explicit implementation gaps.
- Qualification for this provider: 122 selected behavioral unit cases pass; the overlapping broader transient selection passes 577 cases with one existing diagnostic ignore. All 79 native integration tests across eight targets pass, along with three requirements tests and strict library/selected-fixture Clippy. On committed source `03f270c65`, all 24 selected optimized WebAssembly tests pass in a Chrome dedicated worker. The positive GP matrix makes 42 public calls across all three dialects and both polarities, including independent currents, startup impulse and restart checks; the new matched-line oracle makes nine native public calls. `core-gp-behavioral-events-windows-20261006.json` records source scopes, commands, hashes, limits and corrected test-development failures. All 15 packages remain open.

- C03d.1 voltage-controlled delivery: `eed21ead1` adds jointly solved voltage-control constraints to the charge-event operator; `5619108f7` integrates native VCVS/VCCS providers, shared VCCS coefficients across DC/transient/AC/HB, finite G-current observations and impulse ownership. Checkpoint identity is v103. Feedback inputs remain high impedance, and E-output currents include the solved charge impulse.
- Qualification for this slice: 582 transient unit tests pass with one existing diagnostic ignore; the separate VCCS arithmetic case, 118 integration tests across 11 targets, three requirements tests and strict library/selected-fixture Clippy pass. All 26 selected optimized WebAssembly tests pass on committed source in a Chrome dedicated worker. New public fixtures execute 30 GP calls across three dialects and both polarities, nine matched-line calls and three ordinary transient current-observation calls. `core-gp-voltage-controlled-events-windows-20261006.json` records source scopes, commands, hashes, numerical limits and corrected fixture settings. This does not close C03d.1 or any package.

- C03d.1 CCCS delivery: `fc23d131b` extracts the existing bounded exact-elimination kernel for shared PSS/event use; `8fb9d66ad` adds native F-element finite/current-impulse incidence, weighted charge conservation, finite controlling-state equations and I(F) output. A compiled linear voltage seed repairs the reproduced Xyce UIC Newton exhaustion without relaxing charge/KCL audits or increasing the iteration budget. Checkpoint identity is v104.
- Qualification for CCCS: 39 PSS state tests pass for the shared-kernel extraction; 588 transient unit tests pass with one existing diagnostic ignore; all 121 selected native integration tests across 11 targets, three requirements tests and strict library/selected-fixture Clippy pass. All 28 selected optimized WebAssembly tests pass on committed core source in a Chrome dedicated worker. The two new GP fixtures execute 72 public calls across all dialects and both polarities, including signed impulse transfer, OP/UIC, RC timestep refinement and checkpoint continuation. Three additional ordinary transient calls verify selected I(F) output. `core-gp-current-controlled-events-windows-20261006.json` records source scopes, hashes, commands, limits and development failures. All 15 packages remain open.

- C03d.1 descriptor foundation: `79e24649f` extracts the exact derivative/algebraic constraint closure used by PSS. Six new analytical/contract tests cover CCVS flux state, second-derivative charge, exact feedback rank, shared derivative symbols, owner-controlled forcing constraints, and bounds/cancellation. Native CCVS physical-event admission is still open.
- Startup seed repair: `e584ecaef` corrects source and incoming free-coordinate RHS signs. A direct test reproduced a +2 V seed for an authored -2 V source; the earlier Newton/public tests had recovered from that erroneous trial and did not establish seed consistency. The new test checks both source polarities, both VCVS gain signs, free-coordinate preservation and untouched branch slots. Physical acceptance gates and iteration limits are unchanged; checkpoint identity is v105.
- Qualification: 32 descriptor-selected tests, 54 charge-event tests, 39 PSS state tests, three ledger checks and strict library/selected-fixture Clippy pass. These unit selections overlap. All 42 selected native integration tests and 28 optimized core WebAssembly tests pass without ignores. The PSS numerical checks in this batch are native; the worker run exercises the corrected seed through public GP simulation. `core-descriptor-closure-seed-windows-20261006.json` records exact scopes, commands, hashes, unchanged accuracy limits, the reproduced defect and corrected test-development failures. No package is closed.

- C03d.1 transition foundation: `90247985e` adds a test-gated constant-linear descriptor transition kernel. Its joint solve retains finite values/rates and all required impulse derivatives, uses incoming charge/flux directly, checks compatibility with the storage matrix, and audits original and hidden equations before returning. Eleven new tests include CCVS voltage impulses, winding-current jumps, capacitive delta-prime currents, authored startup storage, a length-five nilpotent chain, exact feedback rank, malformed inputs, resources and cancellation. A development test exposed a missing algebraic-current-rate audit; differentiated hidden constraints now catch that corruption. The kernel is not yet called by production circuit preparation, state acceptance or observations.
- Qualification for this foundation: all 17 exact-constraint tests pass on committed source, comprising six existing closure tests and 11 new transition tests. Strict Clippy passes for the core library and every test target after the separate eight-warning fixture cleanup `33d1f4be8`. No test tolerances or physical admission predicates changed. `core-descriptor-transition-kernel-windows-20261006.json` records the source, analytical cases, corrected development failure and remaining integration limits. Native CCVS physical-event support and all 15 packages remain open.

- C03d.1 current observation delivery: `d3d15ca70` adds explicit positive derivative orders and coefficients with units A*s^(order+1), independently of ordinary charge impulses and finite samples. Fourier and rectangular FFT evaluate the analytic distributional contribution at its exact event time. Result document v10, Python pickle v3, compression, live delivery, worker transfer and authenticated dependencies retain the new terms. Shared numeric accounting includes all three derivative fields. The integration review caught and corrected Python crop retention and worker/artifact accounting before publication.
- Qualification for this slice: 58 selected core impulse tests and 54 simulation adapter tests pass on the final source, with strict Clippy for core/results/simulation library and test targets. Earlier bounded runs cover five observation tests, 65 result-document tests, compression, two native public Fourier tests and four result tests; these selections overlap. Python's two Rust impulse tests and 21 actual extension tests pass, along with strict Python library Clippy with native Verilog-A disabled. Both public Fourier tests pass in optimized WebAssembly on committed source. `core-current-derivatives-windows-20261006.json` records exact scopes, versions, hashes and development failures. At that revision, circuit generation of higher current derivatives, voltage impulse observations, nonrectangular FFT taper derivatives, generalized INTEG/AVG boundaries, envelope projection, CCVS coupling and atomic state/restart remained open. The subsequent FFT taper and partial current-integral deliveries are recorded below; the remaining obligations stay open. Explicit temporary refusals do not complete those features. Checkpoint identity remains v105 and all 15 packages remain open.

- C03d.1 FFT delivery `bcaefd8a3` implements analytic current impulse derivatives through every supported window. Finite samples and derivatives share stable window formulas; reflected sine polynomials repair false zero weights near taper endpoints. The public retained-result FFT route uses the same preflight and evaluation as transient post-processing. Singular contributions combine with finite samples and charge before normalization and metrics.
- `fft_current_derivatives.rs` compares 400 spectra (2,000 complex coefficients) against independent 100-digit differentiation of the original window definitions. All 16 windows, both conventions, selected orders and Gaussian/Kaiser alpha boundaries pass. Separate tests cover support/corner smoothness, extreme compensating scales, affine expressions, resource bounds, cancellation and reuse. Native and optimized WASM execution plus exact source/log provenance are recorded in `core-window-derivatives-windows-20261006.json`.
- The FFT delivery replaces the temporary nonrectangular-window implementation refusal. Undefined classical derivatives at nonsmooth corners/support boundaries remain explicit domain errors. This does not generate physical CCVS derivatives, implement voltage impulses or generalized INTEG/AVG/envelope projection, admit H elements, change checkpoint v105, or close a C00–C14 package.

- C03d.1 current integral delivery `ba2f8cfd4` replaces the blanket direct INTEG/AVG refusal with finite primitive evaluation and explicit singular boundary state. Interior higher derivatives have zero regular contribution; a singular fixed lower endpoint is refused, and an upper-event singularity does not poison later regular values. Ordinary charge remains separate. Exact weighted sums classify cancellation across owners without narrowing overflowing or underflowing factors.
- Live measurement programs preserve singular availability separately from numeric values. Actual reads cannot normalize a singular sample into a finite expression sentinel. Direct AT queries inspect the requested time, including off-grid events, and retain the checked live result and axis metadata. The first regression exposed and repaired an offline overwrite of that live result. Constant inactive IF branches remain isolated; unrelated compile failures remain local.
- Six public measurement regressions, all 130 `measure_signals` unit tests and strict core Clippy pass. The six public tests also execute in optimized WASM; `core-measure-derivatives-windows-20261006.json` records the exact source, commands, hashes, analytic cases and development failures. This is partial measurement support: nested generalized integration/averaging and arbitrary derived-expression distribution propagation remain open. Unimplemented interval consumers retain an explicit refusal, including off-grid primitives and aliases. Those refusals do not close the operators. Voltage observations, envelope projection and CCVS physical-event integration remain open; checkpoint v105 and all 15 package statuses are unchanged.

- C04a/C04b DC delivery `1dd0e1146` executes explicit `dc` and declarative `run` through the same public `Engine::run_dc_analysis_with_abort` route as the CLI and WASM deck adapters. The typed result retains compact outer/inner coordinates, units, accepted point results and device reports. Control presentation exposes either axis and refuses arithmetic between different nested grids. Failed/cancelled solves do not publish or consume a dataset ordinal; the cumulative budget includes coordinate storage and reports.
- Integration fixes preserve LIST/DEC/OCT specifications previously discarded by CLI execution and retain both axes in typed DC documents. CLI axis units now follow the shared physical-unit contract. Follow-up `17ea0ca58` preserves authored axis spelling after the existing export regression caught a case-normalization change. The document constructor rejects inconsistent inner coordinates. Existing OP/AC/transient control behavior remains covered.
- Qualification for this slice is recorded in `core-control-dc-windows-20261006.json`: six new public DC regressions, eight existing control tests, six existing nested-sweep tests, three inventory tests, nine CLI/preflight tests, four existing DC export/nested-step checks, a WASM-adapter document test, strict three-crate Clippy, and six optimized WASM regressions. Independent resistive-network equations cover voltage/current signs and temperature response; route equality checks data, reports, grids and publication. CLI runtime tests use installed Rust 1.95 because pinned Rust 1.94 crashes internally while compiling `faer`; the pinned three-crate check and Clippy pass. This compiler failure remains a qualification limitation, not a waived product gate. The ledger now records four implemented variants and 25 missing dispositions. C04 remains in progress and no package closes.

- C04a/C04b ordinary NOISE delivery `ad6230cde` executes explicit `noise` and declarative `run` through the existing named-port solver. It preserves complex observations, qualified datasets, amplitude/power density units, squared gain, DNO/DNI contributions, resolved settings, cumulative result limits and atomic publication. AC and NOISE share bounded frequency-grid construction; cancellation and point limits keep their typed errors. CLI/WASM adapters reuse core results and typed documents; current-referred flat/HDF5 spectra state `A/sqrt(Hz)`.
- Independent RC/current-input tests found authored AC amplitude contaminating BestAvailable/ngspice input referral. Repair `fa7fc79c0` uses the selected unit-source transfer from the existing adjoint, without another solve, and retains Xyce's full-AC convention and all ordinary AC observations. Desktop ngspice 46/Xyce 7.10 source inspection and independent circuit equations document the distinction; no external simulator execution or full dialect certification is claimed.
- The original memristor CLI regression exposed a 1 MiB stack overflow. A Windows exception trace located behavioral-expression reparsing for checkpoint identities under the large integration frame. C02 repair `91bb08b05` prepares the same `CheckpointIdentity` after admission and before entering that frame. The ordinary and diagnostic CLI builds now pass; 36 checkpoint regressions preserve trajectories and identities. General stack bounds and recursive parser frame reduction remain C13/C14 work.
- `core-control-noise-windows-20261006.json` records the noise/control/adapter checks, ten optimized WASM regressions, stack diagnosis and repair, source hashes and local reference inspection. The ledger now has five implemented control variants and 24 missing dispositions. Native CLI runtime checks continue to use Rust 1.95 because of the recorded pinned Rust 1.94/faer compiler failure; strict pinned three-crate Clippy passes. All 15 packages remain unfinished.

## Next implementation batches

Both named C03c.2 regressions are repaired. Implement C03d's missing event providers in bounded slices and complete C03c's remaining qualification gates. C04 result-contract design and independent existing-solver adapters do not depend on completing every C03 row. Keep any unfinished C03 gate explicit when progressing through C04 and C05.

### C03c remaining qualification batches

| Batch | Work and commit boundary | Completion evidence |
|---|---|---|
| C03c.1: bias-dependent charge — delivered | `da18bfa88` adds independent `XTF/VTF/ITF/TR` cases for instantaneous forward/reverse charge alongside delayed forward transport | Both phase laws, three dialects, both polarities, saturation/reverse-active/high-injection, and timestep refinement. The conditioning-based conservation allowance applies only to added charge cases; previous conservation and physical-current accuracy gates are unchanged |
| C03c.2: private nodes and feedback — in progress | Constant-bias startup (`4ec035c19`) and the driven manufactured private-node matrix (`87ff9cbf9`, `0d6ceb788`) are repaired and active. Complete a separate Weil feedback oracle and the remaining private-node charge/parameter and continuation cases | Preserve both active regressions, independent forcing, intrinsic voltages/terminal currents, conservation and refinement. Add rejection/restart and OP/UIC where applicable, plus cutoff, depletion/overlap, parameter-boundary, temperature and scaling gaps. Successful or finite output alone is insufficient |
| C03c.3: allocation and cancellation | Inventory remaining allocations in initialization, accepted history, capture/restore, and retained output; exercise failures at the actual owners. Measure cancellation inside expensive nonlinear/factorization work and add propagation where absent | Requested/allowed resource diagnostics, bounded whole-run memory as well as transport-record quota, unchanged accepted state/checkpoints on failure, successful reuse, and measured cancellation latency with the specific inner operation identified |
| C03c.4: configuration and performance | Reconcile native features/profiles and executed core target coverage with C14. Run remaining distinct configurations and representative feedback/history workloads; retain the existing shared test registration and benchmark harness | Committed source, exact configuration, independent numerical errors, work counts, memory/allocation measurements, and repeatable runtime limits. Record any unexecuted configuration explicitly; compilation alone does not qualify execution |
| C03c.5: closure review | Reconcile every C03 requirement, including C03d integration, against the delivered tests and evidence; remove superseded temporary qualification paths and update the ledger in a separate documentation commit | No required ignored failures, no unassigned C03 requirement, no unexplained accuracy or resource regression, and every required configuration with an outcome. GP periodic continuation remains C06; dependencies on other provider owners cannot silently waive C03 integration |

Preserve C03a/C03b, the delivered transport-byte quota, event-remainder repair, exact-delay oracles, and discrete Weil oracles throughout these batches. Do not expand a qualification task indefinitely without recording a specific missing case and completion test.

### C03c.2 repaired failures and remaining evidence

Both named positive numerical regressions are now active. Their repairs do not complete the broader feedback, parameter and continuation matrix.

| Regression | Current disposition | Evidence to preserve |
|---|---|---|
| `static_private::gp_exact_phase_retains_a_small_reverse_charge_mode_at_private_dc_bias` | Repaired in `4ec035c19`: accepted OP balance initializes finite rates without amplifying static roundoff; structurally constant forcing creates no fictitious startup transport event | All three dialects and both polarities preserve DC bias and terminal currents through 2.5 ns with actual RB/RC/RE, `RELTOL=1e-7`, and the original current limit |
| `private_nodes::gp_exact_delay_solves_biased_charge_through_private_terminal_resistances` | Repaired in `87ff9cbf9` and `0d6ceb788`: audited companion-current predictors prevent false weak-charge rates; balanced unaccepted intervals avoid a near-zero physical-event tail. All 42 public simulations pass | Independently manufactured native-source forcing, recovered intrinsic voltages/lead currents, conservation, original current/voltage limits and the 0.7 refinement gate. Ngspice/Xyce retain 40 ps/4 ps; BestAvailable retains 4 ps accuracy checks and adds 0.4 ps with an actual sample-count refinement requirement |

Run the active driven regression with `cargo test --locked -p rspice-core --test gp_phase_regimes private_nodes -- --nocapture` and `CARGO_INCREMENTAL=0`. The current execution evidence is `core-gp-driven-events-windows-20261006.json`. Earlier failures remain recorded unchanged in `core-gp-operating-point-windows-20261006.json` and `core-gp-charge-events-windows-20261006.json`.

The BestAvailable refinement protocol changed because its adaptive 40 ps request already produced almost the 4 ps grid. No claim is made that the original 4 ps refinement ratio passed. The additional 0.4 ps case exposed a separate 1.4e-22 s event-tail failure, repaired by interval planning before solving. Physical storage-rate residual normalization now retains the static contribution scale, matching the original KCL audit; this change is explicit and does not increase user tolerances or the independent oracle limits.

Remaining implementation/qualification slices:

1. Add an independent Weil private-node/feedback oracle, including its discrete state recurrence and the same public terminal/charge observables. Exact-delay evidence cannot substitute for it.
2. Complete cutoff, depletion/overlap, transit/phase parameter boundaries, temperature and area/multiplicity scaling through private nodes. Identify each case and its independent acceptance gate in the ledger before treating the matrix as complete.
3. Extend the driven private-node cases through rejected trials, exact/scheduled checkpoint restart, horizon extension, and OP/UIC where applicable; compare both numerical output and accepted history.
4. Preserve the small-charge row selection, authenticated OP startup contract, continuous forcing checks, omitted-node KCL budget and exact event clock. Real tiny excitations must retain their physical rates; no capacitance cutoff or blanket residual suppression is authorized.
5. Complete the C03c allocation/cancellation/configuration/performance gates and C03d device integration before closing C03. The absence of these two ignored failures is only one completion condition.

### C03d physical-event integration sequence

`03f270c65` replaces the original smooth-source refusal with physical voltage constraints, current contributions, analytic time slopes and impulse ownership. `gp_phase_regimes/behavioral.rs` uses the original `tests/testdata/qualification/gp-behavioral-event-gap.cir` fixture for independent collector/base-charge equations, OP/UIC startup and checkpoint continuation. Structural regularity guards still refuse switched, circuit-dependent and stateful expressions; those are remaining implementations. `transient_transmission_line.rs` also qualifies smooth behavioral forcing through an independently checked matched-line delay and continuation.

`eed21ead1` and `5619108f7` add native E/G source event equations and VCCS current observations. The voltage-control constraint is solved jointly, including feedback; its control port introduces no current or charge incidence. VCCS uses the shared current law and matrix coefficients. Independent tests cover OP/UIC, nonlinear charge impulses, exact-delay arrivals, checkpoint continuation, matched transmission-line forcing and ordinary transient I(G) selection.

`fc23d131b` and `8fb9d66ad` add native CCCS event/current providers. Controlling ideal-source impulses participate in the full output incidence; exact weighted conservation replaces a simple graph sum only where needed. Finite inductor controlling state remains a constitutive coordinate. The new RC test retains its 4 ps coarse run and requires error reduction below 0.7 at 1 ps, with the original 2 microvolt waveform gate on the refined and resumed results. This distinguishes time-integration error from charge-impulse accuracy.

`79e24649f` supplies shared exact descriptor constraint closure for the next CCVS transition implementation; PSS already consumes it. `e584ecaef` repairs the CCCS startup seed's source/free-coordinate signs, independently of that extraction. For CCVS, the closure tests establish two required cases: an input capacitor driving a CCVS/inductor retains an independent winding state, while a capacitive CCVS output requires a second forcing derivative. The remaining transition must preserve the resulting voltage impulses, current jumps and any higher-order impulse terms, along with finite-rate, nonlinear-domain, observation and restart contracts. The observation contract cannot silently discard an impulse derivative. `d3d15ca70` preserves typed higher current derivatives and their Fourier/rectangular-FFT contributions; `bcaefd8a3` extends FFT evaluation through every taper. `ba2f8cfd4` adds direct current integral boundary state and finite-value recovery. Voltage impulses, complete generalized interval-operator composition and envelope projection remain missing. These foundations do not admit H elements to physical events.

`90247985e` implements and tests the constant-linear transition mathematics. Finish its production integration in these commit boundaries:

1. Connect structurally certified device equations and analytic sided forcing derivatives to circuit preparation. Reuse authoritative linear stamps, and define the actual coupling to nonlinear GP charge/current ports. A sampled DC Jacobian is insufficient. Feed accepted charge/flux and authored startup storage into the transition, and keep physical nonlinear-domain constraints in the joint solve.
2. Complete the observation contract begun in `d3d15ca70`. Add typed voltage impulses, and bind actual descriptor-produced higher current derivatives to their physical owners. Preserve both through live samples, serialization, compression and accepted checkpoint state without replaying past actions. Analytic derivatives for all FFT tapers are delivered in `bcaefd8a3` with independent numerical, boundary, range and cancellation regressions. `ba2f8cfd4` supplies direct INTEG/AVG primitive boundary state, finite values at regular endpoints and selected live/AT consumers. Complete generalized interval-operator composition and envelope projection, replacing remaining temporary refusals with independent numerical regressions. For composed measures, retain singular coefficients and orders as well as their finite primitives; implement nested integration/averaging and supported affine expression propagation at exact event times. Preserve declaration-order references, lazy branches, units, endpoint ownership and cancellation. Distinguish genuinely undefined products and point values from missing operators; conservative support tracking is not completed algebra. Preserve units, coverage and resource bounds; never encode a voltage impulse or delta-prime current as a finite sample or ordinary charge impulse.
3. Integrate finite outgoing coordinates/rates and winding-current jumps with atomic event acceptance, rejection and restart. Exercise public core circuits with inductive and capacitive CCVS loads, nonlinear GP coupling, signed gains, startup ICs, ordering changes and checkpoint seams. Admit H elements only once those equations, state and observation contracts pass; qualify the core target configurations separately.

At the reviewed revision, `PreparedEventCircuit::admitted_family` admits 14 of the 38 declared families, including native VCVS/VCCS/CCCS and a restricted behavioral-source provider. The other 24 need an explicit event formulation or a justified, tested mathematical exclusion. Admitted families still have instance restrictions; behavioral admission currently requires structurally smooth prescribed forcing and supported analytic instructions. The following slices cover the remaining integration gap:

| Slice | Providers and restrictions | Implementation and acceptance |
|---|---|---|
| C03d.1 — in progress | Behavioral source; VCVS, VCCS, CCCS, CCVS | Smooth prescribed B-voltage/current providers and native E/G/F equations, finite currents and charge impulses are delivered. Complete CCVS descriptor transitions, including required voltage impulses and inductor-current jumps; extend behavioral support to remaining regular expression domains, circuit dependence, authored discontinuities and dynamic operators with their actual state contracts |
| C03d.2 | Diode, MOS, BSIM3/4, B3SOI DD/FD/PD, EKV 2.6/3, VDMOS, JFET; admitted BJT variants currently rejected as VBIC/thermal | Reuse each device's actual equations, private nodes and charge derivatives. Include startup, event transitions and nonlinear continuation alongside exact-delay GP; ordinary transient or periodic support alone is insufficient |
| C03d.3 | Memristor; voltage/current/generic switches; coupled lines; multi-winding, Jiles–Atherton and shared Xyce magnetic cores | Bind accepted state, sided constitutive equations, distributed histories and jump/impulse constraints with C07. Test history rollback and coincident events as well as numerical accuracy |
| C03d.4 | XSPICE, runtime/generated Verilog-A, mixed host | Consume actual provider equations and state under C09/C10. Require atomic preparation/acceptance and authenticated restoration; capability metadata cannot replace an evaluator |
| C03d.5 | Admitted thermal resistors, expression/auxiliary-current capacitors, and excluded scalar-line branch/distributed variants | Inventory each predicate in `charge_event/circuit/prepare.rs`; implement its time/state owner and independent boundary test, then remove only the corresponding guard |

Use C01 to share physical equations and C02 to keep prepare/trial/accept responsibilities explicit. Do not simply remove admission guards, skip device stamps, or route a stateful provider through a stateless surrogate. Keep any formulation that is mathematically inapplicable explicit with a tested reason; implementation gaps remain open even when another package supplies the dependency.

### C04 control-host commit sequence

Each result-family batch includes public control/direct equivalence, result identities and units, vector/scalar access, cancellation, cumulative retention limits, and the affected result adapters. Reuse the existing solver entry points. Update the ledger only when the command executes correctly through the core control host.

| Batch | Command variants or shared work | Required behavior |
|---|---|---|
| C04a | Typed results, atomic dataset publication, and producer identity | Extend the existing result contract without changing qualified OP/AC/transient behavior; failed execution cannot consume a dataset identity or publish a partial result |
| C04b | `Dc`, `Noise`, `AcData`, `NoiseData` dispatched and published | Single/nested DC axes and table row identity, device operating-point reports where provided, complex AC values and noise units |
| C04c | `Tf`, `PoleZero`, `Sensitivity`, `Stb`, `Sp`, `Disto`, `DcMatch` | Preserve each existing solver's scalar/complex/result structure, options, reference/probe identity, and error contract |
| C04d | `Four` | Explicit compatible transient producer and interval; retain checked measurement failures and distinguish no producer from invalid data |
| C04e | `Hb`, `Pss`, `Qpss` | Retain complete steady-state producer configuration and state so dependent analyses can authenticate it |
| C04f | `Pac`, `Pxf`, `Pnoise`, `Pstb`, `Envelope`, `Qpac`, `Qpxf`, `Qpnoise` | Validate producer compatibility; preserve sideband/lattice identity, conversion data, correlations, and continuation |
| C04g | `Step`, `Temp`, `MonteCarlo` | Bounded child-analysis orchestration, nested identities, deterministic seeds, completed-child retention, cancellation, and circuit/configuration invalidation |

These groups enumerate the original 26 missing variants exactly once; `Dc`, `Noise`, `AcData`, `NoiseData` and `Tf` have core dispatch and 21 remain. Table documents and CLI/WASM adapters now publish coordinates and binding/completion metadata. Keep OP/DC/AC/NOISE/TF/transient and mixed-script regressions in every affected contract check; producer cards and wrappers must not be forced into standalone analysis semantics.

C04b delivery is tracked in the following slices:

1. **Core result/retention slice delivered in `2bb484dbd`.** `FrequencyDataResult<T>` retains authored columns, canonical targets, row order, requested row count and model-finish metadata. The shared AC/noise executor validates borrowed input before allocating coordinate/results vectors, propagates caller limits into replay, and charges coordinates plus prior points against each row. Legacy APIs share that executor and additionally bound retained root-source bytes. Independent RC equations qualify changing R/C/TEMP, repeated/reordered frequencies, exact budgets, cancellation and compatibility. `733fc6c66` fixes unknown resistor overrides and aliases that failed to replace the consumed binding. Evidence: `core-frequency-data-windows-20261006.json`. This does not qualify every configuration precedence case or bound the entire AST heap.
2. **Core control dispatch/presentation implemented.** `AcTable` and `NoiseTable` retain the shared compact results, including coordinates and model-finish metadata. Executed options overlay only explicitly changed fields, unresolved callers resolve each row, and physical TEMP columns override resolved caller/run temperatures. AC circuit equations and thermal-noise sources use the same temperature. Canonical targets and every coordinate determine grid compatibility, including reordered columns; repeated/reordered frequencies remain intact. Failed/cancelled rows do not publish or consume an ordinal. Public tests cover direct/declarative/explicit parity, alteration immutability, option precedence, mixed OP/DC/AC/NOISE/TRAN scripts and exact retained-value boundaries. Broader configuration/binding qualification remains open.
3. **Document and adapter publication implemented:** version 11 documents carry table names, authored columns, canonical targets, requested rows and model-finish evidence; aligned axes carry each coordinate once. CLI/WASM direct/control routes use the compact runners and shared builders. CLI static `run` preflight admits both table variants, browser metadata exposes their bindings, and windows retain all axes. CLI JSON preserves full metadata; CSV/TSV/RAW/HDF5 preserve numeric coordinates through conversion, clipping and comparison. Noise band totals require at least two strictly increasing frequencies with every other coordinate constant. Tests cover changing resistance/temperature against closed-form AC and noise equations, repeated/reordered frequencies, direct/control parity, rollback and compact retention. Broader configuration, cancellation, transport and runtime qualification remains scoped separately.

Qualification for `cacddf36c`, `33c176b7a`, `b6d545304` and `d7f95d93a` is recorded in `core-control-frequency-data-windows-20261006.json`: 162 native execution tests, 55 optimized WebAssembly tests, 3 inventory checks, strict core/consumer Clippy and workspace formatting. The native/optimized-WASM runtime source is committed revision `d7f95d93a`; the conformance enum-consumer compile check is at `b6d545304`. The expander now preserves tables needed by frequency/control consumers, including shared/continued selectors and ALTER variants. CLI static `run` preflight now admits the two table variants alongside publication. None of these scoped checks closes C04 or certifies production readiness.

Initial publication validation passed 660 CLI tests, native result-document/API tests, strict CLI/WASM Clippy and all-target checks for CLI, desktop, Python and conformance consumers. After integrating `557cbdb0c`, the affected 7 CLI and 20 core tests passed again, as did 4 document and 1 adapter tests executing as wasm32 modules. Those initial WebAssembly runs used the unoptimized test profile.

Hardening in `557cbdb0c`, `58776c350` and `af155b9c6` recognizes implicit TEMP columns, validates borrowed table evidence before bounded numerical projection, preserves caller cancellation/limits through CLI and WASM publication, rejects unknown finish fields, and requires aligned noise contributions. The public version-11 wire shape is unchanged. Only the intended schema-version fact changed in the qualification baseline; existing numerical and performance gates remain intact. `core-frequency-table-documents-windows-20261006.json` records 42 core integration tests, 65 result-document unit tests, 31 CLI tests, 4 native WASM-adapter tests, strict four-crate Clippy, workspace formatting, and 17 optimized WebAssembly tests in a Chrome dedicated worker. Independent RC and voltage/current thermal-noise equations accompany round-trip, window, malformed-input, legacy-schema, exact-budget and cancellation checks. The evidence distinguishes development checks whose source was subsequently committed unchanged from checks executed at committed revision `af155b9c6`. The remaining core qualification matrix and earlier compiler/runner causes stay open.

The table publication contract and remaining qualification obligations are:

- **Typed document contract, implemented** (`execution/result_document.rs` and its frequency-table/builders modules): version-11 AC/noise documents retain table name, authored column names, canonical targets, requested row count and model-finish metadata. Aligned typed axes preserve row order and duplicate/decreasing frequencies through windows and numeric accounting. Validation checks unique targets, axis references and units, row counts, frequency agreement, finite values and completion metadata. Older documents decode without invented table identity and refuse newly added table evidence.
- **Direct/control adapter publication, implemented** (`rspice-cli/src/commands/run/{frequency,control}.rs`, CLI preflight and `rspice-wasm/src/runners/deck{,/control}.rs`): compact table APIs and shared bounded document builders publish direct/declarative/explicit results. Static preflight uses shared borrowed table admission. Preserve resolved configuration precedence, typed cancellation/resource failures, completed-result identities and atomic publication when extending these routes.
- **Exports and transport**: retain coordinates in JSON, flat formats, HDF5 and artifact/window transfer. JSON and browser metadata also retain target bindings and completion evidence; flat files retain numeric rows only. Audit the document consumers for assumptions of a single frequency axis. Charge the actual retained numeric storage and preserve a model-finish prefix without representing it as a complete requested table.
- **Noise integration policy**: publish a band integral only for at least two strictly increasing frequency rows with every other physical coordinate constant. Repeated/decreasing frequencies and changing resistor/TEMP/parameter rows still publish their spectra and coordinates, with an explicit unavailable band result rather than a misleading integral. Keep voltage/current input-referred units correct.
- **Qualification and closure**: the scoped native and optimized-WASM document, row-temperature and adapter checks above are delivered. Preserve those regressions while extending binding/configuration coverage, per-row model/noise catalogs, transport consumers, whole-run memory and expensive-work cancellation. Finish-document tests construct accepted prefixes; shared table-driver finish execution evidence remains separately indexed. C04c-C04g and the remaining C04 qualification obligations are open; this publication batch does not close the package.

The parser work supporting C04b has delivered these contracts:

| Commit | Contract to preserve | Evidence |
|---|---|---|
| `6383db27f` | Parameter-dependent TEMP/TNOM, conditional/source gating, bounded seeded replay, earlier eager values reconciled, physical TEMP coordinate precedence | `core-temperature-options-windows-20261006.json` |
| `8eb0c3139` | Forward scalar temperature options within lexical scopes; captured complex bindings/functions, lazy evaluation, option order, source locations and cancellation checks | `core-temperature-forward-windows-20261006.json` |
| `07b4071bf` | Suspended numeric evaluation of retained ordinary/global graphs, complex values and function frames, distinct namespaces, one-time materialization of shared option dependencies | `core-parameter-resolution-windows-20261006.json` |
| `1a67d5f85` | Ordinary forward declarations in both dialects, bare/signed aliases, duplicate policies, static ngspice/runtime Xyce semantics, failed-probe draw preservation and declaration origins | `core-forward-parameters-windows-20261006.json` |
| `1e21d0d20` | Ordinary parameters finalized before deferred sources/models, one shared sample, source-grammar probes without live draws, physical root-source errors, public transient and DC/AC sensitivity checks | `core-forward-sources-windows-20261006.json` |
| `138b4f16e` | Static global projections for deferred sources/models; retained global bodies and distinct namespaces; shared samples including user functions and two-argument LIMIT; indirect runtime-dependency guard; complex numeric materialization | `core-forward-globals-windows-20261006.json` |
| `9912113ee` | Complete analysis-card transactions, including diagnostic/output/Monte Carlo/noise effects and LIN/FFT auxiliaries; isolated failed statistical probes; strict AC DATA trailing-field validation | `core-analysis-card-transactions-windows-20261006.json` |
| `42ed06c67`, `d9af822b8` | Failed optional operands remain visible to validation; root analysis forward bindings preserve captured values/functions, ordered effects, statistical phases and MC spans; public DC/AC/transient and 26 numeric-family grammar checks | `core-forward-analyses-windows-20261006.json` |
| `022237471` | Physical source locations attached to staged diagnostics before delayed publication; included ready/pending FFT warning regression | `core-forward-analyses-windows-20261006.json` |
| `0bb6ee975`, `5550bb89a` | Demand-driven analysis operands in lexical scopes; parent/default completion, shared samples, inherited snapshots, suspended lazy operands and per-instance body preservation | `core-scoped-analyses-windows-20261006.json` |
| `d63caea8a`, `520527555` | Physical source ownership on typed analysis-card errors through root/scoped deferred binding, nested includes and continuations; preserved typed issues and SDK diagnostics | `core-analysis-origins-windows-20261006.json` |
| `965e40fbd`, `846e41b1c` | Shared lexical environments and delayed TEMP/TNOM completion through unfinished parent/root scopes; completed-owner graphs, preserved local sampling phases, shared option/analysis samples and transactional probes | `core-parent-temperature-windows-20261006.json` |
| `fbf46d0fa` | Retry invalid deferred-analysis operands after discovering changed TEMP/TNOM options or later scalar `.TEMP`; preserve first errors, physical precedence, sampled order and bounded replay | `core-provisional-analysis-temperature-windows-20261006.json` |
| `26b578804` | Recover eager parameter domain errors and root declaration/source/model completion errors after temperature discovery; preserve every assignment, first error, demand-bound TEMP dependencies, source origins and cancellation | `core-provisional-parameter-temperature-windows-20261006.json` |
| `4a33fb9c1`, `7580b76fb` | Preserve schedule draw order with syntax-only look-ahead; retain eager option-field errors while reading later physical temperatures, including vector/schedule boundaries and scoped parent bindings | `core-provisional-option-temperature-windows-20261006.json` |
| `ab1a7acbe`, `8525c535e` | Discover independent temperatures after deferred-group failures without publishing partial bindings; preserve scopes, option precedence and statistical phases, with separate bounded discovery and confirmation attempts | `core-pending-temperature-recovery-windows-20261006.json` |
| `a9e5213f2`, `f49179eb4` | Honor DATA branch selection and logical continuations; retain unresolved IF/ELSEIF decisions without choosing a branch until fresh-source temperature confirmation, preserving scopes, statistical phases and physical diagnostics | `core-conditional-temperature-recovery-windows-20261006.json` |
| `b221a9742` | Retain provisional subcircuit-header default failures until fresh-source TEMP/TNOM validation, preserving formal ownership, per-instance expressions, duplicate selection, isolated sampling and terminal errors | `core-subckt-temperature-recovery-windows-20261006.json` |
| `19ff4bfa4` | Share atomic IC/NODESET parsing and recover IC/NODESET/INITCOND failures after fresh-source temperature validation; preserve actual startup behavior, scopes, provenance, source order, sampling and terminal errors | `core-startup-temperature-recovery-windows-20261006.json` |
| `824189732`, `df8a9bd02`, `289e39df2` | Remove per-field token copies and unnecessary constraint searches; share authored numeric binding; complete forward startup operands with ordered publication, source/instance ownership, exact sampling, duplicate/error precedence and cancellation | `core-forward-startup-windows-20261006.json`: 1,393 native and 126 optimized WASM tests, three repaired CLI reproductions, and about 95% less allocation volume for the qualified 256-entry cards |

Continue shared dependency binding in these concrete cases:

- Complete reconciliation of ngspice expressions that are invalid only at the provisional temperature, such as `1/(TEMP-27)`. Deferred analyses, eager `.PARAM` domain errors, eager option fields, pending temperature-option groups, conditional decisions, subcircuit-header defaults, IC/NODESET/INITCOND startup cards and root declaration/source/model completion now participate when source-card parsing reaches completion. Other eager parameter error classes and earlier card failures remain. Preserve every authoritative assignment and conditional decision, typed resource/cancellation failures, original errors and bounded replay; do not guess a branch or substitute successful default values.
- Forward `.IC`, `.NODESET` and `.INITCOND` bindings and per-field token-copy removal are delivered above; the three original CLI reproductions now pass. Continue qualification of physical overrides combined with forward operands and more complex cross-scope/statistical cases. Scoped voltage values must remain per-instance, including X-line-only bindings. The allocation gate covers 64/256 fresh single-ended targets; general constraint-graph complexity, many independent scoped deferred operands, whole-run peak memory and inner-loop cancellation/performance still require work.

General statistical/runtime binding still requires broader qualification, including temperature-dependent sampling, derived-context policies, cancellation inside validation/classification, and allocation/performance bounds. The static global delivery does not close these contracts.

These are reproduced remaining cases, not successful default substitutions. Retain the qualified binding, duplicate-selection, random-draw and diagnostic contracts while implementing them. The local ngspice 46 `src/frontend/inpcom.c` functions `inp_sort_params` and `inp_get_param_level` provide declaration-planning reference evidence; RSpice's explicit definition policies must still be honored.

C01/C04 frequency-grid allocator diagnostics are delivered in `c13b01791`: checked AC, PAC, PXF, STB, transfer and quasi-periodic grids retain the original `TryReserveError`; engine/control and existing CLI/WASM/worker adapters preserve the resource classification instead of reporting invalid input. `0ee0e3778` extends source preservation to STB result/workspace reservations. Cancellation and configured limits remain distinct. Evidence: `core-frequency-grid-errors-windows-20261006.json`; SDK migration guidance is in the core README. Capacity-refusal tests do not establish behavior under actual memory exhaustion. Other allocation owners, broader table configuration/transport qualification and general temperature dependency resolution remain open; no C00–C14 package is closed.

### C05 IMD commit sequence

1. Define checked tone/product identities, amplitude and power references, and explicit measurement availability. Preserve valid existing post-processing APIs and provide migration guidance for changed fields.
2. Add the missing `2*f2-f1` distortion product and qualify both third-order products and second-order products with independent polynomial circuits, exchanged tones, and signed/coincident frequencies.
3. Implement constructors from qualified Volterra, spectrum, and waveform data, reusing existing Fourier/HB/QPSS infrastructure. Qualify window/resolution limits, unequal tones, noise, and dynamic range.
4. Add intercept estimates with documented assumptions and drive-sweep slope/compression checks; expose them through public core and control result routes. Unavailable or invalid estimates remain explicit.

### Remaining architecture, model, and integration sequence

| Order | Packages | Bounded delivery slices |
|---|---|---|
| Throughout | C00, C01, C02 | Finish predicate and auxiliary-route fixtures; establish residual/charge/state contracts as each provider needs them. Extract transient setup, trial/reject, commit, observation, and checkpoint boundaries in separate behavior-preserving commits |
| Native devices | C06 | Diode/JFET restrictions; B3SOI DD, FD, PD; EKV 2.6; EKV3; VDMOS; remaining classic-MOS predicates; GP thermal and phase-aware periodic state. Deliver each family with equations, derivatives, applicable analysis routes, and independent evidence |
| Stateful devices | C07 | Thermal resistors/memristors; controlled/hysteretic switches; scalar/coupled lines; mutual/multi-winding/nonlinear magnetics; capacitor/behavioral dynamic operators. Complete capture, rollback, and continuation with each physical implementation |
| External providers | C09 | Runtime Verilog-A adapters, generated-model adapters, and individual XSPICE model classes. Depend on the existing compiler/runtime workstreams; qualify actual evaluators and state, including allocation failures |
| Noise | C08 | Follow each completed noisy-device/provider slice with orbit-dependent sources, correlations, folding, and independent integrated-power/convergence checks |
| Mixed equilibrium | C10 | Complete cross-domain accepted state and rollback first; then coupled equilibrium and eligible linear analyses. Reuse the existing scheduler and mixed-state contracts |
| Mixed periodic | C11 | Period closure with pending events/digital state; event-time and state-jump derivatives; applicable periodic conversion/noise; envelope continuation |
| Integration | C12 | Accompany every slice with parser/direct/control/result tests and producer-state checks; finish the full public-route matrix after C03–C11 |
| Organization and measured cost | C13 | Remove demonstrated duplicate numerical contracts and improve responsibility boundaries; profile and optimize the remaining measured bottlenecks in separate commits with unchanged physics gates |
| Final acceptance | C14 | Execute the full declared core configuration/reference corpus on committed source, resolve each ignored/omitted check, and publish the completed ledger and generated support matrix |

Before each implementation commit, run the affected regression and contract checks. Broaden checks when a shared numerical/state interface changes. Push each validated, focused commit to remote `main`; keep implementation, structural refactors, tolerance changes, and evidence updates separately reviewable. Do not commit unrelated in-progress work. No package closes until its acceptance criteria above are met.

### Immediate C03 implementation commits

| Batch | Implementation | Required evidence before marking delivered |
|---|---|---|
| C03a: fixed BJT state allocation — delivered | `6b32f5bb3` reserves all 22 fixed lanes and owned UIC solution copies fallibly; transient/restart/PSS/HB callers preserve typed errors; resume skips superseded initialization | Injected failures cover every lane, UIC copies, restart and periodic state atomicity, and successful reuse. Scoped unit, public integration, feature/profile and Clippy evidence is in `core-bjt-initialization-windows-20261005.json`; other owners and caches remain C01 work |
| C03b: record-ceiling diagnostics — delivered | `d01e555b8` preserves structured validation/resource/allocation errors and complete candidate counts through native GP; `948fd90ff` preserves device/allocation context across existing service/worker contracts | Ordinary, sided and known-order records at the real ceiling; pruning; all-participant failure atomicity; public resume and reuse; malformed checkpoint distinction; portable metadata and optimizer fatality. Evidence: `core-delay-errors-windows-20261005.json` |
| C03c: qualification closure — in progress | Accepted-work cancellation, selected native profiles/configurations and transport-quota measurements are delivered in `d4c2a1412`/`bf65aa27a`; `722363fd0` adds nonlinear oracles/event-tail repair; `12ba3605b`/`b0878b1d0` retain executed WebAssembly tests and worker CI. `da18bfa88` adds charge oracles; `50523d8fb` repairs small-charge event rows. `4ec035c19` repairs constant-bias startup. `87ff9cbf9` and `0d6ceb788` repair and activate the driven private matrix originally added in `4fb1105d8`; finish the remaining cases | Evidence: `core-gp-measurements-windows-20261005.json`, `core-gp-regimes-windows-20261005.json`, `core-gp-wasm-windows-20261006.json`, `core-gp-charge-events-windows-20261006.json`, `core-gp-operating-point-windows-20261006.json`, and `core-gp-driven-events-windows-20261006.json`. Full acceptance requires the remaining topology/parameter and continuation accuracy cases, configuration coverage, whole-run allocation/memory limits, inner-solve cancellation and performance gates |
| C03d: physical-event population — in progress | `03f270c65` delivers smooth prescribed behavioral sources; `eed21ead1` and `5619108f7` deliver native VCVS/VCCS event providers and G-current observations. `fc23d131b` and `8fb9d66ad` deliver native CCCS weighted conservation, startup and current observations. Complete CCVS and remaining behavioral expression/state contracts, then native, stateful and external provider slices. Inventory all family and instance guards | Preserve independent source/charge/impulse/continuation tests recorded in `core-gp-behavioral-events-windows-20261006.json`, `core-gp-voltage-controlled-events-windows-20261006.json` and `core-gp-current-controlled-events-windows-20261006.json`. Replace other refusal probes with public numerical tests as providers are delivered; no missing stamp or incomplete accepted state can be counted as support |

Use separate commits for the shared runtime error contract, core propagation, numerical fixes discovered by tests, and qualification/documentation updates. After these batches, reconcile the C03 ledger against its full acceptance criteria before changing its status.

### C03b acceptance requirements (delivered)

1. Add a structured delay-history acceptance error in the shared runtime, preserving validation details and requested/allowed record counts. Build the complete ordinary or sided-event candidate before the existing retention/counting check; do not duplicate pruning logic or infer error kinds from strings. Exercise samples, unknown/known-order events, pruned records, staged VM candidates, and malformed checkpoint distinctions.
2. Preserve that error through native GP preparation, physical startup/events, accepted-step validation, and public core execution. Attach the elaborated instance identity; classify the ceiling as a nonretryable resource refusal. Keep the existing per-site ceiling and transport-byte quota distinct. Update dependent error adapters without declaring runtime VM allocation/transaction work complete.
3. Add core regressions for refusal before any participant commits, including a later failing device after an earlier valid candidate, unchanged accepted history/input checkpoints, and successful engine reuse. Verify both resource and validation diagnostics through public errors. Run affected runtime/core checks and dependent compilation before pushing this batch.
4. Record the tested revision and update the requirements ledger in a separate qualification commit. C03c remains open until its full matrix and measured resource/cancellation/performance gates pass.

## Remaining-work status

This table summarizes the work still required; the package definitions above supply dependencies, implementation requirements, and acceptance gates. A partial delivery does not close its package.

| Package | Current status | Remaining completion work |
|---|---|---|
| C00 | In progress | Complete instance-predicate and auxiliary-route fixtures; record repeatable numerical, memory, performance, and cancellation baselines |
| C01 | In progress | Complete shared residual/charge/derivative/noise contracts and trial/accepted/capture/restore conformance across providers |
| C02 | In progress | Extract remaining transient phases and consolidate state boundaries after the delivered step-proposal and checkpoint-identity preparation boundaries |
| C03 | In progress | Preserve the repaired constant-bias and driven private-node regressions; complete independent Weil feedback and remaining continuation cases, C03d physical-event provider integration, remaining parameter cases, configuration coverage, whole-run memory/allocation, inner-solve cancellation, and broader performance evidence. Native initialization, typed errors, quotas, accepted-work cancellation, initial measurements, prescribed-terminal nonlinear/charge oracles and selected executed WASM configurations are delivered |
| C04 | In progress | DC/nested DC, ordinary NOISE, AC/noise tables and TF have execution, presentation and CLI/WASM publication. Table documents retain coordinates, bindings and completion with bounded, cancellable projection. TF retains physical units and unbounded scalar determinations and supports finite control scalar reads. Implement the 21 remaining dispositions, producer dependencies and bounded orchestration; finish broader binding/configuration/performance qualification |
| C05 | Open | Checked IMD constructors, upper third-order product, tone/power conventions, intercept evidence, and public/control exposure |
| C06 | Open | Missing semiconductor periodic equations, derivatives, shooting histories, and envelope continuation, delivered per family |
| C07 | Open | Thermal, memory, hysteresis, line, magnetic, and dynamic-operator equations with complete continuation state |
| C08 | Open | Periodically modulated and correlated noise for each newly supported family, including folding and convergence evidence |
| C09 | Open | Real external-model evaluator/state adapters and allocation/rollback contracts; native GP storage repairs do not qualify runtime VM acceptance |
| C10 | Open | Complete mixed accepted state, deterministic rollback/restart, coupled equilibrium, and eligible linear analysis |
| C11 | Open | Mixed periodic closure, moving-event derivatives, RF conversion/noise, and envelope continuation |
| C12 | Open | Full parser/direct/control/result route equivalence and authenticated producer/consumer integration |
| C13 | Open | Remaining responsibility-boundary refactors, duplicate numerical-contract removal, and measured performance/memory improvements |
| C14 | Open | Exact committed-source qualification across intended configurations, independent references, boundary fuzzing, and every required ledger row |

## C04c transfer-function control delivery — October 7, 2026

The TF portion of C04c now uses the existing transfer solver for explicit `tf`
and declarative `run`, including executed temperature settings, differential
voltage probes and source identity. Retained runs have stable `tf1`, `tf2`, ...
names. Finite TF scalars support print/vector expressions, scalar assignments,
conditions and loops. Scalar reads preserve lazy `if` evaluation, function
argument scope, random evaluation order and expression dialect behavior.

Two reproduced correctness defects are repaired: transfer gain units now follow
the actual source/output quantities, and cancellation from the final progress
callback prevents successful publication. The public result constructor takes
an explicit gain unit. Exact unbounded impedance determinations remain typed
through core documents, ordered CLI print output and WASM metadata. A scalar
expression that actually reads one reports an error without manufacturing a
finite value. Flat exports retain the gain and impedance units.

The scoped record is
`crates/rspice-core/tests/testdata/qualification/core-control-transfer-function-windows-20261007.json`.
It distinguishes independent circuit equations, existing ngspice oracle
regressions, route equivalence, native development checks and optimized WASM
execution on committed source. No numerical tolerance or performance gate is
relaxed. Cancellation, cumulative limits, failed-publication rollback and SDK
migrations are included in this delivery's regressions.

There are now eight implemented control dispositions and 21 remaining. C04c
still requires PoleZero, Sensitivity, Stb, Sp, Disto and DcMatch; C04d–C04g and
the broader C04 binding, configuration, resource and performance qualification
remain open. All 15 packages retain their existing open/in-progress status.

## C04c pole-zero prerequisites — October 7, 2026

Direct PZ completion and identity are repaired in `fd274f3f7`: both solver
paths honor cancellation from the final progress callback, and results retain
the physical input/output quantities and differential circuit node names.
`b5289f9c9` moves the shared exact constraint kernel beneath analysis owners
without duplicating its equations or storage/cancellation policy.

`de509f0f3` repairs a numerical defect discovered while preparing PZ control:
a differential RC with H(s)=1/(1+0.001*s) previously produced a spurious unstable
pole and zero near +1.14e19. Exact algebraic closure now separates infinite
chains before solving the finite state matrix. Permanent regressions preserve
true fast modes beyond that magnitude, exact infinite multiplicity, higher-index
zeros, row/column permutations, irregular refusal, inner cancellation and typed
workspace limits. Both new regression targets are included in the existing
optimized Firefox worker CI command; local WASM execution used Chrome.
The scoped evidence is `core-pole-zero-descriptor-windows-20261007.json` in the
qualification directory. It records native and optimized WASM execution,
dependent callers, unchanged thresholds and the unresolved Rust 1.94 MIR ICE.

PZ control support remains unimplemented. Before closing that slice, complete:

1. Dense high-frequency gain, including static gain, zero asymptotes and
   improper transfers. The current dense path leaves `hf_gain` absent even
   for a 1 kohm static transimpedance. Use descriptor equations and analytical
   regressions; an arbitrary high-frequency sample is insufficient.
2. Root/gain unit contracts and complex control assignments, conditions,
   scalar/root indexing and presentations without discarding imaginary parts.
3. Explicit `pz` and declarative `run`, immutable retained dataset identities,
   cumulative budgets, failure rollback, and CLI/WASM publication through the
   shared core card runner.
4. Broader option/model, resource and performance evidence on committed source.

These prerequisite repairs do not change the control inventory: eight of 29
dispositions are implemented, 21 remain, and all 15 packages remain open.
