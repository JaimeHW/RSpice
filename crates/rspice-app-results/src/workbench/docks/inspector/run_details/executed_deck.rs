//! What the run above this record was actually handed, and the route to it.
//!
//! The receipt beside it names project-owned model *definitions* by digest,
//! which is a precise fact about a narrow set: only models this project
//! authored are admitted there at all. A library file, a built-in card and a
//! retained import reach a run without appearing in it.
//!
//! What the run's own decks were sealed under covers all of them, because it
//! is a comment the engine was given rather than a projection of the project.
//! It is coarser — a library, not a digest per definition — and it is the only
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
/// The deck is named by the file a deck run read, `file`, its whole path in
/// the hover; a plan's run read no file, and its deck is the one it executed.
///
/// `executed` is `None` when the archive holds no deck for this run. That is
/// stated rather than papered over: the working deck is a different document
/// and showing it here would be a confident lie. Decks are written to the
/// project file ([`crate::state::ExecutedDeckArchive`]), so a reopened project
/// reopens the decks its retained runs executed, and a missing one was let go
/// because only the newest runs' decks are kept, which its hover says.
pub(super) fn lines(
    lines: &mut Vec<Line>,
    executed: Option<&[String]>,
    file: &str,
) -> Option<usize> {
    let name = (!file.is_empty()).then(|| {
        std::path::Path::new(file)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(file)
    });
    let Some(sources) = executed else {
        let why = format!(
            "RSpice keeps the decks of the last {} runs",
            rspice_results::executed_deck::MAX_RETAINED_RUNS
        );
        lines.push(match name {
            Some(name) => Line::new("Deck", format!("{name}, not kept"))
                .whole(format!("{file}; not kept, for {why}")),
            None => Line::new("Deck", "Not kept").whole(format!("Not kept; {why}")),
        });
        return None;
    };
    let deck = lines.len();
    let line = match name {
        Some(name) => Line::new("Deck", name).whole(file),
        None => Line::new("Deck", "As executed"),
    };
    lines.push(line.linked("Opens the netlist exactly as this run simulated it, read-only"));
    lines.extend(
        sources
            .iter()
            .map(|source| Line::new("Model source", source.as_str())),
    );
    Some(deck)
}
