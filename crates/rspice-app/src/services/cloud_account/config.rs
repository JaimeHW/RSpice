//! Release-pinned cloud endpoint and identity policy.
//!
//! The client-integration contract requires fixed release-configured API and
//! object-storage origins — never values discovered from a server, document,
//! or user-controlled string — and the native license verifier's issuer,
//! audience, and product are release policy in the same sense. All of it is
//! injected at compile time (`RSPICE_CLOUD_*` build environment), so a
//! shipped binary cannot be pointed at another service. A build without the
//! full set has no cloud account surface at all: this resolver fails closed,
//! exactly like the empty production license verifying-key set in
//! `services::license`.
//!
//! Development builds (`rspice_development_build`) may instead take the same
//! variables from the runtime environment to reach a local loopback stack.

/// Everything the cloud account executor needs, resolved once at startup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CloudAccountConfig {
    /// API origin, e.g. `https://api.rspice.app`.
    pub api_origin: String,
    /// Object-storage origin; must differ from the API origin in production.
    pub object_storage_origin: String,
    /// OIDC issuer, matched byte-for-byte by the API.
    pub oidc_issuer: String,
    /// Public OIDC client this application authenticates as.
    pub oidc_client_id: String,
    /// Release-pinned native license policy for `NativeLicenseVerifier`.
    #[cfg(not(target_arch = "wasm32"))]
    pub license_issuer: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub license_audience: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub license_product: String,
    /// Loopback ports registered as OAuth redirect URIs, tried in order.
    #[cfg(not(target_arch = "wasm32"))]
    pub loopback_ports: Vec<u16>,
    /// Exact redirect URI registered to the browser public client. It is
    /// release-pinned and verified against the page origin at runtime.
    #[cfg(target_arch = "wasm32")]
    pub browser_redirect_uri: String,
    /// Loopback development stack (plain HTTP on 127.0.0.1) instead of
    /// production TLS endpoints.
    pub development: bool,
}

/// Ports the identity provider registers as `http://127.0.0.1:{port}/oauth/callback`.
///
/// Fixed because the provider matches redirect URIs exactly; three ports keep
/// sign-in working when one is taken by another process.
#[cfg(not(target_arch = "wasm32"))]
const DEFAULT_LOOPBACK_PORTS: [u16; 3] = [17_872, 27_194, 38_641];

impl CloudAccountConfig {
    /// The configuration this build was produced with, or `None` when the
    /// build carries no cloud endpoints (fail closed, no partial sets).
    pub(crate) fn resolve() -> Option<Self> {
        #[cfg(all(rspice_development_build, not(target_arch = "wasm32")))]
        if let Some(configuration) = Self::from_runtime_environment() {
            return Some(configuration);
        }
        Self::from_build_environment()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn from_build_environment() -> Option<Self> {
        Self::from_parts(
            option_env!("RSPICE_CLOUD_API_ORIGIN")?,
            option_env!("RSPICE_CLOUD_OBJECT_STORAGE_ORIGIN")?,
            option_env!("RSPICE_CLOUD_OIDC_ISSUER")?,
            option_env!("RSPICE_CLOUD_OIDC_CLIENT_ID")?,
            option_env!("RSPICE_CLOUD_LICENSE_ISSUER")?,
            option_env!("RSPICE_CLOUD_LICENSE_AUDIENCE")?,
            option_env!("RSPICE_CLOUD_LICENSE_PRODUCT")?,
            option_env!("RSPICE_CLOUD_LOOPBACK_PORTS"),
        )
    }

    #[cfg(target_arch = "wasm32")]
    fn from_build_environment() -> Option<Self> {
        Self::from_browser_parts(
            option_env!("RSPICE_CLOUD_API_ORIGIN")?,
            option_env!("RSPICE_CLOUD_OBJECT_STORAGE_ORIGIN")?,
            option_env!("RSPICE_CLOUD_OIDC_ISSUER")?,
            option_env!("RSPICE_CLOUD_BROWSER_OIDC_CLIENT_ID")?,
            option_env!("RSPICE_CLOUD_BROWSER_REDIRECT_URI")?,
        )
    }

    /// Development builds may point at a local stack via runtime variables.
    #[cfg(all(rspice_development_build, not(target_arch = "wasm32")))]
    fn from_runtime_environment() -> Option<Self> {
        let variable = |name: &str| std::env::var(name).ok().filter(|value| !value.is_empty());
        Self::from_parts(
            &variable("RSPICE_CLOUD_API_ORIGIN")?,
            &variable("RSPICE_CLOUD_OBJECT_STORAGE_ORIGIN")?,
            &variable("RSPICE_CLOUD_OIDC_ISSUER")?,
            &variable("RSPICE_CLOUD_OIDC_CLIENT_ID")?,
            &variable("RSPICE_CLOUD_LICENSE_ISSUER")?,
            &variable("RSPICE_CLOUD_LICENSE_AUDIENCE")?,
            &variable("RSPICE_CLOUD_LICENSE_PRODUCT")?,
            variable("RSPICE_CLOUD_LOOPBACK_PORTS").as_deref(),
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn from_parts(
        api_origin: &str,
        object_storage_origin: &str,
        oidc_issuer: &str,
        oidc_client_id: &str,
        license_issuer: &str,
        license_audience: &str,
        license_product: &str,
        loopback_ports: Option<&str>,
    ) -> Option<Self> {
        let required = [
            api_origin,
            object_storage_origin,
            oidc_issuer,
            oidc_client_id,
            license_issuer,
            license_audience,
            license_product,
        ];
        if required.iter().any(|value| value.trim().is_empty()) {
            return None;
        }
        let development = api_origin.starts_with("http://");
        let loopback_ports = match loopback_ports {
            Some(list) => parse_ports(list)?,
            None => DEFAULT_LOOPBACK_PORTS.to_vec(),
        };
        Some(Self {
            api_origin: api_origin.trim_end_matches('/').to_owned(),
            object_storage_origin: object_storage_origin.trim_end_matches('/').to_owned(),
            oidc_issuer: oidc_issuer.to_owned(),
            oidc_client_id: oidc_client_id.to_owned(),
            license_issuer: license_issuer.to_owned(),
            license_audience: license_audience.to_owned(),
            license_product: license_product.to_owned(),
            loopback_ports,
            development,
        })
    }

    #[cfg(target_arch = "wasm32")]
    fn from_browser_parts(
        api_origin: &str,
        object_storage_origin: &str,
        oidc_issuer: &str,
        oidc_client_id: &str,
        browser_redirect_uri: &str,
    ) -> Option<Self> {
        let required = [
            api_origin,
            object_storage_origin,
            oidc_issuer,
            oidc_client_id,
            browser_redirect_uri,
        ];
        if required.iter().any(|value| value.trim().is_empty()) {
            return None;
        }
        let api = url::Url::parse(api_origin).ok()?;
        let oidc = url::Url::parse(oidc_issuer).ok()?;
        let redirect = url::Url::parse(browser_redirect_uri).ok()?;
        if api.username() != ""
            || api.password().is_some()
            || oidc.username() != ""
            || oidc.password().is_some()
            || oidc.query().is_some()
            || oidc.fragment().is_some()
            || redirect.username() != ""
            || redirect.password().is_some()
            || redirect.query().is_some()
            || redirect.fragment().is_some()
        {
            return None;
        }
        let development = api.scheme() == "http";
        let schemes_are_safe = if development {
            api.scheme() == "http"
                && oidc.scheme() == "http"
                && redirect.scheme() == "http"
                && matches!(
                    api.host_str(),
                    Some("127.0.0.1") | Some("localhost") | Some("[::1]")
                )
                && matches!(
                    oidc.host_str(),
                    Some("127.0.0.1") | Some("localhost") | Some("[::1]")
                )
                && matches!(
                    redirect.host_str(),
                    Some("127.0.0.1") | Some("localhost") | Some("[::1]")
                )
        } else {
            api.scheme() == "https" && oidc.scheme() == "https" && redirect.scheme() == "https"
        };
        if !schemes_are_safe {
            return None;
        }
        Some(Self {
            api_origin: api_origin.trim_end_matches('/').to_owned(),
            object_storage_origin: object_storage_origin.trim_end_matches('/').to_owned(),
            oidc_issuer: oidc_issuer.to_owned(),
            oidc_client_id: oidc_client_id.to_owned(),
            browser_redirect_uri: browser_redirect_uri.to_owned(),
            development,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_ports(list: &str) -> Option<Vec<u16>> {
    let ports = list
        .split(',')
        .map(|part| part.trim().parse::<u16>().ok().filter(|port| *port != 0))
        .collect::<Option<Vec<u16>>>()?;
    if ports.is_empty() { None } else { Some(ports) }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn a_full_set_of_parts_resolves() {
        let configuration = CloudAccountConfig::from_parts(
            "https://api.rspice.app/",
            "https://objects.rspice.app",
            "https://id.rspice.app/realms/rspice",
            "rspice-desktop",
            "https://licenses.rspice.app/",
            "rspice-native",
            "rspice",
            None,
        )
        .expect("full parts must resolve");
        assert_eq!(configuration.api_origin, "https://api.rspice.app");
        assert!(!configuration.development);
        assert_eq!(
            configuration.loopback_ports,
            DEFAULT_LOOPBACK_PORTS.to_vec()
        );
    }

    #[test]
    fn any_missing_part_fails_closed() {
        assert_eq!(
            CloudAccountConfig::from_parts(
                "https://api.rspice.app",
                "https://objects.rspice.app",
                "https://id.rspice.app/realms/rspice",
                "rspice-desktop",
                "https://licenses.rspice.app/",
                " ",
                "rspice",
                None,
            ),
            None
        );
    }

    #[test]
    fn loopback_origin_selects_development_mode() {
        let configuration = CloudAccountConfig::from_parts(
            "http://127.0.0.1:8080",
            "http://127.0.0.1:9000",
            "http://127.0.0.1:8180/realms/rspice",
            "rspice-local",
            "https://licenses.rspice.local/",
            "rspice-native",
            "rspice",
            Some("3000"),
        )
        .expect("loopback parts must resolve");
        assert!(configuration.development);
        assert_eq!(configuration.loopback_ports, vec![3000]);
    }

    #[test]
    fn malformed_port_lists_fail_closed() {
        assert_eq!(parse_ports("3000,abc"), None);
        assert_eq!(parse_ports("0"), None);
        assert_eq!(parse_ports(""), None);
    }
}
