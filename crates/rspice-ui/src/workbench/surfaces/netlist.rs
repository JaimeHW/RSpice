//! Text-first netlist and automation-pipeline surface.

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::io::NetlistFormat;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{heading, property_row, status_dot};

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    ensure_buffer(app);
    header(ui, app);
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.canvas_bg).show(ui, |ui| {
        let editor_height = (ui.available_height() - 112.0).max(180.0);
        let manual = app.state.workspace.netlist_source.is_some();
        let response = ui.add_sized(
            [ui.available_width(), editor_height],
            egui::TextEdit::multiline(&mut app.state.simulation.netlist_content)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .interactive(manual)
                .desired_width(f32::INFINITY)
                .hint_text("* SPICE deck\nV1 in 0 1\n.end"),
        );
        if response.changed() {
            app.state.workspace.netlist_source = Some(app.state.simulation.netlist_content.clone());
            app.state.workspace.netlist_source_path = None;
            app.state.workspace.netlist_source_dirty = true;
        }
        diagnostics(ui, app);
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

fn diagnostics(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let source = &app.state.simulation.netlist_content;
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| match rspice_core::Netlist::parse(source) {
            Ok(netlist) => {
                ui.horizontal_wrapped(|ui| {
                    status_dot(ui, t.color.ok, "Deck parses cleanly");
                    property_row(ui, "Elements", &netlist.elements.len().to_string());
                    property_row(ui, "Analyses", &netlist.analyses.len().to_string());
                    property_row(ui, "Models", &netlist.models.len().to_string());
                    property_row(ui, "Subcircuits", &netlist.subcircuits.len().to_string());
                });
                if !netlist.diagnostics.is_empty() {
                    ScrollArea::vertical().max_height(70.0).show(ui, |ui| {
                        for diagnostic in netlist.diagnostics {
                            ui.label(
                                egui::RichText::new(format!(
                                    "Line {} · {} · {}",
                                    diagnostic.line, diagnostic.code, diagnostic.message
                                ))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.warn),
                            );
                        }
                    });
                }
            }
            Err(error) => {
                status_dot(ui, t.color.err, "Deck has a blocking parse error");
                ui.label(
                    egui::RichText::new(error.to_string())
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.err),
                );
            }
        });
}
