//! Schematic-context side panels: hierarchy + component browser (left),
//! instance inspector (right).

use egui::Ui;

use crate::common::AppState;
use crate::state::{ComponentType, Tool};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, IconButton, TreeRow, input_row, input_row_readonly, kv_row, mono_input,
    section_header,
};

// ---------------------------------------------------------------------------
// Left panel
// ---------------------------------------------------------------------------

/// Render the schematic context's left panel.
pub fn left(ui: &mut Ui, state: &mut AppState) {
    hierarchy_section(ui, state);
    components_section(ui, state);
    quick_place_section(ui, state);
}

fn hierarchy_section(ui: &mut Ui, state: &mut AppState) {
    if let Some(action) = section_header(ui, "Hierarchy", Some("Descend")) {
        if action.clicked() {
            state.open_selected_instance_master();
        }
    }

    ui.add_space(2.0);
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        let cell_label = state.workspace.active_view.cell.clone();
        TreeRow::new(&cell_label)
            .twist(true)
            .meta("sheet 1/1")
            .show(ui);

        // Instances of the active schematic, selection-synced.
        let rows: Vec<(u64, String, String)> = state
            .schematic
            .components
            .iter()
            .map(|component| {
                let meta = if component.value.is_empty() {
                    component.kind.display_name().to_owned()
                } else {
                    component.value.clone()
                };
                (component.id, component.name.clone(), meta)
            })
            .collect();
        for (id, name, meta) in rows {
            let selected = state.schematic.selection.has_component(id);
            let row = TreeRow::new(&name)
                .meta(&meta)
                .indent(1)
                .mono()
                .selected(selected)
                .show(ui);
            if row.response.clicked() {
                state.schematic.selection.select_only_component(id);
            }
        }
    });
}

/// Library filter for the component browser.
fn cell_sources(state: &AppState) -> Vec<String> {
    let mut libs: Vec<String> = vec!["All libs".to_owned(), "primitives".to_owned()];
    libs.extend(
        state
            .library_manager
            .libraries_sorted()
            .iter()
            .map(|lib| lib.name.clone()),
    );
    libs.dedup();
    libs
}

/// One placeable cell in the browser list.
enum CellEntry {
    /// A built-in primitive (palette entry).
    Primitive(ComponentType, &'static str),
    /// A library cell (library, cell name).
    LibraryCell(String, String),
}

fn matching_cells(state: &AppState) -> Vec<CellEntry> {
    let query = state.shell.cell_search.to_lowercase();
    let lib_filter = &state.shell.cell_lib_filter;
    let mut entries = Vec::new();

    if lib_filter == "All libs" || lib_filter == "primitives" {
        for section in crate::schematic::component_palette() {
            for entry in section.entries {
                if query.is_empty() || entry.label.to_lowercase().contains(&query) {
                    entries.push(CellEntry::Primitive(entry.kind, entry.label));
                }
            }
        }
    }
    for library in state.library_manager.libraries_sorted() {
        if lib_filter != "All libs" && *lib_filter != library.name {
            continue;
        }
        for cell in library.cells_sorted() {
            if query.is_empty()
                || cell.name.to_lowercase().contains(&query)
                || library.name.to_lowercase().contains(&query)
            {
                entries.push(CellEntry::LibraryCell(
                    library.name.clone(),
                    cell.name.clone(),
                ));
            }
        }
    }
    entries
}

fn components_section(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    if let Some(action) = section_header(ui, "Components", Some("Browse library…")) {
        if action.clicked() {
            state.shell.view = crate::shell::WorkspaceView::Library;
        }
    }

    // Search + library filter.
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), t.metrics.ctl_h + 4.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(12.0);
            ui.spacing_mut().item_spacing.x = 6.0;
            let search_width = ui.available_width() - 96.0 - 12.0 - 6.0;
            let search = mono_input(ui, &mut state.shell.cell_search, search_width.max(60.0));
            if state.shell.focus_cell_search {
                search.request_focus();
                state.shell.focus_cell_search = false;
            }
            let libs = cell_sources(state);
            egui::ComboBox::from_id_salt("volta.cell.lib")
                .selected_text(
                    egui::RichText::new(&state.shell.cell_lib_filter)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular)),
                )
                .width(90.0)
                .show_ui(ui, |ui| {
                    for lib in libs {
                        if ui
                            .selectable_label(state.shell.cell_lib_filter == lib, &lib)
                            .clicked()
                        {
                            state.shell.cell_lib_filter = lib;
                        }
                    }
                });
        },
    );
    ui.add_space(4.0);

    // Cell list in an inset well.
    let entries = matching_cells(state);
    let inset = egui::Frame::none()
        .fill(c.bg_inset)
        .stroke(egui::Stroke::new(1.0, c.border))
        .rounding(t.radius)
        .inner_margin(egui::Margin::same(3.0))
        .outer_margin(egui::Margin::symmetric(12.0, 0.0));
    inset.show(ui, |ui| {
        ui.set_min_height(120.0);
        egui::ScrollArea::vertical()
            .id_salt("volta.cell.list")
            .max_height(172.0)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                if entries.is_empty() {
                    ui.add_space(12.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("No cells match — clear the filter")
                                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                                .color(c.text_faint),
                        );
                    });
                    ui.add_space(12.0);
                    return;
                }
                for (idx, entry) in entries.iter().enumerate() {
                    let (label, meta) = match entry {
                        CellEntry::Primitive(_, label) => (*label, "primitives"),
                        CellEntry::LibraryCell(lib, cell) => (cell.as_str(), lib.as_str()),
                    };
                    let selected = state.shell.cell_selected == Some(idx);
                    let row = TreeRow::new(label)
                        .meta(meta)
                        .mono()
                        .selected(selected)
                        .height(24.0)
                        .show(ui);
                    if row.response.clicked() {
                        state.shell.cell_selected = Some(idx);
                    }
                    if row.response.double_clicked() {
                        place_entry(state, entry);
                    }
                }
            });
    });

    // Preview + place.
    if let Some(selected) = state.shell.cell_selected {
        if let Some(entry) = entries.get(selected) {
            ui.add_space(8.0);
            let preview = egui::Frame::none()
                .fill(c.bg_inset)
                .stroke(egui::Stroke::new(1.0, c.border))
                .rounding(t.radius)
                .inner_margin(egui::Margin::symmetric(10.0, 8.0))
                .outer_margin(egui::Margin::symmetric(12.0, 0.0));
            preview.show(ui, |ui| {
                let (name, meta, kind) = match entry {
                    CellEntry::Primitive(kind, label) => (
                        format!("primitives / {label}"),
                        format!("symbol · {}", kind.display_name()),
                        Some(*kind),
                    ),
                    CellEntry::LibraryCell(lib, cell) => (
                        format!("{lib} / {cell}"),
                        "cell · schematic".to_owned(),
                        None,
                    ),
                };

                // Symbol preview canvas.
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 84.0),
                    egui::Sense::hover(),
                );
                if let Some(kind) = kind {
                    crate::schematic::view::draw_symbol_preview(
                        ui.painter(),
                        rect,
                        kind,
                        c.symbol,
                    );
                } else {
                    // Hierarchical cell: generic block symbol.
                    let painter = ui.painter();
                    let block = egui::Rect::from_center_size(
                        rect.center(),
                        egui::vec2(44.0, 52.0),
                    );
                    painter.rect_stroke(block, 0.0, egui::Stroke::new(1.6, c.symbol));
                    for (dx, dy) in [(-1.0, -12.0), (-1.0, 12.0), (1.0, -12.0), (1.0, 12.0)] {
                        let x0 = if dx < 0.0 { block.left() - 22.0 } else { block.right() };
                        painter.hline(
                            egui::Rangef::new(x0, x0 + 22.0),
                            block.center().y + dy,
                            egui::Stroke::new(1.6, c.symbol),
                        );
                    }
                }
                ui.label(
                    egui::RichText::new(name)
                        .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                        .color(c.text),
                );
                ui.label(
                    egui::RichText::new(meta)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
                ui.add_space(8.0);
                if Button::new("Place")
                    .min_width(ui.available_width())
                    .show(ui)
                    .clicked()
                {
                    place_entry(state, entry);
                }
            });
        } else {
            state.shell.cell_selected = None;
        }
    }
}

/// Begin placement for a browser entry.
fn place_entry(state: &mut AppState, entry: &CellEntry) {
    match entry {
        CellEntry::Primitive(kind, _) => {
            state.schematic.tool = Tool::Place(*kind);
        }
        CellEntry::LibraryCell(lib, cell) => {
            state.schematic.pending_library_cell = Some(crate::state::LibraryCellInstance {
                library: lib.clone(),
                cell: cell.clone(),
                view: "schematic".to_owned(),
                source_path: None,
                module_name: None,
                terminal_order: Vec::new(),
            });
            state.schematic.tool = Tool::Place(ComponentType::CellInstance);
        }
    }
    state.shell.view = crate::shell::WorkspaceView::Schematic;
}

fn quick_place_section(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Quick place", None);
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 32.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(12.0);
            ui.spacing_mut().item_spacing.x = 4.0;
            if IconButton::new(Icon::Wire)
                .tooltip("Wire (W)")
                .on(state.schematic.tool.is_wire())
                .show(ui)
                .clicked()
            {
                state.schematic.tool = Tool::Wire;
            }
            if IconButton::new(Icon::Ground)
                .tooltip("Ground")
                .show(ui)
                .clicked()
            {
                state.schematic.tool = Tool::Place(ComponentType::Ground);
            }
            if IconButton::new(Icon::Supply)
                .tooltip("DC supply")
                .show(ui)
                .clicked()
            {
                state.schematic.tool = Tool::Place(ComponentType::VoltageSource);
            }
            if IconButton::new(Icon::NetLabel)
                .tooltip("Net label (N)")
                .on(matches!(state.schematic.tool, Tool::Label))
                .show(ui)
                .clicked()
            {
                state.schematic.tool = Tool::Label;
            }
            if IconButton::new(Icon::Probe)
                .tooltip("Probe (P)")
                .on(matches!(state.schematic.tool, Tool::Probe))
                .show(ui)
                .clicked()
            {
                state.schematic.tool = Tool::Probe;
            }
        },
    );
    ui.add_space(12.0);
}

// ---------------------------------------------------------------------------
// Right panel — inspector
// ---------------------------------------------------------------------------

/// Render the schematic context's right panel (instance inspector).
pub fn right(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Inspector", None);

    let Some(component_id) = state.schematic.selection.single_component() else {
        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new(if state.schematic.selection.is_empty() {
                    "Select an instance to inspect"
                } else {
                    "Multiple instances selected"
                })
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_faint),
            );
        });
        return;
    };

    let Some(component_idx) = state
        .schematic
        .components
        .iter()
        .position(|component| component.id == component_id)
    else {
        return;
    };

    // Editable snapshot — written back through an undo-tracked operation when
    // any field changes.
    let snapshot = state.schematic.components[component_idx].clone();
    let mut name = snapshot.name.clone();
    let mut value = snapshot.value.clone();
    let mut params = snapshot.params.clone();

    ui.add_space(2.0);
    egui::Frame::none()
        .inner_margin(egui::Margin {
            left: 12.0,
            right: 12.0,
            top: 0.0,
            bottom: 12.0,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;

            input_row(ui, "Instance", &mut name);
            input_row_readonly(ui, "Type", snapshot.kind.display_name());
            let value_label =
                crate::properties::property_bridge::get_value_display_name(snapshot.kind);
            input_row(ui, value_label, &mut value);
            input_row(ui, "Params", &mut params);

            // Terminal connectivity.
            let terminals = snapshot.terminal_positions();
            if !terminals.is_empty() {
                section_header(ui, "Nets", None);
                for (terminal, position) in &terminals {
                    let net = state
                        .schematic
                        .net_mapping
                        .get(position)
                        .cloned()
                        .unwrap_or_else(|| "—".to_owned());
                    kv_row(ui, terminal, &net);
                }
            }
        });

    let changed =
        name != snapshot.name || value != snapshot.value || params != snapshot.params;
    if changed {
        state.schematic.with_undo("edit properties", |schematic| {
            let component = &mut schematic.components[component_idx];
            component.name = name;
            component.value = value;
            component.params = params;
        });
        state.schematic.is_dirty = true;
    }
}
