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

// The first line of a SPICE deck is the title, never an element.
let netlist = Netlist::parse("divider\nV1 1 0 10\nR1 1 0 1k\n.end")?;
let engine = Engine::default();
let result = engine.run_dc_op(&netlist)?;
assert_eq!(result.voltage(1), 10.0);
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
| `solver/` | Sparse LU (`sparse.rs` on faer, plus a KLU-class refactorization backend in `klu.rs`), Newton-Raphson (`newton.rs`), convergence checking, damping strategies, arc-length continuation, parallel helpers (behind `parallel`) |
| `engine/` | The orchestrator: DC, AC, transient, harmonic balance (`hb/`), PSS (`pss.rs`, `pss_noise.rs`), stability (`stb.rs`), transfer functions, matrix assembly and stamping, source value evaluation, behavioral-expression hooks, circuit builder, configuration and config resolution, convergence-aid drivers |
| `analysis/` | Analysis algorithms and result types — see [Analyses](#analyses) — plus `.MEAS` evaluation (`measurements.rs`, `advanced/measure.rs`), post-processing, signal-integrity helpers, and result export (`output/`: rawfile export, waveforms, streaming waveforms) |
| `expr/` | Expression engine for behavioral sources and parameters: parser, AST, bytecode compiler, and a small VM so expressions evaluate cheaply inside the Newton loop |
| `library/` | `.lib` / model-library parsing, a library manager, and filesystem discovery of shipped Verilog-A model packs |
| `xspice/` | XSPICE code-model subsystem: `CodeModel` trait, `CodeModelRegistry`, instance/context types, event and digital value types, bundled analog/digital/bridge models, plus an `ifspec.ifs` parser and conformance helpers that diff the registry against an ngspice checkout |
| `compat/` | Compatibility readers (`ltspice_raw.rs`: LTspice RAW files) |
| `constants` | Physical and simulation constants |
| `abort_signal` | `AbortSignal` trait with `AtomicAbort`/`NoAbort` implementations for cancelling long runs |
| `simd/` | SIMD math, reduction, and integration kernels (only with the `simd` feature) |
| `testing/` | The ngspice and Xyce conformance harnesses — see [Testing](#testing) |
| `time_compat` | Wall-clock shim: real `std::time::Instant` natively, a no-op stub on `wasm32` (bare WASM has no clock) |

## Device models

Verified against `src/device/`:

**Passives** (`passive/`) — resistor, capacitor, inductor, coupled
inductors (`K`), saturable inductor, and a Jiles-Atherton magnetic
hysteresis model.

**Semiconductors** (`semiconductor/`) — junction diode; BJT (legacy
Gummel-Poon with no `LEVEL` or `LEVEL=1/2`, native VBIC at
`LEVEL=4/9/11/12/13`). Other BJT levels are rejected with a typed error
naming the supported set; advanced CMC bipolar models are reached through
generated Rust from Verilog-A rather than a hand-written path.

**MOSFETs and FET-family models** (`mosfet/`) —

- Classic Berkeley MOS1/MOS2/MOS3/MOS6 at `LEVEL=1/2/3/6` (`mosfet.rs`,
  `mos_models.rs`)
- Legacy BSIM1/BSIM2 at `LEVEL=4/5` (`legacy_bsim.rs`)
- MOS9 at `LEVEL=9`, which is also where Xyce-style BSIM3 cards land: a
  decisive BSIM3 parameter signature routes to BSIM3v3, and an
  ngspice-shaped card stays MOS9
- BSIM3v3 at `LEVEL=8/49/53` (`bsim3.rs`, `bsim3v3/` — params/temp/eval split)
- BSIM4 v4.8 at `LEVEL=14/54` (`bsim4.rs`, `bsim4v8/`)
- EKV 2.6 at `LEVEL=260` (`ekv.rs`), plus a narrow native EKV3 `LEVEL=301`
  slice (`ekv3.rs`) covering the VA-Models/Xyce 150 nm NMOS/PMOS cards;
  other EKV3 cards fail closed in the builder. The complete EKV3 302.00 model
  is the generated `ekv3_rf` device (`veriloga-model-ekv3-rf`), reached by
  module name on an `X` line rather than by a `LEVEL` selector
- VDMOS power MOSFET at `LEVEL=18` (`vdmos/` — device, recovery, thermal
  submodules)
- B3SOI silicon-on-insulator at `LEVEL=10/55/56/57`, in DD/FD/PD variants
  (`b3soi/dd`, `b3soi/fd`, `b3soi/pd`)
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
against OP, DC sweep, transient, and `LEVEL=54` decks. Implemented: internal
and external bias-dependent S/D resistance (`rdsMod=0/1`), distributed body
and gate-resistance networks (`rbodyMod=0/1/2`, `rgateMod=0/1/2/3`),
transient and AC charge-deficit NQS, `mtrlMod=1` material constants for both
compatibility modes, `capMod=0/1/2` with integer `cvchargeMod=0/1/2/3`,
`mobMod=0..6` (including the high-k/Synopsys variants), `tempMod=0/1/2/3`,
`geoMod=0..10` implicit diffusion geometry, `rgeoMod=1..8` implicit S/D
resistance geometry for omitted `NRD`/`NRS`, `wpemod=1` well-proximity for
`SC` and explicit `SCA`/`SCB`/`SCC` inputs, `igcMod`/`igbMod` gate tunneling
currents, the stress layout correction for active `SA`/`SB` layouts
including the multi-finger `SD` path, and `dioMod=0/1/2` junction diode
selectors.

Selector values outside those ranges are typed errors rather than silent
changes of physics. See `src/device/mosfet/bsim4v8/mod.rs` for the exact
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

**Memristors** — native Xyce `YMEMRISTOR` families: the TEAM model at
`LEVEL=2` (`memristor_team.rs`) and the threshold-adaptive PEM model at
`LEVEL=4` (`memristor_pem.rs`), both solving an internal state variable
alongside the terminal equations.

**Other** — switches (`switch.rs`) and thermal network elements
(`thermal.rs`). GaN HEMT qualification is feature-gated work through
generated Rust from Verilog-A (ASM-HEMT/MVSG CMC).

**Extension points** — external Verilog-A devices via the `rspice-veriloga` compiler
(`veriloga.rs`, behind the `veriloga` feature, with blake3-keyed on-disk
caching of compiled models), build-time generated Verilog-A built-ins
(`veriloga-builtins`, materialized as reusable packages under
`../rspice-veriloga-models/models/` and instantiated by model name when the
feature is enabled), and SIMD batch evaluation for diodes, BJTs,
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
| Volterra distortion (`.DISTO`) | `distortion.rs`, `engine/distortion.rs` |
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

### Periodic large-signal cards

`.PSS`, `.PAC`, `.PNOISE` and `.ENVELOPE` are parsed into typed, fully
validated cards in `netlist`. The analysis layer converts a card into the
configuration its entry point takes (`PssConfig::from(&PssCard)`,
`PacConfig::from(&PacCard)`) — a parsed deck sits below the analyses and
never names them. Every card is
case-insensitive and continues across `+` lines. A field another simulator
accepts here that RSpice cannot honour is refused with a source-located
error rather than parsed and dropped.

**`.PSS`** — shooting periodic steady state. Two disjoint forms; a token
followed by `=` is always a keyword, a token that is not is always
positional, so the two never overlap.

```
.PSS <gfreq> <tstab> <oscnode> <psspoints> <harms> <sciter> [KEY=VALUE ...]
.PSS KEY=VALUE ...
```

The positional field order is ngspice's `.pss` card. It names an oscillator
node, so it selects autonomous period detection. ngspice's trailing
`steadycoeff` and `uic` fields are refused: the shooting solver converges on
a relative periodicity norm rather than an ngspice per-node steady
coefficient, and always starts its stabilization run from the operating
point. Author `TOL=`/`ABSTOL=` and `TSTAB=`/`TSTABPERIODS=` instead. In the
positional form the keywords `FUND`, `PERIODGUESS`, `TSTAB`, `OSCNODE`,
`POINTS`, `HARMS`, `MAXITER` and `AUTONOMOUS` are refused as conflicts,
because the positional fields already bind them.

| Keyword | Positional | Meaning | Default |
| :--- | :--- | :--- | :--- |
| `FUND` | `gfreq` | Fundamental frequency (Hz) | required when driven |
| `HARMS` | `harms` | Harmonics retained in the result | 9 |
| `POINTS` | `psspoints` | Samples per period (≥ 16, ≥ 2·`HARMS`) | 256 |
| `TSTAB` | `tstab` | Stabilization time (s) | 0 |
| `TSTABPERIODS` | — | Stabilization periods when `TSTAB` is 0 | 10 driven, 20 autonomous |
| `MAXITER` | `sciter` | Maximum shooting iterations | 100 |
| `TOL` | — | Relative periodicity tolerance | 1e-6 |
| `ABSTOL` | — | Absolute tolerance | 1e-12 |
| `DAMPING` | — | Newton damping in [0.1, 1.0] | 1.0 |
| `MAXPERIODCHANGE` | — | Relative period change bound | 0.1 |
| `AUTONOMOUS` | implied | Detect the period instead of taking `FUND` | FALSE |
| `PERIODGUESS` | from `gfreq` | Autonomous period seed (s) | 1e-9 |
| `OSCNODE` | `oscnode` | Node the period is detected on | none |
| `METHOD` | — | `TRAP`, `GEAR`, `EULER` or `TRAPGEAR` | engine default |
| `VERBOSE` | — | Log convergence progress | FALSE |

`FUND` and `PERIODGUESS` set the same quantity and may not both appear;
`OSCNODE` implies `AUTONOMOUS=TRUE` and conflicts with `AUTONOMOUS=FALSE`.

**`.PAC`** — periodic small-signal AC around a periodic operating point.
The leading sweep is the input-frequency sweep.

```
.PAC DEC|LIN|OCT <np> <fstart> <fstop> INPUT=<source> OUT=V(node[,ref]) [KEY=VALUE ...]
```

| Keyword | Meaning | Default |
| :--- | :--- | :--- |
| `INPUT` | Small-signal source swept across the sweep | required |
| `OUT` | Output probe, `V(node)` or `V(node,ref)` | required |
| `MAXSIDEBAND` | Symmetric sideband range `-n..=n` | — |
| `SIDEBANDMIN` / `SIDEBANDMAX` | Explicit asymmetric range | -5 / +5 |
| `RELTOL` | Relative tolerance | 1e-3 |
| `ABSTOL` | Absolute tolerance (A) | 1e-12 |
| `FROM` | `PSS` or `HB`: which upstream to linearize around | nearest preceding |

`MAXSIDEBAND` and `SIDEBANDMIN`/`SIDEBANDMAX` are two spellings of one range
and may not be combined.

**`.PNOISE`** — periodic (cyclostationary) noise. The leading sweep is the
offset-frequency sweep.

```
.PNOISE DEC|LIN|OCT <np> <fstart> <fstop> OUT=V(node[,ref]) [KEY=VALUE ...]
```

| Keyword | Meaning | Default |
| :--- | :--- | :--- |
| `OUT` | Output probe, `V(node)` or `V(node,ref)` | required |
| `INPUT` | Source for input-referred noise | none |
| `MAXSIDEBAND` | Folded sideband bound `-n..=n` | 6 |
| `FROM` | `PSS` or `HB` | nearest preceding |

**`.ENVELOPE`** — harmonic-balance envelope continuation. It attaches to the
nearest preceding `.HB` and exposes only what the continuation executes.

```
.ENVELOPE TSTOP=<seconds> [MAXSTEP=<seconds>] [FREEZE=(<source>[,<source>...])]
```

`MAXSTEP` defaults to `TSTOP/50`, as the direct continuation entry point
does. `FREEZE` names the independent sources held at their exact time-zero
values during the carrier solve; a single source may be written without
parentheses, and a repeated source is refused.

`.PAC`, `.PNOISE` and `.ENVELOPE` each consume the periodic operating point
of an upstream analysis. Planning binds each of them to the concrete
upstream instance (`pss-001`, `hb-002`, …) and refuses a card whose upstream
the deck does not author before it.

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
| `veriloga` | no | Verilog-A device support via `rspice-veriloga`, plus serde_json/blake3/dirs for the compiled-model cache |
| `veriloga-native` | no | RSpice-owned native JIT for Verilog-A devices; requested native mode is full JIT or typed construction error |
| `veriloga-model-*` | no | Compiles one checked-in generated Verilog-A model and the shared runtime. Prefer these granular features in production to minimize compile time, peak rustc memory, and binary size |
| `veriloga-builtins-noise` | no | Adds generated noise schedules to whichever `veriloga-model-*` features are selected |
| `veriloga-builtins-models` | no | Enables every checked-in generated Verilog-A model without the optional noise schedules |
| `veriloga-builtins` | no | Backwards-compatible umbrella that enables every generated model plus noise. Each model is a reusable artifact under `../rspice-veriloga-models/models/`; refresh with `cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- regenerate-builtins` and validate with `check-builtins` |
| `wasm` | no | wasm-bindgen + `getrandom/js` so the crate builds on `wasm32-unknown-unknown`; used by `rspice-wasm` and the UI's wasm target, which also set `default-features = false` to drop rayon/SIMD |

The defaults mean the CLI, Python bindings, and the standard test run all
exercise the parallel + SIMD paths.

## Building and testing

```bash
# Build (library only)
cargo build -p rspice-core

# Full test suite — 99 integration test files under tests/
cargo test -p rspice-core

# With Verilog-A device tests (veriloga_*.rs oracle tests need the JIT)
cargo test -p rspice-core --features veriloga-native

# Generated Verilog-A built-in runtime checks (feature-gated)
cargo test -p rspice-core --features veriloga-builtins --test generated_veriloga_runtime

# Production-sized generated model build, with noise only when required
cargo build -p rspice-core --features veriloga-model-vbic13
cargo build -p rspice-core --features veriloga-model-vbic13,veriloga-builtins-noise

# Complete model catalog without compiling the optional noise schedules
cargo build -p rspice-core --features veriloga-builtins-models
```

The solver-kernel micro-benchmark (analyze/factor/refactor/solve in isolation)
lives with the rest of the benchmark rig, and measures `rspice-matrix` directly
rather than through this crate's re-export:

```bash
cargo run --release -p rspice-bench -- klu
```

Library unit tests are excluded from the default package test target by
`[lib] test = false`; run them explicitly with `cargo test -p rspice-core
--lib`. Doctests are off as well (`[lib] doctest = false`), so examples in
rustdoc are checked by review rather than by `cargo test`.

The integration suite in `tests/` includes oracle tests that pin
device and analysis behavior to reference values (diode rectifier, VBIC
excess phase, LTRA AC, native BSIM4, PSP103 via Verilog-A), RF-analysis tests
(HB Jacobian/Krylov/varactor, PSS shooting, pnoise folding, PAC conversion,
STB loop gain), parser robustness tests, and a determinism test.

### Conformance harnesses

`src/testing/ngspice_runner/` runs netlist decks through both ngspice (as
reference) and this engine, comparing per-analysis results under explicit
tolerances: suite discovery, execution, reference datasets, and validation
live there, and `tests/ngspice_regression.rs` /
`tests/ngspice_oracle_audit.rs` drive it.

`src/testing/xyce_runner.rs` does the same for the vendored Xyce regression
corpus, driven by `tests/xyce_regression.rs`. Every retained `.cir` deck is
discovered and reported, but a deck is numerically executed only when its
checked-in, relational, or explicitly qualified generated-oracle contract
can be reproduced without the upstream platform harness, which is not
vendored.

The crate ships three binaries: `rspice-ngspice-case-runner` and
`rspice-xyce-case-runner` orchestrate case runs for those two harnesses, and
`xspice_ifspec_audit` diffs the XSPICE code-model registry against the
`ifspec.ifs` files in an ngspice checkout.

For whole-process performance comparison against ngspice, see
[rspice-bench](../rspice-bench/README.md) instead.

## License

RSpice Core is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
