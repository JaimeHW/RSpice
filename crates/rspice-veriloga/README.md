# RSpice Verilog-A

A Verilog-A compiler written in Rust: it takes Verilog-A source (the analog
subset of the Verilog-AMS LRM 2.4), compiles it through parser, semantic,
device-IR, canonical-IR, and backend stages, and provides the runtime
(`VerilogADevice` + bytecode VM, plus the RSpice-owned native JIT contract) that lets the
compiled model behave as a device inside the rspice-core simulator —
evaluating currents and charges, producing an analytic Jacobian via automatic
differentiation, and contributing noise sources. It is the backend of the
engine's `veriloga`/`veriloga-native` features, the CLI's `rspice compile-va`,
the GUI's Verilog-A dialog, and the generated-Rust built-in path used by
`rspice-core`'s `veriloga-builtins` feature.

## Compilation pipeline and module map

```
source text ─▶ preprocessor ─▶ lexer ─▶ parser ─▶ semantic ─▶ IR (+ autodiff) ─▶ canonical IR
               `include/`define  tokens    AST     symbol/type  device equations   HIR/MIR/OptIR
                                                   resolution   + derivatives            │
                        ┌──────────────────────────────────────────────────────┬─────────┤
                        ▼                                                      ▼         ▼
              codegen: bytecode CompiledModel                          native/ (JIT)  rust_backend
                        │                                              feature        offline
                        ├──▶ vm (interpreter)                          "native"       Rust emitter
                        └──▶ device (VerilogADevice instance) ◀────────────┘
```

The three backends are not interchangeable at run time. `vm` and `native/`
are both driven through `VerilogADevice` in this process; `rust_backend`
runs offline, ahead of the build, and its output is compiled into
`rspice-core` as ordinary Rust with this crate absent from the link.

| Module | Contents |
| :--- | :--- |
| `preprocessor` | `` `include ``/`` `define ``/`` `ifdef `` expansion; supplies built-in `disciplines.vams` and `constants.vams` when not found on disk; records include dependencies |
| `lexer` | Tokenizer for the full token set, including system-function and preprocessor tokens |
| `parser` | Recursive-descent parser producing `SourceFile`/`Module` ASTs; handles ANSI and non-ANSI port styles |
| `ast` | AST types for expressions, statements, declarations, analog operators, event expressions |
| `semantic` | Symbol table, type inference, discipline validation; rejects unsupported constructs with explicit errors |
| `ir` / `expr_converter` | Lowering to device-equation IR; the `autodiff` submodule generates derivative ("shadow") assignments by symbolic forward-mode differentiation, so Jacobians are analytic rather than finite-difference |
| `canonical_ir` | Stable HIR/MIR/OptIR artifact with validation, diagnostics, content digests, and backend input for generated Rust and future native/JIT paths |
| `codegen` | Emits the bytecode `CompiledModel`: assignment programs, per-stamp value and Jacobian programs, reactive (charge) programs, noise metadata |
| `rust_backend` | Deterministic Verilog-A-to-Rust backend for generated built-ins: lowers canonical IR to Rust source folders, registry/support modules, manifest data, and cleanup guards used by `rspice-core`'s `veriloga-builtins` feature |
| `vm` | Bytecode interpreter and per-instance runtime context (state for `ddt`/`idt`, transition/slew filters, delay buffers, event detectors, lookup tables) |
| `laplace` / `zfilter` | State-space runtime for the `laplace_*` (s-domain) and `zi_*` (sampled-data) filter operators |
| `device` | `VerilogADevice`: the per-instance object the simulator drives — see below |
| `native/` | RSpice-owned native JIT backend (feature `native`): full native JIT or typed construction error, no bytecode fallback. x86-64 only — the AArch64 arm of the target dispatch returns `JitError::UnsupportedTarget` |
| `virtual_source` | Sealed, file-system-free source bundles: portable logical paths, include resolution restricted to the bundle plus the built-in headers, and BLAKE3 identities for the source, dependency closure, compiler contract, and runtime contract. The transport boundary for browser workers and retained run snapshots |
| `runtime_report` | In-memory compilation reports: the simulator ABI a compiled artifact exposes, its user-facing diagnostics with source positions, and which runtime targets have actually qualified for it. Performs no file-system access |
| `disciplines` / `stdlib` / `types` | Discipline database, the built-in `disciplines.vams`/`constants.vams` headers (LRM 2.4 physical constants), the type system, function registry, and parameter-range types |
| `source` / `error` | Source maps/spans and the `CompileError`/`CompileResult` types |

## Public API

Every entry point hangs off `VerilogACompiler`, and they differ along two
axes: which artifact you get back, and where the source comes from.

```rust
use rspice_veriloga::{VerilogACompiler, CompilerOptions};

let compiler = VerilogACompiler::new(CompilerOptions::default());

// Bytecode CompiledModel
let model = compiler.compile(source)?;                       // exactly one module
let model = compiler.compile_module(source, Some("nmos"))?;  // pick one of several
let model = compiler.compile_file(path)?;                    // from disk, with includes
let model = compiler.compile_file_module(path, Some("nmos"))?;
let file  = compiler.compile_file_with_metadata(path)?;      // + include dependency list
let file  = compiler.compile_file_module_with_metadata(path, Some("nmos"))?;

// Canonical HIR/MIR/OptIR artifact
let ir = compiler.compile_canonical_ir(source)?;
let ir = compiler.compile_canonical_ir_module(source, Some("nmos"))?;
let ir = compiler.compile_file_canonical_ir_with_metadata(path, Some("nmos"))?;

// Both at once, from one parse/analysis pass
let report = compiler.compile_runtime(source, Some("nmos"))?;
let both   = compiler.compile_file_runtime_with_metadata(path, Some("nmos"))?;

// Sealed bundle, no file-system access at all
let built = compiler.compile_virtual_runtime(&bundle, "nmos", limits)?;
let built = compiler.compile_virtual_runtime_diagnosed(&bundle, "nmos", limits)?;
```

The `*_runtime` family is the one to reach for when a caller needs both
artifacts: preprocessing, lexing, parsing, and semantic analysis run once
and the bytecode model and canonical IR are emitted from the same analyzed
module, then cross-validated. `compile_runtime` and the virtual-bundle
APIs are sealed — they consult the built-in standard headers but never the
configured `include_paths` or the disk — so a caller that needs includes
resolves its own graph into a `VirtualSourceBundle` first.
`compile_virtual_runtime_diagnosed` differs from `compile_virtual_runtime`
only in failure: it keeps source-authentic diagnostics mapped back to
bundle paths instead of collapsing to a bare `CompileError`.

Two environment variables affect compilation, both diagnostic only:
`RSPICE_DEBUG_PP=1` writes the preprocessed source beside the input file
as `*.pp.va`, and `RSPICE_VERILOGA_PHASE_TRACE=1` (or the narrower
`RSPICE_VERILOGA_CANONICAL_IR_PHASE_TRACE=1`) prints per-phase timings to
stderr.

`CompilerOptions` carries three fields that change what the compiler
produces, all of them preprocessor inputs: `include_paths` (searched by
`` `include ``, and only by the file-system entry points), `defines`, and
`undefines` (drops a standard macro so `defines` can replace it).

`enable_ams`, `strict_mode`, and `integration_order` are **reserved**:
they are accepted and they participate in the compiler-contract identity
hash, so changing one invalidates a cached compilation, but no compiler
phase reads them yet. In particular `integration_order` does not pick the
`ddt`/`idt` integration rule — the engine supplies companion coefficients
per timestep, so one compiled model serves backward Euler and Gear-2
alike.

Multi-module foundry files are supported via `compile_module` /
`compile_file_module_with_metadata`; compiling without a module name
errors if the source declares more than one module, listing their names.

### Runtime: `VerilogADevice`

The simulator instantiates `VerilogADevice::new(name, Arc<CompiledModel>,
nodes)` and then drives it directly (no trait indirection): set parameters
(`set_parameter`, `resolve_parameter_defaults`), set simulation state
(`set_temperature`, `set_time`, `set_timestep`, `set_analysis_type`,
`update_voltages`), then `evaluate()` / `stamp()` /
`stamp_reactive()` / `compute_jacobian()` inside the Newton loop, and
`noise_sources()` for noise analysis. Transient control flows back through
`transient_bound_step()` (`$bound_step`) and `discontinuity_pending()`
(`$discontinuity`). The compiled model is shared — a thousand instances of
one model compile (and JIT) once. `is_using_native()` reports whether the
JIT is active for diagnostics.

## Language support

The supported subset, as documented in the crate docs (`src/lib.rs`) and
exercised by the test suite:

- **Analog operators**: `ddt`, `idt`, `idtmod`, `ddx`, `limexp`,
  `absdelay`, `transition`, `slew`, `laplace_zp/zd/np/nd`,
  `zi_nd/zp/zd/np`, `last_crossing`, `$limit`, `$table_model`. The
  integration rule for `ddt`/`idt` is not a compile-time choice — the
  engine supplies the companion coefficients per timestep, so the same
  compiled model runs under backward Euler or Gear-2/trapezoidal
- **Noise**: `white_noise`, `flicker_noise`, `noise_table`,
  `noise_table_log`, injected into `.noise` with amplitude scaling and
  mode gating
- **Indirect contributions**: `V(x): lhs == rhs` as constraint rows on a
  branch unknown
- **System functions**: `$temperature`, `$vt`, `$thermal_vt`, `$abstime`,
  `$realtime`, `$simparam`, `$param_given`, `$port_connected`, `$mfactor`
  (with automatic multiplicity scaling), `$bound_step`, `$discontinuity`,
  and the `analysis()` analysis-name query
- **Data**: 1-D array variables (compile-time and runtime indexing, with
  shadowed derivatives), runtime-bounded loops, parameters with dependent
  defaults/ranges/exclusions, localparams, `aliasparam`, attribute
  instances (`(* desc, units *)`), string parameters
- **Structure**: internal nodes, named branches, ground nets, user
  disciplines beyond electrical (thermal, mechanical, …), ANSI and
  non-ANSI port styles
- **Control flow**, lowered to guarded dataflow: `if`/`else`, `case`,
  compile-time-bounded `for`/`repeat` loops, event controls
  (`initial_step`, `final_step`, `cross`, `above`, `timer`)
- **Functions**: user-defined analog functions (inlined, including
  `output`/`inout` arguments), and the built-in math set (`abs`, `sqrt`,
  `exp`, `ln`, `log`, `log10`, trig/hyperbolic and their inverses,
  `floor`, `ceil`, `pow`, `hypot`, `atan2`, `min`, `max`)

**Known limitations**, every one of them a compile error rather than a
silent miscompile: `noise_table` file input (inline the `{f, p, ...}` pair
list); parameter-dependent `zi_*` sample periods; multi-dimensional
arrays; array locals in analog functions; and `output`/`inout`
analog-function arguments used inside conditional expressions or any other
context that must stay free of side effects.

Two constructs are accepted but inert, so they are worth knowing about:
noise sources are always mutually uncorrelated (the trailing name argument
is a label carried into the results, not a correlation key), and the
no-effect system tasks — `$display`, `$write`, `$strobe`, `$monitor`,
`$info`, `$warning`, `$error`, `$fatal`, `$finish`, `$stop` — parse and
then do nothing, so a model cannot print from the analog block.

## Feature flags

| Feature | Default | Effect |
| :--- | :--- | :--- |
| `native` | off | RSpice-owned native JIT for Verilog-A devices; requested native mode is full native JIT or typed construction error, with no bytecode fallback. Pulls in the platform APIs for executable memory (`windows-sys` / `libc`) |
| `native-bytecode-contract-tests` | off | Internal. Implies `native` and exposes `compile_native`, which JITs straight from the bytecode model without a canonical IR artifact. Backend contract tests only — production native users must supply canonical IR and must not enable it |
| `ams` | off | Declared for Verilog-AMS mixed-signal support; currently gates no code in the crate |

`rspice-core` maps these as `veriloga` (interpreter) and `veriloga-native`
(native JIT) and adds a blake3-keyed on-disk cache for compiled models on
top. Its `veriloga-builtins` feature is a different path entirely: it
compiles the pre-generated Rust in `rspice-core/src/device/veriloga_generated/`
and does not link this compiler at all.

## Building and testing

```bash
cargo build -p rspice-veriloga
cargo test  -p rspice-veriloga                      # interpreter paths
cargo test  -p rspice-veriloga --features native    # + native JIT contract
cargo test  -p rspice-veriloga --test rust_backend   # generated-Rust backend
```

The integration test files under `tests/` cover end-to-end compilation
(`compile_models.rs`), runtime evaluation and Jacobians
(`device_eval.rs`), array variables, `aliasparam`, indirect contributions,
`zi_*` filters, timestep control, `$mfactor` scaling, multi-module
selection, native no-fallback contract coverage (`native_contract.rs`),
canonical IR validation, the Rust backend (`rust_backend.rs`), and real production
models: EKV 2.6 physics, PSP 103, and CMC model frontier tests, plus optional
BSIM4 coverage when `RSPICE_BSIM4_VA` points at an externally supplied clean
source file. The broad shipped-CMC compile-frontier tests are ignored by
default because they are qualification evidence, not a normal fast check; run
them explicitly when working on generated built-ins:

```bash
cargo test -p rspice-veriloga --test shipped_cmc_compile shipped_veriloga_models_compile_end_to_end -- --ignored --nocapture
```

Engine-level oracle tests that compare compiled models against reference results live in
[rspice-core's test suite](../rspice-core/README.md#building-and-testing)
(`veriloga_*.rs`).

## License

RSpice Verilog-A is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
