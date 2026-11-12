//! Check Design over a run host: the check the simulation campaign runs
//! before it queues. The Design menu's checks are the design feature's.

use crate::workbench::app::actions::workspace::design_checks::record_check;
use rspice_app_runs::simulation::RunHost;

/// Check the whole design from the simulation root, under its configuration,
/// and say what it found, as the design feature's Check design does.
pub(crate) fn check_design(host: &mut impl RunHost) {
    let checked = rspice_app_runs::workbench::preflight::check_design(host);
    if record_check(host.model_mut(), "Design checks", checked) {
        host.show_problem_findings();
    }
}
