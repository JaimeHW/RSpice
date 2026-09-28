//! Library records and their governed content revision.

use std::collections::HashMap;

use super::{Library, LibraryEdit};

/// Library membership and content, independent of browser selection and filters.
#[derive(Debug, Clone, Default)]
pub struct LibraryCatalog {
    libraries: HashMap<String, Library>,
    revision: u64,
}

impl LibraryCatalog {
    /// Restore a persisted catalog candidate. The project loader validates its
    /// aggregate identities before publishing it as live state.
    pub fn from_persisted(libraries: HashMap<String, Library>, revision: u64) -> Self {
        Self {
            libraries,
            revision,
        }
    }

    /// Borrow the stored map for persistence without exposing mutable content.
    pub fn libraries(&self) -> &HashMap<String, Library> {
        &self.libraries
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn add_library(&mut self, library: Library) {
        self.libraries.insert(library.name.clone(), library);
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn remove_library(&mut self, name: &str) -> Option<Library> {
        self.revision = self.revision.wrapping_add(1);
        self.libraries.remove(name)
    }

    pub fn get_library(&self, name: &str) -> Option<&Library> {
        self.libraries.get(name)
    }

    pub fn libraries_by_key(&self) -> impl Iterator<Item = (&str, &Library)> {
        self.libraries
            .iter()
            .map(|(key, library)| (key.as_str(), library))
    }

    /// Pessimistically advances revision, including a missing-key lookup.
    pub fn edit_library(&mut self, name: &str) -> Option<LibraryEdit<'_>> {
        self.library_for_edit(name).map(LibraryEdit::new)
    }

    fn library_for_edit(&mut self, name: &str) -> Option<&mut Library> {
        self.revision = self.revision.wrapping_add(1);
        self.libraries.get_mut(name)
    }

    #[cfg(any(test, feature = "schematic-test-fixtures"))]
    pub fn library_mut_for_test(&mut self, name: &str) -> Option<&mut Library> {
        self.library_for_edit(name)
    }

    pub fn libraries_sorted(&self) -> Vec<&Library> {
        let mut libs: Vec<_> = self.libraries.values().collect();
        libs.sort_by(|a, b| a.name.cmp(&b.name));
        libs
    }

    pub fn library_count(&self) -> usize {
        self.libraries.len()
    }

    pub fn total_cell_count(&self) -> usize {
        self.libraries.values().map(|l| l.cell_count()).sum()
    }

    pub fn total_view_count(&self) -> usize {
        self.libraries.values().map(|l| l.total_view_count()).sum()
    }

    pub fn clear(&mut self) {
        self.libraries.clear();
        self.revision = self.revision.wrapping_add(1);
    }

    /// Replace a publication candidate with a checked increment of this catalog
    /// revision. The caller retains project permission and publication checks.
    pub fn replace_catalog_from_snapshot(&mut self, snapshot: &Self) -> Result<u64, String> {
        let revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| "project library content revision is exhausted".to_owned())?;
        self.libraries = snapshot.libraries.clone();
        self.revision = revision;
        Ok(revision)
    }

    pub fn mark_all_views_clean_runtime(&mut self) {
        self.set_all_views_modified_runtime(false);
    }

    /// Project runtime dirty state without changing content revision.
    pub fn set_all_views_modified_runtime(&mut self, modified: bool) {
        for library in self.libraries.values_mut() {
            for cell in library.cells.values_mut() {
                for view in cell.views.values_mut() {
                    view.modified = modified;
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn mark_view_clean_runtime(&mut self, library: &str, cell: &str, view: &str) -> bool {
        self.set_view_modified_runtime(library, cell, view, false)
    }

    /// Project one document dirty bit without claiming a catalog edit.
    pub fn set_view_modified_runtime(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        modified: bool,
    ) -> bool {
        let Some(view) = self
            .libraries
            .get_mut(library)
            .and_then(|library| library.cells.get_mut(cell))
            .and_then(|cell| cell.views.get_mut(view))
        else {
            return false;
        };
        view.modified = modified;
        true
    }

    /// Sanitize a persistence clone while retaining its observed revision.
    pub fn sanitize_views_for_persistence(&mut self) {
        self.mark_all_views_clean_runtime();
        for library in self.libraries.values_mut() {
            for cell in library.cells.values_mut() {
                for view in cell.views.values_mut() {
                    view.is_open = false;
                    view.file_path = None;
                    view.modified_time = None;
                }
            }
        }
    }

    /// Overlay an observed document into a partial-save candidate, including its
    /// generated symbol. Reject a snapshot older than the accepted catalog.
    pub fn overlay_cell_view_document_from_snapshot(
        &mut self,
        source: &Self,
        library_name: &str,
        cell_name: &str,
        view_name: &str,
    ) -> Result<(), String> {
        if self.revision > source.revision {
            return Err(
                "accepted library revision is newer than the working document snapshot".to_owned(),
            );
        }

        let source_library = source
            .libraries
            .get(library_name)
            .ok_or_else(|| format!("missing library '{library_name}'"))?;
        let source_cell = source_library
            .cells
            .get(cell_name)
            .ok_or_else(|| format!("missing cell '{cell_name}'"))?;
        let source_view = source_cell
            .views
            .get(view_name)
            .ok_or_else(|| format!("missing view '{view_name}'"))?
            .clone();
        let generated_symbol = view_name.eq_ignore_ascii_case("schematic").then(|| {
            source_cell
                .views
                .get("symbol")
                .filter(|view| view.metadata.contains_key("generated"))
                .cloned()
        });

        if !self.libraries.contains_key(library_name) {
            let mut library = source_library.clone();
            library.cells.clear();
            self.libraries.insert(library_name.to_owned(), library);
        }
        let target_library = self
            .libraries
            .get_mut(library_name)
            .expect("library was inserted above");
        if !target_library.cells.contains_key(cell_name) {
            let mut cell = source_cell.clone();
            cell.views.clear();
            target_library.cells.insert(cell_name.to_owned(), cell);
        }
        let target_cell = target_library
            .cells
            .get_mut(cell_name)
            .expect("cell was inserted above");
        target_cell.views.insert(view_name.to_owned(), source_view);

        if let Some(generated_symbol) = generated_symbol {
            match generated_symbol {
                Some(symbol) => {
                    target_cell.views.insert("symbol".to_owned(), symbol);
                }
                None if target_cell
                    .views
                    .get("symbol")
                    .is_some_and(|view| view.metadata.contains_key("generated")) =>
                {
                    target_cell.views.remove("symbol");
                }
                None => {}
            }
        }

        self.revision = source.revision;
        Ok(())
    }
}
