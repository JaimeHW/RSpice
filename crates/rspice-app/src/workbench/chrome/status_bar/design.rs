//! The status bar's Design segments: the pointer's coordinates in the open
//! view, what is selected, and the canvas zoom behind the percentage chip.

use crate::workbench::RSpiceApp;
use crate::workbench::state::Workspace;

pub(super) fn schematic_cursor_summary(
    state: &crate::workbench::app_state::AppState,
    grid_x: f64,
    grid_y: f64,
) -> String {
    let grid = f64::from(state.model.schematic.document().grid_size.max(1));
    let sheet = state.model.active_drawing_sheet();
    sheet.cursor_status(grid_x * grid, grid_y * grid)
}

fn empty_schematic_cursor_summary(state: &crate::workbench::app_state::AppState) -> String {
    let unit = state.model.active_drawing_sheet().format.display_unit;
    format!("x — · y — {}", unit.suffix())
}

/// The coordinate segment in Design: where the pointer is in the open view's
/// own units, or the view's empty form while the pointer is off its canvas.
pub(in crate::workbench) fn design_engineering_context(
    state: &crate::workbench::app_state::AppState,
) -> String {
    let view_type = state.model.workspace.content.active_view_type();
    state.design.canvas_hover.map_or_else(
        || match view_type {
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench => {
                empty_schematic_cursor_summary(state)
            }
            crate::state::ViewType::Layout => "x — · y — layout".to_owned(),
            crate::state::ViewType::Symbol => {
                format!(
                    "x — · y — · grid {}",
                    state.model.schematic.document().grid_size.max(1)
                )
            }
            _ => format!(
                "revision {}",
                state.model.workspace.content.project.revision().get()
            ),
        },
        |(x, y)| match view_type {
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench => {
                schematic_cursor_summary(state, x, y)
            }
            crate::state::ViewType::Layout => {
                // The open layout's own database unit is the one its
                // coordinates count in.
                let content = &state.model.workspace.content;
                content
                    .physical_layout_document(&content.active_view)
                    .map(|document| document.technology().database_unit())
                    .or(state.model.pdk_config.layout_database_unit)
                    .map_or_else(
                        || format!("x {x:.0} · y {y:.0} DBU unavailable"),
                        |dbu| {
                            let policy = state.model.preferences.quantity_presentation_policy();
                            format!(
                                "x {} · y {}",
                                policy.format_layout_coordinate(x, dbu),
                                policy.format_layout_coordinate(y, dbu)
                            )
                        },
                    )
            }
            crate::state::ViewType::Symbol => format!(
                "x {x:.0} · y {y:.0} · grid {}",
                state.model.schematic.document().grid_size.max(1)
            ),
            _ => format!(
                "revision {}",
                state.model.workspace.content.project.revision().get()
            ),
        },
    )
}

pub(in crate::workbench) fn selection_summary(app: &RSpiceApp) -> String {
    if !app.state.platform.project_lifecycle.is_open() {
        return "No engineering object selected".to_owned();
    }
    if app.state.workbench.workspace == Workspace::Results {
        return app
            .state
            .model
            .simulation
            .view
            .active_run_idx
            .and_then(|index| app.state.model.simulation.retained.runs.get(index))
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
    if app.state.workbench.workspace != Workspace::Design {
        return app.state.workbench.workspace.label().to_owned();
    }
    let selection = &app.state.model.schematic.session.editor.selection;
    if let Some(id) = selection.single_component() {
        app.state
            .model
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

/// The canvas zoom behind the percentage chip. The Results workspace never
/// reaches here — its chip reports plot magnification instead.
pub(in crate::workbench) fn zoom_factor(app: &RSpiceApp) -> f64 {
    if app.state.workbench.workspace == Workspace::Design
        && app.state.model.workspace.content.active_view_type() == crate::state::ViewType::Symbol
    {
        f64::from(app.state.design.symbol.editor.zoom)
    } else {
        app.state.model.schematic.session.editor.zoom
    }
}
