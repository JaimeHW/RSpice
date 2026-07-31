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
            minor: 2,
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
    let now = crate::time_compat::unix_epoch().as_secs();
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
        crate::workbench::browser::download::download_text_file(path, &contents)
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
    crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "rsch");
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

    for note in &file.schematic.design_notes {
        note.validate().map_err(|error| {
            SchematicIoError::ParseError(format!("invalid design note object {}: {error}", note.id))
        })?;
    }
    for shape in &file.schematic.documentation_shapes {
        shape.validate().map_err(|error| {
            SchematicIoError::ParseError(format!(
                "invalid documentation shape object {}: {error}",
                shape.id
            ))
        })?;
    }
    for probe in &file.schematic.probes {
        probe.validate().map_err(|error| {
            SchematicIoError::ParseError(format!(
                "invalid schematic probe object {}: {error}",
                probe.id
            ))
        })?;
    }

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

    fn documentation_shape_fixture() -> Vec<crate::state::DocumentationShape> {
        use crate::state::{DocumentationShape, DocumentationShapeGeometry, Point};

        [
            DocumentationShapeGeometry::Rectangle {
                first: Point::new(-80, -40),
                opposite: Point::new(20, 30),
            },
            DocumentationShapeGeometry::Line {
                start: Point::new(-25, 70),
                end: Point::new(65, 105),
            },
            DocumentationShapeGeometry::Polygon {
                points: vec![
                    Point::new(100, -20),
                    Point::new(170, 10),
                    Point::new(145, 85),
                    Point::new(80, 45),
                ],
            },
            DocumentationShapeGeometry::Arc {
                start: Point::new(-100, 160),
                through: Point::new(-50, 110),
                end: Point::new(0, 160),
            },
            DocumentationShapeGeometry::Callout {
                tip: Point::new(90, 150),
                elbow: Point::new(130, 125),
                box_corner: Point::new(230, 190),
            },
        ]
        .into_iter()
        .enumerate()
        .map(|(index, geometry)| {
            DocumentationShape::new(10_000 + index as u64, geometry)
                .expect("documentation shape fixture is valid")
        })
        .collect()
    }

    #[test]
    fn documentation_shapes_round_trip_and_legacy_documents_default_empty() {
        let shapes = documentation_shape_fixture();
        let mut schematic = SchematicState::default();
        schematic.documentation_shapes = shapes.clone();

        let json = serialize_schematic_file(&SchematicFile::new(schematic)).unwrap();
        let loaded = load_schematic_text(&json, None).unwrap();
        assert_eq!(loaded.documentation_shapes, shapes);

        let mut legacy: serde_json::Value = serde_json::from_str(&json).unwrap();
        legacy["version"]["minor"] = serde_json::json!(1);
        legacy["schematic"]
            .as_object_mut()
            .unwrap()
            .remove("documentation_shapes");
        let loaded = load_schematic_text(&legacy.to_string(), None).unwrap();
        assert!(loaded.documentation_shapes.is_empty());
    }

    #[test]
    fn probe_flags_round_trip_and_legacy_documents_default_empty() {
        let mut schematic = SchematicState::default();
        schematic.probes.push(
            crate::state::SchematicProbe::new(
                44,
                crate::state::Point::new(30, 20),
                "V(out)",
                Some("V(out)".to_owned()),
            )
            .unwrap(),
        );

        let json = serialize_schematic_file(&SchematicFile::new(schematic)).unwrap();
        let loaded = load_schematic_text(&json, None).unwrap();
        assert_eq!(loaded.probes.len(), 1);
        assert_eq!(
            loaded.probes[0].source_expression.as_deref(),
            Some("V(out)")
        );

        let mut legacy: serde_json::Value = serde_json::from_str(&json).unwrap();
        legacy["schematic"]
            .as_object_mut()
            .unwrap()
            .remove("probes");
        let loaded = load_schematic_text(&legacy.to_string(), None).unwrap();
        assert!(loaded.probes.is_empty());
    }

    #[test]
    fn standalone_files_exclude_and_ignore_transient_editor_state() {
        use crate::state::{ComponentType, Point, Rotation};

        let mut schematic = SchematicState::default();
        let component_id = schematic.add_component(ComponentType::Resistor, Point::new(10, 20));
        schematic.selection.select_only_component(component_id);
        schematic.copy_selection();
        schematic.wire_drawing.start(Point::new(30, 40));
        schematic.wire_drawing.update_preview(Point::new(50, 60));
        schematic.preview_rotation = Rotation::R90;
        schematic.preview_mirror_h = true;

        let selection = serde_json::to_value(&schematic.selection).unwrap();
        let wire_drawing = serde_json::to_value(&schematic.wire_drawing).unwrap();
        let clipboard = serde_json::to_value(&schematic.clipboard).unwrap();
        let preview_rotation = serde_json::to_value(schematic.preview_rotation).unwrap();
        let json = serialize_schematic_file(&SchematicFile::new(schematic)).unwrap();
        let serialized: serde_json::Value = serde_json::from_str(&json).unwrap();
        let document = serialized["schematic"]
            .as_object()
            .expect("schematic is a JSON object");
        for runtime_key in [
            "selection",
            "wire_drawing",
            "clipboard",
            "preview_rotation",
            "preview_mirror_h",
        ] {
            assert!(
                !document.contains_key(runtime_key),
                "{runtime_key} must not be persisted in a design document"
            );
        }

        // Older files may contain these fields. They remain readable, but the
        // runtime-only values are deliberately discarded on import.
        let mut legacy = serialized;
        let document = legacy["schematic"]
            .as_object_mut()
            .expect("schematic is a JSON object");
        document.insert("selection".to_owned(), selection);
        document.insert("wire_drawing".to_owned(), wire_drawing);
        document.insert("clipboard".to_owned(), clipboard);
        document.insert("preview_rotation".to_owned(), preview_rotation);
        document.insert("preview_mirror_h".to_owned(), serde_json::Value::Bool(true));

        let loaded = load_schematic_text(&legacy.to_string(), None).unwrap();
        assert!(loaded.selection.is_empty());
        assert!(loaded.clipboard.is_empty());
        assert!(!loaded.wire_drawing.active);
        assert!(loaded.wire_drawing.points.is_empty());
        assert_eq!(loaded.wire_drawing.preview_pos, None);
        assert_eq!(loaded.preview_rotation, Rotation::R0);
        assert!(!loaded.preview_mirror_h);
    }

    #[test]
    fn malformed_serialized_live_documentation_shapes_fail_closed() {
        let shapes = documentation_shape_fixture();
        let mut schematic = SchematicState::default();
        schematic.documentation_shapes.push(shapes[0].clone());
        let json = serialize_schematic_file(&SchematicFile::new(schematic)).unwrap();

        let mut malformed_live: serde_json::Value = serde_json::from_str(&json).unwrap();
        malformed_live["schematic"]["documentation_shapes"][0]["geometry"] = serde_json::json!({
            "kind": "rectangle",
            "first": { "x": 10, "y": 20 },
            "opposite": { "x": 10, "y": 80 }
        });
        assert!(matches!(
            load_schematic_text(&malformed_live.to_string(), None),
            Err(SchematicIoError::ParseError(message))
                if message.contains("invalid documentation shape object")
        ));
    }

    #[test]
    fn design_notes_round_trip_and_legacy_documents_default_empty() {
        let mut schematic = SchematicState::default();
        let mut review = crate::state::DesignNote::new(
            94,
            crate::state::Point::new(10, 30),
            crate::state::DesignNoteKind::ReviewNote,
            "Review bias path\nSecond line",
        )
        .unwrap();
        review
            .set_review_state(crate::state::DesignReviewState::Resolved)
            .unwrap();
        let notes = vec![
            crate::state::DesignNote::new(
                91,
                crate::state::Point::new(-20, 30),
                crate::state::DesignNoteKind::PlainText,
                "Bias network\nKeep clear",
            )
            .unwrap(),
            crate::state::DesignNote::new(
                92,
                crate::state::Point::new(-10, 30),
                crate::state::DesignNoteKind::PropertyDisplay,
                "${component_count} components",
            )
            .unwrap(),
            crate::state::DesignNote::new(
                93,
                crate::state::Point::new(0, 30),
                crate::state::DesignNoteKind::RequirementLink,
                "https://tracker.example/item?id=91&source=schematic",
            )
            .unwrap(),
            review,
        ];
        schematic.design_notes = notes.clone();
        schematic.selection.select_only_design_note(93);
        schematic.copy_selection();
        assert_eq!(schematic.clipboard.design_notes.len(), 1);
        let json = serialize_schematic_file(&SchematicFile::new(schematic)).unwrap();
        let loaded = load_schematic_text(&json, None).unwrap();
        assert_eq!(loaded.design_notes, notes);
        assert!(loaded.clipboard.design_notes.is_empty());

        let mut legacy: serde_json::Value = serde_json::from_str(&json).unwrap();
        legacy["schematic"]
            .as_object_mut()
            .unwrap()
            .remove("design_notes");
        let loaded = load_schematic_text(&legacy.to_string(), None).unwrap();
        assert!(loaded.design_notes.is_empty());
    }

    #[test]
    fn malformed_serialized_design_note_fails_closed() {
        let mut schematic = SchematicState::default();
        schematic.design_notes.push(
            crate::state::DesignNote::new(
                3,
                crate::state::Point::origin(),
                crate::state::DesignNoteKind::PlainText,
                "valid",
            )
            .unwrap(),
        );
        let json = serialize_schematic_file(&SchematicFile::new(schematic)).unwrap();
        let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
        value["schematic"]["design_notes"][0]["text"] = serde_json::json!("   ");

        assert!(matches!(
            load_schematic_text(&value.to_string(), None),
            Err(SchematicIoError::ParseError(message)) if message.contains("invalid design note")
        ));
    }

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
    fn schematic_text_load_removes_malformed_wires_and_resets_runtime_state() {
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
        assert!(loaded.clipboard.is_empty());
        assert!(loaded.selection.is_empty());
        assert!(
            loaded
                .components
                .iter()
                .any(|component| component.id == live_component_id),
            "durable component content still loads"
        );
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
            loaded.selection.is_empty(),
            "selection is session-local and never reopens with the document"
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
