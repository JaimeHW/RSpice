//! The libraries a project names by folder instead of holding.
//!
//! Like a `cds.lib` or `lib.defs` entry, a reference names the library, the
//! folder it lives in and how the project holds it. Unlike those, it also
//! records the revision and digest of each file of the folder the project
//! last saved with, so a file that moved on since is named rather than read
//! silently.

use std::collections::BTreeMap;

use rspice_design::library::folder::{FileRevision, LibraryFile, LibraryMode, LibraryPath};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One library the project reads from a folder of its own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryReference {
    pub name: String,
    /// The folder, as written: relative to the project file's folder, under
    /// a named root, or absolute.
    pub path: LibraryPath,
    /// The identity the folder's manifest carries.
    pub id: Uuid,
    #[serde(default)]
    pub mode: LibraryMode,
    /// Each file's revision and digest as the project last saved with it.
    #[serde(default)]
    pub files: BTreeMap<LibraryFile, FileRevision>,
}

impl LibraryReference {
    pub fn new(name: impl Into<String>, path: LibraryPath, id: Uuid) -> Self {
        Self {
            name: name.into(),
            path,
            id,
            mode: LibraryMode::ReadOnly,
            files: BTreeMap::new(),
        }
    }

    /// The revision the project last saved `file` with.
    pub fn saved_revision(&self, file: &LibraryFile) -> Option<FileRevision> {
        self.files.get(file).copied()
    }
}
