//! Merging an imported shortcut artifact into the library, in the cases that
//! bind the product's own commands.

use crate::feature_tests::settings::shortcut_merge_rollback::{
    artifact_from, artifact_value_with_coverage, artifact_with_coverage, set, set_key,
    with_command_extra,
};
use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::ProtectedShortcutPolicy;
use crate::state::shortcuts::ShortcutBindingSlot;
use crate::state::shortcuts::ShortcutPreferences;
use crate::state::shortcuts::ShortcutProfileLibrary;
use crate::state::shortcuts::ShortcutSequence;
use crate::state::shortcuts::ShortcutStroke;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::vocabulary::CommandRegistry;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::shortcuts::artifacts::merge::*;
use egui::Key;
use egui::os::OperatingSystem;
use std::collections::BTreeSet;

fn set_sequence(
    profile: &mut ShortcutPreferences,
    command: Command,
    slot: ShortcutBindingSlot,
    platform: CommandPlatform,
    keys: &[Key],
) {
    profile
        .set_binding(
            &CommandRegistry,
            command.key(),
            slot,
            vec![platform],
            Some(
                ShortcutSequence::new(
                    keys.iter()
                        .copied()
                        .map(|key| ShortcutStroke::new(key, false, false, false))
                        .collect(),
                )
                .unwrap(),
            ),
        )
        .unwrap();
}

#[test]
fn exact_cross_command_collision_honors_every_resolution_policy() {
    let cases = [
        (
            ShortcutConflictPolicy::KeepCurrentAndReport,
            "F6",
            "",
            ShortcutImportDecision::KeptCurrent,
        ),
        (
            ShortcutConflictPolicy::UseImportedBinding,
            "",
            "F6",
            ShortcutImportDecision::ReplacedCurrent,
        ),
        (
            ShortcutConflictPolicy::LeaveBothUnbound,
            "",
            "",
            ShortcutImportDecision::UnboundBoth,
        ),
    ];
    for (conflict_policy, zoom_label, select_label, expected_decision) in cases {
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
            Command::SelectTool,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
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
            conflict.kind() == ShortcutImportConflictKind::ExactCollision
                && conflict.decision() == expected_decision
        }));
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                &CommandRegistry,
                Command::ZoomFit.key(),
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            zoom_label
        );
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                &CommandRegistry,
                Command::SelectTool.key(),
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            select_label
        );
    }
}

#[test]
fn prefix_collision_moves_a_multistroke_sequence_atomically() {
    let mut base = ShortcutProfileLibrary::default();
    set(
        base.active_mut().unwrap(),
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    let mut imported = ShortcutPreferences::default();
    set_sequence(
        &mut imported,
        Command::SelectTool,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        &[Key::F6, Key::F7],
    );
    let plan = plan_shortcut_import(
        &base,
        &artifact_from(&imported),
        &ShortcutImportOptions {
            conflict_policy: ShortcutConflictPolicy::UseImportedBinding,
            ..ShortcutImportOptions::default()
        },
    )
    .unwrap();

    assert!(plan.can_apply());
    assert!(plan.conflicts().iter().any(|conflict| {
        conflict.kind() == ShortcutImportConflictKind::PrefixCollision
            && conflict.decision() == ShortcutImportDecision::ReplacedCurrent
    }));
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
            Command::SelectTool.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows
        ),
        "F6 F7"
    );
}

#[test]
fn partial_context_replace_does_not_leak_uncovered_records_or_extras() {
    let mut base_profile = ShortcutPreferences::default();
    set(
        &mut base_profile,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    set_key(
        &mut base_profile,
        ResultsCommand::ToggleLinkedCursors.key(),
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F7,
    );
    let base_profile = with_command_extra(
        &base_profile,
        ResultsCommand::ToggleLinkedCursors.key(),
        "future-field",
        serde_json::json!({"source":"base"}),
    );
    let mut base_raw = serde_json::to_value(base_profile).unwrap();
    base_raw["commands"]["future-routing-command"] =
        serde_json::json!({"bindings":[], "source":"base"});
    let mut base = ShortcutProfileLibrary::default();
    base.replace_active(serde_json::from_value(base_raw).unwrap())
        .unwrap();

    let mut imported = ShortcutPreferences::default();
    set(
        &mut imported,
        Command::ZoomFit,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F8,
    );
    set_key(
        &mut imported,
        ResultsCommand::ToggleLinkedCursors.key(),
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F9,
    );
    let imported = with_command_extra(
        &imported,
        ResultsCommand::ToggleLinkedCursors.key(),
        "future-field",
        serde_json::json!({"source":"imported"}),
    );
    let mut imported_raw = serde_json::to_value(imported).unwrap();
    imported_raw["commands"]["future-routing-command"] =
        serde_json::json!({"bindings":[], "source":"imported"});
    let plan = plan_shortcut_import(
        &base,
        &artifact_value_with_coverage(
            imported_raw,
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
    let candidate = serde_json::to_value(plan.candidate_library().active()).unwrap();

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
            ResultsCommand::ToggleLinkedCursors.key(),
            CommandPlatform::Desktop,
            OperatingSystem::Windows
        ),
        "F7"
    );
    assert_eq!(
        candidate["commands"][ResultsCommand::ToggleLinkedCursors.key().as_str()]["future-field"],
        serde_json::json!({"source":"base"})
    );
    assert_eq!(
        candidate["commands"]["future-routing-command"]["source"],
        "base"
    );
}

#[test]
fn protected_acknowledgements_are_reconfirmed_for_each_import() {
    let mut imported = ShortcutPreferences::default();
    imported
        .policies_mut()
        .set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
    imported
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Alternate,
            vec![
                CommandPlatform::Browser,
                CommandPlatform::Tablet,
                CommandPlatform::Phone,
            ],
            None,
        )
        .unwrap();
    imported.acknowledge_protected_override(Command::Save.key());
    let artifact = artifact_with_coverage(&imported, &["all"], &CommandPlatform::ALL, true);
    assert!(
        !artifact
            .profile()
            .protected_override_acknowledged(Command::Save.key())
    );
    let options = ShortcutImportOptions {
        conflict_policy: ShortcutConflictPolicy::UseImportedBinding,
        ..ShortcutImportOptions::default()
    };
    let review =
        plan_shortcut_import(&ShortcutProfileLibrary::default(), &artifact, &options).unwrap();

    assert!(!review.can_apply());
    assert_eq!(
        review.required_protected_confirmations(),
        &BTreeSet::from([Command::Save.stable_id().to_owned()])
    );
    let confirmed = plan_shortcut_import(
        &ShortcutProfileLibrary::default(),
        &artifact,
        &ShortcutImportOptions {
            protected_confirmations: BTreeSet::from([Command::Save.stable_id().to_owned()]),
            ..options
        },
    )
    .unwrap();
    assert!(confirmed.can_apply());
    assert!(
        confirmed
            .candidate_library()
            .active()
            .protected_override_acknowledged(Command::Save.key())
    );
}

#[test]
fn apply_persists_before_live_mutation() {
    let mut library = ShortcutProfileLibrary::default();
    let mut imported = ShortcutPreferences::default();
    set(
        &mut imported,
        Command::Save,
        ShortcutBindingSlot::Primary,
        CommandPlatform::Desktop,
        Key::F6,
    );
    let plan = plan_shortcut_import(
        &library,
        &artifact_from(&imported),
        &ShortcutImportOptions {
            conflict_policy: ShortcutConflictPolicy::UseImportedBinding,
            ..ShortcutImportOptions::default()
        },
    )
    .unwrap();
    let before = library.clone();
    assert!(apply_shortcut_import(&mut library, &plan, |_| Err("disk full".to_owned())).is_err());
    assert_eq!(library, before);
    apply_shortcut_import(&mut library, &plan, |_| Ok(())).unwrap();
    assert_ne!(
        shortcut_library_digest(&library).unwrap(),
        shortcut_library_digest(&before).unwrap()
    );
}
