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
        domain: Default::default(),
    }
}

/// The design root and the nets its sheet owns, which is the shape automatic
/// selection reads when the design is one level deep.
fn root_occurrence(nets: Vec<DesignNet>) -> Vec<OccurrenceNets> {
    vec![OccurrenceNets {
        occurrence: InstancePath::root(),
        scope: String::new(),
        nets: std::sync::Arc::new(nets),
    }]
}

/// One occurrence below the root, owning the nets of the master it instantiates.
fn instance_occurrence(path: &str, nets: Vec<DesignNet>) -> OccurrenceNets {
    let occurrence = InstancePath::parse(path).expect("a legal occurrence path");
    OccurrenceNets {
        scope: occurrence.to_engine_name().unwrap_or_default(),
        occurrence,
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
            .all(|output| { output.save_policy == SavedOutputPolicy::EveryAcceptedPoint }),
        "each keeps every point its analysis accepts"
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
    assert_eq!(
        first
            .iter()
            .map(|output| output.streaming)
            .collect::<Vec<_>>(),
        vec![
            SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
            SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
            SavedOutputStreaming::StoreOnly,
        ],
        "what the plan plots streams while it runs; a data-browser net does not"
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

/// A first run plots something: the output ports when the design has any,
/// else the nets the drawing names, else, on a small design, every node.
#[test]
fn automatic_outputs_plot_ports_else_named_nets_else_every_node() {
    let plan_id = SimulationPlanId::new();
    let plotted = |nets: Vec<DesignNet>| {
        let (outputs, _) = effective_plan_saved_outputs(
            OutputSelectionMode::Automatic,
            &[],
            &[],
            &root_occurrence(nets),
            plan_id,
        )
        .expect("automatic selection");
        outputs
            .into_iter()
            .filter(|output| output.display_intent == SavedOutputDisplayIntent::Plot)
            .map(|output| output.source_expression)
            .collect::<Vec<_>>()
    };
    let ground = || output_test_net("0", true, NetClass::Ground, None);

    assert_eq!(
        plotted(vec![
            ground(),
            output_test_net("out", true, NetClass::Signal, None),
            output_test_net("net1", false, NetClass::Signal, None),
        ]),
        vec!["V(out)"],
        "with no port, the named net"
    );
    assert_eq!(
        plotted(vec![
            ground(),
            output_test_net("net1", false, NetClass::Signal, None),
            output_test_net("net2", false, NetClass::Signal, None),
        ]),
        vec!["V(net1)", "V(net2)"],
        "with nothing named, every node of a small design"
    );
}

/// A probe joins the bounded automatic set rather than replacing it, so a
/// net probed after a run does not take the other nets out of the next one.
/// A net asked for both ways is saved once, as the probe asked for it; an
/// explicit-only plan still saves just what it asks for.
#[test]
fn a_probe_joins_the_automatic_set_rather_than_replacing_it() {
    let plan_id = SimulationPlanId::new();
    let occurrences = root_occurrence(vec![
        output_test_net("out", true, NetClass::Signal, None),
        output_test_net("mid", true, NetClass::Signal, None),
    ]);
    let probe = SchematicProbe::new(
        1,
        rspice_design_model::primitives::Point::origin(),
        "V(out)",
        Some("V(out)".to_owned()),
    )
    .expect("probe marker");

    let (outputs, used_automatic) = effective_plan_saved_outputs(
        OutputSelectionMode::Automatic,
        &[],
        std::slice::from_ref(&probe),
        &occurrences,
        plan_id,
    )
    .expect("automatic selection");
    assert!(used_automatic);
    assert_eq!(
        outputs
            .iter()
            .map(|output| (output.source_expression.as_str(), output.origin))
            .collect::<Vec<_>>(),
        vec![
            ("V(out)", SavedOutputOrigin::SchematicProbe),
            ("V(mid)", SavedOutputOrigin::Automatic),
        ]
    );

    let (outputs, used_automatic) = effective_plan_saved_outputs(
        OutputSelectionMode::ExplicitOnly,
        &[],
        std::slice::from_ref(&probe),
        &occurrences,
        plan_id,
    )
    .expect("explicit selection");
    assert!(!used_automatic);
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].source_expression, "V(out)");
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

/// An occurrence is found by the name the engine knows it by, however the
/// design spells it, and the occurrences inside one are the run directly
/// below it: not itself, not theirs, and not a sibling's whose name begins
/// with its own.
#[test]
fn occurrences_are_found_by_engine_name_and_list_the_instances_inside() {
    let mut occurrences = root_occurrence(Vec::new());
    for path in [
        "/X10/X2",
        "/amp",
        "/X1/X3",
        "/X1",
        "/X1/X2/X4",
        "/X10",
        "/X1/X2",
    ] {
        occurrences.push(instance_occurrence(path, Vec::new()));
    }
    occurrences[1..].sort_by(|left, right| left.order().cmp(&right.order()));
    let at = |text: &str| {
        occurrence_at(&occurrences, &InstancePath::parse(text).expect("a path"))
            .map(|at| occurrences[at].occurrence().to_string())
    };
    assert_eq!(at("/").as_deref(), Some("/"));
    assert_eq!(at("/x10").as_deref(), Some("/X10"));
    assert_eq!(at("xamp").as_deref(), Some("/amp"));
    assert_eq!(at("x1.x2.x4").as_deref(), Some("/X1/X2/X4"));
    assert_eq!(at("/X2"), None);
    let inside = |text: &str| {
        let index = occurrence_at(&occurrences, &InstancePath::parse(text).expect("a path"))
            .expect("the occurrence is listed");
        occurrences_inside(&occurrences, index)
            .iter()
            .map(|inside| inside.occurrence().to_string())
            .collect::<Vec<_>>()
    };
    assert_eq!(inside("/"), ["/X1", "/X10", "/amp"]);
    assert_eq!(inside("/X1"), ["/X1/X2", "/X1/X3"]);
    assert_eq!(inside("/X10"), ["/X10/X2"]);
    assert_eq!(inside("/X1/X2"), ["/X1/X2/X4"]);
    assert!(inside("/X1/X3").is_empty());
}
