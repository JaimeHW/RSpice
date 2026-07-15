use super::{InputState, Key, Modifiers};

#[derive(Debug, Clone)]
pub(crate) struct ShortcutInputSnapshot {
    pressed_keys: Vec<Key>,
    modifiers: Modifiers,
    has_focus: bool,
}

impl ShortcutInputSnapshot {
    pub(crate) fn from_input_state(input: &InputState, has_focus: bool) -> Self {
        const TRACKED_KEYS: [Key; 47] = [
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
            Key::Backspace,
            Key::A,
            Key::L,
            Key::Backtick,
            Key::F1,
            Key::F11,
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
            Key::Enter,
            Key::Plus,
            Key::Equals,
            Key::Minus,
            Key::Num0,
            Key::Num1,
            Key::Num2,
            Key::Num3,
            Key::Num4,
            Key::Num5,
            Key::Num6,
            Key::Num7,
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

    pub(super) fn primary(&self) -> bool {
        self.modifiers.command
    }

    pub(super) fn alt(&self) -> bool {
        self.modifiers.alt
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

    pub(crate) fn modifiers(&self) -> Modifiers {
        self.modifiers
    }

    #[cfg(test)]
    pub(crate) fn for_test(key: Key, modifiers: Modifiers, has_focus: bool) -> Self {
        Self {
            pressed_keys: vec![key],
            modifiers,
            has_focus,
        }
    }
}
