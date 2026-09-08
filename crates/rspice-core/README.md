# rspice-core

The SPICE circuit simulation engine: netlist parsing, device models, sparse
matrix assembly, Newton-Raphson solving, and the analysis algorithms. Every
other crate in the workspace (the CLI, the GUI, the Python bindings, the WASM
bindings, the benchmark rig) is a frontend over this one. It has no I/O
conventions of its own beyond reading netlist text and `.include` files;
output formatting, exit codes, and configuration live in the frontends.

## Architecture

A netlist flows through the engine in stages:

```
netlist text
    │  netlist/    nom-based lexer + parser → Netlist AST
    │              (elements, models, subcircuits, analysis cards,
    │               parameters, .MEAS statements)
    ▼
  Netlist
    │  engine/builder + circuit/   subcircuit flattening, parameter
    │              resolution, device construction
    ▼
 CircuitData   struct-of-arrays storage: one array per device kind
    │  solver/    one symbolic analysis freezes the sparsity pattern;
    │             a position map gives O(1) stamp lookups thereafter
    ▼
 StaticMatrix  sparse CSC matrix + reusable LU workspace
    │  engine/    Newton-Raphson loop with convergence aids,
    │             transient integration, frequency sweeps
    ▼
 SimulationResult / analysis-specific result types
```

The crate-level entry point is `Engine` (re-exported from `engine/`), with
`Netlist::parse` as the front door:

```rust
use rspice_core::{Engine, Netlist};

// The first line of a SPICE deck is the title, never an element.
let netlist = Netlist::parse("divider\nV1 1 0 10\nR1 1 0 1k\n.end")?;
let engine = Engine::default();
let result = engine.run_dc_op(&netlist)?;
assert_eq!(result.voltage(1), 10.0);
```

`SimulationConfig`, `ConvergenceConfig`, `ConvergencePreset`, and
`DampingStrategy` configure the engine; `AbortSignal`/`AtomicAbort` allow a
frontend to cancel a long transient or sweep cooperatively (this is what
backs Ctrl-C in the CLI and `KeyboardInterrupt` in the Python bindings).

## Module map

| Module | Contents |
| :--- | :--- |
| `netlist/` | Lexer, parser (with submodules for element parsing, command parsing, conditionals, scoping, source specs, transmission lines, Laplace synthesis), AST, subcircuit flattener, hierarchical path handling, `.include` resolution, parameter scoping and expressions, multi-run cards, SPEF and XSPICE card parsing |
| `circuit/` | `CircuitData` struct-of-arrays storage (one typed array per device kind in `storage/`), construction, linear stamping, nonlinear device hooks, magnetic coupling, introspection, external-model attachment |
| `device/` | Device model implementations; see [Device models](#device-models) |
| `solver/` | Newton-Raphson (`newton.rs`), convergence checking, damping strategies, and arc-length continuation. Sparse LU itself lives in [`rspice-matrix`](../rspice-matrix) and is re-exported here |
| `engine/` | The orchestrator: DC, AC, transient (`transient/`), harmonic balance (`hb/`), PSS (`pss.rs`, `pss_noise.rs`), stability (`stb.rs`), transfer functions, matrix assembly and stamping, source value evaluation, behavioral-expression hooks, circuit builder, configuration and config resolution, convergence-aid drivers |
| `analysis/` | Analysis algorithms and result types (see [Analyses](#analyses)) plus `.MEAS` evaluation (`measure.rs`, `measure_signals.rs`, `measurements/`), post-processing, and signal-integrity helpers |
| `expr/` | Expression engine for behavioral sources and parameters: parser, AST, bytecode compiler, and a small VM so expressions evaluate cheaply inside the Newton loop |
| `io/` | Result export and ingestion: rawfile export, streaming waveforms, LTspice RAW reading |
| `execution/` | The canonical analysis plan and the shared `rspice-analysis-result` document every frontend publishes |
| `library/` | `.lib` and model-library parsing, a library manager, and filesystem discovery of shipped Verilog-A and SPICE model packs |
| `xspice/` | XSPICE code-model subsystem: `CodeModel` trait, `CodeModelRegistry`, instance and context types, event and digital value types, bundled analog/digital/bridge models, plus an `ifspec.ifs` parser and conformance helpers that diff the registry against an ngspice checkout |
| `numerics/` | Integration companion models, differentiation, and shared numerical kernels |
| `config`, `resource` | `SimulationConfig` and the resource-budget policy every frontend applies |
| `diagnostics`, `identity`, `naming`, `op_label` | Typed diagnostics, canonical analysis and coordinate identity, signal naming, operating-point labelling |
| `constants` | Physical and simulation constants |
| `abort_signal` | `AbortSignal` trait with `AtomicAbort`/`NoAbort` implementations for cancelling long runs |
| `simd/` | SIMD math, reduction, and integration kernels (only with the `simd` feature) |
| `time_compat` | Wall-clock shim: real `std::time::Instant` natively, a no-op stub on `wasm32` (bare WASM has no clock) |

Conformance harnesses are deliberately *not* here. They live in
[`rspice-conformance`](../rspice-conformance), which can see only this crate's
public API, so every assertion travels the path a user's deck takes.

## Device models

Verified against `src/device/`:

**Passives** (`passive/`): resistor, capacitor, inductor, coupled
inductors (`K`), saturable inductor, and a Jiles-Atherton magnetic
hysteresis model.

Linear-inductor geometry is dialect-specific. Under the ngspice and
`BestAvailable` policies a model card may synthesize inductance from `NT` with
`LENGTH` and `DIA` or `CSECT` (optionally `MU`) using ngspice's Lundin/Nagaoka
correction, qualified in `tests/inductor_geometry.rs` against the analytical
impedance. Under the Xyce policy the Xyce Reference Guide 7.10 (section 2.3.5)
defines a linear inductor as a required instance `L` scaled by a dimensionless
model `L`, the temperature polynomial, and multiplicity, with no geometry
parameters at all; RSpice therefore requires the instance value and rejects
`NT`, `LENGTH`, `DIA`, `CSECT`, and `MU` on a Xyce model card even when an
instance `L` is present, rather than silently ignoring an authored geometry.
Nonlinear `CORE` mutual-inductor geometry is a separate contract.

**Semiconductors** (`semiconductor/`): junction diode; BJT (legacy
Gummel-Poon with no `LEVEL` or `LEVEL=1/2`, native VBIC at
`LEVEL=4/9/11/12/13`). Other BJT levels are rejected with a typed error
naming the supported set; advanced CMC bipolar models are reached through
generated Rust from Verilog-A rather than a hand-written path.

### Compact model routing and boundaries

Advanced CMC compact models (HICUM/L0 and L2, MEXTRAM 505 and its
self-heating and diffusion-charge-split variants, PSP, BSIM-CMG, HiSIM, ASM
HEMT and the rest) are delivered by the Verilog-A model program as generated
Rust modules under `../rspice-veriloga-models/models/`, each behind its own
`veriloga-model-*` feature. This crate owns three things for them and nothing
else: routing a `.MODEL` card to the right module, the result identity the
routed instance produces, and how the analysis-capability descriptors answer
for it. It never carries a hand-written approximation of their equations.

A card naming one of those families therefore has exactly two outcomes. If the
module is compiled in, the card routes to it and no native device is created
alongside it. If it is not, the build is refused with a typed error naming the
compact-model family, the generated module the routing layer would have
selected, and the feature that supplies it. There is never a fall-back to the
native Gummel-Poon or VBIC equations, which are a different model, and never a
name-only route that fails later inside the solver. Level selectors for those
families (`Q LEVEL=8/23/230/234/504/505`) stay rejected in every build: a level
names a *dialect's* device rather than a module, and routing one to a
particular generated artifact would be a guess.

For the analyses, a routed generated instance participates in DC, AC,
transient and noise, and is refused by capability for the periodic analyses:
it declares no exact periodic MNA descriptor and no captured period-map
integration state, so harmonic balance, PAC, periodic noise and PSS
continuation reject it by name (see `engine/periodic_capability.rs`).

**MOSFETs and FET-family models** (`mosfet/`):

- Classic Berkeley MOS1/MOS2/MOS3/MOS6 at `LEVEL=1/2/3/6` (`classic.rs` and
  `classic/`, with the shared parameter bundle in `mos_models.rs`)
- Legacy BSIM1/BSIM2 at `LEVEL=4/5` (`legacy_bsim.rs`)
- MOS9 at `LEVEL=9`, which is also where Xyce-style BSIM3 cards land: a
  decisive BSIM3 parameter signature routes to BSIM3v3, and an
  ngspice-shaped card stays MOS9
- BSIM3v3 at `LEVEL=8/49` (`bsim3v3.rs`, `bsim3v3/`: params/temp/eval split)
- BSIM4 v4.8 at `LEVEL=14/54` (`bsim4v8.rs`, `bsim4v8/`)
- EKV 2.6 at `LEVEL=260` (`ekv.rs`), plus a narrow native EKV3 `LEVEL=301`
  slice (`ekv3.rs`) covering the VA-Models/Xyce 150 nm NMOS/PMOS cards;
  other EKV3 cards fail closed in the builder. The complete EKV3 302.00 model
  is the generated `ekv3_rf` device (`veriloga-model-ekv3-rf`), reached by
  module name on an `X` line rather than by a `LEVEL` selector
- VDMOS power MOSFET at `LEVEL=18` (`vdmos/`: device, recovery, thermal
  submodules)
- B3SOI silicon-on-insulator at `LEVEL=10/55/56/57`, in DD/FD/PD variants
  (`b3soi/dd`, `b3soi/fd`, `b3soi/pd`)
- JFET level 1 and native Parker-Skellern JFET2 (`jfet/`, `jfet.rs`;
  `NJF`/`PJF LEVEL=2`) with ngspice-compatible `P`, `Q`, `XI`, `Z`,
  `VST`, `MVST`, `MXI`, `LFGAM`, `LFG1`, `LFG2`, `HFGAM`, `HFG1`,
  `HFG2`, `HFETA`, `HFE1`, `HFE2`, `TAUG`, `TAUD`, `DELTA`, `ACGAM`,
  `XC`, `CDS`, `IBD`, `VBD`, and `VER` model parameters plus the common
  JFET aliases such as `VT0`/`VTO` and `VBI`/`PB`. `SimulationConfig`
  defaults to best-available Parker-Skellern behavior, while
  `SpiceDialect::Xyce` selects the internal Xyce modified-Shockley JFET2
  compatibility path for Xyce regression coverage.

`bsim4v8/` is the native BSIM4 v4.8 path for MOS `LEVEL=14/54`, ported from
ngspice-46's `src/spicelib/devices/bsim4/`; those upstream BSIM4 files carry
UC Berkeley BSIM4 / ECL-2.0 terms tracked in the root `NOTICE`. It is wired through the builder,
matrix reservation, nonlinear Newton stamping, AC small-signal stamping, and
transient charge integration; `tests/bsim4_native.rs` pins the engine wiring
against OP, DC sweep, transient, and `LEVEL=54` decks. Implemented: internal
and external bias-dependent S/D resistance (`rdsMod=0/1`), distributed body
and gate-resistance networks (`rbodyMod=0/1/2`, `rgateMod=0/1/2/3`),
transient and AC charge-deficit NQS, `mtrlMod=1` material constants for both
compatibility modes, `capMod=0/1/2` with integer `cvchargeMod=0/1/2/3`,
`mobMod=0..6` (including the high-k/Synopsys variants), `tempMod=0/1/2/3`,
`geoMod=0..10` implicit diffusion geometry, `rgeoMod=1..8` implicit S/D
resistance geometry for omitted `NRD`/`NRS`, `wpemod=1` well-proximity for
`SC` and explicit `SCA`/`SCB`/`SCC` inputs, `igcMod`/`igbMod` gate tunneling
currents, the stress layout correction for active `SA`/`SB` layouts
including the multi-finger `SD` path, and `dioMod=0/1/2` junction diode
selectors.

Selector values outside those ranges are typed errors rather than silent
changes of physics. See `src/device/mosfet/bsim4v8/mod.rs` for the exact
ported/not-ported inventory.

**Sources and behavioral**: independent sources (`sources.rs`), the four
controlled sources E/F/G/H (`controlled.rs`), behavioral B-sources whose
expressions compile through `expr/` (`behavioral.rs`), and PWL-from-file
sources (`pwl_file.rs`).

**Transmission lines**: lossless and lossy lines (`transmission_line.rs` plus
`transmission_line/` with delay, distributed, line, response, and TXL
submodules; the LTRA path is checked by `tests/ltra_ac_oracle.rs`), and coupled
multi-conductor lines (`coupled_transmission_line.rs`, `cpl_native.rs`).

*LTRA shunt conductance.* A scalar LTRA card is classified exactly from its
per-unit-length `R`/`L`/`G`/`C`, because those are physical densities and no
absolute epsilon can decide whether an authored `G` is negligible over an
arbitrary length. Four classes execute: RLC and lossless LC lines (`G = 0`),
RC diffusion lines, the memoryless finite-length RG line (`R > 0`, `G > 0`,
`L = C = 0`), and the `LEN = 0` RC/RG ideal-through special case. An RG line
has no reactance, so its ABCD parameters are real constants and one
frequency-independent two-port describes it in DC, AC, transient and the
periodic analyses; it is admitted wherever a linear resistor is, and it
contributes no noise source of its own, matching both reference simulators.
**RLGC with `G != 0` is rejected.** That is a deliberate boundary, not a gap:
neither ngspice-46 nor Xyce 7.10 implements a lossy line with both shunt
conductance and reactance, so there is no reference semantics to match, and
inventing one would produce numbers no oracle can qualify.

**Memristors**: native Xyce `YMEMRISTOR` families: the TEAM model at
`LEVEL=2` (`memristor_team.rs`) and the threshold-adaptive PEM model at
`LEVEL=4` (`memristor_pem.rs`), both solving an internal state variable
alongside the terminal equations.

**MESFETs and HFETs** (`mosfet/jfet/`): the legacy MESFET at `LEVEL=0/1`, MESA
at `LEVEL=2/3/4`, HFET1 at `LEVEL=5`, and HFET2 at `LEVEL=6`.

**Other**: switches (`switch.rs`) and thermal network elements (`thermal.rs`).
GaN HEMT qualification is feature-gated work through generated Rust from
Verilog-A (ASM-HEMT and MVSG CMC).

**Extension points**: external Verilog-A devices via the `rspice-veriloga`
compiler (`veriloga.rs`, behind the `veriloga` feature, with blake3-keyed
on-disk caching of compiled models) and build-time generated Verilog-A built-ins
(`veriloga_builtins.rs`, materialized as reusable packages under
[`../rspice-veriloga-models/models/`](../rspice-veriloga-models) and
instantiated by model name when the feature is enabled).

## Analyses

Core analyses, driven from `engine/`:

Parameter sweeps use `Engine::plan_step_commands`, `StepPlan`, and
`StepPlanLimits`; axis specifications live in `netlist::StepSweep`.
The unused `analysis::parametric` API has been removed. SDK callers should
migrate to the engine planner, which validates dimensions and total run counts
before executing the same sweep path used by the frontends.

The unused `JunctionTempScaling` and `MosfetTempScaling` placeholders and
the unused `CapacitorTempCoeffs::vc1/vc2` fields have also been removed.
Semiconductor temperature behavior belongs to each device model;
`TemperatureContext` and the passive temperature coefficients remain available.

The callback `MonteCarloRunner::run` now returns `Result`; callers must
handle configuration, entropy, and resource errors. `run_with_abort` adds
cooperative cancellation. Unseeded runs use host entropy on native and WASM
targets and retain the seed and sampling-policy version in the shared result
document. Callback sampling now uses the engine's distribution arithmetic,
including the magnitude of negative nominal parameters; its policy is version 2.

`MonteCarloConfig::confidence_pct` controls a two-sided confidence interval
for each output mean. `confidence_method` selects Student-t (exact for
independent normal observations) or a deterministic percentile bootstrap with
an explicit seed and resampling count. Failed trials make the interval
conditional on successful trials. The shared result document includes the
method, level, assumptions, and each interval's availability; fewer than two
samples have no estimated interval. Bootstrap work and storage use the existing
analysis-point and result-value limits.

| Analysis | Module |
| :--- | :--- |
| DC operating point and DC sweep | `analysis/dc.rs`, `engine/dc.rs` |
| AC small-signal sweep | `analysis/ac.rs`, `engine/ac.rs` |
| Transient | `engine/transient/`; shared results in `analysis/transient.rs` |
| Temperature handling | `analysis/temperature.rs` |
| Laplace-defined sources/filters | `netlist/parser/laplace_synthesis.rs` |

Transient integration methods: backward Euler, trapezoidal, Gear-2, and the
hybrid trap/Gear default (selected via `SimulationConfig`; the CLI exposes
them as `--integration-method euler|trap|gear|trapgear`). Timestep control
is LTE-based with breakpoint handling; a transient checkpoint/resume path
exists (`engine/transient/`, exercised by `tests/transient_checkpoint.rs`
and the CLI's `--checkpoint`/`--resume`).

Checkpoint format 35 retains the step and stop defaults that determine
independent-source waveforms. Extending a run or changing its step ceiling
preserves those source parameters. Older checkpoints remain readable; resume
requires fully specified source timing when the original defaults are absent.
Resume also requires the current resolved simulation identity (v14); states
captured under previous source evaluation or behavioral event timing semantics
must be regenerated.

PWL interpolation and repeat timing preserve finite nonzero knot intervals
and positive `TSCALE` values without an absolute machine-epsilon cutoff.
An exact repeat boundary retains the authored endpoint; the next representable
instant evaluates the next cycle. The source-event enumeration API preserves
distinct authored times instead of applying an integrator landing tolerance.

Integration order is deliberately bounded to 1 and 2. Xyce's documented
`TIMEINT` contract defines its variable-order trapezoidal and Gear methods over
orders 1 and 2 only (Users' Guide 7.10, section 7.3.4), so `.OPTIONS TIMEINT
MINORD`/`MAXORD` accept only those values; any other request, including
inverted bounds, is a typed parse or configuration error, never a silent clamp.
The parser (`netlist/parser/commands.rs`), `SimulationConfig` validation
(`config.rs`), and the transient checkpoint identity enforce the same bound,
and `tests/transient_integration_order.rs` qualifies second-order behaviour
against an ngspice BSIM3 timing oracle. Higher-order Gear/BDF would need the
full history, device-state, checkpoint, stability, and manufactured-solution
qualification program, not a wider option range.

Advanced analyses, all flat under `analysis/`:

| Analysis | Module |
| :--- | :--- |
| Fourier / THD (`.FOUR`) | `fourier.rs` |
| Volterra distortion (`.DISTO`) | `distortion.rs`, `engine/distortion.rs` |
| Noise | `noise.rs`, `engine/noise.rs` |
| Pole-zero | `pole_zero.rs`, `pole_zero/` |
| Sensitivity (DC and AC) | `sensitivity.rs` |
| Transfer function (`.TF`) | `transfer.rs`, `transfer/` |
| Parametric sweep (`.STEP`) | `netlist/ast.rs`, `engine/step.rs` |
| Monte Carlo | `monte_carlo.rs` |
| Process corners | `corner.rs` |
| Periodic steady state (shooting) | `pss/`, `engine/pss.rs` |
| Harmonic balance | `harmonic_balance/`, `engine/hb/` |
| Periodic noise (pnoise) | `pnoise/`, `engine/pss_noise.rs` |
| Periodic AC (PAC) | `pac/` |
| Periodic transfer function (PXF) | `pxf.rs` |
| Stability (STB) loop-gain | `stb.rs`, `engine/stb.rs` |
| Periodic stability (PSTB) | `pstb.rs` |
| S-parameters | `s_param.rs`, `s_param/` |
| `.MEAS` evaluation | `measure.rs`, `measure_signals.rs`, `measurements/` |

How each analysis is reached (netlist card, CLI flag, or engine API only)
varies. The [CLI README](../rspice-cli/README.md) documents the netlist-card
and flag surface; anything not listed there is engine-API only.

### Periodic large-signal cards

`.PSS`, `.PAC`, `.PNOISE` and `.ENVELOPE` are parsed into typed, fully
validated cards in `netlist`. The analysis layer converts a card into the
configuration its entry point takes (`PssConfig::from(&PssCard)`,
`PacConfig::from(&PacCard)`). A parsed deck sits below the analyses and never
names them. Every card is case-insensitive and continues across `+` lines. A
field another simulator accepts here that RSpice cannot honour is refused with
a source-located error rather than parsed and dropped.

**`.PSS`**, shooting periodic steady state. Two disjoint forms; a token
followed by `=` is always a keyword, a token that is not is always
positional, so the two never overlap.

```
.PSS <gfreq> <tstab> <oscnode> <psspoints> <harms> <sciter> [KEY=VALUE ...]
.PSS KEY=VALUE ...
```

The positional field order is ngspice's `.pss` card. It names an oscillator
node, so it selects autonomous period detection. ngspice's trailing
`steadycoeff` and `uic` fields are refused: the shooting solver converges on
a relative periodicity norm rather than an ngspice per-node steady
coefficient, and always starts its stabilization run from the operating
point. Author `TOL=`/`ABSTOL=` and `TSTAB=`/`TSTABPERIODS=` instead. In the
positional form the keywords `FUND`, `PERIODGUESS`, `TSTAB`, `OSCNODE`,
`POINTS`, `HARMS`, `MAXITER` and `AUTONOMOUS` are refused as conflicts,
because the positional fields already bind them.

| Keyword | Positional | Meaning | Default |
| :--- | :--- | :--- | :--- |
| `FUND` | `gfreq` | Fundamental frequency (Hz) | required when driven |
| `HARMS` | `harms` | Harmonics retained in the result | 9 |
| `POINTS` | `psspoints` | Samples per period (≥ 16, ≥ 2·`HARMS`) | 256 |
| `TSTAB` | `tstab` | Stabilization time (s) | 0 |
| `TSTABPERIODS` | keyword only | Stabilization periods when `TSTAB` is 0 | 10 driven, 20 autonomous |
| `MAXITER` | `sciter` | Maximum shooting Newton corrections per integration grid | 100 |
| `TOL` | keyword only | Relative periodicity tolerance | 1e-6 |
| `ABSTOL` | keyword only | Absolute tolerance | 1e-12 |
| `DAMPING` | keyword only | Newton damping in [0.1, 1.0] | 1.0 |
| `MAXPERIODCHANGE` | keyword only | Relative period change bound | 0.1 |
| `AUTONOMOUS` | implied | Detect the period instead of taking `FUND` | FALSE |
| `PERIODGUESS` | from `gfreq` | Autonomous period seed (s) | 1e-9 |
| `OSCNODE` | `oscnode` | Node the period is detected on | none |
| `METHOD` | keyword only | `TRAP`, `GEAR`, `EULER` or `TRAPGEAR` | engine default |
| `VERBOSE` | keyword only | Log convergence progress | FALSE |

`FUND` and `PERIODGUESS` set the same quantity and may not both appear;
`OSCNODE` implies `AUTONOMOUS=TRUE` and conflicts with `AUTONOMOUS=FALSE`.

PSS evaluates independent sources under the selected SPICE dialect. Omitted
source timing uses one configured carrier period as the stop default and
`period / POINTS` as the step default. These defaults remain fixed during
stabilization, shooting and PSS-to-transient continuation; an unrelated `.TRAN`
card does not redefine the PSS drive. Autonomous runs use the configured
period guess for these defaults. Xyce SIN requires an authored frequency and
preserves zero as zero; ngspice SIN resolves zero to the inverse stop default.

Driven PSS certifies the authored periods of every independent and behavioral
source, including RF-port tones. Matching sampled endpoints is insufficient:
a half-integer sinusoid can return to zero with the wrong outgoing slope.
Nonrepeating startup prefixes need an explicit frozen-source selection or a
periodic source specification. Sources that prescribe winding currents also
require a continuous waveform with finite outgoing slopes.

`POINTS` sets the minimum base integration grid. The solver first increases
it beyond the Nyquist limit of recognized source clocks and finite behavioral
trigonometric polynomials, including products and integer powers. Resolved
independent PULSE/PAT edges and physical PWL/file corners are inserted directly
into an immutable, nonuniform integration mesh shared by all shooting and
derivative evaluations. Behavioral PULSE and piecewise-linear table corners use
the same mesh, including affine clocks, signed modulo, and resolved temperature
parameters. Smooth tables and other coordinates with a known time rate retain
their base-grid interval bounds; arbitrary circuit-dependent expressions do not
supply that bound. Known extrema and inverse levels of nonlinear sine/cosine
compositions expose features that could vanish on both initial grids. For
continuous sums, products, regular quotients, powers, SQR, exponential and sine/cosine compositions,
value and normalized-time derivative enclosures of the compiled expression
isolate levels. Nonlinear phase inversion uses those same bounds. Tangential roots are
retained as small feature clusters at expression rounding precision. Uncertain
quotient domains are subdivided until their denominator bounds exclude zero;
unresolved domains report a precision or work-limit error. The VM's exact
zero-denominator rule is preserved for known zero denominators. The
collector limits the number of evaluated instructions as well as its events.
Constants use the shared compiler and VM in the resolved environment.
Quotient bounds use direct division and scaled derivatives to avoid reciprocal
overflow. Power bounds preserve the evaluator's operator and named-function
dialect rules, including varying exponents and Xyce's real projection.
Logarithmic bounds cover `ln`, `log10`, and dialect-dependent `log`, preserving
the evaluator's input floor and locating its derivative corner. Exact clamped
plateaus hide internal source geometry; active logarithmic coordinates retain
their roots and receive the same local interpolation qualification.
Square-root and absolute-value bounds preserve the evaluator's zero clamp and
continuous cusps. Square-root rounding error is bounded across zero without
requiring a finite derivative there.
Multi-argument `min` and `max` bounds retain the possible active slopes, discard
proven inactive branches and preserve continuous corners. Candidate branch
crossings share a bounded isolation budget; excessive pair enumeration fails
before copying operand trees. Duplicate operands do not create crossings.
Time-only compilation reuses an already evaluated stateless sibling from the
VM stack, preserving arithmetic order and each independent integral occurrence.
Shooting solves capacitor corrections from voltage differences and retains
the physical current before adding a small correction to the absolute voltage.
Physical KCL checks use those currents, avoiding cancellation of large Norton
companions on very short event intervals. The fixed source mesh is preserved.
Continuity is tracked separately from derivative bounds; signed zero powers
retain their branch transitions and do not certify a smooth constant map.
Internal source features are discarded only when the complete expression is
proven finite and constant over a nonzero neighborhood. This removes inactive
geometry on exact exponential-underflow plateaus without merging close clocks
or discarding another source's events or authored table and pulse corners.
Comparison and step boundaries are located through the actual behavioral
evaluator between adjacent representable timestamps, including finite equality plateaus after
time zero and comparisons between two time coordinates. Transient continuation
uses the same source feature collector. Refinement preserves source times
exactly. Before shooting, supported time-only sources outside a finite Fourier
band receive local interpolation bounds from the shared value/derivative
interpreter, including propagated VM rounding error. These bounds use the
resolved voltage/current tolerances to concentrate points around narrow
features. Bounds cover hyperbolic functions, their inverses and arctangent,
including real domains and Xyce saturation rules. Exact interval endpoints
preserve valid domain boundaries, and scaled derivative arithmetic avoids
avoidable intermediate overflow and underflow.
Centered mean-value bounds preserve cancellation in compound source
coordinates before subsequent operations amplify range uncertainty. This
numerical mesh supplements physical source events. A smooth
source that cannot meet these bounds at representable time precision returns
a precision error; finite ideal jumps retain their adjacent event clocks. Shooting
then solves successively finer grids, comparing the complete voltage and branch
current waveforms at shared phases, using the engine voltage/current tolerances.
Only a grid that agrees with its refined grid is retained. Adjacent representable
source times remain distinct. Where bisection is impossible, a separately solved
orbit with an alternate integration method must also agree; otherwise the run
reports a time-precision error. Backward Euler crosses these boundaries and
restarts the outgoing integration stencil so unresolvable derivative history
cannot contaminate later steps. The returned sample
count reflects the complete mesh; harmonic capacity is limited by its largest
time interval. Dependent spectral analyses integrate the retained samples without
resampling away local source features. Authored source timing
defaults still use the original `POINTS`. `MAXITER` applies to each grid solve,
while the result reports total Newton corrections across all grids. Point and
memory limits also apply to refinement, and cancellation remains available.
Retained PSS operating points from earlier producer versions must be regenerated
before dependent numerical reuse; the current producer identity is version 27.
This convergence check supplements the shooting residual, which measures closure
of a discrete period map. It is not a proof of resolution for all nonlinear
expressions and devices; independent waveform and
spectrum qualification remains necessary for the circuit being simulated.

Autonomous shooting repeats the quiet source window from zero to the trial
period. Startup kicks must lie outside that entire window, including its
outgoing endpoint; they still excite the circuit during stabilization. A
changing source inside the window requires driven PSS. Transient continuation
starts at time zero and reactivates later authored source events, including
startup kicks and explicitly frozen modulation sources.

**`.PAC`**, periodic small-signal AC around a periodic operating point.
The leading sweep is the input-frequency sweep.

```
.PAC DEC|LIN|OCT <np> <fstart> <fstop> INPUT=<source> OUT=V(node[,ref]) [KEY=VALUE ...]
```

| Keyword | Meaning | Default |
| :--- | :--- | :--- |
| `INPUT` | Small-signal source swept across the sweep | required |
| `OUT` | Output probe, `V(node)` or `V(node,ref)` | required |
| `MAXSIDEBAND` | Symmetric sideband range `-n..=n` | none |
| `SIDEBANDMIN` / `SIDEBANDMAX` | Explicit asymmetric range | -5 / +5 |
| `RELTOL` | Relative tolerance | 1e-3 |
| `ABSTOL` | Absolute tolerance (A) | 1e-12 |
| `FROM` | `PSS` or `HB`: which upstream to linearize around | nearest preceding |

`MAXSIDEBAND` and `SIDEBANDMIN`/`SIDEBANDMAX` are two spellings of one range
and may not be combined.

**`.PNOISE`**, periodic (cyclostationary) noise. The leading sweep is the
offset-frequency sweep.

```
.PNOISE DEC|LIN|OCT <np> <fstart> <fstop> OUT=V(node[,ref]) [KEY=VALUE ...]
```

| Keyword | Meaning | Default |
| :--- | :--- | :--- |
| `OUT` | Output probe, `V(node)` or `V(node,ref)` | required |
| `INPUT` | Source for input-referred noise | none |
| `MAXSIDEBAND` | Folded sideband bound `-n..=n` | 6 |
| `FROM` | `PSS` or `HB` | nearest preceding |

**`.ENVELOPE`**, harmonic-balance envelope continuation. It attaches to the
nearest preceding `.HB` and exposes only what the continuation executes.

```
.ENVELOPE TSTOP=<seconds> [MAXSTEP=<seconds>] [FREEZE=(<source>[,<source>...])]
```

`MAXSTEP` defaults to `TSTOP/50`, as the direct continuation entry point
does. `FREEZE` names the independent sources held at their exact time-zero
values during the carrier solve; a single source may be written without
parentheses, and a repeated source is refused.

`.PAC`, `.PNOISE` and `.ENVELOPE` each consume the periodic operating point
of an upstream analysis. Planning binds each of them to the concrete
upstream instance (`pss-001`, `hb-002`, …) and refuses a card whose upstream
the deck does not author before it.

## Solvers and convergence

- **Sparse LU**: the real-valued path defaults to
  [`rspice-matrix`](../rspice-matrix)'s KLU-class backend, whose stored pivots
  make refactorization on the frozen sparsity pattern cheap;
  `RSPICE_SOLVER=faer` opts back into the faer solver. faer also provides the
  complex solves for AC-family analyses, with parallel factorization when the
  `faer-parallel` feature is on.
- **Newton-Raphson** with voltage/residual/charge tolerance checks
  (`solver/newton.rs`, `solver/convergence.rs`).
- **Convergence aids**, attempted when plain Newton fails: GMIN stepping,
  source stepping, pseudo-transient continuation
  (`engine/convergence/`), and arc-length continuation
  (`solver/arc_length.rs`). `ConvergencePreset` bundles them as
  `fast`/`default`/`robust`.
- **Damping strategies** (`solver/damping.rs`): voltage limiting, line
  search, and combinations, selectable via `DampingStrategy`.

## Feature flags

| Feature | Default | Effect |
| :--- | :--- | :--- |
| `faer-parallel` | yes | Adds faer's rayon feature for parallel sparse factorization |
| `parallel` | yes | rayon and portable-atomic for the parallel sweep, transient, and noise paths in `engine/` and `circuit/` |
| `simd` | yes | `wide`-based SIMD kernels in `simd/`, consumed by the Newton loop |
| `veriloga` | no | Verilog-A device support via `rspice-veriloga`, plus `dirs` for the compiled-model cache |
| `veriloga-native` | no | RSpice-owned native JIT for Verilog-A devices; requested native mode is full JIT or typed construction error |
| `veriloga-wasm-jit` | no | Browser JIT. The owning Web Worker compiles and instantiates the module; the core keeps the authenticated artifacts and the synchronous solver integration |
| `veriloga-builtins-base` | no | The runtime integration every generated model shares, without selecting any model |
| `veriloga-model-*` | no | Compiles one checked-in generated Verilog-A model and the shared runtime. Prefer these granular features in production to minimize compile time, peak rustc memory, and binary size |
| `veriloga-builtins-noise` | no | Adds generated noise schedules to whichever `veriloga-model-*` features are selected |
| `veriloga-builtins-models` | no | Enables every checked-in generated Verilog-A model without the optional noise schedules |
| `veriloga-builtins` | no | Backwards-compatible umbrella that enables every generated model plus noise. Each model is a reusable artifact under `../rspice-veriloga-models/models/`; refresh with `cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- regenerate-builtins` and validate with `check-builtins` |
| `wasm` | no | wasm-bindgen, so the crate builds on `wasm32-unknown-unknown`; used by `rspice-wasm` and the UI's wasm target, which also set `default-features = false` to drop rayon and SIMD |

The defaults mean the CLI, Python bindings, and the standard test run all
exercise the parallel + SIMD paths.

## Building and testing

```bash
# Build (library only)
cargo build -p rspice-core

# Full test suite: 184 integration test files under tests/
cargo test -p rspice-core

# With Verilog-A device tests (veriloga_*.rs oracle tests need the JIT)
cargo test -p rspice-core --features veriloga-native

# Generated Verilog-A built-in runtime checks (feature-gated)
cargo test -p rspice-core --features veriloga-builtins --test generated_veriloga_runtime

# Production-sized generated model build, with noise only when required
cargo build -p rspice-core --features veriloga-model-vbic13
cargo build -p rspice-core --features veriloga-model-vbic13,veriloga-builtins-noise

# Complete model catalog without compiling the optional noise schedules
cargo build -p rspice-core --features veriloga-builtins-models
```

The solver-kernel micro-benchmark (analyze/factor/refactor/solve in isolation)
lives with the rest of the benchmark rig, and measures `rspice-matrix` directly
rather than through this crate's re-export:

```bash
cargo run --release -p rspice-bench -- klu
```

Library unit tests are excluded from the default package test target by
`[lib] test = false`; run them explicitly with `cargo test -p rspice-core
--lib`. Doctests are off as well (`[lib] doctest = false`), so examples in
rustdoc are checked by review rather than by `cargo test`.

The integration suite in `tests/` includes oracle tests that pin device and
analysis behavior to reference values (diode rectifier, VBIC excess phase, LTRA
AC, native BSIM4), RF-analysis tests (HB Jacobian, Krylov and varactor, PSS
shooting, pnoise folding, PAC conversion, STB loop gain), parser robustness
tests, and a determinism test.

### Conformance and benchmarking live elsewhere

The ngspice, Xyce, GF180MCU, ISCAS85, Verilog-A, and digital-Verilog suites are
[`rspice-conformance`](../rspice-conformance), along with the case-runner and
oracle-capture binaries they drive. This crate declares no binaries at all, and
`tools/ci/test_ci_configuration.py` asserts it holds no validation harness.

For whole-process performance comparison against ngspice, see
[rspice-bench](../rspice-bench/README.md).

Licensed under the [RSpice Personal Use License](../../LICENSE).
