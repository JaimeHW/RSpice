//! What the run above this record was actually handed, and the route to it.
//!
//! The receipt beside it names project-owned model *definitions* by digest,
//! which is a precise fact about a narrow set: only models this project
//! authored are admitted there at all. A pack release, a built-in card and a
//! retained import reach a run without appearing in it.
//!
//! What the run's own decks were sealed under covers all of them, because it
//! is a comment the engine was given rather than a projection of the project.
//! It is coarser — a release, not a digest per definition — and it is the only
//! statement here that describes every model source a result depends on.
//!
//! # It reports, then acts on the state and nothing wider
//!
//! The record is rendered from a borrow of the run it describes, and opening
//! the deck mutates the session. So [`record`] returns the run a reader asked
//! for and the caller calls [`reveal`] once that borrow is done. Both halves
//! live here — the offer and what honouring it means, including the refusal
//! when the deck is no longer held — but neither takes the whole application
//! to save a line at the call site.

use egui::Ui;

use super::super::super::design_system::{property_row, section_header};

/// The run's sealed model sources, and the control that opens its deck.
///
/// `executed` is `None` when this session does not hold the run's decks — a
/// dataset restored from a project file, or one whose decks the archive has
/// since dropped. That is stated rather than papered over: the working deck is
/// a different document and showing it here would be a confident lie.
pub(super) fn record(ui: &mut Ui, run_id: u64, executed: Option<&[String]>) -> Option<u64> {
    section_header(ui, "Executed deck", None);
    let Some(sources) = executed else {
        property_row(
            ui,
            "Deck",
            "not retained — this session did not run this dataset",
        );
        return None;
    };
    if sources.is_empty() {
        property_row(ui, "Sealed model sources", "none in this deck");
    } else {
        for source in sources {
            property_row(ui, "Sealed model source", source);
        }
    }
    ui.button("Open executed deck")
        .on_hover_text(
            "Opens the exact source this run's engine read, as a read-only document sealed with \
             the run.",
        )
        .clicked()
        .then_some(run_id)
}

/// Open the deck the record above offers, or say why it cannot be opened.
///
/// The route reports whether the run's decks are still held, and a silent
/// no-op on a control that promises a document is the one outcome a reader
/// cannot tell from success. The record is rebuilt every frame from the same
/// retained archive, so this fires when the deck was released between the
/// frame that drew the control and the click.
///
/// Takes the state rather than the application, which is the same rule the
/// header states: the borrow the record was rendered from is finished by the
/// time the caller gets here.
pub(super) fn reveal(state: &mut crate::workbench::AppState, run_id: u64) {
    if crate::workbench::documents::netlist_document::reveal_executed_deck(state, run_id, 0) {
        return;
    }
    state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
        "The decks Run {run_id} executed are no longer retained in this project."
    )));
}
