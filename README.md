<div align="center">

<img src="assets/logo.svg" alt="RSpice Logo" width="180" />

# RSpice
**Rust-based SPICE simulation workspace for analog and mixed-signal design**

[![License](https://img.shields.io/badge/license-Source%20Available-red.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-win%20%7C%20macos%20%7C%20linux-lightgrey.svg?style=flat-square)](https://github.com/JaimeHW/rspice)

<img src="assets/image.png" alt="RSpice Studio schematic and waveform view" width="100%" />
<img src="assets/image2.png" alt="RSpice Studio analysis view" width="100%" />

</div>

## Overview

RSpice is a Rust workspace for circuit simulation and design exploration. It combines a SPICE engine, a desktop UI, a command-line workflow, Python bindings, and a Verilog-A toolchain in one repository.

The project is aimed at practical analog and mixed-signal work: release-oriented execution, regression-driven validation, broad device-model coverage, and tooling that supports both scripted flows and interactive debugging.

## Project Status

RSpice is in active development.

- The repository already contains a broad implementation surface: core analyses, device models, CLI workflows, a desktop UI, Python bindings, and Verilog-A infrastructure.
- The workspace includes extensive automated tests and ngspice-based regression coverage.
- The breadth of implemented functionality is ahead of product polish in some areas.
- It is not yet a drop-in replacement for mature commercial EDA suites.

This README is intentionally scoped to what is implemented in the repository today.

## Highlights

- Rust-native simulation engine with sparse linear algebra, adaptive transient stepping, convergence controls, and parameterized analyses.
- Verilog-A compiler pipeline with VM execution and optional Cranelift JIT support.
- Desktop application for schematic-driven workflows and waveform inspection.
- CLI suitable for scripted runs, regression checks, reporting, and format conversion.
- Python bindings for automation and post-processing.
- Built-in HDF5 waveform storage for large result sets without external native HDF5 dependencies.

## Capabilities

The workspace includes implementations for the following analysis families. Frontend coverage and workflow polish vary by crate.

### Analyses

- DC operating point and DC sweep
- Transient
- AC small-signal
- Noise
- Fourier / `.FOUR`
- Temperature sweep
- Monte Carlo
- Corner analysis
- Sensitivity
- Pole-zero
- Harmonic balance
- PSS / PAC / PNoise / PXF / PSTB
- Envelope and multi-rate flows
- Signal-integrity oriented analyses such as eye-diagram related tooling

### Device and Modeling Support

The repository includes implementations for:

- MOSFET families including BSIM4, BSIM3, EKV, VDMOS, and classic level models
- BJT, diode, JFET, switches, coupled magnetics, and transmission-line elements
- Behavioral sources
- Verilog-A modules
- XSPICE-style mixed-signal elements and digital bridges

## Workspace

RSpice is organized as a five-crate Rust workspace:

| Crate | Purpose |
| :--- | :--- |
| `rspice-core` | Core simulation engine, device models, analyses, parsers, and regression harnesses |
| `rspice-cli` | Command-line workflow for simulation, validation, conversion, and reporting |
| `rspice-ui` | Desktop application for schematic editing and waveform inspection |
| `rspice-veriloga` | Verilog-A parser, semantic pipeline, runtime, and optional native codegen |
| `rspice-python` | Python bindings built with PyO3 |

Additional crate-specific details:
- [CLI README](crates/rspice-cli/README.md)
- [Python README](crates/rspice-python/README.md)

## Building

### Prerequisites

- Rust 1.85 or newer

### Release Build

```bash
git clone https://github.com/JaimeHW/RSpice.git
cd RSpice
cargo build --release --workspace
```

Optional host-tuned build:

```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release --workspace
```

## Quick Start

Create a netlist like `rc_circuit.sp`:

```spice
* Simple RC Circuit
V1 1 0 PULSE(0 5 0 1n 1n 1u 2u)
R1 1 2 1k
C1 2 0 1n

.TRAN 10n 5u
.MEAS TRAN risetime TRIG V(2) VAL=0.5 RISE=1 TARG V(2) VAL=4.5 RISE=1
.END
```

Run it with the CLI:

```bash
cargo run -p rspice-cli --release -- run rc_circuit.sp
```

Write the results directly to HDF5:

```bash
cargo run -p rspice-cli --release -- run rc_circuit.sp -o rc_circuit.h5 --format hdf5
```

## Running the Workspace

### Desktop UI

```bash
cargo run -p rspice-ui --release
```

### CLI

```bash
cargo run -p rspice-cli --release -- --help
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

## CLI Workflow

Primary CLI commands:

| Command | Description |
| :--- | :--- |
| `rspice run` | Execute simulations |
| `rspice info` | Print parsed netlist information |
| `rspice check` | Validate syntax and connectivity |
| `rspice compare` | Compare output against a golden result |
| `rspice convert` | Convert between RAW, ASCII RAW, CSV, JSON, TSV, and HDF5 |
| `rspice compile-va` | Compile Verilog-A models |

Examples:

```bash
# Quiet batch run with JUnit report
rspice run circuit.sp -q --report-format junit --report-file results.xml

# Convert CSV data into HDF5
rspice convert results.csv results.h5 --to hdf5

# Compare results to a golden file
rspice compare results.csv golden.csv --abstol 1e-9 --reltol 1e-6
```

CLI-specific details are documented in [crates/rspice-cli/README.md](crates/rspice-cli/README.md).

## Testing

RSpice is validated at three levels: unit tests inside each crate, integration
tests under `crates/rspice-core/tests/`, and an ngspice regression harness that
runs the official ngspice test suite (vendored under `tests/`, including the
checked-in reference `.out` oracles) against the RSpice engine.

```bash
# Unit and integration tests for the core engine
cargo test --release -p rspice-core

# The ngspice regression suite only
cargo test --release -p rspice-core --test ngspice_regression

# One suite (e.g. transient decks), with per-deck pass/fail and mismatch detail
cargo test --release -p rspice-core --test ngspice_regression -- test_ngspice_transient_suite
```

### ngspice regression harness

The harness (`crates/rspice-core/src/testing/ngspice_runner/`) discovers every
`.cir` deck under `tests/`, executes the analyses each deck requests (`.op`,
`.dc`, `.tran`, `.ac`, `.pz`, `.noise`, `.sens`, `.tf`), and compares results
row-by-row against the reference tables with a 2% relative tolerance plus
probe-aware absolute floors. Every executed analysis must be backed by a
validation oracle — checked-in reference data, `_t`/`_g` gold-node assertions,
or an explicit entry in `tests/validation-manifest.tsv` (execution-only
`smoke` decks and `.control`-scripted upstream decks) — so no deck can pass
silently. Because reference tables sample each binary's internally chosen
timesteps, two narrowly gated fallbacks keep the comparison measuring accuracy
rather than step-sequence reproduction: steep transient rows allow a
slope-gated time-jitter window of one local reference timestep (ngspice itself
reproduces its own tables only to within a step at fast edges), and rows where
the reference oscillates at sample scale (coarse-grid ringing overshoot that
the official binary does not reproduce at converged settings) are compared
against the local reference envelope. Operating points, smooth regions, and
settled levels always keep the strict pointwise tolerances.

Each deck runs in an isolated watchdog-supervised process
(`rspice-ngspice-case-runner`), so a hung simulation cannot stall the suite.
Useful environment variables:

| Variable | Effect |
| :--- | :--- |
| `RSPICE_NGSPICE_HARD_CASE_TIMEOUT_MS` | Raise the per-deck hard watchdog (default 30000) for long ring-oscillator decks |
| `RSPICE_NGSPICE_LIVE_REFERENCES=1` | Compare against a live local ngspice instead of checked-in oracles (requires `NGSPICE_SOURCE_ROOT` and `NGSPICE_EXE`) |
| `RSPICE_LTE_DEBUG=1` / `RSPICE_GRID_DEBUG=1` | Log binding LTE charge branches and accepted-step decisions for timestep-parity debugging |

## License

RSpice is source-available software licensed under the **RSpice Personal Use License**.
