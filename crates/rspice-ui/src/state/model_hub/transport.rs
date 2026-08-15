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

#[cfg(not(target_arch = "wasm32"))]
pub use native::CloudModelHubTransport;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::{ArchiveHandoff, CatalogHandoff, ModelHubError, ModelHubTransport};

    use rspice_cloud_client::{
        ArtifactTransferError, CloudClient, CloudError, PackId, PackVersion,
        contract::ArtifactDownload,
    };

    /// The production transport: the typed cloud client, driven on a runtime
    /// the caller owns.
    ///
    /// It holds the two handoffs it has already resolved so the download step
    /// can present the capability the service issued without asking again. A
    /// capability is short lived, so the pair is refreshed per operation
    /// rather than cached across them.
    pub struct CloudModelHubTransport {
        client: CloudClient,
        runtime: tokio::runtime::Handle,
        pending: std::sync::Mutex<Pending>,
    }

    #[derive(Default)]
    struct Pending {
        catalog: Option<(CatalogHandoff, String)>,
        archive: Option<(ArchiveHandoff, ArtifactDownload)>,
    }

    impl CloudModelHubTransport {
        pub fn new(client: CloudClient, runtime: tokio::runtime::Handle) -> Self {
            Self {
                client,
                runtime,
                pending: std::sync::Mutex::new(Pending::default()),
            }
        }
    }

    impl ModelHubTransport for CloudModelHubTransport {
        fn catalog_handoff(&self) -> Result<CatalogHandoff, ModelHubError> {
            let response = self
                .runtime
                .block_on(self.client.model_catalog())
                .map_err(classify)?;
            let body = response.into_body();
            let handoff = CatalogHandoff {
                generation: body.generation,
                content_length: body.content_length,
                content_sha256: body.content_sha256.clone(),
            };
            if let Ok(mut pending) = self.pending.lock() {
                pending.catalog = Some((handoff.clone(), body.download_url));
            }
            Ok(handoff)
        }

        fn fetch_catalog(&self, handoff: &CatalogHandoff) -> Result<Vec<u8>, ModelHubError> {
            let url = {
                let pending = self.pending.lock().map_err(|_| {
                    ModelHubError::Transport("transport state is poisoned".to_owned())
                })?;
                match pending.catalog.as_ref() {
                    Some((resolved, url)) if resolved == handoff => url.clone(),
                    _ => return Err(ModelHubError::HandoffExpired),
                }
            };
            self.runtime
                .block_on(fetch_capability(&url, handoff.content_length))
        }

        fn archive_handoff(
            &self,
            pack_id: &str,
            version: &str,
        ) -> Result<ArchiveHandoff, ModelHubError> {
            let pack_id = PackId::new(pack_id)
                .map_err(|_| ModelHubError::MalformedRelease("pack identifier"))?;
            let version = PackVersion::new(version)
                .map_err(|_| ModelHubError::MalformedRelease("release version"))?;
            let response = self
                .runtime
                .block_on(self.client.model_pack_download(pack_id, version))
                .map_err(classify)?;
            let body = response.into_body();
            let handoff = ArchiveHandoff {
                content_length: body.content_length,
                content_sha256: body.content_sha256.clone(),
            };
            if let Ok(mut pending) = self.pending.lock() {
                pending.archive = Some((handoff.clone(), body));
            }
            Ok(handoff)
        }

        fn fetch_archive(&self, handoff: &ArchiveHandoff) -> Result<Vec<u8>, ModelHubError> {
            let download = {
                let pending = self.pending.lock().map_err(|_| {
                    ModelHubError::Transport("transport state is poisoned".to_owned())
                })?;
                match pending.archive.as_ref() {
                    Some((resolved, download)) if resolved == handoff => download.clone(),
                    _ => return Err(ModelHubError::HandoffExpired),
                }
            };
            let mut buffer =
                Vec::with_capacity(usize::try_from(handoff.content_length).unwrap_or(0));
            let transfer =
                self.runtime
                    .block_on(self.client.download_artifact_with(&download, |chunk| {
                        buffer.extend_from_slice(&chunk);
                        std::future::ready(Ok::<(), std::convert::Infallible>(()))
                    }));
            transfer.map_err(classify_transfer)?;
            Ok(buffer)
        }
    }

    /// Fetches one presigned capability URL, refusing anything longer than the
    /// length the handoff declared.
    ///
    /// The catalog snapshot is not an artifact, so the client's artifact
    /// transfer path does not describe it and there is nothing in that crate
    /// to reuse. A plain GET is nonetheless safe here for the reason the
    /// contract states outright: the snapshot is an opaque signed blob, and
    /// nothing about it is believed until [`super::super::ModelHub`] has
    /// proved the bytes against the handoff digest and then against the trust
    /// anchor's signature. Redirects and proxies are refused so the request
    /// goes exactly where the service said and nowhere else.
    async fn fetch_capability(url: &str, limit: u64) -> Result<Vec<u8>, ModelHubError> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .build()
            .map_err(|error| ModelHubError::Transport(error.to_string()))?;
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|_| ModelHubError::Offline)?;
        if !response.status().is_success() {
            return Err(ModelHubError::Transport(format!(
                "catalog storage answered HTTP {}",
                response.status().as_u16()
            )));
        }
        let bytes = response.bytes().await.map_err(|_| ModelHubError::Offline)?;
        if bytes.len() as u64 > limit {
            return Err(ModelHubError::LengthMismatch {
                expected: limit,
                actual: bytes.len() as u64,
            });
        }
        Ok(bytes.to_vec())
    }

    fn classify(error: CloudError) -> ModelHubError {
        match error {
            CloudError::Transport { .. } => ModelHubError::Offline,
            CloudError::Problem { details, .. } => ModelHubError::Rejected(details.title.clone()),
            other => ModelHubError::Transport(format!("{other}")),
        }
    }

    fn classify_transfer(error: ArtifactTransferError) -> ModelHubError {
        use rspice_cloud_client::ArtifactTransferFailure;

        match error.failure() {
            ArtifactTransferFailure::Connect
            | ArtifactTransferFailure::Timeout
            | ArtifactTransferFailure::Request
            | ArtifactTransferFailure::Body => ModelHubError::Offline,
            ArtifactTransferFailure::ResponseDigestMismatch
            | ArtifactTransferFailure::ResponseLengthMismatch => ModelHubError::DigestMismatch {
                expected: "the digest the handoff declared".to_owned(),
                actual: "the bytes object storage returned".to_owned(),
            },
            other => ModelHubError::Transport(format!("{other:?}")),
        }
    }
}
