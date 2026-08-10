<div align="center">

<img src="assets/brand/logo.svg" alt="RSpice logo" width="140" />

# RSpice

**An analog and mixed-signal circuit simulator written in Rust.**

SPICE netlists, measured continuously against ngspice and Xyce, with a CLI, a
desktop IDE, Python and WebAssembly bindings, and a Verilog-A compiler.

[![License](https://img.shields.io/badge/license-source--available-informational?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.94-orange?style=flat-square)](rust-toolchain.toml)
[![Platform](https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-lightgrey?style=flat-square)](https://github.com/JaimeHW/RSpice)

[Quick start](#quick-start) · [Analyses](#analyses) · [Devices](#devices) ·
[Dialect](#netlist-dialect) · [Interfaces](#interfaces) ·
[Validation](#validation) · [Status](#status)

</div>

---

RSpice assembles modified-nodal-analysis systems and solves them with a damped
Newton iteration — merit-based line search, gmin and source stepping,
pseudo-transient and arc-length continuation — under an adaptive-timestep
transient loop with local-truncation-error control. Real-valued factorization
uses an in-tree KLU-class sparse solver whose stored pivots make refactorization
on a frozen sparsity pattern cheap; [faer](https://crates.io/crates/faer) backs
the complex solves for AC-family analyses.

Where physics is unported, RSpice raises a typed error naming the parameter or
mode selector rather than falling back to an approximation. A model card that
would silently produce plausible but wrong currents fails closed instead.

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
✓ Transient complete: 1029 time points computed
  Measurement Results (TRAN, 2):
    VPEAK = 4.999773e+00
    RISETIME = 2.197211e-04

Simulation complete in 0.003s.
```

Write results to a file instead — SPICE raw, ASCII raw, CSV, TSV, JSON, or HDF5:

```bash
target/release/rspice run rc_lowpass.sp -o rc.h5 --format hdf5
```

`cargo install --path crates/rspice-cli` puts `rspice` on your `PATH`.

## Analyses

Every analysis below runs end to end on a real circuit. The second column is the
complete set of ways to reach it.

| Analysis | Invoked by | Notes |
| :--- | :--- | :--- |
| Operating point | `.OP` | |
| DC sweep | `.DC` | single source, or nested two-source |
| Temperature sweep | `.TEMP` | list of temperatures |
| Parametric sweep | `.STEP` | parameter, device, or `DATA=` table |
| Transient | `.TRAN` | LTE-controlled timestep, breakpoint handling, `--checkpoint`/`--resume` segmentation |
| AC small-signal | `.AC` | sweep parallelized across cores |
| Noise | `.NOISE` | |
| Pole-zero | `.PZ` | |
| Transfer function | `.TF` | |
| Sensitivity | `.SENS` | DC and AC |
| Distortion | `.DISTO` | third-order Volterra; harmonic and two-tone intermodulation |
| Fourier / THD | `.FOUR` | |
| Monte Carlo | `.MC`, `--monte-carlo` | operating-point parameter variation; gaussian, uniform, or worst-case |
| Process corners | `--corners` | corner definitions via `--corner-lib` |
| Harmonic balance | `.HB`, `--hb-freq` | with envelope continuation |
| Periodic steady state | `--pss-freq` | shooting method |
| S-parameters | `.SP`, `--sparam` | Touchstone export; `--sparam` drives two ports |
| Stability (loop gain) | `.STB` | probes a 0 V source placed in the loop |
| Periodic AC | `Engine::run_pac`, IDE | conversion matrix around the PSS/HB solution |
| Phase noise | `Engine::run_pnoise`, IDE | Floquet projection; PPV for oscillators |
| Periodic transfer function | IDE | composed on PSS |
| Periodic stability | IDE | Floquet multipliers from the monodromy matrix |
| Measurements | `.MEAS` | over TRAN/DC/AC/NOISE; `GOAL`/`TOL` gates the exit status |

`.FFT` parses but is rejected at run time until its post-processing lands.

## Devices

The executable engine, schematic-editor, symbol, generated-model, and target
release contracts are tracked in the
[device support and release matrix](DEVICE_SUPPORT_RELEASE_MATRIX.md).

| Family | Models |
| :--- | :--- |
| MOSFET | Every accepted `M`-card level is native: BSIM4 v4.8 (`LEVEL=14/54`), BSIM3v3.3 (`LEVEL=8/49` plus BSIM3-shaped `LEVEL=9`, `CAPMOD=2/3`), BSIM-SOI in DD/FD/PD variants (`LEVEL=10/55/56/57`), VDMOS (`LEVEL=18`), EKV 2.6 (`LEVEL=260`) and EKV3 (`LEVEL=301`), Berkeley MOS1/MOS2/MOS3/MOS6 and ngspice MOS9, legacy BSIM1/BSIM2 (`LEVEL=4/5`) |
| Bipolar | Native Gummel-Poon (`LEVEL=0/1/2`) and native VBIC 1.3 (`LEVEL=4/9/11/12/13`) |
| Junction | Diode, JFET level 1, native Parker-Skellern JFET2 (`NJF`/`PJF LEVEL=2`) with an internal Xyce modified-Shockley compatibility mode, MES/MESA/HFET-family `Z` devices, GaN HEMT |
| Passives | R / C / L with temperature coefficients, coupled inductors and multi-winding transformers, saturable inductor with Jiles–Atherton hysteresis |
| Transmission lines | Ideal, lossy (LTRA, TXL), coupled multi-conductor (CPL) |
| Sources | Independent V/I with `PULSE`, `SIN`, `EXP`, `PWL`, `PAT`, `SFFM`, `AM`, and `TRNOISE` white + 1/f waveforms; E/F/G/H controlled sources; B behavioral sources; PWL-from-file sources |
| Switches & macromodels | Voltage- and current-controlled switches, op-amp macromodel |
| Mixed-signal | XSPICE-style analog and digital code models, tri-state drivers, A/D–D/A bridges |
| Verilog-A | Generated CMC devices and externally compiled modules — below |

Unlisted `M` levels fail closed rather than falling through to the simplified
MOS approximation. HICUM/L0, HICUM/L2, MEXTRAM, and Xyce HBT_X `Q` levels are
rejected by name; those families are reachable as generated Verilog-A devices.

BSIM-class cards raise typed errors when they request unported physics —
distributed gate and body resistance networks, NQS, material-mode effects,
unknown charge paths. BSIM4 `RDSMOD=0/1` source/drain resistance is native,
including `RGEOMOD=1..8` implicit geometry when `NRD`/`NRS` are omitted, as are
`GEOMOD=0..10`, `WPEMOD=1`, gate tunneling, stress layout correction, and
`DIOMOD=0/1/2`.

### Generated Verilog-A devices

CMC compact-model families with redistributable sources under
[models/veriloga/cmc/](models/veriloga/cmc/) are not hand-ported. They are
generated to Rust from the upstream Verilog-A and checked in under
`crates/rspice-veriloga-models/models/` — one reusable Cargo artifact per
model, 42 devices today, among
them ASM-HEMT, BSIM-BULK, BSIM-CMG, BSIM-IMG, BSIM-SOI, DIODE_CMC, HICUM/L0
and /L2, HiSIM-HV, HiSIM-SOI, JUNCAP200, L-UTSOI, MEXTRAM 505, MVSG-CMC,
PSP104, and VBIC 1.3.

A generated device is instantiated by its module name on an `X` line, not
through an `M`/`Q` `LEVEL` selector, and each compiles in only when its
`veriloga-model-*` feature is enabled — granular features keep compile time,
peak rustc memory, and binary size proportional to the models you actually use.
Where a bundled source exists the generated device is the canonical
implementation; a hand-written native port of the same family serves the
`LEVEL`-card decks that reach it. The generated ASM-HEMT and MVSG-CMC devices
are present but not yet oracle-qualified; the in-tree `Z`-device GaN HEMT is a
physics-style model, not a CMC one.

## Netlist dialect

`.SUBCKT` subcircuits are flattened during elaboration, with hierarchical path
handling and scoped parameters. `.PARAM`/`.CSPARAM` and `.FUNC` evaluate through
a bytecode expression VM that also backs B-sources; `.IF`/`.ELSEIF`/`.ELSE`/`.ENDIF`
select at parse time. `.INCLUDE`, `.LIB`, and `.MODEL` bring in model cards —
the model library ships under [models/spice/](models/spice/), where four open
foundry PDKs sit alongside academic, community and manufacturer packs for some
198,000 model cards, and a starter set for diodes, MOSFETs, transistors and
op-amps is compiled into the binary from
[models/spice/builtin/](models/spice/builtin/). `.GLOBAL`,
`.IC`, `.NODESET`, `.SAVE`/`.PROBE`, `.PRINT`/`.PLOT`, `.OPTIONS`, and `.TEMP`
behave as expected, and the usual engineering suffixes are accepted. Unrecognized
dot-commands surface as diagnostics rather than being silently dropped.

Beyond plain SPICE, the parser ingests SPEF (IEEE 1481) parasitics as
back-annotation onto a parsed netlist, XSPICE code-model cards, and Laplace-defined
sources. LTspice `.raw` files can be read back for comparison.

## Interfaces

### Command line

Built for scripted runs and CI. The exit status is the verification contract.

| Command | Purpose |
| :--- | :--- |
| `rspice run` | Execute the analyses a netlist requests |
| `rspice check` | Validate syntax and connectivity |
| `rspice info` | Print parsed netlist information |
| `rspice compare` | Compare output against a golden result |
| `rspice convert` | Convert between raw, ASCII raw, CSV, TSV, JSON, and HDF5 |
| `rspice health` | Probe process liveness or parser-to-solver readiness |
| `rspice compile-va` | Compile a Verilog-A model |
| `rspice completions` | Generate shell completion scripts |

| Exit code | Meaning |
| :--- | :--- |
| `0` | Success |
| `1` | General error, including non-finite results |
| `2` | Invalid arguments |
| `3` | Verification failure — a `.MEAS` goal missed, or a golden mismatch |
| `65` / `66` | Malformed input / input not found |
| `70` / `74` / `78` | Internal, I/O, or configuration error |
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
diagnostics with `--error-format json`. Full option reference:
[crates/rspice-cli/README.md](crates/rspice-cli/README.md).

### Desktop IDE

An egui application with a wgpu renderer, organized as seven workspaces —
Project, Design, Simulate, Results, Verify, Models, and Netlist — covering
schematic and symbol editing, analysis-plan setup, waveform and RF result views,
verification evidence, model and PDK binding, and direct netlist editing. Its
simulation runner is the only surface that reaches PXF, PSTB, SOA, and
reliability analyses.

```bash
cargo run --release -p rspice-ui
```

### Python

```bash
cd crates/rspice-python
pip install maturin
maturin develop --release
```

```python
import rspice

# Netlist.parse takes statements only — no title line, so nothing is swallowed.
netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")
engine = rspice.Engine()
result = engine.run_dc_op(netlist)
print(result.voltage(1))  # 10.0
```

Transient and AC results come back as NumPy arrays. The crate README carries the
full API reference, the maturin/pytest workflow CI uses, and the Windows
`PYO3_PYTHON` workaround for Microsoft Store interpreter aliases:
[crates/rspice-python/README.md](crates/rspice-python/README.md).

### WebAssembly

`rspice-wasm` exposes netlist summaries, DC operating-point, AC, and transient
runs to JavaScript through `wasm-bindgen`, returning JSON-serializable snapshots
under configurable resource limits.

### Verilog-A

`rspice-veriloga` compiles behavioral modules through parser, semantic analysis,
canonical IR, and either a bytecode VM or the RSpice-owned native JIT (x86-64
hosts plus AArch64 on macOS, Linux, and Windows). When native mode is requested, construction is full JIT or a typed
error — never a silent fall back to the interpreter. The same crate owns the
Rust backend that produces the generated built-in devices above. External models
compile standalone with `rspice compile-va`; examples live in
[models/veriloga/](models/veriloga/).

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

API details and the feature-flag matrix: [crates/rspice-core/README.md](crates/rspice-core/README.md).

## Validation

Correctness is measured at four levels: unit tests per crate, 107 integration
test files in `rspice-core` alone, oracle-replay fixtures for history-coupled
device runtimes, and two simulator corpus harnesses.

The **ngspice harness** runs the vendored `tests/ngspice/` suite deck by deck,
comparing row by row against ngspice reference output at 2% relative tolerance
with probe-aware absolute floors. Each deck runs in a watchdog-supervised
process, so one hung simulation cannot stall the suite. The **Xyce harness**
runs against the trimmed `tests/xyce/` runtime corpus, discovers every retained
`.cir`, records removed `.cir.sh` wrapper contracts in
`RSPICE-HARNESS-MANIFEST.tsv`, and reports unsupported Xyce contracts explicitly
instead of omitting the deck.

Every executed analysis must be backed by a validation oracle, so no deck passes
silently. Nightly release runs ratchet against the recorded failure watermark in
[.github/workflows/nightly.yml](.github/workflows/nightly.yml); that number only
tightens as decks are fixed.

```bash
cargo test --release -p rspice-core                            # unit + integration
cargo test --release -p rspice-core --test ngspice_regression  # ngspice suite
cargo test --release -p rspice-core --test xyce_regression     # Xyce corpus
```

Performance is tracked the same way. `rspice-bench` times whole simulator
processes against a locally installed ngspice over the shared decks in
[benchmarks/circuits/](benchmarks/circuits/); no optimization claim lands without
a before/after scoreboard.

## Operations

The native backend is a stateless, one-shot worker process. Deployments pin
admission limits through the `[resources]` table of an operator-supplied config
file, or the matching `RSPICE_*` variables; both are documented per key in
[the CLI reference](crates/rspice-cli/README.md). Derive the ceilings from the
deployment's own deck mix and container quotas — the defaults assume a
workstation, not an isolated worker.

Releases are cut from annotated version tags for six Linux, macOS, and Windows
target triples, bound to deterministic archives, SHA-256 sidecars, CycloneDX
SBOMs, and GitHub artifact attestations, and published without permitting an
existing asset to be replaced. Archives carry a manifest recording the exact
source commit and payload hashes.

## Repository

| Crate | Purpose |
| :--- | :--- |
| `rspice-core` | Simulation engine: parser, device models, solvers, analyses, validation harnesses |
| `rspice-cli` | Command-line interface for simulation, validation, conversion, and reporting |
| `rspice-ui` | Desktop IDE for schematic capture, simulation setup, and result analysis |
| `rspice-veriloga` | Verilog-A parser, semantic pipeline, bytecode VM, native JIT, generated-Rust backend |
| `rspice-python` | Python bindings built with PyO3 |
| `rspice-wasm` | WebAssembly bindings for the engine |
| `rspice-bench` | Whole-process benchmark rig against local ngspice |

[models/](models/) holds the bundled Verilog-A sources including the
redistributable CMC packages, [tests/](tests/) the vendored simulator corpora
with their manifests and notices, and [benchmarks/](benchmarks/) the
macro-benchmark decks and published scoreboards.

## Status

RSpice is a young project under active development. The surface area above is
broad and every entry is implemented and exercised, but maturity and validation
depth vary by subsystem: a `.TRAN` on a BSIM4 deck rests on far more evidence
than a periodic-stability run. Accuracy is measured against reference simulators
continuously rather than assumed, and the repository should not be presented as
a substitute for hardened commercial EDA tooling.

Platform support is tracked by CI coverage and release gates across the six
target triples above. Mobile and tablet browser use remains launch-gated
separately.

## License

RSpice is source-available under the
[RSpice Personal Use License](LICENSE): personal, educational, and open academic
use are permitted; commercial use requires a separate license. See [NOTICE](NOTICE)
for third-party attributions and [SECURITY.md](SECURITY.md) for vulnerability
reporting.

## Acknowledgments

RSpice's device models and transient engine owe a great deal to
[ngspice](https://ngspice.sourceforge.io/): several models are ported from
BSD-licensed portions of ngspice 46, native BSIM4 acknowledges the UC Berkeley
BSIM Research Group under the upstream BSIM4 terms, and the ngspice test suite is
RSpice's primary accuracy reference. The Xyce Regression Suite is vendored under
its GPL terms and drives the second corpus harness. Compact models come from the
Compact Model Coalition under their respective package licenses. Sparse linear
algebra uses the in-tree KLU-class real solver plus
[faer](https://crates.io/crates/faer) for complex and AC-family paths.
