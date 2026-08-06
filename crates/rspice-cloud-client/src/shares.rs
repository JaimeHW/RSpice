use std::{collections::HashSet, error::Error, fmt};

use http::HeaderValue;
use rspice_cloud_contract::{
    API_VERSION, CAPABILITY_TOKEN_BYTES, CAPABILITY_TOKEN_LENGTH, CircuitShare,
    CreatedCircuitShare, SharePermission, SharedCircuit, Uuid, is_canonical_capability_token,
};
use serde::{Serialize, Serializer, ser::SerializeStruct};
use sha2::{Digest, Sha256};
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    clock::current_time_utc,
    validation::{parse_timestamp_text, valid_clean_circuit_title, valid_revision_snapshot},
};

const MAX_STORED_SHARE_LIFETIME_DAYS: i64 = 366;
const MINIMUM_SHARE_HANDOFF_WINDOW: Duration = Duration::seconds(5);

/// A validated, borrowed RSpice bearer-share token.
///
/// Generate 32 bytes with the platform cryptographic random generator, encode
/// them as unpadded base64url, and retain the resulting token in the platform's
/// protected pending-command state. This type never copies or owns the token.
#[derive(Clone, Copy)]
pub struct ShareToken<'a>(&'a str);

impl<'a> ShareToken<'a> {
    /// Validates the exact canonical bearer-share token representation.
    pub fn new(value: &'a str) -> Result<Self, ShareTokenError> {
        validate_capability_token(value)?;
        Ok(Self(value))
    }

    /// Returns the borrowed raw capability for protected deep-link handling.
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.0
    }

    /// Builds the exact `Authorization: Bearer` value for request-target-safe
    /// resolution and marks it sensitive for HTTP debug formatting.
    #[must_use]
    pub fn authorization_value(self) -> HeaderValue {
        let mut value = HeaderValue::from_str(&format!("Bearer {}", self.0))
            .expect("a validated capability always forms a valid header value");
        value.set_sensitive(true);
        value
    }

    /// Computes the lowercase SHA-256 commitment required by retry-safe share
    /// creation over the exact 43 ASCII token bytes.
    #[must_use]
    pub fn commitment_sha256(self) -> String {
        capability_commitment_sha256(self.0)
    }
}

impl fmt::Debug for ShareToken<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ShareToken([REDACTED])")
    }
}

/// Preferred client-committed bearer-share creation command.
///
/// The command borrows the caller-owned capability and serializes only its
/// SHA-256 commitment. The raw token remains in platform-protected pending
/// state and is never part of the HTTP body.
pub struct CreateCircuitShare<'a> {
    token: ShareToken<'a>,
    revision_id: Option<Uuid>,
    expires_at: Option<&'a str>,
}

impl<'a> CreateCircuitShare<'a> {
    /// Creates a read-only share command pinned to the circuit's current head.
    pub const fn new(token: ShareToken<'a>) -> Self {
        Self {
            token,
            revision_id: None,
            expires_at: None,
        }
    }

    /// Pins an explicit sealed revision instead of resolving the current head.
    #[must_use]
    pub const fn with_revision_id(mut self, revision_id: Uuid) -> Self {
        self.revision_id = Some(revision_id);
        self
    }

    /// Supplies an RFC 3339 expiry accepted by the authoritative API policy.
    #[must_use]
    pub const fn with_expires_at(mut self, expires_at: &'a str) -> Self {
        self.expires_at = Some(expires_at);
        self
    }
}

impl Serialize for CreateCircuitShare<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut field_count = 2;
        field_count += usize::from(self.revision_id.is_some());
        field_count += usize::from(self.expires_at.is_some());
        let mut state = serializer.serialize_struct("CreateCircuitShare", field_count)?;
        state.serialize_field("permission", &SharePermission::View)?;
        if let Some(revision_id) = self.revision_id {
            state.serialize_field("revision_id", &revision_id)?;
        }
        if let Some(expires_at) = self.expires_at {
            state.serialize_field("expires_at", expires_at)?;
        }
        state.serialize_field("token_sha256", &self.token.commitment_sha256())?;
        state.end()
    }
}

impl fmt::Debug for CreateCircuitShare<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CreateCircuitShare")
            .field("permission", &SharePermission::View)
            .field("revision_id", &self.revision_id)
            .field("has_expires_at", &self.expires_at.is_some())
            .field("token", &"[REDACTED]")
            .finish()
    }
}

/// A validated, borrowed workspace-invitation capability.
///
/// This type never owns the secret. Web clients should validate a fragment
/// immediately, remove it from visible history, and borrow it only while
/// constructing the bounded invitation-acceptance body.
#[derive(Clone, Copy)]
pub struct InvitationToken<'a>(&'a str);

impl<'a> InvitationToken<'a> {
    /// Validates the exact canonical invitation-token representation.
    pub fn new(value: &'a str) -> Result<Self, InvitationTokenError> {
        validate_capability_token(value)?;
        Ok(Self(value))
    }

    /// Returns the borrowed capability for the no-store acceptance body.
    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.0
    }

    /// Computes the lowercase SHA-256 commitment required by retry-safe
    /// invitation creation over the exact 43 ASCII token bytes.
    #[must_use]
    pub fn commitment_sha256(self) -> String {
        capability_commitment_sha256(self.0)
    }
}

impl fmt::Debug for InvitationToken<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("InvitationToken([REDACTED])")
    }
}

fn validate_capability_token(value: &str) -> Result<(), CapabilityTokenError> {
    if value.len() != CAPABILITY_TOKEN_LENGTH {
        return Err(CapabilityTokenError::InvalidLength);
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(CapabilityTokenError::InvalidCharacter);
    }
    // A 32-byte unpadded base64url value uses four significant bits in its
    // final character. Requiring the two unused bits to be zero proves the
    // representation is canonical without copying secret material into a
    // decoder-owned buffer.
    if !is_canonical_capability_token(value) {
        return Err(CapabilityTokenError::InvalidEncoding);
    }
    debug_assert_eq!(CAPABILITY_TOKEN_BYTES * 8, CAPABILITY_TOKEN_LENGTH * 6 - 2);
    Ok(())
}

fn capability_commitment_sha256(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    let mut encoded = String::with_capacity(64);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in digest {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

pub(crate) fn share_id_from_location(location: &str, circuit_id: Uuid) -> Option<Uuid> {
    let prefix = format!("/api/{API_VERSION}/circuits/{circuit_id}/shares/");
    let raw_share_id = location.strip_prefix(&prefix)?;
    let share_id = Uuid::parse_str(raw_share_id).ok()?;
    (share_id.to_string() == raw_share_id).then_some(share_id)
}

pub(crate) fn valid_circuit_share(
    share: &CircuitShare,
    expected_circuit_id: Uuid,
    expected_share_id: Option<Uuid>,
) -> bool {
    valid_share_fields(
        ShareFields {
            id: share.id,
            circuit_id: share.circuit_id,
            permission: share.permission,
            revision_id: share.revision_id,
            expires_at: share.expires_at.as_deref(),
            sealed_at: &share.sealed_at,
            revoked_at: share.revoked_at.as_deref(),
            created_at: &share.created_at,
        },
        expected_circuit_id,
        expected_share_id,
    )
}

pub(crate) fn valid_circuit_share_list(shares: &[CircuitShare], circuit_id: Uuid) -> bool {
    if circuit_id.is_nil()
        || shares
            .iter()
            .any(|share| !valid_circuit_share(share, circuit_id, None))
        || shares
            .iter()
            .map(|share| share.id)
            .collect::<HashSet<_>>()
            .len()
            != shares.len()
    {
        return false;
    }

    shares.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].created_at)
            .zip(parse_timestamp_text(&pair[1].created_at))
            .is_some_and(|(newer, older)| (newer, pair[0].id) > (older, pair[1].id))
    })
}

pub(crate) fn created_share_matches_request(
    share: &CreatedCircuitShare,
    circuit_id: Uuid,
    request: &CreateCircuitShare<'_>,
    replayed: bool,
) -> bool {
    if share.token.is_some()
        || !valid_share_fields(
            ShareFields {
                id: share.id,
                circuit_id: share.circuit_id,
                permission: share.permission,
                revision_id: share.revision_id,
                expires_at: share.expires_at.as_deref(),
                sealed_at: &share.sealed_at,
                revoked_at: share.revoked_at.as_deref(),
                created_at: &share.created_at,
            },
            circuit_id,
            None,
        )
        || request.revision_id.is_some_and(|id| id.is_nil())
        || request
            .revision_id
            .is_some_and(|revision_id| share.revision_id != revision_id)
        || (!replayed && share.revoked_at.is_some())
    {
        return false;
    }

    match (request.expires_at, share.expires_at.as_deref()) {
        (None, None) => true,
        (Some(requested), Some(returned)) => normalize_requested_timestamp(requested)
            .zip(parse_timestamp_text(returned))
            .is_some_and(|(requested, returned)| requested == returned),
        _ => false,
    }
}

pub(crate) fn created_share_handoff_is_safe(share: &CreatedCircuitShare) -> bool {
    if share.revoked_at.is_some() || share.expires_at.is_none() {
        return true;
    }
    current_time_utc().is_some_and(|now| created_share_handoff_is_fresh_at(share, now))
}

fn created_share_handoff_is_fresh_at(share: &CreatedCircuitShare, now: OffsetDateTime) -> bool {
    share
        .expires_at
        .as_deref()
        .and_then(parse_timestamp_text)
        .is_some_and(|expires_at| expires_at >= now.saturating_add(MINIMUM_SHARE_HANDOFF_WINDOW))
}

struct ShareFields<'a> {
    id: Uuid,
    circuit_id: Uuid,
    permission: SharePermission,
    revision_id: Uuid,
    expires_at: Option<&'a str>,
    sealed_at: &'a str,
    revoked_at: Option<&'a str>,
    created_at: &'a str,
}

fn valid_share_fields(
    share: ShareFields<'_>,
    expected_circuit_id: Uuid,
    expected_share_id: Option<Uuid>,
) -> bool {
    if share.id.is_nil()
        || expected_circuit_id.is_nil()
        || share.circuit_id != expected_circuit_id
        || expected_share_id.is_some_and(|expected| share.id != expected)
        || share.permission != SharePermission::View
        || share.revision_id.is_nil()
    {
        return false;
    }
    let Some(created_at) = parse_timestamp_text(share.created_at) else {
        return false;
    };
    let Some(sealed_at) = parse_timestamp_text(share.sealed_at) else {
        return false;
    };
    if sealed_at < created_at {
        return false;
    }
    let maximum_expiry = created_at.checked_add(Duration::days(MAX_STORED_SHARE_LIFETIME_DAYS));
    if share.expires_at.is_some_and(|value| {
        parse_timestamp_text(value).is_none_or(|expires_at| {
            expires_at <= sealed_at || maximum_expiry.is_none_or(|maximum| expires_at > maximum)
        })
    }) {
        return false;
    }
    share.revoked_at.is_none_or(|value| {
        parse_timestamp_text(value).is_some_and(|revoked_at| revoked_at >= sealed_at)
    })
}

fn normalize_requested_timestamp(value: &str) -> Option<OffsetDateTime> {
    if value.is_empty() || value.len() > 64 {
        return None;
    }
    let timestamp = OffsetDateTime::parse(value, &Rfc3339).ok()?;
    let microseconds = timestamp.unix_timestamp_nanos().div_euclid(1_000) * 1_000;
    OffsetDateTime::from_unix_timestamp_nanos(microseconds).ok()
}

pub(crate) fn valid_shared_circuit(
    shared: &SharedCircuit,
    expected_circuit_id: Option<Uuid>,
) -> bool {
    let title = shared.title.as_str();
    let revision = &shared.revision;
    !shared.circuit_id.is_nil()
        && expected_circuit_id.is_none_or(|expected| shared.circuit_id == expected)
        && valid_clean_circuit_title(title)
        && shared.permission == SharePermission::View
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

/// Reason a capability token was rejected before a request was made.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityTokenError {
    /// The token was not the 43-character encoding of 32 bytes.
    InvalidLength,
    /// The token contained a character outside unpadded base64url.
    InvalidCharacter,
    /// The token had a non-canonical or otherwise invalid base64url encoding.
    InvalidEncoding,
}

impl fmt::Display for CapabilityTokenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLength => "capability token must be exactly 43 characters",
            Self::InvalidCharacter => "capability token contains an invalid character",
            Self::InvalidEncoding => "capability token is not canonical 32-byte base64url",
        })
    }
}

impl Error for CapabilityTokenError {}

/// Share-token validation error.
pub type ShareTokenError = CapabilityTokenError;
/// Invitation-token validation error.
pub type InvitationTokenError = CapabilityTokenError;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn shared_circuit() -> SharedCircuit {
        SharedCircuit {
            circuit_id: Uuid::from_u128(1),
            title: "Precision amplifier".to_owned(),
            permission: SharePermission::View,
            revision: rspice_cloud_contract::SharedCircuitRevision {
                id: Uuid::from_u128(2),
                schema_version: 1,
                content_digest_version: 2,
                document: json!({"components": []}),
                artifact_ids: vec![Uuid::from_u128(3)],
                content_sha256: "0a".repeat(32),
                created_at: "2026-07-19T00:00:00Z".to_owned(),
            },
        }
    }

    fn circuit_share(id: u128) -> CircuitShare {
        CircuitShare {
            id: Uuid::from_u128(id),
            circuit_id: Uuid::from_u128(10),
            permission: SharePermission::View,
            revision_id: Uuid::from_u128(11),
            expires_at: Some("2026-08-01T00:00:00Z".to_owned()),
            sealed_at: "2026-07-19T00:00:01Z".to_owned(),
            revoked_at: None,
            created_at: "2026-07-19T00:00:00Z".to_owned(),
        }
    }

    #[test]
    fn canonical_tokens_produce_the_exact_commitment_without_debug_exposure() {
        let raw = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let token = ShareToken::new(raw).expect("canonical 32-byte token");
        assert_eq!(token.as_str(), raw);
        assert_eq!(
            token.commitment_sha256(),
            "0f007385b6f9d4b7eeb2748605afe1a984a0a3bfa3f014d09e2a784ce9e5cd1a"
        );
        let debug = format!("{token:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(raw));
        let authorization = token.authorization_value();
        assert_eq!(authorization, format!("Bearer {raw}"));
        assert!(authorization.is_sensitive());
        assert!(!format!("{authorization:?}").contains(raw));
    }

    #[test]
    fn share_commands_serialize_only_the_token_commitment() {
        let raw = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let token = ShareToken::new(raw).expect("canonical share token");
        let revision_id = Uuid::from_u128(7);
        let request = CreateCircuitShare::new(token)
            .with_revision_id(revision_id)
            .with_expires_at("2026-08-01T00:00:00Z");
        let serialized = serde_json::to_string(&request).expect("serialize share command");
        assert!(!serialized.contains(raw));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).expect("share command JSON"),
            serde_json::json!({
                "permission": "view",
                "revision_id": revision_id,
                "expires_at": "2026-08-01T00:00:00Z",
                "token_sha256": "0f007385b6f9d4b7eeb2748605afe1a984a0a3bfa3f014d09e2a784ce9e5cd1a"
            })
        );
        let minimal = serde_json::to_value(CreateCircuitShare::new(token))
            .expect("serialize minimal share command");
        assert_eq!(minimal.as_object().expect("share object").len(), 2);
        assert!(minimal.get("revision_id").is_none());
        assert!(minimal.get("expires_at").is_none());
        let debug = format!("{request:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(raw));
        assert!(!debug.contains("2026-08-01T00:00:00Z"));
    }

    #[test]
    fn share_locations_are_exact_and_circuit_bound() {
        let circuit_id = Uuid::from_u128(8);
        let share_id = Uuid::from_u128(9);
        let location = format!("/api/v1/circuits/{circuit_id}/shares/{share_id}");
        assert_eq!(
            share_id_from_location(&location, circuit_id),
            Some(share_id)
        );
        assert!(share_id_from_location(&location, Uuid::from_u128(10)).is_none());
        assert!(share_id_from_location(&format!("{location}/extra"), circuit_id).is_none());
    }

    #[test]
    fn management_projections_are_circuit_bound_ordered_and_lifecycle_safe() {
        let circuit_id = Uuid::from_u128(10);
        let newer = circuit_share(9);
        let mut older = circuit_share(8);
        older.created_at = "2026-07-18T00:00:00Z".to_owned();
        older.sealed_at = "2026-07-18T00:00:01Z".to_owned();
        assert!(valid_circuit_share(&newer, circuit_id, Some(newer.id)));
        assert!(valid_circuit_share_list(
            &[newer.clone(), older.clone()],
            circuit_id
        ));
        assert!(!valid_circuit_share_list(&[], Uuid::nil()));
        assert!(!valid_circuit_share_list(
            &[older, newer.clone()],
            circuit_id
        ));
        assert!(!valid_circuit_share_list(
            &[newer.clone(), newer.clone()],
            circuit_id
        ));

        let mut invalid = newer.clone();
        invalid.circuit_id = Uuid::from_u128(12);
        assert!(!valid_circuit_share(&invalid, circuit_id, None));
        invalid = newer.clone();
        invalid.sealed_at = "2026-07-18T23:59:59Z".to_owned();
        assert!(!valid_circuit_share(&invalid, circuit_id, None));
        invalid = newer;
        invalid.expires_at = Some("2026-07-19T00:00:01Z".to_owned());
        assert!(!valid_circuit_share(&invalid, circuit_id, None));
        invalid.expires_at = Some("2027-07-21T00:00:00Z".to_owned());
        assert!(!valid_circuit_share(&invalid, circuit_id, None));
    }

    #[test]
    fn keyed_creation_binds_revision_expiry_and_fresh_lifecycle() {
        let token = ShareToken::new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")
            .expect("canonical share token");
        let circuit_id = Uuid::from_u128(10);
        let revision_id = Uuid::from_u128(11);
        let request = CreateCircuitShare::new(token)
            .with_revision_id(revision_id)
            .with_expires_at("2026-08-01T00:00:00.0000009Z");
        let mut response = CreatedCircuitShare {
            id: Uuid::from_u128(9),
            circuit_id,
            token: None,
            permission: SharePermission::View,
            revision_id,
            expires_at: Some("2026-08-01T00:00:00Z".to_owned()),
            sealed_at: "2026-07-19T00:00:01Z".to_owned(),
            revoked_at: None,
            created_at: "2026-07-19T00:00:00Z".to_owned(),
        };
        assert!(created_share_matches_request(
            &response, circuit_id, &request, false
        ));
        let fresh_boundary = OffsetDateTime::parse(
            "2026-07-31T23:59:55Z",
            &time::format_description::well_known::Rfc3339,
        )
        .expect("freshness boundary");
        assert!(created_share_handoff_is_fresh_at(&response, fresh_boundary));
        assert!(!created_share_handoff_is_fresh_at(
            &response,
            fresh_boundary.saturating_add(Duration::seconds(1))
        ));

        response.revision_id = Uuid::from_u128(12);
        assert!(!created_share_matches_request(
            &response, circuit_id, &request, false
        ));
        response.revision_id = revision_id;
        response.revoked_at = Some("2026-07-20T00:00:00Z".to_owned());
        assert!(!created_share_matches_request(
            &response, circuit_id, &request, false
        ));
        assert!(created_share_matches_request(
            &response, circuit_id, &request, true
        ));
        assert!(created_share_handoff_is_safe(&response));
    }

    #[test]
    fn malformed_or_noncanonical_tokens_are_rejected() {
        assert_eq!(
            ShareToken::new("short").expect_err("short token"),
            ShareTokenError::InvalidLength
        );
        assert_eq!(
            ShareToken::new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA/")
                .expect_err("non-base64url character"),
            ShareTokenError::InvalidCharacter
        );
        assert_eq!(
            ShareToken::new("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB")
                .expect_err("noncanonical trailing bits"),
            ShareTokenError::InvalidEncoding
        );

        for final_character in b"AEIMQUYcgkosw048" {
            let token = format!(
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA{}",
                char::from(*final_character)
            );
            ShareToken::new(&token).expect("every canonical final quartet is accepted");
        }
    }

    #[test]
    fn invitation_tokens_are_borrowed_and_debug_redacted() {
        let raw = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let token = InvitationToken::new(raw).expect("canonical invitation token");
        assert_eq!(token.as_str(), raw);
        assert_eq!(
            token.commitment_sha256(),
            "0f007385b6f9d4b7eeb2748605afe1a984a0a3bfa3f014d09e2a784ce9e5cd1a"
        );
        let debug = format!("{token:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains(raw));
        assert_eq!(
            InvitationToken::new("short").expect_err("short invitation token"),
            InvitationTokenError::InvalidLength
        );
    }

    #[test]
    fn shared_circuit_snapshots_are_strictly_bounded_and_well_formed() {
        let shared = shared_circuit();
        assert!(valid_shared_circuit(&shared, Some(shared.circuit_id)));
        assert!(valid_shared_circuit(&shared, None));
        assert!(!valid_shared_circuit(&shared, Some(Uuid::from_u128(99))));

        let mut invalid = shared_circuit();
        invalid.circuit_id = Uuid::nil();
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.title.push(' ');
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.title = "x".repeat(crate::validation::MAX_CIRCUIT_TITLE_CHARACTERS + 1);
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.id = Uuid::nil();
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.schema_version = 0;
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.content_digest_version = 3;
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.document = json!(["not", "an", "object"]);
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.document =
            json!({"payload": "x".repeat(crate::validation::MAX_REVISION_DOCUMENT_BYTES)});
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid
            .revision
            .artifact_ids
            .push(invalid.revision.artifact_ids[0]);
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.artifact_ids = (1..=crate::validation::MAX_REVISION_ARTIFACTS + 1)
            .map(|value| Uuid::from_u128(value as u128))
            .collect();
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.artifact_ids[0] = Uuid::nil();
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.content_sha256 = "0A".repeat(32);
        assert!(!valid_shared_circuit(&invalid, None));

        let mut invalid = shared_circuit();
        invalid.revision.created_at = "2026-07-19 00:00:00Z".to_owned();
        assert!(!valid_shared_circuit(&invalid, None));
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use time::format_description::well_known::Rfc3339;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    #[wasm_bindgen_test]
    fn browser_clock_rejects_stale_active_share_handoffs() {
        let now = current_time_utc().expect("browser clock");
        let mut share = CreatedCircuitShare {
            id: Uuid::from_u128(1),
            circuit_id: Uuid::from_u128(2),
            token: None,
            permission: SharePermission::View,
            revision_id: Uuid::from_u128(3),
            expires_at: Some(
                now.saturating_add(Duration::minutes(1))
                    .format(&Rfc3339)
                    .expect("fresh expiry"),
            ),
            sealed_at: now.format(&Rfc3339).expect("seal time"),
            revoked_at: None,
            created_at: now.format(&Rfc3339).expect("creation time"),
        };
        assert!(created_share_handoff_is_safe(&share));

        share.expires_at = Some(now.format(&Rfc3339).expect("stale expiry"));
        assert!(!created_share_handoff_is_safe(&share));
    }
}
