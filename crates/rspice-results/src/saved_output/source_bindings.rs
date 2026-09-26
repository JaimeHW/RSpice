//! Exact retained source identities; absence and malformed null remain distinct.

use std::collections::BTreeMap;
#[cfg(feature = "engine-evidence")]
mod validation;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedOutputSourceBindings {
    pub axis: SavedOutputAxis,
    pub references: BTreeMap<String, SavedOutputBoundSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SavedOutputAxis {
    OperatingPoint,
    Waveform { name: String },
    DcFamily,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SavedOutputBoundSource {
    Waveform { name: String },
    DcQuantity { quantity: usize },
    Ground,
    Missing,
}

/// Only absence means legacy unknown. Explicit null is malformed evidence.
pub(super) fn deserialize_bindings<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<SavedOutputSourceBindings>, D::Error> {
    serde::Deserialize::deserialize(deserializer).map(Some)
}
