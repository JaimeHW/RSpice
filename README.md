<div align="center">

<img src="assets/logo.svg" alt="RSpice Logo" width="180" />

# RSpice
**The Circuit Simulator, Reimagined.**

[![License](https://img.shields.io/badge/license-Source%20Available-red.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)](https://github.com/JaimeHW/rspice/actions)
[![Platform](https://img.shields.io/badge/platform-win%20|%20macos%20|%20linux%20|%20web-lightgrey.svg?style=flat-square)](https://rspice.org)

<img src="assets/image.png" alt="RSpice Studio" width="100%" style="border-radius: 8px; box-shadow: 0 12px 40px rgba(0,0,0,0.6);" />


</div>




## Table of Contents
- [Overview](#overview)
- [Why RSpice?](#why-rspice)
- [Capabilities](#capabilities)
- [Architecture](#architecture)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [CLI Features](#cli-features)
- [License](#license)

---


## Overview

**RSpice** is a high-performance, general-purpose analog and mixed-signal electronic circuit simulator engineered for modern hardware. Built entirely in **Rust**, it replaces legacy SPICE limitations with a modern, parallelized architecture designed for massive multi-core scalability and cloud-native verification.

Bridging the gap between academic tools and commercial EDA, RSpice features a **JIT-compiled Verilog-A engine**, **RF-grade analysis**, and a **GPU-accelerated interface**, delivering the speed and accuracy required for complex mixed-signal, power, and RF design.

## Why RSpice?

### 🚀 **Engineered for Speed**
*   **Parallel Core**: Built on **`rayon`**, enabling massive parallelism for Monte Carlo analysis and parametric sweeps across all available CPU cores.
*   **Sparse Solver**: Utilizes **`faer`** for state-of-the-art sparse LU factorization, offering superior scaling on large matrices compared to legacy solvers (KLU/Sparse 1.3).
*   **JIT Compilation**: Verilog-A models are compiled directly to native machine code via **Cranelift**—zero interpretation overhead.
*   **SIMD Acceleration**: Optional vectorized math, integration, and reduction kernels for numerically intensive workloads.

### 🎯 **Precision & Reliability**
*   **Adaptive Stepping**: Advanced Gear/Trapezoidal integration with dynamic local truncation error (LTE) control.
*   **Robust Convergence**: Tiered solver strategy featuring **Enhanced Newton-Raphson**, **Arc-Length Continuation**, and Homotopy methods (Gmin, Source Stepping) for absolute convergence stability.
*   **Measurement Engine**: Built-in support for complex `.MEAS` statements including `RISE`, `FALL`, `DELAY`, `INTEG`, `RMS`, and `FIND...WHEN` conditional extractions.

### 🔌 **Extensible Architecture**
*   **FFI Plugin System**: Load external device models compiled as dynamic libraries (`.dll`/`.so`) for proprietary or legacy model integration.
*   **Python Bindings**: Native **PyO3** bindings with **NumPy** zero-copy array access for scripting, post-processing, and automation.
*   **Open Standard**: Fully compliant with standard SPICE netlist syntax and **Verilog-A LRM 2.4**.

---

## Capabilities

### 🔬 Analysis Types
| Domain | Analysis | Description |
| :--- | :--- | :--- |
| **Time** | **Transient** | Time-domain simulation with adaptive timestepping |
| | **Shooting PSS** | Periodic Steady State for switching converters & oscillators |
| | **Fourier** | THD and spectral decomposition |
| | **Envelope** | Multi-rate envelope transient for modulated signals |
| **Freq** | **AC Small-Signal** | Frequency domain response |
| | **Harmonic Balance** | Nonlinear steady-state solution for RF circuits |
| | **PAC** | Periodic AC small-signal analysis |
| | **PNoise** | Phase noise and cyclostationary noise analysis |
| | **PXF** | Periodic Transfer Function analysis |
| | **Noise** | Thermal, Shot, and Flicker (1/f) noise summary |
| | **S-Parameter** | N-port network scattering parameters |
| | **STB** | Loop gain and stability analysis |
| | **XF (Transfer)** | Small-signal transfer function computation |
| **Param** | **DC Sweep** | Nested voltage/current/parameter sweeps |
| | **Temperature** | Temperature sweep analysis |
| | **Monte Carlo** | Statistical yield verification with histogram & 3-sigma |
| | **Corner Analysis** | PVT (Process-Voltage-Temperature) sweep with TT/SS/FF/SF/FS |
| | **Sensitivity** | DC/AC sensitivity to component variations |
| | **Pole-Zero** | Transfer function pole/zero extraction |


### ⚡ Device Models
RSpice includes a comprehensive library of industry-standard device models:

*   **MOSFET**: **BSIM4**, **BSIM3v3.24** (submicron), **EKV**, **VDMOS** (Power), Level 1-3.
*   **BJT**: Gummel-Poon (NPN/PNP) with quasi-saturation, substrate effects, and high-injection modeling.
*   **Diode**: Shockley equation with junction and diffusion capacitance.
*   **JFET/GaN HEMT**: Curtice/Cubic models with self-heating and trapping effects.
*   **Passive**: Lossy Transmission Lines (T-element), **Jiles-Atherton** Magnetic Hysteresis, Coupled Inductors, Saturable Inductors.
*   **Behavioral**: Arbitrary Sources (Equation-based), **Verilog-A** modules, Op-Amp macro models.
*   **Mixed-Signal**: Full **XSPICE** event-driven subsystem (ngspice-compatible) with **A/D & D/A bridges** and Digital Primitives.

---

## Architecture

RSpice is organized as a Rust workspace with five active crates:

| Crate | Description |
| :--- | :--- |
| **`rspice-core`** | Simulation engine — MNA stamping, solvers, device models, analysis suite |
| **`rspice-cli`** | Command-line interface with subcommand-based automation |
| **`rspice-ui`** | GPU-accelerated graphical interface (egui + wgpu) |
| **`rspice-veriloga`** | Verilog-A/AMS compiler — lexer, parser, semantic analysis, Cranelift JIT |
| **`rspice-python`** | Python bindings via PyO3 with NumPy zero-copy array support |

---

## Installation

### Prerequisites
*   **Rust 1.85+** (Required).

### Build from Source
RSpice is optimized for modern instruction sets. For maximum performance:

```bash
git clone https://github.com/JaimeHW/rspice.git
cd rspice

# Build with native CPU optimizations (AVX/SSE)
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Quick Start

Create a simple netlist `rc_circuit.sp`:

```spice
* Simple RC Circuit
V1 1 0 PULSE(0 5 0 1n 1n 1u 2u)
R1 1 2 1k
C1 2 0 1n

.TRAN 10n 5u
.MEAS TRAN risetime TRIG V(2) VAL=0.5 RISE=1 TARG V(2) VAL=4.5 RISE=1
```

Run the simulation:

```bash
cargo run -p rspice-cli --release -- run rc_circuit.sp
```


## Usage

### Graphical Interface
Launch the integrated design environment:
```bash
cargo run -p rspice-ui --release
```

### Headless Engine
Execute batch simulations for CI/CD pipelines:
```bash
cargo run -p rspice-cli --release -- run design.cir
```

### Python Scripting
Use the PyO3-based Python bindings for automation and post-processing:
```bash
# Build the Python module
cd crates/rspice-python
pip install maturin
maturin develop --release
```

```python
import rspice

# Run a simulation
results = rspice.run("circuit.sp")
print(results.variables)
```


## CLI Features

The RSpice CLI provides a subcommand-based interface designed for automation:

| Command | Description |
|---------|-------------|
| `rspice run` | Execute simulations with full analysis support |
| `rspice info` | Display netlist summary |
| `rspice check` | Validate syntax and connectivity |
| `rspice compare` | Golden file regression testing |
| `rspice convert` | Format conversion (RAW, CSV, JSON, TSV, ASCII) |
| `rspice compile-va` | Compile Verilog-A models |

### CI/CD Integration

```bash
# JUnit report for CI pipelines
rspice run circuit.sp -q --report-format junit --report-file results.xml

# Regression testing
rspice compare results.csv golden.csv --abstol 1e-9

# Monte Carlo yield analysis
rspice run circuit.sp --monte-carlo 1000 --seed 42
```
📖 **For complete CLI documentation, see [CLI Reference](crates/rspice-cli/README.md).**

---


## License

RSpice is **Source Available** software licensed under the **RSpice Personal Use License**.
You may view, download, and compile the code for personal, educational, or internal research purposes.
**Redistribution and Commercial Usage are strictly prohibited** without prior written permission.
