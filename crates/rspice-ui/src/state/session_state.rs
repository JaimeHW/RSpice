//! Session State Save/Restore
//!
//! Persistence of window layout, open files, and UI state.
//!
//! # Features
//!
//! - Window positions and sizes
//! - Open document list with scroll positions
//! - Panel visibility and sizes
//! - Active selections and zoom levels
//! - Recently used files
//! - Simulation settings persistence

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// =============================================================================
// Window Geometry
// =============================================================================

/// Window position and size
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WindowGeometry {
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
    /// Whether window is maximized
    pub maximized: bool,
}

impl Default for WindowGeometry {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 1280,
            height: 800,
            maximized: false,
        }
    }
}

impl WindowGeometry {
    /// Create with specific size
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            maximized: false,
        }
    }

    /// Create maximized
    pub fn maximized() -> Self {
        Self {
            maximized: true,
            ..Default::default()
        }
    }
}

// =============================================================================
// Panel State
// =============================================================================

/// Panel visibility and size state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    /// Panel ID
    pub id: String,
    /// Whether panel is visible
    pub visible: bool,
    /// Panel width (for side panels)
    pub width: Option<f32>,
    /// Panel height (for bottom panels)
    pub height: Option<f32>,
    /// Whether panel is collapsed
    pub collapsed: bool,
    /// Panel-specific state data
    pub state_data: HashMap<String, String>,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            id: String::new(),
            visible: true,
            width: None,
            height: None,
            collapsed: false,
            state_data: HashMap::new(),
        }
    }
}

impl PanelState {
    /// Create a new panel state
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            ..Default::default()
        }
    }

    /// Set visibility
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set width
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
}

// =============================================================================
// Document State
// =============================================================================

/// State of a single open document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentState {
    /// File path
    pub path: PathBuf,
    /// Document type
    pub doc_type: String,
    /// Whether document has unsaved changes
    pub modified: bool,
    /// Scroll position X
    pub scroll_x: f32,
    /// Scroll position Y
    pub scroll_y: f32,
    /// Zoom level
    pub zoom: f32,
    /// Cursor position (if applicable)
    pub cursor_line: Option<usize>,
    /// Selection state
    pub selection: Option<String>,
    /// View-specific state
    pub view_state: HashMap<String, String>,
}

impl Default for DocumentState {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            doc_type: "unknown".to_string(),
            modified: false,
            scroll_x: 0.0,
            scroll_y: 0.0,
            zoom: 1.0,
            cursor_line: None,
            selection: None,
            view_state: HashMap::new(),
        }
    }
}

impl DocumentState {
    /// Create from file path
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    /// Set document type
    pub fn with_type(mut self, doc_type: impl Into<String>) -> Self {
        self.doc_type = doc_type.into();
        self
    }

    /// Set zoom
    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    /// Set scroll position
    pub fn with_scroll(mut self, x: f32, y: f32) -> Self {
        self.scroll_x = x;
        self.scroll_y = y;
        self
    }
}

// =============================================================================
// Session State
// =============================================================================

/// Complete session state for save/restore
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionState {
    /// Session name
    pub name: String,
    /// Version string for compatibility
    pub version: String,
    /// Main window geometry
    pub main_window: WindowGeometry,
    /// Additional window geometries
    pub windows: HashMap<String, WindowGeometry>,
    /// Panel states
    pub panels: HashMap<String, PanelState>,
    /// Open documents
    pub documents: Vec<DocumentState>,
    /// Active document index
    pub active_document: Option<usize>,
    /// Recently opened files
    pub recent_files: Vec<PathBuf>,
    /// Recent directories
    pub recent_dirs: Vec<PathBuf>,
    /// User preferences
    pub preferences: HashMap<String, String>,
    /// Timestamp of last save
    pub last_saved: Option<u64>,
    /// Custom session data
    pub custom_data: HashMap<String, String>,
}

impl SessionState {
    /// Current session version
    pub const VERSION: &'static str = "1.0.0";

    /// Create a new session
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: Self::VERSION.to_string(),
            main_window: WindowGeometry::default(),
            ..Default::default()
        }
    }

    /// Add a panel state
    pub fn add_panel(&mut self, panel: PanelState) {
        self.panels.insert(panel.id.clone(), panel);
    }

    /// Get a panel state
    pub fn get_panel(&self, id: &str) -> Option<&PanelState> {
        self.panels.get(id)
    }

    /// Get mutable panel state
    pub fn get_panel_mut(&mut self, id: &str) -> Option<&mut PanelState> {
        self.panels.get_mut(id)
    }

    /// Add an open document
    pub fn add_document(&mut self, doc: DocumentState) {
        self.documents.push(doc);
    }

    /// Get the active document
    pub fn active_document(&self) -> Option<&DocumentState> {
        self.active_document.and_then(|idx| self.documents.get(idx))
    }

    /// Set active document by path
    pub fn set_active_by_path(&mut self, path: &PathBuf) {
        self.active_document = self.documents.iter().position(|d| &d.path == path);
    }

    /// Add to recent files
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);
        // Add to front
        self.recent_files.insert(0, path);
        // Limit to 20
        self.recent_files.truncate(20);
    }

    /// Add to recent directories
    pub fn add_recent_dir(&mut self, path: PathBuf) {
        self.recent_dirs.retain(|p| p != &path);
        self.recent_dirs.insert(0, path);
        self.recent_dirs.truncate(10);
    }

    /// Set a preference
    pub fn set_preference(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.preferences.insert(key.into(), value.into());
    }

    /// Get a preference
    pub fn get_preference(&self, key: &str) -> Option<&String> {
        self.preferences.get(key)
    }

    /// Close a document
    pub fn close_document(&mut self, path: &PathBuf) {
        if let Some(idx) = self.documents.iter().position(|d| &d.path == path) {
            self.documents.remove(idx);
            // Adjust active document
            if let Some(active) = self.active_document {
                if active == idx {
                    self.active_document = if self.documents.is_empty() {
                        None
                    } else {
                        Some(active.min(self.documents.len() - 1))
                    };
                } else if active > idx {
                    self.active_document = Some(active - 1);
                }
            }
        }
    }

    /// Get document count
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    /// Check if any documents have unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.documents.iter().any(|d| d.modified)
    }

    /// Clear all state
    pub fn clear(&mut self) {
        self.documents.clear();
        self.active_document = None;
        self.panels.clear();
        self.windows.clear();
    }
}

// =============================================================================
// Session Manager
// =============================================================================

/// Manager for session save/restore
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionManager {
    /// Current session
    current: SessionState,
    /// Default session path
    pub session_path: Option<PathBuf>,
    /// Auto-save enabled
    pub auto_save_enabled: bool,
    /// Auto-save interval (seconds)
    pub auto_save_interval: u64,
    /// Last auto-save timestamp
    pub last_auto_save: Option<u64>,
}

impl SessionManager {
    /// Create a new manager
    pub fn new() -> Self {
        Self {
            current: SessionState::new("default"),
            auto_save_enabled: true,
            auto_save_interval: 300, // 5 minutes
            ..Default::default()
        }
    }

    /// Get current session
    pub fn current(&self) -> &SessionState {
        &self.current
    }

    /// Get mutable current session
    pub fn current_mut(&mut self) -> &mut SessionState {
        &mut self.current
    }

    /// Set session path
    pub fn set_session_path(&mut self, path: PathBuf) {
        self.session_path = Some(path);
    }

    /// Check if auto-save is due
    pub fn auto_save_due(&self, current_time: u64) -> bool {
        if !self.auto_save_enabled {
            return false;
        }
        match self.last_auto_save {
            Some(last) => current_time - last >= self.auto_save_interval,
            None => true,
        }
    }

    /// Record auto-save
    pub fn record_auto_save(&mut self, timestamp: u64) {
        self.last_auto_save = Some(timestamp);
        self.current.last_saved = Some(timestamp);
    }

    /// Serialize session to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.current)
    }

    /// Deserialize session from JSON
    pub fn load_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
        self.current = serde_json::from_str(json)?;
        Ok(())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
        self.load_json(json)
    }

    /// Create new session
    pub fn new_session(&mut self, name: impl Into<String>) {
        self.current = SessionState::new(name);
        self.session_path = None;
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // WindowGeometry Tests
    // =========================================================================

    #[test]
    fn test_window_geometry_default() {
        let geo = WindowGeometry::default();
        assert_eq!(geo.width, 1280);
        assert_eq!(geo.height, 800);
        assert!(!geo.maximized);
    }

    #[test]
    fn test_window_geometry_new() {
        let geo = WindowGeometry::new(50, 100, 800, 600);
        assert_eq!(geo.x, 50);
        assert_eq!(geo.y, 100);
        assert_eq!(geo.width, 800);
        assert_eq!(geo.height, 600);
    }

    #[test]
    fn test_window_geometry_maximized() {
        let geo = WindowGeometry::maximized();
        assert!(geo.maximized);
    }

    // =========================================================================
    // PanelState Tests
    // =========================================================================

    #[test]
    fn test_panel_state_creation() {
        let panel = PanelState::new("browser");
        assert_eq!(panel.id, "browser");
        assert!(panel.visible);
        assert!(!panel.collapsed);
    }

    #[test]
    fn test_panel_state_with_size() {
        let panel = PanelState::new("left").with_width(300.0).with_visible(true);

        assert_eq!(panel.width, Some(300.0));
        assert!(panel.visible);
    }

    // =========================================================================
    // DocumentState Tests
    // =========================================================================

    #[test]
    fn test_document_state_creation() {
        let doc = DocumentState::new(PathBuf::from("/test/file.sch"));
        assert_eq!(doc.path.to_str().unwrap(), "/test/file.sch");
        assert_eq!(doc.zoom, 1.0);
    }

    #[test]
    fn test_document_state_with_type() {
        let doc = DocumentState::new(PathBuf::from("/test/file.sch"))
            .with_type("schematic")
            .with_zoom(1.5);

        assert_eq!(doc.doc_type, "schematic");
        assert_eq!(doc.zoom, 1.5);
    }

    #[test]
    fn test_document_state_with_scroll() {
        let doc = DocumentState::new(PathBuf::from("/test/file.sch")).with_scroll(100.0, 200.0);

        assert_eq!(doc.scroll_x, 100.0);
        assert_eq!(doc.scroll_y, 200.0);
    }

    // =========================================================================
    // SessionState Tests
    // =========================================================================

    #[test]
    fn test_session_state_creation() {
        let session = SessionState::new("my_session");
        assert_eq!(session.name, "my_session");
        assert_eq!(session.version, SessionState::VERSION);
    }

    #[test]
    fn test_session_add_panel() {
        let mut session = SessionState::new("test");
        session.add_panel(PanelState::new("browser"));

        assert!(session.get_panel("browser").is_some());
    }

    #[test]
    fn test_session_add_document() {
        let mut session = SessionState::new("test");
        session.add_document(DocumentState::new(PathBuf::from("/file1.sch")));
        session.add_document(DocumentState::new(PathBuf::from("/file2.sch")));

        assert_eq!(session.document_count(), 2);
    }

    #[test]
    fn test_session_set_active_by_path() {
        let mut session = SessionState::new("test");
        let path1 = PathBuf::from("/file1.sch");
        let path2 = PathBuf::from("/file2.sch");

        session.add_document(DocumentState::new(path1.clone()));
        session.add_document(DocumentState::new(path2.clone()));

        session.set_active_by_path(&path2);
        assert_eq!(session.active_document, Some(1));
    }

    #[test]
    fn test_session_add_recent_file() {
        let mut session = SessionState::new("test");
        session.add_recent_file(PathBuf::from("/file1.sch"));
        session.add_recent_file(PathBuf::from("/file2.sch"));
        session.add_recent_file(PathBuf::from("/file1.sch")); // Duplicate

        assert_eq!(session.recent_files.len(), 2);
        assert_eq!(session.recent_files[0].to_str().unwrap(), "/file1.sch"); // Most recent
    }

    #[test]
    fn test_session_close_document() {
        let mut session = SessionState::new("test");
        let path1 = PathBuf::from("/file1.sch");
        let path2 = PathBuf::from("/file2.sch");

        session.add_document(DocumentState::new(path1.clone()));
        session.add_document(DocumentState::new(path2.clone()));
        session.active_document = Some(1);

        session.close_document(&path2);
        assert_eq!(session.document_count(), 1);
        assert_eq!(session.active_document, Some(0));
    }

    #[test]
    fn test_session_has_unsaved_changes() {
        let mut session = SessionState::new("test");

        let mut doc = DocumentState::new(PathBuf::from("/file.sch"));
        doc.modified = true;
        session.add_document(doc);

        assert!(session.has_unsaved_changes());
    }

    #[test]
    fn test_session_preferences() {
        let mut session = SessionState::new("test");
        session.set_preference("theme", "dark");

        assert_eq!(session.get_preference("theme"), Some(&"dark".to_string()));
    }

    // =========================================================================
    // SessionManager Tests
    // =========================================================================

    #[test]
    fn test_manager_creation() {
        let mgr = SessionManager::new();
        assert!(mgr.auto_save_enabled);
        assert_eq!(mgr.auto_save_interval, 300);
    }

    #[test]
    fn test_manager_auto_save_due() {
        let mut mgr = SessionManager::new();
        mgr.last_auto_save = Some(1000);
        mgr.auto_save_interval = 300;

        assert!(!mgr.auto_save_due(1200)); // Not yet
        assert!(mgr.auto_save_due(1400)); // Past interval
    }

    #[test]
    fn test_manager_auto_save_disabled() {
        let mut mgr = SessionManager::new();
        mgr.auto_save_enabled = false;

        assert!(!mgr.auto_save_due(9999));
    }

    #[test]
    fn test_manager_record_auto_save() {
        let mut mgr = SessionManager::new();
        mgr.record_auto_save(5000);

        assert_eq!(mgr.last_auto_save, Some(5000));
        assert_eq!(mgr.current.last_saved, Some(5000));
    }

    #[test]
    fn test_manager_to_json() {
        let mgr = SessionManager::new();
        let json = mgr.to_json().unwrap();

        assert!(json.contains("\"version\""));
        assert!(json.contains("default"));
    }

    #[test]
    fn test_manager_from_json() {
        let mut mgr = SessionManager::new();
        mgr.current_mut().set_preference("test", "value");

        let json = mgr.to_json().unwrap();

        let mut mgr2 = SessionManager::new();
        mgr2.from_json(&json).unwrap();

        assert_eq!(
            mgr2.current().get_preference("test"),
            Some(&"value".to_string())
        );
    }

    #[test]
    fn test_manager_new_session() {
        let mut mgr = SessionManager::new();
        mgr.current_mut()
            .add_document(DocumentState::new(PathBuf::from("/test")));

        mgr.new_session("fresh");
        assert_eq!(mgr.current().document_count(), 0);
        assert_eq!(mgr.current().name, "fresh");
    }
}
