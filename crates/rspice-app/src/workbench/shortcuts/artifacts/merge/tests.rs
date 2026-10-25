//! Tests for applying and rolling back an artifact merge.
//!
//! Apply persists before it mutates live state, rollback is monotonic and
//! refuses to erase a later edit, and a failed persistence leaves the applied
//! state exactly as it was.

use super::*;
use crate::state::shortcuts::ShortcutStroke;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::shortcuts::artifacts::{
    ShortcutExportRequest, ShortcutExportScope, build_shortcut_reference_model,
    decode_shortcut_artifact_json, serialize_shortcut_reference_json,
};
use egui::Key;
use egui::os::OperatingSystem;

pub(crate) fn artifact_from(profile: &ShortcutPreferences) -> DecodedShortcutArtifact {
    let model = build_shortcut_reference_model(
        profile,
        &ShortcutExportRequest {
            scope: ShortcutExportScope::UserOverrides,
            include_platform_mappings: true,
            runtime_platform: CommandPlatform::Desktop,
            operating_system: OperatingSystem::Windows,
            current_contexts: Vec::new(),
        },
    )
    .unwrap();
    decode_shortcut_artifact_json(
        "import.json",
        &serialize_shortcut_reference_json(&model).unwrap(),
    )
    .unwrap()
}

pub(crate) fn artifact_with_coverage(
    profile: &ShortcutPreferences,
    contexts: &[&str],
    platforms: &[CommandPlatform],
    policies_included: bool,
) -> DecodedShortcutArtifact {
    artifact_value_with_coverage(
        serde_json::to_value(profile).unwrap(),
        contexts,
        platforms,
        policies_included,
    )
}

pub(crate) fn artifact_value_with_coverage(
    profile: Value,
    contexts: &[&str],
    platforms: &[CommandPlatform],
    policies_included: bool,
) -> DecodedShortcutArtifact {
    let envelope = serde_json::json!({
        "format": "rspice.shortcuts/1",
        "artifact": {
            "schemaVersion": 1,
            "scope": "user-overrides",
            "coverage": {
                "contexts": contexts,
                "platforms": platforms,
                "policiesIncluded": policies_included
            },
            "platformMappingsIncluded": true
        },
        "profile": profile
    });
    decode_shortcut_artifact_json("import.json", &serde_json::to_string(&envelope).unwrap())
        .unwrap()
}

pub(crate) fn set(
    profile: &mut ShortcutPreferences,
    command: Command,
    slot: ShortcutBindingSlot,
    platform: CommandPlatform,
    key: Key,
) {
    set_key(profile, command.key(), slot, platform, key);
}

/// Bind the command `command` names, as [`set`] binds a command value.
pub(crate) fn set_key(
    profile: &mut ShortcutPreferences,
    command: CommandKey,
    slot: ShortcutBindingSlot,
    platform: CommandPlatform,
    key: Key,
) {
    profile
        .set_binding(
            &CommandRegistry,
            command,
            slot,
            vec![platform],
            Some(ShortcutSequence::single(ShortcutStroke::new(
                key, false, false, false,
            ))),
        )
        .unwrap();
}

pub(crate) fn with_command_extra(
    profile: &ShortcutPreferences,
    command: CommandKey,
    key: &str,
    value: Value,
) -> ShortcutPreferences {
    let mut raw = serde_json::to_value(profile).unwrap();
    raw["commands"][command.as_str()][key] = value;
    serde_json::from_value(raw).unwrap()
}

fn with_binding_extra(
    profile: &ShortcutPreferences,
    command: Command,
    platform: CommandPlatform,
    key: &str,
    value: Value,
) -> ShortcutPreferences {
    let mut raw = serde_json::to_value(profile).unwrap();
    let platform = serde_json::to_value(platform).unwrap();
    let binding = raw["commands"][command.stable_id()]["bindings"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|binding| {
            binding["slot"] == "primary"
                && binding["platforms"]
                    .as_array()
                    .is_some_and(|platforms| platforms.contains(&platform))
        })
        .unwrap();
    binding[key] = value;
    serde_json::from_value(raw).unwrap()
}

#[test]
fn same_target_conflict_honors_every_resolution_policy() {
    let cases = [
        (
            ShortcutConflictPolicy::KeepCurrentAndReport,
            "F6",
            ShortcutImportDecision::KeptCurrent,
        ),
        (
            ShortcutConflictPolicy::UseImportedBinding,
            "F7",
            ShortcutImportDecision::ReplacedCurrent,
        ),
        (
            ShortcutConflictPolicy::LeaveBothUnbound,
            "",
            ShortcutImportDecision::UnboundBoth,
        ),
    ];
    for (conflict_policy, expected_label, expected_decision) in cases {
        let mut base = ShortcutProfileLibrary::default();
        set(
            base.active_mut().unwrap(),
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        let mut imported = ShortcutPreferences::default();
        set(
            &mut imported,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F7,
        );
        let plan = plan_shortcut_import(
            &base,
            &artifact_from(&imported),
            &ShortcutImportOptions {
                conflict_policy,
                ..ShortcutImportOptions::default()
            },
        )
        .unwrap();
        assert!(plan.can_apply());
        assert!(plan.conflicts().iter().any(|conflict| {
            conflict.kind() == ShortcutImportConflictKind::SameTarget
                && conflict.platform() == Some(CommandPlatform::Desktop)
                && conflict.decision() == expected_decision
        }));
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                &CommandRegistry,
                Command::ZoomFit.key(),
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            expected_label
        );
    }
}

#[test]
fn disjoint_platform_slices_of_one_command_merge_without_conflict() {
    let mut base = ShortcutProfileLibrary::default();
    set(
        base.active_mut().unwrap(),
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    let mut imported = ShortcutPreferences::default();
    set(
        &mut imported,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Browser,
        Key::F7,
    );
    let plan = plan_shortcut_import(
        &base,
        &artifact_with_coverage(
            &imported,
            &[Command::ZoomFit.shortcut_context().label()],
            &[CommandPlatform::Browser],
            false,
        ),
        &ShortcutImportOptions::default(),
    )
    .unwrap();

    assert!(plan.can_apply());
    assert!(
        plan.conflicts()
            .iter()
            .all(|conflict| conflict.kind() != ShortcutImportConflictKind::SameTarget)
    );
    assert_eq!(
        plan.candidate_library().active().resolved_label(
            &CommandRegistry,
            Command::ZoomFit.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows
        ),
        "F6"
    );
    assert_eq!(
        plan.candidate_library().active().resolved_label(
            &CommandRegistry,
            Command::ZoomFit.key(),
            CommandPlatform::Browser,
            OperatingSystem::Windows
        ),
        "F7"
    );
}

#[test]
fn single_platform_replace_preserves_other_platform_slices() {
    let mut base = ShortcutProfileLibrary::default();
    set(
        base.active_mut().unwrap(),
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    set(
        base.active_mut().unwrap(),
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Browser,
        Key::F7,
    );
    let mut imported = ShortcutPreferences::default();
    set(
        &mut imported,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F8,
    );
    let plan = plan_shortcut_import(
        &base,
        &artifact_with_coverage(
            &imported,
            &[Command::ZoomFit.shortcut_context().label()],
            &[CommandPlatform::Desktop],
            false,
        ),
        &ShortcutImportOptions {
            merge_policy: ShortcutMergePolicy::ReplaceCurrentUserBindings,
            ..ShortcutImportOptions::default()
        },
    )
    .unwrap();

    assert_eq!(
        plan.candidate_library().active().resolved_label(
            &CommandRegistry,
            Command::ZoomFit.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows
        ),
        "F8"
    );
    assert_eq!(
        plan.candidate_library().active().resolved_label(
            &CommandRegistry,
            Command::ZoomFit.key(),
            CommandPlatform::Browser,
            OperatingSystem::Windows
        ),
        "F7"
    );
}

#[test]
fn covered_unbind_only_removes_the_covered_platform_slice() {
    let mut base = ShortcutProfileLibrary::default();
    set(
        base.active_mut().unwrap(),
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    set(
        base.active_mut().unwrap(),
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Browser,
        Key::F7,
    );
    let mut imported = ShortcutPreferences::default();
    imported
        .set_binding(
            &CommandRegistry,
            Command::ZoomFit.key(),
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            None,
        )
        .unwrap();
    let plan = plan_shortcut_import(
        &base,
        &artifact_with_coverage(
            &imported,
            &[Command::ZoomFit.shortcut_context().label()],
            &[CommandPlatform::Desktop],
            false,
        ),
        &ShortcutImportOptions {
            merge_policy: ShortcutMergePolicy::ReplaceCurrentUserBindings,
            ..ShortcutImportOptions::default()
        },
    )
    .unwrap();

    assert_eq!(
        plan.candidate_library().active().resolved_label(
            &CommandRegistry,
            Command::ZoomFit.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows
        ),
        ""
    );
    assert_eq!(
        plan.candidate_library().active().resolved_label(
            &CommandRegistry,
            Command::ZoomFit.key(),
            CommandPlatform::Browser,
            OperatingSystem::Windows
        ),
        "F7"
    );
}

#[test]
fn known_command_and_binding_extension_fields_follow_their_platform_slice() {
    let mut base_profile = ShortcutPreferences::default();
    set(
        &mut base_profile,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Browser,
        Key::F7,
    );
    let base_profile = with_binding_extra(
        &base_profile,
        Command::ZoomFit,
        CommandPlatform::Browser,
        "future-binding",
        serde_json::json!({"source":"base"}),
    );
    let mut base = ShortcutProfileLibrary::default();
    base.replace_active(base_profile).unwrap();

    let mut imported = ShortcutPreferences::default();
    set(
        &mut imported,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F8,
    );
    let imported = with_command_extra(
        &with_binding_extra(
            &imported,
            Command::ZoomFit,
            CommandPlatform::Desktop,
            "future-binding",
            serde_json::json!({"source":"imported"}),
        ),
        Command::ZoomFit.key(),
        "future-command",
        serde_json::json!({"mode":"portable"}),
    );
    let plan = plan_shortcut_import(
        &base,
        &artifact_with_coverage(
            &imported,
            &[Command::ZoomFit.shortcut_context().label()],
            &[CommandPlatform::Desktop],
            false,
        ),
        &ShortcutImportOptions::default(),
    )
    .unwrap();
    let candidate = serde_json::to_value(plan.candidate_library().active()).unwrap();
    let bindings = candidate["commands"][Command::ZoomFit.stable_id()]["bindings"]
        .as_array()
        .unwrap();
    let desktop = bindings
        .iter()
        .find(|binding| binding["platforms"] == serde_json::json!(["desktop"]))
        .unwrap();
    let browser = bindings
        .iter()
        .find(|binding| binding["platforms"] == serde_json::json!(["browser"]))
        .unwrap();

    assert_eq!(
        candidate["commands"][Command::ZoomFit.stable_id()]["future-command"],
        serde_json::json!({"mode":"portable"})
    );
    assert_eq!(
        desktop["future-binding"],
        serde_json::json!({"source":"imported"})
    );
    assert_eq!(
        browser["future-binding"],
        serde_json::json!({"source":"base"})
    );
}

#[test]
fn full_replace_resets_omitted_known_and_unknown_command_overrides() {
    let mut profile = ShortcutPreferences::default();
    set(
        &mut profile,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    let mut raw = serde_json::to_value(profile).unwrap();
    raw["commands"]["future-routing-command"] = serde_json::json!({
        "bindings": [],
        "future": 7
    });
    let mut base = ShortcutProfileLibrary::default();
    base.replace_active(serde_json::from_value(raw).unwrap())
        .unwrap();
    let plan = plan_shortcut_import(
        &base,
        &artifact_from(&ShortcutPreferences::default()),
        &ShortcutImportOptions {
            merge_policy: ShortcutMergePolicy::ReplaceCurrentUserBindings,
            ..ShortcutImportOptions::default()
        },
    )
    .unwrap();
    let candidate = serde_json::to_value(plan.candidate_library().active()).unwrap();

    assert_eq!(
        plan.candidate_library().active().resolved_label(
            &CommandRegistry,
            Command::ZoomFit.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows
        ),
        "F"
    );
    assert!(
        candidate["commands"]
            .get("future-routing-command")
            .is_none()
    );
}

#[test]
fn imported_unknown_profile_and_command_records_are_preserved() {
    let mut profile = ShortcutPreferences::default();
    set(
        &mut profile,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    let mut raw = serde_json::to_value(profile).unwrap();
    raw["future-profile"] = serde_json::json!({"mode":"portable"});
    raw["commands"]["future-routing-command"] = serde_json::json!({
        "bindings": [],
        "future": {"version": 7}
    });
    let plan = plan_shortcut_import(
        &ShortcutProfileLibrary::default(),
        &artifact_value_with_coverage(raw, &["all"], &CommandPlatform::ALL, true),
        &ShortcutImportOptions::default(),
    )
    .unwrap();
    let candidate = serde_json::to_value(plan.candidate_library().active()).unwrap();

    assert_eq!(candidate["future-profile"]["mode"], "portable");
    assert_eq!(
        candidate["commands"]["future-routing-command"]["future"]["version"],
        7
    );
}

#[test]
fn unknown_envelope_metadata_is_reported_as_intentionally_omitted() {
    let envelope = serde_json::json!({
        "format": "rspice.shortcuts/1",
        "artifact": {
            "schemaVersion": 1,
            "scope": "user-overrides",
            "coverage": {
                "contexts": ["all"],
                "platforms": CommandPlatform::ALL,
                "policiesIncluded": true
            },
            "platformMappingsIncluded": true
        },
        "profile": ShortcutPreferences::default(),
        "vendor-metadata": {"schema": 7}
    });
    let artifact =
        decode_shortcut_artifact_json("import.json", &serde_json::to_string(&envelope).unwrap())
            .unwrap();
    let plan = plan_shortcut_import(
        &ShortcutProfileLibrary::default(),
        &artifact,
        &ShortcutImportOptions::default(),
    )
    .unwrap();

    assert_eq!(
        plan.summaries()
            .iter()
            .find(|summary| summary.binding_class.is_none())
            .unwrap()
            .omitted,
        1
    );
    assert!(
        serde_json::to_value(plan.candidate_library())
            .unwrap()
            .get("vendor-metadata")
            .is_none()
    );
    let mut library = ShortcutProfileLibrary::default();
    apply_shortcut_import(&mut library, &plan, |_| Ok(())).unwrap();
}

#[test]
fn named_preset_import_leaves_active_profile_byte_equivalent() {
    let mut base = ShortcutProfileLibrary::default();
    base.insert_named_preset("RF Review", ShortcutPreferences::default(), false)
        .unwrap();
    let active_before = serde_json::to_vec(base.active()).unwrap();
    let mut imported = ShortcutPreferences::default();
    set(
        &mut imported,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    let options = ShortcutImportOptions {
        merge_policy: ShortcutMergePolicy::ImportNamedPreset,
        preset_name: Some("rf review".to_owned()),
        ..ShortcutImportOptions::default()
    };
    assert!(matches!(
        plan_shortcut_import(&base, &artifact_from(&imported), &options),
        Err(ShortcutImportPlanError::Preset(
            ShortcutProfileLibraryError::DuplicateName(_)
        ))
    ));
    let plan = plan_shortcut_import(
        &base,
        &artifact_from(&imported),
        &ShortcutImportOptions {
            overwrite_existing_preset: true,
            ..options
        },
    )
    .unwrap();
    assert_eq!(
        serde_json::to_vec(plan.candidate_library().active()).unwrap(),
        active_before
    );
    assert_eq!(
        plan.candidate_library()
            .named_preset("RF REVIEW")
            .unwrap()
            .profile()
            .resolved_label(
                &CommandRegistry,
                Command::ZoomFit.key(),
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
        "F6"
    );
}

#[test]
fn apply_rejects_a_stale_revision_before_persistence() {
    let mut library = ShortcutProfileLibrary::default();
    let plan = plan_shortcut_import(
        &library,
        &artifact_from(&ShortcutPreferences::default()),
        &ShortcutImportOptions::default(),
    )
    .unwrap();
    let _ = library.active_mut().unwrap();
    let mut persisted = false;

    assert_eq!(
        apply_shortcut_import(&mut library, &plan, |_| {
            persisted = true;
            Ok(())
        })
        .unwrap_err(),
        ShortcutImportApplyError::StaleBase
    );
    assert!(!persisted);
}

#[test]
fn future_library_is_rejected_without_discarding_raw_state() {
    let source = serde_json::json!({
        "library-version": 99,
        "active": {"commands": {}},
        "named-presets": {"Future": {"commands": {}}},
        "revision": 12,
        "future-library": {"required": true}
    });
    let library: ShortcutProfileLibrary = serde_json::from_value(source.clone()).unwrap();

    assert_eq!(
        plan_shortcut_import(
            &library,
            &artifact_from(&ShortcutPreferences::default()),
            &ShortcutImportOptions::default()
        )
        .unwrap_err(),
        ShortcutImportPlanError::IncompatibleLibrary
    );
    assert_eq!(serde_json::to_value(library).unwrap(), source);
}
