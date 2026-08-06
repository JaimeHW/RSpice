use std::{error::Error, fmt, net::IpAddr, time::Duration};

use url::{Host, Url};

const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_REQUEST_TIMEOUT: Duration = Duration::from_secs(5 * 60);
const DEFAULT_TRANSFER_TIMEOUT: Duration = Duration::from_secs(2 * 60 * 60);
const MIN_TRANSFER_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_TRANSFER_TIMEOUT: Duration = Duration::from_secs(24 * 60 * 60);
const DEFAULT_MAX_RESPONSE_BYTES: usize = 4 * 1024 * 1024;
const HARD_MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;

/// Transport environment enforced by a client endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndpointMode {
    /// Requires HTTPS for every endpoint.
    Production,
    /// Allows HTTP or HTTPS only on the local loopback interface.
    LoopbackDevelopment,
}

/// Validated construction settings for [`crate::CloudClient`].
#[derive(Clone)]
pub struct ClientConfig {
    pub(crate) endpoint: Url,
    pub(crate) object_storage_origin: Url,
    pub(crate) mode: EndpointMode,
    pub(crate) request_timeout: Duration,
    pub(crate) transfer_timeout: Duration,
    pub(crate) max_response_bytes: usize,
}

impl ClientConfig {
    /// Creates a production configuration that requires distinct root HTTPS
    /// origins for the API and direct object transfers.
    pub fn production(
        endpoint: &str,
        object_storage_origin: &str,
    ) -> Result<Self, ConfigurationError> {
        Self::parse(endpoint, object_storage_origin, EndpointMode::Production)
    }

    /// Creates a development configuration whose plaintext origins are
    /// restricted to the local loopback interface.
    pub fn loopback_development(
        endpoint: &str,
        object_storage_origin: &str,
    ) -> Result<Self, ConfigurationError> {
        Self::parse(
            endpoint,
            object_storage_origin,
            EndpointMode::LoopbackDevelopment,
        )
    }

    fn parse(
        endpoint: &str,
        object_storage_origin: &str,
        mode: EndpointMode,
    ) -> Result<Self, ConfigurationError> {
        let endpoint = Url::parse(endpoint).map_err(|_| ConfigurationError::InvalidUrl)?;
        validate_endpoint(&endpoint, mode)?;
        let object_storage_origin = Url::parse(object_storage_origin)
            .map_err(|_| ConfigurationError::InvalidObjectStorageOrigin)?;
        validate_endpoint(&object_storage_origin, mode)
            .map_err(|_| ConfigurationError::InvalidObjectStorageOrigin)?;
        if mode == EndpointMode::Production && endpoint.origin() == object_storage_origin.origin() {
            return Err(ConfigurationError::SharedProductionOrigin);
        }
        Ok(Self {
            endpoint,
            object_storage_origin,
            mode,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            transfer_timeout: DEFAULT_TRANSFER_TIMEOUT,
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
        })
    }

    /// Replaces the direct-object HTTP request deadline.
    ///
    /// This is independent of the short control-plane request deadline and of
    /// the capability's start-by expiry. It bounds the object request, not
    /// caller-owned sink work or local file-system operations.
    pub fn with_transfer_timeout(
        mut self,
        transfer_timeout: Duration,
    ) -> Result<Self, ConfigurationError> {
        if !(MIN_TRANSFER_TIMEOUT..=MAX_TRANSFER_TIMEOUT).contains(&transfer_timeout) {
            return Err(ConfigurationError::InvalidTransferTimeout);
        }
        self.transfer_timeout = transfer_timeout;
        Ok(self)
    }

    /// Replaces the per-request deadline.
    pub fn with_request_timeout(
        mut self,
        request_timeout: Duration,
    ) -> Result<Self, ConfigurationError> {
        if request_timeout.is_zero() || request_timeout > MAX_REQUEST_TIMEOUT {
            return Err(ConfigurationError::InvalidRequestTimeout);
        }
        self.request_timeout = request_timeout;
        Ok(self)
    }

    /// Replaces the maximum buffered JSON response size.
    ///
    /// Artifact payloads must use the separate streaming transfer methods and
    /// never this bounded metadata path.
    pub fn with_max_response_bytes(
        mut self,
        max_response_bytes: usize,
    ) -> Result<Self, ConfigurationError> {
        if max_response_bytes == 0 || max_response_bytes > HARD_MAX_RESPONSE_BYTES {
            return Err(ConfigurationError::InvalidResponseLimit);
        }
        self.max_response_bytes = max_response_bytes;
        Ok(self)
    }

    /// Returns the validated API origin.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Returns the only origin accepted for direct object capabilities.
    pub fn object_storage_origin(&self) -> &Url {
        &self.object_storage_origin
    }

    /// Returns the enforced endpoint environment.
    pub fn mode(&self) -> EndpointMode {
        self.mode
    }

    /// Returns the configured per-request deadline.
    pub fn request_timeout(&self) -> Duration {
        self.request_timeout
    }

    /// Returns the direct-object HTTP request deadline.
    pub fn transfer_timeout(&self) -> Duration {
        self.transfer_timeout
    }

    /// Returns the maximum buffered JSON response size.
    pub fn max_response_bytes(&self) -> usize {
        self.max_response_bytes
    }
}

impl fmt::Debug for ClientConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClientConfig")
            .field("endpoint", &self.endpoint)
            .field("object_storage_origin", &self.object_storage_origin)
            .field("mode", &self.mode)
            .field("request_timeout", &self.request_timeout)
            .field("transfer_timeout", &self.transfer_timeout)
            .field("max_response_bytes", &self.max_response_bytes)
            .finish()
    }
}

fn validate_endpoint(endpoint: &Url, mode: EndpointMode) -> Result<(), ConfigurationError> {
    if !endpoint.username().is_empty() || endpoint.password().is_some() {
        return Err(ConfigurationError::CredentialsInUrl);
    }
    if endpoint.query().is_some() {
        return Err(ConfigurationError::QueryInUrl);
    }
    if endpoint.fragment().is_some() {
        return Err(ConfigurationError::FragmentInUrl);
    }
    let Some(host) = endpoint.host() else {
        return Err(ConfigurationError::MissingHost);
    };
    if endpoint.path() != "/" || endpoint.cannot_be_a_base() {
        return Err(ConfigurationError::NonRootPath);
    }

    match mode {
        EndpointMode::Production if endpoint.scheme() != "https" => {
            Err(ConfigurationError::InsecureProductionEndpoint)
        }
        EndpointMode::Production => Ok(()),
        EndpointMode::LoopbackDevelopment => {
            if !matches!(endpoint.scheme(), "http" | "https") {
                return Err(ConfigurationError::UnsupportedScheme);
            }
            if is_loopback_host(host) {
                Ok(())
            } else {
                Err(ConfigurationError::NonLoopbackDevelopmentEndpoint)
            }
        }
    }
}

fn is_loopback_host(host: Host<&str>) -> bool {
    match host {
        Host::Domain(domain) => domain.eq_ignore_ascii_case("localhost"),
        Host::Ipv4(address) => IpAddr::V4(address).is_loopback(),
        Host::Ipv6(address) => IpAddr::V6(address).is_loopback(),
    }
}

/// Reason an HTTP client configuration was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationError {
    /// The endpoint was not an absolute URL.
    InvalidUrl,
    /// The endpoint did not contain a host.
    MissingHost,
    /// The object-storage origin was not a valid root origin for this mode.
    InvalidObjectStorageOrigin,
    /// Production attempted to share one origin between the application and
    /// customer-controlled object bytes.
    SharedProductionOrigin,
    /// User information was embedded in the endpoint URL.
    CredentialsInUrl,
    /// The endpoint contained a query string.
    QueryInUrl,
    /// The endpoint contained a fragment.
    FragmentInUrl,
    /// The endpoint contained a base path instead of a root origin.
    NonRootPath,
    /// A production endpoint used plaintext HTTP.
    InsecureProductionEndpoint,
    /// A development endpoint resolved outside the loopback interface.
    NonLoopbackDevelopmentEndpoint,
    /// A development endpoint used a scheme other than HTTP or HTTPS.
    UnsupportedScheme,
    /// The request deadline was zero or exceeded five minutes.
    InvalidRequestTimeout,
    /// The direct-object transfer deadline was outside 30 seconds to 24 hours.
    InvalidTransferTimeout,
    /// The response bound was zero or exceeded the 16 MiB hard limit.
    InvalidResponseLimit,
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidUrl => "cloud endpoint must be an absolute URL",
            Self::MissingHost => "cloud endpoint must contain a host",
            Self::InvalidObjectStorageOrigin => {
                "object storage must use a credentialless root HTTPS origin (or a loopback HTTP origin in development)"
            }
            Self::SharedProductionOrigin => {
                "production API and object storage must use distinct origins"
            }
            Self::CredentialsInUrl => "cloud endpoint must not contain URL credentials",
            Self::QueryInUrl => "cloud endpoint must not contain a query string",
            Self::FragmentInUrl => "cloud endpoint must not contain a fragment",
            Self::NonRootPath => "cloud endpoint must be a root origin without a base path",
            Self::InsecureProductionEndpoint => "production cloud endpoint must use HTTPS",
            Self::NonLoopbackDevelopmentEndpoint => {
                "development cloud endpoint must use localhost or a loopback IP address"
            }
            Self::UnsupportedScheme => "development cloud endpoint must use HTTP or HTTPS",
            Self::InvalidRequestTimeout => {
                "cloud request timeout must be greater than zero and at most five minutes"
            }
            Self::InvalidTransferTimeout => {
                "artifact transfer timeout must be between 30 seconds and 24 hours"
            }
            Self::InvalidResponseLimit => {
                "cloud JSON response limit must be between one byte and 16 MiB"
            }
        })
    }
}

impl Error for ConfigurationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_requires_clean_distinct_https_origins() {
        assert!(
            ClientConfig::production("https://api.rspice.app", "https://objects.rspice.app")
                .is_ok()
        );
        assert_eq!(
            ClientConfig::production("http://api.rspice.app", "https://objects.rspice.app")
                .expect_err("plaintext"),
            ConfigurationError::InsecureProductionEndpoint
        );
        assert_eq!(
            ClientConfig::production(
                "https://user:secret@api.rspice.app",
                "https://objects.rspice.app"
            )
            .expect_err("credentials"),
            ConfigurationError::CredentialsInUrl
        );
        assert_eq!(
            ClientConfig::production("https://api.rspice.app/api", "https://objects.rspice.app")
                .expect_err("base path"),
            ConfigurationError::NonRootPath
        );
        assert_eq!(
            ClientConfig::production(
                "https://api.rspice.app?token=secret",
                "https://objects.rspice.app"
            )
            .expect_err("query"),
            ConfigurationError::QueryInUrl
        );
        assert_eq!(
            ClientConfig::production(
                "https://api.rspice.app/#fragment",
                "https://objects.rspice.app"
            )
            .expect_err("fragment"),
            ConfigurationError::FragmentInUrl
        );
        assert_eq!(
            ClientConfig::production("https://api.rspice.app", "http://objects.rspice.app")
                .expect_err("plaintext object origin"),
            ConfigurationError::InvalidObjectStorageOrigin
        );
        assert_eq!(
            ClientConfig::production("https://api.rspice.app", "https://api.rspice.app")
                .expect_err("shared application and object origin"),
            ConfigurationError::SharedProductionOrigin
        );
        assert_eq!(
            ClientConfig::production("https://api.rspice.app", "https://api.rspice.app:443")
                .expect_err("default ports identify the same origin"),
            ConfigurationError::SharedProductionOrigin
        );
    }

    #[test]
    fn development_plaintext_is_loopback_only() {
        assert!(
            ClientConfig::loopback_development("http://localhost:8080", "http://localhost:9000")
                .is_ok()
        );
        assert!(
            ClientConfig::loopback_development("http://127.0.0.2:8080", "http://127.0.0.2:9000")
                .is_ok()
        );
        assert!(
            ClientConfig::loopback_development("http://[::1]:8080", "http://[::1]:9000").is_ok()
        );
        assert_eq!(
            ClientConfig::loopback_development("http://192.168.1.10:8080", "http://127.0.0.1:9000")
                .expect_err("non-loopback"),
            ConfigurationError::NonLoopbackDevelopmentEndpoint
        );
        assert_eq!(
            ClientConfig::loopback_development("file:///tmp/rspice", "http://127.0.0.1:9000")
                .expect_err("missing host"),
            ConfigurationError::MissingHost
        );
        assert_eq!(
            ClientConfig::loopback_development("http://127.0.0.1:8080", "http://192.168.1.10:9000")
                .expect_err("non-loopback object origin"),
            ConfigurationError::InvalidObjectStorageOrigin
        );
    }

    #[test]
    fn transfer_deadlines_are_independent_and_bounded() {
        let config =
            ClientConfig::production("https://api.rspice.app", "https://objects.rspice.app")
                .expect("production endpoints");
        assert_eq!(config.request_timeout(), Duration::from_secs(30));
        assert_eq!(config.transfer_timeout(), Duration::from_secs(2 * 60 * 60));
        assert_eq!(
            config
                .clone()
                .with_transfer_timeout(Duration::from_secs(29))
                .expect_err("too-short transfer deadline"),
            ConfigurationError::InvalidTransferTimeout
        );
        assert_eq!(
            config
                .clone()
                .with_transfer_timeout(Duration::from_secs(24 * 60 * 60 + 1))
                .expect_err("too-long transfer deadline"),
            ConfigurationError::InvalidTransferTimeout
        );
        assert_eq!(
            config
                .with_transfer_timeout(Duration::from_secs(30))
                .expect("minimum transfer deadline")
                .transfer_timeout(),
            Duration::from_secs(30)
        );
    }
}
