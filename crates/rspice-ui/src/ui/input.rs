//! Ordered input transfer when a command changes the focused surface.

use egui::{Context, Event, Id, InputState, Key, KeyboardShortcut, Modifiers};

/// Input on either side of an activating command, excluding its key strokes.
pub(crate) struct InputTransition {
    before: Vec<Event>,
    following: Vec<Event>,
}

impl InputTransition {
    pub(crate) fn after_keys(input: &InputState, keys: &[(Key, Modifiers)]) -> Option<Self> {
        let mut indices = matching_key_indices(&input.events, keys)?;
        // Printable custom chords also own the text emitted with their keys.
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
        Some(Self {
            before,
            following: input.events[end..].to_vec(),
        })
    }

    pub(crate) fn after_shortcut(input: &InputState, shortcut: KeyboardShortcut) -> Option<Self> {
        let modifiers = input.events.iter().find_map(|event| match event {
            Event::Key {
                key,
                modifiers,
                pressed: true,
                repeat: false,
                ..
            } if *key == shortcut.logical_key
                && modifiers.matches_logically(shortcut.modifiers) =>
            {
                Some(*modifiers)
            }
            _ => None,
        })?;
        Self::after_keys(input, &[(shortcut.logical_key, modifiers)])
    }

    /// A request already accepted by a menu owns the remaining input.
    pub(crate) fn remaining(input: &InputState) -> Self {
        Self {
            before: Vec::new(),
            following: input.events.clone(),
        }
    }

    /// The continuation of an accepted pointer or keyboard button activation.
    pub(crate) fn after_activation(input: &InputState) -> Self {
        let end = input
            .events
            .iter()
            .position(|event| {
                matches!(
                    event,
                    Event::PointerButton {
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        ..
                    } | Event::Key {
                        key: Key::Enter | Key::Space,
                        pressed: true,
                        repeat: false,
                        ..
                    }
                )
            })
            .map_or(0, |index| index + 1);
        Self {
            before: input.events[..end].to_vec(),
            following: input.events[end..].to_vec(),
        }
    }

    /// Render and commit preceding edits before binding the new surface's state.
    pub(crate) fn preceding_scope(&self, ctx: &Context) -> InputScope {
        InputScope::replace(ctx, Some(self.before.clone()))
    }

    pub(crate) fn route(self, ctx: &Context, owner: Id) {
        ctx.input_mut(|input| input.events = self.before);
        ctx.data_mut(|data| data.insert_temp(owner.with("opening-input"), self.following));
    }
}

/// Give one surface its continuation without replaying it into later controls.
pub(crate) struct InputScope {
    ctx: Context,
    previous: Option<Vec<Event>>,
}

impl InputScope {
    pub(crate) fn enter(ctx: &Context, owner: Id) -> Self {
        let following =
            ctx.data_mut(|data| data.remove_temp::<Vec<Event>>(owner.with("opening-input")));
        Self::replace(ctx, following)
    }

    pub(crate) fn discard(ctx: &Context, owner: Id) {
        ctx.data_mut(|data| data.remove_temp::<Vec<Event>>(owner.with("opening-input")));
    }

    fn replace(ctx: &Context, events: Option<Vec<Event>>) -> Self {
        Self {
            ctx: ctx.clone(),
            previous: events
                .map(|events| ctx.input_mut(|input| std::mem::replace(&mut input.events, events))),
        }
    }
}

impl Drop for InputScope {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.take() {
            self.ctx.input_mut(|input| input.events = previous);
        }
    }
}

/// Remove exact presses atomically; egui's logical consumer removes all matches.
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

fn matching_key_indices(events: &[Event], keys: &[(Key, Modifiers)]) -> Option<Vec<usize>> {
    let mut events = events.iter().enumerate();
    let mut indices = Vec::with_capacity(keys.len());
    for (key, modifiers) in keys {
        let (index, _) = events.find(|(_, event)| {
            matches!(event, Event::Key {
                key: event_key, modifiers: event_modifiers, pressed: true, repeat: false, ..
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
