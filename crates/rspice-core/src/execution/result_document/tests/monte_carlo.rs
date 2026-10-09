use super::*;
use serde_json::{Value, json};

fn scalar<'a>(document: &'a mut Value, name: &str) -> &'a mut Value {
    document["scalars"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|scalar| scalar["name"] == name)
        .unwrap()
}

#[test]
fn monte_carlo_rejects_contradictory_campaigns_and_confidence() {
    let mut result = monte_carlo_result();
    result.successful_trial_indices = Some(vec![7, 8, 9]);
    result.sampling = Some(MonteCarloSampling {
        first_trial: 7,
        seed: u64::MAX,
        policy: "test",
    });
    result
        .compute_mean_confidence(
            95.0,
            crate::analysis::monte_carlo::MeanConfidenceMethod::StudentT,
            crate::ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
    let document = AnalysisResultDocument::from_monte_carlo_with_units(
        instance(AnalysisKind::MonteCarlo),
        &result,
        |_| Some(SignalUnit::Volt),
    )
    .unwrap()
    .build()
    .unwrap();
    let original: Value = serde_json::from_str(&document.to_json().unwrap()).unwrap();
    for case in [
        "failed",
        "converged",
        "successful",
        "completed",
        "count-type",
        "range",
        "overflow",
        "population",
        "duplicate",
        "negative-sigma",
        "extrema",
        "extrema-evidence",
        "mean-evidence",
        "histogram-total",
        "histogram-membership",
        "histogram-empty-interval",
        "histogram-order",
        "histogram-end",
        "histogram-empty",
        "histogram-overflow",
        "level",
        "conditional",
        "reversed-ci",
        "missing-bound",
        "false-state",
        "ci-unit",
        "unknown-variable",
    ] {
        let mut wire = original.clone();
        match case {
            "failed" => scalar(&mut wire, "failed_runs")["value"]["value"] = json!(999),
            "converged" => scalar(&mut wire, "all_converged")["value"]["value"] = json!(false),
            "successful" => scalar(&mut wire, "successful_runs")["value"]["value"] = json!(2),
            "completed" => scalar(&mut wire, "completed_runs")["value"]["value"] = json!(2),
            "count-type" => {
                scalar(&mut wire, "completed_runs")["value"] =
                    json!({"representation":"real","value":3.0})
            }
            "range" => wire["payload"]["successfulTrialIndices"][2] = json!(10),
            "overflow" => scalar(&mut wire, "first_trial")["value"]["value"] = json!(u64::MAX),
            "population" => {
                wire["payload"]
                    .as_object_mut()
                    .unwrap()
                    .remove("successfulTrialIndices");
                wire["payload"]["statistics"][0]["samples"] = json!([1.0, 2.0]);
            }
            "duplicate" => {
                let variable = wire["payload"]["statistics"][0].clone();
                wire["payload"]["statistics"]
                    .as_array_mut()
                    .unwrap()
                    .push(variable);
            }
            "negative-sigma" => wire["payload"]["statistics"][0]["standardDeviation"] = json!(-0.1),
            "extrema" => wire["payload"]["statistics"][0]["maximum"] = json!(0.0),
            "extrema-evidence" => wire["payload"]["statistics"][0]["maximum"] = json!(2.0),
            "mean-evidence" => wire["payload"]["statistics"][0]["mean"] = json!(2.0),
            "histogram-total" => wire["payload"]["statistics"][0]["histogram"] = json!([1, 999]),
            "histogram-membership" => wire["payload"]["statistics"][0]["histogram"] = json!([2, 1]),
            "histogram-empty-interval" => {
                wire["payload"]["statistics"][0]["binEdges"] = json!([0.9, 0.9, 1.1]);
                wire["payload"]["statistics"][0]["samples"][0] = Value::Null;
            }
            "histogram-order" => {
                wire["payload"]["statistics"][0]["binEdges"] = json!([0.9, 0.8, 1.1])
            }
            "histogram-end" => {
                wire["payload"]["statistics"][0]["binEdges"] = json!([0.9, 1.0, 1.05])
            }
            "histogram-empty" => wire["payload"]["statistics"][0]["histogram"] = json!([]),
            "histogram-overflow" => {
                wire["payload"]["statistics"][0]["histogram"] = json!([usize::MAX, usize::MAX])
            }
            "level" => {
                scalar(&mut wire, "mean_confidence_level_pct")["value"]["value"] = json!(100.0)
            }
            "conditional" => {
                scalar(&mut wire, "mean_confidence_conditional_on_success")["value"]["value"] =
                    json!(true)
            }
            "reversed-ci" => {
                scalar(&mut wire, "mean_confidence_lower:56286f757429")["value"]["value"] =
                    json!(100.0)
            }
            "missing-bound" => wire["scalars"]
                .as_array_mut()
                .unwrap()
                .retain(|scalar| scalar["name"] != "mean_confidence_upper:56286f757429"),
            "false-state" => {
                scalar(&mut wire, "mean_confidence_state:56286f757429")["value"]["value"] =
                    json!("insufficient_samples")
            }
            "ci-unit" => {
                scalar(&mut wire, "mean_confidence_lower:56286f757429")["unit"] =
                    json!({"unit":"ampere"})
            }
            "unknown-variable" => {
                scalar(&mut wire, "mean_confidence_lower:56286f757429")["name"] =
                    json!("mean_confidence_lower:deadbeef")
            }
            _ => panic!("unknown mutation: {case}"),
        }
        assert!(
            AnalysisResultDocument::from_json(&wire.to_string()).is_err(),
            "accepted {case}"
        );
    }
    // Legacy absence is supported without fabricating lost identities or units.
    let mut legacy = original;
    legacy["schemaVersion"] = json!(17);
    legacy["payload"]
        .as_object_mut()
        .unwrap()
        .remove("successfulTrialIndices");
    for variable in legacy["payload"]["statistics"].as_array_mut().unwrap() {
        variable.as_object_mut().unwrap().remove("unit");
    }
    legacy["scalars"].as_array_mut().unwrap().retain(|scalar| {
        !scalar["name"]
            .as_str()
            .unwrap()
            .starts_with("mean_confidence")
    });
    assert!(AnalysisResultDocument::from_json(&legacy.to_string()).is_ok());
}

#[test]
fn monte_carlo_histogram_validation_preserves_missingness_and_legacy_intervals() {
    let document = AnalysisResultDocument::from_monte_carlo(
        instance(AnalysisKind::MonteCarlo),
        &monte_carlo_result(),
    )
    .unwrap()
    .build()
    .unwrap();
    for (edges, counts) in [
        (vec![0.8, 0.95, 1.01, 1.2], vec![1, 1, 1]),
        (vec![0.9, 1.0, 1.0, 1.1], vec![1, 0, 2]),
    ] {
        let mut document = document.clone();
        let ResultPayload::MonteCarlo(payload) = &mut document.payload else {
            panic!("Monte Carlo payload expected");
        };
        payload.statistics[0].bin_edges = edges;
        payload.statistics[0].histogram = counts;
        document.schema_version = 17;
        document.validate().unwrap();
        let ResultPayload::MonteCarlo(payload) = &mut document.payload else {
            panic!("Monte Carlo payload expected");
        };
        payload.statistics[0].samples[1] = None;
        let restored = AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap();
        assert_eq!(restored, document);
    }
    for samples in [vec![], vec![1.0; 3]] {
        let mut result = MonteCarloResult::new();
        result.num_runs = samples.len();
        result.all_converged = true;
        result.variables.insert(
            "out".into(),
            VariableStatistics::from_samples("out", samples, 20),
        );
        let document =
            AnalysisResultDocument::from_monte_carlo(instance(AnalysisKind::MonteCarlo), &result)
                .unwrap()
                .build()
                .unwrap();
        let restored = AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap();
        assert_eq!(restored, document);
    }
    let mut missing = document;
    let ResultPayload::MonteCarlo(payload) = &mut missing.payload else {
        panic!("Monte Carlo payload expected");
    };
    let statistics = &mut payload.statistics[0];
    statistics.samples.fill(None);
    statistics.mean = None;
    statistics.standard_deviation = None;
    statistics.minimum = None;
    statistics.maximum = None;
    statistics.histogram.clear();
    statistics.bin_edges.clear();
    let restored = AnalysisResultDocument::from_json(&missing.to_json().unwrap()).unwrap();
    assert_eq!(restored, missing);
}

#[test]
fn monte_carlo_population_validation_honors_cancellation() {
    let mut result = MonteCarloResult::new();
    result.num_runs = 512;
    result.all_converged = true;
    result.variables.insert(
        "out".into(),
        VariableStatistics::from_samples("out", (0..512).map(f64::from).collect(), 128),
    );
    let document =
        AnalysisResultDocument::from_monte_carlo(instance(AnalysisKind::MonteCarlo), &result)
            .unwrap()
            .build()
            .unwrap();
    let count = CountingAbort::new(usize::MAX);
    document.validate_with_abort(&count).unwrap();
    for threshold in 0..count.count() {
        let abort = CountingAbort::new(threshold);
        assert_eq!(
            document.validate_with_abort(&abort),
            Err(ResultDocumentError::Aborted)
        );
        assert_eq!(abort.polls_after_abort(), 0);
    }
}
