//! The network edge of the Model Hub, and what a handoff is allowed to say.
//!
//! Everything above this trait works on bytes that have already been proved
//! against a signature, so the transport is deliberately the least trusted
//! part of the runtime: it may choose *which* bytes arrive and it may fail,
//! but it cannot decide what they mean. That is why the handoff types here
//! carry only length, digest, and a fetch capability — a transport that lied
//! about a pack's contents would have to produce a valid signature to be
//! believed, and it does not hold the key.
//!
//! The trait is synchronous because the pipeline that drives it is: the
//! caller owns the thread or task, exactly as the cloud-account executor owns
//! its runtime, and the runtime here stays testable without one.
//!
//! # The network is handed in
//!
//! Nothing here names a network stack. The transports that reach the service
//! belong to the cloud account; the shell hands the session's Model Hub
//! service a [`ModelHubTransportProvider`] when it starts, and an operation
//! opens its transport through that provider.

use super::ModelHubError;

/// What the service says about the current catalog snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogHandoff {
    /// Monotonic snapshot identity. A client already holding this generation
    /// can skip the download entirely.
    pub generation: u64,
    pub content_length: u64,
    /// Lowercase hexadecimal SHA-256 of the exact snapshot bytes.
    pub content_sha256: String,
}

/// What the service says about one published pack release archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveHandoff {
    pub content_length: u64,
    /// Lowercase hexadecimal SHA-256 of the exact archive bytes.
    pub content_sha256: String,
}

/// Fetches Model Hub handoffs and the bytes they describe.
///
/// An implementation returns [`ModelHubError::Offline`] when it cannot reach
/// the service. That is a first-class outcome, not a failure: a client with a
/// cached snapshot and installed packs works entirely without this trait.
pub trait ModelHubTransport {
    fn catalog_handoff(&self) -> Result<CatalogHandoff, ModelHubError>;

    /// Downloads the snapshot the handoff describes.
    fn fetch_catalog(&self, handoff: &CatalogHandoff) -> Result<Vec<u8>, ModelHubError>;

    fn archive_handoff(
        &self,
        pack_id: &str,
        version: &str,
    ) -> Result<ArchiveHandoff, ModelHubError>;

    /// Downloads the archive the handoff describes.
    fn fetch_archive(&self, handoff: &ArchiveHandoff) -> Result<Vec<u8>, ModelHubError>;
}

/// A transport that is always offline.
///
/// This is the shape of a build with no network at all, and the fixture the
/// offline tests run against: with it in place, everything the runtime can
/// still do is exactly what it can do from disk.
#[derive(Debug, Clone, Copy, Default)]
pub struct OfflineTransport;

impl ModelHubTransport for OfflineTransport {
    fn catalog_handoff(&self) -> Result<CatalogHandoff, ModelHubError> {
        Err(ModelHubError::Offline)
    }

    fn fetch_catalog(&self, _handoff: &CatalogHandoff) -> Result<Vec<u8>, ModelHubError> {
        Err(ModelHubError::Offline)
    }

    fn archive_handoff(
        &self,
        _pack_id: &str,
        _version: &str,
    ) -> Result<ArchiveHandoff, ModelHubError> {
        Err(ModelHubError::Offline)
    }

    fn fetch_archive(&self, _handoff: &ArchiveHandoff) -> Result<Vec<u8>, ModelHubError> {
        Err(ModelHubError::Offline)
    }
}

/// Proves received bytes are the exact bytes a handoff described.
///
/// The pipeline runs this even though the cloud client verifies its own
/// transfers: the check costs one hash of bytes already in memory, and it
/// makes the guarantee a property of this runtime rather than of whichever
/// transport happens to be installed.
pub(crate) fn require_exact_bytes(
    bytes: &[u8],
    expected_length: u64,
    expected_sha256: &str,
) -> Result<(), ModelHubError> {
    if bytes.len() as u64 != expected_length {
        return Err(ModelHubError::LengthMismatch {
            expected: expected_length,
            actual: bytes.len() as u64,
        });
    }
    let actual = rspice_pack::sha256_hex(bytes);
    if actual != expected_sha256 {
        return Err(ModelHubError::DigestMismatch {
            expected: expected_sha256.to_owned(),
            actual,
        });
    }
    Ok(())
}

/// Called with `(received, total)` as an archive arrives.
///
/// The total is the length the *signed snapshot* declared, which the
/// pipeline has already matched against the handoff, so a progress bar
/// driven by it cannot be stretched by a service that overstates a size.
#[cfg(not(target_arch = "wasm32"))]
pub type ArchiveProgress = std::sync::Arc<dyn Fn(u64, u64) + Send + Sync>;

/// Called with `(received, declared)` as an archive arrives.
///
/// `Rc` rather than `Arc`: a browser task runs on the one thread the
/// application has, and asking for a `Send + Sync` closure here would be a
/// bound nothing on this target can satisfy without pretending.
///
/// The declared total is the length the *signed snapshot* publishes, handed
/// in by the caller. It is deliberately not `content_length` from the
/// download handoff: the pipeline has not compared the two yet at priming
/// time, so a service that overstated a size could otherwise stretch a
/// progress bar into looking like a stalled transfer.
#[cfg(target_arch = "wasm32")]
pub type ArchiveProgress = std::rc::Rc<dyn Fn(u64, u64)>;

/// Opens the transport one Model Hub operation runs through.
///
/// A transport is opened per operation because a download capability is
/// short lived and because a session that never installs a pack should never
/// have opened a network client at all.
pub trait ModelHubTransportProvider: std::fmt::Debug + Sync {
    /// The transport one operation drives from its worker thread, reporting
    /// archive progress to `progress`.
    #[cfg(not(target_arch = "wasm32"))]
    fn open(&self, progress: ArchiveProgress) -> Result<Box<dyn ModelHubTransport>, String>;

    /// A transport primed with the bytes one operation will read: the
    /// catalog when `catalog` is set, then the release archive `archive`
    /// names as `(pack id, version, declared length)`.
    ///
    /// A browser has no thread to block, so every fetch happens in this
    /// awaited call and the transport then serves only what it primed.
    #[cfg(target_arch = "wasm32")]
    fn prime(
        &self,
        catalog: bool,
        archive: Option<(String, String, u64)>,
        progress: ArchiveProgress,
    ) -> PrimingTransport<'_>;
}

/// The awaited result of [`ModelHubTransportProvider::prime`].
#[cfg(target_arch = "wasm32")]
pub type PrimingTransport<'a> = std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Box<dyn ModelHubTransport>, String>> + 'a>,
>;
