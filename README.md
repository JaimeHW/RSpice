<div align="center">

<img src="assets/logo.svg" alt="RSpice Logo" width="180" />

# RSpice
**The Circuit Simulator, Reimagined.**

[![License](https://img.shields.io/badge/license-Source%20Available-red.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)](https://github.com/rspice/rspice/actions)
[![Platform](https://img.shields.io/badge/platform-win%20|%20macos%20|%20linux%20|%20web-lightgrey.svg?style=flat-square)](https://rspice.org)

<br/>

<img src="assets/image.png" alt="RSpice Studio" width="100%" style="border-radius: 8px; box-shadow: 0 12px 40px rgba(0,0,0,0.6);" />

<br/>

*Next-generation analog verification for the post-Moore era.*

</div>

---

## Overview

**RSpice** is a high-performance, general-purpose analog circuit simulator engineered for modern hardware. Built entirely in **Rust**, it replaces legacy SPICE limitations with a modern, parallelized architecture designed for massive multi-core scalability and cloud-native verification.

Bridging the gap between academic tools and commercial EDA, RSpice features a **JIT-compiled Verilog-A engine**, **RF-grade analysis**, and a **GPU-accelerated interface**, delivering the speed and accuracy required for complex mixed-signal, power, and RF design.

## Why RSpice?

### 🚀 **Engineered for Speed**
*   **Parallel Core**: Built on **`rayon`**, enabling massive parallelism for Monte Carlo analysis and parametric sweeps across all available CPU cores.
*   **Sparse Solver**: Utilizes **`faer`** for state-of-the-art sparse LU factorization, offering superior scaling on large matrices compared to legacy solvers (KLU/Sparse 1.3).
*   **JIT Compilation**: Verilog-A models are compiled directly to native machine code (AVX2/AVX-512) via **Cranelift**—zero interpretation overhead.

### 🎯 **Precision & Reliability**
*   **Adaptive Stepping**: Advanced Gear/Trapezoidal integration with dynamic local truncation error (LTE) control.
*   **Robust Convergence**: Homotopy methods (Gmin, Source Stepping) and pseudo-transient continuation for difficult operating points.
*   **Measurement Engine**: Built-in support for complex `.MEAS` statements including `RISE`, `FALL`, `DELAY`, `INTEG`, `RMS`, and `FIND...WHEN` conditional extractions.

### 🔌 **Extensible Architecture**
*   **FFI Plugin System**: Load external device models compiled as dynamic libraries (`.dll`/`.so`) for proprietary or legacy model integration.
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
| | **PNoise** | Phase noise and cyclostationary noise analysis |
| | **Noise** | Thermal, Shot, and Flicker (1/f) noise summary |
| | **S-Parameter** | N-port network scattering parameters |
| | **STB** | Loop gain and stability analysis |
| | **Transfer Function** | Transfer function computation |
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
*   **Passive**: Lossy Transmission Lines (T-element), **Jiles-Atherton** Magnetic Hysteresis, Coupled Inductors.
*   **Behavioral**: Arbitrary Sources (Equation-based), **Verilog-A** modules.
*   **Mixed-Signal**: Full **XSPICE** event-driven subsystem (ngspice-compatible) with **A/D & D/A bridges** and Digital Primitives.

---

## RSpice Studio

The visual interface is designed for the modern engineer:

*   **GPU Rendering**: Powered by **WebGPU** and **`wgpu`**, rendering massive waveforms and complex schematics at a fluid 60FPS.
*   **Cross-Probing**: Interactive linking between schematic nodes and waveform traces.
*   **Virtuoso-Style Hierarchy**: Professional Library/Cell/View management for complex design reuse.
*   **Format Support**: Import/Export standard formats including Touchstone (`.s2p`), CSV, and **LTspice® RAW**.

---

## Installation

### Prerequisites
*   **Rust 1.85+** (Required).

### Build from Source
RSpice is optimized for modern instruction sets. For maximum performance:

```bash
git clone https://github.com/rspice/rspice.git
cd rspice

# Build with native CPU optimizations (AVX/SSE)
RUSTFLAGS="-C target-cpu=native" cargo build --release
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
cargo run -p rspice-cli --release -- design.cir
```


## CLI Features

The RSpice CLI provides a subcommand-based interface designed for automation:

| Command | Description |
|---------|-------------|
| `rspice run` | Execute simulations with full analysis support |
| `rspice info` | Display netlist summary |
| `rspice check` | Validate syntax and connectivity |
| `rspice compare` | Golden file regression testing |
| `rspice convert` | Format conversion |
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

## Roadmap

We are consistently pushing the boundaries of open-source EDA:

*   [x] **Advanced RF**: Harmonic Balance, PSS, & PNoise.
*   [x] **Measurement Engine**: Automated extraction of circuit metrics.

## License

RSpice is **Source Available** software licensed under the **RSpice Personal Use License**.
You may view, download, and compile the code for personal, educational, or internal research purposes.
**Redistribution and Commercial Usage are strictly prohibited** without prior written permission.