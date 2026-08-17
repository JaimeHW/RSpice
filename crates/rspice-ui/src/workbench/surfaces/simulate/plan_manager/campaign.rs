//! Queueing several plans as one campaign.
//!
//! One of the five routes the plan-manager shell dispatches to. Each member is
//! frozen as it stands now and dispatched in the table's declared order with its
//! own run, job, dataset and manifest identity — so a campaign is several
//! authenticated runs, not one run over several plans.
//!
//! Every task count here is the run set's own, read through
//! [`PlanCatalogRecord::task_count`]. Multiplying a point count by an
//! enabled-analysis count is different arithmetic, because the run set expands
//! analysis families and their prerequisites, and doing it locally stood a
//! second and wrong answer beside the authoritative one.

use super::*;

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · CAMPAIGN · ONE AUTHENTICATED RUN PER PLAN",
        "Queue simulation campaign",
        "Queue reviewed campaign",
    )
    .description(
        "Each selected plan is frozen now, then dispatched in declared table order with its own run, job, dataset, and manifest identity.",
    )
    .size(DialogSize::WideWorkflow)
    .ghost("Cancel")
    .primary_enabled(draft.campaign.member_ids.len() >= 2)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Campaign name");
            mono_input(ui, &mut draft.campaign.name, ui.available_width().min(360.0));
        });
        let mut combined_tasks = 0_usize;
        egui::Grid::new("simulation.plan-manager.campaign")
            .num_columns(5)
            .striped(true)
            .show(ui, |ui| {
                for heading in ["Member", "Plan", "Analyses", "Points", "Tasks"] {
                    ui.label(egui::RichText::new(heading).strong());
                }
                ui.end_row();
                for record in records.iter().filter(|record| !record.archived) {
                    let mut included = draft.campaign.member_ids.contains(&record.id);
                    if ui.checkbox(&mut included, "").changed() {
                        if included {
                            if !draft.campaign.member_ids.contains(&record.id) {
                                draft.campaign.member_ids.push(record.id);
                            }
                        } else {
                            draft.campaign.member_ids.retain(|id| *id != record.id);
                        }
                    }
                    ui.label(&record.name);
                    ui.label(record.enabled.to_string());
                    ui.label(
                        record
                            .point_count()
                            .map_or_else(|| "invalid".to_owned(), |count| count.to_string()),
                    );
                    let tasks = record.task_count().unwrap_or(0);
                    if included {
                        combined_tasks = combined_tasks.saturating_add(tasks);
                    }
                    ui.label(tasks.to_string());
                    ui.end_row();
                }
            });
        property_row(
            ui,
            "Combined declared scope",
            &format!(
                "{} plans · approximately {} tasks before dependency expansion",
                draft.campaign.member_ids.len(),
                combined_tasks
            ),
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ApplyCampaign),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

pub(super) fn commit_simulation_campaign(
    app: &mut RSpiceApp,
    name: &str,
    member_ids: &[SimulationPlanId],
) -> Result<String, String> {
    if app.state.simulation.has_active_execution() || app.state.simulation.trigger_simulation {
        return Err("A simulation is already running or waiting to start".to_owned());
    }
    app.state.sync_active_schematic_to_workspace();
    crate::workbench::menu_bar::run_design_rule_check(&mut app.state);
    let receipt =
        app.simulation_controller
            .prepare_and_start_campaign(&mut app.state, name, member_ids)?;
    Ok(format!(
        "Campaign {} queued {} plans as {} authenticated tasks.",
        receipt.campaign_id, receipt.member_count, receipt.task_count
    ))
}
