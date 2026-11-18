//! Completion popover — the editor's one elevated surface.
//!
//! What it offers is what the engine reads where the caret is: the dot
//! cards, with their signatures, at the head of a line; and, through the
//! engine's index of the deck, the models, subcircuits and functions the
//! caret's scope sees, its nodes and its parameters. On an element line it
//! says which pin comes next, from the subcircuit's port list or the
//! device's terminals. Anchored at the token being typed; ↑↓ select, ⇥
//! accepts, Esc dismisses until the next edit.

use egui::{Key, Modifiers, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::NetlistDocumentState;

/// Symbol harvested from the parsed deck.
#[derive(Debug, Clone)]
pub struct SymbolEntry {
    pub name: String,
    pub kind: &'static str,
    pub detail: String,
    pub doc: String,
}

/// Where the popover attaches, taken from the editor that owns the caret.
///
/// The popover deliberately knows nothing about which editor widget produced
/// this: it needs a caret index to read the token being typed and a screen
/// point to sit under, and both are facts any editor can state.
#[derive(Debug, Clone, Copy)]
pub struct CompletionAnchor {
    pub focused: bool,
    pub caret_char_index: usize,
    /// `None` when the caret is scrolled or folded out of view. The popover
    /// closes rather than float somewhere the caret is not.
    pub screen_position: Option<egui::Pos2>,
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
    (".param", "<name>=<value> …", "Deck parameter."),
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
    (
        ".initcond",
        "<device> ic=<value>[,<value>…] … | file <path>",
        "Xyce-compatible device initial-condition overrides.",
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
    (
        ".func",
        "<name>(<args>) {<expression>}",
        "User function for expressions.",
    ),
    (".endl", "[section]", "End of a library section."),
    (
        ".tf",
        "v(<out>) <source>",
        "DC transfer function and input/output resistance.",
    ),
    (".sens", "v(<out>) [ac …]", "Sensitivity of an output."),
    (
        ".pz",
        "<in+> <in-> <out+> <out-> vol|cur pol|zer|pz",
        "Pole-zero analysis.",
    ),
    (
        ".step",
        "param <name> <start> <stop> <step>",
        "Repeat the analyses over a parameter.",
    ),
    (".plot", "<analysis> <signal> …", "Plot output selection."),
    (
        ".if",
        "(<condition>)",
        "Read the cards up to .else only if true.",
    ),
    (".elseif", "(<condition>)", "Alternative branch of an .if."),
    (".else", "", "The branch read when no condition held."),
    (".endif", "", "End of an .if block."),
    (
        ".control",
        "",
        "Interactive command block, closed by .endc.",
    ),
    (".endc", "", "End of a .control block."),
    (".temp", "<celsius>", "Simulation temperature."),
    (
        ".four",
        "<freq> <signal> …",
        "Fourier analysis of a transient.",
    ),
    (".end", "", "End of the deck."),
];

pub(crate) fn directive_signature_at(
    source: &str,
    cursor_char_index: usize,
) -> Option<(String, String)> {
    let cursor_byte = source
        .char_indices()
        .nth(cursor_char_index)
        .map_or(source.len(), |(byte, _)| byte);
    let line_start = source[..cursor_byte]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let line_end = source[cursor_byte..]
        .find('\n')
        .map_or(source.len(), |index| cursor_byte + index);
    let directive = source[line_start..line_end].split_whitespace().next()?;
    DOT_COMMANDS
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case(directive))
        .map(|(name, signature, documentation)| {
            let title = if signature.is_empty() {
                (*name).to_owned()
            } else {
                format!("{name} {signature}")
            };
            (title, (*documentation).to_owned())
        })
}

/// Whether the open popover only says which pin comes next, nothing being
/// typed: it then takes Esc alone, so ⇥ and ↑↓ still reach the line.
const PINS_ONLY: &str = "netlist_completion_pins_only";

/// Pre-consume the popover's keys. Must run *before* the `TextEdit` so
/// ⇥ and ↑↓ never reach the buffer while the popover is open. While the
/// editor's right-click menu is open (`menu_open`) the keys are the menu's,
/// and the popover takes none. Returns (move_delta, accept, dismiss).
pub fn consume_keys(ui: &Ui, state: &NetlistDocumentState, menu_open: bool) -> (i32, bool, bool) {
    if !state.completion_open || menu_open {
        return (0, false, false);
    }
    let pins_only = ui
        .ctx()
        .data(|data| data.get_temp::<bool>(egui::Id::new(PINS_ONLY)))
        .unwrap_or(false);
    if pins_only {
        let dismiss = ui
            .ctx()
            .input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape));
        return (0, false, dismiss);
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
    state: &mut NetlistDocumentState,
    anchor: CompletionAnchor,
    buffer: &str,
    keys: (i32, bool, bool),
) -> Option<(usize, usize, String, usize)> {
    let (move_delta, accept, dismiss) = keys;

    if dismiss {
        state.completion_open = false;
        state.completion_dismissed_at = Some(state.revision);
        return None;
    }

    // Only complete while the editor owns focus and the caret is on screen.
    let Some(screen_position) = anchor.screen_position.filter(|_| anchor.focused) else {
        state.completion_open = false;
        return None;
    };
    if state.completion_dismissed_at == Some(state.revision) {
        state.completion_open = false;
        return None;
    }

    let caret = anchor.caret_char_index;
    // After a blank on an element line the next pin is what is offered.
    let (token_start, token, line_start) =
        token_at(buffer, caret).unwrap_or((caret, String::new(), usize::MAX));
    let pins = pin_help(state, buffer, token_start);
    if token.is_empty() && pins.is_none() {
        state.completion_open = false;
        return None;
    }

    let candidates = collect_candidates(state, &token, token_start == line_start, pins);
    if candidates.is_empty() {
        state.completion_open = false;
        return None;
    }
    ui.ctx()
        .data_mut(|data| data.insert_temp(egui::Id::new(PINS_ONLY), token.is_empty()));

    state.completion_index =
        (state.completion_index as i32 + move_delta).rem_euclid(candidates.len() as i32) as usize;
    let selected = state.completion_index.min(candidates.len() - 1);

    let clicked = draw_popover(
        ui,
        screen_position,
        &candidates,
        selected,
        !token.is_empty(),
    );
    let take = if accept { Some(selected) } else { clicked };

    if let Some(index) = take {
        let chosen = &candidates[index];
        let byte_start = char_to_byte(buffer, token_start);
        let byte_end = char_to_byte(buffer, caret);
        let placed_caret = token_start + chosen.insert.chars().count();
        state.completion_open = false;
        return Some((byte_start, byte_end, chosen.insert.clone(), placed_caret));
    }

    state.completion_open = true;
    None
}

/// After a completion splice, park the caret right after the insertion.
///
/// The splice reaches the editor as a source change, so the caret has to
/// arrive the same way — as a request the editor applies once it has
/// synchronized against the new bytes.
pub fn place_caret(ui: &Ui, editor_id: egui::Id, char_index: usize) {
    let ctx = ui.ctx();
    crate::workbench::documents::text_editor_commands::queue_caret_char_index(
        ctx, editor_id, char_index,
    );
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

/// What a position among an element's pins is: the pins in order, the one
/// the caret is on, and, for a subcircuit's instance, the subcircuits it
/// may be of, each with the name of that port.
struct PinHelp {
    /// `M1: drain gate source bulk`, or `X1 → amp: in out vdd vss`.
    order: String,
    /// The name the pin goes by.
    pin: String,
    /// One-based position of the pin, and how many there are.
    position: (usize, usize),
    /// For a subcircuit's instance: each subcircuit it may be of, its port
    /// there and how many ports it has.
    ports: Vec<(String, String, usize)>,
}

/// The terminals an element of this letter connects, in the order its
/// line writes them.
fn device_terminals(letter: char) -> &'static [&'static str] {
    match letter.to_ascii_uppercase() {
        'R' | 'C' | 'L' | 'V' | 'I' | 'B' | 'F' | 'H' | 'W' => &["n+", "n-"],
        'D' => &["anode", "cathode"],
        'Q' => &["collector", "base", "emitter", "substrate"],
        'M' => &["drain", "gate", "source", "bulk"],
        'J' | 'Z' => &["drain", "gate", "source"],
        'E' | 'G' | 'S' => &["n+", "n-", "control+", "control-"],
        'T' => &["port 1+", "port 1-", "port 2+", "port 2-"],
        _ => &[],
    }
}

/// Which pin of the element on its line the token starting at char
/// `token_start` of `buffer` is written for, when it is one: a subcircuit's
/// port, by the subcircuit the line names or, before it names one, each
/// subcircuit in scope with a port there; or a device's terminal.
fn pin_help(state: &NetlistDocumentState, buffer: &str, token_start: usize) -> Option<PinHelp> {
    let start = char_to_byte(buffer, token_start);
    let line_start = buffer[..start].rfind('\n').map_or(0, |at| at + 1);
    if line_start == 0 {
        return None;
    }
    let line_end = buffer[start..]
        .find('\n')
        .map_or(buffer.len(), |at| start + at);
    let before = buffer[line_start..start]
        .split_whitespace()
        .collect::<Vec<_>>();
    let head = *before.first()?;
    let letter = head.chars().next().filter(char::is_ascii_alphabetic)?;
    let written = before.len() - 1;
    if letter.eq_ignore_ascii_case(&'x') {
        let project = state.project_index.last_built()?;
        let scope = project.scope_at(
            state.active_dependency_identity.as_deref(),
            state.cursor_line + 1,
        );
        let subcircuits =
            project.defined_in_reach(scope, &[rspice_core::netlist::DeckSymbolKind::Subcircuit]);
        let words = buffer[line_start..line_end]
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>();
        let named = subcircuits
            .iter()
            .filter(|subcircuit| {
                words
                    .iter()
                    .any(|word| word.eq_ignore_ascii_case(&subcircuit.name))
            })
            .copied()
            .collect::<Vec<_>>();
        let masters = if named.is_empty() { subcircuits } else { named };
        let ports = masters
            .iter()
            .filter_map(|master| {
                let port = project.ports_of(master).get(written)?;
                Some((
                    master.name.clone(),
                    port.clone(),
                    project.ports_of(master).len(),
                ))
            })
            .collect::<Vec<_>>();
        let (master, pin, _) = ports.first().cloned()?;
        let order = masters
            .iter()
            .find(|candidate| candidate.name == master)
            .map(|master| project.ports_of(master).to_vec())
            .unwrap_or_default();
        return Some(PinHelp {
            order: format!("{head} → {master}: {}", order.join(" ")),
            pin,
            position: (written + 1, order.len()),
            ports,
        });
    }
    let terminals = device_terminals(letter);
    let pin = terminals.get(written)?;
    Some(PinHelp {
        order: format!("{head}: {}", terminals.join(" ")),
        pin: (*pin).to_owned(),
        position: (written + 1, terminals.len()),
        ports: Vec::new(),
    })
}

/// Filter both sources against the typed prefix.
fn collect_candidates(
    state: &NetlistDocumentState,
    token: &str,
    at_line_start: bool,
    pins: Option<PinHelp>,
) -> Vec<Candidate> {
    let mut out = Vec::new();
    let lower = token.to_ascii_lowercase();

    // On a pin: the subcircuit's port name, then the nodes the scope has,
    // each saying which pin it would connect.
    if let Some(pins) = pins {
        let (at, of) = pins.position;
        let doc = format!("{}. Pin {at} of {of} is {}.", pins.order, pins.pin);
        let mut seen = std::collections::HashSet::new();
        for (master, port, ports) in &pins.ports {
            if port.to_ascii_lowercase().starts_with(&lower)
                && seen.insert(port.to_ascii_lowercase())
            {
                out.push(Candidate {
                    kind: "pin",
                    label: port.clone(),
                    detail: format!("{master} pin {at} of {ports}"),
                    doc: doc.clone(),
                    insert: port.clone(),
                });
            }
        }
        let project = state.project_index.last_built();
        let scope = project.as_ref().map_or(0, |project| {
            project.scope_at(
                state.active_dependency_identity.as_deref(),
                state.cursor_line + 1,
            )
        });
        let nodes = project
            .as_ref()
            .map(|project| project.nodes_in(scope))
            .unwrap_or_default();
        for node in nodes {
            let node_lower = node.to_ascii_lowercase();
            if node_lower.starts_with(&lower) && node_lower != lower && seen.insert(node_lower) {
                out.push(Candidate {
                    kind: "node",
                    label: node.to_owned(),
                    detail: format!("as {}", pins.pin),
                    doc: doc.clone(),
                    insert: node.to_owned(),
                });
            }
        }
        if token.is_empty() {
            out.truncate(8);
            return out;
        }
    }

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
    let project = state.project_index.last_built();
    // The models, subcircuits and functions the caret's scope sees; the
    // last parse's harvest stands in while the deck has no index yet.
    let project_symbols = project
        .as_ref()
        .map(|index| {
            let scope = index.scope_at(
                state.active_dependency_identity.as_deref(),
                state.cursor_line + 1,
            );
            index.completion_symbols_in(scope)
        })
        .unwrap_or_default();
    let harvested = if project.is_some() {
        &[][..]
    } else {
        state.symbols.as_slice()
    };
    let mut seen = std::collections::HashSet::new();
    for symbol in harvested.iter().chain(&project_symbols) {
        let name_lower = symbol.name.to_ascii_lowercase();
        if name_lower.starts_with(&lower)
            && name_lower != lower
            && seen.insert((symbol.kind, name_lower))
        {
            out.push(Candidate {
                kind: symbol.kind,
                label: symbol.name.clone(),
                detail: symbol.detail.clone(),
                doc: symbol.doc.clone(),
                insert: symbol.name.clone(),
            });
        }
    }
    // Parameter names complete inside `{expressions}` and values: the ones
    // the caret's scope sees, a subcircuit's own and its formals inside it.
    let parameters = project.as_ref().map(|index| {
        index.parameters_visible_at(
            state.active_dependency_identity.as_deref(),
            state.cursor_line + 1,
        )
    });
    for parameter in parameters.into_iter().flatten() {
        let name_lower = parameter.name.to_ascii_lowercase();
        if name_lower.starts_with(&lower) && name_lower != lower {
            out.push(Candidate {
                kind: "param",
                label: parameter.name.clone(),
                detail: parameter.detail.clone(),
                doc: format!("Deck parameter `{}` (.param).", parameter.name),
                insert: parameter.name.clone(),
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

/// The popover itself: items grid + selected-item doc footer, which says
/// ⇥ accepts when the popover takes the keys (`takes_keys`). Returns the row
/// index the pointer accepted, if any.
fn draw_popover(
    ui: &Ui,
    anchor: egui::Pos2,
    candidates: &[Candidate],
    selected: usize,
    takes_keys: bool,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut clicked = None;

    let geometry = completion_popover_geometry(anchor, ui.ctx().content_rect());

    egui::Area::new(egui::Id::new("rspice.netlist.completion"))
        .order(egui::Order::Foreground)
        .fixed_pos(geometry.position)
        .show(ui.ctx(), |ui| {
            egui::Frame::NONE
                .fill(c.bg_elevated)
                .stroke(egui::Stroke::new(1.0, c.border_strong))
                .corner_radius(3.0)
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
                            // Non-breaking blanks keep the key's words
                            // together, and off the start of a line.
                            if takes_keys {
                                format!("{doc}\u{a0}\u{a0}·\u{a0}\u{a0}⇥\u{a0}to\u{a0}accept")
                            } else {
                                doc.clone()
                            },
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
    fn dot_command_completion_includes_initcond_forms() {
        let (_, signature, documentation) = DOT_COMMANDS
            .iter()
            .find(|(command, _, _)| *command == ".initcond")
            .expect(".initcond completion is registered");
        assert!(signature.contains("ic=<value>"));
        assert!(signature.contains("file <path>"));
        assert!(documentation.contains("device initial-condition"));
    }

    #[test]
    fn signature_help_uses_the_directive_on_the_caret_line() {
        let source = ".param gain=2\n.tran 1n 10n\n";
        let caret = source.find("10n").unwrap();
        let (title, detail) = directive_signature_at(source, caret).unwrap();
        assert!(title.starts_with(".tran <step> <stop>"));
        assert_eq!(detail, "Transient analysis.");
    }

    /// What the popover offers with the caret at the end of `deck`, the
    /// deck's index built: each row's kind, label and detail, and the
    /// selected row's note.
    fn offered(deck: &str, token: &str) -> (Vec<(&'static str, String, String)>, String) {
        let mut harness = crate::workbench::code::tests::harness();
        super::super::fixtures::seed_owned_deck(
            &mut harness.model,
            &mut harness.state.netlist,
            deck,
            "* models\n",
        );
        let state = &mut harness.state.netlist;
        assert!(super::super::language::project_index_now(&harness.model, state).is_some());
        state.cursor_line = deck.matches('\n').count();
        let caret = deck.chars().count() - token.chars().count();
        let pins = pin_help(state, deck, caret);
        let rows = collect_candidates(state, token, false, pins);
        let note = rows.first().map(|row| row.doc.clone()).unwrap_or_default();
        (
            rows.into_iter()
                .map(|row| (row.kind, row.label, row.detail))
                .collect(),
            note,
        )
    }

    const SUBCIRCUIT_DECK: &str = "amp test\n.include \"models/resistor.inc\"\n\
        .subckt amp in out vdd\nR1 in out 1k\n.ends amp\nV1 a 0 1\n";

    #[test]
    fn after_two_nodes_of_an_instance_the_subcircuits_third_port_is_offered() {
        let (rows, note) = offered(&format!("{SUBCIRCUIT_DECK}X1 a b "), "");
        assert_eq!(
            rows.first(),
            Some(&("pin", "vdd".to_owned(), "amp pin 3 of 3".to_owned()))
        );
        assert_eq!(note, "X1 → amp: in out vdd. Pin 3 of 3 is vdd.");
        // The deck's own nodes follow, each said as that pin.
        assert!(rows.contains(&("node", "a".to_owned(), "as vdd".to_owned())));
    }

    #[test]
    fn a_device_says_which_terminal_comes_next() {
        let (rows, note) = offered(&format!("{SUBCIRCUIT_DECK}M1 a "), "");
        assert_eq!(note, "M1: drain gate source bulk. Pin 2 of 4 is gate.");
        assert!(rows.iter().all(|(kind, _, _)| *kind == "node"));
    }

    #[test]
    fn a_name_completes_from_what_the_carets_scope_sees() {
        let deck =
            format!("{SUBCIRCUIT_DECK}.subckt buf a b\n.model nloc nmos\n.ends buf\nX2 a b am");
        let (rows, _) = offered(&deck, "am");
        assert!(rows.iter().any(|(_, label, _)| label == "amp"), "{rows:?}");
        let deck =
            format!("{SUBCIRCUIT_DECK}.subckt buf a b\n.model nloc nmos\n.ends buf\nM2 a b 0 0 nl");
        let (rows, _) = offered(&deck, "nl");
        assert!(
            !rows.iter().any(|(_, label, _)| label == "nloc"),
            "a model inside a subcircuit is not seen outside it: {rows:?}"
        );
    }
}
