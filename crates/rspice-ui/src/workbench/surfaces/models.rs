//! Model catalog, symbol contracts, PDK sections, authenticated includes, and
//! the source-owned model qualification and release gate.

mod bins;
mod catalog;
mod corners;
mod include_manifest;
mod qualification;

use bins::*;
use catalog::*;
use corners::*;
use include_manifest::*;
use qualification::*;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

use egui::{Align, Align2, Color32, Key, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};
use sha2::{Digest as _, Sha256};

use crate::diagnostics::ConsoleMessage;
use crate::state::model_library::{
    DeviceModel, ModelBinAuditAxisRange, ModelBinAuditDraft, ModelBinAuditFinding,
    ModelBinAuditFindingKind, ModelBinAuditReceipt, ModelCorrelationState, ModelDefinitionConflict,
    ModelLibrary, ModelQualificationState, ModelSourceEvidenceBinding, ModelSubcircuitInterface,
    PackModelHit, QualificationAnalysis, QualificationPlatform,
};
use crate::state::{CellViewRef, ModelBoundSymbolDefinition, SymbolDocument, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;
use crate::workbench::app::{
    open_create_model_bound_symbol_dialog, open_create_subcircuit_bound_symbol_dialog,
    open_symbol_import_dialog, open_symbol_parameter_form_dialog,
};

use super::super::design_system::{
    property_card, property_row, property_row_toned, workspace_title_row,
};
use super::super::state::{ModelCatalogScope, ModelProjectFacet, ModelsPage, Workspace};
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use crate::workbench::commands::{CommandAvailability, vocabulary::Command};
use crate::workbench::documents::model_editor::{self, ModelEditorSection};

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
        ModelsPage::Bins => bins(&mut surface, app),
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
    // Models owns a scope-aware query field below this tab strip. A second
    // global field here would conflate project filtering with pack discovery.
    let filter_visible = false;
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

fn include_graph(ui: &mut Ui, app: &mut RSpiceApp) {
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
    let selection_is_valid = app
        .state
        .workbench
        .model_include_selected_library
        .as_deref()
        .zip(app.state.workbench.model_include_selected_source.as_deref())
        .is_some_and(|(selected_library, selected_source)| {
            libraries.iter().any(|library| {
                library.name == selected_library
                    && (library.root.as_deref() == Some(selected_source)
                        || library
                            .sources
                            .iter()
                            .any(|(path, _)| path == selected_source))
            })
        });
    if !selection_is_valid {
        app.state.workbench.model_include_selected_library = None;
        app.state.workbench.model_include_selected_source = None;
    }
    let mut direct_only = app.state.workbench.model_include_direct_only;
    surface_title_with_action_reserve(
        ui,
        "Include resolution · content addressed",
        "Model include graph",
        "Inspect ordered dependency resolution, captured paths, source pins, and cycle diagnostics.",
        true,
        360.0,
        |ui| {
            let label = if direct_only {
                "Expand transitive"
            } else {
                "Collapse transitive"
            };
            if Button::new(label).show(ui).clicked() {
                direct_only = !direct_only;
                app.state.workbench.model_include_direct_only = direct_only;
            }
            if Button::new("Export manifest")
                .accent()
                .enabled(!libraries.is_empty())
                .show(ui)
                .clicked()
            {
                match export_model_dependency_manifest(app) {
                    Ok(Some(receipt)) => app.state.ui.toasts.success(
                        ui.ctx(),
                        "Dependency manifest exported",
                        receipt,
                    ),
                    Ok(None) => {}
                    Err(error) => {
                        app.state.push_user_message(ConsoleMessage::error(format!(
                            "Model dependency manifest export failed: {error}"
                        )));
                        app.state.ui.toasts.error_with_title(
                            ui.ctx(),
                            "Dependency manifest export failed",
                            error,
                        );
                    }
                }
            }
        },
    );
    let diagnostics = include_diagnostics(app);
    include_graph_content(ui, app, &libraries, &diagnostics, direct_only);
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
                    if ui.input(|input| {
                        input.key_pressed(Key::Enter) || input.key_pressed(Key::Space)
                    }) {
                        event = Some(TableEvent {
                            key: row.key.clone(),
                            activate: true,
                        });
                    }
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
    conflicts: Vec<ModelDefinitionConflict>,
    resolution_error: Option<String>,
    cyclic_nodes: usize,
    unpinned_roots: usize,
}

fn include_diagnostics(app: &RSpiceApp) -> IncludeDiagnostics {
    let conflicts = app.state.model_library_manager.definition_conflicts();
    let resolution_error = app
        .state
        .model_library_manager
        .validate_definition_resolution()
        .err();
    let libraries = app.state.model_library_manager.libraries_sorted();
    let mut sources = HashSet::<PathBuf>::new();
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
    }
    let definitions = libraries
        .iter()
        .map(|library| library.models.len() + library.subcircuits.len())
        .sum();
    let duplicate_models: usize = conflicts
        .iter()
        .map(|conflict| conflict.providers.len().saturating_sub(1))
        .sum();
    let mut active_subcircuit_names = HashMap::<String, usize>::new();
    for library in &libraries {
        for subcircuit in library.subcircuits.values().filter(|subcircuit| {
            subcircuit.section.is_none()
                || subcircuit.section.as_deref() == library.selected_corner.as_deref()
        }) {
            *active_subcircuit_names
                .entry(subcircuit.name.to_ascii_lowercase())
                .or_default() += 1;
        }
    }
    let duplicate_subcircuits = active_subcircuit_names
        .values()
        .map(|count| count.saturating_sub(1))
        .sum::<usize>();
    IncludeDiagnostics {
        files: sources.len(),
        definitions,
        edges: edges.len(),
        duplicate_definitions: duplicate_models + duplicate_subcircuits,
        conflicts,
        resolution_error,
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
    app: &mut RSpiceApp,
    libraries: &[IncludeLibrary],
    diagnostics: &IncludeDiagnostics,
    collapsed: bool,
) {
    let size = ui.available_size();
    let narrow = size.x <= 1020.0;
    let selected_library = app.state.workbench.model_include_selected_library.clone();
    let selected_source = app.state.workbench.model_include_selected_source.clone();
    let mut requested_selection = None;
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
                    |ui| {
                        draw_include_graph(
                            ui,
                            libraries,
                            collapsed,
                            selected_library.as_deref(),
                            selected_source.as_deref(),
                            &mut requested_selection,
                        )
                    },
                );
                draw_include_diagnostics(ui, app, diagnostics);
            });
    } else {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            let graph_w = ((size.x - 1.0) * 0.695).max(1.0);
            let detail_w = (size.x - graph_w - 1.0).max(1.0);
            ui.allocate_ui_with_layout(
                egui::vec2(graph_w, size.y),
                Layout::top_down(Align::Min),
                |ui| {
                    draw_include_graph(
                        ui,
                        libraries,
                        collapsed,
                        selected_library.as_deref(),
                        selected_source.as_deref(),
                        &mut requested_selection,
                    )
                },
            );
            ui.allocate_ui_with_layout(
                egui::vec2(detail_w, size.y),
                Layout::top_down(Align::Min),
                |ui| {
                    ScrollArea::vertical()
                        .id_salt("models.include.diagnostics")
                        .auto_shrink([false, false])
                        .show(ui, |ui| draw_include_diagnostics(ui, app, diagnostics));
                },
            );
        });
    }
    if let Some((library, source)) = requested_selection {
        app.state.workbench.model_include_selected_library = Some(library);
        app.state.workbench.model_include_selected_source = Some(source);
    }
}

fn draw_include_graph(
    ui: &mut Ui,
    libraries: &[IncludeLibrary],
    collapsed: bool,
    selected_library: Option<&str>,
    selected_source: Option<&Path>,
    requested_selection: &mut Option<(String, PathBuf)>,
) {
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
                let root_selected = selected_library == Some(library.name.as_str())
                    && selected_source == library.root.as_deref();
                let root_response = include_node(
                    ui,
                    &root_label,
                    &format!("{} · {} pinned files", library.name, library.sources.len()),
                    true,
                    root_selected,
                );
                if root_response.clicked()
                    && let Some(root) = library.root.clone()
                {
                    *requested_selection = Some((library.name.clone(), root));
                }
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
                            let selected = selected_library == Some(library.name.as_str())
                                && selected_source == Some(target.as_path());
                            let response = include_node(
                                ui,
                                &path_display_name(target),
                                &format!("{requested} · {digest}"),
                                false,
                                selected,
                            );
                            if response.clicked() {
                                *requested_selection = Some((library.name.clone(), target.clone()));
                            }
                        }
                    });
                }
                ui.add_space(22.0);
            }
        });
}

fn include_node(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    root: bool,
    selected: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let width: f32 = if root { 360.0 } else { 230.0 };
    let width = width.min(ui.available_width().max(180.0));
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 52.0), Sense::click());
    let accessible_label = format!("{title}: {detail}");
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            selected,
            &accessible_label,
        )
    });
    ui.painter().rect(
        rect,
        t.radius,
        if selected {
            t.color.bg_active
        } else if root {
            t.color.accent_dim
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_panel
        },
        Stroke::new(
            if selected { 2.0 } else { 1.0 },
            if selected || root {
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
    response
}

#[derive(Debug, Clone)]
struct IncludeSelectionDetail {
    library: String,
    path: PathBuf,
    digest: String,
    authority: &'static str,
    byte_length: usize,
    root: bool,
    incoming_edges: usize,
    outgoing_edges: usize,
    definitions: usize,
}

fn selected_include_source_detail(app: &RSpiceApp) -> Option<IncludeSelectionDetail> {
    let library_name = app
        .state
        .workbench
        .model_include_selected_library
        .as_deref()?;
    let path = app
        .state
        .workbench
        .model_include_selected_source
        .as_deref()?;
    let library = app.state.model_library_manager.get_library(library_name)?;
    let source = library
        .source_closure
        .iter()
        .find(|source| source.path == path)?;
    let authority = match library.source_authority {
        crate::state::model_library::ModelSourceAuthority::BuiltIn => "built-in metadata",
        crate::state::model_library::ModelSourceAuthority::External => {
            "external · reauthenticated at run"
        }
        crate::state::model_library::ModelSourceAuthority::RetainedImport { .. } => {
            "retained import · digest checked"
        }
        crate::state::model_library::ModelSourceAuthority::ProjectOwned { .. } => {
            "project owned · revision sealed"
        }
    };
    let byte_length = library
        .source_contents
        .iter()
        .find(|content| content.path == path)
        .map_or(0, |content| content.bytes.len());
    let definitions = library
        .models
        .values()
        .filter(|model| model.file_path.as_deref() == Some(path))
        .count()
        + library
            .subcircuits
            .values()
            .filter(|subcircuit| subcircuit.file_path.as_deref() == Some(path))
            .count();
    Some(IncludeSelectionDetail {
        library: library.name.clone(),
        path: path.to_path_buf(),
        digest: source.digest.to_string(),
        authority,
        byte_length,
        root: library.root_path.as_deref() == Some(path),
        incoming_edges: library
            .source_edges
            .iter()
            .filter(|edge| edge.target == path)
            .count(),
        outgoing_edges: library
            .source_edges
            .iter()
            .filter(|edge| edge.owner == path)
            .count(),
        definitions,
    })
}

fn draw_include_selection(ui: &mut Ui, app: &mut RSpiceApp) {
    let Some(detail) = selected_include_source_detail(app) else {
        return;
    };
    property_card(ui, "Selected source", |ui| {
        property_row(ui, "Library", &detail.library);
        property_row(
            ui,
            "File",
            &if detail.root {
                format!("{} · root", path_display_name(&detail.path))
            } else {
                path_display_name(&detail.path)
            },
        );
        property_row(ui, "Authority", detail.authority);
        property_row(ui, "SHA-256", &short_digest(&detail.digest));
        let byte_length = (detail.byte_length > 0)
            .then(|| format!("{} bytes", detail.byte_length))
            .unwrap_or_else(|| "not retained".to_owned());
        property_row(ui, "Retained bytes", &byte_length);
        property_row(
            ui,
            "Graph degree",
            &format!(
                "{} incoming · {} outgoing",
                detail.incoming_edges, detail.outgoing_edges
            ),
        );
        property_row(ui, "Definitions", &detail.definitions.to_string());
        ui.add_space(5.0);
        ui.horizontal_wrapped(|ui| {
            if Button::new("Copy source path").show(ui).clicked() {
                ui.ctx().copy_text(detail.path.display().to_string());
                app.state.push_user_message(ConsoleMessage::info(format!(
                    "Copied model source path '{}'.",
                    detail.path.display()
                )));
            }
            if Button::new("Browse definitions")
                .enabled(detail.definitions > 0)
                .show(ui)
                .clicked()
            {
                app.state.model_library_manager.filter_text = path_display_name(&detail.path);
                app.state.workbench.model_catalog_scope = ModelCatalogScope::Project;
                app.state.workbench.model_project_facet = ModelProjectFacet::All;
                app.state.workbench.models_page = ModelsPage::Models;
            }
        });
    });
    ui.add_space(8.0);
}

#[derive(Debug, Clone)]
struct IncludeDefinitionRow {
    name: String,
    kind: &'static str,
    library: String,
    source: String,
    section: String,
    status: String,
    exception: bool,
    resolved: bool,
}

fn include_definition_rows(app: &RSpiceApp) -> Vec<IncludeDefinitionRow> {
    let libraries = app.state.model_library_manager.libraries_sorted();
    let mut multiplicity = HashMap::<(String, &'static str), usize>::new();
    for library in &libraries {
        for model in library.models.values() {
            *multiplicity
                .entry((model.name.to_ascii_lowercase(), "model"))
                .or_default() += 1;
        }
        for subcircuit in library.subcircuits.values() {
            let active = subcircuit.section.is_none()
                || subcircuit.section.as_deref() == library.selected_corner.as_deref();
            if active {
                *multiplicity
                    .entry((subcircuit.name.to_ascii_lowercase(), "subcircuit"))
                    .or_default() += 1;
            }
        }
    }

    let mut rows = Vec::new();
    for library in libraries {
        for model in library.models.values() {
            let normalized = model.name.to_ascii_lowercase();
            let duplicate = multiplicity
                .get(&(normalized.clone(), "model"))
                .copied()
                .unwrap_or(0)
                > 1;
            let selection = app
                .state
                .model_library_manager
                .definition_resolution(&normalized);
            let selected = selection.is_some_and(|resolution| {
                resolution.provider_library == library.name
                    && resolution.provider_model == model.name
            });
            let status = if !duplicate {
                "unique provider".to_owned()
            } else if selected {
                "explicit winning provider".to_owned()
            } else if selection.is_some() {
                "non-selected candidate".to_owned()
            } else {
                "contested · execution blocked".to_owned()
            };
            rows.push(IncludeDefinitionRow {
                name: model.name.clone(),
                kind: "model",
                library: library.name.clone(),
                source: model
                    .file_path
                    .as_deref()
                    .map(path_display_name)
                    .unwrap_or_else(|| "in-memory".to_owned()),
                section: model
                    .section
                    .clone()
                    .unwrap_or_else(|| "top level".to_owned()),
                status,
                exception: duplicate,
                resolved: !duplicate || selection.is_some(),
            });
        }
        for subcircuit in library.subcircuits.values() {
            let active = subcircuit.section.is_none()
                || subcircuit.section.as_deref() == library.selected_corner.as_deref();
            let duplicate = active
                && multiplicity
                    .get(&(subcircuit.name.to_ascii_lowercase(), "subcircuit"))
                    .copied()
                    .unwrap_or(0)
                    > 1;
            rows.push(IncludeDefinitionRow {
                name: subcircuit.name.clone(),
                kind: "subcircuit",
                library: library.name.clone(),
                source: subcircuit
                    .file_path
                    .as_deref()
                    .map(path_display_name)
                    .unwrap_or_else(|| "in-memory".to_owned()),
                section: subcircuit
                    .section
                    .clone()
                    .unwrap_or_else(|| "top-level".to_owned()),
                status: if !active {
                    "addressable section · inactive".to_owned()
                } else if duplicate {
                    "duplicate interface · execution review required".to_owned()
                } else {
                    "unique provider".to_owned()
                },
                exception: duplicate,
                resolved: !duplicate,
            });
        }
    }
    rows.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
            .then_with(|| left.kind.cmp(right.kind))
            .then_with(|| left.library.cmp(&right.library))
            .then_with(|| left.section.cmp(&right.section))
    });
    rows
}

fn draw_include_definition_resolution(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let rows = include_definition_rows(app);
    let exception_count = rows.iter().filter(|row| row.exception).count();
    property_card(ui, "Definition resolution", |ui| {
        let mut exceptions_only = app.state.workbench.model_include_exceptions_only;
        let mut query = app.state.workbench.model_include_definition_query.clone();
        ui.horizontal_wrapped(|ui| {
            if ui
                .selectable_label(!exceptions_only, "All definitions")
                .clicked()
            {
                exceptions_only = false;
            }
            if ui
                .selectable_label(exceptions_only, format!("Exceptions ({exception_count})"))
                .clicked()
            {
                exceptions_only = true;
            }
        });
        ui.add_space(4.0);
        let response = ui.add(
            egui::TextEdit::singleline(&mut query)
                .hint_text("Filter definition, provider, source, or section…")
                .desired_width(f32::INFINITY),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::TextEdit,
                ui.is_enabled(),
                "Filter include-graph definitions",
            )
        });
        app.state.workbench.model_include_exceptions_only = exceptions_only;
        app.state.workbench.model_include_definition_query = query.clone();

        let query = query.trim().to_ascii_lowercase();
        let matching = rows
            .iter()
            .filter(|row| !exceptions_only || row.exception)
            .filter(|row| {
                query.is_empty()
                    || [
                        row.name.as_str(),
                        row.kind,
                        row.library.as_str(),
                        row.source.as_str(),
                        row.section.as_str(),
                        row.status.as_str(),
                    ]
                    .iter()
                    .any(|value| value.to_ascii_lowercase().contains(&query))
            })
            .collect::<Vec<_>>();
        ui.add_space(5.0);
        if matching.is_empty() {
            ui.label(
                egui::RichText::new("No definitions match the current filter.")
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            return;
        }
        for row in matching.iter().take(100) {
            property_row_toned(
                ui,
                &format!("{} · {}", row.name, row.kind),
                &format!(
                    "{} · {} · {} · {}",
                    row.library, row.source, row.section, row.status
                ),
                if row.resolved {
                    t.color.ok
                } else {
                    t.color.err
                },
            );
        }
        if matching.len() > 100 {
            property_row(
                ui,
                "Additional matching definitions",
                &(matching.len() - 100).to_string(),
            );
        }
    });
}

fn draw_include_diagnostics(ui: &mut Ui, app: &mut RSpiceApp, diagnostics: &IncludeDiagnostics) {
    let t = Tokens::get(ui.ctx());
    draw_include_selection(ui, app);
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
    ui.add_space(8.0);
    draw_include_definition_resolution(ui, app);
    if !diagnostics.conflicts.is_empty() || diagnostics.resolution_error.is_some() {
        ui.add_space(8.0);
        property_card(ui, "Execution resolution", |ui| {
            for conflict in diagnostics.conflicts.iter().take(50) {
                let selected = app
                    .state
                    .model_library_manager
                    .definition_resolution(&conflict.normalized_name)
                    .cloned();
                let resolved = diagnostics.resolution_error.is_none() && selected.is_some();
                property_row_toned(
                    ui,
                    &conflict.normalized_name,
                    if resolved {
                        "resolved by explicit provider precedence"
                    } else if selected.is_some() {
                        "provider selected / complete precedence plan invalid"
                    } else {
                        "unresolved / simulation blocked"
                    },
                    if resolved { t.color.ok } else { t.color.err },
                );
                for provider in &conflict.providers {
                    ui.horizontal_wrapped(|ui| {
                        ui.add_space(10.0);
                        let source = provider.source.as_deref().map_or_else(
                            || provider.library.clone(),
                            |path| {
                                let file = path_display_name(path);
                                provider.source_line.map_or_else(
                                    || format!("{} / {file}", provider.library),
                                    |line| format!("{} / {file}:{line}", provider.library),
                                )
                            },
                        );
                        let is_selected = selected.as_ref().is_some_and(|resolution| {
                            resolution.provider_library == provider.library
                                && resolution.provider_model == provider.model
                        });
                        ui.label(
                            egui::RichText::new(if is_selected {
                                format!("{source} / selected")
                            } else {
                                source
                            })
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(if is_selected {
                                t.color.ok
                            } else {
                                t.color.text_dim
                            }),
                        );
                        let use_label = format!("Use {}", provider.library);
                        if !is_selected
                            && Button::new(&use_label)
                                .enabled(
                                    app.state.project_lifecycle.project_open
                                        && !app.state.workbench.safe_mode.project_read_only(),
                                )
                                .show(ui)
                                .clicked()
                        {
                            let mut candidate = app.state.model_library_manager.clone();
                            let result = candidate
                                .resolve_definition_conflict(
                                    &conflict.normalized_name,
                                    &provider.library,
                                    &provider.model,
                                )
                                .and_then(|()| app.publish_model_library_candidate(candidate));
                            match result {
                                Ok(()) => {
                                    app.state.push_user_message(ConsoleMessage::info(format!(
                                        "Selected exact provider '{}/{}' for contested model '{}'.",
                                        provider.library, provider.model, conflict.normalized_name
                                    )))
                                }
                                Err(error) => {
                                    app.state.push_user_message(ConsoleMessage::error(format!(
                                        "Could not resolve contested model '{}': {error}",
                                        conflict.normalized_name
                                    )))
                                }
                            }
                        }
                    });
                }
                if selected.is_some()
                    && Button::new("Clear provider selection")
                        .ghost()
                        .enabled(
                            app.state.project_lifecycle.project_open
                                && !app.state.workbench.safe_mode.project_read_only(),
                        )
                        .show(ui)
                        .clicked()
                {
                    let mut candidate = app.state.model_library_manager.clone();
                    candidate.clear_definition_resolution(&conflict.normalized_name);
                    if let Err(error) = app.publish_model_library_candidate(candidate) {
                        app.state.push_user_message(ConsoleMessage::error(format!(
                            "Could not clear provider selection for '{}': {error}",
                            conflict.normalized_name
                        )));
                    }
                }
                ui.add_space(5.0);
            }
            if diagnostics.conflicts.len() > 50 {
                property_row(
                    ui,
                    "Additional conflicts",
                    &(diagnostics.conflicts.len() - 50).to_string(),
                );
            }
            if let Some(error) = diagnostics.resolution_error.as_deref() {
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.err),
                );
            } else {
                property_row_toned(
                    ui,
                    "Execution order",
                    "complete, explicit, and acyclic",
                    t.color.ok,
                );
            }
        });
    }
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
mod tests;
