//! Canonical PDK installation configuration and pure settings mutations.

use super::PdkTechnologyRegistry;
use rspice_model_library::pdk::display_profile::PdkDisplayProfileRegistry;
use rspice_model_library::pdk::technology_draft::PdkTechnologyDraft;
use rspice_model_library::pdk::{DiscoveredFile, LibraryPathEntry, PdkPublisherTrustStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// PDK installation configuration and retained runtime validation state.
/// Host discovery and persistence operate on this canonical record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkConfig {
    /// Configured library search paths
    pub library_paths: Vec<LibraryPathEntry>,

    /// Environment variable overrides (e.g., PDK_HOME -> /opt/tsmc180)
    pub environment_variables: HashMap<String, String>,

    /// Recently loaded files for quick access
    pub recent_files: Vec<PathBuf>,

    /// Maximum number of recent files to remember
    #[serde(default = "default_max_recent")]
    pub max_recent_files: usize,

    /// Physical length of one layout database unit, supplied by the active
    /// PDK technology configuration. Absence is preserved rather than
    /// substituting a guessed manufacturing grid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_database_unit: Option<rspice_app_types::quantity::LayoutDatabaseUnit>,

    /// Signed technology-package revisions, exact active binding, and
    /// append-only administrative receipts. Persisted archives regain no
    /// runtime authority until revalidated against the current trust store.
    #[serde(default)]
    pub technology_registry: PdkTechnologyRegistry,

    /// Organization- or administrator-provisioned public publisher keys.
    /// These are verification keys only; private signing material is never
    /// accepted or persisted by RSpice.
    #[serde(default)]
    pub publisher_trust_store: PdkPublisherTrustStore,

    /// Personal-device display overlays. Every immutable revision is bound to
    /// an exact signed technology manifest and has its own hash-chained audit.
    /// Project and organization scopes remain fail-closed until their
    /// repository and policy authorities exist.
    #[serde(default)]
    pub display_profile_registry: PdkDisplayProfileRegistry,

    /// One unsigned authoring candidate derived from an exact trusted package.
    /// The draft has no runtime authority and may be invalid between edits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technology_draft: Option<PdkTechnologyDraft>,

    /// Discovered files from last scan (not persisted by default)
    #[serde(skip)]
    pub discovered_files: Vec<DiscoveredFile>,

    /// Scan errors from last discovery (not persisted)
    #[serde(skip)]
    pub scan_errors: Vec<String>,

    /// Canonical root files admitted by the last successful atomic PDK
    /// application. This is manager ownership provenance, not a discovery
    /// cache: it remains persisted when a configured directory or file later
    /// disappears so the stale external library can still be unloaded without
    /// touching manually attached sources.
    #[serde(default)]
    pub managed_model_sources: Vec<PathBuf>,
}

fn default_max_recent() -> usize {
    20
}

impl Default for PdkConfig {
    fn default() -> Self {
        Self {
            library_paths: Vec::new(),
            environment_variables: HashMap::new(),
            recent_files: Vec::new(),
            max_recent_files: default_max_recent(),
            layout_database_unit: None,
            technology_registry: PdkTechnologyRegistry::default(),
            publisher_trust_store: PdkPublisherTrustStore::default(),
            display_profile_registry: PdkDisplayProfileRegistry::default(),
            technology_draft: None,
            discovered_files: Vec::new(),
            scan_errors: Vec::new(),
            managed_model_sources: Vec::new(),
        }
    }
}

impl PdkConfig {
    // =========================================================================
    // Library Path Management
    // =========================================================================

    /// Add a library search path
    pub fn add_library_path(&mut self, path: impl Into<String>) {
        let entry = LibraryPathEntry::new(path);
        if !self.library_paths.iter().any(|e| e.path == entry.path) {
            self.library_paths.push(entry);
        }
    }

    /// Remove a library path by index
    pub fn remove_library_path(&mut self, index: usize) -> Option<LibraryPathEntry> {
        if index < self.library_paths.len() {
            Some(self.library_paths.remove(index))
        } else {
            None
        }
    }

    // =========================================================================
    // Environment Variables
    // =========================================================================

    /// Set an environment variable override
    pub fn set_env_var(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.environment_variables.insert(name.into(), value.into());
    }

    /// Remove an environment variable
    pub fn remove_env_var(&mut self, name: &str) -> Option<String> {
        self.environment_variables.remove(name)
    }
}

impl PdkConfig {
    // =========================================================================
    // Dialog Integration Accessors
    // =========================================================================

    /// Get library paths (immutable reference)
    pub fn library_paths(&self) -> &[LibraryPathEntry] {
        &self.library_paths
    }

    /// Get library paths (mutable reference)
    pub fn library_paths_mut(&mut self) -> &mut Vec<LibraryPathEntry> {
        &mut self.library_paths
    }

    /// Get environment variable overrides
    pub fn env_overrides(&self) -> &HashMap<String, String> {
        &self.environment_variables
    }

    /// Get discovered files
    pub fn discovered_files(&self) -> &[DiscoveredFile] {
        &self.discovered_files
    }

    /// Toggle path enabled state
    pub fn toggle_path_enabled(&mut self, index: usize) {
        if let Some(entry) = self.library_paths.get_mut(index) {
            entry.enabled = !entry.enabled;
        }
    }

    /// Toggle path recursive state
    pub fn toggle_path_recursive(&mut self, index: usize) {
        if let Some(entry) = self.library_paths.get_mut(index) {
            entry.recursive = !entry.recursive;
        }
    }

    /// Alias for set_env_var for dialog compatibility
    pub fn set_env_override(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.set_env_var(name, value);
    }

    /// Alias for remove_env_var for dialog compatibility
    pub fn remove_env_override(&mut self, name: &str) {
        self.remove_env_var(name);
    }
}

impl PdkConfig {
    // =========================================================================
    // Recent Files
    // =========================================================================

    /// Add a file to the recent files list
    pub fn add_recent_file(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();

        // Remove if already exists (will re-add at front)
        self.recent_files.retain(|p| p != &path);

        // Add to front
        self.recent_files.insert(0, path);

        // Trim to max size
        self.recent_files.truncate(self.max_recent_files);
    }
}
