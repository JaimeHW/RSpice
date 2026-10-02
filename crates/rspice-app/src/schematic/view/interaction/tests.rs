//! Tests for pointer interaction on the schematic canvas.
//!
//! The cases pin what is a legitimate pointer target and what one gesture
//! costs in undo - a junction placed or removed is exactly one step, and an
//! automatic marker is not a toggle target at all.

use std::collections::HashSet;

use super::*;
use crate::schematic::view::schematic_symbol_context;
use crate::state::{
    Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType,
    DesignNoteKind, DocumentationShapeKind, Junction, NetLabel, PendingDesignNotePlacement,
    PendingDocumentationShapePlacement, PendingPortSequence, PlacementAuthority, PortDirection,
    PortDiscipline, PortSignalType, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
    SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SchematicProbe, SheetDefinition,
    SheetPortPolicy, SheetTemplate, Tool, WaveformData, Wire,
};
use rspice_design::connectivity::extract_with_hierarchy as extract;

mod bus_tap;

fn pointer_viewport() -> Viewport {
    Viewport {
        offset: egui::Pos2::ZERO,
        zoom: 1.0,
        bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(400.0)),
    }
}

#[test]
fn library_cell_batch_preserves_binding_transform_and_undo_across_revisions() {
    for sheets in [false, true] {
        let mut state = AppState::default();
        if sheets {
            let document = state.workspace.content.active_schematic_reference();
            state
                .workspace
                .content
                .design_management
                .bootstrap_for_cell_view(&document.key(), "Sheet 1", [])
                .unwrap();
        }
        let descriptor = crate::state::engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.model_type == "d_lut")
            .unwrap();
        let binding = crate::state::builtin_xspice_library_binding(descriptor).unwrap();
        crate::workbench::app::arm_library_cell_placement(&mut state, binding.clone());
        state.schematic.session.editor.preview_rotation = crate::state::Rotation::R90;
        state.schematic.session.editor.preview_mirror_h = true;
        let source = super::super::requests::editor_request_source(&state);
        for position in [Point::new(40, 20), Point::new(80, 20)] {
            place_component(&mut state, ComponentType::CellInstance, position);
            let document = state.schematic.document();
            let placed = document.components.last().unwrap();
            assert_eq!(placed.library_cell.as_ref(), Some(&binding));
            assert_eq!(placed.pos, position);
            assert_eq!(placed.rotation, crate::state::Rotation::R90);
            assert!(placed.mirror_h);
            state.sync_active_schematic_to_workspace();
        }
        assert_eq!(state.schematic.document().components.len(), 2);
        let current = super::super::requests::editor_request_source(&state);
        assert_ne!(source.content_version, current.content_version);
        assert_ne!(source.topology_version, current.topology_version);
        if sheets {
            assert_ne!(source.sheet, current.sheet);
        }
        assert!(
            state
                .schematic
                .session
                .editor
                .pending_library_cell
                .as_ref()
                .unwrap()
                .authority
                .matches(&current)
        );
        assert!(state.schematic.can_undo());
        state.schematic.undo();
        state.sync_active_schematic_to_workspace();
        assert_eq!(state.schematic.document().components.len(), 1);
        place_component(&mut state, ComponentType::CellInstance, Point::new(120, 20));
        assert_eq!(state.schematic.document().components.len(), 2);
        state.schematic.undo();
        assert_eq!(state.schematic.document().components.len(), 1);
        state.schematic.undo();
        assert!(state.schematic.document().components.is_empty());
        assert!(!state.schematic.can_undo());
    }
}

#[test]
fn library_cell_placement_rejects_changed_context_or_permission_without_an_edit() {
    for change in [
        "design",
        "buffer",
        "occurrence",
        "sheet",
        "read-only",
        "safe-mode",
        "missing",
        "tool",
    ] {
        let mut state = AppState::default();
        let master = crate::state::CellViewRef::new("work", "cell_parent", "schematic");
        state
            .workspace
            .descend_into("X1".to_owned(), master.clone(), ViewType::Schematic);
        let first = state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
            .unwrap();
        crate::workbench::app::arm_library_cell_placement(
            &mut state,
            crate::state::LibraryCellInstance::new("work", "child", "schematic"),
        );
        match change {
            "design" => state.design_execution_epoch += 1,
            "buffer" => state.active_schematic_epoch += 1,
            "occurrence" => {
                state.workspace.ascend_one().unwrap();
                state
                    .workspace
                    .descend_into("X2".to_owned(), master, ViewType::Schematic);
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
            "read-only" => state.schematic.session.read_only = true,
            "safe-mode" => state.workbench.safe_mode.activate(
                crate::workbench::state::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "library placement test".to_owned(),
            ),
            "missing" => state.schematic.session.editor.pending_library_cell = None,
            "tool" => state.schematic.arm_tool(Tool::Wire),
            _ => unreachable!(),
        }
        let content = state.schematic.content_version();
        let topology = state.schematic.topology_version();
        place_component(&mut state, ComponentType::CellInstance, Point::new(40, 20));
        assert!(state.schematic.document().components.is_empty(), "{change}");
        assert_eq!(state.schematic.content_version(), content, "{change}");
        assert_eq!(state.schematic.topology_version(), topology, "{change}");
        assert!(!state.schematic.can_undo(), "{change}");
        assert_eq!(
            state.schematic.session.editor.tool,
            Tool::Select,
            "{change}"
        );
    }
}

fn with_test_ui(mut body: impl FnMut(&egui::Ui)) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| body(ui));
    });
}

fn saved_outputs(state: &AppState) -> &[SavedOutput] {
    let plan_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("default state owns a stable plan")
        .id();
    state
        .workspace
        .content
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

    state.schematic.document_mut_for_test().components.push(
        Component::new(41, ComponentType::CellInstance, Point::new(20, 20)).with_library_cell(
            crate::state::LibraryCellInstance::new(
                &schematic_reference.library,
                &schematic_reference.cell,
                &schematic_reference.view,
            ),
        ),
    );
    state.schematic.document_mut_for_test().components.push(
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
        .content
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
    assert_eq!(state.workspace.content.active_view, veriloga_reference);
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
        .document_mut_for_test()
        .wires
        .push(Wire::new(91, vec![Point::new(0, 20), Point::new(80, 20)]));
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(92, Point::new(40, 20), "OUT"));

    assert!(
        state
            .simulation
            .cross_probe
            .net_at_in(
                &state.workspace.content.active_view,
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
    state.schematic.document_mut_for_test().components.push(
        Component::new(17, ComponentType::CellInstance, Point::origin())
            .with_name_value("XAMP", ""),
    );
    let symbols = schematic_symbol_context(&state);

    assert_eq!(
        component_probe_expression(&state, 17, Point::origin(), &symbols),
        None,
        "an unresolved terminal must fail closed instead of inventing V(XAMP)"
    );
}

#[test]
fn voltage_source_component_probe_preserves_device_current_semantics() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.push(
        Component::new(23, ComponentType::VoltageSource, Point::origin())
            .with_name_value("VBIAS", "1.8"),
    );
    let symbols = schematic_symbol_context(&state);

    assert_eq!(
        component_probe_expression(&state, 23, Point::origin(), &symbols),
        Some(('I', "VBIAS".to_owned()))
    );
}

#[test]
fn ordinary_component_body_probe_requests_device_current_not_nearest_pin_voltage() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.push(
        Component::new(24, ComponentType::Resistor, Point::origin()).with_name_value("RLOAD", "1k"),
    );
    let symbols = schematic_symbol_context(&state);

    assert_eq!(
        component_probe_expression(&state, 24, Point::origin(), &symbols),
        Some(('I', "RLOAD".to_owned()))
    );
}

#[test]
fn exact_component_terminal_probe_requests_its_node_voltage() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.push(
        Component::new(25, ComponentType::Resistor, Point::origin()).with_name_value("RLOAD", "1k"),
    );
    let symbols = schematic_symbol_context(&state);

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
            .document_mut_for_test()
            .components
            .push(Component::new(id, kind, Point::origin()));
        let symbols = schematic_symbol_context(&state);

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
    let payloads_before = state.workspace.content.simulation_plan_payloads.clone();
    state.sim_setup.analysis_plan = None;

    let outcome = request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)"));

    assert!(matches!(outcome, ProbeSignalOutcome::Rejected { .. }));
    assert_eq!(
        state.workspace.content.simulation_plan_payloads,
        payloads_before
    );
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
    assert_eq!(state.schematic.document().probes.len(), 1);
    assert_eq!(state.schematic.document().probes[0].id, id);
    assert_eq!(state.schematic.document().probes[0].position, position);
    assert_eq!(
        state.schematic.document().probes[0].reference,
        format!("P{id}")
    );
    assert!(
        state.schematic.document().probes[0]
            .source_expression
            .is_none()
    );
    assert_eq!(
        state.schematic.undo_description(),
        Some("place schematic probe")
    );

    assert!(state.schematic.undo());
    assert!(state.schematic.document().probes.is_empty());
}

#[test]
fn probe_marker_rejects_read_only_and_replaced_view_identity_without_mutation() {
    let mut read_only = AppState::default();
    read_only.schematic.session.read_only = true;
    assert!(retain_probe_flag(&mut read_only, Point::origin(), None, None).is_err());
    assert!(read_only.schematic.document().probes.is_empty());
    assert!(!read_only.schematic.can_undo());

    let mut read_only_reference = AppState::default();
    read_only_reference
        .workspace
        .content
        .set_active_read_only_reference(true);
    assert!(retain_probe_flag(&mut read_only_reference, Point::origin(), None, None).is_err());
    assert!(read_only_reference.schematic.document().probes.is_empty());
    assert!(!read_only_reference.schematic.can_undo());

    let mut replaced = AppState::default();
    replaced.workspace.content.active_view.view = "symbol".to_owned();
    assert!(retain_probe_flag(&mut replaced, Point::origin(), None, None).is_err());
    assert!(replaced.schematic.document().probes.is_empty());
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

    assert_eq!(state.schematic.document().probes[0].reference, "V(OUT)");
    assert_eq!(
        state.schematic.document().probes[0]
            .source_expression
            .as_deref(),
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
    assert_eq!(state.schematic.document().probes[0].reference, "V(/X1/n1)");
    assert_eq!(
        state.schematic.document().probes[0]
            .source_expression
            .as_deref(),
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

    assert_eq!(
        state.schematic.document().probes[0].plan_id,
        Some(binding.0)
    );
    assert_eq!(
        state.schematic.document().probes[0].saved_output_id,
        Some(binding.1)
    );
    assert!(state.schematic.document().probes[0].enabled);
    assert!(state.schematic.document().probes[0].plot_on_materialization);
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
    assert_eq!(state.schematic.document().probes.len(), 1);
    assert_eq!(
        state.schematic.session.editor.selection.single_probe(),
        Some(id)
    );
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
    assert!(state.schematic.document().probes.is_empty());
    assert!(!state.schematic.session.is_dirty);
    assert!(!state.schematic.can_undo());
}

#[test]
fn route_finish_helper_commits_wire_and_bus_without_secondary_click() {
    let mut state = AppState::default();
    state.schematic.start_wire(Point::origin());
    state.schematic.extend_wire(Point::new(20, 0));
    with_test_ui(|ui| assert!(finish_active_route(ui, &mut state)));
    assert!(!state.schematic.session.editor.wire_drawing.active);
    assert_eq!(state.schematic.document().wires.len(), 1);

    state
        .schematic
        .start_bus(
            Point::new(0, 20),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
    state.schematic.extend_bus(Point::new(20, 20));
    with_test_ui(|ui| assert!(finish_active_route(ui, &mut state)));
    assert!(!state.schematic.session.editor.bus_drawing.active);
    assert_eq!(state.schematic.document().buses.len(), 1);
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
        .content
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
        .content
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
fn armed_move_exclusively_owns_selection_drag_routing() {
    assert!(select_drag_is_authorized(Tool::Select, false));
    assert!(!select_drag_is_authorized(Tool::Select, true));
    assert!(!select_drag_is_authorized(Tool::MoveSelection, true));
    assert!(!select_drag_is_authorized(Tool::MoveSelection, false));
}

/// Arm a sequence of `names` on `state`, for the document `state` is in.
fn arm_pins(state: &mut AppState, names: &[&str], direction: PortDirection) {
    let authority = PlacementAuthority::new(super::super::requests::editor_request_source(state));
    state.schematic.session.editor.pending_port_sequence = Some(
        PendingPortSequence::new(
            names.iter().map(|name| (*name).to_owned()),
            direction,
            PortSignalType::Logic,
            PortDiscipline::Logic,
        )
        .with_authority(authority),
    );
    state.schematic.session.editor.tool = Tool::Place(ComponentType::Port);
}

#[test]
fn validated_port_contract_places_once_and_undo_redo_is_exact() {
    let mut state = AppState::default();
    arm_pins(&mut state, &["BIAS_EN"], PortDirection::In);

    place_component(&mut state, ComponentType::Port, Point::new(20, 30));

    assert_eq!(state.schematic.document().components.len(), 1);
    let placed = state.schematic.document().components[0].clone();
    assert_eq!(placed.pos, Point::new(20, 30));
    assert_eq!(placed.value, "BIAS_EN");
    let contract = placed.port_contract().expect("typed interface contract");
    assert_eq!(contract.direction, PortDirection::In);
    assert_eq!(contract.signal_type, PortSignalType::Logic);
    assert_eq!(contract.discipline, PortDiscipline::Logic);
    assert!(!contract.documentation.is_empty());
    // One name, so the batch is finished and the tool returns to Select.
    assert_eq!(state.schematic.session.editor.tool, Tool::Select);
    assert!(
        state
            .schematic
            .session
            .editor
            .pending_port_sequence
            .is_none()
    );
    assert_eq!(
        state.schematic.undo_description(),
        Some("place interface port")
    );

    assert!(state.schematic.undo());
    assert!(state.schematic.document().components.is_empty());
    assert!(state.schematic.redo());
    assert_eq!(state.schematic.document().components, [placed]);
}

/// The batch: one click per name, one undo record per click, the interface
/// order following the document, and the tool ending itself when the names run
/// out rather than on the first click.
#[test]
fn each_click_places_the_next_name_with_the_next_interface_order() {
    let mut state = AppState::default();
    let document = state.workspace.content.active_schematic_reference();
    state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&document.key(), "Sheet 1", [])
        .unwrap();
    arm_pins(&mut state, &["INP", "INN", "OUT"], PortDirection::In);

    for (index, expected) in ["INP", "INN", "OUT"].into_iter().enumerate() {
        place_component(
            &mut state,
            ComponentType::Port,
            Point::new(20 * (index as i32 + 1), 30),
        );
        assert_eq!(state.schematic.document().components.len(), index + 1);
        let placed = &state.schematic.document().components[index];
        assert_eq!(placed.value, expected);
        assert_eq!(
            placed
                .port_contract()
                .and_then(|contract| contract.netlist_order),
            Some(index + 1)
        );
        if index < 2 {
            assert_eq!(
                state.schematic.session.editor.tool,
                Tool::Place(ComponentType::Port),
                "the tool stays armed while names remain"
            );
            assert_eq!(
                state
                    .schematic
                    .session
                    .editor
                    .pending_port_sequence
                    .as_ref()
                    .and_then(PendingPortSequence::next_name),
                Some(["INP", "INN", "OUT"][index + 1])
            );
        }
    }

    assert_eq!(state.schematic.session.editor.tool, Tool::Select);
    assert!(
        state
            .schematic
            .session
            .editor
            .pending_port_sequence
            .is_none()
    );
    for expected in [2, 1, 0] {
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.document().components.len(), expected);
    }
}

/// The ghost's rotation and mirror are the placed pin's rotation and mirror:
/// R and M act on the object, not on a decoration of it.
#[test]
fn rotation_and_mirror_of_the_ghost_land_on_the_placed_pin() {
    let mut state = AppState::default();
    arm_pins(&mut state, &["OUT"], PortDirection::Out);
    state.schematic.session.editor.preview_rotation = crate::state::Rotation::R90;
    state.schematic.session.editor.preview_mirror_h = true;

    place_component(&mut state, ComponentType::Port, Point::new(20, 30));

    let placed = &state.schematic.document().components[0];
    assert_eq!(placed.rotation, crate::state::Rotation::R90);
    assert!(placed.mirror_h);
}

/// A name taken between arming and the click is refused by the model, and the
/// batch survives: the reader can free the name and click again.
#[test]
fn a_name_taken_after_arming_is_refused_at_the_click_and_the_sequence_survives() {
    let mut state = AppState::default();
    arm_pins(&mut state, &["EN", "OUT"], PortDirection::Out);
    let taken = crate::state::PendingPortPlacement::from_contract(
        "en",
        PortDirection::In,
        PortSignalType::Logic,
        PortDiscipline::Logic,
        state.schematic.topology_version(),
        state.schematic.next_interface_order(),
    );
    state
        .schematic
        .place_pending_port(Point::origin(), taken)
        .unwrap();

    place_component(&mut state, ComponentType::Port, Point::new(20, 30));

    assert_eq!(
        state.schematic.document().components.len(),
        1,
        "nothing was placed"
    );
    assert_eq!(
        state.schematic.session.editor.tool,
        Tool::Place(ComponentType::Port)
    );
    assert_eq!(
        state
            .schematic
            .session
            .editor
            .pending_port_sequence
            .as_ref()
            .and_then(PendingPortSequence::next_name),
        Some("EN")
    );
    assert!(state.schematic.undo(), "free the conflicting name");
    place_component(&mut state, ComponentType::Port, Point::new(20, 30));
    assert_eq!(state.schematic.document().components.len(), 1);
    assert_eq!(state.schematic.document().components[0].value, "EN");
    place_component(&mut state, ComponentType::Port, Point::new(40, 30));
    assert_eq!(state.schematic.document().components.len(), 2);
    assert_eq!(state.schematic.document().components[1].value, "OUT");
    assert_eq!(state.schematic.session.editor.tool, Tool::Select);
}

#[test]
fn validated_design_note_contract_places_once_without_changing_topology() {
    let mut state = AppState::default();
    let pending = PendingDesignNotePlacement::new(
        DesignNoteKind::PlainText,
        "Bias network",
        state.schematic.topology_version(),
        &state.schematic.document().design_notes,
    )
    .unwrap()
    .with_source(super::super::requests::editor_request_source(&state));
    let topology = state.schematic.topology_version();
    state.schematic.session.editor.pending_design_note = Some(pending);
    state.schematic.session.editor.tool = Tool::DesignNote;

    place_pending_design_note(&mut state, Point::new(20, 30));

    assert_eq!(state.schematic.document().design_notes.len(), 1);
    assert_eq!(
        state.schematic.document().design_notes[0].pos,
        Point::new(20, 30)
    );
    assert_eq!(state.schematic.topology_version(), topology);
    assert!(state.schematic.session.editor.pending_design_note.is_none());
    assert_eq!(state.schematic.session.editor.tool, Tool::Select);
    assert!(state.schematic.undo());
    assert!(state.schematic.document().design_notes.is_empty());
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
        state.schematic.session.editor.pending_documentation_shape = Some(
            PendingDocumentationShapePlacement::new(
                kind,
                topology,
                &state.schematic.document().documentation_shapes,
            )
            .with_source(super::super::requests::editor_request_source(&state)),
        );
        state.schematic.session.editor.tool = Tool::DocumentationShape;

        for (index, point) in points.iter().copied().enumerate() {
            let finish = finish_on_last_click && index + 1 == points.len();
            with_test_ui(|ui| handle_documentation_shape_click(ui, &mut state, point, finish));
        }

        assert_eq!(
            state.schematic.document().documentation_shapes.len(),
            1,
            "{kind:?}"
        );
        assert_eq!(
            state.schematic.document().documentation_shapes[0].kind(),
            kind
        );
        assert_eq!(state.schematic.topology_version(), topology);
        assert!(state.schematic.document().components.is_empty());
        assert!(state.schematic.document().wires.is_empty());
        assert_eq!(state.schematic.session.editor.tool, Tool::Select);
        assert!(
            state
                .schematic
                .session
                .editor
                .pending_documentation_shape
                .is_none()
        );
        assert!(
            state
                .schematic
                .session
                .editor
                .documentation_shape_drawing
                .points
                .is_empty()
        );
        assert_eq!(
            state.schematic.undo_description(),
            Some("draw documentation shape")
        );
        assert!(state.schematic.undo());
        assert!(state.schematic.document().documentation_shapes.is_empty());
        assert!(
            !state.schematic.can_undo(),
            "{kind:?} must create one undo step"
        );
    }
}

#[test]
fn stale_documentation_shape_authority_is_consumed_without_document_mutation() {
    for finish_polygon in [false, true] {
        for change in [
            "document",
            "occurrence",
            "sheet",
            "content",
            "read-only",
            "safe-mode",
        ] {
            let mut state = AppState::default();
            let master = crate::state::CellViewRef::new("work", "shape_child", "schematic");
            state
                .workspace
                .descend_into("X1".to_owned(), master.clone(), ViewType::Schematic);
            let first = state
                .workspace
                .content
                .design_management
                .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
                .unwrap();
            state.schematic.session.editor.pending_documentation_shape = Some(
                PendingDocumentationShapePlacement::new(
                    DocumentationShapeKind::Polygon,
                    state.schematic.topology_version(),
                    &state.schematic.document().documentation_shapes,
                )
                .with_source(super::super::requests::editor_request_source(&state)),
            );
            state.schematic.session.editor.tool = Tool::DocumentationShape;
            state
                .schematic
                .session
                .editor
                .documentation_shape_drawing
                .points = vec![Point::origin(), Point::new(20, 0), Point::new(10, 10)];
            state
                .schematic
                .session
                .editor
                .documentation_shape_drawing
                .keyboard_cursor = Some(Point::new(10, 10));
            match change {
                "document" => state.active_schematic_epoch += 1,
                "occurrence" => {
                    state.workspace.ascend_one().unwrap();
                    state
                        .workspace
                        .descend_into("X2".to_owned(), master, ViewType::Schematic);
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
                "content" => {
                    let topology = state.schematic.topology_version();
                    let content = state.schematic.content_version();
                    let note = PendingDesignNotePlacement::new(
                        DesignNoteKind::PlainText,
                        "Changed context",
                        topology,
                        &[],
                    )
                    .unwrap();
                    state
                        .schematic
                        .place_pending_design_note(Point::origin(), note)
                        .unwrap();
                    assert_eq!(state.schematic.topology_version(), topology);
                    assert_ne!(state.schematic.content_version(), content);
                    state.schematic.init_undo_history();
                }
                "read-only" => state.schematic.session.read_only = true,
                "safe-mode" => state.workbench.safe_mode.activate(
                    crate::workbench::state::LocalSafeModeOptions {
                        open_project_read_only: true,
                        ..Default::default()
                    },
                    "shape gesture test".to_owned(),
                ),
                _ => unreachable!(),
            }
            let content = state.schematic.content_version();
            let notes = state.schematic.document().design_notes.clone();

            if finish_polygon {
                // Enter completes a polygon without going through the pointer-click gate.
                let ctx = egui::Context::default();
                let _ = ctx.run_ui(
                    egui::RawInput {
                        events: vec![egui::Event::Key {
                            key: egui::Key::Enter,
                            physical_key: None,
                            pressed: true,
                            repeat: false,
                            modifiers: egui::Modifiers::NONE,
                        }],
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            // accessibility-pointer-shim: test-only canvas event harness.
                            let response = ui.interact(
                                ui.max_rect(),
                                egui::Id::new("stale-shape"),
                                egui::Sense::click_and_drag(),
                            );
                            let grid = state.schematic.document().grid_size;
                            handle_documentation_shape_keyboard(
                                ui,
                                &response,
                                &mut state,
                                &pointer_viewport(),
                                grid,
                            );
                        });
                    },
                );
            } else {
                with_test_ui(|ui| {
                    handle_documentation_shape_click(ui, &mut state, Point::new(0, 0), true)
                });
            }

            assert!(
                state.schematic.document().documentation_shapes.is_empty(),
                "{change}, Enter={finish_polygon}"
            );
            assert_eq!(state.schematic.content_version(), content);
            assert_eq!(state.schematic.document().design_notes, notes);
            assert!(
                state
                    .schematic
                    .session
                    .editor
                    .pending_documentation_shape
                    .is_none()
            );
            assert!(
                state
                    .schematic
                    .session
                    .editor
                    .documentation_shape_drawing
                    .points
                    .is_empty()
            );
            assert_eq!(state.schematic.session.editor.tool, Tool::Select);
            assert!(!state.schematic.can_undo());
        }
    }
}

#[test]
fn queued_shape_input_cannot_edit_a_replaced_gesture_or_changed_context() {
    for change in [
        "none", "rearmed", "document", "draft", "kind", "tool", "modal",
    ] {
        let mut state = AppState::default();
        state.schematic.session.editor.pending_documentation_shape = Some(
            PendingDocumentationShapePlacement::new(
                DocumentationShapeKind::Line,
                state.schematic.topology_version(),
                &[],
            )
            .with_source(super::super::requests::editor_request_source(&state)),
        );
        state.schematic.session.editor.tool = Tool::DocumentationShape;
        let drawing = &mut state.schematic.session.editor.documentation_shape_drawing;
        drawing.points.push(Point::origin());
        drawing.keyboard_cursor = Some(Point::origin());
        let expected = drawing.clone();
        let mut next = expected.clone();
        next.keyboard_cursor = Some(Point::new(10, 0));
        next.keyboard_active = true;
        let request = capture_shape_input(
            &state,
            ShapeInputTransition {
                expected,
                next,
                action: Some(ShapeInputAction::PlacePoint(Point::new(10, 0))),
            },
        );
        match change {
            "none" => {}
            "rearmed" => {
                // Same source, kind and visible draft, but a different gesture lifetime.
                let drawing = &mut state.schematic.session.editor.documentation_shape_drawing;
                drawing.clear();
                drawing.points.push(Point::origin());
                drawing.keyboard_cursor = Some(Point::origin());
            }
            "document" => state.active_schematic_epoch += 1,
            "draft" => state
                .schematic
                .session
                .editor
                .documentation_shape_drawing
                .points
                .push(Point::new(20, 20)),
            "kind" => {
                state
                    .schematic
                    .session
                    .editor
                    .pending_documentation_shape
                    .as_mut()
                    .unwrap()
                    .kind = DocumentationShapeKind::Rectangle
            }
            "tool" => state.schematic.session.editor.tool = Tool::Select,
            "modal" => state.dialogs.about = true,
            _ => unreachable!(),
        }
        let drawing_before = state
            .schematic
            .session
            .editor
            .documentation_shape_drawing
            .clone();
        let pending_before = state
            .schematic
            .session
            .editor
            .pending_documentation_shape
            .clone();
        with_test_ui(|ui| apply_shape_input(ui, &mut state, request.clone()));
        if change == "none" {
            assert_eq!(state.schematic.document().documentation_shapes.len(), 1);
            assert!(state.schematic.undo());
        } else {
            assert_eq!(
                state.schematic.session.editor.documentation_shape_drawing, drawing_before,
                "{change}"
            );
            assert_eq!(
                state.schematic.session.editor.pending_documentation_shape, pending_before,
                "{change}"
            );
        }
        assert!(
            state.schematic.document().documentation_shapes.is_empty(),
            "{change}"
        );
        assert!(!state.schematic.can_undo(), "{change}");
    }
}

#[test]
fn focused_keyboard_cursor_places_exact_grid_resolved_shape_points() {
    let mut state = AppState::default();
    let grid = state.schematic.document().grid_size;
    state.schematic.session.editor.pending_documentation_shape = Some(
        PendingDocumentationShapePlacement::new(
            DocumentationShapeKind::Line,
            state.schematic.topology_version(),
            &state.schematic.document().documentation_shapes,
        )
        .with_source(super::super::requests::editor_request_source(&state)),
    );
    state.schematic.session.editor.tool = Tool::DocumentationShape;
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
        state
            .schematic
            .session
            .editor
            .documentation_shape_drawing
            .points,
        vec![Point::new(grid, 0)]
    );
    keyboard_frame(&[egui::Key::ArrowDown, egui::Key::Enter], &mut state);

    assert_eq!(state.schematic.document().documentation_shapes.len(), 1);
    assert_eq!(
        state.schematic.document().documentation_shapes[0].geometry,
        crate::state::DocumentationShapeGeometry::Line {
            start: Point::new(grid, 0),
            end: Point::new(grid, grid),
        }
    );
    assert_eq!(state.schematic.session.editor.tool, Tool::Select);
    assert!(
        state
            .schematic
            .session
            .editor
            .documentation_shape_drawing
            .points
            .is_empty()
    );
    assert!(
        state
            .schematic
            .session
            .editor
            .documentation_shape_drawing
            .keyboard_cursor
            .is_none()
    );
}

#[test]
fn stale_design_note_authority_is_consumed_without_document_mutation() {
    for change in [
        "document",
        "occurrence",
        "sheet",
        "content",
        "read-only",
        "safe-mode",
    ] {
        let mut state = AppState::default();
        let master = crate::state::CellViewRef::new("work", "note_child", "schematic");
        state
            .workspace
            .descend_into("X1".to_owned(), master.clone(), ViewType::Schematic);
        let first = state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
            .unwrap();
        let source = super::super::requests::editor_request_source(&state);
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::ReviewNote,
            "Review bias path",
            state.schematic.topology_version(),
            &state.schematic.document().design_notes,
        )
        .unwrap()
        .with_source(source.clone());
        state.schematic.session.editor.pending_design_note = Some(pending);
        state.schematic.session.editor.tool = Tool::DesignNote;
        match change {
            "document" => state.active_schematic_epoch += 1,
            "occurrence" => {
                state.workspace.ascend_one().unwrap();
                state
                    .workspace
                    .descend_into("X2".to_owned(), master, ViewType::Schematic);
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
            "content" => {
                let topology = state.schematic.topology_version();
                let shape = PendingDocumentationShapePlacement::new(
                    DocumentationShapeKind::Line,
                    topology,
                    &[],
                );
                state
                    .schematic
                    .commit_documentation_shape(
                        shape,
                        crate::state::DocumentationShapeGeometry::Line {
                            start: Point::origin(),
                            end: Point::new(10, 10),
                        },
                    )
                    .unwrap();
                assert_eq!(state.schematic.topology_version(), topology);
                assert_ne!(state.schematic.content_version(), source.content_version);
                state.schematic.init_undo_history();
            }
            "read-only" => state.schematic.session.read_only = true,
            "safe-mode" => state.workbench.safe_mode.activate(
                crate::workbench::state::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "note gesture test".to_owned(),
            ),
            _ => unreachable!(),
        }
        let content = state.schematic.content_version();
        let shapes = state.schematic.document().documentation_shapes.clone();

        place_pending_design_note(&mut state, Point::new(20, 30));

        assert!(
            state.schematic.document().design_notes.is_empty(),
            "{change}"
        );
        assert_eq!(state.schematic.content_version(), content);
        assert_eq!(state.schematic.document().documentation_shapes, shapes);
        assert!(state.schematic.session.editor.pending_design_note.is_none());
        assert_eq!(state.schematic.session.editor.tool, Tool::Select);
        assert!(!state.schematic.can_undo());
    }
}

#[test]
fn port_placement_without_a_current_validated_contract_fails_closed() {
    let mut state = AppState::default();
    state.schematic.session.editor.tool = Tool::Place(ComponentType::Port);

    place_component(&mut state, ComponentType::Port, Point::new(20, 30));

    assert!(state.schematic.document().components.is_empty());
    assert!(!state.schematic.can_undo());
    assert_eq!(state.schematic.session.editor.tool, Tool::Select);
    assert!(
        state
            .schematic
            .session
            .editor
            .pending_port_sequence
            .is_none()
    );
}

/// A topology change no longer ends a batch. It cannot: the first pin placed
/// bumps the version, so a batch that refused a changed topology could never
/// place its second name.
#[test]
fn a_topology_change_alone_does_not_end_the_sequence() {
    let mut state = AppState::default();
    arm_pins(&mut state, &["OUT", "OUT_N"], PortDirection::Out);
    state.schematic.bump_topology_version();
    let note = PendingDesignNotePlacement::new(
        DesignNoteKind::PlainText,
        "Pin context",
        state.schematic.topology_version(),
        &[],
    )
    .unwrap();
    state
        .schematic
        .place_pending_design_note(Point::origin(), note)
        .unwrap();

    place_component(&mut state, ComponentType::Port, Point::new(40, 10));

    assert_eq!(state.schematic.document().components.len(), 1);
    assert_eq!(state.schematic.document().components[0].value, "OUT");
    assert_eq!(
        state.schematic.session.editor.tool,
        Tool::Place(ComponentType::Port)
    );
}

/// A batch is named for one cell. If that cell is no longer the one on
/// screen, the batch ends rather than placing its pins somewhere else.
#[test]
fn a_changed_document_ends_the_sequence_without_placing() {
    for change in [
        "document",
        "design",
        "occurrence",
        "sheet",
        "missing",
        "read-only",
        "safe-mode",
    ] {
        let mut state = AppState::default();
        let master = crate::state::CellViewRef::new("work", "pin_child", "schematic");
        state
            .workspace
            .descend_into("X1".to_owned(), master.clone(), ViewType::Schematic);
        let first = state
            .workspace
            .content
            .design_management
            .bootstrap_for_cell_view(&master.key(), "Sheet 1", [])
            .unwrap();
        arm_pins(&mut state, &["OUT"], PortDirection::Out);
        match change {
            "document" => state.active_schematic_epoch += 1,
            "design" => state.design_execution_epoch += 1,
            "occurrence" => {
                state.workspace.ascend_one().unwrap();
                state
                    .workspace
                    .descend_into("X2".to_owned(), master, ViewType::Schematic);
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
            "missing" => {
                state
                    .schematic
                    .session
                    .editor
                    .pending_port_sequence
                    .as_mut()
                    .unwrap()
                    .authority = None
            }
            "read-only" => state.schematic.session.read_only = true,
            "safe-mode" => state.workbench.safe_mode.activate(
                crate::workbench::state::LocalSafeModeOptions {
                    open_project_read_only: true,
                    ..Default::default()
                },
                "pin placement test".to_owned(),
            ),
            _ => unreachable!(),
        }
        let content = state.schematic.content_version();
        place_component(&mut state, ComponentType::Port, Point::new(40, 10));

        assert!(state.schematic.document().components.is_empty(), "{change}");
        assert_eq!(state.schematic.content_version(), content);
        assert!(!state.schematic.can_undo());
        assert_eq!(state.schematic.session.editor.tool, Tool::Select);
        assert!(
            state
                .schematic
                .session
                .editor
                .pending_port_sequence
                .is_none()
        );
    }
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
        .document_mut_for_test()
        .components
        .push(Component::new(10, ComponentType::Resistor, point));
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(11, Point::new(0, 0), Point::new(20, 0)));
    state
        .schematic
        .document_mut_for_test()
        .junctions
        .push(Junction::new(12, point));
    state.schematic.document_mut_for_test().buses.push(bus);
    state.schematic.document_mut_for_test().bus_taps.push(tap);
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
    state.schematic.document_mut_for_test().components.clear();
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
    state.schematic.document_mut_for_test().bus_taps.clear();
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
    state.schematic.document_mut_for_test().junctions.clear();
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
    state.schematic.document_mut_for_test().buses.clear();
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
    state.schematic.document_mut_for_test().components = vec![
        Component::new(20, ComponentType::Capacitor, point),
        Component::new(10, ComponentType::Resistor, point),
    ];
    let key = state.workspace.content.active_schematic_reference().key();
    let first = state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10, 20])
        .unwrap();
    let catalog = state
        .workspace
        .content
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
    state.schematic.document_mut_for_test().probes = vec![
        SchematicProbe::new(30, point, "V(active)", Some("V(active)".to_owned())).unwrap(),
        SchematicProbe::new(31, point, "V(hidden)", Some("V(hidden)".to_owned())).unwrap(),
    ];
    let key = state.workspace.content.active_schematic_reference().key();
    let first = state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [30, 31])
        .unwrap();
    let catalog = state
        .workspace
        .content
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
    state.schematic.document_mut_for_test().buses.push(bus);
    state.schematic.document_mut_for_test().bus_taps.push(tap);
    let symbol_context = schematic_symbol_context(&state);
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

    assert_eq!(
        state.schematic.session.editor.selection.single_bus_tap(),
        Some(21)
    );
    assert!(matches!(
        state.dialogs.object_properties.draft,
        Some(crate::workbench::app::ObjectPropertiesDraft::BusTap(_))
    ));
}

#[test]
fn net_label_text_bounds_are_a_first_class_pointer_target() {
    let mut state = AppState::default();
    let label = NetLabel::new(31, Point::new(40, 40), "afe_out");
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(label.clone());
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(Component::new(10, ComponentType::Resistor, label.pos));
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(11, Point::new(0, 40), Point::new(100, 40)));
    let symbol_context = SchematicSymbolContext::default();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
    let viewport = pointer_viewport();
    let hit = rspice_schematic_editor::view::net_labels::hit_bounds(&ctx, &viewport, &label)
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

    state.schematic.document_mut_for_test().net_labels.clear();
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
    state.schematic.document_mut_for_test().design_notes.push(
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

    state.schematic.document_mut_for_test().design_notes.push(
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
    state.schematic.document_mut_for_test().wires = vec![
        Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
        Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
    ];
    state.schematic.bump_topology_version();
    state
        .schematic
        .session
        .editor
        .net_highlight
        .highlight_wires([1].into_iter().collect());

    assert_eq!(
        commit_explicit_junction(&mut state, Point::new(20, 20)),
        JunctionPlacementOutcome::Placed(Point::new(20, 20))
    );
    assert!(state.schematic.has_junction(Point::new(20, 20)));
    assert!(!state.schematic.session.editor.net_highlight.active);
    assert!(
        state
            .schematic
            .session
            .editor
            .net_highlight
            .highlighted_wires
            .is_empty()
    );
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
    state.schematic.document_mut_for_test().wires = vec![
        Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
        Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
    ];
    state.schematic.add_junction(Point::new(20, 20));
    state
        .schematic
        .session
        .editor
        .net_highlight
        .highlight_wires([1, 2].into_iter().collect());

    assert_eq!(
        commit_explicit_junction(&mut state, Point::new(20, 20)),
        JunctionPlacementOutcome::Removed(Point::new(20, 20))
    );
    assert!(!state.schematic.has_junction(Point::new(20, 20)));
    assert!(!state.schematic.session.editor.net_highlight.active);
    assert!(
        state
            .schematic
            .session
            .editor
            .net_highlight
            .highlighted_wires
            .is_empty()
    );
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
    plain.schematic.document_mut_for_test().wires = vec![
        Wire::segment(1, Point::new(0, 0), Point::new(40, 0)),
        Wire::segment(2, Point::new(40, 0), Point::new(40, 40)),
    ];
    plain
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(3, Point::new(20, 0), "sense"));

    let mut separated = AppState::default();
    separated.schematic.document_mut_for_test().wires = vec![
        Wire::segment(11, Point::new(0, 0), Point::new(40, 0)),
        Wire::segment(12, Point::new(0, 100), Point::new(40, 100)),
    ];
    separated.schematic.document_mut_for_test().net_labels = vec![
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
    tapped.schematic.document_mut_for_test().buses.push(bus);
    tapped.schematic.document_mut_for_test().bus_taps.push(tap);
    tapped
        .schematic
        .document_mut_for_test()
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

/// The conductors one alt-click lights, and the name it lit them under.
fn lit_net(state: &mut AppState, wire_id: u64) -> (Option<String>, HashSet<u64>) {
    highlight_canvas_net(state, |net| net.wire_ids.contains(&wire_id));
    let highlight = &state.schematic.session.editor.net_highlight;
    (
        highlight.selected_net_name.clone(),
        highlight.highlighted_wires.clone(),
    )
}

/// The differential guard. What the canvas lights for a conductor is the net
/// the one extraction gives that conductor, over every fixture — a second
/// connectivity owner growing back on the canvas fails here first.
#[test]
fn the_canvas_net_partition_is_the_one_extractions_partition() {
    for (label, mut state) in canvas_net_corpus() {
        let connectivity = extract(&state.schematic, None);
        let wire_ids = state
            .schematic
            .document()
            .wires
            .iter()
            .map(|wire| wire.id)
            .collect::<Vec<_>>();
        for wire_id in wire_ids {
            let solved = connectivity
                .net_of_wire(wire_id)
                .map(|net| net.wires.iter().copied().collect())
                .unwrap_or_default();
            let (_, lit) = lit_net(&mut state, wire_id);
            assert_eq!(
                lit, solved,
                "{label}: conductor {wire_id} lights a different net than the deck solves"
            );
        }
    }
}

/// Alt-click is a net gesture, not a geometry gesture: one name across two
/// drawn groups lights both, under the node name a probe would emit.
#[test]
fn alt_click_lights_every_group_the_deck_joins_under_one_name() {
    let (_, mut state) = canvas_net_corpus().swap_remove(1);
    let (name, wire_ids) = lit_net(&mut state, 11);
    assert_eq!(name.as_deref(), Some("VDD"));
    assert_eq!(
        wire_ids,
        [11, 12].into_iter().collect::<HashSet<_>>(),
        "the separated group carrying the same name is on the same node"
    );
}

#[test]
fn automatic_t_marker_is_not_an_explicit_junction_toggle_target() {
    let point = Point::new(20, 20);
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().wires = vec![
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
