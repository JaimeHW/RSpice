//! What a placeable device is for: the functional groups the Component shelf
//! files every built-in part under.
//!
//! A device is filed by function, never by how the engine implements it. A
//! native element, a code model and a behavioral block that do the same job
//! share a group; the implementation shows only as the row's glyph.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComponentCategory {
    HierarchyConnectivity,
    Passives,
    Sources,
    Semiconductors,
    AmplifiersComparators,
    ControlledSourcesSwitches,
    LogicTiming,
    DigitalEventDriven,
    BehavioralMath,
    Magnetics,
    LinesRf,
    ProtectionElectromechanical,
    ProbesMeters,
}

impl ComponentCategory {
    /// Every group, in the order the shelf lists them.
    pub const ALL: [Self; 13] = [
        Self::HierarchyConnectivity,
        Self::Passives,
        Self::Sources,
        Self::Semiconductors,
        Self::AmplifiersComparators,
        Self::ControlledSourcesSwitches,
        Self::LogicTiming,
        Self::DigitalEventDriven,
        Self::BehavioralMath,
        Self::Magnetics,
        Self::LinesRf,
        Self::ProtectionElectromechanical,
        Self::ProbesMeters,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            Self::HierarchyConnectivity => "Hierarchy & connectivity",
            Self::Passives => "Passives",
            Self::Sources => "Sources",
            Self::Semiconductors => "Semiconductors",
            Self::AmplifiersComparators => "Amplifiers & comparators",
            Self::ControlledSourcesSwitches => "Controlled sources & switches",
            Self::LogicTiming => "Logic & timing",
            Self::DigitalEventDriven => "Digital (event-driven)",
            Self::BehavioralMath => "Behavioral & math",
            Self::Magnetics => "Magnetics",
            Self::LinesRf => "Lines & RF",
            Self::ProtectionElectromechanical => "Protection & electromechanical",
            Self::ProbesMeters => "Probes & meters",
        }
    }
}
