//! The keyboard shortcut editor, in the cases that bind the product's own
//! commands.

use crate::feature_tests::settings::preferences_dialogs;
use crate::feature_tests::settings::shortcut_preferences::input;
use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::ProtectedShortcutPolicy;
use crate::state::shortcuts::ShortcutBindingSlot;
use crate::state::shortcuts::ShortcutPreferences;
use crate::state::shortcuts::ShortcutProfileIssueCode;
use crate::state::shortcuts::ShortcutProfileIssueSeverity;
use crate::state::shortcuts::ShortcutSequence;
use crate::state::shortcuts::ShortcutStroke;
use crate::workbench::app::dialogs::preferences::shortcut_preferences::capture::*;
use crate::workbench::app::dialogs::preferences::shortcut_preferences::*;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::commands::vocabulary::CommandRegistry;
use crate::workbench::commands::vocabulary::command_registry;
use egui::Context;
use egui::Key;
use egui::Modifiers;

#[test]
fn capture_platforms_preserve_defaults_and_protected_alternates() {
    let profile = ShortcutPreferences::default();
    assert_eq!(
        binding_platforms(
            &profile,
            Command::Save.key(),
            ShortcutBindingSlot::Alternate
        ),
        vec![
            CommandPlatform::Browser,
            CommandPlatform::Tablet,
            CommandPlatform::Phone
        ]
    );
}

#[test]
fn protected_override_requires_explicit_persisted_draft_acknowledgement() {
    let mut profile = ShortcutPreferences::default();
    profile
        .policies_mut()
        .set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
    profile
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
    assert!(!profile.audit(&CommandRegistry).is_valid());
    assert!(!profile.protected_override_acknowledged(Command::Save.key()));

    profile.acknowledge_protected_override(Command::Save.key());

    assert!(profile.audit(&CommandRegistry).is_valid());
    assert!(profile.protected_override_acknowledged(Command::Save.key()));
    assert!(
        profile
            .audit(&CommandRegistry)
            .issues()
            .iter()
            .any(|issue| {
                issue.code() == ShortcutProfileIssueCode::MissingBrowserAlternate
                    && issue.severity() == ShortcutProfileIssueSeverity::Warning
            })
    );
}

#[test]
fn search_matches_context_and_binding_text() {
    let mut editor = ShortcutEditorState {
        open: true,
        draft: Some(ShortcutPreferences::default()),
        query: "ctrl+alt+l".to_owned(),
        ..ShortcutEditorState::default()
    };
    let os = egui::os::OperatingSystem::Windows;
    assert!(filtered_commands(&editor, os).contains(&Command::GenerateNetlist.key()));
    editor.query = "verification".to_owned();
    assert!(filtered_commands(&editor, os).iter().all(|command| {
        editor_context(*command) == ShortcutEditorContext::Verification
            || command_registry()
                .label(*command)
                .to_lowercase()
                .contains("verification")
    }));
}

#[test]
fn ordered_same_frame_keydowns_produce_one_multi_stroke_sequence() {
    let ctx = Context::default();
    let mut editor = ShortcutEditorState::default();
    editor.open(&ShortcutPreferences::default());
    editor.recording = Some(ShortcutCaptureTarget {
        command: Command::Save.key(),
        slot: ShortcutBindingSlot::Primary,
    });
    let ctrl = Modifiers {
        ctrl: true,
        command: true,
        ..Modifiers::NONE
    };
    let key_event = |key, modifiers| egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    };
    let _ = ctx.run_ui(
        egui::RawInput {
            events: vec![
                key_event(Key::K, ctrl),
                key_event(Key::C, ctrl),
                key_event(Key::Enter, Modifiers::NONE),
            ],
            ..egui::RawInput::default()
        },
        |ctx| process_key_capture(ctx, &mut editor),
    );

    assert!(editor.recording.is_none());
    assert_eq!(
        binding_for_slot(
            editor.draft.as_ref().unwrap(),
            Command::Save.key(),
            ShortcutBindingSlot::Primary,
            egui::os::OperatingSystem::Windows,
        )
        .as_deref(),
        Some("Ctrl+K Ctrl+C")
    );
}

#[test]
fn editor_draft_cancel_never_mutates_live_profile() {
    let live = ShortcutPreferences::default();
    let mut editor = ShortcutEditorState::default();
    editor.open(&live);
    editor
        .draft
        .as_mut()
        .unwrap()
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(ShortcutSequence::single(ShortcutStroke::new(
                Key::F6,
                false,
                false,
                false,
            ))),
        )
        .unwrap();
    assert_ne!(editor.draft.as_ref().unwrap(), &live);
    editor.close_and_discard();
    assert_eq!(live, ShortcutPreferences::default());
    assert!(editor.draft.is_none());
}

#[test]
fn valid_save_atomically_replaces_the_live_profile() {
    let mut state = AppState::default();
    state
        .shortcut_library_parts()
        .enable_volatile_test_shortcut_persistence();
    let live = state.model.preferences.shortcuts().clone();
    state.settings.shortcut_editor.open(&live);
    state
        .settings
        .shortcut_editor
        .draft
        .as_mut()
        .unwrap()
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(ShortcutSequence::single(ShortcutStroke::new(
                Key::F6,
                false,
                false,
                false,
            ))),
        )
        .unwrap();
    state.settings.shortcut_editor.dirty = true;

    commit_editor(&Context::default(), &mut preferences_dialogs(&mut state));

    assert!(!state.settings.shortcut_editor.open);
    assert_eq!(
        binding_for_slot(
            state.model.preferences.shortcuts(),
            Command::Save.key(),
            ShortcutBindingSlot::Primary,
            egui::os::OperatingSystem::Windows,
        )
        .as_deref(),
        Some("F6")
    );
}

#[test]
fn invalid_save_keeps_the_draft_open_and_focuses_the_first_issue() {
    let mut state = AppState::default();
    let live = state.model.preferences.shortcuts().clone();
    state.settings.shortcut_editor.open(&live);
    state
        .settings
        .shortcut_editor
        .draft
        .as_mut()
        .unwrap()
        .set_binding(
            &CommandRegistry,
            Command::Save.key(),
            ShortcutBindingSlot::Primary,
            CommandPlatform::ALL.to_vec(),
            Some(ShortcutSequence::single(ShortcutStroke::new(
                Key::J,
                true,
                false,
                false,
            ))),
        )
        .unwrap();
    state.settings.shortcut_editor.dirty = true;
    state.settings.shortcut_editor.query = "schematic only".to_owned();
    state.settings.shortcut_editor.context = ShortcutEditorContext::Schematic;

    commit_editor(&Context::default(), &mut preferences_dialogs(&mut state));

    assert!(state.settings.shortcut_editor.open);
    assert!(state.settings.shortcut_editor.error_summary.is_some());
    assert!(state.settings.shortcut_editor.focus_error.is_some());
    assert!(state.settings.shortcut_editor.query.is_empty());
    assert_eq!(
        state.settings.shortcut_editor.context,
        ShortcutEditorContext::Global
    );
    assert_eq!(state.model.preferences.shortcuts(), &live);
}

#[test]
fn pending_editor_is_immutable_and_exposes_a_disabled_progress_state() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut state = AppState::default();
    let profile = state.model.preferences.shortcuts().clone();
    state.settings.shortcut_editor.open(&profile);
    state.settings.shortcut_editor.query = "save".to_owned();
    state.settings.shortcut_editor.selected_command = Some(Command::Save.key());
    state.settings.shortcut_editor.recording = Some(ShortcutCaptureTarget {
        command: Command::Save.key(),
        slot: ShortcutBindingSlot::Primary,
    });
    state
        .settings
        .shortcut_editor
        .capture_strokes
        .push(ShortcutStroke::new(Key::K, true, false, false));
    state.settings.shortcut_editor.capture_last_input_at = Some(1.0);
    state.settings.shortcut_editor.dirty = true;
    state.settings.shortcut_editor.persistence_pending = true;
    let before = state.settings.shortcut_editor.clone();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: input(1_100.0, 900.0).screen_rect,
            events: vec![egui::Event::Key {
                key: Key::F6,
                physical_key: Some(Key::F6),
                pressed: true,
                repeat: false,
                modifiers: Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ctx| render_editor(ctx, &mut preferences_dialogs(&mut state)),
    );

    let after = &state.settings.shortcut_editor;
    assert_eq!(after.draft, before.draft);
    assert_eq!(after.query, before.query);
    assert_eq!(after.context, before.context);
    assert_eq!(after.selected_command, before.selected_command);
    assert_eq!(after.recording, before.recording);
    assert_eq!(after.capture_strokes, before.capture_strokes);
    assert_eq!(after.capture_last_input_at, before.capture_last_input_at);
    assert_eq!(after.dirty, before.dirty);
    assert_eq!(after.error_summary, before.error_summary);
    assert_eq!(after.focus_error, before.focus_error);
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Status && node.label() == Some("Saving")
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Button
            && node
                .label()
                .is_some_and(|label| label.starts_with("Record Primary binding for"))
            && node.is_disabled()
    }));
}
