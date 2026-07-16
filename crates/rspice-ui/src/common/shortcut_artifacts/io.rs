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
    crate::common::shortcut_profile_workflow::MAX_SHORTCUT_PROFILE_BYTES;
pub const MAX_SHORTCUT_MARKDOWN_ARTIFACT_BYTES: u64 = 4 * 1024 * 1024;
pub const MAX_SHORTCUT_PDF_ARTIFACT_BYTES: u64 = 16 * 1024 * 1024;

const MAX_FILENAME_STEM_BYTES: usize = 96;

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
    format: ShortcutArtifactFormat,
    filename: String,
    bytes: Vec<u8>,
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
    Published { path: PathBuf },
    DownloadStarted { suggested_filename: String },
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

fn validate_artifact_bytes(
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
struct ObservedArtifactDestination {
    path: PathBuf,
    expectation: ArtifactDestinationExpectation,
}

impl ObservedArtifactDestination {
    fn backend_managed(path: PathBuf) -> Self {
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
enum ArtifactPublication {
    #[cfg(any(test, not(target_arch = "wasm32")))]
    Durable,
    #[cfg(any(test, target_arch = "wasm32"))]
    DownloadStarted,
}

trait ShortcutArtifactIo {
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
struct NativeShortcutArtifactIo;

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
        crate::common::browser_download::download_bytes_file(
            &destination.path,
            artifact.bytes(),
            artifact.format().mime_type(),
        )?;
        Ok(ArtifactPublication::DownloadStarted)
    }
}

fn export_shortcut_artifact_with_io(
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
        #[cfg(any(test, target_arch = "wasm32"))]
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use egui::os::OperatingSystem;

    use super::*;
    use crate::common::shortcut_artifacts::projection::{
        ShortcutExportRequest, ShortcutExportScope, build_shortcut_reference_model,
    };
    use crate::workbench::ShortcutPreferences;
    use crate::workbench::commands::{CommandPlatform, ShortcutContext};

    fn reference_model() -> ShortcutReferenceModel {
        build_shortcut_reference_model(
            &ShortcutPreferences::default(),
            &ShortcutExportRequest {
                scope: ShortcutExportScope::CompleteResolved,
                include_platform_mappings: false,
                runtime_platform: CommandPlatform::Desktop,
                operating_system: OperatingSystem::Windows,
                current_contexts: vec![ShortcutContext::ResultsWorkspace],
            },
        )
        .unwrap()
    }

    fn prepared_json() -> PreparedShortcutArtifact {
        prepare_shortcut_artifact(&reference_model(), ShortcutArtifactFormat::Json, None).unwrap()
    }

    #[test]
    fn format_metadata_is_exact_and_complete() {
        assert_eq!(ShortcutArtifactFormat::ALL.len(), 3);
        assert_eq!(ShortcutArtifactFormat::Json.label(), "JSON");
        assert_eq!(ShortcutArtifactFormat::Json.extension(), "json");
        assert_eq!(ShortcutArtifactFormat::Json.mime_type(), "application/json");
        assert_eq!(
            ShortcutArtifactFormat::Json.default_filename(),
            "rspice-shortcuts.json"
        );
        assert_eq!(
            ShortcutArtifactFormat::Markdown.mime_type(),
            "text/markdown;charset=utf-8"
        );
        assert_eq!(
            ShortcutArtifactFormat::Markdown.default_filename(),
            "rspice-shortcuts.md"
        );
        assert_eq!(ShortcutArtifactFormat::Pdf.mime_type(), "application/pdf");
        assert_eq!(
            ShortcutArtifactFormat::Pdf.default_filename(),
            "rspice-shortcuts.pdf"
        );
        assert_eq!(
            ShortcutArtifactFormat::Json.max_bytes(),
            MAX_SHORTCUT_JSON_ARTIFACT_BYTES
        );
    }

    #[test]
    fn filename_normalization_discards_paths_extensions_and_reserved_names() {
        assert_eq!(
            normalize_shortcut_artifact_filename(
                r"C:\private\project\My shortcuts.PDF",
                ShortcutArtifactFormat::Json,
            ),
            "My shortcuts.json"
        );
        assert_eq!(
            normalize_shortcut_artifact_filename("../../CON.json", ShortcutArtifactFormat::Pdf),
            "_CON.pdf"
        );
        assert_eq!(
            normalize_shortcut_artifact_filename("<>:*?|", ShortcutArtifactFormat::Markdown),
            "rspice-shortcuts.md"
        );
        let long = "electrical-engineering-shortcut-reference-".repeat(8);
        let normalized = normalize_shortcut_artifact_filename(&long, ShortcutArtifactFormat::Json);
        assert!(normalized.len() <= MAX_FILENAME_STEM_BYTES + ".json".len());
    }

    #[test]
    fn preparation_preserves_serializer_bytes_exactly() {
        let model = reference_model();
        let artifact = prepare_shortcut_artifact(
            &model,
            ShortcutArtifactFormat::Json,
            Some("portable shortcuts"),
        )
        .unwrap();
        assert_eq!(artifact.filename(), "portable shortcuts.json");
        assert_eq!(
            artifact.bytes(),
            serialize_shortcut_reference_json(&model)
                .unwrap()
                .as_bytes()
        );
    }

    type FakeWrite = (PathBuf, ShortcutArtifactFormat, String, Vec<u8>);

    struct FakeIo {
        selection: RefCell<Result<Option<PathBuf>, String>>,
        observe_error: RefCell<Option<String>>,
        publish_error: RefCell<Option<String>>,
        publication: RefCell<Option<ArtifactPublication>>,
        observations: RefCell<Vec<PathBuf>>,
        writes: RefCell<Vec<FakeWrite>>,
    }

    impl Default for FakeIo {
        fn default() -> Self {
            Self {
                selection: RefCell::new(Ok(None)),
                observe_error: RefCell::new(None),
                publish_error: RefCell::new(None),
                publication: RefCell::new(None),
                observations: RefCell::new(Vec::new()),
                writes: RefCell::new(Vec::new()),
            }
        }
    }

    impl FakeIo {
        fn selecting(path: impl Into<PathBuf>) -> Self {
            Self {
                selection: RefCell::new(Ok(Some(path.into()))),
                publication: RefCell::new(Some(ArtifactPublication::Durable)),
                ..Self::default()
            }
        }
    }

    impl ShortcutArtifactIo for FakeIo {
        fn choose_destination(
            &self,
            _format: ShortcutArtifactFormat,
            _default_filename: &str,
        ) -> Result<Option<PathBuf>, String> {
            self.selection.borrow().clone()
        }

        fn observe_destination(&self, path: &Path) -> Result<ObservedArtifactDestination, String> {
            if let Some(error) = self.observe_error.borrow().clone() {
                return Err(error);
            }
            self.observations.borrow_mut().push(path.to_path_buf());
            Ok(ObservedArtifactDestination::backend_managed(
                path.to_path_buf(),
            ))
        }

        fn publish_observed(
            &self,
            destination: &ObservedArtifactDestination,
            artifact: &PreparedShortcutArtifact,
        ) -> Result<ArtifactPublication, String> {
            if let Some(error) = self.publish_error.borrow().clone() {
                return Err(error);
            }
            self.writes.borrow_mut().push((
                destination.path.clone(),
                artifact.format(),
                artifact.format().mime_type().to_owned(),
                artifact.bytes().to_vec(),
            ));
            self.publication
                .borrow()
                .ok_or_else(|| "fake publication disposition is missing".to_owned())
        }
    }

    #[test]
    fn cancellation_is_not_an_error_and_publishes_nothing() {
        let io = FakeIo::default();
        let outcome = export_shortcut_artifact_with_io(&prepared_json(), &io).unwrap();
        assert_eq!(outcome, ShortcutArtifactExportOutcome::Cancelled);
        assert!(io.writes.borrow().is_empty());
    }

    #[test]
    fn backend_errors_are_typed_by_stage() {
        let picker = FakeIo {
            selection: RefCell::new(Err("picker unavailable".to_owned())),
            ..FakeIo::default()
        };
        assert_eq!(
            export_shortcut_artifact_with_io(&prepared_json(), &picker)
                .unwrap_err()
                .code(),
            "shortcut-artifact.export-picker"
        );

        let publish = FakeIo::selecting("shortcuts.json");
        *publish.publish_error.borrow_mut() = Some("sink rejected bytes".to_owned());
        assert_eq!(
            export_shortcut_artifact_with_io(&prepared_json(), &publish)
                .unwrap_err()
                .code(),
            "shortcut-artifact.export-publish"
        );
    }

    #[test]
    fn fake_backend_receives_identical_bytes_and_truthful_download_outcome() {
        let artifact = prepared_json();
        let selected = PathBuf::from("folder").join("reference.JSON");
        let io = FakeIo::selecting(&selected);
        *io.publication.borrow_mut() = Some(ArtifactPublication::DownloadStarted);
        let outcome = export_shortcut_artifact_with_io(&artifact, &io).unwrap();
        assert_eq!(
            outcome,
            ShortcutArtifactExportOutcome::DownloadStarted {
                suggested_filename: "reference.JSON".to_owned()
            }
        );
        let writes = io.writes.borrow();
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0].1, ShortcutArtifactFormat::Json);
        assert_eq!(writes[0].2, "application/json");
        assert_eq!(writes[0].3, artifact.bytes());
        assert_eq!(writes[0].0, selected);
    }

    #[test]
    fn picker_authority_cannot_be_retargeted_to_a_different_extension() {
        let artifact = prepared_json();
        for selected in ["authorized.txt", "missing-extension"] {
            let io = FakeIo::selecting(selected);
            let error = export_shortcut_artifact_with_io(&artifact, &io).unwrap_err();
            assert_eq!(error.code(), "shortcut-artifact.export-extension");
            assert!(io.observations.borrow().is_empty());
            assert!(io.writes.borrow().is_empty());
        }

        let exact = PathBuf::from("Authorized.JSON");
        let io = FakeIo::selecting(&exact);
        let outcome = export_shortcut_artifact_with_io(&artifact, &io).unwrap();
        assert_eq!(
            outcome,
            ShortcutArtifactExportOutcome::Published {
                path: exact.clone()
            }
        );
        assert_eq!(&*io.observations.borrow(), &[exact.clone()]);
        assert_eq!(io.writes.borrow()[0].0, exact);
    }

    #[test]
    fn artifact_limits_and_text_encoding_fail_closed() {
        let too_large = vec![0; MAX_SHORTCUT_JSON_ARTIFACT_BYTES as usize + 1];
        assert_eq!(
            validate_artifact_bytes(ShortcutArtifactFormat::Json, &too_large)
                .unwrap_err()
                .code(),
            "shortcut-artifact.byte-limit"
        );
        assert_eq!(
            validate_artifact_bytes(ShortcutArtifactFormat::Markdown, &[0xff])
                .unwrap_err()
                .code(),
            "shortcut-artifact.invalid-utf8"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_cas_rejects_late_edit_and_preserves_external_text() {
        let root = unique_temp_dir("late-edit");
        let path = root.join("shortcuts.json");
        std::fs::write(&path, b"picker-time text").unwrap();
        let backend = NativeShortcutArtifactIo;
        let destination = backend.observe_destination(&path).unwrap();
        std::fs::write(&path, b"late external text").unwrap();

        let error = backend
            .publish_observed(&destination, &prepared_json())
            .unwrap_err();
        assert!(error.contains("destination changed"), "{error}");
        assert_eq!(std::fs::read(&path).unwrap(), b"late external text");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_cas_rejects_late_creation_and_preserves_external_binary() {
        let root = unique_temp_dir("late-creation");
        let path = root.join("shortcuts.pdf");
        let backend = NativeShortcutArtifactIo;
        let destination = backend.observe_destination(&path).unwrap();
        let external = [0x00, 0xff, 0x13, 0x37];
        std::fs::write(&path, external).unwrap();
        let artifact = PreparedShortcutArtifact {
            format: ShortcutArtifactFormat::Pdf,
            filename: "shortcuts.pdf".to_owned(),
            bytes: b"%PDF-1.7\nexact local bytes".to_vec(),
        };

        let error = backend
            .publish_observed(&destination, &artifact)
            .unwrap_err();
        assert!(error.contains("destination changed"), "{error}");
        assert_eq!(std::fs::read(&path).unwrap(), external);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_binary_publication_is_byte_exact() {
        let root = unique_temp_dir("binary-byte-identity");
        let path = root.join("shortcuts.pdf");
        let backend = NativeShortcutArtifactIo;
        let destination = backend.observe_destination(&path).unwrap();
        let bytes = vec![b'%', b'P', b'D', b'F', 0, 0xff, 0x13, 0x37];
        let artifact = PreparedShortcutArtifact {
            format: ShortcutArtifactFormat::Pdf,
            filename: "shortcuts.pdf".to_owned(),
            bytes: bytes.clone(),
        };

        assert_eq!(
            backend.publish_observed(&destination, &artifact).unwrap(),
            ArtifactPublication::Durable
        );
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn unique_temp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-shortcut-artifact-{label}-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).unwrap();
        root
    }
}
