//! Deterministic, fail-closed adaptation of VS Code `keybindings.json` files.
//!
//! This module is deliberately independent of UI and filesystem services. It
//! retains the exact source bytes for audit/rollback, but only emits a
//! canonical native RSpice artifact after every imported construct has been
//! accounted for.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use egui::Key;
use serde_json::{Map, Value};

use super::schema::{
    DecodedShortcutArtifact, SHORTCUT_ARTIFACT_FORMAT, SHORTCUT_ARTIFACT_SCHEMA_VERSION,
    ShortcutArtifactSchemaError, decode_shortcut_artifact_json_with_provenance,
};
use super::{canonical_json_bytes, sha256};
use crate::common::shortcut_profile_workflow::MAX_SHORTCUT_PROFILE_BYTES;
use crate::workbench::commands::{Command, CommandPlatform};
use crate::workbench::shortcuts::{
    CommandShortcutOverride, ContextPrecedencePolicy, ProfileShortcutBinding, ShortcutBindingSlot,
    ShortcutSequence, ShortcutStroke, shortcut_context_precedence_rank, shortcut_contexts_overlap,
};

/// Version of the audited VS Code-to-RSpice command identity table.
pub const VSCODE_MAPPING_VERSION: u16 = 1;

/// One exact, reviewable command identity conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VscodeCommandMapping {
    pub vscode_command_id: &'static str,
    pub rspice_command: Command,
}

/// Supported VS Code command identities. Unknown identities are never guessed.
pub const VSCODE_COMMAND_MAPPINGS: &[VscodeCommandMapping] = &[
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.files.save",
        rspice_command: Command::Save,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.files.saveAs",
        rspice_command: Command::SaveAs,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.files.saveAll",
        rspice_command: Command::SaveAll,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.closeActiveEditor",
        rspice_command: Command::CloseActiveDocument,
    },
    VscodeCommandMapping {
        vscode_command_id: "undo",
        rspice_command: Command::Undo,
    },
    VscodeCommandMapping {
        vscode_command_id: "redo",
        rspice_command: Command::Redo,
    },
    VscodeCommandMapping {
        vscode_command_id: "editor.action.clipboardCutAction",
        rspice_command: Command::Cut,
    },
    VscodeCommandMapping {
        vscode_command_id: "editor.action.clipboardCopyAction",
        rspice_command: Command::Copy,
    },
    VscodeCommandMapping {
        vscode_command_id: "editor.action.clipboardPasteAction",
        rspice_command: Command::Paste,
    },
    VscodeCommandMapping {
        vscode_command_id: "editor.action.selectAll",
        rspice_command: Command::SelectAll,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.openSettings",
        rspice_command: Command::Preferences,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.showCommands",
        rspice_command: Command::CommandPalette,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.openGlobalKeybindings",
        rspice_command: Command::KeyboardShortcuts,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.toggleFullScreen",
        rspice_command: Command::ToggleFullScreen,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.toggleSidebarVisibility",
        rspice_command: Command::ToggleNavigator,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.togglePanel",
        rspice_command: Command::ToggleConsole,
    },
    VscodeCommandMapping {
        vscode_command_id: "workbench.action.toggleZenMode",
        rspice_command: Command::ToggleFocusMode,
    },
];

/// Host whose VS Code platform override and modifier semantics are selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VscodeHostPlatform {
    Windows,
    Macos,
    Linux,
}

impl VscodeHostPlatform {
    const fn key_field(self) -> &'static str {
        match self {
            Self::Windows => "win",
            Self::Macos => "mac",
            Self::Linux => "linux",
        }
    }
}

/// Machine-readable severity for an adaptation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VscodeDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// Stable category for every accepted, ignored, or rejected VS Code construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VscodeDiagnosticCode {
    MalformedEntry,
    MissingCommand,
    UnsupportedCommand,
    UnsupportedWhen,
    UnsupportedArguments,
    UnsupportedKey,
    PlatformVariantNotImported,
    UnsupportedField,
    DuplicateBinding,
    ConflictingBinding,
    CrossCommandCollision,
    UnsupportedRemoval,
}

/// One source-entry-scoped adaptation finding. Messages never include key
/// predicates, arguments, source paths, or source snippets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VscodeEntryDiagnostic {
    pub entry_index: usize,
    pub command_id: Option<String>,
    pub severity: VscodeDiagnosticSeverity,
    pub code: VscodeDiagnosticCode,
    pub message: String,
}

/// Exactly one final disposition is recorded for every source array entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VscodeEntryDisposition {
    Imported,
    Duplicate,
    Unmapped,
    Rejected,
    Conflicting,
}

/// Final source-entry accounting, separate from its potentially multiple
/// explanatory diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VscodeEntryOutcome {
    pub entry_index: usize,
    pub command_id: Option<String>,
    pub disposition: VscodeEntryDisposition,
}

/// Complete deterministic accounting of a VS Code adaptation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VscodeImportReport {
    pub mapping_version: u16,
    pub total_entries: usize,
    pub imported_entries: usize,
    pub duplicate_entries: usize,
    pub unmapped_entries: usize,
    pub rejected_entries: usize,
    pub conflicting_entries: usize,
    pub unmapped_command_ids: Vec<String>,
    pub entry_outcomes: Vec<VscodeEntryOutcome>,
    pub diagnostics: Vec<VscodeEntryDiagnostic>,
}

impl VscodeImportReport {
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == VscodeDiagnosticSeverity::Error)
    }

    /// Import is permitted only when conversion is lossless and changes at
    /// least one known RSpice command.
    #[must_use]
    pub fn is_importable(&self) -> bool {
        !self.has_errors() && self.imported_entries != 0
    }
}

/// A native artifact detected from source, retaining its exact input bytes.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedRspiceShortcutArtifact {
    source_bytes: Vec<u8>,
    source_digest: [u8; 32],
    artifact: DecodedShortcutArtifact,
}

impl DetectedRspiceShortcutArtifact {
    #[must_use]
    pub fn source_bytes(&self) -> &[u8] {
        &self.source_bytes
    }

    #[must_use]
    pub const fn source_digest(&self) -> [u8; 32] {
        self.source_digest
    }

    #[must_use]
    pub const fn artifact(&self) -> &DecodedShortcutArtifact {
        &self.artifact
    }

    #[must_use]
    pub fn into_artifact(self) -> DecodedShortcutArtifact {
        self.artifact
    }
}

/// Canonical RSpice artifact plus the complete VS Code conversion audit.
#[derive(Debug, Clone, PartialEq)]
pub struct VscodeShortcutAdaptation {
    source_bytes: Vec<u8>,
    source_digest: [u8; 32],
    canonical_artifact_json: String,
    artifact: DecodedShortcutArtifact,
    report: VscodeImportReport,
}

impl VscodeShortcutAdaptation {
    #[must_use]
    pub fn source_bytes(&self) -> &[u8] {
        &self.source_bytes
    }

    #[must_use]
    pub const fn source_digest(&self) -> [u8; 32] {
        self.source_digest
    }

    #[must_use]
    pub fn canonical_artifact_json(&self) -> &str {
        &self.canonical_artifact_json
    }

    #[must_use]
    pub const fn artifact(&self) -> &DecodedShortcutArtifact {
        &self.artifact
    }

    #[must_use]
    pub const fn report(&self) -> &VscodeImportReport {
        &self.report
    }

    pub fn into_parts(self) -> (DecodedShortcutArtifact, VscodeImportReport) {
        (self.artifact, self.report)
    }
}

/// Deterministically detected shortcut source kind.
#[derive(Debug, Clone, PartialEq)]
pub enum DetectedShortcutArtifact {
    RSpice(DetectedRspiceShortcutArtifact),
    Vscode(VscodeShortcutAdaptation),
}

/// Structural detection/adaptation failure. Entry-level conversion failures
/// are returned in [`VscodeImportReport`] so they can be reviewed without
/// applying an unsafe partial import.
#[derive(Debug)]
pub struct VscodeAdapterError {
    code: &'static str,
    message: String,
}

impl VscodeAdapterError {
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

impl fmt::Display for VscodeAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

impl std::error::Error for VscodeAdapterError {}

/// Auto-detect a native RSpice artifact object or VS Code keybindings array.
/// JSON-with-comments and trailing commas are accepted without weakening the
/// native schema boundary.
pub fn detect_shortcut_artifact(
    source_name: impl Into<String>,
    source_bytes: &[u8],
    platform: VscodeHostPlatform,
) -> Result<DetectedShortcutArtifact, VscodeAdapterError> {
    validate_source_size(source_bytes)?;
    let source = std::str::from_utf8(source_bytes).map_err(|_| {
        VscodeAdapterError::new(
            "shortcut-adapter.invalid-utf8",
            "shortcut source must be valid UTF-8",
        )
    })?;
    let normalized = normalize_jsonc(source)?;
    let root: Value = serde_json::from_str(&normalized).map_err(|error| {
        VscodeAdapterError::new(
            "shortcut-adapter.invalid-json",
            format!("shortcut source is not valid JSON: {error}"),
        )
    })?;
    let source_name = normalized_source_name(source_name.into());
    let source_digest = sha256(source_bytes);

    match root {
        Value::Object(object) if object.contains_key("format") => {
            let artifact = decode_shortcut_artifact_json_with_provenance(
                source_name,
                &normalized,
                source_bytes,
            )
            .map_err(schema_adapter_error)?;
            Ok(DetectedShortcutArtifact::RSpice(
                DetectedRspiceShortcutArtifact {
                    source_bytes: source_bytes.to_vec(),
                    source_digest,
                    artifact,
                },
            ))
        }
        Value::Array(entries) => adapt_vscode_entries(
            source_name,
            source_bytes.to_vec(),
            source_digest,
            entries,
            platform,
        )
        .map(DetectedShortcutArtifact::Vscode),
        Value::Object(_) => Err(VscodeAdapterError::new(
            "shortcut-adapter.unknown-object",
            "JSON object is not a native RSpice shortcut artifact",
        )),
        _ => Err(VscodeAdapterError::new(
            "shortcut-adapter.unsupported-root",
            "shortcut source must be a native RSpice object or VS Code keybindings array",
        )),
    }
}

/// Explicit VS Code adapter for callers that already know the source type.
pub fn adapt_vscode_keybindings(
    source_name: impl Into<String>,
    source_bytes: &[u8],
    platform: VscodeHostPlatform,
) -> Result<VscodeShortcutAdaptation, VscodeAdapterError> {
    match detect_shortcut_artifact(source_name, source_bytes, platform)? {
        DetectedShortcutArtifact::Vscode(adaptation) => Ok(adaptation),
        DetectedShortcutArtifact::RSpice(_) => Err(VscodeAdapterError::new(
            "shortcut-adapter.expected-vscode",
            "source is a native RSpice shortcut artifact, not VS Code keybindings",
        )),
    }
}

fn validate_source_size(source_bytes: &[u8]) -> Result<(), VscodeAdapterError> {
    if source_bytes.len() as u64 > MAX_SHORTCUT_PROFILE_BYTES {
        return Err(VscodeAdapterError::new(
            "shortcut-adapter.byte-limit",
            format!(
                "shortcut source is {} bytes; maximum is {MAX_SHORTCUT_PROFILE_BYTES}",
                source_bytes.len()
            ),
        ));
    }
    Ok(())
}

fn schema_adapter_error(error: ShortcutArtifactSchemaError) -> VscodeAdapterError {
    VscodeAdapterError::new(error.code(), error.to_string())
}

fn normalized_source_name(source_name: String) -> String {
    let source_name = source_name.trim();
    if source_name.is_empty() {
        "shortcut profile".to_owned()
    } else {
        source_name
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(source_name)
            .to_owned()
    }
}

fn normalize_jsonc(source: &str) -> Result<String, VscodeAdapterError> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        Normal,
        String,
        LineComment,
        BlockComment,
    }

    let mut state = State::Normal;
    let mut escaped = false;
    let mut chars = source.chars().peekable();
    let mut output = String::with_capacity(source.len());

    while let Some(character) = chars.next() {
        match state {
            State::Normal => match character {
                '"' => {
                    output.push(character);
                    state = State::String;
                }
                '/' if chars.peek() == Some(&'/') => {
                    chars.next();
                    output.push_str("  ");
                    state = State::LineComment;
                }
                '/' if chars.peek() == Some(&'*') => {
                    chars.next();
                    output.push_str("  ");
                    state = State::BlockComment;
                }
                _ => output.push(character),
            },
            State::String => {
                output.push(character);
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == '"' {
                    state = State::Normal;
                }
            }
            State::LineComment => {
                if matches!(character, '\n' | '\r') {
                    output.push(character);
                    state = State::Normal;
                } else {
                    output.push(' ');
                }
            }
            State::BlockComment => {
                if character == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    output.push_str("  ");
                    state = State::Normal;
                } else if matches!(character, '\n' | '\r') {
                    output.push(character);
                } else {
                    output.push(' ');
                }
            }
        }
    }

    match state {
        State::String => Err(VscodeAdapterError::new(
            "shortcut-adapter.unterminated-string",
            "shortcut source contains an unterminated JSON string",
        )),
        State::BlockComment => Err(VscodeAdapterError::new(
            "shortcut-adapter.unterminated-comment",
            "shortcut source contains an unterminated block comment",
        )),
        State::Normal | State::LineComment => Ok(remove_trailing_commas(&output)),
    }
}

fn remove_trailing_commas(source: &str) -> String {
    let characters = source.chars().collect::<Vec<_>>();
    let mut output = String::with_capacity(source.len());
    let mut in_string = false;
    let mut escaped = false;

    for (index, character) in characters.iter().copied().enumerate() {
        if in_string {
            output.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        if character == '"' {
            in_string = true;
            output.push(character);
            continue;
        }
        if character == ',' {
            let next = characters[index + 1..]
                .iter()
                .copied()
                .find(|candidate| !candidate.is_whitespace());
            if matches!(next, Some(']' | '}')) {
                continue;
            }
        }
        output.push(character);
    }
    output
}

#[derive(Debug, Clone)]
struct CandidateBinding {
    entry_index: usize,
    source_command_id: &'static str,
    command: Command,
    sequence: ShortcutSequence,
}

struct ReportBuilder {
    report: VscodeImportReport,
    unmapped_command_ids: BTreeSet<String>,
    outcomes: Vec<Option<VscodeEntryOutcome>>,
}

impl ReportBuilder {
    fn new(total_entries: usize) -> Self {
        Self {
            report: VscodeImportReport {
                mapping_version: VSCODE_MAPPING_VERSION,
                total_entries,
                imported_entries: 0,
                duplicate_entries: 0,
                unmapped_entries: 0,
                rejected_entries: 0,
                conflicting_entries: 0,
                unmapped_command_ids: Vec::new(),
                entry_outcomes: Vec::new(),
                diagnostics: Vec::new(),
            },
            unmapped_command_ids: BTreeSet::new(),
            outcomes: vec![None; total_entries],
        }
    }

    fn diagnostic(
        &mut self,
        entry_index: usize,
        command_id: Option<&str>,
        severity: VscodeDiagnosticSeverity,
        code: VscodeDiagnosticCode,
        message: impl Into<String>,
    ) {
        self.report.diagnostics.push(VscodeEntryDiagnostic {
            entry_index,
            command_id: command_id.map(ToOwned::to_owned),
            severity,
            code,
            message: message.into(),
        });
    }

    fn disposition(
        &mut self,
        entry_index: usize,
        command_id: Option<&str>,
        disposition: VscodeEntryDisposition,
    ) {
        self.outcomes[entry_index] = Some(VscodeEntryOutcome {
            entry_index,
            command_id: command_id.map(ToOwned::to_owned),
            disposition,
        });
    }

    fn unmapped(&mut self, entry_index: usize, command_id: &str) {
        self.unmapped_command_ids.insert(command_id.to_owned());
        self.disposition(
            entry_index,
            Some(command_id),
            VscodeEntryDisposition::Unmapped,
        );
        self.diagnostic(
            entry_index,
            Some(command_id),
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::UnsupportedCommand,
            "VS Code command has no exact RSpice identity mapping",
        );
    }

    fn finish(mut self, candidates: &BTreeMap<String, CandidateBinding>) -> VscodeImportReport {
        for candidate in candidates.values() {
            if self.outcomes[candidate.entry_index].is_none() {
                self.disposition(
                    candidate.entry_index,
                    Some(candidate.source_command_id),
                    VscodeEntryDisposition::Imported,
                );
            }
        }
        for entry_index in 0..self.outcomes.len() {
            if self.outcomes[entry_index].is_none() {
                self.diagnostic(
                    entry_index,
                    None,
                    VscodeDiagnosticSeverity::Error,
                    VscodeDiagnosticCode::MalformedEntry,
                    "entry did not reach a safe import disposition",
                );
                self.disposition(entry_index, None, VscodeEntryDisposition::Rejected);
            }
        }
        self.report.entry_outcomes = self.outcomes.into_iter().flatten().collect();
        for outcome in &self.report.entry_outcomes {
            match outcome.disposition {
                VscodeEntryDisposition::Imported => self.report.imported_entries += 1,
                VscodeEntryDisposition::Duplicate => self.report.duplicate_entries += 1,
                VscodeEntryDisposition::Unmapped => self.report.unmapped_entries += 1,
                VscodeEntryDisposition::Rejected => self.report.rejected_entries += 1,
                VscodeEntryDisposition::Conflicting => self.report.conflicting_entries += 1,
            }
        }
        self.report.unmapped_command_ids = self.unmapped_command_ids.into_iter().collect();
        self.report
    }
}

fn adapt_vscode_entries(
    source_name: String,
    source_bytes: Vec<u8>,
    source_digest: [u8; 32],
    entries: Vec<Value>,
    platform: VscodeHostPlatform,
) -> Result<VscodeShortcutAdaptation, VscodeAdapterError> {
    let mut report = ReportBuilder::new(entries.len());
    let mut candidates = BTreeMap::<String, CandidateBinding>::new();
    let mut conflicted_commands = BTreeMap::<String, usize>::new();

    for (entry_index, entry) in entries.iter().enumerate() {
        let Some(candidate) = parse_entry(entry_index, entry, platform, &mut report) else {
            continue;
        };
        let target_id = candidate.command.stable_id().to_owned();

        if let Some(first_index) = conflicted_commands.get(&target_id).copied() {
            report.disposition(
                entry_index,
                Some(candidate.source_command_id),
                VscodeEntryDisposition::Conflicting,
            );
            report.diagnostic(
                entry_index,
                Some(candidate.source_command_id),
                VscodeDiagnosticSeverity::Error,
                VscodeDiagnosticCode::ConflictingBinding,
                format!(
                    "RSpice command '{}' already has incompatible VS Code entries (first at index {first_index})",
                    candidate.command.stable_id()
                ),
            );
            continue;
        }

        match candidates.get(&target_id) {
            None => {
                candidates.insert(target_id, candidate);
            }
            Some(existing) if existing.sequence == candidate.sequence => {
                report.disposition(
                    entry_index,
                    Some(candidate.source_command_id),
                    VscodeEntryDisposition::Duplicate,
                );
                report.diagnostic(
                    entry_index,
                    Some(candidate.source_command_id),
                    VscodeDiagnosticSeverity::Warning,
                    VscodeDiagnosticCode::DuplicateBinding,
                    format!(
                        "duplicate binding for RSpice command '{}' was deduplicated",
                        candidate.command.stable_id()
                    ),
                );
            }
            Some(existing) => {
                let first_index = existing.entry_index;
                let first_command_id = existing.source_command_id;
                candidates.remove(&target_id);
                conflicted_commands.insert(target_id, first_index);
                report.disposition(
                    first_index,
                    Some(first_command_id),
                    VscodeEntryDisposition::Conflicting,
                );
                report.disposition(
                    entry_index,
                    Some(candidate.source_command_id),
                    VscodeEntryDisposition::Conflicting,
                );
                report.diagnostic(
                    entry_index,
                    Some(candidate.source_command_id),
                    VscodeDiagnosticSeverity::Error,
                    VscodeDiagnosticCode::ConflictingBinding,
                    format!(
                        "incompatible bindings target RSpice command '{}' (first at index {first_index})",
                        candidate.command.stable_id()
                    ),
                );
            }
        }
    }

    diagnose_cross_command_collisions(&mut candidates, &mut report);
    let report = report.finish(&candidates);
    let canonical_artifact_json = build_native_artifact(&candidates, &report)?;
    let artifact = decode_shortcut_artifact_json_with_provenance(
        source_name,
        &canonical_artifact_json,
        &source_bytes,
    )
    .map_err(schema_adapter_error)?;

    Ok(VscodeShortcutAdaptation {
        source_bytes,
        source_digest,
        canonical_artifact_json,
        artifact,
        report,
    })
}

fn parse_entry(
    entry_index: usize,
    entry: &Value,
    platform: VscodeHostPlatform,
    report: &mut ReportBuilder,
) -> Option<CandidateBinding> {
    let Some(object) = entry.as_object() else {
        report.disposition(entry_index, None, VscodeEntryDisposition::Rejected);
        report.diagnostic(
            entry_index,
            None,
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::MalformedEntry,
            "VS Code keybinding entry must be a JSON object",
        );
        return None;
    };

    let mut valid = !diagnose_unknown_fields(entry_index, object, report);

    let Some(raw_command) = object.get("command").and_then(Value::as_str) else {
        report.disposition(entry_index, None, VscodeEntryDisposition::Rejected);
        report.diagnostic(
            entry_index,
            None,
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::MissingCommand,
            "VS Code keybinding entry requires a string command identity",
        );
        return None;
    };
    let unbind = raw_command.starts_with('-');
    let command_id = raw_command.strip_prefix('-').unwrap_or(raw_command).trim();
    if command_id.is_empty() {
        report.disposition(entry_index, None, VscodeEntryDisposition::Rejected);
        report.diagnostic(
            entry_index,
            None,
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::MissingCommand,
            "VS Code command identity cannot be empty",
        );
        return None;
    }

    if object.contains_key("args") {
        valid = false;
        report.diagnostic(
            entry_index,
            Some(command_id),
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::UnsupportedArguments,
            "command arguments cannot be represented by RSpice shortcuts",
        );
    }
    if !supported_when(object.get("when")) {
        valid = false;
        report.diagnostic(
            entry_index,
            Some(command_id),
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::UnsupportedWhen,
            "context predicate cannot be represented safely and was rejected",
        );
    }

    let selected_key = selected_platform_key(entry_index, command_id, object, platform, report);
    if selected_key.is_none() {
        valid = false;
    }
    let sequence =
        selected_key
            .as_deref()
            .and_then(|key| match parse_key_sequence(key, platform) {
                Ok(sequence) => Some(sequence),
                Err(message) => {
                    valid = false;
                    report.diagnostic(
                        entry_index,
                        Some(command_id),
                        VscodeDiagnosticSeverity::Error,
                        VscodeDiagnosticCode::UnsupportedKey,
                        message,
                    );
                    None
                }
            });

    if unbind {
        report.diagnostic(
            entry_index,
            Some(command_id),
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::UnsupportedRemoval,
            "VS Code key-specific removal cannot be converted to a whole-command RSpice override",
        );
    }

    let Some(mapping) = mapping_for(command_id) else {
        report.unmapped(entry_index, command_id);
        return None;
    };
    if unbind {
        report.disposition(
            entry_index,
            Some(command_id),
            VscodeEntryDisposition::Rejected,
        );
        return None;
    }
    if !valid {
        report.disposition(
            entry_index,
            Some(command_id),
            VscodeEntryDisposition::Rejected,
        );
        return None;
    }

    let Some(sequence) = sequence else {
        report.disposition(
            entry_index,
            Some(command_id),
            VscodeEntryDisposition::Rejected,
        );
        return None;
    };
    Some(CandidateBinding {
        entry_index,
        source_command_id: mapping.vscode_command_id,
        command: mapping.rspice_command,
        sequence,
    })
}

fn diagnose_unknown_fields(
    entry_index: usize,
    object: &Map<String, Value>,
    report: &mut ReportBuilder,
) -> bool {
    const KNOWN_FIELDS: &[&str] = &["key", "command", "when", "args", "mac", "win", "linux"];
    let mut unknown_fields = object
        .keys()
        .filter(|field| !KNOWN_FIELDS.contains(&field.as_str()))
        .collect::<Vec<_>>();
    unknown_fields.sort_unstable();
    let has_unknown_fields = !unknown_fields.is_empty();
    for _field in unknown_fields {
        report.diagnostic(
            entry_index,
            None,
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::UnsupportedField,
            "unrecognized VS Code entry field could affect behavior and was rejected",
        );
    }
    has_unknown_fields
}

fn supported_when(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => true,
        Some(Value::String(predicate)) => {
            let predicate = predicate.trim();
            predicate.is_empty() || predicate == "true"
        }
        _ => false,
    }
}

fn selected_platform_key(
    entry_index: usize,
    command_id: &str,
    object: &Map<String, Value>,
    platform: VscodeHostPlatform,
    report: &mut ReportBuilder,
) -> Option<String> {
    let mut values = BTreeMap::<&str, &str>::new();
    let mut valid = true;
    for field in ["key", "mac", "win", "linux"] {
        if let Some(value) = object.get(field) {
            if let Some(key) = value.as_str() {
                values.insert(field, key);
            } else {
                valid = false;
                report.diagnostic(
                    entry_index,
                    Some(command_id),
                    VscodeDiagnosticSeverity::Error,
                    VscodeDiagnosticCode::MalformedEntry,
                    format!("VS Code '{field}' key field must be a string"),
                );
            }
        }
    }
    if !valid {
        return None;
    }

    let selected_field = if values.contains_key(platform.key_field()) {
        platform.key_field()
    } else {
        "key"
    };
    let selected = values.get(selected_field).copied();
    let Some(selected) = selected else {
        report.diagnostic(
            entry_index,
            Some(command_id),
            VscodeDiagnosticSeverity::Error,
            VscodeDiagnosticCode::UnsupportedKey,
            "entry has no key for the selected host platform",
        );
        return None;
    };

    for field in ["key", "mac", "win", "linux"] {
        if field == selected_field {
            continue;
        }
        if let Some(other) = values.get(field).copied()
            && !other.eq_ignore_ascii_case(selected)
        {
            report.diagnostic(
                entry_index,
                Some(command_id),
                VscodeDiagnosticSeverity::Info,
                VscodeDiagnosticCode::PlatformVariantNotImported,
                format!("VS Code '{field}' variant was retained in the audit but not imported"),
            );
        }
    }

    Some(selected.to_owned())
}

fn mapping_for(command_id: &str) -> Option<&'static VscodeCommandMapping> {
    VSCODE_COMMAND_MAPPINGS
        .iter()
        .find(|mapping| mapping.vscode_command_id == command_id)
}

fn parse_key_sequence(
    raw_sequence: &str,
    platform: VscodeHostPlatform,
) -> Result<ShortcutSequence, String> {
    let raw_sequence = raw_sequence.trim();
    if raw_sequence.is_empty() {
        return Err("VS Code key sequence cannot be empty".to_owned());
    }
    let mut strokes = Vec::new();
    for raw_stroke in raw_sequence.split_whitespace() {
        strokes.push(parse_stroke(raw_stroke, platform)?);
    }
    ShortcutSequence::new(strokes)
        .map_err(|error| format!("VS Code key sequence is unsupported: {error}"))
}

fn parse_stroke(raw_stroke: &str, platform: VscodeHostPlatform) -> Result<ShortcutStroke, String> {
    let (modifier_text, key_token) = if raw_stroke == "+" {
        (None, "+")
    } else if let Some(prefix) = raw_stroke.strip_suffix("++") {
        (Some(prefix), "+")
    } else if let Some((modifiers, key)) = raw_stroke.rsplit_once('+') {
        (Some(modifiers), key)
    } else {
        (None, raw_stroke)
    };
    if key_token.is_empty() {
        return Err("VS Code stroke has no key".to_owned());
    }

    let mut primary = false;
    let mut alt = false;
    let mut shift = false;
    if let Some(modifier_text) = modifier_text {
        if modifier_text.is_empty() {
            return Err("VS Code stroke contains an empty modifier".to_owned());
        }
        for modifier in modifier_text.split('+') {
            let flag = match modifier.to_ascii_lowercase().as_str() {
                "alt" | "option" => &mut alt,
                "shift" => &mut shift,
                "cmd" | "meta" | "super" if platform == VscodeHostPlatform::Macos => &mut primary,
                "ctrl" | "control" if platform != VscodeHostPlatform::Macos => &mut primary,
                "ctrl" | "control" => {
                    return Err(
                        "physical Control on macOS cannot be represented as RSpice primary"
                            .to_owned(),
                    );
                }
                "cmd" | "meta" | "super" | "win" => {
                    return Err(
                        "platform system-key modifier cannot be represented as RSpice primary"
                            .to_owned(),
                    );
                }
                _ => return Err("VS Code stroke contains an unsupported modifier".to_owned()),
            };
            if *flag {
                return Err("VS Code stroke repeats a modifier".to_owned());
            }
            *flag = true;
        }
    }

    let key = vscode_key(key_token)
        .ok_or_else(|| "VS Code stroke contains an unsupported logical key".to_owned())?;
    Ok(ShortcutStroke::new(key, primary, alt, shift))
}

fn vscode_key(token: &str) -> Option<Key> {
    let token = token.trim().to_ascii_lowercase();
    let key_name = if token.len() == 1 && token.as_bytes()[0].is_ascii_alphabetic() {
        token.to_ascii_uppercase()
    } else if token.len() == 1 && token.as_bytes()[0].is_ascii_digit() {
        format!("Num{token}")
    } else if let Some(number) = token.strip_prefix('f')
        && number
            .parse::<u8>()
            .is_ok_and(|number| (1..=12).contains(&number))
    {
        token.to_ascii_uppercase()
    } else {
        match token.as_str() {
            "backspace" => "Backspace".to_owned(),
            "delete" | "del" => "Delete".to_owned(),
            "escape" | "esc" => "Escape".to_owned(),
            "enter" | "return" => "Enter".to_owned(),
            "tab" => "Tab".to_owned(),
            "space" => "Space".to_owned(),
            "insert" => "Insert".to_owned(),
            "home" => "Home".to_owned(),
            "end" => "End".to_owned(),
            "pageup" => "PageUp".to_owned(),
            "pagedown" => "PageDown".to_owned(),
            "up" => "ArrowUp".to_owned(),
            "down" => "ArrowDown".to_owned(),
            "left" => "ArrowLeft".to_owned(),
            "right" => "ArrowRight".to_owned(),
            "," | "comma" => "Comma".to_owned(),
            "." | "period" => "Period".to_owned(),
            "+" | "=" | "plus" => "Plus".to_owned(),
            "-" | "minus" => "Minus".to_owned(),
            "[" | "openbracket" => "OpenBracket".to_owned(),
            "]" | "closebracket" => "CloseBracket".to_owned(),
            ";" | "semicolon" => "Semicolon".to_owned(),
            "`" | "backtick" => "Backtick".to_owned(),
            "/" | "slash" => "Slash".to_owned(),
            "\\" | "backslash" => "Backslash".to_owned(),
            _ => return None,
        }
    };
    Key::from_name(&key_name)
}

fn diagnose_cross_command_collisions(
    candidates: &mut BTreeMap<String, CandidateBinding>,
    report: &mut ReportBuilder,
) {
    let mut bindings = candidates.values().cloned().collect::<Vec<_>>();
    bindings.sort_by_key(|candidate| candidate.entry_index);
    let mut collided_commands = BTreeSet::new();
    for (left_index, left) in bindings.iter().enumerate() {
        for right in &bindings[left_index + 1..] {
            let left_context = left.command.shortcut_context();
            let right_context = right.command.shortcut_context();
            let policy = ContextPrecedencePolicy::ModalEditorWorkspaceGlobal;
            if shortcut_contexts_overlap(left_context, right_context)
                && shortcut_context_precedence_rank(left_context, policy)
                    == shortcut_context_precedence_rank(right_context, policy)
                && (left.sequence == right.sequence
                    || left.sequence.is_prefix_of(&right.sequence)
                    || right.sequence.is_prefix_of(&left.sequence))
            {
                collided_commands.insert(left.command.stable_id().to_owned());
                collided_commands.insert(right.command.stable_id().to_owned());
                report.disposition(
                    left.entry_index,
                    Some(left.source_command_id),
                    VscodeEntryDisposition::Conflicting,
                );
                report.disposition(
                    right.entry_index,
                    Some(right.source_command_id),
                    VscodeEntryDisposition::Conflicting,
                );
                report.diagnostic(
                    right.entry_index,
                    Some(right.source_command_id),
                    VscodeDiagnosticSeverity::Error,
                    VscodeDiagnosticCode::CrossCommandCollision,
                    format!(
                        "binding collides with RSpice command '{}' in an overlapping context",
                        left.command.stable_id()
                    ),
                );
            }
        }
    }
    candidates.retain(|command_id, _| !collided_commands.contains(command_id));
}

fn build_native_artifact(
    candidates: &BTreeMap<String, CandidateBinding>,
    report: &VscodeImportReport,
) -> Result<String, VscodeAdapterError> {
    let mut commands = BTreeMap::<String, Value>::new();
    let mut contexts = BTreeSet::<String>::new();
    for candidate in candidates.values() {
        contexts.insert(candidate.command.shortcut_context().label().to_owned());
        let binding = ProfileShortcutBinding::new(
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            candidate.sequence.clone(),
        )
        .map_err(|error| {
            VscodeAdapterError::new(
                "shortcut-adapter.generated-binding",
                format!("could not create adapted shortcut binding: {error}"),
            )
        })?;
        let command_override = CommandShortcutOverride::new(vec![binding]).map_err(|error| {
            VscodeAdapterError::new(
                "shortcut-adapter.generated-override",
                format!("could not create adapted shortcut override: {error}"),
            )
        })?;
        let value = serde_json::to_value(command_override).map_err(|error| {
            VscodeAdapterError::new(
                "shortcut-adapter.serialize-override",
                format!("could not serialize adapted shortcut override: {error}"),
            )
        })?;
        commands.insert(candidate.command.stable_id().to_owned(), value);
    }
    if contexts.is_empty() {
        contexts.insert("all".to_owned());
    }

    let root = serde_json::json!({
        "format": SHORTCUT_ARTIFACT_FORMAT,
        "artifact": {
            "schemaVersion": SHORTCUT_ARTIFACT_SCHEMA_VERSION,
            "scope": "user-overrides",
            "coverage": {
                "contexts": contexts.into_iter().collect::<Vec<_>>(),
                "platforms": [CommandPlatform::Desktop],
                "policiesIncluded": false
            },
            "platformMappingsIncluded": false,
            "unknownCommandsOmitted": report.unmapped_entries
        },
        "profile": { "commands": commands }
    });
    let bytes = canonical_json_bytes(root).map_err(|error| {
        VscodeAdapterError::new(
            "shortcut-adapter.serialize-artifact",
            format!("could not serialize adapted shortcut artifact: {error}"),
        )
    })?;
    String::from_utf8(bytes).map_err(|_| {
        VscodeAdapterError::new(
            "shortcut-adapter.internal-encoding",
            "canonical shortcut artifact was not UTF-8",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::commands::COMMAND_REGISTRY;

    fn vscode(source: &str, platform: VscodeHostPlatform) -> VscodeShortcutAdaptation {
        adapt_vscode_keybindings(r"C:\private\keybindings.json", source.as_bytes(), platform)
            .unwrap()
    }

    fn has_code(report: &VscodeImportReport, code: VscodeDiagnosticCode) -> bool {
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code)
    }

    fn assert_every_entry_accounted_for(report: &VscodeImportReport) {
        assert_eq!(report.entry_outcomes.len(), report.total_entries);
        assert_eq!(
            report
                .entry_outcomes
                .iter()
                .map(|outcome| outcome.entry_index)
                .collect::<Vec<_>>(),
            (0..report.total_entries).collect::<Vec<_>>()
        );
        assert_eq!(
            report.imported_entries
                + report.duplicate_entries
                + report.unmapped_entries
                + report.rejected_entries
                + report.conflicting_entries,
            report.total_entries
        );
    }

    #[test]
    fn mapping_table_is_unique_and_resolves_only_registry_commands() {
        let mut source_ids = BTreeSet::new();
        let mut target_ids = BTreeSet::new();
        for mapping in VSCODE_COMMAND_MAPPINGS {
            assert!(source_ids.insert(mapping.vscode_command_id));
            assert!(target_ids.insert(mapping.rspice_command.stable_id()));
            assert!(COMMAND_REGISTRY.contains(&mapping.rspice_command));
            assert_eq!(
                Command::from_stable_id(mapping.rspice_command.stable_id()),
                Some(mapping.rspice_command)
            );
        }
        for vscode_ui_zoom in [
            "workbench.action.zoomIn",
            "workbench.action.zoomOut",
            "workbench.action.zoomReset",
        ] {
            assert!(mapping_for(vscode_ui_zoom).is_none());
        }
    }

    #[test]
    fn jsonc_chord_adaptation_is_deterministic_and_round_trips_as_native() {
        let source = r#"[
            // VS Code permits comments.
            { "key": "ctrl+k ctrl+c", "command": "workbench.action.closeActiveEditor", },
        ]"#;
        let first = vscode(source, VscodeHostPlatform::Windows);
        let second = vscode(source, VscodeHostPlatform::Windows);

        assert_eq!(first.source_digest(), sha256(source.as_bytes()));
        assert_eq!(first.artifact().source_digest(), sha256(source.as_bytes()));
        assert_eq!(first.source_bytes(), source.as_bytes());
        assert_eq!(
            first.canonical_artifact_json(),
            second.canonical_artifact_json()
        );
        assert_eq!(first.report(), second.report());
        assert!(first.report().is_importable());
        let command_override = first
            .artifact()
            .profile()
            .command_override(Command::CloseActiveDocument)
            .unwrap();
        let strokes = command_override.bindings()[0].sequence().strokes();
        assert_eq!(strokes.len(), 2);
        assert_eq!(strokes[0].key(), Key::K);
        assert_eq!(strokes[1].key(), Key::C);
        assert!(strokes.iter().all(|stroke| stroke.primary()));
        assert_eq!(first.artifact().source_name(), "keybindings.json");

        let detected = detect_shortcut_artifact(
            "adapted.json",
            first.canonical_artifact_json().as_bytes(),
            VscodeHostPlatform::Windows,
        )
        .unwrap();
        let DetectedShortcutArtifact::RSpice(round_trip) = detected else {
            panic!("canonical output must detect as a native artifact");
        };
        assert_eq!(
            round_trip.artifact().portable_profile(),
            first.artifact().portable_profile()
        );
    }

    #[test]
    fn platform_override_and_platform_primary_semantics_are_exact() {
        let source = r#"[{
            "key":"ctrl+s",
            "win":"ctrl+shift+s",
            "mac":"cmd+alt+s",
            "linux":"ctrl+s",
            "command":"workbench.action.files.save"
        }]"#;
        let windows = vscode(source, VscodeHostPlatform::Windows);
        let windows_binding = windows
            .artifact()
            .profile()
            .command_override(Command::Save)
            .unwrap();
        let windows_stroke = windows_binding.bindings()[0].sequence().strokes()[0];
        assert!(windows_stroke.primary());
        assert!(windows_stroke.shift());
        assert!(!windows_stroke.alt());
        assert!(has_code(
            windows.report(),
            VscodeDiagnosticCode::PlatformVariantNotImported
        ));

        let mac = vscode(source, VscodeHostPlatform::Macos);
        let mac_binding = mac
            .artifact()
            .profile()
            .command_override(Command::Save)
            .unwrap();
        let mac_stroke = mac_binding.bindings()[0].sequence().strokes()[0];
        assert!(mac_stroke.primary());
        assert!(mac_stroke.alt());
        assert!(!mac_stroke.shift());
    }

    #[test]
    fn physical_control_on_macos_fails_closed() {
        let adaptation = vscode(
            r#"[{"key":"ctrl+s","command":"workbench.action.files.save"}]"#,
            VscodeHostPlatform::Macos,
        );
        assert!(adaptation.report().has_errors());
        assert!(!adaptation.report().is_importable());
        assert!(has_code(
            adaptation.report(),
            VscodeDiagnosticCode::UnsupportedKey
        ));
    }

    #[test]
    fn duplicate_is_deduplicated_but_conflict_fails_closed() {
        let duplicate = vscode(
            r#"[
                {"key":"ctrl+s","command":"workbench.action.files.save"},
                {"key":"ctrl+s","command":"workbench.action.files.save"}
            ]"#,
            VscodeHostPlatform::Windows,
        );
        assert_eq!(duplicate.report().imported_entries, 1);
        assert_eq!(duplicate.report().duplicate_entries, 1);
        assert!(duplicate.report().is_importable());
        assert_every_entry_accounted_for(duplicate.report());
        assert!(has_code(
            duplicate.report(),
            VscodeDiagnosticCode::DuplicateBinding
        ));

        let conflict = vscode(
            r#"[
                {"key":"ctrl+s","command":"workbench.action.files.save"},
                {"key":"ctrl+shift+s","command":"workbench.action.files.save"}
            ]"#,
            VscodeHostPlatform::Windows,
        );
        assert_eq!(conflict.report().imported_entries, 0);
        assert_eq!(conflict.report().conflicting_entries, 2);
        assert!(conflict.report().has_errors());
        assert!(has_code(
            conflict.report(),
            VscodeDiagnosticCode::ConflictingBinding
        ));
        assert!(
            conflict
                .artifact()
                .profile()
                .command_override(Command::Save)
                .is_none()
        );
    }

    #[test]
    fn malformed_unknown_and_unsupported_constructs_are_fully_reported() {
        let adaptation = vscode(
            r#"[
                5,
                {"key":"ctrl+q","command":"extension.privateCommand"},
                {"key":"ctrl+s","command":"workbench.action.files.save","when":"editorFocus"},
                {"key":"ctrl+c","command":"editor.action.clipboardCopyAction","args":{}},
                {"key":8,"command":"undo"},
                {"key":"ctrl+z","command":"undo","futureField":true}
            ]"#,
            VscodeHostPlatform::Windows,
        );
        assert_eq!(adaptation.report().total_entries, 6);
        assert_eq!(adaptation.report().unmapped_entries, 1);
        assert_eq!(
            adaptation.report().unmapped_command_ids,
            ["extension.privateCommand"]
        );
        for code in [
            VscodeDiagnosticCode::MalformedEntry,
            VscodeDiagnosticCode::UnsupportedCommand,
            VscodeDiagnosticCode::UnsupportedWhen,
            VscodeDiagnosticCode::UnsupportedArguments,
            VscodeDiagnosticCode::UnsupportedField,
        ] {
            assert!(has_code(adaptation.report(), code), "missing {code:?}");
        }
        assert!(adaptation.report().has_errors());
        assert!(!adaptation.report().is_importable());
        assert_every_entry_accounted_for(adaptation.report());
    }

    #[test]
    fn unmatched_removal_fails_closed_without_creating_an_unbind() {
        let adaptation = vscode(
            r#"[{"key":"ctrl+q","command":"-workbench.action.files.save"}]"#,
            VscodeHostPlatform::Windows,
        );
        assert_eq!(adaptation.report().imported_entries, 0);
        assert_eq!(adaptation.report().rejected_entries, 1);
        assert!(!adaptation.report().is_importable());
        assert!(has_code(
            adaptation.report(),
            VscodeDiagnosticCode::UnsupportedRemoval
        ));
        assert!(
            adaptation
                .artifact()
                .profile()
                .command_override(Command::Save)
                .is_none()
        );
        assert_every_entry_accounted_for(adaptation.report());
    }

    #[test]
    fn remove_then_replace_is_reviewable_but_cannot_be_partially_applied() {
        let adaptation = vscode(
            r#"[
                {"key":"ctrl+s","command":"-workbench.action.files.save"},
                {"key":"ctrl+shift+s","command":"workbench.action.files.save"}
            ]"#,
            VscodeHostPlatform::Windows,
        );
        assert!(adaptation.report().has_errors());
        assert!(!adaptation.report().is_importable());
        assert_eq!(adaptation.report().rejected_entries, 1);
        assert_eq!(adaptation.report().imported_entries, 1);
        let command_override = adaptation
            .artifact()
            .profile()
            .command_override(Command::Save)
            .unwrap();
        assert!(command_override.bindings()[0].sequence().strokes()[0].shift());
        assert_every_entry_accounted_for(adaptation.report());
    }

    #[test]
    fn mixed_known_and_unmapped_entries_have_no_partial_import_authority() {
        let adaptation = vscode(
            r#"[
                {"key":"ctrl+s","command":"workbench.action.files.save"},
                {"key":"ctrl+q","command":"extension.unmapped"}
            ]"#,
            VscodeHostPlatform::Windows,
        );
        assert_eq!(adaptation.report().imported_entries, 1);
        assert_eq!(adaptation.report().unmapped_entries, 1);
        assert!(adaptation.report().has_errors());
        assert!(!adaptation.report().is_importable());
        assert_every_entry_accounted_for(adaptation.report());
    }

    #[test]
    fn mixed_known_and_unknown_field_entries_have_no_partial_import_authority() {
        let adaptation = vscode(
            r#"[
                {"key":"ctrl+s","command":"workbench.action.files.save"},
                {"key":"ctrl+z","command":"undo","futureBehavior":true}
            ]"#,
            VscodeHostPlatform::Windows,
        );
        assert_eq!(adaptation.report().imported_entries, 1);
        assert_eq!(adaptation.report().rejected_entries, 1);
        assert!(adaptation.report().has_errors());
        assert!(!adaptation.report().is_importable());
        assert_every_entry_accounted_for(adaptation.report());
    }

    #[test]
    fn native_detection_retains_exact_source_bytes_and_digest() {
        let source = br#"{
            // Native artifacts may arrive through a JSONC-capable editor.
            "format":"rspice.shortcuts/1",
            "artifact":{
                "schemaVersion":1,
                "scope":"user-overrides",
                "coverage":{"contexts":["all"],"platforms":["desktop"],"policiesIncluded":false},
                "platformMappingsIncluded":false,
            },
            "profile":{"commands":{}},
        }"#;
        let detected =
            detect_shortcut_artifact(r"C:\private\native.json", source, VscodeHostPlatform::Linux)
                .unwrap();
        let DetectedShortcutArtifact::RSpice(native) = detected else {
            panic!("expected native artifact");
        };
        assert_eq!(native.source_bytes(), source);
        assert_eq!(native.source_digest(), sha256(source));
        assert_eq!(native.artifact().source_digest(), sha256(source));
        assert_eq!(native.artifact().source_name(), "native.json");
    }

    #[test]
    fn diagnostics_do_not_echo_source_values_or_paths() {
        let private_values = [
            "/private/secret/project",
            "private-predicate-value",
            "private-key-token",
            "private-field-name",
        ];
        let source = format!(
            r#"[{{
                "key":"ctrl+{}",
                "command":"workbench.action.files.save",
                "when":"{}",
                "args":{{"path":"{}"}},
                "{}":true
            }}]"#,
            private_values[2], private_values[1], private_values[0], private_values[3]
        );
        let adaptation = vscode(&source, VscodeHostPlatform::Windows);
        let messages = adaptation
            .report()
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        for private_value in private_values {
            assert!(!messages.contains(private_value));
        }
    }

    #[test]
    fn unsupported_root_and_unterminated_jsonc_have_stable_codes() {
        let root_error =
            detect_shortcut_artifact("bad.json", br#"{"commands":{}}"#, VscodeHostPlatform::Linux)
                .unwrap_err();
        assert_eq!(root_error.code(), "shortcut-adapter.unknown-object");

        let comment_error =
            detect_shortcut_artifact("bad.json", b"[/* private", VscodeHostPlatform::Linux)
                .unwrap_err();
        assert_eq!(
            comment_error.code(),
            "shortcut-adapter.unterminated-comment"
        );
    }

    #[test]
    fn colliding_known_commands_are_rejected() {
        let adaptation = vscode(
            r#"[
                {"key":"ctrl+z","command":"undo"},
                {"key":"ctrl+z","command":"redo"}
            ]"#,
            VscodeHostPlatform::Windows,
        );
        assert!(has_code(
            adaptation.report(),
            VscodeDiagnosticCode::CrossCommandCollision
        ));
        assert!(!adaptation.report().is_importable());
    }
}
