//! Toast notifications — transient notices stacked under the toolbar's
//! trailing edge — and the session activity they are retained in.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use egui::text::{LayoutJob, TextFormat, TextWrapping};
use egui::{
    Align2, Area, Context, Frame, Galley, Id, Margin, Order, Rect, Sense, Stroke, Ui, Vec2, pos2,
    vec2,
};

use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

use super::notice;

/// Visual severity of a toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastKind {
    /// A completed action, confirmed by a neutral check glyph.
    Success,
    /// An informational notice, under a neutral info glyph.
    #[default]
    Info,
    /// A warning, marked by a triangle in the warning colour.
    Warn,
    /// An error, marked by an octagon in the error colour.
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
    /// Open the Console on the line this notice was lifted from, by the log
    /// entry's id, among the lines around it. A warning is one sentence here
    /// and a paragraph there, and the paragraph is usually what explains it.
    ShowInConsole { log_id: u64 },
}

impl NotificationAction {
    /// The affordance's own text. Short enough for a 350px toast.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenRunInResults { .. } => "Open in Results",
            Self::ShowInConsole { .. } => "Show in Console",
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
            Self::ShowInConsole { .. } => None,
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
    /// Absolute time at which it leaves, or `None` for a notice that stays
    /// until it is answered. Separate from `created` because a notice under
    /// the pointer is being read, and its clock starts over when the pointer
    /// moves away; and because a held clock pushes it back.
    expires: Option<f64>,
    /// Whether one of its controls had the keyboard on the last pass. A
    /// reader tabbing to "Show in Console" is reading the notice, and it must
    /// not leave from under the focus.
    focus_held: bool,
}

impl Toast {
    /// How much of its lifetime is left, from one down to zero. `None` for a
    /// notice with no deadline, which has no clock to draw.
    fn remaining(&self, now: f64) -> Option<f32> {
        let lifetime = toast_lifetime(self.kind)?;
        let expires = self.expires?;
        Some(((expires - now) / lifetime).clamp(0.0, 1.0) as f32)
    }
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

/// Lifetime of a success or information toast in seconds (matches the
/// reviewed workbench mockup).
const TOAST_LIFETIME: f64 = 5.2;
/// Lifetime of a warning. A warning usually asks for a second look, and five
/// seconds is a glance.
const TOAST_WARN_LIFETIME: f64 = 10.0;

/// How long a toast of this severity stays unattended. An error has no
/// deadline: it is what the reader has to act on, so it leaves when it is
/// dismissed or followed, and never while they are looking elsewhere.
const fn toast_lifetime(kind: ToastKind) -> Option<f64> {
    match kind {
        ToastKind::Success | ToastKind::Info => Some(TOAST_LIFETIME),
        ToastKind::Warn => Some(TOAST_WARN_LIFETIME),
        ToastKind::Error => None,
    }
}
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
/// The drain bar: the lifetime, drawn. It empties as the notice runs out.
const TOAST_DRAIN_HEIGHT: f32 = 2.0;
/// How often the drain bar is redrawn. It moves about two points a step at
/// this rate, which reads as motion without repainting the workbench at the
/// display's full rate for five seconds per notice.
const TOAST_DRAIN_STEP: Duration = Duration::from_millis(33);
const TOAST_COLUMN_GAP: f32 = 10.0;
/// The most lines a toast's message takes before it is cut. The notification
/// panel's rows hold two; a toast arrives alone and can say a little more.
const TOAST_MESSAGE_MAX_ROWS: usize = 4;

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
    /// When the stack was last drawn, and whether the window it was drawn in
    /// had lost focus to another application then. The time since is charged
    /// to that state: a run that finishes while the reader is in another
    /// application is announced when they come back, not to nobody.
    last_pass: Option<(f64, bool)>,
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

    // Retain the existing notification contract: content, routing, tone and
    // visible deduplication are independent inputs.
    #[expect(clippy::too_many_arguments)]
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
            expires: toast_lifetime(kind).map(|lifetime| created + lifetime),
            focus_held: false,
        });
        // Past the visible three, the oldest notice that is not an error makes
        // room. An error is the notice the reader has to act on, and three
        // confirmations arriving behind it must not push it off screen; only
        // a stack of errors gives up its oldest. Either way the record stays
        // in the bell.
        while self.queue.len() > MAX_VISIBLE_TOASTS {
            let oldest = self
                .queue
                .iter()
                .position(|toast| toast.kind != ToastKind::Error)
                .unwrap_or(0);
            self.queue.remove(oldest);
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
            // A line that names nowhere still came from somewhere. A warning
            // or a failure is one sentence here and sits among the lines that
            // explain it in the Console, so that is where it offers to go. An
            // informational line explains itself, and an offer on every row
            // would be a column of links nobody needs.
            let offer = entry.action.or_else(|| {
                matches!(entry.kind, ToastKind::Warn | ToastKind::Error).then_some(
                    NotificationAction::ShowInConsole {
                        log_id: entry.log_id,
                    },
                )
            });
            // Two records offering the same destination are two reports of one
            // event, not two events. The shell announces a finished run in
            // its own words — with the run's identity and what it retained —
            // and a failure it toasted already offers this very line; keeping
            // both counted one event twice in the unread badge.
            if offer.is_some() && self.activity.iter().any(|held| held.action == offer) {
                continue;
            }
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

    /// Take back every offer `retired` matches, from the bell's records and
    /// from the toasts still on screen. The notices keep their title and
    /// message: what happened is still true, but where they offered to go
    /// is no longer there.
    pub fn retire_offers(&mut self, retired: impl Fn(NotificationAction) -> bool) {
        let offers = self
            .activity
            .iter_mut()
            .map(|record| &mut record.action)
            .chain(self.queue.iter_mut().map(|toast| &mut toast.action));
        for offer in offers {
            if offer.is_some_and(&retired) {
                *offer = None;
            }
        }
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
        let now = ctx.input(|i| i.time);
        // Another application has the keyboard, or the window cannot be
        // seen at all: minimised, or a browser tab in the background, which
        // is all a browser says. Nobody is reading the notices, so their
        // clocks stand still until the reader is back.
        let away = ctx.input(|i| {
            let viewport = i.viewport();
            viewport.focused == Some(false) || viewport.occluded == Some(true)
        });
        if let Some((then, was_away)) = self.last_pass.replace((now, away)) {
            hold_clocks(&mut self.queue, now, then, was_away);
        }
        self.queue
            .retain(|toast| toast.expires.is_none_or(|expires| now < expires));
        if self.queue.is_empty() {
            return None;
        }

        let animate = ctx.global_style().animation_time > 0.0;
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
        let mut hovered = Vec::new();
        let mut focused = Vec::new();

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
                    let mut cut = false;
                    let mut control_hovered = false;
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
                                    if outcome.focused {
                                        focused.push(toast.id);
                                    }
                                    cut = outcome.cut;
                                    control_hovered = outcome.control_hovered;
                                });
                            let card = frame.response.rect;
                            // Only a notice with a deadline has a clock to draw.
                            if animate && let Some(remaining) = toast.remaining(now) {
                                paint_drain_bar(ui, card, remaining);
                            }
                            card
                        })
                        .inner;
                    let toast_response = ui.interact(
                        card,
                        ui.id().with(("toast-status", toast.id)),
                        Sense::hover(),
                    );
                    if toast_response.contains_pointer() {
                        hovered.push(toast.id);
                    }
                    ui.ctx().accesskit_node_builder(toast_response.id, |node| {
                        node.set_role(egui::accesskit::Role::Status);
                        // Read out when it arrives, without taking the
                        // reader's focus: a confirmation waits for a pause,
                        // a warning or a failure interrupts.
                        node.set_live(toast_live(toast.kind));
                        node.set_label(format!(
                            "{}: {}. {}",
                            toast.kind.label(),
                            toast.title,
                            toast.message
                        ));
                    });
                    if cut && !control_hovered {
                        // What the card cut short is a hover away, in full,
                        // as it is on the notification panel's rows.
                        toast_response.on_hover_text(format!("{}\n{}", toast.title, toast.message));
                    }
                }
            });

        // A notice under the pointer is being read. Its clock starts over when
        // the pointer leaves rather than resuming, so the drain bar the reader
        // comes back to is a full one and not a sliver about to vanish. One
        // whose control has the keyboard is held until the next pass charges
        // the time to it.
        for toast in &mut self.queue {
            if hovered.contains(&toast.id) {
                toast.expires = toast_lifetime(toast.kind).map(|lifetime| now + lifetime);
            }
            toast.focus_held = focused.contains(&toast.id);
        }
        if let Some(id) = answered {
            // Dismissing a notice, or following it, is having read it. One
            // that simply times out was not necessarily seen, and stays unread.
            self.queue.retain(|toast| toast.id != id);
            self.mark_read(id);
        }

        // Wake for the soonest clock that is running. A held clock has
        // nothing on it that moves, and whatever releases it (the pointer
        // leaving, the keyboard moving, the window coming back) is an event
        // that wakes the frame on its own; so a window left in the
        // background sleeps rather than redrawing a still bar thirty times a
        // second. A stack of errors has no deadline to wake for at all.
        let soonest_expiry = self
            .queue
            .iter()
            .filter(|toast| !away && !toast.focus_held && !hovered.contains(&toast.id))
            .filter_map(|toast| toast.expires)
            .map(|expires| (expires - now).max(0.0))
            .fold(f64::INFINITY, f64::min);
        if soonest_expiry.is_finite() {
            let expiry = Duration::from_secs_f64(soonest_expiry);
            // The drain bar is the clock made visible, so it has to be seen
            // moving. Reduced motion draws no bar and wakes once, at expiry.
            ctx.request_repaint_after(if animate {
                TOAST_DRAIN_STEP.min(expiry)
            } else {
                expiry
            });
        }
        taken
    }

    /// Keep a titled record, and its offer, in the notification center
    /// without raising a toast.
    ///
    /// For an outcome the reader asked not to be interrupted for: it is
    /// still unread, still counted on the bell, and still leads where it
    /// would have led, but nothing appears over their work.
    pub fn record_quietly(
        &mut self,
        ctx: &Context,
        category: NotificationCategory,
        kind: ToastKind,
        title: impl Into<String>,
        message: impl Into<String>,
        action: Option<NotificationAction>,
    ) {
        let created = ctx.input(|i| i.time);
        self.record_activity(
            title.into(),
            message.into(),
            category,
            kind,
            action,
            created,
        );
    }
}

/// Push back the deadline of every notice whose clock was held from the last
/// pass at `then` until `now`: all of them while the window was away, and one
/// whose control had the keyboard. A notice is charged only for the time it
/// was on the stack, so one raised while the reader was away gets its whole
/// lifetime once they are back.
fn hold_clocks(queue: &mut [Toast], now: f64, then: f64, window_away: bool) {
    for toast in queue {
        if let Some(expires) = &mut toast.expires
            && (window_away || toast.focus_held)
        {
            *expires += (now - then.max(toast.created)).max(0.0);
        }
    }
}

/// The drain bar, cut from the card's own rounded shape so it follows the
/// card's corners instead of squaring them off.
///
/// It is a clock, not a severity: the glyph says how serious the notice is,
/// and the bar is drawn in the card's own border tone whatever it says.
fn paint_drain_bar(ui: &Ui, card: Rect, remaining: f32) {
    let inner = card.shrink(1.0);
    let radius = f32::from(TOAST_RADIUS) - 1.0;
    let drain = Rect::from_min_max(
        pos2(inner.left(), inner.bottom() - TOAST_DRAIN_HEIGHT),
        pos2(inner.left() + inner.width() * remaining, inner.bottom()),
    );
    ui.painter()
        .with_clip_rect(drain.intersect(ui.clip_rect()))
        .rect_filled(inner, radius, Tokens::get(ui.ctx()).color.border_strong);
}

/// What one drawn toast reported back.
struct ToastOutcome {
    dismissed: bool,
    taken: Option<NotificationAction>,
    /// One of its controls has the keyboard.
    focused: bool,
    /// Its message ran past the lines a card gives it.
    cut: bool,
    /// The pointer is on one of its controls, which have tooltips of their own.
    control_hovered: bool,
}

/// How a screen reader is told a toast arrived: politely for a
/// confirmation, at once for something that may need the reader.
const fn toast_live(kind: ToastKind) -> egui::accesskit::Live {
    match kind {
        ToastKind::Success | ToastKind::Info => egui::accesskit::Live::Polite,
        ToastKind::Warn | ToastKind::Error => egui::accesskit::Live::Assertive,
    }
}

/// A toast's detail, wrapped to the card and cut at four lines with an
/// ellipsis. A console paragraph must not turn one notice into a page.
fn message_galley(ui: &Ui, message: &str, width: f32) -> Arc<Galley> {
    let mut job = LayoutJob::single_section(
        message.to_owned(),
        TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            color: Tokens::get(ui.ctx()).color.text_dim,
            ..Default::default()
        },
    );
    job.wrap = TextWrapping {
        max_width: width,
        max_rows: TOAST_MESSAGE_MAX_ROWS,
        break_anywhere: false,
        overflow_character: Some('\u{2026}'),
    };
    ui.painter().layout_job(job)
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
    let mut focused = false;
    let mut cut = false;
    let mut control_hovered = false;
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
                let message = message_galley(ui, &toast.message, content_width);
                cut = message.elided;
                ui.label(message);
            }
            // The offer sits under the detail rather than beside the close
            // mark: it is the notice's one positive action, and putting it in
            // the corner strip would make it a second dismissal.
            if let Some(action) = toast.action {
                ui.add_space(1.0);
                let offer = notice::offer_link(ui, action.label(), &toast.title, large_targets);
                focused |= offer.has_focus();
                control_hovered |= offer.contains_pointer();
                if offer.clicked() {
                    taken = Some(action);
                }
            }
        });

        let (close_rect, _) = ui.allocate_exact_size(Vec2::splat(dismiss_side), Sense::hover());
        let dismiss = notice::dismiss_button(
            ui,
            close_rect.translate(vec2(0.0, -2.0)),
            ui.id().with(("toast-dismiss", toast.id)),
            &format!("Dismiss {}", toast.title),
        );
        focused |= dismiss.has_focus();
        control_hovered |= dismiss.contains_pointer();
        dismissed = dismiss.clicked();
    });
    ToastOutcome {
        dismissed,
        taken,
        focused,
        cut,
        control_hovered,
    }
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
            "3 analyses saved with this run.",
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

    /// One failure, one record. A failure is toasted in its own words with an
    /// offer to show the Console line it wrote, and that same line is then
    /// mirrored; the line is the failure said again, not a second failure.
    #[test]
    fn a_toasted_failure_and_its_console_line_are_retained_once() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        let action = NotificationAction::ShowInConsole { log_id: 7 };
        toasts.notify_with_action(
            &ctx,
            NotificationCategory::System,
            ToastKind::Error,
            "Could not save project",
            "The disk is full.",
            action,
        );

        toasts.synchronize_activity(
            8,
            [mirrored(
                7,
                NotificationCategory::System,
                ToastKind::Error,
                "Could not save project: The disk is full.",
                1.0,
            )],
        );

        assert_eq!(
            toasts.activity().len(),
            1,
            "a toasted failure and its mirrored line must leave one record"
        );
        assert_eq!(toasts.activity()[0].title(), "Could not save project");
        assert_eq!(toasts.activity()[0].action(), Some(action));
        assert_eq!(toasts.unread_count(), 1, "so the badge counts it once");

        // A different line is a different event and is still kept.
        toasts.synchronize_activity(
            9,
            [mirrored(
                8,
                NotificationCategory::System,
                ToastKind::Error,
                "Could not save project: The disk is full.",
                2.0,
            )],
        );
        assert_eq!(toasts.activity().len(), 2);
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

    /// A quiet record is everything a notice is except the toast: unread,
    /// counted, titled, and still offering where it leads. It neither raises
    /// a toast nor replaces one that is already up under the same title.
    #[test]
    fn a_quiet_record_reaches_the_bell_without_a_toast() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        toasts.success(&ctx, "Run 3 complete", "2 analyses saved");
        let offer = NotificationAction::OpenRunInResults { run_sequence: 4 };

        toasts.record_quietly(
            &ctx,
            NotificationCategory::Job,
            ToastKind::Success,
            "Run 4 complete",
            "1 analysis saved",
            Some(offer),
        );
        toasts.record_quietly(
            &ctx,
            NotificationCategory::Job,
            ToastKind::Success,
            "Run 3 complete",
            "",
            None,
        );

        assert_eq!(toasts.queue.len(), 1, "no toast was raised or replaced");
        assert_eq!(toasts.queue[0].message, "2 analyses saved");
        assert_eq!(toasts.activity().len(), 3);
        assert_eq!(toasts.unread_count(), 3);
        let quiet = &toasts.activity()[1];
        assert_eq!(quiet.title(), "Run 4 complete");
        assert_eq!(quiet.message(), "1 analysis saved");
        assert_eq!(quiet.category(), NotificationCategory::Job);
        assert_eq!(quiet.kind(), ToastKind::Success);
        assert_eq!(quiet.action(), Some(offer));
        assert!(!quiet.is_read());
        assert_eq!(toasts.activity()[0].action(), None);
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
                Some(NotificationAction::ShowInConsole { log_id: 1 }),
                Some(NotificationAction::ShowInConsole { log_id: 0 }),
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

    /// A retired offer leaves the record and the toast on screen alike, and
    /// the notice itself stays; an offer the predicate spares stays too.
    #[test]
    fn retiring_an_offer_keeps_the_notice_and_takes_the_offer_everywhere() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        let run = NotificationAction::OpenRunInResults { run_sequence: 3 };
        let console = NotificationAction::ShowInConsole { log_id: 9 };
        toasts.notify_with_action(
            &ctx,
            NotificationCategory::Job,
            ToastKind::Success,
            "Run 3 complete",
            "2 analyses saved with this run.",
            run,
        );
        toasts.notify_with_action(
            &ctx,
            NotificationCategory::System,
            ToastKind::Warn,
            "Library reloaded",
            "1 model changed.",
            console,
        );

        toasts.retire_offers(|action| action == run);

        let offers = |toasts: &Toasts| {
            (
                toasts
                    .activity()
                    .iter()
                    .map(NotificationRecord::action)
                    .collect::<Vec<_>>(),
                toasts
                    .queue
                    .iter()
                    .map(|toast| toast.action)
                    .collect::<Vec<_>>(),
            )
        };
        assert_eq!(
            offers(&toasts),
            (vec![Some(console), None], vec![None, Some(console)])
        );
        assert_eq!(toasts.activity()[1].title(), "Run 3 complete");
        assert_eq!(toasts.queue[0].message, "2 analyses saved with this run.");
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
        crate::Theme::default().apply(&ctx);
        let mut toasts = Toasts::default();
        toasts.info(&ctx, "first");
        assert_eq!(
            toasts.queue[0].expires,
            Some(toasts.queue[0].created + TOAST_LIFETIME)
        );

        pass(&ctx, &mut toasts, TOAST_LIFETIME - 0.1);
        assert_eq!(toasts.queue.len(), 1);
        pass(&ctx, &mut toasts, TOAST_LIFETIME + 0.1);
        assert!(toasts.queue.is_empty());
        assert_eq!(
            toasts.unread_count(),
            1,
            "a notice that timed out was not necessarily seen"
        );
    }

    /// One frame of the toast stack at `time` on the egui clock.
    fn pass(ctx: &Context, toasts: &mut Toasts, time: f64) {
        let _ = frame(ctx, toasts, time, None, Vec::new());
    }

    /// One frame at `time`, in a window that does or does not have the
    /// keyboard (`None`: a platform that does not say).
    fn frame(
        ctx: &Context,
        toasts: &mut Toasts,
        time: f64,
        focused: Option<bool>,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        let viewport = egui::ViewportInfo {
            focused,
            ..Default::default()
        };
        frame_in(ctx, toasts, time, viewport, events)
    }

    /// One frame at `time` in a window the platform describes as `viewport`.
    fn frame_in(
        ctx: &Context,
        toasts: &mut Toasts,
        time: f64,
        viewport: egui::ViewportInfo,
        events: Vec<egui::Event>,
    ) -> egui::FullOutput {
        let mut input = egui::RawInput {
            time: Some(time),
            events,
            ..Default::default()
        };
        input.viewports.insert(egui::ViewportId::ROOT, viewport);
        ctx.run_ui(input, |ui| {
            let _ = toasts.show(ui.ctx(), 35.0, 45.0, false);
        })
    }

    /// A browser never says whether its window has the keyboard, only
    /// whether its tab can be seen. A run that finishes in a tab in the
    /// background is announced when the reader comes back to it.
    #[test]
    fn a_window_that_cannot_be_seen_holds_every_clock() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut toasts = Toasts::default();
        toasts.info(&ctx, "Run 4 complete");
        let tab = |hidden: bool| egui::ViewportInfo {
            occluded: Some(hidden),
            ..Default::default()
        };

        let _ = frame_in(&ctx, &mut toasts, 0.0, tab(true), Vec::new());
        let _ = frame_in(&ctx, &mut toasts, 600.0, tab(true), Vec::new());
        assert_eq!(toasts.queue.len(), 1, "nobody could see it");

        let _ = frame_in(&ctx, &mut toasts, 601.0, tab(false), Vec::new());
        let back = 601.0 + TOAST_LIFETIME;
        let _ = frame_in(&ctx, &mut toasts, back - 0.1, tab(false), Vec::new());
        assert_eq!(toasts.queue.len(), 1, "it runs its lifetime once seen");
        let _ = frame_in(&ctx, &mut toasts, back + 0.1, tab(false), Vec::new());
        assert!(toasts.queue.is_empty());
    }

    /// A run that finishes while the reader is in another application is
    /// announced when they come back, for its whole lifetime: the clock stands
    /// still while the window is in the background.
    #[test]
    fn a_window_in_the_background_holds_every_clock() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut toasts = Toasts::default();
        toasts.info(&ctx, "Run 4 complete");

        let _ = frame(&ctx, &mut toasts, 0.0, Some(false), Vec::new());
        let _ = frame(&ctx, &mut toasts, 600.0, Some(false), Vec::new());
        assert_eq!(toasts.queue.len(), 1, "nobody was there to read it");

        let _ = frame(&ctx, &mut toasts, 601.0, Some(true), Vec::new());
        let _ = frame(
            &ctx,
            &mut toasts,
            601.0 + TOAST_LIFETIME - 0.1,
            Some(true),
            Vec::new(),
        );
        assert_eq!(
            toasts.queue.len(),
            1,
            "it runs its lifetime once they are back"
        );
        let _ = frame(
            &ctx,
            &mut toasts,
            601.0 + TOAST_LIFETIME + 0.1,
            Some(true),
            Vec::new(),
        );
        assert!(toasts.queue.is_empty());
    }

    /// A held clock has nothing on it that moves, so a window in the
    /// background does not redraw its toasts thirty times a second.
    #[test]
    fn a_window_in_the_background_does_not_redraw_a_still_drain_bar() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut toasts = Toasts::default();
        toasts.info(&ctx, "Run 4 complete");
        let wake = |output: &egui::FullOutput| {
            output.viewport_output[&egui::ViewportId::ROOT].repaint_delay
        };

        for time in [0.0, 0.5] {
            let _ = frame(&ctx, &mut toasts, time, Some(true), Vec::new());
        }
        let running = frame(&ctx, &mut toasts, 1.0, Some(true), Vec::new());
        assert!(
            wake(&running) <= TOAST_DRAIN_STEP,
            "a running clock is drawn moving: {:?}",
            wake(&running)
        );

        let _ = frame(&ctx, &mut toasts, 1.5, Some(false), Vec::new());
        let held = frame(&ctx, &mut toasts, 2.0, Some(false), Vec::new());
        assert!(
            wake(&held) > Duration::from_secs(60),
            "a held clock asks for no frames: {:?}",
            wake(&held)
        );
    }

    /// A reader who tabbed to a toast's control is reading it, and it does
    /// not leave from under the keyboard; it resumes when the keyboard moves on.
    #[test]
    fn a_toast_whose_control_has_the_keyboard_holds_its_clock() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut toasts = Toasts::default();
        toasts.notify_with_action(
            &ctx,
            NotificationCategory::Job,
            ToastKind::Info,
            "Run 4 complete",
            "2 analyses saved with this run.",
            NotificationAction::OpenRunInResults { run_sequence: 4 },
        );
        let tab = egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };

        // The stack measures itself first; then Tab, and a pass at the same
        // instant for the focus to settle.
        for events in [Vec::new(), Vec::new(), vec![tab], Vec::new()] {
            let _ = frame(&ctx, &mut toasts, 0.0, Some(true), events);
        }
        assert!(
            ctx.memory(|memory| memory.focused()).is_some(),
            "Tab reaches the toast's controls"
        );
        let _ = frame(&ctx, &mut toasts, 20.0, Some(true), Vec::new());
        assert_eq!(toasts.queue.len(), 1, "the focused notice is held");

        ctx.memory_mut(|memory| {
            if let Some(focused) = memory.focused() {
                memory.surrender_focus(focused);
            }
        });
        // Held through 21 s, so it leaves at 21 s plus its whole lifetime.
        let _ = frame(&ctx, &mut toasts, 21.0, Some(true), Vec::new());
        let _ = frame(
            &ctx,
            &mut toasts,
            21.0 + TOAST_LIFETIME - 0.1,
            Some(true),
            Vec::new(),
        );
        assert_eq!(toasts.queue.len(), 1, "the clock resumed where it stood");
        let _ = frame(
            &ctx,
            &mut toasts,
            21.0 + TOAST_LIFETIME + 0.1,
            Some(true),
            Vec::new(),
        );
        assert!(toasts.queue.is_empty());
    }

    /// A confirmation is a glance, a warning a second look, and an error
    /// stays until the reader answers it.
    #[test]
    fn a_toast_lives_as_long_as_its_severity_asks() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut toasts = Toasts::default();
        toasts.success(&ctx, "Project saved", "");
        toasts.warn(&ctx, "2 models were skipped");
        toasts.error_with_title(&ctx, "Export failed", "The folder is read-only.");
        let kinds = |toasts: &Toasts| {
            toasts
                .queue
                .iter()
                .map(|toast| toast.kind)
                .collect::<Vec<_>>()
        };

        pass(&ctx, &mut toasts, TOAST_LIFETIME + 0.1);
        assert_eq!(kinds(&toasts), [ToastKind::Warn, ToastKind::Error]);
        pass(&ctx, &mut toasts, TOAST_WARN_LIFETIME - 0.1);
        assert_eq!(kinds(&toasts), [ToastKind::Warn, ToastKind::Error]);
        pass(&ctx, &mut toasts, TOAST_WARN_LIFETIME + 0.1);
        assert_eq!(kinds(&toasts), [ToastKind::Error]);
        pass(&ctx, &mut toasts, 3_600.0);
        assert_eq!(
            kinds(&toasts),
            [ToastKind::Error],
            "an error leaves when it is answered, not when time passes"
        );
        assert_eq!(
            toasts.queue[0].remaining(3_600.0),
            None,
            "so it draws no clock"
        );

        let error = toasts.queue[0].id;
        assert!(toasts.dismiss(error));
        assert!(toasts.queue.is_empty());
    }

    /// Three confirmations arriving behind an error push confirmations off
    /// the stack, never the error; only a stack of errors gives up its oldest.
    #[test]
    fn a_full_stack_drops_its_oldest_notice_that_is_not_an_error() {
        let ctx = Context::default();
        let mut toasts = Toasts::default();
        toasts.error_with_title(&ctx, "Save failed", "The disk is full.");
        for index in 0..3 {
            toasts.info(&ctx, format!("plotted {index}"));
        }
        let shown = |toasts: &Toasts| {
            toasts
                .queue
                .iter()
                .map(|toast| toast.message.clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            shown(&toasts),
            ["The disk is full.", "plotted 1", "plotted 2"],
            "the error stays on screen"
        );

        for index in 0..3 {
            toasts.error_with_title(&ctx, format!("Export {index} failed"), "");
        }
        assert_eq!(
            toasts
                .queue
                .iter()
                .map(|toast| toast.title.as_str())
                .collect::<Vec<_>>(),
            ["Export 0 failed", "Export 1 failed", "Export 2 failed"],
            "three errors make room by the oldest error"
        );
        assert_eq!(
            toasts.activity().len(),
            7,
            "every notice dropped from the stack keeps its record"
        );
    }

    #[test]
    fn reduced_motion_skips_the_toast_entry_fade() {
        assert_eq!(toast_entry_opacity(0.0, false), 1.0);
        assert_eq!(toast_entry_opacity(TOAST_ENTER / 2.0, false), 1.0);
        assert_eq!(toast_entry_opacity(TOAST_ENTER / 2.0, true), 0.5);
    }

    /// A screen reader hears a toast arrive, politely for a confirmation and
    /// at once for a warning or failure, and the reader's focus stays where
    /// it was.
    #[test]
    fn a_toast_is_announced_by_severity_without_taking_focus() {
        for (kind, live) in [
            (ToastKind::Success, egui::accesskit::Live::Polite),
            (ToastKind::Info, egui::accesskit::Live::Polite),
            (ToastKind::Warn, egui::accesskit::Live::Assertive),
            (ToastKind::Error, egui::accesskit::Live::Assertive),
        ] {
            let ctx = Context::default();
            crate::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let mut toasts = Toasts::default();
            toasts.notify_with_action(
                &ctx,
                NotificationCategory::Job,
                kind,
                "Run 4",
                "Finished.",
                NotificationAction::OpenRunInResults { run_sequence: 4 },
            );
            let mut output = None;
            for _ in 0..3 {
                output = Some(frame(&ctx, &mut toasts, 0.0, Some(true), Vec::new()));
            }
            let update = output
                .and_then(|output| output.platform_output.accesskit_update)
                .expect("accesskit is on");
            let statuses: Vec<_> = update
                .nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Status)
                .map(|(_, node)| (node.label().map(str::to_owned), node.live()))
                .collect();
            assert_eq!(
                statuses,
                [(
                    Some(format!("{}: Run 4. Finished.", kind.label())),
                    Some(live)
                )],
                "{kind:?}"
            );
            assert_eq!(
                ctx.memory(|memory| memory.focused()),
                None,
                "a {kind:?} toast never takes the keyboard"
            );
        }
    }

    /// A console paragraph is one card, not a page: the message stops at four
    /// lines with an ellipsis, and the whole of it is under the pointer.
    #[test]
    fn a_long_message_is_cut_to_four_lines_and_given_in_full_on_hover() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut toasts = Toasts::default();
        let message = "Could not write amplifier.rsp: the folder is read-only. ".repeat(8);
        toasts.error_with_title(&ctx, "Save failed", message.clone());
        let texts = |output: &egui::FullOutput| {
            fn walk(shape: &egui::Shape, out: &mut Vec<Arc<Galley>>) {
                match shape {
                    egui::Shape::Text(text) => out.push(text.galley.clone()),
                    egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
                    _ => {}
                }
            }
            let mut out = Vec::new();
            for clipped in &output.shapes {
                walk(&clipped.shape, &mut out);
            }
            out
        };

        // Past the entrance fade, which paints nothing at its first instant.
        let _ = frame(&ctx, &mut toasts, 0.0, Some(true), Vec::new());
        let painted = texts(&frame(&ctx, &mut toasts, 0.5, Some(true), Vec::new()));
        let detail = painted
            .iter()
            .find(|galley| galley.text().starts_with("Could not write"))
            .expect("the message is painted");
        assert_eq!(detail.rows.len(), TOAST_MESSAGE_MAX_ROWS);
        assert!(detail.elided, "and says it was cut");

        // Rest the pointer on the card, clear of its controls, until the
        // tooltip is due.
        let card = ctx
            .memory(|memory| memory.area_rect(Id::new("rspice.toasts")))
            .expect("the stack is laid out");
        let pointer = card.left_top() + vec2(60.0, 40.0);
        let _ = frame(
            &ctx,
            &mut toasts,
            0.6,
            Some(true),
            vec![egui::Event::PointerMoved(pointer)],
        );
        let mut hovered = Vec::new();
        for time in [1.4, 2.2, 3.0] {
            hovered = texts(&frame(&ctx, &mut toasts, time, Some(true), Vec::new()));
        }
        let full = format!("Save failed\n{message}");
        assert!(
            hovered.iter().any(|galley| galley.text() == full),
            "the hover gives the title and the whole message"
        );
    }
}
