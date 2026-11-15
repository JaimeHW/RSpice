//! The status bar's Design segments: the pointer's coordinates in the open
//! view, what is selected, and the canvas zoom behind the percentage chip.

use crate::workbench::app::actions::workspace::symbol_document::SymbolUiState;
use crate::workbench::app::in_place_session;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::state::{WorkbenchSession, Workspace};
use rspice_schematic_editor::symbol_editor::session::{label_name, shape_kind};

pub fn schematic_cursor_summary(model: &Model, grid_x: f64, grid_y: f64) -> String {
    let grid = f64::from(model.schematic.document().grid_size.max(1));
    let sheet = model.active_drawing_sheet();
    sheet.cursor_status(grid_x * grid, grid_y * grid)
}

fn empty_schematic_cursor_summary(model: &Model) -> String {
    let unit = model.active_drawing_sheet().format.display_unit;
    format!("x — · y — {}", unit.suffix())
}

/// The coordinate segment in Design: where the pointer is in the open view's
/// own units, or the view's empty form while the pointer is off its canvas
/// (`canvas_hover`, the point the Design canvas last reported). A symbol
/// reads its own grid, the one `symbol` snaps to.
pub fn design_engineering_context(
    canvas_hover: Option<(f64, f64)>,
    model: &Model,
    symbol: &SymbolUiState,
) -> String {
    let view_type = model.workspace.content.active_view_type();
    let symbol_grid = symbol.editor.grid_spacing.model_step();
    canvas_hover.map_or_else(
        || match view_type {
            Some(crate::state::ViewType::Schematic | crate::state::ViewType::Testbench) => {
                empty_schematic_cursor_summary(model)
            }
            Some(crate::state::ViewType::Layout) => "x — · y — layout".to_owned(),
            Some(crate::state::ViewType::Symbol) => format!("x — · y — · grid {symbol_grid}"),
            _ => String::new(),
        },
        |(x, y)| match view_type {
            Some(crate::state::ViewType::Schematic | crate::state::ViewType::Testbench) => {
                schematic_cursor_summary(model, x, y)
            }
            Some(crate::state::ViewType::Layout) => {
                // The open layout's own database unit is the one its
                // coordinates count in.
                let content = &model.workspace.content;
                content
                    .physical_layout_document(&content.active_view)
                    .map(|document| document.technology().database_unit())
                    .or(model.pdk_config.layout_database_unit)
                    .map_or_else(
                        || format!("x {x:.0} · y {y:.0} DBU unavailable"),
                        |dbu| {
                            let policy = model.preferences.quantity_presentation_policy();
                            format!(
                                "x {} · y {}",
                                policy.format_layout_coordinate(x, dbu),
                                policy.format_layout_coordinate(y, dbu)
                            )
                        },
                    )
            }
            Some(crate::state::ViewType::Symbol) => {
                format!("x {x:.0} · y {y:.0} · grid {symbol_grid}")
            }
            _ => String::new(),
        },
    )
}

pub fn selection_summary(
    model: &Model,
    wb: &WorkbenchSession,
    platform: &PlatformState,
    symbol: &SymbolUiState,
) -> String {
    if !platform.project_lifecycle.is_open() {
        return "No engineering object selected".to_owned();
    }
    if wb.workspace == Workspace::Results {
        return model
            .simulation
            .view
            .active_run_idx
            .and_then(|index| model.simulation.retained.runs.get(index))
            .map_or_else(
                || "No result dataset selected".to_owned(),
                |run| {
                    let analysis_count = run.analyses.len();
                    format!(
                        "Run {} · {analysis_count} {}",
                        run.id,
                        if analysis_count == 1 {
                            "analysis"
                        } else {
                            "analyses"
                        }
                    )
                },
            );
    }
    if wb.workspace != Workspace::Design {
        return wb.workspace.label().to_owned();
    }
    if model.workspace.content.active_view_type() == Some(crate::state::ViewType::Symbol) {
        return symbol_selection_summary(symbol, model);
    }
    let selection = &model.schematic.session.editor.selection;
    if let Some(id) = selection.single_component() {
        model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .map(|component| format!("{} · {}", component.name, component.value))
            .unwrap_or_else(|| "Selected object unavailable".to_owned())
    } else if selection.count() > 0 {
        format!("{} items selected", selection.count())
    } else {
        "No object selected".to_owned()
    }
}

/// What the symbol editor has selected: one pin by its name ("PIN1"), one
/// shape by its kind ("Rectangle"), one label by its name ("Value"), or how
/// many items.
fn symbol_selection_summary(symbol: &SymbolUiState, model: &Model) -> String {
    let selection = symbol.editor.effective_selection();
    if selection.len() > 1 {
        return format!("{} items selected", selection.len());
    }
    if let Some(name) = selection.pins.first() {
        return name.clone();
    }
    if let Some(index) = selection.shapes.first() {
        return model
            .load_active_symbol_document()
            .ok()
            .and_then(|document| document.body.get(*index).map(shape_kind))
            .unwrap_or("Selected object unavailable")
            .to_owned();
    }
    selection.attributes.first().map_or_else(
        || "No object selected".to_owned(),
        |kind| label_name(*kind).to_owned(),
    )
}

/// The canvas zoom behind the percentage chip: a symbol edited in place
/// reads the in-place view, which frames its parent sheet. The Results
/// workspace never reaches here — its chip reports plot magnification
/// instead.
pub fn zoom_factor(symbol: &SymbolUiState, model: &Model, wb: &WorkbenchSession) -> f64 {
    let symbol_in_front = wb.workspace == Workspace::Design
        && model.workspace.content.active_view_type() == Some(crate::state::ViewType::Symbol);
    if symbol_in_front && let Some(session) = in_place_session(&symbol.in_place, model) {
        session.zoom
    } else if symbol_in_front {
        f64::from(symbol.editor.zoom)
    } else {
        model.schematic.session.editor.zoom
    }
}

/// What the armed symbol drawing tool takes next, while an editable symbol is
/// the document in front: "Line: click the next point, Enter to finish".
pub fn symbol_tool_hint(
    symbol: &SymbolUiState,
    model: &Model,
    wb: &WorkbenchSession,
) -> Option<&'static str> {
    (wb.workspace == Workspace::Design
        && model.workspace.content.active_view_type() == Some(crate::state::ViewType::Symbol)
        && !model.active_view_read_only())
    .then(|| symbol.editor.next_step())
    .flatten()
}
