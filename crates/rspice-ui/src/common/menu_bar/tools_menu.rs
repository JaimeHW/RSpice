use egui::Ui;

use crate::common::app::{AppState, ConsoleMessage};
use crate::common::simulation_analysis_tabs;

pub(super) fn render_tools_menu(ui: &mut Ui, state: &mut AppState) {
    // Verilog-A submenu
    ui.menu_button("Verilog-A", |ui| {
        if ui.button("Compile Module...").clicked() {
            open_veriloga_compile_dialog(state);
            ui.close_menu();
        }
        if ui.button("Module Library...").clicked() {
            open_veriloga_module_library(state);
            ui.close_menu();
        }
        ui.separator();
        if ui.button("Compile Cache Status").clicked() {
            super::veriloga_cache::action_veriloga_cache_status(state);
            ui.close_menu();
        }
        if ui.button("List Compile Cache Entries").clicked() {
            super::veriloga_cache::action_veriloga_cache_list_entries(state);
            ui.close_menu();
        }
        if ui.button("Prune Compile Cache").clicked() {
            super::veriloga_cache::action_veriloga_cache_prune(state);
            ui.close_menu();
        }
        if ui.button("Clear Compile Cache").clicked() {
            super::veriloga_cache::action_veriloga_cache_clear(state);
            ui.close_menu();
        }
        if ui.button("Recompile Global Verilog-A Library").clicked() {
            super::veriloga_cache::action_veriloga_recompile_library(state);
            ui.close_menu();
        }
    });

    if ui.button("Model Browser...").clicked() {
        open_model_browser(state);
        ui.close_menu();
    }

    if ui.button("PDK Settings...").clicked() {
        open_pdk_settings(state);
        ui.close_menu();
    }

    if ui.button("Optimization Engine...").clicked() {
        open_optimization_engine(state);
        ui.close_menu();
    }

    if ui.button("Automation Console").clicked() {
        open_automation_console(state);
        ui.close_menu();
    }

    ui.separator();

    // Calculators submenu
    ui.menu_button("Calculators", |ui| {
        if ui.button("Unit Converter").clicked() {
            open_unit_converter(state);
            ui.close_menu();
        }
        if ui.button("Filter Design").clicked() {
            open_filter_calculator(state);
            ui.close_menu();
        }
        if ui.button("Impedance Matching").clicked() {
            open_impedance_calculator(state);
            ui.close_menu();
        }
        if ui.button("S-Parameter Converter").clicked() {
            open_sparameter_converter(state);
            ui.close_menu();
        }
    });

    ui.separator();

    // Verification tools
    if ui.button("Design Rule Check").clicked() {
        run_design_rule_check(state);
        ui.close_menu();
    }

    if ui.button("Electrical Rule Check").clicked() {
        run_electrical_rule_check(state);
        ui.close_menu();
    }

    if ui.button("LVS Check").clicked() {
        state.push_user_message(ConsoleMessage::info(
            "LVS: Requires layout data (not available in schematic-only mode)",
        ));
        ui.close_menu();
    }

    ui.separator();

    // Waveform tools
    ui.menu_button("Waveform Tools", |ui| {
        if ui.button("Calculator...").clicked() {
            open_waveform_calculator(state);
            ui.close_menu();
        }
        if ui.button("Measurements...").clicked() {
            open_measurements_panel(state);
            ui.close_menu();
        }
        if ui.button("Cross-Probe").clicked() {
            enable_cross_probe(state);
            ui.close_menu();
        }
    });
}

fn open_veriloga_compile_dialog(state: &mut AppState) {
    state.dialogs.veriloga_dialog.open();
}

fn open_veriloga_module_library(state: &mut AppState) {
    state.library_manager.select_library("veriloga");
    state.panels.project_explorer = false;
    state.panels.results_browser = false;
    state.panels.library_browser = true;
}

fn open_model_browser(state: &mut AppState) {
    state.model_browser_state.open = true;
    state.model_browser_state.browse_only = true;
}

fn open_pdk_settings(state: &mut AppState) {
    state.pdk_settings_dialog.open(state.pdk_config.clone());
}

fn open_optimization_engine(state: &mut AppState) {
    state.dialogs.sim_active_tab = simulation_analysis_tabs::TAB_OPTIMIZATION;
    state.dialogs.simulation_dialog = true;
}

fn open_automation_console(state: &mut AppState) {
    state.panels.script_console = true;
}

fn open_unit_converter(state: &mut AppState) {
    state.dialogs.unit_converter_dialog = true;
}

fn open_filter_calculator(state: &mut AppState) {
    state.dialogs.filter_calculator_dialog = true;
}

fn open_impedance_calculator(state: &mut AppState) {
    state.dialogs.impedance_calculator_dialog = true;
}

fn open_sparameter_converter(state: &mut AppState) {
    state.dialogs.sparam_converter_dialog = true;
}

fn open_waveform_calculator(state: &mut AppState) {
    state.dialogs.waveform_calculator_dialog = true;
}

fn open_measurements_panel(state: &mut AppState) {
    state.dialogs.measurements_panel = true;
}

fn enable_cross_probe(state: &mut AppState) {
    state.panels.signal_browser = true;
    state.push_user_message(ConsoleMessage::info("Cross-probe enabled"));
}

fn run_design_rule_check(state: &mut AppState) {
    // Run DRC on current schematic using the extraction bridge.
    let result = crate::services::drc::run_drc_check(&state.schematic);
    let summary = result.summary();
    let msg = if result.passed() {
        format!(
            "DRC passed: {} info, {} warnings",
            summary.info, summary.warnings
        )
    } else {
        format!(
            "DRC found {} violations ({} errors, {} critical)",
            summary.total, summary.errors, summary.critical
        )
    };
    state.push_user_message(ConsoleMessage::info(msg));
    state.dialogs.drc_results = Some(result);
    state.dialogs.drc_dialog = true;
}

fn run_electrical_rule_check(state: &mut AppState) {
    // ERC is part of DRC and runs connectivity checks.
    let result = crate::services::drc::run_drc_check(&state.schematic);
    let msg = if result.passed() {
        "ERC passed: No electrical rule violations".to_string()
    } else {
        format!("ERC found {} violations", result.total_count())
    };
    state.push_user_message(ConsoleMessage::info(msg));
    state.dialogs.drc_results = Some(result);
    state.dialogs.drc_dialog = true;
}
