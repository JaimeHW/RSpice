//! Stable, ordered simulation-plan editor and fail-closed preflight.

mod analysis_form;

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{
    AnalysisDependency, AnalysisDraft, AnalysisKind, AnalysisLifecycleCommand,
    AnalysisLifecycleReceipt, AnalysisLifecycleState, AnalysisPlanIssue,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{card, heading, property_row, status_dot};

const PLAN_NAME: &str = "Lab characterization";

#[derive(Clone)]
struct SelectedAnalysis {
    id: AnalysisInstanceId,
    kind: AnalysisKind,
    draft: AnalysisDraft,
    enabled: bool,
    dependencies: Vec<AnalysisDependency>,
    lifecycle: AnalysisLifecycleState,
    position: usize,
    plan_length: usize,
    issues: Vec<AnalysisPlanIssue>,
}

#[derive(Clone)]
struct AnalysisStackRow {
    id: AnalysisInstanceId,
    kind: AnalysisKind,
    enabled: bool,
    lifecycle: AnalysisLifecycleState,
    summary: String,
    issue_count: usize,
}

#[derive(Debug, Clone, Copy)]
enum AnalysisAction {
    Clone,
    Earlier(usize),
    Later(usize),
    BindDependencies,
    Validate,
    Remove,
    SetEnabled(bool),
}

#[derive(Debug, Clone, Copy)]
enum StackAction {
    Select(AnalysisInstanceId),
    SetEnabled(AnalysisInstanceId, bool),
    Insert(AnalysisKind),
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if let Err(error) = resolve_active_analysis_instance(app) {
        record_failure(app, "Analysis selection", &error);
    }

    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        let surface_width = ui.available_width();
        ScrollArea::vertical()
            .id_salt("workbench.simulate.surface")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let content_width = (surface_width - 44.0).clamp(0.0, 920.0);
                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    ui.add_space(22.0);
                    ui.vertical(|ui| {
                        // A vertical ScrollArea may otherwise size its child
                        // from desired content width. Bind the work surface to
                        // the actual center-pane viewport so compact layouts
                        // cannot retain the desktop 920 px width off-screen.
                        ui.set_width(content_width);
                        plan_heading(ui, app);
                        ui.add_space(16.0);
                        plan_summary(ui, app);
                        ui.add_space(12.0);
                        analysis_workspace(ui, app);
                        ui.add_space(12.0);
                        preflight(ui, app);
                    });
                });
                ui.add_space(22.0);
            });
    });
}

fn analysis_workspace(ui: &mut Ui, app: &mut RSpiceApp) {
    if ui.available_width() >= 760.0 {
        ui.columns(2, |columns| {
            ordered_instance_stack(&mut columns[0], app);
            analysis_editor(&mut columns[1], app);
        });
    } else {
        ordered_instance_stack(ui, app);
        ui.add_space(12.0);
        analysis_editor(ui, app);
    }
}

fn ordered_instance_stack(ui: &mut Ui, app: &mut RSpiceApp) {
    let rows = match analysis_stack_rows(app) {
        Ok(rows) => rows,
        Err(error) => {
            card(ui, "Ordered analyses", |ui| {
                status_dot(ui, Tokens::get(ui.ctx()).color.err, "Plan unavailable");
                ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
            });
            return;
        }
    };
    let active = app.state.workbench.active_analysis_instance;
    let mut action = None;
    card(ui, "Ordered analyses", |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("{} active", rows.len()))
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(Tokens::get(ui.ctx()).color.text_faint),
            );
            ui.add_space(4.0);
            if ui.button("Add analysis…").clicked() {
                app.state.sim_setup.palette_open = true;
                app.state.sim_setup.palette_query.clear();
                app.state.sim_setup.palette_active = 0;
            }
        });
        ui.add_space(6.0);

        if rows.is_empty() {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.warn,
                "No analysis instances",
            );
            ui.label("Use Add analysis to create the first stable instance.");
        } else {
            for (position, row) in rows.iter().enumerate() {
                let selected = active == Some(row.id);
                let t = Tokens::get(ui.ctx());
                egui::Frame::new()
                    .fill(if selected {
                        t.color.accent_dim
                    } else {
                        t.color.bg_inset
                    })
                    .stroke(egui::Stroke::new(
                        1.0,
                        if selected {
                            t.color.accent
                        } else {
                            t.color.border
                        },
                    ))
                    .corner_radius(t.radius)
                    .inner_margin(egui::Margin::symmetric(9, 7))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let label = format!(
                                "{:02} · {} · {}",
                                position + 1,
                                row.kind.stable_id().to_uppercase(),
                                row.kind.label()
                            );
                            if ui.selectable_label(selected, label).clicked() {
                                action = Some(StackAction::Select(row.id));
                            }
                            let mut enabled = row.enabled;
                            if ui
                                .checkbox(&mut enabled, "Enabled")
                                .on_hover_text(format!(
                                    "Enable {} instance {}",
                                    row.kind.label(),
                                    row.id
                                ))
                                .changed()
                            {
                                action = Some(StackAction::SetEnabled(row.id, enabled));
                            }
                        });
                        ui.label(
                            egui::RichText::new(row.id.to_string())
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                        ui.label(
                            egui::RichText::new(&row.summary)
                                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.horizontal_wrapped(|ui| {
                            status_dot(
                                ui,
                                if row.issue_count == 0 {
                                    t.color.ok
                                } else {
                                    t.color.err
                                },
                                if row.issue_count == 0 {
                                    availability_label(row.kind)
                                } else {
                                    "dependency blocked"
                                },
                            );
                            ui.label(
                                egui::RichText::new(format!("\u{00b7} {}", row.lifecycle))
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_faint),
                            );
                        });
                    });
                ui.add_space(6.0);
            }
        }
    });

    analysis_catalog_window(ui.ctx(), app, &rows, &mut action);

    match action {
        Some(StackAction::Select(id)) => {
            app.state.workbench.active_analysis_instance = Some(id);
        }
        Some(StackAction::SetEnabled(id, enabled)) => {
            apply_analysis_action(app, id, AnalysisAction::SetEnabled(enabled));
        }
        Some(StackAction::Insert(kind)) => insert_analysis_instance(app, kind),
        None => {}
    }
}

fn analysis_stack_rows(app: &RSpiceApp) -> Result<Vec<AnalysisStackRow>, String> {
    let setup = &app.state.sim_setup;
    let plan = setup.stable_analysis_plan()?;
    let issues = plan.validation_issues();
    Ok(plan
        .instances()
        .iter()
        .map(|instance| AnalysisStackRow {
            id: instance.id(),
            kind: instance.kind(),
            enabled: instance.enabled(),
            lifecycle: instance.lifecycle(),
            summary: setup.analysis_draft_summary(instance.draft()),
            issue_count: issues
                .iter()
                .filter(|issue| issue_applies_to(issue, instance.id()))
                .count(),
        })
        .collect())
}

fn analysis_catalog_window(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    rows: &[AnalysisStackRow],
    action: &mut Option<StackAction>,
) {
    if !app.state.sim_setup.palette_open {
        return;
    }

    let mut open = true;
    let mut query = app.state.sim_setup.palette_query.clone();
    let mut active = app.state.sim_setup.palette_active;
    let mut chosen = None;
    let mut request_close = false;
    let available_window_width = (ctx.content_rect().width() - 24.0).max(1.0);
    egui::Window::new("Add analysis")
        .id(egui::Id::new("workbench.simulate.analysis_catalog"))
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .default_width(available_window_width.min(560.0))
        .max_width(available_window_width.min(720.0))
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new("ANALYSIS CATALOG")
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(Tokens::get(ui.ctx()).color.accent),
            );
            ui.label(
                egui::RichText::new("Create an independent stable analysis instance.")
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
            let search = ui.add(
                egui::TextEdit::singleline(&mut query)
                    .hint_text("Search analyses")
                    .desired_width(f32::INFINITY),
            );
            if search.changed() {
                active = 0;
            }
            if app.state.sim_setup.palette_query.is_empty() {
                search.request_focus();
            }

            let filtered = filtered_catalog_kinds(&query);
            if filtered.is_empty() {
                active = 0;
            } else {
                active = active.min(filtered.len() - 1);
                if ui.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
                    active = (active + 1).min(filtered.len() - 1);
                }
                if ui.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
                    active = active.saturating_sub(1);
                }
                if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                    chosen = filtered.get(active).copied();
                }
            }
            if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                request_close = true;
            }

            ui.separator();
            ScrollArea::vertical()
                .id_salt("workbench.simulate.analysis_catalog.rows")
                .max_height(440.0)
                .show(ui, |ui| {
                    if filtered.is_empty() {
                        ui.label(
                            egui::RichText::new("No analysis matches this search.")
                                .color(Tokens::get(ui.ctx()).color.text_dim),
                        );
                        return;
                    }
                    let mut previous_group = None;
                    for (index, kind) in filtered.iter().copied().enumerate() {
                        let group = analysis_catalog_group(kind);
                        if previous_group != Some(group) {
                            previous_group = Some(group);
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(group.to_uppercase())
                                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                    .color(Tokens::get(ui.ctx()).color.text_faint),
                            );
                        }
                        let configured = rows.iter().filter(|row| row.kind == kind).count();
                        let disposition = if configured == 0 {
                            "Add instance".to_owned()
                        } else {
                            format!("Add another · {configured} in plan")
                        };
                        let selected = index == active;
                        let compact = ui.available_width() < 440.0;
                        egui::Frame::new()
                            .fill(if selected {
                                Tokens::get(ui.ctx()).color.accent_dim
                            } else {
                                Tokens::get(ui.ctx()).color.bg_inset
                            })
                            .inner_margin(egui::Margin::symmetric(8, 5))
                            .show(ui, |ui| {
                                let mut row_content = |ui: &mut egui::Ui| {
                                    if ui
                                        .selectable_label(
                                            selected,
                                            format!(
                                                "{} · {}",
                                                kind.stable_id().to_uppercase(),
                                                kind.label()
                                            ),
                                        )
                                        .on_hover_text(format!(
                                            "Add {} analysis instance",
                                            kind.label()
                                        ))
                                        .clicked()
                                    {
                                        chosen = Some(kind);
                                    }
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{disposition} · {}",
                                            availability_label(kind)
                                        ))
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(Tokens::get(ui.ctx()).color.text_faint),
                                    );
                                };
                                if compact {
                                    ui.vertical(&mut row_content);
                                } else {
                                    ui.horizontal_wrapped(&mut row_content);
                                }
                            });
                    }
                });
        });

    if let Some(kind) = chosen {
        *action = Some(StackAction::Insert(kind));
        request_close = true;
    }
    if request_close {
        open = false;
    }
    app.state.sim_setup.palette_open = open;
    app.state.sim_setup.palette_query = query;
    app.state.sim_setup.palette_active = active;
}

fn filtered_catalog_kinds(query: &str) -> Vec<AnalysisKind> {
    let query = query.trim().to_ascii_lowercase();
    [
        "Numerical analyses",
        "Sweeps & variation",
        "Derived measurements",
        "Verification analyses",
        "Optimization analyses",
    ]
    .into_iter()
    .flat_map(|group| {
        AnalysisKind::ALL
            .into_iter()
            .filter(move |kind| analysis_catalog_group(*kind) == group)
    })
    .filter(|kind| {
        query.is_empty()
            || format!(
                "{} {} {} {}",
                kind.stable_id(),
                kind.label(),
                analysis_catalog_group(*kind),
                availability_label(*kind)
            )
            .to_ascii_lowercase()
            .contains(&query)
    })
    .collect()
}

fn plan_heading(ui: &mut Ui, app: &mut RSpiceApp) {
    let (eyebrow, description, plan_available) = match app.state.sim_setup.stable_analysis_plan() {
        Ok(plan) => {
            let enabled = plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
                .count();
            let prior = app.state.simulation.runs.first().map_or_else(
                || "no prior dataset".to_owned(),
                |run| format!("prior Run {} immutable", run.id),
            );
            (
                format!("Simulation plan · revision {}", plan.revision().get()),
                format!(
                    "{enabled} enabled instances · {} active · {} canonical types discoverable · {} PVT points · {prior}",
                    plan.instances().len(),
                    AnalysisKind::ALL.len(),
                    configured_pvt_count(plan.instances()),
                ),
                true,
            )
        }
        Err(error) => (
            "Simulation plan · unavailable".to_owned(),
            format!("Stable plan unavailable · {error}"),
            false,
        ),
    };

    let mut validate = false;
    if ui.available_width() >= 520.0 {
        ui.horizontal_top(|ui| {
            let heading_width = (ui.available_width() - 132.0).max(220.0);
            ui.vertical(|ui| {
                ui.set_width(heading_width);
                heading(ui, &eyebrow, PLAN_NAME, &description);
            });
            validate = ui
                .add_enabled(plan_available, egui::Button::new("Validate plan"))
                .clicked();
        });
    } else {
        heading(ui, &eyebrow, PLAN_NAME, &description);
        ui.add_space(6.0);
        validate = ui
            .add_enabled(plan_available, egui::Button::new("Validate plan"))
            .clicked();
    }
    if validate {
        Command::PreflightChecks.execute(app);
    }
}

fn plan_summary(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Active plan", |ui| {
        match app.state.sim_setup.stable_analysis_plan() {
            Ok(plan) => {
                property_row(ui, "Plan name", PLAN_NAME);
                property_row(ui, "Stable plan identity", &plan.id().to_string());
                property_row(ui, "Plan revision", &plan.revision().get().to_string());
                let run_order = plan
                    .instances()
                    .iter()
                    .filter(|instance| instance.enabled())
                    .map(|instance| instance.kind().label())
                    .collect::<Vec<_>>()
                    .join(" → ");
                property_row(
                    ui,
                    "Run order",
                    if run_order.is_empty() {
                        "No analyses enabled"
                    } else {
                        &run_order
                    },
                );
            }
            Err(error) => {
                property_row(ui, "Plan name", PLAN_NAME);
                property_row(ui, "Plan status", "Unavailable");
                ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
            }
        }
        property_row(
            ui,
            "Process corner",
            app.state.sim_setup.reference_pvt.process.short_name(),
        );
        property_row(
            ui,
            "Temperature",
            &format!(
                "{} °C",
                app.state.sim_setup.reference_pvt.temperature_celsius
            ),
        );
        property_row(
            ui,
            "Model libraries",
            &app.state.model_library_manager.library_count().to_string(),
        );
        ui.horizontal_wrapped(|ui| {
            let command = if app.state.simulation.is_running {
                Command::StopSimulation
            } else {
                Command::RunSimulation
            };
            let run = if app.state.simulation.is_running {
                "Stop active run"
            } else {
                "Run active plan"
            };
            if ui
                .add_enabled(command.is_enabled(app), egui::Button::new(run))
                .clicked()
            {
                command.execute(app);
            }
            if ui.button("Global solver and convergence…").clicked() {
                Command::SimulationOptions.execute(app);
            }
            if ui.button("Inspect generated netlist").clicked() {
                Command::GenerateNetlist.execute(app);
            }
        });
    });
}

fn analysis_editor(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected = match selected_analysis(app) {
        Ok(Some(selected)) => selected,
        Ok(None) => {
            card(ui, "Analysis instance", |ui| {
                status_dot(
                    ui,
                    Tokens::get(ui.ctx()).color.err,
                    "No active analysis instance",
                );
                ui.label("The stable plan is empty. Add an analysis from the plan navigator.");
            });
            lifecycle_receipt(ui, app);
            return;
        }
        Err(error) => {
            record_failure(app, "Analysis editor", &error);
            card(ui, "Analysis instance", |ui| {
                status_dot(ui, Tokens::get(ui.ctx()).color.err, "Plan unavailable");
                ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
            });
            lifecycle_receipt(ui, app);
            return;
        }
    };

    let prior_datasets = app.state.simulation.runs.first().map_or_else(
        || "No prior datasets".to_owned(),
        |run| format!("Run {} retained · immutable", run.id),
    );
    let mut draft = selected.draft.clone();
    let serialized_before = serde_json::to_vec(&draft);
    let mut action = None;

    card(ui, selected.kind.label(), |ui| {
        // Keep the durable identity on its own line. UUID-sized values and
        // lifecycle text otherwise collide with the Enabled control in the
        // narrow analysis column and on phones.
        ui.vertical(|ui| {
            let mut enabled = selected.enabled;
            if ui.checkbox(&mut enabled, "Enabled").changed() {
                action = Some(AnalysisAction::SetEnabled(enabled));
            }
            ui.label(
                egui::RichText::new(format!(
                    "{} · lifecycle {}",
                    selected.id, selected.lifecycle
                ))
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(Tokens::get(ui.ctx()).color.text_faint),
            );
        });
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            if ui.button("Clone").clicked() {
                action = Some(AnalysisAction::Clone);
            }
            if ui
                .add_enabled(selected.position > 0, egui::Button::new("Earlier"))
                .clicked()
            {
                action = Some(AnalysisAction::Earlier(selected.position - 1));
            }
            if ui
                .add_enabled(
                    selected.position + 1 < selected.plan_length,
                    egui::Button::new("Later"),
                )
                .clicked()
            {
                action = Some(AnalysisAction::Later(selected.position + 1));
            }
            if ui.button("Bind dependencies").clicked() {
                action = Some(AnalysisAction::BindDependencies);
            }
            if ui.button("Validate").clicked() {
                action = Some(AnalysisAction::Validate);
            }
            if ui.button("Remove").clicked() {
                action = Some(AnalysisAction::Remove);
            }
        });

        ui.add_space(8.0);
        property_row(ui, "Stable instance identity", &selected.id.to_string());
        property_row(
            ui,
            "Ordered position",
            &format!("{} / {}", selected.position + 1, selected.plan_length),
        );
        prerequisite_rows(ui, &selected);
        property_row(ui, "Availability", availability_label(selected.kind));
        property_row(ui, "Prior datasets", &prior_datasets);

        ui.add_space(6.0);
        if let Some(issue) = selected.issues.first() {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.err,
                "Dependency graph blocked",
            );
            ui.label(
                egui::RichText::new(format_plan_issue(issue))
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        } else {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.ok,
                "Dependency identity, order, and enabled state are valid for this instance.",
            );
        }

        ui.separator();
        let note = analysis_form::form(ui, &mut draft);
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(note)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(Tokens::get(ui.ctx()).color.text_dim),
        );
        if let Some(error) = app.state.sim_setup.analysis_draft_validation_error(&draft) {
            ui.add_space(6.0);
            ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
        } else {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.ok,
                "Analysis configuration valid",
            );
        }
    });

    let draft_changed = match (serialized_before, serde_json::to_vec(&draft)) {
        (Ok(before), Ok(after)) => before != after,
        (Err(error), _) | (_, Err(error)) => {
            record_failure(
                app,
                "Analysis edit",
                &format!("the draft could not be serialized exactly: {error}"),
            );
            lifecycle_receipt(ui, app);
            return;
        }
    };

    if draft_changed {
        commit_draft(app, selected.id, draft);
    }
    if let Some(action) = action {
        apply_analysis_action(app, selected.id, action);
    }
    lifecycle_receipt(ui, app);
}

fn prerequisite_rows(ui: &mut Ui, selected: &SelectedAnalysis) {
    if selected.kind.prerequisites().is_empty() {
        property_row(ui, "Prerequisites", "none declared");
        return;
    }
    for prerequisite in selected.kind.prerequisites() {
        let target = selected
            .dependencies
            .iter()
            .find(|dependency| dependency.prerequisite() == *prerequisite)
            .map_or_else(
                || "unbound · preflight blocked".to_owned(),
                |dependency| dependency.target().to_string(),
            );
        property_row(
            ui,
            &format!("{} prerequisite", prerequisite.stable_id().to_uppercase()),
            &target,
        );
    }
}

fn lifecycle_receipt(ui: &mut Ui, app: &RSpiceApp) {
    card(ui, "Lifecycle receipt", |ui| {
        ui.label(
            egui::RichText::new(&app.state.workbench.analysis_lifecycle_status)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(Tokens::get(ui.ctx()).color.text_dim),
        );
        match app.state.sim_setup.stable_analysis_plan() {
            Ok(plan) => {
                let latest = plan.receipts().last().map_or_else(
                    || "No committed receipt".to_owned(),
                    |receipt| {
                        format!(
                            "#{} · {} · {} · revision {}",
                            receipt.sequence(),
                            lifecycle_command_label(receipt.command()),
                            receipt.outcome(),
                            receipt.committed_revision().get()
                        )
                    },
                );
                property_row(ui, "Latest receipt", &latest);
                property_row(ui, "Tombstones", &plan.tombstones().len().to_string());
            }
            Err(error) => {
                property_row(ui, "Latest receipt", "Unavailable");
                ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
            }
        }
    });
}

fn preflight(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Preflight", |ui| {
        let topology_ok = !app.state.schematic.components.is_empty();
        let checks_ok = app.state.current_blocking_drc_result().is_none();
        let (analyses_ok, configurations_ok, graph_ok, plan_error) =
            match app.state.sim_setup.stable_analysis_plan() {
                Ok(plan) => {
                    let enabled = plan
                        .instances()
                        .iter()
                        .filter(|instance| instance.enabled())
                        .collect::<Vec<_>>();
                    (
                        !enabled.is_empty(),
                        enabled.iter().all(|instance| {
                            app.state
                                .sim_setup
                                .analysis_draft_validation_error(instance.draft())
                                .is_none()
                        }),
                        plan.validation_issues().is_empty(),
                        None,
                    )
                }
                Err(error) => (false, false, false, Some(error)),
            };
        preflight_row(
            ui,
            topology_ok,
            "Design topology",
            if topology_ok {
                "Circuit contains devices"
            } else {
                "Add at least one component"
            },
        );
        preflight_row(
            ui,
            analyses_ok,
            "Run set",
            if analyses_ok {
                "At least one analysis instance enabled"
            } else {
                "Enable an analysis instance"
            },
        );
        preflight_row(
            ui,
            graph_ok,
            "Instance graph",
            if graph_ok {
                "Stable dependencies are ordered"
            } else {
                "Resolve plan lifecycle diagnostics"
            },
        );
        preflight_row(
            ui,
            configurations_ok,
            "Analysis configuration",
            if configurations_ok {
                "All enabled instances validate"
            } else {
                "Correct invalid analysis fields"
            },
        );
        preflight_row(
            ui,
            checks_ok,
            "Blocking checks",
            if checks_ok {
                "No current blocking violations"
            } else {
                "Resolve current schematic errors"
            },
        );
        if let Some(error) = plan_error {
            ui.add_space(6.0);
            ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
        } else if let Some(reason) = app.state.simulation_run_block_reason() {
            ui.add_space(6.0);
            ui.label(egui::RichText::new(reason).color(Tokens::get(ui.ctx()).color.warn));
        }
    });
}

fn preflight_row(ui: &mut Ui, pass: bool, name: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        status_dot(
            ui,
            if pass { t.color.ok } else { t.color.err },
            if pass { "PASS" } else { "BLOCK" },
        );
        ui.label(egui::RichText::new(name).strong());
        ui.label(egui::RichText::new(detail).color(t.color.text_dim));
    });
}

fn resolve_active_analysis_instance(app: &mut RSpiceApp) -> Result<(), String> {
    let current = app.state.workbench.active_analysis_instance;
    let legacy_index = app.state.workbench.active_analysis;
    let resolved = {
        let plan = app.state.sim_setup.stable_analysis_plan()?;
        match current {
            Some(id) if plan.instance(id).is_some() => Some(id),
            Some(_) => plan.instances().first().map(|instance| instance.id()),
            None => AnalysisKind::from_legacy_index(legacy_index)
                .and_then(|kind| {
                    plan.instances()
                        .iter()
                        .find(|instance| instance.kind() == kind)
                })
                .or_else(|| plan.instances().first())
                .map(|instance| instance.id()),
        }
    };
    app.state.workbench.active_analysis_instance = resolved;
    Ok(())
}

fn selected_analysis(app: &RSpiceApp) -> Result<Option<SelectedAnalysis>, String> {
    let Some(id) = app.state.workbench.active_analysis_instance else {
        return Ok(None);
    };
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let Some((position, instance)) = plan
        .instances()
        .iter()
        .enumerate()
        .find(|(_, instance)| instance.id() == id)
    else {
        return Err(format!(
            "selected analysis instance {id} is not present in the stable plan"
        ));
    };
    let issues = plan
        .validation_issues()
        .into_iter()
        .filter(|issue| issue_applies_to(issue, id))
        .collect();
    Ok(Some(SelectedAnalysis {
        id,
        kind: instance.kind(),
        draft: instance.draft().clone(),
        enabled: instance.enabled(),
        dependencies: instance.dependencies().to_vec(),
        lifecycle: instance.lifecycle(),
        position,
        plan_length: plan.instances().len(),
        issues,
    }))
}

fn commit_draft(app: &mut RSpiceApp, id: AnalysisInstanceId, draft: AnalysisDraft) {
    let result = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .edit(id, |target| target.clone_from(&draft))
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    match result {
        Ok(((), receipt)) => {
            record_receipt(app, &receipt);
        }
        Err(error) => record_failure(app, "Edit", &error),
    }
}

type InsertAnalysisResult = Result<
    (
        AnalysisInstanceId,
        AnalysisLifecycleReceipt,
        Option<Result<AnalysisLifecycleReceipt, String>>,
    ),
    String,
>;

fn insert_analysis_instance(app: &mut RSpiceApp, kind: AnalysisKind) {
    app.state.sim_setup.palette_open = false;
    app.state.sim_setup.palette_query.clear();
    app.state.sim_setup.palette_active = 0;
    let result: InsertAnalysisResult = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => match plan.insert(kind) {
            Ok((id, insert_receipt)) => {
                let bind_receipt = (!kind.prerequisites().is_empty()).then(|| {
                    plan.auto_bind_dependencies(id)
                        .map_err(|error| error.to_string())
                });
                Ok((id, insert_receipt, bind_receipt))
            }
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error),
    };
    match result {
        Ok((id, insert_receipt, bind_receipt)) => {
            app.state.workbench.active_analysis_instance = Some(id);
            match bind_receipt {
                None => record_receipt(app, &insert_receipt),
                Some(Ok(bind_receipt)) => {
                    let issue = app
                        .state
                        .sim_setup
                        .stable_analysis_plan()
                        .ok()
                        .and_then(|plan| {
                            plan.validation_issues()
                                .iter()
                                .find(|issue| issue_applies_to(issue, id))
                                .map(format_plan_issue)
                        });
                    let readiness = issue.map_or_else(
                        || "Dependencies are explicitly bound.".to_owned(),
                        |issue| format!("Preflight remains blocked: {issue}"),
                    );
                    app.state.workbench.analysis_lifecycle_status = format!(
                        "Receipts #{} and #{} committed for instance {id}. {} {} {readiness} Prior datasets remain immutable.",
                        insert_receipt.sequence(),
                        bind_receipt.sequence(),
                        insert_receipt.detail(),
                        bind_receipt.detail(),
                    );
                }
                Some(Err(error)) => {
                    app.state.workbench.analysis_lifecycle_status = format!(
                        "Receipt #{} committed for instance {id}. {} Automatic dependency binding was rejected fail-closed: {error}. The inserted instance remains selected and preflight blocked; prior datasets remain immutable.",
                        insert_receipt.sequence(),
                        insert_receipt.detail(),
                    );
                }
            }
        }
        Err(error) => record_failure(app, "Add analysis", &error),
    }
}

fn apply_analysis_action(app: &mut RSpiceApp, id: AnalysisInstanceId, action: AnalysisAction) {
    match action {
        AnalysisAction::Clone => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan.clone_instance(id).map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok((clone_id, receipt)) => {
                    app.state.workbench.active_analysis_instance = Some(clone_id);
                    record_receipt(app, &receipt);
                }
                Err(error) => record_failure(app, "Clone", &error),
            }
        }
        AnalysisAction::Earlier(position) | AnalysisAction::Later(position) => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .reorder(id, position)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => record_receipt(app, &receipt),
                Err(error) => record_failure(app, "Reorder", &error),
            }
        }
        AnalysisAction::BindDependencies => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .auto_bind_dependencies(id)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => record_receipt(app, &receipt),
                Err(error) => record_failure(app, "Bind dependencies", &error),
            }
        }
        AnalysisAction::Validate => validate_analysis_instance(app, id),
        AnalysisAction::Remove => remove_analysis_instance(app, id),
        AnalysisAction::SetEnabled(enabled) => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .set_enabled(id, enabled)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => record_receipt(app, &receipt),
                Err(error) => record_failure(app, "Enable state", &error),
            }
        }
    }
}

fn validate_analysis_instance(app: &mut RSpiceApp, id: AnalysisInstanceId) {
    let result = (|| {
        let plan = app.state.sim_setup.stable_analysis_plan()?;
        let instance = plan
            .instance(id)
            .ok_or_else(|| format!("analysis instance {id} no longer exists"))?;
        if let Some(issue) = plan
            .validation_issues()
            .iter()
            .find(|issue| issue_applies_to(issue, id))
        {
            return Err(format_plan_issue(issue));
        }
        if let Some(error) = app
            .state
            .sim_setup
            .analysis_draft_validation_error(instance.draft())
        {
            return Err(error);
        }
        Ok(instance.kind())
    })();

    match result {
        Ok(kind) => {
            app.state.workbench.analysis_lifecycle_status = format!(
                "Validation passed for {} instance {id}. Dependency identity, order, and enabled state are valid for this instance.",
                kind.label()
            );
        }
        Err(error) => record_failure(app, "Validate", &error),
    }
}

fn remove_analysis_instance(app: &mut RSpiceApp, id: AnalysisInstanceId) {
    let prior_run_ids = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| run.find_analysis_by_source_instance(id).is_some())
        .map(|run| run.run_id)
        .collect();
    let position = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .position(|instance| instance.id() == id)
        })
        .unwrap_or(0);
    let result = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .remove(id, prior_run_ids)
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    match result {
        Ok(receipt) => {
            let next = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| {
                    let next_position = position.min(plan.instances().len().saturating_sub(1));
                    plan.instances()
                        .get(next_position)
                        .map(|instance| instance.id())
                });
            app.state.workbench.active_analysis_instance = next;
            record_receipt(app, &receipt);
        }
        Err(error) => record_failure(app, "Remove", &error),
    }
}

fn record_receipt(app: &mut RSpiceApp, receipt: &AnalysisLifecycleReceipt) {
    let related = receipt
        .related_instance_id()
        .map_or_else(String::new, |id| format!(" · related instance {id}"));
    app.state.workbench.analysis_lifecycle_status = format!(
        "Receipt #{} · {} committed for instance {}{related} · revision {} to {} · outcome {}. {} Prior datasets remain immutable.",
        receipt.sequence(),
        lifecycle_command_label(receipt.command()),
        receipt.instance_id(),
        receipt.source_revision().get(),
        receipt.committed_revision().get(),
        receipt.outcome(),
        receipt.detail(),
    );
}

fn record_failure(app: &mut RSpiceApp, action: &str, error: &str) {
    app.state.workbench.analysis_lifecycle_status = format!(
        "{action} rejected fail-closed: {error}. The stable plan is unchanged and prior datasets remain immutable."
    );
}

const fn lifecycle_command_label(command: AnalysisLifecycleCommand) -> &'static str {
    match command {
        AnalysisLifecycleCommand::Insert => "Insert",
        AnalysisLifecycleCommand::Edit => "Edit",
        AnalysisLifecycleCommand::Clone => "Clone",
        AnalysisLifecycleCommand::Disable => "Enable or disable",
        AnalysisLifecycleCommand::Reorder => "Reorder",
        AnalysisLifecycleCommand::Dependency => "Bind dependencies",
        AnalysisLifecycleCommand::Validate => "Validate",
        AnalysisLifecycleCommand::Preflight => "Preflight",
        AnalysisLifecycleCommand::Execute => "Execute",
        AnalysisLifecycleCommand::Remove => "Remove",
    }
}

const fn availability_label(kind: AnalysisKind) -> &'static str {
    match kind {
        AnalysisKind::Pac
        | AnalysisKind::Pnoise
        | AnalysisKind::Pxf
        | AnalysisKind::Pstb
        | AnalysisKind::Envelope
        | AnalysisKind::Reliability => "Preview · non-sign-off",
        AnalysisKind::Disto => "Compatibility · non-sign-off",
        AnalysisKind::OperatingPoint
        | AnalysisKind::Transient
        | AnalysisKind::Ac
        | AnalysisKind::DcSweep
        | AnalysisKind::Noise
        | AnalysisKind::PoleZero
        | AnalysisKind::Sensitivity
        | AnalysisKind::MonteCarlo
        | AnalysisKind::Pss
        | AnalysisKind::Stb
        | AnalysisKind::Temperature
        | AnalysisKind::HarmonicBalance
        | AnalysisKind::SParameter
        | AnalysisKind::TransferFunction
        | AnalysisKind::Corner
        | AnalysisKind::Fourier
        | AnalysisKind::Optimization
        | AnalysisKind::Soa => "Production",
    }
}

const fn analysis_catalog_group(kind: AnalysisKind) -> &'static str {
    match kind {
        AnalysisKind::MonteCarlo | AnalysisKind::Temperature | AnalysisKind::Corner => {
            "Sweeps & variation"
        }
        AnalysisKind::Fourier | AnalysisKind::Disto => "Derived measurements",
        AnalysisKind::Reliability | AnalysisKind::Soa => "Verification analyses",
        AnalysisKind::Optimization => "Optimization analyses",
        AnalysisKind::OperatingPoint
        | AnalysisKind::Transient
        | AnalysisKind::Ac
        | AnalysisKind::DcSweep
        | AnalysisKind::Noise
        | AnalysisKind::PoleZero
        | AnalysisKind::Sensitivity
        | AnalysisKind::Pss
        | AnalysisKind::Stb
        | AnalysisKind::HarmonicBalance
        | AnalysisKind::SParameter
        | AnalysisKind::Pac
        | AnalysisKind::Pnoise
        | AnalysisKind::Pxf
        | AnalysisKind::Pstb
        | AnalysisKind::TransferFunction
        | AnalysisKind::Envelope => "Numerical analyses",
    }
}

fn configured_pvt_count(instances: &[crate::simulation::plan::AnalysisInstance]) -> usize {
    let mut count = 0usize;
    let mut has_nominal_analysis = false;
    for instance in instances.iter().filter(|instance| instance.enabled()) {
        match instance.draft() {
            AnalysisDraft::Corner(draft) => {
                count = count
                    .saturating_add(draft.to_config().map_or(0, |config| config.num_corners()));
            }
            AnalysisDraft::Temperature(draft) => {
                count =
                    count.saturating_add(draft.to_config().map_or(0, |config| config.num_temps()));
            }
            AnalysisDraft::OperatingPoint(_)
            | AnalysisDraft::Transient(_)
            | AnalysisDraft::Ac(_)
            | AnalysisDraft::DcSweep(_)
            | AnalysisDraft::Noise(_)
            | AnalysisDraft::PoleZero(_)
            | AnalysisDraft::Sensitivity(_)
            | AnalysisDraft::MonteCarlo(_)
            | AnalysisDraft::Pss(_)
            | AnalysisDraft::Stb(_)
            | AnalysisDraft::HarmonicBalance(_)
            | AnalysisDraft::SParameter(_)
            | AnalysisDraft::Pac(_)
            | AnalysisDraft::Pnoise(_)
            | AnalysisDraft::Pxf(_)
            | AnalysisDraft::Pstb(_)
            | AnalysisDraft::TransferFunction(_)
            | AnalysisDraft::Envelope(_)
            | AnalysisDraft::Fourier(_)
            | AnalysisDraft::Reliability(_)
            | AnalysisDraft::Optimization(_)
            | AnalysisDraft::Soa(_)
            | AnalysisDraft::Disto(_) => has_nominal_analysis = true,
        }
    }
    count
        .saturating_add(usize::from(has_nominal_analysis))
        .max(1)
}

fn issue_applies_to(issue: &AnalysisPlanIssue, id: AnalysisInstanceId) -> bool {
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => true,
        AnalysisPlanIssue::DuplicateInstanceId { id: issue_id }
        | AnalysisPlanIssue::ReusedTombstonedId { id: issue_id }
        | AnalysisPlanIssue::KindDraftMismatch { id: issue_id, .. }
        | AnalysisPlanIssue::InvalidInstanceRevision { id: issue_id }
        | AnalysisPlanIssue::InvalidLifecycle { id: issue_id, .. } => *issue_id == id,
        AnalysisPlanIssue::MissingPrerequisite { dependent, .. }
        | AnalysisPlanIssue::UnexpectedDependencyRole { dependent, .. }
        | AnalysisPlanIssue::DuplicateDependencyRole { dependent, .. }
        | AnalysisPlanIssue::SelfDependency { dependent }
        | AnalysisPlanIssue::DanglingDependency { dependent, .. }
        | AnalysisPlanIssue::WrongDependencyKind { dependent, .. }
        | AnalysisPlanIssue::DisabledDependency { dependent, .. }
        | AnalysisPlanIssue::DependencyNotEarlier { dependent, .. } => *dependent == id,
        AnalysisPlanIssue::DependencyCycle { members } => members.contains(&id),
        AnalysisPlanIssue::DuplicateTombstoneId { .. }
        | AnalysisPlanIssue::InvalidTombstoneRevision { .. }
        | AnalysisPlanIssue::InvalidReceiptSequence { .. }
        | AnalysisPlanIssue::InvalidReceiptRevision { .. }
        | AnalysisPlanIssue::DanglingReceiptInstance { .. }
        | AnalysisPlanIssue::ReceiptKindMismatch { .. }
        | AnalysisPlanIssue::EmptyReceiptDetail { .. }
        | AnalysisPlanIssue::InvalidNextReceiptSequence { .. } => false,
    }
}

fn format_plan_issue(issue: &AnalysisPlanIssue) -> String {
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => {
            "The simulation plan has no enabled analysis instances.".to_owned()
        }
        AnalysisPlanIssue::DuplicateInstanceId { id } => {
            format!("Stable analysis identity {id} is duplicated.")
        }
        AnalysisPlanIssue::DuplicateTombstoneId { id } => {
            format!("Retired analysis identity {id} has duplicate tombstones.")
        }
        AnalysisPlanIssue::ReusedTombstonedId { id } => {
            format!("Retired analysis identity {id} was reused by an active instance.")
        }
        AnalysisPlanIssue::KindDraftMismatch {
            id,
            expected,
            actual,
        } => format!(
            "Analysis {id} requires a {} draft, but contains {}.",
            expected.label(),
            actual.label()
        ),
        AnalysisPlanIssue::InvalidInstanceRevision { id } => {
            format!("Analysis {id} has an invalid revision range.")
        }
        AnalysisPlanIssue::InvalidLifecycle { id, state, enabled } => {
            format!("Analysis {id} lifecycle {state} conflicts with enabled state {enabled}.")
        }
        AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} requires an earlier enabled {} instance.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::UnexpectedDependencyRole {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} does not accept {} as a prerequisite.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::DuplicateDependencyRole {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} binds {} more than once.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::SelfDependency { dependent } => {
            format!("Analysis {dependent} cannot depend on itself.")
        }
        AnalysisPlanIssue::DanglingDependency { dependent, target } => {
            format!("Analysis {dependent} references missing prerequisite instance {target}.")
        }
        AnalysisPlanIssue::WrongDependencyKind {
            dependent,
            prerequisite,
            target,
            actual,
        } => format!(
            "Analysis {dependent} requires {} at {target}, but that instance is {}.",
            prerequisite.label(),
            actual.label()
        ),
        AnalysisPlanIssue::DisabledDependency { dependent, target } => {
            format!("Analysis {dependent} prerequisite instance {target} is disabled.")
        }
        AnalysisPlanIssue::DependencyNotEarlier { dependent, target } => {
            format!("Analysis {dependent} prerequisite instance {target} must appear earlier.")
        }
        AnalysisPlanIssue::DependencyCycle { members } => format!(
            "Analysis dependency cycle contains {} instance{}.",
            members.len(),
            if members.len() == 1 { "" } else { "s" }
        ),
        AnalysisPlanIssue::InvalidTombstoneRevision { id } => {
            format!("Retired analysis identity {id} has an invalid revision range.")
        }
        AnalysisPlanIssue::InvalidReceiptSequence { sequence } => {
            format!("Lifecycle receipt sequence {sequence} is not contiguous.")
        }
        AnalysisPlanIssue::InvalidReceiptRevision { sequence } => {
            format!("Lifecycle receipt {sequence} has an invalid revision transition.")
        }
        AnalysisPlanIssue::DanglingReceiptInstance { sequence, id } => {
            format!("Lifecycle receipt {sequence} references unknown analysis instance {id}.")
        }
        AnalysisPlanIssue::ReceiptKindMismatch {
            sequence,
            expected,
            actual,
        } => format!(
            "Lifecycle receipt {sequence} identifies {}, but its retained analysis is {}.",
            actual.label(),
            expected.label()
        ),
        AnalysisPlanIssue::EmptyReceiptDetail { sequence } => {
            format!("Lifecycle receipt {sequence} has no status detail.")
        }
        AnalysisPlanIssue::InvalidNextReceiptSequence { expected, actual } => {
            format!("Next lifecycle receipt sequence is {actual}; expected {expected}.")
        }
    }
}
