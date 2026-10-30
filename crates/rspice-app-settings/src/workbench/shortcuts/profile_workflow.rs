//! Staging a legacy `rspice.shortcuts/1` document for import.
//!
//! Parsing is deliberately separated from application state mutation. An
//! import produces a staged candidate; the caller owns the explicit decision
//! to apply it, and reads the audit off the candidate. Export and the
//! pickers live in [`crate::workbench::shortcuts::artifacts`].

use std::fmt;

use serde_json::Value;

use crate::io::file_exchange::MAX_SHORTCUT_PROFILE_BYTES;
use crate::state::shortcuts::{MAX_SHORTCUT_SEQUENCE_STROKES, ShortcutPreferences};

pub const SHORTCUT_PROFILE_FORMAT: &str = "rspice.shortcuts/1";
pub const MAX_SHORTCUT_COMMAND_RECORDS: usize = 2_048;
pub const MAX_SHORTCUT_BINDING_RECORDS: usize = 4_096;

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

    /// The JSON pointer the failure was raised at. Production reads it
    /// through `Display`; the contract tests below assert it directly.
    #[cfg(any(test, feature = "test-support"))]
    pub fn json_path(&self) -> &str {
        &self.path
    }

    /// The human explanation carried beside the code, likewise reached
    /// through `Display` outside tests.
    #[cfg(any(test, feature = "test-support"))]
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

/// A parsed import that has not been applied. The caller names its own
/// source and reads the audit off the candidate, so neither is stored here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedShortcutProfile {
    candidate: ShortcutPreferences,
}

impl StagedShortcutProfile {
    #[must_use]
    pub const fn candidate(&self) -> &ShortcutPreferences {
        &self.candidate
    }

    #[must_use]
    pub fn into_candidate(self) -> ShortcutPreferences {
        self.candidate
    }
}

/// Parse an import without mutating live preferences or `AppState`.
pub fn stage_shortcut_profile_json(
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
    // records, so `candidate.audit()` reports anything this build cannot
    // safely execute while the source survives for a newer build.
    let candidate: ShortcutPreferences = serde_json::from_value(profile).map_err(|error| {
        ShortcutProfileWorkflowError::new(
            "shortcut-profile.invalid-profile",
            "$.profile",
            format!("could not decode shortcut profile payload: {error}"),
        )
    })?;
    Ok(StagedShortcutProfile { candidate })
}

/// Protected-override acknowledgement is a device-local security decision,
/// never portable profile data. Keep this scrub at the legacy boundary even
/// after richer artifact workflows are added so dormant callers stay safe.
fn strip_local_shortcut_state(profile: &mut Value) {
    if let Some(profile) = profile.as_object_mut() {
        profile.remove("protected-override-acknowledgements");
    }
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

fn json_path_key(key: &str) -> String {
    serde_json::to_string(key).unwrap_or_else(|_| "\"<invalid-key>\"".to_owned())
}
