use std::{collections::HashSet, error::Error, fmt};

use crate::validation::{
    decode_lower_hex_sha256, parse_timestamp_text, valid_clean_circuit_title, valid_json_object,
    valid_revision_snapshot, valid_timestamp_text,
};
use rspice_cloud_contract::{
    API_VERSION, ArtifactKind, CURRENT_SIMULATION_REQUEST_DIGEST_VERSION, CreatePublicationRequest,
    PublicPublication, Publication, PublicationArtifactCategory, PublishedArtifact,
    PublishedSimulation, SharedCircuitRevision, Uuid, is_valid_simulation_execution_manifest,
};

const PUBLICATION_SLUG_LENGTH: usize = 20;
const PUBLICATION_SLUG_ALPHABET: &[u8; 32] = b"23456789abcdefghjkmnopqrstuvwxyz";
const MAX_DESCRIPTION_CHARACTERS: usize = 2_000;
const MAX_FILE_NAME_BYTES: usize = 255;
const MAX_CONTENT_TYPE_BYTES: usize = 255;
const MAX_MANIFEST_BYTES: usize = 128 * 1024;
const MAX_ARTIFACTS_PER_CATEGORY: usize = 100;
const MAX_DOWNLOADABLE_ARTIFACTS: usize = 200;

/// Validated, borrowed public publication identifier used by `/c/<slug>`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicationSlug<'a>(&'a str);

impl<'a> PublicationSlug<'a> {
    /// Validates the exact 20-character non-ambiguous publication alphabet.
    pub fn new(value: &'a str) -> Result<Self, PublicationSlugError> {
        valid_publication_slug(value)
            .then_some(Self(value))
            .ok_or(PublicationSlugError)
    }

    /// Returns the public slug text.
    pub const fn as_str(self) -> &'a str {
        self.0
    }

    pub(crate) const fn value(self) -> &'a str {
        self.0
    }
}

/// A value did not match the exact RSpice publication-slug grammar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicationSlugError;

impl fmt::Display for PublicationSlugError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("publication slug must use the exact 20-character public grammar")
    }
}

impl Error for PublicationSlugError {}

pub(crate) fn valid_publication(publication: &Publication, circuit_id: Uuid) -> bool {
    if publication.id.is_nil()
        || circuit_id.is_nil()
        || publication.circuit_id != circuit_id
        || publication.revision_id.is_nil()
        || publication.snapshot_artifact_id.is_nil()
        || publication
            .simulation_run_id
            .is_some_and(|run_id| run_id.is_nil())
        || !valid_sorted_artifact_ids(&publication.input_artifact_ids)
        || !valid_publication_slug(&publication.slug)
        || publication.url_path != publication_url_path(&publication.slug)
        || !valid_title(&publication.title)
        || !publication
            .description
            .as_deref()
            .is_none_or(valid_description)
        || !valid_timestamp_text(&publication.published_at)
        || !valid_timestamp_text(&publication.sealed_at)
        || !publication
            .unpublished_at
            .as_deref()
            .is_none_or(valid_timestamp_text)
    {
        return false;
    }

    let Some((published_at, sealed_at)) = parse_timestamp_text(&publication.published_at)
        .zip(parse_timestamp_text(&publication.sealed_at))
    else {
        return false;
    };
    published_at <= sealed_at
        && publication
            .unpublished_at
            .as_deref()
            .and_then(parse_timestamp_text)
            .is_none_or(|unpublished_at| unpublished_at >= sealed_at)
}

pub(crate) fn valid_publication_list(publications: &[Publication], circuit_id: Uuid) -> bool {
    if circuit_id.is_nil()
        || publications
            .iter()
            .any(|publication| !valid_publication(publication, circuit_id))
        || publications
            .iter()
            .map(|publication| publication.id)
            .collect::<HashSet<_>>()
            .len()
            != publications.len()
        || publications
            .iter()
            .map(|publication| publication.slug.as_str())
            .collect::<HashSet<_>>()
            .len()
            != publications.len()
    {
        return false;
    }

    publications.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].published_at)
            .zip(parse_timestamp_text(&pair[1].published_at))
            .is_some_and(|(newer, older)| (newer, pair[0].id) > (older, pair[1].id))
    })
}

pub(crate) fn publication_matches_request(
    publication: &Publication,
    circuit_id: Uuid,
    request: &CreatePublicationRequest,
    replayed: bool,
) -> bool {
    if !valid_publication(publication, circuit_id)
        || request.snapshot_artifact_id.is_nil()
        || publication.snapshot_artifact_id != request.snapshot_artifact_id
        || request.revision_id.is_some_and(|id| id.is_nil())
        || request.simulation_run_id.is_some_and(|id| id.is_nil())
        || request
            .revision_id
            .is_some_and(|revision_id| publication.revision_id != revision_id)
        || publication.simulation_run_id != request.simulation_run_id
        || (!replayed && publication.unpublished_at.is_some())
        || !valid_requested_artifacts(&request.input_artifact_ids)
    {
        return false;
    }

    let mut requested_input_artifact_ids = request.input_artifact_ids.clone();
    requested_input_artifact_ids.sort_unstable();
    if publication.input_artifact_ids != requested_input_artifact_ids {
        return false;
    }

    if let Some(title) = request.title.as_deref() {
        let Some(title) = normalized_title(title) else {
            return false;
        };
        if publication.title != title {
            return false;
        }
    }

    let Some(description) = normalized_description(request.description.as_deref()) else {
        return false;
    };
    publication.description == description
}

pub(crate) fn valid_public_publication(
    publication: &PublicPublication,
    slug: PublicationSlug<'_>,
) -> bool {
    if publication.id.is_nil()
        || publication.circuit_id.is_nil()
        || publication.slug != slug.value()
        || publication.url_path != publication_url_path(slug.value())
        || !valid_title(&publication.title)
        || !publication
            .description
            .as_deref()
            .is_none_or(valid_description)
        || !valid_timestamp_text(&publication.published_at)
        || !valid_timestamp_text(&publication.sealed_at)
        || !valid_published_revision(&publication.revision)
        || !valid_downloadable_artifacts(
            &publication.revision,
            publication.simulation.as_ref(),
            &publication.downloadable_artifacts,
        )
    {
        return false;
    }

    let Some(((published_at, sealed_at), revision_created_at)) =
        parse_timestamp_text(&publication.published_at)
            .zip(parse_timestamp_text(&publication.sealed_at))
            .zip(parse_timestamp_text(&publication.revision.created_at))
    else {
        return false;
    };
    revision_created_at <= published_at
        && published_at <= sealed_at
        && publication
            .simulation
            .as_ref()
            .is_none_or(valid_published_simulation)
}

fn valid_publication_slug(value: &str) -> bool {
    value.len() == PUBLICATION_SLUG_LENGTH
        && value
            .bytes()
            .all(|byte| PUBLICATION_SLUG_ALPHABET.contains(&byte))
}

fn publication_url_path(slug: &str) -> String {
    format!("/c/{slug}")
}

pub(crate) fn publication_api_path(slug: &str) -> String {
    format!("/api/{API_VERSION}/publications/{slug}")
}

fn normalized_title(value: &str) -> Option<&str> {
    let value = value.trim();
    valid_clean_circuit_title(value).then_some(value)
}

fn valid_title(value: &str) -> bool {
    normalized_title(value) == Some(value)
}

fn normalized_description(value: Option<&str>) -> Option<Option<String>> {
    let Some(value) = value else {
        return Some(None);
    };
    let value = value.replace("\r\n", "\n").replace('\r', "\n");
    let value = value.trim();
    if value.is_empty() {
        return Some(None);
    }
    (value.chars().count() <= MAX_DESCRIPTION_CHARACTERS
        && !value
            .chars()
            .any(|character| character.is_control() && character != '\n'))
    .then(|| Some(value.to_owned()))
}

fn valid_description(value: &str) -> bool {
    normalized_description(Some(value))
        .is_some_and(|normalized| normalized.as_deref() == Some(value))
}

fn valid_requested_artifacts(ids: &[Uuid]) -> bool {
    ids.len() <= MAX_ARTIFACTS_PER_CATEGORY && ids.iter().all(|id| !id.is_nil()) && unique_ids(ids)
}

fn valid_sorted_artifact_ids(ids: &[Uuid]) -> bool {
    ids.len() <= MAX_ARTIFACTS_PER_CATEGORY
        && ids.iter().all(|id| !id.is_nil())
        && ids.windows(2).all(|pair| pair[0] < pair[1])
}

fn valid_published_revision(revision: &SharedCircuitRevision) -> bool {
    valid_revision_snapshot(
        revision.id,
        revision.schema_version,
        revision.content_digest_version,
        &revision.document,
        &revision.artifact_ids,
        &revision.content_sha256,
        &revision.created_at,
    )
}

fn valid_published_simulation(simulation: &PublishedSimulation) -> bool {
    !simulation.id.is_nil()
        && simulation.request_digest_version == CURRENT_SIMULATION_REQUEST_DIGEST_VERSION
        && decode_lower_hex_sha256(&simulation.request_sha256).is_some()
        && is_valid_simulation_execution_manifest(&simulation.execution_manifest)
        && valid_json_object(&simulation.result_manifest, MAX_MANIFEST_BYTES)
        && simulation.artifact_ids.len() <= MAX_ARTIFACTS_PER_CATEGORY
        && simulation.artifact_ids.iter().all(|id| !id.is_nil())
        && unique_ids(&simulation.artifact_ids)
}

fn valid_downloadable_artifacts(
    revision: &SharedCircuitRevision,
    simulation: Option<&PublishedSimulation>,
    artifacts: &[PublishedArtifact],
) -> bool {
    if artifacts.len() > MAX_DOWNLOADABLE_ARTIFACTS
        || artifacts.iter().any(|artifact| !valid_artifact(artifact))
        || !unique_ids(
            &artifacts
                .iter()
                .map(|artifact| artifact.id)
                .collect::<Vec<_>>(),
        )
    {
        return false;
    }

    let revision_ids = revision
        .artifact_ids
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    if artifacts.iter().any(|artifact| {
        artifact.category == PublicationArtifactCategory::Input
            && !revision_ids.contains(&artifact.id)
    }) {
        return false;
    }

    let result_ids = artifacts
        .iter()
        .filter(|artifact| artifact.category == PublicationArtifactCategory::Result)
        .map(|artifact| artifact.id)
        .collect::<Vec<_>>();
    match simulation {
        Some(simulation) => result_ids == simulation.artifact_ids,
        None => result_ids.is_empty(),
    }
}

fn valid_artifact(artifact: &PublishedArtifact) -> bool {
    if artifact.id.is_nil()
        || artifact.size_bytes == 0
        || decode_lower_hex_sha256(&artifact.sha256).is_none()
        || !valid_content_type(&artifact.content_type)
        || !artifact.file_name.as_deref().is_none_or(valid_file_name)
    {
        return false;
    }

    matches!(
        (artifact.category, artifact.kind),
        (
            PublicationArtifactCategory::Input,
            ArtifactKind::CircuitAttachment | ArtifactKind::ModelLibrary
        ) | (
            PublicationArtifactCategory::Result,
            ArtifactKind::SimulationResult
        )
    )
}

fn valid_file_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_FILE_NAME_BYTES
        && value.trim() == value
        && !value.chars().any(char::is_control)
        && !value.contains(['/', '\\'])
}

fn valid_content_type(value: &str) -> bool {
    let Some((kind, subtype)) = value.split_once('/') else {
        return false;
    };
    !kind.is_empty()
        && !subtype.is_empty()
        && value.len() <= MAX_CONTENT_TYPE_BYTES
        && value.matches('/').count() == 1
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(
                    byte,
                    b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-' | b'/'
                )
        })
}

fn unique_ids(ids: &[Uuid]) -> bool {
    ids.iter().copied().collect::<HashSet<_>>().len() == ids.len()
}

#[cfg(test)]
mod tests {
    use rspice_cloud_contract::PublicationPageStatus;
    use serde_json::json;

    use super::*;

    fn execution_manifest() -> serde_json::Value {
        json!({
            "protocol_version": 3,
            "engine_protocol_version": 3,
            "attempt": 1,
            "worker_class": "shared",
            "engine": {
                "name": "rspice-engine",
                "build": "2026.07.20",
                "runtime_mode": "self_contained",
                "adapter_sha256": "ab".repeat(32),
                "solver_sha256": null,
                "model_library_sha256": null,
            },
            "revision": {"content_digest_version": 2},
            "request": {"digest_version": 1},
            "artifacts": [],
        })
    }

    const SLUG: &str = "23456789abcdefghjkmn";

    fn publication() -> Publication {
        Publication {
            id: Uuid::from_u128(1),
            slug: SLUG.to_owned(),
            url_path: format!("/c/{SLUG}"),
            circuit_id: Uuid::from_u128(2),
            revision_id: Uuid::from_u128(3),
            simulation_run_id: Some(Uuid::from_u128(4)),
            snapshot_artifact_id: Uuid::from_u128(6),
            input_artifact_ids: vec![Uuid::from_u128(5)],
            title: "Precision amplifier".to_owned(),
            description: Some("Verified result".to_owned()),
            page_status: PublicationPageStatus::Preparing,
            published_at: "2026-07-19T00:00:00Z".to_owned(),
            sealed_at: "2026-07-19T00:00:01Z".to_owned(),
            unpublished_at: None,
        }
    }

    #[test]
    fn slugs_and_management_responses_are_exact_and_request_bound() {
        assert!(PublicationSlug::new(SLUG).is_ok());
        assert!(PublicationSlug::new("iiiiiiiiiiiiiiiiiiii").is_err());
        let request = CreatePublicationRequest {
            snapshot_artifact_id: Uuid::from_u128(6),
            revision_id: Some(Uuid::from_u128(3)),
            simulation_run_id: Some(Uuid::from_u128(4)),
            title: Some("  Precision amplifier  ".to_owned()),
            description: Some("  Verified result  ".to_owned()),
            input_artifact_ids: vec![Uuid::from_u128(5)],
        };
        let mut response = publication();
        assert!(publication_matches_request(
            &response,
            Uuid::from_u128(2),
            &request,
            false,
        ));

        response.url_path.push_str("?secret=unsafe");
        assert!(!publication_matches_request(
            &response,
            Uuid::from_u128(2),
            &request,
            false,
        ));

        response = publication();
        response.unpublished_at = Some("2026-07-19T00:01:00Z".to_owned());
        assert!(!publication_matches_request(
            &response,
            Uuid::from_u128(2),
            &request,
            false,
        ));
        assert!(publication_matches_request(
            &response,
            Uuid::from_u128(2),
            &request,
            true,
        ));

        response = publication();
        response.input_artifact_ids = vec![Uuid::from_u128(6)];
        assert!(!publication_matches_request(
            &response,
            Uuid::from_u128(2),
            &request,
            false,
        ));

        let mut older = publication();
        older.id = Uuid::from_u128(0x10);
        older.slug = "zzzzzzzzzzzzzzzzzzzz".to_owned();
        older.url_path = format!("/c/{}", older.slug);
        older.published_at = "2026-07-18T00:00:00Z".to_owned();
        older.sealed_at = "2026-07-18T00:00:01Z".to_owned();
        let newer = publication();
        assert!(valid_publication_list(
            &[newer.clone(), older.clone()],
            Uuid::from_u128(2)
        ));
        assert!(!valid_publication_list(
            &[older, newer.clone()],
            Uuid::from_u128(2)
        ));
        assert!(!valid_publication_list(
            &[newer.clone(), newer],
            Uuid::from_u128(2)
        ));
    }

    #[test]
    fn public_snapshots_bind_revision_simulation_and_downloadable_artifacts() {
        let result_id = Uuid::from_u128(7);
        let snapshot = PublicPublication {
            id: Uuid::from_u128(1),
            slug: SLUG.to_owned(),
            url_path: format!("/c/{SLUG}"),
            title: "Precision amplifier".to_owned(),
            description: None,
            circuit_id: Uuid::from_u128(2),
            revision: SharedCircuitRevision {
                id: Uuid::from_u128(3),
                schema_version: 1,
                content_digest_version: 2,
                document: json!({"components": []}),
                artifact_ids: vec![Uuid::from_u128(6)],
                content_sha256: "0a".repeat(32),
                created_at: "2026-07-19T00:00:00Z".to_owned(),
            },
            simulation: Some(PublishedSimulation {
                id: Uuid::from_u128(4),
                request_digest_version: CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
                request_sha256: "0b".repeat(32),
                execution_manifest: execution_manifest(),
                result_manifest: json!({"format": "rspice-result-v1"}),
                artifact_ids: vec![result_id],
            }),
            downloadable_artifacts: vec![PublishedArtifact {
                id: result_id,
                category: PublicationArtifactCategory::Result,
                kind: ArtifactKind::SimulationResult,
                file_name: Some("result.raw".to_owned()),
                content_type: "application/octet-stream".to_owned(),
                size_bytes: 128,
                sha256: "0c".repeat(32),
            }],
            published_at: "2026-07-19T00:00:00Z".to_owned(),
            sealed_at: "2026-07-19T00:00:01Z".to_owned(),
        };
        let slug = PublicationSlug::new(SLUG).expect("publication slug");
        assert!(valid_public_publication(&snapshot, slug));

        let mut mismatched = snapshot.clone();
        mismatched
            .simulation
            .as_mut()
            .expect("simulation")
            .artifact_ids = vec![Uuid::from_u128(8)];
        assert!(!valid_public_publication(&mismatched, slug));

        let impossible_chronology = PublicPublication {
            published_at: "2026-07-18T23:59:59Z".to_owned(),
            ..snapshot.clone()
        };
        assert!(!valid_public_publication(&impossible_chronology, slug));

        let impossible_seal = PublicPublication {
            sealed_at: "2026-07-18T23:59:59Z".to_owned(),
            ..snapshot
        };
        assert!(!valid_public_publication(&impossible_seal, slug));
    }
}
