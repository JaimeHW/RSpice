//! Main toolbar — schematic tools, view controls, history, checks, and the
//! run controls with corner selection on the right.

use egui::{Context, Frame, TopBottomPanel, Ui};

use crate::common::AppState;
use crate::shell::WorkspaceView;
use crate::state::Tool;
use crate::ui::icons::Icon;
use crate::ui::widgets::{Button, IconButton};
use crate::ui::tokens::Tokens;

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

                    let can_run =
                        !state.schematic.components.is_empty() && !state.simulation.is_running;
                    if Button::new("Run")
                        .icon(Icon::Run)
                        .hint("F5")
                        .accent()
                        .enabled(can_run)
                        .show(ui)
                        .clicked()
                    {
                        state.simulation.trigger_simulation = true;
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
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(13.0, TOOLBAR_HEIGHT), egui::Sense::hover());
    ui.painter().vline(
        rect.center().x,
        egui::Rangef::new(rect.center().y - 11.0, rect.center().y + 11.0),
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn schematic_tools(ui: &mut Ui, state: &mut AppState) {
    let on_schematic = state.shell.view == WorkspaceView::Schematic;
    let tool = state.schematic.tool.clone();

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

fn view_controls(ui: &mut Ui, state: &mut AppState) {
    if IconButton::new(Icon::ZoomIn)
        .tooltip("Zoom in")
        .show(ui)
        .clicked()
    {
        state.schematic.zoom = (state.schematic.zoom * 1.25).min(4.0);
    }
    if IconButton::new(Icon::ZoomOut)
        .tooltip("Zoom out")
        .show(ui)
        .clicked()
    {
        state.schematic.zoom = (state.schematic.zoom / 1.25).max(0.25);
    }
    if IconButton::new(Icon::ZoomFit)
        .tooltip("Zoom to fit (F)")
        .show(ui)
        .clicked()
    {
        state.schematic.needs_fit = true;
    }
    if IconButton::new(Icon::Grid)
        .on(state.shell.show_grid)
        .tooltip("Toggle grid")
        .show(ui)
        .clicked()
    {
        state.shell.show_grid = !state.shell.show_grid;
    }
}

fn history_controls(ui: &mut Ui, state: &mut AppState) {
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
    let t = Tokens::get(ui.ctx());
    let corners = crate::state::model_library::ProcessCorner::standard_corners();
    let current_label = corners
        .iter()
        .find(|corner| corner.name == state.shell.corner)
        .map(|corner| format!("{} · {:.0} °C", corner.name, corner.temperature))
        .unwrap_or_else(|| state.shell.corner.clone());

    egui::ComboBox::from_id_salt("volta.corner")
        .selected_text(
            egui::RichText::new(current_label)
                .font(crate::ui::theme::mono(
                    crate::ui::tokens::FS_0,
                    crate::ui::theme::FontWeight::Regular,
                ))
                .color(t.color.text),
        )
        .width(120.0)
        .show_ui(ui, |ui| {
            for corner in &corners {
                let label = format!("{} · {:.0} °C", corner.name, corner.temperature);
                if ui
                    .selectable_label(state.shell.corner == corner.name, label)
                    .clicked()
                {
                    state.shell.corner = corner.name.clone();
                }
            }
        });
}
