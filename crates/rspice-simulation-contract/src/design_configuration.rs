//! Which hierarchy configuration a simulation plan's design elaborates under.
//!
//! A plan is a test: it names the design it simulates the way an ADE test
//! names its lib/cell/config view, so two plans in one project can run the
//! schematic hierarchy and the extracted one side by side. The project marks
//! one configuration "Used for runs"; a plan follows that mark unless it names
//! a configuration of its own.

use rspice_design_model::primitives::ConfigurationSetId;
use serde::{Deserialize, Serialize};

/// The configuration a plan's runs elaborate and netlist.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum DesignConfiguration {
    /// Whichever configuration the project marks "Used for runs", or the
    /// built-in default when none is marked.
    #[default]
    UsedForRuns,
    /// The built-in default: the project's top cell through the default view
    /// search order, whatever the project marks for runs.
    BuiltIn,
    /// One configuration of the project's catalog, by identity, so a rename
    /// keeps the plan on it and a deletion is stated rather than followed.
    Named(ConfigurationSetId),
}

impl DesignConfiguration {
    /// Whether the plan follows the project's "Used for runs" mark, which is
    /// what a plan that never chose is stored as.
    #[must_use]
    pub const fn follows_runs(&self) -> bool {
        matches!(self, Self::UsedForRuns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_named_configuration_round_trips_by_identity() {
        let id = ConfigurationSetId::new();
        let stored = serde_json::to_string(&DesignConfiguration::Named(id)).expect("serializes");
        assert_eq!(stored, format!("{{\"named\":\"{id}\"}}"));
        let restored: DesignConfiguration = serde_json::from_str(&stored).expect("restores");
        assert_eq!(restored, DesignConfiguration::Named(id));
        assert_eq!(
            serde_json::from_str::<DesignConfiguration>("\"built_in\"").expect("restores"),
            DesignConfiguration::BuiltIn
        );
    }
}
