//! The model's operations that read the features' state, for a caller
//! holding the whole application state.
//!
//! The model never reaches up into a slot. Each operation here reads what
//! the model needs from the slots — the [`SlotFacts`], the source editors'
//! findings — when it starts, as the model read them before, and hands them
//! down.

use super::diagnostic_store::listing::{ListingInputs, ListingSource};
use super::diagnostic_store::{BatchStanding, DiagnosticBatch, DiagnosticInputs, DiagnosticRecord};
use super::{AppState, SlotFacts, design_checks};
use crate::product::ContentDigest;

impl AppState {
    /// What model code reads of the features' state, as it stands now.
    pub(crate) fn slot_facts(&self) -> SlotFacts {
        SlotFacts {
            preflight_report_retained: self.runs.preflight.report.is_some(),
            review_markers: self.design.schematic_visibility.review_markers,
        }
    }

    // Application half of a model operation; it becomes a request.
    pub(in crate::workbench) fn publish_diagnostics(
        &mut self,
        inputs: DiagnosticInputs,
        records: Vec<DiagnosticRecord>,
    ) -> Result<(), String> {
        let facts = self.slot_facts();
        self.model.publish_diagnostics(inputs, records, facts)
    }

    // Application half of a model operation; it becomes a request.
    pub(in crate::workbench) fn publish_superseding(
        &mut self,
        inputs: DiagnosticInputs,
        records: Vec<DiagnosticRecord>,
        coverage: &design_checks::RuleCoverage,
    ) -> Result<(), String> {
        let facts = self.slot_facts();
        self.model
            .publish_superseding(inputs, records, coverage, facts)
    }

    pub(in crate::workbench) fn diagnostic_batch_standing(
        &self,
        batch: &DiagnosticBatch,
    ) -> BatchStanding {
        self.model
            .diagnostic_batch_standing(batch, self.slot_facts())
    }

    pub(in crate::workbench) fn diagnostic_live_digest(
        &self,
        inputs: &DiagnosticInputs,
    ) -> Result<ContentDigest, String> {
        self.model.diagnostic_live_digest(inputs, self.slot_facts())
    }

    pub(in crate::workbench) fn dispatch_inputs_at_press(&self) -> Result<ContentDigest, String> {
        self.model.dispatch_inputs_at_press(self.slot_facts())
    }

    // Application half of a model operation; it becomes a request.
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

    // Application half of a model operation; it becomes a request.
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

    // Application half of a model operation; it becomes a request.
    pub(crate) fn open_run_outcome(&mut self, run_sequence: u64) {
        let facts = self.slot_facts();
        self.model.open_run_outcome(run_sequence, facts);
    }

    // Application half of a model operation; it becomes a request.
    pub(in crate::workbench) fn publish_dispatch_diagnostics(
        &mut self,
        report: &crate::state::PreflightReport,
    ) {
        let facts = self.slot_facts();
        self.model.publish_dispatch_diagnostics(report, facts);
    }

    // Application half of a model operation; it becomes a request.
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
    pub(in crate::workbench) fn reconcile_current_finding(&mut self) {
        let sources = self.code.diagnostic_sources();
        self.model.reconcile_current_finding(&sources);
    }

    // Application half of a model operation; it becomes a request.
    pub(crate) fn publish_rule_deck_issues(&mut self) {
        let facts = self.slot_facts();
        self.model.publish_rule_deck_issues(facts);
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
