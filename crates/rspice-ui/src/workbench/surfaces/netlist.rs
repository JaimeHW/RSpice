//! Text-first netlist and automation-pipeline surface.

use egui::Ui;

use crate::common::RSpiceApp;
use crate::io::NetlistFormat;
use crate::ui::tokens::Tokens;

use super::super::commands::Command;
use super::super::design_system::heading;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ensure_buffer(app);
    super::super::netlist_document::prepare(&mut app.state);
    header(ui, app);
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.canvas_bg).show(ui, |ui| {
        super::super::netlist_document::show_editor(ui, &mut app.state);
    });
}

fn ensure_buffer(app: &mut RSpiceApp) {
    if let Some(source) = &app.state.workspace.netlist_source {
        if app.state.simulation.netlist_content != *source
            && !app.state.workspace.netlist_source_dirty
        {
            app.state.simulation.netlist_content = source.clone();
        }
    } else if app.state.simulation.netlist_content.trim().is_empty()
        && !app.state.schematic.components.is_empty()
    {
        crate::common::menu_bar::action_view_netlist(&mut app.state);
    }
}

fn header(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(14, 9))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    heading(
                        ui,
                        "Automation-pipeline owner",
                        if app.state.workspace.netlist_source.is_some() {
                            "Manual SPICE source"
                        } else {
                            "Generated netlist artifact"
                        },
                        &format!(
                            "{} · {} lines",
                            app.state.workspace.active_display_path(),
                            app.state.simulation.netlist_content.lines().count()
                        ),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let running = app.state.simulation.is_running;
                    if ui
                        .add_enabled(
                            if running {
                                true
                            } else {
                                app.state.manual_deck_run_block_reason().is_none()
                            },
                            egui::Button::new(if running { "Stop" } else { "Run deck" }),
                        )
                        .clicked()
                    {
                        if running {
                            Command::StopSimulation.execute(app);
                        } else {
                            app.state.request_netlist_manual_deck_run();
                        }
                    }
                    if ui.button("Copy").clicked() {
                        ui.ctx()
                            .copy_text(app.state.simulation.netlist_content.clone());
                    }
                    ui.menu_button("Export", |ui| {
                        for (label, format) in [
                            ("SPICE…", NetlistFormat::Spice),
                            ("Spectre…", NetlistFormat::Spectre),
                            ("HSPICE…", NetlistFormat::Hspice),
                            ("Xyce…", NetlistFormat::Xyce),
                        ] {
                            if ui.button(label).clicked() {
                                Command::ExportNetlist(format).execute(app);
                                ui.close();
                            }
                        }
                    });
                    if app.state.workspace.netlist_source.is_some() {
                        if ui.button("Regenerate from schematic").clicked() {
                            app.state.workspace.netlist_source = None;
                            app.state.workspace.netlist_source_path = None;
                            app.state.workspace.netlist_source_dirty = false;
                            crate::common::menu_bar::action_view_netlist(&mut app.state);
                        }
                    } else if ui.button("Make editable copy").clicked() {
                        app.state.workspace.netlist_source =
                            Some(app.state.simulation.netlist_content.clone());
                        app.state.workspace.netlist_source_path = None;
                        app.state.workspace.netlist_source_dirty = false;
                    }
                });
            });
        });
}
