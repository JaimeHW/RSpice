//! Production file I/O for versioned shortcut reference artifacts.
//!
//! Serialization is completed before a destination is chosen. Native writes
//! then publish the exact bytes with a picker-time compare-and-exchange
//! precondition. Browser exports are downloads and therefore report only that
//! the download was started, never that a durable file was saved.

use std::fmt;
use std::path::{Path, PathBuf};

use super::markdown::serialize_shortcut_reference_markdown;
use super::pdf::serialize_shortcut_reference_pdf;
use super::projection::{ShortcutReferenceModel, serialize_shortcut_reference_json};

pub const MAX_SHORTCUT_JSON_ARTIFACT_BYTES: u64 =
    crate::io::file_exchange::MAX_SHORTCUT_PROFILE_BYTES;
pub const MAX_SHORTCUT_MARKDOWN_ARTIFACT_BYTES: u64 = 4 * 1024 * 1024;
pub const MAX_SHORTCUT_PDF_ARTIFACT_BYTES: u64 = 16 * 1024 * 1024;

pub const MAX_FILENAME_STEM_BYTES: usize = 96;

/// Supported shortcut reference artifact encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutArtifactFormat {
    Json,
    Markdown,
    Pdf,
}

impl ShortcutArtifactFormat {
    pub const ALL: [Self; 3] = [Self::Json, Self::Markdown, Self::Pdf];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Json => "JSON",
            Self::Markdown => "Markdown",
            Self::Pdf => "PDF",
        }
    }

    #[must_use]
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Markdown => "md",
            Self::Pdf => "pdf",
        }
    }

    #[must_use]
    #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
    pub const fn mime_type(self) -> &'static str {
        match self {
            Self::Json => "application/json",
            Self::Markdown => "text/markdown;charset=utf-8",
            Self::Pdf => "application/pdf",
        }
    }

    #[must_use]
    pub const fn default_filename(self) -> &'static str {
        match self {
            Self::Json => "rspice-shortcuts.json",
            Self::Markdown => "rspice-shortcuts.md",
            Self::Pdf => "rspice-shortcuts.pdf",
        }
    }

    #[must_use]
    #[cfg(not(target_arch = "wasm32"))]
    pub const fn filter_name(self) -> &'static str {
        match self {
            Self::Json => "RSpice Shortcut Artifact",
            Self::Markdown => "Markdown Document",
            Self::Pdf => "PDF Document",
        }
    }

    #[must_use]
    pub const fn max_bytes(self) -> u64 {
        match self {
            Self::Json => MAX_SHORTCUT_JSON_ARTIFACT_BYTES,
            Self::Markdown => MAX_SHORTCUT_MARKDOWN_ARTIFACT_BYTES,
            Self::Pdf => MAX_SHORTCUT_PDF_ARTIFACT_BYTES,
        }
    }

    #[must_use]
    pub const fn is_text(self) -> bool {
        matches!(self, Self::Json | Self::Markdown)
    }
}

/// Fully serialized shortcut artifact ready for publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedShortcutArtifact {
    pub format: ShortcutArtifactFormat,
    pub filename: String,
    pub bytes: Vec<u8>,
}

impl PreparedShortcutArtifact {
    #[must_use]
    pub const fn format(&self) -> ShortcutArtifactFormat {
        self.format
    }

    #[must_use]
    pub fn filename(&self) -> &str {
        &self.filename
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Distinguishes native durable publication, browser handoff, and picker
/// cancellation. A browser handoff intentionally makes no completion claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutArtifactExportOutcome {
    #[cfg(any(test, not(target_arch = "wasm32")))]
    Published {
        path: PathBuf,
    },
    /// Browser handoff: the download began and claims no completion.
    #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
    DownloadStarted {
        suggested_filename: String,
    },
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutArtifactIoError {
    code: &'static str,
    message: String,
}

impl ShortcutArtifactIoError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[must_use]
    #[cfg(any(test, feature = "test-support"))]
    pub const fn code(&self) -> &'static str {
        self.code
    }
}

impl fmt::Display for ShortcutArtifactIoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

impl std::error::Error for ShortcutArtifactIoError {}

/// Serialize one immutable reference model and validate its artifact limit.
pub fn prepare_shortcut_artifact(
    model: &ShortcutReferenceModel,
    format: ShortcutArtifactFormat,
    suggested_filename: Option<&str>,
) -> Result<PreparedShortcutArtifact, ShortcutArtifactIoError> {
    let bytes = match format {
        ShortcutArtifactFormat::Json => serialize_shortcut_reference_json(model)
            .map(String::into_bytes)
            .map_err(|error| {
                ShortcutArtifactIoError::new(
                    "shortcut-artifact.encode-json",
                    format!("could not encode shortcut JSON: {error}"),
                )
            })?,
        ShortcutArtifactFormat::Markdown => {
            serialize_shortcut_reference_markdown(model).into_bytes()
        }
        ShortcutArtifactFormat::Pdf => {
            serialize_shortcut_reference_pdf(model).map_err(|error| {
                ShortcutArtifactIoError::new(
                    "shortcut-artifact.encode-pdf",
                    format!("could not encode shortcut PDF: {error}"),
                )
            })?
        }
    };
    validate_artifact_bytes(format, &bytes)?;
    let filename = normalize_shortcut_artifact_filename(
        suggested_filename.unwrap_or_else(|| format.default_filename()),
        format,
    );
    Ok(PreparedShortcutArtifact {
        format,
        filename,
        bytes,
    })
}

/// Normalize an untrusted suggested filename without retaining a supplied
/// directory or allowing a misleading artifact extension.
#[must_use]
pub fn normalize_shortcut_artifact_filename(
    suggested: &str,
    format: ShortcutArtifactFormat,
) -> String {
    let filename = suggested
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or_default()
        .trim();
    let mut stem = strip_artifact_extension(filename).to_owned();
    let mut normalized = String::with_capacity(stem.len().min(MAX_FILENAME_STEM_BYTES));
    let mut pending_space = false;
    let mut previous_replacement = false;
    for character in stem.drain(..) {
        if character.is_whitespace() {
            pending_space = !normalized.is_empty();
            previous_replacement = false;
            continue;
        }
        if pending_space {
            normalized.push(' ');
            pending_space = false;
        }
        let invalid = character.is_control()
            || matches!(
                character,
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
            );
        if invalid {
            if !previous_replacement && !normalized.is_empty() {
                normalized.push('-');
            }
            previous_replacement = true;
        } else {
            normalized.push(character);
            previous_replacement = false;
        }
        truncate_utf8(&mut normalized, MAX_FILENAME_STEM_BYTES);
        if normalized.len() >= MAX_FILENAME_STEM_BYTES {
            break;
        }
    }
    let trimmed =
        normalized.trim_matches(|character: char| matches!(character, ' ' | '.' | '-' | '_'));
    let mut stem = if trimmed.is_empty() {
        "rspice-shortcuts".to_owned()
    } else {
        trimmed.to_owned()
    };
    if is_windows_reserved_stem(&stem) {
        stem.insert(0, '_');
    }
    format!("{stem}.{}", format.extension())
}

/// Choose and publish a prepared artifact through the production backend.
pub fn export_shortcut_artifact(
    artifact: &PreparedShortcutArtifact,
) -> Result<ShortcutArtifactExportOutcome, ShortcutArtifactIoError> {
    export_shortcut_artifact_with_io(artifact, &NativeShortcutArtifactIo)
}

pub fn validate_artifact_bytes(
    format: ShortcutArtifactFormat,
    bytes: &[u8],
) -> Result<(), ShortcutArtifactIoError> {
    if bytes.len() as u64 > format.max_bytes() {
        return Err(ShortcutArtifactIoError::new(
            "shortcut-artifact.byte-limit",
            format!(
                "{} shortcut artifact is {} bytes; maximum is {} bytes",
                format.label(),
                bytes.len(),
                format.max_bytes()
            ),
        ));
    }
    if format.is_text() && std::str::from_utf8(bytes).is_err() {
        return Err(ShortcutArtifactIoError::new(
            "shortcut-artifact.invalid-utf8",
            format!("{} shortcut artifact is not valid UTF-8", format.label()),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedArtifactDestination {
    pub path: PathBuf,
    expectation: ArtifactDestinationExpectation,
}

impl ObservedArtifactDestination {
    pub fn backend_managed(path: PathBuf) -> Self {
        Self {
            path,
            expectation: ArtifactDestinationExpectation::BackendManaged,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArtifactDestinationExpectation {
    #[cfg(any(test, not(target_arch = "wasm32")))]
    Missing,
    #[cfg(any(test, not(target_arch = "wasm32")))]
    Digest([u8; 32]),
    BackendManaged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactPublication {
    #[cfg(any(test, not(target_arch = "wasm32")))]
    Durable,
    #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
    DownloadStarted,
}

pub trait ShortcutArtifactIo {
    fn choose_destination(
        &self,
        format: ShortcutArtifactFormat,
        default_filename: &str,
    ) -> Result<Option<PathBuf>, String>;

    fn observe_destination(&self, path: &Path) -> Result<ObservedArtifactDestination, String> {
        Ok(ObservedArtifactDestination::backend_managed(
            path.to_path_buf(),
        ))
    }

    fn publish_observed(
        &self,
        destination: &ObservedArtifactDestination,
        artifact: &PreparedShortcutArtifact,
    ) -> Result<ArtifactPublication, String>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NativeShortcutArtifactIo;

impl ShortcutArtifactIo for NativeShortcutArtifactIo {
    #[cfg(not(target_arch = "wasm32"))]
    fn choose_destination(
        &self,
        format: ShortcutArtifactFormat,
        default_filename: &str,
    ) -> Result<Option<PathBuf>, String> {
        Ok(rfd::FileDialog::new()
            .add_filter(format.filter_name(), &[format.extension()])
            .set_file_name(default_filename)
            .set_title("Export Keyboard Shortcut Map")
            .save_file())
    }

    #[cfg(target_arch = "wasm32")]
    fn choose_destination(
        &self,
        _format: ShortcutArtifactFormat,
        default_filename: &str,
    ) -> Result<Option<PathBuf>, String> {
        Ok(Some(PathBuf::from(default_filename)))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn observe_destination(&self, path: &Path) -> Result<ObservedArtifactDestination, String> {
        let expectation = match crate::io::durable_file::observe_expected_content(path)
            .map_err(|error| error.to_string())?
        {
            crate::io::durable_file::ExpectedContent::Missing => {
                ArtifactDestinationExpectation::Missing
            }
            crate::io::durable_file::ExpectedContent::Digest(digest) => {
                ArtifactDestinationExpectation::Digest(digest)
            }
        };
        Ok(ObservedArtifactDestination {
            path: path.to_path_buf(),
            expectation,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn publish_observed(
        &self,
        destination: &ObservedArtifactDestination,
        artifact: &PreparedShortcutArtifact,
    ) -> Result<ArtifactPublication, String> {
        let expected = match destination.expectation {
            ArtifactDestinationExpectation::Missing => {
                crate::io::durable_file::ExpectedContent::Missing
            }
            ArtifactDestinationExpectation::Digest(digest) => {
                crate::io::durable_file::ExpectedContent::Digest(digest)
            }
            ArtifactDestinationExpectation::BackendManaged => {
                return Err(format!(
                    "artifact destination '{}' has no native publication authorization",
                    destination.path.display()
                ));
            }
        };
        crate::io::durable_file::compare_exchange_bytes(
            &destination.path,
            expected,
            artifact.bytes(),
        )
        .map_err(|error| error.to_string())?;
        Ok(ArtifactPublication::Durable)
    }

    #[cfg(target_arch = "wasm32")]
    fn publish_observed(
        &self,
        destination: &ObservedArtifactDestination,
        artifact: &PreparedShortcutArtifact,
    ) -> Result<ArtifactPublication, String> {
        crate::workbench::browser::download::download_bytes_file(
            &destination.path,
            artifact.bytes(),
            artifact.format().mime_type(),
        )?;
        Ok(ArtifactPublication::DownloadStarted)
    }
}

pub fn export_shortcut_artifact_with_io(
    artifact: &PreparedShortcutArtifact,
    io: &(impl ShortcutArtifactIo + ?Sized),
) -> Result<ShortcutArtifactExportOutcome, ShortcutArtifactIoError> {
    validate_artifact_bytes(artifact.format(), artifact.bytes())?;
    let selected = io
        .choose_destination(artifact.format(), artifact.filename())
        .map_err(|error| {
            ShortcutArtifactIoError::new(
                "shortcut-artifact.export-picker",
                format!("could not open shortcut artifact save picker: {error}"),
            )
        })?;
    let Some(selected) = selected else {
        return Ok(ShortcutArtifactExportOutcome::Cancelled);
    };
    validate_selected_extension(&selected, artifact.format())?;
    // Picker overwrite authorization belongs to this exact path. Never
    // normalize or retarget it after the picker returns.
    let path = selected;
    let destination = io.observe_destination(&path).map_err(|error| {
        ShortcutArtifactIoError::new(
            "shortcut-artifact.export-observe",
            format!(
                "could not observe shortcut artifact destination '{}': {error}",
                path.display()
            ),
        )
    })?;
    let publication = io
        .publish_observed(&destination, artifact)
        .map_err(|error| {
            ShortcutArtifactIoError::new(
                "shortcut-artifact.export-publish",
                format!(
                    "could not publish shortcut artifact '{}': {error}",
                    path.display()
                ),
            )
        })?;
    Ok(match publication {
        #[cfg(any(test, not(target_arch = "wasm32")))]
        ArtifactPublication::Durable => ShortcutArtifactExportOutcome::Published { path },
        #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
        ArtifactPublication::DownloadStarted => ShortcutArtifactExportOutcome::DownloadStarted {
            suggested_filename: path
                .file_name()
                .and_then(|filename| filename.to_str())
                .unwrap_or_else(|| artifact.filename())
                .to_owned(),
        },
    })
}

fn validate_selected_extension(
    selected: &Path,
    format: ShortcutArtifactFormat,
) -> Result<(), ShortcutArtifactIoError> {
    let matches = selected
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(format.extension()));
    if matches {
        return Ok(());
    }
    Err(ShortcutArtifactIoError::new(
        "shortcut-artifact.export-extension",
        format!(
            "selected shortcut artifact destination must use the .{} extension",
            format.extension()
        ),
    ))
}

fn strip_artifact_extension(filename: &str) -> &str {
    let lower = filename.to_ascii_lowercase();
    for extension in [".markdown", ".json", ".pdf", ".md"] {
        if lower.ends_with(extension) {
            return &filename[..filename.len() - extension.len()];
        }
    }
    filename
}

fn truncate_utf8(value: &mut String, max_bytes: usize) {
    if value.len() <= max_bytes {
        return;
    }
    let mut boundary = max_bytes;
    while !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    value.truncate(boundary);
}

fn is_windows_reserved_stem(stem: &str) -> bool {
    let basename = stem.split('.').next().unwrap_or(stem).to_ascii_uppercase();
    matches!(basename.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || basename
            .strip_prefix("COM")
            .or_else(|| basename.strip_prefix("LPT"))
            .is_some_and(|suffix| suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9'))
}
