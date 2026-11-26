//! What a probe click asks for: the waveform it shows or hides, the plan
//! output it saves once, and the observable a wire, terminal or component
//! body names, which fails closed rather than inventing one.

use super::*;
use crate::workbench::design::test_host::{DesignTestHost, canvas_parts};

/// A transient run whose active analysis holds `V(OUT)`.
fn hold_out_in_the_last_run(state: &mut DesignTestHost) {
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![RetainedTrace::new(
                "V(OUT)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#ffffff",
            )]),
    );
    state.model.simulation.retained.runs.insert(0, run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
}

/// What the probe asked the results workspace to plot, if anything.
fn plot_request(state: &DesignTestHost) -> Option<(String, ProbePlot, String)> {
    state
        .workbench
        .outbox
        .peek::<PlotProbedTrace>()
        .map(|probed| {
            (
                probed.trace.source_name().to_owned(),
                probed.plot,
                probed.display.clone(),
            )
        })
}

/// A probe of a signal the last run holds asks the results workspace to
/// plot that run's trace, through its Data Browser's tick, and never to take
/// one off. Looking writes nothing: the plan saves no output for it, so the
/// run looked at stays current.
#[test]
fn a_probe_of_a_signal_the_run_holds_asks_the_results_to_plot_it() {
    let mut state = DesignTestHost::default();
    hold_out_in_the_last_run(&mut state);

    for _ in 0..2 {
        assert_eq!(
            request_probe_signal(
                &mut canvas_parts(&mut state),
                &OccurrenceProbeSpelling::verbatim("V(OUT)"),
                ProbePlot::Show
            ),
            ProbeSignalOutcome::PlotRequested
        );
        assert_eq!(
            plot_request(&state),
            Some(("V(OUT)".to_owned(), ProbePlot::Show, "V(OUT)".to_owned()))
        );
        assert!(saved_outputs(&state).is_empty());
        state.workbench.outbox = Default::default();
    }
}

/// The inspector's Plot button asks for its trace to be put on the plot or
/// taken off; the results workspace knows which. A trace the run holds needs
/// no saved output.
#[test]
fn the_inspector_plot_button_asks_for_a_toggle() {
    let mut state = DesignTestHost::default();
    hold_out_in_the_last_run(&mut state);

    toggle_probe_with_feedback(&mut canvas_parts(&mut state), "OUT", "V(OUT)");

    assert_eq!(
        plot_request(&state),
        Some(("V(OUT)".to_owned(), ProbePlot::Toggle, "V(OUT)".to_owned()))
    );
    assert!(saved_outputs(&state).is_empty());
}

#[test]
fn wire_probe_resolves_from_live_connectivity_without_retained_run_data() {
    let mut state = DesignTestHost::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::new(91, vec![Point::new(0, 20), Point::new(80, 20)]));
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(92, Point::new(40, 20), "OUT"));

    assert!(
        state
            .model
            .simulation
            .source
            .cross_probe
            .net_at_in(
                &state.model.workspace.content.active_view,
                state.model.schematic.topology_version(),
                None,
                Point::new(40, 20),
            )
            .is_none(),
        "the fixture must not depend on retained simulation cross-probe data"
    );
    assert_eq!(
        live_wire_probe_net_name(&canvas_parts(&mut state), 91).as_deref(),
        Some("OUT")
    );
}

#[test]
fn component_probe_never_fabricates_a_voltage_node_from_instance_identity() {
    let mut state = DesignTestHost::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(17, ComponentType::CellInstance, Point::origin())
                .with_name_value("XAMP", ""),
        );
    let symbols = schematic_symbol_context(&state);

    assert_eq!(
        component_probe_expression(&canvas_parts(&mut state), 17, Point::origin(), &symbols),
        None,
        "an unresolved terminal must fail closed instead of inventing V(XAMP)"
    );
}

#[test]
fn voltage_source_component_probe_preserves_device_current_semantics() {
    let mut state = DesignTestHost::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(23, ComponentType::VoltageSource, Point::origin())
                .with_name_value("VBIAS", "1.8"),
        );
    let symbols = schematic_symbol_context(&state);

    assert_eq!(
        component_probe_expression(&canvas_parts(&mut state), 23, Point::origin(), &symbols),
        Some(ComponentProbe::Current {
            drawn: "VBIAS".to_owned(),
            engine: "VBIAS".to_owned(),
        })
    );
}

#[test]
fn ordinary_component_body_probe_requests_device_current_not_nearest_pin_voltage() {
    let mut state = DesignTestHost::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(24, ComponentType::Resistor, Point::origin())
                .with_name_value("RLOAD", "1k"),
        );
    let symbols = schematic_symbol_context(&state);

    assert_eq!(
        component_probe_expression(&canvas_parts(&mut state), 24, Point::origin(), &symbols),
        Some(ComponentProbe::Current {
            drawn: "RLOAD".to_owned(),
            engine: "RLOAD".to_owned(),
        })
    );
}

/// The engine knows an element by its own name: a resistor drawn `C4` is
/// `RC4`, a VCVS with an output limit is the behavioural source `B1`, and a
/// memristor is filed under its keyword, `YMEMRISTOR!MR1`. A body click asks
/// the engine for that element.
#[test]
fn component_current_probe_asks_for_the_element_the_engine_knows() {
    let mut state = DesignTestHost::default();
    let mut limited =
        Component::new(26, ComponentType::Vcvs, Point::new(400, 0)).with_name_value("E1", "2");
    limited.params = "vmax=1".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .extend([
            Component::new(27, ComponentType::Resistor, Point::origin())
                .with_name_value("C4", "1k"),
            limited,
            Component::new(28, ComponentType::Memristor, Point::new(800, 0))
                .with_name_value("MR1", ""),
        ]);
    let symbols = schematic_symbol_context(&state);

    assert_eq!(
        component_probe_expression(&canvas_parts(&mut state), 27, Point::origin(), &symbols),
        Some(ComponentProbe::Current {
            drawn: "C4".to_owned(),
            engine: "RC4".to_owned(),
        })
    );
    assert_eq!(
        component_probe_expression(&canvas_parts(&mut state), 26, Point::new(400, 0), &symbols),
        Some(ComponentProbe::Current {
            drawn: "E1".to_owned(),
            engine: "B1".to_owned(),
        })
    );
    assert_eq!(
        component_probe_expression(&canvas_parts(&mut state), 28, Point::new(800, 0), &symbols),
        Some(ComponentProbe::Current {
            drawn: "MR1".to_owned(),
            engine: "YMEMRISTOR!MR1".to_owned(),
        })
    );
}

#[test]
fn exact_component_terminal_probe_requests_its_node_voltage() {
    let mut state = DesignTestHost::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(25, ComponentType::Resistor, Point::origin())
                .with_name_value("RLOAD", "1k"),
        );
    let symbols = schematic_symbol_context(&state);

    assert_eq!(
        component_probe_expression(&canvas_parts(&mut state), 25, Point::new(-20, 0), &symbols),
        Some(ComponentProbe::Voltage("net1".to_owned()))
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
        let mut state = DesignTestHost::default();
        let id = 100 + index as u64;
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(Component::new(id, kind, Point::origin()));
        let symbols = schematic_symbol_context(&state);

        assert_eq!(
            component_probe_expression(&canvas_parts(&mut state), id, Point::origin(), &symbols),
            None,
            "{kind:?} must not fabricate a single body-current observable"
        );
    }
}

#[test]
fn unmaterialized_probe_creates_one_plan_owned_output_idempotently() {
    let mut state = DesignTestHost::default();
    let before_revision = state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();

    assert!(matches!(
        request_probe_signal(
            &mut canvas_parts(&mut state),
            &OccurrenceProbeSpelling::verbatim("V(OUT)"),
            ProbePlot::Show
        ),
        ProbeSignalOutcome::SavedOutputCreated { .. }
    ));
    let after_first_revision = state
        .model
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
        request_probe_signal(
            &mut canvas_parts(&mut state),
            &OccurrenceProbeSpelling::verbatim("v(out)"),
            ProbePlot::Show
        ),
        ProbeSignalOutcome::SavedOutputAlreadyPresent { .. }
    ));
    assert_eq!(saved_outputs(&state).len(), 1);
    assert_eq!(
        state
            .model
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
    let mut state = DesignTestHost::default();
    let payloads_before = state
        .model
        .workspace
        .content
        .simulation_plan_payloads
        .clone();
    state.model.sim_setup.analysis_plan = None;

    let outcome = request_probe_signal(
        &mut canvas_parts(&mut state),
        &OccurrenceProbeSpelling::verbatim("V(OUT)"),
        ProbePlot::Show,
    );

    assert!(matches!(outcome, ProbeSignalOutcome::Rejected { .. }));
    assert_eq!(
        state.model.workspace.content.simulation_plan_payloads,
        payloads_before
    );
    assert!(state.model.sim_setup.analysis_plan.is_none());
}

/// A wire labelled OUT on an otherwise empty sheet, whose menu was opened at
/// `click`.
fn wire_menu_host(click: Point) -> DesignTestHost {
    use crate::workbench::app_state::ContextTarget;
    let mut state = DesignTestHost::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components.clear();
    document.wires.clear();
    document
        .wires
        .push(Wire::new(91, vec![Point::new(0, 20), Point::new(80, 20)]));
    document
        .net_labels
        .push(NetLabel::new(92, Point::new(70, 20), "OUT"));
    state.sync_active_schematic_to_workspace();
    state.model.schematic.init_undo_history();
    state.design.interaction.context_target = Some((ContextTarget::Wire(91), (click.x, click.y)));
    state
}

#[test]
fn the_probe_row_on_a_wire_probes_it_with_no_second_click() {
    use crate::schematic::view::context_menu::execute_context_action;
    use crate::workbench::app_state::ContextTarget;
    use rspice_schematic_editor::view::context_menu::ContextAction;
    // The menu opened a couple of units off the wire, where a right-click
    // still lands on it.
    let click = Point::new(40, 22);
    let mut state = wire_menu_host(click);
    let tool_before = state.model.schematic.session.editor.tool;
    let symbol_context = schematic_symbol_context(&state);

    execute_context_action(
        ContextAction::Probe,
        &mut canvas_parts(&mut state),
        click,
        &symbol_context,
    );

    let outputs = saved_outputs(&state);
    assert_eq!(outputs.len(), 1, "the wire's signal is asked of the run");
    assert_eq!(
        normalized_probe_expression(&outputs[0].source_expression),
        "v(out)"
    );
    let probes = &state.model.schematic.document().probes;
    assert_eq!(probes.len(), 1, "the probe's marker stays on the wire");
    assert_eq!(probes[0].position, Point::new(40, 20));
    assert_eq!(probes[0].reference, "V(OUT)");
    assert_eq!(
        state.model.schematic.session.editor.tool, tool_before,
        "nothing is left armed for a second click"
    );

    // On the bare canvas there is nothing to probe: the row arms the tool.
    state.design.interaction.context_target = Some((ContextTarget::Canvas, (200, 200)));
    execute_context_action(
        ContextAction::Probe,
        &mut canvas_parts(&mut state),
        Point::new(200, 200),
        &symbol_context,
    );
    assert_eq!(state.model.schematic.session.editor.tool, Tool::Probe);
    assert_eq!(state.model.schematic.document().probes.len(), 1);
}

/// A read-only view (a block descended read-only, a shipped library, safe
/// mode) still probes: the Probe tool's click is a probe rather than a
/// refused edit, it plots what the last run holds, and it writes neither a
/// marker into the drawing nor an output into the plan.
#[test]
fn a_read_only_view_probes_without_a_marker_or_a_plan_output() {
    let click = Point::new(40, 20);
    let mut state = wire_menu_host(click);
    state.model.schematic.session.read_only = true;
    hold_out_in_the_last_run(&mut state);
    let payloads_before = state
        .model
        .workspace
        .content
        .simulation_plan_payloads
        .clone();
    let symbol_context = schematic_symbol_context(&state);
    let viewport = pointer_viewport();

    let action = tool_pointer_view(&canvas_parts(&mut state)).action(
        Tool::Probe,
        &viewport,
        &symbol_context,
        viewport.schematic_to_screen(click),
    );
    let Some(ToolPointerAction::Probe(position)) = action else {
        panic!("a probe click on a read-only view is a probe, not a refused edit: {action:?}");
    };
    handle_probe_click(
        &mut canvas_parts(&mut state),
        position,
        &symbol_context,
        ProbePlot::Replace,
    );

    assert_eq!(
        plot_request(&state),
        Some(("V(OUT)".to_owned(), ProbePlot::Replace, "V(OUT)".to_owned()))
    );
    assert!(state.model.schematic.document().probes.is_empty());
    assert!(!state.model.schematic.can_undo());
    assert_eq!(
        state.model.workspace.content.simulation_plan_payloads,
        payloads_before
    );
}

/// The net inspector's Plot, pressed before a run, keeps no marker, so the
/// output it saves is the plan's and the next run saves it, where a
/// probe-owned output with no marker behind it would be dropped. An output a
/// probe once owned becomes the plan's the same way.
#[test]
fn the_net_inspector_plot_before_a_run_is_saved_by_the_next_run() {
    use crate::state::SavedOutputOrigin;
    let next_run_saves_out = |state: &DesignTestHost| {
        let plan_id = state
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let (effective, _, _) =
            rspice_simulation::output_contract::selection::effective_plan_capture(
                state.model.sim_setup.save_policy.output_selection_mode,
                saved_outputs(state),
                &[],
                &state.model.schematic.document().probes,
                &[],
                plan_id,
            )
            .expect("the plan resolves its outputs");
        effective
            .iter()
            .any(|output| normalized_probe_expression(&output.source_expression) == "v(out)")
    };

    let mut state = DesignTestHost::default();
    toggle_probe_with_feedback(&mut canvas_parts(&mut state), "OUT", "V(OUT)");
    assert_eq!(saved_outputs(&state).len(), 1);
    assert_eq!(saved_outputs(&state)[0].origin, SavedOutputOrigin::Plan);
    assert!(next_run_saves_out(&state));

    let mut state = DesignTestHost::default();
    request_probe_signal(
        &mut canvas_parts(&mut state),
        &OccurrenceProbeSpelling::verbatim("V(OUT)"),
        ProbePlot::Show,
    );
    assert_eq!(
        saved_outputs(&state)[0].origin,
        SavedOutputOrigin::SchematicProbe
    );
    assert!(!next_run_saves_out(&state), "no marker owns it");
    toggle_probe_with_feedback(&mut canvas_parts(&mut state), "OUT", "V(OUT)");
    assert_eq!(saved_outputs(&state).len(), 1);
    assert_eq!(saved_outputs(&state)[0].origin, SavedOutputOrigin::Plan);
    assert!(next_run_saves_out(&state));
}
