//! Toast notifications — transient confirmations anchored to the
//! bottom-right corner, above the status bar.

use egui::{Align2, Area, Context, Frame, Id, Order, Stroke, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Visual severity of a toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastKind {
    /// Accent-edged informational toast.
    #[default]
    Info,
    /// Warning-edged toast.
    Warn,
    /// Error-edged toast.
    Error,
}

/// One queued toast.
#[derive(Debug, Clone)]
pub struct Toast {
    message: String,
    kind: ToastKind,
    /// Absolute time (egui clock) at which the toast was created.
    created: f64,
}

/// Lifetime of a toast in seconds (matches the design prototype).
const TOAST_LIFETIME: f64 = 3.2;
/// Fade-out duration after the lifetime elapses.
const TOAST_FADE: f64 = 0.35;

/// The application toast queue. Hold one instance in UI session state and call
/// [`Toasts::show`] once per frame.
#[derive(Debug, Clone, Default)]
pub struct Toasts {
    queue: Vec<Toast>,
}

impl Toasts {
    /// Queue an informational toast.
    pub fn info(&mut self, ctx: &Context, message: impl Into<String>) {
        self.push(ctx, message, ToastKind::Info);
    }

    /// Queue a warning toast.
    pub fn warn(&mut self, ctx: &Context, message: impl Into<String>) {
        self.push(ctx, message, ToastKind::Warn);
    }

    /// Queue an error toast.
    pub fn error(&mut self, ctx: &Context, message: impl Into<String>) {
        self.push(ctx, message, ToastKind::Error);
    }

    fn push(&mut self, ctx: &Context, message: impl Into<String>, kind: ToastKind) {
        self.queue.push(Toast {
            message: message.into(),
            kind,
            created: ctx.input(|i| i.time),
        });
    }

    /// Render all live toasts and drop expired ones.
    pub fn show(&mut self, ctx: &Context) {
        if self.queue.is_empty() {
            return;
        }
        let now = ctx.input(|i| i.time);
        self.queue
            .retain(|toast| now - toast.created < TOAST_LIFETIME + TOAST_FADE);
        if self.queue.is_empty() {
            return;
        }
        // Animate fades.
        ctx.request_repaint();

        let t = Tokens::get(ctx);
        let c = t.color;

        Area::new(Id::new("volta.toasts"))
            .order(Order::Foreground)
            .anchor(Align2::RIGHT_BOTTOM, vec2(-16.0, -38.0))
            .interactable(false)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 8.0;
                for toast in &self.queue {
                    let age = now - toast.created;
                    let opacity = if age > TOAST_LIFETIME {
                        (1.0 - (age - TOAST_LIFETIME) / TOAST_FADE).max(0.0) as f32
                    } else {
                        // Quick slide-in fade.
                        ((age / 0.22).min(1.0)) as f32
                    };
                    let edge = match toast.kind {
                        ToastKind::Info => c.accent,
                        ToastKind::Warn => c.warn,
                        ToastKind::Error => c.err,
                    };
                    Frame::NONE
                        .fill(c.bg_elevated.gamma_multiply(opacity))
                        .stroke(Stroke::new(1.0, c.border_strong.gamma_multiply(opacity)))
                        .rounding(t.radius_lg)
                        .shadow(t.shadow())
                        .inner_margin(egui::Margin {
                            left: 13,
                            right: 13,
                            top: 9,
                            bottom: 9,
                        })
                        .show(ui, |ui| {
                            ui.set_max_width(360.0);
                            // Accent edge strip.
                            let rect = ui.max_rect().expand2(vec2(13.0, 9.0));
                            ui.painter().rect_filled(
                                egui::Rect::from_min_max(
                                    rect.left_top(),
                                    egui::pos2(rect.left() + 3.0, rect.bottom()),
                                ),
                                0.0,
                                edge.gamma_multiply(opacity),
                            );
                            ui.label(
                                egui::RichText::new(&toast.message)
                                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                    .color(c.text.gamma_multiply(opacity)),
                            );
                        });
                }
            });
    }
}
