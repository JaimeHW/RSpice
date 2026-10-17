//! Snap Selection to Grid: put every selected object's anchor back on the
//! document grid, as KiCad's Align to grid does.
//!
//! A part moves to the nearest grid point with its wires re-routed square,
//! as Move's Shove mode routes them; a net label, a note, a probe or a free
//! wire moves by its own anchor. Each object moves by its own small step, the whole
//! command is one undo step, and the cell view's other sheets are out of
//! reach while the wiring is carried and tidied.

use super::sheet_visibility::{retain_selection_on_active_sheet, with_active_wire_topology};
use super::{AppState, SchematicSymbolContext};
use crate::state::{MoveSelectionMode, Point, Selection};
use crate::workbench::app::on_grid;

/// One object and the step that lands its anchor on the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Snap {
    Component(u64, Point),
    NetLabel(u64, Point),
    Wire(u64, Point),
    DesignNote(u64, Point),
    Probe(u64, Point),
}

/// Snap what is selected on the active sheet. Returns how many objects
/// moved; nothing selected off the grid moves nothing and records nothing.
pub(crate) fn snap_selection_to_grid(
    state: &mut AppState,
    symbols: &SchematicSymbolContext,
) -> Result<usize, String> {
    if state.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    retain_selection_on_active_sheet(state);
    let snaps = snaps(state, symbols);
    if snaps.is_empty() {
        return Ok(0);
    }
    let selection = state.schematic.session.editor.selection.clone();
    // The undo step opens with every sheet present; the other sheets step
    // out of reach only while the wiring is carried and tidied.
    state.schematic.begin_operation("snap selection to grid");
    let moved = with_active_wire_topology(state, |schematic, scope| {
        let terminal_points =
            |component: &crate::state::Component| symbols.terminal_points(component);
        // Each object moves alone, and the wires attached to it are
        // re-routed square rather than stretched on a slant.
        let carry =
            |schematic: &mut crate::state::SchematicState, only: Selection, delta: Point| {
                schematic.session.editor.selection = only;
                schematic.move_selection_with_mode_resolved(
                    delta,
                    MoveSelectionMode::Shove,
                    terminal_points,
                    scope,
                )
            };
        for snap in &snaps {
            let mut only = Selection::default();
            match *snap {
                Snap::Component(id, delta) => {
                    only.select_component(id);
                    carry(schematic, only, delta)?;
                }
                Snap::NetLabel(id, delta) => {
                    only.select_net_label(id);
                    carry(schematic, only, delta)?;
                }
                Snap::Wire(id, delta) => {
                    only.select_wire(id);
                    carry(schematic, only, delta)?;
                }
                Snap::DesignNote(id, delta) => schematic.translate_layout_design_note(id, delta),
                Snap::Probe(id, delta) => schematic.translate_layout_probe(id, delta),
            }
        }
        let automatic_junctions = schematic
            .document()
            .document_policy
            .wire_junctions
            .automatic_junctions();
        schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions, scope);
        Ok::<usize, rspice_design::schematic::movement::MoveSelectionError>(snaps.len())
    });
    state.schematic.session.editor.selection = selection;
    match moved {
        Ok(moved) => {
            state.schematic.session.is_dirty = true;
            if state.schematic.end_operation() {
                state.sync_active_schematic_to_workspace();
            }
            Ok(moved)
        }
        Err(error) => {
            state.schematic.cancel_operation();
            Err(format!("Snap to grid was not applied: {error}."))
        }
    }
}

/// The step each selected object's anchor needs, parts first so the wires
/// they carry are settled before a free wire is looked at.
fn snaps(state: &AppState, symbols: &SchematicSymbolContext) -> Vec<Snap> {
    let document = state.schematic.document();
    let grid = document.grid_size;
    let selection = &state.schematic.session.editor.selection;
    let step = |anchor: Point| {
        let delta = on_grid(anchor, grid) - anchor;
        (delta != Point::origin()).then_some(delta)
    };
    let mut snaps: Vec<Snap> = document
        .components
        .iter()
        .filter(|component| selection.has_component(component.id))
        .filter_map(|component| {
            step(component.pos).map(|delta| Snap::Component(component.id, delta))
        })
        .collect();
    let pins: Vec<Point> = document
        .components
        .iter()
        .flat_map(|component| symbols.terminal_points(component))
        .collect();
    // A wire that ends on a pin follows its part; one that does not is
    // carried by its first vertex.
    snaps.extend(
        document
            .wires
            .iter()
            .filter(|wire| selection.has_wire(wire.id) && wire.points.len() >= 2)
            .filter(|wire| {
                let ends = [wire.points[0], wire.points[wire.points.len() - 1]];
                !ends.iter().any(|end| pins.contains(end))
            })
            .filter_map(|wire| step(wire.points[0]).map(|delta| Snap::Wire(wire.id, delta))),
    );
    snaps.extend(
        document
            .net_labels
            .iter()
            .filter(|label| selection.has_net_label(label.id))
            .filter_map(|label| step(label.pos).map(|delta| Snap::NetLabel(label.id, delta))),
    );
    snaps.extend(
        document
            .design_notes
            .iter()
            .filter(|note| selection.has_design_note(note.id))
            .filter_map(|note| step(note.pos).map(|delta| Snap::DesignNote(note.id, delta))),
    );
    snaps.extend(
        document
            .probes
            .iter()
            .filter(|probe| selection.has_probe(probe.id))
            .filter_map(|probe| step(probe.position).map(|delta| Snap::Probe(probe.id, delta))),
    );
    snaps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Wire};

    /// A part placed off the grid, with a wire from its pin, lands on the
    /// nearest grid point; the wire follows square; one undo puts it back
    /// (audit G-8).
    #[test]
    fn an_off_grid_part_lands_on_the_grid_and_its_wire_follows_square() {
        let mut state = AppState::default();
        let grid = state.schematic.document().grid_size;
        let part = Component::new(1, ComponentType::Resistor, Point::new(103, 47));
        let symbols = super::super::schematic_symbol_context(&state);
        let pin = symbols.terminal_points(&part)[0];
        let wire = Wire::segment(2, Point::new(pin.x - 200, pin.y), pin);
        let document = state.schematic.document_mut_for_test();
        document.components = vec![part];
        document.wires = vec![wire];
        document.junctions.clear();
        state.schematic.init_undo_history();
        state.schematic.session.editor.selection.select_component(1);

        assert_eq!(snap_selection_to_grid(&mut state, &symbols), Ok(1));

        let document = state.schematic.document();
        let pos = document.components[0].pos;
        assert_eq!(pos, on_grid(Point::new(103, 47), grid));
        assert!(pos.x % grid == 0 && pos.y % grid == 0);
        let pin = symbols.terminal_points(&document.components[0])[0];
        assert!(
            document.wires.iter().any(|wire| wire.points.contains(&pin)),
            "the wire left the pin: {:?}",
            document.wires
        );
        assert!(
            document.wires.iter().all(|wire| wire
                .points
                .windows(2)
                .all(|pair| pair[0].x == pair[1].x || pair[0].y == pair[1].y)),
            "a wire went slanted: {:?}",
            document.wires
        );
        assert!(state.schematic.undo(), "snap is one undo step");
        assert_eq!(
            state.schematic.document().components[0].pos,
            Point::new(103, 47)
        );
        assert!(!state.schematic.can_undo());
        state.schematic.session.editor.selection.select_component(1);
        assert_eq!(snap_selection_to_grid(&mut state, &symbols), Ok(1));
        assert_eq!(
            snap_selection_to_grid(&mut state, &symbols),
            Ok(0),
            "on the grid already"
        );
    }

    /// Sheet 2 has a wire ending where Sheet 1's off-grid pin sits: it stays
    /// where it is when the part is snapped.
    #[test]
    fn snapping_leaves_another_sheets_wire_whole() {
        let mut state = super::super::sheet_visibility::two_sheet_state(&[1], &[2]);
        let part = Component::new(1, ComponentType::Resistor, Point::new(103, 47));
        let symbols = super::super::schematic_symbol_context(&state);
        let pin = symbols.terminal_points(&part)[0];
        let there = Wire::segment(2, Point::new(pin.x - 200, pin.y), pin);
        let document = state.schematic.document_mut_for_test();
        document.components = vec![part];
        document.wires = vec![there.clone()];
        document.junctions.clear();
        state.schematic.init_undo_history();
        state.schematic.session.editor.selection.select_component(1);

        assert_eq!(snap_selection_to_grid(&mut state, &symbols), Ok(1));
        assert_eq!(state.schematic.document().wires, vec![there]);
    }
}
