//! Clear result history…: what it deletes and what it keeps, asked before
//! anything goes.
//!
//! The command used to delete every retained run on one click, with nothing
//! to undo it. It now counts the runs first. Pinned golden baselines and the
//! runs a saved result document reads stay unless the reader turns on
//! deleting them as well, and the dialog says plainly that what it deletes
//! cannot be restored.

use std::collections::HashSet;

use egui::{Context, RichText, Ui};

use crate::product::DatasetId;
use crate::state::SimulationRun;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, kv_row, switch_row};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::results::ResultsFeature;

const TITLE: &str = "Clear result history";

/// The open question, and the reader's answer to whether the protected runs
/// go too.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ClearHistoryDialog {
    /// Delete pinned golden baselines and the runs saved result documents
    /// read, as well as the rest of the history.
    pub delete_protected: bool,
}

/// The retained runs, by what clearing does with each. A run that is both
/// pinned and read by a document counts once, as pinned.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ClearHistoryCounts {
    /// The rest of the history, which clearing always deletes.
    pub ordinary: usize,
    /// Pinned golden baselines.
    pub pinned: usize,
    /// Runs a saved result document reads.
    pub read_by_documents: usize,
}

impl ClearHistoryCounts {
    /// Count the model's retained runs.
    #[must_use]
    pub fn of(model: &Model) -> Self {
        let read_by_documents = model.document_read_datasets();
        let mut counts = Self::default();
        for run in &model.simulation.retained.runs {
            if run.retention().is_pinned() {
                counts.pinned += 1;
            } else if read_by_documents.contains(&run.dataset_id) {
                counts.read_by_documents += 1;
            } else {
                counts.ordinary += 1;
            }
        }
        counts
    }

    /// How many runs clearing deletes.
    #[must_use]
    pub const fn deleted(self, delete_protected: bool) -> usize {
        if delete_protected {
            self.ordinary + self.protected()
        } else {
            self.ordinary
        }
    }

    const fn protected(self) -> usize {
        self.pinned + self.read_by_documents
    }
}

/// Whether clearing keeps this run unless the reader asks otherwise.
pub(crate) fn is_protected(run: &SimulationRun, read_by_documents: &HashSet<DatasetId>) -> bool {
    run.retention().is_pinned() || read_by_documents.contains(&run.dataset_id)
}

/// Show the question while it is open, and clear as answered.
pub(super) fn show(cx: &mut Cx<'_, ResultsFeature>, ctx: &Context) {
    let Some(mut dialog) = cx.state.clear_history else {
        return;
    };
    let counts = ClearHistoryCounts::of(cx.model);
    let executing = cx.model.simulation.execution.has_active_execution();
    let deleted = counts.deleted(dialog.delete_protected);
    let choice = Dialog::new(TITLE, delete_label(deleted))
        .description(
            "Delete the retained runs of this project's result history. Pinned golden baselines \
             and runs a saved result document reads are kept unless you delete them too.",
        )
        .size(DialogSize::Confirmation)
        .ghost("Cancel")
        .destructive()
        .primary_enabled(deleted > 0 && !executing)
        .show(ctx, |ui| body(ui, counts, &mut dialog));
    match choice {
        DialogChoice::Primary => {
            cx.state.clear_history = None;
            let deleted =
                super::results_parts(cx).clear_simulation_results(dialog.delete_protected);
            let kept = cx.model.simulation.retained.runs.len();
            cx.wb
                .prompt(format!("Deleted {deleted} {}; kept {kept}.", runs(deleted)));
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => cx.state.clear_history = None,
        DialogChoice::Secondary | DialogChoice::None => cx.state.clear_history = Some(dialog),
    }
}

fn body(ui: &mut Ui, counts: ClearHistoryCounts, dialog: &mut ClearHistoryDialog) {
    let delete_protected = dialog.delete_protected;
    let fate = |count: usize| {
        if delete_protected {
            format!("{count} deleted")
        } else {
            format!("{count} kept")
        }
    };
    kv_row(ui, "Other runs", &format!("{} deleted", counts.ordinary));
    kv_row(ui, "Pinned golden baselines", &fate(counts.pinned));
    kv_row(
        ui,
        "Read by saved result documents",
        &fate(counts.read_by_documents),
    );
    if counts.protected() > 0 {
        ui.add_space(6.0);
        switch_row(
            ui,
            "Also delete pinned runs and runs saved documents read",
            &mut dialog.delete_protected,
        );
    }
    ui.add_space(8.0);
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new("Deleted runs cannot be restored.")
            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
            .color(t.color.warn),
    );
}

fn delete_label(deleted: usize) -> String {
    if deleted == 0 {
        "Delete runs".to_owned()
    } else {
        format!("Delete {deleted} {}", runs(deleted))
    }
}

const fn runs(count: usize) -> &'static str {
    if count == 1 { "run" } else { "runs" }
}
