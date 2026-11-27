//! A strip's projection of an AC response or a noise spectrum: what
//! choosing one writes down, and what the stack then draws.

use super::*;
use crate::derived::DerivedSeries;
use crate::waves::{NOISE_DENSITY_UNIT, ProjectionOptions};
use rspice_results::analysis_type::AnalysisType;
use rspice_results::executed_deck::ExecutedDeckArchive;
use rspice_results::family_projection::FamilyTraceStyle;
use rspice_results::provenance::{
    AnalysisResultFamilyMember, AnalysisResultProvenance, AnalysisResultStepCoordinate,
};
use rspice_results::run::{ExecutionTarget, SimulationRun};
use rspice_ui_kit::tokens::Tokens;
use rspice_vocabulary::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use std::collections::HashSet;

/// The delay of the line the tests read.
const DELAY: f64 = 1.0e-6;

/// The response of a line delaying by `delay`, `V(out) = e^(−j2πfτ)`, from
/// 1 kHz over `decades` decades at twenty points a decade, retained as a
/// magnitude carrying its complex samples and a phase folded into ±180°.
fn delay_line_waveforms(delay: f64, decades: u32) -> Vec<RetainedTrace> {
    let frequency: Vec<f64> = (0..=20 * decades)
        .map(|index| 1.0e3 * 10f64.powf(f64::from(index) / 20.0))
        .collect();
    let angle: Vec<f64> = frequency.iter().map(|f| -TAU * f * delay).collect();
    let real: Vec<f64> = angle.iter().map(|angle| angle.cos()).collect();
    let imaginary: Vec<f64> = angle.iter().map(|angle| angle.sin()).collect();
    let phase: Vec<f64> = real
        .iter()
        .zip(&imaginary)
        .map(|(re, im)| im.atan2(*re).to_degrees())
        .collect();
    vec![
        RetainedTrace::new("|V(out)|", frequency.clone(), vec![1.0; real.len()], "")
            .with_unit("V")
            .with_complex_components("V(out)", real, imaginary),
        RetainedTrace::new("phase(V(out))", frequency, phase, ""),
    ]
}

/// An AC run of a 1 µs delay line from 1 kHz to 1 MHz.
fn delay_line() -> SimulationRun<AnalysisResult<RetainedTrace>> {
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Ac, "AC", 0.0)
            .with_waveforms(delay_line_waveforms(DELAY, 3)),
    );
    run
}

/// A stepped AC run of a delay line, numbered `id`: point `k` of five, at
/// `c` = k + 1 nF, delays by `scale` · (k + 1) µs, and is solved as the task
/// `instances[k]` names, as every run of one plan solves it.
fn stepped_delay_lines(
    id: u64,
    scale: f64,
    instances: &[AnalysisInstanceId; 5],
) -> SimulationRun<AnalysisResult<RetainedTrace>> {
    let mut run = SimulationRun::new(id, 0.0, ExecutionTarget::LocalDesktop);
    for (index, instance) in instances.iter().enumerate() {
        let point = index as f64 + 1.0;
        let member = AnalysisResultFamilyMember::StepPoint {
            index,
            coordinate: vec![AnalysisResultStepCoordinate {
                name: "c".to_owned(),
                value: point * 1.0e-9,
                unit: String::new(),
            }],
        };
        let provenance = AnalysisResultProvenance::new(
            *instance,
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([7; 32]),
            Vec::new(),
        )
        .unwrap()
        .with_family_member(Some(member.clone()))
        .unwrap();
        run.analyses.push(
            AnalysisResult::new(
                index as u64 + 1,
                AnalysisType::Ac,
                format!("AC · {}", member.label()),
                0.0,
            )
            .with_waveforms(delay_line_waveforms(scale * point * DELAY, 2))
            .with_provenance(provenance),
        );
    }
    run
}

/// The strips of `runs`, the active run first and its overlays after it.
fn models_of(runs: &[&SimulationRun<AnalysisResult<RetainedTrace>>]) -> Vec<StripModel> {
    models_under(runs, None)
}

/// The strips of `runs` drawn under the sample selection `selection`.
fn models_under(
    runs: &[&SimulationRun<AnalysisResult<RetainedTrace>>],
    selection: Option<&SourceSampleSelection>,
) -> Vec<StripModel> {
    crate::waves::build_models(
        runs,
        &ExecutedDeckArchive::default(),
        &mut DerivedSeries::default(),
        &Tokens::default(),
        ProjectionOptions {
            phase_continuous: false,
            complex_display: Default::default(),
            ac: Default::default(),
            selection,
            hidden_family_traces: &HashSet::new(),
            waveform_visibility: &Default::default(),
            signals: None,
        },
        |_| None,
    )
}

fn model_of(run: &SimulationRun<AnalysisResult<RetainedTrace>>) -> StripModel {
    models_of(&[run]).remove(0)
}

/// What each strip lists, top to bottom: each axis' unit and members.
fn listing(strips: &[Strip]) -> Vec<Vec<(String, Vec<StripMember>)>> {
    strips
        .iter()
        .map(|strip| {
            strip
                .axes
                .iter()
                .map(|axis| (axis.unit.clone(), axis.members.clone()))
                .collect()
        })
        .collect()
}

fn signal(name: &str, form: SignalForm) -> StripMember {
    StripMember::Signal {
        name: name.to_owned(),
        form,
    }
}

/// The group delay of a pure delay is that delay at every sample, however
/// the samples are spaced and however often the phase wraps.
#[test]
fn a_delay_lines_group_delay_is_its_delay_at_every_sample() {
    let frequency = [1.0e3, 2.0e3, 5.0e3, 1.1e4, 3.0e5, 7.0e5, 1.0e6];
    let angle: Vec<f64> = frequency.iter().map(|f| -TAU * f * DELAY).collect();
    let real: Vec<f64> = angle.iter().map(|angle| angle.cos()).collect();
    let imaginary: Vec<f64> = angle.iter().map(|angle| angle.sin()).collect();
    for delay in group_delay(&frequency, &real, &imaginary) {
        assert!((delay / DELAY - 1.0).abs() < 1.0e-9, "{delay}");
    }
}

/// Showing the magnitude strip of a delay line as group delay lists the
/// signal's group delay on a seconds axis there and leaves the phase strip
/// as it was; the stack then draws a flat 1 µs and no decibel curve.
#[test]
fn a_strip_shown_as_group_delay_draws_the_delay_lines_delay() {
    let run = delay_line();
    let mut model = model_of(&run);
    assert_eq!(drawn_projection(&model, 0), Some(StripProjection::Decibels));
    assert_eq!(drawn_projection(&model, 1), Some(StripProjection::Phase));

    let strips = set_projection(&model, 0, StripProjection::GroupDelay).expect("a delay strip");
    assert_eq!(
        listing(&strips),
        [
            vec![(
                "s".to_owned(),
                vec![signal("|V(out)|", SignalForm::GroupDelay)]
            )],
            vec![(
                "\u{b0}".to_owned(),
                vec![signal("phase(V(out))", SignalForm::Phase)]
            )],
        ]
    );

    let arranged = HashMap::from([(model.analysis_key, strips.clone())]);
    let mut models = vec![model];
    project_strips(&mut models, &[&run], &arranged, None);
    model = models.remove(0);
    model.arrangement = strips;
    let kinds: Vec<u8> = model.traces.iter().map(|trace| trace.kind as u8).collect();
    assert_eq!(
        kinds,
        [TraceKind::PhaseDeg as u8, TraceKind::GroupDelay as u8]
    );
    let delay = &model.traces[1];
    assert_eq!(delay.name, "groupDelay(V(out))");
    assert_eq!(model.trace_unit(delay), "s");
    let samples = whole(&delay.y).expect("the delay's samples");
    assert_eq!(samples.len(), 61);
    for sample in samples {
        assert!((sample / DELAY - 1.0).abs() < 1.0e-9, "{sample}");
    }
    assert_eq!(
        drawn_projection(&model, 0),
        Some(StripProjection::GroupDelay)
    );
}

/// Each projection lists the signal in its own form, on an axis of its own
/// unit, and the phase reads off the retained phase.
#[test]
fn each_projection_lists_the_signal_in_its_own_form_and_unit() {
    let model = model_of(&delay_line());
    let shown = |projection| listing(&set_projection(&model, 0, projection).unwrap())[0].clone();
    assert_eq!(
        shown(StripProjection::Magnitude),
        [("V".to_owned(), vec![signal("|V(out)|", SignalForm::Value)])]
    );
    assert_eq!(
        shown(StripProjection::Real),
        [("V".to_owned(), vec![signal("|V(out)|", SignalForm::Real)])]
    );
    assert_eq!(
        shown(StripProjection::Phase),
        [(
            "\u{b0}".to_owned(),
            vec![signal("phase(V(out))", SignalForm::Phase)]
        )]
    );
}

/// What a curve of a family is told apart by: whether it is an overlay, its
/// member, its colour, its member's style, its member's identity and its
/// name.
type CurveIdentity = (
    bool,
    Option<usize>,
    [u8; 4],
    Option<FamilyTraceStyle>,
    u64,
    String,
);

fn identity(trace: &StripTrace) -> CurveIdentity {
    (
        trace.overlay,
        trace.family_group_ordinal,
        trace.color.to_array(),
        trace.family_style,
        trace.presentation_key,
        trace.name.clone(),
    )
}

/// A stepped AC run with its magnitude strip shown as group delay draws
/// every point's delay, in the point's colour and style, under its name and
/// with a visibility of its own; an overlaid run of the same plan draws
/// every point of its own solve the same way. No decibel curve is left.
#[test]
fn a_stepped_runs_group_delay_strip_draws_every_member_and_overlay() {
    let instances: [AnalysisInstanceId; 5] = std::array::from_fn(|_| AnalysisInstanceId::new());
    let shown = stepped_delay_lines(1, 1.0, &instances);
    let earlier = stepped_delay_lines(2, 2.0, &instances);
    let runs = [&shown, &earlier];
    let mut models = models_of(&runs);
    let [model] = models.as_slice() else {
        panic!("one strip for the family, got {}", models.len());
    };
    let analysis_key = model.analysis_key;
    let decibels: Vec<CurveIdentity> = model
        .traces
        .iter()
        .filter(|trace| trace.kind == TraceKind::MagnitudeDb)
        .map(identity)
        .collect();
    assert_eq!(decibels.len(), 10, "five points, each of two runs");
    assert_eq!(drawn_projection(model, 0), Some(StripProjection::Decibels));

    let strips = set_projection(model, 0, StripProjection::GroupDelay).expect("a delay strip");
    let arranged = HashMap::from([(analysis_key, strips.clone())]);
    project_strips(&mut models, &runs, &arranged, None);
    let model = &mut models[0];
    model.arrangement = strips;

    assert!(
        model
            .traces
            .iter()
            .all(|trace| trace.kind != TraceKind::MagnitudeDb),
        "the strip lists no decibel curve"
    );
    let delays: Vec<&StripTrace> = model
        .traces
        .iter()
        .filter(|trace| trace.kind == TraceKind::GroupDelay)
        .collect();
    assert_eq!(delays.len(), 10, "one delay for each curve it replaced");
    for (delay, before) in delays.iter().zip(&decibels) {
        let (overlay, ordinal, color, style, key, name) = before;
        assert_eq!(delay.overlay, *overlay);
        assert_eq!(delay.family_group_ordinal, *ordinal);
        assert_eq!(delay.color.to_array(), *color, "{name}: the point's colour");
        assert_eq!(delay.family_style, *style, "{name}: the point's style");
        assert_eq!(delay.presentation_key, *key);
        assert_eq!(delay.run_id, if *overlay { 2 } else { 1 });
        assert_eq!(
            delay.name,
            name.replace("|V(out)|", "groupDelay(V(out))"),
            "named by its point"
        );
        assert_eq!(
            delay.family_visibility_key,
            (!overlay).then(|| FamilyTraceVisibilityKey::new(
                analysis_key,
                *key,
                "|V(out)|",
                TraceKind::GroupDelay
            )),
            "{name}: each point hides on its own, an overlay with its point"
        );
        assert_eq!(model.trace_unit(delay), "s");
        let point = ordinal.expect("a member of the family") as f64 + 1.0;
        let expected = point * DELAY * if *overlay { 2.0 } else { 1.0 };
        for sample in whole(&delay.y).expect("the delay's samples") {
            assert!((sample / expected - 1.0).abs() < 1.0e-9, "{name}: {sample}");
        }
    }
    // The active run's curves stand before the overlays', where the stack
    // counts its signals.
    assert_eq!(model.signal_trace_count, 10, "five phases and five delays");
    let (signals, overlays) = model.traces.split_at(model.signal_trace_count);
    assert!(signals.iter().all(|trace| !trace.overlay));
    assert!(overlays.iter().all(|trace| trace.overlay));
    assert_eq!(
        drawn_projection(model, 0),
        Some(StripProjection::GroupDelay)
    );
}

/// Under a sample selection the strip draws the selected rows, and a
/// projection made for it reads those rows alone: the delay line's group
/// delay at every other frequency is still its delay, at each of them.
#[test]
fn a_projection_under_a_sample_selection_reads_the_selected_rows() {
    let run = delay_line();
    let rows: Vec<usize> = (0..61).step_by(2).collect();
    let selection = SourceSampleSelection::new(run.dataset_id, run.analyses[0].id, rows.clone())
        .expect("ascending rows");
    let mut models = models_under(&[&run], Some(&selection));
    assert_eq!(models[0].traces[0].x.len(), rows.len());
    let strips = set_projection(&models[0], 0, StripProjection::GroupDelay).expect("a delay strip");
    let arranged = HashMap::from([(models[0].analysis_key, strips)]);
    project_strips(&mut models, &[&run], &arranged, Some(&selection));
    let delay = models[0]
        .traces
        .iter()
        .find(|trace| trace.kind == TraceKind::GroupDelay)
        .expect("a delay curve");
    let samples = whole(&delay.y).expect("the delay's samples");
    assert_eq!(samples.len(), rows.len());
    for sample in samples {
        assert!((sample / DELAY - 1.0).abs() < 1.0e-9, "{sample}");
    }
}

/// A noise run's output spectrum, `onoise`, retained as a power density in
/// V²/Hz, falling a decade of density a decade of frequency.
fn noise_run() -> SimulationRun<AnalysisResult<RetainedTrace>> {
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Noise, "Noise", 0.0).with_waveforms(vec![
            RetainedTrace::new(
                "onoise",
                vec![1.0e1, 1.0e2, 1.0e3, 1.0e4],
                vec![4.0e-14, 4.0e-16, 4.0e-18, 4.0e-20],
                "",
            )
            .with_unit("V^2/Hz"),
        ]),
    );
    run
}

/// A noise strip shown as power density draws the square of the amplitude
/// density it showed, in V²/Hz on an axis of that unit, and offers no
/// complex form; shown as amplitude density again, it lists the density.
#[test]
fn a_noise_strip_shown_as_power_density_reads_the_square_of_its_density() {
    let run = noise_run();
    let mut model = model_of(&run);
    assert_eq!(drawn_projection(&model, 0), Some(StripProjection::Density));
    assert_eq!(model.trace_unit(&model.traces[0]), NOISE_DENSITY_UNIT);
    let density = whole(&model.traces[0].y).expect("the density's samples");
    assert!(set_projection(&model, 0, StripProjection::GroupDelay).is_err());

    let strips =
        set_projection(&model, 0, StripProjection::PowerDensity).expect("a power density strip");
    assert_eq!(
        listing(&strips),
        [vec![(
            "V\u{b2}/Hz".to_owned(),
            vec![signal("onoise", SignalForm::NoisePower)]
        )]]
    );
    let arranged = HashMap::from([(model.analysis_key, strips.clone())]);
    let mut models = vec![model];
    project_strips(&mut models, &[&run], &arranged, None);
    model = models.remove(0);
    model.arrangement = strips;

    let [power] = model.traces.as_slice() else {
        panic!("one curve, got {}", model.traces.len());
    };
    assert!(power.kind == TraceKind::NoisePower);
    assert_eq!(model.trace_unit(power), "V\u{b2}/Hz");
    let samples = whole(&power.y).expect("the power density's samples");
    assert_eq!(samples.len(), density.len());
    for (power, density) in samples.iter().zip(&density) {
        // The density reads in nV/√Hz.
        let squared = (density * 1.0e-9).powi(2);
        assert!(
            (power / squared - 1.0).abs() < 1.0e-12,
            "{power} against {squared}"
        );
    }
    assert_eq!(
        drawn_projection(&model, 0),
        Some(StripProjection::PowerDensity)
    );

    let back = set_projection(&model, 0, StripProjection::Density).expect("a density strip");
    assert_eq!(
        listing(&back),
        [vec![(
            NOISE_DENSITY_UNIT.to_owned(),
            vec![signal("onoise", SignalForm::NoiseDensity)]
        )]]
    );
}
