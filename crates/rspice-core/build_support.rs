use std::borrow::Cow;
use std::path::Path;

pub(crate) fn verify_declared_generated_file<'a>(
    relative_path: &str,
    checkout_bytes: &'a [u8],
    declared_bytes: u64,
    declared_blake3: &str,
) -> Result<Cow<'a, [u8]>, String> {
    let bytes = canonical_generated_text_bytes(Path::new(relative_path), checkout_bytes);
    if bytes.len() as u64 != declared_bytes {
        return Err(format!(
            "'{relative_path}' has {} canonical bytes; manifest declares {declared_bytes}",
            bytes.len()
        ));
    }
    let digest = blake3::hash(bytes.as_ref()).to_hex().to_string();
    if digest != declared_blake3 {
        return Err(format!(
            "'{relative_path}' canonical digest is {digest}; manifest declares {declared_blake3}"
        ));
    }
    Ok(bytes)
}

/// Return the repository-canonical bytes for generated source artifacts.
///
/// Every declared generated artifact is text pinned to LF by `.gitattributes`.
/// A Windows checkout that was materialized before that attribute took effect
/// can nevertheless retain CRLF bytes while remaining clean according to Git.
/// Treat CRLF as its canonical LF representation for manifest verification,
/// but only for the four declared text artifact kinds. No other byte is
/// normalized, so content changes and lone carriage returns still fail the
/// byte count or BLAKE3 check.
fn canonical_generated_text_bytes<'a>(relative_path: &Path, bytes: &'a [u8]) -> Cow<'a, [u8]> {
    if !generated_manifest_path_is_text(relative_path)
        || !bytes.windows(2).any(|pair| pair == b"\r\n")
    {
        return Cow::Borrowed(bytes);
    }

    let mut canonical = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index..].starts_with(b"\r\n") {
            canonical.push(b'\n');
            index += 2;
        } else {
            canonical.push(bytes[index]);
            index += 1;
        }
    }
    Cow::Owned(canonical)
}

fn generated_manifest_path_is_text(relative_path: &Path) -> bool {
    matches!(
        relative_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("rs" | "toml")
    ) || matches!(
        relative_path.file_name().and_then(|name| name.to_str()),
        Some(".rspice-veriloga-model-package" | ".rspice-veriloga-generated")
    )
}
