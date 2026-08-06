//! Stable hashing contract for a resolved remote-simulation request.

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

pub const CURRENT_SIMULATION_REQUEST_DIGEST_VERSION: i16 = 1;

#[derive(Debug, Error)]
pub enum SimulationRequestDigestError {
    #[error("unsupported simulation request digest version {0}")]
    UnsupportedDigestVersion(i16),
    #[error("simulation request manifest could not be serialized")]
    Serialization(#[from] serde_json::Error),
}

/// Commits a queued run to its resolved circuit/revision, immutable revision
/// digest, and normalized analysis payload. Both API and worker call this
/// function so persisted request tampering fails before solver execution.
pub fn simulation_request_digest(
    digest_version: i16,
    circuit_public_id: Uuid,
    revision_id: Uuid,
    revision_sha256: &[u8; 32],
    analysis: &Value,
) -> Result<[u8; 32], SimulationRequestDigestError> {
    if digest_version != CURRENT_SIMULATION_REQUEST_DIGEST_VERSION {
        return Err(SimulationRequestDigestError::UnsupportedDigestVersion(
            digest_version,
        ));
    }
    let manifest = json!({
        "analysis": analysis,
        "circuit_id": circuit_public_id,
        "revision_id": revision_id,
        "revision_sha256": digest_hex(revision_sha256),
    });
    let bytes = serde_json::to_vec(&manifest)?;
    Ok(Sha256::digest(bytes).into())
}

fn digest_hex(bytes: &[u8; 32]) -> String {
    use std::fmt::Write as _;

    let mut encoded = String::with_capacity(64);
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}").expect("writing to String cannot fail");
    }
    encoded
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn request_digest_is_stable_and_binds_revision_identity() {
        let circuit_id = Uuid::now_v7();
        let revision_id = Uuid::now_v7();
        let analysis = json!({"kind": "operating_point"});
        let first = simulation_request_digest(
            CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
            circuit_id,
            revision_id,
            &[1; 32],
            &analysis,
        )
        .expect("request digest");
        let repeated = simulation_request_digest(
            CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
            circuit_id,
            revision_id,
            &[1; 32],
            &analysis,
        )
        .expect("request digest");
        let changed_revision = simulation_request_digest(
            CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
            circuit_id,
            Uuid::now_v7(),
            &[1; 32],
            &analysis,
        )
        .expect("request digest");
        let changed_bytes = simulation_request_digest(
            CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
            circuit_id,
            revision_id,
            &[2; 32],
            &analysis,
        )
        .expect("request digest");

        assert_eq!(first, repeated);
        assert_ne!(first, changed_revision);
        assert_ne!(first, changed_bytes);
        assert!(
            simulation_request_digest(2, circuit_id, revision_id, &[1; 32], &analysis).is_err()
        );
    }
}
