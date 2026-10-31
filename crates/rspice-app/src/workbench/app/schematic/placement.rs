//! Bind catalog placement payloads to the current app-owned document context.

use crate::state::LibraryCellInstance;
use crate::workbench::app_state::AppState;
use rspice_app_state::workbench::app::schematic::placement::arm_cell_instance;

/// Arm a frozen cell binding and return the label used by placement receipts.
pub(crate) fn arm_library_cell_placement(
    state: &mut AppState,
    binding: LibraryCellInstance,
) -> String {
    arm_cell_instance(&mut state.model, binding)
}
