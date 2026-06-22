//! Contextual side panels.
//!
//! The contextual side panels change content with the active
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
/// Premium schematic navigator default width.
const SCHEMATIC_LEFT_WIDTH: f32 = 318.0;
/// Right panel default width.
const RIGHT_WIDTH: f32 = 304.0;
/// Resize bounds — narrow enough to tuck away, wide enough for ultrawides.
const PANEL_MIN: f32 = 200.0;
const PANEL_MAX: f32 = 520.0;
const RESPONSIVE_PANEL_BREAKPOINT: f32 = 760.0;

fn side_panels_visible_for_width(
    viewport_width: f32,
    view: WorkspaceView,
    panels_hidden: bool,
    active_view_is_symbol: bool,
) -> bool {
    if active_view_is_symbol && view == WorkspaceView::Schematic {
        return false;
    }
    !panels_hidden && view.has_side_panels() && viewport_width >= RESPONSIVE_PANEL_BREAKPOINT
}

fn left_default_width(view: WorkspaceView) -> f32 {
    match view {
        WorkspaceView::Schematic => SCHEMATIC_LEFT_WIDTH,
        _ => LEFT_WIDTH,
    }
}

/// Render both contextual side panels for the active view.
pub fn show(
    ctx: &Context,
    state: &mut AppState,
    symbol_library: Option<&crate::schematic::symbols::SymbolLibrary>,
) {
    let active_view_is_symbol =
        state.workspace.active_view_type() == crate::state::ViewType::Symbol;
    if !side_panels_visible_for_width(
        ctx.screen_rect().width(),
        state.shell.view,
        state.shell.panels_hidden,
        active_view_is_symbol,
    ) {
        return;
    }
    let t = Tokens::get(ctx);
    let c = t.color;

    if state.shell.view.has_left_panel() {
        SidePanel::left("volta.left")
            .default_width(left_default_width(state.shell.view))
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
                match state.shell.view {
                    // The schematic rail manages its own vertical layout:
                    // fixed chrome, a filling tree scroll region, and the
                    // place strip pinned to the bottom edge.
                    WorkspaceView::Schematic => schematic::left(ui, state, symbol_library),
                    _ => {
                        ScrollArea::vertical()
                            .id_salt("volta.left.scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| match state.shell.view {
                                WorkspaceView::Simulate => simulate::left(ui, state),
                                WorkspaceView::Results => results::left(ui, state),
                                _ => {}
                            });
                    }
                }
            });
    }

    if state.shell.view.has_right_panel() {
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
                        WorkspaceView::Netlist => {
                            crate::shell::views::netlist::right_panel(ui, state)
                        }
                        _ => {}
                    });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schematic_left_panel_defaults_to_premium_rail_width() {
        assert_eq!(left_default_width(WorkspaceView::Schematic), 318.0);
        assert_eq!(left_default_width(WorkspaceView::Simulate), LEFT_WIDTH);
        assert_eq!(left_default_width(WorkspaceView::Results), LEFT_WIDTH);
    }

    #[test]
    fn phone_width_hides_context_side_panels() {
        assert!(!side_panels_visible_for_width(
            390.0,
            WorkspaceView::Schematic,
            false,
            false
        ));
    }

    #[test]
    fn desktop_width_keeps_context_side_panels_visible() {
        assert!(side_panels_visible_for_width(
            1280.0,
            WorkspaceView::Schematic,
            false,
            false
        ));
    }
}
