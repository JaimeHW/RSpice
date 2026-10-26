//! The shortcut profile and library held to this build's command registry.
//!
//! A profile reads its command facts through a catalog and names no command,
//! so these cases, which check it against the commands this build ships,
//! live where the registry can be named.

use serde_json::Value;

use crate::state::command_catalog::CommandCatalog;
use crate::state::shortcuts::{
    ShortcutPolicies, ShortcutPreferences, ShortcutProfileLibrary, ShortcutProfileLibraryError,
};
use crate::workbench::commands::vocabulary::CommandRegistry;

/// The options `AppState::from_session_ron` reads the eframe session
/// with. The session is written by `ron::ser::to_string`.
fn session_ron_options() -> ron::Options {
    ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
}

#[test]
fn binding_count_is_expanded_across_supported_platforms() {
    let preferences = ShortcutPreferences::default();
    let expected: usize = CommandRegistry
        .commands()
        .iter()
        .copied()
        .flat_map(|command| preferences.resolved_bindings(&CommandRegistry, command))
        .map(|binding| binding.platforms().len())
        .sum();
    assert_eq!(
        preferences.audit(&CommandRegistry).binding_count(),
        expected
    );
}

#[test]
fn malformed_legacy_root_round_trips_and_all_normal_mutation_fails_closed() {
    let mut library: ShortcutProfileLibrary = serde_json::from_str("17").unwrap();
    assert_eq!(serde_json::to_value(&library).unwrap(), 17);
    assert!(!library.active().audit(&CommandRegistry).is_valid());
    assert!(matches!(
        library.active_mut(),
        Err(ShortcutProfileLibraryError::IncompatibleLibrary)
    ));
    assert_eq!(library.revision(), 0);
    assert_eq!(serde_json::to_value(library).unwrap(), 17);
}

#[test]
fn future_library_refuses_preset_mutation_without_discarding_raw_state() {
    let source = r#"{
        "library-version":99,
        "active":{"commands":{}},
        "named-presets":{"Future":{"commands":{}}},
        "revision":12,
        "future-library":{"required":true}
    }"#;
    let mut library: ShortcutProfileLibrary = serde_json::from_str(source).unwrap();
    assert!(!library.is_compatible());
    assert!(matches!(
        library.insert_named_preset("Imported", ShortcutPreferences::default(), false),
        Err(ShortcutProfileLibraryError::IncompatibleLibrary)
    ));
    assert!(matches!(
        library.active_mut(),
        Err(ShortcutProfileLibraryError::IncompatibleLibrary)
    ));
    assert!(matches!(
        library.edit_active(|profile| profile.reset_all(&CommandRegistry)),
        Err(ShortcutProfileLibraryError::IncompatibleLibrary)
    ));
    assert!(matches!(
        library.replace_active(ShortcutPreferences::default()),
        Err(ShortcutProfileLibraryError::IncompatibleLibrary)
    ));
    assert!(matches!(
        library.remove_named_preset("Future"),
        Err(ShortcutProfileLibraryError::IncompatibleLibrary)
    ));
    assert_eq!(
        serde_json::to_value(library).unwrap(),
        serde_json::from_str::<Value>(source).unwrap()
    );
}

/// A session an earlier build saved holds each policy as `()`. It reads
/// as the shipped policies, so the profile is not refused as one this
/// build does not understand.
#[test]
fn a_policy_saved_as_unit_reads_as_the_shipped_policy() {
    let saved = r#"{"library-version":1,"active":{"policies":{"single-key-canvas":(),"chord-timeout":(),"protected-shortcuts":(),"context-precedence":()},"commands":{},"protected-override-acknowledgements":[]},"named-presets":{},"revision":3}"#;

    let read: ShortcutProfileLibrary = session_ron_options()
        .from_str(saved)
        .expect("the session reads the library back");

    assert_eq!(read.active().policies(), &ShortcutPolicies::default());
    assert!(
        read.active()
            .policies()
            .unknown_execution_policy_names()
            .is_empty()
    );
    assert!(read.active().audit(&CommandRegistry).is_valid());
}
