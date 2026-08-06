use std::{collections::HashMap, error::Error, fmt, time::Duration};

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, errors::ErrorKind as JwtErrorKind};
use rspice_cloud_contract::{
    IssueLicenseLeaseRequest, IssuedLicenseLease, LicenseJwkSet, LicenseLease, Uuid,
};
use serde::Deserialize;
use serde_json::Value;
use url::Url;

use crate::{
    licensing::{
        MAX_AUDIENCE_BYTES, MAX_FEATURES_BYTES, MAX_ISSUER_BYTES, MAX_PLAN_BYTES,
        MAX_PRODUCT_BYTES, decode_compact_lease_header, issued_license_lease_matches_request,
        valid_clean_text, valid_license_jwks,
    },
    validation::valid_json_object,
};

const MAX_CLOCK_SKEW: Duration = Duration::from_secs(5 * 60);
const MAX_MINIMUM_REMAINING_VALIDITY: Duration = Duration::from_secs(30 * 24 * 60 * 60);
const MIN_LEASE_LIFETIME_SECONDS: i64 = 5 * 60;
const MAX_LEASE_LIFETIME_SECONDS: i64 = 30 * 24 * 60 * 60;

/// Stable classification for a native-license verification failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeLicenseVerificationFailure {
    /// The pinned issuer, audience, product, or clock policy was invalid.
    InvalidPolicy,
    /// The supplied public key set violated the RSpice PS256 key policy.
    InvalidKeySet,
    /// The token selected a key ID absent from the admitted key set.
    UnknownSigningKey,
    /// The PS256 signature was not valid for the selected public key.
    InvalidSignature,
    /// The authenticated claims violated the RSpice native-license contract.
    InvalidClaims,
    /// The token is not valid yet under the admitted clock-skew policy.
    NotYetValid,
    /// The token is expired or has less than the caller's required lifetime.
    Expired,
}

/// Secret-free error returned when native-license verification fails.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct NativeLicenseVerificationError {
    failure: NativeLicenseVerificationFailure,
}

impl NativeLicenseVerificationError {
    const fn new(failure: NativeLicenseVerificationFailure) -> Self {
        Self { failure }
    }

    /// Returns the stable verification failure category.
    pub const fn failure(&self) -> NativeLicenseVerificationFailure {
        self.failure
    }
}

impl fmt::Debug for NativeLicenseVerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeLicenseVerificationError")
            .field("failure", &self.failure)
            .finish()
    }
}

impl fmt::Display for NativeLicenseVerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "native license verification failed: {:?}",
            self.failure
        )
    }
}

impl Error for NativeLicenseVerificationError {}

/// A cryptographically authenticated, device-bound native license.
///
/// This object never retains the compact token or device fingerprint. Creating
/// it proves the PS256 signature, pinned issuer/audience/product, current time,
/// key ID, device binding, and complete RSpice lease-claim contract.
#[derive(Clone, Eq, PartialEq)]
pub struct VerifiedNativeLicense {
    principal_id: Uuid,
    lease_id: Uuid,
    entitlement_id: Uuid,
    workspace_id: Option<Uuid>,
    product: String,
    plan: String,
    features: Value,
    issued_at_unix_seconds: i64,
    expires_at_unix_seconds: i64,
    signing_key_id: String,
}

impl VerifiedNativeLicense {
    /// Returns the authenticated principal subject.
    pub const fn principal_id(&self) -> Uuid {
        self.principal_id
    }

    /// Returns the authenticated public lease ID.
    pub const fn lease_id(&self) -> Uuid {
        self.lease_id
    }

    /// Returns the authenticated entitlement ID.
    pub const fn entitlement_id(&self) -> Uuid {
        self.entitlement_id
    }

    /// Returns the authenticated workspace scope, or `None` for a personal lease.
    pub const fn workspace_id(&self) -> Option<Uuid> {
        self.workspace_id
    }

    /// Returns the pinned product identifier.
    pub fn product(&self) -> &str {
        &self.product
    }

    /// Returns the authenticated commercial plan identifier.
    pub fn plan(&self) -> &str {
        &self.plan
    }

    /// Returns the authenticated feature policy object.
    pub const fn features(&self) -> &Value {
        &self.features
    }

    /// Returns an authenticated Boolean feature grant when one is present.
    pub fn feature_enabled(&self, feature: &str) -> Option<bool> {
        self.features.get(feature).and_then(Value::as_bool)
    }

    /// Returns the authenticated issue time as Unix seconds.
    pub const fn issued_at_unix_seconds(&self) -> i64 {
        self.issued_at_unix_seconds
    }

    /// Returns the authenticated expiry time as Unix seconds.
    pub const fn expires_at_unix_seconds(&self) -> i64 {
        self.expires_at_unix_seconds
    }

    /// Returns the public signing-key identifier used for verification.
    pub fn signing_key_id(&self) -> &str {
        &self.signing_key_id
    }
}

impl fmt::Debug for VerifiedNativeLicense {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifiedNativeLicense")
            .field("principal_id", &self.principal_id)
            .field("lease_id", &self.lease_id)
            .field("entitlement_id", &self.entitlement_id)
            .field("workspace_id", &self.workspace_id)
            .field("product", &self.product)
            .field("plan", &self.plan)
            .field("features", &"[REDACTED]")
            .field("issued_at_unix_seconds", &self.issued_at_unix_seconds)
            .field("expires_at_unix_seconds", &self.expires_at_unix_seconds)
            .field("signing_key_id", &self.signing_key_id)
            .finish()
    }
}

/// Reusable verifier for native desktop and mobile offline license leases.
///
/// Construct this from a release-pinned issuer, audience, product, allowed
/// clock skew, and a JWKS fetched through [`crate::CloudClient::get_license_jwks`]
/// or loaded from protected cached state. Reconstruct it after an admitted key
/// rotation; it never performs network requests itself.
pub struct NativeLicenseVerifier {
    issuer: String,
    audience: String,
    product: String,
    allowed_clock_skew_seconds: u64,
    keys: HashMap<String, DecodingKey>,
}

impl fmt::Debug for NativeLicenseVerifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeLicenseVerifier")
            .field("issuer", &self.issuer)
            .field("audience", &self.audience)
            .field("product", &self.product)
            .field(
                "allowed_clock_skew_seconds",
                &self.allowed_clock_skew_seconds,
            )
            .field("key_count", &self.keys.len())
            .finish()
    }
}

impl NativeLicenseVerifier {
    /// Admits one exact verification policy and bounded canonical PS256 key set.
    pub fn new(
        jwks: &LicenseJwkSet,
        issuer: &str,
        audience: &str,
        product: &str,
        allowed_clock_skew: Duration,
    ) -> Result<Self, NativeLicenseVerificationError> {
        if !valid_verifier_policy(issuer, audience, product, allowed_clock_skew) {
            return Err(NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::InvalidPolicy,
            ));
        }
        if !valid_license_jwks(jwks) {
            return Err(NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::InvalidKeySet,
            ));
        }

        let mut keys = HashMap::with_capacity(jwks.keys.len());
        for key in &jwks.keys {
            let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e).map_err(|_| {
                NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::InvalidKeySet)
            })?;
            if keys.insert(key.kid.clone(), decoding_key).is_some() {
                return Err(NativeLicenseVerificationError::new(
                    NativeLicenseVerificationFailure::InvalidKeySet,
                ));
            }
        }

        Ok(Self {
            issuer: issuer.to_owned(),
            audience: audience.to_owned(),
            product: product.to_owned(),
            allowed_clock_skew_seconds: allowed_clock_skew.as_secs(),
            keys,
        })
    }

    /// Verifies a newly issued response and its exact caller-owned request.
    ///
    /// `minimum_remaining_validity` lets the application require enough time
    /// to persist and begin using the lease. It is capped at the server's
    /// maximum 30-day lease lifetime. The method retains neither credential.
    pub fn verify_issued(
        &self,
        issued: &IssuedLicenseLease,
        request: &IssueLicenseLeaseRequest,
        minimum_remaining_validity: Duration,
    ) -> Result<VerifiedNativeLicense, NativeLicenseVerificationError> {
        if !issued_license_lease_matches_request(issued, request) {
            return Err(NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::InvalidClaims,
            ));
        }
        let verified = self.verify_token(
            &issued.lease_token,
            &request.device_fingerprint_sha256,
            minimum_remaining_validity,
        )?;
        if !verified_matches_lease(&verified, &issued.lease) {
            return Err(NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::InvalidClaims,
            ));
        }
        Ok(verified)
    }

    /// Verifies a compact token loaded from protected native storage.
    ///
    /// Call this on every startup and before enabling offline licensed
    /// functionality. The expected device digest must be recomputed from the
    /// same stable, privacy-reviewed binding material used for issuance.
    pub fn verify_token(
        &self,
        token: &str,
        expected_device_fingerprint_sha256: &str,
        minimum_remaining_validity: Duration,
    ) -> Result<VerifiedNativeLicense, NativeLicenseVerificationError> {
        if minimum_remaining_validity > MAX_MINIMUM_REMAINING_VALIDITY
            || !valid_device_fingerprint(expected_device_fingerprint_sha256)
        {
            return Err(NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::InvalidPolicy,
            ));
        }

        let Some(unverified_header) = decode_compact_lease_header(token) else {
            return Err(NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::InvalidClaims,
            ));
        };
        if unverified_header.alg != "PS256" || unverified_header.typ != "JWT" {
            return Err(NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::InvalidClaims,
            ));
        }
        let key = self.keys.get(&unverified_header.kid).ok_or_else(|| {
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::UnknownSigningKey)
        })?;

        let mut validation = Validation::new(Algorithm::PS256);
        validation.validate_nbf = true;
        validation.leeway = self.allowed_clock_skew_seconds;
        validation.set_required_spec_claims(&["aud", "exp", "iss", "nbf", "sub"]);
        validation.set_audience(&[self.audience.as_str()]);
        validation.set_issuer(&[self.issuer.as_str()]);
        let decoded = decode::<NativeLicenseClaims>(token, key, &validation)
            .map_err(map_jwt_verification_error)?;

        if decoded.header.alg != Algorithm::PS256
            || decoded.header.typ.as_deref() != Some("JWT")
            || decoded.header.kid.as_deref() != Some(unverified_header.kid.as_str())
            || !valid_authenticated_claims(
                &decoded.claims,
                &self.product,
                expected_device_fingerprint_sha256,
            )
        {
            return Err(NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::InvalidClaims,
            ));
        }
        if !minimum_remaining_validity.is_zero() {
            let required_seconds =
                i64::try_from(minimum_remaining_validity.as_secs()).map_err(|_| {
                    NativeLicenseVerificationError::new(
                        NativeLicenseVerificationFailure::InvalidPolicy,
                    )
                })?;
            let required_until = time::OffsetDateTime::now_utc()
                .unix_timestamp()
                .saturating_add(required_seconds);
            if decoded.claims.exp < required_until {
                return Err(NativeLicenseVerificationError::new(
                    NativeLicenseVerificationFailure::Expired,
                ));
            }
        }

        Ok(VerifiedNativeLicense {
            principal_id: decoded.claims.sub,
            lease_id: decoded.claims.lease_id,
            entitlement_id: decoded.claims.entitlement_id,
            workspace_id: decoded.claims.workspace_id,
            product: decoded.claims.product,
            plan: decoded.claims.plan,
            features: decoded.claims.features,
            issued_at_unix_seconds: decoded.claims.iat,
            expires_at_unix_seconds: decoded.claims.exp,
            signing_key_id: unverified_header.kid,
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeLicenseClaims {
    iss: String,
    aud: String,
    sub: Uuid,
    jti: Uuid,
    iat: i64,
    nbf: i64,
    exp: i64,
    lease_version: u8,
    lease_id: Uuid,
    entitlement_id: Uuid,
    workspace_id: Option<Uuid>,
    product: String,
    plan: String,
    features: Value,
    device_fingerprint_sha256: String,
}

fn valid_verifier_policy(
    issuer: &str,
    audience: &str,
    product: &str,
    allowed_clock_skew: Duration,
) -> bool {
    let issuer_url = Url::parse(issuer).ok();
    issuer_url.is_some_and(|url| {
        !url.cannot_be_a_base()
            && url.username().is_empty()
            && url.password().is_none()
            && url.host_str().is_some()
            && url.query().is_none()
            && url.fragment().is_none()
            && matches!(url.scheme(), "http" | "https")
            && url.as_str() == issuer
    }) && valid_clean_text(issuer, MAX_ISSUER_BYTES)
        && valid_clean_text(audience, MAX_AUDIENCE_BYTES)
        && valid_identifier(product, MAX_PRODUCT_BYTES)
        && allowed_clock_skew <= MAX_CLOCK_SKEW
}

fn valid_identifier(value: &str, maximum_bytes: usize) -> bool {
    valid_clean_text(value, maximum_bytes)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn valid_device_fingerprint(value: &str) -> bool {
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

    URL_SAFE_NO_PAD
        .decode(value)
        .is_ok_and(|decoded| decoded.len() == 32 && URL_SAFE_NO_PAD.encode(decoded) == value)
}

fn valid_authenticated_claims(
    claims: &NativeLicenseClaims,
    expected_product: &str,
    expected_device_fingerprint_sha256: &str,
) -> bool {
    !claims.sub.is_nil()
        && !claims.lease_id.is_nil()
        && claims.jti == claims.lease_id
        && !claims.entitlement_id.is_nil()
        && claims.workspace_id.is_none_or(|id| !id.is_nil())
        && claims.iat == claims.nbf
        && (MIN_LEASE_LIFETIME_SECONDS..=MAX_LEASE_LIFETIME_SECONDS)
            .contains(&claims.exp.saturating_sub(claims.iat))
        && claims.lease_version == 1
        && claims.product == expected_product
        && valid_identifier(&claims.product, MAX_PRODUCT_BYTES)
        && valid_clean_text(&claims.plan, MAX_PLAN_BYTES)
        && valid_json_object(&claims.features, MAX_FEATURES_BYTES)
        && claims.device_fingerprint_sha256 == expected_device_fingerprint_sha256
        && valid_device_fingerprint(&claims.device_fingerprint_sha256)
        && valid_clean_text(&claims.iss, MAX_ISSUER_BYTES)
        && valid_clean_text(&claims.aud, MAX_AUDIENCE_BYTES)
}

fn verified_matches_lease(verified: &VerifiedNativeLicense, lease: &LicenseLease) -> bool {
    verified.lease_id == lease.id
        && verified.entitlement_id == lease.entitlement_id
        && verified.workspace_id == lease.workspace_id
        && verified.product == lease.product
        && verified.plan == lease.plan
        && verified.issued_at_unix_seconds
            == crate::validation::parse_timestamp_text(&lease.issued_at)
                .map(|value| value.unix_timestamp())
                .unwrap_or(i64::MIN)
        && verified.expires_at_unix_seconds
            == crate::validation::parse_timestamp_text(&lease.expires_at)
                .map(|value| value.unix_timestamp())
                .unwrap_or(i64::MIN)
        && verified.signing_key_id == lease.signing_key_id
}

fn map_jwt_verification_error(
    error: jsonwebtoken::errors::Error,
) -> NativeLicenseVerificationError {
    let failure = match error.kind() {
        JwtErrorKind::InvalidSignature => NativeLicenseVerificationFailure::InvalidSignature,
        JwtErrorKind::ExpiredSignature => NativeLicenseVerificationFailure::Expired,
        JwtErrorKind::ImmatureSignature => NativeLicenseVerificationFailure::NotYetValid,
        _ => NativeLicenseVerificationFailure::InvalidClaims,
    };
    NativeLicenseVerificationError::new(failure)
}

#[cfg(test)]
mod tests {
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
    use jsonwebtoken::{EncodingKey, Header, encode};
    use rspice_cloud_contract::LicenseLease;
    use serde::Serialize;
    use serde_json::json;
    use time::{OffsetDateTime, format_description::well_known::Rfc3339};

    use super::*;

    const ISSUER: &str = "https://licenses.rspice.test/";
    const AUDIENCE: &str = "rspice-native";
    const PRODUCT: &str = "rspice";
    const KEY_ID: &str = "test-license-key";
    const PRIVATE_KEY: &[u8] = include_bytes!("../../../testdata/license-test-private-key.pem");
    const PUBLIC_JWKS: &str = include_str!("../../../testdata/license-test-public-jwks.json");

    #[derive(Serialize)]
    struct TestClaims<'a> {
        iss: &'a str,
        aud: &'a str,
        sub: Uuid,
        jti: Uuid,
        iat: i64,
        nbf: i64,
        exp: i64,
        lease_version: u8,
        lease_id: Uuid,
        entitlement_id: Uuid,
        workspace_id: Option<Uuid>,
        product: &'a str,
        plan: &'a str,
        features: Value,
        device_fingerprint_sha256: &'a str,
    }

    fn jwks() -> LicenseJwkSet {
        serde_json::from_str(PUBLIC_JWKS).expect("test public JWKS")
    }

    fn verifier(issuer: &str) -> NativeLicenseVerifier {
        NativeLicenseVerifier::new(&jwks(), issuer, AUDIENCE, PRODUCT, Duration::from_secs(30))
            .expect("test verifier")
    }

    fn issued_lease(expiry_offset_seconds: i64) -> (IssuedLicenseLease, IssueLicenseLeaseRequest) {
        issued_lease_with_policy(-600, expiry_offset_seconds, KEY_ID)
    }

    fn issued_lease_with_policy(
        issue_offset_seconds: i64,
        expiry_offset_seconds: i64,
        signing_key_id: &str,
    ) -> (IssuedLicenseLease, IssueLicenseLeaseRequest) {
        let now = OffsetDateTime::now_utc();
        let issued_at = now.saturating_add(time::Duration::seconds(issue_offset_seconds));
        let expires_at = now.saturating_add(time::Duration::seconds(expiry_offset_seconds));
        let request = IssueLicenseLeaseRequest {
            request_id: Uuid::from_u128(1),
            device_fingerprint_sha256: URL_SAFE_NO_PAD.encode([7_u8; 32]),
            workspace_id: Some(Uuid::from_u128(2)),
        };
        let lease_id = Uuid::from_u128(3);
        let entitlement_id = Uuid::from_u128(4);
        let claims = TestClaims {
            iss: ISSUER,
            aud: AUDIENCE,
            sub: Uuid::from_u128(5),
            jti: lease_id,
            iat: issued_at.unix_timestamp(),
            nbf: issued_at.unix_timestamp(),
            exp: expires_at.unix_timestamp(),
            lease_version: 1,
            lease_id,
            entitlement_id,
            workspace_id: request.workspace_id,
            product: PRODUCT,
            plan: "professional",
            features: json!({"native_license": true, "remote_simulation": true}),
            device_fingerprint_sha256: &request.device_fingerprint_sha256,
        };
        let mut header = Header::new(Algorithm::PS256);
        header.kid = Some(signing_key_id.to_owned());
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(PRIVATE_KEY).expect("test RSA key"),
        )
        .expect("signed test lease");
        let lease = LicenseLease {
            id: lease_id,
            entitlement_id,
            workspace_id: request.workspace_id,
            product: PRODUCT.to_owned(),
            plan: "professional".to_owned(),
            issued_at: issued_at.format(&Rfc3339).expect("issue timestamp"),
            expires_at: expires_at.format(&Rfc3339).expect("expiry timestamp"),
            revoked_at: None,
            revocation_reason: None,
            signing_key_id: signing_key_id.to_owned(),
        };
        (
            IssuedLicenseLease {
                lease,
                request_id: request.request_id,
                lease_token: token,
            },
            request,
        )
    }

    #[test]
    fn verifies_ps256_and_returns_only_authenticated_authority() {
        let (issued, request) = issued_lease(600);
        let verified = verifier(ISSUER)
            .verify_issued(&issued, &request, Duration::from_secs(60))
            .expect("verified native license");
        assert_eq!(verified.principal_id(), Uuid::from_u128(5));
        assert_eq!(verified.lease_id(), issued.lease.id);
        assert_eq!(verified.entitlement_id(), issued.lease.entitlement_id);
        assert_eq!(verified.workspace_id(), request.workspace_id);
        assert_eq!(verified.product(), PRODUCT);
        assert_eq!(verified.plan(), "professional");
        assert_eq!(verified.feature_enabled("native_license"), Some(true));
        assert_eq!(verified.signing_key_id(), KEY_ID);
        let debug = format!("{verified:?}");
        assert!(!debug.contains(&issued.lease_token));
        assert!(!debug.contains(&request.device_fingerprint_sha256));
        assert!(!debug.contains("remote_simulation"));
    }

    #[test]
    fn rejects_signature_policy_device_time_and_metadata_drift() {
        let (issued, request) = issued_lease(600);

        let mut tampered = issued.clone();
        let signature_start = tampered
            .lease_token
            .rfind('.')
            .expect("signature separator")
            + 1;
        let replacement = if tampered.lease_token.as_bytes()[signature_start] == b'A' {
            "B"
        } else {
            "A"
        };
        tampered
            .lease_token
            .replace_range(signature_start..signature_start + 1, replacement);
        assert_eq!(
            verifier(ISSUER)
                .verify_issued(&tampered, &request, Duration::ZERO)
                .expect_err("tampered signature"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::InvalidSignature)
        );

        let mut wrong_device = request.clone();
        wrong_device.device_fingerprint_sha256 = URL_SAFE_NO_PAD.encode([8_u8; 32]);
        assert_eq!(
            verifier(ISSUER)
                .verify_token(
                    &issued.lease_token,
                    &wrong_device.device_fingerprint_sha256,
                    Duration::ZERO
                )
                .expect_err("wrong device"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::InvalidClaims)
        );
        assert_eq!(
            verifier("https://other.rspice.test/")
                .verify_issued(&issued, &request, Duration::ZERO)
                .expect_err("wrong issuer"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::InvalidClaims)
        );

        let (short, short_request) = issued_lease(30);
        assert_eq!(
            verifier(ISSUER)
                .verify_issued(&short, &short_request, Duration::from_secs(60))
                .expect_err("insufficient remaining lifetime"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::Expired)
        );

        let (expired, expired_request) = issued_lease(-120);
        assert_eq!(
            verifier(ISSUER)
                .verify_issued(&expired, &expired_request, Duration::ZERO)
                .expect_err("expired token"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::Expired)
        );

        let (future, future_request) = issued_lease_with_policy(120, 600, KEY_ID);
        assert_eq!(
            verifier(ISSUER)
                .verify_issued(&future, &future_request, Duration::ZERO)
                .expect_err("not-yet-valid token"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::NotYetValid)
        );

        let (unknown_key, unknown_key_request) =
            issued_lease_with_policy(-600, 600, "unknown-test-key");
        assert_eq!(
            verifier(ISSUER)
                .verify_issued(&unknown_key, &unknown_key_request, Duration::ZERO)
                .expect_err("unknown signing key"),
            NativeLicenseVerificationError::new(
                NativeLicenseVerificationFailure::UnknownSigningKey
            )
        );

        assert_eq!(
            verifier(ISSUER)
                .verify_issued(
                    &issued,
                    &request,
                    Duration::from_secs(30 * 24 * 60 * 60 + 1),
                )
                .expect_err("excessive minimum lifetime"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::InvalidPolicy)
        );

        let (undersized_lifetime, undersized_lifetime_request) =
            issued_lease_with_policy(-1, 30, KEY_ID);
        assert_eq!(
            verifier(ISSUER)
                .verify_issued(
                    &undersized_lifetime,
                    &undersized_lifetime_request,
                    Duration::ZERO,
                )
                .expect_err("undersized original lifetime"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::InvalidClaims)
        );

        let mut metadata_drift = issued.clone();
        metadata_drift.lease.plan = "enterprise".to_owned();
        assert_eq!(
            verifier(ISSUER)
                .verify_issued(&metadata_drift, &request, Duration::ZERO)
                .expect_err("metadata drift"),
            NativeLicenseVerificationError::new(NativeLicenseVerificationFailure::InvalidClaims)
        );
    }

    #[test]
    fn verifier_configuration_and_keys_fail_closed() {
        assert_eq!(
            NativeLicenseVerifier::new(&jwks(), "not-a-url", AUDIENCE, PRODUCT, Duration::ZERO,)
                .expect_err("invalid issuer")
                .failure(),
            NativeLicenseVerificationFailure::InvalidPolicy
        );
        assert_eq!(
            NativeLicenseVerifier::new(
                &jwks(),
                ISSUER,
                AUDIENCE,
                PRODUCT,
                Duration::from_secs(301),
            )
            .expect_err("excessive clock skew")
            .failure(),
            NativeLicenseVerificationFailure::InvalidPolicy
        );
        let mut weak = jwks();
        weak.keys[0].n = URL_SAFE_NO_PAD.encode([0x80_u8; 128]);
        assert_eq!(
            NativeLicenseVerifier::new(&weak, ISSUER, AUDIENCE, PRODUCT, Duration::ZERO)
                .expect_err("weak key")
                .failure(),
            NativeLicenseVerificationFailure::InvalidKeySet
        );
    }
}
