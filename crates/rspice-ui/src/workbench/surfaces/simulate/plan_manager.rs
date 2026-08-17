//! The versioned simulation-plan catalog: create, activate, rename, clone,
//! compare, archive, restore, exchange, and queue a campaign over plans.
//!
//! A plan is the unit of ownership here. Activating one is atomic — the setup
//! and the workspace payload move together or not at all — and archiving one
//! never rewrites the immutable results that reference it, which is why every
//! operation below reports what it did to the catalog rather than to the run.
//!
//! Every surface in this module reads one projection of the catalog, built by
//! [`records`]. A count shown in the table, in the comparison, and in the
//! campaign is therefore the same count from the same owner rather than three
//! re-derivations free to disagree.

mod campaign;
mod compare;
mod create;
mod exchange;
mod lifecycle;
mod records;

use super::*;
use records::{PlanCatalogRecord, plan_catalog_records};

use crate::workbench::app_state::ReferencePvtPoint;
use crate::workbench::design_system::property_row_status;

const SIMULATION_PLAN_PACKAGE_VERSION: u16 = 1;
const SIMULATION_PLAN_PACKAGE_FORMAT: &str = "rspice.simulation-plan";

/// What the selected-plan aside says in place of a workload when the plan's
/// run-space declaration does not validate.
///
/// The forecast is absent in that case, and every quantity derived from it with
/// it. Printing zero points, zero tasks and a zero cost would read as "this
/// plan declares no work", which is a different claim and a false one — the
/// plan declares work that cannot be expanded.
const RUN_SET_DOES_NOT_VALIDATE: &str = "does not validate";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PortableSimulationPlanPackage {
    format: String,
    version: u16,
    plan: crate::workbench::app_state::SimulationPlanImportDocument,
    payload: crate::state::SimulationPlanPayload,
}

enum PlanManagerAction {
    Create,
    Campaign,
    Rename,
    Clone,
    Compare,
    Export,
    Import,
    CopyExport,
    ApplyImport,
    ApplyCampaign,
    Archive,
    Restore,
    ApplyCreate,
    ApplyRename,
    ConfirmArchive,
    CancelInline,
}

pub(super) fn plan_manager_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: SimulationPlanManagerDraft,
) {
    let records = plan_catalog_records(app);
    if !records
        .iter()
        .any(|record| record.id == draft.selected_plan_id)
        && let Some(active) = records.iter().find(|record| record.active)
    {
        draft.selected_plan_id = active.id;
        draft.name = active.name.clone();
    }
    let query = draft.filter.trim().to_ascii_lowercase();
    let visible = records
        .iter()
        .filter(|record| match draft.scope {
            1 => !record.archived,
            2 => record.archived,
            _ => true,
        })
        .filter(|record| {
            query.is_empty()
                || record.name.to_ascii_lowercase().contains(&query)
                || record.id.to_string().to_ascii_lowercase().contains(&query)
        })
        .cloned()
        .collect::<Vec<_>>();
    let selected = records
        .iter()
        .find(|record| record.id == draft.selected_plan_id)
        .cloned();
    let can_open = selected.as_ref().is_some_and(|record| !record.archived);
    let mut action = None;

    let choice = Dialog::new(
        "SIMULATION · VERSIONED PLAN LIFECYCLE · SINGLE ACTIVE OWNER",
        "Simulation plans",
        "Open selected plan",
    )
    .description(
        "Create, select, compare, rename, clone, restore, and archive complete simulation plans without rewriting immutable results.",
    )
    .size(DialogSize::WideWorkflow)
    .flush_body()
    .ghost("Close")
    .primary_enabled(can_open)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Filter");
            mono_input(ui, &mut draft.filter, ui.available_width().min(360.0));
            let choices = vec![
                "All plans".to_owned(),
                "Working".to_owned(),
                "Archived".to_owned(),
            ];
            let current = choices
                .get(draft.scope)
                .map(String::as_str)
                .unwrap_or(choices[0].as_str());
            if let Some(scope) = select(
                ui,
                "simulation.plan-manager.scope",
                "Plan scope",
                current,
                &choices,
                150.0,
            ) {
                draft.scope = scope;
            }
            if Button::new("New plan…").accent().show(ui).clicked() {
                action = Some(PlanManagerAction::Create);
            }
            if Button::new("Queue campaign…")
                .enabled(records.iter().filter(|record| !record.archived).count() >= 2)
                .show(ui)
                .clicked()
            {
                action = Some(PlanManagerAction::Campaign);
            }
        });
        ui.add_space(8.0);
        let t = Tokens::get(ui.ctx());
        egui::Frame::new()
            .fill(t.color.bg_panel)
            .stroke(Stroke::new(1.0, t.color.border))
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                egui::Grid::new("simulation.plan-manager.rows")
                    .num_columns(7)
                    .striped(true)
                    .min_col_width(74.0)
                    .show(ui, |ui| {
                        for heading in [
                            "Plan / identity",
                            "Lifecycle",
                            "Revision",
                            "Analyses",
                            "Run set",
                            "Models",
                            "Results",
                        ] {
                            ui.label(egui::RichText::new(heading).strong());
                        }
                        ui.end_row();
                        for record in &visible {
                            let label = format!("{}\n{}", record.name, record.id);
                            if ui
                                .selectable_label(record.id == draft.selected_plan_id, label)
                                .clicked()
                            {
                                draft.selected_plan_id = record.id;
                                draft.name = record.name.clone();
                                draft.mode = SimulationPlanManagerMode::Browse;
                                draft.validation_error = None;
                            }
                            ui.label(record.lifecycle_label());
                            ui.label(record.revision.to_string());
                            ui.label(format!("{} / {}", record.enabled, record.analyses));
                            ui.label(record.point_count().map_or_else(
                                || "invalid".to_owned(),
                                |count| format!("{count} PVT"),
                            ));
                            ui.label(record.model_bindings.to_string());
                            ui.label(record.results.to_string());
                            ui.end_row();
                        }
                    });
            });
        ui.add_space(10.0);
        if let Some(selected) = selected.as_ref() {
            ui.horizontal_wrapped(|ui| {
                if Button::new("Rename…").show(ui).clicked() {
                    action = Some(PlanManagerAction::Rename);
                }
                if Button::new("Clone…")
                    .enabled(!selected.archived)
                    .show(ui)
                    .clicked()
                {
                    action = Some(PlanManagerAction::Clone);
                }
                if Button::new("Compare…").show(ui).clicked() {
                    action = Some(PlanManagerAction::Compare);
                }
                if Button::new("Export…").show(ui).clicked() {
                    action = Some(PlanManagerAction::Export);
                }
                if Button::new("Import…").show(ui).clicked() {
                    action = Some(PlanManagerAction::Import);
                }
                if selected.archived {
                    if Button::new("Restore").show(ui).clicked() {
                        action = Some(PlanManagerAction::Restore);
                    }
                } else if Button::new("Archive…")
                    .enabled(!selected.active)
                    .show(ui)
                    .clicked()
                {
                    action = Some(PlanManagerAction::Archive);
                }
            });
            plan_manager_inline_editor(ui, &mut draft, selected, &records, &mut action);
        }
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });

    match choice {
        DialogChoice::Primary => match commit_activate_plan(app, draft.selected_plan_id) {
            Ok(message) => {
                app.state.workbench.simulation_workflow = None;
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Simulation plan opened", message);
            }
            Err(error) => {
                draft.validation_error = Some(error);
                app.state.workbench.simulation_workflow =
                    Some(SimulationWorkflowDialog::PlanManager(draft));
            }
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.workbench.simulation_workflow = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {
            handle_plan_manager_action(ctx, app, draft, action);
        }
    }
}

fn plan_manager_inline_editor(
    ui: &mut Ui,
    draft: &mut SimulationPlanManagerDraft,
    selected: &PlanCatalogRecord,
    records: &[PlanCatalogRecord],
    action: &mut Option<PlanManagerAction>,
) {
    match draft.mode {
        SimulationPlanManagerMode::Browse => {
            ui.add_space(8.0);
            selected_plan_properties(ui, selected);
        }
        SimulationPlanManagerMode::Create | SimulationPlanManagerMode::Rename => {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(if draft.mode == SimulationPlanManagerMode::Create {
                    "New plan name"
                } else {
                    "Plan name"
                });
                mono_input(ui, &mut draft.name, ui.available_width().min(360.0));
                if Button::new("Apply").accent().show(ui).clicked() {
                    *action = Some(if draft.mode == SimulationPlanManagerMode::Create {
                        PlanManagerAction::ApplyCreate
                    } else {
                        PlanManagerAction::ApplyRename
                    });
                }
                if Button::new("Cancel").show(ui).clicked() {
                    *action = Some(PlanManagerAction::CancelInline);
                }
            });
        }
        SimulationPlanManagerMode::Compare => {
            ui.add_space(10.0);
            let active = records
                .iter()
                .find(|record| record.active)
                .unwrap_or(selected);
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
        SimulationPlanManagerMode::Export => {
            ui.add_space(10.0);
            ui.label("Portable RSpice simulation-plan JSON");
            ui.add(
                egui::TextEdit::multiline(&mut draft.exchange_text)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .desired_rows(12)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );
            ui.horizontal(|ui| {
                if Button::new("Copy JSON").accent().show(ui).clicked() {
                    *action = Some(PlanManagerAction::CopyExport);
                }
                if Button::new("Close export").show(ui).clicked() {
                    *action = Some(PlanManagerAction::CancelInline);
                }
            });
        }
        SimulationPlanManagerMode::Import => {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Imported plan name");
                mono_input(ui, &mut draft.name, ui.available_width().min(360.0));
            });
            ui.label("Paste portable RSpice simulation-plan JSON");
            ui.add(
                egui::TextEdit::multiline(&mut draft.exchange_text)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .desired_rows(12)
                    .desired_width(f32::INFINITY),
            );
            ui.horizontal(|ui| {
                if Button::new("Import plan").accent().show(ui).clicked() {
                    *action = Some(PlanManagerAction::ApplyImport);
                }
                if Button::new("Cancel").show(ui).clicked() {
                    *action = Some(PlanManagerAction::CancelInline);
                }
            });
        }
        SimulationPlanManagerMode::Campaign => {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Campaign name");
                mono_input(
                    ui,
                    &mut draft.campaign_name,
                    ui.available_width().min(360.0),
                );
            });
            ui.label(
                "Each selected plan is frozen now, then dispatched in declared table order with its own run, job, dataset, and manifest identity.",
            );
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
                        let mut included = draft.campaign_member_ids.contains(&record.id);
                        if ui.checkbox(&mut included, "").changed() {
                            if included {
                                if !draft.campaign_member_ids.contains(&record.id) {
                                    draft.campaign_member_ids.push(record.id);
                                }
                            } else {
                                draft.campaign_member_ids.retain(|id| *id != record.id);
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
                    draft.campaign_member_ids.len(),
                    combined_tasks
                ),
            );
            ui.horizontal(|ui| {
                if Button::new("Queue reviewed campaign")
                    .accent()
                    .enabled(draft.campaign_member_ids.len() >= 2)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(PlanManagerAction::ApplyCampaign);
                }
                if Button::new("Cancel").show(ui).clicked() {
                    *action = Some(PlanManagerAction::CancelInline);
                }
            });
        }
        SimulationPlanManagerMode::ConfirmArchive => {
            ui.add_space(10.0);
            ui.label(format!(
                "Archive '{}'? The plan remains recoverable and its {} immutable result reference{} remain unchanged.",
                selected.name,
                selected.results,
                plan_plural_suffix(selected.results)
            ));
            ui.horizontal(|ui| {
                if Button::new("Archive plan")
                    .accent()
                    .destructive(true)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(PlanManagerAction::ConfirmArchive);
                }
                if Button::new("Cancel").show(ui).clicked() {
                    *action = Some(PlanManagerAction::CancelInline);
                }
            });
        }
    }
}

/// Everything the catalog knows about the selected plan, in the order a reader
/// asks for it: which plan this is, what it declares as work, and what it owns
/// that a run would consume or a comparison would diff.
///
/// The mockup's aside is six rows under one heading. Two of those six —
/// a per-plan design and testbench binding, and a named execution profile —
/// have no owner anywhere in RSpice, so this states the strongest facts the
/// catalog does own instead of stubbing them: the reference corner, the run
/// set's own forecast, the model closure, and the plan-owned record counts.
/// That is more facts than fit one list, so they are grouped; eleven flat rows
/// in this column read as a wall rather than as three answers.
///
/// Every quantity here comes off [`PlanCatalogRecord`], never off a second
/// derivation — including the two that go absent with an unvalidated run set.
fn selected_plan_properties(ui: &mut Ui, selected: &PlanCatalogRecord) {
    let t = Tokens::get(ui.ctx());
    workflow_section_heading(ui, "Selected plan");
    property_row(ui, "Name", &selected.name);
    property_row(ui, "Stable identity", &selected.id.to_string());
    property_row(ui, "Revision", &selected.revision.to_string());

    workflow_section_heading(ui, "Declared work");
    property_row(
        ui,
        "Reference PVT corner",
        &reference_pvt_label(selected.reference_pvt),
    );
    if let (Some(points), Some(tasks)) = (selected.point_count(), selected.task_count()) {
        property_row(
            ui,
            "Declared run set",
            &format!(
                "{points} PVT point{} · {tasks} task{}",
                plan_plural_suffix(points),
                plan_plural_suffix(tasks)
            ),
        );
    } else {
        property_row_status(
            ui,
            "Declared run set",
            RUN_SET_DOES_NOT_VALIDATE,
            t.color.err,
            StatusMark::Failure,
        );
    }
    if let (Some(duration), Some(storage)) =
        (selected.estimated_duration(), selected.estimated_storage())
    {
        property_row(ui, "Modelled cost", &format!("{duration} · {storage}"));
    } else {
        property_row_status(
            ui,
            "Modelled cost",
            RUN_SET_DOES_NOT_VALIDATE,
            t.color.err,
            StatusMark::Failure,
        );
    }
    property_row(
        ui,
        "Model closure",
        &format!(
            "{} binding{}",
            selected.model_bindings,
            plan_plural_suffix(selected.model_bindings)
        ),
    );

    workflow_section_heading(ui, "Plan-owned records");
    property_row(
        ui,
        "Variables, outputs, specifications",
        &format!(
            "{} · {} · {}",
            selected.design_variables, selected.saved_outputs, selected.specifications
        ),
    );
    property_row(
        ui,
        "Regression baseline",
        &selected.regression_baseline.map_or_else(
            || "no run pinned".to_owned(),
            |run| format!("run {run}"),
        ),
    );
    property_row(ui, "Source lineage", &lineage_label(selected));
    property_row(
        ui,
        "Immutable result references",
        &selected.results.to_string(),
    );
}

/// The plan's nominal point, corner first, the way the workbench chrome's own
/// PVT control reads it.
fn reference_pvt_label(point: ReferencePvtPoint) -> String {
    format!(
        "{} · {:.1} °C",
        point.process.short_name(),
        point.temperature_celsius
    )
}

/// Where the plan came from. A clone and an import both record a source plan
/// and the exact revision they were taken at, and a plan with neither is a
/// root — which is a fact worth stating rather than an empty row.
fn lineage_label(selected: &PlanCatalogRecord) -> String {
    match (
        selected.lineage.source_plan_id(),
        selected.lineage.source_revision(),
    ) {
        (Some(source), Some(revision)) => {
            format!("from {source} · revision {}", revision.get())
        }
        _ => "root plan · no source".to_owned(),
    }
}

fn handle_plan_manager_action(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: SimulationPlanManagerDraft,
    action: Option<PlanManagerAction>,
) {
    let result = match action {
        Some(PlanManagerAction::Create) => {
            draft.mode = SimulationPlanManagerMode::Create;
            draft.name = "New simulation plan".to_owned();
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::Campaign) => {
            draft.mode = SimulationPlanManagerMode::Campaign;
            draft.validation_error = None;
            draft.campaign_member_ids.clear();
            for record in plan_catalog_records(app)
                .into_iter()
                .filter(|record| !record.archived)
                .take(2)
            {
                draft.campaign_member_ids.push(record.id);
            }
            None
        }
        Some(PlanManagerAction::Rename) => {
            draft.mode = SimulationPlanManagerMode::Rename;
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::Compare) => {
            draft.mode = SimulationPlanManagerMode::Compare;
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::Export) => {
            match export_simulation_plan_package(app, draft.selected_plan_id) {
                Ok(json) => {
                    draft.exchange_text = json;
                    draft.mode = SimulationPlanManagerMode::Export;
                    draft.validation_error = None;
                    None
                }
                Err(error) => Some(Err(error)),
            }
        }
        Some(PlanManagerAction::Import) => {
            draft.mode = SimulationPlanManagerMode::Import;
            draft.name = "Imported simulation plan".to_owned();
            draft.exchange_text.clear();
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::CopyExport) => {
            ctx.copy_text(draft.exchange_text.clone());
            app.state.ui.toasts.success(
                ctx,
                "Simulation plan copied",
                "Portable plan JSON was copied to the clipboard.",
            );
            None
        }
        Some(PlanManagerAction::ApplyImport) => Some(
            commit_import_simulation_plan(app, &draft.exchange_text, &draft.name).map(
                |(id, message)| {
                    draft.selected_plan_id = id;
                    draft.mode = SimulationPlanManagerMode::Browse;
                    message
                },
            ),
        ),
        Some(PlanManagerAction::ApplyCampaign) => {
            match commit_simulation_campaign(app, &draft.campaign_name, &draft.campaign_member_ids)
            {
                Ok(message) => {
                    app.state.workbench.simulation_workflow = None;
                    app.state
                        .ui
                        .toasts
                        .success(ctx, "Simulation campaign queued", message);
                    return;
                }
                Err(error) => Some(Err(error)),
            }
        }
        Some(PlanManagerAction::Archive) => {
            draft.mode = SimulationPlanManagerMode::ConfirmArchive;
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::CancelInline) => {
            draft.mode = SimulationPlanManagerMode::Browse;
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::Clone) => {
            let name = simulation_plan_catalog_entry(&app.state.sim_setup, draft.selected_plan_id)
                .map(|entry| entry.0)
                .unwrap_or_else(|| "Simulation plan".to_owned());
            app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::ClonePlan(
                ClonePlanDraft::for_source(draft.selected_plan_id, &name),
            ));
            return;
        }
        Some(PlanManagerAction::ApplyCreate) => {
            Some(commit_create_plan(app, &draft.name).map(|(id, message)| {
                draft.selected_plan_id = id;
                draft.mode = SimulationPlanManagerMode::Browse;
                message
            }))
        }
        Some(PlanManagerAction::ApplyRename) => {
            Some(commit_rename_plan(app, draft.selected_plan_id, &draft.name))
        }
        Some(PlanManagerAction::ConfirmArchive) => {
            Some(commit_archive_plan(app, draft.selected_plan_id))
        }
        Some(PlanManagerAction::Restore) => Some(commit_restore_plan(app, draft.selected_plan_id)),
        None => None,
    };

    match result {
        Some(Ok(message)) => {
            draft.mode = SimulationPlanManagerMode::Browse;
            draft.validation_error = None;
            app.state
                .ui
                .toasts
                .success(ctx, "Simulation plan updated", message);
        }
        None => {}
        Some(Err(error)) => draft.validation_error = Some(error),
    }
    app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::PlanManager(draft));
}

const fn plan_plural_suffix(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

fn commit_simulation_campaign(
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

fn commit_activate_plan(app: &mut RSpiceApp, id: SimulationPlanId) -> Result<String, String> {
    if app.state.sim_setup.stable_analysis_plan()?.id() == id {
        return Ok(format!(
            "'{}' remains the active editable plan.",
            app.state.sim_setup.active_plan_name()
        ));
    }
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    workspace.migrate_inactive_plan_data(id);
    setup.activate_plan(id).map_err(|error| error.to_string())?;
    workspace.sync_legacy_specs_projection(id);
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    let name = setup.active_plan_name().to_string();
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.invalidate_simulation_preflight();
    Ok(format!(
        "Opened '{name}' atomically with its complete plan-owned configuration."
    ))
}

fn commit_create_plan(
    app: &mut RSpiceApp,
    name: &str,
) -> Result<(SimulationPlanId, String), String> {
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    let id = setup.create_plan(name).map_err(|error| error.to_string())?;
    workspace.migrate_inactive_plan_data(id);
    workspace.sync_legacy_specs_projection(id);
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    let name = setup.active_plan_name().to_string();
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.invalidate_simulation_preflight();
    Ok((
        id,
        format!("Created and activated fresh root plan '{name}' with identity {id}."),
    ))
}

fn commit_rename_plan(
    app: &mut RSpiceApp,
    id: SimulationPlanId,
    name: &str,
) -> Result<String, String> {
    let mut setup = app.state.sim_setup.clone();
    setup
        .rename_plan(id, name)
        .map_err(|error| error.to_string())?;
    setup
        .validate_plan_catalog()
        .map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    app.invalidate_simulation_preflight();
    Ok(format!(
        "Renamed simulation plan {id} to '{}'; identity and immutable result references were preserved.",
        name.trim()
    ))
}

fn commit_archive_plan(app: &mut RSpiceApp, id: SimulationPlanId) -> Result<String, String> {
    let mut setup = app.state.sim_setup.clone();
    setup.archive_plan(id).map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    Ok(format!(
        "Archived simulation plan {id}; configuration and result provenance remain recoverable."
    ))
}

fn commit_restore_plan(app: &mut RSpiceApp, id: SimulationPlanId) -> Result<String, String> {
    let mut setup = app.state.sim_setup.clone();
    setup.restore_plan(id).map_err(|error| error.to_string())?;
    app.state.sim_setup = setup;
    Ok(format!(
        "Restored simulation plan {id} to the working catalog."
    ))
}

pub(super) fn export_simulation_plan_package(
    app: &RSpiceApp,
    id: SimulationPlanId,
) -> Result<String, String> {
    let plan = app
        .state
        .sim_setup
        .export_plan(id)
        .map_err(|error| error.to_string())?;
    let payload = app
        .state
        .workspace
        .plan_data(id)
        .cloned()
        .ok_or_else(|| format!("Simulation plan {id} has no plan-owned payload to export."))?;
    serde_json::to_string_pretty(&PortableSimulationPlanPackage {
        format: SIMULATION_PLAN_PACKAGE_FORMAT.to_owned(),
        version: SIMULATION_PLAN_PACKAGE_VERSION,
        plan,
        payload,
    })
    .map_err(|error| format!("Could not serialize simulation plan {id}: {error}"))
}

pub(super) fn commit_import_simulation_plan(
    app: &mut RSpiceApp,
    json: &str,
    name: &str,
) -> Result<(SimulationPlanId, String), String> {
    if json.trim().is_empty() {
        return Err("Paste a portable RSpice simulation-plan package before importing.".to_owned());
    }
    let mut package: PortableSimulationPlanPackage = serde_json::from_str(json)
        .map_err(|error| format!("Simulation-plan package JSON is invalid: {error}"))?;
    if package.format != SIMULATION_PLAN_PACKAGE_FORMAT {
        return Err(format!(
            "Unsupported simulation-plan package format '{}'.",
            package.format
        ));
    }
    if package.version != SIMULATION_PLAN_PACKAGE_VERSION {
        return Err(format!(
            "Unsupported simulation-plan package version {}; this build accepts version {}.",
            package.version, SIMULATION_PLAN_PACKAGE_VERSION
        ));
    }
    package.plan.name = crate::workbench::app_state::SimulationPlanName::new(name.to_owned())
        .map_err(|error| error.to_string())?;
    app.state
        .model_library_manager
        .validate_simulation_plan_bindings(&package.plan.model_bindings)
        .map_err(|error| format!("Imported model bindings require review: {error}"))?;

    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    let outcome = setup
        .import_plan(package.plan)
        .map_err(|error| error.to_string())?;
    workspace
        .import_plan_data(
            outcome.cloned_plan_id,
            &package.payload,
            &outcome.analysis_identity_map,
        )
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    crate::io::ProjectExecutionContext::from_state(
        workspace.project.id(),
        &setup,
        &app.state.model_library_manager,
    )?;

    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    let id = outcome.cloned_plan_id;
    let imported_name = setup.active_plan_name().to_string();
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.invalidate_simulation_preflight();
    Ok((
        id,
        format!(
            "Imported and activated '{imported_name}' as new local identity {id}; source lineage was retained and result references were not copied."
        ),
    ))
}

// `raster.rs` holds the offscreen dialog renders and is declared by the wave
// that writes them, so its declaration is still absent here.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProcessCorner;

    /// One render of the Browse aside, read two ways.
    #[cfg(not(target_arch = "wasm32"))]
    struct RenderedAside {
        /// The projection the aside was painted from. `records.rs` owns proving
        /// these numbers come from their owners; this module owns proving the
        /// aside states them.
        record: PlanCatalogRecord,
        /// Every string the aside painted, in paint order — section headings
        /// included, which carry no widget and so no accessibility node.
        painted: Vec<String>,
        /// Each property row as `(label, value)`, in paint order.
        rows: Vec<(String, String)>,
    }

    /// Render the aside for one catalog entry.
    ///
    /// A row's value is elided to fit its column, so the painted glyphs are only
    /// what survived the fit while the accessibility node carries the whole
    /// fact. Neither source alone is enough: `property_row` publishes its node
    /// without widget info, so egui never stamps bounds on it and the tree has
    /// no order to sort by. So the order comes from the paint and the values
    /// come from the nodes, which also means a label that is announced but never
    /// painted does not count as a row.
    #[cfg(not(target_arch = "wasm32"))]
    fn rendered_aside(app: &RSpiceApp, plan_id: SimulationPlanId) -> RenderedAside {
        let records = plan_catalog_records(app);
        let selected = records
            .iter()
            .find(|record| record.id == plan_id)
            .expect("the requested plan is projected");
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(980.0, 760.0))),
                ..Default::default()
            },
            |root| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(root, |ui| selected_plan_properties(ui, selected));
            },
        );
        let announced = output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("AccessKit aside tree")
            .nodes
            .iter()
            .filter_map(|(_, node)| Some((node.label()?.to_owned(), node.value()?.to_owned())))
            .collect::<std::collections::BTreeMap<_, _>>();
        let mut painted = Vec::new();
        let mut rows = Vec::new();
        for shape in &output.shapes {
            if let egui::epaint::Shape::Text(text) = &shape.shape {
                let painted_text = text.galley.job.text.clone();
                if let Some(value) = announced.get(&painted_text) {
                    rows.push((painted_text.clone(), value.clone()));
                }
                painted.push(painted_text);
            }
        }
        RenderedAside {
            record: selected.clone(),
            painted,
            rows,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn aside_value<'rows>(rows: &'rows [(String, String)], label: &str) -> &'rows str {
        rows.iter()
            .find(|(row_label, _)| row_label == label)
            .map_or_else(
                || panic!("the aside has no '{label}' row: {rows:?}"),
                |(_, value)| value.as_str(),
            )
    }

    /// The aside is the manager's only statement of what the selected plan is
    /// and what it declares. It named the plan, its identity and its result
    /// count and stopped there, so six facts the projection had already
    /// collected — the corner, the forecast, the model closure, the plan-owned
    /// record counts, the pinned baseline, and the source plan — were
    /// unreachable from the surface that exists to compare plans.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_browse_aside_states_every_fact_the_catalog_owns_about_the_selected_plan() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        app.state.workspace.migrate_active_plan_data(plan_id);
        let mut setup = app.state.sim_setup.clone();
        setup
            .set_reference_pvt(ProcessCorner::FF, -40.0)
            .expect("a physical reference corner");
        app.state.sim_setup = setup;

        let RenderedAside {
            record,
            painted,
            rows,
        } = rendered_aside(&app, plan_id);

        // Eleven rows are grouped under three headings rather than listed flat.
        assert_eq!(
            painted
                .iter()
                .filter(|text| matches!(
                    text.as_str(),
                    "Selected plan" | "Declared work" | "Plan-owned records"
                ))
                .collect::<Vec<_>>(),
            ["Selected plan", "Declared work", "Plan-owned records"]
        );
        assert_eq!(
            rows.iter()
                .map(|(label, _)| label.as_str())
                .collect::<Vec<_>>(),
            [
                "Name",
                "Stable identity",
                "Revision",
                "Reference PVT corner",
                "Declared run set",
                "Modelled cost",
                "Model closure",
                "Variables, outputs, specifications",
                "Regression baseline",
                "Source lineage",
                "Immutable result references",
            ]
        );

        let plan = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan");
        assert_eq!(
            aside_value(&rows, "Name"),
            app.state.sim_setup.active_plan_name().as_str()
        );
        assert_eq!(aside_value(&rows, "Stable identity"), plan_id.to_string());
        assert_eq!(
            aside_value(&rows, "Revision"),
            plan.revision().get().to_string()
        );
        assert_eq!(aside_value(&rows, "Reference PVT corner"), "FF · -40.0 °C");

        // The declared workload is the forecast's, named as points and tasks.
        let points = record.point_count().expect("the default run set validates");
        let tasks = record.task_count().expect("the default run set validates");
        let run_set = aside_value(&rows, "Declared run set");
        assert!(
            run_set.contains("PVT point") && run_set.contains("task"),
            "{run_set}"
        );
        assert!(
            run_set.contains(&points.to_string()) && run_set.contains(&tasks.to_string()),
            "the run-set row must state the forecast's own counts: {run_set}"
        );
        assert_eq!(
            aside_value(&rows, "Modelled cost"),
            format!(
                "{} · {}",
                record.estimated_duration().expect("a validated forecast"),
                record.estimated_storage().expect("a validated forecast")
            )
        );

        let bindings = app.state.sim_setup.model_bindings.len();
        assert_eq!(
            aside_value(&rows, "Model closure"),
            format!("{bindings} binding{}", plan_plural_suffix(bindings))
        );
        assert_eq!(
            aside_value(&rows, "Variables, outputs, specifications"),
            format!(
                "{} · {} · {}",
                record.design_variables, record.saved_outputs, record.specifications
            )
        );
        assert_eq!(aside_value(&rows, "Regression baseline"), "no run pinned");
        assert_eq!(
            aside_value(&rows, "Source lineage"),
            "root plan · no source"
        );
        assert_eq!(
            aside_value(&rows, "Immutable result references"),
            record.results.to_string()
        );

        // The baseline row is the payload's, not a fixed string: pinning a run
        // has to change what it says.
        let run = crate::product::RunId::new();
        app.state
            .workspace
            .ensure_active_plan_data(plan_id)
            .regression_baseline_run = Some(run);
        assert_eq!(
            aside_value(&rendered_aside(&app, plan_id).rows, "Regression baseline"),
            format!("run {run}")
        );
    }

    /// A run set that does not validate carries no forecast, and every quantity
    /// derived from it is absent with it. Zeros there would read as "this plan
    /// declares no work", which is a different claim from work that cannot be
    /// expanded — and the one the reader would act on.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn an_undeclarable_run_set_says_so_instead_of_reporting_zeros() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let mut setup = app.state.sim_setup.clone();
        // Nested composition with a zero maximum depth is a declared run-space
        // error, so there is no workload to forecast.
        setup.run_set.composition.mode =
            crate::simulation::run_set::RunSetCompositionMode::Nested;
        setup.run_set.composition.maximum_depth = 0;
        app.state.sim_setup = setup;

        let RenderedAside { record, rows, .. } = rendered_aside(&app, plan_id);

        assert!(
            record.point_count().is_none(),
            "the fixture's run set must not validate for this case to mean anything"
        );
        for label in ["Declared run set", "Modelled cost"] {
            let value = aside_value(&rows, label);
            assert_eq!(value, RUN_SET_DOES_NOT_VALIDATE);
            assert!(
                !value.contains('0'),
                "'{label}' reported a zero for an absent forecast: {value}"
            );
        }
        // The plan is still identified and its records are still stated: an
        // invalid run space is not a reason to stop describing the plan.
        assert_eq!(aside_value(&rows, "Stable identity"), plan_id.to_string());
        assert_eq!(
            aside_value(&rows, "Reference PVT corner"),
            reference_pvt_label(app.state.sim_setup.reference_pvt)
        );
    }

    /// A clone names the plan and revision it came from, and the source it
    /// leaves behind keeps reporting its own corner. Both facts used to be
    /// unreadable: the lineage had no row, and a stored plan had no accessor
    /// for its reference point, so every inactive plan's corner read as absent.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_clone_names_its_source_and_the_source_keeps_its_own_reference_corner() {
        let mut app = RSpiceApp::test_instance();
        let source = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan");
        let source_id = source.id();
        let source_revision = source.revision();
        let source_corner = app.state.sim_setup.reference_pvt;
        let mut setup = app.state.sim_setup.clone();
        let outcome = setup
            .clone_active_plan(
                "Cloned characterization",
                crate::workbench::app_state::SimulationPlanCloneOptions::ALL_PLAN_CONTENTS,
            )
            .expect("the active plan clones");
        setup
            .set_reference_pvt(ProcessCorner::SS, 125.0)
            .expect("a physical reference corner");
        app.state.sim_setup = setup;

        let clone_rows = rendered_aside(&app, outcome.cloned_plan_id).rows;
        assert_eq!(
            aside_value(&clone_rows, "Source lineage"),
            format!("from {source_id} · revision {}", source_revision.get())
        );
        assert_eq!(aside_value(&clone_rows, "Reference PVT corner"), "SS · 125.0 °C");

        let source_rows = rendered_aside(&app, source_id).rows;
        assert_eq!(
            aside_value(&source_rows, "Source lineage"),
            "root plan · no source"
        );
        assert_eq!(
            aside_value(&source_rows, "Reference PVT corner"),
            reference_pvt_label(source_corner),
            "the retained source reports the corner it was cloned at, not the \
             active plan's and not 'unknown'"
        );
    }
}
