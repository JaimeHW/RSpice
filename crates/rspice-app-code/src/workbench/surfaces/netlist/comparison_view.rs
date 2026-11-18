//! The comparison tab's body: the lines of two texts as one list, each
//! marked as both texts' line, the first's only (−) or the second's only
//! (+), with its number in each text it is in. A quiet row stands for the
//! lines between two changed regions that neither shows. The tab and the
//! inspector name the two texts, so the list carries no headers of its own.

use egui::{Align2, Color32, Rect, Sense, Stroke, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::workbench::documents::editor_font;

/// One row of a comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DiffRow<'a> {
    /// A line both texts have, by its number in each.
    Both {
        left: usize,
        right: usize,
        text: &'a str,
    },
    /// A line only the first text has.
    Removed { left: usize, text: &'a str },
    /// A line only the second text has.
    Added { right: usize, text: &'a str },
    /// The lines between two changed regions, which neither shows.
    Gap,
}

/// The rows of `diff`, a unified diff, each with the one-based line of the
/// diff it was read from: the line the outline's changed regions go to.
/// The two lines naming the texts are not rows, and a region's header is
/// a gap between it and the region before.
pub(crate) fn diff_rows(diff: &str) -> Vec<(usize, DiffRow<'_>)> {
    let mut rows = Vec::new();
    let (mut left, mut right) = (1, 1);
    let mut in_region = false;
    for (index, line) in diff.lines().enumerate() {
        let at = index + 1;
        if let Some(header) = line.strip_prefix("@@") {
            if in_region {
                rows.push((at, DiffRow::Gap));
            }
            in_region = true;
            (left, right) = region_starts(header).unwrap_or((left, right));
            continue;
        }
        // Before the first region are only the lines naming the two texts.
        if !in_region {
            continue;
        }
        if let Some(text) = line.strip_prefix('+') {
            rows.push((at, DiffRow::Added { right, text }));
            right += 1;
        } else if let Some(text) = line.strip_prefix('-') {
            rows.push((at, DiffRow::Removed { left, text }));
            left += 1;
        } else if line.starts_with('\\') {
            // "\ No newline at end of file" says nothing a reader needs.
        } else {
            let text = line.strip_prefix(' ').unwrap_or(line);
            rows.push((at, DiffRow::Both { left, right, text }));
            left += 1;
            right += 1;
        }
    }
    rows
}

/// Where a region's lines start in each text: ` -12,4 +13,5 @@` gives
/// (12, 13). A side the region holds no line of starts at the line before
/// its place, which is no line at all for the first.
fn region_starts(header: &str) -> Option<(usize, usize)> {
    let mut fields = header.split_whitespace();
    let start = |field: Option<&str>, sign: char| -> Option<usize> {
        field?.strip_prefix(sign)?.split(',').next()?.parse().ok()
    };
    let left = start(fields.next(), '-')?;
    let right = start(fields.next(), '+')?;
    Some((left.max(1), right.max(1)))
}

/// Draw the comparison `diff` in the rest of `ui`, and bring the row read
/// from diff line `reveal` into view when the outline asked for one.
pub(crate) fn show(ui: &mut Ui, diff: &str, reveal: Option<usize>) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let rows = diff_rows(diff);
    let size = editor_font::text_size(ui.ctx());
    let font = theme::mono(size, FontWeight::Regular);
    let row_height = editor_font::line_height(size);
    let char_width = ui
        .painter()
        .layout_no_wrap("0".to_owned(), font.clone(), Color32::PLACEHOLDER)
        .size()
        .x
        .max(1.0);
    let highest = rows
        .iter()
        .map(|(_, row)| match *row {
            DiffRow::Both { left, right, .. } => left.max(right),
            DiffRow::Removed { left, .. } => left,
            DiffRow::Added { right, .. } => right,
            DiffRow::Gap => 0,
        })
        .max()
        .unwrap_or(1);
    let digits = highest.to_string().len().max(3) as f32;
    let number_column = digits * char_width + 2.0 * NUMBER_PADDING;
    let gutter = 2.0 * number_column + MARK_COLUMN;
    let widest = rows
        .iter()
        .map(|(_, row)| match row {
            DiffRow::Both { text, .. }
            | DiffRow::Removed { text, .. }
            | DiffRow::Added { text, .. } => text.chars().count(),
            DiffRow::Gap => 0,
        })
        .max()
        .unwrap_or(0) as f32;
    let content_width = gutter + CODE_PADDING + widest * char_width + CODE_PADDING;

    let mut area = egui::ScrollArea::both()
        .id_salt("workbench.netlist.comparison")
        .auto_shrink([false, false]);
    if let Some(line) = reveal
        && let Some(index) = rows.iter().position(|(at, _)| *at >= line)
    {
        area = area.vertical_scroll_offset((index as f32 * row_height - row_height).max(0.0));
    }
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        area.show_rows(ui, row_height, rows.len(), |ui, range| {
            let width = content_width.max(ui.available_width());
            for (_, row) in &rows[range] {
                let (rect, _) = ui.allocate_exact_size(vec2(width, row_height), Sense::hover());
                if !ui.is_rect_visible(rect) {
                    continue;
                }
                let painter = ui.painter();
                let gutter_right = rect.left() + gutter;
                let (left, right, mark, text, wash, ink) = match *row {
                    DiffRow::Both { left, right, text } => {
                        (Some(left), Some(right), "", text, None, c.text_faint)
                    }
                    DiffRow::Removed { left, text } => (
                        Some(left),
                        None,
                        "\u{2212}",
                        text,
                        Some(c.err.gamma_multiply(WASH)),
                        c.err,
                    ),
                    DiffRow::Added { right, text } => (
                        None,
                        Some(right),
                        "+",
                        text,
                        Some(c.ok.gamma_multiply(WASH)),
                        c.ok,
                    ),
                    DiffRow::Gap => {
                        painter.rect_filled(rect, 0.0, c.bg_panel);
                        let rule = Stroke::new(1.0, c.border);
                        painter.hline(rect.x_range(), rect.top() + 0.5, rule);
                        painter.hline(rect.x_range(), rect.bottom() - 0.5, rule);
                        painter.text(
                            egui::pos2(rect.left() + number_column, rect.center().y),
                            Align2::CENTER_CENTER,
                            "\u{22ef}",
                            font.clone(),
                            c.text_faint,
                        );
                        continue;
                    }
                };
                if let Some(wash) = wash {
                    painter.rect_filled(rect, 0.0, wash);
                }
                for (number, column_right) in [
                    (left, rect.left() + number_column),
                    (right, rect.left() + 2.0 * number_column),
                ] {
                    if let Some(number) = number {
                        painter.text(
                            egui::pos2(column_right - NUMBER_PADDING, rect.center().y),
                            Align2::RIGHT_CENTER,
                            number.to_string(),
                            font.clone(),
                            c.text_faint,
                        );
                    }
                }
                painter.text(
                    egui::pos2(gutter_right - MARK_COLUMN / 2.0, rect.center().y),
                    Align2::CENTER_CENTER,
                    mark,
                    font.clone(),
                    ink,
                );
                painter.vline(gutter_right, rect.y_range(), Stroke::new(1.0, c.border));
                painter
                    .with_clip_rect(Rect::from_min_max(
                        egui::pos2(gutter_right + 1.0, rect.top()),
                        rect.right_bottom(),
                    ))
                    .text(
                        egui::pos2(gutter_right + CODE_PADDING, rect.center().y),
                        Align2::LEFT_CENTER,
                        text,
                        font.clone(),
                        c.text,
                    );
            }
        });
    });
}

/// The air either side of a line number in its column.
const NUMBER_PADDING: f32 = 6.0;
/// The column the + and − marks stand in.
const MARK_COLUMN: f32 = 16.0;
/// The air between the gutter and the text.
const CODE_PADDING: f32 = 8.0;
/// How strongly a line only one text has is washed with its mark's colour.
const WASH: f32 = 0.12;

#[cfg(test)]
mod tests {
    use super::*;

    const DIFF: &str = "--- amp.cir on disk\n+++ amp.cir in RSpice\n@@ -1,3 +1,3 @@\n * amp\n-R1 in out 10k\n+R1 in out 22k\n C1 out 0 1n\n@@ -10,2 +10,3 @@\n .tran 1u 5m\n+.probe v(out)\n .end\n\\ No newline at end of file\n";

    /// The lines naming the two texts are not rows; each line is numbered
    /// in the texts it is in; and the second region is set off from the
    /// first by a gap, its numbers starting where its header says.
    #[test]
    fn a_comparison_numbers_each_line_in_both_texts_and_drops_the_headers() {
        let rows = diff_rows(DIFF)
            .into_iter()
            .map(|(_, row)| row)
            .collect::<Vec<_>>();
        assert_eq!(
            rows,
            [
                DiffRow::Both {
                    left: 1,
                    right: 1,
                    text: "* amp"
                },
                DiffRow::Removed {
                    left: 2,
                    text: "R1 in out 10k"
                },
                DiffRow::Added {
                    right: 2,
                    text: "R1 in out 22k"
                },
                DiffRow::Both {
                    left: 3,
                    right: 3,
                    text: "C1 out 0 1n"
                },
                DiffRow::Gap,
                DiffRow::Both {
                    left: 10,
                    right: 10,
                    text: ".tran 1u 5m"
                },
                DiffRow::Added {
                    right: 11,
                    text: ".probe v(out)"
                },
                DiffRow::Both {
                    left: 11,
                    right: 12,
                    text: ".end"
                },
            ]
        );
    }

    /// A row remembers the diff line it was read from, which is where the
    /// outline's changed regions go.
    #[test]
    fn each_row_keeps_the_line_of_the_diff_it_was_read_from() {
        let lines = diff_rows(DIFF)
            .into_iter()
            .map(|(at, _)| at)
            .collect::<Vec<_>>();
        assert_eq!(lines, [4, 5, 6, 7, 8, 9, 10, 11]);
    }
}
