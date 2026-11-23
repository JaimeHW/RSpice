//! A strip's projection of an AC response: what choosing one writes down,
//! and what the stack then draws.

use super::*;
use crate::derived::DerivedSeries;
use crate::waves::ProjectionOptions;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::executed_deck::ExecutedDeckArchive;
use rspice_results::run::{ExecutionTarget, SimulationRun};
use rspice_ui_kit::tokens::Tokens;
use std::collections::HashSet;

/// The delay of the line the tests read.
const DELAY: f64 = 1.0e-6;

/// An AC run of a 1 µs delay line, `V(out) = e^(−j2πfτ)`, from 1 kHz to
/// 1 MHz at twenty points a decade, retained as a magnitude carrying its
/// complex samples and a phase folded into ±180°.
fn delay_line() -> SimulationRun<AnalysisResult<RetainedTrace>> {
    let frequency: Vec<f64> = (0..=60)
        .map(|index| 1.0e3 * 10f64.powf(f64::from(index) / 20.0))
        .collect();
    let angle: Vec<f64> = frequency.iter().map(|f| -TAU * f * DELAY).collect();
    let real: Vec<f64> = angle.iter().map(|angle| angle.cos()).collect();
    let imaginary: Vec<f64> = angle.iter().map(|angle| angle.sin()).collect();
    let phase: Vec<f64> = real
        .iter()
        .zip(&imaginary)
        .map(|(re, im)| im.atan2(*re).to_degrees())
        .collect();
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedTrace::new("|V(out)|", frequency.clone(), vec![1.0; 61], "#fff")
                .with_unit("V")
                .with_complex_components("V(out)", real, imaginary),
            RetainedTrace::new("phase(V(out))", frequency, phase, "#fff"),
        ]),
    );
    run
}

fn model_of(run: &SimulationRun<AnalysisResult<RetainedTrace>>) -> StripModel {
    let mut models = crate::waves::build_models(
        &[run],
        &ExecutedDeckArchive::default(),
        &mut DerivedSeries::default(),
        &Tokens::default(),
        ProjectionOptions {
            phase_continuous: false,
            complex_display: Default::default(),
            ac: Default::default(),
            selection: None,
            hidden_family_traces: &HashSet::new(),
            waveform_visibility: &Default::default(),
        },
        |_| None,
    );
    models.remove(0)
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
    project_strips(&mut models, &run, &arranged);
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
