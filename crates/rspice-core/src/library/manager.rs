//! The device class a library `.MODEL` card declares.

/// Type of SPICE model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelType {
    /// Diode (D)
    Diode,
    /// NPN Bipolar Junction Transistor
    NpnBjt,
    /// PNP Bipolar Junction Transistor
    PnpBjt,
    /// N-channel MOSFET
    Nmos,
    /// P-channel MOSFET
    Pmos,
    /// N-channel JFET
    Njfet,
    /// P-channel JFET
    Pjfet,
    /// Resistor model
    Resistor,
    /// Capacitor model
    Capacitor,
    /// Unknown/other
    Other,
}

impl ModelType {
    /// Parse model type from SPICE model type string
    pub fn from_spice_type(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "D" => ModelType::Diode,
            "NPN" => ModelType::NpnBjt,
            "PNP" => ModelType::PnpBjt,
            "NMOS" => ModelType::Nmos,
            "PMOS" => ModelType::Pmos,
            "NJF" => ModelType::Njfet,
            "PJF" => ModelType::Pjfet,
            "R" => ModelType::Resistor,
            "C" => ModelType::Capacitor,
            _ => ModelType::Other,
        }
    }

    /// Get display name for the model type
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelType::Diode => "Diodes",
            ModelType::NpnBjt => "NPN Transistors",
            ModelType::PnpBjt => "PNP Transistors",
            ModelType::Nmos => "N-MOSFETs",
            ModelType::Pmos => "P-MOSFETs",
            ModelType::Njfet => "N-JFETs",
            ModelType::Pjfet => "P-JFETs",
            ModelType::Resistor => "Resistor Models",
            ModelType::Capacitor => "Capacitor Models",
            ModelType::Other => "Other",
        }
    }

    /// Get SPICE letter prefix for this model type
    pub fn spice_prefix(&self) -> &'static str {
        match self {
            ModelType::Diode => "D",
            ModelType::NpnBjt | ModelType::PnpBjt => "Q",
            ModelType::Nmos | ModelType::Pmos => "M",
            ModelType::Njfet | ModelType::Pjfet => "J",
            ModelType::Resistor => "R",
            ModelType::Capacitor => "C",
            ModelType::Other => "X",
        }
    }
}
