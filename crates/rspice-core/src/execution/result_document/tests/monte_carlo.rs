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
