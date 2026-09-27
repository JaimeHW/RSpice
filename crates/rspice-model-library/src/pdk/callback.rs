//! Retained callback inputs, resource policy and verifiable execution/project receipts.
//!
//! Receipt validation preserves data identity; it does not grant callback execution.

use super::contracts::{PDK_CALLBACK_ABI_VERSION, PdkExecutionTarget, PdkTechnologyBinding};
use super::{PdkTechnologyError, content_digest};
use rspice_app_types::product::{ContentDigest, ObjectRevision, ProjectId, SimulationPlanId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const PDK_CALLBACK_EXECUTION_RECEIPT_SCHEMA_VERSION: u32 = 1;
pub const PROJECT_PDK_CALLBACK_RECEIPT_SCHEMA_VERSION: u32 = 1;
pub const MAX_PROJECT_PDK_CALLBACK_RECEIPTS: usize = 4_096;
pub const PDK_CALLBACK_FUEL_LIMIT: u64 = 10_000_000;
pub const PDK_CALLBACK_MEMORY_LIMIT_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_PDK_CALLBACK_PROJECT_PARAMETERS: usize = 1_024;
pub const MAX_PDK_CALLBACK_PARAMETER_KEY_BYTES: usize = 256;
pub const MAX_PDK_CALLBACK_PARAMETER_VALUE_BYTES: usize = 16 * 1024;
pub const MAX_PDK_CALLBACK_METADATA_ENTRIES: usize = 256;
pub const MAX_PDK_CALLBACK_METADATA_KEY_BYTES: usize = 256;
pub const MAX_PDK_CALLBACK_METADATA_VALUE_BYTES: usize = 64 * 1024;
pub const MAX_PDK_CALLBACK_METADATA_TOTAL_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkCallbackExecutionInput {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub project_parameters: BTreeMap<String, String>,
}

impl PdkCallbackExecutionInput {
    pub fn validate(&self) -> Result<(), PdkCallbackError> {
        if self.project_parameters.len() > MAX_PDK_CALLBACK_PROJECT_PARAMETERS {
            return Err(PdkCallbackError::InvalidInput(format!(
                "project parameter count exceeds {MAX_PDK_CALLBACK_PROJECT_PARAMETERS}"
            )));
        }
        for (key, value) in &self.project_parameters {
            validate_host_key(
                "project parameter key",
                key,
                MAX_PDK_CALLBACK_PARAMETER_KEY_BYTES,
            )?;
            validate_host_text(
                "project parameter value",
                value,
                MAX_PDK_CALLBACK_PARAMETER_VALUE_BYTES,
            )?;
        }
        Ok(())
    }

    pub fn content_digest(&self) -> Result<ContentDigest, PdkCallbackError> {
        self.validate()?;
        let bytes = serde_json::to_vec(self)
            .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkCallbackExecutionReceipt {
    pub schema_version: u32,
    pub package_binding: PdkTechnologyBinding,
    pub archive_digest: ContentDigest,
    pub callback_id: String,
    pub callback_artifact_path: String,
    pub callback_artifact_digest: ContentDigest,
    pub abi_version: u32,
    pub execution_target: PdkExecutionTarget,
    pub input_digest: ContentDigest,
    pub output_digest: ContentDigest,
    pub fuel_limit: u64,
    pub fuel_consumed: u64,
    pub derived_metadata: BTreeMap<String, String>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct CallbackReceiptPayload<'a> {
    schema_version: u32,
    package_binding: &'a PdkTechnologyBinding,
    archive_digest: ContentDigest,
    callback_id: &'a str,
    callback_artifact_path: &'a str,
    callback_artifact_digest: ContentDigest,
    abi_version: u32,
    execution_target: PdkExecutionTarget,
    input_digest: ContentDigest,
    output_digest: ContentDigest,
    fuel_limit: u64,
    fuel_consumed: u64,
    derived_metadata: &'a BTreeMap<String, String>,
}

impl PdkCallbackExecutionReceipt {
    /// Calculate and validate the digest of retained receipt data.
    /// Signed-package validation and execution remain the host's responsibility.
    pub fn with_validated_digest(mut self) -> Result<Self, PdkCallbackError> {
        self.receipt_digest = self.calculate_digest()?;
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), PdkCallbackError> {
        if self.schema_version != PDK_CALLBACK_EXECUTION_RECEIPT_SCHEMA_VERSION {
            return Err(PdkCallbackError::InvalidReceipt(format!(
                "unsupported receipt schema {}",
                self.schema_version
            )));
        }
        validate_host_key("callback id", &self.callback_id, 256)?;
        validate_host_key(
            "callback artifact path",
            &self.callback_artifact_path,
            1_024,
        )?;
        if self.abi_version != PDK_CALLBACK_ABI_VERSION {
            return Err(PdkCallbackError::InvalidReceipt(format!(
                "callback ABI {} is not supported",
                self.abi_version
            )));
        }
        if self.fuel_limit != PDK_CALLBACK_FUEL_LIMIT || self.fuel_consumed > self.fuel_limit {
            return Err(PdkCallbackError::InvalidReceipt(
                "fuel identity is outside the callback execution contract".to_owned(),
            ));
        }
        validate_metadata(&self.derived_metadata)?;
        if digest_metadata(&self.derived_metadata)? != self.output_digest {
            return Err(PdkCallbackError::InvalidReceipt(
                "derived metadata digest does not match its payload".to_owned(),
            ));
        }
        if self.calculate_digest()? != self.receipt_digest {
            return Err(PdkCallbackError::InvalidReceipt(
                "receipt digest does not match its payload".to_owned(),
            ));
        }
        Ok(())
    }

    fn calculate_digest(&self) -> Result<ContentDigest, PdkCallbackError> {
        let payload = CallbackReceiptPayload {
            schema_version: self.schema_version,
            package_binding: &self.package_binding,
            archive_digest: self.archive_digest,
            callback_id: &self.callback_id,
            callback_artifact_path: &self.callback_artifact_path,
            callback_artifact_digest: self.callback_artifact_digest,
            abi_version: self.abi_version,
            execution_target: self.execution_target,
            input_digest: self.input_digest,
            output_digest: self.output_digest,
            fuel_limit: self.fuel_limit,
            fuel_consumed: self.fuel_consumed,
            derived_metadata: &self.derived_metadata,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

/// Project-owned evidence for one exact signed callback invocation.
///
/// The embedded execution receipt proves the sandbox/package boundary. This
/// outer receipt additionally binds the canonical input payload, active plan,
/// project revision transaction, operator identity, and append-only project
/// ledger position so derived metadata cannot be mistaken for ambient or
/// administrator-active state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPdkCallbackReceipt {
    pub schema_version: u32,
    pub sequence: u64,
    pub project_id: ProjectId,
    pub from_project_revision: ObjectRevision,
    pub to_project_revision: ObjectRevision,
    pub plan_id: SimulationPlanId,
    pub plan_revision: ObjectRevision,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub input: PdkCallbackExecutionInput,
    pub execution: PdkCallbackExecutionReceipt,
    pub previous_receipt_digest: Option<ContentDigest>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct ProjectPdkCallbackReceiptPayload<'a> {
    schema_version: u32,
    sequence: u64,
    project_id: ProjectId,
    from_project_revision: ObjectRevision,
    to_project_revision: ObjectRevision,
    plan_id: SimulationPlanId,
    plan_revision: ObjectRevision,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    input: &'a PdkCallbackExecutionInput,
    execution: &'a PdkCallbackExecutionReceipt,
    previous_receipt_digest: Option<ContentDigest>,
}

impl ProjectPdkCallbackReceipt {
    /// Calculate and validate retained project-receipt data before publication.
    pub fn with_validated_digest(mut self) -> Result<Self, PdkCallbackError> {
        self.receipt_digest = self.calculate_digest()?;
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), PdkCallbackError> {
        if self.schema_version != PROJECT_PDK_CALLBACK_RECEIPT_SCHEMA_VERSION {
            return Err(PdkCallbackError::InvalidReceipt(format!(
                "unsupported project callback receipt schema {}",
                self.schema_version
            )));
        }
        if self.sequence == 0 {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback receipt sequence is zero".to_owned(),
            ));
        }
        if self.project_id.as_uuid().is_nil() {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback receipt has a nil project identity".to_owned(),
            ));
        }
        if self.from_project_revision.next().ok() != Some(self.to_project_revision) {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback receipt does not advance exactly one project revision".to_owned(),
            ));
        }
        validate_receipt_text("actor ID", &self.actor_id, 256)?;
        validate_receipt_text("authority ID", &self.authority_id, 256)?;
        validate_receipt_text("reason", &self.reason, 2_048)?;
        self.input.validate()?;
        self.execution.validate()?;
        if self.input.content_digest()? != self.execution.input_digest {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback input payload does not match the sandbox input digest".to_owned(),
            ));
        }
        if self.calculate_digest()? != self.receipt_digest {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback receipt digest does not match its payload".to_owned(),
            ));
        }
        Ok(())
    }

    fn calculate_digest(&self) -> Result<ContentDigest, PdkCallbackError> {
        let payload = ProjectPdkCallbackReceiptPayload {
            schema_version: self.schema_version,
            sequence: self.sequence,
            project_id: self.project_id,
            from_project_revision: self.from_project_revision,
            to_project_revision: self.to_project_revision,
            plan_id: self.plan_id,
            plan_revision: self.plan_revision,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            input: &self.input,
            execution: &self.execution,
            previous_receipt_digest: self.previous_receipt_digest,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PdkCallbackError {
    #[error(transparent)]
    Technology(#[from] PdkTechnologyError),
    #[error("project PDK callback transaction failed: {0}")]
    ProjectTransaction(String),
    #[error("invalid PDK callback input: {0}")]
    InvalidInput(String),
    #[error("signed PDK callback '{0}' is not declared")]
    CallbackNotFound(String),
    #[error("signed PDK callback module is invalid: {0}")]
    InvalidModule(String),
    #[error("signed PDK callback capability violation: {0}")]
    CapabilityViolation(String),
    #[error("signed PDK callback could not be instantiated: {0}")]
    Instantiation(String),
    #[error("signed PDK callback execution failed: {0}")]
    Execution(String),
    #[error("signed PDK callback returned status {0}")]
    GuestStatus(i32),
    #[error("signed PDK callback host contract was violated: {0}")]
    HostViolation(String),
    #[error("invalid PDK callback receipt: {0}")]
    InvalidReceipt(String),
    #[error("PDK callback serialization failed: {0}")]
    Serialization(String),
}

fn validate_receipt_text(field: &str, value: &str, maximum: usize) -> Result<(), PdkCallbackError> {
    if value.is_empty() || value != value.trim() {
        return Err(PdkCallbackError::InvalidReceipt(format!(
            "project callback {field} must be nonempty and trimmed"
        )));
    }
    if value.len() > maximum || value.chars().any(char::is_control) {
        return Err(PdkCallbackError::InvalidReceipt(format!(
            "project callback {field} exceeds {maximum} bytes or contains control characters"
        )));
    }
    Ok(())
}

pub fn validate_metadata(metadata: &BTreeMap<String, String>) -> Result<(), PdkCallbackError> {
    if metadata.len() > MAX_PDK_CALLBACK_METADATA_ENTRIES {
        return Err(PdkCallbackError::InvalidReceipt(format!(
            "derived metadata contains more than {MAX_PDK_CALLBACK_METADATA_ENTRIES} entries"
        )));
    }
    let mut total = 0usize;
    for (key, value) in metadata {
        validate_host_key(
            "derived metadata key",
            key,
            MAX_PDK_CALLBACK_METADATA_KEY_BYTES,
        )?;
        validate_host_text(
            "derived metadata value",
            value,
            MAX_PDK_CALLBACK_METADATA_VALUE_BYTES,
        )?;
        total = total.saturating_add(key.len()).saturating_add(value.len());
    }
    if total > MAX_PDK_CALLBACK_METADATA_TOTAL_BYTES {
        return Err(PdkCallbackError::InvalidReceipt(format!(
            "derived metadata exceeds {MAX_PDK_CALLBACK_METADATA_TOTAL_BYTES} bytes"
        )));
    }
    Ok(())
}

pub fn validate_host_key(field: &str, value: &str, maximum: usize) -> Result<(), PdkCallbackError> {
    validate_host_text(field, value, maximum)?;
    if !value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(byte, b'_' | b'-' | b'.' | b':' | b'/' | b'[' | b']' | b'@')
    }) {
        return Err(PdkCallbackError::InvalidInput(format!(
            "{field} contains unsupported characters"
        )));
    }
    Ok(())
}

pub fn validate_host_text(
    field: &str,
    value: &str,
    maximum: usize,
) -> Result<(), PdkCallbackError> {
    if value.is_empty() || value.len() > maximum || value.chars().any(char::is_control) {
        return Err(PdkCallbackError::InvalidInput(format!(
            "{field} must contain 1..={maximum} non-control UTF-8 bytes"
        )));
    }
    Ok(())
}

pub fn digest_metadata(
    metadata: &BTreeMap<String, String>,
) -> Result<ContentDigest, PdkCallbackError> {
    validate_metadata(metadata)?;
    let bytes = serde_json::to_vec(metadata)
        .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
    Ok(content_digest(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_finalization_preserves_metadata_and_rejects_invalid_resource_evidence() {
        let input = PdkCallbackExecutionInput::default();
        let metadata = BTreeMap::from([("derived.width".to_owned(), "2.5u".to_owned())]);
        let receipt = PdkCallbackExecutionReceipt {
            schema_version: PDK_CALLBACK_EXECUTION_RECEIPT_SCHEMA_VERSION,
            package_binding: PdkTechnologyBinding {
                package_id: "foundry".to_owned(),
                revision: "1.0.0".to_owned(),
                manifest_digest: content_digest(b"manifest"),
            },
            archive_digest: content_digest(b"archive"),
            callback_id: "derive".to_owned(),
            callback_artifact_path: "callback.wasm".to_owned(),
            callback_artifact_digest: content_digest(b"module"),
            abi_version: PDK_CALLBACK_ABI_VERSION,
            execution_target: PdkExecutionTarget::Desktop,
            input_digest: input.content_digest().unwrap(),
            output_digest: digest_metadata(&metadata).unwrap(),
            fuel_limit: PDK_CALLBACK_FUEL_LIMIT,
            fuel_consumed: 7,
            derived_metadata: metadata.clone(),
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        let finalized = receipt.clone().with_validated_digest().unwrap();
        assert_eq!(finalized.derived_metadata, metadata);
        let restored: PdkCallbackExecutionReceipt =
            serde_json::from_slice(&serde_json::to_vec(&finalized).unwrap()).unwrap();
        assert_eq!(restored, finalized);
        restored.validate().unwrap();
        let mut excessive_fuel = receipt.clone();
        excessive_fuel.fuel_consumed = PDK_CALLBACK_FUEL_LIMIT + 1;
        assert!(matches!(
            excessive_fuel.with_validated_digest(),
            Err(PdkCallbackError::InvalidReceipt(_))
        ));
        let mut altered = receipt;
        altered
            .derived_metadata
            .insert("derived.width".to_owned(), "3u".to_owned());
        assert!(matches!(
            altered.with_validated_digest(),
            Err(PdkCallbackError::InvalidReceipt(_))
        ));
    }
}
