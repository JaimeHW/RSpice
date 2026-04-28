use super::{InputState, Key, Modifiers};

#[derive(Debug, Clone)]
pub(crate) struct ShortcutInputSnapshot {
    pressed_keys: Vec<Key>,
    modifiers: Modifiers,
    has_focus: bool,
}

impl ShortcutInputSnapshot {
    pub(crate) fn from_input_state(input: &InputState, has_focus: bool) -> Self {
        const TRACKED_KEYS: [Key; 24] = [
            Key::N,
            Key::O,
            Key::S,
            Key::Z,
            Key::Y,
            Key::C,
            Key::V,
            Key::X,
            Key::Delete,
            Key::A,
            Key::L,
            Key::Backtick,
            Key::F1,
            Key::W,
            Key::G,
            Key::I,
            Key::D,
            Key::M,
            Key::Q,
            Key::P,
            Key::R,
            Key::H,
            Key::E,
            Key::Escape,
        ];

        let pressed_keys = TRACKED_KEYS
            .iter()
            .copied()
            .filter(|key| input.key_pressed(*key))
            .collect();

        Self {
            pressed_keys,
            modifiers: input.modifiers,
            has_focus,
        }
    }


    pub(super) fn key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub(super) fn ctrl(&self) -> bool {
        self.modifiers.ctrl
    }

    pub(super) fn shift(&self) -> bool {
        self.modifiers.shift
    }

    pub(super) fn has_focus(&self) -> bool {
        self.has_focus
    }
}
