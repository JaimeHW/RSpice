//! Toast notifications — transient confirmations anchored to the
//! bottom-right corner, above the status bar.

use std::collections::HashSet;
use std::time::Duration;

use egui::{
    Align2, Area, Context, Frame, Id, Margin, Order, Rect, Sense, Shape, Stroke, Ui, Vec2, pos2,
    vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Visual severity of a toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastKind {
    /// Green success confirmation used by the mockup's normal action feedback.
    Success,
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
            Self::Success => "Success",
            Self::Info => "Information",
            Self::Warn => "Warning",
            Self::Error => "Error",
        }
    }
}

/// One queued toast.
#[derive(Debug, Clone)]
pub struct Toast {
    id: u64,
    title: String,
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
    title: String,
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

    pub fn title(&self) -> &str {
        &self.title
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

/// Lifetime of a toast in seconds (matches the reviewed workbench mockup).
const TOAST_LIFETIME: f64 = 5.2;
/// Mockup entrance animation duration.
const TOAST_ENTER: f64 = 0.18;
/// The mockup shows at most three transient notices at once.
const MAX_VISIBLE_TOASTS: usize = 3;
/// Retained activity is intentionally larger than the transient stack.
const MAX_RETAINED_ACTIVITY: usize = 50;
const TOAST_DESKTOP_WIDTH: f32 = 350.0;
const TOAST_DESKTOP_INSET: f32 = 14.0;
const TOAST_PHONE_INSET: f32 = 8.0;
const TOAST_PHONE_BREAKPOINT: f32 = 560.0;
const TOAST_GAP: f32 = 7.0;
const TOAST_MIN_HEIGHT: f32 = 49.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ToastLayout {
    top: f32,
    right: f32,
    width: f32,
}

impl ToastLayout {
    fn resolve(viewport_width: f32, title_bar_height: f32, toolbar_height: f32) -> Self {
        if viewport_width <= TOAST_PHONE_BREAKPOINT {
            Self {
                // The final mobile mockup rule deliberately omits the title
                // bar from the toast offset.
                top: toolbar_height + 8.0,
                right: TOAST_PHONE_INSET,
                width: (viewport_width - TOAST_PHONE_INSET * 2.0).max(0.0),
            }
        } else {
            Self {
                top: title_bar_height + toolbar_height + 12.0,
                right: TOAST_DESKTOP_INSET,
                width: TOAST_DESKTOP_WIDTH
                    .min((viewport_width - TOAST_DESKTOP_INSET * 2.0).max(0.0)),
            }
        }
    }
}

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
    /// Queue a successful action confirmation with the mockup's title/detail
    /// hierarchy.
    pub fn success(&mut self, ctx: &Context, title: impl Into<String>, message: impl Into<String>) {
        self.notify_with_title(
            ctx,
            NotificationCategory::System,
            ToastKind::Success,
            title,
            message,
        );
    }

    /// Queue an informational toast.
    pub fn info(&mut self, ctx: &Context, message: impl Into<String>) {
        self.notify(ctx, NotificationCategory::System, ToastKind::Info, message);
    }

    /// Queue a titled informational toast.
    pub fn info_with_title(
        &mut self,
        ctx: &Context,
        title: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.notify_with_title(
            ctx,
            NotificationCategory::System,
            ToastKind::Info,
            title,
            message,
        );
    }

    /// Queue a warning toast.
    pub fn warn(&mut self, ctx: &Context, message: impl Into<String>) {
        self.notify(ctx, NotificationCategory::System, ToastKind::Warn, message);
    }

    /// Queue a titled warning toast.
    pub fn warn_with_title(
        &mut self,
        ctx: &Context,
        title: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.notify_with_title(
            ctx,
            NotificationCategory::System,
            ToastKind::Warn,
            title,
            message,
        );
    }

    /// Queue an error toast.
    pub fn error(&mut self, ctx: &Context, message: impl Into<String>) {
        self.notify(ctx, NotificationCategory::System, ToastKind::Error, message);
    }

    /// Queue a titled error toast.
    pub fn error_with_title(
        &mut self,
        ctx: &Context,
        title: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.notify_with_title(
            ctx,
            NotificationCategory::System,
            ToastKind::Error,
            title,
            message,
        );
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
        self.enqueue(ctx, category, kind, kind.label(), message, false);
    }

    /// Queue a titled transient notice and retain the same title/detail pair
    /// in session activity. A repeated tone/title replaces its visible toast,
    /// matching the mockup while preserving both historical activity records.
    pub fn notify_with_title(
        &mut self,
        ctx: &Context,
        category: NotificationCategory,
        kind: ToastKind,
        title: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.enqueue(ctx, category, kind, title, message, true);
    }

    fn enqueue(
        &mut self,
        ctx: &Context,
        category: NotificationCategory,
        kind: ToastKind,
        title: impl Into<String>,
        message: impl Into<String>,
        deduplicate_visible_title: bool,
    ) {
        let title = title.into();
        let message = message.into();
        let created = ctx.input(|i| i.time);
        let id = self.record_activity(title.clone(), message.clone(), category, kind, created);
        if deduplicate_visible_title {
            self.queue
                .retain(|toast| toast.kind != kind || toast.title != title);
        }
        self.queue.push(Toast {
            id,
            title,
            message,
            kind,
            created,
        });
        if self.queue.len() > MAX_VISIBLE_TOASTS {
            self.queue.drain(..self.queue.len() - MAX_VISIBLE_TOASTS);
        }
    }

    fn record_activity(
        &mut self,
        title: String,
        message: String,
        category: NotificationCategory,
        kind: ToastKind,
        created: f64,
    ) -> u64 {
        if self.next_notification_id == u64::MAX {
            // Reaching this requires more than eighteen quintillion notices.
            // Retire the bounded session history before recycling identities
            // so two live records can never share an ID.
            self.activity.clear();
            self.next_notification_id = 0;
        }
        self.next_notification_id += 1;
        let id = self.next_notification_id;
        self.activity.insert(
            0,
            NotificationRecord {
                id,
                title,
                message,
                category,
                kind,
                created,
                read: false,
            },
        );
        self.activity.truncate(MAX_RETAINED_ACTIVITY);
        id
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
                self.record_activity(kind.label().to_owned(), message, category, kind, created);
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

    /// Render all live toasts and drop expired ones. Chrome metrics are passed
    /// explicitly so this reusable widget does not depend on the workbench
    /// layout module.
    pub fn show(&mut self, ctx: &Context, title_bar_height: f32, toolbar_height: f32) {
        if self.queue.is_empty() {
            return;
        }
        let now = ctx.input(|i| i.time);
        self.queue
            .retain(|toast| now - toast.created < TOAST_LIFETIME);
        if self.queue.is_empty() {
            return;
        }

        let soonest_expiry = self
            .queue
            .iter()
            .map(|toast| (toast.created + TOAST_LIFETIME - now).max(0.0))
            .fold(f64::INFINITY, f64::min);
        if soonest_expiry.is_finite() {
            ctx.request_repaint_after(Duration::from_secs_f64(soonest_expiry));
        }
        if self
            .queue
            .iter()
            .any(|toast| now - toast.created < TOAST_ENTER)
        {
            ctx.request_repaint();
        }

        let t = Tokens::get(ctx);
        let c = t.color;
        let viewport_width = ctx.content_rect().width();
        let layout = ToastLayout::resolve(viewport_width, title_bar_height, toolbar_height);
        if layout.width <= 0.0 {
            return;
        }
        let mut dismissed = None;

        Area::new(Id::new("rspice.toasts"))
            .order(Order::Foreground)
            .anchor(Align2::RIGHT_TOP, vec2(-layout.right, layout.top))
            .default_width(layout.width)
            .interactable(true)
            .show(ctx, |ui| {
                ui.set_min_width(layout.width);
                ui.set_max_width(layout.width);
                ui.spacing_mut().item_spacing.y = TOAST_GAP;
                for toast in &self.queue {
                    let age = now - toast.created;
                    let opacity = (age / TOAST_ENTER).clamp(0.0, 1.0) as f32;
                    let edge = match toast.kind {
                        ToastKind::Success => c.ok,
                        ToastKind::Info => c.accent,
                        ToastKind::Warn => c.warn,
                        ToastKind::Error => c.err,
                    };
                    let mut shadow = t.shadow();
                    shadow.color = shadow.color.gamma_multiply(opacity);
                    let frame = Frame::NONE
                        .fill(c.bg_elevated.gamma_multiply(opacity))
                        .stroke(Stroke::new(1.0, c.border_strong.gamma_multiply(opacity)))
                        .corner_radius(t.radius_lg)
                        .shadow(shadow)
                        .inner_margin(Margin::symmetric(10, 9))
                        .show(ui, |ui| {
                            ui.set_min_height(TOAST_MIN_HEIGHT - 18.0 - 2.0);
                            if toast_contents(ui, toast, edge, opacity) {
                                dismissed = Some(toast.id);
                            }
                        });
                    let edge_rect = frame.response.rect;
                    ui.painter().line_segment(
                        [
                            pos2(edge_rect.left() + 1.0, edge_rect.top() + t.radius_lg * 0.5),
                            pos2(
                                edge_rect.left() + 1.0,
                                edge_rect.bottom() - t.radius_lg * 0.5,
                            ),
                        ],
                        Stroke::new(2.0, edge.gamma_multiply(opacity)),
                    );
                    let toast_response = ui.interact(
                        frame.response.rect,
                        ui.id().with(("toast-status", toast.id)),
                        Sense::hover(),
                    );
                    ui.ctx().accesskit_node_builder(toast_response.id, |node| {
                        node.set_role(egui::accesskit::Role::Status);
                        node.set_label(format!("{}: {}", toast.title, toast.message));
                    });
                }
            });

        if let Some(id) = dismissed {
            self.queue.retain(|toast| toast.id != id);
        }
    }
}

fn toast_contents(ui: &mut Ui, toast: &Toast, color: egui::Color32, opacity: f32) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut dismissed = false;
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(19.0), Sense::hover());
        paint_toast_icon(ui, icon_rect, toast.kind, color.gamma_multiply(opacity));

        let content_width = (ui.available_width() - 8.0 * 2.0 - 19.0).max(0.0);
        ui.vertical(|ui| {
            ui.set_min_width(content_width);
            ui.set_max_width(content_width);
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(
                egui::RichText::new(&toast.title)
                    .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text.gamma_multiply(opacity)),
            );
            ui.label(
                egui::RichText::new(&toast.message)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim.gamma_multiply(opacity)),
            );
        });

        let (close_rect, close_response) =
            ui.allocate_exact_size(Vec2::splat(19.0), Sense::click());
        close_response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                "Dismiss notification",
            )
        });
        let close_color = if close_response.hovered() || close_response.has_focus() {
            t.color.text
        } else {
            t.color.text_faint
        };
        paint_close_icon(ui, close_rect, close_color.gamma_multiply(opacity));
        theme::paint_focus_ring(ui, &close_response, close_rect);
        dismissed = close_response.clicked();
        close_response.on_hover_text("Dismiss notification");
    });
    dismissed
}

fn paint_toast_icon(ui: &Ui, rect: Rect, kind: ToastKind, color: egui::Color32) {
    let side = rect.width().min(rect.height());
    let scale = side / 24.0;
    let origin = rect.center() - Vec2::splat(side * 0.5);
    let point = |x: f32, y: f32| pos2(origin.x + x * scale, origin.y + y * scale);
    let stroke = Stroke::new((1.6 * scale).max(1.0), color);
    let line = |points: &[(f32, f32)]| {
        ui.painter().add(Shape::line(
            points.iter().map(|&(x, y)| point(x, y)).collect(),
            stroke,
        ));
    };

    match kind {
        ToastKind::Success => line(&[(5.0, 12.0), (10.0, 17.0), (20.0, 6.0)]),
        ToastKind::Info => {
            ui.painter()
                .circle_stroke(point(12.0, 12.0), 8.0 * scale, stroke);
            ui.painter()
                .circle_filled(point(12.0, 7.5), 1.0 * scale, color);
            line(&[(12.0, 11.0), (12.0, 17.0)]);
        }
        ToastKind::Warn => {
            ui.painter().add(Shape::closed_line(
                [point(12.0, 3.0), point(22.0, 20.0), point(2.0, 20.0)].into(),
                stroke,
            ));
            line(&[(12.0, 8.0), (12.0, 14.0)]);
            ui.painter()
                .circle_filled(point(12.0, 17.0), 1.0 * scale, color);
        }
        ToastKind::Error => {
            ui.painter()
                .circle_stroke(point(12.0, 12.0), 8.0 * scale, stroke);
            line(&[(8.0, 8.0), (16.0, 16.0)]);
            line(&[(16.0, 8.0), (8.0, 16.0)]);
        }
    }
}

fn paint_close_icon(ui: &Ui, rect: Rect, color: egui::Color32) {
    let icon = Rect::from_center_size(rect.center(), Vec2::splat(14.0));
    let stroke = Stroke::new(1.25, color);
    ui.painter()
        .line_segment([icon.left_top(), icon.right_bottom()], stroke);
    ui.painter()
        .line_segment([icon.right_top(), icon.left_bottom()], stroke);
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
        assert_eq!(toasts.activity()[0].title(), "Warning");
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

    #[test]
    fn titled_toasts_replace_only_the_matching_visible_tone_and_title() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        toasts.success(&ctx, "Project saved", "Revision 12 is durable.");
        toasts.success(&ctx, "Project saved", "Revision 13 is durable.");
        toasts.warn_with_title(&ctx, "Project saved", "Remote sync is pending.");

        assert_eq!(toasts.queue.len(), 2);
        assert_eq!(toasts.queue[0].kind, ToastKind::Success);
        assert_eq!(toasts.queue[0].message, "Revision 13 is durable.");
        assert_eq!(toasts.queue[1].kind, ToastKind::Warn);
        assert_eq!(toasts.activity().len(), 3);
        assert_eq!(toasts.activity()[0].title(), "Project saved");
    }

    #[test]
    fn placement_matches_final_desktop_and_phone_mockup_rules() {
        assert_eq!(
            ToastLayout::resolve(1440.0, 35.0, 45.0),
            ToastLayout {
                top: 92.0,
                right: 14.0,
                width: 350.0,
            }
        );
        assert_eq!(
            ToastLayout::resolve(390.0, 40.0, 46.0),
            ToastLayout {
                top: 54.0,
                right: 8.0,
                width: 374.0,
            }
        );
        assert_eq!(TOAST_LIFETIME, 5.2);
        assert_eq!(TOAST_MIN_HEIGHT, 49.0);
        assert_eq!(TOAST_GAP, 7.0);
    }
}
