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
        // The key names each trace as the sheet's legend does; the unit
        // is on its axis.
        let keys = plot
            .traces
            .iter()
            .map(|trace| trace.key_label())
            .collect::<Vec<_>>();
        assert_eq!(keys, [format!("|{chosen}|"), format!("phase({chosen})")]);
    }
}

/// The print mapping, which the printed key is drawn from, names a Bode
/// trace as the sheet's legend does, its unit left to the axis.
#[test]
fn a_printed_bode_key_names_each_trace_as_the_sheet() {
    let frequency = vec![1.0, 10.0, 100.0, 1_000.0];
    let run = completed(
        AnalysisResult::new(2, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("|V(out)|", frequency.clone(), vec![10.0, 9.0, 1.0, 0.1]),
            RetainedWaveform::new("phase(V(out))", frequency, vec![0.0, -10.0, -90.0, -170.0]),
        ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 2, |_| true).unwrap();
    let presentation: ResultsQuickViewPresentation = serde_json::from_str(
        r#"{
        "viewer": "Bode", "fft_selected_source": null,
        "fft_normalization": "peak", "fft_window": "hanning",
        "fft_input_fidelity": "reference", "fft_time_window_auto": true,
        "fft_time_window_start": 0.0, "fft_time_window_end": 1.0,
        "fft_sample_count_auto": true, "fft_sample_count": 4096,
        "histogram_selected": 0, "histogram_bin_count": 20,
        "histogram_custom_range": false, "histogram_custom_min": 0.0,
        "histogram_custom_max": 1.0, "histogram_mode": "count"
    }"#,
    )
    .unwrap();
    let resolved = resolve_results_quick_view_parts(
        "bode".to_owned(),
        ProjectId::new(),
        HardcopyScope::ActivePlotDocument,
        &source,
        &presentation,
    )
    .unwrap();
    let keys = resolved
        .default_print_mapping()
        .entries()
        .iter()
        .filter(|entry| entry.object().kind() == rspice_hardcopy_contract::PrintObjectKind::Trace)
        .map(|entry| entry.object().display_name())
        .collect::<Vec<_>>();
    assert_eq!(keys, ["|V(out)|", "phase(V(out))"]);
}

/// The polar page reads as the sheet: angles signed about 0°, and five
/// rings to the sheet's 1-2-5 rim, each named up the 90° spoke.
#[test]
fn a_printed_polar_reads_its_angles_and_rings_as_the_sheet() {
    let frequency = vec![1.0, 10.0, 100.0];
    let run = completed(
        AnalysisResult::new(2, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("V(out)", frequency, vec![7.3, 5.0, 1.0])
                .with_complex_components("V(out)", vec![7.3, 0.0, -1.0], vec![0.0, 5.0, 0.0]),
        ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 2, |_| true).unwrap();
    let plot = quick_polar_plot(&source).unwrap();
    let captions = plot
        .captions
        .iter()
        .map(|caption| caption.text.as_str())
        .collect::<Vec<_>>();
    for label in [
        "0\u{b0}",
        "90\u{b0}",
        "180\u{b0}",
        "\u{2212}90\u{b0}",
        "\u{2212}150\u{b0}",
    ] {
        assert!(captions.contains(&label), "{label} in {captions:?}");
    }
    assert!(!captions.contains(&"210\u{b0}"), "{captions:?}");
    for ring in ["2", "4", "6", "8", "10"] {
        assert!(captions.contains(&ring), "{ring} in {captions:?}");
    }
}

/// A stored series a page needs whole is asked of the reader service, as the
/// sheets ask, and is the series the store holds.
#[test]
fn a_stored_source_is_asked_of_the_reader_service() {
    use rspice_results::waveform::{
        ColumnSource, Read, ReadService, ReadValue, StoredWindow, WindowLead, perform, read_through,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Asked(AtomicUsize);
    impl ReadService for Asked {
        fn request(
            &self,
            _run: u64,
            values: &WaveformValues,
            x: &WaveformValues,
            read: Read,
        ) -> Option<ReadValue> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Some(perform(values, x, read))
        }
    }
    struct Column(Vec<f64>);
    impl ColumnSource for Column {
        fn name(&self) -> &str {
            "column"
        }
        fn store(&self) -> String {
            "run.rsr".to_owned()
        }
        fn rows(&self) -> u64 {
            self.0.len() as u64
        }
        fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
            let first = usize::try_from(first).map_err(|error| error.to_string())?;
            out.copy_from_slice(&self.0[first..first + out.len()]);
            Ok(())
        }
    }
    let stored = |values: &[f64]| {
        WaveformValues::stored(
            std::sync::Arc::new(Column(values.to_vec())),
            StoredWindow {
                first: 0,
                rows: values.len() as u64,
                lead: WindowLead::None,
            },
        )
        .unwrap()
    };
    static SERVICE: Asked = Asked(AtomicUsize::new(0));
    let (time, level) = ([0.0, 1.0, 2.0], [3.0, 4.0, 5.0]);
    let mut waveform = RetainedWaveform::new("V(out)", time.to_vec(), level.to_vec());
    waveform.x = stored(&time);
    waveform.y = stored(&level);
    let (x, y) = read_through(&SERVICE, || whole_pair(&waveform)).unwrap();
    assert_eq!((&x[..], &y[..]), (&time[..], &level[..]));
    assert_eq!(SERVICE.0.load(Ordering::SeqCst), 2);
}

/// A polar locus of a stored sweep is asked of the reader service a column
/// at a time, and no stored column is read on the thread that prints it.
#[test]
fn a_printed_polar_asks_the_reader_service_for_a_stored_locus() {
    use rspice_results::waveform::column_work::ColumnWork;
    use rspice_results::waveform::{
        ColumnSource, Read, ReadService, ReadValue, StoredWindow, WindowLead, perform, read_through,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Reads on a thread of its own, as the service does.
    struct Asked(AtomicUsize);
    impl ReadService for Asked {
        fn request(
            &self,
            _run: u64,
            values: &WaveformValues,
            x: &WaveformValues,
            read: Read,
        ) -> Option<ReadValue> {
            self.0.fetch_add(1, Ordering::SeqCst);
            std::thread::scope(|scope| scope.spawn(|| perform(values, x, read)).join().ok())
        }
    }
    struct Column(Vec<f64>);
    impl ColumnSource for Column {
        fn name(&self) -> &str {
            "column"
        }
        fn store(&self) -> String {
            "run.rsr".to_owned()
        }
        fn rows(&self) -> u64 {
            self.0.len() as u64
        }
        fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
            let first = usize::try_from(first).map_err(|error| error.to_string())?;
            out.copy_from_slice(&self.0[first..first + out.len()]);
            Ok(())
        }
    }
    let stored = |values: &[f64]| {
        WaveformValues::stored(
            std::sync::Arc::new(Column(values.to_vec())),
            StoredWindow {
                first: 0,
                rows: values.len() as u64,
                lead: WindowLead::None,
            },
        )
        .unwrap()
    };
    static SERVICE: Asked = Asked(AtomicUsize::new(0));
    let (real, imaginary) = ([7.3, 0.0, -1.0], [0.0, 5.0, 0.0]);
    let mut waveform = RetainedWaveform::new("V(out)", vec![1.0, 10.0, 100.0], vec![7.3, 5.0, 1.0])
        .with_complex_components("V(out)", real.to_vec(), imaginary.to_vec());
    let complex = waveform.complex.as_mut().unwrap();
    (complex.real, complex.imag) = (stored(&real), stored(&imaginary));
    let run = completed(
        AnalysisResult::new(2, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![waveform]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 2, |_| true).unwrap();
    let before = ColumnWork::read();
    let plot = read_through(&SERVICE, || quick_polar_plot(&source)).unwrap();
    let work = before.since();
    assert_eq!(
        (work.stored_views, work.stored_reads),
        (0, 0),
        "a stored column was read on the thread that prints"
    );
    assert_eq!(SERVICE.0.load(Ordering::SeqCst), 2);
    let printed = real
        .iter()
        .zip(&imaginary)
        .map(|(re, im)| (re.to_bits(), im.to_bits()))
        .collect::<Vec<_>>();
    assert_eq!(plot.traces[0].source_samples, printed);
}

/// A table sheet prints the tables it showed under the analysis's name,
/// and never tables read off another analysis.
#[test]
fn a_table_sheet_prints_the_tables_it_showed() {
    let run = completed(
        AnalysisResult::new(2, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(vec![
            RetainedWaveform::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0]),
        ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 2, |_| true).unwrap();
    let table = SemanticTable {
        title: "2 rows".to_owned(),
        columns: vec!["Row".to_owned(), "V(out)".to_owned()],
        rows: vec![
            vec!["0".to_owned(), "0.000 V".to_owned()],
            vec!["1".to_owned(), "1.000 V".to_owned()],
        ],
    };
    let shown = |analysis_id| QuickSheetReading {
        analysis_id,
        tables: vec![table.clone()],
        figures: Vec::new(),
    };
    let summary = sheet_summary(ResultViewer::Table, source.analysis, &shown(Some(2))).unwrap();
    assert_eq!(summary.title, "TRAN");
    assert_eq!(summary.tables, std::slice::from_ref(&table));
    for elsewhere in [shown(Some(3)), QuickSheetReading::default()] {
        assert!(matches!(
            sheet_summary(ResultViewer::Table, source.analysis, &elsewhere),
            Err(HardcopySourceError::MissingViewerEvidence(_))
        ));
    }
}

/// A table too long for one page prints as the print dialog plans it: a
/// page to a sheet.
#[test]
fn a_long_printed_table_prints_a_page_to_a_sheet() {
    use crate::render::{HardcopyRenderer, HardcopySceneMetadata};
    use rspice_hardcopy_contract::{
        BackgroundMode, Bleed, ColorMapping, DecorationSetup, FontPolicy, HardcopyPlan,
        HardcopySetup, Length, Orientation, OutputFormat, PageMargins, PaperSize,
        PhysicalPageSetup, RenderSetup, RenderTarget, ScaleMode, StandardPaper, TilingMode,
        TilingSetup, Watermark,
    };
    let run = completed(
        AnalysisResult::new(2, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(vec![
            RetainedWaveform::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0]),
        ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 2, |_| true).unwrap();
    let presentation: ResultsQuickViewPresentation = serde_json::from_str(
        r#"{
        "viewer": "Table", "fft_selected_source": null,
        "fft_normalization": "peak", "fft_window": "hanning",
        "fft_input_fidelity": "reference", "fft_time_window_auto": true,
        "fft_time_window_start": 0.0, "fft_time_window_end": 1.0,
        "fft_sample_count_auto": true, "fft_sample_count": 4096,
        "histogram_selected": 0, "histogram_bin_count": 20,
        "histogram_custom_range": false, "histogram_custom_min": 0.0,
        "histogram_custom_max": 1.0, "histogram_mode": "count"
    }"#,
    )
    .unwrap();
    let presentation = presentation
        .with_sheet_reading(QuickSheetReading {
            analysis_id: Some(2),
            tables: vec![SemanticTable {
                title: "200 rows".to_owned(),
                columns: vec!["Row".to_owned(), "V(out)".to_owned()],
                rows: (0..200)
                    .map(|row| vec![row.to_string(), format!("{row}.000 mV")])
                    .collect(),
            }],
            figures: Vec::new(),
        })
        .unwrap();
    let resolved = resolve_results_quick_view_parts(
        "table".to_owned(),
        ProjectId::new(),
        HardcopyScope::ActivePlotDocument,
        &source,
        &presentation,
    )
    .unwrap();
    let setup = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::Letter),
            PageMargins::uniform(Length::from_micrometres(10_000)),
            Bleed::None,
            Orientation::Landscape,
        )
        .unwrap(),
        ScaleMode::FitPrintableArea,
        TilingSetup::try_new(TilingMode::Automatic, Length::ZERO, true).unwrap(),
        RenderSetup::try_new(
            RenderTarget::ExportArtifact,
            OutputFormat::PdfVector,
            ColorMapping::PrintSafeEngineeringPalette,
            BackgroundMode::White,
            FontPolicy::new(true, true),
            true,
        )
        .unwrap(),
        DecorationSetup::try_new(true, true, true, Watermark::None).unwrap(),
        resolved.default_print_mapping().clone(),
    )
    .unwrap();
    let sections = resolved
        .hardcopy_sections_for_setup(setup.schematic())
        .unwrap();
    assert!(sections.len() > 1, "{} sections", sections.len());
    let extent = resolved
        .content_extent_for_setup(setup.schematic())
        .unwrap();
    let plan = HardcopyPlan::compile_with_sections(
        resolved.authority().clone(),
        setup,
        extent,
        sections.clone(),
    )
    .unwrap();
    let publication = HardcopyRenderer::render_resolved(
        &plan,
        &resolved,
        HardcopySceneMetadata::try_new("Table", "RSpice tests").unwrap(),
    )
    .unwrap();
    let pdf = lopdf::Document::load_mem(publication.single_part().unwrap().bytes()).unwrap();
    assert_eq!(pdf.get_pages().len(), sections.len());
}

/// The figures a card shows print on one line, and a name the card gives
/// twice running reads once, with both its readings.
#[test]
fn a_cards_figures_print_on_one_line() {
    let figures = |pairs: &[(&str, &str)]| {
        pairs
            .iter()
            .map(|(name, reading)| ((*name).to_owned(), (*reading).to_owned()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        fact_row(&figures(&[
            ("Fundamental", "1.000 kHz"),
            ("THD", "0.1230 %"),
            ("THD", "\u{2212}58.20 dB"),
        ]))
        .as_deref(),
        Some("Fundamental 1.000 kHz \u{b7} THD 0.1230 %, \u{2212}58.20 dB")
    );
    assert_eq!(
        fact_row(&figures(&[
            ("THD", "No harmonics"),
            ("THD", "No harmonics")
        ]))
        .as_deref(),
        Some("THD No harmonics")
    );
    assert_eq!(fact_row(&[]), None);
}
