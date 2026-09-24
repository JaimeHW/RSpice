//! Toast notifications — transient notices stacked under the toolbar's
//! trailing edge — and the session activity they are retained in.

use std::collections::HashSet;
use std::time::Duration;

use egui::{
    Align2, Area, Context, Frame, Id, Margin, Order, Rect, Sense, Stroke, Ui, Vec2, pos2, vec2,
};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::notice;

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
    #[default]
    System,
}

impl NotificationCategory {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Job => "Job",
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

/// Where a notice offers to take the reader.
///
/// The widget stores the offer, draws it, and reports it back when it is
/// taken; performing it belongs to the shell. The destination is therefore
/// named in product terms rather than as a workbench command, which this
/// layer cannot see.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationAction {
    /// Activate the retained dataset of the run with this display sequence.
    OpenRunInResults { run_sequence: u64 },
    /// Open the Console, where the line this notice was lifted from sits among
    /// the lines around it. A warning is one sentence here and a paragraph
    /// there, and the paragraph is usually what explains it.
    ShowInConsole,
}

impl NotificationAction {
    /// The affordance's own text. Short enough for a 350px toast.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenRunInResults { .. } => "Open in Results",
            Self::ShowInConsole => "Show in Console",
        }
    }

    /// What to file a record under when the record has no title of its own.
    ///
    /// A destination is an identity, and naming the record after it is what
    /// makes two reports of one event recognisably the same row rather than
    /// one row headed "Information" and another headed by the run. The
    /// Console is a place rather than an identity — every line lives there —
    /// so it names nothing.
    pub fn record_title(self) -> Option<String> {
        match self {
            Self::OpenRunInResults { run_sequence } => Some(format!("Run {run_sequence}")),
            Self::ShowInConsole => None,
        }
    }
}

/// One console entry offered for mirroring into retained activity.
///
/// The log model is not visible from this layer, so the shell projects each
/// entry down to what a notice needs: a stable identity to avoid replaying it,
/// display-safe text, and the destination the entry's anchor named — which is
/// also the key two reports of one event are recognised by.
#[derive(Debug, Clone)]
pub struct MirroredEntry {
    /// The log entry's own id, stable across frames.
    pub log_id: u64,
    pub category: NotificationCategory,
    pub kind: ToastKind,
    pub message: String,
    /// Where this entry leads, when its anchor named somewhere.
    pub action: Option<NotificationAction>,
    /// The original event time on the egui clock.
    pub created: f64,
}

/// One queued toast.
#[derive(Debug, Clone)]
pub struct Toast {
    id: u64,
    title: String,
    message: String,
    kind: ToastKind,
    /// Where this notice offers to go, when it offers anywhere.
    action: Option<NotificationAction>,
    /// Absolute time (egui clock) at which the toast was created.
    created: f64,
    /// Absolute time at which it leaves. Separate from `created` because a
    /// notice under the pointer is being read, and its clock starts over when
    /// the pointer moves away.
    expires: f64,
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
    action: Option<NotificationAction>,
    created: f64,
    read: bool,
}

impl NotificationRecord {
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Where this retained record offers to go, if anywhere. A toast expires
    /// in seconds; the offer it carried outlives it here.
    pub const fn action(&self) -> Option<NotificationAction> {
        self.action
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
const TOAST_RADIUS: u8 = 8;
/// Severity rail down the card's leading edge.
const TOAST_RAIL_WIDTH: f32 = 3.0;
/// The drain bar: the lifetime, drawn. It empties as the notice runs out.
const TOAST_DRAIN_HEIGHT: f32 = 2.0;
/// How often the drain bar is redrawn. It moves about two points a step at
/// this rate, which reads as motion without repainting the workbench at the
/// display's full rate for five seconds per notice.
const TOAST_DRAIN_STEP: Duration = Duration::from_millis(33);
const TOAST_COLUMN_GAP: f32 = 10.0;

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
    #[cfg(test)]
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
        self.enqueue(ctx, category, kind, kind.label(), message, None, false);
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
        self.enqueue(ctx, category, kind, title, message, None, true);
    }

    /// Queue a titled notice that also offers somewhere to go.
    ///
    /// The offer is retained with the activity record, so a reader who missed
    /// the five-second toast still reaches the same destination from the
    /// notification center.
    pub fn notify_with_action(
        &mut self,
        ctx: &Context,
        category: NotificationCategory,
        kind: ToastKind,
        title: impl Into<String>,
        message: impl Into<String>,
        action: NotificationAction,
    ) {
        self.enqueue(ctx, category, kind, title, message, Some(action), true);
    }

    fn enqueue(
        &mut self,
        ctx: &Context,
        category: NotificationCategory,
        kind: ToastKind,
        title: impl Into<String>,
        message: impl Into<String>,
        action: Option<NotificationAction>,
        deduplicate_visible_title: bool,
    ) {
        let title = title.into();
        let message = message.into();
        let created = ctx.input(|i| i.time);
        let id = self.record_activity(
            title.clone(),
            message.clone(),
            category,
            kind,
            action,
            created,
        );
        if deduplicate_visible_title {
            self.queue
                .retain(|toast| toast.kind != kind || toast.title != title);
        }
        self.queue.push(Toast {
            id,
            title,
            message,
            kind,
            action,
            created,
            expires: created + TOAST_LIFETIME,
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
        action: Option<NotificationAction>,
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
                action,
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
        entries: impl IntoIterator<Item = MirroredEntry>,
    ) {
        let first_unobserved = self.observed_log_revision;
        let mut accepted_ids = HashSet::new();
        for entry in entries {
            if entry.log_id < first_unobserved
                || entry.log_id >= log_revision
                || !accepted_ids.insert(entry.log_id)
            {
                continue;
            }
            // Two records naming the same destination are two reports of one
            // event, not two events. The shell announces a finished run in
            // its own words — with the run's identity and what it retained —
            // and this line is the same completion said again; keeping both
            // counted one run twice in the unread badge.
            if entry.action.is_some()
                && self.activity.iter().any(|held| held.action == entry.action)
            {
                continue;
            }
            // A line that names nowhere still came from somewhere. A warning
            // or a failure is one sentence here and sits among the lines that
            // explain it in the Console, so that is where it offers to go. An
            // informational line explains itself, and an offer on every row
            // would be a column of links nobody needs.
            let offer = entry.action.or_else(|| {
                matches!(entry.kind, ToastKind::Warn | ToastKind::Error)
                    .then_some(NotificationAction::ShowInConsole)
            });
            self.record_activity(
                // A mirrored line has no title of its own, so it is filed
                // under its severity. An entry that does name a destination
                // has an identity worth showing instead.
                entry
                    .action
                    .and_then(NotificationAction::record_title)
                    .unwrap_or_else(|| entry.kind.label().to_owned()),
                entry.message,
                entry.category,
                entry.kind,
                offer,
                entry.created,
            );
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

    /// How many records a session keeps before the oldest is dropped. The
    /// panel states it, so a reader who finds an old notice gone knows why.
    pub const fn retention_limit() -> usize {
        MAX_RETAINED_ACTIVITY
    }

    /// Next structured-log identity already consumed by the activity stream.
    pub const fn observed_log_revision(&self) -> u64 {
        self.observed_log_revision
    }

    /// Mark one record read. Returns whether that changed anything, so a
    /// caller can tell a row that was already read from one that is gone.
    pub fn mark_read(&mut self, id: u64) -> bool {
        let Some(item) = self.activity.iter_mut().find(|item| item.id == id) else {
            return false;
        };
        !std::mem::replace(&mut item.read, true)
    }

    /// Drop one record, read or not, along with its toast if that is still on
    /// screen: a notice the reader threw away must not reappear beside the
    /// panel they threw it away from.
    pub fn dismiss(&mut self, id: u64) -> bool {
        let held = self.activity.len();
        self.activity.retain(|item| item.id != id);
        self.queue.retain(|toast| toast.id != id);
        self.activity.len() != held
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
    ///
    /// Returns the navigation offer the reader took, if any. Taking one also
    /// dismisses its toast: the notice has been answered, and leaving it on
    /// screen would invite a second click that lands on the same place.
    pub fn show(
        &mut self,
        ctx: &Context,
        title_bar_height: f32,
        toolbar_height: f32,
        large_targets: bool,
    ) -> Option<NotificationAction> {
        if self.queue.is_empty() {
            return None;
        }
        let now = ctx.input(|i| i.time);
        self.queue.retain(|toast| now < toast.expires);
        if self.queue.is_empty() {
            return None;
        }

        let soonest_expiry = self
            .queue
            .iter()
            .map(|toast| (toast.expires - now).max(0.0))
            .fold(f64::INFINITY, f64::min);
        let animate = ctx.global_style().animation_time > 0.0;
        if animate {
            // The drain bar is the clock made visible, so it has to be seen
            // moving. Reduced motion draws no bar and wakes once, at expiry.
            ctx.request_repaint_after(
                TOAST_DRAIN_STEP.min(Duration::from_secs_f64(soonest_expiry)),
            );
        } else if soonest_expiry.is_finite() {
            ctx.request_repaint_after(Duration::from_secs_f64(soonest_expiry));
        }
        if animate
            && self
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
            return None;
        }
        let mut answered = None;
        let mut taken = None;
        let mut held = Vec::new();

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
                    let tone = notice::tone_color(&t, toast.kind);
                    let remaining = ((toast.expires - now) / TOAST_LIFETIME).clamp(0.0, 1.0);
                    let card = ui
                        .scope(|ui| {
                            // One opacity for the whole card, so the entrance
                            // fades the offer and the glyph with the panel
                            // behind them instead of popping them in over it.
                            ui.set_opacity(toast_entry_opacity(now - toast.created, animate));
                            let frame = Frame::NONE
                                .fill(c.bg_elevated)
                                .stroke(Stroke::new(1.0, c.border_strong))
                                .corner_radius(TOAST_RADIUS)
                                .shadow(t.shadow())
                                .inner_margin(Margin {
                                    left: 12,
                                    right: 10,
                                    top: 11,
                                    bottom: 12,
                                })
                                .show(ui, |ui| {
                                    ui.set_min_height(TOAST_MIN_HEIGHT - 23.0 - 2.0);
                                    let outcome = toast_contents(ui, toast, large_targets);
                                    if outcome.taken.is_some() {
                                        taken = outcome.taken;
                                    }
                                    if outcome.dismissed || outcome.taken.is_some() {
                                        answered = Some(toast.id);
                                    }
                                });
                            let card = frame.response.rect;
                            paint_card_edges(ui, card, tone, animate.then_some(remaining as f32));
                            card
                        })
                        .inner;
                    let toast_response = ui.interact(
                        card,
                        ui.id().with(("toast-status", toast.id)),
                        Sense::hover(),
                    );
                    if toast_response.contains_pointer() {
                        held.push(toast.id);
                    }
                    ui.ctx().accesskit_node_builder(toast_response.id, |node| {
                        node.set_role(egui::accesskit::Role::Status);
                        node.set_label(format!(
                            "{}: {}. {}",
                            toast.kind.label(),
                            toast.title,
                            toast.message
                        ));
                    });
                }
            });

        // A notice under the pointer is being read. Its clock starts over when
        // the pointer leaves rather than resuming, so the drain bar the reader
        // comes back to is a full one and not a sliver about to vanish.
        for toast in &mut self.queue {
            if held.contains(&toast.id) {
                toast.expires = now + TOAST_LIFETIME;
            }
        }
        if let Some(id) = answered {
            // Dismissing a notice, or following it, is having read it. One
            // that simply times out was not necessarily seen, and stays unread.
            self.queue.retain(|toast| toast.id != id);
            self.mark_read(id);
        }
        taken
    }
}

/// The severity rail and the drain bar, both cut from the card's own rounded
/// shape so they follow its corners instead of squaring them off.
fn paint_card_edges(ui: &Ui, card: Rect, tone: egui::Color32, remaining: Option<f32>) {
    let inner = card.shrink(1.0);
    let radius = f32::from(TOAST_RADIUS) - 1.0;
    let rail = Rect::from_min_max(
        inner.left_top(),
        pos2(inner.left() + TOAST_RAIL_WIDTH, inner.bottom()),
    );
    ui.painter()
        .with_clip_rect(rail.intersect(ui.clip_rect()))
        .rect_filled(inner, radius, tone);
    let Some(remaining) = remaining else {
        return;
    };
    let drain = Rect::from_min_max(
        pos2(inner.left(), inner.bottom() - TOAST_DRAIN_HEIGHT),
        pos2(inner.left() + inner.width() * remaining, inner.bottom()),
    );
    ui.painter()
        .with_clip_rect(drain.intersect(ui.clip_rect()))
        .rect_filled(inner, radius, tone.gamma_multiply(0.55));
}

/// What one drawn toast reported back.
struct ToastOutcome {
    dismissed: bool,
    taken: Option<NotificationAction>,
}

fn toast_entry_opacity(age: f64, animate_entry: bool) -> f32 {
    if animate_entry {
        (age / TOAST_ENTER).clamp(0.0, 1.0) as f32
    } else {
        1.0
    }
}

fn toast_contents(ui: &mut Ui, toast: &Toast, large_targets: bool) -> ToastOutcome {
    let t = Tokens::get(ui.ctx());
    let mut dismissed = false;
    let mut taken = None;
    let dismiss_side = if large_targets {
        tokens::TOUCH_TARGET
    } else {
        notice::DISMISS_SIDE
    };
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = TOAST_COLUMN_GAP;
        let (glyph_rect, _) = ui.allocate_exact_size(
            vec2(notice::GLYPH_SIDE, notice::GLYPH_SIDE + 1.0),
            Sense::hover(),
        );
        notice::paint_tone_glyph(
            ui.painter(),
            Rect::from_min_size(
                glyph_rect.left_top() + vec2(0.0, 1.0),
                Vec2::splat(notice::GLYPH_SIDE),
            ),
            toast.kind,
            notice::tone_color(&t, toast.kind),
        );

        let content_width = (ui.available_width() - TOAST_COLUMN_GAP - dismiss_side).max(0.0);
        ui.vertical(|ui| {
            ui.set_min_width(content_width);
            ui.set_max_width(content_width);
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(
                egui::RichText::new(&toast.title)
                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            if !toast.message.is_empty() {
                ui.label(
                    egui::RichText::new(&toast.message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            }
            // The offer sits under the detail rather than beside the close
            // mark: it is the notice's one positive action, and putting it in
            // the corner strip would make it a second dismissal.
            if let Some(action) = toast.action {
                ui.add_space(1.0);
                if notice::offer_link(ui, action.label(), &toast.title, large_targets).clicked() {
                    taken = Some(action);
                }
            }
        });

        let (close_rect, _) = ui.allocate_exact_size(Vec2::splat(dismiss_side), Sense::hover());
        dismissed = notice::dismiss_button(
            ui,
            close_rect.translate(vec2(0.0, -2.0)),
            ui.id().with(("toast-dismiss", toast.id)),
            &format!("Dismiss {}", toast.title),
        )
        .clicked();
    });
    ToastOutcome { dismissed, taken }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A mirrored console line that names nowhere in particular.
    fn mirrored(
        log_id: u64,
        category: NotificationCategory,
        kind: ToastKind,
        message: &str,
        created: f64,
    ) -> MirroredEntry {
        MirroredEntry {
            log_id,
            category,
            kind,
            message: message.to_owned(),
            action: None,
            created,
        }
    }

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
                mirrored(
                    0,
                    NotificationCategory::System,
                    ToastKind::Warn,
                    "save conflict",
                    4.0,
                ),
                mirrored(
                    1,
                    NotificationCategory::System,
                    ToastKind::Info,
                    "project opened",
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
                mirrored(
                    0,
                    NotificationCategory::System,
                    ToastKind::Warn,
                    "save conflict",
                    9.0,
                ),
                mirrored(
                    1,
                    NotificationCategory::System,
                    ToastKind::Info,
                    "project opened",
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
                mirrored(
                    0,
                    NotificationCategory::Job,
                    ToastKind::Info,
                    "run complete",
                    1.0,
                ),
                mirrored(
                    0,
                    NotificationCategory::Job,
                    ToastKind::Info,
                    "run complete",
                    1.0,
                ),
            ],
        );

        assert_eq!(toasts.activity().len(), 1);
    }

    /// One completion, one record. The shell announces a finished run and the
    /// console line for the same run is mirrored moments later; both name the
    /// same dataset, so the second is a repetition rather than new activity.
    #[test]
    fn a_run_reported_twice_is_retained_once_and_keeps_the_offer() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        let action = NotificationAction::OpenRunInResults { run_sequence: 12 };
        toasts.notify_with_action(
            &ctx,
            NotificationCategory::Job,
            ToastKind::Success,
            "Run 12 complete",
            "3 retained analyses in this immutable dataset.",
            action,
        );

        toasts.synchronize_activity(
            1,
            [MirroredEntry {
                log_id: 0,
                category: NotificationCategory::Job,
                kind: ToastKind::Info,
                message: "All 3 analyses completed successfully".to_owned(),
                action: Some(action),
                created: 1.0,
            }],
        );

        assert_eq!(
            toasts.activity().len(),
            1,
            "one finished run must leave one record, not one per reporter"
        );
        let record = &toasts.activity()[0];
        assert_eq!(record.title(), "Run 12 complete");
        assert_eq!(
            record.action(),
            Some(action),
            "and the surviving record must keep the offer the notice carried"
        );
        assert_eq!(toasts.unread_count(), 1, "so the badge counts the run once");
    }

    /// A completion nobody announced is still worth keeping — and now arrives
    /// with the offer the announcement would have carried.
    #[test]
    fn an_unannounced_run_line_is_retained_and_named_after_its_run() {
        let mut toasts = Toasts::default();
        let action = NotificationAction::OpenRunInResults { run_sequence: 4 };

        toasts.synchronize_activity(
            1,
            [MirroredEntry {
                log_id: 0,
                category: NotificationCategory::Job,
                kind: ToastKind::Info,
                message: "Simulation completed successfully".to_owned(),
                action: Some(action),
                created: 1.0,
            }],
        );

        assert_eq!(toasts.activity().len(), 1);
        assert_eq!(
            toasts.activity()[0].title(),
            "Run 4",
            "a line that names a destination is filed under it, not under its severity"
        );
        assert_eq!(toasts.activity()[0].action(), Some(action));
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

    /// A warning lifted from the Console offers the way back to it, and two
    /// warnings are two events: the Console is a place, not an identity, so
    /// the rule that collapses two reports of one run must not collapse them.
    #[test]
    fn mirrored_warnings_offer_the_console_and_are_not_collapsed() {
        let mut toasts = Toasts::default();
        toasts.synchronize_activity(
            3,
            [
                mirrored(
                    0,
                    NotificationCategory::Job,
                    ToastKind::Warn,
                    "gmin stepping was needed",
                    1.0,
                ),
                mirrored(
                    1,
                    NotificationCategory::Job,
                    ToastKind::Error,
                    "singular matrix at node out",
                    2.0,
                ),
                mirrored(
                    2,
                    NotificationCategory::System,
                    ToastKind::Info,
                    "project opened",
                    3.0,
                ),
            ],
        );

        assert_eq!(toasts.activity().len(), 3);
        let offers: Vec<_> = toasts
            .activity()
            .iter()
            .map(NotificationRecord::action)
            .collect();
        assert_eq!(
            offers,
            [
                None,
                Some(NotificationAction::ShowInConsole),
                Some(NotificationAction::ShowInConsole),
            ],
            "newest first: the informational line explains itself, the other two lead back"
        );
        assert_eq!(
            toasts.activity()[1].title(),
            "Error",
            "and a line headed for the Console is still filed under its severity"
        );
    }

    #[test]
    fn dismissing_a_record_takes_its_toast_with_it() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        toasts.success(&ctx, "Project saved", "Revision 13 is durable.");
        toasts.info(&ctx, "kept");
        let saved = toasts.activity()[1].id();

        assert!(toasts.dismiss(saved));
        assert_eq!(toasts.activity().len(), 1);
        assert_eq!(toasts.activity()[0].message(), "kept");
        assert_eq!(toasts.queue.len(), 1, "the dismissed notice left the stack");
        assert!(!toasts.dismiss(saved), "a record is dismissed once");
    }

    #[test]
    fn marking_read_reports_a_change_only_the_first_time() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        toasts.info(&ctx, "first");
        let id = toasts.activity()[0].id();

        assert!(toasts.mark_read(id));
        assert!(!toasts.mark_read(id));
        assert!(!toasts.mark_read(id + 1));
        assert_eq!(toasts.unread_count(), 0);
    }

    /// A notice leaves at its deadline, and the deadline is its own field so a
    /// notice being read can have it moved without rewriting when it arrived.
    #[test]
    fn a_toast_lives_until_its_deadline_and_a_timeout_leaves_it_unread() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut toasts = Toasts::default();
        toasts.info(&ctx, "first");
        assert_eq!(
            toasts.queue[0].expires,
            toasts.queue[0].created + TOAST_LIFETIME
        );

        let pass = |toasts: &mut Toasts, time: f64| {
            let _ = ctx.run_ui(
                egui::RawInput {
                    time: Some(time),
                    ..Default::default()
                },
                |ui| {
                    let _ = toasts.show(ui.ctx(), 35.0, 45.0, false);
                },
            );
        };
        pass(&mut toasts, TOAST_LIFETIME - 0.1);
        assert_eq!(toasts.queue.len(), 1);
        pass(&mut toasts, TOAST_LIFETIME + 0.1);
        assert!(toasts.queue.is_empty());
        assert_eq!(
            toasts.unread_count(),
            1,
            "a notice that timed out was not necessarily seen"
        );
    }

    #[test]
    fn reduced_motion_skips_the_toast_entry_fade() {
        assert_eq!(toast_entry_opacity(0.0, false), 1.0);
        assert_eq!(toast_entry_opacity(TOAST_ENTER / 2.0, false), 1.0);
        assert_eq!(toast_entry_opacity(TOAST_ENTER / 2.0, true), 0.5);
    }
}
