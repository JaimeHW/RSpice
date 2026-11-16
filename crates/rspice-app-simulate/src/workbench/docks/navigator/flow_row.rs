//! The Verify navigator's flow rows: one view a row, its name, what it holds
//! and its state, led by its problem mark when it has one.

use super::verify::VerificationFlowPresentation;
use super::*;
use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::paint_problem_mark;

/// A flow row's height, and where its status line sits, for `detail_lines`
/// wrapped lines of detail and a status line or none.
///
/// The label, then the detail, then the status, each on the row's own pitch; a
/// line the flow has nothing to say on is not drawn and takes no height.
pub(super) fn flow_row_geometry(detail_lines: usize, has_status: bool) -> (f32, f32) {
    let detail_top = |line: usize| FLOW_DETAIL_TOP + line as f32 * FLOW_DETAIL_LINE_HEIGHT;
    let status_top = match detail_lines {
        0 => FLOW_DETAIL_TOP,
        lines => FLOW_STATUS_TOP + (lines - 1) as f32 * FLOW_DETAIL_LINE_HEIGHT,
    };
    let last_top = if has_status {
        status_top
    } else {
        detail_top(detail_lines.saturating_sub(1))
    };
    (last_top + (FLOW_ROW_HEIGHT - FLOW_STATUS_TOP), status_top)
}

pub(super) fn flow_row(
    ui: &mut Ui,
    flow: &VerificationFlowPresentation,
    selected: bool,
) -> egui::Response {
    let VerificationFlowPresentation {
        label,
        detail,
        status,
        problem,
    } = flow;
    let t = Tokens::get(ui.ctx());
    let text_width = (ui.available_width() - FLOW_TEXT_LEFT - 9.0).max(1.0);
    let detail_galley = ui.painter().layout(
        detail.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        text_width,
    );
    let detail_lines = if detail.is_empty() {
        0
    } else {
        detail_galley.rows.len().max(1)
    };
    let (row_height, status_top) = flow_row_geometry(detail_lines, !status.is_empty());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        let spoken = [label.as_str(), detail.as_str(), status.as_str()]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(". ");
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            true,
            selected,
            problem.map_or_else(|| spoken.clone(), |problem| problem.announce(&spoken)),
        )
    });
    if selected || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(rect.min, egui::pos2(rect.left() + 2.0, rect.bottom())),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    // A row with nothing to say under its name keeps its siblings' height and
    // centres the name in it, rather than leaving the space under it blank.
    let label_galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    let label_top = if detail_lines == 0 && status.is_empty() {
        ((row_height - label_galley.size().y) / 2.0).round()
    } else {
        FLOW_LABEL_TOP
    };
    // The leading slot holds the view's problem, in its own mark and colour,
    // and nothing when it has none. There is no tinted disc behind it.
    let lead = egui::pos2(rect.left() + 17.5, rect.top() + label_top + 6.5);
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            egui::Rect::from_center_size(lead, egui::Vec2::splat(11.0)),
            *problem,
        );
    }
    let text_left = rect.left() + FLOW_TEXT_LEFT;
    let text_right = rect.right() - 9.0;
    let clip = egui::Rect::from_x_y_ranges(text_left..=text_right, rect.y_range());
    let painter = ui.painter().with_clip_rect(clip);
    painter.galley(
        egui::pos2(text_left, rect.top() + label_top),
        label_galley,
        t.color.text,
    );
    if detail_lines > 0 {
        painter.galley(
            egui::pos2(text_left, rect.top() + FLOW_DETAIL_TOP),
            detail_galley,
            t.color.text_faint,
        );
    }
    if !status.is_empty() {
        painter.text(
            egui::pos2(text_left, rect.top() + status_top),
            egui::Align2::LEFT_TOP,
            status,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

#[cfg(test)]
mod tests;
