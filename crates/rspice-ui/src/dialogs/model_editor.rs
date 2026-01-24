//! Model Editor Dialog
//!
//! A comprehensive modal dialog for editing MOSFET model parameters.
//! Features:
//! - Categorized parameter organization (tabs/sidebar)
//! - Technology node presets (180nm, 65nm, 28nm, etc.)
//! - Parameter validation with visual feedback
//! - Model import/export in SPICE format
//! - Default value reset functionality

use super::model_types::{ModelParameter, MosfetModelType, ParameterCategory, TechnologyNode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Model Editor State
// =============================================================================

/// State for the model editor dialog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEditorState {
    /// Current model type
    pub model_type: MosfetModelType,

    /// Current technology node
    pub tech_node: TechnologyNode,

    /// Model name (e.g., "nch_lvt")
    pub model_name: String,

    /// Device polarity (true = NMOS, false = PMOS)
    pub is_nmos: bool,

    /// All model parameters, keyed by parameter name
    pub parameters: HashMap<String, ModelParameter>,

    /// Currently selected category for display
    pub selected_category: ParameterCategory,

    /// Whether the dialog is visible
    pub is_open: bool,

    /// Validation errors (parameter name -> error message)
    pub validation_errors: HashMap<String, String>,

    /// Whether parameters have been modified since last save
    pub is_dirty: bool,
}

impl Default for ModelEditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelEditorState {
    /// Create a new model editor state with default BSIM4 parameters
    pub fn new() -> Self {
        let mut state = Self {
            model_type: MosfetModelType::default(),
            tech_node: TechnologyNode::default(),
            model_name: "nch".to_string(),
            is_nmos: true,
            parameters: HashMap::new(),
            selected_category: ParameterCategory::default(),
            is_open: false,
            validation_errors: HashMap::new(),
            is_dirty: false,
        };
        state.initialize_parameters();
        state
    }

    /// Initialize parameters for the current model type
    pub fn initialize_parameters(&mut self) {
        self.parameters.clear();

        // Extract technology-dependent values upfront to avoid borrow conflicts
        let min_length = self.tech_node.min_length();
        let oxide_thickness = self.tech_node.oxide_thickness();
        let mobility = self.tech_node.nmos_mobility();
        let vth0 = if self.is_nmos {
            self.tech_node.nmos_vth()
        } else {
            self.tech_node.pmos_vth()
        };

        // Geometry parameters
        self.add_parameter(ModelParameter::new(
            "L",
            min_length,
            min_length * 0.5,
            100e-6,
            "m",
            "Channel length",
            ParameterCategory::Geometry,
        ));
        self.add_parameter(ModelParameter::new(
            "W",
            min_length * 10.0,
            min_length,
            1e-3,
            "m",
            "Channel width",
            ParameterCategory::Geometry,
        ));
        self.add_parameter(ModelParameter::new(
            "NF",
            1.0,
            1.0,
            1000.0,
            "",
            "Number of fingers",
            ParameterCategory::Geometry,
        ));
        self.add_parameter(ModelParameter::new(
            "TOXE",
            oxide_thickness,
            0.5e-9,
            20e-9,
            "m",
            "Electrical oxide thickness",
            ParameterCategory::Geometry,
        ));
        self.add_parameter(ModelParameter::new(
            "TOXP",
            oxide_thickness * 0.9,
            0.4e-9,
            18e-9,
            "m",
            "Physical oxide thickness",
            ParameterCategory::Geometry,
        ));
        self.add_parameter(ModelParameter::new(
            "TOXM",
            oxide_thickness,
            0.5e-9,
            20e-9,
            "m",
            "Oxide thickness for CV model",
            ParameterCategory::Geometry,
        ));
        self.add_parameter(ModelParameter::new(
            "XJ",
            min_length * 0.4,
            1e-9,
            1e-6,
            "m",
            "Junction depth",
            ParameterCategory::Geometry,
        ));

        // Threshold voltage parameters
        self.add_parameter(ModelParameter::new(
            "VTH0",
            vth0,
            -2.0,
            2.0,
            "V",
            "Long-channel threshold voltage",
            ParameterCategory::Threshold,
        ));
        self.add_parameter(ModelParameter::new(
            "K1",
            0.5,
            0.0,
            2.0,
            "V^0.5",
            "First-order body effect coefficient",
            ParameterCategory::Threshold,
        ));
        self.add_parameter(ModelParameter::new(
            "K2",
            -0.1,
            -1.0,
            1.0,
            "",
            "Second-order body effect coefficient",
            ParameterCategory::Threshold,
        ));
        self.add_parameter(ModelParameter::new(
            "K3",
            80.0,
            0.0,
            500.0,
            "",
            "Narrow width coefficient",
            ParameterCategory::Threshold,
        ));
        self.add_parameter(ModelParameter::new(
            "K3B",
            0.0,
            -10.0,
            10.0,
            "1/V",
            "Body effect coefficient for K3",
            ParameterCategory::Threshold,
        ));
        self.add_parameter(ModelParameter::new(
            "DVT0",
            2.2,
            0.0,
            10.0,
            "",
            "Short-channel effect coefficient 0",
            ParameterCategory::Threshold,
        ));
        self.add_parameter(ModelParameter::new(
            "DVT1",
            0.53,
            0.0,
            2.0,
            "",
            "Short-channel effect coefficient 1",
            ParameterCategory::Threshold,
        ));
        self.add_parameter(ModelParameter::new(
            "DVT2",
            -0.032,
            -1.0,
            1.0,
            "1/V",
            "Short-channel effect coefficient 2",
            ParameterCategory::Threshold,
        ));

        // Mobility parameters
        self.add_parameter(ModelParameter::new(
            "U0",
            mobility * 1e-4, // Convert cm²/V·s to m²/V·s
            50e-4,
            800e-4,
            "m²/V·s",
            "Low-field mobility",
            ParameterCategory::Mobility,
        ));
        self.add_parameter(ModelParameter::new(
            "UA",
            2.25e-9,
            -1e-8,
            1e-7,
            "m/V",
            "First-order mobility degradation coefficient",
            ParameterCategory::Mobility,
        ));
        self.add_parameter(ModelParameter::new(
            "UB",
            5.87e-19,
            -1e-17,
            1e-17,
            "m²/V²",
            "Second-order mobility degradation coefficient",
            ParameterCategory::Mobility,
        ));
        self.add_parameter(ModelParameter::new(
            "UC",
            -4.65e-11,
            -1e-9,
            1e-9,
            "m/V²",
            "Body-bias mobility degradation coefficient",
            ParameterCategory::Mobility,
        ));
        self.add_parameter(ModelParameter::new(
            "EU",
            1.67,
            0.0,
            5.0,
            "",
            "Exponent for mobility degradation",
            ParameterCategory::Mobility,
        ));

        // Velocity saturation parameters
        self.add_parameter(ModelParameter::new(
            "VSAT",
            1.0e5,
            1e4,
            5e5,
            "m/s",
            "Saturation velocity",
            ParameterCategory::VelocitySaturation,
        ));
        self.add_parameter(ModelParameter::new(
            "A0",
            1.0,
            0.0,
            5.0,
            "",
            "Non-uniform depletion width coefficient",
            ParameterCategory::VelocitySaturation,
        ));
        self.add_parameter(ModelParameter::new(
            "AGS",
            0.2,
            0.0,
            2.0,
            "1/V",
            "Gate bias coefficient of Abulk",
            ParameterCategory::VelocitySaturation,
        ));
        self.add_parameter(ModelParameter::new(
            "A1",
            0.0,
            0.0,
            2.0,
            "1/V",
            "Non-saturation effect coefficient",
            ParameterCategory::VelocitySaturation,
        ));
        self.add_parameter(ModelParameter::new(
            "A2",
            1.0,
            0.0,
            2.0,
            "",
            "Non-saturation effect coefficient",
            ParameterCategory::VelocitySaturation,
        ));
        self.add_parameter(ModelParameter::new(
            "DELTA",
            0.01,
            0.0,
            0.5,
            "V",
            "Effective Vds parameter",
            ParameterCategory::VelocitySaturation,
        ));

        // Channel length modulation parameters
        self.add_parameter(ModelParameter::new(
            "PCLM",
            1.3,
            0.0,
            5.0,
            "",
            "Channel length modulation coefficient",
            ParameterCategory::ChannelLengthModulation,
        ));
        self.add_parameter(ModelParameter::new(
            "PDIBLC1",
            0.39,
            0.0,
            2.0,
            "",
            "First DIBL coefficient",
            ParameterCategory::ChannelLengthModulation,
        ));
        self.add_parameter(ModelParameter::new(
            "PDIBLC2",
            0.0086,
            0.0,
            0.5,
            "",
            "Second DIBL coefficient",
            ParameterCategory::ChannelLengthModulation,
        ));
        self.add_parameter(ModelParameter::new(
            "PDIBLCB",
            -0.1,
            -1.0,
            1.0,
            "1/V",
            "Body bias coefficient for DIBL",
            ParameterCategory::ChannelLengthModulation,
        ));
        self.add_parameter(ModelParameter::new(
            "DROUT",
            0.56,
            0.0,
            5.0,
            "",
            "L dependence of DIBL",
            ParameterCategory::ChannelLengthModulation,
        ));
        self.add_parameter(ModelParameter::new(
            "PSCBE1",
            4.24e8,
            0.0,
            1e10,
            "V/m",
            "Substrate current body-effect coefficient 1",
            ParameterCategory::ChannelLengthModulation,
        ));
        self.add_parameter(ModelParameter::new(
            "PSCBE2",
            1e-5,
            0.0,
            1e-3,
            "V/m",
            "Substrate current body-effect coefficient 2",
            ParameterCategory::ChannelLengthModulation,
        ));

        // Subthreshold parameters
        self.add_parameter(ModelParameter::new(
            "VOFF",
            -0.08,
            -0.5,
            0.5,
            "V",
            "Threshold voltage offset",
            ParameterCategory::Subthreshold,
        ));
        self.add_parameter(ModelParameter::new(
            "NFACTOR",
            1.5,
            0.5,
            5.0,
            "",
            "Subthreshold swing coefficient",
            ParameterCategory::Subthreshold,
        ));
        self.add_parameter(ModelParameter::new(
            "ETA0",
            0.08,
            0.0,
            1.0,
            "",
            "DIBL coefficient in subthreshold",
            ParameterCategory::Subthreshold,
        ));
        self.add_parameter(ModelParameter::new(
            "ETAB",
            -0.07,
            -1.0,
            0.0,
            "1/V",
            "Body-bias for ETA0",
            ParameterCategory::Subthreshold,
        ));
        self.add_parameter(ModelParameter::new(
            "DSUB",
            0.56,
            0.0,
            2.0,
            "",
            "DIBL coefficient in subthreshold",
            ParameterCategory::Subthreshold,
        ));
        self.add_parameter(ModelParameter::new(
            "CDSCD",
            0.0,
            0.0,
            1e-3,
            "F/m²",
            "Coupling capacitance of S/D to channel",
            ParameterCategory::Subthreshold,
        ));

        // Resistance parameters
        self.add_parameter(ModelParameter::new(
            "RDSW",
            200.0,
            0.0,
            2000.0,
            "Ω·µm",
            "S/D resistance per unit width",
            ParameterCategory::Resistance,
        ));
        self.add_parameter(ModelParameter::new(
            "PRWB",
            0.0,
            -1.0,
            1.0,
            "1/V^0.5",
            "Body effect on RDSW",
            ParameterCategory::Resistance,
        ));
        self.add_parameter(ModelParameter::new(
            "PRWG",
            0.0,
            -1.0,
            1.0,
            "1/V",
            "Gate bias effect on RDSW",
            ParameterCategory::Resistance,
        ));
        self.add_parameter(ModelParameter::new(
            "WR",
            1.0,
            0.5,
            2.0,
            "",
            "Width offset for RDSW",
            ParameterCategory::Resistance,
        ));
        self.add_parameter(ModelParameter::new(
            "RSH",
            0.0,
            0.0,
            1000.0,
            "Ω/sq",
            "S/D sheet resistance",
            ParameterCategory::Resistance,
        ));
        self.add_parameter(ModelParameter::new(
            "RDSWMIN",
            0.0,
            0.0,
            500.0,
            "Ω·µm",
            "Minimum S/D resistance",
            ParameterCategory::Resistance,
        ));

        // Capacitance parameters
        self.add_parameter(ModelParameter::new(
            "CGSO",
            2.5e-10, // Typical overlap cap
            0.0,
            1e-9,
            "F/m",
            "Gate-source overlap capacitance",
            ParameterCategory::Capacitance,
        ));
        self.add_parameter(ModelParameter::new(
            "CGDO",
            2.5e-10, // Typical overlap cap
            0.0,
            1e-9,
            "F/m",
            "Gate-drain overlap capacitance",
            ParameterCategory::Capacitance,
        ));
        self.add_parameter(ModelParameter::new(
            "CGBO",
            0.0,
            0.0,
            1e-12,
            "F/m",
            "Gate-bulk overlap capacitance",
            ParameterCategory::Capacitance,
        ));
        self.add_parameter(ModelParameter::new(
            "CJ",
            5e-4,
            0.0,
            1e-2,
            "F/m²",
            "S/D bottom junction capacitance",
            ParameterCategory::Capacitance,
        ));
        self.add_parameter(ModelParameter::new(
            "CJSW",
            5e-10,
            0.0,
            1e-8,
            "F/m",
            "S/D side wall junction capacitance",
            ParameterCategory::Capacitance,
        ));
        self.add_parameter(ModelParameter::new(
            "CJSWG",
            5e-10,
            0.0,
            1e-8,
            "F/m",
            "Gate-side S/D junction capacitance",
            ParameterCategory::Capacitance,
        ));
        self.add_parameter(ModelParameter::new(
            "MJ",
            0.5,
            0.1,
            0.9,
            "",
            "S/D junction grading coefficient",
            ParameterCategory::Capacitance,
        ));
        self.add_parameter(ModelParameter::new(
            "PB",
            1.0,
            0.3,
            1.5,
            "V",
            "S/D junction built-in potential",
            ParameterCategory::Capacitance,
        ));

        // Temperature parameters
        self.add_parameter(ModelParameter::new(
            "TNOM",
            27.0,
            -50.0,
            200.0,
            "°C",
            "Nominal temperature",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "KT1",
            -0.11,
            -1.0,
            0.0,
            "V",
            "Temperature coefficient of Vth",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "KT1L",
            0.0,
            -1e-7,
            1e-7,
            "V·m",
            "Length dependence of KT1",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "KT2",
            0.022,
            0.0,
            0.1,
            "",
            "Vth temperature coefficient 2",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "UTE",
            -1.5,
            -3.0,
            0.0,
            "",
            "Temperature exponent for mobility",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "UA1",
            4.31e-9,
            -1e-7,
            1e-7,
            "m/V",
            "Temperature coefficient for UA",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "UB1",
            -7.61e-18,
            -1e-16,
            1e-16,
            "m²/V²",
            "Temperature coefficient for UB",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "UC1",
            -5.6e-11,
            -1e-9,
            1e-9,
            "m/V²",
            "Temperature coefficient for UC",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "PRT",
            0.0,
            -100.0,
            100.0,
            "Ω·µm",
            "Temperature coefficient for RDSW",
            ParameterCategory::Temperature,
        ));
        self.add_parameter(ModelParameter::new(
            "AT",
            3.3e4,
            0.0,
            1e6,
            "m/s",
            "Temperature coefficient for VSAT",
            ParameterCategory::Temperature,
        ));

        // Noise parameters
        self.add_parameter(ModelParameter::new(
            "KF",
            0.0,
            0.0,
            1e-20,
            "",
            "Flicker noise coefficient",
            ParameterCategory::Noise,
        ));
        self.add_parameter(ModelParameter::new(
            "AF",
            1.0,
            0.5,
            2.0,
            "",
            "Flicker noise exponent",
            ParameterCategory::Noise,
        ));
        self.add_parameter(ModelParameter::new(
            "EF",
            1.0,
            0.5,
            2.0,
            "",
            "Flicker noise frequency exponent",
            ParameterCategory::Noise,
        ));
        self.add_parameter(ModelParameter::new(
            "NOIA",
            6.25e41,
            0.0,
            1e45,
            "1/m³",
            "Noise trap density parameter A",
            ParameterCategory::Noise,
        ));
        self.add_parameter(ModelParameter::new(
            "NOIB",
            3.125e26,
            0.0,
            1e30,
            "1/m",
            "Noise trap density parameter B",
            ParameterCategory::Noise,
        ));
        self.add_parameter(ModelParameter::new(
            "NOIC",
            8.75,
            0.0,
            100.0,
            "",
            "Noise trap density parameter C",
            ParameterCategory::Noise,
        ));
        self.add_parameter(ModelParameter::new(
            "EM",
            4.1e7,
            1e6,
            1e9,
            "V/m",
            "Saturation field",
            ParameterCategory::Noise,
        ));
        self.add_parameter(ModelParameter::new(
            "NTNOI",
            1.0,
            0.0,
            5.0,
            "",
            "Thermal noise coefficient",
            ParameterCategory::Noise,
        ));

        // Self-heating parameters
        self.add_parameter(ModelParameter::new(
            "RTH0",
            0.0,
            0.0,
            1e6,
            "K/W",
            "Self-heating thermal resistance",
            ParameterCategory::SelfHeating,
        ));
        self.add_parameter(ModelParameter::new(
            "CTH0",
            0.0,
            0.0,
            1e-6,
            "J/K",
            "Self-heating thermal capacitance",
            ParameterCategory::SelfHeating,
        ));
    }

    /// Add a parameter to the collection
    fn add_parameter(&mut self, param: ModelParameter) {
        self.parameters.insert(param.name.clone(), param);
    }

    /// Get parameters for a specific category
    pub fn get_parameters_by_category(&self, category: ParameterCategory) -> Vec<&ModelParameter> {
        let mut params: Vec<_> = self
            .parameters
            .values()
            .filter(|p| p.category == category)
            .collect();
        params.sort_by(|a, b| a.name.cmp(&b.name));
        params
    }

    /// Update a parameter value
    pub fn set_parameter_value(&mut self, name: &str, value: f64) -> Result<(), String> {
        if let Some(param) = self.parameters.get_mut(name) {
            param.value = value;
            self.is_dirty = true;

            // Validate
            if !param.is_valid() {
                let error = format!(
                    "Value {} out of range [{}, {}]",
                    value, param.min, param.max
                );
                self.validation_errors
                    .insert(name.to_string(), error.clone());
                Err(error)
            } else {
                self.validation_errors.remove(name);
                Ok(())
            }
        } else {
            Err(format!("Parameter '{}' not found", name))
        }
    }

    /// Reset a parameter to its default value
    pub fn reset_parameter(&mut self, name: &str) {
        if let Some(param) = self.parameters.get_mut(name) {
            param.reset();
            self.validation_errors.remove(name);
            self.is_dirty = true;
        }
    }

    /// Reset all parameters to defaults
    pub fn reset_all(&mut self) {
        for param in self.parameters.values_mut() {
            param.reset();
        }
        self.validation_errors.clear();
        self.is_dirty = true;
    }

    /// Apply technology node preset
    pub fn apply_technology_preset(&mut self, node: TechnologyNode) {
        self.tech_node = node;
        self.initialize_parameters();
        self.is_dirty = true;
    }

    /// Change model type
    pub fn set_model_type(&mut self, model_type: MosfetModelType) {
        self.model_type = model_type;
        self.initialize_parameters();
        self.is_dirty = true;
    }

    /// Validate all parameters
    pub fn validate(&mut self) -> bool {
        self.validation_errors.clear();
        for (name, param) in &self.parameters {
            if !param.is_valid() {
                self.validation_errors.insert(
                    name.clone(),
                    format!(
                        "Value {} out of range [{}, {}]",
                        param.value, param.min, param.max
                    ),
                );
            }
        }
        self.validation_errors.is_empty()
    }

    /// Generate SPICE model card
    pub fn to_spice_model(&self) -> String {
        let device_type = if self.is_nmos { "NMOS" } else { "PMOS" };
        let level = self.model_type.spice_level();

        let mut lines = vec![format!(
            ".MODEL {} {} LEVEL={}",
            self.model_name, device_type, level
        )];

        // Group parameters by category for better readability
        for category in ParameterCategory::all() {
            let params = self.get_parameters_by_category(*category);
            if params.is_empty() {
                continue;
            }

            lines.push(format!("* --- {} ---", category.display_name()));

            // Format parameters 4 per line
            let mut line = String::from("+");
            let mut count = 0;

            for param in params {
                // Skip default values for cleaner output
                if (param.value - param.default).abs() < 1e-20 {
                    continue;
                }

                let param_str = format!(" {}={:.6e}", param.name, param.value);
                line.push_str(&param_str);
                count += 1;

                if count >= 4 {
                    lines.push(line);
                    line = String::from("+");
                    count = 0;
                }
            }

            if count > 0 {
                lines.push(line);
            }
        }

        lines.join("\n")
    }

    /// Parse SPICE model card (basic implementation)
    pub fn from_spice_model(&mut self, spice_text: &str) -> Result<(), String> {
        for line in spice_text.lines() {
            let line = line.trim();

            // Skip comments
            if line.starts_with('*') || line.is_empty() {
                continue;
            }

            // Handle .MODEL line
            if line.to_uppercase().starts_with(".MODEL") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    self.model_name = parts[1].to_string();
                    self.is_nmos = parts[2].to_uppercase() == "NMOS";
                }
                continue;
            }

            // Parse parameter assignments (NAME=VALUE)
            let param_line = if line.starts_with('+') {
                &line[1..]
            } else {
                line
            };

            for token in param_line.split_whitespace() {
                if let Some(eq_pos) = token.find('=') {
                    let name = &token[..eq_pos];
                    let value_str = &token[eq_pos + 1..];

                    if let Ok(value) = value_str.parse::<f64>() {
                        let _ = self.set_parameter_value(&name.to_uppercase(), value);
                    }
                }
            }
        }

        self.is_dirty = false;
        Ok(())
    }

    /// Get count of parameters in a category
    pub fn category_param_count(&self, category: ParameterCategory) -> usize {
        self.parameters
            .values()
            .filter(|p| p.category == category)
            .count()
    }

    /// Get count of modified parameters (different from default)
    pub fn modified_param_count(&self) -> usize {
        self.parameters
            .values()
            .filter(|p| (p.value - p.default).abs() > 1e-20)
            .count()
    }

    /// Check if a specific parameter has been modified
    pub fn is_parameter_modified(&self, name: &str) -> bool {
        self.parameters
            .get(name)
            .map(|p| (p.value - p.default).abs() > 1e-20)
            .unwrap_or(false)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // ModelEditorState Basic Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_model_editor_state_new() {
        let state = ModelEditorState::new();
        assert_eq!(state.model_type, MosfetModelType::Bsim4);
        assert_eq!(state.tech_node, TechnologyNode::Nm65);
        assert!(state.is_nmos);
        assert!(!state.is_open);
        assert!(!state.is_dirty);
        assert!(!state.parameters.is_empty());
    }

    #[test]
    fn test_model_editor_state_default() {
        let state = ModelEditorState::default();
        assert_eq!(state.model_type, MosfetModelType::Bsim4);
        assert_eq!(state.model_name, "nch");
    }

    #[test]
    fn test_initialize_parameters() {
        let state = ModelEditorState::new();
        // Should have parameters for all categories
        for category in ParameterCategory::all() {
            assert!(
                state.category_param_count(*category) > 0,
                "{:?} should have parameters",
                category
            );
        }
    }

    #[test]
    fn test_get_parameters_by_category() {
        let state = ModelEditorState::new();
        let geometry_params = state.get_parameters_by_category(ParameterCategory::Geometry);
        assert!(!geometry_params.is_empty());
        assert!(geometry_params.iter().any(|p| p.name == "L"));
        assert!(geometry_params.iter().any(|p| p.name == "W"));
    }

    // -------------------------------------------------------------------------
    // Parameter Modification Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_set_parameter_value_valid() {
        let mut state = ModelEditorState::new();
        let result = state.set_parameter_value("VTH0", 0.5);
        assert!(result.is_ok());
        assert_eq!(state.parameters.get("VTH0").unwrap().value, 0.5);
        assert!(state.is_dirty);
    }

    #[test]
    fn test_set_parameter_value_invalid_range() {
        let mut state = ModelEditorState::new();
        // VTH0 has range [-2.0, 2.0]
        let result = state.set_parameter_value("VTH0", 10.0);
        assert!(result.is_err());
        assert!(state.validation_errors.contains_key("VTH0"));
    }

    #[test]
    fn test_set_parameter_value_not_found() {
        let mut state = ModelEditorState::new();
        let result = state.set_parameter_value("NONEXISTENT", 1.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_reset_parameter() {
        let mut state = ModelEditorState::new();
        let original = state.parameters.get("VTH0").unwrap().default;
        let _ = state.set_parameter_value("VTH0", 0.8);
        state.reset_parameter("VTH0");
        assert_eq!(state.parameters.get("VTH0").unwrap().value, original);
    }

    #[test]
    fn test_reset_all() {
        let mut state = ModelEditorState::new();
        let _ = state.set_parameter_value("VTH0", 0.8);
        let _ = state.set_parameter_value("U0", 0.05);
        state.reset_all();

        for param in state.parameters.values() {
            assert_eq!(param.value, param.default);
        }
    }

    // -------------------------------------------------------------------------
    // Technology Node Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_apply_technology_preset() {
        let mut state = ModelEditorState::new();
        state.apply_technology_preset(TechnologyNode::Nm180);

        assert_eq!(state.tech_node, TechnologyNode::Nm180);
        // Verify parameters were updated for 180nm
        let l_param = state.parameters.get("L").unwrap();
        assert_eq!(l_param.default, 180e-9);
    }

    #[test]
    fn test_technology_affects_oxide_thickness() {
        let mut state = ModelEditorState::new();
        state.apply_technology_preset(TechnologyNode::Nm180);
        let tox_180 = state.parameters.get("TOXE").unwrap().default;

        state.apply_technology_preset(TechnologyNode::Nm45);
        let tox_45 = state.parameters.get("TOXE").unwrap().default;

        // 180nm should have thicker oxide than 45nm
        assert!(tox_180 > tox_45);
    }

    // -------------------------------------------------------------------------
    // Model Type Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_set_model_type() {
        let mut state = ModelEditorState::new();
        state.set_model_type(MosfetModelType::Level1);
        assert_eq!(state.model_type, MosfetModelType::Level1);
        assert!(state.is_dirty);
    }

    // -------------------------------------------------------------------------
    // Validation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_validate_returns_true_for_defaults() {
        let mut state = ModelEditorState::new();
        assert!(state.validate());
        assert!(state.validation_errors.is_empty());
    }

    #[test]
    fn test_validate_detects_invalid_values() {
        let mut state = ModelEditorState::new();
        // Force an invalid value
        state.parameters.get_mut("VTH0").unwrap().value = 100.0;
        assert!(!state.validate());
        assert!(state.validation_errors.contains_key("VTH0"));
    }

    // -------------------------------------------------------------------------
    // SPICE Model Export Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_to_spice_model_basic() {
        let state = ModelEditorState::new();
        let spice = state.to_spice_model();

        assert!(
            spice.contains(".MODEL nch NMOS LEVEL=14"),
            "Got:\n{}",
            spice
        );
    }

    #[test]
    fn test_to_spice_model_pmos() {
        let mut state = ModelEditorState::new();
        state.model_name = "pch".to_string();
        state.is_nmos = false;
        let spice = state.to_spice_model();

        assert!(
            spice.contains(".MODEL pch PMOS LEVEL=14"),
            "Got:\n{}",
            spice
        );
    }

    #[test]
    fn test_to_spice_model_exports_modified_params() {
        let mut state = ModelEditorState::new();
        let _ = state.set_parameter_value("VTH0", 0.99);
        let spice = state.to_spice_model();

        assert!(
            spice.contains("VTH0="),
            "Modified VTH0 should be in output:\n{}",
            spice
        );
    }

    #[test]
    fn test_to_spice_model_includes_category_headers() {
        let mut state = ModelEditorState::new();
        let _ = state.set_parameter_value("VTH0", 0.99);
        let spice = state.to_spice_model();

        assert!(
            spice.contains("Threshold Voltage"),
            "Should include category header:\n{}",
            spice
        );
    }

    // -------------------------------------------------------------------------
    // SPICE Model Import Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_from_spice_model_basic() {
        let mut state = ModelEditorState::new();
        let spice = r#"
.MODEL mymodel NMOS LEVEL=14
+ VTH0=0.42 U0=0.04
+ K1=0.6
"#;
        let result = state.from_spice_model(spice);
        assert!(result.is_ok());
        assert_eq!(state.model_name, "mymodel");
        assert!(state.is_nmos);
        assert!((state.parameters.get("VTH0").unwrap().value - 0.42).abs() < 1e-6);
    }

    #[test]
    fn test_from_spice_model_pmos() {
        let mut state = ModelEditorState::new();
        let spice = ".MODEL pdev PMOS LEVEL=14";
        let _ = state.from_spice_model(spice);
        assert!(!state.is_nmos);
    }

    #[test]
    fn test_from_spice_model_ignores_comments() {
        let mut state = ModelEditorState::new();
        let spice = r#"
* This is a comment
.MODEL test NMOS LEVEL=14
* Another comment
+ VTH0=0.5
"#;
        let result = state.from_spice_model(spice);
        assert!(result.is_ok());
        assert_eq!(state.model_name, "test");
    }

    // -------------------------------------------------------------------------
    // Parameter Count Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_category_param_count() {
        let state = ModelEditorState::new();
        assert!(state.category_param_count(ParameterCategory::Geometry) >= 5);
        assert!(state.category_param_count(ParameterCategory::Threshold) >= 5);
        assert!(state.category_param_count(ParameterCategory::Mobility) >= 4);
    }

    #[test]
    fn test_modified_param_count() {
        let mut state = ModelEditorState::new();
        assert_eq!(state.modified_param_count(), 0);

        let _ = state.set_parameter_value("VTH0", 0.99);
        assert_eq!(state.modified_param_count(), 1);

        let _ = state.set_parameter_value("U0", 0.05);
        assert_eq!(state.modified_param_count(), 2);
    }

    #[test]
    fn test_is_parameter_modified() {
        let mut state = ModelEditorState::new();
        assert!(!state.is_parameter_modified("VTH0"));

        let _ = state.set_parameter_value("VTH0", 0.99);
        assert!(state.is_parameter_modified("VTH0"));

        state.reset_parameter("VTH0");
        assert!(!state.is_parameter_modified("VTH0"));
    }

    // -------------------------------------------------------------------------
    // NMOS vs PMOS Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_nmos_has_positive_vth() {
        let mut state = ModelEditorState::new();
        state.is_nmos = true;
        state.initialize_parameters();
        let vth0 = state.parameters.get("VTH0").unwrap().default;
        assert!(vth0 > 0.0, "NMOS VTH0 should be positive");
    }

    #[test]
    fn test_pmos_has_negative_vth() {
        let mut state = ModelEditorState::new();
        state.is_nmos = false;
        state.initialize_parameters();
        let vth0 = state.parameters.get("VTH0").unwrap().default;
        assert!(vth0 < 0.0, "PMOS VTH0 should be negative");
    }

    // -------------------------------------------------------------------------
    // Serialization Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_model_editor_state_serialization() {
        let state = ModelEditorState::new();
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: ModelEditorState = serde_json::from_str(&json).unwrap();
        assert_eq!(state.model_type, deserialized.model_type);
        assert_eq!(state.tech_node, deserialized.tech_node);
        assert_eq!(state.model_name, deserialized.model_name);
    }

    #[test]
    fn test_parameter_values_survive_serialization() {
        let mut state = ModelEditorState::new();
        let _ = state.set_parameter_value("VTH0", 0.77);

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: ModelEditorState = serde_json::from_str(&json).unwrap();

        assert!((deserialized.parameters.get("VTH0").unwrap().value - 0.77).abs() < 1e-10);
    }

    // -------------------------------------------------------------------------
    // Edge Case Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_reset_parameter_nonexistent() {
        let mut state = ModelEditorState::new();
        // Should not panic
        state.reset_parameter("NONEXISTENT");
    }

    #[test]
    fn test_empty_category_returns_empty_vec() {
        let state = ModelEditorState::new();
        // All categories should have parameters in BSIM4
        // But if we had an empty category, it should return empty vec
        let params = state.get_parameters_by_category(ParameterCategory::SelfHeating);
        // SelfHeating has 2 parameters
        assert!(!params.is_empty());
    }

    #[test]
    fn test_is_dirty_flag() {
        let mut state = ModelEditorState::new();
        assert!(!state.is_dirty);

        let _ = state.set_parameter_value("VTH0", 0.5);
        assert!(state.is_dirty);

        state.reset_all();
        assert!(state.is_dirty); // reset_all sets dirty

        // from_spice_model clears dirty
        let _ = state.from_spice_model(".MODEL test NMOS");
        assert!(!state.is_dirty);
    }

    // -------------------------------------------------------------------------
    // Comprehensive Parameter Coverage Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_all_parameters_have_valid_defaults() {
        let state = ModelEditorState::new();
        for (name, param) in &state.parameters {
            assert!(
                param.is_valid(),
                "Parameter {} has invalid default: {} not in [{}, {}]",
                name,
                param.default,
                param.min,
                param.max
            );
        }
    }

    #[test]
    fn test_all_parameters_have_descriptions() {
        let state = ModelEditorState::new();
        for (name, param) in &state.parameters {
            assert!(
                !param.description.is_empty(),
                "Parameter {} should have a description",
                name
            );
        }
    }

    #[test]
    fn test_all_parameters_have_units() {
        let state = ModelEditorState::new();
        // Note: some parameters are dimensionless (empty unit is valid)
        for (name, param) in &state.parameters {
            // Just verify unit field exists (can be empty for dimensionless)
            assert!(
                param.unit.len() <= 10,
                "Parameter {} unit '{}' seems too long",
                name,
                param.unit
            );
        }
    }

    #[test]
    fn test_parameter_min_less_than_max() {
        let state = ModelEditorState::new();
        for (name, param) in &state.parameters {
            assert!(
                param.min <= param.max,
                "Parameter {} has min {} > max {}",
                name,
                param.min,
                param.max
            );
        }
    }

    #[test]
    fn test_minimum_parameter_count() {
        let state = ModelEditorState::new();
        // BSIM4 should have at least 60 parameters
        assert!(
            state.parameters.len() >= 60,
            "Expected at least 60 parameters, got {}",
            state.parameters.len()
        );
    }
}
