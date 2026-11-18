//! A row's text on the editor's grid of display cells.
//!
//! Every position the editor works out, the caret, a selection, a click, a
//! mark, is a display cell times the width of one. The text is drawn on the
//! same grid: a tab is spaces to the next tab stop, and a glyph its font
//! draws wider or narrower than its cells is followed by the space that puts
//! the next character back on its cell.

use std::ops::Range;

use egui::FontId;
use egui::text::LayoutJob;

use super::super::text_document_model::char_cells;

/// The part `bytes` of `job`, each stretch in the format it has there.
pub(super) fn cut(job: &LayoutJob, bytes: Range<usize>) -> LayoutJob {
    let mut part = LayoutJob {
        text: String::new(),
        sections: Vec::new(),
        ..job.clone()
    };
    for section in &job.sections {
        let start = section.byte_range.start.0.max(bytes.start);
        let end = section.byte_range.end.0.min(bytes.end);
        if start < end {
            part.append(&job.text[start..end], 0.0, section.format.clone());
        }
    }
    part
}

/// `job`, a row's text from display cell `first_cell`, laid on the grid of
/// cells `cell_width` wide. `glyph_width` is how wide a font draws a
/// character.
pub(super) fn fit(
    mut job: LayoutJob,
    first_cell: usize,
    cell_width: f32,
    mut glyph_width: impl FnMut(&FontId, char) -> f32,
) -> LayoutJob {
    if job
        .text
        .bytes()
        .all(|byte| byte.is_ascii() && byte != b'\t')
    {
        return job;
    }
    let text = std::mem::take(&mut job.text);
    let sections = std::mem::take(&mut job.sections);
    let mut cell = first_cell;
    let mut run = String::new();
    // The space that puts the next character on its cell.
    let mut lead = 0.0;
    for section in sections {
        let bytes = section.byte_range.start.0..section.byte_range.end.0;
        for character in text[bytes].chars() {
            let cells = char_cells(character, cell);
            cell += cells;
            if character == '\t' {
                run.extend(std::iter::repeat_n(' ', cells));
            } else if character.is_ascii() {
                run.push(character);
            } else {
                run.push(character);
                job.append(&run, lead, section.format.clone());
                run.clear();
                lead = cells as f32 * cell_width - glyph_width(&section.format.font_id, character);
            }
        }
        if !run.is_empty() {
            job.append(&run, lead, section.format.clone());
            run.clear();
            lead = 0.0;
        }
    }
    job
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job(parts: &[(&str, egui::Color32)]) -> LayoutJob {
        let mut job = LayoutJob::default();
        for (text, color) in parts {
            job.append(
                text,
                0.0,
                egui::TextFormat::simple(FontId::monospace(10.0), *color),
            );
        }
        job
    }

    #[test]
    fn a_tab_is_spaces_to_the_next_stop_from_the_cell_the_row_starts_at() {
        let fitted = fit(job(&[("a\tb", egui::Color32::WHITE)]), 3, 6.0, |_, _| 6.0);
        assert_eq!(fitted.text, "a    b", "cell 3, a at 3, the tab to 8");
    }

    #[test]
    fn a_wide_glyph_is_followed_by_the_space_that_fills_its_two_cells() {
        let fitted = fit(
            job(&[("\u{4E2D}x", egui::Color32::WHITE)]),
            0,
            6.0,
            |_, _| 10.0,
        );
        assert_eq!(fitted.text, "\u{4E2D}x");
        let leads = fitted
            .sections
            .iter()
            .map(|section| section.leading_space)
            .collect::<Vec<_>>();
        assert_eq!(leads, [0.0, 2.0], "two cells of 6 less the glyph's 10");
    }

    #[test]
    fn a_cut_keeps_each_stretch_in_its_format() {
        let whole = job(&[
            ("* a comment", egui::Color32::GRAY),
            (" R1", egui::Color32::WHITE),
        ]);
        let part = cut(&whole, 6..14);
        assert_eq!(part.text, "mment R1");
        assert_eq!(part.sections.len(), 2);
        assert_eq!(part.sections[0].format.color, egui::Color32::GRAY);
        assert_eq!(part.sections[1].format.color, egui::Color32::WHITE);
    }
}
