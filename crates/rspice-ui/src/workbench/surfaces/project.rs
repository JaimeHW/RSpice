//! Project workspace surfaces from the workbench mockup.
//!
//! Every value is resolved from the live project, library, simulation, and
//! recovery state. The surface never substitutes fixture counts or synthetic
//! project history when a project has not produced that evidence yet.

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{card, heading, property_row, status_dot};
use super::super::state::{ModelsPage, ProjectPage, Workspace};

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt("workbench.project.surface")
            .show(ui, |ui| {
                ui.set_max_width(1180.0);
                ui.add_space(22.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| match app.state.workbench.project_page {
                        ProjectPage::Dashboard => dashboard(ui, app),
                        ProjectPage::Configuration => configuration(ui, app),
                        ProjectPage::Technology => technology(ui, app),
                        ProjectPage::Dependencies => dependencies(ui, app),
                        ProjectPage::Recovery => recovery(ui, app),
                    });
                    ui.add_space(24.0);
                });
                ui.add_space(24.0);
            });
    });
}

fn dashboard(ui: &mut Ui, app: &mut RSpiceApp) {
    let revision = app.state.workspace.project.revision().get();
    let project_name = app.state.workspace.project.display_name().to_owned();
    let sheet_count = app.state.workspace.schematic_buffers.len();
    let instance_count = app.state.schematic.components.len();
    let dirty = app.state.schematic.is_dirty || app.state.workspace.any_dirty();

    header_with_actions(
        ui,
        &format!("PROJECT WORKSPACE · REVISION {revision}"),
        &project_name,
        &format!(
            "Hierarchical analog design · {sheet_count} sheet{} · {instance_count} instance{} · {}",
            plural(sheet_count),
            plural(instance_count),
            if dirty {
                "working changes"
            } else {
                "saved state current"
            }
        ),
        |ui, app| {
            if ui.button("Open project…").clicked() {
                Command::OpenProject.execute(app);
            }
            if ui.button("Open top schematic").clicked() {
                Command::OpenWorkspace(Workspace::Design).execute(app);
            }
        },
        app,
    );
    ui.add_space(18.0);
    engineering_status(ui, app);
    ui.add_space(16.0);

    let width = ((ui.available_width() - 16.0) / 2.0).max(300.0);
    ui.horizontal_wrapped(|ui| {
        ui.set_width(width);
        design_entry_points(ui, app);
        ui.set_width(width);
        recent_activity(ui, app);
    });
}

fn engineering_status(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let topology = app.state.schematic.topology_version();
    let checks_current = app.state.dialogs.drc_checked_version == topology;
    let check_summary = checks_current
        .then(|| {
            app.state
                .dialogs
                .drc_results
                .as_ref()
                .map(|result| result.summary())
        })
        .flatten();
    let enabled_analyses = app.state.sim_setup.enabled_analysis_instance_count();
    let valid_analyses = app
        .state
        .sim_setup
        .enabled_analysis_instances()
        .filter(|instance| {
            app.state
                .sim_setup
                .analysis_draft_validation_error(instance.draft())
                .is_none()
        })
        .count();
    let graph_valid = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .is_ok_and(|plan| plan.validation_issues().is_empty());
    let model_files = app.state.pdk_config.discovered_files.len();
    let model_errors = app.state.pdk_config.scan_errors.len();

    ui.horizontal_wrapped(|ui| {
        status_card(
            ui,
            "Schematic checks",
            check_summary
                .map(|summary| format!("{} errors", summary.errors + summary.critical))
                .as_deref()
                .unwrap_or("stale"),
            check_summary
                .map(|summary| format!("{} advisories", summary.warnings + summary.info))
                .as_deref()
                .unwrap_or("Run checks for this design revision"),
            check_summary.is_some_and(|summary| summary.passed),
            || Command::RunChecks.execute(app),
        );
        status_card(
            ui,
            "Simulation plan",
            &format!("{enabled_analyses} analyses"),
            &format!("{valid_analyses} validated"),
            enabled_analyses > 0 && valid_analyses == enabled_analyses && graph_valid,
            || Command::OpenWorkspace(Workspace::Simulate).execute(app),
        );
        let spec_count = app.state.workspace.specs.len();
        status_card(
            ui,
            "Verification",
            &format!("{spec_count} specifications"),
            if app.state.simulation.runs.is_empty() {
                "No result evidence yet"
            } else {
                "Result evidence available"
            },
            spec_count > 0 && !app.state.simulation.runs.is_empty(),
            || Command::OpenWorkspace(Workspace::Verify).execute(app),
        );
        status_card(
            ui,
            "Model dependencies",
            &format!("{model_files} files"),
            if model_errors == 0 {
                "Resolved"
            } else {
                "Review scan diagnostics"
            },
            model_errors == 0,
            || Command::ModelsPage(ModelsPage::Catalog).execute(app),
        );
    });
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(0.0, t.color.border),
    );
}

fn status_card(
    ui: &mut Ui,
    label: &str,
    value: &str,
    detail: &str,
    ok: bool,
    action: impl FnOnce(),
) {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_min_width(230.0);
                ui.label(
                    egui::RichText::new(label.to_uppercase())
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                ui.label(
                    egui::RichText::new(value)
                        .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                        .color(if ok { t.color.ok } else { t.color.warn }),
                );
                ui.label(
                    egui::RichText::new(detail)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
            });
        })
        .response
        .interact(egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("Open {label}: {value}. {detail}"),
        )
    });
    theme::paint_focus_ring(ui, &response, response.rect);
    if response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        action();
    }
}

fn design_entry_points(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Design entry points", |ui| {
        let open_views = app.state.workspace.open_views.clone();
        for document in open_views {
            let active = document.reference == app.state.workspace.active_view;
            let label = format!(
                "{} · {}{}",
                document.reference.cell,
                document.reference.view,
                if document.dirty { " · modified" } else { "" }
            );
            if ui
                .selectable_label(active, label)
                .on_hover_text(document.reference.display_path())
                .clicked()
            {
                app.state.open_workspace_view(document.reference);
                Command::OpenWorkspace(Workspace::Design).execute(app);
            }
        }
        ui.separator();
        if ui
            .button(format!(
                "Simulation plan · {} enabled",
                app.state.sim_setup.enabled_analysis_instance_count()
            ))
            .clicked()
        {
            Command::OpenWorkspace(Workspace::Simulate).execute(app);
        }
        if ui
            .button(format!(
                "Verification cockpit · {} specifications",
                app.state.workspace.specs.len()
            ))
            .clicked()
        {
            Command::OpenWorkspace(Workspace::Verify).execute(app);
        }
    });
}

fn recent_activity(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Recent activity", |ui| {
        if app.state.simulation.runs.is_empty() && app.state.recent_files.is_empty() {
            muted(ui, "No project runs or recently opened files yet.");
        }
        let runs = app
            .state
            .simulation
            .runs
            .iter()
            .enumerate()
            .rev()
            .take(3)
            .map(|(index, run)| {
                let measurement_count: usize = run
                    .analyses
                    .iter()
                    .map(|analysis| analysis.measurements.len())
                    .sum();
                let passed_count = run
                    .analyses
                    .iter()
                    .flat_map(|analysis| &analysis.measurements)
                    .filter(|measurement| measurement.passed)
                    .count();
                (
                    index,
                    run.label.clone(),
                    run.analyses.len(),
                    run.elapsed_time,
                    measurement_count,
                    passed_count,
                )
            })
            .collect::<Vec<_>>();
        for (index, label, analysis_count, elapsed_time, measurement_count, passed_count) in runs {
            let label = format!(
                "{} · {} analyses · {:.3} s",
                label, analysis_count, elapsed_time
            );
            if ui
                .selectable_label(false, label)
                .on_hover_text(format!(
                    "{passed_count} / {measurement_count} measurements passed"
                ))
                .clicked()
            {
                app.state.simulation.select_run(index);
                Command::OpenWorkspace(Workspace::Results).execute(app);
            }
        }
        for recent in app.state.recent_files.iter().take(3) {
            ui.label(
                egui::RichText::new(recent.path.display().to_string())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
        }
        if app.state.schematic.is_dirty || app.state.workspace.any_dirty() {
            ui.separator();
            if ui.button("Open Recovery").clicked() {
                app.state.workbench.project_page = ProjectPage::Recovery;
            }
        }
    });
}

fn configuration(ui: &mut Ui, app: &mut RSpiceApp) {
    header_with_actions(
        ui,
        "TESTBENCH CONFIGURATION",
        "Hierarchy and view binding",
        "Define the exact design, view, model, environment, and netlisting contracts used by every run.",
        |ui, app| {
            if ui.button("Validate configuration").clicked() {
                Command::PreflightChecks.execute(app);
            }
        },
        app,
    );
    ui.add_space(16.0);
    card(ui, "Hierarchy binding", |ui| {
        property_row(
            ui,
            "Active root",
            &app.state.workspace.active_display_path(),
        );
        property_row(
            ui,
            "View type",
            app.state.workspace.active_view_type().display_name(),
        );
        property_row(
            ui,
            "Hierarchy depth",
            &app.state.workspace.hierarchy_stack.len().to_string(),
        );
        property_row(
            ui,
            "Open cellviews",
            &app.state.workspace.open_views.len().to_string(),
        );
        property_row(ui, "Unbound cell policy", "Block execution");
    });
    ui.add_space(12.0);
    let width = ((ui.available_width() - 12.0) / 2.0).max(280.0);
    ui.horizontal_wrapped(|ui| {
        ui.set_width(width);
        card(ui, "Netlisting policy", |ui| {
            property_row(ui, "Hierarchy", "Preserve names");
            property_row(ui, "Parameter evaluation", "Strict units");
            property_row(ui, "Global ground", "Node 0 canonical");
            property_row(ui, "Generated source", "Mapped to cell/view identity");
        });
        ui.set_width(width);
        card(ui, "Environment", |ui| {
            property_row(
                ui,
                "Technology",
                app.state
                    .workspace
                    .project
                    .technology
                    .as_deref()
                    .unwrap_or("Not attached"),
            );
            property_row(
                ui,
                "Process corner",
                app.state.sim_setup.reference_pvt.process.short_name(),
            );
            property_row(
                ui,
                "Model libraries",
                &app.state.model_library_manager.library_count().to_string(),
            );
            property_row(
                ui,
                "Enabled analyses",
                &app.state
                    .sim_setup
                    .enabled_analysis_instance_count()
                    .to_string(),
            );
        });
    });
}

fn technology(ui: &mut Ui, app: &mut RSpiceApp) {
    let technology = app
        .state
        .workspace
        .project
        .technology
        .clone()
        .unwrap_or_else(|| "No technology attached".to_owned());
    header_with_actions(
        ui,
        "TECHNOLOGY ATTACHMENT",
        "Technology and PDK contract",
        "Versioned attachment for symbols, models, sections, and qualification evidence.",
        |ui, app| {
            if ui.button("Attach technology…").clicked() {
                Command::PdkSettings.execute(app);
            }
        },
        app,
    );
    ui.add_space(16.0);
    card(ui, "PDK resources", |ui| {
        property_row(ui, "Attachment", &technology);
        property_row(
            ui,
            "Configured search paths",
            &app.state.pdk_config.library_paths.len().to_string(),
        );
        property_row(
            ui,
            "Discovered model files",
            &app.state.pdk_config.discovered_files.len().to_string(),
        );
        property_row(
            ui,
            "Scan diagnostics",
            &app.state.pdk_config.scan_errors.len().to_string(),
        );
        for path in &app.state.pdk_config.library_paths {
            ui.label(
                egui::RichText::new(path.display_name())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
        }
    });
}

fn dependencies(ui: &mut Ui, app: &mut RSpiceApp) {
    header_with_actions(
        ui,
        "DEPENDENCY MANIFEST · CONTENT ADDRESSED",
        "Project dependency graph",
        "Libraries, model files, behavioral sources, and external contracts required to reproduce the project.",
        |_ui, _app| {},
        app,
    );
    ui.add_space(16.0);
    card(ui, "Locked dependencies", |ui| {
        property_row(
            ui,
            "Design libraries",
            &app.state.library_manager.library_count().to_string(),
        );
        property_row(
            ui,
            "Cells",
            &app.state.library_manager.total_cell_count().to_string(),
        );
        property_row(
            ui,
            "Cellviews",
            &app.state.library_manager.total_view_count().to_string(),
        );
        property_row(
            ui,
            "Model libraries",
            &app.state.model_library_manager.library_count().to_string(),
        );
        property_row(
            ui,
            "Device models",
            &app.state
                .model_library_manager
                .total_model_count()
                .to_string(),
        );
        for library in app.state.library_manager.libraries_sorted() {
            ui.separator();
            property_row(
                ui,
                &library.name,
                if library.read_only {
                    "Read only"
                } else {
                    "Project writable"
                },
            );
        }
    });
    ui.add_space(12.0);
    card(ui, "Reproducibility contract", |ui| {
        property_row(ui, "Project format", "RSpice project schema 1");
        property_row(ui, "Missing dependency", "Fail closed");
        property_row(ui, "Mutable external path", "Explicit project reference");
        property_row(
            ui,
            "Project identity",
            &app.state.workspace.project.id().to_string(),
        );
    });
}

fn recovery(ui: &mut Ui, app: &mut RSpiceApp) {
    header_with_actions(
        ui,
        "PROJECT RECOVERY · NON-DESTRUCTIVE",
        "Recovery Center",
        "Review live working changes and save a durable project copy without replacing recoverable state.",
        |ui, app| {
            if ui.button("Save current project").clicked() {
                Command::Save.execute(app);
            }
            if ui.button("Save project copy…").clicked() {
                Command::SaveAs.execute(app);
            }
        },
        app,
    );
    ui.add_space(16.0);
    let dirty_documents: Vec<_> = app
        .state
        .workspace
        .open_views
        .iter()
        .filter(|view| view.dirty)
        .map(|view| view.reference.display_path())
        .collect();
    card(ui, "Recoverable working state", |ui| {
        property_row(
            ui,
            "Open documents",
            &app.state.workspace.open_views.len().to_string(),
        );
        property_row(ui, "Modified documents", &dirty_documents.len().to_string());
        property_row(
            ui,
            "Active schematic",
            if app.state.schematic.is_dirty {
                "Modified"
            } else {
                "Current"
            },
        );
        property_row(
            ui,
            "Manual netlist",
            if app.state.workspace.netlist_source_dirty {
                "Modified"
            } else {
                "Current"
            },
        );
        property_row(
            ui,
            "Project path",
            &app.state
                .workspace
                .project
                .path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "Not saved yet".to_owned()),
        );
        if dirty_documents.is_empty()
            && !app.state.schematic.is_dirty
            && !app.state.workspace.netlist_source_dirty
        {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.ok,
                "No recovery action required",
            );
        } else {
            for document in dirty_documents {
                muted(ui, &document);
            }
        }
    });
}

fn header_with_actions(
    ui: &mut Ui,
    eyebrow: &str,
    title: &str,
    description: &str,
    actions: impl FnOnce(&mut Ui, &mut RSpiceApp),
    app: &mut RSpiceApp,
) {
    ui.horizontal_wrapped(|ui| {
        ui.vertical(|ui| heading(ui, eyebrow, title, description));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            actions(ui, app);
        });
    });
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_faint),
    );
}
