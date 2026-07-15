//! Model binding, library, PDK, behavioral-source, and qualification surfaces.

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::state::ViewType;
use crate::ui::tokens::Tokens;

use super::super::commands::Command;
use super::super::design_system::{card, heading, property_row, status_dot};
use super::super::state::ModelsPage;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt("workbench.models.surface")
            .show(ui, |ui| {
                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    ui.add_space(22.0);
                    ui.vertical(|ui| {
                        heading(
                            ui,
                            "Model-binding owner",
                            app.state.workbench.models_page.label(),
                            "PDK libraries, device models, cellviews, behavioral sources, and qualification status.",
                        );
                    });
                });
                ui.add_space(14.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(22.0);
                    for page in ModelsPage::ALL {
                        if ui
                            .selectable_label(app.state.workbench.models_page == page, page.label())
                            .clicked()
                        {
                            app.state.workbench.models_page = page;
                        }
                    }
                });
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    ui.add_space(22.0);
                    ui.vertical(|ui| {
                        ui.set_max_width(1120.0);
                        match app.state.workbench.models_page {
                            ModelsPage::Catalog => catalog(ui, app),
                            ModelsPage::Libraries => libraries(ui, app),
                            ModelsPage::Pdk => pdk(ui, app),
                            ModelsPage::Behavioral => behavioral(ui, app),
                            ModelsPage::Qualification => qualification(ui, app),
                        }
                    });
                });
                ui.add_space(24.0);
            });
    });
}

fn catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Device model catalog", |ui| {
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.add(
                egui::TextEdit::singleline(&mut app.state.model_library_manager.filter_text)
                    .desired_width(240.0)
                    .hint_text("Model name or description"),
            );
            if ui.button("Open advanced browser…").clicked() {
                Command::ModelBrowser.execute(app);
            }
        });
        ui.add_space(8.0);
        let query = app.state.model_library_manager.filter_text.clone();
        let models: Vec<_> = app
            .state
            .model_library_manager
            .search_models(&query)
            .into_iter()
            .map(|(library, model)| {
                (
                    library.name.clone(),
                    model.name.clone(),
                    format!("{:?}", model.model_type),
                    format!("{:?}", model.level),
                    model.description.clone(),
                    model.parameters.len(),
                )
            })
            .collect();
        if models.is_empty() {
            ui.label(
                "No models match the active filter. Configure a PDK path or load built-in models.",
            );
        }
        for (library, name, model_type, level, description, parameters) in models {
            let selected = app.state.workbench.selected_model.as_deref() == Some(&name)
                && app.state.model_library_manager.selected_library.as_deref() == Some(&library);
            if ui
                .selectable_label(
                    selected,
                    format!("{name}  ·  {model_type}  ·  {level}  ·  {parameters} parameters"),
                )
                .clicked()
            {
                app.state.model_library_manager.select_library(&library);
                app.state.workbench.selected_model = Some(name.clone());
            }
            if selected && !description.is_empty() {
                ui.label(
                    egui::RichText::new(description)
                        .small()
                        .color(Tokens::get(ui.ctx()).color.text_dim),
                );
            }
        }
    });
}

fn libraries(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Library / cell / view manager", |ui| {
        ui.horizontal_wrapped(|ui| {
            if ui.button("New cell…").clicked() {
                Command::NewCell.execute(app);
            }
            let selected_library = app.state.library_manager.selected_library.clone();
            let selected_cell = app.state.library_manager.selected_cell.clone();
            if ui
                .add_enabled(
                    selected_library.is_some() && selected_cell.is_some(),
                    egui::Button::new("New view…"),
                )
                .clicked()
            {
                app.state.dialogs.new_view_library = selected_library.unwrap_or_default();
                app.state.dialogs.new_view_cell = selected_cell.unwrap_or_default();
                app.state.dialogs.new_view_name.clear();
                app.state.dialogs.new_view_type = ViewType::Schematic;
                app.state.dialogs.new_view_dialog = true;
            }
            if ui.button("Import Verilog-A…").clicked() {
                Command::ImportVerilogA.execute(app);
            }
        });
        ui.add_space(8.0);
        let libraries: Vec<_> = app
            .state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .map(|library| {
                (
                    library.name.clone(),
                    library.read_only,
                    library
                        .cells_sorted()
                        .into_iter()
                        .map(|cell| {
                            (
                                cell.name.clone(),
                                cell.views_sorted()
                                    .into_iter()
                                    .map(|view| (view.name.clone(), view.view_type))
                                    .collect::<Vec<_>>(),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let mut open = None;
        for (library, read_only, cells) in libraries {
            egui::CollapsingHeader::new(if read_only {
                format!("{library} · read only")
            } else {
                library.clone()
            })
            .default_open(library == app.state.workspace.active_view.library)
            .show(ui, |ui| {
                for (cell, views) in cells {
                    egui::CollapsingHeader::new(cell.clone()).show(ui, |ui| {
                        for (view, view_type) in views {
                            if ui
                                .selectable_label(
                                    app.state.workspace.active_view.library == library
                                        && app.state.workspace.active_view.cell == cell
                                        && app.state.workspace.active_view.view == view,
                                    format!("{view} · {}", view_type.display_name()),
                                )
                                .clicked()
                            {
                                app.state
                                    .library_manager
                                    .select_view(&library, &cell, &view);
                                open = Some(crate::state::CellViewRef::new(&library, &cell, &view));
                            }
                        }
                    });
                }
            });
        }
        if let Some(reference) = open {
            app.state.open_workspace_view(reference);
            app.state
                .workbench
                .activate(super::super::state::Workspace::Design);
        }
    });
}

fn pdk(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Process design kit configuration", |ui| {
        property_row(
            ui,
            "Search paths",
            &app.state.pdk_config.library_paths.len().to_string(),
        );
        property_row(
            ui,
            "Environment overrides",
            &app.state.pdk_config.environment_variables.len().to_string(),
        );
        property_row(
            ui,
            "Discovered model files",
            &app.state.pdk_config.discovered_files.len().to_string(),
        );
        property_row(
            ui,
            "Scan errors",
            &app.state.pdk_config.scan_errors.len().to_string(),
        );
        for path in &app.state.pdk_config.library_paths {
            ui.separator();
            ui.label(format!("{path:?}"));
        }
        if !app.state.pdk_config.scan_errors.is_empty() {
            ui.collapsing("Discovery errors", |ui| {
                for error in &app.state.pdk_config.scan_errors {
                    ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
                }
            });
        }
        ui.horizontal_wrapped(|ui| {
            if ui.button("Configure paths and variables…").clicked() {
                Command::PdkSettings.execute(app);
            }
            if ui.button("Open model browser…").clicked() {
                Command::ModelBrowser.execute(app);
            }
        });
    });
}

fn behavioral(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Verilog-A / mixed-signal sources", |ui| {
        let sources: Vec<_> = app
            .state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .flat_map(|library| {
                library.cells_sorted().into_iter().flat_map(move |cell| {
                    cell.views_sorted()
                        .into_iter()
                        .filter(|view| {
                            matches!(view.view_type, ViewType::VerilogA | ViewType::Verilog)
                        })
                        .map(move |view| {
                            (
                                library.name.clone(),
                                cell.name.clone(),
                                view.name.clone(),
                                view.view_type,
                                view.file_path.clone(),
                            )
                        })
                })
            })
            .collect();
        property_row(ui, "Behavioral views", &sources.len().to_string());
        for (library, cell, view, view_type, path) in sources {
            ui.horizontal(|ui| {
                ui.label(format!("{library}/{cell}/{view}"));
                ui.label(view_type.display_name());
                ui.label(path.as_ref().map_or_else(
                    || "project source".to_owned(),
                    |path| path.display().to_string(),
                ));
            });
        }
        ui.horizontal_wrapped(|ui| {
            if ui.button("Import or compile Verilog-A…").clicked() {
                Command::CompileVerilogA.execute(app);
            }
            if ui.button("New behavioral cell…").clicked() {
                Command::NewCell.execute(app);
            }
        });
    });
}

fn qualification(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut missing_source = 0;
    let mut invalid_geometry = 0;
    let mut undocumented = 0;
    let mut total = 0;
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            total += 1;
            missing_source += usize::from(model.file_path.is_none());
            undocumented += usize::from(model.description.trim().is_empty());
            let l_invalid =
                matches!((model.l_min, model.l_max), (Some(min), Some(max)) if min > max);
            let w_invalid =
                matches!((model.w_min, model.w_max), (Some(min), Some(max)) if min > max);
            invalid_geometry += usize::from(l_invalid || w_invalid);
        }
    }
    card(ui, "Qualification audit", |ui| {
        property_row(ui, "Models audited", &total.to_string());
        property_row(ui, "Missing source provenance", &missing_source.to_string());
        property_row(
            ui,
            "Invalid geometry windows",
            &invalid_geometry.to_string(),
        );
        property_row(ui, "Missing descriptions", &undocumented.to_string());
        let blocking = missing_source + invalid_geometry;
        status_dot(
            ui,
            if blocking == 0 {
                Tokens::get(ui.ctx()).color.ok
            } else {
                Tokens::get(ui.ctx()).color.err
            },
            if blocking == 0 {
                "Model provenance and geometry audit passed"
            } else {
                "Qualification findings require review"
            },
        );
        ui.label("The audit is deterministic over every loaded model and reports provenance and geometry-contract gaps.");
    });
}
