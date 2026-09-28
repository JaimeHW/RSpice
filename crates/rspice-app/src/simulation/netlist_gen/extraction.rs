//! Bind application hierarchy lookups to the shared connectivity extraction.

use super::HierarchySource;
#[cfg(test)]
use crate::state::SchematicState;
use crate::state::{Component, Point};
use rspice_design::schematic::document::SchematicDocument;

pub use rspice_design::connectivity::ExtractedConnectivity;

pub fn extract(
    schematic: &impl AsRef<SchematicDocument>,
    hierarchy: Option<&HierarchySource<'_>>,
) -> ExtractedConnectivity {
    rspice_design::connectivity::extract(
        schematic.as_ref(),
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
pub(super) fn display_net_name(schematic: &impl AsRef<SchematicDocument>, name: &str) -> String {
    rspice_design::connectivity::display_net_name(schematic.as_ref(), name)
}

#[cfg(test)]
mod tests;
