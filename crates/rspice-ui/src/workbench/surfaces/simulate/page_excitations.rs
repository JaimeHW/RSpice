//! Excitations, and the analyses that read them.
//!
//! The design side of the plan: what this circuit is driven by. The plan's
//! other registry pages own something the plan itself holds, so they can edit
//! it in place. This one does not — a source is a placed instance, and the
//! sheet that owns it is the schematic. So the page reads, and a row hops to
//! the instance rather than pretending to edit it here.
//!
//! Both this page and the Design navigator's rail render
//! [`crate::simulation::placed_sources::placed_sources`] and nothing else, so
//! the two can never disagree about which analysis reads which source.

use egui::Ui;

use crate::simulation::placed_sources::{PlacedSource, SourceConsumer};
use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;

use super::page_kit::{Tone, card, card_note, ledger_head, ledger_row};

/// Reference, quantity, waveform, terminals, and what reads it.
const EXCITATION_COLUMNS: [f32; 5] = [0.14, 0.06, 0.26, 0.24, 0.30];

/// The page renders the list its own heading counted.
///
/// Lent by [`super::pages::show`] rather than resolved again here: resolving it
/// walks the design's nets, and the heading above this table needs the same
/// answer, so a frame showing this page used to pay for the walk twice.
pub(super) fn show(ui: &mut Ui, state: &mut AppState, sources: &[PlacedSource]) {
    // Readership is run-scoped: a disabled instance is not in the run this plan
    // would dispatch, so a source only it names is one the run drives without
    // reading.
    let unread = sources.iter().filter(|source| !source.is_read()).count();
    let status = if sources.is_empty() {
        Some(("no sources placed", Tone::Warn))
    } else if unread > 0 {
        Some(("sources with no reader", Tone::Warn))
    } else {
        Some(("every source is read", Tone::Ok))
    };

    card(ui, "Placed excitations", status, |ui| {
        if sources.is_empty() {
            card_note(
                ui,
                "This design places no independent sources. An analysis that names a source \
                 will not resolve until one is drawn on the schematic.",
            );
            return;
        }
        ledger_head(
            ui,
            &EXCITATION_COLUMNS,
            &["Reference", "", "Waveform", "Terminals", "Read by"],
        );
        for source in sources {
            let row = excitation_row(ui, state, source);
            if row.clicked() {
                reveal(state, source);
            }
        }
        if unread > 0 {
            card_note(
                ui,
                &format!(
                    "{unread} of {} sources are read by no enabled analysis in this plan \u{2014} \
                     named by none, and with no enabled analysis that reads every source. A \
                     disabled instance that names one is listed on its row and does not count. \
                     They are still netlisted and still drive the circuit.",
                    sources.len()
                ),
            );
        }
    });
}

/// One source's row. The reader column carries the finding, so it is the only
/// cell that takes a tone.
fn excitation_row(ui: &mut Ui, state: &AppState, source: &PlacedSource) -> egui::Response {
    // Only the instances the run contains are counted. A disabled one is named
    // in the tooltip, marked disabled, because it is the thing a reader
    // re-enables to change this answer.
    let reading: Vec<&SourceConsumer> = source.reading_consumers().collect();
    let disabled = source.consumers.len() - reading.len();
    let (readers, tone) = match reading.len() {
        0 if disabled > 0 => (
            format!("no reader \u{00b7} {disabled} disabled"),
            Tone::Warn,
        ),
        0 => ("no reader".to_owned(), Tone::Warn),
        1 => (
            format!("{} \u{00b7} {}", reading[0].analysis, reading[0].role),
            Tone::Neutral,
        ),
        // The roles differ once whole-design readers are listed beside named
        // ones, and naming the first consumer's role for all of them would
        // state a part the other analyses do not play. The tooltip has room
        // for the full reading.
        count => {
            let first = reading[0].role;
            let uniform = reading.iter().all(|consumer| consumer.role == first);
            (
                if uniform {
                    format!("{count} analyses \u{00b7} {first}")
                } else {
                    format!("{count} analyses")
                },
                Tone::Neutral,
            )
        }
    };
    let terminals = source.nets.join(" \u{2192} ");
    let selected = state.schematic.selection.has_component(source.component_id);
    ledger_row(
        ui,
        &EXCITATION_COLUMNS,
        &[
            (source.reference.as_str(), Tone::Neutral),
            (source.quantity(), Tone::Accent),
            (source.summary().as_str(), Tone::Neutral),
            (terminals.as_str(), Tone::Neutral),
            (readers.as_str(), tone),
        ],
        selected,
    )
    .on_hover_text(row_tooltip(source))
}

/// Everything the row had to shorten: every reader, and the part it plays.
fn row_tooltip(source: &PlacedSource) -> String {
    let mut lines = vec![format!(
        "{} \u{00b7} {}",
        source.reference,
        source.summary()
    )];
    if source.consumers.is_empty() {
        lines.push(
            "No analysis in this plan names this source, and none reads every source".to_owned(),
        );
    } else {
        for consumer in &source.consumers {
            lines.push(format!(
                "{} \u{00b7} {}{}",
                consumer.analysis,
                consumer.role,
                if consumer.reads() {
                    ""
                } else {
                    " \u{00b7} disabled"
                }
            ));
        }
    }
    lines.push("Click to select it on the schematic".to_owned());
    lines.join("\n")
}

/// Select the instance and centre the drawing on it, then show the drawing.
///
/// The same select-and-centre transaction the result viewers use to reach a
/// device, because arriving at a selected-but-offscreen instance is the one
/// outcome that reads as a broken link.
fn reveal(state: &mut AppState, source: &PlacedSource) {
    let position = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == source.component_id)
        .map(|component| component.pos);
    state
        .schematic
        .selection
        .select_only_component(source.component_id);
    state.schematic.net_highlight.clear();
    state.schematic.center_request = position;
    state.workbench.activate(Workspace::Design);
}
