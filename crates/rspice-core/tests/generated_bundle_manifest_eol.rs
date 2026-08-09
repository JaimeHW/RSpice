#[path = "../build_support.rs"]
mod build_support;

use build_support::verify_declared_generated_file;
use std::borrow::Cow;

fn declared(content: &[u8]) -> (u64, String) {
    (
        content.len() as u64,
        blake3::hash(content).to_hex().to_string(),
    )
}

#[test]
fn generated_manifest_accepts_canonical_lf_text() {
    let expected = b"generated\nsource\n";
    let (bytes, digest) = declared(expected);
    let verified =
        verify_declared_generated_file("models/example/src/state.rs", expected, bytes, &digest)
            .expect("canonical LF source verifies");
    assert!(matches!(verified, Cow::Borrowed(_)));
    assert_eq!(verified.as_ref(), expected);
}

#[test]
fn generated_manifest_accepts_crlf_equivalent_text() {
    let expected = b"generated\nsource\n";
    let checkout = b"generated\r\nsource\r\n";
    let (bytes, digest) = declared(expected);
    let verified = verify_declared_generated_file(
        "models/example/.rspice-veriloga-generated",
        checkout,
        bytes,
        &digest,
    )
    .expect("CRLF-equivalent generated text verifies against LF manifest bytes");
    assert!(matches!(verified, Cow::Owned(_)));
    assert_eq!(verified.as_ref(), expected);
}

#[test]
fn generated_manifest_rejects_content_mutation_after_eol_normalization() {
    let expected = b"generated\nsource\n";
    let mutation = b"generated\r\nmutated\r\n";
    let (bytes, digest) = declared(expected);
    let error =
        verify_declared_generated_file("models/example/src/state.rs", mutation, bytes, &digest)
            .expect_err("a same-shape content mutation must not verify");
    assert!(
        error.contains("canonical digest") || error.contains("canonical bytes"),
        "content mutation must fail size or digest verification: {error}"
    );
}
