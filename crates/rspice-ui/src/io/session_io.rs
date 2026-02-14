//! Session File I/O
//!
//! Load and save session state with versioning support.
//! Compatible with Cadence CIW session format concepts.
//!
//! # Features
//!
//! - JSON-based session files with schema versioning
//! - Backward compatibility for older session versions
//! - Atomic saves with temporary file + rename
//! - Auto-backup before overwrite

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// =============================================================================
// Session Version
// =============================================================================

/// Session file format version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionVersion {
    /// Major version (breaking changes)
    pub major: u32,
    /// Minor version (new features, backward compatible)
    pub minor: u32,
    /// Patch version (bug fixes)
    pub patch: u32,
}

impl Default for SessionVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl SessionVersion {
    /// Current session format version
    pub const fn current() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }

    /// Check if this version is compatible with current
    pub fn is_compatible(&self) -> bool {
        self.major == Self::current().major
    }
}

impl std::fmt::Display for SessionVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// =============================================================================
// Session Metadata
// =============================================================================

/// Session file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// Format version
    pub version: SessionVersion,
    /// Creation timestamp (Unix epoch seconds)
    pub created_at: u64,
    /// Last modified timestamp
    pub modified_at: u64,
    /// Application version that created this
    pub app_version: String,
    /// User name (optional)
    pub user: Option<String>,
    /// Machine name (optional)
    pub machine: Option<String>,
    /// Session description
    pub description: Option<String>,
}

impl Default for SessionMetadata {
    fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            version: SessionVersion::current(),
            created_at: now,
            modified_at: now,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            user: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .ok(),
            machine: std::env::var("COMPUTERNAME")
                .or_else(|_| std::env::var("HOSTNAME"))
                .ok(),
            description: None,
        }
    }
}

impl SessionMetadata {
    /// Update modified timestamp
    pub fn touch(&mut self) {
        self.modified_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }
}

// =============================================================================
// Window State
// =============================================================================

/// Window geometry and state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindowState {
    /// Window title/name
    pub name: String,
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
    /// Is maximized
    pub maximized: bool,
    /// Is minimized
    pub minimized: bool,
    /// Is visible
    pub visible: bool,
    /// Z-order (stacking)
    pub z_order: i32,
}

impl WindowState {
    /// Create with name and default geometry
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            width: 800,
            height: 600,
            visible: true,
            ..Default::default()
        }
    }

    /// Set position
    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set size
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

// =============================================================================
// Panel State
// =============================================================================

/// Dockable panel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    /// Panel ID
    pub id: String,
    /// Panel title
    pub title: String,
    /// Is panel open
    pub open: bool,
    /// Dock location
    pub dock: DockLocation,
    /// Size (width or height depending on dock)
    pub size: f32,
    /// Custom panel data
    pub custom_data: HashMap<String, String>,
}

/// Dock location for panels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DockLocation {
    /// Left side dock
    Left,
    /// Right side dock
    #[default]
    Right,
    /// Bottom dock
    Bottom,
    /// Top dock
    Top,
    /// Floating
    Floating,
    /// Tab within another panel
    Tab,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            open: true,
            dock: DockLocation::Right,
            size: 300.0,
            custom_data: HashMap::new(),
        }
    }
}

// =============================================================================
// Open File State
// =============================================================================

/// State of an open file/document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenFileState {
    /// File path
    pub path: PathBuf,
    /// File type
    pub file_type: FileType,
    /// Is file modified
    pub modified: bool,
    /// Cursor/scroll position
    pub cursor_line: Option<usize>,
    /// View-specific state
    pub view_state: HashMap<String, String>,
    /// Last access timestamp
    pub last_access: u64,
}

/// Type of open file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    /// Schematic file
    Schematic,
    /// Symbol file
    Symbol,
    /// Netlist file
    Netlist,
    /// Waveform file
    Waveform,
    /// Model library
    ModelLibrary,
    /// Text file
    Text,
    /// Unknown
    Unknown,
}

impl Default for OpenFileState {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            file_type: FileType::Unknown,
            modified: false,
            cursor_line: None,
            view_state: HashMap::new(),
            last_access: 0,
        }
    }
}

impl OpenFileState {
    /// Create from path
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let file_type = FileType::from_extension(path.extension().and_then(|e| e.to_str()));

        Self {
            path,
            file_type,
            ..Default::default()
        }
    }
}

impl FileType {
    /// Detect from file extension
    pub fn from_extension(ext: Option<&str>) -> Self {
        match ext {
            Some("sch") | Some("schematic") => FileType::Schematic,
            Some("sym") | Some("symbol") => FileType::Symbol,
            Some("spice") | Some("spi") | Some("cir") | Some("net") => FileType::Netlist,
            Some("psf") | Some("raw") | Some("tr0") => FileType::Waveform,
            Some("lib") | Some("scs") => FileType::ModelLibrary,
            Some("txt") | Some("log") => FileType::Text,
            _ => FileType::Unknown,
        }
    }
}

// =============================================================================
// Session File
// =============================================================================

/// Complete session state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionFile {
    /// Session metadata
    pub metadata: SessionMetadata,
    /// Main window state
    pub main_window: WindowState,
    /// Additional windows
    pub windows: Vec<WindowState>,
    /// Panel states
    pub panels: Vec<PanelState>,
    /// Open files
    pub open_files: Vec<OpenFileState>,
    /// Active file index
    pub active_file: Option<usize>,
    /// Recent files (paths)
    pub recent_files: Vec<PathBuf>,
    /// Recent projects
    pub recent_projects: Vec<PathBuf>,
    /// User preferences relevant to session
    pub preferences: HashMap<String, String>,
    /// Design variables
    pub variables: HashMap<String, String>,
    /// Current working directory
    pub working_directory: Option<PathBuf>,
    /// Selected corner
    pub corner: Option<String>,
    /// Custom session data
    pub custom: HashMap<String, serde_json::Value>,
}

impl SessionFile {
    /// Create a new empty session
    pub fn new() -> Self {
        Self {
            main_window: WindowState::new("RSpice"),
            ..Default::default()
        }
    }

    /// Add an open file
    pub fn add_open_file(&mut self, file: OpenFileState) {
        self.open_files.push(file);
    }

    /// Add a recent file
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already present
        self.recent_files.retain(|p| p != &path);
        // Add to front
        self.recent_files.insert(0, path);
        // Limit to 20
        self.recent_files.truncate(20);
    }

    /// Get open file by path
    pub fn find_open_file(&self, path: &Path) -> Option<&OpenFileState> {
        self.open_files.iter().find(|f| f.path == path)
    }

    /// Set a preference
    pub fn set_preference(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.preferences.insert(key.into(), value.into());
    }

    /// Get a preference
    pub fn get_preference(&self, key: &str) -> Option<&str> {
        self.preferences.get(key).map(|s| s.as_str())
    }

    /// Validate session data
    pub fn validate(&self) -> Result<(), String> {
        if !self.metadata.version.is_compatible() {
            return Err(format!(
                "Session version {} is not compatible with current version {}",
                self.metadata.version,
                SessionVersion::current()
            ));
        }
        Ok(())
    }
}

// =============================================================================
// Session I/O Functions
// =============================================================================

/// Save session to file
pub fn save_session(session: &SessionFile, path: &Path) -> Result<(), String> {
    // Create backup if file exists
    if path.exists() {
        let backup_path = path.with_extension("session.bak");
        fs::copy(path, &backup_path).map_err(|e| format!("Failed to create backup: {}", e))?;
    }

    // Write to temporary file first
    let temp_path = path.with_extension("session.tmp");

    let file =
        File::create(&temp_path).map_err(|e| format!("Failed to create temp file: {}", e))?;

    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, session)
        .map_err(|e| format!("Failed to serialize session: {}", e))?;

    writer
        .flush()
        .map_err(|e| format!("Failed to flush: {}", e))?;

    // Atomic rename
    fs::rename(&temp_path, path).map_err(|e| format!("Failed to rename temp file: {}", e))?;

    Ok(())
}

/// Load session from file
pub fn load_session(path: &Path) -> Result<SessionFile, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;

    let reader = BufReader::new(file);

    let session: SessionFile =
        serde_json::from_reader(reader).map_err(|e| format!("Failed to parse session: {}", e))?;

    session.validate()?;

    Ok(session)
}

/// Load session or create default
pub fn load_or_create_session(path: &Path) -> SessionFile {
    load_session(path).unwrap_or_else(|_| SessionFile::new())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // =========================================================================
    // SessionVersion Tests
    // =========================================================================

    #[test]
    fn test_version_current() {
        let v = SessionVersion::current();
        assert_eq!(v.major, 1);
        assert!(v.is_compatible());
    }

    #[test]
    fn test_version_to_string() {
        let v = SessionVersion {
            major: 1,
            minor: 2,
            patch: 3,
        };
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_compatibility() {
        let compatible = SessionVersion {
            major: 1,
            minor: 5,
            patch: 0,
        };
        assert!(compatible.is_compatible());

        let incompatible = SessionVersion {
            major: 2,
            minor: 0,
            patch: 0,
        };
        assert!(!incompatible.is_compatible());
    }

    // =========================================================================
    // SessionMetadata Tests
    // =========================================================================

    #[test]
    fn test_metadata_default() {
        let meta = SessionMetadata::default();
        assert!(meta.created_at > 0);
        assert!(meta.version.is_compatible());
    }

    #[test]
    fn test_metadata_touch() {
        let mut meta = SessionMetadata::default();
        let original = meta.modified_at;

        std::thread::sleep(std::time::Duration::from_millis(10));
        meta.touch();

        assert!(meta.modified_at >= original);
    }

    // =========================================================================
    // WindowState Tests
    // =========================================================================

    #[test]
    fn test_window_state_new() {
        let ws = WindowState::new("Test Window");
        assert_eq!(ws.name, "Test Window");
        assert!(ws.visible);
    }

    #[test]
    fn test_window_state_with_position() {
        let ws = WindowState::new("Test")
            .with_position(100, 200)
            .with_size(800, 600);

        assert_eq!(ws.x, 100);
        assert_eq!(ws.y, 200);
        assert_eq!(ws.width, 800);
        assert_eq!(ws.height, 600);
    }

    // =========================================================================
    // FileType Tests
    // =========================================================================

    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(FileType::from_extension(Some("sch")), FileType::Schematic);
        assert_eq!(
            FileType::from_extension(Some("lib")),
            FileType::ModelLibrary
        );
        assert_eq!(FileType::from_extension(Some("psf")), FileType::Waveform);
        assert_eq!(FileType::from_extension(Some("xyz")), FileType::Unknown);
    }

    #[test]
    fn test_open_file_state_from_path() {
        let state = OpenFileState::from_path("/path/to/test.sch");
        assert_eq!(state.file_type, FileType::Schematic);
    }

    // =========================================================================
    // SessionFile Tests
    // =========================================================================

    #[test]
    fn test_session_new() {
        let session = SessionFile::new();
        assert!(session.metadata.version.is_compatible());
        assert_eq!(session.main_window.name, "RSpice");
    }

    #[test]
    fn test_session_add_open_file() {
        let mut session = SessionFile::new();
        session.add_open_file(OpenFileState::from_path("/test.sch"));

        assert_eq!(session.open_files.len(), 1);
    }

    #[test]
    fn test_session_add_recent_file() {
        let mut session = SessionFile::new();
        session.add_recent_file(PathBuf::from("/a.sch"));
        session.add_recent_file(PathBuf::from("/b.sch"));
        session.add_recent_file(PathBuf::from("/a.sch")); // Duplicate

        assert_eq!(session.recent_files.len(), 2);
        assert_eq!(session.recent_files[0], PathBuf::from("/a.sch")); // Most recent first
    }

    #[test]
    fn test_session_preferences() {
        let mut session = SessionFile::new();
        session.set_preference("theme", "dark");

        assert_eq!(session.get_preference("theme"), Some("dark"));
        assert_eq!(session.get_preference("nonexistent"), None);
    }

    #[test]
    fn test_session_validate() {
        let session = SessionFile::new();
        assert!(session.validate().is_ok());
    }

    // =========================================================================
    // I/O Tests
    // =========================================================================

    #[test]
    fn test_save_and_load_session() {
        let mut session = SessionFile::new();
        session.set_preference("test_key", "test_value");
        session.add_recent_file(PathBuf::from("/test/file.sch"));

        let temp = NamedTempFile::new().unwrap();
        let path = temp.path();

        // Save
        save_session(&session, path).unwrap();

        // Load
        let loaded = load_session(path).unwrap();

        assert_eq!(loaded.get_preference("test_key"), Some("test_value"));
        assert_eq!(loaded.recent_files.len(), 1);
    }

    #[test]
    fn test_load_invalid_json() {
        let mut temp = NamedTempFile::new().unwrap();
        writeln!(temp, "{{ invalid json }}").unwrap();

        let result = load_session(temp.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_or_create() {
        let path = Path::new("/nonexistent/path/session.json");
        let session = load_or_create_session(path);

        // Should create default
        assert!(session.metadata.version.is_compatible());
    }

    // =========================================================================
    // Serialization Tests
    // =========================================================================

    #[test]
    fn test_session_json_roundtrip() {
        let mut session = SessionFile::new();
        session.main_window = WindowState::new("Main")
            .with_position(100, 100)
            .with_size(1920, 1080);

        session.panels.push(PanelState {
            id: "browser".to_string(),
            title: "Library Browser".to_string(),
            open: true,
            dock: DockLocation::Left,
            size: 250.0,
            custom_data: HashMap::new(),
        });

        let json = serde_json::to_string_pretty(&session).unwrap();
        let parsed: SessionFile = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.main_window.width, 1920);
        assert_eq!(parsed.panels.len(), 1);
        assert_eq!(parsed.panels[0].dock, DockLocation::Left);
    }
}
