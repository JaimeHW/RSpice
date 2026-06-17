# Symbol Editor Resolver Design

## Purpose

The symbol editor must become the authoritative public-face editor for a cell. A symbol view defines the geometry, label anchors, and pin locations that every parent schematic sees when it places that cell. The current branch stores symbol metadata, but placed instances, schematic rendering, snapping, netlisting, SVG export, and design checks still mostly use generated block symbols or symbol-local checks.

This design implements the resolver-first path approved for `codex/symbol-viewer`: keep `SymbolDocument` authoritative in the library view, resolve that document into a read-only placement context where needed, and move editor polish onto that same contract instead of maintaining a separate prototype surface.

## Requirements

- Symbol views remain in the Schematic workspace family and open from the library browser.
- Writable symbols edit body artwork, pins, origin, and label anchors; read-only libraries show the same amber gate and refuse every edit path with the same console message.
- Parent schematic instances use authored symbol documents for drawing, terminal snapping, hit testing, movement wire-following, netlist connectivity, placement preview, and SVG export.
- If no authored symbol document exists, the resolver falls back to the generated symbol derived from the schematic interface ports.
- Symbol terminals remain on the terminal grid. Body geometry uses a smaller body grid. Body ink remains stroke-only except arrows, dots, and port tips.
- Pin contract checks use the same structured design-check path as schematic ERC/DRC: result storage, console rows, anchors, and F4 navigation.
- Generate from schematic and port-change fixups are additive, undoable transactions. Port changes never silently regenerate over hand-drawn art.
- Preview tiles and placed schematics render the same resolved symbol model, with label substitution for instance name/value.
- Selection, clipboard, transform, shortcut, zoom, pan, and check commands follow the shared schematic grammar without modifier collisions.

## Architecture

### Authoritative Data

`SymbolDocument` remains stored in `View::metadata` under `SYMBOL_DOCUMENT_METADATA_KEY`. It stores:

- `pins`: canonical symbol pin records by name, direction, and optional local terminal position.
- `body`: stroke/fill-limited symbol geometry.
- `origin`: instance reference point.
- `name_anchor` and `value_anchor`: label locations in symbol coordinates.

The sibling schematic view remains the source of the cell interface when it exists. For generated fallback symbol views, the existing `generated` and `ports` metadata continue to provide the legacy contract until the symbol is authored.

### Resolver Layer

Add `crates/rspice-ui/src/state/symbol_resolver.rs` with focused, testable types:

- `SymbolResolver`: resolves a `LibraryCellInstance` or `CellViewRef` against `LibraryManager` and workspace schematic buffers.
- `ResolvedCellSymbol`: immutable placement-time view of body shapes, pins, origin, label anchors, and bounds.
- `ResolvedSymbolPin`: terminal-order-aware pin data with name, direction, optional authored placement status, and local offset.
- `ResolvedSymbolIssue`: non-UI contract issues such as unplaced pins, orphaned pins, and off-grid pins.

The resolver always returns pins in interface/terminal order for connectivity. Authored pin positions override generated positions only for matching schematic ports. Orphaned pins are kept visible in the symbol editor but excluded from placed-instance connectivity. Missing authored pins resolve to generated fallback offsets only for non-authored generated symbols; hand-authored symbols with unplaced pins produce check errors and do not pretend the pin is placed.

### Runtime Use

Schematic rendering, interaction, snapping, movement, netlist connectivity, and SVG export must consume resolved symbol data where a library context is available. Existing component fallback methods remain for primitives and legacy callers without context.

The central rule is: drawing terminals, electrical terminals, and exported terminals come from one resolved source. There must not be one geometry for display and another for connectivity.

### Wire Remapping

Because schematic wires are coordinate-based, saving a symbol that moves pins must remap placed-instance wire endpoints by pin identity.

On symbol store:

1. Resolve the old symbol for the active cell.
2. Store the new document.
3. Resolve the new symbol.
4. For every placed instance bound to that library/cell in the live schematic and workspace buffers, transform each old pin offset and new pin offset through that component's rotation/mirror/origin.
5. Move exact wire points that were attached to the old terminal coordinate onto the new terminal coordinate.
6. Bump topology/dirty state only for buffers that changed.

If a pin becomes unplaced, existing wire endpoints stay where they are and the structured design check reports the unplaced pin. The editor does not silently choose a new location.

### Design Checks

Extend the existing DRC transport instead of creating a separate symbol checker.

- Add symbol pin violation types to `DrcViolationType`: unplaced symbol pin, orphaned symbol pin, and off-grid symbol pin.
- Add a symbol-pin location to `DrcLocation` carrying `CellViewRef`, `pin_name`, and optional local point.
- Extend `LogAnchor` with a symbol target carrying the same information.
- Extract console jump behavior into an app-state helper so symbol anchors open the symbol view, select the pin, and center the symbol canvas when possible.
- Generalize F4 cycling so it can navigate both schematic and symbol anchors.

`Ctrl+E`, the Check menu, and toolbar checks should all call the same context-aware design-check entry point. If the active view is a symbol, run symbol pin checks. If the active view is schematic/testbench, run schematic DRC.

### Editor Surface

The editor remains in `shell/views/symbol.rs`, but geometry drawing that must be shared with schematic instances moves into a reusable renderer module.

Editor improvements:

- Guard local tool hotkeys so modified shortcuts do not also switch tools.
- Make generate-from-schematic one undoable transaction.
- Add a docbar pill fix action for "Place new pins" when the paired schematic adds ports.
- Keep orphan delete additive and undoable.
- Separate toolbar affordances for select, pin, polyline, circle, arc, arrow, and dot.
- Add dashed bounding-box rendering and body-grid polish.
- Render preview tile from the same resolved model as placed instances, with name/value substitution.
- Implement select-all, multi-selection, marquee selection, clipboard payloads, and transforms around `document.origin`.

Arc handles and segment-aware arrow placement should be implemented as editor geometry behavior, not as a separate rendering-only trick. The serialized document should stay stable and small.

## Data Flow

1. Library browser opens `lib/cell/symbol` into the Schematic workspace family.
2. The symbol editor loads `SymbolDocument` from metadata or generated fallback data.
3. Edits store `SymbolDocument` back into metadata and mark the symbol view dirty.
4. `SymbolResolver` resolves placed instances from library/cell/view references to `ResolvedCellSymbol`.
5. Schematic rendering, snapping, netlisting, and export consume the resolved symbol model.
6. Design checks convert symbol contract issues into structured DRC results and anchored console rows.

## Error Handling

- Invalid symbol metadata logs a warning and uses an empty symbol editor document for inspection; it does not crash the app.
- Read-only library edits are refused before mutation and use one consistent user-facing warning.
- Missing symbol views fall back to generated symbols from the interface where possible.
- Missing master schematic/interface data leaves legacy generated behavior in place when no stronger contract exists.
- Unplaced authored pins are surfaced as design-check errors instead of guessed.
- Wire remapping moves only exact terminal-attached points to avoid disturbing unrelated drawn wires.

## Testing Strategy

Use TDD for every behavior change.

Core tests:

- Resolver returns authored pin positions in terminal order.
- Resolver falls back to generated symbols when no authored document exists.
- Schematic terminals, snapping, and netlist connectivity follow authored symbol pin positions.
- Symbol save remaps existing instance wire endpoints by pin identity.
- SVG export includes authored symbol geometry and labels instead of a generic `X` block.
- Symbol pin findings become structured DRC violations with symbol anchors.
- Console symbol anchors open the symbol view, select the pin, and center the canvas.
- F4 cycles through symbol findings instead of no-oping in symbol views.
- Generate from schematic is undoable.
- Modified shortcuts such as Ctrl+C, Ctrl+A, and Ctrl+S do not switch symbol tools.
- Select-all, clipboard, transforms, and read-only refusal paths behave consistently.

Manual QA:

- Writable symbol edit to placed schematic instance.
- Read-only symbol inspection and copy-to-editable escape hatch.
- Port add/drop after symbol art exists.
- Every body tool via toolbar and keyboard.
- Preview tile compared against placed schematic instance.
- Exported SVG inspection for authored symbol body and label anchors.

## Scope Boundaries

This work completes the symbol editor/view contract described in `design/app/volta-symbol-editor.html`. It does not implement layout views, PDK technology binding, a full Cadence-compatible CDF/property system, or a full symbol standard-cell library. Those may build on this resolver later, but they are not required for this symbol editor milestone.
