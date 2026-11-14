//! What the design feature's own tests run against: the design feature's
//! state, the shared model and the workbench session.
//!
//! Each field carries the name `AppState` gives the same value, so a test
//! body reads the same against either and only its constructor differs. A
//! test that reads another slot, the session's other parts or the command
//! registry, or draws a frame, runs against the whole application instead.

use rspice_app_state::workbench::workspace::restore_session_model_library_manager;

use crate::properties::PropertyEditorSchema;
use crate::state::pdk_config::PdkConfig;
use crate::workbench::app_state::Model;
use crate::workbench::slots::DesignState;
use crate::workbench::state::WorkbenchSession;

/// The design feature's state, the shared model and the workbench session,
/// under the names `AppState` gives them.
#[derive(Clone)]
pub(crate) struct DesignTestHost {
    pub(crate) design: Box<DesignState>,
    pub(crate) model: Model,
    pub(crate) workbench: Box<WorkbenchSession>,
}

impl Default for DesignTestHost {
    /// What a new session starts with, as `AppState::default()` builds these
    /// three: the model over the empty PDK configuration a test session reads,
    /// the design state with the property schema and the model's snap grid,
    /// and a new workbench session.
    fn default() -> Self {
        let model = Model::session_start(
            PdkConfig::default(),
            None,
            restore_session_model_library_manager,
        );
        let mut design = Box::<DesignState>::default();
        design.property_registry = PropertyEditorSchema::new();
        design.schematic_snap.grid_size = model
            .schematic
            .document()
            .document_policy
            .grid_pitch
            .canvas_grid_size();
        Self {
            design,
            model,
            workbench: Box::default(),
        }
    }
}

/// The shared model, as `AppState` lends it.
impl AsRef<Model> for DesignTestHost {
    fn as_ref(&self) -> &Model {
        &self.model
    }
}
