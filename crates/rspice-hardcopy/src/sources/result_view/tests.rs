use super::*;
use rspice_results::run::SimulationRunLifecycle;

#[test]
fn quick_view_binding_requires_one_successful_terminal_analysis_and_valid_controls() {
    let analysis =
        AnalysisResult::new(7, AnalysisType::Transient, "Transient", 0.0).with_waveforms(vec![
            RetainedWaveform::new("V(out)", vec![0.0, 1.0], vec![2.0, 3.0]),
        ]);
    let mut run: SimulationRun = SimulationRun::from_persisted_identity(
        1,
        "Run 1".to_owned(),
        0.0,
        RunId::new(),
        DatasetId::new(),
        None,
        None,
    );
    run.analyses.push(analysis);
    assert!(matches!(
        RetainedQuickViewSource::try_new(&run, 7, |_| true),
        Err(HardcopySourceError::UnretainedResult(_))
    ));
    assert!(matches!(
        resolve_results_manifest_source(
            "manifest".to_owned(),
            ProjectId::new(),
            HardcopyScope::ActiveDocument,
            &run
        ),
        Err(HardcopySourceError::UnretainedResult(_))
    ));
    assert!(matches!(
        resolve_results_specs_source(
            "specifications".to_owned(),
            ProjectId::new(),
            HardcopyScope::ActiveDocument,
            &run,
            &[]
        ),
        Err(HardcopySourceError::UnretainedResult(_))
    ));
    run.lifecycle = SimulationRunLifecycle::Completed;
    assert!(matches!(
        RetainedQuickViewSource::try_new(&run, 8, |_| true),
        Err(HardcopySourceError::UnretainedResult(_))
    ));
    let source = RetainedQuickViewSource::try_new(&run, 7, |_| true).unwrap();
    // Schema 8 controls written before the service extraction. Optional overlay
    // and histogram viewport/measurement fields retain their original defaults.
    let legacy = r#"{
        "viewer": "Waves", "specs": [], "fft_selected_source": null,
        "fft_normalization": "peak", "fft_window": "hanning",
        "fft_input_fidelity": "reference", "fft_time_window_auto": true,
        "fft_time_window_start": 0.0, "fft_time_window_end": 1.0,
        "fft_sample_count_auto": true, "fft_sample_count": 4096,
        "histogram_selected": 0, "histogram_bin_count": 20,
        "histogram_custom_range": false, "histogram_custom_min": 0.0,
        "histogram_custom_max": 1.0, "histogram_mode": "count"
    }"#;
    let presentation: ResultsQuickViewPresentation = serde_json::from_str(legacy).unwrap();
    assert!(matches!(
        resolve_results_quick_view_stack(
            "stack".to_owned(),
            ProjectId::new(),
            HardcopyScope::Selection,
            &run,
            &presentation,
            |_| true
        ),
        Err(HardcopySourceError::UnsupportedScope(
            HardcopyScope::Selection
        ))
    ));
    let project = ProjectId::new();
    let resolved = resolve_results_quick_view_parts(
        "retained-wave".to_owned(),
        project,
        HardcopyScope::ActivePlotDocument,
        &source,
        &presentation,
    )
    .unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected plot")
    };
    assert_eq!(
        plot.traces[0].source_samples,
        vec![
            (0.0f64.to_bits(), 2.0f64.to_bits()),
            (1.0f64.to_bits(), 3.0f64.to_bits())
        ]
    );
    let mut encoded = serde_json::to_value(&presentation).unwrap();
    encoded["histogram_x"] = serde_json::json!([2.0, 1.0]);
    let invalid = serde_json::from_value(encoded).unwrap();
    assert!(matches!(
        resolve_results_quick_view_parts(
            "retained-wave".to_owned(),
            project,
            HardcopyScope::ActivePlotDocument,
            &source,
            &invalid
        ),
        Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
    ));

    run.analyses[0].success = false;
    assert!(matches!(
        RetainedQuickViewSource::try_new(&run, 7, |_| true),
        Err(HardcopySourceError::UnretainedResult(_))
    ));
    run.analyses[0].success = true;
    run.analyses.push(run.analyses[0].clone());
    assert!(matches!(
        RetainedQuickViewSource::try_new(&run, 7, |_| true),
        Err(HardcopySourceError::AmbiguousRetainedAnalysis(7))
    ));
    run.analyses.pop();
    run.analyses[0].waveforms[0] = RetainedWaveform::new("V(out)", vec![0.0, 1.0], vec![2.0]);
    assert!(matches!(
        RetainedQuickViewSource::try_new(&run, 7, |_| true),
        Err(HardcopySourceError::InvalidVisualizationSource(_))
    ));
}
