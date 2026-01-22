<div align="center">

<img src="assets/logo.svg" alt="RSpice Logo" width="160" />

**The Circuit Simulator, Reimagined.**

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)](https://github.com/rspice/rspice/actions)

<br/>

<img src="assets/image.png" alt="RSpice Studio" width="100%" style="border-radius: 8px; box-shadow: 0 10px 30px rgba(0,0,0,0.5);" />

<br/>

*Next-generation analog verification for the modern era.*

</div>

---

## Overview

**RSpice** is a high-performance, general-purpose analog circuit simulator engineered for the post-Moore era. Built entirely in **Rust**, it replaces legacy SPICE limitations with a modern, parallelized architecture designed for massive multi-core scalability and cloud-native verification.

Bridging the gap between academic tools and commercial EDA, RSpice features a **JIT-compiled Verilog-A engine** and a **GPU-accelerated interface**, delivering the speed and accuracy required for complex mixed-signal design.

## Core Capabilities

### ⚡ Simulation Engine
*   **Solver Technology**: Utilizes **`faer`** for state-of-the-art sparse LU factorization, offering superior scaling on large matrices compared to legacy solvers.
*   **Parallel Architecture**: The core engine is built on **`rayon`**, enabling massive parallelism for Monte Carlo analysis and parametric sweeps across all available CPU cores.
*   **Adaptive Timestepping**: Implements advanced Gear/Trapezoidal integration methods with dynamic step control for optimal accuracy/speed trade-off.

### 🔬 Analysis Types
| Analysis | Description |
| :--- | :--- |
| **DC Operating Point** | Robust Newton-Raphson nonlinear solver |
| **DC Sweep** | Nested voltage/current/parameter sweeps |
| **AC Small-Signal** | Frequency domain response with complex arithmetic |
| **Transient** | Time-domain simulation with adaptive timestepping |
| **Harmonic Balance** | Frequency-domain steady-state solution for nonlinear RF circuits |
| **PSS** | Periodic Steady State analysis for RF & Mixed-Signal |
| **PNoise** | Periodic Noise (Cyclostationary) analysis |
| **Noise** | Thermal, Shot, and Flicker (1/f) noise analysis |
| **Fourier (THD)** | Harmonic distortion analysis and spectral decomposition |
| **Sensitivity** | DC/AC sensitivity to component variations |
| **Pole-Zero** | Transfer function pole/zero extraction |
| **S-Parameter** | Multi-port network analysis |
| **Monte Carlo** | Statistical yield verification utilizing parallel execution |

### 🔌 Device Model Support
RSpice includes a comprehensive library of industry-standard device models:

*   **MOSFET**: Level 1-3, **BSIM4**, **EKV**, **VDMOS** (Power MOSFETs).
*   **BJT**: Gummel-Poon (NPN/PNP) with quasi-saturation effects.
*   **JFET**: N-Channel and P-Channel junction field-effect transistors.
*   **Diode**: Level 1 (Shockley) and Level 3 (high-injection).
*   **Magnetics**: **Jiles-Atherton** (Magnetic Hysteresis), Coupled Inductors (K-matrix), Saturable Cores.
*   **Digital**: **Tristate Buffers**, Logic Gates, Voltage/Current Controlled Switches.
*   **Passive**: Lossy Transmission Lines (T-element), R/L/C.
*   **Behavioral**: Arbitrary B-Sources (expression parsing), Controlled Sources (E, F, G, H).

### 📜 Verilog-A Compiler
RSpice features a **fully compliant Verilog-A LRM 2.4** compiler defined by its "Native-First" approach:
*   **System-Native JIT**: Models are compiled directly to native machine code (AVX/SSE) via **Cranelift**—offering superior performance without external C compilers or linkers.
*   **Full LRM Support**: Includes analog operators (`ddt`, `idt`, `absdelay`), noise functions (`white_noise`, `flicker_noise`), and system tasks.
*   **Zero Overhead**: Compiled models execute with the same performance as built-in C++ primitives.

### 🖥️ RSpice Studio
The visual interface is designed for the modern engineer:
*   **GPU Rendering**: Powered by **WebGPU** and **`wgpu`**, the interface renders massive waveforms and complex schematics at a fluid 60FPS.
*   **Cross-Platform**: Runs natively on Windows, macOS, and Linux, or directly in the browser via WebAssembly (WASM).
*   **Smart Interaction**: Features **Cross-Probing** between schematic and waveform, and an **Adaptive Grid** system for precise signal analysis.
*   **Integrated Workflow**: Hierarchical schematic capture, real-time simulation control, and interactive waveform analysis in a single IDE.

## Installation

### Prerequisites
*   **Rust 1.85+** (Required for latest language features).

### Build from Source

```bash
git clone https://github.com/rspice/rspice.git
cd rspice

# Build with native optimizations for maximum throughput
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Usage

### Graphical Interface (Studio)
Launch the integrated design environment:
```bash
cargo run -p rspice-ui --release
```

### Command Line (Engine)
Execute headless simulations for batch verification flows:
```bash
cargo run -p rspice-cli --release -- design.cir
```

## Roadmap

We are consistently pushing the boundaries of open-source EDA:

*   [x] **Advanced RF**: Harmonic Balance (HB), Shooting Newton (PSS), & PNoise.
*   [ ] **Cloud Native**: Kubernetes-ready distributed simulation dispatch.
*   [ ] **Model Import**: Direct reading of PSpice/LTspice encrypted libraries.
*   [ ] **Layout**: Integrated PCB & IC layout views with parasitic extraction.

## License

RSpice is open source, dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

---

<div align="center">
  <sub>Built with 🧡 in Rust.</sub>
</div>
