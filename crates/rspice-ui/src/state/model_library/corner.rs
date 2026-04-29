use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A process corner definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessCorner {
    /// Corner name (e.g., "tt", "ff", "ss")
    pub name: String,
    /// Description
    pub description: String,
    /// NMOS corner (typical, fast, slow)
    pub nmos_corner: String,
    /// PMOS corner
    pub pmos_corner: String,
    /// Temperature
    pub temperature: f64,
    /// Supply voltage adjustment factor
    pub vdd_factor: f64,
    /// Corner file path
    pub file_path: Option<PathBuf>,
    /// Whether this is the default/typical corner
    pub is_default: bool,
}

impl Default for ProcessCorner {
    fn default() -> Self {
        Self {
            name: "tt".to_string(),
            description: "Typical-Typical".to_string(),
            nmos_corner: "typical".to_string(),
            pmos_corner: "typical".to_string(),
            temperature: 27.0,
            vdd_factor: 1.0,
            file_path: None,
            is_default: true,
        }
    }
}

impl ProcessCorner {
    /// Create a new corner
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_default: false,
            ..Default::default()
        }
    }

    /// Standard corners for a PDK
    pub fn standard_corners() -> Vec<ProcessCorner> {
        vec![
            ProcessCorner {
                name: "tt".to_string(),
                description: "Typical-Typical".to_string(),
                nmos_corner: "typical".to_string(),
                pmos_corner: "typical".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                is_default: true,
                ..Default::default()
            },
            ProcessCorner {
                name: "ff".to_string(),
                description: "Fast-Fast".to_string(),
                nmos_corner: "fast".to_string(),
                pmos_corner: "fast".to_string(),
                temperature: -40.0,
                vdd_factor: 1.1,
                file_path: None,
                is_default: false,
            },
            ProcessCorner {
                name: "ss".to_string(),
                description: "Slow-Slow".to_string(),
                nmos_corner: "slow".to_string(),
                pmos_corner: "slow".to_string(),
                temperature: 125.0,
                vdd_factor: 0.9,
                file_path: None,
                is_default: false,
            },
            ProcessCorner {
                name: "sf".to_string(),
                description: "Slow-Fast".to_string(),
                nmos_corner: "slow".to_string(),
                pmos_corner: "fast".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                file_path: None,
                is_default: false,
            },
            ProcessCorner {
                name: "fs".to_string(),
                description: "Fast-Slow".to_string(),
                nmos_corner: "fast".to_string(),
                pmos_corner: "slow".to_string(),
                temperature: 27.0,
                vdd_factor: 1.0,
                file_path: None,
                is_default: false,
            },
        ]
    }
}
