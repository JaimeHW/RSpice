//! The commands the simulate feature declares, as the registry lists them:
//! running and stopping the active plan, the run history, the preflight
//! checks, the plan manager, the analysis catalogue, the global solver
//! settings, the Simulate workspace's setup pages and the Verify workspace's
//! pages.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use egui::Key;

use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::state::{SimulationPage, VerificationPage};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};

/// One command of the simulate feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulateCommand {
    /// Run the active plan.
    Run,
    /// Stop the active run.
    Stop,
    /// The run history.
    JobsManager,
    /// Check the active plan's run before starting it.
    Preflight,
    /// The versioned simulation plan manager. The toolbar's plan chip, the
    /// palette and the shortcut map all reach the manager through this one
    /// command rather than constructing its draft themselves.
    ManagePlans,
    /// Open the analysis catalogue over whatever workspace the reader is on.
    ///
    /// The navigator's creating action, the empty analysis rail and the
    /// Simulate menu all arm the catalogue through this one command. They used
    /// to set the four palette fields themselves, which is how the navigator
    /// came to arm a modal on eight routes that had nothing drawing it.
    AddAnalysis,
    /// The global solver and convergence settings.
    Options,
    /// Open the Simulate workspace on one of its setup pages.
    Page(SimulationPage),
    /// Open the Verify workspace on one of its pages.
    Verify(VerificationPage),
}

const DESKTOP: &[CommandPlatform] = &[CommandPlatform::Desktop];
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
const RUN: &[ShortcutBinding] = &[
    primary(chord(Key::F5, false, false, false, "F5"), DESKTOP),
    alternate(chord(Key::Enter, true, false, false, "Ctrl+Enter"), HOST),
];
const STOP: &[ShortcutBinding] = &[
    primary(chord(Key::F5, false, false, true, "Shift+F5"), DESKTOP),
    alternate(
        chord(Key::Enter, true, false, true, "Ctrl+Shift+Enter"),
        HOST,
    ),
];

impl SimulateCommand {
    /// Every command the registry lists, in the feature's own order.
    pub const ALL: [Self; 22] = [
        Self::Run,
        Self::Stop,
        Self::JobsManager,
        Self::Preflight,
        Self::ManagePlans,
        Self::AddAnalysis,
        Self::Options,
        Self::Page(SimulationPage::Analyses),
        Self::Page(SimulationPage::Excitations),
        Self::Page(SimulationPage::Variables),
        Self::Page(SimulationPage::Outputs),
        Self::Page(SimulationPage::Specifications),
        Self::Page(SimulationPage::RunSet),
        Self::Page(SimulationPage::Models),
        Self::Page(SimulationPage::Solver),
        Self::Page(SimulationPage::Save),
        Self::Verify(VerificationPage::Yield),
        Self::Verify(VerificationPage::Corners),
        Self::Verify(VerificationPage::Tuning),
        Self::Verify(VerificationPage::Optimization),
        Self::Verify(VerificationPage::DeviceSafety),
        Self::Verify(VerificationPage::Regression),
    ];

    /// The commands that run but are not listed: none. Parameter tuning,
    /// which once opened only from the instance it tunes, is listed in
    /// Verify and the palette like the other pages.
    pub const UNLISTED: [Self; 0] = [];

    /// The key a shortcut profile names this command by.
    pub const fn key(self) -> CommandKey {
        match self {
            Self::Run => ids::RUN_SIMULATION,
            Self::Stop => ids::STOP_SIMULATION,
            Self::JobsManager => CommandKey::new("jobs-manager"),
            Self::Preflight => CommandKey::new("check"),
            Self::ManagePlans => CommandKey::new("manage-simulation-plans"),
            Self::AddAnalysis => CommandKey::new("add-analysis"),
            Self::Options => CommandKey::new("solver"),
            Self::Page(SimulationPage::Analyses) => ids::SIMULATION_ANALYSES,
            Self::Page(SimulationPage::Excitations) => CommandKey::new("simulation-excitations"),
            Self::Page(SimulationPage::Variables) => CommandKey::new("simulation-variables"),
            Self::Page(SimulationPage::Outputs) => CommandKey::new("simulation-outputs"),
            Self::Page(SimulationPage::Specifications) => {
                CommandKey::new("simulation-specifications")
            }
            Self::Page(SimulationPage::RunSet) => CommandKey::new("simulation-run-set"),
            Self::Page(SimulationPage::Models) => CommandKey::new("simulation-models"),
            Self::Page(SimulationPage::Solver) => CommandKey::new("simulation-solver"),
            Self::Page(SimulationPage::Save) => CommandKey::new("simulation-save-policy"),
            Self::Verify(VerificationPage::Yield) => CommandKey::new("yield"),
            Self::Verify(VerificationPage::Corners) => CommandKey::new("corners"),
            Self::Verify(VerificationPage::Tuning) => ids::TUNING,
            Self::Verify(VerificationPage::Optimization) => CommandKey::new("optimization"),
            Self::Verify(VerificationPage::DeviceSafety) => CommandKey::new("device-safety"),
            Self::Verify(VerificationPage::Regression) => CommandKey::new("regression"),
        }
    }

    /// The command `key` names, listed or not; `None` for a key the feature
    /// does not declare.
    pub fn from_key(key: CommandKey) -> Option<Self> {
        Self::ALL
            .into_iter()
            .chain(Self::UNLISTED)
            .find(|command| command.key() == key)
    }

    /// What the command declares about itself. Every one is in the palette.
    /// Running and stopping act on whatever is open and keep the run chords,
    /// whose desktop key a browser keeps for itself; the run history needs
    /// no project either. Everything else needs an open project. Running,
    /// checking and the pages commit a pending inspector edit before they
    /// act.
    pub const fn declared_spec(self) -> CommandSpec {
        let palette = CommandFlags::PALETTE_VISIBLE;
        let commits = palette.with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let open = palette.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
        let navigates = open.with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let (label, group, flags, context, bindings) = match self {
            Self::Run => (
                "Run active plan",
                "Simulate",
                commits.with(CommandFlags::PRIMARY_RESERVED_BY_BROWSER, true),
                ShortcutContext::RunnableProject,
                RUN,
            ),
            Self::Stop => (
                "Stop active run",
                "Simulate",
                palette.with(CommandFlags::PRIMARY_RESERVED_BY_BROWSER, true),
                ShortcutContext::RunnableProject,
                STOP,
            ),
            Self::JobsManager => (
                "Run history\u{2026}",
                "Simulate",
                open,
                ShortcutContext::Global,
                NONE,
            ),
            // No key of its own: Ctrl+E checks the cellview.
            Self::Preflight => (
                "Check and netlist",
                "Simulate",
                navigates,
                ShortcutContext::SimulationWorkspace,
                NONE,
            ),
            // Deliberately global, unlike the Simulate workspace's own
            // commands: the plan chip that owns this action lives in the
            // context toolbar, which is chrome shown in every workspace. A
            // Simulate-workspace context would withdraw the keyboard route
            // from every place the chip itself is still clickable.
            Self::ManagePlans => (
                "Manage simulation plans\u{2026}",
                "Simulate",
                open,
                ShortcutContext::Global,
                NONE,
            ),
            Self::AddAnalysis => (
                "Add analysis\u{2026}",
                "Simulate",
                open,
                ShortcutContext::Global,
                NONE,
            ),
            Self::Options => (
                "Global solver & convergence",
                "Simulate",
                open,
                ShortcutContext::Global,
                NONE,
            ),
            // Every setup route is addressable, the way the Project, Verify
            // and Models pages are. The Solver route keeps its own dedicated
            // command as well, because that one is also a menu action.
            Self::Page(page) => (
                simulation_page_label(page),
                "Simulate",
                navigates,
                ShortcutContext::Global,
                NONE,
            ),
            Self::Verify(page) => (
                verification_page_label(page),
                "Verify",
                navigates,
                ShortcutContext::VerificationWorkspace,
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
            owner: CommandOwner::Simulate,
        }
    }
}

/// What the command that opens the Simulate workspace on `page` is called.
const fn simulation_page_label(page: SimulationPage) -> &'static str {
    match page {
        SimulationPage::Analyses => "Analyses",
        SimulationPage::Excitations => "Excitations",
        SimulationPage::Variables => "Design variables",
        SimulationPage::Outputs => "Outputs & expressions",
        SimulationPage::Specifications => "Requirements & specifications",
        SimulationPage::RunSet => "PVT, sweeps & variation",
        SimulationPage::Models => "Models & sections",
        SimulationPage::Solver => "Simulator options",
        SimulationPage::Save => "Save, streaming & retention policy",
    }
}

/// What the command that opens the Verify workspace on `page` is called.
const fn verification_page_label(page: VerificationPage) -> &'static str {
    match page {
        VerificationPage::Yield => "Monte Carlo yield",
        VerificationPage::Corners => "Process corners",
        VerificationPage::Tuning => "Parameter tuning",
        VerificationPage::Optimization => "Optimization",
        VerificationPage::DeviceSafety => "Safe operating area",
        VerificationPage::Regression => "Regression",
    }
}
