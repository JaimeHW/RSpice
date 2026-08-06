use std::{error::Error, fmt};

use rspice_cloud_contract::Uuid;
use time::{Duration, OffsetDateTime};

use crate::{
    clock::current_time_utc,
    validation::{decode_lower_hex_sha256, parse_timestamp_text},
};

const MINIMUM_CAPABILITY_START_WINDOW: Duration = Duration::seconds(5);

/// Maximum byte length delivered in one `download_artifact_with` sink call.
pub const MAX_ARTIFACT_SINK_CHUNK_BYTES: usize = 1024 * 1024;

/// Stable, secret-free classification of a direct artifact-transfer failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactTransferFailure {
    /// The supplied handoff did not satisfy the RSpice capability contract.
    InvalidHandoff,
    /// The capability expired or was too close to expiry to start safely.
    CapabilityExpired,
    /// A native upload source was not a regular file.
    InvalidSource,
    /// Upload source bytes did not have the declared exact length.
    SourceLengthMismatch,
    /// Upload source bytes did not have the declared SHA-256.
    SourceDigestMismatch,
    /// A native download destination already existed.
    DestinationExists,
    /// Local file-system access failed.
    FileSystem,
    /// The configured direct-object HTTP request deadline elapsed.
    Timeout,
    /// A native connection could not be established.
    Connect,
    /// The object request could not be constructed or dispatched.
    Request,
    /// The object response body failed while being received.
    Body,
    /// The object transport failed without a more specific stable category.
    Other,
    /// A redirect was observed even though transfers prohibit redirects.
    RedirectedResponse,
    /// The object service returned a status outside the exact transfer contract.
    UnexpectedStatus,
    /// The upload response did not acknowledge the exact submitted SHA-256.
    UploadAcknowledgementMismatch,
    /// Download response headers violated the inert, non-cacheable, unencoded
    /// contract.
    InvalidResponseMetadata,
    /// Downloaded bytes did not have the declared exact length.
    ResponseLengthMismatch,
    /// Downloaded bytes did not have the declared SHA-256.
    ResponseDigestMismatch,
    /// The caller's streaming download sink rejected a chunk.
    Sink,
}

/// Secret-free error returned by a direct object-storage transfer.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ArtifactTransferError {
    failure: ArtifactTransferFailure,
    status: Option<u16>,
}

impl ArtifactTransferError {
    pub(crate) const fn new(failure: ArtifactTransferFailure, status: Option<u16>) -> Self {
        Self { failure, status }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) const fn with_status(self, status: u16) -> Self {
        Self {
            failure: self.failure,
            status: Some(status),
        }
    }

    /// Returns the stable transfer failure category.
    pub const fn failure(&self) -> ArtifactTransferFailure {
        self.failure
    }

    /// Returns the object-service HTTP status, when response headers arrived.
    pub const fn status(&self) -> Option<u16> {
        self.status
    }
}

impl fmt::Debug for ArtifactTransferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ArtifactTransferError")
            .field("failure", &self.failure)
            .field("status", &self.status)
            .finish()
    }
}

impl fmt::Display for ArtifactTransferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(status) = self.status {
            write!(
                formatter,
                "artifact transfer failed at HTTP {status}: {:?}",
                self.failure
            )
        } else {
            write!(formatter, "artifact transfer failed: {:?}", self.failure)
        }
    }
}

impl Error for ArtifactTransferError {}

/// Integrity receipt produced only after every transferred byte is verified.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactTransferReceipt {
    artifact_id: Uuid,
    content_length: u64,
    content_sha256: String,
}

impl ArtifactTransferReceipt {
    pub(crate) fn verified(artifact_id: Uuid, content_length: u64, content_sha256: String) -> Self {
        Self {
            artifact_id,
            content_length,
            content_sha256,
        }
    }

    /// Returns the transferred artifact ID.
    pub const fn artifact_id(&self) -> Uuid {
        self.artifact_id
    }

    /// Returns the exact verified byte length.
    pub const fn content_length(&self) -> u64 {
        self.content_length
    }

    /// Returns the verified lowercase SHA-256.
    pub fn content_sha256(&self) -> &str {
        &self.content_sha256
    }
}

pub(crate) fn expected_digest(value: &str) -> Result<[u8; 32], ArtifactTransferError> {
    decode_lower_hex_sha256(value)
        .ok_or_else(|| ArtifactTransferError::new(ArtifactTransferFailure::InvalidHandoff, None))
}

pub(crate) fn require_fresh_capability(expires_at: &str) -> Result<(), ArtifactTransferError> {
    let now = current_time_utc().ok_or_else(|| {
        ArtifactTransferError::new(ArtifactTransferFailure::CapabilityExpired, None)
    })?;
    require_fresh_capability_at(expires_at, now)
}

fn require_fresh_capability_at(
    expires_at: &str,
    now: OffsetDateTime,
) -> Result<(), ArtifactTransferError> {
    let fresh = parse_timestamp_text(expires_at).is_some_and(|expires_at| {
        expires_at >= now.saturating_add(MINIMUM_CAPABILITY_START_WINDOW)
    });
    if fresh {
        Ok(())
    } else {
        Err(ArtifactTransferError::new(
            ArtifactTransferFailure::CapabilityExpired,
            None,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_must_leave_a_safe_start_window() {
        let now = OffsetDateTime::from_unix_timestamp(1_700_000_000).expect("test timestamp");
        assert!(require_fresh_capability_at("2023-11-14T22:13:25Z", now).is_ok());
        assert_eq!(
            require_fresh_capability_at("2023-11-14T22:13:24Z", now)
                .expect_err("four-second window is too short")
                .failure(),
            ArtifactTransferFailure::CapabilityExpired
        );
        assert_eq!(
            require_fresh_capability_at("not-a-timestamp", now)
                .expect_err("malformed expiry must fail closed")
                .failure(),
            ArtifactTransferFailure::CapabilityExpired
        );
    }

    #[test]
    fn transfer_errors_and_receipts_never_retain_urls() {
        let error = ArtifactTransferError::new(ArtifactTransferFailure::Request, None);
        assert_eq!(error.status(), None);
        assert!(!format!("{error:?}").contains("http"));
        let receipt = ArtifactTransferReceipt::verified(Uuid::from_u128(7), 4, "00".repeat(32));
        assert_eq!(receipt.artifact_id(), Uuid::from_u128(7));
        assert_eq!(receipt.content_length(), 4);
        assert_eq!(receipt.content_sha256(), "00".repeat(32));
    }
}
