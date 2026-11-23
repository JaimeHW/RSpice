//! Netlist-first surfaces.
//!
//! A project opened from a deck has no schematic to draw, so the Design
//! surface is Design's own empty canvas, as when no document is open: it
//! says there is no schematic, names the deck the project runs, and offers
//! the two ways on, the deck in the Netlist workspace or a schematic made
//! here. Simulate has no plan to edit for it, and lists the deck's own setup
//! instead (see [`super::deck_setup`]). The deck's facts are the
//! inspector's, the same rows the Netlist workspace lists beside the deck.
//! These are the source feature's surface, drawn in that feature's context
//! while the deck is the project's simulation.

use egui::Ui;

use crate::state::{NetlistSourceIndex, OutlineEntryKind};
use crate::ui::widgets::Button;
use crate::workbench::app_state::Model;
use crate::workbench::code::SourceFeature;
use crate::workbench::design_system::{Fact, WorkbenchIcon, empty_state_with_actions, fact_rows};
use crate::workbench::documents::netlist_document::NetlistDocumentState;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{Cx, Execute};
use crate::workbench::framework::requests::ShowProjectDeck;
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::{MessageCatalog, MessageId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetlistFirstAction {
    OpenNetlistWorkspace,
    /// The deck in the Netlist workspace with the caret on this 1-based line.
    OpenAtLine(usize),
    CreateSchematic,
}

/// Run the action once the surface has drawn; the surface draws nothing
/// after it. Opening the deck brings the project's deck to the front of the
/// Netlist workspace whatever was in front there, as the project overview's
/// Open SPICE deck does; creating a schematic runs its command.
pub fn raise_netlist_first_action(wb: &mut WorkbenchSession, action: NetlistFirstAction) {
    match action {
        NetlistFirstAction::OpenNetlistWorkspace | NetlistFirstAction::OpenAtLine(_) => {
            wb.outbox.push(ShowProjectDeck);
        }
        NetlistFirstAction::CreateSchematic => wb.outbox.push(Execute(ids::NEW_CELL)),
    }
}

/// The surface the source feature registers, in its context: on Design the
/// empty canvas, on Simulate the deck's own setup.
pub(crate) fn show(cx: &mut Cx<'_, SourceFeature>, ui: &mut Ui) {
    open_deck(cx);
    if cx.wb.workspace == Workspace::Simulate {
        super::deck_setup::show(cx, ui);
        return;
    }
    let messages = cx.wb.messages();
    let name = deck_name(cx.model, &cx.state.netlist, messages);
    let open = messages.text(MessageId::NetlistFirstOpenInNetlist);
    let create = messages.text(MessageId::NetlistFirstCreateSchematic);
    let mut action = None;
    empty_state_with_actions(
        ui,
        WorkbenchIcon::Netlist,
        &messages.text(MessageId::NetlistFirstTitle),
        &messages.format(MessageId::NetlistFirstDescription, &[("name", &name)]),
        |ui| {
            if Button::new(&open).accent().show(ui).clicked() {
                action = Some(NetlistFirstAction::OpenNetlistWorkspace);
            }
            if Button::new(&create).show(ui).clicked() {
                action = Some(NetlistFirstAction::CreateSchematic);
            }
        },
    );
    if let Some(action) = action {
        raise_netlist_first_action(cx.wb, action);
    }
}

/// [`raise_netlist_first_action`], with the caret put on the line an
/// [`NetlistFirstAction::OpenAtLine`] names once the deck is in front.
pub(super) fn carry_out(cx: &mut Cx<'_, SourceFeature>, action: NetlistFirstAction) {
    if let NetlistFirstAction::OpenAtLine(line) = action {
        #[cfg(not(target_arch = "wasm32"))]
        crate::workbench::documents::deck_files::show_project_deck(
            cx.model,
            &mut cx.state.netlist,
            &mut cx.state.netlist_open_documents,
            cx.runs,
            cx.wb.messages(),
        );
        cx.state.netlist.cursor_line = line.saturating_sub(1);
        cx.state.netlist.requested_line = Some(line);
    }
    raise_netlist_first_action(cx.wb, action);
}

/// Open the project's deck as the Netlist page does: a restored session has
/// the deck's entry, not its text, and a deck page names and counts it.
pub(super) fn open_deck(cx: &mut Cx<'_, SourceFeature>) {
    #[cfg(not(target_arch = "wasm32"))]
    crate::workbench::documents::deck_files::open_project_deck(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.netlist_open_documents,
        cx.runs,
        cx.wb.messages(),
    );
    #[cfg(target_arch = "wasm32")]
    let _ = cx;
}

/// The deck the project runs, by its file's name.
pub fn deck_name(
    model: &Model,
    netlist: &NetlistDocumentState,
    messages: MessageCatalog,
) -> String {
    netlist
        .open_deck
        .as_ref()
        .map(|deck| deck.root.name())
        .or_else(|| {
            model
                .workspace
                .content
                .decks
                .first()
                .map(|deck| deck.name())
        })
        .unwrap_or_else(|| messages.text(MessageId::NetlistFirstUntitled))
}

/// Everything a deck page states about the deck, derived from the canonical
/// document and the editor's own cached source index.
pub struct DeckFacts {
    pub name: String,
    pub path: String,
    pub lines: usize,
    pub devices: usize,
    pub analyses: usize,
    pub models: usize,
    pub subcircuits: usize,
    pub dependencies: usize,
}

pub fn deck_facts(
    model: &Model,
    netlist: &NetlistDocumentState,
    index: &NetlistSourceIndex,
    messages: MessageCatalog,
) -> DeckFacts {
    let (mut devices, mut analyses, mut models, mut subcircuits, mut dependencies) =
        (0, 0, 0, 0, 0);
    for entry in index.outline().entries() {
        match entry.kind() {
            OutlineEntryKind::Device => devices += 1,
            OutlineEntryKind::Analysis => analyses += 1,
            OutlineEntryKind::Model => models += 1,
            OutlineEntryKind::Subcircuit => subcircuits += 1,
            OutlineEntryKind::Include | OutlineEntryKind::Library => dependencies += 1,
            _ => {}
        }
    }
    let name = deck_name(model, netlist, messages);
    let content = &model.workspace.content;
    let path = netlist
        .open_deck
        .as_ref()
        .map(|deck| deck.root.path.display().to_string())
        .or_else(|| {
            content.decks.first().and_then(|deck| {
                deck.file_path(content.project.data_root())
                    .map(|path| path.display().to_string())
            })
        })
        .unwrap_or_else(|| name.clone());

    DeckFacts {
        name,
        path,
        lines: index.line_count(),
        devices,
        analyses,
        models,
        subcircuits,
        dependencies,
    }
}

/// The deck's facts as every deck page's inspector lists them, `more` after
/// them in the same list: its file, then what it holds.
pub fn deck_fact_rows(ui: &mut Ui, facts: &DeckFacts, more: &[Fact<'_>], messages: MessageCatalog) {
    let file = messages.text(MessageId::SourceFileFile);
    let counts = [
        (MessageId::NetlistFirstLines, facts.lines),
        (MessageId::NetlistFirstDevices, facts.devices),
        (MessageId::NetlistFirstAnalyses, facts.analyses),
        (MessageId::NetlistFirstModels, facts.models),
        (MessageId::NetlistFirstSubcircuits, facts.subcircuits),
        (MessageId::NetlistFirstIncludes, facts.dependencies),
    ]
    .map(|(label, count)| (messages.text(label), count.to_string()));
    let mut rows = Vec::with_capacity(1 + counts.len() + more.len());
    rows.push(Fact::path(&file, &facts.path));
    rows.extend(counts.iter().map(|(label, count)| Fact::new(label, count)));
    rows.extend_from_slice(more);
    fact_rows(ui, &rows);
}
