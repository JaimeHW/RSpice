//! Workspace-aware navigation tree.

mod design;

use egui::{Align, Layout, ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::state::{CellViewRef, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::{WorkbenchIcon, icon_button, section_header};
use super::super::state::{ModelsPage, ProjectPage, VerificationPage, Workspace};

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    header(ui, app);
    match app.state.workbench.workspace {
        Workspace::Design => design::show(ui, app),
        workspace => {
            workspace_search(ui, app, workspace);
            match workspace {
                Workspace::Project => project(ui, app),
                Workspace::Simulate => simulate(ui, app),
                Workspace::Results => results(ui, app),
                Workspace::Verify => verify(ui, app),
                Workspace::Models => models(ui, app),
                Workspace::Netlist => netlist(ui, app),
                Workspace::Design => unreachable!("handled above"),
            }
        }
    }
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(app.state.workbench.workspace.navigator_title())
                .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                .color(t.color.text),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if icon_button(
                ui,
                WorkbenchIcon::Close,
                "Hide navigator",
                false,
                egui::vec2(30.0, 30.0),
            )
            .clicked()
            {
                app.state.workbench.navigator_visible = false;
            }
        });
    });
}

fn workspace_search(ui: &mut Ui, app: &mut RSpiceApp, workspace: Workspace) {
    let placeholder = match workspace {
        Workspace::Project => "Filter libraries, cells, views…",
        Workspace::Design => "Find instance, net or port…",
        Workspace::Simulate => "Filter setup…",
        Workspace::Results => "Find signal, expression or run…",
        Workspace::Verify => "Filter flows, specs or samples…",
        Workspace::Models => "Search model, device or library…",
        Workspace::Netlist => "Find symbol or line…",
    };
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let response = ui.add_sized(
            [ui.available_width() - 16.0, 28.0],
            egui::TextEdit::singleline(&mut app.state.workbench.navigator_query)
                .id_salt("workbench.navigator.filter")
                .hint_text(placeholder)
                .margin(egui::Margin::symmetric(8, 5)),
        );
        if std::mem::take(&mut app.state.workbench.focus_navigator_search) {
            response.request_focus();
        }
    });
    ui.add_space(6.0);
}

fn project(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(
        ui,
        "Project",
        Some(app.state.workspace.project.display_name()),
    );
    ScrollArea::vertical().show(ui, |ui| {
        for page in ProjectPage::ALL {
            if nav_row(
                ui,
                WorkbenchIcon::Project,
                page.label(),
                app.state.workbench.project_page == page,
                None,
            ) {
                app.state.workbench.project_page = page;
            }
        }
        section_header(
            ui,
            "Design roots",
            Some(&app.state.library_manager.library_count().to_string()),
        );
        library_tree(ui, app, false);
    });
}

fn library_tree(ui: &mut Ui, app: &mut RSpiceApp, open_documents: bool) {
    let query = app.state.workbench.navigator_query.trim().to_lowercase();
    let libraries: Vec<_> = app
        .state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let mut cells: Vec<_> = library
                .cells_sorted()
                .into_iter()
                .map(|cell| {
                    let views = cell
                        .views_sorted()
                        .into_iter()
                        .map(|view| (view.name.clone(), view.view_type))
                        .collect::<Vec<_>>();
                    (cell.name.clone(), views)
                })
                .collect();
            cells.sort_by(|a, b| a.0.cmp(&b.0));
            (library.name.clone(), library.read_only, cells)
        })
        .collect();

    let mut requested = None;
    for (library, read_only, cells) in libraries {
        if !query.is_empty()
            && !library.to_lowercase().contains(&query)
            && !cells.iter().any(|(cell, views)| {
                cell.to_lowercase().contains(&query)
                    || views
                        .iter()
                        .any(|(view, _)| view.to_lowercase().contains(&query))
            })
        {
            continue;
        }
        egui::CollapsingHeader::new(if read_only {
            format!("{library}  ·  read only")
        } else {
            library.clone()
        })
        .default_open(library == app.state.workspace.active_view.library)
        .show(ui, |ui| {
            for (cell, views) in cells {
                egui::CollapsingHeader::new(cell.clone())
                    .default_open(cell == app.state.workspace.active_view.cell)
                    .show(ui, |ui| {
                        for (view, view_type) in views {
                            let reference = CellViewRef::new(&library, &cell, &view);
                            let active = reference == app.state.workspace.active_view;
                            if nav_row(
                                ui,
                                view_icon(view_type),
                                &view,
                                active,
                                Some(view_type.display_name()),
                            ) && open_documents
                            {
                                requested = Some(reference);
                            }
                        }
                    });
            }
        });
    }
    if let Some(reference) = requested {
        app.state.open_workspace_view(reference);
    }
}

fn simulate(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::common::simulation_analysis_tabs::SIMULATION_ANALYSIS_CATEGORIES;
    section_header(
        ui,
        "Run set",
        Some(&format!("{} enabled", app.state.sim_setup.enabled.len())),
    );
    ScrollArea::vertical()
        .id_salt("workbench.simulation.navigator")
        .show(ui, |ui| {
            let query = app.state.workbench.navigator_query.trim().to_lowercase();
            for (category, analyses) in SIMULATION_ANALYSIS_CATEGORIES {
                let filtered: Vec<_> = analyses
                    .iter()
                    .copied()
                    .filter(|(_, name)| query.is_empty() || name.to_lowercase().contains(&query))
                    .collect();
                if filtered.is_empty() {
                    continue;
                }
                section_header(ui, category, None);
                for (index, name) in filtered {
                    let mut enabled = app.state.sim_setup.enabled.contains(&index);
                    let error = app.state.sim_setup.validation_error(index);
                    ui.horizontal(|ui| {
                        ui.add_space(7.0);
                        if ui.checkbox(&mut enabled, "").changed() {
                            if enabled {
                                app.state.sim_setup.enabled.insert(index);
                                app.state.sim_setup.listed.insert(index);
                            } else {
                                app.state.sim_setup.enabled.remove(&index);
                            }
                        }
                        if nav_row(
                            ui,
                            if error.is_some() {
                                WorkbenchIcon::Warning
                            } else {
                                WorkbenchIcon::Simulate
                            },
                            name,
                            app.state.workbench.active_analysis == index,
                            Some(&app.state.sim_setup.summary(index)),
                        ) {
                            app.state.workbench.active_analysis = index;
                        }
                    });
                }
            }
        });
}

fn results(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(
        ui,
        "Datasets",
        Some(&format!("{} runs", app.state.simulation.runs.len())),
    );
    ScrollArea::vertical().show(ui, |ui| {
        if app.state.simulation.runs.is_empty() {
            muted(
                ui,
                "Run a simulation to create an immutable result dataset.",
            );
            return;
        }
        let runs: Vec<_> = app
            .state
            .simulation
            .runs
            .iter()
            .enumerate()
            .map(|(index, run)| {
                (
                    index,
                    run.label.clone(),
                    run.success,
                    run.elapsed_time,
                    run.analyses
                        .iter()
                        .enumerate()
                        .map(|(analysis_index, analysis)| {
                            (analysis_index, analysis.label.clone(), analysis.success)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        for (index, label, success, elapsed, analyses) in runs {
            let active = app.state.simulation.active_run_idx == Some(index);
            egui::CollapsingHeader::new(format!("{label}  ·  {elapsed:.3}s"))
                .default_open(active)
                .show(ui, |ui| {
                    if nav_row(
                        ui,
                        if success {
                            WorkbenchIcon::Success
                        } else {
                            WorkbenchIcon::Warning
                        },
                        "Run summary",
                        active && app.state.simulation.active_analysis_idx.is_none(),
                        None,
                    ) {
                        app.state.simulation.active_run_idx = Some(index);
                        app.state.simulation.active_analysis_idx = None;
                    }
                    for (analysis_index, analysis, analysis_success) in analyses {
                        if nav_row(
                            ui,
                            if analysis_success {
                                WorkbenchIcon::Results
                            } else {
                                WorkbenchIcon::Warning
                            },
                            &analysis,
                            active
                                && app.state.simulation.active_analysis_idx == Some(analysis_index),
                            None,
                        ) {
                            app.state.simulation.active_run_idx = Some(index);
                            app.state.simulation.active_analysis_idx = Some(analysis_index);
                        }
                    }
                });
        }
    });
}

fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Evidence", None);
    for page in VerificationPage::ALL {
        let meta = match page {
            VerificationPage::Specifications => Some(app.state.workspace.specs.len().to_string()),
            VerificationPage::History => Some(app.state.simulation.runs.len().to_string()),
            VerificationPage::Checks => app
                .state
                .dialogs
                .drc_results
                .as_ref()
                .map(|result| result.total_count().to_string()),
            _ => None,
        };
        if nav_row(
            ui,
            WorkbenchIcon::Verify,
            page.label(),
            app.state.workbench.verification_page == page,
            meta.as_deref(),
        ) {
            app.state.workbench.verification_page = page;
        }
    }
}

fn models(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Model ownership", None);
    for page in ModelsPage::ALL {
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            page.label(),
            app.state.workbench.models_page == page,
            None,
        ) {
            app.state.workbench.models_page = page;
        }
    }
    section_header(
        ui,
        "Loaded model libraries",
        Some(&app.state.model_library_manager.library_count().to_string()),
    );
    let libraries: Vec<_> = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| (library.name.clone(), library.model_count()))
        .collect();
    for (name, count) in libraries {
        let selected = app.state.model_library_manager.selected_library.as_deref() == Some(&name);
        if nav_row(
            ui,
            WorkbenchIcon::Models,
            &name,
            selected,
            Some(&count.to_string()),
        ) {
            app.state.model_library_manager.select_library(&name);
            app.state.workbench.models_page = ModelsPage::Catalog;
            app.state.workbench.selected_model = None;
        }
    }
}

fn netlist(ui: &mut Ui, app: &mut RSpiceApp) {
    section_header(ui, "Sources", None);
    let manual = app.state.workspace.netlist_source.is_some();
    let source_label = if manual {
        "Manual SPICE source"
    } else {
        "Generated netlist"
    };
    let _ = nav_row(ui, WorkbenchIcon::Netlist, source_label, true, None);
    section_header(ui, "Automation", None);
    if nav_row(
        ui,
        WorkbenchIcon::Console,
        "Command console",
        false,
        Some("interactive"),
    ) {
        app.state.workbench.console_visible = true;
        app.state.workbench.console_page = super::super::state::ConsolePage::Console;
    }
    if nav_row(
        ui,
        WorkbenchIcon::File,
        "Task log",
        false,
        Some(&app.state.simulation.runs.len().to_string()),
    ) {
        app.state.workbench.console_visible = true;
        app.state.workbench.console_page = super::super::state::ConsolePage::TaskLog;
    }
}

pub(super) fn nav_row(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let height = if meta.is_some() { 42.0 } else { 32.0 };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    if selected || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 17.0, rect.center().y),
            egui::vec2(15.0, 15.0),
        ),
        if selected {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        egui::pos2(
            rect.left() + 31.0,
            if meta.is_some() {
                rect.top() + 13.0
            } else {
                rect.center().y
            },
        ),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(
            tokens::FS_1,
            if selected {
                FontWeight::Medium
            } else {
                FontWeight::Regular
            },
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    if let Some(meta) = meta {
        ui.painter().text(
            egui::pos2(rect.left() + 31.0, rect.bottom() - 10.0),
            egui::Align2::LEFT_CENTER,
            meta,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    response.clicked()
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(12.0);
    ui.label(egui::RichText::new(text).color(t.color.text_faint));
}

const fn view_icon(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => WorkbenchIcon::Netlist,
        _ => WorkbenchIcon::File,
    }
}
