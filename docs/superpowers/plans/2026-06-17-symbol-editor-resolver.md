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

- [x] **Step 1: Write failing resolver tests**

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

- [x] **Step 2: Run tests and verify they fail**

Run:

```powershell
cargo test -p rspice-ui symbol_resolver -- --nocapture
```

Expected: compile failure because `symbol_resolver`, `SymbolResolver`, `ResolvedSymbolSource`, and `ResolvedSymbolIssueKind` do not exist.

- [x] **Step 3: Implement resolver types and exports**

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

- use `LibraryCellInstance::interface()` first for placed bindings so displayed, snapped, and netlisted terminals stay aligned with the instance's saved electrical contract;
- read the sibling schematic contract from `schematic_buffers` first for master/reference resolution, and as fallback for legacy bindings without a complete saved interface;
- load `SymbolDocument` from the `symbol` view metadata;
- generate `SymbolDocument::generated_from_ports(&ports)` when there is no authored metadata;
- fall back to generated geometry with a resolver issue when authored symbol metadata is invalid;
- keep connectable pins in port order;
- convert unplaced, orphaned, off-grid, and invalid-metadata findings into `ResolvedSymbolIssue`.

Export the module from `state/mod.rs`:

```rust
pub mod symbol_resolver;
pub use symbol_resolver::{
    ResolvedCellSymbol, ResolvedSymbolIssue, ResolvedSymbolIssueKind, ResolvedSymbolPin,
    ResolvedSymbolSource, SymbolResolver,
};
```

- [x] **Step 4: Run resolver tests and full state tests**

Run:

```powershell
cargo test -p rspice-ui symbol_resolver -- --nocapture
cargo test -p rspice-ui state::symbol_document_tests -- --nocapture
```

Expected: all selected tests pass.

Evidence (2026-06-17): `cargo test -p rspice-ui state::symbol_resolver --lib -- --nocapture` passed 5/5 resolver tests, and `cargo test -p rspice-ui state::symbol_document_tests --lib -- --nocapture` passed 6/6 symbol document tests.

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

- [x] **Step 1: Write failing terminal-position tests**

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

- [x] **Step 2: Run test and verify it fails**

Run:

```powershell
cargo test -p rspice-ui resolved_instance_terminals_use_authored_symbol_offsets -- --nocapture
```

Expected: compile failure because `terminal_positions_resolved` and `ResolvedCellSymbol::for_test` do not exist.

- [x] **Step 3: Implement resolved terminal helpers**

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

- [x] **Step 4: Thread resolved terminals through schematic view**

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

- [x] **Step 5: Run terminal, snap, and schematic view tests**

```powershell
cargo test -p rspice-ui resolved_instance_terminals_use_authored_symbol_offsets -- --nocapture
cargo test -p rspice-ui state::schematic::snap::tests -- --nocapture
cargo test -p rspice-ui state::schematic::port::tests -- --nocapture
```

Expected: all selected tests pass.

Evidence (2026-06-17): `cargo test -p rspice-ui resolved_instance_terminal --lib -- --nocapture` passed 2/2 tests, including authored terminal offsets and snap behavior. `cargo test -p rspice-ui state::schematic::snap::tests --lib -- --nocapture` passed 28/28, and `cargo test -p rspice-ui state::schematic::port::tests --lib -- --nocapture` passed 5/5.

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

- [x] **Step 1: Write failing netlist test**

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

- [x] **Step 2: Run test and verify it fails**

```powershell
cargo test -p rspice-ui authored_symbol_pin_positions_define_cell_instance_connectivity -- --nocapture
```

Expected: compile failure until the test helper and symbol-aware generator path exist.

- [x] **Step 3: Make netlist generation accept a symbol context**

Add a symbol context field to `NetlistGenerator` and replace direct `component.terminal_positions()` reads in `connectivity.rs` and `instances.rs` with resolved terminal positions. Keep the existing constructor using an empty context so primitive and legacy tests remain unchanged.

The replacement pattern is:

```rust
let resolved = self
    .symbol_context
    .as_ref()
    .and_then(|context| context.resolve_component(component));
let terminals = component.terminal_positions_resolved(resolved);
```

- [x] **Step 4: Write failing wire-remap test**

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

- [x] **Step 5: Implement wire remap on symbol store**

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

- [x] **Step 6: Run netlist and remap tests**

```powershell
cargo test -p rspice-ui authored_symbol_pin_positions_define_cell_instance_connectivity -- --nocapture
cargo test -p rspice-ui storing_symbol_move_remaps_instance_wire_endpoints_by_pin_name -- --nocapture
cargo test -p rspice-ui simulation::netlist_gen::subcircuits::tests -- --nocapture
```

Expected: all selected tests pass.

Evidence (2026-06-17): `cargo test -p rspice-ui simulation::netlist_gen::subcircuits --lib -- --nocapture` passed 11/11, including `authored_symbol_pin_positions_define_cell_instance_connectivity`. The plan's old exact remap filter no longer matches a test name, so the current remap suite was verified with `cargo test -p rspice-ui remap --lib -- --nocapture`, passing 5/5 tests covering open instance wires by pin name, rotated/mirrored instances, one-shot remaps, all open parent buffers, and selected-cell remap-on-rotate.

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

- [x] **Step 1: Write failing export test**

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

- [x] **Step 2: Run test and verify it fails**

```powershell
cargo test -p rspice-ui svg_export_uses_authored_cell_symbol_body_and_labels -- --nocapture
```

Expected: compile failure until the symbol-aware export entry point exists.

- [x] **Step 3: Extract shared renderer**

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

- [x] **Step 4: Use shared renderer in placed schematics and editor preview**

Update schematic drawing and placement preview to use the resolved symbol when available. Update the editor preview tile to resolve its own document through the same renderer and substitute `X1` for `@name` and the active cell name or value for `@value`.

- [x] **Step 5: Run render/export tests and build**

```powershell
cargo test -p rspice-ui svg_export_uses_authored_cell_symbol_body_and_labels -- --nocapture
cargo test -p rspice-ui schematic::symbols::library::tests -- --nocapture
cargo build -p rspice-ui
```

Expected: selected tests pass and build succeeds.

Evidence (2026-06-17): `cargo test -p rspice-ui svg_export_uses_authored_cell_symbol_body_and_labels --lib -- --nocapture` passed 1/1, `cargo test -p rspice-ui schematic::symbols::library::tests --lib -- --nocapture` passed 2/2, and `cargo build -p rspice-ui` completed successfully.

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

- [x] **Step 1: Write failing symbol-check tests**

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

- [x] **Step 2: Run tests and verify they fail**

```powershell
cargo test -p rspice-ui symbol_pin_checks_store_structured_drc_results_with_symbol_anchors -- --nocapture
cargo test -p rspice-ui symbol_log_anchor_opens_symbol_view_and_selects_pin -- --nocapture
```

Expected: compile failure until symbol DRC types and anchors exist.

- [x] **Step 3: Extend DRC and log models**

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

- [x] **Step 4: Implement structured symbol check runner and jumps**

Replace plain `ConsoleMessage` emission in `run_active_symbol_pin_checks` with a `DrcResult`. Use `LogSource::Drc`, the same row cap as schematic checks, and `LogAnchor::Symbol` rows. Add `AppState::jump_to_log_anchor` and have the console click handler call it for both schematic and symbol anchors.

- [x] **Step 5: Generalize F4 cycling**

Move cycling logic to a function that maps each violation to a `LogAnchor`. For symbol anchors, open the symbol view and select the pin. Update symbol shortcut handling so `NextViolation` and `PrevViolation` call the same cycling function instead of returning `true` as a no-op.

- [x] **Step 6: Run check/navigation tests**

```powershell
cargo test -p rspice-ui symbol_pin_checks_store_structured_drc_results_with_symbol_anchors -- --nocapture
cargo test -p rspice-ui symbol_log_anchor_opens_symbol_view_and_selects_pin -- --nocapture
cargo test -p rspice-ui services::drc::checker::tests -- --nocapture
```

Expected: all selected tests pass.

Evidence (2026-06-17): `cargo test -p rspice-ui symbol_pin_checks --lib -- --nocapture` passed 2/2, `cargo test -p rspice-ui symbol_log_anchor_opens_symbol_view_and_selects_pin --lib -- --nocapture` passed 1/1, `cargo test -p rspice-ui symbol_violation --lib -- --nocapture` passed 1/1, and `cargo test -p rspice-ui services::drc::checker::tests --lib -- --nocapture` passed 21/21.

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

- [x] **Step 1: Write failing action tests**

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

- [x] **Step 2: Run tests and verify they fail**

```powershell
cargo test -p rspice-ui generate_symbol_document_is_one_undoable_transaction -- --nocapture
cargo test -p rspice-ui read_only_symbol_edit_paths_use_consistent_refusal_text -- --nocapture
```

Expected: at least the undo test fails because generation currently stores without recording undo.

- [x] **Step 3: Implement transaction fixes**

Change `generate_active_symbol_document` to load the current document, record it once, then store the generated document. Use one helper for read-only message text:

```rust
pub(crate) fn read_only_master_message(&self) -> String {
    format!("Read-only - '{}' masters cannot be edited", self.workspace.active_view.library)
}
```

Use the helper in store, undo, redo, deny edit, and UI banner.

- [x] **Step 4: Implement Place New Pins fix action**

Change `pin_pill` to return an action when there are unplaced pins. Add a button labeled `Place new pins` beside the pill. On click, set `SymbolTool::PlacePin`, select `next_unplaced_pin(document)`, and do not regenerate body art.

- [x] **Step 5: Fix shortcut ownership and toolbar affordances**

Guard raw local key handling with:

```rust
let plain = !input.modifiers.alt && !input.modifiers.ctrl && !input.modifiers.command && !input.modifiers.shift;
if !plain {
    return;
}
```

Add separate toolbar buttons for Circle, Arc, Arrow, and Dot. Keep tooltips naming the exact key.

- [x] **Step 6: Run action and UI state tests**

```powershell
cargo test -p rspice-ui generate_symbol_document_is_one_undoable_transaction -- --nocapture
cargo test -p rspice-ui read_only_symbol_edit_paths_use_consistent_refusal_text -- --nocapture
cargo test -p rspice-ui common::app::app_workspace_actions::tests -- --nocapture
```

Expected: all selected tests pass.

Evidence (2026-06-17): `cargo test -p rspice-ui generate_symbol_document_is_one_undoable_transaction --lib -- --nocapture` passed 1/1, `cargo test -p rspice-ui read_only_symbol_edit_paths_use_consistent_refusal_text --lib -- --nocapture` passed 1/1, `cargo test -p rspice-ui common::app::app_workspace_actions::tests --lib -- --nocapture` passed 20/20, `cargo test -p rspice-ui symbol_action_tests --lib -- --nocapture` passed 2/2, and `cargo test -p rspice-ui shell::views::symbol::tests --lib -- --nocapture` passed 7/7.

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

- [x] **Step 1: Write failing selection tests**

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

- [x] **Step 2: Run tests and verify they fail**

```powershell
cargo test -p rspice-ui select_all_symbol_items_selects_pins_and_shapes -- --nocapture
cargo test -p rspice-ui symbol_transforms_are_about_document_origin -- --nocapture
```

Expected: compile failure until selection helper and origin transform helper exist.

- [x] **Step 3: Implement multi-selection model**

Add to `shell/state.rs`:

```rust
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolSelection {
    pub pins: std::collections::BTreeSet<String>,
    pub shapes: std::collections::BTreeSet<usize>,
}
```

Replace single selected shape/pin paths gradually by keeping the existing fields as compatibility mirrors until the full UI path uses `SymbolSelection`.

- [x] **Step 4: Implement select-all, marquee, clipboard, and origin transforms**

Implement:

- `SymbolSelection::all_in(&SymbolDocument)`;
- marquee rectangle hit tests for pins and shape bounds;
- clipboard payload with selected shapes and selected non-contract pins;
- paste at cursor or symbol view center;
- rotate/mirror selected pins and shapes around `document.origin`.

- [x] **Step 5: Polish grid, bbox, dot, arc, and arrow behavior**

Use the existing `SYMBOL_TERMINAL_GRID` and set body snap to a quarter terminal grid. Render the bbox as dashed. Make dot radius match the design intent at sheet scale. Add arc start/sweep handles and arrow placement along a clicked segment with a flip transform.

- [x] **Step 6: Run selection and action tests**

```powershell
cargo test -p rspice-ui select_all_symbol_items_selects_pins_and_shapes -- --nocapture
cargo test -p rspice-ui symbol_transforms_are_about_document_origin -- --nocapture
cargo test -p rspice-ui state::symbol_document_tests -- --nocapture
```

Expected: all selected tests pass.

Evidence (2026-06-17): `cargo test -p rspice-ui select_all_symbol_items_selects_pins_and_shapes --lib -- --nocapture` passed 1/1, `cargo test -p rspice-ui symbol_transforms_are_about_document_origin --lib -- --nocapture` passed 1/1, `cargo test -p rspice-ui symbol_selection_tests --lib -- --nocapture` passed 2/2, and `cargo test -p rspice-ui state::symbol_document_tests --lib -- --nocapture` passed 6/6.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-ui/src/shell/state.rs crates/rspice-ui/src/shell/views/symbol.rs crates/rspice-ui/src/common/app/app_actions.rs
git commit -m "feat: complete symbol editor selection tools"
```

## Task 8: Verification, Visual QA, And Final Review

**Files:**
- Modify only if failures require fixes in files already touched by Tasks 1-7.

- [x] **Step 1: Run format and full tests**

```powershell
cargo fmt
cargo test -p rspice-ui
cargo build -p rspice-ui
```

Expected: format succeeds, all `rspice-ui` tests pass, and the UI crate builds.

Evidence (2026-06-17): `cargo fmt --all -- --check` passed, `cargo test -p rspice-ui` passed with 269 tests, `cargo build -p rspice-ui` passed, and `cargo check -p rspice-ui --target wasm32-unknown-unknown` passed.

Additional evidence after final review fixes (2026-06-17): `cargo test -p rspice-ui schematic::view --lib -- --nocapture` passed with 14 tests, `cargo test -p rspice-ui common::app::app_workspace_actions::tests::storing_symbol_document --lib -- --nocapture` passed with 5 tests, `cargo test -p rspice-ui state::schematic::component --lib -- --nocapture` passed with 3 tests, `cargo test -p rspice-ui state::symbol_resolver --lib -- --nocapture` passed with 5 tests, `cargo test -p rspice-ui --lib` passed with 280 tests, `cargo fmt --all -- --check` passed, `cargo check -p rspice-ui` passed, `cargo check -p rspice-ui --target wasm32-unknown-unknown` passed, `cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings` passed, and `cargo test -p rspice-ui` passed with 280 tests.

Additional responsive chrome evidence (2026-06-17): after browser QA exposed phone-width clipping, responsive menu/toolbar/tab presentations, phone side-panel suppression, bounded compact project labels, compact overflow history-state parity, breakpoint guardrails, and mobile empty-state text were added with TDD coverage. Post-review verification passed: `cargo fmt --all -- --check`, `cargo check -p rspice-ui`, `cargo check -p rspice-ui --target wasm32-unknown-unknown`, `cargo test -p rspice-ui` with 294 tests, `cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings`, `cargo build -p rspice-ui --target wasm32-unknown-unknown --release`, and `wasm-bindgen target/wasm32-unknown-unknown/release/rspice-ui.wasm --out-dir crates/rspice-ui/web/pkg --target web --no-typescript --out-name rspice-ui`.

- [x] **Step 2: Run manual GUI smoke test**

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

Partial browser smoke evidence (2026-06-17): built the browser UI with `cargo build -p rspice-ui --target wasm32-unknown-unknown --release`, generated `crates/rspice-ui/web/pkg` via `wasm-bindgen`, served `crates/rspice-ui/web` at `http://127.0.0.1:8787/`, and verified in the in-app browser that the IDE loaded to `top -- Untitled Project -- RSpice` with a 1280x720 canvas, no loading overlay, and no console warning/error entries. Screenshot saved as `diagnostics/rspice-ui-browser-smoke.png`. A 390x844 mobile viewport initially loaded without console warnings/errors or DOM overflow, but showed cramped/clipped desktop-style toolbar and tab-strip chrome. Follow-up responsive polish added compact menu/toolbar/tab presentations, phone-width side-panel suppression, shorter mobile empty-state guidance, bounded compact project labels, compact overflow history-state parity, and label-aware tab breakpoints. After rebuilding wasm and regenerating `crates/rspice-ui/web/pkg`, the in-app browser verified 390x844, 900x720, and 1280x720 loads with no visible loading overlay, no horizontal DOM overflow, and no warning/error console entries. Screenshots saved as `diagnostics/rspice-ui-browser-mobile-smoke-review-final.png`, `diagnostics/rspice-ui-browser-midwidth-smoke-review-final.png`, and `diagnostics/rspice-ui-browser-desktop-smoke-review-final.png`. This does not complete the full manual checklist above.

Additional browser symbol smoke evidence (2026-06-17): from the running browser build, loaded File -> Open example -> Hierarchical RC Filter, opened Library, selected `user / rc_filter_core / symbol`, and verified the authored symbol editor rendered with toolbar tools, symbol canvas, pin contract rail, `PINS match schematic`, and the "AS PLACED" preview tile. Ran `Ctrl+E` in the symbol view and confirmed the console emitted `Symbol pins match schematic`. Clicked the Circle body tool and verified the tool echo changed while the preview and pin rail remained stable. Browser console warning/error count remained 0. This still does not complete the full native/manual checklist above.

Native launch attempt (2026-06-17): launched `target/debug/rspice-ui.exe` from the active worktree after approval. Windows reported the `rspice-ui` process alive/responding, but `MainWindowHandle` remained `0` and `EnumWindows` found no visible top-level RSpice window for the process, so no native desktop UI surface was available to inspect or capture. The process was cleaned up with `Stop-Process`. This keeps the native manual smoke item open; the inspectable evidence for this pass is the browser egui canvas smoke above.

Native startup/logging polish follow-up (2026-06-17): rebuilt `target/debug/rspice-ui.exe` and relaunched it with stdout/stderr redirected under the current shell, which has `RUST_LOG=warn`. The reproduced native window reported title `hierarchical_rc_filter_tb* — Hierarchical RC Filter — RSpice`, `MainWindowHandle=17041064`, and `Responding=True`; the process was then cleaned up. The first captured launch showed that generic `RUST_LOG=warn` leaked `wgpu_hal::vulkan` backend probe warnings, so native GUI logging was moved to quiet defaults plus the app-specific `RSPICE_LOG` override. Focused verification passed with `cargo test -p rspice-ui common::logging --lib -- --nocapture` (2/2), `cargo build -p rspice-ui`, and the final captured launch produced zero-byte stdout/stderr logs at `diagnostics/native-rspice-ui-20260617-192632.stdout.log` and `diagnostics/native-rspice-ui-20260617-192632.stderr.log`. This proves native startup now creates a real desktop window without routine console noise, but still does not complete the full manual symbol-edit checklist above.

Native desktop symbol smoke evidence (2026-06-17): captured real desktop window screenshots after launching the rebuilt native executable. `diagnostics/native-rspice-ui-window.png` shows a nonblank hierarchical schematic workspace; `diagnostics/native-rspice-ui-library-tab.png` shows the native placement Library rail; `diagnostics/native-rspice-ui-library-manager.png` shows the three-column Library Manager; `diagnostics/native-rspice-ui-symbol-editor-enter.png` shows `user / rc_filter_core / symbol` opened through Library Manager selection plus Enter with the authored symbol canvas, pin-contract rail, and AS PLACED preview; and `diagnostics/native-rspice-ui-symbol-ctrl-e.png` shows the native symbol editor after `Ctrl+E`, including the console row `Symbol pins match schematic`. A later logging review found that the quiet default over-filtered unlisted dependency warnings; after adding a global `warn` baseline plus explicit `wgpu_core`/`wgpu_hal`/`naga` error-level overrides, `cargo test -p rspice-ui common::logging --lib -- --nocapture` passed 2/2 and a fresh redirected native launch produced a responding window (`MainWindowHandle=21563608`) with zero-byte stdout/stderr logs at `diagnostics/native-rspice-ui-20260617-193835.stdout.log` and `diagnostics/native-rspice-ui-20260617-193835.stderr.log`. The remaining manual-symbol smoke gaps at this point were the gesture-heavy Generate/Undo/Redo, pin-drag remap, export, read-only refusal, and selection/body-tool matrix checks.

Native symbol tool/edit matrix evidence (2026-06-17): in a disposable native session, opened `user / rc_filter_core / symbol`, exercised symbol tool shortcuts `S`, `P`, `W`, `C`, `A`, `D`, and `O`, and captured the resulting tool states as `diagnostics/native-rspice-ui-symbol-tool-select.png`, `native-rspice-ui-symbol-tool-pin.png`, `native-rspice-ui-symbol-tool-polyline.png`, `native-rspice-ui-symbol-tool-circle.png`, `native-rspice-ui-symbol-tool-arc.png`, `native-rspice-ui-symbol-tool-arrow.png`, and `native-rspice-ui-symbol-tool-dot.png`. Then exercised `Ctrl+A`, `Ctrl+C`, `Ctrl+V`, `R`, `H`, and `Y`, with visible selected/copied/transformed symbol geometry in `native-rspice-ui-symbol-select-all.png`, `native-rspice-ui-symbol-copy-paste.png`, `native-rspice-ui-symbol-rotate.png`, `native-rspice-ui-symbol-mirror-h.png`, and `native-rspice-ui-symbol-mirror-v.png`. This closes the native body-tool and edit-shortcut smoke.

Native Generate/Undo/Redo evidence (2026-06-17): in a disposable native symbol-editor session, `Ctrl+A` plus Delete produced an empty symbol with the `Generate from schematic` call to action and three unplaced pins (`diagnostics/native-rspice-ui-symbol-generate-start-empty.png`). Clicking the call to action generated a placed box symbol from the schematic contract (`native-rspice-ui-symbol-generate-clicked.png`), `Ctrl+Z` restored the empty/unplaced state with `Undo: symbol edit` in the console (`native-rspice-ui-symbol-generate-clicked-undo.png`), and `Ctrl+Y` restored the generated symbol with `Redo: symbol edit` in the console (`native-rspice-ui-symbol-generate-clicked-redo.png`). The remaining manual checklist gaps are manual pin-drag remap visual evidence, SVG export through the native save dialog, and read-only refusal copy in the GUI.

Native pin-remap/export/read-only closeout (2026-06-17): after enumerating the real egui window handle instead of the debug console window, captured a complete native pin-remap sequence. `diagnostics/native-rspice-ui-pin-remap-parent-before-confirmed.png` shows the parent `hierarchical_rc_filter_tb / schematic` before the edit; `native-rspice-ui-pin-remap-symbol-before-confirmed.png` shows `user / rc_filter_core / symbol` before dragging `in`; `native-rspice-ui-pin-remap-symbol-after-confirmed.png` shows the `in` pin moved left and selected; and `native-rspice-ui-pin-remap-parent-after-confirmed.png` shows the reopened parent schematic with the incoming XU1 wire endpoint following the moved pin. Backing regression verification passed with `cargo test -p rspice-ui remap --lib -- --nocapture` (6/6). Native SVG export was verified through File -> Export schematic SVG..., with `diagnostics/native-rspice-ui-export-file-menu.png` showing the menu command, `native-rspice-ui-export-save-dialog.png` showing the native `Export SVG` dialog and SVG filter, and `native-rspice-ui-export-console-after.png` showing `Exported SVG: ...\diagnostics\native-svg-export-20260617-201809.svg`; the written SVG is 2558 bytes and starts with XML/SVG headers. During this pass the browser export workflow was also promoted from a graceful "not available" refusal to a real download-blob backend using the existing export IO seam, with focused SVG action tests covering dialog metadata, `.svg` extension enforcement, write success, dialog failure, and write failure. There is no stable default native route to a read-only symbol library because the legacy read-only primitives library is intentionally purged; read-only refusal remains covered by fixture tests, and `cargo test -p rspice-ui read_only_symbol_edit_paths_use_consistent_refusal_text --lib -- --nocapture` plus `cargo test -p rspice-ui symbol_store_refuses_read_only_libraries --lib -- --nocapture` passed.

- [x] **Step 3: Capture visual evidence**

Save screenshots under `diagnostics/` with names:

```text
diagnostics/symbol-editor-authored-instance.png
diagnostics/symbol-editor-structured-checks.png
diagnostics/symbol-editor-preview-and-tools.png
```

Evidence (2026-06-17): captured the three requested browser evidence PNGs under `diagnostics/`. `symbol-editor-authored-instance.png` shows the authored `rc_filter_core` symbol view and as-placed preview, `symbol-editor-structured-checks.png` shows the successful symbol-check console row, and `symbol-editor-preview-and-tools.png` shows the preview tile with the Circle body tool selected. Verified the files existed and rendered from disk; SHA-256 hashes were recorded with `Get-FileHash`.

Fresh verification after browser evidence capture (2026-06-17): `cargo fmt --all -- --check` passed; `cargo test -p rspice-ui shell::views::netlist --lib -- --nocapture` passed 36/36; `cargo test -p rspice-ui shell::views::symbol::tests --lib -- --nocapture` passed 8/8, including `preview_viewport_fits_nonzero_origin_as_placed_symbol`; `cargo test -p rspice-ui simulation::controller --lib -- --nocapture` passed 13/13; `cargo test -p rspice-ui --lib` passed 303/303; `cargo check -p rspice-ui --target wasm32-unknown-unknown` passed; and `cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings` passed.

Post-native-logging verification (2026-06-17): `cargo fmt --all -- --check` passed; `cargo test -p rspice-ui common::logging --lib -- --nocapture` passed 2/2; `cargo test -p rspice-ui --lib` passed 305/305; `cargo check -p rspice-ui --target wasm32-unknown-unknown` passed; and `cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings` passed.

Post-review native-logging verification (2026-06-17): after preserving unlisted dependency warnings with a global `warn` baseline and keeping `wgpu_core`/`wgpu_hal`/`naga` at `error`, `cargo fmt --all -- --check` passed; `cargo test -p rspice-ui common::logging --lib -- --nocapture` passed 2/2; `cargo build -p rspice-ui` passed; a redirected native launch produced a responding window and zero-byte stdout/stderr logs at `diagnostics/native-rspice-ui-20260617-193835.stdout.log` and `diagnostics/native-rspice-ui-20260617-193835.stderr.log`; `cargo test -p rspice-ui --lib` passed 305/305; `cargo check -p rspice-ui --target wasm32-unknown-unknown` passed; and `cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings` passed.

Post-closeout export/pin verification (2026-06-17): `cargo test -p rspice-ui svg_export --lib -- --nocapture` passed 6/6, `cargo check -p rspice-ui --target wasm32-unknown-unknown` passed cleanly after the browser download backend, `cargo build -p rspice-ui` passed, `cargo test -p rspice-ui remap --lib -- --nocapture` passed 6/6, `cargo test -p rspice-ui read_only_symbol_edit_paths_use_consistent_refusal_text --lib -- --nocapture` passed 1/1, and `cargo test -p rspice-ui symbol_store_refuses_read_only_libraries --lib -- --nocapture` passed 1/1. After rustfmt, final verification passed with `cargo fmt --all -- --check`, `cargo test -p rspice-ui svg_export --lib -- --nocapture` (6/6), `cargo check -p rspice-ui --target wasm32-unknown-unknown`, `cargo test -p rspice-ui --lib -- --nocapture` (308/308), and `cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings`.

- [x] **Step 4: Dispatch final subagent code review**

Ask a reviewer subagent to review the full implementation against:

- `design/app/volta-symbol-editor.html`;
- `docs/superpowers/specs/2026-06-17-symbol-editor-resolver-design.md`;
- this implementation plan;
- the diff since commit `1e26c22a`.

Evidence (2026-06-17): Final reviewer Popper found four Important issues: non-zero symbol origins were not honored by placed-instance geometry/remaps/rendering, authored symbol bodies were not used for schematic hit testing, culling/fit-to-content used generic bounds, and placement preview did not render the resolved pending library-cell symbol. Follow-up reviewer Euler was dispatched after the TDD fix pass against the focused uncommitted diff.

- [x] **Step 5: Fix review findings and rerun verification**

For every Critical or Important finding, add or update a failing test first, implement the fix, and rerun:

```powershell
cargo test -p rspice-ui
cargo build -p rspice-ui
```

Evidence (2026-06-17): Popper's four Important findings were fixed with TDD coverage for origin-relative placed terminals, origin-aware wire remaps, origin-relative SVG/bounds, resolved-symbol body hit testing, culling/fit bounds, and pending library-cell placement preview. Euler's follow-up Important findings were fixed with TDD coverage for resolved-symbol marquee commit parity and a no-allocation resolved-bounds fold; Euler's Minor stale-context note was also fixed with a topology-version refresh test. Verification after those fixes: `cargo test -p rspice-ui schematic::view --lib -- --nocapture` passed with 14 tests, `cargo test -p rspice-ui common::app::app_workspace_actions::tests::storing_symbol_document --lib -- --nocapture` passed with 5 tests, `cargo test -p rspice-ui state::schematic::component --lib -- --nocapture` passed with 3 tests, `cargo test -p rspice-ui state::symbol_resolver --lib -- --nocapture` passed with 5 tests, `cargo fmt --all -- --check` passed, `cargo check -p rspice-ui` passed, `cargo check -p rspice-ui --target wasm32-unknown-unknown` passed, `cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings` passed, and `cargo test -p rspice-ui` passed with 280 tests.

Additional verification-fix evidence (2026-06-17): a read-only follow-up review found that the Symbol editor preview tile still fit absolute authored document bounds while the placed-instance renderer draws effective coordinates relative to `document.origin`, so non-zero-origin symbols could be miscentered or clipped in the preview tile. Added `preview_viewport_fits_nonzero_origin_as_placed_symbol` as a failing regression, changed the preview viewport to fit `document_bounds - document.origin`, and verified the new regression plus existing preview tile tests. Focused verification passed: `cargo test -p rspice-ui shell::views::symbol::tests --lib -- --nocapture` with 8/8 tests, and the full `cargo test -p rspice-ui --lib` passed with 301/301 tests.

- [ ] **Step 6: Commit verification fixes**

```powershell
git add crates/rspice-ui docs/superpowers diagnostics
git commit -m "test: verify symbol editor resolver"
```

Use exact paths instead of broad directories if unrelated dirty files are present.
