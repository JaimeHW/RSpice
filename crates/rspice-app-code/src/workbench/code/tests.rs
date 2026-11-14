//! The code feature on its own, for its tests: what a test that only seeds
//! and reads code's state runs over, in place of the application.

use rspice_app_state::workbench::workspace::restore_model_library_manager;

use crate::state::pdk_config::PdkConfig;
use crate::workbench::app_state::Model;
use crate::workbench::documents::code_workspace::bootstrap_automation;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::lifecycle::project_lifecycle::mark_pristine;
use crate::workbench::state::WorkbenchSession;

use super::CodeFeature;

/// The code feature's harness.
pub(crate) type CodeHarness = FeatureHarness<CodeFeature>;

/// The code feature on its own, over what the application's test instance
/// starts with: the model over the empty PDK configuration a test session
/// reads, a new session's workbench, the Automation workspace compiled as a
/// new session compiles it, the untitled project marked pristine, and the
/// managed runtime authorized for that project but never discovered.
pub(crate) fn harness() -> CodeHarness {
    let model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    let mut harness = CodeHarness::with_model(model, WorkbenchSession::default());
    bootstrap_automation(&harness.model.workspace, &mut harness.state.code_workspace);
    harness.own.automation_runtime_project_id = harness.model.workspace.content.project.id();
    mark_pristine(&mut harness);
    harness
}
