//! Signature verification shared by governed technology and drawing-sheet packages.
//!
//! The trust store supplies publisher identity, revocation, and key-validity
//! authority while callers remain responsible for domain-separating payloads.

use super::*;

impl PdkPublisherTrustStore {
    /// Verify an Ed25519 signature against the currently trusted publisher key.
    pub fn verify_publisher_signature(
        &self,
        publisher_id: &str,
        key_id: &str,
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), PdkTechnologyError> {
        let key = self.resolve(publisher_id, key_id)?;
        let verifying_key = VerifyingKey::from_bytes(&key.verifying_key).map_err(|error| {
            PdkTechnologyError::InvalidTrustStore(format!(
                "trusted key {}/{} is invalid: {error}",
                key.publisher_id, key.key_id
            ))
        })?;
        let signature_bytes: [u8; 64] =
            signature
                .try_into()
                .map_err(|_| PdkTechnologyError::InvalidSignatureLength {
                    actual: signature.len(),
                })?;
        verifying_key
            .verify_strict(message, &Signature::from_bytes(&signature_bytes))
            .map_err(|_| PdkTechnologyError::InvalidSignature {
                publisher_id: publisher_id.to_owned(),
                key_id: key_id.to_owned(),
            })
    }
}

/// Let the drawing-sheet package contract consult this trust store without
/// depending on it. The contract crate owns the authenticity rule; the
/// application owns which keys are trusted and which were revoked.
impl rspice_design_model::sheet_package::PublisherTrust for PdkPublisherTrustStore {
    fn verify_publisher_signature(
        &self,
        publisher_id: &str,
        key_id: &str,
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), String> {
        Self::verify_publisher_signature(self, publisher_id, key_id, message, signature)
            .map_err(|error| error.to_string())
    }
}
