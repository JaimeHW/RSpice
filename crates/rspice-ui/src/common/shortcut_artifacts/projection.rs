use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use egui::os::OperatingSystem;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::canonical_json_bytes;
use super::schema::{
    SHORTCUT_ARTIFACT_FORMAT, SHORTCUT_ARTIFACT_SCHEMA_VERSION, ShortcutArtifactCoverage,
    ShortcutArtifactManifest, portable_profile_value, scrub_private_profile_state,
};
use crate::workbench::ShortcutPreferences;
use crate::workbench::commands::{COMMAND_REGISTRY, Command, CommandPlatform, ShortcutContext};
use crate::workbench::shortcuts::{
    CommandShortcutOverride, ProfileShortcutBinding, ShortcutBindingSlot, ShortcutBindingSource,
    ShortcutSequence,
};

pub use super::schema::ShortcutArtifactScope as ShortcutExportScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutExportRequest {
    pub scope: ShortcutExportScope,
    pub include_platform_mappings: bool,
    pub runtime_platform: CommandPlatform,
    pub operating_system: OperatingSystem,
    pub current_contexts: Vec<ShortcutContext>,
}

impl ShortcutExportRequest {
    #[must_use]
    pub fn user_overrides(
        runtime_platform: CommandPlatform,
        operating_system: OperatingSystem,
    ) -> Self {
        Self {
            scope: ShortcutExportScope::UserOverrides,
            include_platform_mappings: true,
            runtime_platform,
            operating_system,
            current_contexts: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShortcutReferenceStatus {
    Default,
    UserOverride,
    Unbound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutReferenceRow {
    pub context: String,
    pub group: String,
    pub command_id: String,
    pub command_label: String,
    pub slot: Option<ShortcutBindingSlot>,
    pub platform: Option<CommandPlatform>,
    pub sequence: Option<ShortcutSequence>,
    pub display_sequence: String,
    pub status: ShortcutReferenceStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShortcutReferenceModel {
    manifest: ShortcutArtifactManifest,
    profile: Value,
    rows: Vec<ShortcutReferenceRow>,
    policy_summary: Vec<(String, String)>,
}

impl ShortcutReferenceModel {
    #[must_use]
    pub const fn manifest(&self) -> &ShortcutArtifactManifest {
        &self.manifest
    }

    #[must_use]
    pub const fn profile(&self) -> &Value {
        &self.profile
    }

    #[must_use]
    pub fn rows(&self) -> &[ShortcutReferenceRow] {
        &self.rows
    }

    #[must_use]
    pub fn policy_summary(&self) -> &[(String, String)] {
        &self.policy_summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutExportError(String);

impl fmt::Display for ShortcutExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for ShortcutExportError {}

pub fn build_shortcut_reference_model(
    profile: &ShortcutPreferences,
    request: &ShortcutExportRequest,
) -> Result<ShortcutReferenceModel, ShortcutExportError> {
    let audit = profile.audit();
    if !audit.is_valid() {
        return Err(ShortcutExportError(
            "shortcut profile has blocking audit findings and cannot be exported".to_owned(),
        ));
    }

    let platforms = if request.include_platform_mappings {
        CommandPlatform::ALL.to_vec()
    } else {
        vec![request.runtime_platform]
    };
    let (contexts, policies_included) = match request.scope {
        ShortcutExportScope::UserOverrides | ShortcutExportScope::CompleteResolved => (
            vec!["all".to_owned()],
            request.scope == ShortcutExportScope::UserOverrides,
        ),
        ShortcutExportScope::CurrentWorkspace => {
            let mut labels = request
                .current_contexts
                .iter()
                .map(|context| context.label().to_owned())
                .collect::<BTreeSet<_>>();
            labels.insert(ShortcutContext::Global.label().to_owned());
            labels.insert(ShortcutContext::ApplicationChrome.label().to_owned());
            labels.insert(ShortcutContext::RunnableProject.label().to_owned());
            (labels.into_iter().collect(), false)
        }
    };
    let coverage = ShortcutArtifactCoverage {
        contexts,
        platforms: platforms.clone(),
        policies_included,
    };

    let raw_profile = portable_profile_value(profile)
        .map_err(|error| ShortcutExportError(format!("could not project profile: {error}")))?;
    let unknown_commands = raw_profile
        .get("commands")
        .and_then(Value::as_object)
        .map_or(0, |commands| {
            commands
                .keys()
                .filter(|id| Command::from_stable_id(id).is_none())
                .count()
        });

    let (mut portable_profile, mut rows) = match request.scope {
        ShortcutExportScope::UserOverrides => (
            slice_user_profile(raw_profile.clone(), &platforms),
            user_override_rows(profile, &platforms, request.operating_system),
        ),
        ShortcutExportScope::CompleteResolved | ShortcutExportScope::CurrentWorkspace => {
            materialized_projection(profile, request, &coverage, &platforms)?
        }
    };
    scrub_private_profile_state(&mut portable_profile);
    sort_rows(&mut rows);

    let policy_summary = raw_profile
        .get("policies")
        .and_then(Value::as_object)
        .map(|policies| {
            policies
                .iter()
                .map(|(name, value)| {
                    (
                        name.clone(),
                        serde_json::to_string(value).unwrap_or_else(|_| "null".to_owned()),
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    let manifest = ShortcutArtifactManifest {
        schema_version: SHORTCUT_ARTIFACT_SCHEMA_VERSION,
        scope: request.scope,
        coverage,
        platform_mappings_included: request.include_platform_mappings,
        unknown_commands_omitted: if request.scope == ShortcutExportScope::UserOverrides {
            0
        } else {
            unknown_commands
        },
    };
    manifest
        .validate()
        .map_err(|error| ShortcutExportError(format!("invalid export coverage: {error}")))?;

    Ok(ShortcutReferenceModel {
        manifest,
        profile: portable_profile,
        rows,
        policy_summary,
    })
}

pub fn serialize_shortcut_reference_json(
    model: &ShortcutReferenceModel,
) -> Result<String, ShortcutExportError> {
    let value = serde_json::json!({
        "format": SHORTCUT_ARTIFACT_FORMAT,
        "artifact": model.manifest,
        "profile": model.profile,
    });
    let bytes = canonical_json_bytes(value)
        .map_err(|error| ShortcutExportError(format!("could not encode artifact: {error}")))?;
    String::from_utf8(bytes).map_err(|error| {
        ShortcutExportError(format!("JSON encoder produced invalid UTF-8: {error}"))
    })
}

fn slice_user_profile(mut profile: Value, platforms: &[CommandPlatform]) -> Value {
    let platform_names = platforms
        .iter()
        .filter_map(|platform| serde_json::to_value(platform).ok())
        .filter_map(|value| value.as_str().map(str::to_owned))
        .collect::<BTreeSet<_>>();
    if let Some(commands) = profile.get_mut("commands").and_then(Value::as_object_mut) {
        for command in commands.values_mut() {
            let Some(bindings) = command.get_mut("bindings").and_then(Value::as_array_mut) else {
                continue;
            };
            bindings.retain_mut(|binding| {
                let Some(binding_platforms) =
                    binding.get_mut("platforms").and_then(Value::as_array_mut)
                else {
                    return true;
                };
                binding_platforms.retain(|platform| {
                    platform
                        .as_str()
                        .is_some_and(|platform| platform_names.contains(platform))
                });
                !binding_platforms.is_empty()
            });
        }
    }
    profile
}

fn user_override_rows(
    profile: &ShortcutPreferences,
    platforms: &[CommandPlatform],
    operating_system: OperatingSystem,
) -> Vec<ShortcutReferenceRow> {
    let mut rows = Vec::new();
    for command in COMMAND_REGISTRY.iter().copied() {
        let Some(command_override) = profile.command_override(command) else {
            continue;
        };
        if command_override.bindings().is_empty() {
            rows.push(unbound_row(command));
            continue;
        }
        for binding in command_override.bindings() {
            for platform in binding
                .platforms()
                .iter()
                .copied()
                .filter(|platform| platforms.contains(platform))
            {
                rows.push(ShortcutReferenceRow {
                    context: command.shortcut_context().label().to_owned(),
                    group: command.spec().group.to_owned(),
                    command_id: command.stable_id().to_owned(),
                    command_label: command.spec().label.to_owned(),
                    slot: Some(binding.slot()),
                    platform: Some(platform),
                    sequence: Some(binding.sequence().clone()),
                    display_sequence: binding.sequence().display_label_for(operating_system),
                    status: ShortcutReferenceStatus::UserOverride,
                });
            }
        }
    }
    rows
}

fn materialized_projection(
    profile: &ShortcutPreferences,
    request: &ShortcutExportRequest,
    coverage: &ShortcutArtifactCoverage,
    platforms: &[CommandPlatform],
) -> Result<(Value, Vec<ShortcutReferenceRow>), ShortcutExportError> {
    let mut commands = BTreeMap::new();
    let mut rows = Vec::new();
    for command in COMMAND_REGISTRY.iter().copied().filter(|command| {
        request.scope != ShortcutExportScope::CurrentWorkspace
            || coverage.covers_context(command.shortcut_context().label())
    }) {
        let mut bindings = Vec::new();
        for binding in profile.effective_bindings(command) {
            let binding_platforms = binding
                .platforms()
                .iter()
                .copied()
                .filter(|platform| platforms.contains(platform))
                .collect::<Vec<_>>();
            if binding_platforms.is_empty() {
                continue;
            }
            bindings.push(
                ProfileShortcutBinding::new(
                    binding.slot(),
                    binding_platforms.clone(),
                    binding.sequence().clone(),
                )
                .map_err(|error| {
                    ShortcutExportError(format!(
                        "could not materialize {}: {error}",
                        command.stable_id()
                    ))
                })?,
            );
            for platform in binding_platforms {
                rows.push(ShortcutReferenceRow {
                    context: command.shortcut_context().label().to_owned(),
                    group: command.spec().group.to_owned(),
                    command_id: command.stable_id().to_owned(),
                    command_label: command.spec().label.to_owned(),
                    slot: Some(binding.slot()),
                    platform: Some(platform),
                    sequence: Some(binding.sequence().clone()),
                    display_sequence: binding
                        .sequence()
                        .display_label_for(request.operating_system),
                    status: match binding.source() {
                        ShortcutBindingSource::Default => ShortcutReferenceStatus::Default,
                        ShortcutBindingSource::User => ShortcutReferenceStatus::UserOverride,
                    },
                });
            }
        }
        if bindings.is_empty() {
            rows.push(unbound_row(command));
        }
        let command_override = CommandShortcutOverride::new(bindings).map_err(|error| {
            ShortcutExportError(format!(
                "could not materialize {}: {error}",
                command.stable_id()
            ))
        })?;
        commands.insert(
            command.stable_id().to_owned(),
            serde_json::to_value(command_override).map_err(|error| {
                ShortcutExportError(format!("could not encode {}: {error}", command.stable_id()))
            })?,
        );
    }
    let profile = serde_json::json!({
        "policies": {},
        "commands": commands,
    });
    Ok((profile, rows))
}

fn unbound_row(command: Command) -> ShortcutReferenceRow {
    ShortcutReferenceRow {
        context: command.shortcut_context().label().to_owned(),
        group: command.spec().group.to_owned(),
        command_id: command.stable_id().to_owned(),
        command_label: command.spec().label.to_owned(),
        slot: None,
        platform: None,
        sequence: None,
        display_sequence: "—".to_owned(),
        status: ShortcutReferenceStatus::Unbound,
    }
}

fn sort_rows(rows: &mut [ShortcutReferenceRow]) {
    rows.sort_by(|left, right| {
        (
            left.context.as_str(),
            left.group.as_str(),
            left.command_id.as_str(),
            left.platform,
            left.slot,
        )
            .cmp(&(
                right.context.as_str(),
                right.group.as_str(),
                right.command_id.as_str(),
                right.platform,
                right.slot,
            ))
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::commands::Command;
    use crate::workbench::shortcuts::{ShortcutSequence, ShortcutStroke};

    #[test]
    fn user_override_json_is_deterministic_private_and_platform_sliced() {
        let mut profile = ShortcutPreferences::default();
        profile
            .set_binding(
                Command::ToggleNavigator,
                ShortcutBindingSlot::Primary,
                CommandPlatform::ALL.to_vec(),
                Some(ShortcutSequence::single(ShortcutStroke::new(
                    egui::Key::F6,
                    false,
                    false,
                    false,
                ))),
            )
            .unwrap();
        profile.acknowledge_protected_override(Command::Save);
        let request = ShortcutExportRequest {
            scope: ShortcutExportScope::UserOverrides,
            include_platform_mappings: false,
            runtime_platform: CommandPlatform::Browser,
            operating_system: OperatingSystem::Windows,
            current_contexts: Vec::new(),
        };
        let model = build_shortcut_reference_model(&profile, &request).unwrap();
        let first = serialize_shortcut_reference_json(&model).unwrap();
        let second = serialize_shortcut_reference_json(&model).unwrap();
        assert_eq!(first, second);
        assert!(!first.contains("protected-override-acknowledgements"));
        assert_eq!(
            model.manifest().coverage.platforms,
            [CommandPlatform::Browser]
        );
        let value: Value = serde_json::from_str(&first).unwrap();
        let platforms =
            &value["profile"]["commands"]["toggle-navigator"]["bindings"][0]["platforms"];
        assert_eq!(platforms, &serde_json::json!(["browser"]));
    }

    #[test]
    fn resolved_export_materializes_defaults_and_current_workspace_filters_contexts() {
        let profile = ShortcutPreferences::default();
        let complete = build_shortcut_reference_model(
            &profile,
            &ShortcutExportRequest {
                scope: ShortcutExportScope::CompleteResolved,
                include_platform_mappings: true,
                runtime_platform: CommandPlatform::Desktop,
                operating_system: OperatingSystem::Windows,
                current_contexts: Vec::new(),
            },
        )
        .unwrap();
        assert!(complete.profile()["commands"].get("save-project").is_some());

        let current = build_shortcut_reference_model(
            &profile,
            &ShortcutExportRequest {
                scope: ShortcutExportScope::CurrentWorkspace,
                include_platform_mappings: false,
                runtime_platform: CommandPlatform::Desktop,
                operating_system: OperatingSystem::Windows,
                current_contexts: vec![ShortcutContext::ResultsWorkspace],
            },
        )
        .unwrap();
        assert!(
            current.profile()["commands"]
                .get("toggle-linked-cursors")
                .is_some()
        );
        assert!(current.profile()["commands"].get("place-wire").is_none());
        assert!(current.profile()["commands"].get("save-project").is_some());
    }
}
