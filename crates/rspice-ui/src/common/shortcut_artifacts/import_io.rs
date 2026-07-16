//! Bounded shortcut-artifact source selection and structural detection.
//!
//! This boundary never mutates live preferences. A successful selection is
//! merely ready for the caller's review/merge transaction; picker cancellation
//! is represented explicitly and is never reported as an import.

use std::fmt;

#[cfg(not(target_arch = "wasm32"))]
use std::io::Read as _;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

use super::{DetectedShortcutArtifact, VscodeAdapterError, VscodeHostPlatform};

/// Exact upper bound shared with the schema and VS Code adapter.
pub const MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES: u64 =
    crate::common::shortcut_profile_workflow::MAX_SHORTCUT_PROFILE_BYTES;

const SHORTCUT_SOURCE_FILTER_NAME: &str = "Shortcut keybindings";
const SHORTCUT_SOURCE_FILTER_EXTENSIONS: &[&str] = &["json", "jsonc"];

/// Stable, machine-readable category for a source-selection failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutArtifactImportErrorKind {
    Picker,
    Read,
    ByteLimit,
    InvalidUtf8,
    InvalidSourceName,
    UnsupportedPlatform,
    ImportBusy,
    Detection,
}

impl ShortcutArtifactImportErrorKind {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Picker => "shortcut-artifact.import-picker",
            Self::Read => "shortcut-artifact.import-read",
            Self::ByteLimit => "shortcut-artifact.import-byte-limit",
            Self::InvalidUtf8 => "shortcut-artifact.import-invalid-utf8",
            Self::InvalidSourceName => "shortcut-artifact.import-source-name",
            Self::UnsupportedPlatform => "shortcut-artifact.import-platform",
            Self::ImportBusy => "shortcut-artifact.import-busy",
            Self::Detection => "shortcut-artifact.import-detection",
        }
    }
}

/// Typed failure retaining safe source context and the detector's stable cause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutArtifactImportError {
    kind: ShortcutArtifactImportErrorKind,
    source_name: Option<String>,
    cause_code: Option<&'static str>,
    message: String,
}

impl ShortcutArtifactImportError {
    fn new(kind: ShortcutArtifactImportErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            source_name: None,
            cause_code: None,
            message: message.into(),
        }
    }

    fn for_source(
        kind: ShortcutArtifactImportErrorKind,
        source_name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_name: Some(source_name.into()),
            cause_code: None,
            message: message.into(),
        }
    }

    fn detection(source_name: String, error: VscodeAdapterError) -> Self {
        Self {
            kind: ShortcutArtifactImportErrorKind::Detection,
            source_name: Some(source_name),
            cause_code: Some(error.code()),
            message: error.to_string(),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> ShortcutArtifactImportErrorKind {
        self.kind
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    #[must_use]
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    #[must_use]
    pub const fn cause_code(&self) -> Option<&'static str> {
        self.cause_code
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ShortcutArtifactImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source_name) = &self.source_name {
            write!(
                formatter,
                "{} ({source_name}) [{}]",
                self.message,
                self.code()
            )
        } else {
            write!(formatter, "{} [{}]", self.message, self.code())
        }
    }
}

impl std::error::Error for ShortcutArtifactImportError {}

/// A bounded, detected source whose exact bytes remain available for review.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadyShortcutArtifactSource {
    source_name: String,
    detected: DetectedShortcutArtifact,
}

impl ReadyShortcutArtifactSource {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn detected(&self) -> &DetectedShortcutArtifact {
        &self.detected
    }

    #[must_use]
    pub fn source_bytes(&self) -> &[u8] {
        match &self.detected {
            DetectedShortcutArtifact::RSpice(artifact) => artifact.source_bytes(),
            DetectedShortcutArtifact::Vscode(adaptation) => adaptation.source_bytes(),
        }
    }

    #[must_use]
    pub const fn source_digest(&self) -> [u8; 32] {
        match &self.detected {
            DetectedShortcutArtifact::RSpice(artifact) => artifact.source_digest(),
            DetectedShortcutArtifact::Vscode(adaptation) => adaptation.source_digest(),
        }
    }

    #[must_use]
    pub fn into_detected(self) -> DetectedShortcutArtifact {
        self.detected
    }
}

/// Explicit picker result. `Ready` still requires review and transactional apply.
#[derive(Debug, Clone, PartialEq)]
pub enum ShortcutArtifactImportOutcome {
    Cancelled,
    Ready(Box<ReadyShortcutArtifactSource>),
}

fn detect_source(
    source_name: String,
    source_bytes: Vec<u8>,
    platform: VscodeHostPlatform,
) -> Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError> {
    ensure_source_limit(source_bytes.len() as u64, &source_name)?;
    std::str::from_utf8(&source_bytes).map_err(|error| {
        ShortcutArtifactImportError::for_source(
            ShortcutArtifactImportErrorKind::InvalidUtf8,
            source_name.clone(),
            format!("selected shortcut source is not valid UTF-8: {error}"),
        )
    })?;
    let detected = super::detect_shortcut_artifact(&source_name, &source_bytes, platform)
        .map_err(|error| ShortcutArtifactImportError::detection(source_name.clone(), error))?;
    Ok(ShortcutArtifactImportOutcome::Ready(Box::new(
        ReadyShortcutArtifactSource {
            source_name,
            detected,
        },
    )))
}

fn ensure_source_limit(
    byte_count: u64,
    source_name: &str,
) -> Result<(), ShortcutArtifactImportError> {
    if byte_count > MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES {
        return Err(ShortcutArtifactImportError::for_source(
            ShortcutArtifactImportErrorKind::ByteLimit,
            source_name,
            format!(
                "selected shortcut source is {byte_count} bytes; maximum is {MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES} bytes"
            ),
        ));
    }
    Ok(())
}

fn normalized_source_filename(name: &str) -> Result<String, ShortcutArtifactImportError> {
    let filename = name
        .rsplit(['/', '\\'])
        .next()
        .map(str::trim)
        .filter(|filename| !filename.is_empty())
        .ok_or_else(|| {
            ShortcutArtifactImportError::new(
                ShortcutArtifactImportErrorKind::InvalidSourceName,
                "selected shortcut source has no stable filename",
            )
        })?;
    Ok(filename.to_owned())
}

fn platform_from_label(label: &str) -> Result<VscodeHostPlatform, ShortcutArtifactImportError> {
    let normalized = label.trim().to_ascii_lowercase();
    if normalized == "windows" || normalized.starts_with("win") {
        Ok(VscodeHostPlatform::Windows)
    } else if normalized == "macos"
        || normalized == "mac os"
        || normalized.starts_with("mac")
        || matches!(normalized.as_str(), "ios" | "iphone" | "ipad" | "ipod")
    {
        Ok(VscodeHostPlatform::Macos)
    } else if normalized == "linux"
        || normalized.starts_with("linux ")
        || normalized == "android"
        || normalized.starts_with("android ")
    {
        Ok(VscodeHostPlatform::Linux)
    } else {
        Err(ShortcutArtifactImportError::new(
            ShortcutArtifactImportErrorKind::UnsupportedPlatform,
            format!("shortcut platform mapping is unsupported for host platform '{label}'"),
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn native_host_platform() -> Result<VscodeHostPlatform, ShortcutArtifactImportError> {
    platform_from_label(std::env::consts::OS)
}

/// Injectable native picker/reader boundary used by production and tests.
#[cfg(not(target_arch = "wasm32"))]
pub trait ShortcutArtifactImportIo {
    fn host_platform(&self) -> Result<VscodeHostPlatform, ShortcutArtifactImportError>;
    fn show_open_dialog(&self) -> Result<Option<PathBuf>, String>;
    fn read_exact_bytes(&self, path: &Path) -> Result<Vec<u8>, String>;
}

/// Production desktop source picker and exact bounded reader.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, Default)]
pub struct NativeShortcutArtifactImportIo;

#[cfg(not(target_arch = "wasm32"))]
impl ShortcutArtifactImportIo for NativeShortcutArtifactImportIo {
    fn host_platform(&self) -> Result<VscodeHostPlatform, ShortcutArtifactImportError> {
        native_host_platform()
    }

    fn show_open_dialog(&self) -> Result<Option<PathBuf>, String> {
        Ok(rfd::FileDialog::new()
            .add_filter(
                SHORTCUT_SOURCE_FILTER_NAME,
                SHORTCUT_SOURCE_FILTER_EXTENSIONS,
            )
            .set_title("Import Shortcut Keybindings")
            .pick_file())
    }

    fn read_exact_bytes(&self, path: &Path) -> Result<Vec<u8>, String> {
        let metadata = std::fs::metadata(path).map_err(|error| error.to_string())?;
        if metadata.len() > MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES {
            return Err(format!(
                "selected file is {} bytes; maximum is {MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES} bytes",
                metadata.len()
            ));
        }

        let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
        let capacity = usize::try_from(metadata.len()).unwrap_or(0);
        let mut bytes = Vec::with_capacity(capacity);
        file.take(MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| error.to_string())?;
        if bytes.len() as u64 > MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES {
            return Err(format!(
                "selected file changed while being read and exceeds the {MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES}-byte maximum"
            ));
        }
        Ok(bytes)
    }
}

/// Select and detect a native source without changing any shortcut state.
#[cfg(not(target_arch = "wasm32"))]
pub fn import_shortcut_artifact_source_with_io(
    io: &(impl ShortcutArtifactImportIo + ?Sized),
) -> Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError> {
    let platform = io.host_platform()?;
    let Some(path) = io.show_open_dialog().map_err(|error| {
        ShortcutArtifactImportError::new(
            ShortcutArtifactImportErrorKind::Picker,
            format!("could not open shortcut source picker: {error}"),
        )
    })?
    else {
        return Ok(ShortcutArtifactImportOutcome::Cancelled);
    };
    let source_name = path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or_else(|| {
            ShortcutArtifactImportError::new(
                ShortcutArtifactImportErrorKind::InvalidSourceName,
                "selected shortcut source filename is not valid Unicode",
            )
        })
        .and_then(normalized_source_filename)?;
    let bytes = io.read_exact_bytes(&path).map_err(|error| {
        let kind = if error.contains("maximum") || error.contains("exceeds") {
            ShortcutArtifactImportErrorKind::ByteLimit
        } else {
            ShortcutArtifactImportErrorKind::Read
        };
        ShortcutArtifactImportError::for_source(
            kind,
            source_name.clone(),
            format!("could not read selected shortcut source: {error}"),
        )
    })?;
    detect_source(source_name, bytes, platform)
}

/// Open the production native source picker and structurally detect its bytes.
#[cfg(not(target_arch = "wasm32"))]
pub fn import_shortcut_artifact_source()
-> Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError> {
    import_shortcut_artifact_source_with_io(&NativeShortcutArtifactImportIo)
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug)]
struct BrowserShortcutArtifactImportCompletion {
    token: crate::common::browser_file_import::TextImportToken,
    result: Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError>,
}

#[cfg(any(test, target_arch = "wasm32"))]
thread_local! {
    static BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT: std::cell::RefCell<Option<BrowserShortcutArtifactImportCompletion>> =
        const { std::cell::RefCell::new(None) };
    static BROWSER_SHORTCUT_ARTIFACT_IMPORT_TOKEN: std::cell::Cell<Option<crate::common::browser_file_import::TextImportToken>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(any(test, target_arch = "wasm32"))]
fn begin_browser_shortcut_artifact_import()
-> Result<crate::common::browser_file_import::TextImportToken, ShortcutArtifactImportError> {
    let token = crate::common::browser_file_import::try_begin_text_import(
        crate::common::browser_file_import::BrowserTextImportKind::ShortcutProfile,
    )
    .map_err(|error| {
        ShortcutArtifactImportError::new(ShortcutArtifactImportErrorKind::ImportBusy, error)
    })?;
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
        *slot.borrow_mut() = None;
    });
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_TOKEN.with(|slot| slot.set(Some(token)));
    Ok(token)
}

#[cfg(target_arch = "wasm32")]
fn browser_host_platform() -> Result<VscodeHostPlatform, ShortcutArtifactImportError> {
    use wasm_bindgen::JsValue;

    let window = web_sys::window().ok_or_else(|| {
        ShortcutArtifactImportError::new(
            ShortcutArtifactImportErrorKind::UnsupportedPlatform,
            "browser window is unavailable; host platform cannot be verified",
        )
    })?;
    let navigator = js_sys::Reflect::get(window.as_ref(), &JsValue::from_str("navigator"))
        .map_err(|_| {
            ShortcutArtifactImportError::new(
                ShortcutArtifactImportErrorKind::UnsupportedPlatform,
                "browser navigator is unavailable; host platform cannot be verified",
            )
        })?;
    let user_agent_data =
        js_sys::Reflect::get(&navigator, &JsValue::from_str("userAgentData")).ok();
    let high_entropy_platform = user_agent_data
        .as_ref()
        .and_then(|data| js_sys::Reflect::get(data, &JsValue::from_str("platform")).ok())
        .and_then(|platform| platform.as_string())
        .filter(|platform| !platform.trim().is_empty());
    let legacy_platform = js_sys::Reflect::get(&navigator, &JsValue::from_str("platform"))
        .ok()
        .and_then(|platform| platform.as_string())
        .filter(|platform| !platform.trim().is_empty());
    let platform = high_entropy_platform.or(legacy_platform).ok_or_else(|| {
        ShortcutArtifactImportError::new(
            ShortcutArtifactImportErrorKind::UnsupportedPlatform,
            "browser did not disclose a supported host platform",
        )
    })?;
    platform_from_label(&platform)
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_picker_result(
    platform: VscodeHostPlatform,
    result: Result<Option<crate::common::browser_file_import::PickedTextFile>, String>,
) -> Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError> {
    match result {
        Ok(Some(file)) => {
            let source_name = normalized_source_filename(&file.name)?;
            detect_source(source_name, file.contents.into_bytes(), platform)
        }
        Ok(None) => Ok(ShortcutArtifactImportOutcome::Cancelled),
        Err(error) => {
            let kind = if error.contains("not valid UTF-8") {
                ShortcutArtifactImportErrorKind::InvalidUtf8
            } else if error.contains("size limit") || error.contains("exceeds") {
                ShortcutArtifactImportErrorKind::ByteLimit
            } else {
                ShortcutArtifactImportErrorKind::Read
            };
            Err(ShortcutArtifactImportError::new(kind, error))
        }
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn complete_browser_shortcut_artifact_import(
    token: crate::common::browser_file_import::TextImportToken,
    result: Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError>,
) -> bool {
    if !crate::common::browser_file_import::text_import_is_current(token) {
        return false;
    }
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
        *slot.borrow_mut() = Some(BrowserShortcutArtifactImportCompletion { token, result });
    });
    true
}

/// Start one browser file-picker operation under the shared import lease.
#[cfg(target_arch = "wasm32")]
pub fn start_browser_shortcut_artifact_import() -> Result<(), ShortcutArtifactImportError> {
    let platform = browser_host_platform()?;
    let token = begin_browser_shortcut_artifact_import()?;
    crate::common::browser_file_import::pick_text_file(
        SHORTCUT_SOURCE_FILTER_NAME,
        SHORTCUT_SOURCE_FILTER_EXTENSIONS,
        move |result| {
            if !crate::common::browser_file_import::text_import_is_current(token) {
                return;
            }
            let result = browser_picker_result(platform, result);
            let _ = complete_browser_shortcut_artifact_import(token, result);
        },
    );
    Ok(())
}

/// Return one current browser completion. Late or replaced completions vanish.
#[cfg(any(test, target_arch = "wasm32"))]
pub fn poll_browser_shortcut_artifact_import()
-> Option<Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError>> {
    let completion =
        BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())?;
    if !crate::common::browser_file_import::finish_text_import(completion.token) {
        clear_browser_shortcut_artifact_import_owner(completion.token);
        return None;
    }
    clear_browser_shortcut_artifact_import_owner(completion.token);
    Some(completion.result)
}

#[cfg(any(test, target_arch = "wasm32"))]
fn clear_browser_shortcut_artifact_import_owner(
    token: crate::common::browser_file_import::TextImportToken,
) {
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_TOKEN.with(|slot| {
        if slot.get() == Some(token) {
            slot.set(None);
        }
    });
}

/// Cancel only this workflow's current shared-gate lease.
///
/// A lease already replaced by another import is never cancelled. Any late
/// callback from the cancelled picker fails its current-token check and is
/// unable to publish a completion or release the replacement gate.
#[cfg(any(test, target_arch = "wasm32"))]
pub fn cancel_browser_shortcut_artifact_import() -> bool {
    let Some(token) = BROWSER_SHORTCUT_ARTIFACT_IMPORT_TOKEN.with(std::cell::Cell::get) else {
        return false;
    };
    if !crate::common::browser_file_import::text_import_is_current(token) {
        clear_browser_shortcut_artifact_import_owner(token);
        BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
            *slot.borrow_mut() = None;
        });
        return false;
    }
    if crate::common::browser_file_import::cancel_active_text_import() != Some(token) {
        return false;
    }
    clear_browser_shortcut_artifact_import_owner(token);
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
        *slot.borrow_mut() = None;
    });
    true
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    const NATIVE_JSONC: &[u8] = br#"{
        // original source comment must survive as provenance
        "format": "rspice.shortcuts/1",
        "profile": { "commands": {}, },
    }"#;

    #[derive(Clone)]
    struct FakeIo {
        platform: Result<VscodeHostPlatform, ShortcutArtifactImportError>,
        selected: Result<Option<PathBuf>, String>,
        bytes: Result<Vec<u8>, String>,
        reads: Cell<usize>,
        picks: Cell<usize>,
    }

    impl FakeIo {
        fn ready(bytes: Vec<u8>) -> Self {
            Self {
                platform: Ok(VscodeHostPlatform::Windows),
                selected: Ok(Some(PathBuf::from("private/keybindings.jsonc"))),
                bytes: Ok(bytes),
                reads: Cell::new(0),
                picks: Cell::new(0),
            }
        }
    }

    impl ShortcutArtifactImportIo for FakeIo {
        fn host_platform(&self) -> Result<VscodeHostPlatform, ShortcutArtifactImportError> {
            self.platform.clone()
        }

        fn show_open_dialog(&self) -> Result<Option<PathBuf>, String> {
            self.picks.set(self.picks.get() + 1);
            self.selected.clone()
        }

        fn read_exact_bytes(&self, _path: &Path) -> Result<Vec<u8>, String> {
            self.reads.set(self.reads.get() + 1);
            self.bytes.clone()
        }
    }

    fn ready(outcome: ShortcutArtifactImportOutcome) -> ReadyShortcutArtifactSource {
        let ShortcutArtifactImportOutcome::Ready(source) = outcome else {
            panic!("source must be ready")
        };
        *source
    }

    fn reset_browser_test_state() {
        if !cancel_browser_shortcut_artifact_import() {
            let _ = crate::common::browser_file_import::cancel_active_text_import();
        }
        BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
            *slot.borrow_mut() = None;
        });
        BROWSER_SHORTCUT_ARTIFACT_IMPORT_TOKEN.with(|slot| slot.set(None));
    }

    #[test]
    fn native_cancel_is_explicit_and_does_not_read_or_claim_success() {
        let io = FakeIo {
            selected: Ok(None),
            ..FakeIo::ready(Vec::new())
        };

        let outcome = import_shortcut_artifact_source_with_io(&io).expect("cancel is valid");

        assert_eq!(outcome, ShortcutArtifactImportOutcome::Cancelled);
        assert_eq!(io.picks.get(), 1);
        assert_eq!(io.reads.get(), 0);
    }

    #[test]
    fn native_jsonc_retains_exact_bytes_digest_and_stable_filename() {
        let io = FakeIo::ready(NATIVE_JSONC.to_vec());

        let source = ready(import_shortcut_artifact_source_with_io(&io).expect("source detects"));

        assert_eq!(source.source_name(), "keybindings.jsonc");
        assert_eq!(source.source_bytes(), NATIVE_JSONC);
        assert_eq!(source.source_digest(), super::super::sha256(NATIVE_JSONC));
        let DetectedShortcutArtifact::RSpice(artifact) = source.detected() else {
            panic!("native object must remain a native RSpice artifact")
        };
        assert_eq!(artifact.artifact().source_name(), "keybindings.jsonc");
        assert_eq!(artifact.artifact().source_digest(), source.source_digest());
    }

    #[test]
    fn native_boundary_classifies_picker_read_limit_utf8_and_detection_errors() {
        let picker = FakeIo {
            selected: Err("dialog unavailable".to_owned()),
            ..FakeIo::ready(Vec::new())
        };
        assert_eq!(
            import_shortcut_artifact_source_with_io(&picker)
                .unwrap_err()
                .kind(),
            ShortcutArtifactImportErrorKind::Picker
        );

        let read = FakeIo {
            bytes: Err("media failure".to_owned()),
            ..FakeIo::ready(Vec::new())
        };
        assert_eq!(
            import_shortcut_artifact_source_with_io(&read)
                .unwrap_err()
                .kind(),
            ShortcutArtifactImportErrorKind::Read
        );

        let oversized = FakeIo::ready(vec![
            b' ';
            usize::try_from(MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES + 1)
                .expect("test limit fits usize")
        ]);
        assert_eq!(
            import_shortcut_artifact_source_with_io(&oversized)
                .unwrap_err()
                .kind(),
            ShortcutArtifactImportErrorKind::ByteLimit
        );

        let utf8 = FakeIo::ready(vec![0xff]);
        assert_eq!(
            import_shortcut_artifact_source_with_io(&utf8)
                .unwrap_err()
                .kind(),
            ShortcutArtifactImportErrorKind::InvalidUtf8
        );

        let detection = FakeIo::ready(b"not json".to_vec());
        let error = import_shortcut_artifact_source_with_io(&detection).unwrap_err();
        assert_eq!(error.kind(), ShortcutArtifactImportErrorKind::Detection);
        assert_eq!(error.cause_code(), Some("shortcut-adapter.invalid-json"));
        assert_eq!(error.source_name(), Some("keybindings.jsonc"));
    }

    #[test]
    fn unsupported_platform_fails_before_picker_or_read() {
        let io = FakeIo {
            platform: Err(ShortcutArtifactImportError::new(
                ShortcutArtifactImportErrorKind::UnsupportedPlatform,
                "unsupported test host",
            )),
            ..FakeIo::ready(NATIVE_JSONC.to_vec())
        };

        let error = import_shortcut_artifact_source_with_io(&io).unwrap_err();

        assert_eq!(
            error.kind(),
            ShortcutArtifactImportErrorKind::UnsupportedPlatform
        );
        assert_eq!(io.picks.get(), 0);
        assert_eq!(io.reads.get(), 0);
    }

    #[test]
    fn platform_labels_map_only_windows_macos_and_linux() {
        for (label, expected) in [
            ("Win32", VscodeHostPlatform::Windows),
            ("Windows", VscodeHostPlatform::Windows),
            ("MacIntel", VscodeHostPlatform::Macos),
            ("macOS", VscodeHostPlatform::Macos),
            ("iPhone", VscodeHostPlatform::Macos),
            ("iPad", VscodeHostPlatform::Macos),
            ("iPod", VscodeHostPlatform::Macos),
            ("Linux x86_64", VscodeHostPlatform::Linux),
            ("Android", VscodeHostPlatform::Linux),
            ("Android armv8l", VscodeHostPlatform::Linux),
        ] {
            assert_eq!(platform_from_label(label).unwrap(), expected);
        }
        for unsupported in ["", "FreeBSD", "Chrome OS"] {
            assert_eq!(
                platform_from_label(unsupported).unwrap_err().kind(),
                ShortcutArtifactImportErrorKind::UnsupportedPlatform
            );
        }
    }

    #[test]
    fn detection_is_deterministic_and_does_not_mutate_live_shortcuts() {
        let live = crate::workbench::shortcuts::ShortcutProfileLibrary::default();
        let before = serde_json::to_value(&live).expect("live library serializes");
        let first = import_shortcut_artifact_source_with_io(&FakeIo::ready(NATIVE_JSONC.to_vec()))
            .expect("first detects");
        let second = import_shortcut_artifact_source_with_io(&FakeIo::ready(NATIVE_JSONC.to_vec()))
            .expect("second detects");

        assert_eq!(first, second);
        assert_eq!(
            serde_json::to_value(&live).expect("live library still serializes"),
            before
        );
    }

    #[test]
    fn browser_picker_cancel_polls_as_explicit_cancelled_outcome() {
        reset_browser_test_state();
        let token = begin_browser_shortcut_artifact_import().expect("lease starts");
        let result = browser_picker_result(VscodeHostPlatform::Windows, Ok(None));
        assert!(complete_browser_shortcut_artifact_import(token, result));

        assert_eq!(
            poll_browser_shortcut_artifact_import(),
            Some(Ok(ShortcutArtifactImportOutcome::Cancelled))
        );
        reset_browser_test_state();
    }

    #[test]
    fn stale_browser_completion_cannot_replace_or_release_current_lease() {
        reset_browser_test_state();
        let stale = begin_browser_shortcut_artifact_import().expect("first lease starts");
        assert_eq!(
            crate::common::browser_file_import::cancel_active_text_import(),
            Some(stale)
        );
        let current = begin_browser_shortcut_artifact_import().expect("replacement starts");

        assert!(!complete_browser_shortcut_artifact_import(
            stale,
            Ok(ShortcutArtifactImportOutcome::Cancelled)
        ));
        assert!(poll_browser_shortcut_artifact_import().is_none());
        assert!(crate::common::browser_file_import::text_import_is_current(
            current
        ));

        assert!(complete_browser_shortcut_artifact_import(
            current,
            Ok(ShortcutArtifactImportOutcome::Cancelled)
        ));
        assert_eq!(
            poll_browser_shortcut_artifact_import(),
            Some(Ok(ShortcutArtifactImportOutcome::Cancelled))
        );
        reset_browser_test_state();
    }

    #[test]
    fn owned_browser_cancel_clears_completion_and_makes_late_callback_inert() {
        reset_browser_test_state();
        let cancelled = begin_browser_shortcut_artifact_import().expect("lease starts");
        assert!(complete_browser_shortcut_artifact_import(
            cancelled,
            Ok(ShortcutArtifactImportOutcome::Cancelled)
        ));

        assert!(cancel_browser_shortcut_artifact_import());
        assert!(poll_browser_shortcut_artifact_import().is_none());
        assert!(!complete_browser_shortcut_artifact_import(
            cancelled,
            Ok(ShortcutArtifactImportOutcome::Cancelled)
        ));

        let replacement = begin_browser_shortcut_artifact_import().expect("replacement starts");
        assert!(crate::common::browser_file_import::text_import_is_current(
            replacement
        ));
        assert!(!complete_browser_shortcut_artifact_import(
            cancelled,
            Ok(ShortcutArtifactImportOutcome::Cancelled)
        ));
        assert!(crate::common::browser_file_import::text_import_is_current(
            replacement
        ));
        assert!(cancel_browser_shortcut_artifact_import());
        reset_browser_test_state();
    }

    #[test]
    fn browser_cancel_never_cancels_another_workflows_lease() {
        reset_browser_test_state();
        let other = crate::common::browser_file_import::try_begin_text_import(
            crate::common::browser_file_import::BrowserTextImportKind::Project,
        )
        .expect("other workflow starts");

        assert!(!cancel_browser_shortcut_artifact_import());
        assert!(crate::common::browser_file_import::text_import_is_current(
            other
        ));
        assert!(crate::common::browser_file_import::finish_text_import(
            other
        ));
        reset_browser_test_state();
    }

    #[test]
    fn browser_read_contract_preserves_typed_limit_utf8_and_detection_failures() {
        let limit = browser_picker_result(
            VscodeHostPlatform::Windows,
            Err(format!(
                "Selected huge.json exceeds the supported {MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES}-byte size limit"
            )),
        )
        .unwrap_err();
        assert_eq!(limit.kind(), ShortcutArtifactImportErrorKind::ByteLimit);

        let utf8 = browser_picker_result(
            VscodeHostPlatform::Windows,
            Err("Selected file is not valid UTF-8: invalid byte".to_owned()),
        )
        .unwrap_err();
        assert_eq!(utf8.kind(), ShortcutArtifactImportErrorKind::InvalidUtf8);

        let detection = browser_picker_result(
            VscodeHostPlatform::Linux,
            Ok(Some(crate::common::browser_file_import::PickedTextFile {
                name: "keybindings.jsonc".to_owned(),
                contents: "[invalid]".to_owned(),
            })),
        )
        .unwrap_err();
        assert_eq!(detection.kind(), ShortcutArtifactImportErrorKind::Detection);
        assert_eq!(
            detection.cause_code(),
            Some("shortcut-adapter.invalid-json")
        );
    }
}
