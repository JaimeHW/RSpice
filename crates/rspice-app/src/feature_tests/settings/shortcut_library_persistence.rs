//! The device-local shortcut library store, in the case that binds the
//! product's own commands.

use crate::state::shortcuts::ShortcutProfileLibrary;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::shortcuts::library_persistence::*;

#[test]
fn protected_override_acknowledgements_round_trip_in_device_local_store() {
    let library: ShortcutProfileLibrary = serde_json::from_value(serde_json::json!({
        "library-version": 1,
        "active": {
            "protected-override-acknowledgements": ["save-project"]
        },
        "named-presets": {},
        "revision": 0
    }))
    .unwrap();
    let bytes = encode_store_bytes(&library, 1, command_registry()).unwrap();
    let ShortcutProfileLibraryRestore::Compatible(restored) =
        decode_store_bytes(&bytes, command_registry())
    else {
        panic!("protected acknowledgement store did not restore");
    };
    assert!(
        restored.library().active().protected_override_acknowledged(
            crate::workbench::framework::command_registry::ids::SAVE
        )
    );
}
