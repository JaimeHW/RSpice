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
    assert!(
        captions
            .iter()
            .any(|caption| caption.starts_with("GM 6.02")),
        "{captions:?}"
    );
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
    let plot = quick_smith_plot(&source).unwrap();
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
