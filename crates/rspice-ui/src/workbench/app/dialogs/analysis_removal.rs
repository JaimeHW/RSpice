//! Confirming the removal of a configured analysis.
//!
//! Removal is only staged when it costs something. An analysis that nothing
//! depends on and that has produced nothing is removed immediately — asking
//! about that would be friction, and friction is how a reader learns to
//! dismiss a dialog without reading it. What is worth stopping for is the
//! removal that silently orphans retained results, or that unbinds an
//! analysis another one needs to run at all.
//!
//! This module only renders and records the answer. The removal itself stays
//! with the Simulate surface that owns the plan, which reads the confirmed
//! answer on its next frame.

use egui::Context;

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, kv_row};
use crate::workbench::RSpiceApp;

impl RSpiceApp {
    pub(in crate::workbench) fn process_analysis_removal_review_dialog(&mut self, ctx: &Context) {
        let review = &self.state.dialogs.analysis_removal_review;
        if review.target.is_none() {
            return;
        }

        let label = review.label.clone();
        let retained_runs = review.retained_runs;
        let retained = retained_runs.to_string();
        let dependent_analyses = review.dependent_analyses.clone();
        let dependents = if dependent_analyses.is_empty() {
            "none".to_owned()
        } else {
            dependent_analyses.join(", ")
        };

        let choice = Dialog::new("Simulate", "Remove analysis", "Remove")
            .description(
                "Remove this analysis from the plan. Retained datasets are not deleted, but they \
                 stop being attributable to a configured analysis.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .destructive()
            .hint("Plan mutation · not undoable from schematic history")
            .show(ctx, |ui| {
                kv_row(ui, "Analysis", &label);
                kv_row(ui, "Retained runs holding its results", &retained);
                kv_row(ui, "Analyses bound to it", &dependents);

                ui.add_space(8.0);
                let t = Tokens::get(ui.ctx());
                if !dependent_analyses.is_empty() {
                    ui.label(
                        egui::RichText::new(
                            "The analyses listed above depend on this one. Removing it leaves \
                             them unbound, and they will refuse to run until they are rebound.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.warn),
                    );
                    ui.add_space(6.0);
                }
                if retained_runs > 0 {
                    ui.label(
                        egui::RichText::new(
                            "Retained results stay in the project and stay readable. What they \
                             lose is the configuration they can be traced back to.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => {
                self.state.dialogs.analysis_removal_review.confirmed = true;
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.analysis_removal_review.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}
