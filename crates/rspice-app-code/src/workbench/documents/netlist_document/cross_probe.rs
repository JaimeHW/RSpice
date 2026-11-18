//! Cross-probing from the netlist editor: what the name under the caret is,
//! as the engine's index of the shown text reads it, and where the line it
//! sits on came from in the schematic.

use std::sync::Arc;

use rspice_core::netlist::{DeckIndex, DeckLocation, DeckSymbolKind};

use super::{ActiveNetlistDocument, Model, NetlistDocumentState};
use crate::state::{
    AnalysisResultSourceDomain, CellViewRef, NetlistDocument, NetlistSourceIndex, SimulationRun,
};
use crate::workbench::app_state::cell_usage::CellInstanceUse;
use crate::workbench::documents::virtual_text_editor::SourceSpan;
use rspice_app_runs::workbench::slots::runs::RunsState;

/// What the name under the caret is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaretName {
    /// A node, as the deck writes it. `top_level` when it is a node of the
    /// deck itself rather than of a subcircuit's body.
    Node { name: String, top_level: bool },
    /// An element: the name its own card is headed by (`head`), or a read of
    /// it on another card, such as the inductors a `K` card couples.
    Element {
        name: String,
        top_level: bool,
        head: bool,
    },
}

/// Where the caret stood in which document when its name was last read.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CaretKey {
    document: ActiveNetlistDocument,
    dependency: Option<String>,
    char_index: usize,
}

impl CaretKey {
    fn of(netlist: &NetlistDocumentState) -> Self {
        Self {
            document: netlist.active_document,
            dependency: netlist.active_dependency_identity.clone(),
            char_index: netlist.cursor_char_index,
        }
    }
}

/// The name under the caret, read once per caret position and text, so the
/// editor's menu and the commands answering for it read a lookup rather than
/// the deck. It keeps the index it was read from, and holds only while that
/// index describes the text as it stands: text replaced under a caret that
/// did not move is read again.
#[derive(Debug, Clone, Default)]
pub struct CaretNameCache {
    key: Option<CaretKey>,
    index: Option<Arc<NetlistSourceIndex>>,
    name: Option<CaretName>,
}

impl CaretNameCache {
    fn holds(&self, netlist: &NetlistDocumentState, source: &str) -> bool {
        self.key.as_ref() == Some(&CaretKey::of(netlist))
            && self
                .index
                .as_ref()
                .is_some_and(|index| index.describes(source))
    }
}

/// Read the name under the caret again once the caret or the text has moved,
/// over the frame's index of the shown text. `line` and `column` are where
/// the editor placed the caret, zero-based. While the writer types, the
/// frame's index may trail the text by a build; a name read from it is not
/// kept, so the next frame reads again.
pub(super) fn follow_caret(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
    line: usize,
    column: usize,
) {
    let source = model.simulation.source.netlist_content.as_str();
    if netlist.caret_name.holds(netlist, source) {
        return;
    }
    let index = super::visible_source_index_in(ctx, model, netlist);
    netlist.caret_name = if index.describes(source) {
        CaretNameCache {
            key: Some(CaretKey::of(netlist)),
            name: name_on(index.deck(), line + 1, column),
            index: Some(index),
        }
    } else {
        CaretNameCache::default()
    };
}

/// The name under the caret, when a frame has read it for the caret and the
/// text as they stand; `None` when nothing is there or it has not been read.
pub(crate) fn caret_name<'a>(
    model: &Model,
    netlist: &'a NetlistDocumentState,
) -> Option<&'a CaretName> {
    let cache = &netlist.caret_name;
    cache
        .holds(netlist, &model.simulation.source.netlist_content)
        .then_some(cache.name.as_ref())
        .flatten()
}

/// The name under the caret as the text stands, read now when no frame has
/// read it for this caret: what a command acts on.
pub(crate) fn caret_name_now(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<CaretName> {
    let source = model.simulation.source.netlist_content.as_str();
    if netlist.caret_name.holds(netlist, source) {
        return netlist.caret_name.name.clone();
    }
    let (line, column) = line_and_column(source, netlist.cursor_char_index);
    let index = super::visible_source_index_now(model, netlist);
    let name = name_on(index.deck(), line + 1, column);
    netlist.caret_name = CaretNameCache {
        key: Some(CaretKey::of(netlist)),
        name: name.clone(),
        index: Some(index),
    };
    name
}

/// The zero-based line and character column of character `char_index` of
/// `source`.
fn line_and_column(source: &str, char_index: usize) -> (usize, usize) {
    source
        .chars()
        .take(char_index)
        .fold((0, 0), |(line, column), character| {
            if character == '\n' {
                (line + 1, 0)
            } else {
                (line, column + 1)
            }
        })
}

/// The name the index has at one-based `line` of the shown text, touching
/// the caret at zero-based character `column`: a node first, then an
/// element's own name, then an element read on another card.
fn name_on(deck: &DeckIndex, line: usize, column: usize) -> Option<CaretName> {
    let caret = column + 1;
    // The shown text is the index's first file; a name touches the caret
    // when the caret stands in it or at either end of it.
    let touches = |location: &DeckLocation, name: &str| {
        location.file == 0
            && location.line == line
            && location.column <= caret
            && caret <= location.column + name.chars().count()
    };
    if let Some(node) = deck
        .nodes()
        .iter()
        .find(|node| touches(&node.location, &node.name))
    {
        return Some(CaretName::Node {
            name: node.name.clone(),
            top_level: node.scope == 0,
        });
    }
    if let Some(symbol) = deck.symbols().iter().find(|symbol| {
        symbol.kind == DeckSymbolKind::Instance && touches(&symbol.location, &symbol.name)
    }) {
        return Some(CaretName::Element {
            name: symbol.name.clone(),
            top_level: symbol.scope == 0,
            head: true,
        });
    }
    deck.references()
        .iter()
        .find(|reference| {
            reference.kind == DeckSymbolKind::Instance
                && touches(&reference.location, &reference.name)
        })
        .map(|reference| CaretName::Element {
            name: reference.name.clone(),
            top_level: reference.scope == 0,
            head: false,
        })
}

/// Where on the schematic a line of the generated netlist came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchematicTarget {
    /// The instance the line is the card of.
    Instance(CellInstanceUse),
    /// A net the line's card connects, in the cell view the line belongs to.
    Net { schematic: CellViewRef, net: String },
}

/// Why a line of the generated netlist has nowhere on the schematic to show.
pub const NO_SCHEMATIC_SOURCE: &str = "this line has no schematic source";

/// Whether the editor shows the generated netlist itself, rather than an
/// include it reads, a comparison or a deck.
pub(crate) fn generated_netlist_in_front(netlist: &NetlistDocumentState) -> bool {
    netlist.active_document == ActiveNetlistDocument::Generated
        && netlist.active_dependency_identity.is_none()
}

/// Where the generated line under the caret came from: the net under the
/// caret when the line's card connects it, else the instance the line is
/// the card of. The generator recorded both as it wrote the line; a line it
/// wrote for no instance (a comment, an analysis, an added line, `.end`) has
/// nowhere to go.
pub(crate) fn schematic_target(
    model: &Model,
    netlist: &NetlistDocumentState,
    name: Option<&CaretName>,
) -> Result<SchematicTarget, &'static str> {
    let entry = generated_artifact(model)
        .and_then(|artifact| artifact.source_map_entry(netlist.cursor_line + 1))
        .ok_or(NO_SCHEMATIC_SOURCE)?;
    let schematic = CellViewRef::from_key(entry.view_identity()).ok_or(NO_SCHEMATIC_SOURCE)?;
    if let Some(CaretName::Node { name, .. }) = name
        && let Some(net) = entry
            .nets()
            .iter()
            .find(|net| net.eq_ignore_ascii_case(name))
    {
        return Ok(SchematicTarget::Net {
            schematic,
            net: net.clone(),
        });
    }
    let component_id = entry.component_id().ok_or(NO_SCHEMATIC_SOURCE)?;
    Ok(SchematicTarget::Instance(CellInstanceUse {
        schematic,
        instance: entry.instance_identity().unwrap_or_default().to_owned(),
        component_id,
    }))
}

/// Which quantity a plot from the editor asks the run for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlotQuantity {
    /// `V(node)`.
    Voltage,
    /// `I(device)`: the terminal current the probe tools plot.
    Current,
}

impl PlotQuantity {
    /// The vector a run keeps the quantity of `name` under.
    pub fn vector(self, name: &str) -> String {
        match self {
            Self::Voltage => format!("V({name})"),
            Self::Current => format!("I({name})"),
        }
    }

    /// Whether a run keeps the quantity of `name` under `vector`: a voltage
    /// under `V(name)`, or, as the engine names a deck's nodes, `name`
    /// itself; a current under `I(name)`.
    fn names(self, vector: &str, name: &str) -> bool {
        vector.eq_ignore_ascii_case(&self.vector(name))
            || (self == Self::Voltage && vector.eq_ignore_ascii_case(name))
    }

    /// The name under the caret this quantity is of, when it is one. A
    /// subcircuit's instance (`X...`) has a current at each of its pins and
    /// none of its own, so it is no device to plot the current of.
    pub fn of(self, name: &CaretName) -> Option<(&str, bool)> {
        match (self, name) {
            (Self::Voltage, CaretName::Node { name, top_level }) => {
                Some((name.as_str(), *top_level))
            }
            (
                Self::Current,
                CaretName::Element {
                    name, top_level, ..
                },
            ) if !name.starts_with(['X', 'x']) => Some((name.as_str(), *top_level)),
            _ => None,
        }
    }
}

/// Why a plot has nothing to plot: the caret is on no node, or no device.
pub const NOT_A_NAME: &str = "the caret is not on a name of this kind";

/// The trace one run holds for a plot from the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlotTarget {
    /// The run's display sequence.
    pub run: u64,
    pub analysis: usize,
    pub waveform: usize,
}

/// Whether the editor shows a document a run can be of: the generated
/// netlist, a deck, or the deck a run executed, each itself rather than an
/// include it reads.
pub(crate) fn plottable_document_in_front(netlist: &NetlistDocumentState) -> bool {
    netlist.active_dependency_identity.is_none()
        && matches!(
            netlist.active_document,
            ActiveNetlistDocument::Generated
                | ActiveNetlistDocument::OwnedSource
                | ActiveNetlistDocument::RunSnapshot
        )
}

/// The finished run of the document in front whose results a plot reads.
/// For the generated netlist that is a run of the design: the one Results
/// shows when it is one, else the newest. For a deck it is the deck's last
/// run, and for the deck a run executed, that run.
fn document_run<'a>(
    model: &'a Model,
    netlist: &NetlistDocumentState,
    runs: &RunsState,
) -> Result<&'a SimulationRun, &'static str> {
    let retained = &model.simulation.retained;
    let kept = |run: Option<&'a SimulationRun>, reason| {
        run.filter(|run| !run.analyses.is_empty()).ok_or(reason)
    };
    match netlist.active_document {
        ActiveNetlistDocument::Generated => {
            let of_design = |run: &&SimulationRun| {
                !run.analyses.is_empty()
                    && run.prepared_receipt().is_some_and(|receipt| {
                        receipt.source_domain() == AnalysisResultSourceDomain::SimulationPlan
                    })
            };
            model
                .simulation
                .active_run()
                .filter(of_design)
                .or_else(|| {
                    retained
                        .runs
                        .iter()
                        .filter(of_design)
                        .max_by_key(|run| run.id)
                })
                .ok_or("the design has not been run")
        }
        ActiveNetlistDocument::OwnedSource => {
            if runs.manual_run.pending_manual_run_id.is_some() {
                return Err("the deck is still running");
            }
            let run = runs
                .manual_run
                .last_run_id
                .ok_or("the deck has not been run")?;
            kept(
                retained.run_by_sequence(run),
                "the deck's run is no longer kept",
            )
        }
        ActiveNetlistDocument::RunSnapshot => {
            let run = netlist
                .executed_deck_view
                .map(|selection| selection.run_id)
                .or(runs.manual_run.last_run_id)
                .ok_or("the run is no longer kept")?;
            kept(retained.run_by_sequence(run), "the run is no longer kept")
        }
        ActiveNetlistDocument::GeneratedDiff => Err("a comparison is not a run's input"),
    }
}

/// The trace the run of the document in front holds for `quantity` of the
/// name under the caret, or why there is none to plot.
pub(crate) fn plot_target(
    model: &Model,
    netlist: &NetlistDocumentState,
    runs: &RunsState,
    quantity: PlotQuantity,
    name: &CaretName,
) -> Result<PlotTarget, &'static str> {
    let (name, top_level) = quantity.of(name).ok_or(NOT_A_NAME)?;
    if !top_level {
        return Err(match quantity {
            PlotQuantity::Voltage => "a node inside a subcircuit has a voltage for each instance",
            PlotQuantity::Current => "a device inside a subcircuit has a current for each instance",
        });
    }
    if quantity == PlotQuantity::Voltage && name == "0" {
        return Err("node 0 is the ground reference");
    }
    let run = document_run(model, netlist, runs)?;
    let find = |analysis: usize| {
        run.analyses.get(analysis).and_then(|result| {
            result
                .waveforms
                .iter()
                .position(|waveform| quantity.names(&waveform.name, name))
                .map(|waveform| (analysis, waveform))
        })
    };
    // The analysis Results shows outranks the run's others.
    let shown = model
        .simulation
        .active_run()
        .is_some_and(|active| active.id == run.id)
        .then_some(model.simulation.view.active_analysis_idx)
        .flatten();
    let (analysis, waveform) = shown
        .and_then(find)
        .or_else(|| (0..run.analyses.len()).find_map(find))
        .ok_or(match quantity {
            PlotQuantity::Voltage => "the run saved no voltage for this node",
            PlotQuantity::Current => "the run saved no current for this device",
        })?;
    Ok(PlotTarget {
        run: run.id,
        analysis,
        waveform,
    })
}

/// Where the generated netlist writes the net last shown from the
/// schematic, kept with the generated text it was read from.
#[derive(Debug, Clone, Default)]
pub struct NetMarks {
    generated: Option<crate::product::ContentDigest>,
    spans: Vec<SourceSpan>,
}

/// Mark each place the generated netlist writes `net` on its one-based
/// `lines`, the lines of the net's own cell, so the editor shows every use
/// around the one the caret lands on.
pub fn mark_net_in_generated_netlist(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    net: &str,
    lines: &[usize],
) {
    let Some(artifact) = generated_artifact(model) else {
        netlist.net_marks = NetMarks::default();
        return;
    };
    let index = NetlistSourceIndex::parse(artifact.source());
    let mut spans = index
        .deck()
        .nodes()
        .iter()
        .filter(|node| {
            node.location.file == 0
                && lines.contains(&node.location.line)
                && node.name.eq_ignore_ascii_case(net)
        })
        .map(|node| {
            let start = node.location.column - 1;
            SourceSpan {
                line: node.location.line - 1,
                columns: start..start + node.name.chars().count(),
            }
        })
        .collect::<Vec<_>>();
    spans.sort_by_key(|span| (span.line, span.columns.start));
    netlist.net_marks = NetMarks {
        generated: Some(artifact.content_digest()),
        spans,
    };
}

/// Forget the marks of the net last shown.
pub fn clear_net_marks(netlist: &mut NetlistDocumentState) {
    netlist.net_marks = NetMarks::default();
}

/// The marks to draw: while the generated netlist they were read from is
/// the text in front, and none otherwise.
pub(super) fn net_marks<'a>(model: &Model, netlist: &'a NetlistDocumentState) -> &'a [SourceSpan] {
    let current = generated_artifact(model).map(|artifact| artifact.content_digest());
    if generated_netlist_in_front(netlist)
        && current.is_some()
        && netlist.net_marks.generated == current
    {
        &netlist.net_marks.spans
    } else {
        &[]
    }
}

/// The generated netlist the project keeps.
fn generated_artifact(model: &Model) -> Option<&crate::state::GeneratedArtifact> {
    model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .and_then(NetlistDocument::generated_artifact)
}

#[cfg(test)]
mod tests;
