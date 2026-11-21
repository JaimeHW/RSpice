use num_complex::Complex64;
use rspice_results::calculator::evaluator::evaluate;
use rspice_results::calculator::parser::try_parse;
use rspice_results::calculator::retained::WaveformsContext;
use rspice_results::calculator::value::{CalcValue, RealValue};
use rspice_results::network_matrix::two_port::TwoPortSweep;
use rspice_results::waveform::Fetched;
use rspice_results_ui::waves::TraceKind;

use super::*;
use crate::state::{AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle};
use crate::ui::tokens::Tokens;
use crate::workbench::documents::result_document::test_host::ResultsTestHost;

const FREQUENCIES: [f64; 4] = [0.5e9, 1.0e9, 2.0e9, 4.0e9];

fn polar(magnitude: f64, degrees: f64) -> Complex64 {
    Complex64::from_polar(magnitude, degrees.to_radians())
}

/// A transistor amplifier's S11, S12, S21 and S22 at each frequency.
fn amplifier_points() -> [[Complex64; 4]; 4] {
    [
        [
            polar(0.85, -30.0),
            polar(0.02, 80.0),
            polar(6.0, 160.0),
            polar(0.75, -15.0),
        ],
        [
            polar(0.78, -60.0),
            polar(0.035, 65.0),
            polar(5.2, 135.0),
            polar(0.70, -28.0),
        ],
        [
            polar(0.68, -110.0),
            polar(0.05, 45.0),
            polar(3.8, 100.0),
            polar(0.60, -45.0),
        ],
        [
            polar(0.60, -165.0),
            polar(0.07, 30.0),
            polar(2.4, 60.0),
            polar(0.55, -75.0),
        ],
    ]
}

/// A complex column as the converter keeps it: `|name|` holding the
/// magnitude and the components, and `phase(name)` in degrees.
fn complex_columns(name: &str, values: &[Complex64]) -> [RetainedTrace; 2] {
    let x = FREQUENCIES.to_vec();
    [
        RetainedTrace::new(
            format!("|{name}|"),
            x.clone(),
            values.iter().map(|value| value.norm()).collect::<Vec<_>>(),
            "#00aaff",
        )
        .with_complex_components(
            name,
            values.iter().map(|value| value.re).collect::<Vec<_>>(),
            values.iter().map(|value| value.im).collect::<Vec<_>>(),
        )
        .with_unit("1"),
        RetainedTrace::new(
            format!("phase({name})"),
            x,
            values
                .iter()
                .map(|value| value.arg().to_degrees())
                .collect::<Vec<_>>(),
            "#00aaff",
        )
        .with_unit("°"),
    ]
}

/// A two-port amplifier's SP run with port noise, in the shape the runner
/// and converter retain it.
fn amplifier() -> AnalysisResult {
    let points = amplifier_points();
    let mut waveforms = Vec::new();
    for (index, name) in ["S11", "S12", "S21", "S22"].into_iter().enumerate() {
        let values = points.iter().map(|point| point[index]).collect::<Vec<_>>();
        waveforms.extend(complex_columns(name, &values));
    }
    let real = |name: &str, unit: &str, values: [f64; 4]| {
        RetainedTrace::new(name, FREQUENCIES.to_vec(), values.to_vec(), "#00aaff").with_unit(unit)
    };
    waveforms.push(real("F", "1", [1.5, 1.6, 1.9, 2.4]));
    waveforms.push(real("Fmin", "1", [1.2, 1.25, 1.35, 1.6]));
    waveforms.push(real("Rn", "Ω", [12.0, 11.0, 10.0, 9.0]));
    waveforms.extend(complex_columns(
        "Sopt",
        &[
            polar(0.6, 40.0),
            polar(0.55, 70.0),
            polar(0.5, 110.0),
            polar(0.45, 160.0),
        ],
    ));
    AnalysisResult::new(1, AnalysisType::SParameter, "SP")
        .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm: vec![50.0, 50.0],
            noise_reference_temperature_kelvin: Some(290.0),
        })
        .with_waveforms(waveforms)
}

fn finished_run(analysis: AnalysisResult) -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    run.lifecycle = SimulationRunLifecycle::Completed;
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    assert!(state.model.simulation.select_analysis(0));
    state
}

fn analysis_key(state: &ResultsTestHost) -> AnalysisPresentationKey {
    let run = state.model.simulation.active_run().expect("a run");
    AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
}

/// A two-port amplifier's run opens on the S-parameter sheet, as ADS opens
/// an S-parameter dataset, with S21 drawn in decibels against frequency and
/// none of the run's noise columns sharing the decibel pane.
#[test]
fn a_two_port_amplifier_opens_on_the_s_parameter_sheet_with_s21_in_decibels() {
    let mut state = finished_run(amplifier());
    state.results.results.session.viewer = ResultViewer::Bode;

    super::super::reconcile_active_viewer(&mut state.results_parts());

    assert_eq!(
        state.results.results.session.viewer,
        ResultViewer::SParameters
    );
    let display = state
        .model
        .preferences
        .result_presentation_policy()
        .complex_number_display();
    let models = super::super::waves::cached_models(
        &state.model.simulation,
        &mut state.results.results,
        display,
        &Tokens::default(),
    );
    assert_eq!(models.len(), 1);
    let model = &models[0];
    assert_eq!(model.x_unit, "Hz");
    let s21 = model
        .traces
        .iter()
        .find(|trace| trace.source_waveform_name == "|S21|" && trace.kind == TraceKind::MagnitudeDb)
        .expect("S21 is drawn in decibels");
    let drawn = s21.y.read_whole().expect("held in memory");
    for (sample, point) in amplifier_points().iter().enumerate() {
        let expected = 20.0 * point[2].norm().log10();
        assert!(
            (drawn[sample] - expected).abs() < 1.0e-9,
            "{} dB at sample {sample}, expected {expected}",
            drawn[sample]
        );
    }
    for trace in &model.traces {
        assert!(
            view::is_coefficient(&trace.source_waveform_name),
            "{} is not a coefficient",
            trace.source_waveform_name
        );
    }
    assert!(model.signal_trace_count <= model.traces.len());
}

/// The K and μ curves the Plot menu adds read, at each of three
/// frequencies, exactly what the Smith chart's two-port marker reads there.
#[test]
fn the_stability_curves_read_what_the_smith_marker_reads() {
    let analysis = amplifier();
    let Fetched::Ready(Some(sweep)) = TwoPortSweep::resolve(&analysis.data) else {
        panic!("the amplifier is a two-port");
    };
    let context = WaveformsContext::new(&analysis.waveforms);
    let curve = |text: &str| {
        let parsed = try_parse(text, |text| {
            text.parse().map_err(|error| format!("{error}"))
        })
        .expect("the menu's expression parses");
        match evaluate(&parsed, &context).expect("the menu's expression evaluates") {
            CalcValue::Real(RealValue::Waveform(x, values)) => {
                assert_eq!(x, FREQUENCIES);
                values
            }
            other => panic!("{text} returned {other:?}"),
        }
    };
    let state = finished_run(analysis.clone());
    let (_, groups) = offer(state.results_reads()).expect("the run offers the Plot menu");
    let expression = |label: &str| {
        groups
            .iter()
            .flat_map(|group| &group.entries)
            .find(|entry| entry.label == label)
            .map(|entry| entry.expression.clone())
            .expect("offered")
    };
    let (k, mu, mu_prime) = (
        curve(&expression("K")),
        curve(&expression("μ")),
        curve(&expression("μ′")),
    );
    for marker in [0.5e9, 1.9e9, 4.2e9] {
        let index = sweep
            .nearest_index(marker)
            .expect("a sample near the marker");
        let point = &sweep.scattering[index];
        for (name, curve, marker_value) in [
            ("K", &k, point.rollett_k()),
            ("μ", &mu, point.mu_load()),
            ("μ′", &mu_prime, point.mu_source()),
        ] {
            assert!(
                (curve[index] - marker_value).abs() <= 1.0e-12 * marker_value.abs(),
                "{name} at {marker} Hz: curve {} against marker {marker_value}",
                curve[index]
            );
        }
    }
}

/// A Plot menu entry adds its function to the analysis' strip, and picking
/// it again hides and shows it rather than adding a second.
#[test]
fn a_plot_entry_adds_its_curve_once_then_hides_and_shows_it() {
    let mut state = finished_run(amplifier());
    state.results.results.session.viewer = ResultViewer::SParameters;
    let key = analysis_key(&state);
    let (_, groups) = offer(state.results_reads()).expect("the run offers the Plot menu");
    let noise_figure = groups
        .iter()
        .find(|group| group.title == "Noise")
        .expect("the run kept its noise figures")
        .entries[0]
        .expression
        .clone();
    assert_eq!(noise_figure, "dB10(F)");
    let traces = |state: &ResultsTestHost| {
        state.results.results.session.analysis_exprs[&key]
            .iter()
            .map(|trace| (trace.text.clone(), trace.visible))
            .collect::<Vec<_>>()
    };

    toggle(&mut state.results_parts(), key, noise_figure.clone());
    assert_eq!(traces(&state), [(noise_figure.clone(), true)]);
    toggle(&mut state.results_parts(), key, noise_figure.clone());
    assert_eq!(traces(&state), [(noise_figure.clone(), false)]);
    toggle(&mut state.results_parts(), key, noise_figure.clone());
    assert_eq!(traces(&state), [(noise_figure, true)]);
}
