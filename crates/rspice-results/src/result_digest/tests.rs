//! Canonical payload tags remain unique across independently added analyses.

/// Every typed payload arm opens with a tag no other arm uses.
///
/// Two lanes added a payload on the same day and each took "the next unused
/// tag" from its own base: both wrote 11, nothing conflicted textually, and
/// every gate passed. The tags are read out of the encoder's own source so the
/// list cannot fall behind it.
#[test]
fn every_typed_payload_digests_under_its_own_tag() {
    let source = include_str!("../result_digest.rs");
    let mut tags: Vec<(u32, String)> = Vec::new();
    let mut arm: Option<String> = None;
    for line in source.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("AnalysisResultPayload::") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            arm = Some(name);
        } else if let (Some(name), Some(rest)) = (arm.as_ref(), line.strip_prefix("writer.u8("))
            && let Some(tag) = rest
                .strip_suffix(");")
                .and_then(|tag| tag.parse::<u32>().ok())
        {
            tags.push((tag, name.clone()));
            arm = None;
        }
    }
    assert!(
        tags.len() >= 14,
        "the scan stopped finding the payload arms it is here to compare: {tags:?}"
    );
    let mut seen = std::collections::BTreeMap::new();
    for (tag, name) in &tags {
        if let Some(first) = seen.insert(*tag, name.clone()) {
            assert_eq!(
                &first, name,
                "payload tag {tag} is written by both {first} and {name}"
            );
        }
    }
}
