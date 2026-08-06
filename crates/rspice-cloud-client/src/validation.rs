use std::collections::HashSet;

use rspice_cloud_contract::Uuid;
use serde_json::Value;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub(crate) const MAX_CIRCUIT_TITLE_CHARACTERS: usize = 240;
pub(crate) const MAX_REVISION_ARTIFACTS: usize = 100;
pub(crate) const MAX_REVISION_DOCUMENT_BYTES: usize = 1024 * 1024;

pub(crate) fn valid_timestamp_text(value: &str) -> bool {
    parse_timestamp_text(value).is_some()
}

pub(crate) fn parse_timestamp_text(value: &str) -> Option<OffsetDateTime> {
    if value.is_empty() || value.len() > 64 {
        return None;
    }
    let timestamp = OffsetDateTime::parse(value, &Rfc3339).ok()?;
    (timestamp.format(&Rfc3339).ok().as_deref() == Some(value)).then_some(timestamp)
}

pub(crate) fn decode_lower_hex_sha256(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 {
        return None;
    }
    let mut digest = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        digest[index] = (hex_value(pair[0])? << 4) | hex_value(pair[1])?;
    }
    Some(digest)
}

pub(crate) fn valid_clean_circuit_title(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && value.chars().count() <= MAX_CIRCUIT_TITLE_CHARACTERS
        && !value.chars().any(char::is_control)
}

pub(crate) fn valid_json_object(value: &Value, max_bytes: usize) -> bool {
    value.is_object() && serde_json::to_vec(value).is_ok_and(|bytes| bytes.len() <= max_bytes)
}

pub(crate) fn valid_revision_snapshot(
    id: Uuid,
    schema_version: i32,
    content_digest_version: i16,
    document: &Value,
    artifact_ids: &[Uuid],
    content_sha256: &str,
    created_at: &str,
) -> bool {
    !id.is_nil()
        && schema_version > 0
        && matches!(content_digest_version, 1 | 2)
        && valid_json_object(document, MAX_REVISION_DOCUMENT_BYTES)
        && valid_artifact_ids(artifact_ids)
        && decode_lower_hex_sha256(content_sha256).is_some()
        && valid_timestamp_text(created_at)
}

pub(crate) fn valid_revision_input(
    schema_version: u32,
    document: &Value,
    artifact_ids: Option<&[Uuid]>,
) -> bool {
    schema_version > 0
        && i32::try_from(schema_version).is_ok()
        && valid_json_object(document, MAX_REVISION_DOCUMENT_BYTES)
        && valid_artifact_ids(artifact_ids.unwrap_or_default())
}

fn valid_artifact_ids(artifact_ids: &[Uuid]) -> bool {
    artifact_ids.len() <= MAX_REVISION_ARTIFACTS
        && artifact_ids.iter().all(|id| !id.is_nil())
        && artifact_ids.iter().copied().collect::<HashSet<_>>().len() == artifact_ids.len()
}

const fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn timestamps_and_digests_require_canonical_wire_shapes() {
        assert!(valid_timestamp_text("2026-07-19T00:05:00Z"));
        assert!(valid_timestamp_text("2026-07-19T00:05:00.123456Z"));
        assert!(!valid_timestamp_text("not-a-timestamp"));
        assert!(!valid_timestamp_text("2026-07-19 00:05:00Z"));
        assert!(!valid_timestamp_text("2026-07-19T00:05:00+00:00"));

        assert_eq!(decode_lower_hex_sha256(&"0a".repeat(32)), Some([10; 32]));
        assert!(decode_lower_hex_sha256(&"0A".repeat(32)).is_none());
        assert!(decode_lower_hex_sha256(&"0".repeat(63)).is_none());
    }

    #[test]
    fn circuit_titles_and_revision_payloads_are_canonical_and_bounded() {
        assert!(valid_clean_circuit_title("Precision amplifier"));
        assert!(!valid_clean_circuit_title(" Precision amplifier"));
        assert!(!valid_clean_circuit_title("line\nbreak"));

        let revision_id = Uuid::from_u128(1);
        let artifact_id = Uuid::from_u128(2);
        assert!(valid_revision_snapshot(
            revision_id,
            1,
            2,
            &json!({"components": []}),
            &[artifact_id],
            &"0a".repeat(32),
            "2026-07-19T00:00:00Z",
        ));
        assert!(!valid_revision_snapshot(
            revision_id,
            1,
            2,
            &json!({}),
            &[artifact_id, artifact_id],
            &"0a".repeat(32),
            "2026-07-19T00:00:00Z",
        ));
        assert!(valid_revision_input(1, &json!({}), Some(&[artifact_id])));
        assert!(!valid_revision_input(0, &json!({}), None));
    }
}
