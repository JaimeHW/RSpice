//! Engine wire contract: the exact protocol-4 request/response boundary the
//! RSpice Cloud worker speaks to a self-contained engine executor.
//!
//! # Protocol 4
//!
//! The request envelope is unchanged from protocol 3: the same digest
//! versions, the same artifact manifest, the same byte bounds. The version was
//! advanced because the *response* contract changed — every analysis family
//! the planner recognizes now executes, each publishing one shared
//! `rspice-analysis-result` document instead of a CSV plus an adapter-owned
//! analog document, and `analysis.kind` `mixed_signal` was removed. See
//! [`crate::execute`] for the complete consumer-visible difference. Protocol 3
//! is refused here rather than served with a response its reader cannot
//! interpret.
//!
//! This module deliberately reimplements the `rspice-cloud-engine-contract`
//! crate rather than depending on it across repositories. Byte-for-byte
//! equivalence is enforced three ways: the release smoke request's pinned
//! digests are asserted in tests below, the release probe replays the same
//! request against the packaged binary, and every conformance case
//! exercises the digest recomputation end to end. Any drift fails closed
//! before a release can be admitted.
//!
//! Canonicalization invariant: all hashing serializes through `serde_json`
//! with its default `BTreeMap` object representation (sorted keys). Enabling
//! `preserve_order` anywhere in this crate's dependency graph would silently
//! change every digest; the pinned vectors exist to catch exactly that.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

pub const INTEGRITY_ENGINE_PROTOCOL_VERSION: u8 = 4;
pub const CURRENT_SIMULATION_REQUEST_DIGEST_VERSION: i16 = 1;
pub const CURRENT_REVISION_CONTENT_DIGEST_VERSION: i16 = 2;
pub const MAX_ENGINE_REQUEST_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_ENGINE_RESPONSE_BYTES: usize = 256 * 1024;
pub const MAX_ENGINE_ANALYSIS_BYTES: usize = 128 * 1024;
pub const MAX_ENGINE_DOCUMENT_BYTES: usize = 1024 * 1024;
pub const MAX_ENGINE_RESULT_MANIFEST_BYTES: usize = 128 * 1024;
pub const MAX_ENGINE_RESULT_ARTIFACTS: usize = 100;
pub const MAX_ENGINE_ARTIFACTS: usize = 100;
pub const MAX_ENGINE_ARTIFACT_BYTES: u64 = 50 * 1024 * 1024 * 1024;
/// Maximum encoded result bytes retained in adapter memory for one response.
///
/// The protocol's 50-GiB per-file ceiling is a storage contract, not a safe
/// process-memory budget. The adapter currently stages result files in memory,
/// so it applies this independent aggregate ceiling until result streaming is
/// introduced.
pub const MAX_ENGINE_RETAINED_RESULT_BYTES: u64 = 512 * 1024 * 1024;
pub const MAX_SIMULATION_ATTEMPTS: i32 = 20;
pub const MAX_FAILURE_CODE_BYTES: usize = 120;
pub const MAX_FAILURE_DETAIL_CHARS: usize = 1_024;

/// Exact protocol-4 request delivered on standard input.
///
/// The worker validated this envelope before launch; the adapter validates it
/// again so a compromised or drifted controller cannot feed it content whose
/// integrity digests do not match. Protocols 1, 2, and 3 are earlier response
/// contracts; this executor accepts only protocol 4, and any other version —
/// older or newer — is refused as controller drift rather than served a
/// response its reader cannot interpret.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EngineRequest {
    pub protocol_version: u8,
    pub simulation_run_id: Uuid,
    pub circuit_id: Uuid,
    pub attempt: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_digest_version: Option<i16>,
    pub request_sha256: String,
    pub analysis: Value,
    pub revision: EngineRevision,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EngineRevision {
    pub id: Uuid,
    pub schema_version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_digest_version: Option<i16>,
    pub content_sha256: String,
    pub document: Value,
    pub artifacts: Vec<EngineArtifact>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EngineArtifact {
    pub id: Uuid,
    pub kind: String,
    pub file_name: Option<String>,
    pub content_type: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub path: String,
}

/// Strict wire response emitted on standard output.
#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum EngineResponse {
    Succeeded {
        result_manifest: Value,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        result_artifacts: Vec<EngineResultArtifactDescriptor>,
    },
    Failed {
        failure_code: String,
        failure_detail: String,
    },
}

#[derive(Serialize)]
pub struct EngineResultArtifactDescriptor {
    pub path: String,
    pub content_type: String,
}

impl EngineResponse {
    /// Canonical bounded failure response. The code must already satisfy the
    /// wire grammar; the detail is normalized here so callers can pass
    /// engine diagnostics through without re-checking control characters.
    pub fn failed(failure_code: &str, failure_detail: &str) -> Self {
        debug_assert!(valid_failure_code(failure_code));
        Self::Failed {
            failure_code: failure_code.to_owned(),
            failure_detail: sanitize_failure_detail(failure_detail),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum RequestError {
    #[error("engine request exceeds its byte limit")]
    RequestTooLarge,
    #[error("engine request is not the strict JSON request contract")]
    InvalidDocument,
    #[error("engine request structure is invalid")]
    InvalidStructure,
    #[error("engine request protocol version is unsupported or inconsistent")]
    InvalidProtocol,
    #[error("engine request artifact manifest is invalid")]
    InvalidArtifact,
    #[error("engine request revision digest does not match its content")]
    RevisionIntegrity,
    #[error("engine request digest does not match its resolved request")]
    RequestIntegrity,
}

/// Bounds, strictly decodes, and semantically validates the one request this
/// process serves. The sole request decoding entry point.
pub fn parse_engine_request(bytes: &[u8]) -> Result<EngineRequest, RequestError> {
    if bytes.len() > MAX_ENGINE_REQUEST_BYTES {
        return Err(RequestError::RequestTooLarge);
    }
    let request: EngineRequest =
        serde_json::from_slice(bytes).map_err(|_| RequestError::InvalidDocument)?;
    validate_engine_request(&request)?;
    Ok(request)
}

fn validate_engine_request(request: &EngineRequest) -> Result<(), RequestError> {
    if request.simulation_run_id.is_nil()
        || request.circuit_id.is_nil()
        || !(1..=MAX_SIMULATION_ATTEMPTS).contains(&request.attempt)
        || request.revision.id.is_nil()
        || request.revision.schema_version <= 0
        || !bounded_json_object(&request.analysis, MAX_ENGINE_ANALYSIS_BYTES)
        || !bounded_json_object(&request.revision.document, MAX_ENGINE_DOCUMENT_BYTES)
    {
        return Err(RequestError::InvalidStructure);
    }

    // A protocol-4 component set only ever receives protocol 4 with both
    // digest versions explicit; anything else — including a future protocol
    // this build has never seen — is controller drift.
    if request.protocol_version != INTEGRITY_ENGINE_PROTOCOL_VERSION
        || request.request_digest_version != Some(CURRENT_SIMULATION_REQUEST_DIGEST_VERSION)
        || request.revision.content_digest_version != Some(CURRENT_REVISION_CONTENT_DIGEST_VERSION)
    {
        return Err(RequestError::InvalidProtocol);
    }

    let request_digest =
        decode_digest(&request.request_sha256).ok_or(RequestError::InvalidStructure)?;
    let asserted_revision_digest =
        decode_digest(&request.revision.content_sha256).ok_or(RequestError::InvalidStructure)?;

    validate_artifacts(&request.revision.artifacts)?;

    let computed_revision_digest = revision_content_digest(
        request.revision.schema_version,
        &request.revision.document,
        &request.revision.artifacts,
    )
    .map_err(|_| RequestError::RevisionIntegrity)?;
    if computed_revision_digest != asserted_revision_digest {
        return Err(RequestError::RevisionIntegrity);
    }

    let computed_request_digest = simulation_request_digest(
        request.circuit_id,
        request.revision.id,
        &computed_revision_digest,
        &request.analysis,
    )
    .map_err(|_| RequestError::RequestIntegrity)?;
    if computed_request_digest != request_digest {
        return Err(RequestError::RequestIntegrity);
    }
    Ok(())
}

fn validate_artifacts(artifacts: &[EngineArtifact]) -> Result<(), RequestError> {
    if artifacts.len() > MAX_ENGINE_ARTIFACTS {
        return Err(RequestError::InvalidArtifact);
    }
    let mut prior_id = None;
    for artifact in artifacts {
        if decode_digest(&artifact.sha256).is_none()
            || artifact.id.is_nil()
            || prior_id.is_some_and(|prior| prior >= artifact.id)
            || !matches!(
                artifact.kind.as_str(),
                "circuit_attachment" | "model_library"
            )
            || artifact
                .file_name
                .as_deref()
                .is_some_and(|name| !valid_file_name(name))
            || canonical_media_type(&artifact.content_type).as_deref()
                != Some(artifact.content_type.as_str())
            || !(1..=MAX_ENGINE_ARTIFACT_BYTES).contains(&artifact.size_bytes)
            || artifact.path != format!("artifacts/{}/object", artifact.id)
        {
            return Err(RequestError::InvalidArtifact);
        }
        prior_id = Some(artifact.id);
    }
    Ok(())
}

/// Version-2 revision digest: commits to the schema version, the document,
/// and the identity, interpretation, size, and SHA-256 of every artifact.
/// Field order matches the manifest struct in `rspice-cloud-domain` exactly.
/// Public so tests and integration tooling build requests through the same
/// canonicalization the executor verifies against.
pub fn revision_content_digest(
    schema_version: i32,
    document: &Value,
    artifacts: &[EngineArtifact],
) -> Result<[u8; 32], serde_json::Error> {
    #[derive(Serialize)]
    struct RevisionDigestManifest<'a> {
        content_digest_version: i16,
        schema_version: i32,
        document: &'a Value,
        artifacts: Vec<ArtifactDigestManifest<'a>>,
    }
    #[derive(Serialize)]
    struct ArtifactDigestManifest<'a> {
        id: Uuid,
        kind: &'a str,
        file_name: Option<&'a str>,
        content_type: &'a str,
        size_bytes: u64,
        sha256: &'a str,
    }

    let mut sorted = artifacts.iter().collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|artifact| artifact.id);
    let artifacts = sorted
        .into_iter()
        .map(|artifact| ArtifactDigestManifest {
            id: artifact.id,
            kind: &artifact.kind,
            file_name: artifact.file_name.as_deref(),
            content_type: &artifact.content_type,
            size_bytes: artifact.size_bytes,
            sha256: &artifact.sha256,
        })
        .collect();
    let bytes = serde_json::to_vec(&RevisionDigestManifest {
        content_digest_version: CURRENT_REVISION_CONTENT_DIGEST_VERSION,
        schema_version,
        document,
        artifacts,
    })?;
    Ok(Sha256::digest(bytes).into())
}

/// Version-1 resolved-request digest binding the run to its exact circuit,
/// revision, revision digest, and normalized analysis payload.
pub fn simulation_request_digest(
    circuit_id: Uuid,
    revision_id: Uuid,
    revision_sha256: &[u8; 32],
    analysis: &Value,
) -> Result<[u8; 32], serde_json::Error> {
    let manifest = serde_json::json!({
        "analysis": analysis,
        "circuit_id": circuit_id,
        "revision_id": revision_id,
        "revision_sha256": digest_hex(revision_sha256),
    });
    let bytes = serde_json::to_vec(&manifest)?;
    Ok(Sha256::digest(bytes).into())
}

pub fn digest_hex(bytes: &[u8; 32]) -> String {
    use std::fmt::Write as _;

    let mut encoded = String::with_capacity(64);
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}").expect("writing to String cannot fail");
    }
    encoded
}

pub fn decode_digest(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 {
        return None;
    }
    let mut output = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        output[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
    }
    Some(output)
}

const fn hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

fn bounded_json_object(value: &Value, maximum: usize) -> bool {
    value.is_object() && serde_json::to_vec(value).is_ok_and(|bytes| bytes.len() <= maximum)
}

pub fn valid_file_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && value.trim() == value
        && !matches!(value, "." | "..")
        && !value.chars().any(char::is_control)
        && !value.contains(['/', '\\'])
}

/// Single-component result path below `results/`, exactly as the worker
/// re-validates it before registration.
pub fn valid_result_path(value: &str) -> bool {
    value.strip_prefix("results/").is_some_and(|file_name| {
        !file_name.is_empty()
            && file_name.len() <= 255
            && !matches!(file_name, "." | "..")
            && file_name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    })
}

pub fn canonical_media_type(value: &str) -> Option<String> {
    let value = value.trim().to_ascii_lowercase();
    let (kind, subtype) = value.split_once('/')?;
    (value.len() <= 255
        && !kind.is_empty()
        && !subtype.is_empty()
        && value.matches('/').count() == 1
        && kind.bytes().all(valid_media_token_byte)
        && subtype.bytes().all(valid_media_token_byte))
    .then_some(value)
}

const fn valid_media_token_byte(value: u8) -> bool {
    value.is_ascii_alphanumeric()
        || matches!(
            value,
            b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-'
        )
}

pub fn valid_failure_code(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_FAILURE_CODE_BYTES
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

fn sanitize_failure_detail(value: &str) -> String {
    let normalized: String = value
        .chars()
        .filter_map(|character| {
            if character.is_control() {
                matches!(character, '\n' | '\r' | '\t').then_some(' ')
            } else {
                Some(character)
            }
        })
        .collect();
    let bounded: String = normalized
        .trim()
        .chars()
        .take(MAX_FAILURE_DETAIL_CHARS)
        .collect();
    if bounded.is_empty() {
        "The simulation engine reported an unspecified failure.".to_owned()
    } else {
        bounded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact deterministic request the release workflow's probe replays
    /// against the packaged adapter. Its two digests were computed by the
    /// authoritative `rspice-cloud-domain` implementation; reproducing them
    /// here proves this module's canonicalization is byte-identical.
    pub(crate) fn release_smoke_request_bytes() -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "protocol_version": 4,
            "simulation_run_id": "019f76ae-0000-7000-8000-000000000703",
            "circuit_id": "019f76ae-0000-7000-8000-000000000701",
            "attempt": 1,
            "request_digest_version": 1,
            "request_sha256": "62ed89a0528762860da1f9897b6ecd35e0fa5c6e7eb7568f575034f92c6d7d1c",
            "analysis": {"kind": "operating_point"},
            "revision": {
                "id": "019f76ae-0000-7000-8000-000000000702",
                "schema_version": 1,
                "content_digest_version": 2,
                "content_sha256": "88767794c6fcedd647712673b43e3f4c409697cd4e4a133377ba45af4dd25bcc",
                "document": {"components": [], "schema": "rspice-circuit-v1"},
                "artifacts": []
            }
        }))
        .expect("serialize release smoke request")
    }

    #[test]
    fn release_smoke_request_digests_are_reproduced_exactly() {
        let request = parse_engine_request(&release_smoke_request_bytes())
            .expect("the pinned release smoke request must validate");
        assert_eq!(request.protocol_version, INTEGRITY_ENGINE_PROTOCOL_VERSION);
        assert!(request.revision.artifacts.is_empty());
    }

    #[test]
    fn tampered_content_fails_the_matching_integrity_digest() {
        let mut tampered: Value =
            serde_json::from_slice(&release_smoke_request_bytes()).expect("request JSON");
        tampered["revision"]["document"]["tampered"] = serde_json::json!(true);
        assert_eq!(
            parse_engine_request(&serde_json::to_vec(&tampered).expect("request bytes")).err(),
            Some(RequestError::RevisionIntegrity)
        );

        let mut tampered: Value =
            serde_json::from_slice(&release_smoke_request_bytes()).expect("request JSON");
        tampered["analysis"]["kind"] = serde_json::json!("transient");
        assert_eq!(
            parse_engine_request(&serde_json::to_vec(&tampered).expect("request bytes")).err(),
            Some(RequestError::RequestIntegrity)
        );
    }

    #[test]
    fn unknown_fields_legacy_protocols_and_oversize_fail_closed() {
        let mut extended: Value =
            serde_json::from_slice(&release_smoke_request_bytes()).expect("request JSON");
        extended["unexpected"] = serde_json::json!(true);
        assert_eq!(
            parse_engine_request(&serde_json::to_vec(&extended).expect("request bytes")).err(),
            Some(RequestError::InvalidDocument)
        );

        let mut legacy: Value =
            serde_json::from_slice(&release_smoke_request_bytes()).expect("request JSON");
        legacy["protocol_version"] = serde_json::json!(1);
        legacy
            .as_object_mut()
            .expect("request object")
            .remove("request_digest_version");
        legacy["revision"]
            .as_object_mut()
            .expect("revision object")
            .remove("content_digest_version");
        assert_eq!(
            parse_engine_request(&serde_json::to_vec(&legacy).expect("request bytes")).err(),
            Some(RequestError::InvalidProtocol)
        );

        assert_eq!(
            parse_engine_request(&vec![b' '; MAX_ENGINE_REQUEST_BYTES + 1]).err(),
            Some(RequestError::RequestTooLarge)
        );
    }

    #[test]
    fn the_superseded_and_the_unseen_protocol_are_both_refused() {
        // Protocol 3 is the response contract this build replaced, and any
        // higher number is a controller this build has never been reviewed
        // against. Serving either would publish a result set the caller
        // cannot interpret, so both fail closed on the same code.
        for version in [
            INTEGRITY_ENGINE_PROTOCOL_VERSION - 1,
            INTEGRITY_ENGINE_PROTOCOL_VERSION + 1,
        ] {
            let mut drifted: Value =
                serde_json::from_slice(&release_smoke_request_bytes()).expect("request JSON");
            drifted["protocol_version"] = serde_json::json!(version);
            assert_eq!(
                parse_engine_request(&serde_json::to_vec(&drifted).expect("request bytes")).err(),
                Some(RequestError::InvalidProtocol),
                "protocol {version} must be refused"
            );
        }
    }

    #[test]
    fn artifact_manifests_bind_ids_paths_and_digests() {
        // A single-artifact request with digests recomputed through this
        // module's own functions must round-trip its validation, and every
        // artifact-shape violation must fail closed.
        let artifact_id = Uuid::from_u128(3);
        let artifact = EngineArtifact {
            id: artifact_id,
            kind: "model_library".to_owned(),
            file_name: Some("models.lib".to_owned()),
            content_type: "text/plain".to_owned(),
            sha256: "07".repeat(32),
            size_bytes: 128,
            path: format!("artifacts/{artifact_id}/object"),
        };
        let document = serde_json::json!({"schema": "rspice-circuit-v1", "components": []});
        let analysis = serde_json::json!({"kind": "operating_point"});
        let circuit_id = Uuid::from_u128(1);
        let revision_id = Uuid::from_u128(2);
        let revision_digest =
            revision_content_digest(1, &document, std::slice::from_ref(&artifact))
                .expect("revision digest");
        let request_digest =
            simulation_request_digest(circuit_id, revision_id, &revision_digest, &analysis)
                .expect("request digest");
        let request = serde_json::json!({
            "protocol_version": 4,
            "simulation_run_id": Uuid::from_u128(4),
            "circuit_id": circuit_id,
            "attempt": 1,
            "request_digest_version": 1,
            "request_sha256": digest_hex(&request_digest),
            "analysis": analysis,
            "revision": {
                "id": revision_id,
                "schema_version": 1,
                "content_digest_version": 2,
                "content_sha256": digest_hex(&revision_digest),
                "document": document,
                "artifacts": [artifact],
            }
        });
        parse_engine_request(&serde_json::to_vec(&request).expect("request bytes"))
            .expect("valid artifact-bearing request");

        let mut escaped = request.clone();
        escaped["revision"]["artifacts"][0]["path"] = serde_json::json!("../secret");
        assert_eq!(
            parse_engine_request(&serde_json::to_vec(&escaped).expect("request bytes")).err(),
            Some(RequestError::InvalidArtifact)
        );

        let mut renamed = request;
        renamed["revision"]["artifacts"][0]["file_name"] = serde_json::json!("a/b.lib");
        assert_eq!(
            parse_engine_request(&serde_json::to_vec(&renamed).expect("request bytes")).err(),
            Some(RequestError::InvalidArtifact)
        );
    }

    #[test]
    fn responses_serialize_to_the_strict_wire_shape() {
        let succeeded = serde_json::to_value(EngineResponse::Succeeded {
            result_manifest: serde_json::json!({"format": "rspice-result-v3"}),
            result_artifacts: Vec::new(),
        })
        .expect("response JSON");
        assert_eq!(
            succeeded,
            serde_json::json!({
                "status": "succeeded",
                "result_manifest": {"format": "rspice-result-v3"},
            })
        );

        let failed = serde_json::to_value(EngineResponse::failed(
            "engine.convergence_error",
            "\u{0}\nNewton\titeration stalled",
        ))
        .expect("response JSON");
        assert_eq!(
            failed,
            serde_json::json!({
                "status": "failed",
                "failure_code": "engine.convergence_error",
                "failure_detail": "Newton iteration stalled",
            })
        );
    }

    #[test]
    fn result_paths_and_failure_codes_follow_the_worker_grammar() {
        assert!(valid_result_path("results/waveform.raw"));
        assert!(!valid_result_path("results/../secret"));
        assert!(!valid_result_path("results/a/b.raw"));
        assert!(!valid_result_path("waveform.raw"));
        assert!(valid_failure_code("dc_nonconvergence.v2"));
        assert!(!valid_failure_code("Mixed Case"));
        assert!(!valid_failure_code(""));
    }
}
