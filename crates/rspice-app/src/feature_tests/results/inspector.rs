//! The result inspector's cross-probe into Design, and its route to the
//! run's details, observed through a whole application.

use crate::workbench::docks::inspector::results::cross_probe_trace_to_design;
use crate::workbench::docks::inspector::results::tests::{
    INSPECTOR_DEFAULT_WIDTH, result_app_with_current_out_map, run_inspector, waves_details_app,
};
use crate::workbench::docks::inspector::run_details;
use crate::workbench::state::Workspace;

#[test]
fn historical_result_revision_cannot_cross_probe_current_geometry() {
    let mut app = result_app_with_current_out_map(false);
    let stale_revision = app
        .state
        .model
        .workspace
        .content
        .project
        .revision()
        .next()
        .expect("next project revision");
    let run = app
        .state
        .model
        .simulation
        .active_run()
        .expect("prepared retained run");
    let analysis_id = run.analyses[0]
        .provenance()
        .expect("prepared analysis provenance")
        .source_instance_id();
    let receipt = crate::state::PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
        project_revision: stale_revision,
        prepared_snapshot_digest: crate::product::ContentDigest::from_bytes([0x51; 32]),
        source_content_digest: crate::product::ContentDigest::from_bytes([0x52; 32]),
        source_check_receipt: crate::state::PreparedSourceCheckReceipt::SchematicDrc(
            crate::product::ContentDigest::from_bytes([0x53; 32]),
        ),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            crate::state::PreparedRunTaskReceipt::new(
                analysis_id,
                stale_revision,
                Vec::new(),
                1,
                crate::product::ContentDigest::from_bytes([0x54; 32]),
            )
            .expect("valid prepared task"),
        ],
    })
    .expect("valid stale receipt");
    let mut stale_run = crate::state::SimulationRun::new_prepared(1, receipt);
    stale_run.add_analysis(
        crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "historical TRAN",
        )
        .with_provenance(
            crate::state::AnalysisResultProvenance::new_with_source_domain(
                crate::state::AnalysisResultSourceDomain::SimulationPlan,
                analysis_id,
                stale_revision,
                crate::product::ContentDigest::from_bytes([0x54; 32]),
                Vec::new(),
            )
            .expect("valid historical analysis provenance"),
        )
        .with_waveforms(vec![crate::state::RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            "#ffbd2e",
        )]),
    );
    app.state.model.simulation.retained.runs[0] = stale_run;
    app.state.results.results.session.selected_trace = Some(
        rspice_results_ui::selection::SelectedResultTrace::from_run_indices(
            &app.state.model.simulation.retained.runs[0],
            0,
            0,
        )
        .expect("selected historical trace"),
    );

    let error = cross_probe_trace_to_design(&mut app, "V(out)")
        .expect_err("historical result must fail closed");

    assert!(error.contains("different project revision"));
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty()
    );
}

/// The Results inspector names the run and leaves its provenance to Run
/// details…, which holds the identifiers and the executed deck the panel no
/// longer shows.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn run_provenance_lives_in_run_details_not_on_the_panel() {
    let mut app = waves_details_app();
    let (_, nodes) = run_inspector(&mut app, INSPECTOR_DEFAULT_WIDTH, 2400.0);
    let labelled = |nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)], label: &str| {
        nodes.iter().any(|(_, node)| node.label() == Some(label))
    };
    assert!(
        labelled(&nodes, "Run details\u{2026}"),
        "no route to the run's details"
    );
    for gone in [
        "Dataset ID",
        "Run sequence",
        "Execution target",
        "Samples",
        "Visibility",
    ] {
        assert!(!labelled(&nodes, gone), "{gone} is still on the panel");
    }

    let run_id = app
        .state
        .model
        .simulation
        .active_run()
        .expect("retained run")
        .id;
    run_details::open(
        &mut app.state.results.run_details,
        &app.state.model.simulation,
        run_id,
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut nodes = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(egui::RawInput::default(), |ui| {
            run_details::show(ui.ctx(), &mut app);
        });
        nodes = output
            .platform_output
            .accesskit_update
            .map(|update| update.nodes)
            .unwrap_or_default();
    }
    for present in ["Dataset ID", "Dataset digest", "Run ID"] {
        assert!(labelled(&nodes, present), "Run details lacks {present}");
    }

    run_details::open(
        &mut app.state.results.run_details,
        &app.state.model.simulation,
        u64::MAX,
    );
    assert_eq!(
        app.state
            .results
            .run_details
            .as_ref()
            .map(|details| details.run_id),
        Some(run_id),
        "a run no longer retained opens nothing"
    );
}
