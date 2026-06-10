//! Library manager view — three-column Library / Cell / View browser with a
//! metadata strip, in the Cadence library-manager tradition.

use egui::Ui;

use crate::common::AppState;
use crate::shell::WorkspaceView;
use crate::state::{CellViewRef, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, TreeRow, docbar, mono_input};

/// Render the library manager.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    docbar(ui, |ui| {
        let filter_width = 240.0;
        ui.scope(|ui| {
            mono_input(ui, &mut state.library_manager.filter_text, filter_width);
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            if Button::new("New cell").show(ui).clicked() {
                state.dialogs.new_cell_dialog = true;
                if let Some(library) = &state.library_manager.selected_library {
                    state.dialogs.new_cell_library = library.clone();
                }
            }
            if Button::new("New view").show(ui).clicked() {
                state.dialogs.new_view_dialog = true;
                // Prefill the target from the browser selection — the
                // dialog shows these as fixed context.
                if let Some(library) = &state.library_manager.selected_library {
                    state.dialogs.new_view_library = library.clone();
                }
                if let Some(cell) = &state.library_manager.selected_cell {
                    state.dialogs.new_view_cell = cell.clone();
                }
            }
        });
    });

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let filter = state.library_manager.filter_text.to_lowercase();

    // Reserve the meta strip at the bottom.
    let meta_height = 36.0;
    let columns_height = (ui.available_height() - meta_height).max(0.0);
    let full_width = ui.available_width();
    let column_width = (full_width / 3.0).floor();

    ui.allocate_ui_with_layout(
        egui::vec2(full_width, columns_height),
        egui::Layout::left_to_right(egui::Align::TOP),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            // ---- Library column
            column(ui, column_width, columns_height, "Library", true, |ui| {
                let libraries: Vec<(String, usize)> = state
                    .library_manager
                    .libraries_sorted()
                    .iter()
                    .map(|lib| (lib.name.clone(), lib.cells.len()))
                    .collect();
                for (name, cell_count) in libraries {
                    let selected = state.library_manager.selected_library.as_deref()
                        == Some(name.as_str());
                    let row = TreeRow::new(&name)
                        .meta(&cell_count.to_string())
                        .selected(selected)
                        .show(ui);
                    if row.response.clicked() {
                        state.library_manager.select_library(&name);
                    }
                }
            });

            // ---- Cell column
            column(ui, column_width, columns_height, "Cell", true, |ui| {
                let Some(library) = state
                    .library_manager
                    .selected_library
                    .clone()
                else {
                    return;
                };
                let cells: Vec<String> = state
                    .library_manager
                    .get_library(&library)
                    .map(|lib| {
                        lib.cells_sorted()
                            .iter()
                            .filter(|cell| {
                                filter.is_empty()
                                    || cell.name.to_lowercase().contains(&filter)
                            })
                            .map(|cell| cell.name.clone())
                            .collect()
                    })
                    .unwrap_or_default();
                for name in cells {
                    let selected =
                        state.library_manager.selected_cell.as_deref() == Some(name.as_str());
                    let row = TreeRow::new(&name).mono().selected(selected).show(ui);
                    if row.response.clicked() {
                        state.library_manager.select_cell(&library, &name);
                    }
                }
            });

            // ---- View column
            column(ui, column_width, columns_height, "View", false, |ui| {
                let (Some(library), Some(cell)) = (
                    state.library_manager.selected_library.clone(),
                    state.library_manager.selected_cell.clone(),
                ) else {
                    return;
                };
                let views: Vec<(String, ViewType)> = state
                    .library_manager
                    .get_library(&library)
                    .and_then(|lib| lib.get_cell(&cell))
                    .map(|cell| {
                        cell.views_sorted()
                            .iter()
                            .map(|view| (view.name.clone(), view.view_type))
                            .collect()
                    })
                    .unwrap_or_default();
                for (name, view_type) in views {
                    let selected =
                        state.library_manager.selected_view.as_deref() == Some(name.as_str());
                    let meta = if view_type == ViewType::Schematic {
                        "double-click to open"
                    } else {
                        ""
                    };
                    let row = TreeRow::new(&name)
                        .meta(meta)
                        .mono()
                        .selected(selected)
                        .show(ui);
                    if row.response.clicked() {
                        state
                            .library_manager
                            .select_view(&library, &cell, &name);
                    }
                    if row.response.double_clicked() && view_type == ViewType::Schematic {
                        state.open_workspace_view(CellViewRef {
                            library: library.clone(),
                            cell: cell.clone(),
                            view: name.clone(),
                        });
                        state.shell.view = WorkspaceView::Schematic;
                    }
                }
            });
        },
    );

    // ---- Meta strip
    let (meta_rect, _) = ui.allocate_exact_size(
        egui::vec2(full_width, meta_height),
        egui::Sense::hover(),
    );
    let painter = ui.painter();
    painter.rect_filled(meta_rect, 0.0, c.bg_panel);
    painter.hline(
        meta_rect.x_range(),
        meta_rect.top() + 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    let cell_path = match (
        &state.library_manager.selected_library,
        &state.library_manager.selected_cell,
    ) {
        (Some(library), Some(cell)) => format!("{library} / {cell}"),
        (Some(library), None) => library.clone(),
        _ => "—".to_owned(),
    };
    let mut x = meta_rect.left() + 14.0;
    for (key, value) in [
        ("Cell", cell_path.as_str()),
        ("Checked out", "you"),
        ("Technology", "—"),
    ] {
        let key_galley = ui.fonts(|f| {
            f.layout_no_wrap(
                format!("{key} "),
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            )
        });
        let value_galley = ui.fonts(|f| {
            f.layout_no_wrap(
                value.to_owned(),
                theme::mono(tokens::FS_1, FontWeight::Medium),
                c.text,
            )
        });
        let cy = meta_rect.center().y;
        painter.galley(
            egui::pos2(x, cy - key_galley.size().y * 0.5),
            key_galley.clone(),
            c.text_dim,
        );
        x += key_galley.size().x;
        painter.galley(
            egui::pos2(x, cy - value_galley.size().y * 0.5),
            value_galley.clone(),
            c.text,
        );
        x += value_galley.size().x + 28.0;
    }
}

/// One bordered browser column with an uppercase header.
fn column(
    ui: &mut Ui,
    width: f32,
    height: f32,
    title: &str,
    right_border: bool,
    add_contents: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    ui.allocate_ui_with_layout(
        egui::vec2(width, height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_min_height(height);
            let rect = ui.max_rect();
            if right_border {
                ui.painter().vline(
                    rect.right() - 0.5,
                    rect.y_range(),
                    egui::Stroke::new(1.0, c.border),
                );
            }

            // Column header.
            let (head_rect, _) = ui.allocate_exact_size(
                egui::vec2(width, 30.0),
                egui::Sense::hover(),
            );
            let painter = ui.painter();
            painter.rect_filled(head_rect, 0.0, c.bg_panel);
            painter.hline(
                head_rect.x_range(),
                head_rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
            let mut job = egui::text::LayoutJob::default();
            job.append(
                &title.to_uppercase(),
                0.0,
                egui::TextFormat {
                    font_id: theme::sans(tokens::FS_0, FontWeight::SemiBold),
                    color: c.text_faint,
                    extra_letter_spacing: 0.09 * tokens::FS_0,
                    ..Default::default()
                },
            );
            let galley = ui.fonts(|f| f.layout_job(job));
            painter.galley(
                egui::pos2(
                    head_rect.left() + 12.0,
                    head_rect.center().y - galley.size().y * 0.5,
                ),
                galley,
                c.text_faint,
            );

            egui::ScrollArea::vertical()
                .id_salt(("volta.lib.column", title))
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add_space(4.0);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(6.0, 0.0))
                        .show(ui, add_contents);
                });
        },
    );
}
