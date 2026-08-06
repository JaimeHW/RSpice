use std::collections::HashSet;

use rspice_cloud_contract::{
    CreateWorkspaceRequest, UpdateWorkspaceRequest, Uuid, Workspace, WorkspaceRole,
};

use crate::validation::parse_timestamp_text;

const MAX_WORKSPACE_NAME_CHARACTERS: usize = 120;

pub(crate) fn valid_workspace(workspace: &Workspace, expected_id: Option<Uuid>) -> bool {
    if workspace.id.is_nil()
        || expected_id.is_some_and(|id| workspace.id != id)
        || !valid_workspace_slug(&workspace.slug)
        || !valid_workspace_name(&workspace.name)
        || workspace.row_version < 0
    {
        return false;
    }

    let Some(created_at) = parse_timestamp_text(&workspace.created_at) else {
        return false;
    };
    parse_timestamp_text(&workspace.updated_at).is_some_and(|updated_at| updated_at >= created_at)
}

pub(crate) fn valid_workspace_list(workspaces: &[Workspace]) -> bool {
    if workspaces
        .iter()
        .any(|workspace| !valid_workspace(workspace, None))
        || workspaces
            .iter()
            .map(|workspace| workspace.id)
            .collect::<HashSet<_>>()
            .len()
            != workspaces.len()
    {
        return false;
    }

    workspaces.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].created_at)
            .zip(parse_timestamp_text(&pair[1].created_at))
            .is_some_and(|(newer, older)| (newer, pair[0].id) > (older, pair[1].id))
    })
}

pub(crate) fn workspace_matches_create_request(
    workspace: &Workspace,
    request: &CreateWorkspaceRequest,
    replayed: bool,
) -> bool {
    let normalized_name = request.name.trim();
    if !valid_workspace(workspace, None)
        || !valid_workspace_slug(&request.slug)
        || !valid_workspace_name(normalized_name)
        || workspace.slug != request.slug
    {
        return false;
    }
    replayed
        || (workspace.name == normalized_name
            && workspace.role == WorkspaceRole::Owner
            && workspace.row_version == 0
            && workspace.updated_at == workspace.created_at)
}

pub(crate) fn workspace_matches_update_request(
    workspace: &Workspace,
    workspace_id: Uuid,
    request: &UpdateWorkspaceRequest,
) -> bool {
    let normalized_name = request.name.trim();
    valid_workspace(workspace, Some(workspace_id))
        && request.expected_row_version >= 0
        && valid_workspace_name(normalized_name)
        && workspace.name == normalized_name
        && (workspace.row_version == request.expected_row_version
            || request
                .expected_row_version
                .checked_add(1)
                .is_some_and(|version| workspace.row_version == version))
}

fn valid_workspace_slug(value: &str) -> bool {
    let bytes = value.as_bytes();
    (1..=63).contains(&bytes.len())
        && matches!(bytes.first(), Some(b'a'..=b'z' | b'0'..=b'9'))
        && matches!(bytes.last(), Some(b'a'..=b'z' | b'0'..=b'9'))
        && bytes
            .iter()
            .all(|byte| matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'-'))
}

fn valid_workspace_name(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && value.chars().count() <= MAX_WORKSPACE_NAME_CHARACTERS
        && !value.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace(id: u128, row_version: i64, created_at: &str) -> Workspace {
        Workspace {
            id: Uuid::from_u128(id),
            slug: format!("workspace-{id}"),
            name: format!("Workspace {id}"),
            role: WorkspaceRole::Owner,
            row_version,
            created_at: created_at.to_owned(),
            updated_at: created_at.to_owned(),
        }
    }

    #[test]
    fn workspace_pages_are_canonical_unique_and_newest_first() {
        let newer = workspace(1, 0, "2026-07-19T00:00:00Z");
        let older = workspace(2, 3, "2026-07-18T00:00:00Z");
        assert!(valid_workspace_list(&[newer.clone(), older.clone()]));
        assert!(!valid_workspace_list(&[older, newer.clone()]));

        let mut invalid = newer;
        invalid.slug = "trailing-".to_owned();
        assert!(!valid_workspace(&invalid, None));
    }

    #[test]
    fn workspace_mutations_bind_normalized_commands_and_versions() {
        let request = CreateWorkspaceRequest {
            slug: "precision-lab".to_owned(),
            name: "  Precision Lab  ".to_owned(),
        };
        let mut created = workspace(3, 0, "2026-07-19T00:00:00Z");
        created.slug = "precision-lab".to_owned();
        created.name = "Precision Lab".to_owned();
        assert!(workspace_matches_create_request(&created, &request, false));

        let update = UpdateWorkspaceRequest {
            expected_row_version: 0,
            name: "Production Lab".to_owned(),
        };
        created.name = "Production Lab".to_owned();
        created.row_version = 1;
        created.updated_at = "2026-07-19T00:00:01Z".to_owned();
        assert!(workspace_matches_update_request(
            &created,
            Uuid::from_u128(3),
            &update,
        ));
    }
}
