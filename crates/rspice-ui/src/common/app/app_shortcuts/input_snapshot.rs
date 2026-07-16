use egui::Event;

use super::{InputState, Key, Modifiers};

/// One non-repeating key-down event in exact egui event order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ShortcutKeyPress {
    key: Key,
    modifiers: Modifiers,
}

impl ShortcutKeyPress {
    pub(super) const fn key(self) -> Key {
        self.key
    }

    pub(super) const fn modifiers(self) -> Modifiers {
        self.modifiers
    }

    pub(super) fn matches(self, key: Key, primary: bool, alt: bool, shift: bool) -> bool {
        self.key == key && modifiers_match(self.modifiers, primary, alt, shift)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ShortcutInputSnapshot {
    key_presses: Vec<ShortcutKeyPress>,
    non_canvas_focus: bool,
}

impl ShortcutInputSnapshot {
    pub(crate) fn from_input_state(input: &InputState, non_canvas_focus: bool) -> Self {
        Self {
            key_presses: collect_key_presses(&input.events),
            non_canvas_focus,
        }
    }

    pub(super) fn key_presses(&self) -> &[ShortcutKeyPress] {
        &self.key_presses
    }

    pub(super) const fn has_non_canvas_focus(&self) -> bool {
        self.non_canvas_focus
    }

    #[cfg(test)]
    pub(crate) fn from_events_for_test(events: &[Event], non_canvas_focus: bool) -> Self {
        Self {
            key_presses: collect_key_presses(events),
            non_canvas_focus,
        }
    }

    #[cfg(test)]
    pub(crate) fn empty_for_test(non_canvas_focus: bool) -> Self {
        Self {
            key_presses: Vec::new(),
            non_canvas_focus,
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
        // ambiguous physical state: Control held together with macOS Command.
        actual.command && !(actual.ctrl && actual.mac_cmd)
    } else {
        // Physical macOS Control does not set `command`; include every source
        // so Ctrl+W cannot accidentally become the plain W canvas tool.
        !actual.command && !actual.ctrl && !actual.mac_cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key_event(key: Key, modifiers: Modifiers, pressed: bool, repeat: bool) -> Event {
        Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat,
            modifiers,
        }
    }

    #[test]
    fn snapshot_preserves_order_and_ignores_repeat_and_release() {
        let events = [
            key_event(Key::K, Modifiers::COMMAND, true, false),
            key_event(Key::C, Modifiers::COMMAND, true, true),
            key_event(Key::V, Modifiers::COMMAND, false, false),
            key_event(Key::S, Modifiers::COMMAND, true, false),
        ];
        let snapshot = ShortcutInputSnapshot::from_events_for_test(&events, false);
        assert_eq!(
            snapshot
                .key_presses()
                .iter()
                .map(|press| press.key())
                .collect::<Vec<_>>(),
            vec![Key::K, Key::S]
        );
    }

    #[test]
    fn primary_modifier_accepts_ctrl_or_command_but_not_both() {
        for modifiers in [
            Modifiers {
                ctrl: true,
                command: true,
                ..Modifiers::NONE
            },
            Modifiers {
                mac_cmd: true,
                command: true,
                ..Modifiers::NONE
            },
        ] {
            assert!(
                ShortcutKeyPress {
                    key: Key::K,
                    modifiers
                }
                .matches(Key::K, true, false, false)
            );
        }
        assert!(
            !ShortcutKeyPress {
                key: Key::K,
                modifiers: Modifiers {
                    ctrl: true,
                    mac_cmd: true,
                    command: true,
                    ..Modifiers::NONE
                },
            }
            .matches(Key::K, true, false, false)
        );
        assert!(
            !ShortcutKeyPress {
                key: Key::W,
                modifiers: Modifiers {
                    ctrl: true,
                    ..Modifiers::NONE
                },
            }
            .matches(Key::W, false, false, false)
        );
    }
}
