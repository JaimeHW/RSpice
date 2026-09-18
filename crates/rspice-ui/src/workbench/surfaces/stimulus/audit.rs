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

use egui::Ui;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{StatusMark, paint_status_mark};
use super::Stage;

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage) {
    let messages = state.ui.messages();
    let palette = Tokens::get(ui.ctx()).color;
    let errors = stage.errors();
    let advisories = stage.advisories();
    let (verdict, color) = if errors > 0 {
        (
            messages.format(
                MessageId::StimulusAuditErrors,
                &[("count", &errors.to_string())],
            ),
            palette.err,
        )
    } else if advisories > 0 {
        (
            messages.format(
                MessageId::StimulusAuditAdvisories,
                &[("count", &advisories.to_string())],
            ),
            palette.warn,
        )
    } else {
        (messages.text(MessageId::StimulusAuditValid), palette.ok)
    };

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.add(egui::Label::new(
            egui::RichText::new(&verdict)
                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                .color(color),
        ));
        ui.add_space(8.0);
        ui.add(egui::Label::new(
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
    });

    egui::ScrollArea::vertical()
        .id_salt("workbench.stimulus.audit")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if stage.findings.is_empty() {
                finding_line(
                    ui,
                    StatusMark::Success,
                    palette.ok,
                    &messages.format(
                        MessageId::StimulusAuditSatisfied,
                        &[("family", stage.working.family().label())],
                    ),
                );
                return;
            }
            for finding in &stage.findings {
                let (mark, color) = if finding.blocking {
                    (StatusMark::Failure, palette.err)
                } else {
                    (StatusMark::Warning, palette.warn)
                };
                finding_line(ui, mark, color, &finding.message);
            }
        });
}

/// One finding: its mark, painted as geometry, and what it says.
fn finding_line(ui: &mut Ui, mark: StatusMark, color: egui::Color32, message: &str) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(12.0), egui::Sense::hover());
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
    });
}
