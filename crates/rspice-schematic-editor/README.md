# rspice-schematic-editor

The schematic and symbol editors' presentation and interaction over canonical
design data: canvas painting, hit testing, gesture and keyboard interpretation,
per-document editor sessions, property and placement drafts, and SVG export.
Schematic documents are only ever borrowed read-only, and the symbol editor
edits a draft its caller owns; what comes back is a typed request, draft or
edit list, and `rspice-app` validates it before applying or publishing
anything. The crate stands apart so that boundary can be enforced:
`tools/ci/check_app_crate_dependencies.py` refuses `rspice-app`,
`rspice-project`, `rspice-project-contract`, `rspice-simulation`,
`rspice-worker`, eframe, winit, wgpu and rfd in its closure.

## What's in here

| Module | Contents |
| :--- | :--- |
| `view` | Active-sheet painting and interaction: the scene painter, grid, wires, buses, net labels, resolved symbols, the drawing sheet, design notes, documentation shapes and check-violation markers; coordinate conversion, pan and zoom, pointer targeting and snap resolution; move, stretch, array and selection-drag gestures, align and distribute, keyboard navigation, the context menu, touch controls and cross-probe selection |
| `session` | `EditorSession`, the per-document interaction state: selection, active tool, unfinished wire and bus gestures, zoom and pan, clipboard, armed placements, the snap engine and net highlighting. None of it is saved with the design |
| `requests` | `EditorRequest` and `EditorRequestSource`: an editor intent bound to the project, document, epochs and versions it was produced against |
| `symbols` | `SymbolLibrary`: the embedded component artwork from `rspice-design`, with cached flattened geometry for interactive painting |
| `component_palette` | The placeable device kinds, grouped into the sections the palette shows |
| `symbol_editor` | The symbol editor's canvas, viewport and hit testing, pointer interaction, clipboard commands, inspector and publication form, over a caller-owned `SymbolDocument` draft |
| `export` | Schematic SVG export, `SvgExportConfig` and `SvgDesignContext`, drawn with the same resolved-symbol and bus-tap geometry as the canvas |
| `component_properties`, `object_properties` | Component-parameter and object-property drafts and their forms, which prepare a validated field delta without mutating the design |
| `property_values` | Typed value editors for property rows: mono inputs for numbers, strings and expressions, chips for small enums, checks for booleans |
| `pwl_editor` | The piecewise-linear point table shared by source-property surfaces |
| `selection_forms` | The Move and Stretch command forms |
| `annotations`, `bus_tap_placement`, `net_label_placement`, `pin_placement`, `xspice_placement` | Placement forms for design notes and documentation shapes, bus taps, net labels and off-sheet connectors, pins, and built-in XSPICE vector widths |
| `bus_geometry` | The angled bus-tap breakout shared by the canvas, hit testing, selection and SVG export |
| `vector_preview` | The deck bit names a declared pin or tap range expands to, shown while the range is still a draft |
| `port_overlay` | Direction marks layered over the interface-port body |
| `source_labels` | Concise source-value labels for the canvas, independent of netlist serialization |

## Boundaries

- Design documents, connectivity, component kinds and symbol artwork belong
  to `rspice-design`.
- Document authority, undo history, live validation and commits belong to
  `rspice-app`, whose `schematic/` module binds this crate to open documents.
- Run provenance, probe materialization and operating-point values are
  resolved by the host; the canvas paints the values it is handed.
- Design tokens, dialogs, form primitives and the command shells belong to
  `rspice-ui-kit`.

## Entry points

- `SymbolLibrary::load_embedded` for the component artwork the scene and
  export painters draw.
- `session::EditorSession`, one per open schematic document.
- `view::scene::draw_content` with `SceneView`, `SceneSymbols` and
  `SceneOverlays` to paint the active sheet.
- `view::tool_input::ToolInput::read` for a frame's canvas events,
  `ToolPointerView::action` for the typed `ToolPointerAction` a click makes,
  and `view::pointer_target::pointer_target` for what lies under the pointer.
- `requests::EditorRequest` with `EditorRequestSource`, which the host compares
  with its current owners before applying.
- `component_palette` for the placeable device kinds, grouped into palette
  sections.
- `symbol_editor::draw_canvas` and `symbol_editor::interaction::edit_canvas`
  with a `SymbolEditorSession`.
- `export::export_to_svg_with_symbol_resolver_and_context` with an
  `SvgExportConfig`.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `test-support` | Exposes `view::net_labels::hit_bounds`, a net label's exact screen hit rectangle, to interaction tests outside this crate. `rspice-app` enables it as a dev-dependency |

## Testing

```bash
cargo test -p rspice-schematic-editor
```

Licensed under the [RSpice Personal Use License](../../LICENSE).
