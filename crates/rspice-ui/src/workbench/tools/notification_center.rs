//! The session notification panel: what happened while the reader was looking
//! elsewhere, hung from the bell that counts it.
//!
//! A panel rather than a dialog. Checking on a finished run is a glance, and a
//! modal made it an errand: the workbench went away, a manager-sized window
//! arrived, and it had to be closed again by hand. The panel opens under the
//! bell, leaves the workbench in view, and goes away on Escape, on a press
//! anywhere else, or on the bell again.
//!
//! Every row is drawn in the grammar its toast was (`ui::widgets::notice`), so
//! a notice reads the same arriving as it does looked up an hour later. The
//! source of truth is the real toast stream; nothing here manufactures
//! activity, and nothing outlives the application session.

use std::sync::Arc;
use std::time::Duration;

use egui::text::{LayoutJob, TextFormat, TextWrapping};
use egui::{
    Align, Area, Color32, CornerRadius, Frame, Galley, Id, Layout, Order, Pos2, Rect, Response,
    Sense, Stroke, Ui, UiBuilder, Vec2, pos2, vec2,
};

use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::notice;
use crate::ui::widgets::{
    NotificationAction, NotificationCategory, NotificationRecord, SWITCH_WIDTH, Toasts,
    mark_response_disabled, paint_switch,
};
use crate::workbench::chrome::title_bar;
use crate::workbench::design_system::{WorkbenchIcon, elide_text};
use crate::workbench::state::NotificationFilter;
use crate::workbench::{AppState, RSpiceApp};

const PANEL_WIDTH: f32 = 420.0;
const PANEL_MAX_HEIGHT: f32 = 560.0;
const PANEL_RADIUS: u8 = 8;
/// Clearance kept between the panel and every screen edge.
const SCREEN_MARGIN: f32 = 8.0;
/// Clearance under the panel on a desktop, so it never reads as docked to the
/// status bar it stops short of.
const DESKTOP_BOTTOM_CLEARANCE: f32 = 24.0;
/// Bell's lower edge to the panel's upper edge. The caret lives in this gap.
const ANCHOR_GAP: f32 = 8.0;
/// How far the panel's trailing edge runs past the bell's, so the caret lands
/// under the bell rather than on the panel's corner.
const ANCHOR_OVERHANG: f32 = 6.0;
const CARET_HALF_WIDTH: f32 = 6.0;
const CARET_HEIGHT: f32 = 6.0;
const PHONE_BREAKPOINT: f32 = 560.0;
const LARGE_TARGET_MAX_WIDTH: f32 = 820.0;

const HEAD_HEIGHT: f32 = 40.0;
const FOOT_HEIGHT: f32 = 34.0;
const BAND_PADDING_LEFT: f32 = 14.0;
const BAND_PADDING_RIGHT: f32 = 8.0;
const QUIET_ACTION_HEIGHT: f32 = 26.0;
const QUIET_ACTION_PADDING_X: f32 = 8.0;

const FILTER_PADDING_Y: f32 = 8.0;
const FILTER_PADDING_LEFT: f32 = 12.0;
const FILTER_PADDING_RIGHT: f32 = 10.0;
const SEGMENT_HEIGHT: f32 = 24.0;
const SEGMENT_PADDING_X: f32 = 10.0;
const SEGMENT_COUNT_GAP: f32 = 6.0;
const SEGMENT_TRACK_PADDING: f32 = 2.0;
const SWITCH_LABEL_GAP: f32 = 7.0;
const SWITCH_PADDING_X: f32 = 6.0;

const GROUP_HEIGHT: f32 = 27.0;
const GROUP_TRACKING: f32 = 0.6;
const ROW_PADDING_LEFT: f32 = 14.0;
const ROW_PADDING_RIGHT: f32 = 12.0;
const ROW_PADDING_TOP: f32 = 9.0;
const ROW_PADDING_BOTTOM: f32 = 10.0;
const ROW_COLUMN_GAP: f32 = 10.0;
/// A fixed line box. The dismiss mark is taller than the timestamp it takes
/// the place of, and a row that grew under the pointer would nudge every row
/// below it.
const ROW_LINE_HEIGHT: f32 = 20.0;
const ROW_LINE_GAP: f32 = 8.0;
const ROW_TEXT_GAP: f32 = 2.0;
const ROW_UNREAD_RAIL: f32 = 2.0;
const MESSAGE_LINE_HEIGHT: f32 = 16.0;
const MESSAGE_MAX_ROWS: usize = 2;
const OFFER_GAP: f32 = 3.0;

const EMPTY_PADDING_TOP: f32 = 40.0;
const EMPTY_PADDING_BOTTOM: f32 = 44.0;
const EMPTY_MARK_SIDE: f32 = 26.0;
const EMPTY_TEXT_WIDTH: f32 = 270.0;

/// Notices newer than this are "Recent". A time group rather than an unread
/// one, so marking a row read never moves it out from under the pointer.
const RECENT_WINDOW: f64 = 15.0 * 60.0;
const ENTER_TIME: f32 = 0.12;
const ENTER_RISE: f32 = 4.0;
/// How often an open panel wakes to re-word its ages.
const AGE_REFRESH: Duration = Duration::from_secs(20);

/// Whether the notification surfaces raise their controls to touch size. The
/// rule the title bar applies to the bell they hang from.
pub(in crate::workbench) fn large_targets(ctx: &egui::Context, state: &AppState) -> bool {
    ctx.content_rect().width() <= LARGE_TARGET_MAX_WIDTH || state.workbench.coarse_pointer
}

/// What the reader asked for during one pass. Collected while the records are
/// borrowed for drawing and applied once they are not.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Intent {
    MarkRead(u64),
    Dismiss(u64),
    Follow(u64, NotificationAction),
    MarkAllRead,
    ClearRead,
}

/// The panel's two view axes, copied out of the session for the pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PanelView {
    filter: NotificationFilter,
    unread_only: bool,
}

pub(in crate::workbench) fn show(ctx: &egui::Context, app: &mut RSpiceApp, chrome_bottom: f32) {
    let was_open_id = panel_id().with("was-open");
    let enter_id = panel_id().with("enter");
    if !app.state.workbench.notification_center_showing() {
        ctx.data_mut(|data| data.insert_temp(was_open_id, false));
        // Walk the entrance back to zero while closed, so the next opening
        // has somewhere to animate from.
        let _ = ctx.animate_bool_with_time(enter_id, false, 0.0);
        return;
    }
    let opening = !ctx
        .data(|data| data.get_temp::<bool>(was_open_id))
        .unwrap_or(false);
    ctx.data_mut(|data| data.insert_temp(was_open_id, true));

    let anchor = title_bar::notification_anchor(ctx);
    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
        app.state.workbench.close_notification_center();
        if let Some((_, bell)) = anchor {
            ctx.memory_mut(|memory| memory.request_focus(bell));
        }
        return;
    }

    let large = large_targets(ctx, &app.state);
    let placement = PanelPlacement::resolve(
        ctx.content_rect(),
        anchor.map(|(rect, _)| rect),
        chrome_bottom,
    );
    if placement.width <= 0.0 || placement.max_height <= 0.0 {
        return;
    }
    let entered = if ctx.global_style().animation_time > 0.0 {
        ctx.animate_bool_with_time(enter_id, true, ENTER_TIME)
    } else {
        1.0
    };
    if opening && !ctx.input(|input| input.pointer.any_click()) {
        // Opened from the keyboard: start the reader inside the panel rather
        // than leaving Tab to walk the whole workbench to reach it.
        let selected = NotificationFilter::ALL
            .iter()
            .position(|filter| *filter == app.state.workbench.notification_filter)
            .unwrap_or(0);
        ctx.memory_mut(|memory| memory.request_focus(segment_id(selected)));
    }
    ctx.request_repaint_after(AGE_REFRESH);

    let now = ctx.input(|input| input.time);
    let mut view = PanelView {
        filter: app.state.workbench.notification_filter,
        unread_only: app.state.workbench.notification_unread_only,
    };
    let mut intents = Vec::new();
    let records = app.state.ui.toasts.activity();

    let area = Area::new(panel_id())
        .order(Order::Foreground)
        .fixed_pos(placement.min + vec2(0.0, (entered - 1.0) * ENTER_RISE))
        .interactable(true)
        .show(ctx, |ui| {
            ui.set_opacity(entered);
            let t = Tokens::get(ui.ctx());
            let frame = Frame::NONE
                .fill(t.color.bg_panel)
                .stroke(Stroke::new(1.0, t.color.border_strong))
                .corner_radius(PANEL_RADIUS)
                .shadow(t.shadow())
                .show(ui, |ui| {
                    ui.set_width((placement.width - 2.0).max(0.0));
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    panel_contents(
                        ui,
                        &mut view,
                        records,
                        now,
                        large,
                        placement.max_height - 2.0,
                        &mut intents,
                    );
                });
            if let Some(caret_x) = placement.caret_x {
                paint_caret(ui, frame.response.rect, caret_x);
            }
        });
    ctx.accesskit_node_builder(area.response.id, |node| {
        node.set_role(egui::accesskit::Role::Dialog);
        node.set_label("Notifications");
    });

    app.state.workbench.notification_filter = view.filter;
    app.state.workbench.notification_unread_only = view.unread_only;

    let mut followed = None;
    for intent in intents {
        let toasts = &mut app.state.ui.toasts;
        match intent {
            Intent::MarkRead(id) => {
                toasts.mark_read(id);
            }
            Intent::Dismiss(id) => {
                toasts.dismiss(id);
            }
            Intent::MarkAllRead => toasts.mark_all_read(),
            Intent::ClearRead => toasts.clear_read(),
            Intent::Follow(id, action) => {
                // Following a notice is having read it.
                toasts.mark_read(id);
                followed = Some(action);
            }
        }
    }
    if let Some(action) = followed {
        // The offer leaves, so the panel goes with it. Routed after the pass
        // so the hop mutates the session once the records' borrow is over.
        app.state.workbench.close_notification_center();
        crate::workbench::commands::result_navigation::perform_notification_action(app, action);
        return;
    }
    // The press that opened the panel landed on the bell, which is elsewhere.
    if !opening && area.response.clicked_elsewhere() {
        app.state.workbench.close_notification_center();
    }
}

fn panel_id() -> Id {
    Id::new("rspice.notification-panel")
}

fn segment_id(index: usize) -> Id {
    panel_id().with(("segment", index))
}

/// Where the panel goes, resolved from the screen and the bell alone.
#[derive(Debug, Clone, Copy, PartialEq)]
struct PanelPlacement {
    min: Pos2,
    width: f32,
    max_height: f32,
    /// Where the caret points, when there is a bell for it to point at and
    /// the panel's straight upper edge reaches under it.
    caret_x: Option<f32>,
}

impl PanelPlacement {
    fn resolve(screen: Rect, anchor: Option<Rect>, chrome_bottom: f32) -> Self {
        let phone = screen.width() <= PHONE_BREAKPOINT;
        let room = (screen.width() - SCREEN_MARGIN * 2.0).max(0.0);
        let width = if phone { room } else { PANEL_WIDTH.min(room) };
        let top = anchor.map_or(screen.top() + chrome_bottom, |bell| bell.bottom()) + ANCHOR_GAP;
        let right = if phone {
            screen.right() - SCREEN_MARGIN
        } else {
            anchor
                .map_or(screen.right() - SCREEN_MARGIN, |bell| {
                    bell.right() + ANCHOR_OVERHANG
                })
                .clamp(
                    screen.left() + SCREEN_MARGIN + width,
                    screen.right() - SCREEN_MARGIN,
                )
        };
        let max_height = if phone {
            // A sheet: it may take the whole height under the chrome.
            screen.bottom() - SCREEN_MARGIN - top
        } else {
            PANEL_MAX_HEIGHT.min(screen.bottom() - DESKTOP_BOTTOM_CLEARANCE - top)
        }
        .max(0.0);
        let left = right - width;
        let radius = f32::from(PANEL_RADIUS);
        let caret_x = anchor.map(|bell| bell.center().x).filter(|x| {
            *x - CARET_HALF_WIDTH >= left + radius && *x + CARET_HALF_WIDTH <= right - radius
        });
        Self {
            min: pos2(left, top),
            width,
            max_height,
            caret_x,
        }
    }
}

/// The caret is the panel's own edge turned up toward the bell: the head's
/// fill, the panel's hairline, and a stroke of fill that opens the border
/// where the two meet.
fn paint_caret(ui: &Ui, panel: Rect, caret_x: f32) {
    let t = Tokens::get(ui.ctx());
    let base_y = panel.top() + 1.0;
    let left = pos2(caret_x - CARET_HALF_WIDTH, base_y);
    let right = pos2(caret_x + CARET_HALF_WIDTH, base_y);
    let tip = pos2(caret_x, base_y - CARET_HEIGHT);
    ui.painter().add(egui::Shape::convex_polygon(
        vec![left, tip, right],
        t.color.bg_panel_2,
        Stroke::NONE,
    ));
    ui.painter().add(egui::Shape::line(
        vec![left, tip, right],
        Stroke::new(1.0, t.color.border_strong),
    ));
    ui.painter().line_segment(
        [
            pos2(left.x + 1.0, panel.top() + 0.5),
            pos2(right.x - 1.0, panel.top() + 0.5),
        ],
        Stroke::new(1.0, t.color.bg_panel_2),
    );
}

fn panel_contents(
    ui: &mut Ui,
    view: &mut PanelView,
    records: &[NotificationRecord],
    now: f64,
    large: bool,
    max_height: f32,
    intents: &mut Vec<Intent>,
) {
    let unread = records.iter().filter(|record| !record.is_read()).count();
    let has_read = records.iter().any(NotificationRecord::is_read);
    let top = ui.cursor().top();

    head(ui, unread, large, intents);
    filters(ui, view, records, large);

    let foot_height = band_height(FOOT_HEIGHT, large);
    let used = ui.cursor().top() - top + foot_height;
    let visible: Vec<&NotificationRecord> = records
        .iter()
        .filter(|record| {
            filter_includes(view.filter, record.category())
                && (!view.unread_only || !record.is_read())
        })
        .collect();
    if visible.is_empty() {
        empty_state(ui, records.is_empty(), view.unread_only);
    } else {
        notice_list(
            ui,
            &visible,
            now,
            large,
            (max_height - used).max(0.0),
            intents,
        );
    }
    foot(ui, records.len(), has_read, large, intents);
}

/// A band's height: its authored height on a pointer, room for a touch target
/// plus the band's own breathing space under a finger.
fn band_height(authored: f32, large: bool) -> f32 {
    if large {
        tokens::TOUCH_TARGET + 8.0
    } else {
        authored
    }
}

fn head(ui: &mut Ui, unread: usize, large: bool, intents: &mut Vec<Intent>) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), band_height(HEAD_HEIGHT, large)),
        Sense::hover(),
    );
    let inner_radius = PANEL_RADIUS - 1;
    ui.painter().rect_filled(
        rect,
        CornerRadius {
            nw: inner_radius,
            ne: inner_radius,
            sw: 0,
            se: 0,
        },
        t.color.bg_panel_2,
    );

    let title = ui.painter().layout_no_wrap(
        "Notifications".to_owned(),
        theme::sans(tokens::FS_2, FontWeight::SemiBold),
        t.color.text,
    );
    let title_pos = pos2(
        rect.left() + BAND_PADDING_LEFT,
        rect.center().y - title.size().y * 0.5,
    );
    let pill_left = title_pos.x + title.size().x + 8.0;
    ui.painter().galley(title_pos, title, t.color.text);

    let action = quiet_action(
        ui,
        panel_id().with("mark-all-read"),
        pos2(rect.right() - BAND_PADDING_RIGHT, rect.center().y),
        "Mark all read",
        unread > 0,
        large,
    );
    if action.clicked() {
        intents.push(Intent::MarkAllRead);
    }

    if unread > 0 {
        let label = ui.painter().layout_no_wrap(
            format!("{unread} unread"),
            theme::mono(tokens::FS_MICRO, FontWeight::SemiBold),
            t.color.accent,
        );
        let pill = Rect::from_min_size(
            pos2(pill_left, rect.center().y - (label.size().y + 4.0) * 0.5),
            label.size() + vec2(12.0, 4.0),
        );
        // Dropped rather than squeezed when a narrow sheet has no room for
        // it: the bell's badge and the rows' rails already say the same.
        if pill.right() + 8.0 <= action.rect.left() {
            ui.painter()
                .rect_filled(pill, 9.0, t.color.accent.gamma_multiply(0.14));
            ui.painter()
                .galley(pill.min + vec2(6.0, 2.0), label, t.color.accent);
        }
    }
}

/// A borderless text action for the panel's head and foot. A framed button in
/// a 40-point band reads as the band's subject; these are its footnotes.
fn quiet_action(
    ui: &Ui,
    id: Id,
    right_center: Pos2,
    label: &str,
    enabled: bool,
    large: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        Color32::PLACEHOLDER,
    );
    let height = if large {
        tokens::TOUCH_TARGET
    } else {
        QUIET_ACTION_HEIGHT
    };
    let rect = Rect::from_min_max(
        pos2(
            right_center.x - galley.size().x - QUIET_ACTION_PADDING_X * 2.0,
            right_center.y - height * 0.5,
        ),
        pos2(right_center.x, right_center.y + height * 0.5),
    );
    let mut response = ui.interact(
        rect,
        id,
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if !enabled {
        mark_response_disabled(&mut response);
    }
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));

    let lit = enabled && (response.hovered() || response.has_focus());
    if lit {
        let plate = Rect::from_center_size(
            rect.center(),
            vec2(rect.width(), QUIET_ACTION_HEIGHT.min(rect.height())),
        );
        ui.painter().rect_filled(plate, t.radius, t.color.bg_hover);
    }
    let color = if !enabled {
        t.color.text_faint.gamma_multiply(0.55)
    } else if lit {
        t.color.text
    } else {
        t.color.text_dim
    };
    ui.painter().galley(
        pos2(
            rect.left() + QUIET_ACTION_PADDING_X,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        color,
    );
    theme::paint_focus_ring(ui, &response, rect);
    if enabled {
        response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }
    response
}

fn filters(ui: &mut Ui, view: &mut PanelView, records: &[NotificationRecord], large: bool) {
    let t = Tokens::get(ui.ctx());
    // Touch size is a taller press, not a taller control: the track is drawn
    // the same at every size and only what answers a finger grows, into the
    // padding the band already has.
    let hit_height = if large {
        tokens::TOUCH_TARGET
    } else {
        SEGMENT_HEIGHT
    };
    let (rect, _) = ui.allocate_exact_size(
        vec2(
            ui.available_width(),
            (SEGMENT_HEIGHT + SEGMENT_TRACK_PADDING * 2.0 + FILTER_PADDING_Y * 2.0).max(hit_height),
        ),
        Sense::hover(),
    );
    let rule = Stroke::new(1.0, t.color.border);
    ui.painter().hline(rect.x_range(), rect.top() + 0.5, rule);
    ui.painter()
        .hline(rect.x_range(), rect.bottom() - 0.5, rule);

    let segments_right = filter_segments(ui, rect, view, records, hit_height);
    let room = rect.right() - FILTER_PADDING_RIGHT - segments_right - 8.0;
    unread_only_switch(ui, rect, view, room, hit_height);
}

/// The domain filter: one of these, and here are all of them, each with how
/// many it holds so an empty one can be seen to be empty before it is chosen.
/// Returns the group's trailing edge.
fn filter_segments(
    ui: &Ui,
    band: Rect,
    view: &mut PanelView,
    records: &[NotificationRecord],
    hit_height: f32,
) -> f32 {
    let t = Tokens::get(ui.ctx());
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let count_font = theme::mono(tokens::FS_MICRO, FontWeight::Medium);
    let cells: Vec<_> = NotificationFilter::ALL
        .into_iter()
        .map(|filter| {
            let count = records
                .iter()
                .filter(|record| filter_includes(filter, record.category()))
                .count();
            let label = ui.painter().layout_no_wrap(
                filter.label().to_owned(),
                label_font.clone(),
                Color32::PLACEHOLDER,
            );
            let count_text = ui.painter().layout_no_wrap(
                count.to_string(),
                count_font.clone(),
                Color32::PLACEHOLDER,
            );
            let width =
                SEGMENT_PADDING_X * 2.0 + label.size().x + SEGMENT_COUNT_GAP + count_text.size().x;
            (filter, count, label, count_text, width)
        })
        .collect();
    let track = Rect::from_min_size(
        pos2(
            band.left() + FILTER_PADDING_LEFT,
            band.center().y - SEGMENT_HEIGHT * 0.5 - SEGMENT_TRACK_PADDING,
        ),
        vec2(
            cells.iter().map(|cell| cell.4).sum::<f32>() + SEGMENT_TRACK_PADDING * 2.0,
            SEGMENT_HEIGHT + SEGMENT_TRACK_PADDING * 2.0,
        ),
    );
    ui.painter().rect(
        track,
        6.0,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let group = ui.interact(track, panel_id().with("segments"), Sense::hover());
    ui.ctx().accesskit_node_builder(group.id, |node| {
        node.set_role(egui::accesskit::Role::RadioGroup);
        node.set_label("Show");
    });

    let mut focused = None;
    let mut left = track.left() + SEGMENT_TRACK_PADDING;
    for (index, (filter, count, label, count_text, width)) in cells.into_iter().enumerate() {
        let cell = Rect::from_min_size(
            pos2(left, track.top() + SEGMENT_TRACK_PADDING),
            vec2(width, SEGMENT_HEIGHT),
        );
        left += width;
        let press = Rect::from_center_size(cell.center(), vec2(width, hit_height));
        let response = ui.interact(press, segment_id(index), Sense::click());
        let selected = view.filter == filter;
        let announced = format!("{}, {count}", filter.label());
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::RadioButton,
                ui.is_enabled(),
                selected,
                &announced,
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::RadioButton);
            node.set_selected(selected);
        });
        if response.clicked() {
            view.filter = filter;
            response.request_focus();
        }
        if response.has_focus() {
            focused = Some(index);
        }

        let lit = selected || response.hovered() || response.has_focus();
        if selected {
            // Lifted, not tinted: the chosen segment is a key cap standing
            // proud of the track the others sit in.
            ui.painter().rect(
                cell,
                4.0,
                t.color.bg_elevated,
                Stroke::new(1.0, t.color.border_strong),
                egui::StrokeKind::Inside,
            );
        }
        let text_y = |galley: &Arc<Galley>| cell.center().y - galley.size().y * 0.5;
        let label_x = cell.left() + SEGMENT_PADDING_X;
        let count_x = label_x + label.size().x + SEGMENT_COUNT_GAP;
        let label_pos = pos2(label_x, text_y(&label));
        let count_pos = pos2(count_x, text_y(&count_text) + 0.5);
        ui.painter().galley(
            label_pos,
            label,
            if lit { t.color.text } else { t.color.text_dim },
        );
        ui.painter().galley(
            count_pos,
            count_text,
            if selected {
                t.color.text_dim
            } else {
                t.color.text_faint
            },
        );
        theme::paint_focus_ring(ui, &response, cell);
    }

    // A radio group is walked with the arrows, and choosing follows focus.
    if let Some(index) = focused {
        let step = ui.input(|input| {
            let forward =
                input.key_pressed(egui::Key::ArrowRight) || input.key_pressed(egui::Key::ArrowDown);
            let back =
                input.key_pressed(egui::Key::ArrowLeft) || input.key_pressed(egui::Key::ArrowUp);
            i32::from(forward) - i32::from(back)
        });
        if step != 0 {
            let count = NotificationFilter::ALL.len() as i32;
            let next = (index as i32 + step).rem_euclid(count) as usize;
            view.filter = NotificationFilter::ALL[next];
            ui.memory_mut(|memory| memory.request_focus(segment_id(next)));
        }
    }
    track.right()
}

/// "Unread only" is a second axis, so it is a switch beside the filter and not
/// a fourth segment inside it: unread crosses Jobs and System, it does not
/// stand next to them.
fn unread_only_switch(ui: &Ui, band: Rect, view: &mut PanelView, room: f32, hit_height: f32) {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let measure = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), Color32::PLACEHOLDER)
    };
    let natural =
        |label_width: f32| SWITCH_PADDING_X * 2.0 + SWITCH_WIDTH + SWITCH_LABEL_GAP + label_width;
    // The words give way before the control does.
    let full = measure("Unread only");
    let label = if natural(full.size().x) <= room {
        Some(full)
    } else {
        Some(measure("Unread")).filter(|short| natural(short.size().x) <= room)
    };
    let width = label
        .as_ref()
        .map_or(SWITCH_PADDING_X * 2.0 + SWITCH_WIDTH, |label| {
            natural(label.size().x)
        });
    let rect = Rect::from_min_max(
        pos2(
            band.right() - FILTER_PADDING_RIGHT - width,
            band.center().y - hit_height * 0.5,
        ),
        pos2(
            band.right() - FILTER_PADDING_RIGHT,
            band.center().y + hit_height * 0.5,
        ),
    );
    let response = ui
        .interact(rect, panel_id().with("unread-only"), Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    let on = view.unread_only;
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            on,
            "Unread only",
        )
    });
    if response.clicked() {
        view.unread_only = !on;
    }
    let lit = response.hovered() || response.has_focus();
    paint_switch(
        ui,
        pos2(
            rect.left() + SWITCH_PADDING_X + SWITCH_WIDTH * 0.5,
            rect.center().y,
        ),
        on,
        lit,
        ui.clip_rect(),
    );
    if let Some(label) = label {
        ui.painter().galley(
            pos2(
                rect.left() + SWITCH_PADDING_X + SWITCH_WIDTH + SWITCH_LABEL_GAP,
                rect.center().y - label.size().y * 0.5,
            ),
            label,
            if on || lit {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
    }
    let ring = Rect::from_center_size(rect.center(), vec2(rect.width(), SEGMENT_HEIGHT));
    theme::paint_focus_ring(ui, &response, ring);
}

const fn filter_includes(filter: NotificationFilter, category: NotificationCategory) -> bool {
    matches!(filter, NotificationFilter::All)
        || matches!(
            (filter, category),
            (NotificationFilter::Jobs, NotificationCategory::Job)
                | (NotificationFilter::System, NotificationCategory::System)
        )
}

fn is_recent(now: f64, created: f64) -> bool {
    now - created < RECENT_WINDOW
}

fn notice_list(
    ui: &mut Ui,
    visible: &[&NotificationRecord],
    now: f64,
    large: bool,
    max_height: f32,
    intents: &mut Vec<Intent>,
) {
    // The bar floats over the rows instead of taking a gutter from them: a
    // gutter is a stripe of panel fill down the trailing edge of every row.
    ui.spacing_mut().scroll = egui::style::ScrollStyle::floating();
    // Partitioned by age rather than cut at the first old record: a console
    // line is mirrored with the time it was written, which can put an older
    // record above a newer one, and a cut there would file both as old.
    let (recent, earlier): (Vec<_>, Vec<_>) = visible
        .iter()
        .copied()
        .partition(|record| is_recent(now, record.created()));
    let groups = [("Recent", recent), ("Earlier this session", earlier)];
    let mut extents = Vec::with_capacity(groups.len());

    let scroll = egui::ScrollArea::vertical()
        .id_salt("notification-panel-list")
        .max_height(max_height)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let list = ui.scope(|ui| {
                for (label, records) in groups {
                    if records.is_empty() {
                        continue;
                    }
                    let top = ui.cursor().top();
                    group_heading(ui, label, None);
                    for (index, record) in records.iter().enumerate() {
                        notice_row(ui, record, now, index == 0, large, intents);
                    }
                    extents.push((label, top, ui.cursor().top()));
                }
            });
            ui.ctx().accesskit_node_builder(list.response.id, |node| {
                node.set_role(egui::accesskit::Role::List);
                node.set_label("Notifications from this session");
            });
        });

    // The heading of whichever group crosses the top of the viewport stays
    // there, and is pushed out by the next group's heading as it arrives.
    let viewport = scroll.inner_rect;
    if let Some((label, _, bottom)) = extents
        .iter()
        .find(|(_, top, bottom)| *top < viewport.top() - 0.5 && *bottom > viewport.top())
    {
        let band_top = viewport.top().min(bottom - GROUP_HEIGHT);
        let band = Rect::from_min_size(
            pos2(viewport.left(), band_top),
            vec2(viewport.width(), GROUP_HEIGHT),
        );
        let mut pinned = ui.new_child(UiBuilder::new().max_rect(band));
        pinned.set_clip_rect(viewport.intersect(ui.clip_rect()));
        group_heading(&mut pinned, label, Some(band));
        // Swallow presses: the row scrolled under the heading is not what a
        // reader pressing the heading means.
        ui.interact(band, panel_id().with("pinned-group"), Sense::click());
    }
}

/// A time group's heading. `pinned` repaints it over the rows at a fixed
/// place, with a rule under it to part it from what scrolls beneath.
fn group_heading(ui: &mut Ui, label: &str, pinned: Option<Rect>) {
    let t = Tokens::get(ui.ctx());
    let rect = pinned.unwrap_or_else(|| {
        ui.allocate_exact_size(vec2(ui.available_width(), GROUP_HEIGHT), Sense::hover())
            .0
    });
    if pinned.is_some() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
        ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            Stroke::new(1.0, t.color.border.gamma_multiply(0.65)),
        );
    }
    let galley = ui.painter().layout_job(LayoutJob::single_section(
        label.to_uppercase(),
        TextFormat {
            font_id: theme::sans(tokens::FS_MICRO, FontWeight::SemiBold),
            extra_letter_spacing: GROUP_TRACKING,
            color: t.color.text_faint,
            ..Default::default()
        },
    ));
    ui.painter().galley(
        pos2(
            rect.left() + ROW_PADDING_LEFT,
            rect.bottom() - 5.0 - galley.size().y,
        ),
        galley,
        t.color.text_faint,
    );
}

/// "now", then minutes, then hours. No "ago": the column is already a column
/// of ages, and the word would be the widest thing in it.
fn age_label(now: f64, created: f64) -> String {
    let age = (now - created).max(0.0);
    if age < 60.0 {
        "now".to_owned()
    } else if age < 3_600.0 {
        format!("{} min", (age / 60.0).floor() as u64)
    } else {
        format!("{} hr", (age / 3_600.0).floor() as u64)
    }
}

fn message_galley(ui: &Ui, message: &str, width: f32) -> Arc<Galley> {
    let mut job = LayoutJob::single_section(
        message.to_owned(),
        TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            line_height: Some(MESSAGE_LINE_HEIGHT),
            color: Color32::PLACEHOLDER,
            ..Default::default()
        },
    );
    // One clause, two lines at most. The whole sentence is a hover away, and
    // a console paragraph must not turn one row into the list.
    job.wrap = TextWrapping {
        max_width: width,
        max_rows: MESSAGE_MAX_ROWS,
        break_anywhere: false,
        overflow_character: Some('\u{2026}'),
    };
    ui.painter().layout_job(job)
}

fn notice_row(
    ui: &mut Ui,
    record: &NotificationRecord,
    now: f64,
    first_in_group: bool,
    large: bool,
    intents: &mut Vec<Intent>,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let unread = !record.is_read();
    let body_offset = ROW_PADDING_LEFT + notice::GLYPH_SIDE + ROW_COLUMN_GAP;
    let body_width = (width - body_offset - ROW_PADDING_RIGHT).max(0.0);

    let message =
        (!record.message().is_empty()).then(|| message_galley(ui, record.message(), body_width));
    let offer_height = if large {
        tokens::TOUCH_TARGET
    } else {
        notice::OFFER_HEIGHT
    };
    let height = ROW_PADDING_TOP
        + ROW_LINE_HEIGHT
        + message
            .as_ref()
            .map_or(0.0, |galley| ROW_TEXT_GAP + galley.size().y)
        + record.action().map_or(0.0, |_| OFFER_GAP + offer_height)
        + ROW_PADDING_BOTTOM;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());

    let dismiss_id = panel_id().with(("dismiss", record.id()));
    let dismiss_focused = ui.memory(|memory| memory.has_focus(dismiss_id));
    let lit = response.contains_pointer() || response.has_focus() || dismiss_focused;
    let line = Rect::from_min_size(
        pos2(rect.left() + body_offset, rect.top() + ROW_PADDING_TOP),
        vec2(body_width, ROW_LINE_HEIGHT),
    );

    if ui.is_rect_visible(rect) {
        if lit {
            ui.painter().rect_filled(
                rect,
                0.0,
                mix(t.color.bg_panel_2, t.color.bg_elevated, 0.35),
            );
        }
        if !first_in_group {
            ui.painter().hline(
                rect.x_range(),
                rect.top() + 0.5,
                Stroke::new(1.0, t.color.border.gamma_multiply(0.65)),
            );
        }
        if unread {
            // Unread is a rail and a weight, not an opacity: a read row stays
            // fully legible, it simply stops asking.
            ui.painter().rect_filled(
                Rect::from_min_max(
                    rect.left_top(),
                    pos2(rect.left() + ROW_UNREAD_RAIL, rect.bottom()),
                ),
                0.0,
                t.color.accent,
            );
        }
        let tone = notice::tone_color(&t, record.kind());
        notice::paint_tone_glyph(
            ui.painter(),
            Rect::from_min_size(
                pos2(rect.left() + ROW_PADDING_LEFT, line.top() + 2.0),
                Vec2::splat(notice::GLYPH_SIDE),
            ),
            record.kind(),
            if unread {
                tone
            } else {
                mix(tone, t.color.text_faint, 0.55)
            },
        );
    }

    // The age gives way to the dismiss mark under the pointer, so the row
    // never carries a column that is empty until the pointer arrives. Under a
    // finger there is no hover to wait for, and both are shown.
    let when = ui.painter().layout_no_wrap(
        format!(
            "{} \u{00b7} {}",
            record.category().label(),
            age_label(now, record.created())
        ),
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
    );
    let dismiss_shown = large || lit;
    let dismiss_reserve = if large {
        notice::DISMISS_SIDE + 4.0
    } else {
        0.0
    };
    // The title keeps one width whether the age or the mark is showing, so it
    // is not re-elided as the pointer crosses the row.
    let trailing = when.size().x.max(notice::DISMISS_SIDE - 4.0) + dismiss_reserve;
    let title_font = theme::sans(
        tokens::FS_1,
        if unread {
            FontWeight::SemiBold
        } else {
            FontWeight::Medium
        },
    );
    let title_width = (line.width() - trailing - ROW_LINE_GAP).max(0.0);
    let title = elide_text(ui, record.title(), &title_font, title_width);
    let title_elided = title != record.title();
    if ui.is_rect_visible(rect) {
        ui.painter()
            .with_clip_rect(line.intersect(ui.clip_rect()))
            .text(
                line.left_center(),
                egui::Align2::LEFT_CENTER,
                title,
                title_font,
                if unread {
                    t.color.text
                } else {
                    t.color.text_dim
                },
            );
        if large || !lit {
            ui.painter().galley(
                pos2(
                    line.right() - dismiss_reserve - when.size().x,
                    line.center().y - when.size().y * 0.5,
                ),
                when,
                t.color.text_faint,
            );
        }
        if let Some(message) = &message {
            ui.painter().galley(
                pos2(line.left(), line.bottom() + ROW_TEXT_GAP),
                message.clone(),
                if unread {
                    t.color.text_dim
                } else {
                    t.color.text_faint
                },
            );
        }
    }

    if let Some(action) = record.action() {
        let top = rect.bottom() - ROW_PADDING_BOTTOM - offer_height;
        let mut offer = ui.new_child(
            UiBuilder::new()
                .max_rect(Rect::from_min_size(
                    pos2(line.left(), top),
                    vec2(body_width, offer_height),
                ))
                .layout(Layout::left_to_right(Align::Center)),
        );
        if notice::offer_link(&mut offer, action.label(), record.title(), large).clicked() {
            intents.push(Intent::Follow(record.id(), action));
        }
    }
    if dismiss_shown {
        let side = if large {
            tokens::TOUCH_TARGET
        } else {
            notice::DISMISS_SIDE
        };
        let center = pos2(
            line.right() + 4.0 - notice::DISMISS_SIDE * 0.5,
            line.center().y,
        );
        let target = Rect::from_center_size(center, Vec2::splat(side));
        if notice::dismiss_button(
            ui,
            target,
            dismiss_id,
            &format!("Dismiss {}", record.title()),
        )
        .clicked()
        {
            intents.push(Intent::Dismiss(record.id()));
        }
    }

    if response.clicked() && unread {
        intents.push(Intent::MarkRead(record.id()));
    }
    if response.has_focus()
        && ui.input(|input| {
            input.key_pressed(egui::Key::Delete) || input.key_pressed(egui::Key::Backspace)
        })
    {
        intents.push(Intent::Dismiss(record.id()));
    }
    if response.has_focus() && ui.is_rect_visible(rect) {
        ui.painter().rect_stroke(
            rect,
            0.0,
            Stroke::new(1.0, t.color.accent),
            egui::StrokeKind::Inside,
        );
    }
    let announced = format!(
        "{} {}{}: {}. {}",
        record.category().label(),
        record.kind().label(),
        if unread { ", unread" } else { "" },
        record.title(),
        record.message(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &announced)
    });
    let message_elided = message.as_ref().is_some_and(|galley| galley.elided);
    if title_elided || message_elided {
        // Whatever the row cut short is one hover away, in full.
        response.on_hover_text(format!("{}\n{}", record.title(), record.message()));
    }
}

/// What an empty list says depends on why it is empty: nothing has happened,
/// or something has and the view is hiding it.
fn empty_copy(no_records: bool, unread_only: bool) -> (&'static str, &'static str) {
    if no_records {
        (
            "No notifications yet",
            "Finished runs, saves and warnings from this session are kept here.",
        )
    } else if unread_only {
        (
            "Nothing unread",
            "Every notification in this view has been read.",
        )
    } else {
        (
            "Nothing in this view",
            "Choose another filter to see the rest of this session.",
        )
    }
}

fn empty_state(ui: &mut Ui, no_records: bool, unread_only: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let (title, detail) = empty_copy(no_records, unread_only);
    let width = ui.available_width();
    let title = ui.painter().layout_no_wrap(
        title.to_owned(),
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text,
    );
    let mut job = LayoutJob::single_section(
        detail.to_owned(),
        TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            line_height: Some(MESSAGE_LINE_HEIGHT),
            color: t.color.text_dim,
            ..Default::default()
        },
    );
    job.wrap.max_width = EMPTY_TEXT_WIDTH.min((width - 56.0).max(0.0));
    job.halign = Align::Center;
    let detail = ui.painter().layout_job(job);

    let height = EMPTY_PADDING_TOP
        + EMPTY_MARK_SIDE
        + 8.0
        + title.size().y
        + 4.0
        + detail.size().y
        + EMPTY_PADDING_BOTTOM;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let center_x = rect.center().x;
    let mut y = rect.top() + EMPTY_PADDING_TOP;
    WorkbenchIcon::Bell.paint(
        ui.painter(),
        Rect::from_min_size(
            pos2(center_x - EMPTY_MARK_SIDE * 0.5, y),
            Vec2::splat(EMPTY_MARK_SIDE),
        ),
        t.color.text_faint,
    );
    y += EMPTY_MARK_SIDE + 8.0;
    let title_height = title.size().y;
    ui.painter().galley(
        pos2(center_x - title.size().x * 0.5, y),
        title,
        t.color.text,
    );
    y += title_height + 4.0;
    // A centred galley is laid out around its own origin.
    ui.painter()
        .galley(pos2(center_x, y), detail, t.color.text_dim);
    response
}

fn foot(ui: &mut Ui, held: usize, has_read: bool, large: bool, intents: &mut Vec<Intent>) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), band_height(FOOT_HEIGHT, large)),
        Sense::hover(),
    );
    let inner_radius = PANEL_RADIUS - 1;
    ui.painter().rect_filled(
        rect,
        CornerRadius {
            nw: 0,
            ne: 0,
            sw: inner_radius,
            se: inner_radius,
        },
        t.color.bg_panel_2,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.top() + 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let action = quiet_action(
        ui,
        panel_id().with("clear-read"),
        pos2(rect.right() - BAND_PADDING_RIGHT, rect.center().y),
        "Clear read",
        has_read,
        large,
    );
    if action.clicked() {
        intents.push(Intent::ClearRead);
    }

    // Retention is stated where the reader would look for a missing notice:
    // at the end of the list it fell off.
    let font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let room = action.rect.left() - 8.0 - (rect.left() + BAND_PADDING_LEFT);
    let text = elide_text(
        ui,
        &format!(
            "This session \u{00b7} {held} of {} kept",
            Toasts::retention_limit()
        ),
        &font,
        room,
    );
    ui.painter().text(
        pos2(rect.left() + BAND_PADDING_LEFT, rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        font,
        t.color.text_faint,
    );
}

#[cfg(test)]
mod raster;
#[cfg(test)]
mod tests;
