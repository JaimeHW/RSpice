# rspice-simulation

The simulation execution services of the RSpice application: schematic
netlisting, exact source and model preparation, immutable run snapshots with
one-use dispatch authorization, the engine adapters for each analysis family,
run scheduling with progress, cancellation and live transient delivery, and
conversion of engine completions into retained evidence. It also validates
signed PDKs before a caller publishes an installation, and compiles project
Verilog-A sources. Controllers in `rspice-app` dispatch sealed requests into
it; the browser runs the same code in the `rspice-worker` module worker, and
the CLI's `rspice study` and the mixed-signal conformance corpus drive its
headless preparation path. `tools/ci/check_app_crate_dependencies.py`
refuses it any path to `rspice-project`, so it reads project records through
`rspice-project-contract` and `rspice-simulation-contract`; the check allows
it only those two, `rspice-app-types`, `rspice-design`,
`rspice-model-library`, `rspice-results` and `rspice-formats` among the
application crates, and refuses egui, eframe, winit, wgpu and rfd in its
closure.

## What's in here

| Module | Contents |
| :--- | :--- |
| `netlist_gen` | Schematic-to-SPICE netlisting over `rspice-design` connectivity and hierarchy: placed project cells become `.SUBCKT` definitions and the deck is self-contained. `NetlistResult` carries the deck, its nets, diagnostics and the emitted hierarchy mapping |
| `preparation`, `analysis_preparation`, `netlist_preparation`, `netlist_sources`, `sealed_source`, `prepared_dependency`, `measurement_references` | Preparation inputs and typed failures; analysis lowering over borrowed domain inputs; exact source composition with process and supply corners; filesystem-free materialization of retained netlist dependencies; the one encoding a run's executable source is sealed under; producer requirements between frozen requests; captured comparison tables |
| `execution`, `execution_identity`, `execution_artifact`, `execution_options`, `output_contract`, `capture_ledger` | `PreparedRunSnapshot`, an immutable resolution of every execution input, and `PreparedRunAuthorization`, its generation-bound permit consumed immediately before dispatch; headless task-graph preparation; the canonical execution-identity encoding; authenticated artifacts handed between prepared tasks; compiled saved-output contracts; what a plan's retained evidence costs |
| `runner`, `status`, `progress`, `error`, `engine_log`, `live_transient` | `SimulationRunner`, which runs one prepared task at a time on a background thread natively or in the module worker in the browser, with progress, cooperative abort, bounded engine-log capture and lossless, ordered delivery of accepted transient samples |
| `engine_bridge` and `engine_services` (private), `periodic`, `sweeps`, `point_family`, `study`, `optimization`, `soa_duration` | The engine adapters: parse the sealed source, resolve each analysis's configuration, call `rspice-core`, and translate its results and errors. Periodic configuration and lowering; sweep points and the families PVT declarations produce; frozen study requests; bounded optimization search over a caller's evaluator; SOA duration qualification |
| `results`, `result_conversion` | `SimulationResult` and its projections; conversion of an engine completion into exact retained `rspice-results` evidence |
| `transient_checkpoint`, `monte_carlo_checkpoint` | Authenticated transient continuation and Monte Carlo checkpoint requests, with bounded delivery of the latest snapshot |
| `manual_deck`, `study_document` | Running a hand-written deck with the analyses it declares; `StudyDocument`, the versioned JSON study the CLI reads |
| `model_sources`, `model_import`, `compilation`, `veriloga`, `project_veriloga` | Authentication and materialization of model-source snapshots; compiler preparation for model imports; the compilation policy shared by project, model-library and signed-PDK sources; sealed Verilog-A/AMS artifacts and their engine registration; project Verilog-A compilation with typed failures |
| `pdk`, `project_technology` | Signed-PDK import, runtime validation, executable source sealing and bounded callback execution (`PdkTechnologyRegistry`); project technology validation against the descriptor and model catalog |

## Boundaries

- Analysis kinds, drafts, plans and specifications belong to
  [`rspice-simulation-contract`](../rspice-simulation-contract).
- The numerical engine, devices and analyses belong to
  [`rspice-core`](../rspice-core); the Verilog-A compiler to
  [`rspice-veriloga`](../rspice-veriloga).
- Connectivity extraction and hierarchy resolution belong to
  [`rspice-design`](../rspice-design); netlisting reads that extraction and
  traces nothing of its own.
- Retained evidence types belong to [`rspice-results`](../rspice-results);
  persisting runs belongs to [`rspice-project`](../rspice-project) and
  [`rspice-formats`](../rspice-formats).
- Run controllers and Simulate forms belong to [`rspice-app`](../rspice-app);
  the browser worker binary to [`rspice-worker`](../rspice-worker).

## Entry points

- `netlist_gen::generate_netlist_hierarchical_with_variables` for a deck from
  a schematic, its `HierarchySource` and the plan's design variables.
- `execution::prepare_headless_run(HeadlessRunInput::new(source, origin,
  tasks), &abort)` for a `PreparedRunSnapshot` from literal source.
- `execution::PreparedRunAuthorization::retain(snapshot)`, then
  `consume(intent, rebuild)` for the one `AuthorizedRunDispatch`.
- `AuthorizedTaskDispatch::resolve_dependency_artifacts(&artifacts)`, then
  `runner::SimulationRunner::start_prepared(dispatch, stream_samples)` and
  `poll_result`.
- `result_conversion::convert(result, analysis_type, label, now)` for an
  `AnalysisResult`.
- `study_document::StudyDocument::from_json(source, byte_limit, &abort)`.

## Study document

`study_document::StudyDocument` is the JSON study `rspice study check`, `plan`
and `run` read: `schema_version` (`STUDY_SCHEMA_VERSION`), an `id` that
namespaces task identities, a `circuit` path resolved against the study file
with ordered include search paths, and `tasks`, each a named `AnalysisSpec`
with its dependencies and numeric overrides. `from_json` takes an explicit
byte limit and refuses unknown fields, even inside the shared analysis
specifications; decoding alone authorizes no run. The user-facing description
is in the [CLI README](../rspice-cli/README.md).

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `generated-veriloga-catalog` | Forwards to `rspice-design/generated-veriloga-catalog`, so the generated Verilog-A models can be placed, bound and netlisted. Forwarded from the flags of the same name on `rspice-app` and `rspice-worker` |
| `wasm-jit` | Enables `rspice-veriloga/wasm-jit`, so project Verilog-A compilation can return a verified WebAssembly JIT module (`WasmJitWorkerArtifact`) to the browser worker |
| `browser-worker` | Implies `wasm-jit` and enables `rspice-core/veriloga-wasm-jit`. On `wasm32` it builds the worker-side entry points `rspice-worker` dispatches, `runner::run_worker_request_value` and the Verilog-A JIT request functions. Set by `rspice-worker` |
| `pdk-test-fixtures` | Builds `pdk::test_fixtures`, signed-PDK archives and trust stores for dependent tests, pulling in `ed25519-dalek` and `wat`. Enabled by `rspice-app`'s dev-dependency |

## Testing

```bash
cargo test -p rspice-simulation

# CI runs the callback host code-generation regression on its own
cargo test -p rspice-simulation --test wasmi_callback_codegen -- --test-threads=1
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
