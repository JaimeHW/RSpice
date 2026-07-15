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
    let is_drawer = app.state.workbench.drawer == Some(super::super::state::Drawer::Navigator);
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(
                app.state
                    .workbench
                    .workspace
                    .navigator_title()
                    .to_ascii_uppercase(),
            )
            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
            .color(t.color.text),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if is_drawer
                && icon_button(
                    ui,
                    WorkbenchIcon::Close,
                    "Close navigator",
                    false,
                    egui::vec2(30.0, 30.0),
                )
                .clicked()
            {
                app.state.workbench.dismiss_navigator();
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
        "Project library",
        Some(app.state.workspace.project.display_name()),
    );
    ScrollArea::vertical().show(ui, |ui| {
        ui.horizontal(|ui| {
            for (page, label) in [
                (ProjectPage::Dashboard, "Library"),
                (ProjectPage::Configuration, "Configuration"),
                (ProjectPage::Recovery, "Recovery"),
            ] {
                if ui
                    .selectable_label(app.state.workbench.project_page == page, label)
                    .clicked()
                {
                    app.state.workbench.project_page = page;
                }
            }
        });
        section_header(
            ui,
            "Design libraries",
            Some(&app.state.library_manager.library_count().to_string()),
        );
        library_tree(ui, app, false);
        section_header(ui, "Project contracts", None);
        for page in [
            ProjectPage::Configuration,
            ProjectPage::Technology,
            ProjectPage::Dependencies,
        ] {
            if nav_row(
                ui,
                WorkbenchIcon::Sliders,
                page.label(),
                app.state.workbench.project_page == page,
                None,
            ) {
                app.state.workbench.project_page = page;
            }
        }
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
    let query = app.state.workbench.navigator_query.trim().to_lowercase();
    let active_run = app.state.simulation.active_run_idx;
    let active_analysis = app.state.simulation.active_analysis_idx;
    let runs = app
        .state
        .simulation
        .runs
        .iter()
        .enumerate()
        .filter_map(|(run_index, run)| {
            let analyses = run
                .analyses
                .iter()
                .enumerate()
                .filter_map(|(analysis_index, analysis)| {
                    let signals = analysis
                        .waveforms
                        .iter()
                        .enumerate()
                        .filter(|(_, waveform)| {
                            query.is_empty() || waveform.name.to_lowercase().contains(&query)
                        })
                        .map(|(waveform_index, waveform)| {
                            let unit = if waveform.name.trim_start().starts_with("I(") {
                                "A"
                            } else {
                                analysis.analysis_type.axis_info().3
                            };
                            ResultSignal {
                                waveform_index,
                                name: waveform.name.clone(),
                                color: waveform.color.clone(),
                                visible: waveform.visible,
                                value: waveform
                                    .y
                                    .iter()
                                    .rev()
                                    .copied()
                                    .find(|value| value.is_finite())
                                    .map(|value| crate::ui::plot::fmt_si(value, unit, 3)),
                            }
                        })
                        .collect::<Vec<_>>();
                    let matches_analysis = query.is_empty()
                        || analysis.label.to_lowercase().contains(&query)
                        || analysis
                            .analysis_type
                            .display_name()
                            .to_lowercase()
                            .contains(&query)
                        || !signals.is_empty();
                    matches_analysis.then(|| ResultAnalysis {
                        analysis_index,
                        label: analysis.label.clone(),
                        short_label: analysis.analysis_type.short_label(),
                        success: analysis.success,
                        signals,
                    })
                })
                .collect::<Vec<_>>();
            let matches_run = query.is_empty()
                || run.label.to_lowercase().contains(&query)
                || !analyses.is_empty();
            matches_run.then(|| ResultRun {
                run_index,
                id: run.id,
                label: run.label.clone(),
                success: run.success,
                analyses,
            })
        })
        .collect::<Vec<_>>();

    section_header(
        ui,
        "Datasets",
        Some(&format!("{} runs", app.state.simulation.runs.len())),
    );
    ScrollArea::vertical()
        .id_salt("workbench.results.navigator")
        .show(ui, |ui| {
            if runs.is_empty() {
                muted(
                    ui,
                    if app.state.simulation.runs.is_empty() {
                        "Run a simulation to create an immutable result dataset."
                    } else {
                        "No dataset, analysis, or signal matches this filter."
                    },
                );
                return;
            }
            for run in runs {
                let run_active = active_run == Some(run.run_index);
                egui::CollapsingHeader::new(format!("Run {} · {}", run.id, run.label))
                    .default_open(run_active)
                    .show(ui, |ui| {
                        if nav_row(
                            ui,
                            if run.success {
                                WorkbenchIcon::Success
                            } else {
                                WorkbenchIcon::Warning
                            },
                            "Run summary",
                            run_active && active_analysis.is_none(),
                            None,
                        ) {
                            app.state.simulation.select_run(run.run_index);
                        }
                        for analysis in run.analyses {
                            let analysis_active =
                                run_active && active_analysis == Some(analysis.analysis_index);
                            if nav_row(
                                ui,
                                if analysis.success {
                                    WorkbenchIcon::Results
                                } else {
                                    WorkbenchIcon::Warning
                                },
                                &analysis.label,
                                analysis_active,
                                Some(analysis.short_label),
                            ) {
                                if !run_active {
                                    app.state.simulation.select_run(run.run_index);
                                }
                                app.state
                                    .simulation
                                    .select_analysis(analysis.analysis_index);
                            }
                            if analysis_active {
                                for signal in analysis.signals {
                                    let t = Tokens::get(ui.ctx());
                                    let color = super::super::result_document::trace_color(
                                        &signal.color,
                                        t.color.traces
                                            [signal.waveform_index % t.color.traces.len()],
                                    );
                                    if signal_row(
                                        ui,
                                        &signal.name,
                                        signal.value.as_deref(),
                                        color,
                                        signal.visible,
                                    ) {
                                        super::super::result_document::toggle_visibility(
                                            &mut app.state,
                                            analysis.analysis_index,
                                            signal.waveform_index,
                                        );
                                    }
                                }
                            }
                        }
                    });
            }

            let Some(analysis_index) = app.state.simulation.active_analysis_idx else {
                return;
            };
            let expressions = app
                .state
                .ui
                .results
                .exprs
                .get(&analysis_index)
                .cloned()
                .unwrap_or_default();
            expression_header(ui, app);
            let mut toggled_expression = None;
            for (expression_index, expression) in expressions.iter().enumerate() {
                if !query.is_empty() && !expression.text.to_lowercase().contains(&query) {
                    continue;
                }
                let t = Tokens::get(ui.ctx());
                if signal_row(
                    ui,
                    &expression.text,
                    Some("expression"),
                    t.color.traces[expression_index % t.color.traces.len()],
                    expression.visible,
                ) {
                    toggled_expression = Some(expression_index);
                }
            }
            if let Some(expression_index) = toggled_expression
                && let Some(expression) = app
                    .state
                    .ui
                    .results
                    .exprs
                    .get_mut(&analysis_index)
                    .and_then(|expressions| expressions.get_mut(expression_index))
            {
                expression.visible = !expression.visible;
            }
        });
}

struct ResultRun {
    run_index: usize,
    id: u64,
    label: String,
    success: bool,
    analyses: Vec<ResultAnalysis>,
}

struct ResultAnalysis {
    analysis_index: usize,
    label: String,
    short_label: &'static str,
    success: bool,
    signals: Vec<ResultSignal>,
}

struct ResultSignal {
    waveform_index: usize,
    name: String,
    color: String,
    visible: bool,
    value: Option<String>,
}

fn expression_header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 28.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 12.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "EXPRESSIONS",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    let add_rect = egui::Rect::from_center_size(
        egui::pos2(rect.right() - 16.0, rect.center().y),
        egui::vec2(28.0, 28.0),
    );
    let response = ui.interact(
        add_rect,
        ui.id().with("add-result-expression"),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Open waveform calculator",
        )
    });
    if response.hovered() {
        ui.painter()
            .rect_filled(add_rect, t.radius, t.color.bg_hover);
    }
    WorkbenchIcon::Add.paint(ui.painter(), add_rect.shrink(7.0), t.color.text_dim);
    theme::paint_focus_ring(ui, &response, add_rect);
    if response.on_hover_text("Open calculator").clicked() {
        super::super::commands::Command::WaveformCalculator.execute(app);
    }
}

fn signal_row(
    ui: &mut Ui,
    name: &str,
    value: Option<&str>,
    color: egui::Color32,
    visible: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 30.0), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            visible,
            format!("{name} trace visibility"),
        )
    });
    if visible || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if visible {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    let swatch = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 19.0, rect.center().y),
        egui::vec2(12.0, 3.0),
    );
    ui.painter().rect_filled(
        swatch,
        1.0,
        if visible { color } else { t.color.text_faint },
    );
    ui.painter().text(
        egui::pos2(rect.left() + 31.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if visible {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    if let Some(value) = value {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            value,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
        .on_hover_text(if visible { "Hide trace" } else { "Show trace" })
        .clicked()
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
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
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
    theme::paint_focus_ring(ui, &response, rect);
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
