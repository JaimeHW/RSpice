use rspice_cloud_contract::{
    CAPABILITY_TOKEN_LENGTH, COLLABORATION_PROTOCOL, COLLABORATION_TICKET_PROTOCOL_PREFIX,
    CollaborationTicketProtocol, Uuid, is_canonical_capability_token,
};
use time::{Duration, OffsetDateTime};

use crate::{clock::current_time_utc, validation::parse_timestamp_text};

const MINIMUM_TICKET_START_WINDOW: Duration = Duration::seconds(5);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CollaborationTicketFailure {
    Invalid,
    CapabilityExpired,
}

pub(crate) fn validate_collaboration_ticket(
    ticket: &CollaborationTicketProtocol,
    circuit_id: Uuid,
    client_instance_id: Uuid,
) -> Result<(), CollaborationTicketFailure> {
    let expires_at = validated_ticket_expiry(ticket, circuit_id, client_instance_id)?;
    let now = current_time_utc().ok_or(CollaborationTicketFailure::CapabilityExpired)?;
    require_fresh_ticket(expires_at, now)
}

fn validated_ticket_expiry(
    ticket: &CollaborationTicketProtocol,
    circuit_id: Uuid,
    client_instance_id: Uuid,
) -> Result<OffsetDateTime, CollaborationTicketFailure> {
    if ticket.protocol != COLLABORATION_PROTOCOL
        || circuit_id.is_nil()
        || client_instance_id.is_nil()
        || ticket.client_instance_id != client_instance_id
        || ticket.websocket_endpoint != format!("/api/v1/collaboration/{circuit_id}/connect")
        || ticket.document.base_revision_id.is_nil()
        || ticket.document.schema_version <= 0
        || ticket.document.latest_sequence < 0
    {
        return Err(CollaborationTicketFailure::Invalid);
    }

    let valid_credential = ticket
        .ticket_protocol
        .strip_prefix(COLLABORATION_TICKET_PROTOCOL_PREFIX)
        .is_some_and(|token| {
            token.len() == CAPABILITY_TOKEN_LENGTH && is_canonical_capability_token(token)
        });
    if !valid_credential {
        return Err(CollaborationTicketFailure::Invalid);
    }
    parse_timestamp_text(&ticket.expires_at).ok_or(CollaborationTicketFailure::Invalid)
}

fn require_fresh_ticket(
    expires_at: OffsetDateTime,
    now: OffsetDateTime,
) -> Result<(), CollaborationTicketFailure> {
    if expires_at >= now.saturating_add(MINIMUM_TICKET_START_WINDOW) {
        Ok(())
    } else {
        Err(CollaborationTicketFailure::CapabilityExpired)
    }
}

#[cfg(test)]
mod tests {
    use rspice_cloud_contract::CollaborationDocument;

    use super::*;

    fn ticket(circuit_id: Uuid, client_instance_id: Uuid) -> CollaborationTicketProtocol {
        CollaborationTicketProtocol {
            protocol: COLLABORATION_PROTOCOL.to_owned(),
            websocket_endpoint: format!("/api/v1/collaboration/{circuit_id}/connect"),
            ticket_protocol: format!(
                "{COLLABORATION_TICKET_PROTOCOL_PREFIX}{}",
                "A".repeat(CAPABILITY_TOKEN_LENGTH)
            ),
            client_instance_id,
            expires_at: "2026-07-19T00:00:05Z".to_owned(),
            document: CollaborationDocument {
                base_revision_id: Uuid::from_u128(3),
                schema_version: 1,
                latest_sequence: 0,
            },
        }
    }

    #[test]
    fn tickets_are_exact_circuit_bound_and_use_only_canonical_protocol_credentials() {
        let circuit_id = Uuid::from_u128(1);
        let client_instance_id = Uuid::from_u128(2);
        let now = OffsetDateTime::parse(
            "2026-07-19T00:00:00Z",
            &time::format_description::well_known::Rfc3339,
        )
        .expect("test time");
        let mut response = ticket(circuit_id, client_instance_id);
        assert_eq!(
            validated_ticket_expiry(&response, circuit_id, client_instance_id)
                .and_then(|expires_at| require_fresh_ticket(expires_at, now)),
            Ok(())
        );

        response.expires_at = "2026-07-19T00:00:04Z".to_owned();
        assert_eq!(
            validated_ticket_expiry(&response, circuit_id, client_instance_id)
                .and_then(|expires_at| require_fresh_ticket(expires_at, now)),
            Err(CollaborationTicketFailure::CapabilityExpired)
        );

        response = ticket(circuit_id, client_instance_id);
        response.expires_at = "not-a-timestamp".to_owned();
        assert_eq!(
            validated_ticket_expiry(&response, circuit_id, client_instance_id),
            Err(CollaborationTicketFailure::Invalid)
        );

        response = ticket(circuit_id, client_instance_id);
        response.websocket_endpoint =
            format!("/api/v1/collaboration/{}/connect", Uuid::from_u128(4));
        assert_eq!(
            validated_ticket_expiry(&response, circuit_id, client_instance_id),
            Err(CollaborationTicketFailure::Invalid)
        );

        response = ticket(circuit_id, client_instance_id);
        response.ticket_protocol = format!(
            "{COLLABORATION_TICKET_PROTOCOL_PREFIX}{}",
            "B".repeat(CAPABILITY_TOKEN_LENGTH)
        );
        assert_eq!(
            validated_ticket_expiry(&response, circuit_id, client_instance_id),
            Err(CollaborationTicketFailure::Invalid)
        );

        response = ticket(circuit_id, client_instance_id);
        response.document.latest_sequence = -1;
        assert_eq!(
            validated_ticket_expiry(&response, circuit_id, client_instance_id),
            Err(CollaborationTicketFailure::Invalid)
        );

        response = ticket(circuit_id, client_instance_id);
        assert_eq!(
            validated_ticket_expiry(&response, Uuid::nil(), client_instance_id),
            Err(CollaborationTicketFailure::Invalid)
        );
        assert_eq!(
            validated_ticket_expiry(&response, circuit_id, Uuid::nil()),
            Err(CollaborationTicketFailure::Invalid)
        );
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use time::format_description::well_known::Rfc3339;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use rspice_cloud_contract::CollaborationDocument;

    #[wasm_bindgen_test]
    fn browser_clock_admits_only_fresh_collaboration_tickets() {
        let circuit_id = Uuid::from_u128(1);
        let client_instance_id = Uuid::from_u128(2);
        let now = current_time_utc().expect("browser clock");
        let ticket = CollaborationTicketProtocol {
            protocol: COLLABORATION_PROTOCOL.to_owned(),
            websocket_endpoint: format!("/api/v1/collaboration/{circuit_id}/connect"),
            ticket_protocol: format!(
                "{COLLABORATION_TICKET_PROTOCOL_PREFIX}{}",
                "A".repeat(CAPABILITY_TOKEN_LENGTH)
            ),
            client_instance_id,
            expires_at: now
                .saturating_add(Duration::seconds(60))
                .format(&Rfc3339)
                .expect("fresh ticket timestamp"),
            document: CollaborationDocument {
                base_revision_id: Uuid::from_u128(3),
                schema_version: 1,
                latest_sequence: 0,
            },
        };
        assert_eq!(
            validate_collaboration_ticket(&ticket, circuit_id, client_instance_id),
            Ok(())
        );

        let mut expired = ticket;
        expired.expires_at = now.format(&Rfc3339).expect("expired ticket timestamp");
        assert_eq!(
            validate_collaboration_ticket(&expired, circuit_id, client_instance_id),
            Err(CollaborationTicketFailure::CapabilityExpired)
        );
    }
}
