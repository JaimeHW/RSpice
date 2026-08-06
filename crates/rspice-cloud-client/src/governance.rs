use std::collections::HashSet;

use rspice_cloud_contract::{
    API_VERSION, AuditEvent, CreatedWorkspaceInvitation, Uuid, WorkspaceInvitation,
    WorkspaceMember, WorkspaceRole,
};
use serde_json::Value;
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{CreateWorkspaceInvitation, clock::current_time_utc, validation::parse_timestamp_text};

const MAX_EMAIL_BYTES: usize = 320;
const MAX_DISPLAY_NAME_BYTES: usize = 256;
const MAX_AUDIT_ACTION_BYTES: usize = 120;
const MAX_AUDIT_TARGET_TYPE_BYTES: usize = 80;
const MAX_AUDIT_METADATA_BYTES: usize = 64 * 1024;
const MAX_INVITATION_LIFETIME: Duration = Duration::days(30);
const MINIMUM_INVITATION_HANDOFF_WINDOW: Duration = Duration::seconds(5);

struct InvitationFields<'a> {
    id: Uuid,
    workspace_id: Uuid,
    email: &'a str,
    expires_at: &'a str,
    accepted_at: Option<&'a str>,
    revoked_at: Option<&'a str>,
    created_at: &'a str,
}

pub(crate) fn valid_audit_events(events: &[AuditEvent], workspace_id: Uuid) -> bool {
    !workspace_id.is_nil()
        && events
            .iter()
            .all(|event| valid_audit_event(event, workspace_id))
        && unique_ids(events.iter().map(|event| event.id))
        && strictly_descending(
            events
                .iter()
                .map(|event| (event.occurred_at.as_str(), event.id)),
        )
}

pub(crate) fn valid_workspace_members(members: &[WorkspaceMember], workspace_id: Uuid) -> bool {
    !workspace_id.is_nil()
        && members
            .iter()
            .all(|member| valid_workspace_member(member, Some(workspace_id), None, None))
        && unique_ids(members.iter().map(|member| member.principal_id))
        && strictly_descending(
            members
                .iter()
                .map(|member| (member.joined_at.as_str(), member.principal_id)),
        )
}

pub(crate) fn valid_workspace_member(
    member: &WorkspaceMember,
    workspace_id: Option<Uuid>,
    principal_id: Option<Uuid>,
    role: Option<WorkspaceRole>,
) -> bool {
    !member.workspace_id.is_nil()
        && !member.principal_id.is_nil()
        && workspace_id.is_none_or(|id| member.workspace_id == id)
        && principal_id.is_none_or(|id| member.principal_id == id)
        && role.is_none_or(|expected| member.role == expected)
        && member.email.as_deref().is_none_or(valid_profile_email)
        && member
            .display_name
            .as_deref()
            .is_none_or(|value| valid_profile_text(value, MAX_DISPLAY_NAME_BYTES))
        && parse_timestamp_text(&member.joined_at).is_some()
}

pub(crate) fn valid_workspace_invitations(
    invitations: &[WorkspaceInvitation],
    workspace_id: Uuid,
) -> bool {
    !workspace_id.is_nil()
        && invitations
            .iter()
            .all(|invitation| valid_workspace_invitation(invitation, workspace_id))
        && unique_ids(invitations.iter().map(|invitation| invitation.id))
        && strictly_descending(
            invitations
                .iter()
                .map(|invitation| (invitation.created_at.as_str(), invitation.id)),
        )
}

pub(crate) fn created_invitation_matches_request(
    invitation: &CreatedWorkspaceInvitation,
    workspace_id: Uuid,
    request: &CreateWorkspaceInvitation<'_>,
    replayed: bool,
) -> bool {
    if invitation.token.is_some()
        || !valid_invitation_fields(
            InvitationFields {
                id: invitation.id,
                workspace_id: invitation.workspace_id,
                email: &invitation.email,
                expires_at: &invitation.expires_at,
                accepted_at: invitation.accepted_at.as_deref(),
                revoked_at: invitation.revoked_at.as_deref(),
                created_at: &invitation.created_at,
            },
            workspace_id,
        )
        || normalize_email(request.email()).as_deref() != Some(invitation.email.as_str())
        || invitation.role != request.role()
        || (!replayed && (invitation.accepted_at.is_some() || invitation.revoked_at.is_some()))
    {
        return false;
    }

    request.expires_at().is_none_or(|requested| {
        normalize_requested_timestamp(requested).is_some_and(|requested| {
            parse_timestamp_text(&invitation.expires_at) == Some(requested)
        })
    })
}

pub(crate) fn created_invitation_handoff_is_safe(invitation: &CreatedWorkspaceInvitation) -> bool {
    if invitation.accepted_at.is_some() || invitation.revoked_at.is_some() {
        return true;
    }
    current_time_utc().is_some_and(|now| created_invitation_handoff_is_fresh_at(invitation, now))
}

fn created_invitation_handoff_is_fresh_at(
    invitation: &CreatedWorkspaceInvitation,
    now: OffsetDateTime,
) -> bool {
    parse_timestamp_text(&invitation.expires_at).is_some_and(|expires_at| {
        expires_at >= now.saturating_add(MINIMUM_INVITATION_HANDOFF_WINDOW)
    })
}

pub(crate) fn workspace_invitation_api_path(workspace_id: Uuid, invitation_id: Uuid) -> String {
    format!("/api/{API_VERSION}/workspaces/{workspace_id}/invitations/{invitation_id}")
}

fn valid_audit_event(event: &AuditEvent, workspace_id: Uuid) -> bool {
    !event.id.is_nil()
        && event.workspace_id == workspace_id
        && !workspace_id.is_nil()
        && event
            .actor_principal_id
            .is_none_or(|principal_id| !principal_id.is_nil())
        && event.target_id.is_none_or(|target_id| !target_id.is_nil())
        && valid_audit_action(&event.action)
        && valid_identifier(&event.target_type, MAX_AUDIT_TARGET_TYPE_BYTES)
        && valid_json_object(&event.metadata, MAX_AUDIT_METADATA_BYTES)
        && parse_timestamp_text(&event.occurred_at).is_some()
}

fn valid_workspace_invitation(invitation: &WorkspaceInvitation, workspace_id: Uuid) -> bool {
    valid_invitation_fields(
        InvitationFields {
            id: invitation.id,
            workspace_id: invitation.workspace_id,
            email: &invitation.email,
            expires_at: &invitation.expires_at,
            accepted_at: invitation.accepted_at.as_deref(),
            revoked_at: invitation.revoked_at.as_deref(),
            created_at: &invitation.created_at,
        },
        workspace_id,
    )
}

fn valid_invitation_fields(invitation: InvitationFields<'_>, expected_workspace_id: Uuid) -> bool {
    if invitation.id.is_nil()
        || expected_workspace_id.is_nil()
        || invitation.workspace_id != expected_workspace_id
        || !valid_normalized_email(invitation.email)
        || invitation.accepted_at.is_some() && invitation.revoked_at.is_some()
    {
        return false;
    }

    let Some(created_at) = parse_timestamp_text(invitation.created_at) else {
        return false;
    };
    let Some(expires_at) = parse_timestamp_text(invitation.expires_at) else {
        return false;
    };
    if invitation
        .accepted_at
        .is_some_and(|value| parse_timestamp_text(value).is_none())
        || invitation
            .revoked_at
            .is_some_and(|value| parse_timestamp_text(value).is_none())
    {
        return false;
    }
    let accepted_at = invitation.accepted_at.and_then(parse_timestamp_text);
    let revoked_at = invitation.revoked_at.and_then(parse_timestamp_text);

    created_at < expires_at
        && expires_at <= created_at + MAX_INVITATION_LIFETIME
        && accepted_at.is_none_or(|accepted| accepted >= created_at && accepted < expires_at)
        && revoked_at.is_none_or(|revoked| revoked >= created_at)
}

fn valid_profile_text(value: &str, maximum_bytes: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum_bytes
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

fn valid_profile_email(value: &str) -> bool {
    valid_profile_text(value, MAX_EMAIL_BYTES)
        && !value.chars().any(char::is_whitespace)
        && value.matches('@').count() == 1
        && value
            .split_once('@')
            .is_some_and(|(local, domain)| !local.is_empty() && !domain.is_empty())
}

fn normalize_email(value: &str) -> Option<String> {
    let value = value.trim();
    if value.len() < 3
        || value.len() > MAX_EMAIL_BYTES
        || !value.is_ascii()
        || value.chars().any(char::is_control)
        || value.chars().any(char::is_whitespace)
    {
        return None;
    }
    let (local, domain) = value.rsplit_once('@')?;
    if local.is_empty()
        || domain.len() < 3
        || domain.starts_with('.')
        || domain.ends_with('.')
        || !domain.contains('.')
        || value.matches('@').count() != 1
    {
        return None;
    }
    Some(value.to_ascii_lowercase())
}

fn valid_normalized_email(value: &str) -> bool {
    normalize_email(value).as_deref() == Some(value)
}

fn normalize_requested_timestamp(value: &str) -> Option<OffsetDateTime> {
    if value.is_empty() || value.len() > 64 {
        return None;
    }
    let timestamp = OffsetDateTime::parse(value, &Rfc3339).ok()?;
    let microseconds = timestamp.unix_timestamp_nanos().div_euclid(1_000) * 1_000;
    OffsetDateTime::from_unix_timestamp_nanos(microseconds).ok()
}

fn valid_audit_action(value: &str) -> bool {
    if value.is_empty() || value.len() > MAX_AUDIT_ACTION_BYTES {
        return false;
    }
    let mut segments = value.split('.');
    let Some(first) = segments.next() else {
        return false;
    };
    let Some(second) = segments.next() else {
        return false;
    };
    valid_identifier(first, MAX_AUDIT_ACTION_BYTES)
        && valid_identifier(second, MAX_AUDIT_ACTION_BYTES)
        && segments.all(|segment| valid_identifier(segment, MAX_AUDIT_ACTION_BYTES))
}

fn valid_identifier(value: &str, maximum_bytes: usize) -> bool {
    let mut bytes = value.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        && value.len() <= maximum_bytes
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn valid_json_object(value: &Value, maximum_bytes: usize) -> bool {
    value.is_object() && serde_json::to_vec(value).is_ok_and(|bytes| bytes.len() <= maximum_bytes)
}

fn unique_ids(mut ids: impl Iterator<Item = Uuid>) -> bool {
    let mut seen = HashSet::new();
    ids.all(|id| seen.insert(id))
}

fn strictly_descending<'a>(entries: impl Iterator<Item = (&'a str, Uuid)>) -> bool {
    let mut previous = None;
    for (timestamp, id) in entries {
        let Some(timestamp) = parse_timestamp_text(timestamp) else {
            return false;
        };
        let current = (timestamp, id);
        if previous.is_some_and(|previous| previous <= current) {
            return false;
        }
        previous = Some(current);
    }
    true
}

#[cfg(test)]
mod tests {
    use rspice_cloud_contract::{UpdateWorkspaceMemberRequest, WorkspaceInvitationRole};
    use serde_json::json;

    use crate::InvitationToken;

    use super::*;

    #[test]
    fn governance_records_are_workspace_bound_clean_and_strictly_ordered() {
        let workspace_id = Uuid::from_u128(1);
        let audit_events = vec![
            AuditEvent {
                id: Uuid::from_u128(4),
                workspace_id,
                actor_principal_id: Some(Uuid::from_u128(2)),
                action: "workspace.member_added".to_owned(),
                target_type: "workspace_member".to_owned(),
                target_id: Some(Uuid::from_u128(3)),
                metadata: json!({"role": "editor"}),
                occurred_at: "2026-07-19T00:00:01Z".to_owned(),
            },
            AuditEvent {
                id: Uuid::from_u128(3),
                workspace_id,
                actor_principal_id: None,
                action: "workspace.created".to_owned(),
                target_type: "workspace".to_owned(),
                target_id: Some(workspace_id),
                metadata: json!({}),
                occurred_at: "2026-07-19T00:00:00Z".to_owned(),
            },
        ];
        assert!(valid_audit_events(&audit_events, workspace_id));

        let mut wrong_workspace = audit_events;
        wrong_workspace[0].workspace_id = Uuid::from_u128(9);
        assert!(!valid_audit_events(&wrong_workspace, workspace_id));

        let members = vec![WorkspaceMember {
            workspace_id,
            principal_id: Uuid::from_u128(5),
            email: Some("Engineer@rspice.app".to_owned()),
            display_name: Some("Precision Engineer".to_owned()),
            role: WorkspaceRole::Editor,
            joined_at: "2026-07-19T00:00:00Z".to_owned(),
        }];
        assert!(valid_workspace_members(&members, workspace_id));

        let mut malformed_profile = members;
        malformed_profile[0].email = Some("not-an-email".to_owned());
        assert!(!valid_workspace_members(&malformed_profile, workspace_id));
    }

    #[test]
    fn invitation_lifecycle_and_creation_response_are_exactly_bound() {
        let workspace_id = Uuid::from_u128(1);
        let token = InvitationToken::new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
            .expect("canonical invitation token");
        let request = CreateWorkspaceInvitation::new(
            "  Engineer@RSpice.App  ",
            WorkspaceInvitationRole::Editor,
            token,
        )
        .with_expires_at("2026-07-26T00:00:00.0000004Z");
        let mut response = CreatedWorkspaceInvitation {
            id: Uuid::from_u128(2),
            workspace_id,
            token: None,
            email: "engineer@rspice.app".to_owned(),
            role: WorkspaceInvitationRole::Editor,
            expires_at: "2026-07-26T00:00:00Z".to_owned(),
            accepted_at: None,
            revoked_at: None,
            created_at: "2026-07-19T00:00:00Z".to_owned(),
        };
        assert!(created_invitation_matches_request(
            &response,
            workspace_id,
            &request,
            false,
        ));
        let fresh_boundary = OffsetDateTime::parse(
            "2026-07-25T23:59:55Z",
            &time::format_description::well_known::Rfc3339,
        )
        .expect("freshness boundary");
        assert!(created_invitation_handoff_is_fresh_at(
            &response,
            fresh_boundary
        ));
        assert!(!created_invitation_handoff_is_fresh_at(
            &response,
            fresh_boundary.saturating_add(Duration::seconds(1))
        ));

        response.accepted_at = Some("2026-07-20T00:00:00Z".to_owned());
        assert!(!created_invitation_matches_request(
            &response,
            workspace_id,
            &request,
            false,
        ));
        assert!(created_invitation_matches_request(
            &response,
            workspace_id,
            &request,
            true,
        ));
        assert!(created_invitation_handoff_is_safe(&response));
        assert_eq!(
            workspace_invitation_api_path(workspace_id, response.id),
            "/api/v1/workspaces/00000000-0000-0000-0000-000000000001/invitations/00000000-0000-0000-0000-000000000002"
        );

        let impossible = WorkspaceInvitation {
            id: Uuid::from_u128(3),
            workspace_id,
            email: "engineer@rspice.app".to_owned(),
            role: WorkspaceInvitationRole::Viewer,
            expires_at: "2026-07-26T00:00:00Z".to_owned(),
            accepted_at: Some("2026-07-20T00:00:00Z".to_owned()),
            revoked_at: Some("2026-07-21T00:00:00Z".to_owned()),
            created_at: "2026-07-19T00:00:00Z".to_owned(),
        };
        assert!(!valid_workspace_invitations(&[impossible], workspace_id));
    }

    #[test]
    fn update_commands_reject_unknown_fields() {
        assert!(
            serde_json::from_value::<UpdateWorkspaceMemberRequest>(json!({
                "role": "viewer",
                "unexpected": true
            }))
            .is_err()
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use time::format_description::well_known::Rfc3339;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn browser_clock_rejects_stale_active_invitation_handoffs() {
        let now = current_time_utc().expect("browser clock");
        let mut invitation = CreatedWorkspaceInvitation {
            id: Uuid::from_u128(1),
            workspace_id: Uuid::from_u128(2),
            token: None,
            email: "engineer@rspice.test".to_owned(),
            role: rspice_cloud_contract::WorkspaceInvitationRole::Viewer,
            expires_at: now
                .saturating_add(Duration::minutes(1))
                .format(&Rfc3339)
                .expect("fresh expiry"),
            accepted_at: None,
            revoked_at: None,
            created_at: now.format(&Rfc3339).expect("creation time"),
        };
        assert!(created_invitation_handoff_is_safe(&invitation));

        invitation.expires_at = now.format(&Rfc3339).expect("stale expiry");
        assert!(!created_invitation_handoff_is_safe(&invitation));
    }
}
