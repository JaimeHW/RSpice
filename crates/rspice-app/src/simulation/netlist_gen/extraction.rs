//! Bind application hierarchy lookups to the shared connectivity extraction.

use super::HierarchySource;
use crate::state::{Component, Point, SchematicState};

mod pass;
pub use pass::{
    ConnectivityAnchor, ConnectivityDiagnosticKind, ExtractedConnectivity, ExtractedTerminal,
};

pub fn extract(
    schematic: &SchematicState,
    hierarchy: Option<&HierarchySource<'_>>,
) -> ExtractedConnectivity {
    pass::extract(
        &schematic.document,
        |binding| hierarchy?.resolved_symbol_for(binding),
        |name| hierarchy?.canonical_global_label(name),
    )
}

pub(super) fn terminal_positions(
    component: &Component,
    hierarchy: Option<&HierarchySource<'_>>,
) -> Vec<(String, Point)> {
    pass::terminal_positions(component, &|binding| {
        hierarchy?.resolved_symbol_for(binding)
    })
}

#[cfg(test)]
pub(super) fn display_net_name(schematic: &SchematicState, name: &str) -> String {
    pass::display_net_name(&schematic.document, name)
}

#[cfg(test)]
mod tests;
