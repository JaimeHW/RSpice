# rspice-project-contract

The project's own identity records: `ProjectDescriptor`, with its technology
binding and its authenticated audit trails, and `ProjectLibraries`, the
library catalog as a project file stores it. `rspice-project` persists them,
`rspice-simulation` validates a project's technology against the descriptor
and `rspice-hardcopy` carries a library snapshot to resolve a schematic's
symbols for print. `tools/ci/check_app_crate_dependencies.py` refuses
`rspice-simulation`, `rspice-worker` and this crate any path to
`rspice-project`, so these records live here, where execution and hardcopy can
read them without linking the project file and its save lifecycle. The check
allows this crate only `rspice-app-types`, `rspice-design` and
`rspice-model-library` among the application crates, and refuses egui,
eframe, winit, wgpu and rfd in its closure.

## What's in here

- **`descriptor`**: `ProjectDescriptor`, with the stable `ProjectId`, logical
  revision, name, root library and top cell, include search paths, the RSpice
  Cloud publication binding and the technology binding
  (`ProjectTechnologyBinding` and `ProjectSignedTechnologyPin`, re-exported
  from `rspice-model-library`). Its hash-chained audit trails include
  `ProjectTechnologyChangeReceipt`, the record of an audited technology
  attach or replace, committed only with checkpoint evidence for the exact
  pre-change revision, and `ProjectLibraryMutationReceipt`, prepared before
  and published after a library mutation. Versioned by
  `PROJECT_DESCRIPTOR_SCHEMA_VERSION`,
  `PROJECT_TECHNOLOGY_CHANGE_RECEIPT_SCHEMA_VERSION`,
  `PROJECT_LIBRARY_MUTATION_RECEIPT_SCHEMA_VERSION` and
  `PROJECT_CLOUD_PUBLICATION_BINDING_SCHEMA_VERSION`; bounded by
  `MAX_PROJECT_TECHNOLOGY_CHANGE_RECEIPTS` and
  `MAX_PROJECT_LIBRARY_MUTATION_RECEIPTS`.
- **`library_publication`**: `ProjectLibraryPublicationReceipt`, the third
  trail the descriptor carries: an immutable, hash-chained record of a
  published library snapshot (the exact digest and size of the validated
  project file it froze, revisions, identity and authority). The caller
  persists the artifact itself. Versioned
  by `PROJECT_LIBRARY_PUBLICATION_RECEIPT_SCHEMA_VERSION`; bounded by
  `MAX_PROJECT_LIBRARY_PUBLICATION_RECEIPTS`.
- **`libraries`**: `ProjectLibraries`, the governed `LibraryCatalog` plus
  the persisted selection and filter fields, serialized in the project file's
  existing `libraries` layout; the initial user library.

## Boundaries

- The project file, its migration and its save lifecycle belong to
  [`rspice-project`](../rspice-project), which re-exports this crate.
- Libraries, cells, views and their mutations belong to
  [`rspice-design`](../rspice-design); the technology binding and signed-PDK
  pin types to [`rspice-model-library`](../rspice-model-library).

## Entry points

- `ProjectDescriptor::new(name, root_library, top_cell)`, then `validate`.
- `ProjectDescriptor::attach_technology_audited(binding, context)` with a
  `ProjectTechnologyChangeContext` carrying the actor, authority and checkpoint
  evidence.
- `ProjectDescriptor::prepare_library_mutation`, then
  `publish_library_mutation` once the catalog change has been applied.
- `ProjectDescriptor::publish_library_snapshot` with a
  `ProjectLibraryPublicationDraft`.
- `ProjectLibraries::with_primitives` for a new project's catalog;
  `edit_library` for a revisioned library edit.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `library-test-fixtures` | Exposes `ProjectLibraries::get_library_mut`, direct mutable access that bypasses library edit transactions, and enables `rspice-design/schematic-test-fixtures`. Reached through `rspice-project/library-test-fixtures` from `rspice-app`'s dev-dependency |

## Testing

```bash
cargo test -p rspice-project-contract
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
