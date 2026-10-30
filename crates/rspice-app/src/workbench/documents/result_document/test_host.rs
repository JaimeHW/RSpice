//! What the results feature's own tests run against: the shared model, the
//! session, and the results, platform and runs slots.
//!
//! The fields carry the names `AppState` gives the same values, and the
//! methods answer as `AppState`'s do, so a test body reads the same against
//! either and only its constructor differs. The command registry is the one
//! difference: a feature compiled on its own cannot name the application's,
//! so a new host reads none, and a test that presses a chord or reads a
//! shortcut label runs against the whole application in the shell's feature
//! tests.

use std::sync::LazyLock;

use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_app_services::workbench::slots::platform::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;
use rspice_app_state::workbench::workspace::restore_model_library_manager;

use crate::state::pdk_config::PdkConfig;
use crate::state::{RunEvent, RunEventSink};
use crate::workbench::framework::command_registry::Registry;
use crate::workbench::slots::ResultsState;
use crate::workbench::state::WorkbenchSession;

use super::{ResultsParts, ResultsReads};

/// The commands a new host reads: none is registered.
static NO_COMMANDS: LazyLock<Registry> = LazyLock::new(|| Registry::new(&[], Vec::new(), &[]));

/// The shared model, the session and the slots the results feature reads,
/// under the names `AppState` gives them.
#[derive(Clone)]
pub(crate) struct ResultsTestHost {
    pub(crate) model: Model,
    pub(crate) results: Box<ResultsState>,
    pub(crate) workbench: Box<WorkbenchSession>,
    pub(crate) platform: Box<PlatformState>,
    pub(crate) runs: Box<RunsState>,
    /// Every command the parts read as registered.
    pub(crate) commands: &'static Registry,
}

impl Default for ResultsTestHost {
    /// The model a new session starts with, over the empty PDK configuration
    /// a test session reads, and every slot as a new session builds it.
    fn default() -> Self {
        Self {
            model: Model::session_start(PdkConfig::default(), None, restore_model_library_manager),
            results: Box::default(),
            workbench: Box::default(),
            platform: Box::default(),
            runs: Box::default(),
            commands: &NO_COMMANDS,
        }
    }
}

impl ResultsTestHost {
    /// The prompt on the status bar, if one is showing.
    pub(crate) fn prompt_text(&self) -> Option<&str> {
        self.workbench
            .prompt
            .as_ref()
            .map(|prompt| prompt.text.as_str())
    }

    /// Whether a viewer is currently available for activation.
    pub(crate) fn viewer_is_available(&self, viewer: crate::state::ActiveViewer) -> bool {
        self.results_reads().viewer_capability(viewer).available
    }

    /// The parts the results workspace's predicates read.
    pub(crate) fn results_reads(&self) -> ResultsReads<'_> {
        self.results.reads(&self.model, &self.workbench)
    }

    /// The parts the results workspace's commands write.
    pub(crate) fn results_parts(&mut self) -> ResultsParts<'_> {
        self.results.parts(
            &mut self.model,
            &mut self.workbench,
            &self.platform,
            &mut self.runs.preflight,
            self.commands,
        )
    }
}

/// The results workspace's viewers take what the run tier reports.
impl RunEventSink for ResultsTestHost {
    fn apply_run_event(&mut self, event: RunEvent) {
        self.results_parts().apply_run_event(event);
    }
}
