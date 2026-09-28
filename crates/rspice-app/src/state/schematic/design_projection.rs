//! App candidates retain their editor state around design-owned projections.
use crate::state::{DesignManagementCatalog, SchematicState};
impl SchematicState {
    pub(crate) fn materialize_design_management_schematic(
        &self,
        design_management: &DesignManagementCatalog,
        cell_view_key: &str,
    ) -> Result<Self, crate::state::DesignManagementError> {
        let (design, repaired) = self
            .design
            .materialize_design_management_schematic(design_management, cell_view_key)?;
        let mut projected = self.clone_with_design(design);
        projected.repair_clipboard_after_load();
        projected.remove_stale_runtime_references(&repaired);
        Ok(projected)
    }
    pub(crate) fn apply_variant_replacement(
        &mut self,
        prepared: rspice_design::schematic::component::PreparedVariantReplacement,
    ) {
        self.design.apply_variant_replacement(prepared);
    }
}
