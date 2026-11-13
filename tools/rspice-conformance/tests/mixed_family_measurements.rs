//! A measurement on a Monte Carlo family, evaluated once per trial by the
//! results calculator, against the mixed corpus runner's own per-trial numbers.
//!
//! C6's `vdig` Monte Carlo transient draws L and C per trial. Each trial is
//! handed to the family evaluation as one solved point of the authored
//! analysis, the way a run that solves one task per trial delivers it, and
//! each of the design's per-trial quantities (`vss`, `ripple`, `ilmean` in
//! `tolerance.toml`) is written as the calculator's windowed `avg` or `pp`.
//! The runner's `mean` and `pp` are the same trapezoid and the same window
//! extremes with interpolated ends, so the two must agree to rounding.
//!
//! Release only: the three trials take about two seconds there.

use std::time::Duration;

use rspice_conformance::suites::mixed::case::Observation;
use rspice_conformance::suites::mixed::corpus_root;
use rspice_conformance::suites::mixed::design::{Design, MeasureSpec};
use rspice_conformance::suites::mixed::dispatch::{CaseRequest, run_case};
use rspice_conformance::suites::mixed::measure::{Measured, evaluate};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::calculator::spice_parser;
use rspice_results::family_measurements::{evaluate_members, family_members};
use rspice_results::provenance::{AnalysisResultProvenance, AnalysisResultSourceDomain};
use rspice_results::waveform::RetainedWaveform;
use rspice_vocabulary::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};

const VARIANT: &str = "vdig";
const ANALYSIS: &str = "mc-tran";

/// One authored transient, solved once per trial.
fn family_of(trials: &[Observation]) -> Vec<AnalysisResult> {
    let authored = AnalysisInstanceId::new();
    trials
        .iter()
        .enumerate()
        .map(|(index, trial)| {
            let provenance = AnalysisResultProvenance::new_with_authored_source_domain(
                AnalysisResultSourceDomain::SimulationPlan,
                AnalysisInstanceId::new(),
                authored,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([6; 32]),
                Vec::new(),
            )
            .expect("valid provenance");
            let waveforms = trial
                .real
                .iter()
                .map(|(name, values)| {
                    RetainedWaveform::new(name.as_str(), trial.axis.clone(), values.clone())
                })
                .collect();
            AnalysisResult::new(
                index as u64,
                AnalysisType::Transient,
                format!("Trial {index}"),
                0.0,
            )
            .with_waveforms(waveforms)
            .with_provenance(provenance)
        })
        .collect()
}

/// The calculator's spelling of a windowed `mean` or `pp` quantity.
fn calculator_expression(spec: &MeasureSpec) -> String {
    let function = match spec.kind.as_str() {
        "mean" => "avg",
        "pp" => "pp",
        other => panic!("no calculator spelling for a {other} quantity"),
    };
    let signal = spec
        .signal
        .as_ref()
        .and_then(|signal| signal.for_variant(VARIANT))
        .expect("a single-signal quantity");
    let [from, to] = spec.window.expect("a windowed quantity");
    format!("{function}({signal}, {from:e}, {to:e})")
}

#[test]
#[cfg_attr(
    debug_assertions,
    ignore = "runs C6's Monte Carlo transient; run with --release"
)]
fn per_trial_family_measurements_match_the_corpus_runner_on_c6() {
    let design = Design::load(&corpus_root().join("c06-buck")).expect("C6 loads");
    let result = run_case(&CaseRequest {
        design: &design,
        variant: VARIANT,
        analysis: ANALYSIS,
        o5: false,
        budget: Duration::from_secs(300),
    });
    assert_eq!(result.outcome, "completed", "{:?}", result.error);
    let trials = &result
        .results
        .iter()
        .find(|observation| observation.kind == "mc")
        .expect("a Monte Carlo observation")
        .trials;
    assert_eq!(trials.len(), 3);
    let family = family_of(trials);
    let members = family_members(&family[0], &family).expect("the trials form one family");
    assert_eq!(members.len(), trials.len());

    let quantities: Vec<_> = design
        .quantities_for(VARIANT, ANALYSIS)
        .filter(|quantity| quantity.of.is_some())
        .collect();
    let ids: Vec<&str> = quantities.iter().map(|q| q.id.as_str()).collect();
    assert_eq!(ids, ["vss", "ripple", "ilmean"]);
    for quantity in quantities {
        let spec = quantity.of.as_ref().expect("a per-trial measure");
        let runner: Vec<f64> = trials
            .iter()
            .map(|trial| match evaluate(spec, VARIANT, trial) {
                Ok(Measured::Values(values)) if values.len() == 1 => values[0],
                other => panic!("{}: the runner measured {other:?}", quantity.id),
            })
            .collect();
        let expression = calculator_expression(spec);
        let parsed = spice_parser::try_parse(&expression)
            .unwrap_or_else(|error| panic!("{expression}: {error:?}"));
        let measured = evaluate_members(&parsed, &members);
        let values: Vec<f64> = measured
            .members
            .iter()
            .map(|member| {
                member
                    .value
                    .clone()
                    .unwrap_or_else(|reason| panic!("{expression} on {}: {reason}", member.label))
            })
            .collect();
        println!(
            "{} {expression}: runner {runner:?}, family {values:?}",
            quantity.id
        );
        for (trial, (family, runner)) in values.iter().zip(&runner).enumerate() {
            assert!(
                (family - runner).abs() <= 1e-12 * runner.abs(),
                "{} trial {trial}: family {family:e}, runner {runner:e}",
                quantity.id
            );
        }

        // The statistics are the closed forms over the runner's numbers:
        // the mean and the sample (n - 1) standard deviation.
        let n = runner.len() as f64;
        let mean = runner.iter().sum::<f64>() / n;
        let sigma = (runner.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
        let statistics = measured.statistics().expect("three measured trials");
        assert_eq!(statistics.count, 3);
        assert_eq!(statistics.unmeasured, 0);
        assert!((statistics.mean - mean).abs() <= 1e-12 * mean.abs());
        let reported = statistics.std_dev.expect("sigma of three trials");
        assert!(
            (reported - sigma).abs() <= 1e-9 * sigma,
            "{} sigma: family {reported:e}, closed form {sigma:e}",
            quantity.id
        );
        let min = runner.iter().copied().fold(f64::INFINITY, f64::min);
        let max = runner.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!((statistics.min - min).abs() <= 1e-12 * min.abs());
        assert!((statistics.max - max).abs() <= 1e-12 * max.abs());
    }
}
