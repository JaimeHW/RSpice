//! Staging a legacy `rspice.shortcuts/1` document for import.
//!
//! Parsing is deliberately separated from application state mutation. An
//! import produces a staged candidate; the caller owns the explicit decision
//! to apply it, and reads the audit off the candidate. Export and the
//! pickers live in [`crate::workbench::shortcuts::artifacts`].

use std::fmt;

use serde_json::Value;

use crate::workbench::ShortcutPreferences;
use crate::workbench::shortcuts::MAX_SHORTCUT_SEQUENCE_STROKES;

pub const SHORTCUT_PROFILE_FORMAT: &str = "rspice.shortcuts/1";
pub const MAX_SHORTCUT_PROFILE_BYTES: u64 = 512 * 1024;
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
    #[cfg(test)]
    fn json_path(&self) -> &str {
        &self.path
    }

    /// The human explanation carried beside the code, likewise reached
    /// through `Display` outside tests.
    #[cfg(test)]
    fn context(&self) -> &str {
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

#[cfg(test)]
mod tests {
    use super::*;

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
        let staged = stage_shortcut_profile_json(VALID_SOURCE).unwrap();
        assert!(staged.candidate().audit().issues().iter().any(|issue| {
            issue
                .command_id()
                .is_some_and(|id| id.as_str() == "future-command")
        }));

        let reparsed = serde_json::json!({ "profile": staged.candidate() });
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
    fn legacy_import_never_trusts_transferred_protected_acknowledgements() {
        let staged = stage_shortcut_profile_json(
            r#"{
                "format":"rspice.shortcuts/1",
                "profile":{
                    "protected-override-acknowledgements":["save-project"],
                    "commands":{}
                }
            }"#,
        )
        .unwrap();

        assert!(!staged.candidate().protected_override_acknowledged(
            crate::workbench::commands::vocabulary::Command::Save
        ));
        // The scrub happens before decoding, so the transferred entry cannot
        // reappear through the candidate's own serialization either.
        let reencoded = serde_json::to_value(staged.candidate()).unwrap();
        assert_eq!(
            reencoded["protected-override-acknowledgements"]
                .as_array()
                .map(Vec::len),
            Some(0)
        );
    }

    #[test]
    fn syntax_envelope_and_sequence_errors_have_stable_paths() {
        let syntax = stage_shortcut_profile_json("{").unwrap_err();
        assert_eq!(syntax.json_path(), "$");
        assert_eq!(syntax.code(), "shortcut-profile.invalid-json");

        let format = stage_shortcut_profile_json(r#"{"format":"rspice.shortcuts/2","profile":{}}"#)
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
        let error = stage_shortcut_profile_json(&source.to_string()).unwrap_err();
        assert_eq!(
            error.json_path(),
            "$.profile.commands[\"save\"].bindings[0].sequence"
        );
        assert!(error.context().contains("maximum is 4"));
    }
}
