//! The Overview's Project status card: one row per status area with the
//! action that addresses it at the row's right edge, and under them the
//! findings that stop a run, each a link to the Problems page standing on it.

use egui::{Align2, Color32, Sense, Stroke, Ui, pos2, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::table::{
    CELL_PAD, Cell, Column, DataTable, LineHeights, RowKey, TABLE_EDGE_INSET, TableState,
};

use super::super::page::paint_elided;
use super::snapshot::{ProblemSnapshot, StatusSnapshot};
use super::{OverviewIntent, overview_card, paint_tone_mark};

/// Where a row's area name starts: past the mark's gutter.
const NAME_LEFT: f32 = 31.0;
/// The gap between a row's columns.
const GAP: f32 = 9.0;
/// The inset of a row's last column from the card's edge.
const RIGHT_INSET: f32 = 14.0;
/// Below this a row's words go under its name rather than beside it.
const STACKED_BELOW: f32 = 480.0;
/// The room over and under the words of a row that takes several lines.
const ROW_PAD: f32 = 6.0;
/// The most finding rows the strip shows before it scrolls.
const FINDING_ROWS_SHOWN: f32 = 6.0;

pub(super) fn status_card(
    ui: &mut Ui,
    statuses: &[StatusSnapshot],
    problems: &[ProblemSnapshot],
    findings: &mut TableState,
) -> Option<OverviewIntent> {
    // The rows carry their own marks and words; a verdict on the card's head
    // was the same facts once more.
    overview_card(ui, "Project status", "", |ui| {
        let row = status_rows(ui, statuses);
        let finding = finding_rows(ui, problems, findings);
        row.or(finding)
    })
}

fn status_rows(ui: &mut Ui, statuses: &[StatusSnapshot]) -> Option<OverviewIntent> {
    let t = Tokens::get(ui.ctx());
    ui.add_space(5.0);
    let width = ui.available_width().max(1.0);
    let stacked = width < STACKED_BELOW;
    // One line of words is a list row high, and a finger's reach under a
    // coarse pointer.
    let base_height = if t.metrics.is_touch() {
        t.metrics.touch_target()
    } else {
        t.metrics.row_h
    };
    let name_font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let words_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let line = ui.fonts_mut(|fonts| fonts.row_height(&words_font));
    // One name column for every row, as wide as the widest name: the words
    // start in one place down the card.
    let name_width = statuses
        .iter()
        .map(|status| text_width(ui, status.area, &name_font))
        .fold(96.0_f32, f32::max)
        .min(width * 0.4);
    let action_width = statuses
        .iter()
        .map(|status| text_width(ui, status.action, &words_font))
        .fold(0.0_f32, f32::max);
    let action_left = width - RIGHT_INSET - action_width;
    let (words_left, words_width) = if stacked {
        (NAME_LEFT, (width - NAME_LEFT - RIGHT_INSET).max(1.0))
    } else {
        let left = NAME_LEFT + name_width + GAP;
        (left, (action_left - GAP - left).max(1.0))
    };
    let mut requested = None;
    for (index, status) in statuses.iter().enumerate() {
        let (name_color, words_color, action_color) = if status.enabled {
            (t.color.text, t.color.text_dim, t.color.text_dim)
        } else {
            (t.color.text_faint, t.color.text_faint, t.color.text_faint)
        };
        // The words wrap rather than being cut: on a phone the row grows and
        // every word of it stays readable without a pointer to hover.
        let words =
            ui.painter()
                .layout(status.text(), words_font.clone(), words_color, words_width);
        // A row of one line centres it; a row whose words take more lines
        // starts them under a pad and grows to hold them.
        let multi = stacked || words.rows.len() > 1;
        let height = match (multi, stacked) {
            (false, _) => base_height,
            (true, false) => (ROW_PAD + words.size().y + ROW_PAD).max(base_height),
            (true, true) => (ROW_PAD + line + 4.0 + words.size().y + ROW_PAD).max(base_height),
        };
        let (rect, response) = ui.allocate_exact_size(
            vec2(width, height),
            if status.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        let hovered = status.enabled && (response.hovered() || response.has_focus());
        if hovered {
            ui.painter()
                .rect_filled(rect.shrink2(vec2(8.0, 2.0)), 4.0, t.color.bg_panel_2);
        }
        // The first line's middle: the name, the action and the mark sit on
        // it whatever the words under or beside them take.
        let first_line = if multi {
            rect.top() + ROW_PAD + line * 0.5
        } else {
            rect.center().y
        };
        // Only a row the reader has to act on is marked; a clean row keeps
        // the gutter, so marked and clean rows keep their columns.
        paint_tone_mark(ui, status.tone, rect.left() + 14.0, first_line);
        paint_elided(
            ui,
            pos2(rect.left() + NAME_LEFT, first_line - line * 0.5),
            status.area,
            name_font.clone(),
            name_color,
            if stacked {
                (action_left - GAP - NAME_LEFT).max(1.0)
            } else {
                name_width
            },
        );
        let words_top = match (multi, stacked) {
            (false, _) => rect.center().y - words.size().y * 0.5,
            (true, false) => first_line - line * 0.5,
            (true, true) => first_line + line * 0.5 + 4.0,
        };
        ui.painter().galley(
            pos2(rect.left() + words_left, words_top),
            words,
            words_color,
        );
        // What the row does, at its right edge, where a click on the row is
        // aimed: the verb was a tooltip only.
        ui.painter().text(
            pos2(rect.right() - RIGHT_INSET, first_line),
            Align2::RIGHT_CENTER,
            status.action,
            words_font.clone(),
            if hovered { t.color.text } else { action_color },
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
            response.on_hover_cursor(egui::CursorIcon::PointingHand)
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
    requested
}

/// The findings that stop a run, under a hairline: what is wrong first,
/// then where, the location column as wide as its longest text. Each row
/// opens the Problems page on its finding.
fn finding_rows(
    ui: &mut Ui,
    problems: &[ProblemSnapshot],
    state: &mut TableState,
) -> Option<OverviewIntent> {
    if problems.is_empty() {
        return None;
    }
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    let (rule, _) =
        ui.allocate_exact_size(vec2(ui.available_width().max(1.0), 1.0), Sense::hover());
    ui.painter().hline(
        (rule.left() + 14.0)..=(rule.right() - 14.0),
        rule.top(),
        Stroke::new(1.0, t.color.border),
    );
    ui.add_space(4.0);
    let width = ui.available_width().max(1.0);
    let cell_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let location = problems
        .iter()
        .map(|problem| text_width(ui, &problem.path, &cell_font))
        .fold(0.0_f32, f32::max)
        + CELL_PAD
        + TABLE_EDGE_INSET
        + 2.0;
    let columns = [
        Column::name("Finding", 120.0),
        Column::words("Location", location.min(width * 0.4)),
    ];
    let keys = problems
        .iter()
        .enumerate()
        .map(|(index, problem)| {
            RowKey::Text(problem.finding.map_or_else(
                || format!("{index}:{}", problem.path),
                |finding| finding.to_string(),
            ))
        })
        .collect::<Vec<_>>();
    let rows_tall = LineHeights::list(&t).row * FINDING_ROWS_SHOWN;
    let response = DataTable::new(
        "workbench.project.overview.findings",
        &columns,
        problems.len(),
    )
    .state(state)
    .keys(&keys)
    .freeze(1)
    .max_height(rows_tall)
    .list()
    .show(ui, |index| {
        let problem = &problems[index];
        let message = problem.tone.problem().map_or_else(
            || Cell::text(&problem.message),
            |mark| Cell::problem(&problem.message, mark),
        );
        vec![message.hint(&problem.message), Cell::text(&problem.path)]
    });
    response
        .activate
        .or(response.clicked)
        .and_then(|index| problems.get(index))
        .map(|problem| OverviewIntent::ShowFinding {
            message: problem.message.clone(),
            finding: problem.finding,
        })
}

fn text_width(ui: &Ui, text: &str, font: &egui::FontId) -> f32 {
    ui.painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
}
