//! Canonical task-owner surfaces.

mod automation;
mod design;
mod models;
mod netlist;
mod project;
mod results;
mod simulate;
mod verify;
mod veriloga;

use egui::Ui;

use crate::common::RSpiceApp;

use super::state::Workspace;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.workbench.current_route().surface_id() == super::SurfaceId::VisualizationStudio {
        super::visualization_studio::show(ui, app);
        return;
    }
    match app.state.workbench.workspace {
        Workspace::Project => project::show(ui, app),
        Workspace::Design => design::show(ui, app),
        Workspace::Simulate => simulate::show(ui, app),
        Workspace::Results => results::show(ui, app),
        Workspace::Verify => verify::show(ui, app),
        Workspace::Models => models::show(ui, app),
        Workspace::Netlist => {
            netlist::prepare_workspace(app);
            match app.state.ui.code_workspace.page {
                super::code_workspace::CodeWorkspacePage::Netlist => {
                    netlist::show_prepared(ui, app)
                }
                super::code_workspace::CodeWorkspacePage::VerilogA => veriloga::show(ui, app),
                super::code_workspace::CodeWorkspacePage::Automation => automation::show(ui, app),
            }
        }
    }
}
