//! The run controls at the toolbar's trailing edge, drawn in the simulate
//! feature's context: Run or Stop with its menu, the simulation-plan chip
//! and the reference PVT chip.
//!
//! The chrome draws from the parts read here and hands back what the reader
//! pressed. A pressed command is raised, and the shell answers it once the
//! trailing group has drawn; a chosen reference PVT is committed here.

use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::run_preflight;
use crate::workbench::chrome::toolbar::run_config_chip::run_config_selector;
use crate::workbench::chrome::toolbar::simulate::{
    RunControls, RunEntry, pvt_selector, run_controls,
};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::feature::Cx;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::Workspace;

use super::commands::SimulateCommand;
use super::{SimulateFeature, invalidate_preflight};

/// Draw the trailing group: the run controls, then the plan chip and the PVT
/// chip where `layout` has room for them.
pub fn show_trailing(cx: &mut Cx<'_, SimulateFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    let controls = run_control_parts(cx, ui);
    if let Some(key) = run_controls(ui, &controls, layout) {
        cx.execute(key);
    }
    let manage = SimulateCommand::ManagePlans.key();
    if layout.show_run_config_selector
        && run_config_selector(
            ui,
            cx.model,
            cx.foreign_availability(manage),
            layout.run_control_height,
        )
    {
        cx.execute(manage);
    }
    if layout.show_pvt_selector
        && let Some(choice) = pvt_selector(ui, cx.model, layout.toolbar_control_height)
    {
        match commit_reference_pvt(cx, choice.process, choice.temperature_celsius) {
            Ok(true) => cx
                .wb
                .prompt(format!("Reference PVT changed to {}.", choice.label)),
            Ok(false) => {}
            Err(error) => cx
                .wb
                .prompt(format!("Reference PVT was not changed: {error}")),
        }
    }
}

/// What the run control and its menu show, as the context stands: Run, or
/// Stop while a run or a preflight is active, the dispatcher's answers for
/// it and the menu's other two entries, and why it is unavailable.
fn run_control_parts<'a>(cx: &'a Cx<'_, SimulateFeature>, ui: &egui::Ui) -> RunControls<'a> {
    // A preflight preparing its run or building its decks is stopped the way
    // a run is: the control is Stop until it finishes.
    let checking = cx.runs.preflight.checking.is_some();
    let execution_active = cx.model.simulation.execution.has_active_execution() || checking;
    let primary = if execution_active {
        SimulateCommand::Stop
    } else {
        SimulateCommand::Run
    }
    .key();
    let entry = |key: CommandKey| RunEntry {
        key,
        label: cx.commands.label(key),
        enabled: cx.foreign_available(key),
        shortcut: cx.model.preferences.shortcuts().resolved_label(
            cx.commands,
            key,
            crate::workbench::app_state::runtime_command_platform(ui.ctx()),
            ui.ctx().os(),
        ),
    };
    let primary = entry(primary);
    let unavailable_reason =
        (!primary.enabled).then(|| unavailable_reason(cx, primary.key, execution_active));
    RunControls {
        model: cx.model,
        checking,
        primary,
        preflight: entry(SimulateCommand::Preflight.key()),
        options: entry(SimulateCommand::Options.key()),
        unavailable_reason,
    }
}

/// Why Run or Stop, `command`, is unavailable: while a run is active, the
/// dispatcher's reason; on the Netlist workspace, why its deck cannot run;
/// elsewhere, why the active plan cannot.
fn unavailable_reason(
    cx: &Cx<'_, SimulateFeature>,
    command: CommandKey,
    execution_active: bool,
) -> String {
    let reason = if execution_active {
        match cx.foreign_availability(command) {
            CommandAvailability::Disabled(reason) => Some(reason.to_owned()),
            CommandAvailability::Available | CommandAvailability::Hidden => None,
        }
    } else if cx.wb.workspace == Workspace::Netlist {
        cx.peers
            .code()
            .manual_deck_run_block_reason(cx.model, cx.wb.messages())
            .or_else(|| {
                run_preflight::manual_deck_authorization_block_reason(
                    cx.model,
                    cx.handles.run_controller,
                )
            })
    } else {
        run_preflight::simulation_run_block_reason(
            cx.model,
            cx.runs.slot_facts(cx.peers.design().review_markers()),
        )
    };
    reason.unwrap_or_else(|| "Command unavailable".to_owned())
}

/// Make (`process`, `temperature_celsius`) the reference PVT, as a change to
/// the active plan's configuration with its receipt. Returns whether it
/// changed anything; why the plan refused it is the error.
pub fn commit_reference_pvt(
    cx: &mut Cx<'_, SimulateFeature>,
    process: crate::product::ProcessCorner,
    temperature_celsius: f64,
) -> Result<bool, String> {
    let current = cx.model.sim_setup.reference_pvt;
    if current.process == process
        && (current.temperature_celsius - temperature_celsius).abs() < f64::EPSILON
    {
        return Ok(false);
    }
    let mut setup = cx.model.sim_setup.clone();
    setup.set_reference_pvt(process, temperature_celsius)?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "Reference PVT changed to {} at {temperature_celsius} degrees Celsius.",
            process.short_name()
        ))
        .map_err(|error| error.to_string())?;
    cx.model.sim_setup = setup;
    invalidate_preflight(cx);
    cx.model.record_plan_receipt(receipt.status_line());
    Ok(true)
}
