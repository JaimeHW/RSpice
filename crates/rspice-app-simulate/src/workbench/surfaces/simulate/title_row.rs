//! The Analyses route's title row: the plan's heading and configuration row,
//! the hop to the plan's last run, and the band of a run held for memory.

use super::*;

const TITLE_ACTION_SPACING: f32 = 6.0;
/// Air between the plan's name and the configuration row under it.
const CONFIGURATION_ROW_GAP: f32 = 4.0;
/// The narrowest the plan heading is allowed to become before the title row
/// gives up on holding the actions beside it.
const TITLE_HEADING_MIN_WIDTH: f32 = 220.0;

/// The band of the plan's run held because it needs more memory than the
/// machine has free, and what the reader answers there. Nothing while no
/// run is held.
pub(super) fn memory_band(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let answer = rspice_app_workbench::workbench::surfaces::memory_bands::held_run(
        ui,
        cx.runs,
        cx.wb.messages(),
        rspice_simulation::execution::run_memory::RunOf::Design,
    );
    match answer {
        Some(crate::ui::widgets::HeldRunAnswer::RunAnyway) => cx
            .wb
            .outbox
            .push(crate::workbench::framework::requests::RunPlanAnyway),
        Some(crate::ui::widgets::HeldRunAnswer::Stop) => cx.execute(ids::STOP_SIMULATION),
        None => {}
    }
}

pub(super) fn plan_heading(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>, surface_width: f32) {
    let plan_name = cx.model.sim_setup.active_plan_name().as_str().to_owned();
    // The heading is the plan's name. What the plan holds is the rail's, and
    // its last run is the hop's: the hop's label and its own availability read
    // the one derivation, so they cannot disagree.
    let prior = prior_plan_run(cx.model);
    let plan_available = cx.model.sim_setup.stable_analysis_plan().is_ok();

    // The split toggle sits beside the hop because both answer "and now show
    // me what the last run produced" — one by leaving this page, one by
    // keeping it.
    let open_prior_label = prior.map_or_else(
        || "Open prior run".to_owned(),
        |prior| format!("Open Run {}", prior.sequence),
    );
    let split_availability = cx.foreign_availability(ids::SHOW_PLOT);
    let split_enabled = split_availability.is_available();
    let split_reason = match split_availability {
        crate::workbench::framework::command_registry::CommandAvailability::Disabled(reason) => {
            Some(reason)
        }
        crate::workbench::framework::command_registry::CommandAvailability::Available
        | crate::workbench::framework::command_registry::CommandAvailability::Hidden => None,
    };
    let split_label = if cx
        .wb
        .dock_shows(crate::workbench::state::workbench_session::ConsolePage::Plot)
    {
        "Hide plot"
    } else {
        "Show plot"
    };
    let mut manage_plans = false;
    let mut clone_plan = false;
    let mut validate = false;
    let mut open_prior = false;
    let mut toggle_split = false;
    // The standing every other setup route states, from the same owner. This
    // route already offers the *action* — "Validate plan" is `SimulateCommand::Preflight`
    // — and stated none of the standing, so the one page that lists what a run
    // will dispatch was the one page that never said whether the report
    // authorizing it still held.
    let mut rerun_preflight = false;
    let configuration_choices = plan_configuration::ConfigurationChoices::of(cx.model);
    let picked_configuration;
    let chip = pages::PreflightChip::resolve(cx.model, cx.runs);
    let chip_reserve = chip.as_ref().map_or(0.0, |chip| chip.reserve(ui));
    let actions_width =
        title_action_group_width(ui, &open_prior_label, split_label, TITLE_ACTION_SPACING);
    let heading_width = ui.available_width() - actions_width - chip_reserve - TITLE_ACTION_SPACING;
    // The actions at their own widths, in one row, as the wide title row
    // and the row under a narrow centre's heading both draw them.
    let mut natural_actions = |ui: &mut Ui| {
        if let Some(chip) = &chip {
            rerun_preflight = chip.show(ui);
        }
        open_prior = prior_run_button(ui, &open_prior_label, prior.is_some());
        toggle_split = split_button(ui, split_label, split_enabled, split_reason);
        manage_plans = Button::new("Plans")
            .enabled(plan_available)
            .show(ui)
            .clicked();
        clone_plan = Button::new("Clone plan")
            .icon(Icon::Copy)
            .enabled(plan_available)
            .show(ui)
            .clicked();
        validate = Button::new("Validate plan")
            .icon(Icon::Check)
            .accent()
            .enabled(plan_available)
            .show(ui)
            .clicked();
    };
    let desktop = surface_width > TITLE_ACTION_STACK_BREAKPOINT;
    if desktop && heading_width >= TITLE_HEADING_MIN_WIDTH {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = TITLE_ACTION_SPACING;
            ui.allocate_ui_with_layout(
                vec2(heading_width, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    // `allocate_ui_with_layout` advances its parent past the
                    // content's `min_rect`, not past the size asked for, and a
                    // top-down block of labels is only as wide as its widest
                    // label. Without this the block collapsed to the text, the
                    // action group started that much early, and its right edge
                    // sat at the same x on a 1000-point pane and a 2560-point
                    // one — 114 points shy of the inset at 1100, and 1569 shy
                    // at 2560. It also bounds the galleys: a description longer
                    // than the reserve wraps inside the heading's own column
                    // instead of running under the group.
                    ui.set_width(heading_width);
                    heading(ui, &plan_name, None);
                },
            );
            natural_actions(ui);
        });
        // Under the whole title row, not in the heading's column: the column
        // is what the actions leave, which on a tablet is too narrow for the
        // configuration's name.
        ui.add_space(CONFIGURATION_ROW_GAP);
        picked_configuration = plan_configuration::configuration_row(ui, &configuration_choices);
    } else if desktop && actions_width + chip_reserve <= ui.available_width() {
        // A centre the docks narrowed keeps the actions in one row under the
        // heading, at their own widths: wrapped, the last of them dropped
        // to a line of its own under four of unequal widths.
        heading(ui, &plan_name, None);
        ui.add_space(CONFIGURATION_ROW_GAP);
        picked_configuration = plan_configuration::configuration_row(ui, &configuration_choices);
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = TITLE_ACTION_SPACING;
            natural_actions(ui);
        });
    } else {
        heading(ui, &plan_name, None);
        ui.add_space(CONFIGURATION_ROW_GAP);
        picked_configuration = plan_configuration::configuration_row(ui, &configuration_choices);
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = TITLE_ACTION_SPACING;
            let action_width = ((ui.available_width() - 12.0) / 3.0).max(1.0);
            if let Some(chip) = &chip {
                rerun_preflight = chip.show(ui);
            }
            open_prior = prior_run_button(ui, &open_prior_label, prior.is_some());
            toggle_split = split_button(ui, split_label, split_enabled, split_reason);
            manage_plans = Button::new("Plans")
                .min_width(action_width)
                .max_width(action_width)
                .enabled(plan_available)
                .show(ui)
                .clicked();
            clone_plan = Button::new("Clone plan")
                .icon(Icon::Copy)
                .min_width(action_width)
                .max_width(action_width)
                .enabled(plan_available)
                .show(ui)
                .clicked();
            validate = Button::new("Validate plan")
                .icon(Icon::Check)
                .accent()
                .min_width(action_width)
                .max_width(action_width)
                .enabled(plan_available)
                .show(ui)
                .clicked();
        });
    }
    if let Some(design) = picked_configuration
        && let Err(error) = plan_configuration::commit_design_configuration(cx, design)
    {
        cx.wb
            .prompt(format!("The plan's configuration was not changed: {error}"));
    }
    if manage_plans {
        cx.execute(SimulateCommand::ManagePlans.key());
    }
    if clone_plan {
        cx.state.simulation_workflow = Some(SimulationWorkflowDialog::ClonePlan(
            ClonePlanDraft::for_source(
                cx.model.sim_setup.stable_analysis_plan().unwrap().id(),
                &plan_name,
            ),
        ));
    }
    // One dispatch for two controls, because they are one request: the chip is
    // the standing and the offer to refresh it, and the accent action is the
    // same refresh named as a step.
    if validate || rerun_preflight {
        cx.execute(SimulateCommand::Preflight.key());
    }
    if open_prior && let Some(PriorPlanRun { index, .. }) = prior {
        // Selecting first is what carries the object: the command owns the
        // workspace half of the destination and nothing else.
        if cx.model.simulation.select_run(index) {
            cx.execute(ids::OPEN_RUN_IN_RESULTS);
        }
    }
    if toggle_split {
        cx.execute(ids::SHOW_PLOT);
    }
}

/// What the five title-row actions will take, asked before the heading beside
/// them is laid out.
///
/// The heading grows into whatever the actions leave, so the row cannot be laid
/// out until their real extent is known. The width this replaces was authored
/// beside the row — a second account of the same arithmetic, and one that had
/// drifted: it reserved 570 points for a group that measures more, so at the
/// 1000-point gate the accent action was clipped by the surface edge.
fn title_action_group_width(
    ui: &mut Ui,
    open_prior_label: &str,
    split_label: &str,
    spacing: f32,
) -> f32 {
    let widths = [
        Button::new(open_prior_label).measured_width(ui),
        Button::new(split_label).measured_width(ui),
        Button::new("Plans").measured_width(ui),
        Button::new("Clone plan")
            .icon(Icon::Copy)
            .measured_width(ui),
        Button::new("Validate plan")
            .icon(Icon::Check)
            .accent()
            .measured_width(ui),
    ];
    widths.iter().sum::<f32>() + spacing * (widths.len() - 1) as f32
}

/// The hop to the dataset this plan last produced.
///
/// Disabled rather than hidden when there is none: "no prior dataset" is the
/// heading's own claim, and a control that vanishes with it would leave the
/// reader wondering whether one ever existed.
fn prior_run_button(ui: &mut Ui, label: &str, available: bool) -> bool {
    let response = Button::new(label).enabled(available).show(ui);
    if !available {
        response.on_hover_text(
            "This plan has produced no retained dataset yet, so there is nothing to open.",
        );
        return false;
    }
    response
        .on_hover_text("Activate this plan's newest retained dataset in Results")
        .clicked()
}

/// Show the retained result's plot in the dock under this page, rather than
/// leaving the page for it.
fn split_button(ui: &mut Ui, label: &str, enabled: bool, reason: Option<&str>) -> bool {
    let response = Button::new(label).enabled(enabled).show(ui);
    if !enabled {
        response.on_hover_text(reason.unwrap_or("the plot is unavailable on this page"));
        return false;
    }
    response
        .on_hover_text("Show the retained result's plot in the dock under this plan")
        .clicked()
}
