//! Ordered simulation-plan bindings resolved without host source reads.

use super::ModelCatalog;
use crate::{ModelLibrary, SimulationPlanModelBinding, model_library_source_digest};
use std::collections::HashSet;

impl ModelCatalog {
    pub fn default_simulation_plan_bindings(&self) -> Vec<SimulationPlanModelBinding> {
        let mut libraries = self
            .libraries()
            .filter(|library| library.source_authority.has_execution_source())
            .collect::<Vec<_>>();
        libraries.sort_by(|left, right| left.name.cmp(&right.name));
        libraries
            .into_iter()
            .map(|library| SimulationPlanModelBinding {
                library_name: library.name.clone(),
                source_digest: model_library_source_digest(library),
                selected_corner: library.selected_corner.clone(),
            })
            .collect()
    }

    pub fn simulation_plan_binding(
        &self,
        library_name: &str,
    ) -> Result<SimulationPlanModelBinding, String> {
        let library = self
            .get_library(library_name)
            .ok_or_else(|| format!("Model library '{library_name}' is no longer loaded"))?;
        if !library.source_authority.has_execution_source() {
            return Err(format!(
                "Model library '{}' has no executable source authority",
                library.name
            ));
        }
        Ok(SimulationPlanModelBinding {
            library_name: library.name.clone(),
            source_digest: model_library_source_digest(library),
            selected_corner: library.selected_corner.clone(),
        })
    }

    pub fn validate_simulation_plan_bindings(
        &self,
        bindings: &[SimulationPlanModelBinding],
    ) -> Result<(), String> {
        self.resolve_simulation_plan_bindings(bindings).map(|_| ())
    }

    pub fn resolve_simulation_plan_bindings(
        &self,
        bindings: &[SimulationPlanModelBinding],
    ) -> Result<Vec<(&ModelLibrary, Option<String>)>, String> {
        let mut names = HashSet::with_capacity(bindings.len());
        let mut resolved = Vec::with_capacity(bindings.len());
        for (index, binding) in bindings.iter().enumerate() {
            binding
                .validate()
                .map_err(|error| format!("Model binding {} is invalid: {error}", index + 1))?;
            let uniqueness_key = binding.library_name.to_ascii_lowercase();
            if !names.insert(uniqueness_key) {
                return Err(format!(
                    "Simulation plan binds model library '{}' more than once",
                    binding.library_name
                ));
            }
            let library = self.get_library(&binding.library_name).ok_or_else(|| {
                format!(
                    "Simulation-plan model binding '{}' is stale because that library is no longer loaded",
                    binding.library_name
                )
            })?;
            if !library.source_authority.has_execution_source() {
                return Err(format!(
                    "Simulation-plan model binding '{}' is not executable because the library has no source authority",
                    binding.library_name
                ));
            }
            let actual_digest = model_library_source_digest(library);
            if actual_digest != binding.source_digest && !library.source_closure.is_empty() {
                return Err(format!(
                    "Simulation-plan model binding '{}' is stale because its accepted source digest changed; review and reattach the library",
                    binding.library_name
                ));
            }
            if let Some(selected) = binding.selected_corner.as_deref()
                && !library
                    .corners
                    .values()
                    .any(|corner| corner.name.eq_ignore_ascii_case(selected))
            {
                return Err(format!(
                    "Simulation-plan model binding '{}' selects missing corner section '{}'",
                    binding.library_name, selected
                ));
            }
            resolved.push((library, binding.selected_corner.clone()));
        }
        Ok(resolved)
    }
}
