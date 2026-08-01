//! Model catalog, symbol contracts, PDK sections, authenticated includes, and
//! the source-owned model qualification and release gate.

mod manager;
mod qualification;

use qualification::*;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

use egui::{Align, Align2, Color32, Key, Layout, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};
use sha2::{Digest as _, Sha256};

use crate::diagnostics::ConsoleMessage;
use crate::state::ModelBoundSymbolDefinition;
use crate::state::model_library::{
    DeviceModel, ModelCorrelationState, ModelLibrary, ModelQualificationState,
    ModelSourceEvidenceBinding, QualificationAnalysis, QualificationPlatform,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;
use crate::workbench::app::{
    open_create_model_bound_symbol_dialog, open_symbol_import_dialog,
    open_symbol_parameter_form_dialog,
};

use super::super::design_system::{property_card, property_row, workspace_title_row};
use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use crate::workbench::commands::{CommandAvailability, vocabulary::Command};
use crate::workbench::documents::model_editor::{self, ModelEditorSection};

const TABLE_HEAD_H: f32 = 27.0;
const TABLE_CARD_HEAD_H: f32 = 37.0;
const MODEL_PHONE_BREAKPOINT: f32 = 560.0;
const MODEL_SUMMARY_BREAKPOINT: f32 = 820.0;
const MODEL_TITLE_MIN_CONTENT_H: f32 = 48.0;
const QUALIFICATION_MIN_CONTENT_H: f32 = 680.0;
const QUALIFICATION_STACKED_MIN_CONTENT_H: f32 = 1000.0;
const QUALIFICATION_GATE_COPY: &str = "Dispositions, reruns, replacement, and retirement remain source-owned here. Release promotion cannot override missing or failing vector outcomes.";

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    manager::show(ui, app);
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

fn model_title_actions_stack(surface_width: f32) -> bool {
    surface_width <= MODEL_PHONE_BREAKPOINT
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
) {
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
    if let Some((id, _)) = requested_focus {
        ui.memory_mut(|memory| memory.request_focus(id));
    }
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

#[derive(Debug, Clone)]
struct IncludeDiagnostics {
    files: usize,
    edges: usize,
    cyclic_nodes: usize,
    unpinned_roots: usize,
}

fn include_diagnostics(app: &RSpiceApp) -> IncludeDiagnostics {
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
    IncludeDiagnostics {
        files: sources.len(),
        edges: edges.len(),
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

#[cfg(test)]
mod tests;
