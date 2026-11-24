//! The bands above a page whose run the machine's memory holds back, in the
//! words of the run's source: a deck's run on the Netlist page, a plan's run
//! of the design on the Simulate page. A run that needs more memory than is
//! free waits for Run anyway or Stop, and where Run anyway would keep no
//! unsaved work first, the band says to save first. A run that needs more
//! than is installed is refused, and the band says what it needs against
//! what the machine has, in a whole sentence.

use egui::Ui;

use crate::simulation::execution::run_memory::{MemoryShortfall, RunOf};
use crate::ui::widgets::{HeldRunAnswer, HeldRunBand, held_run_band, refused_run_band};
use crate::workbench::localization::{MessageCatalog, MessageId};
use rspice_app_runs::workbench::slots::runs::RunsState;

/// The band of the run of `source` held because it needs more memory than
/// the machine has free, and what the reader answered there. Nothing while
/// no run of it is held.
pub fn held_run(
    ui: &mut Ui,
    runs: &RunsState,
    messages: MessageCatalog,
    source: RunOf,
) -> Option<HeldRunAnswer> {
    let warning = runs.held_run(source)?;
    let unkept = runs.held_run_unkept;
    let run_anyway_hint = match (source, unkept) {
        (RunOf::Deck, false) => MessageId::NetlistMemoryRunAnywayTooltip,
        (RunOf::Deck, true) => MessageId::NetlistMemoryRunAnywayUnkeptTooltip,
        (RunOf::Design, false) => MessageId::SimulateMemoryRunAnywayTooltip,
        (RunOf::Design, true) => MessageId::SimulateMemoryRunAnywayUnkeptTooltip,
    };
    let caution = unkept.then(|| {
        (
            messages.text(MessageId::MemoryUnsavedUnkept),
            messages.text(MessageId::MemoryUnsavedUnkeptTooltip),
        )
    });
    held_run_band(
        ui,
        &HeldRunBand {
            numbers: &warning.numbers_of(source),
            message: &warning.message_of(source),
            caution: caution
                .as_ref()
                .map(|(clause, why)| (clause.as_str(), why.as_str())),
            run_anyway: &messages.text(MessageId::NetlistMemoryRunAnyway),
            run_anyway_hint: &messages.text(run_anyway_hint),
            stop: &messages.text(MessageId::NetlistMemoryStop),
            stop_hint: &messages.text(MessageId::NetlistMemoryStopTooltip),
        },
    )
}

/// The band of the run of `source` refused because it needs more memory
/// than the machine has installed (`shortfall`): what it needs against what
/// the machine has, the whole refusal on its hover.
pub fn refused_run(ui: &mut Ui, shortfall: &MemoryShortfall, source: RunOf) {
    refused_run_band(
        ui,
        &shortfall.numbers_of(source),
        &shortfall.message_of(source),
    );
}
