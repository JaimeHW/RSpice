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

/// The simulator a project's deck was written for, where that changes what
/// its cards mean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum SourceSimulator {
    #[default]
    Native,
    Ltspice,
    Qspice,
}

impl SourceSimulator {
    pub const fn is_native(&self) -> bool {
        matches!(self, Self::Native)
    }

    /// The simulator whose source defaults a drawing from this program asks
    /// for; a native project names none.
    pub const fn core_source(self) -> Option<rspice_core::netlist::SourceSimulator> {
        match self {
            Self::Native => None,
            Self::Ltspice => Some(rspice_core::netlist::SourceSimulator::Ltspice),
            Self::Qspice => Some(rspice_core::netlist::SourceSimulator::Qspice),
        }
    }
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

    /// The choices a plan offers. A plan drawn from a schematic has no source
    /// of its own to defer to, so inheriting is RSpice's own numerics and is
    /// offered once, as RSpice; a netlist test may state a dialect of its own.
    pub const fn offered(netlist_test: bool) -> &'static [Self] {
        if netlist_test {
            &Self::ALL
        } else {
            &[Self::Inherit, Self::Ngspice, Self::Xyce]
        }
    }

    pub const fn label(self, netlist_test: bool) -> &'static str {
        match self {
            Self::Inherit if netlist_test => "As the netlist says",
            Self::Inherit | Self::BestAvailable => "RSpice",
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

#[cfg(test)]
mod tests {
    use super::SimulationCompatibility;

    /// A plan drawn from a schematic offers RSpice, ngspice and Xyce; only a
    /// netlist test offers to follow what its own deck states.
    #[test]
    fn only_a_netlist_test_offers_to_follow_its_deck() {
        let words = |netlist_test| {
            SimulationCompatibility::offered(netlist_test)
                .iter()
                .map(|choice| choice.label(netlist_test))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            words(false),
            ["RSpice", "ngspice compatible", "Xyce compatible"]
        );
        assert_eq!(
            words(true),
            [
                "As the netlist says",
                "RSpice",
                "ngspice compatible",
                "Xyce compatible"
            ]
        );
    }
}
