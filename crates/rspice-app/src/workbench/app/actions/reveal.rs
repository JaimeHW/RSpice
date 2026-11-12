//! Showing one placed instance on the drawing it is in, from the whole
//! application state. The transaction is the workbench's `app::reveal`,
//! which every list that names an instance elsewhere comes through.

use crate::state::InstancePath;
use crate::workbench::app_state::AppState;
pub(in crate::workbench) use rspice_app_workbench::workbench::app::reveal::{
    placed_instance_in, reaches_in,
};

/// Show the design with this instance selected and centred.
pub(in crate::workbench) fn placed_instance(
    state: &mut AppState,
    occurrence: Option<&InstancePath>,
    component_id: u64,
) {
    placed_instance_in(
        &mut state.model,
        &mut state.workbench,
        occurrence,
        component_id,
    );
}
