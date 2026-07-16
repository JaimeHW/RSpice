//! Durable, validated preferences that have concrete runtime consumers.
//!
//! Mockup settings are added here only after their owning subsystem consumes
//! the value. This prevents a persisted form value from being mistaken for an
//! implemented engineering policy.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::shortcuts::ShortcutPreferences;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChoicePreference {
    /// Retired persisted key from the first Preferences implementation.
    /// Layout presets were not a single source of truth for dock commands,
    /// so restored values are accepted for migration and then discarded.
    #[serde(rename = "workspace-preset")]
    LegacyWorkspacePreset,
    /// Retired persisted key. Console visibility is owned by the workbench
    /// session until a distinct launch-policy owner exists.
    #[serde(rename = "console-on-launch")]
    LegacyConsoleOnLaunch,
    /// Retired persisted key. Schematic grid size is currently document
    /// presentation state, not a globally enforced new-document policy.
    #[serde(rename = "schematic-grid")]
    LegacySchematicGrid,
    InterfaceScale,
    MinimumTouchTarget,
}

impl ChoicePreference {
    const ALL: [Self; 5] = [
        Self::LegacyWorkspacePreset,
        Self::LegacyConsoleOnLaunch,
        Self::LegacySchematicGrid,
        Self::InterfaceScale,
        Self::MinimumTouchTarget,
    ];

    const fn stable_id(self) -> &'static str {
        match self {
            Self::LegacyWorkspacePreset => "workspace-preset",
            Self::LegacyConsoleOnLaunch => "console-on-launch",
            Self::LegacySchematicGrid => "schematic-grid",
            Self::InterfaceScale => "interface-scale",
            Self::MinimumTouchTarget => "minimum-touch-target",
        }
    }

    const fn max_value(self) -> u8 {
        match self {
            Self::LegacyWorkspacePreset | Self::LegacySchematicGrid => 2,
            Self::LegacyConsoleOnLaunch | Self::MinimumTouchTarget => 1,
            Self::InterfaceScale => 3,
        }
    }

    const fn is_runtime_consumed(self) -> bool {
        matches!(self, Self::InterfaceScale | Self::MinimumTouchTarget)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TogglePreference {
    ReducedMotion,
}

impl TogglePreference {
    const ALL: [Self; 1] = [Self::ReducedMotion];

    const fn stable_id(self) -> &'static str {
        match self {
            Self::ReducedMotion => "reduced-motion",
        }
    }
}

/// User/device overrides consumed by the workbench at runtime. Missing values
/// read as the reviewed zero/false defaults, keeping legacy sessions valid.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPreferences {
    /// String-keyed wire storage keeps a newer preference from invalidating
    /// the complete recoverable application session in an older build.
    /// Known keys are validated through the typed accessors below; unknown
    /// keys are retained byte-semantically for a later compatible build.
    choices: BTreeMap<String, Value>,
    toggles: BTreeMap<String, Value>,
    shortcuts: ShortcutPreferences,
    /// Forward-compatible typed domains that this build does not understand.
    #[serde(flatten)]
    unknown_domains: BTreeMap<String, Value>,
}

impl UserPreferences {
    #[must_use]
    pub const fn shortcuts(&self) -> &ShortcutPreferences {
        &self.shortcuts
    }

    pub const fn shortcuts_mut(&mut self) -> &mut ShortcutPreferences {
        &mut self.shortcuts
    }

    #[must_use]
    pub fn choice(&self, key: ChoicePreference) -> usize {
        if !key.is_runtime_consumed() {
            return 0;
        }
        usize::from(
            self.choices
                .get(key.stable_id())
                .and_then(Value::as_u64)
                .and_then(|value| u8::try_from(value).ok())
                .filter(|value| *value <= key.max_value())
                .unwrap_or_default(),
        )
    }

    pub fn set_choice(&mut self, key: ChoicePreference, value: usize) -> Result<(), &'static str> {
        if !key.is_runtime_consumed() {
            return Err("preference no longer has a runtime owner");
        }
        let value = u8::try_from(value).map_err(|_| "choice index is not representable")?;
        if value > key.max_value() {
            return Err("choice index is outside the preference domain");
        }
        if value == 0 {
            self.choices.remove(key.stable_id());
        } else {
            self.choices
                .insert(key.stable_id().to_owned(), Value::from(value));
        }
        Ok(())
    }

    #[must_use]
    pub fn toggle(&self, key: TogglePreference) -> bool {
        self.toggles
            .get(key.stable_id())
            .and_then(Value::as_bool)
            .unwrap_or_default()
    }

    pub fn set_toggle(&mut self, key: TogglePreference, value: bool) {
        if value {
            self.toggles
                .insert(key.stable_id().to_owned(), Value::Bool(true));
        } else {
            self.toggles.remove(key.stable_id());
        }
    }

    pub(crate) fn normalize(&mut self) {
        for key in ChoicePreference::ALL {
            if !key.is_runtime_consumed()
                || self
                    .choices
                    .get(key.stable_id())
                    .and_then(Value::as_u64)
                    .is_none_or(|value| value > u64::from(key.max_value()))
            {
                self.choices.remove(key.stable_id());
            }
        }
        for key in TogglePreference::ALL {
            if self
                .toggles
                .get(key.stable_id())
                .is_some_and(|value| !value.is_boolean())
            {
                self.toggles.remove(key.stable_id());
            }
        }
    }

    #[must_use]
    pub fn interface_scale(&self) -> f32 {
        [1.0, 1.1, 1.25, 1.5][self.choice(ChoicePreference::InterfaceScale)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumed_overrides_round_trip() {
        let mut preferences = UserPreferences::default();
        preferences
            .set_choice(ChoicePreference::InterfaceScale, 2)
            .unwrap();
        preferences.set_toggle(TogglePreference::ReducedMotion, true);
        let json = serde_json::to_string(&preferences).unwrap();
        let restored: UserPreferences = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.choice(ChoicePreference::InterfaceScale), 2);
        assert!(restored.toggle(TogglePreference::ReducedMotion));
        assert_eq!(restored.interface_scale(), 1.25);
    }

    #[test]
    fn setters_reject_out_of_domain_values() {
        let mut preferences = UserPreferences::default();
        assert!(
            preferences
                .set_choice(ChoicePreference::MinimumTouchTarget, 2)
                .is_err()
        );
    }

    #[test]
    fn retired_storage_only_preferences_are_migrated_out() {
        let mut preferences: UserPreferences = serde_json::from_str(
            r#"{"choices":{"workspace-preset":2,"console-on-launch":1,"schematic-grid":1},"toggles":{}}"#,
        )
        .unwrap();

        assert_eq!(
            preferences.choice(ChoicePreference::LegacyWorkspacePreset),
            0
        );
        preferences.normalize();

        assert_eq!(
            preferences.choice(ChoicePreference::LegacyWorkspacePreset),
            0
        );
        assert_eq!(
            preferences.choice(ChoicePreference::LegacyConsoleOnLaunch),
            0
        );
        assert_eq!(preferences.choice(ChoicePreference::LegacySchematicGrid), 0);
        assert!(
            !serde_json::to_string(&preferences)
                .unwrap()
                .contains("workspace-preset")
        );
        assert!(
            preferences
                .set_choice(ChoicePreference::LegacyWorkspacePreset, 1)
                .is_err()
        );
    }

    #[test]
    fn unknown_preference_keys_and_domains_do_not_invalidate_or_disappear() {
        let source = r#"{
            "choices":{"interface-scale":2,"future-density-mode":7},
            "toggles":{"reduced-motion":true,"future-motion-policy":{"mode":"quiet"}},
            "units":{},
            "future-results-policy":{"digits":11,"mode":"exact"}
        }"#;
        let mut restored: UserPreferences = serde_json::from_str(source).unwrap();
        restored.normalize();

        assert_eq!(restored.choice(ChoicePreference::InterfaceScale), 2);
        assert!(restored.toggle(TogglePreference::ReducedMotion));
        let encoded = serde_json::to_value(&restored).unwrap();
        assert_eq!(encoded["choices"]["future-density-mode"], 7);
        assert_eq!(encoded["toggles"]["future-motion-policy"]["mode"], "quiet");
        assert_eq!(encoded["future-results-policy"]["digits"], 11);
    }

    #[test]
    fn malformed_known_values_are_isolated_without_touching_unknown_values() {
        let mut restored: UserPreferences = serde_json::from_str(
            r#"{
                "choices":{"interface-scale":"large","future-choice":9},
                "toggles":{"reduced-motion":17,"future-toggle":false}
            }"#,
        )
        .unwrap();
        restored.normalize();

        assert_eq!(restored.choice(ChoicePreference::InterfaceScale), 0);
        assert!(!restored.toggle(TogglePreference::ReducedMotion));
        let encoded = serde_json::to_value(restored).unwrap();
        assert_eq!(encoded["choices"]["future-choice"], 9);
        assert_eq!(encoded["toggles"]["future-toggle"], false);
        assert!(encoded["choices"].get("interface-scale").is_none());
        assert!(encoded["toggles"].get("reduced-motion").is_none());
    }

    #[test]
    fn shortcut_profile_round_trips_inside_user_preferences() {
        let mut preferences = UserPreferences::default();
        preferences
            .shortcuts_mut()
            .set_binding(
                crate::workbench::commands::Command::Save,
                crate::workbench::shortcuts::ShortcutBindingSlot::Primary,
                crate::workbench::commands::CommandPlatform::ALL.to_vec(),
                Some(crate::workbench::shortcuts::ShortcutSequence::single(
                    crate::workbench::shortcuts::ShortcutStroke::new(
                        egui::Key::F6,
                        false,
                        false,
                        false,
                    ),
                )),
            )
            .unwrap();
        let encoded = serde_json::to_string(&preferences).unwrap();
        let restored: UserPreferences = serde_json::from_str(&encoded).unwrap();
        assert_eq!(
            restored
                .shortcuts()
                .resolved_bindings(crate::workbench::commands::Command::Save)
                .into_iter()
                .find(|binding| {
                    binding.slot() == crate::workbench::shortcuts::ShortcutBindingSlot::Primary
                })
                .unwrap()
                .display_label(),
            "F6"
        );
    }

    #[test]
    fn future_incompatible_shortcut_root_does_not_invalidate_the_session() {
        let mut preferences: UserPreferences = serde_json::from_str(r#"{"shortcuts":17}"#).unwrap();
        assert_eq!(serde_json::to_value(&preferences).unwrap()["shortcuts"], 17);
        assert!(!preferences.shortcuts().audit().is_valid());

        preferences.shortcuts_mut().reset_all();
        assert!(serde_json::to_value(&preferences).unwrap()["shortcuts"].is_object());
    }
}
