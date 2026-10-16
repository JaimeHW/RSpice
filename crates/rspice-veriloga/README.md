# rspice-veriloga

A Verilog-AMS compiler written in Rust, covering both halves of LRM 2.4: the
supported analog subset commonly called Verilog-A, and the supported discrete
subset (IEEE 1364-2005 digital constructs carrying four-state values and
real-valued nets), with connect modules inserted automatically at discipline
boundaries. There is one language and one pipeline: `.va` and `.vams` are the
same dialect here, with no mode selector between them. A host that executes
the digital half of a mixed module opts in through
`CompilerOptions::enable_ams`, which widens what a runtime report carries, not
what the front end accepts.

It is separate from `rspice-core` because it is a compiler, not an engine: four
independent backends consume one canonical artifact, and one of them runs
offline, ahead of the build, with this crate absent from the shipped link.

## What's in here

```
source text ─▶ preprocessor ─▶ lexer ─▶ parser ─▶ semantic ─▶ IR (+ autodiff) ─▶ canonical IR
               `include/`define  tokens    AST     symbol/type  device equations      │
                                                   resolution   + derivatives         │
┌────────────────────────┬─────────────────┬─────────────────────┬────────────────────┘
▼                        ▼                 ▼                     ▼
codegen                  native/           wasm_jit/             rust_backend
bytecode CompiledModel   feature "native"  feature "wasm-jit"    feature "rust-codegen"
run by vm                machine code      one wasm module       offline Rust emitter
│                        │                 │                     │
└────────────────────────┴─────────────────┘                     ▼
                         ▼                                       rspice-veriloga-models
            device: VerilogADevice instance                      (this crate is not linked)
```

The backends are not interchangeable at run time. `vm`, `native/` and
`wasm_jit/` are driven through `VerilogADevice` in this process; `rust_backend`
runs offline and emits Rust that is checked in as feature-selectable crates
under `rspice-veriloga-models/` and compiled against the generated-model
runtime ABI.

| Module | Contents |
| :--- | :--- |
| `preprocessor` | `` `include ``/`` `define ``/`` `ifdef `` expansion; supplies the built-in `disciplines.vams` and `constants.vams` when not found on disk; records include dependencies |
| `lexer` / `parser` / `ast` | Tokenizer, recursive-descent parser producing `SourceFile`/`Module` ASTs in both ANSI and non-ANSI port styles, and the AST types |
| `semantic` | Symbol table, type inference, discipline validation; rejects unsupported constructs with explicit errors rather than dropping them |
| `ir` / `expr_converter` | Lowering to device-equation IR; the `autodiff` submodule generates derivative assignments by symbolic forward-mode differentiation, so Jacobians are analytic rather than finite-difference |
| `canonical_ir` | Stable HIR/MIR artifact with validation, diagnostics and content digests: the single input every backend consumes |
| `codegen` / `vm` | Emits the bytecode `CompiledModel` — assignment programs, per-stamp value and Jacobian programs, reactive programs, noise metadata — and the interpreter and per-instance context that run it |
| `device` | `VerilogADevice`: the per-instance object the simulator drives |
| `native/` | RSpice-owned native JIT: full native JIT or typed construction error, no bytecode fallback. x86-64 and desktop AArch64 on macOS, Linux and Windows, with platform-native executable memory and unwind registration |
| `wasm_jit/` | Browser JIT: emits one WebAssembly module per model through `wasm-encoder` and validates it with `wasmparser`. The owning Web Worker compiles and instantiates it |
| `rust_backend` | Offline Verilog-A-to-Rust emitter: source folders, registry and support modules, manifest data and the staleness digests behind `rspice-veriloga-gen` |
| `laplace` / `zfilter` | State-space runtime for the `laplace_*` (s-domain) and `zi_*` (sampled-data) filter operators |
| `connect` / `four_state` | Connect-module insertion at discipline boundaries, with its own clause-by-clause table of the LRM 2.4 clause 7 rules and what it refuses by name; and the four-state logic values of the discrete half |
| `virtual_source` | Sealed, file-system-free source bundles: portable logical paths, includes resolved inside the bundle plus the built-in headers, and BLAKE3 identities for the source, dependency closure, compiler contract and runtime contract. The transport boundary for browser workers and retained run snapshots |
| `runtime_report` | In-memory compilation reports: the simulator ABI an artifact exposes, user-facing diagnostics with source positions, and which backends have qualified for it. Performs no file-system access |
| `specialist` / `metrics` | Structural specialization findings a model author sees; stable phase identifiers, timing and work-size reports, and opt-in performance budgets |
| `disciplines` / `stdlib` / `types` | Discipline database, the built-in headers (LRM 2.4 physical constants), the type system, function registry and parameter-range types |
| `source` / `error` | Source maps and spans, and the `CompileError`/`CompileResult` types |
| `analog_tasks` / `analog_occurrences` / `time_scale` | Ordered analog task calls and assignment notifications shared by every lowering; declared module timing and delay rounding |
| `generated_source` / `json_float` | Data-only generated-source report types, available without `rust-codegen`; the JSON-safe float encoding serialized artifacts use |

## Boundaries

- The runtime ABI generated models link against is
  [`rspice-veriloga-runtime`](../rspice-veriloga-runtime); the generated
  artifacts are [`rspice-veriloga-models`](../rspice-veriloga-models).
- Model caching, device routing and the netlist directives that load a source
  file (`.va`, `.veriloga`, `.hdl`, `.vams` and `.verilog` all name the same
  directive) belong to [`rspice-core`](../rspice-core/README.md), whose suite
  also holds the engine-level oracle tests, as `veriloga_*.rs`.

## Entry points

Every compile starts at `VerilogACompiler::new(CompilerOptions)`. The entry
points differ along two axes: which artifact comes back, and where the source
comes from.

- `compile` / `compile_module` — source text to a bytecode `CompiledModel`.
  Compiling without a module name errors if the source declares more than one
  module, listing their names.
- `compile_runtime` — source text to a `RuntimeCompileReport`: bytecode model
  and canonical IR from one parse, cross-validated and qualified against the
  available backends. Nothing is written to disk.
- `compile_canonical_ir` / `compile_canonical_ir_module` — the canonical
  artifact alone, for a caller that only drives a JIT or the offline emitter.
- `compile_file_with_metadata` / `compile_file_runtime_with_metadata` — the
  same products from a path, returning the include dependencies preprocessing
  discovered.
- `compile_virtual_runtime` — the same products from a sealed
  `VirtualSourceBundle`, never touching the file system.
- `VerilogADevice::try_new(name, model, nodes)` — the instance the simulator
  drives directly, with no trait indirection: set parameters and simulation
  state, then `evaluate()` / `stamp()` / `stamp_reactive()` /
  `compute_jacobian()` inside the Newton loop, and `noise_sources()` for noise.
  `try_new_with_canonical_ir` is the form that admits the native JIT. The
  compiled model is shared, so a thousand instances of one model compile once.
- `rust_backend::RustTranspiler` — the offline emitter; `rspice-veriloga-gen`
  is the binary that drives it over a model tree.

```rust,ignore
let compiler = VerilogACompiler::new(CompilerOptions::default());
let report = compiler.compile_runtime(source, Some("resistor"))?;
let device = VerilogADevice::try_new("R1", report.model, &[1, 0])?;
```

## Language support

Unsupported input is always a `CompileError` naming the construct and its span.
The compiler does not silently drop what it cannot lower, and `semantic`
carries tests pinning that.

- **Analog operators**: `ddt`, `idt`, `idtmod`, `ddx`, `limexp`, `absdelay`,
  `transition`, `slew`, `laplace_zp/zd/np/nd`, `zi_nd/zp/zd/np`,
  `last_crossing`, `$limit`, `$table_model`. The `zi_*` sample periods may
  depend on scalar parameters and freeze per instance when its parameters
  resolve. The integration rule for `ddt`/`idt` is not a compile-time choice:
  the engine supplies companion coefficients per timestep, so one compiled
  model runs under backward Euler or Gear-2/trapezoidal.
- **Noise**: `white_noise`, `flicker_noise`, `noise_table`, `noise_table_log`,
  injected into `.noise` with amplitude scaling and mode gating. Distinct
  processes are uncorrelated; reusing one process through an assigned
  expression preserves coherent contributions, including cancellation. The
  trailing name argument is a display label, not a correlation key.
- **Indirect contributions**: `V(x): lhs == rhs` as constraint rows on a branch
  unknown, one per source, with analysis-constant control flow and an access
  function — optionally under `ddt`, `idt` or `idtmod` — on the left side.
- **System functions**: `$temperature`, `$vt`, `$thermal_vt`, `$abstime`,
  `$realtime`, `$simparam`, `$param_given`, `$port_connected`, `$mfactor`
  (with automatic multiplicity scaling), `$bound_step`, `$discontinuity`, and
  the `analysis()` query. AC and noise operating points expose `static`;
  transient ones expose `tran`, `ic` and `static`. An unknown query name
  evaluates false.
- **Data**: 1-D array variables with compile-time and runtime indexing and
  shadowed derivatives, runtime-bounded loops, parameters with dependent
  defaults, ranges and exclusions, localparams, `aliasparam`, attribute
  instances (`(* desc, units *)`), string parameters.
- **Structure**: internal nodes, named branches, scalar ground nets, user
  disciplines beyond electrical, ANSI and non-ANSI port styles. Ground
  qualifiers inherit the declared discipline (`thermal g; ground g;`) and typed
  grounds (`ground thermal g;`) are accepted; the shorthand `ground g;`
  defaults to electrical. Derived natures retain their ancestry, tolerance,
  units and calculus relationships. An analog net bus on a port takes one deck
  node per member, leading index first.
- **Control flow**, lowered to guarded dataflow: `if`/`else`, `case`,
  compile-time-bounded `for`/`repeat`, and the event controls `initial_step`,
  `final_step`, `cross`, `above` and `timer`.
- **Library maps and configurations** (IEEE 1364-2005 clause 13):
  `.options hdl_libmap="lib.map"` reads a map relative to the deck, with
  wildcard library paths, nested `include`, and `config` blocks; a deck selects
  a configuration with `module=<config>` as it selects a module, and `%l`
  prints the bound `library.cell`. Binding runs right after parsing and leaves
  an ordinary module set. Hierarchical configurations, several design cells in
  one configuration, empty liblists and a library's `-incdir` are refused by
  name.
- **Functions**: user-defined analog functions, inlined, including
  `output`/`inout` arguments, and the built-in math set (`abs`, `sqrt`, `exp`,
  `ln`, `log`, `log10`, trig and hyperbolic functions with their inverses,
  `floor`, `ceil`, `pow`, `hypot`, `atan2`, `min`, `max`).

Refused by name, each a compile error rather than a silent miscompile:
`noise_table` file input, which requires the `{f, p, ...}` pair list inline;
array locals in analog functions; `output`/`inout` analog-function arguments
used inside a conditional expression or any other context that must stay free
of side effects; ground declarations on module ports, which are connected to
ground in the containing circuit instead; a runtime loop that feeds a `ddx`
result back into its own operand; and analog `$fatal` and `$stop`, whose
simulation-control semantics are not implemented. `$error` is refused in
pre-simulation initialization, where it must stop the run. These checks follow
transitive function calls and branches removed by constant folding.
`$display`, `$write`, `$strobe`, `$monitor`, `$info`, `$warning` and ordinary
analog `$error` are accepted and discarded with a warning: they produce no
runtime output.

Real `%` is a remainder, differentiated as `da - trunc(a/b)*db` away from its
discontinuities. Analog real-to-integer assignment rounds to nearest, half away
from zero; that conversion and the integer bitwise and shift rules live in
`rspice-veriloga-runtime`, so every backend shares them.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `rust-codegen` | On by default. Offline Rust-source emitter and the `rspice-veriloga-gen` binary. Disable default features for runtime-only compilation; the serialized artifact types in `generated_source` stay available |
| `native` | RSpice-owned native JIT. Requested native mode is full native JIT or typed construction error, with no bytecode fallback. Pulls in the Windows APIs for executable memory (`windows-sys`) |
| `wasm-jit` | Browser JIT backend, adding `wasm-encoder` and `wasmparser`. Reached through `rspice-core/veriloga-wasm-jit` |
| `native-bytecode-contract-tests` | Internal. Implies `native` and exposes `compile_native`, which JITs straight from the bytecode model without a canonical artifact. Backend contract tests only: production native users supply canonical IR and must not enable it |

`rspice-core` disables this crate's defaults, because simulation does not need
Rust-source emission, and maps the rest as `veriloga` (interpreter),
`veriloga-native` and `veriloga-wasm-jit`, adding a blake3-keyed on-disk cache
for compiled models. Its `veriloga-builtins` feature is a different path
entirely: it selects pre-generated artifacts and does not link this compiler.

Backend qualification is report-only by default, so an editor can show every
target's readiness. Release tooling that must not fall back to the interpreter
passes `RuntimeQualificationOptions::GENERATED_RUST_REQUIRED`,
`RuntimeQualificationOptions::NATIVE_REQUIRED` or
`rejecting_interpreter_fallback()`; an unavailable or refusing backend then
returns a typed `BackendQualification` error instead of running bytecode.

A signed macOS executable that enables the hardened runtime must be signed
with [`RSpice.entitlements`](../rspice-app/macos/RSpice.entitlements), which
grants only Apple's narrow `MAP_JIT` and JIT write-callback permissions.

## Generating the built-in device models

`rspice-veriloga-gen` walks a tree of Verilog-A sources, compiles each module
to canonical IR, and emits one Cargo package per model into
`crates/rspice-veriloga-models/models/` alongside the catalog's `registry.rs`
and `manifest.txt`.

```bash
# Full regeneration; rewrites every device, registry.rs and manifest.txt
cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- \
    regenerate-builtins [--models PATH] [--out PATH] [--jobs N]

# One model at a time while iterating; writes to target/veriloga-generated-subset
cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- \
    generate-builtins-subset --filter FILTER [--models PATH] [--out PATH] [--jobs N]

# Verify the checked-in output is current; no writes
cargo run -p rspice-veriloga --bin rspice-veriloga-gen -- \
    check-builtins [--models PATH] [--out PATH]
```

`--models` defaults to `models/veriloga`. `--jobs` defaults to the available
parallelism capped at 4, or to `RSPICE_VERILOGA_GENERATOR_JOBS` when that is
set, and is clamped to the number of modules generated. `--filter` belongs to
the subset command alone: full regeneration rejects it, because a partial
rewrite would leave the registry and manifest describing devices that are no
longer there. The subset command rewrites neither `registry.rs` nor
`manifest.txt`, so its output is for inspection, not for committing. The
`generator` profile is release-optimized; the full corpus under the dev profile
is impractically slow.

Staleness rests on two digests recorded in `manifest.txt`. The
`source_tree_digest` covers the model sources under `--models`; the
`generator_digest` covers the `GENERATOR_SOURCE_DIGEST_INPUTS` list in
`src/rust_backend/builtins.rs`, every compiler source a generator run compiles,
`build.rs`, `Cargo.toml` and the workspace `Cargo.toml`/`Cargo.lock`, so
editing the compiler invalidates its output exactly as editing a model does.
The in-process runtime backends are the deliberate omission, and the comment on
that list says why they cannot move a generated byte. `check-builtins` compares
both and fails with the exact regeneration command when either moved.

Two markers steer discovery: `.rspice-veriloga-skip` excludes a directory, and
`.rspice-veriloga-profile` supplies the `defines`/`undefines` a source needs to
preprocess. A profile also accepts `namespace SOURCE::MODULE=HEX_ID`, with
`SOURCE` relative to the profile directory and `HEX_ID` eight hexadecimal
digits, which preserves published generated paths and duplicate model names
across source repairs, one declaration per source and module. Discovery rejects
a missing source or module, and generation rejects a colliding folder or
package name before writing anything.

There is one emitter and no tier to select. A model either lowers through the
canonical CFG backend or generation fails naming the construct that stopped it.

## Testing

```bash
cargo test -p rspice-veriloga                      # interpreter paths
cargo test -p rspice-veriloga --features native    # + native JIT contract
```

Most suites under `tests/` require `rust-codegen`, which is on by default; the
per-suite `required-features` entries in `Cargo.toml` name them. Whole-corpus
gates are `#[ignore]`d, because they are qualification evidence rather than a
fast check. Run them explicitly when working on the compiler frontier or on
generated built-ins:

```bash
cargo test -p rspice-veriloga --test shipped_cmc_compile \
    shipped_veriloga_models_compile_end_to_end -- --ignored --nocapture
cargo test -p rspice-veriloga --test canonical_device \
    the_whole_corpus_reports_what_the_canonical_backend_carries -- --ignored --nocapture
```

`bsim4_frontier.rs` activates only when `RSPICE_BSIM4_VA` points at an
externally supplied BSIM4.8 source.

Licensed under the [RSpice Personal Use License](../../LICENSE).
