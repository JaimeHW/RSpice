use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::{ModelLevel, ModelType};

/// A single device model definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceModel {
    /// Model name (e.g., "nmos_3p3_svt")
    pub name: String,
    /// Model type
    pub model_type: ModelType,
    /// Model level
    pub level: ModelLevel,
    /// Description
    pub description: String,
    /// Minimum channel length (for MOS)
    pub l_min: Option<f64>,
    /// Maximum channel length
    pub l_max: Option<f64>,
    /// Minimum channel width
    pub w_min: Option<f64>,
    /// Maximum channel width
    pub w_max: Option<f64>,
    /// Operating voltage (Vdd)
    pub vdd: Option<f64>,
    /// Threshold voltage (typical)
    pub vth0: Option<f64>,
    /// Model file path
    pub file_path: Option<PathBuf>,
    /// Model parameters (key-value)
    pub parameters: HashMap<String, f64>,
}

impl Default for DeviceModel {
    fn default() -> Self {
        Self {
            name: String::new(),
            model_type: ModelType::Nmos,
            level: ModelLevel::Bsim4,
            description: String::new(),
            l_min: None,
            l_max: None,
            w_min: None,
            w_max: None,
            vdd: None,
            vth0: None,
            file_path: None,
            parameters: HashMap::new(),
        }
    }
}

impl DeviceModel {
    /// Create a new model
    pub fn new(name: impl Into<String>, model_type: ModelType) -> Self {
        Self {
            name: name.into(),
            model_type,
            ..Default::default()
        }
    }

    /// Set level
    pub fn with_level(mut self, level: ModelLevel) -> Self {
        self.level = level;
        self
    }

    /// Set geometry limits
    pub fn with_geometry(mut self, l_min: f64, l_max: f64, w_min: f64, w_max: f64) -> Self {
        self.l_min = Some(l_min);
        self.l_max = Some(l_max);
        self.w_min = Some(w_min);
        self.w_max = Some(w_max);
        self
    }

    /// Set operating voltage
    pub fn with_vdd(mut self, vdd: f64) -> Self {
        self.vdd = Some(vdd);
        self
    }

    /// Add a parameter
    pub fn add_parameter(&mut self, name: &str, value: f64) {
        self.parameters.insert(name.to_string(), value);
    }

    /// Check if geometry is within model limits
    pub fn check_geometry(&self, l: f64, w: f64) -> bool {
        let l_ok = match (self.l_min, self.l_max) {
            (Some(min), Some(max)) => l >= min && l <= max,
            (Some(min), None) => l >= min,
            (None, Some(max)) => l <= max,
            (None, None) => true,
        };
        let w_ok = match (self.w_min, self.w_max) {
            (Some(min), Some(max)) => w >= min && w <= max,
            (Some(min), None) => w >= min,
            (None, Some(max)) => w <= max,
            (None, None) => true,
        };
        l_ok && w_ok
    }
}
