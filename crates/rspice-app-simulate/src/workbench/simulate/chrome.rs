//! The run controls at the toolbar's trailing edge, drawn in the simulate
//! feature's context: Run or Stop with its menu, the simulation-plan chip
//! and the reference PVT chip.
//!
//! The chrome draws from the parts read here and hands back what the reader
//! pressed. A pressed command is raised, and the shell answers it once the
//! trailing group has drawn; a chosen reference PVT is committed here.

use crate::state::command_catalog::CommandKey;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::run_preflight;
use crate::workbench::chrome::toolbar::run_config_chip::{
    RunChipRequest, deck_chip, run_config_selector,
};
use crate::workbench::chrome::toolbar::simulate::{
    RunControls, RunEntry, pvt_selector, run_controls,
};
use crate::workbench::framework::command_registry::{CommandAvailability, ids};
use crate::workbench::framework::feature::{CodePort, Cx};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::Workspace;

use super::commands::SimulateCommand;
use super::{SimulateFeature, invalidate_preflight};

/// Draw the trailing group: the run controls, then the plan chip and the PVT
/// chip where `layout` has room for them. Where Run runs a deck, the deck's
/// chip stands in the plan chip's place, and pressing it brings the deck's
/// tab to the front of the Netlist workspace. The Netlist workspace runs
/// either, so there the chip naming what Run runs stays wherever the
/// toolbar's tools keep their labels.
pub fn show_trailing(cx: &mut Cx<'_, SimulateFeature>, ui: &mut egui::Ui, layout: LayoutSpec) {
    let names_target = layout.show_run_config_selector
        || (cx.wb.workspace == Workspace::Netlist && layout.toolbar_labels);
    let mut controls = run_control_parts(cx, ui);
    let pressed = run_controls(ui, &controls, layout);
    let deck = controls.deck.take();
    if let Some(key) = pressed {
        cx.execute(key);
    }
    if let Some(deck) = deck {
        if names_target
            && deck_chip(
                ui,
                &deck,
                cx.foreign_availability(ids::SHOW_RUN_DECK),
                layout.run_control_height,
            )
        {
            cx.execute(ids::SHOW_RUN_DECK);
        }
        return;
    }
    let manage = SimulateCommand::ManagePlans.key();
    if names_target
        && let Some(request) = run_config_selector(
            ui,
            cx.model,
            cx.foreign_availability(manage),
            layout.run_control_height,
        )
    {
        apply_run_chip_request(cx, request);
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

/// Do what the reader asked of the run chip: open the plan manager, or change
/// the analysis it names, as the Analyses page changes it.
fn apply_run_chip_request(cx: &mut Cx<'_, SimulateFeature>, request: RunChipRequest) {
    let lead = crate::workbench::chrome::toolbar::run_config_chip::lead_analysis(cx.model)
        .map(|lead| (lead.id(), lead.kind()));
    match (request, lead) {
        (RunChipRequest::ManagePlans, _) => cx.execute(SimulateCommand::ManagePlans.key()),
        (RunChipRequest::Refused(words), _) => cx.wb.prompt(words),
        (RunChipRequest::Kind(kind), Some((id, current))) if kind != current => {
            crate::workbench::surfaces::simulate::lifecycle::replace_analysis_kind(cx, id, kind);
        }
        (RunChipRequest::Draft(draft), Some((id, current))) if draft.kind() == current => {
            crate::workbench::surfaces::simulate::lifecycle::commit_draft(cx, id, *draft);
        }
        // A plan with nothing enabled gains the kind chosen.
        (RunChipRequest::Kind(kind), None) => {
            crate::workbench::surfaces::simulate::lifecycle::insert_analysis_instance(cx, kind);
        }
        (RunChipRequest::Kind(_) | RunChipRequest::Draft(_), _) => {}
    }
}

/// What the run control and its menu show, as the context stands: Run, or
/// Stop while a run or a preflight is active, the dispatcher's answers for
/// it and the menu's other two entries, and why it is unavailable.
fn run_control_parts<'a>(cx: &'a Cx<'_, SimulateFeature>, ui: &egui::Ui) -> RunControls<'a> {
    // A preflight preparing its run or building its decks, and a deck's run
    // being prepared, are stopped the way a run is: the control is Stop until
    // they finish.
    let checking = cx.runs.preflight.checking.is_some();
    let preparing_deck = cx.runs.preparing_deck.is_some();
    let execution_active =
        cx.model.simulation.execution.has_active_execution() || checking || preparing_deck;
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
    let mut primary = entry(primary);
    let (unreadable, deck) = run_deck(cx.wb.workspace, cx.peers.code(), cx.model);
    // The menu names what Run runs: a deck is not the active plan.
    if deck.is_some() && primary.key == SimulateCommand::Run.key() {
        primary.label = RUN_DECK;
    }
    let unavailable_reason = (!primary.enabled).then(|| match unreadable {
        Some(deck) if !execution_active => missing_deck_reason(cx, deck),
        _ => unavailable_reason(cx, primary.key, execution_active),
    });
    RunControls {
        model: cx.model,
        checking,
        preparing_deck,
        primary,
        preflight: entry(SimulateCommand::Preflight.key()),
        options: entry(SimulateCommand::Options.key()),
        unavailable_reason,
        deck,
    }
}

/// What Run reads as wherever it runs a deck: the run menu's entry, the run
/// history's primary and every list of the commands.
pub const RUN_DECK: &str = "Run deck";

/// The deck Run runs as the context stands, by its file's name, and the
/// project's entry for it while that file cannot be read; no name while Run
/// runs the plan. On the Netlist workspace that is the deck tab Run runs;
/// elsewhere, the project's deck while the deck is the project's
/// simulation. A deck whose file is missing is named too, and Run waits
/// until it is found.
pub fn run_deck<'m>(
    workspace: Workspace,
    code: &dyn CodePort,
    model: &'m Model,
) -> (Option<&'m crate::state::ProjectDeck>, Option<String>) {
    let project_deck = |id| {
        model
            .workspace
            .content
            .decks
            .iter()
            .find(|deck| deck.id == id)
    };
    if workspace == Workspace::Netlist {
        let unreadable = code.unreadable_run_deck().and_then(project_deck);
        let deck = code
            .manual_deck()
            .map(|deck| deck.name.to_owned())
            .or_else(|| unreadable.map(crate::state::ProjectDeck::name));
        (unreadable, deck)
    } else {
        let run_deck = code.project_run_deck(model);
        let named = run_deck.and_then(|deck| project_deck(deck.id));
        let unreadable = named.filter(|_| run_deck.is_some_and(|deck| deck.missing));
        (unreadable, named.map(crate::state::ProjectDeck::name))
    }
}

/// Whether Run runs a deck as the context stands, rather than the plan.
pub fn runs_deck(workspace: Workspace, code: &dyn CodePort, model: &Model) -> bool {
    run_deck(workspace, code, model).1.is_some()
}

/// Why Run cannot run `deck`: its file is not where the project says.
fn missing_deck_reason(cx: &Cx<'_, SimulateFeature>, deck: &crate::state::ProjectDeck) -> String {
    let path = deck
        .file_path(cx.model.workspace.content.project.data_root())
        .map(|path| path.display().to_string())
        .unwrap_or_default();
    cx.wb.messages().format(
        crate::workbench::localization::MessageId::NetlistDeckMissing,
        &[("name", &deck.name()), ("path", &path)],
    )
}

/// Why Run or Stop, `command`, is unavailable: while a run is active, the
/// dispatcher's reason; otherwise why the active plan cannot run.
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
    let step = cx.model.setup_snapshot(
        crate::workbench::app_state::design_history::SetupParts::REFERENCE_PVT
            .and(crate::workbench::app_state::design_history::SetupParts::OPTIONS),
    );
    let mut setup = cx.model.sim_setup.clone();
    setup.set_reference_pvt(process, temperature_celsius)?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "Reference PVT changed to {} at {temperature_celsius} degrees Celsius.",
            process.short_name()
        ))
        .map_err(|error| error.to_string())?;
    cx.model.sim_setup = setup;
    cx.model.record_setup_change("reference PVT", step);
    invalidate_preflight(cx);
    cx.model.record_plan_receipt(receipt.status_line());
    Ok(true)
}
