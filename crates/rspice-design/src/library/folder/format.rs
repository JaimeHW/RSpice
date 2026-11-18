//! The files of a library folder and their exact bytes.

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

use rspice_vocabulary::product::ContentDigest;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::library::{Cell, LIBRARY_DESCRIPTION, LIBRARY_DISPLAY_NAME, Library, View, ViewType};
use crate::physical_layout::PhysicalLayoutDocument;
use crate::project_sources::ProjectSourceBundle;
use crate::schematic::owned::Schematic;

/// The manifest's file name, at the top of the library folder.
pub const LIBRARY_MANIFEST_FILE: &str = "library.rspicelib";
/// A cell's own file, in the cell's folder.
pub const CELL_FILE: &str = "cell.json";
/// What a lock file adds to the name of the file it locks.
pub const LOCK_SUFFIX: &str = ".lock";
/// The word every library file opens with.
pub const LIBRARY_FILE_FORMAT: &str = "rspice-design-library";
/// The newest format this build reads and the one it writes.
pub const LIBRARY_FILE_VERSION: u32 = 1;

/// How a project holds a library in its own folder: read-only, as the
/// library tools open one by default, or editable, with a lock per cellview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LibraryMode {
    #[default]
    ReadOnly,
    Editable,
}

impl LibraryMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReadOnly => "Read-only",
            Self::Editable => "Editable",
        }
    }

    pub const fn is_editable(self) -> bool {
        matches!(self, Self::Editable)
    }
}

/// One file of a library folder, by what it holds. Its text form is its
/// path inside the folder, which is also how a project records it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LibraryFile {
    /// The manifest: the library's name, identity, technology and properties.
    Manifest,
    /// A cell's properties.
    Cell { cell: String },
    /// One cellview: the view record and what it holds.
    CellView {
        cell: String,
        view: String,
        view_type: ViewType,
    },
}

impl LibraryFile {
    pub fn cell(cell: impl Into<String>) -> Self {
        Self::Cell { cell: cell.into() }
    }

    pub fn cell_view(cell: impl Into<String>, view: &View) -> Self {
        Self::CellView {
            cell: cell.into(),
            view: view.name.clone(),
            view_type: view.view_type,
        }
    }

    /// The file's path inside the library folder, always with `/`.
    pub fn relative_path(&self) -> String {
        self.to_string()
    }

    /// The path of the lock file beside it.
    pub fn lock_path(&self) -> String {
        format!("{self}{LOCK_SUFFIX}")
    }

    /// The cell the file belongs to; none for the manifest.
    pub fn cell_name(&self) -> Option<&str> {
        match self {
            Self::Manifest => None,
            Self::Cell { cell } | Self::CellView { cell, .. } => Some(cell),
        }
    }

    /// The view the file holds; only a cellview file holds one.
    pub fn view_name(&self) -> Option<&str> {
        match self {
            Self::CellView { view, .. } => Some(view),
            _ => None,
        }
    }

    /// The file's name in its folder: what a view's file is called is its
    /// type's word, `schematic.json`, `veriloga.json`.
    pub fn file_name(&self) -> String {
        match self {
            Self::Manifest => LIBRARY_MANIFEST_FILE.to_owned(),
            Self::Cell { .. } => CELL_FILE.to_owned(),
            Self::CellView { view_type, .. } => format!("{}.json", view_type.display_name()),
        }
    }
}

impl fmt::Display for LibraryFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manifest => formatter.write_str(LIBRARY_MANIFEST_FILE),
            Self::Cell { cell } => write!(formatter, "{cell}/{CELL_FILE}"),
            Self::CellView { cell, view, .. } => {
                write!(formatter, "{cell}/{view}/{}", self.file_name())
            }
        }
    }
}

impl FromStr for LibraryFile {
    type Err = LibraryFileError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts = text.split('/').collect::<Vec<_>>();
        let damaged = || LibraryFileError::Damaged(format!("'{text}' names no library file"));
        match parts.as_slice() {
            [LIBRARY_MANIFEST_FILE] => Ok(Self::Manifest),
            [cell, CELL_FILE] if !cell.is_empty() => Ok(Self::cell(*cell)),
            [cell, view, file] if !cell.is_empty() && !view.is_empty() => {
                let word = file.strip_suffix(".json").ok_or_else(damaged)?;
                let view_type = ViewType::ALL
                    .into_iter()
                    .find(|candidate| candidate.display_name() == word)
                    .ok_or_else(damaged)?;
                Ok(Self::CellView {
                    cell: (*cell).to_owned(),
                    view: (*view).to_owned(),
                    view_type,
                })
            }
            _ => Err(damaged()),
        }
    }
}

impl Serialize for LibraryFile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for LibraryFile {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

/// Where one file stands in its own history: how many saves have written it
/// and the digest of its exact bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileRevision {
    pub revision: u64,
    pub digest: ContentDigest,
}

/// The library folder's manifest. The manifest names its library, as an
/// OpenAccess library's cells name theirs, so a folder that holds another
/// library is caught rather than resolved wrongly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryManifest {
    pub format: String,
    pub version: u32,
    pub revision: u64,
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub technology: String,
    /// Display name, description and every other library-level property.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

impl LibraryManifest {
    /// The manifest of `library`, at `revision`.
    pub fn of_library(library: &Library, id: Uuid, revision: u64) -> Self {
        Self {
            format: LIBRARY_FILE_FORMAT.to_owned(),
            version: LIBRARY_FILE_VERSION,
            revision,
            id,
            name: library.name.clone(),
            technology: library.technology.clone(),
            metadata: sorted(&library.metadata),
        }
    }

    /// The library record it describes, with no cells yet.
    pub fn library(&self, read_only: bool) -> Library {
        Library {
            technology: self.technology.clone(),
            read_only,
            metadata: self.metadata.clone().into_iter().collect(),
            ..Library::new(self.name.clone())
        }
    }

    pub fn display_name(&self) -> &str {
        self.metadata
            .get(LIBRARY_DISPLAY_NAME)
            .map_or(self.name.as_str(), String::as_str)
    }

    pub fn description(&self) -> &str {
        self.metadata
            .get(LIBRARY_DESCRIPTION)
            .map_or("", String::as_str)
    }
}

/// A cell's own file: its properties. Its views are files of their own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CellFile {
    pub format: String,
    pub version: u32,
    pub revision: u64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

impl CellFile {
    pub fn of_cell(cell: &Cell, revision: u64) -> Self {
        Self {
            format: LIBRARY_FILE_FORMAT.to_owned(),
            version: LIBRARY_FILE_VERSION,
            revision,
            name: cell.name.clone(),
            description: cell.description.clone(),
            metadata: sorted(&cell.metadata),
        }
    }

    /// The cell record it describes, with no views yet.
    pub fn cell(&self) -> Cell {
        Cell {
            name: self.name.clone(),
            views: HashMap::new(),
            description: self.description.clone(),
            metadata: self.metadata.clone().into_iter().collect(),
        }
    }
}

/// A view record as a file holds it: the parts that are its content, with
/// no unsaved-changes mark and no save time, which describe a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ViewRecord {
    pub name: String,
    pub view_type: ViewType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<PathBuf>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

impl ViewRecord {
    pub fn of_view(view: &View) -> Self {
        Self {
            name: view.name.clone(),
            view_type: view.view_type,
            file_path: view.file_path.clone(),
            metadata: sorted(&view.metadata),
        }
    }

    pub fn view(&self) -> View {
        View {
            name: self.name.clone(),
            view_type: self.view_type,
            file_path: self.file_path.clone(),
            modified: false,
            saved_at: None,
            metadata: self.metadata.clone().into_iter().collect(),
        }
    }
}

/// One cellview's file: the view record and what the view holds, each in
/// the form the project file gives it. A source bundle lives with the view
/// that owns it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CellViewFile {
    pub format: String,
    pub version: u32,
    pub revision: u64,
    pub view: ViewRecord,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drawing: Option<Schematic>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<PhysicalLayoutDocument>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ProjectSourceBundle>,
}

impl CellViewFile {
    /// A file holding `view` alone, at `revision`; the caller adds what the
    /// view holds.
    pub fn of_view(view: &View, revision: u64) -> Self {
        Self {
            format: LIBRARY_FILE_FORMAT.to_owned(),
            version: LIBRARY_FILE_VERSION,
            revision,
            view: ViewRecord::of_view(view),
            drawing: None,
            layout: None,
            source: None,
        }
    }
}

/// Why a library file cannot be taken as one.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LibraryFileError {
    #[error("Not a library: {0}")]
    NotALibrary(String),
    #[error("Written by a newer RSpice (library format {0})")]
    NewerFormat(u32),
    #[error("Damaged: {0}")]
    Damaged(String),
    #[error("Cannot be written: {0}")]
    Encode(String),
}

/// A file's exact bytes and their digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedFile {
    pub bytes: Vec<u8>,
    pub digest: ContentDigest,
}

/// The exact bytes of a library file: pretty JSON with every object's keys
/// in order, so one content is one byte string whatever order a map was
/// built in, and a save that changed nothing writes nothing.
pub fn encode_library_file(value: &impl Serialize) -> Result<EncodedFile, LibraryFileError> {
    let value =
        serde_json::to_value(value).map_err(|error| LibraryFileError::Encode(error.to_string()))?;
    let mut text = serde_json::to_string_pretty(&canonical(value))
        .map_err(|error| LibraryFileError::Encode(error.to_string()))?;
    text.push('\n');
    let bytes = text.into_bytes();
    let digest = digest_file_bytes(&bytes);
    Ok(EncodedFile { bytes, digest })
}

/// The digest a library file is known by: SHA-256 over its exact bytes.
pub fn digest_file_bytes(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

impl LibraryManifest {
    pub fn decode(bytes: &[u8]) -> Result<Self, LibraryFileError> {
        let manifest: Self = decode_checked(bytes)?;
        if manifest.name.trim().is_empty() {
            return Err(LibraryFileError::Damaged(
                "the manifest names no library".to_owned(),
            ));
        }
        Ok(manifest)
    }
}

impl CellFile {
    pub fn decode(bytes: &[u8]) -> Result<Self, LibraryFileError> {
        decode_checked(bytes)
    }
}

impl CellViewFile {
    pub fn decode(bytes: &[u8]) -> Result<Self, LibraryFileError> {
        decode_checked(bytes)
    }
}

/// Every library file opens with its format word and version; check those
/// before the rest, so a newer file says so rather than reading as damaged.
fn decode_checked<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, LibraryFileError> {
    #[derive(Deserialize)]
    struct Head {
        format: Option<String>,
        version: Option<u32>,
    }
    let head: Head = serde_json::from_slice(bytes)
        .map_err(|error| LibraryFileError::NotALibrary(error.to_string()))?;
    if head.format.as_deref() != Some(LIBRARY_FILE_FORMAT) {
        return Err(LibraryFileError::NotALibrary(
            "the file is not an RSpice library file".to_owned(),
        ));
    }
    match head.version {
        Some(version) if version > LIBRARY_FILE_VERSION => {
            Err(LibraryFileError::NewerFormat(version))
        }
        Some(_) => serde_json::from_slice(bytes)
            .map_err(|error| LibraryFileError::Damaged(error.to_string())),
        None => Err(LibraryFileError::Damaged(
            "the file states no format version".to_owned(),
        )),
    }
}

fn canonical(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut entries = map.into_iter().collect::<Vec<_>>();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            serde_json::Value::Object(
                entries
                    .into_iter()
                    .map(|(key, value)| (key, canonical(value)))
                    .collect(),
            )
        }
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.into_iter().map(canonical).collect())
        }
        other => other,
    }
}

fn sorted(map: &HashMap<String, String>) -> BTreeMap<String, String> {
    map.iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}
