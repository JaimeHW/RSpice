# RSpice Core

The SPICE circuit simulation engine: netlist parsing, device models, sparse
matrix assembly, Newton-Raphson solving, and the analysis algorithms. Every
other crate in the workspace — the CLI, the GUI, the Python bindings, the
WASM bindings, the benchmark rig — is a frontend over this one. It has no
I/O conventions of its own beyond reading netlist text and `.include` files;
output formatting, exit codes, and configuration live in the frontends.

## Architecture

A netlist flows through the engine in stages:

```
netlist text
    │  netlist/    nom-based lexer + parser → Netlist AST
    │              (elements, models, subcircuits, analysis cards,
    │               parameters, .MEAS statements)
    ▼
  Netlist
    │  engine/builder + circuit/   subcircuit flattening, parameter
    │              resolution, device construction
    ▼
 CircuitData   struct-of-arrays storage: one array per device kind
    │  solver/    one symbolic analysis freezes the sparsity pattern;
    │             a position map gives O(1) stamp lookups thereafter
    ▼
 StaticMatrix  sparse CSC matrix + reusable LU workspace
    │  engine/    Newton-Raphson loop with convergence aids,
    │             transient integration, frequency sweeps
    ▼
 SimulationResult / analysis-specific result types
```

The crate-level entry point is `Engine` (re-exported from `engine/`), with
`Netlist::parse` as the front door:

```rust
use rspice_core::{Engine, Netlist};

let netlist = Netlist::parse("V1 1 0 10\nR1 1 0 1k\n.end")?;
let engine = Engine::default();
let result = engine.run_dc_op(&netlist)?;
```

`SimulationConfig`, `ConvergenceConfig`, `ConvergencePreset`, and
`DampingStrategy` configure the engine; `AbortSignal`/`AtomicAbort` allow a
frontend to cancel a long transient or sweep cooperatively (this is what
backs Ctrl-C in the CLI and `KeyboardInterrupt` in the Python bindings).

## Module map

| Module | Contents |
| :--- | :--- |
| `netlist/` | Lexer, parser (with submodules for element parsing, command parsing, conditionals, scoping, source specs, transmission lines, Laplace synthesis), AST, subcircuit flattener, hierarchical path handling, `.include` resolution, parameter scoping and expressions, multi-run cards, SPEF and XSPICE card parsing |
| `circuit/` | `CircuitData` struct-of-arrays storage (one typed array per device kind in `storage/`), construction, linear stamping, nonlinear device hooks, magnetic coupling, introspection, external-model attachment |
| `device/` | Device model implementations — see [Device models](#device-models) |
| `solver/` | Sparse LU (`sparse.rs` on faer, plus a KLU-class refactorization backend in `klu.rs`), Newton-Raphson (`newton.rs`, `enhanced_newton.rs`), convergence checking, damping strategies, arc-length continuation, parallel helpers (behind `parallel`) |
| `engine/` | The orchestrator: DC, AC, transient, harmonic balance (`hb/`), PSS (`pss.rs`, `pss_noise.rs`), stability (`stb.rs`), transfer functions, matrix assembly and stamping, source value evaluation, behavioral-expression hooks, circuit builder, configuration and config resolution, convergence-aid drivers |
| `analysis/` | Analysis algorithms and result types — see [Analyses](#analyses) — plus `.MEAS` evaluation (`measurements.rs`, `advanced/measure.rs`), post-processing, signal-integrity helpers, and result export (`output/`: rawfile export, waveforms, streaming waveforms) |
| `expr/` | Expression engine for behavioral sources and parameters: parser, AST, bytecode compiler, and a small VM so expressions evaluate cheaply inside the Newton loop |
| `library/` | `.lib` / model-library parsing and a library manager |
| `xspice/` | XSPICE code-model subsystem: `CodeModel` trait, `CodeModelRegistry`, instance/context types, event and digital value types, and bundled analog/digital/bridge models |
| `compat/` | Compatibility readers (`ltspice_raw.rs`: LTspice RAW files) |
| `constants` | Physical and simulation constants |
| `abort_signal` | `AbortSignal` trait with `AtomicAbort`/`NoAbort` implementations for cancelling long runs |
| `simd/` | SIMD math, reduction, and integration kernels (only with the `simd` feature) |
| `testing/` | The ngspice conformance harness — see [Testing](#testing) |
| `time_compat` | Wall-clock shim: real `std::time::Instant` natively, a no-op stub on `wasm32` (bare WASM has no clock) |

## Device models

Verified against `src/device/`:

**Passives** (`passive/`) — resistor, capacitor, inductor, coupled
inductors (`K`), saturable inductor, and a Jiles-Atherton magnetic
hysteresis model.

**Semiconductors** (`semiconductor/`) — junction diode; BJT (legacy
Gummel-Poon by default / `LEVEL=1`, VBIC at `LEVEL=4`, and explicit
rejection for unsupported advanced BJT levels).

**MOSFETs and FET-family models** (`mosfet/`) —

- Classic Berkeley MOS1/MOS2/MOS3/MOS6 (`mosfet.rs`, `mos_models.rs`)
- Legacy BSIM1/BSIM2 levels 4/5 (`legacy_bsim.rs`)
- BSIM3v3 (`bsim3.rs`, `bsim3v3/` — params/temp/eval split)
- BSIM4 v4.8 (`bsim4.rs`)
- EKV (`ekv.rs`)
- VDMOS power MOSFET (`vdmos/` — device, recovery, thermal submodules)
- B3SOI silicon-on-insulator, in DD/FD/PD variants (`b3soi/dd`, `b3soi/fd`, `b3soi/pd`)
- JFET level 1 and native Parker-Skellern JFET2 (`jfet/`, `jfet.rs`;
  `NJF`/`PJF LEVEL=2`) with ngspice-compatible `P`, `Q`, `XI`, `Z`,
  `VST`, `MVST`, `MXI`, `LFGAM`, `LFG1`, `LFG2`, `HFGAM`, `HFG1`,
  `HFG2`, `HFETA`, `HFE1`, `HFE2`, `TAUG`, `TAUD`, `DELTA`, `ACGAM`,
  `XC`, `CDS`, `IBD`, `VBD`, and `VER` model parameters plus the common
  JFET aliases such as `VT0`/`VTO` and `VBI`/`PB`. `SimulationConfig`
  defaults to best-available Parker-Skellern behavior, while
  `SpiceDialect::Xyce` selects the internal Xyce modified-Shockley JFET2
  compatibility path for Xyce regression coverage.

`bsim4v8/` is the native BSIM4 v4.8 path for MOS `LEVEL=14/54`, ported from
ngspice-46's `src/spicelib/devices/bsim4/`; those upstream BSIM4 files carry
UC Berkeley BSIM4 / ECL-2.0 terms tracked in the root `NOTICE`. It is wired through the builder,
matrix reservation, nonlinear Newton stamping, AC small-signal stamping, and
transient charge integration; `tests/bsim4_native.rs` pins the engine wiring
against OP, DC sweep, transient, and `LEVEL=54` decks. The port supports
internal and external bias-dependent S/D resistance (`rdsMod=0/1`) and still
rejects unported mode selectors with typed errors rather than silently changing
physics: distributed body and gate-resistance networks (`rbodyMod`, `rgateMod`), NQS,
unknown `cvchargeMod` charge paths, `mtrlMod=1`, and
other unported mode selectors. `GEOMOD=0..10` implicit diffusion geometry,
`RGEOMOD=1..8` implicit S/D resistance geometry, `WPEMOD=1` well-proximity
adjustment, `igcMod`/`igbMod` gate tunneling currents, BSIM4 stress layout
correction, and `dioMod=0/1/2` junction diode selectors are implemented. See
`src/device/mosfet/bsim4v8/params.rs` and `device.rs` for the exact
ported/not-ported inventory.

**Sources and behavioral** — independent sources (`sources.rs`), the four
controlled sources E/F/G/H (`controlled.rs`), behavioral B-sources whose
expressions compile through `expr/` (`behavioral.rs`), and PWL-from-file
sources (`pwl_file.rs`).

**Transmission lines** — lossless and lossy lines (`transmission_line.rs`
plus `transmission_line/` with delay, distributed, lossy, and TXL
submodules; the LTRA path is checked by `tests/ltra_ac_oracle.rs`), and
coupled multi-conductor lines (`coupled_transmission_line.rs`,
`cpl_native.rs`).

**Other** — switches (`switch.rs`), ideal op-amp (`opamp.rs`), GaN HEMT
(`gan_hemt.rs`, in-tree physics-style model; ASM-HEMT/MVSG CMC
qualification is feature-gated work through generated Rust from Verilog-A), thermal network elements (`thermal.rs`), tristate
(`tristate.rs`), device bypass for latent devices (`model_bypass.rs`).

**Extension points** — external Verilog-A devices via the `rspice-veriloga` compiler
(`veriloga.rs`, behind the `veriloga` feature, with blake3-keyed on-disk
caching of compiled models), build-time generated Verilog-A built-ins
(`veriloga-builtins`, materialized under `src/device/veriloga_generated/` and
instantiated by model name when the feature is enabled), dynamically loaded FFI
models (`ffi.rs`, behind `ffi`), and SIMD batch evaluation for diodes, BJTs,
JFETs, and MOSFET batches (`batch/`, behind `simd`).

## Analyses

Core analyses (`analysis/core/`, driven from `engine/`):

| Analysis | Module |
| :--- | :--- |
| DC operating point and DC sweep | `core/dc.rs`, `engine/dc.rs` |
| AC small-signal sweep | `core/ac.rs`, `engine/ac.rs` |
| Transient | `core/transient.rs`, `engine/transient/` |
| Temperature handling | `core/temperature.rs` |
| Laplace-defined sources/filters | `core/laplace.rs` |

Transient integration methods: backward Euler, trapezoidal, Gear-2, and the
hybrid trap/Gear default (selected via `SimulationConfig`; the CLI exposes
them as `--integration-method euler|trap|gear|trapgear`). Timestep control
is LTE-based with breakpoint handling; a transient checkpoint/resume path
exists (`engine/transient/`, exercised by `tests/transient_checkpoint.rs`
and the CLI's `--checkpoint`/`--resume`).

Advanced analyses (`analysis/advanced/`):

| Analysis | Module |
| :--- | :--- |
| Fourier / THD (`.FOUR`) | `fourier.rs` |
| Noise | `noise.rs`, `engine/advanced/noise.rs` |
| Pole-zero | `pole_zero.rs`, `pole_zero/` |
| Sensitivity (DC and AC) | `sensitivity.rs` |
| Transfer function (`.TF`) | `transfer.rs`, `transfer/` |
| Parametric sweep (`.STEP`) | `parametric.rs`, `engine/advanced/step.rs` |
| Monte Carlo | `monte_carlo.rs` |
| Process corners | `corner.rs` |
| Periodic steady state (shooting) | `pss/`, `engine/pss.rs` |
| Harmonic balance | `harmonic_balance/`, `engine/hb/` |
| Periodic noise (pnoise) | `pnoise/`, `engine/pss_noise.rs` |
| Periodic AC (PAC) | `pac/` |
| Periodic transfer function (PXF) | `pxf.rs` |
| Stability (STB) loop-gain | `stb.rs`, `engine/stb.rs` |
| Periodic stability (PSTB) | `pstb.rs` |
| S-parameters | `s_param.rs`, `s_param/` |
| `.MEAS` evaluation | `measure.rs`, `measure_signals.rs`, `measurements.rs` |

How each analysis is reached (netlist card, CLI flag, or engine API only)
varies — the [CLI README](../rspice-cli/README.md) documents the
netlist-card and flag surface; anything not listed there is engine-API
only.

## Solvers and convergence

- **Sparse LU**: the real-valued path defaults to the KLU-class backend
  (`solver/klu.rs` — stored pivots make refactorization on the frozen
  sparsity pattern cheap); `RSPICE_SOLVER=faer` opts back into the faer
  solver. faer also provides the complex solves for AC-family analyses,
  with parallel factorization when the `faer-parallel` feature is on.
- **Newton-Raphson** with voltage/residual/charge tolerance checks
  (`solver/newton.rs`, `solver/convergence.rs`).
- **Convergence aids**, attempted when plain Newton fails: GMIN stepping,
  source stepping, pseudo-transient continuation
  (`engine/convergence/`), and arc-length continuation
  (`solver/arc_length.rs`). `ConvergencePreset` bundles them as
  `fast`/`default`/`robust`.
- **Damping strategies** (`solver/damping.rs`): voltage limiting, line
  search, and combinations, selectable via `DampingStrategy`.

## Feature flags

| Feature | Default | Effect |
| :--- | :--- | :--- |
| `faer-parallel` | yes | Adds faer's rayon feature for parallel sparse factorization |
| `parallel` | yes | rayon + portable-atomic for parallel solver paths (`solver/parallel.rs`) |
| `simd` | yes | `wide`-based SIMD kernels (`simd/` module, `device/batch/`) |
| `veriloga` | no | Verilog-A device support via `rspice-veriloga`, plus serde/bincode/blake3/dirs for the compiled-model cache |
| `veriloga-native` | no | Native-only Cranelift JIT performance backend for supported Verilog-A fragments (implies `veriloga`); unsupported fragments fall back to the interpreter, and any expansion of the raw-pointer boundary requires a targeted safety review |
| `veriloga-builtins` | no | Build-time generated Rust devices from bundled Verilog-A sources; uses `build.rs` to materialize `src/device/veriloga_generated/` and compile a generated registry. Feature-gated until CMC model qualification and product gates are explicit |
| `wasm` | no | wasm-bindgen + `getrandom/js` so the crate builds on `wasm32-unknown-unknown`; used by `rspice-wasm` and the UI's wasm target, which also set `default-features = false` to drop rayon/SIMD |
| `ffi` | no | Experimental `libloading` integration for dynamically loaded external device models; not a production-stable device ABI until the callback ownership, library lifetime, and stamping contracts are audited |

The defaults mean the CLI, Python bindings, and the standard test run all
exercise the parallel + SIMD paths.

## Building and testing

```bash
# Build (library only)
cargo build -p rspice-core

# Full test suite — 75 integration test files under tests/
cargo test -p rspice-core

# With Verilog-A device tests (veriloga_*.rs oracle tests need the JIT)
cargo test -p rspice-core --features veriloga-native

# Generated Verilog-A built-in runtime checks (feature-gated)
cargo test -p rspice-core --features veriloga-builtins --test generated_veriloga_runtime

# Solver kernel micro-benchmark (factor/refactor/solve in isolation)
cargo run --release -p rspice-core --example klu_bench
```

Library unit tests are excluded from the default package test target by
`[lib] test = false`; run them explicitly with `cargo test -p rspice-core
--lib`. The integration suite in `tests/` includes oracle tests that pin
device and analysis behavior to reference values (diode rectifier, VBIC
excess phase, LTRA AC, native BSIM4, PSP103 via Verilog-A), RF-analysis tests
(HB Jacobian/Krylov/varactor, PSS shooting, pnoise folding, PAC conversion,
STB loop gain), parser robustness tests, and a determinism test.

### ngspice conformance harness

`src/testing/ngspice_runner/` is a framework for running netlist decks
through both ngspice (as reference) and this engine, comparing per-analysis
results under explicit tolerances: suite discovery, execution, reference
datasets, and validation live there, and `tests/ngspice_regression.rs` /
`tests/ngspice_oracle_audit.rs` drive it. The crate also ships one binary,
`rspice-ngspice-case-runner` (`src/bin/ngspice_case_runner.rs`), for
orchestrating those case runs. For whole-process performance comparison
against ngspice, see [rspice-bench](../rspice-bench/README.md) instead.

## License

RSpice Core is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
