//! Placing a source that is born having adopted a library definition.
//!
//! A definition is a placeable part, exactly as a model pack's card is: the
//! reader picks it, the cursor carries it, and the instance it becomes already
//! holds the definition's card and the receipt for it. Anything else would make
//! placement a two-step ritual — place a default source, then adopt onto it —
//! with an intermediate instance in the undo history that nobody authored.
//!
//! The payload is the receipt itself rather than a second copy of the four
//! fields. [`StimulusProvenance::of`] is the one place that decides what a
//! receipt records, so carrying the receipt is what keeps a placed adopter and
//! a dialog adopter provably identical: both stamp the same value.

use serde::{Deserialize, Serialize};

use super::super::component::Component;
use super::super::component_type::ComponentType;
use super::super::point::Point;
use super::SchematicState;
use crate::state::stimulus_library::definition::StimulusDefinition;
use crate::state::stimulus_library::provenance::StimulusProvenance;

/// One stimulus definition armed on the placement cursor.
///
/// Runtime interaction state, like every other `pending_*` payload: it is
/// retired when another tool is armed and it is never written to a document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingStimulusPlacement {
    /// The exact placement tool this definition belongs to, so re-arming any
    /// other tool retires it — the rule [`super::PendingPartModel`] keeps for
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
            receipt: StimulusProvenance::of(definition),
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
        component.value = self.receipt.value.clone();
        component.params = self.receipt.params.clone();
        component.stimulus_provenance = Some(self.receipt.clone());
    }
}

impl SchematicState {
    /// Place one `kind` at `pos` the way the armed tool means it: as an adopter
    /// of the armed stimulus definition when one is armed for exactly this
    /// type, and as a default instance otherwise.
    ///
    /// The choice lives here rather than at the click that asks for it,
    /// because the payload is this document's runtime state and the rule for
    /// when it applies is the same one [`Self::arm_tool`] retires it by.
    pub fn add_armed_component(&mut self, kind: ComponentType, pos: Point) -> u64 {
        match self
            .pending_stimulus
            .clone()
            .filter(|armed| armed.component_type == kind)
        {
            Some(armed) => self.add_stimulus_component(&armed, pos),
            None => self.add_component(kind, pos),
        }
    }

    /// Place one source that has already adopted `placement`.
    ///
    /// The copy and the receipt are written inside the same call that creates
    /// the instance, so a caller wrapping this in one `with_undo` group gets
    /// one undo entry that removes an adopted instance rather than two that
    /// step through a default source nobody placed.
    pub fn add_stimulus_component(
        &mut self,
        placement: &PendingStimulusPlacement,
        pos: Point,
    ) -> u64 {
        let id = self.add_component(placement.component_type, pos);
        if let Some(component) = self
            .components
            .iter_mut()
            .find(|component| component.id == id)
        {
            placement.stamp_onto(component);
        }
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::stimulus_library::provenance::ProvenanceState;
    use crate::state::{StimulusLibrary, Tool};

    fn library_with_sin() -> (StimulusLibrary, StimulusDefinition) {
        let mut definition =
            StimulusDefinition::new("sensor_drive", ComponentType::VoltageSourceSin)
                .expect("definition");
        definition.value = "0".to_owned();
        definition.params = "va=3m freq=1k".to_owned();
        let mut library = StimulusLibrary::default();
        library.insert(definition.clone()).expect("insert");
        (library, definition)
    }

    #[test]
    fn a_placed_instance_is_born_having_adopted_the_saved_revision() {
        let (library, definition) = library_with_sin();
        let placement = PendingStimulusPlacement::of(&definition);
        let mut schematic = SchematicState::default();
        schematic.init_undo_history();

        let id = schematic.add_stimulus_component(&placement, Point::new(10, 20));
        let component = schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("the instance was placed");
        assert_eq!(component.kind, ComponentType::VoltageSourceSin);
        assert_eq!(component.value, "0");
        assert_eq!(component.params, "va=3m freq=1k");
        assert_eq!(
            library.provenance_state(component),
            ProvenanceState::Adopted { revision: 1 }
        );
    }

    /// One undo removes the instance, because the copy and the receipt are
    /// written inside the same operation that created it. Driven through
    /// `add_armed_component`, which is the call the canvas click makes.
    #[test]
    fn one_undo_removes_the_whole_adopted_placement() {
        let (_, definition) = library_with_sin();
        let mut schematic = SchematicState::default();
        schematic.init_undo_history();
        schematic.pending_stimulus = Some(PendingStimulusPlacement::of(&definition));

        schematic.with_undo("place a sine source", |schematic| {
            schematic.add_armed_component(ComponentType::VoltageSourceSin, Point::new(4, 4));
        });
        assert_eq!(schematic.components.len(), 1);
        assert!(
            schematic.components[0].stimulus_provenance.is_some(),
            "the armed definition is on the placed instance"
        );
        assert!(schematic.undo());
        assert!(schematic.components.is_empty());
    }

    /// A definition armed for one type says nothing about another: the same
    /// call places a default instance of any other type.
    #[test]
    fn an_armed_definition_applies_only_to_its_own_type() {
        let (_, definition) = library_with_sin();
        let mut schematic = SchematicState::default();
        schematic.pending_stimulus = Some(PendingStimulusPlacement::of(&definition));

        let id = schematic.add_armed_component(ComponentType::VoltageSourcePulse, Point::new(4, 4));
        let placed = schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("placed");
        assert_eq!(placed.kind, ComponentType::VoltageSourcePulse);
        assert!(placed.stimulus_provenance.is_none());
    }

    /// Arming anything retires the armed definition, so a resistor can never
    /// be placed carrying a sine card. Arming is the only way to set one, and
    /// the payload is written after the tool, which is what leaves it armed.
    #[test]
    fn arming_another_tool_retires_the_armed_definition() {
        let (_, definition) = library_with_sin();
        let mut schematic = SchematicState::default();
        schematic.arm_tool(Tool::Place(ComponentType::VoltageSourceSin));
        schematic.pending_stimulus = Some(PendingStimulusPlacement::of(&definition));
        assert!(schematic.pending_stimulus.is_some());

        schematic.arm_tool(Tool::Place(ComponentType::Resistor));
        assert!(schematic.pending_stimulus.is_none());
    }

    #[test]
    fn cancelling_the_tool_clears_the_armed_definition() {
        let (_, definition) = library_with_sin();
        let mut schematic = SchematicState::default();
        schematic.arm_tool(Tool::Place(ComponentType::VoltageSourceSin));
        schematic.pending_stimulus = Some(PendingStimulusPlacement::of(&definition));

        schematic.cancel_tool();
        assert!(schematic.pending_stimulus.is_none());
        assert_eq!(schematic.tool, Tool::Select);
    }
}
