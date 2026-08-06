//! Realizes case drafts into full qualification cases by running the
//! engine through the adapter library — the exact code path the shipped
//! worker component executes.
//!
//! The split of authority is deliberate: `expected_decimal` and the
//! tolerances come from the draft's engine-independent oracle, while
//! `sample_count` and `series_sha256` are captured from the engine, because
//! release admission requires observed series identity to equal the oracle's
//! exactly. Accuracy is proven against independent truth; the hashes pin
//! bit-exact reproducibility of that already-proven answer. A draft whose
//! engine value strays outside its own tolerance fails generation here,
//! with the case name in the error.

use rspice_engine_adapter::document::CircuitContent;
use rspice_engine_adapter::execute::execute;
use rspice_engine_adapter::measure::canonical_decimal;
use rspice_engine_adapter::wire::EngineResponse;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::canonical::canonical_sha256;
use crate::contract;
use crate::decimal::within_tolerance;

/// One engine-independent expectation about a named engine measurement.
pub struct Probe {
    /// Adapter measurement name, e.g. `v(out)`.
    pub name: String,
    pub unit: &'static str,
    /// The oracle value from closed-form analysis or reference data —
    /// never from the engine under test.
    pub expected: f64,
    pub absolute_tolerance: &'static str,
    pub relative_tolerance: &'static str,
}

/// One case-specification parameter: the physical quantities that make
/// this case distinct within its family.
pub struct Parameter {
    pub name: &'static str,
    pub unit: &'static str,
    pub value: f64,
}

/// What a case family asserts about its deck before capture.
pub enum Expectation {
    /// The engine must succeed and every probe must land within its own
    /// tolerance of the independent oracle value.
    Succeeds(Vec<Probe>),
    /// The engine must fail with exactly this stable failure code.
    FailsWith(&'static str),
}

/// A family-authored case before engine capture.
pub struct CaseDraft {
    /// Strictly ascending across the corpus; also the fixture file stem.
    pub id: String,
    pub primary_category: &'static str,
    /// Secondary categories; the primary is inserted and sorted here.
    pub extra_categories: Vec<&'static str>,
    /// Complete SPICE deck, title line first, LF-only, trailing newline.
    pub deck: String,
    pub parameters: Vec<Parameter>,
    pub temperature_celsius: f64,
    pub repetitions: u64,
    pub expectation: Expectation,
}

/// Runs the draft through the engine and assembles the corpus case value.
pub fn realize(draft: CaseDraft) -> Result<Value, String> {
    let CaseDraft {
        id,
        primary_category,
        extra_categories,
        deck,
        parameters,
        temperature_celsius,
        repetitions,
        expectation,
    } = draft;
    let fail = |detail: String| format!("case {id}: {detail}");

    if !contract::valid_case_id(&id) {
        return Err(fail("identifier violates the case grammar".to_owned()));
    }
    let analysis_kind = contract::analysis_kind_for_primary(primary_category)
        .ok_or_else(|| fail(format!("unknown primary category {primary_category}")))?;
    let engine_kind = contract::engine_analysis_kind(analysis_kind)
        .ok_or_else(|| fail(format!("unmapped analysis kind {analysis_kind}")))?;
    let mut categories: Vec<&str> = extra_categories;
    categories.push(primary_category);
    categories.sort_unstable();
    categories.dedup();

    if deck.is_empty() || !deck.ends_with('\n') || deck.contains('\r') || deck.contains('\0') {
        return Err(fail("deck violates the fixture byte contract".to_owned()));
    }
    let file_name = format!("{}.cir", id.replace('.', "-"));
    if !contract::valid_fixture_file_name(&file_name) {
        return Err(fail(format!(
            "fixture name {file_name} violates the grammar"
        )));
    }

    // The engine run: same library path the shipped adapter binary takes.
    let analysis = json!({ "kind": engine_kind });
    let content = CircuitContent::Deck {
        expanded_netlist: deck.clone(),
    };
    let execution = execute(&analysis, &content, env!("CARGO_PKG_VERSION"));

    let oracle_measurements = match (&expectation, &execution.response) {
        (Expectation::FailsWith(expected_code), EngineResponse::Failed { failure_code, .. }) => {
            if failure_code != expected_code {
                return Err(fail(format!(
                    "expected failure {expected_code}, engine failed as {failure_code}"
                )));
            }
            Vec::new()
        }
        (Expectation::FailsWith(expected_code), EngineResponse::Succeeded { .. }) => {
            return Err(fail(format!(
                "expected failure {expected_code}, engine succeeded"
            )));
        }
        (
            Expectation::Succeeds(_),
            EngineResponse::Failed {
                failure_code,
                failure_detail,
            },
        ) => {
            return Err(fail(format!(
                "engine failed as {failure_code}: {failure_detail}"
            )));
        }
        (
            Expectation::Succeeds(probes),
            EngineResponse::Succeeded {
                result_manifest, ..
            },
        ) => realize_probes(probes, result_manifest).map_err(fail)?,
    };

    let expected_outcome = match &expectation {
        Expectation::Succeeds(_) => "succeeded",
        Expectation::FailsWith(_) => "failed",
    };
    let expected_error_code = match &expectation {
        Expectation::Succeeds(_) => Value::Null,
        Expectation::FailsWith(code) => json!(code),
    };

    let mut parameter_values = Vec::new();
    for parameter in &parameters {
        parameter_values.push(json!({
            "name": parameter.name,
            "unit": parameter.unit,
            "value_decimal": canonical_decimal(parameter.value)
                .ok_or_else(|| fail(format!("parameter {} is not finite", parameter.name)))?,
        }));
    }
    parameter_values.sort_by(|left, right| {
        left["name"]
            .as_str()
            .unwrap_or_default()
            .cmp(right["name"].as_str().unwrap_or_default())
    });
    let case_spec = json!({
        "format_version": 1,
        "analysis_kind": analysis_kind,
        "parameters": parameter_values,
        "temperature_celsius_decimal": canonical_decimal(temperature_celsius)
            .ok_or_else(|| fail("temperature is not finite".to_owned()))?,
    });
    let fixture = json!({
        "format_version": 1,
        "media_type": contract::NETLIST_MEDIA_TYPE,
        "file_name": file_name,
        "content_utf8": deck,
    });
    let oracle = json!({
        "format_version": 1,
        "expected_outcome": expected_outcome,
        "expected_error_code": expected_error_code,
        "measurements": oracle_measurements.iter().map(|entry| entry.0.clone()).collect::<Vec<_>>(),
    });
    let tolerance = json!({
        "format_version": 1,
        "decimal_comparison_algorithm": contract::DECIMAL_COMPARISON_ALGORITHM,
        "measurements": oracle_measurements.iter().map(|entry| entry.1.clone()).collect::<Vec<_>>(),
    });

    let fixture_digest: [u8; 32] = Sha256::digest(deck.as_bytes()).into();
    Ok(json!({
        "case_id": id,
        "case_spec_sha256": canonical_sha256(&case_spec).ok_or_else(|| fail("spec digest".into()))?,
        "case_spec": case_spec,
        "fixture_sha256": rspice_engine_adapter::wire::digest_hex(&fixture_digest),
        "fixture": fixture,
        "analysis_kind": analysis_kind,
        "primary_category": primary_category,
        "categories": categories,
        "platforms": [contract::PLATFORM],
        "required_repetitions": repetitions,
        "expected_outcome": expected_outcome,
        "expected_error_code": expected_error_code,
        "oracle_manifest_sha256": canonical_sha256(&oracle)
            .ok_or_else(|| fail("oracle digest".into()))?,
        "oracle": oracle,
        "tolerance_spec_sha256": canonical_sha256(&tolerance)
            .ok_or_else(|| fail("tolerance digest".into()))?,
        "tolerance": tolerance,
    }))
}

/// Pairs each probe with the engine measurement of the same name and
/// gates the engine value against the independent expectation.
fn realize_probes(
    probes: &[Probe],
    result_manifest: &Value,
) -> Result<Vec<(Value, Value)>, String> {
    let measurements = result_manifest["measurements"]
        .as_array()
        .ok_or("result manifest has no measurement list")?;
    let mut realized = Vec::new();
    for probe in probes {
        let observed = measurements
            .iter()
            .find(|entry| entry["name"] == json!(probe.name))
            .ok_or_else(|| format!("engine reported no measurement named {}", probe.name))?;
        if observed["unit"] != json!(probe.unit) {
            return Err(format!(
                "probe {} expects unit {}, engine reported {}",
                probe.name, probe.unit, observed["unit"]
            ));
        }
        let expected_decimal = canonical_decimal(probe.expected)
            .ok_or_else(|| format!("probe {} oracle value is not finite", probe.name))?;
        let observed_decimal = observed["value_decimal"]
            .as_str()
            .ok_or_else(|| format!("probe {} has no engine value", probe.name))?;
        let comparison = within_tolerance(
            observed_decimal,
            &expected_decimal,
            probe.absolute_tolerance,
            probe.relative_tolerance,
        )
        .ok_or_else(|| {
            format!(
                "probe {} tolerances are not admissible decimals",
                probe.name
            )
        })?;
        if !comparison.passed {
            return Err(format!(
                "probe {}: engine value {observed_decimal} is outside {} ± ({} + {} relative) \
                 of the independent oracle (error {})",
                probe.name,
                expected_decimal,
                probe.absolute_tolerance,
                probe.relative_tolerance,
                comparison.absolute_error_decimal,
            ));
        }
        realized.push((
            json!({
                "name": probe.name,
                "unit": probe.unit,
                "expected_decimal": expected_decimal,
                "sample_count": observed["sample_count"],
                "series_sha256": observed["series_sha256"],
            }),
            json!({
                "name": probe.name,
                "absolute_tolerance_decimal": probe.absolute_tolerance,
                "relative_tolerance_decimal": probe.relative_tolerance,
            }),
        ));
    }
    realized.sort_by(|left, right| {
        left.0["name"]
            .as_str()
            .unwrap_or_default()
            .cmp(right.0["name"].as_str().unwrap_or_default())
    });
    Ok(realized)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn divider_draft() -> CaseDraft {
        CaseDraft {
            id: "op.divider.smoke".to_owned(),
            primary_category: "operating_point",
            extra_categories: Vec::new(),
            deck: "* equal resistive divider qualification smoke\n\
                   v1 in 0 dc 10\n\
                   r1 in out 1k\n\
                   r2 out 0 1k\n\
                   .op\n\
                   .end\n"
                .to_owned(),
            parameters: vec![
                Parameter {
                    name: "r1",
                    unit: "Ohm",
                    value: 1e3,
                },
                Parameter {
                    name: "r2",
                    unit: "Ohm",
                    value: 1e3,
                },
                Parameter {
                    name: "v1",
                    unit: "V",
                    value: 10.0,
                },
            ],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::Succeeds(vec![Probe {
                name: "v(out)".to_owned(),
                unit: "V",
                expected: 5.0,
                absolute_tolerance: "1e-9",
                relative_tolerance: "1e-9",
            }]),
        }
    }

    #[test]
    fn a_divider_case_realizes_with_exact_scalar_identity() {
        let case = realize(divider_draft()).expect("realize the divider case");
        assert_eq!(case["expected_outcome"], json!("succeeded"));
        assert_eq!(
            case["oracle"]["measurements"][0]["expected_decimal"],
            json!("5e0")
        );
        assert_eq!(case["oracle"]["measurements"][0]["sample_count"], json!(1));
        assert_eq!(
            case["oracle"]["measurements"][0]["series_sha256"],
            Value::Null
        );
        assert_eq!(
            case["tolerance"]["measurements"][0]["name"],
            json!("v(out)")
        );
        // The realized case must survive a full corpus validation pass
        // once wrapped; spot-check the digests here.
        assert_eq!(
            case["case_spec_sha256"],
            json!(canonical_sha256(&case["case_spec"]).expect("spec digest"))
        );
    }

    #[test]
    fn an_engine_value_outside_the_oracle_tolerance_fails_generation() {
        let mut draft = divider_draft();
        let Expectation::Succeeds(probes) = &mut draft.expectation else {
            unreachable!("divider draft succeeds");
        };
        probes[0].expected = 5.1;
        let error = realize(draft).expect_err("a wrong oracle must fail generation");
        assert!(error.contains("outside"), "unexpected error: {error}");
    }

    #[test]
    fn failure_drafts_pin_the_exact_engine_code() {
        let draft = CaseDraft {
            id: "fail.no-directive.smoke".to_owned(),
            primary_category: "convergence_and_failure",
            extra_categories: Vec::new(),
            deck: "* a deck with no operating-point directive\n\
                   v1 in 0 dc 1\n\
                   r1 in 0 1k\n\
                   .tran 1u 10u\n\
                   .end\n"
                .to_owned(),
            parameters: vec![Parameter {
                name: "r1",
                unit: "Ohm",
                value: 1e3,
            }],
            temperature_celsius: 27.0,
            repetitions: 1,
            expectation: Expectation::FailsWith("analysis.directive_missing"),
        };
        let case = realize(draft).expect("realize the failure case");
        assert_eq!(case["expected_outcome"], json!("failed"));
        assert_eq!(
            case["expected_error_code"],
            json!("analysis.directive_missing")
        );
        assert_eq!(case["oracle"]["measurements"], json!([]));
        assert_eq!(case["analysis_kind"], json!("failure_path"));
    }
}
