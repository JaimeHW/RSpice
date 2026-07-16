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

use std::fs::File;
use std::io::{BufReader, Read};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{BufWriter, Write};
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
        self.major == Self::current().major
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
        "Use the browser schematic import workflow for web file selection".to_string(),
    ))
}

#[cfg(target_arch = "wasm32")]
pub fn show_save_dialog(default_name: Option<&str>) -> Result<PathBuf, SchematicIoError> {
    Ok(suggested_schematic_save_path(default_name))
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
    #[cfg(target_arch = "wasm32")]
    {
        let contents = serialize_schematic_file(file)?;
        crate::common::browser_download::download_text_file(path, &contents)
            .map_err(SchematicIoError::Io)?;
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Publish the predecessor backup transactionally as well. A failed
        // backup never truncates an earlier backup or the primary file.
        if path.exists() {
            let backup_path = path.with_extension("rsch.bak");
            if let Err(e) = crate::io::durable_file::atomic_copy(path, &backup_path) {
                log::warn!("Failed to create backup: {}", e);
                // The primary remains the authoritative predecessor until the
                // atomic publication below succeeds.
            }
        }

        crate::io::durable_file::atomic_write_with::<SchematicIoError>(path, |temp| {
            let mut writer = BufWriter::new(temp);
            serde_json::to_writer_pretty(&mut writer, file)
                .map_err(|e| SchematicIoError::SerializeError(e.to_string()))?;
            writer.flush()?;
            Ok(())
        })?;

        log::info!("Saved schematic to: {}", path.display());
        Ok(())
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn suggested_schematic_save_path(default_name: Option<&str>) -> PathBuf {
    let mut path = PathBuf::from(
        default_name
            .filter(|name| !name.trim().is_empty())
            .unwrap_or("untitled.rsch"),
    );
    crate::common::file_actions::ensure_file_extension(&mut path, "rsch");
    path
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn serialize_schematic_file(file: &SchematicFile) -> Result<String, SchematicIoError> {
    let mut contents = serde_json::to_string_pretty(file)
        .map_err(|e| SchematicIoError::SerializeError(e.to_string()))?;
    contents.push('\n');
    Ok(contents)
}

/// Load schematic from file
///
/// Validates version compatibility and recalculates runtime state.
pub fn load_schematic(path: &Path) -> Result<SchematicState, SchematicIoError> {
    let file = load_schematic_file(path)?;
    let schematic = prepare_loaded_schematic(file, Some(path))?;

    log::info!("Loaded schematic from: {}", path.display());
    Ok(schematic)
}

pub(crate) fn load_schematic_text(
    contents: &str,
    source_path: Option<&Path>,
) -> Result<SchematicState, SchematicIoError> {
    let file: SchematicFile =
        serde_json::from_str(contents).map_err(|e| SchematicIoError::ParseError(e.to_string()))?;
    prepare_loaded_schematic(file, source_path)
}

fn prepare_loaded_schematic(
    file: SchematicFile,
    source_path: Option<&Path>,
) -> Result<SchematicState, SchematicIoError> {
    file.validate()?;

    let mut schematic = file.schematic;

    // Recalculate runtime state (IDs, counters, etc.)
    schematic.recalculate_runtime_state();

    // Set the file path for subsequent saves.
    schematic.current_file = source_path.map(Path::to_path_buf);

    // Mark for fit-to-view and history reset.
    schematic.needs_fit = true;
    schematic.needs_history_reset = true;

    // Clear dirty flag since we just loaded.
    schematic.is_dirty = false;

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
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    serde_json::from_str(&contents).map_err(|e| SchematicIoError::ParseError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schematic_version_requires_current_major() {
        assert!(SchematicVersion::current().is_compatible());
        assert!(
            !SchematicVersion {
                major: 0,
                minor: 9,
                patch: 0,
            }
            .is_compatible()
        );
        assert!(
            !SchematicVersion {
                major: 2,
                minor: 0,
                patch: 0,
            }
            .is_compatible()
        );
    }

    #[test]
    fn suggested_schematic_save_path_defaults_and_enforces_extension() {
        assert_eq!(
            suggested_schematic_save_path(None),
            PathBuf::from("untitled.rsch")
        );
        assert_eq!(
            suggested_schematic_save_path(Some("filter")),
            PathBuf::from("filter.rsch")
        );
        assert_eq!(
            suggested_schematic_save_path(Some("filter.rsch")),
            PathBuf::from("filter.rsch")
        );
    }

    #[test]
    fn schematic_file_serializes_to_versioned_json() {
        let file = SchematicFile::new(SchematicState::default());

        let json = serialize_schematic_file(&file).expect("schematic serializes");

        assert!(json.contains("\"version\""));
        assert!(json.contains("\"schematic\""));
        assert!(json.ends_with('\n'));
    }

    #[test]
    fn schematic_text_load_validates_and_prepares_runtime_state() {
        let mut original = SchematicState::default();
        original.current_file = Some(PathBuf::from("stale-native-path.rsch"));
        original.is_dirty = true;
        original.needs_fit = false;
        original.needs_history_reset = false;
        let file = SchematicFile::new(original);
        let json = serialize_schematic_file(&file).expect("schematic serializes");

        let loaded = load_schematic_text(&json, Some(Path::new("browser-filter.rsch")))
            .expect("schematic text loads");

        assert_eq!(
            loaded.current_file.as_deref(),
            Some(Path::new("browser-filter.rsch"))
        );
        assert!(loaded.needs_fit);
        assert!(loaded.needs_history_reset);
        assert!(!loaded.is_dirty);
    }

    #[test]
    fn schematic_text_load_without_source_path_clears_stale_file_identity() {
        let mut original = SchematicState::default();
        original.current_file = Some(PathBuf::from("stale-native-path.rsch"));
        let file = SchematicFile::new(original);
        let json = serialize_schematic_file(&file).expect("schematic serializes");

        let loaded = load_schematic_text(&json, None).expect("schematic text loads");

        assert!(loaded.current_file.is_none());
        assert!(!loaded.is_dirty);
    }

    #[test]
    fn schematic_text_load_reports_parse_errors_without_filesystem() {
        let err = load_schematic_text("{not valid json", Some(Path::new("bad.rsch")))
            .expect_err("invalid schematic text fails");

        assert!(matches!(err, SchematicIoError::ParseError(_)));
    }

    #[test]
    fn schematic_text_load_removes_malformed_wires_and_stale_selection() {
        use crate::state::{ComponentType, Point, Wire};

        let mut original = SchematicState::default();
        let live_component_id = original.add_component(ComponentType::Resistor, Point::new(0, 0));
        original.wires.push(Wire::new(98, Vec::new()));
        original.wires.push(Wire::new(99, vec![Point::new(5, 5)]));
        original
            .wires
            .push(Wire::new(100, vec![Point::new(0, 0), Point::new(20, 0)]));
        original
            .clipboard
            .wires
            .push(Wire::new(101, vec![Point::new(7, 7)]));
        original
            .clipboard
            .wires
            .push(Wire::new(102, vec![Point::new(10, 10), Point::new(20, 10)]));

        original.selection.select_component(live_component_id);
        original.selection.select_component(404);
        original.selection.select_wire(98);
        original.selection.select_wire(100);
        original.selection.select_wire_segment(99, 0);
        original.selection.select_wire_segment(100, 0);
        original.selection.select_wire_vertex(99, 0);
        original.selection.select_wire_vertex(100, 1);

        let json =
            serialize_schematic_file(&SchematicFile::new(original)).expect("schematic serializes");

        let mut loaded = load_schematic_text(&json, Some(Path::new("corrupt-import.rsch")))
            .expect("repairable schematic loads");

        assert_eq!(loaded.wires.len(), 1);
        assert_eq!(loaded.wires[0].id, 100);
        assert_eq!(loaded.clipboard.wires.len(), 1);
        assert_eq!(loaded.clipboard.wires[0].id, 102);
        assert!(loaded.selection.has_component(live_component_id));
        assert!(!loaded.selection.has_component(404));
        assert!(loaded.selection.has_wire(100));
        assert!(!loaded.selection.has_wire(98));
        assert_eq!(loaded.selection.wire_segments.len(), 1);
        assert!(loaded.selection.has_wire_segment(100, 0));
        assert_eq!(loaded.selection.wire_vertices.len(), 1);
        assert!(loaded.selection.has_wire_vertex(100, 1));
        assert_eq!(loaded.wire_vertex_at(Point::new(5, 5)), None);

        loaded.ensure_canvas_cache();
        assert_eq!(loaded.wire_vertex_at(Point::new(5, 5)), None);
    }

    #[test]
    fn schematic_text_load_repairs_duplicate_wire_ids() {
        use crate::state::{Point, Wire};
        use std::collections::HashSet;

        let mut original = SchematicState::default();
        original
            .wires
            .push(Wire::segment(40, Point::new(0, 0), Point::new(20, 0)));
        original
            .wires
            .push(Wire::segment(40, Point::new(0, 10), Point::new(20, 10)));
        original.selection.select_wire(40);

        let json =
            serialize_schematic_file(&SchematicFile::new(original)).expect("schematic serializes");

        let mut loaded = load_schematic_text(&json, Some(Path::new("duplicate-wires.rsch")))
            .expect("repairable schematic loads");

        let mut wire_ids: HashSet<u64> = loaded.wires.iter().map(|wire| wire.id).collect();
        assert_eq!(wire_ids.len(), loaded.wires.len());
        assert_eq!(
            loaded.wires.iter().filter(|wire| wire.id == 40).count(),
            1,
            "the original duplicate id must stay with exactly one wire"
        );
        assert!(
            loaded.selection.has_wire(40),
            "ambiguous selection should remain attached to the first retained wire"
        );

        let fresh_id = loaded.add_wire(vec![Point::new(0, 20), Point::new(20, 20)]);
        assert!(
            fresh_id.is_some_and(|id| wire_ids.insert(id)),
            "new wires must not reuse repaired ids"
        );
    }
}

// =============================================================================
// Tests
// =============================================================================
