# RSpice Verilog-A

A Verilog-A compiler written in Rust: it takes Verilog-A source (the analog
subset of the Verilog-AMS LRM 2.4), compiles it through a five-stage
pipeline into a bytecode `CompiledModel`, and provides the runtime
(`VerilogADevice` + bytecode VM, plus an optional Cranelift JIT) that lets
the compiled model behave as a device inside the rspice-core simulator —
evaluating currents and charges, producing an analytic Jacobian via
automatic differentiation, and contributing noise sources. It is the
backend of the engine's `veriloga`/`veriloga-native` features, the CLI's
`rspice compile-va`, and the GUI's Verilog-A dialog.

## Compilation pipeline and module map

```
source text ──▶ preprocessor ──▶ lexer ──▶ parser ──▶ semantic ──▶ IR (+ autodiff) ──▶ codegen
                `include/`define   tokens     AST      symbol/type    device equations    bytecode
                                                       resolution     + derivatives       CompiledModel
                                                                                              │
                                            runtime:  vm (interpreter)  ◀─────────────────────┤
                                                      native/ (Cranelift JIT, feature "native")
                                                      device (VerilogADevice instance)
```

| Module | Contents |
| :--- | :--- |
| `preprocessor` | `` `include ``/`` `define ``/`` `ifdef `` expansion; supplies built-in `disciplines.vams` and `constants.vams` when not found on disk; records include dependencies |
| `lexer` | Tokenizer for the full token set, including system-function and preprocessor tokens |
| `parser` | Recursive-descent parser producing `SourceFile`/`Module` ASTs; handles ANSI and non-ANSI port styles |
| `ast` | AST types for expressions, statements, declarations, analog operators, event expressions |
| `semantic` | Symbol table, type inference, discipline validation; rejects unsupported constructs with explicit errors |
| `ir` / `expr_converter` | Lowering to device-equation IR; the `autodiff` submodule generates derivative ("shadow") assignments by symbolic forward-mode differentiation, so Jacobians are analytic rather than finite-difference |
| `codegen` | Emits the bytecode `CompiledModel`: assignment programs, per-stamp value and Jacobian programs, reactive (charge) programs, noise metadata |
| `vm` | Bytecode interpreter and per-instance runtime context (state for `ddt`/`idt`, transition/slew filters, delay buffers, event detectors, lookup tables) |
| `laplace` / `zfilter` | State-space runtime for the `laplace_*` (s-domain) and `zi_*` (sampled-data) filter operators |
| `device` | `VerilogADevice`: the per-instance object the simulator drives — see below |
| `native/` | Cranelift JIT (feature `native`): compiles bytecode programs to machine code with a hybrid execution plan — chunks that compile run native, anything unsupported falls back to the interpreter per-step, and loop conditions always interpret |
| `disciplines` / `stdlib` / `types` | Discipline database, the built-in `disciplines.vams`/`constants.vams` headers (LRM 2.4 physical constants), the type system, function registry, and parameter-range types |
| `source` / `error` | Source maps/spans and the `CompileError`/`CompileResult` types |

## Public API

```rust
use rspice_veriloga::{VerilogACompiler, CompilerOptions};

let compiler = VerilogACompiler::new(CompilerOptions::default());
let model = compiler.compile(source)?;                       // exactly one module
let model = compiler.compile_module(source, Some("nmos"))?;  // pick one of several
let model = compiler.compile_file(path)?;                    // from disk, with includes
let file  = compiler.compile_file_with_metadata(path)?;      // + include dependency list
```

`CompilerOptions` fields: `enable_ams`, `include_paths`, `defines`,
`strict_mode` (strict LRM compliance — errors on extensions), and
`integration_order` (`First` = backward Euler, `Second` = Gear-2/
trapezoidal, the default) for the `ddt`/`idt` companion models.
Multi-module foundry files are supported via `compile_module` /
`compile_file_module_with_metadata`; compiling without a module name
errors if the source declares more than one module, listing their names.
Setting `RSPICE_DEBUG_PP=1` dumps the preprocessed source next to the
input file.

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

- **Analog operators**: `ddt`, `idt`, `idtmod` (backward Euler), `ddx`,
  `limexp`, `absdelay`, `transition`, `slew`, `laplace_zp/zd/np/nd`,
  `zi_nd/zp/zd/np`, `$limit`, `$table_model`
- **Noise**: `white_noise`, `flicker_noise`, `noise_table`,
  `noise_table_log`, injected into `.noise` with amplitude scaling and
  mode gating
- **Indirect contributions**: `V(x): lhs == rhs` as constraint rows on a
  branch unknown
- **System functions**: `$temperature`, `$vt`, `$abstime`, `$simparam`,
  `$param_given`, `$port_connected`, `$mfactor` (with automatic
  multiplicity scaling), `$bound_step`, `$discontinuity`
- **Data**: 1-D array variables (compile-time and runtime indexing, with
  shadowed derivatives), runtime-bounded loops, parameters with dependent
  defaults/ranges/exclusions, localparams, `aliasparam`, attribute
  instances (`(* desc, units *)`), string parameters
- **Structure**: internal nodes, named branches, ground nets, user
  disciplines beyond electrical (thermal, mechanical, …), ANSI and
  non-ANSI port styles
- **Control flow**, lowered to guarded dataflow: `if`/`else`, `case`,
  compile-time-bounded `for`/`repeat` loops, event controls
  (`initial_step`, `cross`, `above`, `timer`)
- **Functions**: user-defined analog functions (inlined), and the built-in
  math set (`abs`, `sqrt`, `exp`, `ln`, `log`, trig/hyperbolic and their
  inverses, `floor`, `ceil`, `pow`, `hypot`, `atan2`, `min`, `max`)

**Known limitations** (clean compile errors, never silent): `noise_table`
file input (inline the pair list) and correlated noise;
parameter-dependent `zi_*` sample periods; multi-dimensional arrays; array
locals and `output`/`inout` arguments in analog functions.

## Feature flags

| Feature | Default | Effect |
| :--- | :--- | :--- |
| `native` | off | Cranelift JIT (`cranelift*` 0.115 crates); without it models run on the bytecode interpreter only |
| `ams` | off | Declared for Verilog-AMS mixed-signal support; currently gates no code in the crate |

`rspice-core` maps these as `veriloga` (interpreter) and `veriloga-native`
(JIT) and adds a blake3-keyed on-disk cache for compiled models on top.

## Building and testing

```bash
cargo build -p rspice-veriloga
cargo test  -p rspice-veriloga                      # interpreter paths
cargo test  -p rspice-veriloga --features native    # + bytecode/JIT equivalence
```

The 14 integration test files under `tests/` cover end-to-end compilation
(`compile_models.rs`), runtime evaluation and Jacobians
(`device_eval.rs`), array variables, `aliasparam`, indirect contributions,
`zi_*` filters, timestep control, `$mfactor` scaling, multi-module
selection, bytecode-vs-native equivalence (`native_equivalence.rs`), and
real production models: EKV 2.6 physics, PSP 103, and CMC model frontier
tests, plus optional BSIM4 coverage when `RSPICE_BSIM4_VA` points at an
externally supplied clean source file. Engine-level oracle tests
that compare compiled models against reference results live in
[rspice-core's test suite](../rspice-core/README.md#building-and-testing)
(`veriloga_*.rs`).

## License

RSpice Verilog-A is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).
