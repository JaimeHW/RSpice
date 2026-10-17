//! Interactive tuning.
//!
//! Live parameter adjustment against the last run, with the dirty tracking
//! that decides when the displayed result no longer matches the tuned
//! values. The page is the shared Verify grammar around its two working
//! parts: the variable editors on the left and the response they produce on
//! the right, with the limits judged against the baseline and the candidate
//! run under them.

use super::table::{Cell, Column, DataTable, TABLE_EDGE_INSET};
use super::toolbar::{PageToolbar, RunChoice, Summary, ToolbarAction, empty_line};
use super::*;

/// Least and greatest height of the variables and response panes.
const TUNING_PANE_MIN_H: f32 = 240.0;
const TUNING_PANE_MAX_H: f32 = 380.0;
/// The row over the response plot: its waveform's name and the legend.
const PLOT_HEADER_H: f32 = 20.0;

pub(super) fn sync_tuning_session(app: &mut RSpiceApp) {
    let active_plan_run = verification_run(app).and_then(|run| {
        run.prepared_receipt()
            .filter(|receipt| {
                receipt.simulation_plan_id()
                    == app
                        .state
                        .sim_setup
                        .stable_analysis_plan()
                        .ok()
                        .map(|plan| plan.id())
            })
            .map(|_| run.run_id)
    });
    let source = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| (plan.id(), plan.revision()))
        .and_then(|(plan_id, revision)| {
            app.state
                .workspace
                .content
                .plan_data(plan_id)
                .map(|payload| (plan_id, revision, payload.design_variables.clone()))
        });
    let session = &mut app.state.workbench.verification;
    let Some((plan_id, plan_revision, variables)) = source else {
        session.tuning_plan_id = None;
        session.tuning_plan_revision = None;
        session.tuning_variables.clear();
        session.tuning_instance_binding = None;
        session.tuning_selected_variable = None;
        session.tuning_focus_variable = None;
        session.tuning_baseline_run = None;
        session.tuning_review_open = false;
        return;
    };
    if session.tuning_plan_id == Some(plan_id)
        && session.tuning_plan_revision == Some(plan_revision)
    {
        return;
    }
    session.tuning_plan_id = Some(plan_id);
    session.tuning_plan_revision = Some(plan_revision);
    session.tuning_baseline_run = active_plan_run;
    session.tuning_review_open = false;
    session.tuning_instance_binding = None;
    session.tuning_selected_variable = None;
    session.tuning_focus_variable = None;
    session.tuning_variables = variables
        .into_iter()
        .map(|variable| crate::workbench::state::TuningVariableDraft {
            variable_id: variable.id,
            baseline_expression: variable.expression.clone(),
            candidate_expression: variable.expression,
            validation_error: None,
            proposed: false,
        })
        .collect();
}

pub(super) fn tuning_is_dirty(app: &RSpiceApp) -> bool {
    let session = &app.state.workbench.verification;
    session
        .tuning_variables
        .iter()
        .any(crate::workbench::state::TuningVariableDraft::is_dirty)
        || session.tuning_instance_binding.as_ref().is_some_and(
            crate::workbench::state::TuningInstanceBindingDraft::requires_schematic_edit,
        )
}

pub(super) fn tuning_is_valid(app: &RSpiceApp) -> bool {
    let session = &app.state.workbench.verification;
    let drafts = &session.tuning_variables;
    let binding_is_consistent = session
        .tuning_instance_binding
        .as_ref()
        .is_none_or(|binding| {
            drafts.iter().any(|draft| {
                draft.variable_id == binding.variable.id
                    && (!binding.creates_variable || draft.proposed)
            })
        });
    !drafts.is_empty()
        && binding_is_consistent
        && drafts.iter().all(|draft| draft.validation_error.is_none())
}

fn tuning_schematic_edit_block_reason(app: &RSpiceApp) -> Option<String> {
    app.state
        .workbench
        .verification
        .tuning_instance_binding
        .as_ref()
        .filter(|binding| binding.requires_schematic_edit())
        .filter(|_| app.state.schematic_edit_read_only())
        .map(|_| {
            "The staged instance Value binding cannot be committed because the active schematic is read-only."
                .to_owned()
        })
}

pub(super) fn tuning_commit_block_reason(app: &RSpiceApp) -> String {
    let drafts = &app.state.workbench.verification.tuning_variables;
    if drafts.is_empty() {
        return "Add at least one design variable to the active simulation plan.".to_owned();
    }
    if !tuning_is_dirty(app) {
        return "The sandbox matches the committed plan revision.".to_owned();
    }
    if let Some(reason) = tuning_schematic_edit_block_reason(app) {
        return reason;
    }
    if let Some(error) = drafts
        .iter()
        .find_map(|draft| draft.validation_error.as_deref())
    {
        return format!("Resolve the invalid candidate value: {error}");
    }
    app.state
        .simulation_run_block_reason()
        .unwrap_or_else(|| "The candidate is ready to commit and run.".to_owned())
}

pub(super) fn revert_tuning_session(app: &mut RSpiceApp) {
    let session = &mut app.state.workbench.verification;
    session.tuning_review_open = false;
    session.tuning_variables.retain(|draft| !draft.proposed);
    for draft in &mut session.tuning_variables {
        draft
            .candidate_expression
            .clone_from(&draft.baseline_expression);
        draft.validation_error = None;
    }
    session.tuning_instance_binding = None;
    session.tuning_selected_variable = None;
    session.tuning_focus_variable = None;
    session.action_problem = None;
}

fn tuning_session_variables(app: &RSpiceApp) -> Vec<crate::state::DesignVariable> {
    let session = &app.state.workbench.verification;
    let mut variables = session
        .tuning_plan_id
        .and_then(|plan_id| app.state.workspace.content.plan_data(plan_id))
        .map(|payload| payload.design_variables.clone())
        .unwrap_or_default();
    if let Some(binding) = session
        .tuning_instance_binding
        .as_ref()
        .filter(|binding| binding.creates_variable)
        && !variables
            .iter()
            .any(|variable| variable.id == binding.variable.id)
    {
        variables.push(binding.variable.clone());
    }
    variables
}

fn tuning_baseline_run(app: &RSpiceApp) -> Option<&crate::state::SimulationRun> {
    let run_id = app.state.workbench.verification.tuning_baseline_run?;
    app.state
        .simulation
        .retained
        .runs
        .iter()
        .find(|run| run.run_id == run_id)
}

fn tuning_candidate_run(app: &RSpiceApp) -> Option<&crate::state::SimulationRun> {
    let baseline_id = app.state.workbench.verification.tuning_baseline_run;
    let baseline_sequence = tuning_baseline_run(app).map(|run| run.id);
    let plan_id = app.state.workbench.verification.tuning_plan_id?;
    verification_run(app).filter(|run| {
        Some(run.run_id) != baseline_id
            && baseline_sequence.is_none_or(|sequence| run.id > sequence)
            && run
                .prepared_receipt()
                .and_then(|receipt| receipt.simulation_plan_id())
                == Some(plan_id)
    })
}

pub(super) fn commit_tuning_and_run(app: &mut RSpiceApp) -> Result<(), String> {
    if !tuning_is_dirty(app) {
        return Err("the tuning sandbox contains no provisional changes".to_owned());
    }
    if let Some(reason) = tuning_schematic_edit_block_reason(app) {
        return Err(reason);
    }
    if !tuning_is_valid(app) {
        return Err(tuning_commit_block_reason(app));
    }
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let plan_id = plan.id();
    let plan_revision = plan.revision();
    let verification = &app.state.workbench.verification;
    if verification.tuning_plan_id != Some(plan_id)
        || verification.tuning_plan_revision != Some(plan_revision)
    {
        return Err(
            "the active plan changed while the tuning sandbox was open; review its current values"
                .to_owned(),
        );
    }
    let binding = verification.tuning_instance_binding.clone();
    let changes = verification
        .tuning_variables
        .iter()
        .filter(|draft| draft.is_dirty() && !draft.proposed)
        .map(|draft| (draft.variable_id, draft.candidate_expression.clone()))
        .collect::<Vec<_>>();
    let proposed_variable = binding
        .as_ref()
        .filter(|binding| binding.creates_variable)
        .map(|binding| {
            let draft = verification
                .tuning_variables
                .iter()
                .find(|draft| draft.variable_id == binding.variable.id && draft.proposed)
                .ok_or_else(|| {
                    "the selected instance's proposed variable is missing from the sandbox"
                        .to_owned()
                })?;
            let mut variable = binding.variable.clone();
            variable.expression.clone_from(&draft.candidate_expression);
            variable.validate().map_err(|error| {
                format!(
                    "the proposed variable '{}' is invalid: {error}",
                    variable.name
                )
            })?;
            Ok::<crate::state::DesignVariable, String>(variable)
        })
        .transpose()?;
    let variable_change_count = changes.len() + usize::from(proposed_variable.is_some());
    let original_workspace = app.state.workspace.clone();
    let original_setup = app.state.sim_setup.clone();
    let original_schematic = app.state.schematic.clone();
    let original_preflight = app.state.workbench.preflight.clone();
    let mut workspace = original_workspace.clone();
    let mut setup = original_setup.clone();
    let mut schematic = original_schematic.clone();
    if !changes.is_empty() {
        workspace
            .content
            .update_design_variable_expressions(plan_id, &changes)
            .map_err(|error| error.to_string())?;
    }
    if let Some(variable) = proposed_variable {
        workspace
            .content
            .add_design_variable(plan_id, variable)
            .map_err(|error| error.to_string())?;
    }
    let binding_changed = if let Some(binding) = binding.as_ref() {
        if app.state.workspace.content.active_schematic_reference() != binding.source_view {
            return Err(format!(
                "the active schematic changed from {} while the tuning sandbox was open",
                binding.source_view.display_path()
            ));
        }
        if schematic.topology_version() != binding.source_topology_version {
            return Err(format!(
                "{} changed after its Value binding was staged; reopen Tune against the current revision",
                binding.component_name
            ));
        }
        let component = schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == binding.component_id)
            .ok_or_else(|| {
                format!(
                    "{} no longer exists in the staged schematic revision",
                    binding.component_name
                )
            })?;
        if component.name != binding.component_name || component.value != binding.source_value {
            return Err(format!(
                "{} no longer matches the Value row that opened the tuning sandbox",
                binding.component_name
            ));
        }
        if binding.requires_schematic_edit() {
            let component_id = binding.component_id;
            let binding_expression = binding.binding_expression.clone();
            let description = format!(
                "bind {} to {}",
                binding.component_name, binding.variable.name
            );
            let changed =
                schematic.bind_component_value(component_id, binding_expression, description);
            if !changed {
                return Err(format!(
                    "{} could not be bound because the schematic became read-only",
                    binding.component_name
                ));
            }
            true
        } else {
            false
        }
    } else {
        false
    };
    if binding_changed {
        workspace.save_active_schematic(&schematic);
    }
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let binding_change_count = usize::from(binding_changed);
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "committed {} tuned design variable{} and {} instance binding{}",
            variable_change_count,
            if variable_change_count == 1 { "" } else { "s" },
            binding_change_count,
            if binding_change_count == 1 { "" } else { "s" }
        ))
        .map_err(|error| error.to_string())?;

    app.state.workspace = workspace;
    app.state.sim_setup = setup;
    app.state.schematic = schematic;
    app.state.workbench.preflight = Default::default();
    if let Some(reason) = app.state.simulation_run_block_reason() {
        app.state.workspace = original_workspace;
        app.state.sim_setup = original_setup;
        app.state.schematic = original_schematic;
        app.state.workbench.preflight = original_preflight;
        return Err(format!(
            "the tuned revision was not committed because its required run is blocked: {reason}"
        ));
    }
    if let Err(error) = app
        .simulation_controller
        .prepare_run_set_for_preflight(&app.state)
    {
        app.state.workspace = original_workspace;
        app.state.sim_setup = original_setup;
        app.state.schematic = original_schematic;
        app.state.workbench.preflight = original_preflight;
        return Err(format!(
            "the tuned revision was not committed because the run could not be prepared: {}",
            error.message()
        ));
    }
    app.state.request_run_set_simulation();
    sync_tuning_session(app);
    let candidate_count = variable_change_count + binding_change_count;
    app.state.record_plan_receipt(format!(
        "Tuning receipt #{} · revision {} to {} · {} candidate{} queued.",
        receipt.sequence(),
        receipt.source_revision().get(),
        receipt.committed_revision().get(),
        candidate_count,
        if candidate_count == 1 { "" } else { "s" }
    ));
    Ok(())
}

pub(super) fn tuning_review_dialog(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app.state.workbench.verification.tuning_review_open {
        return;
    }
    if app.state.workbench.verification_page != VerificationPage::Tuning
        || !tuning_is_dirty(app)
        || !tuning_is_valid(app)
    {
        app.state.workbench.verification.tuning_review_open = false;
        return;
    }
    let variables = tuning_session_variables(app);
    let instance_binding = app
        .state
        .workbench
        .verification
        .tuning_instance_binding
        .clone();
    let mut rows = app
        .state
        .workbench
        .verification
        .tuning_variables
        .iter()
        .filter(|draft| draft.is_dirty())
        .filter_map(|draft| {
            let variable = variables
                .iter()
                .find(|variable| variable.id == draft.variable_id)?;
            let committed = if draft.proposed {
                "not in active plan".to_owned()
            } else {
                draft.baseline_expression.clone()
            };
            let delta = if draft.proposed {
                "new typed variable".to_owned()
            } else {
                tuning_delta(variable, draft)
            };
            let destination = instance_binding
                .as_ref()
                .filter(|binding| binding.variable.id == variable.id)
                .map_or_else(
                    || variable.scope.label().to_owned(),
                    |binding| {
                        format!(
                            "{} / {} Value",
                            variable.scope.label(),
                            binding.component_name
                        )
                    },
                );
            Some(vec![
                Cell::number(&variable.name),
                Cell::number(committed),
                Cell::number(&draft.candidate_expression),
                Cell::number(delta),
                Cell::text(destination),
            ])
        })
        .collect::<Vec<_>>();
    if let Some(binding) = instance_binding
        .as_ref()
        .filter(|binding| binding.requires_schematic_edit())
    {
        rows.push(vec![
            Cell::number(format!("{}.Value", binding.component_name)),
            Cell::number(&binding.source_value),
            Cell::number(&binding.binding_expression),
            Cell::number("instance binding"),
            Cell::text(binding.source_view.display_path()),
        ]);
    }
    if rows.is_empty() {
        app.state.workbench.verification.tuning_review_open = false;
        return;
    }

    let runnable = app.state.can_run_simulation();
    let block_reason = tuning_commit_block_reason(app);
    let mut open = true;
    let mut cancel = false;
    let mut commit = false;
    let viewport = ctx.content_rect().size();
    let width = (viewport.x - 24.0).clamp(420.0, 860.0);
    let window_response = egui::Window::new("Commit tuned parameters")
        .id(egui::Id::new("workbench.verify.tuning_review"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .default_width(width)
        .min_width(width)
        .max_width(width)
        .show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            egui::Frame::new()
                .fill(t.color.bg_panel_2)
                .stroke(egui::Stroke::new(1.0, t.color.border))
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Results go stale until you rerun.")
                                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                });
            ui.add_space(8.0);
            let columns = [
                Column::words("Parameter", 120.0),
                Column::words("Committed", 150.0),
                Column::words("Proposed", 150.0),
                Column::number("Delta", 110.0),
                Column::name("Destination", 120.0),
            ];
            let _ = DataTable::new("verify.tuning.review", &columns, rows.len())
                .show(ui, |index| rows[index].clone());
            ui.add_space(8.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let primary = Button::new("Commit to working revision").enabled(runnable);
                let primary = if runnable { primary.accent() } else { primary };
                if primary
                    .show(ui)
                    .on_disabled_hover_text(block_reason.as_str())
                    .clicked()
                {
                    commit = true;
                }
                if Button::new("Cancel").show(ui).clicked() {
                    cancel = true;
                }
            });
        });
    if let Some(response) = &window_response {
        ctx.memory_mut(|memory| memory.set_modal_layer(response.response.layer_id));
        ctx.accesskit_node_builder(response.response.id, |node| {
            node.set_role(egui::accesskit::Role::Dialog);
            node.set_label("Commit tuned parameters");
            node.set_description(
                "Review and transactionally commit provisional active-plan design variables.",
            );
            node.set_modal();
        });
    }

    if commit {
        let result = commit_tuning_and_run(app);
        let succeeded = result.is_ok();
        record_action(&mut app.state.workbench.verification, result);
        if succeeded {
            open = false;
        }
    }
    if cancel {
        open = false;
    }
    app.state.workbench.verification.tuning_review_open = open;
}

fn tuning_delta(
    variable: &crate::state::DesignVariable,
    draft: &crate::workbench::state::TuningVariableDraft,
) -> String {
    let resolve = |expression: &str| {
        let mut candidate = variable.clone();
        candidate.expression = expression.to_owned();
        candidate.resolved_value_si().ok()
    };
    let Some(delta) = resolve(&draft.candidate_expression)
        .zip(resolve(&draft.baseline_expression))
        .map(|(candidate, baseline)| candidate - baseline)
    else {
        return "unavailable".to_owned();
    };
    let formatted = crate::quantity::format_engineering_value(delta);
    let sign = if delta > 0.0 { "+" } else { "" };
    let unit = match variable.quantity {
        crate::state::DesignVariableQuantity::Resistance => "Ω",
        crate::state::DesignVariableQuantity::Capacitance => "F",
        crate::state::DesignVariableQuantity::Voltage => "V",
        crate::state::DesignVariableQuantity::Current => "A",
        crate::state::DesignVariableQuantity::Temperature => "K",
        crate::state::DesignVariableQuantity::Dimensionless => "",
    };
    format!("{sign}{formatted}{unit}")
}

pub(super) fn tuning(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some((_plan_id, _plan_revision, _committed_variables, specs)) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| (plan.id(), plan.revision()))
        .and_then(|(plan_id, revision)| {
            app.state
                .workspace
                .content
                .plan_data(plan_id)
                .map(|payload| {
                    (
                        plan_id,
                        revision,
                        payload.design_variables.clone(),
                        payload.specs.clone(),
                    )
                })
        })
    else {
        let _ = PageToolbar {
            title: "Tuning",
            runs: &RunChoice::default(),
            summary: None,
            actions: &[],
        }
        .show(ui);
        if empty_line(
            ui,
            "No active simulation plan. Add an analysis in the Simulation setup.",
            Some("Open analyses"),
        ) {
            Command::SimulationPage(crate::workbench::state::SimulationPage::Analyses).execute(app);
        }
        return;
    };
    let variables = tuning_session_variables(app);
    let dirty_count = app
        .state
        .workbench
        .verification
        .tuning_variables
        .iter()
        .filter(|draft| draft.is_dirty())
        .count()
        + usize::from(
            app.state
                .workbench
                .verification
                .tuning_instance_binding
                .as_ref()
                .is_some_and(
                    crate::workbench::state::TuningInstanceBindingDraft::requires_schematic_edit,
                ),
        );
    let invalid_count = app
        .state
        .workbench
        .verification
        .tuning_variables
        .iter()
        .filter(|draft| draft.validation_error.is_some())
        .count();
    let baseline = tuning_baseline_run(app).map(|run| run.id);
    let candidate = tuning_candidate_run(app).map(|run| run.id);
    let plural = |count: usize| if count == 1 { "" } else { "s" };
    let runs_text = match (candidate, baseline) {
        (Some(candidate), Some(baseline)) => Some(format!(
            "candidate Run {candidate} vs baseline Run {baseline}"
        )),
        (None, Some(baseline)) => Some(format!("baseline Run {baseline}")),
        _ => None,
    };
    let summary = if invalid_count > 0 {
        Some(Summary::failing(format!(
            "{invalid_count} invalid value{}",
            plural(invalid_count)
        )))
    } else if dirty_count > 0 {
        Some(Summary::plain(
            [
                Some(format!(
                    "{dirty_count} uncommitted change{}",
                    plural(dirty_count)
                )),
                runs_text,
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" \u{00b7} "),
        ))
    } else {
        runs_text.map(Summary::plain)
    };
    let dirty = tuning_is_dirty(app);
    let valid = tuning_is_valid(app);
    let block_reason = tuning_commit_block_reason(app);
    let actions = [
        ToolbarAction::new("Revert").enabled(dirty),
        ToolbarAction::new("Review & commit\u{2026}")
            .enabled(dirty && valid)
            .disabled_reason(&block_reason),
    ];
    let output = PageToolbar {
        title: "Tuning",
        runs: &RunChoice::default(),
        summary,
        actions: &actions,
    }
    .show(ui);
    match output.action {
        Some(0) => revert_tuning_session(app),
        Some(1) => app.state.workbench.verification.tuning_review_open = true,
        _ => {}
    }
    action_problem_line(ui, app);

    if variables.is_empty() {
        if empty_line(
            ui,
            "No design variables are configured. Add variables in the Simulation setup.",
            Some("Open variables"),
        ) {
            Command::SimulationPage(crate::workbench::state::SimulationPage::Variables)
                .execute(app);
        }
        return;
    }
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    // Whole points: the table under the panes starts on the pixel grid, so
    // the hairline between them is one line rather than two faint ones.
    let height = (ui.available_height() * 0.55)
        .clamp(TUNING_PANE_MIN_H, TUNING_PANE_MAX_H)
        .round();
    let left_width = (width * 0.4).clamp(280.0, 420.0).min(width).round();
    let top = ui.cursor().top();
    let left = ui.max_rect().left();
    let variables_rect =
        egui::Rect::from_min_size(egui::pos2(left, top), egui::vec2(left_width, height));
    let plot_rect = egui::Rect::from_min_max(
        egui::pos2(left + left_width + 1.0, top),
        egui::pos2(left + width, top + height),
    );
    let _ = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    #[cfg(test)]
    {
        super::raster::probe(ui, "tuning.variables", variables_rect);
        super::raster::probe(ui, "tuning.response", plot_rect);
    }
    ui.painter().vline(
        variables_rect.right() + 0.5,
        variables_rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let mut variables_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(variables_rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    variables_ui.set_clip_rect(variables_rect.intersect(ui.clip_rect()));
    tuning_variables_pane(&mut variables_ui, app, &variables);
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(plot_rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    plot_ui.set_clip_rect(plot_rect.intersect(ui.clip_rect()));
    tuning_response_pane(&mut plot_ui, app);
    ui.painter().hline(
        left..=left + width,
        top + height + 0.5,
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.add_space(1.0);
    tuning_specification_table(ui, app, &specs, dirty_count > 0);
}

/// The variable editors, each a name, its value, a slider over its allowed
/// range and the committed value it started from.
fn tuning_variables_pane(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    variables: &[crate::state::DesignVariable],
) {
    crate::workbench::design_system::section_header(ui, "Variables", None);
    ScrollArea::vertical()
        .id_salt("workbench.verify.tuning.variables")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            crate::workbench::design_system::panel_body(ui, |ui| {
                for variable in variables {
                    tuning_variable_editor(ui, app, variable);
                }
            });
        });
}

fn tuning_variable_editor(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    variable: &crate::state::DesignVariable,
) {
    let Some(index) = app
        .state
        .workbench
        .verification
        .tuning_variables
        .iter()
        .position(|draft| draft.variable_id == variable.id)
    else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    let proposed = app.state.workbench.verification.tuning_variables[index].proposed;
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(&variable.name)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(if proposed {
                    format!("Proposed · {}", variable.quantity.label())
                } else {
                    variable.quantity.label().to_owned()
                })
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        });
    });
    let response = {
        let draft = &mut app.state.workbench.verification.tuning_variables[index];
        ui.add_sized(
            [ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h],
            egui::TextEdit::singleline(&mut draft.candidate_expression)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .id_source(("workbench.verify.tuning.variable", variable.id)),
        )
    };
    if app.state.workbench.verification.tuning_focus_variable == Some(variable.id) {
        response.request_focus();
        app.state.workbench.verification.tuning_focus_variable = None;
    }
    if response.clicked() || response.gained_focus() {
        app.state.workbench.verification.tuning_selected_variable = Some(variable.id);
    }
    if response.changed() {
        let expression = app.state.workbench.verification.tuning_variables[index]
            .candidate_expression
            .clone();
        let mut candidate = variable.clone();
        candidate.expression = expression;
        app.state.workbench.verification.tuning_variables[index].validation_error =
            candidate.validate().err();
    }
    let current_value = {
        let draft = &app.state.workbench.verification.tuning_variables[index];
        let mut candidate = variable.clone();
        candidate.expression.clone_from(&draft.candidate_expression);
        candidate.resolved_value_si().ok()
    };
    if let (Some((minimum, maximum)), Some(mut value)) = (tuning_range_si(variable), current_value)
        && ui
            .add_sized(
                [ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h],
                egui::Slider::new(&mut value, minimum..=maximum)
                    .show_value(false)
                    .smart_aim(false),
            )
            .changed()
    {
        let draft = &mut app.state.workbench.verification.tuning_variables[index];
        draft.candidate_expression = format!("{value:.12e}");
        let mut candidate = variable.clone();
        candidate.expression.clone_from(&draft.candidate_expression);
        draft.validation_error = candidate.validate().err();
    }
    let draft = &app.state.workbench.verification.tuning_variables[index];
    if let Some(error) = &draft.validation_error {
        let _ = crate::ui::widgets::problem_line(ui, Problem::Failure, error);
    } else {
        let range = variable.allowed_range.as_ref().map_or_else(
            || "No explicit tuning bounds".to_owned(),
            |range| format!("Allowed {} … {}", range.minimum, range.maximum),
        );
        let baseline = if draft.proposed {
            format!("Source literal {}", draft.baseline_expression)
        } else {
            format!("Committed {}", draft.baseline_expression)
        };
        ui.label(
            egui::RichText::new(format!("{baseline} · {range}"))
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    }
}

fn tuning_range_si(variable: &crate::state::DesignVariable) -> Option<(f64, f64)> {
    let range = variable.allowed_range.as_ref()?;
    let resolve = |expression: &str| {
        let mut candidate = variable.clone();
        candidate.expression = expression.to_owned();
        candidate.resolved_value_si().ok()
    };
    let minimum = resolve(&range.minimum)?;
    let maximum = resolve(&range.maximum)?;
    (minimum.is_finite() && maximum.is_finite() && minimum < maximum).then_some((minimum, maximum))
}

/// The baseline run's first waveform, dashed, under the candidate run's same
/// waveform once a committed candidate has run.
fn tuning_response_pane(ui: &mut Ui, app: &RSpiceApp) {
    crate::workbench::design_system::section_header(ui, "Response", None);
    let analysis = tuning_baseline_run(app)
        .and_then(|run| run.analyses.iter().find(|analysis| analysis.success));
    let baseline = analysis.and_then(|analysis| analysis.waveforms.first());
    let candidate = tuning_candidate_run(app)
        .and_then(|run| run.analyses.iter().find(|analysis| analysis.success))
        .and_then(|analysis| {
            baseline.and_then(|baseline| {
                analysis
                    .waveforms
                    .iter()
                    .find(|waveform| waveform.name.eq_ignore_ascii_case(&baseline.name))
            })
        });
    let sweep_unit = analysis.map_or("", |analysis| axes::sweep_unit(analysis.analysis_type));
    tuning_waveform_preview(ui, baseline, candidate, sweep_unit);
}

fn tuning_waveform_preview(
    ui: &mut Ui,
    baseline: Option<&crate::state::WaveformData>,
    candidate: Option<&crate::state::WaveformData>,
    sweep_unit: &str,
) {
    let t = Tokens::get(ui.ctx());
    let (pane, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
    let rect = pane.shrink(TABLE_EDGE_INSET);
    if rect.width() <= 1.0 || rect.height() <= 1.0 {
        return;
    }
    let valid_waveform = |waveform: &&crate::state::WaveformData| {
        waveform.x.len() == waveform.y.len()
            && waveform.x.len() > 1
            && waveform
                .x
                .iter()
                .chain(waveform.y.iter())
                .all(|value| value.is_finite())
    };
    let waveforms = [baseline, candidate]
        .into_iter()
        .flatten()
        .filter(valid_waveform)
        .collect::<Vec<_>>();
    if waveforms.is_empty() {
        ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Run the plan to see its response",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    }
    let x_min = waveforms
        .iter()
        .flat_map(|waveform| waveform.x.iter().copied())
        .fold(f64::INFINITY, f64::min);
    let x_max = waveforms
        .iter()
        .flat_map(|waveform| waveform.x.iter().copied())
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = waveforms
        .iter()
        .flat_map(|waveform| waveform.y.iter().copied())
        .fold(f64::INFINITY, f64::min);
    let y_max = waveforms
        .iter()
        .flat_map(|waveform| waveform.y.iter().copied())
        .fold(f64::NEG_INFINITY, f64::max);
    let named = baseline.or(candidate);
    let axes = axes::Axes {
        x: (x_min, x_max),
        y: (y_min, y_max),
        x_unit: sweep_unit,
        y_unit: named
            .and_then(|waveform| waveform.unit.as_deref())
            .unwrap_or(""),
    };

    // The header row carries the waveform's name and what each line is; the
    // axes are painted under it, so neither sits on the other.
    let header = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), PLOT_HEADER_H));
    ui.painter().text(
        header.left_center(),
        egui::Align2::LEFT_CENTER,
        named.map_or("Retained waveform", |waveform| waveform.name.as_str()),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let mut legend = vec![("baseline", egui::Stroke::new(1.3, t.color.text_dim), true)];
    if candidate.is_some() {
        legend.insert(
            0,
            (
                "candidate",
                egui::Stroke::new(1.5, t.color.traces[0]),
                false,
            ),
        );
    }
    axes::legend(ui, header, &legend);

    let plot = axes::paint(
        ui,
        egui::Rect::from_min_max(header.left_bottom(), rect.max),
        &axes,
    );
    if let Some(baseline) = baseline.filter(valid_waveform) {
        ui.painter().extend(egui::Shape::dashed_line(
            &axes::points(plot, &axes, &baseline.x, &baseline.y),
            egui::Stroke::new(1.3, t.color.text_dim),
            5.0,
            4.0,
        ));
    }
    if let Some(candidate) = candidate.filter(valid_waveform) {
        ui.painter().add(egui::Shape::line(
            axes::points(plot, &axes, &candidate.x, &candidate.y),
            egui::Stroke::new(1.5, t.color.traces[0]),
        ));
    }
}

/// The plan's limits against the baseline run and the committed candidate.
///
/// A candidate value exists only once a committed candidate has run; while
/// the sandbox holds uncommitted changes, no run has measured them, so the
/// candidate column is empty and the verdict is the baseline's.
fn tuning_specification_table(
    ui: &mut Ui,
    app: &RSpiceApp,
    specs: &[SpecEntry],
    provisional: bool,
) {
    if specs.is_empty() {
        return;
    }
    let baseline_run = tuning_baseline_run(app);
    let candidate_run = tuning_candidate_run(app).filter(|_| !provisional);
    let rows = specs
        .iter()
        .map(|spec| {
            (
                spec,
                baseline_run.and_then(|run| worst_measurement_in_run(run, spec)),
                candidate_run.and_then(|run| worst_measurement_in_run(run, spec)),
            )
        })
        .collect::<Vec<_>>();
    let columns = [
        Column::name("Output", 140.0),
        Column::words("Spec", 104.0),
        Column::number("Baseline", 104.0),
        Column::number("Candidate", 104.0),
        Column::words("Pass/fail", 70.0),
    ];
    let height = ui.available_height();
    let _ = DataTable::new("verify.tuning.specs", &columns, rows.len())
        .max_height(height)
        .show(ui, |index| {
            let (spec, baseline, candidate) = rows[index];
            let value = |value: Option<f64>| {
                value.map_or_else(Cell::absent, |value| {
                    Cell::number(format_value(value, &spec.unit))
                })
            };
            vec![
                Cell::text(&spec.measurement),
                limit_cell(spec),
                value(baseline),
                value(candidate),
                candidate
                    .or(baseline)
                    .map_or_else(Cell::empty, |value| Cell::verdict(judge(spec, value))),
            ]
        });
}
