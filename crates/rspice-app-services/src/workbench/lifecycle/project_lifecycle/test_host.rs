//! What the platform services' own tests run against: the shared model and the
//! platform slot, with the workspace in front beside them.
//!
//! The fields carry the names `AppState` gives the same values, so a test body
//! reads the same against either and only its constructor differs. Nothing
//! here reaches a feature: a test that needs one, or the application's design
//! checks, runs against the whole application in the shell's feature tests.

use rspice_app_state::workbench::workspace::restore_model_library_manager;
use rspice_design::drc::DrcResult;

use super::{
    LifecycleHost, LifecycleParts, LifecycleSource, LifecycleSourceMut, LifecycleView, Model,
    PlatformState, ResultPresentationSource,
};
use crate::state::SimulationState;
use crate::state::pdk_config::PdkConfig;
use crate::state::result_presentation::ResultPresentation;
use rspice_app_state::workbench::workspace::Workspace;

#[derive(Clone)]
pub(crate) struct ServicesHost {
    pub(crate) model: Model,
    pub(crate) platform: Box<PlatformState>,
    pub(crate) workbench: HostWorkbench,
}

/// The one part of the workbench the lifecycle reads: the workspace in front,
/// which names the active document.
#[derive(Clone, Copy)]
pub(crate) struct HostWorkbench {
    pub(crate) workspace: Workspace,
}

impl HostWorkbench {
    pub(crate) fn activate(&mut self, workspace: Workspace) {
        self.workspace = workspace;
    }
}

impl Default for ServicesHost {
    /// The model, platform slot and workspace a new session starts with, over
    /// the empty PDK configuration a test session reads.
    fn default() -> Self {
        Self {
            model: Model::session_start(PdkConfig::default(), None, restore_model_library_manager),
            platform: Box::default(),
            workbench: HostWorkbench {
                workspace: Workspace::Design,
            },
        }
    }
}

impl ServicesHost {
    /// The model's synchronization of the active schematic into the
    /// workspace. What the application would say about it on the status bar
    /// has no status bar here.
    pub(crate) fn sync_active_schematic_to_workspace(&mut self) {
        let _ = self.model.sync_active_schematic_to_workspace();
    }

    /// Record an event in the model's Console log, as the session does.
    pub(crate) fn record(
        &mut self,
        severity: crate::diagnostics::LogSeverity,
        source: crate::diagnostics::LogSource,
        text: impl Into<String>,
    ) -> u64 {
        self.model.record(severity, source, text)
    }
}

/// What a session that has presented no results saves: no markers, panes or
/// expression groups, and a marker high-water mark of zero.
struct NoPresentation;

impl ResultPresentationSource for NoPresentation {
    fn project_presentation(&self, _simulation: &SimulationState) -> ResultPresentation {
        ResultPresentation {
            marker_id_high_water: Some(0),
            ..ResultPresentation::default()
        }
    }
}

impl LifecycleSource for ServicesHost {
    fn lifecycle_view(&self) -> LifecycleView<'_> {
        LifecycleView {
            model: &self.model,
            platform: &self.platform,
            workspace: self.workbench.workspace,
            presentation: &NoPresentation,
        }
    }
}

impl LifecycleSourceMut for ServicesHost {
    fn lifecycle_parts(&mut self) -> LifecycleParts<'_> {
        LifecycleParts {
            model: &mut self.model,
            platform: &mut self.platform,
            workspace: self.workbench.workspace,
            presentation: &NoPresentation,
        }
    }
}

impl LifecycleHost for ServicesHost {
    fn run_active_design_checks(&mut self) -> Result<DrcResult, String> {
        panic!("the design checks are the application's; a test that runs them uses AppState")
    }
}
