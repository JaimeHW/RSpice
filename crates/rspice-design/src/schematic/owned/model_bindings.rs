//! Model binding changes after catalog validation, with their original undo semantics.

use super::super::history::SchematicSnapshot;
use super::Schematic;
use std::path::PathBuf;

impl Schematic {
    pub fn select_instance_model(
        &mut self,
        component_id: u64,
        candidate_name: String,
        candidate_source: Option<PathBuf>,
    ) -> bool {
        let before = SchematicSnapshot::capture(&self.document);
        let component = self
            .document
            .components
            .iter_mut()
            .find(|component| component.id == component_id)
            .expect("the validated component remains present until mutation");
        let binding = component
            .library_cell
            .as_mut()
            .expect("the validated library binding remains present until mutation");
        let mut changed = binding.module_name.as_deref() != Some(candidate_name.as_str());
        binding.module_name = Some(candidate_name);
        if candidate_source.is_some() && binding.source_path != candidate_source {
            binding.source_path = candidate_source;
            binding.model_section = None;
            changed = true;
        }
        if changed {
            self.invalidate_topology();
            self.begin_operation_from(before, "select instance model");
            self.end_operation();
        }
        changed
    }

    pub fn select_instance_model_section(
        &mut self,
        component_id: u64,
        selected: Option<String>,
    ) -> bool {
        let before = SchematicSnapshot::capture(&self.document);
        let Some(binding) = self
            .document
            .components
            .iter_mut()
            .find(|component| component.id == component_id)
            .and_then(|component| component.library_cell.as_mut())
        else {
            return false;
        };
        if binding.model_section == selected {
            return false;
        }
        binding.model_section = selected;
        self.invalidate_topology();
        self.begin_operation_from(before, "select instance model section");
        self.end_operation();
        true
    }
}
