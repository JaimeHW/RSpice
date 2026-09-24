//! Source-qualified provider decisions and ordered model-library bindings.

use rspice_app_types::product::ContentDigest;
use serde::{Deserialize, Serialize};

pub const MODEL_RESOLUTION_RECORD_SCHEMA_VERSION: u16 = 1;

/// Consumer namespace governed by one explicit provider decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelConsumerScope {
    PrimitiveModel,
    Subcircuit,
}

impl ModelConsumerScope {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PrimitiveModel => "primitive model",
            Self::Subcircuit => "subcircuit",
        }
    }

    /// Key for a definition name already normalized by the caller.
    #[must_use]
    pub fn record_key(self, normalized_name: &str) -> String {
        let namespace = match self {
            Self::PrimitiveModel => "model",
            Self::Subcircuit => "subckt",
        };
        format!("{namespace}:{normalized_name}")
    }
}

/// Exact project-owned decision for a contested executable definition.
///
/// The provider's authenticated source digest makes the decision expire when
/// a source is refreshed, even if the library and definition names are reused.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelResolutionRecord {
    pub schema_version: u16,
    pub consumer_scope: ModelConsumerScope,
    pub normalized_name: String,
    pub provider_library: String,
    pub provider_definition: String,
    pub provider_source_digest: ContentDigest,
    pub audit_reason: String,
    pub created_at_unix_ms: u64,
}

impl ModelResolutionRecord {
    #[must_use]
    pub fn key(&self) -> String {
        self.consumer_scope.record_key(&self.normalized_name)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != MODEL_RESOLUTION_RECORD_SCHEMA_VERSION {
            return Err(format!(
                "model-resolution record for '{}' uses unsupported schema {}",
                self.normalized_name, self.schema_version
            ));
        }
        let normalized = self.normalized_name.trim().to_ascii_lowercase();
        if normalized.is_empty() || normalized != self.normalized_name {
            return Err("model-resolution name must be nonempty canonical lowercase".to_owned());
        }
        if self.provider_definition.to_ascii_lowercase() != self.normalized_name {
            return Err(
                "model-resolution provider definition does not match its canonical name".to_owned(),
            );
        }
        for (field, value, maximum) in [
            (
                "provider library",
                self.provider_library.as_str(),
                512_usize,
            ),
            (
                "provider definition",
                self.provider_definition.as_str(),
                512_usize,
            ),
            ("audit reason", self.audit_reason.as_str(), 2_048_usize),
        ] {
            if value.is_empty()
                || value != value.trim()
                || value.len() > maximum
                || value.chars().any(|character| {
                    character.is_control()
                        && !(field == "audit reason" && matches!(character, '\n' | '\r' | '\t'))
                })
            {
                return Err(format!(
                    "model-resolution {field} must be nonempty, trimmed, at most {maximum} bytes, and contain no unsupported control characters"
                ));
            }
        }
        if self.created_at_unix_ms == 0 {
            return Err("model-resolution timestamp must be nonzero".to_owned());
        }
        Ok(())
    }
}

/// One ordered model-library binding owned by a simulation plan.
///
/// The name is the project-catalog identity, the digest prevents a refreshed
/// or replaced source from being accepted under an old plan, and the optional
/// corner is the plan's nominal section override. Vector order is executable
/// precedence; it is never reconstructed from the manager's hash map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanModelBinding {
    pub library_name: String,
    pub source_digest: ContentDigest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_corner: Option<String>,
}

impl SimulationPlanModelBinding {
    pub fn validate(&self) -> Result<(), String> {
        for (field, value) in [("library name", self.library_name.as_str())]
            .into_iter()
            .chain(
                self.selected_corner
                    .as_deref()
                    .map(|value| ("corner section", value)),
            )
        {
            if value.is_empty() || value != value.trim() || value.chars().any(char::is_control) {
                return Err(format!(
                    "Simulation-plan model {field} must be nonempty, trimmed, and control-free"
                ));
            }
        }
        Ok(())
    }
}
