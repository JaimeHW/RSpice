use egui::Event;

use super::{InputState, Key, Modifiers};

#[derive(Debug, Clone, Copy)]
struct ShortcutKeyPress {
    key: Key,
    modifiers: Modifiers,
}

#[derive(Debug, Clone)]
pub(crate) struct ShortcutInputSnapshot {
    key_presses: Vec<ShortcutKeyPress>,
    has_focus: bool,
}

impl ShortcutInputSnapshot {
    pub(crate) fn from_input_state(input: &InputState, has_focus: bool) -> Self {
        Self {
            key_presses: collect_key_presses(&input.events),
            has_focus,
        }
    }

    pub(super) fn matching_modifiers(
        &self,
        key: Key,
        primary: bool,
        alt: bool,
        shift: bool,
    ) -> Option<Modifiers> {
        self.key_presses
            .iter()
            .find(|press| press.key == key && modifiers_match(press.modifiers, primary, alt, shift))
            .map(|press| press.modifiers)
    }

    pub(super) fn has_focus(&self) -> bool {
        self.has_focus
    }

    #[cfg(test)]
    pub(crate) fn for_test(key: Key, modifiers: Modifiers, has_focus: bool) -> Self {
        Self {
            key_presses: vec![ShortcutKeyPress { key, modifiers }],
            has_focus,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_events_for_test(events: &[Event], has_focus: bool) -> Self {
        Self {
            key_presses: collect_key_presses(events),
            has_focus,
        }
    }
}

fn collect_key_presses(events: &[Event]) -> Vec<ShortcutKeyPress> {
    events
        .iter()
        .filter_map(|event| match event {
            Event::Key {
                key,
                pressed: true,
                repeat: false,
                modifiers,
                ..
            } => Some(ShortcutKeyPress {
                key: *key,
                modifiers: *modifiers,
            }),
            _ => None,
        })
        .collect()
}

fn modifiers_match(actual: Modifiers, primary: bool, alt: bool, shift: bool) -> bool {
    if actual.alt != alt || actual.shift != shift {
        return false;
    }
    if primary {
        // `command` is egui's portable primary modifier. Reject the only
        // genuinely ambiguous state: physical Control held together with
        // macOS Command. Normal Windows/Linux Ctrl and macOS Command events
        // each set only their own physical source plus `command`.
        actual.command && !(actual.ctrl && actual.mac_cmd)
    } else {
        // A physical macOS Control key does not set `command`; include every
        // source here so Ctrl+W cannot accidentally become the plain W tool.
        !actual.command && !actual.ctrl && !actual.mac_cmd
    }
}
