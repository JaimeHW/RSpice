//! Tests for pointer interaction on the schematic canvas.
//!
//! The cases pin what is a legitimate pointer target and what one gesture
//! costs in undo - a junction placed or removed is exactly one step, and an
//! automatic marker is not a toggle target at all.

use super::*;
use crate::simulation::netlist_gen::extraction::extract;
use crate::state::{
    Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType,
    DesignNoteKind, DocumentationShapeKind, Junction, NetLabel, PendingDesignNotePlacement,
    PendingDocumentationShapePlacement, PendingPortPlacement, PortDirection, PortDirectionType,
    PortDiscipline, PortSignalType, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
    SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SchematicProbe, SheetDefinition,
    SheetPortPolicy, SheetTemplate, Tool, WaveformData, Wire,
};

fn pointer_viewport() -> Viewport {
    Viewport {
        offset: egui::Pos2::ZERO,
        zoom: 1.0,
        bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(400.0)),
    }
}

fn with_test_ui(mut body: impl FnMut(&egui::Ui)) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| body(ui));
    });
}

fn arm_test_move(state: &mut AppState, mode: crate::state::MoveSelectionMode) {
    crate::workbench::app::open_move_selection_dialog(state);
    state.dialogs.move_selection.mode = mode;
    state.dialogs.move_selection.arm();
    state.schematic.arm_tool(Tool::MoveSelection);
}

fn move_keyboard_input() -> egui::RawInput {
    egui::RawInput {
        events: [egui::Key::ArrowRight, egui::Key::Enter]
            .into_iter()
            .map(|key| egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            })
            .collect(),
        ..Default::default()
    }
}

fn saved_outputs(state: &AppState) -> &[SavedOutput] {
    let plan_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("default state owns a stable plan")
        .id();
    state
        .workspace
        .plan_data(plan_id)
        .map_or(&[], |payload| payload.saved_outputs.as_slice())
}

#[test]
fn empty_canvas_double_click_ascends_only_from_a_descended_context() {
    let mut state = AppState::default();
    assert_eq!(
        select_double_click_action(&state, None, true),
        SelectDoubleClickAction::None
    );

    state.workspace.descend_into(
        "X1".to_owned(),
        crate::state::CellViewRef::new("work", "child", "schematic"),
        ViewType::Schematic,
    );
    assert_eq!(
        select_double_click_action(&state, None, true),
        SelectDoubleClickAction::Ascend
    );
    assert_eq!(
        select_double_click_action(&state, None, false),
        SelectDoubleClickAction::None,
        "a filtered or otherwise non-empty hit must not masquerade as empty canvas"
    );
}

#[test]
fn schematic_and_exact_veriloga_instance_double_click_destinations_are_distinct() {
    let mut state = AppState::default();
    let schematic_reference = crate::state::CellViewRef::new("double_click", "child", "schematic");
    let veriloga_reference = crate::state::CellViewRef::new("double_click", "behavior", "veriloga");
    let mut library = crate::state::Library::new("double_click");
    let mut schematic_cell = crate::state::Cell::new("child");
    schematic_cell.add_view(crate::state::View::new(
        "schematic",
        crate::state::ViewType::Schematic,
    ));
    library.add_cell(schematic_cell);
    let mut veriloga_cell = crate::state::Cell::new("behavior");
    let mut veriloga_view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
    veriloga_view
        .metadata
        .insert("veriloga.module".to_owned(), "behavior".to_owned());
    veriloga_cell.add_view(veriloga_view);
    library.add_cell(veriloga_cell);
    state.library_manager.add_library(library);

    state.schematic.components.push(
        Component::new(41, ComponentType::CellInstance, Point::new(20, 20)).with_library_cell(
            crate::state::LibraryCellInstance::new(
                &schematic_reference.library,
                &schematic_reference.cell,
                &schematic_reference.view,
            ),
        ),
    );
    state.schematic.components.push(
        Component::new(42, ComponentType::CellInstance, Point::new(40, 20)).with_library_cell(
            crate::state::LibraryCellInstance::new(
                &veriloga_reference.library,
                &veriloga_reference.cell,
                &veriloga_reference.view,
            ),
        ),
    );

    assert_eq!(
        select_double_click_action(&state, Some(PointerTarget::Component(41)), false),
        SelectDoubleClickAction::Descend(41)
    );
    assert_eq!(
        select_double_click_action(&state, Some(PointerTarget::Component(42)), false),
        SelectDoubleClickAction::OpenProperties,
        "a Verilog-A-looking view without its exact source owner must fail closed"
    );

    state
        .workspace
        .project_sources
        .insert_bundle(
            crate::state::ProjectSourceBundle::try_new(
                crate::state::ProjectSourceOwner::cell_view(veriloga_reference.clone()),
                crate::state::ProjectSourceLanguage::VerilogA,
                "behavior.va",
                "module behavior(p, n); inout p, n; electrical p, n; endmodule",
                Vec::<crate::state::ProjectSourceFile>::new(),
                Vec::<crate::state::ProjectSourceDependency>::new(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        select_double_click_action(&state, Some(PointerTarget::Component(42)), false),
        SelectDoubleClickAction::OpenVerilogA(42)
    );
    assert!(state.open_veriloga_source_for_component(42));
    assert_eq!(state.workspace.active_view, veriloga_reference);
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Netlist
    );
    assert_eq!(
        state.ui.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA
    );
}

#[test]
fn materialized_probe_toggles_immediately_and_preserves_future_save_intent() {
    let mut state = AppState::default();
    state.simulation.waveforms.push(WaveformData::new(
        "V(OUT)",
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        "#ffffff",
    ));

    assert_eq!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::WaveformHidden
    );
    assert!(!state.simulation.waveforms[0].visible);
    assert_eq!(saved_outputs(&state).len(), 1);

    assert_eq!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::WaveformShown
    );
    assert!(state.simulation.waveforms[0].visible);
    assert_eq!(saved_outputs(&state).len(), 1);
}

#[test]
fn ensure_visible_probe_action_never_hides_an_existing_trace() {
    let mut state = AppState::default();
    state.simulation.waveforms.push(WaveformData::new(
        "V(OUT)",
        vec![0.0, 1.0],
        vec![0.0, 1.0],
        "#ffffff",
    ));

    assert_eq!(
        request_probe_signal_visible(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::WaveformAlreadyVisible
    );
    assert!(state.simulation.waveforms[0].visible);
    assert_eq!(saved_outputs(&state).len(), 1);

    state.simulation.waveforms[0].visible = false;
    assert_eq!(
        request_probe_signal_visible(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::WaveformShown
    );
    assert!(state.simulation.waveforms[0].visible);
    assert_eq!(saved_outputs(&state).len(), 1);
}

#[test]
fn wire_probe_resolves_from_live_connectivity_without_retained_run_data() {
    let mut state = AppState::default();
    state
        .schematic
        .wires
        .push(Wire::new(91, vec![Point::new(0, 20), Point::new(80, 20)]));
    state
        .schematic
        .net_labels
        .push(NetLabel::new(92, Point::new(40, 20), "OUT"));

    assert!(
        state
            .simulation
            .cross_probe
            .net_at_in(
                &state.workspace.active_view,
                state.schematic.topology_version(),
                Point::new(40, 20),
            )
            .is_none(),
        "the fixture must not depend on retained simulation cross-probe data"
    );
    assert_eq!(live_wire_probe_net_name(&state, 91).as_deref(), Some("OUT"));
}

#[test]
fn component_probe_never_fabricates_a_voltage_node_from_instance_identity() {
    let mut state = AppState::default();
    state.schematic.components.push(
        Component::new(17, ComponentType::CellInstance, Point::origin())
            .with_name_value("XAMP", ""),
    );
    let symbols = SchematicSymbolContext::from_state(&state);

    assert_eq!(
        component_probe_expression(&state, 17, Point::origin(), &symbols),
        None,
        "an unresolved terminal must fail closed instead of inventing V(XAMP)"
    );
}

#[test]
fn voltage_source_component_probe_preserves_device_current_semantics() {
    let mut state = AppState::default();
    state.schematic.components.push(
        Component::new(23, ComponentType::VoltageSource, Point::origin())
            .with_name_value("VBIAS", "1.8"),
    );
    let symbols = SchematicSymbolContext::from_state(&state);

    assert_eq!(
        component_probe_expression(&state, 23, Point::origin(), &symbols),
        Some(('I', "VBIAS".to_owned()))
    );
}

#[test]
fn ordinary_component_body_probe_requests_device_current_not_nearest_pin_voltage() {
    let mut state = AppState::default();
    state.schematic.components.push(
        Component::new(24, ComponentType::Resistor, Point::origin()).with_name_value("RLOAD", "1k"),
    );
    let symbols = SchematicSymbolContext::from_state(&state);

    assert_eq!(
        component_probe_expression(&state, 24, Point::origin(), &symbols),
        Some(('I', "RLOAD".to_owned()))
    );
}

#[test]
fn exact_component_terminal_probe_requests_its_node_voltage() {
    let mut state = AppState::default();
    state.schematic.components.push(
        Component::new(25, ComponentType::Resistor, Point::origin()).with_name_value("RLOAD", "1k"),
    );
    let symbols = SchematicSymbolContext::from_state(&state);

    assert_eq!(
        component_probe_expression(&state, 25, Point::new(-20, 0), &symbols),
        Some(('V', "net1".to_owned()))
    );
}

#[test]
fn synthesized_and_multi_port_component_bodies_fail_closed() {
    for (index, kind) in [
        ComponentType::Ground,
        ComponentType::Port,
        ComponentType::Transformer,
        ComponentType::CoupledInductor,
        ComponentType::XspiceGain,
    ]
    .into_iter()
    .enumerate()
    {
        let mut state = AppState::default();
        let id = 100 + index as u64;
        state
            .schematic
            .components
            .push(Component::new(id, kind, Point::origin()));
        let symbols = SchematicSymbolContext::from_state(&state);

        assert_eq!(
            component_probe_expression(&state, id, Point::origin(), &symbols),
            None,
            "{kind:?} must not fabricate a single body-current observable"
        );
    }
}

#[test]
fn unmaterialized_probe_creates_one_plan_owned_output_idempotently() {
    let mut state = AppState::default();
    let before_revision = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();

    assert!(matches!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));
    let after_first_revision = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();
    assert!(after_first_revision > before_revision);
    let outputs = saved_outputs(&state);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].kind, SavedOutputKind::RawVoltageOrCurrent);
    assert_eq!(outputs[0].name, "V(OUT)");
    assert_eq!(outputs[0].source_expression, "V(OUT)");
    assert_eq!(
        outputs[0].compatible_analyses,
        SavedOutputCompatibility::AllCompatibleAnalyses
    );
    assert_eq!(
        outputs[0].save_policy,
        SavedOutputPolicy::SelectedAndFinalPoints
    );
    assert_eq!(
        outputs[0].stored_precision,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision
    );
    assert_eq!(
        outputs[0].streaming,
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation
    );

    assert!(matches!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("v(out)")),
        ProbeSignalOutcome::SavedOutputAlreadyPresent { .. }
    ));
    assert_eq!(saved_outputs(&state).len(), 1);
    assert_eq!(
        state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision(),
        after_first_revision,
        "an idempotent probe must not create a second configuration revision"
    );
}

#[test]
fn probe_without_stable_plan_fails_closed() {
    let mut state = AppState::default();
    let payloads_before = state.workspace.simulation_plan_payloads.clone();
    state.sim_setup.analysis_plan = None;

    let outcome = request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)"));

    assert!(matches!(outcome, ProbeSignalOutcome::Rejected { .. }));
    assert_eq!(state.workspace.simulation_plan_payloads, payloads_before);
    assert!(state.sim_setup.analysis_plan.is_none());
}

#[test]
fn ground_probe_is_reference_only_and_never_creates_output() {
    let mut state = AppState::default();
    let before_revision = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();

    assert_eq!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim(" v ( 0 ) ")),
        ProbeSignalOutcome::GroundReference
    );
    assert!(saved_outputs(&state).is_empty());
    assert_eq!(
        state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .revision(),
        before_revision
    );
}

#[test]
fn empty_space_probe_retains_one_unbound_marker_and_undo_removes_it() {
    let mut state = AppState::default();
    let position = Point::new(30, 40);

    let id =
        retain_probe_flag(&mut state, position, None, None).expect("editable active schematic");
    assert_eq!(state.schematic.probes.len(), 1);
    assert_eq!(state.schematic.probes[0].id, id);
    assert_eq!(state.schematic.probes[0].position, position);
    assert_eq!(state.schematic.probes[0].reference, format!("P{id}"));
    assert!(state.schematic.probes[0].source_expression.is_none());
    assert_eq!(
        state.schematic.undo_description(),
        Some("place schematic probe")
    );

    assert!(state.schematic.undo());
    assert!(state.schematic.probes.is_empty());
}

#[test]
fn probe_marker_rejects_read_only_and_replaced_view_identity_without_mutation() {
    let mut read_only = AppState::default();
    read_only.schematic.read_only = true;
    assert!(retain_probe_flag(&mut read_only, Point::origin(), None, None).is_err());
    assert!(read_only.schematic.probes.is_empty());
    assert!(!read_only.schematic.can_undo());

    let mut read_only_reference = AppState::default();
    read_only_reference
        .workspace
        .set_active_read_only_reference(true);
    assert!(retain_probe_flag(&mut read_only_reference, Point::origin(), None, None).is_err());
    assert!(read_only_reference.schematic.probes.is_empty());
    assert!(!read_only_reference.schematic.can_undo());

    let mut replaced = AppState::default();
    replaced.workspace.active_view.view = "symbol".to_owned();
    assert!(retain_probe_flag(&mut replaced, Point::origin(), None, None).is_err());
    assert!(replaced.schematic.probes.is_empty());
    assert!(!replaced.schematic.can_undo());
}

#[test]
fn bound_probe_marker_retains_the_exact_source_expression() {
    let mut state = AppState::default();
    retain_probe_flag(
        &mut state,
        Point::new(10, 20),
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        None,
    )
    .expect("bound marker");

    assert_eq!(state.schematic.probes[0].reference, "V(OUT)");
    assert_eq!(
        state.schematic.probes[0].source_expression.as_deref(),
        Some("V(OUT)")
    );
}

#[test]
fn probe_placed_while_descended_names_the_occurrence() {
    let mut state = AppState::default();
    state.workspace.descend_into(
        "X1".to_owned(),
        crate::state::CellViewRef::new("user", "amp", "schematic"),
        ViewType::Schematic,
    );

    let spelling = probe_spelling_for(&state, "n1", "V(n1)").expect("an ASCII occurrence spells");

    assert_eq!(spelling.display(), "V(/X1/n1)");
    assert_eq!(spelling.engine(), "V(x1.n1)");
    assert_eq!(
        probe_spelling_for(&state, "V(x1.n1)", "V(x1.n1)")
            .expect("an exact expression is not re-scoped")
            .display(),
        "V(x1.n1)",
        "an expression that already names a node must be used as written"
    );
}

#[test]
fn an_occurrence_probe_saves_the_display_name_and_requests_the_engine_node() {
    let mut state = AppState::default();
    let spelling = OccurrenceProbeSpelling::for_leaf(
        &crate::state::InstancePath::parse("/X1").expect("one descent"),
        'V',
        "n1",
    )
    .expect("an ASCII occurrence spells");

    assert!(matches!(
        request_probe_signal(&mut state, &spelling),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));

    let outputs = saved_outputs(&state);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].name, "V(/X1/n1)");
    assert_eq!(
        outputs[0].source_expression, "V(x1.n1)",
        "the plan must request the node the engine solved, not the one drawn"
    );

    retain_probe_flag(&mut state, Point::new(10, 20), Some(&spelling), None)
        .expect("marker retains");
    assert_eq!(state.schematic.probes[0].reference, "V(/X1/n1)");
    assert_eq!(
        state.schematic.probes[0].source_expression.as_deref(),
        Some("V(x1.n1)")
    );
}

#[test]
fn bound_probe_marker_retains_stable_plan_output_identity() {
    let mut state = AppState::default();
    assert!(matches!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));
    let binding = current_probe_output_binding(&state, "V(OUT)").expect("output binding");

    retain_probe_flag(
        &mut state,
        Point::new(10, 20),
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        Some(binding),
    )
    .expect("bound marker");

    assert_eq!(state.schematic.probes[0].plan_id, Some(binding.0));
    assert_eq!(state.schematic.probes[0].saved_output_id, Some(binding.1));
    assert!(state.schematic.probes[0].enabled);
    assert!(state.schematic.probes[0].plot_on_materialization);
}

#[test]
fn equivalent_probe_marker_placement_reuses_identity_without_an_undo_step() {
    let mut state = AppState::default();
    let position = Point::new(10, 20);
    let id = retain_probe_flag(
        &mut state,
        position,
        Some(&OccurrenceProbeSpelling::verbatim("V(OUT)")),
        None,
    )
    .expect("first marker");
    state.schematic.clear_undo_history();

    let repeated = retain_probe_flag(
        &mut state,
        position,
        Some(&OccurrenceProbeSpelling::verbatim(" v ( out ) ")),
        None,
    )
    .expect("existing marker");

    assert_eq!(repeated, id);
    assert_eq!(state.schematic.probes.len(), 1);
    assert_eq!(state.schematic.selection.single_probe(), Some(id));
    assert!(!state.schematic.can_undo());
}

#[test]
fn late_safe_mode_activation_rejects_probe_marker_without_mutation() {
    let mut state = AppState::default();
    state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::workbench::state::LocalSafeModeOptions::default()
        },
        String::new(),
    );

    assert!(retain_probe_flag(&mut state, Point::new(10, 20), None, None).is_err());
    assert!(state.schematic.probes.is_empty());
    assert!(!state.schematic.is_dirty);
    assert!(!state.schematic.can_undo());
}

#[test]
fn route_finish_helper_commits_wire_and_bus_without_secondary_click() {
    let mut state = AppState::default();
    state.schematic.start_wire(Point::origin());
    state.schematic.extend_wire(Point::new(20, 0));
    with_test_ui(|ui| assert!(finish_active_route(ui, &mut state)));
    assert!(!state.schematic.wire_drawing.active);
    assert_eq!(state.schematic.wires.len(), 1);

    state
        .schematic
        .start_bus(
            Point::new(0, 20),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
    state.schematic.extend_bus(Point::new(20, 20));
    with_test_ui(|ui| assert!(finish_active_route(ui, &mut state)));
    assert!(!state.schematic.bus_drawing.active);
    assert_eq!(state.schematic.buses.len(), 1);
}

#[test]
fn preexisting_equivalent_output_prevents_duplicate_probe_output() {
    let mut state = AppState::default();
    let plan_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    state
        .workspace
        .add_saved_output(
            plan_id,
            SavedOutput::new(
                SavedOutputKind::RawVoltageOrCurrent,
                "Output voltage",
                "V(out)",
                SavedOutputCompatibility::OpTranAc,
                SavedOutputPolicy::EveryAcceptedPoint,
                SavedOutputPrecision::FullSourcePrecision,
                SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
            )
            .expect("valid fixture output"),
        )
        .expect("fixture output commits");

    assert!(matches!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::SavedOutputAlreadyPresent { .. }
    ));
    assert_eq!(saved_outputs(&state).len(), 1);
    assert_eq!(saved_outputs(&state)[0].name, "Output voltage");
}

#[test]
fn unrelated_output_name_collision_gets_a_deterministic_probe_name() {
    let mut state = AppState::default();
    let plan_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .id();
    state
        .workspace
        .add_saved_output(
            plan_id,
            SavedOutput::new(
                SavedOutputKind::RawVoltageOrCurrent,
                "V(OUT)",
                "V(IN)",
                SavedOutputCompatibility::OpTranAc,
                SavedOutputPolicy::EveryAcceptedPoint,
                SavedOutputPrecision::FullSourcePrecision,
                SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
            )
            .expect("valid fixture output"),
        )
        .expect("fixture output commits");

    assert!(matches!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));
    assert_eq!(saved_outputs(&state).len(), 2);
    assert_eq!(saved_outputs(&state)[1].name, "Schematic probe 1");
    assert_eq!(saved_outputs(&state)[1].source_expression, "V(OUT)");
}

#[test]
fn armed_move_keyboard_leaves_keys_unconsumed_without_canvas_focus() {
    let ctx = egui::Context::default();
    let mut intent = None;
    let mut keys_remain = None;

    let _ = ctx.run_ui(move_keyboard_input(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // accessibility-pointer-shim: test-only canvas focus harness.
            let response = ui.interact(
                ui.max_rect(),
                egui::Id::new("unfocused-move-canvas"),
                egui::Sense::click_and_drag(),
            );
            assert!(!response.has_focus());
            intent = Some(consume_armed_move_keyboard(ui, response.has_focus(), 10));
            keys_remain = Some(ui.input(|input| {
                (
                    input.key_pressed(egui::Key::ArrowRight),
                    input.key_pressed(egui::Key::Enter),
                )
            }));
        });
    });

    assert_eq!(intent, Some((Point::origin(), false)));
    assert_eq!(keys_remain, Some((true, true)));
}

#[test]
fn armed_move_keyboard_consumes_keys_when_canvas_has_focus() {
    let ctx = egui::Context::default();
    let mut intent = None;
    let mut keys_remain = None;

    let _ = ctx.run_ui(move_keyboard_input(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // accessibility-pointer-shim: test-only canvas focus harness.
            let response = ui.interact(
                ui.max_rect(),
                egui::Id::new("focused-move-canvas"),
                egui::Sense::click_and_drag(),
            );
            response.request_focus();
            assert!(response.has_focus());
            intent = Some(consume_armed_move_keyboard(ui, response.has_focus(), 10));
            keys_remain = Some(ui.input(|input| {
                (
                    input.key_pressed(egui::Key::ArrowRight),
                    input.key_pressed(egui::Key::Enter),
                )
            }));
        });
    });

    assert_eq!(intent, Some((Point::new(10, 0), true)));
    assert_eq!(keys_remain, Some((false, false)));
}

#[test]
fn armed_move_exclusively_owns_selection_drag_routing() {
    assert!(select_drag_is_authorized(Tool::Select, false));
    assert!(!select_drag_is_authorized(Tool::Select, true));
    assert!(!select_drag_is_authorized(Tool::MoveSelection, true));
    assert!(!select_drag_is_authorized(Tool::MoveSelection, false));
}

#[test]
fn armed_move_commits_once_syncs_workspace_and_retains_selection() {
    let mut state = AppState::default();
    state
        .schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    let terminal = state.schematic.components[0].terminal_positions()[0].1;
    state
        .schematic
        .wires
        .push(Wire::segment(2, terminal, Point::new(20, 0)));
    state.schematic.selection.select_only_component(1);
    state.schematic.init_undo_history();
    arm_test_move(&mut state, crate::state::MoveSelectionMode::Connected);
    state.dialogs.move_selection.preview_delta = Point::new(0, 10);
    let symbols = SchematicSymbolContext::from_state(&state);

    commit_armed_move_selection(&mut state, &symbols);

    assert_eq!(state.schematic.components[0].pos, Point::new(0, 10));
    assert_eq!(
        state.schematic.wires[0].points[0],
        Point::new(terminal.x, terminal.y + 10)
    );
    assert_eq!(state.schematic.undo_description(), Some("move selection"));
    assert!(state.schematic.selection.has_component(1));
    assert_eq!(state.schematic.tool, Tool::Select);
    assert!(!state.dialogs.move_selection.armed);
    assert_eq!(
        state
            .workspace
            .active_schematic()
            .expect("active workspace buffer")
            .components[0]
            .pos,
        Point::new(0, 10)
    );
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.components[0].pos, Point::origin());
    assert!(
        !state.schematic.can_undo(),
        "the gesture owns one undo record"
    );
}

#[test]
fn cancelling_armed_move_preserves_geometry_selection_and_history() {
    let mut state = AppState::default();
    state
        .schematic
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    state.schematic.selection.select_only_component(1);
    state.schematic.init_undo_history();
    arm_test_move(&mut state, crate::state::MoveSelectionMode::Shove);
    state.dialogs.move_selection.preview_delta = Point::new(40, 10);

    crate::workbench::app::cancel_armed_move_selection(&mut state);

    assert_eq!(state.schematic.components[0].pos, Point::origin());
    assert!(state.schematic.selection.has_component(1));
    assert_eq!(state.schematic.tool, Tool::Select);
    assert!(!state.schematic.can_undo());
}

#[test]
fn validated_port_contract_places_once_and_undo_redo_is_exact() {
    let mut state = AppState::default();
    let pending = PendingPortPlacement::new(
        "BIAS_EN",
        PortDirectionType::InputLogic,
        PortDiscipline::Logic,
        state.schematic.topology_version(),
        state.schematic.next_interface_order(),
    )
    .with_document_authority(
        state.design_execution_epoch,
        state.active_schematic_epoch,
        state.workspace.active_view.display_path(),
    );
    state.schematic.pending_port = Some(pending);
    state.schematic.tool = Tool::Place(ComponentType::Port);

    place_component(&mut state, ComponentType::Port, Point::new(20, 30));

    assert_eq!(state.schematic.components.len(), 1);
    let placed = state.schematic.components[0].clone();
    assert_eq!(placed.pos, Point::new(20, 30));
    assert_eq!(placed.value, "BIAS_EN");
    let contract = placed.port_contract().expect("typed interface contract");
    assert_eq!(contract.direction, PortDirection::In);
    assert_eq!(contract.signal_type, PortSignalType::Logic);
    assert_eq!(contract.discipline, PortDiscipline::Logic);
    assert!(!contract.documentation.is_empty());
    assert_eq!(state.schematic.tool, Tool::Select);
    assert!(state.schematic.pending_port.is_none());
    assert_eq!(
        state.schematic.undo_description(),
        Some("place interface port")
    );

    assert!(state.schematic.undo());
    assert!(state.schematic.components.is_empty());
    assert!(state.schematic.redo());
    assert_eq!(state.schematic.components, [placed]);
}

#[test]
fn validated_design_note_contract_places_once_without_changing_topology() {
    let mut state = AppState::default();
    let pending = PendingDesignNotePlacement::new(
        DesignNoteKind::PlainText,
        "Bias network",
        state.schematic.topology_version(),
        &state.schematic.design_notes,
    )
    .unwrap()
    .with_document_authority(
        state.design_execution_epoch,
        state.active_schematic_epoch,
        state.workspace.active_view.display_path(),
    );
    let topology = state.schematic.topology_version();
    state.schematic.pending_design_note = Some(pending);
    state.schematic.tool = Tool::DesignNote;

    place_pending_design_note(&mut state, Point::new(20, 30));

    assert_eq!(state.schematic.design_notes.len(), 1);
    assert_eq!(state.schematic.design_notes[0].pos, Point::new(20, 30));
    assert_eq!(state.schematic.topology_version(), topology);
    assert!(state.schematic.pending_design_note.is_none());
    assert_eq!(state.schematic.tool, Tool::Select);
    assert!(state.schematic.undo());
    assert!(state.schematic.design_notes.is_empty());
}

#[test]
fn every_documentation_shape_gesture_commits_once_and_remains_non_electrical() {
    let cases = [
        (
            DocumentationShapeKind::Rectangle,
            vec![Point::new(0, 0), Point::new(20, 10)],
            false,
        ),
        (
            DocumentationShapeKind::Line,
            vec![Point::new(0, 0), Point::new(20, 10)],
            false,
        ),
        (
            DocumentationShapeKind::Polygon,
            vec![Point::new(0, 0), Point::new(20, 0), Point::new(10, 10)],
            true,
        ),
        (
            DocumentationShapeKind::Arc,
            vec![Point::new(0, 10), Point::new(10, 0), Point::new(20, 10)],
            false,
        ),
        (
            DocumentationShapeKind::Callout,
            vec![Point::new(0, 0), Point::new(10, 10), Point::new(30, 20)],
            false,
        ),
    ];

    for (kind, points, finish_on_last_click) in cases {
        let mut state = AppState::default();
        let topology = state.schematic.topology_version();
        state.schematic.pending_documentation_shape = Some(
            PendingDocumentationShapePlacement::new(
                kind,
                topology,
                &state.schematic.documentation_shapes,
            )
            .with_document_authority(
                state.design_execution_epoch,
                state.active_schematic_epoch,
                state.workspace.active_view.display_path(),
            ),
        );
        state.schematic.tool = Tool::DocumentationShape;

        for (index, point) in points.iter().copied().enumerate() {
            let finish = finish_on_last_click && index + 1 == points.len();
            with_test_ui(|ui| handle_documentation_shape_click(ui, &mut state, point, finish));
        }

        assert_eq!(state.schematic.documentation_shapes.len(), 1, "{kind:?}");
        assert_eq!(state.schematic.documentation_shapes[0].kind(), kind);
        assert_eq!(state.schematic.topology_version(), topology);
        assert!(state.schematic.components.is_empty());
        assert!(state.schematic.wires.is_empty());
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(state.schematic.pending_documentation_shape.is_none());
        assert!(
            state
                .schematic
                .documentation_shape_drawing
                .points
                .is_empty()
        );
        assert_eq!(
            state.schematic.undo_description(),
            Some("draw documentation shape")
        );
        assert!(state.schematic.undo());
        assert!(state.schematic.documentation_shapes.is_empty());
        assert!(
            !state.schematic.can_undo(),
            "{kind:?} must create one undo step"
        );
    }
}

#[test]
fn stale_documentation_shape_authority_is_consumed_without_document_mutation() {
    let mut state = AppState::default();
    state.schematic.pending_documentation_shape = Some(
        PendingDocumentationShapePlacement::new(
            DocumentationShapeKind::Line,
            state.schematic.topology_version(),
            &state.schematic.documentation_shapes,
        )
        .with_document_authority(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.workspace.active_view.display_path(),
        ),
    );
    state.schematic.tool = Tool::DocumentationShape;
    state.active_schematic_epoch = state.active_schematic_epoch.wrapping_add(1);

    with_test_ui(|ui| handle_documentation_shape_click(ui, &mut state, Point::new(0, 0), false));

    assert!(state.schematic.documentation_shapes.is_empty());
    assert!(state.schematic.pending_documentation_shape.is_none());
    assert!(
        state
            .schematic
            .documentation_shape_drawing
            .points
            .is_empty()
    );
    assert_eq!(state.schematic.tool, Tool::Select);
    assert!(!state.schematic.can_undo());
}

#[test]
fn focused_keyboard_cursor_places_exact_grid_resolved_shape_points() {
    let mut state = AppState::default();
    let grid = state.schematic.grid_size;
    state.schematic.pending_documentation_shape = Some(
        PendingDocumentationShapePlacement::new(
            DocumentationShapeKind::Line,
            state.schematic.topology_version(),
            &state.schematic.documentation_shapes,
        )
        .with_document_authority(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.workspace.active_view.display_path(),
        ),
    );
    state.schematic.tool = Tool::DocumentationShape;
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let keyboard_frame = |keys: &[egui::Key], state: &mut AppState| {
        let input = egui::RawInput {
            events: keys
                .iter()
                .copied()
                .map(|key| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                })
                .collect(),
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // accessibility-pointer-shim: test-only canvas event harness.
                let response = ui.interact(
                    ui.max_rect(),
                    egui::Id::new("documentation-shape-keyboard-test"),
                    egui::Sense::click_and_drag(),
                );
                let viewport = pointer_viewport();
                handle_documentation_shape_keyboard(ui, &response, state, &viewport, grid);
            });
        });
    };

    keyboard_frame(&[egui::Key::ArrowRight, egui::Key::Space], &mut state);
    assert_eq!(
        state.schematic.documentation_shape_drawing.points,
        vec![Point::new(grid, 0)]
    );
    keyboard_frame(&[egui::Key::ArrowDown, egui::Key::Enter], &mut state);

    assert_eq!(state.schematic.documentation_shapes.len(), 1);
    assert_eq!(
        state.schematic.documentation_shapes[0].geometry,
        crate::state::DocumentationShapeGeometry::Line {
            start: Point::new(grid, 0),
            end: Point::new(grid, grid),
        }
    );
    assert_eq!(state.schematic.tool, Tool::Select);
    assert!(
        state
            .schematic
            .documentation_shape_drawing
            .points
            .is_empty()
    );
    assert!(
        state
            .schematic
            .documentation_shape_drawing
            .keyboard_cursor
            .is_none()
    );
}

#[test]
fn stale_design_note_authority_is_consumed_without_document_mutation() {
    let mut state = AppState::default();
    let pending = PendingDesignNotePlacement::new(
        DesignNoteKind::ReviewNote,
        "Review bias path",
        state.schematic.topology_version(),
        &state.schematic.design_notes,
    )
    .unwrap()
    .with_document_authority(
        state.design_execution_epoch,
        state.active_schematic_epoch,
        state.workspace.active_view.display_path(),
    );
    state.schematic.pending_design_note = Some(pending);
    state.schematic.tool = Tool::DesignNote;
    state.active_schematic_epoch = state.active_schematic_epoch.wrapping_add(1);

    place_pending_design_note(&mut state, Point::new(20, 30));

    assert!(state.schematic.design_notes.is_empty());
    assert!(state.schematic.pending_design_note.is_none());
    assert_eq!(state.schematic.tool, Tool::Select);
    assert!(!state.schematic.can_undo());
}

#[test]
fn port_placement_without_a_current_validated_contract_fails_closed() {
    let mut state = AppState::default();
    state.schematic.tool = Tool::Place(ComponentType::Port);

    place_component(&mut state, ComponentType::Port, Point::new(20, 30));

    assert!(state.schematic.components.is_empty());
    assert!(!state.schematic.can_undo());
    assert_eq!(state.schematic.tool, Tool::Select);
    assert!(state.schematic.pending_port.is_none());
}

#[test]
fn topology_change_rejects_frozen_port_without_partial_mutation() {
    let mut state = AppState::default();
    state.schematic.pending_port = Some(
        PendingPortPlacement::new(
            "OUT",
            PortDirectionType::OutputAnalog,
            PortDiscipline::Electrical,
            state.schematic.topology_version(),
            state.schematic.next_interface_order(),
        )
        .with_document_authority(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.workspace.active_view.display_path(),
        ),
    );
    state.schematic.tool = Tool::Place(ComponentType::Port);
    state.schematic.bump_topology_version();

    place_component(&mut state, ComponentType::Port, Point::new(40, 10));

    assert!(state.schematic.components.is_empty());
    assert!(!state.schematic.can_undo());
    assert_eq!(state.schematic.tool, Tool::Select);
}

#[test]
fn armed_port_rejects_a_replaced_active_document_even_when_topology_matches() {
    let mut state = AppState::default();
    state.schematic.pending_port = Some(
        PendingPortPlacement::new(
            "OUT",
            PortDirectionType::OutputAnalog,
            PortDiscipline::Electrical,
            state.schematic.topology_version(),
            state.schematic.next_interface_order(),
        )
        .with_document_authority(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.workspace.active_view.display_path(),
        ),
    );
    state.schematic.tool = Tool::Place(ComponentType::Port);
    state.active_schematic_epoch = state.active_schematic_epoch.wrapping_add(1);

    place_component(&mut state, ComponentType::Port, Point::new(40, 10));

    assert!(state.schematic.components.is_empty());
    assert!(!state.schematic.can_undo());
    assert_eq!(state.schematic.tool, Tool::Select);
}

#[test]
fn primary_drag_is_reserved_only_when_a_pan_modifier_owns_it() {
    assert!(select_drag_can_start(false));
    assert!(!select_drag_can_start(true));
}

#[test]
fn click_and_drag_share_one_overlapping_object_priority() {
    let point = Point::new(10, 0);
    let bus = Bus::segment(
        20,
        Point::new(0, 0),
        Point::new(20, 0),
        Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
    )
    .unwrap();
    let tap = BusTap::new(
        21,
        &bus,
        point,
        Point::new(10, 10),
        BusSlice::parse("DATA[3]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    let mut state = AppState::default();
    state
        .schematic
        .components
        .push(Component::new(10, ComponentType::Resistor, point));
    state
        .schematic
        .wires
        .push(Wire::segment(11, Point::new(0, 0), Point::new(20, 0)));
    state.schematic.junctions.push(Junction::new(12, point));
    state.schematic.buses.push(bus);
    state.schematic.bus_taps.push(tap);
    let context = SchematicSymbolContext::default();
    let ctx = egui::Context::default();
    let viewport = pointer_viewport();
    let screen_point = egui::pos2(point.x as f32, point.y as f32);

    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(point, point),
            1,
            &context,
            &ctx,
            &viewport,
            screen_point,
        ),
        Some(PointerTarget::Component(10))
    );
    state.ui.schematic_selection_filter.instances = false;
    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(point, point),
            1,
            &context,
            &ctx,
            &viewport,
            screen_point,
        ),
        Some(PointerTarget::BusTap(21)),
        "disabled instance hit-testing must fall through to enabled conductors"
    );
    state.ui.schematic_selection_filter.instances = true;
    state.schematic.components.clear();
    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(point, point),
            1,
            &context,
            &ctx,
            &viewport,
            screen_point,
        ),
        Some(PointerTarget::BusTap(21))
    );
    state.schematic.bus_taps.clear();
    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(point, point),
            1,
            &context,
            &ctx,
            &viewport,
            screen_point,
        ),
        Some(PointerTarget::Junction(point))
    );
    state.schematic.junctions.clear();
    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(point, point),
            1,
            &context,
            &ctx,
            &viewport,
            screen_point,
        ),
        Some(PointerTarget::Bus(20))
    );
    state.schematic.buses.clear();
    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(point, point),
            1,
            &context,
            &ctx,
            &viewport,
            screen_point,
        ),
        Some(PointerTarget::Wire(11))
    );
}

#[test]
fn hidden_overlapping_component_cannot_block_active_component_hit() {
    let point = Point::new(10, 10);
    let mut state = AppState::default();
    state.schematic.components = vec![
        Component::new(20, ComponentType::Capacitor, point),
        Component::new(10, ComponentType::Resistor, point),
    ];
    let key = state.workspace.active_schematic_reference().key();
    let first = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10, 20])
        .unwrap();
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
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
    catalog
        .assign_objects(catalog.revision(), second, [20])
        .unwrap();
    catalog.set_active(first).unwrap();
    let context = SchematicSymbolContext::default();
    let ctx = egui::Context::default();
    let viewport = pointer_viewport();

    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(point, point),
            1,
            &context,
            &ctx,
            &viewport,
            egui::pos2(point.x as f32, point.y as f32),
        ),
        Some(PointerTarget::Component(10))
    );
}

#[test]
fn inactive_sheet_probe_cannot_block_active_probe_hit() {
    let point = Point::new(10, 10);
    let mut state = AppState::default();
    state.schematic.probes = vec![
        SchematicProbe::new(30, point, "V(active)", Some("V(active)".to_owned())).unwrap(),
        SchematicProbe::new(31, point, "V(hidden)", Some("V(hidden)".to_owned())).unwrap(),
    ];
    let key = state.workspace.active_schematic_reference().key();
    let first = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [30, 31])
        .unwrap();
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
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
    catalog
        .assign_objects(catalog.revision(), second, [31])
        .unwrap();
    catalog.set_active(first).unwrap();
    let context = SchematicSymbolContext::default();
    let ctx = egui::Context::default();
    let viewport = pointer_viewport();

    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(point, point),
            1,
            &context,
            &ctx,
            &viewport,
            viewport.schematic_to_screen(point),
        ),
        Some(PointerTarget::Probe(30))
    );
}

#[test]
fn double_click_property_dispatch_selects_taps_before_their_source_bus() {
    let mut state = AppState::default();
    let bus = Bus::segment(
        20,
        Point::new(0, 0),
        Point::new(20, 0),
        Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
    )
    .unwrap();
    let tap = BusTap::new(
        21,
        &bus,
        Point::new(10, 0),
        Point::new(10, 10),
        BusSlice::parse("DATA[3]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    state.schematic.buses.push(bus);
    state.schematic.bus_taps.push(tap);
    let symbol_context = SchematicSymbolContext::from_state(&state);
    let ctx = egui::Context::default();
    let viewport = pointer_viewport();
    let screen_point = egui::pos2(10.0, 0.0);

    open_object_properties(
        &mut state,
        PointerHit::new(Point::new(10, 0), Point::new(10, 0)),
        1,
        &symbol_context,
        &ctx,
        &viewport,
        screen_point,
    );

    assert_eq!(state.schematic.selection.single_bus_tap(), Some(21));
    assert!(matches!(
        state.dialogs.object_properties.draft,
        Some(crate::workbench::app::ObjectPropertiesDraft::BusTap(_))
    ));
}

#[test]
fn net_label_text_bounds_are_a_first_class_pointer_target() {
    let mut state = AppState::default();
    let label = NetLabel::new(31, Point::new(40, 40), "afe_out");
    state.schematic.net_labels.push(label.clone());
    state
        .schematic
        .components
        .push(Component::new(10, ComponentType::Resistor, label.pos));
    state
        .schematic
        .wires
        .push(Wire::segment(11, Point::new(0, 40), Point::new(100, 40)));
    let symbol_context = SchematicSymbolContext::default();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
    let viewport = pointer_viewport();
    let hit = super::super::net_labels::hit_bounds(&ctx, &viewport, &label)
        .expect("visible label")
        .center();

    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(label.pos, label.pos),
            1,
            &symbol_context,
            &ctx,
            &viewport,
            hit,
        ),
        Some(PointerTarget::NetLabel(31))
    );

    open_object_properties(
        &mut state,
        PointerHit::new(label.pos, label.pos),
        1,
        &symbol_context,
        &ctx,
        &viewport,
        hit,
    );
    assert!(state.dialogs.object_properties.open);
    assert!(matches!(
        state.dialogs.object_properties.draft.as_ref(),
        Some(crate::workbench::app::ObjectPropertiesDraft::NetLabel(draft))
            if draft.original.id == label.id
    ));

    state.schematic.net_labels.clear();
    assert_eq!(
        pointer_target(
            &state,
            PointerHit::new(label.pos, label.pos),
            1,
            &symbol_context,
            &ctx,
            &viewport,
            hit,
        ),
        Some(PointerTarget::Component(10)),
        "once the visually topmost label is absent the component receives the pointer"
    );
}

#[test]
fn requirement_link_activation_uses_owned_specifications_or_safe_external_url() {
    let mut state = AppState::default();
    state.schematic.design_notes.push(
        crate::state::DesignNote::new(
            32,
            Point::new(20, 20),
            DesignNoteKind::RequirementLink,
            "REQ-19",
        )
        .unwrap(),
    );
    let ctx = egui::Context::default();
    assert!(activate_requirement_link(&mut state, 32, &ctx));
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Results
    );
    assert_eq!(
        state.ui.results.viewer,
        crate::workbench::ResultViewer::Specs
    );
    assert!(state.ui.results.spec_drafts.is_some());

    state.schematic.design_notes.push(
        crate::state::DesignNote::new(
            33,
            Point::new(30, 20),
            DesignNoteKind::RequirementLink,
            "https://tracker.example/item?id=19&from=schematic%20note",
        )
        .unwrap(),
    );
    let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
        assert!(activate_requirement_link(&mut state, 33, ctx));
    });
    assert!(
        output
            .platform_output
            .commands
            .iter()
            .any(|command| matches!(
                command,
                egui::OutputCommand::OpenUrl(open)
                    if open.url == "https://tracker.example/item?id=19&from=schematic%20note"
                        && open.new_tab
            ))
    );
}

#[test]
fn explicit_junction_placement_requires_two_wires_and_is_one_undo_step() {
    let mut state = AppState::default();
    state.schematic.wires = vec![
        Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
        Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
    ];
    state.schematic.bump_topology_version();
    state
        .schematic
        .net_highlight
        .highlight_wires([1].into_iter().collect());

    assert_eq!(
        commit_explicit_junction(&mut state, Point::new(20, 20)),
        JunctionPlacementOutcome::Placed(Point::new(20, 20))
    );
    assert!(state.schematic.has_junction(Point::new(20, 20)));
    assert!(!state.schematic.net_highlight.active);
    assert!(state.schematic.net_highlight.highlighted_wires.is_empty());
    assert!(state.schematic.can_undo());
    assert!(state.schematic.undo());
    assert!(!state.schematic.has_junction(Point::new(20, 20)));

    assert_eq!(
        commit_explicit_junction(&mut state, Point::new(100, 100)),
        JunctionPlacementOutcome::NoIntersection
    );
    assert!(!state.schematic.can_undo());
}

#[test]
fn clicking_an_existing_junction_removes_it_as_one_undo_step() {
    let mut state = AppState::default();
    state.schematic.wires = vec![
        Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
        Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
    ];
    state.schematic.add_junction(Point::new(20, 20));
    state
        .schematic
        .net_highlight
        .highlight_wires([1, 2].into_iter().collect());

    assert_eq!(
        commit_explicit_junction(&mut state, Point::new(20, 20)),
        JunctionPlacementOutcome::Removed(Point::new(20, 20))
    );
    assert!(!state.schematic.has_junction(Point::new(20, 20)));
    assert!(!state.schematic.net_highlight.active);
    assert!(state.schematic.net_highlight.highlighted_wires.is_empty());
    let disconnected = extract(&state.schematic, None);
    assert_eq!(
        disconnected.net_of_wire(1).map(|net| net.wires.clone()),
        Some(vec![1])
    );
    assert_eq!(
        disconnected.net_of_wire(2).map(|net| net.wires.clone()),
        Some(vec![2])
    );
    assert!(state.schematic.can_undo());
    assert!(state.schematic.undo());
    assert!(state.schematic.has_junction(Point::new(20, 20)));
    let connected = extract(&state.schematic, None);
    assert_eq!(
        connected.net_of_wire(1).map(|net| net.wires.clone()),
        Some(vec![1, 2])
    );
    assert!(!state.schematic.can_undo());
}

/// A plain conductor pair, two separated groups one name joins, and a typed
/// bus tap feeding a conductor — the three shapes a canvas-local connectivity
/// owner gets wrong.
fn canvas_net_corpus() -> Vec<(&'static str, AppState)> {
    let mut plain = AppState::default();
    plain.schematic.wires = vec![
        Wire::segment(1, Point::new(0, 0), Point::new(40, 0)),
        Wire::segment(2, Point::new(40, 0), Point::new(40, 40)),
    ];
    plain
        .schematic
        .net_labels
        .push(NetLabel::new(3, Point::new(20, 0), "sense"));

    let mut separated = AppState::default();
    separated.schematic.wires = vec![
        Wire::segment(11, Point::new(0, 0), Point::new(40, 0)),
        Wire::segment(12, Point::new(0, 100), Point::new(40, 100)),
    ];
    separated.schematic.net_labels = vec![
        NetLabel::new(21, Point::new(20, 0), "VDD"),
        NetLabel::new(22, Point::new(20, 100), "VDD"),
    ];

    let mut tapped = AppState::default();
    let bus = Bus::segment(
        40,
        Point::new(0, 0),
        Point::new(80, 0),
        Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
    )
    .unwrap();
    let tap = BusTap::new(
        41,
        &bus,
        Point::new(40, 0),
        Point::new(40, 20),
        BusSlice::parse("DATA[3]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    tapped.schematic.buses.push(bus);
    tapped.schematic.bus_taps.push(tap);
    tapped
        .schematic
        .wires
        .push(Wire::segment(42, Point::new(40, 20), Point::new(80, 20)));

    let mut corpus = vec![
        ("plain conductor pair", plain),
        ("separated same-name groups", separated),
        ("typed bus tap", tapped),
    ];
    for (_, state) in &mut corpus {
        state.sync_active_schematic_to_workspace();
    }
    corpus
}

/// The differential guard. What the canvas lights for a conductor is the net
/// the one extraction gives that conductor, over every fixture — a second
/// connectivity owner growing back on the canvas fails here first.
#[test]
fn the_canvas_net_partition_is_the_one_extractions_partition() {
    for (label, state) in canvas_net_corpus() {
        let connectivity = extract(&state.schematic, None);
        for wire in &state.schematic.wires {
            let lit = canvas_net_highlight(&state, |net| net.wire_ids.contains(&wire.id))
                .map(|(_, wire_ids)| wire_ids)
                .unwrap_or_default();
            let solved = connectivity
                .net_of_wire(wire.id)
                .map(|net| net.wires.iter().copied().collect())
                .unwrap_or_default();
            assert_eq!(
                lit, solved,
                "{label}: conductor {} lights a different net than the deck solves",
                wire.id
            );
        }
    }
}

/// Alt-click is a net gesture, not a geometry gesture: one name across two
/// drawn groups lights both, under the node name a probe would emit.
#[test]
fn alt_click_lights_every_group_the_deck_joins_under_one_name() {
    let (_, state) = canvas_net_corpus().swap_remove(1);
    let (name, wire_ids) = canvas_net_highlight(&state, |net| net.wire_ids.contains(&11))
        .expect("a labelled conductor resolves to a net");
    assert_eq!(name, "VDD");
    assert_eq!(
        wire_ids,
        [11, 12]
            .into_iter()
            .collect::<std::collections::HashSet<_>>(),
        "the separated group carrying the same name is on the same node"
    );
}

#[test]
fn automatic_t_marker_is_not_an_explicit_junction_toggle_target() {
    let point = Point::new(20, 20);
    let mut state = AppState::default();
    state.schematic.wires = vec![
        Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
        Wire::new(2, vec![point, Point::new(20, 40)]),
    ];
    state.schematic.add_junction(point);

    assert_eq!(
        commit_explicit_junction(&mut state, point),
        JunctionPlacementOutcome::NoIntersection
    );
    assert!(state.schematic.has_junction(point));
    assert!(!state.schematic.can_undo());
}
