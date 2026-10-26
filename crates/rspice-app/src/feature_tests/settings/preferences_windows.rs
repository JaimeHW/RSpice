//! Preferences and the windows it opens over a whole application.

use crate::feature_tests::settings::preferences_dialogs;
use crate::workbench::RouteTransitionSource;
use crate::workbench::SurfaceId;
use crate::workbench::SurfaceRoute;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::preferences::preferences_shell::PreferenceCategory;
use crate::workbench::app::dialogs::preferences::*;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::preferences_file::SETTINGS_FILE;
use crate::workbench::{BrowserHistoryEffect, ChoicePreference, TogglePreference};
use egui::Context;

pub(crate) fn preferences_route() -> SurfaceRoute {
    SurfaceRoute::surface(SurfaceId::Preferences)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn test_app() -> RSpiceApp {
    RSpiceApp::test_instance()
}

#[test]
fn close_returns_to_the_exact_source_route_and_requests_history_traversal() {
    let mut state = AppState::default();
    let source = state.workbench.current_route();
    state
        .workbench
        .navigate(preferences_route(), RouteTransitionSource::User)
        .unwrap();
    assert_eq!(
        state.workbench.take_browser_history_effect(),
        Some(BrowserHistoryEffect::Push(preferences_route()))
    );
    close_preferences(&mut preferences_dialogs(&mut state));
    assert_eq!(state.workbench.current_route(), source);
    assert_eq!(
        state.workbench.take_browser_history_effect(),
        Some(BrowserHistoryEffect::Traverse {
            delta: -1,
            destination: source,
        })
    );
}

#[test]
fn direct_deep_link_close_replaces_with_the_retained_workspace() {
    let mut state = AppState::default();
    let fallback = SurfaceRoute::surface(SurfaceId::Design);
    state
        .workbench
        .navigate(preferences_route(), RouteTransitionSource::BrowserPop)
        .unwrap();
    close_preferences(&mut preferences_dialogs(&mut state));
    assert_eq!(state.workbench.current_route(), fallback);
    assert_eq!(
        state.workbench.take_browser_history_effect(),
        Some(BrowserHistoryEffect::Replace(fallback))
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn menu_and_shortcut_entries_open_the_canonical_preferences_route() {
    use crate::workbench::framework::command_registry::ids;
    use crate::workbench::framework::host::Host;
    let mut menu_app = test_app();
    Host::execute(&mut menu_app, ids::PREFERENCES);
    assert_eq!(
        menu_app.state.workbench.current_route(),
        preferences_route()
    );

    let mut shortcut_app = test_app();
    shortcut_app.execute_command_key(ids::PREFERENCES);
    assert_eq!(
        shortcut_app.state.workbench.current_route(),
        preferences_route()
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn stale_retention_marker_cannot_orphan_preferences() {
    let ctx = Context::default();
    let mut app = test_app();
    app.state.settings.preferences_open = true;
    let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
        preferences_dialogs(&mut app.state).render(ctx);
        app.after_contribution();
    });
    assert!(output.platform_output.accesskit_update.is_none());
    assert!(!app.state.settings.preferences_open);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        SurfaceId::Design
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn route_loss_releases_idle_preferences_child_modals() {
    let ctx = Context::default();
    let mut app = test_app();
    app.state.settings.preferences_open = true;
    app.state.settings.shortcut_portability.open_import();
    app.state
        .settings
        .workspace_layout_manager
        .open(crate::workbench::WorkspacePreset::Engineering);

    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        preferences_dialogs(&mut app.state).render(ctx);
        app.after_contribution();
    });

    assert!(!app.state.settings.preferences_open);
    assert!(
        !app.state
            .settings
            .shortcut_portability
            .application_modal_open()
    );
    assert!(!app.state.settings.workspace_layout_manager.open);
    assert!(!app.state.dialog_modal_open());
}

/// Every category reaches a page with something on it that can be
/// operated.
///
/// This replaces a check that each page painted a level-3 heading, which
/// the three categories that held nothing else also passed: a heading and
/// a sentence saying the page had no settings satisfied it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_category_renders_a_control_the_reader_can_operate() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    for category in PreferenceCategory::ALL {
        let mut state = AppState::default();
        let mut actions = PreferencePageActions::default();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 1200.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    preference_pages::render(
                        ui,
                        category,
                        &mut preferences_dialogs(&mut state),
                        &mut actions,
                    );
                });
            },
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes;
        let controls = nodes
            .iter()
            .filter(|(_, node)| {
                matches!(
                    node.role(),
                    egui::accesskit::Role::ComboBox
                        | egui::accesskit::Role::CheckBox
                        | egui::accesskit::Role::Button
                        | egui::accesskit::Role::Slider
                        | egui::accesskit::Role::SpinButton
                        | egui::accesskit::Role::RadioButton
                )
            })
            .count();
        assert!(
            controls > 0,
            "{} renders no control at all",
            category.label()
        );
    }
}

#[test]
fn complete_preference_model_round_trips_through_the_app_session() {
    let mut state = AppState::default();
    state
        .model
        .preferences
        .set_choice(ChoicePreference::InterfaceScale, 2)
        .unwrap();
    state
        .model
        .preferences
        .set_toggle(TogglePreference::ReducedMotion, true);
    let json = serde_json::to_string(&state).unwrap();
    let restored: AppState = serde_json::from_str(&json).unwrap();
    assert_eq!(
        restored
            .model
            .preferences
            .choice(ChoicePreference::InterfaceScale),
        2
    );
    assert!(
        restored
            .model
            .preferences
            .toggle(TogglePreference::ReducedMotion)
    );
    assert_eq!(restored.model.preferences.interface_scale(), 1.25);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_layout_manager_stays_reachable_without_a_preferences_row() {
    use crate::workbench::framework::command_registry::ids;
    use crate::workbench::framework::host::Host;
    let mut layout_app = test_app();
    Host::execute(&mut layout_app, ids::WORKSPACE_LAYOUTS);
    assert!(layout_app.state.settings.workspace_layout_manager.open);
    assert!(layout_app.state.dialog_modal_open());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn shortcut_editor_action_opens_an_isolated_profile_draft() {
    let mut app = test_app();
    preferences_dialogs(&mut app.state).execute_preference_page_actions(
        &Context::default(),
        PreferencePageActions {
            open_shortcut_editor: true,
            ..PreferencePageActions::default()
        },
    );
    assert!(app.state.settings.shortcut_editor.open);
    assert_eq!(
        app.state.settings.shortcut_editor.draft.as_ref(),
        Some(app.state.model.preferences.shortcuts())
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_shortcut_preferences_action_has_a_retained_executor() {
    let ctx = Context::default();

    let mut import_app = test_app();
    preferences_dialogs(&mut import_app.state).execute_preference_page_actions(
        &ctx,
        PreferencePageActions {
            open_shortcut_import: true,
            ..PreferencePageActions::default()
        },
    );
    assert!(
        import_app
            .state
            .settings
            .shortcut_portability
            .application_modal_open()
    );

    let mut export_app = test_app();
    preferences_dialogs(&mut export_app.state).execute_preference_page_actions(
        &ctx,
        PreferencePageActions {
            open_shortcut_export: true,
            ..PreferencePageActions::default()
        },
    );
    assert!(
        export_app
            .state
            .settings
            .shortcut_portability
            .application_modal_open()
    );
}

/// Export… writes a settings file through the shared picker, and Import…
/// on another installation puts its contents in force.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn settings_file_actions_move_settings_between_installations() {
    use crate::io::file_exchange::{ScriptedChoice, script_next_choice, take_pickers_opened};

    let ctx = Context::default();
    let folder =
        std::env::temp_dir().join(format!("rspice-settings-file-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&folder).expect("temp folder");
    let path = folder.join("rspice-settings.json");
    let _ = take_pickers_opened();

    let mut source = test_app();
    source.state.platform.autosave_minutes = 10;
    script_next_choice(ScriptedChoice::Chose(path.clone()));
    preferences_dialogs(&mut source.state).execute_preference_page_actions(
        &ctx,
        PreferencePageActions {
            export_settings_file: true,
            ..PreferencePageActions::default()
        },
    );
    preferences_dialogs(&mut source.state).poll_settings_file_transfer(&ctx);
    assert_eq!(
        source.state.prompt_text(),
        Some("Settings exported to rspice-settings.json.")
    );

    let mut target = test_app();
    target
        .state
        .shortcut_library_parts()
        .enable_volatile_test_shortcut_persistence();
    script_next_choice(ScriptedChoice::Chose(path));
    preferences_dialogs(&mut target.state).execute_preference_page_actions(
        &ctx,
        PreferencePageActions {
            import_settings_file: true,
            ..PreferencePageActions::default()
        },
    );
    preferences_dialogs(&mut target.state).poll_settings_file_transfer(&ctx);
    assert_eq!(target.state.platform.autosave_minutes, 10);
    assert_eq!(
        target.state.prompt_text(),
        Some("Settings imported from rspice-settings.json.")
    );
    assert_eq!(
        take_pickers_opened(),
        vec![SETTINGS_FILE.label, SETTINGS_FILE.label]
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn shortcut_import_and_export_actions_map_to_their_exact_dialogs() {
    fn rendered_dialog_title(actions: PreferencePageActions) -> String {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = test_app();
        preferences_dialogs(&mut app.state).execute_preference_page_actions(&ctx, actions);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 720.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| preferences_dialogs(&mut app.state).render_shortcut_portability_dialogs(ctx),
        );
        output
            .platform_output
            .accesskit_update
            .expect("dialog AccessKit update")
            .nodes
            .iter()
            .find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::Dialog)
                    .then(|| node.label().map(str::to_owned))
                    .flatten()
            })
            .expect("exact portability dialog title")
    }

    assert_eq!(
        rendered_dialog_title(PreferencePageActions {
            open_shortcut_import: true,
            ..PreferencePageActions::default()
        }),
        shortcut_portability_dialogs::IMPORT_TITLE
    );
    assert_eq!(
        rendered_dialog_title(PreferencePageActions {
            open_shortcut_export: true,
            ..PreferencePageActions::default()
        }),
        shortcut_portability_dialogs::EXPORT_TITLE
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn shortcut_policy_action_publishes_before_the_candidate_becomes_live() {
    let ctx = Context::default();
    let mut app = test_app();
    app.state
        .shortcut_library_parts()
        .enable_volatile_test_shortcut_persistence();
    let before = app.state.model.preferences.shortcuts().clone();
    let mut candidate = before.clone();
    candidate
        .policies_mut()
        .set_chord_timeout(crate::state::shortcuts::ChordTimeoutPolicy::ThreeSeconds);

    preferences_dialogs(&mut app.state).execute_preference_page_actions(
        &ctx,
        PreferencePageActions {
            shortcut_policy_candidate: Some(candidate.clone()),
            ..PreferencePageActions::default()
        },
    );

    assert_ne!(before, candidate);
    assert_eq!(app.state.model.preferences.shortcuts(), &candidate);
    assert_eq!(
        app.state
            .settings
            .shortcut_library_persistence
            .persisted()
            .expect("volatile canonical shortcut snapshot")
            .library()
            .active(),
        &candidate
    );
    assert!(app.state.settings.shortcut_policy_candidate.is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn shortcut_policy_persistence_failure_keeps_live_state_unchanged_and_recovers_controls() {
    let ctx = Context::default();
    let mut app = test_app();
    let before = app.state.model.preferences.shortcuts().clone();
    let mut candidate = before.clone();
    candidate
        .policies_mut()
        .set_chord_timeout(crate::state::shortcuts::ChordTimeoutPolicy::ThreeSeconds);
    app.state.settings.shortcut_library_persistence =
        crate::workbench::app_state::session::shortcut_library::ShortcutLibraryPersistenceRuntime::Unavailable(
            "simulated canonical storage failure".to_owned(),
        );

    preferences_dialogs(&mut app.state).execute_preference_page_actions(
        &ctx,
        PreferencePageActions {
            shortcut_policy_candidate: Some(candidate),
            ..PreferencePageActions::default()
        },
    );

    assert_eq!(app.state.model.preferences.shortcuts(), &before);
    assert!(app.state.settings.shortcut_policy_candidate.is_none());
    assert!(
        app.state
            .settings
            .shortcut_library_publication_continuation
            .is_none()
    );
}

/// Restore defaults asks first, names the page it covers, and changes
/// nothing until the reader answers; Escape answers Cancel.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn restore_defaults_asks_before_it_resets_the_page() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = test_app();
    app.state
        .workbench
        .navigate(preferences_route(), RouteTransitionSource::User)
        .unwrap();
    app.state.settings.preferences_open = true;
    app.state.workbench.theme.mode = crate::ui::Mode::Light;
    set_pending_restore(&ctx, Some(PreferenceCategory::Appearance));

    let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
        preferences_dialogs(&mut app.state).render(ctx);
        app.after_contribution();
    });
    assert_eq!(app.state.workbench.theme.mode, crate::ui::Mode::Light);
    assert!(
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit update")
            .nodes
            .iter()
            .any(|(_, node)| node.role() == egui::accesskit::Role::Dialog
                && node.label() == Some("Restore Appearance settings to their defaults?")),
        "the question names the page it resets"
    );

    let _ = ctx.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ctx| {
            preferences_dialogs(&mut app.state).render(ctx);
            app.after_contribution();
        },
    );
    assert!(pending_restore(&ctx).is_none());
    assert_eq!(app.state.workbench.theme.mode, crate::ui::Mode::Light);
    assert!(
        app.state.settings.preferences_open,
        "Escape answers the question; it does not also close Preferences"
    );
}

#[test]
fn legacy_autosave_values_migrate_to_mockup_supported_intervals() {
    assert_eq!(normalize_autosave_minutes(0), 2);
    assert_eq!(normalize_autosave_minutes(5), 5);
    assert_eq!(normalize_autosave_minutes(15), 10);
}
