//! Active-sheet visibility authority for the schematic canvas and the docks
//! that read it.
//!
//! Multi-sheet membership is project-owned, while object geometry remains in
//! the canonical schematic buffer. Keeping this predicate in one module makes
//! paint and hit-test code consume the same stable sheet identity.
//!
//! The canvas always paints the active sheet; a dock may list either it or the
//! whole cell view, so [`SheetScope`] and the position the session is holding
//! live here too. One owner is the point: a navigator row, an inspector list
//! and a canvas hit test cannot disagree about which objects are on screen.

use std::borrow::Cow;

use egui::{Context, Id};

use crate::state::{ActiveSheetTopology, SchematicState, Selection, SheetCatalog};
use crate::workbench::app_state::AppState;

use super::{SchematicSymbolContext, SelectionWindow};

/// How much of a cell view a dock lists.
///
/// Listing the whole cell view is the default: a scope that removes rows is
/// asked for explicitly, and is offered only where there is more than one
/// sheet to choose between.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum SheetScope {
    /// Only the objects the active sheet owns.
    ActiveSheet,
    /// Every object of the cell view, whichever sheet owns it.
    #[default]
    AllSheets,
}

impl SheetScope {
    /// The positions a scope control offers, in the order it offers them.
    pub(crate) const OPTIONS: [Self; 2] = [Self::AllSheets, Self::ActiveSheet];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::AllSheets => "All sheets",
            Self::ActiveSheet => "This sheet",
        }
    }
}

/// Whether one object is listed under `scope`.
pub(crate) fn object_is_in_scope(state: &AppState, scope: SheetScope, object_id: u64) -> bool {
    scope == SheetScope::AllSheets || object_is_on_active_sheet(state, object_id)
}

/// The active cell view's sheet catalog, and only where it holds more than one
/// sheet.
///
/// A single-sheet cell view *is* its sheet. Every surface that scopes to a
/// sheet, names one, or counts them asks this first and says nothing at all
/// when it answers `None`, so none of them can offer a control with one
/// position or a page number with one page.
pub(crate) fn multi_sheet_catalog(state: &AppState) -> Option<&SheetCatalog> {
    let key = state.workspace.content.active_schematic_reference().key();
    state
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
        .filter(|catalog| catalog.sheets().len() > 1)
}

/// The active cell view's sheet catalog, however many sheets it holds.
pub(crate) fn active_sheet_catalog(state: &AppState) -> Option<&SheetCatalog> {
    let key = state.workspace.content.active_schematic_reference().key();
    state
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
}

/// The sheet the canvas is showing, as a run's net geometry keys its sheets:
/// `None` for a cell view with no sheet catalog.
pub(crate) fn active_sheet_id(state: &AppState) -> Option<crate::state::SheetId> {
    active_sheet_catalog(state).and_then(SheetCatalog::active_sheet_id)
}

/// The net geometry the active sheet draws, in its own coordinates.
pub(crate) fn active_sheet_net_geometry(
    state: &AppState,
) -> &rspice_simulation::netlist_gen::SheetNetGeometry {
    state
        .simulation
        .source
        .cross_probe
        .on_sheet(active_sheet_id(state))
}

/// The reading position the docks are holding.
///
/// It is session state and nothing else: a scope is how one reader is looking
/// at a drawing this minute, so it never reaches the project file and never
/// advances a revision.
pub(crate) fn sheet_scope(ctx: &Context) -> SheetScope {
    ctx.data(|data| data.get_temp::<SheetScope>(sheet_scope_id()))
        .unwrap_or_default()
}

pub(crate) fn set_sheet_scope(ctx: &Context, scope: SheetScope) {
    ctx.data_mut(|data| data.insert_temp(sheet_scope_id(), scope));
}

fn sheet_scope_id() -> Id {
    Id::new("schematic.sheet_scope")
}

pub(crate) fn object_is_on_active_sheet(state: &AppState, object_id: u64) -> bool {
    object_is_on_active_sheet_in_workspace(&state.workspace, object_id)
}

fn object_is_on_active_sheet_in_workspace(
    workspace: &crate::state::ProjectWorkspace,
    object_id: u64,
) -> bool {
    let key = workspace.content.active_schematic_reference().key();
    rspice_schematic_editor::view::design_view::object_is_on_active_sheet(
        workspace.content.design_management.sheet_catalog(&key),
        object_id,
    )
}

/// Return candidates in their authored z-order, excluding objects owned by a
/// different sheet before any nearest/first-hit resolver runs.
///
/// The borrowed fast path keeps legacy single-sheet projects allocation-free.
pub(super) fn objects_on_active_sheet<'a, T: Clone>(
    state: &AppState,
    objects: &'a [T],
    id: impl Fn(&T) -> u64,
) -> Cow<'a, [T]> {
    super::schematic_design_view(state).objects_on_active_sheet(objects, id)
}

/// Build a live selection containing only objects owned by the active sheet.
/// This is also the sheet-switch boundary: stale IDs from a previously active
/// sheet can never flow into keyboard commands or context-menu actions.
pub(super) fn selection_on_active_sheet(state: &AppState) -> Selection {
    selection_filtered_to_active_sheet(state, &state.schematic.session.editor.selection)
}

pub(crate) fn selection_filtered_to_active_sheet(
    state: &AppState,
    source: &Selection,
) -> Selection {
    let mut selection = Selection::default();
    // This runs twice a frame. Nothing selected keeps nothing, and the sheet
    // catalog is looked up once rather than once per object.
    if source.is_empty() {
        return selection;
    }
    let catalog = active_sheet_catalog(state);
    let on_sheet = |id: u64| {
        rspice_schematic_editor::view::design_view::object_is_on_active_sheet(catalog, id)
    };

    for object in &state.schematic.document().components {
        if source.has_component(object.id) && on_sheet(object.id) {
            selection.select_component(object.id);
        }
    }
    let wire_parts = !source.wire_segments.is_empty() || !source.wire_vertices.is_empty();
    for object in &state.schematic.document().wires {
        let whole = source.has_wire(object.id);
        if !(whole || wire_parts) || !on_sheet(object.id) {
            continue;
        }
        if whole {
            selection.select_wire(object.id);
        }
        if wire_parts {
            selection.wire_segments.extend(
                source
                    .wire_segments
                    .iter()
                    .filter(|selected| selected.wire_id == object.id)
                    .copied(),
            );
            selection.wire_vertices.extend(
                source
                    .wire_vertices
                    .iter()
                    .filter(|selected| selected.wire_id == object.id)
                    .copied(),
            );
        }
    }
    for object in &state.schematic.document().junctions {
        if source.has_junction(object.pos) && on_sheet(object.id) {
            selection.select_junction(object.pos);
        }
    }
    for object in &state.schematic.document().buses {
        if source.has_bus(object.id) && on_sheet(object.id) {
            selection.select_bus(object.id);
        }
    }
    for object in &state.schematic.document().bus_taps {
        if source.has_bus_tap(object.id) && on_sheet(object.id) {
            selection.select_bus_tap(object.id);
        }
    }
    for object in &state.schematic.document().net_labels {
        if source.has_net_label(object.id) && on_sheet(object.id) {
            selection.select_net_label(object.id);
        }
    }
    for object in &state.schematic.document().design_notes {
        if source.has_design_note(object.id) && on_sheet(object.id) {
            selection.select_design_note(object.id);
        }
    }
    for object in &state.schematic.document().documentation_shapes {
        if source.has_documentation_shape(object.id) && on_sheet(object.id) {
            selection.select_documentation_shape(object.id);
        }
    }
    for object in &state.schematic.document().probes {
        if source.has_probe(object.id) && on_sheet(object.id) {
            selection.select_probe(object.id);
        }
    }
    selection
}

pub(crate) fn retain_selection_on_active_sheet(state: &mut AppState) {
    let selection = selection_on_active_sheet(state);
    state.schematic.session.editor.selection = selection;
}

/// Every complete object on the active sheet the session's selection filter
/// admits.
///
/// This is what Select all selects. It is written here, beside the other
/// active-sheet predicates, because "which objects does the reader see" is
/// this module's question and not the edit command's.
pub(crate) fn selectable_objects_on_active_sheet(state: &AppState) -> Selection {
    let filter = state.ui.schematic_selection_filter;
    let mut selection = Selection::default();
    if filter.instances {
        for object in &state.schematic.document().components {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_component(object.id);
            }
        }
    }
    if filter.wires {
        for object in &state.schematic.document().wires {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_wire(object.id);
            }
        }
        for object in &state.schematic.document().junctions {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_junction(object.pos);
            }
        }
        for object in &state.schematic.document().buses {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_bus(object.id);
            }
        }
        for object in &state.schematic.document().bus_taps {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_bus_tap(object.id);
            }
        }
    }
    if filter.labels {
        for object in &state.schematic.document().net_labels {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_net_label(object.id);
            }
        }
    }
    if filter.annotations {
        for object in &state.schematic.document().design_notes {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_design_note(object.id);
            }
        }
        for object in &state.schematic.document().documentation_shapes {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_documentation_shape(object.id);
            }
        }
        for object in &state.schematic.document().probes {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_probe(object.id);
            }
        }
    }
    selection
}

pub(super) fn select_in_rect_on_active_sheet(
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    window: SelectionWindow,
    add_to_selection: bool,
) -> usize {
    if add_to_selection {
        retain_selection_on_active_sheet(state);
    }
    let selected_before = if add_to_selection {
        state.schematic.session.editor.selection.count()
    } else {
        0
    };
    let (document, editor) = state.schematic.document_and_editor();
    symbol_context.select_in_rect(document, &mut editor.selection, window, add_to_selection);
    retain_selection_on_active_sheet(state);
    state
        .ui
        .schematic_selection_filter
        .retain_matching(&mut state.schematic.session.editor.selection);
    state
        .schematic
        .session
        .editor
        .selection
        .count()
        .saturating_sub(selected_before)
}

pub(super) fn active_wire_at(state: &AppState, point: crate::state::Point) -> Option<u64> {
    super::schematic_design_view(state).active_wire_at(point)
}
pub(super) fn active_junction_at(state: &AppState, point: crate::state::Point) -> Option<u64> {
    super::schematic_design_view(state).active_junction_at(point)
}

/// Run a topology operation with hidden-sheet conductors removed from the
/// mutable working set, then restore them unchanged. Operations such as
/// junction dragging and cleanup intentionally scan the entire wire graph;
/// this boundary gives those mature algorithms exactly one sheet to operate
/// on without weakening their single-sheet behavior.
pub(crate) fn with_active_wire_topology<R>(
    state: &mut AppState,
    operation: impl FnOnce(&mut SchematicState, &ActiveSheetTopology) -> R,
) -> R {
    state
        .schematic
        .with_active_wire_topology(off_active_sheet(&state.workspace), operation)
}

/// Whether an object belongs to one of the cell view's other sheets.
///
/// The catalog is found once, not once per object: a topology scope asks
/// this of every object in the document, on every frame of a drag, and
/// spelling the cell view's key and looking its catalog up afresh for each
/// of them cost 13 ms of every drag frame at production scale.
pub(crate) fn off_active_sheet(
    workspace: &crate::state::ProjectWorkspace,
) -> impl Fn(u64) -> bool + '_ {
    let key = workspace.content.active_schematic_reference().key();
    let catalog = workspace.content.design_management.sheet_catalog(&key);
    move |id| !rspice_schematic_editor::view::design_view::object_is_on_active_sheet(catalog, id)
}

/// The same operation on a schematic borrowed apart from its workspace: a
/// preview's copy, or the buffer inside an undo record.
pub(crate) fn with_active_wire_topology_in<R>(
    workspace: &crate::state::ProjectWorkspace,
    schematic: &mut SchematicState,
    operation: impl FnOnce(&mut SchematicState, &ActiveSheetTopology) -> R,
) -> R {
    schematic.with_active_wire_topology(off_active_sheet(workspace), operation)
}

/// Finish the route being drawn on the active sheet. Conductors on the cell
/// view's other sheets share its coordinates; they are kept out of reach so
/// the route can neither split nor dot them.
pub(crate) fn finish_wire_on_active_sheet(state: &mut AppState) -> Option<u64> {
    state
        .schematic
        .finish_wire(off_active_sheet(&state.workspace))
}

pub(super) fn active_sheet_has_objects(state: &AppState) -> bool {
    state
        .schematic
        .document()
        .components
        .iter()
        .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .document()
            .wires
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .document()
            .buses
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .document()
            .bus_taps
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .document()
            .junctions
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .document()
            .net_labels
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .document()
            .design_notes
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .document()
            .documentation_shapes
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .document()
            .probes
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
}

/// A cell view with two sheets: `active_ids` stay on the active Sheet 1 and
/// `hidden_ids` belong to Sheet 2. Shared by the canvas modules' tests.
#[cfg(test)]
pub(crate) fn two_sheet_state(active_ids: &[u64], hidden_ids: &[u64]) -> AppState {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    let mut state = AppState::default();
    let key = state.workspace.content.active_schematic_reference().key();
    let all_ids = active_ids
        .iter()
        .chain(hidden_ids)
        .copied()
        .collect::<Vec<_>>();
    let first = state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", all_ids)
        .expect("first sheet");
    let catalog = state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Sheet 2".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("second sheet");
    if !hidden_ids.is_empty() {
        catalog
            .assign_objects(catalog.revision(), second, hidden_ids.iter().copied())
            .expect("hidden assignments");
    }
    catalog.set_active(first).expect("active sheet");
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point};
    use crate::state::{Junction, Wire};

    /// The scope is a reading position over the one membership authority: it
    /// narrows what a dock lists and decides nothing else, so `AllSheets` must
    /// admit an object the active sheet does not own.
    #[test]
    fn a_scope_narrows_the_cell_view_to_the_active_sheet_and_defaults_to_neither() {
        let state = two_sheet_state(&[10], &[20]);

        assert_eq!(SheetScope::default(), SheetScope::AllSheets);
        assert!(object_is_in_scope(&state, SheetScope::AllSheets, 10));
        assert!(object_is_in_scope(&state, SheetScope::AllSheets, 20));
        assert!(object_is_in_scope(&state, SheetScope::ActiveSheet, 10));
        assert!(!object_is_in_scope(&state, SheetScope::ActiveSheet, 20));
    }

    /// A cell view with one sheet has nothing to scope, name or count, and a
    /// cell view with no catalog at all has less than that.
    #[test]
    fn only_a_cell_view_above_one_sheet_offers_a_catalog_to_scope_by() {
        let mut single = AppState::default();
        assert!(multi_sheet_catalog(&single).is_none());

        let key = single.workspace.content.active_schematic_reference().key();
        single
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Sheet 1", [10])
            .expect("the fixture cell view takes a sheet catalog");
        assert!(multi_sheet_catalog(&single).is_none());

        assert!(multi_sheet_catalog(&two_sheet_state(&[10], &[20])).is_some());
    }

    #[test]
    fn overlapping_hidden_object_is_removed_before_marquee_selection() {
        let mut state = two_sheet_state(&[10], &[20]);
        state.schematic.document_mut_for_test().components = vec![
            Component::new(20, ComponentType::Capacitor, Point::new(10, 10)),
            Component::new(10, ComponentType::Resistor, Point::new(10, 10)),
        ];

        select_in_rect_on_active_sheet(
            &mut state,
            &SchematicSymbolContext::default(),
            SelectionWindow::new(0, 0, 20, 20, false),
            false,
        );

        assert!(state.schematic.session.editor.selection.has_component(10));
        assert!(!state.schematic.session.editor.selection.has_component(20));
    }

    #[test]
    fn select_all_and_sheet_switch_selection_are_membership_aware() {
        let mut state = two_sheet_state(&[10], &[20]);
        state.schematic.document_mut_for_test().components = vec![
            Component::new(10, ComponentType::Resistor, Point::origin()),
            Component::new(20, ComponentType::Capacitor, Point::origin()),
        ];
        state
            .schematic
            .session
            .editor
            .selection
            .select_component(20);

        retain_selection_on_active_sheet(&mut state);
        assert!(state.schematic.session.editor.selection.is_empty());

        state.schematic.session.editor.selection = selectable_objects_on_active_sheet(&state);
        assert!(state.schematic.session.editor.selection.has_component(10));
        assert!(!state.schematic.session.editor.selection.has_component(20));
    }

    #[test]
    fn select_all_honors_the_persisted_schematic_selection_filter() {
        let mut state = two_sheet_state(&[10, 11], &[]);
        state.schematic.document_mut_for_test().components =
            vec![Component::new(10, ComponentType::Resistor, Point::origin())];
        state.schematic.document_mut_for_test().wires =
            vec![Wire::segment(11, Point::origin(), Point::new(20, 0))];
        state.ui.schematic_selection_filter.instances = false;

        state.schematic.session.editor.selection = selectable_objects_on_active_sheet(&state);

        assert!(!state.schematic.session.editor.selection.has_component(10));
        assert!(state.schematic.session.editor.selection.has_wire(11));
    }

    #[test]
    fn topology_operation_restores_hidden_conductors_unchanged() {
        let mut state = two_sheet_state(&[10, 11], &[20, 21]);
        let origin = Point::new(10, 10);
        let destination = Point::new(15, 15);
        state.schematic.document_mut_for_test().wires = vec![
            Wire::segment(20, origin, Point::new(30, 10)),
            Wire::segment(10, origin, Point::new(10, 30)),
        ];
        state.schematic.document_mut_for_test().junctions =
            vec![Junction::new(21, origin), Junction::new(11, origin)];
        let hidden_wire = state.schematic.document().wires[0].clone();
        let hidden_junction = state.schematic.document().junctions[0];

        assert!(with_active_wire_topology(&mut state, |schematic, scope| {
            schematic.move_all_vertices_at(origin, destination, scope)
        }));

        assert_eq!(
            state
                .schematic
                .document()
                .wires
                .iter()
                .find(|wire| wire.id == 20),
            Some(&hidden_wire)
        );
        assert_eq!(
            state
                .schematic
                .document()
                .junctions
                .iter()
                .find(|junction| junction.id == 21),
            Some(&hidden_junction)
        );
        assert!(
            state
                .schematic
                .document()
                .wires
                .iter()
                .find(|wire| wire.id == 10)
                .is_some_and(|wire| wire.points.contains(&destination))
        );
        assert!(
            state
                .schematic
                .document()
                .junctions
                .iter()
                .find(|junction| junction.id == 11)
                .is_some_and(|junction| junction.pos == destination)
        );
    }

    /// Sheet 2's wire runs through the point where a route drawn on Sheet 1
    /// ends. The two sheets share coordinates, not conductors: the hidden wire
    /// keeps its id and points, gets no dot, and undo needs nothing from it.
    #[test]
    fn a_route_drawn_on_one_sheet_leaves_another_sheets_wire_whole() {
        let mut state = two_sheet_state(&[], &[20]);
        let hidden = Wire::segment(20, Point::new(0, 100), Point::new(200, 100));
        state.schematic.document_mut_for_test().wires = vec![hidden.clone()];
        state.schematic.init_undo_history();

        state.schematic.start_wire(Point::new(100, 0));
        state.schematic.extend_wire(Point::new(100, 100));
        let drawn = finish_wire_on_active_sheet(&mut state).expect("the route is drawn");

        let wires = &state.schematic.document().wires;
        assert_eq!(wires.iter().find(|wire| wire.id == 20), Some(&hidden));
        assert_eq!(wires.len(), 2, "the hidden wire was not split");
        assert!(object_is_on_active_sheet(&state, drawn));
        assert!(state.schematic.document().junctions.is_empty());

        assert!(state.schematic.undo());
        assert_eq!(state.schematic.document().wires, vec![hidden]);
    }

    /// Joining collinear wires reaches only the active sheet. Sheet 2's wire
    /// carries on from Sheet 1's on the same line, but it is on another
    /// page: Sheet 1's two wires become one and Sheet 2's stays as drawn.
    #[test]
    fn joining_collinear_wires_leaves_another_sheets_wire_whole() {
        let mut state = two_sheet_state(&[1, 2], &[3]);
        let hidden = Wire::segment(3, Point::new(100, 0), Point::new(160, 0));
        state.schematic.document_mut_for_test().wires = vec![
            Wire::segment(1, Point::new(0, 0), Point::new(40, 0)),
            Wire::segment(2, Point::new(40, 0), Point::new(100, 0)),
            hidden.clone(),
        ];

        assert!(with_active_wire_topology(&mut state, |schematic, scope| {
            schematic.join_collinear_wires(scope)
        }));

        let wires = &state.schematic.document().wires;
        assert_eq!(wires.len(), 2, "{wires:?}");
        assert_eq!(wires.iter().find(|wire| wire.id == 3), Some(&hidden));
        assert_eq!(
            wires
                .iter()
                .find(|wire| wire.id == 1)
                .map(|wire| &wire.points),
            Some(&vec![Point::new(0, 0), Point::new(100, 0)])
        );
    }

    /// Two wires meeting at a pin get a dot, but only a pin on the same
    /// sheet: Sheet 2's resistor under Sheet 1's route corner joins nothing.
    #[test]
    fn another_sheets_pin_under_a_route_corner_adds_no_dot() {
        let mut state = two_sheet_state(&[], &[20]);
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(Component::new(
                20,
                ComponentType::Resistor,
                Point::new(20, 0),
            ));
        let pin = state.schematic.document().components[0].terminal_positions()[0].1;
        state.schematic.init_undo_history();

        state.schematic.start_wire(Point::new(pin.x - 40, pin.y));
        state.schematic.extend_wire(pin);
        state.schematic.extend_wire(Point::new(pin.x, pin.y + 40));
        finish_wire_on_active_sheet(&mut state).expect("the route is drawn");

        assert!(state.schematic.document().junctions.is_empty());
        assert_eq!(state.schematic.document().components.len(), 1);
    }

    /// A deletion on Sheet 1 leaves Sheet 2's objects alone, even a dot
    /// that marks nothing on Sheet 1's wires, and its undo step holds both
    /// sheets.
    #[test]
    fn a_deletion_keeps_another_sheets_objects_and_undo_restores_both_sheets() {
        let mut state = two_sheet_state(&[10], &[21]);
        state.schematic.document_mut_for_test().wires =
            vec![Wire::segment(10, Point::origin(), Point::new(20, 0))];
        state.schematic.document_mut_for_test().junctions =
            vec![Junction::new(21, Point::new(200, 200))];
        state.schematic.session.editor.selection.select_wire(10);
        state.schematic.init_undo_history();

        assert!(state.delete_schematic_selection());
        assert!(state.schematic.document().wires.is_empty());
        assert_eq!(
            state.schematic.document().junctions,
            vec![Junction::new(21, Point::new(200, 200))]
        );

        assert!(state.schematic.undo());
        assert!(
            state
                .schematic
                .document()
                .wires
                .iter()
                .any(|wire| wire.id == 10)
        );
        assert_eq!(
            state.schematic.document().junctions,
            vec![Junction::new(21, Point::new(200, 200))]
        );
    }
}
