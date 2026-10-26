//! Hardcopy source resolution, in the cases that print a result: the
//! dataset, viewer and pane the Results workspace shows.

mod app_resolution;
mod quick_view;

use crate::product::{ContentDigest, DatasetBinding, DatasetId, ObjectRevision};
use crate::results::visualization_document::{
    AxisScale, ColumnRole, DocumentEdit, EntityRef, SourceColumn, SourceDataset, SourceRow,
    TypedValue, ValueType, VisualizationDocument,
};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, MonteCarloVariableMetadata,
    PeriodicNoiseOutputQuantity, Point, RetainedTrace, SimulationRun, SimulationRunLifecycle, Wire,
};
use crate::workbench::AppState;
use crate::workbench::documents::result_document::ResultViewer;
use crate::workbench::hardcopy_adapters::sources::{
    RetainedHardcopySourceAvailability, SourceReads, active_app_hardcopy_source_available,
    active_visualization_document_pane, enumerate_retained_hardcopy_sources,
    prepare_retained_hardcopy_resolution, quick_result_analysis_index, quick_result_availability,
    resolve_retained_hardcopy_source,
};
use crate::workbench::state::{Workspace, WorkspaceDocumentId};
use rspice_hardcopy::sources::{
    HardcopySemanticDocument, PLOT_INSET_UM, PLOT_WIDTH_UM, PreparedRetainedHardcopyResolution,
    ResolvedHardcopyDocument, SemanticAxisKind,
};
use rspice_hardcopy_contract::sources::HardcopySourceError;
use rspice_hardcopy_contract::{HardcopyDocumentId, HardcopyScope, PrintObjectKind};
use rspice_results::renderability::ordinary_noise_spectrum_is_renderable;

fn phase_noise_metadata() -> AnalysisResultFamilyMetadata {
    AnalysisResultFamilyMetadata::PeriodicNoise {
        output_quantity: PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
        carrier_frequency_hz: Some(2.4e9),
    }
}

fn quick_view_state(analysis: AnalysisResult, viewer: ResultViewer) -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(analysis);
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.results.results.session.viewer = viewer;
    state
}

/// The active run printed as File > Print takes it from the Results
/// workspace: its dataset open as the Results document, resolved through the
/// retained path the dialog's worker runs.
fn resolve_quick_view(state: &AppState) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let dataset_id = state
        .model
        .simulation
        .active_run()
        .ok_or_else(|| {
            HardcopySourceError::UnretainedResult("no active result dataset is selected".to_owned())
        })?
        .dataset_id;
    let mut shown = state.clone();
    shown.workbench.workspace = Workspace::Results;
    shown
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset_id));
    let source_key = format!(
        "project:{}:result-dataset:{dataset_id}",
        shown.model.workspace.content.project.id().as_uuid()
    );
    resolve_retained_hardcopy_source(&shown, &source_key, HardcopyScope::ActivePlotDocument)
}

#[test]
fn bode_hardcopy_exports_displayed_decibels_and_phase_for_periodic_response() {
    let analysis = AnalysisResult::new(1, AnalysisType::Pac, "PAC").with_waveforms(vec![
        RetainedTrace::new("|V(out)|", vec![1.0, 10.0], vec![10.0, 1.0], "#00aaff"),
        RetainedTrace::new(
            "phase(V(out))",
            vec![1.0, 10.0],
            vec![-90.0, -135.0],
            "#ffbd2e",
        ),
    ]);
    let state = quick_view_state(analysis, ResultViewer::Bode);

    let resolved = resolve_quick_view(&state).expect("PAC Bode semantic hardcopy");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic Bode plot")
    };
    assert_eq!(plot.viewer, ResultViewer::Bode);
    assert_eq!(plot.traces.len(), 2);
    assert_eq!(
        plot.traces[0]
            .source_samples
            .iter()
            .map(|(_, value)| f64::from_bits(*value))
            .collect::<Vec<_>>(),
        vec![20.0, 0.0]
    );
    assert_eq!(
        plot.traces[1]
            .source_samples
            .iter()
            .map(|(_, value)| f64::from_bits(*value))
            .collect::<Vec<_>>(),
        vec![-90.0, -135.0]
    );
}

#[test]
fn disto_hardcopy_preserves_retained_engineering_values() {
    let analysis = AnalysisResult::new(1, AnalysisType::Disto, "DISTO").with_waveforms(vec![
        RetainedTrace::new(
            "V(out) HD3(dBc)",
            vec![1.0e3, 1.0e4],
            vec![-80.0, -60.0],
            "#00aaff",
        )
        .with_unit("dBc"),
    ]);
    let state = quick_view_state(analysis, ResultViewer::Bode);

    let resolved = resolve_quick_view(&state).expect("DISTO semantic hardcopy");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic distortion plot")
    };
    assert_eq!(
        plot.traces[0]
            .source_samples
            .iter()
            .map(|(_, value)| f64::from_bits(*value))
            .collect::<Vec<_>>(),
        vec![-80.0, -60.0]
    );
}

#[test]
fn harmonic_balance_quick_view_exports_only_retained_complex_coefficients() {
    let spectrum = RetainedTrace::new(
        "|V(out)|",
        vec![0.0, 1.0e6, 2.0e6],
        vec![0.1, 1.0, 0.2],
        "#ffbd2e",
    )
    .with_complex_components("V(out)", vec![0.1, 1.0, 0.2], vec![0.0, 0.0, 0.0]);
    let state = quick_view_state(
        AnalysisResult::new(1, AnalysisType::HarmonicBalance, "HB").with_waveforms(vec![spectrum]),
        ResultViewer::HarmonicBalance,
    );

    let resolved = resolve_quick_view(&state).expect("HB semantic hardcopy");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic HB plot")
    };
    assert_eq!(plot.viewer, ResultViewer::HarmonicBalance);
    assert_eq!(plot.traces.len(), 1);
    assert_eq!(plot.traces[0].label, "V(out)");
}

#[test]
fn phase_noise_quick_view_never_exports_ordinary_periodic_noise_as_l_of_f() {
    let state = quick_view_state(
        AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE")
            .with_waveforms(vec![
                RetainedTrace::new(
                    "onoise",
                    vec![1.0, 1.0e3, 1.0e6],
                    vec![1.0e-18, 1.0e-19, 1.0e-20],
                    "#4ec9b0",
                ),
                RetainedTrace::new(
                    "phase_noise",
                    vec![1.0, 1.0e3, 1.0e6],
                    vec![-72.0, -103.0, -132.0],
                    "#ffbd2e",
                ),
            ])
            .with_family_metadata(phase_noise_metadata()),
        ResultViewer::PhaseNoise,
    );

    let resolved = resolve_quick_view(&state).expect("phase-noise semantic hardcopy");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic phase-noise plot")
    };
    assert_eq!(plot.viewer, ResultViewer::PhaseNoise);
    assert_eq!(plot.traces.len(), 1);
    assert!(plot.traces[0].label.contains("phase_noise"));
    assert!(plot.traces[0].label.contains("dBc/Hz"));
}

#[test]
fn waves_quick_view_selects_retained_transient_when_another_analysis_is_active() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::DcSweep, "DC").with_waveforms(vec![
            RetainedTrace::new("V(dc)", vec![0.0, 1.0], vec![0.0, 2.0], "#ffbd2e"),
        ]),
    );
    run.analyses.push(
        AnalysisResult::new(2, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(time)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.results.results.session.viewer = ResultViewer::Waves;

    let resolved = resolve_quick_view(&state).expect("transient Waves semantic hardcopy");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic Waves plot")
    };
    assert_eq!(plot.viewer, ResultViewer::Waves);
    assert_eq!(plot.traces.len(), 1);
    assert_eq!(plot.traces[0].label, "V(time)");
}

#[test]
fn production_figure_preparation_uses_the_open_dataset_not_the_global_run_selector() {
    let mut state = AppState::default();
    let mut displayed = SimulationRun::new(1);
    displayed.lifecycle = SimulationRunLifecycle::Completed;
    displayed.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Displayed").with_waveforms(vec![
            RetainedTrace::new("V(displayed)", vec![0.0, 1.0], vec![0.25, 0.75], "#00aaff"),
        ]),
    );
    let displayed_dataset = displayed.dataset_id;

    let mut background = SimulationRun::new(2);
    background.lifecycle = SimulationRunLifecycle::Completed;
    background.analyses.push(
        AnalysisResult::new(2, AnalysisType::Transient, "Background").with_waveforms(vec![
            RetainedTrace::new(
                "V(background)",
                vec![0.0, 1.0],
                vec![100.0, 200.0],
                "#ff00aa",
            ),
        ]),
    );

    state.model.simulation.retained.runs = vec![displayed, background].into();
    state.model.simulation.view.active_run_idx = Some(1);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.results.results.session.viewer = ResultViewer::Waves;
    state.workbench.workspace = Workspace::Results;
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(displayed_dataset));

    let source_key = format!(
        "project:{}:result-dataset:{}",
        state.model.workspace.content.project.id().as_uuid(),
        displayed_dataset
    );
    let descriptors = enumerate_retained_hardcopy_sources(SourceReads::of(&state));
    assert!(descriptors.iter().any(|descriptor| {
        descriptor.source_key == source_key && descriptor.availability.is_available()
    }));

    let resolved = prepare_retained_hardcopy_resolution(
        SourceReads::of(&state),
        &source_key,
        HardcopyScope::ActivePlotDocument,
    )
    .expect("displayed result prepares")
    .resolve_owned()
    .expect("displayed result resolves");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected displayed semantic plot")
    };
    assert_eq!(plot.traces.len(), 1);
    assert_eq!(plot.traces[0].label, "V(displayed)");
    assert_eq!(
        plot.traces[0].source_samples,
        vec![
            (0.0f64.to_bits(), 0.25f64.to_bits()),
            (1.0f64.to_bits(), 0.75f64.to_bits()),
        ]
    );
}

#[test]
fn production_manifest_hardcopy_retains_the_complete_displayed_dataset_through_worker_handoff() {
    let mut state = AppState::default();
    let mut displayed = SimulationRun::new(1);
    displayed.lifecycle = SimulationRunLifecycle::Completed;
    displayed
        .analyses
        .push(AnalysisResult::new(1, AnalysisType::Transient, "Transient"));
    displayed
        .analyses
        .push(AnalysisResult::new(2, AnalysisType::Ac, "AC"));
    displayed
        .restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("terminal fixture has explicit legacy provenance");
    let displayed_dataset = displayed.dataset_id;

    let mut background = SimulationRun::new(2);
    background.lifecycle = SimulationRunLifecycle::Completed;
    background
        .analyses
        .push(AnalysisResult::new(3, AnalysisType::DcOp, "Background OP"));
    background
        .restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("background fixture has explicit legacy provenance");
    state.model.simulation.retained.runs = vec![displayed, background].into();
    state.model.simulation.view.active_run_idx = Some(1);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.results.results.session.viewer = ResultViewer::Manifest;
    state.workbench.workspace = Workspace::Results;
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(displayed_dataset));

    let source_key = format!(
        "project:{}:result-dataset:{}",
        state.model.workspace.content.project.id().as_uuid(),
        displayed_dataset
    );
    let prepared = prepare_retained_hardcopy_resolution(
        SourceReads::of(&state),
        &source_key,
        HardcopyScope::ActivePlotDocument,
    )
    .expect("complete displayed manifest prepares");
    let worker_bytes = prepared
        .into_worker_snapshot_json()
        .expect("manifest worker snapshot serializes");
    let restored = PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&worker_bytes)
        .expect("manifest worker snapshot restores");
    let resolved = restored.resolve_owned().expect("manifest resolves");
    let HardcopySemanticDocument::ResultSummary(summary) = resolved.semantic_document() else {
        panic!("expected semantic manifest summary")
    };
    assert_eq!(summary.viewer, ResultViewer::Manifest);
    assert_eq!(summary.tables[0].rows.len(), 2);
    assert_eq!(summary.tables[0].rows[0][0], "Transient");
    assert_eq!(summary.tables[0].rows[1][0], "AC Analysis");
}

#[test]
fn production_stacked_results_hardcopy_retains_every_displayed_analysis() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "First").with_waveforms(vec![
            RetainedTrace::new("V(first)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    run.analyses.push(
        AnalysisResult::new(2, AnalysisType::Transient, "Second").with_waveforms(vec![
            RetainedTrace::new("V(second)", vec![0.0, 2.0], vec![2.0, 3.0], "#ff00aa"),
        ]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("terminal fixture has explicit legacy provenance");
    let dataset_id = run.dataset_id;
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.results.results.session.viewer = ResultViewer::Waves;
    state.workbench.workspace = Workspace::Results;
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset_id));
    let source_key = format!(
        "project:{}:result-dataset:{}",
        state.model.workspace.content.project.id().as_uuid(),
        dataset_id
    );

    let worker_bytes = prepare_retained_hardcopy_resolution(
        SourceReads::of(&state),
        &source_key,
        HardcopyScope::ActivePlotDocument,
    )
    .expect("stacked Results prepares")
    .into_worker_snapshot_json()
    .expect("stacked Results serializes");
    let encoded: serde_json::Value = serde_json::from_slice(&worker_bytes).unwrap();
    assert_eq!(
        encoded["payload"]["simulation_results"],
        serde_json::to_value(crate::io::capture_simulation_results(
            &state.model.simulation
        ))
        .unwrap(),
        "portable capture preserves the complete result-owner wire value",
    );
    let resolved = PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&worker_bytes)
        .expect("stacked Results restores")
        .resolve_owned()
        .expect("stacked Results resolves");
    assert_eq!(
        resolved.authority().scope(),
        &HardcopyScope::ActivePlotDocument
    );
    let HardcopySemanticDocument::Aggregate(aggregate) = resolved.semantic_document() else {
        panic!("expected stacked semantic aggregate")
    };
    assert_eq!(aggregate.children.len(), 2);
    let labels = aggregate
        .children
        .iter()
        .map(|child| match child.document.as_ref() {
            HardcopySemanticDocument::Plot(plot) => plot.traces[0].label.as_str(),
            _ => panic!("stack child is not a plot"),
        })
        .collect::<Vec<_>>();
    assert_eq!(labels, ["V(first)", "V(second)"]);
}

#[test]
fn production_specs_hardcopy_judges_the_complete_dataset() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_measurements(vec![rspice_core::MeasureResult::success("delay", 2.0e-9)]),
    );
    run.analyses.push(
        AnalysisResult::new(2, AnalysisType::Ac, "AC")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.5)]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("terminal fixture has explicit legacy provenance");
    let dataset_id = run.dataset_id;
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.model.workspace.content.specs = vec![
        crate::state::SpecEntry {
            measurement: "gain".to_owned(),
            expression: "max gain".to_owned(),
            min: Some(1.0),
            max: Some(2.0),
            unit: "V/V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
        crate::state::SpecEntry {
            measurement: "delay".to_owned(),
            expression: "propagation delay".to_owned(),
            min: None,
            max: Some(3.0e-9),
            unit: "s".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
    ];
    state.results.results.session.viewer = ResultViewer::Specs;
    state.workbench.workspace = Workspace::Results;
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset_id));
    let source_key = format!(
        "project:{}:result-dataset:{}",
        state.model.workspace.content.project.id().as_uuid(),
        dataset_id
    );

    let worker_bytes = prepare_retained_hardcopy_resolution(
        SourceReads::of(&state),
        &source_key,
        HardcopyScope::ActivePlotDocument,
    )
    .expect("Specs prepares")
    .into_worker_snapshot_json()
    .expect("Specs serializes");
    let resolved = PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&worker_bytes)
        .expect("Specs restores")
        .resolve_owned()
        .expect("Specs resolves");
    let HardcopySemanticDocument::ResultSummary(summary) = resolved.semantic_document() else {
        panic!("expected Specs result summary")
    };
    assert_eq!(summary.viewer, ResultViewer::Specs);
    assert_eq!(summary.tables[0].rows.len(), 2);
    assert_eq!(summary.tables[0].rows[0][0], "gain");
    assert_eq!(summary.tables[0].rows[0][8], "pass");
    assert_eq!(summary.tables[0].rows[1][0], "delay");
    assert_eq!(summary.tables[0].rows[1][8], "pass");
}

/// A workspace on the Specs sheet of one completed run whose receipt froze
/// `gain >= 10 dB`, with one attributed analysis that measured 12 dB.
fn signed_off_specification_workspace() -> (AppState, String) {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResultProvenance, AnalysisResultSourceDomain, PreparedRunReceipt,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt, PreparedSpecification, SpecEntry,
        SpecPointScope,
    };

    let task_id = AnalysisInstanceId::new();
    let snapshot = ContentDigest::from_bytes([0x71; 32]);
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision: ObjectRevision::INITIAL,
        prepared_snapshot_digest: snapshot,
        source_content_digest: ContentDigest::from_bytes([0x73; 32]),
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes(
            [0x74; 32],
        )),
        project_model_sources: Vec::new(),
        specifications: vec![
            PreparedSpecification::new(SpecEntry {
                measurement: "gain".to_owned(),
                expression: "max gain".to_owned(),
                min: Some(10.0),
                max: None,
                unit: "dB".to_owned(),
                scope: SpecPointScope::AllPoints,
            })
            .expect("prepared requirement"),
        ],
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            PreparedRunTaskReceipt::new(
                task_id,
                ObjectRevision::INITIAL,
                Vec::new(),
                2,
                ContentDigest::from_bytes([0x72; 32]),
            )
            .expect("task receipt"),
        ],
    })
    .expect("prepared receipt");
    let mut run = SimulationRun::new_prepared(1, receipt);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 12.0)])
            .with_provenance(
                AnalysisResultProvenance::new(
                    task_id,
                    ObjectRevision::INITIAL,
                    snapshot,
                    Vec::new(),
                )
                .expect("prepared provenance"),
            ),
    );
    run.mark_running().expect("the engine accepted the run");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("the controller seals the run");

    let dataset_id = run.dataset_id;
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.results.results.session.viewer = ResultViewer::Specs;
    state.workbench.workspace = Workspace::Results;
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset_id));
    let source_key = format!(
        "project:{}:result-dataset:{}",
        state.model.workspace.content.project.id().as_uuid(),
        dataset_id
    );
    (state, source_key)
}

/// What a print of the Specs sheet says, and the identity it says it under.
fn printed_specification_bounds(
    state: &AppState,
    source_key: &str,
) -> (String, HardcopyDocumentId) {
    let resolved = prepare_retained_hardcopy_resolution(
        SourceReads::of(state),
        source_key,
        HardcopyScope::ActivePlotDocument,
    )
    .expect("Specs prepares")
    .into_worker_snapshot_json()
    .and_then(|bytes| PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&bytes))
    .expect("Specs round-trips through the worker snapshot")
    .resolve_owned()
    .expect("Specs resolves");
    let HardcopySemanticDocument::ResultSummary(summary) = resolved.semantic_document() else {
        panic!("expected Specs result summary")
    };
    (
        summary.tables[0].rows[0][3].clone(),
        resolved.authority().document_id(),
    )
}

/// The print states the bound the run was judged against, whatever the
/// workspace's contract has become since.
///
/// The capture read `state.workspace.specs` — the live, editable contract —
/// so editing a limit after a completed run both restated that limit on the
/// printed page beside the verdict the run had actually earned, and moved the
/// printed document's identity, which is derived from the captured
/// requirements. Neither is a thing an immutable result may do.
#[test]
fn printed_specifications_are_the_ones_the_run_froze_not_the_ones_now_authored() {
    let (mut state, source_key) = signed_off_specification_workspace();
    state.model.workspace.content.specs = vec![crate::state::SpecEntry {
        measurement: "gain".to_owned(),
        expression: "max gain".to_owned(),
        min: Some(10.0),
        max: None,
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    }];
    let (before, identity_before) = printed_specification_bounds(&state, &source_key);

    // The reader raises the limit after the run. The retained dataset was
    // never judged against 20 dB.
    state.model.workspace.content.specs[0].min = Some(20.0);
    let (after, identity_after) = printed_specification_bounds(&state, &source_key);

    assert_eq!(
        after,
        format!("value >= {:.17e}", 10.0),
        "the printed page states the frozen bound, not the one the workspace now holds"
    );
    assert_eq!(before, after);
    assert_eq!(
        identity_before, identity_after,
        "an immutable result's printed identity does not move when the plan is edited"
    );
}

#[test]
fn quick_view_reads_exact_active_retained_waveform_without_report_reference() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(7, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![-0.25, 0.5, 1.25],
                "#00ffff",
            ),
        ]),
    );
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state.model.simulation.view.data_version = 9;
    state.results.results.session.viewer = ResultViewer::Waves;

    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic plot")
    };
    assert_eq!(plot.viewer, ResultViewer::Waves);
    assert_eq!(
        plot.traces[0].source_samples,
        vec![
            (0.0f64.to_bits(), (-0.25f64).to_bits()),
            (1.0f64.to_bits(), 0.5f64.to_bits()),
            (2.0f64.to_bits(), 1.25f64.to_bits()),
        ]
    );
    assert!(
        resolved
            .default_print_mapping()
            .entries()
            .iter()
            .any(|entry| {
                entry.object().kind() == PrintObjectKind::Trace
                    && entry.object().stable_id() == format!("trace:{}", plot.traces[0].trace_id)
            })
    );
    assert_eq!(resolved.authority().revision(), ObjectRevision::INITIAL);
}

#[test]
fn nyquist_and_smith_require_active_retained_complex_samples() {
    let real_only =
        AnalysisResult::new(10, AnalysisType::Ac, "AC").with_waveforms(vec![RetainedTrace::new(
            "V(out)",
            vec![1.0; 16],
            vec![2.0; 16],
            "#00ffff",
        )]);
    let mut stale = quick_view_state(real_only, ResultViewer::Nyquist);
    stale.results.analysis.nyquist_state.load_data(
        rspice_results_ui::nyquist::NyquistData::from_arrays(
            "stale",
            &[1.0, 2.0],
            &[9_999.0, 8_888.0],
            &[7_777.0, 6_666.0],
        ),
    );
    assert!(matches!(
        resolve_quick_view(&stale),
        Err(HardcopySourceError::UnavailableRetainedSource { .. })
    ));

    let complex = AnalysisResult::new(10, AnalysisType::Ac, "AC").with_waveforms(vec![
        RetainedTrace::new(
            "S(1,1)",
            (0..16).map(|index| index as f64).collect::<Vec<_>>(),
            vec![0.0; 16],
            "#00ffff",
        )
        .with_complex_components(
            "S(1,1)",
            (0..16).map(|index| index as f64 / 16.0).collect::<Vec<_>>(),
            (0..16)
                .map(|index| -(index as f64) / 32.0)
                .collect::<Vec<_>>(),
        ),
    ]);
    let state = quick_view_state(complex.clone(), ResultViewer::Nyquist);
    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected complex plot")
    };
    assert_eq!(plot.viewer, ResultViewer::Nyquist);
    assert_eq!(
        plot.traces[0].source_samples[1],
        ((1.0f64 / 16.0).to_bits(), (-1.0f64 / 32.0).to_bits())
    );

    let smith = quick_view_state(complex, ResultViewer::Smith);
    assert!(matches!(
        resolve_quick_view(&smith),
        Err(HardcopySourceError::UnavailableRetainedSource { .. })
    ));
}

#[test]
fn device_only_operating_point_hardcopy_preserves_the_device_table() {
    let analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_device_op(
        rspice_core::circuit::DeviceOpReport {
            entries: vec![rspice_core::circuit::DeviceOpEntry {
                name: "M1".to_owned(),
                device_kind: "MOSFET",
                region: Some("saturation"),
                params: vec![("gm", 1.25e-3), ("vds", 0.8)],
            }],
        },
    );
    let state = quick_view_state(analysis, ResultViewer::Op);
    let resolved = resolve_quick_view(&state).expect("device-only OP resolves");
    let HardcopySemanticDocument::ResultSummary(summary) = resolved.semantic_document() else {
        panic!("expected operating-point summary")
    };
    let table = summary
        .tables
        .iter()
        .find(|table| table.title == "Device operating point")
        .expect("device table is retained");
    assert_eq!(table.rows.len(), 2);
    assert_eq!(&table.rows[0][..4], ["M1", "MOSFET", "saturation", "gm"]);
    assert_eq!(table.rows[0][5], "S");
    assert_eq!(table.rows[0][4].parse::<f64>().unwrap(), 1.25e-3);
    assert_eq!(table.rows[1][3], "vds");
    assert_eq!(table.rows[1][5], "V");
}

#[test]
fn project_visualization_hardcopy_uses_the_selected_pane_not_the_first_pane() {
    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x7d; 32]));
    let dataset = SourceDataset::new(
        binding,
        vec![
            SourceColumn::new("x", "X", ValueType::Real, ColumnRole::Coordinate, None).unwrap(),
            SourceColumn::new("y", "Y", ValueType::Real, ColumnRole::Signal, None).unwrap(),
        ],
        vec![SourceRow::new(vec![
            TypedValue::Real(0.0),
            TypedValue::Real(1.0),
        ])],
    )
    .unwrap();
    let mut document = VisualizationDocument::new("Selected pane", vec![dataset]).unwrap();
    let page_id = document.pages()[0].id;
    let first_pane = document.panes()[0].id;
    let pane_binding = crate::results::visualization_document::PaneDataBinding {
        analysis_id: crate::product::AnalysisInstanceId::new(),
        dataset: binding,
    };
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneSource {
                pane_id: first_pane,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(pane_binding),
            }],
        )
        .unwrap();
    let receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddBoundPane(
                crate::results::visualization_document::NewPane {
                    page_id,
                    title: "Selected second pane".to_owned(),
                    kind: crate::results::visualization_document::PaneKind::Cartesian,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(pane_binding),
                    placement: crate::results::visualization_document::PanePlacement::Below {
                        anchor_pane_id: first_pane,
                    },
                },
            )],
        )
        .unwrap();
    let second_pane = receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Pane(id) => Some(*id),
            _ => None,
        })
        .unwrap();
    let document_id = document.id();
    let mut state = AppState::default();
    let project = &mut state.model.workspace.content;
    project.visualization_documents.push(document);
    state
        .results
        .results
        .session
        .select_persistent_document_pane(document_id, second_pane);

    let (_, _, selected) =
        active_visualization_document_pane(&state.model, &*state.results, document_id).unwrap();
    assert_eq!(selected.id, second_pane);
    assert_eq!(selected.title, "Selected second pane");
}

#[test]
fn qpnoise_result_noise_figure_export_preserves_signed_output_frequencies() {
    let result = crate::simulation::results::qpnoise_retained_test_fixture();
    let expected = result.waveforms.len();
    let state = quick_view_state(result, ResultViewer::NoiseContrib);
    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected noise spectrum figure");
    };
    assert_eq!(plot.viewer, ResultViewer::NoiseContrib);
    assert_eq!(plot.traces.len(), expected);
    assert!(
        plot.traces
            .iter()
            .flat_map(|t| &t.source_samples)
            .any(|p| f64::from_bits(p.0) < 0.0)
    );
}
