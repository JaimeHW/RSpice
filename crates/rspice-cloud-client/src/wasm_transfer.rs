//! Browser direct-object artifact transfer implementation.

use std::{cell::Cell, future::Future, rc::Rc};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use gloo_timers::callback::Timeout;
use http::StatusCode;
use rspice_cloud_contract::{ArtifactDownload, ArtifactUpload};
use sha2::{Digest as _, Sha256};
use url::Url;
use wasm_bindgen::JsCast as _;
use wasm_bindgen_futures::JsFuture;
use wasm_streams::ReadableStream;

use futures_util::StreamExt as _;

use crate::{
    ArtifactTransferError, ArtifactTransferFailure, ArtifactTransferReceipt, CloudClient,
    MAX_ARTIFACT_SINK_CHUNK_BYTES,
    artifacts::{valid_transfer_artifact_download, valid_transfer_artifact_upload},
    transfer::{expected_digest, require_fresh_capability},
    wasm_transport::{fetch_with_request, timeout_millis},
};

const TRANSFER_ACCEPT: &str = "application/octet-stream";

impl CloudClient {
    /// Verifies and uploads an immutable browser [`web_sys::Blob`] without
    /// buffering the complete artifact in WebAssembly memory.
    ///
    /// The blob stream is first counted and hashed in Rust. Fetch then streams
    /// the same immutable blob through the exact checksum-bound headers. The
    /// method makes one object request and exposes no capability URL in errors.
    pub async fn upload_artifact_from_blob(
        &self,
        upload: &ArtifactUpload,
        source: &web_sys::Blob,
    ) -> Result<ArtifactTransferReceipt, ArtifactTransferError> {
        if !valid_transfer_artifact_upload(
            upload,
            self.config.mode,
            &self.config.object_storage_origin,
        ) {
            return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
        }
        if source.size() != upload.artifact.content_length as f64 {
            return Err(transfer_error(
                ArtifactTransferFailure::SourceLengthMismatch,
            ));
        }
        let observed = hash_blob(source, upload.artifact.content_length).await?;
        let expected = expected_digest(&upload.artifact.content_sha256)?;
        if observed != expected {
            return Err(transfer_error(
                ArtifactTransferFailure::SourceDigestMismatch,
            ));
        }

        let Some(expires_at) = upload.upload_expires_at.as_deref() else {
            return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
        };
        require_fresh_capability(expires_at)?;
        let Some(upload_url) = upload.upload_url.as_deref() else {
            return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
        };
        let url = Url::parse(upload_url)
            .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidHandoff))?;
        let headers = web_sys::Headers::new()
            .map_err(|_| transfer_error(ArtifactTransferFailure::Request))?;
        let Some(upload_headers) = upload.upload_headers.as_ref() else {
            return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
        };
        for (name, value) in upload_headers {
            headers
                .append(name, value)
                .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidHandoff))?;
        }
        let transfer = BrowserTransfer::start(self.config.transfer_timeout)?;
        let init = hardened_request_init("PUT", &headers);
        init.set_signal(Some(&transfer.controller.signal()));
        init.set_body_opt_blob(Some(source));
        let request = web_sys::Request::new_with_str_and_init(url.as_str(), &init)
            .map_err(|_| transfer_error(ArtifactTransferFailure::Request))?;
        let response = transfer.send(&request).await?;
        if let Err(error) = validate_upload_response(&response, &url, &STANDARD.encode(expected)) {
            transfer.controller.abort();
            return Err(error);
        }

        Ok(ArtifactTransferReceipt::verified(
            upload.artifact.id,
            upload.artifact.content_length,
            upload.artifact.content_sha256.clone(),
        ))
    }

    /// Streams a browser artifact download through an asynchronous chunk sink.
    ///
    /// The sink must quarantine chunks until this method returns a receipt;
    /// an error means all previously delivered chunks are untrusted and must
    /// be discarded. This permits OPFS, File System Access, IndexedDB, or
    /// simulator-owned sinks without buffering the complete artifact in WASM.
    pub async fn download_artifact_with<F, Fut, SinkError>(
        &self,
        download: &ArtifactDownload,
        mut sink: F,
    ) -> Result<ArtifactTransferReceipt, ArtifactTransferError>
    where
        F: FnMut(Vec<u8>) -> Fut,
        Fut: Future<Output = Result<(), SinkError>>,
    {
        if !valid_transfer_artifact_download(
            download,
            self.config.mode,
            &self.config.object_storage_origin,
        ) {
            return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
        }
        require_fresh_capability(&download.download_expires_at)?;
        let url = Url::parse(&download.download_url)
            .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidHandoff))?;
        let headers = web_sys::Headers::new()
            .map_err(|_| transfer_error(ArtifactTransferFailure::Request))?;
        headers
            .append("accept", TRANSFER_ACCEPT)
            .map_err(|_| transfer_error(ArtifactTransferFailure::Request))?;
        let transfer = BrowserTransfer::start(self.config.transfer_timeout)?;
        let init = hardened_request_init("GET", &headers);
        init.set_signal(Some(&transfer.controller.signal()));
        let request = web_sys::Request::new_with_str_and_init(url.as_str(), &init)
            .map_err(|_| transfer_error(ArtifactTransferFailure::Request))?;
        let response = transfer.send(&request).await?;
        let status = match validate_download_response(&response, &url, download.content_length) {
            Ok(status) => status,
            Err(error) => {
                transfer.controller.abort();
                return Err(error);
            }
        };
        let raw_stream = response.body().ok_or_else(|| {
            transfer.controller.abort();
            ArtifactTransferError::new(ArtifactTransferFailure::Body, Some(status))
        })?;
        let expected = expected_digest(&download.content_sha256)?;
        let mut stream = ReadableStream::from_raw(raw_stream).into_stream();
        let mut digest = Sha256::new();
        let mut received = 0_u64;
        while let Some(chunk) = stream.next().await {
            if transfer.timed_out.get() {
                return Err(ArtifactTransferError::new(
                    ArtifactTransferFailure::Timeout,
                    Some(status),
                ));
            }
            let chunk = chunk
                .map_err(|_| {
                    transfer.controller.abort();
                    ArtifactTransferError::new(
                        if transfer.timed_out.get() {
                            ArtifactTransferFailure::Timeout
                        } else {
                            ArtifactTransferFailure::Body
                        },
                        Some(status),
                    )
                })?
                .dyn_into::<js_sys::Uint8Array>()
                .map_err(|_| {
                    transfer.controller.abort();
                    ArtifactTransferError::new(ArtifactTransferFailure::Body, Some(status))
                })?;
            let chunk_length =
                usize::try_from(chunk.length()).expect("WebAssembly Uint8Array length fits usize");
            received = checked_received(received, chunk_length, download.content_length).map_err(
                |failure| {
                    transfer.controller.abort();
                    ArtifactTransferError::new(failure, Some(status))
                },
            )?;
            for (start, end) in chunk_segments(chunk.length()) {
                let segment = chunk.subarray(start, end);
                let mut bytes =
                    vec![0_u8; usize::try_from(end - start).expect("segment fits usize")];
                segment.copy_to(&mut bytes);
                digest.update(&bytes);
                sink(bytes).await.map_err(|_| {
                    transfer.controller.abort();
                    ArtifactTransferError::new(ArtifactTransferFailure::Sink, Some(status))
                })?;
            }
        }
        if transfer.timed_out.get() {
            return Err(ArtifactTransferError::new(
                ArtifactTransferFailure::Timeout,
                Some(status),
            ));
        }
        verify_received(received, digest, download.content_length, expected)
            .map_err(|failure| ArtifactTransferError::new(failure, Some(status)))?;
        Ok(ArtifactTransferReceipt::verified(
            download.artifact_id,
            download.content_length,
            download.content_sha256.clone(),
        ))
    }
}

struct BrowserTransfer {
    controller: web_sys::AbortController,
    timed_out: Rc<Cell<bool>>,
    _timeout: Timeout,
}

impl BrowserTransfer {
    fn start(timeout: std::time::Duration) -> Result<Self, ArtifactTransferError> {
        let controller = web_sys::AbortController::new()
            .map_err(|_| transfer_error(ArtifactTransferFailure::Request))?;
        let timed_out = Rc::new(Cell::new(false));
        let timeout_flag = Rc::clone(&timed_out);
        let timeout_controller = controller.clone();
        let timeout = Timeout::new(timeout_millis(timeout), move || {
            timeout_flag.set(true);
            timeout_controller.abort();
        });
        Ok(Self {
            controller,
            timed_out,
            _timeout: timeout,
        })
    }

    async fn send(
        &self,
        request: &web_sys::Request,
    ) -> Result<web_sys::Response, ArtifactTransferError> {
        let response = JsFuture::from(fetch_with_request(request))
            .await
            .map_err(|_| {
                transfer_error(if self.timed_out.get() {
                    ArtifactTransferFailure::Timeout
                } else {
                    ArtifactTransferFailure::Request
                })
            })?;
        response
            .dyn_into::<web_sys::Response>()
            .map_err(|_| transfer_error(ArtifactTransferFailure::Other))
    }
}

fn hardened_request_init(method: &str, headers: &web_sys::Headers) -> web_sys::RequestInit {
    let init = web_sys::RequestInit::new();
    init.set_method(method);
    init.set_headers_headers(headers);
    init.set_mode(web_sys::RequestMode::Cors);
    init.set_credentials(web_sys::RequestCredentials::Omit);
    init.set_cache(web_sys::RequestCache::NoStore);
    init.set_redirect(web_sys::RequestRedirect::Error);
    init.set_referrer_policy(web_sys::ReferrerPolicy::NoReferrer);
    init
}

async fn hash_blob(
    source: &web_sys::Blob,
    expected_length: u64,
) -> Result<[u8; 32], ArtifactTransferError> {
    let mut stream = ReadableStream::from_raw(source.stream()).into_stream();
    let mut digest = Sha256::new();
    let mut observed = 0_u64;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk
            .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidSource))?
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidSource))?;
        let chunk_length =
            usize::try_from(chunk.length()).expect("WebAssembly Uint8Array length fits usize");
        observed = checked_received(observed, chunk_length, expected_length)
            .map_err(|_| transfer_error(ArtifactTransferFailure::SourceLengthMismatch))?;
        for (start, end) in chunk_segments(chunk.length()) {
            let segment = chunk.subarray(start, end);
            let mut bytes = vec![0_u8; usize::try_from(end - start).expect("segment fits usize")];
            segment.copy_to(&mut bytes);
            digest.update(&bytes);
        }
    }
    if observed != expected_length {
        return Err(transfer_error(
            ArtifactTransferFailure::SourceLengthMismatch,
        ));
    }
    Ok(digest.finalize().into())
}

fn validate_upload_response(
    response: &web_sys::Response,
    requested: &Url,
    expected_checksum: &str,
) -> Result<(), ArtifactTransferError> {
    let status = validate_response_status(response, requested)?;
    if !exact_header(
        &response.headers(),
        "x-amz-checksum-sha256",
        expected_checksum,
    ) {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::UploadAcknowledgementMismatch,
            Some(status),
        ));
    }
    Ok(())
}

fn validate_response_status(
    response: &web_sys::Response,
    requested: &Url,
) -> Result<u16, ArtifactTransferError> {
    let status = response.status();
    if response.redirected()
        || !same_url(&response.url(), requested)
        || (300..400).contains(&status)
    {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::RedirectedResponse,
            Some(status),
        ));
    }
    if status != StatusCode::OK.as_u16() {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::UnexpectedStatus,
            Some(status),
        ));
    }
    Ok(status)
}

fn validate_download_response(
    response: &web_sys::Response,
    requested: &Url,
    expected_length: u64,
) -> Result<u16, ArtifactTransferError> {
    let status = validate_response_status(response, requested)?;
    let headers = response.headers();
    let valid_policy = exact_header(&headers, "cache-control", "private, no-store")
        && exact_header(&headers, "content-disposition", "attachment")
        && exact_header(&headers, "content-type", "application/octet-stream")
        && absent_header(&headers, "content-encoding");
    if !valid_policy {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::InvalidResponseMetadata,
            Some(status),
        ));
    }
    let declared_length = headers
        .get("content-length")
        .ok()
        .flatten()
        .and_then(|value| value.parse::<u64>().ok());
    if declared_length != Some(expected_length) {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::ResponseLengthMismatch,
            Some(status),
        ));
    }
    Ok(status)
}

fn exact_header(headers: &web_sys::Headers, name: &str, expected: &str) -> bool {
    headers
        .get(name)
        .ok()
        .flatten()
        .is_some_and(|value| value == expected)
}

fn absent_header(headers: &web_sys::Headers, name: &str) -> bool {
    headers.get(name).is_ok_and(|value| value.is_none())
}

fn same_url(observed: &str, expected: &Url) -> bool {
    Url::parse(observed).is_ok_and(|observed| observed == *expected)
}

fn chunk_segments(length: u32) -> impl Iterator<Item = (u32, u32)> {
    let maximum =
        u32::try_from(MAX_ARTIFACT_SINK_CHUNK_BYTES).expect("artifact sink chunk bound fits u32");
    (0..length)
        .step_by(MAX_ARTIFACT_SINK_CHUNK_BYTES)
        .map(move |start| {
            let end = start.saturating_add(maximum).min(length);
            (start, end)
        })
}

fn checked_received(
    received: u64,
    chunk_length: usize,
    expected_length: u64,
) -> Result<u64, ArtifactTransferFailure> {
    let chunk_length =
        u64::try_from(chunk_length).map_err(|_| ArtifactTransferFailure::ResponseLengthMismatch)?;
    let received = received
        .checked_add(chunk_length)
        .ok_or(ArtifactTransferFailure::ResponseLengthMismatch)?;
    if received > expected_length {
        return Err(ArtifactTransferFailure::ResponseLengthMismatch);
    }
    Ok(received)
}

fn verify_received(
    received: u64,
    digest: Sha256,
    expected_length: u64,
    expected_digest: [u8; 32],
) -> Result<(), ArtifactTransferFailure> {
    if received != expected_length {
        return Err(ArtifactTransferFailure::ResponseLengthMismatch);
    }
    let observed: [u8; 32] = digest.finalize().into();
    if observed != expected_digest {
        return Err(ArtifactTransferFailure::ResponseDigestMismatch);
    }
    Ok(())
}

const fn transfer_error(failure: ArtifactTransferFailure) -> ArtifactTransferError {
    ArtifactTransferError::new(failure, None)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    fn blob(bytes: &[u8]) -> web_sys::Blob {
        let parts = js_sys::Array::new();
        parts.push(&js_sys::Uint8Array::from(bytes).into());
        web_sys::Blob::new_with_u8_array_sequence(&parts).expect("test Blob")
    }

    #[wasm_bindgen_test]
    async fn blob_hashing_executes_through_the_javascript_stream_boundary() {
        let bytes = b"rspice-browser-artifact";
        let observed = hash_blob(
            &blob(bytes),
            u64::try_from(bytes.len()).expect("fixture length fits u64"),
        )
        .await
        .expect("hash immutable Blob");
        let expected: [u8; 32] = Sha256::digest(bytes).into();

        assert_eq!(observed, expected);
        let mismatch = hash_blob(&blob(bytes), 1)
            .await
            .expect_err("declared Blob length must remain authoritative");
        assert_eq!(
            mismatch.failure(),
            ArtifactTransferFailure::SourceLengthMismatch
        );
    }

    #[wasm_bindgen_test]
    fn browser_request_construction_enforces_the_transfer_policy() {
        let headers = web_sys::Headers::new().expect("test headers");
        headers
            .set("accept", TRANSFER_ACCEPT)
            .expect("set test Accept header");
        let init = hardened_request_init("GET", &headers);
        let request =
            web_sys::Request::new_with_str_and_init("https://objects.rspice.test/artifact", &init)
                .expect("construct hardened test request");

        assert_eq!(request.method(), "GET");
        assert_eq!(request.mode(), web_sys::RequestMode::Cors);
        assert_eq!(request.credentials(), web_sys::RequestCredentials::Omit);
        assert_eq!(request.cache(), web_sys::RequestCache::NoStore);
        assert_eq!(request.redirect(), web_sys::RequestRedirect::Error);
        assert_eq!(
            request.referrer_policy(),
            web_sys::ReferrerPolicy::NoReferrer
        );
        assert_eq!(
            request.headers().get("accept").expect("read test header"),
            Some(TRANSFER_ACCEPT.to_owned())
        );
    }

    #[wasm_bindgen_test]
    fn browser_headers_reject_ambiguity_and_visible_content_coding() {
        let headers = web_sys::Headers::new().expect("test headers");
        headers
            .append("cache-control", "private, no-store")
            .expect("append exact cache policy");
        assert!(exact_header(&headers, "cache-control", "private, no-store"));
        headers
            .append("cache-control", "private, no-store")
            .expect("append duplicate cache policy");
        assert!(!exact_header(
            &headers,
            "cache-control",
            "private, no-store"
        ));

        assert!(absent_header(&headers, "content-encoding"));
        headers
            .set("content-encoding", "gzip")
            .expect("set visible content coding");
        assert!(!absent_header(&headers, "content-encoding"));
    }

    #[wasm_bindgen_test]
    fn webassembly_clock_supports_capability_freshness_checks() {
        require_fresh_capability("9999-12-31T23:59:59Z")
            .expect("WebAssembly wall clock admits a far-future capability");
    }
}
