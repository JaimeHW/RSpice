//! A library's parts in the project's stores, as the files of a folder of
//! its own, and back.
//!
//! Nothing is converted: the catalog's records, the drawings, the layouts
//! and the source bundles move between a store and a file in the form each
//! already takes in the project file. Sheet catalogs stay in the project
//! with the rest of Design Management, whose variants and annotation name
//! their sheets.

use std::collections::BTreeMap;

use rspice_design::library::folder::{
    CellFile, CellViewFile, EncodedFile, LibraryFile, LibraryFileError, LibraryManifest,
    encode_library_file,
};
use rspice_design::project_sources::ProjectSourceOwner;
use rspice_design_model::cell_view::CellViewRef;
use rspice_project_contract::ProjectLibraries;
use uuid::Uuid;

use super::ProjectWorkspace;

/// Everything a library folder holds, as its files' contents. Each file's
/// revision is stamped when it is encoded.
#[derive(Debug, Clone)]
pub struct LibraryImage {
    pub manifest: LibraryManifest,
    pub cells: BTreeMap<String, CellFile>,
    pub cellviews: BTreeMap<LibraryFile, CellViewFile>,
}

impl LibraryImage {
    /// Every file the folder holds: the manifest, then each cell's own file
    /// and its cellviews.
    pub fn files(&self) -> Vec<LibraryFile> {
        std::iter::once(LibraryFile::Manifest)
            .chain(self.cells.keys().map(LibraryFile::cell))
            .chain(self.cellviews.keys().cloned())
            .collect()
    }

    pub fn name(&self) -> &str {
        &self.manifest.name
    }

    /// The exact bytes of `file` at `revision`; none for a file the folder
    /// does not hold.
    pub fn encode(
        &self,
        file: &LibraryFile,
        revision: u64,
    ) -> Option<Result<EncodedFile, LibraryFileError>> {
        Some(match file {
            LibraryFile::Manifest => {
                let mut manifest = self.manifest.clone();
                manifest.revision = revision;
                encode_library_file(&manifest)
            }
            LibraryFile::Cell { cell } => {
                let mut record = self.cells.get(cell)?.clone();
                record.revision = revision;
                encode_library_file(&record)
            }
            LibraryFile::CellView { .. } => {
                let mut record = self.cellviews.get(file)?.clone();
                record.revision = revision;
                encode_library_file(&record)
            }
        })
    }

    /// The revision each file was read at, as its own record states it.
    pub fn revision(&self, file: &LibraryFile) -> Option<u64> {
        match file {
            LibraryFile::Manifest => Some(self.manifest.revision),
            LibraryFile::Cell { cell } => self.cells.get(cell).map(|record| record.revision),
            LibraryFile::CellView { .. } => self.cellviews.get(file).map(|record| record.revision),
        }
    }
}

impl ProjectWorkspace {
    /// The parts of `library` as the files of its folder, from the catalog
    /// and the stores as they stand. Every revision is zero.
    pub fn library_image(
        &self,
        libraries: &ProjectLibraries,
        name: &str,
        id: Uuid,
    ) -> Option<LibraryImage> {
        let library = libraries.get_library(name)?;
        let mut cells = BTreeMap::new();
        let mut cellviews = BTreeMap::new();
        for cell in library.cells.values() {
            cells.insert(cell.name.clone(), CellFile::of_cell(cell, 0));
            for view in cell.views.values() {
                let reference = CellViewRef::new(name, &cell.name, &view.name);
                let mut file = CellViewFile::of_view(view, 0);
                file.drawing = self.schematic_buffers.get(&reference.key()).cloned();
                file.layout = self.physical_layout_document(&reference).cloned();
                file.source = self
                    .project_sources
                    .bundle_for_owner(&ProjectSourceOwner::cell_view(reference))
                    .cloned();
                cellviews.insert(LibraryFile::cell_view(&cell.name, view), file);
            }
        }
        Some(LibraryImage {
            manifest: LibraryManifest::of_library(library, id, 0),
            cells,
            cellviews,
        })
    }

    /// Take every part of `library` out of the stores: its drawings,
    /// layouts and source bundles. The catalog record is the caller's.
    pub fn remove_library_parts(&mut self, libraries: &ProjectLibraries, name: &str) {
        let Some(library) = libraries.get_library(name) else {
            return;
        };
        let cells = library.cells.keys().cloned().collect::<Vec<_>>();
        let prefix = format!("{name}/");
        self.schematic_buffers
            .retain(|key, _| !key.starts_with(&prefix));
        let layouts = self
            .physical_layout_documents
            .keys()
            .filter(|key| key.starts_with(&prefix))
            .cloned()
            .collect::<Vec<_>>();
        if !layouts.is_empty() {
            let documents = &mut *self.physical_layout_documents;
            for key in layouts {
                documents.remove(&key);
            }
        }
        let mut released = false;
        for cell in &cells {
            released |= !self
                .project_sources
                .remove_cell_view_bundles(name, cell, None)
                .is_empty();
        }
        if released {
            self.project_sources_dirty = true;
        }
    }

    /// Take one cellview's parts out of the stores.
    pub fn remove_cell_view_parts(&mut self, reference: &CellViewRef) {
        self.schematic_buffers.remove(&reference.key());
        if self
            .physical_layout_documents
            .contains_key(&reference.key())
        {
            self.physical_layout_documents.remove(&reference.key());
        }
        if !self
            .project_sources
            .remove_cell_view_bundles(&reference.library, &reference.cell, Some(&reference.view))
            .is_empty()
        {
            self.project_sources_dirty = true;
        }
    }

    /// Put a library's files into the catalog and the stores. A layout
    /// drawn for another technology than `technology_matches` accepts is
    /// left out; returns the cellviews left out that way.
    pub fn insert_library_image(
        &mut self,
        libraries: &mut ProjectLibraries,
        image: &LibraryImage,
        read_only: bool,
        technology_matches: &dyn Fn(
            &rspice_design::physical_layout::PhysicalLayoutDocument,
        ) -> bool,
    ) -> Result<Vec<LibraryFile>, String> {
        let name = image.name().to_owned();
        let mut library = image.manifest.library(read_only);
        for record in image.cells.values() {
            library.add_cell(record.cell());
        }
        for (file, record) in &image.cellviews {
            let Some(cell) = file.cell_name() else {
                continue;
            };
            library
                .cells
                .entry(cell.to_owned())
                .or_insert_with(|| rspice_design::library::Cell::new(cell))
                .add_view(record.view.view());
        }
        if libraries.get_library(&name).is_some() {
            self.remove_library_parts(libraries, &name);
            libraries.remove_library(&name);
        }
        libraries.add_library(library);
        let mut left_out = Vec::new();
        for (file, record) in &image.cellviews {
            if !self.insert_cell_view_parts(&name, file, record, technology_matches)? {
                left_out.push(file.clone());
            }
        }
        Ok(left_out)
    }

    /// Put one cellview file's parts into the stores, replacing what they
    /// held for it. Returns false when its layout was left out for being
    /// drawn for another technology.
    pub fn insert_cell_view_parts(
        &mut self,
        library: &str,
        file: &LibraryFile,
        record: &CellViewFile,
        technology_matches: &dyn Fn(
            &rspice_design::physical_layout::PhysicalLayoutDocument,
        ) -> bool,
    ) -> Result<bool, String> {
        let (Some(cell), Some(view)) = (file.cell_name(), file.view_name()) else {
            return Ok(true);
        };
        let reference = CellViewRef::new(library, cell, view);
        self.remove_cell_view_parts(&reference);
        if let Some(drawing) = &record.drawing {
            self.schematic_buffers
                .insert(reference.key(), drawing.clone());
        }
        let mut kept = true;
        if let Some(layout) = &record.layout {
            if technology_matches(layout) {
                self.physical_layout_documents
                    .insert(reference.key(), layout.clone());
            } else {
                kept = false;
            }
        }
        if let Some(source) = &record.source {
            self.project_sources.remove_bundle(source.id());
            self.project_sources
                .insert_bundle(source.clone())
                .map_err(|error| {
                    format!("The source of {} cannot be read: {error}", reference.key())
                })?;
            self.project_sources_dirty = true;
        }
        Ok(kept)
    }
}
