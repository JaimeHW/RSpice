//! Qualification harness stages: `plan` expands the published corpus into
//! the exact protocol-3 engine requests the released worker adapter
//! executes; `assemble` turns the captured engine responses into the
//! eleven retained evidence artifacts, gating every observation against
//! the corpus oracle exactly as release admission will.
//!
//! Identifiers are derived deterministically from the suite run identifier
//! and the execution key, so a plan is a pure function of the published
//! corpus bytes and one externally chosen UUID; regenerating a plan cannot
//! silently reorder or relabel executions. Every emitted request is
//! round-tripped through the adapter's own strict request parser before it
//! is written, so a planning defect fails here instead of mid-suite.

use std::path::Path;

use rspice_engine_adapter::wire::{
    CURRENT_REVISION_CONTENT_DIGEST_VERSION, CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
    EngineRequest, EngineRevision, INTEGRITY_ENGINE_PROTOCOL_VERSION, digest_hex,
    parse_engine_request, revision_content_digest, simulation_request_digest,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::canonical::canonical_sha256;
use crate::contract;
use crate::decimal::within_tolerance;

/// The circuit-document schema every qualification fixture executes under.
pub const DOCUMENT_SCHEMA: &str = "rspice-circuit-v1";

const IDENTITY_DOMAIN: &[u8] = b"rspice-qualification-plan-v1\0";

/// One planned execution, in the exact corpus execution order.
pub struct PlannedExecution {
    pub case_id: String,
    pub platform: String,
    pub repeat_ordinal: u64,
    pub expected_outcome: String,
    pub expected_error_code: Option<String>,
    pub engine_analysis_kind: &'static str,
    pub simulation_run_id: Uuid,
    pub circuit_id: Uuid,
    pub revision_id: Uuid,
    pub request_sha256: String,
    pub revision_sha256: String,
    pub request_bytes: Vec<u8>,
}

/// A deterministic RFC 4122 version-8 identifier bound to the suite run
/// and one role within one execution.
fn derived_uuid(suite_run_id: Uuid, execution_key: &str, role: &str) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(IDENTITY_DOMAIN);
    hasher.update(suite_run_id.as_bytes());
    hasher.update(execution_key.as_bytes());
    hasher.update([0]);
    hasher.update(role.as_bytes());
    let digest: [u8; 32] = hasher.finalize().into();
    let mut bytes = [0_u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = 0x80 | (bytes[6] & 0x0f);
    bytes[8] = 0x80 | (bytes[8] & 0x3f);
    Uuid::from_bytes(bytes)
}

/// Expands validated policy and corpus bytes into the ordered execution
/// plan. The bytes must be the published canonical serializations; the
/// full admission validation runs first.
pub fn plan(
    policy_bytes: &[u8],
    corpus_bytes: &[u8],
    suite_run_id: Uuid,
) -> Result<Vec<PlannedExecution>, String> {
    contract::validate_policy_and_corpus(policy_bytes, corpus_bytes)?;
    if suite_run_id.is_nil() {
        return Err("suite run identifier must not be nil".to_owned());
    }
    let corpus: Value =
        serde_json::from_slice(corpus_bytes).map_err(|error| format!("corpus: {error}"))?;
    let cases = corpus["cases"].as_array().expect("validated corpus");

    let mut executions = Vec::new();
    for case in cases {
        let case_id = case["case_id"].as_str().expect("validated case id");
        let analysis_kind = case["analysis_kind"].as_str().expect("validated kind");
        let engine_kind = contract::engine_analysis_kind(analysis_kind)
            .ok_or_else(|| format!("case {case_id}: unmapped analysis kind {analysis_kind}"))?;
        let expected_outcome = case["expected_outcome"]
            .as_str()
            .expect("validated outcome")
            .to_owned();
        let expected_error_code = case["expected_error_code"].as_str().map(str::to_owned);
        let repetitions = case["required_repetitions"]
            .as_u64()
            .expect("validated repetitions");
        let netlist = case["fixture"]["content_utf8"]
            .as_str()
            .expect("validated fixture");

        let document = json!({
            "netlist_utf8": netlist,
            "schema": DOCUMENT_SCHEMA,
        });
        let analysis = json!({ "kind": engine_kind });

        for platform in case["platforms"].as_array().expect("validated platforms") {
            let platform = platform.as_str().expect("validated platform");
            for repeat_ordinal in 1..=repetitions {
                let execution_key = format!("{case_id}\u{0}{platform}\u{0}{repeat_ordinal}");
                let simulation_run_id = derived_uuid(suite_run_id, &execution_key, "run");
                let circuit_id = derived_uuid(suite_run_id, &execution_key, "circuit");
                let revision_id = derived_uuid(suite_run_id, &execution_key, "revision");

                let revision_digest = revision_content_digest(1, &document, &[])
                    .map_err(|error| format!("case {case_id}: revision digest: {error}"))?;
                let request_digest =
                    simulation_request_digest(circuit_id, revision_id, &revision_digest, &analysis)
                        .map_err(|error| format!("case {case_id}: request digest: {error}"))?;

                let request = EngineRequest {
                    protocol_version: INTEGRITY_ENGINE_PROTOCOL_VERSION,
                    simulation_run_id,
                    circuit_id,
                    attempt: 1,
                    request_digest_version: Some(CURRENT_SIMULATION_REQUEST_DIGEST_VERSION),
                    request_sha256: digest_hex(&request_digest),
                    analysis: analysis.clone(),
                    revision: EngineRevision {
                        id: revision_id,
                        schema_version: 1,
                        content_digest_version: Some(CURRENT_REVISION_CONTENT_DIGEST_VERSION),
                        content_sha256: digest_hex(&revision_digest),
                        document: document.clone(),
                        artifacts: Vec::new(),
                    },
                };
                let request_bytes = serde_json::to_vec(&request)
                    .map_err(|error| format!("case {case_id}: request serialization: {error}"))?;
                parse_engine_request(&request_bytes).map_err(|error| {
                    format!("case {case_id}: planned request failed the adapter parser: {error}")
                })?;

                executions.push(PlannedExecution {
                    case_id: case_id.to_owned(),
                    platform: platform.to_owned(),
                    repeat_ordinal,
                    expected_outcome: expected_outcome.clone(),
                    expected_error_code: expected_error_code.clone(),
                    engine_analysis_kind: engine_kind,
                    simulation_run_id,
                    circuit_id,
                    revision_id,
                    request_sha256: digest_hex(&request_digest),
                    revision_sha256: digest_hex(&revision_digest),
                    request_bytes,
                });
            }
        }
    }
    Ok(executions)
}

/// Writes a plan to disk: one request file per execution plus an index the
/// assembler joins against, in execution order.
pub fn write_plan(
    executions: &[PlannedExecution],
    suite_run_id: Uuid,
    output: &Path,
) -> Result<(), String> {
    let requests = output.join("requests");
    std::fs::create_dir_all(&requests)
        .map_err(|error| format!("cannot create {}: {error}", requests.display()))?;
    let mut index = Vec::with_capacity(executions.len());
    for (ordinal, execution) in executions.iter().enumerate() {
        let file_name = format!("{ordinal:05}.request.json");
        std::fs::write(requests.join(&file_name), &execution.request_bytes)
            .map_err(|error| format!("cannot write request {file_name}: {error}"))?;
        index.push(json!({
            "case_id": execution.case_id,
            "platform": execution.platform,
            "repeat_ordinal": execution.repeat_ordinal,
            "expected_outcome": execution.expected_outcome,
            "expected_error_code": execution.expected_error_code,
            "engine_analysis_kind": execution.engine_analysis_kind,
            "simulation_run_id": execution.simulation_run_id,
            "circuit_id": execution.circuit_id,
            "revision_id": execution.revision_id,
            "request_sha256": execution.request_sha256,
            "revision_sha256": execution.revision_sha256,
            "request_file": format!("requests/{file_name}"),
        }));
    }
    let plan = json!({
        "format_version": 1,
        "suite_run_id": suite_run_id,
        "executions": index,
    });
    let plan_bytes =
        serde_json::to_vec_pretty(&plan).map_err(|error| format!("plan serialization: {error}"))?;
    std::fs::write(output.join("plan.json"), plan_bytes)
        .map_err(|error| format!("cannot write plan.json: {error}"))?;
    Ok(())
}

/// The release-manifest values every assembled payload binds to, prepared
/// by the workflow from the verified release manifest.
#[derive(Deserialize)]
pub struct ReleaseBinding {
    pub worker_image_reference: String,
    pub worker_platform_digest: String,
    pub runtime_mode: String,
    pub engine_name: String,
    pub engine_build: String,
    pub adapter_image_reference: String,
    pub adapter_sha256: String,
    pub solver_image_reference: Option<String>,
    pub solver_sha256: Option<String>,
    pub model_library_image_reference: Option<String>,
    pub model_library_sha256: Option<String>,
    pub observer_image_reference: String,
    pub components: Vec<ComponentContract>,
}

/// One attested simulator component. Field order is load-bearing: the
/// admission verifier compares the serialized array byte for byte against
/// its own construction in exactly this key order.
#[derive(Deserialize, Serialize)]
pub struct ComponentContract {
    pub role: String,
    pub image_reference: String,
    pub exported_file_sha256: String,
    pub predicate_type: String,
    pub attestation_statement_sha256: String,
    pub source_repository: String,
    pub source_sha: String,
    pub build_recipe_sha256: String,
    pub capabilities_manifest_sha256: String,
}

/// What `assemble` reports for evidence finalization: the recomputed case
/// set digest, the execution counts, and the per-category coverage in the
/// policy's category order.
pub struct AssembleSummary {
    pub case_set_sha256: String,
    pub executed_case_count: usize,
    pub category_counts_in_policy_order: Vec<(String, usize)>,
}

fn read_response(responses: &Path, ordinal: usize) -> Result<Value, String> {
    let path = responses.join(format!("{ordinal:05}.response.json"));
    let bytes =
        std::fs::read(&path).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

fn write_artifact(output: &Path, file_name: &str, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(output.join(file_name), bytes)
        .map_err(|error| format!("cannot write {file_name}: {error}"))
}

/// Assembles the eleven retained qualification artifacts from the plan and
/// the captured engine responses. Every observation is gated here exactly
/// as admission gates it: outcome and error code must match the corpus
/// expectation, raw series identity must equal the oracle's, and every
/// evaluator comparison must pass in exact decimal arithmetic.
#[allow(clippy::too_many_lines)]
pub fn assemble(
    policy_bytes: &[u8],
    corpus_bytes: &[u8],
    suite_run_id: Uuid,
    responses: &Path,
    binding: &ReleaseBinding,
    log_bytes: &[u8],
    output: &Path,
) -> Result<AssembleSummary, String> {
    let summary = contract::validate_policy_and_corpus(policy_bytes, corpus_bytes)?;
    let executions = plan(policy_bytes, corpus_bytes, suite_run_id)?;
    let corpus: Value = serde_json::from_slice(corpus_bytes).expect("validated corpus");
    let cases = corpus["cases"].as_array().expect("validated corpus");
    let case_by_id = |case_id: &str| {
        cases
            .iter()
            .find(|case| case["case_id"] == json!(case_id))
            .expect("planned case exists")
    };
    std::fs::create_dir_all(output)
        .map_err(|error| format!("cannot create {}: {error}", output.display()))?;

    let mut case_result_rows = Vec::new();
    let mut execution_entries = Vec::new();
    let mut trace_entries = Vec::new();
    let mut raw_entries = Vec::new();
    let mut evaluator_entries = Vec::new();

    for (ordinal, execution) in executions.iter().enumerate() {
        let fail = |detail: String| {
            format!(
                "execution {ordinal} ({} {} repeat {}): {detail}",
                execution.case_id, execution.platform, execution.repeat_ordinal
            )
        };
        let case = case_by_id(&execution.case_id);
        let response = read_response(responses, ordinal)?;
        let status = response["status"]
            .as_str()
            .ok_or_else(|| fail("response has no status".to_owned()))?;
        let (observed_outcome, observed_error_code) = match status {
            "succeeded" => ("succeeded".to_owned(), Value::Null),
            "failed" => ("failed".to_owned(), response["failure_code"].clone()),
            other => return Err(fail(format!("unknown response status {other}"))),
        };
        if observed_outcome != execution.expected_outcome
            || observed_error_code
                != execution
                    .expected_error_code
                    .as_deref()
                    .map_or(Value::Null, |code| json!(code))
        {
            return Err(fail(format!(
                "engine outcome {observed_outcome}/{observed_error_code} does not match \
                 the corpus expectation {}/{:?}",
                execution.expected_outcome, execution.expected_error_code
            )));
        }

        // Raw measurements: the oracle-name-ordered projection of the
        // engine's manifest, with series identity required to be exact.
        let oracle_measurements = case["oracle"]["measurements"]
            .as_array()
            .expect("validated oracle");
        let mut raw_measurements = Vec::with_capacity(oracle_measurements.len());
        let mut evaluator_measurements = Vec::with_capacity(oracle_measurements.len());
        if observed_outcome == "succeeded" {
            let reported = response["result_manifest"]["measurements"]
                .as_array()
                .ok_or_else(|| fail("response has no measurement list".to_owned()))?;
            let tolerance_entries = case["tolerance"]["measurements"]
                .as_array()
                .expect("validated tolerance");
            for (oracle, tolerance) in oracle_measurements.iter().zip(tolerance_entries) {
                let name = oracle["name"].as_str().expect("validated name");
                let observed = reported
                    .iter()
                    .find(|entry| entry["name"] == oracle["name"])
                    .ok_or_else(|| fail(format!("engine reported no measurement {name}")))?;
                if observed["unit"] != oracle["unit"]
                    || observed["sample_count"] != oracle["sample_count"]
                    || observed["series_sha256"] != oracle["series_sha256"]
                {
                    return Err(fail(format!(
                        "measurement {name} series identity differs from the oracle"
                    )));
                }
                let observed_decimal = observed["value_decimal"]
                    .as_str()
                    .ok_or_else(|| fail(format!("measurement {name} has no value")))?;
                let expected_decimal = oracle["expected_decimal"].as_str().expect("validated");
                let absolute = tolerance["absolute_tolerance_decimal"]
                    .as_str()
                    .expect("validated");
                let relative = tolerance["relative_tolerance_decimal"]
                    .as_str()
                    .expect("validated");
                let comparison =
                    within_tolerance(observed_decimal, expected_decimal, absolute, relative)
                        .ok_or_else(|| fail(format!("measurement {name} is not comparable")))?;
                if !comparison.passed {
                    return Err(fail(format!(
                        "measurement {name}: engine value {observed_decimal} is outside \
                         tolerance of {expected_decimal} (error {})",
                        comparison.absolute_error_decimal
                    )));
                }
                raw_measurements.push(json!({
                    "name": oracle["name"],
                    "unit": oracle["unit"],
                    "observed_decimal": observed_decimal,
                    "sample_count": oracle["sample_count"],
                    "series_sha256": oracle["series_sha256"],
                }));
                evaluator_measurements.push(json!({
                    "name": oracle["name"],
                    "unit": oracle["unit"],
                    "expected_decimal": expected_decimal,
                    "observed_decimal": observed_decimal,
                    "absolute_tolerance_decimal": absolute,
                    "relative_tolerance_decimal": relative,
                    "absolute_error_decimal": comparison.absolute_error_decimal,
                    "passed": true,
                }));
            }
        }

        let run_id = json!(execution.simulation_run_id);
        let execution_payload = json!({
            "format_version": 3,
            "case_id": execution.case_id,
            "platform": execution.platform,
            "repeat_ordinal": execution.repeat_ordinal,
            "simulation_run_public_id": run_id,
            "request_sha256": execution.request_sha256,
            "revision_sha256": execution.revision_sha256,
            "worker_image_reference": binding.worker_image_reference,
            "worker_platform_digest": binding.worker_platform_digest,
            "runtime_mode": binding.runtime_mode,
            "engine_name": binding.engine_name,
            "engine_build": binding.engine_build,
            "engine_protocol_version": 3,
            "adapter_image_reference": binding.adapter_image_reference,
            "adapter_sha256": binding.adapter_sha256,
            "solver_image_reference": binding.solver_image_reference,
            "solver_sha256": binding.solver_sha256,
            "model_library_image_reference": binding.model_library_image_reference,
            "model_library_sha256": binding.model_library_sha256,
            "observed_outcome": observed_outcome,
            "observed_error_code": observed_error_code,
        });
        let trace_payload = json!({
            "format_version": 1,
            "case_id": execution.case_id,
            "platform": execution.platform,
            "repeat_ordinal": execution.repeat_ordinal,
            "simulation_run_public_id": run_id,
            "observer_image_reference": binding.observer_image_reference,
            "adapter_invocation_count": 1,
            "solver_invocation_count": if binding.runtime_mode == "delegating" { 1 } else { 0 },
            "model_library_read_count": if binding.model_library_sha256.is_some() { 1 } else { 0 },
        });
        let raw_payload = json!({
            "format_version": 1,
            "case_id": execution.case_id,
            "platform": execution.platform,
            "repeat_ordinal": execution.repeat_ordinal,
            "simulation_run_public_id": run_id,
            "observed_outcome": observed_outcome,
            "observed_error_code": observed_error_code,
            "measurements": raw_measurements,
        });
        let evaluator_payload = json!({
            "format_version": 1,
            "case_id": execution.case_id,
            "platform": execution.platform,
            "repeat_ordinal": execution.repeat_ordinal,
            "simulation_run_public_id": run_id,
            "result": "passed",
            "measurements": evaluator_measurements,
        });

        case_result_rows.push(json!({
            "case_id": execution.case_id,
            "platform": execution.platform,
            "repeat_ordinal": execution.repeat_ordinal,
            "result": "passed",
            "expected_outcome": execution.expected_outcome,
            "expected_error_code": execution.expected_error_code,
            "observed_outcome": observed_outcome,
            "observed_error_code": observed_error_code,
            "simulation_run_public_id": run_id,
            "request_sha256": execution.request_sha256,
            "revision_sha256": execution.revision_sha256,
            "execution_manifest_sha256": canonical_sha256(&execution_payload)
                .ok_or_else(|| fail("execution manifest digest".to_owned()))?,
            "invocation_trace_sha256": canonical_sha256(&trace_payload)
                .ok_or_else(|| fail("trace digest".to_owned()))?,
            "oracle_payload_sha256": case["oracle_manifest_sha256"],
            "tolerance_payload_sha256": case["tolerance_spec_sha256"],
            "result_manifest_sha256": canonical_sha256(&raw_payload)
                .ok_or_else(|| fail("raw result digest".to_owned()))?,
            "measurement_report_sha256": canonical_sha256(&evaluator_payload)
                .ok_or_else(|| fail("evaluator digest".to_owned()))?,
        }));
        let entry = |payload: Value| {
            json!({
                "case_id": execution.case_id,
                "platform": execution.platform,
                "repeat_ordinal": execution.repeat_ordinal,
                "payload": payload,
            })
        };
        execution_entries.push(entry(execution_payload));
        trace_entries.push(entry(trace_payload));
        raw_entries.push(entry(raw_payload));
        evaluator_entries.push(entry(evaluator_payload));
    }

    let aggregate = |entries: Vec<Value>| {
        json!({
            "format_version": 1,
            "case_set_sha256": summary.case_set_sha256,
            "entries": entries,
        })
    };
    let case_entries = |field: &str| {
        cases
            .iter()
            .map(|case| {
                json!({
                    "case_id": case["case_id"],
                    "payload": case[field],
                })
            })
            .collect::<Vec<_>>()
    };

    write_artifact(output, "simulator-qualification-policy.json", policy_bytes)?;
    write_artifact(output, "simulator-qualification-corpus.json", corpus_bytes)?;
    let serialize = |value: &Value| serde_json::to_vec(value).expect("serializable artifact");
    write_artifact(
        output,
        "case-results.json",
        &serialize(&json!({
            "format_version": 1,
            "suite_run_id": suite_run_id,
            "case_set_sha256": summary.case_set_sha256,
            "results": case_result_rows,
        })),
    )?;
    write_artifact(
        output,
        "oracle-manifest.json",
        &serialize(&aggregate(case_entries("oracle"))),
    )?;
    write_artifact(
        output,
        "tolerance-manifest.json",
        &serialize(&aggregate(case_entries("tolerance"))),
    )?;
    write_artifact(
        output,
        "execution-manifests.json",
        &serialize(&aggregate(execution_entries)),
    )?;
    write_artifact(
        output,
        "invocation-traces.json",
        &serialize(&aggregate(trace_entries)),
    )?;
    write_artifact(
        output,
        "raw-results.json",
        &serialize(&aggregate(raw_entries)),
    )?;
    write_artifact(
        output,
        "evaluator-report.json",
        &serialize(&aggregate(evaluator_entries)),
    )?;

    // Component contracts: struct serialization preserves the load-bearing
    // key order the admission verifier compares against.
    #[derive(Serialize)]
    struct ComponentContracts<'a> {
        format_version: i32,
        engine_name: &'a str,
        engine_build: &'a str,
        protocol_versions: [i32; 1],
        components: &'a [ComponentContract],
    }
    write_artifact(
        output,
        "component-contracts.json",
        &serde_json::to_vec(&ComponentContracts {
            format_version: 1,
            engine_name: &binding.engine_name,
            engine_build: &binding.engine_build,
            protocol_versions: [3],
            components: &binding.components,
        })
        .expect("serializable contracts"),
    )?;

    let log_digest: [u8; 32] = Sha256::digest(log_bytes).into();
    write_artifact(
        output,
        "logs.json",
        &serialize(&json!({
            "format_version": 1,
            "suite_run_id": suite_run_id,
            "redacted": true,
            "entries_sha256": digest_hex(&log_digest),
        })),
    )?;

    let category_counts_in_policy_order: Vec<(String, usize)> = contract::REQUIRED_CATEGORIES
        .iter()
        .map(|category| {
            (
                (*category).to_owned(),
                summary.category_counts.get(*category).copied().unwrap_or(0),
            )
        })
        .collect();
    // The finalizer reads this to build the evidence document; category
    // order is the policy order, carried as an array because JSON objects
    // through this crate's canonical serialization would re-sort keys.
    write_artifact(
        output,
        "assembly-summary.json",
        &serialize(&json!({
            "format_version": 1,
            "suite_run_id": suite_run_id,
            "case_set_sha256": summary.case_set_sha256,
            "executed_case_count": executions.len(),
            "required_case_count": executions.len(),
            "categories_in_policy_order": category_counts_in_policy_order
                .iter()
                .map(|(name, count)| json!({"key": name, "value": count}))
                .collect::<Vec<_>>(),
        })),
    )?;

    Ok(AssembleSummary {
        case_set_sha256: summary.case_set_sha256,
        executed_case_count: executions.len(),
        category_counts_in_policy_order,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn the_plan_covers_every_execution_with_unique_parseable_requests() {
        let (policy, corpus) = crate::emit::canonical_documents().expect("generate documents");
        let suite_run_id = Uuid::from_u128(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef);
        let executions = plan(&policy, &corpus, suite_run_id).expect("plan the suite");

        let corpus_value: Value = serde_json::from_slice(&corpus).expect("corpus json");
        let expected: usize = corpus_value["cases"]
            .as_array()
            .expect("cases")
            .iter()
            .map(|case| {
                case["platforms"].as_array().expect("platforms").len()
                    * usize::try_from(case["required_repetitions"].as_u64().expect("reps"))
                        .expect("small")
            })
            .sum();
        assert_eq!(executions.len(), expected);

        let mut run_ids = BTreeSet::new();
        for execution in &executions {
            assert!(
                run_ids.insert(execution.simulation_run_id),
                "duplicate run id for {}",
                execution.case_id
            );
        }

        // Determinism: the same corpus and suite run id replan to the same
        // identifiers and request bytes.
        let replanned = plan(&policy, &corpus, suite_run_id).expect("replan");
        assert_eq!(executions.len(), replanned.len());
        for (first, second) in executions.iter().zip(&replanned) {
            assert_eq!(first.simulation_run_id, second.simulation_run_id);
            assert_eq!(first.request_bytes, second.request_bytes);
        }
    }

    /// The full offline loop: plan the suite, execute every request
    /// through the live engine library, and assemble the retained
    /// evidence artifacts. A drift anywhere — corpus, adapter semantics,
    /// digest recipes, oracle gating — fails here before it can reach a
    /// dispatched harness run.
    #[test]
    fn the_assembled_evidence_artifacts_cover_every_execution() {
        use rspice_engine_adapter::document::CircuitContent;
        use rspice_engine_adapter::execute::execute;

        let (policy, corpus) = crate::emit::canonical_documents().expect("generate documents");
        let suite_run_id = Uuid::from_u128(0xfeed_face_0123_4567_89ab_cdef_0123_4567);
        let executions = plan(&policy, &corpus, suite_run_id).expect("plan the suite");

        let scratch = std::env::temp_dir().join(format!("rspice-harness-{suite_run_id}"));
        let responses = scratch.join("responses");
        let artifacts = scratch.join("artifacts");
        std::fs::create_dir_all(&responses).expect("create response directory");

        for (ordinal, execution) in executions.iter().enumerate() {
            let request =
                parse_engine_request(&execution.request_bytes).expect("planned request parses");
            let netlist = request.revision.document["netlist_utf8"]
                .as_str()
                .expect("planned document");
            let outcome = execute(
                &request.analysis,
                &CircuitContent::Deck {
                    expanded_netlist: netlist.to_owned(),
                },
                "harness-test",
            );
            let bytes = serde_json::to_vec(&outcome.response).expect("serializable response");
            std::fs::write(responses.join(format!("{ordinal:05}.response.json")), bytes)
                .expect("write response");
        }

        let binding = ReleaseBinding {
            worker_image_reference:
                "ghcr.io/example/rspice-cloud-worker:0.1.0-test@sha256:1111111111111111111111111111111111111111111111111111111111111111"
                    .to_owned(),
            worker_platform_digest:
                "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                    .to_owned(),
            runtime_mode: "self_contained".to_owned(),
            engine_name: "rspice".to_owned(),
            engine_build: "0.1.0+testsha".to_owned(),
            adapter_image_reference:
                "ghcr.io/example/rspice-engine-adapter:0.1.0-test@sha256:3333333333333333333333333333333333333333333333333333333333333333"
                    .to_owned(),
            adapter_sha256: "4".repeat(64),
            solver_image_reference: None,
            solver_sha256: None,
            model_library_image_reference: None,
            model_library_sha256: None,
            observer_image_reference:
                "ghcr.io/example/rspice-qualification-harness:0.1.0-test@sha256:5555555555555555555555555555555555555555555555555555555555555555"
                    .to_owned(),
            components: vec![ComponentContract {
                role: "adapter".to_owned(),
                image_reference:
                    "ghcr.io/example/rspice-engine-adapter:0.1.0-test@sha256:3333333333333333333333333333333333333333333333333333333333333333"
                        .to_owned(),
                exported_file_sha256: "4".repeat(64),
                predicate_type: "https://rspice.app/attestations/simulator-component/v1"
                    .to_owned(),
                attestation_statement_sha256: "6".repeat(64),
                source_repository: "example/rspice".to_owned(),
                source_sha: "7".repeat(40),
                build_recipe_sha256: "8".repeat(64),
                capabilities_manifest_sha256: "9".repeat(64),
            }],
        };
        let summary = assemble(
            &policy,
            &corpus,
            suite_run_id,
            &responses,
            &binding,
            b"redacted harness log\n",
            &artifacts,
        )
        .expect("assemble the retained artifacts");

        assert_eq!(summary.executed_case_count, executions.len());
        assert_eq!(summary.category_counts_in_policy_order.len(), 12);
        for file_name in [
            "assembly-summary.json",
            "case-results.json",
            "component-contracts.json",
            "evaluator-report.json",
            "execution-manifests.json",
            "invocation-traces.json",
            "logs.json",
            "oracle-manifest.json",
            "raw-results.json",
            "simulator-qualification-corpus.json",
            "simulator-qualification-policy.json",
            "tolerance-manifest.json",
        ] {
            let path = artifacts.join(file_name);
            assert!(path.is_file(), "missing artifact {file_name}");
        }
        let results: Value = serde_json::from_slice(
            &std::fs::read(artifacts.join("case-results.json")).expect("read case results"),
        )
        .expect("case results json");
        assert_eq!(
            results["results"].as_array().map(Vec::len),
            Some(executions.len())
        );
        assert_eq!(results["case_set_sha256"], json!(summary.case_set_sha256));
        // The component-contracts serialization must keep the reviewed key
        // order; spot-check the first two keys are not alphabetized.
        let contracts = std::fs::read_to_string(artifacts.join("component-contracts.json"))
            .expect("read contracts");
        let role_at = contracts.find("\"role\"").expect("role key");
        let image_at = contracts.find("\"image_reference\"").expect("image key");
        assert!(role_at < image_at, "component key order must be preserved");

        std::fs::remove_dir_all(&scratch).ok();
    }
}
