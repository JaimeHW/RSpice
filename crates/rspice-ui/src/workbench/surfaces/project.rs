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

const PROJECT_RESPONSIVE_BREAKPOINT: f32 = 820.0;
const PROJECT_CONTENT_MAX_WIDTH: f32 = 1180.0;
const PROJECT_HORIZONTAL_GUTTER: f32 = 24.0;
const PROJECT_GRID_GAP: f32 = 1.0;
const PROJECT_HISTORY_MIN_WIDTH: f32 = 280.0;
const PROJECT_PRIMARY_SHARE: f32 = 0.65;
const STATUS_CARD_MIN_HEIGHT: f32 = 68.0;
const STATUS_CARD_INNER_MARGIN: i8 = 12;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ProjectSurfaceLayout {
    content_width: f32,
    horizontal_inset: f32,
}

impl ProjectSurfaceLayout {
    fn resolve(surface_width: f32) -> Self {
        let surface_width = surface_width.max(0.0);
        let content_width =
            (surface_width - PROJECT_HORIZONTAL_GUTTER * 2.0).clamp(0.0, PROJECT_CONTENT_MAX_WIDTH);
        Self {
            content_width,
            horizontal_inset: (surface_width - content_width) * 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DashboardLayout {
    status_columns: usize,
    status_card_width: f32,
    project_columns: usize,
    project_primary_width: f32,
    project_history_width: f32,
}

impl DashboardLayout {
    fn resolve(viewport_width: f32, content_width: f32) -> Self {
        let content_width = content_width.max(0.0);
        let compact = viewport_width <= PROJECT_RESPONSIVE_BREAKPOINT;
        let status_columns = if compact { 2 } else { 4 };
        let project_columns = if compact { 1 } else { 2 };
        let status_card_width = content_width / status_columns as f32;

        let (project_primary_width, project_history_width) = if compact {
            (content_width, content_width)
        } else {
            let usable_width = (content_width - PROJECT_GRID_GAP).max(0.0);
            let history_width = (usable_width * (1.0 - PROJECT_PRIMARY_SHARE))
                .max(PROJECT_HISTORY_MIN_WIDTH.min(usable_width));
            (usable_width - history_width, history_width)
        };

        Self {
            status_columns,
            status_card_width,
            project_columns,
            project_primary_width,
            project_history_width,
        }
    }
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        let surface = ProjectSurfaceLayout::resolve(ui.available_width());
        ScrollArea::vertical()
            .id_salt("workbench.project.surface")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(22.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.add_space(surface.horizontal_inset);
                    ui.allocate_ui_with_layout(
                        egui::vec2(surface.content_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(surface.content_width);
                            match app.state.workbench.project_page {
                                ProjectPage::Dashboard => dashboard(ui, app),
                                ProjectPage::Configuration => configuration(ui, app),
                                ProjectPage::Technology => technology(ui, app),
                                ProjectPage::Dependencies => dependencies(ui, app),
                                ProjectPage::Recovery => recovery(ui, app),
                            }
                        },
                    );
                    ui.add_space(surface.horizontal_inset);
                });
                ui.add_space(24.0);
            });
    });
}

fn dashboard(ui: &mut Ui, app: &mut RSpiceApp) {
    let layout = DashboardLayout::resolve(ui.ctx().content_rect().width(), ui.available_width());
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
    engineering_status(ui, app, layout);
    ui.add_space(16.0);

    project_grid(ui, app, layout);
}

struct StatusCardSpec {
    label: &'static str,
    value: String,
    detail: String,
    ok: bool,
    command: Command,
}

fn engineering_status(ui: &mut Ui, app: &mut RSpiceApp, layout: DashboardLayout) {
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
    let spec_count = app.state.workspace.specs.len();
    let has_result_evidence = !app.state.simulation.runs.is_empty();
    let cards = [
        StatusCardSpec {
            label: "Schematic checks",
            value: check_summary
                .map(|summary| format!("{} errors", summary.errors + summary.critical))
                .unwrap_or_else(|| "stale".to_owned()),
            detail: check_summary
                .map(|summary| format!("{} advisories", summary.warnings + summary.info))
                .unwrap_or_else(|| "Run checks for this design revision".to_owned()),
            ok: check_summary.is_some_and(|summary| summary.passed),
            command: Command::RunChecks,
        },
        StatusCardSpec {
            label: "Simulation plan",
            value: format!("{enabled_analyses} analyses"),
            detail: format!("{valid_analyses} validated"),
            ok: enabled_analyses > 0 && valid_analyses == enabled_analyses && graph_valid,
            command: Command::OpenWorkspace(Workspace::Simulate),
        },
        StatusCardSpec {
            label: "Verification",
            value: format!("{spec_count} specifications"),
            detail: if has_result_evidence {
                "Result evidence available".to_owned()
            } else {
                "No result evidence yet".to_owned()
            },
            ok: spec_count > 0 && has_result_evidence,
            command: Command::OpenWorkspace(Workspace::Verify),
        },
        StatusCardSpec {
            label: "Model dependencies",
            value: format!("{model_files} files"),
            detail: if model_errors == 0 {
                "Resolved".to_owned()
            } else {
                "Review scan diagnostics".to_owned()
            },
            ok: model_errors == 0,
            command: Command::ModelsPage(ModelsPage::Catalog),
        },
    ];
    let mut requested_command = None;

    egui::Grid::new("workbench.project.engineering-status")
        .num_columns(layout.status_columns)
        .spacing(egui::Vec2::ZERO)
        .show(ui, |ui| {
            for (index, card) in cards.iter().enumerate() {
                let clicked = ui
                    .allocate_ui_with_layout(
                        egui::vec2(layout.status_card_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(layout.status_card_width);
                            status_card(ui, card.label, &card.value, &card.detail, card.ok)
                        },
                    )
                    .inner;
                if clicked {
                    requested_command = Some(card.command);
                }
                if (index + 1) % layout.status_columns == 0 {
                    ui.end_row();
                }
            }
        });
    if let Some(command) = requested_command {
        command.execute(app);
    }
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(0.0, t.color.border),
    );
}

fn status_card(ui: &mut Ui, label: &str, value: &str, detail: &str, ok: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::same(STATUS_CARD_INNER_MARGIN))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                let content_width = ui.available_width().max(0.0);
                ui.set_min_width(content_width);
                ui.set_max_width(content_width);
                ui.set_min_height(
                    STATUS_CARD_MIN_HEIGHT - f32::from(STATUS_CARD_INNER_MARGIN) * 2.0,
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(label.to_uppercase())
                            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                            .color(t.color.text_dim),
                    )
                    .truncate(),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(value)
                            .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                            .color(if ok { t.color.ok } else { t.color.warn }),
                    )
                    .truncate(),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(detail)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    )
                    .truncate(),
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
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

fn project_grid(ui: &mut Ui, app: &mut RSpiceApp, layout: DashboardLayout) {
    if layout.project_columns == 1 {
        ui.allocate_ui_with_layout(
            egui::vec2(layout.project_primary_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(layout.project_primary_width);
                design_entry_points(ui, app);
            },
        );
        ui.add_space(PROJECT_GRID_GAP);
        ui.allocate_ui_with_layout(
            egui::vec2(layout.project_history_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(layout.project_history_width);
                recent_activity(ui, app);
            },
        );
        return;
    }

    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = PROJECT_GRID_GAP;
        ui.allocate_ui_with_layout(
            egui::vec2(layout.project_primary_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(layout.project_primary_width);
                design_entry_points(ui, app);
            },
        );
        ui.allocate_ui_with_layout(
            egui::vec2(layout.project_history_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(layout.project_history_width);
                recent_activity(ui, app);
            },
        );
    });
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
    if project_header_stacks(ui.ctx().content_rect().width()) {
        ui.vertical(|ui| {
            heading(ui, eyebrow, title, description);
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| actions(ui, app));
        });
        return;
    }
    ui.horizontal_wrapped(|ui| {
        ui.vertical(|ui| heading(ui, eyebrow, title, description));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            actions(ui, app);
        });
    });
}

fn project_header_stacks(viewport_width: f32) -> bool {
    viewport_width <= PROJECT_RESPONSIVE_BREAKPOINT
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() <= 0.001, "{actual} != {expected}");
    }

    #[test]
    fn dashboard_columns_switch_at_the_exact_mockup_breakpoint() {
        let compact = DashboardLayout::resolve(PROJECT_RESPONSIVE_BREAKPOINT, 772.0);
        let wide = DashboardLayout::resolve(PROJECT_RESPONSIVE_BREAKPOINT + 0.01, 772.0);

        assert_eq!(compact.status_columns, 2);
        assert_eq!(compact.project_columns, 1);
        assert_close(compact.status_card_width, 386.0);
        assert_close(compact.project_primary_width, 772.0);
        assert_close(compact.project_history_width, 772.0);

        assert_eq!(wide.status_columns, 4);
        assert_eq!(wide.project_columns, 2);
        assert_close(wide.status_card_width, 193.0);
        assert_close(
            wide.project_primary_width + PROJECT_GRID_GAP + wide.project_history_width,
            772.0,
        );
        assert_close(wide.project_history_width, PROJECT_HISTORY_MIN_WIDTH);
        assert!(project_header_stacks(PROJECT_RESPONSIVE_BREAKPOINT));
        assert!(!project_header_stacks(PROJECT_RESPONSIVE_BREAKPOINT + 0.01));
    }

    #[test]
    fn responsive_geometry_never_requests_width_outside_the_surface() {
        for surface_width in [0.0, 20.0, 820.0, 1600.0] {
            let surface = ProjectSurfaceLayout::resolve(surface_width);
            assert!(surface.content_width >= 0.0);
            assert!(surface.horizontal_inset >= 0.0);
            assert_close(
                surface.horizontal_inset * 2.0 + surface.content_width,
                surface_width,
            );
        }

        for (viewport_width, content_width) in [
            (390.0, 342.0),
            (820.0, 772.0),
            (821.0, 773.0),
            (1440.0, 1180.0),
        ] {
            let layout = DashboardLayout::resolve(viewport_width, content_width);
            assert_close(
                layout.status_card_width * layout.status_columns as f32,
                content_width,
            );
            if layout.project_columns == 1 {
                assert_close(layout.project_primary_width, content_width);
                assert_close(layout.project_history_width, content_width);
            } else {
                assert_close(
                    layout.project_primary_width + PROJECT_GRID_GAP + layout.project_history_width,
                    content_width,
                );
            }
        }
    }
}
