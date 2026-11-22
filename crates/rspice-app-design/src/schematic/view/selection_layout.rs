//! Validate editor layout requests and apply one connected-movement undo unit.

use super::sheet_visibility::with_active_wire_topology_in;
use super::{
    Model, SchematicSymbolContext, WorkbenchSession, model_design_view,
    requests::editor_request_source, sync_active_schematic,
};
use crate::state::{Component, Point};
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;
use rspice_schematic_editor::view::selection_layout::{
    self, SelectionLayoutObject, SelectionLayoutRequest, SelectionLayoutView,
};
pub(crate) use rspice_schematic_editor::view::selection_layout::{
    SelectionLayoutCommand, SelectionLayoutError,
};

fn view<'a>(
    model: &'a Model,
    review_markers: SchematicReviewMarkerVisibility,
    symbols: &'a SchematicSymbolContext,
) -> SelectionLayoutView<'a> {
    SelectionLayoutView {
        design: model_design_view(model, review_markers),
        selection: &model.schematic.session.editor.selection,
        symbols,
        can_edit: !model.schematic_edit_read_only(),
    }
}

/// Whether `command` can lay out the selection of the drawing the model
/// holds, which shows `review_markers`, and why not.
pub(crate) fn selection_layout_availability(
    model: &Model,
    review_markers: SchematicReviewMarkerVisibility,
    command: SelectionLayoutCommand,
) -> Result<(), SelectionLayoutError> {
    let symbols = model.schematic_symbol_context();
    selection_layout::availability(&view(model, review_markers, &symbols), command)
}

pub(crate) fn apply_selection_layout(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    review_markers: SchematicReviewMarkerVisibility,
    symbols: &SchematicSymbolContext,
    command: SelectionLayoutCommand,
) -> Result<bool, SelectionLayoutError> {
    let request = selection_layout::prepare(
        &view(model, review_markers, symbols),
        editor_request_source(model),
        command,
    )?;
    apply_request(model, wb, symbols, request)
}

fn apply_request(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    symbol_context: &SchematicSymbolContext,
    request: SelectionLayoutRequest,
) -> Result<bool, SelectionLayoutError> {
    if model.schematic_edit_read_only() {
        return Err(SelectionLayoutError::ReadOnly);
    }
    if request.source != editor_request_source(model)
        || request.selection != model.schematic.session.editor.selection
    {
        return Err(SelectionLayoutError::StaleSelection);
    }
    let command = request.command;
    let deltas = request.moves;
    if deltas.iter().all(|edit| edit.delta == Point::origin()) {
        return Ok(false);
    }

    model.schematic.begin_operation(command.label());
    // A wire end on another sheet can share a moved pin's coordinates; the
    // parts carry only their own sheet's wires.
    with_active_wire_topology_in(
        &model.workspace,
        &mut model.schematic,
        |schematic, scope| {
            // The parts move together, each by its own step, as a drag moves
            // them: their wires are laid again square, what sits on their
            // pins and wires follows, a pin taken off a pin keeps a wire to
            // it, and the junction policy tidies the result.
            let parts = deltas
                .iter()
                .filter_map(|edit| match edit.object {
                    SelectionLayoutObject::Component(id) => Some((id, edit.delta)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if !parts.is_empty() {
                let terminal_points =
                    |component: &Component| symbol_context.terminal_points(component);
                schematic.move_components_with_wires_resolved(&parts, terminal_points, scope);
                let moved = parts.iter().map(|(id, _)| *id).collect();
                schematic.connect_moved_pins(&moved, terminal_points, scope);
                let automatic_junctions = schematic.session.editor.automatic_junctions;
                schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions, scope);
            }
            for edit in deltas {
                let delta = edit.delta;
                match edit.object {
                    SelectionLayoutObject::Component(_) => {}
                    SelectionLayoutObject::DesignNote(id) => {
                        schematic.translate_layout_design_note(id, delta)
                    }
                    SelectionLayoutObject::DocumentationShape(id) => {
                        schematic.translate_layout_documentation_shape(id, delta)
                    }
                    SelectionLayoutObject::Probe(id) => schematic.translate_layout_probe(id, delta),
                }
            }
        },
    );
    model.schematic.session.is_dirty = true;
    let recorded = model.schematic.end_operation();
    if recorded {
        sync_active_schematic(model, wb);
    }
    Ok(recorded)
}

#[cfg(test)]
mod tests {
    use super::super::schematic_symbol_context;
    use super::*;
    use crate::workbench::design::test_host::DesignTestHost;
    use rspice_design::connectivity::summary::net_names_by_terminal;
    use std::collections::{BTreeMap, BTreeSet};

    /// [`apply_selection_layout`] over the session's parts.
    fn apply(
        state: &mut DesignTestHost,
        symbols: &SchematicSymbolContext,
        command: SelectionLayoutCommand,
    ) -> Result<bool, SelectionLayoutError> {
        let markers = state.design.schematic_visibility.review_markers;
        apply_selection_layout(
            &mut state.model,
            &mut state.workbench,
            markers,
            symbols,
            command,
        )
    }
    use crate::state::{
        Component, ComponentType, SchematicProbe, SheetDefinition, SheetPortPolicy,
    };

    fn selected_components(positions: &[(u64, i32, i32)]) -> DesignTestHost {
        let mut state = DesignTestHost::default();
        for &(id, x, y) in positions {
            state
                .model
                .schematic
                .document_mut_for_test()
                .components
                .push(Component::new(
                    id,
                    ComponentType::Resistor,
                    Point::new(x, y),
                ));
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .select_component(id);
        }
        state.model.schematic.init_undo_history();
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
                .model
                .schematic
                .document()
                .components
                .iter()
                .map(|component| component.pos)
                .collect();
            let context = schematic_symbol_context(&state);
            let terminal =
                context.terminal_points(&state.model.schematic.document().components[0])[0];
            let original_wire = crate::state::Wire::segment(90, terminal, Point::new(300, 300));
            state
                .model
                .schematic
                .document_mut_for_test()
                .wires
                .push(original_wire.clone());
            state.model.schematic.init_undo_history();

            assert!(apply(&mut state, &context, command).unwrap());
            let aligned: Vec<_> = state
                .model
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
                state.model.schematic.document().wires[0].points[0],
                context.terminal_points(&state.model.schematic.document().components[0])[0]
            );
            assert_eq!(
                state.model.schematic.undo_description(),
                Some(command.label())
            );
            assert!(state.model.schematic.undo());
            assert_eq!(state.model.schematic.document().wires[0], original_wire);
            assert_eq!(
                state
                    .model
                    .schematic
                    .document()
                    .components
                    .iter()
                    .map(|component| component.pos)
                    .collect::<Vec<_>>(),
                original
            );
            assert!(
                !state.model.schematic.can_undo(),
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
                apply(
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
                .model
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
            // Every move is a whole number of grid steps, so the gaps are
            // equal to within one step; part 15 starts off the grid.
            let grid = state.model.schematic.document().grid_size;
            assert!(
                gaps.windows(2)
                    .all(|pair| (pair[0] - pair[1]).abs() <= grid),
                "{gaps:?}"
            );
        }
    }

    /// A source is 28 wide around its origin and a resistor 40, so lining up
    /// their edges or centres, or spacing them evenly, asks for moves that are
    /// no whole number of grid steps. Every part still lands on the grid, its
    /// pins with it, within one step of the exact layout.
    #[test]
    fn align_and_distribute_land_mixed_parts_on_the_grid() {
        use SelectionLayoutCommand::*;
        for command in [
            AlignLeft,
            AlignCenter,
            AlignRight,
            AlignTop,
            AlignMiddle,
            AlignBottom,
            DistributeHorizontal,
            DistributeVertical,
        ] {
            let vertical = matches!(
                command,
                AlignTop | AlignMiddle | AlignBottom | DistributeVertical
            );
            let mut state = DesignTestHost::default();
            for (id, kind, position) in [
                (1, ComponentType::Resistor, Point::new(100, 0)),
                (2, ComponentType::VoltageSource, Point::new(200, 100)),
                (3, ComponentType::CurrentSource, Point::new(330, 250)),
            ] {
                let mut component = Component::new(id, kind, position);
                if vertical {
                    component.rotation = crate::state::Rotation::R90;
                }
                state
                    .model
                    .schematic
                    .document_mut_for_test()
                    .components
                    .push(component);
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_component(id);
            }
            state.model.schematic.init_undo_history();
            let grid = state.model.schematic.document().grid_size;
            let context = schematic_symbol_context(&state);

            assert!(apply(&mut state, &context, command).unwrap(), "{command:?}");

            let components = &state.model.schematic.document().components;
            for component in components {
                let pins = context.terminal_points(component);
                for point in std::iter::once(component.pos).chain(pins) {
                    assert!(
                        point.x % grid == 0 && point.y % grid == 0,
                        "{command:?}: #{} has {point:?} off the {grid} grid",
                        component.id
                    );
                }
            }
            let mut spans: Vec<(i32, i32)> = components
                .iter()
                .map(|component| {
                    let (min, max) = context.component_bounds(component);
                    if vertical {
                        (min.y, max.y)
                    } else {
                        (min.x, max.x)
                    }
                })
                .collect();
            spans.sort_unstable();
            let measured: Vec<i32> = match command {
                AlignLeft | AlignTop => spans.iter().map(|span| span.0).collect(),
                AlignRight | AlignBottom => spans.iter().map(|span| span.1).collect(),
                AlignCenter | AlignMiddle => spans.iter().map(|span| span.0 + span.1).collect(),
                DistributeHorizontal | DistributeVertical => {
                    spans.windows(2).map(|pair| pair[1].0 - pair[0].1).collect()
                }
            };
            let spread = measured.iter().max().unwrap() - measured.iter().min().unwrap();
            // A centre is measured doubled.
            let tolerance = if matches!(command, AlignCenter | AlignMiddle) {
                2 * grid
            } else {
                grid
            };
            assert!(spread <= tolerance, "{command:?}: {measured:?}");
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
            state.model.workspace.descend_into(
                "X1".to_owned(),
                master.clone(),
                crate::state::ViewType::Schematic,
            );
            let first = state
                .model
                .workspace
                .content
                .design_management
                .bootstrap_for_cell_view(&master.key(), "Sheet 1", [1, 2])
                .unwrap();
            let symbols = schematic_symbol_context(&state);
            let request = selection_layout::prepare(
                &view(
                    &state.model,
                    state.design.schematic_visibility.review_markers,
                    &symbols,
                ),
                editor_request_source(&state),
                SelectionLayoutCommand::AlignLeft,
            )
            .unwrap();
            match change {
                "document" => state.model.active_schematic_epoch += 1,
                "occurrence" => {
                    state.model.workspace.ascend_one().unwrap();
                    state.model.workspace.descend_into(
                        "X2".to_owned(),
                        master.clone(),
                        crate::state::ViewType::Schematic,
                    );
                }
                "sheet" => {
                    let catalog = state
                        .model
                        .workspace
                        .content
                        .design_management
                        .sheet_catalog_mut(&master.key())
                        .unwrap();
                    let second = catalog
                        .create_sheet(
                            SheetDefinition {
                                name: "Sheet 2".to_owned(),
                                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                                explicit_page_number: Some(2),
                            },
                            Some(first),
                        )
                        .unwrap();
                    catalog.set_active(second).unwrap();
                }
                "selection" => state.model.schematic.session.editor.selection.clear(),
                "content" => state.model.schematic.bump_topology_version(),
                "read-only" => state.model.schematic.session.read_only = true,
                "none" => {}
                _ => unreachable!(),
            }
            let before = state.model.schematic.document().components.clone();
            let outcome = apply_request(&mut state.model, &mut state.workbench, &symbols, request);
            if change == "none" {
                assert_eq!(outcome, Ok(true));
                assert_eq!(state.model.schematic.document().components[1].pos.x, 0);
                assert!(state.model.schematic.undo());
            } else {
                assert!(outcome.is_err(), "{change}");
            }
            assert_eq!(
                state.model.schematic.document().components,
                before,
                "{change}"
            );
            assert!(!state.model.schematic.can_undo(), "{change}");
        }
    }

    #[test]
    fn incompatible_and_read_only_selections_fail_without_undo() {
        let mut state = selected_components(&[(1, 0, 0), (2, 40, 0)]);
        state
            .model
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::new(
                9,
                vec![Point::origin(), Point::new(10, 0)],
            ));
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_wire(9);
        let context = schematic_symbol_context(&state);
        assert_eq!(
            apply(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::IncompatibleSelection)
        );
        assert!(!state.model.schematic.can_undo());

        state.model.schematic.session.editor.selection.wires.clear();
        state.model.schematic.session.read_only = true;
        assert_eq!(
            apply(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::ReadOnly)
        );
        assert!(!state.model.schematic.can_undo());
    }

    #[test]
    fn probe_layout_is_non_electrical_and_undoable() {
        let mut state = DesignTestHost::default();
        state.model.schematic.document_mut_for_test().probes = vec![
            SchematicProbe::new(20, Point::new(10, 10), "V(out)", None).unwrap(),
            SchematicProbe::new(21, Point::new(40, 30), "V(out)", None).unwrap(),
        ];
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_probe(20);
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_probe(21);
        state.model.schematic.init_undo_history();
        let original = state.model.schematic.document().probes.clone();
        let topology = state.model.schematic.topology_version();
        let context = schematic_symbol_context(&state);

        assert!(apply(&mut state, &context, SelectionLayoutCommand::AlignLeft).unwrap());
        assert_eq!(state.model.schematic.document().probes[0].position.x, 10);
        assert_eq!(state.model.schematic.document().probes[1].position.x, 10);
        assert_eq!(state.model.schematic.topology_version(), topology);

        assert!(state.model.schematic.undo());
        assert_eq!(state.model.schematic.document().probes, original);
        assert_eq!(state.model.schematic.topology_version(), topology);
        assert!(!state.model.schematic.can_undo());
    }

    /// Probes align by what is drawn: aligned right, a short label and a
    /// long one end together on the sheet, to the grid, rather than by a box
    /// padded for status text the probes may never show.
    #[test]
    fn probes_align_by_their_drawn_labels() {
        let mut state = DesignTestHost::default();
        state.model.schematic.document_mut_for_test().probes = vec![
            SchematicProbe::new(20, Point::new(10, 10), "V(out)", None).unwrap(),
            SchematicProbe::new(21, Point::new(10, 40), "V(a_very_long_net_name)", None).unwrap(),
        ];
        let selection = &mut state.model.schematic.session.editor.selection;
        selection.select_probe(20);
        selection.select_probe(21);
        state.model.schematic.init_undo_history();
        let context = schematic_symbol_context(&state);

        assert!(apply(&mut state, &context, SelectionLayoutCommand::AlignRight).unwrap());
        let document = state.model.schematic.document();
        let ends = document
            .probes
            .iter()
            .map(|probe| probe.layout_bounds().1.x)
            .collect::<Vec<_>>();
        assert!(
            (ends[0] - ends[1]).abs() * 2 <= document.grid_size,
            "drawn label ends {ends:?} on a grid of {}",
            document.grid_size
        );
    }

    /// Sheet 2 shares Sheet 1's coordinates, not its conductors: aligning
    /// Sheet 1's parts carries their own wires and leaves Sheet 2's wire end,
    /// lying on a moved pin's old position, where it was.
    #[test]
    fn aligning_parts_leaves_another_sheets_wire_end_in_place() {
        let mut state = crate::schematic::view::sheet_visibility::two_sheet_host(&[1, 2], &[90]);
        for (id, position) in [(1, Point::new(10, 10)), (2, Point::new(40, 50))] {
            state
                .model
                .schematic
                .document_mut_for_test()
                .components
                .push(Component::new(id, ComponentType::Resistor, position));
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .select_component(id);
        }
        let context = schematic_symbol_context(&state);
        let terminal = context.terminal_points(&state.model.schematic.document().components[1])[0];
        let hidden = crate::state::Wire::segment(90, terminal, Point::new(300, 300));
        state
            .model
            .schematic
            .document_mut_for_test()
            .wires
            .push(hidden.clone());
        state.model.schematic.init_undo_history();

        assert!(apply(&mut state, &context, SelectionLayoutCommand::AlignLeft).unwrap());

        assert_ne!(
            context.terminal_points(&state.model.schematic.document().components[1])[0],
            terminal,
            "the part moved"
        );
        assert_eq!(state.model.schematic.document().wires, vec![hidden]);
    }

    #[test]
    fn cross_sheet_selection_fails_closed_instead_of_moving_a_visible_subset() {
        let mut state = selected_components(&[(1, 0, 0), (2, 40, 0)]);
        let key = state
            .model
            .workspace
            .content
            .active_schematic_reference()
            .key();
        let first = state
            .model
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&key, "Sheet 1", [1, 2])
            .expect("first sheet");
        let catalog = state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .expect("sheet catalog");
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Sheet 2".to_owned(),
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
        let before = state.model.schematic.document().components.clone();

        assert_eq!(
            apply(&mut state, &context, SelectionLayoutCommand::AlignLeft),
            Err(SelectionLayoutError::OffActiveSheet)
        );
        assert_eq!(state.model.schematic.document().components, before);
        assert!(!state.model.schematic.can_undo());
    }

    /// R1 carries the label OUT on its right pin, joined by name to the
    /// label on R3's pin, a typed tap on its left pin, a probe on its output
    /// wire and a dot where R2's wire tees into it. R5 and R6 touch pin to
    /// pin, one lying and one standing, so lining up their tops moves them
    /// by different steps.
    fn attached_parts() -> DesignTestHost {
        use crate::state::{
            Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Junction, NetLabel, Rotation,
            Wire,
        };
        let mut state = DesignTestHost::default();
        let document = state.model.schematic.document_mut_for_test();
        let part = |id, x, y, rotation| {
            let mut part = Component::new(id, ComponentType::Resistor, Point::new(x, y));
            part.rotation = rotation;
            part
        };
        document.components.extend([
            part(1, 100, 100, Rotation::R0),
            part(2, 180, 180, Rotation::R90),
            part(3, 400, 200, Rotation::R0),
            part(5, 100, 400, Rotation::R0),
            part(6, 120, 420, Rotation::R90),
            part(7, 300, 360, Rotation::R0),
            part(8, -100, 100, Rotation::R0),
        ]);
        document.wires.extend([
            Wire::segment(10, Point::new(120, 100), Point::new(240, 100)),
            Wire::segment(11, Point::new(180, 100), Point::new(180, 160)),
            Wire::segment(12, Point::new(80, 100), Point::new(80, 140)),
        ]);
        document
            .junctions
            .push(Junction::new(20, Point::new(180, 100)));
        document.net_labels.extend([
            NetLabel::new(30, Point::new(120, 100), "OUT"),
            NetLabel::new(31, Point::new(380, 200), "OUT"),
        ]);
        let bus = Bus::segment(
            40,
            Point::new(60, 60),
            Point::new(60, 140),
            Some(BusDeclaration::parse("D[1:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            41,
            &bus,
            Point::new(60, 100),
            Point::new(80, 100),
            BusSlice::parse("D[0]").unwrap(),
            BusTapOrientation::Right,
        )
        .unwrap();
        document.buses.push(bus);
        document.bus_taps.push(tap);
        document.probes.push(
            SchematicProbe::new(
                50,
                Point::new(150, 100),
                "V(OUT)",
                Some("V(OUT)".to_owned()),
            )
            .unwrap(),
        );
        state.model.schematic.init_undo_history();
        state
    }

    /// Each net as the pins on it, under its name when the drawing names it.
    fn nets(state: &DesignTestHost) -> BTreeSet<(String, BTreeSet<(u64, String)>)> {
        let mut by_name = BTreeMap::<String, BTreeSet<(u64, String)>>::new();
        for ((id, pin), name) in net_names_by_terminal(state.model.schematic.document()) {
            by_name.entry(name).or_default().insert((id, pin));
        }
        by_name
            .into_iter()
            .map(|(name, pins)| {
                let generated = name
                    .strip_prefix("net")
                    .is_some_and(|number| number.chars().all(|c| c.is_ascii_digit()));
                (if generated { String::new() } else { name }, pins)
            })
            .collect()
    }

    /// Align and Distribute move parts as a drag does: every pin keeps its
    /// net and name, so the label, tap and probe go with the pins and wires
    /// they sit on, parts that touched pin to pin stay joined, and every
    /// wire stays square.
    #[test]
    fn align_and_distribute_keep_every_pins_net_and_every_wire_square() {
        use SelectionLayoutCommand::*;
        let cases: [(SelectionLayoutCommand, &[u64]); 3] = [
            (AlignBottom, &[1, 3]),
            (DistributeHorizontal, &[8, 1, 3]),
            (AlignTop, &[5, 6, 7]),
        ];
        for (command, parts) in cases {
            let mut state = attached_parts();
            for id in parts {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_component(*id);
            }
            let context = schematic_symbol_context(&state);
            let before_positions = state.model.schematic.document().components.clone();
            let before = nets(&state);
            assert!(apply(&mut state, &context, command).unwrap(), "{command:?}");

            let document = state.model.schematic.document();
            assert_ne!(
                document.components, before_positions,
                "{command:?} moved nothing"
            );
            assert_eq!(nets(&state), before, "{command:?} changed the netlist");
            for wire in &document.wires {
                assert!(
                    wire.is_orthogonal(),
                    "{command:?} left wire {} slanted: {:?}",
                    wire.id,
                    wire.points
                );
            }
            let probe = document.probes[0].position;
            assert!(
                document.wires.iter().any(|wire| wire.contains_point(probe)),
                "{command:?} left the probe off its wire at {probe:?}"
            );
            let tap = &document.bus_taps[0];
            assert!(
                document
                    .wires
                    .iter()
                    .any(|wire| wire.contains_point(tap.connection_point)),
                "{command:?} left the tap off its wire: {tap:?}"
            );
            assert!(state.model.schematic.undo(), "{command:?} is one undo step");
        }
    }
}
