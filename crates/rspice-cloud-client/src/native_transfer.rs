use std::{future::Future, io::ErrorKind, path::Path};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use http::{
    HeaderMap, HeaderName, HeaderValue, Method, StatusCode,
    header::{
        ACCEPT, CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING, CONTENT_LENGTH, CONTENT_TYPE,
    },
};
use reqwest::Response;
use rspice_cloud_contract::{ArtifactDownload, ArtifactUpload};
use sha2::{Digest as _, Sha256};
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncReadExt as _, AsyncSeekExt as _, AsyncWriteExt as _, SeekFrom},
};
use tokio_util::io::ReaderStream;
use url::Url;

use crate::{
    ArtifactTransferError, ArtifactTransferFailure, ArtifactTransferReceipt, CloudClient,
    MAX_ARTIFACT_SINK_CHUNK_BYTES,
    artifacts::{valid_transfer_artifact_download, valid_transfer_artifact_upload},
    transfer::{expected_digest, require_fresh_capability},
};

const TRANSFER_ACCEPT: &str = "application/octet-stream";

impl CloudClient {
    /// Verifies and streams one regular local file through a checksum-bound
    /// direct-upload capability without buffering the complete artifact.
    ///
    /// The source is read once to prove its exact length and SHA-256, rewound,
    /// and then streamed through the returned signed headers. The method makes
    /// one network attempt and never logs or returns the capability URL.
    pub async fn upload_artifact_from_path(
        &self,
        upload: &ArtifactUpload,
        source: impl AsRef<Path>,
    ) -> Result<ArtifactTransferReceipt, ArtifactTransferError> {
        if !valid_transfer_artifact_upload(
            upload,
            self.config.mode,
            &self.config.object_storage_origin,
        ) {
            return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
        }
        let source = source.as_ref();
        let source_type = fs::symlink_metadata(source)
            .await
            .map_err(|_| transfer_error(ArtifactTransferFailure::FileSystem))?
            .file_type();
        if !source_type.is_file() || source_type.is_symlink() {
            return Err(transfer_error(ArtifactTransferFailure::InvalidSource));
        }

        let mut file = File::open(source)
            .await
            .map_err(|_| transfer_error(ArtifactTransferFailure::FileSystem))?;
        let metadata = file
            .metadata()
            .await
            .map_err(|_| transfer_error(ArtifactTransferFailure::FileSystem))?;
        if !metadata.is_file() {
            return Err(transfer_error(ArtifactTransferFailure::InvalidSource));
        }
        if metadata.len() != upload.artifact.content_length {
            return Err(transfer_error(
                ArtifactTransferFailure::SourceLengthMismatch,
            ));
        }

        let observed_digest = hash_file(&mut file, upload.artifact.content_length).await?;
        let expected = expected_digest(&upload.artifact.content_sha256)?;
        if observed_digest != expected {
            return Err(transfer_error(
                ArtifactTransferFailure::SourceDigestMismatch,
            ));
        }
        file.seek(SeekFrom::Start(0))
            .await
            .map_err(|_| transfer_error(ArtifactTransferFailure::FileSystem))?;

        let Some(expires_at) = upload.upload_expires_at.as_deref() else {
            return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
        };
        require_fresh_capability(expires_at)?;
        let Some(upload_url) = upload.upload_url.as_deref() else {
            return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
        };
        let url = Url::parse(upload_url)
            .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidHandoff))?;
        let mut headers = upload_headers(upload)?;
        headers.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(&upload.artifact.content_length.to_string())
                .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidHandoff))?,
        );
        let response = self
            .http
            .request(Method::PUT, url.clone())
            .headers(headers)
            .timeout(self.config.transfer_timeout)
            .body(reqwest::Body::wrap_stream(ReaderStream::new(file)))
            .send()
            .await
            .map_err(map_transport)?;
        validate_upload_response(&response, &url, &STANDARD.encode(expected))?;

        Ok(ArtifactTransferReceipt::verified(
            upload.artifact.id,
            upload.artifact.content_length,
            upload.artifact.content_sha256.clone(),
        ))
    }

    /// Streams and verifies a direct artifact download into a new local file.
    ///
    /// The destination is published only after the complete byte count and
    /// SHA-256 pass. Publication uses a same-directory hard link so an existing
    /// destination is never overwritten and no partial file appears there.
    pub async fn download_artifact_to_path(
        &self,
        download: &ArtifactDownload,
        destination: impl AsRef<Path>,
    ) -> Result<ArtifactTransferReceipt, ArtifactTransferError> {
        let destination = destination.as_ref();
        require_absent_destination(destination).await?;
        let mut response = self.start_artifact_download(download).await?;
        let (temporary_path, mut file) = create_temporary_destination(destination)
            .await
            .map_err(|error| error.with_status(StatusCode::OK.as_u16()))?;
        let result = stream_response_to_file(&mut response, download, &mut file)
            .await
            .map_err(|error| error.with_status(StatusCode::OK.as_u16()));
        if result.is_ok() && file.sync_all().await.is_err() {
            drop(file);
            let _ = fs::remove_file(&temporary_path).await;
            return Err(ArtifactTransferError::new(
                ArtifactTransferFailure::FileSystem,
                Some(StatusCode::OK.as_u16()),
            ));
        }
        drop(file);
        if let Err(error) = result {
            let _ = fs::remove_file(&temporary_path).await;
            return Err(error);
        }
        if let Err(error) = fs::hard_link(&temporary_path, destination).await {
            let _ = fs::remove_file(&temporary_path).await;
            return Err(ArtifactTransferError::new(
                if error.kind() == ErrorKind::AlreadyExists {
                    ArtifactTransferFailure::DestinationExists
                } else {
                    ArtifactTransferFailure::FileSystem
                },
                Some(StatusCode::OK.as_u16()),
            ));
        }
        let _ = fs::remove_file(&temporary_path).await;
        Ok(ArtifactTransferReceipt::verified(
            download.artifact_id,
            download.content_length,
            download.content_sha256.clone(),
        ))
    }

    /// Streams a verified artifact through an asynchronous chunk sink.
    ///
    /// The sink must quarantine chunks until this method returns a receipt;
    /// an error means all previously delivered chunks are untrusted and must
    /// be discarded. Chunk buffers are bounded by the HTTP transport and the
    /// complete artifact is never buffered by this client.
    pub async fn download_artifact_with<F, Fut, SinkError>(
        &self,
        download: &ArtifactDownload,
        mut sink: F,
    ) -> Result<ArtifactTransferReceipt, ArtifactTransferError>
    where
        F: FnMut(Vec<u8>) -> Fut,
        Fut: Future<Output = Result<(), SinkError>>,
    {
        let mut response = self.start_artifact_download(download).await?;
        let expected = expected_digest(&download.content_sha256)?;
        let mut digest = Sha256::new();
        let mut received = 0_u64;
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error| map_response_transport(error, StatusCode::OK))?
        {
            received = checked_received(received, chunk.len(), download.content_length)
                .map_err(|error| error.with_status(StatusCode::OK.as_u16()))?;
            digest.update(&chunk);
            for segment in chunk.chunks(MAX_ARTIFACT_SINK_CHUNK_BYTES) {
                sink(segment.to_vec()).await.map_err(|_| {
                    ArtifactTransferError::new(
                        ArtifactTransferFailure::Sink,
                        Some(StatusCode::OK.as_u16()),
                    )
                })?;
            }
        }
        verify_received(received, digest, download.content_length, expected)
            .map_err(|error| error.with_status(StatusCode::OK.as_u16()))?;
        Ok(ArtifactTransferReceipt::verified(
            download.artifact_id,
            download.content_length,
            download.content_sha256.clone(),
        ))
    }

    async fn start_artifact_download(
        &self,
        download: &ArtifactDownload,
    ) -> Result<Response, ArtifactTransferError> {
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
        let response = self
            .http
            .request(Method::GET, url.clone())
            .header(ACCEPT, TRANSFER_ACCEPT)
            .timeout(self.config.transfer_timeout)
            .send()
            .await
            .map_err(map_transport)?;
        validate_download_response(&response, &url, download.content_length)?;
        Ok(response)
    }
}

async fn hash_file(
    file: &mut File,
    expected_length: u64,
) -> Result<[u8; 32], ArtifactTransferError> {
    let mut digest = Sha256::new();
    let mut observed = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .await
            .map_err(|_| transfer_error(ArtifactTransferFailure::FileSystem))?;
        if count == 0 {
            break;
        }
        observed = checked_received(observed, count, expected_length)
            .map_err(|_| transfer_error(ArtifactTransferFailure::SourceLengthMismatch))?;
        digest.update(&buffer[..count]);
    }
    if observed != expected_length {
        return Err(transfer_error(
            ArtifactTransferFailure::SourceLengthMismatch,
        ));
    }
    Ok(digest.finalize().into())
}

fn upload_headers(upload: &ArtifactUpload) -> Result<HeaderMap, ArtifactTransferError> {
    let mut headers = HeaderMap::new();
    let Some(upload_headers) = upload.upload_headers.as_ref() else {
        return Err(transfer_error(ArtifactTransferFailure::InvalidHandoff));
    };
    for (name, value) in upload_headers {
        let name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidHandoff))?;
        let value = HeaderValue::from_str(value)
            .map_err(|_| transfer_error(ArtifactTransferFailure::InvalidHandoff))?;
        headers.insert(name, value);
    }
    Ok(headers)
}

fn validate_upload_response(
    response: &Response,
    requested: &Url,
    expected_checksum: &str,
) -> Result<(), ArtifactTransferError> {
    let status = response.status();
    if response.url() != requested || status.is_redirection() {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::RedirectedResponse,
            Some(status.as_u16()),
        ));
    }
    if status != StatusCode::OK {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::UnexpectedStatus,
            Some(status.as_u16()),
        ));
    }
    if !exact_header(
        response.headers(),
        HeaderName::from_static("x-amz-checksum-sha256"),
        expected_checksum,
    ) {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::UploadAcknowledgementMismatch,
            Some(status.as_u16()),
        ));
    }
    Ok(())
}

fn validate_download_response(
    response: &Response,
    requested: &Url,
    expected_length: u64,
) -> Result<(), ArtifactTransferError> {
    let status = response.status();
    if response.url() != requested || status.is_redirection() {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::RedirectedResponse,
            Some(status.as_u16()),
        ));
    }
    if status != StatusCode::OK {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::UnexpectedStatus,
            Some(status.as_u16()),
        ));
    }
    let headers = response.headers();
    let valid_policy = exact_header(headers, CACHE_CONTROL, "private, no-store")
        && exact_header(headers, CONTENT_DISPOSITION, "attachment")
        && exact_header(headers, CONTENT_TYPE, "application/octet-stream")
        && headers.get_all(CONTENT_ENCODING).iter().next().is_none();
    if !valid_policy {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::InvalidResponseMetadata,
            Some(status.as_u16()),
        ));
    }
    let declared_length = unique_header(headers, CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());
    if declared_length != Some(expected_length) {
        return Err(ArtifactTransferError::new(
            ArtifactTransferFailure::ResponseLengthMismatch,
            Some(status.as_u16()),
        ));
    }
    Ok(())
}

fn exact_header(headers: &HeaderMap, name: http::header::HeaderName, expected: &str) -> bool {
    unique_header(headers, name).is_some_and(|value| value.as_bytes() == expected.as_bytes())
}

fn unique_header(headers: &HeaderMap, name: http::header::HeaderName) -> Option<&HeaderValue> {
    let mut values = headers.get_all(name).iter();
    let value = values.next()?;
    values.next().is_none().then_some(value)
}

async fn require_absent_destination(destination: &Path) -> Result<(), ArtifactTransferError> {
    match fs::symlink_metadata(destination).await {
        Ok(_) => Err(transfer_error(ArtifactTransferFailure::DestinationExists)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(_) => Err(transfer_error(ArtifactTransferFailure::FileSystem)),
    }
}

async fn create_temporary_destination(
    destination: &Path,
) -> Result<(std::path::PathBuf, File), ArtifactTransferError> {
    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    for _ in 0..8 {
        let mut random = [0_u8; 16];
        getrandom::fill(&mut random)
            .map_err(|_| transfer_error(ArtifactTransferFailure::FileSystem))?;
        let random = random
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let path = parent.join(format!(".rspice-download-{random}.part"));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            options.mode(0o600);
        }
        match options.open(&path).await {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(_) => return Err(transfer_error(ArtifactTransferFailure::FileSystem)),
        }
    }
    Err(transfer_error(ArtifactTransferFailure::FileSystem))
}

async fn stream_response_to_file(
    response: &mut Response,
    download: &ArtifactDownload,
    file: &mut File,
) -> Result<(), ArtifactTransferError> {
    let expected = expected_digest(&download.content_sha256)?;
    let mut digest = Sha256::new();
    let mut received = 0_u64;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| map_response_transport(error, StatusCode::OK))?
    {
        received = checked_received(received, chunk.len(), download.content_length)?;
        digest.update(&chunk);
        file.write_all(&chunk)
            .await
            .map_err(|_| transfer_error(ArtifactTransferFailure::FileSystem))?;
    }
    file.flush()
        .await
        .map_err(|_| transfer_error(ArtifactTransferFailure::FileSystem))?;
    verify_received(received, digest, download.content_length, expected)
}

fn checked_received(
    received: u64,
    chunk_length: usize,
    expected_length: u64,
) -> Result<u64, ArtifactTransferError> {
    let chunk_length = u64::try_from(chunk_length)
        .map_err(|_| transfer_error(ArtifactTransferFailure::ResponseLengthMismatch))?;
    let received = received
        .checked_add(chunk_length)
        .ok_or_else(|| transfer_error(ArtifactTransferFailure::ResponseLengthMismatch))?;
    if received > expected_length {
        return Err(transfer_error(
            ArtifactTransferFailure::ResponseLengthMismatch,
        ));
    }
    Ok(received)
}

fn verify_received(
    received: u64,
    digest: Sha256,
    expected_length: u64,
    expected_digest: [u8; 32],
) -> Result<(), ArtifactTransferError> {
    if received != expected_length {
        return Err(transfer_error(
            ArtifactTransferFailure::ResponseLengthMismatch,
        ));
    }
    let observed: [u8; 32] = digest.finalize().into();
    if observed != expected_digest {
        return Err(transfer_error(
            ArtifactTransferFailure::ResponseDigestMismatch,
        ));
    }
    Ok(())
}

fn map_transport(error: reqwest::Error) -> ArtifactTransferError {
    let status = error.status().map(|status| status.as_u16());
    let failure = if error.is_timeout() {
        ArtifactTransferFailure::Timeout
    } else if error.is_connect() {
        ArtifactTransferFailure::Connect
    } else if error.is_request() {
        ArtifactTransferFailure::Request
    } else if error.is_body() {
        ArtifactTransferFailure::Body
    } else {
        ArtifactTransferFailure::Other
    };
    ArtifactTransferError::new(failure, status)
}

fn map_response_transport(error: reqwest::Error, status: StatusCode) -> ArtifactTransferError {
    let failure = map_transport(error).failure();
    ArtifactTransferError::new(failure, Some(status.as_u16()))
}

const fn transfer_error(failure: ArtifactTransferFailure) -> ArtifactTransferError {
    ArtifactTransferError::new(failure, None)
}
