//! What a drifted model source looks like to the reader: the mark in the
//! source column, the banner over a model's detail and the resolve dialog.
//!
//! Detecting the drift is the model's ([`Model::scan_source_drift`]), so the
//! Simulation Studio's Models page reads the same report this page repairs.
//!
//! [`Model::scan_source_drift`]: crate::workbench::app_state::Model::scan_source_drift

use super::*;

/// What a drifted source means for the project, where the reader meets it.
///
/// Split from what the repair does, because the two are read in different
/// places: the banner is where a reader decides whether to care, and the
/// dialog is where they decide whether to commit. One sentence printed in
/// both was the same paragraph twice on one screen, the modal sitting over
/// the banner it came from.
pub(super) const DRIFT_EFFECT: &str =
    "A run checks every source against its pin, so this library is refused until it is re-pinned.";

/// And what the repair does, above the control that performs it.
pub(super) const REPIN_EFFECT: &str = "Re-pinning accepts the bytes as they are now and records the change in a new project revision.";

/// The word this mark writes in the source column.
const SOURCE_CHIP_WORD: &str = "drift";

/// How much of the source column the mark claims: the word, the mark and its
/// advance, the air between the mark and whatever is elided against it, and
/// the gap the word keeps from the column's edge.
///
/// The cell that carries it has to be elided against this, because the mark
/// is drawn from the column's own right edge and the value from its left — so
/// a path long enough to fill the column ran straight under the mark. Fixture
/// libraries are called `models.lib` and never showed it; a foundry's are
/// called `tsmc18_rf_lvt_corners_2024.lib`.
pub(super) fn source_chip_width(ui: &Ui) -> f32 {
    let color = Tokens::get(ui.ctx()).color.text_dim;
    let text = ui
        .painter()
        .layout_no_wrap(
            SOURCE_CHIP_WORD.to_owned(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            color,
        )
        .size()
        .x;
    text + MARK_ADVANCE + MARK_GAP + 5.0
}

/// Paints the catalog row's drift mark inside the source column.
///
/// Exception-only and right-aligned against the column it qualifies, because
/// it is a fact *about* the source rather than another source column: a row
/// whose bytes still match says nothing at all.
pub(super) fn paint_source_chip(ui: &Ui, rect: egui::Rect, column_end: f32) {
    let t = Tokens::get(ui.ctx());
    let right = rect.left() + rect.width() * column_end - 5.0;
    let word = ui.painter().text(
        egui::pos2(right, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        SOURCE_CHIP_WORD,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    paint_problem_mark(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(
                word.left() - MARK_ADVANCE + MARK_SIDE * 0.5,
                rect.center().y,
            ),
            egui::Vec2::splat(MARK_SIDE),
        ),
        Problem::Warning,
    );
}

/// The selected model's drift banner: both digests, when, and what it means.
///
/// One banner rather than a per-source list, because the pane it sits in is
/// about one model and the list belongs to the library. Opening it is what the
/// resolve flow is for.
pub(super) fn detail_banner(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, library: &str) {
    let findings = app.model.source_drift_findings(library);
    let Some(first) = findings.first() else {
        return;
    };
    // Grammatical in the case it is nearly always read in — one source — and
    // true of a source that could not be read at all, which does not hash to
    // anything and so cannot be described as hashing to the wrong thing.
    let headline = if findings.len() == 1 {
        "1 pinned source no longer matches what this project accepted".to_owned()
    } else {
        format!(
            "{} pinned sources no longer match what this project accepted",
            findings.len()
        )
    };
    // The banner names one finding, so when it is not the only one it says
    // so: three findings under a line naming one file read as one finding
    // with a count nobody could place.
    let remainder = match findings.len() {
        1 => String::new(),
        count => format!(" · and {} more", count - 1),
    };
    let digests = format!(
        "{} · pinned {} → now {}{remainder}",
        path_label(&first.path),
        first.pinned,
        first.on_disk.as_deref().unwrap_or("unreadable")
    );
    let detected = app.model.source_drift.scanned_at.clone().map_or_else(
        || "detected by the last source scan".to_owned(),
        |at| format!("detected {at}"),
    );
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.spacing_mut().item_spacing.y = 3.0;
            hub::marked_line(ui, Problem::Warning, &headline);
            hub::announced(
                ui,
                RichText::new(&digests)
                    .small()
                    .monospace()
                    .color(t.color.text),
                &digests,
            );
            hub::announced(
                ui,
                RichText::new(&detected).small().color(t.color.text_faint),
                &detected,
            );
            hub::announced(
                ui,
                RichText::new(DRIFT_EFFECT).small().color(t.color.text_dim),
                DRIFT_EFFECT,
            );
            if Button::new("Resolve drift…").show(ui).clicked() {
                app.state.models_view.dialog = Some(ModelsWorkbenchDialog::ResolveDrift {
                    library: library.to_owned(),
                });
            }
        });
}

/// The resolve dialog: every finding this library has, and the one repair.
///
/// It opens on the findings a scan actually produced rather than re-running
/// the check, so what a reader decided against is what they were shown. The
/// repair is a re-pin — the same refresh the source pane offers — which reads
/// the bytes as they are now and publishes a revision recording the change.
pub(super) fn resolve_dialog(ui: &mut Ui, app: &ManagerRenderContext<'_>, library: &str) {
    let findings = app.model.source_drift_findings(library).to_vec();
    let t = Tokens::get(ui.ctx());
    // The dialog's own title names the library and the banner behind it
    // stated the count; what this surface owes a reader is the list below
    // and what the button under it will do.
    ui.label(RichText::new(REPIN_EFFECT).small().color(t.color.text_dim));
    card(ui, |ui| {
        card_title(ui, "Findings", Some("pinned → present"));
        if findings.is_empty() {
            empty_state(
                ui,
                "This library's sources all match their pins.",
                "The last scan found nothing to resolve here.",
            );
            return;
        }
        ScrollArea::vertical()
            .id_salt("models-drift-findings")
            .auto_shrink([false, true])
            .max_height(260.0)
            .show(ui, |ui| {
                for finding in &findings {
                    property(
                        ui,
                        &path_label(&finding.path),
                        &format!(
                            "{} → {}",
                            finding.pinned,
                            finding.on_disk.as_deref().unwrap_or("unreadable")
                        ),
                        // What moved, when both copies are on hand to say.
                        // The two sentences that stood here otherwise said
                        // what the value beside them already says — and the
                        // longer of them was clipped by the column it sat
                        // in, which is how a render finds a disclaimer.
                        finding.change.as_deref().unwrap_or_default(),
                    );
                }
            });
    });
}

/// Why the re-pin cannot run right now, if it cannot.
///
/// The dialog's footer owns the commit, so the reason a commit is blocked has
/// to travel there with it.
pub(super) fn repin_block_reason(
    app: &ManagerRenderContext<'_>,
    library: &str,
) -> Option<&'static str> {
    if app.model.source_drift_findings(library).is_empty() {
        return Some("There is nothing here to re-pin.");
    }
    app.state
        .models_view
        .model_import_in_progress
        .then_some("Another model-source operation is still running.")
}

#[cfg(test)]
mod tests;
