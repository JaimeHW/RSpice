//! Stable, versioned hashing contract for immutable circuit revisions.

use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use thiserror::Error;
use uuid::Uuid;

pub const LEGACY_REVISION_CONTENT_DIGEST_VERSION: i16 = 1;
pub const CURRENT_REVISION_CONTENT_DIGEST_VERSION: i16 = 2;

/// Storage-verified input metadata that contributes to revision identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevisionArtifactIntegrity {
    pub id: Uuid,
    pub kind: String,
    pub file_name: Option<String>,
    pub content_type: String,
    pub size_bytes: u64,
    pub sha256: [u8; 32],
}

#[derive(Debug, Error)]
pub enum RevisionDigestError {
    #[error("unsupported revision content digest version {0}")]
    UnsupportedVersion(i16),
    #[error("revision manifest could not be serialized")]
    Serialization(#[from] serde_json::Error),
}

/// Computes the exact digest contract associated with a persisted revision.
///
/// Version 1 is retained solely to verify revisions created before the
/// versioned manifest existed. Version 2 commits to the schema version and to
/// the identity, interpretation, size, and SHA-256 digest of every artifact.
pub fn revision_content_digest(
    content_digest_version: i16,
    schema_version: i32,
    document: &Value,
    artifacts: &[RevisionArtifactIntegrity],
) -> Result<[u8; 32], RevisionDigestError> {
    match content_digest_version {
        LEGACY_REVISION_CONTENT_DIGEST_VERSION => legacy_digest(document, artifacts),
        CURRENT_REVISION_CONTENT_DIGEST_VERSION => {
            current_digest(schema_version, document, artifacts)
        }
        version => Err(RevisionDigestError::UnsupportedVersion(version)),
    }
}

fn legacy_digest(
    document: &Value,
    artifacts: &[RevisionArtifactIntegrity],
) -> Result<[u8; 32], RevisionDigestError> {
    if artifacts.is_empty() {
        return hash_json(document);
    }
    let mut artifact_ids = artifacts
        .iter()
        .map(|artifact| artifact.id)
        .collect::<Vec<_>>();
    artifact_ids.sort_unstable();
    hash_json(&json!({
        "document": document,
        "artifact_ids": artifact_ids,
    }))
}

fn current_digest(
    schema_version: i32,
    document: &Value,
    artifacts: &[RevisionArtifactIntegrity],
) -> Result<[u8; 32], RevisionDigestError> {
    let mut artifacts = artifacts.iter().collect::<Vec<_>>();
    artifacts.sort_unstable_by_key(|artifact| artifact.id);
    let artifacts = artifacts
        .into_iter()
        .map(|artifact| RevisionArtifactDigestManifest {
            id: artifact.id,
            kind: &artifact.kind,
            file_name: artifact.file_name.as_deref(),
            content_type: &artifact.content_type,
            size_bytes: artifact.size_bytes,
            sha256: digest_hex(&artifact.sha256),
        })
        .collect();
    hash_json(&RevisionDigestManifest {
        content_digest_version: CURRENT_REVISION_CONTENT_DIGEST_VERSION,
        schema_version,
        document,
        artifacts,
    })
}

fn hash_json(value: &impl Serialize) -> Result<[u8; 32], RevisionDigestError> {
    let bytes = serde_json::to_vec(value)?;
    Ok(Sha256::digest(bytes).into())
}

#[derive(Serialize)]
struct RevisionDigestManifest<'a> {
    content_digest_version: i16,
    schema_version: i32,
    document: &'a Value,
    artifacts: Vec<RevisionArtifactDigestManifest<'a>>,
}

#[derive(Serialize)]
struct RevisionArtifactDigestManifest<'a> {
    id: Uuid,
    kind: &'a str,
    file_name: Option<&'a str>,
    content_type: &'a str,
    size_bytes: u64,
    sha256: String,
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

    fn artifact(id: Uuid, digest_byte: u8) -> RevisionArtifactIntegrity {
        RevisionArtifactIntegrity {
            id,
            kind: "model_library".to_owned(),
            file_name: Some("model.lib".to_owned()),
            content_type: "text/plain".to_owned(),
            size_bytes: 128,
            sha256: [digest_byte; 32],
        }
    }

    #[test]
    fn current_digest_is_order_independent_and_commits_to_schema_and_bytes() {
        let first = artifact(Uuid::now_v7(), 1);
        let second = artifact(Uuid::now_v7(), 2);
        let document = json!({"components": []});
        let artifacts = vec![first.clone(), second.clone()];

        let first_order = revision_content_digest(
            CURRENT_REVISION_CONTENT_DIGEST_VERSION,
            1,
            &document,
            &artifacts,
        )
        .expect("current digest");
        let second_order = revision_content_digest(
            CURRENT_REVISION_CONTENT_DIGEST_VERSION,
            1,
            &document,
            &[second, first.clone()],
        )
        .expect("current digest");
        let changed_schema = revision_content_digest(
            CURRENT_REVISION_CONTENT_DIGEST_VERSION,
            2,
            &document,
            &artifacts,
        )
        .expect("current digest");
        let mut changed_artifacts = artifacts;
        changed_artifacts[0].sha256 = [3; 32];
        let changed_bytes = revision_content_digest(
            CURRENT_REVISION_CONTENT_DIGEST_VERSION,
            1,
            &document,
            &changed_artifacts,
        )
        .expect("current digest");

        assert_eq!(first_order, second_order);
        assert_ne!(first_order, changed_schema);
        assert_ne!(first_order, changed_bytes);
    }

    #[test]
    fn legacy_digest_preserves_document_only_contract() {
        let document = json!({"a": 1, "b": [2, 3]});
        let actual =
            revision_content_digest(LEGACY_REVISION_CONTENT_DIGEST_VERSION, 99, &document, &[])
                .expect("legacy digest");
        let expected: [u8; 32] =
            Sha256::digest(serde_json::to_vec(&document).expect("serialize document")).into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn unknown_digest_versions_fail_closed() {
        assert!(matches!(
            revision_content_digest(7, 1, &json!({}), &[]),
            Err(RevisionDigestError::UnsupportedVersion(7))
        ));
    }
}
