# RSpice

**RSpice** is a high-performance, modern analog circuit simulator written in Rust. It is designed to be a faster, safer, and more extensible alternative to traditional SPICE engines, leveraging the safety and parallelism of Rust alongside state-of-the-art sparse linear algebra.

RSpice aims to bridge the gap between open-source simulation tools and professional-grade commercial simulators like LTspice or Cadence Spectre, offering advanced features like Verilog-A support, GPU-accelerated visualization, and a modern cross-platform UI.

## ✨ Key Features

- **🚀 High Performance**: Built on the [faer](https://github.com/sarah-ek/faer-rs) sparse linear algebra library and Rayon for parallel processing, RSpice delivers exceptionally fast simulation times for large netlists.
- **🔌 Verilog-A Support**: Integrated compiler and virtual machine for Verilog-A/AMS, allowing for the definition of custom compact devices and behavioral models standard in the semiconductor industry.
- **🖥️ Modern UI**: A responsive, GPU-accelerated user interface built with Dioxus and WebGPU. Runs natively on Windows, Linux, and macOS, or directly in the browser via WebAssembly.
- **📊 Rich Analysis Suite**:
  - **OP**: DC Operating Point analysis.
  - **DC**: DC Sweep analysis with nested sweeps.
  - **AC**: Small-signal AC analysis (Bode plots, Noise analysis).
  - **TRAN**: Transient analysis with adaptive time-stepping.
  - **.STEP**: Parametric sweeps for design optimization.
- **🛠️ Professional CLI**: Robust command-line interface with progress bars, formatted output, and extensive export options (`.raw`, `.csv`, `.json`).
- **📏 Measurement**: Full support for `.MEAS` directives to extract precise metrics from simulation results.

## 📦 Installation

### From Source

Ensure you have [Rust](https://rustup.rs/) (edition 2024 or newer) installed.

```bash
git clone https://github.com/rspice/rspice.git
cd rspice
cargo install --path crates/rspice-cli
```

To build the UI:

```bash
cargo run --bin rspice-ui --release
```

## 🚀 Getting Started

### CLI Usage

RSpice is compatible with standard SPICE netlists (`.sp`, `.cir`).

**Run a simple simulation:**
```bash
rspice examples/amplifier.sp
```

**Export results to a raw file (LTspice compatible):**
```bash
rspice circuit.net -o output.raw
```

**Run a parametric sweep and export to CSV:**
```bash
rspice sweep.sp --format csv -o results.csv
```

### GUI Usage

Launch the UI to access the schematic editor and waveform viewer:

```bash
cargo run --bin rspice-ui
```

- **Schematic Editor**: Intuitive drag-and-drop interface for circuit design.
- **Waveform Viewer**: High-performance, GPU-rendered viewer with instant pan/zoom and precise cursors.

## 🏗️ Project Structure

The project is organized as a Rust workspace:

- **`crates/rspice-core`**: The heart of the simulator. Contains the simulation engine, device models, and solvers.
- **`crates/rspice-cli`**: The command-line interface driver.
- **`crates/rspice-ui`**: The Dioxus-based graphical user interface.
- **`crates/rspice-veriloga`**: The Verilog-A-to-IR compiler and runtime.
- **`crates/rspice-wasm`**: WASM bindings for running the core engine in web environments.

## 🤝 Contributing

Contributions are welcome! We are actively looking for help with:
- Adding new MOSFET models (BSIM, PSP).
- Improving Verilog-A compatibility.
- Enhancing the schematic capture UI.

## 📄 License

This project is licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
