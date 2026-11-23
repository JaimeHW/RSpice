//! The source editor's commands as its menus show them, group by group in
//! the order reference editors keep: what undoes, the clipboard and the
//! selection, finding and going to a place, the lines under the caret, the
//! document's language and the problems it finds. The Edit menu and the
//! editor's right-click menu are both made of these groups, and a test walks
//! the source feature's command table to find a row for every command the
//! editor answers.

use super::commands::SourceCommand;
use super::{MenuContext, MenuEntry, Separator, ids};

/// The command's row under the label it declares.
const fn code(command: SourceCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// `entries` in a menu of the source editor's: shown wherever the editor
/// is, which is wherever its menus are.
const fn group(entries: &'static [MenuEntry]) -> MenuEntry {
    MenuEntry::Only(MenuContext::SourceEditor, entries)
}

/// Undo and Redo, of the editor's history.
const HISTORY: &[MenuEntry] = &[
    MenuEntry::command(ids::UNDO).access('U'),
    MenuEntry::command(ids::REDO).access('R'),
];

/// The clipboard.
const CLIPBOARD: &[MenuEntry] = &[
    MenuEntry::command(ids::CUT).access('T'),
    MenuEntry::command(ids::COPY).access('C'),
    MenuEntry::command(ids::PASTE).access('P'),
];

/// Deleting what is selected, or the character after the caret: Edit's
/// alone, as the Delete key is beside every right-click.
const DELETE: MenuEntry = MenuEntry::command(ids::DELETE).access('D');

/// Selecting the whole document.
const SELECT_ALL: MenuEntry = MenuEntry::command(ids::SELECT_ALL).access('A');

/// Finding and replacing text, and going to a line or the bracket that
/// matches.
const FIND: &[MenuEntry] = &[
    code(SourceCommand::FindCodeDocument).access('F'),
    code(SourceCommand::ReplaceInCodeDocument).access('M'),
    code(SourceCommand::GoToLine).access('G'),
    code(SourceCommand::GoToMatchingBracket).access('B'),
];

/// What is done to the lines under the caret or the selection.
const LINE: &[MenuEntry] = &[
    code(SourceCommand::ToggleLineComment).access('M'),
    code(SourceCommand::IndentLines).access('I'),
    code(SourceCommand::OutdentLines).access('O'),
    code(SourceCommand::DuplicateLine).access('D'),
    code(SourceCommand::DeleteLine).access('L'),
    code(SourceCommand::MoveLineUp).access('U'),
    code(SourceCommand::MoveLineDown).access('W'),
];

/// The document's language: what a hover would say at the caret, going to
/// what a name refers to and renaming it, where the language service
/// answers them, and formatting.
const LANGUAGE: &[MenuEntry] = &[
    code(SourceCommand::ShowValueAtCaret).access('W'),
    MenuEntry::Only(
        MenuContext::NavigableSourceEditor,
        &[
            code(SourceCommand::GoToDefinition).access('E'),
            code(SourceCommand::FindReferences).access('I'),
            code(SourceCommand::RenameSymbol).access('Y'),
        ],
    ),
    code(SourceCommand::FormatDocument).access('O'),
];

/// The problems the language finds in the document.
const PROBLEMS: &[MenuEntry] = &[
    code(SourceCommand::NextProblem).access('X'),
    code(SourceCommand::PreviousProblem).access('V'),
    code(SourceCommand::ShowProblemAtCaret).access('H'),
];

/// From the line under the caret to what the schematic drew it from, and to
/// a waveform of the run.
const CROSS_PROBE: &[MenuEntry] = &[
    code(SourceCommand::ShowInSchematic).access('H'),
    code(SourceCommand::PlotNodeVoltage).access('V'),
    code(SourceCommand::PlotDeviceCurrent).access('L'),
];

/// Edit, while a source editor is the document on screen.
pub const SOURCE_EDIT_MENU: &[MenuEntry] = &[
    group(HISTORY),
    Separator,
    group(CLIPBOARD),
    DELETE,
    SELECT_ALL,
    Separator,
    group(FIND),
    Separator,
    MenuEntry::submenu("Line", LINE).access('L'),
    Separator,
    group(LANGUAGE),
    Separator,
    group(PROBLEMS),
];

/// View > Folding, while a source editor is the document on screen.
pub const SOURCE_FOLDING_MENU: &[MenuEntry] = &[
    code(SourceCommand::FoldSelection).access('F'),
    code(SourceCommand::UnfoldAtCaret).access('C'),
    code(SourceCommand::FoldAll).access('A'),
    code(SourceCommand::UnfoldAll).access('U'),
];

/// The View menu's rows while a source editor is the document on screen.
pub const SOURCE_VIEW_ROWS: &[MenuEntry] = &[
    MenuEntry::submenu("Folding", SOURCE_FOLDING_MENU).access('L'),
    code(SourceCommand::ToggleWordWrap).access('W'),
];

/// The source editor's right-click menu: the language group first, as the
/// name under the pointer is what a right-click most often asks about, then
/// the way to the schematic and the waveforms, the history, the clipboard,
/// and the line's comment. Every page that draws a source editor is the code
/// feature's: the deck and a Verilog-A view.
pub const SOURCE_EDITOR_MENU: &[MenuEntry] = &[
    group(LANGUAGE),
    Separator,
    group(CROSS_PROBE),
    Separator,
    group(HISTORY),
    Separator,
    group(CLIPBOARD),
    SELECT_ALL,
    Separator,
    code(SourceCommand::ToggleLineComment).access('M'),
];
