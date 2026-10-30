//! Creating a result document through a whole application: the dialog's
//! family, name and viewer choices, and a commit that the project owns.

use crate::results::visualization_document::ResultDocumentTracking;
use crate::results::visualization_document::TypedValue;
use crate::state::AnalysisResult;
use crate::state::AnalysisResultFamilyMetadata;
use crate::state::AnalysisResultPayload;
use crate::state::AnalysisType;
use crate::state::DigitalEventPointEvidence;
use crate::state::DigitalEventTraceEvidence;
use crate::state::RetainedTrace;
use crate::state::SimulationRun;
use crate::state::SimulationRunLifecycle;
use crate::state::SoaEvaluationEvidence;
use crate::state::SoaParameterEvidence;
use crate::state::SoaRuleVerdictEvidence;
use crate::state::SoaViolationEvidence;
use crate::state::SoaViolationSeverityEvidence;
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::create_document::*;
use crate::workbench::state::Workspace;
use crate::workbench::state::WorkspaceDocumentId;
use rspice_results::document_creation::ResultDocumentFamily;
use rspice_results::document_creation::ResultDocumentLayout;
use rspice_results_ui::create_document::{CreateDocumentHost, CreateResultDocumentDialogState};

fn retained_run_with(analysis_type: AnalysisType) -> SimulationRun {
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    let (name, x) = if analysis_type.is_bode_response() {
        ("|V(out)|", vec![1.0, 2.0, 3.0])
    } else if analysis_type.is_raw_frequency_curve() {
        ("V(out)", vec![1.0, 2.0, 3.0])
    } else {
        ("V(out)", vec![0.0, 0.5, 1.0])
    };
    run.analyses.push(
        AnalysisResult::new(11, analysis_type, "analysis").with_waveforms(vec![
            RetainedTrace::new(name, x, vec![0.0, 1.0, 0.0], "#55aaff"),
        ]),
    );
    run
}

#[test]
fn compatible_filter_uses_the_selected_dataset_not_the_active_dataset() {
    let mut app = RSpiceApp::test_instance();
    let transient = retained_run_with(AnalysisType::Transient);
    let transient_id = transient.dataset_id;
    let ac = retained_run_with(AnalysisType::Ac);
    let ac_id = ac.dataset_id;
    app.state.model.simulation.retained.runs = vec![transient, ac].into();

    assert_eq!(
        first_compatible_viewer(
            app.state.results_reads(),
            Some(transient_id),
            ResultDocumentFamily::WaveformWorksheet,
        ),
        Some("viewer-waveform")
    );
    assert_eq!(
        first_compatible_viewer(
            app.state.results_reads(),
            Some(ac_id),
            ResultDocumentFamily::FrequencyAndStability,
        ),
        Some("viewer-bode")
    );
}

#[test]
fn explicit_nondefault_viewer_choice_is_creatable_and_resolves_unchanged() {
    let mut app = RSpiceApp::test_instance();
    let run = retained_run_with(AnalysisType::Transient);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    let family = ResultDocumentFamily::WaveformWorksheet;
    assert!(viewer_is_creatable(
        app.state.results_reads(),
        Some(dataset_id),
        family,
        "eye-viewer",
    ));
    let draft = CreateResultDocumentDialogState {
        open: true,
        name: "Eye review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: family.id().to_owned(),
        viewer_id: "eye-viewer".to_owned(),
        layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
        validation_error: None,
    };

    let resolved =
        resolve_draft(app.state.results_reads(), &draft).expect("explicit viewer resolves");

    assert_eq!(resolved.viewer.id, "eye-viewer");
}

#[test]
fn untouched_default_name_follows_the_selected_family() {
    let mut app = RSpiceApp::test_instance();
    open(&mut app.state.results_parts());
    let mut draft = app.state.results.create_result_document.clone();
    let family = ResultDocumentFamily::FrequencyAndStability;

    CreateDocumentSources(app.state.results_reads()).select_family(&mut draft, family);

    assert_eq!(draft.name, format!("{} · 01", family.label()));
    assert!(!draft.name_touched);
}

#[test]
fn typed_name_survives_a_family_change() {
    let mut app = RSpiceApp::test_instance();
    open(&mut app.state.results_parts());
    let mut draft = app.state.results.create_result_document.clone();
    draft.name = "Loop gain review".to_owned();
    draft.name_touched = true;

    CreateDocumentSources(app.state.results_reads())
        .select_family(&mut draft, ResultDocumentFamily::FrequencyAndStability);

    assert_eq!(draft.name, "Loop gain review");
    assert_eq!(
        draft.family_id,
        ResultDocumentFamily::FrequencyAndStability.id()
    );
    assert!(draft.name_touched);
}

#[test]
fn commit_is_project_owned_dirty_and_stably_selected() {
    let mut app = RSpiceApp::test_instance();
    let run = retained_run_with(AnalysisType::Transient);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: ResultDocumentFamily::WaveformWorksheet.id().to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: ResultDocumentLayout::TwoLinkedPanes.id().to_owned(),
        validation_error: None,
    };

    let document_id = commit(&mut app.state.results_parts()).expect("document commits");

    assert!(
        app.state
            .model
            .workspace
            .content
            .visualization_documents_dirty
    );
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("project owns created document");
    assert_eq!(document.title(), "Transient review");
    assert_eq!(document.panes().len(), 2);
    assert_eq!(document.axes().len(), 4);
    assert_eq!(document.traces().len(), 2);
    assert!(document.traces().iter().all(|trace| {
        trace.label == "V(out)"
            && trace.coordinate_key == "x"
            && trace.signal_key == "y"
            && trace.row_predicates.len() == 4
    }));
    assert_eq!(document.tracking(), ResultDocumentTracking::pinned());
    assert_eq!(
        app.state.workbench.documents.active(Workspace::Results),
        Some(&WorkspaceDocumentId::VisualizationDocument(document_id))
    );
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::Results
    );
    assert_eq!(
        app.state.results.results.session.viewer,
        crate::workbench::ResultViewer::Waves
    );
}

#[test]
fn release_target_table_commits_from_typed_operating_point_without_waveforms() {
    let mut app = RSpiceApp::test_instance();
    let mut analysis = AnalysisResult::new(11, AnalysisType::DcOp, "Operating point");
    analysis.dc_op = Some(crate::state::DcOpResult {
        node_voltages: vec![crate::state::OperatingPointValue {
            name: "V(out)".to_owned(),
            value: 1.234_567_890_123_456,
            unit: "V".to_owned(),
        }],
        branch_currents: Vec::new(),
        power_dissipation: Vec::new(),
    });
    let mut run = SimulationRun::new(7);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(analysis);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.results.create_result_document = CreateResultDocumentDialogState {
        open: true,
        name: "Typed OP datasheet".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: ResultDocumentFamily::StatisticsAndYield.id().to_owned(),
        viewer_id: "viewer-table".to_owned(),
        layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
        validation_error: None,
    };

    let document_id = commit(&mut app.state.results_parts()).expect("typed OP table commits");
    let document = app
        .state
        .model
        .workspace
        .content
        .visualization_document(document_id)
        .expect("workspace owns typed table");
    assert_eq!(document.panes()[0].viewer_id, "viewer-table");
    assert_eq!(document.axes().len(), 2);
    assert!(document.traces().is_empty());
    assert!(
        app.state.model.simulation.retained.runs[0].analyses[0]
            .waveforms
            .is_empty()
    );
}

#[test]
fn typed_release_target_campaign_viewers_commit_with_embedded_source_evidence() {
    let events = AnalysisResult::new(21, AnalysisType::Transient, "TRAN events")
        .with_result_payload(AnalysisResultPayload::TransientEvents {
            voltage_impulses: None,
            current_impulses: None,
            digital_traces: vec![DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: vec![
                    DigitalEventPointEvidence {
                        time_s: 0.0,
                        value_code: 0,
                    },
                    DigitalEventPointEvidence {
                        time_s: 1.0e-9,
                        value_code: 1,
                    },
                ],
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        });
    let soa_time = vec![0.0, 1.0e-9];
    let soa = AnalysisResult::new(22, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa {
            time: soa_time.clone(),
        })
        .with_waveforms(vec![
            RetainedTrace::new(
                "SOA_VIOLATION_COUNT",
                soa_time.clone(),
                vec![0.0, 0.0],
                "#ffbd2e",
            ),
            RetainedTrace::new(
                crate::results::safety::soa_stress_waveform_name(
                    "M1",
                    crate::results::safety::SoAParameter::Vds,
                ),
                soa_time,
                vec![2.0, 3.0],
                "#00aaff",
            ),
        ])
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                envelope: None,
                derating: None,
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value: 3.0,
                worst_time_s: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                verdict: SoaRuleVerdictEvidence::Warning,
            }],
            violations: vec![SoaViolationEvidence {
                device_id: "M1".to_owned(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: 3.0,
                time_s: 1.0e-9,
                severity: SoaViolationSeverityEvidence::Warning,
            }],
        });
    let optimization = AnalysisResult::new(23, AnalysisType::Optimization, "Optimization")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            best_objectives: Vec::new(),
            best_constraints: Vec::new(),
            iterations: vec![0.0, 1.0],
            best_cost: 0.25,
            best_variables: [("w".to_owned(), 1.5e-6)].into_iter().collect(),
            converged: true,
        })
        .with_waveforms(vec![
            RetainedTrace::new("OPT_COST", vec![0.0, 1.0], vec![1.0, 0.25], "#ffbd2e"),
            RetainedTrace::new("OPT_w", vec![0.0, 1.0], vec![1.0e-6, 1.5e-6], "#00aaff"),
        ]);

    for (analysis, family, viewer_id, evidence_kind) in [
        (
            events,
            ResultDocumentFamily::DigitalAndAmsEvents,
            "viewer-digital-events",
            "analysis-result-payload",
        ),
        (
            soa,
            ResultDocumentFamily::VerificationAndOptimization,
            "viewer-soa",
            "analysis-result-payload",
        ),
        (
            optimization,
            ResultDocumentFamily::VerificationAndOptimization,
            "viewer-optimization",
            "analysis-family-metadata",
        ),
    ] {
        let mut app = RSpiceApp::test_instance();
        let mut run = SimulationRun::new(31);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses.push(analysis);
        let dataset_id = run.dataset_id;
        app.state.model.simulation.retained.runs = vec![run].into();
        app.state.results.create_result_document = CreateResultDocumentDialogState {
            open: true,
            name: format!("{viewer_id} review"),
            name_touched: true,
            dataset_id: Some(dataset_id),
            family_id: family.id().to_owned(),
            viewer_id: viewer_id.to_owned(),
            layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
            validation_error: None,
        };

        let document_id =
            commit(&mut app.state.results_parts()).expect("typed release-target viewer commits");
        let document = app
            .state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("workspace owns typed document");
        assert_eq!(document.panes()[0].viewer_id, viewer_id);
        assert!(document.datasets()[0].rows().iter().any(|row| {
            matches!(row.values().get(2), Some(TypedValue::Text(kind)) if kind == evidence_kind)
        }));
    }
}

#[test]
fn duplicate_name_revalidation_leaves_project_unchanged() {
    let mut app = RSpiceApp::test_instance();
    let run = retained_run_with(AnalysisType::Transient);
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    let draft = CreateResultDocumentDialogState {
        open: true,
        name: "Transient review".to_owned(),
        name_touched: true,
        dataset_id: Some(dataset_id),
        family_id: ResultDocumentFamily::WaveformWorksheet.id().to_owned(),
        viewer_id: "viewer-waveform".to_owned(),
        layout_id: ResultDocumentLayout::SinglePane.id().to_owned(),
        validation_error: None,
    };
    app.state.results.create_result_document = draft.clone();
    commit(&mut app.state.results_parts()).expect("first document commits");
    app.state.results.create_result_document = draft;
    let before = app
        .state
        .model
        .workspace
        .content
        .visualization_documents
        .len();

    assert!(commit(&mut app.state.results_parts()).is_err());
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .visualization_documents
            .len(),
        before
    );
}
