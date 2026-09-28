//! Publication of stimulus cards already realized by the stimulus library.
use super::super::{component::Component, stimulus_provenance::StimulusProvenance};
use super::Schematic;
impl Schematic {
    pub fn stamp_placed_stimulus(&mut self, id: u64, receipt: &StimulusProvenance) {
        if let Some(component) = self
            .document
            .components
            .iter_mut()
            .find(|component| component.id == id)
        {
            receipt.stamp_onto(component);
        }
    }
    pub fn replace_stimulus_family(&mut self, candidate: Component, description: String) -> bool {
        let component_id = candidate.id;
        self.begin_operation(description);
        if let Some(held) = self
            .document
            .components
            .iter_mut()
            .find(|component| component.id == component_id)
        {
            *held = candidate;
        }
        self.invalidate_topology();
        self.end_operation()
    }
}
