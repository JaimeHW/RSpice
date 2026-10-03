//! PDK host discovery/persistence adapters and canonical installation types.

mod discovery;
#[cfg(test)]
mod display_profile_tests;
mod errors;
mod paths;
mod persistence;
mod technology_callback;
mod technology_diff;
#[cfg(test)]
mod technology_draft_tests;

pub(crate) use discovery::discover_model_files;
#[cfg(test)]
pub(crate) use discovery::discovered_file;
pub use errors::ConfigError;
pub(crate) use paths::expand_path;
#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::default_config_path;
#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub(crate) use persistence::load;
pub(crate) use persistence::save;
#[cfg(target_arch = "wasm32")]
pub(crate) use persistence::{
    BrowserPdkConfigReceipt, BrowserPdkConfigRestore, BrowserPdkStorageDurability,
    BrowserPdkStorageStatus, start_browser_pdk_config_load, start_browser_pdk_config_save,
};
#[cfg(test)]
pub use rspice_model_library::pdk::PdkPublisherTrustStore;
#[cfg(any(test, target_arch = "wasm32"))]
pub use rspice_model_library::pdk::contracts::MAX_PDK_ARCHIVE_BYTES;
pub use rspice_model_library::pdk::contracts::{
    PdkConnectivityEdge, PdkExecutionTarget, PdkExtractionContract,
    PdkExtractionQualificationVector, PdkExtractionQuantity, PdkLayerAlias, PdkLayerKind,
    PdkLayerPurposeRef, PdkRecognitionContract, PdkRecognitionQualificationVector,
    PdkRecognitionTerminal, PdkStreamMapEntry, PdkTechnologyArtifactKind, PdkTechnologyBinding,
    PdkTechnologyLayer, PdkViaDefinition,
};
pub use rspice_model_library::pdk::display_profile::{
    PdkDisplayFillStyle, PdkDisplayLayerStyle, PdkDisplayProfileAuditAction,
    PdkDisplayProfileAuditReceipt, PdkDisplayProfileBinding, PdkDisplayProfileDraft,
    PdkDisplayProfileRevision, PdkDisplayProfileScope,
};
pub use rspice_model_library::pdk::manifest::PdkTechnologyManifest;
pub use rspice_model_library::pdk::technology_draft::PdkTechnologyDraft;
pub use rspice_model_library::pdk::{DiscoveredFile, LibraryPathEntry};
pub use rspice_model_library::pdk::{
    PdkAdministrativeAuthority, PdkTrustAuditAction, PdkTrustAuditReceipt, TrustedPdkPublisherKey,
};
#[cfg(test)]
pub use rspice_simulation::pdk::PdkTechnologyRegistry;
#[cfg(test)]
pub(crate) use rspice_simulation::pdk::test_fixtures::{
    fixture_archive as signed_technology_test_fixture,
    fixture_archive_with_symbols as signed_symbol_technology_test_fixture,
    fixture_archive_with_veriloga as signed_veriloga_technology_test_fixture,
    fixture_archive_with_veriloga_source as signed_veriloga_source_test_fixture,
};
pub use rspice_simulation::pdk::{
    PdkConfig, PdkTechnologyAuditAction, PdkTechnologyAuditReceipt, ValidatedPdkTechnologyPackage,
};
pub use technology_callback::{PdkCallbackExecutionInput, ProjectPdkCallbackReceipt};
#[cfg(test)]
pub(crate) use technology_diff::tests::fixture_revision_archives as signed_technology_diff_test_fixture;
pub use technology_diff::{
    PdkTechnologyDiffArea, PdkTechnologyDiffEntry, PdkTechnologyDiffError, PdkTechnologyDiffImpact,
    PdkTechnologyDiffKind, PdkTechnologyMigrationEvidence, PdkTechnologyRevisionDiff,
};

/// Supported model file extensions.
pub const MODEL_FILE_EXTENSIONS: &[&str] = &["lib", "scs", "mod", "sp", "cir"];
/// Default configuration file name.
pub const CONFIG_FILE_NAME: &str = "pdk_config.json";
/// Maximum directory recursion depth for scanning.
pub const MAX_SCAN_DEPTH: usize = 10;
