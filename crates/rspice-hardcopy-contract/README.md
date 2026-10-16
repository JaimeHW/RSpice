# rspice-hardcopy-contract

The hardcopy data contract: page, render and print-mapping setup; the
immutable `HardcopyPlan` an exact document snapshot and a validated setup
compile into, paginated deterministically in integer micrometres; and the
sealed receipts, setup store, mapping catalogs and source sets a project
retains. `rspice-project` stores these records in the project file, and
`tools/ci/check_app_crate_dependencies.py` does not allow `rspice-project` to
depend on `rspice-hardcopy`, which carries the PDF, SVG and raster backends;
the contract is its own crate so the project file and the renderer read one
definition. The check allows this crate only `rspice-app-types` and
`rspice-results` among the application crates, and refuses egui, eframe,
winit, wgpu and rfd in its closure.

## What's in here

- **`contract`**: identities (`HardcopyDocumentId`, `HardcopyPlanId`);
  `HardcopyDocumentKind` and the `HardcopyScope` values each kind admits;
  `ActiveHardcopySource`, the exact revision and content digest a plan is
  bound to. The setup: `PhysicalPageSetup` (paper, margins, orientation,
  bleed), scale and tiling, `RenderSetup` with its `OutputFormat` and
  `RenderTarget` (printer job settings included), decorations and watermarks,
  the schematic extent policy, and `PrintMappingTable` for color and object
  mapping.
  `HardcopyPlan` with its `PreviewPagination`; `HardcopyOutcome` and
  `HardcopyReceipt`, which records what the platform accepted and never
  claims more. The persisted `HardcopySetupStore` and `HardcopyReceiptLedger`.
- **`mappings`**: `PrintMappingPresetCatalog`, reusable mapping tables owned
  by the project or by a personal profile, routed by each table's declared
  save scope. Document-scoped mappings stay inside the document's setup.
- **`sources`**: `HardcopySourceSet`, the persisted, digest-authenticated
  list of exact document revisions a multi-document print covers, and the
  source identity and validation helpers the document adapters share.

## Boundaries

- Resolving live documents into printable scenes, and rendering them to PDF,
  SVG, PNG, TIFF or a print document, belong to
  [`rspice-hardcopy`](../rspice-hardcopy).
- The dialog, printer and browser print integration belong to
  [`rspice-app`](../rspice-app).

## Entry points

- `ActiveHardcopySource::try_new(document_id, revision, content_digest,
  display_name, kind, scope)` for the source authority.
- `HardcopySetup::try_new` (or `try_new_with_schematic`) from its parts, then
  `HardcopyPlan::compile(source, setup, content_extent)`.
- `HardcopyPlan::pagination` for the plan's pages.
- `HardcopyReceipt::record(&plan, outcome)`, then
  `HardcopyReceiptLedger::append`.
- `HardcopySetupStore::save(&source, setup)` and `setup_for` for per-document
  Page Setup.

## Persisted records

The project file carries each record below; personal mapping presets are kept
with the user's preferences instead. Every record is revalidated, fail-closed,
when it is read.

| Record | Contract |
| :--- | :--- |
| `HardcopySetupStore` | Per-document setups keyed by `HardcopyDocumentId`, each sealed by a content digest; versioned by the private `SETUP_STORE_SCHEMA_VERSION` |
| `HardcopyReceiptLedger` | Individually sealed outcomes, including cancellations and failures; at its private `MAX_RETAINED_HARDCOPY_RECEIPTS` bound the oldest receipt is dropped |
| `PrintMappingPresetCatalog` | Named mapping tables with an owner and revision; versioned by the private `PRINT_MAPPING_CATALOG_SCHEMA_VERSION` and bounded by the private `MAX_PRINT_MAPPING_PRESETS` |
| `HardcopySourceSet` | Versioned by `HARDCOPY_SOURCE_SET_SCHEMA_VERSION`; at most `MAX_HARDCOPY_SOURCE_SET_MEMBERS` members, each pinning a document revision and content digest |

A plan holds at most `MAX_PREVIEW_PAGES` pages, and raster output stays
between `MIN_RASTER_DPI` and `MAX_RASTER_DPI`.

## Testing

```bash
cargo test -p rspice-hardcopy-contract
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
