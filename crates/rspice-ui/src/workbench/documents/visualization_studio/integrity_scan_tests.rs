//! Integrity-scan tests for exact visualization sources, comparisons, and saved properties.

use super::dock::{
    commit_comparison_execution, evaluate_scalar_measurement, execute_comparison_draft,
    execute_comparison_draft_with_differences, retain_difference_trace_sets,
    save_document_properties,
};
use super::*;
use crate::state::{AnalysisResult, AnalysisType, SimulationRun, SpecEntry, WaveformData};
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, WavePanePresentationKey,
};

fn app_with_exact_source() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let analysis = AnalysisResult::new(17, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new(
            "V(out)",
            vec![0.0, 0.5, 1.0],
            vec![-1.25, 2.5, 4.0],
            "#00aaff",
        ),
        WaveformData::new("I(R1)", vec![10.0, 20.0], vec![0.125, -0.25], "#ffaa00"),
    ]);
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    app.state.simulation.runs = vec![run];
    assert!(app.state.simulation.select_run(0));
    app
}

fn activate_voltage_wave_pane(app: &mut RSpiceApp) -> AnalysisPresentationKey {
    let run = app.state.simulation.active_run().expect("active run");
    let analysis = app
        .state
        .simulation
        .active_analysis()
        .expect("active analysis");
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    app.state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
        analysis: key,
        unit: "V".to_owned(),
    });
    key
}

fn apply_queued_view_gesture(ctx: &egui::Context, app: &mut RSpiceApp) {
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            result_document::apply_pending_view_gesture(ui, &mut app.state);
        });
    });
}

fn activate_project_visualization_document(app: &mut RSpiceApp) -> ResultDocumentId {
    let (source, analysis_id) = {
        let run = app.state.simulation.active_run().expect("active run");
        let analysis = app
            .state
            .simulation
            .active_analysis()
            .expect("active analysis");
        let source = result_document::visualization_source_dataset(run, analysis)
            .expect("retained source projects");
        let analysis_id = analysis.provenance().map_or_else(
            || {
                let name = format!("legacy-analysis-v1/{}", analysis.id);
                AnalysisInstanceId::from_namespace(run.dataset_id.as_uuid(), name.as_bytes())
            },
            |provenance| provenance.source_instance_id(),
        );
        (source, analysis_id)
    };
    let binding = crate::results::visualization_document::PaneDataBinding {
        analysis_id,
        dataset: source.binding(),
    };
    let mut document = crate::results::visualization_document::VisualizationDocument::new(
        "Engineering review",
        vec![source],
    )
    .expect("visualization document");
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneSource {
                pane_id,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(binding),
            }],
        )
        .expect("pane binding commits");
    let document_id = app
        .state
        .workspace
        .insert_visualization_document(document)
        .expect("document inserted");
    app.state.workbench.activate(Workspace::Results);
    assert!(
        crate::workbench::chrome::document_bar::activate_document_by_id(
            &mut app.state,
            &WorkspaceDocumentId::VisualizationDocument(document_id),
        )
    );
    document_id
}

#[test]
fn document_properties_commit_to_the_active_project_document() {
    let mut app = app_with_exact_source();
    let document_id = activate_project_visualization_document(&mut app);
    let before = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("active document")
        .revision();

    save_document_properties(&mut app, 13, true).expect("properties commit");

    let document = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("active document");
    assert!(document.revision() > before);
    assert_eq!(document.presentation().significant_digits, 13);
    assert!(document.presentation().phase_continuous);
    assert!(app.state.workspace.visualization_documents_dirty);
}

#[test]
fn studio_export_requires_the_selected_successful_completed_binding() {
    let mut app = app_with_exact_source();
    activate_project_visualization_document(&mut app);
    reconcile_document(&mut app);
    assert!(!active_studio_exact_export_available(&app.state));

    app.state.simulation.runs[0].lifecycle = crate::state::SimulationRunLifecycle::Completed;
    assert!(active_studio_exact_export_available(&app.state));
    assert!(active_studio_figure_export_available(&app.state));

    app.state.simulation.runs[0].analyses[0].success = false;
    assert!(!active_studio_exact_export_available(&app.state));
    assert!(!active_studio_figure_export_available(&app.state));
}

#[test]
fn canonical_result_entities_commit_and_project_without_parallel_authority() {
    let mut app = app_with_exact_source();
    let document_id = activate_project_visualization_document(&mut app);
    reconcile_document(&mut app);

    let document = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("active document");
    assert_eq!(document.traces().len(), 2);
    assert!(document.traces().iter().all(|trace| {
        trace.coordinate_key == "x" && trace.signal_key == "y" && trace.row_predicates.len() == 4
    }));

    add_marker_at_midpoint(&mut app);
    let (pane_id, trace_id) =
        active_project_pane_and_trace(&app.state, Some("V(out)")).expect("bound trace");
    transact_active_project_document(
        &mut app,
        vec![DocumentEdit::AddScalarMeasurement {
            pane_id,
            trace_ids: vec![trace_id],
            expression: "rms(V(out))".to_owned(),
            value: 2.75,
        }],
    )
    .expect("measurement commits");
    transact_active_project_document(
        &mut app,
        vec![DocumentEdit::AddAnnotation {
            pane_id,
            anchor: crate::results::visualization_document::AnnotationAnchor::Trace {
                trace_id,
                coordinate: TypedValue::Real(0.5),
            },
            text: "Review this exact source point".to_owned(),
        }],
    )
    .expect("annotation commits");
    reconcile_document(&mut app);

    let document = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("active document");
    assert_eq!(document.markers().len(), 1);
    assert_eq!(document.measurements().len(), 1);
    assert_eq!(document.annotations().len(), 1);
    assert_eq!(app.state.workbench.visualization_studio.markers.len(), 1);
    assert_eq!(app.state.workbench.visualization_studio.markers[0].x, 0.5);
    assert_eq!(
        app.state.workbench.visualization_studio.measurements[0].expression,
        "rms(V(out))"
    );
    assert_eq!(
        app.state.workbench.visualization_studio.annotations[0].text,
        "Review this exact source point"
    );

    transact_active_project_document(&mut app, vec![DocumentEdit::ClearMarkers { pane_id: None }])
        .expect("marker clear commits atomically");
    reconcile_document(&mut app);
    assert!(
        app.state
            .workspace
            .visualization_document(document_id)
            .expect("active document")
            .markers()
            .is_empty()
    );
    assert!(app.state.workbench.visualization_studio.markers.is_empty());
}

#[test]
fn canonical_ab_cursors_persist_link_move_and_clear_as_document_entities() {
    let mut app = app_with_exact_source();
    let document_id = activate_project_visualization_document(&mut app);
    reconcile_document(&mut app);
    let first_pane = app
        .state
        .workbench
        .visualization_studio
        .active_pane
        .expect("project pane");

    add_cursor_at_midpoint(&mut app);
    add_cursor_at_midpoint(&mut app);
    let document = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("active document");
    assert_eq!(document.cursors().len(), 2);
    assert_eq!(
        canonical_cursor_pair(document, document.panes()[0].id).unwrap(),
        (Some(0.5), Some(0.5))
    );

    assert!(commit_active_project_cursor_pair(
        &mut app,
        first_pane,
        (None, None)
    ));
    assert!(
        app.state
            .workspace
            .visualization_document(document_id)
            .expect("active document")
            .cursors()
            .is_empty()
    );

    let (page_id, anchor_pane, binding) = {
        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document");
        let pane = &document.panes()[0];
        (pane.page_id, pane.id, pane.binding)
    };
    let receipt = transact_active_project_document(
        &mut app,
        vec![DocumentEdit::AddBoundPane(
            crate::results::visualization_document::NewPane {
                page_id,
                title: "Linked waveform".to_owned(),
                kind: crate::results::visualization_document::PaneKind::Cartesian,
                viewer_id: "viewer-waveform".to_owned(),
                binding,
                placement: crate::results::visualization_document::PanePlacement::Below {
                    anchor_pane_id: anchor_pane,
                },
            },
        )],
    )
    .expect("second pane commits");
    let second_pane = receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Pane(pane_id) => Some(*pane_id),
            _ => None,
        })
        .expect("second pane id");
    let cursor_edits = {
        let document = app
            .state
            .workspace
            .visualization_document(document_id)
            .expect("active document");
        [anchor_pane, second_pane]
            .into_iter()
            .flat_map(|pane_id| {
                let axis_id = document
                    .axes()
                    .iter()
                    .find(|axis| {
                        axis.pane_id == pane_id && axis.orientation == AxisOrientation::Horizontal
                    })
                    .expect("horizontal axis")
                    .id;
                [("A", 0.1), ("B", 0.2)].map(|(label, position)| DocumentEdit::AddCursor {
                    pane_id,
                    axis_id,
                    position: TypedValue::Real(position),
                    label: label.to_owned(),
                })
            })
            .collect::<Vec<_>>()
    };
    transact_active_project_document(&mut app, cursor_edits).expect("cursors commit");
    reconcile_document(&mut app);
    assert!(set_active_project_cursor_links(&mut app, true));

    assert!(commit_active_project_cursor_pair(
        &mut app,
        first_pane,
        (Some(0.25), Some(0.75))
    ));
    let document = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("active document");
    assert_eq!(document.link_groups().len(), 2);
    assert!(document.cursors().iter().all(|cursor| {
        let expected = if cursor.label == "A" { 0.25 } else { 0.75 };
        real_cursor_position(cursor)
            .is_some_and(|position| same_cursor_position(position, expected))
    }));

    assert!(commit_active_project_cursor_pair(
        &mut app,
        first_pane,
        (None, None)
    ));
    let document = app
        .state
        .workspace
        .visualization_document(document_id)
        .expect("active document");
    assert!(document.cursors().is_empty());
    assert!(document.link_groups().is_empty());
}

#[test]
fn restored_exact_extrema_policy_normalizes_for_non_wave_renderer() {
    let mut app = app_with_exact_source();
    app.state.ui.results.viewer = ResultViewer::Bode;
    app.state.workbench.visualization_studio.autoscale = VisualizationAutoscale::ExactExtrema;

    reconcile_document(&mut app);

    assert_eq!(app.state.ui.results.viewer, ResultViewer::Bode);
    assert_eq!(
        app.state.workbench.visualization_studio.autoscale,
        VisualizationAutoscale::RobustVisible
    );
    assert_eq!(fit_block_reason(&app.state), None);
}

#[test]
fn fit_contract_reports_exact_source_and_specification_blockers() {
    let mut app = app_with_exact_source();
    app.state.workbench.visualization_studio.autoscale = VisualizationAutoscale::ExactExtrema;
    assert_eq!(fit_block_reason(&app.state), None);

    for waveform in &mut app.state.simulation.runs[0].analyses[0].waveforms {
        waveform.visible = false;
    }
    assert_eq!(
        fit_block_reason(&app.state),
        Some("Exact-extrema fitting requires at least one visible waveform with finite samples.")
    );

    app.state.workbench.visualization_studio.autoscale =
        VisualizationAutoscale::SpecificationBounds;
    assert_eq!(
        fit_block_reason(&app.state),
        Some(
            "Specification-bound fitting requires a visible waveform whose exact quantity name matches a configured project specification."
        )
    );

    app.state.ui.results.viewer = ResultViewer::Smith;
    assert_eq!(
        fit_block_reason(&app.state),
        Some("Specification-bound fitting is available only for the waveform renderer.")
    );
}

#[test]
fn robust_fit_is_available_for_non_wave_renderers() {
    let mut app = app_with_exact_source();
    app.state.ui.results.viewer = ResultViewer::Bode;
    app.state.workbench.visualization_studio.autoscale = VisualizationAutoscale::RobustVisible;
    app.state.workbench.visualization_studio.zoom = 2.5;

    assert_eq!(fit_block_reason(&app.state), None);
    fit_active_view(&mut app);

    assert_eq!(app.state.workbench.visualization_studio.zoom, 1.0);
}

#[test]
fn specification_bound_fit_requires_an_exact_visible_quantity_binding() {
    let mut app = app_with_exact_source();
    app.state.workspace.specs.push(SpecEntry {
        measurement: "v(OUT)".to_owned(),
        expression: String::new(),
        min: Some(-2.0),
        max: Some(5.0),
        unit: "V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    });

    assert_eq!(
        specification_bound_fit(&app.state),
        Some(((0.0, 20.0), (-2.0, 5.0)))
    );

    app.state.workspace.specs[0].measurement = "V(unrelated)".to_owned();
    assert_eq!(specification_bound_fit(&app.state), None);
}

#[test]
fn specification_bound_autoscale_commits_the_exact_data_and_limit_envelope() {
    let mut app = app_with_exact_source();
    let analysis_key = activate_voltage_wave_pane(&mut app);
    app.state.workspace.specs.push(SpecEntry {
        measurement: "V(out)".to_owned(),
        expression: String::new(),
        min: Some(-2.0),
        max: Some(5.0),
        unit: "V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    });
    app.state.workbench.visualization_studio.autoscale =
        VisualizationAutoscale::SpecificationBounds;
    app.state.workbench.visualization_studio.zoom = 3.0;

    fit_active_view(&mut app);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    apply_queued_view_gesture(&ctx, &mut app);

    let view = app
        .state
        .ui
        .results
        .analysis_plot_view_pane(ResultViewer::Waves, analysis_key, 0);
    assert_eq!(view.x, Some((0.0, 20.0)));
    assert_eq!(view.y, Some((-2.0, 5.0)));
    assert_eq!(app.state.workbench.visualization_studio.zoom, 1.0);
}

#[test]
fn transfer_function_result_resolves_the_canonical_xf_viewer_contract() {
    let analysis_ids = [analysis_manifest_id(AnalysisType::Tf)];
    assert_eq!(analysis_ids, ["xf"]);
    assert_eq!(
        viewer_compatibility(
            "viewer-transfer-function",
            ViewerCapabilities {
                analysis_ids: &analysis_ids,
                external_capabilities: &[],
            },
        ),
        ViewerCompatibility::Compatible
    );
}

#[test]
fn comparison_source_projection_preserves_dc_and_hb_specialist_modes() {
    let dc = AnalysisResult::new(1, AnalysisType::DcSweep, "DC");
    assert_eq!(
        result_document::project_viewer_for_analysis(ResultViewer::Waves, &dc),
        ResultViewer::DcSweep
    );

    let hb = AnalysisResult::new(2, AnalysisType::HarmonicBalance, "HB").with_waveforms(vec![
        WaveformData::new(
            "|V(out) Spectrum|",
            vec![1.0e9, 2.0e9],
            vec![1.0, 0.1],
            "#00aaff",
        )
        .with_complex_components("V(out) Spectrum", vec![1.0, 0.1], vec![0.0, 0.02]),
    ]);
    assert_eq!(
        result_document::project_viewer_for_analysis(ResultViewer::Fft, &hb),
        ResultViewer::HarmonicBalance
    );

    let transient = AnalysisResult::new(3, AnalysisType::Transient, "TRAN");
    assert_eq!(
        result_document::project_viewer_for_analysis(ResultViewer::Waves, &transient),
        ResultViewer::Waves
    );
}

fn exact_source_checksum(app: &RSpiceApp) -> u64 {
    let mut checksum = 0xcbf2_9ce4_8422_2325_u64;
    for (&x, &y) in app
        .state
        .simulation
        .active_analysis()
        .expect("test analysis must remain selected")
        .waveforms
        .iter()
        .flat_map(|waveform| waveform.x.iter().zip(waveform.y.iter()))
    {
        checksum ^= x.to_bits();
        checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
        checksum ^= y.to_bits();
        checksum = checksum.wrapping_mul(0x0000_0100_0000_01b3);
    }
    checksum
}

#[test]
fn processes_every_exact_sample_and_completes_with_expected_checksum() {
    let mut app = app_with_exact_source();
    let expected_checksum = exact_source_checksum(&app);
    let expected_binding = source_integrity_scan_binding(&app.state)
        .expect("exact retained samples must produce a scan binding");

    start_source_integrity_scan(&mut app);

    let studio = &app.state.workbench.visualization_studio;
    assert_eq!(studio.operation_state, OperationState::Running);
    assert_eq!(studio.operation_dataset_id, Some(expected_binding.0));
    assert_eq!(studio.operation_analysis_sequence, Some(17));
    assert_eq!(studio.operation_processed, 0);
    assert_eq!(studio.operation_total, 5);

    advance_source_integrity_scan(&mut app).expect("first exact chunk must scan");
    assert_eq!(
        app.state.workbench.visualization_studio.operation_processed,
        2
    );
    advance_source_integrity_scan(&mut app).expect("second exact chunk must scan");
    assert_eq!(
        app.state.workbench.visualization_studio.operation_processed,
        4
    );
    advance_source_integrity_scan(&mut app).expect("final exact chunk must scan");

    let studio = &app.state.workbench.visualization_studio;
    assert_eq!(studio.operation_state, OperationState::Completed);
    assert_eq!(studio.operation_processed, studio.operation_total);
    assert_eq!(studio.operation_checksum, expected_checksum);
    assert_eq!(
        advance_source_integrity_scan(&mut app),
        Err("The source-integrity scan is already complete".to_owned())
    );
}

#[test]
fn recovery_fails_closed_when_immutable_source_binding_disappears() {
    let mut app = app_with_exact_source();
    start_source_integrity_scan(&mut app);
    advance_source_integrity_scan(&mut app).expect("initial exact chunk must scan");
    app.state.workbench.visualization_studio.operation_state = OperationState::Cancelled;

    app.state.simulation.runs.clear();

    assert_eq!(
        recover_source_integrity_scan(&mut app),
        Err("The cancelled integrity scan's immutable source is unavailable".to_owned())
    );
    assert_eq!(
        app.state.workbench.visualization_studio.operation_state,
        OperationState::Cancelled
    );
}

#[test]
fn scalar_measurements_validate_against_the_exact_active_analysis() {
    let app = app_with_exact_source();
    let (dataset_id, analysis_sequence, value) =
        evaluate_scalar_measurement(&app.state, "rms(V(out))")
            .expect("RMS must produce a finite scalar measurement");
    assert_eq!(
        Some(dataset_id),
        app.state.simulation.active_run().map(|run| run.dataset_id)
    );
    assert_eq!(analysis_sequence, 17);
    assert!((value - (7.515625_f64).sqrt()).abs() < 1.0e-14); // ∫y²dt, time-weighted
    assert_eq!(
        evaluate_scalar_measurement(&app.state, "V(out)").unwrap_err(),
        "The expression produces a trace; reduce it with avg(), rms(), or another scalar function"
    );
}

#[test]
fn explicit_comparison_executes_exact_contract_without_mutating_sources() {
    let mut app = app_with_exact_source();
    let mut baseline = SimulationRun::new(2);
    baseline.add_analysis(
        AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 0.5, 1.0],
                vec![-1.20, 2.45, 3.95],
                "#00aaff",
            ),
            WaveformData::new("I(R1)", vec![10.0, 20.0], vec![0.125, -0.25], "#ffaa00"),
        ]),
    );
    let baseline_id = baseline.dataset_id;
    app.state.simulation.runs.push(baseline);
    app.state
        .workbench
        .visualization_studio
        .draft_comparison_dataset = Some(baseline_id);
    app.state
        .workbench
        .visualization_studio
        .draft_comparison_absolute_tolerance = 0.1;
    app.state
        .workbench
        .visualization_studio
        .draft_comparison_alignment = ComparisonAlignmentDraft::AbsoluteXAxis;
    let candidate_digest = app
        .state
        .simulation
        .active_run()
        .unwrap()
        .dataset_content_digest();

    let execution = execute_comparison_draft_with_differences(&app)
        .expect("exact comparison and checked difference traces must execute");
    let receipt = execution.receipt.clone();
    let result = app
        .state
        .workbench
        .visualization_studio
        .transact(|studio| retain_difference_trace_sets(studio, execution.difference_traces));
    result.expect("derived series identities must commit atomically");

    assert_eq!(receipt.rows_compared, 3);
    assert!(matches!(
        receipt.policy.execution.alignment,
        ComparisonAlignmentMethod::AbsoluteXAxis
    ));
    let trace_set = &app
        .state
        .workbench
        .visualization_studio
        .difference_trace_sets[0];
    assert_eq!(trace_set.coordinates, vec![0.0, 0.5, 1.0]);
    assert!(
        trace_set
            .absolute
            .values
            .iter()
            .all(|value| (*value - 0.05).abs() <= 1.0e-12)
    );
    assert!(
        trace_set
            .normalized
            .values
            .iter()
            .all(|value| (*value - 0.5).abs() <= 1.0e-12)
    );
    assert_eq!(
        HashSet::from([
            trace_set.id,
            trace_set.absolute.id,
            trace_set.relative.id,
            trace_set.normalized.id,
        ])
        .len(),
        4
    );
    assert_eq!(
        receipt.disposition,
        crate::results::visualization_document::ComparisonDisposition::Passed
    );
    assert_eq!(
        app.state
            .simulation
            .active_run()
            .unwrap()
            .dataset_content_digest(),
        candidate_digest
    );
}

#[test]
fn project_document_owns_comparison_receipts_and_studio_only_projects_them() {
    let mut app = app_with_exact_source();
    let document_id = activate_project_visualization_document(&mut app);
    reconcile_document(&mut app);
    let mut baseline = SimulationRun::new(2);
    baseline.add_analysis(
        AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 0.5, 1.0],
                vec![-1.20, 2.45, 3.95],
                "#00aaff",
            ),
            WaveformData::new("I(R1)", vec![10.0, 20.0], vec![0.125, -0.25], "#ffaa00"),
        ]),
    );
    let baseline_id = baseline.dataset_id;
    app.state.simulation.runs.push(baseline);
    let studio = &mut app.state.workbench.visualization_studio;
    studio.draft_comparison_dataset = Some(baseline_id);
    studio.draft_comparison_absolute_tolerance = 0.1;
    studio.draft_comparison_alignment = ComparisonAlignmentDraft::AbsoluteXAxis;

    let execution = execute_comparison_draft_with_differences(&app).unwrap();
    let expected_receipt = execution.receipt.clone();
    commit_comparison_execution(&mut app, execution).unwrap();

    let document = app
        .state
        .workspace
        .visualization_document(document_id)
        .unwrap();
    assert_eq!(document.comparisons(), &[expected_receipt.clone()]);
    assert_eq!(document.datasets().len(), 2);
    assert_eq!(
        app.state.workbench.visualization_studio.comparison_receipts,
        vec![expected_receipt]
    );
    assert!(
        !app.state
            .workbench
            .visualization_studio
            .difference_trace_sets
            .is_empty()
    );
}

#[test]
fn comparison_records_threshold_and_cross_correlation_alignment_parameters() {
    let mut threshold_app = app_with_exact_source();
    threshold_app.state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
        "V(out)",
        vec![0.0, 1.0, 2.0],
        vec![-1.0, 1.0, 3.0],
        "#00aaff",
    )];
    let mut threshold_baseline = SimulationRun::new(2);
    threshold_baseline.add_analysis(
        AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![10.0, 11.0, 12.0],
                vec![-2.0, 2.0, 4.0],
                "#00aaff",
            ),
        ]),
    );
    let threshold_baseline_id = threshold_baseline.dataset_id;
    threshold_app.state.simulation.runs.push(threshold_baseline);
    let threshold_studio = &mut threshold_app.state.workbench.visualization_studio;
    threshold_studio.draft_comparison_dataset = Some(threshold_baseline_id);
    threshold_studio.draft_comparison_alignment = ComparisonAlignmentDraft::FirstThresholdCrossing;
    threshold_studio.draft_comparison_alignment_signal = "V(out)".to_owned();
    threshold_studio.draft_comparison_threshold = 0.0;
    threshold_studio.draft_comparison_difference_trace = false;

    let threshold_receipt =
        execute_comparison_draft(&threshold_app).expect("threshold alignment must execute");
    assert!(matches!(
        threshold_receipt.policy.execution.alignment,
        ComparisonAlignmentMethod::FirstThresholdCrossing {
            signal_key,
            threshold: 0.0,
            baseline_crossing: 10.5,
            candidate_crossing: 0.5,
        } if signal_key == "signal:0"
    ));
    assert_eq!(
        threshold_receipt.policy.execution.resampling,
        ComparisonResamplingPolicy::BaselineOntoCandidateGrid
    );

    let mut correlation_app = app_with_exact_source();
    correlation_app.state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
        "V(out)",
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        vec![0.0, 0.0, 1.0, 0.0, -1.0, 0.0],
        "#00aaff",
    )];
    let mut correlation_baseline = SimulationRun::new(2);
    correlation_baseline.add_analysis(
        AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0],
                "#00aaff",
            ),
        ]),
    );
    let correlation_baseline_id = correlation_baseline.dataset_id;
    correlation_app
        .state
        .simulation
        .runs
        .push(correlation_baseline);
    let correlation_studio = &mut correlation_app.state.workbench.visualization_studio;
    correlation_studio.draft_comparison_dataset = Some(correlation_baseline_id);
    correlation_studio.draft_comparison_alignment = ComparisonAlignmentDraft::CrossCorrelation;
    correlation_studio.draft_comparison_alignment_signal = "V(out)".to_owned();
    correlation_studio.draft_comparison_maximum_lag_samples = 2;
    correlation_studio.draft_comparison_difference_trace = false;

    let correlation_receipt =
        execute_comparison_draft(&correlation_app).expect("correlation alignment must execute");
    assert!(matches!(
        correlation_receipt.policy.execution.alignment,
        ComparisonAlignmentMethod::CrossCorrelation {
            selected_lag_samples: 1,
            sample_interval: 1.0,
            baseline_shift: 1.0,
            ..
        }
    ));
}

#[test]
fn comparison_fails_closed_for_nonmonotonic_source_coordinates() {
    let mut app = app_with_exact_source();
    app.state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
        "V(out)",
        vec![0.0, 1.0, 2.0],
        vec![0.0, 1.0, 2.0],
        "#00aaff",
    )];
    let mut baseline = SimulationRun::new(2);
    baseline.add_analysis(
        AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 0.5],
                vec![0.0, 1.0, 2.0],
                "#00aaff",
            ),
        ]),
    );
    let baseline_id = baseline.dataset_id;
    app.state.simulation.runs.push(baseline);
    app.state
        .workbench
        .visualization_studio
        .draft_comparison_dataset = Some(baseline_id);
    app.state
        .workbench
        .visualization_studio
        .draft_comparison_alignment = ComparisonAlignmentDraft::AbsoluteXAxis;

    let error = execute_comparison_draft(&app)
        .expect_err("nonmonotonic immutable data must never be resampled");
    assert!(error.contains("nonmonotonic"));
}

#[test]
fn results_comparison_handoff_rebinds_a_stale_owner_to_the_active_document() {
    let mut app = app_with_exact_source();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.workspace = Workspace::Results;
    let candidate_id = app.state.simulation.runs[0].dataset_id;
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(candidate_id));

    let mut baseline = SimulationRun::new(2);
    baseline.add_analysis(
        AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 0.5, 1.0],
                vec![-1.20, 2.45, 3.95],
                "#00aaff",
            ),
        ]),
    );
    let baseline_id = baseline.dataset_id;
    app.state.simulation.runs.push(baseline);

    app.state.workbench.visualization_studio.panes = vec![VisualizationPane {
        id: 1,
        viewer: ResultViewer::Waves,
        viewer_document_id: ResultViewer::Waves
            .viewer_document_id()
            .expect("waveform viewer has a catalog document")
            .to_owned(),
        dataset_id: baseline_id,
        analysis_sequence: 29,
        x_link: None,
        cursor_group: None,
        page: "Engineering".to_owned(),
        placement: VisualizationPanePlacement::BelowSelected,
    }];
    app.state.workbench.visualization_studio.active_pane = Some(1);
    app.state.workbench.visualization_studio.next_identity = 2;

    open_results_comparison_inner(&mut app)
        .expect("a compatible retained baseline must open the real comparison owner");

    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        SurfaceId::VisualizationStudio
    );
    assert_eq!(
        app.state.simulation.active_run().map(|run| run.dataset_id),
        Some(candidate_id)
    );
    assert_eq!(
        app.state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.id),
        Some(17)
    );
    let owner = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .expect("comparison handoff must activate an exact owner pane");
    assert_eq!(owner.dataset_id, candidate_id);
    assert_eq!(owner.analysis_sequence, 17);
    assert_eq!(
        app.state
            .workbench
            .visualization_studio
            .draft_comparison_dataset,
        Some(baseline_id)
    );
    assert_eq!(
        app.state.workbench.visualization_studio.dock,
        Some(VisualizationDock::Comparison)
    );
}

#[test]
fn results_comparison_fails_closed_before_navigation_without_a_compatible_baseline() {
    let mut app = app_with_exact_source();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.workspace = Workspace::Results;
    let candidate_id = app.state.simulation.runs[0].dataset_id;
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(candidate_id));
    let route_before = app.state.workbench.current_route();

    let mut incompatible = SimulationRun::new(2);
    incompatible.add_analysis(
        AnalysisResult::new(29, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            WaveformData::new(
                "V(other)",
                vec![0.0, 0.25, 1.0],
                vec![0.0, 1.0, 0.0],
                "#00aaff",
            ),
        ]),
    );
    app.state.simulation.runs.push(incompatible);

    assert!(!results_comparison_available(&app.state));
    let error = open_results_comparison_inner(&mut app)
        .expect_err("an incompatible retained run must not open a comparison owner");
    assert!(error.contains("second compatible immutable dataset"));
    assert_eq!(app.state.workbench.current_route(), route_before);
    assert_eq!(app.state.workbench.visualization_studio.dock, None);
}

/// What "feature complete" means for the Results workspace, stated as a
/// gate rather than a claim: a sheet answers for every view the product
/// manifest puts in this release's scope. Publish a release-target row with
/// no sheet behind it and this fails.
#[test]
fn every_release_target_viewer_document_ships_a_renderer() {
    use crate::results::viewer_catalog::ViewerReleaseClass;

    let mut in_scope = 0;
    for definition in VIEWER_DOCUMENTS {
        if definition.release != ViewerReleaseClass::ReleaseTarget {
            continue;
        }
        in_scope += 1;
        assert!(
            ResultViewer::from_viewer_document_id(definition.id).is_some(),
            "{} is in this release's scope but no sheet draws it",
            definition.id
        );
    }
    assert!(
        in_scope > 0,
        "the catalog declares no view in release scope"
    );
}

/// The viewer library publishes every designed view, and most of them this
/// build does not draw. The row a reader cannot click has to say which kind
/// of unavailable it is: "requires X analysis data" is an instruction they
/// can act on, so a view no sheet draws must not borrow that phrasing — it
/// reports the manifest's own release scope instead.
#[test]
fn a_view_no_sheet_draws_reports_its_release_scope_not_a_data_requirement() {
    use sections::resolved_viewer_availability;

    let app = app_with_exact_source();
    let analysis_ids = available_analysis_ids(&app.state);
    let mut drawable = 0;
    for definition in VIEWER_DOCUMENTS {
        let availability = resolved_viewer_availability(
            &app.state,
            definition,
            ViewerCapabilities {
                analysis_ids: &analysis_ids,
                external_capabilities: &[],
            },
        );
        let ships_a_renderer = ResultViewer::from_viewer_document_id(definition.id).is_some()
            && definition.release
                == crate::results::viewer_catalog::ViewerReleaseClass::ReleaseTarget;
        drawable += usize::from(ships_a_renderer);
        match availability {
            Ok(_) => assert!(ships_a_renderer, "{} drew without a sheet", definition.id),
            Err(reason) => assert_eq!(
                reason == definition.unavailable_reason(),
                !ships_a_renderer,
                "{} reports the wrong kind of unavailable: {reason}",
                definition.id
            ),
        }
    }
    assert!(
        drawable > 0 && drawable < VIEWER_DOCUMENTS.len(),
        "the catalog and the shipped renderers have stopped disagreeing; \
         fold this expectation into whichever is now the whole truth"
    );
}

/// Three sheets render `viewer-table`, so a retained pane's sheet cannot be
/// recovered from its viewer document — only checked against it. Reading
/// that check backwards would reject every retained Specs and OP pane as
/// having no renderer, because the inverse can only name one of the three.
#[test]
fn a_retained_pane_sharing_a_viewer_document_still_validates() {
    let mut app = app_with_exact_source();
    app.state.simulation.runs[0].analyses[0].measurements =
        vec![rspice_core::MeasureResult::success("V(out)", 4.0)];
    reconcile_document(&mut app);
    let panes = &mut app.state.workbench.visualization_studio.panes;
    let pane = panes
        .first_mut()
        .expect("the reconciled document has a pane");
    pane.viewer = ResultViewer::Specs;
    pane.viewer_document_id = ResultViewer::Specs
        .viewer_document_id()
        .expect("Specs renders a catalog viewer document")
        .to_owned();
    assert_eq!(pane.viewer_document_id, "viewer-table");
    assert_ne!(
        ResultViewer::from_viewer_document_id("viewer-table"),
        Some(ResultViewer::Specs),
        "this test is only meaningful while the inverse names a different sheet"
    );

    visualization_configuration_status(&app.state)
        .expect("a retained Specs pane names viewer-table truthfully");
}

#[test]
fn configuration_status_fails_closed_when_a_retained_binding_disappears() {
    let mut app = app_with_exact_source();
    reconcile_document(&mut app);
    visualization_configuration_status(&app.state)
        .expect("a compatible retained pane binding must validate");

    app.state.simulation.runs.clear();

    assert!(
        visualization_configuration_status(&app.state)
            .unwrap_err()
            .contains("unavailable dataset")
    );
}

fn append_retained_pole_zero_run(app: &mut RSpiceApp) -> (DatasetId, u64) {
    let analysis_sequence = 29;
    let analysis = AnalysisResult::new(analysis_sequence, AnalysisType::PoleZero, "PZ 29")
        .with_result_payload(AnalysisResultPayload::PoleZero {
            poles: vec![
                crate::state::ComplexResultValue {
                    real: -10.0,
                    imaginary: 20.0,
                },
                crate::state::ComplexResultValue {
                    real: -10.0,
                    imaginary: -20.0,
                },
            ],
            zeros: vec![crate::state::ComplexResultValue {
                real: -3.0,
                imaginary: 0.0,
            }],
            pole_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: crate::state::PoleZeroRootSetEvidence::LegacyUnknown,
            gain: Some(4.25),
        });
    let mut run = SimulationRun::new(2);
    run.add_analysis(analysis);
    let dataset_id = run.dataset_id;
    app.state.simulation.runs.push(run);
    (dataset_id, analysis_sequence)
}

fn append_retained_sensitivity_run(app: &mut RSpiceApp) -> (DatasetId, u64) {
    let analysis_sequence = 31;
    let analysis = AnalysisResult::new(analysis_sequence, AnalysisType::Sensitivity, "SENS 31")
        .with_result_payload(AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Ac {
                frequency_hz: 2.5e6,
            },
            rows: vec![
                SensitivityResultRow {
                    parameter: "c1".to_owned(),
                    raw: -1.25e3,
                    normalized: -0.75,
                },
                SensitivityResultRow {
                    parameter: "r1".to_owned(),
                    raw: 4.5e-3,
                    normalized: 0.25,
                },
            ],
        });
    let mut run = SimulationRun::new(3);
    run.add_analysis(analysis);
    let dataset_id = run.dataset_id;
    app.state.simulation.runs.push(run);
    (dataset_id, analysis_sequence)
}

#[test]
fn table_binding_accepts_payload_only_and_zero_mode_periodic_results() {
    let mut app = app_with_exact_source();
    let definition = viewer_document("viewer-table").expect("registered table viewer");

    for (run_sequence, analysis_type) in [(40, AnalysisType::Pss), (41, AnalysisType::Pstb)] {
        let analysis_sequence = run_sequence + 100;
        let analysis = AnalysisResult::new(analysis_sequence, analysis_type, "Periodic")
            .with_result_payload(
                AnalysisResultPayload::legacy_periodic_marker(analysis_type)
                    .expect("periodic payload marker"),
            );
        assert!(analysis.waveforms.is_empty());
        let mut run = SimulationRun::new(run_sequence);
        run.add_analysis(analysis);
        let dataset_id = run.dataset_id;
        app.state.simulation.runs.push(run);

        assert_eq!(
            resolved_viewer_availability_for_binding(
                &app.state,
                definition,
                Some(dataset_id),
                Some(analysis_sequence),
            ),
            Ok(ResultViewer::Table)
        );
    }
}

#[test]
fn historical_sensitivity_binding_uses_its_retained_payload() {
    let mut app = app_with_exact_source();
    let (dataset_id, analysis_sequence) = append_retained_sensitivity_run(&mut app);
    assert_eq!(app.state.simulation.active_run_idx, Some(0));

    let definition =
        viewer_document("viewer-contribution").expect("registered contribution viewer");
    assert_eq!(
        resolved_viewer_availability_for_binding(
            &app.state,
            definition,
            Some(dataset_id),
            Some(analysis_sequence),
        ),
        Ok(ResultViewer::Contribution)
    );
}

#[test]
fn sensitivity_exact_data_rows_preserve_parameter_values_output_and_basis() {
    let mut app = app_with_exact_source();
    append_retained_sensitivity_run(&mut app);
    assert!(app.state.simulation.select_run(1));

    let rows = exact_source_rows(&app.state);
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].stable_row, "31:sensitivity[0].raw");
    assert_eq!(rows[0].value, format!("{:.17e}", -1.25e3));
    assert_eq!(rows[0].origin, "V(out)");
    assert!(rows[0].coordinate.contains("parameter=c1"));
    assert!(rows[0].coordinate.contains("ac@2.50000000000000000e6Hz"));
    assert_eq!(rows[1].stable_row, "31:sensitivity[0].normalized");
    assert_eq!(rows[1].value, format!("{:.17e}", -0.75));
    assert_eq!(rows[2].stable_row, "31:sensitivity[1].raw");
    assert_eq!(rows[3].stable_row, "31:sensitivity[1].normalized");
}

#[test]
fn historical_pole_zero_binding_uses_its_retained_payload_without_derived_state() {
    let mut app = app_with_exact_source();
    let (dataset_id, analysis_sequence) = append_retained_pole_zero_run(&mut app);
    assert_eq!(app.state.simulation.active_run_idx, Some(0));
    assert!(app.state.analysis.pole_zero_state.is_empty());

    let definition = viewer_document("viewer-pz").expect("registered PZ viewer");
    assert_eq!(
        resolved_viewer_availability_for_binding(
            &app.state,
            definition,
            Some(dataset_id),
            Some(analysis_sequence),
        ),
        Ok(ResultViewer::PoleZero)
    );
}

#[test]
fn pole_zero_exact_data_rows_preserve_root_order_components_and_gain() {
    let mut app = app_with_exact_source();
    let (_, _) = append_retained_pole_zero_run(&mut app);
    assert!(app.state.simulation.select_run(1));

    let rows = exact_source_rows(&app.state);
    assert_eq!(rows.len(), 9);
    assert_eq!(rows[0].stable_row, "29:gain");
    assert_eq!(rows[0].value, format!("{:.17e}", 4.25));
    assert_eq!(rows[1].stable_row, "29:pole_evidence.status");
    assert_eq!(rows[1].value, "legacy unknown");
    assert_eq!(rows[2].stable_row, "29:zero_evidence.status");
    assert_eq!(rows[2].value, "legacy unknown");
    assert_eq!(rows[3].stable_row, "29:pole[0].real");
    assert_eq!(rows[3].value, format!("{:.17e}", -10.0));
    assert_eq!(rows[4].stable_row, "29:pole[0].imaginary");
    assert_eq!(rows[4].value, format!("{:.17e}", 20.0));
    assert_eq!(rows[5].stable_row, "29:pole[1].real");
    assert_eq!(rows[6].stable_row, "29:pole[1].imaginary");
    assert_eq!(rows[7].stable_row, "29:zero[0].real");
    assert_eq!(rows[8].stable_row, "29:zero[0].imaginary");
}

#[test]
fn link_groups_apply_the_same_exact_x_range_and_cursor_pair() {
    let mut app = app_with_exact_source();
    reconcile_document(&mut app);
    let analysis_key = activate_voltage_wave_pane(&mut app);
    let first = app
        .state
        .workbench
        .visualization_studio
        .active_pane()
        .cloned()
        .expect("reconciliation must create the first pane");
    app.state
        .ui
        .results
        .analysis_plot_view_pane_mut(first.viewer, analysis_key, 0)
        .x = Some((0.25, 0.75));
    app.state.ui.results.cursors.a = Some(0.3);
    app.state.ui.results.cursors.b = Some(0.7);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    capture_active_link_state(&ctx, &mut app);

    let mut second = first.clone();
    second.id = first.id + 1;
    app.state
        .workbench
        .visualization_studio
        .panes
        .push(second.clone());
    app.state.workbench.visualization_studio.next_identity = second.id + 1;
    app.state.workbench.visualization_studio.active_pane = Some(second.id);
    app.state.workbench.visualization_studio.applied_link_pane = None;
    app.state
        .ui
        .results
        .analysis_plot_view_pane_mut(second.viewer, analysis_key, 0)
        .x = None;
    app.state.ui.results.cursors.clear();

    apply_active_link_state(&mut app);
    apply_queued_view_gesture(&ctx, &mut app);

    assert_eq!(
        app.state
            .ui
            .results
            .analysis_plot_view_pane(second.viewer, analysis_key, 0)
            .x,
        Some((0.25, 0.75))
    );
    assert_eq!(app.state.ui.results.cursors.a, Some(0.3));
    assert_eq!(app.state.ui.results.cursors.b, Some(0.7));
}

#[test]
fn add_pane_binds_the_requested_retained_analysis_and_selects_its_run() {
    let mut app = app_with_exact_source();
    reconcile_document(&mut app);

    let historical_analysis =
        AnalysisResult::new(23, AnalysisType::Transient, "TRAN 23").with_waveforms(vec![
            WaveformData::new("V(history)", vec![0.0, 1.0], vec![0.0, 1.0], "#55ddaa"),
        ]);
    let mut historical_run = SimulationRun::new(2);
    historical_run.add_analysis(historical_analysis);
    let historical_dataset = historical_run.dataset_id;
    app.state.simulation.runs.push(historical_run);

    add_viewer_pane_bound(
        &mut app,
        "viewer-waveform",
        ResultViewer::Waves,
        historical_dataset,
        23,
        VisualizationPanePlacement::RightOfSelected,
        String::new(),
    );

    let studio = &app.state.workbench.visualization_studio;
    let pane = studio.active_pane().expect("new pane becomes active");
    assert_eq!(pane.dataset_id, historical_dataset);
    assert_eq!(pane.analysis_sequence, 23);
    assert_eq!(pane.placement, VisualizationPanePlacement::RightOfSelected);
    assert_eq!(pane.page, "Engineering");
    assert_eq!(pane.x_link, Some(1));
    assert_eq!(pane.cursor_group, Some(1));
    assert_eq!(app.state.simulation.active_run_idx, Some(1));
    assert_eq!(app.state.simulation.active_analysis_idx, Some(0));
}

#[test]
fn new_page_pane_is_unlinked_and_commits_as_one_valid_transaction() {
    let mut app = app_with_exact_source();
    reconcile_document(&mut app);
    let binding = app
        .state
        .simulation
        .active_run()
        .and_then(|run| {
            app.state
                .simulation
                .active_analysis()
                .map(|analysis| (run.dataset_id, analysis.id))
        })
        .expect("fixture has an active immutable binding");
    let before_revision = app.state.workbench.visualization_studio.revision;

    add_viewer_pane_bound(
        &mut app,
        "viewer-waveform",
        ResultViewer::Waves,
        binding.0,
        binding.1,
        VisualizationPanePlacement::NewWorksheetPage,
        "Statistics".to_owned(),
    );

    let studio = &app.state.workbench.visualization_studio;
    assert_eq!(studio.revision, before_revision + 1);
    assert_eq!(studio.panes.len(), 2);
    let pane = studio.active_pane().expect("new page pane becomes active");
    assert_eq!(pane.page, "Statistics");
    assert_eq!(pane.placement, VisualizationPanePlacement::NewWorksheetPage);
    assert_eq!(pane.x_link, None);
    assert_eq!(pane.cursor_group, None);
    studio
        .validate_presentation()
        .expect("the aggregate pane edit remains valid");
}

#[test]
fn unavailable_add_pane_binding_leaves_the_document_unchanged() {
    let mut app = app_with_exact_source();
    reconcile_document(&mut app);
    let before = app.state.workbench.visualization_studio.clone();

    add_viewer_pane_bound(
        &mut app,
        "viewer-waveform",
        ResultViewer::Waves,
        DatasetId::new(),
        17,
        VisualizationPanePlacement::BelowSelected,
        String::new(),
    );

    assert_eq!(app.state.workbench.visualization_studio, before);
}

#[test]
fn versioned_entity_projection_retains_exact_bindings_and_stable_identities() {
    let mut app = app_with_exact_source();
    let dataset_id = app
        .state
        .simulation
        .active_run()
        .expect("fixture retains an active run")
        .dataset_id;
    app.state.ui.results.cursors.a = Some(0.5);
    app.state
        .workbench
        .visualization_studio
        .markers
        .push(VisualizationMarker {
            id: 31,
            dataset_id,
            analysis_sequence: 17,
            waveform_name: "V(out)".to_owned(),
            sample_index: 1,
            x: 0.5,
            y: 2.5,
            label: "source sample".to_owned(),
        });
    app.state
        .workbench
        .visualization_studio
        .measurements
        .push(VisualizationMeasurement {
            id: 32,
            dataset_id,
            analysis_sequence: 17,
            expression: "rms(V(out))".to_owned(),
            value: 2.0,
        });
    app.state
        .workbench
        .visualization_studio
        .annotations
        .push(VisualizationAnnotation {
            id: 33,
            dataset_id,
            analysis_sequence: 17,
            x: 0.5,
            text: "review exact point".to_owned(),
        });

    let rows = result_entity_rows(&app.state);
    assert_eq!(
        rows.iter().map(|row| row.kind).collect::<Vec<_>>(),
        [
            "axis",
            "trace",
            "trace",
            "cursor",
            "marker",
            "measurement",
            "annotation",
        ]
    );
    assert_eq!(rows[0].identity, "axis:17:x");
    assert_eq!(rows[1].identity, "trace:17:0");
    assert!(rows[1].binding.contains("V(out)"));
    assert_eq!(rows[3].identity, "cursor:A");
    assert_eq!(rows[4].identity, "marker:31");
    assert_eq!(rows[5].identity, "measurement:32");
    assert_eq!(rows[6].identity, "annotation:33");
    let dataset_prefix = short_dataset(dataset_id);
    assert!(rows[1].binding.starts_with(&dataset_prefix));
    assert!(rows[4].binding.starts_with(&dataset_prefix));
    assert!(rows[6].binding.starts_with(&dataset_prefix));
}

#[test]
fn viewer_columns_preserve_the_mockup_side_widths_exactly() {
    assert_eq!(visible_available_width(1_312.0, 50.0, 1_280.0), 1_230.0);
    let desktop = Rect::from_min_size(egui::Pos2::ZERO, vec2(1_230.0, 540.0));
    let [library, stage, inspector] = viewer_column_rects(desktop, 190.0, 224.0);
    assert_eq!(library.width(), 190.0);
    assert_eq!(inspector.width(), 224.0);
    assert_eq!(stage.width(), 814.0);
    assert_eq!(stage.left() - library.right(), 1.0);
    assert_eq!(inspector.left() - stage.right(), 1.0);

    let tablet = Rect::from_min_size(egui::Pos2::ZERO, vec2(900.0, 430.0));
    let [library, stage, inspector] = viewer_column_rects(tablet, 158.0, 196.0);
    assert_eq!(library.width(), 158.0);
    assert_eq!(inspector.width(), 196.0);
    assert_eq!(stage.width(), 544.0);
}

#[test]
fn compact_dialog_body_stays_inside_phone_and_tablet_frames() {
    assert_eq!(compact_dock_geometry(390.0), (372.0, 348.0));
    assert_eq!(compact_dock_geometry(800.0), (520.0, 496.0));
    let (window, body) = compact_dock_geometry(180.0);
    assert!(window <= 180.0);
    assert!(body < window);
}
