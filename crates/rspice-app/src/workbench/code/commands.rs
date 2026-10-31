//! The commands the code feature declares: opening, importing, exporting
//! and generating a netlist, the Code workspace's find, validate and
//! document tools, the source editor's line, navigation and folding
//! commands, the generated deck's revision comparison, the Verilog-A
//! compiler and its build settings, and the Automation workspace.
//!
//! What each command does is the feature's own; its key is all the shell's
//! menus, palette and shortcut profile read of it. A command only another
//! feature names, or only a shortcut reaches, is declared with the rest of
//! its kind when the feature registers its commands.

use crate::io::NetlistFormat;
use crate::state::command_catalog::CommandKey;
use crate::workbench::framework::command_registry::ids;

/// One command of the code feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CodeCommand {
    OpenNetlist,
    ImportNetlist,
    ImportVerilogA,
    ExportNetlist(NetlistFormat),
    GenerateNetlist,
    FindCodeDocument,
    ValidateCodeDocument,
    ManageSourceDocument,
    /// The source editor's line, navigation and folding commands. Each acts
    /// on the source editor the page on screen shows as its document, and
    /// only there.
    ToggleLineComment,
    DuplicateLine,
    GoToLine,
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

impl CodeCommand {
    /// The key a shortcut profile names this command by.
    pub(crate) const fn key(self) -> CommandKey {
        match self {
            Self::OpenNetlist => ids::OPEN_NETLIST,
            Self::ImportNetlist => CommandKey::new("import-netlist"),
            Self::ImportVerilogA => CommandKey::new("import-veriloga"),
            Self::ExportNetlist(NetlistFormat::Spectre) => {
                CommandKey::new("export-netlist-spectre")
            }
            Self::ExportNetlist(NetlistFormat::Spice) => CommandKey::new("export-netlist-spice"),
            Self::GenerateNetlist => CommandKey::new("generated-netlist"),
            Self::FindCodeDocument => CommandKey::new("find-code"),
            Self::ValidateCodeDocument => CommandKey::new("validate-code"),
            Self::ManageSourceDocument => CommandKey::new("manage-source-document"),
            Self::ToggleLineComment => CommandKey::new("toggle-line-comment"),
            Self::DuplicateLine => CommandKey::new("duplicate-line"),
            Self::GoToLine => CommandKey::new("go-to-line"),
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
}
