//! The symbols the open drawing resolves through, read from the model alone.
//! The canvas, the sheet's drawing and the Sheet inspector all resolve the
//! drawing's symbols, so each is one answer whichever part asks.

use super::Model;
use crate::state::symbol_context_cache::SymbolContextInputs;
use rspice_schematic_editor::view::symbol_context::SchematicSymbolContext;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

/// The model is itself a holder of the model, so an operation that takes any
/// holder (`impl AsRef<Model>`) takes a feature's model as it takes the whole
/// application state.
impl AsRef<Model> for Model {
    fn as_ref(&self) -> &Model {
        self
    }
}

impl Model {
    /// A fingerprint of everything the open drawing's symbols resolve from:
    /// the libraries and every schematic buffer's topology.
    pub(crate) fn symbol_context_revision(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.library_manager.revision().hash(&mut hasher);
        self.workspace
            .content
            .schematic_buffers
            .len()
            .hash(&mut hasher);
        let mut folded_xor = 0_u64;
        let mut folded_sum = 0_u64;
        for (key, buffer) in &self.workspace.content.schematic_buffers {
            let mut entry = DefaultHasher::new();
            key.hash(&mut entry);
            buffer.topology_version().hash(&mut entry);
            let entry = entry.finish();
            folded_xor ^= entry;
            folded_sum = folded_sum.wrapping_add(entry.rotate_left(17));
        }
        folded_xor.hash(&mut hasher);
        folded_sum.hash(&mut hasher);
        hasher.finish()
    }

    /// Compose the editor's resolved symbols from the current model-owned
    /// sources.
    pub(crate) fn schematic_symbol_context(&self) -> SchematicSymbolContext {
        let resolver = rspice_design::symbol_resolver::SymbolResolver::new(
            self.library_manager.catalog(),
            &self.workspace.content.schematic_buffers,
        );
        SchematicSymbolContext::new(
            self.schematic.document(),
            self.schematic
                .session
                .editor
                .pending_library_cell
                .as_ref()
                .map(|pending| &pending.binding),
            &resolver,
            self.symbol_context_revision(),
        )
    }

    /// The resolved symbols a frame paints and hit-tests with: the context
    /// the open document's session holds, while everything it was resolved
    /// from stands still, and one resolved afresh and held otherwise.
    pub(crate) fn frame_symbol_context(&mut self) -> Arc<SchematicSymbolContext> {
        let revision = self.symbol_context_revision();
        let topology_version = self.schematic.topology_version();
        let pending = self
            .schematic
            .session
            .editor
            .pending_library_cell
            .as_ref()
            .map(|pending| &pending.binding);
        if let Some(context) = self.schematic.session.symbol_context.current(
            revision,
            topology_version,
            pending,
            self.schematic.document(),
        ) {
            return context;
        }
        let inputs = SymbolContextInputs::capture(
            revision,
            topology_version,
            pending,
            self.schematic.document(),
        );
        let context = Arc::new(self.schematic_symbol_context());
        self.schematic
            .session
            .symbol_context
            .store(inputs, Arc::clone(&context));
        context
    }
}
