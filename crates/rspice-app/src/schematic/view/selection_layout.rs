//! Validate editor layout requests and apply one connected-movement undo unit.

use super::{SchematicSymbolContext, requests::editor_request_source, schematic_symbol_context};
use crate::state::Point;
use crate::workbench::app_state::AppState;
use rspice_schematic_editor::view::selection_layout::{
    self, SelectionLayoutObject, SelectionLayoutRequest, SelectionLayoutView,
};
pub(crate) use rspice_schematic_editor::view::selection_layout::{
    SelectionLayoutCommand, SelectionLayoutError,
};

fn view<'a>(state: &'a AppState, symbols: &'a SchematicSymbolContext) -> SelectionLayoutView<'a> {
    SelectionLayoutView {
        design: super::schematic_design_view(state),
        selection: &state.schematic.session.editor.selection,
        symbols,
        can_edit: !state.schematic_edit_read_only(),
    }
}

pub(crate) fn selection_layout_availability(
    state: &AppState,
    command: SelectionLayoutCommand,
) -> Result<(), SelectionLayoutError> {
    let symbols = schematic_symbol_context(state);
    selection_layout::availability(&view(state, &symbols), command)
}

pub(crate) fn apply_selection_layout(
    state: &mut AppState,
    symbols: &SchematicSymbolContext,
    command: SelectionLayoutCommand,
) -> Result<bool, SelectionLayoutError> {
    let request =
        selection_layout::prepare(&view(state, symbols), editor_request_source(state), command)?;
    apply_request(state, symbols, request)
}

fn apply_request(
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    request: SelectionLayoutRequest,
) -> Result<bool, SelectionLayoutError> {
    if state.schematic_edit_read_only() {
        return Err(SelectionLayoutError::ReadOnly);
    }
    if request.source != editor_request_source(state)
        || request.selection != state.schematic.session.editor.selection
    {
        return Err(SelectionLayoutError::StaleSelection);
    }
    let command = request.command;
    let deltas = request.moves;
    if deltas.iter().all(|edit| edit.delta == Point::origin()) {
        return Ok(false);
    }

    state.schematic.begin_operation(command.label());
    for edit in deltas {
        let delta = edit.delta;
        match edit.object {
            SelectionLayoutObject::Component(id) => state
                .schematic
                .move_component_with_wires_resolved(id, delta, |component| {
                    symbol_context.terminal_points(component)
                }),
            SelectionLayoutObject::DesignNote(id) => {
                state.schematic.translate_layout_design_note(id, delta)
            }
            SelectionLayoutObject::DocumentationShape(id) => state
                .schematic
                .translate_layout_documentation_shape(id, delta),
            SelectionLayoutObject::Probe(id) => state.schematic.translate_layout_probe(id, delta),
        }
    }
    state.schematic.session.is_dirty = true;
    let recorded = state.schematic.end_operation();
    if recorded {
        state.sync_active_schematic_to_workspace();
    }
    Ok(recorded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Component, ComponentType, SchematicProbe, SheetDefinition, SheetPortPolicy, SheetTemplate,
    };

    fn selected_components(positions: &[(u64, i32, i32)]) -> AppState {
        let mut state = AppState::default();
        for &(id, x, y) in positions {
            state
                .schematic
                .document_mut_for_test()
                .components
                .push(Component::new(
                    id,
                    ComponentType::Resistor,
                    Point::new(x, y),
                ));
            state
                .schematic
                .session
                .editor
                .selection
                .select_component(id);
        }
        state.schematic.init_undo_history();
        state
    }

    #[test]
    fn all_alignment_edges_are_deterministic_and_one_undo_unit() {
        for command in [
            SelectionLayoutCommand::AlignLeft,
            SelectionLayoutCommand::AlignCenter,
            SelectionLayoutCommand::AlignRight,
            SelectionLayoutCommand::AlignTop,
            SelectionLayoutCommand::AlignMiddle,
            SelectionLayoutCommand::AlignBottom,
        ] {
            let mut state = selected_components(&[(3, 70, 90), (1, 10, 10), (2, 40, 50)]);
            let original: Vec<Point> = state
                .schematic
                .document()
                .components
                .iter()
                .map(|component| component.pos)
                .collect();
            let context = schematic_symbol_context(&state);
            let terminal = context.terminal_points(&state.schematic.document().components[0])[0];
            let original_wire = crate::state::Wire::segment(90, terminal, Point::new(300, 300));
            state
                .schematic
                .document_mut_for_test()
                .wires
                .push(original_wire.clone());
            state.schematic.init_undo_history();

            assert!(apply_selection_layout(&mut state, &context, command).unwrap());
            let aligned: Vec<_> = state
                .schematic
                .document()
                .components
                .iter()
                .map(|component| {
                    let (min, max) = context.component_bounds(component);
                    match command {
                        SelectionLayoutCommand::AlignLeft => min.x,
                        SelectionLayoutCommand::AlignCenter => (min.x + max.x) / 2,
                        SelectionLayoutCommand::AlignRight => max.x,
                        SelectionLayoutCommand::AlignTop => min.y,
                        SelectionLayoutCommand::AlignMiddle => (min.y + max.y) / 2,
                        SelectionLayoutCommand::AlignBottom => max.y,
                        _ => unreachable!(),
                    }
                })
                .collect();
            assert!(
                aligned.windows(2).all(|pair| pair[0] == pair[1]),
                "{command:?}"
            );
            assert_eq!(
                state.schematic.document().wires[0].points[0],
                context.terminal_points(&state.schematic.document().components[0])[0]
            );
            assert_eq!(state.schematic.undo_description(), Some(command.label()));
            assert!(state.schematic.undo());
            assert_eq!(state.schematic.document().wires[0], original_wire);
            assert_eq!(
                state
                    .schematic
                    .document()
                    .components
                    .iter()
                    .map(|component| component.pos)
                    .collect::<Vec<_>>(),
                original
            );
            assert!(
                !state.schematic.can_undo(),
                "layout command is one undo unit"
            );
        }
    }

    #[test]
    fn distribution_uses_equal_visible_gaps_and_stable_tie_breaking() {
        for horizontal in [true, false] {
            let mut state =
                selected_components(&[(30, 80, 70), (10, 0, 10), (20, 30, 40), (15, 30, 25)]);
            let context = schematic_symbol_context(&state);

            assert!(
                apply_selection_layout(
                    &mut state,
                    &context,
                    if horizontal {
                        SelectionLayoutCommand::DistributeHorizontal
                    } else {
                        SelectionLayoutCommand::DistributeVertical
                    },
                )
                .unwrap()
            );
            let context = schematic_symbol_context(&state);
            let mut bounds: Vec<_> = state
                .schematic
                .document()
                .components
                .iter()
                .map(|component| {
                    let (min, max) = context.component_bounds(component);
                    if horizontal {
                        (min.x, max.x, component.id)
                    } else {
                        (min.y, max.y, component.id)
                    }
                })
                .collect();
            bounds.sort_by_key(|&(min, _, id)| (min, id));
            let gaps: Vec<_> = bounds
                .windows(2)
                .map(|pair| pair[1].0 - pair[0].1)
                .collect();
            assert!(gaps.windows(2).all(|pair| (pair[0] - pair[1]).abs() <= 1));
        }
    }

    #[test]
    fn layout_requests_recheck_document_occurrence_sheet_selection_and_permission() {
        for change in [
            "none",
            "document",
            "occurrence",
            "sheet",
            "selection",
            "content",
            "read-only",
        ] {
            let mut state = selected_components(&[(1, 0, 0), (2, 40, 30)]);
            let master = crate::state::CellViewRef::new("work", "layout_child", "schematic");
            state.workspace.descend_into(
                "X1".to_owned(),
                master.clone(),
                crate::state::ViewType::Schematic,
            );
            let first = state
                .workspace
                .content
                .design_management
                .bootstrap_for_cell_view(&master.key(), "Sheet 1", [1, 2])
                .unwrap();
            let symbols = schematic_symbol_context(&state);
            let request = selection_layout::prepare(
                &view(&state, &symbols),
                editor_request_source(&state),
                SelectionLayoutCommand::AlignLeft,
            )
            .unwrap();
            match change {
                "document" => state.active_schematic_epoch += 1,
                "occurrence" => {
                    state.workspace.ascend_one().unwrap();
                    state.workspace.descend_into(
                        "X2".to_owned(),
                        master.clone(),
                        crate::state::ViewType::Schematic,
                    );
                }
                "sheet" => {
                    let catalog = state
                        .workspace
                        .content
                        .design_management
                        .sheet_catalog_mut(&master.key())
                        .unwrap();
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
                        .unwrap();
                    catalog.set_active(second).unwrap();
                }
                "selection" => state.schematic.session.editor.selection.clear(),
                "content" => state.schematic.bump_topology_version(),
                "read-only" => state.schematic.session.read_only = true,
                "none" => {}
                _ => unreachable!(),
            }
            let before = state.schematic.document().components.clone();
            let outcome = apply_request(&mut state, &symbols, request);
            if change == "none" {
                assert_eq!(outcome, Ok(true));
                assert_eq!(state.schematic.document().components[1].pos.x, 0);
                assert!(state.schematic.undo());
            } else {
                assert!(outcome.is_err(), "{change}");
            }
            assert_eq!(state.schematic.document().components, before, "{change}");
            assert!(!state.schematic.can_undo(), "{change}");
        }
    }

    #[test]
    fn incompatible_and_read_only_selections_fail_without_undo() {
        let mut state = selected_components(&[(1, 0, 0), (2, 40, 0)]);
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::new(
                9,
                vec![Point::origin(), Point::new(10, 0)],
            ));
        state.schematic.session.editor.selection.select_wire(9);
        let context = schematic_symbol_context(&state);
        assert_eq!(
            apply_selection_layout(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::IncompatibleSelection)
        );
        assert!(!state.schematic.can_undo());

        state.schematic.session.editor.selection.wires.clear();
        state.schematic.session.read_only = true;
        assert_eq!(
            apply_selection_layout(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::ReadOnly)
        );
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn probe_layout_is_non_electrical_and_undoable() {
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().probes = vec![
            SchematicProbe::new(20, Point::new(10, 10), "V(out)", None).unwrap(),
            SchematicProbe::new(21, Point::new(40, 30), "V(out)", None).unwrap(),
        ];
        state.schematic.session.editor.selection.select_probe(20);
        state.schematic.session.editor.selection.select_probe(21);
        state.schematic.init_undo_history();
        let original = state.schematic.document().probes.clone();
        let topology = state.schematic.topology_version();
        let context = schematic_symbol_context(&state);

        assert!(
            apply_selection_layout(&mut state, &context, SelectionLayoutCommand::AlignLeft)
                .unwrap()
        );
        assert_eq!(state.schematic.document().probes[0].position.x, 10);
        assert_eq!(state.schematic.document().probes[1].position.x, 10);
        assert_eq!(state.schematic.topology_version(), topology);

        assert!(state.schematic.undo());
        assert_eq!(state.schematic.document().probes, original);
        assert_eq!(state.schematic.topology_version(), topology);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn cross_sheet_selection_fails_closed_instead_of_moving_a_visible_subset() {
        let mut state = selected_components(&[(1, 0, 0), (2, 40, 0)]);
        let key = state.workspace.content.active_schematic_reference().key();
        let first = state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Sheet 1", [1, 2])
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
        catalog
            .assign_objects(catalog.revision(), second, [2])
            .expect("hidden assignment");
        catalog.set_active(first).expect("active sheet");
        let context = schematic_symbol_context(&state);
        let before = state.schematic.document().components.clone();

        assert_eq!(
            apply_selection_layout(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::OffActiveSheet)
        );
        assert_eq!(state.schematic.document().components, before);
        assert!(!state.schematic.can_undo());
    }
}
