//! The VOLTA modal primitive.
//!
//! Every dialog in the application is the same three-part surface over a
//! scrim:
//!
//! - **Scrim** — full-viewport canvas-black wash that blocks interaction
//!   with everything underneath. Clicking it does not dismiss; `Esc` does.
//! - **Surface** — `bg_panel`, 1 px `border_strong`, large radius, pop
//!   shadow. Three widths (Sm 400 / Md 560 / Lg 780); the body scrolls
//!   once the surface reaches 82 % of the viewport height.
//! - **Header / footer** — mono uppercase kicker + semibold title + close
//!   on top; footer is `[ghost] [secondary] [primary]` with the primary
//!   always rightmost, always exactly one, accent-filled (or `err` when
//!   destructive), plus an optional mono hint on the left.
//!
//! Keys: `Esc` cancels, `Enter` activates the primary when it is enabled.
//! Dialogs edit a draft and commit on the primary — the primary and
//! cancel are never the same operation.

use egui::{
    Context, FocusDirection, Frame, Id, Key, Margin, Modifiers, Order, Popup, Rect, Sense, Stroke,
    Ui, UiKind, WidgetInfo, WidgetType, vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Dialog surface width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogSize {
    /// Confirmations, small forms (400 pt).
    Sm,
    /// Standard forms (560 pt).
    Md,
    /// Multi-pane surfaces — browsers, options (780 pt).
    Lg,
}

impl DialogSize {
    fn width(self) -> f32 {
        match self {
            Self::Sm => 400.0,
            Self::Md => 560.0,
            Self::Lg => 780.0,
        }
    }
}

/// What the user chose this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogChoice {
    /// Still open, nothing chosen.
    None,
    /// The accent (or destructive) primary action.
    Primary,
    /// The plain secondary button, when present.
    Secondary,
    /// The ghost button, when present.
    Ghost,
    /// Dismissed: `Esc` or the ✕ close control.
    Cancelled,
}

#[derive(Debug, Clone, Copy, Default)]
struct DialogFocusState {
    prior_focus: Option<Id>,
    last_seen_pass: u64,
}

/// Declarative description of one modal frame.
pub struct Dialog<'a> {
    kicker: &'a str,
    title: &'a str,
    size: DialogSize,
    primary: &'a str,
    primary_enabled: bool,
    primary_on_enter: bool,
    destructive: bool,
    secondary: Option<&'a str>,
    ghost: Option<&'a str>,
    ghost_enabled: bool,
    hint: Option<&'a str>,
}

impl<'a> Dialog<'a> {
    /// A dialog with the given header kicker (domain, e.g. "Simulate"),
    /// title, and primary-action label.
    pub fn new(kicker: &'a str, title: &'a str, primary: &'a str) -> Self {
        Self {
            kicker,
            title,
            size: DialogSize::Md,
            primary,
            primary_enabled: true,
            primary_on_enter: true,
            destructive: false,
            secondary: None,
            ghost: None,
            ghost_enabled: true,
            hint: None,
        }
    }

    /// Disable the Enter→primary mapping — for dialogs whose body owns the
    /// Enter key (multiline editors, where Enter must insert a newline).
    pub fn primary_on_enter(mut self, on: bool) -> Self {
        self.primary_on_enter = on;
        self
    }

    /// Surface width.
    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    /// Disable the primary action (validation pending). `Enter` is inert
    /// while disabled.
    pub fn primary_enabled(mut self, enabled: bool) -> Self {
        self.primary_enabled = enabled;
        self
    }

    /// Render the primary in the destructive (`err`) treatment.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// Plain secondary button, left of the primary.
    pub fn secondary(mut self, label: &'a str) -> Self {
        self.secondary = Some(label);
        self
    }

    /// Ghost button, leftmost in the footer (Revert, Cancel).
    pub fn ghost(mut self, label: &'a str) -> Self {
        self.ghost = Some(label);
        self
    }

    /// Enable or disable the optional ghost action.
    pub fn ghost_enabled(mut self, enabled: bool) -> Self {
        self.ghost_enabled = enabled;
        self
    }

    /// Mono footer hint (validation count, shortcut reminder).
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Show the dialog and render `body` into the scrollable middle
    /// region. Returns what the user chose this frame; the caller owns
    /// open/close state and reacts to the choice.
    pub fn show(self, ctx: &Context, body: impl FnOnce(&mut Ui)) -> DialogChoice {
        let t = Tokens::get(ctx);
        let c = t.color;
        let screen = ctx.screen_rect();
        let id = Id::new(("volta.dialog", self.title));
        let focus_id = id.with("move");
        let focus_state_id = id.with("focus-state");

        let area = egui::Area::new(id)
            .kind(UiKind::Modal)
            .sense(Sense::focusable_noninteractive())
            .order(Order::Foreground)
            .fixed_pos(screen.min);
        let modal_layer = area.layer();
        let any_popup_open = Popup::is_any_open(ctx);
        let is_top_modal = ctx.memory_mut(|memory| {
            memory.set_modal_layer(modal_layer);
            memory.top_modal_layer() == Some(modal_layer)
        });

        let opened_this_pass = begin_dialog_focus(ctx, focus_state_id);
        if opened_this_pass || !focus_is_within_modal(ctx, modal_layer) {
            // Claim focus before any dialog fields are rendered. A body field
            // may intentionally request focus later in this pass.
            ctx.memory_mut(|memory| {
                memory.request_focus(focus_id);
                if opened_this_pass {
                    // Do not let the Tab direction that triggered opening
                    // carry a stale target from the underlying focus chain.
                    memory.move_focus(FocusDirection::None);
                }
            });
        }

        let mut choice = DialogChoice::None;
        let area_response = area.show(ctx, |ui| {
            // Scrim: swallow pointer interaction with everything below.
            // accessibility-pointer-shim: the scrim consumes pointer
            // gestures but is deliberately absent from keyboard/AT order.
            ui.allocate_rect(
                screen,
                Sense::click_and_drag().difference(Sense::focusable_noninteractive()),
            );
            ui.painter()
                .rect_filled(screen, 0.0, c.canvas_bg.gamma_multiply(0.55));

            let width = self.size.width().min(screen.width() - 32.0);
            let max_height = screen.height() * 0.82;
            // Dialogs sit at a fixed optical position — top edge at 18 %
            // of the viewport — so confirmations and browsers alike open
            // in the same place.
            let top_left = egui::pos2(
                screen.center().x - width * 0.5,
                screen.top() + screen.height() * 0.18,
            );

            let mut surface = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(Rect::from_min_size(top_left, vec2(width, max_height)))
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );
            surface.set_width(width);

            Frame::NONE
                .fill(c.bg_panel)
                .stroke(Stroke::new(1.0, c.border_strong))
                .rounding(t.radius_lg)
                .shadow(t.shadow())
                .show(&mut surface, |ui| {
                    ui.set_width(width);
                    if self.header(ui, &t) {
                        choice = DialogChoice::Cancelled;
                    }
                    egui::ScrollArea::vertical()
                        .id_salt(id.with("body"))
                        .max_height(max_height - 2.0 * 46.0)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            Frame::NONE
                                .inner_margin(Margin::symmetric(16, 14))
                                .show(ui, body);
                        });
                    match self.footer(ui, &t) {
                        DialogChoice::None => {}
                        chosen => choice = chosen,
                    }
                });
        });

        let response = area_response.response;
        let enabled = response.enabled();
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Window, enabled, self.title));
        ctx.accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Dialog);
            node.set_label(self.title);
            node.set_modal();
        });
        if choice == DialogChoice::None && response.should_close() {
            choice = DialogChoice::Cancelled;
        }

        // Handle keys after the contents. This lets a focused secondary,
        // ghost, close, or body button own Enter without also firing the
        // primary action. Single-line text fields retain the canonical
        // Enter-to-submit behavior; multiline owners opt out explicitly with
        // `primary_on_enter(false)`.
        if choice == DialogChoice::None && is_top_modal && !any_popup_open {
            if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
                choice = DialogChoice::Cancelled;
            } else if self.primary_enabled
                && self.primary_on_enter
                && !focused_control_owns_enter(ctx, focus_id)
                && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter))
            {
                choice = DialogChoice::Primary;
            }
        }

        if choice != DialogChoice::None {
            restore_dialog_focus(ctx, focus_state_id, focus_id, modal_layer);
        }

        choice
    }

    /// Header strip; returns `true` when the close control fired.
    fn header(&self, ui: &mut Ui, t: &Tokens) -> bool {
        let c = t.color;
        let mut closed = false;
        Frame::NONE
            .inner_margin(Margin::symmetric(16, 11))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    let mut kicker = egui::text::LayoutJob::default();
                    kicker.append(
                        &self.kicker.to_uppercase(),
                        0.0,
                        egui::TextFormat {
                            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                            color: if self.destructive { c.err } else { c.accent },
                            extra_letter_spacing: 0.14 * tokens::FS_0,
                            ..Default::default()
                        },
                    );
                    ui.label(kicker);
                    ui.label(
                        egui::RichText::new(self.title)
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(c.text),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if crate::ui::widgets::IconButton::new(crate::ui::icons::Icon::Close)
                            .side(24.0)
                            .tooltip("Close (Esc)")
                            .show(ui)
                            .clicked()
                        {
                            closed = true;
                        }
                    });
                });
            });
        let line_y = ui.cursor().top();
        ui.painter().hline(
            ui.max_rect().x_range(),
            line_y,
            Stroke::new(1.0, t.color.border),
        );
        closed
    }

    /// Footer strip with the canonical button order.
    fn footer(&self, ui: &mut Ui, t: &Tokens) -> DialogChoice {
        let c = t.color;
        let mut choice = DialogChoice::None;
        let line_y = ui.cursor().top();
        ui.painter()
            .hline(ui.max_rect().x_range(), line_y, Stroke::new(1.0, c.border));
        Frame::NONE
            .inner_margin(Margin::symmetric(16, 11))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    if let Some(label) = self.ghost
                        && crate::ui::widgets::Button::new(label)
                            .ghost()
                            .enabled(self.ghost_enabled)
                            .show(ui)
                            .clicked()
                    {
                        choice = DialogChoice::Ghost;
                    }
                    if let Some(hint) = self.hint {
                        ui.label(
                            egui::RichText::new(hint)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.text_faint),
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let primary = crate::ui::widgets::Button::new(self.primary)
                            .accent()
                            .destructive(self.destructive)
                            .enabled(self.primary_enabled)
                            .show(ui);
                        if primary.clicked() {
                            choice = DialogChoice::Primary;
                        }
                        if let Some(label) = self.secondary
                            && crate::ui::widgets::Button::new(label).show(ui).clicked()
                        {
                            choice = DialogChoice::Secondary;
                        }
                    });
                });
            });
        choice
    }
}

fn begin_dialog_focus(ctx: &Context, state_id: Id) -> bool {
    let pass = ctx.cumulative_pass_nr();
    let current_focus = ctx.memory(|memory| memory.focused());
    ctx.data_mut(|data| {
        let previous = data.get_temp::<DialogFocusState>(state_id);
        let continuing =
            previous.is_some_and(|state| pass <= state.last_seen_pass.saturating_add(1));
        let state = match previous {
            Some(mut state) if continuing => {
                state.last_seen_pass = pass;
                state
            }
            _ => DialogFocusState {
                prior_focus: current_focus,
                last_seen_pass: pass,
            },
        };
        data.insert_temp(state_id, state);
        !continuing
    })
}

fn focused_control_owns_enter(ctx: &Context, dialog_focus_id: Id) -> bool {
    let Some(focused) = ctx.memory(|memory| memory.focused()) else {
        return false;
    };
    if focused == dialog_focus_id {
        return false;
    }

    // Buttons and button-like controls already synthesize their own click for
    // Enter. Drag-capable fields include egui text edits; a single-line edit
    // deliberately falls through to the dialog's default submit behavior.
    ctx.read_response(focused)
        .is_some_and(|response| response.sense.senses_click() && !response.sense.senses_drag())
}

fn focus_is_within_modal(ctx: &Context, dialog_layer: egui::LayerId) -> bool {
    let Some(focused) = ctx.memory(|memory| memory.focused()) else {
        return false;
    };
    let Some(response) = ctx.read_response(focused) else {
        return false;
    };
    response.layer_id == dialog_layer
        || ctx.memory(|memory| memory.is_above_modal_layer(response.layer_id))
}

fn restore_dialog_focus(
    ctx: &Context,
    state_id: Id,
    dialog_focus_id: Id,
    dialog_layer: egui::LayerId,
) {
    let state = ctx.data_mut(|data| data.remove_temp::<DialogFocusState>(state_id));
    let prior_focus = state.and_then(|state| state.prior_focus);
    let restorable = prior_focus.filter(|prior| {
        *prior != dialog_focus_id
            && ctx.read_response(*prior).is_some_and(|response| {
                response.layer_id != dialog_layer
                    && response.enabled()
                    && response.sense.is_focusable()
            })
    });

    ctx.memory_mut(|memory| {
        // Never leave focus attached to a dialog control that is about to
        // disappear. Invalid or disabled prior targets safely yield no focus.
        if let Some(current) = memory.focused() {
            memory.surrender_focus(current);
        }
        if let Some(prior) = restorable {
            memory.request_focus(prior);
        }
    });
}

/// Center a header strip text baseline helper used by dialog tabs (mono
/// uppercase underline tabs, as in the results docbar).
pub fn dialog_tabs(ui: &mut Ui, tabs: &[&str], active: &mut usize) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (index, label) in tabs.iter().enumerate() {
            let selected = *active == index;
            let mut job = egui::text::LayoutJob::default();
            job.append(
                &label.to_uppercase(),
                0.0,
                egui::TextFormat {
                    font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                    color: egui::Color32::PLACEHOLDER,
                    extra_letter_spacing: 0.06 * tokens::FS_0,
                    ..Default::default()
                },
            );
            let galley = ui.fonts_mut(|f| f.layout_job(job));
            let (rect, response) =
                ui.allocate_exact_size(vec2(galley.size().x + 22.0, 24.0), Sense::click());
            response.widget_info(|| {
                WidgetInfo::labeled(WidgetType::SelectableLabel, ui.is_enabled(), label)
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Tab);
                if selected {
                    node.set_selected(true);
                } else {
                    node.set_selected(false);
                }
            });
            let hover =
                ui.ctx()
                    .animate_bool_with_time(response.id, response.hovered() && !selected, 0.16);
            let color = if selected {
                c.accent
            } else {
                crate::ui::theme::mix(c.text_dim, c.text, hover)
            };
            ui.painter().galley(
                egui::pos2(rect.left() + 11.0, rect.center().y - galley.size().y * 0.5),
                galley,
                color,
            );
            if selected {
                ui.painter().hline(
                    egui::Rangef::new(rect.left() + 6.0, rect.right() - 6.0),
                    rect.bottom() - 1.0,
                    Stroke::new(2.0, c.accent),
                );
            }
            theme::paint_focus_ring(ui, &response, rect);
            if response.clicked() {
                *active = index;
            }
        }
    });
    let y = ui.cursor().top();
    ui.painter()
        .hline(ui.max_rect().x_range(), y, Stroke::new(1.0, t.color.border));
    ui.add_space(10.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TITLE: &str = "Modal behavior";

    fn dialog_id() -> Id {
        Id::new(("volta.dialog", TEST_TITLE))
    }

    fn dialog_focus_id() -> Id {
        dialog_id().with("move")
    }

    fn underlying_id() -> Id {
        Id::new("dialog-test-underlying-editor")
    }

    fn raw_input(events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1_000.0, 800.0))),
            events,
            ..Default::default()
        }
    }

    fn key_event(key: Key, modifiers: Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    fn focus_underlying_editor(ctx: &Context, underlying: &mut String) {
        let _ = ctx.run(raw_input(Vec::new()), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = ui
                    .add(
                        egui::TextEdit::singleline(underlying)
                            .id(underlying_id())
                            .desired_width(240.0),
                    )
                    .request_focus();
            });
        });
    }

    fn run_dialog(
        ctx: &Context,
        input: egui::RawInput,
        underlying: &mut String,
        primary_on_enter: bool,
        mut body: impl FnMut(&mut Ui),
    ) -> (DialogChoice, egui::FullOutput) {
        let mut choice = DialogChoice::None;
        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = ui.add(
                    egui::TextEdit::singleline(underlying)
                        .id(underlying_id())
                        .desired_width(240.0),
                );
            });
            choice = Dialog::new("TEST", TEST_TITLE, "Accept")
                .primary_on_enter(primary_on_enter)
                .show(ctx, |ui| body(ui));
        });
        (choice, output)
    }

    fn assert_focus_is_on_dialog_layer(ctx: &Context) {
        let focused = ctx
            .memory(|memory| memory.focused())
            .expect("dialog should own keyboard focus");
        assert_ne!(focused, underlying_id());
        let response = ctx
            .read_response(focused)
            .expect("focused dialog widget should still exist");
        assert_eq!(
            response.layer_id,
            egui::LayerId::new(Order::Foreground, dialog_id())
        );
    }

    #[test]
    fn dialog_publishes_modal_accessibility_semantics() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut underlying = String::new();

        let (_, output) = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
            let _ = ui.label("Dialog body");
        });

        let nodes = output
            .platform_output
            .accesskit_update
            .expect("AccessKit tree update")
            .nodes;
        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Dialog
                && node.label() == Some(TEST_TITLE)
                && node.is_modal()
        }));
    }

    #[test]
    fn modal_focus_is_trapped_and_restored_without_leaking_text() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut underlying = "baseline".to_owned();
        focus_underlying_editor(&ctx, &mut underlying);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));

        let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
            let _ = ui.button("Body action");
        });
        assert_eq!(
            ctx.memory(|memory| memory.focused()),
            Some(dialog_focus_id())
        );

        let _ = run_dialog(
            &ctx,
            raw_input(vec![egui::Event::Text("blocked".to_owned())]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.button("Body action");
            },
        );
        assert_eq!(underlying, "baseline");

        let _ = run_dialog(
            &ctx,
            raw_input(vec![key_event(Key::Tab, Modifiers::NONE)]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.button("Body action");
            },
        );
        assert_focus_is_on_dialog_layer(&ctx);

        let _ = run_dialog(
            &ctx,
            raw_input(vec![key_event(
                Key::Tab,
                Modifiers {
                    shift: true,
                    ..Modifiers::NONE
                },
            )]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.button("Body action");
            },
        );
        assert_focus_is_on_dialog_layer(&ctx);

        let (choice, _) = run_dialog(
            &ctx,
            raw_input(vec![key_event(Key::Escape, Modifiers::NONE)]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.button("Body action");
            },
        );
        assert_eq!(choice, DialogChoice::Cancelled);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));
    }

    #[test]
    fn focused_body_button_owns_enter_instead_of_primary() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut underlying = String::new();
        let mut activated = false;

        let _ = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
            ui.button("Body action").request_focus();
        });
        let (choice, _) = run_dialog(
            &ctx,
            raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
            &mut underlying,
            true,
            |ui| {
                let response = ui.button("Body action");
                if response.clicked() {
                    activated = true;
                }
                response.request_focus();
            },
        );

        assert!(activated);
        assert_eq!(choice, DialogChoice::None);
    }

    #[test]
    fn body_close_request_cancels_and_restores_prior_focus() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut underlying = String::new();
        focus_underlying_editor(&ctx, &mut underlying);

        let (choice, _) = run_dialog(&ctx, raw_input(Vec::new()), &mut underlying, true, |ui| {
            ui.close_kind(UiKind::Modal)
        });

        assert_eq!(choice, DialogChoice::Cancelled);
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(underlying_id()));
    }

    #[test]
    fn enter_submits_by_default_but_multiline_opt_out_keeps_newlines() {
        let submit_ctx = Context::default();
        crate::ui::Theme::default().apply(&submit_ctx);
        let mut underlying = String::new();
        let _ = run_dialog(
            &submit_ctx,
            raw_input(Vec::new()),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.label("Ready");
            },
        );
        let (choice, _) = run_dialog(
            &submit_ctx,
            raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
            &mut underlying,
            true,
            |ui| {
                let _ = ui.label("Ready");
            },
        );
        assert_eq!(choice, DialogChoice::Primary);
        assert_eq!(submit_ctx.memory(|memory| memory.focused()), None);

        let multiline_ctx = Context::default();
        crate::ui::Theme::default().apply(&multiline_ctx);
        let mut underlying = String::new();
        let mut body_text = String::new();
        let _ = run_dialog(
            &multiline_ctx,
            raw_input(Vec::new()),
            &mut underlying,
            false,
            |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut body_text).id(Id::new("dialog-test-multiline")),
                )
                .request_focus();
            },
        );
        let (choice, _) = run_dialog(
            &multiline_ctx,
            raw_input(vec![key_event(Key::Enter, Modifiers::NONE)]),
            &mut underlying,
            false,
            |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut body_text).id(Id::new("dialog-test-multiline")),
                )
                .request_focus();
            },
        );

        assert_eq!(choice, DialogChoice::None);
        assert_eq!(body_text, "\n");
    }
}
