//! Device Model Types
//!
//! This module defines the UI-facing model types for the advanced
//! device model editor. Supports MOSFET, BJT, and Diode models with
//! structured interfaces for selecting model types, organizing
//! parameters by category, and applying technology presets.

use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================
// Device Model Type (Top-Level)
// =============================================================================

/// Top-level device type selection
///
/// Determines which device category (MOSFET, BJT, Diode) is being edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DeviceModelType {
    /// MOSFET (Level 1, BSIM3, BSIM4, EKV)
    #[default]
    Mosfet,

    /// BJT (Gummel-Poon, VBIC)
    Bjt,

    /// Diode (Standard, Schottky)
    Diode,
}

impl DeviceModelType {
    /// Get all device types
    pub fn all() -> &'static [DeviceModelType] {
        &[
            DeviceModelType::Mosfet,
            DeviceModelType::Bjt,
            DeviceModelType::Diode,
        ]
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            DeviceModelType::Mosfet => "MOSFET",
            DeviceModelType::Bjt => "BJT",
            DeviceModelType::Diode => "Diode",
        }
    }

    /// Get SPICE device prefix
    pub fn spice_prefix(&self) -> &'static str {
        match self {
            DeviceModelType::Mosfet => "M",
            DeviceModelType::Bjt => "Q",
            DeviceModelType::Diode => "D",
        }
    }
}

impl fmt::Display for DeviceModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// =============================================================================
// BJT Model Type
// =============================================================================

/// BJT model type selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BjtModelType {
    /// Ebers-Moll model (basic)
    EbersMoll,

    /// Gummel-Poon model (standard)
    #[default]
    GummelPoon,

    /// VBIC model (advanced)
    Vbic,

    /// HICUM model (high-frequency)
    Hicum,
}

impl BjtModelType {
    /// Get all BJT model types
    pub fn all() -> &'static [BjtModelType] {
        &[
            BjtModelType::EbersMoll,
            BjtModelType::GummelPoon,
            BjtModelType::Vbic,
            BjtModelType::Hicum,
        ]
    }

    /// Get the SPICE LEVEL parameter value
    pub fn spice_level(&self) -> u32 {
        match self {
            BjtModelType::EbersMoll => 1,
            BjtModelType::GummelPoon => 1, // GP is level 1 with more params
            BjtModelType::Vbic => 4,
            BjtModelType::Hicum => 8,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            BjtModelType::EbersMoll => "Ebers-Moll",
            BjtModelType::GummelPoon => "Gummel-Poon",
            BjtModelType::Vbic => "VBIC",
            BjtModelType::Hicum => "HICUM L2",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            BjtModelType::EbersMoll => "Basic large-signal model (~10 params)",
            BjtModelType::GummelPoon => "Industry standard (~40 params)",
            BjtModelType::Vbic => "Vertical Bipolar IC model (~100 params)",
            BjtModelType::Hicum => "High-Current Model for RF (~200 params)",
        }
    }
}

impl fmt::Display for BjtModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// =============================================================================
// Diode Model Type
// =============================================================================

/// Diode model type selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DiodeModelType {
    /// Standard PN junction diode
    #[default]
    Standard,

    /// Schottky barrier diode
    Schottky,

    /// Zener/breakdown diode
    Zener,

    /// Varactor (variable capacitance)
    Varactor,
}

impl DiodeModelType {
    /// Get all diode model types
    pub fn all() -> &'static [DiodeModelType] {
        &[
            DiodeModelType::Standard,
            DiodeModelType::Schottky,
            DiodeModelType::Zener,
            DiodeModelType::Varactor,
        ]
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            DiodeModelType::Standard => "PN Junction",
            DiodeModelType::Schottky => "Schottky",
            DiodeModelType::Zener => "Zener",
            DiodeModelType::Varactor => "Varactor",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            DiodeModelType::Standard => "Standard PN junction (~20 params)",
            DiodeModelType::Schottky => "Metal-semiconductor junction",
            DiodeModelType::Zener => "Reverse breakdown operation",
            DiodeModelType::Varactor => "Variable capacitance diode",
        }
    }
}

impl fmt::Display for DiodeModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// =============================================================================
// MOSFET Model Type
// =============================================================================

/// MOSFET model type selection
///
/// Represents the available MOSFET compact models supported by the simulator.
/// Each model offers different levels of accuracy and complexity tradeoffs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MosfetModelType {
    /// Level 1 Shichman-Hodges model
    /// - Simple quadratic I-V model
    /// - Good for hand calculations and learning
    /// - ~10 parameters
    Level1,

    /// Level 3 BSIM3-like model
    /// - Short-channel effects
    /// - Better accuracy for submicron
    /// - ~50 parameters
    Level3,

    /// BSIM4v4.8 model (default for IC design)
    /// - Industry standard for 180nm and below
    /// - Full short-channel, quantum, noise effects
    /// - ~300 parameters
    #[default]
    Bsim4,

    /// EKV 2.6 model
    /// - Physics-based, continuous model
    /// - Good for low-power analog/RF
    /// - ~40 parameters
    Ekv,
}

impl MosfetModelType {
    /// Get all available model types
    pub fn all() -> &'static [MosfetModelType] {
        &[
            MosfetModelType::Level1,
            MosfetModelType::Level3,
            MosfetModelType::Bsim4,
            MosfetModelType::Ekv,
        ]
    }

    /// Get the SPICE LEVEL parameter value
    pub fn spice_level(&self) -> u32 {
        match self {
            MosfetModelType::Level1 => 1,
            MosfetModelType::Level3 => 3,
            MosfetModelType::Bsim4 => 14, // BSIM4 uses LEVEL=14
            MosfetModelType::Ekv => 55,   // EKV uses LEVEL=55
        }
    }

    /// Get the display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            MosfetModelType::Level1 => "Level 1 (Shichman-Hodges)",
            MosfetModelType::Level3 => "Level 3 (BSIM3-like)",
            MosfetModelType::Bsim4 => "BSIM4v4.8",
            MosfetModelType::Ekv => "EKV 2.6",
        }
    }

    /// Get a short description of the model
    pub fn description(&self) -> &'static str {
        match self {
            MosfetModelType::Level1 => "Simple quadratic model for hand calculations",
            MosfetModelType::Level3 => "Short-channel effects, submicron accuracy",
            MosfetModelType::Bsim4 => "Industry standard for deep-submicron IC design",
            MosfetModelType::Ekv => "Physics-based model for low-power analog/RF",
        }
    }
}

impl fmt::Display for MosfetModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// =============================================================================
// Parameter Category
// =============================================================================

/// Parameter category for tabbed display
///
/// Organizes MOSFET model parameters into logical groups for
/// easier navigation in the model editor UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ParameterCategory {
    /// Device geometry (L, W, NF, TOX)
    #[default]
    Geometry,

    /// Threshold voltage parameters (VTH0, K1, K2, DIBL)
    Threshold,

    /// Mobility degradation (U0, UA, UB, UC)
    Mobility,

    /// Velocity saturation (VSAT, A0, AGS)
    VelocitySaturation,

    /// Channel length modulation (PCLM, PDIBLC)
    ChannelLengthModulation,

    /// Subthreshold behavior (VOFF, NFACTOR)
    Subthreshold,

    /// Source/drain parasitic resistance (RSH, RDSW)
    Resistance,

    /// Parasitic capacitances (CGSO, CGDO, CJ)
    Capacitance,

    /// Temperature coefficients (TNOM, KT1, UTE)
    Temperature,

    /// Noise model parameters (KF, AF, NOIA)
    Noise,

    /// Self-heating model (RTH0, CTH0)
    SelfHeating,
}

impl ParameterCategory {
    /// Get all parameter categories
    pub fn all() -> &'static [ParameterCategory] {
        &[
            ParameterCategory::Geometry,
            ParameterCategory::Threshold,
            ParameterCategory::Mobility,
            ParameterCategory::VelocitySaturation,
            ParameterCategory::ChannelLengthModulation,
            ParameterCategory::Subthreshold,
            ParameterCategory::Resistance,
            ParameterCategory::Capacitance,
            ParameterCategory::Temperature,
            ParameterCategory::Noise,
            ParameterCategory::SelfHeating,
        ]
    }

    /// Get the display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            ParameterCategory::Geometry => "Geometry",
            ParameterCategory::Threshold => "Threshold Voltage",
            ParameterCategory::Mobility => "Mobility",
            ParameterCategory::VelocitySaturation => "Velocity Saturation",
            ParameterCategory::ChannelLengthModulation => "CLM / Output",
            ParameterCategory::Subthreshold => "Subthreshold",
            ParameterCategory::Resistance => "S/D Resistance",
            ParameterCategory::Capacitance => "Capacitance",
            ParameterCategory::Temperature => "Temperature",
            ParameterCategory::Noise => "Noise",
            ParameterCategory::SelfHeating => "Self-Heating",
        }
    }

    /// Get the icon/shorthand for sidebar
    pub fn icon(&self) -> &'static str {
        match self {
            ParameterCategory::Geometry => "📐",
            ParameterCategory::Threshold => "⚡",
            ParameterCategory::Mobility => "🔄",
            ParameterCategory::VelocitySaturation => "🚀",
            ParameterCategory::ChannelLengthModulation => "📈",
            ParameterCategory::Subthreshold => "📉",
            ParameterCategory::Resistance => "Ω",
            ParameterCategory::Capacitance => "⚡",
            ParameterCategory::Temperature => "🌡",
            ParameterCategory::Noise => "📊",
            ParameterCategory::SelfHeating => "🔥",
        }
    }
}

impl fmt::Display for ParameterCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// =============================================================================
// Technology Node
// =============================================================================

/// Technology node presets
///
/// Provides pre-configured parameter sets for common technology nodes.
/// These are typical foundry values and can be used as starting points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TechnologyNode {
    /// 180nm node (mature process)
    Nm180,

    /// 130nm node
    Nm130,

    /// 90nm node
    Nm90,

    /// 65nm node (default for BSIM4)
    #[default]
    Nm65,

    /// 45nm node
    Nm45,

    /// 28nm node
    Nm28,

    /// 14nm FinFET
    Nm14,

    /// Custom (user-defined parameters)
    Custom,
}

impl TechnologyNode {
    /// Get all technology nodes
    pub fn all() -> &'static [TechnologyNode] {
        &[
            TechnologyNode::Nm180,
            TechnologyNode::Nm130,
            TechnologyNode::Nm90,
            TechnologyNode::Nm65,
            TechnologyNode::Nm45,
            TechnologyNode::Nm28,
            TechnologyNode::Nm14,
            TechnologyNode::Custom,
        ]
    }

    /// Get the display name
    pub fn display_name(&self) -> &'static str {
        match self {
            TechnologyNode::Nm180 => "180nm",
            TechnologyNode::Nm130 => "130nm",
            TechnologyNode::Nm90 => "90nm",
            TechnologyNode::Nm65 => "65nm",
            TechnologyNode::Nm45 => "45nm",
            TechnologyNode::Nm28 => "28nm",
            TechnologyNode::Nm14 => "14nm FinFET",
            TechnologyNode::Custom => "Custom",
        }
    }

    /// Get minimum channel length (m)
    pub fn min_length(&self) -> f64 {
        match self {
            TechnologyNode::Nm180 => 180e-9,
            TechnologyNode::Nm130 => 130e-9,
            TechnologyNode::Nm90 => 90e-9,
            TechnologyNode::Nm65 => 65e-9,
            TechnologyNode::Nm45 => 45e-9,
            TechnologyNode::Nm28 => 28e-9,
            TechnologyNode::Nm14 => 14e-9,
            TechnologyNode::Custom => 65e-9,
        }
    }

    /// Get typical oxide thickness (m)
    pub fn oxide_thickness(&self) -> f64 {
        match self {
            TechnologyNode::Nm180 => 4.0e-9,
            TechnologyNode::Nm130 => 2.5e-9,
            TechnologyNode::Nm90 => 2.0e-9,
            TechnologyNode::Nm65 => 1.4e-9,
            TechnologyNode::Nm45 => 1.2e-9,
            TechnologyNode::Nm28 => 1.0e-9,
            TechnologyNode::Nm14 => 0.9e-9,
            TechnologyNode::Custom => 1.4e-9,
        }
    }

    /// Get typical threshold voltage for NMOS (V)
    pub fn nmos_vth(&self) -> f64 {
        match self {
            TechnologyNode::Nm180 => 0.5,
            TechnologyNode::Nm130 => 0.45,
            TechnologyNode::Nm90 => 0.42,
            TechnologyNode::Nm65 => 0.4,
            TechnologyNode::Nm45 => 0.35,
            TechnologyNode::Nm28 => 0.3,
            TechnologyNode::Nm14 => 0.25,
            TechnologyNode::Custom => 0.4,
        }
    }

    /// Get typical threshold voltage for PMOS (V)
    pub fn pmos_vth(&self) -> f64 {
        -self.nmos_vth() // PMOS has negative Vth
    }

    /// Get typical low-field mobility for NMOS (cm²/V·s)
    pub fn nmos_mobility(&self) -> f64 {
        match self {
            TechnologyNode::Nm180 => 450.0,
            TechnologyNode::Nm130 => 420.0,
            TechnologyNode::Nm90 => 400.0,
            TechnologyNode::Nm65 => 380.0,
            TechnologyNode::Nm45 => 350.0,
            TechnologyNode::Nm28 => 320.0,
            TechnologyNode::Nm14 => 280.0,
            TechnologyNode::Custom => 400.0,
        }
    }

    /// Get typical supply voltage (V)
    pub fn supply_voltage(&self) -> f64 {
        match self {
            TechnologyNode::Nm180 => 1.8,
            TechnologyNode::Nm130 => 1.2,
            TechnologyNode::Nm90 => 1.0,
            TechnologyNode::Nm65 => 1.0,
            TechnologyNode::Nm45 => 1.0,
            TechnologyNode::Nm28 => 0.9,
            TechnologyNode::Nm14 => 0.8,
            TechnologyNode::Custom => 1.0,
        }
    }
}

impl fmt::Display for TechnologyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// =============================================================================
// Model Parameter Definition
// =============================================================================

/// A single model parameter with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParameter {
    /// Parameter name (e.g., "VTH0")
    pub name: String,

    /// Current value
    pub value: f64,

    /// Default value
    pub default: f64,

    /// Minimum valid value
    pub min: f64,

    /// Maximum valid value
    pub max: f64,

    /// Unit string for display (e.g., "V", "cm²/V·s")
    pub unit: String,

    /// Short description
    pub description: String,

    /// Parameter category
    pub category: ParameterCategory,
}

impl ModelParameter {
    /// Create a new model parameter
    pub fn new(
        name: &str,
        default: f64,
        min: f64,
        max: f64,
        unit: &str,
        description: &str,
        category: ParameterCategory,
    ) -> Self {
        Self {
            name: name.to_string(),
            value: default,
            default,
            min,
            max,
            unit: unit.to_string(),
            description: description.to_string(),
            category,
        }
    }

    /// Check if value is within valid range
    pub fn is_valid(&self) -> bool {
        self.value >= self.min && self.value <= self.max
    }

    /// Reset to default value
    pub fn reset(&mut self) {
        self.value = self.default;
    }

    /// Get formatted value with unit
    pub fn formatted_value(&self) -> String {
        format_engineering(self.value, &self.unit)
    }
}

/// Format a value in engineering notation with unit
fn format_engineering(value: f64, unit: &str) -> String {
    let abs_val = value.abs();
    let (scaled, prefix) = if abs_val >= 1e12 {
        (value / 1e12, "T")
    } else if abs_val >= 1e9 {
        (value / 1e9, "G")
    } else if abs_val >= 1e6 {
        (value / 1e6, "M")
    } else if abs_val >= 1e3 {
        (value / 1e3, "k")
    } else if abs_val >= 1.0 || abs_val == 0.0 {
        (value, "")
    } else if abs_val >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_val >= 1e-6 {
        (value * 1e6, "µ")
    } else if abs_val >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_val >= 1e-12 {
        (value * 1e12, "p")
    } else {
        (value * 1e15, "f")
    };

    // Format with appropriate precision
    let formatted_value = if scaled.abs() >= 100.0 {
        format!("{:.1}", scaled)
    } else if scaled.abs() >= 10.0 {
        format!("{:.2}", scaled)
    } else if scaled.abs() >= 1.0 {
        format!("{:.3}", scaled)
    } else {
        format!("{:.4}", scaled)
    };

    if prefix.is_empty() {
        format!("{}{}", formatted_value, unit)
    } else {
        format!("{}{}{}", formatted_value, prefix, unit)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // MosfetModelType Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_mosfet_model_type_default() {
        let model = MosfetModelType::default();
        assert_eq!(model, MosfetModelType::Bsim4);
    }

    #[test]
    fn test_mosfet_model_type_all() {
        let all = MosfetModelType::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&MosfetModelType::Level1));
        assert!(all.contains(&MosfetModelType::Level3));
        assert!(all.contains(&MosfetModelType::Bsim4));
        assert!(all.contains(&MosfetModelType::Ekv));
    }

    #[test]
    fn test_mosfet_model_type_spice_level() {
        assert_eq!(MosfetModelType::Level1.spice_level(), 1);
        assert_eq!(MosfetModelType::Level3.spice_level(), 3);
        assert_eq!(MosfetModelType::Bsim4.spice_level(), 14);
        assert_eq!(MosfetModelType::Ekv.spice_level(), 55);
    }

    #[test]
    fn test_mosfet_model_type_display_name() {
        assert!(!MosfetModelType::Level1.display_name().is_empty());
        assert!(!MosfetModelType::Bsim4.display_name().is_empty());
        assert!(MosfetModelType::Bsim4.display_name().contains("BSIM4"));
    }

    #[test]
    fn test_mosfet_model_type_description() {
        for model in MosfetModelType::all() {
            assert!(!model.description().is_empty());
        }
    }

    #[test]
    fn test_mosfet_model_type_display_trait() {
        let s = format!("{}", MosfetModelType::Bsim4);
        assert!(s.contains("BSIM4"));
    }

    // -------------------------------------------------------------------------
    // ParameterCategory Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_parameter_category_default() {
        let cat = ParameterCategory::default();
        assert_eq!(cat, ParameterCategory::Geometry);
    }

    #[test]
    fn test_parameter_category_all() {
        let all = ParameterCategory::all();
        assert_eq!(all.len(), 11);
        assert!(all.contains(&ParameterCategory::Geometry));
        assert!(all.contains(&ParameterCategory::Threshold));
        assert!(all.contains(&ParameterCategory::Noise));
        assert!(all.contains(&ParameterCategory::SelfHeating));
    }

    #[test]
    fn test_parameter_category_display_name() {
        for cat in ParameterCategory::all() {
            assert!(!cat.display_name().is_empty());
        }
    }

    #[test]
    fn test_parameter_category_icon() {
        for cat in ParameterCategory::all() {
            assert!(!cat.icon().is_empty());
        }
    }

    #[test]
    fn test_parameter_category_display_trait() {
        let s = format!("{}", ParameterCategory::Threshold);
        assert_eq!(s, "Threshold Voltage");
    }

    // -------------------------------------------------------------------------
    // TechnologyNode Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_technology_node_default() {
        let node = TechnologyNode::default();
        assert_eq!(node, TechnologyNode::Nm65);
    }

    #[test]
    fn test_technology_node_all() {
        let all = TechnologyNode::all();
        assert_eq!(all.len(), 8);
        assert!(all.contains(&TechnologyNode::Nm180));
        assert!(all.contains(&TechnologyNode::Nm65));
        assert!(all.contains(&TechnologyNode::Custom));
    }

    #[test]
    fn test_technology_node_min_length() {
        // Verify scaling: smaller node = smaller minimum length
        assert!(TechnologyNode::Nm180.min_length() > TechnologyNode::Nm130.min_length());
        assert!(TechnologyNode::Nm130.min_length() > TechnologyNode::Nm90.min_length());
        assert!(TechnologyNode::Nm90.min_length() > TechnologyNode::Nm65.min_length());
        assert!(TechnologyNode::Nm65.min_length() > TechnologyNode::Nm45.min_length());
    }

    #[test]
    fn test_technology_node_oxide_thickness() {
        // Verify scaling: smaller node = thinner oxide
        assert!(TechnologyNode::Nm180.oxide_thickness() > TechnologyNode::Nm90.oxide_thickness());
        assert!(TechnologyNode::Nm90.oxide_thickness() > TechnologyNode::Nm45.oxide_thickness());
    }

    #[test]
    fn test_technology_node_vth() {
        // NMOS Vth is positive, PMOS is negative
        for node in TechnologyNode::all() {
            assert!(
                node.nmos_vth() > 0.0,
                "{:?} NMOS Vth should be positive",
                node
            );
            assert!(
                node.pmos_vth() < 0.0,
                "{:?} PMOS Vth should be negative",
                node
            );
        }
    }

    #[test]
    fn test_technology_node_mobility() {
        // Mobility should be positive and reasonable (100-1000 cm²/V·s)
        for node in TechnologyNode::all() {
            let mu = node.nmos_mobility();
            assert!(
                mu > 100.0 && mu < 1000.0,
                "{:?} mobility {} out of range",
                node,
                mu
            );
        }
    }

    #[test]
    fn test_technology_node_supply_voltage() {
        // Supply voltage scaling with technology
        assert!(TechnologyNode::Nm180.supply_voltage() > TechnologyNode::Nm65.supply_voltage());
        assert!(TechnologyNode::Nm65.supply_voltage() >= TechnologyNode::Nm28.supply_voltage());
    }

    #[test]
    fn test_technology_node_display_trait() {
        let s = format!("{}", TechnologyNode::Nm65);
        assert_eq!(s, "65nm");
    }

    // -------------------------------------------------------------------------
    // ModelParameter Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_model_parameter_new() {
        let param = ModelParameter::new(
            "VTH0",
            0.4,
            0.0,
            2.0,
            "V",
            "Threshold voltage",
            ParameterCategory::Threshold,
        );
        assert_eq!(param.name, "VTH0");
        assert_eq!(param.value, 0.4);
        assert_eq!(param.default, 0.4);
        assert_eq!(param.min, 0.0);
        assert_eq!(param.max, 2.0);
        assert_eq!(param.unit, "V");
        assert_eq!(param.category, ParameterCategory::Threshold);
    }

    #[test]
    fn test_model_parameter_is_valid() {
        let mut param = ModelParameter::new(
            "VTH0",
            0.4,
            0.0,
            2.0,
            "V",
            "Threshold voltage",
            ParameterCategory::Threshold,
        );
        assert!(param.is_valid());

        param.value = -1.0;
        assert!(!param.is_valid());

        param.value = 3.0;
        assert!(!param.is_valid());

        param.value = 1.5;
        assert!(param.is_valid());
    }

    #[test]
    fn test_model_parameter_reset() {
        let mut param = ModelParameter::new(
            "VTH0",
            0.4,
            0.0,
            2.0,
            "V",
            "Threshold voltage",
            ParameterCategory::Threshold,
        );
        param.value = 1.0;
        assert_ne!(param.value, param.default);

        param.reset();
        assert_eq!(param.value, param.default);
    }

    #[test]
    fn test_model_parameter_formatted_value() {
        let param = ModelParameter::new(
            "TOX",
            1.4e-9,
            0.5e-9,
            10e-9,
            "m",
            "Oxide thickness",
            ParameterCategory::Geometry,
        );
        let formatted = param.formatted_value();
        assert!(
            formatted.contains("n") || formatted.contains("1.4"),
            "Got: {}",
            formatted
        );
    }

    // -------------------------------------------------------------------------
    // format_engineering Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_engineering_tera() {
        let s = format_engineering(1.5e12, "Hz");
        assert!(s.contains("T") && s.contains("Hz"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_giga() {
        let s = format_engineering(2.4e9, "Hz");
        assert!(s.contains("G") && s.contains("Hz"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_mega() {
        let s = format_engineering(1.0e6, "Ω");
        assert!(s.contains("M") && s.contains("Ω"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_kilo() {
        let s = format_engineering(4.7e3, "Ω");
        assert!(s.contains("k") && s.contains("Ω"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_unit() {
        let s = format_engineering(1.5, "V");
        assert!(s.contains("V") && s.contains("1.5"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_milli() {
        let s = format_engineering(1e-3, "A");
        assert!(s.contains("m") && s.contains("A"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_micro() {
        let s = format_engineering(1e-6, "F");
        assert!(s.contains("µ") && s.contains("F"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_nano() {
        let s = format_engineering(65e-9, "m");
        assert!(s.contains("n") && s.contains("m"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_pico() {
        let s = format_engineering(1e-12, "F");
        assert!(s.contains("p") && s.contains("F"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_femto() {
        let s = format_engineering(1e-15, "F");
        assert!(s.contains("f") && s.contains("F"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_zero() {
        let s = format_engineering(0.0, "V");
        assert!(s.contains("0") && s.contains("V"), "Got: {}", s);
    }

    #[test]
    fn test_format_engineering_negative() {
        let s = format_engineering(-1.5e-3, "A");
        assert!(s.contains("-") && s.contains("m"), "Got: {}", s);
    }

    // -------------------------------------------------------------------------
    // Serialization Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_mosfet_model_type_serialization() {
        let model = MosfetModelType::Bsim4;
        let json = serde_json::to_string(&model).unwrap();
        let deserialized: MosfetModelType = serde_json::from_str(&json).unwrap();
        assert_eq!(model, deserialized);
    }

    #[test]
    fn test_parameter_category_serialization() {
        let cat = ParameterCategory::Noise;
        let json = serde_json::to_string(&cat).unwrap();
        let deserialized: ParameterCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(cat, deserialized);
    }

    #[test]
    fn test_technology_node_serialization() {
        let node = TechnologyNode::Nm45;
        let json = serde_json::to_string(&node).unwrap();
        let deserialized: TechnologyNode = serde_json::from_str(&json).unwrap();
        assert_eq!(node, deserialized);
    }

    #[test]
    fn test_model_parameter_serialization() {
        let param = ModelParameter::new(
            "VTH0",
            0.4,
            0.0,
            2.0,
            "V",
            "Threshold voltage",
            ParameterCategory::Threshold,
        );
        let json = serde_json::to_string(&param).unwrap();
        let deserialized: ModelParameter = serde_json::from_str(&json).unwrap();
        assert_eq!(param.name, deserialized.name);
        assert_eq!(param.value, deserialized.value);
        assert_eq!(param.category, deserialized.category);
    }

    // -------------------------------------------------------------------------
    // DeviceModelType Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_device_model_type_default() {
        let device = DeviceModelType::default();
        assert_eq!(device, DeviceModelType::Mosfet);
    }

    #[test]
    fn test_device_model_type_all() {
        let all = DeviceModelType::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&DeviceModelType::Mosfet));
        assert!(all.contains(&DeviceModelType::Bjt));
        assert!(all.contains(&DeviceModelType::Diode));
    }

    #[test]
    fn test_device_model_type_display_name() {
        assert_eq!(DeviceModelType::Mosfet.display_name(), "MOSFET");
        assert_eq!(DeviceModelType::Bjt.display_name(), "BJT");
        assert_eq!(DeviceModelType::Diode.display_name(), "Diode");
    }

    #[test]
    fn test_device_model_type_spice_prefix() {
        assert_eq!(DeviceModelType::Mosfet.spice_prefix(), "M");
        assert_eq!(DeviceModelType::Bjt.spice_prefix(), "Q");
        assert_eq!(DeviceModelType::Diode.spice_prefix(), "D");
    }

    #[test]
    fn test_device_model_type_display_trait() {
        let s = format!("{}", DeviceModelType::Bjt);
        assert_eq!(s, "BJT");
    }

    #[test]
    fn test_device_model_type_serialization() {
        for device in DeviceModelType::all() {
            let json = serde_json::to_string(device).unwrap();
            let deserialized: DeviceModelType = serde_json::from_str(&json).unwrap();
            assert_eq!(*device, deserialized);
        }
    }

    // -------------------------------------------------------------------------
    // BjtModelType Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_bjt_model_type_default() {
        let model = BjtModelType::default();
        assert_eq!(model, BjtModelType::GummelPoon);
    }

    #[test]
    fn test_bjt_model_type_all() {
        let all = BjtModelType::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&BjtModelType::EbersMoll));
        assert!(all.contains(&BjtModelType::GummelPoon));
        assert!(all.contains(&BjtModelType::Vbic));
        assert!(all.contains(&BjtModelType::Hicum));
    }

    #[test]
    fn test_bjt_model_type_spice_level() {
        assert_eq!(BjtModelType::EbersMoll.spice_level(), 1);
        assert_eq!(BjtModelType::GummelPoon.spice_level(), 1);
        assert_eq!(BjtModelType::Vbic.spice_level(), 4);
        assert_eq!(BjtModelType::Hicum.spice_level(), 8);
    }

    #[test]
    fn test_bjt_model_type_display_name() {
        assert_eq!(BjtModelType::EbersMoll.display_name(), "Ebers-Moll");
        assert_eq!(BjtModelType::GummelPoon.display_name(), "Gummel-Poon");
        assert_eq!(BjtModelType::Vbic.display_name(), "VBIC");
        assert_eq!(BjtModelType::Hicum.display_name(), "HICUM L2");
    }

    #[test]
    fn test_bjt_model_type_description() {
        for model in BjtModelType::all() {
            assert!(!model.description().is_empty());
        }
    }

    #[test]
    fn test_bjt_model_type_display_trait() {
        let s = format!("{}", BjtModelType::Vbic);
        assert_eq!(s, "VBIC");
    }

    #[test]
    fn test_bjt_model_type_serialization() {
        for model in BjtModelType::all() {
            let json = serde_json::to_string(model).unwrap();
            let deserialized: BjtModelType = serde_json::from_str(&json).unwrap();
            assert_eq!(*model, deserialized);
        }
    }

    // -------------------------------------------------------------------------
    // DiodeModelType Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_diode_model_type_default() {
        let model = DiodeModelType::default();
        assert_eq!(model, DiodeModelType::Standard);
    }

    #[test]
    fn test_diode_model_type_all() {
        let all = DiodeModelType::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&DiodeModelType::Standard));
        assert!(all.contains(&DiodeModelType::Schottky));
        assert!(all.contains(&DiodeModelType::Zener));
        assert!(all.contains(&DiodeModelType::Varactor));
    }

    #[test]
    fn test_diode_model_type_display_name() {
        assert_eq!(DiodeModelType::Standard.display_name(), "PN Junction");
        assert_eq!(DiodeModelType::Schottky.display_name(), "Schottky");
        assert_eq!(DiodeModelType::Zener.display_name(), "Zener");
        assert_eq!(DiodeModelType::Varactor.display_name(), "Varactor");
    }

    #[test]
    fn test_diode_model_type_description() {
        for model in DiodeModelType::all() {
            assert!(!model.description().is_empty());
        }
    }

    #[test]
    fn test_diode_model_type_display_trait() {
        let s = format!("{}", DiodeModelType::Schottky);
        assert_eq!(s, "Schottky");
    }

    #[test]
    fn test_diode_model_type_serialization() {
        for model in DiodeModelType::all() {
            let json = serde_json::to_string(model).unwrap();
            let deserialized: DiodeModelType = serde_json::from_str(&json).unwrap();
            assert_eq!(*model, deserialized);
        }
    }
}
