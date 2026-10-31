//! The Model Hub's network transports: the typed cloud client behind the
//! synchronous [`ModelHubTransport`] the pipeline drives.
//!
//! The client is unauthenticated by design: a catalog snapshot is an opaque
//! signed blob and a release archive carries its own signature, so nothing
//! either endpoint returns is believed until the pipeline has proved it
//! against the trust anchor.
//!
//! # The browser cannot block, so it primes instead
//!
//! A browser build has no thread to block: `wasm32-unknown-unknown` offers no
//! `block_on`, so a synchronous transport that awaited anything would be a
//! compile error at best and a deadlock at worst. The wasm transport therefore
//! splits acquisition from service: an async priming step fetches the exact
//! bytes a handoff describes into the transport, and the synchronous trait
//! methods hand those bytes to the pipeline. [`crate::state::model_hub::ModelHub`]
//! is unchanged — the same digests are checked in the same order against the
//! same signed snapshot — and the only difference is *when* the bytes arrived.

use rspice_cloud_client::{ArtifactTransferError, CloudError};

#[cfg(target_arch = "wasm32")]
use crate::state::model_hub::PrimingTransport;
use crate::state::model_hub::{
    ArchiveProgress, ModelHubError, ModelHubTransport, ModelHubTransportProvider,
};

/// Opens the cloud transport each Model Hub operation runs through.
///
/// The shell installs it on the session's Model Hub service at startup; it is
/// the only provider a running application has.
#[derive(Debug)]
pub struct CloudModelHubTransportProvider;

impl ModelHubTransportProvider for CloudModelHubTransportProvider {
    #[cfg(not(target_arch = "wasm32"))]
    fn open(&self, progress: ArchiveProgress) -> Result<Box<dyn ModelHubTransport>, String> {
        // The hub pipeline is synchronous, so it needs a runtime whose own threads
        // drive IO — a current-thread runtime cannot be driven through a `Handle`
        // from outside, which is exactly how this worker uses it.
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .map_err(|error| format!("The model-hub transfer runtime could not start: {error}"))?;
        let client = super::CloudAccountService::unauthenticated_client()?;
        let handle = runtime.handle().clone();
        let transport = CloudModelHubTransport::new(client, handle).with_progress(progress);
        Ok(Box::new(native::OwnedRuntimeTransport {
            transport,
            _runtime: runtime,
        }))
    }

    #[cfg(target_arch = "wasm32")]
    fn prime(
        &self,
        catalog: bool,
        archive: Option<(String, String, u64)>,
        progress: ArchiveProgress,
    ) -> PrimingTransport<'_> {
        Box::pin(async move {
            let client = super::CloudAccountService::unauthenticated_client()?;
            let transport = BrowserModelHubTransport::new(client).with_progress(progress);
            if catalog {
                transport
                    .prime_catalog()
                    .await
                    .map_err(|error| error.to_string())?;
            }
            if let Some((pack_id, version, declared)) = archive {
                transport
                    .prime_archive(&pack_id, &version, declared)
                    .await
                    .map_err(|error| error.to_string())?;
            }
            Ok::<Box<dyn ModelHubTransport>, String>(Box::new(transport))
        })
    }
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

#[cfg(target_arch = "wasm32")]
pub use browser::BrowserModelHubTransport;

/// The browser transport: the same typed cloud client, primed before use.
///
/// Every fetch happens in an async priming call the caller awaits inside its
/// own `spawn_local` task; the trait methods then serve what was primed and
/// refuse anything else as
/// [`ModelHubError::HandoffExpired`](crate::state::model_hub::ModelHubError::HandoffExpired).
/// That refusal is the whole safety story of this shape: the pipeline can
/// never be handed bytes that no priming step actually retrieved.
#[cfg(target_arch = "wasm32")]
mod browser {
    use std::cell::RefCell;

    use rspice_cloud_client::{CloudClient, PackId, PackVersion, contract::ArtifactDownload};

    use super::{classify, classify_transfer};

    use crate::state::model_hub::ModelHubError;
    use crate::state::model_hub::transport::{
        ArchiveHandoff, ArchiveProgress, CatalogHandoff, ModelHubTransport,
    };

    pub struct BrowserModelHubTransport {
        client: CloudClient,
        primed: RefCell<Primed>,
        progress: Option<ArchiveProgress>,
    }

    #[derive(Default)]
    struct Primed {
        catalog: Option<(CatalogHandoff, Vec<u8>)>,
        archive: Option<(ArchiveHandoff, Vec<u8>)>,
    }

    impl std::fmt::Debug for BrowserModelHubTransport {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("BrowserModelHubTransport")
        }
    }

    impl BrowserModelHubTransport {
        pub fn new(client: CloudClient) -> Self {
            Self {
                client,
                primed: RefCell::new(Primed::default()),
                progress: None,
            }
        }

        /// Reports archive transfer progress to a caller that shows it.
        #[must_use]
        pub fn with_progress(mut self, progress: ArchiveProgress) -> Self {
            self.progress = Some(progress);
            self
        }

        /// Resolves the catalog handoff and downloads the snapshot it names.
        ///
        /// The declared length bounds the read, so a service that promises a
        /// small snapshot cannot deliver an unbounded one. Nothing here is
        /// believed: the digest and the signature are both settled later, by
        /// the pipeline, against the trust anchor.
        pub async fn prime_catalog(&self) -> Result<CatalogHandoff, ModelHubError> {
            let response = self.client.model_catalog().await.map_err(classify)?;
            let body = response.into_body();
            let handoff = CatalogHandoff {
                generation: body.generation,
                content_length: body.content_length,
                content_sha256: body.content_sha256.clone(),
            };
            let bytes = fetch_capability(&body.download_url, handoff.content_length).await?;
            self.primed.borrow_mut().catalog = Some((handoff.clone(), bytes));
            Ok(handoff)
        }

        /// Resolves one release's download capability and streams the archive.
        ///
        /// `declared_length` is what the signed catalog says this release's
        /// archive weighs, and it is the only denominator a progress report
        /// here is allowed to use. Zero means this session has no snapshot yet
        /// — an install that had to fetch the catalog in the same breath —
        /// and reports nothing rather than a fraction of an unknown.
        pub async fn prime_archive(
            &self,
            pack_id: &str,
            version: &str,
            declared_length: u64,
        ) -> Result<ArchiveHandoff, ModelHubError> {
            let pack = PackId::new(pack_id)
                .map_err(|_| ModelHubError::MalformedRelease("pack identifier"))?;
            let release = PackVersion::new(version)
                .map_err(|_| ModelHubError::MalformedRelease("release version"))?;
            let response = self
                .client
                .model_pack_download(pack, release)
                .await
                .map_err(classify)?;
            let download: ArtifactDownload = response.into_body();
            let handoff = ArchiveHandoff {
                content_length: download.content_length,
                content_sha256: download.content_sha256.clone(),
            };
            let mut buffer = Vec::with_capacity(
                usize::try_from(handoff.content_length)
                    .unwrap_or_default()
                    .min(
                        // A promised length is a claim, not an allocation order.
                        MAX_PRIMED_BYTES,
                    ),
            );
            let progress = self.progress.clone();
            self.client
                .download_artifact_with(&download, |chunk| {
                    buffer.extend_from_slice(&chunk);
                    if let Some(progress) = progress.as_ref()
                        && declared_length > 0
                    {
                        progress(buffer.len() as u64, declared_length);
                    }
                    std::future::ready(Ok::<(), std::convert::Infallible>(()))
                })
                .await
                .map_err(classify_transfer)?;
            self.primed.borrow_mut().archive = Some((handoff.clone(), buffer));
            Ok(handoff)
        }
    }

    /// Ceiling on one primed body. A browser session holds it in linear
    /// memory, so the bound is a resource decision rather than a trust one.
    const MAX_PRIMED_BYTES: usize = 256 * 1024 * 1024;

    impl ModelHubTransport for BrowserModelHubTransport {
        fn catalog_handoff(&self) -> Result<CatalogHandoff, ModelHubError> {
            self.primed
                .borrow()
                .catalog
                .as_ref()
                .map(|(handoff, _)| handoff.clone())
                .ok_or(ModelHubError::HandoffExpired)
        }

        fn fetch_catalog(&self, handoff: &CatalogHandoff) -> Result<Vec<u8>, ModelHubError> {
            match self.primed.borrow().catalog.as_ref() {
                Some((primed, bytes)) if primed == handoff => Ok(bytes.clone()),
                _ => Err(ModelHubError::HandoffExpired),
            }
        }

        fn archive_handoff(
            &self,
            _pack_id: &str,
            _version: &str,
        ) -> Result<ArchiveHandoff, ModelHubError> {
            self.primed
                .borrow()
                .archive
                .as_ref()
                .map(|(handoff, _)| handoff.clone())
                .ok_or(ModelHubError::HandoffExpired)
        }

        fn fetch_archive(&self, handoff: &ArchiveHandoff) -> Result<Vec<u8>, ModelHubError> {
            match self.primed.borrow().archive.as_ref() {
                Some((primed, bytes)) if primed == handoff => Ok(bytes.clone()),
                _ => Err(ModelHubError::HandoffExpired),
            }
        }
    }

    /// Fetches one presigned capability URL through the browser's own stack.
    ///
    /// The snapshot is not an artifact, so the client's transfer path does not
    /// describe it. A plain fetch is safe for the same reason it is on the
    /// desktop: the bytes are an opaque blob until the pipeline proves them
    /// against the handoff digest and then against the trust anchor. Redirects
    /// are refused so the request goes exactly where the service said.
    async fn fetch_capability(url: &str, limit: u64) -> Result<Vec<u8>, ModelHubError> {
        use wasm_bindgen::JsCast as _;

        let window =
            web_sys::window().ok_or_else(|| transport("the browser window is unavailable"))?;
        let init = web_sys::RequestInit::new();
        init.set_method("GET");
        init.set_redirect(web_sys::RequestRedirect::Error);
        init.set_credentials(web_sys::RequestCredentials::Omit);
        let request = web_sys::Request::new_with_str_and_init(url, &init)
            .map_err(|_| transport("the catalog capability is not a fetchable request"))?;
        let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| ModelHubError::Offline)?
            .dyn_into::<web_sys::Response>()
            .map_err(|_| transport("the catalog fetch did not answer with a response"))?;
        if !response.ok() {
            return Err(ModelHubError::Transport(format!(
                "catalog storage answered HTTP {}",
                response.status()
            )));
        }
        let buffer = wasm_bindgen_futures::JsFuture::from(
            response
                .array_buffer()
                .map_err(|_| transport("the catalog response has no readable body"))?,
        )
        .await
        .map_err(|_| ModelHubError::Offline)?;
        let view = js_sys::Uint8Array::new(&buffer);
        let length = view.length() as u64;
        if length > limit {
            return Err(ModelHubError::LengthMismatch {
                expected: limit,
                actual: length,
            });
        }
        let mut bytes = vec![0_u8; view.length() as usize];
        view.copy_to(&mut bytes);
        Ok(bytes)
    }

    fn transport(detail: &str) -> ModelHubError {
        ModelHubError::Transport(detail.to_owned())
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::CloudModelHubTransport;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::state::model_hub::ModelHubError;
    use crate::state::model_hub::transport::{
        ArchiveHandoff, ArchiveProgress, CatalogHandoff, ModelHubTransport,
    };

    use rspice_cloud_client::{CloudClient, PackId, PackVersion, contract::ArtifactDownload};

    use super::{classify, classify_transfer};

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
        progress: Option<ArchiveProgress>,
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
                progress: None,
            }
        }

        /// Reports archive transfer progress to a caller that shows it.
        #[must_use]
        pub fn with_progress(mut self, progress: ArchiveProgress) -> Self {
            self.progress = Some(progress);
            self
        }
    }

    /// A transport and the runtime that drives it, owned together.
    ///
    /// Fields drop in declaration order, so the runtime outlives the transport
    /// that drives it: dropping it first would leave a client whose executor is
    /// already gone.
    pub(super) struct OwnedRuntimeTransport {
        pub(super) transport: CloudModelHubTransport,
        pub(super) _runtime: tokio::runtime::Runtime,
    }

    impl ModelHubTransport for OwnedRuntimeTransport {
        fn catalog_handoff(&self) -> Result<CatalogHandoff, ModelHubError> {
            self.transport.catalog_handoff()
        }

        fn fetch_catalog(&self, handoff: &CatalogHandoff) -> Result<Vec<u8>, ModelHubError> {
            self.transport.fetch_catalog(handoff)
        }

        fn archive_handoff(
            &self,
            pack_id: &str,
            version: &str,
        ) -> Result<ArchiveHandoff, ModelHubError> {
            self.transport.archive_handoff(pack_id, version)
        }

        fn fetch_archive(&self, handoff: &ArchiveHandoff) -> Result<Vec<u8>, ModelHubError> {
            self.transport.fetch_archive(handoff)
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
            let total = handoff.content_length;
            let progress = self.progress.clone();
            let transfer =
                self.runtime
                    .block_on(self.client.download_artifact_with(&download, |chunk| {
                        buffer.extend_from_slice(&chunk);
                        if let Some(progress) = progress.as_ref() {
                            progress(buffer.len() as u64, total);
                        }
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
    /// nothing about it is believed until [`crate::state::model_hub::ModelHub`] has
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
}
