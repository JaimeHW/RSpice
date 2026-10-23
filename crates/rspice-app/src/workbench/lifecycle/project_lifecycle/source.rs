//! What the project lifecycle works on.
//!
//! The lifecycle owns the platform state and reads and writes the shared
//! model. Two values it reads belong to the features above it: the workspace
//! in front, which names the active document, and the result presentation a
//! project file saves. They come in beside the model, read through these
//! traits by whoever holds the whole application; the lifecycle never reaches
//! into a feature for them.

use rspice_design::drc::DrcResult;

use super::{Model, PlatformState};
use crate::state::SimulationState;
use crate::state::result_presentation::ResultPresentation;
use crate::workbench::state::Workspace;

/// The result presentation a project file saves: the reader's markers, panes
/// and expression groups about the datasets the project still holds.
pub(crate) trait ResultPresentationSource {
    fn project_presentation(&self, simulation: &SimulationState) -> ResultPresentation;
}

/// What the lifecycle reads.
#[derive(Clone, Copy)]
pub(crate) struct LifecycleView<'a> {
    pub(crate) model: &'a Model,
    pub(crate) platform: &'a PlatformState,
    /// The workspace in front; it names the active document.
    pub(crate) workspace: Workspace,
    /// Asked only when a snapshot is captured, after the project's inputs.
    pub(crate) presentation: &'a dyn ResultPresentationSource,
}

/// What the lifecycle writes, with the values it reads beside them.
pub(crate) struct LifecycleParts<'a> {
    pub(crate) model: &'a mut Model,
    pub(crate) platform: &'a mut PlatformState,
    /// The workspace in front; it names the active document.
    pub(crate) workspace: Workspace,
    /// Asked only when a snapshot is captured, after the project's inputs.
    pub(crate) presentation: &'a dyn ResultPresentationSource,
}

/// Where the lifecycle reads the project from.
pub(crate) trait LifecycleSource {
    fn lifecycle_view(&self) -> LifecycleView<'_>;
}

/// Where the lifecycle changes the project.
pub(crate) trait LifecycleSourceMut: LifecycleSource {
    fn lifecycle_parts(&mut self) -> LifecycleParts<'_>;
}

/// The application a save completes in. A save the preference asks to be
/// checked runs the design checks there, after the save is adopted and before
/// its transaction ends.
pub(crate) trait LifecycleHost: LifecycleSourceMut {
    fn run_active_design_checks(&mut self) -> Result<DrcResult, String>;
}

impl LifecycleSource for LifecycleView<'_> {
    fn lifecycle_view(&self) -> LifecycleView<'_> {
        *self
    }
}

impl LifecycleSource for LifecycleParts<'_> {
    fn lifecycle_view(&self) -> LifecycleView<'_> {
        LifecycleView {
            model: &*self.model,
            platform: &*self.platform,
            workspace: self.workspace,
            presentation: self.presentation,
        }
    }
}

impl LifecycleSourceMut for LifecycleParts<'_> {
    fn lifecycle_parts(&mut self) -> LifecycleParts<'_> {
        LifecycleParts {
            model: &mut *self.model,
            platform: &mut *self.platform,
            workspace: self.workspace,
            presentation: self.presentation,
        }
    }
}
