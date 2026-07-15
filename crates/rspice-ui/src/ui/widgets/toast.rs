//! Toast notifications — transient confirmations anchored to the
//! bottom-right corner, above the status bar.

use std::collections::HashSet;

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

/// Activity domains defined by the mockup notification-center contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationCategory {
    Job,
    Approval,
    #[default]
    System,
}

impl NotificationCategory {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Job => "Job",
            Self::Approval => "Approval",
            Self::System => "System",
        }
    }
}

impl ToastKind {
    /// Human-facing category used by the retained activity center.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Info => "Information",
            Self::Warn => "Warning",
            Self::Error => "Error",
        }
    }
}

/// One queued toast.
#[derive(Debug, Clone)]
pub struct Toast {
    message: String,
    kind: ToastKind,
    /// Absolute time (egui clock) at which the toast was created.
    created: f64,
}

/// One retained session activity item. Toasts disappear after a few seconds,
/// while this record remains available from the mockup-specified notification
/// center until the user clears it or the application session ends.
#[derive(Debug, Clone, PartialEq)]
pub struct NotificationRecord {
    id: u64,
    message: String,
    category: NotificationCategory,
    kind: ToastKind,
    created: f64,
    read: bool,
}

impl NotificationRecord {
    pub const fn id(&self) -> u64 {
        self.id
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn category(&self) -> NotificationCategory {
        self.category
    }

    pub const fn kind(&self) -> ToastKind {
        self.kind
    }

    pub const fn created(&self) -> f64 {
        self.created
    }

    pub const fn is_read(&self) -> bool {
        self.read
    }
}

/// Lifetime of a toast in seconds (matches the design prototype).
const TOAST_LIFETIME: f64 = 3.2;
/// Fade-out duration after the lifetime elapses.
const TOAST_FADE: f64 = 0.35;
/// The mockup shows at most three transient notices at once.
const MAX_VISIBLE_TOASTS: usize = 3;
/// Retained activity is intentionally larger than the transient stack.
const MAX_RETAINED_ACTIVITY: usize = 50;

/// The application toast queue. Hold one instance in UI session state and call
/// [`Toasts::show`] once per frame.
#[derive(Debug, Clone, Default)]
pub struct Toasts {
    queue: Vec<Toast>,
    activity: Vec<NotificationRecord>,
    next_notification_id: u64,
    observed_log_revision: u64,
}

impl Toasts {
    /// Queue an informational toast.
    pub fn info(&mut self, ctx: &Context, message: impl Into<String>) {
        self.notify(ctx, NotificationCategory::System, ToastKind::Info, message);
    }

    /// Queue a warning toast.
    pub fn warn(&mut self, ctx: &Context, message: impl Into<String>) {
        self.notify(ctx, NotificationCategory::System, ToastKind::Warn, message);
    }

    /// Queue an error toast.
    pub fn error(&mut self, ctx: &Context, message: impl Into<String>) {
        self.notify(ctx, NotificationCategory::System, ToastKind::Error, message);
    }

    /// Queue a transient notice and retain it in its mockup-defined activity
    /// domain. Approval producers must call this only after a real approval
    /// request or decision exists; the UI never manufactures such records.
    pub fn notify(
        &mut self,
        ctx: &Context,
        category: NotificationCategory,
        kind: ToastKind,
        message: impl Into<String>,
    ) {
        let message = message.into();
        let created = ctx.input(|i| i.time);
        self.queue.push(Toast {
            message: message.clone(),
            kind,
            created,
        });
        if self.queue.len() > MAX_VISIBLE_TOASTS {
            self.queue.drain(..self.queue.len() - MAX_VISIBLE_TOASTS);
        }
        self.record_activity(message, category, kind, created);
    }

    fn record_activity(
        &mut self,
        message: String,
        category: NotificationCategory,
        kind: ToastKind,
        created: f64,
    ) {
        if self.next_notification_id == u64::MAX {
            // Reaching this requires more than eighteen quintillion notices.
            // Retire the bounded session history before recycling identities
            // so two live records can never share an ID.
            self.activity.clear();
            self.next_notification_id = 0;
        }
        self.next_notification_id += 1;
        self.activity.insert(
            0,
            NotificationRecord {
                id: self.next_notification_id,
                message,
                category,
                kind,
                created,
                read: false,
            },
        );
        self.activity.truncate(MAX_RETAINED_ACTIVITY);
    }

    /// Mirror new structured-console entries into retained activity without
    /// replaying older records on subsequent frames. Every item carries its
    /// stable log identity and original event time on the egui clock. The
    /// caller supplies only display-safe fields, keeping this widget
    /// independent of the log model.
    pub fn synchronize_activity(
        &mut self,
        log_revision: u64,
        entries: impl IntoIterator<Item = (u64, NotificationCategory, ToastKind, String, f64)>,
    ) {
        let first_unobserved = self.observed_log_revision;
        let mut accepted_ids = HashSet::new();
        for (id, category, kind, message, created) in entries {
            if id >= first_unobserved && id < log_revision && accepted_ids.insert(id) {
                self.record_activity(message, category, kind, created);
            }
        }
        self.observed_log_revision = self.observed_log_revision.max(log_revision);
    }

    /// Newest-first session activity shown by the notification center.
    pub fn activity(&self) -> &[NotificationRecord] {
        &self.activity
    }

    pub fn unread_count(&self) -> usize {
        self.activity.iter().filter(|item| !item.read).count()
    }

    /// Next structured-log identity already consumed by the activity stream.
    pub const fn observed_log_revision(&self) -> u64 {
        self.observed_log_revision
    }

    pub fn mark_read(&mut self, id: u64) -> bool {
        let Some(item) = self.activity.iter_mut().find(|item| item.id == id) else {
            return false;
        };
        item.read = true;
        true
    }

    pub fn mark_all_read(&mut self) {
        for item in &mut self.activity {
            item.read = true;
        }
    }

    pub fn clear_read(&mut self) {
        self.activity.retain(|item| !item.read);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_is_newest_first_and_tracks_read_state() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        toasts.info(&ctx, "first");
        toasts.warn(&ctx, "second");

        assert_eq!(toasts.unread_count(), 2);
        assert_eq!(toasts.activity()[0].message(), "second");
        assert_eq!(toasts.activity()[0].kind(), ToastKind::Warn);
        assert!(toasts.mark_read(toasts.activity()[0].id()));
        assert_eq!(toasts.unread_count(), 1);
        toasts.clear_read();
        assert_eq!(toasts.activity().len(), 1);
        assert_eq!(toasts.activity()[0].message(), "first");
    }

    #[test]
    fn activity_retains_fifty_items_while_transient_stack_keeps_exactly_three() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        for index in 0..(MAX_RETAINED_ACTIVITY + 7) {
            toasts.info(&ctx, format!("notice {index}"));
        }

        assert_eq!(toasts.activity().len(), MAX_RETAINED_ACTIVITY);
        assert_eq!(toasts.queue.len(), MAX_VISIBLE_TOASTS);
        assert_eq!(toasts.queue[0].message, "notice 54");
        assert_eq!(toasts.queue[2].message, "notice 56");
        assert_eq!(toasts.activity()[0].message(), "notice 56");
        toasts.mark_all_read();
        assert_eq!(toasts.unread_count(), 0);
    }

    #[test]
    fn structured_activity_uses_stable_ids_without_dropping_real_duplicates() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        toasts.warn(&ctx, "save conflict");
        toasts.warn(&ctx, "save conflict");
        assert_eq!(toasts.activity().len(), 2);

        toasts.synchronize_activity(
            2,
            [
                (
                    0,
                    NotificationCategory::System,
                    ToastKind::Warn,
                    "save conflict".to_owned(),
                    4.0,
                ),
                (
                    1,
                    NotificationCategory::System,
                    ToastKind::Info,
                    "project opened".to_owned(),
                    7.0,
                ),
            ],
        );
        assert_eq!(toasts.activity().len(), 4);
        assert_eq!(toasts.activity()[0].created(), 7.0);
        assert_eq!(toasts.activity()[1].created(), 4.0);

        toasts.synchronize_activity(
            2,
            [
                (
                    0,
                    NotificationCategory::System,
                    ToastKind::Warn,
                    "save conflict".to_owned(),
                    9.0,
                ),
                (
                    1,
                    NotificationCategory::System,
                    ToastKind::Info,
                    "project opened".to_owned(),
                    9.0,
                ),
            ],
        );
        assert_eq!(toasts.activity().len(), 4);
    }

    #[test]
    fn duplicate_structured_ids_in_one_revision_are_ignored() {
        let mut toasts = Toasts::default();
        toasts.synchronize_activity(
            1,
            [
                (
                    0,
                    NotificationCategory::Job,
                    ToastKind::Info,
                    "run complete".to_owned(),
                    1.0,
                ),
                (
                    0,
                    NotificationCategory::Job,
                    ToastKind::Info,
                    "run complete".to_owned(),
                    1.0,
                ),
            ],
        );

        assert_eq!(toasts.activity().len(), 1);
    }
}
