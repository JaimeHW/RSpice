//! The command facts a persisted shortcut profile is written against.
//!
//! A profile names a command by its stable id, [`CommandKey`]. Which commands
//! exist, the keys each ships with and the context its keys act in are the
//! command registry's to declare; the profile reads them through
//! [`CommandCatalog`]. The values those facts are stated in, platforms,
//! contexts and chords, live here, so the model names them without naming a
//! command.

use egui::Key;
use serde::{Deserialize, Serialize};

use crate::product::CommandId;

/// A command, named by its stable id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandKey(&'static str);

impl CommandKey {
    #[must_use]
    pub const fn new(id: &'static str) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// The id a profile stores the command's bindings under.
    #[must_use]
    pub fn id(self) -> CommandId {
        CommandId::new(self.0).expect("registry command IDs are validated product IDs")
    }
}

/// The command facts a shortcut profile reads.
///
/// A profile caches whether it is valid, so every call on one profile passes
/// the same catalog. The application has one: the command registry.
pub trait CommandCatalog {
    /// Every command, in registry order.
    fn commands(&self) -> &[CommandKey];

    /// The command a stored id names, a retired id included; `None` for an
    /// id this build has no command for.
    fn resolve(&self, id: &str) -> Option<CommandKey>;

    /// The context the command's keys act in; `None` for a key the catalog
    /// does not hold.
    fn context(&self, command: CommandKey) -> Option<ShortcutContext>;

    /// The bindings the command ships with.
    fn default_bindings(&self, command: CommandKey) -> &'static [ShortcutBinding];

    /// Whether `platform` keeps the command's primary chord for itself, so
    /// the command needs an alternate there.
    fn primary_is_reserved_on(&self, command: CommandKey, platform: CommandPlatform) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommandPlatform {
    Desktop,
    Browser,
    Tablet,
    Phone,
}

impl CommandPlatform {
    pub const ALL: [Self; 4] = [Self::Desktop, Self::Browser, Self::Tablet, Self::Phone];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::Browser => "Browser",
            Self::Tablet => "Tablet",
            Self::Phone => "Phone",
        }
    }

    pub const fn has_browser_reserved_primary(self) -> bool {
        matches!(self, Self::Browser)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutContext {
    Global,
    ApplicationChrome,
    EditContext,
    EngineeringCanvas,
    SymbolCanvas,
    DesignWorkspace,
    SimulationWorkspace,
    ResultsWorkspace,
    VerificationWorkspace,
    ViolationNavigation,
    RunnableProject,
    /// The source editor a page shows as its document. Its keys act only
    /// while that editor holds keyboard focus.
    SourceEditor,
}

impl ShortcutContext {
    pub const ALL: [Self; 12] = [
        Self::Global,
        Self::ApplicationChrome,
        Self::EditContext,
        Self::EngineeringCanvas,
        Self::SymbolCanvas,
        Self::DesignWorkspace,
        Self::SimulationWorkspace,
        Self::ResultsWorkspace,
        Self::VerificationWorkspace,
        Self::ViolationNavigation,
        Self::RunnableProject,
        Self::SourceEditor,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::ApplicationChrome => "application-chrome",
            Self::EditContext => "edit-context",
            Self::EngineeringCanvas => "engineering-canvas",
            Self::SymbolCanvas => "symbol-canvas",
            Self::DesignWorkspace => "design-workspace",
            Self::SimulationWorkspace => "simulation-workspace",
            Self::ResultsWorkspace => "results-workspace",
            Self::VerificationWorkspace => "verification-workspace",
            Self::ViolationNavigation => "violation-navigation",
            Self::RunnableProject => "runnable-project",
            Self::SourceEditor => "source-editor",
        }
    }

    /// The source editor is itself the text that has focus, so its context is
    /// the one text focus activates rather than suppresses.
    pub const fn suppressed_by_text_focus(self) -> bool {
        !matches!(
            self,
            Self::Global | Self::RunnableProject | Self::SourceEditor
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutKind {
    Primary,
    Alternate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortcutChord {
    pub key: Key,
    pub primary: bool,
    pub alt: bool,
    pub shift: bool,
    pub label: &'static str,
}

impl ShortcutChord {
    pub const fn new(key: Key, primary: bool, alt: bool, shift: bool, label: &'static str) -> Self {
        Self {
            key,
            primary,
            alt,
            shift,
            label,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortcutBinding {
    pub chord: ShortcutChord,
    pub platforms: &'static [CommandPlatform],
    pub kind: ShortcutKind,
}

impl ShortcutBinding {
    #[cfg(any(test, feature = "test-support"))]
    pub const fn supports(self, platform: CommandPlatform) -> bool {
        let mut index = 0;
        while index < self.platforms.len() {
            if self.platforms[index] as u8 == platform as u8 {
                return true;
            }
            index += 1;
        }
        false
    }
}
