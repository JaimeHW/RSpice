//! Checking the design from the workbench.
//!
//! The model checks the design as it is configured. Where the drawing's
//! annotation could not be restored, the editor brings it into the workspace
//! first, so the check reads what the reader sees.

use rspice_design::drc::DrcResult;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_checks::CheckScope;

impl AppState {
    /// Check Cellview: the open cellview's own findings.
    ///
    /// The hierarchy below is walked only for what no single cellview can
    /// answer — whether the design reaches ground and whether a cycle runs
    /// through it — so it answers in the time the one drawing takes.
    pub(crate) fn check_active_cellview(&mut self) -> Result<DrcResult, String> {
        if !matches!(
            self.model.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            return Err("only a schematic or a testbench has design rules to check".to_owned());
        }
        let subject = self.model.workspace.content.active_schematic_reference();
        self.run_design_check(subject, CheckScope::Cellview)
    }

    /// Check Design: the configured design, walked from the simulation root,
    /// with every schematic of the project's own libraries it never reaches.
    pub(crate) fn check_design(&mut self) -> Result<DrcResult, String> {
        let root = self.model.workspace.content.simulation_root_reference();
        self.run_design_check(root, CheckScope::Design)
    }

    /// Check `subject` as far as `scope` reaches, unless a check of it that
    /// reached exactly as far still describes the design.
    ///
    /// For a workflow that decides from the checks rather than being one:
    /// Check and Save and the run's preflight read the receipt this leaves
    /// current, and the page lists what that check published, so none of
    /// them can say something else about the same design.
    pub(in crate::workbench) fn ensure_design_check(
        &mut self,
        subject: CellViewRef,
        scope: CheckScope,
    ) -> Result<(), String> {
        if self.current_design_check(&subject, scope).is_some() {
            return Ok(());
        }
        self.run_design_check(subject, scope).map(|_| ())
    }

    /// The open cellview and every cellview its walk reaches.
    pub(crate) fn run_active_design_checks(&mut self) -> Result<DrcResult, String> {
        let subject = self.model.workspace.content.active_schematic_reference();
        self.run_design_check(subject, CheckScope::Hierarchy)
    }

    // Application half of a model operation; it becomes a request.
    fn run_design_check(
        &mut self,
        subject: CellViewRef,
        scope: CheckScope,
    ) -> Result<DrcResult, String> {
        if self
            .model
            .workspace
            .annotation_restoration_error()
            .is_some()
        {
            self.sync_active_schematic_to_workspace();
        }
        let facts = self.slot_facts();
        self.model.run_design_check(subject, scope, facts)
    }
}
