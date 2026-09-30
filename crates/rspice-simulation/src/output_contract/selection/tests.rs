use super::*;
use rspice_design::connectivity::summary::NetClass;

fn output_test_net(
    name: &str,
    authored_name: bool,
    class: NetClass,
    port: Option<PortDirection>,
) -> DesignNet {
    DesignNet {
        name: name.to_owned(),
        authored_name,
        class,
        terminals: Vec::new(),
        port,
        wire_ids: Vec::new(),
    }
}

/// The design root and the nets its sheet owns, which is the shape automatic
/// selection reads when the design is one level deep.
fn root_occurrence(nets: Vec<DesignNet>) -> Vec<OccurrenceNets> {
    vec![OccurrenceNets {
        occurrence: InstancePath::root(),
        nets: std::sync::Arc::new(nets),
    }]
}

/// One occurrence below the root, owning the nets of the master it instantiates.
fn instance_occurrence(path: &str, nets: Vec<DesignNet>) -> OccurrenceNets {
    OccurrenceNets {
        occurrence: InstancePath::parse(path).expect("a legal occurrence path"),
        nets: std::sync::Arc::new(nets),
    }
}

#[test]
fn automatic_output_selection_is_bounded_prioritized_and_deterministic() {
    let plan_id = SimulationPlanId::new();
    let mut nets = vec![
        output_test_net("0", true, NetClass::Ground, Some(PortDirection::Supply)),
        output_test_net("z_out", true, NetClass::Signal, Some(PortDirection::Out)),
        output_test_net("a_io", true, NetClass::Signal, Some(PortDirection::InOut)),
        output_test_net("mid", true, NetClass::Signal, None),
    ];
    nets.extend(
        (0..20).map(|index| output_test_net(&format!("net{index}"), false, NetClass::Signal, None)),
    );

    let occurrences = root_occurrence(nets);
    let (first, used_fallback) = effective_plan_saved_outputs(
        OutputSelectionMode::Automatic,
        &[],
        &[],
        &occurrences,
        plan_id,
    )
    .expect("automatic selection");
    let (second, _) = effective_plan_saved_outputs(
        OutputSelectionMode::Automatic,
        &[],
        &[],
        &occurrences,
        plan_id,
    )
    .expect("repeat automatic selection");

    assert!(used_fallback);
    assert_eq!(
        first
            .iter()
            .map(|output| output.source_expression.as_str())
            .collect::<Vec<_>>(),
        vec!["V(z_out)", "V(a_io)", "V(mid)"]
    );
    assert_eq!(
        first.iter().map(|output| output.id).collect::<Vec<_>>(),
        second.iter().map(|output| output.id).collect::<Vec<_>>()
    );
    assert!(
        first
            .iter()
            .all(|output| { output.save_policy == SavedOutputPolicy::SelectedAndFinalPoints })
    );
    assert_eq!(
        first
            .iter()
            .map(|output| output.display_intent)
            .collect::<Vec<_>>(),
        vec![
            SavedOutputDisplayIntent::Plot,
            SavedOutputDisplayIntent::Plot,
            SavedOutputDisplayIntent::DataBrowserOnly,
        ]
    );
}

#[test]
fn explicit_output_modes_do_not_invent_quantities() {
    let plan_id = SimulationPlanId::new();
    let occurrences = root_occurrence(vec![output_test_net(
        "out",
        true,
        NetClass::Signal,
        Some(PortDirection::Out),
    )]);
    for mode in [
        OutputSelectionMode::ExplicitOnly,
        OutputSelectionMode::SaveAll,
    ] {
        let (outputs, used_fallback) =
            effective_plan_saved_outputs(mode, &[], &[], &occurrences, plan_id).expect("selection");
        assert!(outputs.is_empty());
        assert!(!used_fallback);
    }
}

#[test]
fn automatic_outputs_cover_every_occurrence_exactly_once() {
    let plan_id = SimulationPlanId::new();
    let master_net = || vec![output_test_net("n1", true, NetClass::Signal, None)];
    let mut occurrences = root_occurrence(vec![output_test_net(
        "out",
        true,
        NetClass::Signal,
        Some(PortDirection::Out),
    )]);
    occurrences.push(instance_occurrence("/X1", master_net()));
    occurrences.push(instance_occurrence("/X2", master_net()));

    let (outputs, used_fallback) = effective_plan_saved_outputs(
        OutputSelectionMode::Automatic,
        &[],
        &[],
        &occurrences,
        plan_id,
    )
    .expect("automatic selection");

    assert!(used_fallback);
    assert_eq!(
        outputs
            .iter()
            .map(|output| (output.name.as_str(), output.source_expression.as_str()))
            .collect::<Vec<_>>(),
        vec![
            ("V(out)", "V(out)"),
            ("V(/X1/n1)", "V(x1.n1)"),
            ("V(/X2/n1)", "V(x2.n1)"),
        ],
        "each occurrence contributes its own node once, and the root keeps today's spelling"
    );
    assert_eq!(
        outputs
            .iter()
            .map(|output| output.id)
            .collect::<std::collections::HashSet<_>>()
            .len(),
        3,
        "two instances of one master must not collapse onto one saved-output identity"
    );
}

#[test]
fn an_occurrence_the_engine_cannot_name_contributes_no_automatic_output() {
    let plan_id = SimulationPlanId::new();
    let mut occurrences = root_occurrence(Vec::new());
    occurrences.push(instance_occurrence(
        "/Xé",
        vec![output_test_net("n1", true, NetClass::Signal, None)],
    ));

    let (outputs, _) = effective_plan_saved_outputs(
        OutputSelectionMode::Automatic,
        &[],
        &[],
        &occurrences,
        plan_id,
    )
    .expect("automatic selection");

    assert!(
        outputs.is_empty(),
        "the engine has no name for this node, so there is nothing to ask it for"
    );
}

#[test]
fn probe_owned_output_is_effective_only_while_enabled_marker_references_it() {
    let plan_id = SimulationPlanId::new();
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "V(out)",
        "V(out)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::SelectedAndFinalPoints,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
    )
    .expect("probe output")
    .with_origin(SavedOutputOrigin::SchematicProbe);
    let mut probe = SchematicProbe::new(
        1,
        rspice_design_model::primitives::Point::origin(),
        "V(out)",
        Some("V(out)".to_owned()),
    )
    .expect("probe marker");
    probe.bind_saved_output(plan_id, output.id);

    let (linked, _) = effective_plan_saved_outputs(
        OutputSelectionMode::ExplicitOnly,
        std::slice::from_ref(&output),
        std::slice::from_ref(&probe),
        &[],
        plan_id,
    )
    .expect("linked selection");
    assert_eq!(linked.len(), 1);
    assert_eq!(linked[0].display_intent, SavedOutputDisplayIntent::Plot);

    probe.plot_on_materialization = false;
    let (browser_only, _) = effective_plan_saved_outputs(
        OutputSelectionMode::ExplicitOnly,
        std::slice::from_ref(&output),
        std::slice::from_ref(&probe),
        &[],
        plan_id,
    )
    .expect("browser-only selection");
    assert_eq!(
        browser_only[0].display_intent,
        SavedOutputDisplayIntent::DataBrowserOnly
    );

    let mut rebound_row = output.clone();
    rebound_row.name = "V(other)".to_owned();
    rebound_row.source_expression = "V(other)".to_owned();
    let (stale_binding, _) = effective_plan_saved_outputs(
        OutputSelectionMode::ExplicitOnly,
        std::slice::from_ref(&rebound_row),
        std::slice::from_ref(&probe),
        &[],
        plan_id,
    )
    .expect("stale binding replacement");
    assert_eq!(stale_binding.len(), 1);
    assert_eq!(stale_binding[0].source_expression, "V(out)");
    assert_ne!(stale_binding[0].id, rebound_row.id);

    probe.enabled = false;
    let (disabled, _) = effective_plan_saved_outputs(
        OutputSelectionMode::ExplicitOnly,
        std::slice::from_ref(&output),
        std::slice::from_ref(&probe),
        &[],
        plan_id,
    )
    .expect("disabled selection");
    assert!(disabled.is_empty());

    probe.enabled = true;
    probe.bind_saved_output(SimulationPlanId::new(), output.id);
    let (cross_plan, _) = effective_plan_saved_outputs(
        OutputSelectionMode::ExplicitOnly,
        std::slice::from_ref(&output),
        std::slice::from_ref(&probe),
        &[],
        plan_id,
    )
    .expect("cross-plan probe selection");
    assert_eq!(cross_plan.len(), 1);
    assert_ne!(cross_plan[0].id, output.id);
    assert_eq!(cross_plan[0].source_expression, "V(out)");
    assert_eq!(cross_plan[0].origin, SavedOutputOrigin::SchematicProbe);

    let (deleted, _) = effective_plan_saved_outputs(
        OutputSelectionMode::ExplicitOnly,
        &[output],
        &[],
        &[],
        plan_id,
    )
    .expect("deleted marker selection");
    assert!(deleted.is_empty());
}
