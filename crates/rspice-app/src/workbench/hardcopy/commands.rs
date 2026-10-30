//! The commands the hardcopy feature declares: the schematic and publication
//! exports, Print and Export active view, and the report editor, as the
//! registry lists them; and the report route's own toolbar actions, which the
//! registry leaves out.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use egui::Key;

use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};

/// One command of the hardcopy feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardcopyCommand {
    ExportSchematicSvg,
    ExportPublicationSnapshot,
    PrintHardcopy,
    ExportActiveView,
    ReportAuthoring,
    SaveReportDocument,
    AddReportPage,
    ReportPageProperties,
}

const NONE: &[ShortcutBinding] = &[];

const DESKTOP: &[CommandPlatform] = &[CommandPlatform::Desktop];
/// The browser keeps Ctrl+P and Ctrl+Shift+P for itself, so a browser,
/// tablet or phone profile takes the Alt chords instead.
const HOST: &[CommandPlatform] = &[
    CommandPlatform::Browser,
    CommandPlatform::Tablet,
    CommandPlatform::Phone,
];

const fn binding(
    kind: ShortcutKind,
    platforms: &'static [CommandPlatform],
    key: Key,
    alt: bool,
    shift: bool,
    label: &'static str,
) -> ShortcutBinding {
    ShortcutBinding {
        chord: ShortcutChord::new(key, true, alt, shift, label),
        platforms,
        kind,
    }
}

const PRINT_HARDCOPY: &[ShortcutBinding] = &[
    binding(
        ShortcutKind::Primary,
        DESKTOP,
        Key::P,
        false,
        false,
        "Ctrl+P",
    ),
    binding(
        ShortcutKind::Alternate,
        HOST,
        Key::P,
        true,
        false,
        "Ctrl+Alt+P",
    ),
];
const EXPORT_ACTIVE_VIEW: &[ShortcutBinding] = &[
    binding(
        ShortcutKind::Primary,
        DESKTOP,
        Key::P,
        false,
        true,
        "Ctrl+Shift+P",
    ),
    binding(
        ShortcutKind::Alternate,
        HOST,
        Key::E,
        true,
        true,
        "Ctrl+Alt+Shift+E",
    ),
];

/// What every listed hardcopy command declares: it is in the palette and
/// needs an open project.
const LISTED_COMMAND: CommandFlags =
    CommandFlags::PALETTE_VISIBLE.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);

/// A [`LISTED_COMMAND`] that exports what the project holds, so a value typed
/// into the inspector and not yet committed is committed before it, and the
/// export does not run when that commit is refused.
const PROJECT_EXPORT: CommandFlags =
    LISTED_COMMAND.with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);

/// A [`LISTED_COMMAND`] that resolves the active view, so it waits for a
/// project operation.
const ACTIVE_VIEW_OUTPUT: CommandFlags =
    LISTED_COMMAND.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);

/// A report toolbar action: it needs an open project and acts on the
/// document the route shows, so the palette has nothing to give it.
const REPORT_ACTION: CommandFlags =
    CommandFlags::NONE.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);

impl HardcopyCommand {
    /// The commands the registry lists, in the shell's order.
    pub const LISTED: [Self; 5] = [
        Self::ExportSchematicSvg,
        Self::ExportPublicationSnapshot,
        Self::PrintHardcopy,
        Self::ExportActiveView,
        Self::ReportAuthoring,
    ];

    /// The report route's toolbar actions. The toolbar holds each by key; the
    /// registry leaves them out, so no menu, shortcut profile, palette or
    /// Repeat reaches them.
    pub const REPORT_TOOLBAR: [Self; 3] = [
        Self::SaveReportDocument,
        Self::AddReportPage,
        Self::ReportPageProperties,
    ];

    /// The command `key` names, whether the registry lists it or not.
    pub fn from_key(key: CommandKey) -> Option<Self> {
        Self::LISTED
            .into_iter()
            .chain(Self::REPORT_TOOLBAR)
            .find(|command| command.key() == key)
    }

    /// The key a shortcut profile or the report toolbar names this command
    /// by.
    pub const fn key(self) -> CommandKey {
        match self {
            Self::ExportSchematicSvg => CommandKey::new("export-schematic-svg"),
            Self::ExportPublicationSnapshot => CommandKey::new("export-publication-snapshot"),
            Self::PrintHardcopy => ids::PRINT_HARDCOPY,
            Self::ExportActiveView => CommandKey::new("export-active-view"),
            Self::ReportAuthoring => CommandKey::new("report-page-editor"),
            Self::SaveReportDocument => CommandKey::new("report-save-document"),
            Self::AddReportPage => CommandKey::new("report-add-page"),
            Self::ReportPageProperties => CommandKey::new("report-page-properties"),
        }
    }

    /// What the command declares about itself.
    pub const fn declared_spec(self) -> CommandSpec {
        let (label, group, flags, context, bindings) = match self {
            Self::ExportSchematicSvg => (
                "Export schematic SVG\u{2026}",
                "File",
                PROJECT_EXPORT,
                ShortcutContext::Global,
                NONE,
            ),
            Self::ExportPublicationSnapshot => (
                "Export publication snapshot\u{2026}",
                "File",
                PROJECT_EXPORT,
                ShortcutContext::Global,
                NONE,
            ),
            Self::PrintHardcopy => (
                "Print / hardcopy\u{2026}",
                "File",
                ACTIVE_VIEW_OUTPUT,
                ShortcutContext::Global,
                PRINT_HARDCOPY,
            ),
            Self::ExportActiveView => (
                "Export active view\u{2026}",
                "File",
                ACTIVE_VIEW_OUTPUT,
                ShortcutContext::Global,
                EXPORT_ACTIVE_VIEW,
            ),
            Self::ReportAuthoring => (
                "Report page and datasheet editor\u{2026}",
                "Results",
                LISTED_COMMAND,
                ShortcutContext::Global,
                NONE,
            ),
            Self::SaveReportDocument => (
                "Save report document",
                "Results",
                REPORT_ACTION.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true),
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::AddReportPage => (
                "Add report page",
                "Results",
                REPORT_ACTION,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
            Self::ReportPageProperties => (
                "Page properties",
                "Results",
                REPORT_ACTION,
                ShortcutContext::ResultsWorkspace,
                NONE,
            ),
        };
        CommandSpec {
            key: self.key(),
            label,
            group,
            flags,
            context,
            bindings,
            owner: CommandOwner::Hardcopy,
        }
    }
}
