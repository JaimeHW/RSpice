//! Hierarchical design document surface.

use egui::Ui;

use crate::common::RSpiceApp;
use crate::state::ViewType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::design_system::empty_state;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    breadcrumb(ui, app);
    if app.state.active_view_read_only() {
        read_only_banner(ui, app);
    }
    match app.state.workspace.active_view_type() {
        ViewType::Schematic | ViewType::Testbench => {
            crate::schematic::view::render_schematic_view(
                ui,
                &mut app.state,
                app.symbol_library.as_ref(),
            );
        }
        ViewType::Symbol => crate::schematic::symbol_editor::show(ui, &mut app.state),
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => source_document(ui, app),
        view_type => unsupported_document(ui, app, view_type),
    }
}

fn breadcrumb(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for (index, reference) in app.state.workspace.hierarchy_stack.iter().enumerate() {
                    if index > 0 {
                        ui.label(egui::RichText::new("/").color(t.color.text_faint));
                    }
                    ui.label(
                        egui::RichText::new(reference.display_path())
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(if index + 1 == app.state.workspace.hierarchy_stack.len() {
                                t.color.text
                            } else {
                                t.color.text_dim
                            }),
                    );
                }
            });
        });
}

fn read_only_banner(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.warn.gamma_multiply(0.14))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} is read only. Create an editable copy before changing this document.",
                    app.state.workspace.active_display_path()
                ))
                .color(t.color.warn),
            );
        });
}

fn source_document(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let reference = app.state.workspace.active_view.clone();
    let contents = app
        .state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .and_then(|view| view.metadata.get("source"))
        .cloned()
        .unwrap_or_default();
    if contents.is_empty() {
        empty_state(
            ui,
            super::super::design_system::WorkbenchIcon::Netlist,
            "No source text stored",
            "Import or compile this behavioral view from the Models workspace.",
        );
        return;
    }
    let mut display = contents;
    egui::Frame::new().fill(t.color.canvas_bg).show(ui, |ui| {
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::multiline(&mut display)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .interactive(false),
        );
    });
}

fn unsupported_document(ui: &mut Ui, app: &RSpiceApp, view_type: ViewType) {
    empty_state(
        ui,
        super::super::design_system::WorkbenchIcon::File,
        &format!("{} view", view_type.display_name()),
        &format!(
            "{} is registered in the project and available for downstream integrations.",
            app.state.workspace.active_display_path()
        ),
    );
}
