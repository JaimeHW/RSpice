//! Comparing one plan in the catalog against another.
//!
//! Which two is the route's own selection, resolved by [`compared_plans`]; until
//! a side is picked it is the active plan against the selected row, which is the
//! only pair this surface could name when neither side was choosable.
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
    let (base, target) = compared_plans(draft, records);
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
        if let (Some(base), Some(target)) = (base, target) {
            property_row(ui, "Comparison", &format!("{} ↔ {}", base.name, target.name));
            property_row(
                ui,
                "Analyses",
                &format!("{} ↔ {}", base.analyses, target.analyses),
            );
            property_row(
                ui,
                "PVT points",
                &format!(
                    "{} ↔ {}",
                    base.point_count().unwrap_or(0),
                    target.point_count().unwrap_or(0)
                ),
            );
            property_row(
                ui,
                "Model bindings",
                &format!("{} ↔ {}", base.model_bindings, target.model_bindings),
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

/// The two plans this comparison diffs, left side first.
///
/// Each side comes from the route's own selection. An unpicked side falls back to
/// the plan this surface would have compared anyway — the active plan on the
/// base, the selected row on the target — so the pair a freshly opened manager
/// states is the one pair this route could state before either side was
/// choosable, and picking a side narrows it rather than emptying the surface.
///
/// A side naming a plan the projection no longer carries falls back the same
/// way, because the catalog can lose a plan between the frame that picked it and
/// this one. Resolving through `records` is what makes that a stale selection
/// rather than a blank comparison.
pub(super) fn compared_plans<'records>(
    draft: &SimulationPlanManagerDraft,
    records: &'records [PlanCatalogRecord],
) -> (
    Option<&'records PlanCatalogRecord>,
    Option<&'records PlanCatalogRecord>,
) {
    let by_id = |id: SimulationPlanId| records.iter().find(move |record| record.id == id);
    let base = draft
        .comparison
        .base_plan_id
        .and_then(by_id)
        .or_else(|| records.iter().find(|record| record.active));
    let target = draft
        .comparison
        .target_plan_id
        .and_then(by_id)
        .or_else(|| by_id(draft.selected_plan_id));
    (base, target)
}
