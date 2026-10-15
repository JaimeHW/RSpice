//! The card this definition emits, and what the engine will say about it.
//!
//! One band, because both halves answer one question: what comes out of this
//! record. On the left is the definition's own card — the exact text adoption
//! copies, between named rather than resolved nets, because a library
//! definition has no sheet. On the right is what `source_contract` says about
//! it, at the two strengths that module speaks at: a refusal is something the
//! deck would refuse, and it blocks Apply; an advisory is something the card
//! does without saying so, and it does not. Inventing a third tally here would
//! mean inventing an owner for it, and the whole point of reading
//! `source_contract` rather than restating it is that there is one.
//!
//! This band used to list every adopter under that card, and to head itself
//! with a count. The inspector lists the adopters instead — all of them rather
//! than the three this band had room for, with the same card, the same
//! provenance, the same two verbs and a third this band could not reach — and
//! the identity band already names which of them have drifted, instance by
//! instance, beside the count. Neither the verdict word nor its tally is
//! written here either: the identity band's lifecycle chip states `saved · rN`
//! or `draft · N errors` in the same three colours, two bands up.
//!
//! It is the last band on purpose. A reader who has changed a field looks
//! down, and the answer is in the same place every time.

use egui::{Rect, Sense, Ui, UiBuilder, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{
    StatusMark, WorkbenchIcon, card_well, icon_button, paint_status_mark,
};
use super::{Finding, Stage, StageAction};

/// The band's first line: the card, and the first thing said about it.
const HEAD_HEIGHT: f32 = 28.0;
/// Each finding after the first.
const LINE_HEIGHT: f32 = 22.0;
/// How many findings after the first the band shows before it scrolls. The
/// band is a verdict, not a log: three lines answer "what is wrong", and a
/// fourth is one scroll away rather than forty points off the plot.
const VISIBLE_EXTRA_LINES: usize = 2;
const INSET: f32 = 10.0;
const BUTTON: Vec2 = Vec2::new(24.0, 20.0);
/// Air between the card and the verb that copies it.
const CARD_GAP: f32 = 6.0;

/// The column the verdict and every finding under it start at.
///
/// A fixed share of the band rather than whatever the card left over: the
/// findings beneath the first have to start where the first one does, and a
/// column measured from the card would move every time a field was typed.
fn verdict_left(rect: Rect) -> f32 {
    let width = (rect.width() * 0.45).clamp(160.0, 340.0);
    (rect.right() - width).max(rect.left())
}

/// How tall the band is for this definition's findings.
pub(super) fn content_height(stage: &Stage) -> f32 {
    HEAD_HEIGHT
        + stage
            .findings
            .len()
            .saturating_sub(1)
            .min(VISIBLE_EXTRA_LINES) as f32
            * LINE_HEIGHT
}

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let messages = state.ui.messages();
    let band = ui.available_rect_before_wrap();
    let verdict = verdict_left(band);
    ui.spacing_mut().item_spacing.y = 0.0;

    let (head, _) = ui.allocate_exact_size(Vec2::new(band.width(), HEAD_HEIGHT), Sense::hover());
    let mut row = ui.new_child(
        UiBuilder::new()
            .id_salt("workbench.stimulus.footer.card")
            .max_rect(head)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    row.set_clip_rect(ui.clip_rect().intersect(Rect::from_min_max(
        head.min,
        egui::pos2(verdict, head.bottom()),
    )));
    row.add_space(INSET);
    // The card is elided against the track it really has, which is the band
    // less the column the verdict owns — not the whole band, which would run
    // a long card straight under the verdict's first word.
    let card = stage.card.clone().map_err(|errors| errors.join("; "));
    card_well(
        &mut row,
        card.as_deref().map_err(String::as_str),
        band.right() - verdict + BUTTON.x + CARD_GAP,
    );
    if let Ok(text) = &card {
        row.add_space(CARD_GAP);
        if icon_button(
            &mut row,
            WorkbenchIcon::Copy,
            &messages.text(MessageId::StimulusCopyCard),
            false,
            BUTTON,
        )
        .clicked()
        {
            actions.push(StageAction::Copy(text.clone()));
        }
    }

    let satisfied = messages.format(
        MessageId::StimulusAuditSatisfied,
        &[("family", stage.working.family().label())],
    );
    let palette = Tokens::get(ui.ctx()).color;
    let first = Rect::from_min_max(egui::pos2(verdict, head.top()), head.max);
    let mut line = ui.new_child(
        UiBuilder::new()
            .id_salt("workbench.stimulus.footer.verdict")
            .max_rect(first)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    line.set_clip_rect(ui.clip_rect().intersect(first));
    match stage.findings.first() {
        Some(finding) => finding_line(&mut line, finding),
        None => finding_line_with(&mut line, StatusMark::Success, palette.ok, &satisfied),
    }

    let rest = stage.findings.get(1..).unwrap_or_default();
    if rest.is_empty() {
        return;
    }
    egui::ScrollArea::vertical()
        .id_salt("workbench.stimulus.footer")
        .auto_shrink([false, false])
        .max_height(rest.len().min(VISIBLE_EXTRA_LINES) as f32 * LINE_HEIGHT)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            for (index, finding) in rest.iter().enumerate() {
                let (row, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), LINE_HEIGHT),
                    Sense::hover(),
                );
                let row = Rect::from_min_max(egui::pos2(verdict, row.top()), row.max);
                let mut line = ui.new_child(
                    UiBuilder::new()
                        .id_salt(("workbench.stimulus.footer.line", index))
                        .max_rect(row)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );
                line.set_clip_rect(ui.clip_rect().intersect(row));
                finding_line(&mut line, finding);
            }
        });
}

/// One finding, toned by whether it blocks Apply.
fn finding_line(ui: &mut Ui, finding: &Finding) {
    let palette = Tokens::get(ui.ctx()).color;
    let (mark, color) = if finding.blocking {
        (StatusMark::Failure, palette.err)
    } else {
        (StatusMark::Warning, palette.warn)
    };
    finding_line_with(ui, mark, color, &finding.message);
}

/// One finding: its mark, painted as geometry, and what it says.
fn finding_line_with(ui: &mut Ui, mark: StatusMark, color: egui::Color32, message: &str) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
    paint_status_mark(ui.painter(), rect, mark, color);
    ui.add_space(4.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(message)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(Tokens::get(ui.ctx()).color.text_dim),
        )
        .truncate(),
    )
    .on_hover_text(message);
}
