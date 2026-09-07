//! A frame of keyboard input.
//!
//! Captured once per frame so every resolution step sees the same input,
//! and consumed keys are reported back to egui exactly once.

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

    /// Remove the exact non-repeating presses reported by the resolver in
    /// event order. `InputState::consume_key` removes every logical match, so
    /// calling it once per press loses repeated strokes and can also consume
    /// a different Shift/Alt binding. Validate the whole batch before removal.
    pub(crate) fn consume_keys(input: &mut InputState, keys: &[(Key, Modifiers)]) -> bool {
        if keys.is_empty() {
            return true;
        }
        let Some(indices) = matching_key_indices(&input.events, keys) else {
            return false;
        };
        remove_events_at_indices(&mut input.events, indices);
        true
    }

    /// Separate input for the old and new focus owners, excluding the command
    /// strokes themselves. Text, paste, IME, and editing keys retain their order.
    pub(crate) fn partition_after_keys(
        input: &InputState,
        keys: &[(Key, Modifiers)],
    ) -> Option<(Vec<Event>, Vec<Event>)> {
        let mut indices = matching_key_indices(&input.events, keys)?;
        // Integrations emit a printable key's text immediately after its key
        // event. A custom bare/Shift/Alt chord owns that text too; otherwise
        // the opening character would become part of the new search query.
        for (position, (key, modifiers)) in keys.iter().enumerate() {
            let text_index = indices[position] + 1;
            if !modifiers.command
                && !modifiers.ctrl
                && !modifiers.mac_cmd
                && (key.name().chars().count() == 1 || *key == Key::Space)
                && matches!(input.events.get(text_index), Some(Event::Text(_)))
            {
                indices.push(text_index);
            }
        }
        indices.sort_unstable();
        let end = indices.last().map_or(0, |index| index + 1);
        let mut before = input.events[..end].to_vec();
        remove_events_at_indices(&mut before, indices);
        Some((before, input.events[end..].to_vec()))
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

fn matching_key_indices(events: &[Event], keys: &[(Key, Modifiers)]) -> Option<Vec<usize>> {
    let mut events = events.iter().enumerate();
    let mut indices = Vec::with_capacity(keys.len());
    for (key, modifiers) in keys {
        let (index, _) = events.find(|(_, event)| {
            matches!(event, Event::Key {
                key: event_key,
                modifiers: event_modifiers,
                pressed: true,
                repeat: false,
                ..
            } if event_key == key && event_modifiers == modifiers)
        })?;
        indices.push(index);
    }
    Some(indices)
}

fn remove_events_at_indices(events: &mut Vec<Event>, indices: Vec<usize>) {
    let mut indices = indices.into_iter().peekable();
    let mut index = 0;
    events.retain(|_| {
        let consume = indices.peek() == Some(&index);
        if consume {
            indices.next();
        }
        index += 1;
        !consume
    });
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
    fn consumption_preserves_text_releases_repeats_and_other_bindings() {
        let plain = key_event(Key::S, Modifiers::COMMAND, true, false);
        let shifted = key_event(Key::S, Modifiers::COMMAND | Modifiers::SHIFT, true, false);
        let release = key_event(Key::S, Modifiers::COMMAND, false, false);
        let repeat = key_event(Key::S, Modifiers::COMMAND, true, true);
        let text = Event::Text("s".to_owned());
        let mut input = InputState::default();
        input.events = vec![
            plain.clone(),
            text.clone(),
            release.clone(),
            shifted.clone(),
            repeat.clone(),
            plain,
        ];

        assert!(ShortcutInputSnapshot::consume_keys(
            &mut input,
            &[(Key::S, Modifiers::COMMAND), (Key::S, Modifiers::COMMAND)],
        ));
        assert_eq!(input.events, vec![text, release, shifted, repeat]);
    }

    #[test]
    fn incomplete_consumption_preserves_the_entire_input_queue() {
        let events = vec![
            key_event(Key::K, Modifiers::COMMAND, true, false),
            Event::Text("query".to_owned()),
        ];
        let mut input = InputState::default();
        input.events = events.clone();

        assert!(!ShortcutInputSnapshot::consume_keys(
            &mut input,
            &[(Key::K, Modifiers::COMMAND), (Key::S, Modifiers::COMMAND)],
        ));
        assert_eq!(input.events, events);
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
