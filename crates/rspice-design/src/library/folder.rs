//! A design library kept in a folder of its own, outside any project.
//!
//! The folder holds a manifest that names the library, one folder per cell
//! holding the cell's own file, and inside it one folder per view whose file
//! is named by the view's type, as Virtuoso names `sch.oa`:
//!
//! ```text
//! analog/library.rspicelib
//! analog/opamp/cell.json
//! analog/opamp/schematic/schematic.json
//! analog/opamp/schematic/schematic.json.lock    while someone edits it
//! ```
//!
//! Every file counts its own saves and is known by the digest of its exact
//! bytes, so two projects can edit different cellviews of one library at
//! once and a save writes only the files that changed. A project names the
//! folder by a [`LibraryPath`] and records, for each file, the revision and
//! digest it last saved with.
//!
//! Nothing here reads or writes the disk: this is the format and its rules.

mod format;
mod lock;
mod path;
mod status;

pub use format::{
    CELL_FILE, CellFile, CellViewFile, EncodedFile, FileRevision, LIBRARY_FILE_FORMAT,
    LIBRARY_FILE_VERSION, LIBRARY_MANIFEST_FILE, LOCK_SUFFIX, LibraryFile, LibraryFileError,
    LibraryManifest, LibraryMode, ViewRecord, digest_file_bytes, encode_library_file,
};
pub use lock::LibraryFileLock;
pub use path::{
    HostPlatform, LibraryPath, LibraryPathChoice, LibraryPathError, LibraryPathForm, PathRoots,
    RootTable, choose_library_path,
};
pub use status::{CHANGED_SINCE_EDITING, CellViewStatus, Missing, ReferenceStatus, StatusMark};

#[cfg(test)]
mod tests;
