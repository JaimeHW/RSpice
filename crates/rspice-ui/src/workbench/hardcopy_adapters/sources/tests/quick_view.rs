//! What a quick-view hardcopy derives, for the panes that derive rather than plot.
//!
//! A noise density, an FFT, an eye, a histogram: none of these are the
//! retained trace. Each is computed from it, so each can be computed from the
//! wrong evidence — a stale cache, a summary that was never produced, a
//! contributor list standing in for a spectrum — and still render something
//! plausible. These pin the derivation to the exact retained analysis it is
//! entitled to read.

use super::*;

#[test]
fn noise_quick_view_exports_retained_psd_as_amplitude_density_without_summary() {
    let analysis = AnalysisResult::new(9, AnalysisType::Noise, "Noise").with_waveforms(vec![
        WaveformData::new(
            "onoise",
            vec![1.0, 10.0, 100.0],
            vec![1.0e-18, 4.0e-18, 9.0e-18],
            "#00ffff",
        ),
        WaveformData::new(
            "noise(R1)",
            vec![1.0, 10.0, 100.0],
            vec![0.25e-18, 1.0e-18, 2.25e-18],
            "#ff00ff",
        ),
    ]);
    let state = quick_view_state(analysis, ResultViewer::NoiseContrib);
    let run = state.simulation.active_run().unwrap();

    assert_eq!(
        quick_result_availability(&state, run),
        RetainedHardcopySourceAvailability::Available
    );
    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic noise plot")
    };
    assert_eq!(plot.viewer, ResultViewer::NoiseContrib);
    assert_eq!(plot.traces.len(), 2);
    let samples = plot.traces[0]
        .source_samples
        .iter()
        .map(|(x, y)| (f64::from_bits(*x), f64::from_bits(*y)))
        .collect::<Vec<_>>();
    assert_eq!(
        samples.iter().map(|sample| sample.0).collect::<Vec<_>>(),
        [1.0, 10.0, 100.0]
    );
    for (actual, expected) in samples.iter().map(|sample| sample.1).zip([1.0, 2.0, 3.0]) {
        assert!((actual - expected).abs() < 1.0e-12);
    }
}

#[test]
fn hbnoise_quick_view_exports_retained_psd_as_amplitude_density() {
    let analysis = AnalysisResult::new(9, AnalysisType::Hbnoise, "HBNOISE").with_waveforms(vec![
        WaveformData::new(
            "onoise",
            vec![1.0e3, 1.0e4, 1.0e5],
            vec![1.0e-18, 4.0e-18, 9.0e-18],
            "#00ffff",
        ),
    ]);
    let state = quick_view_state(analysis, ResultViewer::NoiseContrib);
    let resolved =
        resolve_quick_view(&state).expect("HBNOISE density exports through the noise instrument");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic HBNOISE plot");
    };
    assert_eq!(plot.viewer, ResultViewer::NoiseContrib);
    assert_eq!(plot.traces.len(), 1);
    let samples = plot.traces[0]
        .source_samples
        .iter()
        .map(|(x, y)| (f64::from_bits(*x), f64::from_bits(*y)))
        .collect::<Vec<_>>();
    for (actual, expected) in samples.iter().map(|sample| sample.1).zip([1.0, 2.0, 3.0]) {
        assert!((actual - expected).abs() < 1.0e-15);
    }
}

#[test]
fn noise_quick_view_rejects_contributor_only_evidence() {
    let analysis = AnalysisResult::new(10, AnalysisType::Noise, "Noise").with_waveforms(vec![
        WaveformData::new(
            "noise(R1)",
            vec![1.0, 10.0],
            vec![1.0e-18, 4.0e-18],
            "#ff00ff",
        ),
    ]);
    let state = quick_view_state(analysis, ResultViewer::NoiseContrib);
    let run = state.simulation.active_run().unwrap();

    assert!(!quick_result_availability(&state, run).is_available());
    assert!(matches!(
        resolve_quick_view(&state),
        Err(HardcopySourceError::UnretainedResult(reason))
            if reason.contains("no retained analysis can provide exact evidence")
    ));
}

#[test]
fn noise_quick_view_prefers_input_reference_exactly_like_the_results_instrument() {
    let analysis = AnalysisResult::new(11, AnalysisType::Noise, "Noise").with_waveforms(vec![
        WaveformData::new(
            "onoise",
            vec![1.0, 10.0],
            vec![100.0e-18, 400.0e-18],
            "#00ffff",
        ),
        WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-18, 4.0e-18], "#ff00ff"),
    ]);
    let state = quick_view_state(analysis, ResultViewer::NoiseContrib);

    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic noise plot")
    };
    assert_eq!(plot.traces.len(), 1);
    assert_eq!(plot.traces[0].label, "inoise");
    let density = f64::from_bits(plot.traces[0].source_samples[1].1);
    assert!((density - 2.0).abs() < 1.0e-12);
}

#[test]
fn noise_quick_view_falls_back_to_the_first_renderable_noise_analysis() {
    let mut state = quick_view_state(
        AnalysisResult::new(12, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00ffff"),
        ]),
        ResultViewer::NoiseContrib,
    );
    state.simulation.active_run_mut().unwrap().analyses.push(
        AnalysisResult::new(13, AnalysisType::Noise, "Noise").with_waveforms(vec![
            WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 4.0e-18], "#ff00ff"),
        ]),
    );
    let run = state.simulation.active_run().unwrap();

    assert_eq!(
        quick_result_availability(&state, run),
        RetainedHardcopySourceAvailability::Available
    );
    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic noise plot")
    };
    assert_eq!(plot.traces[0].label, "onoise");
}

#[test]
fn fft_quick_view_ignores_stale_cache_and_global_data_version() {
    let time = (0..64)
        .map(|index| index as f64 * 1.0e-6)
        .collect::<Vec<_>>();
    let values = (0..64)
        .map(|index| (index as f64 * std::f64::consts::TAU / 8.0).sin())
        .collect::<Vec<_>>();
    let analysis =
        AnalysisResult::new(7, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(active)", time, values, "#00ffff"),
        ]);
    let mut state = quick_view_state(analysis, ResultViewer::Fft);
    state.analysis.fft_state.selected_source = Some("V(active)".to_owned());
    state.analysis.fft_state.data = Some(crate::analysis::FftData::from_spectrum(
        "stale",
        &[9_999.0, 10_000.0],
        &[8_888.0, 7_777.0],
        &[0.0, 0.0],
        20_000.0,
    ));
    state.simulation.data_version = 9;

    let first = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = first.semantic_document() else {
        panic!("expected FFT plot")
    };
    assert_eq!(plot.viewer, ResultViewer::Fft);
    assert!(
        plot.traces[0]
            .source_samples
            .iter()
            .all(|(x, y)| *x != 9_999.0f64.to_bits() && *y != 8_888.0f64.to_bits())
    );

    state.simulation.data_version = 10_000;
    state.analysis.fft_state.data = Some(crate::analysis::FftData::from_spectrum(
        "different stale cache",
        &[123_456.0],
        &[654_321.0],
        &[0.0],
        1.0,
    ));
    let second = resolve_quick_view(&state).unwrap();
    assert_eq!(
        first.authority().document_id(),
        second.authority().document_id()
    );
    assert_eq!(first.authority().revision(), second.authority().revision());
    assert_eq!(
        first.authority().content_digest(),
        second.authority().content_digest()
    );
}

#[test]
fn eye_quick_view_reconstructs_the_interactive_source_contract() {
    let time = (0..161)
        .map(|index| index as f64 * 0.25)
        .collect::<Vec<_>>();
    let ignored = vec![42.0; time.len()];
    let selected = time
        .iter()
        .map(|time| if (*time as i64) % 2 == 0 { -1.0 } else { 1.0 })
        .collect::<Vec<_>>();
    let analysis =
        AnalysisResult::new(8, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("A(first)", time.clone(), ignored, "#ff00ff"),
            WaveformData::new("V(selected)", time.clone(), selected.clone(), "#00ffff"),
        ]);
    let mut state = quick_view_state(analysis, ResultViewer::Eye);
    state.analysis.fft_state.selected_source = Some("|V(selected)|".to_owned());
    let mut stale_eye = crate::analysis::EyeData::new(99.0, 7);
    stale_eye.add_trace(crate::analysis::EyeTrace::new(
        vec![0.0, 1.0],
        vec![9_999.0, 9_999.0],
    ));
    state.analysis.eye_diagram_state.load_data(stale_eye);

    let period = retained_eye_bit_period(&time, &selected).unwrap();
    let expected = crate::analysis::eye_diagram::EyeDataBuilder::new()
        .bit_period(period)
        .ui_count(2)
        .skip_initial(2)
        .build(&time, &selected);
    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected eye plot")
    };
    assert_eq!(plot.traces.len(), expected.traces.len());
    for (actual, expected) in plot.traces.iter().zip(expected.traces.iter()) {
        assert_eq!(
            actual.source_samples,
            expected
                .time
                .iter()
                .copied()
                .zip(expected.amplitude.iter().copied())
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect::<Vec<_>>()
        );
        assert!(
            actual
                .source_samples
                .iter()
                .all(|(_, y)| *y != 9_999.0f64.to_bits())
        );
    }
}

#[test]
fn histogram_quick_view_derives_only_from_active_monte_carlo_metadata() {
    let samples = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    let analysis = AnalysisResult::new(9, AnalysisType::MonteCarlo, "Monte Carlo")
        .with_family_metadata(AnalysisResultFamilyMetadata::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 17,
            runs_requested: samples.len(),
            runs_completed: samples.len(),
            failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableMetadata {
                name: "gain".to_owned(),
                samples: samples.clone(),
                mean: 0.0,
                std_dev: 2.0f64.sqrt(),
                min: -2.0,
                max: 2.0,
            }],
        });
    let mut state = quick_view_state(analysis, ResultViewer::Hist);
    state.analysis.histogram_state.load_histogram(
        crate::analysis::HistogramBuilder::new()
            .name("stale")
            .bin_count(3)
            .build(&[9_999.0; 20]),
    );
    state.analysis.histogram_state.bin_count = 5;

    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected histogram plot")
    };
    assert_eq!(plot.traces[0].label, "gain");
    let retained_count = plot.traces[0]
        .source_samples
        .iter()
        .map(|(_, count)| f64::from_bits(*count))
        .sum::<f64>();
    assert_eq!(retained_count, samples.len() as f64);
    assert!(
        plot.traces[0]
            .source_samples
            .iter()
            .all(|(center, _)| *center != 9_999.0f64.to_bits())
    );
}

/// The page carries the reading, not only the samples.
///
/// A quick marker the reader anchored, and the A/B cursors they placed, were
/// absent from every printed quick-view page: the capture froze the retained
/// samples and the viewer controls and nothing else. They are placed here
/// through the same mapping the traces go through, so a marker sits on its
/// curve on the page for the reason it sits on it on screen — and the exact
/// source coordinates travel beside the geometry.
#[test]
fn a_printed_quick_view_carries_the_markers_and_cursors_the_reader_placed() {
    use crate::workbench::documents::result_document::{
        AnalysisPresentationKey, marker_anchor_for,
    };

    let analysis =
        AnalysisResult::new(4, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 4.0, 8.0], "#0af"),
        ]);
    let mut state = quick_view_state(analysis, ResultViewer::Waves);
    let analysis_key = {
        let run = state.simulation.active_run().expect("active run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    };
    let marker_id = state.ui.results.add_marker(
        analysis_key,
        marker_anchor_for(analysis_key, "V(out)"),
        "V(out)".to_owned(),
        1.0,
    );
    state.ui.results.cursors.a = Some(0.0);
    state.ui.results.cursors.b = Some(2.0);

    let resolved = resolve_quick_view(&state).expect("quick view resolves");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected a semantic waveform plot")
    };

    // The marker rides its trace: the exact source coordinates travel, and
    // the resampled Y is the trace's own value at the anchor.
    assert_eq!(plot.markers.len(), 1, "{:?}", plot.markers);
    let marker = &plot.markers[0];
    assert_eq!(marker.label, format!("M{marker_id}"));
    assert_eq!(marker.source_x_bits, Some(1.0f64.to_bits()));
    assert_eq!(marker.source_y_bits, Some(4.0f64.to_bits()));
    assert_eq!(marker.trace_id, Some(plot.traces[0].trace_id));

    // The anchor is the midpoint of a linear 0..2 sweep, so it lands on the
    // horizontal centre of the axis rectangle.
    let position = marker.position.expect("a marker carries a page position");
    assert_eq!(position.x_um, PLOT_WIDTH_UM / 2);

    // Both cursors are full-height lines at their own source X: A on the
    // left inset, B on the right.
    let cursors: Vec<&str> = plot
        .cursors
        .iter()
        .map(|cursor| cursor.label.as_str())
        .collect();
    assert_eq!(cursors, ["A", "B"]);
    let a = &plot.cursors[0];
    let b = &plot.cursors[1];
    assert_eq!(a.source_x_bits, 0.0f64.to_bits());
    assert_eq!(b.source_x_bits, 2.0f64.to_bits());
    assert_eq!(a.start.x_um, a.end.x_um);
    assert_eq!(b.start.x_um, b.end.x_um);
    assert!(a.start.x_um < b.start.x_um);
    assert_ne!(a.start.y_um, a.end.y_um);

    // A page with nothing placed on it carries neither.
    state.ui.results.markers.clear();
    state.ui.results.cursors.a = None;
    state.ui.results.cursors.b = None;
    let bare = resolve_quick_view(&state).expect("quick view resolves");
    let HardcopySemanticDocument::Plot(bare) = bare.semantic_document() else {
        panic!("expected a semantic waveform plot")
    };
    assert!(bare.markers.is_empty());
    assert!(bare.cursors.is_empty());
}

/// A retained document marker reaches the page as itself.
///
/// The two stores allocate independently and label distinctly, so a `D`
/// marker must arrive with its own tag rather than being restated as a quick
/// one. A spec limit arrives as the full-height line the sheet draws, because
/// it constrains the axis position rather than one curve.
#[test]
fn a_retained_document_marker_and_a_spec_limit_reach_the_page_as_themselves() {
    use crate::workbench::documents::result_document::MarkerKind;

    let series = vec![QuickResultSeries {
        identity: "trace-identity".to_owned(),
        label: "V(out)".to_owned(),
        points: vec![(0.0, 0.0), (1.0, 4.0), (2.0, 8.0)],
    }];
    let overlay = RetainedQuickViewOverlay::for_test(
        None,
        None,
        vec![
            RetainedQuickMarker {
                label: "D7 · settling".to_owned(),
                kind: MarkerKind::Peak,
                x: 1.0,
                trace_name: Some("V(out)".to_owned()),
            },
            RetainedQuickMarker {
                label: "M2 · upper limit".to_owned(),
                kind: MarkerKind::Spec,
                x: 2.0,
                trace_name: None,
            },
            RetainedQuickMarker {
                label: "M3 · other pane".to_owned(),
                kind: MarkerKind::Note,
                x: 1.0,
                trace_name: Some("V(elsewhere)".to_owned()),
            },
        ],
    );

    let plot = quick_plot_from_series(ResultViewer::Waves, "Results", 0, series, Some(&overlay))
        .expect("plot resolves");

    assert_eq!(
        plot.markers
            .iter()
            .map(|marker| marker.label.as_str())
            .collect::<Vec<_>>(),
        ["D7 · settling"],
        "a spec limit is a line, and a marker whose trace is not on this page is skipped"
    );
    assert_eq!(plot.markers[0].source_y_bits, Some(4.0f64.to_bits()));
    assert_eq!(
        plot.cursors
            .iter()
            .map(|cursor| cursor.label.as_str())
            .collect::<Vec<_>>(),
        ["M2 · upper limit"]
    );
    let limit = &plot.cursors[0];
    assert_eq!(limit.start.x_um, limit.end.x_um);
    assert_eq!(limit.source_x_bits, 2.0f64.to_bits());
}

/// A trace the reader hid does not print.
///
/// The capture filtered on the dataset's own `visible` flag, which a session
/// override never touches, so a trace hidden on the sheet stayed on the page.
#[test]
fn a_trace_hidden_on_the_sheet_is_not_printed() {
    let analysis =
        AnalysisResult::new(5, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#0af"),
            WaveformData::new("V(mid)", vec![0.0, 1.0], vec![0.0, 0.5], "#fa0"),
        ]);
    let mut state = quick_view_state(analysis, ResultViewer::Waves);

    let resolved = resolve_quick_view(&state).expect("quick view resolves");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected a semantic waveform plot")
    };
    assert_eq!(plot.traces.len(), 2);

    crate::workbench::documents::result_document::toggle_visibility(&mut state, 0, 1);
    let resolved = resolve_quick_view(&state).expect("quick view resolves");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected a semantic waveform plot")
    };
    assert_eq!(
        plot.traces
            .iter()
            .map(|trace| trace.label.as_str())
            .collect::<Vec<_>>(),
        ["V(out)"]
    );
}

/// A frequency response prints on the axis it is read on.
///
/// The page mapped every sheet the same way — value minus minimum, over the
/// span, across the frame — so a 1 Hz to 1 MHz sweep printed with five of its
/// six decades squeezed into the leftmost tenth of the frame, under nine
/// evenly spaced division lines ruled across a span that has no even
/// divisions. The decade positions below are the analytic oracle: on a log
/// axis each decade occupies an equal share of the frame, so the midpoint of
/// a six-decade sweep is the middle of the plot and 10 Hz sits one fifth in.
#[test]
fn a_frequency_sweep_is_printed_in_decades_and_ruled_at_them() {
    let frequency = (0..=6)
        .map(|decade| 10.0f64.powi(decade))
        .collect::<Vec<_>>();
    let gain = frequency.iter().map(|_| 1.0).collect::<Vec<_>>();
    let analysis = AnalysisResult::new(6, AnalysisType::Disto, "DISTO")
        .with_waveforms(vec![WaveformData::new("V(out)", frequency, gain, "#0af")]);
    let state = quick_view_state(analysis, ResultViewer::Bode);

    let resolved = resolve_quick_view(&state).expect("the sweep resolves");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected a semantic frequency plot")
    };

    assert_eq!(plot.x_scale, AxisScale::Logarithmic);

    // Seven decade lines, each captioned with the frequency it stands for.
    let major = plot
        .axis_ticks
        .iter()
        .filter(|tick| tick.major)
        .collect::<Vec<_>>();
    assert_eq!(major.len(), 7, "{:?}", plot.axis_ticks);
    assert_eq!(
        major
            .iter()
            .map(|tick| tick.label.as_str())
            .collect::<Vec<_>>(),
        ["1.00", "10.0", "100", "1.00 k", "10.0 k", "100 k", "1.00 M"]
    );
    // A span of exactly six decades still rules its mantissas, because that
    // is what the sheet rules: `ui::plot::scale::minor_grid_values` stands
    // them down past six decades of *span*, and the page follows the sheet.
    // Eight above each of the first six decades; the seventh is the window's
    // own right edge and has nothing above it.
    assert_eq!(
        plot.axis_ticks.iter().filter(|tick| !tick.major).count(),
        48,
        "{:?}",
        plot.axis_ticks
    );

    // Equal decades: 1 Hz on the left inset, 1 kHz in the middle, 1 MHz on
    // the right edge. Under the old linear mapping 1 kHz sat a thousandth of
    // the way across.
    let position = |index: usize| major[index].start.x_um;
    assert_eq!(position(0), PLOT_INSET_UM);
    assert_eq!(position(3), PLOT_WIDTH_UM / 2);
    assert_eq!(position(6), PLOT_WIDTH_UM - PLOT_INSET_UM);
    assert_eq!(
        position(1) - position(0),
        position(6) - position(5),
        "every decade occupies the same width"
    );

    // A sweep of two decades rules its minor lines as well, eight to a decade.
    let frequency = [1.0, 10.0, 100.0];
    let analysis = AnalysisResult::new(7, AnalysisType::Disto, "DISTO").with_waveforms(vec![
        WaveformData::new("V(out)", frequency.to_vec(), vec![1.0, 1.0, 1.0], "#0af"),
    ]);
    let state = quick_view_state(analysis, ResultViewer::Bode);
    let resolved = resolve_quick_view(&state).expect("the sweep resolves");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected a semantic frequency plot")
    };
    assert_eq!(plot.axis_ticks.iter().filter(|tick| tick.major).count(), 3);
    assert_eq!(
        plot.axis_ticks.iter().filter(|tick| !tick.major).count(),
        16,
        "eight minor lines above each of the first two decades, none above the last"
    );
    assert!(
        plot.axis_ticks
            .iter()
            .all(|tick| tick.major != tick.label.is_empty()),
        "a minor line carries no caption, and a decade always does"
    );
}

/// The spectrum prints in the unit its sheet reads.
///
/// The FFT sheet is a decibel instrument — it plots `magnitude_db` against a
/// reference-aware level unit — and the page took the linear magnitude, so a
/// printed spectrum showed one peak on a flat floor where the sheet showed
/// every harmonic above a noise floor sixty decibels down.
#[test]
fn the_printed_spectrum_is_in_decibels() {
    let time = (0..64)
        .map(|index| index as f64 * 1.0e-6)
        .collect::<Vec<_>>();
    let values = (0..64)
        .map(|index| (index as f64 * std::f64::consts::TAU / 8.0).sin())
        .collect::<Vec<_>>();
    let analysis = AnalysisResult::new(8, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![WaveformData::new("V(active)", time, values, "#0af")]);
    let mut state = quick_view_state(analysis, ResultViewer::Fft);
    state.analysis.fft_state.selected_source = Some("V(active)".to_owned());

    let resolved = resolve_quick_view(&state).expect("the spectrum resolves");
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected a semantic spectrum plot")
    };

    assert_eq!(plot.y_scale, AxisScale::Decibels);
    // The sheet's frequency axis is linear, so the page's is too.
    assert_eq!(plot.x_scale, AxisScale::Linear);
    assert!(plot.axis_ticks.is_empty());

    let levels = plot.traces[0]
        .source_samples
        .iter()
        .map(|(_, y)| f64::from_bits(*y))
        .collect::<Vec<_>>();
    // Decibels: the peak is near zero and the floor is far below it. A linear
    // magnitude spectrum of the same signal is bounded by one and above zero.
    let peak = levels.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let floor = levels.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(peak <= 0.5, "peak level {peak} is not a decibel level");
    assert!(
        floor < -20.0,
        "floor level {floor} did not come from a decibel projection"
    );
}

/// A failed noise solve is not printable evidence.
///
/// The hardcopy resolver carried its own copy of the sheet's renderability
/// predicate, and that copy had no success gate. A noise run that did not
/// converge retains whatever vectors the engine emitted before it gave up;
/// the sheet refuses to draw them and the page must refuse to print them.
#[test]
fn a_failed_noise_solve_is_neither_offered_nor_printed() {
    let mut analysis = AnalysisResult::new(9, AnalysisType::Noise, "Noise").with_waveforms(vec![
        WaveformData::new(
            "onoise",
            vec![1.0, 10.0, 100.0],
            vec![1.0e-18, 4.0e-18, 9.0e-18],
            "#00ffff",
        ),
    ]);
    analysis.success = false;
    analysis.error_message = Some("noise analysis did not converge".to_owned());

    let mut state = quick_view_state(analysis, ResultViewer::NoiseContrib);
    let run = state.simulation.active_run().unwrap();

    assert!(!quick_result_availability(&state, run).is_available());
    assert!(resolve_quick_view(&state).is_err());

    // And it does not shadow one that did converge. The run-wide fallback
    // scanned for the first analysis the ungated predicate accepted, so a run
    // that solved noise twice — once badly, once well — bound the page to the
    // failed attempt and then refused the whole page for being unsuccessful.
    state.simulation.runs[0].analyses.push(
        AnalysisResult::new(10, AnalysisType::Noise, "Noise").with_waveforms(vec![
            WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 4.0e-18], "#0ff"),
        ]),
    );
    state.simulation.runs[0].analyses.push(AnalysisResult::new(
        11,
        AnalysisType::Transient,
        "TRAN",
    ));
    state.simulation.active_analysis_idx = Some(2);
    let run = state.simulation.active_run().unwrap();
    assert_eq!(
        quick_result_analysis_index(&state, run, ResultViewer::NoiseContrib),
        Some(1)
    );
    assert_eq!(
        quick_result_availability(&state, run),
        RetainedHardcopySourceAvailability::Available
    );
}

/// A selected noise analysis binds strictly on paper too.
///
/// The resolver stepped to the next renderable result whenever the reader's
/// own selection was a noise analysis carrying no ordinary spectrum — so a
/// page printed from a selected PNOISE result showed a different analysis's
/// spectrum, under the selected analysis's name. A selection that expresses
/// no noise intent at all still falls back run-wide, exactly as the sheet
/// does.
#[test]
fn a_selected_noise_analysis_is_never_substituted_on_the_printed_page() {
    let phase = AnalysisResult::new(1, AnalysisType::Pnoise, "PNOISE").with_waveforms(vec![
        WaveformData::new(
            "phase_noise",
            vec![1.0, 10.0],
            vec![1.0e-18, 4.0e-18],
            "#fff",
        ),
    ]);
    let ordinary = AnalysisResult::new(2, AnalysisType::Noise, "NOISE").with_waveforms(vec![
        WaveformData::new("onoise", vec![1.0, 10.0], vec![1.0e-18, 4.0e-18], "#fff"),
    ]);
    let transient = AnalysisResult::new(3, AnalysisType::Transient, "TRAN");

    let mut state = quick_view_state(phase, ResultViewer::NoiseContrib);
    state.simulation.runs[0].analyses.push(ordinary);
    state.simulation.runs[0].analyses.push(transient);

    // The selection is a noise-family analysis with no ordinary spectrum:
    // nothing is offered rather than the neighbouring NOISE result.
    state.simulation.active_analysis_idx = Some(0);
    let run = state.simulation.active_run().unwrap();
    assert_eq!(
        quick_result_analysis_index(&state, run, ResultViewer::NoiseContrib),
        None
    );
    assert!(!quick_result_availability(&state, run).is_available());

    // The selection is the ordinary-noise result itself.
    state.simulation.active_analysis_idx = Some(1);
    let run = state.simulation.active_run().unwrap();
    assert_eq!(
        quick_result_analysis_index(&state, run, ResultViewer::NoiseContrib),
        Some(1)
    );

    // A transient selection expresses no noise intent, so the run-wide
    // fallback still finds the one printable spectrum.
    state.simulation.active_analysis_idx = Some(2);
    let run = state.simulation.active_run().unwrap();
    assert_eq!(
        quick_result_analysis_index(&state, run, ResultViewer::NoiseContrib),
        Some(1)
    );
}

/// The specifications page is offered on the requirements the run was judged
/// against, which is what the capture writes.
///
/// The offering read `workspace.specs` — the currently authored contract. A
/// receipt-backed run whose frozen requirements had since been deleted from
/// the workspace was refused a page it could fill, and a run prepared with no
/// requirements at all was offered one that resolves to an empty table the
/// moment a limit is authored.
#[test]
fn the_specifications_page_is_offered_on_the_requirements_the_run_froze() {
    use crate::product::{AnalysisInstanceId, SimulationPlanId};
    use crate::state::{
        AnalysisResultSourceDomain, PreparedRunReceipt, PreparedRunTaskReceipt,
        PreparedSourceCheckReceipt, PreparedSpecification, SpecEntry, SpecPointScope,
        SpecificationDefinition,
    };

    fn receipt_run(requirements: usize) -> SimulationRun {
        let task = PreparedRunTaskReceipt::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            1,
            ContentDigest::from_bytes([0x72; 32]),
        )
        .expect("task receipt");
        let definitions = (0..requirements)
            .map(|index| {
                let projection = SpecEntry {
                    measurement: format!("gain{index}"),
                    expression: String::new(),
                    min: Some(0.0),
                    max: Some(1.0),
                    unit: "dB".to_owned(),
                    scope: SpecPointScope::AllPoints,
                };
                PreparedSpecification::from_definition(SpecificationDefinition::from_legacy(
                    SimulationPlanId::new(),
                    index,
                    &projection,
                ))
                .expect("prepared requirement")
            })
            .collect();
        let receipt = PreparedRunReceipt::new_with_project_model_sources_and_specifications(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x71; 32]),
            ContentDigest::from_bytes([0x73; 32]),
            PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x74; 32])),
            Vec::new(),
            definitions,
            vec![task],
        )
        .expect("prepared receipt");
        let mut run = SimulationRun::new_prepared(1, receipt);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.analyses
            .push(AnalysisResult::new(1, AnalysisType::Ac, "AC"));
        run
    }

    fn specs_state(run: SimulationRun) -> AppState {
        let mut state = AppState::default();
        state.simulation.runs.push(run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);
        state.ui.results.viewer = ResultViewer::Specs;
        state
    }

    let workspace_only = SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(0.0),
        max: Some(1.0),
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    };

    // Frozen requirements, no live workspace contract: the page is offered.
    let state = specs_state(receipt_run(1));
    assert!(state.workspace.specs.is_empty());
    let run = state.simulation.active_run().unwrap();
    assert_eq!(
        quick_result_availability(&state, run),
        RetainedHardcopySourceAvailability::Available
    );

    // No frozen requirements: a limit authored after the run does not conjure
    // a page, because the capture resolves the frozen set and would write an
    // empty table.
    let mut state = specs_state(receipt_run(0));
    state.workspace.specs.push(workspace_only.clone());
    let run = state.simulation.active_run().unwrap();
    assert!(!quick_result_availability(&state, run).is_available());

    // A legacy dataset with no receipt still reads the workspace contract.
    let mut legacy = SimulationRun::new(1);
    legacy.lifecycle = SimulationRunLifecycle::Completed;
    legacy
        .analyses
        .push(AnalysisResult::new(1, AnalysisType::Ac, "AC"));
    let mut state = specs_state(legacy);
    state.workspace.specs.push(workspace_only);
    let run = state.simulation.active_run().unwrap();
    assert_eq!(
        quick_result_availability(&state, run),
        RetainedHardcopySourceAvailability::Available
    );
}

/// The page is taken from the sheet as the reader left it.
///
/// Traces they hid, cursors they placed and markers they anchored all travel
/// with the capture, because those are the reading. The window they zoomed
/// into did not: the page re-derived its own extents from every retained
/// sample, so printing a detail a reader had pinned handed back the whole
/// sweep.
#[test]
fn a_pinned_window_is_part_of_the_page_the_reader_captured() {
    let analysis = AnalysisResult::new(4, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        WaveformData::new(
            "V(out)",
            vec![0.0, 0.25, 0.5, 0.75, 1.0],
            vec![0.0, 1.0, 0.0, -1.0, 0.0],
            "#00ffff",
        ),
    ]);
    let mut state = quick_view_state(analysis, ResultViewer::Waves);

    let point_count = |state: &AppState| {
        let resolved = resolve_quick_view(state).expect("the retained transient resolves");
        let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
            panic!("expected a semantic waveform plot")
        };
        plot.traces[0].paths.iter().map(Vec::len).sum::<usize>()
    };

    assert_eq!(point_count(&state), 5, "the whole sweep is five samples");

    state
        .ui
        .results
        .plot_view_mut(ResultViewer::Waves, 0)
        .apply(&crate::ui::plot::ViewChange {
            x: Some((0.4, 0.6)),
            y: None,
            reset: false,
        });

    assert_eq!(
        point_count(&state),
        3,
        "the printed page ignored the window the reader had pinned on the sheet"
    );
}
