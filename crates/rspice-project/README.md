# rspice-project

The project aggregate of the RSpice application: `ProjectFile` and the
workspace it carries, the bounded decoding, migration and validation that open
it, the per-document fingerprints that decide Save, Revert and dirty state, the
single-transaction save lifecycle, and the native and browser persistence
authorities. Only `rspice-app` depends on it.
`tools/ci/check_app_crate_dependencies.py` refuses `rspice-simulation`,
`rspice-worker` and `rspice-project-contract` any path to this crate, and keeps
this crate off `rspice-simulation` and `rspice-hardcopy`. It persists their
contract crates' records instead, so execution, hardcopy and the browser
worker read the same records without linking the project file and its
lifecycle. The check also refuses egui, eframe, winit, wgpu and rfd in its
closure.

## What's in here

| Module | Contents |
| :--- | :--- |
| `file` | `ProjectFile`, the top-level record, and `ProjectVersion`. `serialize_project_file` validates before writing; `decode_project_text` bounds the text, migrates legacy identity, document occurrences, generated Verilog-A bindings, the execution context and retained results, and validates the result, returning a `DecodedProject` whose load-repair warnings are never written back |
| `workspace` | `ProjectWorkspace`: the project descriptor, configuration sets, design-management catalog, connectivity contract, open views and schematic buffers, specifications and per-plan payloads, physical layouts, signed-PDK callback receipts, hardcopy setups, receipts, print mappings and source sets, engineering-table views, report and visualization documents, the stimulus library, the owned netlist document and the sealed Verilog-A and automation sources. `ProjectHierarchy` resolves hierarchy against that authority; `ProjectReferenceTransaction` carries reference renames across configurations and occurrences |
| `execution_context` | `ProjectExecutionContext`: the simulation setup, `ProjectModelLibrary` catalogs, model-resolution records and validation receipt a plan needs to reproduce, with forward migration from every earlier schema |
| `registry` | `ProjectDocumentId`, the documents Save, Save all and Revert act on; `DocumentRegistry` dirty state; canonical document fingerprints and `ResultFingerprintCache` |
| `accepted`, `candidate` | `AcceptedProject`, the immutable accepted baseline with its fingerprints; document-scoped save candidates that overlay one working document onto that baseline and validate the result |
| `working` | `ProjectWorkingSet::capture`, which freezes working content into a validated `ProjectFile` through caller-held `SnapshotSessions` without owning editor sessions |
| `lifecycle` | `ProjectLifecycle`: project incarnation, accepted generation, the one in-flight save or replacement transaction, revert review, and staged browser saves with their continuation authority |
| `persistence` | `ProjectBytes`, one bounded read paired with its digest; `native` destination authority (`NativeSaveDestination`, `NativeCopyDestination`; external-change and writer-lease refusals; not built for `wasm32`); `browser` binding receipts, restoration and staged writes over OPFS or an external file |
| `open_view` | `OpenCellView`, one persisted document tab with its occurrence and read-only marking |

## Boundaries

- The project descriptor, technology binding and library catalog record belong
  to [`rspice-project-contract`](../rspice-project-contract); this crate
  re-exports them.
- Retained result-history persistence belongs to
  [`rspice-formats`](../rspice-formats) and is re-exported here as `results`.
- The simulation setup, plan payload and saved-output vocabulary belongs to
  [`rspice-simulation-contract`](../rspice-simulation-contract); page setup
  and hardcopy receipts to
  [`rspice-hardcopy-contract`](../rspice-hardcopy-contract).
- File dialogs, the `NativeProjectStorage` and `BrowserProjectStorage`
  implementations and editor sessions belong to [`rspice-app`](../rspice-app).

## Entry points

- `persistence::ProjectBytes::read(reader, advertised)`, then `decode`; or
  `decode_project_text(contents, source_path, source_files)` for text already
  in memory.
- `serialize_project_file(&project)`, or `persistence::serialized_project` for
  the bytes together with their content digest.
- `ProjectWorkingSet { workspace, active_schematic, libraries }.capture(...)`
  to freeze the working project.
- `ProjectLifecycle`: `accept_content`, `begin_save`, `dirty_documents`;
  `AcceptedProject::document_candidate` for a single-document save.
- `persistence::native::NativeSaveDestination::new(...)`, then `publish`,
  over a host `NativeProjectStorage`.

## Project file contract

A project is one UTF-8 JSON document, pretty-printed with a trailing newline.

| Element | Contract |
| :--- | :--- |
| Container version | `ProjectVersion::current()`; a file with a different major version is refused as `ProjectIoError::IncompatibleVersion` |
| Size | `MAX_PROJECT_FILE_BYTES`, on both read and write; a legacy file that needs an identity injected has a smaller bound |
| Top-level fields | `version`, `workspace`, `libraries`, `simulation_results`, the flattened `ResultPresentation` fields (`result_markers` and the rest) and `execution_context` |
| Project identity | `workspace.project`, versioned by `PROJECT_DESCRIPTOR_SCHEMA_VERSION`. A descriptor with neither a schema version nor an ID receives a UUID-v5 derived from the complete file bytes; a versioned descriptor without an ID, or an explicit `null` for either field, is refused |
| Execution inputs | `execution_context`, versioned by `PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION`; earlier schemas migrate forward on load and a newer one fails the open |
| Retained results | `simulation_results`, versioned and validated per schema by `rspice-formats`. Results that fail migration or validation are dropped with a warning and the project still opens |
| Bounded collections | `MAX_PROJECT_VISUALIZATION_DOCUMENTS`, `MAX_PROJECT_HARDCOPY_SOURCE_SETS`, and the receipt limits of `rspice-project-contract` and `rspice-hardcopy-contract` |
| Content identity | SHA-256 over the exact serialized bytes (`persistence::digest_bytes`) |

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `library-test-fixtures` | Forwards to `rspice-project-contract/library-test-fixtures`, which exposes direct mutable library access for fixtures that model invalid external catalog edits. Enabled by `rspice-app`'s dev-dependency |
| `document-fingerprint-observation` | Exposes `registry::FINGERPRINT_PASSES`, a per-thread count of full document-fingerprint passes, for cost regression tests. Enabled by `rspice-app`'s dev-dependency |

## Testing

```bash
cargo test -p rspice-project
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
