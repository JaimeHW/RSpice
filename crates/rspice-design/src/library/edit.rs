//! Scoped catalog edits keep live library records behind their owner.

use super::{Cell, Library, View, ViewType};
use crate::model_bound_symbol::{SymbolConstructionPlan, prepare_symbol_construction};
use crate::symbol::SymbolDocument;
use crate::symbol::edit::{SymbolDocumentSnapshot, restore_symbol_snapshot_in_view};
use crate::symbol::publication::EncodedSymbolEditorBundle;
use rspice_model_library::symbol::{ModelBoundSymbolDefinition, SymbolDefinitionError};
use std::collections::HashMap;

/// One catalog mutation scope. The exclusive borrow fixes its target library;
/// records can be inspected but only the named operations can mutate them.
/// Project permissions and publication remain with the enclosing transaction.
pub struct LibraryEdit<'a> {
    library: &'a mut Library,
}

impl<'a> LibraryEdit<'a> {
    pub(super) fn new(library: &'a mut Library) -> Self {
        Self { library }
    }

    pub fn library(&self) -> &Library {
        self.library
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.library.add_cell(cell);
    }

    pub fn remove_cell(&mut self, name: &str) -> bool {
        self.library.remove_cell(name)
    }

    pub fn rename_cell(&mut self, name: &str, new_name: &str) -> bool {
        let Some(mut moved) = self.library.cells.remove(name) else {
            return false;
        };
        moved.name = new_name.to_owned();
        self.library.cells.insert(new_name.to_owned(), moved);
        true
    }

    pub fn add_view(&mut self, cell: &str, view: View) -> Option<()> {
        self.library.get_cell_mut(cell)?.add_view(view);
        Some(())
    }

    pub fn remove_view(&mut self, cell: &str, view: &str) -> Option<bool> {
        Some(self.library.get_cell_mut(cell)?.remove_view(view))
    }

    pub fn clear_views(&mut self, cell: &str) {
        if let Some(cell) = self.library.get_cell_mut(cell) {
            cell.views.clear();
        }
    }

    pub fn rename_view(&mut self, cell: &str, view: &str, new_name: &str) -> Option<bool> {
        let cell = self.library.get_cell_mut(cell)?;
        let Some(mut moved) = cell.views.remove(view) else {
            return Some(false);
        };
        moved.name = new_name.to_owned();
        cell.views.insert(new_name.to_owned(), moved);
        Some(true)
    }

    pub fn ensure_cell_view(
        &mut self,
        cell_name: &str,
        view_name: &str,
        view_type: ViewType,
        description: &str,
    ) {
        if self.library.get_cell(cell_name).is_none() {
            let mut cell = Cell::new(cell_name);
            cell.description = description.to_owned();
            cell.add_view(View::new(view_name, view_type));
            self.library.add_cell(cell);
        } else if let Some(cell) = self.library.get_cell_mut(cell_name)
            && cell.get_view(view_name).is_none()
        {
            cell.add_view(View::new(view_name, view_type));
        }
    }

    fn view_mut(&mut self, cell: &str, view: &str) -> Option<&mut View> {
        self.library.get_cell_mut(cell)?.get_view_mut(view)
    }

    /// An engineering edit's dirty marker within the already advanced scope.
    /// Save-acceptance runtime markers use the catalog's separate runtime API.
    pub fn mark_view_modified(&mut self, cell: &str, view: &str) {
        if let Some(view) = self.view_mut(cell, view) {
            view.modified = true;
        }
    }

    pub fn replace_view_metadata(
        &mut self,
        cell: &str,
        view: &str,
        metadata: HashMap<String, String>,
    ) {
        if let Some(view) = self.view_mut(cell, view) {
            view.metadata = metadata;
        }
    }

    pub fn restore_symbol_snapshot(
        &mut self,
        cell: &str,
        view: &str,
        snapshot: &SymbolDocumentSnapshot,
    ) -> Option<()> {
        restore_symbol_snapshot_in_view(self.view_mut(cell, view)?, snapshot);
        Some(())
    }

    pub fn store_symbol_document(
        &mut self,
        cell: &str,
        view: &str,
        document: &SymbolDocument,
    ) -> Option<Result<(), String>> {
        let view = self.view_mut(cell, view)?;
        Some(document.store_in_view(view).map(|()| {
            view.metadata.remove("generated");
            view.metadata.remove("ports");
        }))
    }

    pub fn store_symbol_editor_bundle(
        &mut self,
        cell: &str,
        view: &str,
        bundle: EncodedSymbolEditorBundle,
    ) -> Option<()> {
        bundle.store_in_view(self.view_mut(cell, view)?);
        Some(())
    }

    pub fn construct_symbol(
        &mut self,
        definition: &ModelBoundSymbolDefinition,
    ) -> Result<(), SymbolDefinitionError> {
        prepare_symbol_construction(definition, self.library)?.commit(self.library)
    }

    pub fn commit_symbol_construction(
        &mut self,
        plan: SymbolConstructionPlan,
    ) -> Result<(), SymbolDefinitionError> {
        plan.commit(self.library)
    }
}
