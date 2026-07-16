//! Portable shortcut-profile import and export.
//!
//! Parsing is deliberately separated from application state mutation. An
//! import produces a staged candidate and its complete audit; the caller owns
//! the explicit decision to apply that candidate.

use std::collections::BTreeMap;
use std::fmt;
#[cfg(any(test, not(target_arch = "wasm32")))]
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;

use crate::common::export_workflow::{ExportWorkflowIo, NativeExportWorkflowIo, SaveDialogConfig};
use crate::workbench::shortcuts::MAX_SHORTCUT_SEQUENCE_STROKES;
use crate::workbench::{ShortcutPreferences, ShortcutProfileAudit};

pub const SHORTCUT_PROFILE_FORMAT: &str = "rspice.shortcuts/1";
pub const MAX_SHORTCUT_PROFILE_BYTES: u64 = 512 * 1024;
pub const MAX_SHORTCUT_COMMAND_RECORDS: usize = 2_048;
pub const MAX_SHORTCUT_BINDING_RECORDS: usize = 4_096;

const SHORTCUT_PROFILE_FILTER_NAME: &str = "RSpice Shortcut Profile";
const SHORTCUT_PROFILE_FILTER_EXTENSIONS: &[&str] = &["json"];
const SHORTCUT_PROFILE_DEFAULT_NAME: &str = "rspice-shortcuts.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutProfileWorkflowError {
    code: &'static str,
    path: String,
    context: String,
}

impl ShortcutProfileWorkflowError {
    fn new(code: &'static str, path: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            code,
            path: path.into(),
            context: context.into(),
        }
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    pub fn json_path(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn context(&self) -> &str {
        &self.context
    }
}

impl fmt::Display for ShortcutProfileWorkflowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {} [{}]", self.path, self.context, self.code)
    }
}

impl std::error::Error for ShortcutProfileWorkflowError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedShortcutProfile {
    source_name: String,
    candidate: ShortcutPreferences,
    audit: ShortcutProfileAudit,
}

impl StagedShortcutProfile {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn candidate(&self) -> &ShortcutPreferences {
        &self.candidate
    }

    #[must_use]
    pub const fn audit(&self) -> &ShortcutProfileAudit {
        &self.audit
    }

    #[must_use]
    pub fn into_candidate(self) -> ShortcutPreferences {
        self.candidate
    }
}

/// Serialize a portable, deterministic `rspice.shortcuts/1` document.
pub fn serialize_shortcut_profile(
    profile: &ShortcutPreferences,
) -> Result<String, ShortcutProfileWorkflowError> {
    let mut profile = serde_json::to_value(profile).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.serialize",
            "$.profile",
            format!("could not serialize shortcut profile: {error}"),
        )
    })?;
    strip_local_shortcut_state(&mut profile);
    validate_profile_limits(&profile, "$.profile")?;

    let mut envelope = BTreeMap::new();
    envelope.insert(
        "format".to_owned(),
        Value::String(SHORTCUT_PROFILE_FORMAT.to_owned()),
    );
    envelope.insert("profile".to_owned(), profile);
    let canonical = canonicalize_value(Value::Object(envelope.into_iter().collect()));
    let mut encoded = serde_json::to_string_pretty(&canonical).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.serialize",
            "$",
            format!("could not encode shortcut profile document: {error}"),
        )
    })?;
    encoded.push('\n');
    ensure_byte_limit(encoded.len() as u64, "$", "encoded profile")?;
    Ok(encoded)
}

/// Parse and audit an import without mutating live preferences or `AppState`.
pub fn stage_shortcut_profile_json(
    source_name: impl Into<String>,
    contents: &str,
) -> Result<StagedShortcutProfile, ShortcutProfileWorkflowError> {
    ensure_byte_limit(contents.len() as u64, "$", "selected profile")?;
    let root: Value = serde_json::from_str(contents).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.invalid-json",
            "$",
            format!(
                "invalid JSON at line {}, column {}: {}",
                error.line(),
                error.column(),
                error
            ),
        )
    })?;
    let root = root.as_object().ok_or_else(|| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.invalid-envelope",
            "$",
            "shortcut profile document must be a JSON object",
        )
    })?;
    let format = root.get("format").ok_or_else(|| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.missing-format",
            "$.format",
            format!("required format marker must be '{SHORTCUT_PROFILE_FORMAT}'"),
        )
    })?;
    let Some(format) = format.as_str() else {
        return Err(ShortcutProfileWorkflowError::new(
            "shortcut-profile.invalid-format",
            "$.format",
            "format marker must be a string",
        ));
    };
    if format != SHORTCUT_PROFILE_FORMAT {
        return Err(ShortcutProfileWorkflowError::new(
            "shortcut-profile.unsupported-format",
            "$.format",
            format!("unsupported format '{format}'; expected '{SHORTCUT_PROFILE_FORMAT}'"),
        ));
    }
    let profile = root.get("profile").ok_or_else(|| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.missing-profile",
            "$.profile",
            "shortcut profile document has no profile payload",
        )
    })?;
    let mut profile = profile.clone();
    strip_local_shortcut_state(&mut profile);
    validate_profile_limits(&profile, "$.profile")?;

    // ShortcutPreferences intentionally retains unknown and malformed raw
    // records. The resulting audit records anything this build cannot safely
    // execute while preserving the source for a newer build.
    let candidate: ShortcutPreferences = serde_json::from_value(profile).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.invalid-profile",
            "$.profile",
            format!("could not decode shortcut profile payload: {error}"),
        )
    })?;
    let audit = candidate.audit();
    Ok(StagedShortcutProfile {
        source_name: normalized_source_name(source_name.into()),
        candidate,
        audit,
    })
}

/// Protected-override acknowledgement is a device-local security decision,
/// never portable profile data. Keep this scrub at the legacy boundary even
/// after richer artifact workflows are added so dormant callers stay safe.
fn strip_local_shortcut_state(profile: &mut Value) {
    if let Some(profile) = profile.as_object_mut() {
        profile.remove("protected-override-acknowledgements");
    }
}

/// Export through the shared observed-destination contract. Native production
/// IO therefore publishes durably with an exact picker-time CAS precondition.
pub(crate) fn export_shortcut_profile_with_io(
    profile: &ShortcutPreferences,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> Result<Option<PathBuf>, ShortcutProfileWorkflowError> {
    let encoded = serialize_shortcut_profile(profile)?;
    let config = SaveDialogConfig {
        title: "Export Shortcut Profile",
        default_name: SHORTCUT_PROFILE_DEFAULT_NAME,
        filter_name: SHORTCUT_PROFILE_FILTER_NAME,
        filter_extensions: SHORTCUT_PROFILE_FILTER_EXTENSIONS,
    };
    let Some(path) = io.show_save_dialog(config).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.export-picker",
            "$destination",
            format!("could not open shortcut profile save picker: {error}"),
        )
    })?
    else {
        return Ok(None);
    };
    let destination = io.observe_destination(&path).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.export-observe",
            "$destination",
            format!("could not observe '{}': {error}", path.display()),
        )
    })?;
    io.write_text_file_observed(&destination, &encoded)
        .map_err(|error| {
            ShortcutProfileWorkflowError::new(
                "shortcut-profile.export-publish",
                "$destination",
                format!("could not publish '{}': {error}", path.display()),
            )
        })?;
    Ok(Some(path))
}

pub fn export_shortcut_profile(
    profile: &ShortcutPreferences,
) -> Result<Option<PathBuf>, ShortcutProfileWorkflowError> {
    export_shortcut_profile_with_io(profile, &NativeExportWorkflowIo)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) trait ShortcutProfileImportIo {
    fn show_open_dialog(&self) -> Result<Option<PathBuf>, String>;
    fn read_exact_bytes(&self, path: &Path) -> Result<Vec<u8>, String>;
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NativeShortcutProfileImportIo;

#[cfg(not(target_arch = "wasm32"))]
impl ShortcutProfileImportIo for NativeShortcutProfileImportIo {
    fn show_open_dialog(&self) -> Result<Option<PathBuf>, String> {
        Ok(rfd::FileDialog::new()
            .add_filter(
                SHORTCUT_PROFILE_FILTER_NAME,
                SHORTCUT_PROFILE_FILTER_EXTENSIONS,
            )
            .add_filter("All Files", &["*"])
            .set_title("Import Shortcut Profile")
            .pick_file())
    }

    fn read_exact_bytes(&self, path: &Path) -> Result<Vec<u8>, String> {
        use std::io::Read as _;

        let metadata = std::fs::metadata(path).map_err(|error| error.to_string())?;
        if metadata.len() > MAX_SHORTCUT_PROFILE_BYTES {
            return Err(format!(
                "selected file is {} bytes; maximum is {} bytes",
                metadata.len(),
                MAX_SHORTCUT_PROFILE_BYTES
            ));
        }
        let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.take(MAX_SHORTCUT_PROFILE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| error.to_string())?;
        if bytes.len() as u64 > MAX_SHORTCUT_PROFILE_BYTES {
            return Err(format!(
                "selected file changed while being read and exceeds the {}-byte maximum",
                MAX_SHORTCUT_PROFILE_BYTES
            ));
        }
        Ok(bytes)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn import_shortcut_profile_with_io(
    io: &(impl ShortcutProfileImportIo + ?Sized),
) -> Result<Option<StagedShortcutProfile>, ShortcutProfileWorkflowError> {
    let Some(path) = io.show_open_dialog().map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.import-picker",
            "$source",
            format!("could not open shortcut profile picker: {error}"),
        )
    })?
    else {
        return Ok(None);
    };
    let bytes = io.read_exact_bytes(&path).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.import-read",
            "$source",
            format!("could not read '{}': {error}", path.display()),
        )
    })?;
    ensure_byte_limit(bytes.len() as u64, "$", "selected profile")?;
    let contents = String::from_utf8(bytes).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.invalid-utf8",
            "$",
            format!("'{}' is not valid UTF-8: {error}", path.display()),
        )
    })?;
    stage_shortcut_profile_json(display_name(&path), &contents).map(Some)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn import_shortcut_profile()
-> Result<Option<StagedShortcutProfile>, ShortcutProfileWorkflowError> {
    import_shortcut_profile_with_io(&NativeShortcutProfileImportIo)
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug)]
struct BrowserShortcutProfileImportCompletion {
    token: crate::common::browser_file_import::TextImportToken,
    result: Result<Option<StagedShortcutProfile>, ShortcutProfileWorkflowError>,
}

#[cfg(any(test, target_arch = "wasm32"))]
thread_local! {
    static BROWSER_SHORTCUT_PROFILE_IMPORT_RESULT: std::cell::RefCell<Option<BrowserShortcutProfileImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
pub fn start_browser_shortcut_profile_import() -> Result<(), String> {
    let token = crate::common::browser_file_import::try_begin_text_import(
        crate::common::browser_file_import::BrowserTextImportKind::ShortcutProfile,
    )?;
    BROWSER_SHORTCUT_PROFILE_IMPORT_RESULT.with(|slot| {
        *slot.borrow_mut() = None;
    });
    crate::common::browser_file_import::pick_text_file(
        SHORTCUT_PROFILE_FILTER_NAME,
        SHORTCUT_PROFILE_FILTER_EXTENSIONS,
        move |result| {
            if !crate::common::browser_file_import::text_import_is_current(token) {
                return;
            }
            let result = match result {
                Ok(Some(file)) => stage_shortcut_profile_json(file.name, &file.contents).map(Some),
                Ok(None) => Ok(None),
                Err(error) => Err(ShortcutProfileWorkflowError::new(
                    "shortcut-profile.import-read",
                    "$source",
                    error,
                )),
            };
            complete_browser_shortcut_profile_import(token, result);
        },
    );
    Ok(())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn complete_browser_shortcut_profile_import(
    token: crate::common::browser_file_import::TextImportToken,
    result: Result<Option<StagedShortcutProfile>, ShortcutProfileWorkflowError>,
) {
    if !crate::common::browser_file_import::text_import_is_current(token) {
        return;
    }
    BROWSER_SHORTCUT_PROFILE_IMPORT_RESULT.with(|slot| {
        *slot.borrow_mut() = Some(BrowserShortcutProfileImportCompletion { token, result });
    });
}

#[cfg(any(test, target_arch = "wasm32"))]
pub fn poll_browser_shortcut_profile_import()
-> Option<Result<Option<StagedShortcutProfile>, ShortcutProfileWorkflowError>> {
    let completion =
        BROWSER_SHORTCUT_PROFILE_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())?;
    if !crate::common::browser_file_import::finish_text_import(completion.token) {
        return None;
    }
    Some(completion.result)
}

fn validate_profile_limits(
    profile: &Value,
    profile_path: &str,
) -> Result<(), ShortcutProfileWorkflowError> {
    let Some(commands) = profile.get("commands") else {
        return Ok(());
    };
    let Some(commands) = commands.as_object() else {
        // This is retained by ShortcutPreferences and reported by its audit.
        return Ok(());
    };
    if commands.len() > MAX_SHORTCUT_COMMAND_RECORDS {
        return Err(limit_error(
            &format!("{profile_path}.commands"),
            "command records",
            commands.len(),
            MAX_SHORTCUT_COMMAND_RECORDS,
        ));
    }

    let mut binding_count = 0usize;
    for (command_id, command) in commands {
        let Some(bindings) = command.get("bindings").and_then(Value::as_array) else {
            continue;
        };
        binding_count = binding_count.saturating_add(bindings.len());
        if binding_count > MAX_SHORTCUT_BINDING_RECORDS {
            return Err(limit_error(
                &format!(
                    "{profile_path}.commands[{}].bindings",
                    json_path_key(command_id)
                ),
                "binding records",
                binding_count,
                MAX_SHORTCUT_BINDING_RECORDS,
            ));
        }
        for (binding_index, binding) in bindings.iter().enumerate() {
            let Some(sequence) = binding.get("sequence").and_then(Value::as_array) else {
                continue;
            };
            if sequence.len() > MAX_SHORTCUT_SEQUENCE_STROKES {
                return Err(limit_error(
                    &format!(
                        "{profile_path}.commands[{}].bindings[{binding_index}].sequence",
                        json_path_key(command_id)
                    ),
                    "sequence strokes",
                    sequence.len(),
                    MAX_SHORTCUT_SEQUENCE_STROKES,
                ));
            }
        }
    }
    Ok(())
}

fn ensure_byte_limit(
    byte_count: u64,
    path: &str,
    context: &str,
) -> Result<(), ShortcutProfileWorkflowError> {
    if byte_count > MAX_SHORTCUT_PROFILE_BYTES {
        return Err(ShortcutProfileWorkflowError::new(
            "shortcut-profile.byte-limit",
            path,
            format!(
                "{context} is {byte_count} bytes; maximum is {MAX_SHORTCUT_PROFILE_BYTES} bytes"
            ),
        ));
    }
    Ok(())
}

fn limit_error(
    path: &str,
    label: &str,
    actual: usize,
    maximum: usize,
) -> ShortcutProfileWorkflowError {
    ShortcutProfileWorkflowError::new(
        "shortcut-profile.record-limit",
        path,
        format!("{label} count is {actual}; maximum is {maximum}"),
    )
}

fn canonicalize_value(value: Value) -> Value {
    match value {
        Value::Object(values) => {
            let sorted = values
                .into_iter()
                .map(|(key, value)| (key, canonicalize_value(value)))
                .collect::<BTreeMap<_, _>>();
            Value::Object(sorted.into_iter().collect())
        }
        Value::Array(values) => Value::Array(values.into_iter().map(canonicalize_value).collect()),
        scalar => scalar,
    }
}

fn normalized_source_name(source_name: String) -> String {
    let trimmed = source_name.trim();
    if trimmed.is_empty() {
        "shortcut profile".to_owned()
    } else {
        trimmed.to_owned()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn display_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| path.display().to_string())
}

fn json_path_key(key: &str) -> String {
    serde_json::to_string(key).unwrap_or_else(|_| "\"<invalid-key>\"".to_owned())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::common::export_workflow::{ObservedExportDestination, SaveDialogConfig};

    const VALID_SOURCE: &str = r#"{
        "format": "rspice.shortcuts/1",
        "profile": {
            "commands": {
                "future-command": {
                    "bindings": [{
                        "slot": "primary",
                        "platforms": ["desktop"],
                        "sequence": [{"key": "F9"}],
                        "future-binding-field": {"preserve": true}
                    }],
                    "future-command-field": 73
                }
            },
            "future-profile-field": [1, 2, 3]
        }
    }"#;

    #[test]
    fn stage_is_transactional_and_preserves_unknown_future_commands() {
        let staged = stage_shortcut_profile_json("future.json", VALID_SOURCE).unwrap();
        assert_eq!(staged.source_name(), "future.json");
        assert!(staged.audit().issues().iter().any(|issue| {
            issue
                .command_id()
                .is_some_and(|id| id.as_str() == "future-command")
        }));

        let encoded = serialize_shortcut_profile(staged.candidate()).unwrap();
        let reparsed: Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(
            reparsed["profile"]["commands"]["future-command"]["future-command-field"],
            73
        );
        assert_eq!(
            reparsed["profile"]["commands"]["future-command"]["bindings"][0]["future-binding-field"]
                ["preserve"],
            true
        );
        assert_eq!(
            reparsed["profile"]["future-profile-field"],
            serde_json::json!([1, 2, 3])
        );
    }

    #[test]
    fn legacy_export_strips_device_local_protected_acknowledgements() {
        let mut profile = ShortcutPreferences::default();
        profile.acknowledge_protected_override(crate::workbench::commands::Command::Save);

        let encoded = serialize_shortcut_profile(&profile).unwrap();
        let document: Value = serde_json::from_str(&encoded).unwrap();

        assert!(
            document["profile"]
                .get("protected-override-acknowledgements")
                .is_none()
        );
    }

    #[test]
    fn legacy_import_never_trusts_transferred_protected_acknowledgements() {
        let staged = stage_shortcut_profile_json(
            "untrusted.json",
            r#"{
                "format":"rspice.shortcuts/1",
                "profile":{
                    "protected-override-acknowledgements":["save-project"],
                    "commands":{}
                }
            }"#,
        )
        .unwrap();

        assert!(
            !staged
                .candidate()
                .protected_override_acknowledged(crate::workbench::commands::Command::Save)
        );
        let reencoded = serialize_shortcut_profile(staged.candidate()).unwrap();
        assert!(!reencoded.contains("protected-override-acknowledgements"));
    }

    #[test]
    fn export_is_deterministic_and_versioned() {
        let staged = stage_shortcut_profile_json("future.json", VALID_SOURCE).unwrap();
        let first = serialize_shortcut_profile(staged.candidate()).unwrap();
        let second = serialize_shortcut_profile(staged.candidate()).unwrap();
        assert_eq!(first, second);
        assert!(first.ends_with('\n'));
        let root: Value = serde_json::from_str(&first).unwrap();
        assert_eq!(root["format"], SHORTCUT_PROFILE_FORMAT);
    }

    #[test]
    fn syntax_envelope_and_sequence_errors_have_stable_paths() {
        let syntax = stage_shortcut_profile_json("bad.json", "{").unwrap_err();
        assert_eq!(syntax.json_path(), "$");
        assert_eq!(syntax.code(), "shortcut-profile.invalid-json");

        let format = stage_shortcut_profile_json(
            "bad.json",
            r#"{"format":"rspice.shortcuts/2","profile":{}}"#,
        )
        .unwrap_err();
        assert_eq!(format.json_path(), "$.format");

        let strokes = (0..=MAX_SHORTCUT_SEQUENCE_STROKES)
            .map(|_| serde_json::json!({"key": "A"}))
            .collect::<Vec<_>>();
        let source = serde_json::json!({
            "format": SHORTCUT_PROFILE_FORMAT,
            "profile": {"commands": {"save": {"bindings": [{
                "slot": "primary", "platforms": ["desktop"], "sequence": strokes
            }]}}}
        });
        let error = stage_shortcut_profile_json("too-long.json", &source.to_string()).unwrap_err();
        assert_eq!(
            error.json_path(),
            "$.profile.commands[\"save\"].bindings[0].sequence"
        );
        assert!(error.context().contains("maximum is 4"));
    }

    #[derive(Default)]
    struct MemoryExportIo {
        picked_path: PathBuf,
        observed: RefCell<Vec<PathBuf>>,
        writes: RefCell<Vec<(PathBuf, String)>>,
    }

    impl ExportWorkflowIo for MemoryExportIo {
        fn show_save_dialog(
            &self,
            config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            assert_eq!(config.filter_extensions, &["json"]);
            Ok(Some(self.picked_path.clone()))
        }

        fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.writes
                .borrow_mut()
                .push((path.to_path_buf(), contents.to_owned()));
            Ok(())
        }

        fn write_text_file_observed(
            &self,
            destination: &ObservedExportDestination,
            contents: &str,
        ) -> Result<(), String> {
            self.observed
                .borrow_mut()
                .push(destination.path().to_path_buf());
            self.write_text_file(destination.path(), contents)
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            unreachable!()
        }
    }

    #[test]
    fn export_observes_destination_before_publication() {
        let io = MemoryExportIo {
            picked_path: PathBuf::from("profile.json"),
            ..MemoryExportIo::default()
        };
        let saved = export_shortcut_profile_with_io(&ShortcutPreferences::default(), &io).unwrap();
        assert_eq!(saved.as_deref(), Some(Path::new("profile.json")));
        assert_eq!(&*io.observed.borrow(), &[PathBuf::from("profile.json")]);
        assert_eq!(io.writes.borrow().len(), 1);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Default)]
    struct MemoryImportIo {
        picked_path: PathBuf,
        bytes: Vec<u8>,
    }

    #[cfg(not(target_arch = "wasm32"))]
    impl ShortcutProfileImportIo for MemoryImportIo {
        fn show_open_dialog(&self) -> Result<Option<PathBuf>, String> {
            Ok(Some(self.picked_path.clone()))
        }

        fn read_exact_bytes(&self, _path: &Path) -> Result<Vec<u8>, String> {
            Ok(self.bytes.clone())
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_shape_reads_exact_utf8_text_before_staging() {
        let io = MemoryImportIo {
            picked_path: PathBuf::from("portable.json"),
            bytes: VALID_SOURCE.as_bytes().to_vec(),
        };
        let staged = import_shortcut_profile_with_io(&io)
            .unwrap()
            .expect("selected import");
        assert_eq!(staged.source_name(), "portable.json");

        let invalid = MemoryImportIo {
            picked_path: PathBuf::from("binary.json"),
            bytes: vec![0xff, 0xfe],
        };
        let error = import_shortcut_profile_with_io(&invalid).unwrap_err();
        assert_eq!(error.code(), "shortcut-profile.invalid-utf8");
        assert_eq!(error.json_path(), "$");
    }

    #[test]
    fn import_rejects_oversized_test_double_payload() {
        let io = MemoryImportIo {
            picked_path: PathBuf::from("oversized.json"),
            bytes: vec![b' '; MAX_SHORTCUT_PROFILE_BYTES as usize + 1],
        };
        let error = import_shortcut_profile_with_io(&io).unwrap_err();
        assert_eq!(error.code(), "shortcut-profile.byte-limit");
        assert_eq!(error.json_path(), "$");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_reader_rejects_oversized_file_before_allocating_it() {
        let path = std::env::temp_dir().join(format!(
            "rspice-oversized-shortcuts-{}.json",
            uuid::Uuid::new_v4()
        ));
        let file = std::fs::File::create(&path).expect("create isolated oversized fixture");
        file.set_len(MAX_SHORTCUT_PROFILE_BYTES + 1)
            .expect("size isolated fixture");

        let error = NativeShortcutProfileImportIo
            .read_exact_bytes(&path)
            .expect_err("oversized native file must be rejected");
        assert!(error.contains("maximum"), "{error}");

        std::fs::remove_file(path).expect("remove isolated oversized fixture");
    }

    #[test]
    fn stale_browser_completion_cannot_release_replacement_lease() {
        use crate::common::browser_file_import::{
            BrowserTextImportKind, cancel_active_text_import, text_import_is_current,
            try_begin_text_import,
        };

        assert_eq!(
            BrowserTextImportKind::ShortcutProfile.max_bytes(),
            MAX_SHORTCUT_PROFILE_BYTES
        );

        let stale = try_begin_text_import(BrowserTextImportKind::ShortcutProfile).unwrap();
        assert_eq!(cancel_active_text_import(), Some(stale));
        let replacement = try_begin_text_import(BrowserTextImportKind::ShortcutProfile).unwrap();
        complete_browser_shortcut_profile_import(stale, Ok(None));
        assert!(poll_browser_shortcut_profile_import().is_none());
        assert!(text_import_is_current(replacement));

        complete_browser_shortcut_profile_import(replacement, Ok(None));
        assert!(matches!(
            poll_browser_shortcut_profile_import(),
            Some(Ok(None))
        ));
    }
}
