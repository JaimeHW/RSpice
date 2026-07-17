//! Model catalog, symbol contracts, PDK sections, authenticated includes, and
//! a truthful audit of the metadata currently retained for loaded models.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use egui::{Align, Align2, Color32, Key, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::common::RSpiceApp;
use crate::state::model_library::{DeviceModel, ModelLibrary};
use crate::state::{CellViewRef, SymbolDocument, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::super::design_system::{heading, property_card, property_row, workspace_title_row};
use super::super::state::{ModelsPage, Workspace};

const TABLE_HEAD_H: f32 = 27.0;
const MODEL_TABLE_MIN_W: f32 = 780.0;
const MODEL_PHONE_TABLE_MIN_W: f32 = 690.0;
const GENERAL_TABLE_MIN_W: f32 = 760.0;
const MODEL_PHONE_BREAKPOINT: f32 = 560.0;
const MODEL_SUMMARY_BREAKPOINT: f32 = 820.0;
const MODEL_TABLE_MIN_H: f32 = 120.0;
const MODEL_WIDE_SUMMARY_H: f32 = 150.0;
const MODEL_STACKED_SUMMARY_H: f32 = 300.0;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        model_tabs(ui, app);
        match app.state.workbench.models_page {
            ModelsPage::Models => models_catalog(ui, app),
            ModelsPage::Symbols => symbols(ui, app),
            ModelsPage::Corners => corners(ui, app),
            ModelsPage::Include => include_graph(ui, app),
            ModelsPage::Qualification => metadata_audit(ui, app),
        }
    });
}

fn model_tabs(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let touch = t.metrics.ctl_h >= 44.0;
    let tab_h = if touch { 44.0 } else { 37.0 };
    let filter_visible = app.state.workbench.models_page == ModelsPage::Models;
    let strip_h = model_tab_strip_height(touch, filter_visible);
    let surface_w = ui.available_width().max(1.0);
    let filter_outer_w = if filter_visible {
        if surface_w <= 560.0 {
            (surface_w * 0.44).clamp(126.0, 190.0) + 8.0
        } else {
            234.0
        }
    } else {
        0.0
    };
    let (strip_rect, _) = ui.allocate_exact_size(egui::vec2(surface_w, strip_h), Sense::hover());
    ui.painter().rect_filled(strip_rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        strip_rect.x_range(),
        strip_rect.top(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().hline(
        strip_rect.x_range(),
        strip_rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );

    let tabs_rect = Rect::from_min_max(
        strip_rect.min,
        egui::pos2(
            (strip_rect.right() - filter_outer_w).max(strip_rect.left()),
            strip_rect.bottom(),
        ),
    );
    let mut selected = None;
    let mut tabs = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(tabs_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    tabs.spacing_mut().item_spacing = Vec2::ZERO;
    ScrollArea::horizontal()
        .id_salt("workbench.models.tabs")
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
        .show(&mut tabs, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                for page in ModelsPage::ALL {
                    let label = page.label();
                    let galley = ui.painter().layout_no_wrap(
                        label.to_owned(),
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_dim,
                    );
                    let width = (galley.size().x + 24.0).max(if touch { 44.0 } else { 0.0 });
                    let (rect, response) =
                        ui.allocate_exact_size(egui::vec2(width, tab_h), Sense::click());
                    let active = app.state.workbench.models_page == page;
                    response.widget_info(|| {
                        egui::WidgetInfo::selected(
                            egui::WidgetType::Button,
                            ui.is_enabled(),
                            active,
                            label,
                        )
                    });
                    if response.hovered() {
                        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                    }
                    ui.painter().galley(
                        egui::pos2(
                            rect.center().x - galley.size().x * 0.5,
                            rect.center().y - galley.size().y * 0.5,
                        ),
                        galley,
                        if active {
                            t.color.text
                        } else {
                            t.color.text_dim
                        },
                    );
                    if active {
                        ui.painter().rect_filled(
                            Rect::from_min_max(
                                egui::pos2(rect.left() + 9.0, rect.bottom() - 2.0),
                                egui::pos2(rect.right() - 9.0, rect.bottom()),
                            ),
                            0.0,
                            t.color.accent,
                        );
                    }
                    theme::paint_focus_ring(ui, &response, rect);
                    if response.clicked() {
                        selected = Some(page);
                    }
                }
            });
        });

    if filter_visible {
        let margin_x = if surface_w <= 560.0 { 4.0 } else { 7.0 };
        let filter_rect = Rect::from_min_max(
            egui::pos2(
                strip_rect.right() - filter_outer_w + margin_x,
                strip_rect.top() + 5.0,
            ),
            egui::pos2(strip_rect.right() - margin_x, strip_rect.bottom() - 5.0),
        );
        ui.painter().rect(
            filter_rect,
            t.radius,
            t.color.bg_inset,
            Stroke::new(1.0, t.color.border),
            egui::StrokeKind::Inside,
        );
        let icon_rect = Rect::from_center_size(
            egui::pos2(filter_rect.left() + 14.0, filter_rect.center().y),
            Vec2::splat(14.0),
        );
        super::super::design_system::WorkbenchIcon::Search.paint(
            ui.painter(),
            icon_rect,
            t.color.text_faint,
        );
        let edit_rect = Rect::from_min_max(
            egui::pos2(filter_rect.left() + 27.0, filter_rect.top() + 1.0),
            egui::pos2(filter_rect.right() - 5.0, filter_rect.bottom() - 1.0),
        );
        let mut edit = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(edit_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        let count = app.state.model_library_manager.total_model_count();
        edit.add_sized(
            edit_rect.size(),
            egui::TextEdit::singleline(&mut app.state.model_library_manager.filter_text)
                .frame(egui::Frame::NONE)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .hint_text(format!("Filter {count} models…")),
        );
    }

    if let Some(page) = selected {
        app.state.workbench.models_page = page;
    }
    ui.add_space(1.0);
}

fn models_catalog(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let query = app.state.model_library_manager.filter_text.clone();
    let rows = app
        .state
        .model_library_manager
        .search_models(&query)
        .into_iter()
        .map(|(library, model)| {
            let selected = app.state.workbench.selected_model.as_deref() == Some(&model.name)
                && app.state.model_library_manager.selected_library.as_deref()
                    == Some(&library.name);
            let geometry_invalid = model_geometry_invalid(model);
            let externally_unpinned =
                library.root_path.is_some() && library.source_closure.is_empty();
            let undocumented = model.description.trim().is_empty();
            let (status, status_color) = if geometry_invalid {
                ("geometry review", t.color.err)
            } else if externally_unpinned {
                ("source unpinned", t.color.err)
            } else if undocumented {
                ("documentation", t.color.warn)
            } else if library.root_path.is_some() {
                ("source pinned", t.color.ok)
            } else {
                ("in memory", t.color.info)
            };
            DataRow {
                key: model_key(&library.name, &model.name),
                selected,
                cells: vec![
                    DataCell::mono(&model.name),
                    DataCell::plain(format!(
                        "{} · {}",
                        model.model_type.display_name(),
                        model.level.display_name()
                    )),
                    DataCell::mono(model_source_label(library, model)),
                    DataCell::plain(&library.name),
                    DataCell::plain(model_sections_or_runtime(library, model)),
                    DataCell::mono_colored("not recorded", t.color.text_faint),
                    DataCell::mono_colored(status, status_color),
                ],
            }
        })
        .collect::<Vec<_>>();

    let columns = [
        ("Model", 0.15),
        ("Family", 0.17),
        ("Source", 0.17),
        ("Library", 0.14),
        ("Sections / runtime", 0.18),
        ("Tests", 0.10),
        ("Status", 0.09),
    ];
    if let Some(event) = data_table(
        ui,
        "models.catalog",
        model_catalog_min_width(ui.available_width()),
        &columns,
        &rows,
        ui.available_size(),
        "No models match the active filter.",
    ) {
        let (library, model) = split_model_key(&event.key);
        app.state.model_library_manager.select_library(library);
        app.state.workbench.selected_model = Some(model.to_owned());
    }
}

fn symbols(ui: &mut Ui, app: &mut RSpiceApp) {
    surface_title(
        ui,
        "Symbols, pins & device forms",
        "Symbol and component-definition manager",
        "Bind graphical symbols and explicit terminal contracts without hiding netlist semantics.",
        true,
        |ui| {
            if Button::new("Create symbol").accent().show(ui).clicked() {
                open_create_symbol_dialog(app);
            }
        },
    );

    let t = Tokens::get(ui.ctx());
    let mut targets = HashMap::<String, CellViewRef>::new();
    let mut stats = SymbolStats::default();
    let mut rows = Vec::new();
    for library in app.state.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            for view in cell
                .views_sorted()
                .into_iter()
                .filter(|view| view.view_type == ViewType::Symbol)
            {
                stats.symbols += 1;
                let key = view_key(&library.name, &cell.name, &view.name);
                targets.insert(
                    key.clone(),
                    CellViewRef::new(&library.name, &cell.name, &view.name),
                );
                let model_family = symbol_model_family(app, cell);
                let parameter_form = metadata_value(
                    [&cell.metadata, &view.metadata],
                    &["parameter.form", "parameter_form", "cdf", "cdf.form"],
                )
                .unwrap_or_else(|| "not defined".to_owned());
                if parameter_form != "not defined" {
                    stats.parameter_forms += 1;
                }
                let netlist_template = metadata_value(
                    [&cell.metadata, &view.metadata],
                    &["netlist.template", "netlist_template"],
                )
                .or_else(|| {
                    cell.views_sorted()
                        .into_iter()
                        .find(|candidate| candidate.view_type == ViewType::Spice)
                        .map(|candidate| candidate.name.clone())
                })
                .unwrap_or_else(|| "not defined".to_owned());

                let (pins, status, tone) = match SymbolDocument::load_from_view(view) {
                    Ok(document) if document.pins.is_empty() => {
                        stats.missing_contracts += 1;
                        (
                            "not defined".to_owned(),
                            "pin contract missing",
                            t.color.warn,
                        )
                    }
                    Ok(document) => {
                        stats.pins += document.pins.len();
                        let unplaced = document
                            .pins
                            .iter()
                            .filter(|pin| pin.position.is_none())
                            .count();
                        stats.unplaced_pins += unplaced;
                        let names = document
                            .pins
                            .iter()
                            .map(|pin| pin.name.as_str())
                            .collect::<Vec<_>>()
                            .join(" ");
                        if unplaced > 0 {
                            (names, "unplaced pins", t.color.warn)
                        } else if model_family == "unbound" {
                            (names, "model unbound", t.color.warn)
                        } else {
                            (names, "bound", t.color.ok)
                        }
                    }
                    Err(_) => {
                        stats.invalid_documents += 1;
                        ("invalid metadata".to_owned(), "invalid", t.color.err)
                    }
                };
                let selected = app.state.library_manager.selected_library.as_deref()
                    == Some(&library.name)
                    && app.state.library_manager.selected_cell.as_deref() == Some(&cell.name)
                    && app.state.library_manager.selected_view.as_deref() == Some(&view.name);
                rows.push(DataRow {
                    key,
                    selected,
                    cells: vec![
                        DataCell::mono(format!("{}/{}", cell.name, view.name)),
                        DataCell::plain(model_family),
                        DataCell::mono(pins),
                        DataCell::plain(parameter_form),
                        DataCell::mono(netlist_template),
                        DataCell::mono_colored(status, tone),
                    ],
                });
            }
        }
    }

    let available = ui.available_size();
    let layout = model_table_summary_layout(available, t.metrics.ctl_h >= 44.0);
    let columns = [
        ("Symbol", 0.20),
        ("Model family", 0.17),
        ("Pins", 0.20),
        ("Parameter form", 0.17),
        ("Netlist template", 0.17),
        ("Status", 0.09),
    ];
    let event = table_summary_composition(
        ui,
        "models.symbols.composition",
        available,
        layout,
        |ui, table_h| {
            let event = data_table(
                ui,
                "models.symbols",
                GENERAL_TABLE_MIN_W,
                &columns,
                &rows,
                egui::vec2(ui.available_width(), table_h),
                "No symbol views are present in the loaded design libraries.",
            );
            symbol_summary(ui, stats, layout.narrow);
            event
        },
    );
    if let Some(event) = event
        && let Some(reference) = targets.get(&event.key).cloned()
    {
        app.state
            .library_manager
            .select_view(&reference.library, &reference.cell, &reference.view);
        if event.activate {
            app.state.open_workspace_view(reference);
            app.state.workbench.activate(Workspace::Design);
        }
    }
}

fn corners(ui: &mut Ui, app: &mut RSpiceApp) {
    let quantity_policy = app.state.ui.preferences.quantity_presentation_policy();
    let corner_count = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| library.corners.keys())
        .map(|name| name.to_ascii_lowercase())
        .collect::<BTreeSet<_>>()
        .len();
    surface_title(
        ui,
        &format!("PDK sections · {corner_count} process corners"),
        "Corner and section binding",
        "Inspect exact process bindings and source availability before simulation task expansion.",
        false,
        |_| {},
    );

    let t = Tokens::get(ui.ctx());
    let mut rows = Vec::new();
    let mut targets = HashMap::<String, (String, String)>::new();
    let mut temperatures = BTreeSet::<String>::new();
    let mut supplies = BTreeSet::<String>::new();
    let mut resolved = 0usize;
    let mut unresolved = 0usize;
    for library in app.state.model_library_manager.libraries_sorted() {
        let mut corners = library.corners.values().collect::<Vec<_>>();
        corners.sort_by(|left, right| left.name.cmp(&right.name));
        for corner in corners {
            temperatures.insert(format!("{:.6}", corner.temperature));
            supplies.insert(format!("{:.6}", corner.vdd_factor));
            let resolved_source = corner.file_path.is_some()
                || (corner.name.eq_ignore_ascii_case("tt") && library.root_path.is_none());
            if resolved_source {
                resolved += 1;
            } else {
                unresolved += 1;
            }
            let key = model_key(&library.name, &corner.name);
            targets.insert(key.clone(), (library.name.clone(), corner.name.clone()));
            let selected = app.state.model_library_manager.selected_library.as_deref()
                == Some(&library.name)
                && library.selected_corner.as_deref() == Some(&corner.name);
            rows.push(DataRow {
                key,
                selected,
                cells: vec![
                    DataCell::mono(corner.name.to_uppercase()),
                    DataCell::mono(&corner.nmos_corner),
                    DataCell::mono(&corner.pmos_corner),
                    DataCell::mono(
                        corner
                            .file_path
                            .as_deref()
                            .and_then(Path::file_name)
                            .and_then(|name| name.to_str())
                            .unwrap_or("not bound"),
                    ),
                    DataCell::mono(
                        quantity_policy.format_temperature(corner.temperature + 273.15, 3),
                    ),
                    DataCell::mono(format!("{:.6}", corner.vdd_factor)),
                    DataCell::mono_colored(
                        if resolved_source {
                            "resolved"
                        } else {
                            "unresolved"
                        },
                        if resolved_source {
                            t.color.ok
                        } else {
                            t.color.err
                        },
                    ),
                ],
            });
        }
    }

    let available = ui.available_size();
    let layout = model_table_summary_layout(available, t.metrics.ctl_h >= 44.0);
    let columns = [
        ("Corner", 0.11),
        ("NMOS section", 0.16),
        ("PMOS section", 0.16),
        ("Model source", 0.20),
        ("Temperature", 0.14),
        ("Supply factor", 0.13),
        ("Status", 0.10),
    ];
    let temperature_axis = if temperatures.is_empty() {
        "not declared".to_owned()
    } else {
        temperatures
            .iter()
            .filter_map(|value| value.parse::<f64>().ok())
            .map(|celsius| quantity_policy.format_temperature(celsius + 273.15, 3))
            .collect::<Vec<_>>()
            .join(" / ")
    };
    let supply_axis = format_numeric_axis(&supplies, "× nominal");
    let event = table_summary_composition(
        ui,
        "models.corners.composition",
        available,
        layout,
        |ui, table_h| {
            let event = data_table(
                ui,
                "models.corners",
                GENERAL_TABLE_MIN_W,
                &columns,
                &rows,
                egui::vec2(ui.available_width(), table_h),
                "No process-corner bindings are present in the loaded model libraries.",
            );
            summary_cards(
                ui,
                layout.narrow,
                "Binding policy",
                &[
                    ("Resolved bindings", resolved.to_string()),
                    ("Unresolved bindings", unresolved.to_string()),
                    ("Missing non-TT section", "fail closed".to_owned()),
                ],
                "Environment axes",
                &[
                    ("Temperature", temperature_axis),
                    ("Supply factor", supply_axis),
                    (
                        "PDK search paths",
                        app.state.pdk_config.library_paths.len().to_string(),
                    ),
                ],
            );
            event
        },
    );
    if let Some(event) = event
        && let Some((library, corner)) = targets.get(&event.key).cloned()
    {
        app.state.model_library_manager.select_library(&library);
        if let Some(library) = app.state.model_library_manager.get_library_mut(&library) {
            library.select_corner(&corner);
        }
    }
}

fn include_graph(ui: &mut Ui, app: &mut RSpiceApp) {
    let collapsed_id = ui.make_persistent_id("models.include.collapsed");
    let mut collapsed = ui
        .ctx()
        .data_mut(|data| data.get_temp::<bool>(collapsed_id).unwrap_or(false));
    let before = collapsed;
    surface_title(
        ui,
        "Include resolution · content addressed",
        "Model include graph",
        "Inspect ordered dependency resolution, captured paths, source pins, and cycle diagnostics.",
        true,
        |ui| {
            let label = if collapsed {
                "Expand transitive"
            } else {
                "Collapse transitive"
            };
            if Button::new(label).show(ui).clicked() {
                collapsed = !collapsed;
            }
        },
    );
    if collapsed != before {
        ui.ctx()
            .data_mut(|data| data.insert_temp(collapsed_id, collapsed));
    }

    let libraries = app
        .state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| IncludeLibrary {
            name: library.name.clone(),
            root: library.root_path.clone(),
            sources: library
                .source_closure
                .iter()
                .map(|source| (source.path.clone(), source.digest.to_string()))
                .collect(),
            edges: library
                .source_edges
                .iter()
                .map(|edge| {
                    (
                        edge.owner.clone(),
                        edge.requested_path.clone(),
                        edge.target.clone(),
                    )
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    let diagnostics = include_diagnostics(app);
    include_graph_content(ui, &libraries, &diagnostics, collapsed);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuditTone {
    Neutral,
    Info,
    Error,
}

impl AuditTone {
    fn color(self, tokens: &Tokens) -> Color32 {
        match self {
            Self::Neutral => tokens.color.text_dim,
            Self::Info => tokens.color.info,
            Self::Error => tokens.color.err,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModelMetadataAudit {
    source: &'static str,
    source_tone: AuditTone,
    source_blocking: bool,
    geometry: &'static str,
    geometry_tone: AuditTone,
    geometry_blocking: bool,
    geometry_declared: bool,
    description_missing: bool,
}

fn audit_model_metadata(library: &ModelLibrary, model: &DeviceModel) -> ModelMetadataAudit {
    let external = library.root_path.is_some() || model.file_path.is_some();
    let source = model.file_path.as_deref().or(library.root_path.as_deref());
    let source_pinned = library.root_path.is_some()
        && source.is_some_and(|source| library.source_closure.iter().any(|pin| pin.path == source));
    let (source_label, source_tone, source_blocking) = if !external {
        ("in memory", AuditTone::Neutral, false)
    } else if source.is_none() {
        ("path missing", AuditTone::Error, true)
    } else if library.source_closure.is_empty() {
        ("unpinned", AuditTone::Error, true)
    } else if source_pinned {
        ("pinned", AuditTone::Info, false)
    } else {
        ("outside closure", AuditTone::Error, true)
    };

    let geometry_declared = model.l_min.is_some()
        || model.l_max.is_some()
        || model.w_min.is_some()
        || model.w_max.is_some();
    let geometry_invalid = model_geometry_invalid(model);
    let (geometry, geometry_tone) = if geometry_invalid {
        ("inconsistent", AuditTone::Error)
    } else if geometry_declared {
        ("recorded", AuditTone::Info)
    } else {
        ("not declared", AuditTone::Neutral)
    };

    ModelMetadataAudit {
        source: source_label,
        source_tone,
        source_blocking,
        geometry,
        geometry_tone,
        geometry_blocking: geometry_invalid,
        geometry_declared,
        description_missing: model.description.trim().is_empty(),
    }
}

fn metadata_audit(ui: &mut Ui, app: &mut RSpiceApp) {
    surface_title(
        ui,
        "Model metadata · catalog audit",
        "Model metadata audit",
        "Inspect retained source pins, geometry limits, descriptions, and parameter records. This audit does not qualify numerical behavior or runtime parity.",
        false,
        |_| {},
    );

    let t = Tokens::get(ui.ctx());
    let mut total = 0usize;
    let mut source_findings = 0usize;
    let mut invalid_geometry = 0usize;
    let mut undeclared_geometry = 0usize;
    let mut undocumented = 0usize;
    let mut rows = Vec::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        let mut models = library.models.values().collect::<Vec<_>>();
        models.sort_by(|left, right| left.name.cmp(&right.name));
        for model in models {
            total += 1;
            let audit = audit_model_metadata(library, model);
            source_findings += usize::from(audit.source_blocking);
            invalid_geometry += usize::from(audit.geometry_blocking);
            undeclared_geometry += usize::from(!audit.geometry_declared);
            undocumented += usize::from(audit.description_missing);
            let blocking = audit.source_blocking || audit.geometry_blocking;
            let selected = app.state.workbench.selected_model.as_deref() == Some(&model.name)
                && app.state.model_library_manager.selected_library.as_deref()
                    == Some(&library.name);
            rows.push(DataRow {
                key: model_key(&library.name, &model.name),
                selected,
                cells: vec![
                    DataCell::mono(&model.name),
                    DataCell::plain(&library.name),
                    DataCell::mono_colored(audit.source, audit.source_tone.color(&t)),
                    DataCell::mono_colored(audit.geometry, audit.geometry_tone.color(&t)),
                    DataCell::mono_colored(
                        if audit.description_missing {
                            "missing"
                        } else {
                            "recorded"
                        },
                        if audit.description_missing {
                            t.color.warn
                        } else {
                            t.color.info
                        },
                    ),
                    DataCell::mono(model.parameters.len().to_string()),
                    DataCell::mono_colored(
                        if blocking {
                            "review"
                        } else if audit.description_missing {
                            "advisory"
                        } else {
                            "metadata complete"
                        },
                        if blocking {
                            t.color.err
                        } else if audit.description_missing {
                            t.color.warn
                        } else {
                            t.color.info
                        },
                    ),
                ],
            });
        }
    }

    let blocking = source_findings + invalid_geometry;
    let metrics = [
        Kpi::new(
            "Models audited",
            total.to_string(),
            "complete loaded catalog",
            t.color.text,
        ),
        Kpi::new(
            "Source records",
            (total.saturating_sub(source_findings)).to_string(),
            format!("{source_findings} require review"),
            if source_findings == 0 {
                t.color.info
            } else {
                t.color.err
            },
        ),
        Kpi::new(
            "Geometry metadata",
            (total.saturating_sub(undeclared_geometry)).to_string(),
            format!("{invalid_geometry} inconsistent · {undeclared_geometry} not declared"),
            if invalid_geometry == 0 {
                t.color.info
            } else {
                t.color.err
            },
        ),
        Kpi::new(
            "Documentation",
            (total.saturating_sub(undocumented)).to_string(),
            format!("{undocumented} missing"),
            if undocumented == 0 {
                t.color.info
            } else {
                t.color.warn
            },
        ),
    ];
    kpi_strip(ui, &metrics);

    let footer_h = if ui.available_width() <= 560.0 {
        66.0
    } else {
        44.0
    };
    let table_size = egui::vec2(
        ui.available_width(),
        (ui.available_height() - footer_h).max(120.0),
    );
    let columns = [
        ("Model", 0.18),
        ("Library", 0.15),
        ("Source record", 0.16),
        ("Geometry metadata", 0.16),
        ("Documentation", 0.15),
        ("Parameters", 0.10),
        ("Audit", 0.10),
    ];
    if let Some(event) = data_table(
        ui,
        "models.metadata-audit",
        820.0,
        &columns,
        &rows,
        table_size,
        "No loaded models are available for metadata audit.",
    ) {
        let (library, model) = split_model_key(&event.key);
        app.state.model_library_manager.select_library(library);
        app.state.workbench.selected_model = Some(model.to_owned());
    }
    metadata_audit_footer(ui, blocking, undocumented);
}

fn surface_title(
    ui: &mut Ui,
    eyebrow: &str,
    title: &str,
    description: &str,
    has_actions: bool,
    actions: impl FnOnce(&mut Ui),
) {
    let narrow = model_title_actions_stack(ui.available_width());
    workspace_title_row(ui, |ui| {
        if narrow {
            ui.vertical(|ui| {
                heading(ui, eyebrow, title, description);
                if has_actions {
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(actions);
                }
            });
        } else {
            ui.horizontal_top(|ui| {
                let action_reserve = if has_actions {
                    240.0_f32.min(ui.available_width() * 0.34)
                } else {
                    0.0
                };
                ui.allocate_ui_with_layout(
                    egui::vec2((ui.available_width() - action_reserve).max(1.0), 0.0),
                    Layout::top_down(Align::Min),
                    |ui| heading(ui, eyebrow, title, description),
                );
                if has_actions {
                    ui.with_layout(Layout::right_to_left(Align::Center), actions);
                }
            });
        }
    });
}

fn model_tab_strip_height(touch: bool, filter_visible: bool) -> f32 {
    if touch && filter_visible {
        54.0
    } else if touch {
        44.0
    } else {
        38.0
    }
}

fn model_catalog_min_width(surface_width: f32) -> f32 {
    if surface_width <= MODEL_PHONE_BREAKPOINT {
        MODEL_PHONE_TABLE_MIN_W
    } else {
        MODEL_TABLE_MIN_W
    }
}

fn model_title_actions_stack(surface_width: f32) -> bool {
    surface_width <= MODEL_PHONE_BREAKPOINT
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ModelTableSummaryLayout {
    narrow: bool,
    table_height: f32,
    owns_vertical_scroll: bool,
}

fn model_table_summary_layout(available: Vec2, touch: bool) -> ModelTableSummaryLayout {
    let narrow = available.x <= MODEL_SUMMARY_BREAKPOINT || touch;
    let summary_height = if narrow {
        MODEL_STACKED_SUMMARY_H
    } else {
        MODEL_WIDE_SUMMARY_H
    };
    let table_height = (available.y - summary_height).max(MODEL_TABLE_MIN_H);
    ModelTableSummaryLayout {
        narrow,
        table_height,
        owns_vertical_scroll: available.y < summary_height + MODEL_TABLE_MIN_H,
    }
}

fn table_summary_composition<R>(
    ui: &mut Ui,
    salt: &'static str,
    available: Vec2,
    layout: ModelTableSummaryLayout,
    content: impl FnOnce(&mut Ui, f32) -> R,
) -> R {
    if !layout.owns_vertical_scroll {
        return content(ui, layout.table_height);
    }

    let (viewport, _) = ui.allocate_exact_size(available.max(Vec2::splat(1.0)), Sense::hover());
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing = Vec2::ZERO;
    ScrollArea::vertical()
        .id_salt(salt)
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_min_width(viewport.width());
            content(ui, layout.table_height)
        })
        .inner
}

#[derive(Debug, Clone)]
struct DataCell {
    text: String,
    mono: bool,
    color: Option<Color32>,
}

impl DataCell {
    fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mono: false,
            color: None,
        }
    }

    fn mono(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mono: true,
            color: None,
        }
    }

    fn mono_colored(text: impl Into<String>, color: Color32) -> Self {
        Self {
            text: text.into(),
            mono: true,
            color: Some(color),
        }
    }
}

#[derive(Debug, Clone)]
struct DataRow {
    key: String,
    selected: bool,
    cells: Vec<DataCell>,
}

#[derive(Debug, Clone)]
struct TableEvent {
    key: String,
    activate: bool,
}

fn data_table(
    ui: &mut Ui,
    salt: &'static str,
    min_width: f32,
    columns: &[(&str, f32)],
    rows: &[DataRow],
    desired_size: Vec2,
    empty_message: &str,
) -> Option<TableEvent> {
    let t = Tokens::get(ui.ctx());
    let desired_size = egui::vec2(desired_size.x.max(1.0), desired_size.y.max(1.0));
    let (viewport, _) = ui.allocate_exact_size(desired_size, Sense::hover());
    ui.painter().rect_filled(viewport, 0.0, t.color.bg_panel);
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing = Vec2::ZERO;
    let table_width = viewport.width().max(min_width);
    let mut event = None;
    let mut requested_focus = None;
    ScrollArea::both()
        .id_salt(("workbench.data-table", salt))
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.set_min_width(table_width);
            let (head_rect, _) =
                ui.allocate_exact_size(egui::vec2(table_width, TABLE_HEAD_H), Sense::hover());
            ui.painter().rect_filled(head_rect, 0.0, t.color.bg_panel_2);
            ui.painter().hline(
                head_rect.x_range(),
                head_rect.bottom(),
                Stroke::new(1.0, t.color.border),
            );
            paint_table_cells(ui, head_rect, columns, None, true);

            for (index, row) in rows.iter().enumerate() {
                let row_h = t.metrics.row_h.max(28.0);
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(table_width, row_h), Sense::hover());
                let id = ui.id().with((salt, row.key.as_str()));
                let response = ui.interact(rect, id, Sense::click());
                response.widget_info(|| {
                    egui::WidgetInfo::selected(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        row.selected,
                        row.cells.first().map_or("", |cell| cell.text.as_str()),
                    )
                });
                if row.selected {
                    ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
                    ui.painter().rect_filled(
                        Rect::from_min_max(rect.min, egui::pos2(rect.left() + 2.0, rect.bottom())),
                        0.0,
                        t.color.accent,
                    );
                } else if response.hovered() {
                    ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                }
                ui.painter().hline(
                    rect.x_range(),
                    rect.bottom(),
                    Stroke::new(1.0, t.color.border.gamma_multiply(0.75)),
                );
                paint_table_cells(ui, rect, columns, Some(&row.cells), false);
                theme::paint_focus_ring_outset(ui, &response, rect);

                if response.clicked() {
                    response.request_focus();
                    event = Some(TableEvent {
                        key: row.key.clone(),
                        activate: response.double_clicked(),
                    });
                }
                if response.has_focus() {
                    let move_to = ui.input(|input| {
                        if input.key_pressed(Key::Home) {
                            Some(0)
                        } else if input.key_pressed(Key::End) {
                            Some(rows.len().saturating_sub(1))
                        } else if input.key_pressed(Key::ArrowUp) {
                            Some(index.saturating_sub(1))
                        } else if input.key_pressed(Key::ArrowDown) {
                            Some((index + 1).min(rows.len().saturating_sub(1)))
                        } else {
                            None
                        }
                    });
                    if let Some(target) = move_to.filter(|target| *target != index) {
                        requested_focus = rows.get(target).map(|target_row| {
                            (
                                ui.id().with((salt, target_row.key.as_str())),
                                target_row.key.clone(),
                            )
                        });
                    }
                }
            }

            if rows.is_empty() {
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(table_width, t.metrics.row_h.max(44.0)),
                    Sense::hover(),
                );
                ui.painter().text(
                    egui::pos2(rect.left() + 8.0, rect.center().y),
                    Align2::LEFT_CENTER,
                    empty_message,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_dim,
                );
            }
        });
    if let Some((id, key)) = requested_focus {
        ui.memory_mut(|memory| memory.request_focus(id));
        event = Some(TableEvent {
            key,
            activate: false,
        });
    }
    event
}

fn paint_table_cells(
    ui: &Ui,
    row_rect: Rect,
    columns: &[(&str, f32)],
    cells: Option<&[DataCell]>,
    header: bool,
) {
    let t = Tokens::get(ui.ctx());
    let mut x = row_rect.left();
    for (index, (title, fraction)) in columns.iter().enumerate() {
        let right = if index + 1 == columns.len() {
            row_rect.right()
        } else {
            (x + row_rect.width() * fraction).min(row_rect.right())
        };
        let rect = Rect::from_min_max(
            egui::pos2(x, row_rect.top()),
            egui::pos2(right, row_rect.bottom()),
        );
        let (text, mono, color) = if header {
            (title.to_uppercase(), false, t.color.text_faint)
        } else if let Some(cell) = cells.and_then(|cells| cells.get(index)) {
            (
                cell.text.clone(),
                cell.mono,
                cell.color.unwrap_or(t.color.text_dim),
            )
        } else {
            (String::new(), false, t.color.text_dim)
        };
        let font = if mono {
            theme::mono(tokens::FS_0, FontWeight::Regular)
        } else {
            theme::sans(
                tokens::FS_0,
                if header {
                    FontWeight::Medium
                } else {
                    FontWeight::Regular
                },
            )
        };
        let text = elide(ui, &text, &font, (rect.width() - 16.0).max(1.0));
        ui.painter().with_clip_rect(rect).text(
            egui::pos2(rect.left() + 8.0, rect.center().y),
            Align2::LEFT_CENTER,
            text,
            font,
            color,
        );
        x = right;
    }
}

fn elide(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let chars = text.chars().collect::<Vec<_>>();
    let mut low = 0usize;
    let mut high = chars.len();
    while low < high {
        let mid = (low + high).div_ceil(2);
        let candidate = format!("{}…", chars[..mid].iter().collect::<String>());
        let width = ui
            .painter()
            .layout_no_wrap(candidate, font.clone(), Color32::WHITE)
            .size()
            .x;
        if width <= max_width {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    format!("{}…", chars[..low].iter().collect::<String>())
}

#[derive(Debug, Default, Clone, Copy)]
struct SymbolStats {
    symbols: usize,
    pins: usize,
    unplaced_pins: usize,
    missing_contracts: usize,
    invalid_documents: usize,
    parameter_forms: usize,
}

fn symbol_summary(ui: &mut Ui, stats: SymbolStats, narrow: bool) {
    summary_cards(
        ui,
        narrow,
        "Pin contract",
        &[
            ("Symbol views", stats.symbols.to_string()),
            ("Explicit pins", stats.pins.to_string()),
            ("Unplaced pins", stats.unplaced_pins.to_string()),
        ],
        "Parameter form",
        &[
            ("Defined forms", stats.parameter_forms.to_string()),
            ("Missing pin contracts", stats.missing_contracts.to_string()),
            (
                "Invalid symbol metadata",
                stats.invalid_documents.to_string(),
            ),
        ],
    );
}

fn summary_cards(
    ui: &mut Ui,
    narrow: bool,
    left_title: &str,
    left: &[(&str, String)],
    right_title: &str,
    right: &[(&str, String)],
) {
    let width = ui.available_width().max(1.0);
    if narrow {
        property_card(ui, left_title, |ui| {
            for (label, value) in left {
                property_row(ui, label, value);
            }
        });
        property_card(ui, right_title, |ui| {
            for (label, value) in right {
                property_row(ui, label, value);
            }
        });
    } else {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            let column_w = ((width - 1.0) * 0.5).max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(column_w, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    property_card(ui, left_title, |ui| {
                        for (label, value) in left {
                            property_row(ui, label, value);
                        }
                    });
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(column_w, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    property_card(ui, right_title, |ui| {
                        for (label, value) in right {
                            property_row(ui, label, value);
                        }
                    });
                },
            );
        });
    }
}

#[derive(Debug, Clone)]
struct IncludeLibrary {
    name: String,
    root: Option<PathBuf>,
    sources: Vec<(PathBuf, String)>,
    edges: Vec<(PathBuf, String, PathBuf)>,
}

#[derive(Debug, Clone)]
struct IncludeDiagnostics {
    files: usize,
    definitions: usize,
    edges: usize,
    duplicate_definitions: usize,
    cyclic_nodes: usize,
    unpinned_roots: usize,
}

fn include_diagnostics(app: &RSpiceApp) -> IncludeDiagnostics {
    let libraries = app.state.model_library_manager.libraries_sorted();
    let mut sources = HashSet::<PathBuf>::new();
    let mut names = HashMap::<String, usize>::new();
    let mut edges = Vec::<(PathBuf, PathBuf)>::new();
    let mut unpinned_roots = 0usize;
    for library in &libraries {
        if library.root_path.is_some() && library.source_closure.is_empty() {
            unpinned_roots += 1;
        }
        for source in &library.source_closure {
            sources.insert(source.path.clone());
        }
        for edge in &library.source_edges {
            edges.push((edge.owner.clone(), edge.target.clone()));
        }
        for name in library.models.keys() {
            *names.entry(name.to_ascii_lowercase()).or_default() += 1;
        }
    }
    let duplicate_definitions = names.values().map(|count| count.saturating_sub(1)).sum();
    IncludeDiagnostics {
        files: sources.len(),
        definitions: names.values().sum(),
        edges: edges.len(),
        duplicate_definitions,
        cyclic_nodes: cyclic_node_count(&edges),
        unpinned_roots,
    }
}

fn cyclic_node_count(edges: &[(PathBuf, PathBuf)]) -> usize {
    let mut nodes = HashSet::<PathBuf>::new();
    let mut indegree = HashMap::<PathBuf, usize>::new();
    let mut outgoing = HashMap::<PathBuf, Vec<PathBuf>>::new();
    for (owner, target) in edges {
        nodes.insert(owner.clone());
        nodes.insert(target.clone());
        outgoing
            .entry(owner.clone())
            .or_default()
            .push(target.clone());
        *indegree.entry(target.clone()).or_default() += 1;
        indegree.entry(owner.clone()).or_default();
    }
    let mut queue = indegree
        .iter()
        .filter_map(|(node, degree)| (*degree == 0).then_some(node.clone()))
        .collect::<Vec<_>>();
    let mut visited = 0usize;
    while let Some(node) = queue.pop() {
        visited += 1;
        for target in outgoing.get(&node).into_iter().flatten() {
            if let Some(degree) = indegree.get_mut(target) {
                *degree = degree.saturating_sub(1);
                if *degree == 0 {
                    queue.push(target.clone());
                }
            }
        }
    }
    nodes.len().saturating_sub(visited)
}

fn include_graph_content(
    ui: &mut Ui,
    libraries: &[IncludeLibrary],
    diagnostics: &IncludeDiagnostics,
    collapsed: bool,
) {
    let size = ui.available_size();
    let narrow = size.x <= 1020.0;
    if narrow {
        let (viewport, _) = ui.allocate_exact_size(size, Sense::hover());
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(viewport)
                .layout(Layout::top_down(Align::Min)),
        );
        ScrollArea::vertical()
            .id_salt("models.include.stack")
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(viewport.width(), 330.0),
                    Layout::top_down(Align::Min),
                    |ui| draw_include_graph(ui, libraries, collapsed),
                );
                draw_include_diagnostics(ui, diagnostics);
            });
    } else {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            let graph_w = ((size.x - 1.0) * 0.695).max(1.0);
            let detail_w = (size.x - graph_w - 1.0).max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(graph_w, size.y),
                Layout::top_down(Align::Min),
                |ui| draw_include_graph(ui, libraries, collapsed),
            );
            ui.allocate_ui_with_layout(
                egui::vec2(detail_w, size.y),
                Layout::top_down(Align::Min),
                |ui| draw_include_diagnostics(ui, diagnostics),
            );
        });
    }
}

fn draw_include_graph(ui: &mut Ui, libraries: &[IncludeLibrary], collapsed: bool) {
    let t = Tokens::get(ui.ctx());
    let size = ui.available_size();
    let (viewport, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(viewport, 0.0, t.color.bg_inset);
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport.shrink(26.0))
            .layout(Layout::top_down(Align::Center)),
    );
    ScrollArea::both()
        .id_salt("models.include.graph")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_min_width(400.0);
            if libraries.is_empty() {
                ui.label(
                    egui::RichText::new("No loaded model libraries expose an include graph.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                return;
            }
            for library in libraries {
                let root_label = library
                    .root
                    .as_deref()
                    .map(path_display_name)
                    .unwrap_or_else(|| format!("{} · in-memory", library.name));
                include_node(
                    ui,
                    &root_label,
                    &format!("{} · {} pinned files", library.name, library.sources.len()),
                    true,
                );
                let root = library.root.as_deref();
                let visible_edges = library
                    .edges
                    .iter()
                    .filter(|(owner, _, _)| !collapsed || root.is_some_and(|root| owner == root))
                    .collect::<Vec<_>>();
                if visible_edges.is_empty() {
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new(
                            if library.root.is_some() && library.sources.is_empty() {
                                "source closure not pinned"
                            } else {
                                "no transitive includes"
                            },
                        )
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(
                            if library.root.is_some() && library.sources.is_empty() {
                                t.color.err
                            } else {
                                t.color.text_faint
                            },
                        ),
                    );
                } else {
                    let (edge_rect, _) =
                        ui.allocate_exact_size(egui::vec2(1.0, 28.0), Sense::hover());
                    ui.painter().vline(
                        edge_rect.center().x,
                        edge_rect.y_range(),
                        Stroke::new(1.0, t.color.border_strong),
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(14.0, 14.0);
                        for (_, requested, target) in visible_edges {
                            let digest = library
                                .sources
                                .iter()
                                .find(|(path, _)| path == target)
                                .map(|(_, digest)| short_digest(digest))
                                .unwrap_or_else(|| "digest unavailable".to_owned());
                            include_node(
                                ui,
                                &path_display_name(target),
                                &format!("{requested} · {digest}"),
                                false,
                            );
                        }
                    });
                }
                ui.add_space(22.0);
            }
        });
}

fn include_node(ui: &mut Ui, title: &str, detail: &str, root: bool) {
    let t = Tokens::get(ui.ctx());
    let width: f32 = if root { 360.0 } else { 230.0 };
    let width = width.min(ui.available_width().max(180.0));
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 52.0), Sense::hover());
    ui.painter().rect(
        rect,
        t.radius,
        if root {
            t.color.accent_dim
        } else {
            t.color.bg_panel
        },
        Stroke::new(
            1.0,
            if root {
                t.color.accent
            } else {
                t.color.border_strong
            },
        ),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 13.0, rect.top() + 16.0),
        Align2::LEFT_CENTER,
        elide(
            ui,
            title,
            &theme::sans(tokens::FS_0, FontWeight::SemiBold),
            rect.width() - 26.0,
        ),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 13.0, rect.bottom() - 14.0),
        Align2::LEFT_CENTER,
        elide(
            ui,
            detail,
            &theme::mono(tokens::FS_0, FontWeight::Regular),
            rect.width() - 26.0,
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

fn draw_include_diagnostics(ui: &mut Ui, diagnostics: &IncludeDiagnostics) {
    property_card(ui, "Resolution diagnostics", |ui| {
        property_row(ui, "Pinned files", &diagnostics.files.to_string());
        property_row(ui, "Definitions", &diagnostics.definitions.to_string());
        property_row(ui, "Captured edges", &diagnostics.edges.to_string());
        property_row(
            ui,
            "Duplicate definitions",
            &diagnostics.duplicate_definitions.to_string(),
        );
        property_row(ui, "Cyclic nodes", &diagnostics.cyclic_nodes.to_string());
        property_row(
            ui,
            "Unpinned external roots",
            &diagnostics.unpinned_roots.to_string(),
        );
    });
}

#[derive(Debug, Clone)]
struct Kpi {
    label: &'static str,
    value: String,
    foot: String,
    tone: Color32,
}

impl Kpi {
    fn new(label: &'static str, value: String, foot: impl Into<String>, tone: Color32) -> Self {
        Self {
            label,
            value,
            foot: foot.into(),
            tone,
        }
    }
}

fn kpi_strip(ui: &mut Ui, items: &[Kpi; 4]) {
    let t = Tokens::get(ui.ctx());
    let columns = if ui.available_width() <= 820.0 { 2 } else { 4 };
    let rows = items.len().div_ceil(columns);
    let cell_h = if t.metrics.ctl_h >= 44.0 { 76.0 } else { 73.0 };
    let total_h = cell_h * rows as f32 + (rows.saturating_sub(1)) as f32;
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), total_h), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.border);
    let cell_w = (rect.width() - (columns - 1) as f32) / columns as f32;
    for (index, item) in items.iter().enumerate() {
        let column = index % columns;
        let row = index / columns;
        let min = egui::pos2(
            rect.left() + column as f32 * (cell_w + 1.0),
            rect.top() + row as f32 * (cell_h + 1.0),
        );
        let cell = Rect::from_min_size(min, egui::vec2(cell_w, cell_h));
        ui.painter().rect_filled(cell, 0.0, t.color.bg_panel);
        ui.painter().text(
            egui::pos2(cell.left() + 10.0, cell.top() + 16.0),
            Align2::LEFT_CENTER,
            item.label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        ui.painter().text(
            egui::pos2(cell.left() + 10.0, cell.top() + 39.0),
            Align2::LEFT_CENTER,
            &item.value,
            theme::mono(18.0, FontWeight::Medium),
            item.tone,
        );
        ui.painter().text(
            egui::pos2(cell.left() + 10.0, cell.bottom() - 10.0),
            Align2::LEFT_BOTTOM,
            elide(
                ui,
                &item.foot,
                &theme::sans(tokens::FS_0, FontWeight::Regular),
                cell.width() - 20.0,
            ),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
}

fn metadata_audit_status(blocking: usize) -> &'static str {
    if blocking == 0 {
        "No blocking metadata inconsistencies found"
    } else {
        "Metadata inconsistencies require review"
    }
}

fn metadata_audit_footer(ui: &mut Ui, blocking: usize, undocumented: usize) {
    let t = Tokens::get(ui.ctx());
    let narrow = ui.available_width() <= 560.0;
    let height = if narrow { 66.0 } else { 44.0 };
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    let status = metadata_audit_status(blocking);
    if narrow {
        ui.painter().text(
            egui::pos2(rect.left() + 10.0, rect.top() + 17.0),
            Align2::LEFT_CENTER,
            "Metadata audit",
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
        );
        ui.painter().text(
            egui::pos2(rect.left() + 10.0, rect.top() + 36.0),
            Align2::LEFT_CENTER,
            status,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if blocking == 0 {
                t.color.info
            } else {
                t.color.err
            },
        );
        ui.painter().text(
            egui::pos2(rect.left() + 10.0, rect.bottom() - 8.0),
            Align2::LEFT_BOTTOM,
            format!("{undocumented} advisories · metadata only · not qualified"),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    } else {
        ui.painter().text(
            egui::pos2(rect.left() + 10.0, rect.center().y),
            Align2::LEFT_CENTER,
            "Metadata audit",
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
        );
        ui.painter().text(
            egui::pos2(rect.left() + 116.0, rect.center().y),
            Align2::LEFT_CENTER,
            format!(
                "{status} · {undocumented} description advisories · no numerical/runtime qualification"
            ),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if blocking == 0 {
                t.color.info
            } else {
                t.color.err
            },
        );
    }
}

fn open_create_symbol_dialog(app: &mut RSpiceApp) {
    let selected = app
        .state
        .library_manager
        .selected_library
        .as_ref()
        .and_then(|name| {
            app.state
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        });
    let target = selected.or_else(|| {
        app.state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| !library.read_only)
            .map(|library| library.name.clone())
    });
    let dialogs = &mut app.state.dialogs;
    dialogs.new_cell_library = target.unwrap_or_default();
    dialogs.new_cell_name.clear();
    dialogs.new_cell_description.clear();
    dialogs.new_cell_create_schematic = false;
    dialogs.new_cell_create_symbol = true;
    dialogs.new_cell_create_testbench = false;
    dialogs.new_cell_error = None;
    dialogs.new_cell_dialog = true;
}

fn symbol_model_family(app: &RSpiceApp, cell: &crate::state::Cell) -> String {
    if let Some(value) = metadata_value(
        [&cell.metadata],
        &["model.family", "model_family", "model", "model.name"],
    ) {
        return value;
    }
    app.state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .flat_map(|library| library.models.values())
        .find(|model| model.name.eq_ignore_ascii_case(&cell.name))
        .map(|model| model.name.clone())
        .unwrap_or_else(|| "unbound".to_owned())
}

fn metadata_value<const N: usize>(
    maps: [&HashMap<String, String>; N],
    keys: &[&str],
) -> Option<String> {
    maps.into_iter()
        .flat_map(|map| keys.iter().filter_map(|key| map.get(*key)))
        .find(|value| !value.trim().is_empty())
        .cloned()
}

fn model_geometry_invalid(model: &DeviceModel) -> bool {
    matches!((model.l_min, model.l_max), (Some(min), Some(max)) if min > max)
        || matches!((model.w_min, model.w_max), (Some(min), Some(max)) if min > max)
}

fn model_source_label(library: &ModelLibrary, model: &DeviceModel) -> String {
    model
        .file_path
        .as_deref()
        .or(library.root_path.as_deref())
        .map(path_display_name)
        .unwrap_or_else(|| "in-memory".to_owned())
}

fn model_sections_or_runtime(library: &ModelLibrary, model: &DeviceModel) -> String {
    let mut sections: Vec<String> = library
        .corners
        .values()
        .filter(|corner| corner.file_path.is_some())
        .map(|corner| corner.name.clone())
        .collect::<Vec<_>>();
    sections.sort_by_key(|section| section.to_ascii_lowercase());
    sections.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    if sections.is_empty() {
        model.level.display_name().to_owned()
    } else {
        sections.join(" / ")
    }
}

fn path_display_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

fn short_digest(digest: &str) -> String {
    if digest.len() <= 12 {
        digest.to_owned()
    } else {
        format!("{}…{}", &digest[..8], &digest[digest.len() - 4..])
    }
}

fn model_key(library: &str, item: &str) -> String {
    format!("{library}\u{1f}{item}")
}

fn split_model_key(key: &str) -> (&str, &str) {
    key.split_once('\u{1f}').unwrap_or((key, ""))
}

fn view_key(library: &str, cell: &str, view: &str) -> String {
    format!("{library}\u{1f}{cell}\u{1f}{view}")
}

fn format_numeric_axis(values: &BTreeSet<String>, suffix: &str) -> String {
    if values.is_empty() {
        return "not declared".to_owned();
    }
    let formatted = values
        .iter()
        .filter_map(|value| value.parse::<f64>().ok())
        .map(format_axis_value)
        .collect::<Vec<_>>()
        .join(" / ");
    format!("{formatted} {suffix}")
}

fn format_axis_value(value: f64) -> String {
    let formatted = format!("{value:.6}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_tabs_match_the_mockup_taxonomy() {
        assert_eq!(
            ModelsPage::ALL.map(ModelsPage::label),
            [
                "Models",
                "Symbols & CDF",
                "Corners & sections",
                "Include graph",
                "Metadata audit",
            ]
        );
    }

    #[test]
    fn responsive_model_geometry_matches_mockup_breakpoints() {
        assert_eq!(model_tab_strip_height(false, false), 38.0);
        assert_eq!(model_tab_strip_height(false, true), 38.0);
        assert_eq!(model_tab_strip_height(true, false), 44.0);
        assert_eq!(model_tab_strip_height(true, true), 54.0);

        assert_eq!(model_catalog_min_width(561.0), 780.0);
        assert_eq!(model_catalog_min_width(560.0), 690.0);
        assert!(!model_title_actions_stack(820.0));
        assert!(!model_title_actions_stack(561.0));
        assert!(model_title_actions_stack(560.0));
    }

    #[test]
    fn symbol_and_corner_compositions_own_overflow_without_changing_desktop_geometry() {
        let desktop = model_table_summary_layout(egui::vec2(1_120.0, 620.0), false);
        assert!(!desktop.narrow);
        assert_eq!(desktop.table_height, 470.0);
        assert!(!desktop.owns_vertical_scroll);

        let short_desktop = model_table_summary_layout(egui::vec2(1_120.0, 240.0), false);
        assert!(!short_desktop.narrow);
        assert_eq!(short_desktop.table_height, MODEL_TABLE_MIN_H);
        assert!(short_desktop.owns_vertical_scroll);

        let narrow = model_table_summary_layout(egui::vec2(560.0, 500.0), false);
        assert!(narrow.narrow);
        assert_eq!(narrow.table_height, 200.0);
        assert!(!narrow.owns_vertical_scroll);

        let short_narrow = model_table_summary_layout(egui::vec2(560.0, 380.0), false);
        assert!(short_narrow.narrow);
        assert_eq!(short_narrow.table_height, MODEL_TABLE_MIN_H);
        assert!(short_narrow.owns_vertical_scroll);

        let touch = model_table_summary_layout(egui::vec2(1_120.0, 380.0), true);
        assert!(touch.narrow);
        assert!(touch.owns_vertical_scroll);
    }

    #[test]
    fn external_model_without_source_closure_is_an_audit_finding() {
        let path = PathBuf::from("models/vendor.lib");
        let mut library = ModelLibrary::new("vendor");
        library.root_path = Some(path.clone());
        let mut model =
            DeviceModel::new("vendor_nmos", crate::state::model_library::ModelType::Nmos);
        model.file_path = Some(path);

        let audit = audit_model_metadata(&library, &model);

        assert_eq!(audit.source, "unpinned");
        assert!(audit.source_blocking);
        assert_eq!(audit.source_tone, AuditTone::Error);
    }

    #[test]
    fn pinned_source_is_metadata_evidence_not_a_qualification_gate() {
        let path = PathBuf::from("models/vendor.lib");
        let mut library = ModelLibrary::new("vendor");
        library.root_path = Some(path.clone());
        library
            .source_closure
            .push(crate::state::model_library::ModelSourcePin {
                path: path.clone(),
                digest: crate::product::ContentDigest::from_bytes([0x2a; 32]),
            });
        let mut model =
            DeviceModel::new("vendor_nmos", crate::state::model_library::ModelType::Nmos);
        model.file_path = Some(path);

        let audit = audit_model_metadata(&library, &model);

        assert_eq!(audit.source, "pinned");
        assert!(!audit.source_blocking);
        assert_eq!(audit.source_tone, AuditTone::Info);
        assert_eq!(
            metadata_audit_status(0),
            "No blocking metadata inconsistencies found"
        );
        assert!(!metadata_audit_status(0).contains("qualified"));
        assert!(!metadata_audit_status(0).contains("pass"));
    }

    #[test]
    fn in_memory_models_are_truthfully_distinct_from_pinned_sources() {
        let library = ModelLibrary::new("scratch");
        let model = DeviceModel::new(
            "scratch_resistor",
            crate::state::model_library::ModelType::Resistor,
        );

        let audit = audit_model_metadata(&library, &model);

        assert_eq!(audit.source, "in memory");
        assert!(!audit.source_blocking);
        assert_eq!(audit.source_tone, AuditTone::Neutral);
    }

    #[test]
    fn table_column_contracts_are_normalized() {
        let sum: f32 = [0.15, 0.17, 0.17, 0.14, 0.18, 0.10, 0.09].into_iter().sum();
        assert!((sum - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn cycle_diagnostics_report_only_nodes_remaining_after_topological_sort() {
        let a = PathBuf::from("a");
        let b = PathBuf::from("b");
        let c = PathBuf::from("c");
        assert_eq!(cyclic_node_count(&[(a.clone(), b.clone()), (b, c)]), 0);
        assert_eq!(cyclic_node_count(&[(a.clone(), a)]), 1);
    }
}
