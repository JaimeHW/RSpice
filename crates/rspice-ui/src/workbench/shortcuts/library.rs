use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use unicode_normalization::UnicodeNormalization;

use super::ShortcutPreferences;

const SHORTCUT_PROFILE_LIBRARY_VERSION: u16 = 1;
const MAX_PRESET_NAME_CHARS: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortcutPresetName(String);

impl ShortcutPresetName {
    pub fn new(name: impl Into<String>) -> Result<Self, ShortcutProfileLibraryError> {
        let name = name.into().nfc().collect::<String>();
        validate_preset_name(&name)?;
        Ok(Self(name))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn case_folded(&self) -> String {
        case_fold_name(&self.0)
    }
}

impl Serialize for ShortcutPresetName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ShortcutPresetName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        Self::new(name).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedShortcutPreset {
    name: ShortcutPresetName,
    profile: ShortcutPreferences,
}

impl NamedShortcutPreset {
    #[must_use]
    pub const fn name(&self) -> &ShortcutPresetName {
        &self.name
    }

    #[must_use]
    pub const fn profile(&self) -> &ShortcutPreferences {
        &self.profile
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutProfileLibraryError {
    InvalidName(String),
    DuplicateName(String),
    MissingPreset(String),
    IncompatibleLibrary,
    RevisionExhausted,
}

impl fmt::Display for ShortcutProfileLibraryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(message) => {
                write!(formatter, "invalid shortcut preset name: {message}")
            }
            Self::DuplicateName(name) => {
                write!(formatter, "shortcut preset '{name}' already exists")
            }
            Self::MissingPreset(name) => {
                write!(formatter, "shortcut preset '{name}' does not exist")
            }
            Self::IncompatibleLibrary => write!(
                formatter,
                "shortcut profile library belongs to an incompatible version and must be repaired explicitly"
            ),
            Self::RevisionExhausted => write!(
                formatter,
                "shortcut profile library revision space is exhausted; the library was not modified"
            ),
        }
    }
}

impl std::error::Error for ShortcutProfileLibraryError {}

#[derive(Debug, Clone)]
pub struct ShortcutProfileLibrary {
    active: ShortcutPreferences,
    named_presets: BTreeMap<ShortcutPresetName, ShortcutPreferences>,
    revision: Cell<u64>,
    incompatible_named_presets: BTreeMap<String, Value>,
    unknown_fields: BTreeMap<String, Value>,
    legacy_incompatible_root: Option<Value>,
}

impl Default for ShortcutProfileLibrary {
    fn default() -> Self {
        Self {
            active: ShortcutPreferences::default(),
            named_presets: BTreeMap::new(),
            revision: Cell::new(0),
            incompatible_named_presets: BTreeMap::new(),
            unknown_fields: BTreeMap::new(),
            legacy_incompatible_root: None,
        }
    }
}

impl PartialEq for ShortcutProfileLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.active == other.active
            && self.named_presets == other.named_presets
            && self.revision.get() == other.revision.get()
            && self.incompatible_named_presets == other.incompatible_named_presets
            && self.unknown_fields == other.unknown_fields
            && self.legacy_incompatible_root == other.legacy_incompatible_root
    }
}

impl Eq for ShortcutProfileLibrary {}

#[derive(Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct ShortcutProfileLibraryWire {
    library_version: u16,
    active: ShortcutPreferences,
    named_presets: BTreeMap<String, Value>,
    revision: u64,
    #[serde(flatten)]
    unknown_fields: BTreeMap<String, Value>,
}

impl Default for ShortcutProfileLibraryWire {
    fn default() -> Self {
        Self {
            library_version: SHORTCUT_PROFILE_LIBRARY_VERSION,
            active: ShortcutPreferences::default(),
            named_presets: BTreeMap::new(),
            revision: 0,
            unknown_fields: BTreeMap::new(),
        }
    }
}

impl Serialize for ShortcutProfileLibrary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(raw) = &self.legacy_incompatible_root {
            return raw.serialize(serializer);
        }
        let mut named_presets = self.incompatible_named_presets.clone();
        for (name, profile) in &self.named_presets {
            named_presets.insert(
                name.as_str().to_owned(),
                serde_json::to_value(profile).map_err(serde::ser::Error::custom)?,
            );
        }
        ShortcutProfileLibraryWire {
            library_version: SHORTCUT_PROFILE_LIBRARY_VERSION,
            active: self.active.clone(),
            named_presets,
            revision: self.revision.get(),
            unknown_fields: self.unknown_fields.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ShortcutProfileLibrary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        let is_library = raw
            .as_object()
            .and_then(|object| object.get("library-version"))
            .is_some();
        if !is_library {
            let active = serde_json::from_value(raw.clone()).map_err(serde::de::Error::custom)?;
            return Ok(Self {
                active,
                legacy_incompatible_root: (!raw.is_object()).then_some(raw),
                ..Self::default()
            });
        }
        let wire: ShortcutProfileLibraryWire =
            serde_json::from_value(raw.clone()).map_err(serde::de::Error::custom)?;
        if wire.library_version != SHORTCUT_PROFILE_LIBRARY_VERSION || wire.revision == u64::MAX {
            let active = serde_json::from_value(Value::Number(wire.library_version.into()))
                .map_err(serde::de::Error::custom)?;
            return Ok(Self {
                active,
                legacy_incompatible_root: Some(raw),
                ..Self::default()
            });
        }

        let mut named_presets = BTreeMap::new();
        let mut incompatible_named_presets = BTreeMap::new();
        let mut folded_names = BTreeMap::<String, String>::new();
        for (raw_name, raw_profile) in wire.named_presets {
            let Ok(name) = ShortcutPresetName::new(raw_name.clone()) else {
                incompatible_named_presets.insert(raw_name, raw_profile);
                continue;
            };
            let folded = name.case_folded();
            if folded_names.contains_key(&folded) {
                incompatible_named_presets.insert(raw_name, raw_profile);
                continue;
            }
            match serde_json::from_value(raw_profile.clone()) {
                Ok(profile) => {
                    folded_names.insert(folded, name.as_str().to_owned());
                    named_presets.insert(name, profile);
                }
                Err(_) => {
                    incompatible_named_presets.insert(raw_name, raw_profile);
                }
            }
        }
        Ok(Self {
            active: wire.active,
            named_presets,
            revision: Cell::new(wire.revision),
            incompatible_named_presets,
            unknown_fields: wire.unknown_fields,
            legacy_incompatible_root: None,
        })
    }
}

impl ShortcutProfileLibrary {
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.legacy_incompatible_root.is_none()
    }

    #[must_use]
    pub const fn active(&self) -> &ShortcutPreferences {
        &self.active
    }

    /// Revision advances on mutable borrow, even when the caller performs no
    /// edit; this conservative rule ensures no direct mutation can bypass CAS
    /// invalidation. Incompatible raw libraries always fail closed.
    pub fn active_mut(&mut self) -> Result<&mut ShortcutPreferences, ShortcutProfileLibraryError> {
        self.ensure_compatible()?;
        self.bump_revision()?;
        Ok(&mut self.active)
    }

    #[must_use]
    pub fn revision(&self) -> u64 {
        self.revision.get()
    }

    pub fn edit_active<R>(
        &mut self,
        edit: impl FnOnce(&mut ShortcutPreferences) -> R,
    ) -> Result<R, ShortcutProfileLibraryError> {
        self.ensure_compatible()?;
        let mut candidate = self.active.clone();
        let result = edit(&mut candidate);
        if candidate != self.active {
            self.bump_revision()?;
            self.active = candidate;
        }
        Ok(result)
    }

    pub fn replace_active(
        &mut self,
        profile: ShortcutPreferences,
    ) -> Result<(), ShortcutProfileLibraryError> {
        self.ensure_compatible()?;
        if self.active != profile {
            self.bump_revision()?;
            self.active = profile;
        }
        Ok(())
    }

    pub fn named_presets(&self) -> impl Iterator<Item = NamedShortcutPreset> + '_ {
        self.named_presets
            .iter()
            .map(|(name, profile)| NamedShortcutPreset {
                name: name.clone(),
                profile: profile.clone(),
            })
    }

    #[must_use]
    pub fn named_preset(&self, name: &str) -> Option<NamedShortcutPreset> {
        let folded = case_fold_name(name);
        self.named_presets
            .iter()
            .find(|(candidate, _)| candidate.case_folded() == folded)
            .map(|(name, profile)| NamedShortcutPreset {
                name: name.clone(),
                profile: profile.clone(),
            })
    }

    pub fn insert_named_preset(
        &mut self,
        name: impl Into<String>,
        profile: ShortcutPreferences,
        overwrite: bool,
    ) -> Result<ShortcutPresetName, ShortcutProfileLibraryError> {
        if !self.is_compatible() {
            return Err(ShortcutProfileLibraryError::IncompatibleLibrary);
        }
        let name = ShortcutPresetName::new(name)?;
        let existing = self
            .named_presets
            .keys()
            .find(|candidate| candidate.case_folded() == name.case_folded())
            .cloned();
        if let Some(existing) = &existing
            && !overwrite
        {
            return Err(ShortcutProfileLibraryError::DuplicateName(
                existing.as_str().to_owned(),
            ));
        }
        self.bump_revision()?;
        if let Some(existing) = existing {
            self.named_presets.remove(&existing);
        }
        self.named_presets.insert(name.clone(), profile);
        Ok(name)
    }

    pub fn remove_named_preset(
        &mut self,
        name: &str,
    ) -> Result<NamedShortcutPreset, ShortcutProfileLibraryError> {
        if !self.is_compatible() {
            return Err(ShortcutProfileLibraryError::IncompatibleLibrary);
        }
        let folded = case_fold_name(name);
        let key = self
            .named_presets
            .keys()
            .find(|candidate| candidate.case_folded() == folded)
            .cloned()
            .ok_or_else(|| ShortcutProfileLibraryError::MissingPreset(name.to_owned()))?;
        let profile = self
            .named_presets
            .get(&key)
            .cloned()
            .expect("located preset exists");
        self.bump_revision()?;
        self.named_presets.remove(&key);
        Ok(NamedShortcutPreset { name: key, profile })
    }

    /// Restore predecessor content as a new revision. The caller owns CAS
    /// verification; this operation deliberately never copies old revision.
    pub(crate) fn replace_content_from(
        &mut self,
        predecessor: &Self,
    ) -> Result<(), ShortcutProfileLibraryError> {
        self.ensure_compatible()?;
        self.bump_revision()?;
        self.active = predecessor.active.clone();
        self.named_presets = predecessor.named_presets.clone();
        self.incompatible_named_presets = predecessor.incompatible_named_presets.clone();
        self.unknown_fields = predecessor.unknown_fields.clone();
        self.legacy_incompatible_root = predecessor.legacy_incompatible_root.clone();
        Ok(())
    }

    fn ensure_compatible(&self) -> Result<(), ShortcutProfileLibraryError> {
        if self.is_compatible() {
            Ok(())
        } else {
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        }
    }

    fn bump_revision(&self) -> Result<(), ShortcutProfileLibraryError> {
        let revision = self
            .revision
            .get()
            .checked_add(1)
            .filter(|revision| *revision != u64::MAX)
            .ok_or(ShortcutProfileLibraryError::RevisionExhausted)?;
        self.revision.set(revision);
        Ok(())
    }
}

fn validate_preset_name(name: &str) -> Result<(), ShortcutProfileLibraryError> {
    if name.is_empty() || name.trim() != name {
        return Err(ShortcutProfileLibraryError::InvalidName(
            "name must be non-empty and have no leading or trailing whitespace".to_owned(),
        ));
    }
    if name.chars().count() > MAX_PRESET_NAME_CHARS {
        return Err(ShortcutProfileLibraryError::InvalidName(format!(
            "name exceeds {MAX_PRESET_NAME_CHARS} characters"
        )));
    }
    if name.chars().any(char::is_control) {
        return Err(ShortcutProfileLibraryError::InvalidName(
            "control characters are not allowed".to_owned(),
        ));
    }
    if name.ends_with('.')
        || name
            .chars()
            .any(|character| r#"/\:*?"<>|"#.contains(character))
        || matches!(name, "." | "..")
    {
        return Err(ShortcutProfileLibraryError::InvalidName(
            "path-like names are not allowed".to_owned(),
        ));
    }
    let folded = case_fold_name(name);
    let device_basename = folded.split('.').next().unwrap_or(folded.as_str());
    let numbered_device = (device_basename.starts_with("com")
        || device_basename.starts_with("lpt"))
        && device_basename.get(3..).is_some_and(|suffix| {
            matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
        });
    if matches!(
        device_basename,
        "active" | "current" | "default" | "none" | "rspice" | "con" | "prn" | "aux" | "nul"
    ) || numbered_device
    {
        return Err(ShortcutProfileLibraryError::InvalidName(
            "name is reserved".to_owned(),
        ));
    }
    Ok(())
}

/// Locale-independent Unicode caseless key suitable for identity checks.
/// Uppercasing before lowercasing expands characters such as German sharp-s,
/// making `Straße` and `STRASSE` the same preset identity.
fn case_fold_name(name: &str) -> String {
    name.nfc()
        .flat_map(char::to_uppercase)
        .flat_map(char::to_lowercase)
        .collect::<String>()
        .nfc()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_profile_migrates_to_active_library_and_preserves_unknowns() {
        let library: ShortcutProfileLibrary = serde_json::from_str(
            r#"{"commands":{"future-command":{"bindings":[]}},"future-profile":7}"#,
        )
        .unwrap();
        assert_eq!(library.revision(), 0);
        let encoded = serde_json::to_value(library).unwrap();
        assert_eq!(encoded["library-version"], 1);
        assert_eq!(encoded["active"]["future-profile"], 7);
    }

    #[test]
    fn malformed_legacy_root_round_trips_and_all_normal_mutation_fails_closed() {
        let mut library: ShortcutProfileLibrary = serde_json::from_str("17").unwrap();
        assert_eq!(serde_json::to_value(&library).unwrap(), 17);
        assert!(!library.active().audit().is_valid());
        assert!(matches!(
            library.active_mut(),
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        ));
        assert_eq!(library.revision(), 0);
        assert_eq!(serde_json::to_value(library).unwrap(), 17);
    }

    #[test]
    fn preset_names_are_strict_and_case_fold_duplicates_require_overwrite() {
        for invalid in [
            "",
            " Default",
            "default",
            "a/b",
            "..",
            "name.",
            "COM1",
            "CON.txt",
            "line\nfeed",
        ] {
            assert!(
                ShortcutPresetName::new(invalid).is_err(),
                "accepted {invalid:?}"
            );
        }
        let mut library = ShortcutProfileLibrary::default();
        library
            .insert_named_preset("RF review", ShortcutPreferences::default(), false)
            .unwrap();
        assert!(matches!(
            library.insert_named_preset("rf REVIEW", ShortcutPreferences::default(), false),
            Err(ShortcutProfileLibraryError::DuplicateName(_))
        ));
        library
            .insert_named_preset("rf REVIEW", ShortcutPreferences::default(), true)
            .unwrap();
        assert_eq!(library.named_presets().count(), 1);

        library
            .insert_named_preset("Straße", ShortcutPreferences::default(), false)
            .unwrap();
        assert!(matches!(
            library.insert_named_preset("STRASSE", ShortcutPreferences::default(), false),
            Err(ShortcutProfileLibraryError::DuplicateName(_))
        ));

        library
            .insert_named_preset("Café", ShortcutPreferences::default(), false)
            .unwrap();
        assert!(matches!(
            library.insert_named_preset("Cafe\u{301}", ShortcutPreferences::default(), false),
            Err(ShortcutProfileLibraryError::DuplicateName(_))
        ));
        assert_eq!(
            library.named_preset("Cafe\u{301}").unwrap().name().as_str(),
            "Café"
        );
    }

    #[test]
    fn preset_overwrite_replaces_casefold_identity_without_mutating_active() {
        let mut library = ShortcutProfileLibrary::default();
        let active_before = serde_json::to_vec(library.active()).unwrap();
        library
            .insert_named_preset("RF Review", ShortcutPreferences::default(), false)
            .unwrap();
        let replacement: ShortcutPreferences = serde_json::from_value(serde_json::json!({
            "commands": {},
            "future-profile": {"mode":"portable"}
        }))
        .unwrap();
        assert!(matches!(
            library.insert_named_preset("rf review", replacement.clone(), false),
            Err(ShortcutProfileLibraryError::DuplicateName(_))
        ));
        library
            .insert_named_preset("rf review", replacement.clone(), true)
            .unwrap();

        assert_eq!(library.named_presets().count(), 1);
        assert_eq!(
            library.named_preset("RF REVIEW").unwrap().profile(),
            &replacement
        );
        assert_eq!(serde_json::to_vec(library.active()).unwrap(), active_before);
    }

    #[test]
    fn every_mutable_borrow_invalidates_cas_even_when_spurious() {
        let mut library = ShortcutProfileLibrary::default();
        let _ = library.active_mut().unwrap();
        assert_eq!(library.revision(), 1);
        let _ = library.active_mut().unwrap();
        assert_eq!(library.revision(), 2);
    }

    #[test]
    fn predecessor_content_restores_at_one_new_monotonic_revision() {
        let predecessor = ShortcutProfileLibrary::default();
        let mut current = predecessor.clone();
        current
            .insert_named_preset("Imported", ShortcutPreferences::default(), false)
            .unwrap();
        let applied_revision = current.revision();
        current.replace_content_from(&predecessor).unwrap();
        assert_eq!(current.revision(), applied_revision + 1);
        assert!(current.named_preset("Imported").is_none());
    }

    #[test]
    fn unknown_library_and_incompatible_preset_fields_round_trip() {
        let source = r#"{
            "library-version":1,
            "active":{"commands":{}},
            "named-presets":{"bad/name":{"commands":{}},"Good":{"commands":{}}},
            "revision":9,
            "future-library":{"mode":"portable"}
        }"#;
        let library: ShortcutProfileLibrary = serde_json::from_str(source).unwrap();
        assert_eq!(library.revision(), 9);
        assert!(library.named_preset("Good").is_some());
        let encoded = serde_json::to_value(library).unwrap();
        assert_eq!(
            encoded["named-presets"]["bad/name"]["commands"],
            serde_json::json!({})
        );
        assert_eq!(encoded["future-library"]["mode"], "portable");
    }

    #[test]
    fn future_library_refuses_preset_mutation_without_discarding_raw_state() {
        let source = r#"{
            "library-version":99,
            "active":{"commands":{}},
            "named-presets":{"Future":{"commands":{}}},
            "revision":12,
            "future-library":{"required":true}
        }"#;
        let mut library: ShortcutProfileLibrary = serde_json::from_str(source).unwrap();
        assert!(!library.is_compatible());
        assert!(matches!(
            library.insert_named_preset("Imported", ShortcutPreferences::default(), false),
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        ));
        assert!(matches!(
            library.active_mut(),
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        ));
        assert!(matches!(
            library.edit_active(|profile| profile.reset_all()),
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        ));
        assert!(matches!(
            library.replace_active(ShortcutPreferences::default()),
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        ));
        assert!(matches!(
            library.remove_named_preset("Future"),
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        ));
        assert_eq!(
            serde_json::to_value(library).unwrap(),
            serde_json::from_str::<Value>(source).unwrap()
        );
    }

    #[test]
    fn exhausted_persisted_revision_is_incompatible_and_round_trips_raw() {
        let source = serde_json::json!({
            "library-version": 1,
            "active": {"commands": {}},
            "named-presets": {},
            "revision": u64::MAX,
            "future-library": {"required": true}
        });
        let mut library: ShortcutProfileLibrary = serde_json::from_value(source.clone()).unwrap();

        assert!(!library.is_compatible());
        assert!(matches!(
            library.replace_active(ShortcutPreferences::default()),
            Err(ShortcutProfileLibraryError::IncompatibleLibrary)
        ));
        assert_eq!(serde_json::to_value(library).unwrap(), source);
    }

    #[test]
    fn final_usable_revision_refuses_mutation_without_partial_state_change() {
        let source = serde_json::json!({
            "library-version": 1,
            "active": {"commands": {}},
            "named-presets": {},
            "revision": u64::MAX - 1
        });
        let mut library: ShortcutProfileLibrary = serde_json::from_value(source.clone()).unwrap();
        let before = serde_json::to_value(&library).unwrap();

        assert!(library.is_compatible());
        assert!(matches!(
            library.insert_named_preset("RF Review", ShortcutPreferences::default(), false),
            Err(ShortcutProfileLibraryError::RevisionExhausted)
        ));
        assert_eq!(serde_json::to_value(library).unwrap(), before);
    }
}
