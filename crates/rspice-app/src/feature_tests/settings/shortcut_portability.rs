//! Shortcut import, in the cases that bind the product's own commands.

use crate::feature_tests::settings::shortcut_portability_dialogs::{
    detected, exported_artifact, open_with_artifact,
};
use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::ProtectedShortcutPolicy;
use crate::state::shortcuts::ShortcutBindingSlot;
use crate::state::shortcuts::ShortcutPreferences;
use crate::state::shortcuts::ShortcutProfileLibrary;
use crate::workbench::app::dialogs::preferences::shortcut_portability_dialogs::*;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::shortcuts::artifacts::ShortcutConflictPolicy;

#[test]
fn protected_binding_requires_fresh_explicit_confirmation() {
    let mut imported = ShortcutPreferences::default();
    imported
        .policies_mut()
        .set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
    imported
        .set_binding(
            &crate::workbench::commands::vocabulary::CommandRegistry,
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
    let artifact = exported_artifact(&imported);
    let base = ShortcutProfileLibrary::default();
    let mut state = ShortcutImportDialogState::default();
    open_with_artifact(&mut state, &base, artifact);
    state.options.conflict_policy = ShortcutConflictPolicy::UseImportedBinding;
    state.plan_dirty = true;
    state.synchronize_plan(&base, command_registry());
    let command_id = Command::Save.stable_id().to_owned();
    assert!(
        state
            .plan
            .as_ref()
            .unwrap()
            .required_protected_confirmations()
            .contains(&command_id)
    );
    assert!(!state.plan.as_ref().unwrap().can_apply());

    state.options.protected_confirmations.insert(command_id);
    state.plan_dirty = true;
    state.synchronize_plan(&base, command_registry());
    assert!(state.plan.as_ref().unwrap().can_apply());
}

#[test]
fn replacing_source_clears_digest_specific_protected_consent() {
    let base = ShortcutProfileLibrary::default();
    let mut state = ShortcutImportDialogState::default();
    state.open();
    state
        .options
        .protected_confirmations
        .insert(Command::Save.stable_id().to_owned());

    state.accept_detected(
        &base,
        detected(&ShortcutPreferences::default()),
        command_registry(),
    );

    assert!(state.options.protected_confirmations.is_empty());
}
