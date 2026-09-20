//! Rating metadata from the exact model selected during circuit construction.
use super::*;
use std::collections::BTreeMap;

/// An authored safety parameter after normal elaboration. Inspection never
/// evaluates expressions again: in particular it must not draw new random values.
#[derive(Debug, Clone, PartialEq)]
pub enum ModelSafetyValue {
    Numeric(f64),
    /// Kept explicitly so an importer can reject an unavailable limit instead
    /// of silently treating the device as unrated.
    Unresolved(String),
}

/// Native BJT implementation selected after normal model routing, including
/// parameter-inferred VBIC cards without an explicit LEVEL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BjtModelSafetyFamily {
    GummelPoon,
    Vbic,
}

/// Native MOS families with qualified model-card voltage-rating conventions.
/// Recorded at the selected construction route rather than inferred from LEVEL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosModelSafetyFamily {
    Bsim3,
    Bsim4,
    Vdmos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MosModelSafety {
    pub family: MosModelSafetyFamily,
    /// Polarity resolved by the engine, including VDMOS type aliases and flags.
    pub p_channel: bool,
}

/// Compact safety-related subset of a selected model card. Values retain the
/// card's units and spelling-independent uppercase keys; these are metadata,
/// not a claim that every device implementation supports every rating.
#[derive(Debug, Clone)]
pub struct DeviceModelSafety {
    pub model_name: String,
    pub model_type: String,
    pub generated: bool,
    pub bjt_family: Option<BjtModelSafetyFamily>,
    pub mos: Option<MosModelSafety>,
    pub parameters: BTreeMap<String, ModelSafetyValue>,
}

impl CircuitData {
    /// Model ratings for a concrete flattened device, including scoped/bin and
    /// statistical model selection. No entry means no authored rating fields.
    pub fn device_model_safety(&self, device: &str) -> Option<&DeviceModelSafety> {
        self.device_model_safety.get(&device.to_ascii_uppercase())
    }
}
