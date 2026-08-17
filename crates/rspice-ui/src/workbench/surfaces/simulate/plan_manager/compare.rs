//! Comparing the selected plan against the active one.
//!
//! One of the five routes the plan-manager shell dispatches to, and the only one
//! with nothing to commit: it states a difference and closes. Nothing in this
//! file is handed the application, because a comparison has no reason to be able
//! to change anything.
//!
//! Every quantity is read from [`PlanCatalogRecord`], so a count shown here is
//! the same count the records table shows rather than a second derivation free
//! to disagree with it.

use super::*;

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let selected = records
        .iter()
        .find(|record| record.id == draft.selected_plan_id);
    let active = records.iter().find(|record| record.active);
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · PLAN COMPARISON · ACTIVE VERSUS SELECTED",
        "Compare simulation plans",
        "Close comparison",
    )
    .description(
        "The active plan on the left of each row, the selected plan on the right. Comparing changes nothing in either.",
    )
    .size(DialogSize::SimulationWorkflow)
    .show(ctx, |ui| {
        if let (Some(active), Some(selected)) = (active, selected) {
            property_row(
                ui,
                "Comparison",
                &format!("{} ↔ {}", active.name, selected.name),
            );
            property_row(
                ui,
                "Analyses",
                &format!("{} ↔ {}", active.analyses, selected.analyses),
            );
            property_row(
                ui,
                "PVT points",
                &format!(
                    "{} ↔ {}",
                    active.point_count().unwrap_or(0),
                    selected.point_count().unwrap_or(0)
                ),
            );
            property_row(
                ui,
                "Model bindings",
                &format!("{} ↔ {}", active.model_bindings, selected.model_bindings),
            );
        }
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}
