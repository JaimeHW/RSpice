use std::collections::HashSet;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rspice_cloud_contract::{
    Entitlement, IssueLicenseLeaseRequest, IssuedLicenseLease, LicenseJwkSet, LicenseLease,
    is_valid_issue_license_lease_request, is_valid_license_jwk_set,
};
use serde::Deserialize;
use serde_json::Value;

use crate::validation::{parse_timestamp_text, valid_json_object};

pub(crate) const MAX_PRODUCT_BYTES: usize = 120;
pub(crate) const MAX_PLAN_BYTES: usize = 120;
const MAX_SIGNING_KEY_ID_BYTES: usize = 200;
const MAX_REVOCATION_REASON_BYTES: usize = 120;
pub(crate) const MAX_ISSUER_BYTES: usize = 2_048;
pub(crate) const MAX_AUDIENCE_BYTES: usize = 256;
pub(crate) const MAX_FEATURES_BYTES: usize = 16 * 1024;
const MAX_COMPACT_LEASE_BYTES: usize = 64 * 1024;
const MAX_JWS_HEADER_BYTES: usize = 1_024;
const MAX_JWS_PAYLOAD_BYTES: usize = 32 * 1024;
const MIN_RSA_SIGNATURE_BYTES: usize = 256;
const MAX_RSA_SIGNATURE_BYTES: usize = 1_024;

pub(crate) fn valid_license_jwks(jwks: &LicenseJwkSet) -> bool {
    is_valid_license_jwk_set(jwks)
}

pub(crate) fn valid_entitlement(entitlement: &Entitlement) -> bool {
    if entitlement.id.is_nil()
        || (entitlement.principal_id.is_some() as u8) + (entitlement.workspace_id.is_some() as u8)
            != 1
        || entitlement.principal_id.is_some_and(|id| id.is_nil())
        || entitlement.workspace_id.is_some_and(|id| id.is_nil())
        || !valid_clean_text(&entitlement.product, MAX_PRODUCT_BYTES)
        || !valid_clean_text(&entitlement.plan, MAX_PLAN_BYTES)
        || !valid_json_object(&entitlement.features, MAX_FEATURES_BYTES)
    {
        return false;
    }

    let Some(valid_from) = parse_timestamp_text(&entitlement.valid_from) else {
        return false;
    };
    if parse_timestamp_text(&entitlement.created_at).is_none() {
        return false;
    }
    entitlement.valid_until.as_deref().is_none_or(|value| {
        parse_timestamp_text(value).is_some_and(|valid_until| valid_until > valid_from)
    })
}

pub(crate) fn valid_entitlement_list(entitlements: &[Entitlement]) -> bool {
    if entitlements
        .iter()
        .any(|entitlement| !valid_entitlement(entitlement))
        || entitlements
            .iter()
            .map(|entitlement| entitlement.id)
            .collect::<HashSet<_>>()
            .len()
            != entitlements.len()
    {
        return false;
    }

    entitlements.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].created_at)
            .zip(parse_timestamp_text(&pair[1].created_at))
            .is_some_and(|(newer, older)| (newer, pair[0].id) > (older, pair[1].id))
    })
}

pub(crate) fn valid_license_lease(lease: &LicenseLease) -> bool {
    if lease.id.is_nil()
        || lease.entitlement_id.is_nil()
        || lease.workspace_id.is_some_and(|id| id.is_nil())
        || !valid_clean_text(&lease.product, MAX_PRODUCT_BYTES)
        || !valid_clean_text(&lease.plan, MAX_PLAN_BYTES)
        || !valid_signing_key_id(&lease.signing_key_id)
        || (lease.revoked_at.is_none() != lease.revocation_reason.is_none())
        || !lease
            .revocation_reason
            .as_deref()
            .is_none_or(|reason| valid_clean_text(reason, MAX_REVOCATION_REASON_BYTES))
    {
        return false;
    }

    let Some(issued_at) = parse_timestamp_text(&lease.issued_at) else {
        return false;
    };
    let Some(expires_at) = parse_timestamp_text(&lease.expires_at) else {
        return false;
    };
    expires_at > issued_at
        && lease.revoked_at.as_deref().is_none_or(|value| {
            parse_timestamp_text(value).is_some_and(|revoked_at| revoked_at >= issued_at)
        })
}

pub(crate) fn valid_license_lease_list(leases: &[LicenseLease]) -> bool {
    if leases.iter().any(|lease| !valid_license_lease(lease))
        || leases
            .iter()
            .map(|lease| lease.id)
            .collect::<HashSet<_>>()
            .len()
            != leases.len()
    {
        return false;
    }

    leases.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].issued_at)
            .zip(parse_timestamp_text(&pair[1].issued_at))
            .is_some_and(|(newer, older)| (newer, pair[0].id) > (older, pair[1].id))
    })
}

pub(crate) fn issued_license_lease_matches_request(
    issued: &IssuedLicenseLease,
    request: &IssueLicenseLeaseRequest,
) -> bool {
    if !is_valid_issue_license_lease_request(request)
        || issued.request_id != request.request_id
        || !valid_license_lease(&issued.lease)
        || issued.lease.workspace_id != request.workspace_id
        || issued.lease.revoked_at.is_some()
        || issued.lease.revocation_reason.is_some()
    {
        return false;
    }

    let Some((header, claims)) = decode_compact_lease(&issued.lease_token) else {
        return false;
    };
    let Some(issued_at) = parse_timestamp_text(&issued.lease.issued_at) else {
        return false;
    };
    let Some(expires_at) = parse_timestamp_text(&issued.lease.expires_at) else {
        return false;
    };

    header.alg == "PS256"
        && header.typ == "JWT"
        && header.kid == issued.lease.signing_key_id
        && valid_clean_text(&claims.iss, MAX_ISSUER_BYTES)
        && valid_clean_text(&claims.aud, MAX_AUDIENCE_BYTES)
        && !claims.sub.is_nil()
        && claims.jti == issued.lease.id
        && claims.iat == issued_at.unix_timestamp()
        && claims.nbf == claims.iat
        && claims.exp == expires_at.unix_timestamp()
        && claims.exp > claims.iat
        && claims.lease_version == 1
        && claims.lease_id == issued.lease.id
        && claims.entitlement_id == issued.lease.entitlement_id
        && claims.workspace_id == issued.lease.workspace_id
        && claims.product == issued.lease.product
        && claims.plan == issued.lease.plan
        && valid_json_object(&claims.features, MAX_FEATURES_BYTES)
        && claims.device_fingerprint_sha256 == request.device_fingerprint_sha256
}

pub(crate) fn valid_clean_text(value: &str, maximum_bytes: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum_bytes
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

fn valid_signing_key_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_SIGNING_KEY_ID_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LicenseHeader {
    pub(crate) typ: String,
    pub(crate) alg: String,
    pub(crate) kid: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UnverifiedLicenseClaims {
    iss: String,
    aud: String,
    sub: rspice_cloud_contract::Uuid,
    jti: rspice_cloud_contract::Uuid,
    iat: i64,
    nbf: i64,
    exp: i64,
    lease_version: u8,
    lease_id: rspice_cloud_contract::Uuid,
    entitlement_id: rspice_cloud_contract::Uuid,
    workspace_id: Option<rspice_cloud_contract::Uuid>,
    product: String,
    plan: String,
    features: Value,
    device_fingerprint_sha256: String,
}

fn decode_compact_lease(value: &str) -> Option<(LicenseHeader, UnverifiedLicenseClaims)> {
    if value.is_empty() || value.len() > MAX_COMPACT_LEASE_BYTES {
        return None;
    }
    let mut segments = value.split('.');
    let header = segments.next()?;
    let payload = segments.next()?;
    let signature = segments.next()?;
    if segments.next().is_some() {
        return None;
    }

    let header = decode_base64url(header, MAX_JWS_HEADER_BYTES)?;
    let payload = decode_base64url(payload, MAX_JWS_PAYLOAD_BYTES)?;
    let signature = decode_base64url(signature, MAX_RSA_SIGNATURE_BYTES)?;
    if !(MIN_RSA_SIGNATURE_BYTES..=MAX_RSA_SIGNATURE_BYTES).contains(&signature.len()) {
        return None;
    }

    Some((
        serde_json::from_slice(&header).ok()?,
        serde_json::from_slice(&payload).ok()?,
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn decode_compact_lease_header(value: &str) -> Option<LicenseHeader> {
    decode_compact_lease(value).map(|(header, _)| header)
}

fn decode_base64url(value: &str, maximum_bytes: usize) -> Option<Vec<u8>> {
    if value.is_empty() || value.len() > maximum_bytes.saturating_mul(4).div_ceil(3) {
        return None;
    }
    let decoded = URL_SAFE_NO_PAD.decode(value).ok()?;
    (decoded.len() <= maximum_bytes && URL_SAFE_NO_PAD.encode(&decoded) == value).then_some(decoded)
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
    use rspice_cloud_contract::{
        Entitlement, EntitlementStatus, IssueLicenseLeaseRequest, IssuedLicenseLease, LicenseJwk,
        LicenseJwkSet, LicenseKeyAlgorithm, LicenseKeyType, LicenseKeyUse, LicenseLease, Uuid,
    };
    use serde_json::json;

    use super::*;

    fn lease() -> LicenseLease {
        LicenseLease {
            id: Uuid::from_u128(1),
            entitlement_id: Uuid::from_u128(2),
            workspace_id: Some(Uuid::from_u128(3)),
            product: "rspice".to_owned(),
            plan: "professional".to_owned(),
            issued_at: "2026-07-19T00:00:00Z".to_owned(),
            expires_at: "2026-07-20T00:00:00Z".to_owned(),
            revoked_at: None,
            revocation_reason: None,
            signing_key_id: "license-2026".to_owned(),
        }
    }

    fn entitlement() -> Entitlement {
        Entitlement {
            id: Uuid::from_u128(10),
            principal_id: Some(Uuid::from_u128(11)),
            workspace_id: None,
            product: "rspice".to_owned(),
            plan: "professional".to_owned(),
            status: EntitlementStatus::Active,
            features: json!({"native_license": true}),
            valid_from: "2026-07-01T00:00:00Z".to_owned(),
            valid_until: Some("2026-08-01T00:00:00Z".to_owned()),
            created_at: "2026-07-19T00:00:00Z".to_owned(),
        }
    }

    fn request() -> IssueLicenseLeaseRequest {
        IssueLicenseLeaseRequest {
            request_id: Uuid::from_u128(4),
            device_fingerprint_sha256: URL_SAFE_NO_PAD.encode([7_u8; 32]),
            workspace_id: Some(Uuid::from_u128(3)),
        }
    }

    fn compact_lease(lease: &LicenseLease, request: &IssueLicenseLeaseRequest) -> String {
        let issued_at = parse_timestamp_text(&lease.issued_at)
            .expect("issued timestamp")
            .unix_timestamp();
        let expires_at = parse_timestamp_text(&lease.expires_at)
            .expect("expiry timestamp")
            .unix_timestamp();
        let header = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&json!({
                "typ": "JWT",
                "alg": "PS256",
                "kid": lease.signing_key_id,
            }))
            .expect("header"),
        );
        let payload = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&json!({
                "iss": "https://licenses.rspice.test/",
                "aud": "rspice-native",
                "sub": Uuid::from_u128(5),
                "jti": lease.id,
                "iat": issued_at,
                "nbf": issued_at,
                "exp": expires_at,
                "lease_version": 1,
                "lease_id": lease.id,
                "entitlement_id": lease.entitlement_id,
                "workspace_id": lease.workspace_id,
                "product": lease.product,
                "plan": lease.plan,
                "features": {"native_license": true},
                "device_fingerprint_sha256": request.device_fingerprint_sha256,
            }))
            .expect("claims"),
        );
        let signature = URL_SAFE_NO_PAD.encode([9_u8; MIN_RSA_SIGNATURE_BYTES]);
        format!("{header}.{payload}.{signature}")
    }

    #[test]
    fn jwks_require_unique_strong_canonical_rsa_keys() {
        let key = LicenseJwk {
            kty: LicenseKeyType::Rsa,
            n: URL_SAFE_NO_PAD.encode(
                std::iter::once(0x80_u8)
                    .chain(std::iter::repeat_n(0x5a, 255))
                    .collect::<Vec<_>>(),
            ),
            e: "AQAB".to_owned(),
            kid: "license-2026".to_owned(),
            key_use: LicenseKeyUse::Signature,
            alg: LicenseKeyAlgorithm::Ps256,
        };
        assert!(valid_license_jwks(&LicenseJwkSet {
            keys: vec![key.clone()],
        }));
        assert!(!valid_license_jwks(&LicenseJwkSet {
            keys: vec![key.clone(), key],
        }));
    }

    #[test]
    fn entitlement_pages_are_scoped_unique_and_newest_first() {
        let newer = entitlement();
        let mut older = newer.clone();
        older.id = Uuid::from_u128(12);
        older.created_at = "2026-07-18T00:00:00Z".to_owned();
        assert!(valid_entitlement_list(&[newer.clone(), older.clone()]));
        assert!(!valid_entitlement_list(&[older, newer.clone()]));

        let mut invalid = newer;
        invalid.workspace_id = Some(Uuid::from_u128(13));
        assert!(!valid_entitlement(&invalid));
    }

    #[test]
    fn lease_pages_are_canonical_unique_and_newest_first() {
        let newer = lease();
        let mut older = newer.clone();
        older.id = Uuid::from_u128(6);
        older.issued_at = "2026-07-18T00:00:00Z".to_owned();
        assert!(valid_license_lease_list(&[newer.clone(), older.clone()]));
        assert!(!valid_license_lease_list(&[older, newer.clone()]));

        let mut invalid = newer;
        invalid.revocation_reason = Some("self_revoked".to_owned());
        assert!(!valid_license_lease(&invalid));
    }

    #[test]
    fn issued_leases_bind_request_metadata_and_compact_claims() {
        let request = request();
        let lease = lease();
        let mut issued = IssuedLicenseLease {
            request_id: request.request_id,
            lease_token: compact_lease(&lease, &request),
            lease,
        };
        assert!(issued_license_lease_matches_request(&issued, &request));

        issued.request_id = Uuid::from_u128(99);
        assert!(!issued_license_lease_matches_request(&issued, &request));
        issued.request_id = request.request_id;
        issued.lease.plan = "different".to_owned();
        assert!(!issued_license_lease_matches_request(&issued, &request));
    }
}
