//! Interactive tuning.
//!
//! Live parameter adjustment against the last run, with the dirty tracking
//! that decides when the displayed result no longer matches the tuned
//! values.

use super::*;

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
                .active_plan_data(plan_id)
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
    session.action_receipt =
        "The tuning sandbox was restored to the committed active-plan values.".to_owned();
}

fn tuning_session_variables(app: &RSpiceApp) -> Vec<crate::state::DesignVariable> {
    let session = &app.state.workbench.verification;
    let mut variables = session
        .tuning_plan_id
        .and_then(|plan_id| app.state.workspace.active_plan_data(plan_id))
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
            .update_design_variable_expressions(plan_id, &changes)
            .map_err(|error| error.to_string())?;
    }
    if let Some(variable) = proposed_variable {
        workspace
            .add_design_variable(plan_id, variable)
            .map_err(|error| error.to_string())?;
    }
    let binding_changed = if let Some(binding) = binding.as_ref() {
        if app.state.workspace.active_schematic_reference() != binding.source_view {
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
                "{} no longer matches the exact Value row that opened the sandbox",
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
            let changed = schematic.with_undo(description, move |schematic| {
                let index = schematic
                    .components
                    .iter()
                    .position(|component| component.id == component_id)
                    .expect("staged component identity was validated before the transaction");
                schematic.components[index].value = binding_expression;
                schematic.is_dirty = true;
                schematic.bump_topology_version();
            });
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
            "the tuned revision was not committed because exact run preparation failed: {}",
            error.message()
        ));
    }
    app.state.request_run_set_simulation();
    sync_tuning_session(app);
    let candidate_count = variable_change_count + binding_change_count;
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt(format!(
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
                TableCell::mono(&variable.name),
                TableCell::mono(committed),
                TableCell::mono(&draft.candidate_expression),
                TableCell::mono(delta),
                TableCell::text(destination),
            ])
        })
        .collect::<Vec<_>>();
    if let Some(binding) = instance_binding
        .as_ref()
        .filter(|binding| binding.requires_schematic_edit())
    {
        rows.push(vec![
            TableCell::mono(format!("{}.Value", binding.component_name)),
            TableCell::mono(&binding.source_value),
            TableCell::mono(&binding.binding_expression),
            TableCell::mono("instance binding"),
            TableCell::text(binding.source_view.display_path()),
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
            ui.label(
                egui::RichText::new("TUNING · REVIEWABLE WORKING CHANGE")
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.accent),
            );
            ui.label(
                egui::RichText::new(
                    "Review the exact active-plan expressions before committing one transactional revision.",
                )
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_dim),
            );
            ui.add_space(8.0);
            egui::Frame::new()
                .fill(t.color.bg_panel_2)
                .stroke(egui::Stroke::new(1.0, t.color.warn))
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    status_dot(ui, t.color.warn, "Existing retained evidence stays immutable");
                    ui.add(
                        egui::Label::new(
                            "The commit changes design inputs. Existing checks and results remain preserved and become stale until the required production run completes.",
                        )
                        .wrap(),
                    );
                });
            ui.add_space(8.0);
            let headers = vec![
                ("Parameter".to_owned(), 0.16),
                ("Committed".to_owned(), 0.22),
                ("Proposed".to_owned(), 0.22),
                ("Delta".to_owned(), 0.18),
                ("Destination".to_owned(), 0.22),
            ];
            render_data_table(ui, "verify-tuning-review", &headers, &rows, None);
            ui.add_space(8.0);
            property_row(ui, "New simulation", "required · queued through active plan");
            property_row(ui, "Release evidence", "never copied from sandbox");
            property_row(ui, "Change set", "one transactional working revision");
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
        record_verification_action(
            app,
            result,
            "Tuned variables were committed as one plan revision and dispatched through the production run pipeline.",
        );
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
    let Some((_plan_id, plan_revision, _committed_variables, specs)) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| (plan.id(), plan.revision()))
        .and_then(|(plan_id, revision)| {
            app.state
                .workspace
                .active_plan_data(plan_id)
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
        card(ui, "Exploration variables", |ui| {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.warn,
                "No active simulation plan payload",
            );
            ui.label("Create or activate a simulation plan before opening the parameter tuner.");
        });
        return;
    };
    let variables = tuning_session_variables(app);
    let t = Tokens::get(ui.ctx());
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
    let baseline_run = tuning_baseline_run(app);
    let candidate_run = tuning_candidate_run(app);
    verification_kpi_strip(
        ui,
        &[
            (
                "Design variables".to_owned(),
                variables.len().to_string(),
                format!("active plan · revision {}", plan_revision.get()),
                if variables.is_empty() {
                    t.color.warn
                } else {
                    t.color.text
                },
            ),
            (
                "Provisional changes".to_owned(),
                dirty_count.to_string(),
                if dirty_count == 0 {
                    "committed baseline".to_owned()
                } else {
                    "sandbox only · not committed".to_owned()
                },
                if dirty_count == 0 {
                    t.color.ok
                } else {
                    t.color.warn
                },
            ),
            (
                "Validation".to_owned(),
                if invalid_count == 0 {
                    "Ready"
                } else {
                    "Blocked"
                }
                .to_owned(),
                if invalid_count == 0 {
                    "typed values and bounds valid".to_owned()
                } else {
                    format!("{invalid_count} invalid candidate value(s)")
                },
                if invalid_count == 0 {
                    t.color.ok
                } else {
                    t.color.err
                },
            ),
            (
                "Retained baseline".to_owned(),
                baseline_run.map_or_else(|| "None".to_owned(), |run| format!("Run {}", run.id)),
                if candidate_run.is_some() {
                    "candidate result retained".to_owned()
                } else if baseline_run.is_some() {
                    "immutable comparison source".to_owned()
                } else {
                    "run required for this plan".to_owned()
                },
                if candidate_run.is_some() {
                    t.color.ok
                } else {
                    t.color.warn
                },
            ),
        ],
    );

    let width = ui.available_width();
    if width > VERIFY_KPI_BREAKPOINT {
        let left_width = ((width - 1.0) * 0.44).max(360.0);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui_with_layout(
                egui::vec2(left_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| tuning_variable_card(ui, app, &variables),
            );
            ui.allocate_ui_with_layout(
                egui::vec2((width - left_width - 1.0).max(1.0), 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| tuning_baseline_card(ui, app),
            );
        });
    } else {
        tuning_variable_card(ui, app, &variables);
        ui.add_space(1.0);
        tuning_baseline_card(ui, app);
    }
    tuning_specification_contract(ui, app, &specs, dirty_count > 0);
    action_receipt(ui, app);
}

fn tuning_variable_card(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    variables: &[crate::state::DesignVariable],
) {
    let dirty = tuning_is_dirty(app);
    card(ui, "Exploration variables", |ui| {
        status_dot(
            ui,
            if dirty {
                Tokens::get(ui.ctx()).color.warn
            } else {
                Tokens::get(ui.ctx()).color.ok
            },
            if dirty {
                "Provisional · not committed"
            } else {
                "Committed baseline"
            },
        );
        if variables.is_empty() {
            ui.label("No design variables are configured. Add typed variables in Simulation Studio before tuning.");
            return;
        }
        for variable in variables {
            tuning_variable_editor(ui, app, variable);
        }
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
    let selected = app.state.workbench.verification.tuning_selected_variable == Some(variable.id);
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(&variable.name)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(if selected {
                    t.color.accent
                } else {
                    t.color.text
                }),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(if proposed {
                    format!("PROPOSED / {}", variable.quantity.label())
                } else {
                    variable.quantity.label().to_owned()
                })
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if proposed {
                    t.color.warn
                } else {
                    t.color.text_dim
                }),
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
        ui.label(
            egui::RichText::new(error)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.err),
        );
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

fn tuning_baseline_card(ui: &mut Ui, app: &RSpiceApp) {
    card(ui, "Retained nominal response", |ui| {
        let run = tuning_baseline_run(app);
        let baseline = run
            .and_then(|run| run.analyses.iter().find(|analysis| analysis.success))
            .and_then(|analysis| analysis.waveforms.first());
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
        tuning_waveform_preview(ui, baseline, candidate);
        if let Some(run) = run {
            property_row(ui, "Dataset", &run.dataset_id.to_string());
            property_row(ui, "Run", &format!("Run {} · immutable", run.id));
        } else {
            ui.label("No source-aligned retained waveform is available. Committing a valid candidate dispatches the active production plan and retains its result as a new immutable run.");
        }
    });
}

fn tuning_waveform_preview(
    ui: &mut Ui,
    baseline: Option<&crate::state::WaveformData>,
    candidate: Option<&crate::state::WaveformData>,
) {
    let t = Tokens::get(ui.ctx());
    let height = 210.0;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    for index in 1..4 {
        let x = egui::lerp(rect.x_range(), index as f32 / 4.0);
        let y = egui::lerp(rect.y_range(), index as f32 / 4.0);
        ui.painter().vline(
            x,
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.canvas_grid),
        );
        ui.painter().hline(
            rect.x_range(),
            y,
            egui::Stroke::new(1.0, t.color.canvas_grid),
        );
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
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "No retained nominal waveform",
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
    let x_span = (x_max - x_min).max(f64::EPSILON);
    let y_span = (y_max - y_min).max(f64::EPSILON);
    let points_for = |waveform: &crate::state::WaveformData| {
        waveform
            .x
            .iter()
            .zip(waveform.y.iter())
            .map(|(x, y)| {
                egui::pos2(
                    rect.left() + (((x - x_min) / x_span) as f32) * rect.width(),
                    rect.bottom() - (((y - y_min) / y_span) as f32) * rect.height(),
                )
            })
            .collect::<Vec<_>>()
    };
    if let Some(baseline) = baseline.filter(valid_waveform) {
        ui.painter().extend(egui::Shape::dashed_line(
            &points_for(baseline),
            egui::Stroke::new(1.3, t.color.text_dim),
            5.0,
            4.0,
        ));
    }
    if let Some(candidate) = candidate.filter(valid_waveform) {
        ui.painter().add(egui::Shape::line(
            points_for(candidate),
            egui::Stroke::new(1.7, t.color.accent),
        ));
    }
    ui.painter().text(
        rect.left_top() + egui::vec2(8.0, 7.0),
        egui::Align2::LEFT_TOP,
        baseline
            .or(candidate)
            .map_or("Retained waveform", |waveform| waveform.name.as_str()),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        rect.right_top() + egui::vec2(-8.0, 7.0),
        egui::Align2::RIGHT_TOP,
        if candidate.is_some() {
            "solid candidate · dashed baseline"
        } else {
            "dashed immutable baseline"
        },
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

fn tuning_specification_contract(
    ui: &mut Ui,
    app: &RSpiceApp,
    specs: &[SpecEntry],
    provisional: bool,
) {
    table_section_header(
        ui,
        "Provisional measurement contract",
        Some(if provisional {
            "sandbox values · excluded from release evidence"
        } else {
            "committed retained evidence"
        }),
        None,
    );
    let t = Tokens::get(ui.ctx());
    let baseline_run = tuning_baseline_run(app);
    let candidate_run = tuning_candidate_run(app);
    let headers = vec![
        ("Executable check".to_owned(), 0.25),
        ("Committed baseline".to_owned(), 0.22),
        ("Provisional".to_owned(), 0.22),
        ("Limit".to_owned(), 0.16),
        ("Gate".to_owned(), 0.15),
    ];
    let rows = specs
        .iter()
        .map(|spec| {
            let baseline_value =
                baseline_run.and_then(|run| measurement_in_run(run, &spec.measurement));
            let candidate_value =
                candidate_run.and_then(|run| measurement_in_run(run, &spec.measurement));
            let baseline = baseline_value
                .map_or_else(|| "—".to_owned(), |value| format_value(value, &spec.unit));
            let passed = candidate_value.is_some_and(|value| spec.passes(value));
            vec![
                TableCell::text(&spec.measurement),
                TableCell::mono(&baseline),
                TableCell::mono(if provisional {
                    "awaiting commit + run".to_owned()
                } else if let Some(value) = candidate_value {
                    format_value(value, &spec.unit)
                } else {
                    "awaiting retained candidate".to_owned()
                }),
                TableCell::mono(limit_text(spec)),
                TableCell::tone(
                    if provisional || candidate_value.is_none() {
                        "NOT EVALUATED"
                    } else if passed {
                        "PASS"
                    } else {
                        "FAIL"
                    },
                    if provisional || candidate_value.is_none() {
                        t.color.warn
                    } else if passed {
                        t.color.ok
                    } else {
                        t.color.err
                    },
                ),
            ]
        })
        .collect::<Vec<_>>();
    render_data_table(ui, "verify-tuning-contract", &headers, &rows, None);
}
