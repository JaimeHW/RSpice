//! The commands the code feature declares, as the registry lists them:
//! opening, importing, exporting and generating a netlist, importing an
//! LTspice or QSPICE schematic, exporting the bill of materials, a new
//! Verilog-A device, where an instance sits in the generated deck, the Code
//! workspace's find, validate and document tools, the source editor's
//! line, navigation, finding and folding commands, the generated deck's
//! revision comparison, the Verilog-A compiler, its build settings and the
//! symbol of the module it compiles, and the Automation workspace.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use egui::Key;

use crate::io::NetlistFormat;
use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};
use crate::workbench::framework::requests::{
    Drain, EditorViewCommand, Request, SourceEditorRequest, StructuralCommand,
};

/// One command of the code feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CodeCommand {
    /// A cell whose Verilog-A module the reader writes next, with the symbol
    /// its ports generate.
    NewVerilogADevice,
    OpenNetlist,
    ImportNetlist,
    ImportVerilogA,
    ImportLtspiceSchematic,
    ImportQspiceSchematic,
    ExportNetlist(NetlistFormat),
    ExportBillOfMaterials,
    GenerateNetlist,
    /// Sole owner of "locate this instance in the generated deck". The
    /// canvas context menu, the instance inspector and the design navigator
    /// all route here rather than each moving the workspace themselves.
    ShowInNetlist,
    FindCodeDocument,
    ValidateCodeDocument,
    ManageSourceDocument,
    /// The source editor's line, navigation and folding commands. Each acts
    /// on the source editor the page on screen shows as its document, and
    /// only there.
    ToggleLineComment,
    IndentLines,
    OutdentLines,
    DuplicateLine,
    DeleteLine,
    MoveLineUp,
    MoveLineDown,
    GoToMatchingBracket,
    GoToLine,
    /// The next or previous finding in the focused source document.
    NextProblem,
    PreviousProblem,
    /// Say what the finding at the caret of the focused source document is.
    ShowProblemAtCaret,
    FormatDocument,
    FoldSelection,
    UnfoldAtCaret,
    FoldAll,
    UnfoldAll,
    /// Language navigation from the caret of the active source editor, for
    /// languages whose language service answers it.
    GoToDefinition,
    FindReferences,
    RenameSymbol,
    CompareGeneratedRevisions,
    CompileVerilogA,
    CancelVerilogACompile,
    VerilogABuildSettings,
    VerilogAOpenSymbol,
    AutomationConsole,
}

/// What the code feature hands the shell, which carries it out over the
/// whole application as the shell's table once did in place: opening and
/// importing a netlist or a Verilog-A file through the file workflow,
/// importing an LTspice or QSPICE schematic, exporting the deck from outside
/// the netlist page, exporting the bill of materials, the generated deck, the
/// selected instance's line in it, the Code workspace's
/// find, validation and document tools, language navigation from
/// the caret, the Verilog-A compile, its build settings and the symbol of
/// the module it compiles; and, from the feature's dialogs, creating the
/// device New Verilog-A device drafted, importing the cells Import
/// Verilog-A planned, and replacing the source Replace source from file
/// confirmed. Each command or dialog step that raises one raises one. Taken
/// straight after the call that raised it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CodeRequest {
    OpenNetlist,
    ImportNetlist,
    ImportVerilogA,
    ImportLtspiceSchematic,
    ImportQspiceSchematic,
    ExportNetlist(NetlistFormat),
    ExportBillOfMaterials,
    GenerateNetlist,
    ShowInNetlist,
    FindCodeDocument,
    ValidateCodeDocument,
    ManageSourceDocument,
    GoToDefinition,
    FindReferences,
    RenameSymbol,
    CompileVerilogA,
    VerilogABuildSettings,
    VerilogAOpenSymbol,
    CreateVerilogADevice,
    ImportVerilogACells,
    ReplaceVerilogASource,
}

impl Request for CodeRequest {
    const DRAIN: Drain = Drain::AfterContribution;
}

const ALL: &[CommandPlatform] = &CommandPlatform::ALL;
const DESKTOP: &[CommandPlatform] = &[CommandPlatform::Desktop];
const DESKTOP_BROWSER: &[CommandPlatform] = &[CommandPlatform::Desktop, CommandPlatform::Browser];
const HOST: &[CommandPlatform] = &[
    CommandPlatform::Browser,
    CommandPlatform::Tablet,
    CommandPlatform::Phone,
];

const fn primary(chord: ShortcutChord, platforms: &'static [CommandPlatform]) -> ShortcutBinding {
    ShortcutBinding {
        chord,
        platforms,
        kind: ShortcutKind::Primary,
    }
}

const fn alternate(chord: ShortcutChord, platforms: &'static [CommandPlatform]) -> ShortcutBinding {
    ShortcutBinding {
        chord,
        platforms,
        kind: ShortcutKind::Alternate,
    }
}

const fn chord(
    key: Key,
    primary_modifier: bool,
    alt: bool,
    shift: bool,
    label: &'static str,
) -> ShortcutChord {
    ShortcutChord::new(key, primary_modifier, alt, shift, label)
}

const NONE: &[ShortcutBinding] = &[];
const OPEN_NETLIST: &[ShortcutBinding] = &[
    primary(chord(Key::L, true, false, true, "Ctrl+Shift+L"), DESKTOP),
    alternate(chord(Key::L, true, true, true, "Ctrl+Alt+Shift+L"), HOST),
];
const GENERATED_NETLIST: &[ShortcutBinding] = &[
    primary(chord(Key::L, true, false, false, "Ctrl+L"), DESKTOP),
    alternate(chord(Key::L, true, true, false, "Ctrl+Alt+L"), HOST),
];
const AUTOMATION_CONSOLE: &[ShortcutBinding] = &[primary(
    chord(Key::Backtick, true, false, false, "Ctrl+`"),
    ALL,
)];
// The source editor's own keys. Ctrl+D is Duplicate on a schematic too; the
// two contexts never hold focus at once, so the focused surface decides.
const TOGGLE_LINE_COMMENT: &[ShortcutBinding] = &[primary(
    chord(Key::Slash, true, false, false, "Ctrl+/"),
    ALL,
)];
const DUPLICATE_LINE: &[ShortcutBinding] =
    &[primary(chord(Key::D, true, false, false, "Ctrl+D"), ALL)];
const DELETE_LINE: &[ShortcutBinding] = &[primary(
    chord(Key::K, true, false, true, "Ctrl+Shift+K"),
    ALL,
)];
const MOVE_LINE_UP: &[ShortcutBinding] = &[primary(
    chord(Key::ArrowUp, false, true, false, "Alt+Up"),
    ALL,
)];
const MOVE_LINE_DOWN: &[ShortcutBinding] = &[primary(
    chord(Key::ArrowDown, false, true, false, "Alt+Down"),
    ALL,
)];
const GO_TO_MATCHING_BRACKET: &[ShortcutBinding] = &[primary(
    chord(Key::Backslash, true, false, true, "Ctrl+Shift+\\"),
    ALL,
)];
const GO_TO_LINE: &[ShortcutBinding] = &[primary(chord(Key::G, true, false, false, "Ctrl+G"), ALL)];
const FOLD_SELECTION: &[ShortcutBinding] = &[primary(
    chord(Key::OpenBracket, true, false, true, "Ctrl+Shift+["),
    ALL,
)];
const UNFOLD_AT_CARET: &[ShortcutBinding] = &[primary(
    chord(Key::CloseBracket, true, false, true, "Ctrl+Shift+]"),
    ALL,
)];
// F12 opens a browser's developer tools, so definition and references are
// desktop keys. F2 is Rename selection on a schematic and Rename symbol in
// a focused source editor; as with Ctrl+D the focused surface decides.
const GO_TO_DEFINITION: &[ShortcutBinding] = &[primary(
    chord(Key::F12, false, false, false, "F12"),
    DESKTOP,
)];
const FIND_REFERENCES: &[ShortcutBinding] = &[primary(
    chord(Key::F12, false, false, true, "Shift+F12"),
    DESKTOP,
)];
const RENAME_SYMBOL: &[ShortcutBinding] = &[primary(
    chord(Key::F2, false, false, false, "F2"),
    DESKTOP_BROWSER,
)];
// F8 also steps through a drawing's violations; with a source editor focused
// the editor's context outranks the workspace's, so the key follows focus.
const NEXT_PROBLEM: &[ShortcutBinding] = &[primary(chord(Key::F8, false, false, false, "F8"), ALL)];
const PREVIOUS_PROBLEM: &[ShortcutBinding] =
    &[primary(chord(Key::F8, false, false, true, "Shift+F8"), ALL)];
// The build keys of Visual Studio and VS Code. A browser keeps Ctrl+Shift+B
// for its bookmarks bar, so there F7 is the only one.
const COMPILE_VERILOGA: &[ShortcutBinding] = &[
    primary(chord(Key::F7, false, false, false, "F7"), ALL),
    alternate(chord(Key::B, true, false, true, "Ctrl+Shift+B"), DESKTOP),
];
// Ctrl+F in a focused source editor finds in that source; on a drawing it
// finds in the design. As with Ctrl+D the focused surface decides.
const FIND_CODE_DOCUMENT: &[ShortcutBinding] =
    &[primary(chord(Key::F, true, false, false, "Ctrl+F"), ALL)];

impl CodeCommand {
    /// Every command the registry lists, in the feature's own order.
    pub(crate) const ALL: [Self; 40] = [
        Self::NewVerilogADevice,
        Self::OpenNetlist,
        Self::ImportNetlist,
        Self::ImportVerilogA,
        Self::ImportLtspiceSchematic,
        Self::ImportQspiceSchematic,
        Self::ExportNetlist(NetlistFormat::Spectre),
        Self::ExportNetlist(NetlistFormat::Spice),
        Self::ExportBillOfMaterials,
        Self::GenerateNetlist,
        Self::ShowInNetlist,
        Self::FindCodeDocument,
        Self::ValidateCodeDocument,
        Self::ManageSourceDocument,
        Self::ToggleLineComment,
        Self::IndentLines,
        Self::OutdentLines,
        Self::DuplicateLine,
        Self::DeleteLine,
        Self::MoveLineUp,
        Self::MoveLineDown,
        Self::GoToMatchingBracket,
        Self::GoToLine,
        Self::NextProblem,
        Self::PreviousProblem,
        Self::ShowProblemAtCaret,
        Self::FormatDocument,
        Self::FoldSelection,
        Self::UnfoldAtCaret,
        Self::FoldAll,
        Self::UnfoldAll,
        Self::GoToDefinition,
        Self::FindReferences,
        Self::RenameSymbol,
        Self::CompareGeneratedRevisions,
        Self::CompileVerilogA,
        Self::CancelVerilogACompile,
        Self::VerilogABuildSettings,
        Self::VerilogAOpenSymbol,
        Self::AutomationConsole,
    ];

    /// The key a shortcut profile names this command by.
    pub(crate) const fn key(self) -> CommandKey {
        match self {
            Self::NewVerilogADevice => ids::NEW_VERILOGA_DEVICE,
            Self::OpenNetlist => ids::OPEN_NETLIST,
            Self::ImportNetlist => CommandKey::new("import-netlist"),
            Self::ImportVerilogA => CommandKey::new("import-veriloga"),
            Self::ImportLtspiceSchematic => CommandKey::new("import-ltspice-schematic"),
            Self::ImportQspiceSchematic => CommandKey::new("import-qspice-schematic"),
            Self::ExportNetlist(NetlistFormat::Spectre) => {
                CommandKey::new("export-netlist-spectre")
            }
            Self::ExportNetlist(NetlistFormat::Spice) => CommandKey::new("export-netlist-spice"),
            Self::ExportBillOfMaterials => CommandKey::new("export-bill-of-materials"),
            Self::GenerateNetlist => CommandKey::new("generated-netlist"),
            Self::ShowInNetlist => ids::SHOW_IN_NETLIST,
            Self::FindCodeDocument => CommandKey::new("find-code"),
            Self::ValidateCodeDocument => CommandKey::new("validate-code"),
            Self::ManageSourceDocument => CommandKey::new("manage-source-document"),
            Self::ToggleLineComment => CommandKey::new("toggle-line-comment"),
            Self::IndentLines => CommandKey::new("indent-lines"),
            Self::OutdentLines => CommandKey::new("outdent-lines"),
            Self::DuplicateLine => CommandKey::new("duplicate-line"),
            Self::DeleteLine => CommandKey::new("delete-line"),
            Self::MoveLineUp => CommandKey::new("move-line-up"),
            Self::MoveLineDown => CommandKey::new("move-line-down"),
            Self::GoToMatchingBracket => CommandKey::new("go-to-matching-bracket"),
            Self::GoToLine => CommandKey::new("go-to-line"),
            Self::NextProblem => CommandKey::new("next-problem"),
            Self::PreviousProblem => CommandKey::new("previous-problem"),
            Self::ShowProblemAtCaret => CommandKey::new("show-problem-at-caret"),
            Self::FormatDocument => CommandKey::new("format-document"),
            Self::FoldSelection => CommandKey::new("fold-selection"),
            Self::UnfoldAtCaret => CommandKey::new("unfold-at-caret"),
            Self::FoldAll => CommandKey::new("fold-all"),
            Self::UnfoldAll => CommandKey::new("unfold-all"),
            Self::GoToDefinition => CommandKey::new("go-to-definition"),
            Self::FindReferences => CommandKey::new("find-references"),
            Self::RenameSymbol => CommandKey::new("rename-symbol"),
            Self::CompareGeneratedRevisions => CommandKey::new("compare-generated-revisions"),
            Self::CompileVerilogA => ids::COMPILE_VERILOG_A,
            Self::CancelVerilogACompile => CommandKey::new("veriloga-cancel"),
            Self::VerilogABuildSettings => CommandKey::new("veriloga-build-settings"),
            Self::VerilogAOpenSymbol => CommandKey::new("veriloga-open-symbol"),
            Self::AutomationConsole => CommandKey::new("automation"),
        }
    }

    /// The command `key` names; `None` for a key the feature does not
    /// declare.
    pub(crate) fn from_key(key: CommandKey) -> Option<Self> {
        Self::ALL.into_iter().find(|command| command.key() == key)
    }

    /// What this command asks of a source editor when one is the document on
    /// screen: the same thing its shortcut does with the editor focused.
    pub(crate) const fn source_editor_request(self) -> Option<SourceEditorRequest> {
        Some(match self {
            Self::ToggleLineComment => {
                SourceEditorRequest::Structural(StructuralCommand::ToggleComment)
            }
            Self::IndentLines => SourceEditorRequest::Structural(StructuralCommand::Indent),
            Self::OutdentLines => SourceEditorRequest::Structural(StructuralCommand::Outdent),
            Self::DuplicateLine => {
                SourceEditorRequest::Structural(StructuralCommand::DuplicateLine)
            }
            Self::DeleteLine => SourceEditorRequest::Structural(StructuralCommand::DeleteLine),
            Self::MoveLineUp => SourceEditorRequest::Structural(StructuralCommand::MoveLineUp),
            Self::MoveLineDown => SourceEditorRequest::Structural(StructuralCommand::MoveLineDown),
            Self::GoToMatchingBracket => {
                SourceEditorRequest::Structural(StructuralCommand::MatchBracket)
            }
            Self::GoToLine => SourceEditorRequest::GoToLine,
            Self::FormatDocument => SourceEditorRequest::FormatDocument,
            Self::FoldSelection => SourceEditorRequest::View(EditorViewCommand::FoldSelection),
            Self::UnfoldAtCaret => SourceEditorRequest::View(EditorViewCommand::UnfoldAtCaret),
            Self::FoldAll => SourceEditorRequest::View(EditorViewCommand::FoldAll),
            Self::UnfoldAll => SourceEditorRequest::View(EditorViewCommand::UnfoldAll),
            _ => return None,
        })
    }

    /// Whether this command exists only for the source editor in front of the
    /// reader. Elsewhere it is not offered, rather than offered disabled.
    ///
    /// Find is the exception: its key acts in the focused editor, but the
    /// command searches whichever code page is on screen, from its toolbar
    /// and the palette as well.
    pub(crate) const fn source_editor_only(self) -> bool {
        matches!(self.context(), ShortcutContext::SourceEditor)
            && !matches!(self, Self::FindCodeDocument)
    }

    /// The language-service group: offered only where the document's
    /// language service can answer it.
    pub(crate) const fn navigates_source_language(self) -> bool {
        matches!(
            self,
            Self::GoToDefinition | Self::FindReferences | Self::RenameSymbol
        )
    }

    /// Where the command's shortcut is live: the source editor's own
    /// commands in a focused source editor, the jump to the deck on the
    /// design canvas, and everything else anywhere.
    const fn context(self) -> ShortcutContext {
        match self {
            Self::ToggleLineComment
            | Self::IndentLines
            | Self::OutdentLines
            | Self::DuplicateLine
            | Self::DeleteLine
            | Self::MoveLineUp
            | Self::MoveLineDown
            | Self::GoToMatchingBracket
            | Self::GoToLine
            | Self::NextProblem
            | Self::PreviousProblem
            | Self::ShowProblemAtCaret
            | Self::FormatDocument
            | Self::FoldSelection
            | Self::UnfoldAtCaret
            | Self::FoldAll
            | Self::UnfoldAll
            | Self::GoToDefinition
            | Self::FindReferences
            | Self::RenameSymbol
            | Self::FindCodeDocument => ShortcutContext::SourceEditor,
            Self::ShowInNetlist => ShortcutContext::DesignWorkspace,
            _ => ShortcutContext::Global,
        }
    }

    const fn bindings(self) -> &'static [ShortcutBinding] {
        match self {
            Self::OpenNetlist => OPEN_NETLIST,
            Self::GenerateNetlist => GENERATED_NETLIST,
            Self::AutomationConsole => AUTOMATION_CONSOLE,
            Self::ToggleLineComment => TOGGLE_LINE_COMMENT,
            // Tab and Shift+Tab stay the focused editor's own keys.
            Self::IndentLines | Self::OutdentLines => NONE,
            Self::DuplicateLine => DUPLICATE_LINE,
            Self::DeleteLine => DELETE_LINE,
            Self::MoveLineUp => MOVE_LINE_UP,
            Self::MoveLineDown => MOVE_LINE_DOWN,
            Self::GoToMatchingBracket => GO_TO_MATCHING_BRACKET,
            Self::GoToLine => GO_TO_LINE,
            Self::NextProblem => NEXT_PROBLEM,
            Self::PreviousProblem => PREVIOUS_PROBLEM,
            Self::FoldSelection => FOLD_SELECTION,
            Self::UnfoldAtCaret => UNFOLD_AT_CARET,
            Self::GoToDefinition => GO_TO_DEFINITION,
            Self::FindReferences => FIND_REFERENCES,
            Self::RenameSymbol => RENAME_SYMBOL,
            Self::FindCodeDocument => FIND_CODE_DOCUMENT,
            Self::CompileVerilogA => COMPILE_VERILOGA,
            _ => NONE,
        }
    }

    /// What the command declares about itself. Every one is in the palette.
    /// The commands that act on a project need one open, and the ones that
    /// open a file or a new device wait for a project operation in flight.
    /// The generated deck and the jump to it commit a pending inspector edit
    /// before they act, and the generated deck keeps the chord whose desktop
    /// key a browser keeps for itself.
    pub(crate) const fn declared_spec(self) -> CommandSpec {
        let palette = CommandFlags::PALETTE_VISIBLE;
        let open = palette.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
        let opens_file = palette.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);
        let imports = open.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);
        let navigates = open.with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let (label, group, flags) = match self {
            Self::NewVerilogADevice => ("New Verilog-A device\u{2026}", "File", imports),
            Self::OpenNetlist => ("Open netlist as project\u{2026}", "File", opens_file),
            Self::ImportNetlist => ("Import SPICE deck\u{2026}", "File", imports),
            Self::ImportVerilogA => ("Import Verilog-A/AMS\u{2026}", "File", imports),
            Self::ImportLtspiceSchematic => ("Import LTspice schematic\u{2026}", "File", imports),
            Self::ImportQspiceSchematic => ("Import QSPICE schematic\u{2026}", "File", imports),
            Self::ExportNetlist(NetlistFormat::Spectre) => {
                ("Export Spectre netlist\u{2026}", "File", open)
            }
            Self::ExportNetlist(NetlistFormat::Spice) => {
                ("Export SPICE netlist\u{2026}", "File", open)
            }
            Self::ExportBillOfMaterials => ("Export bill of materials\u{2026}", "File", open),
            Self::GenerateNetlist => (
                "Open generated netlist",
                "Simulate",
                navigates.with(CommandFlags::PRIMARY_RESERVED_BY_BROWSER, true),
            ),
            Self::ShowInNetlist => ("Show in netlist", "Design", navigates),
            Self::FindCodeDocument => ("Find in source\u{2026}", "Code", palette),
            Self::ValidateCodeDocument => ("Validate active Code document", "Code", palette),
            Self::ManageSourceDocument => ("Manage source document\u{2026}", "Code", palette),
            Self::ToggleLineComment => ("Toggle line comment", "Edit", palette),
            Self::IndentLines => ("Indent lines", "Edit", palette),
            Self::OutdentLines => ("Outdent lines", "Edit", palette),
            Self::DuplicateLine => ("Duplicate line", "Edit", palette),
            Self::DeleteLine => ("Delete line", "Edit", palette),
            Self::MoveLineUp => ("Move line up", "Edit", palette),
            Self::MoveLineDown => ("Move line down", "Edit", palette),
            Self::GoToMatchingBracket => ("Go to matching bracket", "Edit", palette),
            Self::GoToLine => ("Go to line\u{2026}", "Edit", palette),
            Self::NextProblem => ("Next problem", "Edit", palette),
            Self::PreviousProblem => ("Previous problem", "Edit", palette),
            Self::ShowProblemAtCaret => ("Show problem at caret", "Edit", palette),
            Self::FormatDocument => ("Format document", "Edit", palette),
            Self::FoldSelection => ("Fold selection", "View", palette),
            Self::UnfoldAtCaret => ("Unfold at caret", "View", palette),
            Self::FoldAll => ("Fold all", "View", palette),
            Self::UnfoldAll => ("Unfold all", "View", palette),
            Self::GoToDefinition => ("Go to definition", "Code", palette),
            Self::FindReferences => ("Find references", "Code", palette),
            Self::RenameSymbol => ("Rename symbol\u{2026}", "Code", palette),
            Self::CompareGeneratedRevisions => ("Compare generated revisions", "Code", palette),
            Self::CompileVerilogA => ("Compile Verilog-A/AMS", "Design", open),
            Self::CancelVerilogACompile => ("Cancel Verilog-A/AMS compile", "Design", open),
            Self::VerilogABuildSettings => ("Build settings\u{2026}", "Design", open),
            Self::VerilogAOpenSymbol => ("Open symbol", "Design", open),
            Self::AutomationConsole => ("Scripts", "Automation", open),
        };
        CommandSpec {
            key: self.key(),
            label,
            group,
            flags,
            context: self.context(),
            bindings: self.bindings(),
            owner: CommandOwner::Code,
        }
    }
}
