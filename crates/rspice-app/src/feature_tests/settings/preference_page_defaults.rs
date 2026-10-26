//! The Preferences pages' shipped values, defaults and search over a whole application.

use crate::feature_tests::settings::preferences_dialogs;
use crate::workbench::ChoicePreference;
use crate::workbench::ConsoleLaunchBehavior;
use crate::workbench::FolderListPreference;
use crate::workbench::RSpiceApp;
use crate::workbench::ScalarPreference;
use crate::workbench::TogglePreference;
use crate::workbench::app::dialogs::preferences::PreferencePageActions;
use crate::workbench::app::dialogs::preferences::preference_pages::*;
use crate::workbench::app::dialogs::preferences::preferences_shell::PreferenceCategory;
use crate::workbench::app_state::AppState;

/// The page source, with each file's test module cut off its end.
///
/// The pages kept in their own modules come first, so that they fall on
/// the pages' side of the split [`pages_and_restore`] makes and every
/// scan below reads them as it reads the pages written here.
fn production_source() -> String {
    [
        include_str!("../../workbench/app/dialogs/preferences/preference_pages/libraries.rs"),
        include_str!("../../workbench/app/dialogs/preferences/preference_pages/navigation.rs"),
        include_str!("../../workbench/app/dialogs/preferences/preference_pages/simulation.rs"),
        include_str!("../../workbench/app/dialogs/preferences/preference_pages/startup_files.rs"),
        include_str!(
            "../../workbench/app/dialogs/preferences/preference_pages/waveform_defaults.rs"
        ),
        include_str!("../../workbench/app/dialogs/preferences/preference_pages.rs"),
    ]
    .map(|source| {
        source
            .split("#[cfg(test)]")
            .next()
            .expect("production source precedes tests")
    })
    .concat()
}

/// The same source with whole-line comments dropped.
///
/// The scans below look for things the pages must not *do*, and several
/// of this file's comments name a deleted row in order to say why it is
/// gone. Reading the prose as if it were code makes an explanation of a
/// fix look exactly like the defect it explains.
fn production_code() -> String {
    production_source()
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The pages, and the reset that has to cover them, as two source halves.
fn pages_and_restore() -> (String, String) {
    let code = production_code();
    let (pages, restore) = code
        .split_once("pub(crate) fn restore_defaults(")
        .expect("restore_defaults follows the pages");
    (pages.to_owned(), restore.to_owned())
}

/// Every preference key named in `source`.
fn named_keys(source: &str) -> Vec<String> {
    let mut keys = Vec::new();
    for enum_name in [
        "ChoicePreference::",
        "TogglePreference::",
        "ScalarPreference::",
        "FolderListPreference::",
    ] {
        for fragment in source.split(enum_name).skip(1) {
            let variant: String = fragment
                .chars()
                .take_while(char::is_ascii_alphanumeric)
                .collect();
            if !variant.is_empty() {
                keys.push(format!("{enum_name}{variant}"));
            }
        }
    }
    keys
}

/// The first option on the Appearance scale row is the scale RSpice ships.
#[test]
fn the_shipped_interface_scale_is_one() {
    assert!(
        (crate::workbench::UserPreferences::default().interface_scale() - 1.0).abs() < f32::EPSILON
    );
}

/// The shipped worker ceiling leaves the engine's own in place.
#[test]
fn the_shipped_worker_ceiling_is_the_engines_own() {
    assert_eq!(
        crate::workbench::UserPreferences::default().solver_worker_limit(),
        None
    );
}

/// Restore defaults is only as good as its coverage: a row added to a page
/// and not to [`restore_defaults`] is a setting the footer's control
/// silently skips, with nothing on screen to say which.
#[test]
fn restore_defaults_covers_every_preference_the_pages_expose() {
    let (pages, restore) = pages_and_restore();
    let restored = named_keys(&restore);
    for key in named_keys(&pages) {
        assert!(
            restored.contains(&key),
            "{key} is on a page but Restore defaults does not reset it"
        );
    }
}

/// And the reset has to reach the stored value in every domain the store
/// keeps a setting in: the plain key maps, the typed Units and Results
/// domains, the workspace domain, and the two settings that live on the
/// theme and the session rather than in preferences at all.
#[test]
fn restoring_a_page_clears_every_override_behind_it() {
    let shipped = serde_json::to_value(crate::workbench::UserPreferences::default())
        .expect("default preferences serialize");
    for category in PreferenceCategory::ALL {
        let mut app = RSpiceApp::test_instance();
        if !override_every_setting_on(category, &mut app.state) {
            continue;
        }
        assert_ne!(
            serde_json::to_value(&app.state.model.preferences).expect("preferences serialize"),
            shipped,
            "{} set no stored override to restore",
            category.label()
        );
        restore_defaults(
            category,
            &mut preferences_dialogs(&mut app.state),
            &mut PreferencePageActions::default(),
        );
        assert_eq!(
            serde_json::to_value(&app.state.model.preferences).expect("preferences serialize"),
            shipped,
            "{} left a stored override behind",
            category.label()
        );
        assert!(
            !app.state.settings.browser_spoken_feedback,
            "{} left the browser speaking control changes",
            category.label()
        );
        assert_eq!(
            app.state.workbench.theme,
            crate::ui::Theme::default(),
            "{} left the theme changed",
            category.label()
        );
        assert_eq!(
            app.state.platform.autosave_minutes,
            crate::services::session::DEFAULT_AUTOSAVE_MINUTES,
            "{} left the autosave interval changed",
            category.label()
        );
    }
}

/// Every label and text the real pages publish while `query` is typed
/// into the search field.
#[cfg(not(target_arch = "wasm32"))]
fn labels_found_by(query: &str) -> Vec<String> {
    use crate::workbench::app::dialogs::preferences::preferences_shell::{
        set_filter_for_review, show,
    };

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    set_filter_for_review(&ctx, query);
    let mut app = RSpiceApp::test_instance();
    let mut category = PreferenceCategory::Appearance;
    let mut update = None;
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_060.0, 760.0),
                )),
                ..Default::default()
            },
            |ctx| {
                let state = &mut app.state;
                let mut actions = PreferencePageActions::default();
                let _ = show(ctx, &mut category, true, 44.0, |ui, category| {
                    render(ui, category, &mut preferences_dialogs(state), &mut actions);
                });
            },
        );
        update = output.platform_output.accesskit_update;
    }
    update
        .expect("AccessKit update")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| {
            node.label()
                .or_else(|| node.value())
                .map(std::string::ToString::to_string)
        })
        .collect()
}

/// A search finds a setting by the word people type for it, which is
/// often neither its label nor anything in its help.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_search_finds_settings_by_what_people_type() {
    for (query, row) in [
        ("dark", "Color mode"),
        ("theme", "Color mode"),
        ("zoom", "Interface scale"),
        ("font", "Interface scale"),
        ("threads", "Solver threads"),
        ("cpu", "Solver threads"),
        ("hotkeys", "Key bindings"),
        ("trackpad", "Mouse wheel"),
        ("retention", "Runs kept"),
        ("launch", "On startup"),
        ("mru", "Recent projects listed"),
        ("backup", "Keep a backup when saving"),
        ("include", "Library folders"),
        ("models", "Library folders"),
    ] {
        assert!(
            labels_found_by(query).iter().any(|label| label == row),
            "searching {query:?} did not find {row}"
        );
    }
}

/// Move every setting shown on `category` off its default. Returns `false`
/// for a page that stores nothing in `UserPreferences`.
fn override_every_setting_on(category: PreferenceCategory, state: &mut AppState) -> bool {
    let (_, restore) = pages_and_restore();
    let arm = restore
        .split(&format!("PreferenceCategory::{category:?} =>"))
        .nth(1)
        .and_then(|source| source.split("PreferenceCategory::").next())
        .unwrap_or_default();
    let keys = named_keys(arm);
    let mut stored = false;
    for key in ChoicePreference::ALL {
        if !keys.contains(&format!("ChoicePreference::{key:?}")) {
            continue;
        }
        state
            .model
            .preferences
            .set_choice(key, 1)
            .expect("every exposed choice has a second option");
        stored = true;
    }
    for key in TogglePreference::ALL {
        if !keys.contains(&format!("TogglePreference::{key:?}")) {
            continue;
        }
        let flipped = !state.model.preferences.toggle(key);
        state.model.preferences.set_toggle(key, flipped);
        stored = true;
    }
    for (key, value) in [
        (ScalarPreference::DisplayedSignificantDigits, 12),
        (ScalarPreference::LocalParallelSlots, 1),
        (ScalarPreference::RunsKept, 50),
        (ScalarPreference::RecentFilesListed, 20),
    ] {
        if keys.contains(&format!("ScalarPreference::{key:?}")) {
            state
                .model
                .preferences
                .set_scalar(key, value)
                .expect("each override is inside its preference domain");
            stored = true;
        }
    }
    for key in FolderListPreference::ALL {
        if keys.contains(&format!("FolderListPreference::{key:?}")) {
            state
                .model
                .preferences
                .set_folders(key, vec![std::env::temp_dir()])
                .expect("the temporary folder is a full path");
            stored = true;
        }
    }
    if category == PreferenceCategory::Appearance {
        state.workbench.theme.mode = crate::ui::Mode::Light;
        state.workbench.theme.canvas_contrast = 12;
    }
    if category == PreferenceCategory::Projects {
        state.platform.autosave_minutes = 10;
    }
    if category == PreferenceCategory::Accessibility {
        state.settings.browser_spoken_feedback = true;
    }
    if category == PreferenceCategory::Workspace
        && let Ok(workspace) = state.model.preferences.workspace_mut()
    {
        workspace.set_console_on_launch(
            ConsoleLaunchBehavior::from_index(1).expect("the console may launch open"),
        );
        stored = true;
    }
    stored
}
