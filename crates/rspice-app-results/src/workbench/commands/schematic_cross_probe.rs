//! Showing a result signal on the schematic: the conductor of a net's
//! voltage, the part a current flows through, or the pin a current is read
//! at. Every Results surface that names a signal (a data-browser row, the
//! inspector's selected trace, a run's analyses in Run details, a noise
//! contributor) takes this one route, so a signal reaches its place in one
//! action from wherever the reader found it.

use rspice_app_workbench::workbench::framework::requests::{Drain, Request};
use rspice_schematic_editor::view::cross_probe::{self, LocateSignalError};

use crate::state::InstancePath;
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Execute;
use crate::workbench::framework::requests::PromptAfterCommands;
use crate::workbench::state::Workspace;

/// Land the Design canvas on the occurrence `scope` names, as the engine
/// spells it, and show `signal` there. Raised for a signal read in another
/// occurrence than the tab in front, and answered by the shell straight after
/// the call that raised it, since only Design ascends and descends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowSignalOnSchematic {
    pub signal: String,
    pub scope: InstancePath,
    /// What the status line says the selection was made from: the signal
    /// itself, or the result row that names it.
    pub from: String,
}

impl Request for ShowSignalOnSchematic {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Why `signal` has no one place on the schematic: it is computed from other
/// signals rather than carried by one net or part.
pub fn schematic_refusal(signal: &str) -> Option<String> {
    (!cross_probe::locates_on_schematic(signal)).then(|| LocateSignalError::NotANet.message(signal))
}

/// Show `signal` where the drawing carries it, selected and centred, and
/// open Design there. A signal read on another sheet brings that sheet
/// forward; one read in another occurrence takes the canvas to that
/// occurrence first. The status line says what was selected, or why nothing
/// was, in which case the reader stays where they are.
pub fn show_signal_on_schematic(state: &mut ResultsParts<'_>, signal: &str) -> Result<(), String> {
    if let Some(refusal) = schematic_refusal(signal) {
        return Err(refusal);
    }
    show_on_schematic(state, signal, signal)
}

/// Show what `signal` locates, saying it was selected `from` what the reader
/// acted on.
fn show_on_schematic(state: &mut ResultsParts<'_>, signal: &str, from: &str) -> Result<(), String> {
    if let Some(scope) = state.model.signal_read_elsewhere(signal) {
        state.workbench.outbox.push(ShowSignalOnSchematic {
            signal: signal.to_owned(),
            scope,
            from: from.to_owned(),
        });
        return Ok(());
    }
    let place = state
        .model
        .select_signal_on_drawing(signal)
        .map_err(|error| error.message(from))?;
    if state.workbench.workspace != Workspace::Design {
        state.workbench.outbox.push(Execute(ids::DESIGN_WORKSPACE));
    }
    state
        .workbench
        .outbox
        .push(PromptAfterCommands(place.message(from)));
    Ok(())
}
