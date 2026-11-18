//! The source editor's commands as its menus show them, group by group in
//! the order reference editors keep: what undoes, the clipboard and the
//! selection, finding and going to a place, the lines under the caret, the
//! document's language and the problems it finds. The Edit menu and the
//! editor's right-click menu are both made of these groups, and a test walks
//! the code feature's command table to find a row for every command the
//! editor answers.

use super::commands::CodeCommand;
use super::{MenuContext, MenuEntry, Separator, ids};

/// The command's row under the label it declares.
const fn code(command: CodeCommand) -> MenuEntry {
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
    code(CodeCommand::FindCodeDocument).access('F'),
    code(CodeCommand::ReplaceInCodeDocument).access('S'),
    code(CodeCommand::GoToLine).access('G'),
    code(CodeCommand::GoToMatchingBracket).access('B'),
];

/// What is done to the lines under the caret or the selection.
const LINE: &[MenuEntry] = &[
    code(CodeCommand::ToggleLineComment).access('M'),
    code(CodeCommand::IndentLines).access('I'),
    code(CodeCommand::OutdentLines).access('O'),
    code(CodeCommand::DuplicateLine).access('D'),
    code(CodeCommand::DeleteLine).access('L'),
    code(CodeCommand::MoveLineUp).access('U'),
    code(CodeCommand::MoveLineDown).access('W'),
];

/// The document's language: going to what a name refers to and renaming
/// it, where the language service answers them, and formatting.
const LANGUAGE: &[MenuEntry] = &[
    MenuEntry::Only(
        MenuContext::NavigableSourceEditor,
        &[
            code(CodeCommand::GoToDefinition).access('E'),
            code(CodeCommand::FindReferences).access('I'),
            code(CodeCommand::RenameSymbol).access('Y'),
        ],
    ),
    code(CodeCommand::FormatDocument).access('O'),
];

/// The problems the language finds in the document.
const PROBLEMS: &[MenuEntry] = &[
    code(CodeCommand::NextProblem).access('X'),
    code(CodeCommand::PreviousProblem).access('V'),
    code(CodeCommand::ShowProblemAtCaret).access('H'),
];

/// From the line under the caret to what the schematic drew it from, and to
/// a waveform of the run.
const CROSS_PROBE: &[MenuEntry] = &[
    code(CodeCommand::ShowInSchematic).access('H'),
    code(CodeCommand::PlotNodeVoltage).access('V'),
    code(CodeCommand::PlotDeviceCurrent).access('L'),
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
    code(CodeCommand::FoldSelection).access('F'),
    code(CodeCommand::UnfoldAtCaret).access('C'),
    code(CodeCommand::FoldAll).access('A'),
    code(CodeCommand::UnfoldAll).access('U'),
];

/// The View menu's rows while a source editor is the document on screen.
pub const SOURCE_VIEW_ROWS: &[MenuEntry] = &[
    MenuEntry::submenu("Folding", SOURCE_FOLDING_MENU).access('L'),
    code(CodeCommand::ToggleWordWrap).access('W'),
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
    code(CodeCommand::ToggleLineComment).access('M'),
];
