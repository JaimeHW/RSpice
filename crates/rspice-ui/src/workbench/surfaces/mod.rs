//! Canonical task-owner surfaces.

mod design;
mod models;
mod netlist;
mod project;
mod results;
mod simulate;
mod verify;

use egui::Ui;

use crate::common::RSpiceApp;

use super::state::Workspace;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    match app.state.workbench.workspace {
        Workspace::Project => project::show(ui, app),
        Workspace::Design => design::show(ui, app),
        Workspace::Simulate => simulate::show(ui, app),
        Workspace::Results => results::show(ui, app),
        Workspace::Verify => verify::show(ui, app),
        Workspace::Models => models::show(ui, app),
        Workspace::Netlist => netlist::show(ui, app),
    }
}
