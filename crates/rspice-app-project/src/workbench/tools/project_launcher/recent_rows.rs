//! What a recent-project row offers wherever one is listed: the launcher's
//! Projects page and the no-project landing. A pin that writes the pin, the
//! full path and the time it was last opened on hover or long-press, and,
//! for a file an open found gone, a way to point the row at where it went
//! or take it off the list.

use egui::{Align2, Id, Rect, Response, Sense, Ui, Vec2, WidgetInfo, WidgetType, vec2};

use crate::services::recent_files::RecentFile;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Problem, paint_problem_mark};
use crate::workbench::design_system::WorkbenchIcon;

/// What the reader asked of one recent row, besides opening it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecentRowAction {
    /// Pin the row, or unpin it.
    TogglePin(std::path::PathBuf),
    /// Point the row at the file where it now is, and open it.
    Locate(RecentFile),
    /// Take the row off the list.
    Remove(std::path::PathBuf),
}

/// When a recent project was last opened, on the reader's clock: the list is
/// this computer's own record of what it opened.
pub(crate) fn last_opened(opened_at_unix_ms: u64) -> String {
    if opened_at_unix_ms == 0 {
        return "\u{2014}".to_owned();
    }
    crate::time_compat::local_date_time(opened_at_unix_ms).map_or_else(
        || "\u{2014}".to_owned(),
        |stamp| {
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}",
                stamp.year(),
                u8::from(stamp.month()),
                stamp.day(),
                stamp.hour(),
                stamp.minute()
            )
        },
    )
}

/// Everything a row shortens: its full path, when it was last opened and how
/// long ago, and that the file was not found when it was.
pub(crate) fn full_text(recent: &RecentFile) -> String {
    let mut text = recent.path.display().to_string();
    if recent.opened_at_unix_ms != 0 {
        let now: u64 = crate::time_compat::unix_epoch()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX);
        text.push_str(&format!(
            "\nLast opened {} ({})",
            last_opened(recent.opened_at_unix_ms),
            crate::time_compat::relative_age(std::time::Duration::from_millis(
                now.saturating_sub(recent.opened_at_unix_ms)
            ))
        ));
    }
    if recent.missing {
        text.push_str("\nThe file was not found when RSpice last tried to open it.");
    }
    text
}

/// Show `text` while `response` is hovered, and from a long-press until the
/// next press anywhere: a touch reader has no hover.
pub(crate) fn hover_and_long_press(ui: &Ui, response: Response, text: &str) -> Response {
    let held = Id::new("workbench.recent_rows.long_press");
    if response.long_touched() {
        ui.ctx()
            .data_mut(|data| data.insert_temp(held, response.id));
    } else if ui.input(|input| input.pointer.any_pressed())
        && ui
            .ctx()
            .data(|data| data.get_temp::<Id>(held))
            .is_some_and(|id| id == response.id)
    {
        ui.ctx().data_mut(|data| data.remove::<Id>(held));
    }
    let pressed_open = ui
        .ctx()
        .data(|data| data.get_temp::<Id>(held))
        .is_some_and(|id| id == response.id);
    if pressed_open {
        response.show_tooltip_text(text);
        response
    } else {
        response.on_hover_text(text)
    }
}

/// The side of a row's pin, at the metrics in force.
pub(crate) fn pin_side(ui: &Ui) -> f32 {
    let t = Tokens::get(ui.ctx());
    if t.metrics.is_touch() {
        t.metrics.touch_target()
    } else {
        24.0
    }
}

/// A row's pin, centred in `rect`: lit when the row is pinned. Returns
/// whether it was pressed.
pub(crate) fn pin_toggle(ui: &mut Ui, rect: Rect, pinned: bool, name: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let side = pin_side(ui).min(rect.height()).min(rect.width());
    let rect = Rect::from_center_size(rect.center(), Vec2::splat(side));
    let response = ui.interact(rect, ui.id().with(("recent-pin", name)), Sense::click());
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    WorkbenchIcon::Pin.paint(
        ui.painter(),
        Rect::from_center_size(rect.center(), Vec2::splat(12.0)),
        if pinned {
            t.color.accent
        } else if response.hovered() {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    let label = if pinned {
        format!("Unpin {name}")
    } else {
        format!("Pin {name}")
    };
    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, true, pinned, &label));
    theme::paint_focus_ring(ui, &response, rect);
    let response = response.on_hover_text(if pinned { "Unpin" } else { "Pin to the top" });
    response.clicked()
}

/// The strip under a row whose file was not found: what happened, and the
/// two ways on. Returns the action pressed.
pub(crate) fn missing_strip(
    ui: &mut Ui,
    rect: Rect,
    recent: &RecentFile,
) -> Option<RecentRowAction> {
    let t = Tokens::get(ui.ctx());
    let mut action = None;
    let mut strip = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    strip.spacing_mut().item_spacing.x = 6.0;
    let (mark, _) = strip.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
    paint_problem_mark(strip.painter(), mark, Problem::Warning);
    let message = strip.painter().text(
        mark.right_center() + vec2(6.0, 0.0),
        Align2::LEFT_CENTER,
        "File not found",
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    strip.add_space(message.width() + 6.0);
    strip.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if Button::new("Remove").show(ui).clicked() {
            action = Some(RecentRowAction::Remove(recent.path.clone()));
        }
        if Button::new("Locate\u{2026}").show(ui).clicked() {
            action = Some(RecentRowAction::Locate(recent.clone()));
        }
    });
    action
}

/// The height of the strip [`missing_strip`] draws.
pub(crate) fn missing_strip_height(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.ctl_h + 8.0
}
