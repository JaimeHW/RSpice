//! Center workspace views.

use egui::{Context, Frame};

use crate::common::RSpiceApp;
use crate::shell::WorkspaceView;
use crate::ui::tokens::Tokens;

pub mod library;
pub mod netlist;
pub mod results;
pub mod schematic;
pub mod simulate;
pub mod symbol;

/// Render the central panel for the active workspace view.
pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    let t = Tokens::get(ctx);
    let c = t.color;

    egui::CentralPanel::default()
        .frame(Frame::NONE.fill(c.bg_app))
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            match app.state.shell.view {
                WorkspaceView::Library => library::show(ui, &mut app.state),
                WorkspaceView::Schematic => {
                    let symbol_library = app.symbol_library.as_ref();
                    if app.state.workspace.active_view_type() == crate::state::ViewType::Symbol {
                        symbol::show(ui, &mut app.state);
                    } else {
                        schematic::show(ui, &mut app.state, symbol_library);
                    }
                }
                WorkspaceView::Netlist => netlist::show(ui, &mut app.state),
                WorkspaceView::Simulate => simulate::show(ui, &mut app.state),
                WorkspaceView::Results => results::show(ui, app),
            }
        });
}
