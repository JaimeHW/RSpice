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
//! # It reports; the caller asks for the deck
//!
//! The record is rendered from a borrow of the run it describes, and opening
//! the deck mutates the session. So the deck's line is a link whose place
//! [`lines`] returns, and once that borrow is done the caller asks for the
//! run's executed deck, which opens it or says by name that it is no longer
//! held.

use super::Line;

/// The deck the run executed and the model sources it was sealed under, as
/// lines of Run details…'s Inputs section; the place of the line that opens
/// the deck.
///
/// `executed` is `None` when the archive holds no deck for this run. That is
/// stated rather than papered over: the working deck is a different document
/// and showing it here would be a confident lie. Decks are written to the
/// project file ([`crate::state::ExecutedDeckArchive`]), so a reopened project
/// reopens the decks its retained runs executed, and a missing one was let go
/// because only the newest runs' decks are kept.
pub(super) fn lines(lines: &mut Vec<Line>, executed: Option<&[String]>) -> Option<usize> {
    let Some(sources) = executed else {
        lines.push(Line::new(
            "Deck",
            format!(
                "Not kept; RSpice keeps the decks of the last {} runs",
                rspice_results::executed_deck::MAX_RETAINED_RUNS
            ),
        ));
        return None;
    };
    let deck = lines.len();
    lines.push(
        Line::new("Deck", "As executed, read-only")
            .linked("Opens the netlist exactly as this run simulated it, read-only"),
    );
    lines.extend(
        sources
            .iter()
            .map(|source| Line::new("Model source", source.as_str())),
    );
    Some(deck)
}
