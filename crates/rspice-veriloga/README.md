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
               `include/`define  tokens    AST     symbol/type  device equations      HIR/MIR
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
| `canonical_ir` | Stable HIR/MIR artifact with validation, diagnostics, content digests, and backend input for generated Rust and native/JIT paths |
| `codegen` | Emits the bytecode `CompiledModel`: assignment programs, per-stamp value and Jacobian programs, reactive (charge) programs, noise metadata |
| `rust_backend` | Deterministic Verilog-A-to-Rust backend for generated built-ins: lowers canonical IR to Rust source folders, registry/support modules, manifest data, and cleanup guards used by `rspice-core`'s `veriloga-builtins` feature |
| `vm` | Bytecode interpreter and per-instance runtime context (state for `ddt`/`idt`, transition/slew filters, delay buffers, event detectors, lookup tables) |
| `laplace` / `zfilter` | State-space runtime for the `laplace_*` (s-domain) and `zi_*` (sampled-data) filter operators |
| `device` | `VerilogADevice`: the per-instance object the simulator drives — see below |
| `native/` | RSpice-owned native JIT backend (feature `native`): full native JIT or typed construction error, no bytecode fallback. x86-64 only — the AArch64 arm of the target dispatch returns `JitError::UnsupportedTarget` |
| `virtual_source` | Sealed, file-system-free source bundles: portable logical paths, include resolution restricted to the bundle plus the built-in headers, and BLAKE3 identities for the source, dependency closure, compiler contract, and runtime contract. The transport boundary for browser workers and retained run snapshots |
| `runtime_report` | In-memory compilation reports: the simulator ABI a compiled artifact exposes, its user-facing diagnostics with source positions, and which runtime targets have actually qualified for it. Performs no file-system access |
| `metrics` | Stable phase identifiers, structured timing/work-size reports, measured-result wrappers, and opt-in performance budgets shared by the compiler and offline Rust backend |
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
let measured = compiler.compile_measured(source)?;           // model + phase metrics
let model = compiler.compile_module(source, Some("nmos"))?;  // pick one of several
let model = compiler.compile_file(path)?;                    // from disk, with includes
let model = compiler.compile_file_module(path, Some("nmos"))?;
let file  = compiler.compile_file_with_metadata(path)?;      // + include dependency list
let file  = compiler.compile_file_module_with_metadata(path, Some("nmos"))?;

// Canonical HIR/MIR artifact
let ir = compiler.compile_canonical_ir(source)?;
let measured_ir = compiler.compile_canonical_ir_measured(source)?;
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

Runtime reports and file-metadata results carry `PipelineMetrics`.
`compile_measured` and `compile_canonical_ir_measured` expose the same data
for source-only artifact calls. `RustTranspiler::transpile_measured` reports
the offline backend's CFG lowering, differentiation, optimization,
scheduling, emission, and exact generated byte/line counts. Timings are
operational evidence and never participate in artifact or cache identities.
`CompilerOptions::performance_budget` and
`RustTranspileOptions::performance_budget` can enforce opt-in total or
per-phase limits; empty budgets are the default.
The `*_with_control` measured entry points accept a `PipelineControl` for
cooperative cancellation and progress callbacks. Cancellation is polled at
phase boundaries and inside the packed-AD and CFG-optimization hot loops, so
stopping a large compact-model generation does not wait for the entire
transpile to finish.

Two environment variables also provide diagnostic-only output:
`RSPICE_DEBUG_PP=1` writes the preprocessed source beside the input file
as `*.pp.va`, and `RSPICE_VERILOGA_PHASE_TRACE=1` (or the narrower
`RSPICE_VERILOGA_CANONICAL_IR_PHASE_TRACE=1`) prints per-phase timings to
stderr.

`CompilerOptions` carries three fields that change generated artifacts, all
of them preprocessor inputs: `include_paths` (searched by
`` `include ``, and only by the file-system entry points), `defines`, and
`undefines` (drops a standard macro so `defines` can replace it). The
performance budget changes only whether a slow invocation is accepted and
is excluded from compiler-contract identities.

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
selects pre-generated artifacts from `rspice-veriloga-models/models/` and does
not link this compiler at all.

## Generating the built-in device models

The crate ships one binary, `rspice-veriloga-gen`. It walks a tree of
Verilog-A sources, compiles each module to canonical IR, and emits a Rust
device folder per module into
`crates/rspice-veriloga-models/models/`, with one Cargo package per model plus
a feature-selectable catalog, `registry.rs`, and `manifest.txt`. Cargo can
compile those packages in parallel and reuse an unchanged model artifact
without rebuilding it through `rspice-core`. That generated Rust — not this
compiler — is what `rspice-core`'s `veriloga-builtins` feature builds.

The generated backend preserves the compact-model parameter convention in
the source: unmarked parameters are model-card parameters, while
`(* type="instance" *)` marks per-device geometry and switches. Canonical
dependency analysis separates model, instance, temperature, timestep, and
Newton work. Model-stage outputs are interned by the final model-parameter
values and `$param_given` bits, so devices bound to the same card reuse one
Verilog-derived preprocessing result; instance geometry and solver state are
never shared.

Automatic differentiation keeps exact sparse lane shapes. Its fixed point uses
a bounded compact bit matrix for ordinary compact models and falls back to
sparse storage before hostile lane counts can force an oversized dense
allocation. One-lane derivative values emit as plain `f64`; widths two and above
remain packed `Lanes<N>` values, avoiding both scalar source explosion and
one-element array overhead. Generator progress reports scalar/packed value
counts, seed count, and maximum width so representation changes are visible on
the shipped corpus.

When the same cached model/instance predicate guards at least three Newton
regions, the emitter evaluates a two-variant specialization candidate. It is
accepted only when neither outcome controls a loop and the complete specialized
body grows by at most 2%; otherwise the ordinary CFG is emitted unchanged.
There is no exponential variant set and no frozen numerical condition. Generator
progress reports model/instance structural-guard counts, their Newton impact,
the scheduling time, and generated bytes so this policy is observable on the
real corpus.

```bash
# Full regeneration; must rewrite every device, registry.rs and manifest.txt
cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- \
    regenerate-builtins [--models PATH] [--out PATH] [--jobs N]

# One model at a time while iterating; writes to target/veriloga-generated-subset
cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- \
    generate-builtins-subset --filter FILTER [--models PATH] [--out PATH] [--jobs N]

# Verify the checked-in output is current; no writes
cargo run -p rspice-veriloga --bin rspice-veriloga-gen -- \
    check-builtins [--models PATH] [--out PATH]
```

`--models` defaults to `models/veriloga`. `--jobs` defaults to the
available parallelism capped at 4, or to
`RSPICE_VERILOGA_GENERATOR_JOBS` when that is set, and is in any case
clamped to the number of modules being generated. `--filter` belongs to
the subset command only: full
regeneration rejects it, because a partial rewrite would leave the
registry and manifest describing devices that are no longer there. The
subset command says so in its output — it deliberately does not rewrite
`registry.rs` or `manifest.txt`, so its output is for inspection, not for
committing. The `generator` profile matters: it is release-optimized, and
compiling the full model corpus under the dev profile is impractically
slow.

Staleness is detected by two digests recorded in `manifest.txt`. The
`source_tree_digest` covers the model sources; the `generator_digest` is
`RSPICE_VERILOGA_GENERATOR_SOURCE_DIGEST`, computed in `build.rs` over
this crate's own sources plus the workspace `Cargo.toml`/`Cargo.lock`, so
that editing the compiler invalidates its output exactly like editing a
model does. `check-builtins` compares both and fails with the exact
regeneration command when either has moved.

Two markers steer discovery inside the model tree: a `.rspice-veriloga-skip`
file excludes a directory, and a `.rspice-veriloga-profile` file supplies
the `defines`/`undefines` a source needs to preprocess.

There is one emitter and no tier to select. A model either lowers through the
canonical CFG backend or generation fails naming the construct that stopped it;
there is no fallback to regress onto and no environment variable that changes
which emitter runs.

## Building and testing

```bash
cargo build -p rspice-veriloga
cargo test  -p rspice-veriloga                      # interpreter paths
cargo test  -p rspice-veriloga --features native    # + native JIT contract
```

The integration tests under `tests/` group into five bands.

**Compilation and language semantics** — end-to-end compilation
(`compile_models.rs`), multi-module selection (`module_selection.rs`),
parameter constraints and `aliasparam` (`parameter_validation.rs`,
`aliasparam.rs`), array variables (`array_vars.rs`), expression truth and
equality rules (`expression_semantics.rs`), `analysis()` queries
(`analysis_queries.rs`), and `syntax_integrity.rs`, which pins that
unsupported constructs are rejected rather than silently dropped.

**Runtime numerics** — evaluation and Jacobians against hand-derived
companion-model values (`device_eval.rs`), indirect contributions
(`indirect_contributions.rs`), `$mfactor` scaling (`mfactor.rs`), solver
companion coefficients (`integration_methods.rs`), state installation
(`runtime_configuration.rs`), and `numeric_integrity.rs`, which pins that
non-finite values are reported rather than zeroed away.

**Stateful operators** — `zi_*` filters (`zi_filters.rs`), events
(`event_semantics.rs`), timers (`timer_semantics.rs`), `last_crossing`
(`last_crossing_semantics.rs`), `$bound_step`/`$discontinuity`
(`timestep_control.rs`), and `stateful_operator_idempotence.rs`, which
pins that Newton re-evaluation never consumes accepted history.

**Artifacts and backends** — canonical IR validation (`canonical_ir.rs`),
runtime reports and cross-artifact digest drift
(`runtime_compile_report.rs`), sealed bundles (`virtual_source.rs`), the
native no-fallback contract (`native_contract.rs`), and the Rust backend
(`rust_backend.rs`, plus `generated_output_audit.rs` auditing the
checked-in generated devices).

**Production-model frontiers** — PSP 103.6 via the IHP SG13G2 open PDK
(`psp103_frontier.rs`) and the shipped CMC r3_cmc and JUNCAP200 models
(`cmc_frontier.rs`). `bsim4_frontier.rs` is optional and activates only
when `RSPICE_BSIM4_VA` points at an externally supplied clean BSIM4.8
source.

Two whole-corpus gates are `#[ignore]`d, because they are qualification
evidence rather than a fast check. Run them explicitly when working on the
compiler frontier or on generated built-ins:

```bash
cargo test -p rspice-veriloga --test shipped_cmc_compile shipped_veriloga_models_compile_end_to_end -- --ignored --nocapture
cargo test -p rspice-veriloga --test rust_backend_frontier shipped_rust_backend_frontier -- --ignored --nocapture
```

Engine-level oracle tests that compare compiled models against reference results live in
[rspice-core's test suite](../rspice-core/README.md#building-and-testing)
(`veriloga_*.rs`).

## License

RSpice Verilog-A is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
