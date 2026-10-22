//! What a probe click asks for: the waveform it shows or hides, the plan
//! output it saves once, and the observable a wire, terminal or component
//! body names, which fails closed rather than inventing one.

use super::*;

#[test]
fn materialized_probe_toggles_immediately_and_preserves_future_save_intent() {
    let mut state = AppState::default();
    state
        .model
        .simulation
        .view
        .waveforms
        .push(RetainedTrace::new(
            "V(OUT)",
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            "#ffffff",
        ));

    assert_eq!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::WaveformHidden
    );
    assert!(!state.model.simulation.view.waveforms[0].visible);
    assert_eq!(saved_outputs(&state).len(), 1);

    assert_eq!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::WaveformShown
    );
    assert!(state.model.simulation.view.waveforms[0].visible);
    assert_eq!(saved_outputs(&state).len(), 1);
}

#[test]
fn ensure_visible_probe_action_never_hides_an_existing_trace() {
    let mut state = AppState::default();
    state
        .model
        .simulation
        .view
        .waveforms
        .push(RetainedTrace::new(
            "V(OUT)",
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            "#ffffff",
        ));

    assert_eq!(
        request_probe_signal_visible(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::WaveformAlreadyVisible
    );
    assert!(state.model.simulation.view.waveforms[0].visible);
    assert_eq!(saved_outputs(&state).len(), 1);

    state.model.simulation.view.waveforms[0].visible = false;
    assert_eq!(
        request_probe_signal_visible(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
        ProbeSignalOutcome::WaveformShown
    );
    assert!(state.model.simulation.view.waveforms[0].visible);
    assert_eq!(saved_outputs(&state).len(), 1);
}

#[test]
fn wire_probe_resolves_from_live_connectivity_without_retained_run_data() {
    let mut state = AppState::default();
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
    assert_eq!(live_wire_probe_net_name(&state, 91).as_deref(), Some("OUT"));
}

#[test]
fn component_probe_never_fabricates_a_voltage_node_from_instance_identity() {
    let mut state = AppState::default();
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
        component_probe_expression(&state, 17, Point::origin(), &symbols),
        None,
        "an unresolved terminal must fail closed instead of inventing V(XAMP)"
    );
}

#[test]
fn voltage_source_component_probe_preserves_device_current_semantics() {
    let mut state = AppState::default();
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
        component_probe_expression(&state, 23, Point::origin(), &symbols),
        Some(ComponentProbe::Current {
            drawn: "VBIAS".to_owned(),
            engine: "VBIAS".to_owned(),
        })
    );
}

#[test]
fn ordinary_component_body_probe_requests_device_current_not_nearest_pin_voltage() {
    let mut state = AppState::default();
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
        component_probe_expression(&state, 24, Point::origin(), &symbols),
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
    let mut state = AppState::default();
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
        component_probe_expression(&state, 27, Point::origin(), &symbols),
        Some(ComponentProbe::Current {
            drawn: "C4".to_owned(),
            engine: "RC4".to_owned(),
        })
    );
    assert_eq!(
        component_probe_expression(&state, 26, Point::new(400, 0), &symbols),
        Some(ComponentProbe::Current {
            drawn: "E1".to_owned(),
            engine: "B1".to_owned(),
        })
    );
    assert_eq!(
        component_probe_expression(&state, 28, Point::new(800, 0), &symbols),
        Some(ComponentProbe::Current {
            drawn: "MR1".to_owned(),
            engine: "YMEMRISTOR!MR1".to_owned(),
        })
    );
}

#[test]
fn exact_component_terminal_probe_requests_its_node_voltage() {
    let mut state = AppState::default();
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
        component_probe_expression(&state, 25, Point::new(-20, 0), &symbols),
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
        let mut state = AppState::default();
        let id = 100 + index as u64;
        state
            .model
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
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan")
        .revision();

    assert!(matches!(
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)")),
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
        request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("v(out)")),
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
    let mut state = AppState::default();
    let payloads_before = state
        .model
        .workspace
        .content
        .simulation_plan_payloads
        .clone();
    state.model.sim_setup.analysis_plan = None;

    let outcome = request_probe_signal(&mut state, &OccurrenceProbeSpelling::verbatim("V(OUT)"));

    assert!(matches!(outcome, ProbeSignalOutcome::Rejected { .. }));
    assert_eq!(
        state.model.workspace.content.simulation_plan_payloads,
        payloads_before
    );
    assert!(state.model.sim_setup.analysis_plan.is_none());
}
