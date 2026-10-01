//! Stimulus provenance carried by an armed independent-source placement.

use rspice_design::schematic::{component::Component, component_type::ComponentType};
use rspice_design::stimulus_library::{
    definition::StimulusDefinition, provenance::StimulusProvenance,
};
use serde::{Deserialize, Serialize};

/// One stimulus definition armed on the placement cursor.
///
/// Runtime interaction state, like every other `pending_*` payload: it is
/// retired when another tool is armed and it is never written to a document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingStimulusPlacement {
    /// The exact placement tool this definition belongs to, so re-arming any
    /// other tool retires it — the rule [`super::placement::PendingPartModel`] keeps for
    /// the same reason.
    pub component_type: ComponentType,
    /// The copy the placed instance is born holding: the definition's name,
    /// the revision, and the two card fields as they were read.
    pub receipt: StimulusProvenance,
}

impl PendingStimulusPlacement {
    /// Arm this definition's saved revision.
    #[must_use]
    pub fn of(definition: &StimulusDefinition) -> Self {
        Self {
            component_type: definition.component_type(),
            receipt: definition.provenance(),
        }
    }

    /// The definition's name, as the console line and the shelf row spell it.
    #[must_use]
    pub fn definition(&self) -> &str {
        &self.receipt.definition
    }

    /// The revision that will be stamped.
    #[must_use]
    pub const fn revision(&self) -> u32 {
        self.receipt.revision
    }

    /// Write the copy and its receipt onto a freshly placed instance.
    ///
    /// The same two fields adoption writes, from the same record, so an
    /// instance placed from a definition and one adopted onto afterwards carry
    /// byte-identical cards.
    pub fn stamp_onto(&self, component: &mut Component) {
        self.receipt.stamp_onto(component);
    }
}
