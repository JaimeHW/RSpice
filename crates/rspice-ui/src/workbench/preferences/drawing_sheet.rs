//! Personal drawing-sheet defaults and reusable custom sizes.
//!
//! Project-owned formats live in `DesignManagementCatalog`. This domain is
//! deliberately owned by `UserPreferences`: it seeds new projects and holds
//! personal presets, but a project captures a complete project-owned copy
//! before an authored sheet may reference one.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as _, Sha256};

use crate::product::ContentDigest;
use crate::state::{
    DrawingSheetInheritance, DrawingSheetPreset, DrawingSheetPresetScope, DrawingSheetStandard,
    SchematicPageOrientation, SchematicSheetFormat,
};

use super::UserPreferences;

const STORAGE_KEY: &str = "drawing-sheet";
const MAX_PERSONAL_PRESETS: usize = crate::state::MAX_DRAWING_SHEET_PROJECT_PRESETS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct DrawingSheetPersonalPreferences {
    pub(crate) default_format: SchematicSheetFormat,
    pub(crate) presets: Vec<DrawingSheetPreset>,
}

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct DrawingSheetPersonalPreferencesWire {
    default_format: SchematicSheetFormat,
    presets: Vec<DrawingSheetPreset>,
}

impl Default for DrawingSheetPersonalPreferencesWire {
    fn default() -> Self {
        let value = DrawingSheetPersonalPreferences::default();
        Self {
            default_format: value.default_format,
            presets: value.presets,
        }
    }
}

impl<'de> Deserialize<'de> for DrawingSheetPersonalPreferences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = DrawingSheetPersonalPreferencesWire::deserialize(deserializer)?;
        let mut value = Self {
            default_format: wire.default_format,
            presets: wire
                .presets
                .into_iter()
                .map(DrawingSheetPreset::normalized_for_storage)
                .collect::<Result<Vec<_>, _>>()
                .map_err(serde::de::Error::custom)?,
        };
        value.normalize_default_preset_reference();
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl Default for DrawingSheetPersonalPreferences {
    fn default() -> Self {
        Self {
            default_format: SchematicSheetFormat::from_standard(
                DrawingSheetStandard::IsoA4,
                SchematicPageOrientation::Landscape,
            )
            .try_update(|draft| {
                draft.inheritance = DrawingSheetInheritance::UserDefault;
            })
            .expect("the canonical personal drawing-sheet default is valid"),
            presets: Vec::new(),
        }
    }
}

impl DrawingSheetPersonalPreferences {
    fn available_preset_ids(&self) -> BTreeMap<String, (String, bool)> {
        self.presets
            .iter()
            .map(|preset| {
                (
                    crate::state::drawing_sheet_preset_key(&preset.id),
                    (
                        preset.name.clone(),
                        matches!(
                            &preset.format.authored_size,
                            crate::state::AuthoredDrawingSheetSize::Custom {
                                snapshot: crate::state::CustomDrawingSheetSnapshot {
                                    source_preset_unavailable: true,
                                    ..
                                }
                            }
                        ),
                    ),
                )
            })
            .collect()
    }

    fn normalize_default_preset_reference(&mut self) {
        let available = self.available_preset_ids();
        let crate::state::AuthoredDrawingSheetSize::Custom { snapshot } =
            &mut self.default_format.authored_size
        else {
            return;
        };
        if let Some(id) = snapshot.preset_id.as_ref() {
            if let Some((name, unavailable)) =
                available.get(&crate::state::drawing_sheet_preset_key(id))
            {
                snapshot.name.clone_from(name);
                snapshot.source_preset_unavailable = *unavailable;
            } else {
                snapshot.source_preset_unavailable = true;
            }
        } else {
            snapshot.source_preset_unavailable = false;
        }
    }

    pub(crate) fn semantic_digest(&self) -> ContentDigest {
        #[derive(Serialize)]
        struct DigestMaterial<'a> {
            schema: &'static str,
            value: &'a DrawingSheetPersonalPreferences,
        }
        let bytes = serde_json::to_vec(&DigestMaterial {
            schema: "rspice-personal-drawing-sheet-preferences/v1",
            value: self,
        })
        .expect("personal drawing-sheet preferences are always serializable");
        ContentDigest::from_bytes(Sha256::digest(bytes).into())
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.default_format.inheritance != DrawingSheetInheritance::UserDefault {
            return Err(
                "The personal drawing-sheet default must retain personal-default ownership."
                    .to_owned(),
            );
        }
        self.default_format
            .validate()
            .map_err(|error| error.to_string())?;
        crate::state::validate_format_preset_reference(
            &self.default_format,
            &self.available_preset_ids(),
        )
        .map_err(|error| error.to_string())?;
        if self.default_format != self.default_format.without_project_owned_title_values() {
            return Err(
                "Personal drawing-sheet defaults cannot duplicate project-owned title values."
                    .to_owned(),
            );
        }
        if self.presets.len() > MAX_PERSONAL_PRESETS {
            return Err(format!(
                "Personal drawing-sheet presets exceed the {MAX_PERSONAL_PRESETS} item limit."
            ));
        }
        let mut ids = BTreeSet::new();
        let mut names = BTreeSet::new();
        for preset in &self.presets {
            preset.validate().map_err(|error| error.to_string())?;
            if preset.format != preset.format.without_project_owned_title_values() {
                return Err(
                    "Personal drawing-sheet presets cannot duplicate project-owned title values."
                        .to_owned(),
                );
            }
            if preset.scope != DrawingSheetPresetScope::User
                || preset.format.inheritance != DrawingSheetInheritance::Explicit
            {
                return Err(
                    "A personal drawing-sheet preset must be user-owned and explicit.".to_owned(),
                );
            }
            let id = crate::state::drawing_sheet_preset_key(&preset.id);
            let name = preset.name.trim().to_lowercase();
            if !ids.insert(id) {
                return Err(format!(
                    "Personal drawing-sheet preset id '{}' is duplicated.",
                    preset.id
                ));
            }
            if !names.insert(name) {
                return Err(format!(
                    "Personal drawing-sheet preset name '{}' is duplicated.",
                    preset.name
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn find_preset(&self, id: &str) -> Option<&DrawingSheetPreset> {
        self.presets
            .iter()
            .find(|preset| preset.id.eq_ignore_ascii_case(id))
    }
}

impl UserPreferences {
    /// Typed personal sheet preferences. An incompatible future payload stays
    /// retained in the preference map while this build resolves safe defaults.
    #[must_use]
    pub(crate) fn drawing_sheet_personal_preferences(&self) -> DrawingSheetPersonalPreferences {
        self.unknown_domains
            .get(STORAGE_KEY)
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
            .filter(|settings: &DrawingSheetPersonalPreferences| settings.validate().is_ok())
            .unwrap_or_default()
    }

    /// Atomically replace the personal drawing-sheet preference domain.
    pub(crate) fn set_drawing_sheet_personal_preferences(
        &mut self,
        mut settings: DrawingSheetPersonalPreferences,
    ) -> Result<(), String> {
        settings.presets = settings
            .presets
            .into_iter()
            .map(DrawingSheetPreset::normalized_for_storage)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?;
        settings.normalize_default_preset_reference();
        settings.validate()?;
        if settings == DrawingSheetPersonalPreferences::default() {
            self.unknown_domains.remove(STORAGE_KEY);
            return Ok(());
        }
        let value = serde_json::to_value(settings)
            .map_err(|error| format!("Could not encode drawing-sheet preferences: {error}"))?;
        self.unknown_domains.insert(STORAGE_KEY.to_owned(), value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        DrawingSheetDisplayUnit, DrawingSheetPresetScope, SchematicPageOrientation,
    };

    #[test]
    fn personal_sheet_preferences_round_trip_through_user_preference_authority() {
        let mut preferences = UserPreferences::default();
        let mut settings = preferences.drawing_sheet_personal_preferences();
        settings.default_format = settings
            .default_format
            .try_update(|draft| {
                draft.display_unit = DrawingSheetDisplayUnit::Inches;
            })
            .unwrap();
        settings.presets.push(
            DrawingSheetPreset {
                id: "personal-review-strip".to_owned(),
                name: "Review strip".to_owned(),
                scope: DrawingSheetPresetScope::User,
                format: SchematicSheetFormat::try_custom(
                    "Review strip",
                    210_000,
                    594_000,
                    SchematicPageOrientation::Portrait,
                )
                .unwrap(),
            }
            .normalized_for_storage()
            .unwrap(),
        );

        preferences
            .set_drawing_sheet_personal_preferences(settings.clone())
            .unwrap();
        let encoded = serde_json::to_string(&preferences).unwrap();
        let restored: UserPreferences = serde_json::from_str(&encoded).unwrap();

        assert_eq!(restored.drawing_sheet_personal_preferences(), settings);
    }

    #[test]
    fn personal_authority_rejects_project_owned_presets() {
        let mut settings = DrawingSheetPersonalPreferences::default();
        settings.presets.push(
            DrawingSheetPreset {
                id: "wrong-owner".to_owned(),
                name: "Wrong owner".to_owned(),
                scope: DrawingSheetPresetScope::Project,
                format: SchematicSheetFormat::default(),
            }
            .normalized_for_storage()
            .unwrap(),
        );
        assert!(settings.validate().is_err());
    }
}
