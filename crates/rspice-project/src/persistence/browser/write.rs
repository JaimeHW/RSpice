//! Staged browser publication through host-owned asynchronous storage.

use super::{BrowserBindingBackend, BrowserWriteIntent};
use crate::persistence::digest_bytes;
use rspice_app_types::product::ContentDigest;
use std::{fmt, future::Future};

/// The host retains permissions, locks and handles for the whole operation.
/// Futures need not be Send: browser handles stay on their owning thread.
pub trait BrowserProjectStorage {
    type Writable;

    fn create_writable(
        &self,
        backend: BrowserBindingBackend,
    ) -> impl Future<Output = Result<Self::Writable, String>>;
    fn write(
        &self,
        writable: &Self::Writable,
        bytes: &[u8],
    ) -> impl Future<Output = Result<(), String>>;
    fn read(&self) -> impl Future<Output = Result<Vec<u8>, String>>;
    fn abort(&self, writable: &Self::Writable) -> impl Future<Output = Result<(), String>>;
    fn close(&self, writable: &Self::Writable) -> impl Future<Output = Result<(), String>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserWriteOperation {
    Write,
    PreCommitVerification,
    Close,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserWriteError {
    Platform(String),
    ExternalChange(ContentDigest),
    /// Close failure remains uncertain even if a subsequent abort succeeds.
    OperationFailed {
        operation: BrowserWriteOperation,
        error: String,
        abort_error: Option<String>,
    },
    ChangedDuringStaging {
        observed_digest: ContentDigest,
        abort_error: String,
    },
    ReadBackFailed(String),
    ReadBackMismatch {
        staged_digest: ContentDigest,
        observed_digest: ContentDigest,
    },
}

impl From<String> for BrowserWriteError {
    fn from(error: String) -> Self {
        Self::Platform(error)
    }
}

impl fmt::Display for BrowserWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Platform(error) => f.write_str(error),
            Self::ExternalChange(_) => {
                f.write_str("canonical browser project changed outside RSpice")
            }
            Self::OperationFailed {
                operation,
                error,
                abort_error,
            } => {
                let label = match operation {
                    BrowserWriteOperation::Write => "write",
                    BrowserWriteOperation::PreCommitVerification => "pre-commit verification",
                    BrowserWriteOperation::Close => "close",
                };
                write!(f, "browser project {label} failed")?;
                if abort_error.is_none() {
                    f.write_str(" and staging was aborted")?;
                }
                write!(f, ": {error}")?;
                if *operation == BrowserWriteOperation::Close {
                    f.write_str("; publication outcome is uncertain")?;
                }
                if let Some(abort_error) = abort_error {
                    write!(f, "; staging abort also failed: {abort_error}")?;
                }
                Ok(())
            }
            Self::ChangedDuringStaging { abort_error, .. } => write!(
                f,
                "canonical browser project changed while staged bytes were pending, and staging abort failed: {abort_error}"
            ),
            Self::ReadBackFailed(error) => write!(
                f,
                "browser write completed, but read-back verification failed: {error}"
            ),
            Self::ReadBackMismatch { .. } => f.write_str(
                "browser write completed, but read-back bytes do not match the staged project",
            ),
        }
    }
}

impl std::error::Error for BrowserWriteError {}

impl BrowserWriteIntent {
    /// Admit accepted content before creating a writable. Newly selected
    /// destinations also capture their current bytes and recheck before close.
    /// Success requires exact read-back; no failure returns acceptance.
    pub async fn publish_bytes(
        &self,
        storage: &impl BrowserProjectStorage,
        bytes: &[u8],
    ) -> Result<ContentDigest, BrowserWriteError> {
        let expected_before_commit = digest_bytes(&storage.read().await?);
        if self
            .expected_digest
            .is_some_and(|accepted| accepted != expected_before_commit)
        {
            return Err(BrowserWriteError::ExternalChange(expected_before_commit));
        }
        let writable = storage.create_writable(self.backend).await?;
        if let Err(error) = storage.write(&writable, bytes).await {
            return Err(BrowserWriteError::OperationFailed {
                operation: BrowserWriteOperation::Write,
                error,
                abort_error: storage.abort(&writable).await.err(),
            });
        }
        match storage.read().await {
            Ok(current) => {
                let observed_digest = digest_bytes(&current);
                if observed_digest != expected_before_commit {
                    return match storage.abort(&writable).await {
                        Ok(()) => Err(BrowserWriteError::ExternalChange(observed_digest)),
                        Err(abort_error) => Err(BrowserWriteError::ChangedDuringStaging {
                            observed_digest,
                            abort_error,
                        }),
                    };
                }
            }
            Err(error) => {
                return Err(BrowserWriteError::OperationFailed {
                    operation: BrowserWriteOperation::PreCommitVerification,
                    error,
                    abort_error: storage.abort(&writable).await.err(),
                });
            }
        }
        if let Err(error) = storage.close(&writable).await {
            return Err(BrowserWriteError::OperationFailed {
                operation: BrowserWriteOperation::Close,
                error,
                abort_error: storage.abort(&writable).await.err(),
            });
        }
        drop(writable);
        let verified = storage
            .read()
            .await
            .map_err(BrowserWriteError::ReadBackFailed)?;
        let staged_digest = digest_bytes(bytes);
        let observed_digest = digest_bytes(&verified);
        if observed_digest != staged_digest {
            return Err(BrowserWriteError::ReadBackMismatch {
                staged_digest,
                observed_digest,
            });
        }
        Ok(staged_digest)
    }
}

#[cfg(test)]
mod tests;
