//! Model catalog, symbol contracts, PDK sections, authenticated includes, and
//! the source-owned model qualification and release gate.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use egui::{Align, Align2, Color32, Key, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};
use sha2::{Digest as _, Sha256};

use crate::diagnostics::ConsoleMessage;
use crate::workbench::RSpiceApp;
use crate::workbench::app::{
    open_create_model_bound_symbol_dialog, open_symbol_import_dialog,
    open_symbol_parameter_form_dialog,
};
use crate::state::model_library::{
    DeviceModel, ModelCorrelationState, ModelLibrary, ModelQualificationState,
    ModelSourceEvidenceBinding, QualificationAnalysis, QualificationPlatform,
};
use crate::state::{CellViewRef, ModelBoundSymbolDefinition, SymbolDocument, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::super::commands::{Command, CommandAvailability};
use super::super::design_system::{
    property_card, property_row, property_row_toned, workspace_title_row,
};
use super::super::model_editor::{self, ModelEditorSection};
use super::super::state::{ModelsPage, Workspace};
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

const TABLE_HEAD_H: f32 = 27.0;
const TABLE_CARD_HEAD_H: f32 = 37.0;
const MODEL_TABLE_MIN_W: f32 = 780.0;
const MODEL_PHONE_TABLE_MIN_W: f32 = 690.0;
const GENERAL_TABLE_MIN_W: f32 = 760.0;
const MODEL_PHONE_BREAKPOINT: f32 = 560.0;
const MODEL_SUMMARY_BREAKPOINT: f32 = 820.0;
const MODEL_TABLE_MIN_H: f32 = 180.0;
const MODEL_WIDE_SUMMARY_H: f32 = 150.0;
const MODEL_STACKED_SUMMARY_H: f32 = 300.0;
const MODEL_TITLE_MIN_CONTENT_H: f32 = 48.0;
const QUALIFICATION_MIN_CONTENT_H: f32 = 680.0;
const QUALIFICATION_STACKED_MIN_CONTENT_H: f32 = 1000.0;
const QUALIFICATION_GATE_COPY: &str = "Dispositions, reruns, replacement, and retirement remain source-owned here. Release promotion cannot override missing or failing vector outcomes.";

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    // Own the viewport explicitly. Giving a framed child the parent's complete
    // available size as its minimum made that minimum leak into the first
    // title-row child: right-aligned actions were then laid out at the bottom
    // of the workspace and the actual page body received no usable height.
    let size = ui.available_size().max(Vec2::splat(1.0));
    let (viewport, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(viewport, 0.0, t.color.bg_app);
    let mut surface = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    surface.spacing_mut().item_spacing = Vec2::ZERO;
    // Keep the tab strip, filter reservation, and body on one page snapshot
    // for the entire frame. Applying a click only after the current body is
    // rendered avoids a one-frame hybrid (new body with old tab geometry).
    let current_page = app.state.workbench.models_page;
    let requested_page = model_tabs(&mut surface, app, current_page);
    match current_page {
        ModelsPage::Models => models_catalog(&mut surface, app),
        ModelsPage::Symbols => symbols(&mut surface, app),
        ModelsPage::Corners => corners(&mut surface, app),
        ModelsPage::Include => include_graph(&mut surface, app),
        ModelsPage::Qualification => qualification(&mut surface, app),
    }
    if let Some(page) = requested_page {
        app.state.workbench.models_page = page;
        ui.ctx().request_repaint();
    }
}

fn model_tabs(ui: &mut Ui, app: &mut RSpiceApp, current_page: ModelsPage) -> Option<ModelsPage> {
    let t = Tokens::get(ui.ctx());
    let touch = t.metrics.ctl_h >= 44.0;
    let tab_h = if touch { 44.0 } else { 37.0 };
    let filter_visible = current_page == ModelsPage::Models;
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
    let mut rendered_tabs = Vec::new();
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
                    if response.hovered() {
                        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
                    }
                    theme::paint_focus_ring(ui, &response, rect);
                    if response.clicked() {
                        selected = Some(page);
                    }
                    rendered_tabs.push((page, rect, galley, ui.painter().clone(), response));
                }
            });
        });

    for (page, rect, galley, painter, response) in rendered_tabs {
        let active = current_page == page;
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Button,
                ui.is_enabled(),
                active,
                page.label(),
            )
        });
        painter.galley(
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
            painter.rect_filled(
                Rect::from_min_max(
                    egui::pos2(rect.left() + 9.0, rect.bottom() - 2.0),
                    egui::pos2(rect.right() - 9.0, rect.bottom()),
                ),
                0.0,
                t.color.accent,
            );
        }
    }

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
        // The shipped packs are searched but never loaded, so the hint has to
        // distinguish the two: a count of what is resident, and a count of what
        // typing will reach.
        let indexed = app.state.model_library_manager.pack_definition_count();
        let hint = if indexed > 0 {
            format!("Filter {count} loaded · search {indexed} shipped…")
        } else {
            format!("Filter {count} models…")
        };
        edit.add_sized(
            edit_rect.size(),
            egui::TextEdit::singleline(&mut app.state.model_library_manager.filter_text)
                .frame(egui::Frame::NONE)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .hint_text(hint),
        );
    }

    ui.add_space(1.0);
    selected.filter(|page| *page != current_page)
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

    let mut rows = rows;
    let pack_rows = pack_catalog_rows(app, &query, &t);
    let pack_truncated = pack_rows.len() >= PACK_ROW_LIMIT;
    rows.extend(pack_rows);

    let empty_message = if app.state.model_library_manager.pack_definition_count() > 0 {
        "No models match the active filter, in the loaded libraries or the shipped packs."
    } else {
        "No models match the active filter."
    };

    let columns = [
        ("Model", 0.15),
        ("Family", 0.17),
        ("Source", 0.17),
        ("Library", 0.14),
        ("Sections / runtime", 0.18),
        ("Tests", 0.10),
        ("Status", 0.09),
    ];
    // A truncated result set must say so. Reporting a bounded window as if it
    // were the whole match set is the failure this table exists to avoid.
    let mut body = ui.available_size();
    if pack_truncated {
        body.y = (body.y - 16.0).max(1.0);
    }

    if let Some(event) = data_table(
        ui,
        "models.catalog",
        model_catalog_min_width(ui.available_width()),
        &columns,
        &rows,
        body,
        empty_message,
    ) {
        let (library, model) = split_model_key(&event.key);
        // Shipped-pack rows are keyed by pack id, which is not a loaded
        // library. Selecting one would leave the inspector bound to whatever
        // library was selected before while the model name changed underneath
        // it — showing another library's data under this name. Until a pack
        // definition can be loaded on demand, such a row is not selectable.
        if app.state.model_library_manager.get_library(library).is_some() {
            app.state.model_library_manager.select_library(library);
            app.state.workbench.selected_model = Some(model.to_owned());
        }
    }

    if pack_truncated {
        ui.label(
            egui::RichText::new(format!(
                "Showing the first {PACK_ROW_LIMIT} shipped-pack matches; narrow the filter to see more."
            ))
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
        );
    }
}

fn symbols(ui: &mut Ui, app: &mut RSpiceApp) {
    surface_title(
        ui,
        "Symbols, pins & device forms",
        "Symbol and component-definition manager",
        "Bind graphical symbols, terminals, parameter forms and model families without hiding netlist semantics.",
        true,
        |ui| {
            if Button::new("Create symbol").accent().show(ui).clicked() {
                open_create_model_bound_symbol_dialog(&mut app.state);
            }
            if Button::new("Import symbol").show(ui).clicked() {
                open_symbol_import_dialog(&mut app.state);
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
                let key = view_key(&library.name, &cell.name, &view.name);
                targets.insert(
                    key.clone(),
                    CellViewRef::new(&library.name, &cell.name, &view.name),
                );
                let typed_definition = ModelBoundSymbolDefinition::load_from_view(view);
                let model_family = typed_definition
                    .as_ref()
                    .ok()
                    .and_then(Option::as_ref)
                    .and_then(|definition| definition.netlist.model.as_ref())
                    .map(|model| model.model.clone())
                    .unwrap_or_else(|| symbol_model_family(app, cell));
                let parameter_form = typed_definition
                    .as_ref()
                    .ok()
                    .and_then(Option::as_ref)
                    .map(symbol_parameter_form_label)
                    .unwrap_or_else(|| {
                        metadata_value(
                            [&cell.metadata, &view.metadata],
                            &["parameter.form", "parameter_form", "cdf", "cdf.form"],
                        )
                        .unwrap_or_else(|| "not defined".to_owned())
                    });
                let netlist_template = typed_definition
                    .as_ref()
                    .ok()
                    .and_then(Option::as_ref)
                    .map(|definition| definition.netlist.template.trim())
                    .filter(|template| !template.is_empty())
                    .map(str::to_owned)
                    .or_else(|| {
                        metadata_value(
                            [&cell.metadata, &view.metadata],
                            &["netlist.template", "netlist_template"],
                        )
                    })
                    .or_else(|| {
                        cell.views_sorted()
                            .into_iter()
                            .find(|candidate| candidate.view_type == ViewType::Spice)
                            .map(|candidate| candidate.name.clone())
                    })
                    .unwrap_or_else(|| "not defined".to_owned());

                let (pins, status, tone) = match typed_definition.as_ref() {
                    Err(_) => {
                        stats.invalid_documents += 1;
                        ("invalid metadata".to_owned(), "invalid", t.color.err)
                    }
                    Ok(None) => match SymbolDocument::load_from_view(view) {
                        Ok(document) => {
                            stats.missing_contracts += 1;
                            let pins = if document.pins.is_empty() {
                                "not defined".to_owned()
                            } else {
                                document
                                    .pins
                                    .iter()
                                    .map(|pin| pin.name.as_str())
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            };
                            (pins, "legacy review", t.color.warn)
                        }
                        Err(_) => {
                            stats.invalid_documents += 1;
                            ("invalid metadata".to_owned(), "invalid", t.color.err)
                        }
                    },
                    Ok(Some(_)) => match SymbolDocument::load_from_view(view) {
                        Ok(document) if document.pins.is_empty() => {
                            stats.missing_contracts += 1;
                            (
                                "not defined".to_owned(),
                                "pin contract missing",
                                t.color.warn,
                            )
                        }
                        Ok(document) => {
                            let unplaced = document
                                .pins
                                .iter()
                                .filter(|pin| pin.position.is_none())
                                .count();
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
                    },
                };
                if status == "bound" {
                    stats.resolved += 1;
                }
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
            symbol_registry_header(ui, stats.resolved);
            let event = data_table(
                ui,
                "models.symbols",
                GENERAL_TABLE_MIN_W,
                &columns,
                &rows,
                egui::vec2(ui.available_width(), (table_h - TABLE_CARD_HEAD_H).max(1.0)),
                "No symbol views are present in the loaded design libraries.",
            );
            symbol_summary(
                ui,
                app,
                stats,
                layout.narrow,
                layout.summary_height,
                layout.owns_vertical_scroll,
            );
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
                layout.summary_height,
                layout.owns_vertical_scroll,
                SummaryCardSpec::new(
                    "Binding policy",
                    &[
                        ("Resolved bindings", resolved.to_string()),
                        ("Unresolved bindings", unresolved.to_string()),
                        ("Missing non-TT section", "fail closed".to_owned()),
                    ],
                ),
                SummaryCardSpec::new(
                    "Environment axes",
                    &[
                        ("Temperature", temperature_axis),
                        ("Supply factor", supply_axis),
                        (
                            "PDK search paths",
                            app.state.pdk_config.library_paths.len().to_string(),
                        ),
                    ],
                ),
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
enum QualificationGate {
    Qualified,
    Review,
    Unqualified,
    Blocked,
}

impl QualificationGate {
    const fn label(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Review => "review",
            Self::Unqualified => "unqualified",
            Self::Blocked => "blocked",
        }
    }

    fn color(self, tokens: &Tokens) -> Color32 {
        match self {
            Self::Qualified => tokens.color.ok,
            Self::Review | Self::Unqualified => tokens.color.warn,
            Self::Blocked => tokens.color.err,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum QualificationDomain {
    Dc,
    Ac,
    Transient,
    Noise,
}

impl QualificationDomain {
    const fn label(self) -> &'static str {
        match self {
            Self::Dc => "DC operating curves",
            Self::Ac => "AC / charge",
            Self::Transient => "Transient",
            Self::Noise => "Noise",
        }
    }

    const fn from_analysis(analysis: &QualificationAnalysis) -> Self {
        match analysis {
            QualificationAnalysis::DcOperatingPoint | QualificationAnalysis::DcSweep { .. } => {
                Self::Dc
            }
            QualificationAnalysis::AcSweep { .. } => Self::Ac,
            QualificationAnalysis::Transient { .. } => Self::Transient,
            QualificationAnalysis::Noise { .. } => Self::Noise,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct QualificationDomainAccumulator {
    vectors: usize,
    references: usize,
    quantities: BTreeSet<String>,
    tolerance_contracts: BTreeMap<(u64, u64), String>,
    evidenced_vectors: usize,
    passing_vectors: usize,
    open_dispositions: usize,
}

#[derive(Debug, Clone)]
struct QualificationDomainSummary {
    domain: QualificationDomain,
    vectors: usize,
    reference_coverage: String,
    tolerance: String,
    disposition: String,
    tone: QualificationGate,
}

#[derive(Debug, Clone)]
struct QualificationModelSummary {
    key: String,
    library: String,
    model: String,
    source_revision: String,
    source_error: Option<String>,
    suites: usize,
    vectors: usize,
    evidenced_vectors: usize,
    passing_vectors: usize,
    dc_vectors: usize,
    ac_vectors: usize,
    transient_vectors: usize,
    noise_vectors: usize,
    temperature_points: usize,
    references: usize,
    desktop_passing: usize,
    wasm_passing: usize,
    parity_suites: usize,
    worst_relative_error: Option<f64>,
    evidence_digest: Option<String>,
    open_dispositions: usize,
    releases: usize,
    comparison_available: bool,
    correlation_status: String,
    correlation_evidence_digest: Option<String>,
    gate: QualificationGate,
    domains: Vec<QualificationDomainSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QualificationPageAction {
    ReviewVectors,
    ReviewReleaseBinding,
    RunSuite,
    CompareRelease,
    OpenCorrelation,
}

fn qualification(ui: &mut Ui, app: &mut RSpiceApp) {
    let available = ui.available_size();
    let required_height = qualification_required_content_height(available.x);
    if available.y >= required_height {
        qualification_content(ui, app);
        return;
    }

    let (viewport, _) = ui.allocate_exact_size(available.max(Vec2::splat(1.0)), Sense::hover());
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing = Vec2::ZERO;
    ScrollArea::vertical()
        .id_salt("models.qualification.workspace")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            let content_width = ui.available_width().max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(
                    content_width,
                    qualification_required_content_height(content_width),
                ),
                Layout::top_down(Align::Min),
                |ui| qualification_content(ui, app),
            );
        });
}

fn qualification_required_content_height(width: f32) -> f32 {
    if width <= MODEL_SUMMARY_BREAKPOINT {
        QUALIFICATION_STACKED_MIN_CONTENT_H
    } else {
        QUALIFICATION_MIN_CONTENT_H
    }
}

fn qualification_content(ui: &mut Ui, app: &mut RSpiceApp) {
    let summaries = qualification_summaries(app);
    let selected = selected_qualification_summary(app, &summaries).cloned();
    let selected_vector_count = selected.as_ref().map_or(0, |summary| summary.vectors);
    let review_label = format!("Review {selected_vector_count} vectors");
    let run_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::RunSuite,
    );
    let review_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::ReviewVectors,
    );
    let compare_blocker = qualification_action_block_reason(
        app,
        selected.as_ref(),
        QualificationPageAction::CompareRelease,
    );
    let mut requested_action = None;

    surface_title_with_action_reserve(
        ui,
        "Model qualification · source-owned release gate",
        "Model qualification",
        "Versioned vectors, retained golden references, runtime parity, tolerances, and governed dispositions. Release closure consumes only source-owned evidence.",
        true,
        430.0,
        |ui| {
            let run_enabled = run_blocker.is_none();
            let mut run_button = Button::new("Run suite").enabled(run_enabled);
            if run_enabled {
                run_button = run_button.accent();
            }
            let run = run_button.show(ui);
            let run_clicked = run.clicked();
            if let Some(reason) = run_blocker.as_deref() {
                run.on_disabled_hover_text(reason);
            }
            if run_clicked {
                requested_action = Some(QualificationPageAction::RunSuite);
            }

            let review = Button::new(&review_label)
                .enabled(review_blocker.is_none())
                .show(ui);
            let review_clicked = review.clicked();
            if let Some(reason) = review_blocker.as_deref() {
                review.on_disabled_hover_text(reason);
            }
            if review_clicked {
                requested_action = Some(QualificationPageAction::ReviewVectors);
            }

            let compare = Button::new("Compare approved model")
                .enabled(compare_blocker.is_none())
                .show(ui);
            let compare_clicked = compare.clicked();
            if let Some(reason) = compare_blocker.as_deref() {
                compare.on_disabled_hover_text(reason);
            }
            if compare_clicked {
                requested_action = Some(QualificationPageAction::CompareRelease);
            }

            ui.menu_button("More", |ui| {
                if ui.button("Measurement correlation").clicked() {
                    requested_action = Some(QualificationPageAction::OpenCorrelation);
                    ui.close();
                }
            });
        },
    );

    if let Some(action) = requested_action {
        execute_qualification_action(app, action);
    }

    let t = Tokens::get(ui.ctx());
    let total_vectors = summaries
        .iter()
        .map(|summary| summary.vectors)
        .sum::<usize>();
    let passing_vectors = summaries
        .iter()
        .map(|summary| summary.passing_vectors)
        .sum::<usize>();
    let evidence_vectors = summaries
        .iter()
        .filter(|summary| summary.source_error.is_none())
        .map(|summary| summary.evidenced_vectors)
        .sum::<usize>();
    let qualified_models = summaries
        .iter()
        .filter(|summary| summary.gate == QualificationGate::Qualified)
        .count();
    let source_owned_models = summaries
        .iter()
        .filter(|summary| summary.source_error.is_none())
        .count();
    let parity_models = summaries
        .iter()
        .filter(|summary| {
            summary.suites > 0
                && summary.parity_suites == summary.suites
                && summary.source_error.is_none()
        })
        .count();
    let worst_relative_error = summaries
        .iter()
        .filter_map(|summary| summary.worst_relative_error)
        .max_by(f64::total_cmp);
    let metrics = [
        Kpi::new(
            "Vectors passing",
            format!("{passing_vectors} / {total_vectors}"),
            format!(
                "{} source dispositions pending",
                summaries
                    .iter()
                    .map(|summary| summary.open_dispositions)
                    .sum::<usize>()
            ),
            if total_vectors > 0 && passing_vectors == total_vectors {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        Kpi::new(
            "Reference coverage",
            format!("{evidence_vectors} / {total_vectors}"),
            "exact retained golden-reference evidence",
            if total_vectors > 0 && evidence_vectors == total_vectors {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
        Kpi::new(
            "Worst deviation",
            worst_relative_error
                .map(|value| format!("{:.4}%", value * 100.0))
                .unwrap_or_else(|| "No evidence".to_owned()),
            format!("{qualified_models} qualified models"),
            if worst_relative_error.is_some() {
                t.color.text
            } else {
                t.color.warn
            },
        ),
        Kpi::new(
            "Interpreter parity",
            if source_owned_models == 0 {
                "No source".to_owned()
            } else {
                format!("{parity_models} / {source_owned_models}")
            },
            if source_owned_models == 0 {
                "no project-owned models"
            } else {
                "desktop · WebAssembly"
            },
            if source_owned_models > 0 && parity_models == source_owned_models {
                t.color.ok
            } else {
                t.color.warn
            },
        ),
    ];
    kpi_strip(ui, &metrics);

    let detail_height = if ui.available_width() <= MODEL_SUMMARY_BREAKPOINT {
        460.0 + qualification_gate_footer_height(ui, ui.available_width())
    } else {
        330.0
    };
    let table_height = (ui.available_height() - detail_height)
        .clamp(135.0, 320.0)
        .min(ui.available_height().max(1.0));
    if let Some(event) = qualification_suite_table(
        ui,
        app,
        &summaries,
        egui::vec2(ui.available_width(), table_height),
    ) {
        let (library, model) = split_model_key(&event.key);
        app.state.model_library_manager.select_library(library);
        app.state.workbench.selected_model = Some(model.to_owned());
        if event.review {
            execute_qualification_action(app, QualificationPageAction::ReviewVectors);
        }
    }

    let selected_detail = selected_qualification_summary(app, &summaries);
    if let Some(action) = qualification_detail(ui, app, selected_detail, detail_height) {
        execute_qualification_action(app, action);
    }
}

fn qualification_summaries(app: &RSpiceApp) -> Vec<QualificationModelSummary> {
    let mut summaries = Vec::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        let mut models = library.models.values().collect::<Vec<_>>();
        models.sort_by(|left, right| left.name.cmp(&right.name));
        for model in models {
            summaries.push(qualification_model_summary(app, library, model));
        }
    }
    summaries
}

fn qualification_model_summary(
    app: &RSpiceApp,
    library: &ModelLibrary,
    model: &DeviceModel,
) -> QualificationModelSummary {
    let key = model_key(&library.name, &model.name);
    let open_draft = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .filter(|draft| {
            draft.library_name.eq_ignore_ascii_case(&library.name)
                && draft.model_name.eq_ignore_ascii_case(&model.name)
        });
    let resolved = model_editor::resolve_project_model_for_editor(
        &app.state.model_library_manager,
        &library.name,
        &model.name,
    );
    let (source_revision, source_error, state, source) = match resolved {
        Ok(resolved) => {
            let qualification = open_draft.map_or_else(
                || resolved.qualification.clone(),
                |draft| draft.qualification.clone(),
            );
            let source_id = open_draft.map_or_else(|| resolved.source_id, |draft| draft.source_id);
            let source_digest =
                open_draft.map_or(resolved.model_digest, |draft| draft.base_source_digest);
            let source_revision =
                open_draft.map_or(resolved.model_revision, |draft| draft.base_source_revision);
            let source = ModelSourceEvidenceBinding::try_new_project_bound(
                &model.name,
                source_id,
                source_digest,
                source_revision,
            );
            match source {
                Ok(source) => (
                    if open_draft.is_some_and(|draft| draft.qualification_is_dirty()) {
                        format!(
                            "{}@{} · working qualification",
                            model.name,
                            source_revision.get()
                        )
                    } else {
                        format!("{}@{}", model.name, source_revision.get())
                    },
                    None,
                    qualification,
                    Some(source),
                ),
                Err(error) => (
                    "invalid source identity".to_owned(),
                    Some(error.to_string()),
                    qualification,
                    None,
                ),
            }
        }
        Err(error) => (
            "not source-owned".to_owned(),
            Some(error),
            library
                .model_qualification
                .get(&model.name)
                .cloned()
                .unwrap_or_default(),
            None,
        ),
    };

    let mut summary = summarize_qualification_state(
        key,
        &library.name,
        model,
        source_revision,
        source_error,
        &state,
        source.as_ref(),
    );
    apply_correlation_qualification_contract(
        &mut summary,
        library.model_correlation.get(&model.name),
        source.as_ref(),
    );
    summary
}

fn summarize_qualification_state(
    key: String,
    library_name: &str,
    model: &DeviceModel,
    source_revision: String,
    mut source_error: Option<String>,
    state: &ModelQualificationState,
    source: Option<&ModelSourceEvidenceBinding>,
) -> QualificationModelSummary {
    if source_error.is_none()
        && let Err(error) = state.validate_for_model(&model.name)
    {
        source_error = Some(format!("Retained qualification state is invalid: {error}"));
    }
    let exact_suites = source
        .and_then(|source| state.exact_suites_for_source(source).ok())
        .unwrap_or_default();
    let mut vectors = 0usize;
    let mut evidenced_vectors = 0usize;
    let mut passing_vectors = 0usize;
    let mut dc_vectors = 0usize;
    let mut ac_vectors = 0usize;
    let mut transient_vectors = 0usize;
    let mut noise_vectors = 0usize;
    let mut temperatures = BTreeSet::<String>::new();
    let mut references = 0usize;
    let mut desktop_passing = 0usize;
    let mut wasm_passing = 0usize;
    let mut parity_suites = 0usize;
    let mut worst_relative_error: Option<f64> = None;
    let mut evidence_members = Vec::<(String, u64, crate::product::ContentDigest)>::new();
    let mut all_suites_have_passing_evidence = !exact_suites.is_empty();
    let mut domain_accumulators =
        BTreeMap::<QualificationDomain, QualificationDomainAccumulator>::new();

    for suite in &exact_suites {
        vectors += suite.vectors.len();
        let evidence = source.and_then(|source| {
            state.evidence.iter().find(|evidence| {
                evidence.source == *source
                    && evidence.suite_id.eq_ignore_ascii_case(&suite.id)
                    && evidence.suite_revision == suite.revision
            })
        });
        for vector in &suite.vectors {
            references += vector.references.len();
            let domain = QualificationDomain::from_analysis(&vector.analysis);
            let accumulator = domain_accumulators.entry(domain).or_default();
            accumulator.vectors += 1;
            accumulator.references += vector.references.len();
            for reference in &vector.references {
                accumulator.quantities.insert(reference.quantity.clone());
                let absolute = reference.absolute_tolerance.get();
                let relative = reference.relative_tolerance.get();
                accumulator
                    .tolerance_contracts
                    .entry(qualification_tolerance_key(absolute, relative))
                    .or_insert_with(|| qualification_tolerance_label(absolute, relative));
            }
            if let Some(outcome) = evidence.and_then(|evidence| {
                evidence
                    .vector_outcomes
                    .iter()
                    .find(|outcome| outcome.vector_id.eq_ignore_ascii_case(&vector.id))
            }) {
                accumulator.evidenced_vectors += 1;
                accumulator.passing_vectors += usize::from(outcome.passed);
            }
            accumulator.open_dispositions += source.map_or(0, |source| {
                state
                    .vector_dispositions
                    .iter()
                    .filter(|disposition| {
                        disposition.is_open()
                            && disposition.vector.source == *source
                            && disposition.vector.suite_id.eq_ignore_ascii_case(&suite.id)
                            && disposition.vector.suite_revision == suite.revision
                            && disposition
                                .vector
                                .vector_id
                                .eq_ignore_ascii_case(&vector.id)
                    })
                    .count()
            });

            match &vector.analysis {
                QualificationAnalysis::DcOperatingPoint | QualificationAnalysis::DcSweep { .. } => {
                    dc_vectors += 1;
                }
                QualificationAnalysis::AcSweep { .. } => ac_vectors += 1,
                QualificationAnalysis::Transient { .. } => transient_vectors += 1,
                QualificationAnalysis::Noise {
                    temperature_kelvin, ..
                } => {
                    noise_vectors += 1;
                    temperatures.insert(format!("{:.6}", temperature_kelvin.get()));
                }
            }
        }

        if let Some(evidence) = evidence {
            evidenced_vectors += evidence.vector_outcomes.len();
            passing_vectors += evidence
                .vector_outcomes
                .iter()
                .filter(|outcome| outcome.passed)
                .count();
            all_suites_have_passing_evidence &= evidence.passed;
            if let Ok(digest) = evidence.content_digest() {
                evidence_members.push((suite.id.clone(), suite.revision.get(), digest));
            }
            for reference in evidence
                .vector_outcomes
                .iter()
                .flat_map(|outcome| &outcome.platforms)
                .flat_map(|platform| &platform.references)
            {
                worst_relative_error = Some(
                    worst_relative_error.map_or(reference.relative_error.get(), |current| {
                        current.max(reference.relative_error.get())
                    }),
                );
            }
        } else {
            all_suites_have_passing_evidence = false;
        }

        let platform_run = |platform| {
            source.and_then(|source| {
                state.platform_runs.iter().find(|run| {
                    run.platform == platform
                        && run.source == *source
                        && run.suite_id.eq_ignore_ascii_case(&suite.id)
                        && run.suite_revision == suite.revision
                })
            })
        };
        let desktop = platform_run(QualificationPlatform::Desktop);
        let wasm = platform_run(QualificationPlatform::WebAssembly);
        desktop_passing += desktop.map_or(0, |run| {
            run.vector_outcomes
                .iter()
                .filter(|outcome| outcome.outcome.passed)
                .count()
        });
        wasm_passing += wasm.map_or(0, |run| {
            run.vector_outcomes
                .iter()
                .filter(|outcome| outcome.outcome.passed)
                .count()
        });
        parity_suites += usize::from(
            desktop.is_some_and(|run| run.passed) && wasm.is_some_and(|run| run.passed),
        );
    }

    let open_dispositions = source.map_or(0, |source| {
        state
            .vector_dispositions
            .iter()
            .filter(|disposition| disposition.is_open() && disposition.vector.source == *source)
            .count()
    });
    let gate = if source_error.is_some() {
        QualificationGate::Blocked
    } else if exact_suites.is_empty() {
        QualificationGate::Unqualified
    } else if all_suites_have_passing_evidence
        && parity_suites == exact_suites.len()
        && open_dispositions == 0
    {
        QualificationGate::Qualified
    } else {
        QualificationGate::Review
    };
    let evidence_digest = qualification_evidence_contract_digest(&mut evidence_members);
    let domains = qualification_domain_summaries(domain_accumulators);

    QualificationModelSummary {
        key,
        library: library_name.to_owned(),
        model: model.name.clone(),
        source_revision,
        source_error,
        suites: exact_suites.len(),
        vectors,
        evidenced_vectors,
        passing_vectors,
        dc_vectors,
        ac_vectors,
        transient_vectors,
        noise_vectors,
        temperature_points: temperatures.len(),
        references,
        desktop_passing,
        wasm_passing,
        parity_suites,
        worst_relative_error,
        evidence_digest,
        open_dispositions,
        releases: state.releases.len(),
        comparison_available: !state.candidates.is_empty() && !state.releases.is_empty(),
        correlation_status: "not configured".to_owned(),
        correlation_evidence_digest: None,
        gate,
        domains,
    }
}

fn apply_correlation_qualification_contract(
    summary: &mut QualificationModelSummary,
    correlation: Option<&ModelCorrelationState>,
    source: Option<&ModelSourceEvidenceBinding>,
) {
    let Some(correlation) = correlation.filter(|state| !state.suites.is_empty()) else {
        return;
    };
    if let Err(error) = correlation.validate_for_model(&summary.model) {
        summary.correlation_status = format!("invalid retained state: {error}");
        summary.gate = QualificationGate::Blocked;
        return;
    }
    let Some(source) = source else {
        summary.correlation_status = "source identity unavailable".to_owned();
        summary.gate = QualificationGate::Blocked;
        return;
    };
    let suites = correlation
        .suite_lineages()
        .into_iter()
        .filter(|suite| suite.source == *source)
        .collect::<Vec<_>>();
    if suites.is_empty() {
        summary.correlation_status =
            "configured evidence is stale for this model revision".to_owned();
        if summary.gate != QualificationGate::Blocked {
            summary.gate = QualificationGate::Review;
        }
        return;
    }

    let mut approved = 0usize;
    let mut evidence_members = Vec::new();
    for suite in &suites {
        let evidence = correlation
            .evidence
            .iter()
            .filter(|evidence| {
                evidence.suite_id.eq_ignore_ascii_case(&suite.id)
                    && evidence.suite_revision == suite.revision
                    && evidence.source == *source
                    && evidence.validate_current(suite).is_ok()
            })
            .max_by_key(|evidence| (evidence.reviewed_at_unix_ms, evidence.id.as_str()));
        if let Some(evidence) = evidence.filter(|evidence| evidence.approved()) {
            approved += 1;
            if let Ok(digest) = evidence.content_digest() {
                evidence_members.push((suite.id.clone(), suite.revision.get(), digest));
            }
        }
    }
    summary.correlation_evidence_digest =
        qualification_evidence_contract_digest(&mut evidence_members);
    summary.correlation_status = if approved == suites.len() {
        format!(
            "{} current suite{} approved",
            approved,
            if approved == 1 { "" } else { "s" }
        )
    } else {
        format!(
            "{approved}/{} current suite approvals retained",
            suites.len()
        )
    };
    if approved != suites.len() && summary.gate != QualificationGate::Blocked {
        summary.gate = QualificationGate::Review;
    }
}

fn qualification_domain_summaries(
    accumulators: BTreeMap<QualificationDomain, QualificationDomainAccumulator>,
) -> Vec<QualificationDomainSummary> {
    accumulators
        .into_iter()
        .map(|(domain, accumulated)| {
            let reference_coverage = if accumulated.references == 0 {
                "No retained references".to_owned()
            } else {
                let quantity_label = if accumulated.quantities.len() == 1 {
                    "quantity"
                } else {
                    "quantities"
                };
                format!(
                    "{} refs · {} {quantity_label}",
                    accumulated.references,
                    accumulated.quantities.len()
                )
            };
            let tolerance = match accumulated.tolerance_contracts.len() {
                0 => "not declared".to_owned(),
                1 => accumulated
                    .tolerance_contracts
                    .values()
                    .next()
                    .cloned()
                    .expect("one retained tolerance contract"),
                count => format!("{count} declared contracts · varies"),
            };
            let (disposition, tone) = if accumulated.open_dispositions > 0 {
                (
                    format!("{} open", accumulated.open_dispositions),
                    QualificationGate::Review,
                )
            } else if accumulated.evidenced_vectors < accumulated.vectors {
                (
                    format!(
                        "{} without evidence",
                        accumulated.vectors - accumulated.evidenced_vectors
                    ),
                    QualificationGate::Unqualified,
                )
            } else if accumulated.passing_vectors == accumulated.vectors {
                ("accepted".to_owned(), QualificationGate::Qualified)
            } else {
                (
                    format!(
                        "{} review",
                        accumulated.vectors - accumulated.passing_vectors
                    ),
                    QualificationGate::Review,
                )
            };
            QualificationDomainSummary {
                domain,
                vectors: accumulated.vectors,
                reference_coverage,
                tolerance,
                disposition,
                tone,
            }
        })
        .collect()
}

fn qualification_tolerance_key(absolute: f64, relative: f64) -> (u64, u64) {
    let canonical_bits = |value: f64| {
        if value == 0.0 {
            0.0_f64.to_bits()
        } else {
            value.to_bits()
        }
    };
    (canonical_bits(absolute), canonical_bits(relative))
}

fn qualification_tolerance_label(absolute: f64, relative: f64) -> String {
    match (absolute > 0.0, relative > 0.0) {
        (false, false) => "exact".to_owned(),
        (true, false) => format!("{absolute:.3e} absolute"),
        (false, true) => format!("{:.4}% relative", relative * 100.0),
        (true, true) => format!("{absolute:.3e} abs · {:.4}% rel", relative * 100.0),
    }
}

fn qualification_evidence_contract_digest(
    members: &mut Vec<(String, u64, crate::product::ContentDigest)>,
) -> Option<String> {
    if members.is_empty() {
        return None;
    }
    members.sort_by(|left, right| {
        left.0
            .to_ascii_lowercase()
            .cmp(&right.0.to_ascii_lowercase())
            .then_with(|| left.1.cmp(&right.1))
            .then_with(|| left.2.cmp(&right.2))
    });
    if let [(suite, revision, digest)] = members.as_slice() {
        return Some(format!(
            "{suite}@{revision} · {}",
            short_digest(&digest.to_string())
        ));
    }

    let mut hasher = Sha256::new();
    hasher.update(b"rspice:model-qualification-evidence-set:v1\0");
    let member_count = members.len();
    for (suite, revision, digest) in members.iter() {
        hasher.update((suite.len() as u64).to_le_bytes());
        hasher.update(suite.as_bytes());
        hasher.update(revision.to_le_bytes());
        hasher.update(digest.as_bytes());
    }
    let digest = crate::product::ContentDigest::from_bytes(hasher.finalize().into());
    Some(format!(
        "{} suites · {}",
        member_count,
        short_digest(&digest.to_string())
    ))
}

fn selected_qualification_summary<'a>(
    app: &RSpiceApp,
    summaries: &'a [QualificationModelSummary],
) -> Option<&'a QualificationModelSummary> {
    let library = app
        .state
        .model_library_manager
        .selected_library
        .as_deref()?;
    let model = app.state.workbench.selected_model.as_deref()?;
    summaries.iter().find(|summary| {
        summary.library.eq_ignore_ascii_case(library) && summary.model.eq_ignore_ascii_case(model)
    })
}

#[derive(Debug, Clone)]
struct QualificationSuiteTableEvent {
    key: String,
    review: bool,
}

fn qualification_suite_table(
    ui: &mut Ui,
    app: &RSpiceApp,
    summaries: &[QualificationModelSummary],
    size: Vec2,
) -> Option<QualificationSuiteTableEvent> {
    let t = Tokens::get(ui.ctx());
    let aggregate_gate = if summaries.is_empty() {
        QualificationGate::Unqualified
    } else if summaries
        .iter()
        .any(|summary| summary.gate == QualificationGate::Blocked)
    {
        QualificationGate::Blocked
    } else if summaries
        .iter()
        .any(|summary| summary.gate != QualificationGate::Qualified)
    {
        QualificationGate::Review
    } else {
        QualificationGate::Qualified
    };
    let columns = [
        ("Model family", 0.17),
        ("DC", 0.07),
        ("AC / charge", 0.12),
        ("Transient", 0.10),
        ("Noise", 0.08),
        ("Temperature", 0.14),
        ("References", 0.12),
        ("Gate", 0.10),
        ("", 0.10),
    ];
    let mut event = None;
    table_card(
        ui,
        "Qualification suites",
        Some((aggregate_gate.label(), aggregate_gate.color(&t))),
        size,
        |ui, table_size| {
            let table_size = table_size.max(Vec2::splat(1.0));
            let (viewport, _) = ui.allocate_exact_size(table_size, Sense::hover());
            ui.painter().rect_filled(viewport, 0.0, t.color.bg_panel);
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(viewport)
                    .layout(Layout::top_down(Align::Min)),
            );
            child.spacing_mut().item_spacing = Vec2::ZERO;
            let table_width = viewport.width().max(760.0);
            ScrollArea::both()
                .id_salt("models.qualification.suites")
                .auto_shrink([false, false])
                .show(&mut child, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    ui.set_min_width(table_width);
                    let (head, _) = ui
                        .allocate_exact_size(egui::vec2(table_width, TABLE_HEAD_H), Sense::hover());
                    ui.painter().rect_filled(head, 0.0, t.color.bg_panel_2);
                    ui.painter().hline(
                        head.x_range(),
                        head.bottom(),
                        Stroke::new(1.0, t.color.border),
                    );
                    paint_table_cells(ui, head, &columns, None, true);

                    for summary in summaries {
                        let selected = app.state.workbench.selected_model.as_deref()
                            == Some(summary.model.as_str())
                            && app.state.model_library_manager.selected_library.as_deref()
                                == Some(summary.library.as_str());
                        let cells = [
                            DataCell::mono(&summary.model),
                            DataCell::mono(summary.dc_vectors.to_string()),
                            DataCell::mono(summary.ac_vectors.to_string()),
                            DataCell::mono(summary.transient_vectors.to_string()),
                            DataCell::mono(summary.noise_vectors.to_string()),
                            DataCell::mono(if summary.temperature_points == 0 {
                                "not declared".to_owned()
                            } else {
                                format!("{} points", summary.temperature_points)
                            }),
                            DataCell::mono(format!("{} refs", summary.references)),
                            DataCell::mono_colored(summary.gate.label(), summary.gate.color(&t)),
                        ];
                        let row_height = t.metrics.row_h.max(30.0);
                        let (row, _) = ui.allocate_exact_size(
                            egui::vec2(table_width, row_height),
                            Sense::hover(),
                        );
                        let response = ui.interact(
                            row,
                            ui.id()
                                .with(("models.qualification.suite", summary.key.as_str())),
                            Sense::click(),
                        );
                        if selected {
                            ui.painter().rect_filled(row, 0.0, t.color.accent_dim);
                            ui.painter().rect_filled(
                                Rect::from_min_max(
                                    row.min,
                                    egui::pos2(row.left() + 2.0, row.bottom()),
                                ),
                                0.0,
                                t.color.accent,
                            );
                        } else if response.hovered() {
                            ui.painter().rect_filled(row, 0.0, t.color.bg_hover);
                        }
                        ui.painter().hline(
                            row.x_range(),
                            row.bottom(),
                            Stroke::new(1.0, t.color.border.gamma_multiply(0.75)),
                        );
                        paint_table_cells(ui, row, &columns, Some(&cells), false);

                        let action_left = row.left()
                            + row.width()
                                * columns[..columns.len() - 1]
                                    .iter()
                                    .map(|(_, fraction)| *fraction)
                                    .sum::<f32>();
                        let action_rect = Rect::from_min_max(
                            egui::pos2(action_left + 4.0, row.top() + 3.0),
                            egui::pos2(row.right() - 4.0, row.bottom() - 3.0),
                        );
                        let mut action_ui =
                            ui.new_child(egui::UiBuilder::new().max_rect(action_rect).layout(
                                Layout::centered_and_justified(egui::Direction::LeftToRight),
                            ));
                        let blocker = qualification_action_block_reason(
                            app,
                            Some(summary),
                            QualificationPageAction::ReviewVectors,
                        );
                        let action = Button::new(if summary.gate == QualificationGate::Qualified {
                            "Inspect"
                        } else {
                            "Review"
                        })
                        .enabled(blocker.is_none())
                        .show(&mut action_ui);
                        let action_clicked = action.clicked();
                        if let Some(reason) = blocker.as_deref() {
                            action.on_disabled_hover_text(reason);
                        }

                        if action_clicked || response.double_clicked() {
                            event = Some(QualificationSuiteTableEvent {
                                key: summary.key.clone(),
                                review: true,
                            });
                        } else if response.clicked() {
                            event = Some(QualificationSuiteTableEvent {
                                key: summary.key.clone(),
                                review: false,
                            });
                        }
                        theme::paint_focus_ring_outset(ui, &response, row);
                    }

                    if summaries.is_empty() {
                        let empty_height =
                            (viewport.height() - TABLE_HEAD_H).max(t.metrics.row_h.max(44.0));
                        let (empty, _) = ui.allocate_exact_size(
                            egui::vec2(table_width, empty_height),
                            Sense::hover(),
                        );
                        ui.painter().text(
                            empty.center(),
                            Align2::CENTER_CENTER,
                            "No loaded model records are available for qualification.",
                            theme::sans(tokens::FS_0, FontWeight::Regular),
                            t.color.text_dim,
                        );
                    }
                });
        },
    );
    event
}

fn qualification_detail(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    height: f32,
) -> Option<QualificationPageAction> {
    let width = ui.available_width().max(1.0);
    let narrow = width <= MODEL_SUMMARY_BREAKPOINT;
    let footer_height = qualification_gate_footer_height(ui, width);
    let body_height = (height - footer_height).max(1.0);
    let mut requested_action = None;

    if narrow {
        let domain_height = (body_height - 250.0).max(190.0).min(body_height);
        qualification_domain_table(ui, selected, egui::vec2(width, domain_height));
        let contract_height = (body_height - domain_height).max(1.0);
        if qualification_contract_card(ui, app, selected, egui::vec2(width, contract_height)) {
            requested_action = Some(QualificationPageAction::ReviewVectors);
        }
    } else {
        let (body, _) = ui.allocate_exact_size(egui::vec2(width, body_height), Sense::hover());
        let left_width = (body.width() * 0.61).floor();
        let left_rect = Rect::from_min_max(
            body.left_top(),
            egui::pos2(body.left() + left_width, body.bottom()),
        );
        let right_rect = Rect::from_min_max(
            egui::pos2(left_rect.right() + 1.0, body.top()),
            body.right_bottom(),
        );
        let mut left_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(left_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        left_ui.spacing_mut().item_spacing = Vec2::ZERO;
        qualification_domain_table(&mut left_ui, selected, left_rect.size());

        let mut right_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(right_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        right_ui.spacing_mut().item_spacing = Vec2::ZERO;
        if qualification_contract_card(&mut right_ui, app, selected, right_rect.size()) {
            requested_action = Some(QualificationPageAction::ReviewVectors);
        }
    }

    if qualification_gate_footer(ui, app, selected, egui::vec2(width, footer_height)) {
        requested_action = Some(QualificationPageAction::ReviewReleaseBinding);
    }
    requested_action
}

fn qualification_gate_footer_height(ui: &Ui, width: f32) -> f32 {
    if width > MODEL_SUMMARY_BREAKPOINT {
        return 58.0;
    }
    let t = Tokens::get(ui.ctx());
    let body = ui.painter().layout(
        QUALIFICATION_GATE_COPY.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (width - 22.0).max(1.0),
    );
    (7.0 + 18.0 + body.size().y + 8.0 + 30.0 + 8.0).max(96.0)
}

fn qualification_domain_table(
    ui: &mut Ui,
    selected: Option<&QualificationModelSummary>,
    size: Vec2,
) {
    let t = Tokens::get(ui.ctx());
    let rows = selected
        .into_iter()
        .flat_map(|selected| selected.domains.iter())
        .map(|domain| DataRow {
            key: domain.domain.label().to_owned(),
            selected: false,
            cells: vec![
                DataCell::plain(domain.domain.label()),
                DataCell::mono(domain.vectors.to_string()),
                DataCell::plain(&domain.reference_coverage),
                DataCell::mono(&domain.tolerance),
                DataCell::mono_colored(&domain.disposition, domain.tone.color(&t)),
            ],
        })
        .collect::<Vec<_>>();
    let columns = [
        ("Domain", 0.22),
        ("Vectors", 0.13),
        ("References", 0.20),
        ("Tolerance", 0.25),
        ("Disposition", 0.20),
    ];
    let title = selected.map_or("Selected qualification", |selected| selected.model.as_str());
    table_card(
        ui,
        title,
        selected.map(|selected| (selected.gate.label(), selected.gate.color(&t))),
        size,
        |ui, table_size| {
            let _ = data_table(
                ui,
                "models.qualification.domains",
                470.0,
                &columns,
                &rows,
                table_size,
                "No executable qualification domains are retained for the selected source revision.",
            );
        },
    );
}

fn qualification_contract_card(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    size: Vec2,
) -> bool {
    let (viewport, _) = ui.allocate_exact_size(size.max(Vec2::splat(1.0)), Sense::hover());
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing = Vec2::ZERO;
    let review_blocker =
        qualification_action_block_reason(app, selected, QualificationPageAction::ReviewVectors);
    let mut clicked = false;
    ScrollArea::vertical()
        .id_salt("models.qualification.contract")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_min_width(viewport.width());
            let rows = [
                (
                    "Model revision",
                    selected.map_or("not selected".to_owned(), |selected| {
                        selected.source_revision.clone()
                    }),
                ),
                (
                    "Source authority",
                    selected.map_or("select a model family".to_owned(), |selected| {
                        selected
                            .source_error
                            .clone()
                            .unwrap_or_else(|| "exact project-owned revision".to_owned())
                    }),
                ),
                (
                    "Runtime parity",
                    selected.map_or("not evaluated".to_owned(), |selected| {
                        format!(
                            "desktop {}/{} · WASM {}/{}",
                            selected.desktop_passing,
                            selected.vectors,
                            selected.wasm_passing,
                            selected.vectors
                        )
                    }),
                ),
                (
                    "Evidence set",
                    selected.map_or("not retained".to_owned(), |selected| {
                        selected
                            .evidence_digest
                            .clone()
                            .unwrap_or_else(|| "not retained".to_owned())
                    }),
                ),
                (
                    "Measurement correlation",
                    selected.map_or("not configured".to_owned(), |selected| {
                        selected.correlation_evidence_digest.as_ref().map_or_else(
                            || selected.correlation_status.clone(),
                            |digest| format!("{} · {digest}", selected.correlation_status),
                        )
                    }),
                ),
                (
                    "Approved releases",
                    selected.map_or("0".to_owned(), |selected| selected.releases.to_string()),
                ),
            ];
            property_card(ui, "Qualification contract", |ui| {
                for (label, value) in &rows {
                    property_row(ui, label, value);
                }
                let review = Button::new(selected.map_or("Review qualification", |selected| {
                    if selected.open_dispositions > 0 {
                        "Review dispositions"
                    } else if selected.vectors == 0 {
                        "Configure suite"
                    } else {
                        "Inspect vectors"
                    }
                }))
                .enabled(review_blocker.is_none())
                .show(ui);
                if let Some(reason) = review_blocker.as_deref() {
                    review.on_disabled_hover_text(reason);
                } else if review.clicked() {
                    clicked = true;
                }
            });
        });
    clicked
}

fn qualification_gate_footer(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    size: Vec2,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(size.max(Vec2::splat(1.0)), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    let stacked = rect.width() <= MODEL_SUMMARY_BREAKPOINT;
    let button_width = 150.0_f32.min((rect.width() - 22.0).max(1.0));
    let button_rect = if stacked {
        Rect::from_min_max(
            egui::pos2(rect.right() - button_width - 8.0, rect.bottom() - 38.0),
            egui::pos2(rect.right() - 8.0, rect.bottom() - 8.0),
        )
    } else {
        Rect::from_center_size(
            egui::pos2(rect.right() - button_width * 0.5 - 8.0, rect.center().y),
            egui::vec2(button_width, 30.0),
        )
    };
    let copy_rect = if stacked {
        Rect::from_min_max(
            egui::pos2(rect.left() + 11.0, rect.top() + 7.0),
            egui::pos2(rect.right() - 11.0, button_rect.top() - 8.0),
        )
    } else {
        Rect::from_min_max(
            egui::pos2(rect.left() + 11.0, rect.top() + 7.0),
            egui::pos2(button_rect.left() - 10.0, rect.bottom() - 7.0),
        )
    };
    let heading_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let body_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    ui.painter().text(
        copy_rect.left_top(),
        Align2::LEFT_TOP,
        "Gate ownership",
        heading_font,
        t.color.text,
    );
    let body = ui.painter().layout(
        QUALIFICATION_GATE_COPY.to_owned(),
        body_font,
        t.color.text_dim,
        copy_rect.width(),
    );
    ui.painter().galley(
        egui::pos2(copy_rect.left(), copy_rect.top() + 18.0),
        body,
        t.color.text_dim,
    );

    let blocker = qualification_action_block_reason(
        app,
        selected,
        QualificationPageAction::ReviewReleaseBinding,
    );
    let mut button_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(button_rect)
            .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
    );
    let response = Button::new("Review release binding")
        .enabled(blocker.is_none())
        .show(&mut button_ui);
    if let Some(reason) = blocker.as_deref() {
        response.on_disabled_hover_text(reason);
        false
    } else {
        response.clicked()
    }
}

fn qualification_action_block_reason(
    app: &RSpiceApp,
    selected: Option<&QualificationModelSummary>,
    action: QualificationPageAction,
) -> Option<String> {
    let Some(selected) = selected else {
        return Some("Select a model family first".to_owned());
    };
    if !app.state.project_lifecycle.project_open {
        return Some("Open a project before using model qualification".to_owned());
    }
    if selected.source_error.is_some() {
        return Some("Select an exact project-owned model revision".to_owned());
    }
    if app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .is_some()
    {
        return Some(
            "Finish or cancel the active model qualification run before changing workflows"
                .to_owned(),
        );
    }
    if let Some(draft) = app.state.workbench.model_editor.draft.as_ref() {
        let same_model = draft.library_name.eq_ignore_ascii_case(&selected.library)
            && draft.model_name.eq_ignore_ascii_case(&selected.model);
        if draft.is_dirty() && !same_model {
            return Some(format!(
                "Save or discard unsaved model candidate '{}/{}' first",
                draft.library_name, draft.model_name
            ));
        }
        if same_model
            && draft.definition_is_dirty()
            && matches!(
                action,
                QualificationPageAction::RunSuite | QualificationPageAction::CompareRelease
            )
        {
            return Some(
                "Save the changed model definition before running or comparing qualification"
                    .to_owned(),
            );
        }
    }
    match action {
        QualificationPageAction::ReviewVectors
        | QualificationPageAction::ReviewReleaseBinding
        | QualificationPageAction::OpenCorrelation => None,
        QualificationPageAction::RunSuite if app.state.workbench.safe_mode.project_read_only() => {
            Some("Qualification cannot run while the project is read-only".to_owned())
        }
        QualificationPageAction::RunSuite if selected.suites == 0 => {
            Some("Author at least one executable qualification suite first".to_owned())
        }
        QualificationPageAction::RunSuite => None,
        QualificationPageAction::CompareRelease if !selected.comparison_available => {
            Some("The selected model has no immutable approved release to compare".to_owned())
        }
        QualificationPageAction::CompareRelease => None,
    }
}

fn execute_qualification_action(app: &mut RSpiceApp, action: QualificationPageAction) {
    let summaries = qualification_summaries(app);
    let selected = selected_qualification_summary(app, &summaries);
    if let Some(reason) = qualification_action_block_reason(app, selected, action) {
        app.state.push_user_message(ConsoleMessage::warning(reason));
        return;
    }
    if action == QualificationPageAction::OpenCorrelation {
        if let Err(error) = app.state.workbench.navigate(
            SurfaceRoute::surface(SurfaceId::ModelCorrelation),
            RouteTransitionSource::User,
        ) {
            app.state.push_user_message(ConsoleMessage::warning(format!(
                "Measurement correlation cannot be opened: {error}"
            )));
        }
        return;
    }
    let Some(library) = app.state.model_library_manager.selected_library.clone() else {
        app.state.push_user_message(ConsoleMessage::warning(
            "Select a model family before opening qualification.",
        ));
        return;
    };
    let Some(model) = app.state.workbench.selected_model.clone() else {
        app.state.push_user_message(ConsoleMessage::warning(
            "Select a model family before opening qualification.",
        ));
        return;
    };
    if let Err(error) = model_editor::open_project_model(app, &library, &model) {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Qualification cannot be opened: {error}"
        )));
        return;
    }
    if let Err(error) = app.state.workbench.navigate(
        SurfaceRoute::surface(SurfaceId::ModelEditor),
        RouteTransitionSource::User,
    ) {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Qualification editor cannot be shown: {error}"
        )));
        return;
    }

    match action {
        QualificationPageAction::ReviewVectors => {
            app.state.workbench.model_editor.active_section = ModelEditorSection::Tests;
            app.state.workbench.model_editor.qualification_plan_open = true;
        }
        QualificationPageAction::ReviewReleaseBinding => {
            app.state.workbench.model_editor.active_section = ModelEditorSection::Release;
        }
        QualificationPageAction::RunSuite => {
            app.state.workbench.model_editor.active_section = ModelEditorSection::Tests;
            model_editor::validate_open_candidate(app);
            let command = Command::ModelRunQualificationTests;
            if command.is_enabled(app) {
                command.execute(app);
            } else if let CommandAvailability::Disabled(reason) = command.availability(app) {
                app.state.push_user_message(ConsoleMessage::warning(format!(
                    "Qualification cannot run: {reason}."
                )));
            }
        }
        QualificationPageAction::CompareRelease => {
            app.state.workbench.model_editor.active_section = ModelEditorSection::Release;
            let command = Command::ModelCompareRelease;
            if command.is_enabled(app) {
                command.execute(app);
            } else if let CommandAvailability::Disabled(reason) = command.availability(app) {
                app.state.push_user_message(ConsoleMessage::warning(format!(
                    "Approved-model comparison is unavailable: {reason}."
                )));
            }
        }
        QualificationPageAction::OpenCorrelation => {}
    }
}

fn surface_title(
    ui: &mut Ui,
    eyebrow: &str,
    title: &str,
    description: &str,
    has_actions: bool,
    actions: impl FnOnce(&mut Ui),
) {
    surface_title_with_action_reserve(ui, eyebrow, title, description, has_actions, 240.0, actions);
}

fn surface_title_with_action_reserve(
    ui: &mut Ui,
    eyebrow: &str,
    title: &str,
    description: &str,
    has_actions: bool,
    requested_action_reserve: f32,
    actions: impl FnOnce(&mut Ui),
) {
    let surface_width = ui.available_width();
    let narrow = model_title_actions_stack(surface_width)
        || (has_actions && requested_action_reserve > 300.0 && surface_width < 720.0);
    workspace_title_row(ui, |ui| {
        if narrow {
            ui.vertical(|ui| {
                model_heading(ui, eyebrow, title, description);
                if has_actions {
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(actions);
                }
            });
        } else {
            let width = ui.available_width().max(1.0);
            let action_reserve = if has_actions {
                requested_action_reserve.min(width * 0.5)
            } else {
                0.0
            };
            let heading_width = (width - action_reserve).max(1.0);
            let row_height =
                model_title_content_height(ui, heading_width, eyebrow, title, description);
            let (row, _) = ui.allocate_exact_size(egui::vec2(width, row_height), Sense::hover());
            let heading_rect = Rect::from_min_max(
                row.left_top(),
                egui::pos2((row.right() - action_reserve).max(row.left()), row.bottom()),
            );
            let mut heading_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(heading_rect)
                    .layout(Layout::top_down(Align::Min)),
            );
            model_heading(&mut heading_ui, eyebrow, title, description);
            if has_actions {
                let action_rect = Rect::from_min_max(
                    egui::pos2(heading_rect.right(), row.top()),
                    row.right_bottom(),
                );
                let mut action_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(action_rect)
                        .layout(Layout::right_to_left(Align::Center)),
                );
                actions(&mut action_ui);
            }
        }
    });
}

fn model_heading(ui: &mut Ui, eyebrow: &str, title: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    let eyebrow_response = ui.label(
        egui::RichText::new(eyebrow.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint),
    );
    accessible_model_text(ui, &eyebrow_response, eyebrow);
    ui.add_space(2.0);
    let title_response = ui.label(
        egui::RichText::new(title)
            .font(theme::sans(15.0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.ctx().accesskit_node_builder(title_response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title);
    });
    let description_response = ui.label(
        egui::RichText::new(description)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    accessible_model_text(ui, &description_response, description);
}

fn accessible_model_text(ui: &Ui, response: &egui::Response, text: &str) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(text);
    });
}

fn model_title_content_height(
    ui: &Ui,
    heading_width: f32,
    eyebrow: &str,
    title: &str,
    description: &str,
) -> f32 {
    let t = Tokens::get(ui.ctx());
    let heading_width = heading_width.max(1.0);
    let eyebrow = ui.painter().layout(
        eyebrow.to_uppercase(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
        heading_width,
    );
    let title = ui.painter().layout(
        title.to_owned(),
        theme::sans(15.0, FontWeight::SemiBold),
        t.color.text,
        heading_width,
    );
    let description = ui.painter().layout(
        description.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        heading_width,
    );
    (eyebrow.size().y + 2.0 + title.size().y + description.size().y).max(MODEL_TITLE_MIN_CONTENT_H)
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
    summary_height: f32,
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
        summary_height,
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

fn table_card(
    ui: &mut Ui,
    title: &str,
    status: Option<(&str, Color32)>,
    desired_size: Vec2,
    body: impl FnOnce(&mut Ui, Vec2),
) {
    let t = Tokens::get(ui.ctx());
    let desired_size = desired_size.max(Vec2::splat(1.0));
    let (viewport, response) = ui.allocate_exact_size(desired_size, Sense::hover());
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing = Vec2::ZERO;
    let (head, _) = child.allocate_exact_size(
        egui::vec2(viewport.width(), TABLE_CARD_HEAD_H),
        Sense::hover(),
    );
    child.painter().rect_filled(head, 0.0, t.color.bg_panel);
    child.painter().hline(
        head.x_range(),
        head.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    child.painter().hline(
        head.x_range(),
        head.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    child.painter().text(
        egui::pos2(head.left() + 11.0, head.center().y),
        Align2::LEFT_CENTER,
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    if let Some((status, color)) = status {
        child.painter().text(
            egui::pos2(head.right() - 11.0, head.center().y),
            Align2::RIGHT_CENTER,
            status,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            color,
        );
    }
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    body(
        &mut child,
        egui::vec2(
            viewport.width(),
            (viewport.height() - TABLE_CARD_HEAD_H).max(1.0),
        ),
    );
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
                let empty_height =
                    (viewport.height() - TABLE_HEAD_H).max(t.metrics.row_h.max(44.0));
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(table_width, empty_height), Sense::hover());
                let mut empty = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(rect)
                        .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
                );
                let response = empty.label(
                    egui::RichText::new(empty_message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                accessible_model_text(&empty, &response, empty_message);
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
    resolved: usize,
    missing_contracts: usize,
    invalid_documents: usize,
}

fn symbol_registry_header(ui: &mut Ui, resolved: usize) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, TABLE_CARD_HEAD_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 11.0, rect.center().y),
        Align2::LEFT_CENTER,
        "Symbol registry",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(rect.right() - 11.0, rect.center().y),
        Align2::RIGHT_CENTER,
        format!("{resolved} resolved"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.ok,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("Symbol registry, {resolved} resolved"),
        )
    });
}

fn symbol_summary(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    stats: SymbolStats,
    narrow: bool,
    summary_height: f32,
    parent_owns_scroll: bool,
) {
    let width = ui.available_width().max(1.0);
    let mut open_form_designer = false;
    if parent_owns_scroll {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            ui.set_min_width(width);
            let start = ui.cursor().top();
            symbol_summary_content(ui, width, narrow, stats, &mut open_form_designer);
            let consumed = ui.cursor().top() - start;
            if consumed < summary_height {
                ui.add_space(summary_height - consumed);
            }
        });
    } else {
        let viewport_size = egui::vec2(width, summary_height.max(1.0));
        let (viewport, _) = ui.allocate_exact_size(viewport_size, Sense::hover());
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(viewport)
                .layout(Layout::top_down(Align::Min)),
        );
        child.spacing_mut().item_spacing = Vec2::ZERO;
        ScrollArea::vertical()
            .id_salt("models.symbols.summary")
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                ui.set_min_width(viewport.width());
                symbol_summary_content(
                    ui,
                    viewport.width(),
                    narrow,
                    stats,
                    &mut open_form_designer,
                );
            });
    }
    if open_form_designer {
        open_symbol_parameter_form_dialog(&mut app.state);
    }
}

fn symbol_summary_content(
    ui: &mut Ui,
    width: f32,
    narrow: bool,
    stats: SymbolStats,
    open_form_designer: &mut bool,
) {
    let render_pin_contract = |ui: &mut Ui| {
        property_card(ui, "Pin contract", |ui| {
            let t = Tokens::get(ui.ctx());
            if stats.invalid_documents == 0 && stats.missing_contracts == 0 {
                property_row_toned(ui, "Electrical types", "validated", t.color.ok);
            } else {
                property_row_toned(ui, "Electrical types", "review required", t.color.err);
            }
            property_row(ui, "Hidden power pins", "forbidden");
            property_row_toned(ui, "Pin-order mismatch", "block netlist", t.color.err);
        });
    };
    let mut render_parameter_form = |ui: &mut Ui| {
        property_card(ui, "Parameter form", |ui| {
            let t = Tokens::get(ui.ctx());
            egui::Frame::new()
                .inner_margin(egui::Margin::same(11))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Typed engineering values, defaults, constraints, model inheritance and device-specific help are versioned with each symbol.")
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                    if Button::new("Open form designer").show(ui).clicked() {
                        *open_form_designer = true;
                    }
                });
        });
    };

    if narrow {
        render_pin_contract(ui);
        render_parameter_form(ui);
    } else {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            let column_w = ((width - 1.0) * 0.5).max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(column_w, 0.0),
                Layout::top_down(Align::Min),
                render_pin_contract,
            );
            ui.allocate_ui_with_layout(
                egui::vec2(column_w, 0.0),
                Layout::top_down(Align::Min),
                render_parameter_form,
            );
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct SummaryCardSpec<'a> {
    title: &'a str,
    rows: &'a [(&'a str, String)],
}

impl<'a> SummaryCardSpec<'a> {
    const fn new(title: &'a str, rows: &'a [(&'a str, String)]) -> Self {
        Self { title, rows }
    }
}

fn summary_cards(
    ui: &mut Ui,
    narrow: bool,
    summary_height: f32,
    parent_owns_scroll: bool,
    left: SummaryCardSpec<'_>,
    right: SummaryCardSpec<'_>,
) -> egui::Response {
    let width = ui.available_width().max(1.0);
    if parent_owns_scroll {
        // The enclosing table/summary composition is already the vertical
        // scroll owner. Render at natural height here so short workspaces do
        // not create a second, partially hidden scroll viewport at the
        // Console boundary.
        return ui
            .scope(|ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                ui.set_min_width(width);
                let start = ui.cursor().top();
                summary_cards_content(ui, width, narrow, left, right);
                let consumed = ui.cursor().top() - start;
                if consumed < summary_height {
                    ui.add_space(summary_height - consumed);
                }
            })
            .response;
    }

    let viewport_size = egui::vec2(ui.available_width().max(1.0), summary_height.max(1.0));
    let (viewport, response) = ui.allocate_exact_size(viewport_size, Sense::hover());
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    child.spacing_mut().item_spacing = Vec2::ZERO;
    ScrollArea::vertical()
        .id_salt(("models.summary-cards", left.title, right.title))
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_min_width(viewport.width());
            summary_cards_content(ui, viewport.width(), narrow, left, right);
        });
    response
}

fn summary_cards_content(
    ui: &mut Ui,
    width: f32,
    narrow: bool,
    left: SummaryCardSpec<'_>,
    right: SummaryCardSpec<'_>,
) {
    if narrow {
        property_card(ui, left.title, |ui| {
            for (label, value) in left.rows {
                property_row(ui, label, value);
            }
        });
        property_card(ui, right.title, |ui| {
            for (label, value) in right.rows {
                property_row(ui, label, value);
            }
        });
    } else {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            let column_w = ((width - 1.0) * 0.5).max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(column_w, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    property_card(ui, left.title, |ui| {
                        for (label, value) in left.rows {
                            property_row(ui, label, value);
                        }
                    });
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(column_w, 0.0),
                Layout::top_down(Align::Min),
                |ui| {
                    property_card(ui, right.title, |ui| {
                        for (label, value) in right.rows {
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
    let content_rect = viewport.shrink(26.0);
    if libraries.is_empty() {
        let mut empty = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(content_rect)
                .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
        );
        let message = "No loaded model libraries expose an include graph.";
        let response = empty.label(
            egui::RichText::new(message)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        accessible_model_text(&empty, &response, message);
        return;
    }
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(Layout::top_down(Align::Center)),
    );
    ScrollArea::both()
        .id_salt("models.include.graph")
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            ui.set_min_width(400.0);
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

fn symbol_parameter_form_label(definition: &ModelBoundSymbolDefinition) -> String {
    let sections = &definition.parameter_form.sections;
    let field_count = sections
        .iter()
        .map(|section| section.fields.len())
        .sum::<usize>();
    match sections.as_slice() {
        [] => "not defined".to_owned(),
        [section] if !section.label.trim().is_empty() => section.label.trim().to_owned(),
        _ => format!("{} sections · {field_count} fields", sections.len()),
    }
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

/// Cap on shipped-pack rows shown for one query.
///
/// The packs hold around 199,000 definitions and a two-character query matches
/// tens of thousands. The catalogue is a browser, so it shows a bounded window
/// and says so rather than trying to render the whole match set.
const PACK_ROW_LIMIT: usize = 200;

/// Rows for definitions found in the shipped packs rather than a loaded library.
///
/// These are catalogue entries, not parsed models: RSpice knows where each one
/// lives and nothing more until the deck includes it. They are labelled
/// "indexed" so the distinction survives into the table, and rows from a pack
/// whose redistribution is unestablished say so.
fn pack_catalog_rows(app: &RSpiceApp, query: &str, t: &Tokens) -> Vec<DataRow> {
    let hits = app
        .state
        .model_library_manager
        .search_pack_models(query, PACK_ROW_LIMIT);

    hits.into_iter()
        .map(|hit| {
            let (status, status_color) = if hit.redistributable {
                ("indexed", t.color.info)
            } else {
                ("indexed · unlicensed", t.color.warn)
            };
            let source = hit
                .source
                .as_ref()
                .and_then(|path| path.file_name())
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| "-".to_owned());

            DataRow {
                key: model_key(&hit.pack, &hit.name),
                // Never marked selected: nothing is loaded to select.
                selected: false,
                cells: vec![
                    DataCell::mono(&hit.name),
                    DataCell::plain(format!("{} · {}", hit.device, hit.kind)),
                    DataCell::mono(format!("{source}:{}", hit.line)),
                    DataCell::plain(&hit.pack_name),
                    DataCell::plain("not loaded"),
                    DataCell::mono_colored("not recorded", t.color.text_faint),
                    DataCell::mono_colored(status, status_color),
                ],
            }
        })
        .collect()
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
                "Qualification",
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
        assert_eq!(desktop.summary_height, MODEL_WIDE_SUMMARY_H);
        assert!(!desktop.owns_vertical_scroll);

        let short_desktop = model_table_summary_layout(egui::vec2(1_120.0, 240.0), false);
        assert!(!short_desktop.narrow);
        assert_eq!(short_desktop.table_height, MODEL_TABLE_MIN_H);
        assert!(short_desktop.owns_vertical_scroll);

        let narrow = model_table_summary_layout(egui::vec2(560.0, 500.0), false);
        assert!(narrow.narrow);
        assert_eq!(narrow.table_height, 200.0);
        assert_eq!(narrow.summary_height, MODEL_STACKED_SUMMARY_H);
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
    fn action_title_keeps_its_button_in_the_title_band_and_leaves_body_space() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut action_rect = None;
        let mut body_rect = None;
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_431.0, 560.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    surface_title(
                        ui,
                        "SYMBOLS, PINS & DEVICE FORMS",
                        "Symbol and component-definition manager",
                        "Bind graphical symbols and explicit terminal contracts.",
                        true,
                        |ui| action_rect = Some(ui.button("Create symbol").rect),
                    );
                    body_rect = Some(ui.label("BODY CONTENT").rect);
                });
            },
        );
        let action_rect = action_rect.expect("title action rendered");
        let body_rect = body_rect.expect("body rendered");

        assert!(
            action_rect.top() < 90.0,
            "action was pushed below title: {action_rect:?}"
        );
        assert!(
            body_rect.top() < 130.0,
            "title consumed the surface: {body_rect:?}"
        );
        assert!(body_rect.top() >= action_rect.bottom() - 1.0);
        assert!(!output.shapes.is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn complete_models_surface_keeps_action_pages_inside_the_title_band() {
        for (page, label, description, body_label) in [
            (
                ModelsPage::Symbols,
                "Create symbol",
                "Bind graphical symbols, terminals, parameter forms and model families without hiding netlist semantics.",
                "No symbol views are present in the loaded design libraries.",
            ),
            (
                ModelsPage::Include,
                "Collapse transitive",
                "Inspect ordered dependency resolution, captured paths, source pins, and cycle diagnostics.",
                "No loaded model libraries expose an include graph.",
            ),
        ] {
            for width in [1_431.0, 820.0, 720.0, 561.0] {
                let ctx = egui::Context::default();
                crate::ui::Theme::default().apply(&ctx);
                ctx.enable_accesskit();
                let mut app = RSpiceApp::test_instance();
                app.state.workbench.models_page = page;
                app.state.library_manager.clear();
                app.state.model_library_manager.clear();

                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 560.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default()
                            .frame(egui::Frame::NONE)
                            .show(ctx, |ui| show(ui, &mut app));
                    },
                );

                let nodes = output
                    .platform_output
                    .accesskit_update
                    .expect("models accessibility tree")
                    .nodes;
                let bounds = nodes
                    .iter()
                    .find(|(_, node)| {
                        node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
                    })
                    .and_then(|(_, node)| node.bounds())
                    .unwrap_or_else(|| panic!("missing {label} action"));
                assert!(
                    bounds.y1 <= 150.0,
                    "{label} escaped the models title band on {page:?} at {width}: {bounds:?}"
                );
                let description_bounds = nodes
                    .iter()
                    .find(|(_, node)| node.label() == Some(description))
                    .and_then(|(_, node)| node.bounds())
                    .unwrap_or_else(|| panic!("missing {description} title description"));
                let body_bounds = nodes
                    .iter()
                    .find(|(_, node)| node.label() == Some(body_label))
                    .and_then(|(_, node)| node.bounds())
                    .unwrap_or_else(|| panic!("missing {body_label} body state"));
                assert!(
                    body_bounds.y0 >= bounds.y1 - 1.0
                        && body_bounds.y0 >= description_bounds.y1 - 1.0
                        && body_bounds.y1 <= 560.0,
                    "{body_label} overlaps the title or leaves the visible body on {page:?} at {width}: action={bounds:?}, description={description_bounds:?}, body={body_bounds:?}"
                );
                assert!(
                    (body_bounds.y0 + body_bounds.y1) * 0.5 >= 220.0,
                    "{body_label} is stranded at the top of an otherwise empty table on {page:?} at {width}: {body_bounds:?}"
                );
            }
        }
    }

    #[test]
    fn summary_cards_reserve_exact_height_when_long_values_wrap() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut consumed = None;
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(825.0, 420.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let long_value = "C:/commercial/pdk/releases/current/models/process/sections/temperature-and-voltage-corner".to_owned();
                    let left = [
                        ("Resolved bindings", long_value.clone()),
                        ("Unresolved bindings", long_value.clone()),
                        ("Missing non-TT section", long_value.clone()),
                    ];
                    let right = [
                        ("Temperature", long_value.clone()),
                        ("Supply factor", long_value.clone()),
                        ("PDK search paths", long_value),
                    ];
                    let response = summary_cards(
                        ui,
                        false,
                        MODEL_WIDE_SUMMARY_H,
                        false,
                        SummaryCardSpec::new("Binding policy", &left),
                        SummaryCardSpec::new("Environment axes", &right),
                    );
                    consumed = Some(response.rect.height());
                });
            },
        );

        assert!((consumed.expect("summary rendered") - MODEL_WIDE_SUMMARY_H).abs() <= 0.5);
    }

    #[test]
    fn parent_scrolled_summary_uses_natural_height_instead_of_nesting_scrollbars() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut consumed = None;
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(620.0, 260.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let long = "C:/commercial/pdk/releases/current/models/process/sections/temperature-and-voltage-corner".to_owned();
                    let left = [
                        ("Resolved bindings", long.clone()),
                        ("Unresolved bindings", long.clone()),
                        ("Missing non-TT section", long.clone()),
                    ];
                    let right = [
                        ("Temperature", long.clone()),
                        ("Supply factor", long.clone()),
                        ("PDK search paths", long),
                    ];
                    consumed = Some(
                        summary_cards(
                            ui,
                            false,
                            MODEL_WIDE_SUMMARY_H,
                            true,
                            SummaryCardSpec::new("Binding policy", &left),
                            SummaryCardSpec::new("Environment axes", &right),
                        )
                        .rect
                        .height(),
                    );
                });
            },
        );

        assert!(
            consumed.expect("summary rendered") > MODEL_WIDE_SUMMARY_H,
            "natural content must expand inside the single parent scroll owner"
        );
    }

    #[test]
    fn qualification_tab_uses_the_mockup_contract_label() {
        assert_eq!(ModelsPage::Qualification.label(), "Qualification");
        assert_eq!(
            Command::ModelsPage(ModelsPage::Qualification).stable_id(),
            "model-qualification"
        );
        assert!(QUALIFICATION_MIN_CONTENT_H > 600.0);
        assert_eq!(
            qualification_required_content_height(MODEL_SUMMARY_BREAKPOINT),
            QUALIFICATION_STACKED_MIN_CONTENT_H
        );
        assert_eq!(
            qualification_required_content_height(MODEL_SUMMARY_BREAKPOINT + 1.0),
            QUALIFICATION_MIN_CONTENT_H
        );
    }

    #[test]
    fn qualification_evidence_set_digest_is_order_independent_and_suite_qualified() {
        let first = crate::product::ContentDigest::from_bytes([0x11; 32]);
        let second = crate::product::ContentDigest::from_bytes([0x22; 32]);
        let mut one = vec![("dc".to_owned(), 3, first)];
        let one_label =
            qualification_evidence_contract_digest(&mut one).expect("single digest label");
        assert!(one_label.starts_with("dc@3 · "));

        let mut forward = vec![
            ("transient".to_owned(), 4, second),
            ("dc".to_owned(), 3, first),
        ];
        let mut reverse = forward.iter().cloned().rev().collect::<Vec<_>>();
        let forward_label =
            qualification_evidence_contract_digest(&mut forward).expect("aggregate digest");
        let reverse_label =
            qualification_evidence_contract_digest(&mut reverse).expect("aggregate digest");
        assert_eq!(forward_label, reverse_label);
        assert!(forward_label.starts_with("2 suites · "));
    }

    #[test]
    fn qualification_domain_projection_never_invents_oracle_provenance() {
        let mut quantities = BTreeSet::new();
        quantities.insert("v(out)".to_owned());
        let domains = qualification_domain_summaries(BTreeMap::from([(
            QualificationDomain::Ac,
            QualificationDomainAccumulator {
                vectors: 2,
                references: 2,
                quantities,
                tolerance_contracts: BTreeMap::from([(
                    qualification_tolerance_key(1.0e-6, 0.005),
                    qualification_tolerance_label(1.0e-6, 0.005),
                )]),
                evidenced_vectors: 1,
                passing_vectors: 1,
                open_dispositions: 0,
            },
        )]));

        assert_eq!(domains.len(), 1);
        assert_eq!(domains[0].domain, QualificationDomain::Ac);
        assert_eq!(domains[0].reference_coverage, "2 refs · 1 quantity");
        assert_eq!(domains[0].disposition, "1 without evidence");
        assert_eq!(domains[0].tone, QualificationGate::Unqualified);
        assert!(
            !domains[0]
                .reference_coverage
                .to_ascii_lowercase()
                .contains("vendor")
        );
        assert!(
            !domains[0]
                .reference_coverage
                .to_ascii_lowercase()
                .contains("oracle")
        );
    }

    #[test]
    fn qualification_domain_projection_preserves_distinct_tolerance_contracts() {
        let domains = qualification_domain_summaries(BTreeMap::from([(
            QualificationDomain::Dc,
            QualificationDomainAccumulator {
                vectors: 2,
                references: 2,
                quantities: BTreeSet::from(["v(out)".to_owned()]),
                tolerance_contracts: BTreeMap::from([
                    (
                        qualification_tolerance_key(1.0001e-6, 0.0),
                        qualification_tolerance_label(1.0001e-6, 0.0),
                    ),
                    (
                        qualification_tolerance_key(1.0002e-6, 0.0),
                        qualification_tolerance_label(1.0002e-6, 0.0),
                    ),
                ]),
                evidenced_vectors: 0,
                passing_vectors: 0,
                open_dispositions: 0,
            },
        )]));

        assert_eq!(
            qualification_tolerance_label(1.0001e-6, 0.0),
            qualification_tolerance_label(1.0002e-6, 0.0)
        );
        assert_eq!(domains[0].tolerance, "2 declared contracts · varies");
    }

    #[test]
    fn qualification_footer_reserves_wrapped_copy_before_stacking_its_action() {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut heights = None;
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 300.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    heights = Some((
                        qualification_gate_footer_height(ui, 460.0),
                        qualification_gate_footer_height(ui, 900.0),
                    ));
                });
            },
        );
        let (stacked, wide) = heights.expect("footer heights measured");
        assert!(stacked > wide);
        assert_eq!(wide, 58.0);
        assert!(stacked >= 96.0);
    }

    #[test]
    fn project_model_without_suites_is_truthfully_unqualified() {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let definition = crate::state::model_library::ProjectModelDefinition {
            name: "nch_owned".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Project-owned qualification fixture".to_owned(),
            numeric_parameters: std::collections::BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("vth0".to_owned(), 0.48),
            ]),
            string_parameters: std::collections::BTreeMap::new(),
        };
        app.state
            .model_library_manager
            .create_project_model("owned-models", &definition)
            .expect("create project model");

        let summary = qualification_summaries(&app)
            .into_iter()
            .find(|summary| summary.model == "nch_owned")
            .expect("qualification summary");

        assert!(summary.source_error.is_none());
        assert_eq!(summary.gate, QualificationGate::Unqualified);
        assert_eq!(summary.suites, 0);
        assert_eq!(summary.vectors, 0);
        assert_eq!(summary.passing_vectors, 0);
        assert!(summary.evidence_digest.is_none());

        app.state
            .model_library_manager
            .select_library(&summary.library);
        app.state.workbench.selected_model = Some(summary.model.clone());
        assert_eq!(
            qualification_action_block_reason(
                &app,
                Some(&summary),
                QualificationPageAction::RunSuite
            )
            .as_deref(),
            Some("Author at least one executable qualification suite first")
        );
        assert_eq!(
            qualification_action_block_reason(
                &app,
                Some(&summary),
                QualificationPageAction::ReviewVectors
            ),
            None
        );
        execute_qualification_action(&mut app, QualificationPageAction::ReviewVectors);
        assert_eq!(
            app.state.workbench.model_editor.active_section,
            ModelEditorSection::Tests
        );
        assert!(app.state.workbench.model_editor.qualification_plan_open);
        assert_eq!(
            app.state.workbench.current_route().surface_id(),
            SurfaceId::ModelEditor
        );

        let editor = &mut app.state.workbench.model_editor;
        editor.begin_qualification_suite();
        let authoring = &mut editor.qualification_authoring;
        authoring.suite_id = "dc-op".to_owned();
        authoring.suite_name = "DC operating point".to_owned();
        authoring.vector_id = "nominal".to_owned();
        authoring.vector_name = "Nominal bias".to_owned();
        authoring.executable_input =
            "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.op\n.end\n".to_owned();
        authoring.quantity = "v(out)".to_owned();
        authoring.probe_target = "out".to_owned();
        authoring.expected = "1".to_owned();
        authoring.absolute_tolerance = "1e-9".to_owned();
        authoring.relative_tolerance = "1e-9".to_owned();
        assert!(
            editor.commit_qualification_suite(),
            "{:?}",
            editor.qualification_authoring.error
        );

        let working_summary = qualification_summaries(&app)
            .into_iter()
            .find(|summary| summary.model == "nch_owned")
            .expect("working qualification summary");
        assert_eq!(working_summary.suites, 1);
        assert_eq!(working_summary.vectors, 1);
        assert!(
            working_summary
                .source_revision
                .ends_with("· working qualification")
        );
        assert_eq!(
            qualification_action_block_reason(
                &app,
                Some(&working_summary),
                QualificationPageAction::RunSuite
            ),
            None
        );
    }

    #[test]
    fn non_project_model_never_receives_synthetic_qualification_evidence() {
        let app = RSpiceApp::test_instance();
        let library = ModelLibrary::new("built-in");
        let model = DeviceModel::new(
            "builtin_resistor",
            crate::state::model_library::ModelType::Resistor,
        );

        let summary = qualification_model_summary(&app, &library, &model);

        assert!(summary.source_error.is_some());
        assert_eq!(summary.gate, QualificationGate::Blocked);
        assert_eq!(summary.vectors, 0);
        assert_eq!(summary.passing_vectors, 0);
        assert!(summary.evidence_digest.is_none());
    }

    #[test]
    fn configured_correlation_requires_current_approved_evidence_for_qualification() {
        use crate::state::model_library::{
            CorrelationDatasetClass, CorrelationDatasetRevision, CorrelationSuite,
        };

        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager = crate::state::model_library::ModelLibraryManager::new();
        let definition = crate::state::model_library::ProjectModelDefinition {
            name: "nch_correlated".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Correlation handoff fixture".to_owned(),
            numeric_parameters: BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("vth0".to_owned(), 0.48),
            ]),
            string_parameters: BTreeMap::new(),
        };
        app.state
            .model_library_manager
            .create_project_model("owned-models", &definition)
            .unwrap();
        let resolved = model_editor::resolve_project_model_for_editor(
            &app.state.model_library_manager,
            "owned-models",
            "nch_correlated",
        )
        .unwrap();
        let source = ModelSourceEvidenceBinding::try_new_project_bound(
            "nch_correlated",
            resolved.source_id,
            resolved.model_digest,
            resolved.model_revision,
        )
        .unwrap();
        let dataset = CorrelationDatasetRevision::try_from_csv(
            "bench",
            crate::product::ObjectRevision::INITIAL,
            "Bench",
            CorrelationDatasetClass::BenchMeasurement,
            "lab",
            "lot-1",
            "fixture-1",
            "calibration-1",
            "bench.csv",
            b"id,quantity,value,unit\np1,V(out),1,V\n".to_vec(),
            None,
        )
        .unwrap();
        let suite = CorrelationSuite::try_new(
            "bench-correlation",
            crate::product::ObjectRevision::INITIAL,
            "Bench correlation",
            "model-owner",
            source.clone(),
            vec![dataset],
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let correlation = ModelCorrelationState::try_new(vec![suite], Vec::new()).unwrap();
        let library = app
            .state
            .model_library_manager
            .get_library("owned-models")
            .unwrap();
        let model = library.models.get("nch_correlated").unwrap();
        let mut summary = qualification_model_summary(&app, library, model);
        summary.gate = QualificationGate::Qualified;

        apply_correlation_qualification_contract(&mut summary, Some(&correlation), Some(&source));

        assert_eq!(summary.gate, QualificationGate::Review);
        assert_eq!(
            summary.correlation_status,
            "0/1 current suite approvals retained"
        );
        assert!(summary.correlation_evidence_digest.is_none());
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
