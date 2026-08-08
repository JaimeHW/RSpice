//! Live-session response validation (ADR 0082).
//!
//! The relay connect credential is a bearer capability: these checks pin the
//! exact session-bound endpoint, the canonical ticket protocol, and the
//! caller's client-instance echo before the credential is ever offered to a
//! WebSocket, mirroring the collaboration ticket discipline.

use rspice_cloud_contract::{
    CAPABILITY_TOKEN_LENGTH, COLLABORATION_TICKET_PROTOCOL_PREFIX, JoinedLiveSession,
    LIVE_SESSION_PROTOCOL, LiveSession, LiveSessionAdmission, LiveSessionTicketProtocol, Uuid,
    is_canonical_capability_token,
};
use time::{Duration, OffsetDateTime};

use crate::{clock::current_time_utc, validation::parse_timestamp_text};

const MINIMUM_TICKET_START_WINDOW: Duration = Duration::seconds(5);

/// Crockford base-32 alphabet, matching the service's join-code generator.
const JOIN_CODE_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
/// Join codes display as `XXXXX-XXXXX`: ten alphabet characters and a dash.
const JOIN_CODE_DISPLAY_LENGTH: usize = 11;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveSessionTicketFailure {
    Invalid,
    CapabilityExpired,
}

pub(crate) fn validate_live_session_ticket(
    ticket: &LiveSessionTicketProtocol,
    session_id: Uuid,
    client_instance_id: Uuid,
) -> Result<(), LiveSessionTicketFailure> {
    let expires_at = validated_ticket_expiry(ticket, session_id, client_instance_id)?;
    let now = current_time_utc().ok_or(LiveSessionTicketFailure::CapabilityExpired)?;
    require_fresh_ticket(expires_at, now)
}

fn validated_ticket_expiry(
    ticket: &LiveSessionTicketProtocol,
    session_id: Uuid,
    client_instance_id: Uuid,
) -> Result<OffsetDateTime, LiveSessionTicketFailure> {
    if ticket.protocol != LIVE_SESSION_PROTOCOL
        || session_id.is_nil()
        || client_instance_id.is_nil()
        || ticket.client_instance_id != client_instance_id
        || ticket.websocket_endpoint != format!("/api/v1/live-sessions/{session_id}/connect")
    {
        return Err(LiveSessionTicketFailure::Invalid);
    }

    let valid_credential = ticket
        .ticket_protocol
        .strip_prefix(COLLABORATION_TICKET_PROTOCOL_PREFIX)
        .is_some_and(|token| {
            token.len() == CAPABILITY_TOKEN_LENGTH && is_canonical_capability_token(token)
        });
    if !valid_credential {
        return Err(LiveSessionTicketFailure::Invalid);
    }
    parse_timestamp_text(&ticket.expires_at).ok_or(LiveSessionTicketFailure::Invalid)
}

fn require_fresh_ticket(
    expires_at: OffsetDateTime,
    now: OffsetDateTime,
) -> Result<(), LiveSessionTicketFailure> {
    if expires_at >= now.saturating_add(MINIMUM_TICKET_START_WINDOW) {
        Ok(())
    } else {
        Err(LiveSessionTicketFailure::CapabilityExpired)
    }
}

/// A session response is structurally sound when it names itself and its
/// host, and the host is present in the roster the service filtered.
pub(crate) fn valid_live_session(session: &LiveSession) -> bool {
    !session.id.is_nil()
        && !session.host_principal_id.is_nil()
        && session
            .participants
            .iter()
            .any(|participant| participant.principal_id == session.host_principal_id)
}

/// A join response is coherent when the ticket's presence matches the
/// admission: admitted joins carry a ticket, pending joins carry none, and a
/// removed admission is never a successful join.
pub(crate) fn joined_session_is_coherent(joined: &JoinedLiveSession) -> bool {
    match joined.admission {
        LiveSessionAdmission::Admitted => joined.ticket.is_some(),
        LiveSessionAdmission::Pending => joined.ticket.is_none(),
        LiveSessionAdmission::Removed => false,
    }
}

/// A join code is exactly its display form: `XXXXX-XXXXX` over the Crockford
/// alphabet. The code is human-relayable and anything else is a service bug.
pub(crate) fn valid_join_code(code: &str) -> bool {
    code.len() == JOIN_CODE_DISPLAY_LENGTH
        && code.bytes().enumerate().all(|(index, byte)| {
            if index == JOIN_CODE_DISPLAY_LENGTH / 2 {
                byte == b'-'
            } else {
                JOIN_CODE_ALPHABET.contains(&byte)
            }
        })
}

#[cfg(test)]
mod tests {
    use rspice_cloud_contract::{LiveSessionCapability, LiveSessionParticipant, LiveSessionPolicy};

    use super::*;

    fn ticket(session_id: Uuid, client_instance_id: Uuid) -> LiveSessionTicketProtocol {
        LiveSessionTicketProtocol {
            protocol: LIVE_SESSION_PROTOCOL.to_owned(),
            websocket_endpoint: format!("/api/v1/live-sessions/{session_id}/connect"),
            ticket_protocol: format!(
                "{COLLABORATION_TICKET_PROTOCOL_PREFIX}{}",
                "A".repeat(CAPABILITY_TOKEN_LENGTH)
            ),
            client_instance_id,
            expires_at: "2026-07-19T00:00:05Z".to_owned(),
        }
    }

    fn session(session_id: Uuid, host_principal_id: Uuid) -> LiveSession {
        LiveSession {
            id: session_id,
            host_principal_id,
            circuit_id: None,
            policy: LiveSessionPolicy {
                default_capability: LiveSessionCapability::Edit,
                approve_joins: false,
                allow_save_copy: false,
            },
            created_at: "2026-07-19T00:00:00Z".to_owned(),
            participants: vec![LiveSessionParticipant {
                principal_id: host_principal_id,
                display_name: "Host".to_owned(),
                capability: LiveSessionCapability::Edit,
                admission: LiveSessionAdmission::Admitted,
                joined_at: "2026-07-19T00:00:00Z".to_owned(),
            }],
        }
    }

    #[test]
    fn tickets_are_exact_session_bound_and_use_only_canonical_protocol_credentials() {
        let session_id = Uuid::from_u128(1);
        let client_instance_id = Uuid::from_u128(2);
        let now = OffsetDateTime::parse(
            "2026-07-19T00:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .expect("test time");
        let mut response = ticket(session_id, client_instance_id);
        assert_eq!(
            validated_ticket_expiry(&response, session_id, client_instance_id)
                .and_then(|expires_at| require_fresh_ticket(expires_at, now)),
            Ok(())
        );

        response.expires_at = "2026-07-19T00:00:04Z".to_owned();
        assert_eq!(
            validated_ticket_expiry(&response, session_id, client_instance_id)
                .and_then(|expires_at| require_fresh_ticket(expires_at, now)),
            Err(LiveSessionTicketFailure::CapabilityExpired)
        );

        response = ticket(session_id, client_instance_id);
        response.expires_at = "not-a-timestamp".to_owned();
        assert_eq!(
            validated_ticket_expiry(&response, session_id, client_instance_id),
            Err(LiveSessionTicketFailure::Invalid)
        );

        response = ticket(session_id, client_instance_id);
        response.websocket_endpoint =
            format!("/api/v1/live-sessions/{}/connect", Uuid::from_u128(4));
        assert_eq!(
            validated_ticket_expiry(&response, session_id, client_instance_id),
            Err(LiveSessionTicketFailure::Invalid)
        );

        response = ticket(session_id, client_instance_id);
        response.protocol = "rspice.automerge.v1".to_owned();
        assert_eq!(
            validated_ticket_expiry(&response, session_id, client_instance_id),
            Err(LiveSessionTicketFailure::Invalid)
        );

        response = ticket(session_id, client_instance_id);
        response.ticket_protocol = format!(
            "{COLLABORATION_TICKET_PROTOCOL_PREFIX}{}",
            "!".repeat(CAPABILITY_TOKEN_LENGTH)
        );
        assert_eq!(
            validated_ticket_expiry(&response, session_id, client_instance_id),
            Err(LiveSessionTicketFailure::Invalid)
        );

        response = ticket(session_id, Uuid::from_u128(9));
        assert_eq!(
            validated_ticket_expiry(&response, session_id, client_instance_id),
            Err(LiveSessionTicketFailure::Invalid),
            "client-instance echo is exact"
        );
    }

    #[test]
    fn sessions_must_name_their_host_in_the_roster() {
        let session_id = Uuid::from_u128(1);
        let host = Uuid::from_u128(2);
        assert!(valid_live_session(&session(session_id, host)));

        let mut missing_host = session(session_id, host);
        missing_host.participants.clear();
        assert!(!valid_live_session(&missing_host));

        let mut nil_id = session(session_id, host);
        nil_id.id = Uuid::nil();
        assert!(!valid_live_session(&nil_id));
    }

    #[test]
    fn join_coherence_matches_ticket_presence_to_admission() {
        let base = session(Uuid::from_u128(1), Uuid::from_u128(2));
        let admitted_with_ticket = JoinedLiveSession {
            session: base.clone(),
            admission: LiveSessionAdmission::Admitted,
            capability: LiveSessionCapability::Edit,
            ticket: Some(ticket(Uuid::from_u128(1), Uuid::from_u128(3))),
        };
        assert!(joined_session_is_coherent(&admitted_with_ticket));

        let admitted_without_ticket = JoinedLiveSession {
            ticket: None,
            ..admitted_with_ticket
        };
        assert!(!joined_session_is_coherent(&admitted_without_ticket));

        let pending_without_ticket = JoinedLiveSession {
            session: base.clone(),
            admission: LiveSessionAdmission::Pending,
            capability: LiveSessionCapability::View,
            ticket: None,
        };
        assert!(joined_session_is_coherent(&pending_without_ticket));

        let pending_with_ticket = JoinedLiveSession {
            ticket: Some(ticket(Uuid::from_u128(1), Uuid::from_u128(3))),
            ..pending_without_ticket
        };
        assert!(!joined_session_is_coherent(&pending_with_ticket));

        let removed = JoinedLiveSession {
            session: base,
            admission: LiveSessionAdmission::Removed,
            capability: LiveSessionCapability::View,
            ticket: None,
        };
        assert!(!joined_session_is_coherent(&removed));
    }

    #[test]
    fn join_codes_are_exactly_their_display_form() {
        assert!(valid_join_code("ABCDE-FGHJK"));
        assert!(valid_join_code("01234-56789"));
        assert!(!valid_join_code("abcde-fghjk"), "lowercase is not display");
        assert!(!valid_join_code("ABCDEFGHJK"), "the dash is required");
        assert!(!valid_join_code("ABCDE-FGHU!"), "bad charset");
        assert!(!valid_join_code("ABCDE-FGHJKX"), "too long");
        assert!(!valid_join_code("ABCDU-FGHJK"), "U is not Crockford");
    }
}
