use std::collections::{BTreeMap, HashSet};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use http::{HeaderName, HeaderValue};
use rspice_cloud_contract::{
    Artifact, ArtifactDownload, ArtifactKind, ArtifactState, ArtifactUpload,
    CreateArtifactUploadRequest, Uuid,
};
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time};
use url::{Host, Url};

use crate::{
    EndpointMode,
    validation::{decode_lower_hex_sha256, parse_timestamp_text},
};

const MAX_ARTIFACT_BYTES: u64 = 53_687_091_200;
const MAX_FILE_NAME_BYTES: usize = 255;
const MAX_CONTENT_TYPE_BYTES: usize = 255;
const MAX_UPLOAD_HEADER_BYTES: usize = 64 * 1024;
const MAX_CAPABILITY_URL_BYTES: usize = 16 * 1024;
const MAX_ACCESS_KEY_ID_BYTES: usize = 256;
const MAX_SESSION_TOKEN_BYTES: usize = 4_096;
const MAX_UPLOAD_PRESIGN_SECONDS: u64 = 7 * 24 * 60 * 60;
const MIN_DOWNLOAD_PRESIGN_SECONDS: u64 = 30;
const MAX_DOWNLOAD_PRESIGN_SECONDS: u64 = 15 * 60;
const SIGV4_ALGORITHM: &str = "AWS4-HMAC-SHA256";
const SIGV4_SERVICE: &str = "s3";
const SIGV4_TERMINATOR: &str = "aws4_request";
const UPLOAD_SIGNED_HEADERS: &str = "content-type;host;x-amz-checksum-sha256";
const DOWNLOAD_SIGNED_HEADERS: &str = "host";
const SIGV4_QUERY_NAMES: [&str; 6] = [
    "X-Amz-Algorithm",
    "X-Amz-Credential",
    "X-Amz-Date",
    "X-Amz-Expires",
    "X-Amz-SignedHeaders",
    "X-Amz-Signature",
];
const DOWNLOAD_QUERY_NAMES: [&str; 3] = [
    "response-cache-control",
    "response-content-disposition",
    "response-content-type",
];

pub(crate) fn valid_artifact(
    artifact: &Artifact,
    expected_id: Option<Uuid>,
    expected_workspace_id: Option<Uuid>,
) -> bool {
    if artifact.id.is_nil()
        || artifact.workspace_id.is_nil()
        || expected_id.is_some_and(|id| artifact.id != id)
        || expected_workspace_id.is_some_and(|id| artifact.workspace_id != id)
        || artifact
            .file_name
            .as_deref()
            .is_some_and(|value| !valid_file_name(value))
        || !valid_content_type(&artifact.content_type)
        || !(1..=MAX_ARTIFACT_BYTES).contains(&artifact.content_length)
        || decode_lower_hex_sha256(&artifact.content_sha256).is_none()
    {
        return false;
    }

    let Some(created_at) = parse_timestamp_text(&artifact.created_at) else {
        return false;
    };
    let Some(updated_at) = parse_timestamp_text(&artifact.updated_at) else {
        return false;
    };
    if updated_at < created_at {
        return false;
    }
    let verified_at = match artifact.verified_at.as_deref() {
        Some(value) => {
            let Some(value) = parse_timestamp_text(value) else {
                return false;
            };
            if value < created_at {
                return false;
            }
            Some(value)
        }
        None => None,
    };

    match artifact.state {
        ArtifactState::Uploading | ArtifactState::Rejected => verified_at.is_none(),
        ArtifactState::Available => verified_at.is_some(),
        ArtifactState::Deleted => true,
    }
}

pub(crate) fn valid_artifact_list(artifacts: &[Artifact], workspace_id: Uuid) -> bool {
    if artifacts.iter().any(|artifact| {
        !valid_artifact(artifact, None, Some(workspace_id))
            || artifact.state == ArtifactState::Deleted
    }) || artifacts
        .iter()
        .map(|artifact| artifact.id)
        .collect::<HashSet<_>>()
        .len()
        != artifacts.len()
    {
        return false;
    }

    artifacts.windows(2).all(|pair| {
        parse_timestamp_text(&pair[0].created_at)
            .zip(parse_timestamp_text(&pair[1].created_at))
            .is_some_and(|(newer, older)| newer >= older)
    })
}

pub(crate) fn artifact_matches_request(
    artifact: &Artifact,
    workspace_id: Uuid,
    request: &CreateArtifactUploadRequest,
) -> bool {
    let normalized_file_name = request.file_name.as_deref().map(str::trim);
    let normalized_content_type = request.content_type.trim().to_ascii_lowercase();
    let normalized_digest = request.content_sha256.trim();
    valid_artifact(artifact, None, Some(workspace_id))
        && normalized_file_name.is_none_or(valid_file_name)
        && valid_content_type(&normalized_content_type)
        && decode_lower_hex_sha256(normalized_digest).is_some()
        && artifact.file_name.as_deref() == normalized_file_name
        && artifact.content_type == normalized_content_type
        && artifact.kind == ArtifactKind::from(request.kind)
        && artifact.content_length == request.content_length
        && artifact.content_sha256 == normalized_digest
}

pub(crate) fn valid_artifact_upload(
    upload: &ArtifactUpload,
    workspace_id: Uuid,
    request: &CreateArtifactUploadRequest,
    replayed: bool,
    mode: EndpointMode,
    object_storage_origin: &Url,
) -> bool {
    if !artifact_matches_request(&upload.artifact, workspace_id, request) {
        return false;
    }

    match (
        upload.upload_url.as_deref(),
        upload.upload_headers.as_ref(),
        upload.upload_expires_at.as_deref(),
    ) {
        (Some(url), Some(headers), Some(expires_at)) => {
            upload.artifact.state == ArtifactState::Uploading
                && valid_storage_capability_url(url, mode, object_storage_origin)
                && valid_upload_headers(headers, &upload.artifact)
                && valid_sigv4_capability(
                    url,
                    expires_at,
                    1,
                    MAX_UPLOAD_PRESIGN_SECONDS,
                    UPLOAD_SIGNED_HEADERS,
                    &[],
                )
                && parse_timestamp_text(expires_at).is_some_and(|expires_at| {
                    parse_timestamp_text(&upload.artifact.created_at)
                        .is_some_and(|created_at| expires_at > created_at)
                })
        }
        (None, None, None) => replayed,
        _ => false,
    }
}

pub(crate) fn valid_artifact_download(
    download: &ArtifactDownload,
    expected_artifact_id: Uuid,
    mode: EndpointMode,
    object_storage_origin: &Url,
) -> bool {
    !download.artifact_id.is_nil()
        && download.artifact_id == expected_artifact_id
        && download.file_name.as_deref().is_none_or(valid_file_name)
        && valid_content_type(&download.content_type)
        && (1..=MAX_ARTIFACT_BYTES).contains(&download.content_length)
        && decode_lower_hex_sha256(&download.content_sha256).is_some()
        && valid_storage_capability_url(&download.download_url, mode, object_storage_origin)
        && valid_download_response_policy(&download.download_url)
        && valid_sigv4_capability(
            &download.download_url,
            &download.download_expires_at,
            MIN_DOWNLOAD_PRESIGN_SECONDS,
            MAX_DOWNLOAD_PRESIGN_SECONDS,
            DOWNLOAD_SIGNED_HEADERS,
            &DOWNLOAD_QUERY_NAMES,
        )
}

pub(crate) fn valid_transfer_artifact_upload(
    upload: &ArtifactUpload,
    mode: EndpointMode,
    object_storage_origin: &Url,
) -> bool {
    use rspice_cloud_contract::ClientUploadArtifactKind;

    let kind = match upload.artifact.kind {
        ArtifactKind::CircuitAttachment => ClientUploadArtifactKind::CircuitAttachment,
        ArtifactKind::ModelLibrary => ClientUploadArtifactKind::ModelLibrary,
        ArtifactKind::PublicationSnapshot => ClientUploadArtifactKind::PublicationSnapshot,
        ArtifactKind::SimulationResult => return false,
    };
    let request = CreateArtifactUploadRequest {
        kind,
        file_name: upload.artifact.file_name.clone(),
        content_type: upload.artifact.content_type.clone(),
        content_length: upload.artifact.content_length,
        content_sha256: upload.artifact.content_sha256.clone(),
    };
    valid_artifact_upload(
        upload,
        upload.artifact.workspace_id,
        &request,
        false,
        mode,
        object_storage_origin,
    )
}

pub(crate) fn valid_transfer_artifact_download(
    download: &ArtifactDownload,
    mode: EndpointMode,
    object_storage_origin: &Url,
) -> bool {
    valid_artifact_download(download, download.artifact_id, mode, object_storage_origin)
}

fn valid_download_response_policy(value: &str) -> bool {
    let Ok(url) = Url::parse(value) else {
        return false;
    };
    exact_query_value(&url, "response-cache-control", "private, no-store")
        && exact_query_value(&url, "response-content-disposition", "attachment")
        && exact_query_value(&url, "response-content-type", "application/octet-stream")
}

fn exact_query_value(url: &Url, name: &str, expected: &str) -> bool {
    let mut values = url
        .query_pairs()
        .filter_map(|(observed_name, value)| (observed_name == name).then_some(value));
    values.next().is_some_and(|value| value == expected) && values.next().is_none()
}

fn valid_sigv4_capability(
    value: &str,
    reported_expires_at: &str,
    minimum_ttl_seconds: u64,
    maximum_ttl_seconds: u64,
    expected_signed_headers: &str,
    additional_query_names: &[&str],
) -> bool {
    let Ok(url) = Url::parse(value) else {
        return false;
    };
    let mut observed_names = HashSet::new();
    for (name, _) in url.query_pairs() {
        if (!SIGV4_QUERY_NAMES.contains(&name.as_ref())
            && name != "X-Amz-Security-Token"
            && !additional_query_names.contains(&name.as_ref()))
            || !observed_names.insert(name.into_owned())
        {
            return false;
        }
    }

    if !exact_query_value(&url, "X-Amz-Algorithm", SIGV4_ALGORITHM)
        || !exact_query_value(&url, "X-Amz-SignedHeaders", expected_signed_headers)
    {
        return false;
    }
    let Some(amz_date) = unique_query_value(&url, "X-Amz-Date") else {
        return false;
    };
    let Some(signed_at) = parse_amz_timestamp(&amz_date) else {
        return false;
    };
    let Some(ttl_text) = unique_query_value(&url, "X-Amz-Expires") else {
        return false;
    };
    let Ok(ttl_seconds) = ttl_text.parse::<u64>() else {
        return false;
    };
    if ttl_text != ttl_seconds.to_string()
        || !(minimum_ttl_seconds..=maximum_ttl_seconds).contains(&ttl_seconds)
    {
        return false;
    }
    let Some(credential) = unique_query_value(&url, "X-Amz-Credential") else {
        return false;
    };
    if !valid_sigv4_credential(&credential, &amz_date) {
        return false;
    }
    let Some(signature) = unique_query_value(&url, "X-Amz-Signature") else {
        return false;
    };
    if decode_lower_hex_sha256(&signature).is_none() || !valid_canonical_presign_query(&url) {
        return false;
    }
    if let Some(session_token) = optional_query_value(&url, "X-Amz-Security-Token")
        && (session_token.is_empty()
            || session_token.len() > MAX_SESSION_TOKEN_BYTES
            || session_token.chars().any(char::is_control))
    {
        return false;
    }

    let Ok(ttl_seconds) = i64::try_from(ttl_seconds) else {
        return false;
    };
    signed_at
        .checked_add(Duration::seconds(ttl_seconds))
        .zip(parse_timestamp_text(reported_expires_at))
        .is_some_and(|(signed_expiry, reported_expiry)| signed_expiry == reported_expiry)
}

fn valid_canonical_presign_query(url: &Url) -> bool {
    let Some(observed_query) = url.query() else {
        return false;
    };
    let Some(signature) = unique_query_value(url, "X-Amz-Signature") else {
        return false;
    };
    let mut parameters = url
        .query_pairs()
        .filter(|(name, _)| name != "X-Amz-Signature")
        .map(|(name, value)| (aws_encode(&name), aws_encode(&value)))
        .collect::<Vec<_>>();
    parameters.sort_unstable();
    let canonical_without_signature = parameters
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join("&");
    observed_query
        == format!(
            "{canonical_without_signature}&X-Amz-Signature={}",
            aws_encode(&signature)
        )
}

fn aws_encode(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            output.push(char::from(byte));
        } else {
            use std::fmt::Write as _;
            write!(&mut output, "%{byte:02X}").expect("writing into a String cannot fail");
        }
    }
    output
}

fn unique_query_value(url: &Url, name: &str) -> Option<String> {
    let mut values = url
        .query_pairs()
        .filter_map(|(observed_name, value)| (observed_name == name).then_some(value.into_owned()));
    let value = values.next()?;
    values.next().is_none().then_some(value)
}

fn optional_query_value(url: &Url, name: &str) -> Option<String> {
    url.query_pairs()
        .find_map(|(observed_name, value)| (observed_name == name).then_some(value.into_owned()))
}

fn valid_sigv4_credential(value: &str, amz_date: &str) -> bool {
    let mut segments = value.split('/');
    let Some(access_key_id) = segments.next() else {
        return false;
    };
    let Some(date) = segments.next() else {
        return false;
    };
    let Some(region) = segments.next() else {
        return false;
    };
    let Some(service) = segments.next() else {
        return false;
    };
    let Some(terminator) = segments.next() else {
        return false;
    };
    !access_key_id.is_empty()
        && access_key_id.len() <= MAX_ACCESS_KEY_ID_BYTES
        && !access_key_id.chars().any(char::is_control)
        && amz_date.get(..8) == Some(date)
        && valid_sigv4_region(region)
        && service == SIGV4_SERVICE
        && terminator == SIGV4_TERMINATOR
        && segments.next().is_none()
}

fn valid_sigv4_region(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
}

fn parse_amz_timestamp(value: &str) -> Option<OffsetDateTime> {
    if value.len() != 16
        || value.as_bytes().get(8) != Some(&b'T')
        || value.as_bytes().get(15) != Some(&b'Z')
    {
        return None;
    }
    let year = value.get(0..4)?.parse::<i32>().ok()?;
    let month = Month::try_from(value.get(4..6)?.parse::<u8>().ok()?).ok()?;
    let day = value.get(6..8)?.parse::<u8>().ok()?;
    let hour = value.get(9..11)?.parse::<u8>().ok()?;
    let minute = value.get(11..13)?.parse::<u8>().ok()?;
    let second = value.get(13..15)?.parse::<u8>().ok()?;
    let date = Date::from_calendar_date(year, month, day).ok()?;
    let time = Time::from_hms(hour, minute, second).ok()?;
    Some(PrimitiveDateTime::new(date, time).assume_utc())
}

fn valid_file_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_FILE_NAME_BYTES
        && value.trim() == value
        && !matches!(value, "." | "..")
        && !value.chars().any(char::is_control)
        && !value.contains(['/', '\\'])
}

fn valid_content_type(value: &str) -> bool {
    if value.len() < 3
        || value.len() > MAX_CONTENT_TYPE_BYTES
        || value != value.to_ascii_lowercase()
    {
        return false;
    }
    let Some((kind, subtype)) = value.split_once('/') else {
        return false;
    };
    !kind.is_empty()
        && !subtype.is_empty()
        && value.matches('/').count() == 1
        && kind.bytes().all(valid_media_token_byte)
        && subtype.bytes().all(valid_media_token_byte)
}

const fn valid_media_token_byte(value: u8) -> bool {
    value.is_ascii_lowercase()
        || value.is_ascii_digit()
        || matches!(
            value,
            b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-'
        )
}

fn valid_storage_capability_url(
    value: &str,
    mode: EndpointMode,
    object_storage_origin: &Url,
) -> bool {
    if value.len() > MAX_CAPABILITY_URL_BYTES {
        return false;
    }
    let Ok(url) = Url::parse(value) else {
        return false;
    };
    if url.cannot_be_a_base()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || url.path() == "/"
    {
        return false;
    }
    let Some(host) = url.host() else {
        return false;
    };
    if url.origin() != object_storage_origin.origin() {
        return false;
    }

    match mode {
        EndpointMode::Production => url.scheme() == "https",
        EndpointMode::LoopbackDevelopment if url.scheme() == "https" => true,
        EndpointMode::LoopbackDevelopment if url.scheme() == "http" => is_loopback_host(host),
        EndpointMode::LoopbackDevelopment => false,
    }
}

fn is_loopback_host(host: Host<&str>) -> bool {
    match host {
        Host::Domain(domain) => domain.eq_ignore_ascii_case("localhost"),
        Host::Ipv4(address) => address.is_loopback(),
        Host::Ipv6(address) => address.is_loopback(),
    }
}

fn valid_upload_headers(headers: &BTreeMap<String, String>, artifact: &Artifact) -> bool {
    if headers.len() != 2 {
        return false;
    }
    let Some(expected_checksum) =
        decode_lower_hex_sha256(&artifact.content_sha256).map(|digest| STANDARD.encode(digest))
    else {
        return false;
    };
    let mut total_bytes = 0_usize;
    let mut normalized_names = HashSet::with_capacity(headers.len());
    let mut has_content_type = false;
    let mut has_checksum = false;
    for (name, value) in headers {
        let Ok(name) = HeaderName::from_bytes(name.as_bytes()) else {
            return false;
        };
        let name_length = name.as_str().len();
        if !normalized_names.insert(name.clone()) || HeaderValue::try_from(value.as_str()).is_err()
        {
            return false;
        }
        let Some(bytes) = total_bytes
            .checked_add(name_length)
            .and_then(|bytes| bytes.checked_add(value.len()))
        else {
            return false;
        };
        total_bytes = bytes;
        match name.as_str() {
            "content-type" => has_content_type = value == &artifact.content_type,
            "x-amz-checksum-sha256" => has_checksum = value == &expected_checksum,
            _ => {}
        }
    }
    total_bytes <= MAX_UPLOAD_HEADER_BYTES && has_content_type && has_checksum
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_cloud_contract::{ClientUploadArtifactKind, CreateArtifactUploadRequest};

    const TEST_SIGNATURE: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    fn upload_capability_url() -> String {
        format!(
            "https://objects.rspice.test/presigned?\
             X-Amz-Algorithm=AWS4-HMAC-SHA256&\
             X-Amz-Credential=TESTACCESS%2F20260719%2Fus-east-1%2Fs3%2Faws4_request&\
             X-Amz-Date=20260719T000000Z&\
             X-Amz-Expires=300&\
             X-Amz-SignedHeaders=content-type%3Bhost%3Bx-amz-checksum-sha256&\
             X-Amz-Signature={TEST_SIGNATURE}"
        )
    }

    fn download_capability_url() -> String {
        format!(
            "https://objects.rspice.test/presigned?\
             X-Amz-Algorithm=AWS4-HMAC-SHA256&\
             X-Amz-Credential=TESTACCESS%2F20260719%2Fus-east-1%2Fs3%2Faws4_request&\
             X-Amz-Date=20260719T000000Z&\
             X-Amz-Expires=300&\
             X-Amz-SignedHeaders=host&\
             response-cache-control=private%2C%20no-store&\
             response-content-disposition=attachment&\
             response-content-type=application%2Foctet-stream&\
             X-Amz-Signature={TEST_SIGNATURE}"
        )
    }

    fn production_object_storage_origin() -> Url {
        Url::parse("https://objects.rspice.test").expect("test object-storage origin")
    }

    fn loopback_object_storage_origin() -> Url {
        Url::parse("http://127.0.0.1:9000").expect("test loopback object-storage origin")
    }

    fn request() -> CreateArtifactUploadRequest {
        CreateArtifactUploadRequest {
            kind: ClientUploadArtifactKind::ModelLibrary,
            file_name: Some("models.lib".to_owned()),
            content_type: "text/plain".to_owned(),
            content_length: 128,
            content_sha256: "00".repeat(32),
        }
    }

    fn upload_fixture() -> ArtifactUpload {
        ArtifactUpload {
            artifact: Artifact {
                id: Uuid::from_u128(1),
                workspace_id: Uuid::from_u128(2),
                kind: ArtifactKind::ModelLibrary,
                state: ArtifactState::Uploading,
                file_name: Some("models.lib".to_owned()),
                content_type: "text/plain".to_owned(),
                content_length: 128,
                content_sha256: "00".repeat(32),
                verified_at: None,
                created_at: "2026-07-19T00:00:00Z".to_owned(),
                updated_at: "2026-07-19T00:00:00Z".to_owned(),
            },
            upload_url: Some(upload_capability_url()),
            upload_headers: Some(BTreeMap::from([
                ("content-type".to_owned(), "text/plain".to_owned()),
                (
                    "x-amz-checksum-sha256".to_owned(),
                    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_owned(),
                ),
            ])),
            upload_expires_at: Some("2026-07-19T00:05:00Z".to_owned()),
        }
    }

    #[test]
    fn artifact_projections_are_structural_chronological_and_lifecycle_safe() {
        let mut artifact = upload_fixture().artifact;
        assert!(valid_artifact(
            &artifact,
            Some(Uuid::from_u128(1)),
            Some(Uuid::from_u128(2))
        ));

        artifact.state = ArtifactState::Available;
        assert!(!valid_artifact(&artifact, None, None));
        artifact.verified_at = Some("2026-07-19T00:00:01Z".to_owned());
        artifact.updated_at = "2026-07-19T00:00:01Z".to_owned();
        assert!(valid_artifact(&artifact, None, None));

        artifact.file_name = Some("../models.lib".to_owned());
        assert!(!valid_artifact(&artifact, None, None));
        artifact.file_name = Some("models.lib".to_owned());
        artifact.content_type = "Text/Plain".to_owned();
        assert!(!valid_artifact(&artifact, None, None));
        artifact.content_type = "text/plain".to_owned();
        artifact.updated_at = "2026-07-18T23:59:59Z".to_owned();
        assert!(!valid_artifact(&artifact, None, None));
    }

    #[test]
    fn artifact_pages_are_workspace_bound_unique_and_newest_first() {
        let newer = upload_fixture().artifact;
        let mut older = newer.clone();
        older.id = Uuid::from_u128(3);
        older.created_at = "2026-07-18T00:00:00Z".to_owned();
        older.updated_at = older.created_at.clone();
        assert!(valid_artifact_list(
            &[newer.clone(), older.clone()],
            Uuid::from_u128(2)
        ));
        assert!(!valid_artifact_list(
            &[older, newer.clone()],
            Uuid::from_u128(2)
        ));
        assert!(!valid_artifact_list(
            &[newer.clone(), newer],
            Uuid::from_u128(2)
        ));
    }

    #[test]
    fn upload_capabilities_are_atomic_bounded_and_transport_safe() {
        let request = request();
        let mut upload = upload_fixture();
        assert!(valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        upload.upload_headers = None;
        assert!(!valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        upload = upload_fixture();
        upload
            .upload_headers
            .as_mut()
            .expect("upload headers")
            .insert("x-amz-checksum-sha256".to_owned(), "AAAA".to_owned());
        assert!(!valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        upload.upload_headers = None;
        upload.upload_url = None;
        upload.upload_expires_at = None;
        assert!(!valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            true,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        upload = upload_fixture();
        upload.upload_headers = Some(BTreeMap::from([
            ("Content-Type".to_owned(), "text/plain".to_owned()),
            ("content-type".to_owned(), "text/plain".to_owned()),
        ]));
        assert!(!valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        upload.upload_headers = Some(BTreeMap::from([(
            "x-amz-meta-unsafe".to_owned(),
            "line one\r\nline two".to_owned(),
        )]));
        assert!(!valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        upload = upload_fixture();
        upload.artifact.file_name = Some("wrong.lib".to_owned());
        assert!(!valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
    }

    #[test]
    fn capability_urls_reject_plaintext_remote_hosts_and_embedded_credentials() {
        assert!(valid_storage_capability_url(
            "https://objects.rspice.test/object?signature=secret",
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(!valid_storage_capability_url(
            "http://objects.rspice.test/object?signature=secret",
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(!valid_storage_capability_url(
            "http://objects.rspice.test/object?signature=secret",
            EndpointMode::LoopbackDevelopment,
            &loopback_object_storage_origin(),
        ));
        assert!(valid_storage_capability_url(
            "http://127.0.0.1:9000/object?signature=secret",
            EndpointMode::LoopbackDevelopment,
            &loopback_object_storage_origin(),
        ));
        assert!(!valid_storage_capability_url(
            "https://user:secret@objects.rspice.test/object",
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(!valid_storage_capability_url(
            "https://objects.rspice.test/object#secret",
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(!valid_storage_capability_url(
            "https://objects.rspice.test/?signature=secret",
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(!valid_storage_capability_url(
            &format!(
                "https://objects.rspice.test/object?value={}",
                "a".repeat(MAX_CAPABILITY_URL_BYTES)
            ),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(!valid_storage_capability_url(
            "https://other-objects.rspice.test/object?signature=secret",
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(!valid_storage_capability_url(
            "https://objects.rspice.test:8443/object?signature=secret",
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        assert!(valid_storage_capability_url(
            "https://objects.rspice.test:443/object?signature=secret",
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
    }

    #[test]
    fn download_capabilities_require_exact_inert_non_cacheable_response_policy() {
        let safe = ArtifactDownload {
            artifact_id: Uuid::from_u128(2),
            kind: ArtifactKind::ModelLibrary,
            file_name: Some("models.lib".to_owned()),
            content_type: "text/plain".to_owned(),
            content_length: 128,
            content_sha256: "01".repeat(32),
            download_url: download_capability_url(),
            download_expires_at: "2026-07-19T00:05:00Z".to_owned(),
        };
        assert!(valid_artifact_download(
            &safe,
            Uuid::from_u128(2),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        let mut wrong_artifact = safe.clone();
        wrong_artifact.artifact_id = Uuid::from_u128(3);
        assert!(!valid_artifact_download(
            &wrong_artifact,
            Uuid::from_u128(2),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        let mut invalid_metadata = safe.clone();
        invalid_metadata.file_name = Some("../models.lib".to_owned());
        assert!(!valid_artifact_download(
            &invalid_metadata,
            Uuid::from_u128(2),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        invalid_metadata = safe.clone();
        invalid_metadata.content_type = "Text/Plain".to_owned();
        assert!(!valid_artifact_download(
            &invalid_metadata,
            Uuid::from_u128(2),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        invalid_metadata = safe.clone();
        invalid_metadata.content_length = 0;
        assert!(!valid_artifact_download(
            &invalid_metadata,
            Uuid::from_u128(2),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        invalid_metadata = safe.clone();
        invalid_metadata.content_sha256 = "AA".repeat(32);
        assert!(!valid_artifact_download(
            &invalid_metadata,
            Uuid::from_u128(2),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        for unsafe_url in [
            safe.download_url
                .replace("response-cache-control=private%2C%20no-store&", ""),
            safe.download_url.replace("private%2C%20no-store", "public"),
            safe.download_url.replace("attachment", "inline"),
            safe.download_url
                .replace("application%2Foctet-stream", "text%2Fhtml"),
            safe.download_url.replace(
                "response-content-disposition=attachment&",
                "response-content-disposition=attachment&response-content-disposition=inline&",
            ),
        ] {
            let mut unsafe_download = safe.clone();
            unsafe_download.download_url = unsafe_url;
            assert!(!valid_artifact_download(
                &unsafe_download,
                Uuid::from_u128(2),
                EndpointMode::Production,
                &production_object_storage_origin(),
            ));
        }
    }

    #[test]
    fn capability_envelopes_bind_expiry_signature_scope_and_required_headers() {
        let request = request();
        let upload = upload_fixture();
        assert!(valid_artifact_upload(
            &upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        for unsafe_url in [
            upload_capability_url().replace("X-Amz-Expires=300", "X-Amz-Expires=0300"),
            upload_capability_url().replace("20260719%2F", "20260718%2F"),
            upload_capability_url().replace("TESTACCESS%2F20260719", "INVALID%2FACCESS%2F20260719"),
            upload_capability_url().replace("%2Fs3%2F", "%2Fec2%2F"),
            upload_capability_url().replace(
                "content-type%3Bhost%3Bx-amz-checksum-sha256",
                "content-type%3Bhost",
            ),
            upload_capability_url().replace(TEST_SIGNATURE, &"A".repeat(64)),
            format!("{}&X-Amz-Date=20260719T000000Z", upload_capability_url()),
            format!("{}&unexpected=value", upload_capability_url()),
            upload_capability_url().replace("%2F20260719", "%2f20260719"),
            upload_capability_url().replace(
                "X-Amz-Date=20260719T000000Z&X-Amz-Expires=300",
                "X-Amz-Expires=300&X-Amz-Date=20260719T000000Z",
            ),
        ] {
            let mut unsafe_upload = upload.clone();
            unsafe_upload.upload_url = Some(unsafe_url);
            assert!(!valid_artifact_upload(
                &unsafe_upload,
                Uuid::from_u128(2),
                &request,
                false,
                EndpointMode::Production,
                &production_object_storage_origin(),
            ));
        }

        let mut overstated_upload = upload.clone();
        overstated_upload.upload_expires_at = Some("2026-07-19T00:05:01Z".to_owned());
        assert!(!valid_artifact_upload(
            &overstated_upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        let mut extra_header_upload = upload;
        extra_header_upload
            .upload_headers
            .as_mut()
            .expect("upload headers")
            .insert("x-amz-meta-unsafe".to_owned(), "value".to_owned());
        assert!(!valid_artifact_upload(
            &extra_header_upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        let mut temporary_credential_upload = upload_fixture();
        temporary_credential_upload.upload_url =
            temporary_credential_upload.upload_url.map(|url| {
                url.replace(
                    "X-Amz-SignedHeaders=",
                    "X-Amz-Security-Token=temporary%2Ftoken%2Bvalue%3D&X-Amz-SignedHeaders=",
                )
            });
        assert!(valid_artifact_upload(
            &temporary_credential_upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        temporary_credential_upload.upload_url = temporary_credential_upload
            .upload_url
            .map(|url| url.replace("temporary%2Ftoken%2Bvalue%3D", "unsafe%0Atoken"));
        assert!(!valid_artifact_upload(
            &temporary_credential_upload,
            Uuid::from_u128(2),
            &request,
            false,
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));

        let download = ArtifactDownload {
            artifact_id: Uuid::from_u128(2),
            kind: ArtifactKind::ModelLibrary,
            file_name: Some("models.lib".to_owned()),
            content_type: "text/plain".to_owned(),
            content_length: 128,
            content_sha256: "01".repeat(32),
            download_url: download_capability_url(),
            download_expires_at: "2026-07-19T00:05:00Z".to_owned(),
        };
        let mut overstated_download = download.clone();
        overstated_download.download_expires_at = "2026-07-19T00:05:01Z".to_owned();
        assert!(!valid_artifact_download(
            &overstated_download,
            Uuid::from_u128(2),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
        let mut long_lived_download = download.clone();
        long_lived_download.download_url = long_lived_download
            .download_url
            .replace("X-Amz-Expires=300", "X-Amz-Expires=901");
        long_lived_download.download_expires_at = "2026-07-19T00:15:01Z".to_owned();
        assert!(!valid_artifact_download(
            &long_lived_download,
            Uuid::from_u128(2),
            EndpointMode::Production,
            &production_object_storage_origin(),
        ));
    }
}
