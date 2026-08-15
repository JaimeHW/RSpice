use ed25519_dalek::{Signature, Signer};
use sha2::{Digest, Sha256};

pub use ed25519_dalek::{SigningKey, VerifyingKey};

use crate::error::PackError;

/// Raw bytes of an ed25519 secret scalar seed.
pub const SIGNING_KEY_BYTES: usize = 32;
/// Raw bytes of an ed25519 public key.
pub const VERIFYING_KEY_BYTES: usize = 32;
/// Raw bytes of an ed25519 signature.
pub const SIGNATURE_BYTES: usize = 64;

/// Builds a signing key from caller-supplied secret bytes.
///
/// Key material never originates inside the portable crate: a client that only
/// verifies must not link a random-number source, and a signer's key comes
/// from its own custody path.
#[must_use]
pub fn signing_key(secret: &[u8; SIGNING_KEY_BYTES]) -> SigningKey {
    SigningKey::from_bytes(secret)
}

/// Builds a verifying key, refusing points that are not canonical encodings.
pub fn verifying_key(public: &[u8; VERIFYING_KEY_BYTES]) -> Result<VerifyingKey, PackError> {
    VerifyingKey::from_bytes(public).map_err(|_| PackError::InvalidVerifyingKey)
}

/// Builds a verifying key from its lowercase hexadecimal form.
pub fn verifying_key_from_hex(text: &str) -> Result<VerifyingKey, PackError> {
    let bytes = decode_hex(text).ok_or(PackError::InvalidVerifyingKey)?;
    verifying_key(&bytes)
}

/// Signs the exact bytes that will be stored as `manifest.json`.
#[must_use]
pub fn sign_manifest(manifest_bytes: &[u8], key: &SigningKey) -> [u8; SIGNATURE_BYTES] {
    key.sign(manifest_bytes).to_bytes()
}

/// Verifies a detached signature over the exact stored manifest bytes.
///
/// Strict verification rejects small-order and non-canonical public keys, so a
/// signature that would verify under more than one key cannot be produced.
pub fn verify_manifest_signature(
    manifest_bytes: &[u8],
    signature: &[u8; SIGNATURE_BYTES],
    key: &VerifyingKey,
) -> Result<(), PackError> {
    key.verify_strict(manifest_bytes, &Signature::from_bytes(signature))
        .map_err(|_| PackError::BadSignature)
}

/// Lowercase hexadecimal of a SHA-256 digest over `bytes`.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    encode_hex(&Sha256::digest(bytes))
}

#[must_use]
pub fn encode_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(char::from(DIGITS[usize::from(byte >> 4)]));
        text.push(char::from(DIGITS[usize::from(byte & 0x0F)]));
    }
    text
}

fn decode_hex(text: &str) -> Option<[u8; VERIFYING_KEY_BYTES]> {
    // `from_str_radix` would also accept uppercase digits and a leading sign;
    // the canonical form on the wire is lowercase hexadecimal only.
    let canonical = text.len() == VERIFYING_KEY_BYTES * 2
        && text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !canonical {
        return None;
    }
    let mut bytes = [0_u8; VERIFYING_KEY_BYTES];
    for (index, slot) in bytes.iter_mut().enumerate() {
        let pair = text.get(index * 2..index * 2 + 2)?;
        *slot = u8::from_str_radix(pair, 16).ok()?;
    }
    Some(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signatures_verify_only_under_their_own_key_and_message() {
        let key = signing_key(&[9_u8; SIGNING_KEY_BYTES]);
        let other = signing_key(&[11_u8; SIGNING_KEY_BYTES]);
        let message = b"manifest bytes".as_slice();
        let signature = sign_manifest(message, &key);
        verify_manifest_signature(message, &signature, &key.verifying_key()).expect("verifies");
        assert!(matches!(
            verify_manifest_signature(message, &signature, &other.verifying_key()),
            Err(PackError::BadSignature)
        ));
        assert!(matches!(
            verify_manifest_signature(b"other bytes", &signature, &key.verifying_key()),
            Err(PackError::BadSignature)
        ));
        let mut flipped = signature;
        flipped[0] ^= 0x01;
        assert!(matches!(
            verify_manifest_signature(message, &flipped, &key.verifying_key()),
            Err(PackError::BadSignature)
        ));
    }

    #[test]
    fn verifying_keys_round_trip_through_lowercase_hexadecimal() {
        let key = signing_key(&[3_u8; SIGNING_KEY_BYTES]).verifying_key();
        let text = encode_hex(key.as_bytes());
        assert_eq!(
            verifying_key_from_hex(&text).expect("parses").to_bytes(),
            key.to_bytes()
        );
        assert!(verifying_key_from_hex(&text.to_uppercase()).is_err());
        assert!(verifying_key_from_hex("00").is_err());
    }

    #[test]
    fn digests_match_the_published_empty_vector() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
