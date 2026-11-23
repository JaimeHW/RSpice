//! What a hover says, asked for at the caret.
//!
//! A hover answers the pointer: what the parameter or expression under it
//! evaluates to, or what the name under it is bound to. Show value asks the
//! same question of the caret, so the keyboard and the menus reach every
//! answer the pointer does. The card opens just below the caret and stays
//! until the caret moves, the text changes, Escape is pressed or the reader
//! clicks; the status line says the same words, for a screen reader.

use egui::Ui;

use super::NetlistDocumentState;
use super::completion::CompletionAnchor;
use super::language::{NetlistValueHover, ProjectSymbolHover};
use crate::workbench::MessageId;
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

/// Show value, raised by its command for the deck editor on screen, which
/// takes it as it draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShowValueAtCaret;

impl crate::workbench::framework::requests::Request for ShowValueAtCaret {
    const DRAIN: crate::workbench::framework::requests::Drain =
        crate::workbench::framework::requests::Drain::SourceEditor;
}

/// What a hover over one place of the deck says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum HoverCard {
    Value(NetlistValueHover),
    Name(ProjectSymbolHover),
}

impl HoverCard {
    /// The card's lines: `token = value` in mono with where it came from or
    /// why there is no value, or the name's title and what binds it. No
    /// severity colour: a name the deck has not bound yet is an ordinary
    /// state of a document being written.
    pub(super) fn ui(&self, ui: &mut Ui) {
        ui.set_max_width(CARD_WIDTH);
        match self {
            Self::Value(hover) => {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.label(egui::RichText::new(&hover.token).monospace().strong());
                    if let Some(value) = &hover.value {
                        ui.label(egui::RichText::new(format!("= {value}")).monospace());
                    }
                });
                if let Some(detail) = &hover.detail {
                    ui.weak(detail);
                }
            }
            Self::Name(hover) => {
                ui.label(egui::RichText::new(&hover.title).strong());
                ui.label(&hover.detail);
            }
        }
    }

    /// The card as one line, for the status line.
    fn spoken(&self) -> String {
        let (head, detail) = match self {
            Self::Value(hover) => (
                hover.value.as_ref().map_or_else(
                    || hover.token.clone(),
                    |value| format!("{} = {value}", hover.token),
                ),
                hover.detail.clone(),
            ),
            Self::Name(hover) => (hover.title.clone(), Some(hover.detail.clone())),
        };
        let detail = detail
            .map(|detail| {
                detail
                    .lines()
                    .map(str::trim)
                    .collect::<Vec<_>>()
                    .join(" · ")
            })
            .filter(|detail| !detail.is_empty());
        match detail {
            Some(detail) => format!("{head} · {detail}"),
            None => head,
        }
    }
}

/// What a hover at the first of `char_indices` that answers says: a value
/// the deck answers, or the parser's word that the line is not read, before
/// a name's binding, and a name that binds nothing before silence. The
/// pointer asks of the one character under it; the caret of the character
/// after it and the one before, as it stands in a token or just ends one.
/// A command waits (`wait`) for the map of the deck's values a pointer finds
/// still being built.
pub(super) fn hover_card_at(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
    messages: crate::workbench::MessageCatalog,
    char_indices: &[usize],
    wait: bool,
) -> Option<HoverCard> {
    let mut unbound = None;
    for &index in char_indices {
        match super::language::value_hover_at_in(
            ctx,
            netlist,
            messages,
            &model.simulation.source.netlist_content,
            index,
            wait,
        ) {
            Some(value) if value.value.is_some() || value.not_read => {
                return Some(HoverCard::Value(value));
            }
            Some(value) => {
                unbound.get_or_insert(value);
            }
            None => {}
        }
    }
    char_indices
        .iter()
        .find_map(|&index| super::language::name_hover(ctx, model, netlist, index, messages))
        .map(HoverCard::Name)
        .or_else(|| unbound.map(HoverCard::Value))
}

/// The card Show value opened, and the caret and text it was read at.
#[derive(Debug, Clone)]
struct OpenCard {
    revision: u64,
    caret: usize,
    card: HoverCard,
}

fn open_card_id(editor_id: egui::Id) -> egui::Id {
    editor_id.with("value-at-caret")
}

/// Before the editor draws: Escape closes an open card rather than reaching
/// the editor, unless the editor's right-click menu is open and takes it.
pub(super) fn take_escape(ctx: &egui::Context, editor_id: egui::Id, menu_open: bool) {
    let id = open_card_id(editor_id);
    if menu_open || ctx.data(|data| data.get_temp::<OpenCard>(id)).is_none() {
        return;
    }
    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
        ctx.data_mut(|data| data.remove::<OpenCard>(id));
    }
}

/// After the editor drew: open the card Show value asked for, at the caret,
/// or say there is nothing there; then draw the open card, inside the
/// editor's `view`, while the caret and the text are where it was read.
pub(super) fn show(
    ui: &Ui,
    model: &Model,
    netlist: &mut NetlistDocumentState,
    workbench: &mut WorkbenchSession,
    editor_id: egui::Id,
    anchor: CompletionAnchor,
    view: egui::Rect,
) {
    let ctx = ui.ctx();
    let id = open_card_id(editor_id);
    let caret = anchor.caret_char_index;
    let asked = workbench.outbox.take::<ShowValueAtCaret>().is_some();
    if asked {
        let messages = workbench.messages();
        // A caret that ends a name reads that name first: `{w|}` is about
        // `w`, not the braces around it.
        let card = match caret.checked_sub(1) {
            Some(before) if ends_word(&model.simulation.source.netlist_content, caret) => {
                hover_card_at(ctx, model, netlist, messages, &[before, caret], true)
            }
            Some(before) => hover_card_at(ctx, model, netlist, messages, &[caret, before], true),
            None => hover_card_at(ctx, model, netlist, messages, &[caret], true),
        };
        match card {
            Some(card) => {
                workbench.prompt(card.spoken());
                ctx.data_mut(|data| {
                    data.insert_temp(
                        id,
                        OpenCard {
                            revision: netlist.revision,
                            caret,
                            card,
                        },
                    );
                });
                // From a menu the editor gave its focus up; the caret the
                // card points at is the editor's to keep moving.
                ctx.memory_mut(|memory| memory.request_focus(editor_id));
            }
            None => {
                ctx.data_mut(|data| data.remove::<OpenCard>(id));
                workbench.prompt(messages.text(MessageId::NetlistShowValueNothing));
            }
        }
    }
    let Some(open) = ctx.data(|data| data.get_temp::<OpenCard>(id)) else {
        return;
    };
    // A click anywhere is the reader moving on, unless it was the click
    // that chose Show value from a menu.
    let clicked = !asked && ctx.input(|input| input.pointer.any_pressed());
    let Some(below_line) = anchor
        .screen_position
        .filter(|_| open.revision == netlist.revision && open.caret == caret && !clicked)
    else {
        ctx.data_mut(|data| data.remove::<OpenCard>(id));
        return;
    };
    // Above the caret's line, as a hover over text sits, so the lines the
    // reader goes on to stay in view and a completion list below the caret
    // is not covered; below it when the line is the editor's first.
    let line_height = crate::workbench::documents::editor_font::line_height(
        crate::workbench::documents::editor_font::text_size(ctx),
    );
    let line_top = below_line.y - CARET_ANCHOR_GAP - line_height;
    let line_bottom = below_line.y - CARET_ANCHOR_GAP;
    let height = ctx.data(|data| data.get_temp::<f32>(id.with("height")));
    let above = height.is_none_or(|height| line_top - CARD_GAP - height >= view.top());
    let x = below_line.x.min(view.right() - CARD_WIDTH).max(view.left());
    let (pivot, y) = if above {
        (egui::Align2::LEFT_BOTTOM, line_top - CARD_GAP)
    } else {
        (egui::Align2::LEFT_TOP, line_bottom + CARD_GAP)
    };
    let shown = egui::Area::new(id)
        .order(egui::Order::Foreground)
        .pivot(pivot)
        .fixed_pos(egui::pos2(x, y))
        .show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| open.card.ui(ui));
        });
    let height = shown.response.rect.height();
    ctx.data_mut(|data| data.insert_temp(id.with("height"), height));
}

/// Whether a word character stands just before character `caret` of
/// `text` and none at it.
fn ends_word(text: &str, caret: usize) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    let mut around = text.chars().skip(caret.saturating_sub(1));
    caret > 0 && around.next().is_some_and(word) && !around.next().is_some_and(word)
}

/// How far below the caret's line the editor puts the point popovers hang
/// from.
const CARET_ANCHOR_GAP: f32 = 4.0;
/// The air between the card and the caret's line.
const CARD_GAP: f32 = 2.0;
/// The card's widest, which keeps it inside the editor at its right edge.
const CARD_WIDTH: f32 = 360.0;
