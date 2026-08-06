//! Qualification harness planning: expands the published corpus into the
//! exact protocol-3 engine requests the released worker adapter executes,
//! one per `(case, platform, repeat)` execution.
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
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::contract;

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
}
