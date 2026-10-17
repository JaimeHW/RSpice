//! The canvas's resolved symbols, kept across frames while what they were
//! resolved from stands still.
//!
//! Resolving an authored symbol parses its master's stored document. Rebuilt
//! every frame, that was one parse per hierarchical instance per frame, and
//! the Sheet inspector paid it a second time.

use std::sync::Arc;

use rspice_design::schematic::{
    component::LibraryCellInstance, component_type::ComponentType, document::SchematicDocument,
};
use rspice_schematic_editor::view::symbol_context::SchematicSymbolContext;

/// Everything a resolved symbol context was built from.
///
/// The revision and topology counters are cheap, but each is only as good as
/// every writer's discipline. The instance bindings are therefore compared by
/// content too, so a binding changed without a topology bump still resolves
/// afresh.
#[derive(Clone, PartialEq)]
pub(crate) struct SymbolContextInputs {
    revision: u64,
    topology_version: u64,
    pending: Option<LibraryCellInstance>,
    bindings: Vec<(u64, LibraryCellInstance)>,
}

/// The bound instances a symbol context resolves, in document order.
fn instance_bindings(
    document: &SchematicDocument,
) -> impl Iterator<Item = (u64, &LibraryCellInstance)> {
    document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::CellInstance)
        .filter_map(|component| Some((component.id, component.library_cell.as_ref()?)))
}

impl SymbolContextInputs {
    pub(crate) fn capture(
        revision: u64,
        topology_version: u64,
        pending: Option<&LibraryCellInstance>,
        document: &SchematicDocument,
    ) -> Self {
        Self {
            revision,
            topology_version,
            pending: pending.cloned(),
            bindings: instance_bindings(document)
                .map(|(id, binding)| (id, binding.clone()))
                .collect(),
        }
    }

    /// Whether these are the inputs as they stand now. Allocation-free, since
    /// it runs every frame.
    fn match_now(
        &self,
        revision: u64,
        topology_version: u64,
        pending: Option<&LibraryCellInstance>,
        document: &SchematicDocument,
    ) -> bool {
        self.revision == revision
            && self.topology_version == topology_version
            && self.pending.as_ref() == pending
            && self
                .bindings
                .iter()
                .map(|(id, binding)| (*id, binding))
                .eq(instance_bindings(document))
    }
}

/// One cached symbol context and the inputs it was resolved from.
#[derive(Clone, Default)]
pub struct SymbolContextCache {
    entry: Option<(SymbolContextInputs, Arc<SchematicSymbolContext>)>,
}

impl std::fmt::Debug for SymbolContextCache {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SymbolContextCache")
            .field("cached", &self.entry.is_some())
            .finish()
    }
}

impl SymbolContextCache {
    /// The cached context, if it was resolved from exactly these inputs.
    pub(crate) fn current(
        &self,
        revision: u64,
        topology_version: u64,
        pending: Option<&LibraryCellInstance>,
        document: &SchematicDocument,
    ) -> Option<Arc<SchematicSymbolContext>> {
        self.entry
            .as_ref()
            .filter(|(inputs, _)| inputs.match_now(revision, topology_version, pending, document))
            .map(|(_, context)| Arc::clone(context))
    }

    pub(crate) fn store(
        &mut self,
        inputs: SymbolContextInputs,
        context: Arc<SchematicSymbolContext>,
    ) {
        self.entry = Some((inputs, context));
    }
}
