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

With the `veriloga` feature, a deck can select a named Verilog-AMS connection
configuration from its active `.VERILOGA` sources:

```spice
.veriloga "models.vams" DRIVER module=driver
.veriloga "connections.vams" Connections
.options connectrules=LowVoltage connectrules_source=Connections
```

`LowVoltage` is a case-sensitive `connectrules` identifier. One configuration
is selected for the whole deck. Without an explicit selection, a single block
is implicit; multiple blocks require a choice. An unknown or multiply declared
selected name reports the source closures and declaration offsets. File,
cached, and registered virtual models use the same selection path. Changing
the option reselects connections while allowing unchanged device code to remain
cached. The option also participates in the netlist's configuration fingerprint.
The optional `connectrules_source` qualifier names an explicit `.VERILOGA`
import alias, compared case-insensitively as a SPICE name. It restricts the
selection to that source, resolving duplicate rule names in different libraries.
Unknown aliases or an alias identifying multiple sources are errors. A qualified
source with no rule block is also an error. The qualifier uses the source already
resolved for the import and survives remapping to a sealed virtual source key.
An explicitly selected empty block supplies no insertion rules; a mixed boundary
requiring one is an error. Hierarchical library/view configuration
binding, product workflows for standalone virtual libraries, and execution of
arbitrary authored connect bodies remain implementation work.

The public `register_project_veriloga_sources_for_session` API accepts a single
transaction of `ProjectVerilogASourceRegistration::Runtime` and `::Connections`
entries. A connection entry carries the compiler's `ConnectionLibraryArtifact`
under an exact sealed source key, with no dummy device module. Import that key
with `.VERILOGA` and select its rules normally. Both entry kinds share one bounded
cache and the same key/alias collision checks; failed validation or aggregate
budgets preserve the installed set. The `_with_limits` variant also bounds
expanded connection source. Existing runtime-only registration APIs use the
same transaction path. A connection-library key cannot be selected as a device
or replaced by a conflicting device registration.

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
[`rspice-conformance`](../../tools/rspice-conformance), which can see only this crate's
public API, so every assertion travels the path a user's deck takes.

## Device models

Verified against `src/device/`:

**Passives** (`passive/`): resistor, capacitor, inductor, coupled
inductors (`K`), saturable inductor, and a Jiles-Atherton magnetic
hysteresis model.

The programmatic `MultiWindingTransformer` API owns immutable winding and
coupling arrays so its cached flux matrix cannot become stale. `new` now
returns `Result`, validates the complete matrix shape and finite symmetric
coupling data, and rejects nonpositive self-inductances. Read access uses
`nodes()`, `inductances()`, `coupling_matrix()`, `branches()`, and
`num_windings()`. `set_branches` and `set_initial_current` also return
`Result`; invalid assignments preserve the previous binding and history.

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

`CORE LEVEL=1/2` model coefficients `TC1` and `TC2` scale every winding's
effective turns by `1 + TC1*(T-TNOM) + TC2*(T-TNOM)^2`. Both the material
field and the full self/mutual inductance matrix use those effective turns.
Model `TNOM` takes precedence over the global nominal temperature (default
27 C); authored temperatures are Celsius and the engine configuration uses
Kelvin. Expressions for these thermal parameters use the analysis temperature.
Each build starts from the authored turns, so repeated temperature runs do
not accumulate scaling. Coefficients must be finite and the resulting turns
factor must remain positive; invalid domains identify the CORE model.

For `CORE LEVEL=2`, AC, noise and pole-zero analysis use the material
susceptibility at the DC winding bias, including all windings' ampere-turns
and the authored gap. The small-signal winding matrix is
`mu0 * AREA / PATH * Ni * Nj * (1 + (1 - GAP/PATH) * P)`.
The nonlinear K-card scalar follows Xyce 7.10's unity coupling behavior;
ordinary linear K elements retain their authored coupling coefficient.

RSpice converges the provisional DC material update before linearization.
In a fresh core this solves `M = Happ * P(Happ, M)` with the DC voltage
direction equal to zero. Xyce 7.10 advances that provisional update once per
device evaluation, so its biased AC results can vary with the number of DC
iterations, including iterations caused by an electrically unrelated circuit.
RSpice intentionally uses the converged constitutive initialization rather
than reproducing that evaluation-count dependence. Undefined or unconverged
material tangents and nonfinite coefficients report the owning core as an
error. Small-signal evaluation does not advance accepted magnetic history or
change the transient update and reset contract. This LEVEL=2 qualification
does not establish LEVEL=1 small-signal or periodic-analysis support.

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

Disk model includes accept `.va filename [MODELNAME] [module=MODULE]` (or
`.veriloga`). `MODULE` selects a Verilog module by its exact, case-sensitive name;
`MODELNAME` is an independent SPICE model alias. For example, two devices from
one source can be loaded with `.va devices.va fast module=FastDevice` and
`.va devices.va slow module=SlowDevice`, then instantiated as `X1 p n fast` and
`X2 p n slow`. Without `module=`, a disk source must declare exactly one device
module, which can be instantiated by its declared name even when that name
differs from the source filename. Connect-only libraries can be included without
a device selection.
Explicit aliases take precedence over module names, which take precedence over
file stems. Conflicting bindings at the same priority are rejected when used;
identical compiled models may share a name. An authored include takes precedence
over a generated built-in with the same name.

## Analyses

Core analyses, driven from `engine/`:

Checked frequency-grid APIs retain the original `TryReserveError` in
`FrequencyGridError::Allocation { requested, source }`. `Error::source()`
exposes that cause, including through STB and PXF wrappers. Conversion into
`SimulationError` preserves the `allocation_failed` resource classification;
configured point limits and cancellation remain distinct. Malformed sweep
diagnostics retain their analysis or input context.

STB result and workspace reservations also retain their allocator cause in
`StbAnalysisError::Allocation { object, requested, source }` or
`SimulationError::Allocation { object, source }`. Callers can inspect the
failed buffer's identity and the error chain without parsing diagnostic text.

SDK migration: `FrequencyGridError` and `StbAnalysisError` implement `Clone`
and are no longer `Copy`. Borrow or explicitly clone errors when reusing them;
match allocation variants with `..` when their cause is not needed. These
checked APIs report allocation refusal, while the legacy infallible
`ac_sweep_frequencies` wrapper still returns an empty vector on failure.

Parameter sweeps use `Engine::plan_step_commands`, `StepPlan`, and
`StepPlanLimits`; axis specifications live in `netlist::StepSweep`.
The unused `analysis::parametric` API has been removed. SDK callers should
migrate to the engine planner, which validates dimensions and total run counts
before executing the same sweep path used by the frontends.

Temperature options accept expressions using parameters available at the option
card, or scalar parameters declared later in the same or an enclosing lexical
scope. For example, `.options temp={ambient}` can precede `.param ambient=85`,
including when a child option closes before that parent declaration.
Already-bound values and functions keep their option-card meaning. Forward
expressions resolve after their own scope and the required declaration owners
close, in authored order within each scope. Local-only options keep their scope
closure sampling phase; delayed child groups retry in scope-close order when
an ancestor closes. An unresolved parent graph uses that parent's completed
definitions, independently of child overrides.
An unsuccessful forward-reference probe consumes no retained random draws.
Later option assignments still win, and superseded assignments are validated.
The parser reconciles the selected temperature with earlier eager expressions
through bounded replay of the same immutable source and statistical seed.
Inactive branches and text after `.END` cannot author temperature options.
`TEMP`, `TEMPER`, and `VT` agree with the selected physical temperature;
`TNOM` defaults to 27 C without adding a stored parameter binding. A physical
table/study `TEMP` coordinate takes precedence over options and a single `.TEMP`
directive. Temperature selections that keep changing on replay are rejected.

Retained ordinary/global parameter chains resolve on demand for temperature
options, including subcircuit scopes. Numeric evaluation preserves complex
values, function-argument shadowing and lazy branches. Shared dependencies
materialize once, so repeated references do not resample statistical parameters.
Deferred child options, parent options and later analysis cards share those
owner samples. A failed or cancelled scope probe publishes no options or cache.
Static parameter validation uses the numeric expression language, including
complex functions such as `IMG`.

Both expression dialects retain ordinary forward `.PARAM` declarations,
including bare and signed aliases. Failed probes preserve the random stream;
selected definitions retain their source location, and duplicate selection and
diagnostic policies still apply. Ordinary static declarations and global numeric
projections finalize before deferred source specifications and model expressions
read them, so sources, models, and their parameters share one sample.
Ngspice ordinary parameters remain static; Xyce runtime expressions remain
symbolic where required.

Global expression bodies remain available for runtime binding and derived
contexts. Deterministic projections follow resolved dependencies, while captured
static statistical values are reused. Sample detection includes user-function
bodies and two-argument `LIMIT`; three-argument clipping remains deterministic.
Ordinary bindings still shadow globals without replacing their namespace.
Available-parameter materialization uses the same complex numeric resolver, so
model expressions such as `IMG(global_value)` retain their imaginary input.

Independent-source specifications containing function calls are probed using an
isolated random stream before live evaluation. Failed forward-binding attempts
and deferral classification consume no live draws, including when a sampled
field precedes a missing field. Deferred root-source errors retain their physical
card location. Grouped runtime-dependent sources retain behavioral evaluation.

Analysis cards stage their result, diagnostics, output requests, Monte Carlo
source identity and transient-noise selection before changing parser state.
The whole card must validate before publication, including trailing fields
after `.AC DATA=...`. Function-bearing cards use the same
isolated sampling probe as independent sources; failed card probes retain no
live draws, and successful fields evaluate once on the live stream. Optional
numeric readers leave failed expressions for validation instead of silently
omitting authored fields.

A deferred analysis that fails at the provisional temperature retains its error
while the parser discovers any later `.TEMP` selection. If options or that
directive select a different temperature, a fresh pass must validate the whole
deck before publication. The original error remains fatal at a stable setting;
cancellation and resource errors return immediately. Successful passes retain
their three-attempt consistency limit. Failed discovery has its own
three-attempt limit because an invalid operand can skip later random draws;
either limit stops replay, for at most five total attempts. Publication still
requires a complete pass at the same selected temperature. Physical study
overrides and statistical order are preserved.

Typed `AnalysisCardError` values retain the physical card origin in `origin`.
Both their `source_location()` method and `ParseError::source_location()` expose
the included or root path and source-local line, including deferred cards and
continuations. The typed card and issue remain available for programmatic handling.
In-memory errors keep line-only locations. SDK callers constructing these errors
should use `AnalysisCardError::new`; existing struct literals need the added
`origin` field (`None` when no physical owner is known).

Top-level analysis cards that cannot bind immediately retry their original
grammar after root declarations, source specifications and model expressions
have completed. Already available scalar/complex values, strings, user functions
and temperature builtins retain their authored bindings; missing bindings use
the completed root scope. Ready cards retain their eager values and random draws.
Pending cards draw in authored order in the completion phase. A shared resolved
statistical parameter is reused rather than sampled by each consumer.

An ordered parser plan merges primary analyses, `.LIN`/`.FFT` auxiliaries,
Fourier/FFT outputs, Monte Carlo source identities and diagnostics back into
their original positions. LIN uniqueness and transient-noise consistency are
checked in authored order, including when an earlier card was pending. Root
cards do not inherit child-local bindings.

Deferred subcircuit analysis operands resolve only demanded dependencies in
their declaring scopes, including later parent/root declarations and unresolved
header defaults. Siblings remain isolated. Known inherited values retain their
snapshot when a parent is later redefined. Suspended numeric operands preserve
lazy branches and random-draw order across grammar retries; shared dependencies
sample once in their owner. Original subcircuit bodies remain available for
per-instance overrides and evaluation. Only scopes needed by pending cards are
retained beyond `.ENDS`.

Eager parameter domain errors and root declaration/source/model completion
errors also participate in temperature reconciliation. Failed declarations
retain their expressions without numeric placeholders, and every later assignment
on the same card is still parsed. A later redefinition cannot erase the original
error. Pending `.TEMP` dependencies can resolve through their declaration owners
when ordinary parameter finalization is incomplete. No failed pass publishes a
netlist, and cancellation and resource failures remain terminal.

Eager option fields use the same failed-pass error contract. Scalar fields,
package transitions, time-point vectors and output/restart schedules retain
their grammar boundaries while later fields select TEMP/TNOM. A fresh pass must
validate every field; later assignments cannot erase an earlier error. Option
overlays applied to an already materialized circuit keep their atomic error
contract. Schedule look-ahead classifies tokens without evaluating operands,
so only actual value reads consume statistical draws.

A failed deferred temperature-option group also probes its later operands.
Successful isolated values are retry candidates, not published options or
parameter bindings; probes consume no live random draws. Groups still wait for
unfinished declaration owners, and assignment order controls candidate
precedence across scopes. Ordinary failures at `.ENDS` retain their physical
card error while later independent source cards can select temperatures. Every
assignment must validate in a fresh pass before the circuit is returned.

Failed `.IF`/`.ELSEIF` expressions leave their chain unresolved for the
current pass. No branch of that chain emits cards or evaluates later decisions,
but nesting and branch structure remain checked. Independent later temperature
declarations can trigger a fresh pass; every active decision must then evaluate
successfully. Inactive `.DATA` blocks neither evaluate rows nor publish tables,
and their continuations remain attached to their original logical card.

Subcircuit headers retain default-expression failures after exhausting forward
progress among their defaults. Unresolved defaults stay symbolic, with no
numeric fallback. Later temperature declarations can trigger a fresh pass;
selected defaults must then validate even when unused or overridden by an
instance. Formal ownership, per-instance expressions, duplicate-selection
policy and isolated default sampling remain intact. A dependent undefined name
does not hide another default's concrete domain error.

`.IC` and `.NODESET` use one parser that stages a complete card in the existing
startup-entry representation. Failed cards publish neither partial execution
values nor provenance records. These cards and inline `.INITCOND` retain their
ordinary failures while later declarations select temperatures; a fresh pass
must validate every active card. Scoped expressions, startup identities,
duplicate rules, statistical phases and typed device-condition errors remain
intact. Terminal resource failures and cancellation still stop immediately.

Root `.IC`, `.NODESET` and `.INITCOND` operands now accept forward parameter
references. Startup and analysis cards share captured authored bindings and the
lexical dependency resolver. Pending startup cards bind after declaration,
source and model completion, before pending analyses, and publish in authored
order. Already-known values and functions retain their source-order meaning.
Scoped voltage expressions still evaluate with each instance's parameters,
including values supplied only by an X-line. Device startup values retain their
authored lexical scope, and duplicate `.INITCOND` cards retain typed errors.

Long startup cards use cursor checkpoints instead of copying all tokens for
every target/value. Constraint validation skips path searches when an endpoint
has not entered the graph. `tests/startup_card_allocations.rs` enforces parser
allocation budgets for IC/NODESET cards.

General dependency planning remains incomplete. Other earlier card failures and
eager parameter error classes can still prevent temperature discovery. Further
physical-override combinations, statistical/runtime binding, general graph and
binding complexity, and broader resource behavior still require qualification.

Control scalar evaluation preserves both real and imaginary components through
`let`, `$name`/`$&name` substitution and print samples. Conditions are true when
either component is nonzero, including subnormal values. Repeat counts, circuit
alterations, thread counts, plot limits and numeric control options require real
values; use `real()`, `imag()` or `mag()` to request a projection explicitly.
Nonfinite components are rejected before scalar assignment or publication.
SDK hosts implementing `ControlScalarEvaluator::evaluate_scalar` must now return
`ComplexValue`; real hosts can construct it with `ComplexValue::from(value)`.

Analysis-card numeric operands require finite real values in both direct and
control execution. Required fields, optional fields and deferred parameter
bindings all apply the same check. Complex parameter arithmetic is still valid
when its result is real, or when an explicit `real()`, `imag()` or `mag()`
projection is used. Probe and table names are unaffected. Monte Carlo integer
fields retain exact 64-bit literal values; numeric expressions are evaluated
before their integer domain is checked.
AC, noise, SP, distortion and AC sensitivity point counts require positive
integers within the target platform's `usize` range. Literal counts preserve
their full precision; fractional and oversized values are rejected before a
cast can truncate or saturate them. Periodic count fields likewise exclude the
first integer beyond the platform range.

`ControlCircuit` executes `ac DATA=<table>` and `noise ... DATA=<table>` through
the shared compact table runners. `ControlAnalysisResult::AcTable` and
`NoiseTable` retain authored columns, canonical targets, accepted row coordinates,
requested row count and model-finish metadata. Presentation exposes column names
and preserves complex samples and noise units. Cross-dataset arithmetic requires
matching canonical targets and every coordinate, independent of column order.
Repeated or decreasing frequencies remain in their authored order.

Physical `TEMP` columns override the caller's resolved run temperature. Otherwise
resolved callers retain their policies; unresolved callers resolve each row's
authored options and executed control overrides. Only options explicitly changed
by control commands survive as overrides during source replay. Circuit equations
and noise densities use the same temperature; the noise fallback applies to
unresolved rows without a temperature option. Version 11 result documents retain
table bindings, aligned coordinate axes, requested rows and model-finish evidence.
The bounded `from_ac_table_with_limits_and_abort` and
`from_noise_table_with_limits_and_abort` document constructors validate borrowed
coordinates and admit projected numeric storage before copying it. They preserve
cancellation during projection and count completion metadata and aligned noise
contributions. Finish metadata rejects unknown fields at every analysis stage.
The CLI and WASM direct/control adapters use these documents and compact table
runners; browser metadata exposes the bindings and bounded windows carry every
coordinate. CLI flat exports retain coordinates as `data(column)` columns; typed
JSON additionally retains binding and completion metadata. Noise band totals
require at least two strictly increasing frequencies with all other coordinates
constant.
Textual multi-run expansion preserves tables needed by frequency/control consumers,
including shared tables and ALTER variants. Regression coverage is in
`tests/control_frequency_data.rs`.

`ControlCircuit` also executes explicit `tf` and declarative `.TF` through the
ordinary transfer-function solver. `ControlAnalysisResult::TransferFunction`
retains the gain, physical gain unit, impedances and source/probe identities.
Named `tf1`, `tf2`, ... datasets expose `transfer_function`/`transfer_gain`,
`input_impedance` and `output_impedance`, including the ngspice impedance labels.
Finite expressions use the existing one-point vector presentation contract.
Exact infinite impedances requested by `print` are retained as typed
`ControlPresentation::scalars`, with `ControlScalar::position` preserving order
among scalar entries and vector traces. They are never converted into finite
plot samples. Hosts must publish these scalar determinations as well as `kind`.
Failed or cancelled analyses publish no dataset and consume no ordinal; the
three retained transfer values count against the cumulative result allowance.
Control scalar assignments and conditions can read these parameter-style names,
including `let gain = tf1.transfer_function`. Reads use the same lazy expression
evaluation and user-function scoping as ordinary parameters. An active read of
an infinite impedance returns an expression error, rather than a finite overflow
sentinel; an unselected `if` branch does not read it.

Explicit `pz` commands and declarative `.PZ` cards executed by `run` use the
shared PZ card runner. `ControlAnalysisResult::PoleZero` retains root evidence,
physical port names and gains; SDK hosts matching this enum must handle the new
variant. Immutable `pz1`, `pz2`, ... datasets support `pole(index)`, `zero(index)`,
`dc_gain` and `hf_gain`/`high_frequency_gain`, with optional dataset qualification
such as `pz1.pole(2)`. Root indices are one-based real integers. Roots retain
both complex components in rad/s; gains retain their physical units. Missing
roots and gains with no finite value produce errors when evaluated. Scalar
assignments and conditions retain lazy branches, user-function scopes and
random-draw order. `print`, complex-plane `plot` expressions using `real()` and
`imag()`, and `settype` use the same retained roots. CLI and WASM publication
preserve the ordinary PZ result document. Cancelled or failed analyses publish
no new dataset or ordinal, and retained roots consume the session's cumulative
result allowance.

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
`RELTOL` and frontend relative-tolerance overrides preserve the independent
absolute voltage/current tolerances, including the legacy absolute voltage
fallback when `VNTOL` has not been specified.

Checkpoint format 35 retains the step and stop defaults that determine
independent-source waveforms. Extending a run or changing its step ceiling
preserves those source parameters. Older checkpoints remain readable; resume
requires fully specified source timing when the original defaults are absent.
Resume also requires the current resolved simulation identity (v16); states
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
| DC mismatch variance (`.DCMATCH`) | `dcmatch.rs`, `engine/dcmatch.rs` |
| Parametric sweep (`.STEP`) | `netlist/ast.rs`, `engine/step.rs` |
| Monte Carlo | `monte_carlo.rs` |
| Process corners | `corner.rs` |
| Periodic steady state (shooting) | `pss/`, `engine/pss.rs` |
| Harmonic balance | `harmonic_balance/`, `engine/hb/` |
| Periodic noise (pnoise) | `engine/hb/pnoise.rs`, `engine/pss_noise.rs`; spectral results in `pnoise/` |
| Periodic AC (PAC) | `pac/` |
| Periodic transfer function (PXF) | `pxf.rs` |
| Stability (STB) loop-gain | `stb.rs`, `engine/stb.rs` |
| Periodic stability (PSTB) | `pstb.rs` |
| S-parameters | `s_param.rs`, `s_param/` |
| `.MEAS` evaluation | `measure.rs`, `measure_signals.rs`, `measurements/` |

Transfer-function results retain `gain_unit`: voltage/current gain is in ohms,
current/voltage gain is in siemens, and equal-quantity gains are dimensionless.
The engine determines these units from the elaborated input source and output
probe, including hierarchical sources. Typed documents preserve that unit.
SDK migration: `TransferFunctionResult::new` now takes
the explicit `SignalUnit` immediately after `gain`; struct literals must supply
`gain_unit`. Do not infer a source's quantity from its hierarchical name.

Computed pole-zero results populate `hf_gain` with the finite high-frequency
limit, including zero and static gains. Exact descriptor equations combine
the selected input and output before classifying improper transfers; `None`
means a nonzero positive power of frequency remains. A finite limit that
overflows binary64 or loses a nonzero value to underflow returns
`PoleZeroAnalysisError::UnrepresentableGain`. It is not replaced with zero or
classified as unbounded. Exact preparation retains cancellation and configured
workspace limits. This contract applies to newly computed results; manually
constructed or historical results can still have an unavailable gain.

PZ poles and zeros use angular frequency (`SignalUnit::RadianPerSecond`,
`rad/s`); their numeric values are not Hz. Both gains retain `gain_unit` as
ohms for current excitation and dimensionless for voltage excitation. SDK
callers constructing `PoleZeroResult` now pass that unit to `new` and include
it in struct literals. Document version 12 carries `rootUnit` and `gainUnit`;
versions 1–11 retain absent unit metadata without guessing from port labels.

How each analysis is reached (netlist card, CLI flag, or engine API only)
varies. The [CLI README](../rspice-cli/README.md) documents the netlist-card
and flag surface; anything not listed there is engine-API only.

A sensitivity study differentiates one output — a node voltage, a differential
voltage or a branch current, at the operating point or at each frequency of an
AC sweep — with respect to the variables one filter list names. The variable
universe is the union of two namespaces: the **device variables** of the
flattened circuit (instance parameters, element and source values, model scalar
and vector parameters) and the **design parameters** of the authored root scope
(`.PARAM`/`.GLOBAL_PARAM`). A filter item addresses the design parameters if
and only if it begins with the literal `PARAM:`, whose remainder globs
parameter names; a filter without that prefix never selects one, not even `*`.
An empty filter list means every device and model variable and no design
parameter, which is ngspice's `.sens`. A design-parameter derivative is
**total**: the deck is replayed with the parameter moved, so every parameter
defined from it moves with it — `PARAM:a` includes `b={…a…}` while `PARAM:b`
holds `a` fixed. Design rows are named `PARAM:<NAME>` and tagged as parameter
sensitivities; they are computed, never recombined from device rows, because a
parameter inside a behavioural expression, a source argument or `.options`
reaches the circuit through no differentiated field at all.

Engine results retain physical circuit names in their output probes, including
numeric and hierarchical node names. Either voltage terminal may be ground;
`V(0,out)` reverses `V(out)`, while `V(0)` and `V(out,out)` have zero output and
zero absolute derivatives. Normalization remains unavailable at zero output.

Method of record. A design parameter on a qualified linear circuit is **exact**:
the parser's forward-mode derivative of every expression the parameter reaches
is contracted with one transpose solve per frequency, so the error is the LU
roundoff. Eligible primitive resistance, capacitance and inductance value fields
use the same physical derivative kernel. This retains small nonzero derivatives
that finite output perturbations cannot resolve. The primitive path requires a
qualified linear deck with resolved, positive passive values and no passive
models or instance scaling; enforced capacitor initial-condition constraints,
terminal-current meters and topology reduction also require the refinement
path. Other device fields and unqualified design
parameters use refined finite differences with the acceptance rule below.
One study has one nominal output and one run budget, whichever path each row took.

The low-level DC adjoint API differentiates effective linear resistances
(including branch-form resistors) and independent source amplitudes. Its
resistance and nonzero-output normalization arithmetic preserves finite extreme
scales without absolute resistance/conductance/output cutoffs. The standalone
`SensitivityAnalyzer` returns `None` for malformed matrices, vectors or element
indices, unsupported derivatives, and nonfinite or inaccurate adjoint solves;
cancellation remains a distinct error. The dense path verifies its original
transpose residual. Authored device/model-parameter studies use the separate
complete-sensitivity engine APIs.

Complete XSPICE studies use the code model's declared parameter types for both
model cards and instance overrides, including deferred scalar/vector values.
Only real scalars and real-vector entries are differentiated; integer, Boolean,
string and complex channels are excluded. Model aliases resolve through the
same builtin catalog used by circuit construction. Declared types take precedence
over parameter-name heuristics, so a real parameter is not excluded because its
name resembles a selector. Native families without descriptors retain their
existing selector exclusions.

Scalar and complete DC/AC sensitivity share one refinement driver. It compares
quadratic derivatives at successively halved, representable coordinates and
checks agreement between one-sided estimates when both sides can be evaluated.
Richardson extrapolation uses the actual rounded coordinate products. Trials
must agree to a relative threshold of 1e-4 plus a scale-dependent arithmetic
roundoff allowance; there is no absolute voltage or current floor. A study that
cannot resolve a consistent derivative within twelve refinements is diagnosed
with its parameter identity, rather than returning the first stencil's value.
Before refinement, observed changes that are too small relative to the probe's
arithmetic noise trigger up to twelve step doublings. A response first observed
as nonzero must remain resolved; equal samples at larger steps cannot erase that
evidence. The final stencil also checks response resolution, so smaller steps
cannot use a growing roundoff allowance to excuse lost probe digits. Initial
steps, including explicit deltas, may grow during this calibration.
An expanded stencil must also reproduce the original nearby samples within
arithmetic noise; it cannot discard evidence of a different local response or
reinterpret nearby failed trials as physical domain boundaries.
These checks provide numerical consistency evidence, not a proof of model
regularity or an independent bound on operating-point solver error. A response
that never changes at the sampled coordinates can still conceal variation below
the evaluator's precision; owning-parameter scales and error evidence remain
necessary for broader accuracy qualification.

At physical or finite-range boundaries the driver compares successive one-sided
quadratic stencils. Only explicit `ParameterDomain` errors or exhausted finite
coordinates establish a boundary. Unclassified circuit, netlist and solver
failures can trigger smaller trials but cannot justify a one-sided derivative.
Capacitor/inductor values, authored Level-1 MOS bounds and XSPICE hard numeric
bounds retain this distinction. Soft bounds remain evaluable so clamping is
subject to the directional checks.
Other model constraints need typed domain diagnostics to admit boundary studies.
Domain/solver failures remain candidates for a smaller step;
cancellation, resource limits and other typed fatal errors stop immediately.
Every attempted trial consumes the shared batch-run budget, including failures.
An ordinary central study uses a nominal solve plus four perturbations; further
refinement consumes additional runs. The default relative step reserves enough
ULPs for refinement at subnormal values and remains finite at f64::MAX. Explicit
steps still require distinct finite coordinates and consistency qualification.

AC studies differentiate the complex output before projecting magnitude, phase
or normalized sensitivities. The single-parameter API replays the requested
nominal override, even when it differs from the authored value, and diagnoses
undefined or out-of-range magnitude derivatives with parameter/frequency context.
Scaled arithmetic retains finite derivatives through overflowing spans and
cancelling weighted sums. Probe extraction and refinement poll cancellation.

Derived sensitivities use `SensitivityValue<T>`: an available number or an
explicit reason. Relative sensitivity and phase are undefined at zero output;
the magnitude derivative is nondifferentiable there when the complex derivative
is nonzero. A zero complex derivative retains a zero magnitude derivative.
Unrepresentable derived values report `OutOfRange` while preserving the absolute
derivative. Invalid inputs remain errors at the result-document boundary.
Sensitivity results retain voltage/current units independently of display names.
Version 13 documents require the nominal output in volts or amperes. Decoding
version 5–12 sensitivity documents preserves their values and marks their former
placeholder output units unspecified. Sensitivity documents before version 5
require a rerun because their zero values do not establish availability.
Other supported legacy result families remain readable. Python exposes missing
DC normalization as `None`, and unavailable AC samples as NumPy NaNs paired with
`*_unavailability` reason arrays; rankings include only available normalization.

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
Circular bounds cover `asin`, `acos`, `tan` and `atan2`, preserving VM clamps,
tangent poles and signed-zero angular seams. Angular derivatives use scaled
coordinates to avoid radius overflow or underflow. A proved VM evaluation order
can establish an exact plateau from equal endpoint values; sampled endpoint
equality alone cannot discard an interval or its internal events.
Value bounds retain defined infinite ranges through bounded outer functions,
including arctangent and hyperbolic tangent across tangent poles. Indeterminate
operations such as zero times infinity, opposite infinities added together, or
an infinite trigonometric argument remain unresolved before any outer clamp
can certify the source. Unbounded derivative intervals remain separate from
that value-domain decision.
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
with the current engine before dependent numerical reuse.
The retained orbit includes canonical MNA branch-current waveforms on the same
time grid as node voltages. Both identities and samples are authenticated and
projected into dependent analyses; omitted branch currents are never inferred
to be zero. Current traces also survive result documents and worker, artifact,
and Python pickle transport. Exact zero-ohm branches constrain charge voltages
without adding spurious shooting coordinates. AC resistance and the threshold
used to select the branch representation do not impose this constraint. Cascaded
and differential VCVS relations also constrain charge coordinates. Their exact
binary64 coefficient rank is reduced once during setup, and initialization
projects device charge derivatives into the independent voltage rates to retain
physical displacement currents. Constant RLC networks with E/F/G/H controls use
an exact MNA descriptor to close hidden charge and flux constraints, including
loading of algebraic control nodes. Prescribed capacitor voltages and winding
currents use analytic forcing derivatives in their transient companions.
Qualified time-only behavioral voltage and current sources supply higher
derivatives through a bounded Taylor evaluator that preserves the selected
dialect's constant-expression semantics. Periodicity and derivative regularity
are checked across the complete orbit; poles and unresolved derivative jumps
are diagnosed. Expressions outside this descriptor's supported operator set
retain their existing path. Memoryless monotone C1 diode islands can also close
algebraic control voltages: exact port dependencies exclude state or nonlinear
current-derivative feedback, and an exact passivity check on each feedback
component certifies a unique algebraic solution. Trial currents and their first
time derivatives use the canonical full device law and its implicit Jacobian.
Solved port voltages and their analytic rates enter the exact projection
directly, preserving small voltages beneath large resistor drops. Overflowing
Jacobian products use a jointly scaled row and right-hand side. A descriptor
with no independent dynamic state supplies the complete candidate at each time;
the same physical residual checks used after Newton certify it before acceptance.
Residuals scale to the actual control voltages, so large controlled-source gains
cannot hide small input errors behind an absolute voltage floor. Diode junction,
sidewall, recombination and tunneling currents preserve the exponential law near
zero bias without subtractive cancellation or avoidable intermediate range loss.
Breakdown/recombination joins, injection knees, higher constitutive derivatives,
solution-dependent behavioral sources and general nonlinear charge/flux
manifolds still require further closure support. Behavioral displacement
currents on the existing path require a qualified analytic outgoing derivative;
unsupported derivatives are reported instead of using Newton's pointwise slope.
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

Rust callers use `Engine::run_pnoise` for driven conversion noise and
`Engine::run_pnoise_oscillator` for autonomous phase noise. The
`run_pnoise_from_pss_with_abort`, `run_pnoise_from_hb_with_abort`, and
`run_pnoise_oscillator_from_pss_with_abort` variants consume an existing
authenticated orbit. `PnoiseCard` is the typed authored request; PSS/PSTB
expose the shared `FloquetSpectrumEvidence` stability contract.

The unused prototype `analysis::pnoise::{PnoiseSolver, PnoiseState,
FloquetAnalyzer}` and its configuration, mode and transfer helper types have
been removed. Its waveform/Jacobian inputs had no public initialization path,
and its phase response estimated charge from voltage and period. Use the
engine entry points above; the spectral result types used by result documents
remain available in `analysis::pnoise`.

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
[`rspice-conformance`](../../tools/rspice-conformance), along with the case-runner and
oracle-capture binaries they drive. This crate declares no binaries at all, and
`tools/ci/test_ci_configuration.py` asserts it holds no validation harness.

For whole-process performance comparison against ngspice, see
[rspice-bench](../../tools/rspice-bench/README.md).

Licensed under the [RSpice Personal Use License](../../LICENSE).

### Physical-event flux tolerance

`SimulationConfig::transient_event_flux_abstol` sets the absolute flux-linkage
conservation tolerance at physical transient events, in weber-turns. Its
default is `1e-24`, and values must be finite and positive. The shared netlist
option is `.options eventfluxtol=1e-24`; ordered `option` commands also accept
it. Explicit configuration overrides take precedence over authored options.
This setting controls event conservation independently of charge tolerance
and ordinary inductor timestep truncation. Existing device admission limits
still apply. Checkpoint configuration identity version 96 binds this setting,
the GP transport-event tracking policy, physical-event integration restart,
exact-history stop-step fitting, and accepted OneStep residual refresh across
hybrid Gear2 intervals;
checkpoints with an earlier configuration identity require a fresh run.
Native GP models in ngspice mode now use ngspice 46's thermal constants
(`k=1.38064852e-23`, `q=1.6021766208e-19`) for their temperature-scaled
equations. BestAvailable, Xyce and VBIC retain their respective constants.
Ordinary and periodic noise also use ngspice's constant pair for generic
thermal and shot sources in ngspice mode. Model-specific noise constants
remain owned by their model.

`SimulationConfig::gp_transient_phase_model` selects `ExactDelay` (default)
or `NgspiceWeil` independently of the evaluator dialect. The latter uses
ngspice's discrete two-sample forward-current filter, with immutable trial
evaluation and accepted state preserved across charge-integration restarts.
Checkpoint format 49 stores its accepted input, two outputs, delay and step
size; changing the selected phase law rejects the old checkpoint. Both phase
operators are available through the public transient, checkpoint, continuation,
and compression APIs. The nominal delay `TF*PTF*pi/180` must be finite and
nonnegative; a negative delay is rejected before startup or checkpoint
publication. Zero delay retains the ordinary GP equations. A physical arrival
that cannot be represented within the timestep limits is diagnosed explicitly.

Native GP acceptance reserves transport-record storage before advancing device
histories. Capture and restore use fallible copies of BJT histories, and restart
normalization consumes its owned checkpoint without duplicating it. Retained
exact-delay samples, event sides, and derivative-order records count toward
`ResourceLimits::max_result_values` along with the waveform and other checkpoint
state.

`ResourceLimits::max_transport_history_bytes` separately bounds native GP
exact-delay record storage across the analysis (256 MiB by default). It includes
spare capacity, event sides, derivative-order records, and simultaneous live and
retained checkpoint copies. Initialization, acceptance, capture, and restore
check the requested storage before allocating it. Resume applies the new
caller's budget to the restored copy; the caller-owned input checkpoint is
outside this analysis's ownership. Fixed device state, other model providers,
and solver workspaces are outside this specific record-storage budget.

These policy refusals carry `ResourceKind::TransportHistoryBytes`. Fallible GP
transport initialization, growth, fixed BJT history lanes, owned UIC solution
seeds, and BJT history copies preserve `SimulationError::Allocation` and the
stable `allocation_failed` code, including through capture and restore.
Both belong to the resource-error category and are not automatically retried.

BJT restart and shooting initialization prepare this storage before changing
accepted state; periodic reconstruction reserves all history generations before
installing BJT bias. Resume uses restored BJT history without constructing a
discarded replacement. This allocation contract covers BJT history storage and
its UIC copies; other model state, caches, and solver workspaces have separate
allocation paths.

The shared delay buffer also enforces its fixed per-site ceiling of 1,048,576
accepted records, counting ordinary samples, separate left limits, and known
derivative-order records after pruning. Native GP propagates this refusal as
`SimulationError::DeviceResourceLimit`, carrying the elaborated instance and
`ResourceKind::TransportHistoryRecords` with requested/allowed counts. Its
descriptor has the nonretryable `resource_limit` code. This record ceiling is
distinct from the analysis-wide byte quota; malformed or oversized checkpoint
data remains a state-validation error. Delay-buffer acceptance methods return
`DelayAcceptanceError` so callers need not classify diagnostic text.

The GP exact-transport path tracks all unknown events and
known discontinuities through derivative order two. Solver-certified C2
inputs retain their delay-history knots and interpolation error control but
do not force another arrival onto the integration grid. This policy covers
the currently implemented BE, trapezoidal, Gear2 and hybrid methods. Before
solving the last adaptive intervals, exact-history runs fit a rounding-sized
remainder to the requested horizon while preserving timestep bounds and event
clocks. This avoids relabeling an already solved state with a different time.
The hybrid integrator also refreshes retained OneStep static residuals during
Gear2 intervals, so returning to trapezoidal integration uses the latest
accepted currents.

Native GP physical events retain outgoing charge/flux rates and delay memory,
then start a new integration epoch. Gear2 uses one BE interval before returning
to second order. Capacitor/inductor error control and checkpoint restoration
share the same accepted interval lengths at that boundary.

Public transient qualification includes a manufactured nonlinear GP orbit
with finite base/collector impedances, exponential current, Early feedback and
TF diffusion charge. Independent harmonic current forcing produces prescribed
base and collector voltages over three periods with a 1 us transport delay.
Ngspice-mode trapezoidal and Xyce-mode TrapGear meet 2 uV base / 20 uV collector
bounds on the complete trajectory. Independent public tests also check delayed
exponential refinement, both polarities, temperature, area and multiplicity,
private resistances, OP/UIC startup, cancellation, and checkpoint continuation.

The selected Weil path is separately compared with ngspice 46 for a clamped
NPN at 1 GHz, TF=1 ns and PTF=21/90 degrees. Both complete 20 ns trajectories
retain all 5,008 recorded ngspice times, including startup. Maximum current
errors are below 5.5 fA within the unchanged absolute-plus-signal tolerance;
packed checkpoint resumes reproduce every remaining voltage and branch-current
sample exactly. The committed decks, raw numeric samples and hash manifest
are in `tests/testdata/gp_weil_*`. Each comparison qualifies its specific model,
topology and parameters; the remaining core qualification ledger stays explicit
about broader coverage.


## Monte Carlo checkpoints and pooling

`Engine::new_monte_carlo_checkpoint` binds an empty journal to a frozen netlist,
sampling seed and policy, environment, ordered measurement names, and an explicit
caller-supplied digest of the complete configured evaluator and prerequisites.
The engine includes the source expressions, semantic circuit, AST overrides and
numerical configuration in this population identity. Resource budgets and worker
counts can change; the identity of the experiment cannot.

`Engine::run_monte_carlo_measurements_checkpointed_with_abort` reuses completed
rows and solves only missing trial indices. It accepts the same scalar evaluator
as the ordinary configured study. A publication callback runs serially after each
newly completed trial and can save the checkpoint at the caller's desired cadence.
Completed rows remain available after cancellation, deadlines, fatal solver errors
or publication failures. Ordinary statistical failures remain explicit completed
rows and are not retried automatically. An interrupted trial remains unfinished.

`MonteCarloCheckpoint::merge_with_limits` pools matching populations, accepts exact
overlap only once, and rejects conflicting overlapping outcomes without modifying
the destination. Resuming a pooled checkpoint rebuilds statistics from the exact
trial samples in the requested range, including its failed-trial count. It does
not average batch means or append duplicate observations. Ranges, histogram bins
and mean-confidence settings can change in `MonteCarloStudyConfig` while the
original circuit and evaluator contract remain frozen.

`to_bytes_with_limits` and `from_bytes_with_limits` provide a versioned binary
format with a content checksum, exact IEEE-754 value bits, trial identities and
failure markers. Byte, trial and retained-value budgets are checked; malformed,
nonfinite, duplicate, reordered or corrupted records are refused. These checkpoints
currently carry numerical observations and trial failure markers. Simulation Studio
checkpoint selection, worker publication, project retention and measurement-verdict
metadata still require frontend integration.
