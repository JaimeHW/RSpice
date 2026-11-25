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
    run.lifecycle = SimulationRunLifecycle::Completed;
    assert!(matches!(
        RetainedQuickViewSource::try_new(&run, 8, |_| true),
        Err(HardcopySourceError::UnretainedResult(_))
    ));
    let source = RetainedQuickViewSource::try_new(&run, 7, |_| true).unwrap();
    // Schema 8 controls written before the service extraction. Optional overlay
    // and histogram viewport/measurement fields retain their original defaults.
    let legacy = r#"{
        "viewer": "Waves", "fft_selected_source": null,
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

/// The S-parameter sheet prints each coefficient it draws in dB, as ADS
/// prints dB(S(i,j)), and leaves the phase and noise columns off its one
/// ordinate.
#[test]
fn the_s_parameter_sheet_prints_its_coefficients_in_decibels() {
    let coefficient = |name: &str, magnitude: [f64; 2]| {
        RetainedWaveform::new(format!("|{name}|"), vec![1.0e9, 2.0e9], magnitude.to_vec())
            .with_complex_components(name, magnitude.to_vec(), vec![0.0, 0.0])
    };
    let analysis =
        AnalysisResult::new(3, AnalysisType::SParameter, "SP", 0.0).with_waveforms(vec![
            coefficient("S11", [0.5, 0.25]),
            coefficient("S21", [10.0, 1.0]),
            RetainedWaveform::new("phase(S21)", vec![1.0e9, 2.0e9], vec![0.0, 0.0]),
            RetainedWaveform::new("F", vec![1.0e9, 2.0e9], vec![1.5, 1.6]),
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
    run.lifecycle = SimulationRunLifecycle::Completed;
    let source = RetainedQuickViewSource::try_new(&run, 3, |_| true).unwrap();
    let plot = quick_network_plot(&source, &RetainedQuickViewOverlay::default()).unwrap();
    let labels = plot
        .traces
        .iter()
        .map(|trace| trace.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(labels, ["|S11| (dB)", "|S21| (dB)"]);
    let decibels = |trace: &SemanticPlotTrace| {
        trace
            .source_samples
            .iter()
            .map(|(_, y)| f64::from_bits(*y))
            .collect::<Vec<_>>()
    };
    let s21 = decibels(&plot.traces[1]);
    assert!(
        (s21[0] - 20.0).abs() < 1e-12 && s21[1].abs() < 1e-12,
        "{s21:?}"
    );
    let s11 = decibels(&plot.traces[0]);
    assert!((s11[0] - 20.0 * 0.5f64.log10()).abs() < 1e-12, "{s11:?}");
}

/// A completed run of one analysis.
fn completed(analysis: AnalysisResult) -> SimulationRun {
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
    run.lifecycle = SimulationRunLifecycle::Completed;
    run
}

/// The words a printed plot states its axes in: the abscissa's caption, and
/// for each strip, top to bottom, the value on its top tick.
fn stated_axes(plot: &SemanticPlot) -> (Vec<String>, Vec<String>) {
    let captions = plot
        .captions
        .iter()
        .map(|caption| caption.text.clone())
        .collect();
    let tops = plot
        .frames
        .iter()
        .map(|frame| {
            plot.axis_ticks
                .iter()
                .filter(|tick| {
                    tick.axis == SemanticAxisKind::Vertical
                        && !tick.label.is_empty()
                        && (frame.minimum.y_um..=frame.maximum.y_um).contains(&tick.start.y_um)
                })
                .min_by_key(|tick| tick.start.y_um)
                .map(|tick| tick.label.clone())
                .unwrap_or_default()
        })
        .collect();
    (captions, tops)
}

/// A printed sheet says what it plots: the abscissa's quantity and unit in
/// its row's gutter, and each strip's unit on its top value. A current and a
/// voltage get a strip each, so a milliamp is not a flat line on a volt axis.
#[test]
fn a_printed_plot_states_its_axes_and_gives_each_unit_a_strip() {
    let time = vec![0.0, 1.0e-3, 2.0e-3];
    let run = completed(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0).with_waveforms(vec![
            RetainedWaveform::new("V(out)", time.clone(), vec![0.0, 1.0, 2.0]),
            RetainedWaveform::new("I(R1)", time, vec![0.0, 1.0e-3, 2.0e-3]),
        ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 1, |_| true).unwrap();
    let plot = quick_waveform_plot(
        &source,
        ResultViewer::Waves,
        &RetainedQuickViewOverlay::default(),
    )
    .unwrap();
    let (captions, tops) = stated_axes(&plot);
    assert!(
        captions.iter().any(|caption| caption == "TIME \u{b7} s"),
        "{captions:?}"
    );
    assert_eq!(plot.frames.len(), 2, "a volt strip and an amp strip");
    assert!(tops[0].ends_with('V'), "{tops:?}");
    assert!(tops[1].ends_with("mA"), "{tops:?}");
    let in_frame = |trace: &SemanticPlotTrace, frame: &SemanticPlotFrame| {
        trace
            .paths
            .iter()
            .flatten()
            .all(|point| (frame.minimum.y_um..=frame.maximum.y_um).contains(&point.y_um))
    };
    assert!(in_frame(&plot.traces[0], &plot.frames[0]));
    assert!(in_frame(&plot.traces[1], &plot.frames[1]));
}

/// The Bode page prints magnitude and phase each on its own axis, on a
/// decade abscissa it names.
#[test]
fn a_printed_bode_gives_magnitude_and_phase_their_own_axes() {
    let frequency = vec![1.0, 10.0, 100.0, 1_000.0];
    let run = completed(
        AnalysisResult::new(2, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("|V(out)|", frequency.clone(), vec![10.0, 9.0, 1.0, 0.1]),
            RetainedWaveform::new("phase(V(out))", frequency, vec![0.0, -10.0, -90.0, -170.0]),
        ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 2, |_| true).unwrap();
    let plot = quick_bode_plot(&source, &RetainedQuickViewOverlay::default(), None).unwrap();
    let (captions, tops) = stated_axes(&plot);
    assert!(
        captions.iter().any(|caption| caption == "FREQ \u{b7} Hz"),
        "{captions:?}"
    );
    assert_eq!(plot.frames.len(), 2);
    assert!(tops[0].ends_with("dB"), "{tops:?}");
    assert!(tops[1].ends_with('°'), "{tops:?}");
    assert_eq!(plot.x_scale, AxisScale::Logarithmic);
}

/// A spectrum prints with the window the sheet took it with, a Kaiser
/// window's own β included, and states it with the record and resolution.
#[test]
fn a_printed_spectrum_keeps_and_states_the_sheets_window() {
    use rspice_results::fft::window::{KaiserBeta, WindowFunction};
    let time = (0..4_096)
        .map(|index| f64::from(index) * 1.0e-6)
        .collect::<Vec<_>>();
    let values = time.iter().map(|t| (t * 6.0e4).sin()).collect::<Vec<_>>();
    let run = completed(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0)
            .with_waveforms(vec![RetainedWaveform::new("V(out)", time, values)]),
    );
    let presentation = ResultsQuickViewPresentation::try_new(
        ResultViewer::Fft,
        RetainedQuickViewOverlays::default(),
        QuickFftSettings {
            selected_source: Some("V(out)".to_owned()),
            normalization: rspice_results::fft::data::SpectrumNormalization::Peak,
            window: WindowFunction::Kaiser(KaiserBeta::new(12.0).unwrap()),
            input_fidelity: rspice_results::fft::InputFidelity::Reference,
            time_window_auto: true,
            time_window_start: 0.0,
            time_window_end: 1.0,
            sample_count_auto: true,
            sample_count: 4096,
        },
        QuickHistogramSettings {
            x: None,
            y: None,
            selected: 0,
            measurement: None,
            bin_count: 20,
            custom_range: false,
            custom_min: 0.0,
            custom_max: 1.0,
            mode: rspice_results::histogram::HistogramDisplayMode::Count,
        },
    )
    .unwrap();
    let restored: ResultsQuickViewPresentation =
        serde_json::from_slice(&serde_json::to_vec(&presentation).unwrap()).unwrap();
    assert_eq!(
        restored.fft_window(),
        WindowFunction::Kaiser(KaiserBeta::new(12.0).unwrap())
    );
    let source = RetainedQuickViewSource::try_new(&run, 1, |_| true).unwrap();
    let plot = quick_fft_plot(&restored, &source).unwrap();
    let settings = plot
        .captions
        .iter()
        .find(|caption| caption.text.starts_with("Kaiser"))
        .map(|caption| caption.text.clone())
        .unwrap_or_default();
    assert!(
        settings.starts_with("Kaiser \u{3b2} 12 \u{b7} peak \u{b7} ")
            && settings.contains(" samples \u{b7} RBW "),
        "{settings}"
    );
}

/// The Bode page prints the response the card measures: the one the reader
/// chose, not the first the analysis retained.
#[test]
fn a_printed_bode_draws_the_response_the_card_measures() {
    let frequency = vec![1.0, 10.0, 100.0, 1_000.0];
    let run = completed(
        AnalysisResult::new(2, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("|V(in)|", frequency.clone(), vec![1.0; 4]),
            RetainedWaveform::new("phase(V(in))", frequency.clone(), vec![0.0; 4]),
            RetainedWaveform::new("|V(out)|", frequency.clone(), vec![10.0, 9.0, 1.0, 0.1]),
            RetainedWaveform::new("phase(V(out))", frequency, vec![0.0, -10.0, -90.0, -170.0]),
        ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 2, |_| true).unwrap();
    for (chosen, printed) in [("V(in)", "|V(in)| (dB)"), ("V(out)", "|V(out)| (dB)")] {
        let plot =
            quick_bode_plot(&source, &RetainedQuickViewOverlay::default(), Some(chosen)).unwrap();
        assert_eq!(plot.traces[0].label, printed);
    }
}
