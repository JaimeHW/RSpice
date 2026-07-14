//! Project ownership, dependencies, activity, and recovery.

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{card, heading, property_row, status_dot};
use super::super::state::ProjectPage;

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
                    ui.vertical(|ui| {
                        heading(
                            ui,
                            "Project owner",
                            app.state.workspace.project.display_name(),
                            "Authoritative design identity, dependencies, recent work, and recovery state.",
                        );
                    });
                });
                ui.add_space(20.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(24.0);
                    for page in ProjectPage::ALL {
                        if ui
                            .selectable_label(app.state.workbench.project_page == page, page.label())
                            .clicked()
                        {
                            app.state.workbench.project_page = page;
                        }
                    }
                });
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    ui.vertical(|ui| match app.state.workbench.project_page {
                        ProjectPage::Overview => overview(ui, app),
                        ProjectPage::Dependencies => dependencies(ui, app),
                        ProjectPage::Activity => activity(ui, app),
                        ProjectPage::Recovery => recovery(ui, app),
                    });
                });
                ui.add_space(24.0);
            });
    });
}

fn overview(ui: &mut Ui, app: &mut RSpiceApp) {
    let available = ui.available_width();
    let card_width = ((available - 16.0) / 2.0).max(280.0);
    ui.horizontal_wrapped(|ui| {
        ui.set_width(card_width);
        card(ui, "Design context", |ui| {
            property_row(
                ui,
                "Root library",
                &app.state.workspace.project.root_library,
            );
            property_row(ui, "Top cell", &app.state.workspace.project.top_cell);
            property_row(
                ui,
                "Active document",
                &app.state.workspace.active_display_path(),
            );
            property_row(
                ui,
                "Open documents",
                &app.state.workspace.open_views.len().to_string(),
            );
            if ui.button("Open design workspace").clicked() {
                Command::OpenWorkspace(super::super::state::Workspace::Design).execute(app);
            }
        });
        ui.set_width(card_width);
        card(ui, "Project health", |ui| {
            let dirty = app
                .state
                .workspace
                .open_views
                .iter()
                .filter(|view| view.dirty)
                .count()
                + usize::from(app.state.workspace.netlist_source_dirty);
            status_dot(
                ui,
                if dirty == 0 {
                    Tokens::get(ui.ctx()).color.ok
                } else {
                    Tokens::get(ui.ctx()).color.warn
                },
                if dirty == 0 {
                    "All documents saved"
                } else {
                    "Unsaved project changes"
                },
            );
            property_row(ui, "Dirty documents", &dirty.to_string());
            property_row(
                ui,
                "Schematic revision",
                &app.state.schematic.topology_version().to_string(),
            );
            property_row(
                ui,
                "Result datasets",
                &app.state.simulation.runs.len().to_string(),
            );
            if ui.button("Save project").clicked() {
                Command::Save.execute(app);
            }
        });
    });
    ui.add_space(12.0);
    card(ui, "Quick actions", |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui.button("New project…").clicked() {
                Command::NewProject.execute(app);
            }
            if ui.button("Open project…").clicked() {
                Command::OpenProject.execute(app);
            }
            if ui.button("New cell…").clicked() {
                Command::NewCell.execute(app);
            }
            if ui.button("Import SPICE deck…").clicked() {
                Command::ImportNetlist.execute(app);
            }
            if ui.button("Configure PDK…").clicked() {
                Command::PdkSettings.execute(app);
            }
        });
    });
}

fn dependencies(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Design libraries", |ui| {
        property_row(
            ui,
            "Libraries",
            &app.state.library_manager.library_count().to_string(),
        );
        property_row(
            ui,
            "Cells",
            &app.state.library_manager.total_cell_count().to_string(),
        );
        property_row(
            ui,
            "Views",
            &app.state.library_manager.total_view_count().to_string(),
        );
        for library in app.state.library_manager.libraries_sorted() {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&library.name).strong());
                ui.label(if library.read_only {
                    "read only"
                } else {
                    "editable"
                });
                ui.label(format!("{} cells", library.cell_count()));
                if !library.technology.is_empty() {
                    ui.label(format!("technology {}", library.technology));
                }
            });
        }
    });
    ui.add_space(12.0);
    card(ui, "Models and process", |ui| {
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
        property_row(
            ui,
            "Configured search paths",
            &app.state.pdk_config.library_paths.len().to_string(),
        );
        property_row(
            ui,
            "Discovered files",
            &app.state.pdk_config.discovered_files.len().to_string(),
        );
        property_row(
            ui,
            "Scan errors",
            &app.state.pdk_config.scan_errors.len().to_string(),
        );
        if ui.button("Manage dependencies…").clicked() {
            Command::PdkSettings.execute(app);
        }
    });
}

fn activity(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Recent documents", |ui| {
        if app.state.recent_files.is_empty() {
            muted(ui, "No recent project or schematic files in this session.");
        }
        for recent in &app.state.recent_files {
            property_row(
                ui,
                &format!("{:?}", recent.kind),
                &recent.path.display().to_string(),
            );
        }
    });
    ui.add_space(12.0);
    card(ui, "Simulation activity", |ui| {
        if app.state.simulation.runs.is_empty() {
            muted(ui, "No completed simulations yet.");
        }
        for run in app.state.simulation.runs.iter().take(12) {
            ui.horizontal(|ui| {
                let t = Tokens::get(ui.ctx());
                status_dot(
                    ui,
                    if run.success { t.color.ok } else { t.color.err },
                    &run.label,
                );
                ui.label(format!(
                    "{} analyses · {:.3} s",
                    run.analyses.len(),
                    run.elapsed_time
                ));
            });
        }
    });
}

fn recovery(ui: &mut Ui, app: &mut RSpiceApp) {
    let dirty_documents: Vec<_> = app
        .state
        .workspace
        .open_views
        .iter()
        .filter(|view| view.dirty)
        .map(|view| view.reference.display_path())
        .collect();
    card(ui, "Recoverable session state", |ui| {
        property_row(
            ui,
            "Open documents",
            &app.state.workspace.open_views.len().to_string(),
        );
        property_row(ui, "Unsaved documents", &dirty_documents.len().to_string());
        property_row(
            ui,
            "Manual netlist dirty",
            if app.state.workspace.netlist_source_dirty {
                "Yes"
            } else {
                "No"
            },
        );
        property_row(
            ui,
            "Undo available",
            if app.state.schematic.can_undo() {
                "Yes"
            } else {
                "No"
            },
        );
        if dirty_documents.is_empty() && !app.state.workspace.netlist_source_dirty {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.ok,
                "No recovery action required",
            );
        } else {
            for document in dirty_documents {
                muted(ui, &document);
            }
            if ui.button("Save recoverable work").clicked() {
                Command::Save.execute(app);
            }
            if ui.button("Save a recovery copy…").clicked() {
                Command::SaveAs.execute(app);
            }
        }
    });
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_faint),
    );
}
