use egui::Ui;

use crate::common::app::{AppState, ConsoleMessage};
use crate::common::export_workflow::ExportWorkflowIo;
use crate::common::simulation_analysis_tabs;

pub(super) fn render_simulate_menu(
    ui: &mut Ui,
    state: &mut AppState,
    export_workflow_io: &(impl ExportWorkflowIo + ?Sized),
) {
    // Quick run - uses currently selected analysis
    if ui.button("Run Simulation    F5").clicked() {
        start_simulation(state, None);
        ui.close_menu();
    }

    // Run Analysis submenu for quick access to specific analyses
    ui.menu_button("Run Analysis", |ui| {
        for &(name, tab) in simulation_analysis_tabs::QUICK_RUN_ANALYSES {
            if ui.button(name).clicked() {
                state.dialogs.sim_active_tab = tab;
                start_simulation(state, Some(name));
                ui.close_menu();
            }
        }
    });

    if ui.button("Stop Simulation").clicked() {
        stop_simulation(state);
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Setup...     Ctrl+Shift+S").clicked() {
        state.dialogs.simulation_dialog = true;
        ui.close_menu();
    }

    if ui.button("Options...").clicked() {
        open_simulation_options(state);
        ui.close_menu();
    }

    ui.separator();

    // Netlist menu
    ui.menu_button("Netlist", |ui| {
        if ui.button("View Netlist").clicked() {
            super::export_actions::action_view_netlist(state);
            ui.close_menu();
        }
        if ui.button("Export SPICE Netlist...").clicked() {
            super::export_actions::action_export_netlist_with_io(
                state,
                crate::io::NetlistFormat::Spice,
                export_workflow_io,
            );
            ui.close_menu();
        }
        if ui.button("Export Spectre Netlist...").clicked() {
            super::export_actions::action_export_netlist_with_io(
                state,
                crate::io::NetlistFormat::Spectre,
                export_workflow_io,
            );
            ui.close_menu();
        }
    });
}

fn start_simulation(state: &mut AppState, analysis_name: Option<&str>) {
    log::info!(
        "Run button clicked! Components: {}",
        state.schematic.components.len()
    );

    if state.schematic.components.is_empty() {
        state.push_user_message(ConsoleMessage::warning(
            "No circuit to simulate. Add components first.",
        ));
        return;
    }

    log::info!("Setting trigger_simulation = true");
    state.simulation.trigger_simulation = true;
    match analysis_name {
        Some(name) => state.push_user_message(ConsoleMessage::info(format!(
            "Starting {} analysis...",
            name
        ))),
        None => state.push_user_message(ConsoleMessage::info("Simulation started...")),
    }
}

fn stop_simulation(state: &mut AppState) {
    state.simulation.is_running = false;
    state.push_user_message(ConsoleMessage::warning("Simulation stopped"));
}

fn open_simulation_options(state: &mut AppState) {
    state.dialogs.simulation_options_state =
        crate::simulation::dialog::OptionsDialogState::from_options(
            &state.dialogs.simulation_options_config,
        );
    state.dialogs.simulation_options_errors.clear();
    state.dialogs.simulation_options = true;
}

