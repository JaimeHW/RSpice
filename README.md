# RSpice

A high-performance SPICE circuit simulator written in Rust.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Overview

RSpice is a modern circuit simulation engine that provides accurate analog circuit analysis with a focus on performance and extensibility. It supports standard SPICE netlist syntax with extensions for advanced analysis types.

## Features

### Analysis Types
- **DC Analysis** - Operating point and DC sweep
- **AC Analysis** - Small-signal frequency response
- **Transient Analysis** - Time-domain simulation with adaptive timestep
- **Noise Analysis** - Thermal, shot, and flicker noise
- **Fourier Analysis** - THD and harmonic distortion
- **Monte Carlo** - Statistical variation analysis
- **Sensitivity Analysis** - Component sensitivity
- **S-Parameter** - RF network analysis

### Device Models
- **Passive** - Resistors, capacitors, inductors (with saturation)
- **Semiconductors** - Diodes, BJTs (Gummel-Poon), MOSFETs (Level 1-3, BSIM3/4)
- **Sources** - DC, AC, PULSE, SIN, PWL, EXP waveforms
- **Controlled Sources** - VCVS, CCCS, VCCS, CCVS
- **Advanced** - Transmission lines, coupled inductors, behavioral sources

### Technical Highlights
- **C1-Continuous Models** - Smooth region transitions for robust Newton-Raphson convergence
- **TrapGear Integration** - Automatic switching between Trapezoidal and Gear-2 methods
- **Adaptive Timestepping** - LTE-based error control with breakpoint handling
- **Sparse Matrix Solver** - Efficient direct LU factorization
- **WebAssembly Support** - Optional WASM compilation for browser-based simulation

## Quick Start

```bash
# Clone and build
git clone https://github.com/your-org/rspice.git
cd rspice
cargo build --release

# Run a simulation
cargo run --release -- simulate circuit.cir -o results.raw
```

### Example Netlist

```spice
RC Lowpass Filter
V1 in 0 AC 1 SIN(0 1 1k)
R1 in out 1k
C1 out 0 1u
.AC DEC 10 1 100k
.TRAN 1u 10m
.END
```

## Installation

### From Source
```bash
cargo install --path crates/rspice-cli
```

### As a Library
```toml
[dependencies]
rspice-core = { path = "crates/rspice-core" }
```

## Architecture

```
rspice/
├── crates/
│   ├── rspice-core/           # Simulation engine
│   │   ├── analysis/
│   │   │   ├── core/          # DC, AC, transient, laplace, temperature
│   │   │   ├── advanced/      # Noise, fourier, monte_carlo, sensitivity, s_param, pole_zero
│   │   │   └── output/        # Waveform recording, raw export
│   │   ├── circuit/           # Circuit construction and stamping
│   │   ├── device/
│   │   │   ├── passive/       # Resistor, capacitor, inductor, coupled inductors
│   │   │   ├── semiconductor/ # Diode, BJT
│   │   │   └── mosfet/        # MOSFET Level 1-3, BSIM3/4, EKV, JFET, VDMOS
│   │   ├── engine/            # Simulation orchestration
│   │   ├── netlist/           # SPICE parser
│   │   └── solver/            # Sparse matrix solver
│   └── rspice-cli/            # Command-line interface
```

### Key Modules

| Module | Description |
|--------|-------------|
| `netlist` | SPICE-compatible parser with subcircuit support |
| `device` | Semiconductor and passive device models |
| `solver` | Sparse matrix and Newton-Raphson iteration |
| `analysis` | Simulation engines (DC, AC, transient, noise) |
| `engine` | High-level simulation orchestration |

## API Example

```rust
use rspice_core::{Netlist, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let netlist = Netlist::parse(r#"
        Voltage Divider
        V1 in 0 DC 10
        R1 in out 1k
        R2 out 0 1k
        .OP
        .END
    "#)?;

    let engine = Engine::new(netlist)?;
    let result = engine.run_dc_op()?;

    println!("V(out) = {:.3}V", result.voltage("out")?);
    Ok(())
}
```

## Supported SPICE Commands

| Command | Description |
|---------|-------------|
| `.OP` | DC operating point |
| `.DC` | DC sweep analysis |
| `.AC` | AC frequency sweep |
| `.TRAN` | Transient time-domain |
| `.NOISE` | Noise analysis |
| `.FOUR` | Fourier/THD analysis |
| `.PARAM` | Parameter definition |
| `.MODEL` | Device model definition |
| `.SUBCKT` | Subcircuit definition |
| `.INCLUDE` | File inclusion |
| `.STEP` | Parametric sweep |
| `.MEAS` | Measurement statements |

## Performance

RSpice uses several optimizations for high performance:

- **Sparse Matrix Storage** - CSC format for memory efficiency
- **Direct Indexing** - Pre-computed matrix indices for O(1) stamping
- **Parallel Analysis** - Optional multi-threaded sweeps (via `rayon`)
- **Streaming Output** - Memory-efficient waveform storage

## Building with Features

```bash
# Enable parallel processing
cargo build --release --features parallel

# Build for WebAssembly
cargo build --target wasm32-unknown-unknown --features wasm
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure `cargo test --all` passes
5. Submit a pull request

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

RSpice draws inspiration from:
- SPICE3 (UC Berkeley)
- Ngspice
- LTspice

---

*Built with ❤️ in Rust*
