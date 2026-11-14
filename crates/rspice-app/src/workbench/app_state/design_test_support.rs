//! The design feature's operations the tests take over the whole
//! application state.
//!
//! Each is the session form a test drives the design feature through, kept
//! with the state it reads rather than in the feature's own code.

use rspice_design::drc::DrcResult;

use super::AppState;
use crate::workbench::app::actions::workspace::design_checks::check_active_cellview;

impl AppState {
    /// [`check_active_cellview`] for the session.
    pub(crate) fn check_active_cellview(&mut self) -> Result<DrcResult, String> {
        let facts = self.slot_facts();
        check_active_cellview(&mut self.model, &mut self.workbench, facts)
    }

    /// Check Design: the configured design, walked from the simulation root,
    /// with every schematic of the project's own libraries it never reaches.
    pub(crate) fn check_design(&mut self) -> Result<DrcResult, String> {
        rspice_app_runs::workbench::preflight::check_design(self)
    }
}
