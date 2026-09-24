//! Durable model-validation evidence and its canonical integrity digest.

use rspice_app_types::product::{ContentDigest, ObjectRevision};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

pub const MODEL_VALIDATION_RECEIPT_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelValidationFindingSeverity {
    Information,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelValidationFinding {
    pub code: String,
    pub severity: ModelValidationFindingSeverity,
    pub message: String,
}

/// Durable evidence that one exact project revision passed the executable
/// model pipeline on one supported platform and engine/schema build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelValidationReceipt {
    pub schema_version: u16,
    pub project_revision: ObjectRevision,
    pub model_execution_plan_digest: ContentDigest,
    pub execution_catalog_digest: ContentDigest,
    /// Number of authenticated ordinary-library source members represented by
    /// `source_closure_digest`. The receipt stores one canonical digest rather
    /// than duplicating every source path and digest into project metadata.
    pub source_count: u64,
    pub source_closure_digest: ContentDigest,
    pub pdk_archive_digest: Option<ContentDigest>,
    pub engine_version: String,
    pub execution_schema_version: u32,
    pub platform: String,
    pub findings: Vec<ModelValidationFinding>,
    pub validated_at_unix_ms: u64,
    pub receipt_digest: ContentDigest,
}

/// Evidence and host identity supplied after successful executable validation.
#[derive(Debug)]
pub struct ModelValidationReceiptInput {
    pub project_revision: ObjectRevision,
    pub model_execution_plan_digest: ContentDigest,
    pub execution_catalog_digest: ContentDigest,
    pub source_count: u64,
    pub source_closure_digest: ContentDigest,
    pub pdk_archive_digest: Option<ContentDigest>,
    pub engine_version: String,
    pub execution_schema_version: u32,
    pub platform: String,
    pub findings: Vec<ModelValidationFinding>,
    pub validated_at_unix_ms: u64,
}

impl ModelValidationReceipt {
    /// Seal evidence from a completed validation run using the host's identity and timestamp.
    pub fn issue(input: ModelValidationReceiptInput) -> Result<Self, String> {
        let mut receipt = Self {
            schema_version: MODEL_VALIDATION_RECEIPT_SCHEMA_VERSION,
            project_revision: input.project_revision,
            model_execution_plan_digest: input.model_execution_plan_digest,
            execution_catalog_digest: input.execution_catalog_digest,
            source_count: input.source_count,
            source_closure_digest: input.source_closure_digest,
            pdk_archive_digest: input.pdk_archive_digest,
            engine_version: input.engine_version,
            execution_schema_version: input.execution_schema_version,
            platform: input.platform,
            findings: input.findings,
            validated_at_unix_ms: input.validated_at_unix_ms,
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.receipt_digest = receipt.computed_digest()?;
        receipt.verify()?;
        Ok(receipt)
    }

    pub fn verify(&self) -> Result<(), String> {
        if self.schema_version != MODEL_VALIDATION_RECEIPT_SCHEMA_VERSION {
            return Err(format!(
                "model-validation receipt uses unsupported schema {}",
                self.schema_version
            ));
        }
        if self.engine_version.trim().is_empty()
            || self.engine_version != self.engine_version.trim()
            || self.engine_version.len() > 128
            || !matches!(
                self.platform.as_str(),
                "desktop-windows" | "desktop-macos" | "desktop-linux" | "browser-wasm32"
            )
            || self.validated_at_unix_ms == 0
        {
            return Err(
                "model-validation receipt has an invalid engine, platform, or timestamp identity"
                    .to_owned(),
            );
        }
        if self.findings.is_empty() || self.findings.len() > 64 {
            return Err(
                "model-validation receipt must retain between 1 and 64 bounded findings".to_owned(),
            );
        }
        for finding in &self.findings {
            for (field, value, maximum) in [
                ("finding code", finding.code.as_str(), 128_usize),
                ("finding message", finding.message.as_str(), 2_048_usize),
            ] {
                if value.is_empty()
                    || value != value.trim()
                    || value.len() > maximum
                    || value.chars().any(char::is_control)
                {
                    return Err(format!(
                        "model-validation {field} must be nonempty, trimmed, control-free, and at most {maximum} bytes"
                    ));
                }
            }
        }
        let expected = self.computed_digest()?;
        if expected != self.receipt_digest {
            return Err("model-validation receipt digest does not match its payload".to_owned());
        }
        Ok(())
    }

    fn computed_digest(&self) -> Result<ContentDigest, String> {
        let bytes = serde_json::to_vec(&(
            MODEL_VALIDATION_RECEIPT_SCHEMA_VERSION,
            self.project_revision,
            self.model_execution_plan_digest,
            self.execution_catalog_digest,
            self.source_count,
            self.source_closure_digest,
            self.pdk_archive_digest,
            &self.engine_version,
            self.execution_schema_version,
            &self.platform,
            &self.findings,
            self.validated_at_unix_ms,
        ))
        .map_err(|error| format!("Cannot serialize model-validation receipt payload: {error}"))?;
        Ok(ContentDigest::from_bytes(Sha256::digest(bytes).into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_one_receipt_retains_its_pre_extraction_digest() {
        let receipt = ModelValidationReceipt::issue(ModelValidationReceiptInput {
            project_revision: ObjectRevision::new(7).expect("revision seven"),
            model_execution_plan_digest: ContentDigest::from_bytes([0x11; 32]),
            execution_catalog_digest: ContentDigest::from_bytes([0x22; 32]),
            source_count: 3,
            source_closure_digest: ContentDigest::from_bytes([0x33; 32]),
            pdk_archive_digest: Some(ContentDigest::from_bytes([0x44; 32])),
            engine_version: "0.1.0".to_owned(),
            execution_schema_version: 16,
            platform: "browser-wasm32".to_owned(),
            findings: vec![ModelValidationFinding {
                code: "SPICE_NAMESPACE_COMPILED".to_owned(),
                severity: ModelValidationFindingSeverity::Information,
                message: "The frozen test namespace compiled.".to_owned(),
            }],
            validated_at_unix_ms: 1_700_000_000_123,
        })
        .expect("valid retained evidence");
        // Frozen from the original app schema-one digest tuple and JSON encoding.
        assert_eq!(
            receipt.receipt_digest.to_string(),
            "95f12fd61176213e15bc8ca6cb27f412adeeb52c9885d97df56336710d32bafe"
        );
        let encoded = serde_json::to_vec(&receipt).expect("encode receipt");
        let mut restored: ModelValidationReceipt =
            serde_json::from_slice(&encoded).expect("restore receipt");
        assert_eq!(restored, receipt);
        restored.verify().expect("retained receipt authenticates");
        restored.pdk_archive_digest = None;
        assert!(
            restored
                .verify()
                .expect_err("PDK identity is bound")
                .contains("digest")
        );
    }
}
