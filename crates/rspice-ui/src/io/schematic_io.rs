//! Schematic File I/O
//!
//! Save and load schematic files with versioning support.
//! Uses JSON format with schema versioning for forward compatibility.
//!
//! # File Format
//!
//! RSpice schematic files (`.rsch`) are JSON documents containing:
//! - Version information for migration support
//! - All components, wires, junctions, and net labels
//! - Grid settings and other metadata
//!
//! # Features
//!
//! - **Atomic Saves**: Write to temp file, then rename to prevent corruption
//! - **Automatic Backup**: Creates `.bak` file before overwriting
//! - **Version Migration**: Handles older file format versions gracefully
//! - **Native Dialogs**: Uses rfd for platform-native file pickers

use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::state::SchematicState;

// =============================================================================
// File Format Version
// =============================================================================

/// Schematic file format version
///
/// Follow semantic versioning:
/// - Major: Breaking changes that require migration
/// - Minor: New features, backward compatible
/// - Patch: Bug fixes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchematicVersion {
    /// Major version (breaking changes)
    pub major: u32,
    /// Minor version (new features)
    pub minor: u32,
    /// Patch version (bug fixes)
    pub patch: u32,
}

impl Default for SchematicVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl SchematicVersion {
    /// Current schematic format version
    pub const fn current() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }

    /// Check if this version is compatible with the current format
    ///
    /// We can read files from the same major version, but not
    /// files from future major versions.
    pub fn is_compatible(&self) -> bool {
        self.major <= Self::current().major
    }

    /// Check if migration is needed (older minor/patch version)
    pub fn needs_migration(&self) -> bool {
        self.major < Self::current().major
            || self.minor < Self::current().minor
            || self.patch < Self::current().patch
    }
}

impl std::fmt::Display for SchematicVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// =============================================================================
// Schematic File
// =============================================================================

/// Complete schematic file structure
///
/// This is the top-level structure serialized to/from `.rsch` files.
/// Contains version info for migration and the actual schematic data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicFile {
    /// File format version
    pub version: SchematicVersion,
    /// Schematic data (components, wires, etc.)
    pub schematic: SchematicState,
    /// Optional file metadata
    #[serde(default)]
    pub metadata: SchematicMetadata,
}

/// Optional metadata for schematic files
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchematicMetadata {
    /// Human-readable title
    pub title: Option<String>,
    /// Description/notes
    pub description: Option<String>,
    /// Author name
    pub author: Option<String>,
    /// Creation timestamp (Unix epoch seconds)
    pub created_at: Option<u64>,
    /// Last modified timestamp
    pub modified_at: Option<u64>,
}

impl SchematicFile {
    /// Create a new schematic file from state
    pub fn new(schematic: SchematicState) -> Self {
        Self {
            version: SchematicVersion::current(),
            schematic,
            metadata: SchematicMetadata::default(),
        }
    }

    /// Create with metadata
    pub fn with_metadata(schematic: SchematicState, title: impl Into<String>) -> Self {
        let now = crate::common::time_compat::unix_epoch().as_secs();

        Self {
            version: SchematicVersion::current(),
            schematic,
            metadata: SchematicMetadata {
                title: Some(title.into()),
                created_at: Some(now),
                modified_at: Some(now),
                ..Default::default()
            },
        }
    }

    /// Validate the file for loading
    pub fn validate(&self) -> Result<(), SchematicIoError> {
        if !self.version.is_compatible() {
            return Err(SchematicIoError::IncompatibleVersion {
                file_version: self.version.to_string(),
                app_version: SchematicVersion::current().to_string(),
            });
        }
        Ok(())
    }
}

// =============================================================================
// Error Types
// =============================================================================

/// Schematic I/O errors
#[derive(Debug, Clone)]
pub enum SchematicIoError {
    /// User cancelled the file dialog
    Cancelled,
    /// File not found
    NotFound(PathBuf),
    /// Permission denied
    PermissionDenied(PathBuf),
    /// File format version is incompatible
    IncompatibleVersion {
        file_version: String,
        app_version: String,
    },
    /// JSON parse error
    ParseError(String),
    /// JSON serialization error
    SerializeError(String),
    /// Generic I/O error
    Io(String),
}

impl std::fmt::Display for SchematicIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchematicIoError::Cancelled => write!(f, "Operation cancelled"),
            SchematicIoError::NotFound(p) => write!(f, "File not found: {}", p.display()),
            SchematicIoError::PermissionDenied(p) => {
                write!(f, "Permission denied: {}", p.display())
            }
            SchematicIoError::IncompatibleVersion {
                file_version,
                app_version,
            } => {
                write!(
                    f,
                    "File version {} is not compatible with app version {}",
                    file_version, app_version
                )
            }
            SchematicIoError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            SchematicIoError::SerializeError(msg) => write!(f, "Serialize error: {}", msg),
            SchematicIoError::Io(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for SchematicIoError {}

impl From<std::io::Error> for SchematicIoError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => SchematicIoError::NotFound(PathBuf::new()),
            std::io::ErrorKind::PermissionDenied => {
                SchematicIoError::PermissionDenied(PathBuf::new())
            }
            _ => SchematicIoError::Io(e.to_string()),
        }
    }
}

// =============================================================================
// File Filters
// =============================================================================

/// Standard file filter for RSpice schematic files
pub const SCHEMATIC_FILTER: (&str, &[&str]) = ("RSpice Schematic", &["rsch", "json"]);

/// All supported file filters for open dialog
pub const OPEN_FILTERS: &[(&str, &[&str])] = &[
    ("RSpice Schematic", &["rsch", "json"]),
    ("All Files", &["*"]),
];

// =============================================================================
// Native File Dialog Functions
// =============================================================================

/// Show an "Open File" dialog and return the selected path
///
/// Uses the platform's native file picker dialog.
/// Returns `Err(Cancelled)` if user cancels.
#[cfg(not(target_arch = "wasm32"))]
pub fn show_open_dialog() -> Result<PathBuf, SchematicIoError> {
    let handle = rfd::FileDialog::new()
        .add_filter(SCHEMATIC_FILTER.0, SCHEMATIC_FILTER.1)
        .add_filter("All Files", &["*"])
        .set_title("Open Schematic")
        .pick_file();

    handle.ok_or(SchematicIoError::Cancelled)
}

/// Show a "Save As" dialog and return the selected path
///
/// Uses the platform's native file picker dialog.
/// Returns `Err(Cancelled)` if user cancels.
#[cfg(not(target_arch = "wasm32"))]
pub fn show_save_dialog(default_name: Option<&str>) -> Result<PathBuf, SchematicIoError> {
    let mut dialog = rfd::FileDialog::new()
        .add_filter(SCHEMATIC_FILTER.0, SCHEMATIC_FILTER.1)
        .set_title("Save Schematic");

    if let Some(name) = default_name {
        dialog = dialog.set_file_name(name);
    } else {
        dialog = dialog.set_file_name("untitled.rsch");
    }

    let mut path = dialog.save_file().ok_or(SchematicIoError::Cancelled)?;

    // Ensure .rsch extension
    let has_rsch_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rsch"));
    if !has_rsch_extension {
        path.set_extension("rsch");
    }

    Ok(path)
}

// Web stubs
#[cfg(target_arch = "wasm32")]
pub fn show_open_dialog() -> Result<PathBuf, SchematicIoError> {
    Err(SchematicIoError::Io(
        "File dialogs not supported on web".to_string(),
    ))
}

#[cfg(target_arch = "wasm32")]
pub fn show_save_dialog(_default_name: Option<&str>) -> Result<PathBuf, SchematicIoError> {
    Err(SchematicIoError::Io(
        "File dialogs not supported on web".to_string(),
    ))
}

// =============================================================================
// Save/Load Functions
// =============================================================================

/// Save schematic to file
///
/// Uses atomic write pattern:
/// 1. Create backup of existing file (if any)
/// 2. Write to temporary file
/// 3. Rename temp file to target (atomic on most filesystems)
pub fn save_schematic(schematic: &SchematicState, path: &Path) -> Result<(), SchematicIoError> {
    // Create the file structure
    let mut file = SchematicFile::new(schematic.clone());

    // Update metadata timestamp
    let now = crate::common::time_compat::unix_epoch().as_secs();
    file.metadata.modified_at = Some(now);

    // Extract title from filename
    if let Some(stem) = path.file_stem() {
        file.metadata.title = Some(stem.to_string_lossy().to_string());
    }

    save_schematic_file(&file, path)
}

/// Save a complete schematic file structure
///
/// This is the low-level save function used by `save_schematic`.
pub fn save_schematic_file(file: &SchematicFile, path: &Path) -> Result<(), SchematicIoError> {
    // Create backup if file exists
    if path.exists() {
        let backup_path = path.with_extension("rsch.bak");
        if let Err(e) = fs::copy(path, &backup_path) {
            log::warn!("Failed to create backup: {}", e);
            // Continue anyway - backup failure shouldn't block save
        }
    }

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write to temporary file first
    let temp_path = path.with_extension("rsch.tmp");
    let temp_file = File::create(&temp_path)?;
    let mut writer = BufWriter::new(temp_file);

    serde_json::to_writer_pretty(&mut writer, file)
        .map_err(|e| SchematicIoError::SerializeError(e.to_string()))?;

    writer.flush()?;

    // Atomic rename
    fs::rename(&temp_path, path)?;

    log::info!("Saved schematic to: {}", path.display());
    Ok(())
}

/// Load schematic from file
///
/// Validates version compatibility and recalculates runtime state.
pub fn load_schematic(path: &Path) -> Result<SchematicState, SchematicIoError> {
    let file = load_schematic_file(path)?;
    file.validate()?;

    let mut schematic = file.schematic;

    // Recalculate runtime state (IDs, counters, etc.)
    schematic.recalculate_runtime_state();

    // Set the file path for subsequent saves
    schematic.current_file = Some(path.to_path_buf());

    // Mark for fit-to-view and history reset
    schematic.needs_fit = true;
    schematic.needs_history_reset = true;

    // Clear dirty flag since we just loaded
    schematic.is_dirty = false;

    log::info!("Loaded schematic from: {}", path.display());
    Ok(schematic)
}

/// Load a complete schematic file structure
///
/// This is the low-level load function used by `load_schematic`.
pub fn load_schematic_file(path: &Path) -> Result<SchematicFile, SchematicIoError> {
    if !path.exists() {
        return Err(SchematicIoError::NotFound(path.to_path_buf()));
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).map_err(|e| SchematicIoError::ParseError(e.to_string()))
}

// =============================================================================
// Tests
// =============================================================================
