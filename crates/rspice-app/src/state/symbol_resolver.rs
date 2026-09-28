//! Resolve symbols over the current editor buffers and library catalog.

use super::{CellViewRef, LibraryCellInstance, LibraryManager, SchematicState};
pub use rspice_design::resolved_symbol::{
    ResolvedCellSymbol, ResolvedSymbolIssueKind, ResolvedSymbolSource,
};
use rspice_design::symbol_resolver as resolution;
use std::collections::HashMap;

pub struct SymbolResolver<'a> {
    resolver: resolution::SymbolResolver<'a, SchematicState>,
}

impl<'a> SymbolResolver<'a> {
    pub fn new(
        libraries: &'a LibraryManager,
        schematic_buffers: &'a HashMap<String, SchematicState>,
    ) -> Self {
        Self {
            resolver: resolution::SymbolResolver::new(libraries.catalog(), schematic_buffers),
        }
    }
    /// Overlay an unsaved editor buffer without cloning the workspace.
    pub(crate) fn with_active_schematic(
        mut self,
        reference: &'a CellViewRef,
        schematic: &'a SchematicState,
    ) -> Self {
        self.resolver = self
            .resolver
            .with_active_schematic(reference, schematic.document());
        self
    }
    pub fn resolve_binding(&self, binding: &LibraryCellInstance) -> Option<ResolvedCellSymbol> {
        self.resolver.resolve_binding(binding)
    }
    pub fn resolve_reference(&self, reference: &CellViewRef) -> Option<ResolvedCellSymbol> {
        self.resolver.resolve_reference(reference)
    }
}
