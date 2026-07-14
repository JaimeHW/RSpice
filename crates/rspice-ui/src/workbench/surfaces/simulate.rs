//! Ordered simulation-plan editor and preflight.

mod analysis_form;

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{card, heading, property_row, status_dot};

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt("workbench.simulate.surface")
            .show(ui, |ui| {
                ui.add_space(18.0);
                ui.horizontal(|ui| {
                    ui.add_space(22.0);
                    ui.vertical(|ui| {
                        heading(
                            ui,
                            "Simulation-plan owner",
                            "Lab characterization",
                            "Ordered analyses, model context, solver policy, and fail-closed preflight.",
                        );
                    });
                });
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    ui.add_space(22.0);
                    ui.vertical(|ui| {
                        ui.set_max_width(920.0);
                        plan_summary(ui, app);
                        ui.add_space(12.0);
                        analysis_editor(ui, app);
                        ui.add_space(12.0);
                        preflight(ui, app);
                    });
                });
                ui.add_space(22.0);
            });
    });
}

fn plan_summary(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Active plan", |ui| {
        let enabled = ordered_enabled(app);
        let run_order = if enabled.is_empty() {
            "No analyses enabled".to_owned()
        } else {
            enabled.join(" → ")
        };
        property_row(ui, "Run order", &run_order);
        property_row(
            ui,
            "Process corner",
            app.state.sim_setup.reference_pvt.process.short_name(),
        );
        property_row(
            ui,
            "Temperature",
            &format!(
                "{} °C",
                app.state.sim_setup.reference_pvt.temperature_celsius
            ),
        );
        property_row(
            ui,
            "Model libraries",
            &app.state.model_library_manager.library_count().to_string(),
        );
        ui.horizontal_wrapped(|ui| {
            let run = if app.state.simulation.is_running {
                "Stop active run"
            } else {
                "Run active plan"
            };
            if ui
                .add_enabled(
                    Command::RunSimulation.is_enabled(app) || app.state.simulation.is_running,
                    egui::Button::new(run),
                )
                .clicked()
            {
                if app.state.simulation.is_running {
                    Command::StopSimulation.execute(app);
                } else {
                    Command::RunSimulation.execute(app);
                }
            }
            if ui.button("Global solver and convergence…").clicked() {
                Command::SimulationOptions.execute(app);
            }
            if ui.button("Inspect generated netlist").clicked() {
                Command::GenerateNetlist.execute(app);
            }
        });
    });
}

fn analysis_editor(ui: &mut Ui, app: &mut RSpiceApp) {
    let index = app.state.workbench.active_analysis.min(24);
    app.state.workbench.active_analysis = index;
    let name = analysis_name(index);
    card(ui, name, |ui| {
        let mut enabled = app.state.sim_setup.enabled.contains(&index);
        ui.horizontal(|ui| {
            if ui
                .checkbox(&mut enabled, "Include in active run set")
                .changed()
            {
                if enabled {
                    app.state.sim_setup.enabled.insert(index);
                    app.state.sim_setup.listed.insert(index);
                } else {
                    app.state.sim_setup.enabled.remove(&index);
                }
            }
            ui.label(
                egui::RichText::new(format!("Order {}", run_order(app, index)))
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(Tokens::get(ui.ctx()).color.text_faint),
            );
        });
        ui.separator();
        let note = analysis_form::form(ui, &mut app.state.sim_setup, index);
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(note)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(Tokens::get(ui.ctx()).color.text_dim),
        );
        if let Some(error) = app.state.sim_setup.validation_error(index) {
            ui.add_space(6.0);
            ui.label(egui::RichText::new(error).color(Tokens::get(ui.ctx()).color.err));
        } else {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.ok,
                "Analysis configuration valid",
            );
        }
    });
}

fn preflight(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Preflight", |ui| {
        let topology_ok = !app.state.schematic.components.is_empty();
        let analyses_ok = !app.state.sim_setup.enabled.is_empty();
        let checks_ok = app.state.current_blocking_drc_result().is_none();
        let configurations_ok = app
            .state
            .sim_setup
            .enabled
            .iter()
            .all(|index| app.state.sim_setup.validation_error(*index).is_none());
        preflight_row(
            ui,
            topology_ok,
            "Design topology",
            if topology_ok {
                "Circuit contains devices"
            } else {
                "Add at least one component"
            },
        );
        preflight_row(
            ui,
            analyses_ok,
            "Run set",
            if analyses_ok {
                "At least one analysis enabled"
            } else {
                "Enable an analysis"
            },
        );
        preflight_row(
            ui,
            configurations_ok,
            "Analysis configuration",
            if configurations_ok {
                "All enabled analyses validate"
            } else {
                "Correct invalid analysis fields"
            },
        );
        preflight_row(
            ui,
            checks_ok,
            "Blocking checks",
            if checks_ok {
                "No current blocking violations"
            } else {
                "Resolve current schematic errors"
            },
        );
        if let Some(reason) = app.state.simulation_run_block_reason() {
            ui.add_space(6.0);
            ui.label(egui::RichText::new(reason).color(Tokens::get(ui.ctx()).color.warn));
        }
    });
}

fn preflight_row(ui: &mut Ui, pass: bool, name: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        status_dot(
            ui,
            if pass { t.color.ok } else { t.color.err },
            if pass { "PASS" } else { "BLOCK" },
        );
        ui.label(egui::RichText::new(name).strong());
        ui.label(egui::RichText::new(detail).color(t.color.text_dim));
    });
}

fn ordered_enabled(app: &RSpiceApp) -> Vec<&'static str> {
    (0..25)
        .filter(|index| app.state.sim_setup.enabled.contains(index))
        .map(analysis_name)
        .collect()
}

fn run_order(app: &RSpiceApp, index: usize) -> String {
    if !app.state.sim_setup.enabled.contains(&index) {
        return "—".to_owned();
    }
    let position = app
        .state
        .sim_setup
        .enabled
        .iter()
        .filter(|candidate| **candidate <= index)
        .count();
    position.to_string()
}

fn analysis_name(index: usize) -> &'static str {
    crate::common::simulation_analysis_tabs::SIMULATION_ANALYSIS_CATEGORIES
        .iter()
        .flat_map(|(_, analyses)| analyses.iter())
        .find_map(|(candidate, label)| (*candidate == index).then_some(*label))
        .unwrap_or("Unknown analysis")
}
