use super::{InputState, Key, Modifiers};

#[derive(Debug, Clone)]
pub(crate) struct ShortcutInputSnapshot {
    pressed_keys: Vec<Key>,
    modifiers: Modifiers,
    has_focus: bool,
}

impl ShortcutInputSnapshot {
    pub(crate) fn from_input_state(input: &InputState, has_focus: bool) -> Self {
        const TRACKED_KEYS: [Key; 37] = [
            Key::K,
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
            Key::U,
            Key::Slash,
            Key::Escape,
            Key::F,
            Key::B,
            Key::J,
            Key::Tab,
            Key::F5,
            Key::Plus,
            Key::Equals,
            Key::Minus,
            Key::Num0,
            Key::Comma,
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

    #[cfg(test)]
    pub(crate) fn from_modifiers_for_test(modifiers: Modifiers) -> Self {
        Self {
            pressed_keys: Vec::new(),
            modifiers,
            has_focus: false,
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

    pub(crate) fn plain(&self) -> bool {
        !self.modifiers.alt
            && !self.modifiers.ctrl
            && !self.modifiers.command
            && !self.modifiers.shift
    }

    pub(super) fn has_focus(&self) -> bool {
        self.has_focus
    }
}
