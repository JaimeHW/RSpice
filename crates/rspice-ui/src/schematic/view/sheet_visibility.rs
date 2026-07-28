//! Active-sheet visibility authority for the schematic canvas.
//!
//! Multi-sheet membership is project-owned, while object geometry remains in
//! the canonical schematic buffer. Keeping this predicate in one module makes
//! paint and hit-test code consume the same stable sheet identity.

use std::borrow::Cow;

use crate::workbench::app_state::AppState;
use crate::state::{BusTap, Junction, SchematicState, Selection, Wire};

use super::{SchematicSymbolContext, SelectionWindow};

pub(crate) fn object_is_on_active_sheet(state: &AppState, object_id: u64) -> bool {
    let key = state.workspace.active_schematic_reference().key();
    let Some(catalog) = state.workspace.design_management.sheet_catalog(&key) else {
        return true;
    };
    let Some(active_sheet_id) = catalog.active_sheet_id() else {
        return true;
    };
    state
        .workspace
        .design_management
        .sheet_for_object_or_active(&key, object_id)
        == Some(active_sheet_id)
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
    if objects
        .iter()
        .all(|object| object_is_on_active_sheet(state, id(object)))
    {
        Cow::Borrowed(objects)
    } else {
        Cow::Owned(
            objects
                .iter()
                .filter(|object| object_is_on_active_sheet(state, id(object)))
                .cloned()
                .collect(),
        )
    }
}

/// Build a live selection containing only objects owned by the active sheet.
/// This is also the sheet-switch boundary: stale IDs from a previously active
/// sheet can never flow into keyboard commands or context-menu actions.
pub(super) fn selection_on_active_sheet(state: &AppState) -> Selection {
    selection_filtered_to_active_sheet(state, &state.schematic.selection)
}

pub(crate) fn selection_filtered_to_active_sheet(
    state: &AppState,
    source: &Selection,
) -> Selection {
    let mut selection = Selection::default();

    for object in &state.schematic.components {
        if source.has_component(object.id) && object_is_on_active_sheet(state, object.id) {
            selection.select_component(object.id);
        }
    }
    for object in &state.schematic.wires {
        if object_is_on_active_sheet(state, object.id) {
            if source.has_wire(object.id) {
                selection.select_wire(object.id);
            }
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
    for object in &state.schematic.junctions {
        if source.has_junction(object.pos) && object_is_on_active_sheet(state, object.id) {
            selection.select_junction(object.pos);
        }
    }
    for object in &state.schematic.buses {
        if source.has_bus(object.id) && object_is_on_active_sheet(state, object.id) {
            selection.select_bus(object.id);
        }
    }
    for object in &state.schematic.bus_taps {
        if source.has_bus_tap(object.id) && object_is_on_active_sheet(state, object.id) {
            selection.select_bus_tap(object.id);
        }
    }
    for object in &state.schematic.net_labels {
        if source.has_net_label(object.id) && object_is_on_active_sheet(state, object.id) {
            selection.select_net_label(object.id);
        }
    }
    for object in &state.schematic.design_notes {
        if source.has_design_note(object.id) && object_is_on_active_sheet(state, object.id) {
            selection.select_design_note(object.id);
        }
    }
    for object in &state.schematic.documentation_shapes {
        if source.has_documentation_shape(object.id) && object_is_on_active_sheet(state, object.id)
        {
            selection.select_documentation_shape(object.id);
        }
    }
    selection
}

pub(crate) fn retain_selection_on_active_sheet(state: &mut AppState) {
    let selection = selection_on_active_sheet(state);
    state.schematic.selection = selection;
}

#[cfg(test)]
pub(crate) fn select_all_on_active_sheet(state: &mut AppState) {
    let filter = state.ui.schematic_selection_filter;
    let mut selection = Selection::default();
    if filter.instances {
        for object in &state.schematic.components {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_component(object.id);
            }
        }
    }
    if filter.wires {
        for object in &state.schematic.wires {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_wire(object.id);
            }
        }
        for object in &state.schematic.junctions {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_junction(object.pos);
            }
        }
        for object in &state.schematic.buses {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_bus(object.id);
            }
        }
        for object in &state.schematic.bus_taps {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_bus_tap(object.id);
            }
        }
    }
    if filter.labels {
        for object in &state.schematic.net_labels {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_net_label(object.id);
            }
        }
    }
    if filter.annotations {
        for object in &state.schematic.design_notes {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_design_note(object.id);
            }
        }
        for object in &state.schematic.documentation_shapes {
            if object_is_on_active_sheet(state, object.id) {
                selection.select_documentation_shape(object.id);
            }
        }
    }
    state.schematic.selection = selection;
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
        state.schematic.selection.count()
    } else {
        0
    };
    symbol_context.select_in_rect(&mut state.schematic, window, add_to_selection);
    retain_selection_on_active_sheet(state);
    state
        .ui
        .schematic_selection_filter
        .retain_matching(&mut state.schematic.selection);
    state
        .schematic
        .selection
        .count()
        .saturating_sub(selected_before)
}

pub(super) fn active_wire_at(state: &AppState, point: crate::state::Point) -> Option<u64> {
    state
        .schematic
        .wires
        .iter()
        .find(|wire| object_is_on_active_sheet(state, wire.id) && wire.contains_point(point))
        .map(|wire| wire.id)
}

pub(super) fn active_junction_at(state: &AppState, point: crate::state::Point) -> Option<u64> {
    state
        .schematic
        .junctions
        .iter()
        .find(|junction| junction.pos == point && object_is_on_active_sheet(state, junction.id))
        .map(|junction| junction.id)
}

pub(super) fn active_wire_point_is_draggable(state: &AppState, point: crate::state::Point) -> bool {
    active_junction_at(state, point).is_some()
        || state
            .schematic
            .wires
            .iter()
            .any(|wire| object_is_on_active_sheet(state, wire.id) && wire.points.contains(&point))
}

/// Run a topology operation with hidden-sheet conductors removed from the
/// mutable working set, then restore them unchanged. Operations such as
/// junction dragging and cleanup intentionally scan the entire wire graph;
/// this boundary gives those mature algorithms exactly one sheet to operate
/// on without weakening their single-sheet behavior.
pub(crate) fn with_active_wire_topology<R>(
    state: &mut AppState,
    operation: impl FnOnce(&mut SchematicState) -> R,
) -> R {
    let hidden_wire_ids = state
        .schematic
        .wires
        .iter()
        .filter(|wire| !object_is_on_active_sheet(state, wire.id))
        .map(|wire| wire.id)
        .collect::<std::collections::HashSet<_>>();
    let hidden_junction_ids = state
        .schematic
        .junctions
        .iter()
        .filter(|junction| !object_is_on_active_sheet(state, junction.id))
        .map(|junction| junction.id)
        .collect::<std::collections::HashSet<_>>();

    let hidden_wires = take_hidden(&mut state.schematic.wires, |wire: &Wire| {
        hidden_wire_ids.contains(&wire.id)
    });
    let hidden_junctions = take_hidden(&mut state.schematic.junctions, |junction: &Junction| {
        hidden_junction_ids.contains(&junction.id)
    });
    let result = operation(&mut state.schematic);
    restore_hidden(&mut state.schematic.wires, hidden_wires);
    restore_hidden(&mut state.schematic.junctions, hidden_junctions);
    result
}

/// Protect hidden conductors around an operation that owns its own undo
/// transaction. Keeping hidden objects present while that transaction captures
/// snapshots preserves complete undo/redo state; restoring their authored
/// values afterward removes any incidental cross-sheet endpoint remap or orphan
/// cleanup performed by the legacy whole-buffer algorithm.
pub(crate) fn with_hidden_wire_topology_preserved<R>(
    state: &mut AppState,
    operation: impl FnOnce(&mut SchematicState) -> R,
) -> R {
    let hidden_wires = state
        .schematic
        .wires
        .iter()
        .enumerate()
        .filter(|(_, wire)| !object_is_on_active_sheet(state, wire.id))
        .map(|(index, wire)| (index, wire.clone()))
        .collect::<Vec<_>>();
    let hidden_junctions = state
        .schematic
        .junctions
        .iter()
        .enumerate()
        .filter(|(_, junction)| !object_is_on_active_sheet(state, junction.id))
        .map(|(index, junction)| (index, *junction))
        .collect::<Vec<_>>();
    let hidden_bus_taps = state
        .schematic
        .bus_taps
        .iter()
        .enumerate()
        .filter(|(_, tap)| !object_is_on_active_sheet(state, tap.id))
        .map(|(index, tap)| (index, tap.clone()))
        .collect::<Vec<_>>();

    let result = operation(&mut state.schematic);
    restore_authored_by_id(&mut state.schematic.wires, hidden_wires, |wire: &Wire| {
        wire.id
    });
    restore_authored_by_id(
        &mut state.schematic.junctions,
        hidden_junctions,
        |junction: &Junction| junction.id,
    );
    restore_authored_by_id(
        &mut state.schematic.bus_taps,
        hidden_bus_taps,
        |tap: &BusTap| tap.id,
    );
    result
}

fn take_hidden<T>(items: &mut Vec<T>, hidden: impl Fn(&T) -> bool) -> Vec<(usize, T)> {
    let mut retained = Vec::with_capacity(items.len());
    let mut removed = Vec::new();
    for (index, item) in std::mem::take(items).into_iter().enumerate() {
        if hidden(&item) {
            removed.push((index, item));
        } else {
            retained.push(item);
        }
    }
    *items = retained;
    removed
}

fn restore_hidden<T>(items: &mut Vec<T>, hidden: Vec<(usize, T)>) {
    for (index, item) in hidden {
        items.insert(index.min(items.len()), item);
    }
}

fn restore_authored_by_id<T>(
    items: &mut Vec<T>,
    authored: Vec<(usize, T)>,
    id: impl Fn(&T) -> u64,
) {
    for (original_index, object) in authored {
        if let Some(current_index) = items
            .iter()
            .position(|candidate| id(candidate) == id(&object))
        {
            items[current_index] = object;
        } else {
            items.insert(original_index.min(items.len()), object);
        }
    }
}

pub(super) fn active_sheet_has_objects(state: &AppState) -> bool {
    state
        .schematic
        .components
        .iter()
        .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .wires
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .buses
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .bus_taps
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .junctions
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .net_labels
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .design_notes
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .documentation_shapes
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
        || state
            .schematic
            .probes
            .iter()
            .any(|object| object_is_on_active_sheet(state, object.id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Component, ComponentType, Point, SheetDefinition, SheetPortPolicy, SheetTemplate,
    };

    fn two_sheet_state(active_ids: &[u64], hidden_ids: &[u64]) -> AppState {
        let mut state = AppState::default();
        let key = state.workspace.active_schematic_reference().key();
        let all_ids = active_ids
            .iter()
            .chain(hidden_ids)
            .copied()
            .collect::<Vec<_>>();
        let first = state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Sheet 1", all_ids)
            .expect("first sheet");
        let catalog = state
            .workspace
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

    #[test]
    fn overlapping_hidden_object_is_removed_before_marquee_selection() {
        let mut state = two_sheet_state(&[10], &[20]);
        state.schematic.components = vec![
            Component::new(20, ComponentType::Capacitor, Point::new(10, 10)),
            Component::new(10, ComponentType::Resistor, Point::new(10, 10)),
        ];

        select_in_rect_on_active_sheet(
            &mut state,
            &SchematicSymbolContext::default(),
            SelectionWindow::new(0, 0, 20, 20, false),
            false,
        );

        assert!(state.schematic.selection.has_component(10));
        assert!(!state.schematic.selection.has_component(20));
    }

    #[test]
    fn select_all_and_sheet_switch_selection_are_membership_aware() {
        let mut state = two_sheet_state(&[10], &[20]);
        state.schematic.components = vec![
            Component::new(10, ComponentType::Resistor, Point::origin()),
            Component::new(20, ComponentType::Capacitor, Point::origin()),
        ];
        state.schematic.selection.select_component(20);

        retain_selection_on_active_sheet(&mut state);
        assert!(state.schematic.selection.is_empty());

        select_all_on_active_sheet(&mut state);
        assert!(state.schematic.selection.has_component(10));
        assert!(!state.schematic.selection.has_component(20));
    }

    #[test]
    fn select_all_honors_the_persisted_schematic_selection_filter() {
        let mut state = two_sheet_state(&[10, 11], &[]);
        state.schematic.components =
            vec![Component::new(10, ComponentType::Resistor, Point::origin())];
        state.schematic.wires = vec![Wire::segment(11, Point::origin(), Point::new(20, 0))];
        state.ui.schematic_selection_filter.instances = false;

        select_all_on_active_sheet(&mut state);

        assert!(!state.schematic.selection.has_component(10));
        assert!(state.schematic.selection.has_wire(11));
    }

    #[test]
    fn topology_operation_restores_hidden_conductors_unchanged() {
        let mut state = two_sheet_state(&[10, 11], &[20, 21]);
        let origin = Point::new(10, 10);
        let destination = Point::new(15, 15);
        state.schematic.wires = vec![
            Wire::segment(20, origin, Point::new(30, 10)),
            Wire::segment(10, origin, Point::new(10, 30)),
        ];
        state.schematic.junctions = vec![Junction::new(21, origin), Junction::new(11, origin)];
        let hidden_wire = state.schematic.wires[0].clone();
        let hidden_junction = state.schematic.junctions[0];

        assert!(with_active_wire_topology(&mut state, |schematic| {
            schematic.move_all_vertices_at(origin, destination)
        }));

        assert_eq!(
            state.schematic.wires.iter().find(|wire| wire.id == 20),
            Some(&hidden_wire)
        );
        assert_eq!(
            state
                .schematic
                .junctions
                .iter()
                .find(|junction| junction.id == 21),
            Some(&hidden_junction)
        );
        assert!(
            state
                .schematic
                .wires
                .iter()
                .find(|wire| wire.id == 10)
                .is_some_and(|wire| wire.points.contains(&destination))
        );
        assert!(
            state
                .schematic
                .junctions
                .iter()
                .find(|junction| junction.id == 11)
                .is_some_and(|junction| junction.pos == destination)
        );
    }

    #[test]
    fn self_undoing_edit_preserves_hidden_objects_and_complete_undo_snapshot() {
        let mut state = two_sheet_state(&[10], &[21]);
        state.schematic.wires = vec![Wire::segment(10, Point::origin(), Point::new(20, 0))];
        state.schematic.junctions = vec![Junction::new(21, Point::new(200, 200))];
        state.schematic.selection.select_wire(10);
        state.schematic.init_undo_history();

        assert!(with_hidden_wire_topology_preserved(
            &mut state,
            |schematic| schematic.delete_selection()
        ));
        assert!(state.schematic.wires.is_empty());
        assert_eq!(
            state.schematic.junctions,
            vec![Junction::new(21, Point::new(200, 200))]
        );

        assert!(state.schematic.undo());
        assert!(state.schematic.wires.iter().any(|wire| wire.id == 10));
        assert_eq!(
            state.schematic.junctions,
            vec![Junction::new(21, Point::new(200, 200))]
        );
    }
}
