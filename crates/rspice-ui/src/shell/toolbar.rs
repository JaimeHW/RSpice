//! Main toolbar — schematic tools, view controls, history, checks, and the
//! run controls with corner selection on the right.

use egui::{Context, Frame, TopBottomPanel, Ui};

use crate::common::AppState;
use crate::shell::{SymbolTool, WorkspaceView};
use crate::state::Tool;
use crate::ui::icons::Icon;
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Button, IconButton};

/// Toolbar height.
pub const TOOLBAR_HEIGHT: f32 = 42.0;

/// Render the toolbar panel.
pub fn show(ctx: &Context, state: &mut AppState) {
    let t = Tokens::get(ctx);
    let c = t.color;

    TopBottomPanel::top("volta.toolbar")
        .exact_height(TOOLBAR_HEIGHT)
        .frame(Frame::none().fill(c.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let panel_rect = ui.max_rect();
            ui.painter().hline(
                panel_rect.x_range(),
                panel_rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );

            ui.horizontal_centered(|ui| {
                ui.add_space(8.0);
                ui.spacing_mut().item_spacing.x = 2.0;

                schematic_tools(ui, state);
                toolbar_separator(ui);
                view_controls(ui, state);
                toolbar_separator(ui);
                history_controls(ui, state);
                toolbar_separator(ui);
                check_controls(ui, state);

                // Right cluster: corner select, stop, run.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    ui.spacing_mut().item_spacing.x = 8.0;

                    let can_run = state.can_run_simulation();
                    if Button::new("Run")
                        .icon(Icon::Run)
                        .hint("F5")
                        .accent()
                        .enabled(can_run)
                        .show(ui)
                        .clicked()
                    {
                        state.request_run_set_simulation();
                        state.shell.view = WorkspaceView::Simulate;
                    }
                    if Button::new("Stop")
                        .icon(Icon::Stop)
                        .enabled(state.simulation.is_running)
                        .show(ui)
                        .clicked()
                    {
                        state.simulation.trigger_abort = true;
                    }
                    corner_select(ui, state);
                });
            });
        });
}

fn toolbar_separator(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(egui::vec2(13.0, TOOLBAR_HEIGHT), egui::Sense::hover());
    ui.painter().vline(
        rect.center().x,
        egui::Rangef::new(rect.center().y - 11.0, rect.center().y + 11.0),
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn schematic_tools(ui: &mut Ui, state: &mut AppState) {
    let on_schematic = state.shell.view == WorkspaceView::Schematic;
    if on_schematic && state.workspace.active_view_type() == crate::state::ViewType::Symbol {
        symbol_tools(ui, state);
        return;
    }
    let tool = state.schematic.tool;

    let select_on = on_schematic && tool.is_select();
    if IconButton::new(Icon::Select)
        .on(select_on)
        .enabled(on_schematic)
        .tooltip("Select (Esc)")
        .show(ui)
        .clicked()
    {
        state.schematic.tool = Tool::Select;
    }
    let wire_on = on_schematic && tool.is_wire();
    if IconButton::new(Icon::Wire)
        .on(wire_on)
        .enabled(on_schematic)
        .tooltip("Draw wire (W)")
        .show(ui)
        .clicked()
    {
        state.schematic.tool = Tool::Wire;
    }
    let label_on = on_schematic && matches!(tool, Tool::Label);
    if IconButton::new(Icon::NetLabel)
        .on(label_on)
        .enabled(on_schematic)
        .tooltip("Net label (N)")
        .show(ui)
        .clicked()
    {
        state.schematic.tool = Tool::Label;
    }
    let probe_on = on_schematic && matches!(tool, Tool::Probe);
    if IconButton::new(Icon::Probe)
        .on(probe_on)
        .enabled(on_schematic)
        .tooltip("Probe net (P)")
        .show(ui)
        .clicked()
    {
        state.schematic.tool = Tool::Probe;
    }
}

fn symbol_tools(ui: &mut Ui, state: &mut AppState) {
    let tool = state.shell.symbol.tool;
    if IconButton::new(Icon::Select)
        .on(tool == SymbolTool::Select)
        .tooltip("Select symbol geometry (S)")
        .show(ui)
        .clicked()
    {
        state.shell.symbol.tool = SymbolTool::Select;
    }
    if IconButton::new(Icon::Pin)
        .on(tool == SymbolTool::PlacePin)
        .tooltip("Place next pin (P)")
        .show(ui)
        .clicked()
    {
        state.shell.symbol.tool = SymbolTool::PlacePin;
        state.shell.symbol.clear_selection();
    }
    if IconButton::new(Icon::Wire)
        .on(tool == SymbolTool::Polyline)
        .tooltip("Draw symbol polyline (W)")
        .show(ui)
        .clicked()
    {
        state.shell.symbol.tool = SymbolTool::Polyline;
        state.shell.symbol.pending_polyline.clear();
    }
    if IconButton::new(Icon::SymbolCircle)
        .on(tool == SymbolTool::Circle)
        .tooltip("Circle body tool (C)")
        .show(ui)
        .clicked()
    {
        state.shell.symbol.tool = SymbolTool::Circle;
        state.shell.symbol.shape_start = None;
    }
    if IconButton::new(Icon::SymbolArc)
        .on(tool == SymbolTool::Arc)
        .tooltip("Arc body tool (A)")
        .show(ui)
        .clicked()
    {
        state.shell.symbol.tool = SymbolTool::Arc;
        state.shell.symbol.shape_start = None;
    }
    if IconButton::new(Icon::SymbolArrow)
        .on(tool == SymbolTool::Arrow)
        .tooltip("Arrow marker tool (D)")
        .show(ui)
        .clicked()
    {
        state.shell.symbol.tool = SymbolTool::Arrow;
    }
    if IconButton::new(Icon::SymbolDot)
        .on(tool == SymbolTool::Dot)
        .tooltip("Dot marker tool (O)")
        .show(ui)
        .clicked()
    {
        state.shell.symbol.tool = SymbolTool::Dot;
    }
}

fn view_controls(ui: &mut Ui, state: &mut AppState) {
    if IconButton::new(Icon::ZoomIn)
        .tooltip("Zoom in")
        .show(ui)
        .clicked()
    {
        if state.workspace.active_view_type() == crate::state::ViewType::Symbol {
            state.shell.symbol.zoom = (state.shell.symbol.zoom * 1.25).min(18.0);
        } else {
            state.schematic.zoom = (state.schematic.zoom * 1.25).min(4.0);
        }
    }
    if IconButton::new(Icon::ZoomOut)
        .tooltip("Zoom out")
        .show(ui)
        .clicked()
    {
        if state.workspace.active_view_type() == crate::state::ViewType::Symbol {
            state.shell.symbol.zoom = (state.shell.symbol.zoom / 1.25).max(1.0);
        } else {
            state.schematic.zoom = (state.schematic.zoom / 1.25).max(0.25);
        }
    }
    if IconButton::new(Icon::ZoomFit)
        .tooltip("Zoom to fit (F)")
        .show(ui)
        .clicked()
    {
        if state.workspace.active_view_type() == crate::state::ViewType::Symbol {
            state.shell.symbol.needs_fit = true;
        } else {
            state.schematic.needs_fit = true;
        }
    }
    // The glyph mirrors the active style; clicking cycles dots → lines → off.
    let grid = state.shell.grid;
    let icon = match grid {
        crate::shell::GridStyle::Dots => Icon::GridDots,
        _ => Icon::Grid,
    };
    let tooltip = match grid {
        crate::shell::GridStyle::Dots => "Grid: dots — click for lines",
        crate::shell::GridStyle::Lines => "Grid: lines — click to hide",
        crate::shell::GridStyle::Off => "Grid: off — click for dots",
    };
    if IconButton::new(icon)
        .on(grid.visible())
        .tooltip(tooltip)
        .show(ui)
        .clicked()
    {
        state.shell.grid = grid.cycled();
    }
}

fn history_controls(ui: &mut Ui, state: &mut AppState) {
    if state.workspace.active_view_type() == crate::state::ViewType::Symbol {
        if IconButton::new(Icon::Undo)
            .enabled(state.can_undo_active_symbol_document())
            .tooltip("Undo symbol edit (Ctrl+Z)")
            .show(ui)
            .clicked()
        {
            let _ = state.undo_active_symbol_document();
        }
        if IconButton::new(Icon::Redo)
            .enabled(state.can_redo_active_symbol_document())
            .tooltip("Redo symbol edit (Ctrl+Y)")
            .show(ui)
            .clicked()
        {
            let _ = state.redo_active_symbol_document();
        }
        return;
    }
    if IconButton::new(Icon::Undo)
        .enabled(state.schematic.can_undo())
        .tooltip("Undo (Ctrl+Z)")
        .show(ui)
        .clicked()
    {
        state.schematic.undo();
    }
    if IconButton::new(Icon::Redo)
        .enabled(state.schematic.can_redo())
        .tooltip("Redo (Ctrl+Shift+Z)")
        .show(ui)
        .clicked()
    {
        state.schematic.redo();
    }
}

fn check_controls(ui: &mut Ui, state: &mut AppState) {
    if state.workspace.active_view_type() == crate::state::ViewType::Symbol {
        if IconButton::new(Icon::Check)
            .tooltip("Run symbol pin checks")
            .show(ui)
            .clicked()
        {
            state.run_active_symbol_pin_checks();
        }
        IconButton::new(Icon::File)
            .enabled(false)
            .tooltip("Open the schematic view to generate a netlist")
            .show(ui);
        return;
    }
    if IconButton::new(Icon::Check)
        .tooltip("Run design checks")
        .show(ui)
        .clicked()
    {
        crate::common::menu_bar::run_design_rule_check(state);
    }
    if IconButton::new(Icon::File)
        .tooltip("Generate netlist")
        .show(ui)
        .clicked()
    {
        crate::common::menu_bar::action_view_netlist(state);
        state.shell.view = WorkspaceView::Netlist;
    }
}

/// Process corner + temperature selector. The selected corner also feeds the
/// model library's corner selection.
fn corner_select(ui: &mut Ui, state: &mut AppState) {
    // The standard corner table is immutable; the labels bake once.
    static CORNERS: std::sync::LazyLock<Vec<crate::state::model_library::ProcessCorner>> =
        std::sync::LazyLock::new(crate::state::model_library::ProcessCorner::standard_corners);
    static LABELS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
        CORNERS
            .iter()
            .map(|corner| format!("{} · {:.0} °C", corner.name, corner.temperature))
            .collect()
    });

    let corners = &*CORNERS;
    let current_label = corners
        .iter()
        .position(|corner| corner.name == state.shell.corner)
        .map(|index| LABELS[index].clone())
        .unwrap_or_else(|| state.shell.corner.clone());

    if let Some(index) =
        crate::ui::widgets::select(ui, "volta.corner", &current_label, &LABELS, 120.0)
    {
        state.shell.corner = corners[index].name.clone();
    }
}
