use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{canonical_json_bytes, canonicalize_value, sha256};
use crate::common::shortcut_profile_workflow::{
    MAX_SHORTCUT_PROFILE_BYTES, SHORTCUT_PROFILE_FORMAT, stage_shortcut_profile_json,
};
use crate::workbench::ShortcutPreferences;
use crate::workbench::commands::CommandPlatform;

pub const SHORTCUT_ARTIFACT_FORMAT: &str = SHORTCUT_PROFILE_FORMAT;
pub const SHORTCUT_ARTIFACT_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShortcutArtifactScope {
    UserOverrides,
    CompleteResolved,
    CurrentWorkspace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutArtifactCoverage {
    pub contexts: Vec<String>,
    pub platforms: Vec<CommandPlatform>,
    pub policies_included: bool,
}

impl ShortcutArtifactCoverage {
    pub fn validate(&self, platform_mappings_included: bool) -> Result<(), String> {
        if self.contexts.is_empty() {
            return Err("coverage must declare at least one context".to_owned());
        }
        if self.platforms.is_empty() {
            return Err("coverage must declare at least one platform".to_owned());
        }
        let contexts = self.contexts.iter().collect::<BTreeSet<_>>();
        if contexts.len() != self.contexts.len() {
            return Err("coverage contains a duplicate context".to_owned());
        }
        let platforms = self.platforms.iter().collect::<BTreeSet<_>>();
        if platforms.len() != self.platforms.len() {
            return Err("coverage contains a duplicate platform".to_owned());
        }
        if !platform_mappings_included && self.platforms.len() != 1 {
            return Err(
                "an artifact without platform mappings must cover exactly one platform".to_owned(),
            );
        }
        Ok(())
    }

    #[must_use]
    pub fn covers_context(&self, context: &str) -> bool {
        self.contexts
            .iter()
            .any(|covered| covered == "all" || covered == context)
    }

    #[must_use]
    pub fn covers_platform(&self, platform: CommandPlatform) -> bool {
        self.platforms.contains(&platform)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutArtifactManifest {
    pub schema_version: u16,
    pub scope: ShortcutArtifactScope,
    pub coverage: ShortcutArtifactCoverage,
    pub platform_mappings_included: bool,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub unknown_commands_omitted: usize,
}

const fn is_zero(value: &usize) -> bool {
    *value == 0
}

impl ShortcutArtifactManifest {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != SHORTCUT_ARTIFACT_SCHEMA_VERSION {
            return Err(format!(
                "unsupported shortcut artifact schema {}; expected {}",
                self.schema_version, SHORTCUT_ARTIFACT_SCHEMA_VERSION
            ));
        }
        self.coverage.validate(self.platform_mappings_included)
    }

    fn legacy() -> Self {
        Self {
            schema_version: SHORTCUT_ARTIFACT_SCHEMA_VERSION,
            scope: ShortcutArtifactScope::UserOverrides,
            coverage: ShortcutArtifactCoverage {
                contexts: vec!["all".to_owned()],
                platforms: CommandPlatform::ALL.to_vec(),
                policies_included: true,
            },
            platform_mappings_included: true,
            unknown_commands_omitted: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecodedShortcutArtifact {
    source_name: String,
    source_digest: [u8; 32],
    manifest: ShortcutArtifactManifest,
    profile: ShortcutPreferences,
    portable_profile: Value,
    warnings: Vec<String>,
    unknown_fields: BTreeMap<String, Value>,
}

impl DecodedShortcutArtifact {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[must_use]
    pub const fn source_digest(&self) -> [u8; 32] {
        self.source_digest
    }

    #[must_use]
    pub const fn manifest(&self) -> &ShortcutArtifactManifest {
        &self.manifest
    }

    #[must_use]
    pub const fn profile(&self) -> &ShortcutPreferences {
        &self.profile
    }

    #[must_use]
    pub const fn portable_profile(&self) -> &Value {
        &self.portable_profile
    }

    #[must_use]
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    #[must_use]
    pub const fn unknown_fields(&self) -> &BTreeMap<String, Value> {
        &self.unknown_fields
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutArtifactSchemaError {
    code: &'static str,
    message: String,
}

impl ShortcutArtifactSchemaError {
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

impl fmt::Display for ShortcutArtifactSchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

impl std::error::Error for ShortcutArtifactSchemaError {}

pub fn decode_shortcut_artifact_json(
    source_name: impl Into<String>,
    contents: &str,
) -> Result<DecodedShortcutArtifact, ShortcutArtifactSchemaError> {
    decode_shortcut_artifact_json_with_provenance(source_name, contents, contents.as_bytes())
}

/// Decode canonical artifact contents while retaining the original source
/// bytes as the audit provenance. Adapters use this when comments,
/// normalization, or foreign-source conversion changes the decoded bytes.
pub(crate) fn decode_shortcut_artifact_json_with_provenance(
    source_name: impl Into<String>,
    contents: &str,
    provenance_bytes: &[u8],
) -> Result<DecodedShortcutArtifact, ShortcutArtifactSchemaError> {
    if contents.len() as u64 > MAX_SHORTCUT_PROFILE_BYTES {
        return Err(ShortcutArtifactSchemaError::new(
            "shortcut-artifact.byte-limit",
            format!(
                "shortcut artifact is {} bytes; maximum is {MAX_SHORTCUT_PROFILE_BYTES}",
                contents.len()
            ),
        ));
    }
    if provenance_bytes.len() as u64 > MAX_SHORTCUT_PROFILE_BYTES {
        return Err(ShortcutArtifactSchemaError::new(
            "shortcut-artifact.provenance-byte-limit",
            format!(
                "shortcut artifact provenance is {} bytes; maximum is {MAX_SHORTCUT_PROFILE_BYTES}",
                provenance_bytes.len()
            ),
        ));
    }
    let root: Value = serde_json::from_str(contents).map_err(|error| {
        ShortcutArtifactSchemaError::new(
            "shortcut-artifact.invalid-json",
            format!("invalid shortcut artifact JSON: {error}"),
        )
    })?;
    let object = root.as_object().ok_or_else(|| {
        ShortcutArtifactSchemaError::new(
            "shortcut-artifact.invalid-envelope",
            "shortcut artifact must be a JSON object",
        )
    })?;
    if object.get("format").and_then(Value::as_str) != Some(SHORTCUT_ARTIFACT_FORMAT) {
        return Err(ShortcutArtifactSchemaError::new(
            "shortcut-artifact.unsupported-format",
            format!("format must be '{SHORTCUT_ARTIFACT_FORMAT}'"),
        ));
    }
    let raw_profile = object.get("profile").cloned().ok_or_else(|| {
        ShortcutArtifactSchemaError::new(
            "shortcut-artifact.missing-profile",
            "shortcut artifact has no profile payload",
        )
    })?;

    let (manifest, warnings) = match object.get("artifact") {
        Some(value) => {
            let manifest: ShortcutArtifactManifest = serde_json::from_value(value.clone())
                .map_err(|error| {
                    ShortcutArtifactSchemaError::new(
                        "shortcut-artifact.invalid-manifest",
                        format!("invalid shortcut artifact manifest: {error}"),
                    )
                })?;
            manifest.validate().map_err(|error| {
                ShortcutArtifactSchemaError::new("shortcut-artifact.invalid-coverage", error)
            })?;
            (manifest, Vec::new())
        }
        None => (
            ShortcutArtifactManifest::legacy(),
            vec![
                "Legacy shortcut artifact has no coverage manifest; treating it as full user-override coverage across all known contexts and platforms."
                    .to_owned(),
            ],
        ),
    };

    // Route through the hardened legacy decoder so limits, malformed-record
    // isolation, and acknowledgement stripping have one security boundary.
    let legacy = canonical_json_bytes(serde_json::json!({
        "format": SHORTCUT_ARTIFACT_FORMAT,
        "profile": raw_profile,
    }))
    .map_err(|error| {
        ShortcutArtifactSchemaError::new(
            "shortcut-artifact.serialize-stage",
            format!("could not stage shortcut artifact: {error}"),
        )
    })?;
    let legacy = String::from_utf8(legacy).expect("canonical JSON is UTF-8");
    let source_name = normalized_source_name(source_name.into());
    let staged = stage_shortcut_profile_json(source_name.clone(), &legacy)
        .map_err(|error| ShortcutArtifactSchemaError::new(error.code(), error.to_string()))?;
    let portable_profile = portable_profile_value(staged.candidate()).map_err(|error| {
        ShortcutArtifactSchemaError::new(
            "shortcut-artifact.profile-serialization",
            error.to_string(),
        )
    })?;

    let unknown_fields = object
        .iter()
        .filter(|(key, _)| !matches!(key.as_str(), "format" | "artifact" | "profile"))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();

    Ok(DecodedShortcutArtifact {
        source_name,
        source_digest: sha256(provenance_bytes),
        manifest,
        profile: staged.into_candidate(),
        portable_profile,
        warnings,
        unknown_fields,
    })
}

pub(crate) fn portable_profile_value(
    profile: &ShortcutPreferences,
) -> Result<Value, serde_json::Error> {
    let mut value = serde_json::to_value(profile)?;
    scrub_private_profile_state(&mut value);
    Ok(canonicalize_value(value))
}

pub(crate) fn scrub_private_profile_state(value: &mut Value) {
    const PRIVATE_KEYS: &[&str] = &[
        "protected-override-acknowledgements",
        "recent-commands",
        "macros",
        "credentials",
        "automation-source",
        "source-path",
        "project-identity",
        "private-state",
    ];
    match value {
        Value::Object(values) => {
            values.retain(|key, _| !PRIVATE_KEYS.contains(&key.as_str()));
            for value in values.values_mut() {
                scrub_private_profile_state(value);
            }
        }
        Value::Array(values) => {
            for value in values {
                scrub_private_profile_state(value);
            }
        }
        _ => {}
    }
}

fn normalized_source_name(source_name: String) -> String {
    let name = source_name.trim();
    if name.is_empty() {
        "shortcut profile".to_owned()
    } else {
        name.rsplit(['/', '\\']).next().unwrap_or(name).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::commands::Command;

    #[test]
    fn legacy_artifact_migrates_to_explicit_full_coverage() {
        let decoded = decode_shortcut_artifact_json(
            "legacy.json",
            r#"{"format":"rspice.shortcuts/1","profile":{"commands":{}}}"#,
        )
        .unwrap();
        assert_eq!(
            decoded.manifest().scope,
            ShortcutArtifactScope::UserOverrides
        );
        assert_eq!(decoded.manifest().coverage.contexts, ["all"]);
        assert_eq!(decoded.manifest().coverage.platforms, CommandPlatform::ALL);
        assert_eq!(decoded.warnings().len(), 1);
    }

    #[test]
    fn imported_acknowledgements_and_source_paths_are_stripped() {
        let decoded = decode_shortcut_artifact_json(
            r"C:\private\portable.json",
            r#"{
                "format":"rspice.shortcuts/1",
                "artifact":{
                    "schemaVersion":1,
                    "scope":"user-overrides",
                    "coverage":{"contexts":["all"],"platforms":["desktop"],"policiesIncluded":true},
                    "platformMappingsIncluded":false
                },
                "profile":{
                    "protected-override-acknowledgements":["save-project"],
                    "source-path":"C:/secret/project",
                    "commands":{}
                }
            }"#,
        )
        .unwrap();

        assert_eq!(decoded.source_name(), "portable.json");
        assert!(
            !decoded
                .profile()
                .protected_override_acknowledged(Command::Save)
        );
        assert!(
            decoded
                .portable_profile()
                .get("protected-override-acknowledgements")
                .is_none()
        );
        assert!(decoded.portable_profile().get("source-path").is_none());
    }

    #[test]
    fn mappings_disabled_requires_exactly_one_platform() {
        let error = decode_shortcut_artifact_json(
            "bad.json",
            r#"{
                "format":"rspice.shortcuts/1",
                "artifact":{
                    "schemaVersion":1,
                    "scope":"user-overrides",
                    "coverage":{"contexts":["all"],"platforms":["desktop","browser"],"policiesIncluded":true},
                    "platformMappingsIncluded":false
                },
                "profile":{"commands":{}}
            }"#,
        )
        .unwrap_err();
        assert_eq!(error.code(), "shortcut-artifact.invalid-coverage");
    }

    #[test]
    fn adapter_decode_retains_original_source_provenance() {
        let canonical = r#"{
            "format":"rspice.shortcuts/1",
            "artifact":{
                "schemaVersion":1,
                "scope":"user-overrides",
                "coverage":{"contexts":["all"],"platforms":["desktop"],"policiesIncluded":false},
                "platformMappingsIncluded":false
            },
            "profile":{"commands":{}}
        }"#;
        let original = b"foreign source bytes";
        let decoded = decode_shortcut_artifact_json_with_provenance(
            r"C:\private\keybindings.json",
            canonical,
            original,
        )
        .unwrap();

        assert_eq!(decoded.source_name(), "keybindings.json");
        assert_eq!(decoded.source_digest(), sha256(original));
        assert_ne!(decoded.source_digest(), sha256(canonical.as_bytes()));
    }
}
