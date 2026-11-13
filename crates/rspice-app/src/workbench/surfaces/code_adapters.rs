//! The Verilog-A editor drawn as the Design workspace's document, from the
//! application: the code feature's page draws in its own context, and what
//! the reader pressed on its interface strip is carried out on the session.
//! It takes the application whole, so it stays in the shell when the code
//! feature moves into a crate of its own.

use egui::Ui;

use crate::workbench::RSpiceApp;
use crate::workbench::surfaces::veriloga::{InterfaceAction, prepare, show, source_menu_open};

/// A Verilog-A cell view, as the Design workspace's document.
///
/// The design page draws the view through this, which lends the page the
/// code feature's context: the page prepares, putting its editor in front,
/// and then draws in a context holding the source menu's answers while
/// [`source_menu_open`] says the menu is open or opening, and whether an
/// application-modal dialog is open. What the reader pressed on the
/// interface strip changes the design's own state, past the code feature,
/// so it is carried out here once the page has drawn. This adapter goes
/// when design stops calling it and the shell draws the page by name.
pub fn show_in_design(ui: &mut Ui, app: &mut RSpiceApp) {
    let application_modal = app.state.application_modal_open();
    prepare(&mut crate::workbench::features::cx(app));
    let ctx = ui.ctx().clone();
    let pressed = show(
        ui,
        &mut crate::workbench::features::cx_with_menu_answers(app, |app| {
            source_menu_open(&ctx, &app.state.workbench)
        })
        .with_application_modal(application_modal),
    );
    match pressed {
        None => {}
        Some(InterfaceAction::UpdateSymbol) => app.state.update_active_veriloga_symbol(),
        Some(InterfaceAction::Place) => match app.state.place_offered_veriloga_device() {
            // The canvas takes the keyboard, as the shelf leaves it, so R, M
            // and Esc work before the pointer reaches the sheet. No status
            // line: the armed cursor is the answer, and a sentence about it
            // outlives the arm.
            Ok(()) => rspice_schematic_editor::view::canvas::request_focus(ui.ctx()),
            Err(error) => app.state.prompt(error),
        },
    }
}
