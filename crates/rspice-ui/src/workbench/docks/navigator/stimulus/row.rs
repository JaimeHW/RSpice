//! The two rows the library browser is made of.
//!
//! Both are painted rather than assembled from labels. A definition row carries
//! a waveform, two registers of text and a state dot inside one click target,
//! and a label laid over a click rect eats the press that was meant for the row.
//! Selection is the active fill plus the two point accent bar every other list
//! in the product uses, never a coloured box.

use egui::{Rect, Sense, Ui, Vec2, vec2};

use super::Row;
use crate::properties::source_preview;
use crate::state::stimulus_library::definition::StimulusFamily;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// A family heading, and one definition under it.
///
/// The definition row carries two registers, which is also the room a 20 point
/// mini needs with air either side of it.
const GROUP_HEIGHT: f32 = 24.0;
pub(super) const ROW_HEIGHT: f32 = 34.0;
/// Where a heading's mark and a row's mini both start, so the two rows read as
/// one column rather than as a heading with a list beside it.
pub(super) const INDENT: f32 = 20.0;
/// Air between the mini and the text that names it.
pub(super) const TEXT_GAP: f32 = 8.0;
/// The inset every row keeps at its right edge.
pub(super) const RIGHT_INSET: f32 = 8.0;
/// The state mark's diameter and the column it sits in.
const DOT: f32 = 7.0;
pub(super) const DOT_TRACK: f32 = 14.0;
/// The twist arrow's half-size, and where its column ends.
const TWIST: f32 = 3.2;
/// The family mark on a heading.
const MARK: f32 = 14.0;

/// One family heading: its mark, its keyword, and how many definitions it
/// holds. Returns whether the reader asked to fold or unfold it.
pub(super) fn group_row(
    ui: &mut Ui,
    family: StimulusFamily,
    definitions: usize,
    expanded: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), GROUP_HEIGHT), Sense::click());
    let announcement = format!("{} \u{b7} {definitions}", family.label());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::CollapsingHeader, true, &announcement)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(expanded);
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    let painter = ui.painter();
    let centre = egui::pos2(rect.left() + 11.0, rect.center().y);
    let points = if expanded {
        vec![
            centre + vec2(-TWIST, -TWIST * 0.5),
            centre + vec2(TWIST, -TWIST * 0.5),
            centre + vec2(0.0, TWIST * 0.9),
        ]
    } else {
        vec![
            centre + vec2(-TWIST * 0.5, -TWIST),
            centre + vec2(TWIST * 0.9, 0.0),
            centre + vec2(-TWIST * 0.5, TWIST),
        ]
    };
    painter.add(egui::Shape::convex_polygon(
        points,
        t.color.text_faint,
        egui::Stroke::NONE,
    ));

    let mark = Rect::from_min_size(
        egui::pos2(rect.left() + INDENT, rect.center().y - MARK * 0.5),
        Vec2::splat(MARK),
    );
    source_preview::paint_family_mark(painter, mark, family, t.color.text_dim);
    painter.text(
        egui::pos2(mark.right() + 7.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        family.label(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    painter.text(
        egui::pos2(rect.right() - RIGHT_INSET, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        definitions.to_string(),
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
}

/// One definition: the waveform it saved, its name, what it is, and whether the
/// editor is holding an unapplied draft of it.
pub(super) fn definition_row(ui: &mut Ui, row: &Row<'_>, selected: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), ROW_HEIGHT), Sense::click());
    let announcement = announcement(row);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            true,
            selected,
            &announcement,
        )
    });
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
        ui.painter().rect_filled(
            Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    let mini = Rect::from_min_size(
        egui::pos2(
            rect.left() + INDENT,
            rect.center().y - source_preview::MINI_SIZE.y * 0.5,
        ),
        source_preview::MINI_SIZE,
    );
    source_preview::paint_mini(ui.painter(), &t, mini, row.curve, row.family, true);
    if row.dirty {
        state_mark(
            ui,
            row,
            &t,
            egui::pos2(
                rect.right() - RIGHT_INSET - DOT_TRACK * 0.5,
                rect.top() + 12.0,
            ),
        );
    }

    let text_left = mini.right() + TEXT_GAP;
    let revision = format!("r{}", row.revision);
    let revision_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let revision_width = measured(ui, &revision, revision_font.clone());
    // A definition with nothing to say in its second register takes the whole
    // row for its name rather than leaving a band of nothing under it: a
    // refused DC level has no key figure and no adopters, and an empty line is
    // a fact the reader looks for and does not find.
    let (name_band, meta_band) = if row.meta.is_empty() {
        ((rect.top() + 9.0, rect.bottom() - 9.0), None)
    } else {
        (
            (rect.top() + 4.0, rect.top() + 18.0),
            Some((rect.bottom() - 17.0, rect.bottom() - 3.0)),
        )
    };
    painted_run(
        ui,
        Rect::from_min_max(
            egui::pos2(text_left, name_band.0),
            egui::pos2(
                rect.right()
                    - RIGHT_INSET
                    - if row.dirty { DOT_TRACK } else { 0.0 }
                    - if meta_band.is_some() {
                        0.0
                    } else {
                        revision_width + 8.0
                    },
                name_band.1,
            ),
        ),
        &row.name,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
        false,
    );
    let revision_band = meta_band.unwrap_or(name_band);
    painted_run(
        ui,
        Rect::from_min_max(
            egui::pos2(rect.right() - RIGHT_INSET - revision_width, revision_band.0),
            egui::pos2(rect.right() - RIGHT_INSET, revision_band.1),
        ),
        &revision,
        revision_font,
        t.color.text_faint,
        true,
    );
    if let Some(meta_band) = meta_band {
        painted_run(
            ui,
            Rect::from_min_max(
                egui::pos2(text_left, meta_band.0),
                egui::pos2(
                    (rect.right() - RIGHT_INSET - revision_width - 8.0).max(text_left + 1.0),
                    meta_band.1,
                ),
            ),
            &row.meta,
            theme::mono(tokens::FS_MICRO, FontWeight::Regular),
            t.color.text_faint,
            false,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(announcement)
}

/// The dot that says the editor is holding an unapplied draft of this
/// definition, and whether that draft is one the deck would refuse.
///
/// A saved card the engine will not write is deliberately not marked here: the
/// mini already carries that warning, in the slot the curve would have been in,
/// and a second mark on the same row for the same fact is a column a reader has
/// to learn twice.
fn state_mark(ui: &Ui, row: &Row<'_>, t: &Tokens, centre: egui::Pos2) {
    ui.painter().circle_filled(
        centre,
        DOT * 0.5,
        if row.refused {
            t.color.err
        } else {
            t.color.warn
        },
    );
}

/// The whole row as one sentence: what a reader working by ear is told, and
/// what a reader whose name was elided sees under the pointer.
fn announcement(row: &Row<'_>) -> String {
    let mut line = format!("{}, r{}", row.name, row.revision);
    if !row.meta.is_empty() {
        line.push_str(", ");
        line.push_str(&row.meta);
    }
    if row.dirty {
        line.push_str(if row.refused {
            ", draft refused"
        } else {
            ", draft not applied"
        });
    }
    if let Err(refusal) = row.curve {
        line.push_str(". ");
        line.push_str(refusal);
    }
    line
}

/// One run of text, vertically centred in `rect` and elided into its width.
fn painted_run(
    ui: &mut Ui,
    rect: Rect,
    text: &str,
    font: egui::FontId,
    color: egui::Color32,
    right_aligned: bool,
) {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_owned(), font, color);
    job.wrap.max_width = rect.width().max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.overflow_character = Some('\u{2026}');
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let x = if right_aligned {
        rect.right() - galley.size().x
    } else {
        rect.left()
    };
    ui.painter().galley(
        egui::pos2(x, rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
}

/// How wide one run lays out, for placing what follows it.
fn measured(ui: &mut Ui, text: &str, font: egui::FontId) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE)
            .size()
            .x
    })
}
