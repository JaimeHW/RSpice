//! Excitations, and the analyses that read them.
//!
//! The design side of the plan: what this circuit is driven by. The plan's
//! other registry pages own something the plan itself holds, so they can edit
//! it in place. This one does not — a source is a placed instance, and the
//! sheet that owns it is the schematic. So the page reads, and a row hops to
//! the instance rather than pretending to edit it here.
//!
//! Both this page and the Design navigator's rail render
//! [`crate::simulation::placed_sources::design_sources`] and
//! [`crate::simulation::placed_sources::design_rf_ports`] and nothing else, so
//! the two can never disagree about which analysis reads which excitation.
//!
//! The two lists share one table under two heads rather than sitting in two
//! cards. A reader asking what drives this circuit is asking one question, and
//! a port is an answer to it — but the third column means a different thing for
//! each, so each block names its own columns.
//!
//! Both are whole-design lists. A run flattens the hierarchy, so a source drawn
//! inside a child master drives this circuit exactly as one drawn at the root
//! does, and the Occurrence column is what tells the two apart — including the
//! two rows one drawn source becomes when two instances reach its master.

use egui::Ui;

use crate::simulation::placed_sources::{
    PlacedRfPort, PlacedSource, SourceConsumer, duplicate_port_numbers,
};
use crate::state::InstancePath;
use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;

use super::page_kit::{Tone, card, card_note, ledger_head, ledger_row};

/// Reference, quantity, the occurrence it is reached through, waveform,
/// terminals, and what reads it.
///
/// The occurrence sits beside the reference because it qualifies it: `V1` is
/// not a name in a hierarchical design until the path in front of it is read,
/// and two rows can carry one reference.
const EXCITATION_COLUMNS: [f32; 6] = [0.13, 0.05, 0.14, 0.22, 0.20, 0.26];

/// The page renders the lists its own heading counted.
///
/// Lent by [`super::pages::show`] rather than resolved again here: resolving
/// either one walks the design's nets, and the heading above this table needs
/// the same answers, so a frame showing this page used to pay for the walk
/// twice.
pub(super) fn show(
    ui: &mut Ui,
    state: &mut AppState,
    sources: &[PlacedSource],
    ports: &[PlacedRfPort],
) {
    // Readership is run-scoped: a disabled instance is not in the run this plan
    // would dispatch, so a source only it names is one the run drives without
    // reading.
    let unread = sources.iter().filter(|source| !source.is_read()).count();

    card(
        ui,
        "Placed excitations",
        Some(verdict(sources, ports)),
        |ui| {
            if sources.is_empty() && ports.is_empty() {
                card_note(
                    ui,
                    "This design places no independent sources. An analysis that names a source \
                     will not resolve until one is drawn on the schematic.",
                );
                return;
            }
            if !sources.is_empty() {
                ledger_head(
                    ui,
                    &EXCITATION_COLUMNS,
                    &[
                        "Reference",
                        "",
                        "Occurrence",
                        "Waveform",
                        "Terminals",
                        "Read by",
                    ],
                );
                for source in sources {
                    let row = excitation_row(ui, state, source);
                    if row.clicked() {
                        reveal(state, source.occurrence.as_ref(), source.component_id);
                    }
                }
            }
            // A second head rather than a group caption: a port's third cell
            // states what the port is and the impedance it presents, which is
            // not a waveform, and a column label that covered both would name
            // neither.
            if !ports.is_empty() {
                ledger_head(
                    ui,
                    &EXCITATION_COLUMNS,
                    &[
                        "Reference",
                        "",
                        "Occurrence",
                        "Port",
                        "Terminals",
                        "Read by",
                    ],
                );
                for port in ports {
                    let row = port_row(ui, state, port);
                    if row.clicked() {
                        reveal(state, port.occurrence.as_ref(), port.component_id);
                    }
                }
            }
            if unread > 0 {
                card_note(ui, &unread_source_note(unread, sources.len()));
            }
            if sources.is_empty() && !ports.iter().any(PlacedRfPort::is_read) {
                card_note(ui, &unread_port_note(ports.len()));
            }
            let collisions = duplicate_port_numbers(ports);
            if !collisions.is_empty() {
                card_note(ui, &duplicate_port_note(&collisions));
            }
        },
    );
}

/// What the card's chip says this design's excitations amount to.
///
/// An RF port is an excitation a plan can read, so a testbench whose every
/// excitation is a placed port is not a design with nothing placed on it —
/// which is what this chip said, in the flow this product is sharpest at.
///
/// Port readership decides the chip only when the ports are all there is. A
/// port is a Z0 termination in every analysis, loading the design exactly as a
/// resistor does, so an unread port beside sources that are driving is the
/// ordinary state of an RF testbench swept in the time domain rather than a
/// finding. The one design where nothing reading the ports is the finding is
/// the one with nothing else to read.
fn verdict(sources: &[PlacedSource], ports: &[PlacedRfPort]) -> (&'static str, Tone) {
    let unread = sources.iter().filter(|source| !source.is_read()).count();
    if sources.is_empty() && ports.is_empty() {
        ("no sources placed", Tone::Warn)
    } else if ports.iter().any(PlacedRfPort::is_read) && !duplicate_port_numbers(ports).is_empty() {
        // Only once something indexes them. Two ports sharing a number is a
        // defect the moment a run addresses one by number, and until then it is
        // a bench still being drawn — flagging it before there is an `.sp` to
        // confuse would fire on every second port the moment it is placed.
        ("ports share a number", Tone::Warn)
    } else if unread > 0 {
        ("sources with no reader", Tone::Warn)
    } else if !sources.is_empty() {
        ("every source is read", Tone::Ok)
    } else if ports.iter().any(PlacedRfPort::is_read) {
        ("S-parameter ports drive this design", Tone::Ok)
    } else {
        ("ports with no S-parameter run", Tone::Warn)
    }
}

/// What the page says about the sources no enabled analysis reads.
///
/// The verb follows the count, not the noun it was attached to: "1 of 3
/// sources are read by no enabled analysis" is the sentence a reader stops
/// trusting before they reach the finding it carries. The closing clause
/// follows the same count, because one source is an "it".
fn unread_source_note(unread: usize, total: usize) -> String {
    let (verb, subject, netlisted) = if unread == 1 {
        ("is", "It is", "drives")
    } else {
        ("are", "They are", "drive")
    };
    format!(
        "{unread} of {total} sources {verb} read by no enabled analysis in this plan \u{2014} \
         named by none, and with no enabled analysis that reads every source. A disabled \
         instance that names one is listed on its row and does not count. {subject} still \
         netlisted and still {netlisted} the circuit."
    )
}

/// What the page says about a design whose only excitations are RF ports that
/// nothing reads.
///
/// The verb follows the count, as it does in [`unread_source_note`]. The note
/// states what the ports still are rather than only what they are not: a
/// terminator no run indexes is doing half its job, and a reader who has just
/// been told nothing reads it needs to know the other half is unaffected.
fn unread_port_note(ports: usize) -> String {
    let (subject, verb) = if ports == 1 {
        ("this port", "It is")
    } else {
        ("these ports", "They are")
    };
    format!(
        "No enabled S-parameter analysis in this plan reads {subject} \u{2014} and no other \
         analysis addresses a port by number. {verb} still netlisted, and still terminating the \
         design into the reference impedance each one declares."
    )
}

/// What the page says when two placed ports claim one port number.
///
/// The numbers are named rather than counted: the reader's next action is to
/// open the ports carrying them, and a note saying "2 collisions" sends them
/// through every port on the sheet to find which. Which port wins, and whether
/// the run is refused, is the dispatching surface's answer to give — this only
/// states that the design asked one question twice.
fn duplicate_port_note(collisions: &[u32]) -> String {
    let numbers: Vec<String> = collisions.iter().map(u32::to_string).collect();
    let (subject, verb) = if collisions.len() == 1 {
        ("Port number", "is claimed")
    } else {
        ("Port numbers", "are claimed")
    };
    format!(
        "{subject} {} {verb} by more than one placed port. An S-parameter run \
         addresses a port by its number, so the ports sharing one cannot both be \
         the port that run measures.",
        numbers.join(", ")
    )
}

/// One source's row. The reader column carries the finding, so it is the only
/// cell that takes a tone.
fn excitation_row(ui: &mut Ui, state: &AppState, source: &PlacedSource) -> egui::Response {
    let (readers, tone) = readers_cell(&source.consumers, ("no reader", Tone::Warn));
    let terminals = source.nets.join(" \u{2192} ");
    let selected = reveals(state, source.occurrence.as_ref(), source.component_id);
    let summary = source.summary();
    let occurrence = source.occurrence_label();
    ledger_row(
        ui,
        &EXCITATION_COLUMNS,
        &[
            (source.reference.as_str(), Tone::Neutral),
            (source.quantity(), Tone::Accent),
            (occurrence.as_str(), Tone::Neutral),
            (summary.as_str(), Tone::Neutral),
            (terminals.as_str(), Tone::Neutral),
            (readers.as_str(), tone),
        ],
        selected,
    )
    .on_hover_text(row_tooltip(
        &format!(
            "{occurrence} \u{00b7} {} \u{00b7} {summary}",
            source.reference
        ),
        &source.consumers,
        "No analysis in this plan names this source, and none reads every source",
        elsewhere(state, source.occurrence.as_ref()),
    ))
}

/// One RF port's row, in the columns the source rows above it use.
///
/// The unread cell is stated rather than flagged. A port no `.sp` run indexes
/// is still terminating the design, which is not the case a `no reader` warning
/// was written for, and painting every termination in a time-domain testbench
/// as a finding is how a page stops being read.
fn port_row(ui: &mut Ui, state: &AppState, port: &PlacedRfPort) -> egui::Response {
    let (readers, tone) = readers_cell(&port.consumers, ("no S-parameter run", Tone::Neutral));
    let terminals = port.nets.join(" \u{2192} ");
    let selected = reveals(state, port.occurrence.as_ref(), port.component_id);
    let summary = port.summary();
    let occurrence = port.occurrence_label();
    ledger_row(
        ui,
        &EXCITATION_COLUMNS,
        &[
            (port.reference.as_str(), Tone::Neutral),
            (port.quantity(), Tone::Accent),
            (occurrence.as_str(), Tone::Neutral),
            (summary.as_str(), Tone::Neutral),
            (terminals.as_str(), Tone::Neutral),
            (readers.as_str(), tone),
        ],
        selected,
    )
    .on_hover_text(row_tooltip(
        &format!(
            "{occurrence} \u{00b7} {} \u{00b7} port {} \u{00b7} {summary}",
            port.reference, port.port_number
        ),
        &port.consumers,
        "No S-parameter analysis in this plan reads this port",
        elsewhere(state, port.occurrence.as_ref()),
    ))
}

/// The occurrence a row is drawn in when that is not the one on screen.
///
/// `None` means the row's instance is in the buffer in front of the reader, so
/// the page's own select-and-centre transaction reaches it.
fn elsewhere<'a>(
    state: &AppState,
    occurrence: Option<&'a InstancePath>,
) -> Option<&'a InstancePath> {
    occurrence.filter(|occurrence| **occurrence != state.workspace.occurrence_path())
}

/// Whether the drawing on screen is showing this row's instance selected.
///
/// The occurrence is part of the question now that the list crosses masters: a
/// component id is unique inside one buffer and repeats across them, so a row
/// naming an instance of a child master would otherwise paint itself selected
/// whenever the sheet on screen happened to hold that id.
fn reveals(state: &AppState, occurrence: Option<&InstancePath>, component_id: u64) -> bool {
    elsewhere(state, occurrence).is_none() && state.schematic.selection.has_component(component_id)
}

/// The `Read by` cell: who reads this row, and whether that is a finding.
///
/// Only the instances the run contains are counted. A disabled one is named in
/// the tooltip, marked disabled, because it is the thing a reader re-enables to
/// change this answer — and it still shows here, because a plan holding a
/// disabled reader is a different situation from one holding none.
///
/// `nothing_reads` is the one part the two row kinds disagree on: a source no
/// analysis reads is a finding, and a port no analysis reads is a termination.
fn readers_cell(
    consumers: &[SourceConsumer],
    nothing_reads: (&'static str, Tone),
) -> (String, Tone) {
    let reading: Vec<&SourceConsumer> = consumers
        .iter()
        .filter(|consumer| consumer.reads())
        .collect();
    let disabled = consumers.len() - reading.len();
    match reading.len() {
        0 if disabled > 0 => (
            format!("no reader \u{00b7} {disabled} disabled"),
            Tone::Warn,
        ),
        0 => (nothing_reads.0.to_owned(), nothing_reads.1),
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
    }
}

/// Everything the row had to shorten: every reader, the part it plays, and
/// what the click does.
///
/// The closing line follows the occurrence. A row on the sheet in front of the
/// reader is selected by clicking it; a row inside a child master is reached by
/// descending to that occurrence first, which the Design navigator's excitation
/// rail does — so the line names the occurrence rather than promising a
/// selection this page cannot make.
fn row_tooltip(
    identity: &str,
    consumers: &[SourceConsumer],
    nothing_reads: &str,
    elsewhere: Option<&InstancePath>,
) -> String {
    let mut lines = vec![identity.to_owned()];
    if consumers.is_empty() {
        lines.push(nothing_reads.to_owned());
    } else {
        for consumer in consumers {
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
    lines.push(elsewhere.map_or_else(
        || "Click to select it on the schematic".to_owned(),
        |occurrence| {
            format!(
                "Drawn inside {occurrence} \u{2014} open that occurrence from the design \
                 navigator's Excitations rail to select it"
            )
        },
    ));
    lines.join("\n")
}

/// Select the instance and centre the drawing on it, then show the drawing.
///
/// The same select-and-centre transaction the result viewers use to reach a
/// device, because arriving at a selected-but-offscreen instance is the one
/// outcome that reads as a broken link.
///
/// A row naming an instance of another occurrence shows the drawing and stops
/// there. Selecting is a transaction against the buffer on screen, and a
/// component id is unique only inside one — running it for a row of a child
/// master would select whatever instance of the sheet in front of the reader
/// happened to carry that id. Descending to the owning occurrence first is the
/// Design navigator's own excitation rail, which lists the same row and lands
/// on it; the tooltip here says which occurrence to look in.
fn reveal(state: &mut AppState, occurrence: Option<&InstancePath>, component_id: u64) {
    state.workbench.activate(Workspace::Design);
    if occurrence.is_some_and(|occurrence| *occurrence != state.workspace.occurrence_path()) {
        return;
    }
    let position = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .map(|component| component.pos);
    state
        .schematic
        .selection
        .select_only_component(component_id);
    state.schematic.net_highlight.clear();
    state.schematic.center_request = position;
}

#[cfg(test)]
mod tests {
    use super::{Tone, duplicate_port_note, unread_port_note, unread_source_note, verdict};
    use crate::simulation::placed_sources::{placed_rf_ports, placed_sources};
    use crate::simulation::plan::{AnalysisKind, SimulationPlan};
    use crate::state::{Component, ComponentType, Point, SchematicState};

    fn schematic_with(components: Vec<Component>) -> SchematicState {
        let mut schematic = SchematicState::default();
        schematic.components = components;
        schematic
    }

    fn rf_port(id: u64, name: &str, params: &str) -> Component {
        let mut component =
            Component::new(id, ComponentType::RfPort, Point::origin()).with_name_value(name, "");
        component.params = params.to_owned();
        component
    }

    fn plan_with(kind: AnalysisKind, enabled: bool) -> SimulationPlan {
        let mut plan = SimulationPlan::empty();
        let (instance, _) = plan.insert(kind).expect("the fixture analysis inserts");
        plan.set_enabled(instance, enabled)
            .expect("the fixture analysis takes its enabled flag");
        plan
    }

    /// The finding agrees with the number in front of it.
    ///
    /// The note read "1 of 3 sources are read by no enabled analysis", which is
    /// the sentence a reader stops trusting before they reach the finding it
    /// carries — and the finding is the whole reason the note is on the page.
    #[test]
    fn the_unread_source_note_agrees_with_its_own_count() {
        let one = unread_source_note(1, 3);
        assert!(one.starts_with("1 of 3 sources is read by"), "{one}");
        assert!(one.contains("It is still netlisted"), "{one}");
        assert!(one.contains("still drives the circuit"), "{one}");

        let many = unread_source_note(2, 3);
        assert!(many.starts_with("2 of 3 sources are read by"), "{many}");
        assert!(many.contains("They are still netlisted"), "{many}");
        assert!(many.contains("still drive the circuit"), "{many}");
    }

    #[test]
    fn the_unread_port_note_agrees_with_its_own_count() {
        let one = unread_port_note(1);
        assert!(one.contains("reads this port"), "{one}");
        assert!(one.contains("It is still netlisted"), "{one}");

        let many = unread_port_note(3);
        assert!(many.contains("reads these ports"), "{many}");
        assert!(many.contains("They are still netlisted"), "{many}");
    }

    /// The chip this lane exists for.
    ///
    /// An S-parameter testbench places its excitations as RF ports, and this
    /// page told the reader the design places no sources at all — a warning, on
    /// the correct setup of the flow the product is sharpest at.
    #[test]
    fn an_s_parameter_bench_driven_by_ports_reads_as_driven() {
        let schematic =
            schematic_with(vec![rf_port(1, "P1", "port=1"), rf_port(2, "P2", "port=2")]);
        let ports = placed_rf_ports(&schematic, Some(&plan_with(AnalysisKind::SParameter, true)));
        assert_eq!(
            verdict(&[], &ports),
            ("S-parameter ports drive this design", Tone::Ok)
        );
    }

    /// The note names the numbers, because they are what the reader opens next.
    #[test]
    fn the_duplicate_port_note_names_the_numbers_it_found() {
        let one = duplicate_port_note(&[2]);
        assert!(one.starts_with("Port number 2 is claimed"), "{one}");

        let many = duplicate_port_note(&[1, 3]);
        assert!(many.starts_with("Port numbers 1, 3 are claimed"), "{many}");
    }

    /// A collision is a finding once something indexes the ports, and a bench
    /// still being drawn is not.
    #[test]
    fn colliding_port_numbers_are_a_finding_only_once_a_run_reads_them() {
        let schematic =
            schematic_with(vec![rf_port(1, "P1", "port=1"), rf_port(2, "P2", "port=1")]);

        let read = placed_rf_ports(&schematic, Some(&plan_with(AnalysisKind::SParameter, true)));
        assert_eq!(verdict(&[], &read), ("ports share a number", Tone::Warn));

        let unread = placed_rf_ports(&schematic, None);
        assert_eq!(
            verdict(&[], &unread),
            ("ports with no S-parameter run", Tone::Warn),
            "with nothing indexing them the bench is unfinished, not miswired"
        );
    }

    /// A design with nothing on it still says so, in the words it always used.
    #[test]
    fn a_design_that_places_nothing_still_states_that_it_places_nothing() {
        assert_eq!(verdict(&[], &[]), ("no sources placed", Tone::Warn));
    }

    /// Ports and no run that reads them is the one finding a ports-only bench
    /// can carry, and a disabled `.sp` is the same finding: the run this plan
    /// would dispatch does not contain it.
    #[test]
    fn ports_are_a_finding_when_nothing_reads_them_and_they_are_all_there_is() {
        let schematic = schematic_with(vec![rf_port(1, "P1", "port=1")]);
        for plan in [
            None,
            Some(plan_with(AnalysisKind::SParameter, false)),
            Some(plan_with(AnalysisKind::Transient, true)),
        ] {
            let ports = placed_rf_ports(&schematic, plan.as_ref());
            assert_eq!(
                verdict(&[], &ports),
                ("ports with no S-parameter run", Tone::Warn),
                "{plan:?}"
            );
        }
    }

    /// The source verdicts are unchanged by any of this, including beside a
    /// port that nothing reads: a termination in a transient testbench is the
    /// ordinary state, not a finding.
    #[test]
    fn the_source_verdicts_are_what_they_were() {
        let schematic = schematic_with(vec![
            Component::new(1, ComponentType::VoltageSourcePulse, Point::origin())
                .with_name_value("V1", "0"),
            rf_port(2, "P1", "port=1"),
        ]);
        let transient = plan_with(AnalysisKind::Transient, true);
        let sources = placed_sources(&schematic, Some(&transient));
        let ports = placed_rf_ports(&schematic, Some(&transient));
        assert_eq!(
            verdict(&sources, &ports),
            ("every source is read", Tone::Ok)
        );

        let ac_only = plan_with(AnalysisKind::Ac, true);
        let sources = placed_sources(&schematic, Some(&ac_only));
        let ports = placed_rf_ports(&schematic, Some(&ac_only));
        assert_eq!(
            verdict(&sources, &ports),
            ("sources with no reader", Tone::Warn),
            "a PULSE carries no AC magnitude, so an AC-only plan reads it not at all"
        );
    }
}
