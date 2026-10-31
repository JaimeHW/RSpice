//! The preferences file, in the case that carries a shortcut profile bound
//! to the product's own commands.

use crate::workbench::UserPreferences;
use crate::workbench::framework::command_registry::ids;

#[test]
fn shortcut_profile_round_trips_inside_user_preferences() {
    use crate::state::command_catalog::CommandPlatform;
    use crate::state::shortcuts::{ShortcutBindingSlot, ShortcutSequence, ShortcutStroke};
    use crate::workbench::commands::vocabulary::CommandRegistry;
    use egui::Key;

    let mut preferences = UserPreferences::default();
    preferences
        .shortcuts_mut()
        .unwrap()
        .set_binding(
            &CommandRegistry,
            ids::SAVE,
            ShortcutBindingSlot::Primary,
            CommandPlatform::ALL.to_vec(),
            Some(ShortcutSequence::single(ShortcutStroke::new(
                Key::F6,
                false,
                false,
                false,
            ))),
        )
        .unwrap();
    let encoded = serde_json::to_string(&preferences).unwrap();
    let restored: UserPreferences = serde_json::from_str(&encoded).unwrap();
    assert_eq!(
        restored
            .shortcuts()
            .resolved_bindings(&CommandRegistry, ids::SAVE)
            .into_iter()
            .find(|binding| { binding.slot() == ShortcutBindingSlot::Primary })
            .unwrap()
            .display_label(),
        "F6"
    );
}
