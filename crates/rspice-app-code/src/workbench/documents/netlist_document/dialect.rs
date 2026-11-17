//! How the engine reads the deck the Netlist page shows: the source it is
//! read from, the dialect its options declare, the one edit that declares
//! another, and the card that names the program that wrote it.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the reading, the declaration and the hint are used once the deck's check carries its reading"
    )
)]

use std::ops::Range;
use std::path::PathBuf;

use rspice_core::netlist::{SourceKind, comment_start, is_spectre_source};

use super::{ActiveNetlistDocument, Model, NetlistDocumentState};
use crate::workbench::MessageId;
use crate::workbench::documents::text_editor_commands::EditorSyntax;

/// Where the shown document is read from, and whether it is read as the root
/// deck: what decides, with its text, how the engine reads it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ShownSource {
    /// The file, or a name that carries its extension; empty for text that is
    /// no file, such as the generated netlist.
    pub path: PathBuf,
    pub root: bool,
}

impl ShownSource {
    /// How the engine reads `text` from here.
    pub(crate) fn kind(&self, text: &str) -> SourceKind {
        SourceKind::of(&self.path, text, self.root)
    }

    /// The syntax the editor edits the source in: Spectre for a file that is
    /// Spectre from its first line, which its name alone decides.
    pub(crate) fn syntax(&self) -> EditorSyntax {
        if self.kind("") == SourceKind::Spectre {
            EditorSyntax::Spectre
        } else {
            EditorSyntax::Spice
        }
    }
}

/// The source the page shows, over the model and the netlist editor's state;
/// `None` for the comparison, which is a diff rather than a deck.
pub(crate) fn shown_source_in(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<ShownSource> {
    if let Some(dependency) = super::active_dependency_in(model, netlist) {
        let locator = dependency.locator();
        return Some(ShownSource {
            path: PathBuf::from(
                locator
                    .native_origin()
                    .unwrap_or_else(|| locator.logical_identity()),
            ),
            root: false,
        });
    }
    let path = match netlist.active_document {
        ActiveNetlistDocument::OwnedSource => {
            let content = &model.workspace.content;
            content.netlist_source_path.clone().or_else(|| {
                content
                    .netlist_descriptor
                    .as_ref()
                    .map(|descriptor| PathBuf::from(&descriptor.artifact_name))
            })
        }
        ActiveNetlistDocument::Generated | ActiveNetlistDocument::RunSnapshot => None,
        ActiveNetlistDocument::GeneratedDiff => return None,
    };
    Some(ShownSource {
        path: path.unwrap_or_default(),
        root: true,
    })
}

/// A dialect a deck can declare: how the engine reads its cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Dialect {
    /// No declaration: the engine's own reading.
    RSpice,
    /// `.options rspice_dialect=ngspice`: ngspice's device models and number
    /// suffixes.
    Ngspice,
    /// `.options rspice_source=ltspice`: LTspice's defaults for what a card
    /// leaves out.
    Ltspice,
    /// `.options rspice_source=qspice`: QSPICE's defaults for what a card
    /// leaves out.
    Qspice,
}

impl Dialect {
    /// Every dialect a deck can be given, in the order they are offered.
    pub(crate) const CHOICES: [Self; 4] =
        [Self::RSpice, Self::Ngspice, Self::Ltspice, Self::Qspice];

    /// The option and value that declare it; RSpice is declared by neither.
    const fn declaration(self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::RSpice => None,
            Self::Ngspice => Some((DIALECT_OPTION, "ngspice")),
            Self::Ltspice => Some((SOURCE_OPTION, "ltspice")),
            Self::Qspice => Some((SOURCE_OPTION, "qspice")),
        }
    }

    /// Its name.
    pub(crate) const fn name(self) -> MessageId {
        match self {
            Self::RSpice => MessageId::NetlistDialectRspice,
            Self::Ngspice => MessageId::NetlistDialectNgspice,
            Self::Ltspice => MessageId::NetlistDialectLtspice,
            Self::Qspice => MessageId::NetlistDialectQspice,
        }
    }
}

/// The option whose value picks the device models and number reading.
const DIALECT_OPTION: &str = "rspice_dialect";
/// The option whose value names the program a deck was written for.
const SOURCE_OPTION: &str = "rspice_source";

/// How the engine reads a deck, as its last check found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeckReading {
    Declared(Dialect),
    /// `.options rspice_dialect=xyce`: Xyce's device models. Xyce's
    /// expression syntax is read only by `rspice run --spice-dialect xyce`.
    XyceDevices,
    /// The Spectre front end reads it: the file's `.scs` name or a
    /// `simulator lang=` line decides, not a choice.
    Spectre,
}

impl DeckReading {
    /// The reading of a deck read from `source` holding `text`, whose check
    /// found `options`. A deck that names two programs is read with both;
    /// the program it was written for names it.
    pub(crate) fn of(
        options: &rspice_core::netlist::SimulationOptions,
        source: &ShownSource,
        text: &str,
    ) -> Self {
        use rspice_core::SpiceDialect;
        use rspice_core::netlist::SourceSimulator;

        if is_spectre_source(&source.path, text) {
            return Self::Spectre;
        }
        match (options.spice_dialect, options.source_simulator) {
            (Some(SpiceDialect::Xyce), _) => Self::XyceDevices,
            (_, Some(SourceSimulator::Ltspice)) => Self::Declared(Dialect::Ltspice),
            (_, Some(SourceSimulator::Qspice)) => Self::Declared(Dialect::Qspice),
            (Some(SpiceDialect::Ngspice), _) => Self::Declared(Dialect::Ngspice),
            _ => Self::Declared(Dialect::RSpice),
        }
    }

    /// Its name, as the chip shows it.
    pub(crate) const fn name(self) -> MessageId {
        match self {
            Self::Declared(dialect) => dialect.name(),
            Self::XyceDevices => MessageId::NetlistDialectXyceDevices,
            Self::Spectre => MessageId::NetlistDialectSpectre,
        }
    }

    /// What the chip says of it under the pointer.
    pub(crate) const fn detail(self) -> MessageId {
        match self {
            Self::Declared(_) => MessageId::NetlistDialectDetail,
            Self::XyceDevices => MessageId::NetlistDialectXyceDetail,
            Self::Spectre => MessageId::NetlistDialectSpectreDetail,
        }
    }

    /// Whether another dialect can be chosen for a deck read so: a Spectre
    /// deck's is its file's to say.
    pub(crate) const fn choosable(self) -> bool {
        !matches!(self, Self::Spectre)
    }
}

/// One edit to a deck: the bytes at `range` replaced by `text`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeckEdit {
    pub range: Range<usize>,
    pub text: String,
}

impl DeckEdit {
    /// The edit that turns `before` into `after`: the stretch between what
    /// they share at each end.
    fn between(before: &str, after: &str) -> Option<Self> {
        if before == after {
            return None;
        }
        let prefix = before
            .char_indices()
            .zip(after.chars())
            .find(|((_, old), new)| old != new)
            .map_or(before.len().min(after.len()), |((index, _), _)| index);
        let suffix = before[prefix..]
            .chars()
            .rev()
            .zip(after[prefix..].chars().rev())
            .take_while(|(old, new)| old == new)
            .map(|(old, _)| old.len_utf8())
            .sum::<usize>();
        Some(Self {
            range: prefix..before.len() - suffix,
            text: after[prefix..after.len() - suffix].to_owned(),
        })
    }

    /// `deck` with the edit made.
    pub(crate) fn applied_to(&self, deck: &str) -> String {
        let mut edited = deck.to_owned();
        edited.replace_range(self.range.clone(), &self.text);
        edited
    }

    /// Ask the editor `editor_id`, which shows `deck`, to make this edit as
    /// one step of its own history, saved like any edit.
    pub(crate) fn queue(self, context: &egui::Context, editor_id: egui::Id, deck: &str) {
        let chars = |byte: usize| deck[..byte].chars().count();
        crate::workbench::documents::text_editor_commands::queue_text_replacement(
            context,
            editor_id,
            chars(self.range.start)..chars(self.range.end),
            self.text,
        );
    }
}

/// One `rspice_dialect=` or `rspice_source=` assignment on an options card.
#[derive(Debug, Clone)]
struct Declaration {
    /// The physical line it is on, from zero.
    line: usize,
    option: &'static str,
    /// From the option's name to the end of its value, in the line.
    assignment: Range<usize>,
    /// The value, in the line.
    value: Range<usize>,
}

/// The `.options` cards of a root deck, each as its range of physical lines
/// (the `.options` line and its `+` lines), and the dialect declarations on
/// them, read where the engine reads cards: not on the title line, not in a
/// comment.
fn options_cards(deck: &str) -> (Vec<Range<usize>>, Vec<Declaration>) {
    let mut cards: Vec<Range<usize>> = Vec::new();
    let mut declarations = Vec::new();
    let mut in_options = false;
    for (line, text) in deck.split('\n').enumerate().skip(1) {
        let code = code_of(text);
        let trimmed = code.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let indent = code.len() - trimmed.len();
        let fields_at = if trimmed.starts_with('+') {
            if !in_options {
                continue;
            }
            if let Some(card) = cards.last_mut() {
                card.end = line + 1;
            }
            indent + 1
        } else {
            let head = trimmed.split_whitespace().next().unwrap_or_default();
            in_options = OPTIONS_SPELLINGS
                .iter()
                .any(|spelling| head.eq_ignore_ascii_case(spelling));
            if !in_options {
                continue;
            }
            cards.push(line..line + 1);
            indent + head.len()
        };
        declarations.extend(assignments(&code[fields_at..]).into_iter().map(
            |(option, assignment, value)| Declaration {
                line,
                option,
                assignment: assignment.start + fields_at..assignment.end + fields_at,
                value: value.start + fields_at..value.end + fields_at,
            },
        ));
    }
    (cards, declarations)
}

/// Every spelling of the options card the engine reads.
const OPTIONS_SPELLINGS: [&str; 4] = [".options", ".option", ".opt", ".opts"];

/// A line's text before its comment, by the engine's rule for a SPICE deck.
fn code_of(line: &str) -> &str {
    let line = line.strip_suffix('\r').unwrap_or(line);
    let end = comment_start(line, SourceKind::Included).unwrap_or(line.len());
    &line[..end]
}

/// Each `rspice_dialect=` and `rspice_source=` assignment among `fields`:
/// the option, the whole assignment and the value, as byte ranges.
fn assignments(fields: &str) -> Vec<(&'static str, Range<usize>, Range<usize>)> {
    let mut found = Vec::new();
    let mut at = 0;
    while let Some(start) = fields[at..].find(|character: char| !character.is_whitespace()) {
        let name_start = at + start;
        let name_end = fields[name_start..]
            .find(|character: char| character.is_whitespace() || character == '=')
            .map_or(fields.len(), |end| name_start + end);
        let after_name = &fields[name_end..];
        let equals = name_end + (after_name.len() - after_name.trim_start().len());
        if !fields[equals..].starts_with('=') {
            at = name_end;
            continue;
        }
        let value_area = &fields[equals + 1..];
        let value_start = equals + 1 + (value_area.len() - value_area.trim_start().len());
        let value_end = fields[value_start..]
            .find(char::is_whitespace)
            .map_or(fields.len(), |end| value_start + end);
        let name = &fields[name_start..name_end];
        if let Some(option) = [DIALECT_OPTION, SOURCE_OPTION]
            .into_iter()
            .find(|option| name.eq_ignore_ascii_case(option))
            && value_end > value_start
        {
            found.push((option, name_start..value_end, value_start..value_end));
        }
        at = value_end.max(equals + 1);
    }
    found
}

/// Whether the root deck `deck` declares a dialect at all.
pub(crate) fn declares_dialect(deck: &str) -> bool {
    !options_cards(deck).1.is_empty()
}

/// The one edit that makes the root deck `deck` declare `dialect`: the
/// declaration rewritten where it stands, `.options` with it added after the
/// title line, or, for RSpice, the declaration taken out, with its card when
/// nothing else is on it. Any other declaration goes. `None` when the deck
/// declares just that.
pub(crate) fn declare(deck: &str, dialect: Dialect) -> Option<DeckEdit> {
    let (cards, declarations) = options_cards(deck);
    let wanted = dialect.declaration();
    // The declaration kept, and what of it is rewritten to what.
    let kept = wanted.and_then(|(option, value)| {
        match declarations
            .iter()
            .position(|declaration| declaration.option == option)
        {
            Some(index) => Some((index, declarations[index].value.clone(), value.to_owned())),
            None => declarations.first().map(|declaration| {
                (
                    0,
                    declaration.assignment.clone(),
                    format!("{option}={value}"),
                )
            }),
        }
    });
    let mut lines: Vec<String> = deck.split('\n').map(str::to_owned).collect();
    for (index, declaration) in declarations.iter().enumerate().rev() {
        let line = &mut lines[declaration.line];
        match &kept {
            Some((kept, range, text)) if *kept == index => line.replace_range(range.clone(), text),
            _ => line.replace_range(removal(line, &declaration.assignment), ""),
        }
    }
    // A card left with nothing on it goes, and so does an empty `+` line.
    let empty = |line: &str| {
        let code = code_of(line).trim_start();
        let rest = code.strip_prefix('+').unwrap_or_else(|| {
            code.split_once(char::is_whitespace)
                .map_or("", |(_, rest)| rest)
        });
        rest.trim().is_empty()
    };
    let mut gone = vec![false; lines.len()];
    for card in &cards {
        if card.clone().all(|line| empty(&lines[line])) {
            gone[card.clone()].fill(true);
        } else {
            for line in card.clone().skip(1) {
                gone[line] = empty(&lines[line]);
            }
        }
    }
    let mut after: Vec<&str> = lines
        .iter()
        .zip(&gone)
        .filter(|(_, gone)| !**gone)
        .map(|(line, _)| line.as_str())
        .collect();
    let added;
    if kept.is_none()
        && let Some((option, value)) = wanted
    {
        let ending = if lines[0].ends_with('\r') { "\r" } else { "" };
        added = format!(".options {option}={value}{ending}");
        after.insert(1, &added);
    }
    DeckEdit::between(deck, &after.join("\n"))
}

/// The stretch of `line` that taking out `assignment` removes: it, and the
/// blanks after it, or before it when nothing follows it.
fn removal(line: &str, assignment: &Range<usize>) -> Range<usize> {
    let blank = |character: char| character == ' ' || character == '\t';
    let tail = &line[assignment.end..];
    let after = tail.len() - tail.trim_start_matches(blank).len();
    let follows = !tail[after..].trim_end_matches('\r').is_empty();
    if follows {
        assignment.start..assignment.end + after
    } else {
        let head = &line[..assignment.start];
        head.trim_end_matches(blank).len()..assignment.end + after
    }
}

/// What a deck that declares no dialect says of the program that wrote it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DialectHint {
    pub dialect: Dialect,
    /// The card that says so.
    pub marker: &'static str,
    /// Its line, from one.
    pub line: usize,
}

/// The cards that name the program a deck was written for: LTspice writes
/// `.backanno` into every netlist it exports, and a `.control` block is
/// ngspice's. No other marker is read: a guess is worse than none.
const MARKERS: [(&str, Dialect); 2] = [
    (".backanno", Dialect::Ltspice),
    (".control", Dialect::Ngspice),
];

/// The first marker card in the root deck `deck`, when it declares no
/// dialect.
pub(crate) fn hint(deck: &str) -> Option<DialectHint> {
    if declares_dialect(deck) {
        return None;
    }
    deck.split('\n')
        .enumerate()
        .skip(1)
        .find_map(|(line, text)| {
            let head = code_of(text).split_whitespace().next()?;
            MARKERS
                .iter()
                .find(|(marker, _)| head.eq_ignore_ascii_case(marker))
                .map(|&(marker, dialect)| DialectHint {
                    dialect,
                    marker,
                    line: line + 1,
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::{MessageCatalog, UiTextLocale};

    fn deck_source(path: &str) -> ShownSource {
        ShownSource {
            path: path.into(),
            root: true,
        }
    }

    /// How the engine reads `deck`, checked as the file `amp.cir`.
    fn checked(deck: &str) -> DeckReading {
        let netlist = rspice_core::Netlist::parse(deck).expect("the deck parses");
        DeckReading::of(&netlist.options, &deck_source("amp.cir"), deck)
    }

    fn named(reading: DeckReading) -> String {
        MessageCatalog::new(UiTextLocale::EnglishUnitedStates).text(reading.name())
    }

    #[test]
    fn the_chip_names_the_dialect_the_check_read() {
        let deck = |options: &str| format!("amp\n{options}\nR1 a 0 1k\n.op\n.end\n");
        for (options, name) in [
            ("", "RSpice"),
            (".options rspice_dialect=best_available", "RSpice"),
            (".options rspice_dialect=ngspice", "ngspice"),
            (".OPTION RSPICE_SOURCE=LTSPICE", "LTspice"),
            (".options rspice_source=qspice", "QSPICE"),
            (".options rspice_dialect=xyce", "Xyce devices"),
        ] {
            assert_eq!(named(checked(&deck(options))), name, "{options}");
        }
        let messages = MessageCatalog::new(UiTextLocale::EnglishUnitedStates);
        assert_eq!(
            messages.text(checked(&deck(".options rspice_dialect=xyce")).detail()),
            "Xyce expression syntax is read only by rspice run --spice-dialect xyce"
        );
    }

    #[test]
    fn a_spectre_deck_is_read_as_spectre_and_offers_no_choice() {
        let options = rspice_core::netlist::SimulationOptions::default();
        let spectre = "simulator lang=spectre\nr1 (a 0) resistor r=1k\n";
        for (path, text) in [
            ("amp.scs", "r1 (a 0) resistor r=1k\n"),
            ("amp.cir", spectre),
        ] {
            let reading = DeckReading::of(&options, &deck_source(path), text);
            assert_eq!(reading, DeckReading::Spectre, "{path}");
            assert_eq!(named(reading), "Spectre");
            assert!(!reading.choosable());
        }
        assert!(DeckReading::XyceDevices.choosable());
    }

    #[test]
    fn a_choice_rewrites_one_declaration_in_place() {
        for (deck, dialect, after) in [
            (
                "amp\nR1 a 0 1k\n",
                Dialect::Ltspice,
                "amp\n.options rspice_source=ltspice\nR1 a 0 1k\n",
            ),
            (
                "amp\r\nR1 a 0 1k\r\n",
                Dialect::Ngspice,
                "amp\r\n.options rspice_dialect=ngspice\r\nR1 a 0 1k\r\n",
            ),
            ("amp", Dialect::Qspice, "amp\n.options rspice_source=qspice"),
            (
                "amp\n.options reltol=1e-4 rspice_dialect=ngspice\n",
                Dialect::Ltspice,
                "amp\n.options reltol=1e-4 rspice_source=ltspice\n",
            ),
            (
                "amp\n.OPTIONS RSPICE_SOURCE = LTSPICE\n",
                Dialect::Qspice,
                "amp\n.OPTIONS RSPICE_SOURCE = qspice\n",
            ),
            (
                "amp\n.options rspice_dialect=ngspice\n.options rspice_source=ltspice\n",
                Dialect::Qspice,
                "amp\n.options rspice_source=qspice\n",
            ),
            (
                "amp\n.options rspice_dialect=ngspice\nR1 a 0 1k\n",
                Dialect::RSpice,
                "amp\nR1 a 0 1k\n",
            ),
            (
                "amp\n.options rspice_dialect=ngspice ; why\n",
                Dialect::RSpice,
                "amp\n",
            ),
            (
                "amp\n.options reltol=1e-4 rspice_dialect=ngspice\n",
                Dialect::RSpice,
                "amp\n.options reltol=1e-4\n",
            ),
            (
                "amp\n.options\n+ rspice_dialect=ngspice\n+ reltol=1e-4\n",
                Dialect::RSpice,
                "amp\n.options\n+ reltol=1e-4\n",
            ),
            (
                "amp\n* .options rspice_dialect=ngspice\n",
                Dialect::Ngspice,
                "amp\n.options rspice_dialect=ngspice\n* .options rspice_dialect=ngspice\n",
            ),
        ] {
            let edit = declare(deck, dialect).expect("the deck changes");
            assert_eq!(edit.applied_to(deck), after, "{deck:?} to {dialect:?}");
        }
        assert_eq!(
            declare("amp\n.options rspice_source=ltspice\n", Dialect::Ltspice),
            None
        );
        assert_eq!(
            declare("amp\n* .options rspice_source=ltspice\n", Dialect::RSpice),
            None
        );
    }

    #[test]
    fn the_engine_reads_every_choice_as_chosen() {
        for deck in [
            "amp\nR1 a 0 1k\n.op\n.end\n",
            "amp\n.options reltol=1e-4 rspice_dialect=xyce\nR1 a 0 1k\n.op\n.end\n",
            "amp\n.options rspice_source=ltspice\n+ reltol=1e-4\nR1 a 0 1k\n.op\n.end\n",
        ] {
            for dialect in Dialect::CHOICES {
                let chosen = declare(deck, dialect)
                    .map_or_else(|| deck.to_owned(), |edit| edit.applied_to(deck));
                assert_eq!(
                    checked(&chosen),
                    DeckReading::Declared(dialect),
                    "{deck:?} as {dialect:?}: {chosen:?}"
                );
            }
        }
    }

    #[test]
    fn a_deck_that_declares_nothing_is_hinted_by_its_marker_card() {
        let ltspice = "* C:\\sim\\amp.asc\nR1 a 0 1k\n.backanno\n.end\n";
        assert_eq!(
            hint(ltspice),
            Some(DialectHint {
                dialect: Dialect::Ltspice,
                marker: ".backanno",
                line: 3,
            })
        );
        let read = declare(ltspice, hint(ltspice).expect("hinted").dialect)
            .expect("the line is added")
            .applied_to(ltspice);
        assert_eq!(
            read,
            "* C:\\sim\\amp.asc\n.options rspice_source=ltspice\nR1 a 0 1k\n.backanno\n.end\n"
        );
        assert_eq!(hint(&read), None);

        let ngspice = "amp\nR1 a 0 1k\n.control\nrun\nplot v(a)\n.endc\n.end\n";
        assert_eq!(
            hint(ngspice).map(|hint| (hint.dialect, hint.line)),
            Some((Dialect::Ngspice, 3))
        );
        assert_eq!(hint("amp\n* .backanno\nR1 a 0 1k ; .control\n"), None);
        assert_eq!(
            hint(".backanno\nR1 a 0 1k\n"),
            None,
            "a title is not a card"
        );
    }
}
