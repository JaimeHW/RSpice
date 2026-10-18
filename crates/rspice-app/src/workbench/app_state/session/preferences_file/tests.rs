//! The preferences file against real files in a temporary folder: what it
//! keeps, what it sets aside, when it is written, and what an import may
//! change. Nothing here reads or writes the real settings folder.

use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::*;
use crate::ui::Mode;
use crate::workbench::{
    ChoicePreference, FolderListPreference, ScalarPreference, TogglePreference,
};

/// A preferences file in a folder of its own, never the real settings folder.
fn temp_store(label: &str) -> (PathBuf, PreferencesStore) {
    let folder = std::env::temp_dir().join(format!(
        "rspice-preferences-{label}-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&folder).expect("temp folder");
    let path = folder.join(FILE_NAME);
    (path.clone(), PreferencesStore::File(path))
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).expect("preferences file")).expect("JSON")
}

#[derive(Default)]
struct MemoryStorage(std::collections::HashMap<String, String>);

impl eframe::Storage for MemoryStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    fn set_string(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }

    fn remove_string(&mut self, key: &str) {
        self.0.remove(key);
    }

    fn flush(&mut self) {}
}

/// Move every setting the file holds off its default.
fn customize(state: &mut AppState) {
    state.ui.theme.mode = Mode::Light;
    state.ui.theme.canvas_contrast = 40;
    state.ui.autosave_minutes = 10;
    state.ui.browser_spoken_feedback = true;
    state
        .ui
        .preferences
        .set_choice(ChoicePreference::InterfaceScale, 2)
        .expect("interface scale");
    state
        .ui
        .preferences
        .set_toggle(TogglePreference::ReducedMotion, true);
    state
        .ui
        .preferences
        .set_scalar(ScalarPreference::DisplayedSignificantDigits, 12)
        .expect("significant digits");
    state
        .ui
        .preferences
        .set_folders(
            FolderListPreference::LibraryFolders,
            vec![
                std::env::temp_dir().join("rspice-library-b"),
                std::env::temp_dir().join("rspice-library-a"),
            ],
        )
        .expect("library folders");
}

/// The four settings the file holds, comparable across states.
fn settings_of(state: &AppState) -> Value {
    let mut preferences = serde_json::to_value(&state.ui.preferences).expect("preferences");
    preferences
        .as_object_mut()
        .expect("preferences object")
        .remove(SHORTCUTS);
    json!({
        "theme": serde_json::to_value(state.ui.theme).expect("theme"),
        "autosave": state.ui.autosave_minutes,
        "spoken": state.ui.browser_spoken_feedback,
        "preferences": preferences,
    })
}

fn bound(store: &PreferencesStore) -> AppState {
    let mut state = AppState::default();
    state.bind_preferences_file(store.clone());
    state
}

fn console(state: &AppState) -> Vec<(LogSeverity, String)> {
    state
        .log_buffer
        .entries()
        .map(|entry| (entry.severity, entry.message.clone()))
        .collect()
}

#[test]
fn settings_round_trip_with_unknown_keys_and_domains_kept() {
    let (path, store) = temp_store("round-trip");
    let mut first = AppState::default();
    customize(&mut first);
    first.bind_preferences_file(store.clone());

    // Make it the file a newer build left behind: a later version, a key of
    // its own at the top, a theme field, a preference key, and a whole
    // preference domain this build has never heard of.
    let mut file = read_json(&path);
    assert!(file[PREFERENCES].get(SHORTCUTS).is_none(), "{file}");
    file["version"] = json!(7);
    file["future-sync"] = json!({"enabled": true});
    file[THEME]["future-accent"] = json!("teal");
    file[PREFERENCES]["choices"]["future-density"] = json!(3);
    file[PREFERENCES]["future-domain"] = json!({"rows": [1, 2]});
    let newer = crate::workbench::shortcuts::artifacts::canonical_json_bytes(file).unwrap();
    std::fs::write(&path, &newer).unwrap();

    let mut second = bound(&store);
    assert_eq!(second.ui.theme, first.ui.theme);
    assert_eq!(second.ui.autosave_minutes, 10);
    assert!(second.ui.browser_spoken_feedback);
    assert_eq!(
        second
            .ui
            .preferences
            .choice(ChoicePreference::InterfaceScale),
        2
    );
    second.flush_preferences_file();
    assert_eq!(
        std::fs::read(&path).unwrap(),
        newer,
        "nothing changed, so the newer build's file is left byte for byte"
    );

    second.ui.theme.mode = Mode::Dark;
    second.flush_preferences_file();
    let rewritten = read_json(&path);
    assert_eq!(rewritten["version"], 7);
    assert_eq!(rewritten["future-sync"]["enabled"], true);
    assert_eq!(rewritten[THEME]["future-accent"], "teal");
    assert_eq!(
        rewritten[THEME]["mode"],
        serde_json::to_value(Mode::Dark).unwrap()
    );
    assert_eq!(rewritten[PREFERENCES]["choices"]["future-density"], 3);
    assert_eq!(
        rewritten[PREFERENCES]["future-domain"]["rows"],
        json!([1, 2])
    );

    let third = bound(&store);
    assert_eq!(settings_of(&third), settings_of(&second));
}

#[test]
fn a_setting_a_newer_build_wrote_is_kept_until_it_is_changed_here() {
    let (path, store) = temp_store("unread");
    let mut first = AppState::default();
    customize(&mut first);
    first.bind_preferences_file(store.clone());
    let mut file = read_json(&path);
    file[THEME]["mode"] = json!("future-mode");
    file[THEME]["future-accent"] = json!("teal");
    std::fs::write(&path, serde_json::to_vec(&file).unwrap()).unwrap();

    let mut state = bound(&store);
    assert_eq!(state.ui.theme, Theme::default(), "the theme falls back");
    assert_eq!(state.ui.autosave_minutes, 10, "the rest still loads");
    assert!(state.ui.preferences.toggle(TogglePreference::ReducedMotion));

    state
        .ui
        .preferences
        .set_toggle(TogglePreference::ReducedMotion, false);
    state.flush_preferences_file();
    assert_eq!(
        read_json(&path)[THEME],
        file[THEME],
        "another change leaves the theme it could not read as it was"
    );

    state.ui.theme.mode = Mode::Light;
    state.flush_preferences_file();
    let written = read_json(&path);
    assert_eq!(
        written[THEME]["mode"],
        serde_json::to_value(Mode::Light).unwrap()
    );
    assert_eq!(written[THEME]["future-accent"], "teal");
}

/// A file written before library folders existed reads as none and is left
/// exactly as it was. A folder list this build cannot read takes nothing else
/// with it, and is written back as it was found.
#[test]
fn library_folders_decode_from_files_that_predate_or_garble_them() {
    let (path, store) = temp_store("library-folders");
    let mut first = AppState::default();
    customize(&mut first);
    first
        .ui
        .preferences
        .restore_folder_defaults(&[FolderListPreference::LibraryFolders]);
    first.bind_preferences_file(store.clone());
    let older = std::fs::read(&path).expect("the file was written");
    assert!(
        read_json(&path)[PREFERENCES].get("folders").is_none(),
        "no list is the shipped state and is not written"
    );

    let mut state = bound(&store);
    assert!(
        state
            .ui
            .preferences
            .folders(FolderListPreference::LibraryFolders)
            .is_empty()
    );
    state.flush_preferences_file();
    assert_eq!(std::fs::read(&path).unwrap(), older);

    let mut file = read_json(&path);
    file[PREFERENCES]["folders"] = json!({"library-folders": "not a list"});
    std::fs::write(&path, serde_json::to_vec(&file).unwrap()).unwrap();
    let mut state = bound(&store);
    assert!(
        state
            .ui
            .preferences
            .folders(FolderListPreference::LibraryFolders)
            .is_empty()
    );
    assert_eq!(
        state
            .ui
            .preferences
            .choice(ChoicePreference::InterfaceScale),
        2,
        "every other preference still loads"
    );
    state.ui.theme.mode = Mode::Dark;
    state.flush_preferences_file();
    assert_eq!(
        read_json(&path)[PREFERENCES]["folders"]["library-folders"],
        "not a list"
    );
}

#[test]
fn a_session_that_will_not_decode_keeps_the_preferences() {
    let (_path, store) = temp_store("session-failure");
    let mut earlier = AppState::default();
    customize(&mut earlier);
    earlier.bind_preferences_file(store.clone());

    let mut storage = MemoryStorage::default();
    eframe::Storage::set_string(&mut storage, eframe::APP_KEY, "(not a session".to_owned());
    let restored = AppState::restore_with_store(Some(&storage), Some(store));

    assert_eq!(settings_of(&restored), settings_of(&earlier));
    assert!(
        console(&restored)
            .iter()
            .any(|(_, message)| message.starts_with("The previous session could not be restored")),
        "the session's own warning still stands"
    );
}

#[test]
fn a_session_only_profile_migrates_into_the_file_once_and_loses_nothing() {
    let (path, store) = temp_store("migration");
    let mut old = AppState::default();
    customize(&mut old);
    let mut storage = MemoryStorage::default();
    eframe::set_value(&mut storage, eframe::APP_KEY, &old);

    let first = AppState::restore_with_store(Some(&storage), Some(store.clone()));
    assert_eq!(settings_of(&first), settings_of(&old));
    assert_eq!(first.preferences_file.writes, 1);
    let migrated = std::fs::read(&path).expect("the file was written");

    // The next launch restores a session that knows nothing of the file. The
    // file wins, and nothing is written again.
    eframe::set_value(&mut storage, eframe::APP_KEY, &AppState::default());
    let second = AppState::restore_with_store(Some(&storage), Some(store));
    assert_eq!(settings_of(&second), settings_of(&old));
    assert_eq!(second.preferences_file.writes, 0);
    assert_eq!(std::fs::read(&path).unwrap(), migrated);
}

#[test]
fn an_unreadable_file_is_set_aside_and_named() {
    let (path, store) = temp_store("corrupt");
    std::fs::write(&path, b"{ this is not json").unwrap();
    let mut state = AppState::default();
    customize(&mut state);

    state.bind_preferences_file(store);

    assert_eq!(settings_of(&state), settings_of(&AppState::default()));
    let backups: Vec<PathBuf> = std::fs::read_dir(path.parent().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|candidate| {
            candidate
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("preferences.json.bad-"))
        })
        .collect();
    assert_eq!(backups.len(), 1, "{backups:?}");
    assert_eq!(
        std::fs::read(&backups[0]).unwrap(),
        b"{ this is not json".to_vec()
    );
    let warnings = console(&state);
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert_eq!(warnings[0].0, LogSeverity::Warning);
    assert!(
        warnings[0].1.contains(&backups[0].display().to_string()),
        "{}",
        warnings[0].1
    );
    assert_eq!(
        read_json(&path)["format"],
        FORMAT,
        "a fresh file replaces it"
    );
}

#[test]
fn a_change_is_written_once_after_it_settles_not_every_frame() {
    let (path, store) = temp_store("debounce");
    let mut state = bound(&store);
    assert_eq!(state.preferences_file.writes, 1, "the first file");
    let start = Instant::now();
    let at = |ms: u64| start + Duration::from_millis(ms);

    // A value dragged for half a second changes on every frame.
    for frame in 0..30_u64 {
        let digits = 6 + u32::try_from(frame % 6).unwrap();
        state
            .ui
            .preferences
            .set_scalar(ScalarPreference::DisplayedSignificantDigits, digits)
            .unwrap();
        let wait = state.persist_preferences_file_at(at(frame * 16), true);
        assert!(
            wait.is_some(),
            "frame {frame} waits for the change to settle"
        );
    }
    assert_eq!(state.preferences_file.writes, 1, "nothing while it moves");

    // It settles: frames without input write it once.
    let mut written_at = None;
    for frame in 30..120_u64 {
        let _ = state.persist_preferences_file_at(at(frame * 16), false);
        if written_at.is_none() && state.preferences_file.writes == 2 {
            written_at = Some(frame);
        }
    }
    assert_eq!(state.preferences_file.writes, 2);
    let settled = written_at.expect("written") * 16;
    assert!(
        (29 * 16 + 500..29 * 16 + 500 + 32).contains(&settled),
        "written {settled} ms in, half a second after the last change"
    );
    assert_eq!(settings_of(&bound(&store)), settings_of(&state));

    // Frames that change nothing write nothing, however many there are.
    for frame in 120..400_u64 {
        let _ = state.persist_preferences_file_at(at(frame * 16), frame % 2 == 0);
    }
    assert_eq!(state.preferences_file.writes, 2);
    assert!(path.exists());
}

#[test]
fn pointer_movement_alone_never_hashes_the_settings() {
    let (_path, store) = temp_store("pointer-move");
    let mut state = bound(&store);
    let start = Instant::now();
    let at = |ms: u64| start + Duration::from_millis(ms);
    let _ = state.persist_preferences_file_at(at(0), false);
    let looks = state.preferences_file.looks;

    // A canvas drag or a hover: nothing but movement, every frame, for
    // under the idle second.
    let hover = [
        egui::Event::PointerMoved(egui::pos2(10.0, 20.0)),
        egui::Event::MouseMoved(egui::vec2(1.0, 0.0)),
    ];
    for frame in 1..60_u64 {
        let _ =
            state.persist_preferences_file_at(at(frame * 16), input_can_change_a_setting(&hover));
    }
    assert_eq!(
        state.preferences_file.looks, looks,
        "movement hashed nothing"
    );

    let release = [egui::Event::PointerButton {
        pos: egui::pos2(10.0, 20.0),
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::default(),
    }];
    let _ = state.persist_preferences_file_at(at(960), input_can_change_a_setting(&release));
    assert_eq!(
        state.preferences_file.looks,
        looks + 1,
        "a release is looked at"
    );

    // The shortcut library is not in the file, so it is not in the hash, and
    // it comes back from the hash exactly as it went in.
    let before = fingerprint(&mut state.ui);
    state
        .ui
        .preferences
        .shortcut_profiles_mut()
        .active_mut()
        .unwrap()
        .policies_mut()
        .set_chord_timeout(crate::workbench::ChordTimeoutPolicy::ThreeSeconds);
    let edited = state.ui.preferences.shortcut_profiles().clone();
    assert_eq!(fingerprint(&mut state.ui), before);
    assert_eq!(state.ui.preferences.shortcut_profiles(), &edited);
}

#[test]
fn a_failed_write_is_reported_once_and_retried_on_the_next_change() {
    let (path, store) = temp_store("write-failure");
    let mut state = bound(&store);
    // Something else takes the file's name: every write now fails.
    std::fs::remove_file(&path).unwrap();
    std::fs::create_dir(&path).unwrap();
    let start = Instant::now();

    for (step, minutes) in [2_u8, 10, 2].into_iter().enumerate() {
        state.ui.autosave_minutes = minutes;
        let base = u64::try_from(step).unwrap() * 2_000;
        let _ = state.persist_preferences_file_at(start + Duration::from_millis(base), true);
        let _ = state.persist_preferences_file_at(start + Duration::from_millis(base + 600), false);
    }
    let errors: Vec<_> = console(&state)
        .into_iter()
        .filter(|(severity, _)| *severity == LogSeverity::Error)
        .collect();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(
        errors[0]
            .1
            .starts_with("Preferences could not be saved to ")
    );

    std::fs::remove_dir(&path).unwrap();
    state.ui.autosave_minutes = 10;
    let _ = state.persist_preferences_file_at(start + Duration::from_secs(10), true);
    let _ = state.persist_preferences_file_at(start + Duration::from_secs(11), false);
    assert_eq!(read_json(&path)[AUTOSAVE], 10, "the next change is written");
}

#[test]
fn an_unbound_state_never_touches_the_disk() {
    let mut state = AppState::default();
    customize(&mut state);
    assert_eq!(
        state.persist_preferences_file_at(Instant::now(), true),
        None
    );
    state.flush_preferences_file();
    assert_eq!(state.preferences_file.writes, 0);
}

fn exported(label: &str) -> (AppState, Vec<u8>) {
    let (_path, store) = temp_store(label);
    let mut source = AppState::default();
    customize(&mut source);
    source.enable_volatile_test_shortcut_persistence();
    let mut shortcuts = source.ui.preferences.shortcut_profiles().clone();
    shortcuts
        .active_mut()
        .unwrap()
        .policies_mut()
        .set_chord_timeout(crate::workbench::ChordTimeoutPolicy::ThreeSeconds);
    *source.ui.preferences.shortcut_profiles_mut() = shortcuts;
    source.bind_preferences_file(store);
    let bytes = source.settings_file_bytes().expect("export");
    (source, bytes)
}

#[test]
fn a_settings_file_carries_preferences_and_shortcuts_to_another_machine() {
    let (source, bytes) = exported("export");
    let (path, store) = temp_store("import");
    let mut target = bound(&store);
    target.enable_volatile_test_shortcut_persistence();

    let (shortcuts, staged) = target
        .stage_settings_import("rspice-settings.json", &bytes)
        .expect("a file RSpice exported imports");
    assert_eq!(
        settings_of(&target),
        settings_of(&AppState::default()),
        "staging changes nothing"
    );
    assert_eq!(shortcuts.active(), source.ui.preferences.shortcuts());
    assert!(shortcuts.revision() > target.ui.preferences.shortcut_profiles().revision());

    let ctx = egui::Context::default();
    assert!(matches!(
        target.publish_shortcut_library_candidate(&shortcuts, &ctx),
        Ok(super::super::shortcut_library::ShortcutLibraryPublication::Published)
    ));
    target.finish_settings_import(staged, Ok(()));

    assert_eq!(settings_of(&target), settings_of(&source));
    assert_eq!(
        target.ui.preferences.shortcuts(),
        source.ui.preferences.shortcuts()
    );
    assert_eq!(
        target.prompt_text(),
        Some("Settings imported from rspice-settings.json.")
    );
    assert_eq!(settings_of(&bound(&store)), settings_of(&source));
    assert!(path.exists());
}

#[test]
fn a_malformed_settings_file_changes_nothing() {
    let (_source, good) = exported("malformed-source");
    let (path, store) = temp_store("malformed");
    let mut state = AppState::default();
    customize(&mut state);
    state.bind_preferences_file(store);
    state.enable_volatile_test_shortcut_persistence();
    let before = settings_of(&state);
    let shortcuts_before = state.ui.preferences.shortcut_profiles().clone();
    let file_before = std::fs::read(&path).unwrap();

    let mut wrong_theme: Value = serde_json::from_slice(&good).unwrap();
    wrong_theme[PREFERENCES][THEME]["mode"] = json!("future-mode");
    let mut wrong_shortcuts: Value = serde_json::from_slice(&good).unwrap();
    wrong_shortcuts[SHORTCUTS] = json!("not a library");
    let mut no_preferences: Value = serde_json::from_slice(&good).unwrap();
    no_preferences.as_object_mut().unwrap().remove(PREFERENCES);
    let preferences_file_instead =
        serde_json::from_slice::<Value>(&good).unwrap()[PREFERENCES].clone();

    for (case, bytes, reason) in [
        ("not JSON", b"{ nope".to_vec(), "is not valid JSON"),
        (
            "another format",
            serde_json::to_vec(&preferences_file_instead).unwrap(),
            "not an RSpice settings file",
        ),
        (
            "no preferences",
            serde_json::to_vec(&no_preferences).unwrap(),
            "holds no preferences",
        ),
        (
            "unreadable theme",
            serde_json::to_vec(&wrong_theme).unwrap(),
            "its theme could not be read",
        ),
        (
            "unreadable shortcuts",
            serde_json::to_vec(&wrong_shortcuts).unwrap(),
            "keyboard shortcuts",
        ),
    ] {
        let refusal = state
            .stage_settings_import("bad.json", &bytes)
            .expect_err(case);
        assert!(refusal.contains(reason), "{case}: {refusal}");
    }
    assert_eq!(settings_of(&state), before);
    assert_eq!(state.ui.preferences.shortcut_profiles(), &shortcuts_before);
    assert_eq!(std::fs::read(&path).unwrap(), file_before);
}

#[test]
fn an_import_whose_shortcuts_cannot_be_saved_changes_nothing() {
    let (_source, bytes) = exported("unsaved-source");
    let (path, store) = temp_store("unsaved");
    let mut state = bound(&store);
    let before = settings_of(&state);
    let file_before = std::fs::read(&path).unwrap();

    let (_shortcuts, staged) = state
        .stage_settings_import("rspice-settings.json", &bytes)
        .unwrap();
    state.finish_settings_import(staged, Err("the shortcut store is unavailable".to_owned()));

    assert_eq!(settings_of(&state), before);
    assert_eq!(std::fs::read(&path).unwrap(), file_before);
    assert_eq!(
        state.prompt_text(),
        Some(
            "Settings were not imported from rspice-settings.json: the shortcut store is \
             unavailable. Nothing was changed."
        )
    );
}

#[test]
fn show_file_opens_the_folder_that_holds_it() {
    let (path, store) = temp_store("show");
    let mut state = bound(&store);
    SHOWN_FOLDERS.with_borrow_mut(Vec::clear);
    state.show_preferences_file();
    assert_eq!(
        SHOWN_FOLDERS.with_borrow(Clone::clone),
        vec![path.parent().unwrap().to_path_buf()]
    );

    let mut unbound = AppState::default();
    unbound.show_preferences_file();
    assert_eq!(
        unbound.prompt_text(),
        Some("Preferences are not kept in a file on this machine.")
    );
}
