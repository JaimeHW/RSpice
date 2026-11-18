//! Cross-probing from the netlist editor: what the name under the caret is,
//! as the engine's index of the shown text reads it, and where the line it
//! sits on came from in the schematic.

use std::sync::Arc;

use rspice_core::netlist::{DeckIndex, DeckLocation, DeckSymbolKind};

use super::{ActiveNetlistDocument, Model, NetlistDocumentState};
use crate::state::{CellViewRef, NetlistDocument, NetlistSourceIndex};
use crate::workbench::app_state::cell_usage::CellInstanceUse;
use crate::workbench::documents::virtual_text_editor::SourceSpan;

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
