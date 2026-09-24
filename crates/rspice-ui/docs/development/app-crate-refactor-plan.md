**RSpice application refactor — implementation plan**

Prepared 2026-09-23 against `5d02b0e8b5a1811a46ca7c8f08a8e0113aea8a5e`; execution starts from `43d4d30c97afe83fa987e96ea25ef38e6f7bf743`. Status: **in progress; R00 inventory and R01 boundary repair**. This plan follows the local architecture review, `crates/rspice-ui/docs/development/app-crate-architecture.md`.

The outcome is a renamed `rspice-app` that composes independently testable design, project, simulation, results, rendering, and platform services. The browser worker must build independently of the application and GUI stack. Supported behavior, numerical meaning, project data, and transactional guarantees must survive the refactor.

**User direction incorporated:** remove legacy code that is no longer used. Confirmed-unused implementations, adapters, aliases, dependencies, fixtures, and compatibility scaffolding should be deleted as their owners are migrated. Retention must have a concrete consumer or supported compatibility purpose; age or an old comment is not a reason to keep code. A saved-file migration with supported input is still a consumer.

All paths beginning with `src/` below refer to the application package: currently `crates/rspice-ui`, and `crates/rspice-app` after R02. Proposed new package paths are under `crates/`. Work-package IDs R00–R18 are stable progress identifiers. Every package below starts **not started**; a checkbox is checked only after its exit conditions are demonstrated on the integration commit.

**Implementation decisions**

| Decision | Implementation |
| --- | --- |
| Package identity | Rename package, directory, and Rust library to `rspice-app`, `crates/rspice-app`, and `rspice_app`. Rename the main Cargo binary to `rspice-app`. |
| Browser delivery compatibility | Keep the existing JS exports, DOM IDs, persisted keys, protocol identifiers, and delivered `rspice-ui*.js`/Wasm output stems during this refactor. Cargo artifact names and public delivery filenames are different contracts. Change build inputs to the new binary paths and retain explicit wasm-bindgen `--out-name` values. |
| Worker transition | Retain the current worker binary temporarily in the renamed app package. At R13 replace it with the `rspice-worker` package/binary, update build inputs, and delete the old binary and worker implementation exports from the app. |
| Data compatibility | Package relocation does not change project/session schemas, UUID namespaces, digest domains, field ordering used for authentication, or numerical serialization. Explicitly preserve required legacy readers; delete readers only when their input is outside the established support inventory and has no runtime use. |
| Source compatibility | No permanent `rspice-ui` compatibility crate. Temporary internal re-exports may forward to one owner during a transition and have a named removal step. Required re-exports of canonical existing types are not obsolete aliases. |
| Versioning | New libraries inherit workspace version, toolchain, license, lints, and `publish = false`. Do not promise a public SDK or independent release cadence. |
| Existing architecture | Preserve `rspice-core`, matrix, Verilog-A/runtime/generated models, cloud, pack trust, output, publication, viewer, and automation boundaries. Their existing consumers remain operational. |
| Platforms | Preserve existing desktop/browser behavior and portable mobile engine compilation. Native mobile hosts and new browser support are separate product work, not implied by moving Rust crates. |
| Scope control | Relocation, ownership repair, and unused-code removal are in scope. Numerical algorithm changes, new formats, engine replacement, UI redesign, and dependency upgrades are separate changes. |

The implementation adds one refinement to the review: **separate `rspice-hardcopy-contract` from `rspice-hardcopy`**. Design/project documents persist print settings, while hardcopy rendering consumes design/results. A single crate depending in both directions would create a cycle or bring rendering into the persisted model. The existing `src/hardcopy` versus `workbench/hardcopy_adapters` separation already supplies this boundary.

**Target dependency rules**

This table describes permitted application-layer dependencies, not a requirement to add every listed edge. Existing engine/trust/publication dependencies must be recorded explicitly in R00/R03. No new library may depend on `rspice-app`, a presentation crate, or an executable package unless its row explicitly allows the presentation dependency. These rules cover normal and build dependencies; tests spanning layers belong in the upper integration harness.

| Package | Permitted lower application packages | Owns |
| --- | --- | --- |
| `rspice-app-types` | None | Shared IDs/revisions, portable quantity policy, common source/hierarchy reference vocabulary actually needed by multiple owners |
| `rspice-results` | `app-types` | Exact datasets, result evidence, derived mathematics, visualization/report documents |
| `rspice-model-library` | `app-types`, `results` where correlation evidence requires it | Catalog/model/PDK data, retained source closures, trust/validation, import preparation |
| `rspice-simulation-contract` | `app-types`, `results`, `model-library` for portable binding types only | Plans, saved outputs/specifications, authored configuration, validated request/message vocabulary |
| `rspice-hardcopy-contract` | `app-types`, `results` where authenticated report references require it | Persisted print contracts, source identities, pagination and geometry |
| `rspice-design` | `app-types`, `model-library`, `hardcopy-contract` | Design documents, library hierarchy, connectivity, edit/reference transactions |
| `rspice-project` | `app-types`, `design`, `model-library`, `simulation-contract`, `results`, `hardcopy-contract` | Project aggregate, document registry, migration, snapshots, save transaction semantics |
| `rspice-formats` | `app-types`, `results` | Engineering-data codecs and bounded stream/byte conversion |
| `rspice-simulation` | `app-types`, `design`, `model-library`, `simulation-contract`, `results`; `formats` only for an actual output codec | Preparation, source compilation, engine adapters, scheduling/execution, live events, checkpoints |
| `rspice-hardcopy` | `app-types`, `hardcopy-contract`, `design`, `results` | Frozen-source adapters, semantic scene construction, deterministic rendering |
| `rspice-ui-kit` | `app-types`; `results` only for shared headless plot sampling contracts/functions | Egui tokens, widgets, plot painting and disposable display caches |
| `rspice-schematic-editor` | `app-types`, `design`, `model-library`, `ui-kit`; narrow `simulation-contract` types when required by property controls | Schematic/symbol interaction, editor sessions, property presentation |
| `rspice-results-ui` | `app-types`, `results`, `ui-kit` | Result viewers, Visualization Studio, report editing and local interaction |
| `rspice-worker` | `simulation`, `simulation-contract`, `model-library`, `hardcopy`, `formats`, and their lower dependencies | Worker host, JS transfer glue and task dispatch |
| `rspice-app` | The libraries above and existing platform/cloud/automation services | Composition, navigation, cross-feature coordinators, platform adapters and session integration |

Names in dependency cells omit the `rspice-` prefix for readability. `app-types` may reuse `rspice-design-model` primitives; that existing crate must never acquire an upward dependency. In particular, `ContentDigest` and `ConfigurationSetId` retain their existing canonical owners.

Neither results nor model-library may depend on simulation-contract or the execution implementation. Result evidence needed by plans is defined below those plans. Design does not own a full `ProjectWorkspace` containing plans/results; the aggregate belongs to project. Runtime compiler execution moves out of model-library into simulation, with typed validation inputs/outcomes at the boundary.

Existing references to `rspice_core::Value`, result payloads, parser helpers, and device descriptors do not justify duplicating core types or algorithms. A documented, minimally featured `rspice-core` dependency is acceptable where those contracts currently live. It is **not** acceptable to introduce a dependency on the extracted simulation runtime. Record core/compiler feature closure separately; do not claim that a headless library is engine-free merely because it has no GUI. Any additional core-type extraction requires a separate bounded proposal.

The UI kit may share headless sampling with results, avoiding a second interpolation implementation. If that edge materially broadens its resolved dependency closure, provide exact readouts through the plot caller instead; measure that tradeoff in R14. Either design is acyclic: results must never import the UI kit.

**Permanent invariants**

- One authoritative owner per document, source bundle, plan, dataset, identity, and edit/dispatch permit. UI projections and caches never become alternate authorities.
- Invalid or stale mutations leave live state unchanged. Preserve document/run epochs, revision checks, source authentication, and single-use authorization.
- Preserve authoritative f64/complex128 values, digital/event payloads, units, provenance, deterministic ordering, and current precision policy. Display simplification never feeds measurements, exports, or saved data.
- Project Save, Save all, recovery, expected-content checks, native writer leases, browser binding generations, and publication uncertainty retain their distinct meanings.
- Cancellation remains bound to the exact execution identity. Old tasks, old workers, and late callbacks cannot overwrite or cancel replacement work.
- Keep validation with the type or operation that enforces it. Moving a private type across crates is not permission to expose unchecked field construction or serializable execution permits.
- Keep hot paths on typed calls, borrowed views, and shared immutable buffers. Do not introduce whole-project copying, per-sample JSON messages, or a universal event bus.
- No new advertised feature, deleted supported feature, or relaxed numerical tolerance is hidden in a mechanical extraction.

**Ordered work packages**

The prerequisites determine ordering. Each substantial package is implemented as several reviewable changes: first narrow ownership/interfaces in place, then move the code, then switch consumers and delete superseded paths. Those subchanges must retain a buildable application; a migration package is not complete while its temporary duplicate route remains.

| ID | Work | Prerequisites | Completion |
| --- | --- | --- | --- |
| R00 | Pin baseline, inventories and evidence | None | [ ] |
| R01 | Restore architecture guards and prepare internal boundaries | R00 | [ ] |
| R02 | Rename application package and main binary | R01 | [ ] |
| R03 | Extract shared application value types | R02 | [ ] |
| R04 | Extract results and exact result mathematics | R03 | [ ] |
| R05 | Extract model/PDK library and import service | R04 | [ ] |
| R06 | Extract simulation plans and message contracts | R04, R05 | [ ] |
| R07 | Extract hardcopy persistence contracts | R04 | [ ] |
| R08 | Extract design documents and edit transactions | R03, R05, R07 | [ ] |
| R09 | Extract project aggregate, lifecycle and persistence | R06, R08 | [ ] |
| R10 | Extract engineering-data codecs | R04 | [ ] |
| R11 | Extract simulation preparation and execution | R06, R08, R09, R10 | [ ] |
| R12 | Extract hardcopy scene/rendering services | R07, R08, R09 | [ ] |
| R13 | Separate browser worker package | R05, R11, R12 | [ ] |
| R14 | Extract egui UI kit | R03, R04 | [ ] |
| R15 | Extract schematic/symbol editor | R08, R09, R14 | [ ] |
| R16 | Extract results/report presentation | R04, R09, R14 | [ ] |
| R17 | Finish app composition and legacy retirement | R13, R15, R16 | [ ] |
| R18 | Integrated qualification and architecture closeout | R17 | [ ] |

Recommended execution order is **R00 → R01 → R02 → R03 → R04 → R05 → R06 → R07 → R08 → R09 → R10 → R11 → R12 → R13 → R14 → R15 → R16 → R17 → R18**. R10 and R14 can be moved earlier once their prerequisites pass. Avoid concurrent edits to manifests, crate roots, the dependency policy, or shared state while these boundaries are moving.

**R00 — Establish a reproducible baseline**

Scope: workspace manifests/lockfile, application source, `tests/module_layering.rs`, `tests/hierarchy_guards.rs`, existing CI/tooling, release/build scripts, and supported file/worker schemas.

- [ ] Record source commit and relevant tree hashes, dirty files, lockfile digest, Rust/tool versions, host/target, features, build profile and environment. Implementation should use an isolated `codex/` worktree so unrelated work cannot silently change evidence.
- [ ] Collect direct and resolved normal/build dependency graphs for native app, browser app, and worker independently. Classify the existing 866 whole-app references by actual consumer rather than treating the textual count as a semantic dependency graph.
- [ ] Inventory schema versions, Serde defaults/aliases/custom readers, digest domains, generated catalogs, JS exports, buffer protocols, runtime asset paths, application data keys, and installer inputs.
- [ ] Inventory existing unit/integration tests, selected feature-only tests, ignores, browser harnesses, and fixtures. Record retained coverage by scenario, not only test counts.
- [ ] Run the existing application suites and record pre-existing failures. The review's previous 6-pass/5-fail architecture result is historical evidence, not a substitute for a current baseline.
- [ ] Record clean and incremental build/test-link timings, browser raw/gzip artifact sizes, startup, representative interaction latency, and memory using reproducible fixtures. Keep host-dependent timings as comparisons, consistent with `.github/CI.md`; do not invent universal limits.
- [ ] Start the legacy-removal register described below. Inspect cfg/feature consumers and serialized-data consumers before calling a path unused.

Deliver evidence under an ignored `target/app-refactor/<commit>/<target>/` directory or the existing CI artifact mechanism. Put lasting dependency policy and verification code under tracked `tools/ci/`; `/design/` is currently ignored, so this planning document is local and must not be the only place future CI rules exist.

Exit: a reproducible baseline and complete work inventory exist. Record unrelated failures with owners; repair refactor-relevant failures in R01 rather than waiting indefinitely for unrelated solver work.

**R01 — Restore guards through ownership repair**

The reviewed violations are `io → simulation` 15/13, `services → simulation` 32/8, `simulation → workbench` 44/28, `state → io` 7/5, `state → services` 46/9, and `state → simulation` 19/9. Re-measure before editing. Other known failures are 91 lint suppressions versus 54, three nested public-module declarations versus zero, missing module headers, and oversized source files.

Progress at `ea4e5a697`: the three nested public modules are crate-private; all 27 missing module headers have descriptions; the editor-owned source-digest wrapper is removed in favor of the canonical state function; and SOA evidence/evaluation now lives under `results`, with worker-wire assertions in the simulation tests. The guard improved from 6 passed/5 failed to 8 passed/3 failed. Remaining excess edges are `io → simulation` 15/13, `services → simulation` 28/8, `simulation → workbench` 31/28, `state → io` 7/5, and `state → simulation` 19/9. Lint suppressions remain 91/54 and the oversized-file guard still fails. Native app/test checks, the browser app check, and the browser-worker check pass; focused SOA duration, derating, and worker JSON tests pass. R00 and R01 remain open.

Progress at `9c6d4b214`: the canonical quantity parser and checkpoint persistence test are now in lower/upper appropriate owners, respectively, and three shipped-example netlist checks live with the workbench fixture. The guard no longer reports `state → io`, `state → services`, or `simulation → workbench` overages. Remaining excess edges are `io → simulation` 15/13, `services → simulation` 28/8, and `state → simulation` 18/9; lint suppressions and oversized files still fail. A pinned-base UI library run and a refactor run both have the same 22 failing test names, with the projection ratchet skipped after an earlier run exceeded 12 minutes. See `app-crate-refactor-baseline.md` for the exact comparison. R00 and R01 remain open.

- [ ] Classify each excess reference as production logic, shared data, misplaced pure function, test-only integration, or stale source inspection. Move cross-owner integration tests upward; keep their assertions and meaningful execution coverage.
- [ ] Move pure safety/yield/result evidence and checkpoint records below their executors; split execution from the methods validating retained evidence. Move model corner vocabulary below both catalog and runner.
- [ ] Move source-content identity helpers out of the code editor. Remove the calculator's dependency on a controller-owned numeric parser while continuing to use the canonical SPICE suffix semantics.
- [ ] Separate durable plan/catalog fields from workbench buffers, and introduce a narrow preparation input instead of passing full application state to lower-level helpers.
- [ ] Move host storage calls out of persisted PDK state. The adapter can remain an application module until the consuming project/model interface is extracted.
- [ ] Delete confirmed-unused items and stale suppressions; repair real lints. Restrict the three nested public modules unless a demonstrated consumer needs the exposure. Do not remove functioning analyses or useful tests merely to satisfy a count.
- [ ] Split newly oversized files at actual owners, retaining source-inspection tests and their production/test partition rules. Add module descriptions that state each new owner's responsibility.
- [ ] Restore all existing architectural checks. Existing approved violations may remain temporarily with a named R04–R16 removal owner; new growth may not be hidden by increasing ceilings.

Exit: the existing guard suite passes, behavior checks for the moved code pass, and remaining old dependencies have concrete retirement steps. Where the guard itself misclassifies an edge, fix its semantics with evidence and regression coverage; do not disable it or rename imports merely to evade it. R01 internal modules are staging locations, not permanent extra abstraction layers.

**R02 — Mechanical rename**

- [ ] Rename `crates/rspice-ui` to `crates/rspice-app`; update workspace members/default-members, manifest package/main-binary names, lockfile package identity, and Rust `rspice_ui` references in entrypoints/tests.
- [ ] Update native packaging, CI/coverage/nightly/native-release workflows, tool defaults and source-path assertions, README commands, asset includes, fixture discovery, and macOS entitlement paths. Keep the visible product name RSpice and stable application data locations.
- [ ] Update `.gitignore`'s exception for `web/python/pyodide-*/python_stdlib.zip`. Verify the pinned Python archive remains present in a fresh checkout/package after the directory move.
- [ ] Update browser build input artifact paths while keeping the delivery filenames and JS exports fixed. During this stage the worker still builds from `-p rspice-app --bin rspice-ui-worker --features browser-worker`.
- [ ] Update `tools/ci/test_ui_build_identity.py`, `test_ci_configuration.py`, `test_ide_worker.py`, `test_automation_runtime_assets.py`, `test_qualify_drawing_sheet.py`, browser/loader tests, and `tools/release/test_package_native.py` where they name the old path. Audit `tools/models/build_manifest.py` too.
- [ ] Verify generated catalog includes, Windows icon resources, linked-worktree build identity, packaged worker/Python URLs and native launch from the staged bundle.

Exit: native app and both Wasm images build through the renamed package, tooling/packaging tests pass, and a search finds old Rust package/path names only in an explicit historical-or-delivery-compatibility inventory. No permanent package alias or second implementation is introduced.

**R03 — Shared value vocabulary and package policy**

Sources: `product`, `quantity`, `source_revision`, and the independently reusable portions of `state/hierarchy_path`, document/source references, and diagnostic anchors.

- [ ] Extract IDs, revisions, portable source/hierarchy references, and pure quantity parsing/formatting policy to `rspice-app-types`. Keep ownership-specific models with their future owner.
- [ ] Preserve `ContentDigest`, `ConfigurationSetId`, geometry primitives and any other already shared types from `rspice-design-model` by reusing their exact implementations. Preserve UUID namespaces, normalization and path grammar.
- [ ] Keep Windows/JS locale discovery, entropy acquisition details, and host integration outside pure value modules. Select target-specific dependencies explicitly where generating identities requires them.
- [ ] Keep UI command routing with the app; only command identifiers genuinely crossing service boundaries belong in shared types. Do not move `AppState`, an execution context, or service locators here.
- [ ] Add a checked-in allowlist such as `tools/ci/app_crate_dependencies.json` and a checker over Cargo metadata. Enforce direct edges and forbidden transitive dependencies by target/feature selection. Keep module-level checks for remaining monolithic regions.
- [ ] Centralize dependencies shared by the new packages in workspace dependency declarations without upgrading versions. Preserve explicit `default-features` and target selections, `serde_json/float_roundtrip`, build-only catalog ordering, and native window/backend feature activation. Do not rely on another workspace member accidentally enabling a required feature.
- [ ] Rehome clock/diagnostic helpers into their consuming lower owner or a small value module when they are genuinely shared; keep egui repaint hooks and application log presentation at the boundary. Avoid a miscellaneous utility crate.

Exit: values compile/test independently on native and wasm32; the dependency checker catches an intentionally invalid graph in its own tests; app integration passes. New Cargo packages are registered in lint/test selection as they are added.

**R04 — Results, mathematics and result evidence**

Sources: `results/*`, `analysis/*`, `state/result_presentation`, selected `state/simulation/*`, pure safety/yield evidence, `ui/plot/sample.rs` and exact interpolation currently in `ui/plot/decimate.rs`.

- [ ] Extract authoritative waveform/dataset storage, typed evidence, provenance/digests, history, visualization/report documents and exact queries into `rspice-results`.
- [ ] Divide `SimulationState`: retained result state moves here; active run handles/progress/cancellation move toward execution; selected viewer, trigger flags and display state remain with app/presentation. Avoid copying the entire struct into a library.
- [ ] Move calculation, sampling, calculator grammar/evaluation and derived-data computation to results. Resolve the calculator-to-controller reference using the existing canonical parser semantics, not a copied suffix table.
- [ ] Move result-owned safety/yield/optimization observation records and their pure invariants below simulation. Separate authored execution configuration from the evidence required to interpret an existing result.
- [ ] Keep core result conversion explicit. Preserve the existing engine schema; adapters must retain field meaning, numeric bits and diagnostics. Do not serialize a result through generic JSON just to cross a Rust boundary.
- [ ] Move `contracts/result-data-contract.json` and the relevant generator logic to the results owner. Keep exactly one generated catalog and preserve ordering/identities. Remove superseded app build-script generation.
- [ ] Move codec bodies outward for R10 while retaining only data/projection interfaces here; display caches and egui painting stay out.

Exit: result calculations/documents test without the app or egui; result digests, exact cursor/measurement outputs, complex/mixed-signal payloads and representative imported/derived datasets agree with baseline. Serialization and reference-currentness failures remain explicit. No results import of simulation-contract/runtime, workbench or UI is left.

**R05 — Model/PDK library and import service**

Sources: `state/model_library`, `state/pdk_config`, portable parts of `state/model_hub`, model import/PDK operations currently hosted by workbench, and pure model/PDK authority from services.

- [ ] Extract catalog/model records, bindings, retained dependency/source bytes, revisions, qualification/correlation evidence, signed-PDK validation, deterministic callback contracts and import preparation into `rspice-model-library`.
- [ ] Resolve model references to `CornerModelBinding`/`CornerProcess` through portable catalog-owned declarations. Model-library must not depend on simulation-contract/runtime to describe its data.
- [ ] Move `model_library/compilation` and executable qualification orchestration into the simulation service boundary. Keep typed inputs, full validation outcomes, authenticated source tokens and publication conditions; reject stale or incomplete receipts as before.
- [ ] Keep credential-bearing cloud transports, host discovery, installation persistence and file acquisition behind explicit adapters. Reuse `rspice-pack` trust and existing cloud-client operations.
- [ ] Move model/PDK worker import handlers out of `workbench::app`; separate bounded decode/validate/prepare from publication into a live catalog. Browser worker results must be bound to the source/project generation that requested them.
- [ ] Keep symbol/design materialization in the design/app adapter rather than introducing model-library → design. Move a genuinely shared descriptor below both owners if required; preserve one canonical schema.

Exit: catalog/import validation and trust tests run headlessly; installed/catalog/retained model semantics remain distinct; source pinning, callback metering, invalid signature, incompatible technology and stale-import tests pass. No GUI, app state, cloud credentials, host picker, or simulation-runtime dependency is present in the portable library.

**R06 — Simulation plan and wire contracts**

Sources: `simulation/plan`, `config`, `dialog` data/validation, `run_set` declarations, `workbench/app_state/sim_setup*`, selected `state/workspace/plan_data` and `saved_output`, worker request/response definitions.

- [ ] Create a persisted plan catalog and authored analysis model in `rspice-simulation-contract`; move stable IDs, revisions, dependencies, tombstones, lineage, plan model bindings, save policies, specifications and saved outputs with their authoritative owner.
- [ ] Classify every `SimSetupState` field by actual Serde behavior. `skip_serializing` legacy read fields still participate in migration; `serde(skip)` edit buffers and migration notices are session concerns. Preserve supported read behavior through dedicated migration adapters, then delete the old mixed struct when consumers have moved.
- [ ] Move pure plan validation and authored-expression handling; keep design/model resolution and resource/capability checks requiring live services in preparation. Do not invent a second validator with different rules.
- [ ] Move portable progress/error/request/envelope types and protocol constants from `runner/worker_contract`; keep solver calls, core-result conversion, JS marshalling and transfer allocation in runtime/transport adapters. Retained checkpoint payload identity belongs below execution; checkpoint production remains above it.
- [ ] Preserve field-to-engine projection and solver-option ownership tests across the new boundary. Compile-time accessors must not allow arbitrary construction of validated plans/permits.

Exit: project code can name and persist plans without importing workbench or execution. Every supported authored field still reaches the same engine-facing projection or documented non-engine consumer. Native/browser packet semantics and protocol version remain unchanged by relocation.

**R07 — Hardcopy contracts**

Sources: `hardcopy/contract`, `mappings`, `sources`, their public value types and pagination tests.

- [ ] Extract `rspice-hardcopy-contract` containing persisted settings, print mappings, physical geometry, frozen source references and deterministic pagination/receipt rules.
- [ ] Keep report authentication references through results and existing drawing-sheet types through `rspice-design-model`; move no renderer, symbol library, printer API or egui type into this crate.
- [ ] Update design/project fields to import the contract. Remove the old application-owned copy and temporary aliases after consumers migrate.

Exit: persisted print settings and authenticated source records are compatible; pagination tests pass natively and compile on wasm32. The offline sheet publisher's dependency closure remains unchanged by this extraction.

**R08 — Design documents and edit transactions**

Sources: selected `state/schematic`, `symbol`, `library_browser`, `workspace` design portions, `configuration_set`, source registry/netlist documents, `property_types`, pure schematic connectivity and `services/drc`.

- [ ] Split `SchematicState` into document content, edit/history authority and per-view interaction. Preserve serialized field names/layout via the existing format adapter rather than changing the file because a Rust field moved.
- [ ] Extract documents, library hierarchy, configuration binding, symbol/property definitions, source text/closure ownership, connectivity, DRC and atomic design/reference edits into `rspice-design`.
- [ ] Keep pan/zoom/selection/gesture/hover/render caches for R15. History of committed document edits belongs with the transaction owner; an in-progress gesture belongs with the editor session.
- [ ] Move only design-owned portions of `ProjectWorkspace`. The outer project descriptor, per-plan payload collection and retained result aggregation wait for R09. Reference transactions spanning design and saved outputs get an upper project coordinator, not a design → simulation-contract edge.
- [ ] Move reusable symbol geometry/source descriptors below rendering where required. Preserve model-bound pins, arrays, buses, hierarchy identity and source maps exactly.
- [ ] Replace broad mutation with typed operations using expected revision/owner. Document references borrowed for rendering must not permit bypassing validation.

Exit: headless edit/undo/redo, hierarchy/variant/annotation/configuration and source-identity cases pass; stale or invalid cross-document edits remain atomic; netlist outputs agree through the still-integrated app. No egui, file chooser, application state or project aggregate dependency exists.

**R09 — Project aggregate, format and save lifecycle**

Sources: `io/project_io*`, `project_execution*`, project-level `state/workspace` operations, `workbench/lifecycle/project_lifecycle/*`, project snapshot/history and persistence adapters.

- [ ] Create the project aggregate over design, plans/models, retained results and print contracts. Maintain one accepted baseline and one explicit working set; derived active-document projections must identify their authority.
- [ ] Move project decode/validate/migrate/encode, `ProjectExecutionContext`, result persistence and document registry into `rspice-project`.
- [ ] Move headless dirty tracking, Save/Save all candidate construction, accepted-generation updates, conflict outcomes and transaction completion into project. Keep chooser dialogs, confirmation presentation, recent-files UI and egui storage integration in app.
- [ ] Define storage operations that preserve expected-content/CAS, binding identity, publication uncertainty, cancellation/generation and recovery outcomes. Keep native/browser implementations outside domain rules and injectable in tests.
- [ ] Audit `io/durable_file` against `rspice-output` before sharing implementation. Preserve writer leases, recovery journals/slots, predecessor bytes, rollback failure and durability guarantees rather than replacing them with generic atomic write.
- [ ] Split canonical project persistence from eframe session recovery. A recoverable session or filename alone must not acquire canonical Save authority.

Exit: native/browser open/save/recovery tests, selected-tab Save versus Save all, bad/stale files, crash/fault injection, session restoration and project publication/rollback cases pass. Project loading does not depend on simulation execution or GUI. Supported baseline files preserve their required round-trip/canonical digest behavior.

**R10 — Engineering-data formats**

Sources: `io/waveform_io`, format portions of `state/engineering_table`, `workbench/menu_bar/waveform_export/*`, and `workbench/workflows/result_import_adapters*`.

- [ ] Move bounded codecs and typed projection-to-bytes conversion to `rspice-formats`; provide reader/writer or byte APIs and structured errors.
- [ ] Move direct codec dependencies with their actual consumers: Arrow/Parquet, spreadsheet, CSV, FST/VCD, HDF5, NumPy and MATLAB. Delete dependencies genuinely left unused.
- [ ] Keep active-view selection, filenames/dialogs, atomic host publication and success/failure notifications in app coordinators. Project schemas remain owned by project.
- [ ] Preserve exact units, complex values, event ordering, selected/hidden-column policy, missing/nonfinite representation and metadata. Keep import/export validation consistent with result contracts.

Exit: existing format fixtures and round-trip/independent-reader checks pass; empty/truncated/oversized/unsupported inputs keep their documented errors. Coders can test codec behavior without starting eframe. New optional feature selections must be qualified independently rather than relying on workspace feature union.

**R11 — Simulation preparation and execution**

Sources: `simulation/netlist_gen`, `execution`, `runner`, `engine_bridge`, `veriloga`, `multi_run`, `optimizer`, `output_contract`, execution parts of `controller`, and `services/simulation_runner`, safety/yield execution.

- [ ] Separate controller work into input/preparation coordination, headless execution, and application reaction. Pass borrowed design/model/plan inputs and exact revisions, not `AppState` or `ProjectWorkspace` wholesale.
- [ ] Extract `rspice-simulation` with source sealing/compilation, netlisting, dependency resolution, resource/capability checks, immutable prepared snapshots, permits, scheduling, cancellation and typed output publication.
- [ ] Keep plan/core lowering and engine/result conversion together by analysis semantics. Consolidate duplicates only where behavior, errors and numerical contracts actually match; preserve the existing engine as the numerical authority.
- [ ] Move compile worker operations from `workbench/documents/code_workspace/veriloga_worker` into the compiler service. Editor diagnostics become a UI projection of structured compiler outcomes.
- [ ] Remove `PreparedRunSnapshot::apply(&mut AppState)` and viewer/export workflow imports from runtime. App receives typed completion/derived-data events and decides which document or tab to update.
- [ ] Preserve monotonic project/document/run identity, stale preparation rejection, single-use dispatch, cancellation during compilation/engine startup/execution, checkpoint resume identity, bounded live queues, and lossless terminal results.
- [ ] Keep native/portable/Wasm-JIT features explicitly wired at platform entrypoints. Preserve current generated-model catalog feature selection and bytecode fallback semantics.
- [ ] Preserve execution authorization and capability checks at the operation boundary. New public library APIs must not bypass the validation, source trust, licensing/capability policy or single-use dispatch conditions that currently gate the same action.

Exit: headless generated-design and authored-deck runs pass representative analog, RF, statistical, mixed-signal and Verilog-A cases; prepared/dispatch and field-projection suites pass; cancellation and native/browser transport tests pass. No workbench, egui, active viewer, export dialog or application-state dependency remains.

**R12 — Hardcopy rendering and worker services**

Sources: `workbench/hardcopy_adapters/{sources,render}`, `workbench/app/dialogs/hardcopy` execution/worker operations, printable symbol geometry and plot/report adapters.

- [ ] Extract `rspice-hardcopy` over frozen design/result inputs and hardcopy-contract. Separate resolving live application selections from rendering the resolved immutable source set.
- [ ] Preserve semantic scenes, fonts, units, pagination, raster limits, source authentication and output provenance. Replace egui-dependent symbol access with the shared geometry contract from R08.
- [ ] Move worker decode/render/encode operations to the service. Keep actual printer discovery, native window handles, driver dialogs, spooling and browser print-window lifecycle in target adapters.
- [ ] Move Krilla/SVG/raster/print codec dependencies according to their actual consumers. Keep common assets pinned and included from one owner.

Exit: hardcopy tests, rendered reference artifacts, source-currentness checks, PDF validation and print cancellation/error behavior pass. Worker rendering has no GUI dependency; persisted models depend on hardcopy-contract, never on the renderer.

**R13 — Independent browser worker**

Sources: `worker_main.rs`, worker exports in `lib.rs`, runtime transport under `runner/worker_contract`/`wasm_worker`, `web/simulation-worker.js`, loader/build scripts.

- [ ] Create `rspice-worker`, with native host testability for portable logic and a wasm32 worker entrypoint. Move JS exports and worker-specific JIT bridge/probes to this host.
- [ ] Route every current task family to its library: simulation, Verilog-A compile, hardcopy, model import, and PDK import. Verify dispatch completeness against the current JS worker routes.
- [ ] Keep typed transferable buffers, size validation, detach/ownership semantics, nonfinite handling, error payloads, protocol versioning, startup probes and per-worker generation checks.
- [ ] Replace app-package worker build commands with `-p rspice-worker --bin rspice-worker`; keep existing delivered worker filenames through wasm-bindgen output naming. Remove `browser-worker` from the app once its last worker-only path is gone.
- [ ] Remove the old `worker_main.rs`, app worker exports, worker-only JIT/encoder dependencies and obsolete tests/fixtures replaced by the new owner. Keep cross-image scenarios in the integration harness.
- [ ] Verify app and worker build identity/protocol compatibility from the same release source and immutable asset cohort. Preserve current mismatch rejection; add a focused mismatch regression if an existing check is insufficient.

Exit: `rspice-worker`'s target-resolved normal/build dependency closure contains **no `rspice-app`, egui, eframe, egui-wgpu, or wgpu**. All five task families, JIT startup/probes, startup cancellation, checkpoint resume and worker restart recovery pass against real browser assets. An unused symbol disappearing through LTO is not evidence of a clean dependency graph.

**R14 — Egui UI kit**

Sources: `ui/{tokens,palette,theme,fonts,icons,widgets,input,accessibility,viewport,plot,raster}` after exact math ownership is settled in R04.

- [ ] Extract `rspice-ui-kit` with narrow reusable rendering/interaction APIs. Own theme application and generic controls; leave workbench composition and feature-specific availability outside.
- [ ] Separate `PlotSpec`/painting and disposable decimation from authoritative sampling. Share exact helpers from results or accept readout values/callbacks from the caller; do not fork interpolation rules.
- [ ] Move pinned font/icon assets and include paths without changing visual output. Preserve DPI, text sizing, keyboard focus, accessibility descriptions and gesture behavior.
- [ ] Keep engine/project/model-service dependencies out of widget APIs. Use typed display inputs and interaction outcomes rather than callbacks granted the full app.

Exit: representative widget and plot render/interaction tests pass in a small egui harness; no workbench or whole-app dependency remains. Measure the optional results edge and incremental rebuild effect before fixing the final minimal dependency list.

**R15 — Schematic and symbol editor**

Sources: `schematic/view`, symbol editor/palette/rendering, relevant property widgets, and local editor session portions of workbench.

- [ ] Extract `rspice-schematic-editor`. Its input is a design view, per-view editor session, explicit capabilities and narrowly scoped services; its output is typed edits/navigation/cross-probe requests.
- [ ] Validate and commit edits through R08/R09 owners. Carry expected document/revision/occurrence so gestures begun on one document cannot complete into another.
- [ ] Preserve selection, wire/bus routing, symbol geometry, annotation, clipboard, undo grouping, grid/snap, touch/pen/keyboard interactions, zoom/pan and context menus.
- [ ] Route cross-feature actions through app coordination. Remove `RSpiceApp`/`AppState` parameters from the extracted code and remove temporary widget/editor duplicates.

Exit: focused egui harnesses and actual app workflows cover edit→undo→save→reload→netlist→simulate, reused hierarchy occurrences, symbol/model edits and document switching during gestures. Desktop and narrow/touch interaction retain their behavior; no application package dependency exists.

**R16 — Results, Visualization Studio and report UI**

Sources: `workbench/documents/result_document*`, `visualization_studio*`, result/report authoring surfaces and their local session state.

- [ ] Extract `rspice-results-ui` over explicit dataset/document views and local viewer state. Keep simulation start/stop, project save, file dialogs and application routing outside.
- [ ] Route exact cursor/readout/calculator operations to results; return cross-probe/export/navigation actions with source identity to app.
- [ ] Preserve linked views, axis/unit transforms, trace visibility, selection, provenance/currentness, streamed partial-result status, report references and print/export selection semantics.
- [ ] Move reusable viewer tests alongside the implementation; keep application lifecycle and real-worker scenarios in app integration tests.

Exit: viewers render and interact in a focused harness; baseline visual and numerical readouts agree; stale results, in-flight document replacement, progressive updates, report currentness and bounded large datasets behave correctly. No global application mutation escapes this boundary.

**R17 — Application composition and legacy closeout**

- [ ] Reduce `RSpiceApp` to subsystem ownership, host lifecycle and wiring. Replace the mixed `AppState` with explicit document/project, execution, result, and UI-session owners; retain an app-level aggregate only as composition.
- [ ] Keep menus, docks, commands and navigation in coherent workbench modules. Do not create a crate per panel. Give commands only the owners/services they need; document the small number of genuine cross-feature coordinators.
- [ ] Retain cloud account/licensing, live collaboration, automation runtime integration and platform adapters as explicit app/service modules over the existing crates. Preserve their authority checks, session invalidation and source access rules.
- [ ] For Code/Automation, move source ownership/compilation to the already extracted owners; retain editor/debugger presentation and cross-workspace orchestration. Do not leave a second source registry in workbench.
- [ ] Delete superseded module implementations, stale re-export chains, unreachable catalog entries with no product/format consumer, dead feature flags, unused dependencies, old binary entrypoints and redundant migration scaffolding according to the removal register.
- [ ] Replace the old single-crate public-module/line-count assumptions with appropriate per-crate API/layering checks. Retain meaningful size ratchets; shrinking a file is not a substitute for resolving ownership.
- [ ] Remove temporary compatibility imports, duplicated source trees and migration TODOs. Update every package README and the application architecture documentation to describe the implemented graph.

Exit: no extracted library accepts `RSpiceApp` or `AppState`; no production dependency points toward app; each remaining app coordinator has a concrete composition role. Every removal/register item is deleted or has a documented live consumer, not an unexplained deferred cleanup status.

**R18 — Integrated qualification**

- [ ] Qualify the exact integrated source commit using the matrix below. Verify tests moved across packages are still selected by CI; list/run per-package tests rather than relying on one giant workspace feature union.
- [ ] Compare complete supported scenario coverage, golden/canonical data, numerical results, native/browser transfer, platform operations and packaged launch against R00.
- [ ] Compare clean/incremental builds, test linking, artifact size, startup, interaction latency and memory using the same toolchain/fixtures/host methodology. Investigate material repeatable regressions; do not claim an improvement without a matched measurement.
- [ ] Re-run dependency and feature closure checks for each shipped image and every new headless library. Confirm the drawing-sheet publisher and existing non-UI consumers retain appropriate dependency closure.
- [ ] Finish architecture/removal registers and package documentation. Record remaining product qualification gaps separately from refactor completion; do not silently mark an unsupported physical device or OS as tested.

Exit: every work-package condition is satisfied on the integration commit; no waived new failures, duplicate implementation routes or unresolved supported-data regressions remain. Refactor completion establishes the architecture and preservation evidence described here, not certification of every numerical/product capability.

**Legacy-removal procedure**

Perform this during each owner migration, with a final sweep in R17. Do not postpone all removal to the end.

| Candidate | Required evidence | Action |
| --- | --- | --- |
| Unreachable production function/module/feature branch | Check native + Wasm app + worker, supported feature selections, entrypoint exports, generated/macro registration, callbacks and tests | Delete implementation, obsolete exposure and unused dependency; retain or relocate meaningful behavior tests |
| Replaced internal adapter, alias or compatibility facade | All live callers use the new owner; old path owns no wire/storage identity | Delete in the same migration package |
| Prototype or unimplemented advertised route | Verify product catalog, command availability, imports and saved identifiers; establish it is not a supported live feature | Remove unused code and stale metadata together without silently changing supported behavior |
| Test-only helper/fixture | Determine whether it tests a supported invariant or only a deleted implementation | Move useful scenario coverage to the real owner; delete obsolete scaffolding |
| Legacy project/session/checkpoint reader | Check supported-version inventory, golden files, recovery semantics and any writer still producing it | Delete only if there is no supported input or live writer; otherwise retain the smallest actual migration |
| JS export, ABI name, storage key, runtime asset | Check external/packaged consumers and loader/release contracts; source search alone is insufficient | Preserve while live; delete obsolete internal duplicates, not the currently consumed identifier |
| `allow(dead_code)` or stale lint waiver | Compile the meaningful target/feature variants and examine actual diagnostics | Delete unused code or waiver; retain a narrowly justified exception only for an actual supported build condition |

The register records candidate, former owner, remaining consumers, target/feature evidence, persisted-format impact, test coverage disposition, dependency savings, deletion commit and verification result. Do not keep an item “just in case” after evidence establishes no consumer. When evidence shows a live compatibility role, record that role and continue other work rather than treating the item as dead.

**Verification matrix and commands**

These are implementation acceptance checks, not checks already executed while writing this plan. Commands use the final names where indicated; apply the transitional worker command from R02 before R13. Follow the pinned versions and environment in `.github/workflows/ci.yml` and `.github/CI.md` rather than upgrading tooling during the refactor.

| Gate | Run when | Coverage |
| --- | --- | --- |
| G1 — Local owner | Every ownership/edit/deletion slice | Affected owner tests, moved scenarios, formatting, affected-package all-target Clippy, relevant module/package guards |
| G2 — Application integration | Each completed extraction | Application units and integration tests, project/session lifecycle, representative design→run→results flows and all changed contract consumers |
| G3 — Browser boundary | Changed shared types/transport/build paths | App and worker wasm32 checks, worker transfer tests, window/worker clocks, mismatched/stale messages, error and cancellation paths |
| G4 — Real browser | R02, R13, presentation integrations and final | Optimized UI/worker assets, JIT, workbench, recovery, startup cancellation, checkpoint workflows, packaged runtime assets |
| G5 — Native hosts | Platform/storage/render changes and final | Windows/macOS/Linux app tests/builds, native JIT policies, file durability, printing, managed automation and staged package launch |
| G6 — Compatibility | Every persisted/wire owner move | Supported JSON/RON versions, digests/identities, old recovery/checkpoint fixtures, malformed input, source trust, independent export validation |
| G7 — Performance | Baseline, representative extractions and final | Matched rebuild/link measurements; Wasm raw/gzip sizes; startup, memory, bounded queues/caches and interaction hot paths |
| G8 — Full integration | R18 | Existing CI/nightly qualification selections, existing bindings/CLI/publishers, dependency policy, legacy register and source attribution |

Core application commands after R02:

```text
cargo test --locked -p rspice-app --no-fail-fast
cargo test --locked -p rspice-app --features generated-veriloga-catalog --lib generated_veriloga
cargo clippy --locked -p rspice-app --all-targets -- -D warnings
cargo check --locked -p rspice-app --features generated-veriloga-catalog --target wasm32-unknown-unknown
cargo test --locked -p rspice-app --test module_layering --test hierarchy_guards
```

Update the last command as those guards become workspace/package checks; keep the invariant and scenario coverage, not a stale path. Add `cargo test --locked -p <new-package> --no-fail-fast`, native all-target Clippy and wasm32 checks for each portable package when it is introduced. Select default and required nondefault features independently. A “passed” command that selected zero intended tests is not acceptance.

Shipping Wasm builds after R13:

```text
cargo build --locked --profile web-release -p rspice-app --bin rspice-app --features generated-veriloga-catalog --target wasm32-unknown-unknown
cargo build --locked --profile web-release -p rspice-worker --bin rspice-worker --features generated-veriloga-catalog --target wasm32-unknown-unknown
```

`rspice-worker` must define/forward `generated-veriloga-catalog`; its browser JIT execution dependencies are owned by that worker build. Generate bindings with the existing pinned wasm-bindgen version and explicit existing output stems, then run the production artifact-size reporting tool. Build qualification-instrumented app assets separately so instrumentation does not distort shipping measurements.

The feature matrix must explicitly retain qualified desktop native-JIT/parallel/SIMD configurations, portable bytecode configurations on other supported targets, the interactive Wasm image's portable validation/artifact requirements, and worker-only secondary-module JIT emission/verification. Qualify generated-catalog enabled and disabled builds. Inspect each image independently; a successful workspace build can mask a missing feature declaration in one package.

Use the existing harnesses after updating their root defaults:

```text
python tools/ci/test_ci_configuration.py
python tools/ci/test_ui_build_identity.py
python tools/release/test_package_native.py
python tools/ci/test_automation_runtime_assets.py
python tools/ci/test_ide_worker.py
python tools/ci/test_qualify_drawing_sheet.py
python tools/ci/test_browser_workbench.py
python tools/ci/check_wasm_jit_browser.py --web-root crates/rspice-app/web
python tools/ci/check_browser_workbench.py --web-root crates/rspice-app/web --output target/app-refactor/browser-workbench
python tools/ci/check_browser_engine_recovery.py --web-root crates/rspice-app/web --output target/app-refactor/browser-recovery
python tools/ci/check_browser_engine_recovery.py --web-root crates/rspice-app/web --output target/app-refactor/browser-cancel --cancel-startup
python tools/ci/check_browser_monte_carlo_checkpoint.py --web-root crates/rspice-app/web --output target/app-refactor/browser-checkpoint
```

Use `py -3` on Windows when required. Browser commands require the assets, browser/driver and environment used by the existing harness. Preserve the Node loader/structured-clone tests and the Firefox window/dedicated-worker clock lanes from CI; move tests to the appropriate new owner without losing their browser-runner configuration. Run `check_browser_release.py` against the staged release output with its existing CLI, because the source web tree is not a packaged deployment.

Use current native/bindings/conformance matrices rather than inventing an all-features build as a replacement. Respect the repository's $0 infrastructure policy. Building/qualifying local staged packages is sufficient here; do not dispatch publication or signing/release workflows merely to test the refactor.

**Change sizing, evidence and rollback**

Each integration change states its source commit, owner being changed, temporary adapters, deleted legacy, retained schemas/ABIs, test selections/results, and measured effects. Mechanical moves should be reviewable separately from changed algorithms or validation decisions. Keep manifests, build scripts and their consumers in the same integration change.

Tests migrate with their owners. Place cross-domain scenarios at the lowest consumer that legitimately depends on both owners, usually app integration tests; avoid a lower crate's dev-dependency on the app. Use narrow test support only for genuinely shared fixtures, without exposing unchecked production constructors or compiling all fixtures into shipping images.

On a regression, first identify the owner change and reproduce it on the recorded source/feature combination. Revert the specific refactor change or its dependent migration batch through ordinary reviewable commits; preserve other work. Do not restore a permanent duplicate implementation to make a gate green. Because ordinary package moves preserve persisted schemas, rollback should not require modifying user project files. Any unavoidable format/protocol change must be separated, versioned and qualified before it can enter this sequence.

The integration receipt records source/lock hashes, target/features/profile, command exit status, passed/failed/ignored tests, exact browser assets, numerical/serialization comparisons and performance methodology. A passing subtotal from a failing process is a failure. Re-run only checks affected by new changes during iteration, then the broader integration gates when the work package is complete.

**Completion checklist**

- [ ] `rspice-app` is the actual package/directory/library and main Cargo binary; build/package consumers are updated.
- [ ] Each new library has a clear owner, narrow validated API, executable tests, documented dependencies and no upward app dependency.
- [ ] Design/plan/result/project ownership is explicit; no duplicate authoritative state or migrated whole-app service locator remains.
- [ ] The worker is a separate GUI-free package and preserves every supported task family and transport guarantee.
- [ ] Required file, recovery, checkpoint, publication, source-authentication and numerical behavior is preserved.
- [ ] All confirmed-unused legacy implementations and temporary refactor scaffolding are removed; retained compatibility code has a named live consumer.
- [ ] Existing engine, CLI, bindings, sheet publisher, cloud and automation consumers continue to pass their relevant gates.
- [ ] Desktop/browser/presentation qualification is recorded accurately; mobile compile coverage is not presented as tablet interaction qualification.
- [ ] Dependency guards run in CI, moved tests are selected, documentation matches code, and measured regressions are resolved or explicitly handled before completion.
- [ ] R00–R18 are complete on the same integrated source lineage, with evidence sufficient to reproduce the result.

The first implementation batch is R00/R01: capture the baseline, restore the architectural guards through real ownership repair and confirmed-unused-code removal, and establish the staged boundaries. The rename is the next separately reviewable change. The rest of the plan follows the dependency order above.
