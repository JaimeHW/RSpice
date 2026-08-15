use crate::error::PackError;

/// Longest archive path accepted, in bytes.
pub const MAX_PATH_BYTES: usize = 255;

/// The archive entry holding the signed document.
pub const MANIFEST_ENTRY: &str = "manifest.json";
/// The archive entry holding the raw 64-byte ed25519 signature.
pub const SIGNATURE_ENTRY: &str = "signature.ed25519";

/// Accepts only paths that name a file, relative to the archive root, in one
/// unambiguous way.
///
/// The charset is deliberately narrower than the ZIP format allows: with no
/// backslash, colon, or non-ASCII byte there is no drive letter, UNC prefix,
/// alternate data stream, or Unicode-normalization ambiguity to resolve, and
/// with no empty or dot components there is no traversal or self-reference to
/// normalize. A trailing separator is refused outright because the container
/// lists files only — directories are implied by the paths.
pub fn validate_path(path: &str) -> Result<(), PackError> {
    let illegal = || PackError::IllegalPath(path.to_owned());
    if path.is_empty() || path.len() > MAX_PATH_BYTES {
        return Err(illegal());
    }
    if !path
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'/' | b'-'))
    {
        return Err(illegal());
    }
    if path.starts_with('/') || path.ends_with('/') {
        return Err(illegal());
    }
    for component in path.split('/') {
        if component.is_empty() || component == "." || component == ".." {
            return Err(illegal());
        }
    }
    Ok(())
}

/// Returns whether a path names one of the two entries the container owns.
#[must_use]
pub fn is_reserved_path(path: &str) -> bool {
    path == MANIFEST_ENTRY || path == SIGNATURE_ENTRY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_model_paths_are_accepted() {
        for path in ["models/opamps.lib", "LICENSE", "a/b/c-1_2.lib", "NOTICE"] {
            validate_path(path).expect("path is within the pack rules");
        }
    }

    #[test]
    fn traversal_absolute_and_windows_shapes_are_refused() {
        for path in [
            "",
            "../x",
            "a/../b",
            "./a",
            "/a",
            "a/",
            "a//b",
            "a\\b",
            "C:/a",
            "mod\u{e9}ls/a.lib",
            "a b",
            "a*b",
        ] {
            assert!(
                matches!(validate_path(path), Err(PackError::IllegalPath(_))),
                "{path:?} must be refused"
            );
        }
    }

    #[test]
    fn overlong_paths_are_refused() {
        let path = "a".repeat(MAX_PATH_BYTES + 1);
        assert!(matches!(
            validate_path(&path),
            Err(PackError::IllegalPath(_))
        ));
    }
}
