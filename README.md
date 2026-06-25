<div align="center">

<img src="assets/logo.svg" alt="RSpice logo" width="140" />

# RSpice

**An analog circuit simulator written in Rust.**

SPICE-compatible netlists, validated against ngspice — with a CLI, a desktop UI,
Python and WebAssembly bindings, and a Verilog-A compiler.

[![License](https://img.shields.io/badge/license-source--available-informational?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.94-orange?style=flat-square)](rust-toolchain.toml)
[![Platform](https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-lightgrey?style=flat-square)](https://github.com/JaimeHW/RSpice)

</div>

## Overview

RSpice simulates analog and mixed-signal circuits described as SPICE netlists. The engine assembles modified-nodal-analysis systems and solves them with a damped Newton iteration — merit-based line search, gmin and source stepping, pseudo-transient continuation — under an adaptive-timestep transient loop with local-truncation-error control. The real-valued path defaults to an in-tree KLU-class sparse solver, while [faer](https://crates.io/crates/faer) backs complex/AC-family sparse solves; AC sweeps and Monte Carlo runs parallelize across cores with rayon, and the hottest device-evaluation paths have optional SIMD batch implementations.

Around the engine sit a CLI built for batch runs and CI, a desktop application for schematic capture and waveform inspection, and Python and WebAssembly bindings; a Verilog-A compiler brings behavioral models to the runtimes where that pipeline is enabled.

## Status

RSpice is a young project under active development. The surface area below is broad, and all of it exists in code and is exercised by tests — but maturity varies between areas, and accuracy is measured against ngspice continuously rather than assumed. It is not yet a substitute for hardened EDA tooling.

Platform support is tracked by evidence in [docs/platform-support.md](docs/platform-support.md); mobile/tablet browser use and signed release artifacts are still experimental or launch-only until their gates exist.

## Quick start

The toolchain is pinned to Rust 1.94 via [rust-toolchain.toml](rust-toolchain.toml); rustup picks it up automatically.

```bash
git clone https://github.com/JaimeHW/RSpice.git
cd RSpice
cargo build --release -p rspice-cli   # the first release build takes a few minutes (fat LTO)
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
✓ Transient complete: 1053 time points computed
  Measurement Results (TRAN, 2):
    VPEAK = 4.999773e+00
    RISETIME = 2.197215e-04

Simulation complete in 0.001s.
```

Results can be written to a file instead — SPICE raw, CSV, TSV, JSON, or HDF5:

```bash
target/release/rspice run rc_lowpass.sp -o rc.h5 --format hdf5
```

`cargo install --path crates/rspice-cli` puts `rspice` on your `PATH`.

## What's implemented

### Analyses

| Domain | Analyses |
| :--- | :--- |
| Operating point & sweeps | `.OP`, `.DC` (including two-source sweeps), temperature sweeps, `.STEP` |
| Time domain | `.TRAN`, with LTE-controlled adaptive timestepping and checkpoint/resume segmentation |
| Small-signal | `.AC`, `.NOISE`, `.PZ`, `.TF`, `.SENS` |
| Statistical | Operating-point Monte Carlo parameter variation, process corners † |
| Periodic / RF | Periodic steady state (shooting), harmonic balance, two-port S-parameters with Touchstone export † |
| Post-processing | `.MEAS` over TRAN/DC/AC/NOISE with `GOAL`/`TOL` pass-fail gating, `.FOUR`; THD/IMD, eye-diagram, and jitter metrics † |

† selected through a mix of deck cards and CLI/IDE surfaces: `.mc` and `.stb` have deck-card paths, while Monte Carlo, process corners, PSS, HB, and S-parameters are also exposed through CLI flags such as `--monte-carlo`, `--corners`, `--pss-freq`, `--hb-freq`, and `--sparam`. `.DISTO` cards are parsed for compatibility and currently run the matching small-signal AC sweep; there is no dedicated Volterra distortion engine yet, so distortion figures come from THD/IMD post-processing. PAC, PNoise, PXF, PSTB, envelope, and multi-rate ship as engine-level mathematics (conversion-matrix and monodromy kernels) without a circuit-extraction layer yet; they are not end-to-end analyses and are not claimed as such.

### Devices

| Family | Models |
| :--- | :--- |
| MOSFET | Native BSIM4 v4.8 (`LEVEL=14/54`, canonical mode set), BSIM3v3.3 (`LEVEL=8/9/49`, `CAPMOD=2/3`), BSIM3-SOI (FD / DD / PD), EKV, VDMOS, Berkeley MOS1/MOS2/MOS3/MOS6, and legacy BSIM1/BSIM2; unsupported MOS levels fail closed until native support and validation are added |
| Bipolar | Gummel-Poon BJT (default / `LEVEL=1`) and native VBIC (`LEVEL=4`, including excess phase); other advanced BJT levels fail explicitly until their native models land |
| Junction | Diode, JFET level 1 and native Parker-Skellern JFET2 (`NJF`/`PJF LEVEL=2`, default/best-available), an internal Xyce modified-Shockley JFET2 compatibility mode, MES/MESA/HFET-family `Z` devices, GaN HEMT |
| Passives | R / C / L with temperature coefficients, coupled inductors and multi-winding transformers, saturable inductor (Jiles–Atherton hysteresis) |
| Transmission lines | Ideal, lossy (LTRA, TXL), coupled (CPL) |
| Sources | Independent V/I with `PULSE`, `SIN`, `EXP`, `PWL`, `SFFM`, `AM`, and `TRNOISE` white + 1/f waveforms; E/F/G/H controlled sources; B behavioral sources; PWL file sources |
| Switches & macromodels | Voltage- and current-controlled switches, op-amp macromodel |
| Mixed-signal | XSPICE-style analog/digital elements, tri-state drivers, A/D–D/A bridges |
| Verilog-A | Compiled behavioral modules (below) |

CMC compact-model families that ship redistributable Verilog-A sources under
`models/veriloga/cmc/` are no longer planned as hand-maintained native ports.
The strategic native path for those devices is generated Rust from the upstream
Verilog-A source, produced by the planned Verilog-A to Rust transpiler. Any
historical hand-native CMC experiments should be treated as compatibility or
reference work only; active CMC device coverage should come from generated
implementations.

BSIM-class models fail with typed errors when a model card requests unported
physics such as BSIM4 gate/body resistance networks, NQS, material-mode
effects, or unsupported mode selectors. BSIM4 `RDSMOD=0/1` source/drain
resistance paths are native, including `RGEOMOD=1..8` implicit S/D resistance
geometry when `NRD`/`NRS` are omitted. That is deliberate: a commercial simulator should
reject unsupported physics rather than silently produce plausible but wrong
currents. The GaN HEMT path is an in-tree physics-style model; CMC ASM-HEMT /
MVSG qualification remains roadmap work.

### Netlist dialect

`.SUBCKT` subcircuits (flattened during elaboration), `.PARAM` with expression evaluation, `.INCLUDE` and `.LIB`, `.OPTIONS`, `.TEMP`, and the usual engineering suffixes. Starter device libraries ship under [models/](models/).

## Interfaces

### Command line

The CLI is built for scripted runs and CI: it executes the analyses a netlist requests, validates and inspects netlists, converts between output formats, and compares results against golden references. The exit status is the verification contract — failed `.MEAS` checks exit 3, non-finite results exit 1, comparison mismatches exit 3, `--timeout` overruns exit 124 — and runs can emit JUnit or TAP reports, a JSON run summary (`--summary`), and machine-readable measurement files.

| Command | Description |
| :--- | :--- |
| `rspice run` | Execute simulations |
| `rspice info` | Print parsed netlist information |
| `rspice check` | Validate syntax and connectivity |
| `rspice compare` | Compare output against a golden result |
| `rspice convert` | Convert between RAW, ASCII RAW, CSV, JSON, TSV, and HDF5 |
| `rspice compile-va` | Compile Verilog-A models |
| `rspice completions` | Generate shell completion scripts |

```bash
# Quiet batch run with a JUnit report and a time budget
rspice run circuit.sp -q --report-format junit --report-file results.xml --timeout 600

# Compare results to a golden file (mismatches exit 3)
rspice compare results.csv golden.csv --abstol 1e-9 --reltol 1e-6

# Accept a reviewed waveform change as the new reference
rspice compare results.csv golden.csv --bless
```

Full command and option reference: [crates/rspice-cli/README.md](crates/rspice-cli/README.md).

### Desktop UI

Schematic capture with a component palette, waveform inspection, and analysis views (harmonic-balance tones, phase noise), built on egui with a wgpu renderer:

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

netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")
engine = rspice.Engine()
result = engine.run_dc_op(netlist)
print(result.voltage(1))
```

Transient and AC results come back as NumPy arrays. Full API reference: [crates/rspice-python/README.md](crates/rspice-python/README.md). A CI-ready pytest pattern — circuit and `.MEAS` criteria asserted from a test — lives in [examples/python/](examples/python/) and runs in this repo's own CI.

For local checks, keep the fast Rust workspace pass separate from the PyO3
extension and wasm target:

```bash
cargo check --workspace --exclude rspice-python --exclude rspice-wasm
cargo check -p rspice-python
```

On Windows, if `python` or `python3` resolves to the Microsoft Store alias,
PyO3 may fail while checking `rspice-python`. Point it at a real interpreter
explicitly:

```powershell
py -3 -m venv .venv
$env:PYO3_PYTHON = (Resolve-Path .\.venv\Scripts\python.exe).Path
cargo check -p rspice-python
```

For binding tests, prefer the maturin/pytest workflow used by CI:

```bash
cd crates/rspice-python
python -m pip install maturin numpy pytest
maturin develop --release
python -m pytest tests/ -v
```

`cargo test -p rspice-python --lib` is useful only for Rust-side library
coverage; it is not a substitute for importing the extension in pytest.

### WebAssembly

`rspice-wasm` exposes netlist summaries plus DC operating-point, AC, and transient runs to JavaScript through `wasm-bindgen`, returning JSON-serializable result snapshots.

### Verilog-A

`rspice-veriloga` compiles behavioral modules through a parser → semantic analysis → VM pipeline, with optional native code generation via a Cranelift JIT (the `native` feature). The CMC-model roadmap extends this front end with a Verilog-A to Rust transpiler so packages under [models/veriloga/cmc/](models/veriloga/cmc/) can become generated native Rust devices instead of hand-written ports. Models compile standalone with `rspice compile-va`; examples live in [models/veriloga/](models/veriloga/).

## Validation

Correctness is measured rather than assumed, at four levels: unit tests in each crate, integration tests, oracle-replay fixtures for history-coupled device runtimes, and a regression harness that runs the vendored ngspice test suite deck-by-deck against the RSpice engine — comparing row-by-row against ngspice's reference outputs at 2% relative tolerance with probe-aware absolute floors. Every executed analysis must be backed by a validation oracle, so no deck can pass silently, and each deck runs in a watchdog-supervised process so a hung simulation cannot stall the suite.

```bash
cargo test --release -p rspice-core                            # unit + integration
cargo test --release -p rspice-core --test ngspice_regression  # ngspice suite
```

The harness design — oracle-replay methodology, comparison gating, debug environment variables — is documented in [docs/testing.md](docs/testing.md).

## Workspace

| Crate | Purpose |
| :--- | :--- |
| `rspice-core` | Simulation engine: device models, analyses, netlist parser, validation harnesses |
| `rspice-cli` | Command-line interface for simulation, validation, conversion, and reporting |
| `rspice-ui` | Desktop application for schematic editing and waveform inspection |
| `rspice-veriloga` | Verilog-A parser, semantic pipeline, VM runtime, optional native codegen |
| `rspice-python` | Python bindings built with PyO3 |
| `rspice-wasm` | WebAssembly bindings for the simulation engine |
| `rspice-bench` | Whole-process benchmark rig against local ngspice |

Beyond the crates: [models/](models/) holds starter SPICE and Verilog-A libraries, [tests/](tests/) the vendored ngspice test suite and validation manifest, [examples/](examples/) CI-ready usage patterns, [benchmarks/](benchmarks/) the macro-benchmark decks and published scoreboards, and [docs/](docs/) the user manual, testing methodology, and roadmap.

## License

RSpice is source-available software under the [RSpice Personal Use License](LICENSE): personal, educational, and open academic use are permitted; commercial use requires a separate license. See the license text for details, and [NOTICE](NOTICE) for third-party attributions.

## Acknowledgments

RSpice's device models and transient engine owe a great deal to [ngspice](https://ngspice.sourceforge.io/): several models are ported from BSD-licensed portions of ngspice 46, native BSIM4 acknowledges the UC Berkeley BSIM Research Group under the upstream BSIM4 terms, and the ngspice test suite is RSpice's primary accuracy reference. Sparse linear algebra uses the in-tree KLU-class real solver plus [faer](https://crates.io/crates/faer) for complex/AC-family paths.
