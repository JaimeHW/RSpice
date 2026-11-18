//! The project's decks, over the outline of the one in front: each opens
//! from its row. A project with one deck shows none, since the outline
//! already names it.

use egui::Ui;

use super::{
    NetlistNavigator, NetlistNavigatorAsked, NetlistOutlineRowShape, OutlineRowVisual,
    netlist_outline_row,
};
use crate::ui::panels::{WorkbenchIcon, section_header};
use crate::ui::tokens::Tokens;
use crate::workbench::MessageId;
use crate::workbench::documents::deck_files;

/// The Decks list, when the project has more than one deck; a row pressed
/// brings its deck to the front.
pub(super) fn list(
    ui: &mut Ui,
    parts: &mut NetlistNavigator<'_>,
    height: f32,
    asked: &mut NetlistNavigatorAsked,
) {
    let decks = &parts.model.workspace.content.decks;
    if decks.len() < 2 {
        return;
    }
    let messages = parts.messages;
    let folder = parts.model.workspace.content.project.data_root();
    let open = parts.netlist.open_deck.as_ref().map(|deck| deck.id);
    let rows = decks
        .iter()
        .map(|deck| {
            let path = deck.file_path(folder);
            let meta = match &path {
                None => Some(messages.text(MessageId::NetlistNavigatorDeckStored)),
                Some(path) if cfg!(not(target_arch = "wasm32")) && !path.exists() => {
                    Some(messages.text(MessageId::NetlistNavigatorDeckMissing))
                }
                Some(_) => None,
            };
            let missing = path.is_some() && meta.is_some();
            let tooltip = path.map_or_else(
                || messages.text(MessageId::NetlistNavigatorDeckStored),
                |path| path.display().to_string(),
            );
            (deck.id, deck.name(), meta, missing, tooltip)
        })
        .collect::<Vec<_>>();
    section_header(ui, &messages.text(MessageId::NetlistNavigatorDecks), None);
    let mut pressed = None;
    for (id, name, meta, missing, tooltip) in &rows {
        let response = netlist_outline_row(
            ui,
            OutlineRowVisual {
                label: name,
                meta: meta.as_deref(),
                meta_tone: missing.then(|| Tokens::get(ui.ctx()).color.warn),
                icon: Some(WorkbenchIcon::File),
                shape: NetlistOutlineRowShape::Leaf,
                selected: open == Some(*id),
                enabled: true,
                height,
                hover: None,
            },
        )
        .on_hover_text(tooltip);
        if response.clicked() {
            pressed = Some(*id);
        }
    }
    if let Some(id) = pressed
        && let Err(error) = deck_files::open_deck_by_id(
            parts.model,
            parts.netlist,
            parts.open_documents,
            parts.runs,
            id,
        )
    {
        asked.prompt = Some(error);
    }
}
