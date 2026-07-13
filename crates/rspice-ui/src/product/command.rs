use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use super::{ObjectRef, ObjectRevision};

/// Stable field identity, matching the handoff schema
/// `^[a-z0-9]+(?:[.-][a-z0-9]+)*$`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldId(String);

impl FieldId {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        validate_segmented_id(&value, true)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable command identity. Commands use lowercase alphanumeric segments
/// separated by a single hyphen.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandId(String);

impl CommandId {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        validate_segmented_id(&value, false)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable GUI validation/denial/failure code. The value is deliberately not
/// localized; user-facing copy is carried separately.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationCode(String);

impl ValidationCode {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentifierError> {
        let value = value.into();
        if !value.starts_with("GUI-")
            || value.len() <= 4
            || value
                .bytes()
                .skip(4)
                .any(|byte| !byte.is_ascii_uppercase() && !byte.is_ascii_digit() && byte != b'-')
            || value.ends_with('-')
            || value.contains("--")
        {
            return Err(IdentifierError::InvalidValidationCode(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn validate_segmented_id(value: &str, allow_period: bool) -> Result<(), IdentifierError> {
    if value.is_empty() {
        return Err(IdentifierError::Empty);
    }
    let mut previous_separator = true;
    for byte in value.bytes() {
        let separator = byte == b'-' || (allow_period && byte == b'.');
        if !byte.is_ascii_lowercase() && !byte.is_ascii_digit() && !separator {
            return Err(IdentifierError::Character(byte as char));
        }
        if separator && previous_separator {
            return Err(IdentifierError::Separator);
        }
        previous_separator = separator;
    }
    if previous_separator {
        return Err(IdentifierError::Separator);
    }
    Ok(())
}

macro_rules! string_id_serde {
    ($type:ty) => {
        impl fmt::Display for $type {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $type {
            type Err = IdentifierError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

string_id_serde!(FieldId);
string_id_serde!(CommandId);
string_id_serde!(ValidationCode);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IdentifierError {
    #[error("identifier must not be empty")]
    Empty,
    #[error("identifier contains unsupported character {0:?}")]
    Character(char),
    #[error("identifier separators must occur singly between alphanumeric segments")]
    Separator,
    #[error("validation code must match GUI-[A-Z0-9]+(?:-[A-Z0-9]+)*: {0}")]
    InvalidValidationCode(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValidationSeverity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub field_id: FieldId,
    pub code: ValidationCode,
    pub message: String,
    pub severity: ValidationSeverity,
}

impl ValidationIssue {
    pub fn error(field_id: FieldId, code: ValidationCode, message: impl Into<String>) -> Self {
        Self {
            field_id,
            code,
            message: message.into(),
            severity: ValidationSeverity::Error,
        }
    }
}

/// Receipt for one committed transaction. The transaction ID and object
/// revision are durable; a toast or console line is only a rendering of this
/// record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub transaction_id: Uuid,
    pub command_id: CommandId,
    pub target: ObjectRef,
    pub committed_revision: ObjectRevision,
    pub changed_fields: Vec<FieldId>,
    pub invalidated_objects: Vec<ObjectRef>,
    pub timestamp_unix_ms: u64,
}

impl TransactionReceipt {
    #[must_use]
    pub fn new(
        command_id: CommandId,
        target: ObjectRef,
        committed_revision: ObjectRevision,
        timestamp_unix_ms: u64,
    ) -> Self {
        Self {
            transaction_id: Uuid::new_v4(),
            command_id,
            target,
            committed_revision,
            changed_fields: Vec::new(),
            invalidated_objects: Vec::new(),
            timestamp_unix_ms,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthorityDenialKind {
    Permission,
    Entitlement,
    Reauthentication,
    IndependentApproval,
    Offline,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityDenial {
    pub kind: AuthorityDenialKind,
    pub code: ValidationCode,
    pub message: String,
    pub corrective_action: Option<CommandId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictReceipt {
    pub target: ObjectRef,
    pub local_revision: ObjectRevision,
    pub remote_revision: ObjectRevision,
    pub conflicting_fields: Vec<FieldId>,
    pub resolution_command: CommandId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationReceipt {
    pub command_id: CommandId,
    pub target: ObjectRef,
    pub retained_artifacts: Vec<ObjectRef>,
    pub timestamp_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionFailure {
    pub code: ValidationCode,
    pub message: String,
    pub retryable: bool,
    pub diagnostics: Option<ObjectRef>,
    pub recovery_command: Option<CommandId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryReceipt {
    pub command_id: CommandId,
    pub target: ObjectRef,
    pub restored_revision: Option<ObjectRevision>,
    pub recovered_artifacts: Vec<ObjectRef>,
    pub timestamp_unix_ms: u64,
}

/// Exhaustive command result contract. Presentation code must handle every
/// variant and may never translate an unknown outcome into silent success.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "kebab-case")]
pub enum CommandOutcome<T> {
    Committed {
        value: T,
        receipt: TransactionReceipt,
    },
    ValidationFailed {
        issues: Vec<ValidationIssue>,
    },
    Denied {
        denial: AuthorityDenial,
    },
    Conflict {
        conflict: ConflictReceipt,
    },
    Cancelled {
        receipt: CancellationReceipt,
    },
    ExecutionFailed {
        failure: ExecutionFailure,
    },
    Recovered {
        value: T,
        receipt: RecoveryReceipt,
    },
}

impl<T> CommandOutcome<T> {
    #[must_use]
    pub const fn is_committed(&self) -> bool {
        matches!(self, Self::Committed { .. })
    }

    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> CommandOutcome<U> {
        match self {
            Self::Committed { value, receipt } => CommandOutcome::Committed {
                value: map(value),
                receipt,
            },
            Self::ValidationFailed { issues } => CommandOutcome::ValidationFailed { issues },
            Self::Denied { denial } => CommandOutcome::Denied { denial },
            Self::Conflict { conflict } => CommandOutcome::Conflict { conflict },
            Self::Cancelled { receipt } => CommandOutcome::Cancelled { receipt },
            Self::ExecutionFailed { failure } => CommandOutcome::ExecutionFailed { failure },
            Self::Recovered { value, receipt } => CommandOutcome::Recovered {
                value: map(value),
                receipt,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ProductObjectKind, ProjectId};

    #[test]
    fn stable_ids_reject_ambiguous_or_unportable_forms() {
        for invalid in [
            "",
            "Project.Name",
            ".project",
            "project.",
            "project..name",
            "project name",
        ] {
            assert!(
                FieldId::new(invalid).is_err(),
                "accepted invalid field ID {invalid}"
            );
        }
        assert_eq!(
            FieldId::new("analysis.tran.stop-time")
                .expect("valid field ID")
                .as_str(),
            "analysis.tran.stop-time"
        );
        assert!(CommandId::new("create.result").is_err());
        assert!(ValidationCode::new("GUI-RESULT-DATASET-INCOMPATIBLE").is_ok());
        assert!(ValidationCode::new("gui-result-invalid").is_err());
    }

    #[test]
    fn identifier_deserialization_is_validated() {
        assert!(serde_json::from_str::<FieldId>("\"bad field\"").is_err());
        assert!(serde_json::from_str::<CommandId>("\"create-result-document\"").is_ok());
    }

    #[test]
    fn command_outcomes_round_trip_with_durable_receipts() {
        let project = ProjectId::new();
        let receipt = TransactionReceipt::new(
            CommandId::new("rename-project").expect("command id"),
            ObjectRef::new(ProductObjectKind::Project, project.as_uuid())
                .expect("project identity is non-nil"),
            ObjectRevision::new(2).expect("revision"),
            42,
        );
        let outcome = CommandOutcome::Committed {
            value: "precision-afe".to_owned(),
            receipt,
        };
        let json = serde_json::to_string(&outcome).expect("serialize outcome");
        let restored: CommandOutcome<String> =
            serde_json::from_str(&json).expect("deserialize outcome");

        assert!(restored.is_committed());
        assert_eq!(restored, outcome);
    }
}
