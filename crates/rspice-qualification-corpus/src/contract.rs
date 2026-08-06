//! The simulator-qualification policy/corpus contract, mirrored from the
//! cloud's release-admission tools. The cloud side stays authoritative: this
//! mirror exists so corpus generation fails at build time instead of at
//! release admission, and it deliberately enforces the *strictest* reading —
//! every bound the admission chain applies anywhere (corpus validation or
//! evidence verification) is applied here to everything the corpus emits.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::canonical::{canonical_json, canonical_sha256};
use crate::decimal::parse_decimal;

pub const EVIDENCE_TYPE: &str = "rspice.simulator-qualification/v1";
pub const SUITE: &str = "rspice-commercial-cloud-v1";
pub const PROFILE: &str = "rspice-cloud-linux-amd64";
pub const DECIMAL_COMPARISON_ALGORITHM: &str = "rspice-decimal-v1";
pub const NETLIST_MEDIA_TYPE: &str = "application/vnd.rspice.spice-netlist.v1";
pub const PLATFORM: &str = "linux/amd64";
pub const MINIMUM_CASE_COUNT: usize = 250;
pub const MAXIMUM_EXECUTION_COUNT: usize = 10_000;
pub const MAXIMUM_REPEAT_COUNT: u64 = 10;
pub const CATEGORY_MINIMUM: u64 = 10;

/// The twelve reviewed coverage categories, in the exact policy order.
pub const REQUIRED_CATEGORIES: [&str; 12] = [
    "operating_point",
    "dc_sweep",
    "transient",
    "ac_small_signal",
    "noise",
    "nonlinear_devices",
    "semiconductor_models",
    "temperature_and_corners",
    "mixed_signal",
    "convergence_and_failure",
    "deterministic_reproducibility",
    "security_and_resource_limits",
];

/// The corpus analysis kind each primary category binds to.
pub fn analysis_kind_for_primary(category: &str) -> Option<&'static str> {
    Some(match category {
        "operating_point" | "nonlinear_devices" if category == "operating_point" => {
            "operating_point"
        }
        "nonlinear_devices" => "operating_point",
        "operating_point" => "operating_point",
        "dc_sweep" | "semiconductor_models" | "temperature_and_corners" => "dc_sweep",
        "transient" | "deterministic_reproducibility" => "transient",
        "ac_small_signal" => "ac_small_signal",
        "noise" => "noise",
        "mixed_signal" => "mixed_signal",
        "convergence_and_failure" => "failure_path",
        "security_and_resource_limits" => "resource_limit",
        _ => return None,
    })
}

/// The engine analysis the qualification harness submits for a corpus
/// analysis kind. `failure_path` and `resource_limit` are corpus-level
/// kinds with a fixed engine mapping; the cloud harness applies the same
/// rule, so failure decks must fail under `.op` semantics and resource
/// decks under `.tran` semantics.
pub fn engine_analysis_kind(corpus_kind: &str) -> Option<&'static str> {
    Some(match corpus_kind {
        "operating_point" => "operating_point",
        "dc_sweep" => "dc_sweep",
        "transient" => "transient",
        "ac_small_signal" => "ac_small_signal",
        "noise" => "noise",
        "mixed_signal" => "mixed_signal",
        "failure_path" => "operating_point",
        "resource_limit" => "transient",
        _ => return None,
    })
}

/// Builds the fixed qualification policy document.
pub fn build_policy() -> Value {
    json!({
        "format_version": 1,
        "evidence_type": EVIDENCE_TYPE,
        "suite": SUITE,
        "suite_version": 1,
        "profile": PROFILE,
        "profile_version": 1,
        "policy_epoch": 1,
        "minimum_case_count": MINIMUM_CASE_COUNT,
        "maximum_execution_count": MAXIMUM_EXECUTION_COUNT,
        "required_platforms": [PLATFORM],
        "required_categories": REQUIRED_CATEGORIES,
        "required_category_minimums": REQUIRED_CATEGORIES
            .iter()
            .map(|category| ((*category).to_owned(), json!(CATEGORY_MINIMUM)))
            .collect::<serde_json::Map<_, _>>(),
        "decimal_comparison_algorithm": DECIMAL_COMPARISON_ALGORITHM,
        "maximum_repeat_count": MAXIMUM_REPEAT_COUNT,
    })
}

fn matches_grammar(value: &str, first: fn(u8) -> bool, rest: fn(u8) -> bool, max: usize) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= max
        && first(bytes[0])
        && bytes[1..].iter().all(|byte| rest(*byte))
}

fn lower_alphanumeric(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte.is_ascii_digit()
}

pub fn valid_case_id(value: &str) -> bool {
    matches_grammar(
        value,
        lower_alphanumeric,
        |byte| lower_alphanumeric(byte) || matches!(byte, b'.' | b'_' | b'-'),
        160,
    )
}

pub fn valid_analysis_kind(value: &str) -> bool {
    matches_grammar(
        value,
        lower_alphanumeric,
        |byte| lower_alphanumeric(byte) || matches!(byte, b'.' | b'_' | b'-'),
        80,
    )
}

pub fn valid_fixture_file_name(value: &str) -> bool {
    let Some(stem) = value.strip_suffix(".cir") else {
        return false;
    };
    matches_grammar(
        stem,
        lower_alphanumeric,
        |byte| lower_alphanumeric(byte) || matches!(byte, b'.' | b'_' | b'-'),
        120 - ".cir".len(),
    )
}

pub fn valid_parameter_name(value: &str) -> bool {
    matches_grammar(
        value,
        |byte| byte.is_ascii_lowercase(),
        |byte| lower_alphanumeric(byte) || matches!(byte, b'.' | b'_' | b'-'),
        80,
    )
}

pub fn valid_measurement_name(value: &str) -> bool {
    matches_grammar(
        value,
        |byte| byte.is_ascii_lowercase(),
        |byte| {
            lower_alphanumeric(byte)
                || matches!(
                    byte,
                    b'_' | b'(' | b')' | b'.' | b':' | b'+' | b'/' | b'[' | b']' | b'-'
                )
        },
        120,
    )
}

pub fn valid_unit(value: &str) -> bool {
    matches_grammar(
        value,
        |byte| byte.is_ascii_alphabetic(),
        |byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'*' | b'^' | b'.' | b'_' | b'-')
        },
        40,
    )
}

pub fn valid_error_code(value: &str) -> bool {
    matches_grammar(
        value,
        lower_alphanumeric,
        |byte| lower_alphanumeric(byte) || matches!(byte, b'.' | b'_' | b'-'),
        120,
    )
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// A corpus decimal that admission can also *parse*: the corpus grammar
/// plus the evidence verifier's ±100 exponent bound. Everything the corpus
/// emits must satisfy both, because oracle and tolerance values flow into
/// the exact-arithmetic comparison at admission.
fn valid_corpus_decimal(value: &str) -> bool {
    parse_decimal(value).is_some()
}

fn as_str(value: &Value) -> Result<&str, String> {
    value
        .as_str()
        .ok_or_else(|| format!("expected a string, found {value}"))
}

fn keys_of(value: &Value, expected: &[&str], location: &str) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| format!("{location} must be an object"))?;
    let mut observed: Vec<&str> = object.keys().map(String::as_str).collect();
    observed.sort_unstable();
    let mut wanted = expected.to_vec();
    wanted.sort_unstable();
    if observed != wanted {
        return Err(format!(
            "{location} has keys {observed:?}, expected {wanted:?}"
        ));
    }
    Ok(())
}

/// Everything downstream tooling needs to know about a validated corpus.
pub struct CorpusSummary {
    pub case_count: usize,
    pub execution_count: usize,
    pub category_counts: BTreeMap<String, usize>,
    pub case_set_sha256: String,
    pub policy_sha256: String,
}

/// Validates final policy and corpus bytes exactly as release admission
/// will. `policy_bytes` must be the literal file content: the corpus binds
/// to the policy by the digest of those bytes, not of a re-serialization.
pub fn validate_policy_and_corpus(
    policy_bytes: &[u8],
    corpus_bytes: &[u8],
) -> Result<CorpusSummary, String> {
    let policy: Value =
        serde_json::from_slice(policy_bytes).map_err(|error| format!("policy: {error}"))?;
    let corpus: Value =
        serde_json::from_slice(corpus_bytes).map_err(|error| format!("corpus: {error}"))?;
    let policy_digest: [u8; 32] = Sha256::digest(policy_bytes).into();
    let policy_sha256 = rspice_engine_adapter::wire::digest_hex(&policy_digest);

    if canonical_json(&policy).as_deref().map(str::as_bytes) != Some(policy_bytes) {
        return Err("policy bytes are not the canonical serialization".to_owned());
    }
    if canonical_json(&corpus).as_deref().map(str::as_bytes) != Some(corpus_bytes) {
        return Err("corpus bytes are not the canonical serialization".to_owned());
    }
    if build_policy() != policy {
        return Err("policy does not match the reviewed policy document".to_owned());
    }

    keys_of(
        &corpus,
        &[
            "format_version",
            "suite",
            "suite_version",
            "profile",
            "profile_version",
            "policy_sha256",
            "cases",
        ],
        "corpus",
    )?;
    if corpus["format_version"] != json!(1)
        || corpus["suite"] != policy["suite"]
        || corpus["suite_version"] != policy["suite_version"]
        || corpus["profile"] != policy["profile"]
        || corpus["profile_version"] != policy["profile_version"]
        || corpus["policy_sha256"] != json!(policy_sha256)
    {
        return Err("corpus header does not match the approved policy".to_owned());
    }
    let cases = corpus["cases"]
        .as_array()
        .ok_or("corpus cases must be an array")?;
    if cases.len() < MINIMUM_CASE_COUNT || cases.len() > MAXIMUM_EXECUTION_COUNT {
        return Err(format!(
            "corpus has {} cases, outside the policy bounds",
            cases.len()
        ));
    }

    let mut previous_case_id = String::new();
    let mut observed_spec_digests = Vec::new();
    let mut category_counts: BTreeMap<String, usize> = REQUIRED_CATEGORIES
        .iter()
        .map(|category| ((*category).to_owned(), 0))
        .collect();
    let mut execution_count = 0_usize;
    let mut canonical_cases = Vec::new();

    for case in cases {
        let case_id = as_str(&case["case_id"])?.to_owned();
        let location = format!("case {case_id}");
        keys_of(
            case,
            &[
                "case_id",
                "case_spec_sha256",
                "case_spec",
                "fixture_sha256",
                "fixture",
                "analysis_kind",
                "primary_category",
                "categories",
                "platforms",
                "required_repetitions",
                "expected_outcome",
                "expected_error_code",
                "oracle_manifest_sha256",
                "oracle",
                "tolerance_spec_sha256",
                "tolerance",
            ],
            &location,
        )?;
        if !valid_case_id(&case_id) || case_id <= previous_case_id {
            return Err(format!(
                "{location}: identifier must be valid and strictly ascending"
            ));
        }
        let analysis_kind = as_str(&case["analysis_kind"])?;
        let primary = as_str(&case["primary_category"])?;
        if !valid_analysis_kind(analysis_kind)
            || analysis_kind_for_primary(primary) != Some(analysis_kind)
        {
            return Err(format!(
                "{location}: analysis kind does not bind to {primary}"
            ));
        }
        let categories = case["categories"]
            .as_array()
            .ok_or_else(|| format!("{location}: categories must be an array"))?;
        if categories.is_empty() || categories.len() > 4 {
            return Err(format!("{location}: 1..=4 categories required"));
        }
        let mut category_names = Vec::new();
        for entry in categories {
            let name = as_str(entry)?;
            if !REQUIRED_CATEGORIES.contains(&name) {
                return Err(format!("{location}: unknown category {name}"));
            }
            if category_names
                .last()
                .is_some_and(|previous: &&str| *previous >= name)
            {
                return Err(format!("{location}: categories must be strictly ascending"));
            }
            category_names.push(name);
        }
        if !category_names.contains(&primary) {
            return Err(format!("{location}: categories must include the primary"));
        }
        if case["platforms"] != json!([PLATFORM]) {
            return Err(format!(
                "{location}: platforms must be exactly [{PLATFORM}]"
            ));
        }
        let repetitions = case["required_repetitions"]
            .as_u64()
            .filter(|count| (1..=MAXIMUM_REPEAT_COUNT).contains(count))
            .ok_or_else(|| format!("{location}: invalid repetition count"))?;
        let expected_outcome = as_str(&case["expected_outcome"])?;
        let expected_error_code = &case["expected_error_code"];
        match expected_outcome {
            "succeeded" if expected_error_code.is_null() => {}
            "failed" if expected_error_code.as_str().is_some_and(valid_error_code) => {}
            _ => {
                return Err(format!(
                    "{location}: outcome and error code are inconsistent"
                ));
            }
        }

        validate_case_spec(case, analysis_kind, &location)?;
        validate_fixture(case, &location)?;
        validate_oracle_and_tolerance(case, expected_outcome, expected_error_code, &location)?;

        let spec_digest = as_str(&case["case_spec_sha256"])?.to_owned();
        if observed_spec_digests.contains(&spec_digest) {
            return Err(format!("{location}: duplicate case specification"));
        }
        observed_spec_digests.push(spec_digest);

        execution_count += usize::try_from(repetitions).unwrap_or(usize::MAX);
        if execution_count > MAXIMUM_EXECUTION_COUNT {
            return Err("corpus exceeds the bounded execution count".to_owned());
        }
        for name in &category_names {
            *category_counts.get_mut(*name).expect("known category") += 1;
        }
        canonical_cases.push(json!({
            "case_id": case["case_id"],
            "case_spec_sha256": case["case_spec_sha256"],
            "fixture_sha256": case["fixture_sha256"],
            "analysis_kind": case["analysis_kind"],
            "primary_category": case["primary_category"],
            "categories": case["categories"],
            "platforms": case["platforms"],
            "required_repetitions": case["required_repetitions"],
            "expected_outcome": case["expected_outcome"],
            "expected_error_code": case["expected_error_code"],
            "oracle_manifest_sha256": case["oracle_manifest_sha256"],
            "tolerance_spec_sha256": case["tolerance_spec_sha256"],
        }));
        previous_case_id = case_id;
    }

    for (category, count) in &category_counts {
        if *count < usize::try_from(CATEGORY_MINIMUM).expect("small constant") {
            return Err(format!(
                "category {category} covers {count} cases, below the floor"
            ));
        }
    }

    Ok(CorpusSummary {
        case_count: cases.len(),
        execution_count,
        category_counts,
        case_set_sha256: canonical_sha256(&Value::Array(canonical_cases))
            .ok_or("case set must serialize canonically")?,
        policy_sha256,
    })
}

fn validate_case_spec(case: &Value, analysis_kind: &str, location: &str) -> Result<(), String> {
    let spec = &case["case_spec"];
    keys_of(
        spec,
        &[
            "format_version",
            "analysis_kind",
            "parameters",
            "temperature_celsius_decimal",
        ],
        &format!("{location} spec"),
    )?;
    if spec["format_version"] != json!(1)
        || spec["analysis_kind"] != json!(analysis_kind)
        || !as_str(&spec["temperature_celsius_decimal"]).is_ok_and(valid_corpus_decimal)
    {
        return Err(format!("{location}: invalid case specification header"));
    }
    let parameters = spec["parameters"]
        .as_array()
        .ok_or_else(|| format!("{location}: parameters must be an array"))?;
    if parameters.is_empty() || parameters.len() > 100 {
        return Err(format!("{location}: 1..=100 parameters required"));
    }
    let mut previous = String::new();
    for parameter in parameters {
        keys_of(
            parameter,
            &["name", "unit", "value_decimal"],
            &format!("{location} parameter"),
        )?;
        let name = as_str(&parameter["name"])?;
        if !valid_parameter_name(name)
            || (!previous.is_empty() && previous.as_str() >= name)
            || !as_str(&parameter["unit"]).is_ok_and(valid_unit)
            || !as_str(&parameter["value_decimal"]).is_ok_and(valid_corpus_decimal)
        {
            return Err(format!("{location}: invalid parameter {name}"));
        }
        previous = name.to_owned();
    }
    let canonical =
        canonical_json(spec).ok_or_else(|| format!("{location}: non-canonical spec"))?;
    if canonical.len() > 32 * 1024 {
        return Err(format!("{location}: case specification exceeds 32KiB"));
    }
    if canonical_sha256(spec).as_deref() != case["case_spec_sha256"].as_str() {
        return Err(format!("{location}: case_spec_sha256 mismatch"));
    }
    Ok(())
}

fn validate_fixture(case: &Value, location: &str) -> Result<(), String> {
    let fixture = &case["fixture"];
    keys_of(
        fixture,
        &["format_version", "media_type", "file_name", "content_utf8"],
        &format!("{location} fixture"),
    )?;
    let content = as_str(&fixture["content_utf8"])?;
    if fixture["format_version"] != json!(1)
        || fixture["media_type"] != json!(NETLIST_MEDIA_TYPE)
        || !as_str(&fixture["file_name"]).is_ok_and(valid_fixture_file_name)
        || content.is_empty()
        || content.len() > 1024 * 1024
        || content.contains('\0')
        || content.contains('\r')
        || !content.ends_with('\n')
    {
        return Err(format!("{location}: fixture violates the netlist contract"));
    }
    let digest: [u8; 32] = Sha256::digest(content.as_bytes()).into();
    if case["fixture_sha256"].as_str()
        != Some(rspice_engine_adapter::wire::digest_hex(&digest).as_str())
    {
        return Err(format!("{location}: fixture_sha256 mismatch"));
    }
    Ok(())
}

fn validate_oracle_and_tolerance(
    case: &Value,
    expected_outcome: &str,
    expected_error_code: &Value,
    location: &str,
) -> Result<(), String> {
    let oracle = &case["oracle"];
    keys_of(
        oracle,
        &[
            "format_version",
            "expected_outcome",
            "expected_error_code",
            "measurements",
        ],
        &format!("{location} oracle"),
    )?;
    if oracle["format_version"] != json!(1)
        || oracle["expected_outcome"] != json!(expected_outcome)
        || oracle["expected_error_code"] != *expected_error_code
    {
        return Err(format!("{location}: oracle header mismatch"));
    }
    let measurements = oracle["measurements"]
        .as_array()
        .ok_or_else(|| format!("{location}: oracle measurements must be an array"))?;
    if expected_outcome == "failed" {
        if !measurements.is_empty() {
            return Err(format!(
                "{location}: failed cases must have no measurements"
            ));
        }
    } else {
        if measurements.is_empty() || measurements.len() > 10_000 {
            return Err(format!(
                "{location}: 1..=10000 oracle measurements required"
            ));
        }
        let mut previous = String::new();
        for measurement in measurements {
            keys_of(
                measurement,
                &[
                    "name",
                    "unit",
                    "expected_decimal",
                    "sample_count",
                    "series_sha256",
                ],
                &format!("{location} oracle measurement"),
            )?;
            let name = as_str(&measurement["name"])?;
            if !valid_measurement_name(name)
                || (!previous.is_empty() && previous.as_str() >= name)
                || !as_str(&measurement["unit"]).is_ok_and(valid_unit)
                || !as_str(&measurement["expected_decimal"]).is_ok_and(valid_corpus_decimal)
                || !measurement["sample_count"]
                    .as_u64()
                    .is_some_and(|count| (1..=10_000_000).contains(&count))
                || !(measurement["series_sha256"].is_null()
                    || measurement["series_sha256"]
                        .as_str()
                        .is_some_and(valid_digest))
            {
                return Err(format!("{location}: invalid oracle measurement {name}"));
            }
            previous = name.to_owned();
        }
    }
    if canonical_sha256(oracle).as_deref() != case["oracle_manifest_sha256"].as_str() {
        return Err(format!("{location}: oracle_manifest_sha256 mismatch"));
    }

    let tolerance = &case["tolerance"];
    keys_of(
        tolerance,
        &[
            "format_version",
            "decimal_comparison_algorithm",
            "measurements",
        ],
        &format!("{location} tolerance"),
    )?;
    if tolerance["format_version"] != json!(1)
        || tolerance["decimal_comparison_algorithm"] != json!(DECIMAL_COMPARISON_ALGORITHM)
    {
        return Err(format!("{location}: tolerance header mismatch"));
    }
    let entries = tolerance["measurements"]
        .as_array()
        .ok_or_else(|| format!("{location}: tolerance measurements must be an array"))?;
    if entries.len() != measurements.len() {
        return Err(format!("{location}: tolerance and oracle lengths differ"));
    }
    for (entry, measurement) in entries.iter().zip(measurements) {
        keys_of(
            entry,
            &[
                "name",
                "absolute_tolerance_decimal",
                "relative_tolerance_decimal",
            ],
            &format!("{location} tolerance measurement"),
        )?;
        let absolute = as_str(&entry["absolute_tolerance_decimal"])?;
        let relative = as_str(&entry["relative_tolerance_decimal"])?;
        if entry["name"] != measurement["name"]
            || !valid_corpus_decimal(absolute)
            || absolute.starts_with('-')
            || !valid_corpus_decimal(relative)
            || relative.starts_with('-')
        {
            return Err(format!("{location}: invalid tolerance entry"));
        }
    }
    if canonical_sha256(tolerance).as_deref() != case["tolerance_spec_sha256"].as_str() {
        return Err(format!("{location}: tolerance_spec_sha256 mismatch"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_policy_document_is_the_reviewed_constant() {
        let policy = build_policy();
        assert_eq!(policy["minimum_case_count"], json!(250));
        assert_eq!(
            policy["required_categories"].as_array().map(Vec::len),
            Some(12)
        );
        assert_eq!(
            policy["required_category_minimums"]
                .as_object()
                .map(|minimums| minimums.len()),
            Some(12)
        );
        assert!(
            canonical_json(&policy).is_some(),
            "policy must be canonical-serializable"
        );
    }

    #[test]
    fn category_bindings_cover_every_category_exactly() {
        for category in REQUIRED_CATEGORIES {
            let kind = analysis_kind_for_primary(category).expect("bound analysis kind");
            assert!(
                engine_analysis_kind(kind).is_some(),
                "{kind} must map to an engine analysis"
            );
        }
        assert_eq!(
            analysis_kind_for_primary("nonlinear_devices"),
            Some("operating_point")
        );
        assert_eq!(
            analysis_kind_for_primary("semiconductor_models"),
            Some("dc_sweep")
        );
        assert_eq!(
            analysis_kind_for_primary("convergence_and_failure"),
            Some("failure_path")
        );
        assert_eq!(
            engine_analysis_kind("failure_path"),
            Some("operating_point")
        );
        assert_eq!(engine_analysis_kind("resource_limit"), Some("transient"));
        assert_eq!(analysis_kind_for_primary("unknown"), None);
        assert_eq!(engine_analysis_kind("unknown"), None);
    }

    #[test]
    fn grammar_validators_transliterate_the_contract_patterns() {
        assert!(valid_case_id("op.divider.001"));
        assert!(!valid_case_id("Op.divider"));
        assert!(!valid_case_id(".leading-dot"));
        assert!(valid_fixture_file_name("rc-charge-001.cir"));
        assert!(!valid_fixture_file_name("deck.sp"));
        assert!(!valid_fixture_file_name(".cir"));
        assert!(valid_parameter_name("r1.ohms"));
        assert!(!valid_parameter_name("1r"));
        assert!(valid_measurement_name("v(out)"));
        assert!(valid_measurement_name("i(v1-branch)"));
        assert!(!valid_measurement_name("V(out)"));
        assert!(valid_unit("V"));
        assert!(valid_unit("A"));
        // The unit grammar has no parentheses; noise densities spell the
        // root as an exponent.
        assert!(valid_unit("V/Hz^0.5"));
        assert!(!valid_unit("V/sqrt(Hz)"));
        assert!(!valid_unit("1V"));
        assert!(valid_error_code("engine.singular_matrix"));
        assert!(!valid_error_code("Engine.Error"));
    }
}
