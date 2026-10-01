//! Preserve the edit/cancel boundary before egui clears keyboard focus.
//!
//! Only registered, enabled modal dialogs intercept Escape. Popup dismissal
//! remains with egui. Input after cancellation is processed in a subsequent
//! pass, after the caller has applied its close or discard-confirmation policy.

use std::collections::BTreeMap;

use egui::{Context, Event, Id, Key, LayerId, Modifiers, Popup, RawInput, ViewportId};

#[derive(Clone, Copy)]
struct Registration {
    pass: u64,
    enabled: bool,
    open: bool,
}

#[derive(Default)]
struct DialogKeyboard {
    deferred: BTreeMap<ViewportId, Vec<Event>>,
    boundary: BTreeMap<ViewportId, (LayerId, Vec<Event>)>,
    resume_focus: BTreeMap<ViewportId, Id>,
}

fn registration_id(layer: LayerId) -> Id {
    layer.id.with("dialog-keyboard")
}

pub(super) fn register(ctx: &Context, layer: LayerId, enabled: bool) {
    ctx.plugin_or_default::<DialogKeyboard>();
    let registration = Registration {
        pass: ctx.cumulative_pass_nr(),
        enabled,
        open: true,
    };
    ctx.data_mut(|data| data.insert_temp(registration_id(layer), registration));
}

pub(super) fn unregister(ctx: &Context, layer: LayerId) {
    ctx.data_mut(|data| {
        if let Some(mut registration) = data.get_temp::<Registration>(registration_id(layer)) {
            registration.open = false;
            data.insert_temp(registration_id(layer), registration);
        }
    });
}

pub(super) fn resume_focus(ctx: &Context, target: Id) {
    ctx.plugin_or_default::<DialogKeyboard>()
        .lock()
        .resume_focus
        .insert(ctx.viewport_id(), target);
    ctx.request_repaint();
}

fn closing_modal(ctx: &Context) -> bool {
    ctx.memory(|memory| {
        memory.top_modal_layer().is_some_and(|layer| {
            memory
                .data
                .get_temp::<Registration>(registration_id(layer))
                .is_some_and(|registration| !registration.open)
        })
    })
}

/// A newly opened area is ordered at the end of its first pass. egui's modal
/// cache can still name its parent, so use the completed area order as well.
pub(super) fn top_layer(ctx: &Context) -> Option<LayerId> {
    let pass = ctx.cumulative_pass_nr();
    ctx.memory(|memory| {
        let modal = memory.top_modal_layer();
        let native_modal = modal.filter(|layer| {
            memory
                .data
                .get_temp::<Registration>(registration_id(*layer))
                .is_none()
        });
        let mut above_native_modal = native_modal.is_none();
        memory
            .layer_ids()
            .filter(|layer| {
                above_native_modal |= native_modal == Some(*layer);
                above_native_modal
                    && memory
                        .data
                        .get_temp::<Registration>(registration_id(*layer))
                        .is_some_and(|registration| {
                            registration.open && pass <= registration.pass.saturating_add(1)
                        })
            })
            .last()
    })
}

/// Consume only this dialog's intercepted cancellation. An earlier accepted
/// action leaves Escape queued too, so it belongs to the resulting surface.
pub(super) fn cancel(ctx: &Context, layer: LayerId, accepted: bool) -> bool {
    let handle = ctx.plugin_or_default::<DialogKeyboard>();
    let mut keyboard = handle.lock();
    let viewport = ctx.viewport_id();
    if !keyboard
        .boundary
        .get(&viewport)
        .is_some_and(|(owner, _)| *owner == layer)
    {
        return false;
    }
    let (_, mut events) = keyboard.boundary.remove(&viewport).unwrap();
    if accepted {
        events.remove(0);
    }
    if !events.is_empty() {
        keyboard.deferred.insert(viewport, events);
        ctx.request_repaint();
    }
    accepted
}

impl egui::Plugin for DialogKeyboard {
    fn debug_name(&self) -> &'static str {
        "RSpice dialog cancellation"
    }

    fn input_hook(&mut self, ctx: &Context, input: &mut RawInput) {
        // If a dialog disappeared without rendering, preserve the unhandled
        // boundary for the current surface instead of leaving stale input.
        let unhandled = self
            .boundary
            .remove(&input.viewport_id)
            .map(|(_, events)| events);
        let continuation = self
            .deferred
            .remove(&input.viewport_id)
            .or(unhandled)
            .or_else(|| {
                self.resume_focus
                    .contains_key(&input.viewport_id)
                    .then(Vec::new)
            });
        if let Some(mut events) = continuation {
            events.append(&mut input.events);
            // The old modal still owns egui's hit-test cache for one pass.
            // Let that cache retire before delivering pointer or keyboard
            // continuation to the restored surface.
            if ctx.viewport_id() == input.viewport_id && closing_modal(ctx) {
                self.deferred.insert(input.viewport_id, events);
                ctx.request_repaint();
                return;
            }
            input.events = events;
        }

        // egui's Memory access refers to the currently selected viewport.
        // A different viewport must first render its own modal registration.
        if ctx.viewport_id() != input.viewport_id || Popup::is_any_open(ctx) {
            return;
        }
        let Some(layer) = top_layer(ctx) else {
            return;
        };
        let registration = ctx.data(|data| data.get_temp::<Registration>(registration_id(layer)));
        if !registration.is_some_and(|registration| registration.enabled) {
            return;
        }
        let Some(index) = input.events.iter().position(|event| {
            matches!(event, Event::Key {
                key: Key::Escape, pressed: true, repeat: false, modifiers, ..
            } if *modifiers == Modifiers::NONE)
        }) else {
            return;
        };
        let following = input.events.split_off(index);
        self.boundary.insert(input.viewport_id, (layer, following));
        ctx.request_repaint();
    }

    fn on_begin_pass(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx();
        if closing_modal(ctx) {
            return;
        }
        let Some(target) = self.resume_focus.remove(&ctx.viewport_id()) else {
            return;
        };
        // The stale modal pass can surrender the restored editor's focus.
        // Revalidate against the most recently rendered widgets once egui
        // retires that layer; never override a new owner or revive a removed
        // or disabled field.
        if ctx.memory(|memory| memory.focused().is_none())
            && ctx.read_response(target).is_some_and(|response| {
                response.enabled()
                    && response.sense.is_focusable()
                    && ctx.memory(|memory| memory.allows_interaction(response.layer_id))
            })
        {
            ctx.memory_mut(|memory| memory.request_focus(target));
        }
    }
}
