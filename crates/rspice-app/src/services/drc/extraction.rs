//! Bind the active hierarchy to the headless design rule checker.

use super::netlist_gen::HierarchySource;
use super::netlist_gen::extraction::ExtractedConnectivity;
use crate::state::ComponentType;
use rspice_design::drc::DrcResult;
use rspice_design::drc::{ComponentInfo, DrcConfig, extract_components};

/// Resolve the design once, and bind every placed component to it.
pub(super) fn extract_checked_design(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
) -> (Vec<ComponentInfo>, ExtractedConnectivity) {
    let connectivity = super::netlist_gen::extraction::extract(schematic, Some(hierarchy));
    let components = extract_components(&schematic.document(), &connectivity, |comp| {
        if comp.kind != ComponentType::CellInstance {
            return Some(true);
        }
        let Some(binding) = comp.library_cell.as_ref() else {
            return Some(false);
        };
        if binding.source_path.is_some()
            || binding.netlist_template.is_some()
            || binding.is_executable_builtin()
        {
            return Some(true);
        }
        if hierarchy.has_execution_plan() {
            // Top-level DRC extraction has no instance path with which to
            // query an execution-plan rebind. Do not judge the placed
            // binding when the executable authority may select another
            // master.
            return None;
        }
        Some(hierarchy.schematic_master_for_binding(binding).is_some())
    });
    (components, connectivity)
}

/// Run DRC using the active hierarchy's symbol and binding authority.
pub fn run_drc_check_with_hierarchy_and_config(
    schematic: &crate::state::SchematicState,
    hierarchy: &HierarchySource<'_>,
    config: DrcConfig,
) -> DrcResult {
    rspice_design::drc::run_check(&schematic.document(), config, || {
        extract_checked_design(schematic, hierarchy)
    })
}

#[cfg(test)]
mod tests;
