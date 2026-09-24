//! What the engine will say about this definition, before a run says it.
//!
//! Two strengths, because the module that owns these rules speaks at exactly
//! two: a refusal is something the deck would refuse, and it blocks Apply; an
//! advisory is something the card does without saying so, and it does not.
//! Inventing a third tally here would mean inventing an owner for it, and the
//! whole point of reading `source_contract` rather than restating it is that
//! there is one.
//!
//! The strip is the last band on purpose. A reader who has changed a field
//! looks down, and the verdict is in the same place every time.
//!
//! It is one line when there is one thing to say, which is nearly always: the
//! verdict, the tallies and the finding share it. Further findings take a line
//! each beneath the first, up to three in all before the strip scrolls, and
//! the stage sizes the band to [`content_height`] so a clean definition does
//! not pay for the room a refused one needs.

use egui::{Rect, Sense, Ui, UiBuilder, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{StatusMark, paint_status_mark};
use super::{Finding, Stage};

/// The strip's first line: the verdict, the tallies and the first finding.
const HEAD_HEIGHT: f32 = 28.0;
/// Each finding after the first.
const LINE_HEIGHT: f32 = 22.0;
/// How many findings after the first the strip shows before it scrolls. The
/// strip is a verdict, not a log: three lines answer "what is wrong", and a
/// fourth is one scroll away rather than forty points off the plot.
const VISIBLE_EXTRA_LINES: usize = 2;
const INSET: f32 = 10.0;

/// How tall the strip is for this definition's findings.
pub(super) fn content_height(stage: &Stage) -> f32 {
    HEAD_HEIGHT
        + stage
            .findings
            .len()
            .saturating_sub(1)
            .min(VISIBLE_EXTRA_LINES) as f32
            * LINE_HEIGHT
}

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let errors = stage.errors();
    let advisories = stage.advisories();
    let (verdict, color) = if errors > 0 {
        (
            messages.format(
                if errors == 1 {
                    MessageId::StimulusAuditErrorSingular
                } else {
                    MessageId::StimulusAuditErrors
                },
                &[("count", &errors.to_string())],
            ),
            palette.err,
        )
    } else if advisories > 0 {
        (
            messages.format(
                if advisories == 1 {
                    MessageId::StimulusAuditAdvisorySingular
                } else {
                    MessageId::StimulusAuditAdvisories
                },
                &[("count", &advisories.to_string())],
            ),
            palette.warn,
        )
    } else {
        (messages.text(MessageId::StimulusAuditValid), palette.ok)
    };
    let satisfied = messages.format(
        MessageId::StimulusAuditSatisfied,
        &[("family", stage.working.family().label())],
    );

    ui.spacing_mut().item_spacing.y = 0.0;
    let width = ui.available_width();
    let (head, _) = ui.allocate_exact_size(Vec2::new(width, HEAD_HEIGHT), Sense::hover());
    let mut row = ui.new_child(
        UiBuilder::new()
            .id_salt("workbench.stimulus.audit.head")
            .max_rect(head)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    row.set_clip_rect(ui.clip_rect().intersect(head));
    row.add_space(INSET);
    row.add(egui::Label::new(
        egui::RichText::new(&verdict)
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(color),
    ));
    row.add_space(4.0);
    row.add(egui::Label::new(
        egui::RichText::new(messages.format(
            MessageId::StimulusAuditCounts,
            &[
                ("errors", &errors.to_string()),
                ("advisories", &advisories.to_string()),
            ],
        ))
        .font(theme::mono(tokens::FS_MICRO, FontWeight::Regular))
        .color(palette.text_faint),
    ))
    .on_hover_text(messages.text(MessageId::StimulusAuditCountsHint));
    row.add_space(6.0);
    // The rule between the verdict and what it is a verdict on. Every finding
    // below starts at the same x, so the strip reads as one list with a head.
    let rule = row.cursor().left();
    ui.painter().vline(
        rule,
        head.y_range().shrink(7.0),
        egui::Stroke::new(1.0, palette.border),
    );
    row.add_space(INSET);
    let findings_left = row.cursor().left();
    match stage.findings.first() {
        Some(first) => finding_line(&mut row, first),
        None => finding_line_with(&mut row, StatusMark::Success, palette.ok, &satisfied),
    }

    let rest = stage.findings.get(1..).unwrap_or_default();
    if rest.is_empty() {
        return;
    }
    egui::ScrollArea::vertical()
        .id_salt("workbench.stimulus.audit")
        .auto_shrink([false, false])
        .max_height(rest.len().min(VISIBLE_EXTRA_LINES) as f32 * LINE_HEIGHT)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            for (index, finding) in rest.iter().enumerate() {
                let (line, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), LINE_HEIGHT),
                    Sense::hover(),
                );
                let line = Rect::from_min_max(egui::pos2(findings_left, line.top()), line.max);
                let mut row = ui.new_child(
                    UiBuilder::new()
                        .id_salt(("workbench.stimulus.audit.line", index))
                        .max_rect(line)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );
                row.set_clip_rect(ui.clip_rect().intersect(line));
                finding_line(&mut row, finding);
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
    ui.add_space(2.0);
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
