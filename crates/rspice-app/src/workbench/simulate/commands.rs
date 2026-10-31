//! The commands the simulate feature declares: running and stopping the
//! active plan, the run history, the preflight checks, the plan manager,
//! the analysis catalogue, the global solver settings, the Simulate
//! workspace's setup pages and the Verify workspace's pages.
//!
//! What each command does is the feature's own; its key is all the shell's
//! menus, palette and shortcut profile read of it.

use crate::state::command_catalog::CommandKey;
use crate::state::{SimulationPage, VerificationPage};
use crate::workbench::framework::command_registry::ids;

/// One command of the simulate feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SimulateCommand {
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
    AddAnalysis,
    /// The global solver and convergence settings.
    Options,
    /// Open the Simulate workspace on one of its setup pages.
    Page(SimulationPage),
    /// Open the Verify workspace on one of its pages.
    Verify(VerificationPage),
}

impl SimulateCommand {
    /// The key a shortcut profile names this command by.
    pub(crate) const fn key(self) -> CommandKey {
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
            Self::Verify(VerificationPage::Tuning) => CommandKey::new("tuning"),
            Self::Verify(VerificationPage::Optimization) => CommandKey::new("optimization"),
            Self::Verify(VerificationPage::DeviceSafety) => CommandKey::new("device-safety"),
            Self::Verify(VerificationPage::Regression) => CommandKey::new("regression"),
        }
    }
}
