# Verilog-A Rust Transpiler Design

## Purpose

RSpice needs a build-time Rust backend for bundled Verilog-A device models. When the simulator is compiled, the build must discover RSpice's built-in Verilog-A source files, compile them through the canonical IR, transpile each device model into Rust source, and include that generated source in the normal simulator binary. Built-in devices should not compile on first use, should not depend on the bytecode interpreter, and should not depend on Cranelift.

The first Rust transpiler design targets bundled RSpice devices. User-imported external Verilog-A files can continue to use the current runtime cache while the new backend matures, but that path is not the architecture for built-in commercial devices.

## Requirements

- The normal simulator build generates Rust source for bundled Verilog-A modules before the simulator crate is compiled.
- Discovery is directory-scan based for now. The build scans `models/veriloga` recursively in deterministic order.
- Generated code is split by device. The generator must not emit every model into one giant Rust file.
- Each module-bearing Verilog-A source is compiled through the canonical IR. The old bytecode and Cranelift paths do not shape the generated Rust ABI.
- The generated Rust is included in `rspice-core` through `OUT_DIR` and compiled into desktop, browser, mobile, and tablet builds.
- Runtime lookup resolves bundled generated models without requiring `.veriloga` directives.
- Explicit user `.veriloga` includes remain supported as an external-model path during migration.
- Unsupported Verilog-A semantics fail closed during build with file, module, phase, and source-span context.
- Hot evaluation code avoids heap allocation, dynamic string lookup, parser/compiler dependencies, and runtime code generation.
- Generated source is deterministic for the same source tree, compiler version, feature set, and canonical IR.

## Non-Goals

- This design does not implement the custom x64/ARM JIT.
- This design does not remove the existing bytecode runtime immediately.
- This design does not require a manifest file for built-in device discovery.
- This design does not introduce text dump fixtures as a validation strategy.
- This design does not redesign the solver or matrix storage.

## Architecture

The build-time flow is:

```text
models/veriloga directory scan
  -> candidate Verilog-A root files
  -> preprocessor / lexer / parser / semantic analysis
  -> canonical HIR/MIR/OptIR artifact
  -> Rust transpiler
  -> OUT_DIR/veriloga_builtins/devices/<device-folder>/*.rs
  -> OUT_DIR/veriloga_builtins/registry.rs
  -> include! from rspice-core
  -> final simulator binary
```

`rspice-veriloga` owns Verilog-A source semantics and Rust source generation. `rspice-core` owns the simulator-facing generated-device runtime contract and Cargo build integration.

### Transpiler Library

Add a Rust transpiler module to `rspice-veriloga` that accepts a validated `CanonicalIrArtifact` and produces a structured generated module:

```rust
pub struct GeneratedRustDevice {
    pub module_name: String,
    pub public_model_name: String,
    pub folder_name: String,
    pub files: Vec<GeneratedRustFile>,
    pub metadata: GeneratedRustMetadata,
}
```

The transpiler should consume MIR and OptIR, not legacy `CompiledModel` bytecode. MIR supplies the simulator equation contract. OptIR supplies scheduled, optimized expression graphs as it becomes richer. Until OptIR carries all needed schedules, the transpiler may use MIR plus existing canonical expression arenas, but the public backend contract remains canonical IR.

The generated code should be valid Rust source, not serialized bytecode in Rust syntax.

### Build Integration

Add a build script integration for `rspice-core`. The build script scans the repository-level `models/veriloga` directory when Verilog-A built-ins are enabled for the product build. The build script:

- Walks the directory recursively.
- Sorts paths by normalized canonical path.
- Treats `.va` files as candidate source roots.
- Treats `.vams`, `.include`, and other include files as dependencies, not top-level devices.
- Parses each candidate enough to determine whether it declares modules.
- Skips files with no modules.
- Compiles every declared module in a module-bearing file through canonical IR.
- Fails the build if a module-bearing file cannot compile or transpile.
- Emits `cargo:rerun-if-changed` for every scanned source/include file and every dependency returned by the preprocessor.

Directory scanning should be deterministic and boring. If two generated built-ins expose the same model name or alias, the build fails. Silent shadowing would be a commercial-quality bug.

### Generated File Layout

Generated source lives under `OUT_DIR` and is split by device folder:

```text
OUT_DIR/
  veriloga_builtins/
    registry.rs
    devices/
      ekv26_mod__ekv26__7d3a91c2/
        mod.rs
        metadata.rs
        params.rs
        state.rs
        eval.rs
        stamp.rs
      psp103__psp103__91ce44b0/
        mod.rs
        metadata.rs
        params.rs
        state.rs
        eval.rs
        stamp.rs
```

The exact internal split can vary by model complexity, but each device gets its own folder. `registry.rs` is intentionally small: it declares the generated modules with `#[path = "..."] mod ...;` and exposes a static registry. It must not contain the generated device bodies.

Folder names are deterministic and collision-resistant:

```text
<source-stem>__<module-name>__<short-source-or-ir-digest>
```

Rust identifiers are separately sanitized and must include a digest suffix when needed to avoid collisions.

### Generated Runtime ABI

`rspice-core` should define a narrow generated-device runtime module, for example `crate::device::veriloga_generated`. Generated code implements that contract.

The runtime contract should cover:

- Model metadata: name, source package, source digest, compiler version, terminal names, parameter names, aliases, ranges, units, and supported analysis domains.
- Instance construction from nodes and parameter overrides.
- Parameter default resolution and `$param_given`.
- Temperature update.
- Time and timestep update.
- Newton residual/Jacobian stamping.
- AC/reactive stamping.
- Noise source evaluation.
- Timestep bounds and discontinuity reporting.
- Accepted-timestep state advance.
- Operating-point reporting.

The registry should expose model construction through generated match dispatch, not hot-path string maps:

```rust
pub fn builtin_veriloga_model(name: &str) -> Option<BuiltinVerilogAModelDescriptor>;
pub fn instantiate_builtin_veriloga(
    model_name: &str,
    instance_name: &str,
    nodes: &[usize],
    params: &[(String, f64)],
) -> Option<BuiltinVerilogAInstance>;
```

Name lookup can normalize once at circuit construction. Device evaluation must use typed generated structs and dense indexes.

### Runtime Resolution

`Engine::build_circuit` should seed its Verilog-A model registry from generated built-ins before processing external `.veriloga` directives. A netlist can instantiate a bundled generated device by model name without an include directive.

External `.veriloga` directives remain valid. During migration, explicit external includes may register or override models in the per-circuit registry. Override behavior must be deterministic and logged clearly, because built-in model shadowing affects reproducibility.

### Generated Evaluation Strategy

Generated devices should use static layouts:

- Parameters: fixed indexes and generated setters.
- Variables and state: generated structs or fixed arrays, chosen by model shape.
- Ports, internal nodes, and branch-current unknowns: dense generated indexes.
- Expression temporaries: local variables emitted in schedule order.
- Tables and constant coefficient arrays: `static` or generated const data.

The hot path should fuse residual and Jacobian evaluation where the IR schedule permits it, so shared compact-model intermediates are computed once. The first implementation can start with a correct direct lowering, but the design target is generated residual/Jacobian code from canonical schedules, not interpreted expression trees.

Effectful analog operators must be lowered through explicit runtime support:

- `ddt`, `idt`, and `idtmod` use generated state slots and analysis-mode semantics.
- `limexp` and `$limit` use convergence-safe helper functions.
- `absdelay`, `transition`, `slew`, `last_crossing`, event functions, Laplace filters, and `zi_*` filters use typed runtime state.
- Noise functions produce large-signal zero in time-domain evaluation and explicit noise-analysis sources.
- `ddx` lowers to analytic derivative requests. Missing derivative support is a compile error.

Generated code should call small, audited helper functions for numerical primitives whose behavior must be identical across models. The generator should not paste slightly different versions of sensitive functions into every model.

## Error Handling

Build failures should point to the offending source file and module. Diagnostics should include the compiler phase: preprocessing, parsing, semantic analysis, canonical IR validation, Rust lowering, generated source writing, or generated registry construction.

Rules:

- Include-only files are skipped.
- Module-bearing files are build-critical.
- Duplicate public model names or aliases fail the build.
- Unsupported operators fail the build.
- Invalid parameter defaults, invalid ranges, invalid table data, non-finite required constants, unresolved symbols, and missing derivatives fail the build.
- Generated Rust write failures fail the build.

The build should print a concise summary of generated models so release logs can show what was compiled into the simulator.

## Validation

Validation should be behavioral and compiler-facing, not based on checked-in generated text dumps.

Required tests:

- Directory discovery tests against temporary source trees: sorted scanning, include-only skip behavior, module detection, duplicate-name rejection, and dependency tracking.
- Transpiler unit tests for name mangling, folder layout, file splitting, registry generation, and unsupported construct diagnostics.
- Generated-code compile tests for small fixture devices.
- Behavioral tests that instantiate generated devices through the built-in registry without `.veriloga`.
- Equivalence tests comparing generated devices to the current interpreter for controlled small models during migration.
- Regression tests proving external `.veriloga` includes still work.
- Target checks for native and wasm builds once the generated runtime is wired into browser/mobile feature sets.

No backend replaces existing behavior until tests prove equivalence or a reviewed intentional correction.

## Rollout

1. Add the generated-device runtime trait and registry inclusion point in `rspice-core`.
2. Add the Rust transpiler module in `rspice-veriloga` with deterministic file/folder emission.
3. Add directory discovery and build-script generation for bundled Verilog-A modules.
4. Wire generated built-ins into circuit model resolution.
5. Add generated-code compile and behavioral tests for simple devices.
6. Expand lowering coverage across stateful analog operators, noise, AC/reactive behavior, and operating-point reporting.
7. Move bundled Verilog-A devices from runtime compilation to generated Rust as coverage reaches production quality.
8. Retire bytecode/Cranelift for built-in devices after generated Rust covers the shipped device set.

## Commercial Quality Criteria

- Built-in Verilog-A devices are compiled into the simulator binary.
- Generated code is split per device folder and remains inspectable.
- Runtime execution does not require a filesystem, compiler, JIT, or source parser.
- Discovery and generation are deterministic.
- Unsupported semantics fail at build time.
- Hot paths avoid heap allocation and dynamic lookup.
- Diagnostics identify file, module, phase, and source span.
- Generated devices share one simulator runtime ABI.
- The old bytecode and Cranelift backend are migration paths only.
