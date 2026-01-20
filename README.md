<div align="center">

# <img src="assets/logo.svg" alt="RSpice Logo" width="300" />

**The Circuit Simulator, Reimagined.**

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)](https://github.com/rspice/rspice/actions)

<br/>

<img src="assets/screenshot.png" alt="RSpice Studio" width="100%" style="border-radius: 8px; box-shadow: 0 10px 30px rgba(0,0,0,0.5);" />

<br/>

*Next-generation analog verification for the modern era.*

</div>

---

## Why RSpice?

Legacy circuit simulators are stuck in the past—burdened by decades of technical debt, clunky interfaces, and inefficient solvers.

**RSpice** is built for **now**. Engineered from the ground up in Rust, it merges the accuracy of commercial SPICE tools with the performance of modern parallel computing and the usability of next-gen design tools.

It isn't just a simulator; it's a complete verification platform designed for speed, scale, and the cloud-native future.

## Engineered for Performance

### ⚡ Uncompromising Speed
Powered by **`faer`** and **`rayon`**, the core engine leverages state-of-the-art sparse linear algebra and multi-threaded execution. Experience transient analysis and parametric sweeps that fully utilize your hardware limits.

### 🔌 Native Verilog-A
Forget external C compilers. RSpice features a **JIT-compiled Verilog-A engine** (via Cranelift). It compiles industry-standard compact models directly to native machine code in milliseconds, offering commercial-grade model support without the toolchain headaches.

### 🎨 GPU-Accelerated Studio
The RSpice Studio interfaces (Web & Desktop) are rendered entirely on the GPU. With **`wgpu`** and **`dioxus`**, you get fluid 60FPS schematic capture, real-time waveform navigation, and a responsive experience that feels alive.

## Getting Started

### Prerequisites
*   **Rust 1.85+** (The bleeding edge of performance requires the latest tools).

### Installation

```bash
git clone https://github.com/rspice/rspice.git
cd rspice

# Build with native optimizations for maximum throughput
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Running RSpice

### 🖥️ The Studio (GUI)
Launch the integrated design environment:
```bash
cargo run -p rspice-ui --release
```

### ⚙️ The Engine (CLI)
Execute headless simulations for batch verification:
```bash
cargo run -p rspice-cli --release -- design.cir
```

## Roadmap

We are building the foundation for the next decade of EDA.

*   [ ] **Advanced RF**: Harmonic Balance & Shooting Newton.
*   [ ] **Cloud Native**: Distributed simulation dispatch.
*   [ ] **Model Import**: Direct reading of PSpice/LTspice encrypted libraries.
*   [ ] **Layout**: Integrated PCB & IC layout views.

## License

RSpice is open source, dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

---

<div align="center">
  <sub>Built with 🧡 in Rust.</sub>
</div>
