//! Signed-package import preparation and checked native/browser completions.
//!
//! The application chooses files and publishes configuration; this service owns
//! background validation, worker transport, and stale-candidate rejection.

use crate::state::pdk_config::{MAX_PDK_ARCHIVE_BYTES, PdkAdministrativeAuthority, PdkConfig};

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::{start_native_package_import, take_native_package_imports};

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) use browser::run_pdk_import_worker_request_value;
#[cfg(target_arch = "wasm32")]
pub(crate) use browser::{
    cancel_browser_package_import, fail_browser_package_import, start_browser_package_import,
    take_browser_package_imports,
};

pub(crate) struct PackageImportCandidate {
    pub(crate) config: PdkConfig,
    pub(crate) package_id: String,
    pub(crate) revision: String,
    pub(crate) sequence: u64,
}
