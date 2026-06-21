use serde::{Deserialize, Serialize};

/// Type/category of device model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ModelType {
    /// NMOS transistor
    #[default]
    Nmos,
    /// PMOS transistor
    Pmos,
    /// NPN bipolar
    Npn,
    /// PNP bipolar
    Pnp,
    /// Resistor
    Resistor,
    /// Capacitor
    Capacitor,
    /// Inductor
    Inductor,
    /// Diode
    Diode,
    /// Varactor
    Varactor,
    /// RF device
    Rf,
    /// ESD protection
    Esd,
    /// Custom/other
    Other,
}

impl ModelType {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelType::Nmos => "NMOS",
            ModelType::Pmos => "PMOS",
            ModelType::Npn => "NPN",
            ModelType::Pnp => "PNP",
            ModelType::Resistor => "Resistor",
            ModelType::Capacitor => "Capacitor",
            ModelType::Inductor => "Inductor",
            ModelType::Diode => "Diode",
            ModelType::Varactor => "Varactor",
            ModelType::Rf => "RF",
            ModelType::Esd => "ESD",
            ModelType::Other => "Other",
        }
    }

    /// Parse from string
    pub fn from_name(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "nmos" | "nch" | "n" => ModelType::Nmos,
            "pmos" | "pch" | "p" => ModelType::Pmos,
            "npn" => ModelType::Npn,
            "pnp" => ModelType::Pnp,
            "r" | "res" | "resistor" => ModelType::Resistor,
            "c" | "cap" | "capacitor" => ModelType::Capacitor,
            "l" | "ind" | "inductor" => ModelType::Inductor,
            "d" | "diode" => ModelType::Diode,
            "var" | "varactor" => ModelType::Varactor,
            "rf" => ModelType::Rf,
            "esd" => ModelType::Esd,
            _ => ModelType::Other,
        }
    }

    /// Icon for UI
    pub fn icon(&self) -> &'static str {
        match self {
            ModelType::Nmos => "NM",
            ModelType::Pmos => "PM",
            ModelType::Npn => "QN",
            ModelType::Pnp => "QP",
            ModelType::Resistor => "R",
            ModelType::Capacitor => "C",
            ModelType::Inductor => "L",
            ModelType::Diode => "D",
            ModelType::Varactor => "VAR",
            ModelType::Rf => "RF",
            ModelType::Esd => "ESD",
            ModelType::Other => "?",
        }
    }
}

/// SPICE model level/type.
///
/// This classifies what a model card *claims to be* for browsing and
/// filtering — it is not a statement of native engine support. Cards run
/// natively only for the levels the core implements; bundled model metadata
/// is not a promise that a third-party Verilog-A source is shipped or
/// simulation-ready.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ModelLevel {
    /// BSIM3 v3.3
    Bsim3v3,
    /// BSIM4
    Bsim4,
    /// BSIM CMG (FinFET)
    BsimCmg,
    /// PSP
    Psp,
    /// EKV
    Ekv,
    /// Verilog-A compact model
    VerilogA,
    /// SPICE Level 1
    SpiceLevel1,
    /// SPICE Level 3
    SpiceLevel3,
    /// Unknown/custom
    #[default]
    Unknown,
}

impl ModelLevel {
    /// Display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelLevel::Bsim3v3 => "BSIM3v3",
            ModelLevel::Bsim4 => "BSIM4",
            ModelLevel::BsimCmg => "BSIM-CMG",
            ModelLevel::Psp => "PSP",
            ModelLevel::Ekv => "EKV",
            ModelLevel::VerilogA => "Verilog-A",
            ModelLevel::SpiceLevel1 => "SPICE L1",
            ModelLevel::SpiceLevel3 => "SPICE L3",
            ModelLevel::Unknown => "Unknown",
        }
    }
}
