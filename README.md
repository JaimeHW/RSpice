<div align="center">

<img src="assets/brand/logo.svg" alt="RSpice logo" width="140" />

# RSpice

**An analog and mixed-signal circuit simulator, written in Rust.**

Run SPICE netlists from a command line, a desktop application, Python, or the
browser: operating points and transient waveforms through harmonic balance and
phase noise.

[![License](https://img.shields.io/badge/license-source--available-informational?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.94-orange?style=flat-square)](rust-toolchain.toml)
[![Platform](https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-lightgrey?style=flat-square)](https://github.com/JaimeHW/RSpice)

[Quick start](#quick-start) · [Analyses](#analyses) · [Devices](#devices) ·
[Dialect](#netlist-dialect) · [Interfaces](#interfaces) ·
[Validation](#validation) · [Repository](#repository)

</div>

---

## Overview

RSpice is a circuit simulator for standard SPICE netlists. It has DC operating
point, transient, small-signal AC, noise and distortion analyses, plus RF
steady-state coverage through harmonic balance, periodic and quasi-periodic
steady state, and phase noise. Its device library spans the classic SPICE
elements, native BSIM3, BSIM4, BSIM-SOI, VBIC and EKV compact models, and 43
compact models generated from their Verilog-A sources. Mixed-signal decks are
first-class: RSpice has both halves of Verilog-AMS, so digital and real-number
modules simulate alongside the analog solve, with connect modules inserted
automatically at discipline boundaries.

RSpice has one engine behind every interface. The command line, the desktop
and browser application, the Python bindings and the WebAssembly build all run
the same `rspice-core` code, so a result never depends on how the run was
launched. Where a deck needs physics RSpice does not have, the answer is a
typed error naming the parameter or model level responsible, never a silent
approximation.

## Quick start

The toolchain is pinned to Rust 1.94 by [rust-toolchain.toml](rust-toolchain.toml);
rustup picks it up automatically.

```bash
git clone https://github.com/JaimeHW/RSpice.git
cd RSpice
cargo build --release -p rspice-cli
```

Describe a circuit, `rc_lowpass.sp`:

```spice
* RC low-pass step response
V1 in 0 PULSE(0 5 0 100n 100n 1m 2m)
R1 in out 1k
C1 out 0 100n

.TRAN 1u 1m
.MEAS TRAN vpeak    MAX V(out) FROM=0 TO=1m
.MEAS TRAN risetime TRIG V(out) VAL=0.5 RISE=1 TARG V(out) VAL=4.5 RISE=1
.END
```

Run it:

```console
$ target/release/rspice run rc_lowpass.sp --meas
✓ Transient complete: 1020 time points computed
  Measurement Results (TRAN, 2):
    VPEAK = 4.999773e+00
    RISETIME = 2.197212e-04
```

Write results to a file instead, in SPICE raw, ASCII raw, CSV, TSV, JSON, HDF5,
or VCD for digital and mixed-signal runs:

```bash
target/release/rspice run rc_lowpass.sp -o rc.h5 --format hdf5
```

`cargo install --path crates/rspice-cli` puts `rspice` on your `PATH`.

At the workspace root, plain `cargo build`, `cargo check` and `cargo test`
select the CLI and the application and their dependencies, not every model or
development tool. Use `-p <package>` for the bindings and tools, or
`--workspace` for everything.

## Analyses

How each analysis is requested from a deck or from `rspice run`:

| Analysis | Invoked by | Notes |
| :--- | :--- | :--- |
| Operating point | `.OP` | A deck with no analysis card runs one implicitly |
| DC sweep | `.DC` | One source, or nested two-source |
| Temperature sweep | `.TEMP` | A run axis over a list of temperatures |
| Parametric sweep | `.STEP` | A run axis over a parameter, a device or a `DATA=` table |
| Transient | `.TRAN` | Adaptive timestep, breakpoint handling, `TRNOISE` transient noise, `--checkpoint`/`--resume` segmentation |
| AC small-signal | `.AC` | Including `.AC DATA=` |
| Noise | `.NOISE` | Including `.NOISE DATA=` |
| Pole-zero | `.PZ`, `--pz-input`/`--pz-output` | |
| Transfer function | `.TF` | |
| Sensitivity | `.SENS`, `--sens-output`/`--sens-param` | DC and AC |
| Distortion | `.DISTO` | Third-order Volterra; harmonic and two-tone intermodulation |
| DC mismatch | `.DCMATCH` | |
| Fourier / THD | `.FOUR` | Post-processes a transient |
| Transient spectrum | `.FFT` | Typed spectra, Xyce-compatible windows, calibrated DC-through-Nyquist bins |
| Monte Carlo | `.MC`, `--monte-carlo` | Operating-point voltages or named `.MEAS` results per trial; gaussian, uniform or worst-case |
| Process corners | `--corners` | Reruns the deck's analyses once per corner from `--corner-lib` |
| S-parameters | `.SP`, `--sparam` | Touchstone export |
| Stability (loop gain) | `.STB` | Probes a 0 V source placed in the loop |
| Harmonic balance | `.HB`, `--hb-freq` | |
| Envelope | `.ENVELOPE` | Envelope continuation over a preceding `.HB` |
| Periodic steady state | `.PSS`, `--pss-freq` | Shooting method; driven or autonomous |
| Periodic AC, transfer function and noise | `.PAC`, `.PXF`, `.PNOISE` | Linearized around the preceding `.PSS` or `.HB`; phase noise by Floquet projection, PPV for oscillators |
| Periodic stability | `.PSTB` | Floquet multipliers of a shooting `.PSS` |
| Quasi-periodic | `.QPSS`, `.QPAC`, `.QPXF`, `.QPNOISE` | Multi-tone steady state and its small-signal analyses |
| Periodic S-parameters, HB S-parameters, HB noise | Configured studies | `rspice study` and the application |
| Safe operating area, optimization | Configured studies | `rspice study` and the application |
| Measurements | `.MEAS` | Over TRAN, DC, AC and NOISE; `GOAL`/`TOL` gates the exit status |

`.STEP`, `.TEMP` and `--corners` multiply the analyses a deck requests rather
than adding one of their own. Whether an analysis may run on a given circuit
is decided before any solver work, and a refusal names the analysis, the
instance and the reason.

## Devices

Every schematic device has a single descriptor behind all of its surfaces
(properties, persistence, hierarchy, rule checks, netlisting, preview, export
and hardcopy). A device whose id, registry entry, terminal contract,
parameter contract or symbol cannot be resolved fails closed; nothing falls
back to a generic two-terminal element or drops unsupported parameters.

| Family | Models |
| :--- | :--- |
| MOSFET | Native: Berkeley MOS1/MOS2/MOS3/MOS6 and ngspice MOS9, legacy BSIM1/BSIM2 (`LEVEL=4/5`), BSIM3 3.3.0 (`LEVEL=8/49`), BSIM4 v4.8 (`LEVEL=14/54`), BSIM-SOI in FD/DD/PD variants (`LEVEL=10/55/56/57`), VDMOS (`LEVEL=18`), EKV 2.6 (`LEVEL=260`) and EKV3 (`LEVEL=301`). Generated: BSIM-BULK, BSIM-CMG, BSIM-IMG, PSP104, HiSIM-HV, HiSIM-SOI, HiSIM-SOTB, L-UTSOI, MVSG-CMC and others, by model type or by their `LEVEL` |
| Bipolar | Native Gummel-Poon (`LEVEL=0/1/2`) and VBIC (`LEVEL=4/9/11/12/13`); generated HICUM/L0, HICUM/L2, MEXTRAM 505 and VBIC 1.3 |
| Junction | Diode (`LEVEL=0/1/2/3`), generated DIODE_CMC and JUNCAP200, JFET level 1, Parker-Skellern JFET2 (`NJF`/`PJF LEVEL=2`), MES/MESA/HFET-family `Z` devices, GaN HEMT |
| Passives | R / C / L with temperature coefficients, generated R2_CMC and R3_CMC resistors, coupled inductors and multi-winding transformers, saturable inductor with Jiles–Atherton hysteresis |
| Transmission lines | Ideal, lossy (LTRA, TXL), coupled multi-conductor (CPL) |
| Sources | Independent V/I with `PULSE`, `SIN`, `EXP`, `PWL`, `PAT`, `SFFM`, `AM`, and `TRNOISE` white + 1/f waveforms; E/F/G/H controlled sources; B behavioral sources; PWL-from-file sources |
| Switches & macromodels | Voltage- and current-controlled switches, op-amp macromodel |
| Mixed-signal | Verilog-AMS digital and real-number (`wreal`) modules with automatic connect-module insertion; XSPICE-style analog and digital code models, tri-state drivers, A/D–D/A bridges |
| Verilog-A | Generated compact models and externally compiled modules; see [below](#generated-verilog-a-devices) |

Unlisted `M` levels fail closed rather than falling through to the simplified
MOS equations, and an `M` level that names a compact model with no native port
is refused by family name unless a compiled-in generated model serves it.
`Q LEVEL=8/23/230/234/504/505`, other simulators' selectors for HICUM, HBT_X
and MEXTRAM, are refused by name in every build; those families are reached by
model type instead. BSIM-class cards raise typed errors for unported physics
rather than approximating around it. Under the Xyce dialect, `LEVEL=9` and
`LEVEL=49` use Xyce's BSIM3 3.2.2 equations, and `Q LEVEL=11/12` route to the
generated VBIC. The [`rspice-core` README](crates/rspice-core/README.md#device-routing)
has the full routing table.

### Generated Verilog-A devices

Compact-model families with redistributable sources under
[models/veriloga/](models/veriloga/) are not hand-ported. They are generated to
Rust from the upstream Verilog-A and checked in under
[`crates/rspice-veriloga-models/`](crates/rspice-veriloga-models/README.md),
one reusable Cargo artifact per model; that crate's README lists all 43 models
with their features.

A deck reaches a generated device in either of two ways: by its model name on
an `X` line, or through an ordinary element whose `.model` card names the
model's type or a `LEVEL` routed to it (`.model qh hicuml0va`,
`.model dcmc d level=2002`). Each compiles in only when its `veriloga-model-*`
feature is enabled, so compile time, peak rustc memory and binary size stay
proportional to the models a build uses. `M LEVEL=260` is the one deliberate
overlap: the generated EKV 2.6 when it is compiled in, the native port
otherwise.

All 43 models reach every shipped target: desktop through
`rspice-app --features generated-veriloga-catalog`, WebAssembly with no native
JIT dependency, and Android or iOS ARM64 through
`rspice-core --no-default-features --features veriloga-builtins`.

Catalog and round-trip checks prove a device is reachable and consistently
represented, not that it is numerically accurate; that needs comparison
against an independent reference for each compact-model version. ASM-HEMT and
MVSG-CMC have no such qualification, and the in-tree `Z`-device GaN HEMT is a
physics-style model, not a CMC one.

## Netlist dialect

`.SUBCKT` subcircuits are flattened during elaboration, with hierarchical path
handling and scoped parameters. `.PARAM`/`.CSPARAM` resolve at elaboration,
`.FUNC` defines functions for parameter and behavioral expressions, B-sources
compile to a bytecode expression VM, and `.IF`/`.ELSEIF`/`.ELSE`/`.ENDIF`
select at parse time. `.INCLUDE`, `.LIB` and
`.MODEL` bring in model cards. `.GLOBAL`, `.IC`, `.NODESET`, `.SAVE`/`.PROBE`,
`.PRINT`/`.PLOT`, `.OPTIONS` and `.TEMP` behave as expected, the usual
engineering suffixes are accepted, and unrecognized dot-commands surface as
diagnostics rather than being silently dropped.

HSPICE `.ALTER` and `.DATA` expand into a plan of concrete decks, each parsed
and solved independently, tagged so one run cannot overwrite another, and
spread across workers with `-j`. `--spice-dialect` picks the expression,
measurement and device semantics to match: `ngspice`, `xyce`, or `best` for
RSpice's most accurate evaluator with ngspice-compatible expression syntax.

Beyond plain SPICE, the parser ingests SPEF (IEEE 1481) parasitics as
back-annotation onto a parsed netlist, XSPICE code-model cards, and
Laplace-defined sources. A fail-closed Spectre front end handles the statements
foundry model libraries and macromodels use, lowering them to canonical SPICE
with unsupported semantics raised as errors rather than guessed. LTspice `.raw`
files can be read back for comparison.

Every product build ships the small, generic
[foundation library](models/spice/foundation/). It is the only SPICE pack in
this repository, and like everything under `models/spice/`, it is authored by
the RSpice project. Projects can import their own model sources. Further packs
are published through the Model Hub as signed, immutable archives: a canonical
manifest under ed25519 trust, a signed catalog snapshot and hardened archive
reading, so an installed pack's provenance is checkable rather than assumed.

## Interfaces

### Command line

Built for scripted runs and CI. The exit status is the verification contract.

| Command | Purpose |
| :--- | :--- |
| `rspice run` | Execute the analyses a netlist requests |
| `rspice study` | Check, plan and run a versioned study over named analysis tasks |
| `rspice check` | Validate syntax and connectivity |
| `rspice info` | Print parsed netlist information |
| `rspice models` | List the shipped SPICE model packs and look up parts in them |
| `rspice compare` | Compare output against a golden result |
| `rspice convert` | Convert between raw, ASCII raw, CSV, TSV, JSON, HDF5 and VCD |
| `rspice health` | Probe process liveness or parser-to-solver readiness |
| `rspice capabilities` | Describe the execution routes and result representations this build supports |
| `rspice compile-va` | Compile a Verilog-A model |
| `rspice completions` | Generate shell completion scripts |

| Exit code | Meaning |
| :--- | :--- |
| `0` | Success |
| `1` | A failure with no engine category: a Verilog-A compile or format-conversion error |
| `2` | Invalid arguments |
| `3` | Verification failure: a `.MEAS` goal missed, or a golden mismatch |
| `65` / `66` | Malformed input / input not found |
| `69` | The deck is well formed and this build does not execute it |
| `70` / `73` / `74` | Internal error / artifact not published / I/O error |
| `75` / `76` / `78` | Resource budget exceeded / incompatible artifact version / configuration error |
| `80`–`85` | Engine failures: construction, solver, convergence, missing signal, schema mismatch, plan mismatch |
| `124` | `--timeout` exceeded |
| `130` | Interrupted |

```bash
# Quiet batch run with a JUnit report and a time budget
rspice run circuit.sp -q --report-format junit --report-file results.xml --timeout 600

# Deployment readiness probe (versioned JSON, nonzero when not ready)
rspice health --json

# Compare against a golden file, then accept a reviewed change
rspice compare results.csv golden.csv --abstol 1e-9 --reltol 1e-6
rspice compare results.csv golden.csv --bless
```

Runs can emit JUnit or TAP reports, a versioned JSON run summary (`--summary`),
and machine-readable measurement files. Production wrappers can select
newline-delimited JSON logs with `--log-format json` and structured fatal
diagnostics with `--error-format json`. The full manual, which also ships in
every native release package:
[crates/rspice-cli/README.md](crates/rspice-cli/README.md).

### Desktop and browser application

An egui application with a wgpu renderer, organized as eight workspaces
(Project, Design, Simulate, Stimulus Library, Results, Verify, Models, and
Netlist & Script Editor) covering schematic and symbol editing, analysis setup,
reusable stimulus definitions, waveform and RF result views, verification
evidence, model and PDK binding, and direct netlist editing. Desktop, browser
and tablet layouts share one workbench state and command registry; the browser
build is an experimental IDE that needs a WebGPU-capable browser.

The application also hosts the surfaces built around a run rather than inside
it: signed model packs, governed Python automation, published figure and report
bundles, organization-managed drawing sheets, and an optional cloud worker
client.

```bash
cargo run --release -p rspice-app
```

### Python

```bash
cd crates/rspice-python
pip install maturin
maturin develop --release
```

```python
import rspice

# Netlist.parse takes statements only: no title line, so nothing is swallowed.
netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")
engine = rspice.Engine()
result = engine.run_dc_op(netlist)
print(result.voltage(1))  # 10.0
```

Transient and AC results come back as NumPy arrays. The crate README covers
installation, verification, export, errors and cancellation, the
maturin/pytest workflow CI uses, and the Windows `PYO3_PYTHON` override for
Microsoft Store interpreter aliases:
[crates/rspice-python/README.md](crates/rspice-python/README.md).

These bindings put RSpice inside your Python. The application has the reverse,
Python automation inside RSpice. The two are separate: automation runs on a
signed, application-local CPython staged into the release, never an
interpreter discovered from `PATH`, the registry, a virtualenv, or user
site-packages.

### Rust

`rspice-core` is the engine every other crate wraps. `Netlist::parse` is the
front door and `Engine` the orchestrator; `AbortSignal` lets a frontend cancel a
long run cooperatively.

```rust
use rspice_core::{Engine, Netlist};

// The first line of a SPICE deck is the title, never an element.
let netlist = Netlist::parse("divider\nV1 1 0 10\nR1 1 0 1k\n.end")?;
let result = Engine::default().run_dc_op(&netlist)?;
assert_eq!(result.voltage(1), 10.0);
```

Entry points, device routing and the feature-flag table:
[crates/rspice-core/README.md](crates/rspice-core/README.md).

### WebAssembly

`rspice-wasm` exposes netlist summaries and authored-deck execution to
JavaScript through `wasm-bindgen`, returning the same typed result document the
other frontends publish, under configurable resource limits and cooperative
deadlines: [crates/rspice-wasm/README.md](crates/rspice-wasm/README.md).

### Verilog-AMS

Verilog-A is the analog subset of Verilog-AMS, and RSpice treats them as the
one language they are: a single compiler against LRM 2.4, with `.va` and
`.vams` sources going through the same pipeline and no mode selector between
them.

`rspice-veriloga` compiles behavioral modules through parser, semantic
analysis, canonical IR, and either a bytecode VM or the RSpice-owned native JIT
(x86-64 hosts plus AArch64 on macOS, Linux and Windows). When native mode is
requested, construction is full JIT or a typed error, never a silent fall back
to the interpreter. The same crate owns the Rust backend that produces the
generated devices above. External models compile standalone with
`rspice compile-va`; examples live in [models/veriloga/](models/veriloga/).

The discrete half is the IEEE 1364-2005 digital subset: four-state (`0 1 x z`)
logic, gate primitives, blocking and non-blocking assignment, vectors, module
hierarchy, and clause 13 library maps and configurations, driven by an event
wheel alongside the analog solve. Real-number modeling has `wreal` nets and the
four resolved forms. Mixed-discipline boundaries follow LRM 2.4 clause 7
(discipline resolution, `connect_mode` auto-insertion, `resolveto` with
`exclude`, and `merged`/`split` segmentation), with the detail resolution mode
of Annex F.2.2 refused by name rather than approximated.

Digital and mixed runs export to VCD with `rspice run --format vcd`, one wire
per digital node and one N-bit vector per declared bus (`--expand-buses` writes
member scalars for readers that cannot take vectors).

## Validation

Correctness is measured at four levels: unit tests per crate, the integration
suites under each crate's `tests/`, oracle-replay fixtures for history-coupled
device runtimes, and the corpus harnesses under [tests/](tests/): ngspice and
Xyce alongside GF180MCU, ISCAS85, mixed-signal and paranoia suites. The digital
Verilog corpus is checked against Icarus Verilog and Verilator, two independent
implementations sharing no code with RSpice or with each other.

The **ngspice harness** runs the vendored `tests/ngspice/` suite deck by deck,
comparing row by row against ngspice reference output at 2% relative tolerance
with probe-aware absolute floors. Each deck runs in a watchdog-supervised
process, so one hung simulation cannot stall the suite. The **Xyce harness**
runs against the trimmed `tests/xyce/` runtime corpus, discovers every retained
`.cir`, records removed `.cir.sh` wrapper contracts in
`RSPICE-HARNESS-MANIFEST.tsv`, and reports unsupported Xyce contracts explicitly
instead of omitting the deck.

Every executed analysis must be backed by a validation oracle, so no deck passes
silently. Nightly release runs gate on the recorded failure watermark in
[.github/workflows/nightly.yml](.github/workflows/nightly.yml), which is zero:
any failing deck fails the run.

```bash
cargo test --release -p rspice-core                                   # unit + integration
cargo test --release -p rspice-conformance --test ngspice_regression  # ngspice suite
cargo test --release -p rspice-conformance --test xyce_regression     # Xyce corpus
```

Performance is tracked the same way. `rspice-bench` times whole simulator
processes against a locally installed ngspice over the shared decks in
[tools/rspice-bench/circuits/](tools/rspice-bench/circuits/).

## Under the hood

The engine is a modified-nodal-analysis formulation with a damped Newton
solver. It has a merit-based line search, four convergence aids for hard
circuits (gmin stepping, source stepping, pseudo-transient continuation and
arc-length continuation), and local-truncation-error control over the transient
timestep.

Real systems factor through an in-tree KLU-class sparse solver, whose stored
pivots keep refactorization cheap while the sparsity pattern holds, or through
[faer](https://crates.io/crates/faer), chosen by measured fill unless
`.OPTIONS SOLVER` says otherwise. Complex solves behind the AC-family analyses
use faer.

## Repository

**Engine**

| Crate | Purpose |
| :--- | :--- |
| [`rspice-core`](crates/rspice-core) | The simulation engine: parser, device models, solvers, analyses |
| [`rspice-matrix`](crates/rspice-matrix) | Sparse matrix storage and LU factorization |
| [`rspice-veriloga`](crates/rspice-veriloga) | Verilog-AMS compiler: bytecode VM, native and browser JITs, and the generator for the built-in models |
| [`rspice-veriloga-runtime`](crates/rspice-veriloga-runtime) | Runtime ABI the generated models link against |
| [`rspice-veriloga-models`](crates/rspice-veriloga-models) | The generated Verilog-A model catalog, one feature per model |
| [`rspice-units`](crates/rspice-units) | Measurement units shared by the engine and the result store |
| [`rspice-output`](crates/rspice-output) | Transactional publication of result files and checkpoints |

**Frontends**

| Crate | Purpose |
| :--- | :--- |
| [`rspice-cli`](crates/rspice-cli) | The `rspice` command-line simulator |
| [`rspice-app`](crates/rspice-app) | The desktop and browser application |
| [`rspice-python`](crates/rspice-python) | Python bindings built with PyO3 |
| [`rspice-wasm`](crates/rspice-wasm) | WebAssembly bindings and the browser playground |

**Application libraries**

| Crate | Purpose |
| :--- | :--- |
| [`rspice-app-types`](crates/rspice-app-types) | Portable value types every application crate shares |
| [`rspice-design`](crates/rspice-design) | Schematic documents, connectivity, hierarchy, rule checks and symbols |
| [`rspice-design-model`](crates/rspice-design-model) | Design-management data and the signed drawing-sheet package |
| [`rspice-schematic-editor`](crates/rspice-schematic-editor) | Schematic and symbol editor painting and interaction |
| [`rspice-ui-kit`](crates/rspice-ui-kit) | Design tokens, widgets and the plot engine |
| [`rspice-project`](crates/rspice-project) | The project file, its workspace and the save lifecycle |
| [`rspice-project-contract`](crates/rspice-project-contract) | Project identity, technology binding and library catalog records |
| [`rspice-simulation`](crates/rspice-simulation) | Source preparation, scheduling and engine adapters for the application |
| [`rspice-simulation-contract`](crates/rspice-simulation-contract) | Simulation requests and the application's analysis vocabulary |
| [`rspice-worker`](crates/rspice-worker) | The browser module worker that runs simulations off the UI thread |
| [`rspice-results`](crates/rspice-results) | Exact result data, calculations and report documents |
| [`rspice-results-ui`](crates/rspice-results-ui) | The result viewers |
| [`rspice-formats`](crates/rspice-formats) | Result file codecs and project result persistence |
| [`rspice-model-library`](crates/rspice-model-library) | Model and PDK libraries, bindings and publisher trust |
| [`rspice-hardcopy`](crates/rspice-hardcopy) | Print and export rendering of schematics, plots and reports |
| [`rspice-hardcopy-contract`](crates/rspice-hardcopy-contract) | Page setup, print plans and the receipts a project retains |

**Automation, cloud and publication**

| Crate | Purpose |
| :--- | :--- |
| [`rspice-automation-protocol`](crates/rspice-automation-protocol) | The governed Python automation worker protocol |
| [`rspice-automation-runtime`](crates/rspice-automation-runtime) | Verified managed Python runtime and worker transport |
| [`rspice-engine-adapter`](crates/rspice-engine-adapter) | Engine executor for the cloud worker protocol |
| [`rspice-pack`](crates/rspice-pack) | Signed, immutable model pack format |
| [`rspice-cloud-domain`](crates/rspice-cloud-domain), [`rspice-cloud-contract`](crates/rspice-cloud-contract), [`rspice-cloud-client`](crates/rspice-cloud-client) | The RSpice Cloud client, vendored from its own repository |
| [`rspice-publication-contract`](crates/rspice-publication-contract) | Versioned schema for sealed circuit publications |
| [`rspice-publish`](crates/rspice-publish) | Renderer from publication snapshots to page bundles |
| [`rspice-viewer`](crates/rspice-viewer) | Figure runtime for published pages |

**Tools** under `tools/`: `rspice-conformance` (reference-simulator corpus
harnesses), `rspice-bench` (performance benchmarks), `rspice-sheet-publisher`
(offline drawing-sheet signing), and the `xspice-catalog-dump` and
`veriloga-catalog-dump` registry audits.

[models/](models/) holds the bundled SPICE and Verilog-A model sources,
including the redistributable CMC packages; [tests/](tests/) holds the
vendored simulator corpora with their manifests and notices.

## Platforms

CI builds Windows, macOS and Linux on x86-64 and AArch64, Android and iOS on
AArch64, and `wasm32-unknown-unknown` for the browser. Maturity and validation
depth vary by subsystem: a `.TRAN` on a BSIM4 deck rests on far more evidence
than a periodic-stability run. Browser use on phones and tablets is not a
qualified release target.

## License

RSpice is source-available under the
[RSpice Personal Use License](LICENSE): personal, educational and open academic
use are permitted; commercial use requires a separate license. See
[NOTICE](NOTICE) for third-party attributions and [SECURITY.md](SECURITY.md)
for vulnerability reporting.

## Acknowledgments

Some device models derive from BSD-licensed ngspice 46 source, and native BSIM4
acknowledges the UC Berkeley BSIM Research Group under the upstream BSIM4
terms. Compact models come from the Compact Model Coalition; the Xyce
Regression Suite is vendored under GPL terms. Full attributions and license
texts: [NOTICE](NOTICE).
