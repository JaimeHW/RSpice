//! Completion popover — the editor's one elevated surface.
//!
//! Two completion sources: dot-commands with signature docs, and the
//! deck's own symbols (`.model` / `.subckt` definitions, harvested from
//! the same parse that drives diagnostics). Anchored at the token being
//! typed; ↑↓ select, ⇥ accepts, Esc dismisses until the next edit.

use egui::text::{CCursor, CCursorRange};
use egui::text_edit::TextEditOutput;
use egui::{Key, Modifiers, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::NetlistEditorState;

/// Symbol harvested from the parsed deck.
#[derive(Debug, Clone)]
pub struct SymbolEntry {
    pub name: String,
    pub kind: &'static str,
    pub detail: String,
    pub doc: String,
}

/// One row offered by the popover.
struct Candidate {
    kind: &'static str,
    label: String,
    detail: String,
    doc: String,
    insert: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CompletionPopoverGeometry {
    position: egui::Pos2,
    width: f32,
}

const POPOVER_DESIRED_WIDTH: f32 = 300.0;
const POPOVER_SCREEN_MARGIN: f32 = 8.0;

/// Dot-commands with signatures (the popover's static source).
const DOT_COMMANDS: &[(&str, &str, &str)] = &[
    (
        ".tran",
        "<step> <stop> [start [maxstep]]",
        "Transient analysis.",
    ),
    (
        ".ac",
        "dec|oct|lin <points> <fstart> <fstop>",
        "Small-signal frequency sweep.",
    ),
    (
        ".dc",
        "<source> <start> <stop> <step>",
        "DC transfer sweep.",
    ),
    (".op", "", "DC operating point."),
    (
        ".noise",
        "v(<out>) <source> dec <pts> <f0> <f1>",
        "Output-referred noise analysis.",
    ),
    (
        ".meas",
        "tran|ac|dc <name> <type> <signal> …",
        "Post-run measurement (drives the specs matrix).",
    ),
    (
        ".param",
        "<name>=<value> …",
        "Deck parameter — appears in the tuner.",
    ),
    (
        ".model",
        "<name> <type> (<param>=<value> …)",
        "Device model card.",
    ),
    (
        ".subckt",
        "<name> <ports…> [params:]",
        "Subcircuit definition, closed by .ends.",
    ),
    (".ends", "[name]", "End of the enclosing .subckt."),
    (".include", "<path>", "Splice another deck file."),
    (".lib", "<path> <section>", "Include a library section."),
    (
        ".options",
        "<name>=<value> …",
        "Simulator options (reltol, abstol, temp…).",
    ),
    (
        ".ic",
        "v(<node>)=<value> …",
        "Transient initial conditions.",
    ),
    (".nodeset", "v(<node>)=<value> …", "DC convergence hints."),
    (".save", "<signal> …", "Restrict saved outputs."),
    (".probe", "<signal> …", "Mark signals for output."),
    (
        ".print",
        "<analysis> <signal> …",
        "Tabular output selection.",
    ),
    (".global", "<node> …", "Declare global nodes."),
    (".temp", "<celsius>", "Simulation temperature."),
    (
        ".four",
        "<freq> <signal> …",
        "Fourier analysis of a transient.",
    ),
    (".end", "", "End of the deck."),
];

/// Pre-consume the popover's keys. Must run *before* the `TextEdit` so
/// ⇥ and ↑↓ never reach the buffer while the popover is open. Returns
/// (move_delta, accept, dismiss).
pub fn consume_keys(ui: &Ui, state: &NetlistEditorState) -> (i32, bool, bool) {
    if !state.completion_open {
        return (0, false, false);
    }
    ui.ctx().input_mut(|input| {
        let down = input.consume_key(Modifiers::NONE, Key::ArrowDown);
        let up = input.consume_key(Modifiers::NONE, Key::ArrowUp);
        let accept = input.consume_key(Modifiers::NONE, Key::Tab);
        let dismiss = input.consume_key(Modifiers::NONE, Key::Escape);
        (down as i32 - up as i32, accept, dismiss)
    })
}

/// Drive the popover for this frame: trigger detection, rendering, and
/// acceptance. Returns the byte splice to apply, if a completion landed:
/// `(start, end, replacement, caret_char_index)`.
pub fn show(
    ui: &mut Ui,
    state: &mut NetlistEditorState,
    output: &TextEditOutput,
    buffer: &str,
    keys: (i32, bool, bool),
) -> Option<(usize, usize, String, usize)> {
    let (move_delta, accept, dismiss) = keys;

    if dismiss {
        state.completion_open = false;
        state.completion_dismissed_at = Some(state.revision);
        return None;
    }

    // Only complete while the editor owns focus and a cursor exists.
    let focused = output.response.has_focus();
    let Some(range) = output.cursor_range.filter(|_| focused) else {
        state.completion_open = false;
        return None;
    };
    if state.completion_dismissed_at == Some(state.revision) {
        state.completion_open = false;
        return None;
    }

    let cursor = range.primary;
    let Some((token_start, token, line_start)) = token_at(buffer, cursor.index) else {
        state.completion_open = false;
        return None;
    };

    let candidates = collect_candidates(state, buffer, &token, token_start == line_start);
    if candidates.is_empty() {
        state.completion_open = false;
        return None;
    }

    state.completion_index =
        (state.completion_index as i32 + move_delta).rem_euclid(candidates.len() as i32) as usize;
    let selected = state.completion_index.min(candidates.len() - 1);

    let clicked = draw_popover(ui, output, &candidates, selected);
    let take = if accept { Some(selected) } else { clicked };

    if let Some(index) = take {
        let chosen = &candidates[index];
        let end = cursor.index;
        let byte_start = char_to_byte(buffer, token_start);
        let byte_end = char_to_byte(buffer, end);
        let caret = token_start + chosen.insert.chars().count();
        state.completion_open = false;
        return Some((byte_start, byte_end, chosen.insert.clone(), caret));
    }

    state.completion_open = true;
    None
}

/// After a completion splice, park the caret right after the insertion.
pub fn place_caret(ui: &Ui, editor_id: egui::Id, char_index: usize) {
    let ctx = ui.ctx();
    let mut te_state = egui::text_edit::TextEditState::load(ctx, editor_id).unwrap_or_default();
    te_state
        .cursor
        .set_char_range(Some(CCursorRange::one(CCursor::new(char_index))));
    te_state.store(ctx, editor_id);
    ctx.memory_mut(|memory| memory.request_focus(editor_id));
}

/// The identifier token ending at the caret: `(token_start_chars, text,
/// line_first_nonws_chars)`. `None` when the caret doesn't trail a token.
fn token_at(buffer: &str, caret: usize) -> Option<(usize, String, usize)> {
    let chars: Vec<char> = buffer.chars().collect();
    if caret > chars.len() {
        return None;
    }
    let is_token_char =
        |ch: char| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '!';
    let mut start = caret;
    while start > 0 && is_token_char(chars[start - 1]) {
        start -= 1;
    }
    if start == caret {
        return None; // caret not at the end of a token
    }
    // The line's first non-whitespace position (dot-command context check).
    let mut line_begin = start;
    while line_begin > 0 && chars[line_begin - 1] != '\n' {
        line_begin -= 1;
    }
    let mut first_nonws = line_begin;
    while first_nonws < start && chars[first_nonws].is_whitespace() {
        first_nonws += 1;
    }
    let token: String = chars[start..caret].iter().collect();
    Some((start, token, first_nonws))
}

fn char_to_byte(buffer: &str, char_index: usize) -> usize {
    buffer
        .char_indices()
        .nth(char_index)
        .map(|(byte, _)| byte)
        .unwrap_or(buffer.len())
}

/// Filter both sources against the typed prefix.
fn collect_candidates(
    state: &NetlistEditorState,
    buffer: &str,
    token: &str,
    at_line_start: bool,
) -> Vec<Candidate> {
    let mut out = Vec::new();
    let lower = token.to_ascii_lowercase();

    if token.starts_with('.') {
        if !at_line_start {
            return out;
        }
        for (cmd, signature, doc) in DOT_COMMANDS {
            if cmd.starts_with(&lower) && *cmd != lower {
                out.push(Candidate {
                    kind: "cmd",
                    label: (*cmd).to_owned(),
                    detail: (*signature).to_owned(),
                    doc: format!("{cmd} {signature} — {doc}"),
                    insert: format!("{cmd} "),
                });
            }
        }
        return out;
    }

    // Deck symbols (models, subckts) for bare identifiers of 2+ chars.
    if token.len() < 2 {
        return out;
    }
    for symbol in &state.symbols {
        let name_lower = symbol.name.to_ascii_lowercase();
        if name_lower.starts_with(&lower) && name_lower != lower {
            out.push(Candidate {
                kind: symbol.kind,
                label: symbol.name.clone(),
                detail: symbol.detail.clone(),
                doc: symbol.doc.clone(),
                insert: symbol.name.clone(),
            });
        }
    }
    // Parameter names complete inside `{expressions}` and values.
    for (name, _, _) in super::tuner::buffer_assignments(buffer) {
        let name_lower = name.to_ascii_lowercase();
        if name_lower.starts_with(&lower) && name_lower != lower {
            out.push(Candidate {
                kind: "param",
                label: name.clone(),
                detail: String::new(),
                doc: format!("Deck parameter `{name}` (.param)."),
                insert: name,
            });
        }
    }
    out.truncate(8);
    out
}

fn completion_popover_geometry(
    anchor: egui::Pos2,
    screen_rect: egui::Rect,
) -> CompletionPopoverGeometry {
    let max_width = (screen_rect.width() - POPOVER_SCREEN_MARGIN * 2.0).max(0.0);
    let width = POPOVER_DESIRED_WIDTH.min(max_width);
    let min_x = screen_rect.left() + POPOVER_SCREEN_MARGIN;
    let max_x = (screen_rect.right() - POPOVER_SCREEN_MARGIN - width).max(min_x);
    let min_y = screen_rect.top() + POPOVER_SCREEN_MARGIN;
    let max_y = (screen_rect.bottom() - POPOVER_SCREEN_MARGIN).max(min_y);

    CompletionPopoverGeometry {
        position: egui::pos2(anchor.x.clamp(min_x, max_x), anchor.y.clamp(min_y, max_y)),
        width,
    }
}

/// The popover itself: items grid + selected-item doc footer. Returns the
/// row index the pointer accepted, if any.
fn draw_popover(
    ui: &Ui,
    output: &TextEditOutput,
    candidates: &[Candidate],
    selected: usize,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut clicked = None;

    // Anchor at the caret's baseline.
    let range = output.cursor_range?;
    let caret_rect = output.galley.pos_from_cursor(range.primary);
    let anchor = output.galley_pos + caret_rect.left_bottom().to_vec2() + egui::vec2(0.0, 4.0);
    let geometry = completion_popover_geometry(anchor, ui.ctx().screen_rect());

    egui::Area::new(egui::Id::new("volta.netlist.completion"))
        .order(egui::Order::Foreground)
        .fixed_pos(geometry.position)
        .show(ui.ctx(), |ui| {
            egui::Frame::NONE
                .fill(c.bg_elevated)
                .stroke(egui::Stroke::new(1.0, c.border_strong))
                .rounding(3.0)
                .shadow(egui::epaint::Shadow {
                    offset: [0, 6],
                    blur: 24,
                    spread: 0,
                    color: egui::Color32::from_black_alpha(115),
                })
                .show(ui, |ui| {
                    ui.set_min_width(geometry.width);
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                    for (idx, candidate) in candidates.iter().enumerate() {
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(geometry.width, 24.0),
                            egui::Sense::click(),
                        );
                        response.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::SelectableLabel,
                                ui.is_enabled(),
                                format!(
                                    "{} completion: {}, {}",
                                    candidate.kind, candidate.label, candidate.detail
                                ),
                            )
                        });
                        ui.ctx().accesskit_node_builder(response.id, |node| {
                            node.set_role(egui::accesskit::Role::ListBoxOption);
                            node.set_selected(idx == selected);
                        });
                        if response.clicked() {
                            clicked = Some(idx);
                        }
                        if idx == selected {
                            ui.painter().rect_filled(rect, 0.0, c.accent_dim);
                        } else if response.hovered() {
                            ui.painter().rect_filled(rect, 0.0, c.bg_hover);
                        }
                        ui.painter().text(
                            egui::pos2(rect.left() + 10.0, rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            candidate.kind,
                            theme::mono(tokens::FS_0, FontWeight::Regular),
                            c.text_faint,
                        );
                        ui.painter().text(
                            egui::pos2(rect.left() + 58.0, rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            &candidate.label,
                            theme::mono(tokens::FS_1, FontWeight::Regular),
                            c.text,
                        );
                        ui.painter().text(
                            egui::pos2(rect.right() - 10.0, rect.center().y),
                            egui::Align2::RIGHT_CENTER,
                            &candidate.detail,
                            theme::mono(tokens::FS_0, FontWeight::Regular),
                            c.text_faint,
                        );
                        theme::paint_focus_ring(ui, &response, rect);
                    }

                    // Doc footer for the selected item.
                    let doc = &candidates[selected].doc;
                    if !doc.is_empty() {
                        let galley = ui.painter().layout(
                            format!("{doc}  ·  ⇥ to accept"),
                            theme::sans(tokens::FS_0, FontWeight::Regular),
                            c.text_dim,
                            (geometry.width - 20.0).max(0.0),
                        );
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(geometry.width, galley.size().y + 14.0),
                            egui::Sense::hover(),
                        );
                        ui.painter().hline(
                            rect.x_range(),
                            rect.top() + 0.5,
                            egui::Stroke::new(1.0, c.border),
                        );
                        ui.painter()
                            .galley(rect.min + egui::vec2(10.0, 7.0), galley, c.text_dim);
                    }
                });
        });
    clicked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popover_geometry_clamps_width_and_position_to_phone_viewport() {
        let screen = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(320.0, 640.0));
        let anchor = egui::pos2(292.0, 40.0);

        let geometry = completion_popover_geometry(anchor, screen);

        assert!(geometry.width <= 304.0);
        assert!(geometry.position.x >= 8.0);
        assert!(geometry.position.x + geometry.width <= 312.0);
    }
}
