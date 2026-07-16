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

#[cfg(target_arch = "wasm32")]
pub use import_io::{
    BrowserShortcutArtifactImportCancelOutcome, cancel_browser_shortcut_artifact_import,
    poll_browser_shortcut_artifact_import, start_browser_shortcut_artifact_import,
};
pub use import_io::{
    MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES, ReadyShortcutArtifactSource, ShortcutArtifactImportError,
    ShortcutArtifactImportErrorKind, ShortcutArtifactImportOutcome,
};
#[cfg(not(target_arch = "wasm32"))]
pub use import_io::{
    NativeShortcutArtifactImportIo, ShortcutArtifactImportIo, import_shortcut_artifact_source,
    import_shortcut_artifact_source_with_io,
};
pub use io::{
    MAX_SHORTCUT_JSON_ARTIFACT_BYTES, MAX_SHORTCUT_MARKDOWN_ARTIFACT_BYTES,
    MAX_SHORTCUT_PDF_ARTIFACT_BYTES, PreparedShortcutArtifact, ShortcutArtifactExportOutcome,
    ShortcutArtifactFormat, ShortcutArtifactIoError, export_shortcut_artifact,
    normalize_shortcut_artifact_filename, prepare_shortcut_artifact,
};
pub use markdown::serialize_shortcut_reference_markdown;
pub use merge::{
    ImportBindingClass, ImportClassSummary, ShortcutConflictPolicy, ShortcutImportApplyError,
    ShortcutImportConflict, ShortcutImportConflictKind, ShortcutImportDecision,
    ShortcutImportOptions, ShortcutImportPlan, ShortcutImportPlanError, ShortcutImportReceipt,
    ShortcutMergePolicy, apply_shortcut_import, plan_shortcut_import, rollback_shortcut_import,
    shortcut_library_digest,
};
pub use pdf::{ShortcutPdfError, serialize_shortcut_reference_pdf};
pub use projection::{
    ShortcutExportError, ShortcutExportRequest, ShortcutExportScope, ShortcutReferenceModel,
    ShortcutReferenceRow, build_shortcut_reference_model, serialize_shortcut_reference_json,
};
pub use schema::{
    DecodedShortcutArtifact, SHORTCUT_ARTIFACT_FORMAT, SHORTCUT_ARTIFACT_SCHEMA_VERSION,
    ShortcutArtifactCoverage, ShortcutArtifactManifest, ShortcutArtifactSchemaError,
    decode_shortcut_artifact_json,
};
pub use vscode::{
    DetectedRspiceShortcutArtifact, DetectedShortcutArtifact, VSCODE_COMMAND_MAPPINGS,
    VSCODE_MAPPING_VERSION, VscodeAdapterError, VscodeCommandMapping, VscodeDiagnosticCode,
    VscodeDiagnosticSeverity, VscodeEntryDiagnostic, VscodeEntryDisposition, VscodeEntryOutcome,
    VscodeHostPlatform, VscodeImportReport, VscodeShortcutAdaptation, adapt_vscode_keybindings,
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
