//! Signed-package import preparation and checked native/browser completions.
//!
//! The application chooses files and publishes configuration; this service owns
//! background validation, worker transport, and stale-candidate rejection.

use super::PdkConfig;
use rspice_model_library::pdk::{PdkAdministrativeAuthority, contracts::MAX_PDK_ARCHIVE_BYTES};

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::{start_native_package_import, take_native_package_imports};

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
pub use browser::run_pdk_import_worker_request_value;
#[cfg(target_arch = "wasm32")]
pub use browser::{
    cancel_browser_package_import, fail_browser_package_import, start_browser_package_import,
    take_browser_package_imports,
};

pub struct PackageImportCandidate {
    pub config: PdkConfig,
    pub package_id: String,
    pub revision: String,
    pub sequence: u64,
}
