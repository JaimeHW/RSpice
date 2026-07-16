//! Workspace-aware navigation tree.

mod design;

use egui::{Align, Layout, ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::state::{CellViewRef, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::super::design_system::{PANEL_HEADER_H, WorkbenchIcon, property_row, section_header};
use super::super::state::{ModelsPage, ProjectPage, VerificationPage, Workspace};

const EXPRESSION_HEADER_HEIGHT: f32 = 28.0;
const SIGNAL_ROW_HEIGHT: f32 = 30.0;
const TOUCH_TARGET_HEIGHT: f32 = 44.0;
const PANEL_SEARCH_MARGIN_X: f32 = 8.0;
const EMPTY_HINT_PADDING_X: i8 = 12;
const EMPTY_HINT_PADDING_Y: i8 = 20;
// The mock's 49 px is a minimum. Its three grid rows resolve to 63 px at the
// reviewed 11 px type scale (title/detail row, 4 px gap, status row).
const FLOW_ROW_HEIGHT: f32 = 63.0;
const FLOW_TEXT_LEFT: f32 = 35.0;
const FLOW_LABEL_TOP: f32 = 7.0;
const FLOW_DETAIL_TOP: f32 = 23.0;
const FLOW_STATUS_TOP: f32 = 43.0;
const FLOW_DETAIL_LINE_HEIGHT: f32 = 15.0;
const FLOW_STATUS_DOT_SIZE: f32 = 5.0;
const FLOW_STATUS_GAP: f32 = 6.0;
const CAPABILITY_BANNER_MARGIN: i8 = 8;
const CAPABILITY_BANNER_ICON_SIZE: f32 = 15.0;
const CAPABILITY_BANNER_GAP: f32 = 7.0;

fn panel_search_field_width(available_width: f32) -> f32 {
    (available_width - PANEL_SEARCH_MARGIN_X * 2.0).max(1.0)
}

fn responsive_result_control_height(desktop_height: f32, control_height: f32) -> f32 {
    if control_height >= TOUCH_TARGET_HEIGHT {
        control_height.max(TOUCH_TARGET_HEIGHT)
    } else {
        desktop_height
    }
}

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
                Workspace::Verify => {
                    ScrollArea::vertical()
                        .id_salt("workbench.verify.navigator")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            verify(ui, app);
                        });
                }
                Workspace::Models => models(ui, app),
                Workspace::Netlist => netlist(ui, app),
                Workspace::Design => unreachable!("handled above"),
            }
        }
    }
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANEL_HEADER_H),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let title = match app.state.workbench.workspace {
        Workspace::Project => "Project library",
        Workspace::Design => "Design navigator",
        Workspace::Simulate => "Simulation Studio",
        Workspace::Results => "Data browser",
        Workspace::Verify => "Verification flows",
        Workspace::Models => "Library browser",
        Workspace::Netlist => "Netlist outline",
    };
    ui.painter().text(
        egui::pos2(rect.left() + 11.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title.to_ascii_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
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
    panel_search(
        ui,
        &mut app.state.workbench.navigator_query,
        "workbench.navigator.filter",
        placeholder,
        &mut app.state.workbench.focus_navigator_search,
    );
}

pub(super) fn panel_search(
    ui: &mut Ui,
    query: &mut String,
    id: &'static str,
    placeholder: &'static str,
    focus_pending: &mut bool,
) {
    let t = Tokens::get(ui.ctx());
    let field_width = panel_search_field_width(ui.available_width());
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(PANEL_SEARCH_MARGIN_X);
        let response = ui.add_sized(
            [field_width, t.metrics.ctl_h],
            egui::TextEdit::singleline(query)
                .id_salt(id)
                .hint_text(placeholder)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .margin(egui::Margin {
                    left: 29,
                    right: 8,
                    top: 5,
                    bottom: 5,
                }),
        );
        WorkbenchIcon::Search.paint(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
                egui::vec2(16.0, 16.0),
            ),
            t.color.text_faint,
        );
        if std::mem::take(focus_pending) {
            response.request_focus();
        }
    });
    ui.add_space(8.0);
}

fn project(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical().show(ui, |ui| {
        section_header(
            ui,
            "Design libraries",
            Some(&app.state.library_manager.library_count().to_string()),
        );
        library_tree(ui, app, true);
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
        let _ = nav_row_indented(
            ui,
            WorkbenchIcon::Folder,
            &library,
            library == app.state.workspace.active_view.library,
            read_only.then_some("read-only"),
            0,
        );
        for (cell, views) in cells {
            let _ = nav_row_indented(
                ui,
                WorkbenchIcon::Project,
                &cell,
                library == app.state.workspace.active_view.library
                    && cell == app.state.workspace.active_view.cell,
                Some(&views.len().to_string()),
                1,
            );
            for (view, view_type) in views {
                let reference = CellViewRef::new(&library, &cell, &view);
                let active = reference == app.state.workspace.active_view;
                if nav_row_indented(
                    ui,
                    view_icon(view_type),
                    &view,
                    active,
                    Some(view_type.display_name()),
                    2,
                ) && open_documents
                {
                    requested = Some(reference);
                }
            }
        }
    }
    if let Some(reference) = requested {
        app.state.open_workspace_view(reference);
    }
}

fn simulate(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::simulation::plan::AnalysisKind;

    let enabled = app.state.sim_setup.enabled_analysis_instance_count();
    let total = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_or(0, |plan| plan.instances().len());
    let query = app.state.workbench.navigator_query.trim().to_lowercase();
    section_header(ui, "Lab characterization", Some(&format!("{enabled} on")));
    ScrollArea::vertical()
        .id_salt("workbench.simulation.navigator")
        .show(ui, |ui| {
            if query.is_empty() || "analyses".contains(&query) {
                let meta = format!("{total} active · {enabled} enabled");
                let _ = nav_row(ui, WorkbenchIcon::Results, "Analyses", true, Some(&meta));
            }
            section_header(ui, "Run set", Some("Reference point"));
            nav_property(
                ui,
                "Process",
                app.state.sim_setup.reference_pvt.process.short_name(),
            );
            nav_property(
                ui,
                "Temperature",
                &format!(
                    "{} °C",
                    app.state.sim_setup.reference_pvt.temperature_celsius
                ),
            );
            nav_property(
                ui,
                "Variation",
                if app
                    .state
                    .sim_setup
                    .has_enabled_analysis_kind(AnalysisKind::MonteCarlo)
                {
                    "enabled"
                } else {
                    "disabled"
                },
            );
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                if Button::new("Add analysis…")
                    .accent()
                    .min_width((ui.available_width() - 16.0).max(1.0))
                    .show(ui)
                    .clicked()
                {
                    // On tablet and phone this action is hosted by the
                    // navigator drawer. Close that transient layer before
                    // opening the catalog so the modal is never obscured by
                    // its invoker.
                    app.state.workbench.close_drawer();
                    app.state.sim_setup.palette_open = true;
                    app.state.sim_setup.palette_query.clear();
                    app.state.sim_setup.palette_active = 0;
                    app.state.sim_setup.palette_scroll_to_active = true;
                }
            });
            section_header(ui, "Capability policy", None);
            capability_policy_banner(ui);
        });
}

fn capability_policy_banner(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let response = egui::Frame::new()
        .fill(t.color.bg_inset)
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::same(CAPABILITY_BANNER_MARGIN))
        .outer_margin(egui::Margin::same(CAPABILITY_BANNER_MARGIN))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = CAPABILITY_BANNER_GAP;
                let (icon_rect, _) = ui.allocate_exact_size(
                    egui::Vec2::splat(CAPABILITY_BANNER_ICON_SIZE),
                    egui::Sense::hover(),
                );
                WorkbenchIcon::Info.paint(ui.painter(), icon_rect, t.color.text_dim);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(
                            "Every analysis declares release-engine, preview, compatibility, platform and sign-off contracts before it can be added.",
                        )
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            });
        })
        .response;
    let border_rect = response.rect.shrink(f32::from(CAPABILITY_BANNER_MARGIN));
    let outline = [
        border_rect.left_top(),
        border_rect.right_top(),
        border_rect.right_bottom(),
        border_rect.left_bottom(),
        border_rect.left_top(),
    ];
    ui.painter().add(egui::Shape::dashed_line(
        &outline,
        egui::Stroke::new(1.0, t.color.border_strong),
        3.0,
        3.0,
    ));
}

fn nav_property(ui: &mut Ui, label: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(t.color.text),
            );
        });
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
                let run_meta = format!("{} analyses", run.analyses.len());
                if nav_row_indented(
                    ui,
                    if run.success {
                        WorkbenchIcon::Success
                    } else {
                        WorkbenchIcon::Warning
                    },
                    &run.label,
                    run_active && active_analysis.is_none(),
                    Some(&run_meta),
                    0,
                ) {
                    app.state.simulation.select_run(run.run_index);
                    app.state.simulation.active_analysis_idx = None;
                }
                for analysis in run.analyses {
                    let analysis_active =
                        run_active && active_analysis == Some(analysis.analysis_index);
                    if nav_row_indented(
                        ui,
                        if analysis.success {
                            WorkbenchIcon::Results
                        } else {
                            WorkbenchIcon::Warning
                        },
                        &analysis.label,
                        analysis_active,
                        Some(analysis.short_label),
                        1,
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
                                t.color.traces[signal.waveform_index % t.color.traces.len()],
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
    let header_height = responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, t.metrics.ctl_h);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), header_height),
        egui::Sense::hover(),
    );
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
    let add_size = responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, t.metrics.ctl_h);
    let add_center_x = if add_size >= TOUCH_TARGET_HEIGHT {
        rect.right() - 10.0 - add_size * 0.5
    } else {
        rect.right() - 16.0
    };
    let add_rect = egui::Rect::from_center_size(
        egui::pos2(add_center_x, rect.center().y),
        egui::vec2(add_size, add_size),
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
    WorkbenchIcon::Add.paint(
        ui.painter(),
        egui::Rect::from_center_size(add_rect.center(), egui::vec2(14.0, 14.0)),
        t.color.text_dim,
    );
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
    let row_height = responsive_result_control_height(SIGNAL_ROW_HEIGHT, t.metrics.ctl_h);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::click(),
    );
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
    theme::paint_focus_ring_outset(ui, &response, rect);
    response
        .on_hover_text(if visible { "Hide trace" } else { "Show trace" })
        .clicked()
}

fn verify(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_query
        .trim()
        .to_ascii_lowercase();
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
    for page in VerificationPage::ALL {
        let (label, detail, status, tone) = match page {
            VerificationPage::Cockpit => {
                let result = active_dataset_yield_results(&app.state.simulation)
                    .iter()
                    .min_by(|a, b| {
                        a.yield_percent
                            .partial_cmp(&b.yield_percent)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                (
                    "PVT & Monte Carlo".to_owned(),
                    result.map_or_else(
                        || "No retained Monte Carlo evidence for active dataset".to_owned(),
                        |result| format!("{} samples · {}", result.total_runs, result.spec.target),
                    ),
                    result.map_or_else(
                        || "not run".to_owned(),
                        |result| format!("{:.2}% yield", result.yield_percent),
                    ),
                    if result.is_some_and(|result| result.fail_count == 0) {
                        FlowTone::Ok
                    } else {
                        FlowTone::Warn
                    },
                )
            }
            VerificationPage::Specifications => {
                let active = app
                    .state
                    .simulation
                    .active_run()
                    .or_else(|| app.state.simulation.runs.first());
                let covered = app
                    .state
                    .workspace
                    .specs
                    .iter()
                    .filter(|spec| {
                        active.is_some_and(|run| {
                            run.analyses.iter().any(|analysis| {
                                analysis.measurements.iter().any(|measurement| {
                                    measurement.name.eq_ignore_ascii_case(&spec.measurement)
                                        && measurement.value.is_some()
                                })
                            })
                        })
                    })
                    .count();
                let total = app.state.workspace.specs.len();
                (
                    "Specification evidence".to_owned(),
                    format!("{total} tracked measurement limits"),
                    format!("{covered} / {total} covered"),
                    if total > 0 && covered == total {
                        FlowTone::Ok
                    } else {
                        FlowTone::Warn
                    },
                )
            }
            VerificationPage::Checks => {
                let result = app.state.dialogs.drc_results.as_ref();
                let errors = result.map_or(0, |result| {
                    let summary = result.summary();
                    summary.critical + summary.errors
                });
                (
                    "Design checks".to_owned(),
                    "Schematic connectivity and electrical-rule checks".to_owned(),
                    result.map_or_else(
                        || "not run".to_owned(),
                        |result| format!("{} markers", result.total_count()),
                    ),
                    if result.is_some() && errors == 0 {
                        FlowTone::Ok
                    } else {
                        FlowTone::Warn
                    },
                )
            }
            VerificationPage::Reliability => {
                let violations = app.state.simulation.soa_violations.len();
                let aging = app.state.simulation.reliability_results.len();
                (
                    "Electrical reliability & SOA".to_owned(),
                    format!("{aging} aging results · {violations} SOA violations"),
                    if aging == 0 && violations == 0 {
                        "not run".to_owned()
                    } else if violations == 0 {
                        "SOA pass".to_owned()
                    } else {
                        format!("{violations} violations")
                    },
                    if violations == 0 && aging > 0 {
                        FlowTone::Ok
                    } else {
                        FlowTone::Warn
                    },
                )
            }
            VerificationPage::History => {
                let runs = app.state.simulation.runs.len();
                (
                    "Retained run history".to_owned(),
                    "Immutable datasets available for review".to_owned(),
                    format!("{runs} runs"),
                    if runs > 0 {
                        FlowTone::Accent
                    } else {
                        FlowTone::Neutral
                    },
                )
            }
        };
        if !query.is_empty()
            && !format!("{label} {detail} {status}")
                .to_ascii_lowercase()
                .contains(&query)
        {
            continue;
        }
        if flow_row(
            ui,
            &label,
            &detail,
            &status,
            tone,
            app.state.workbench.verification_page == page,
        ) {
            app.state.workbench.verification_page = page;
        }
    }
    section_header(ui, "Active evidence coverage", None);
    property_row(
        ui,
        "Specifications",
        &app.state.workspace.specs.len().to_string(),
    );
    property_row(
        ui,
        "Retained datasets",
        &app.state.simulation.runs.len().to_string(),
    );
    let executable = active_mc_sample_trail(&app.state.simulation);
    property_row(ui, "MC sample trail", &executable.to_string());
    ui.painter().hline(
        ui.max_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

fn active_dataset_yield_results(
    simulation: &crate::state::SimulationState,
) -> &[crate::services::yield_manager::YieldResult] {
    simulation.yield_results_for_active_dataset().unwrap_or(&[])
}

fn active_mc_sample_trail(simulation: &crate::state::SimulationState) -> usize {
    active_dataset_yield_results(simulation)
        .iter()
        .map(|result| result.trail.len())
        .max()
        .unwrap_or(0)
}

#[derive(Clone, Copy)]
enum FlowTone {
    Neutral,
    Accent,
    Ok,
    Warn,
}

fn flow_row_geometry(detail_lines: usize) -> (f32, f32) {
    let extra_detail_height = detail_lines.saturating_sub(1) as f32 * FLOW_DETAIL_LINE_HEIGHT;
    (
        FLOW_ROW_HEIGHT + extra_detail_height,
        FLOW_STATUS_TOP + extra_detail_height,
    )
}

fn flow_row(
    ui: &mut Ui,
    label: &str,
    detail: &str,
    status: &str,
    tone: FlowTone,
    selected: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let text_width = (ui.available_width() - FLOW_TEXT_LEFT - 9.0).max(1.0);
    let detail_galley = ui.painter().layout(
        detail.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        text_width,
    );
    let (row_height, status_top) = flow_row_geometry(detail_galley.rows.len().max(1));
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_height),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("{label}. {detail}. {status}"),
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
            egui::Rect::from_min_max(rect.min, egui::pos2(rect.left() + 2.0, rect.bottom())),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let (icon_ink, glyph, status_dot, status_ink) = match tone {
        FlowTone::Neutral => (
            t.color.text_faint,
            "·",
            t.color.text_faint,
            t.color.text_dim,
        ),
        FlowTone::Accent => (t.color.accent, "T", t.color.accent, t.color.text_dim),
        FlowTone::Ok => (t.color.ok, "✓", t.color.ok, t.color.text_dim),
        FlowTone::Warn => (t.color.warn, "△", t.color.warn, t.color.warn),
    };
    let status_circle = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 17.5, rect.top() + 16.5),
        egui::vec2(17.0, 17.0),
    );
    let circle_fill = match tone {
        FlowTone::Ok => t.color.ok.gamma_multiply(0.14),
        FlowTone::Warn => t.color.warn.gamma_multiply(0.14),
        FlowTone::Accent => t.color.accent.gamma_multiply(0.14),
        FlowTone::Neutral => t.color.bg_panel_2,
    };
    let circle_border = match tone {
        FlowTone::Ok | FlowTone::Warn | FlowTone::Accent => icon_ink.gamma_multiply(0.7),
        FlowTone::Neutral => icon_ink,
    };
    ui.painter()
        .circle_filled(status_circle.center(), 8.5, circle_fill);
    ui.painter().circle_stroke(
        status_circle.center(),
        8.0,
        egui::Stroke::new(1.0, circle_border),
    );
    ui.painter().text(
        status_circle.center(),
        egui::Align2::CENTER_CENTER,
        glyph,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        icon_ink,
    );
    let text_left = rect.left() + FLOW_TEXT_LEFT;
    let text_right = rect.right() - 9.0;
    let clip = egui::Rect::from_x_y_ranges(text_left..=text_right, rect.y_range());
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(text_left, rect.top() + FLOW_LABEL_TOP),
        egui::Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    painter.galley(
        egui::pos2(text_left, rect.top() + FLOW_DETAIL_TOP),
        detail_galley,
        t.color.text_faint,
    );
    let status_dot_rect = egui::Rect::from_min_size(
        egui::pos2(text_left, rect.top() + status_top + 4.0),
        egui::Vec2::splat(FLOW_STATUS_DOT_SIZE),
    );
    painter.circle_filled(
        status_dot_rect.center(),
        FLOW_STATUS_DOT_SIZE * 0.5,
        status_dot,
    );
    painter.text(
        egui::pos2(
            text_left + FLOW_STATUS_DOT_SIZE + FLOW_STATUS_GAP,
            rect.top() + status_top,
        ),
        egui::Align2::LEFT_TOP,
        status,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        status_ink,
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.clicked()
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
            app.state.workbench.models_page = ModelsPage::Models;
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
    nav_row_indented(ui, icon, label, selected, meta, 0)
}

pub(super) fn nav_row_indented(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> bool {
    nav_row_indented_styled(ui, icon, label, selected, meta, level, false)
}

pub(super) fn nav_row_indented_mono(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> bool {
    nav_row_indented_styled(ui, icon, label, selected, meta, level, true)
}

fn nav_row_indented_styled(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
    mono: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.row_h;
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
    let indent = 14.0 * level as f32;
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 31.0 + indent, rect.center().y),
            egui::vec2(15.0, 15.0),
        ),
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let meta_width = meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(
                meta.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x
    });
    let label_left = rect.left() + 45.0 + indent;
    let label_right = if meta.is_some() {
        rect.right() - 14.0 - meta_width
    } else {
        rect.right() - 8.0
    };
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            if mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
    if let Some(meta) = meta {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
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
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: EMPTY_HINT_PADDING_X,
            right: EMPTY_HINT_PADDING_X,
            top: EMPTY_HINT_PADDING_Y,
            bottom: EMPTY_HINT_PADDING_Y,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                )
                .wrap()
                .halign(Align::Center),
            );
        });
}

const fn view_icon(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => WorkbenchIcon::Netlist,
        _ => WorkbenchIcon::File,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CAPABILITY_BANNER_GAP, CAPABILITY_BANNER_ICON_SIZE, CAPABILITY_BANNER_MARGIN,
        EMPTY_HINT_PADDING_X, EMPTY_HINT_PADDING_Y, EXPRESSION_HEADER_HEIGHT, FLOW_DETAIL_TOP,
        FLOW_LABEL_TOP, FLOW_ROW_HEIGHT, FLOW_STATUS_TOP, FLOW_TEXT_LEFT, PANEL_SEARCH_MARGIN_X,
        SIGNAL_ROW_HEIGHT, TOUCH_TARGET_HEIGHT, active_mc_sample_trail, flow_row_geometry,
        panel_search_field_width, responsive_result_control_height,
    };
    use crate::services::{
        DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
    };
    use crate::state::{SimulationRun, SimulationState};

    fn result(trail: Vec<bool>) -> YieldResult {
        let pass_count = trail.iter().filter(|passes| **passes).count();
        YieldResult {
            spec: YieldSpec::lower("gain", 0.0, ""),
            total_runs: trail.len(),
            pass_count,
            fail_count: trail.len() - pass_count,
            yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
            stats: DistributionStats::default(),
            samples: vec![1.0; trail.len()],
            trail,
        }
    }

    #[test]
    fn navigator_search_and_empty_hint_match_mockup_spacing() {
        assert_eq!(PANEL_SEARCH_MARGIN_X, 8.0);
        assert_eq!(panel_search_field_width(260.0), 244.0);
        assert_eq!(EMPTY_HINT_PADDING_X, 12);
        assert_eq!(EMPTY_HINT_PADDING_Y, 20);
    }

    #[test]
    fn verification_flow_rows_match_the_mock_grid_without_text_collisions() {
        assert_eq!(FLOW_ROW_HEIGHT, 63.0);
        assert_eq!(FLOW_TEXT_LEFT, 35.0);
        assert_eq!(FLOW_LABEL_TOP, 7.0);
        assert_eq!(FLOW_DETAIL_TOP, 23.0);
        assert_eq!(FLOW_STATUS_TOP, 43.0);
        assert!(FLOW_DETAIL_TOP - (FLOW_LABEL_TOP + 11.0) >= 4.0);
        assert!(FLOW_STATUS_TOP - (FLOW_DETAIL_TOP + 11.0) >= 4.0);
        assert!(FLOW_ROW_HEIGHT - (FLOW_STATUS_TOP + 11.0) >= 7.0);
        assert_eq!(flow_row_geometry(1), (63.0, 43.0));
        assert_eq!(flow_row_geometry(2), (78.0, 58.0));
        assert_eq!(flow_row_geometry(3), (93.0, 73.0));
    }

    #[test]
    fn capability_policy_banner_matches_mock_spacing() {
        assert_eq!(CAPABILITY_BANNER_MARGIN, 8);
        assert_eq!(CAPABILITY_BANNER_ICON_SIZE, 15.0);
        assert_eq!(CAPABILITY_BANNER_GAP, 7.0);
    }

    #[test]
    fn mc_sample_trail_is_visible_only_for_its_active_provenance_dataset() {
        let source = SimulationRun::new(1);
        let source_run_id = source.run_id;
        let source_dataset_id = source.dataset_id;
        let other = SimulationRun::new(2);
        let mut simulation = SimulationState {
            runs: vec![source, other],
            active_run_idx: Some(0),
            ..SimulationState::default()
        };
        simulation.replace_yield_evidence(
            vec![result(vec![true, false, true])],
            Some(YieldAnalysisProvenance {
                source_run_id,
                source_dataset_id,
                seed: 7,
                runs_requested: 3,
                runs_completed: 3,
                sampling_mode: MonteCarloSamplingMode::PseudoRandom,
            }),
        );

        assert_eq!(active_mc_sample_trail(&simulation), 3);
        simulation.active_run_idx = Some(1);
        assert_eq!(active_mc_sample_trail(&simulation), 0);
        simulation.active_run_idx = None;
        assert_eq!(active_mc_sample_trail(&simulation), 0);
    }

    #[test]
    fn result_navigator_preserves_desktop_density_and_expands_touch_controls() {
        assert_eq!(
            responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, 32.0),
            EXPRESSION_HEADER_HEIGHT
        );
        assert_eq!(
            responsive_result_control_height(SIGNAL_ROW_HEIGHT, 32.0),
            SIGNAL_ROW_HEIGHT
        );
        assert_eq!(
            responsive_result_control_height(EXPRESSION_HEADER_HEIGHT, TOUCH_TARGET_HEIGHT),
            TOUCH_TARGET_HEIGHT
        );
        assert_eq!(
            responsive_result_control_height(SIGNAL_ROW_HEIGHT, TOUCH_TARGET_HEIGHT),
            TOUCH_TARGET_HEIGHT
        );
    }
}
