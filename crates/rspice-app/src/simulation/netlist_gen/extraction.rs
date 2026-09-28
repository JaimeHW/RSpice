//! Bind application hierarchy lookups to the shared connectivity extraction.

use super::HierarchySource;
use crate::state::{Component, Point, SchematicState};

pub use rspice_design::connectivity::ExtractedConnectivity;

pub fn extract(
    schematic: &SchematicState,
    hierarchy: Option<&HierarchySource<'_>>,
) -> ExtractedConnectivity {
    rspice_design::connectivity::extract(
        &schematic.document(),
        |binding| hierarchy?.resolved_symbol_for(binding),
        |name| hierarchy?.canonical_global_label(name),
    )
}

pub(super) fn terminal_positions(
    component: &Component,
    hierarchy: Option<&HierarchySource<'_>>,
) -> Vec<(String, Point)> {
    rspice_design::connectivity::terminal_positions(component, &|binding| {
        hierarchy?.resolved_symbol_for(binding)
    })
}

#[cfg(test)]
pub(super) fn display_net_name(schematic: &SchematicState, name: &str) -> String {
    rspice_design::connectivity::display_net_name(&schematic.document, name)
}

#[cfg(test)]
mod tests;
