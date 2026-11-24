//! Completion popover — the editor's one elevated surface.
//!
//! What it offers is what the engine reads where the caret is: the dot
//! cards, with their signatures, at the head of a line, every analysis from
//! the parser's own table of them; on an `.options` card, the names the
//! parser reads there, from its table of options; and, through the
//! engine's index of the deck, the models, subcircuits and functions the
//! caret's scope sees, its nodes and its parameters. On an element line it
//! says which pin comes next, from the subcircuit's port list or the
//! device's terminals. Anchored at the token being typed; ↑↓ select, ⇥
//! accepts, Esc dismisses until the next edit.

use egui::{Key, Modifiers, Ui};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::NetlistDocumentState;
use crate::workbench::{MessageCatalog, MessageId};

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
    /// The caret's byte in the text, read from the editor's rope.
    pub caret_byte: usize,
    /// `None` when the caret is scrolled or folded out of view. The popover
    /// closes rather than float somewhere the caret is not.
    pub screen_position: Option<egui::Pos2>,
}

/// One row offered by the popover.
#[derive(Clone)]
struct Candidate {
    kind: String,
    label: String,
    detail: String,
    doc: String,
    insert: String,
}

/// What the popover offers, row by row: the rows before the deck's nodes
/// and after them, and the nodes, read from the deck's index as each row
/// shows, so a deck of a million nodes offers every one at the cost of the
/// rows in view.
#[derive(Default)]
struct Offered {
    /// A subcircuit's ports, on a pin.
    before: Vec<Candidate>,
    nodes: Option<NodeRows>,
    after: Vec<Candidate>,
}

/// The nodes offered on a pin: the names the caret's scope sees that start
/// with the token, less the token itself and the ports offered before them.
struct NodeRows {
    project: std::sync::Arc<super::language::ProjectTables>,
    scope: usize,
    /// The names' positions in the scope's, as the index orders them.
    starting: std::ops::Range<usize>,
    /// The positions among them not offered, in order.
    skipped: Vec<usize>,
    kind: String,
    detail: String,
    doc: String,
}

impl NodeRows {
    fn len(&self) -> usize {
        self.starting.len() - self.skipped.len()
    }

    fn row(&self, row: usize) -> Option<Candidate> {
        let mut position = self.starting.start + row;
        for skipped in &self.skipped {
            if *skipped <= position {
                position += 1;
            }
        }
        if position >= self.starting.end {
            return None;
        }
        let name = self.project.node_at(self.scope, position)?;
        Some(Candidate {
            kind: self.kind.clone(),
            label: name.to_string(),
            detail: self.detail.clone(),
            doc: self.doc.clone(),
            insert: name.into_owned(),
        })
    }
}

impl Offered {
    fn rows(rows: Vec<Candidate>) -> Self {
        Self {
            after: rows,
            ..Self::default()
        }
    }

    fn len(&self) -> usize {
        self.before.len() + self.nodes.as_ref().map_or(0, NodeRows::len) + self.after.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The row at `row`, made for a node.
    fn row(&self, row: usize) -> Option<std::borrow::Cow<'_, Candidate>> {
        if let Some(candidate) = self.before.get(row) {
            return Some(std::borrow::Cow::Borrowed(candidate));
        }
        let row = row - self.before.len();
        let nodes = self.nodes.as_ref().map_or(0, NodeRows::len);
        if row < nodes {
            return self.nodes.as_ref()?.row(row).map(std::borrow::Cow::Owned);
        }
        self.after.get(row - nodes).map(std::borrow::Cow::Borrowed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CompletionPopoverGeometry {
    position: egui::Pos2,
    width: f32,
}

const POPOVER_DESIRED_WIDTH: f32 = 300.0;
const POPOVER_SCREEN_MARGIN: f32 = 8.0;

/// The dot cards that are not analyses, with their signatures. The
/// analyses are the parser's own table, `ANALYSIS_KEYWORDS`.
const DOT_COMMANDS: &[(&str, &str, MessageId)] = &[
    (
        ".meas",
        "tran|ac|dc <name> <type> <signal> …",
        MessageId::NetlistDirectiveMeas,
    ),
    (
        ".param",
        "<name>=<value> …",
        MessageId::NetlistDirectiveParam,
    ),
    (
        ".model",
        "<name> <type> (<param>=<value> …)",
        MessageId::NetlistDirectiveModel,
    ),
    (
        ".subckt",
        "<name> <ports…> [params:]",
        MessageId::NetlistDirectiveSubckt,
    ),
    (".ends", "[name]", MessageId::NetlistDirectiveEnds),
    (".include", "<path>", MessageId::NetlistDirectiveInclude),
    (".lib", "<path> <section>", MessageId::NetlistDirectiveLib),
    (
        ".options",
        "<name>=<value> …",
        MessageId::NetlistDirectiveOptions,
    ),
    (".ic", "v(<node>)=<value> …", MessageId::NetlistDirectiveIc),
    (
        ".initcond",
        "<device> ic=<value>[,<value>…] … | file <path>",
        MessageId::NetlistDirectiveInitcond,
    ),
    (
        ".nodeset",
        "v(<node>)=<value> …",
        MessageId::NetlistDirectiveNodeset,
    ),
    (".save", "<signal> …", MessageId::NetlistDirectiveSave),
    (".probe", "<signal> …", MessageId::NetlistDirectiveProbe),
    (
        ".print",
        "<analysis> <signal> …",
        MessageId::NetlistDirectivePrint,
    ),
    (".global", "<node> …", MessageId::NetlistDirectiveGlobal),
    (
        ".func",
        "<name>(<args>) {<expression>}",
        MessageId::NetlistDirectiveFunc,
    ),
    (".endl", "[section]", MessageId::NetlistDirectiveEndl),
    (
        ".plot",
        "<analysis> <signal> …",
        MessageId::NetlistDirectivePlot,
    ),
    (".if", "(<condition>)", MessageId::NetlistDirectiveIf),
    (
        ".elseif",
        "(<condition>)",
        MessageId::NetlistDirectiveElseif,
    ),
    (".else", "", MessageId::NetlistDirectiveElse),
    (".endif", "", MessageId::NetlistDirectiveEndif),
    (".control", "", MessageId::NetlistDirectiveControl),
    (".endc", "", MessageId::NetlistDirectiveEndc),
    (".end", "", MessageId::NetlistDirectiveEnd),
];

pub(crate) fn directive_signature_at(
    source: &str,
    cursor_char_index: usize,
    messages: MessageCatalog,
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
    if let Some(analysis) = rspice_core::netlist::analysis_keyword(directive) {
        let name = analysis.spellings[0];
        let title = format!("{name} {}", analysis.signature);
        return Some((title.trim_end().to_owned(), analysis.description.to_owned()));
    }
    DOT_COMMANDS
        .iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case(directive))
        .map(|(name, signature, documentation)| {
            let title = if signature.is_empty() {
                (*name).to_owned()
            } else {
                format!("{name} {signature}")
            };
            (title, messages.text(*documentation))
        })
}

/// Whether the open popover only says which pin comes next, nothing being
/// typed: it then takes Esc alone, so ⇥ and ↑↓ still reach the line.
const PINS_ONLY: &str = "netlist_completion_pins_only";

/// The token the popover's list was made for in the frame before.
const SHOWN_FOR: &str = "netlist_completion_shown_for";

/// One row of the popover, and how many show at once; the rest of the list
/// scrolls.
const ROW_HEIGHT: f32 = 24.0;
const VISIBLE_ROWS: usize = 8;

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
    buffer: &crate::state::RevisedText,
    keys: (i32, bool, bool),
    messages: MessageCatalog,
) -> Option<(usize, usize, String, usize)> {
    let (move_delta, accept, dismiss) = keys;

    if dismiss || state.shown_language_off() {
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

    let caret = anchor.caret_byte.min(buffer.len());
    // Only the card the caret is on is read, as the deck reads there; what
    // is found in it is placed back by the byte it starts at.
    let (card, from) = card_around(buffer, caret);
    let at = caret - from;
    // After a blank on an element line the next pin is what is offered.
    let (token_start, token, line_start) =
        token_at(&card, at).unwrap_or((at, String::new(), usize::MAX));
    let pins = pin_help(state, &card, token_start, messages);
    let parameters = parameter_site(state, &card, token_start);
    let unprompted = parameters
        .as_ref()
        .is_some_and(ParameterSite::offers_unprompted);
    if token.is_empty() && pins.is_none() && !unprompted {
        state.completion_open = false;
        return None;
    }

    let candidates = collect_candidates(
        state,
        &token,
        token_start == line_start,
        pins,
        parameters,
        messages,
    );
    if candidates.is_empty() {
        state.completion_open = false;
        return None;
    }
    ui.ctx()
        .data_mut(|data| data.insert_temp(egui::Id::new(PINS_ONLY), token.is_empty()));
    // A list the typing changed shows its selected row, as a key moving the
    // selection does; the wheel scrolls it freely otherwise.
    let shown = (from + token_start, token.clone());
    let retyped = ui.ctx().data_mut(|data| {
        let id = egui::Id::new(SHOWN_FOR);
        let before = data.get_temp::<(usize, String)>(id);
        data.insert_temp(id, shown.clone());
        before != Some(shown)
    });

    state.completion_index = (state.completion_index as i64 + i64::from(move_delta))
        .rem_euclid(candidates.len() as i64) as usize;
    let selected = state.completion_index.min(candidates.len() - 1);

    let clicked = draw_popover(
        ui,
        screen_position,
        &candidates,
        selected,
        move_delta != 0 || retyped,
        !token.is_empty(),
        messages,
    );
    let take = if accept { Some(selected) } else { clicked };

    if let Some(chosen) = take.and_then(|index| candidates.row(index)) {
        // A token is ASCII, a byte to a character.
        let placed_caret = anchor.caret_char_index - token.len() + chosen.insert.chars().count();
        state.completion_open = false;
        return Some((
            from + token_start,
            caret,
            chosen.insert.clone(),
            placed_caret,
        ));
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

/// The part of `buffer` the popover reads for the caret at byte `caret`, and
/// the byte it starts at: the card the caret is on, from the line break
/// before its first line to the end of the caret's line, which reads as the
/// whole deck does there (see [`card_before`]). A deck held as a rope is not
/// put in one piece for it.
fn card_around(
    buffer: &crate::state::RevisedText,
    caret: usize,
) -> (std::borrow::Cow<'_, str>, usize) {
    let line_from = |start: usize| buffer.slice(start..buffer.line_end_at(start));
    let mut head = buffer.line_start_at(caret);
    while head > 0 && line_from(head).trim_start().starts_with('+') {
        let mut above = head;
        loop {
            let previous = buffer.line_start_at(above - 1);
            let text = buffer.slice(previous..above - 1);
            let text = text.trim();
            above = previous;
            if previous == 0 || !(text.is_empty() || text.starts_with('*')) {
                break;
            }
        }
        head = above;
    }
    let from = head.saturating_sub(1);
    (buffer.slice(from..buffer.line_end_at(caret)), from)
}

/// The identifier token ending at the caret's byte `caret`: `(token's
/// first byte, text, byte of its line's first non-blank)`. `None` when the
/// caret doesn't trail a token. Only the caret's line is read, and on a
/// dash-joined name the card it continues.
fn token_at(buffer: &str, caret: usize) -> Option<(usize, String, usize)> {
    let before = buffer.get(..caret)?;
    let bytes = before.as_bytes();
    let is_token_byte =
        |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'.' || byte == b'!';
    let mut start = bytes
        .iter()
        .rposition(|&byte| !is_token_byte(byte))
        .map_or(0, |at| at + 1);
    if start == caret {
        return None; // caret not at the end of a token
    }
    // On an `.options` card a name joins words with `-`, as the parser reads
    // `NONLIN-TRAN`: the token starts at its first word.
    let joined = |at: usize| {
        let mut word = at - 1;
        while word > 0 && (bytes[word - 1].is_ascii_alphanumeric() || bytes[word - 1] == b'_') {
            word -= 1;
        }
        (word < at - 1).then_some(word)
    };
    let on_options_card = || {
        let card = card_before(buffer, start);
        card.split_whitespace()
            .next()
            .is_some_and(rspice_core::netlist::is_options_card)
    };
    if start > 0 && bytes[start - 1] == b'-' && on_options_card() {
        while start > 0
            && bytes[start - 1] == b'-'
            && let Some(word) = joined(start)
        {
            start = word;
        }
    }
    // The line's first non-whitespace position (dot-command context check).
    let line_begin = before[..start].rfind('\n').map_or(0, |at| at + 1);
    let first_nonws = before[line_begin..start]
        .char_indices()
        .find(|(_, ch)| !ch.is_whitespace())
        .map_or(start, |(at, _)| line_begin + at);
    Some((start, before[start..].to_owned(), first_nonws))
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
fn device_terminals(letter: char) -> &'static [MessageId] {
    use MessageId::{
        NetlistTerminalAnode as ANODE, NetlistTerminalBase as BASE, NetlistTerminalBulk as BULK,
        NetlistTerminalCathode as CATHODE, NetlistTerminalCollector as COLLECTOR,
        NetlistTerminalControlNegative as CONTROL_MINUS,
        NetlistTerminalControlPositive as CONTROL_PLUS, NetlistTerminalDrain as DRAIN,
        NetlistTerminalEmitter as EMITTER, NetlistTerminalGate as GATE,
        NetlistTerminalNegative as MINUS, NetlistTerminalPortOneNegative as PORT_1_MINUS,
        NetlistTerminalPortOnePositive as PORT_1_PLUS,
        NetlistTerminalPortTwoNegative as PORT_2_MINUS,
        NetlistTerminalPortTwoPositive as PORT_2_PLUS, NetlistTerminalPositive as PLUS,
        NetlistTerminalSource as SOURCE, NetlistTerminalSubstrate as SUBSTRATE,
    };
    match letter.to_ascii_uppercase() {
        'R' | 'C' | 'L' | 'V' | 'I' | 'B' | 'F' | 'H' | 'W' => &[PLUS, MINUS],
        'D' => &[ANODE, CATHODE],
        'Q' => &[COLLECTOR, BASE, EMITTER, SUBSTRATE],
        'M' => &[DRAIN, GATE, SOURCE, BULK],
        'J' | 'Z' => &[DRAIN, GATE, SOURCE],
        'E' | 'G' | 'S' => &[PLUS, MINUS, CONTROL_PLUS, CONTROL_MINUS],
        'T' => &[PORT_1_PLUS, PORT_1_MINUS, PORT_2_PLUS, PORT_2_MINUS],
        _ => &[],
    }
}

/// Which pin of the element on its line the token starting at byte
/// `start` of `buffer` is written for, when it is one: a subcircuit's
/// port, by the subcircuit the line names or, before it names one, each
/// subcircuit in scope with a port there; or a device's terminal.
fn pin_help(
    state: &NetlistDocumentState,
    buffer: &str,
    start: usize,
    messages: MessageCatalog,
) -> Option<PinHelp> {
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
        let scope = project.scope_at(state.front.include(), state.cursor_line + 1);
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
                    .any(|word| word.eq_ignore_ascii_case(&subcircuit.name()))
            })
            .copied()
            .collect::<Vec<_>>();
        let masters = if named.is_empty() { subcircuits } else { named };
        let ports = masters
            .iter()
            .filter_map(|master| {
                let ports = project.ports_of(master);
                let port = ports.get(written)?.to_string();
                Some((master.name().into_owned(), port, ports.len()))
            })
            .collect::<Vec<_>>();
        let (master, pin, _) = ports.first().cloned()?;
        let order = masters
            .iter()
            .find(|candidate| *candidate.name() == *master)
            .map(|master| {
                project
                    .ports_of(master)
                    .into_iter()
                    .map(std::borrow::Cow::into_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        return Some(PinHelp {
            order: format!("{head} → {master}: {}", order.join(" ")),
            pin,
            position: (written + 1, order.len()),
            ports,
        });
    }
    let terminals = device_terminals(letter)
        .iter()
        .map(|terminal| messages.text(*terminal))
        .collect::<Vec<_>>();
    let pin = terminals.get(written)?;
    Some(PinHelp {
        order: format!("{head}: {}", terminals.join(" ")),
        pin: pin.clone(),
        position: (written + 1, terminals.len()),
        ports: Vec::new(),
    })
}

/// The parameters the engine reads where a name is being written: on a
/// `.model` card, its type's, from the engine's table of model parameters;
/// on an element's line past its pins and model, the instance parameters
/// its device reads, from the table its build checks the line against.
enum ParameterSite {
    Model {
        /// The card's type, as the card writes it.
        written_type: String,
        parameters: &'static [rspice_core::library::ModelParameterDefault],
    },
    Instance {
        /// The element's name, as its line writes it.
        element: String,
        parameters: &'static [&'static str],
    },
    /// On an `.options` card, the names the parser reads where the caret
    /// is: outside any package, or in the package the card last opened.
    Options { package: Option<&'static str> },
}

impl ParameterSite {
    /// Whether the site's names are offered before any is typed: an
    /// `.options` card's are, as a list of what the card can set.
    fn offers_unprompted(&self) -> bool {
        matches!(self, Self::Options { .. })
    }
}

/// Where a name started at byte `start` of an `.options` card goes: outside
/// any package, or in the package the card last opened. A word that names a
/// package opens it, as the parser reads the card; one that opens only
/// before its own names does so when such a name follows it, or nothing yet.
/// `None` where a value goes, after `=`.
fn options_site(buffer: &str, start: usize, words: &[&str]) -> Option<ParameterSite> {
    if buffer[..start].trim_end().ends_with('=') {
        return None;
    }
    let mut package = None;
    for (at, word) in words.iter().enumerate().skip(1) {
        let Some(found) = rspice_core::netlist::option_package(&word.to_ascii_uppercase()) else {
            continue;
        };
        let opens = found.opens_alone
            || words
                .get(at + 1)
                .is_none_or(|next| found.lists(&next.to_ascii_uppercase()));
        if opens {
            package = Some(found.name);
        }
    }
    Some(ParameterSite::Options { package })
}

/// The card the token starting at byte `start` of `buffer` is on, from the
/// start of its first line up to the token: a line `+` continues is read
/// with the line it continues, past any comment between them.
fn card_before(buffer: &str, start: usize) -> &str {
    let mut head = buffer[..start].rfind('\n').map_or(0, |at| at + 1);
    while head > 0 && buffer[head..].trim_start().starts_with('+') {
        let mut above = head;
        loop {
            let previous = buffer[..above - 1].rfind('\n').map_or(0, |at| at + 1);
            let text = buffer[previous..above - 1].trim();
            above = previous;
            if previous == 0 || !(text.is_empty() || text.starts_with('*')) {
                break;
            }
        }
        head = above;
    }
    &buffer[head..start]
}

/// The level a `.model` card states before the caret, if it states one.
fn stated_level(card: &str) -> Option<u32> {
    let upper = card.to_ascii_uppercase();
    upper.match_indices("LEVEL").find_map(|(at, word)| {
        let starts_word = upper[..at]
            .chars()
            .next_back()
            .is_none_or(|before| !before.is_ascii_alphanumeric() && before != '_');
        let value = upper[at + word.len()..]
            .trim_start()
            .strip_prefix('=')?
            .trim_start();
        let digits = value
            .find(|ch: char| !ch.is_ascii_digit())
            .unwrap_or(value.len());
        starts_word.then(|| value[..digits].parse().ok()).flatten()
    })
}

/// Which parameters a name started at byte `start` of `buffer` would be,
/// when it starts a field: the token follows a blank, `(` or `,`, never `=`
/// or `{`, so a value or an expression is never offered a parameter.
fn parameter_site(
    state: &NetlistDocumentState,
    buffer: &str,
    start: usize,
) -> Option<ParameterSite> {
    let before = buffer[..start].chars().next_back()?;
    if !(before.is_whitespace() || before == '(' || before == ',') {
        return None;
    }
    let card = card_before(buffer, start);
    let words = card
        .split(|ch: char| ch.is_whitespace() || matches!(ch, '(' | ')' | ','))
        .map(|word| word.trim_start_matches('+'))
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    let head = *words.first()?;
    if rspice_core::netlist::is_options_card(head) {
        return options_site(buffer, start, &words);
    }
    if head.eq_ignore_ascii_case(".model") {
        // Past the card's name and type.
        let written_type = *words.get(2)?;
        let defaults = rspice_core::library::device_model_defaults(written_type)?;
        if stated_level(card).is_some_and(|level| level != defaults.level.unwrap_or(1)) {
            return None;
        }
        return Some(ParameterSite::Model {
            written_type: written_type.to_owned(),
            parameters: defaults.parameters,
        });
    }
    let letter = head.chars().next().filter(char::is_ascii_alphabetic)?;
    let parameters = rspice_core::engine::native_instance_parameters(letter);
    if parameters.is_empty() {
        return None;
    }
    // Past the pins and the model: more words than the device has pins, or
    // a word already naming a model the caret's scope sees.
    let past_pins = words.len() - 1 > device_terminals(letter).len();
    let names_a_model = || {
        state.project_index.last_built().is_some_and(|project| {
            let scope = project.scope_at(state.front.include(), state.cursor_line + 1);
            let models =
                project.defined_in_reach(scope, &[rspice_core::netlist::DeckSymbolKind::Model]);
            words[1..].iter().any(|word| {
                models
                    .iter()
                    .any(|model| model.name().eq_ignore_ascii_case(word))
            })
        })
    };
    (past_pins || names_a_model()).then(|| ParameterSite::Instance {
        element: head.to_owned(),
        parameters,
    })
}

/// `value` as a card would write it: plainly, or with an exponent when it
/// is very small or very large.
fn card_number(value: f64) -> String {
    if value != 0.0 && !(1e-3..1e6).contains(&value.abs()) {
        format!("{value:e}")
    } else {
        format!("{value}")
    }
}

/// The rows a parameter site offers for the typed prefix `lower`: each
/// inserts the name and its `=`, in the case the writer is typing in.
fn parameter_candidates(
    site: &ParameterSite,
    token: &str,
    messages: MessageCatalog,
) -> Vec<Candidate> {
    let lower = token.to_ascii_lowercase();
    let spelled = |name: &str| {
        if token.chars().any(|ch| ch.is_ascii_uppercase()) {
            format!("{name}=")
        } else {
            format!("{}=", name.to_ascii_lowercase())
        }
    };
    let kind = messages.text(MessageId::NetlistCompletionParameter);
    let offered = |name: &str| name.to_ascii_lowercase().starts_with(&lower);
    match site {
        ParameterSite::Model {
            written_type,
            parameters,
        } => parameters
            .iter()
            .filter(|parameter| offered(parameter.name))
            .map(|parameter| Candidate {
                kind: kind.clone(),
                label: parameter.name.to_owned(),
                detail: format!("{} {}", card_number(parameter.default), parameter.unit)
                    .trim_end()
                    .to_owned(),
                doc: messages.format(
                    MessageId::NetlistCompletionModelParameterNote,
                    &[
                        ("type", written_type),
                        ("name", parameter.name),
                        ("description", parameter.description),
                    ],
                ),
                insert: spelled(parameter.name),
            })
            .collect(),
        ParameterSite::Instance {
            element,
            parameters,
        } => parameters
            .iter()
            .filter(|name| offered(name))
            .map(|name| Candidate {
                kind: kind.clone(),
                label: (*name).to_owned(),
                detail: element.clone(),
                doc: messages.format(
                    MessageId::NetlistCompletionInstanceParameterNote,
                    &[("element", element), ("name", name)],
                ),
                insert: spelled(name),
            })
            .collect(),
        ParameterSite::Options { package } => option_candidates(*package, token, messages),
    }
}

/// The rows an `.options` card offers for `token` in `package`: the names
/// the parser reads there, then the packages the card can open, each in
/// alphabetical order. A name is offered by the spelling being typed, the
/// one typed whole or else the first the typing starts, and inserted with
/// its `=`; a package with the blank after it. Either is written in the
/// case the writer is typing in.
fn option_candidates(
    package: Option<&'static str>,
    token: &str,
    messages: MessageCatalog,
) -> Vec<Candidate> {
    let upper = token.to_ascii_uppercase();
    let spelled = |name: &str, after: char| {
        if token.chars().any(|ch| ch.is_ascii_uppercase()) {
            format!("{name}{after}")
        } else {
            format!("{}{after}", name.to_ascii_lowercase())
        }
    };
    let typed = |spellings: &'static [&'static str]| {
        spellings
            .iter()
            .find(|spelling| **spelling == upper)
            .or_else(|| {
                spellings
                    .iter()
                    .find(|spelling| spelling.starts_with(&upper))
            })
            .copied()
    };
    let mut names = rspice_core::netlist::OPTION_NAMES
        .iter()
        .filter(|row| row.scope.admits(package))
        .filter_map(|row| typed(row.spellings).map(|name| (row, name)))
        .map(|(row, name)| Candidate {
            kind: messages.text(MessageId::NetlistCompletionOption),
            label: name.to_owned(),
            detail: package.unwrap_or_default().to_owned(),
            doc: option_note(row, name, messages),
            insert: spelled(name, '='),
        })
        .collect::<Vec<_>>();
    names.sort_by(|a, b| a.label.cmp(&b.label));
    let mut packages = rspice_core::netlist::OPTION_PACKAGES
        .iter()
        .filter(|found| found.name.starts_with(&upper))
        .map(|found| Candidate {
            kind: messages.text(MessageId::NetlistCompletionOptionPackage),
            label: found.name.to_owned(),
            detail: String::new(),
            doc: package_note(found, messages),
            insert: spelled(found.name, ' '),
        })
        .collect::<Vec<_>>();
    packages.sort_by(|a, b| a.label.cmp(&b.label));
    names.extend(packages);
    names
}

/// What the option table holds of `row`, offered as `spelling`: where the
/// parser reads it, its other spellings, and whether a number may end it.
/// A spelling another row reads elsewhere (TIMEINT's RELTOL) is another
/// name there, so only this row's places are said. The table holds no
/// value or meaning, so neither is said.
fn option_note(
    row: &rspice_core::netlist::OptionName,
    spelling: &str,
    messages: MessageCatalog,
) -> String {
    let outside = row.scope.admits(None);
    let after = rspice_core::netlist::OPTION_PACKAGES
        .iter()
        .filter(|found| row.scope.admits(Some(found.name)))
        .map(|found| found.name)
        .collect::<Vec<_>>();
    let packages = match after.split_last() {
        Some((last, [])) => (*last).to_owned(),
        Some((last, rest)) => messages.format(
            MessageId::NetlistCompletionOptionPackages,
            &[("packages", &rest.join(", ")), ("last", last)],
        ),
        None => String::new(),
    };
    let mut sentences = vec![match (outside, after.is_empty()) {
        (true, true) => messages.text(MessageId::NetlistCompletionOptionReadOutside),
        (true, false) => messages.format(
            MessageId::NetlistCompletionOptionReadAnywhere,
            &[("packages", &packages)],
        ),
        (false, _) => messages.format(
            MessageId::NetlistCompletionOptionReadIn,
            &[("packages", &packages)],
        ),
    }];
    let others = row
        .spellings
        .iter()
        .filter(|other| **other != spelling)
        .copied()
        .collect::<Vec<_>>();
    if !others.is_empty() {
        sentences.push(messages.format(
            MessageId::NetlistCompletionOptionSpellings,
            &[("spellings", &others.join(", "))],
        ));
    }
    if row.numbered {
        sentences.push(messages.format(
            MessageId::NetlistCompletionOptionNumbered,
            &[("example", &format!("{spelling}2"))],
        ));
    }
    sentences.join(" ")
}

/// What the option table holds of `package`: how many names it lists,
/// how many others are read after it, and, for one that opens only before
/// a name of its own, that it does.
fn package_note(package: &rspice_core::netlist::OptionPackage, messages: MessageCatalog) -> String {
    use rspice_core::netlist::{OPTION_NAMES, OptionScope};
    let own = OPTION_NAMES
        .iter()
        .filter(|row| {
            matches!(row.scope, OptionScope::In(packages) | OptionScope::GlobalAnd(packages)
                if packages.contains(&package.name))
        })
        .count();
    let others = OPTION_NAMES
        .iter()
        .filter(|row| row.scope.admits(Some(package.name)))
        .count()
        - own;
    let mut sentences = vec![messages.counted(
        own,
        MessageId::NetlistCompletionPackageListsOne,
        MessageId::NetlistCompletionPackageListsMany,
    )];
    sentences.push(if others == 0 {
        messages.text(MessageId::NetlistCompletionPackageNoOthers)
    } else {
        messages.counted(
            others,
            MessageId::NetlistCompletionPackageOthersOne,
            MessageId::NetlistCompletionPackageOthersMany,
        )
    });
    if !package.opens_alone {
        sentences.push(if own == 1 {
            messages.text(MessageId::NetlistCompletionPackageOpensBeforeOne)
        } else {
            messages.text(MessageId::NetlistCompletionPackageOpensBeforeMany)
        });
    }
    sentences.join(" ")
}

/// Filter both sources against the typed prefix. Where a parameter is being
/// written, the parameters the engine reads there are what is offered, and
/// the deck's names only when none of them starts so.
fn collect_candidates(
    state: &NetlistDocumentState,
    token: &str,
    at_line_start: bool,
    pins: Option<PinHelp>,
    parameters: Option<ParameterSite>,
    messages: MessageCatalog,
) -> Offered {
    let lower = token.to_ascii_lowercase();
    if let Some(site) = parameters.filter(|site| !token.is_empty() || site.offers_unprompted()) {
        let rows = parameter_candidates(&site, token, messages);
        if !rows.is_empty() {
            return Offered::rows(rows);
        }
    }
    let mut before = Vec::new();
    let mut nodes = None;
    let mut out = Vec::new();

    // On a pin: the subcircuit's port name, then the nodes the scope has,
    // each saying which pin it would connect.
    if let Some(pins) = pins {
        let (at, of) = pins.position;
        let doc = messages.format(
            MessageId::NetlistCompletionPinNote,
            &[
                ("order", &pins.order),
                ("position", &at.to_string()),
                ("count", &of.to_string()),
                ("pin", &pins.pin),
            ],
        );
        let mut seen = std::collections::HashSet::new();
        for (master, port, ports) in &pins.ports {
            if port.to_ascii_lowercase().starts_with(&lower)
                && seen.insert(port.to_ascii_lowercase())
            {
                before.push(Candidate {
                    kind: messages.text(MessageId::NetlistCompletionPin),
                    label: port.clone(),
                    detail: messages.format(
                        MessageId::NetlistCompletionPinDetail,
                        &[
                            ("subcircuit", master),
                            ("position", &at.to_string()),
                            ("count", &ports.to_string()),
                        ],
                    ),
                    doc: doc.clone(),
                    insert: port.clone(),
                });
            }
        }
        // The nodes the token starts, found by a search of the index's
        // names: not the token itself, nor a port offered above.
        nodes = state.project_index.last_built().map(|project| {
            let scope = project.scope_at(state.front.include(), state.cursor_line + 1);
            let project = project.tables();
            let starting = project.nodes_starting(scope, &lower);
            let mut skipped = seen
                .iter()
                .chain(std::iter::once(&lower))
                .filter_map(|name| project.node_named(scope, name))
                .filter(|position| starting.contains(position))
                .collect::<Vec<_>>();
            skipped.sort_unstable();
            skipped.dedup();
            NodeRows {
                project,
                scope,
                starting,
                skipped,
                kind: messages.text(MessageId::NetlistCompletionNode),
                detail: messages.format(MessageId::NetlistCompletionAsPin, &[("pin", &pins.pin)]),
                doc,
            }
        });
        if token.is_empty() {
            return Offered {
                before,
                nodes,
                after: out,
            };
        }
    }

    if token.starts_with('.') {
        if !at_line_start {
            return Offered {
                before,
                nodes,
                after: out,
            };
        }
        // An analysis is offered by the spelling being typed, the usual one
        // when the typing starts it.
        for analysis in rspice_core::netlist::ANALYSIS_KEYWORDS {
            let Some(cmd) = analysis
                .spellings
                .iter()
                .find(|spelling| spelling.starts_with(&lower) && **spelling != lower)
            else {
                continue;
            };
            let signature = analysis.signature;
            out.push(Candidate {
                kind: messages.text(MessageId::NetlistCompletionAnalysis),
                label: (*cmd).to_owned(),
                detail: signature.to_owned(),
                doc: format!("{cmd} {signature} — {}", analysis.description),
                insert: format!("{cmd} "),
            });
        }
        for (cmd, signature, doc) in DOT_COMMANDS {
            if cmd.starts_with(&lower) && *cmd != lower {
                out.push(Candidate {
                    kind: messages.text(MessageId::NetlistCompletionDirective),
                    label: (*cmd).to_owned(),
                    detail: (*signature).to_owned(),
                    doc: format!("{cmd} {signature} — {}", messages.text(*doc)),
                    insert: format!("{cmd} "),
                });
            }
        }
        return Offered {
            before,
            nodes,
            after: out,
        };
    }

    // Deck symbols (models, subckts) for bare identifiers of 2+ chars.
    if token.len() < 2 {
        return Offered {
            before,
            nodes,
            after: out,
        };
    }
    let project = state.project_index.last_built();
    // The models, subcircuits and functions the caret's scope sees; the
    // last parse's harvest stands in while the deck has no index yet.
    let project_symbols = project
        .as_ref()
        .map(|index| {
            let scope = index.scope_at(state.front.include(), state.cursor_line + 1);
            index.completion_symbols_in(messages, scope)
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
                kind: symbol.kind.to_owned(),
                label: symbol.name.clone(),
                detail: symbol.detail.clone(),
                doc: symbol.doc.clone(),
                insert: symbol.name.clone(),
            });
        }
    }
    // Parameter names complete inside `{expressions}` and values: the ones
    // the caret's scope sees, a subcircuit's own and its formals inside it.
    let parameters = project
        .as_ref()
        .map(|index| index.parameters_visible_at(state.front.include(), state.cursor_line + 1));
    for parameter in parameters.into_iter().flatten() {
        let name = parameter.name();
        let name_lower = name.to_ascii_lowercase();
        if name_lower.starts_with(&lower) && name_lower != lower {
            out.push(Candidate {
                kind: messages.text(MessageId::NetlistCompletionParameter),
                label: name.to_string(),
                detail: parameter.detail().into_owned(),
                doc: messages.format(
                    MessageId::NetlistCompletionParameterNote,
                    &[("name", &name)],
                ),
                insert: name.into_owned(),
            });
        }
    }
    Offered {
        before,
        nodes,
        after: out,
    }
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

/// Where the list scrolled to `offset` scrolls so the row `selected` shows:
/// not at all when it shows already, else just far enough.
fn revealing(offset: f32, selected: usize) -> f32 {
    let top = selected as f32 * ROW_HEIGHT;
    let view = ROW_HEIGHT * VISIBLE_ROWS as f32;
    offset.min(top).max(top + ROW_HEIGHT - view)
}

/// The salt of the popover's list, which scrolls.
const ROWS: &str = "rspice.netlist.completion.rows";

/// Every word a row's kind column can show; the column is as wide as the
/// widest, so a name starts at one place in every list.
const KINDS: [MessageId; 7] = [
    MessageId::NetlistCompletionPin,
    MessageId::NetlistCompletionNode,
    MessageId::NetlistCompletionDirective,
    MessageId::NetlistCompletionAnalysis,
    MessageId::NetlistCompletionParameter,
    MessageId::NetlistCompletionOption,
    MessageId::NetlistCompletionOptionPackage,
];

/// The popover itself: items grid + selected-item doc footer, which says
/// ⇥ accepts when the popover takes the keys (`takes_keys`). The list
/// brings the selected row into view when `reveal`. Returns the row index
/// the pointer accepted, if any.
fn draw_popover(
    ui: &Ui,
    anchor: egui::Pos2,
    candidates: &Offered,
    selected: usize,
    reveal: bool,
    takes_keys: bool,
    messages: MessageCatalog,
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
                    ui.set_max_width(geometry.width);
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                    let kind_font = theme::mono(tokens::FS_0, FontWeight::Regular);
                    let kind_width = KINDS
                        .iter()
                        .map(|kind| {
                            ui.painter()
                                .layout_no_wrap(
                                    messages.text(*kind),
                                    kind_font.clone(),
                                    c.text_faint,
                                )
                                .size()
                                .x
                        })
                        .fold(0.0, f32::max);
                    let label_left = 10.0 + kind_width + 12.0;

                    // Every row the engine offers is in the list; eight show at
                    // once and the rest scroll, the selected row brought into
                    // view when a key or the typing moved it. The list is
                    // eight rows tall whatever room the popover had the frame
                    // before, when the list may have been shorter, so its last
                    // row ends whole on the note's rule.
                    let rows_id = ui.make_persistent_id(ROWS);
                    let list_height = ROW_HEIGHT * VISIBLE_ROWS as f32;
                    let mut rows = egui::ScrollArea::vertical()
                        .id_salt(ROWS)
                        .max_height(list_height)
                        .min_scrolled_height(list_height)
                        .auto_shrink([false, true]);
                    if reveal {
                        let offset = egui::scroll_area::State::load(ui.ctx(), rows_id)
                            .map_or(0.0, |state| state.offset.y);
                        rows = rows.vertical_scroll_offset(revealing(offset, selected));
                    }
                    rows.show_rows(ui, ROW_HEIGHT, candidates.len(), |ui, shown| {
                        for idx in shown {
                            let Some(candidate) = candidates.row(idx) else {
                                continue;
                            };
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), ROW_HEIGHT),
                                egui::Sense::click(),
                            );
                            response.widget_info(|| {
                                egui::WidgetInfo::labeled(
                                    egui::WidgetType::SelectableLabel,
                                    ui.is_enabled(),
                                    messages.format(
                                        MessageId::NetlistCompletionAccessible,
                                        &[
                                            ("kind", &candidate.kind),
                                            ("label", &candidate.label),
                                            ("detail", &candidate.detail),
                                        ],
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
                                &candidate.kind,
                                kind_font.clone(),
                                c.text_faint,
                            );
                            let label = ui.painter().text(
                                egui::pos2(rect.left() + label_left, rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                &candidate.label,
                                theme::mono(tokens::FS_1, FontWeight::Regular),
                                c.text,
                            );
                            // The detail takes the room the label leaves, cut
                            // at a word; the note below the list has it whole.
                            let detail_font = theme::mono(tokens::FS_0, FontWeight::Regular);
                            let room = rect.right() - 10.0 - (label.right() + 12.0);
                            ui.painter().text(
                                egui::pos2(rect.right() - 10.0, rect.center().y),
                                egui::Align2::RIGHT_CENTER,
                                crate::ui::panels::elide_at_words(
                                    ui,
                                    &candidate.detail,
                                    &detail_font,
                                    room,
                                ),
                                detail_font,
                                c.text_faint,
                            );
                            theme::paint_focus_ring(ui, &response, rect);
                        }
                    });

                    // Doc footer for the selected item.
                    let doc = candidates
                        .row(selected)
                        .map(|candidate| candidate.doc.clone())
                        .unwrap_or_default();
                    if !doc.is_empty() {
                        let galley = ui.painter().layout(
                            // Non-breaking blanks keep the key's words
                            // together, and off the start of a line.
                            if takes_keys {
                                messages
                                    .format(MessageId::NetlistCompletionAccept, &[("note", &doc)])
                            } else {
                                doc
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

    fn english() -> MessageCatalog {
        MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
    }

    #[test]
    fn dot_command_completion_includes_initcond_forms() {
        let (_, signature, documentation) = DOT_COMMANDS
            .iter()
            .find(|(command, _, _)| *command == ".initcond")
            .expect(".initcond completion is registered");
        assert!(signature.contains("ic=<value>"));
        assert!(signature.contains("file <path>"));
        assert!(
            english()
                .text(*documentation)
                .contains("initial conditions for devices")
        );
    }

    #[test]
    fn signature_help_uses_the_directive_on_the_caret_line() {
        let source = ".param gain=2\n.tran 1n 10n\n";
        let caret = source.find("10n").unwrap();
        let (title, detail) = directive_signature_at(source, caret, english()).unwrap();
        assert!(title.starts_with(".tran <step> <stop>"));
        assert_eq!(detail, "Transient analysis.");
    }

    /// What the popover offers with the caret at the end of `deck`, the
    /// deck's index built: each row's kind, label and detail, and the
    /// selected row's note.
    fn offered(deck: &str, token: &str) -> (Vec<(String, String, String)>, String) {
        let mut harness = crate::workbench::code::tests::harness();
        super::super::fixtures::seed_deck(
            &mut harness.model,
            &mut harness.state.netlist,
            deck,
            "* models\n",
        );
        let state = &mut harness.state.netlist;
        assert!(super::super::language::project_index_now(&harness.model, state).is_some());
        state.cursor_line = deck.matches('\n').count();
        let caret = deck.len() - token.len();
        let rows = rows_offered(state, deck, caret, token);
        let note = rows.first().map(|row| row.doc.clone()).unwrap_or_default();
        (
            rows.into_iter()
                .map(|row| (row.kind, row.label, row.detail))
                .collect(),
            note,
        )
    }

    /// The rows the popover offers for `token` ending at the end of `deck`,
    /// starting at char `caret`, as `show` makes them.
    fn rows_offered(
        state: &NetlistDocumentState,
        deck: &str,
        caret: usize,
        token: &str,
    ) -> Vec<Candidate> {
        let pins = pin_help(state, deck, caret, english());
        let parameters = parameter_site(state, deck, caret);
        every_row(&collect_candidates(
            state,
            token,
            false,
            pins,
            parameters,
            english(),
        ))
    }

    /// Every row `offered` offers, in order.
    fn every_row(offered: &Offered) -> Vec<Candidate> {
        (0..offered.len())
            .filter_map(|row| offered.row(row).map(std::borrow::Cow::into_owned))
            .collect()
    }

    /// What `token`, typed at the end of `deck`, would insert, row by row.
    fn inserted(deck: &str, token: &str) -> Vec<String> {
        let mut harness = crate::workbench::code::tests::harness();
        super::super::fixtures::seed_deck(
            &mut harness.model,
            &mut harness.state.netlist,
            deck,
            "* models\n",
        );
        let state = &mut harness.state.netlist;
        assert!(super::super::language::project_index_now(&harness.model, state).is_some());
        state.cursor_line = deck.matches('\n').count();
        let caret = deck.len() - token.len();
        rows_offered(state, deck, caret, token)
            .into_iter()
            .map(|row| row.insert)
            .collect()
    }

    #[test]
    fn a_model_card_completes_its_types_parameters_from_the_engines_table() {
        let (rows, note) = offered(&format!("{HEAD}.model d1 D(I"), "I");
        let param =
            |name: &str, detail: &str| ("parameter".to_owned(), name.to_owned(), detail.to_owned());
        assert_eq!(
            rows,
            [param("IS", "1e-14 A"), param("ISR", "0 A")],
            "a diode card is offered the diode's parameters"
        );
        assert_eq!(note, "D model parameter IS: saturation current.");
        // A line the card's `+` continues, past a comment, is the card's.
        let (rows, _) = offered(&format!("{HEAD}.model q1 npn (bf=50\n* gain\n+ vj"), "vj");
        assert_eq!(
            rows,
            [
                param("VJE", "0.75 V"),
                param("VJC", "0.75 V"),
                param("VJS", "0.75 V")
            ],
            "a BJT card continued is offered the BJT's parameters"
        );
        assert_eq!(
            inserted(&format!("{HEAD}.model d1 d(n"), "n"),
            ["n=", "nr="],
            "the name is written as the writer types, with its ="
        );
    }

    #[test]
    fn a_model_card_of_another_level_or_a_value_is_offered_no_parameter() {
        // The table is the level 1 MOSFET's; a level 3 card reads others.
        let (rows, _) = offered(&format!("{HEAD}.model n1 nmos level=3 v"), "v");
        assert!(rows.is_empty(), "{rows:?}");
        let (rows, _) = offered(&format!("{HEAD}.model n1 nmos (level=1 v"), "v");
        assert!(rows.iter().any(|(_, label, _)| label == "VTO"), "{rows:?}");
        // A value is never a parameter's name.
        let (rows, _) = offered(&format!("{HEAD}.model d1 D(IS=1"), "1");
        assert!(rows.is_empty(), "{rows:?}");
    }

    #[test]
    fn an_element_past_its_model_completes_the_instance_parameters_its_device_reads() {
        let deck = format!("{SUBCIRCUIT_DECK}.model nm nmos\nM1 a a 0 0 nm w");
        let (rows, note) = offered(&deck, "w");
        assert_eq!(
            rows,
            [
                ("parameter".to_owned(), "W".to_owned(), "M1".to_owned()),
                ("parameter".to_owned(), "WIDTH".to_owned(), "M1".to_owned()),
            ]
        );
        assert_eq!(note, "M1 reads W as an instance parameter.");
        // A diode's model is its third word, after its two pins.
        let deck = format!("{SUBCIRCUIT_DECK}.model dm d\nD1 a 0 dm AR");
        assert_eq!(inserted(&deck, "AR"), ["AREA="]);
        // Where its model goes, the models are what is offered.
        let deck = format!("{SUBCIRCUIT_DECK}.model dmod d\nD1 a 0 dm");
        let (rows, _) = offered(&deck, "dm");
        assert!(rows.iter().any(|(_, label, _)| label == "dmod"), "{rows:?}");
        assert!(
            !rows.iter().any(|(kind, _, _)| kind == "parameter"),
            "{rows:?}"
        );
    }

    #[test]
    fn every_row_that_matches_is_offered_however_many() {
        let nodes = (1..=12)
            .map(|index| format!("R{index} n{index} 0 1k\n"))
            .collect::<String>();
        let (rows, _) = offered(&format!("{HEAD}{nodes}R0 n"), "n");
        assert_eq!(rows.len(), 12, "{rows:?}");
    }

    /// The nodes a pin offers, found by a search of the index's names, are
    /// those a reading of every node of the deck finds: written in the
    /// caret's scope, global or ground, starting with the token in any case,
    /// each once as first written and never the token itself, in the order
    /// of their names.
    #[test]
    fn the_nodes_a_pin_offers_are_those_a_reading_of_every_node_finds() {
        const TOP: &str = ".global VDD\n.subckt amp in out\nR1 in Mid 1k\nR2 mid out 1k\n\
            .ends amp\nV1 Net_A 0 1\nR3 net_a net_b 1k\nR4 NET_B nb2 1k\nR5 n1 vdd 1k\n";
        const INSIDE: &str = ".global VDD\n.subckt amp in out\nR1 in Mid 1k\nR2 mid out 1k\n";
        let cases = [
            (TOP, ""),
            (TOP, "n"),
            (TOP, "N"),
            (TOP, "net_"),
            (TOP, "NET_A"),
            (TOP, "net_b"),
            (TOP, "nb2"),
            (TOP, "vd"),
            (TOP, "0"),
            (TOP, "x"),
            (INSIDE, ""),
            (INSIDE, "m"),
            (INSIDE, "i"),
        ];
        for (nodes, token) in cases {
            let deck = format!("{HEAD}{nodes}R9 {token}");
            let mut harness = crate::workbench::code::tests::harness();
            super::super::fixtures::seed_deck(
                &mut harness.model,
                &mut harness.state.netlist,
                &deck,
                "* models\n",
            );
            let state = &mut harness.state.netlist;
            let project = super::super::language::project_index_now(&harness.model, state)
                .expect("the deck's index");
            state.cursor_line = deck.matches('\n').count();
            let offered = rows_offered(state, &deck, deck.len() - token.len(), token)
                .into_iter()
                .filter(|row| row.kind == "node")
                .map(|row| row.label)
                .collect::<Vec<_>>();

            let scope = project.scope_at(None, state.cursor_line + 1);
            let lower = token.to_ascii_lowercase();
            let mut seen = std::collections::HashSet::new();
            let index = project.deck();
            let mut read = index
                .nodes()
                .filter(|node| {
                    node.scope() == scope
                        || node.card_kind() == rspice_core::netlist::DeckCardKind::Global
                        || node.name() == "0"
                })
                .filter(|node| seen.insert(node.name().to_ascii_lowercase()))
                .map(|node| node.name().into_owned())
                .filter(|name| {
                    let name = name.to_ascii_lowercase();
                    name.starts_with(&lower) && name != lower
                })
                .collect::<Vec<_>>();
            read.sort_by_key(|name| name.to_ascii_lowercase());
            assert_eq!(offered, read, "{token:?} after {nodes:?}");
        }
        // A name written in two cases is offered once, as first written.
        let deck = format!("{HEAD}{TOP}R9 net_");
        let (rows, _) = offered(&deck, "net_");
        let labels = rows
            .iter()
            .filter(|(kind, _, _)| kind == "node")
            .map(|(_, label, _)| label.as_str())
            .collect::<Vec<_>>();
        assert_eq!(labels, ["Net_A", "net_b"]);
    }

    #[test]
    fn a_key_scrolls_the_list_just_far_enough_to_show_the_selected_row() {
        let row = ROW_HEIGHT;
        assert_eq!(revealing(0.0, 3), 0.0, "a row in view moves nothing");
        assert_eq!(revealing(0.0, 8), row, "the ninth row scrolls one row");
        assert_eq!(
            revealing(5.0 * row, 2),
            2.0 * row,
            "a row above scrolls up to it"
        );
        assert_eq!(
            revealing(9.0 * row, 0),
            0.0,
            "wrapping to the first goes to the top"
        );
    }

    /// A deck's title and the include every deck the tests seed names.
    const HEAD: &str = "amp test\n.include \"models/resistor.inc\"\n";

    const SUBCIRCUIT_DECK: &str = "amp test\n.include \"models/resistor.inc\"\n\
        .subckt amp in out vdd\nR1 in out 1k\n.ends amp\nV1 a 0 1\n";

    #[test]
    fn after_two_nodes_of_an_instance_the_subcircuits_third_port_is_offered() {
        let (rows, note) = offered(&format!("{SUBCIRCUIT_DECK}X1 a b "), "");
        assert_eq!(
            rows.first(),
            Some(&(
                "pin".to_owned(),
                "vdd".to_owned(),
                "amp pin 3 of 3".to_owned()
            ))
        );
        assert_eq!(note, "X1 → amp: in out vdd. Pin 3 of 3 is vdd.");
        // The deck's own nodes follow, each said as that pin.
        assert!(rows.contains(&("node".to_owned(), "a".to_owned(), "as vdd".to_owned())));
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

    /// A deck's editing state, its index built, for rows offered at the end
    /// of any buffer.
    fn deck_state() -> NetlistDocumentState {
        let mut harness = crate::workbench::code::tests::harness();
        super::super::fixtures::seed_deck(
            &mut harness.model,
            &mut harness.state.netlist,
            HEAD,
            "* models
",
        );
        assert!(
            super::super::language::project_index_now(&harness.model, &mut harness.state.netlist)
                .is_some()
        );
        harness.state.netlist
    }

    /// The rows the popover offers with the caret at the end of `buffer`, the
    /// token and its site found as `show` finds them.
    fn rows_at_end(state: &NetlistDocumentState, buffer: &str) -> Vec<Candidate> {
        let caret = buffer.len();
        let (token_start, token, line_start) =
            token_at(buffer, caret).unwrap_or((caret, String::new(), usize::MAX));
        let pins = pin_help(state, buffer, token_start, english());
        let parameters = parameter_site(state, buffer, token_start);
        every_row(&collect_candidates(
            state,
            &token,
            token_start == line_start,
            pins,
            parameters,
            english(),
        ))
    }

    #[test]
    fn every_option_the_parser_reads_is_offered_where_it_reads_it() {
        use rspice_core::netlist::{OPTION_NAMES, OPTION_PACKAGES};
        let state = deck_state();
        let packages = std::iter::once(None)
            .chain(OPTION_PACKAGES.iter().map(|package| Some(package.name)))
            .collect::<Vec<_>>();
        for row in OPTION_NAMES {
            for package in packages
                .iter()
                .filter(|package| row.scope.admits(**package))
            {
                // A package that opens only before its own names is written
                // with one of them first.
                let opened = match package {
                    Some(name) => format!("{name} {} ", row.spellings[0]),
                    None => String::new(),
                };
                for spelling in row.spellings {
                    let card = format!("{HEAD}.options {opened}{spelling}");
                    let rows = rows_at_end(&state, &card);
                    assert!(
                        rows.iter().any(|row| row.label == *spelling),
                        "{spelling} in {package:?} is not offered: {:?}",
                        rows.iter().map(|row| &row.label).collect::<Vec<_>>()
                    );
                }
            }
        }
        for package in OPTION_PACKAGES {
            let rows = rows_at_end(&state, &format!("{HEAD}.options {}", package.name));
            assert!(
                rows.iter().any(|row| row.label == package.name),
                "{} is not offered",
                package.name
            );
        }
    }

    #[test]
    fn an_options_card_offers_what_the_parser_reads_where_the_caret_is() {
        let state = deck_state();
        let labels = |buffer: &str| {
            rows_at_end(&state, &format!("{HEAD}{buffer}"))
                .into_iter()
                .map(|row| row.label)
                .collect::<Vec<_>>()
        };
        // Before any typing, every name read outside a package, then the
        // packages.
        let all = labels(".options ");
        assert_eq!(all.first().map(String::as_str), Some("ABSTOL"), "{all:?}");
        assert!(all.contains(&"RELTOL".to_owned()) && all.contains(&"TIMEINT".to_owned()));
        assert!(
            !all.contains(&"DELMAX".to_owned()),
            "DELMAX is TIMEINT's alone"
        );
        // In a package, its own names; a closed package reads no others.
        let timeint = labels(".options timeint ");
        assert!(timeint.contains(&"DELMAX".to_owned()), "{timeint:?}");
        assert!(!timeint.contains(&"MAXTIMESTEP".to_owned()), "{timeint:?}");
        assert!(!timeint.contains(&"GMIN".to_owned()), "{timeint:?}");
        // An open package reads the names outside packages too.
        assert!(labels(".options device g").contains(&"GMIN".to_owned()));
        // A spelling is offered as typed; a value is never a name.
        assert_eq!(labels(".options min_r"), ["MIN_RES"]);
        assert_eq!(
            labels(".options nonlin-tr"),
            ["NONLIN-TRAN", "NONLIN-TRANSIENT"],
            "a name joined with - is one name"
        );
        assert!(labels(".options reltol=").is_empty());
        assert!(labels(".options reltol= 1").is_empty());
        let rows = rows_at_end(&state, &format!("{HEAD}.options timeint rel"));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].insert, "reltol=");
        assert_eq!(rows[0].detail, "TIMEINT");
        assert_eq!(rows[0].doc, "Read only after TIMEINT.");
        assert_eq!(
            rows_at_end(&state, &format!("{HEAD}.OPTIONS TIME"))[0].insert,
            "TIMEINT "
        );
    }

    #[test]
    fn every_analysis_the_parser_reads_is_offered_at_the_head_of_a_line() {
        let state = deck_state();
        for analysis in rspice_core::netlist::ANALYSIS_KEYWORDS {
            for spelling in analysis.spellings {
                let typed = &spelling[..spelling.len() - 1];
                let rows = rows_at_end(&state, &format!("{HEAD}{typed}"));
                assert!(
                    rows.iter()
                        .any(|row| analysis.spellings.contains(&row.label.as_str())),
                    "{spelling} is not offered for {typed}"
                );
                let (title, detail) = directive_signature_at(&format!("{spelling} "), 0, english())
                    .expect("an analysis line has its signature");
                assert!(title.starts_with(analysis.spellings[0]), "{title}");
                assert_eq!(detail, analysis.description);
            }
        }
        // A spelling not written whole offers its card by the usual one.
        let rows = rows_at_end(&state, &format!("{HEAD}.fo"));
        assert!(rows.iter().any(|row| row.label == ".four"));
        assert!(!rows.iter().any(|row| row.label == ".fourier"));
    }

    /// The popover reads only the card the caret is on, held in one piece or
    /// as the editor's rope, and finds in it what it finds reading the whole
    /// deck: the token at the caret, the line it starts and the card it is
    /// on, across continued lines, a comment and a blank line between them.
    #[test]
    fn the_card_at_the_caret_reads_as_the_whole_deck_does() {
        let deck = "* title\n.options reltol=1e-3\n+ abstol=1e-9\n\n* note\n+ nonlin-tr\r\nM1 d g s b nch\n+ w=1u l\n";
        let whole = crate::state::RevisedText::from(deck);
        let roped =
            crate::state::RevisedText::from_snapshot(&crate::state::TextSnapshot::new(deck));
        for text in [&whole, &roped] {
            for caret in 0..=deck.len() {
                let (card, from) = card_around(text, caret);
                assert_eq!(&*card, &deck[from..from + card.len()], "caret {caret}");
                let found = token_at(deck, caret);
                let part = token_at(&card, caret - from)
                    .map(|(start, token, line)| (from + start, token, from + line));
                assert_eq!(part, found, "caret {caret}");
                if let Some((start, ..)) = found {
                    assert_eq!(
                        card_before(&card, start - from),
                        card_before(deck, start),
                        "caret {caret}"
                    );
                }
            }
        }
        assert!(!roped.is_in_one_piece(), "the rope is read in place");
    }
}
