//! Cross-platform OIDC authorization-code + PKCE engine.
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

const MAX_DISCOVERY_BYTES: usize = 64 * 1024;
const MAX_TOKEN_RESPONSE_BYTES: usize = 32 * 1024;
const MAX_TOKEN_BYTES: usize = 16 * 1024;
#[cfg(any(target_arch = "wasm32", test))]
const MAX_CALLBACK_PARAMETERS_BYTES: usize = 8 * 1024;
const MIN_ACCESS_TOKEN_LIFETIME_SECONDS: u64 = 10;
const MAX_ACCESS_TOKEN_LIFETIME_SECONDS: u64 = 24 * 60 * 60;

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

/// Bounded classification of parameters on the registered browser callback.
/// Unknown route parameters are ignored; known OAuth parameters make the URL
/// credential-bearing and therefore subject to immediate history cleanup.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Default, Eq, PartialEq)]
pub(super) struct AuthorizationCallbackParameters {
    pub code: Option<String>,
    pub state: Option<String>,
    pub has_error: bool,
    pub has_oauth_parameter: bool,
    pub has_duplicate: bool,
}

#[cfg(any(target_arch = "wasm32", test))]
pub(super) fn parse_authorization_callback_parameters(
    encoded: Option<&str>,
) -> Result<AuthorizationCallbackParameters, ()> {
    let Some(encoded) = encoded else {
        return Ok(AuthorizationCallbackParameters::default());
    };
    if encoded.len() > MAX_CALLBACK_PARAMETERS_BYTES {
        return Err(());
    }
    let mut parsed = AuthorizationCallbackParameters::default();
    let mut error_seen = false;
    for (key, value) in url::form_urlencoded::parse(encoded.as_bytes()) {
        match key.as_ref() {
            "code" => {
                parsed.has_oauth_parameter = true;
                if parsed.code.replace(value.into_owned()).is_some() {
                    parsed.has_duplicate = true;
                }
            }
            "state" => {
                parsed.has_oauth_parameter = true;
                if parsed.state.replace(value.into_owned()).is_some() {
                    parsed.has_duplicate = true;
                }
            }
            "error" => {
                parsed.has_oauth_parameter = true;
                parsed.has_error = true;
                if error_seen {
                    parsed.has_duplicate = true;
                }
                error_seen = true;
            }
            _ => {}
        }
    }
    Ok(parsed)
}

#[cfg(any(target_arch = "wasm32", test))]
pub(super) fn is_exact_browser_callback_route(current: &url::Url, redirect: &url::Url) -> bool {
    current.scheme() == redirect.scheme()
        && current.host_str() == redirect.host_str()
        && current.port_or_known_default() == redirect.port_or_known_default()
        && current.path() == redirect.path()
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

/// The transport the identity calls below are handed.
///
/// Native builds carry a configured `reqwest::Client`. Browser builds issue
/// every identity request through [`browser_request`] instead, because
/// reqwest's wasm backend cannot express `redirect: error` and this flow has
/// to reject a redirect before a credential can be forwarded — so on that
/// target there is no client to carry, and the type says so. Writing
/// `reqwest::Client` into these shared signatures instead meant the browser
/// built a client that every wasm arm went on to ignore, and that could fail
/// the sign-in with "client construction failed" for a client nothing would
/// have used.
#[cfg(not(target_arch = "wasm32"))]
pub(super) type IdentityHttpClient = reqwest::Client;
#[cfg(target_arch = "wasm32")]
pub(super) type IdentityHttpClient = BrowserIdentityTransport;

/// The browser's identity transport, which holds nothing: the Fetch options
/// this flow requires are set per request, not per client.
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug)]
pub(super) struct BrowserIdentityTransport;

/// HTTP client for identity traffic: no redirects, bounded time.
pub(super) fn identity_http_client() -> Result<IdentityHttpClient, IdentityError> {
    #[cfg(not(target_arch = "wasm32"))]
    return reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .user_agent(concat!("rspice/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|_| IdentityError::Contract("client construction failed"));
    #[cfg(target_arch = "wasm32")]
    return Ok(BrowserIdentityTransport);
}

/// Fetch and validate `/.well-known/openid-configuration` for `issuer`.
pub(super) async fn discover(
    http: &IdentityHttpClient,
    issuer: &str,
    development: bool,
) -> Result<ProviderEndpoints, IdentityError> {
    #[cfg(target_arch = "wasm32")]
    let _ = http;
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
    #[cfg(not(target_arch = "wasm32"))]
    let document: DiscoveryDocument = {
        let response = http
            .get(&url)
            .send()
            .await
            .map_err(|_| IdentityError::Unreachable)?;
        if !response.status().is_success() {
            return Err(IdentityError::Contract("discovery request failed"));
        }
        require_json_content_type(
            response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
        )?;
        let body = bounded_native_body(response, MAX_DISCOVERY_BYTES).await?;
        serde_json::from_slice(&body)
            .map_err(|_| IdentityError::Contract("discovery document was not valid JSON"))?
    };
    #[cfg(target_arch = "wasm32")]
    let document: DiscoveryDocument = {
        let (status, body, content_type) =
            browser_request("GET", &url, None, MAX_DISCOVERY_BYTES).await?;
        if !(200..300).contains(&status) {
            return Err(IdentityError::Contract("discovery request failed"));
        }
        require_json_content_type(content_type.as_deref())?;
        serde_json::from_slice(&body)
            .map_err(|_| IdentityError::Contract("discovery document was not valid JSON"))?
    };

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
    #[cfg(not(target_arch = "wasm32"))]
    getrandom::fill(&mut bytes).map_err(|_| IdentityError::Contract("no entropy source"))?;
    #[cfg(target_arch = "wasm32")]
    web_sys::window()
        .ok_or(IdentityError::Contract("no browser window"))?
        .crypto()
        .map_err(|_| IdentityError::Contract("no browser entropy source"))?
        .get_random_values_with_u8_array(&mut bytes)
        .map_err(|_| IdentityError::Contract("no browser entropy source"))?;
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
    http: &IdentityHttpClient,
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
    http: &IdentityHttpClient,
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
    http: &IdentityHttpClient,
    endpoints: &ProviderEndpoints,
    client_id: &str,
    refresh_token: &str,
) -> Result<(), IdentityError> {
    #[cfg(target_arch = "wasm32")]
    let _ = http;
    let Some(revocation_endpoint) = endpoints.revocation_endpoint.as_deref() else {
        return Ok(());
    };
    let body = form_body(&[
        ("client_id", client_id),
        ("token", refresh_token),
        ("token_type_hint", "refresh_token"),
    ]);
    #[cfg(not(target_arch = "wasm32"))]
    let success = http
        .post(revocation_endpoint)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .map_err(|_| IdentityError::Unreachable)?
        .status()
        .is_success();
    #[cfg(target_arch = "wasm32")]
    let success = {
        let (status, _, _) =
            browser_request("POST", revocation_endpoint, Some(&body), 4096).await?;
        (200..300).contains(&status)
    };
    if success {
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
    http: &IdentityHttpClient,
    token_endpoint: &str,
    form: &[(&str, &str)],
) -> Result<TokenGrant, IdentityError> {
    #[cfg(target_arch = "wasm32")]
    let _ = http;
    let form = form_body(form);
    #[cfg(not(target_arch = "wasm32"))]
    let grant: TokenGrant = {
        let response = http
            .post(token_endpoint)
            .header("content-type", "application/x-www-form-urlencoded")
            .body(form)
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
        require_json_content_type(
            response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok()),
        )?;
        let body = bounded_native_body(response, MAX_TOKEN_RESPONSE_BYTES).await?;
        serde_json::from_slice(&body)
            .map_err(|_| IdentityError::Contract("token response was not valid JSON"))?
    };
    #[cfg(target_arch = "wasm32")]
    let grant: TokenGrant = {
        let (status, body, content_type) = browser_request(
            "POST",
            token_endpoint,
            Some(&form),
            MAX_TOKEN_RESPONSE_BYTES,
        )
        .await?;
        if (400..500).contains(&status) {
            return Err(IdentityError::Rejected);
        }
        if !(200..300).contains(&status) {
            return Err(IdentityError::Contract("token endpoint failure"));
        }
        require_json_content_type(content_type.as_deref())?;
        serde_json::from_slice(&body)
            .map_err(|_| IdentityError::Contract("token response was not valid JSON"))?
    };
    if !grant.token_type.eq_ignore_ascii_case("bearer") {
        return Err(IdentityError::Contract("unsupported token type"));
    }
    if !(MIN_ACCESS_TOKEN_LIFETIME_SECONDS..=MAX_ACCESS_TOKEN_LIFETIME_SECONDS)
        .contains(&grant.expires_in)
        || grant.access_token.is_empty()
        || grant.access_token.len() > MAX_TOKEN_BYTES
        || grant
            .refresh_token
            .as_ref()
            .is_some_and(|token| token.is_empty() || token.len() > MAX_TOKEN_BYTES)
    {
        return Err(IdentityError::Contract(
            "token size or lifetime outside policy",
        ));
    }
    Ok(grant)
}

fn require_json_content_type(content_type: Option<&str>) -> Result<(), IdentityError> {
    let Some(essence) = content_type
        .and_then(|value| value.split(';').next())
        .map(str::trim)
        .map(str::to_ascii_lowercase)
    else {
        return Err(IdentityError::Contract("JSON content type missing"));
    };
    if essence == "application/json" || essence.ends_with("+json") {
        Ok(())
    } else {
        Err(IdentityError::Contract("JSON content type required"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn bounded_native_body(
    mut response: reqwest::Response,
    max_response_bytes: usize,
) -> Result<Vec<u8>, IdentityError> {
    if response
        .content_length()
        .is_some_and(|length| length > u64::try_from(max_response_bytes).expect("usize fits u64"))
    {
        return Err(IdentityError::Contract("identity response too large"));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| IdentityError::Unreachable)?
    {
        let next = bytes
            .len()
            .checked_add(chunk.len())
            .ok_or(IdentityError::Contract("identity response too large"))?;
        if next > max_response_bytes {
            return Err(IdentityError::Contract("identity response too large"));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

/// Browser identity Fetch with redirect rejection before credentials can be
/// forwarded, omitted cookies/referrer, a hard deadline, and streaming body
/// bounds. Reqwest's wasm backend cannot currently express redirect:error.
#[cfg(target_arch = "wasm32")]
async fn browser_request(
    method: &str,
    url: &str,
    body: Option<&str>,
    max_response_bytes: usize,
) -> Result<(u16, Vec<u8>, Option<String>), IdentityError> {
    use std::cell::Cell;
    use std::rc::Rc;

    use futures_util::StreamExt as _;
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen_futures::JsFuture;

    let headers = web_sys::Headers::new()
        .map_err(|_| IdentityError::Contract("request headers unavailable"))?;
    if body.is_some() {
        headers
            .set("content-type", "application/x-www-form-urlencoded")
            .map_err(|_| IdentityError::Contract("request headers unavailable"))?;
    }
    headers
        .set("accept", "application/json")
        .map_err(|_| IdentityError::Contract("request headers unavailable"))?;
    let controller = web_sys::AbortController::new()
        .map_err(|_| IdentityError::Contract("request cancellation unavailable"))?;
    let timed_out = Rc::new(Cell::new(false));
    let timeout_flag = Rc::clone(&timed_out);
    let timeout_controller = controller.clone();
    let timeout = gloo_timers::callback::Timeout::new(30_000, move || {
        timeout_flag.set(true);
        timeout_controller.abort();
    });
    let init = web_sys::RequestInit::new();
    init.set_method(method);
    init.set_headers_headers(&headers);
    init.set_mode(web_sys::RequestMode::Cors);
    init.set_credentials(web_sys::RequestCredentials::Omit);
    init.set_cache(web_sys::RequestCache::NoStore);
    init.set_redirect(web_sys::RequestRedirect::Error);
    init.set_referrer_policy(web_sys::ReferrerPolicy::NoReferrer);
    init.set_signal(Some(&controller.signal()));
    if let Some(body) = body {
        init.set_body(&wasm_bindgen::JsValue::from_str(body));
    }
    let request = web_sys::Request::new_with_str_and_init(url, &init)
        .map_err(|_| IdentityError::Contract("request construction failed"))?;
    let window = web_sys::window().ok_or(IdentityError::Contract("browser window unavailable"))?;
    let value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| IdentityError::Unreachable)?;
    let response = value
        .dyn_into::<web_sys::Response>()
        .map_err(|_| IdentityError::Contract("invalid Fetch response"))?;
    let requested_url = url::Url::parse(url)
        .map_err(|_| IdentityError::Contract("identity request URL invalid"))?;
    let response_url = url::Url::parse(&response.url())
        .map_err(|_| IdentityError::Contract("identity response URL invalid"))?;
    if response.redirected() || response_url != requested_url {
        controller.abort();
        return Err(IdentityError::Contract("identity redirect rejected"));
    }
    let content_type = response
        .headers()
        .get("content-type")
        .map_err(|_| IdentityError::Contract("invalid response headers"))?;
    if let Some(length) = response
        .headers()
        .get("content-length")
        .map_err(|_| IdentityError::Contract("invalid response headers"))?
    {
        let length = length
            .parse::<u64>()
            .map_err(|_| IdentityError::Contract("invalid response length"))?;
        if length > u64::try_from(max_response_bytes).expect("usize fits u64") {
            controller.abort();
            return Err(IdentityError::Contract("identity response too large"));
        }
    }
    let mut bytes = Vec::new();
    if let Some(stream) = response.body() {
        let mut stream = wasm_streams::ReadableStream::from_raw(stream).into_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk
                .map_err(|_| IdentityError::Unreachable)?
                .dyn_into::<js_sys::Uint8Array>()
                .map_err(|_| IdentityError::Contract("invalid response body"))?;
            let length = usize::try_from(chunk.length()).expect("Uint8Array length fits usize");
            let next = bytes
                .len()
                .checked_add(length)
                .ok_or(IdentityError::Contract("identity response too large"))?;
            if next > max_response_bytes {
                controller.abort();
                return Err(IdentityError::Contract("identity response too large"));
            }
            let offset = bytes.len();
            bytes.resize(next, 0);
            chunk.copy_to(&mut bytes[offset..]);
        }
    }
    if timed_out.get() {
        return Err(IdentityError::Unreachable);
    }
    drop(timeout);
    Ok((response.status(), bytes, content_type))
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
    fn browser_callback_parser_preserves_routes_and_classifies_oauth_strictly() {
        assert_eq!(
            parse_authorization_callback_parameters(Some("project=abc&panel=live")),
            Ok(AuthorizationCallbackParameters::default())
        );
        let parsed = parse_authorization_callback_parameters(Some(
            "project=abc&code=authorization-code&state=returned-state",
        ))
        .expect("bounded callback");
        assert!(parsed.has_oauth_parameter);
        assert!(!parsed.has_duplicate);
        assert!(!parsed.has_error);
        assert_eq!(parsed.code.as_deref(), Some("authorization-code"));
        assert_eq!(parsed.state.as_deref(), Some("returned-state"));

        let duplicate =
            parse_authorization_callback_parameters(Some("code=first&code=second&state=s"))
                .expect("bounded callback");
        assert!(duplicate.has_oauth_parameter);
        assert!(duplicate.has_duplicate);
        assert_eq!(duplicate.code.as_deref(), Some("second"));

        let denied = parse_authorization_callback_parameters(Some(
            "error=access_denied&state=returned-state",
        ))
        .expect("bounded callback");
        assert!(denied.has_oauth_parameter);
        assert!(denied.has_error);
        assert_eq!(denied.state.as_deref(), Some("returned-state"));

        let oversized = "x".repeat(MAX_CALLBACK_PARAMETERS_BYTES + 1);
        assert_eq!(
            parse_authorization_callback_parameters(Some(&oversized)),
            Err(())
        );
    }

    #[test]
    fn browser_callback_route_match_is_exact() {
        let redirect =
            url::Url::parse("https://app.rspice.test/oauth/callback").expect("redirect URL");
        for candidate in [
            "https://app.rspice.test/oauth/callback",
            "https://app.rspice.test/oauth/callback?code=a",
            "https://app.rspice.test/oauth/callback#code=a",
        ] {
            let candidate = url::Url::parse(candidate).expect("candidate URL");
            assert!(is_exact_browser_callback_route(&candidate, &redirect));
        }
        for candidate in [
            "https://app.rspice.test/oauth/callback/",
            "https://app.rspice.test/other",
            "https://other.rspice.test/oauth/callback",
            "http://app.rspice.test/oauth/callback",
        ] {
            let candidate = url::Url::parse(candidate).expect("candidate URL");
            assert!(!is_exact_browser_callback_route(&candidate, &redirect));
        }
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
