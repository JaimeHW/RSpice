//! Authored numerical and device-model compatibility, independent of source syntax.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum SimulationCompatibility {
    /// Preserve a source-authored policy or the engine default.
    #[default]
    Inherit,
    BestAvailable,
    Ngspice,
    Xyce,
}

impl SimulationCompatibility {
    pub const ALL: [Self; 4] = [
        Self::Inherit,
        Self::BestAvailable,
        Self::Ngspice,
        Self::Xyce,
    ];

    pub const fn is_inherited(&self) -> bool {
        matches!(self, Self::Inherit)
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Inherit => "From netlist (default: RSpice)",
            Self::BestAvailable => "RSpice best available",
            Self::Ngspice => "ngspice compatible",
            Self::Xyce => "Xyce compatible",
        }
    }

    pub const fn core_override(self) -> Option<rspice_core::SpiceDialect> {
        match self {
            Self::Inherit => None,
            Self::BestAvailable => Some(rspice_core::SpiceDialect::BestAvailable),
            Self::Ngspice => Some(rspice_core::SpiceDialect::Ngspice),
            Self::Xyce => Some(rspice_core::SpiceDialect::Xyce),
        }
    }

    pub const fn option_value(self) -> Option<&'static str> {
        match self {
            Self::Inherit => None,
            Self::BestAvailable => Some("BEST_AVAILABLE"),
            Self::Ngspice => Some("NGSPICE"),
            Self::Xyce => Some("XYCE"),
        }
    }
}
