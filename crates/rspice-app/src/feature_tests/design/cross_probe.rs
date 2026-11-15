//! Cross-probing from the schematic through the whole application: the end-frame step
//! projects the selection into the results' traces and the netlist's lines, which only the
//! whole application holds.

use crate::workbench::AppState;
use rspice_app_state::product::ContentDigest;
use rspice_app_state::product::{AnalysisInstanceId, ObjectRevision, SimulationPlanId};
use rspice_app_state::state::{
    AnalysisResult, AnalysisResultSourceDomain, AnalysisType, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, RetainedTrace, SimulationRun,
};
use rspice_app_state::workbench::preferences::TogglePreference;
use rspice_app_workbench::workbench::framework::feature::CodePort as _;

/// The end-frame step as the shell runs it: the projection, applied
/// through the results' and the netlist's own fns.
fn synchronize_schematic_cross_probe(state: &mut AppState) {
    if let Some(projection) =
        rspice_app_design::workbench::cross_probe::synchronize_schematic_cross_probe(
            &state.model,
            &mut state.design.schematic_cross_probe,
            state.code.shown_netlist_document_key(),
        )
    {
        state
            .results
            .results
            .project_cross_probe_trace(projection.released_trace, projection.trace);
        state
            .code
            .netlist
            .project_cross_probe_line(projection.released_line, projection.line);
    }
}

fn prepared_run(revision: ObjectRevision, waveform_name: &str) -> SimulationRun {
    prepared_run_in(revision, waveform_name, Vec::new())
}

fn prepared_run_in(
    revision: ObjectRevision,
    waveform_name: &str,
    hierarchy: Vec<rspice_app_state::state::HierarchyMapRow>,
) -> SimulationRun {
    let instance = AnalysisInstanceId::new();
    let receipt = PreparedRunReceipt::new(rspice_app_state::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision: revision,
        prepared_snapshot_digest: digest(1),
        source_content_digest: digest(2),
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(3)),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            PreparedRunTaskReceipt::new(instance, revision, Vec::new(), 5, digest(4))
                .expect("valid task"),
        ],
    })
    .expect("valid receipt")
    .with_hierarchy_map(hierarchy)
    .expect("distinct occurrences seal");
    let mut run = SimulationRun::new_prepared(1, receipt);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new(waveform_name, vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
        ]),
    );
    run
}

fn install_current_map(state: &mut AppState, net: &str) {
    let point = rspice_app_state::state::Point::new(0, 0);
    state.model.simulation.source.cross_probe.update(
        state.model.workspace.content.active_view.clone(),
        std::collections::HashMap::from([(point, net.to_owned())]),
        std::collections::HashMap::from([(net.to_owned(), vec![point])]),
        std::collections::HashMap::new(),
        state.model.schematic.topology_version(),
    );
}

#[test]
fn selected_net_highlights_only_a_current_exact_retained_trace() {
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(rspice_app_state::state::NetLabel::new(
            7,
            rspice_app_state::state::Point::new(0, 0),
            "OUT",
        ));
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_net_label(7);
    install_current_map(&mut state, "OUT");
    let revision = state.model.workspace.content.project.revision();
    state
        .model
        .simulation
        .retained
        .runs
        .push(prepared_run(revision, "V(OUT)"));
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);

    synchronize_schematic_cross_probe(&mut state);

    assert_eq!(
        state
            .results
            .results
            .session
            .selected_trace
            .as_ref()
            .map(|trace| trace.source_name()),
        Some("V(OUT)")
    );

    state.model.schematic.bump_topology_version();
    synchronize_schematic_cross_probe(&mut state);
    assert!(state.results.results.session.selected_trace.is_none());
}

#[test]
fn a_descended_tab_selects_the_trace_named_for_its_occurrence() {
    let mut state = AppState::default();
    let child = rspice_app_state::state::CellViewRef::new("user", "amp", "schematic");
    state.model.workspace.descend_into(
        "X1".to_owned(),
        child.clone(),
        rspice_app_state::state::ViewType::Schematic,
    );
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(rspice_app_state::state::NetLabel::new(
            7,
            rspice_app_state::state::Point::new(0, 0),
            "n1",
        ));
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_net_label(7);
    install_current_map(&mut state, "n1");
    let revision = state.model.workspace.content.project.revision();
    state.model.simulation.retained.runs.push(prepared_run_in(
        revision,
        "V(x1.n1)",
        vec![
            rspice_app_state::state::HierarchyMapRow::new("/X1", "amp_1", "X1", child)
                .expect("one occurrence row"),
        ],
    ));
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);

    synchronize_schematic_cross_probe(&mut state);

    assert_eq!(
        state
            .results
            .results
            .session
            .selected_trace
            .as_ref()
            .map(|trace| trace.source_name()),
        Some("V(x1.n1)"),
        "a net read inside /X1 must select the node the engine solved there"
    );
}

#[test]
fn disabled_preference_never_projects_the_schematic_selection() {
    let mut state = AppState::default();
    state
        .model
        .preferences
        .set_toggle(TogglePreference::CrossProbeBehavior, false);
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(rspice_app_state::state::NetLabel::new(
            7,
            rspice_app_state::state::Point::new(0, 0),
            "OUT",
        ));
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_net_label(7);
    install_current_map(&mut state, "OUT");
    let revision = state.model.workspace.content.project.revision();
    state
        .model
        .simulation
        .retained
        .runs
        .push(prepared_run(revision, "V(OUT)"));
    state.model.simulation.view.active_run_idx = Some(0);

    synchronize_schematic_cross_probe(&mut state);

    assert!(state.results.results.session.selected_trace.is_none());
    assert!(state.code.netlist.cross_probe_line.is_none());
}

#[test]
fn selected_component_uses_its_emitted_device_identity() {
    let mut state = AppState::default();
    let component_id = state.model.schematic.add_component(
        rspice_app_state::state::ComponentType::Resistor,
        rspice_app_state::state::Point::new(20, 20),
    );
    let emitted = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .map(rspice_app_state::state::Component::written_element_name)
        .expect("component exists");
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component_id);
    install_current_map(&mut state, "OUT");
    let revision = state.model.workspace.content.project.revision();
    let signal = format!("I({emitted})");
    state
        .model
        .simulation
        .retained
        .runs
        .push(prepared_run(revision, &signal));
    state.model.simulation.view.active_run_idx = Some(0);

    synchronize_schematic_cross_probe(&mut state);

    assert_eq!(
        state
            .results
            .results
            .session
            .selected_trace
            .as_ref()
            .map(|trace| trace.source_name()),
        Some(signal.as_str())
    );
}

/// The engine files a memristor under its card's keyword, so its trace is
/// `I(YMEMRISTOR!MR1)`; `I(MR1)` would name a MOSFET.
#[test]
fn a_selected_memristor_selects_the_trace_the_engine_names_it_by() {
    let mut state = AppState::default();
    let component_id = state.model.schematic.add_component(
        rspice_app_state::state::ComponentType::Memristor,
        rspice_app_state::state::Point::new(20, 20),
    );
    let written = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .map(rspice_app_state::state::Component::written_element_name)
        .expect("component exists");
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component_id);
    install_current_map(&mut state, "OUT");
    let revision = state.model.workspace.content.project.revision();
    let signal = format!("I(YMEMRISTOR!{written})");
    state
        .model
        .simulation
        .retained
        .runs
        .push(prepared_run(revision, &signal));
    state.model.simulation.view.active_run_idx = Some(0);

    synchronize_schematic_cross_probe(&mut state);

    assert_eq!(
        state
            .results
            .results
            .session
            .selected_trace
            .as_ref()
            .map(|trace| trace.source_name()),
        Some(signal.as_str())
    );
}

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

/// A run that named nothing: the Results page offers no highlight for it, and
/// marking its sites marks nothing on the drawing. The offer is the results
/// feature's and the marking the design feature's, so the case runs here.
#[test]
fn a_run_that_named_nothing_offers_no_highlight() {
    let mut state =
        rspice_app_design::workbench::app::actions::workspace::tests::state_with_probed_conductors(
        );

    assert!(
        !rspice_app_results::workbench::documents::result_document::operational_state::active_failure_names_objects(
            &state.model
        ),
        "with no failed analysis on display there is nothing to offer"
    );
    assert!(
        !rspice_app_design::workbench::app::actions::workspace::highlight_active_failure_sites(
            &mut state.model,
            &mut state.workbench
        ),
        "and nothing to mark"
    );
    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty()
    );
}
