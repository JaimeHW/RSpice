<p align="center">
  <img src="crates/rspice-ui/assets/logo.svg" width="200" alt="RSpice Logo" />
</p>

# RSpice

**RSpice** is a next-generation analog circuit simulator engineered in Rust. It combines the safety and parallelism of modern systems programming with state-of-the-art sparse linear algebra to deliver a high-performance, commercial-grade simulation engine.

Designed to bridge the gap between open-source tools and industry-standard simulators like LTspice™ or Cadence Spectre™, RSpice features a fully compliant Verilog-A compiler, a modern GPU-accelerated interface, and a robust CLI for automation pipelines.

## ✨ Key Capabilities

### 🚀 High-Performance Engine
- **State-of-the-Art Solvers**: Built on [`faer`](https://github.com/sarah-ek/faer-rs) for optimized sparse LU decomposition.
- **Parallel Execution**: Leverages `rayon` for multi-threaded model evaluation and matrix stamping.
- **Precision**: Full 64-bit floating-point accuracy with adaptive time-stepping algorithms.

### 🔌 Verilog-A & Modeling
- **Integrated Compiler**: Custom compiler for Verilog-A LRM 2.4, transforming standard model definitions into high-performance IR.
- **Device Support**: Native support for industry-standard BSIM4, PSP, and custom user-defined models.
- **Mixed-Signal**: Foundation for Verilog-AMS mixed-signal simulation.

### 📊 Advanced Analysis
- **DC Analysis**: Operating Point (OP) and nested DC Sweeps.
- **AC Analysis**: Small-signal frequency response and Noise analysis.
- **Transient Analysis**: Time-domain simulation with Gear/Trapezoidal integration methods.
- **Parametric Sweeps**: Full `.STEP` support for optimizing circuit parameters.
- **Monte Carlo**: Statistical analysis for yield estimation.
- **Sensitivity**: Automated sensitivity analysis for design robustness.

### 🖥️ Modern Experience
- **Cross-Platform UI**: A reactive, GPU-accelerated schematic capture and waveform viewer built with Dioxus and WebGPU.
- **Web Assembly**: Runs natively in modern web browsers via WASM.
- **Professional CLI**: Rich terminal output, progress tracking, and structured export (Raw, CSV, JSON).

## 📦 Installation

**Prerequisites**: [Rust](https://rustup.rs/) (2024 edition or newer).

```bash
# Clone the repository
git clone https://github.com/rspice/rspice.git
cd rspice

# Install the CLI tool
cargo install --path crates/rspice-cli

# Run the UI directly
cargo run --bin rspice-ui --release
```

## ⚡ Quick Start

RSpice accepts standard SPICE netlists (`.sp`, `.cir`) and produces industry-compatible output.

**cli**
```bash
# Run a simple simulation
rspice examples/amplifier.sp

# Run a parametric sweep and export to binary raw format (LTspice compatible)
rspice design/filter_sweep.sp --format raw -o data.raw
```

**ui**
```bash
# Launch the graphical interface
cargo run --bin rspice-ui
```

## 🏗️ Architecture

The project is structured as a modular Rust workspace:

| Crate | Description |
|-------|-------------|
| `rspice-core` | The simulation kernel, solvers, and device models. |
| `rspice-veriloga` | Verilog-A to IR compiler and virtual machine. |
| `rspice-ui` | Dioxus-based GUI for schematic capture and visualization. |
| `rspice-cli` | Command-line driver for batch processing and automation. |
| `rspice-wasm` | WebAssembly bindings for browser integration. |

## 🤝 License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
