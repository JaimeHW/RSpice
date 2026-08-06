use std::{error::Error, fmt, time::Duration};

use rspice_cloud_contract::ProblemDetails;

/// Bounded response metadata useful for support and caller-controlled recovery.
#[derive(Clone, Default, Eq, PartialEq)]
pub struct ResponseMetadata {
    pub(crate) request_id: Option<String>,
    pub(crate) retry_after: Option<Duration>,
    pub(crate) idempotency_replayed: Option<bool>,
    pub(crate) location: Option<String>,
}

impl ResponseMetadata {
    /// Returns the API request ID, when the response supplied one.
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// Returns the server-requested retry delay, when present and valid.
    ///
    /// The client never sleeps or retries automatically.
    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }

    /// Reports whether the server identified an idempotent mutation replay.
    pub fn idempotency_replayed(&self) -> Option<bool> {
        self.idempotency_replayed
    }

    /// Returns a validated same-service resource location, when supplied.
    ///
    /// Locations are deliberately redacted from diagnostic formatting even
    /// though the protocol permits only stable, query-free API paths.
    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }
}

impl fmt::Debug for ResponseMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResponseMetadata")
            .field("request_id", &self.request_id)
            .field("retry_after", &self.retry_after)
            .field("idempotency_replayed", &self.idempotency_replayed)
            .field("location", &self.location.as_ref().map(|_| "[REDACTED]"))
            .finish()
    }
}

/// Stable, URL-free classification of a request transport failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportFailure {
    /// The HTTP client could not be initialized.
    ClientBuild,
    /// The per-request deadline elapsed.
    Timeout,
    /// A network connection could not be established.
    Connect,
    /// The request could not be constructed or dispatched.
    Request,
    /// The response body failed while being received.
    Body,
    /// The transport failed without a more specific stable classification.
    Other,
}

/// Stable classification of a response that violated the RSpice Cloud protocol.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolFailure {
    /// The browser or transport followed a redirect.
    RedirectedResponse,
    /// The HTTP status was not declared for the requested operation.
    UnexpectedStatus,
    /// The response media type did not match the declared representation.
    UnexpectedContentType,
    /// The success body was not valid JSON for the shared contract type.
    InvalidJson,
    /// A valid success body violated an operation-specific security invariant.
    InvalidSuccessResponse,
    /// A returned bearer capability had too little lifetime left for safe use.
    CapabilityExpired,
    /// An RFC 9457 response was malformed or disagreed with its HTTP status.
    InvalidProblem,
    /// A body was returned for a response that must be empty.
    UnexpectedBody,
    /// A bounded response metadata header was malformed.
    InvalidMetadata,
    /// A declared response length was malformed.
    InvalidContentLength,
}

/// Failure returned by an RSpice Cloud request.
pub enum CloudError {
    /// Network or HTTP-stack failure, intentionally stripped of URLs and bodies.
    Transport {
        /// Stable transport category.
        failure: TransportFailure,
        /// HTTP status when the failure occurred after response headers arrived.
        status: Option<u16>,
        /// Bounded response metadata available after headers arrived.
        metadata: Option<ResponseMetadata>,
    },
    /// The response exceeded the caller's configured buffered-body limit.
    ResponseTooLarge {
        /// Configured maximum number of bytes.
        limit: usize,
        /// HTTP status associated with the oversized response.
        status: u16,
        /// Metadata parsed before body buffering stopped.
        metadata: ResponseMetadata,
    },
    /// The service returned a valid RFC 9457 problem response.
    Problem {
        /// Structured service error suitable for deliberate UI presentation.
        details: Box<ProblemDetails>,
        /// Support and retry metadata associated with the response.
        metadata: ResponseMetadata,
    },
    /// The service or an intermediary violated the declared wire protocol.
    Protocol {
        /// Stable violation category.
        failure: ProtocolFailure,
        /// HTTP status, if a response was received.
        status: Option<u16>,
        /// Metadata parsed before the violation was detected.
        metadata: ResponseMetadata,
    },
}

impl CloudError {
    /// Returns the HTTP status associated with a server response.
    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Problem { details, .. } => Some(details.status),
            Self::Protocol { status, .. } | Self::Transport { status, .. } => *status,
            Self::ResponseTooLarge { status, .. } => Some(*status),
        }
    }

    /// Returns the structured service problem, if one was received and validated.
    pub fn problem(&self) -> Option<&ProblemDetails> {
        match self {
            Self::Problem { details, .. } => Some(details),
            _ => None,
        }
    }

    /// Returns response metadata when the request reached an HTTP server.
    pub fn metadata(&self) -> Option<&ResponseMetadata> {
        match self {
            Self::Problem { metadata, .. }
            | Self::Protocol { metadata, .. }
            | Self::ResponseTooLarge { metadata, .. } => Some(metadata),
            Self::Transport { metadata, .. } => metadata.as_ref(),
        }
    }
}

impl fmt::Debug for CloudError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport {
                failure,
                status,
                metadata,
            } => formatter
                .debug_struct("Transport")
                .field("failure", failure)
                .field("status", status)
                .field("metadata", metadata)
                .finish(),
            Self::ResponseTooLarge {
                limit,
                status,
                metadata,
            } => formatter
                .debug_struct("ResponseTooLarge")
                .field("limit", limit)
                .field("status", status)
                .field("metadata", metadata)
                .finish(),
            Self::Problem { details, metadata } => formatter
                .debug_struct("Problem")
                .field("type", &details.kind)
                .field("title", &details.title)
                .field("status", &details.status)
                .field("detail", &"[REDACTED]")
                .field("instance", &"[REDACTED]")
                .field("metadata", metadata)
                .finish(),
            Self::Protocol {
                failure,
                status,
                metadata,
            } => formatter
                .debug_struct("Protocol")
                .field("failure", failure)
                .field("status", status)
                .field("metadata", metadata)
                .finish(),
        }
    }
}

impl fmt::Display for CloudError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport { failure, .. } => {
                write!(formatter, "cloud transport failed: {failure:?}")
            }
            Self::ResponseTooLarge { limit, .. } => {
                write!(formatter, "cloud response exceeded the {limit}-byte limit")
            }
            Self::Problem { details, .. } => {
                write!(
                    formatter,
                    "cloud request failed with HTTP {}",
                    details.status
                )
            }
            Self::Protocol {
                failure, status, ..
            } => match status {
                Some(status) => write!(
                    formatter,
                    "cloud response violated the protocol at HTTP {status}: {failure:?}"
                ),
                None => write!(
                    formatter,
                    "cloud response violated the protocol: {failure:?}"
                ),
            },
        }
    }
}

impl Error for CloudError {}

pub(crate) fn map_transport(error: &reqwest::Error) -> TransportFailure {
    if error.is_timeout() {
        return TransportFailure::Timeout;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if error.is_connect() {
        return TransportFailure::Connect;
    }
    if error.is_request() {
        return TransportFailure::Request;
    }
    if error.is_body() {
        return TransportFailure::Body;
    }
    TransportFailure::Other
}
