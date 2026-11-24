//! The band a page states one condition of its document in, above the
//! document, and the band of a run held for more memory than is free, which
//! the deck's page and the plan's page ask in alike.

use egui::Ui;

use crate::tokens::Tokens;

use super::button::Button;
use super::status_mark::{PROBLEM_MARK_SIDE, Problem, paint_problem_mark};

/// The gap between a band's routes, and before the first of them.
const BAND_GAP: f32 = 8.0;
/// The gap between a band's mark and its words.
const BAND_MARK_GAP: f32 = 6.0;

/// The banner recipe every advisory band above a document shares: one inset
/// row that states a condition of the document, with a problem's mark as the
/// only colour on it. It is one row of the header's height: its buttons are
/// the kit's compact ones, with the same gap above and below them as in the
/// header, so the band and the header read as one family.
pub fn advisory_band(ui: &mut Ui, problem: Option<Problem>, contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::symmetric(8, 3))
        .show(ui, |ui| {
            // A frame shrinks to its content: the band runs the page's whole
            // width however short its words are.
            ui.set_min_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                if let Some(problem) = problem {
                    let (mark, _) = ui.allocate_exact_size(
                        egui::vec2(PROBLEM_MARK_SIDE, PROBLEM_MARK_SIDE),
                        egui::Sense::hover(),
                    );
                    paint_problem_mark(ui.painter(), mark, problem);
                }
                contents(ui);
            });
        });
}

/// What the reader answered a run held for more memory than is free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeldRunAnswer {
    /// Run it though it needs more memory than is free.
    RunAnyway,
    /// Drop it: nothing of it runs.
    Stop,
}

/// What a held run's band says, in the words of the page that asks.
pub struct HeldRunBand<'a> {
    /// What the run needs against what is free and installed: the band's
    /// words.
    pub numbers: &'a str,
    /// The same with what running it anyway does: the words' hover.
    pub message: &'a str,
    pub run_anyway: &'a str,
    pub run_anyway_hint: &'a str,
    pub stop: &'a str,
    pub stop_hint: &'a str,
}

/// The band of a run held because it needs more memory than the machine has
/// free: the warning's mark, what the run needs against what is free and
/// installed, and the answers at its far end, Run anyway before Stop.
pub fn held_run_band(ui: &mut Ui, band: &HeldRunBand<'_>) -> Option<HeldRunAnswer> {
    let mut answer = None;
    advisory_band(ui, Some(Problem::Warning), |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = BAND_GAP;
            let stop = Button::new(band.stop).compact().show(ui);
            if stop.on_hover_text(band.stop_hint).clicked() {
                answer = Some(HeldRunAnswer::Stop);
            }
            let run = Button::new(band.run_anyway).compact().show(ui);
            if run.on_hover_text(band.run_anyway_hint).clicked() {
                answer = Some(HeldRunAnswer::RunAnyway);
            }
            ui.add_space(BAND_GAP);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                // The mark keeps the gap from the words a run band's dot does.
                ui.add_space(BAND_MARK_GAP);
                ui.add(egui::Label::new(band.numbers).truncate())
                    .on_hover_text(band.message);
            });
        });
    });
    answer
}
