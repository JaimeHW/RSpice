//! Canonical Simulation Studio page for plan-owned design variables.
//!
//! The registry is a projection of the active plan payload. Editing happens in
//! a runtime draft and commits through the same validated, revisioned owner as
//! execution; no table control mutates project state directly.

use super::*;

use egui::{ComboBox, Frame, RichText, TextEdit};
use egui_extras::{Column, TableBuilder};

use crate::product::DesignVariableId;
use crate::state::{
    DesignVariable, DesignVariableBoundsPolicy, DesignVariableOverridePolicy,
    DesignVariableQuantity, DesignVariableScope, DesignVariableSweepEligibility,
    DesignVariableUnitPolicy,
};
use crate::workbench::state::{ImportDesignVariablesDraft, RemoveDesignVariableDraft};

const REGISTRY_MIN_WIDTH: f32 = 1_130.0;
const CARD_HEADER_HEIGHT: f32 = 37.0;
const TABLE_HEADER_HEIGHT: f32 = 27.0;
const TABLE_ROW_HEIGHT: f32 = 29.0;
const LOWER_SPLIT_BREAKPOINT: f32 = 820.0;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ensure_selection(app);
    let t = Tokens::get(ui.ctx());
    Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt("workbench.simulate.design-variables")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width(ui.available_width());
                page_heading(ui, app);
                Frame::NONE.fill(t.color.border).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.spacing_mut().item_spacing.y = 1.0;
                    registry_card(ui, app);
                    lower_workspace(ui, app);
                });
            });
    });
}

fn page_heading(ui: &mut Ui, app: &mut RSpiceApp) {
    let count = active_variables(app).len();
    workspace_title_row(ui, |ui| {
        let mut back = false;
        let mut add = false;
        if ui.available_width() > TITLE_ACTION_STACK_BREAKPOINT {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let heading_width = (ui.available_width() - 214.0).max(260.0);
                ui.allocate_ui_with_layout(
                    vec2(heading_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        heading(
                            ui,
                            &format!("Parameterization · {count} variables"),
                            "Design variables",
                            "Typed, unit-aware parameters shared by schematic, analyses, sweeps and optimization.",
                        );
                    },
                );
                back = Button::new("Analyses").icon(Icon::ChevronLeft).show(ui).clicked();
                add = Button::new("Add variable")
                    .icon(Icon::Add)
                    .accent()
                    .show(ui)
                    .clicked();
            });
        } else {
            heading(
                ui,
                &format!("Parameterization · {count} variables"),
                "Design variables",
                "Typed, unit-aware parameters shared by schematic, analyses, sweeps and optimization.",
            );
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                back = Button::new("Analyses")
                    .icon(Icon::ChevronLeft)
                    .show(ui)
                    .clicked();
                add = Button::new("Add variable")
                    .icon(Icon::Add)
                    .accent()
                    .show(ui)
                    .clicked();
            });
        }
        if back {
            app.state.workbench.simulation_page = crate::workbench::state::SimulationPage::Analyses;
        }
        if add {
            open_add_dialog(app);
        }
    });
}

fn registry_card(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let variables = active_variables(app);
    let selected = app.state.workbench.simulation_variable_selection;
    let valid = variables
        .iter()
        .filter(|variable| variable.validate().is_ok())
        .count();
    let mut add = false;
    let mut duplicate = false;
    let mut remove = false;
    let mut import = false;
    let registry_status = format!("{valid} typed · dimensionally valid");

    table_card(ui, |ui| {
        card_header(
            ui,
            "Variable registry",
            Some((&registry_status, t.color.ok)),
            |ui| {
                import = Button::new("Import...").show(ui).clicked();
                remove = Button::new("Remove")
                    .enabled(selected.is_some())
                    .show(ui)
                    .clicked();
                duplicate = Button::new("Duplicate")
                    .enabled(selected.is_some())
                    .show(ui)
                    .clicked();
                add = Button::new("Add...").show(ui).clicked();
            },
        );

        let table_width = ui.available_width().max(REGISTRY_MIN_WIDTH);
        ScrollArea::horizontal()
            .id_salt("simulation-design-variable-registry-horizontal")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_min_width(table_width);
                let mut requested_selection = None;
                TableBuilder::new(ui)
                    .id_salt("simulation-design-variable-registry")
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .column(Column::initial(112.0).at_least(88.0))
                    .column(Column::initial(102.0).at_least(86.0))
                    .column(Column::remainder().at_least(142.0))
                    .column(Column::initial(124.0).at_least(102.0))
                    .column(Column::initial(142.0).at_least(116.0))
                    .column(Column::initial(132.0).at_least(102.0))
                    .column(Column::initial(122.0).at_least(96.0))
                    .column(Column::initial(142.0).at_least(116.0))
                    .column(Column::initial(74.0).at_least(68.0))
                    .header(TABLE_HEADER_HEIGHT, |mut header| {
                        for label in [
                            "NAME",
                            "TYPE",
                            "EXPRESSION",
                            "RESOLVED",
                            "BOUNDS",
                            "SCOPE",
                            "USED BY",
                            "SWEEP ROLE",
                            "STATUS",
                        ] {
                            header.col(|ui| table_header_cell(ui, label));
                        }
                    })
                    .body(|mut body| {
                        let mut focus_responses = Vec::new();
                        if variables.is_empty() {
                            body.row(TABLE_ROW_HEIGHT, |mut row| {
                                row.col(|ui| {
                                    ui.label("None");
                                });
                                for _ in 0..8 {
                                    row.col(|ui| {
                                        ui.label("");
                                    });
                                }
                            });
                        } else {
                            for variable in &variables {
                                let selected_row = selected == Some(variable.id);
                                let resolved = resolved_variable_display(variable).unwrap_or_else(
                                    |_| "unresolved".to_owned(),
                                );
                                let bounds = variable.allowed_range.as_ref().map_or_else(
                                    || "unbounded".to_owned(),
                                    |range| format!("{} ... {}", range.minimum, range.maximum),
                                );
                                let consumers = variable_impacts(app, variable).len();
                                let status = match variable.resolved_value() {
                                    Ok(crate::state::ResolvedDesignVariableValue::Numeric(
                                        resolution,
                                    )) if resolution.bounds_adjustment.is_some() => "clamped",
                                    Ok(crate::state::ResolvedDesignVariableValue::Numeric(
                                        resolution,
                                    )) if resolution.unit_coercion_applied => "coerced",
                                    Ok(_) => "valid",
                                    Err(_) => "blocked",
                                };
                                body.row(TABLE_ROW_HEIGHT, |mut row| {
                                    row.set_selected(selected_row);
                                    row.col(|ui| {
                                        ui.label(RichText::new(&variable.name).monospace());
                                    });
                                    row.col(|ui| {
                                        ui.label(variable.quantity.label());
                                    });
                                    row.col(|ui| {
                                        ui.label(RichText::new(&variable.expression).monospace());
                                    });
                                    row.col(|ui| {
                                        ui.label(RichText::new(&resolved).monospace());
                                    });
                                    row.col(|ui| {
                                        ui.label(RichText::new(&bounds).monospace());
                                    });
                                    row.col(|ui| {
                                        ui.label(variable.scope.label());
                                    });
                                    row.col(|ui| {
                                        ui.label(format!(
                                            "{consumers} {}",
                                            if consumers == 1 {
                                                "consumer"
                                            } else {
                                                "consumers"
                                            }
                                        ));
                                    });
                                    row.col(|ui| {
                                        ui.label(variable.sweep_eligibility.label());
                                    });
                                    row.col(|ui| {
                                        let color = match status {
                                            "valid" => Tokens::get(ui.ctx()).color.ok,
                                            "blocked" => Tokens::get(ui.ctx()).color.err,
                                            _ => Tokens::get(ui.ctx()).color.warn,
                                        };
                                        ui.label(RichText::new(status).color(color));
                                    });
                                    let response = row.response().interact(Sense::click());
                                    response.widget_info(|| {
                                        egui::WidgetInfo::selected(
                                            egui::WidgetType::SelectableLabel,
                                            true,
                                            selected_row,
                                            format!("Select design variable {}", variable.name),
                                        )
                                    });
                                    focus_responses.push(response.clone());
                                    let keyboard_activation = response.has_focus()
                                        && response.ctx.input(|input| {
                                            input.key_pressed(egui::Key::Enter)
                                                || input.key_pressed(egui::Key::Space)
                                        });
                                    if response.clicked() || keyboard_activation {
                                        requested_selection = Some(variable.id);
                                    }
                                });
                            }
                        }
                        for response in focus_responses {
                            let rect = response.rect;
                            theme::paint_focus_ring(body.ui_mut(), &response, rect);
                        }
                    });
                if variables.is_empty() {
                    ui.add_space(7.0);
                    empty_hint(
                        ui,
                        "No design variables are configured. Add or import a typed variable to begin.",
                    );
                    ui.add_space(7.0);
                }
                if let Some(id) = requested_selection {
                    select_variable(app, id);
                }
            });
    });

    if add {
        open_add_dialog(app);
    } else if duplicate {
        duplicate_selected(app);
    } else if remove {
        open_remove_dialog(app);
    } else if import {
        app.state.workbench.simulation_workflow = Some(
            SimulationWorkflowDialog::ImportDesignVariables(ImportDesignVariablesDraft::default()),
        );
    }
}

fn lower_workspace(ui: &mut Ui, app: &mut RSpiceApp) {
    let available = ui.available_width();
    if available >= LOWER_SPLIT_BREAKPOINT {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            let left = ((available - 1.0) * 0.5).floor();
            ui.allocate_ui_with_layout(vec2(left, 0.0), Layout::top_down(Align::Min), |ui| {
                editor_card(ui, app);
            });
            ui.allocate_ui_with_layout(
                vec2(available - left - 1.0, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    impact_card(ui, app);
                },
            );
        });
    } else {
        editor_card(ui, app);
        impact_card(ui, app);
    }
}

fn editor_card(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected_name = selected_variable(app).map(|v| v.name.clone());
    table_card(ui, |ui| {
        let title = selected_name.as_deref().map_or_else(
            || "Selected variable".to_owned(),
            |name| format!("Selected variable · {name}"),
        );
        card_header(
            ui,
            &title,
            selected_name
                .as_ref()
                .map(|_| ("plan-owned", Tokens::get(ui.ctx()).color.ok)),
            |_| {},
        );
        ui.add_space(10.0);
        let Some(mut draft) = app.state.workbench.simulation_variable_editor.clone() else {
            empty_hint(
                ui,
                "Select a variable in the registry to inspect or edit it.",
            );
            ui.add_space(10.0);
            return;
        };

        let (mut minimum, mut maximum) = split_range(&draft.allowed_range);
        egui::Grid::new("simulation-design-variable-editor-fields")
            .num_columns(2)
            .spacing(vec2(10.0, 7.0))
            .show(ui, |ui| {
                field_text(ui, "Name", &mut draft.name, true);
                field_combo(
                    ui,
                    "Quantity type",
                    "variable-editor-quantity",
                    &mut draft.quantity,
                    &DesignVariableQuantity::ALL.map(DesignVariableQuantity::label),
                );
                ui.end_row();
                field_text(ui, "Expression", &mut draft.expression, true);
                field_combo(
                    ui,
                    "Unit policy",
                    "variable-editor-unit-policy",
                    &mut draft.unit_policy,
                    &DesignVariableUnitPolicy::ALL.map(DesignVariableUnitPolicy::label),
                );
                ui.end_row();
                field_text(ui, "Minimum", &mut minimum, true);
                field_text(ui, "Maximum", &mut maximum, true);
                ui.end_row();
                field_combo(
                    ui,
                    "Scope",
                    "variable-editor-scope",
                    &mut draft.scope,
                    &[
                        "Lab characterization - testbench",
                        "Project",
                        "Selected cell",
                        "Selected analysis only",
                        "Run set",
                    ],
                );
                field_combo(
                    ui,
                    "Sweep role",
                    "variable-editor-sweep",
                    &mut draft.sweep_eligibility,
                    &DesignVariableSweepEligibility::ALL.map(DesignVariableSweepEligibility::label),
                );
                ui.end_row();
                field_combo(
                    ui,
                    "Override policy",
                    "variable-editor-override",
                    &mut draft.override_policy,
                    &DesignVariableOverridePolicy::ALL.map(DesignVariableOverridePolicy::label),
                );
                field_combo(
                    ui,
                    "Out-of-bounds",
                    "variable-editor-bounds-policy",
                    &mut draft.bounds_policy,
                    &DesignVariableBoundsPolicy::ALL.map(DesignVariableBoundsPolicy::label),
                );
                ui.end_row();
            });
        draft.allowed_range = join_range(&minimum, &maximum);
        field_text(ui, "Description", &mut draft.description, false);
        let validation = validate_design_variable_draft(app, &draft).err();
        let dirty = design_variable_draft_is_dirty(app, &draft);
        ui.add_space(8.0);

        let candidate = design_variable_from_draft(app, &draft);
        let resolved = candidate
            .as_ref()
            .map_err(Clone::clone)
            .and_then(resolved_variable_display)
            .unwrap_or_else(|_| "unresolved".to_owned());
        property_row(ui, "Resolved now", &resolved);
        let dimensional_check = candidate
            .as_ref()
            .ok()
            .and_then(|variable| variable.resolved_value().ok())
            .map_or("blocked", |resolution| match resolution {
                crate::state::ResolvedDesignVariableValue::Numeric(resolution)
                    if resolution.unit_coercion_applied =>
                {
                    "coerced · conversion recorded"
                }
                _ => "consistent",
            });
        property_row(ui, "Dimensional check", dimensional_check);
        if let Some(variable) = selected_variable(app) {
            property_row(
                ui,
                "Defined at",
                &format!("revision {}", variable.revision.get()),
            );
        }
        ui.add_space(6.0);
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 36.0),
            Layout::top_down(Align::Min),
            |ui| {
                if let Some(error) = &validation {
                    ui.label(RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
                } else if !dirty {
                    ui.label(
                        RichText::new("No unsaved variable changes.")
                            .color(Tokens::get(ui.ctx()).color.text_dim),
                    );
                }
            },
        );
        ui.horizontal(|ui| {
            let apply = Button::new("Apply changes")
                .accent()
                .enabled(validation.is_none() && draft.variable_id.is_some() && dirty)
                .show(ui)
                .clicked();
            let revert = Button::new("Revert").enabled(dirty).show(ui).clicked();
            if apply {
                match commit_design_variable(app, &draft) {
                    Ok(message) => {
                        app.state.workbench.analysis_lifecycle_status = message;
                        if let Some(id) = draft.variable_id {
                            select_variable(app, id);
                        }
                    }
                    Err(error) => {
                        draft.validation_error = Some(error.clone());
                        app.state.workbench.simulation_variable_editor = Some(draft.clone());
                        record_failure(app, "Design variable update", &error);
                    }
                }
            } else if revert {
                if let Some(id) = draft.variable_id {
                    select_variable(app, id);
                }
            } else {
                app.state.workbench.simulation_variable_editor = Some(draft);
            }
        });
        ui.add_space(10.0);
    });
}

fn impact_card(ui: &mut Ui, app: &RSpiceApp) {
    table_card(ui, |ui| {
        let impacts = selected_variable(app)
            .map(|variable| variable_impacts(app, variable))
            .unwrap_or_default();
        card_header(
            ui,
            "Consumers and change impact",
            selected_variable(app).map(|_| {
                if impacts.is_empty() {
                    ("no direct consumers", Tokens::get(ui.ctx()).color.text_dim)
                } else {
                    ("rerun required", Tokens::get(ui.ctx()).color.warn)
                }
            }),
            |_| {},
        );
        ui.add_space(10.0);
        let Some(variable) = selected_variable(app) else {
            empty_hint(
                ui,
                "Select a variable to review its exact consumers and retained-result impact.",
            );
            ui.add_space(10.0);
            return;
        };
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(&variable.name).color(Tokens::get(ui.ctx()).color.accent));
            ui.label("->");
            if impacts.is_empty() {
                ui.label("no direct consumers");
            } else {
                for (consumer, _, _) in impacts.iter().take(3) {
                    ui.label(consumer);
                }
            }
        });
        ui.add_space(8.0);
        impact_header(ui);
        for (consumer, kind, effect) in &impacts {
            impact_row(ui, consumer, kind, effect);
        }
        let plan_id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .map(|plan| plan.id());
        let retained_runs = app.state.simulation.runs.iter().filter(|run| {
            run.prepared_receipt()
                .and_then(|receipt| receipt.simulation_plan_id())
                .is_some_and(|source_plan_id| Some(source_plan_id) == plan_id)
        });
        let mut retained_count = 0;
        for run in retained_runs {
            retained_count += 1;
            impact_row(
                ui,
                &format!("Run {}", run.id),
                "same-plan immutable result",
                "retained · historical",
            );
        }
        if retained_count == 0 {
            impact_row(ui, "Prior results", "immutable result", "none retained");
        }
        ui.add_space(8.0);
        ui.label(RichText::new("Editing invalidates dependent future runs. Prior immutable datasets are never rewritten.").color(Tokens::get(ui.ctx()).color.text_dim));
        ui.add_space(10.0);
    });
}

fn active_variables(app: &RSpiceApp) -> Vec<DesignVariable> {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| app.state.workspace.active_plan_data(plan.id()))
        .map(|payload| payload.design_variables.clone())
        .unwrap_or_default()
}

fn selected_variable(app: &RSpiceApp) -> Option<&DesignVariable> {
    let id = app.state.workbench.simulation_variable_selection?;
    let plan_id = app.state.sim_setup.stable_analysis_plan().ok()?.id();
    app.state
        .workspace
        .active_plan_data(plan_id)?
        .design_variables
        .iter()
        .find(|variable| variable.id == id)
}

fn ensure_selection(app: &mut RSpiceApp) {
    let variables = active_variables(app);
    let selected = app.state.workbench.simulation_variable_selection;
    let selected_exists =
        selected.is_some_and(|id| variables.iter().any(|variable| variable.id == id));
    if !selected_exists {
        app.state.workbench.simulation_variable_selection =
            variables.first().map(|variable| variable.id);
    }
    let selected = app.state.workbench.simulation_variable_selection;
    let editor_matches = app
        .state
        .workbench
        .simulation_variable_editor
        .as_ref()
        .and_then(|draft| draft.variable_id)
        == selected;
    if !editor_matches {
        app.state.workbench.simulation_variable_editor = selected.and_then(|id| {
            variables
                .iter()
                .find(|variable| variable.id == id)
                .map(DesignVariableDraft::from_variable)
        });
    }
}

fn select_variable(app: &mut RSpiceApp, id: DesignVariableId) {
    app.state.workbench.simulation_variable_selection = Some(id);
    app.state.workbench.simulation_variable_editor =
        selected_variable(app).map(DesignVariableDraft::from_variable);
}

fn open_add_dialog(app: &mut RSpiceApp) {
    app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::DesignVariable(
        DesignVariableDraft::default(),
    ));
}

fn duplicate_selected(app: &mut RSpiceApp) {
    let Some(variable) = selected_variable(app).cloned() else {
        return;
    };
    let Some(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
    else {
        return;
    };
    let name = unique_copy_name(&variable.name, &active_variables(app));
    let mut workspace = app.state.workspace.clone();
    match workspace.duplicate_design_variable(plan_id, variable.id, name) {
        Ok(id) => {
            let mut setup = app.state.sim_setup.clone();
            match setup.commit_active_plan_configuration_change(format!(
                "Duplicated design variable {}.",
                variable.name
            )) {
                Ok(receipt) => {
                    app.state.workspace = workspace;
                    app.state.sim_setup = setup;
                    app.invalidate_simulation_preflight();
                    select_variable(app, id);
                    app.state.workbench.analysis_lifecycle_status = receipt.status_line();
                }
                Err(error) => record_failure(app, "Design variable duplicate", &error.to_string()),
            }
        }
        Err(error) => record_failure(app, "Design variable duplicate", &error.to_string()),
    }
}

fn open_remove_dialog(app: &mut RSpiceApp) {
    let Some(variable) = selected_variable(app) else {
        return;
    };
    app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::RemoveDesignVariable(
        RemoveDesignVariableDraft {
            variable_id: variable.id,
            variable_name: variable.name.clone(),
        },
    ));
}

fn unique_copy_name(base: &str, variables: &[DesignVariable]) -> String {
    for suffix in 1..=10_000 {
        let candidate = if suffix == 1 {
            format!("{base}_copy")
        } else {
            format!("{base}_copy_{suffix}")
        };
        if !variables
            .iter()
            .any(|variable| variable.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    format!("{base}_copy_{}", DesignVariableId::new())
}

fn variable_impacts(
    app: &RSpiceApp,
    variable: &DesignVariable,
) -> Vec<(String, &'static str, &'static str)> {
    let mut impacts = Vec::new();
    for component in &app.state.schematic.components {
        if contains_identifier(&component.value, &variable.name)
            || contains_identifier(&component.params, &variable.name)
        {
            impacts.push((component.name.clone(), "schematic instance", "re-netlist"));
        }
    }
    if let Ok(plan) = app.state.sim_setup.stable_analysis_plan() {
        if let DesignVariableScope::SelectedAnalysis { analysis_id } = variable.scope
            && let Some(instance) = plan
                .instances()
                .iter()
                .find(|instance| instance.id() == analysis_id)
        {
            impacts.push((
                instance.kind().code().to_owned(),
                "analysis-scoped owner",
                "recompute",
            ));
        }
        if let Some(payload) = app.state.workspace.active_plan_data(plan.id()) {
            impacts.extend(
                payload
                    .saved_outputs
                    .iter()
                    .filter(|output| contains_identifier(&output.source_expression, &variable.name))
                    .map(|output| (output.name.clone(), "saved output", "recompute")),
            );
            impacts.extend(
                payload
                    .specs
                    .iter()
                    .filter(|spec| contains_identifier(&spec.expression, &variable.name))
                    .map(|spec| (spec.measurement.clone(), "specification", "re-evaluate")),
            );
        }
    }
    impacts
}

fn contains_identifier(text: &str, identifier: &str) -> bool {
    if text.is_empty() || identifier.is_empty() {
        return false;
    }
    let normalized_text = text.to_ascii_lowercase();
    let normalized_identifier = identifier.to_ascii_lowercase();
    normalized_text
        .match_indices(&normalized_identifier)
        .any(|(index, matched)| {
            let before = normalized_text[..index].chars().next_back();
            let after = normalized_text[index + matched.len()..].chars().next();
            before.is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_')
                && after
                    .is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_')
        })
}

fn resolved_variable_display(variable: &DesignVariable) -> Result<String, String> {
    match variable.resolved_value()? {
        crate::state::ResolvedDesignVariableValue::Numeric(resolution) => {
            let value = if variable.quantity == DesignVariableQuantity::Integer {
                format!("{:.0}", resolution.effective_value_si)
            } else {
                crate::quantity::format_engineering_value(resolution.effective_value_si)
            };
            let unit = match variable.quantity {
                DesignVariableQuantity::Resistance => " ohm",
                DesignVariableQuantity::Capacitance => " F",
                DesignVariableQuantity::Voltage => " V",
                DesignVariableQuantity::Current => " A",
                DesignVariableQuantity::Temperature => " K",
                DesignVariableQuantity::Time => " s",
                DesignVariableQuantity::Frequency => " Hz",
                DesignVariableQuantity::Dimensionless | DesignVariableQuantity::Integer => "",
                DesignVariableQuantity::Enumeration => {
                    unreachable!("enumeration variables resolve through the non-numeric branch")
                }
            };
            Ok(format!("{value}{unit}"))
        }
        crate::state::ResolvedDesignVariableValue::Enumeration(member) => Ok(member),
    }
}

fn table_card(ui: &mut Ui, add: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_width(ui.available_width());
        add(ui);
    });
}

fn card_header(
    ui: &mut Ui,
    title: &str,
    meta: Option<(&str, egui::Color32)>,
    actions: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), CARD_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.scope_builder(
        egui::UiBuilder::new().max_rect(rect.shrink2(vec2(10.0, 0.0))),
        |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(
                    RichText::new(title).font(theme::sans(tokens::FS_1, FontWeight::SemiBold)),
                );
                if let Some((meta, tone)) = meta {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(meta)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(tone),
                    );
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| actions(ui));
            });
        },
    );
}

fn table_header_cell(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, t.color.bg_panel_2);
    ui.label(
        RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
}

fn empty_hint(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).color(Tokens::get(ui.ctx()).color.text_dim));
}

fn field_text(ui: &mut Ui, label: &str, value: &mut String, mono: bool) {
    ui.vertical(|ui| {
        ui.label(RichText::new(label).color(Tokens::get(ui.ctx()).color.text_dim));
        let edit = TextEdit::singleline(value).desired_width(f32::INFINITY);
        ui.add(if mono {
            edit.font(egui::TextStyle::Monospace)
        } else {
            edit
        });
    });
}

fn field_combo(ui: &mut Ui, label: &str, id: &'static str, selected: &mut usize, options: &[&str]) {
    ui.vertical(|ui| {
        ui.label(RichText::new(label).color(Tokens::get(ui.ctx()).color.text_dim));
        let current = options
            .get(*selected)
            .copied()
            .unwrap_or("Invalid selection");
        ComboBox::from_id_salt(id)
            .selected_text(current)
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                for (index, option) in options.iter().enumerate() {
                    ui.selectable_value(selected, index, *option);
                }
            });
    });
}

fn split_range(range: &str) -> (String, String) {
    range
        .split_once('…')
        .or_else(|| range.split_once("..."))
        .or_else(|| range.split_once("..="))
        .or_else(|| range.split_once(".."))
        .map_or_else(
            || (String::new(), String::new()),
            |(minimum, maximum)| (minimum.trim().to_owned(), maximum.trim().to_owned()),
        )
}

fn impact_header(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), TABLE_HEADER_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.scope_builder(
        egui::UiBuilder::new().max_rect(rect.shrink2(vec2(8.0, 0.0))),
        |ui| {
            ui.columns(3, |columns| {
                for (column, label) in
                    columns
                        .iter_mut()
                        .zip(["CONSUMER", "KIND", "EFFECT OF CHANGE"])
                {
                    column.label(
                        RichText::new(label)
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text_dim),
                    );
                }
            });
        },
    );
}

fn join_range(minimum: &str, maximum: &str) -> String {
    if minimum.trim().is_empty() && maximum.trim().is_empty() {
        String::new()
    } else {
        format!("{} ... {}", minimum.trim(), maximum.trim())
    }
}

fn impact_row(ui: &mut Ui, consumer: &str, kind: &str, effect: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(vec2(ui.available_width(), TABLE_ROW_HEIGHT), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.scope_builder(
        egui::UiBuilder::new().max_rect(rect.shrink2(vec2(8.0, 0.0))),
        |ui| {
            ui.columns(3, |columns| {
                columns[0].label(RichText::new(consumer).monospace());
                columns[1].label(kind);
                columns[2].label(
                    RichText::new(effect).color(if effect.starts_with("retained") {
                        t.color.ok
                    } else {
                        t.color.warn
                    }),
                );
            });
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_names_are_case_insensitively_unique() {
        let first = DesignVariable::new(
            "RLOAD_copy",
            "10 ohm",
            DesignVariableQuantity::Resistance,
            DesignVariableScope::Project,
            "",
            None,
            DesignVariableSweepEligibility::FixedParameter,
            DesignVariableOverridePolicy::InheritOwnerOnly,
        )
        .unwrap();
        assert_eq!(unique_copy_name("RLOAD", &[first]), "RLOAD_copy_2");
    }

    #[test]
    fn range_editor_round_trips_empty_and_bounded_values() {
        assert_eq!(
            split_range("1 ohm ... 10 ohm"),
            ("1 ohm".to_owned(), "10 ohm".to_owned())
        );
        assert_eq!(join_range("", ""), "");
        assert_eq!(join_range("1 ohm", "10 ohm"), "1 ohm ... 10 ohm");
        assert_eq!(
            split_range("1 ohm … 10 ohm"),
            ("1 ohm".to_owned(), "10 ohm".to_owned())
        );
    }

    #[test]
    fn identifier_matching_rejects_substrings_and_accepts_expression_tokens() {
        assert!(contains_identifier("2 * RLOAD + 1", "RLOAD"));
        assert!(contains_identifier("2 * rload + 1", "RLOAD"));
        assert!(contains_identifier("{RLOAD}", "RLOAD"));
        assert!(!contains_identifier("RLOAD_A", "RLOAD"));
        assert!(!contains_identifier("MY_RLOAD", "RLOAD"));
    }
}
