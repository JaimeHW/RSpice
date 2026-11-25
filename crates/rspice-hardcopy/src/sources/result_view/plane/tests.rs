use super::*;
use rspice_results::run::{SimulationRun, SimulationRunLifecycle};

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

/// The width and height of everything a guide draws, in micrometres.
fn extent(guide: &SemanticPlotGuide) -> (i64, i64) {
    let points = guide.paths.iter().flatten();
    let xs = points.clone().map(|point| point.x_um);
    let ys = points.map(|point| point.y_um);
    (
        xs.clone().max().unwrap() - xs.min().unwrap(),
        ys.clone().max().unwrap() - ys.min().unwrap(),
    )
}

/// L(jω) = 4 / (1 + jω)³: its phase crosses −180° at ω = √3, where |L| is
/// one half, so its gain margin is 6.02 dB.
fn loop_gain(omega: f64) -> (f64, f64) {
    let (mut re, mut im) = (4.0, 0.0);
    for _ in 0..3 {
        // Divide by (1 + jω).
        let denominator = 1.0 + omega * omega;
        (re, im) = (
            (re + im * omega) / denominator,
            (im - re * omega) / denominator,
        );
    }
    (re, im)
}

fn stability_run() -> SimulationRun {
    let frequency = (0..=200)
        .map(|index| 10f64.powf(-3.0 + f64::from(index) * 0.025))
        .collect::<Vec<_>>();
    let gain = frequency
        .iter()
        .map(|f| loop_gain(std::f64::consts::TAU * f))
        .collect::<Vec<_>>();
    completed(
        AnalysisResult::new(4, AnalysisType::Stb, "STB", 0.0).with_waveforms(vec![
            RetainedWaveform::new(
                rspice_results::nyquist::LOOP_GAIN_REAL_TRACE,
                frequency.clone(),
                gain.iter().map(|point| point.0).collect::<Vec<_>>(),
            ),
            RetainedWaveform::new(
                rspice_results::nyquist::LOOP_GAIN_IMAGINARY_TRACE,
                frequency,
                gain.iter().map(|point| point.1).collect::<Vec<_>>(),
            ),
        ]),
    )
}

/// The print draws the loop gain the sheet draws, read from the two real
/// traces a stability run keeps it as: the locus, its mirror dashed, the
/// critical point and the gain margin where it is read.
#[test]
fn the_nyquist_print_draws_a_stability_runs_loop_gain() {
    let run = stability_run();
    let source = RetainedQuickViewSource::try_new(&run, 4, |_| true).unwrap();
    let plot = quick_nyquist_plot(&source, &RetainedQuickViewOverlay::default()).unwrap();
    assert_eq!(plot.traces.len(), 2, "the locus and its mirror");
    assert_eq!(plot.traces[0].label, LOOP_GAIN_LOCUS_LABEL);
    assert!(!plot.traces[0].companion && plot.traces[1].companion);
    assert!(!plot.traces[0].paths.is_empty(), "the locus is in view");
    let captions = plot
        .captions
        .iter()
        .map(|caption| caption.text.as_str())
        .collect::<Vec<_>>();
    assert!(captions.contains(&"\u{2212}1 + j0"), "{captions:?}");
    assert!(captions.contains(&"GM 6.02 dB"), "{captions:?}");
}

/// The plane fills the plot area on one scale: the unit circle prints
/// round, and the window is wider than it is tall, as the page is.
#[test]
fn the_nyquist_plane_fills_the_page_on_one_scale() {
    let run = stability_run();
    let source = RetainedQuickViewSource::try_new(&run, 4, |_| true).unwrap();
    let plot = quick_nyquist_plot(&source, &RetainedQuickViewOverlay::default()).unwrap();
    let (width, height) = extent(&plot.guides[2]);
    assert!(
        width > 10_000 && (width - height).abs() <= 2,
        "{width} × {height}"
    );
    let plane = Plane::new((-2.0, 1.0), (-1.5, 1.5));
    let per_x = (plane.x.1 - plane.x.0) / AREA.0 as f64;
    let per_y = (plane.y.1 - plane.y.0) / AREA.1 as f64;
    assert!((per_x - per_y).abs() < 1.0e-12 * per_x);
    assert!(plane.x.1 - plane.x.0 > 3.0, "the spare width is filled");
}

/// A Smith page rules the unit circle firmly, five resistance circles and
/// ten reactance arcs, labels them as the sheet does, and keeps it round.
#[test]
fn a_smith_print_rules_the_chart_and_keeps_it_round() {
    let frequency = (0..50)
        .map(|index| 1.0e8 * f64::from(index + 1))
        .collect::<Vec<_>>();
    let angle = frequency.iter().map(|f| -f / 1.0e9).collect::<Vec<_>>();
    let run = completed(
        AnalysisResult::new(5, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("|V(out)|", frequency.clone(), vec![0.5; frequency.len()])
                .with_complex_components(
                    "V(out)",
                    angle.iter().map(|a| 0.5 * a.cos()).collect::<Vec<_>>(),
                    angle.iter().map(|a| 0.5 * a.sin()).collect::<Vec<_>>(),
                ),
        ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 5, |_| true).unwrap();
    let sheet = QuickSmithSettings {
        responses: vec!["|V(out)|".to_owned()],
        ..QuickSmithSettings::default()
    };
    let plot = quick_smith_plot(&source, &sheet).unwrap();
    assert_eq!(plot.traces.len(), 1);
    assert!(
        plot.guides[0].major,
        "the rim is drawn as firmly as the frame"
    );
    assert_eq!(plot.guides.len(), 2 + 5 + 10);
    let (width, height) = extent(&plot.guides[0]);
    assert!((width - height).abs() <= 2, "{width} × {height}");
    let captions = plot
        .captions
        .iter()
        .map(|caption| caption.text.as_str())
        .collect::<Vec<_>>();
    for label in ["0.2", "1", "5", "j0.5", "\u{2212}j2"] {
        assert!(captions.contains(&label), "{label} in {captions:?}");
    }
}

/// An AC run whose one complex response is 150 + j0 Ω at every frequency.
fn impedance_run() -> SimulationRun {
    let frequency = vec![1.0e6, 2.0e6, 3.0e6];
    completed(
        AnalysisResult::new(5, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("Zin", frequency, vec![150.0; 3]).with_complex_components(
                "Zin",
                vec![150.0; 3],
                vec![0.0; 3],
            ),
        ]),
    )
}

/// A response the sheet reads as an impedance lands where the sheet puts
/// it, Γ = (Z − Z₀)/(Z + Z₀), and the page states the reading and the Z₀
/// as the sheet's header does. A response the sheet does not list is not
/// drawn.
#[test]
fn a_smith_print_reads_a_response_as_the_sheet_reads_it() {
    let run = impedance_run();
    let source = RetainedQuickViewSource::try_new(&run, 5, |_| true).unwrap();
    assert!(
        quick_smith_plot(&source, &QuickSmithSettings::default()).is_err(),
        "a sheet listing no response draws nothing"
    );
    let sheet = QuickSmithSettings {
        reading: ResponseReading::Impedance,
        responses: vec!["Zin".to_owned()],
        ..QuickSmithSettings::default()
    };
    let plot = quick_smith_plot(&source, &sheet).unwrap();
    let samples = &plot.traces[0].source_samples;
    assert_eq!(samples.len(), 3);
    for (re, im) in samples {
        assert!((f64::from_bits(*re) - 0.5).abs() < 1.0e-12);
        assert!(f64::from_bits(*im).abs() < 1.0e-12);
    }
    assert_eq!(
        plot.captions[0].text,
        "Z\u{2080} = 50 \u{3a9} \u{b7} responses read as Z"
    );
    // At 150 Ω the same response reads as the centre of the chart.
    let matched = QuickSmithSettings {
        reference_ohm: 150.0,
        ..sheet.clone()
    };
    let plot = quick_smith_plot(&source, &matched).unwrap();
    let (re, _) = plot.traces[0].source_samples[0];
    assert!(f64::from_bits(re).abs() < 1.0e-12);
    // An expression the sheet lists is read through the calculator.
    let doubled = QuickSmithSettings {
        responses: vec!["Zin*2".to_owned()],
        ..sheet
    };
    let plot = quick_smith_plot(&source, &doubled).unwrap();
    let (re, _) = plot.traces[0].source_samples[0];
    assert!((f64::from_bits(re) - 250.0 / 350.0).abs() < 1.0e-12);
}

/// A network prints the terms its sheet draws: reflection terms, not
/// transmission terms, each switched as the reader switched it, and each in
/// the colour of its place among every term the sheet lists.
#[test]
fn a_smith_print_draws_the_network_terms_the_sheet_draws() {
    let frequency = vec![1.0e9, 2.0e9];
    let term = |name: &str, re: f64| {
        RetainedWaveform::new(format!("|{name}|"), frequency.clone(), vec![re; 2])
            .with_complex_components(name, vec![re; 2], vec![0.0; 2])
    };
    let run = completed(
        AnalysisResult::new(5, AnalysisType::SParameter, "SP", 0.0)
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm: vec![50.0, 75.0],
                noise_reference_temperature_kelvin: None,
            })
            .with_waveforms(vec![
                term("S22", 0.4),
                term("S21", 0.3),
                term("S12", 0.2),
                term("S11", 0.1),
            ]),
    );
    let source = RetainedQuickViewSource::try_new(&run, 5, |_| true).unwrap();
    let plot = quick_smith_plot(&source, &QuickSmithSettings::default()).unwrap();
    let drawn = |plot: &SemanticPlot| {
        plot.traces
            .iter()
            .map(|trace| (trace.label.clone(), trace.palette_slot))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        drawn(&plot),
        [("S11".to_owned(), Some(0)), ("S22".to_owned(), Some(3))]
    );
    assert_eq!(plot.captions[0].text, "Per-port retained Z\u{2080}");
    let switched = QuickSmithSettings {
        toggled: vec!["S11".to_owned(), "S21".to_owned()],
        ..QuickSmithSettings::default()
    };
    let plot = quick_smith_plot(&source, &switched).unwrap();
    assert_eq!(
        drawn(&plot),
        [("S21".to_owned(), Some(2)), ("S22".to_owned(), Some(3))]
    );
    assert_eq!(plot.captions[0].text, "Z\u{2080} = 75 \u{3a9}");
    assert_eq!(plot.trace_slots(), [2, 3]);
}

/// The admittance grid is the impedance grid reflected through the centre;
/// ruled with it, it is the second family, in its own ink, labelled below
/// the real axis and leaving the rim to the impedance grid.
#[test]
fn a_smith_print_rules_the_grid_the_sheet_rules() {
    let run = impedance_run();
    let source = RetainedQuickViewSource::try_new(&run, 5, |_| true).unwrap();
    let sheet = |grid| QuickSmithSettings {
        grid,
        responses: vec!["Zin".to_owned()],
        ..QuickSmithSettings::default()
    };
    let admittance = quick_smith_plot(&source, &sheet(SmithGrid::Admittance)).unwrap();
    assert_eq!(admittance.guides.len(), 2 + 5 + 10);
    assert!(admittance.guides.iter().all(|guide| !guide.second));
    // The g = 1 circle, after the rim, the real axis and the 0.2 and 0.5
    // families, runs from Γ = −1 to the centre: its leftmost point is the
    // rim's.
    let leftmost = |guide: &SemanticPlotGuide| {
        guide
            .paths
            .iter()
            .flatten()
            .map(|point| point.x_um)
            .min()
            .unwrap()
    };
    let rim = leftmost(&admittance.guides[0]);
    assert!((leftmost(&admittance.guides[2 + 3 * 2]) - rim).abs() <= 2);
    let both = quick_smith_plot(&source, &sheet(SmithGrid::ImpedanceAdmittance)).unwrap();
    assert_eq!(both.guides.len(), 2 + 15 + 15);
    assert_eq!(both.guides.iter().filter(|guide| guide.second).count(), 15);
    let rim_labels = |plot: &SemanticPlot| {
        plot.captions
            .iter()
            .filter(|caption| caption.text.contains('j'))
            .count()
    };
    assert_eq!(rim_labels(&admittance), 10);
    assert_eq!(rim_labels(&both), 10);
    assert_eq!(both.captions.len(), 1 + 15 + 5);
}
