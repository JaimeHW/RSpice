//! PDK Configuration Management
//!
//! Commercial-grade Process Design Kit (PDK) path configuration with persistent
//! storage, environment variable expansion, and automatic model file discovery.
//!
//! # Architecture
//!
//! Matches Cadence Spectre's model library management:
//! - **Library Paths**: User-configured directories to scan for model files
//! - **Environment Variables**: `$PDK_HOME`, `$MY_TECH` style variable support
//! - **File Discovery**: Automatic scanning for `.lib`, `.scs`, `.mod` files
//! - **Persistence**: JSON-based configuration saved to user's config directory
//!
//! # Usage
//!
//! ```rust
//! use rspice_ui::state::pdk_config::{PdkConfig, DiscoveredFile};
//!
//! let mut config = PdkConfig::new();
//! config.add_library_path("/path/to/pdk/models");
//! config.set_env_var("PDK_HOME", "/opt/tsmc180");
//!
//! // Expand paths with environment variables
//! let expanded = config.expand_path("$PDK_HOME/models/nmos.lib");
//!
//! // Discover model files in configured paths
//! let files = config.discover_model_files();
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

mod accessors;
mod discovered_file;
mod discovery;
mod display_profile;
mod errors;
mod library_path;
mod paths;
mod persistence;
mod recent;
mod technology_callback;
mod technology_diff;
mod technology_package;

pub use discovered_file::DiscoveredFile;
pub use display_profile::{
    PdkDisplayFillStyle, PdkDisplayLayerStyle, PdkDisplayProfileAuditAction,
    PdkDisplayProfileAuditReceipt, PdkDisplayProfileBinding, PdkDisplayProfileDraft,
    PdkDisplayProfileRegistry, PdkDisplayProfileRevision, PdkDisplayProfileScope,
};
pub use errors::ConfigError;
pub use library_path::LibraryPathEntry;
#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::{
    BrowserPdkConfigReceipt, BrowserPdkConfigRestore, BrowserPdkStorageDurability,
    BrowserPdkStorageStatus, start_browser_pdk_config_load, start_browser_pdk_config_save,
};
pub use technology_callback::{
    MAX_PROJECT_PDK_CALLBACK_RECEIPTS, PdkCallbackError, PdkCallbackExecutionInput,
    PdkCallbackExecutionReceipt, ProjectPdkCallbackReceipt,
};
#[cfg(test)]
pub(crate) use technology_diff::tests::fixture_revision_archives as signed_technology_diff_test_fixture;
pub use technology_diff::{
    PdkTechnologyDiffArea, PdkTechnologyDiffEntry, PdkTechnologyDiffError, PdkTechnologyDiffImpact,
    PdkTechnologyDiffKind, PdkTechnologyRevisionDiff,
};
#[cfg(test)]
pub(crate) use technology_package::tests::fixture_archive as signed_technology_test_fixture;
#[cfg(test)]
pub(crate) use technology_package::tests::fixture_archive_with_veriloga as signed_veriloga_technology_test_fixture;
pub use technology_package::{
    MAX_PDK_ARCHIVE_BYTES, MAX_PDK_ARTIFACT_BYTES, MAX_PDK_ARTIFACTS, MAX_PDK_TOTAL_ARTIFACT_BYTES,
    PdkAdministrativeAuthority, PdkExecutionTarget, PdkExtractionQuantity, PdkModelProcess,
    PdkPublisherTrustStore, PdkTechnologyArtifactKind, PdkTechnologyAuditAction,
    PdkTechnologyAuditReceipt, PdkTechnologyBinding, PdkTechnologyLayer, PdkTechnologyRegistry,
    PdkTrustAuditAction, PdkTrustAuditReceipt, TrustedPdkPublisherKey,
    ValidatedPdkTechnologyPackage,
};
pub(crate) use technology_package::{
    SealedPdkModelProcessBinding, SealedPdkModelSources, SealedPdkVerilogAArtifact,
    SealedPdkVerilogABinding,
};
// =============================================================================
// Constants
// =============================================================================

/// Supported model file extensions
pub const MODEL_FILE_EXTENSIONS: &[&str] = &["lib", "scs", "mod", "sp", "cir"];

/// Default configuration file name
pub const CONFIG_FILE_NAME: &str = "pdk_config.json";

/// Maximum directory recursion depth for scanning
pub const MAX_SCAN_DEPTH: usize = 10;

// =============================================================================
// PDK Configuration
// =============================================================================

/// PDK configuration with library paths and environment variables
///
/// Provides persistent storage and automatic model file discovery
/// matching Cadence Spectre's model library management workflow.
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
    pub layout_database_unit: Option<crate::quantity::LayoutDatabaseUnit>,

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
            discovered_files: Vec::new(),
            scan_errors: Vec::new(),
            managed_model_sources: Vec::new(),
        }
    }
}

impl PdkConfig {
    /// Create a new empty configuration
    #[cfg(test)]
    pub fn new() -> Self {
        Self::default()
    }
}
