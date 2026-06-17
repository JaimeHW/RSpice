# Symbol Editor Resolver Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make symbol views authoritative for placed cell instances, structured design checks, and the symbol editor surface described in `design/app/volta-symbol-editor.html`.

**Architecture:** Keep `SymbolDocument` as the master data stored on symbol view metadata, then add a resolver that produces immutable resolved symbols for schematic consumers. Drawing, snapping, netlisting, SVG export, checks, and editor preview all consume resolved symbol data so display geometry and electrical terminals cannot diverge.

**Tech Stack:** Rust, egui/eframe, serde JSON metadata, existing `LibraryManager`, `SchematicState`, `DrcResult`, `LogAnchor`, and `cargo test -p rspice-ui`.

---

## Scope Check

This plan replaces the earlier prototype plan at `docs/superpowers/plans/2026-06-16-symbol-viewer.md` without modifying it. The work is one integrated symbol-editor milestone, but the implementation must be sequential around the shared resolver model to avoid conflicting edits in central files.

## File Structure

- Create `crates/rspice-ui/src/state/symbol_resolver.rs`: symbol resolution, resolved pin/body model, contract issues, and resolver tests.
- Modify `crates/rspice-ui/src/state/mod.rs`: export resolver types.
- Modify `crates/rspice-ui/src/state/symbol.rs`: small helpers for bounds, authored/generated metadata, and stable pin issue conversion.
- Modify `crates/rspice-ui/src/state/schematic/component.rs`: expose owned terminal-position helpers that can use resolved symbol pins.
- Modify `crates/rspice-ui/src/schematic/view/drawing.rs`, `scene.rs`, `preview.rs`, and `mod.rs`: draw authored symbols and labels through shared resolved geometry.
- Modify `crates/rspice-ui/src/state/schematic/snap.rs` and schematic interaction call sites: snap to resolved authored terminals.
- Modify `crates/rspice-ui/src/simulation/netlist_gen/connectivity.rs`, `instances.rs`, and generator construction: seed nets and instance nodes from resolved terminals.
- Modify `crates/rspice-ui/src/schematic/export/*` and export menu call sites: export resolved authored symbol art instead of generic X blocks.
- Modify `crates/rspice-ui/src/services/drc/types.rs`, `common/menu_bar/tools_menu.rs`, `schematic/view/violations.rs`, `panels/log_panel.rs`, `shell/console.rs`, and `common/app/app_workspace_actions.rs`: structured symbol checks and navigation.
- Modify `crates/rspice-ui/src/shell/views/symbol.rs`, `shell/state.rs`, `shell/toolbar.rs`, and `common/app/app_actions.rs`: editor transactions, selection, shortcuts, fix-it actions, and preview polish.

## Task 1: Resolver Core

**Files:**
- Create: `crates/rspice-ui/src/state/symbol_resolver.rs`
- Modify: `crates/rspice-ui/src/state/mod.rs`
- Modify: `crates/rspice-ui/src/state/symbol.rs`

- [ ] **Step 1: Write failing resolver tests**

Add this test module to the new file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, LibraryManager, Point,
        PortDirection, SchematicState, SymbolDocument, SymbolPin, SymbolShape, View, ViewType,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> crate::state::PortSpec {
        crate::state::PortSpec { name: name.to_owned(), direction }
    }

    fn library_with_amp(symbol: Option<SymbolDocument>) -> (LibraryManager, HashMap<String, SchematicState>) {
        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        cell.add_view(View::new("schematic", ViewType::Schematic));
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        if let Some(document) = symbol {
            document.store_in_view(&mut symbol_view).expect("symbol stores");
        }
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut schematic = SchematicState::default();
        let in_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
        schematic.components.iter_mut().find(|c| c.id == in_id).unwrap().value = "IN".to_owned();
        let out_id = schematic.add_component(ComponentType::Port, Point::new(40, 0));
        schematic.components.iter_mut().find(|c| c.id == out_id).unwrap().value = "OUT".to_owned();
        let mut buffers = HashMap::new();
        buffers.insert(CellViewRef::new("work", "amp", "schematic").key(), schematic);
        (libraries, buffers)
    }

    #[test]
    fn authored_symbol_positions_override_generated_geometry_in_interface_order() {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            body: vec![SymbolShape::Polyline {
                points: vec![Point::new(-20, -20), Point::new(20, -20), Point::new(20, 20)],
                closed: false,
            }],
            ..SymbolDocument::default()
        };
        let (libraries, buffers) = library_with_amp(Some(document));
        let resolver = SymbolResolver::new(&libraries, &buffers);
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[port("IN", PortDirection::In), port("OUT", PortDirection::Out)]);

        let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");
        let pins: Vec<(&str, Point)> = resolved
            .connectable_pins()
            .map(|pin| (pin.name.as_str(), pin.offset))
            .collect();

        assert_eq!(pins, vec![("IN", Point::new(-40, -10)), ("OUT", Point::new(70, 20))]);
        assert!(resolved.issues().is_empty());
        assert!(matches!(resolved.source(), ResolvedSymbolSource::Authored));
    }

    #[test]
    fn resolver_falls_back_to_generated_symbol_when_no_authored_metadata_exists() {
        let (libraries, buffers) = library_with_amp(None);
        let resolver = SymbolResolver::new(&libraries, &buffers);
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[port("IN", PortDirection::In), port("OUT", PortDirection::Out)]);

        let resolved = resolver.resolve_binding(&binding).expect("fallback resolves");
        let pins: Vec<(&str, Point)> = resolved
            .connectable_pins()
            .map(|pin| (pin.name.as_str(), pin.offset))
            .collect();

        assert_eq!(pins.len(), 2);
        assert_eq!(pins[0].0, "IN");
        assert_eq!(pins[1].0, "OUT");
        assert!(matches!(resolved.source(), ResolvedSymbolSource::Generated));
    }

    #[test]
    fn authored_unplaced_pin_reports_issue_and_is_not_connectable() {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, None),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(30, 0))),
            ],
            ..SymbolDocument::default()
        };
        let (libraries, buffers) = library_with_amp(Some(document));
        let resolver = SymbolResolver::new(&libraries, &buffers);
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[port("IN", PortDirection::In), port("OUT", PortDirection::Out)]);

        let resolved = resolver.resolve_binding(&binding).expect("symbol resolves");
        let pins: Vec<&str> = resolved.connectable_pins().map(|pin| pin.name.as_str()).collect();

        assert_eq!(pins, vec!["OUT"]);
        assert!(resolved.issues().iter().any(|issue| {
            matches!(issue.kind, ResolvedSymbolIssueKind::UnplacedPin) && issue.pin_name == "IN"
        }));
    }
}
```

- [ ] **Step 2: Run tests and verify they fail**

Run:

```powershell
cargo test -p rspice-ui symbol_resolver -- --nocapture
```

Expected: compile failure because `symbol_resolver`, `SymbolResolver`, `ResolvedSymbolSource`, and `ResolvedSymbolIssueKind` do not exist.

- [ ] **Step 3: Implement resolver types and exports**

Create `symbol_resolver.rs` with these public types and methods:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedSymbolSource {
    Authored,
    Generated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedSymbolIssueKind {
    UnplacedPin,
    OrphanedPin,
    PinOffGrid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSymbolIssue {
    pub kind: ResolvedSymbolIssueKind,
    pub pin_name: String,
    pub point: Option<Point>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSymbolPin {
    pub name: String,
    pub direction: PortDirection,
    pub offset: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCellSymbol {
    source: ResolvedSymbolSource,
    document: SymbolDocument,
    pins: Vec<ResolvedSymbolPin>,
    issues: Vec<ResolvedSymbolIssue>,
}

impl ResolvedCellSymbol {
    pub fn source(&self) -> ResolvedSymbolSource { self.source }
    pub fn document(&self) -> &SymbolDocument { &self.document }
    pub fn issues(&self) -> &[ResolvedSymbolIssue] { &self.issues }
    pub fn connectable_pins(&self) -> impl Iterator<Item = &ResolvedSymbolPin> { self.pins.iter() }
}

pub struct SymbolResolver<'a> {
    libraries: &'a LibraryManager,
    schematic_buffers: &'a HashMap<String, SchematicState>,
}
```

Implement `SymbolResolver::new`, `resolve_binding`, `resolve_reference`, and internal helpers that:

- read the sibling schematic contract from `schematic_buffers`;
- fall back to `LibraryCellInstance::interface()` when available;
- load `SymbolDocument` from the `symbol` view metadata;
- generate `SymbolDocument::generated_from_ports(&ports)` when there is no authored metadata;
- keep connectable pins in port order;
- convert unplaced, orphaned, and off-grid findings into `ResolvedSymbolIssue`.

Export the module from `state/mod.rs`:

```rust
pub mod symbol_resolver;
pub use symbol_resolver::{
    ResolvedCellSymbol, ResolvedSymbolIssue, ResolvedSymbolIssueKind, ResolvedSymbolPin,
    ResolvedSymbolSource, SymbolResolver,
};
```

- [ ] **Step 4: Run resolver tests and full state tests**

Run:

```powershell
cargo test -p rspice-ui symbol_resolver -- --nocapture
cargo test -p rspice-ui state::symbol_document_tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 5: Commit**

```powershell
git add crates/rspice-ui/src/state/symbol_resolver.rs crates/rspice-ui/src/state/mod.rs crates/rspice-ui/src/state/symbol.rs
git commit -m "feat: resolve authored cell symbols"
```

## Task 2: Terminal Consumers Use Resolved Symbols

**Files:**
- Modify: `crates/rspice-ui/src/state/schematic/component.rs`
- Modify: `crates/rspice-ui/src/state/schematic/snap.rs`
- Modify: `crates/rspice-ui/src/schematic/view/interaction.rs`
- Modify: `crates/rspice-ui/src/schematic/view/drawing.rs`
- Modify: `crates/rspice-ui/src/schematic/view/scene.rs`
- Modify: `crates/rspice-ui/src/schematic/view/mod.rs`

- [ ] **Step 1: Write failing terminal-position tests**

Add tests to `component.rs`:

```rust
#[test]
fn resolved_instance_terminals_use_authored_symbol_offsets() {
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        super::port::PortSpec { name: "IN".to_owned(), direction: super::port::PortDirection::In },
        super::port::PortSpec { name: "OUT".to_owned(), direction: super::port::PortDirection::Out },
    ]);
    let component = Component::new(7, ComponentType::CellInstance, Point::new(100, 50))
        .with_library_cell(binding);
    let resolved = crate::state::ResolvedCellSymbol::for_test(
        vec![
            crate::state::ResolvedSymbolPin {
                name: "IN".to_owned(),
                direction: super::port::PortDirection::In,
                offset: Point::new(-40, -10),
            },
            crate::state::ResolvedSymbolPin {
                name: "OUT".to_owned(),
                direction: super::port::PortDirection::Out,
                offset: Point::new(70, 20),
            },
        ],
    );

    let terminals = component.terminal_positions_resolved(Some(&resolved));

    assert_eq!(
        terminals,
        vec![
            ("IN".to_owned(), Point::new(60, 40)),
            ("OUT".to_owned(), Point::new(170, 70)),
        ]
    );
}
```

- [ ] **Step 2: Run test and verify it fails**

Run:

```powershell
cargo test -p rspice-ui resolved_instance_terminals_use_authored_symbol_offsets -- --nocapture
```

Expected: compile failure because `terminal_positions_resolved` and `ResolvedCellSymbol::for_test` do not exist.

- [ ] **Step 3: Implement resolved terminal helpers**

Add these helpers:

```rust
impl Component {
    pub fn terminal_positions_resolved(
        &self,
        resolved_symbol: Option<&crate::state::ResolvedCellSymbol>,
    ) -> Vec<(String, Point)> {
        if self.kind == ComponentType::CellInstance {
            if let Some(symbol) = resolved_symbol {
                return symbol
                    .connectable_pins()
                    .map(|pin| {
                        let transformed = self.transform_point(pin.offset);
                        (pin.name.clone(), Point::new(self.pos.x + transformed.x, self.pos.y + transformed.y))
                    })
                    .collect();
            }
        }
        self.terminal_positions()
            .into_iter()
            .map(|(name, point)| (name.to_owned(), point))
            .collect()
    }
}
```

Add a `#[cfg(test)]` constructor to `ResolvedCellSymbol`:

```rust
#[cfg(test)]
impl ResolvedCellSymbol {
    pub fn for_test(pins: Vec<ResolvedSymbolPin>) -> Self {
        Self {
            source: ResolvedSymbolSource::Authored,
            document: SymbolDocument::default(),
            pins,
            issues: Vec::new(),
        }
    }
}
```

- [ ] **Step 4: Thread resolved terminals through schematic view**

Add a lightweight per-frame context:

```rust
pub struct SchematicSymbolContext {
    resolved: std::collections::HashMap<String, ResolvedCellSymbol>,
}

impl SchematicSymbolContext {
    pub fn resolve_component(&self, component: &Component) -> Option<&ResolvedCellSymbol> {
        let binding = component.library_cell.as_ref()?;
        self.resolved.get(&format!("{}/{}", binding.library, binding.cell))
    }
}
```

Build it from `AppState` before drawing and interaction. Update drawing and snap call sites to use `component.terminal_positions_resolved(context.resolve_component(component))` when context is available.

- [ ] **Step 5: Run terminal, snap, and schematic view tests**

```powershell
cargo test -p rspice-ui resolved_instance_terminals_use_authored_symbol_offsets -- --nocapture
cargo test -p rspice-ui state::schematic::snap::tests -- --nocapture
cargo test -p rspice-ui state::schematic::port::tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 6: Commit**

```powershell
git add crates/rspice-ui/src/state/schematic/component.rs crates/rspice-ui/src/state/schematic/snap.rs crates/rspice-ui/src/schematic/view/interaction.rs crates/rspice-ui/src/schematic/view/drawing.rs crates/rspice-ui/src/schematic/view/scene.rs crates/rspice-ui/src/schematic/view/mod.rs crates/rspice-ui/src/state/symbol_resolver.rs
git commit -m "feat: use resolved symbol terminals in schematics"
```

## Task 3: Netlist And Wire Remapping

**Files:**
- Modify: `crates/rspice-ui/src/simulation/netlist_gen/connectivity.rs`
- Modify: `crates/rspice-ui/src/simulation/netlist_gen/instances.rs`
- Modify: `crates/rspice-ui/src/simulation/netlist_gen/mod.rs`
- Modify: `crates/rspice-ui/src/common/app/app_workspace_actions.rs`

- [ ] **Step 1: Write failing netlist test**

Add a test in `simulation/netlist_gen/subcircuits.rs`:

```rust
#[test]
fn authored_symbol_pin_positions_define_cell_instance_connectivity() {
    let mut top = SchematicState::default();
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        PortSpec { name: "IN".to_owned(), direction: PortDirection::In },
        PortSpec { name: "OUT".to_owned(), direction: PortDirection::Out },
    ]);
    let id = top.add_library_cell_component(Point::new(100, 100), binding);
    top.components.iter_mut().find(|component| component.id == id).unwrap().name = "X1".to_owned();
    top.add_wire(vec![Point::new(60, 90), Point::new(0, 90)]);
    top.add_wire(vec![Point::new(170, 120), Point::new(220, 120)]);

    let mut amp = SchematicState::default();
    let in_id = amp.add_component(ComponentType::Port, Point::new(0, 0));
    amp.components.iter_mut().find(|component| component.id == in_id).unwrap().value = "IN".to_owned();
    let out_id = amp.add_component(ComponentType::Port, Point::new(40, 0));
    amp.components.iter_mut().find(|component| component.id == out_id).unwrap().value = "OUT".to_owned();

    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
        ],
        ..SymbolDocument::default()
    };

    let netlist = generate_netlist_with_symbol_documents(&top, &[("work", "amp", amp)], &[("work", "amp", document)])
        .expect("netlist generated");

    assert!(netlist.contains("X1 net"));
    assert!(netlist.contains(" amp"));
}
```

- [ ] **Step 2: Run test and verify it fails**

```powershell
cargo test -p rspice-ui authored_symbol_pin_positions_define_cell_instance_connectivity -- --nocapture
```

Expected: compile failure until the test helper and symbol-aware generator path exist.

- [ ] **Step 3: Make netlist generation accept a symbol context**

Add a symbol context field to `NetlistGenerator` and replace direct `component.terminal_positions()` reads in `connectivity.rs` and `instances.rs` with resolved terminal positions. Keep the existing constructor using an empty context so primitive and legacy tests remain unchanged.

The replacement pattern is:

```rust
let resolved = self
    .symbol_context
    .as_ref()
    .and_then(|context| context.resolve_component(component));
let terminals = component.terminal_positions_resolved(resolved);
```

- [ ] **Step 4: Write failing wire-remap test**

Add an app-level test in `app_workspace_actions.rs`:

```rust
#[test]
fn storing_symbol_move_remaps_instance_wire_endpoints_by_pin_name() {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let amp_ref = CellViewRef::new("work", "amp", "schematic");
    let mut amp_schematic = SchematicState::default();
    let in_id = amp_schematic.add_component(ComponentType::Port, Point::new(0, 0));
    amp_schematic.components.iter_mut().find(|component| component.id == in_id).unwrap().value = "IN".to_owned();
    state.workspace.schematic_buffers.insert(amp_ref.key(), amp_schematic);

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    let before = SymbolDocument {
        pins: vec![SymbolPin::new("IN", PortDirection::In, Some(Point::new(-30, 0)))],
        ..SymbolDocument::default()
    };
    state.store_active_symbol_document(&before).expect("initial symbol stores");

    state.open_workspace_view(CellViewRef::new("work", "top", "schematic"));
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[PortSpec { name: "IN".to_owned(), direction: PortDirection::In }]);
    let instance_id = state.schematic.add_library_cell_component(Point::new(100, 100), binding);
    state.schematic.components.iter_mut().find(|component| component.id == instance_id).unwrap().name = "X1".to_owned();
    state.schematic.add_wire(vec![Point::new(70, 100), Point::new(20, 100)]).expect("wire created");

    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    let after = SymbolDocument {
        pins: vec![SymbolPin::new("IN", PortDirection::In, Some(Point::new(-50, 0)))],
        ..SymbolDocument::default()
    };
    state.store_active_symbol_document(&after).expect("moved symbol stores");
    state.open_workspace_view(CellViewRef::new("work", "top", "schematic"));

    assert_eq!(state.schematic.wires[0].points[0], Point::new(50, 100));
    assert_eq!(state.schematic.wires[0].points[1], Point::new(20, 100));
}
```

- [ ] **Step 5: Implement wire remap on symbol store**

In `store_active_symbol_document`, capture the old resolved symbol before metadata mutation and the new resolved symbol after mutation. Add helper:

```rust
fn remap_instance_wires_for_symbol_change(
    schematic: &mut SchematicState,
    library: &str,
    cell: &str,
    before: &ResolvedCellSymbol,
    after: &ResolvedCellSymbol,
) -> bool
```

The helper maps pin names to offsets, transforms offsets through each matching component, and rewrites only wire points exactly equal to the old transformed terminal point.

- [ ] **Step 6: Run netlist and remap tests**

```powershell
cargo test -p rspice-ui authored_symbol_pin_positions_define_cell_instance_connectivity -- --nocapture
cargo test -p rspice-ui storing_symbol_move_remaps_instance_wire_endpoints_by_pin_name -- --nocapture
cargo test -p rspice-ui simulation::netlist_gen::subcircuits::tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-ui/src/simulation/netlist_gen/connectivity.rs crates/rspice-ui/src/simulation/netlist_gen/instances.rs crates/rspice-ui/src/simulation/netlist_gen/mod.rs crates/rspice-ui/src/common/app/app_workspace_actions.rs
git commit -m "feat: netlist authored symbol terminals"
```

## Task 4: Shared Rendering, Preview, And SVG Export

**Files:**
- Create: `crates/rspice-ui/src/schematic/view/resolved_symbol_render.rs`
- Modify: `crates/rspice-ui/src/schematic/view/drawing.rs`
- Modify: `crates/rspice-ui/src/schematic/view/preview.rs`
- Modify: `crates/rspice-ui/src/shell/views/symbol.rs`
- Modify: `crates/rspice-ui/src/schematic/export/mod.rs`
- Modify: `crates/rspice-ui/src/schematic/export/block_symbols.rs`
- Modify: `crates/rspice-ui/src/common/menu_bar/export_actions.rs`

- [ ] **Step 1: Write failing export test**

Add a test under `schematic/export/mod.rs`:

```rust
#[test]
fn svg_export_uses_authored_cell_symbol_body_and_labels() {
    let mut schematic = SchematicState::default();
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        PortSpec { name: "IN".to_owned(), direction: PortDirection::In },
        PortSpec { name: "OUT".to_owned(), direction: PortDirection::Out },
    ]);
    let id = schematic.add_library_cell_component(Point::new(100, 100), binding);
    schematic.components.iter_mut().find(|component| component.id == id).unwrap().name = "XAMP".to_owned();

    let resolved = ResolvedCellSymbol::for_test(vec![
        ResolvedSymbolPin { name: "IN".to_owned(), direction: PortDirection::In, offset: Point::new(-40, 0) },
        ResolvedSymbolPin { name: "OUT".to_owned(), direction: PortDirection::Out, offset: Point::new(40, 0) },
    ]);
    let svg = export_to_svg_with_resolved_symbols(&schematic, &ExportConfig::default(), &[("work/amp", resolved)])
        .expect("svg exports");

    assert!(svg.contains("XAMP"));
    assert!(!svg.contains(">X<"));
}
```

- [ ] **Step 2: Run test and verify it fails**

```powershell
cargo test -p rspice-ui svg_export_uses_authored_cell_symbol_body_and_labels -- --nocapture
```

Expected: compile failure until the symbol-aware export entry point exists.

- [ ] **Step 3: Extract shared renderer**

Move body/pin/label drawing logic out of `shell/views/symbol.rs` into `resolved_symbol_render.rs`. Provide functions for egui drawing and SVG serialization:

```rust
pub fn draw_resolved_symbol(
    painter: &egui::Painter,
    origin: egui::Pos2,
    scale: f32,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    stroke: egui::Stroke,
)

pub fn write_resolved_symbol_svg(
    svg: &mut String,
    cx: f64,
    cy: f64,
    component: &Component,
    symbol: &ResolvedCellSymbol,
    config: &ExportConfig,
)
```

- [ ] **Step 4: Use shared renderer in placed schematics and editor preview**

Update schematic drawing and placement preview to use the resolved symbol when available. Update the editor preview tile to resolve its own document through the same renderer and substitute `X1` for `@name` and the active cell name or value for `@value`.

- [ ] **Step 5: Run render/export tests and build**

```powershell
cargo test -p rspice-ui svg_export_uses_authored_cell_symbol_body_and_labels -- --nocapture
cargo test -p rspice-ui schematic::symbols::library::tests -- --nocapture
cargo build -p rspice-ui
```

Expected: selected tests pass and build succeeds.

- [ ] **Step 6: Commit**

```powershell
git add crates/rspice-ui/src/schematic/view/resolved_symbol_render.rs crates/rspice-ui/src/schematic/view/drawing.rs crates/rspice-ui/src/schematic/view/preview.rs crates/rspice-ui/src/shell/views/symbol.rs crates/rspice-ui/src/schematic/export/mod.rs crates/rspice-ui/src/schematic/export/block_symbols.rs crates/rspice-ui/src/common/menu_bar/export_actions.rs
git commit -m "feat: render and export authored symbols"
```

## Task 5: Structured Symbol Checks And Navigation

**Files:**
- Modify: `crates/rspice-ui/src/services/drc/types.rs`
- Modify: `crates/rspice-ui/src/panels/log_panel.rs`
- Modify: `crates/rspice-ui/src/shell/console.rs`
- Modify: `crates/rspice-ui/src/schematic/view/violations.rs`
- Modify: `crates/rspice-ui/src/common/menu_bar/tools_menu.rs`
- Modify: `crates/rspice-ui/src/common/app/app_workspace_actions.rs`
- Modify: `crates/rspice-ui/src/common/app/app_actions.rs`
- Modify: `crates/rspice-ui/src/shell/state.rs`

- [ ] **Step 1: Write failing symbol-check tests**

Add tests in `app_workspace_actions.rs`:

```rust
fn app_state_with_amp_symbol_pin(pin_name: &str, position: Option<Point>) -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    let port_id = schematic.add_component(ComponentType::Port, Point::new(0, 0));
    schematic.components.iter_mut().find(|component| component.id == port_id).unwrap().value = pin_name.to_owned();
    state.workspace.schematic_buffers.insert(CellViewRef::new("work", "amp", "schematic").key(), schematic);
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    let document = SymbolDocument {
        pins: vec![SymbolPin::new(pin_name, PortDirection::In, position)],
        ..SymbolDocument::default()
    };
    state.store_active_symbol_document(&document).expect("symbol document stores");
    state
}

#[test]
fn symbol_pin_checks_store_structured_drc_results_with_symbol_anchors() {
    let mut state = app_state_with_amp_symbol_pin("IN", None);
    state.run_active_symbol_pin_checks();

    let result = state.dialogs.drc_results.as_ref().expect("result stored");
    assert!(result.violations().iter().any(|violation| {
        violation.violation_type == DrcViolationType::SymbolUnplacedPin
            && matches!(violation.location, DrcLocation::SymbolPin { ref pin_name, .. } if pin_name == "IN")
    }));
    assert!(state.log_buffer.entries().any(|entry| matches!(entry.anchor, Some(LogAnchor::Symbol { .. }))));
}

#[test]
fn symbol_log_anchor_opens_symbol_view_and_selects_pin() {
    let mut state = app_state_with_amp_symbol_pin("IN", Some(Point::new(-30, 0)));
    let reference = CellViewRef::new("work", "amp", "symbol");

    state.jump_to_log_anchor(LogAnchor::Symbol {
        reference: reference.clone(),
        pin_name: "IN".to_owned(),
        point: Some(Point::new(-30, 0)),
    });

    assert_eq!(state.workspace.active_view, reference);
    assert_eq!(state.shell.symbol.selected_pin.as_deref(), Some("IN"));
    assert_eq!(state.shell.view, WorkspaceView::Schematic);
}
```

- [ ] **Step 2: Run tests and verify they fail**

```powershell
cargo test -p rspice-ui symbol_pin_checks_store_structured_drc_results_with_symbol_anchors -- --nocapture
cargo test -p rspice-ui symbol_log_anchor_opens_symbol_view_and_selects_pin -- --nocapture
```

Expected: compile failure until symbol DRC types and anchors exist.

- [ ] **Step 3: Extend DRC and log models**

Add variants:

```rust
pub enum DrcViolationType {
    SymbolUnplacedPin,
    SymbolOrphanedPin,
    SymbolPinOffGrid,
    // existing variants
}

pub enum DrcLocation {
    SymbolPin {
        reference: CellViewRef,
        pin_name: String,
        point: Option<Point>,
    },
    // existing variants
}

pub enum LogAnchor {
    Symbol {
        reference: CellViewRef,
        pin_name: String,
        point: Option<Point>,
    },
    Schematic { x: i32, y: i32, component: Option<u64>, wire: Option<u64> },
}
```

Give symbol pin violation types `DrcSeverity::Error`, stable descriptions, and stable suggested fixes.

- [ ] **Step 4: Implement structured symbol check runner and jumps**

Replace plain `ConsoleMessage` emission in `run_active_symbol_pin_checks` with a `DrcResult`. Use `LogSource::Drc`, the same row cap as schematic checks, and `LogAnchor::Symbol` rows. Add `AppState::jump_to_log_anchor` and have the console click handler call it for both schematic and symbol anchors.

- [ ] **Step 5: Generalize F4 cycling**

Move cycling logic to a function that maps each violation to a `LogAnchor`. For symbol anchors, open the symbol view and select the pin. Update symbol shortcut handling so `NextViolation` and `PrevViolation` call the same cycling function instead of returning `true` as a no-op.

- [ ] **Step 6: Run check/navigation tests**

```powershell
cargo test -p rspice-ui symbol_pin_checks_store_structured_drc_results_with_symbol_anchors -- --nocapture
cargo test -p rspice-ui symbol_log_anchor_opens_symbol_view_and_selects_pin -- --nocapture
cargo test -p rspice-ui services::drc::checker::tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-ui/src/services/drc/types.rs crates/rspice-ui/src/panels/log_panel.rs crates/rspice-ui/src/shell/console.rs crates/rspice-ui/src/schematic/view/violations.rs crates/rspice-ui/src/common/menu_bar/tools_menu.rs crates/rspice-ui/src/common/app/app_workspace_actions.rs crates/rspice-ui/src/common/app/app_actions.rs crates/rspice-ui/src/shell/state.rs
git commit -m "feat: integrate symbol checks with drc navigation"
```

## Task 6: Editor Transactions, Shortcuts, And Fix-It Flow

**Files:**
- Modify: `crates/rspice-ui/src/shell/views/symbol.rs`
- Modify: `crates/rspice-ui/src/common/app/app_workspace_actions.rs`
- Modify: `crates/rspice-ui/src/common/app/app_actions.rs`
- Modify: `crates/rspice-ui/src/shell/state.rs`
- Modify: `crates/rspice-ui/src/shell/toolbar.rs`

- [ ] **Step 1: Write failing action tests**

Add tests in `app_workspace_actions.rs`:

```rust
fn app_state_with_amp_ports<const N: usize>(names: [&str; N]) -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);

    let mut schematic = SchematicState::default();
    for (index, name) in names.iter().enumerate() {
        let port_id = schematic.add_component(ComponentType::Port, Point::new(index as i32 * 20, 0));
        schematic.components.iter_mut().find(|component| component.id == port_id).unwrap().value = (*name).to_owned();
    }
    state.workspace.schematic_buffers.insert(CellViewRef::new("work", "amp", "schematic").key(), schematic);
    state
}

fn read_only_symbol_state() -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("readonly");
    library.read_only = true;
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    state.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("readonly", "amp", "symbol"));
    state
}

#[test]
fn generate_symbol_document_is_one_undoable_transaction() {
    let mut state = app_state_with_amp_ports(["IN", "OUT"]);
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));

    state.generate_active_symbol_document().expect("generated");
    assert!(state.undo_active_symbol_document().expect("undo works"));

    let document = state.load_active_symbol_document().expect("document loads");
    assert!(document.body.is_empty());
}

#[test]
fn read_only_symbol_edit_paths_use_consistent_refusal_text() {
    let mut state = read_only_symbol_state();
    let error = state.store_active_symbol_document(&SymbolDocument::default()).unwrap_err();
    assert_eq!(error, "Read-only - 'readonly' masters cannot be edited");
}
```

Add a shortcut unit test in `app_actions.rs` or the shortcut resolver test module:

```rust
#[test]
fn modified_shortcuts_do_not_switch_symbol_tools() {
    let mut app = RSpiceApp::default();
    app.state.workspace.open_view(CellViewRef::new("work", "amp", "symbol"), ViewType::Symbol);
    app.state.shell.symbol.tool = SymbolTool::Select;

    let handled = app.execute_symbol_shortcut_command(ShortcutCommand::EditCopy);

    assert!(handled);
    assert_eq!(app.state.shell.symbol.tool, SymbolTool::Select);
}
```

- [ ] **Step 2: Run tests and verify they fail**

```powershell
cargo test -p rspice-ui generate_symbol_document_is_one_undoable_transaction -- --nocapture
cargo test -p rspice-ui read_only_symbol_edit_paths_use_consistent_refusal_text -- --nocapture
```

Expected: at least the undo test fails because generation currently stores without recording undo.

- [ ] **Step 3: Implement transaction fixes**

Change `generate_active_symbol_document` to load the current document, record it once, then store the generated document. Use one helper for read-only message text:

```rust
pub(crate) fn read_only_master_message(&self) -> String {
    format!("Read-only - '{}' masters cannot be edited", self.workspace.active_view.library)
}
```

Use the helper in store, undo, redo, deny edit, and UI banner.

- [ ] **Step 4: Implement Place New Pins fix action**

Change `pin_pill` to return an action when there are unplaced pins. Add a button labeled `Place new pins` beside the pill. On click, set `SymbolTool::PlacePin`, select `next_unplaced_pin(document)`, and do not regenerate body art.

- [ ] **Step 5: Fix shortcut ownership and toolbar affordances**

Guard raw local key handling with:

```rust
let plain = !input.modifiers.alt && !input.modifiers.ctrl && !input.modifiers.command && !input.modifiers.shift;
if !plain {
    return;
}
```

Add separate toolbar buttons for Circle, Arc, Arrow, and Dot. Keep tooltips naming the exact key.

- [ ] **Step 6: Run action and UI state tests**

```powershell
cargo test -p rspice-ui generate_symbol_document_is_one_undoable_transaction -- --nocapture
cargo test -p rspice-ui read_only_symbol_edit_paths_use_consistent_refusal_text -- --nocapture
cargo test -p rspice-ui common::app::app_workspace_actions::tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-ui/src/shell/views/symbol.rs crates/rspice-ui/src/common/app/app_workspace_actions.rs crates/rspice-ui/src/common/app/app_actions.rs crates/rspice-ui/src/shell/state.rs crates/rspice-ui/src/shell/toolbar.rs
git commit -m "feat: polish symbol editor transactions"
```

## Task 7: Selection, Clipboard, Geometry Polish

**Files:**
- Modify: `crates/rspice-ui/src/shell/state.rs`
- Modify: `crates/rspice-ui/src/shell/views/symbol.rs`
- Modify: `crates/rspice-ui/src/common/app/app_actions.rs`

- [ ] **Step 1: Write failing selection tests**

Add pure state tests for a new `SymbolSelection` helper:

```rust
#[test]
fn select_all_symbol_items_selects_pins_and_shapes() {
    let document = SymbolDocument {
        pins: vec![SymbolPin::new("IN", PortDirection::In, Some(Point::new(-30, 0)))],
        body: vec![SymbolShape::Dot { center: Point::origin(), radius: 2 }],
        ..SymbolDocument::default()
    };
    let selection = SymbolSelection::all_in(&document);
    assert!(selection.pins.contains("IN"));
    assert!(selection.shapes.contains(&0));
}

#[test]
fn symbol_transforms_are_about_document_origin() {
    let origin = Point::new(10, 10);
    let point = rotate_point_cw_about(Point::new(20, 10), origin);
    assert_eq!(point, Point::new(10, 20));
}
```

- [ ] **Step 2: Run tests and verify they fail**

```powershell
cargo test -p rspice-ui select_all_symbol_items_selects_pins_and_shapes -- --nocapture
cargo test -p rspice-ui symbol_transforms_are_about_document_origin -- --nocapture
```

Expected: compile failure until selection helper and origin transform helper exist.

- [ ] **Step 3: Implement multi-selection model**

Add to `shell/state.rs`:

```rust
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolSelection {
    pub pins: std::collections::BTreeSet<String>,
    pub shapes: std::collections::BTreeSet<usize>,
}
```

Replace single selected shape/pin paths gradually by keeping the existing fields as compatibility mirrors until the full UI path uses `SymbolSelection`.

- [ ] **Step 4: Implement select-all, marquee, clipboard, and origin transforms**

Implement:

- `SymbolSelection::all_in(&SymbolDocument)`;
- marquee rectangle hit tests for pins and shape bounds;
- clipboard payload with selected shapes and selected non-contract pins;
- paste at cursor or symbol view center;
- rotate/mirror selected pins and shapes around `document.origin`.

- [ ] **Step 5: Polish grid, bbox, dot, arc, and arrow behavior**

Use the existing `SYMBOL_TERMINAL_GRID` and set body snap to a quarter terminal grid. Render the bbox as dashed. Make dot radius match the design intent at sheet scale. Add arc start/sweep handles and arrow placement along a clicked segment with a flip transform.

- [ ] **Step 6: Run selection and action tests**

```powershell
cargo test -p rspice-ui select_all_symbol_items_selects_pins_and_shapes -- --nocapture
cargo test -p rspice-ui symbol_transforms_are_about_document_origin -- --nocapture
cargo test -p rspice-ui state::symbol_document_tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-ui/src/shell/state.rs crates/rspice-ui/src/shell/views/symbol.rs crates/rspice-ui/src/common/app/app_actions.rs
git commit -m "feat: complete symbol editor selection tools"
```

## Task 8: Verification, Visual QA, And Final Review

**Files:**
- Modify only if failures require fixes in files already touched by Tasks 1-7.

- [ ] **Step 1: Run format and full tests**

```powershell
cargo fmt
cargo test -p rspice-ui
cargo build -p rspice-ui
```

Expected: format succeeds, all `rspice-ui` tests pass, and the UI crate builds.

- [ ] **Step 2: Run manual GUI smoke test**

Launch:

```powershell
cargo run -p rspice-ui
```

Manual checks:

- open a writable symbol view;
- generate from schematic, undo, redo;
- move a pin and confirm a placed parent instance terminal follows;
- run Ctrl+E in symbol view and click an anchored console row;
- export SVG for a parent schematic containing the authored symbol;
- open a read-only symbol and verify edit refusal copy;
- exercise select-all, copy, paste, rotate, mirror, marquee, and each body tool.

- [ ] **Step 3: Capture visual evidence**

Save screenshots under `diagnostics/` with names:

```text
diagnostics/symbol-editor-authored-instance.png
diagnostics/symbol-editor-structured-checks.png
diagnostics/symbol-editor-preview-and-tools.png
```

- [ ] **Step 4: Dispatch final subagent code review**

Ask a reviewer subagent to review the full implementation against:

- `design/app/volta-symbol-editor.html`;
- `docs/superpowers/specs/2026-06-17-symbol-editor-resolver-design.md`;
- this implementation plan;
- the diff since commit `1e26c22a`.

- [ ] **Step 5: Fix review findings and rerun verification**

For every Critical or Important finding, add or update a failing test first, implement the fix, and rerun:

```powershell
cargo test -p rspice-ui
cargo build -p rspice-ui
```

- [ ] **Step 6: Commit verification fixes**

```powershell
git add crates/rspice-ui docs/superpowers diagnostics
git commit -m "test: verify symbol editor resolver"
```

Use exact paths instead of broad directories if unrelated dirty files are present.
