//! The model's operations that read the features' state, for a caller
//! holding the whole application state.
//!
//! The model never reaches up into a slot. Each operation here reads what
//! the model needs from the slots — the [`SlotFacts`], the source editors'
//! findings — when it starts, as the model read them before, and hands them
//! down.

use std::time::Duration;

use rspice_design::drc::{DrcLocation, DrcResult, DrcSeverity};

use super::design_checks::{CheckScope, DesignCheckStatus};
use super::diagnostic_store::listing::{ListingInputs, ListingSource};
#[cfg(test)]
use super::diagnostic_store::{BatchStanding, DiagnosticBatch};
use super::diagnostic_store::{DiagnosticInputs, DiagnosticRecord};
use super::{AppState, Model, SlotFacts};
use crate::state::CellViewRef;
use crate::time_compat::Instant;

/// The model a session holds, for the operations that read nothing else and
/// take any holder of it.
impl AsRef<Model> for AppState {
    fn as_ref(&self) -> &Model {
        &self.model
    }
}

impl AppState {
    /// [`Model::apply_schematic_editing_preferences`] for the session's model.
    pub(crate) fn apply_schematic_editing_preferences(&mut self) {
        self.model.apply_schematic_editing_preferences();
    }

    /// What model code reads of the features' state, as it stands now.
    pub(crate) fn slot_facts(&self) -> SlotFacts {
        SlotFacts {
            preflight_report_retained: self.runs.preflight.report.is_some(),
            review_markers: self.design.schematic_visibility.review_markers,
        }
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn publish_diagnostics(
        &mut self,
        inputs: DiagnosticInputs,
        records: Vec<DiagnosticRecord>,
    ) -> Result<(), String> {
        let facts = self.slot_facts();
        self.model.publish_diagnostics(inputs, records, facts)
    }

    #[cfg(test)]
    pub(crate) fn diagnostic_batch_standing(&self, batch: &DiagnosticBatch) -> BatchStanding {
        self.model
            .diagnostic_batch_standing(batch, self.slot_facts())
    }

    #[cfg(test)]
    pub(crate) fn publish_run_failure(
        &mut self,
        analysis: &str,
        message: &str,
        attribution: Option<&crate::state::ConvergenceAttribution>,
        run_sequence: u64,
    ) {
        let facts = self.slot_facts();
        self.model
            .publish_run_failure(analysis, message, attribution, run_sequence, facts);
    }

    #[cfg(test)]
    pub(crate) fn publish_campaign_member_refusal(
        &mut self,
        campaign: crate::product::SimulationCampaignId,
        member_index: u32,
        member: &str,
        refusal: crate::state::CampaignMemberRefusal,
        error: &str,
    ) {
        let facts = self.slot_facts();
        self.model.publish_campaign_member_refusal(
            campaign,
            member_index,
            member,
            refusal,
            error,
            facts,
        );
    }

    #[cfg(test)]
    pub(crate) fn open_run_outcome(&mut self, run_sequence: u64) {
        let facts = self.slot_facts();
        self.model.open_run_outcome(run_sequence, facts);
    }

    #[cfg(test)]
    pub(crate) fn publish_dispatch_diagnostics(&mut self, report: &crate::state::PreflightReport) {
        let facts = self.slot_facts();
        self.model.publish_dispatch_diagnostics(report, facts);
    }

    #[cfg(test)]
    pub(crate) fn publish_run_notices(
        &mut self,
        analysis: &str,
        notices: Vec<(rspice_core::notice::RecordedNotice, Option<usize>)>,
        transient: Option<(
            &rspice_results::convergence_quality::TransientConvergenceEvidence,
            Option<usize>,
        )>,
        failed_measurements: &[rspice_core::MeasureResult],
        run_sequence: u64,
    ) {
        let facts = self.slot_facts();
        self.model.publish_run_notices(
            analysis,
            notices,
            transient,
            failed_measurements,
            run_sequence,
            facts,
        );
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn reconcile_current_finding(&mut self) {
        let sources = self.code.diagnostic_sources();
        self.model.reconcile_current_finding(&sources);
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn publish_rule_deck_issues(&mut self) {
        let facts = self.slot_facts();
        self.model.publish_rule_deck_issues(facts);
    }

    pub(crate) fn design_check_status(&self, subject: &CellViewRef) -> DesignCheckStatus<'_> {
        self.model.design_check_status(subject, self.slot_facts())
    }

    pub(crate) fn active_design_check_status(&self) -> DesignCheckStatus<'_> {
        self.model.active_design_check_status(self.slot_facts())
    }

    pub(crate) fn simulation_root_design_check_status(&self) -> DesignCheckStatus<'_> {
        self.model
            .simulation_root_design_check_status(self.slot_facts())
    }

    pub(in crate::workbench) fn current_design_check(
        &self,
        subject: &CellViewRef,
        scope: CheckScope,
    ) -> Option<&DrcResult> {
        self.model
            .current_design_check(subject, scope, self.slot_facts())
    }

    pub(crate) fn current_finding_related_marks(
        &self,
        cell_view: &CellViewRef,
    ) -> Option<(DrcSeverity, Vec<DrcLocation>)> {
        self.model
            .current_finding_related_marks(cell_view, self.slot_facts())
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn publish_active_design_check_result(
        &mut self,
        result: DrcResult,
    ) -> Result<(), String> {
        let facts = self.slot_facts();
        self.model.publish_active_design_check_result(result, facts)
    }

    // Application half of a model operation; it becomes a request.
    #[cfg(test)]
    pub(in crate::workbench) fn publish_configured_root_design_check_result(
        &mut self,
        result: DrcResult,
    ) -> Result<(), String> {
        let facts = self.slot_facts();
        self.model
            .publish_configured_root_design_check_result(result, facts)
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn clear_active_design_check(&mut self) {
        let facts = self.slot_facts();
        self.model.clear_active_design_check(facts);
    }

    // Application half of a model operation; it becomes a request.
    pub(in crate::workbench) fn restate_design_checks(&mut self) {
        let facts = self.slot_facts();
        self.model.restate_design_checks(facts);
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn refresh_active_design_check_projection(&mut self) {
        let facts = self.slot_facts();
        self.model.refresh_active_design_check_projection(facts);
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn refresh_live_checks(&mut self, now: Instant) -> Option<Duration> {
        let facts = self.slot_facts();
        self.model.refresh_live_checks(now, facts)
    }

    // Application half of a model operation; it becomes a request.
    #[cfg(test)]
    pub(crate) fn run_live_check(&mut self, subject: CellViewRef) -> Result<DrcResult, String> {
        let facts = self.slot_facts();
        self.model.run_live_check(subject, facts)
    }

    #[cfg(test)]
    pub(crate) fn current_live_receipt(
        &self,
        subject: &CellViewRef,
    ) -> Option<&super::design_checks::CellViewCheckReceipt> {
        self.model.current_live_receipt(subject, self.slot_facts())
    }
}

impl ListingSource for AppState {
    fn listing_inputs(&self) -> ListingInputs<'_> {
        ListingInputs {
            model: &self.model,
            sources: self.code.diagnostic_sources(),
            facts: self.slot_facts(),
        }
    }
}
