# rspice-design

The headless design model of the RSpice application: schematic documents and
their edit transactions, the one connectivity extraction, hierarchy resolution
under configuration sets, the electrical rule checker, libraries and symbol
views, and the embedded component-symbol artwork. The application, project,
simulation, hardcopy, worker and schematic-editor crates all read a design
through this crate, so the emitted deck, the net summary and the rule checker
cannot disagree about what is connected.
`tools/ci/check_app_crate_dependencies.py` allows it only `rspice-app-types`,
`rspice-model-library` and `rspice-hardcopy-contract` among the application
crates, and refuses egui, eframe, winit, wgpu and rfd in its closure.

## What's in here

| Module | Contents |
| :--- | :--- |
| `schematic` | `SchematicDocument`, the persisted drawing: components, wires, buses and taps, net labels, junctions, probes, design notes, documentation shapes and the validated-save journal. `owned::Schematic`, the editable document with identity allocation, nested operation scopes and bounded undo history. `component_type::ComponentType`, whose `ALL` is the pinned list of component kinds. The document edits (placement, wiring, buses, arrays, clipboard, deletion, movement, stretch, replacement) and the built-in XSPICE and generated Verilog-A device catalogs |
| `connectivity` | `extract` and `extract_with_hierarchy`: wires, junctions, terminals, net labels, interface ports, bus taps and ground become nets here and nowhere else; bus analysis, deck and display bit names, and the per-terminal net summary |
| `connectivity_contract` | `ConnectivityContract`: project connectivity policy and the technology and dialect catalogs that declare which net names are equivalent |
| `hierarchy` | `resolve_hierarchy`: walks placed instances to exact Library/Cell/View bindings, records whether each row was configured or fell back, and names each master once by binding digest. `HierarchySource`, the borrowed symbol, catalog and global-label authority design queries and netlisting share |
| `configuration_set` | `ConfigurationSetCatalog`: the design root, DUT, executable and stop views, scoped overrides and model profile that take part in netlisting, changed by clone-and-validate transactions |
| `projection` | `DesignProjection`: the live design paired with the active configuration's execution plan, memoized by content, which runs, checks and exports all read; `ProjectionContext` and `DesignProjectionCache` |
| `drc` | The electrical rule checker: `DrcChecker`, `DrcConfig` (which rules run and at what severity) and `run_check_with_hierarchy`, which checks the subject cell view and every cell view its instances reach |
| `library` | `LibraryCatalog`, `Library`, `Cell`, `View` and `ViewType`; revisioned library mutations and the collaboration-lock authority |
| `symbol`, `symbol_generation`, `symbol_resolver`, `resolved_symbol` | `SymbolDocument`, the persisted symbol view (body artwork, label anchors, pins) with its edit and publication rules; block symbols generated from a cell's interface ports; `SymbolResolver`, which finds the authored or generated symbol for a binding; `ResolvedCellSymbol` and its pin diagnostics |
| `symbol_artwork` | `SymbolLibrary`: the built-in component artwork parsed from the SVGs in `assets/component_symbols/`, with component bindings and terminal geometry. Built only with `symbol-artwork` |
| `model_bound_symbol` | Symbols bound to `rspice-model-library` definitions: materialization, import review and guarded library construction |
| `properties` | `PropertyCatalog`, the typed property sheet for each component kind, and source-parameter contract findings |
| `parameters` | The lexer for authored SPICE parameter text that lookups and structural edits share |
| `netlist_document` | `NetlistDocument`, the project-owned netlist source with its include outline, provenance and bounded find and replace |
| `owned_netlist` | Owned SPICE source metadata, edit strategies and bounded revision history |
| `project_sources` | `ProjectSourceRegistry`: sealed Verilog-A and automation source bundles under portable logical paths, valid on native and WebAssembly alike |
| `occurrence` | `DocumentOccurrence`, the rooted instance chain an open document edits, and its pruning against the surviving library |
| `references` | Hierarchy reference renames and lossless probe-expression rewrites |
| `rf_ports` | `RfPort` and `RfPortMode`, the RF-port declarations schematic inspection and simulation preparation share |
| `physical_layout` | `PhysicalLayoutDocument`: layout cell views in integer database units, changed by expected-revision transactions |
| `stimulus_library` | `StimulusLibrary`: reusable source definitions adopted by copy, and the provenance a placed source keeps |

## Boundaries

- Canvas painting, gestures, editor sessions and SVG export belong to
  [`rspice-schematic-editor`](../rspice-schematic-editor).
- Netlist generation and simulation preparation belong to
  [`rspice-simulation`](../rspice-simulation); turning a stimulus definition
  into a card or waveform belongs to [`rspice-app`](../rspice-app).
- The design-management catalog, `CellViewRef`, ports, buses, page sizes and
  symbol-geometry primitives belong to
  [`rspice-design-model`](../rspice-design-model).
- Project persistence belongs to [`rspice-project`](../rspice-project).

## Entry points

- `schematic::owned::Schematic`: `begin_operation`, the document edits, then
  `end_operation` or `cancel_operation`; `undo` and `redo`.
- `schematic::component_type::ComponentType::ALL` for every component kind.
- `hierarchy::HierarchySource::from_workspace(libraries, buffers)` with
  `with_connectivity(contract)`, or `from_design_projection`; then
  `connectivity::extract_with_hierarchy` for an `ExtractedConnectivity`.
- `hierarchy::resolve_hierarchy(context, root, configuration)` for a
  `HierarchyResolution` and its `ConfigurationExecutionPlan`.
- `projection::ProjectionContext::inspect_design_projection` for a shared
  `Arc<DesignProjection>`.
- `drc::run_check_with_hierarchy(schematic, &hierarchy, DrcConfig::default())`
  for a `DrcResult`.
- `symbol_resolver::SymbolResolver::new(libraries, buffers)`, then
  `resolve_binding`.
- `symbol_artwork::SymbolLibrary::load_embedded` for the built-in artwork.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `symbol-artwork` | Builds `symbol_artwork`: `build.rs` embeds every SVG in `assets/component_symbols/` and `usvg` parses them. Enabled by `rspice-app`, `rspice-hardcopy` and `rspice-schematic-editor` |
| `generated-veriloga-catalog` | Enables `rspice-core/veriloga-builtins`, so the build-time generated Verilog-A models are in the device catalog and can be placed and bound; without it that catalog is empty |
| `symbol-test-observation` | Counts symbol-view deserializations for cache regression tests. Enabled by `rspice-app`'s dev-dependency |
| `work-observation` | Counts the work an edit does (document snapshots, connectivity passes and their steps, wire cleanups) in `rspice-design-model`'s `work_tally`, for tests that hold a gesture to its per-frame work. Enabled by `rspice-app`'s dev-dependency |
| `schematic-test-fixtures` | Exposes test-only hooks (`Schematic::document_mut_for_test`, `set_identity_for_test`, materialization counters) to dependent tests that construct external or invalid edits. Enabled by the dev-dependencies of `rspice-app`, `rspice-schematic-editor` and `rspice-simulation` |

## Testing

```bash
cargo test -p rspice-design
cargo test -p rspice-design --features symbol-artwork,generated-veriloga-catalog
```

The second run adds the embedded-artwork tests and the bindings checked
against the shipped generated Verilog-A catalog.

Licensed under the [RSpice Personal Use License](../../LICENSE).
