//! Dense project library browser.
//!
//! The surface projects the persisted design-library manager directly: each
//! visible row names a real library, cell, or view, selection writes back to
//! that manager, and opening a view uses the normal workspace transaction.

use super::*;
use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::AppState;

const CONTEXT_HEIGHT: f32 = 36.0;
const DETAIL_HEIGHT: f32 = 40.0;
const COLUMN_HEADER_HEIGHT: f32 = 28.0;
const BROWSER_ROW_HEIGHT: f32 = 31.0;
const MIN_BROWSER_WIDTH: f32 = 620.0;
const USAGE_REPORT_ID: &str = "workbench.project.library.where-used";
const AUDIT_REPORT_ID: &str = "workbench.project.library.audit-history";
const LOCK_REPORT_ID: &str = "workbench.project.library.edit-locks";

#[derive(Debug, Clone, PartialEq, Eq)]
enum LibraryIntent {
    SelectLibrary(String),
    SelectCell { library: String, cell: String },
    SelectView(CellViewRef),
    OpenView(CellViewRef, ViewType),
    ReportUsage(CellViewRef),
    NewCell,
    NewView { library: String, cell: String },
    CopyCell { library: String, cell: String },
    RenameCell { library: String, cell: String },
    DeleteCell { library: String, cell: String },
    DeleteView(CellViewRef),
    ShowAuditHistory,
    ShowEditLocks,
    PublishLibrary,
    RollbackPublication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LibraryUsageReport {
    reference: CellViewRef,
    consumers: Vec<String>,
}

struct BrowserRowSpec<'a> {
    icon: WorkbenchIcon,
    label: &'a str,
    meta: &'a str,
    status: &'a str,
    selected: bool,
    read_only: bool,
}

pub(super) fn library(ui: &mut Ui, app: &mut RSpiceApp) {
    let ctx = ui.ctx().clone();
    ensure_valid_selection(
        &mut app.state.library_manager,
        &app.state.workspace.active_view,
    );
    library_context(ui, &mut app.state);

    let available = ui.available_size();
    let browser_height = (available.y - DETAIL_HEIGHT).max(120.0);
    let visible_width = available.x.min(visible_workspace_width(ui)).max(1.0);
    let mut intent = None;
    ScrollArea::horizontal()
        .id_salt("workbench.project.library.columns")
        .auto_shrink([false, true])
        .max_height(browser_height)
        .show(ui, |ui| {
            let width = visible_width.max(MIN_BROWSER_WIDTH);
            ui.set_min_width(width);
            let library_width = (width * 0.25).max(150.0);
            let cell_width = (width * 0.37).max(210.0);
            let view_width = (width - library_width - cell_width).max(220.0);
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let library_intent = library_column(ui, app, library_width, browser_height);
                let cell_intent = cell_column(ui, app, cell_width, browser_height);
                let view_intent = view_column(ui, app, view_width, browser_height);
                intent = library_intent.or(cell_intent).or(view_intent);
            });
        });
    if let Some(intent) = intent {
        apply_library_intent(&ctx, app, intent);
    }
    if let Some(intent) = library_detail(ui, &app.state) {
        apply_library_intent(&ctx, app, intent);
    }
    show_library_usage_report(&ctx);
    show_library_audit_history(&ctx, &app.state);
    show_library_edit_locks(&ctx, &app.state);
    super::library_publication::show(&ctx, app);
}

fn ensure_valid_selection(manager: &mut crate::state::LibraryManager, active_view: &CellViewRef) {
    let selected_library_valid = manager
        .selected_library
        .as_deref()
        .is_some_and(|name| manager.get_library(name).is_some());
    if !selected_library_valid {
        let preferred = manager
            .get_library(&active_view.library)
            .map(|library| library.name.clone())
            .or_else(|| {
                manager
                    .libraries_sorted()
                    .first()
                    .map(|library| library.name.clone())
            });
        if let Some(library) = preferred {
            manager.select_library(&library);
        }
    }

    let Some(library_name) = manager.selected_library.clone() else {
        return;
    };
    let selected_cell_valid = manager
        .get_library(&library_name)
        .and_then(|library| {
            manager
                .selected_cell
                .as_deref()
                .and_then(|name| library.get_cell(name))
        })
        .is_some();
    if !selected_cell_valid {
        let preferred = manager.get_library(&library_name).and_then(|library| {
            library
                .get_cell(&active_view.cell)
                .map(|cell| cell.name.clone())
                .or_else(|| library.cells_sorted().first().map(|cell| cell.name.clone()))
        });
        if let Some(cell) = preferred {
            manager.select_cell(&library_name, &cell);
        }
    }

    let Some(cell_name) = manager.selected_cell.clone() else {
        return;
    };
    let selected_view_valid = manager
        .get_library(&library_name)
        .and_then(|library| library.get_cell(&cell_name))
        .and_then(|cell| {
            manager
                .selected_view
                .as_deref()
                .and_then(|name| cell.get_view(name))
        })
        .is_some();
    if !selected_view_valid {
        let preferred = manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell(&cell_name))
            .and_then(|cell| {
                cell.get_view(&active_view.view)
                    .map(|view| view.name.clone())
                    .or_else(|| cell.views_sorted().first().map(|view| view.name.clone()))
            });
        if let Some(view) = preferred {
            manager.select_view(&library_name, &cell_name, &view);
        }
    }
}

fn library_context(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let selection = state
        .library_manager
        .selected_lcv_path()
        .unwrap_or_else(|| "No library selection".to_owned());
    let access = state
        .library_manager
        .current_library()
        .map_or("unavailable", |library| {
            if library.read_only {
                "read only"
            } else {
                "writable"
            }
        });
    let response = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.set_min_height(CONTEXT_HEIGHT - 10.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                let search_width = (ui.available_width() * 0.38).clamp(180.0, 420.0);
                let search = ui.add_sized(
                    [search_width, t.metrics.ctl_h.min(28.0)],
                    egui::TextEdit::singleline(&mut state.workbench.project_library_filter)
                        .hint_text("Filter cells and views\u{2026}"),
                );
                ui.ctx().accesskit_node_builder(search.id, |node| {
                    node.set_label("Filter project cells and views");
                });
                ui.separator();
                context_value(ui, "Selection", &selection, t.color.text);
                ui.separator();
                context_value(
                    ui,
                    "Access",
                    access,
                    if access == "writable" {
                        t.color.ok
                    } else {
                        t.color.text_dim
                    },
                );
            });
        });
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn context_value(ui: &mut Ui, label: &str, value: &str, color: Color32) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
    ui.label(
        egui::RichText::new(value)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(color),
    );
}

fn library_column(ui: &mut Ui, app: &RSpiceApp, width: f32, height: f32) -> Option<LibraryIntent> {
    let libraries = app
        .state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            (
                library.name.clone(),
                library.cell_count(),
                library.read_only,
            )
        })
        .collect::<Vec<_>>();
    let selected = app.state.library_manager.selected_library.as_deref();
    browser_column(ui, "Library", libraries.len(), width, height, |ui| {
        let mut intent = None;
        for (name, cells, read_only) in &libraries {
            let cell_count = cells.to_string();
            if browser_row(
                ui,
                BrowserRowSpec {
                    icon: WorkbenchIcon::Folder,
                    label: name,
                    meta: &cell_count,
                    status: if *read_only { "RO" } else { "RW" },
                    selected: selected == Some(name.as_str()),
                    read_only: *read_only,
                },
            )
            .clicked()
            {
                intent = Some(LibraryIntent::SelectLibrary(name.clone()));
            }
        }
        intent
    })
}

fn cell_column(ui: &mut Ui, app: &RSpiceApp, width: f32, height: f32) -> Option<LibraryIntent> {
    let selected_library = app.state.library_manager.selected_library.clone();
    let query = app
        .state
        .workbench
        .project_library_filter
        .trim()
        .to_ascii_lowercase();
    let cells = selected_library
        .as_deref()
        .and_then(|library| app.state.library_manager.get_library(library))
        .map(|library| {
            library
                .cells_sorted()
                .into_iter()
                .filter(|cell| {
                    query.is_empty()
                        || cell.name.to_ascii_lowercase().contains(&query)
                        || cell.description.to_ascii_lowercase().contains(&query)
                        || cell
                            .views_sorted()
                            .iter()
                            .any(|view| view.name.to_ascii_lowercase().contains(&query))
                })
                .map(|cell| {
                    (
                        cell.name.clone(),
                        cell.view_count(),
                        cell.description.clone(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_cell = app.state.library_manager.selected_cell.as_deref();
    browser_column(ui, "Cell", cells.len(), width, height, |ui| {
        let mut intent = None;
        for (name, views, description) in &cells {
            let view_count = views.to_string();
            let response = browser_row(
                ui,
                BrowserRowSpec {
                    icon: WorkbenchIcon::Project,
                    label: name,
                    meta: &view_count,
                    status: "",
                    selected: selected_cell == Some(name.as_str()),
                    read_only: false,
                },
            );
            let response = if description.trim().is_empty() {
                response
            } else {
                response.on_hover_text(description)
            };
            if response.clicked()
                && let Some(library) = selected_library.as_ref()
            {
                intent = Some(LibraryIntent::SelectCell {
                    library: library.clone(),
                    cell: name.clone(),
                });
            }
        }
        intent
    })
}

fn view_column(ui: &mut Ui, app: &RSpiceApp, width: f32, height: f32) -> Option<LibraryIntent> {
    let selected_library = app.state.library_manager.selected_library.clone();
    let selected_cell = app.state.library_manager.selected_cell.clone();
    let views = selected_library
        .as_deref()
        .and_then(|library| app.state.library_manager.get_library(library))
        .and_then(|library| {
            selected_cell
                .as_deref()
                .and_then(|cell| library.get_cell(cell))
        })
        .map(|cell| {
            cell.views_sorted()
                .into_iter()
                .map(|view| {
                    (
                        view.name.clone(),
                        view.view_type,
                        view.modified,
                        view.file_path.clone(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let selected_view = app.state.library_manager.selected_view.as_deref();
    browser_column(ui, "View", views.len(), width, height, |ui| {
        let mut intent = None;
        for (name, view_type, modified, path) in &views {
            let status = if *modified { "modified" } else { "saved" };
            let response = browser_row(
                ui,
                BrowserRowSpec {
                    icon: icon_for_view(*view_type),
                    label: name,
                    meta: view_type.display_name(),
                    status,
                    selected: selected_view == Some(name.as_str()),
                    read_only: false,
                },
            );
            let response = if let Some(path) = path {
                response.on_hover_text(path.display().to_string())
            } else {
                response
            };
            if (response.clicked() || response.double_clicked())
                && let (Some(library), Some(cell)) =
                    (selected_library.as_ref(), selected_cell.as_ref())
            {
                let reference = CellViewRef::new(library, cell, name);
                intent = Some(if response.double_clicked() {
                    LibraryIntent::OpenView(reference, *view_type)
                } else {
                    LibraryIntent::SelectView(reference)
                });
            }
        }
        intent
    })
}

fn browser_column(
    ui: &mut Ui,
    title: &str,
    count: usize,
    width: f32,
    height: f32,
    rows: impl FnOnce(&mut Ui) -> Option<LibraryIntent>,
) -> Option<LibraryIntent> {
    let t = Tokens::get(ui.ctx());
    let shown =
        ui.allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
            ui.set_width(width);
            let (header, _) =
                ui.allocate_exact_size(vec2(width, COLUMN_HEADER_HEIGHT), Sense::hover());
            ui.painter().rect_filled(header, 0.0, t.color.bg_panel);
            ui.painter().text(
                pos2(header.left() + 10.0, header.center().y),
                Align2::LEFT_CENTER,
                title,
                theme::sans(tokens::FS_0, FontWeight::SemiBold),
                t.color.text,
            );
            ui.painter().text(
                pos2(header.right() - 10.0, header.center().y),
                Align2::RIGHT_CENTER,
                count.to_string(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            );
            ui.painter().hline(
                header.x_range(),
                header.bottom(),
                Stroke::new(1.0, t.color.border),
            );
            ScrollArea::vertical()
                .id_salt(("workbench.project.library.column", title))
                .auto_shrink([false, false])
                .show(ui, rows)
                .inner
        });
    ui.painter().vline(
        shown.response.rect.right(),
        shown.response.rect.y_range(),
        Stroke::new(1.0, t.color.border_strong),
    );
    shown.inner
}

fn browser_row(ui: &mut Ui, spec: BrowserRowSpec<'_>) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), BROWSER_ROW_HEIGHT),
        Sense::click(),
    );
    if spec.selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_active);
        ui.painter().rect_filled(
            Rect::from_min_size(rect.left_top(), vec2(2.0, rect.height())),
            0.0,
            t.color.accent,
        );
    } else if response.hovered() || response.has_focus() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    spec.icon.paint(
        ui.painter(),
        Rect::from_center_size(pos2(rect.left() + 17.0, rect.center().y), vec2(14.0, 14.0)),
        if spec.read_only {
            t.color.text_faint
        } else {
            t.color.accent
        },
    );
    ui.painter().text(
        pos2(rect.left() + 31.0, rect.center().y),
        Align2::LEFT_CENTER,
        spec.label,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    let status_width = if spec.status.is_empty() { 0.0 } else { 52.0 };
    ui.painter().text(
        pos2(rect.right() - status_width - 8.0, rect.center().y),
        Align2::RIGHT_CENTER,
        spec.meta,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    if !spec.status.is_empty() {
        ui.painter().text(
            pos2(rect.right() - 8.0, rect.center().y),
            Align2::RIGHT_CENTER,
            spec.status,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            if spec.status == "modified" {
                t.color.warn
            } else {
                t.color.text_dim
            },
        );
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            spec.selected,
            spec.label,
        )
    });
    theme::paint_focus_ring_outset(ui, &response, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn library_detail(ui: &mut Ui, state: &AppState) -> Option<LibraryIntent> {
    let t = Tokens::get(ui.ctx());
    let selected = selected_view(state);
    let selected_cell = selected_cell(state);
    let writable_library_available = state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .any(|library| !library.read_only);
    let mut intent = None;
    let shown = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.set_min_height(DETAIL_HEIGHT - 10.0);
            ui.horizontal(|ui| {
                match selected.as_ref() {
                    Some((reference, view_type, modified, read_only)) => {
                        ui.label(
                            egui::RichText::new(reference.display_path())
                                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.text),
                        );
                        ui.separator();
                        ui.label(view_type.display_name());
                        ui.separator();
                        ui.label(if *read_only { "read only" } else { "writable" });
                        if *modified {
                            ui.separator();
                            ui.colored_label(t.color.warn, "working changes");
                        }
                    }
                    None => {
                        ui.label(
                            egui::RichText::new("Select a view to inspect or open")
                                .color(t.color.text_dim),
                        );
                    }
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let governance_write_allowed = state.project_lifecycle.project_open
                        && !state.workbench.safe_mode.project_read_only()
                        && !state.simulation.has_active_execution();
                    let rollback = Button::new("Rollback\u{2026}")
                        .enabled(
                            governance_write_allowed
                                && !state.workspace.project.library_publications().is_empty(),
                        )
                        .show(ui)
                        .on_disabled_hover_text(if state.simulation.has_active_execution() {
                            "Library rollback is unavailable while a simulation is running."
                        } else if state.workbench.safe_mode.project_read_only() {
                            "Library rollback is unavailable while the project is read-only."
                        } else {
                            "No retained library publication is available to roll back."
                        });
                    if rollback.clicked() {
                        intent = Some(LibraryIntent::RollbackPublication);
                    }
                    let publish = Button::new("Publish\u{2026}")
                        .enabled(governance_write_allowed)
                        .show(ui)
                        .on_disabled_hover_text(if state.simulation.has_active_execution() {
                            "Library publication is unavailable while a simulation is running."
                        } else {
                            "Library publication is unavailable while the project is read-only."
                        });
                    if publish.clicked() {
                        intent = Some(LibraryIntent::PublishLibrary);
                    }
                    if Button::new("Edit locks\u{2026}").show(ui).clicked() {
                        intent = Some(LibraryIntent::ShowEditLocks);
                    }
                    if Button::new("Audit history\u{2026}").show(ui).clicked() {
                        intent = Some(LibraryIntent::ShowAuditHistory);
                    }
                    if Button::new("New cell\u{2026}").show(ui).clicked() {
                        intent = Some(LibraryIntent::NewCell);
                    }
                    if let Some((reference, _, _, _)) = selected.clone()
                        && Button::new("Where used\u{2026}").show(ui).clicked()
                    {
                        intent = Some(LibraryIntent::ReportUsage(reference));
                    }
                    if let Some((reference, view_type, _, _)) = selected.clone()
                        && Button::new(open_view_label(view_type))
                            .accent()
                            .show(ui)
                            .clicked()
                    {
                        intent = Some(LibraryIntent::OpenView(reference, view_type));
                    }
                    if let Some((library, cell, read_only)) = selected_cell.clone() {
                        ui.menu_button("Cell actions", |ui| {
                            if ui
                                .add_enabled(!read_only, egui::Button::new("New view\u{2026}"))
                                .on_disabled_hover_text(
                                    "The selected library is read-only; create the view in a writable library.",
                                )
                                .clicked()
                            {
                                intent = Some(LibraryIntent::NewView {
                                    library: library.clone(),
                                    cell: cell.clone(),
                                });
                                ui.close();
                            }
                            if ui
                                .add_enabled(
                                    writable_library_available,
                                    egui::Button::new("Copy cell\u{2026}"),
                                )
                                .on_disabled_hover_text(
                                    "No writable destination library is available.",
                                )
                                .clicked()
                            {
                                intent = Some(LibraryIntent::CopyCell {
                                    library: library.clone(),
                                    cell: cell.clone(),
                                });
                                ui.close();
                            }
                            if ui
                                .add_enabled(!read_only, egui::Button::new("Rename cell\u{2026}"))
                                .on_disabled_hover_text(
                                    "The selected library is read-only and cannot be renamed.",
                                )
                                .clicked()
                            {
                                intent = Some(LibraryIntent::RenameCell {
                                    library: library.clone(),
                                    cell: cell.clone(),
                                });
                                ui.close();
                            }
                            ui.separator();
                            if let Some((reference, _, _, _)) = selected.clone()
                                && ui
                                    .add_enabled(
                                        !read_only,
                                        egui::Button::new("Delete view\u{2026}"),
                                    )
                                    .on_disabled_hover_text(
                                        "The selected library is read-only and cannot be changed.",
                                    )
                                    .clicked()
                            {
                                intent = Some(LibraryIntent::DeleteView(reference));
                                ui.close();
                            }
                            if ui
                                .add_enabled(
                                    !read_only,
                                    egui::Button::new("Delete cell\u{2026}"),
                                )
                                .on_disabled_hover_text(
                                    "The selected library is read-only and cannot be changed.",
                                )
                                .clicked()
                            {
                                intent = Some(LibraryIntent::DeleteCell { library, cell });
                                ui.close();
                            }
                        });
                    }
                });
            });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    intent
}

fn selected_cell(state: &AppState) -> Option<(String, String, bool)> {
    let manager = &state.library_manager;
    let library = manager.current_library()?;
    let cell = manager.current_cell()?;
    Some((library.name.clone(), cell.name.clone(), library.read_only))
}

fn selected_view(state: &AppState) -> Option<(CellViewRef, ViewType, bool, bool)> {
    let manager = &state.library_manager;
    let library = manager.current_library()?;
    let cell = manager.current_cell()?;
    let view = manager.current_view()?;
    Some((
        CellViewRef::new(&library.name, &cell.name, &view.name),
        view.view_type,
        view.modified,
        library.read_only,
    ))
}

fn apply_library_intent(ctx: &Context, app: &mut RSpiceApp, intent: LibraryIntent) {
    match intent {
        LibraryIntent::SelectLibrary(library) => {
            app.state.library_manager.select_library(&library);
        }
        LibraryIntent::SelectCell { library, cell } => {
            app.state.library_manager.select_cell(&library, &cell);
        }
        LibraryIntent::SelectView(reference) => {
            app.state.library_manager.select_view(
                &reference.library,
                &reference.cell,
                &reference.view,
            );
        }
        LibraryIntent::OpenView(reference, view_type) => {
            app.state.open_workspace_view(reference);
            let workspace = match view_type {
                ViewType::Verilog | ViewType::VerilogA | ViewType::Spice => Workspace::Netlist,
                ViewType::Config => {
                    app.state.workbench.project_page = ProjectPage::Configuration;
                    Workspace::Project
                }
                _ => Workspace::Design,
            };
            Command::OpenWorkspace(workspace).execute(app);
        }
        LibraryIntent::ReportUsage(reference) => {
            let consumers = library_view_usage(&app.state, &reference);
            let count = consumers.len();
            ctx.data_mut(|data| {
                data.insert_temp(
                    egui::Id::new(USAGE_REPORT_ID),
                    LibraryUsageReport {
                        reference: reference.clone(),
                        consumers,
                    },
                );
            });
            app.state.push_user_message(ConsoleMessage::info(format!(
                "{} has {count} loaded schematic consumer{}.",
                reference.display_path(),
                if count == 1 { "" } else { "s" }
            )));
        }
        LibraryIntent::NewCell => Command::NewCell.execute(app),
        LibraryIntent::NewView { library, cell } => {
            if let Err(error) = app.state.open_new_view_dialog(&library, &cell) {
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        }
        LibraryIntent::CopyCell { library, cell } => {
            if let Err(error) = app.state.open_copy_cell_dialog(&library, &cell) {
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        }
        LibraryIntent::RenameCell { library, cell } => {
            if let Err(error) = app.state.open_rename_cell_dialog(&library, &cell) {
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        }
        LibraryIntent::DeleteCell { library, cell } => {
            if let Err(error) = app.state.open_library_cell_deletion_review(&library, &cell) {
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        }
        LibraryIntent::DeleteView(reference) => {
            if let Err(error) = app.state.open_library_view_deletion_review(
                &reference.library,
                &reference.cell,
                &reference.view,
            ) {
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        }
        LibraryIntent::ShowAuditHistory => {
            ctx.data_mut(|data| {
                data.insert_temp(egui::Id::new(AUDIT_REPORT_ID), true);
            });
        }
        LibraryIntent::ShowEditLocks => {
            ctx.data_mut(|data| {
                data.insert_temp(egui::Id::new(LOCK_REPORT_ID), true);
            });
        }
        LibraryIntent::PublishLibrary => {
            super::library_publication::open_publication(ctx, &app.state);
        }
        LibraryIntent::RollbackPublication => {
            super::library_publication::open_rollback(ctx, &app.state);
        }
    }
}

fn library_view_usage(state: &AppState, reference: &CellViewRef) -> Vec<String> {
    let mut consumers = Vec::new();
    let active_key = state.workspace.active_view.key();
    collect_library_view_usage(
        &state.schematic,
        &state.workspace.active_view.display_path(),
        reference,
        &mut consumers,
    );
    for (key, schematic) in &state.workspace.schematic_buffers {
        if key == &active_key {
            continue;
        }
        collect_library_view_usage(schematic, key, reference, &mut consumers);
    }
    consumers.sort_by_key(|consumer| consumer.to_ascii_lowercase());
    consumers.dedup();
    consumers
}

fn collect_library_view_usage(
    schematic: &crate::state::SchematicState,
    owner: &str,
    reference: &CellViewRef,
    consumers: &mut Vec<String>,
) {
    for component in &schematic.components {
        let Some(binding) = component.library_cell.as_ref() else {
            continue;
        };
        if binding.library.eq_ignore_ascii_case(&reference.library)
            && binding.cell.eq_ignore_ascii_case(&reference.cell)
            && binding.view.eq_ignore_ascii_case(&reference.view)
        {
            consumers.push(format!("{owner} / {}", component.name));
        }
    }
}

fn show_library_usage_report(ctx: &Context) {
    let Some(report) =
        ctx.data(|data| data.get_temp::<LibraryUsageReport>(egui::Id::new(USAGE_REPORT_ID)))
    else {
        return;
    };
    let description = format!(
        "Loaded schematic consumers of {}. This report is derived from current live buffers.",
        report.reference.display_path()
    );
    let choice = Dialog::new("PROJECT \u{00b7} LIBRARY", "Where used", "Close")
        .description(&description)
        .size(DialogSize::Transaction)
        .show(ctx, |ui| {
            property_card(ui, "Selected library view", |ui| {
                property_row(
                    ui,
                    "Library / cell / view",
                    &report.reference.display_path(),
                );
                property_row(ui, "Loaded consumers", &report.consumers.len().to_string());
            });
            ui.add_space(8.0);
            property_card(ui, "Schematic instances", |ui| {
                if report.consumers.is_empty() {
                    property_row(ui, "Consumers", "none in loaded schematic documents");
                } else {
                    for consumer in &report.consumers {
                        property_row(ui, "Instance", consumer);
                    }
                }
            });
        });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        ctx.data_mut(|data| {
            data.remove::<LibraryUsageReport>(egui::Id::new(USAGE_REPORT_ID));
        });
    }
}

fn show_library_audit_history(ctx: &Context, state: &AppState) {
    if !ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new(AUDIT_REPORT_ID))
            .unwrap_or(false)
    }) {
        return;
    }

    let receipts = state.workspace.project.library_mutation_audit();
    let publications = state.workspace.project.library_publications();
    let description = format!(
        "{} immutable mutation receipt{} and {} content-addressed publication{}.",
        receipts.len(),
        if receipts.len() == 1 { "" } else { "s" },
        publications.len(),
        if publications.len() == 1 { "" } else { "s" }
    );
    let choice = Dialog::new("PROJECT \u{00b7} LIBRARY", "Audit history", "Close")
        .description(&description)
        .size(DialogSize::Manager)
        .show(ctx, |ui| {
            property_card(ui, "Audit chain", |ui| {
                property_row(
                    ui,
                    "Project revision",
                    &state.workspace.project.revision().get().to_string(),
                );
                property_row(
                    ui,
                    "Library revision",
                    &state.library_manager.revision().to_string(),
                );
                property_row(ui, "Retained receipts", &receipts.len().to_string());
                property_row(ui, "Retained publications", &publications.len().to_string());
            });
            ui.add_space(8.0);
            ScrollArea::vertical()
                .id_salt("workbench.project.library.audit-history.receipts")
                .max_height(360.0)
                .show(ui, |ui| {
                    if receipts.is_empty() && publications.is_empty() {
                        property_card(ui, "No retained mutations", |ui| {
                            property_row(
                                ui,
                                "Status",
                                "The project has no audited library changes or publications.",
                            );
                        });
                    }
                    for publication in publications.iter().rev() {
                        let title = format!(
                            "Publication #{:04} \u{00b7} {}",
                            publication.sequence(),
                            publication.label()
                        );
                        property_card(ui, &title, |ui| {
                            property_row(
                                ui,
                                "Publication identity",
                                &publication.publication_id().to_string(),
                            );
                            property_row(ui, "Actor", publication.actor_id());
                            property_row(ui, "Authority", publication.authority_id());
                            property_row(ui, "Reason", publication.reason());
                            property_row(
                                ui,
                                "Project revision",
                                &format!(
                                    "{} \u{2192} {}",
                                    publication.source_project_revision().get(),
                                    publication.receipt_project_revision().get()
                                ),
                            );
                            property_row(
                                ui,
                                "Library revision",
                                &publication.library_revision().to_string(),
                            );
                            property_row(
                                ui,
                                "Artifact",
                                &format!(
                                    "{} bytes \u{00b7} {}",
                                    publication.snapshot_byte_len(),
                                    publication.snapshot_digest()
                                ),
                            );
                            property_row(
                                ui,
                                "Receipt digest",
                                &publication.receipt_digest().to_string(),
                            );
                        });
                        ui.add_space(6.0);
                    }
                    for receipt in receipts.iter().rev() {
                        let title = format!(
                            "#{:04} \u{00b7} {}",
                            receipt.sequence(),
                            receipt.mutation().operation_label()
                        );
                        property_card(ui, &title, |ui| {
                            property_row(
                                ui,
                                "Object",
                                &library_mutation_summary(receipt.mutation()),
                            );
                            property_row(
                                ui,
                                "Project revision",
                                &format!(
                                    "{} \u{2192} {}",
                                    receipt.source_project_revision().get(),
                                    receipt.target_project_revision().get()
                                ),
                            );
                            property_row(
                                ui,
                                "Library revision",
                                &format!(
                                    "{} \u{2192} {}",
                                    receipt.source_library_revision(),
                                    receipt.target_library_revision()
                                ),
                            );
                            property_row(
                                ui,
                                "Receipt digest",
                                &receipt.receipt_digest().to_string(),
                            );
                        });
                        ui.add_space(6.0);
                    }
                });
        });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        ctx.data_mut(|data| {
            data.remove::<bool>(egui::Id::new(AUDIT_REPORT_ID));
        });
    }
}

fn show_library_edit_locks(ctx: &Context, state: &AppState) {
    if !ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new(LOCK_REPORT_ID))
            .unwrap_or(false)
    }) {
        return;
    }

    let snapshot = state.library_edit_locks.snapshot();
    let description = match snapshot {
        Some(snapshot) => format!(
            "{} authoritative edit lock{} at synchronization generation {}.",
            snapshot.locks().len(),
            if snapshot.locks().len() == 1 { "" } else { "s" },
            snapshot.generation()
        ),
        None => {
            "This device-local project has no connected collaboration lock authority.".to_owned()
        }
    };
    let choice = Dialog::new("PROJECT \u{00b7} LIBRARY", "Edit locks", "Close")
        .description(&description)
        .size(DialogSize::Manager)
        .show(ctx, |ui| {
            property_card(ui, "Authority boundary", |ui| match snapshot {
                Some(snapshot) => {
                    property_row(ui, "Mode", "authoritative collaboration snapshot");
                    property_row(ui, "Project", &snapshot.project_id().to_string());
                    property_row(ui, "Authority", snapshot.authority_id());
                    property_row(ui, "Generation", &snapshot.generation().to_string());
                    property_row(
                        ui,
                        "Project revision",
                        &snapshot.project_revision().get().to_string(),
                    );
                    property_row(
                        ui,
                        "Library revision",
                        &snapshot.library_revision().to_string(),
                    );
                    property_row(
                        ui,
                        "Snapshot digest",
                        &snapshot.snapshot_digest().to_string(),
                    );
                }
                None => {
                    property_row(ui, "Mode", "unmanaged local project");
                    property_row(
                        ui,
                        "Behavior",
                        "Local revision checks remain active; no distributed lock is inferred.",
                    );
                }
            });
            ui.add_space(8.0);
            ScrollArea::vertical()
                .id_salt("workbench.project.library.edit-locks.rows")
                .max_height(360.0)
                .show(ui, |ui| {
                    let Some(snapshot) = snapshot else {
                        return;
                    };
                    if snapshot.locks().is_empty() {
                        property_card(ui, "No active leases", |ui| {
                            property_row(
                                ui,
                                "Status",
                                "The authoritative snapshot contains no library edit locks.",
                            );
                        });
                    }
                    for lock in snapshot.locks() {
                        property_card(ui, &lock.scope().display_path(), |ui| {
                            property_row(ui, "Owner", lock.owner_id());
                            property_row(ui, "Authority", lock.authority_id());
                            property_row(ui, "Lease identity", &lock.lock_id().to_string());
                            property_row(
                                ui,
                                "Acquired (Unix ms)",
                                &lock.acquired_unix_ms().to_string(),
                            );
                            property_row(
                                ui,
                                "Lease until (Unix ms)",
                                &lock.lease_until_unix_ms().to_string(),
                            );
                        });
                        ui.add_space(6.0);
                    }
                });
        });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        ctx.data_mut(|data| {
            data.remove::<bool>(egui::Id::new(LOCK_REPORT_ID));
        });
    }
}

fn library_mutation_summary(mutation: &crate::state::ProjectLibraryMutation) -> String {
    match mutation {
        crate::state::ProjectLibraryMutation::CreateCell { library, cell }
        | crate::state::ProjectLibraryMutation::DeleteCell { library, cell } => {
            format!("{library}/{cell}")
        }
        crate::state::ProjectLibraryMutation::CreateView {
            library,
            cell,
            view,
        }
        | crate::state::ProjectLibraryMutation::DeleteView {
            library,
            cell,
            view,
        } => format!("{library}/{cell}/{view}"),
        crate::state::ProjectLibraryMutation::CopyCell {
            source_library,
            source_cell,
            target_library,
            target_cell,
        } => format!("{source_library}/{source_cell} \u{2192} {target_library}/{target_cell}"),
        crate::state::ProjectLibraryMutation::RenameCell {
            library,
            from_cell,
            to_cell,
        } => format!("{library}/{from_cell} \u{2192} {library}/{to_cell}"),
        crate::state::ProjectLibraryMutation::RollbackPublication {
            publication_id,
            publication_label,
            ..
        } => format!("{publication_label} ({publication_id})"),
    }
}

fn icon_for_view(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench | ViewType::Extracted => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Project,
        ViewType::Verilog | ViewType::VerilogA | ViewType::Spice => WorkbenchIcon::Code,
        ViewType::Config => WorkbenchIcon::Sliders,
        ViewType::Layout | ViewType::Document | ViewType::Abstract | ViewType::Custom => {
            WorkbenchIcon::File
        }
    }
}

fn open_view_label(view_type: ViewType) -> &'static str {
    match view_type {
        ViewType::Schematic => "Open schematic",
        ViewType::Symbol => "Open symbol",
        ViewType::Layout => "Open layout",
        ViewType::Testbench => "Open testbench",
        ViewType::Verilog => "Open Verilog",
        ViewType::VerilogA => "Open Verilog-A",
        ViewType::Spice => "Open SPICE source",
        ViewType::Document => "Open document",
        ViewType::Extracted => "Open extracted view",
        ViewType::Abstract => "Open abstract view",
        ViewType::Config => "Open configuration",
        ViewType::Custom => "Open view",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, Library, View};

    #[test]
    fn selection_repairs_to_real_library_cell_and_view() {
        let app = RSpiceApp::test_instance();
        let mut manager = app.state.library_manager.clone();
        manager.selected_library = Some("missing".to_owned());
        manager.selected_cell = Some("missing".to_owned());
        manager.selected_view = Some("missing".to_owned());

        ensure_valid_selection(&mut manager, &app.state.workspace.active_view);

        assert!(manager.current_library().is_some());
        assert!(manager.current_cell().is_some());
        assert!(manager.current_view().is_some());
    }

    #[test]
    fn source_views_route_to_the_netlist_workspace() {
        assert_eq!(icon_for_view(ViewType::VerilogA), WorkbenchIcon::Code);
        assert_eq!(icon_for_view(ViewType::Spice), WorkbenchIcon::Code);
        assert_eq!(icon_for_view(ViewType::Config), WorkbenchIcon::Sliders);

        let mut app = RSpiceApp::test_instance();
        let reference = app.state.workspace.active_view.clone();
        apply_library_intent(
            &egui::Context::default(),
            &mut app,
            LibraryIntent::OpenView(reference, ViewType::Spice),
        );
        assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    }

    #[test]
    fn configuration_views_route_to_the_project_configuration_page() {
        let mut app = RSpiceApp::test_instance();
        let reference = app.state.workspace.active_view.clone();
        apply_library_intent(
            &egui::Context::default(),
            &mut app,
            LibraryIntent::OpenView(reference, ViewType::Config),
        );
        assert_eq!(app.state.workbench.workspace, Workspace::Project);
        assert_eq!(app.state.workbench.project_page, ProjectPage::Configuration);
    }

    #[test]
    fn cell_action_intents_open_exact_validated_transactions() {
        let mut app = RSpiceApp::test_instance();
        let mut library = Library::new("intent_test");
        let mut cell = Cell::new("filter");
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
        let ctx = egui::Context::default();

        apply_library_intent(
            &ctx,
            &mut app,
            LibraryIntent::NewView {
                library: "intent_test".to_owned(),
                cell: "filter".to_owned(),
            },
        );
        assert!(app.state.dialogs.new_view_dialog);
        assert_eq!(app.state.dialogs.new_view_library, "intent_test");
        assert_eq!(app.state.dialogs.new_view_cell, "filter");

        apply_library_intent(
            &ctx,
            &mut app,
            LibraryIntent::CopyCell {
                library: "intent_test".to_owned(),
                cell: "filter".to_owned(),
            },
        );
        assert!(app.state.dialogs.copy_cell_dialog);
        assert_eq!(app.state.dialogs.copy_cell_source_library, "intent_test");
        assert_eq!(app.state.dialogs.copy_cell_source_cell, "filter");

        apply_library_intent(
            &ctx,
            &mut app,
            LibraryIntent::RenameCell {
                library: "intent_test".to_owned(),
                cell: "filter".to_owned(),
            },
        );
        assert!(app.state.dialogs.rename_cell_dialog);
        assert_eq!(app.state.dialogs.rename_cell_library, "intent_test");
        assert_eq!(app.state.dialogs.rename_cell_current, "filter");

        apply_library_intent(
            &ctx,
            &mut app,
            LibraryIntent::DeleteView(CellViewRef::new("intent_test", "filter", "symbol")),
        );
        assert_eq!(
            app.state
                .dialogs
                .library_deletion_review
                .target
                .as_ref()
                .map(|target| target.display_path()),
            Some("intent_test/filter/symbol".to_owned())
        );
        app.state.dialogs.library_deletion_review.close();

        apply_library_intent(
            &ctx,
            &mut app,
            LibraryIntent::DeleteCell {
                library: "intent_test".to_owned(),
                cell: "filter".to_owned(),
            },
        );
        assert_eq!(
            app.state
                .dialogs
                .library_deletion_review
                .target
                .as_ref()
                .map(|target| target.display_path()),
            Some("intent_test/filter".to_owned())
        );
    }

    #[test]
    fn stale_cell_action_intents_leave_all_transaction_dialogs_closed() {
        let mut app = RSpiceApp::test_instance();
        let ctx = egui::Context::default();

        for intent in [
            LibraryIntent::NewView {
                library: "missing".to_owned(),
                cell: "filter".to_owned(),
            },
            LibraryIntent::CopyCell {
                library: "missing".to_owned(),
                cell: "filter".to_owned(),
            },
            LibraryIntent::RenameCell {
                library: "missing".to_owned(),
                cell: "filter".to_owned(),
            },
            LibraryIntent::DeleteCell {
                library: "missing".to_owned(),
                cell: "filter".to_owned(),
            },
            LibraryIntent::DeleteView(CellViewRef::new("missing", "filter", "symbol")),
        ] {
            apply_library_intent(&ctx, &mut app, intent);
        }

        assert!(!app.state.dialogs.new_view_dialog);
        assert!(!app.state.dialogs.copy_cell_dialog);
        assert!(!app.state.dialogs.rename_cell_dialog);
        assert!(app.state.dialogs.library_deletion_review.target.is_none());
    }

    #[test]
    fn where_used_report_reads_live_and_inactive_schematic_buffers() {
        let mut app = RSpiceApp::test_instance();
        let reference = app.state.workspace.active_view.clone();
        let binding = crate::state::LibraryCellInstance::new(
            &reference.library,
            &reference.cell,
            &reference.view,
        );
        app.state
            .schematic
            .add_library_cell_component(crate::state::Point::origin(), binding.clone());

        let mut inactive = crate::state::SchematicState::default();
        inactive.add_library_cell_component(crate::state::Point::origin(), binding);
        app.state
            .workspace
            .schematic_buffers
            .insert("work/consumer/schematic".to_owned(), inactive);

        let consumers = library_view_usage(&app.state, &reference);
        assert_eq!(consumers.len(), 2);
        assert!(
            consumers
                .iter()
                .any(|consumer| consumer.contains("work/consumer/schematic"))
        );
    }

    #[test]
    fn audit_history_intent_opens_the_project_library_receipt_view() {
        let mut app = RSpiceApp::test_instance();
        let ctx = egui::Context::default();

        apply_library_intent(&ctx, &mut app, LibraryIntent::ShowAuditHistory);

        assert!(ctx.data(|data| {
            data.get_temp::<bool>(egui::Id::new(AUDIT_REPORT_ID))
                .unwrap_or(false)
        }));
    }

    #[test]
    fn edit_lock_intent_opens_the_authority_view() {
        let mut app = RSpiceApp::test_instance();
        let ctx = egui::Context::default();

        apply_library_intent(&ctx, &mut app, LibraryIntent::ShowEditLocks);

        assert!(ctx.data(|data| {
            data.get_temp::<bool>(egui::Id::new(LOCK_REPORT_ID))
                .unwrap_or(false)
        }));
    }

    #[test]
    fn publication_and_rollback_intents_open_the_governed_transactions() {
        let mut app = RSpiceApp::test_instance();
        let ctx = egui::Context::default();

        apply_library_intent(&ctx, &mut app, LibraryIntent::PublishLibrary);
        assert!(super::library_publication::publication_dialog_open(&ctx));
        assert!(!super::library_publication::rollback_dialog_open(&ctx));

        let candidate = app
            .prepare_project_library_publication(
                "governance-intent-1.0.0",
                "release-engineer@example.test",
                "organization-release-authority",
                "Exercise exact rollback reachability",
            )
            .expect("publication candidate prepares");
        app.commit_project_library_publication(candidate)
            .expect("test-owned durable boundary commits");
        apply_library_intent(&ctx, &mut app, LibraryIntent::RollbackPublication);

        assert!(!super::library_publication::publication_dialog_open(&ctx));
        assert!(super::library_publication::rollback_dialog_open(&ctx));
    }

    #[test]
    fn authoritative_library_lock_blocks_preflight_without_partial_mutation() {
        use crate::state::library_browser::{
            ProjectLibraryEditLock, ProjectLibraryEditLockScope, ProjectLibraryLockSnapshot,
        };

        let mut app = RSpiceApp::test_instance();
        let project_revision = app.state.workspace.project.revision();
        let library_revision = app.state.library_manager.revision();
        let library = app
            .state
            .library_manager
            .libraries_sorted()
            .first()
            .expect("bootstrapped library")
            .name
            .clone();
        let snapshot = ProjectLibraryLockSnapshot::try_new(
            app.state.workspace.project.id(),
            1,
            project_revision,
            library_revision,
            "organization-lock-service",
            vec![ProjectLibraryEditLock::new(
                uuid::Uuid::new_v4(),
                "other-engineer@example.test",
                "organization-lock-service",
                ProjectLibraryEditLockScope::Library {
                    library: library.clone(),
                },
                10,
                20,
            )],
        )
        .expect("valid lock authority snapshot");
        app.install_project_library_lock_snapshot(snapshot)
            .expect("lock authority installs");

        let error = app
            .state
            .preflight_project_library_mutation(crate::state::ProjectLibraryMutation::CreateCell {
                library,
                cell: "locked_candidate".to_owned(),
            })
            .expect_err("foreign library lock must block the mutation");

        assert!(error.contains("locked by other-engineer@example.test"));
        assert_eq!(app.state.workspace.project.revision(), project_revision);
        assert_eq!(app.state.library_manager.revision(), library_revision);
    }

    #[test]
    fn publication_exports_exact_pre_receipt_project_and_commits_one_audited_revision() {
        use sha2::Digest as _;

        let mut app = RSpiceApp::test_instance();
        let project_id = app.state.workspace.project.id();
        let project_revision = app.state.workspace.project.revision();
        let library_revision = app.state.library_manager.revision();

        let candidate = app
            .prepare_project_library_publication(
                "analog-core-1.0.0",
                "release-engineer@example.test",
                "organization-release-authority",
                "Qualified library handoff",
            )
            .expect("publication prepares");
        let bytes = candidate.artifact_bytes().to_vec();
        let receipt = app
            .commit_project_library_publication(candidate)
            .expect("durably accepted publication commits");

        assert_eq!(receipt.project_id(), project_id);
        assert_eq!(receipt.source_project_revision(), project_revision);
        assert_eq!(
            receipt.receipt_project_revision().get(),
            project_revision.get() + 1
        );
        assert_eq!(receipt.library_revision(), library_revision);
        assert_eq!(receipt.snapshot_byte_len(), bytes.len() as u64);
        assert_eq!(
            receipt.snapshot_digest(),
            crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(&bytes).into())
        );
        let text = std::str::from_utf8(&bytes).expect("publication is UTF-8");
        let artifact =
            crate::io::project_io::load_project_text(text, None).expect("artifact validates");
        assert_eq!(artifact.workspace.project.id(), project_id);
        assert_eq!(artifact.workspace.project.revision(), project_revision);
        assert!(artifact.workspace.project.library_publications().is_empty());
        assert_eq!(
            app.state.workspace.project.library_publications(),
            [receipt.clone()]
        );
        assert!(app.state.workspace.project_metadata_dirty);
        let mut tampered_descriptor =
            serde_json::to_value(&app.state.workspace.project).expect("descriptor serializes");
        tampered_descriptor["library_publications"][0]["reason"] =
            serde_json::json!("silently changed reason");
        let tampered_descriptor: crate::state::ProjectDescriptor =
            serde_json::from_value(tampered_descriptor).expect("wire shape remains valid");
        assert!(
            tampered_descriptor.validate().is_err(),
            "publication receipt content is digest sealed"
        );

        let writable_library = app
            .state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| !library.read_only)
            .expect("writable project library")
            .name
            .clone();
        let prepared = app
            .state
            .preflight_project_library_mutation(crate::state::ProjectLibraryMutation::CreateCell {
                library: writable_library.clone(),
                cell: "post_publication_change".to_owned(),
            })
            .expect("post-publication mutation preflights");
        assert!(
            app.state
                .library_manager
                .create_cell(&writable_library, "post_publication_change")
        );
        app.state.publish_project_library_mutation(prepared);
        assert!(
            app.state
                .library_manager
                .get_library(&writable_library)
                .and_then(|library| library.get_cell("post_publication_change"))
                .is_some()
        );

        let mut tampered = bytes.clone();
        let last = tampered.last_mut().expect("nonempty publication artifact");
        *last ^= 0x01;
        let revision_before_rejection = app.state.workspace.project.revision();
        let tampered_result = app.rollback_project_library_publication(
            receipt.publication_id(),
            &tampered,
            "release-engineer@example.test",
            "organization-release-authority",
            "Tampered artifact must fail",
        );
        assert!(tampered_result.is_err());
        assert_eq!(
            app.state.workspace.project.revision(),
            revision_before_rejection
        );
        assert!(
            app.state
                .library_manager
                .get_library(&writable_library)
                .and_then(|library| library.get_cell("post_publication_change"))
                .is_some(),
            "rejected rollback must not partially replace the catalog"
        );

        app.rollback_project_library_publication(
            receipt.publication_id(),
            &bytes,
            "release-engineer@example.test",
            "organization-release-authority",
            "Restore the qualified handoff",
        )
        .expect("exact publication rollback succeeds");
        assert!(
            app.state
                .library_manager
                .get_library(&writable_library)
                .and_then(|library| library.get_cell("post_publication_change"))
                .is_none()
        );
        assert_eq!(
            app.state.workspace.project.library_publications(),
            [receipt.clone()],
            "rollback retains immutable publication lineage"
        );
        assert!(matches!(
            app.state
                .workspace
                .project
                .library_mutation_audit()
                .last()
                .map(|entry| entry.mutation()),
            Some(crate::state::ProjectLibraryMutation::RollbackPublication {
                publication_id,
                actor_id,
                authority_id,
                reason,
                ..
            }) if *publication_id == receipt.publication_id()
                && actor_id == "release-engineer@example.test"
                && authority_id == "organization-release-authority"
                && reason == "Restore the qualified handoff"
        ));

        let revision_after_first = app.state.workspace.project.revision();
        let duplicate = app.prepare_project_library_publication(
            "ANALOG-CORE-1.0.0",
            "release-engineer@example.test",
            "organization-release-authority",
            "Duplicate labels are forbidden",
        );
        assert!(duplicate.is_err());
        assert_eq!(
            app.state.workspace.project.revision(),
            revision_after_first,
            "rejected publication must not partially advance project state"
        );
    }

    #[test]
    fn publication_commit_rejects_intervening_catalog_change_atomically() {
        let mut app = RSpiceApp::test_instance();
        let candidate = app
            .prepare_project_library_publication(
                "stale-candidate-1.0.0",
                "release-engineer@example.test",
                "organization-release-authority",
                "Prepared before an intervening edit",
            )
            .expect("publication prepares");
        let writable_library = app
            .state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| !library.read_only)
            .expect("writable project library")
            .name
            .clone();
        let prepared_mutation = app
            .state
            .preflight_project_library_mutation(crate::state::ProjectLibraryMutation::CreateCell {
                library: writable_library.clone(),
                cell: "intervening_change".to_owned(),
            })
            .expect("intervening mutation preflights");
        assert!(
            app.state
                .library_manager
                .create_cell(&writable_library, "intervening_change")
        );
        app.state
            .publish_project_library_mutation(prepared_mutation);
        let revision_after_mutation = app.state.workspace.project.revision();

        let error = app
            .commit_project_library_publication(candidate)
            .expect_err("stale publication cannot commit");

        assert!(error.contains("stale"));
        assert!(
            app.state
                .workspace
                .project
                .library_publications()
                .is_empty()
        );
        assert_eq!(
            app.state.workspace.project.revision(),
            revision_after_mutation
        );
    }

    #[test]
    fn audit_history_formats_exact_source_and_target_identities() {
        assert_eq!(
            library_mutation_summary(&crate::state::ProjectLibraryMutation::CopyCell {
                source_library: "work".to_owned(),
                source_cell: "amp".to_owned(),
                target_library: "release".to_owned(),
                target_cell: "amp_v2".to_owned(),
            }),
            "work/amp \u{2192} release/amp_v2"
        );
        assert_eq!(
            library_mutation_summary(&crate::state::ProjectLibraryMutation::DeleteView {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
                view: "symbol".to_owned(),
            }),
            "work/amp/symbol"
        );
    }
}
