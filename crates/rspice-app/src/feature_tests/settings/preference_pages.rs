//! The Preferences pages, in the cases that read the product's own commands
//! or another feature's state: restoring Shortcuts saves the shipped profile
//! before it goes live, and the waveform defaults reach the viewer.

use crate::feature_tests::settings::preferences_dialogs;
use crate::workbench::ChoicePreference;
use crate::workbench::RSpiceApp;
use crate::workbench::TogglePreference;
use crate::workbench::app::dialogs::preferences::PreferencePageActions;
use crate::workbench::app::dialogs::preferences::preference_pages::*;
use crate::workbench::app::dialogs::preferences::preferences_shell::PreferenceCategory;

/// Shortcuts restore to the shipped bindings and policies, and only by
/// saving that profile first: the reset hands the page a candidate and
/// leaves the live profile alone until the save has gone through.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn restoring_shortcuts_saves_the_shipped_profile_before_it_goes_live() {
    use crate::state::command_catalog::CommandPlatform;
    use crate::state::shortcuts::{
        ChordTimeoutPolicy, ShortcutBindingSlot, ShortcutPreferences, ShortcutSequence,
        ShortcutStroke,
    };
    use crate::workbench::commands::vocabulary::CommandRegistry;
    use crate::workbench::framework::command_registry::ids;

    let ctx = egui::Context::default();
    let mut app = RSpiceApp::test_instance();
    app.state
        .shortcut_library_parts()
        .enable_volatile_test_shortcut_persistence();
    let mut edited = app.state.model.preferences.shortcuts().clone();
    edited
        .policies_mut()
        .set_chord_timeout(ChordTimeoutPolicy::ThreeSeconds);
    edited
        .set_binding(
            &CommandRegistry,
            ids::SAVE,
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(ShortcutSequence::single(ShortcutStroke::new(
                egui::Key::F6,
                false,
                false,
                false,
            ))),
        )
        .expect("F6 is a valid binding");
    preferences_dialogs(&mut app.state).execute_preference_page_actions(
        &ctx,
        PreferencePageActions {
            shortcut_policy_candidate: Some(edited.clone()),
            ..PreferencePageActions::default()
        },
    );
    assert_eq!(app.state.model.preferences.shortcuts(), &edited);

    let mut actions = PreferencePageActions::default();
    restore_defaults(
        PreferenceCategory::Shortcuts,
        &mut preferences_dialogs(&mut app.state),
        &mut actions,
    );
    assert_eq!(
        app.state.model.preferences.shortcuts(),
        &edited,
        "the reset must not write the live profile directly"
    );
    preferences_dialogs(&mut app.state).execute_preference_page_actions(&ctx, actions);

    let shipped = ShortcutPreferences::default();
    assert_eq!(app.state.model.preferences.shortcuts(), &shipped);
    assert_eq!(
        app.state
            .settings
            .shortcut_library_persistence
            .persisted()
            .expect("volatile shortcut snapshot")
            .library()
            .active(),
        &shipped
    );
}

/// The waveform defaults reach the viewer: a viewer cleared for a new
/// project takes the minor grid the page asks for, and restoring the
/// page starts the open viewer wrapped and without it again.
#[test]
fn waveform_defaults_reach_the_viewer() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .preferences
        .set_choice(ChoicePreference::AcPhase, 1)
        .expect("phase may start unwrapped");
    app.state
        .model
        .preferences
        .set_toggle(TogglePreference::WaveMinorGrid, true);
    app.state
        .replace_project_execution_context("Opened project.");
    assert!(app.state.results.results.session.show_minor_grid);

    app.state.results.results.session.phase_continuous = true;
    restore_defaults(
        PreferenceCategory::Results,
        &mut preferences_dialogs(&mut app.state),
        &mut PreferencePageActions::default(),
    );
    app.after_contribution();
    assert!(!app.state.results.results.session.phase_continuous);
    assert!(!app.state.results.results.session.show_minor_grid);
}
