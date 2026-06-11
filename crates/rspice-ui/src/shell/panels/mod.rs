//! Contextual side panels.
//!
//! The left (264 px) and right (304 px) panels change content with the active
//! workspace view: schematic editing gets hierarchy/components + inspector,
//! simulation gets run history + analysis detail, results get the signal
//! browser + cursors/measurements. Library and netlist views are full-bleed.

use egui::{Context, Frame, ScrollArea, SidePanel};

use crate::common::AppState;
use crate::shell::WorkspaceView;
use crate::ui::tokens::Tokens;

pub mod results;
pub mod schematic;
pub mod simulate;
pub(in crate::shell) mod simulate_forms;

/// Left panel default width.
const LEFT_WIDTH: f32 = 264.0;
/// Right panel default width.
const RIGHT_WIDTH: f32 = 304.0;
/// Resize bounds — narrow enough to tuck away, wide enough for ultrawides.
const PANEL_MIN: f32 = 200.0;
const PANEL_MAX: f32 = 520.0;

/// Render both contextual side panels for the active view.
pub fn show(
    ctx: &Context,
    state: &mut AppState,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    if state.shell.panels_hidden || !state.shell.view.has_side_panels() {
        return;
    }
    let t = Tokens::get(ctx);
    let c = t.color;

    SidePanel::left("volta.left")
        .default_width(LEFT_WIDTH)
        .width_range(PANEL_MIN..=PANEL_MAX)
        .resizable(true)
        .frame(Frame::none().fill(c.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().vline(
                rect.right() - 0.5,
                rect.y_range(),
                egui::Stroke::new(1.0, c.border),
            );
            ScrollArea::vertical()
                .id_salt("volta.left.scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| match state.shell.view {
                    WorkspaceView::Schematic => schematic::left(ui, state, symbol_library),
                    WorkspaceView::Simulate => simulate::left(ui, state),
                    WorkspaceView::Results => results::left(ui, state),
                    _ => {}
                });
        });

    SidePanel::right("volta.right")
        .default_width(RIGHT_WIDTH)
        .width_range(PANEL_MIN..=PANEL_MAX)
        .resizable(true)
        .frame(Frame::none().fill(c.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().vline(
                rect.left() + 0.5,
                rect.y_range(),
                egui::Stroke::new(1.0, c.border),
            );
            ScrollArea::vertical()
                .id_salt("volta.right.scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| match state.shell.view {
                    WorkspaceView::Schematic => schematic::right(ui, state),
                    WorkspaceView::Simulate => simulate::right(ui, state),
                    WorkspaceView::Results => results::right(ui, state),
                    _ => {}
                });
        });
}
