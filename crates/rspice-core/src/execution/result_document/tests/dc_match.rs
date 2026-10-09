use super::*;
use serde_json::{Value, json};

fn document(result: &DcMatchResult) -> AnalysisResultDocument {
    AnalysisResultDocument::from_dc_match(instance(AnalysisKind::DcMatch), result)
        .unwrap()
        .build()
        .unwrap()
}

#[test]
fn mismatch_reports_reject_contradictory_quantities_at_decode_and_construction() {
    let original: Value = serde_json::to_value(document_for(AnalysisResultKind::DcMatch)).unwrap();
    for case in [
        "nominal",
        "total",
        "negative-sigma",
        "zero-multiplier",
        "negative-parameter-sigma",
        "contribution",
        "share",
        "false-share",
        "duplicate",
        "quoted",
        "unit",
        "missing",
        "unavailable",
        "independent-total",
    ] {
        let mut raw = original.clone();
        match case {
            "nominal" => raw["payload"]["nominalValue"] = json!(1.0),
            "total" => raw["payload"]["sigmaTotal"] = json!(1.0),
            "negative-sigma" => raw["payload"]["sigmaMismatch"] = json!(-1.0),
            "zero-multiplier" => raw["payload"]["sigmaMultiplier"] = json!(0.0),
            "negative-parameter-sigma" => {
                raw["payload"]["contributors"][0]["sigmaParameter"] = json!(-0.005)
            }
            "contribution" => raw["payload"]["contributors"][0]["contribution"] = json!(0.0),
            "share" => raw["payload"]["contributors"][0]["share"] = json!(-0.5),
            "false-share" => raw["payload"]["contributors"][0]["share"] = json!(0.25),
            "duplicate" => raw["payload"]["contributors"][1]["instance"] = json!("m1"),
            "quoted" => raw["scalars"][4]["value"]["value"] = json!(1.0),
            "unit" => raw["scalars"][0]["unit"]["unit"] = json!("ampere"),
            "missing" => {
                raw["scalars"].as_array_mut().unwrap().remove(0);
            }
            "unavailable" => raw["scalars"][0]["value"]["value"] = Value::Null,
            "independent-total" => {
                raw["payload"]["sigmaTotal"] = json!(1.0);
                raw["payload"]["sigmaMismatch"] = json!(1.0);
                raw["scalars"][1]["value"]["value"] = json!(1.0);
                raw["scalars"][2]["value"]["value"] = json!(1.0);
                raw["scalars"][4]["value"]["value"] = json!(3.0);
            }
            _ => panic!("case"),
        }
        assert!(
            AnalysisResultDocument::from_json(&raw.to_string()).is_err(),
            "{case}"
        );
        let decoded: AnalysisResultDocument = serde_json::from_value(raw).unwrap();
        assert!(decoded.validate().is_err(), "{case}");
    }
    let mut bad = dc_match_result();
    bad.contributors[0].contribution = 0.0;
    assert!(
        AnalysisResultDocument::from_dc_match(instance(AnalysisKind::DcMatch), &bad)
            .unwrap()
            .build()
            .is_err()
    );
}

#[test]
fn mismatch_validation_preserves_trimmed_correlated_and_extreme_finite_reports() {
    let mut trimmed = dc_match_result();
    trimmed.contributors.truncate(1);
    document(&trimmed);
    trimmed.contributors.clear();
    document(&trimmed);
    // sqrt(2) subnormal units rounds to one unit. The complete independent
    // report retains enough information to recover the unrounded variance.
    let mut subnormal = dc_match_result();
    subnormal.sigma_multiplier = 1.0;
    subnormal.sigma_total = f64::from_bits(1);
    subnormal.sigma_mismatch = f64::from_bits(1);
    for entry in &mut subnormal.contributors {
        entry.sigma_parameter = f64::from_bits(1);
        entry.sensitivity = 1.0;
        entry.contribution = f64::from_bits(1);
    }
    document(&subnormal);
    subnormal.contributors.truncate(1);
    document(&subnormal);
    for scale in [f64::from_bits(1), 1e-200, 1.0, 1e200, f64::MAX / 16.0] {
        let mut result = dc_match_result();
        result.sigma_multiplier = 1.0;
        for (index, contributor) in result.contributors.iter_mut().enumerate() {
            contributor.scope = if index == 0 {
                DcMatchScope::Mismatch
            } else {
                DcMatchScope::Process
            };
            contributor.sigma_parameter = scale;
            contributor.sensitivity = (index + 3) as f64;
            contributor.contribution = contributor.sensitivity * scale;
            contributor.share = if index == 0 { 0.36 } else { 0.64 };
        }
        result.sigma_mismatch = 3.0 * scale;
        result.sigma_process = 4.0 * scale;
        result.sigma_total = 5.0 * scale;
        let built = document(&result);
        assert_eq!(
            AnalysisResultDocument::from_json(&built.to_json().unwrap()).unwrap(),
            built
        );
    }
    let mut correlated = dc_match_result();
    correlated.sigma_total = 1.0;
    correlated.sigma_mismatch = 1.0;
    correlated.applied_correlations_mismatch = 1;
    for (index, contributor) in correlated.contributors.iter_mut().enumerate() {
        contributor.instance = "M1".into();
        contributor.parameter = format!("P{index}");
        contributor.sigma_parameter = 1.0;
        contributor.sensitivity = if index == 0 { 2.0 } else { -1.0 };
        contributor.contribution = contributor.sensitivity;
        contributor.share = contributor.sensitivity;
    }
    document(&correlated);
    correlated.sigma_total = 0.0;
    correlated.sigma_mismatch = 0.0;
    correlated.contributors[0].sensitivity = 1.0;
    correlated.contributors[0].contribution = 1.0;
    correlated
        .contributors
        .iter_mut()
        .for_each(|entry| entry.share = 0.0);
    document(&correlated);
    correlated.contributors[0].share = 1.0;
    assert!(
        AnalysisResultDocument::from_dc_match(instance(AnalysisKind::DcMatch), &correlated)
            .unwrap()
            .build()
            .is_err()
    );
}

#[test]
fn mismatch_validation_rejects_nonzero_products_that_cannot_be_retained() {
    for (left, right) in [(1e-200, 1e-200), (1e200, 1e200)] {
        let mut result = dc_match_result();
        result.contributors[0].sensitivity = left;
        result.contributors[0].sigma_parameter = right;
        result.contributors[0].contribution = 0.0;
        assert!(
            AnalysisResultDocument::from_dc_match(instance(AnalysisKind::DcMatch), &result)
                .unwrap()
                .build()
                .is_err()
        );
    }
    let mut result = dc_match_result();
    result.sigma_multiplier = f64::from_bits(1);
    assert!(
        AnalysisResultDocument::from_dc_match(instance(AnalysisKind::DcMatch), &result)
            .unwrap()
            .build()
            .is_err()
    );
}

#[test]
fn mismatch_validation_accepts_legacy_independent_reports_and_observes_abort() {
    let document = document_for(AnalysisResultKind::DcMatch);
    let mut raw = serde_json::to_value(&document).unwrap();
    raw["schemaVersion"] = json!(7);
    raw["payload"]
        .as_object_mut()
        .unwrap()
        .remove("appliedCorrelationsMismatch");
    raw["payload"]
        .as_object_mut()
        .unwrap()
        .remove("appliedCorrelationsProcess");
    AnalysisResultDocument::from_json(&raw.to_string()).unwrap();
    assert_eq!(
        super::super::dc_match::validate(&document, &CountingAbort::new(2)),
        Err(ResultDocumentError::Aborted)
    );
}
