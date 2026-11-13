# rspice-model-library

Portable device-model definitions and the evidence that travels with them:
model libraries and the source authority each came from, the content-pinned
source closure a run executes, process corners, the executable model
namespace of one run, project-authored model revisions, model-bound symbols,
qualification and measurement-correlation evidence, and the PDK technology
vocabulary. `rspice-design`, `rspice-formats`,
`rspice-project`, `rspice-simulation` and the project and simulation contract
crates read the same definitions, so a model's identity is decided once. It
sits below `rspice-design`, whose model-bound symbols name these definitions.
`tools/ci/check_app_crate_dependencies.py` allows it only `rspice-vocabulary`
and `rspice-results` among the application crates, and refuses egui, eframe,
winit, wgpu and rfd in its closure.

## What's in here

| Module | Contents |
| :--- | :--- |
| `model`, `types` | `DeviceModel`; `ModelType`, with the device each maps onto, and `ModelLevel` |
| `device_class` | The catalog's device class for a `.model` card (`card_device`), mirrored from the table in `tools/models/build_manifest.py`, so a project's own cards and a pack's land under the same class |
| `library`, `source_paths` | `ModelLibrary` and `ModelSourceAuthority` (built-in, external, retained import, project-owned): where a library came from and what that entitles it to. The pinned source closure (`ModelSourcePin`), its retained bytes and authenticated include edges, Model Hub pack pins, and portable source-path identities |
| `catalog`, `projection` | `ModelCatalog`, the project's library-name map, with atomic commits to project-owned definitions and `model_library_source_digest`; projection of `rspice-core`'s parsed library and subcircuit records into catalog entries and callable interfaces |
| `source_bundle` | Authentication of an in-memory set of `(portable name, bytes)` members, such as a browser upload or a Model Hub pack part, into one model library |
| `corner`, `binding`, `corner_expansion` | `ProcessCorner` and section bindings; `CornerModelBinding`, materialized cards bound to a corner; `RetainedClosure`, the one rule for whether a run can expand a corner from the retained closure |
| `resolution`, `execution` | `ModelResolutionRecord` provider decisions and `SimulationPlanModelBinding`, a plan's ordered library bindings; `ModelExecutionPlan`, the immutable, content-addressed model namespace of one nominal run, which refuses a contested name instead of falling back to the first definition |
| `authoring`, `project_revision`, `definition_metadata` | `ProjectModelDefinition`, models a project authors itself; `ProjectModelRevisionDefinition`, which admits a revision only when its source renders and reparses without losing a parameter, section or type; `ModelDefinitionMetadata`, the typed parameters, section inheritance, statistics and temperature behaviour the model editor presents |
| `technology_binding` | `ProjectTechnologyBinding` and `ProjectPdkPackagePin`: a project's model-source attachment and its exact PDK package pin |
| `pdk` | PDK archive and manifest contracts; `verify_archive_contents`; technology-revision diffs and library search paths |
| `qualification`, `correlation`, `validation` | Model qualification and release evidence (golden vectors, desktop and WebAssembly parity, fail-closed promotion); versioned measurement-correlation datasets, alignment, metrics and reviewer evidence; `ModelValidationReceipt` and its integrity digest |
| `symbol` | Model-bound symbol definitions, parameter forms and retained graphics, stored under the `rspice.symbol.*` metadata keys |
| `facts` | `ClosureFacts`: facts about a model closure that more than one surface reports, derived in one place |

## Boundaries

- SPICE and Spectre model-library parsing belongs to
  [`rspice-core`](../rspice-core); this crate projects its parsed records.
- PDK installation and executable source compilation
  belong to [`rspice-simulation`](../rspice-simulation) `pdk`.
- Materializing model-bound symbols into design libraries belongs to
  [`rspice-design`](../rspice-design) `model_bound_symbol`.
- The Model Hub client and the Models workspace belong to
  [`rspice-app`](../rspice-app).

## Entry points

- `ModelCatalog::add_library`, `get_library` and `libraries_sorted` over
  `ModelLibrary` values.
- `source_bundle::prepare(display_name, root_member, files, section, limits)`
  to capture an in-memory source bundle.
- `ModelExecutionPlan::try_new(process, selected_library_corners,
  materialized, records)` for one run's model namespace.
- `model_library_source_digest(&library)` for a library's source identity.
- `pdk::package::verify_archive_contents(&archive)` for a PDK archive.

## PDK archive

A publisher ships one JSON `PdkTechnologyArchive`: `schema_version`
(`PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION`), `manifest_base64`,
and `files`, each a `path` and `content_base64`. In `verify_archive_contents`,
every artifact the manifest declares must be present with its declared size and
SHA-256, paths are unique without regard to case, and an undeclared file is
refused. Manifest schemas from `MINIMUM_PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION`
to `PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION` are accepted; size and count bounds
are the `MAX_PDK_*` constants in `pdk::contracts`, `MAX_PDK_ARCHIVE_BYTES`
among them.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `catalog-test-observation` | Exports `CATALOG_LIBRARY_SERIALIZATIONS`, a per-thread count of the whole-library serializations behind `model_library_source_digest`, for cost regression tests. Enabled by `rspice-app`'s dev-dependency |

## Testing

```bash
cargo test -p rspice-model-library
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
