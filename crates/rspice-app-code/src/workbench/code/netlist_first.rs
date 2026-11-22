//! Netlist-first design surface.
//!
//! A project opened from a deck has no schematic to draw, so the Design
//! surface says so in one compact block on the page grid: what the project
//! is, the deck it runs, and the two ways on from here, the deck in the
//! Netlist workspace or a schematic made here. The deck's facts are the
//! inspector's, the same rows the Netlist workspace lists beside the deck.
//! It is the code feature's surface, drawn in that feature's context while
//! the deck is the project's simulation.

use egui::{Align, Layout, Rect, Sense, Ui, UiBuilder, Vec2, WidgetInfo, WidgetType, pos2};

use crate::state::{NetlistSourceIndex, OutlineEntryKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::app_state::Model;
use crate::workbench::code::CodeFeature;
use crate::workbench::design_system::{Fact, centered_content_rect, fact_rows};
use crate::workbench::documents::netlist_document::NetlistDocumentState;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{Cx, Execute};
use crate::workbench::framework::requests::ShowProjectDeck;
use crate::workbench::state::WorkbenchSession;
use crate::workbench::{MessageCatalog, MessageId};

/// The page grid the workbench's landing surfaces share.
const CONTENT_MAX_WIDTH: f32 = 1240.0;
const DESKTOP_GUTTER: f32 = 30.0;
/// How far under the document's top edge a landing's block starts.
const BLOCK_TOP: f32 = 40.0;
/// A landing block's width: its sentence wraps at it and its buttons share it.
const BLOCK_WIDTH: f32 = 480.0;
/// Between the block's sentence and what follows it.
pub(super) const BLOCK_GAP: f32 = 16.0;
/// Between two buttons in the block's row.
const BUTTON_GAP: f32 = 8.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetlistFirstAction {
    OpenNetlistWorkspace,
    CreateSchematic,
}

/// Run the action once the surface has drawn; the surface draws nothing
/// after it. Opening the deck brings the project's deck to the front of the
/// Netlist workspace whatever was in front there, as the project overview's
/// Open SPICE deck does; creating a schematic runs its command.
pub fn raise_netlist_first_action(wb: &mut WorkbenchSession, action: NetlistFirstAction) {
    match action {
        NetlistFirstAction::OpenNetlistWorkspace => wb.outbox.push(ShowProjectDeck),
        NetlistFirstAction::CreateSchematic => wb.outbox.push(Execute(ids::NEW_CELL)),
    }
}

/// The surface the code feature registers: the landing, in its context.
pub(crate) fn show(cx: &mut Cx<'_, CodeFeature>, ui: &mut Ui) {
    open_deck(cx);
    let messages = cx.wb.messages();
    let name = deck_name(cx.model, &cx.state.netlist, messages);
    let open = messages.text(MessageId::NetlistFirstOpenInNetlist);
    let create = messages.text(MessageId::NetlistFirstCreateSchematic);
    let mut action = None;
    landing_page(ui, |block| {
        landing_head(
            block,
            &LandingText {
                eyebrow: messages.text(MessageId::NetlistFirstEyebrow),
                title: &name,
                sentence: messages.text(MessageId::NetlistFirstDescription),
                accessible: messages.format(MessageId::NetlistFirstAccessible, &[("name", &name)]),
            },
        );
        block.add_space(BLOCK_GAP);
        action = match landing_buttons(block, &[(&open, true), (&create, false)]) {
            Some(0) => Some(NetlistFirstAction::OpenNetlistWorkspace),
            Some(_) => Some(NetlistFirstAction::CreateSchematic),
            None => None,
        };
    });
    if let Some(action) = action {
        raise_netlist_first_action(cx.wb, action);
    }
}

/// Open the project's deck as the Netlist page does: a restored session has
/// the deck's entry, not its text, and a deck page names and counts it.
pub(super) fn open_deck(cx: &mut Cx<'_, CodeFeature>) {
    #[cfg(not(target_arch = "wasm32"))]
    crate::workbench::documents::deck_files::open_project_deck(
        cx.model,
        &mut cx.state.netlist,
        &mut cx.state.netlist_open_documents,
        cx.runs,
    );
    #[cfg(target_arch = "wasm32")]
    let _ = cx;
}

/// A landing page: the page's ground taken whole, and `block` drawn in a
/// column [`BLOCK_WIDTH`] wide (or the grid's width where that is less) at
/// the grid's left edge near the top.
pub(super) fn landing_page(ui: &mut Ui, block: impl FnOnce(&mut Ui)) {
    let available = ui.available_rect_before_wrap();
    let (page, _) = ui.allocate_exact_size(available.size(), Sense::hover());
    ui.painter()
        .rect_filled(page, 0.0, Tokens::get(ui.ctx()).color.bg_app);
    let grid = centered_content_rect(page, DESKTOP_GUTTER, CONTENT_MAX_WIDTH);
    let top = (grid.top() + BLOCK_TOP).min(grid.bottom());
    let column = Rect::from_min_max(
        pos2(grid.left(), top),
        pos2(grid.left() + grid.width().min(BLOCK_WIDTH), grid.bottom()),
    );
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(column)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(column.intersect(ui.clip_rect()));
    child.spacing_mut().item_spacing = Vec2::ZERO;
    block(&mut child);
}

/// What a landing's block says: the line over its title, the title, the one
/// sentence under it, and what a screen reader hears for the three.
pub(super) struct LandingText<'a> {
    pub(super) eyebrow: String,
    pub(super) title: &'a str,
    pub(super) sentence: String,
    pub(super) accessible: String,
}

/// The block's heading in the landing typography: the eyebrow in small
/// spaced capitals, the title, and the sentence wrapped at the block's width.
pub(super) fn landing_head(ui: &mut Ui, text: &LandingText<'_>) {
    let t = Tokens::get(ui.ctx());
    let top = ui.cursor().top();
    ui.add(
        egui::Label::new(
            egui::RichText::new(&text.eyebrow)
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(t.color.text_faint)
                .extra_letter_spacing(0.09 * tokens::FS_0),
        )
        .truncate(),
    );
    ui.add_space(5.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(text.title)
                .font(theme::sans(20.0, FontWeight::SemiBold))
                .color(t.color.text),
        )
        .truncate(),
    );
    ui.add_space(6.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(&text.sentence)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
    let column = ui.max_rect();
    let head = Rect::from_min_max(
        pos2(column.left(), top),
        pos2(column.right(), ui.cursor().top()),
    );
    let response = ui.interact(head, ui.id().with("landing-head"), Sense::hover());
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &text.accessible));
}

/// The block's buttons in one row of equal widths across the block, each
/// `primary` one in the accent. Returns which was pressed.
pub(super) fn landing_buttons(ui: &mut Ui, buttons: &[(&str, bool)]) -> Option<usize> {
    let count = buttons.len().max(1) as f32;
    let width = ((ui.available_width() - BUTTON_GAP * (count - 1.0)) / count)
        .floor()
        .max(1.0);
    let mut pressed = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = BUTTON_GAP;
        for (index, &(label, primary)) in buttons.iter().enumerate() {
            let button = Button::new(label).min_width(width).max_width(width);
            let button = if primary { button.accent() } else { button };
            if button.show(ui).clicked() {
                pressed = Some(index);
            }
        }
    });
    pressed
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
    let file = messages.text(MessageId::CodeSourceFileFile);
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
