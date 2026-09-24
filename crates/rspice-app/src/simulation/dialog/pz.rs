//! Pole-Zero Analysis Configuration
//!
//! Configuration for pole-zero analysis (.pz).
//! Computes poles and zeros of a network transfer function.
//!
//! The extraction is a generalized eigenvalue solve, not an iteration, so
//! there is no iteration count or convergence tolerance to configure — the
//! ports and which roots to report are the whole contract.

use serde::{Deserialize, Deserializer};

/// Pole-Zero analysis type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PzAnalysisType {
    #[default]
    PolesAndZeros,
    PolesOnly,
    ZerosOnly,
}

/// Transfer function type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PzTransferType {
    #[default]
    Voltage,
    Current,
}

/// Pole-Zero analysis configuration
#[derive(Debug, Clone)]
pub struct PzConfig {
    /// Input positive node
    pub input_pos: String,
    /// Input negative node
    pub input_neg: String,
    /// Output positive node
    pub output_pos: String,
    /// Output negative node
    pub output_neg: String,
    /// Transfer type
    pub transfer_type: PzTransferType,
    /// Analysis type
    pub analysis_type: PzAnalysisType,
}

impl Default for PzConfig {
    fn default() -> Self {
        Self {
            input_pos: "IN".into(),
            input_neg: "0".into(),
            output_pos: "OUT".into(),
            output_neg: "0".into(),
            transfer_type: PzTransferType::Voltage,
            analysis_type: PzAnalysisType::PolesAndZeros,
        }
    }
}

impl PzConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.input_pos.is_empty() {
            return Err("Input positive node required".into());
        }
        if self.output_pos.is_empty() {
            return Err("Output positive node required".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PzDialogState {
    pub input_pos: String,
    pub input_neg: String,
    pub output_pos: String,
    pub output_neg: String,
    pub transfer_idx: usize,
    pub analysis_idx: usize,
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedPzDialogState {
    #[serde(default)]
    input_pos: String,
    #[serde(default)]
    input_neg: String,
    #[serde(default)]
    output_pos: String,
    #[serde(default)]
    output_neg: String,
    #[serde(default)]
    transfer_idx: usize,
    #[serde(default)]
    analysis_idx: usize,
    /// Retired. The eigenvalue extraction has no tolerance to accept, so this
    /// never reached it. Accepted so earlier projects still open.
    #[serde(default)]
    #[allow(dead_code)]
    tolerance: serde::de::IgnoredAny,
}

impl<'de> Deserialize<'de> for PzDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedPzDialogState::deserialize(deserializer)?;
        Ok(Self {
            input_pos: persisted.input_pos,
            input_neg: persisted.input_neg,
            output_pos: persisted.output_pos,
            output_neg: persisted.output_neg,
            transfer_idx: persisted.transfer_idx,
            analysis_idx: persisted.analysis_idx,
            initialized: false,
        })
    }
}

impl PzDialogState {
    pub fn from_config(config: &PzConfig) -> Self {
        Self {
            input_pos: config.input_pos.clone(),
            input_neg: config.input_neg.clone(),
            output_pos: config.output_pos.clone(),
            output_neg: config.output_neg.clone(),
            transfer_idx: match config.transfer_type {
                PzTransferType::Voltage => 0,
                PzTransferType::Current => 1,
            },
            analysis_idx: match config.analysis_type {
                PzAnalysisType::PolesAndZeros => 0,
                PzAnalysisType::PolesOnly => 1,
                PzAnalysisType::ZerosOnly => 2,
            },
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<PzConfig, String> {
        let transfer = match self.transfer_idx {
            0 => PzTransferType::Voltage,
            _ => PzTransferType::Current,
        };
        let analysis = match self.analysis_idx {
            0 => PzAnalysisType::PolesAndZeros,
            1 => PzAnalysisType::PolesOnly,
            _ => PzAnalysisType::ZerosOnly,
        };
        let config = PzConfig {
            input_pos: self.input_pos.clone(),
            input_neg: self.input_neg.clone(),
            output_pos: self.output_pos.clone(),
            output_neg: self.output_neg.clone(),
            transfer_type: transfer,
            analysis_type: analysis,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&PzConfig::default());
        }
    }
}
