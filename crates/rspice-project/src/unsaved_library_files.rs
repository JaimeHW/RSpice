//! Edits to libraries kept in folders of their own that are not yet saved
//! to their folders.
//!
//! A project file names such a library by path alone, so a recovery
//! checkpoint of a project with unsaved edits to one would otherwise hold
//! nothing of them. It holds each edited file instead, as edited, with the
//! revision the edit started from. Restoring one puts each over what the
//! folder holds now; the file is written to its folder only by a Save.

use rspice_design::library::folder::{
    CellFile, CellViewFile, FileRevision, LibraryFile, LibraryManifest, digest_file_bytes,
};
use rspice_vocabulary::product::ContentDigest;
use serde::{Deserialize, Serialize};

use crate::{LibraryImage, ProjectFile};

/// One file of a library kept in a folder of its own, edited here and not
/// yet saved to its folder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsavedLibraryFile {
    pub library: String,
    pub file: LibraryFile,
    /// The file as this session loaded it, which the edit started from;
    /// none for a file the edit made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loaded: Option<FileRevision>,
    /// What was loaded, as this build writes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentDigest>,
    /// The file as edited; none when the edit took it out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited: Option<LibraryRecord>,
}

impl UnsavedLibraryFile {
    /// The edited file's digest, for telling two edits of one file apart.
    pub fn edited_digest(&self) -> Option<ContentDigest> {
        let edited = self.edited.as_ref()?;
        serde_json::to_vec(edited)
            .ok()
            .map(|bytes| digest_file_bytes(&bytes))
    }
}

/// What one file of a library folder holds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibraryRecord {
    Manifest(LibraryManifest),
    Cell(CellFile),
    CellView(Box<CellViewFile>),
}

impl LibraryImage {
    /// What `file` holds here; none for a file the folder would not hold.
    pub fn record(&self, file: &LibraryFile) -> Option<LibraryRecord> {
        match file {
            LibraryFile::Manifest => Some(LibraryRecord::Manifest(self.manifest.clone())),
            LibraryFile::Cell { cell } => self.cells.get(cell).cloned().map(LibraryRecord::Cell),
            LibraryFile::CellView { .. } => self
                .cellviews
                .get(file)
                .cloned()
                .map(|record| LibraryRecord::CellView(Box::new(record))),
        }
    }

    /// Make `file` hold `record`, or take it out for none. A record of
    /// another kind than the file, or a manifest taken out, is refused.
    pub fn put(&mut self, file: &LibraryFile, record: Option<LibraryRecord>) -> Result<(), String> {
        match (file, record) {
            (LibraryFile::Manifest, Some(LibraryRecord::Manifest(manifest))) => {
                self.manifest = manifest;
            }
            (LibraryFile::Cell { cell }, Some(LibraryRecord::Cell(record))) => {
                self.cells.insert(cell.clone(), record);
            }
            (LibraryFile::Cell { cell }, None) => {
                self.cells.remove(cell);
            }
            (LibraryFile::CellView { .. }, Some(LibraryRecord::CellView(record))) => {
                self.cellviews.insert(file.clone(), *record);
            }
            (LibraryFile::CellView { .. }, None) => {
                self.cellviews.remove(file);
            }
            (file, _) => return Err(format!("{file} does not hold what its edit holds.")),
        }
        Ok(())
    }
}

impl ProjectFile {
    /// Put each of `edits` to the library `name` over what this project
    /// holds of it now: what its folder held when it was read.
    pub fn put_unsaved_library_files(
        &mut self,
        name: &str,
        edits: &[UnsavedLibraryFile],
        read_only: bool,
    ) -> Result<(), String> {
        let id = self
            .libraries
            .reference(name)
            .map(|reference| reference.id)
            .ok_or_else(|| format!("Library '{name}' is not kept in a folder of its own."))?;
        let mut image = self
            .workspace
            .library_image(&self.libraries, name, id)
            .ok_or_else(|| format!("Library '{name}' has not been read."))?;
        for edit in edits {
            image.put(&edit.file, edit.edited.clone())?;
        }
        self.workspace
            .insert_library_image(&mut self.libraries, &image, read_only, &|_| true)
            .map(drop)
    }
}
