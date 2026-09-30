//! Sealing the model sources a run will execute against.
//!
//! A seal pins each source by content digest and closes over its dependency
//! edges, so what a run executes is fixed at seal time and cannot be changed
//! by editing a file afterwards. Digests are computed, never accepted: a
//! source whose content no longer matches its stored digest fails the seal
//! rather than being silently re-pinned to the new bytes.

use super::*;

impl ModelLibraryManager {
    /// Compute the canonical SHA-256 identity used to pin an external model
    /// source. Callers compare this value with the digest stored by the last
    /// explicit load/refresh; computing it never accepts new content.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn calculate_source_digest(
        path: impl AsRef<std::path::Path>,
    ) -> Result<crate::product::ContentDigest, String> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)
            .map_err(|error| format!("Failed to read '{}': {error}", path.display()))?;
        Ok(crate::product::ContentDigest::from_bytes(
            Sha256::digest(&bytes).into(),
        ))
    }

    /// Prove that the project-owned technology attachment still names the
    /// exact live execution catalog entry accepted at attachment time.
    pub fn validate_attached_technology(
        &self,
        binding: Option<&crate::state::ProjectTechnologyBinding>,
    ) -> Result<(), String> {
        let Some(binding) = binding else {
            return Ok(());
        };
        let library = self.get_library(binding.model_library()).ok_or_else(|| {
            format!(
                "Attached technology library '{}' was removed; reattach an authenticated model library before simulation",
                binding.model_library()
            )
        })?;
        binding.validate_model_library(library).map_err(|error| {
            format!(
                "Attached technology contract is stale: {error}. Reattach the current model library before simulation"
            )
        })
    }

    /// Build one all-or-nothing source snapshot for a simulation run.
    ///
    /// Every unique pinned member is read exactly once. The digest is computed
    /// over those same bytes, and only the authenticated UTF-8 content is
    /// published to the in-memory resolver.
    pub fn seal_execution_sources(&self) -> Result<SealedModelExecutionSources, String> {
        rspice_simulation::model_sources::seal_catalog_execution_sources(
            &self.catalog,
            &self.resolution_records,
        )
    }

    /// Capture the manager's current executable catalog as an explicit plan
    /// binding list. This is used only for schema migration and deliberate
    /// user attachment; execution never calls it implicitly.
    #[must_use]
    pub fn default_simulation_plan_bindings(&self) -> Vec<SimulationPlanModelBinding> {
        self.catalog.default_simulation_plan_bindings()
    }

    /// Create one binding for a library the user explicitly attaches to a
    /// plan. Source-less catalog entries cannot be execution bindings.
    pub fn simulation_plan_binding(
        &self,
        library_name: &str,
    ) -> Result<SimulationPlanModelBinding, String> {
        self.catalog.simulation_plan_binding(library_name)
    }

    /// Seal exactly the libraries selected by a simulation plan, preserving
    /// their declared order and nominal section overrides.
    pub fn seal_execution_sources_for_plan(
        &self,
        bindings: &[SimulationPlanModelBinding],
    ) -> Result<SealedModelExecutionSources, String> {
        rspice_simulation::model_sources::seal_plan_execution_sources(
            &self.catalog,
            &self.resolution_records,
            bindings,
        )
    }

    /// Validate binding identities and selected sections against the current
    /// catalog without reading external source bytes.
    pub fn validate_simulation_plan_bindings(
        &self,
        bindings: &[SimulationPlanModelBinding],
    ) -> Result<(), String> {
        self.catalog.validate_simulation_plan_bindings(bindings)
    }
}
