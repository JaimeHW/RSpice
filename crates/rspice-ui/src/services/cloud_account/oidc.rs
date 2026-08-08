//! OIDC authorization-code + PKCE engine (native).
//!
//! The cloud client crate deliberately owns no identity flow, so this module
//! implements the parts the application is responsible for: provider
//! discovery, the S256 code challenge, the authorization URL, and the token
//! endpoint (exchange, refresh, revocation). It is provider-neutral — the
//! endpoints come from `/.well-known/openid-configuration`, with the
//! document's `issuer` matched byte-for-byte against release policy, the same
//! way the API matches `iss` — so the deployment's identity product remains a
//! hosting decision, not a client assumption.
//!
//! Tokens returned here go straight to the executor's in-memory state. They
//! are never logged, never put in a URL, and never rendered.

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::Digest as _;

/// Discovered provider endpoints, validated against release policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ProviderEndpoints {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    /// RFC 7009 revocation endpoint, when the provider publishes one.
    pub revocation_endpoint: Option<String>,
}

/// A PKCE verifier/challenge pair (RFC 7636, S256 only).
#[derive(Clone)]
pub(super) struct PkcePair {
    pub verifier: String,
    pub challenge: String,
}

/// Successful token-endpoint response. Refresh tokens are optional because a
/// refresh grant may rotate or omit them.
#[derive(Clone, serde::Deserialize)]
pub(super) struct TokenGrant {
    pub access_token: String,
    pub expires_in: u64,
    #[serde(default)]
    pub refresh_token: Option<String>,
    pub token_type: String,
}

impl std::fmt::Debug for TokenGrant {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TokenGrant")
            .field("access_token", &"[REDACTED]")
            .field("expires_in", &self.expires_in)
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("token_type", &self.token_type)
            .finish()
    }
}

/// Presentation-safe identity-flow failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum IdentityError {
    /// The provider could not be reached.
    Unreachable,
    /// The discovery document or a token response violated the contract.
    Contract(&'static str),
    /// The provider rejected the request (bad grant, revoked session, ...).
    Rejected,
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreachable => write!(formatter, "the sign-in service could not be reached"),
            Self::Contract(detail) => {
                write!(
                    formatter,
                    "the sign-in service response was invalid: {detail}"
                )
            }
            Self::Rejected => write!(formatter, "the sign-in service rejected the request"),
        }
    }
}

/// HTTP client for identity traffic: no redirects, bounded time.
pub(super) fn identity_http_client() -> Result<reqwest::Client, IdentityError> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .user_agent(concat!("rspice/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|_| IdentityError::Contract("client construction failed"))
}

/// Fetch and validate `/.well-known/openid-configuration` for `issuer`.
pub(super) async fn discover(
    http: &reqwest::Client,
    issuer: &str,
    development: bool,
) -> Result<ProviderEndpoints, IdentityError> {
    #[derive(serde::Deserialize)]
    struct DiscoveryDocument {
        issuer: String,
        authorization_endpoint: String,
        token_endpoint: String,
        #[serde(default)]
        revocation_endpoint: Option<String>,
        #[serde(default)]
        code_challenge_methods_supported: Option<Vec<String>>,
    }

    let url = format!(
        "{}/.well-known/openid-configuration",
        issuer.trim_end_matches('/')
    );
    let response = http
        .get(&url)
        .send()
        .await
        .map_err(|_| IdentityError::Unreachable)?;
    if !response.status().is_success() {
        return Err(IdentityError::Contract("discovery request failed"));
    }
    let document: DiscoveryDocument = response
        .json()
        .await
        .map_err(|_| IdentityError::Contract("discovery document was not valid JSON"))?;

    // The API matches `iss` byte-for-byte; hold discovery to the same bar so
    // a misconfigured provider fails here instead of at the first API call.
    if document.issuer != issuer {
        return Err(IdentityError::Contract("issuer mismatch"));
    }
    for endpoint in [
        document.authorization_endpoint.as_str(),
        document.token_endpoint.as_str(),
    ]
    .into_iter()
    .chain(document.revocation_endpoint.as_deref())
    {
        if !endpoint_admissible(endpoint, development) {
            return Err(IdentityError::Contract("endpoint origin not admissible"));
        }
    }
    if let Some(methods) = &document.code_challenge_methods_supported
        && !methods.iter().any(|method| method == "S256")
    {
        return Err(IdentityError::Contract(
            "provider does not support PKCE S256",
        ));
    }

    Ok(ProviderEndpoints {
        authorization_endpoint: document.authorization_endpoint,
        token_endpoint: document.token_endpoint,
        revocation_endpoint: document.revocation_endpoint,
    })
}

/// HTTPS everywhere in production; loopback plain HTTP only in development.
fn endpoint_admissible(endpoint: &str, development: bool) -> bool {
    let Ok(parsed) = url::Url::parse(endpoint) else {
        return false;
    };
    if parsed.username() != "" || parsed.password().is_some() {
        return false;
    }
    match parsed.scheme() {
        "https" => true,
        "http" if development => matches!(
            parsed.host_str(),
            Some("127.0.0.1") | Some("localhost") | Some("[::1]")
        ),
        _ => false,
    }
}

/// Random bytes from the operating system.
fn random_bytes<const N: usize>() -> Result<[u8; N], IdentityError> {
    let mut bytes = [0_u8; N];
    getrandom::fill(&mut bytes).map_err(|_| IdentityError::Contract("no entropy source"))?;
    Ok(bytes)
}

/// Fresh PKCE pair: 48 random bytes → 64-char verifier, S256 challenge.
pub(super) fn pkce_pair() -> Result<PkcePair, IdentityError> {
    let verifier = URL_SAFE_NO_PAD.encode(random_bytes::<48>()?);
    let challenge = URL_SAFE_NO_PAD.encode(sha2::Sha256::digest(verifier.as_bytes()));
    Ok(PkcePair {
        verifier,
        challenge,
    })
}

/// Fresh opaque `state` value binding the callback to this attempt.
pub(super) fn state_value() -> Result<String, IdentityError> {
    Ok(URL_SAFE_NO_PAD.encode(random_bytes::<32>()?))
}

/// The authorization URL the system browser opens.
pub(super) fn authorization_url(
    endpoints: &ProviderEndpoints,
    client_id: &str,
    redirect_uri: &str,
    state: &str,
    pkce: &PkcePair,
) -> Result<String, IdentityError> {
    let mut url = url::Url::parse(&endpoints.authorization_endpoint)
        .map_err(|_| IdentityError::Contract("authorization endpoint unparsable"))?;
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("scope", "openid profile email")
        .append_pair("state", state)
        .append_pair("code_challenge", &pkce.challenge)
        .append_pair("code_challenge_method", "S256");
    Ok(url.into())
}

/// Exchange an authorization code for tokens.
pub(super) async fn exchange_code(
    http: &reqwest::Client,
    endpoints: &ProviderEndpoints,
    client_id: &str,
    redirect_uri: &str,
    code: &str,
    verifier: &str,
) -> Result<TokenGrant, IdentityError> {
    token_request(
        http,
        &endpoints.token_endpoint,
        &[
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("redirect_uri", redirect_uri),
            ("code", code),
            ("code_verifier", verifier),
        ],
    )
    .await
}

/// Refresh the session with a stored refresh token.
pub(super) async fn refresh_grant(
    http: &reqwest::Client,
    endpoints: &ProviderEndpoints,
    client_id: &str,
    refresh_token: &str,
) -> Result<TokenGrant, IdentityError> {
    token_request(
        http,
        &endpoints.token_endpoint,
        &[
            ("grant_type", "refresh_token"),
            ("client_id", client_id),
            ("refresh_token", refresh_token),
        ],
    )
    .await
}

/// Best-effort RFC 7009 revocation of the refresh token at sign-out.
pub(super) async fn revoke_refresh_token(
    http: &reqwest::Client,
    endpoints: &ProviderEndpoints,
    client_id: &str,
    refresh_token: &str,
) -> Result<(), IdentityError> {
    let Some(revocation_endpoint) = endpoints.revocation_endpoint.as_deref() else {
        return Ok(());
    };
    let response = http
        .post(revocation_endpoint)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(form_body(&[
            ("client_id", client_id),
            ("token", refresh_token),
            ("token_type_hint", "refresh_token"),
        ]))
        .send()
        .await
        .map_err(|_| IdentityError::Unreachable)?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(IdentityError::Rejected)
    }
}

/// URL-encoded form body via the `url` crate; the pinned reqwest feature set
/// deliberately excludes its own form encoder.
fn form_body(pairs: &[(&str, &str)]) -> String {
    let mut serializer = url::form_urlencoded::Serializer::new(String::new());
    for (key, value) in pairs {
        serializer.append_pair(key, value);
    }
    serializer.finish()
}

async fn token_request(
    http: &reqwest::Client,
    token_endpoint: &str,
    form: &[(&str, &str)],
) -> Result<TokenGrant, IdentityError> {
    let response = http
        .post(token_endpoint)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(form_body(form))
        .send()
        .await
        .map_err(|_| IdentityError::Unreachable)?;
    let status = response.status();
    if status.is_client_error() {
        return Err(IdentityError::Rejected);
    }
    if !status.is_success() {
        return Err(IdentityError::Contract("token endpoint failure"));
    }
    let grant: TokenGrant = response
        .json()
        .await
        .map_err(|_| IdentityError::Contract("token response was not valid JSON"))?;
    if !grant.token_type.eq_ignore_ascii_case("bearer") {
        return Err(IdentityError::Contract("unsupported token type"));
    }
    if grant.expires_in == 0 || grant.access_token.is_empty() {
        return Err(IdentityError::Contract(
            "empty or non-expiring access token",
        ));
    }
    Ok(grant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pkce_challenge_is_s256_of_the_verifier() {
        let pair = pkce_pair().expect("entropy");
        assert_eq!(pair.verifier.len(), 64);
        let expected = URL_SAFE_NO_PAD.encode(sha2::Sha256::digest(pair.verifier.as_bytes()));
        assert_eq!(pair.challenge, expected);
    }

    #[test]
    fn authorization_url_carries_the_exact_oauth_parameters() {
        let endpoints = ProviderEndpoints {
            authorization_endpoint: "https://id.example/realms/rspice/protocol/openid-connect/auth"
                .to_owned(),
            token_endpoint: "https://id.example/token".to_owned(),
            revocation_endpoint: None,
        };
        let pkce = PkcePair {
            verifier: "v".repeat(64),
            challenge: "challenge-value".to_owned(),
        };
        let rendered = authorization_url(
            &endpoints,
            "rspice-desktop",
            "http://127.0.0.1:17872/oauth/callback",
            "state-value",
            &pkce,
        )
        .expect("well-formed URL");
        let parsed = url::Url::parse(&rendered).expect("parse");
        let pairs: std::collections::BTreeMap<_, _> = parsed.query_pairs().collect();
        assert_eq!(pairs["response_type"], "code");
        assert_eq!(pairs["client_id"], "rspice-desktop");
        assert_eq!(
            pairs["redirect_uri"],
            "http://127.0.0.1:17872/oauth/callback"
        );
        assert_eq!(pairs["scope"], "openid profile email");
        assert_eq!(pairs["state"], "state-value");
        assert_eq!(pairs["code_challenge"], "challenge-value");
        assert_eq!(pairs["code_challenge_method"], "S256");
    }

    #[test]
    fn endpoint_admission_is_https_only_in_production() {
        assert!(endpoint_admissible("https://id.rspice.app/auth", false));
        assert!(!endpoint_admissible("http://id.rspice.app/auth", false));
        assert!(!endpoint_admissible("http://127.0.0.1:8180/auth", false));
        assert!(endpoint_admissible("http://127.0.0.1:8180/auth", true));
        assert!(!endpoint_admissible("http://192.168.1.4:8180/auth", true));
        assert!(!endpoint_admissible(
            "https://user:pw@id.rspice.app/auth",
            false
        ));
    }

    #[test]
    fn token_grant_debug_never_prints_tokens() {
        let grant = TokenGrant {
            access_token: "secret-access".to_owned(),
            expires_in: 300,
            refresh_token: Some("secret-refresh".to_owned()),
            token_type: "Bearer".to_owned(),
        };
        let rendered = format!("{grant:?}");
        assert!(!rendered.contains("secret-access"));
        assert!(!rendered.contains("secret-refresh"));
        assert!(rendered.contains("[REDACTED]"));
    }
}
