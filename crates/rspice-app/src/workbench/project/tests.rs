//! What the project feature's own tests run against.

use rspice_app_state::workbench::workspace::restore_model_library_manager;

use crate::state::pdk_config::PdkConfig;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::state::WorkbenchSession;

use super::ProjectFeature;

/// The project feature on its own: nothing of its own open, over the model a
/// new session starts with (from the empty PDK configuration a test session
/// reads) and a new session, with every peer answering as a fresh session
/// does.
pub(crate) fn harness() -> FeatureHarness<ProjectFeature> {
    FeatureHarness::with_model(
        Model::session_start(PdkConfig::default(), None, restore_model_library_manager),
        WorkbenchSession::default(),
    )
}
