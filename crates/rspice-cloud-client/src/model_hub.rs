use std::{error::Error, fmt};

use rspice_cloud_contract::{ArtifactKind, ModelCatalogDownload};
use url::Url;

use crate::{
    EndpointMode,
    artifacts::{
        DOWNLOAD_QUERY_NAMES, DOWNLOAD_SIGNED_HEADERS, MAX_DOWNLOAD_PRESIGN_SECONDS,
        MIN_DOWNLOAD_PRESIGN_SECONDS, valid_download_response_policy, valid_sigv4_capability,
        valid_storage_capability_url,
    },
    validation::{decode_lower_hex_sha256, parse_timestamp_text},
};

/// The pack format's fixed snapshot ceiling, mirrored so an oversized handoff
/// is refused before a single byte is fetched.
const MAX_CATALOG_SNAPSHOT_BYTES: u64 = 32 * 1024 * 1024;
const MAX_PACK_IDENTIFIER_BYTES: usize = 64;

/// Validated, borrowed Model Hub pack identifier used in a hub route path.
///
/// The grammar mirrors the pack format's own `pack.id` rule so a value this
/// client will place in a URL is one the signed manifest could have carried.
/// It is a request-shaping check, never a trust decision: the archive's
/// signature remains the only authority over what a pack actually is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackId<'a>(&'a str);

impl<'a> PackId<'a> {
    /// Validates the exact `rspice-<family>` lowercase kebab-case grammar.
    pub fn new(value: &'a str) -> Result<Self, PackIdError> {
        valid_pack_id(value)
            .then_some(Self(value))
            .ok_or(PackIdError)
    }

    /// Returns the pack identifier text.
    pub const fn as_str(self) -> &'a str {
        self.0
    }

    pub(crate) const fn value(self) -> &'a str {
        self.0
    }
}

/// A value did not match the exact RSpice pack-identifier grammar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackIdError;

impl fmt::Display for PackIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("pack identifier must be rspice-<family> in lowercase kebab case")
    }
}

impl Error for PackIdError {}

/// Validated, borrowed Model Hub release version used in a hub route path.
///
/// The grammar is Semantic Versioning 2.0.0, mirroring the pack format's own
/// release rule for the same reason [`PackId`] mirrors its identifier rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackVersion<'a>(&'a str);

impl<'a> PackVersion<'a> {
    /// Validates the exact Semantic Versioning 2.0.0 grammar.
    pub fn new(value: &'a str) -> Result<Self, PackVersionError> {
        valid_semantic_version(value)
            .then_some(Self(value))
            .ok_or(PackVersionError)
    }

    /// Returns the release version text.
    pub const fn as_str(self) -> &'a str {
        self.0
    }

    pub(crate) const fn value(self) -> &'a str {
        self.0
    }
}

/// A value did not match the exact Semantic Versioning 2.0.0 grammar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PackVersionError;

impl fmt::Display for PackVersionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("pack version must be a Semantic Versioning 2.0.0 release")
    }
}

impl Error for PackVersionError {}

/// Proves a catalog handoff is self-verifying before the snapshot is fetched.
///
/// The handoff carries no catalog content, so the only facts to check are the
/// ones that let the caller reject the wrong bytes without asking the service
/// again: a real generation, a bounded exact length, an exact digest, and a
/// download capability that points at this deployment's object storage under
/// the same inert, short-lived, non-cacheable policy every artifact uses.
pub(crate) fn valid_model_catalog_download(
    download: &ModelCatalogDownload,
    mode: EndpointMode,
    object_storage_origin: &Url,
) -> bool {
    download.generation > 0
        && (1..=MAX_CATALOG_SNAPSHOT_BYTES).contains(&download.content_length)
        && decode_lower_hex_sha256(&download.content_sha256).is_some()
        && parse_timestamp_text(&download.signed_at).is_some()
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

/// The classification a Model Hub archive download must report. The hub stores
/// every `.rspicepack` as a model library, so a handoff of any other kind is
/// the wrong object regardless of how well formed it is.
pub(crate) const PACK_ARCHIVE_KIND: ArtifactKind = ArtifactKind::ModelLibrary;

fn valid_pack_id(value: &str) -> bool {
    let family = value.strip_prefix("rspice-").unwrap_or_default();
    !family.is_empty() && valid_kebab(value)
}

fn valid_kebab(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_PACK_IDENTIFIER_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn valid_semantic_version(value: &str) -> bool {
    if value.is_empty() || value.len() > MAX_PACK_IDENTIFIER_BYTES || !value.is_ascii() {
        return false;
    }
    let boundary = value
        .bytes()
        .position(|byte| matches!(byte, b'-' | b'+'))
        .unwrap_or(value.len());
    let (core, remainder) = value.split_at(boundary);
    let mut numbers = core.split('.');
    for _ in 0..3 {
        let Some(number) = numbers.next() else {
            return false;
        };
        if !valid_numeric_identifier(number) {
            return false;
        }
    }
    if numbers.next().is_some() {
        return false;
    }
    let (prerelease, build) = match remainder.strip_prefix('-') {
        Some(rest) => match rest.split_once('+') {
            Some((prerelease, build)) => (Some(prerelease), Some(build)),
            None => (Some(rest), None),
        },
        None => (None, remainder.strip_prefix('+')),
    };
    prerelease.is_none_or(|prerelease| {
        prerelease.split('.').all(|identifier| {
            valid_version_identifier(identifier)
                && (!identifier.bytes().all(|byte| byte.is_ascii_digit())
                    || valid_numeric_identifier(identifier))
        })
    }) && build.is_none_or(|build| build.split('.').all(valid_version_identifier))
}

fn valid_version_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
}

fn valid_numeric_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOWNLOAD_SECRET: &str =
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn object_storage_origin() -> Url {
        Url::parse("http://127.0.0.1:9000").expect("loopback object storage origin")
    }

    fn catalog_url() -> String {
        format!(
            "http://127.0.0.1:9000/model-hub/catalog/7.rspicecat?\
             X-Amz-Algorithm=AWS4-HMAC-SHA256&\
             X-Amz-Credential=TESTACCESS%2F20260815%2Fus-east-1%2Fs3%2Faws4_request&\
             X-Amz-Date=20260815T000000Z&\
             X-Amz-Expires=300&\
             X-Amz-SignedHeaders=host&\
             response-cache-control=private%2C%20no-store&\
             response-content-disposition=attachment&\
             response-content-type=application%2Foctet-stream&\
             X-Amz-Signature={DOWNLOAD_SECRET}"
        )
    }

    fn catalog() -> ModelCatalogDownload {
        ModelCatalogDownload {
            generation: 7,
            content_length: 4_096,
            content_sha256: "0a".repeat(32),
            signed_at: "2026-08-15T00:00:00Z".to_owned(),
            download_url: catalog_url(),
            download_expires_at: "2026-08-15T00:05:00Z".to_owned(),
        }
    }

    #[test]
    fn pack_identifiers_and_versions_use_the_exact_format_grammar() {
        for accepted in ["rspice-opamps", "rspice-bjt-small-signal", "rspice-a1"] {
            assert!(PackId::new(accepted).is_ok(), "{accepted:?} is a pack id");
        }
        for refused in [
            "",
            "rspice-",
            "opamps",
            "RSPICE-OPAMPS",
            "rspice--opamps",
            "rspice-opamps-",
            "rspice-op/amps",
            "rspice-op amps",
            "../secrets",
        ] {
            assert!(
                PackId::new(refused).is_err(),
                "{refused:?} is not a pack id"
            );
        }

        for accepted in [
            "1.0.0",
            "0.1.0",
            "1.2.3-rc.1",
            "1.2.3+build.5",
            "1.2.3-rc.1+b",
        ] {
            assert!(
                PackVersion::new(accepted).is_ok(),
                "{accepted:?} is a version"
            );
        }
        for refused in [
            "",
            "1.0",
            "1.0.0.0",
            "01.0.0",
            "1.0.0-",
            "1.0.0-01",
            "1.0.0+",
            "v1.0.0",
            "1.0.0/../secrets",
            "latest",
        ] {
            assert!(
                PackVersion::new(refused).is_err(),
                "{refused:?} is not a version"
            );
        }
    }

    #[test]
    fn a_catalog_handoff_must_be_bounded_exact_and_capability_shaped() {
        assert!(valid_model_catalog_download(
            &catalog(),
            EndpointMode::LoopbackDevelopment,
            &object_storage_origin(),
        ));

        let unpublished = ModelCatalogDownload {
            generation: 0,
            ..catalog()
        };
        assert!(!valid_model_catalog_download(
            &unpublished,
            EndpointMode::LoopbackDevelopment,
            &object_storage_origin(),
        ));

        let unbounded = ModelCatalogDownload {
            content_length: MAX_CATALOG_SNAPSHOT_BYTES + 1,
            ..catalog()
        };
        assert!(!valid_model_catalog_download(
            &unbounded,
            EndpointMode::LoopbackDevelopment,
            &object_storage_origin(),
        ));

        let empty = ModelCatalogDownload {
            content_length: 0,
            ..catalog()
        };
        assert!(!valid_model_catalog_download(
            &empty,
            EndpointMode::LoopbackDevelopment,
            &object_storage_origin(),
        ));

        let uppercase_digest = ModelCatalogDownload {
            content_sha256: "0A".repeat(32),
            ..catalog()
        };
        assert!(!valid_model_catalog_download(
            &uppercase_digest,
            EndpointMode::LoopbackDevelopment,
            &object_storage_origin(),
        ));

        let foreign_origin = ModelCatalogDownload {
            download_url: catalog_url().replace("127.0.0.1:9000", "objects.invalid"),
            ..catalog()
        };
        assert!(!valid_model_catalog_download(
            &foreign_origin,
            EndpointMode::LoopbackDevelopment,
            &object_storage_origin(),
        ));

        let cacheable = ModelCatalogDownload {
            download_url: catalog_url().replace("private%2C%20no-store", "public"),
            ..catalog()
        };
        assert!(!valid_model_catalog_download(
            &cacheable,
            EndpointMode::LoopbackDevelopment,
            &object_storage_origin(),
        ));

        let plaintext_in_production = catalog();
        assert!(!valid_model_catalog_download(
            &plaintext_in_production,
            EndpointMode::Production,
            &object_storage_origin(),
        ));
    }
}
