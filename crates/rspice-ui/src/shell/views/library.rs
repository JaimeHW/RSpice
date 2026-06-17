//! Library manager view — three-column Library / Cell / View browser with a
//! metadata strip, in the Cadence library-manager tradition.

use egui::Ui;

use crate::common::AppState;
use crate::common::app::LibraryDeleteTarget;
use crate::shell::WorkspaceView;
use crate::state::{CellViewRef, NavColumn, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, TreeRow, docbar, mono_input};

/// Destructive confirmation for cell/view deletion: the deletion only
/// lands once the user confirms the err-styled primary.
fn delete_confirm_dialog(ctx: &egui::Context, state: &mut AppState) {
    let Some(target) = state.dialogs.library_delete_confirm.clone() else {
        return;
    };
    let (title, body) = match &target {
        LibraryDeleteTarget::Cell { library, cell } => (
            format!("Delete cell '{cell}'"),
            format!(
                "Removes '{cell}' and all of its views from library '{library}'. \
                 This cannot be undone."
            ),
        ),
        LibraryDeleteTarget::View { cell, view, .. } => (
            format!("Delete view '{view}'"),
            format!("Removes view '{view}' from cell '{cell}'. This cannot be undone."),
        ),
    };
    let choice = Dialog::new("Library", &title, "Delete")
        .size(DialogSize::Sm)
        .destructive()
        .ghost("Cancel")
        .show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                egui::RichText::new(body)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
    match choice {
        DialogChoice::Primary => {
            match target {
                LibraryDeleteTarget::Cell { library, cell } => {
                    state.pending_delete_cell = Some((library, cell));
                }
                LibraryDeleteTarget::View {
                    library,
                    cell,
                    view,
                } => {
                    state.pending_delete_view = Some((library, cell, view));
                }
            }
            state.library_manager.selected_view = None;
            state.dialogs.library_delete_confirm = None;
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.dialogs.library_delete_confirm = None;
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// The cell names the Cell column lists for the current selection+filter —
/// rendering and keyboard navigation must agree on this.
fn visible_cells(state: &AppState) -> Vec<String> {
    let Some(library) = state.library_manager.selected_library.as_deref() else {
        return Vec::new();
    };
    let filter = state.library_manager.filter_text.to_lowercase();
    state
        .library_manager
        .get_library(library)
        .map(|lib| {
            lib.cells_sorted()
                .iter()
                .filter(|cell| filter.is_empty() || cell.name.to_lowercase().contains(&filter))
                .map(|cell| cell.name.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// The view names the View column lists.
fn visible_views(state: &AppState) -> Vec<String> {
    let (Some(library), Some(cell)) = (
        state.library_manager.selected_library.as_deref(),
        state.library_manager.selected_cell.as_deref(),
    ) else {
        return Vec::new();
    };
    state
        .library_manager
        .get_library(library)
        .and_then(|lib| lib.get_cell(cell))
        .map(|cell| {
            cell.views_sorted()
                .iter()
                .map(|view| view.name.clone())
                .collect()
        })
        .unwrap_or_default()
}

/// Whether a Library/Cell/View document opens in the Schematic-family editor.
fn is_workspace_openable(view_type: ViewType) -> bool {
    matches!(
        view_type,
        ViewType::Schematic | ViewType::Testbench | ViewType::Symbol
    )
}

/// Open a view in the workspace if it carries design-editor content.
fn open_view_if_editable(state: &mut AppState, library: &str, cell: &str, view: &str) {
    let openable = state
        .library_manager
        .get_library(library)
        .and_then(|lib| lib.get_cell(cell))
        .and_then(|c| c.views_sorted().iter().find(|v| v.name == view).copied())
        .is_some_and(|v| is_workspace_openable(v.view_type));
    if openable {
        state.open_workspace_view(CellViewRef {
            library: library.to_owned(),
            cell: cell.to_owned(),
            view: view.to_owned(),
        });
        state.shell.view = WorkspaceView::Schematic;
    }
}

/// Keyboard navigation: ↑↓ move within the focused column, ←→ hop columns,
/// Enter opens. Inert while any text field (the filter) has focus.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_views_are_schematic_family_openable() {
        assert!(is_workspace_openable(ViewType::Schematic));
        assert!(is_workspace_openable(ViewType::Testbench));
        assert!(is_workspace_openable(ViewType::Symbol));
        assert!(!is_workspace_openable(ViewType::Layout));
        assert!(!is_workspace_openable(ViewType::VerilogA));
        assert!(!is_workspace_openable(ViewType::Spice));
    }
}

fn handle_keyboard_nav(ui: &Ui, state: &mut AppState) {
    if ui.ctx().wants_keyboard_input() {
        return;
    }
    let (down, up, left, right, enter) = ui.input(|i| {
        (
            i.key_pressed(egui::Key::ArrowDown),
            i.key_pressed(egui::Key::ArrowUp),
            i.key_pressed(egui::Key::ArrowLeft),
            i.key_pressed(egui::Key::ArrowRight),
            i.key_pressed(egui::Key::Enter),
        )
    });
    if !(down || up || left || right || enter) {
        return;
    }

    let nav = state.library_manager.nav_column;
    if left {
        state.library_manager.nav_column = match nav {
            NavColumn::View => NavColumn::Cell,
            _ => NavColumn::Library,
        };
    }
    if right {
        state.library_manager.nav_column = match nav {
            NavColumn::Library if state.library_manager.selected_library.is_some() => {
                NavColumn::Cell
            }
            NavColumn::Cell if state.library_manager.selected_cell.is_some() => NavColumn::View,
            other => other,
        };
    }

    if down || up {
        let step: isize = if down { 1 } else { -1 };
        match state.library_manager.nav_column {
            NavColumn::Library => {
                let names: Vec<String> = state
                    .library_manager
                    .libraries_sorted()
                    .iter()
                    .map(|lib| lib.name.clone())
                    .collect();
                if let Some(next) = step_in(
                    &names,
                    state.library_manager.selected_library.as_deref(),
                    step,
                ) {
                    state.library_manager.select_library(&next);
                }
            }
            NavColumn::Cell => {
                let names = visible_cells(state);
                let library = state.library_manager.selected_library.clone();
                if let (Some(library), Some(next)) = (
                    library,
                    step_in(&names, state.library_manager.selected_cell.as_deref(), step),
                ) {
                    state.library_manager.select_cell(&library, &next);
                }
            }
            NavColumn::View => {
                let names = visible_views(state);
                let context = (
                    state.library_manager.selected_library.clone(),
                    state.library_manager.selected_cell.clone(),
                );
                if let ((Some(library), Some(cell)), Some(next)) = (
                    context,
                    step_in(&names, state.library_manager.selected_view.as_deref(), step),
                ) {
                    state.library_manager.select_view(&library, &cell, &next);
                }
            }
        }
        state.library_manager.nav_scroll = true;
    }

    if enter {
        let selection = (
            state.library_manager.selected_library.clone(),
            state.library_manager.selected_cell.clone(),
            state.library_manager.selected_view.clone(),
        );
        match (state.library_manager.nav_column, selection) {
            (NavColumn::View, (Some(library), Some(cell), Some(view))) => {
                open_view_if_editable(state, &library, &cell, &view);
            }
            // Enter on a cell opens its schematic view when it has one.
            (NavColumn::Cell, (Some(library), Some(cell), _)) => {
                open_view_if_editable(state, &library, &cell, "schematic");
            }
            _ => {}
        }
    }
}

/// Next selection for a ±1 keyboard step: clamped, and a step with no
/// current selection lands on the first (or last) entry.
fn step_in(names: &[String], current: Option<&str>, step: isize) -> Option<String> {
    if names.is_empty() {
        return None;
    }
    let index = match current.and_then(|c| names.iter().position(|n| n == c)) {
        Some(index) => (index as isize + step).clamp(0, names.len() as isize - 1) as usize,
        None if step > 0 => 0,
        None => names.len() - 1,
    };
    names.get(index).cloned()
}

/// Render the library manager.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    delete_confirm_dialog(ui.ctx(), state);
    handle_keyboard_nav(ui, state);
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
            // Delete targets the deepest selection: view, else cell.
            let delete_target = match (
                state.library_manager.selected_library.clone(),
                state.library_manager.selected_cell.clone(),
                state.library_manager.selected_view.clone(),
            ) {
                (Some(library), Some(cell), Some(view)) => Some(LibraryDeleteTarget::View {
                    library,
                    cell,
                    view,
                }),
                (Some(library), Some(cell), None) => {
                    Some(LibraryDeleteTarget::Cell { library, cell })
                }
                _ => None,
            };
            if Button::new("Delete")
                .ghost()
                .enabled(delete_target.is_some())
                .show(ui)
                .clicked()
            {
                state.dialogs.library_delete_confirm = delete_target;
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
            let lib_focus = state.library_manager.nav_column == NavColumn::Library;
            column(
                ui,
                column_width,
                columns_height,
                "Library",
                true,
                lib_focus,
                |ui| {
                    let libraries: Vec<(String, usize, bool)> = state
                        .library_manager
                        .libraries_sorted()
                        .iter()
                        .map(|lib| (lib.name.clone(), lib.cells.len(), lib.read_only))
                        .collect();
                    for (name, cell_count, read_only) in libraries {
                        let selected = state.library_manager.selected_library.as_deref()
                            == Some(name.as_str());
                        // Read-only libraries wear the same "ro ·" mark as the
                        // rail's palette groups.
                        let meta = if read_only {
                            format!("ro · {cell_count}")
                        } else {
                            cell_count.to_string()
                        };
                        let row = TreeRow::new(&name).meta(&meta).selected(selected).show(ui);
                        if selected && state.library_manager.nav_scroll {
                            row.response.scroll_to_me(Some(egui::Align::Center));
                        }
                        if row.response.clicked() {
                            state.library_manager.select_library(&name);
                            state.library_manager.nav_column = NavColumn::Library;
                        }
                        if read_only {
                            row.response
                                .on_hover_text("Read-only library — placeable, never editable");
                        }
                    }
                },
            );

            // ---- Cell column
            let cell_focus = state.library_manager.nav_column == NavColumn::Cell;
            column(
                ui,
                column_width,
                columns_height,
                "Cell",
                true,
                cell_focus,
                |ui| {
                    let Some(library) = state.library_manager.selected_library.clone() else {
                        column_empty(ui, "Select a library");
                        return;
                    };
                    let (cells, unfiltered): (Vec<String>, usize) = state
                        .library_manager
                        .get_library(&library)
                        .map(|lib| {
                            let all = lib.cells_sorted();
                            let unfiltered = all.len();
                            let names = all
                                .iter()
                                .filter(|cell| {
                                    filter.is_empty() || cell.name.to_lowercase().contains(&filter)
                                })
                                .map(|cell| cell.name.clone())
                                .collect();
                            (names, unfiltered)
                        })
                        .unwrap_or_default();
                    if cells.is_empty() {
                        // Name the filter so a stale one can't gaslight the browser.
                        if unfiltered == 0 {
                            column_empty(ui, "No cells yet — New cell creates the first");
                        } else {
                            column_empty(
                                ui,
                                &format!(
                                    "No cells match '{}'",
                                    state.library_manager.filter_text.trim()
                                ),
                            );
                        }
                        return;
                    }
                    let library_read_only = state
                        .library_manager
                        .get_library(&library)
                        .is_some_and(|lib| lib.read_only);
                    for name in cells {
                        let selected =
                            state.library_manager.selected_cell.as_deref() == Some(name.as_str());
                        let row = TreeRow::new(&name).mono().selected(selected).show(ui);
                        if selected && state.library_manager.nav_scroll {
                            row.response.scroll_to_me(Some(egui::Align::Center));
                        }
                        if row.response.clicked() {
                            state.library_manager.select_cell(&library, &name);
                            state.library_manager.nav_column = NavColumn::Cell;
                        }
                        // Right-click selects and offers the cell operations —
                        // Copy stays live on read-only libraries (copying out
                        // is the point); Rename and Delete disable.
                        if row.response.secondary_clicked() {
                            state.library_manager.select_cell(&library, &name);
                            state.library_manager.nav_column = NavColumn::Cell;
                        }
                        cell_row_menu(&row.response, state, &library, &name, library_read_only);
                    }
                },
            );

            // ---- View column
            let view_focus = state.library_manager.nav_column == NavColumn::View;
            column(
                ui,
                column_width,
                columns_height,
                "View",
                false,
                view_focus,
                |ui| {
                    if state.library_manager.selected_library.is_none() {
                        column_empty(ui, "Select a library");
                        return;
                    }
                    let (Some(library), Some(cell)) = (
                        state.library_manager.selected_library.clone(),
                        state.library_manager.selected_cell.clone(),
                    ) else {
                        column_empty(ui, "Select a cell");
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
                        // Schematic-content views open in the editor; the rest
                        // say so instead of staying silent.
                        let openable = is_workspace_openable(view_type);
                        let meta = if openable {
                            "double-click to open"
                        } else {
                            "no editor yet"
                        };
                        let row = TreeRow::new(&name)
                            .meta(meta)
                            .mono()
                            .selected(selected)
                            .show(ui);
                        if selected && state.library_manager.nav_scroll {
                            row.response.scroll_to_me(Some(egui::Align::Center));
                        }
                        if row.response.clicked() {
                            state.library_manager.select_view(&library, &cell, &name);
                            state.library_manager.nav_column = NavColumn::View;
                        }
                        if row.response.double_clicked() && openable {
                            state.open_workspace_view(CellViewRef {
                                library: library.clone(),
                                cell: cell.clone(),
                                view: name.clone(),
                            });
                            state.shell.view = WorkspaceView::Schematic;
                        }
                    }
                },
            );
        },
    );

    // Keyboard scroll requests are one-shot — consumed by this frame's rows.
    state.library_manager.nav_scroll = false;

    // ---- Meta strip
    let (meta_rect, _) =
        ui.allocate_exact_size(egui::vec2(full_width, meta_height), egui::Sense::hover());
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
    // Technology and path come off the selected library; "—" only when the
    // library genuinely has none.
    let selected_lib = state
        .library_manager
        .selected_library
        .as_deref()
        .and_then(|name| state.library_manager.get_library(name));
    let technology = selected_lib
        .map(|lib| lib.technology.trim())
        .filter(|tech| !tech.is_empty())
        .unwrap_or("—")
        .to_owned();
    let lib_path = selected_lib
        .and_then(|lib| lib.path.as_ref())
        .map(|path| path.display().to_string());
    let mut x = meta_rect.left() + 14.0;
    for (key, value) in [
        ("Cell", cell_path.as_str()),
        ("Checked out", "you"),
        ("Technology", technology.as_str()),
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

    // Library path, right-aligned and faint — drawn only when it fits.
    if let Some(path) = lib_path {
        let path_galley = ui.fonts(|f| {
            f.layout_no_wrap(
                path,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_faint,
            )
        });
        let px = meta_rect.right() - 14.0 - path_galley.size().x;
        if px >= x {
            painter.galley(
                egui::pos2(px, meta_rect.center().y - path_galley.size().y * 0.5),
                path_galley,
                c.text_faint,
            );
        }
    }
}

/// Right-click menu for a cell row: Open, the cell operations, Delete.
/// `design/app/volta-library-manager.html` §07.
fn cell_row_menu(
    response: &egui::Response,
    state: &mut AppState,
    library: &str,
    cell: &str,
    read_only: bool,
) {
    use crate::shell::menubar::{item, item_disabled, separator};
    response.clone().context_menu(|ui| {
        ui.set_min_width(200.0);
        if item(ui, "Open", None) {
            open_view_if_editable(state, library, cell, "schematic");
            ui.close_menu();
        }
        separator(ui);
        if item(ui, "Copy cell…", None) {
            state.open_copy_cell_dialog(library, cell);
            ui.close_menu();
        }
        if read_only {
            item_disabled(ui, "Rename cell…", None);
        } else if item(ui, "Rename cell…", None) {
            state.open_rename_cell_dialog(library, cell);
            ui.close_menu();
        }
        separator(ui);
        if read_only {
            item_disabled(ui, "Delete…", None);
        } else if item(ui, "Delete…", None) {
            state.dialogs.library_delete_confirm = Some(LibraryDeleteTarget::Cell {
                library: library.to_owned(),
                cell: cell.to_owned(),
            });
            ui.close_menu();
        }
    });
}

/// Quiet one-liner for a column with nothing to list — names the action
/// that fills it.
fn column_empty(ui: &mut Ui, text: &str) {
    let c = Tokens::get(ui.ctx()).color;
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(c.text_faint),
    );
}

/// One bordered browser column with an uppercase header. The header takes
/// the accent ink while the keyboard owns the column.
fn column(
    ui: &mut Ui,
    width: f32,
    height: f32,
    title: &str,
    right_border: bool,
    focused: bool,
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
            let (head_rect, _) =
                ui.allocate_exact_size(egui::vec2(width, 30.0), egui::Sense::hover());
            let painter = ui.painter();
            painter.rect_filled(head_rect, 0.0, c.bg_panel);
            painter.hline(
                head_rect.x_range(),
                head_rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
            let header_color = if focused { c.accent } else { c.text_faint };
            let mut job = egui::text::LayoutJob::default();
            job.append(
                &title.to_uppercase(),
                0.0,
                egui::TextFormat {
                    font_id: theme::sans(tokens::FS_0, FontWeight::SemiBold),
                    color: header_color,
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
                header_color,
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
