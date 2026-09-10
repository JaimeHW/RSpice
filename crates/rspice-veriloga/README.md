# rspice-veriloga

A Verilog-AMS compiler written in Rust, covering both halves of LRM 2.4: the
supported analog subset commonly called Verilog-A, and the supported discrete
subset (IEEE 1364-2005 digital constructs carrying four-state values and
real-valued nets), with connect modules inserted automatically at discipline
boundaries.

Source is compiled through parser, semantic, device-IR, canonical-IR, and
backend stages, and the crate supplies the runtime that lets the result behave
as a device inside the rspice-core simulator: evaluating currents and charges,
producing an analytic Jacobian by automatic differentiation, and contributing
noise sources. It backs the engine's `veriloga`, `veriloga-native`, and
`veriloga-wasm-jit` features, the CLI's `rspice compile-va`, the GUI's
Verilog-A dialog, and the generated-Rust built-in path behind
`rspice-core`'s `veriloga-builtins`.

## Compilation pipeline and module map

```
source text ─▶ preprocessor ─▶ lexer ─▶ parser ─▶ semantic ─▶ IR (+ autodiff) ─▶ canonical IR
               `include/`define  tokens    AST     symbol/type  device equations      │
                                                   resolution   + derivatives         │
┌────────────────────────┬─────────────────┬─────────────────────┬────────────────────┘
▼                        ▼                 ▼                     ▼
codegen                  native/           wasm_jit/             rust_backend
bytecode CompiledModel   feature "native"  feature "wasm-jit"    offline Rust emitter
run by vm                machine code      one wasm module       compiled into rspice-core
│                        │                 │                     │
└────────────────────────┴─────────────────┘                     ▼
                         ▼                                       rspice-veriloga-models
            device: VerilogADevice instance                      (this crate is not linked)
```

The four backends are not interchangeable at run time. `vm`, `native/`, and
`wasm_jit/` are all driven through `VerilogADevice` in this process;
`rust_backend` runs offline, ahead of the build, and its output is compiled
into `rspice-core` as ordinary Rust with this compiler absent from the link.

Analog real-to-integer assignments round to nearest, with half cases away
from zero, before automatic differentiation. Scalar and array writes,
variable initializers, and function arguments and results share this conversion.
The checked signed-32-bit conversion and bitwise/shift rules live in
`rspice-veriloga-runtime::integer` and are used by the generated Rust path
as well as the portable, native, and Wasm evaluators. This does not yet
provide signed-32-bit wrapping for all ordinary integer arithmetic.

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
| `device` | `VerilogADevice`: the per-instance object the simulator drives; see below |
| `native/` | RSpice-owned native JIT backend (feature `native`): full native JIT or typed construction error, no bytecode fallback. Supports x86-64 and desktop AArch64 on macOS, Linux, and Windows, with platform-native executable-memory and unwind registration |
| `wasm_jit/` | Browser JIT backend (feature `wasm-jit`): emits a WebAssembly module per model through `wasm-encoder`, validates it with `wasmparser`, and dispatches into it. The owning Web Worker compiles and instantiates the module |
| `connect` | Connect-module insertion at discipline boundaries, for mixed Verilog-AMS decks |
| `four_state` | The four-state logic values and their propagation, for the discrete half of Verilog-AMS |
| `specialist` / `reaching_definition` / `timing_contract` | Structural specialization candidates, the reaching-definition analysis they rest on, and the digital timing contract |
| `array_index` / `integer_runtime` / `numeric_literal` / `json_float` | Array indexing rules, integer semantics, literal parsing, and the exact float round-trip the artifact digests depend on |
| `canonical_compat` | Compatibility between canonical-IR artifact versions |
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
APIs are sealed: they consult the built-in standard headers but never the
configured `include_paths` or the disk, so a caller that needs includes
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

`enable_ams` lets the runtime compilation APIs return the analog model
alongside a canonical digital plan. It defaults to false: callers enabling
it must execute that plan on the event scheduler, since the analog model
alone cannot execute digital behavior. Canonical-IR compilation can lower
digital processes independently of this option.

`strict_mode` and `integration_order` are **reserved**. They participate in
the compiler-contract identity but currently gate no behavior.
`integration_order` does not pick the
`ddt`/`idt` integration rule; the engine supplies companion coefficients
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
(`$discontinuity`). The compiled model is shared: a thousand instances of
one model compile (and JIT) once. `is_using_native()` reports whether the
JIT is active for diagnostics.

Native compilation coalesces requests for the same artifact and allows up to
two independent compilations at once, bounded by available parallelism.
Ready cache hits do not wait for unrelated compilation. Retention is limited
to 1,024 entries and 512 MiB of executable images by default (overridable with
`RSPICE_VERILOGA_NATIVE_CACHE_MAX_BYTES`); one oversized image is retained to
prevent repeated compilation. Live devices retain their images after eviction.
The cancellable constructor, `try_new_with_canonical_ir_and_control`, lets a
waiting caller leave without interrupting compilation needed by other callers.
An already-started compilation finishes before its owner reports cancellation.

## Language support

Finite nested `ddx` readbacks are supported by canonical AD and generated
Rust. The portable evaluator allocates the derivative orders needed through
assignments and arrays, including the extra order required by Newton's
Jacobian. Runtime loops that feed a `ddx` result back into its own operand
remain unsupported. Native/Wasm assignment and internal-variable readback
passes still have higher-derivative limits; native readback explicitly rejects
simultaneous `ddx` self-updates instead of publishing partially updated shadows.
This is not complete Verilog-AMS support.

Real `%` uses remainder semantics. Away from its discontinuities, Jacobians,
`ddx` readbacks and noise gains follow `da - trunc(a/b)*db` for `a % b`.
A constant divisor does not require forming the quotient for differentiation.

The supported subset, as documented in the crate docs (`src/lib.rs`) and
exercised by the test suite:

- **Analog operators**: `ddt`, `idt`, `idtmod`, `ddx`, `limexp`,
  `absdelay`, `transition`, `slew`, `laplace_zp/zd/np/nd`,
  `zi_nd/zp/zd/np`, `last_crossing`, `$limit`, `$table_model`. The
  `zi_*` sample periods may depend on scalar parameters and are frozen per
  instance when its parameters are resolved. The integration rule for
  `ddt`/`idt` is not a compile-time choice; the
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
  and the `analysis()` analysis-name query. AC/noise operating points expose
  `static`; transient operating points expose `tran`, `ic`, and `static`.
  Phase changes retain analog initialization state. Global-event analysis
  lists match physical analysis names; unknown query names evaluate false.
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
list); multi-dimensional
arrays; array locals in analog functions; and `output`/`inout`
analog-function arguments used inside conditional expressions or any other
context that must stay free of side effects.

Distinct noise processes are uncorrelated. Reusing one process through an
assigned expression preserves coherent contributions, including cancellation.
The trailing name argument is a display label, not a correlation key.

Analog `$fatal` and `$stop` are rejected because their simulation-control
semantics are not implemented. `$error` is also rejected during pre-simulation
initialization, where it must prevent simulation from proceeding. These checks
include transitive function calls and branches removed by constant folding.

`$finish` calls retain their argument snapshots and execution order in a
transactional journal. Runtime, generated, and mixed-signal device hosts can
consume accepted calls through `CircuitData::visit_accepted_analog_tasks`.
`Engine::run_with_outcome` reports initialization `$finish` as normal completion
without a numerical result. Accepted DC operating-point and sweep `$finish`
retain the final solution and the accepted prefix of a sweep. Early sweep
termination solves `final_step` before committing model history, and a failed
final solve remains an error. Rebuilt parameter/temperature sweep points restore
the accepted state without executing declaration or analog initial programs again.
The run's completion state is separate from caller
cancellation and is not reused by later analyses. Mixed digital processes start
only after the circuit-wide analog initialization barrier. Result-only engine
APIs cannot represent an initialization finish and return `ModelFinished`.
Accepted transient/frequency-domain termination, their early `final_step` delivery,
source-located diagnostic formatting, and frontend outcome handling remain
incomplete; this does not establish full `$finish` support.

The engine's discrete host also accepts digital-only modules and analog
procedural blocks without contribution equations. Digital boundary ports still
use the circuit's A/D and D/A bridges, and analog initialization retains its
ordering before digital startup. AC rejects unsupported discrete simulation
even when a module has no ports or matrix unknowns.

`$display`, `$write`, `$strobe`, `$monitor`, `$info`, `$warning`, and ordinary
analog `$error` are still discarded with a warning. They produce no runtime
output. Models that rely on these missing task behaviors are not supported.

## Feature flags

| Feature | Default | Effect |
| :--- | :--- | :--- |
| `native` | off | RSpice-owned native JIT for Verilog-A devices; requested native mode is full native JIT or typed construction error, with no bytecode fallback. Pulls in the platform APIs for executable memory (`windows-sys` / `libc`) |
| `wasm-jit` | off | Browser JIT backend, adding `wasm-encoder` and `wasmparser`. Reached through `rspice-core/veriloga-wasm-jit` |
| `native-bytecode-contract-tests` | off | Internal. Implies `native` and exposes `compile_native`, which JITs straight from the bytecode model without a canonical IR artifact. Backend contract tests only: production native users must supply canonical IR and must not enable it |

`rspice-core` maps these as `veriloga` (interpreter) and `veriloga-native`
(native JIT) and adds a blake3-keyed on-disk cache for compiled models on
top. Its `veriloga-builtins` feature is a different path entirely: it
selects pre-generated artifacts from `rspice-veriloga-models/models/` and does
not link this compiler at all.

Signed macOS executables that enable the hardened runtime must be signed with
[`RSpice.entitlements`](../rspice-ui/macos/RSpice.entitlements). It grants only
Apple's narrow `MAP_JIT` and JIT write-callback permissions; it does not disable
executable-memory protection or library validation. CI ad-hoc signs a native
test executable with this exact file and executes generated ARM64 code through
the callback-only publication path.

Optional backend qualification is report-only by default so the editor can
show every target's readiness. Release tooling must use
`RuntimeQualificationOptions::GENERATED_RUST_REQUIRED`,
`RuntimeQualificationOptions::NATIVE_REQUIRED`, or
`rejecting_interpreter_fallback()` when interpreter execution is not permitted.
A requested backend that is unavailable or rejects the model then returns a
typed `BackendQualification` compile error; it never silently runs bytecode.

## Generating the built-in device models

The crate ships one binary, `rspice-veriloga-gen`. It walks a tree of
Verilog-A sources, compiles each module to canonical IR, and emits a Rust
device folder per module into
`crates/rspice-veriloga-models/models/`, with one Cargo package per model plus
a feature-selectable catalog, `registry.rs`, and `manifest.txt`. Cargo can
compile those packages in parallel and reuse an unchanged model artifact
without rebuilding it through `rspice-core`. That generated Rust, not this
compiler, is what `rspice-core`'s `veriloga-builtins` feature builds.

The generated backend preserves the compact-model parameter convention in
the source: unmarked parameters are model-card parameters, while
`(* type="instance" *)` marks per-device geometry and switches. Canonical
dependency analysis separates model, instance, temperature, timestep, and
Newton work. Model-stage outputs are interned by the final model-parameter
values and `$param_given` bits, so devices bound to the same card reuse one
Verilog-derived preprocessing result; instance geometry and solver state are
never shared.

Xyce Q `LEVEL=11` and `LEVEL=12` route to the generated VBIC 1.3 modules.
After the three or four electrical terminals, respectively, a card may expose
six optional internal connections in order: `dt`, `cx`, `ci`, `bx`, `bi`, `ei`.
The three-terminal thermal variant supplies `dt` through its declared fourth
terminal; other optional connections bind existing generated node slots. Omitted
connections retain their internal equations and state, and excess terminals are
rejected. This mapping reuses the shipped model equations and requires no
additional generated model variant. On every VBIC variant, the card's `AREA`,
`M`, and `MULT` factors combine into the model multiplicity, including factors
supplied through parameter expressions.

Generated devices retain `$bound_step` requests across conditional calls,
runtime loops, and nested analog instances. The transient engine uses the
smallest active bound, including zero as a request for its supported minimum;
rollback and checkpoints preserve accepted bounds. Ordinary analog
`$discontinuity` calls preserve independent transient and Newton hints in the
portable VM, native backend, and generated Rust. Nonnegative degrees request a
transient restart on a rising edge; `-1` prevents Newton convergence without
requesting a time event. Degree expressions must be constant, numeric, and
finite integers at least `-1`; instance-dependent values are checked during
evaluation. Custom `$limit` callbacks accept function-identifier selectors;
structured callback bodies and their convergence hints remain incomplete.
Runtime state version 10 and transient checkpoint format 41 preserve limiter
history and reject older payloads that omitted it or used older discontinuity
semantics. Executable limiter value and derivative programs share the previous
Newton history throughout one evaluation. Generated Rust also applies the
limiter's affine RHS correction; integrating that correction into the executable
VM, native, and WebAssembly stamping paths remains outstanding.

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
subset command says so in its output: it deliberately does not rewrite
`registry.rs` or `manifest.txt`, so its output is for inspection, not for
committing. The `generator` profile matters: it is release-optimized, and
compiling the full model corpus under the dev profile is impractically
slow.

Staleness is detected by two digests recorded in `manifest.txt`. The
`source_tree_digest` covers the model sources under `--models`; the
`generator_digest` is computed at generation time over the
`GENERATOR_SOURCE_DIGEST_INPUTS` list in `src/rust_backend/builtins.rs`,
every compiler source a generator run compiles, `build.rs` and
`Cargo.toml`, plus the workspace `Cargo.toml`/`Cargo.lock`, so that
editing the compiler invalidates its output exactly like editing a model
does. The in-process runtime backends are the deliberate omission, and the
comment on that list says why they cannot move a generated byte.
`check-builtins` compares both digests and fails with the exact
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

**Compilation and language semantics**: end-to-end compilation
(`compile_models.rs`), multi-module selection (`module_selection.rs`),
parameter constraints and `aliasparam` (`parameter_validation.rs`,
`aliasparam.rs`), array variables (`array_vars.rs`), expression truth and
equality rules (`expression_semantics.rs`), `analysis()` queries
(`analysis_queries.rs`), and `syntax_integrity.rs`, which pins that
unsupported constructs are rejected rather than silently dropped.

**Runtime numerics**: evaluation and Jacobians against hand-derived
companion-model values (`device_eval.rs`), indirect contributions
(`indirect_contributions.rs`), `$mfactor` scaling (`mfactor.rs`), solver
companion coefficients (`integration_methods.rs`), state installation
(`runtime_configuration.rs`), and `numeric_integrity.rs`, which pins that
non-finite values are reported rather than zeroed away.

**Stateful operators**: `zi_*` filters (`zi_filters.rs`), events
(`event_semantics.rs`), timers (`timer_semantics.rs`), `last_crossing`
(`last_crossing_semantics.rs`), `$bound_step`/`$discontinuity`
(`timestep_control.rs`), and `stateful_operator_idempotence.rs`, which
pins that Newton re-evaluation never consumes accepted history.

**Artifacts and backends**: canonical IR validation (`canonical_ir.rs`),
runtime reports and cross-artifact digest drift
(`runtime_compile_report.rs`), sealed bundles (`virtual_source.rs`), the
native no-fallback contract (`native_contract.rs`), and the Rust backend
(`rust_backend.rs`, plus `generated_output_audit.rs` auditing the
checked-in generated devices).

**Production-model frontiers**: PSP 103.6 via the IHP SG13G2 open PDK
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

Licensed under the [RSpice Personal Use License](../../LICENSE).
