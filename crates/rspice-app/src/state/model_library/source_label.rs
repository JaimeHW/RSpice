//! Shared display spelling for model-source digests.

/// A digest, shortened for display.
///
/// One rendering, because a digest shown two ways in two panes reads as two
/// digests. Head and tail are both kept so a shared prefix stays visible.
#[must_use]
pub fn short_digest(digest: &str) -> String {
    if digest.len() <= 12 {
        digest.to_owned()
    } else {
        format!("{}…{}", &digest[..8], &digest[digest.len() - 4..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_app_types::product::ContentDigest;

    #[test]
    fn one_digest_rendering_keeps_the_head_and_the_tail() {
        let digest = ContentDigest::from_bytes([0xab; 32]).to_string();
        let short = short_digest(&digest);
        assert!(short.starts_with(&digest[..8]));
        assert!(short.ends_with(&digest[digest.len() - 4..]));
        assert_eq!(short_digest("short"), "short");
    }
}
