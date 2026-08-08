//! The cloud publish sequence (native).
//!
//! One publish takes canonical `.rspicepub` snapshot bytes to a live `/c/`
//! page: verify or create the circuit, upload the snapshot as a verified
//! `publication_snapshot` artifact, seal a circuit revision recording the
//! exact snapshot digest, and create the immutable publication. Every
//! creation is idempotent with a key derived from the snapshot digest, so a
//! lost response retried with the same bytes converges on the same
//! publication instead of minting a duplicate.
//!
//! This module is pure client orchestration: no UI types, no tokens stored,
//! no clocks. The executor owns credentials and drives it.

use rspice_cloud_client::contract::{
    ClientUploadArtifactKind, CreateArtifactUploadRequest, CreateCircuitRequest,
    CreateCircuitRevisionRequest, CreatePublicationRequest, Publication,
};
use rspice_cloud_client::{BearerToken, CloudClient, IdempotencyKey};
use sha2::Digest as _;

use super::{PublishReceipt, PublishRequest, PublishTarget};

/// Presentation-safe publish failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PublishError {
    /// The service rejected the command (authority, entitlement, validation).
    Rejected(String),
    /// The service could not be reached; nothing may have been committed.
    Unreachable,
    /// A response violated the client contract.
    Contract,
    /// Local snapshot staging failed (temp file for the streamed upload).
    Staging,
}

impl std::fmt::Display for PublishError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rejected(detail) => write!(formatter, "{detail}"),
            Self::Unreachable => {
                write!(
                    formatter,
                    "RSpice Cloud could not be reached; nothing was published."
                )
            }
            Self::Contract => {
                write!(
                    formatter,
                    "RSpice Cloud sent an invalid response; try again later."
                )
            }
            Self::Staging => write!(formatter, "The snapshot could not be staged for upload."),
        }
    }
}

fn classify(error: &rspice_cloud_client::CloudError) -> PublishError {
    match error {
        rspice_cloud_client::CloudError::Problem { details, .. } => {
            // RFC 9457 titles are bounded, server-authored presentation text.
            PublishError::Rejected(details.title.clone())
        }
        rspice_cloud_client::CloudError::Transport { .. } => PublishError::Unreachable,
        _ => PublishError::Contract,
    }
}

/// Schema tag for the provenance document a publish seals into the circuit
/// revision. The page's content is the snapshot artifact; the revision
/// records exactly which snapshot bytes each publication version came from.
const PUBLICATION_SOURCE_SCHEMA_VERSION: u32 = 1;

fn provenance_document(title: &str, snapshot_sha256_hex: &str) -> serde_json::Value {
    serde_json::json!({
        "kind": "rspice-publication-source",
        "snapshot_sha256": snapshot_sha256_hex,
        "title": title,
    })
}

/// Idempotency keys are digest-scoped: retrying the same snapshot bytes
/// replays the same creations instead of minting duplicates.
fn idempotency_value(prefix: &str, snapshot_sha256_hex: &str) -> String {
    format!("{prefix}-{snapshot_sha256_hex}")
}

fn idempotency_key(value: &str) -> Result<IdempotencyKey<'_>, PublishError> {
    IdempotencyKey::new(value).map_err(|_| PublishError::Contract)
}

/// Run the whole publish sequence. `access` is the current bearer token.
pub(super) async fn publish(
    cloud: &CloudClient,
    access: &str,
    request: &PublishRequest,
) -> Result<PublishReceipt, PublishError> {
    let token = BearerToken::new(access).map_err(|_| PublishError::Contract)?;
    let snapshot_sha256 = sha2::Sha256::digest(&request.snapshot_bytes);
    let sha_hex = hex_lower(&snapshot_sha256);

    // 1. Resolve or create the circuit that owns this page's lineage.
    let (workspace_id, circuit_id, revision_id) = match &request.target {
        PublishTarget::NewCircuit {
            workspace_id,
            public,
        } => {
            let command = CreateCircuitRequest {
                title: request.title.clone(),
                visibility: Some(if *public {
                    rspice_cloud_client::contract::CircuitVisibility::Public
                } else {
                    rspice_cloud_client::contract::CircuitVisibility::Unlisted
                }),
                schema_version: PUBLICATION_SOURCE_SCHEMA_VERSION,
                document: provenance_document(&request.title, &sha_hex),
                artifact_ids: None,
            };
            let key_value = idempotency_value("publish-circuit", &sha_hex);
            let circuit = cloud
                .create_circuit_idempotent(
                    &token,
                    &idempotency_key(&key_value)?,
                    *workspace_id,
                    &command,
                )
                .await
                .map_err(|error| classify(&error))?
                .into_body();
            let head = circuit.head_revision_id.ok_or(PublishError::Contract)?;
            (*workspace_id, circuit.id, head)
        }
        PublishTarget::ExistingCircuit {
            workspace_id,
            circuit_id,
        } => {
            let circuit = cloud
                .get_circuit(&token, *circuit_id)
                .await
                .map_err(|error| classify(&error))?
                .into_body();
            // Seal a fresh revision recording this snapshot's digest so the
            // new publication version pins its own provenance.
            let command = CreateCircuitRevisionRequest {
                expected_row_version: circuit.row_version,
                parent_revision_id: circuit.head_revision_id,
                schema_version: PUBLICATION_SOURCE_SCHEMA_VERSION,
                document: provenance_document(&request.title, &sha_hex),
                artifact_ids: None,
            };
            let key_value = idempotency_value("publish-revision", &sha_hex);
            let created = cloud
                .create_circuit_revision_idempotent(
                    &token,
                    &idempotency_key(&key_value)?,
                    *circuit_id,
                    &command,
                )
                .await
                .map_err(|error| classify(&error))?
                .into_body();
            (*workspace_id, *circuit_id, created.revision_id())
        }
    };

    // 2. Upload the snapshot as a verified publication_snapshot artifact.
    let upload_key_value = idempotency_value("publish-snapshot", &sha_hex);
    let upload = cloud
        .create_artifact_upload_idempotent(
            &token,
            &idempotency_key(&upload_key_value)?,
            workspace_id,
            &CreateArtifactUploadRequest {
                kind: ClientUploadArtifactKind::PublicationSnapshot,
                file_name: Some("snapshot.rspicepub".to_owned()),
                content_type: "application/octet-stream".to_owned(),
                content_length: request.snapshot_bytes.len() as u64,
                content_sha256: sha_hex.clone(),
            },
        )
        .await
        .map_err(|error| classify(&error))?
        .into_body();
    let artifact_id = upload.artifact.id;
    if upload.upload_url.is_some() {
        // A fresh (non-replayed) handoff: stream the bytes to object storage.
        let staged = stage_snapshot(&request.snapshot_bytes, &sha_hex)?;
        let sent = cloud
            .upload_artifact_from_path(&upload, staged.path())
            .await;
        staged.remove();
        sent.map_err(|_| PublishError::Unreachable)?;
    }
    let artifact = cloud
        .complete_artifact_upload(&token, artifact_id)
        .await
        .map_err(|error| classify(&error))?
        .into_body();
    if artifact.state != rspice_cloud_client::contract::ArtifactState::Available {
        return Err(PublishError::Contract);
    }

    // 3. Create the immutable publication for the sealed revision.
    let page_key_value = idempotency_value("publish-page", &sha_hex);
    let publication: Publication = cloud
        .create_circuit_publication_idempotent(
            &token,
            &idempotency_key(&page_key_value)?,
            circuit_id,
            &CreatePublicationRequest {
                snapshot_artifact_id: artifact_id,
                revision_id: Some(revision_id),
                simulation_run_id: None,
                title: Some(request.title.clone()),
                description: if request.description.trim().is_empty() {
                    None
                } else {
                    Some(request.description.clone())
                },
                input_artifact_ids: Vec::new(),
            },
        )
        .await
        .map_err(|error| classify(&error))?
        .into_body();

    Ok(PublishReceipt {
        workspace_id: workspace_id.to_string(),
        circuit_id: circuit_id.to_string(),
        publication_id: publication.id.to_string(),
        slug: publication.slug,
        url_path: publication.url_path,
        page_status: page_status_str(publication.page_status).to_owned(),
    })
}

pub(super) fn page_status_str(
    status: rspice_cloud_client::contract::PublicationPageStatus,
) -> &'static str {
    use rspice_cloud_client::contract::PublicationPageStatus;
    match status {
        PublicationPageStatus::Preparing => "preparing",
        PublicationPageStatus::Live => "live",
        PublicationPageStatus::Failed => "failed",
    }
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut rendered = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        let _ = write!(rendered, "{byte:02x}");
    }
    rendered
}

/// The streamed upload reads from a path; stage the bytes in a private
/// scratch file named by digest so concurrent publishes never collide.
struct StagedSnapshot {
    path: std::path::PathBuf,
}

impl StagedSnapshot {
    fn path(&self) -> &std::path::Path {
        &self.path
    }

    fn remove(self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn stage_snapshot(bytes: &[u8], sha_hex: &str) -> Result<StagedSnapshot, PublishError> {
    let path = std::env::temp_dir().join(format!("rspice-publish-{sha_hex}.rspicepub"));
    std::fs::write(&path, bytes).map_err(|_| PublishError::Staging)?;
    Ok(StagedSnapshot { path })
}

/// A verifier-independent digest check would be wrong here: the server
/// re-verifies length and SHA-256 at completion, and the client crate binds
/// the streamed upload to the declared checksum.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idempotency_keys_are_digest_scoped_and_valid() {
        let sha = "a".repeat(64);
        for prefix in [
            "publish-circuit",
            "publish-revision",
            "publish-snapshot",
            "publish-page",
        ] {
            let value = idempotency_value(prefix, &sha);
            assert!(value.starts_with(prefix));
            assert!(value.len() <= 200);
            idempotency_key(&value).expect("valid key");
        }
    }

    #[test]
    fn provenance_document_records_the_exact_snapshot_digest() {
        let document = provenance_document("AFE demo", &"b".repeat(64));
        assert_eq!(document["kind"], "rspice-publication-source");
        assert_eq!(document["snapshot_sha256"], "b".repeat(64));
        assert_eq!(document["title"], "AFE demo");
    }

    #[test]
    fn hex_encoding_is_lowercase_and_exact() {
        assert_eq!(hex_lower(&[0x00, 0xff, 0x1a]), "00ff1a");
    }

    #[test]
    fn staged_snapshots_land_in_scratch_and_are_removed() {
        let staged = stage_snapshot(b"snapshot", &"c".repeat(64)).expect("staging");
        let path = staged.path().to_owned();
        assert!(path.exists());
        staged.remove();
        assert!(!path.exists());
    }
}
