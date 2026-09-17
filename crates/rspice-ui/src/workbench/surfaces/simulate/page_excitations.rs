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
use crate::state::stimulus_library::provenance::ProvenanceState;
use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;

use super::page_kit;
use super::page_kit::{RowPress, Tone, card, card_note, ledger_head, ledger_row};

/// Reference, quantity, the occurrence it is reached through, waveform,
/// terminals, what reads it, which stimulus definition it copied, and the
/// row's own verbs.
///
/// The occurrence sits beside the reference because it qualifies it: `V1` is
/// not a name in a hierarchical design until the path in front of it is read,
/// and two rows can carry one reference.
///
/// The Definition column is last before the verbs because it is the one cell a
/// reader acts on: what it says and what the button beside it offers are the
/// same subject, and splitting them across the table would put the verb a
/// column away from the fact that justifies it.
const EXCITATION_COLUMNS: [f32; 8] = [0.11, 0.04, 0.11, 0.16, 0.14, 0.18, 0.18, 0.08];

/// The head labels of a source block. The quantity and the verb columns are
/// unlabelled: one carries a single letter the row itself explains, and the
/// other carries a control that names itself.
const EXCITATION_HEAD: [&str; 8] = [
    "Reference",
    "",
    "Occurrence",
    "Waveform",
    "Terminals",
    "Read by",
    "Definition",
    "",
];

/// The head labels of the RF-port block. A port's third cell states what the
/// port is and the impedance it presents, which is not a waveform, and a port
/// adopts no stimulus definition.
const PORT_HEAD: [&str; 8] = [
    "Reference",
    "",
    "Occurrence",
    "Port",
    "Terminals",
    "Read by",
    "",
    "",
];

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
    let (verdict, verdict_tone) = verdict(sources, ports);

    card(
        ui,
        "Placed excitations",
        Some((verdict.as_str(), verdict_tone)),
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
                ledger_head(ui, &EXCITATION_COLUMNS, &EXCITATION_HEAD);
                let mut taken = None;
                for source in sources {
                    let row = excitation_row(ui, state, source);
                    if let Some(verb) = source_row_verbs(ui, state, source, row.rect) {
                        taken = Some((source.component_id, source.occurrence.clone(), verb));
                    } else if row.clicked() {
                        reveal(state, source.occurrence.as_ref(), source.component_id);
                    }
                }
                if let Some((component_id, occurrence, verb)) = taken {
                    take_source_verb(state, component_id, occurrence.as_ref(), &verb);
                }
            }
            // A second head rather than a group caption: a port's third cell
            // states what the port is and the impedance it presents, which is
            // not a waveform, and a column label that covered both would name
            // neither.
            if !ports.is_empty() {
                ledger_head(ui, &EXCITATION_COLUMNS, &PORT_HEAD);
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
fn verdict(sources: &[PlacedSource], ports: &[PlacedRfPort]) -> (String, Tone) {
    let unread = sources.iter().filter(|source| !source.is_read()).count();
    let adopted = sources
        .iter()
        .filter(|source| source.provenance != ProvenanceState::FromSchematic)
        .count();
    let behind = sources
        .iter()
        .filter(|source| {
            matches!(
                source.provenance,
                ProvenanceState::Behind { .. } | ProvenanceState::ModifiedBehind { .. }
            )
        })
        .count();
    let stated = |text: &str, tone| (text.to_owned(), tone);
    if sources.is_empty() && ports.is_empty() {
        stated("no sources placed", Tone::Warn)
    } else if ports.iter().any(PlacedRfPort::is_read) && !duplicate_port_numbers(ports).is_empty() {
        // Only once something indexes them. Two ports sharing a number is a
        // defect the moment a run addresses one by number, and until then it is
        // a bench still being drawn — flagging it before there is an `.sp` to
        // confuse would fire on every second port the moment it is placed.
        stated("ports share a number", Tone::Warn)
    } else if behind > 0 {
        // The library has published past what these instances copied, and the
        // row that says so also carries the verb that fixes it. It outranks
        // the unread finding because an unread source still drives the
        // circuit, while a source behind its definition drives it with a card
        // the project no longer describes.
        (
            format!("{adopted} adopted \u{00b7} {behind} behind"),
            Tone::Warn,
        )
    } else if unread > 0 {
        stated("sources with no reader", Tone::Warn)
    } else if !sources.is_empty() {
        stated("every source is read", Tone::Ok)
    } else if ports.iter().any(PlacedRfPort::is_read) {
        stated("S-parameter ports drive this design", Tone::Ok)
    } else {
        stated("ports with no S-parameter run", Tone::Warn)
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
            (
                source.definition_cell.as_str(),
                definition_tone(source.provenance),
            ),
            ("", Tone::Neutral),
        ],
        selected,
        RowPress::Taken,
    )
    .on_hover_text(row_tooltip(
        &format!(
            "{occurrence} \u{00b7} {} \u{00b7} {summary} \u{00b7} {}",
            source.reference, source.definition_cell
        ),
        &source.consumers,
        "No analysis in this plan names this source, and none reads every source",
        elsewhere(state, source.occurrence.as_ref()),
    ))
}

/// What the Definition cell's colour says.
///
/// Only a state a reader has to act on takes a colour. `behind` and
/// `definition removed` are the library having moved out from under the
/// instance, which is a finding; `modified` is a deliberate local edit, which
/// is worth marking but is not a defect; everything else is the ordinary state
/// and stays in the metadata register with the cells beside it.
fn definition_tone(provenance: ProvenanceState) -> Tone {
    match provenance {
        ProvenanceState::Behind { .. }
        | ProvenanceState::ModifiedBehind { .. }
        | ProvenanceState::Removed { .. } => Tone::Warn,
        ProvenanceState::Modified { .. } => Tone::Accent,
        ProvenanceState::FromSchematic | ProvenanceState::Adopted { .. } => Tone::Neutral,
    }
}

/// One verb a source row offers.
#[derive(Debug, Clone, PartialEq, Eq)]
enum SourceVerb {
    /// Select the instance and open Component Properties on it.
    Properties,
    /// Show the named definition in the Stimulus Library workspace.
    OpenDefinition(String),
    /// Copy the library's current revision of the adopted definition back onto
    /// this instance, in one undo group.
    Readopt(String),
}

impl SourceVerb {
    /// The row a reader picks, in the words the menu shows.
    fn label(&self, provenance: ProvenanceState) -> String {
        match self {
            Self::Properties => "Properties".to_owned(),
            Self::OpenDefinition(_) => "Open in Stimulus Library".to_owned(),
            Self::Readopt(_) => match provenance {
                ProvenanceState::Behind { library, .. }
                | ProvenanceState::ModifiedBehind { library, .. } => {
                    format!("Re-adopt r{library}")
                }
                ProvenanceState::Modified { from } => format!("Re-adopt r{from}"),
                _ => "Re-adopt".to_owned(),
            },
        }
    }
}

/// The verbs this row offers, in the order they are listed.
///
/// Every source row offers Properties, because every source has an editor. The
/// other two are the provenance's own answers — a definition the library still
/// holds can be opened, and an instance that has drifted can be re-adopted —
/// so nothing here re-derives a lifecycle word.
fn source_verbs(source: &PlacedSource) -> Vec<SourceVerb> {
    let mut verbs = vec![SourceVerb::Properties];
    let held = source.definition.as_ref().filter(|_| {
        !matches!(source.provenance, ProvenanceState::Removed { .. })
            && source.provenance != ProvenanceState::FromSchematic
    });
    if let Some(definition) = held {
        verbs.push(SourceVerb::OpenDefinition(definition.clone()));
        if source.provenance.offers_readoption() {
            verbs.push(SourceVerb::Readopt(definition.clone()));
        }
    }
    verbs
}

/// Paint the row's trailing verb control and report the verb taken.
///
/// The control is placed over the row's own last column rather than in a
/// column of its own layout: the row owns its height and its selection, and a
/// second allocation would put the menu on a line of its own. Within a layer
/// the later widget wins the pointer, so the button takes the press and the
/// row keeps every other part of itself — the same order the run-set table's
/// per-row controls keep.
fn source_row_verbs(
    ui: &mut Ui,
    state: &AppState,
    source: &PlacedSource,
    row: egui::Rect,
) -> Option<SourceVerb> {
    let verbs = source_verbs(source);
    let cells = page_kit::column_rects(row, &EXCITATION_COLUMNS);
    let cell = *cells.last()?;
    let unreachable = elsewhere(state, source.occurrence.as_ref())
        .is_some()
        .then_some(
            "This instance is drawn inside another occurrence; descend to it from the design \
         navigator's Excitations rail first",
        );
    let choices: Vec<page_kit::PopupChoice> = verbs
        .iter()
        .map(|verb| page_kit::PopupChoice {
            label: verb.label(source.provenance),
            unavailable: match verb {
                SourceVerb::Properties | SourceVerb::Readopt(_) => unreachable,
                SourceVerb::OpenDefinition(_) => None,
            },
        })
        .collect();
    let salt = format!("excitation-verbs-{}", source.component_id);
    let mut child = page_kit::cell_ui(ui, cell);
    let taken = page_kit::command_popup(
        &mut child,
        &salt,
        crate::ui::widgets::Button::new("Actions")
            .ghost()
            .accessible_label(&source.reference),
        "This source offers no action here",
        &choices,
    )?;
    verbs.get(taken).cloned()
}

/// Run one row verb against the design.
fn take_source_verb(
    state: &mut AppState,
    component_id: u64,
    occurrence: Option<&InstancePath>,
    verb: &SourceVerb,
) {
    match verb {
        SourceVerb::Properties => {
            reveal(state, occurrence, component_id);
            crate::workbench::app::open_property_editor(state, component_id);
        }
        SourceVerb::OpenDefinition(definition) => {
            crate::workbench::app::open_stimulus_definition(state, definition);
        }
        SourceVerb::Readopt(definition) => {
            reveal(state, occurrence, component_id);
            let outcome = crate::workbench::app::commit_readoption(state, component_id, definition);
            state.push_user_message(match outcome {
                Ok(line) => crate::diagnostics::ConsoleMessage::info(line),
                Err(refusal) => crate::diagnostics::ConsoleMessage::warning(refusal),
            });
        }
    }
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
            ("", Tone::Neutral),
            ("", Tone::Neutral),
        ],
        selected,
        RowPress::Taken,
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
    use super::{
        ProvenanceState, SourceVerb, Tone, definition_tone, duplicate_port_note, source_verbs,
        unread_port_note, unread_source_note, verdict,
    };
    use crate::simulation::placed_sources::{placed_rf_ports, placed_sources};
    use crate::simulation::plan::{AnalysisKind, SimulationPlan};
    use crate::state::stimulus_library::definition::StimulusDefinition;
    use crate::state::stimulus_library::draft::DefinitionDraft;
    use crate::state::{Component, ComponentType, Point, SchematicState, StimulusLibrary};

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
            ("S-parameter ports drive this design".to_owned(), Tone::Ok)
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
        assert_eq!(
            verdict(&[], &read),
            ("ports share a number".to_owned(), Tone::Warn)
        );

        let unread = placed_rf_ports(&schematic, None);
        assert_eq!(
            verdict(&[], &unread),
            ("ports with no S-parameter run".to_owned(), Tone::Warn),
            "with nothing indexing them the bench is unfinished, not miswired"
        );
    }

    /// A design with nothing on it still says so, in the words it always used.
    #[test]
    fn a_design_that_places_nothing_still_states_that_it_places_nothing() {
        assert_eq!(
            verdict(&[], &[]),
            ("no sources placed".to_owned(), Tone::Warn)
        );
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
                ("ports with no S-parameter run".to_owned(), Tone::Warn),
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
        let sources = placed_sources(&schematic, &StimulusLibrary::default(), Some(&transient));
        let ports = placed_rf_ports(&schematic, Some(&transient));
        assert_eq!(
            verdict(&sources, &ports),
            ("every source is read".to_owned(), Tone::Ok)
        );

        let ac_only = plan_with(AnalysisKind::Ac, true);
        let sources = placed_sources(&schematic, &StimulusLibrary::default(), Some(&ac_only));
        let ports = placed_rf_ports(&schematic, Some(&ac_only));
        assert_eq!(
            verdict(&sources, &ports),
            ("sources with no reader".to_owned(), Tone::Warn),
            "a PULSE carries no AC magnitude, so an AC-only plan reads it not at all"
        );
    }

    /// A design fixture whose one source has adopted `definition`, with the
    /// library holding `library_revision` of it.
    fn adopted_design(library_revision: u32) -> (SchematicState, StimulusLibrary) {
        let mut definition =
            StimulusDefinition::new("sensor_drive", ComponentType::VoltageSourceSin)
                .expect("definition");
        definition.value = "0".to_owned();
        definition.params = "va=3m freq=1k".to_owned();
        let mut component = Component::new(1, ComponentType::VoltageSourceSin, Point::origin())
            .with_name_value("V1", "0");
        definition
            .adopt_onto(&mut component)
            .expect("the fixture instance is of the definition's own type");
        let mut library = StimulusLibrary::default();
        library.insert(definition.clone()).expect("insert");
        // Published through the library itself: `apply` is the one verb that
        // advances a revision, so a fixture that moved the number some other
        // way would be testing a state the product cannot reach.
        let mut draft = DefinitionDraft::new(definition);
        for revision in 1..library_revision {
            draft.edit(|working| working.params = format!("va={revision}m freq=1k"));
            library.apply(&mut draft);
        }
        (schematic_with(vec![component]), library)
    }

    /// The Definition column states which definition an instance copied and
    /// where it stands, in the library's own words rather than the page's.
    #[test]
    fn the_definition_cell_is_the_provenance_state_the_library_reports() {
        let (schematic, library) = adopted_design(1);
        let sources = placed_sources(&schematic, &library, None);
        let source = sources.first().expect("the fixture places one source");
        assert_eq!(source.definition.as_deref(), Some("sensor_drive"));
        assert_eq!(source.provenance, ProvenanceState::Adopted { revision: 1 });
        assert_eq!(source.definition_cell, "sensor_drive · r1");
        assert_eq!(definition_tone(source.provenance), Tone::Neutral);

        let (schematic, library) = adopted_design(3);
        let sources = placed_sources(&schematic, &library, None);
        let source = sources.first().expect("the fixture places one source");
        assert_eq!(
            source.provenance,
            ProvenanceState::Behind {
                adopted: 1,
                library: 3
            }
        );
        assert_eq!(source.definition_cell, "sensor_drive · r1 · library r3");
        assert_eq!(definition_tone(source.provenance), Tone::Warn);
    }

    /// An instance drawn on the sheet adopted nothing, so its cell says so and
    /// its only verb is the editor every source has.
    #[test]
    fn a_source_that_adopted_nothing_offers_only_its_editor() {
        let schematic = schematic_with(vec![
            Component::new(1, ComponentType::VoltageSourcePulse, Point::origin())
                .with_name_value("V1", "0"),
        ]);
        let sources = placed_sources(&schematic, &StimulusLibrary::default(), None);
        let source = sources.first().expect("the fixture places one source");
        assert_eq!(source.provenance, ProvenanceState::FromSchematic);
        assert_eq!(source.definition_cell, "— · from schematic");
        assert_eq!(source_verbs(source), vec![SourceVerb::Properties]);
    }

    /// Re-adoption is offered exactly when the model says it would change
    /// something, and the verb names the revision it would copy.
    #[test]
    fn the_readopt_verb_appears_only_once_the_library_has_moved_past_the_copy() {
        let (schematic, library) = adopted_design(1);
        let current = placed_sources(&schematic, &library, None);
        let current = current.first().expect("one source");
        assert_eq!(
            source_verbs(current),
            vec![
                SourceVerb::Properties,
                SourceVerb::OpenDefinition("sensor_drive".to_owned())
            ],
            "an instance holding the library's own revision has nothing to re-adopt"
        );

        let (schematic, library) = adopted_design(4);
        let behind = placed_sources(&schematic, &library, None);
        let behind = behind.first().expect("one source");
        assert_eq!(
            source_verbs(behind),
            vec![
                SourceVerb::Properties,
                SourceVerb::OpenDefinition("sensor_drive".to_owned()),
                SourceVerb::Readopt("sensor_drive".to_owned())
            ]
        );
        assert_eq!(
            SourceVerb::Readopt("sensor_drive".to_owned()).label(behind.provenance),
            "Re-adopt r4"
        );
    }

    /// The chip counts the drift, because that is the finding a reader can act
    /// on from this page. An unread source still drives the circuit; a source
    /// behind its definition drives it with a card the project no longer holds.
    #[test]
    fn the_chip_counts_adopters_that_are_behind_their_definition() {
        let (schematic, library) = adopted_design(2);
        let sources = placed_sources(&schematic, &library, None);
        assert_eq!(
            verdict(&sources, &[]),
            ("1 adopted · 1 behind".to_owned(), Tone::Warn)
        );

        let (schematic, library) = adopted_design(1);
        let sources = placed_sources(&schematic, &library, None);
        assert_eq!(
            verdict(&sources, &[]),
            ("sources with no reader".to_owned(), Tone::Warn),
            "with nothing behind, the page states the finding it always stated"
        );
    }
}
