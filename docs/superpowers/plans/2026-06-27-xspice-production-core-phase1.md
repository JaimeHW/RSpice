# XSPICE Production Core Phase 1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first missing production XSPICE core surface to RSpice: vector model parameters, official ngspice-compatible analog aliases, and the official `pwl` and `pwlts` lookup models, with focused native and ngspice-regression coverage.

**Architecture:** Extend the netlist AST and XSPICE model-resolution path to carry typed vector parameters from `.model` cards into `CmContext`, then add models through the existing `CodeModel` trait and central registry. Keep behavior in reusable helpers under `crates/rspice-core/src/xspice/models` and avoid test-specific simulator branches.

**Tech Stack:** Rust, `rspice-core`, existing ngspice regression harness, existing XSPICE device registry, `cargo test`, local ngspice oracle decks.

---

## Non-Negotiables

- Do not create or switch branches.
- Do not modify generated device/model code.
- Do not translate GPL ngspice `src/xspice/icm/table` code.
- Only stage files touched by this plan.
- Keep wasm/browser behavior identical to desktop behavior by avoiding platform gates around XSPICE models.
- Do not special-case tests or regression paths inside solver behavior.

## Target Files

- `crates/rspice-core/src/netlist/ast.rs`
- `crates/rspice-core/src/netlist/parser/values.rs`
- `crates/rspice-core/src/netlist/xspice_parser.rs`
- `crates/rspice-core/src/simulation/circuit.rs`
- `crates/rspice-core/src/xspice/context.rs`
- `crates/rspice-core/src/xspice/instance.rs`
- `crates/rspice-core/src/xspice/traits.rs`
- `crates/rspice-core/src/xspice/models/analog.rs`
- `crates/rspice-core/src/xspice/models/lookup.rs`
- `crates/rspice-core/src/xspice/models/mod.rs`
- `crates/rspice-core/src/xspice/registry.rs`
- `crates/rspice-core/tests/xspice_analog_models.rs`
- `crates/rspice-core/tests/ngspice_regression.rs`
- `tests/ngspice/xspice/production_core/pwl_basic.cir`
- `tests/ngspice/xspice/production_core/pwlts_basic.cir`
- `tests/ngspice/xspice/production_core/official_aliases.cir`

## Task 1: Add Typed Vector Parameters To `.model` Parsing

- [ ] Add failing parser tests before implementation.

  File: `crates/rspice-core/src/netlist/parser/values.rs`

  Add tests under the existing parser test module:

  ```rust
  #[test]
  fn model_vector_params_parse_decimal_vectors() {
      let ast = parse_netlist(".model lut pwl (x_array=[-1 0 0.5 2] y_array=[0 -2 4 8])\n").unwrap();
      let model = ast.models.iter().find(|m| m.name == "lut").unwrap();
      assert_eq!(model.real_vector_params.get("x_array").unwrap().len(), 4);
      assert_eq!(model.real_vector_params.get("y_array").unwrap()[2], 4.0);
  }

  #[test]
  fn model_vector_params_store_integer_literals_as_numeric_vectors() {
      let ast = parse_netlist(".model lut d_lut (table_values=[0 1 1 0])\n").unwrap();
      let model = ast.models.iter().find(|m| m.name == "lut").unwrap();
      assert_eq!(model.real_vector_params.get("table_values").unwrap(), &[0.0, 1.0, 1.0, 0.0]);
  }
  ```

  Run:

  ```powershell
  cargo test -p rspice-core model_vector_params --lib -- --nocapture
  ```

  Expected before implementation: compilation fails because `ModelDef` has no vector parameter fields.

- [ ] Extend `ModelDef`.

  File: `crates/rspice-core/src/netlist/ast.rs`

  Add fields:

  ```rust
  pub real_vector_params: std::collections::HashMap<String, Vec<Value>>,
  pub integer_vector_params: std::collections::HashMap<String, Vec<i64>>,
  ```

  Initialize both maps anywhere `ModelDef` is constructed.

- [ ] Extend parsed model parameter storage.

  File: `crates/rspice-core/src/netlist/parser/values.rs`

  Add the same two fields to the parser's intermediate model-parameter struct. Preserve existing numeric, expression, and string parameter behavior.

- [ ] Parse bracketed vectors in `.model` cards.

  File: `crates/rspice-core/src/netlist/parser/values.rs`

  Implement these rules:

  - `[1 2 3]`, `[1,2,3]`, and `[1, 2, 3]` are equivalent.
  - Real vectors accept decimal and scientific notation values already accepted by scalar parameters.
  - The parser stores bracketed numeric vectors as `real_vector_params`; XSPICE model resolution converts vectors to integer vectors in Task 2 when `ParamSpec` says the target parameter is `IntegerVector`.
  - Empty vectors are rejected with a parser error naming the parameter.
  - Missing closing `]` is rejected with the existing parser error machinery and the source line number.

- [ ] Re-run parser tests.

  ```powershell
  cargo test -p rspice-core model_vector_params --lib -- --nocapture
  ```

  Expected after implementation: both parser tests pass.

- [ ] Commit only Task 1 files.

  ```powershell
  git add crates/rspice-core/src/netlist/ast.rs crates/rspice-core/src/netlist/parser/values.rs
  git commit -m "feat(xspice): parse vector model parameters"
  ```

## Task 2: Carry Vector Parameters Into XSPICE Contexts

- [ ] Add failing XSPICE context tests.

  File: `crates/rspice-core/tests/xspice_analog_models.rs`

  Add a minimal test-only model inside the test file:

  ```rust
  struct VectorProbe;

  impl CodeModel for VectorProbe {
      fn name(&self) -> &'static str { "vector_probe" }
      fn description(&self) -> &'static str { "test vector params" }
      fn port_spec(&self) -> Vec<PortSpec> {
          vec![PortSpec::new("out", PortType::Voltage, PortDirection::Output)]
      }
      fn parameter_spec(&self) -> Vec<ParamSpec> {
          vec![ParamSpec::real_vector("points", vec![1.0, 2.0])]
      }
      fn evaluate(&self, ctx: &CmContext) -> CmResult<Vec<CmOutput>> {
          let points = ctx.real_vector_param("points").unwrap();
          Ok(vec![CmOutput::voltage("out", points.iter().sum())])
      }
  }
  ```

  Add `xspice_model_receives_real_vector_params` with a deck containing `.model vp vector_probe (points=[1 2.5 4])`; assert output voltage is `7.5`.

  Add `xspice_model_uses_real_vector_default` with a deck containing `.model vp vector_probe ()`; assert output voltage is `3.0`.

  Add `xspice_model_converts_integer_vector_params` using a second test-only model with `ParamSpec::integer_vector("bits", vec![1, 0])`; feed `.model ip int_vector_probe (bits=[1 1 0 1])` and assert the model receives `[1, 1, 0, 1]`.

  Run:

  ```powershell
  cargo test -p rspice-core xspice_model_ --test xspice_analog_models -- --nocapture
  ```

  Expected before implementation: compilation fails because `ParamSpec::real_vector`, `ParamSpec::integer_vector`, `CmContext::real_vector_param`, and `CmContext::integer_vector_param` do not exist.

- [ ] Extend `ParamSpec`.

  File: `crates/rspice-core/src/xspice/traits.rs`

  Add explicit default storage:

  ```rust
  pub real_vector_default: Option<Vec<Value>>,
  pub integer_vector_default: Option<Vec<i64>>,
  ```

  Add constructors:

  ```rust
  pub fn real_vector(name: impl Into<String>, default: Vec<Value>) -> Self
  pub fn integer_vector(name: impl Into<String>, default: Vec<i64>) -> Self
  ```

  Ensure all existing constructors set both default fields to `None`.

- [ ] Extend resolved XSPICE model data.

  File: `crates/rspice-core/src/netlist/xspice_parser.rs`

  Add:

  ```rust
  pub real_vector_params: Vec<(String, Vec<Value>)>,
  pub integer_vector_params: Vec<(String, Vec<i64>)>,
  ```

  During model resolution, convert `ModelDef.real_vector_params` into integer vectors when the target `ParamSpec` has `ParamType::IntegerVector`. Reject non-integer entries for integer vectors with an error containing the model name, parameter name, and offending value.

- [ ] Extend context storage and accessors.

  File: `crates/rspice-core/src/xspice/context.rs`

  Add maps:

  ```rust
  real_vector_params: HashMap<String, Vec<Value>>,
  integer_vector_params: HashMap<String, Vec<i64>>,
  ```

  Add accessors:

  ```rust
  pub fn real_vector_param(&self, name: &str) -> Option<&[Value]>
  pub fn integer_vector_param(&self, name: &str) -> Option<&[i64]>
  ```

  Insert defaults from `ParamSpec` when model cards omit vector params.

- [ ] Extend instance construction without adding platform gates.

  File: `crates/rspice-core/src/xspice/instance.rs`

  Thread vector params into `CmContext`. If the existing constructor signature becomes unwieldy, introduce:

  ```rust
  #[derive(Debug, Clone, Default)]
  pub struct XspiceParams {
      pub numeric: Vec<(String, Value)>,
      pub strings: Vec<(String, String)>,
      pub real_vectors: Vec<(String, Vec<Value>)>,
      pub integer_vectors: Vec<(String, Vec<i64>)>,
  }
  ```

  Replace call sites in one commit so construction has one parameter bundle rather than four parallel vectors.

- [ ] Extend circuit build path.

  File: `crates/rspice-core/src/simulation/circuit.rs`

  Pass `ResolvedXspiceModel.real_vector_params` and `ResolvedXspiceModel.integer_vector_params` into `XspiceInstance`.

- [ ] Re-run vector context tests.

  ```powershell
  cargo test -p rspice-core xspice_model_ --test xspice_analog_models -- --nocapture
  ```

  Expected after implementation: all three vector context tests pass.

- [ ] Commit only Task 2 files.

  ```powershell
  git add crates/rspice-core/src/netlist/xspice_parser.rs crates/rspice-core/src/simulation/circuit.rs crates/rspice-core/src/xspice/context.rs crates/rspice-core/src/xspice/instance.rs crates/rspice-core/src/xspice/traits.rs crates/rspice-core/tests/xspice_analog_models.rs
  git commit -m "feat(xspice): carry vector parameters into code models"
  ```

## Task 3: Register Official Analog Model Aliases

- [ ] Add failing alias tests.

  File: `crates/rspice-core/tests/xspice_analog_models.rs`

  Add tests proving these official names work:

  - `int` behaves like existing `integrator`.
  - `d_dt` behaves like existing `differentiator`.
  - `divide` behaves like existing `divider`.

  Use tiny one-output operating-point checks with current accepted parameters. Avoid transient-only assertions here.

  Run:

  ```powershell
  cargo test -p rspice-core xspice_official_alias --test xspice_analog_models -- --nocapture
  ```

  Expected before implementation: unknown model errors for `int`, `d_dt`, and `divide`.

- [ ] Add alias model wrappers.

  File: `crates/rspice-core/src/xspice/models/analog.rs`

  Add zero-sized wrappers:

  ```rust
  pub struct IntegratorAlias;
  pub struct DifferentiatorAlias;
  pub struct DividerAlias;
  ```

  Each wrapper must delegate `description`, `port_spec`, `parameter_spec`, `init`, and `evaluate` to the existing canonical implementation and return the official name from `name()`.

- [ ] Register aliases centrally.

  File: `crates/rspice-core/src/xspice/registry.rs`

  Register:

  ```rust
  registry.register(Box::new(IntegratorAlias));
  registry.register(Box::new(DifferentiatorAlias));
  registry.register(Box::new(DividerAlias));
  ```

- [ ] Re-run alias tests.

  ```powershell
  cargo test -p rspice-core xspice_official_alias --test xspice_analog_models -- --nocapture
  ```

  Expected after implementation: all alias tests pass.

- [ ] Commit only Task 3 files.

  ```powershell
  git add crates/rspice-core/src/xspice/models/analog.rs crates/rspice-core/src/xspice/registry.rs crates/rspice-core/tests/xspice_analog_models.rs
  git commit -m "feat(xspice): register official analog aliases"
  ```

## Task 4: Implement `pwl` And `pwlts` Lookup Models

- [ ] Create local ngspice oracle decks before implementation.

  Files:

  - `tests/ngspice/xspice/production_core/pwl_basic.cir`
  - `tests/ngspice/xspice/production_core/pwlts_basic.cir`

  Use only the source decks and behavior needed for the tests. Do not copy build-system files or unrelated upstream data. Keep headers identifying source/provenance if copied from official examples.

- [ ] Run ngspice oracle for boundary behavior.

  Use the local ngspice executable configured for the existing regression suite. Capture these expected behaviors:

  - `pwl` interpolation between adjacent points.
  - `pwl` behavior below the first point.
  - `pwl` behavior above the last point.
  - `pwl` duplicate x-value handling if the official deck exercises it.
  - `pwlts` output before first timestamp.
  - `pwlts` output between timestamps.
  - `pwlts` output after final timestamp.

  Save only the resulting expected assertions in Rust tests and regression expected-output handling. Do not commit raw simulator logs.

- [ ] Add failing native tests.

  File: `crates/rspice-core/tests/xspice_analog_models.rs`

  Add `xspice_pwl_interpolates_between_points` using `x_array=[0 1 2]`, `y_array=[0 10 20]`, and a `0.25 V` input; assert `2.5 V`.

  Add `xspice_pwl_matches_ngspice_boundary_behavior` using the boundary values measured in the oracle step for input below `0 V` and above `2 V`.

  Add `xspice_pwl_rejects_mismatched_vectors` using `x_array=[0 1]` and `y_array=[0]`; assert netlist execution returns an XSPICE model error naming both arrays.

  Add `xspice_pwlts_matches_ngspice_time_lookup` using the exact timestamps and output values measured in the oracle step at `0 s`, a midpoint, and the final timestamp.

  Run:

  ```powershell
  cargo test -p rspice-core xspice_pwl --test xspice_analog_models -- --nocapture
  ```

  Expected before implementation: unknown model errors for `pwl` and `pwlts`.

- [ ] Implement lookup helpers.

  File: `crates/rspice-core/src/xspice/models/lookup.rs`

  Add:

  ```rust
  pub struct PiecewiseLinear;
  pub struct PiecewiseLinearTimeSeries;
  ```

  Shared helper contract:

  ```rust
  fn validate_table(x: &[Value], y: &[Value]) -> CmResult<()>
  fn interpolate_or_boundary(x: &[Value], y: &[Value], input: Value) -> Value
  ```

  Validation rules:

  - `x_array` and `y_array` must have equal length.
  - Length must be at least two.
  - `x_array` must be monotonic non-decreasing.
  - Non-finite values are rejected.

  `pwl` model:

  - Ports: one voltage input named `in`, one voltage output named `out`.
  - Params: `x_array` real vector default `vec![0.0, 1.0]`, `y_array` real vector default `vec![0.0, 1.0]`.
  - Evaluates output from the current input voltage using oracle-confirmed interpolation and boundary behavior.

  `pwlts` model:

  - Ports: one voltage output named `out`.
  - Params: `x_array` real vector default `vec![0.0, 1.0]`, `y_array` real vector default `vec![0.0, 1.0]`.
  - Evaluates output from `ctx.time()` using oracle-confirmed interpolation and boundary behavior.

- [ ] Wire lookup module.

  Files:

  - `crates/rspice-core/src/xspice/models/mod.rs`
  - `crates/rspice-core/src/xspice/registry.rs`

  Export and register `PiecewiseLinear` as `pwl` and `PiecewiseLinearTimeSeries` as `pwlts`.

- [ ] Re-run native lookup tests.

  ```powershell
  cargo test -p rspice-core xspice_pwl --test xspice_analog_models -- --nocapture
  ```

  Expected after implementation: all lookup tests pass.

- [ ] Commit only Task 4 files.

  ```powershell
  git add crates/rspice-core/src/xspice/models/lookup.rs crates/rspice-core/src/xspice/models/mod.rs crates/rspice-core/src/xspice/registry.rs crates/rspice-core/tests/xspice_analog_models.rs tests/ngspice/xspice/production_core/pwl_basic.cir tests/ngspice/xspice/production_core/pwlts_basic.cir
  git commit -m "feat(xspice): add piecewise lookup models"
  ```

## Task 5: Add Focused Ngspice Production-Core Regression Coverage

- [ ] Add official-alias regression deck.

  File: `tests/ngspice/xspice/production_core/official_aliases.cir`

  Cover `int`, `d_dt`, and `divide` by name. Keep assertions numeric and deterministic so they do not depend on waveform plotting.

- [ ] Register the new suite directory.

  File: `crates/rspice-core/tests/ngspice_regression.rs`

  Add `xspice/production_core` to the discoverable-suite allow list and add a focused test:

  ```rust
  #[test]
  fn test_ngspice_xspice_production_core_suite() {
      run_ngspice_suite("xspice/production_core");
  }
  ```

- [ ] Run the focused suite.

  ```powershell
  cargo test -p rspice-core test_ngspice_xspice_production_core_suite --test ngspice_regression -- --nocapture
  ```

  Expected after implementation: the new production-core decks pass.

- [ ] Run the existing digital XSPICE suite.

  ```powershell
  cargo test -p rspice-core test_ngspice_xspice_digital_suite --test ngspice_regression -- --nocapture
  ```

  Expected after implementation: the existing digital suite still passes.

- [ ] Commit only Task 5 files.

  ```powershell
  git add crates/rspice-core/tests/ngspice_regression.rs tests/ngspice/xspice/production_core/official_aliases.cir
  git commit -m "test(xspice): cover production core models"
  ```

## Task 6: Full Verification And Cleanup

- [ ] Run formatting.

  ```powershell
  cargo fmt
  ```

- [ ] Run focused native tests.

  ```powershell
  cargo test -p rspice-core --test xspice_analog_models -- --nocapture
  cargo test -p rspice-core --test xspice_digital_models -- --nocapture
  ```

  Expected: all tests in both files pass.

- [ ] Run focused ngspice suites.

  ```powershell
  cargo test -p rspice-core test_ngspice_xspice_production_core_suite --test ngspice_regression -- --nocapture
  cargo test -p rspice-core test_ngspice_xspice_digital_suite --test ngspice_regression -- --nocapture
  ```

  Expected: both suites pass.

- [ ] Run the full ngspice regression summary.

  ```powershell
  cargo test -p rspice-core test_ngspice_regression_summary --test ngspice_regression -- --nocapture
  ```

  Expected: all supported decks pass and no new expected-unsupported entries are introduced.

- [ ] Run the core crate test suite.

  ```powershell
  cargo test -p rspice-core
  ```

  Expected: all `rspice-core` tests pass.

- [ ] Run wasm-relevant checks.

  ```powershell
  cargo check --target wasm32-unknown-unknown -p rspice-ui
  ```

  Expected: XSPICE changes compile for the browser IDE build without feature loss.

- [ ] Inspect git state.

  ```powershell
  git status --short
  git diff --stat HEAD
  ```

  Expected: only intentional files from this plan are modified or committed; unrelated worktree changes remain untouched.

- [ ] Final cleanup commit if `cargo fmt` changed files not included in prior commits.

  ```powershell
  git add crates/rspice-core/src/netlist/ast.rs crates/rspice-core/src/netlist/parser/values.rs crates/rspice-core/src/netlist/xspice_parser.rs crates/rspice-core/src/simulation/circuit.rs crates/rspice-core/src/xspice/context.rs crates/rspice-core/src/xspice/instance.rs crates/rspice-core/src/xspice/traits.rs crates/rspice-core/src/xspice/models/analog.rs crates/rspice-core/src/xspice/models/lookup.rs crates/rspice-core/src/xspice/models/mod.rs crates/rspice-core/src/xspice/registry.rs crates/rspice-core/tests/xspice_analog_models.rs crates/rspice-core/tests/ngspice_regression.rs
  git commit -m "style(xspice): format production core changes"
  ```

## Completion Criteria

- `.model` vector parameters flow from parser to code-model context.
- `ParamSpec` supports real and integer vectors with defaults.
- `pwl` and `pwlts` are available through the central registry on every target.
- `int`, `d_dt`, and `divide` official aliases are registered.
- Native tests cover parser, context, aliases, lookup behavior, and validation failures.
- Ngspice regression coverage includes `xspice/production_core`.
- Existing ngspice XSPICE digital regression coverage still passes.
- Full `rspice-core` tests pass.
- Wasm check for the browser IDE build passes.
