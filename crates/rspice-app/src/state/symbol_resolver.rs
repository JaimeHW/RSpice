//! Resolve symbols over the current editor buffers and library catalog.

use super::{CellViewRef, LibraryCellInstance, LibraryManager};
pub use rspice_design::resolved_symbol::ResolvedCellSymbol;
use rspice_design::schematic::{document::SchematicDocument, owned::Schematic};
use rspice_design::symbol_resolver as resolution;
use std::collections::HashMap;

pub struct SymbolResolver<'a, S: AsRef<SchematicDocument> = Schematic> {
    resolver: resolution::SymbolResolver<'a, S>,
}

impl<'a, S: AsRef<SchematicDocument>> SymbolResolver<'a, S> {
    pub fn new(libraries: &'a LibraryManager, schematic_buffers: &'a HashMap<String, S>) -> Self {
        Self {
            resolver: resolution::SymbolResolver::new(libraries.catalog(), schematic_buffers),
        }
    }
    pub fn resolve_binding(&self, binding: &LibraryCellInstance) -> Option<ResolvedCellSymbol> {
        self.resolver.resolve_binding(binding)
    }
    pub fn resolve_reference(&self, reference: &CellViewRef) -> Option<ResolvedCellSymbol> {
        self.resolver.resolve_reference(reference)
    }
}
