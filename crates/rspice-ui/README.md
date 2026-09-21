# rspice-ui

The graphical front end for RSpice: schematic capture, netlist editing,
simulation setup and control, and result viewing in one egui/eframe
application with wgpu rendering. The same crate builds for the desktop
(Linux/macOS/Windows, multi-threaded engine with the Verilog-A JIT) and for
`wasm32-unknown-unknown` (single-threaded engine in the browser); the
platform split is handled entirely through target-specific dependencies in
`Cargo.toml`.

## What the application is

The contract-driven workbench (`src/workbench/`) is the sole owner of the
application chrome: menu bar, contextual toolbar, activity rail, document
strip, responsive docks and drawers, console, status bar, phone navigation,
and the central engineering surface. Its seven canonical workspaces are:

- **Project**: project identity, documents, run history, configuration,
  storage state, and project-level actions.
- **Design**: the schematic editor with component placement from a palette,
  orthogonal wire routing with grid and magnetic snap, net labels and
  junctions, selection with net highlighting, rotation/mirroring, copy/
  paste, and an undo/redo history. Symbols are SVG, embedded into the
  binary at build time from `assets/component_symbols/`.
- **Simulate**: analysis setup forms.
- **Results**: immutable run/dataset selection and precision result viewers.
- **Verify**: checks, specifications, measurements, yield, and reliability
  evidence owned by the project.
- **Models**: model and library catalog, bindings, Verilog-A, and PDK setup.
- **Netlist**: a syntax-highlighted SPICE netlist editor with completion
  and a parameter tuner panel.

Desktop, browser, and tablet use the same workbench state and command
registry. Layout composition adapts to available width and pointer capability;
document engines never create a second application shell.

Result viewers live in `src/workbench/documents/result_document/`: waveform
strips with
expression traces and A/B cursors (`waves.rs`, `strip.rs`), Bode
(`bode.rs`), FFT spectrum (`fft.rs`), eye diagram (`eye.rs`), histogram
(`hist.rs`), operating-point inspector (`op_inspector.rs`), noise
contributor ranking (`noise_contrib.rs`), a measurement/spec matrix
(`specs.rs`), Nyquist (`nyquist.rs`), Smith chart (`smith.rs`), and
pole-zero (`pz.rs`). The data/state side of these viewers lives in
`src/analysis/`.

Other user-facing machinery, all verified in source:

- **Command palette** (`workbench/app/command_palette.rs`) with ranked fuzzy
  matching, match-character highlighting, a recents section, and
  hierarchy verbs (descend/ascend) that are dimmed with a reason when
  unavailable.
- **Checks**: a schematic rule checker (`services/drc/`: rule engine, net
  extraction and connectivity, violation types) surfaced through the Check
  menu and toolbar/docbar pills, plus safe-operating-area checking
  (`services/safety/`).
- **About dialog** with version, 9-character build hash (injected by
  `build.rs` via `git rev-parse --short=9 HEAD`, `"unknown"` outside a git
  checkout), engine info, license status, and a copy-diagnostics button.
- **License keys** (`services/license.rs`): offline verification of
  `RSPICE-K1.*` keys, with an Ed25519 signature over a domain-separated
  payload,
  Crockford base32 wire format, compiled-in public keys, a denylist, and
  perpetual-fallback semantics (the expiry is an updates-until date, not a
  kill switch). Issuance, the signing half, lives in that file's test
  module, not in the application; production issuance is the platform
  backend's cold-key flow.

## Harmonic balance operating point

A Simulation Studio HB run uses its explicitly bound OP instance's temperature,
supply scaling and full node-voltage/branch-current seed. The default and `TAHB=2`
startup use that seed directly; `TAHB=1` starts its transient-assisted initialization
from that same state. Explicit `TAHB=0` retains a zero start at the selected physical
operating environment. OP and HB numerical overrides remain separate.

The retained HB state carries the bound temperature and supply settings through
worker transport to PAC, PXF, PNOISE, HBSP and HBNOISE. Temperature-dependent parser
expressions resolve at that temperature before the circuit is built. A consumer
need not repeat the producer's startup-mode option. Changed circuit/model data or
tampered retained state are still rejected. Manual `.HB` decks receive one implicit
OP when none is authored; multiple OP producers are rejected as ambiguous.

HB artifact validation includes the effective `.OPTIONS NONLIN-HB MAXSTEP` from
the frozen producer deck, including temperature-dependent expressions. Consumers
reuse that authenticated orbit without repeating its startup or iteration-budget
options. A different returned solver configuration is still rejected.

Configured HB, HBSP, HBNOISE and HB-carried PAC/PXF/PNOISE studies retain the full
selected OP configuration, including initialization, numerical options and producer
identity. Each varied Monte Carlo or optimization candidate gets a fresh configured
OP before default, DC or transient-assisted HB startup. Explicit zero startup uses
the same physical supply and temperature without running OP. Study Run Set supply
scaling is applied once; explicit OP temperature stays fixed, while OP temperature
modes that follow the Run Set use its selected temperature.

## Harmonic balance current outputs

HB results expose the engine's retained MNA branch currents as `I(device)`,
including voltage sources and inductors, plus exact capacitor currents. These
appear in the result signal chooser and can be selected in studies with, for
example, `bin:1:magnitude:I(V1)` or `bin:0:real:I(L1)`. Current is positive from
the device's positive terminal to its negative terminal, so a supplying voltage
source can have negative real DC current. Voltage traces carry volts and current
traces carry amps through worker transport and plotting. Inductor currents are
published once even though they are present in both MNA and reactive state.
Only currents actually retained with an exact DC component are exposed.

## PSS in configured studies

Monte Carlo and optimization can select shooting PSS as a base. The study freezes
that PSS instance and its explicitly bound operating-point configuration. Each
varied circuit first runs the configured OP startup, accuracy and homotopy policy,
then initializes shooting from that fresh, complete MNA solution. OP and PSS
numerical overrides apply separately; neither stage inherits the other's overrides.
Run Set supply scaling occurs once before both solves, and Run Set OP temperature
modes follow the current study point. Both stages use the bound OP's resolved
temperature, including an explicitly authored OP temperature. Previous-state OP startup still requires an
identity-compatible retained state; it never falls back to a different startup.

Use `last:V(out)` for the final periodic sample, `bin:1:magnitude:V(out)` for a
voltage harmonic, or `bin:1:imag:I(V1)` for a branch-current harmonic. Harmonics
use the actual periodic time grid, retain signed DC, and use peak amplitudes and
cosine-reference phase in degrees. The harmonic index cannot exceed the selected
PSS retention count. `scalar:pss.frequency`, `scalar:pss.period` and
`scalar:pss.iterations` expose the solved frequency, period and shooting corrections.

## Periodic operating-point handoff

Shooting PSS uses its bound OP temperature and supply settings throughout the solve.
PAC, PXF, PNOISE, PSTB and PSP inherit the same environment with the saved PSS state,
including through browser-worker transport. The producer's process-bound circuit
source follows the dependency chain. Consumers still authenticate that circuit and
the numerical state before using the orbit.

A Simulation Studio QPSS run uses its bound operating-point instance's resolved
temperature and explicitly selected supply sources. The temperature is applied before
parsing so temperature-dependent parameters use the same environment as the solver.
With DC initialization enabled,
the full OP node/branch solution seeds QPSS; with it disabled, QPSS starts from zero
at the same physical run point. QPAC, QPXF and QPNOISE receive that physical environment
with the retained QPSS state, including when the state crosses a browser worker.
Manual QP decks without an authored `.OP` get one implicit OP task; multiple `.OP`
producers must be made unambiguous before execution.

The prepared handoff identifies the circuit source before analysis-local numerical
options are appended and separately checks the actual dispatched deck. OP and QPSS
can therefore use different solver settings on the same circuit; changing the
circuit or the carried environment invalidates the handoff. Downstream QP analyses
still authenticate their materialized circuit against the retained orbit, so an
incompatible physical/numerical change requires a new QPSS solve.

## Quasi-periodic analyses in configured studies

Monte Carlo and optimization can select driven QPSS, QPAC, QPXF or QPNOISE.
Each candidate uses its varied physical circuit and the complete selected QPSS
configuration, including tone assignments, lattice retention, sampling, numerical
solver and zero/DC initialization policy. A dependent study reruns its exact
bound QPSS producer and then applies the complete consumer specification to that
retained state. Consumer frequency axes, explicit frequencies, lattices, drive
magnitude/phase, source/output selections, group delay, noise filters, integration
bands and solver controls survive worker transport and participate in result identity.

A configured QP study captures the QPSS producer's exact bound operating-point
instance, including its initialization policy, previous-state payload, convergence
controls, numerical overrides and resolved temperature/supply point. DC initialization
runs that OP on each varied candidate and seeds the full node-voltage/branch-current
state. OP and QPSS numerical overrides stay separate. Zero initialization uses the
same physical temperature and supply point without solving an OP or adding a DC seed.
A study Run Set scales its explicitly selected supply sources once; OP temperature
modes that follow the Run Set inherit its temperature, while an explicit OP temperature
remains fixed. QPAC/QPXF/QPNOISE consume the orbit and physical circuit from that trial.

For QPSS, `tuple:1,-1:magnitude:V(out)` selects the named lattice component;
`real`, `imag` and `phase` (degrees) are also available. Negative tuples retain
conjugate phase, and DC remains signed. Coordinates must belong to the configured
retained lattice. `bin:index:quantity:signal` selects the sorted nonnegative
spectrum, while `scalar:qpss.iterations` and `scalar:qpss.normalized_residual`
select native solve statistics.

QPAC, QPXF and QPNOISE support explicit `bin:index:quantity:trace` and
`last:trace` selections using their full result trace names, including lattice
labels. Separate Monte Carlo observations with semicolons or newlines; semicolons
inside trace parentheses or brackets remain part of that trace name.
Real noise densities and group delay require `real`; cross spectra permit
complex quantities. Native integrated noise selections use one-based output
numbers: `scalar:qpnoise.output_rms(1)`, `scalar:qpnoise.input_rms(1)`,
`scalar:qpnoise.contributor_rms(1,RS thermal)` and
`scalar:qpnoise.contributor_share_percent(1,RS thermal)`. These require the
corresponding integration, input referral or ranking output to be enabled.
Unavailable referrals or integration results remain unavailable, rather than zero.

## Periodic RF analyses in configured studies

Monte Carlo and optimization can select PAC, PXF, PNOISE, PSTB or PSP. Each trial
reruns the selected periodic producer on its varied circuit. Shooting and seeded HB
chains first rerun the producer's bound OP configuration. PAC, PXF and PNOISE also
accept an explicitly bound HB producer. The study retains the complete consumer
configuration: sweep, sidebands, source and differential output, amplitude,
solver tolerances, noise controls, stability controls, ports and mixed mode.

Use explicit observations such as `bin:0:magnitude:V(out)[sb=+0]` for PAC,
`bin:0:real:H(sb1->sb1, V(out))` for PXF, `bin:0:real:output_noise` or
`scalar:noise.output_rms` for PNOISE, `scalar:pstb.max_multiplier_magnitude`
for PSTB, and `bin:0:real:S11` for PSP. PNOISE's `meas:phase_rms_rad` and
`meas:timing_jitter_rms_s` require the corresponding enabled oscillator noise
outputs. A PSTB mode curve can be selected with `bin:index:real:signal`.
Noise integration and contributor observations require those outputs to be enabled.

Numerical overrides are applied at their own stage. A dependent run must remain
compatible with its retained producer state; overrides that change the physical
circuit or invalidate the resolved producer configuration are rejected by the
same authentication checks used by ordinary dependent runs.

## HBSP and HBNOISE in configured studies

Monte Carlo and optimization can select HBSP or HBNOISE as their base. The study
freezes the selected consumer and its explicitly bound HB instance, including
ports, impedances, sidebands, noise references and producer solver settings.
Every trial or candidate solves HB afresh and executes its consumer on that same
materialized circuit, preserving parameter expressions and Run Set changes.
Producer numerical overrides are applied before the consumer's overrides.

HBSP supports complex scattering observations such as `bin:0:real:S11` and
`bin:1:magnitude:S21[k=+0,m=+0]`. Real noise-parameter traces use `real`, for
example `bin:0:real:PN_NF`. HBNOISE supports `bin:0:real:output_noise`,
`bin:0:real:input_noise`, `bin:0:real:noise_figure_db` and contributor names;
`last:signal` explicitly selects the final frequency sample. Indices are zero
based on the configured consumer sweep, not the HB carrier harmonics.
`scalar:noise.output_rms` and `scalar:noise.input_rms` select integrated noise.
Enable the corresponding noise figure, integration or contributor option to
measure that result. Unavailable observations fail instead of supplying defaults.
The PSD, noise-figure and integrated selectors are also available to basic Noise
studies when their selected analysis produces those quantities.

## Harmonic balance in configured studies

Monte Carlo and optimization can select a harmonic balance instance as their base.
Its full tone definitions, source routing, mixing order, collocation grid and solver
controls are frozen with the study and passed to every varied circuit. Parameter
expressions and Run Set temperature/supply changes remain materialized throughout
the solve. The HB verbosity switch also controls the study's solver log.

Use `bin:index:quantity:V(node)` for a retained harmonic phasor; `quantity`
is `real`, `imag`, `magnitude` or `phase` in degrees. Bin indices follow the HB
result's frequency grid (including the mixed-frequency grid for multiple tones).
These use the displayed spectrum's peak-amplitude convention: a unit sine has
magnitude 1 and cosine-reference phase -90 degrees at its fundamental. A signal suffix is required when
more than one complex waveform is present.

## Fourier and FFT in configured studies

Monte Carlo and optimization can select a Fourier or FFT instance as their base.
The study freezes that instance's exact bound transient producer, including its
start/stop times, step limits, initial-condition policy and numerical overrides.
Every varied trial or candidate reruns that transient on its materialized circuit.
The spectrum comes from that fresh result; no nominal retained trajectory is reused.

The producer's numerical options are applied before the selected consumer's
options, so an explicit Fourier override wins when both set the same option.
An FFT has no independent solver options. Its complete request is carried by the
trial transient, and unrelated FFT cards are excluded from that study solve.

Use `bin:1:magnitude`, `bin:1:real`, `bin:1:imag`, or `bin:1:phase` (degrees) to
select a zero-based retained spectral bin. Fourier with multiple outputs requires
a signal suffix, for example `bin:1:magnitude:V(out) Spectrum`. `scalar:DC` and
`scalar:THD(%)` select the Fourier scalars when requested. FFT exposes `scalar:fft.dc`
and retained `fft.fundamental_magnitude`, `fft.thd_ratio`, `fft.thd_db`, `fft.sndr_db`,
`fft.enob_bits`, `fft.snr_db`, `fft.sfdr_db`, `fft.sfdr_spur_frequency_hz` scalars;
those metric values require the deck's `.OPTIONS FFT FFTOUT=1`. Missing, incomplete,
ambiguous or non-finite observations are not replaced with zeros or nominal data.

## Temperature in configured studies

Configured studies resolve the selected OP or Run Set temperature before parsing
circuit expressions and conditional topology. The same temperature is retained
when Monte Carlo trials redraw statistical expressions or optimization candidates
replay parameter changes. Explicit OP temperatures take precedence over the Run Set;
OP modes that follow the Run Set inherit its temperature.

A fixed study environment keeps `TEMP`, `TEMPER` and `VT` fixed instead of treating
them as generic tolerance parameters. Existing parameter/device overrides and
statistical trial coordinates survive replay. Supply scaling is applied once after
the trial circuit has been materialized.

## Monte Carlo parameter bounds and truncation

Custom parameter distributions support optional absolute lower and upper bounds.
Gaussian and lognormal variations also support a symmetric sigma cutoff; for
lognormal this applies in log space around the nominal median. Both constraints
are intersected when supplied together. Bounds apply after each process or
mismatch draw; mismatch uses that instance's process-shifted nominal.

The sampler rejects draws outside the bounds rather than clipping them to an
endpoint. Correlated parameters are redrawn together, while independent variables
retain separate keyed streams. Authored Pearson correlations describe the
population before conditioning: truncation can change means, variances and
correlations. Replay retains the same bounded draw for the same trial identity.

Maximum sampling attempts is configurable from 1 to 1,000,000 (default 10,000).
A correlated group uses the smallest limit of its bounded members. Empty support,
incompatible singular correlations, or an exhausted attempt limit produce an
explicit error. Extremely narrow or remote-tail bounds can need a larger limit;
no partially accepted or clipped population is returned. Saved drafts, worker
requests and task identity retain all limits. Older drafts remain unbounded.
Linearized DCMATCH cannot currently compute truncated joint moments and explicitly
refuses bounded statistics; sampled Monte Carlo executes them.

## Monte Carlo trial ranges and replay

Simulation Studio's **First trial index** selects the beginning of a batch; **Samples** selects its length. Indices start at zero and match the trial identities retained in results. The default first index is zero, preserving existing studies.

For example, first index 1000 and 100 samples run trials 1000 through 1099. To reproduce a particular trial, enter its retained index and use one sample. To continue after a previous batch, use the first index after that batch's final requested trial, including failed trials.

Keep the seed, parameter/distribution settings, configured base analysis, measurements, circuit, models, and Run Set point unchanged to reproduce the same population. A blank Studio seed uses the existing repeatable default; an explicit seed makes the intended stream clear when moving between frontends.

The equivalent authored card is:

```spice
.mc 100 START 1000 SEED 42 DIST GAUSS SPREAD 0.05 PARAMS RLOAD
```

`START=1000` is also accepted. `START` is a zero-based trial index; the first positional number remains the number of requested runs. Put `PARAMS` last, as it consumes the remainder of the card. Negative, fractional, repeated START options and overflowing ranges are rejected.

Each batch publishes its own samples, statistics, mean-confidence intervals, and failure count. Failed trials retain their original identities and do not shift later indices. Statistics for a continuation batch describe that batch; previous batches are not automatically pooled. The result's recorded trial indices and sampling seed identify the original draws.

Generic parameter tolerances replay the preceding random draws without solving their circuits, preserving the original shared random stream even when Gaussian sampling rejects an endpoint. Consequently, large starting indices still take time to advance that stream. Deck-expression and native process/mismatch statistics address the requested trial coordinates directly. All routes solve and retain only the requested batch, and changing worker count preserves sample order.

The CLI and Python execution of authored `.MC` cards honor START. Core callers can use `MonteCarloRunConfig.first_trial` with `Engine::run_monte_carlo_voltages_with_abort`, or `MonteCarloStudyConfig.first_trial` for configured measurement studies. Existing convenience entry points retain their original first index of zero. Nonzero starting indices are also retained as the `first_trial` scalar in shared result documents.

## Module map

| Module | Contents |
| :--- | :--- |
| `workbench/` | The `RSpiceApp` application type (the egui `App` impl) and everything around it: contract-driven responsive chrome, typed command registry and command palette, dialogs, project launcher, preflight, workspace surfaces, docks and drawers, `documents/` (netlist document and the result-document viewers) |
| `schematic/` | Schematic rendering: canvas view (pan/zoom/interaction), SVG symbol library, component palette, source labels, SVG export |
| `state/` | Application state: schematic state (components, wires, nets, selection, snap, clipboard, undo history, symbol generation), simulation state (runs, waveforms, cross-probing), workspace, library browser, model library, property registry, PDK config |
| `simulation/` | Simulation control: the controller state machine, `engine_bridge/` (the rspice-core adapter: parsing, per-analysis dispatch, result conversion, abort handling), netlist generation from the schematic, multi-run batching, optimizer, options translation, automation, netlist viewer |
| `services/` | Backend services: `drc/` rule checking, `license.rs`, `safety/` SOA checks, `simulation_runner/` per-analysis launchers (AC, DC, transient, HB, PSS, noise, pole-zero, sensitivity, Monte Carlo, sweeps, optimization, reliability, distortion, transfer function, pnoise sidebands, PAC/PXF), `yield_manager.rs` |
| `analysis/` | Result-viewer data and state: Bode, FFT, histogram, Nyquist, pole-zero, Smith chart, eye diagram, phase noise, HB tones, waveform calculator |
| `io/` | File formats: schematic JSON, project files, SPICE `.lib` parsing, netlist export, waveform I/O, Cadence PSF (including binary) |
| `properties/` | Property editing: engineering-notation value parsing/formatting, model browser, PWL editor, tabbed property dialog, property bridge |
| `results/` | Result-set ownership and the projection each viewer reads |
| `hardcopy/` | The print and export pipeline: page geometry in integral micrometres, sheet composition, hand-off to the publication contract |
| `automation_runtime`, `automation_workflow/` | The Automation worker host, native and browser, over [`rspice-automation-protocol`](../rspice-automation-protocol), and the workflows built on it |
| `product/` | Edition, entitlement, and feature-availability gating |
| `quantity/` | Typed physical quantities and their formatting |
| `output_spec`, `diagnostics/` | Authored output selection, and the typed diagnostics surface |
| `ui/` | The RSpice design system: mockup-governed semantic tokens and dark/light palettes, mode/density preferences, embedded IBM Plex fonts, vector icon set, the widget vocabulary (buttons, chips, dialogs, docbar, forms, pills, tables, toasts, trees…), and the strip-plot engine (axes, scales, traces, cursors, min/max decimation, SI formatting) |
| `time_compat` | Validated wall timestamps and monotonic elapsed time on native, browser, and worker targets |

## Engine integration

The UI never calls `rspice-core` from a surface or widget. Execution enters
through `src/simulation/runner/`: config-backed SPICE analyses are adapted by
`src/simulation/engine_bridge/`, while specialized RF, periodic, statistical,
reliability, optimization, and sweep analyses are adapted by
`src/services/simulation_runner/`. Both adapters consume the same
preflight-sealed netlist and abort signal and convert engine results into the
UI's waveform containers. Platform differences are set in `Cargo.toml`:

- **Desktop** (`cfg(not(target_arch = "wasm32"))`): `rspice-core` with
  default features (parallel + SIMD solver paths) plus `veriloga-native`
  (RSpice-owned native JIT for Verilog-A devices: full JIT or a typed
  construction error); multi-threaded tokio runtime.
- **wasm32**: `rspice-core` with `default-features = false` and the `veriloga`
  and `wasm` features: portable Verilog-A, no rayon or SIMD; current-thread
  tokio runtime; `web-sys`/`wasm-bindgen` for the DOM. Runs execute in a module
  worker, so cancellation terminates the worker and does not leave detached
  computation. The separately qualified browser JIT
  (`rspice-core/veriloga-wasm-jit`) is added by the `browser-worker` feature,
  not by the base wasm32 image.

Native execution remains on a background thread and every analysis family now
cooperatively polls the same typed abort signal through parsing, expansion,
solver, transform, and result-conversion loops. The Stop command is therefore
enabled on both native and browser targets: native runs unwind cooperatively,
while browser runs additionally terminate their isolated module worker.

Sealed model-library imports retain both device runtimes and standalone
Verilog-AMS connection libraries. Each source has an exact virtual key, import
alias and artifact identity; prepared decks inventory both kinds and install
them in one core transaction. `.options connectrules=NAME
connectrules_source=ALIAS` selects a named configuration without relying on the
original filesystem path. Model-library compilation prepares each source root
once across its module selections and retains canonical digital plans for mixed
devices. Connection libraries never enter the device JIT inventory.

Project-editor, configured project-cell and signed-PDK model compilation also
retain mixed analog/digital artifacts for the unified host. Project execution
uses the editor's build profile, including macros, include paths, selected entry
modules, cell-binding checks and required backend qualifications. Signed-PDK
admission and execution validate the same authenticated source bytes. Standalone
connection-library entries in project-editor and PDK manifests remain separate
integration work; these model routes still select an executable module.

Worker request protocol 10 requires the complete source inventory, including an
explicit empty connection list when there are no libraries. Older requests must
be rebuilt with the matching application/worker release. Native contract checks
do not qualify actual browser or tablet execution; authored connect-body
execution and broader mixed-analysis support remain separate engine work.

The pure-Rust `rspice-veriloga` compiler is a direct dependency on all
platforms (it backs the Verilog-A dialog), and `ed25519-dalek` is used
std-only so license verification also works on wasm32.

## Accessibility runtime

Painter-backed controls publish egui widget metadata and visible keyboard
focus indicators. Native builds enable eframe's AccessKit bridge so the
semantic tree is handed to supported platform assistive-technology APIs.
The eframe 0.35 browser backend does not expose that AccessKit tree through
the DOM; browser builds therefore offer an opt-in **Speak control changes**
preference backed by eframe's Web Speech event feedback. That spoken-event
fallback is not a substitute for a browser accessibility tree, so real
screen-reader and device qualification remains a release gate.

## Feature flags

| Feature | Default | Effect |
| :--- | :--- | :--- |
| `generated-veriloga-catalog` | off | Turns on `rspice-core/veriloga-builtins`, so a build ships the generated Verilog-A device catalog. Every release image sets it |
| `browser-worker` | off | Builds the isolated browser simulation/compiler/hardcopy worker entry image; never enable this on the interactive UI image because it defeats code-size separation |
| `browser-qualification` | off | Exposes the actual browser UI's rendered control tree for functional WebDriver tests; polls for observation requests every 100 ms while idle |

`default = []`, and this crate declares no other flags. Nothing native is
feature-selected: desktop-only behavior is chosen by target-specific
dependencies and `cfg(not(target_arch = "wasm32"))` code paths, so there is
no `desktop` flag to pass. The engine's Verilog-A support is wired through
the **target-specific** `rspice-core` features above and, for the shipped
device catalog, through `generated-veriloga-catalog`.

## Building, running, testing

Run these commands from the workspace root:

```bash
# Desktop application (binary name: rspice-ui)
cargo run -p rspice-ui --release --features generated-veriloga-catalog

# Unit tests (inline #[cfg(test)] modules across the crate)
cargo test -p rspice-ui

# Browser release images are deliberately built separately so Cargo feature
# unification cannot pull worker execution paths back into the UI image.
cargo build --locked --profile web-release -p rspice-ui --bin rspice-ui --features generated-veriloga-catalog --target wasm32-unknown-unknown
cargo build --locked --profile web-release -p rspice-ui --bin rspice-ui-worker --features browser-worker,generated-veriloga-catalog --target wasm32-unknown-unknown
wasm-bindgen --target web --out-name rspice-ui --out-dir crates/rspice-ui/web/pkg target/wasm32-unknown-unknown/web-release/rspice-ui.wasm
wasm-bindgen --target web --out-name rspice-ui-worker --out-dir crates/rspice-ui/web/pkg target/wasm32-unknown-unknown/web-release/rspice-ui-worker.wasm
python3 tools/ci/check_wasm_jit_browser.py
```

The browser qualification page starts only the optimized simulation worker
and fails unless its secondary-module ABI probe and real Verilog-A transient
solver/Jacobian/matrix/RHS probe both pass. CI reports the raw and gzip sizes
of the delivered production UI and worker modules before building the
instrumented workbench. These reports do not enforce size thresholds.

For the real workbench qualification, build the worker as above, then replace
the UI bindings with the instrumented image. A WebGPU-capable Chrome installation
and a matching ChromeDriver are required; `--browser` and `--driver` select them
when automatic discovery is unsuitable.

```bash
cargo build --locked --profile web-release -p rspice-ui --bin rspice-ui --features browser-qualification,generated-veriloga-catalog --target wasm32-unknown-unknown
wasm-bindgen --target web --out-name rspice-ui --out-dir crates/rspice-ui/web/pkg target/wasm32-unknown-unknown/web-release/rspice-ui.wasm
python3 tools/ci/check_browser_workbench.py --output target/workbench-qualification
python3 tools/ci/check_browser_engine_recovery.py --web-root crates/rspice-ui/web --output target/engine-recovery-qualification
```

Append `--software-webgpu` to use Chrome's SwiftShader WebGPU adapter for
functional CI. This does not qualify physical GPU support or performance.

The workbench journey injects invalid wall-clock readings while posting and
resolving reviews, creating checkpoints, saving validated revisions and
publishing model-validation receipts and provider decisions. It checks retained
drafts and reads durable project bytes independently, then
retries with a valid clock and reloads the saved history. Forward and backward
clock adjustments also exercise the displayed ages. The validated-save case
temporarily makes the OS file picker unavailable to exercise normal OPFS
publication; it does not qualify OS picker or external-file permission behavior.
Model validation checks both Save all and active-model Save, retained receipt
bytes after clock failures, and successful retry without unrelated project edits.
The provider workflow imports actual source folders, retains multiline audit
reasons through failed publication, and saves and reloads replacement decisions.

The engine recovery check delays the first worker module response, queues an
authored deck, fails that response, and retries startup through the status bar.
It verifies terminal failure, a successful rerun, and its analytic operating
point in a durable checkpoint. The served worker code is unchanged.
Pass `--cancel-startup` to exercise Stop and engine restart while the initial
worker response is still pending.

Each run requires an empty output directory and creates a fresh browser profile
and HTTP origin. It posts and resolves a review, creates an IndexedDB checkpoint,
checks durable timestamps and content, exports an independent recovery copy, and
opens revision history through real keyboard and pointer input. Screenshots,
rendered control trees, browser errors, asset hashes, and saved bytes remain in
the output directory. Opening input includes a palette shortcut followed immediately
by text and Select All, plus a New comment click followed by immediate typing in
the same WebDriver request. These steps have no intervening control-settling wait;
native regressions additionally exercise a single egui pass. The observer adds no editor commands or authorization
overrides. Its periodic repaint excludes this image from idle/performance budgets;
it does not provide a production accessibility bridge. CI runs the harness's
integrity regressions and the workbench sequence with software WebGPU, retaining
the evidence on success or failure. Hardware and device qualification remain separate.

The site assembler packages the two bindings and compressed modules together with
`simulation-worker.js`, `wasm-loader.js`, `automation-worker.js`, and the pinned
`python/` runtime in one content-addressed directory. The client owns decompression
and worker routing; assembly only stamps the origin-rooted `/ide/assets/<hash>`
path. The Python worker starts on demand using that same page-bound identity.

Qualify the assembled production tree with a matching Chrome/ChromeDriver:

```bash
python3 tools/ci/check_wasm_jit_browser.py --web-root ../RSpice-Site/_site --worker-path ide/assets/<hash>/simulation-worker.js
python3 tools/ci/check_browser_release.py --web-root ../RSpice-Site/_site --output target/browser-release-qualification
```

The second gate checks actual UI startup at three emulated viewport sizes,
playground transient solves with finite rendered traces and unclipped controls,
and Python breakpoint/evaluate/step integration. Version 2 qualification receipts
are required; earlier receipts could accept an empty plot with a solved notice.
It applies the packaged `_headers` policy and records screenshots, browser
errors, and the hashes of the packaged inputs and response-header policy.
Each case opens its own tab, preserving the application's unsaved-work guard.
The report records the qualification-tool revision separately from the artifact's
build revision, so a harness correction can recheck unchanged release bytes.
It does not qualify physical tablet input, accessibility, or the full engineering
workflow; the instrumented workbench and device qualification remain separate.

The default test suite is self-contained. Parity checks against the separately
governed `rspice-workbench-host` mockup sources are `#[ignore]`d, because that
tree is a different repository and is not part of this checkout. Fixture example
names are checked for internal consistency; shipping capability labels and
implemented result formats are owned by the in-tree catalog and its tests. Run them
explicitly, optionally pointing `RSPICE_MOCKUP_ROOT` at the checkout:

```bash
RSPICE_MOCKUP_ROOT=/path/to/rspice-workbench-host \
  cargo test -p rspice-ui --lib workbench::feature_availability_data::tests:: -- --ignored
```

Capability-resolver security tests do not depend on that external tree. Their
closed-contract design fixture is tracked as Rust test data beside the
resolver so clean checkouts exercise the fail-closed policy on every CI run.

There are no runtime asset files to install: `build.rs` embeds the
component-symbol SVGs and exports the git hash at compile time; fonts and
the window icon are compiled in; on Windows, `winresource` embeds
`rspice.ico` into the .exe (degrading to a build warning if the resource
compiler is missing). User configuration is persisted under the platform
config directory (via `dirs`).

For the wasm build of the full UI, the target is wired up in `Cargo.toml`
(`main.rs` has a `wasm32` entry point that attaches to a `#rspice_canvas`
element). The `/ide/` surface is an experimental browser IDE that requires a
WebGPU-capable browser and routes simulations through the module worker guarded
by `tools/ci/test_ide_worker.py`. The narrower
[rspice-wasm](../rspice-wasm/README.md) `/play/` playground remains the
lightweight OP/AC/TRAN engine demo. Both routes are assembled and published by
the separate RSpice-Site repository (`tools/build_simulator.py`); this repo
builds and reports raw/gzip sizes of the wasm images without arbitrary size
caps, and runs browser qualification. It does not deploy them; hosting upload
limits must be checked against the assets packaged by the deployment repository.

### License issuance

Issuance lives in `services/license.rs`'s test module rather than in a binary
target. It needs the private payload layout, and reaching it from a separate
target would mean `pub` re-exports from the crate root, the visibility hole
`tests/module_layering.rs` exists to keep shut. `cfg(test)` also guarantees
that no signing code is linked into a shipped binary, which a Cargo feature
could not.

Both entry points are `#[ignore]`d: this is a fixture generator run when the
wire format changes, not routine tooling.

```bash
# Regenerate a signed fixture (Ed25519 signing is deterministic, so the same
# secret and parameters reproduce a byte-identical key)
RSPICE_LICENSE_SECRET=<hex64> RSPICE_LICENSE_NAME="Name" \
  cargo test -p rspice-ui --lib mint_signed_key -- --ignored --nocapture
```

```bash
# Rotate the development signer (key id 0x01); paste the printed Rust array
# into DEVELOPMENT_VERIFYING_KEYS and never commit the secret
cargo test -p rspice-ui --lib mint_development_signer -- --ignored --nocapture
```

`mint_signed_key` accepts `RSPICE_LICENSE_{KEY_ID,TIER,SEATS,ISSUED_DAYS,
EXPIRES_DAYS,FEATURES,LICENSE_ID}` as overrides. Production issuance is out of
scope for this repository: it belongs to the platform backend's cold-key flow.
The signing path itself is covered by `issued_key_round_trips`, which runs in
CI, so a wire-format change cannot silently break issuance.

Licensed under the [RSpice Personal Use License](../../LICENSE).
