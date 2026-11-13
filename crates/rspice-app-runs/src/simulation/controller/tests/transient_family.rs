//! A deck's Monte Carlo or `.STEP` transient runs through the controller as
//! one retained transient per member, each the member the corpus runner's
//! headless route solves.
use std::path::{Path, PathBuf};

use crate::state::{AnalysisResult, SimulationRunLifecycle};
use rspice_results::provenance::AnalysisResultFamilyMember;

fn corpus_deck(relative: &str) -> (String, PathBuf) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/mixed-corpus")
        .join(relative);
    let source = std::fs::read_to_string(&path).expect("corpus deck");
    (source, path)
}

fn signal<'a>(analysis: &'a AnalysisResult, name: &str) -> &'a crate::state::RetainedTrace {
    analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("{name} in {}", analysis.label))
}

/// The value the signal holds at `time`: its last sample at or before it.
fn held_at(analysis: &AnalysisResult, name: &str, time: f64) -> f64 {
    let waveform = signal(analysis, name);
    let last = waveform.x.iter().rposition(|t| *t <= time).expect("sample");
    waveform.y[last]
}

fn member(analysis: &AnalysisResult) -> &AnalysisResultFamilyMember {
    analysis
        .provenance()
        .and_then(|provenance| provenance.family_member())
        .unwrap_or_else(|| panic!("{} names no member", analysis.label))
}

#[test]
fn a_step_transient_keeps_one_transient_per_point_at_the_points_value() {
    let (source, path) = corpus_deck("probes/p06b-mc-step-mixed/deck/base.step-tran.cir");
    let run = super::super::test_execution::run_manual_batch_at(
        &source,
        &path,
        SimulationRunLifecycle::Completed,
    );
    let points: Vec<_> = run.analyses.iter().collect();
    assert_eq!(
        points.len(),
        3,
        "{:?}",
        run.analyses.iter().map(|a| &a.label).collect::<Vec<_>>()
    );
    for (index, (analysis, r)) in points.iter().zip([1e3, 2e3, 3e3]).enumerate() {
        let AnalysisResultFamilyMember::StepPoint {
            index: point,
            coordinate,
        } = member(analysis)
        else {
            panic!("{:?}", member(analysis));
        };
        assert_eq!(*point, index);
        assert_eq!(coordinate.len(), 1);
        assert_eq!((coordinate[0].name.as_str(), coordinate[0].value), ("r", r));
        // The corpus runner's per-point reading (P06b ivp, abs 1e-9): 1 mA
        // through the module plus 1 V through the point's own r.
        let ivp = held_at(analysis, "i(vp)", 5e-8);
        assert!(
            (ivp + 1e-3 + 1.0 / r).abs() < 1e-9,
            "point {index}: {ivp:e}"
        );
    }
    assert_eq!(points[1].label, "Transient · r = 2 k");
}

#[test]
fn a_monte_carlo_transient_keeps_one_transient_per_trial_as_drawn() {
    let (source, path) = corpus_deck("probes/p06b-mc-step-mixed/deck/base.mc-tran.cir");
    let run = super::super::test_execution::run_manual_batch_at(
        &source,
        &path,
        SimulationRunLifecycle::Completed,
    );
    assert_eq!(run.analyses.len(), 4);
    // Each trial's draw, read off the deck by the sampler the corpus runner
    // reads it with.
    let netlist = rspice_core::Netlist::parse_with_path(&source, &path).unwrap();
    let members = rspice_simulation::deck_family::monte_carlo_members(&netlist, true).unwrap();
    let draws = rspice_simulation::deck_family::member_parameters(
        &rspice_core::Engine::new(Default::default()),
        &netlist,
        &members,
    )
    .unwrap();
    for (index, (analysis, draw)) in run.analyses.iter().zip(&draws).enumerate() {
        assert_eq!(
            member(analysis),
            &AnalysisResultFamilyMember::MonteCarloTrial { index }
        );
        let ivp = held_at(analysis, "i(vp)", 5e-8);
        assert!(
            (ivp + 1e-3 + 1.0 / draw["r"]).abs() < 1e-9,
            "trial {index}: {ivp:e}"
        );
    }
    // The corpus runner's headless-route reading of trial 0.
    let trial0 = held_at(&run.analyses[0], "i(vp)", 5e-8);
    assert!((trial0 - -2.072789e-3).abs() < 5e-10, "{trial0:e}");
}

/// Peak-to-peak of `name` over `[start, stop]`, the corpus runner's `pp`:
/// the samples inside the window and the signal interpolated at its ends.
fn peak_to_peak(analysis: &AnalysisResult, name: &str, start: f64, stop: f64) -> f64 {
    let waveform = signal(analysis, name);
    let at = |time: f64| {
        let right = waveform.x.partition_point(|t| *t < time);
        let left = right.saturating_sub(1);
        let (t0, t1) = (waveform.x[left], waveform.x[right]);
        let (y0, y1) = (waveform.y[left], waveform.y[right]);
        if t1 == t0 {
            y1
        } else {
            y0 + (y1 - y0) * (time - t0) / (t1 - t0)
        }
    };
    let window = waveform
        .x
        .iter()
        .zip(waveform.y.iter())
        .filter(|(t, _)| **t > start && **t < stop)
        .map(|(_, y)| *y)
        .chain([at(start), at(stop)]);
    let (low, high) = window.fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), y| {
        (low.min(y), high.max(y))
    });
    high - low
}

/// The bytes the run's member transients retain, each shared sample array
/// counted once.
fn retained_bytes(analyses: &[AnalysisResult]) -> usize {
    let mut seen = std::collections::HashSet::new();
    let mut bytes = 0;
    for waveform in analyses.iter().flat_map(|analysis| &analysis.waveforms) {
        for values in [&waveform.x, &waveform.y] {
            if seen.insert(std::sync::Arc::as_ptr(values) as *const u8 as usize) {
                bytes += values.len() * std::mem::size_of::<f64>();
            }
        }
    }
    bytes
}

#[test]
fn a_buck_converters_monte_carlo_trials_are_the_corpus_runners_trials() {
    let (source, path) = corpus_deck("c06-buck/deck/vdig.mc-tran.cir");
    let run = super::super::test_execution::run_manual_batch_at(
        &source,
        &path,
        SimulationRunLifecycle::Completed,
    );
    assert_eq!(run.analyses.len(), 3);
    for (index, analysis) in run.analyses.iter().enumerate() {
        assert_eq!(
            member(analysis),
            &AnalysisResultFamilyMember::MonteCarloTrial { index }
        );
    }
    // The corpus runner's headless-route ripple for each trial (C6 ripple,
    // pp i(lbuck) over the settled 400-500 us window), read off its case
    // result for the same deck: the app solves the members it solves.
    for (index, expected) in [
        (0, 0.130_747_345_957_544_23),
        (1, 0.096_596_109_459_500_26),
        (2, 0.125_145_032_438_699_9),
    ] {
        let ripple = peak_to_peak(&run.analyses[index], "i(lbuck)", 4e-4, 5e-4);
        assert!(
            ((ripple - expected) / expected).abs() < 1e-9,
            "trial {index}: {ripple:e}"
        );
    }
    // A trial holds one time axis and one array per signal, each once: the
    // 500-trial run of this deck holds 500 of these, not two copies of each.
    let expected: usize = run
        .analyses
        .iter()
        .map(|analysis| {
            (analysis.waveforms.len() + 1) * analysis.waveforms[0].x.len() * size_of::<f64>()
        })
        .sum();
    assert_eq!(retained_bytes(&run.analyses), expected);
}
