# rspice-core

The simulation engine: netlist parsing, device models, circuit assembly,
Newton-Raphson solving, and the analysis algorithms. The CLI, the desktop and
browser application, the Python and WebAssembly bindings, and the cloud engine
adapter all solve a deck through this one crate. It decides analysis identity,
result shape and failure classification; how a result is printed, which exit
status a failure maps to, and how a command line configures a run belong to
the frontends.

## Architecture

A netlist flows through the engine in stages:

```text
netlist text
    │  netlist/         nom-based lexer + parser → Netlist AST
    │                   (elements, models, subcircuits, analysis cards,
    │                    parameters, .MEAS statements)
    ▼
 Netlist
    │  engine/builder   subcircuit flattening, parameter resolution,
    │                   model routing, device construction
    ▼
 CircuitData            circuit/: struct-of-arrays storage, one typed array
    │                   per device kind
    │  engine/matrix    the MNA sparsity pattern, reserved once and frozen,
    │                   with O(1) stamp handles thereafter
    ▼
 StaticMatrix           sparse CSC matrix + reusable LU (rspice-matrix)
    │  engine/          Newton-Raphson with convergence aids, transient
    │                   integration, frequency sweeps
    ▼
 SimulationResult / analysis-specific result types
```

## What's in here

| Module | Contents |
| :--- | :--- |
| `netlist/` | Lexer and parser (element, command, conditional, scoping, source-spec, transmission-line and Laplace-synthesis submodules), the `Netlist` AST, subcircuit flattener, hierarchical paths, `.include`/`.lib` resolution, parameter scoping and the complex-valued `.PARAM` evaluator, `.ALTER`/`.DATA` multi-run expansion, SPEF parasitics, XSPICE cards, and the fail-closed Spectre model-library adapter |
| `circuit/` | `CircuitData` struct-of-arrays storage (one typed array per device kind under `storage/`), construction, linear stamping, nonlinear device hooks, magnetic coupling, mixed-signal and XSPICE event scheduling, attachment of XSPICE, Verilog-A and generated devices, transient device noise, and read-only introspection including the `.OP` device report |
| `device/` | Device models: passives (`passive/`, including coupled inductors and Jiles-Atherton cores), diode and BJT (`semiconductor/`), the MOS and FET family (`mosfet/`: Berkeley MOS, BSIM1 to BSIM4, BSIM-SOI, EKV, VDMOS, JFET, MESFET/HFET), independent, controlled and behavioral sources, transmission lines, memristors, switches, thermal elements, and the adapters for runtime (`veriloga`) and generated (`veriloga_builtins`) Verilog-A devices |
| `solver/` | The Newton-Raphson loop, damping strategies, source-stepping and pseudo-transient controllers, arc-length continuation, and `SimulationResult`. Sparse LU is [`rspice-matrix`](../rspice-matrix), re-exported here |
| `engine/` | The orchestrator: `Engine`, one driver per analysis family, the builder that elaborates a `Netlist` and routes model cards, MNA reservation and stamping, independent-source evaluation, convergence aids, configuration resolution, the analysis-capability table, Verilog-A source loading and caching, and `ControlCircuit`, the host for ngspice control scripts |
| `analysis/` | Analysis algorithms and result types, `.MEAS` evaluation (`measure`, `measure_signals`, `measurements`), Fourier and post-processing, temperature handling, process corners, and signal-integrity helpers |
| `execution/` | Target-neutral deck planning (`DeckPlan`, run axes and coordinates), the typed result document every frontend publishes, signal schemas and projections, event and bus export, the resumable control-flow machine, and bounded, cancellable output sinks |
| `expr/` | Bytecode expression engine for behavioral sources: parser, AST, compiler and VM, with symbolic derivatives and time-enclosure bounds |
| `io/` | Waveform files: SPICE RAW read and write (whole-result and streaming), VCD read and write, HDF5 publication, Xyce `.prn` output, and JSON decoding that refuses numeric information loss |
| `library/` | `.lib` parsing with corner selection, the registry of libraries compiled into the binary, foundation-card fallbacks for unbound devices, and discovery of shipped SPICE and Verilog-A model packs |
| `xspice/` | XSPICE code models: `CodeModel`, `CodeModelRegistry`, instance and context types, the discrete-event scheduler, bundled analog, digital and bridge models, the native IEEE 1364-2005 digital Verilog host (`verilog`), and an `ifspec.ifs` parser with conformance helpers that diff the registry against an ngspice checkout |
| `numerics/` | Integration methods and companion coefficients, Krylov and eigenspectrum kernels, exact rank arithmetic for constraint elimination, and RustFFT resource qualification |
| `config`, `resource` | `SimulationConfig`, `ConvergenceConfig` and their selectors; `ResourceLimits` and the limit errors that govern untrusted and batch workloads |
| `diagnostics`, `identity`, `op_label` | Structured warnings and convergence-quality metrics; stable analysis-instance and run-coordinate identities; the closed vocabulary of operating-point report labels |
| `abort_signal`, `constants`, `time_compat` | Cooperative cancellation (`AbortSignal`, `AtomicAbort`, `NoAbort`); physical and simulation constants; a wall-clock shim that is a no-op on `wasm32` |
| `simd` | `wide`-based vector, reduction and integration kernels (with the `simd` feature) |

## Device routing

The builder decides which implementation a card reaches. The generated column
applies when that model's `veriloga-model-*` feature is compiled in; the
generated module is then the only device created for the card.

| Element | Native | Generated Verilog-A route |
| :--- | :--- | :--- |
| `M` | `LEVEL=1/2/3/6` Berkeley MOS; `4/5` BSIM1/BSIM2; `8/49` BSIM3 3.3.0 (Xyce's 3.2.2 equations for `LEVEL=49` under the Xyce dialect, which refuses `LEVEL=8`); `9` MOS9, or Xyce's BSIM3 3.2.2 under the Xyce dialect and, under best-available, for a card with a decisive BSIM3 signature; `14/54` BSIM4 v4.8; `10/55/56/57` BSIM-SOI (`55` FD, `56` DD, `57` PD); `18` and `VDMOS` cards; `260` EKV 2.6; `301` a native EKV3 slice for the VA-Models/Xyce 150 nm cards | `LEVEL=70/104/107/108/110/111/260/1000/2002/10240/70470`, and model types such as `BSIMBULK`, `BSIMCMG`, `BSIMIMG`, `PSP104`, `HISIMHV`, `HISIMSOI`, `L_UTSOI`, `MVSG`, `ASMHEMT`, `ANGELOV` and `EKV3` |
| `Q` | Gummel-Poon (no `LEVEL`, `0/1/2`); VBIC (`4/9/11/12/13`) | Types `HICUML0`, `HICUML2`, `MEXTRAM505`, the `BJT505*`/`BJTD505*` variants and the `VBIC*` spellings; `LEVEL=11/12` under the Xyce dialect |
| `D` | `LEVEL=0/1/2/3` | Types `DIODE_CMC` and `JUNCAP200`; `LEVEL=200/2002` |
| `R` | Plain and model-card resistors | Types `R2_CMC`, `R2_ET_CMC` and `R3_CMC`; `LEVEL=1002/1003` |
| `J` | JFET level 1; Parker-Skellern JFET2 (`LEVEL=2`) | None |
| `Z` | MESFET `0/1`, MESA `2/3/4`, HFET1 `5`, HFET2 `6` | None |
| `X` | The deck's subcircuits, then authored Verilog-A modules | Then a generated model by its model name |

What fails closed, and how:

- A `Q` card whose type names HICUM or MEXTRAM, built without that module, is
  refused by an error naming the family, the generated module and the feature
  that supplies it. `Q LEVEL=8/23/230/234/504/505`, other simulators' selectors
  for HICUM, HBT_X and MEXTRAM, are refused in every build: a level names a
  dialect's device, not a module.
- An `M` level that belongs to a compact model with no native port (BSIM-CMG,
  PSP, HiSIM, B4SOI, BSIM6, MVS and others) is refused, naming that family,
  unless a compiled-in generated module serves it; any other unlisted level is
  refused with the supported list. Neither falls back to the simplified MOS
  equations.
- `M LEVEL=260` is the one deliberate overlap: the generated `ekv_va` when it
  is compiled in, the native EKV 2.6 otherwise.

## Analyses

Analysis drivers live in `engine/`, and algorithms and result types in
`analysis/`. The families are operating point and DC sweep, AC, transient
(with checkpoint and resume), noise and transient noise, distortion,
pole-zero, sensitivity, transfer function, DC mismatch, S-parameters,
loop-gain stability, Monte Carlo, process corners and `.STEP` sweeps, and the
periodic family built on PSS shooting and harmonic balance: PAC, periodic
noise, PXF, PSTB, periodic S-parameters, envelope continuation, and
quasi-periodic QPSS/QPAC/QPNOISE/QPXF. `.MEAS`, `.FOUR` and `.FFT`
post-process results. The root README's
[analysis table](../../README.md#analyses) says how each is invoked.

Whether an analysis may run on a circuit is one table, `engine::capability_cell`
(exported whole as `engine::capability_matrix`), indexed by analysis and by
instance family: native SPICE, XSPICE analog and event models, runtime and
generated Verilog-A, mixed Verilog-AMS, and real-number nets. Every analysis
consults it before solver work, and a refused cell names the analysis, the
instance, its module and the reason. Generated compact models, for example,
are refused by every periodic analysis.

### Periodic analysis cards

`.PSS`, `.PAC`, `.PXF`, `.PNOISE`, `.PSTB` and `.ENVELOPE` are validated whole
at parse time (`netlist/parser/periodic_cards.rs`). Keywords are
case-insensitive and bind once, cards continue across `+` lines, and a field
another simulator accepts that RSpice cannot honour is refused with a
source-located error rather than parsed and dropped.

```text
.PSS <gfreq> <tstab> <oscnode> <psspoints> <harms> <sciter> [KEY=VALUE ...]
.PSS KEY=VALUE ...
.PAC    DEC|LIN|OCT <np> <fstart> <fstop> KEY=VALUE ...
.PXF    DEC|LIN|OCT <np> <fstart> <fstop> KEY=VALUE ...
.PNOISE DEC|LIN|OCT <np> <fstart> <fstop> KEY=VALUE ...
.PSTB PROBE=<instance> [KEY=VALUE ...]
.ENVELOPE TSTOP=<t> [MAXSTEP=<t>] [FREEZE=(<source>[,<source>...])]
```

| Card | Keywords |
| :--- | :--- |
| `.PSS` | `FUND`, `PERIODGUESS`, `AUTONOMOUS`, `OSCNODE`, `HARMS`, `TSTAB`, `TSTABPERIODS`, `MAXITER`, `TOL`, `ABSTOL`, `DAMPING`, `MAXPERIODCHANGE`, `POINTS`, `METHOD`, `VERBOSE` |
| `.PAC` | `INPUT` and `OUT` (required), `MAXSIDEBAND`, `SIDEBANDMIN`, `SIDEBANDMAX`, `RELTOL`, `ABSTOL`, `PACMAG`, `INCLUDEDC`, `FROM` |
| `.PXF` | `INPUT` and `OUT` (required), `INPUTSIDEBAND`, `OUTSIDEBAND`, `MAXSIDEBAND`, `RELTOL`, `ABSTOL`, `FROM` |
| `.PNOISE` | `OUT` (required), `INPUT`, `MAXSIDEBAND`, `INPUTSIDEBAND`, `OUTSIDEBAND`, `NOISEREF`, `INTEGRATEDNOISE`, `NOISESUMMARY`, `FROM`; sampling controls `SAMPLING`, `SAMPLEPHASE`, `REFOUT`, `PERIODS`, `THRESHOLD`, `DIRECTION`, `OCCURRENCE`, `PHASETOL`, `MINSLEW`, and the `REF`-prefixed forms of the last five |
| `.PSTB` | `PROBE` (required), `MAXHARM`, `NMULTS`, `STABILITYTHRESHOLD`, `DETECTSUBHARMONICS`, `EIGENTOL` |
| `.ENVELOPE` | `TSTOP` (required), `MAXSTEP`, `FREEZE` |

The positional `.PSS` form is ngspice's oscillator card. Its fields bind
`FUND`, `PERIODGUESS`, `AUTONOMOUS`, `OSCNODE`, `TSTAB`, `POINTS`, `HARMS` and
`MAXITER`, and ngspice's trailing `steadycoeff` and `uic` are refused.
`FROM=PSS|HB` names the carrier a dependent card linearizes; `.PSTB` reads a
shooting `.PSS` and takes none.

## Boundaries

- Sparse matrix storage and LU: [`rspice-matrix`](../rspice-matrix).
- Compiling Verilog-AMS: [`rspice-veriloga`](../rspice-veriloga/README.md);
  the generated models it emits are
  [`rspice-veriloga-models`](../rspice-veriloga-models/README.md), built
  against [`rspice-veriloga-runtime`](../rspice-veriloga-runtime).
- Writing result files and checkpoints to disk atomically:
  [`rspice-output`](../rspice-output).
- Command-line flags, exit codes and output formats:
  [`rspice-cli`](../rspice-cli/README.md).
- Reference-simulator conformance suites:
  [`rspice-conformance`](../../tools/rspice-conformance), which sees only this
  crate's public API, so every assertion takes the path a user's deck takes.

## Entry points

- `Netlist::parse`: deck text to a validated AST. The first line of a deck is
  its title, never an element.
- `Engine::new(SimulationConfig)` or `Engine::default()`, then its `run_*`
  methods. Each analysis family has variants that take an `&dyn AbortSignal`
  (`_with_abort`, `_and_abort`); frontends call those, which is what backs
  Ctrl-C in the CLI and `KeyboardInterrupt` in Python.
- `SimulationConfig`, `ConvergenceConfig`, `ConvergencePreset` and
  `SpiceDialect`: tolerances, convergence aids, and the dialect
  (best-available, ngspice or Xyce) whose semantics apply.
- `ResourceLimits`: the budget applied to untrusted input.
- `execution::DeckPlan::from_netlist`: the analyses, run axes and coordinates
  a deck authors, in the order every frontend executes them.
- `SimulationError::descriptor()`: a stable code, category and retry flag, so
  a frontend classifies a failure without parsing its message.

## Solvers and convergence

- **Sparse LU.** Real systems use `rspice-matrix`'s KLU-class kernel, whose
  stored pivots make refactorization on the frozen pattern cheap; faer handles
  complex AC-family solves and is the certified fallback.
  `.OPTIONS SOLVER=AUTO|KLU|FAER` or `RSPICE_SOLVER` selects the real backend.
  Left unstated, measured fill decides, and the Xyce dialect reproduces Xyce's
  own KLU factorization policy.
- **Newton-Raphson** (`solver/newton.rs`), with tolerance, iteration-budget and
  residual checks in `engine/convergence/`.
- **Convergence aids**, tried in order of cost when plain Newton fails: GMIN
  stepping, source stepping, pseudo-transient and arc-length continuation,
  bundled by `ConvergencePreset` as `fast`, `default` and `robust`. An aid
  returns only the unmodified circuit's solution, certified by the residual
  test a direct solve faces, never a deformed stage.
- **Damping** (`solver/damping.rs`): line search, junction voltage limiting,
  Bank-Rose, and the combined default, selected by `DampingStrategy`.
- **Transient integration**: backward Euler, trapezoidal, Gear-2, and the
  trapezoidal/Gear-2 hybrid default (`IntegrationMethod`), with LTE-based
  timestep control and breakpoint handling. Integration order is 1 or 2, and
  Xyce's `TIMEINT MINORD`/`MAXORD` accept only those values.

## Persisted formats

- **Result document.** `execution::AnalysisResultDocument`, identified by
  `ANALYSIS_RESULT_DOCUMENT_SCHEMA` and `ANALYSIS_RESULT_DOCUMENT_VERSION`, is
  the one typed result every frontend publishes. The CLI, Python, WebAssembly
  and engine-adapter surfaces format it and define none of their own.
- **Transient checkpoints.** `engine::TransientCheckpoint` is versioned,
  line-oriented text whose floats round-trip bit-exactly;
  `TransientCheckpointEncoding::Packed` wraps a zlib-compressed copy in a
  versioned binary envelope with a BLAKE3 integrity seal. Resume refuses a
  checkpoint whose deck or resolved-configuration identity differs from the
  run's.
- **Monte Carlo journals.** `engine::MonteCarloCheckpoint`: completed trials,
  failures included, as versioned binary with exact value bits and a checksum.
- **Operating-point labels.** `op_label::OpLabel` is the closed label set of a
  device operating-point report. Saved reports are read back into it, so no
  label a reader cannot resolve is ever written.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `parallel` | On by default. Rayon and `portable-atomic` for the parallel paths in `engine/`, `circuit/` and `device/` |
| `simd` | On by default. `wide`: the `simd` module's kernels and a vectorized Newton update test |
| `faer-parallel` | On by default. faer's Rayon-parallel factorization, forwarded to `rspice-matrix` and `rspice-veriloga-runtime` |
| `wasm` | Adds `wasm-bindgen`. The browser builds enable it with `default-features = false`, which also drops Rayon and `wide` |
| `veriloga` | Runtime Verilog-AMS devices through `rspice-veriloga`'s bytecode interpreter: a deck's `.va`, `.veriloga`, `.vams`, `.hdl` or `.verilog` source compiles at elaboration. Native targets cache compiled models on disk under the platform cache directory, or `RSPICE_VERILOGA_CACHE_DIR` |
| `veriloga-native` | Implies `veriloga`. RSpice's native JIT: a requested native device is fully JIT-compiled or refused with a typed construction error, never run on bytecode |
| `veriloga-wasm-jit` | Implies `veriloga`. Browser JIT: the owning Web Worker compiles and instantiates the module, while this crate keeps the authenticated artifacts and the synchronous solver integration |
| `veriloga-builtins-base` | The engine integration every generated model shares, selecting no model |
| `veriloga-model-*` | One flag per generated model, as listed in `Cargo.toml`: compiles that checked-in model and the shared integration. Prefer these in production builds for compile time, peak `rustc` memory and binary size. The models are regenerated as [`rspice-veriloga`](../rspice-veriloga/README.md#generating-the-built-in-device-models) describes |
| `veriloga-builtins-noise` | Adds the generated noise schedules to whichever models are selected |
| `veriloga-builtins-models` | Every generated model, without the noise schedules |
| `veriloga-builtins` | Every generated model with its noise schedules |

## Building and testing

```bash
cargo test -p rspice-core                             # suites in tests/
cargo test -p rspice-core --lib                       # unit tests
cargo test -p rspice-core --features veriloga         # + veriloga_*.rs suites
cargo test -p rspice-core --features veriloga-native  # same, native JIT
cargo test -p rspice-core --features veriloga-model-diode-cmc \
    --test generated_veriloga_feature_shards          # one generated model
cargo build -p rspice-core \
    --features veriloga-model-vbic13,veriloga-builtins-noise
```

`[lib] test = false` keeps unit tests out of the default package target,
hence the separate `--lib` run, and `doctest = false` leaves rustdoc examples
checked by review. The WebAssembly suites run under `wasm-bindgen-test-runner`
with `--target wasm32-unknown-unknown --release --no-default-features
--features wasm,veriloga-wasm-jit`. The solver-kernel benchmark measures
`rspice-matrix` directly: `cargo run --release -p rspice-bench -- klu`.

Licensed under the [RSpice Personal Use License](../../LICENSE).
