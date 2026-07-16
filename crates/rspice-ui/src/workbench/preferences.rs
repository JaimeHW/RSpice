//! Durable, validated preferences that have concrete runtime consumers.
//!
//! Mockup settings are added here only after their owning subsystem consumes
//! the value. This prevents a persisted form value from being mistaken for an
//! implemented engineering policy.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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

/// User/device overrides consumed by the workbench at runtime. Missing values
/// read as the reviewed zero/false defaults, keeping legacy sessions valid.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UserPreferences {
    choices: BTreeMap<ChoicePreference, u8>,
    toggles: BTreeMap<TogglePreference, bool>,
}

impl UserPreferences {
    #[must_use]
    pub fn choice(&self, key: ChoicePreference) -> usize {
        if !key.is_runtime_consumed() {
            return 0;
        }
        usize::from(
            self.choices
                .get(&key)
                .copied()
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
            self.choices.remove(&key);
        } else {
            self.choices.insert(key, value);
        }
        Ok(())
    }

    #[must_use]
    pub fn toggle(&self, key: TogglePreference) -> bool {
        self.toggles.get(&key).copied().unwrap_or_default()
    }

    pub fn set_toggle(&mut self, key: TogglePreference, value: bool) {
        if value {
            self.toggles.insert(key, true);
        } else {
            self.toggles.remove(&key);
        }
    }

    pub(crate) fn normalize(&mut self) {
        self.choices
            .retain(|key, value| key.is_runtime_consumed() && *value <= key.max_value());
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
}
