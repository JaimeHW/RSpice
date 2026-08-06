use std::collections::HashSet;

use rspice_cloud_contract::{
    API_VERSION, Circuit, CircuitRevision, CircuitVisibility, CreateCircuitRequest,
    CreateCircuitRevisionRequest, UpdateCircuitRequest, Uuid,
};

use crate::validation::{
    parse_timestamp_text, valid_clean_circuit_title, valid_revision_input, valid_revision_snapshot,
};

/// Result of creating or exactly replaying an immutable circuit revision.
///
/// The circuit is the server's current mutable projection. `revision_id`
/// identifies the immutable revision created by the original command even
/// when a later exact replay observes a newer circuit head.
pub struct CreatedCircuitRevision {
    circuit: Circuit,
    revision_id: Uuid,
}

impl CreatedCircuitRevision {
    pub(crate) const fn new(circuit: Circuit, revision_id: Uuid) -> Self {
        Self {
            circuit,
            revision_id,
        }
    }

    /// Returns the circuit's current server projection.
    pub fn circuit(&self) -> &Circuit {
        &self.circuit
    }

    /// Returns the immutable revision created by the original command.
    pub const fn revision_id(&self) -> Uuid {
        self.revision_id
    }

    /// Consumes the result and returns the circuit's current projection.
    pub fn into_circuit(self) -> Circuit {
        self.circuit
    }
}

pub(crate) fn revision_id_from_location(location: &str, circuit_id: Uuid) -> Option<Uuid> {
    let prefix = format!("/api/{API_VERSION}/circuits/{circuit_id}/revisions/");
    let raw_revision_id = location.strip_prefix(&prefix)?;
    let revision_id = Uuid::parse_str(raw_revision_id).ok()?;
    (revision_id.to_string() == raw_revision_id).then_some(revision_id)
}

pub(crate) fn valid_circuit(
    circuit: &Circuit,
    expected_id: Option<Uuid>,
    expected_workspace_id: Option<Uuid>,
) -> bool {
    if circuit.id.is_nil()
        || circuit.workspace_id.is_nil()
        || expected_id.is_some_and(|expected| circuit.id != expected)
        || expected_workspace_id.is_some_and(|expected| circuit.workspace_id != expected)
        || !valid_clean_circuit_title(&circuit.title)
        || circuit.head_revision_id.is_none_or(|id| id.is_nil())
        || circuit.row_version < 1
    {
        return false;
    }

    let Some(created_at) = parse_timestamp_text(&circuit.created_at) else {
        return false;
    };
    let Some(updated_at) = parse_timestamp_text(&circuit.updated_at) else {
        return false;
    };
    if updated_at < created_at {
        return false;
    }
    circuit.archived_at.as_deref().is_none_or(|value| {
        parse_timestamp_text(value)
            .is_some_and(|archived_at| archived_at >= created_at && archived_at <= updated_at)
    })
}

pub(crate) fn valid_circuit_list(circuits: &[Circuit], workspace_id: Uuid) -> bool {
    if circuits.iter().any(|circuit| {
        !valid_circuit(circuit, None, Some(workspace_id)) || circuit.archived_at.is_some()
    }) || circuits
        .iter()
        .map(|circuit| circuit.id)
        .collect::<HashSet<_>>()
        .len()
        != circuits.len()
    {
        return false;
    }

    circuits.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].updated_at)
            .zip(parse_timestamp_text(&pair[1].updated_at))
            .is_some_and(|(newer, older)| (newer, pair[0].id) > (older, pair[1].id))
    })
}

pub(crate) fn circuit_matches_create_request(
    circuit: &Circuit,
    workspace_id: Uuid,
    request: &CreateCircuitRequest,
    replayed: bool,
) -> bool {
    if !valid_circuit(circuit, None, Some(workspace_id))
        || !valid_revision_input(
            request.schema_version,
            &request.document,
            request.artifact_ids.as_deref(),
        )
    {
        return false;
    }
    if replayed {
        return true;
    }

    let title = request.title.trim();
    valid_clean_circuit_title(title)
        && circuit.title == title
        && circuit.visibility == request.visibility.unwrap_or(CircuitVisibility::Private)
        && circuit.head_revision_id.is_some()
        && circuit.row_version == 1
        && circuit.archived_at.is_none()
}

pub(crate) fn circuit_matches_update_request(
    circuit: &Circuit,
    circuit_id: Uuid,
    request: &UpdateCircuitRequest,
) -> bool {
    let Some(expected_row_version) = request.expected_row_version.checked_add(1) else {
        return false;
    };
    if !valid_circuit(circuit, Some(circuit_id), None)
        || request.expected_row_version < 0
        || (request.title.is_none() && request.visibility.is_none() && request.archived.is_none())
        || circuit.row_version != expected_row_version
        || request
            .visibility
            .is_some_and(|visibility| circuit.visibility != visibility)
        || request
            .archived
            .is_some_and(|archived| archived != circuit.archived_at.is_some())
    {
        return false;
    }

    request.title.as_deref().is_none_or(|title| {
        let title = title.trim();
        valid_clean_circuit_title(title) && circuit.title == title
    })
}

pub(crate) fn valid_circuit_revision(
    revision: &CircuitRevision,
    expected_id: Option<Uuid>,
) -> bool {
    expected_id.is_none_or(|expected| revision.id == expected)
        && revision
            .parent_revision_id
            .is_none_or(|parent_id| !parent_id.is_nil() && parent_id != revision.id)
        && valid_revision_snapshot(
            revision.id,
            revision.schema_version,
            revision.content_digest_version,
            &revision.document,
            &revision.artifact_ids,
            &revision.content_sha256,
            &revision.created_at,
        )
}

pub(crate) fn valid_circuit_revision_list(revisions: &[CircuitRevision]) -> bool {
    if revisions
        .iter()
        .any(|revision| !valid_circuit_revision(revision, None))
        || revisions
            .iter()
            .map(|revision| revision.id)
            .collect::<HashSet<_>>()
            .len()
            != revisions.len()
    {
        return false;
    }

    revisions.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].created_at)
            .zip(parse_timestamp_text(&pair[1].created_at))
            .is_some_and(|(newer, older)| (newer, pair[0].id) > (older, pair[1].id))
    })
}

pub(crate) fn circuit_matches_revision_request(
    circuit: &Circuit,
    circuit_id: Uuid,
    revision_id: Uuid,
    request: &CreateCircuitRevisionRequest,
    replayed: bool,
) -> bool {
    let Some(minimum_row_version) = request.expected_row_version.checked_add(1) else {
        return false;
    };
    valid_circuit(circuit, Some(circuit_id), None)
        && request.expected_row_version >= 0
        && request
            .parent_revision_id
            .is_none_or(|parent_id| !parent_id.is_nil() && parent_id != revision_id)
        && !revision_id.is_nil()
        && valid_revision_input(
            request.schema_version,
            &request.document,
            request.artifact_ids.as_deref(),
        )
        && if replayed {
            circuit.row_version >= minimum_row_version
        } else {
            circuit.row_version == minimum_row_version
                && circuit.head_revision_id == Some(revision_id)
        }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn circuit(id: u128, row_version: i64, updated_at: &str) -> Circuit {
        Circuit {
            id: Uuid::from_u128(id),
            workspace_id: Uuid::from_u128(10),
            title: "Precision amplifier".to_owned(),
            visibility: CircuitVisibility::Private,
            head_revision_id: Some(Uuid::from_u128(20)),
            row_version,
            archived_at: None,
            created_at: "2026-07-19T00:00:00Z".to_owned(),
            updated_at: updated_at.to_owned(),
        }
    }

    fn revision(id: u128, created_at: &str) -> CircuitRevision {
        CircuitRevision {
            id: Uuid::from_u128(id),
            parent_revision_id: None,
            schema_version: 1,
            content_digest_version: 2,
            document: json!({"components": []}),
            artifact_ids: vec![Uuid::from_u128(30)],
            content_sha256: "0a".repeat(32),
            created_at: created_at.to_owned(),
        }
    }

    #[test]
    fn revision_locations_are_exact_and_circuit_bound() {
        let circuit_id = Uuid::from_u128(1);
        let revision_id = Uuid::from_u128(2);
        let location = format!("/api/v1/circuits/{circuit_id}/revisions/{revision_id}");
        assert_eq!(
            revision_id_from_location(&location, circuit_id),
            Some(revision_id)
        );
        assert!(revision_id_from_location(&location, Uuid::from_u128(3)).is_none());
        assert!(revision_id_from_location(&format!("{location}/extra"), circuit_id).is_none());
        assert!(
            revision_id_from_location(&format!("{location}?token=secret"), circuit_id).is_none()
        );
    }

    #[test]
    fn circuit_and_revision_pages_are_clean_scoped_unique_and_newest_first() {
        let newer = circuit(2, 4, "2026-07-19T00:02:00Z");
        let older = circuit(1, 3, "2026-07-19T00:01:00Z");
        assert!(valid_circuit(
            &newer,
            Some(newer.id),
            Some(newer.workspace_id)
        ));
        assert!(valid_circuit_list(
            &[newer.clone(), older.clone()],
            newer.workspace_id
        ));
        assert!(!valid_circuit_list(
            &[older, newer.clone()],
            newer.workspace_id
        ));

        let mut invalid = newer;
        invalid.updated_at = "2026-07-18T23:59:59Z".to_owned();
        assert!(!valid_circuit(&invalid, None, None));
        invalid.updated_at = "2026-07-19T00:02:00Z".to_owned();
        invalid.head_revision_id = None;
        assert!(!valid_circuit(&invalid, None, None));
        invalid.head_revision_id = Some(Uuid::from_u128(20));
        invalid.row_version = 0;
        assert!(!valid_circuit(&invalid, None, None));

        let newer = revision(2, "2026-07-19T00:02:00Z");
        let older = revision(1, "2026-07-19T00:01:00Z");
        assert!(valid_circuit_revision_list(&[newer.clone(), older.clone()]));
        assert!(!valid_circuit_revision_list(&[older, newer.clone()]));

        let mut invalid = newer;
        invalid.parent_revision_id = Some(invalid.id);
        assert!(!valid_circuit_revision(&invalid, None));
    }

    #[test]
    fn mutation_successes_bind_to_normalized_commands_and_row_versions() {
        let workspace_id = Uuid::from_u128(10);
        let artifact_id = Uuid::from_u128(30);
        let create = CreateCircuitRequest {
            title: "  Precision amplifier  ".to_owned(),
            visibility: None,
            schema_version: 1,
            document: json!({"components": []}),
            artifact_ids: Some(vec![artifact_id]),
        };
        let created = circuit(1, 1, "2026-07-19T00:01:00Z");
        assert!(circuit_matches_create_request(
            &created,
            workspace_id,
            &create,
            false
        ));

        let update = UpdateCircuitRequest {
            expected_row_version: 1,
            title: Some("  Updated amplifier  ".to_owned()),
            visibility: Some(CircuitVisibility::Public),
            archived: Some(false),
        };
        let mut updated = circuit(1, 2, "2026-07-19T00:02:00Z");
        updated.title = "Updated amplifier".to_owned();
        updated.visibility = CircuitVisibility::Public;
        assert!(circuit_matches_update_request(
            &updated, updated.id, &update
        ));
        updated.row_version = 1;
        assert!(!circuit_matches_update_request(
            &updated, updated.id, &update
        ));

        let revision_id = Uuid::from_u128(21);
        let revision = CreateCircuitRevisionRequest {
            expected_row_version: 2,
            parent_revision_id: Some(Uuid::from_u128(20)),
            schema_version: 1,
            document: json!({"components": []}),
            artifact_ids: Some(vec![artifact_id]),
        };
        let mut successor = circuit(1, 3, "2026-07-19T00:03:00Z");
        successor.head_revision_id = Some(revision_id);
        assert!(circuit_matches_revision_request(
            &successor,
            successor.id,
            revision_id,
            &revision,
            false,
        ));
        successor.head_revision_id = Some(Uuid::from_u128(22));
        assert!(!circuit_matches_revision_request(
            &successor,
            successor.id,
            revision_id,
            &revision,
            false,
        ));
    }
}
