# rspice-simulation-contract

The authored simulation vocabulary of the RSpice application. `AnalysisKind`
is the application-level list of analyses; around it sit the persisted setup
document and analysis plan, the per-analysis drafts, typed configurations and
portable `AnalysisSpec` specifications with their validation and SPICE card
generation, run sets, design variables, saved outputs, the numerical
dependency contracts between analyses, and the browser-worker wire types.
`rspice-project` persists these values and `rspice-simulation` prepares and
executes them; `tools/ci/check_app_crate_dependencies.py` does not allow
`rspice-project` to depend on `rspice-simulation`, so the vocabulary is its own
crate below both. The check allows it only `rspice-app-types`,
`rspice-results` and `rspice-model-library` among the application crates, and
refuses egui, eframe, winit, wgpu and rfd in its closure. It takes
`rspice-core` without default features, for the engine's netlist and solver
vocabulary.

## What's in here

| Module | Contents |
| :--- | :--- |
| `analysis_kind` | `AnalysisKind`: `ALL` and `MANIFEST_ORDER`, the stable serialized IDs (`stable_id`, `from_stable_id`), labels, categories, availability and prerequisites |
| `setup_document`, `setup_state`, `legacy_plan_migration` | `SimulationSetupDocument`, the plan state a project saves, and `SimulationSetup`, which restores it together with the compatibility projections older consumers read; deterministic migration of schema-3 singleton plans |
| `plan_model`, `plan_catalog`, `plan_dependency`, `plan_payload`, `plan_diagnostics`, `analysis_lifecycle`, `analysis_run_at` | `SimulationPlan`, the revisioned plan of analysis instances with typed dependencies, lifecycle receipts and per-instance run-space participation; named plans and clone lineage; per-plan payloads; what a plan refuses and reports |
| `analysis_draft`, `drafts` and the `*_draft` modules | `AnalysisDraft`, the persisted draft of one analysis instance, over the draft modules behind it (`drafts` for AC, DC, transient, noise, distortion, FFT, DC mismatch and transient noise, then `op_draft`, `pss_draft`, `hb_draft`, `mc_draft`, `soa_draft` and the rest) |
| `analysis_spec`, `analysis_spec_values`, `analysis_run_type`, `config` | `AnalysisSpec`, the portable typed specification of one analysis in a multi-run job, and the validation that refuses one the engine cannot execute; `AnalysisConfig`, typed fields, validation and card generation for the classic SPICE analyses |
| `options`, `numeric_override`, `accuracy` | Authored solver policy, per-analysis departures from it, and the one meaning of the accuracy tier |
| `periodic_carrier`, `quasi_periodic_controls`, `envelope_initialization`, `envelope_multirate`, `envelope_policy`, `hbnoise_policy`, `fourier_output` | Policy shared beyond one draft: the large-signal solve a small-signal periodic run linearizes around, quasi-periodic request controls, envelope initialization, multirate execution and extraction choices, HB-noise reference admission, and the Fourier output spelling |
| `run_set`, `corner_config`, `design_variable`, `design_variable_quantity` | Run-set dimensions, values, budgets and composition; corner configuration; plan design variables with exact unit validation |
| `saved_output`, `capture_group`, `output_policy`, `regression_policy`, `study_measurement` | Saved-output records and capture groups, output selection and retention policy, regression tolerances, and study-measurement selectors |
| `dependency_contract` | Numerical capability contracts between dependent analyses, one set of predicates for the plan editor and the execution boundary |
| `soa_rule`, `soa_observation`, `optimization_expression`, `optimization_search`, `mc_statistics`, `mc_checkpoint`, `tran_checkpoint` | Safe-operating-area limits and observation windows; optimization expressions and search domains; per-parameter Monte Carlo statistics; checkpoint retention and continuation images |
| `worker_*`, `progress`, `convergence_transport`, `resource_failure` | The browser-worker wire types (protocol identity and execution environment, run configuration, specification, waveforms, events, measurements, noise, SOA, recorded FFT spectra, result values, numeric-series transport, errors); the progress vocabulary; compact convergence metadata; resource failures with device and allocation context |

## Boundaries

- Netlist generation, source preparation, dispatch and the engine adapters
  belong to [`rspice-simulation`](../rspice-simulation).
- Retained results, evidence and specifications belong to
  [`rspice-results`](../rspice-results); this crate re-exports its
  `spice_value` parser.
- The Simulate workspace forms that edit these drafts belong to
  [`rspice-app`](../rspice-app).

## Entry points

- `analysis_kind::AnalysisKind::ALL` for every analysis kind, and
  `AnalysisKind::from_stable_id` for a persisted ID.
- `setup_state::SimulationSetup::new()` for a project's setup.
- `plan_model::SimulationPlan::insert(kind)`, then `edit(id, |draft| ...)`;
  `freeze` for the `FrozenSimulationPlan` execution reads.
- `analysis_spec::AnalysisSpec` for one analysis, and `config::AnalysisConfig`
  with `validate` and `to_spice`.

## Worker wire contract

The application and the separately built `rspice-worker` image exchange
requests and results as the `worker_*` types. Each side refuses a peer whose
protocol differs from `worker_protocol::WORKER_REQUEST_TRANSPORT_PROTOCOL` or
`WORKER_RESPONSE_TRANSPORT_PROTOCOL`, whose doc comments say what each recent
revision changed. Numeric series cross inline or as indexed transfer buffers
(`WorkerF64Series`), bounded by `worker_transport::MAX_WORKER_F64_VALUES` and
`MAX_WORKER_TRANSFER_BUFFERS`.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `derivation-test-observation` | Builds `cost_probe`, per-thread counts of the plan derivations a Simulation Studio frame may compute only once, for cost regression tests. Enabled by `rspice-app`'s dev-dependency |

## Testing

```bash
cargo test -p rspice-simulation-contract
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
