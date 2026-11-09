//! The Verilog-A editor's completion list, signature help and name card.
//!
//! Keys the list owns are taken before the editor sees them; what the keys
//! and the typing asked for is applied once the editor has drawn, against the
//! text and caret it now holds. An accepted completion goes back to the
//! editor as one undoable edit. All of it can be reached without a pointer:
//! Ctrl+Space opens the list, Ctrl+Shift+Space shows what the pointer's hover
//! would, and the same facts are in the editor's accessible description.

use std::sync::Arc;

use egui::{Key, Modifiers, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::documents::text_editor_commands::queue_text_replacement;
use crate::workbench::documents::virtual_text_editor::{
    VirtualEditorOutput, bounded_accessible_text,
};
use crate::workbench::{MessageCatalog, MessageId};

use super::super::veriloga_assist::{
    CallContext, Candidate, CompletionQuery, NameInformation, QueryKind, SignatureHelp, call_at,
    candidates, completion_query, information_at, is_callable, signature_help,
};
use super::super::veriloga_symbols::VerilogASymbols;
use super::{FONT_SIZE, LINE_HEIGHT};

const ROW_HEIGHT: f32 = 24.0;
const POPUP_WIDTH: f32 = 460.0;
const CARD_WIDTH: f32 = 420.0;
const SCREEN_MARGIN: f32 = 8.0;
const VISIBLE_ROWS: usize = 10;
/// The kind column, wide enough for the longest kind's word.
const KIND_WIDTH: f32 = 76.0;
/// The longest a sentence added to the accessible description may be.
const MAX_ASSIST_SENTENCE_CHARS: usize = 384;

/// What the keys of one frame asked of the list and the cards.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct AssistKeys {
    step: i32,
    accept: bool,
    dismiss: bool,
    ask_completion: bool,
    ask_information: bool,
}

#[derive(Debug, Clone, Default)]
struct Popup {
    /// Where the word being completed starts, in bytes.
    start: usize,
    /// Opened by Ctrl+Space or by a `$`, a backtick or an include: Enter
    /// accepts. Opened by a plain word being typed, Enter still ends the line
    /// until the list has been moved in.
    asked_for: bool,
    moved: bool,
    selected: usize,
    first_shown: usize,
    prefix: String,
}

/// The file's symbols as last read, and the last reading that succeeded.
#[derive(Clone, Default)]
struct Reading {
    serial: u64,
    forgiven: Option<usize>,
    symbols: Option<Arc<VerilogASymbols>>,
    last_good: Option<Arc<VerilogASymbols>>,
}

#[derive(Clone, Default)]
struct AssistState {
    seen_revision: Option<u64>,
    /// Counts every change to the text, from the editor or from outside it.
    serial: u64,
    length: usize,
    reading: Reading,
    popup: Option<Popup>,
    dismissed_at: Option<u64>,
    signature_open: bool,
    /// A row was taken this frame; the text arrives the next.
    took_a_row: bool,
    /// The card Ctrl+Shift+Space opened, standing while the text and caret
    /// it was opened on do.
    card: Option<(u64, usize)>,
    /// The hover answered last, by text and character.
    hover: Option<(u64, usize, Option<NameInformation>)>,
    /// What the caret was last said to be on, by text and character.
    spoken: Option<(u64, usize, Option<NameInformation>)>,
}

fn state_id(editor_id: egui::Id) -> egui::Id {
    editor_id.with("veriloga-assist")
}

/// Take the keys the list and the cards own, before the editor would.
pub(super) fn consume_keys(ui: &Ui, editor_id: egui::Id, editable: bool) -> AssistKeys {
    if !ui.memory(|memory| memory.has_focus(editor_id)) {
        return AssistKeys::default();
    }
    let state = ui
        .ctx()
        .data(|data| data.get_temp::<AssistState>(state_id(editor_id)))
        .unwrap_or_default();
    let card_open = state.signature_open || state.card.is_some();
    let popup = state.popup.as_ref();
    ui.ctx().input_mut(|input| {
        let mut keys = AssistKeys {
            // With Shift first: the plain chord would match it too.
            ask_information: input.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::Space),
            ..AssistKeys::default()
        };
        keys.ask_completion = editable && input.consume_key(Modifiers::CTRL, Key::Space);
        if let Some(popup) = popup {
            let down = input.consume_key(Modifiers::NONE, Key::ArrowDown);
            let up = input.consume_key(Modifiers::NONE, Key::ArrowUp);
            keys.step = i32::from(down) - i32::from(up);
            let enter = (popup.asked_for || popup.moved || keys.step != 0)
                && input.consume_key(Modifiers::NONE, Key::Enter);
            let tab = !input.modifiers.shift && input.consume_key(Modifiers::NONE, Key::Tab);
            keys.accept = enter || tab;
        }
        if popup.is_some() || card_open {
            keys.dismiss = input.consume_key(Modifiers::NONE, Key::Escape);
        }
        keys
    })
}

/// What one frame's text and editor reported.
pub(super) struct AssistFrame<'a> {
    pub source: &'a str,
    pub source_revision: u64,
    pub editable: bool,
    pub include_files: &'a [String],
    pub messages: MessageCatalog,
}

/// What the editor shows or says on the assist's behalf.
#[derive(Debug, Default)]
pub(super) struct AssistAnswer {
    /// What the name under the pointer is.
    pub hover: Option<NameInformation>,
    /// For the status bar: Ctrl+Shift+Space found nothing to explain.
    pub announcement: Option<String>,
}

impl AssistState {
    fn note_text(&mut self, revision: u64, changed: bool, length: usize) -> bool {
        let moved_on = changed || self.seen_revision != Some(revision);
        if moved_on {
            self.serial += 1;
        }
        self.seen_revision = Some(revision);
        let grew = changed && length > self.length;
        self.length = length;
        grew
    }

    /// The file's symbols for this text, or `None` when neither it nor it
    /// without the caret's line can be read.
    fn symbols(&mut self, source: &str, line: usize) -> Option<Arc<VerilogASymbols>> {
        let reading = &self.reading;
        if reading.serial == self.serial
            && reading.symbols.is_some()
            && reading.forgiven.is_none_or(|forgiven| forgiven == line)
        {
            return reading.symbols.clone();
        }
        let plain = VerilogASymbols::read(source);
        let (symbols, forgiven) = if plain.unreadable_line().is_none() {
            (Some(plain), None)
        } else {
            let forgiving = VerilogASymbols::read_forgiving(source, Some(line));
            (
                forgiving.unreadable_line().is_none().then_some(forgiving),
                Some(line),
            )
        };
        let symbols = symbols.map(Arc::new);
        self.reading.serial = self.serial;
        self.reading.forgiven = forgiven;
        self.reading.symbols.clone_from(&symbols);
        if symbols.is_some() {
            self.reading.last_good.clone_from(&symbols);
        }
        symbols
    }

    /// The symbols a list may offer from: this text's, or, while it cannot
    /// be read, the last that could. Declarations hardly move in a few
    /// keystrokes, so an approximate scope beats an empty list.
    fn offered_symbols(&mut self, source: &str, line: usize) -> Option<Arc<VerilogASymbols>> {
        self.symbols(source, line)
            .or_else(|| self.reading.last_good.clone())
    }
}

fn byte_at(source: &str, char_index: usize) -> usize {
    source
        .char_indices()
        .nth(char_index)
        .map_or(source.len(), |(byte, _)| byte)
}

/// Run the list and the cards for this frame.
pub(super) fn show(
    ui: &Ui,
    editor_id: egui::Id,
    frame: &AssistFrame<'_>,
    output: &VirtualEditorOutput,
    keys: AssistKeys,
) -> AssistAnswer {
    let context = ui.ctx().clone();
    let id = state_id(editor_id);
    let mut state = context
        .data(|data| data.get_temp::<AssistState>(id))
        .unwrap_or_default();
    let source = frame.source;
    let grew = state.note_text(frame.source_revision, output.changed, source.len());
    let caret_char = output.cursor_char_index;
    let caret = byte_at(source, caret_char);
    let caret_line = source[..caret].matches('\n').count() + 1;
    let focused = output.response.has_focus();
    let mut answer = AssistAnswer::default();

    // The list.
    if keys.dismiss && state.popup.is_some() {
        state.popup = None;
        state.dismissed_at = Some(state.serial);
    } else if keys.dismiss {
        state.signature_open = false;
        state.card = None;
    }
    if frame.editable && focused {
        run_completion(
            ui,
            editor_id,
            frame,
            output,
            keys,
            &mut state,
            (caret, caret_line, grew),
        );
    } else {
        state.popup = None;
    }

    // Signature help and the card.
    if keys.ask_information {
        state.card = Some((state.serial, caret_char));
    }
    state.card = state
        .card
        .filter(|place| *place == (state.serial, caret_char) && focused);
    let typed_into_call = grew && matches!(source[..caret].chars().next_back(), Some('(' | ','));
    let wants_call = typed_into_call || state.signature_open || state.card.is_some();
    let symbols = if wants_call || keys.ask_information {
        state.symbols(source, caret_line)
    } else {
        None
    };
    let call = wants_call
        .then(|| call_at(source, caret, |name| is_callable(name, symbols.as_deref())))
        .flatten();
    let help = call
        .as_ref()
        .and_then(|call| signature_help(call, symbols.as_deref()));
    if typed_into_call || keys.ask_information {
        state.signature_open = help.is_some();
    }
    if help.is_none() || !focused {
        state.signature_open = false;
    }
    let card_information = state
        .card
        .is_some()
        .then(|| information_at(source, caret, symbols.as_deref()))
        .flatten();
    if keys.ask_information && help.is_none() && card_information.is_none() {
        state.card = None;
        answer.announcement = Some(frame.messages.text(MessageId::VerilogAAssistNothingAtCaret));
    }
    if let Some(anchor) = output.caret_anchor {
        let shown_help = help.as_ref().filter(|_| state.signature_open);
        if shown_help.is_some() || card_information.is_some() {
            draw_card(ui, editor_id, anchor, shown_help, card_information.as_ref());
        }
    }

    // The pointer's hover, kept while it stays on the same character.
    if let Some(index) = output.hover_char_index.filter(|_| !output.changed) {
        let cached = state
            .hover
            .as_ref()
            .filter(|(serial, at, _)| *serial == state.serial && *at == index)
            .map(|(_, _, information)| information.clone());
        let information = cached.unwrap_or_else(|| {
            let symbols = state.symbols(source, caret_line);
            let information = information_at(source, byte_at(source, index), symbols.as_deref());
            state.hover = Some((state.serial, index, information.clone()));
            information
        });
        answer.hover = information;
    }

    // What a screen reader is told about the caret.
    if context.accesskit_node_builder(editor_id, |_| ()).is_some() {
        let sentences = caret_sentences(&mut state, frame, output, caret, caret_line);
        if !sentences.is_empty() {
            context.accesskit_node_builder(editor_id, |node| {
                let mut description = node.description().unwrap_or_default().to_owned();
                for sentence in &sentences {
                    if !description.is_empty() {
                        description.push(' ');
                    }
                    description.push_str(sentence);
                }
                node.set_description(description);
            });
        }
    }
    context.data_mut(|data| data.insert_temp(id, state));
    answer
}

/// Open, update, draw and act on the completion list.
fn run_completion(
    ui: &Ui,
    editor_id: egui::Id,
    frame: &AssistFrame<'_>,
    output: &VirtualEditorOutput,
    keys: AssistKeys,
    state: &mut AssistState,
    (caret, caret_line, grew): (usize, usize, bool),
) {
    let source = frame.source;
    let query = completion_query(source, caret);
    // The text just taken from the list is not more typing for it to answer.
    let took_a_row = std::mem::take(&mut state.took_a_row);
    if keys.ask_completion {
        state.popup = query.as_ref().map(|query| Popup {
            start: query.range.start,
            asked_for: true,
            prefix: query.prefix.clone(),
            ..Popup::default()
        });
        state.dismissed_at = None;
    } else if grew
        && !took_a_row
        && state.popup.is_none()
        && state.dismissed_at != Some(state.serial)
        && let Some(query) = query.as_ref()
        && opens_by_typing(query)
    {
        state.popup = Some(Popup {
            start: query.range.start,
            asked_for: query.kind != QueryKind::Name,
            prefix: query.prefix.clone(),
            ..Popup::default()
        });
    }
    let Some(mut popup) = state.popup.take() else {
        return;
    };
    let Some(query) = query.filter(|query| {
        query.range.start == popup.start
            && (popup.asked_for || popup.moved || opens_by_typing(query))
    }) else {
        return;
    };
    let Some(anchor) = output.caret_anchor else {
        return;
    };
    let symbols = state.offered_symbols(source, caret_line);
    let offered = candidates(
        &query,
        source,
        symbols.as_deref(),
        frame.include_files,
        popup.asked_for,
    );
    if offered.is_empty() {
        return;
    }
    if query.prefix != popup.prefix {
        popup.prefix.clone_from(&query.prefix);
        popup.selected = 0;
        popup.first_shown = 0;
    }
    if keys.step != 0 {
        popup.moved = true;
    }
    popup.selected = (popup.selected as i32 + keys.step).rem_euclid(offered.len() as i32) as usize;
    popup.first_shown = popup
        .first_shown
        .min(popup.selected)
        .max((popup.selected + 1).saturating_sub(VISIBLE_ROWS));
    let clicked = draw_popup(ui, editor_id, anchor, &offered, &popup, frame.messages);
    let accepted = if keys.accept {
        Some(popup.selected)
    } else {
        clicked
    };
    match accepted.and_then(|index| offered.get(index)) {
        Some(chosen) => {
            let start = source[..query.range.start].chars().count();
            let context = ui.ctx();
            queue_text_replacement(
                context,
                editor_id,
                start..output.cursor_char_index,
                chosen.insert.clone(),
            );
            context.memory_mut(|memory| memory.request_focus(editor_id));
            context.request_repaint();
            state.took_a_row = true;
        }
        None => state.popup = Some(popup),
    }
}

/// Whether a list opens by itself for this much typed: a `$`, a backtick and
/// an include open one at once, a plain word from its second character.
fn opens_by_typing(query: &CompletionQuery) -> bool {
    query.kind != QueryKind::Name || query.prefix.chars().count() >= 2
}

/// The sentences added to the editor's accessible description: the list's
/// choice while it is open, what the caret's name is, and the argument being
/// written.
fn caret_sentences(
    state: &mut AssistState,
    frame: &AssistFrame<'_>,
    output: &VirtualEditorOutput,
    caret: usize,
    caret_line: usize,
) -> Vec<String> {
    let source = frame.source;
    let messages = frame.messages;
    let mut sentences = Vec::new();
    if let Some(popup) = state.popup.clone()
        && let Some(query) = completion_query(source, caret)
    {
        let symbols = state.offered_symbols(source, caret_line);
        let offered = candidates(
            &query,
            source,
            symbols.as_deref(),
            frame.include_files,
            popup.asked_for,
        );
        if let Some(chosen) = offered.get(popup.selected) {
            sentences.push(messages.format(
                MessageId::VerilogACompletionAccessible,
                &[
                    ("count", &offered.len().to_string()),
                    ("label", &chosen.label),
                    ("kind", chosen.kind),
                    ("description", &chosen.description),
                ],
            ));
        }
    }
    let symbols = state.symbols(source, caret_line);
    let char_index = output.cursor_char_index;
    let spoken = state
        .spoken
        .as_ref()
        .filter(|(serial, at, _)| *serial == state.serial && *at == char_index)
        .map(|(_, _, information)| information.clone());
    let information = spoken.unwrap_or_else(|| {
        let information = information_at(source, caret, symbols.as_deref());
        state.spoken = Some((state.serial, char_index, information.clone()));
        information
    });
    if let Some(information) = information {
        sentences.push(information.sentence());
    }
    if let Some(call) = call_at(source, caret, |name| is_callable(name, symbols.as_deref()))
        && let Some(help) = signature_help(&call, symbols.as_deref())
    {
        sentences.push(call_sentence(&call, &help, messages));
    }
    sentences
        .iter()
        .map(|sentence| bounded_accessible_text(sentence, MAX_ASSIST_SENTENCE_CHARS))
        .collect()
}

fn call_sentence(call: &CallContext, help: &SignatureHelp, messages: MessageCatalog) -> String {
    let signature = help.signature();
    let position = (call.argument + 1).to_string();
    if help.active.is_empty() {
        messages.format(
            MessageId::VerilogACallAccessible,
            &[("signature", &signature)],
        )
    } else {
        messages.format(
            MessageId::VerilogACallArgumentAccessible,
            &[
                ("position", &position),
                ("signature", &signature),
                ("argument", help.active.trim_end_matches('…')),
            ],
        )
    }
}

/// The list: a row per name, the selected one's explanation under them.
/// Returns the row the pointer chose.
fn draw_popup(
    ui: &Ui,
    editor_id: egui::Id,
    anchor: egui::Pos2,
    offered: &[Candidate],
    popup: &Popup,
    messages: MessageCatalog,
) -> Option<usize> {
    let colors = Tokens::get(ui.ctx()).color;
    let screen = ui.ctx().content_rect();
    let width = POPUP_WIDTH.min((screen.width() - SCREEN_MARGIN * 2.0).max(0.0));
    let rows = offered.len().min(VISIBLE_ROWS);
    let selected = popup.selected.min(offered.len() - 1);
    let footer_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let description = &offered[selected].description;
    let footer = if description.is_empty() {
        messages.text(MessageId::VerilogACompletionHint)
    } else {
        format!(
            "{description}  ·  {}",
            messages.text(MessageId::VerilogACompletionHint)
        )
    };
    let footer = ui.painter().layout(
        footer,
        footer_font,
        colors.text_dim,
        (width - 20.0).max(0.0),
    );
    let height = rows as f32 * ROW_HEIGHT + footer.size().y + 14.0 + 2.0;
    // Under the caret, or above its line where the bottom of the window
    // would cut it.
    let fits_below = anchor.y + height <= screen.bottom() - SCREEN_MARGIN;
    let x = anchor
        .x
        .min(screen.right() - SCREEN_MARGIN - width)
        .max(screen.left() + SCREEN_MARGIN);
    let (position, pivot) = if fits_below {
        (egui::pos2(x, anchor.y), egui::Align2::LEFT_TOP)
    } else {
        (
            egui::pos2(x, anchor.y - LINE_HEIGHT - 8.0),
            egui::Align2::LEFT_BOTTOM,
        )
    };
    let mut clicked = None;
    egui::Area::new(editor_id.with("completion"))
        .order(egui::Order::Foreground)
        .pivot(pivot)
        .fixed_pos(position)
        .show(ui.ctx(), |ui| {
            popup_frame(colors).show(ui, |ui| {
                ui.set_width(width);
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                let shown = offered
                    .iter()
                    .enumerate()
                    .skip(popup.first_shown)
                    .take(VISIBLE_ROWS);
                for (index, candidate) in shown {
                    let is_selected = index == selected;
                    if completion_row(ui, candidate, is_selected, width, messages).clicked() {
                        clicked = Some(index);
                    }
                }
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(width, footer.size().y + 14.0),
                    egui::Sense::hover(),
                );
                ui.painter().hline(
                    rect.x_range(),
                    rect.top() + 0.5,
                    egui::Stroke::new(1.0, colors.border),
                );
                ui.painter()
                    .galley(rect.min + egui::vec2(10.0, 7.0), footer, colors.text_dim);
            });
        });
    clicked
}

fn popup_frame(colors: crate::ui::palette::Palette) -> egui::Frame {
    egui::Frame::NONE
        .fill(colors.bg_elevated)
        .stroke(egui::Stroke::new(1.0, colors.border_strong))
        .corner_radius(3.0)
        .shadow(egui::epaint::Shadow {
            offset: [0, 6],
            blur: 24,
            spread: 0,
            color: egui::Color32::from_black_alpha(115),
        })
}

fn completion_row(
    ui: &mut Ui,
    candidate: &Candidate,
    selected: bool,
    width: f32,
    messages: MessageCatalog,
) -> egui::Response {
    let colors = Tokens::get(ui.ctx()).color;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, ROW_HEIGHT), egui::Sense::click());
    let announced = messages.format(
        MessageId::VerilogACompletionRow,
        &[
            ("kind", candidate.kind),
            ("label", &candidate.label),
            ("detail", &candidate.detail),
        ],
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            announced.clone(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::ListBoxOption);
        node.set_selected(selected);
    });
    if selected {
        ui.painter().rect_filled(rect, 0.0, colors.accent_dim);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, colors.bg_hover);
    }
    let center = rect.center().y;
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, center),
        egui::Align2::LEFT_CENTER,
        candidate.kind,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        colors.text_faint,
    );
    let label_font = theme::mono(tokens::FS_1, FontWeight::Regular);
    let label_left = rect.left() + 10.0 + KIND_WIDTH;
    let label = ui.painter().text(
        egui::pos2(label_left, center),
        egui::Align2::LEFT_CENTER,
        &candidate.label,
        label_font,
        colors.text,
    );
    if !candidate.detail.is_empty() {
        let detail_font = theme::mono(tokens::FS_0, FontWeight::Regular);
        let room = rect.right() - 10.0 - (label.right() + 16.0);
        let detail = crate::workbench::design_system::elide_text(
            ui,
            &candidate.detail,
            &detail_font,
            room.max(0.0),
        );
        ui.painter().text(
            egui::pos2(rect.right() - 10.0, center),
            egui::Align2::RIGHT_CENTER,
            detail,
            detail_font,
            colors.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    response
}

/// Signature help, the name card, or both, over the caret's line.
fn draw_card(
    ui: &Ui,
    editor_id: egui::Id,
    anchor: egui::Pos2,
    help: Option<&SignatureHelp>,
    information: Option<&NameInformation>,
) {
    let colors = Tokens::get(ui.ctx()).color;
    let screen = ui.ctx().content_rect();
    let width = CARD_WIDTH.min((screen.width() - SCREEN_MARGIN * 2.0).max(0.0));
    let x = anchor
        .x
        .min(screen.right() - SCREEN_MARGIN - width)
        .max(screen.left() + SCREEN_MARGIN);
    egui::Area::new(editor_id.with("signature"))
        .order(egui::Order::Foreground)
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(x, anchor.y - LINE_HEIGHT - 8.0))
        .interactable(false)
        .show(ui.ctx(), |ui| {
            popup_frame(colors)
                .inner_margin(egui::Margin::symmetric(10, 7))
                .show(ui, |ui| {
                    ui.set_max_width(width);
                    if let Some(help) = help {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            let mono = theme::mono(FONT_SIZE, FontWeight::Regular);
                            ui.label(
                                egui::RichText::new(&help.before)
                                    .font(mono.clone())
                                    .color(colors.text_dim),
                            );
                            ui.label(
                                egui::RichText::new(&help.active)
                                    .font(theme::mono(FONT_SIZE, FontWeight::SemiBold))
                                    .color(colors.accent)
                                    .underline(),
                            );
                            ui.label(
                                egui::RichText::new(&help.after)
                                    .font(mono)
                                    .color(colors.text_dim),
                            );
                        });
                        ui.label(
                            egui::RichText::new(&help.description)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(colors.text_dim),
                        );
                    }
                    if let Some(information) = information {
                        information_body(ui, information, &colors);
                    }
                });
        });
}

/// The body of a hover or a name card: how the name is written and what it
/// is, over what its declaration or the language says of it.
pub(super) fn information_body(
    ui: &mut Ui,
    information: &NameInformation,
    colors: &crate::ui::palette::Palette,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new(&information.title)
                .font(theme::mono(FONT_SIZE, FontWeight::Medium))
                .color(colors.text),
        );
        ui.label(
            egui::RichText::new(information.kind)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(colors.text_faint),
        );
    });
    if let Some(detail) = &information.detail {
        ui.label(
            egui::RichText::new(detail)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(colors.text_dim),
        );
    }
    if let Some(description) = &information.description {
        ui.label(
            egui::RichText::new(description)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(colors.text_dim),
        );
    }
}
