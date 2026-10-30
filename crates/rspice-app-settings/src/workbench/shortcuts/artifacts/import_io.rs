//! Bounded shortcut-artifact source selection and structural detection.
//!
//! This boundary never mutates live preferences. A successful selection is
//! merely ready for the caller's review/merge transaction; picker cancellation
//! is represented explicitly and is never reported as an import.

use crate::workbench::framework::command_registry::Registry;
use std::fmt;

#[cfg(not(target_arch = "wasm32"))]
use std::io::Read as _;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

use super::{DetectedShortcutArtifact, VscodeAdapterError, VscodeHostPlatform};

/// Exact upper bound shared with the schema and VS Code adapter.
pub const MAX_SHORTCUT_ARTIFACT_SOURCE_BYTES: u64 =
    crate::io::file_exchange::MAX_SHORTCUT_PROFILE_BYTES;

const SHORTCUT_SOURCE_FILTER_NAME: &str = "Shortcut keybindings";
const SHORTCUT_SOURCE_FILTER_EXTENSIONS: &[&str] = &["json", "jsonc"];

/// Stable, machine-readable category for a source-selection failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutArtifactImportErrorKind {
    #[cfg(not(target_arch = "wasm32"))]
    Picker,
    Read,
    ByteLimit,
    InvalidUtf8,
    InvalidSourceName,
    UnsupportedPlatform,
    #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
    ImportBusy,
    Detection,
}

impl ShortcutArtifactImportErrorKind {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::Picker => "shortcut-artifact.import-picker",
            Self::Read => "shortcut-artifact.import-read",
            Self::ByteLimit => "shortcut-artifact.import-byte-limit",
            Self::InvalidUtf8 => "shortcut-artifact.import-invalid-utf8",
            Self::InvalidSourceName => "shortcut-artifact.import-source-name",
            Self::UnsupportedPlatform => "shortcut-artifact.import-platform",
            #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
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
    pub fn new(kind: ShortcutArtifactImportErrorKind, message: impl Into<String>) -> Self {
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
    #[cfg(any(test, feature = "test-support"))]
    pub const fn kind(&self) -> ShortcutArtifactImportErrorKind {
        self.kind
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    #[must_use]
    #[cfg(any(test, feature = "test-support"))]
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    #[must_use]
    #[cfg(any(test, feature = "test-support"))]
    pub const fn cause_code(&self) -> Option<&'static str> {
        self.cause_code
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
    #[cfg(any(test, feature = "test-support"))]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    #[cfg(any(test, feature = "test-support"))]
    pub const fn detected(&self) -> &DetectedShortcutArtifact {
        &self.detected
    }

    #[must_use]
    #[cfg(any(test, feature = "test-support"))]
    pub fn source_bytes(&self) -> &[u8] {
        match &self.detected {
            DetectedShortcutArtifact::RSpice(artifact) => artifact.source_bytes(),
            DetectedShortcutArtifact::Vscode(adaptation) => adaptation.source_bytes(),
        }
    }

    #[must_use]
    #[cfg(any(test, feature = "test-support"))]
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
    registry: &Registry,
) -> Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError> {
    ensure_source_limit(source_bytes.len() as u64, &source_name)?;
    std::str::from_utf8(&source_bytes).map_err(|error| {
        ShortcutArtifactImportError::for_source(
            ShortcutArtifactImportErrorKind::InvalidUtf8,
            source_name.clone(),
            format!("selected shortcut source is not valid UTF-8: {error}"),
        )
    })?;
    let detected = super::detect_shortcut_artifact(&source_name, &source_bytes, platform, registry)
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

pub fn platform_from_label(label: &str) -> Result<VscodeHostPlatform, ShortcutArtifactImportError> {
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
    registry: &Registry,
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
    detect_source(source_name, bytes, platform, registry)
}

/// Open the production native source picker and structurally detect its bytes.
#[cfg(not(target_arch = "wasm32"))]
pub fn import_shortcut_artifact_source(
    registry: &Registry,
) -> Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError> {
    import_shortcut_artifact_source_with_io(&NativeShortcutArtifactImportIo, registry)
}

#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
#[derive(Debug)]
pub struct BrowserShortcutArtifactImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError>,
}

#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
thread_local! {
    pub static BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT: std::cell::RefCell<Option<BrowserShortcutArtifactImportCompletion>> =
        const { std::cell::RefCell::new(None) };
    pub static BROWSER_SHORTCUT_ARTIFACT_IMPORT_TOKEN: std::cell::Cell<Option<crate::workbench::browser::file_import::TextImportToken>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
pub fn begin_browser_shortcut_artifact_import()
-> Result<crate::workbench::browser::file_import::TextImportToken, ShortcutArtifactImportError> {
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::ShortcutProfile,
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

#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
pub fn browser_picker_result(
    platform: VscodeHostPlatform,
    result: Result<Option<crate::workbench::browser::file_import::PickedTextFile>, String>,
    registry: &Registry,
) -> Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError> {
    match result {
        Ok(Some(file)) => {
            let source_name = normalized_source_filename(&file.name)?;
            detect_source(source_name, file.contents.into_bytes(), platform, registry)
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

#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
pub fn complete_browser_shortcut_artifact_import(
    token: crate::workbench::browser::file_import::TextImportToken,
    result: Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError>,
) -> bool {
    if !crate::workbench::browser::file_import::text_import_is_current(token) {
        return false;
    }
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
        *slot.borrow_mut() = Some(BrowserShortcutArtifactImportCompletion { token, result });
    });
    true
}

/// Start one browser file-picker operation under the shared import lease.
#[cfg(target_arch = "wasm32")]
pub fn start_browser_shortcut_artifact_import(
    registry: &'static Registry,
) -> Result<(), ShortcutArtifactImportError> {
    let platform = browser_host_platform()?;
    let token = begin_browser_shortcut_artifact_import()?;
    crate::workbench::browser::file_import::pick_text_file(
        SHORTCUT_SOURCE_FILTER_NAME,
        SHORTCUT_SOURCE_FILTER_EXTENSIONS,
        move |result| {
            if !crate::workbench::browser::file_import::text_import_is_current(token) {
                return;
            }
            let result = browser_picker_result(platform, result, registry);
            let _ = complete_browser_shortcut_artifact_import(token, result);
        },
    );
    Ok(())
}

/// Return one current browser completion. Late or replaced completions vanish.
#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
pub fn poll_browser_shortcut_artifact_import()
-> Option<Result<ShortcutArtifactImportOutcome, ShortcutArtifactImportError>> {
    let completion =
        BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())?;
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        clear_browser_shortcut_artifact_import_owner(completion.token);
        return None;
    }
    clear_browser_shortcut_artifact_import_owner(completion.token);
    Some(completion.result)
}

#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
fn clear_browser_shortcut_artifact_import_owner(
    token: crate::workbench::browser::file_import::TextImportToken,
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
#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserShortcutArtifactImportCancelOutcome {
    /// This workflow's exact picker lease was cancelled.
    Cancelled,
    /// This workflow no longer owns a picker; no completion can arrive.
    AlreadyReleased,
    /// The exact owner still exists and must be polled or cancelled later.
    StillOwned,
}

#[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
pub fn cancel_browser_shortcut_artifact_import() -> BrowserShortcutArtifactImportCancelOutcome {
    let Some(token) = BROWSER_SHORTCUT_ARTIFACT_IMPORT_TOKEN.with(std::cell::Cell::get) else {
        return BrowserShortcutArtifactImportCancelOutcome::AlreadyReleased;
    };
    if !crate::workbench::browser::file_import::text_import_is_current(token) {
        clear_browser_shortcut_artifact_import_owner(token);
        BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
            *slot.borrow_mut() = None;
        });
        return BrowserShortcutArtifactImportCancelOutcome::AlreadyReleased;
    }
    if crate::workbench::browser::file_import::cancel_active_text_import() != Some(token) {
        return BrowserShortcutArtifactImportCancelOutcome::StillOwned;
    }
    clear_browser_shortcut_artifact_import_owner(token);
    BROWSER_SHORTCUT_ARTIFACT_IMPORT_RESULT.with(|slot| {
        *slot.borrow_mut() = None;
    });
    BrowserShortcutArtifactImportCancelOutcome::Cancelled
}
