//! The cloud contract as both account executors use it.
//!
//! The native and browser executors differ in how they run (a background
//! thread against `spawn_local`), not in what they ask the service or what
//! they make of its answers. The live-session requests, the snapshot rows
//! built from contract records and the presentation-safe failure text live
//! here once, so the two runtimes cannot drift apart.

use rspice_cloud_client::contract::{
    CreatedLiveSession, Entitlement, JoinedLiveSession, LiveSession, LiveSessionAdmission,
    LiveSessionCapability, LiveSessionPolicy, LiveSessionTicketProtocol,
};
use rspice_cloud_client::{BearerToken, CloudClient, CloudError, IdempotencyKey};

use super::{
    EntitlementSummary, LiveParticipantSummary, LiveSessionPolicySummary, LiveSessionSummary,
};

/// Presentation fallback when the service cannot be reached at all.
pub(super) const LIVE_SESSION_UNREACHABLE: &str = "The live-session service could not be reached.";

pub(super) enum LiveSessionReadFailure {
    /// The server refused the read: ended, aged out, or removed —
    /// indistinguishable by design.
    Gone,
    /// Transport or token trouble; the next poll retries.
    Transient,
}

pub(super) async fn create_live_session(
    cloud: &CloudClient,
    access: &str,
    create_key: &str,
    policy: LiveSessionPolicy,
    circuit_id: Option<uuid::Uuid>,
    client_instance_id: uuid::Uuid,
) -> Result<CreatedLiveSession, String> {
    let token = BearerToken::new(access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
    let key = IdempotencyKey::new(create_key).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
    cloud
        .create_live_session_idempotent(&token, &key, policy, circuit_id, client_instance_id)
        .await
        .map(rspice_cloud_client::CloudResponse::into_body)
        .map_err(|error| live_session_error_message(&error))
}

pub(super) async fn join_live_session(
    cloud: &CloudClient,
    access: &str,
    code: &str,
    client_instance_id: uuid::Uuid,
) -> Result<JoinedLiveSession, String> {
    let token = BearerToken::new(access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
    cloud
        .join_live_session(&token, code, client_instance_id)
        .await
        .map(rspice_cloud_client::CloudResponse::into_body)
        .map_err(|error| live_session_error_message(&error))
}

pub(super) async fn issue_live_session_ticket(
    cloud: &CloudClient,
    access: &str,
    session_id: uuid::Uuid,
    client_instance_id: uuid::Uuid,
) -> Result<LiveSessionTicketProtocol, String> {
    let token = BearerToken::new(access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
    cloud
        .issue_live_session_ticket(&token, session_id, client_instance_id)
        .await
        .map(rspice_cloud_client::CloudResponse::into_body)
        .map_err(|error| live_session_error_message(&error))
}

pub(super) async fn read_live_session(
    cloud: &CloudClient,
    access: &str,
    session_id: uuid::Uuid,
) -> Result<LiveSession, LiveSessionReadFailure> {
    let token = BearerToken::new(access).map_err(|_| LiveSessionReadFailure::Transient)?;
    match cloud.get_live_session(&token, session_id).await {
        Ok(response) => Ok(response.into_body()),
        Err(CloudError::Problem { details, .. }) if matches!(details.status, 403 | 404 | 410) => {
            Err(LiveSessionReadFailure::Gone)
        }
        Err(_) => Err(LiveSessionReadFailure::Transient),
    }
}

pub(super) fn contract_policy(policy: LiveSessionPolicySummary) -> LiveSessionPolicy {
    LiveSessionPolicy {
        default_capability: if policy.guests_edit {
            LiveSessionCapability::Edit
        } else {
            LiveSessionCapability::View
        },
        approve_joins: policy.approve_joins,
        allow_save_copy: policy.allow_save_copy,
    }
}

pub(super) fn live_session_summary(
    session: &LiveSession,
    self_principal: Option<uuid::Uuid>,
    hosting: bool,
    join_code: Option<String>,
) -> LiveSessionSummary {
    let participants: Vec<LiveParticipantSummary> = session
        .participants
        .iter()
        .map(|participant| {
            let is_host = participant.principal_id == session.host_principal_id;
            LiveParticipantSummary {
                principal_id: participant.principal_id.to_string(),
                display_name: participant.display_name.clone(),
                is_host,
                is_self: self_principal.is_some_and(|own| own == participant.principal_id),
                editor: is_host || participant.capability == LiveSessionCapability::Edit,
                pending: participant.admission == LiveSessionAdmission::Pending,
                joined_at: participant.joined_at.clone(),
            }
        })
        .collect();
    let editor = hosting
        || participants
            .iter()
            .any(|participant| participant.is_self && participant.editor);
    LiveSessionSummary {
        session_id: session.id.to_string(),
        join_code,
        hosting,
        editor,
        policy: LiveSessionPolicySummary {
            guests_edit: session.policy.default_capability == LiveSessionCapability::Edit,
            approve_joins: session.policy.approve_joins,
            allow_save_copy: session.policy.allow_save_copy,
        },
        participants,
        started_at: session.created_at.clone(),
        relay_connected: false,
        notice: None,
    }
}

/// Presentation-safe failure text: the service's own problem title when the
/// server answered deliberately, a fixed line otherwise.
pub(super) fn live_session_error_message(error: &CloudError) -> String {
    match error {
        CloudError::Problem { details, .. } => details.title.clone(),
        _ => LIVE_SESSION_UNREACHABLE.to_owned(),
    }
}

/// Presentation-safe reading of the relay's deliberate close reasons.
pub(super) fn session_over_message(reason: &str) -> String {
    match reason {
        "removed from the live session" => "The host removed you from this live session.",
        "live session expired" => "This live session reached its maximum duration and ended.",
        _ => "This live session has ended.",
    }
    .to_owned()
}

pub(super) fn entitlement_summary(entitlement: &Entitlement) -> EntitlementSummary {
    EntitlementSummary {
        status: entitlement.status.as_str().to_owned(),
        granted_features: granted_features(&entitlement.features),
        valid_until: entitlement.valid_until.clone(),
    }
}

/// Explicit `true` grants only; everything else is denial.
pub(super) fn granted_features(features: &serde_json::Value) -> Vec<String> {
    let Some(map) = features.as_object() else {
        return Vec::new();
    };
    let mut granted: Vec<String> = map
        .iter()
        .filter(|(_, value)| value.as_bool() == Some(true))
        .map(|(name, _)| name.clone())
        .collect();
    granted.sort();
    granted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn granted_features_admits_only_explicit_true() {
        let features = serde_json::json!({
            "cloud_publishing": true,
            "live_collaboration": false,
            "max_devices": 3,
            "native_license": "true",
        });
        assert_eq!(granted_features(&features), vec!["cloud_publishing"]);
        assert!(granted_features(&serde_json::json!([])).is_empty());
    }

    #[test]
    fn live_session_summaries_mark_host_self_pending_and_editors() {
        use rspice_cloud_client::contract::LiveSessionParticipant;
        let host = uuid::Uuid::from_u128(1);
        let guest = uuid::Uuid::from_u128(2);
        let session = LiveSession {
            id: uuid::Uuid::from_u128(9),
            host_principal_id: host,
            circuit_id: None,
            policy: LiveSessionPolicy {
                default_capability: LiveSessionCapability::View,
                approve_joins: true,
                allow_save_copy: false,
            },
            created_at: "2026-08-08T00:00:00Z".to_owned(),
            participants: vec![
                LiveSessionParticipant {
                    principal_id: host,
                    display_name: "Host".to_owned(),
                    capability: LiveSessionCapability::Edit,
                    admission: LiveSessionAdmission::Admitted,
                    joined_at: "2026-08-08T00:00:00Z".to_owned(),
                },
                LiveSessionParticipant {
                    principal_id: guest,
                    display_name: "Guest".to_owned(),
                    capability: LiveSessionCapability::View,
                    admission: LiveSessionAdmission::Pending,
                    joined_at: "2026-08-08T00:01:00Z".to_owned(),
                },
            ],
        };

        let summary = live_session_summary(&session, Some(guest), false, None);
        assert!(!summary.hosting);
        assert!(!summary.editor, "a viewer guest is not an editor");
        assert!(!summary.policy.guests_edit);
        assert!(summary.policy.approve_joins);
        assert!(summary.join_code.is_none());
        let host_row = &summary.participants[0];
        assert!(host_row.is_host && host_row.editor && !host_row.is_self);
        let guest_row = &summary.participants[1];
        assert!(guest_row.is_self && guest_row.pending && !guest_row.editor);

        let hosted =
            live_session_summary(&session, Some(host), true, Some("ABCDE-FGHJK".to_owned()));
        assert!(hosted.hosting && hosted.editor);
        assert_eq!(hosted.join_code.as_deref(), Some("ABCDE-FGHJK"));
    }

    #[test]
    fn live_session_policy_maps_editors_to_the_contract_capability() {
        let editors = contract_policy(LiveSessionPolicySummary {
            guests_edit: true,
            approve_joins: false,
            allow_save_copy: true,
        });
        assert_eq!(editors.default_capability, LiveSessionCapability::Edit);
        assert!(!editors.approve_joins);
        assert!(editors.allow_save_copy);
        let viewers = contract_policy(LiveSessionPolicySummary {
            guests_edit: false,
            approve_joins: true,
            allow_save_copy: false,
        });
        assert_eq!(viewers.default_capability, LiveSessionCapability::View);
    }

    #[test]
    fn live_session_errors_surface_only_deliberate_problem_titles() {
        let details: rspice_cloud_client::contract::ProblemDetails =
            serde_json::from_value(serde_json::json!({
                "type": "about:blank",
                "title": "Live collaboration requires an active entitlement.",
                "status": 403,
                "detail": "internal wording that stays out of the summary line",
                "instance": "about:blank",
            }))
            .expect("problem details");
        let problem = CloudError::Problem {
            details: Box::new(details),
            metadata: rspice_cloud_client::ResponseMetadata::default(),
        };
        assert_eq!(
            live_session_error_message(&problem),
            "Live collaboration requires an active entitlement."
        );
    }
}
