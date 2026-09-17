//! What the schematic selection context menu promises, and to whom.
//!
//! The catalog, its availability predicates, the keyboard contract, and the
//! destination each row hands off to. Split from `context_menu.rs` because a
//! command contract and the evidence that it holds are separate concerns.

use super::*;

use crate::state::{
    Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType, DesignNote,
    DesignNoteKind, DocumentationShape, DocumentationShapeGeometry, Junction, LibraryCellInstance,
    NetLabel, Wire,
};

fn pointer_viewport() -> Viewport {
    Viewport {
        offset: egui::Pos2::ZERO,
        zoom: 1.0,
        bounds: Rect::from_min_size(egui::Pos2::ZERO, vec2(400.0, 400.0)),
    }
}

fn context_key_input(key: Key) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(640.0, 480.0))),
        focused: true,
        events: vec![egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }],
        ..Default::default()
    }
}

#[test]
fn command_catalog_matches_the_mockup_exactly() {
    let labels: Vec<_> = CONTEXT_ENTRIES
        .iter()
        .filter_map(|entry| match entry {
            ContextEntry::Command(command) => Some((command.label, command.shortcut_command)),
            ContextEntry::Separator => None,
        })
        .collect();
    assert_eq!(
        labels,
        vec![
            ("Object properties…", Some(Command::ObjectProperties)),
            ("Rotate 90°", Some(Command::RotateSelection)),
            ("Mirror", Some(Command::MirrorSelectionHorizontal)),
            ("Copy selection", Some(Command::Copy)),
            ("Duplicate and place", Some(Command::Duplicate)),
            ("Delete selection…", Some(Command::Delete)),
            (
                "Descend into selected instance",
                Some(Command::DescendHierarchyDirect),
            ),
            (
                "Update instance interface",
                Some(Command::UpdateInstanceInterface),
            ),
            ("Replace instance…", Some(Command::ReplaceInstance)),
            (
                "Create hierarchy from selection…",
                Some(Command::CreateHierarchy),
            ),
            ("Create symbol from schematic ports…", None),
            ("Page setup…", Some(Command::PageSetup)),
            ("Fit schematic content", Some(Command::FitSchematicContent),),
            ("Show in netlist", Some(Command::ShowInNetlist)),
            ("Add voltage or current probe…", Some(Command::PlaceProbe)),
            (
                "Open operating point",
                Some(Command::ResultViewer(ResultViewer::Op)),
            ),
        ]
    );
    assert_eq!(
        CONTEXT_ENTRIES
            .iter()
            .filter(|entry| matches!(entry, ContextEntry::Separator))
            .count(),
        4
    );
}

#[test]
fn header_summary_uses_the_selected_instance_identity_and_master() {
    let mut state = AppState::default();
    state.schematic.components.clear();
    let mut component =
        Component::new(7, ComponentType::CellInstance, Point::origin()).with_library_cell(
            LibraryCellInstance::new("vendor_analog", "OPA189_A", "schematic"),
        );
    component.name = "U1".to_owned();
    state.schematic.components.push(component);
    state.schematic.selection.select_component(7);

    let summary = selection_summary(&state, ContextTarget::Canvas);

    assert!(summary.starts_with("U1 · OPA189_A · /"));
}

#[test]
fn surface_geometry_matches_desktop_and_touch_contracts() {
    let desktop = SurfaceGeometry::for_viewport(vec2(1440.0, 900.0), ContextInvocation::Pointer);
    assert_eq!(desktop.width, 286.0);
    assert_eq!(desktop.max_height, 520.0);
    assert_eq!(desktop.row_height, 27.0);
    assert_eq!(desktop.radius, 3);
    // Sixteen rows and four separators on the mockup's 27 px row measure
    // 517 px: 47 header + 432 rows + 36 separators + 2 border.
    assert_eq!(desktop.outer_height(), 517.0);
    // `outer_height` clamps to `max_height`, so measuring strictly under
    // the ceiling is the same statement as "no row is below the fold".
    // A future entry that would turn the menu into a scroller fails here
    // rather than silently hiding the entries it pushed past the edge.
    assert!(
        desktop.outer_height() < desktop.max_height,
        "the desktop context menu must fit without scrolling"
    );

    let touch = SurfaceGeometry::for_viewport(vec2(390.0, 844.0), ContextInvocation::TouchSheet);
    assert_eq!(touch.width, 374.0);
    assert_eq!(touch.max_height, 560.0);
    assert_eq!(touch.row_height, 44.0);
    assert_eq!(touch.radius, 7);
    assert_eq!(touch.outer_height(), 560.0);

    let short_touch =
        SurfaceGeometry::for_viewport(vec2(1024.0, 500.0), ContextInvocation::TouchSheet);
    assert_eq!(short_touch.width, 420.0);
    assert_eq!(short_touch.max_height, 350.0);
    assert_eq!(short_touch.outer_height(), 350.0);
}

#[test]
fn desktop_origin_and_keyboard_anchor_match_the_mockup_contract() {
    let screen = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0));
    let desktop = SurfaceGeometry::for_viewport(screen.size(), ContextInvocation::Pointer);

    assert_eq!(
        clamp_desktop_surface_origin(screen, pos2(-20.0, -10.0), desktop),
        pos2(6.0, 6.0)
    );
    // A click near the bottom edge lifts the whole 517 px surface so it
    // hangs off neither the right edge nor the bottom: 600 - 517 - 6.
    assert_eq!(
        clamp_desktop_surface_origin(screen, pos2(790.0, 590.0), desktop),
        pos2(508.0, 77.0)
    );
    assert_eq!(
        keyboard_surface_anchor(Rect::from_min_size(pos2(100.0, 200.0), vec2(1000.0, 500.0),)),
        pos2(124.0, 224.0)
    );
    assert_eq!(
        keyboard_surface_anchor(Rect::from_min_size(pos2(100.0, 200.0), vec2(20.0, 10.0),)),
        pos2(110.0, 205.0)
    );
}

#[test]
fn focused_keyboard_context_row_activates_with_enter_or_space() {
    for key in [Key::Enter, Key::Space] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = AppState::default();
        state.schematic.components.clear();
        state.schematic.components.push(Component::new(
            7,
            ComponentType::Resistor,
            Point::origin(),
        ));
        state.schematic.selection.select_only_component(7);
        state.dialogs.interaction.context_target = Some((ContextTarget::Component(7), (0, 0)));
        let symbol_context = SchematicSymbolContext::from_state(&state);

        let _ = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_context_contents(ui, &mut state, &symbol_context, DESKTOP_ROW_HEIGHT, true);
            });
        });

        let mut key_still_available = true;
        let _ = ctx.run_ui(context_key_input(key), |root| {
            egui::CentralPanel::default().show(root, |ui| {
                render_context_contents(ui, &mut state, &symbol_context, DESKTOP_ROW_HEIGHT, false);
                key_still_available = ui
                    .ctx()
                    .input_mut(|input| input.consume_key(Modifiers::NONE, key));
            });
        });

        assert!(
            state.tabbed_property_dialog.open || state.dialogs.object_properties.open,
            "{key:?} should activate the focused Object properties row"
        );
        assert!(
            !key_still_available,
            "{key:?} should be owned by the focused context-menu row"
        );
    }
}

#[test]
fn context_shadow_matches_dark_and_light_mockup_tokens() {
    let dark = Tokens::new(
        tokens::Direction::Instrument,
        tokens::Mode::Dark,
        tokens::Density::default(),
    );
    let light = Tokens::new(
        tokens::Direction::Instrument,
        tokens::Mode::Light,
        tokens::Density::default(),
    );

    assert_eq!(
        context_shadow(&dark),
        Shadow {
            offset: [0, 16],
            blur: 40,
            spread: 0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 97),
        }
    );
    assert_eq!(
        context_shadow(&light),
        Shadow {
            offset: [0, 16],
            blur: 40,
            spread: 0,
            color: Color32::from_rgba_premultiplied(9, 10, 11, 56),
        }
    );
}

#[test]
fn actions_are_truthfully_disabled_without_a_compatible_selection() {
    let mut state = AppState::default();
    assert!(!action_availability(ContextAction::Properties, &state).0);
    assert!(!action_availability(ContextAction::Rotate, &state).0);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    assert!(!action_availability(ContextAction::Delete, &state).0);
    assert!(action_availability(ContextAction::Probe, &state).0);
    assert!(!action_availability(ContextAction::OperatingPoint, &state).0);

    state.schematic.selection.select_wire_segment(17, 0);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    assert!(!action_availability(ContextAction::Duplicate, &state).0);
    assert!(!action_availability(ContextAction::Delete, &state).0);

    state.schematic.selection.select_component(999);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    state.schematic.read_only = true;
    assert!(!action_availability(ContextAction::Probe, &state).0);

    let mut junction_state = AppState::default();
    let point = Point::new(4, 4);
    junction_state
        .schematic
        .junctions
        .push(Junction::new(81, point));
    junction_state
        .schematic
        .selection
        .select_only_junction(point);
    assert!(action_availability(ContextAction::Delete, &junction_state).0);
    assert!(action_availability(ContextAction::Copy, &junction_state).0);
    assert!(!action_availability(ContextAction::Duplicate, &junction_state).0);
}

#[test]
fn properties_context_action_is_available_for_one_live_bus_or_tap() {
    let mut state = AppState::default();
    let bus = Bus::segment(
        31,
        Point::new(0, 0),
        Point::new(20, 0),
        Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
    )
    .unwrap();
    let tap = BusTap::new(
        32,
        &bus,
        Point::new(5, 0),
        Point::new(5, 5),
        BusSlice::parse("DATA[3]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    state.schematic.buses.push(bus);
    state.schematic.bus_taps.push(tap);

    state.schematic.selection.select_only_bus(31);
    assert!(action_availability(ContextAction::Properties, &state).0);
    state.schematic.selection.select_only_bus_tap(32);
    assert!(action_availability(ContextAction::Properties, &state).0);

    state.schematic.read_only = true;
    assert!(!action_availability(ContextAction::Properties, &state).0);
}

#[test]
fn pointer_target_prefers_a_junction_over_its_underlying_wire() {
    let mut state = AppState::default();
    let point = Point::new(10, 10);
    state.schematic.wires = vec![Wire::new(17, vec![Point::new(0, 10), Point::new(20, 10)])];
    state.schematic.junctions = vec![Junction::new(18, point)];
    let symbol_context = SchematicSymbolContext::from_state(&state);
    let ctx = Context::default();
    let viewport = pointer_viewport();

    let target = select_pointer_target(
        &mut state,
        PointerHit::new(point, point),
        1,
        &symbol_context,
        &ctx,
        &viewport,
        pos2(point.x as f32, point.y as f32),
    );

    assert!(matches!(target, ContextTarget::Canvas));
    assert_eq!(state.schematic.selection.single_junction(), Some(point));
    assert!(state.schematic.selection.wires.is_empty());
}

#[test]
fn net_label_context_exposes_the_complete_object_lifecycle() {
    let mut state = AppState::default();
    let label = NetLabel::new(73, Point::new(40, 40), "afe_out");
    state.schematic.net_labels.push(label.clone());
    state.schematic.selection.select_only_net_label(label.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    assert!(selection_summary(&state, ContextTarget::Canvas).contains("net label · afe_out"));

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbol_context = SchematicSymbolContext::from_state(&state);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            execute_context_action(
                ContextAction::Properties,
                ui,
                &mut state,
                label.pos,
                &symbol_context,
            );
        });
    });
    assert!(state.dialogs.object_properties.open);
    assert!(!state.dialogs.rename_selection.open);
    assert!(matches!(
        state.dialogs.object_properties.draft,
        Some(crate::workbench::app::ObjectPropertiesDraft::NetLabel(ref draft))
            if draft.original == label
    ));
}

#[test]
fn design_note_context_exposes_only_compatible_object_lifecycle_actions() {
    let mut state = AppState::default();
    let note = DesignNote::new(
        74,
        Point::new(40, 40),
        DesignNoteKind::ReviewNote,
        "Review bias path",
    )
    .unwrap();
    state.schematic.design_notes.push(note.clone());
    state.schematic.selection.select_only_design_note(note.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    assert!(!action_availability(ContextAction::Rotate, &state).0);
    assert!(!action_availability(ContextAction::Mirror, &state).0);
    let summary = selection_summary(&state, ContextTarget::Canvas);
    assert!(summary.contains("Review note"));
    assert!(summary.contains("Review bias path"));

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbol_context = SchematicSymbolContext::from_state(&state);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            execute_context_action(
                ContextAction::Properties,
                ui,
                &mut state,
                note.pos,
                &symbol_context,
            );
        });
    });
    assert!(matches!(
        state.dialogs.object_properties.draft,
        Some(crate::workbench::app::ObjectPropertiesDraft::DesignNote(ref draft))
            if draft.original == note
    ));
}

#[test]
fn documentation_shape_context_exposes_the_complete_non_electrical_lifecycle() {
    let mut state = AppState::default();
    let shape = DocumentationShape::new(
        75,
        DocumentationShapeGeometry::Callout {
            tip: Point::new(10, 10),
            elbow: Point::new(30, 20),
            box_corner: Point::new(80, 50),
        },
    )
    .unwrap();
    state.schematic.documentation_shapes.push(shape.clone());
    state
        .schematic
        .selection
        .select_only_documentation_shape(shape.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    assert!(!action_availability(ContextAction::Rotate, &state).0);
    assert!(!action_availability(ContextAction::Mirror, &state).0);
    assert!(selection_summary(&state, ContextTarget::Canvas).contains("Callout"));

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbol_context = SchematicSymbolContext::from_state(&state);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            execute_context_action(
                ContextAction::Properties,
                ui,
                &mut state,
                Point::new(30, 20),
                &symbol_context,
            );
        });
    });
    assert!(matches!(
        state.dialogs.object_properties.draft,
        Some(crate::workbench::app::ObjectPropertiesDraft::DocumentationShape(ref draft))
            if draft.original == shape
    ));
}

#[test]
fn duplicate_and_delete_are_real_undoable_transactions() {
    let mut state = AppState::default();
    state.schematic.components.clear();
    state.schematic.wires.clear();
    state.schematic.init_undo_history();
    let mut component = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    component.name = "R1".to_owned();
    state.schematic.components.push(component);
    state.schematic.selection.select_component(41);

    duplicate_selection_at(&mut state, Point::new(20, 30));
    assert_eq!(state.schematic.components.len(), 2);
    assert!(state.schematic.can_undo());
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.components.len(), 1);

    state.schematic.selection.select_component(41);
    let request = DeleteSelectionRequest {
        selection: state.schematic.selection.clone(),
        topology_version: state.schematic.topology_version(),
        expected_junctions: Vec::new(),
        expected_design_notes: Vec::new(),
        expected_documentation_shapes: Vec::new(),
    };
    apply_delete_request(&mut state, request);
    assert!(state.schematic.components.is_empty());
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.components.len(), 1);

    let label = NetLabel::new(73, Point::new(40, 40), "afe_out");
    state.schematic.net_labels.push(label.clone());
    state.schematic.selection.select_only_net_label(label.id);

    duplicate_selection_at(&mut state, label.pos);
    assert_eq!(state.schematic.net_labels.len(), 2);
    assert!(state.schematic.can_undo());
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.net_labels, vec![label.clone()]);

    state.schematic.selection.select_only_net_label(label.id);
    let request = DeleteSelectionRequest {
        selection: state.schematic.selection.clone(),
        topology_version: state.schematic.topology_version(),
        expected_junctions: Vec::new(),
        expected_design_notes: Vec::new(),
        expected_documentation_shapes: Vec::new(),
    };
    apply_delete_request(&mut state, request);
    assert!(state.schematic.net_labels.is_empty());
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.net_labels, vec![label]);
}

#[test]
fn stale_delete_review_fails_closed() {
    let mut state = AppState::default();
    state.schematic.components.clear();
    let component = Component::new(9, ComponentType::Capacitor, Point::new(0, 0));
    state.schematic.components.push(component);
    state.schematic.selection.select_component(9);
    let request = DeleteSelectionRequest {
        selection: state.schematic.selection.clone(),
        topology_version: state.schematic.topology_version(),
        expected_junctions: Vec::new(),
        expected_design_notes: Vec::new(),
        expected_documentation_shapes: Vec::new(),
    };
    state.schematic.bump_topology_version();

    apply_delete_request(&mut state, request);

    assert_eq!(state.schematic.components.len(), 1);
}

#[test]
fn documentation_shape_delete_review_fails_closed_on_non_topological_drift() {
    let mut state = AppState::default();
    let shape = DocumentationShape::new(
        93,
        DocumentationShapeGeometry::Line {
            start: Point::new(0, 0),
            end: Point::new(20, 10),
        },
    )
    .unwrap();
    state.schematic.documentation_shapes.push(shape.clone());
    state
        .schematic
        .selection
        .select_only_documentation_shape(shape.id);
    let request = DeleteSelectionRequest {
        selection: state.schematic.selection.clone(),
        topology_version: state.schematic.topology_version(),
        expected_junctions: Vec::new(),
        expected_design_notes: Vec::new(),
        expected_documentation_shapes: vec![shape],
    };
    state.schematic.documentation_shapes[0].translate(Point::new(1, 0));

    apply_delete_request(&mut state, request);

    assert_eq!(state.schematic.documentation_shapes.len(), 1);
}

#[test]
fn delete_review_owns_modal_shortcuts_and_fails_closed_without_payload() {
    let ctx = Context::default();
    let mut state = AppState::default();
    state.schematic.selection.select_component(23);
    state.schematic.selection.select_wire_segment(17, 0);
    state.schematic.selection.select_junction(Point::new(5, 8));
    request_delete_confirmation(&ctx, &mut state);

    assert!(state.dialogs.interaction.schematic_delete_confirmation_open);
    assert!(state.dialogs.application_modal_open());
    let request = ctx
        .data(|data| data.get_temp::<DeleteSelectionRequest>(delete_request_id()))
        .expect("delete request is retained");
    assert!(request.selection.has_component(23));
    assert!(request.selection.wire_segments.is_empty());
    assert!(request.selection.has_junction(Point::new(5, 8)));

    ctx.data_mut(|data| data.remove::<DeleteSelectionRequest>(delete_request_id()));
    let symbol_context = SchematicSymbolContext::from_state(&state);
    assert!(!show_delete_confirmation(&ctx, &mut state, &symbol_context,));
    assert!(!state.dialogs.interaction.schematic_delete_confirmation_open);
}

/// The canvas row asks the schematic the same staleness question the
/// Design command asks, and runs the same repair. A row with its own
/// answer would offer the repair where the deck does not need it.
#[test]
fn the_interface_repair_row_is_offered_and_runs_only_for_a_stale_instance() {
    const MASTER: &str = "work/div/schematic";

    let mut state = AppState::default();
    let mut master = crate::state::SchematicState::default();
    let port = master.add_component(ComponentType::Port, Point::new(20, 0));
    master
        .components
        .iter_mut()
        .find(|component| component.id == port)
        .expect("the placed port is retained")
        .value = "a".to_owned();
    let mut binding = LibraryCellInstance::new("work", "div", "schematic");
    binding.bind_interface(&master.interface_ports());
    state
        .workspace
        .schematic_buffers
        .insert(MASTER.to_owned(), master);
    let instance = state
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    state.schematic.selection.select_only_component(instance);

    assert!(
        !action_availability(ContextAction::UpdateInstanceInterface, &state).0,
        "a placement that still matches its master is not stale"
    );

    state
        .workspace
        .schematic_buffers
        .get_mut(MASTER)
        .expect("the fixture registers the master")
        .components
        .iter_mut()
        .find(|component| component.value == "a")
        .expect("the master declares port a")
        .value = "ain".to_owned();

    assert!(action_availability(ContextAction::UpdateInstanceInterface, &state).0);

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbol_context = SchematicSymbolContext::from_state(&state);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            execute_context_action(
                ContextAction::UpdateInstanceInterface,
                ui,
                &mut state,
                Point::origin(),
                &symbol_context,
            );
        });
    });

    assert_eq!(
        state.schematic.components[0]
            .library_cell
            .as_ref()
            .expect("the instance stays bound")
            .terminal_order,
        ["ain"]
    );
    assert!(!action_availability(ContextAction::UpdateInstanceInterface, &state).0);
}

/// The replacement row is offered exactly where the command is, and opens
/// the same reviewed transaction. A row with its own answer would offer a
/// replacement the dialog would immediately refuse.
#[test]
fn the_replace_instance_row_is_offered_and_runs_only_for_one_replaceable_instance() {
    let mut state = AppState::default();
    assert!(
        !action_availability(ContextAction::ReplaceInstance, &state).0,
        "no selection is not one replaceable instance"
    );

    let first = state
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(40, 0));
    let second = state
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(120, 0));
    state.schematic.selection.select_only_component(first);
    assert!(action_availability(ContextAction::ReplaceInstance, &state).0);
    assert_eq!(
        action_availability(ContextAction::ReplaceInstance, &state).0,
        replace_instance_available(&state),
        "the row must answer with the command's own predicate"
    );

    state.schematic.selection.select_component(second);
    assert!(
        !action_availability(ContextAction::ReplaceInstance, &state).0,
        "two selected instances are not one replaceable instance"
    );

    state.schematic.selection.select_only_component(first);
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let symbol_context = SchematicSymbolContext::from_state(&state);
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            execute_context_action(
                ContextAction::ReplaceInstance,
                ui,
                &mut state,
                Point::origin(),
                &symbol_context,
            );
        });
    });

    assert!(state.dialogs.replace_instance.open);
    assert_eq!(state.dialogs.replace_instance.source_component_id, first);
}

/// One reported device, selected on the canvas.
fn state_with_reported_device_op(device: &str) -> AppState {
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};

    let mut state = AppState::default();
    let mut component = Component::new(1, ComponentType::Nmos, Point::new(40, 30));
    component.name = device.to_owned();
    state.schematic.components.push(component);
    state.schematic.selection.select_only_component(1);

    let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
    analysis.device_op = Some(rspice_core::circuit::DeviceOpReport {
        entries: vec![rspice_core::circuit::DeviceOpEntry {
            name: device.to_owned(),
            device_kind: "MOSFET",
            region: Some("saturation"),
            params: vec![("id", 1.0e-4)],
        }],
    });
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.simulation.runs.insert(0, run);
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    state
}

#[test]
fn operating_point_hop_carries_the_clicked_device_into_the_op_inspector() {
    let mut state = state_with_reported_device_op("M1");
    assert!(action_availability(ContextAction::OperatingPoint, &state).0);

    open_operating_point(&mut state);

    assert_eq!(state.workbench.workspace, Workspace::Results);
    assert_eq!(state.ui.results.viewer, ResultViewer::Op);
    assert_eq!(
        state.ui.results.op_filter, "M1",
        "the Op inspector must arrive selected on the device the menu named"
    );
}

#[test]
fn operating_point_hop_leaves_the_report_unfiltered_when_the_device_is_unreported() {
    let mut state = state_with_reported_device_op("M1");
    // A second instance the run never reported. Filtering to it would show
    // an empty inspector instead of the report the reader asked to open.
    let mut unreported = Component::new(2, ComponentType::Nmos, Point::new(80, 30));
    unreported.name = "M2".to_owned();
    state.schematic.components.push(unreported);
    state.schematic.selection.select_only_component(2);

    // A stale filter is the failure this guards: the previous device's name
    // was left in place, so the inspector opened on another instance's row
    // and read as a hop that had worked.
    state.ui.results.op_filter = "M1".to_owned();

    open_operating_point(&mut state);

    assert_eq!(state.workbench.workspace, Workspace::Results);
    assert_eq!(state.ui.results.viewer, ResultViewer::Op);
    assert!(state.ui.results.op_filter.is_empty());
}

/// The hop selects the operating point, not just the workspace and viewer.
///
/// The Op inspector renders the *selected* analysis. A run of [OP, TRAN] with
/// the transient selected — which is where a reader usually is when they go
/// looking for a bias point — landed on "not a DC operating-point result": the
/// workspace and the viewer were right and the one thing that decides what they
/// show was left wherever it had been.
#[test]
fn operating_point_hop_selects_the_operating_point_analysis() {
    use crate::state::{AnalysisResult, AnalysisType};

    let mut state = state_with_reported_device_op("M1");
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    state
        .simulation
        .runs
        .first_mut()
        .expect("the fixture run is retained")
        .add_analysis(transient);
    state.simulation.active_analysis_idx = Some(1);
    assert_eq!(
        state
            .simulation
            .active_analysis()
            .expect("the fixture selects an analysis")
            .analysis_type,
        AnalysisType::Transient,
        "the reader starts on the transient"
    );

    open_operating_point(&mut state);

    assert_eq!(
        state
            .simulation
            .active_analysis()
            .expect("the hop leaves an analysis selected")
            .analysis_type,
        AnalysisType::DcOp,
        "the hop must select the result its viewer can render"
    );
    assert_eq!(state.ui.results.op_filter, "M1");
}
