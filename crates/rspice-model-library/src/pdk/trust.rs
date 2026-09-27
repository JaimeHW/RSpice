//! Immutable publisher keys, signature verification and hash-chained trust changes.

use super::{
    ContentDigest, MAX_PDK_PUBLISHER_KEYS, MAX_PDK_TRUST_AUDIT_RECEIPTS, PdkTechnologyError,
    content_digest, validate_identifier, validate_text,
};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedPdkPublisherKey {
    pub publisher_id: String,
    pub key_id: String,
    pub verifying_key: [u8; 32],
    #[serde(default)]
    pub revoked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkTrustAuditAction {
    Provision,
    Revoke,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTrustAuditReceipt {
    pub sequence: u64,
    pub action: PdkTrustAuditAction,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub publisher_id: String,
    pub key_id: String,
    pub key_fingerprint: ContentDigest,
    pub before_revoked: Option<bool>,
    pub after_revoked: bool,
    pub previous_receipt_digest: Option<ContentDigest>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct PdkTrustAuditPayload<'a> {
    sequence: u64,
    action: PdkTrustAuditAction,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    publisher_id: &'a str,
    key_id: &'a str,
    key_fingerprint: ContentDigest,
    before_revoked: Option<bool>,
    after_revoked: bool,
    previous_receipt_digest: Option<ContentDigest>,
}

impl PdkTrustAuditReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, PdkTechnologyError> {
        let payload = PdkTrustAuditPayload {
            sequence: self.sequence,
            action: self.action,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            publisher_id: &self.publisher_id,
            key_id: &self.key_id,
            key_fingerprint: self.key_fingerprint,
            before_revoked: self.before_revoked,
            after_revoked: self.after_revoked,
            previous_receipt_digest: self.previous_receipt_digest,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkPublisherTrustStore {
    pub keys: Vec<TrustedPdkPublisherKey>,
    #[serde(default)]
    audit: Vec<PdkTrustAuditReceipt>,
}

impl PdkPublisherTrustStore {
    pub fn validate(&self) -> Result<(), PdkTechnologyError> {
        if self.keys.len() > MAX_PDK_PUBLISHER_KEYS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "publisher trust store exceeds {MAX_PDK_PUBLISHER_KEYS} keys"
            )));
        }
        let mut identities = BTreeSet::new();
        for (index, key) in self.keys.iter().enumerate() {
            validate_identifier(
                &format!("trust_store.keys[{index}].publisher_id"),
                &key.publisher_id,
            )?;
            validate_identifier(&format!("trust_store.keys[{index}].key_id"), &key.key_id)?;
            VerifyingKey::from_bytes(&key.verifying_key).map_err(|error| {
                PdkTechnologyError::InvalidTrustStore(format!(
                    "trust_store.keys[{index}].verifying_key is invalid: {error}"
                ))
            })?;
            let identity = (
                key.publisher_id.to_ascii_lowercase(),
                key.key_id.to_ascii_lowercase(),
            );
            if !identities.insert(identity) {
                return Err(PdkTechnologyError::InvalidTrustStore(format!(
                    "trust_store.keys[{index}] repeats a case-insensitive publisher/key identity"
                )));
            }
        }
        self.validate_audit_chain()
    }

    #[must_use]
    pub fn audit(&self) -> &[PdkTrustAuditReceipt] {
        &self.audit
    }

    pub fn provision_key(
        &mut self,
        key: TrustedPdkPublisherKey,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTrustAuditReceipt, PdkTechnologyError> {
        self.validate()?;
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        if key.revoked {
            return Err(PdkTechnologyError::InvalidTrustStore(
                "a newly provisioned key cannot begin revoked".to_owned(),
            ));
        }
        let mut candidate = self.clone();
        if candidate.keys.iter().any(|existing| {
            existing
                .publisher_id
                .eq_ignore_ascii_case(&key.publisher_id)
                && existing.key_id.eq_ignore_ascii_case(&key.key_id)
        }) {
            return Err(PdkTechnologyError::ImmutableTrustKey(format!(
                "{}/{} is already provisioned",
                key.publisher_id, key.key_id
            )));
        }
        candidate.keys.push(key.clone());
        candidate.keys.sort_by(|left, right| {
            (
                left.publisher_id.to_ascii_lowercase(),
                left.key_id.to_ascii_lowercase(),
            )
                .cmp(&(
                    right.publisher_id.to_ascii_lowercase(),
                    right.key_id.to_ascii_lowercase(),
                ))
        });
        let receipt = candidate.append_trust_receipt(
            PdkTrustAuditAction::Provision,
            &key,
            None,
            false,
            authority,
            reason,
        )?;
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    pub fn revoke_key(
        &mut self,
        publisher_id: &str,
        key_id: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTrustAuditReceipt, PdkTechnologyError> {
        self.validate()?;
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        let mut candidate = self.clone();
        let key = candidate
            .keys
            .iter_mut()
            .find(|key| {
                key.publisher_id.eq_ignore_ascii_case(publisher_id)
                    && key.key_id.eq_ignore_ascii_case(key_id)
            })
            .ok_or_else(|| PdkTechnologyError::UntrustedPublisher {
                publisher_id: publisher_id.to_owned(),
                key_id: key_id.to_owned(),
            })?;
        if key.revoked {
            return Err(PdkTechnologyError::ImmutableTrustKey(format!(
                "{}/{} is already revoked",
                key.publisher_id, key.key_id
            )));
        }
        key.revoked = true;
        let key = key.clone();
        let receipt = candidate.append_trust_receipt(
            PdkTrustAuditAction::Revoke,
            &key,
            Some(false),
            true,
            authority,
            reason,
        )?;
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    fn append_trust_receipt(
        &mut self,
        action: PdkTrustAuditAction,
        key: &TrustedPdkPublisherKey,
        before_revoked: Option<bool>,
        after_revoked: bool,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTrustAuditReceipt, PdkTechnologyError> {
        if self.audit.len() >= MAX_PDK_TRUST_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "publisher trust audit receipts exceed {MAX_PDK_TRUST_AUDIT_RECEIPTS}"
            )));
        }
        let mut receipt = PdkTrustAuditReceipt {
            sequence: u64::try_from(self.audit.len())
                .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?
                .checked_add(1)
                .ok_or_else(|| {
                    PdkTechnologyError::LimitExceeded(
                        "publisher trust audit sequence is exhausted".to_owned(),
                    )
                })?,
            action,
            actor_id: authority.actor_id.clone(),
            authority_id: authority.authority_id.clone(),
            reason: reason.to_owned(),
            publisher_id: key.publisher_id.clone(),
            key_id: key.key_id.clone(),
            key_fingerprint: content_digest(&key.verifying_key),
            before_revoked,
            after_revoked,
            previous_receipt_digest: self.audit.last().map(|receipt| receipt.receipt_digest),
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;
        self.audit.push(receipt.clone());
        Ok(receipt)
    }

    fn validate_audit_chain(&self) -> Result<(), PdkTechnologyError> {
        if self.audit.len() > MAX_PDK_TRUST_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                "receipt count exceeds {MAX_PDK_TRUST_AUDIT_RECEIPTS}"
            )));
        }
        let mut previous = None;
        let mut audited = BTreeMap::<(String, String), (ContentDigest, bool)>::new();
        for (index, receipt) in self.audit.iter().enumerate() {
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| {
                    PdkTechnologyError::TrustAuditCorrupted(
                        "receipt sequence is exhausted".to_owned(),
                    )
                })?;
            if receipt.sequence != expected_sequence
                || receipt.previous_receipt_digest != previous
                || receipt.calculate_digest()? != receipt.receipt_digest
            {
                return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                    "receipt #{} has invalid sequence or digest linkage",
                    receipt.sequence
                )));
            }
            validate_identifier("trust_audit.publisher_id", &receipt.publisher_id)?;
            validate_identifier("trust_audit.key_id", &receipt.key_id)?;
            validate_text("trust_audit.reason", &receipt.reason, 1_024)?;
            PdkAdministrativeAuthority {
                actor_id: receipt.actor_id.clone(),
                authority_id: receipt.authority_id.clone(),
            }
            .validate()?;
            let identity = (
                receipt.publisher_id.to_ascii_lowercase(),
                receipt.key_id.to_ascii_lowercase(),
            );
            match receipt.action {
                PdkTrustAuditAction::Provision => {
                    if receipt.before_revoked.is_some()
                        || receipt.after_revoked
                        || audited
                            .insert(identity, (receipt.key_fingerprint, false))
                            .is_some()
                    {
                        return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                            "receipt #{} is not a valid immutable provision transition",
                            receipt.sequence
                        )));
                    }
                }
                PdkTrustAuditAction::Revoke => {
                    if receipt.before_revoked != Some(false) || !receipt.after_revoked {
                        return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                            "receipt #{} is not a valid revocation transition",
                            receipt.sequence
                        )));
                    }
                    if let Some((fingerprint, revoked)) = audited.get_mut(&identity) {
                        if *fingerprint != receipt.key_fingerprint || *revoked {
                            return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                                "receipt #{} revokes a different or already-revoked key",
                                receipt.sequence
                            )));
                        }
                        *revoked = true;
                    } else {
                        // A first receipt may revoke a key provisioned by a
                        // legacy configuration predating trust audit support.
                        audited.insert(identity, (receipt.key_fingerprint, true));
                    }
                }
            }
            previous = Some(receipt.receipt_digest);
        }
        for ((publisher_id, key_id), (fingerprint, revoked)) in audited {
            let key = self
                .keys
                .iter()
                .find(|key| {
                    key.publisher_id.eq_ignore_ascii_case(&publisher_id)
                        && key.key_id.eq_ignore_ascii_case(&key_id)
                })
                .ok_or_else(|| {
                    PdkTechnologyError::TrustAuditCorrupted(format!(
                        "audited key {publisher_id}/{key_id} is absent"
                    ))
                })?;
            if content_digest(&key.verifying_key) != fingerprint || key.revoked != revoked {
                return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                    "audited key {publisher_id}/{key_id} does not match its final receipt state"
                )));
            }
        }
        Ok(())
    }

    fn resolve(
        &self,
        publisher_id: &str,
        key_id: &str,
    ) -> Result<&TrustedPdkPublisherKey, PdkTechnologyError> {
        self.validate()?;
        let key = self
            .keys
            .iter()
            .find(|candidate| {
                candidate.publisher_id.eq_ignore_ascii_case(publisher_id)
                    && candidate.key_id.eq_ignore_ascii_case(key_id)
            })
            .ok_or_else(|| PdkTechnologyError::UntrustedPublisher {
                publisher_id: publisher_id.to_owned(),
                key_id: key_id.to_owned(),
            })?;
        if key.revoked {
            return Err(PdkTechnologyError::RevokedPublisherKey {
                publisher_id: publisher_id.to_owned(),
                key_id: key_id.to_owned(),
            });
        }
        Ok(key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkAdministrativeAuthority {
    pub actor_id: String,
    pub authority_id: String,
}

impl PdkAdministrativeAuthority {
    pub fn validate(&self) -> Result<(), PdkTechnologyError> {
        validate_identifier("authority.actor_id", &self.actor_id)?;
        validate_identifier("authority.authority_id", &self.authority_id)
    }
}

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
/// trust store owns which keys are trusted and which were revoked.
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

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer as _, SigningKey};

    #[test]
    fn publisher_authority_survives_round_trip_and_refuses_revocation_or_tampering() {
        let signing = SigningKey::from_bytes(&[0x42; 32]);
        let key = TrustedPdkPublisherKey {
            publisher_id: "foundry".to_owned(),
            key_id: "release".to_owned(),
            verifying_key: signing.verifying_key().to_bytes(),
            revoked: false,
        };
        let authority = PdkAdministrativeAuthority {
            actor_id: "admin".to_owned(),
            authority_id: "pdk-admin".to_owned(),
        };
        let mut trust = PdkPublisherTrustStore::default();
        trust
            .provision_key(key.clone(), &authority, "Approved publisher")
            .unwrap();
        let bytes = b"exact signed payload";
        let signature = signing.sign(bytes).to_bytes();
        trust
            .verify_publisher_signature("FOUNDRY", "RELEASE", bytes, &signature)
            .unwrap();
        assert!(matches!(
            trust.verify_publisher_signature("foundry", "release", b"tampered", &signature),
            Err(PdkTechnologyError::InvalidSignature { .. })
        ));
        assert!(matches!(
            trust.verify_publisher_signature("foundry", "release", bytes, &signature[..63]),
            Err(PdkTechnologyError::InvalidSignatureLength { actual: 63 })
        ));
        let before = trust.clone();
        assert!(matches!(
            trust.provision_key(key, &authority, "Repeated"),
            Err(PdkTechnologyError::ImmutableTrustKey(_))
        ));
        assert_eq!(trust, before);
        let receipt = trust
            .revoke_key("foundry", "release", &authority, "Key retired")
            .unwrap();
        assert_eq!(
            receipt.previous_receipt_digest,
            Some(before.audit()[0].receipt_digest)
        );
        let restored: PdkPublisherTrustStore =
            serde_json::from_slice(&serde_json::to_vec(&trust).unwrap()).unwrap();
        assert_eq!(restored, trust);
        restored.validate().unwrap();
        assert!(matches!(
            restored.verify_publisher_signature("foundry", "release", bytes, &signature),
            Err(PdkTechnologyError::RevokedPublisherKey { .. })
        ));
        let mut tampered = restored;
        tampered.audit[0].reason = "altered".to_owned();
        assert!(matches!(
            tampered.validate(),
            Err(PdkTechnologyError::TrustAuditCorrupted(_))
        ));
    }
}
