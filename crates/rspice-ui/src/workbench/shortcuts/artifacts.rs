//! Pure shortcut portability contracts.
//!
//! This module owns deterministic, versioned artifact projection and
//! transactional import planning. It deliberately has no egui rendering or
//! platform IO so desktop, browser, and mobile surfaces consume one model.

mod import_io;
mod io;
mod markdown;
mod merge;
mod pdf;
mod projection;
mod schema;
mod vscode;

#[cfg(not(target_arch = "wasm32"))]
pub use import_io::import_shortcut_artifact_source;
#[cfg(target_arch = "wasm32")]
pub use import_io::{
    BrowserShortcutArtifactImportCancelOutcome, cancel_browser_shortcut_artifact_import,
    poll_browser_shortcut_artifact_import, start_browser_shortcut_artifact_import,
};
pub use import_io::{ReadyShortcutArtifactSource, ShortcutArtifactImportOutcome};
pub use io::{
    PreparedShortcutArtifact, ShortcutArtifactExportOutcome, ShortcutArtifactFormat,
    export_shortcut_artifact, prepare_shortcut_artifact,
};
pub use merge::{
    ImportBindingClass, ShortcutConflictPolicy, ShortcutImportOptions, ShortcutImportPlan,
    ShortcutImportReceipt, ShortcutMergePolicy, apply_shortcut_import, plan_shortcut_import,
    rollback_shortcut_import, shortcut_library_digest,
};
pub use projection::{ShortcutExportRequest, ShortcutExportScope, build_shortcut_reference_model};
pub use schema::DecodedShortcutArtifact;
// Both serializers are reached through their own modules by the artifact
// writers; only the tests name them from here, and the modules are private.
#[cfg(test)]
pub use projection::serialize_shortcut_reference_json;
#[cfg(test)]
pub use schema::decode_shortcut_artifact_json;
pub use vscode::{
    DetectedShortcutArtifact, VscodeAdapterError, VscodeHostPlatform, VscodeImportReport,
    detect_shortcut_artifact,
};

use std::collections::BTreeMap;

use serde_json::Value;
use sha2::{Digest, Sha256};

pub(crate) fn canonicalize_value(value: Value) -> Value {
    match value {
        Value::Object(values) => Value::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, canonicalize_value(value)))
                .collect::<BTreeMap<_, _>>()
                .into_iter()
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.into_iter().map(canonicalize_value).collect()),
        scalar => scalar,
    }
}

pub(crate) fn canonical_json_bytes(value: Value) -> Result<Vec<u8>, serde_json::Error> {
    let mut bytes = serde_json::to_vec_pretty(&canonicalize_value(value))?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

pub(crate) fn hex_digest(digest: [u8; 32]) -> String {
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
