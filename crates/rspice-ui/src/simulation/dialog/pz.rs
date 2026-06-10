//! Pole-Zero Analysis Configuration
//!
//! Configuration for pole-zero analysis (.pz).
//! Computes poles and zeros of a network transfer function.

use super::options::parse_si_value;

/// Pole-Zero analysis type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PzAnalysisType {
    #[default]
    PolesAndZeros,
    PolesOnly,
    ZerosOnly,
}

impl PzAnalysisType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::PolesAndZeros => "Poles & Zeros",
            Self::PolesOnly => "Poles Only",
            Self::ZerosOnly => "Zeros Only",
        }
    }
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::PolesAndZeros => "pz",
            Self::PolesOnly => "pol",
            Self::ZerosOnly => "zer",
        }
    }
    pub fn all() -> &'static [PzAnalysisType] {
        &[Self::PolesAndZeros, Self::PolesOnly, Self::ZerosOnly]
    }
}

/// Transfer function type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PzTransferType {
    #[default]
    Voltage,
    Current,
}

impl PzTransferType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Voltage => "Voltage (V/V)",
            Self::Current => "Current (I/I)",
        }
    }
    pub fn spice_keyword(&self) -> &'static str {
        match self {
            Self::Voltage => "vol",
            Self::Current => "cur",
        }
    }
    pub fn all() -> &'static [PzTransferType] {
        &[Self::Voltage, Self::Current]
    }
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
    /// Maximum iterations
    pub max_iter: u32,
    /// Tolerance
    pub tolerance: f64,
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
            max_iter: 100,
            tolerance: 1e-6,
        }
    }
}

impl PzConfig {
    pub fn new(input: &str, output: &str) -> Self {
        Self {
            input_pos: input.to_uppercase(),
            output_pos: output.to_uppercase(),
            ..Default::default()
        }
    }

    pub fn with_type(mut self, t: PzAnalysisType) -> Self {
        self.analysis_type = t;
        self
    }
    pub fn with_transfer(mut self, t: PzTransferType) -> Self {
        self.transfer_type = t;
        self
    }

    pub fn to_spice(&self) -> String {
        format!(
            ".pz {} {} {} {} {} {}",
            self.input_pos,
            self.input_neg,
            self.output_pos,
            self.output_neg,
            self.transfer_type.spice_keyword(),
            self.analysis_type.spice_keyword()
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.input_pos.is_empty() {
            return Err("Input positive node required".into());
        }
        if self.output_pos.is_empty() {
            return Err("Output positive node required".into());
        }
        if self.tolerance <= 0.0 {
            return Err("Tolerance must be positive".into());
        }
        if self.max_iter == 0 {
            return Err("Max iterations must be at least 1".into());
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct PzDialogState {
    pub input_pos: String,
    pub input_neg: String,
    pub output_pos: String,
    pub output_neg: String,
    pub transfer_idx: usize,
    pub analysis_idx: usize,
    pub tolerance: String,
    pub initialized: bool,
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
            tolerance: format!("{:.0e}", config.tolerance),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<PzConfig, String> {
        let tol = parse_si_value(&self.tolerance).map_err(|e| format!("Bad tolerance: {}", e))?;
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
            max_iter: 100,
            tolerance: tol,
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
