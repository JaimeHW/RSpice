//! Project library catalog and its persisted selection and presentation fields.

use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};
use std::collections::HashMap;

use rspice_design::library::{Cell, Library, LibraryCatalog, View};
use rspice_design_model::cell_view::CellViewRef;

mod bootstrap;

#[cfg(test)]
mod tests;

/// The project format's library record over one governed catalog.
#[derive(Debug, Clone, Default)]
pub struct ProjectLibraries {
    catalog: LibraryCatalog,
    pub selected_library: Option<String>,
    pub selected_cell: Option<String>,
    pub selected_view: Option<String>,
    pub filter_text: String,
    pub show_read_only: bool,
}

// Preserve the existing project JSON and session RON layout and field order.
// Serialization borrows the catalog; it does not clone engineering content.
impl Serialize for ProjectLibraries {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut record = serializer.serialize_struct("LibraryManager", 7)?;
        record.serialize_field("libraries", self.catalog.libraries())?;
        record.serialize_field("selected_library", &self.selected_library)?;
        record.serialize_field("selected_cell", &self.selected_cell)?;
        record.serialize_field("selected_view", &self.selected_view)?;
        record.serialize_field("filter_text", &self.filter_text)?;
        record.serialize_field("show_read_only", &self.show_read_only)?;
        record.serialize_field("revision", &self.catalog.revision())?;
        record.end()
    }
}

impl<'de> Deserialize<'de> for ProjectLibraries {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename = "LibraryManager")]
        struct Record {
            libraries: HashMap<String, Library>,
            selected_library: Option<String>,
            selected_cell: Option<String>,
            selected_view: Option<String>,
            filter_text: String,
            show_read_only: bool,
            #[serde(default)]
            revision: u64,
        }
        let record = Record::deserialize(deserializer)?;
        Ok(Self {
            catalog: LibraryCatalog::from_persisted(record.libraries, record.revision),
            selected_library: record.selected_library,
            selected_cell: record.selected_cell,
            selected_view: record.selected_view,
            filter_text: record.filter_text,
            show_read_only: record.show_read_only,
        })
    }
}

impl ProjectLibraries {
    pub fn catalog(&self) -> &LibraryCatalog {
        &self.catalog
    }

    /// Create a new library manager
    pub fn new() -> Self {
        Self {
            show_read_only: true,
            ..Default::default()
        }
    }

    /// Content revision, for caches over library listings.
    pub fn revision(&self) -> u64 {
        self.catalog.revision()
    }

    /// Add a library
    pub fn add_library(&mut self, library: Library) {
        self.catalog.add_library(library);
    }

    /// Remove a library
    pub fn remove_library(&mut self, name: &str) -> Option<Library> {
        self.catalog.remove_library(name)
    }

    /// Get a library by name
    pub fn get_library(&self, name: &str) -> Option<&Library> {
        self.catalog.get_library(name)
    }

    /// Iterate libraries with their canonical map keys.
    pub fn libraries_by_key(&self) -> impl Iterator<Item = (&str, &Library)> {
        self.catalog.libraries_by_key()
    }

    pub fn edit_library(&mut self, name: &str) -> Option<rspice_design::library::LibraryEdit<'_>> {
        self.catalog.edit_library(name)
    }

    #[cfg(feature = "library-test-fixtures")]
    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut Library> {
        self.catalog.library_mut_for_test(name)
    }

    /// Get libraries sorted by name
    pub fn libraries_sorted(&self) -> Vec<&Library> {
        let mut libraries = self.catalog.libraries_sorted();
        if !self.show_read_only {
            libraries.retain(|library| !library.read_only);
        }
        libraries
    }

    /// Get the currently selected library
    pub fn current_library(&self) -> Option<&Library> {
        self.selected_library
            .as_ref()
            .and_then(|name| self.catalog.get_library(name))
    }

    /// Get the currently selected cell
    pub fn current_cell(&self) -> Option<&Cell> {
        self.current_library().and_then(|lib| {
            self.selected_cell
                .as_ref()
                .and_then(|name| lib.cells.get(name))
        })
    }

    /// Get the currently selected view
    pub fn current_view(&self) -> Option<&View> {
        self.current_cell().and_then(|cell| {
            self.selected_view
                .as_ref()
                .and_then(|name| cell.views.get(name))
        })
    }

    /// Select a library
    pub fn select_library(&mut self, name: &str) {
        if self.catalog.get_library(name).is_some() {
            self.selected_library = Some(name.to_string());
            self.selected_cell = None;
            self.selected_view = None;
        }
    }

    /// Select a cell
    pub fn select_cell(&mut self, library: &str, cell: &str) {
        if let Some(lib) = self.catalog.get_library(library)
            && lib.cells.contains_key(cell)
        {
            self.selected_library = Some(library.to_string());
            self.selected_cell = Some(cell.to_string());
            self.selected_view = None;
        }
    }

    /// Select a view
    pub fn select_view(&mut self, library: &str, cell: &str, view: &str) {
        if let Some(lib) = self.catalog.get_library(library)
            && let Some(c) = lib.cells.get(cell)
            && c.views.contains_key(view)
        {
            self.selected_library = Some(library.to_string());
            self.selected_cell = Some(cell.to_string());
            self.selected_view = Some(view.to_string());
        }
    }

    /// Get the LCV path string (library/cell/view)
    pub fn selected_lcv_path(&self) -> Option<String> {
        match (
            &self.selected_library,
            &self.selected_cell,
            &self.selected_view,
        ) {
            (Some(lib), Some(cell), Some(view)) => Some(format!("{}/{}/{}", lib, cell, view)),
            (Some(lib), Some(cell), None) => Some(format!("{}/{}", lib, cell)),
            (Some(lib), None, None) => Some(lib.clone()),
            _ => None,
        }
    }

    /// Count total libraries
    pub fn library_count(&self) -> usize {
        self.catalog.library_count()
    }

    /// Count total cells across all libraries
    pub fn total_cell_count(&self) -> usize {
        self.catalog.total_cell_count()
    }

    /// Count total views across all libraries
    pub fn total_view_count(&self) -> usize {
        self.catalog.total_view_count()
    }

    /// Clear all libraries
    pub fn clear(&mut self) {
        self.catalog.clear();
        self.selected_library = None;
        self.selected_cell = None;
        self.selected_view = None;
    }

    /// Replace the complete governed library catalog from a validated
    /// publication snapshot while retaining this session's presentation
    /// preferences and advancing, never rewinding, its content revision.
    pub fn replace_catalog_from_snapshot(&mut self, snapshot: &Self) -> Result<u64, String> {
        let revision = self
            .catalog
            .replace_catalog_from_snapshot(&snapshot.catalog)?;
        self.selected_library = snapshot.selected_library.clone();
        self.selected_cell = snapshot.selected_cell.clone();
        self.selected_view = snapshot.selected_view.clone();
        Ok(revision)
    }

    /// Clear runtime dirty markers without claiming an engineering catalog
    /// mutation. Save acceptance must not advance the persisted content
    /// revision merely because editor presentation state became clean.
    pub fn mark_all_views_clean_runtime(&mut self) {
        self.catalog.mark_all_views_clean_runtime();
    }

    /// Project a known-clean or unverified registry into runtime markers
    /// without changing the engineering catalog revision.
    pub fn set_all_views_modified_runtime(&mut self, modified: bool) {
        self.catalog.set_all_views_modified_runtime(modified);
    }

    /// Clear one view's runtime dirty marker without advancing the governed
    /// library revision.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn mark_view_clean_runtime(&mut self, library: &str, cell: &str, view: &str) -> bool {
        self.catalog.mark_view_clean_runtime(library, cell, view)
    }

    /// Project one document-registry dirty bit into presentation state
    /// without claiming a library content mutation.
    pub fn set_view_modified_runtime(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        modified: bool,
    ) -> bool {
        self.catalog
            .set_view_modified_runtime(library, cell, view, modified)
    }

    /// Strip runtime-only view presentation from a serialization clone while
    /// preserving the exact governed catalog revision.
    pub fn sanitize_views_for_persistence(&mut self) {
        self.catalog.sanitize_views_for_persistence();
    }

    /// Overlay one cell-view document from an already-snapshotted working
    /// catalog without manufacturing a new revision while assembling a
    /// partial-save candidate.
    ///
    /// The source snapshot is the authority for the observed revision. Parent
    /// library/cell records are copied only when the document does not yet
    /// exist in the accepted target. A generated symbol owned by a schematic
    /// document is synchronized as part of the same atomic overlay.
    pub fn overlay_cell_view_document_from_snapshot(
        &mut self,
        source: &Self,
        library_name: &str,
        cell_name: &str,
        view_name: &str,
    ) -> Result<(), String> {
        self.catalog.overlay_cell_view_document_from_snapshot(
            &source.catalog,
            library_name,
            cell_name,
            view_name,
        )
    }

    /// Copy this library/cell structure while retaining matching view documents
    /// from the supplied content, so project setup does not publish view drafts.
    pub fn with_document_content_from(&self, content: &Self) -> Self {
        let mut merged = self.clone();
        let cells = merged
            .libraries_by_key()
            .flat_map(|(library_key, library)| {
                library
                    .cells
                    .keys()
                    .map(move |cell_key| (library_key.to_owned(), cell_key.to_owned()))
            })
            .collect::<Vec<_>>();
        for (library, cell) in cells {
            if let Some(mut library) = merged.edit_library(&library) {
                library.clear_views(&cell);
            }
        }
        let references = content
            .libraries_by_key()
            .flat_map(|(library_key, library)| {
                library.cells.iter().flat_map(move |(cell_key, cell)| {
                    cell.views
                        .keys()
                        .map(move |view_key| CellViewRef::new(library_key, cell_key, view_key))
                })
            })
            .collect::<Vec<_>>();
        for reference in references {
            let Some(view) = content
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
                .cloned()
            else {
                continue;
            };
            if let Some(mut library) = merged.edit_library(&reference.library) {
                library.add_view(&reference.cell, view);
            }
        }
        merged
    }
}
