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
