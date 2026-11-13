//! What the simulate feature's own tests run against: the shared model.
//!
//! The field carries the name `AppState` gives the same value, so a test
//! body reads the same against either and only its constructor differs. A
//! test that reads a slot, the session or the command registry, or draws a
//! frame, runs against the whole application instead.

use rspice_app_state::workbench::workspace::restore_model_library_manager;

use crate::state::pdk_config::PdkConfig;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::state::WorkbenchSession;

/// The shared model the simulate feature reads, under the name `AppState`
/// gives it.
#[derive(Clone)]
pub(crate) struct SimulateTestHost {
    pub(crate) model: Model,
}

impl Default for SimulateTestHost {
    /// The model a new session starts with, over the empty PDK configuration
    /// a test session reads.
    fn default() -> Self {
        Self {
            model: Model::session_start(PdkConfig::default(), None, restore_model_library_manager),
        }
    }
}

impl SimulateTestHost {
    /// The simulate feature on its own, for a test that lends its context:
    /// the model a new session starts with, a new session, and every peer
    /// answering as a fresh session does.
    pub(crate) fn harness() -> FeatureHarness<SimulateFeature> {
        FeatureHarness::with_model(Self::default().model, WorkbenchSession::default())
    }
}
