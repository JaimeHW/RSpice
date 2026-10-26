//! The shortcut library in use over a whole application.

use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::{ShortcutBindingSlot, ShortcutSequence, ShortcutStroke};
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::shortcut_library::*;
use crate::workbench::app_state::session::shortcuts::{ShortcutEnvironment, ShortcutInputSnapshot};
use crate::workbench::commands::vocabulary::CommandRegistry;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::requests::ResetShortcutResolver;
use crate::workbench::shortcuts::library_persistence::PersistedShortcutProfileLibrary;

#[test]
fn unacknowledged_completion_blocks_reentrant_publication() {
    let mut state = AppState::default();
    state
        .shortcut_library_parts()
        .enable_volatile_test_shortcut_persistence();
    let predecessor = state
        .settings
        .shortcut_library_persistence
        .persisted()
        .expect("volatile predecessor")
        .clone();
    state.settings.shortcut_library_persistence = ShortcutLibraryPersistenceRuntime::Ready {
        persisted: Box::new(predecessor),
        publication_completion: Some(Ok(())),
        volatile_test: true,
    };
    let before = state.model.preferences.shortcut_profiles().clone();
    let mut candidate = before.clone();
    candidate
        .insert_named_preset(
            "Reentrant candidate",
            crate::state::shortcuts::ShortcutPreferences::default(),
            false,
        )
        .unwrap();

    let error = state
        .shortcut_library_parts()
        .publish_shortcut_library_candidate(
            &candidate,
            &egui::Context::default(),
            command_registry(),
        )
        .unwrap_err();

    assert!(error.contains("awaiting acknowledgement"));
    assert_eq!(state.model.preferences.shortcut_profiles(), &before);
    assert!(
        state
            .shortcut_library_parts()
            .shortcut_library_publication_completion_pending()
    );
}

/// Type one stroke with the primary modifier, `at` into the session, and
/// answer the command the resolver dispatches for it.
fn type_stroke(
    state: &mut AppState,
    key: egui::Key,
    at: std::time::Duration,
) -> Option<crate::state::command_catalog::CommandKey> {
    let press = egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers {
            command: true,
            ctrl: true,
            ..egui::Modifiers::NONE
        },
    };
    let profile = state.model.preferences.shortcuts().clone();
    state
        .shortcut_resolver
        .resolve(
            &ShortcutInputSnapshot::from_events_for_test(&[press], false),
            &profile,
            &CommandRegistry,
            CommandPlatform::Desktop,
            ShortcutEnvironment {
                workspace: crate::workbench::state::Workspace::Design,
                active_view: crate::state::ViewType::Schematic,
                canvas_focus: false,
                source_editor_focus: false,
            },
            at,
            |_| true,
        )
        .command
}

/// A session whose library binds Command palette to Ctrl+K, Ctrl+P, kept
/// in a store a test may write, with Ctrl+K already typed: Ctrl+P would
/// complete the chord.
fn with_half_typed_chord() -> AppState {
    let mut state = AppState::default();
    state
        .model
        .preferences
        .shortcuts_mut()
        .unwrap()
        .set_binding(
            &CommandRegistry,
            ids::COMMAND_PALETTE,
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(
                ShortcutSequence::new(vec![
                    ShortcutStroke::new(egui::Key::K, true, false, false),
                    ShortcutStroke::new(egui::Key::P, true, false, false),
                ])
                .unwrap(),
            ),
        )
        .unwrap();
    state
        .shortcut_library_parts()
        .enable_volatile_test_shortcut_persistence();
    assert_eq!(
        type_stroke(&mut state, egui::Key::K, std::time::Duration::ZERO),
        None,
        "the first stroke of the chord waits for the second"
    );
    state
}

/// Ctrl+P, typed after the library changed, and what it would have done
/// had the resolver kept the chord begun under the previous library.
fn complete_the_chord(state: &mut AppState) {
    let later = std::time::Duration::from_millis(100);
    let mut kept = state.clone();
    assert_eq!(
        type_stroke(&mut kept, egui::Key::P, later),
        Some(ids::COMMAND_PALETTE),
        "with the chord kept, the second stroke completes it"
    );
    assert!(
        state
            .workbench
            .outbox
            .peek::<ResetShortcutResolver>()
            .is_some(),
        "putting a library in use asks for the reset"
    );
    state.take_shortcut_resolver_reset();
    assert!(
        state
            .workbench
            .outbox
            .peek::<ResetShortcutResolver>()
            .is_none()
    );
    let mut fresh = state.clone();
    fresh.shortcut_resolver = Default::default();
    let answered = type_stroke(state, egui::Key::P, later);
    assert_ne!(
        answered,
        Some(ids::COMMAND_PALETTE),
        "the chord begun under the previous library does not complete"
    );
    assert_eq!(
        answered,
        type_stroke(&mut fresh, egui::Key::P, later),
        "Ctrl+P is read on its own, as a resolver with nothing typed reads it"
    );
}

/// Whatever puts a library in use resets the resolver before the next
/// shortcut is resolved: a publication the Preferences dialog makes, and
/// the library a launch restores from its store. The library in use keeps
/// its active profile, so only the reset drops the half-typed chord.
#[test]
fn a_library_put_in_use_resets_the_resolver_before_the_next_shortcut() {
    let mut published = with_half_typed_chord();
    let mut candidate = published.model.preferences.shortcut_profiles().clone();
    candidate
        .insert_named_preset(
            "Another preset",
            crate::state::shortcuts::ShortcutPreferences::default(),
            false,
        )
        .unwrap();
    assert!(matches!(
        published
            .shortcut_library_parts()
            .publish_shortcut_library_candidate(
                &candidate,
                &egui::Context::default(),
                command_registry()
            ),
        Ok(ShortcutLibraryPublication::Published)
    ));
    complete_the_chord(&mut published);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut restored = with_half_typed_chord();
        let library = restored.model.preferences.shortcut_profiles().clone();
        restored
            .shortcut_library_parts()
            .install_shortcut_library_startup(NativeOrBrowserStartup::Native(
                crate::workbench::shortcuts::library_persistence::ShortcutProfileLibraryStartup::Ready {
                    persisted: Box::new(PersistedShortcutProfileLibrary::test_snapshot(
                        library, 2,
                    )),
                    initialized_from_session: false,
                },
            ));
        complete_the_chord(&mut restored);
    }
}
