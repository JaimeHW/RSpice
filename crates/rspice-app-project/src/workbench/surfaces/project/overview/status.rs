//! The Overview's Project status card: one row per status area with the
//! action that addresses it, and the strip naming the finding that stands out.

use egui::{Align2, Color32, Sense, Stroke, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::page::{elide_text, paint_elided};
use super::snapshot::{ProblemSnapshot, StatusSnapshot};
use super::{OverviewIntent, overview_card, paint_tone_mark};

const STATUS_ROW_HEIGHT: f32 = 27.0;

pub(super) fn status_card(
    ui: &mut Ui,
    statuses: &[StatusSnapshot],
    problem: Option<&ProblemSnapshot>,
) -> Option<OverviewIntent> {
    // The rows carry their own marks and words; a verdict on the card's head
    // was the same facts once more.
    overview_card(ui, "Project status", "", |ui| {
        status_rows(ui, statuses, problem)
    })
}

fn status_rows(
    ui: &mut Ui,
    statuses: &[StatusSnapshot],
    problem: Option<&ProblemSnapshot>,
) -> Option<OverviewIntent> {
    let tokens = Tokens::get(ui.ctx());
    ui.add_space(5.0);
    let width = ui.available_width().max(1.0);
    let mut requested = None;
    for (index, status) in statuses.iter().enumerate() {
        // The reference row is its own grid — `auto minmax(96px, auto)
        // minmax(0, 1fr)` with 9px gaps — so the detail starts just after
        // this row's name, never at a shared fraction of the panel width.
        let name_font = theme::sans(tokens::FS_0, FontWeight::Medium);
        let name_column = ui
            .painter()
            .layout_no_wrap(status.area.to_owned(), name_font.clone(), Color32::WHITE)
            .size()
            .x
            .max(96.0)
            .min(width * 0.5);
        let detail_offset = (31.0 + name_column + 9.0).min((width - 64.0).max(1.0));
        let (rect, response) = ui.allocate_exact_size(
            vec2(width, STATUS_ROW_HEIGHT),
            if status.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        if status.enabled && (response.hovered() || response.has_focus()) {
            ui.painter()
                .rect_filled(rect.shrink2(vec2(8.0, 2.0)), 4.0, tokens.color.bg_panel_2);
        }
        let primary_color = if status.enabled {
            tokens.color.text
        } else {
            tokens.color.text_faint
        };
        let detail_color = if status.enabled {
            tokens.color.text_dim
        } else {
            tokens.color.text_faint
        };
        // Six dots down the card, five of them green, said what the state
        // word beside each one already said. Only a row the reader has to act
        // on is marked, and the gutter the dots held is kept so a marked row
        // and a clean one keep their columns.
        paint_tone_mark(ui, status.tone, rect.left() + 14.0, rect.center().y);
        paint_elided(
            ui,
            pos2(rect.left() + 31.0, rect.center().y - tokens::FS_0 * 0.5),
            status.area,
            name_font,
            primary_color,
            (detail_offset - 40.0).max(1.0),
        );
        paint_elided(
            ui,
            pos2(
                rect.left() + detail_offset,
                rect.center().y - tokens::FS_0 * 0.5,
            ),
            &status.text(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            detail_color,
            (width - detail_offset - 8.0).max(1.0),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                status.enabled,
                format!("{}: {}. {}", status.area, status.text(), status.action),
            )
        });
        theme::paint_focus_ring_outset(ui, &response, rect);
        let response = if status.enabled {
            response
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text(format!("{}\n{}", status.text(), status.action))
        } else {
            response.on_hover_text(status.disabled_reason.unwrap_or("Action unavailable"))
        };
        if status.enabled && response.clicked() {
            requested = Some(status.intent.clone());
        }
        if index + 1 < statuses.len() {
            ui.add_space(3.0);
        }
    }
    let Some(problem) = problem else {
        return requested;
    };
    ui.add_space(4.0);
    let (rect, _) = ui.allocate_exact_size(vec2(width, 20.0), Sense::hover());
    ui.painter().hline(
        (rect.left() + 14.0)..=(rect.right() - 14.0),
        rect.top(),
        Stroke::new(1.0, tokens.color.border),
    );
    // The strip's cells are `auto minmax(0, 1fr) auto` with 9px gaps inside
    // the panel's 14px insets: the code takes its own width, the message the
    // rest, and the path right-aligns against the far inset.
    let code_font = theme::mono(tokens::FS_MICRO, FontWeight::Medium);
    let mark = paint_tone_mark(ui, problem.tone, rect.left() + 14.0, rect.center().y);
    let code_width = ui
        .painter()
        .layout_no_wrap(problem.code.clone(), code_font.clone(), Color32::WHITE)
        .size()
        .x
        .min(width * 0.22);
    let path_width = 128.0_f32.min(width * 0.34);
    paint_elided(
        ui,
        pos2(
            rect.left() + 14.0 + mark,
            rect.center().y - tokens::FS_MICRO * 0.5,
        ),
        &problem.code,
        code_font,
        tokens.color.text_faint,
        code_width.max(1.0),
    );
    paint_elided(
        ui,
        pos2(
            rect.left() + 14.0 + mark + code_width + 9.0,
            rect.center().y - tokens::FS_MICRO * 0.5,
        ),
        &problem.message,
        theme::sans(tokens::FS_MICRO, FontWeight::Regular),
        tokens.color.text_dim,
        (width - 14.0 - mark - code_width - 9.0 - path_width - 9.0 - 14.0).max(1.0),
    );
    let path_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let path = elide_text(ui, &problem.path, &path_font, path_width.max(1.0));
    ui.painter().text(
        pos2(rect.right() - 14.0, rect.center().y),
        Align2::RIGHT_CENTER,
        path,
        path_font,
        tokens.color.text_faint,
    );
    requested
}
