//! Project Explorer Panel
//!
//! Project-level navigation for the logical design database. The Library
//! Browser remains the place for component browsing and placement.

use std::path::PathBuf;

use egui::{CollapsingHeader, RichText, Ui};

use crate::common::app::AppState;
use crate::state::{CellViewRef, LibraryManager, OpenCellView, View, ViewType};

#[derive(Debug, Clone)]
struct ViewNode {
    name: String,
    view_type: ViewType,
    file_path: Option<PathBuf>,
    modified: bool,
}

#[derive(Debug, Clone)]
struct CellNode {
    name: String,
    category: String,
    views: Vec<ViewNode>,
}

#[derive(Debug, Clone)]
struct LibraryNode {
    name: String,
    path: Option<PathBuf>,
    technology: String,
    read_only: bool,
    cell_count: usize,
    view_count: usize,
    cells: Vec<CellNode>,
}

/// Render the project explorer panel.
pub fn render_project_explorer(ui: &mut Ui, state: &mut AppState) {
    let project_dirty = state.schematic.is_dirty || state.workspace.any_dirty();
    render_project_header(ui, state, project_dirty);

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(6.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            render_workspace_tree(ui, state, project_dirty);
            render_open_views_tree(ui, state);
            render_hierarchy_tree(ui, state);
            render_design_libraries_tree(ui, state);
            render_reference_libraries_tree(ui, state);
        });
}

fn render_project_header(ui: &mut Ui, state: &AppState, dirty: bool) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(state.workspace.project.display_name())
                    .strong()
                    .color(ui.visuals().text_color()),
            );
            if dirty {
                ui.label(
                    RichText::new("modified")
                        .size(10.0)
                        .color(ui.visuals().warn_fg_color),
                );
            }
        });

        let location = state
            .workspace
            .project
            .directory()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "Unsaved project".to_string());
        ui.label(
            RichText::new(location)
                .size(10.0)
                .color(ui.visuals().text_color().gamma_multiply(0.55)),
        );
    });
}

fn render_workspace_tree(ui: &mut Ui, state: &AppState, dirty: bool) {
    CollapsingHeader::new("Project")
        .default_open(true)
        .show(ui, |ui| {
            let project_file = state
                .workspace
                .project
                .path
                .as_ref()
                .and_then(|path| path.file_name())
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| "untitled.rspiceproj".to_string());
            render_info_row(ui, "File", &project_file);
            render_info_row(ui, "State", if dirty { "modified" } else { "clean" });
            render_info_row(
                ui,
                "Root",
                &format!(
                    "{}/{}",
                    state.workspace.project.root_library, state.workspace.project.top_cell
                ),
            );
            render_info_row(ui, "Active", &state.workspace.active_display_path());
            if let Some(technology) = state.workspace.project.technology.as_deref()
                && !technology.trim().is_empty()
            {
                render_info_row(ui, "Tech", technology);
            }
        });
}

fn render_open_views_tree(ui: &mut Ui, state: &mut AppState) {
    let open_views = state.workspace.open_views.clone();
    let active_view = state.workspace.active_view.clone();
    CollapsingHeader::new(format!("Open Views ({})", open_views.len()))
        .default_open(true)
        .show(ui, |ui| {
            if open_views.is_empty() {
                render_empty_row(ui, "none");
                return;
            }

            for open in open_views {
                render_open_view_row(ui, state, &active_view, open);
            }
        });
}

fn render_hierarchy_tree(ui: &mut Ui, state: &mut AppState) {
    let hierarchy = state.workspace.hierarchy_stack.clone();
    CollapsingHeader::new(format!("Hierarchy ({})", hierarchy.len()))
        .default_open(true)
        .show(ui, |ui| {
            if hierarchy.is_empty() {
                render_empty_row(ui, "none");
                return;
            }

            for (index, reference) in hierarchy.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.add_space((index as f32) * 12.0);
                    let label = if index == 0 {
                        format!("root  {}", reference.display_path())
                    } else {
                        format!("{}  {}", index, reference.display_path())
                    };
                    if ui
                        .selectable_label(index + 1 == hierarchy.len(), label)
                        .on_hover_text(reference.display_path())
                        .clicked()
                    {
                        state.focus_workspace_breadcrumb(index);
                    }
                });
            }
        });
}

fn render_design_libraries_tree(ui: &mut Ui, state: &mut AppState) {
    let root_library = state.workspace.project.root_library.clone();
    let libraries = collect_libraries(state, |library| library.name == root_library);

    CollapsingHeader::new(format!("Design Libraries ({})", libraries.len()))
        .default_open(true)
        .show(ui, |ui| {
            if libraries.is_empty() {
                render_empty_row(ui, "none");
                return;
            }

            for library in libraries {
                render_library_node(ui, state, library, true);
            }
        });
}

fn render_reference_libraries_tree(ui: &mut Ui, state: &mut AppState) {
    let root_library = state.workspace.project.root_library.clone();
    let libraries = collect_libraries(state, |library| {
        library.name != root_library && library.name != LibraryManager::PRIMITIVES_LIBRARY
    });
    let has_primitives = state
        .library_manager
        .get_library(LibraryManager::PRIMITIVES_LIBRARY)
        .is_some();

    let count = libraries.len() + usize::from(has_primitives);
    CollapsingHeader::new(format!("References ({})", count))
        .default_open(false)
        .show(ui, |ui| {
            if has_primitives {
                render_info_row(ui, "primitives", "component library");
            }

            if libraries.is_empty() && !has_primitives {
                render_empty_row(ui, "none");
                return;
            }

            for library in libraries {
                render_library_node(ui, state, library, false);
            }
        });
}

fn render_library_node(ui: &mut Ui, state: &mut AppState, library: LibraryNode, show_cells: bool) {
    let mut label = format!(
        "{}  {} cells, {} views",
        library.name, library.cell_count, library.view_count
    );
    if library.read_only {
        label.push_str("  read-only");
    }

    CollapsingHeader::new(label)
        .default_open(show_cells)
        .show(ui, |ui| {
            if let Some(path) = &library.path {
                render_info_row(ui, "Path", &path.display().to_string());
            }
            if !library.technology.trim().is_empty() {
                render_info_row(ui, "Tech", &library.technology);
            }

            if !show_cells {
                return;
            }

            for cell in library.cells {
                render_cell_node(ui, state, &library.name, cell);
            }
        });
}

fn render_cell_node(ui: &mut Ui, state: &mut AppState, library_name: &str, cell: CellNode) {
    let mut label = format!("{}  {} views", cell.name, cell.views.len());
    if !cell.category.trim().is_empty() {
        label.push_str(&format!("  {}", cell.category));
    }

    CollapsingHeader::new(label)
        .default_open(cell.name == state.workspace.project.top_cell)
        .show(ui, |ui| {
            for view in cell.views {
                let reference =
                    CellViewRef::new(library_name.to_string(), cell.name.clone(), view.name);
                render_view_row(
                    ui,
                    state,
                    reference,
                    view.view_type,
                    view.modified,
                    view.file_path,
                );
            }
        });
}

fn render_open_view_row(
    ui: &mut Ui,
    state: &mut AppState,
    active_view: &CellViewRef,
    open: OpenCellView,
) {
    let is_active = open.reference == *active_view;
    let mut label = format!(
        "{}  {}",
        view_kind_label(open.view_type),
        open.reference.display_path()
    );
    if open.dirty {
        label.push_str(" *");
    }

    if ui
        .selectable_label(is_active, label)
        .on_hover_text(open.reference.display_path())
        .clicked()
    {
        state.open_workspace_view(open.reference);
    }
}

fn render_view_row(
    ui: &mut Ui,
    state: &mut AppState,
    reference: CellViewRef,
    view_type: ViewType,
    modified: bool,
    file_path: Option<PathBuf>,
) {
    let is_active = state.workspace.active_view == reference;
    let mut label = format!("{}  {}", view_kind_label(view_type), reference.view);
    if modified {
        label.push_str(" *");
    }

    let hover = file_path
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| reference.display_path());
    if ui
        .selectable_label(is_active, label)
        .on_hover_text(hover)
        .clicked()
    {
        state.open_workspace_view(reference);
    }
}

fn render_info_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(label)
                .size(11.0)
                .color(ui.visuals().text_color().gamma_multiply(0.55)),
        );
        ui.label(RichText::new(value).size(11.0));
    });
}

fn render_empty_row(ui: &mut Ui, label: &str) {
    ui.label(
        RichText::new(label)
            .size(11.0)
            .color(ui.visuals().text_color().gamma_multiply(0.55)),
    );
}

fn collect_libraries(
    state: &AppState,
    predicate: impl Fn(&crate::state::Library) -> bool,
) -> Vec<LibraryNode> {
    state
        .library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| predicate(library))
        .map(|library| LibraryNode {
            name: library.name.clone(),
            path: library.path.clone(),
            technology: library.technology.clone(),
            read_only: library.read_only,
            cell_count: library.cell_count(),
            view_count: library.total_view_count(),
            cells: library
                .cells_sorted()
                .into_iter()
                .map(|cell| CellNode {
                    name: cell.name.clone(),
                    category: cell.category.clone(),
                    views: cell
                        .views_sorted()
                        .into_iter()
                        .map(view_node_from_view)
                        .collect(),
                })
                .collect(),
        })
        .collect()
}

fn view_node_from_view(view: &View) -> ViewNode {
    ViewNode {
        name: view.name.clone(),
        view_type: view.view_type,
        file_path: view.file_path.clone(),
        modified: view.modified,
    }
}

fn view_kind_label(view_type: ViewType) -> &'static str {
    match view_type {
        ViewType::Schematic => "SCH",
        ViewType::Symbol => "SYM",
        ViewType::Layout => "LAY",
        ViewType::Testbench => "TB",
        ViewType::Verilog => "V",
        ViewType::VerilogA => "VA",
        ViewType::Spice => "SP",
        ViewType::Document => "DOC",
        ViewType::Extracted => "EXT",
        ViewType::Abstract => "ABS",
        ViewType::Config => "CFG",
        ViewType::Custom => "USR",
    }
}
