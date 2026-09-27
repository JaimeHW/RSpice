//! Authoritative retained run evidence, lifecycle and result ownership.
//!
//! Where a run executes, and the lifecycle it moves through from queued to
//! finished. The lifecycle is explicit because a queued re-run, a cancelled
//! run, and a failed run are all different things to the UI.

mod evidence_domain;
pub use evidence_domain::EvidenceDomain;

use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_result::AnalysisResult;
use crate::provenance::AnalysisResultSourceDomain;
use crate::result_digest::ResultDigestEncoding;
use crate::run_receipt::{PreparedRunReceipt, SimulationRunProvenance};
use crate::specification_verdict::SpecificationVerdict;
use crate::waveform::RetainedWaveform;
use rspice_app_types::product::{
    AnalysisInstanceId, ContentDigest, DatasetId, JobId, RunId, SimulationCampaignId,
};

/// Immutable membership of one run in a reviewed multi-plan campaign.
///
/// A campaign groups independently authenticated plan runs. It never replaces
/// their run, job, dataset, or prepared-snapshot identity.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationCampaignMembership {
    campaign_id: SimulationCampaignId,
    name: String,
    member_index: u32,
    member_count: u32,
}

impl SimulationCampaignMembership {
    pub fn new(
        campaign_id: SimulationCampaignId,
        name: impl Into<String>,
        member_index: u32,
        member_count: u32,
    ) -> Result<Self, String> {
        let membership = Self {
            campaign_id,
            name: name.into().trim().to_owned(),
            member_index,
            member_count,
        };
        membership.validate()?;
        Ok(membership)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("simulation campaign name must not be empty".to_owned());
        }
        if self.name.chars().count() > 160 {
            return Err("simulation campaign name must not exceed 160 characters".to_owned());
        }
        if self.member_count < 2 {
            return Err("simulation campaign must contain at least two plan members".to_owned());
        }
        if self.member_index == 0 || self.member_index > self.member_count {
            return Err(format!(
                "simulation campaign member index {} is outside 1..={}",
                self.member_index, self.member_count
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn campaign_id(&self) -> SimulationCampaignId {
        self.campaign_id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn member_index(&self) -> u32 {
        self.member_index
    }

    #[must_use]
    pub const fn member_count(&self) -> u32 {
        self.member_count
    }
}

/// Qualified execution runtime that owns a simulation job.
///
/// The target is retained with the run so Jobs, exported manifests, and
/// project reloads never infer execution provenance from the machine that is
/// currently viewing a historical result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionTarget {
    LocalDesktop,
    NativeMobile,
    BrowserWorker,
}

/// Authoritative lifecycle retained with a simulation run.
///
/// `LegacyUnknown` is deliberately distinct from every current lifecycle:
/// older project files did not retain enough evidence to reconstruct whether
/// a run completed, failed, or was cancelled. New runs never use that value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SimulationRunLifecycle {
    LegacyUnknown,
    Preparing,
    Running,
    Cancelling,
    Completed,
    Failed,
    Aborted,
    /// The executor ownership boundary was lost before a terminal engine
    /// acknowledgement could be retained (for example, project reload or
    /// design-context replacement). This is terminal and never resumable.
    Interrupted,
}

impl SimulationRunLifecycle {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Aborted | Self::Interrupted
        )
    }
}

/// Stable identity for the exact execution currently owned by the runner.
/// Cancellation requests carry this pair so a delayed UI action cannot abort
/// a newer run that happens to be active when the request is processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimulationExecutionIdentity {
    pub job_id: JobId,
    pub run_id: RunId,
}

impl ExecutionTarget {
    #[must_use]
    pub const fn current() -> Self {
        if cfg!(target_arch = "wasm32") {
            Self::BrowserWorker
        } else if cfg!(any(target_os = "android", target_os = "ios")) {
            Self::NativeMobile
        } else {
            Self::LocalDesktop
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalDesktop => "Local desktop engine",
            Self::NativeMobile => "Native mobile engine",
            Self::BrowserWorker => "Browser simulation worker",
        }
    }

    #[must_use]
    pub const fn runtime(self) -> &'static str {
        match self {
            Self::LocalDesktop => "native",
            Self::NativeMobile => "native-mobile",
            Self::BrowserWorker => "wasm-worker",
        }
    }
}

/// Whether retention pruning is allowed to discard a run.
///
/// A named classification rather than a flag: pruning reads it as a policy
/// decision, and a golden baseline is a claim about the dataset — this is the
/// result a regression is signed off against — not a UI preference about
/// which row to keep.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunRetention {
    /// Ordinary history. The oldest of these is the first thing the retention
    /// limit discards.
    #[default]
    Pruneable,
    /// A regression baseline. Retention never discards one, so the reference a
    /// sign-off is measured against cannot be lost to routine iteration.
    GoldenBaseline,
}

impl RunRetention {
    /// Whether retention must keep this run whatever the limit says.
    #[must_use]
    pub const fn is_pinned(&self) -> bool {
        matches!(self, Self::GoldenBaseline)
    }

    /// Whether retention may discard this run.
    #[must_use]
    pub const fn is_pruneable(&self) -> bool {
        !self.is_pinned()
    }
}

/// A simulation run containing multiple analysis results.
///
/// This represents a single invocation of the simulator, which may contain
/// multiple analyses (e.g., DC Op + Transient + AC in one run).
///
/// Follows Cadence Spectre PSF database conventions:
/// - Timestamped runs for identification
/// - Multiple analyses per run
/// - Retained in history for comparison
#[derive(Debug, Clone)]
pub struct SimulationRun<A = AnalysisResult> {
    /// Stable execution-job identity. Historical projects that predate job
    /// identity retain `None`; current dispatches always allocate one.
    pub job_id: Option<JobId>,
    /// Stable product identity. This survives project moves, history reorder,
    /// and save/reload; it is the authoritative reference for selections and
    /// dataset provenance.
    pub run_id: RunId,
    /// Stable identity of the result dataset assembled by this run. The
    /// dataset contains the run's analyses and becomes the immutable unit
    /// addressed by result documents, comparisons, and overlays after
    /// completion.
    pub dataset_id: DatasetId,
    /// Runtime that executed this run. `None` is an explicit legacy state,
    /// never permission to infer the target from the current platform.
    pub execution_target: Option<ExecutionTarget>,
    /// Persisted lifecycle evidence for this exact run. Current dispatches
    /// transition this value monotonically; migrated history is explicitly
    /// `LegacyUnknown` rather than inferred from incomplete result vectors.
    pub lifecycle: SimulationRunLifecycle,
    /// Human-facing run sequence (monotonically increasing within a project).
    /// It is a label/order value, not persistent object identity.
    pub id: u64,
    /// Human-readable label with timestamp (e.g., "Run 3 (1:04:01 PM)")
    pub label: String,
    /// Unix timestamp when run started
    pub timestamp: f64,
    /// All analysis results from this run
    pub analyses: Vec<A>,
    /// Durable run-level authority. `None` is a transient, unsealed run and
    /// must never be persisted as a legacy classification.
    provenance: Option<SimulationRunProvenance>,
    /// What retention may do with this run. Private because changing it
    /// changes what the project's limit is allowed to discard, and that policy
    /// is applied by the state rather than by whoever holds the run.
    retention: RunRetention,
    /// Observed elapsed duration of this run in seconds, sealed at completion.
    pub elapsed_time: f64,
    /// Whether all analyses in this run succeeded
    pub success: bool,
    /// Terminal judgments against the exact specification definitions sealed
    /// into this run. `None` is reserved for legacy history that never carried
    /// frozen specifications, or for a run that has not reached a terminal
    /// lifecycle yet.
    specification_verdicts: Option<Vec<SpecificationVerdict>>,
    /// Optional immutable grouping identity for a multi-plan campaign.
    campaign_membership: Option<SimulationCampaignMembership>,
}

impl<A> AsRef<SimulationRun<A>> for SimulationRun<A> {
    fn as_ref(&self) -> &SimulationRun<A> {
        self
    }
}

impl<A> AsMut<SimulationRun<A>> for SimulationRun<A> {
    fn as_mut(&mut self) -> &mut SimulationRun<A> {
        self
    }
}

impl<A> SimulationRun<A> {
    /// Digest the ordered retained analyses with the selected canonical encoding.
    /// Run addresses, timestamps and waveform presentation do not enter this identity.
    #[must_use]
    pub fn dataset_content_digest_with_encoding<W: AsRef<RetainedWaveform>>(
        &self,
        version: ResultDigestEncoding,
    ) -> ContentDigest
    where
        A: AsRef<AnalysisResult<W>>,
    {
        crate::result_digest::dataset_content_digest(
            self.analyses
                .iter()
                .map(AsRef::as_ref)
                .map(|analysis| (analysis.id, analysis.result_data_ref())),
            version,
        )
    }

    /// Create retained run evidence using the host-supplied timestamp and target.
    pub fn new(run_number: u64, timestamp: f64, execution_target: ExecutionTarget) -> Self {
        let time_str = Self::format_time(timestamp);
        Self {
            job_id: Some(JobId::new()),
            run_id: RunId::new(),
            dataset_id: DatasetId::new(),
            execution_target: Some(execution_target),
            lifecycle: SimulationRunLifecycle::Preparing,
            id: run_number,
            label: format!("Run {} ({})", run_number, time_str),
            timestamp,
            analyses: Vec::new(),
            provenance: None,
            retention: RunRetention::Pruneable,
            elapsed_time: 0.0,
            success: true,
            specification_verdicts: None,
            campaign_membership: None,
        }
    }

    #[must_use]
    pub fn campaign_membership(&self) -> Option<&SimulationCampaignMembership> {
        self.campaign_membership.as_ref()
    }

    pub fn set_campaign_membership(
        &mut self,
        membership: SimulationCampaignMembership,
    ) -> Result<(), String> {
        if self.campaign_membership.is_some() {
            return Err(format!(
                "simulation run {} already belongs to a campaign",
                self.id
            ));
        }
        membership.validate()?;
        self.campaign_membership = Some(membership);
        Ok(())
    }

    pub fn restore_campaign_membership(
        &mut self,
        membership: Option<SimulationCampaignMembership>,
    ) -> Result<(), String> {
        if let Some(membership) = &membership {
            membership.validate()?;
        }
        self.campaign_membership = membership;
        Ok(())
    }

    /// What retention may do with this run.
    #[must_use]
    pub fn retention(&self) -> RunRetention {
        self.retention
    }

    /// Reclassify the run for retention. Callers inside the state go through
    /// the application state coordinator, which resolves the run by its
    /// stable identity; project restore uses this directly because it is
    /// rebuilding the history rather than editing it.
    pub fn set_retention(&mut self, retention: RunRetention) {
        self.retention = retention;
    }

    /// Stable execution identity, available for current runs and absent for
    /// legacy records that predate job identity.
    #[must_use]
    pub fn execution_identity(&self) -> Option<SimulationExecutionIdentity> {
        self.job_id.map(|job_id| SimulationExecutionIdentity {
            job_id,
            run_id: self.run_id,
        })
    }

    /// Advance the non-terminal lifecycle after the execution engine accepts
    /// the first prepared task.
    pub fn mark_running(&mut self) -> Result<(), String> {
        self.transition_lifecycle(SimulationRunLifecycle::Running)
    }

    /// Record an identity-bound cancellation request while the engine
    /// acknowledges it.
    pub fn mark_cancelling(&mut self) -> Result<(), String> {
        self.transition_lifecycle(SimulationRunLifecycle::Cancelling)
    }

    /// Seal a terminal lifecycle and its monotonic elapsed duration together.
    pub fn finish_lifecycle<W: AsRef<RetainedWaveform>>(
        &mut self,
        terminal: SimulationRunLifecycle,
        elapsed: impl FnOnce() -> Result<std::time::Duration, String>,
    ) -> Result<(), String>
    where
        A: AsRef<AnalysisResult<W>>,
    {
        if !terminal.is_terminal() {
            return Err(format!("{terminal:?} is not a terminal run lifecycle"));
        }
        let verdicts = match self.prepared_receipt() {
            Some(receipt) => Some(SpecificationVerdict::evaluate(
                receipt.specifications(),
                self.analyses.iter().map(AsRef::as_ref),
            )),
            None => None,
        };
        if let Some(existing) = &self.specification_verdicts
            && Some(existing) != verdicts.as_ref()
        {
            return Err(format!(
                "simulation run {} terminal specification verdicts are immutable",
                self.id
            ));
        }
        // A repeated acknowledgement validates the immutable judgments but
        // cannot rewrite elapsed time or upgrade a legacy verdict record.
        if self.lifecycle.is_terminal() {
            return self.transition_lifecycle(terminal);
        }
        let elapsed = elapsed()?.as_secs_f64();
        self.transition_lifecycle(terminal)?;
        self.specification_verdicts = verdicts;
        self.elapsed_time = elapsed;
        Ok(())
    }

    pub fn validate_elapsed_time(elapsed: f64) -> Result<(), String> {
        if !elapsed.is_finite() || elapsed < 0.0 {
            return Err("run elapsed time must be finite and nonnegative".to_owned());
        }
        Ok(())
    }

    /// Restore timing evidence without claiming ownership of the old executor.
    /// Active records become interrupted; historical durations remain exact.
    pub fn restore_lifecycle(
        &mut self,
        lifecycle: SimulationRunLifecycle,
        elapsed: f64,
    ) -> Result<(), String> {
        Self::validate_elapsed_time(elapsed)?;
        self.lifecycle = match lifecycle {
            SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling => SimulationRunLifecycle::Interrupted,
            lifecycle => lifecycle,
        };
        self.elapsed_time = elapsed;
        Ok(())
    }

    fn transition_lifecycle(&mut self, next: SimulationRunLifecycle) -> Result<(), String> {
        use SimulationRunLifecycle as Lifecycle;

        let valid = self.lifecycle == next
            || matches!(
                (self.lifecycle, next),
                (
                    Lifecycle::Preparing,
                    Lifecycle::Running
                        | Lifecycle::Cancelling
                        | Lifecycle::Failed
                        | Lifecycle::Aborted
                        | Lifecycle::Interrupted
                ) | (
                    Lifecycle::Running,
                    Lifecycle::Cancelling
                        | Lifecycle::Completed
                        | Lifecycle::Failed
                        | Lifecycle::Aborted
                        | Lifecycle::Interrupted
                ) | (
                    Lifecycle::Cancelling,
                    Lifecycle::Completed
                        | Lifecycle::Failed
                        | Lifecycle::Aborted
                        | Lifecycle::Interrupted
                )
            );
        if !valid {
            return Err(format!(
                "simulation run {} cannot transition from {:?} to {:?}",
                self.id, self.lifecycle, next
            ));
        }
        self.lifecycle = next;
        Ok(())
    }

    pub fn new_prepared(
        run_number: u64,
        timestamp: f64,
        execution_target: ExecutionTarget,
        receipt: PreparedRunReceipt,
    ) -> Self {
        let mut run = Self::new(run_number, timestamp, execution_target);
        run.provenance = Some(SimulationRunProvenance::Prepared(Box::new(receipt)));
        run
    }

    /// Authoritative persisted classification for this run. Fresh fixture or
    /// UI-only runs remain `None` until dispatch or migration seals them.
    #[must_use]
    pub fn provenance(&self) -> Option<&SimulationRunProvenance> {
        self.provenance.as_ref()
    }

    /// Exact consumed-snapshot receipt for a current prepared run.
    #[must_use]
    pub fn prepared_receipt(&self) -> Option<&PreparedRunReceipt> {
        match self.provenance.as_ref() {
            Some(SimulationRunProvenance::Prepared(receipt)) => Some(receipt),
            None
            | Some(
                SimulationRunProvenance::LegacyUnattributed
                | SimulationRunProvenance::LegacyPreparedUnclassified,
            ) => None,
        }
    }

    /// Immutable terminal specification judgments, when this result era
    /// retained them.
    #[must_use]
    pub fn specification_verdicts(&self) -> Option<&[SpecificationVerdict]> {
        self.specification_verdicts.as_deref()
    }

    /// Whether the exact governed policy sealed with this run prevents the
    /// retained result from being accepted as satisfying its requirements.
    #[must_use]
    pub fn specification_acceptance_is_blocked<W: AsRef<RetainedWaveform>>(&self) -> bool
    where
        A: AsRef<AnalysisResult<W>>,
    {
        let Some(receipt) = self.prepared_receipt() else {
            return false;
        };
        let Some(verdicts) = self.specification_verdicts() else {
            return !receipt.specifications().is_empty();
        };
        crate::specification_verdict::acceptance_is_blocked(
            receipt.specifications(),
            receipt.specification_policy().policy(),
            verdicts,
            self.analyses.iter().map(AsRef::as_ref),
        )
    }

    /// Restore terminal judgments and prove them against the frozen receipt
    /// and retained analysis evidence instead of trusting serialized labels.
    pub fn restore_specification_verdicts<W: AsRef<RetainedWaveform>>(
        &mut self,
        verdicts: Option<Vec<SpecificationVerdict>>,
    ) -> Result<(), String>
    where
        A: AsRef<AnalysisResult<W>>,
    {
        if !self.lifecycle.is_terminal() {
            if verdicts.is_some() {
                return Err(format!(
                    "non-terminal simulation run {} cannot carry specification verdicts",
                    self.id
                ));
            }
            self.specification_verdicts = None;
            return Ok(());
        }
        let Some(receipt) = self.prepared_receipt() else {
            if verdicts.is_some() {
                return Err(format!(
                    "legacy simulation run {} cannot claim current specification verdicts",
                    self.id
                ));
            }
            self.specification_verdicts = None;
            return Ok(());
        };
        if verdicts.is_none() && receipt.specifications().is_empty() {
            // Prepared histories written before frozen specifications existed
            // carry no requirement rows and make no reconstructed claim.
            self.specification_verdicts = None;
            return Ok(());
        }
        let expected = SpecificationVerdict::evaluate(
            receipt.specifications(),
            self.analyses.iter().map(AsRef::as_ref),
        );
        let verdicts = verdicts.ok_or_else(|| {
            format!(
                "simulation run {} has frozen specifications but no terminal verdicts",
                self.id
            )
        })?;
        if verdicts != expected {
            return Err(format!(
                "simulation run {} specification verdicts do not match its frozen requirements and retained evidence",
                self.id
            ));
        }
        self.specification_verdicts = Some(verdicts);
        Ok(())
    }

    /// Seal the deterministic missing/partial judgments created when a saved
    /// in-flight run is restored as interrupted. This is the sole migration
    /// path allowed to synthesize a current verdict from an absent field.
    pub fn seal_interrupted_specification_verdicts<W: AsRef<RetainedWaveform>>(
        &mut self,
    ) -> Result<(), String>
    where
        A: AsRef<AnalysisResult<W>>,
    {
        if self.lifecycle != SimulationRunLifecycle::Interrupted
            || self.specification_verdicts.is_some()
        {
            return Err(format!(
                "simulation run {} is not an unsealed interrupted run",
                self.id
            ));
        }
        let receipt = self.prepared_receipt().ok_or_else(|| {
            format!(
                "interrupted simulation run {} has no prepared-run authority",
                self.id
            )
        })?;
        self.specification_verdicts = Some(SpecificationVerdict::evaluate(
            receipt.specifications(),
            self.analyses.iter().map(AsRef::as_ref),
        ));
        Ok(())
    }

    /// Restore an explicit persisted classification without inferring it from
    /// the current result vector. Reserved for versioned project migration.
    pub fn restore_provenance<W: AsRef<RetainedWaveform>>(
        &mut self,
        provenance: SimulationRunProvenance,
    ) -> Result<(), String>
    where
        A: AsRef<AnalysisResult<W>>,
    {
        if self.provenance.is_some() {
            return Err(format!(
                "simulation run {} already has authoritative provenance",
                self.id
            ));
        }
        Self::validate_provenance_against_results::<W>(&provenance, &self.analyses)?;
        self.provenance = Some(provenance);
        Ok(())
    }

    /// Validate that retained results are an exact ordered prefix of the
    /// authoritative receipt. Aborted and partial runs may omit the remaining
    /// suffix; they may not rewrite or reorder completed task identity.
    pub fn validate_provenance<W: AsRef<RetainedWaveform>>(&self) -> Result<(), String>
    where
        A: AsRef<AnalysisResult<W>>,
    {
        if let Some(membership) = &self.campaign_membership {
            membership.validate()?;
        }
        let provenance = self.provenance.as_ref().ok_or_else(|| {
            format!(
                "simulation run {} is unsealed and has no authoritative provenance",
                self.id
            )
        })?;
        Self::validate_provenance_against_results::<W>(provenance, &self.analyses)
    }

    fn validate_provenance_against_results<W: AsRef<RetainedWaveform>>(
        provenance: &SimulationRunProvenance,
        analyses: &[A],
    ) -> Result<(), String>
    where
        A: AsRef<AnalysisResult<W>>,
    {
        match provenance {
            SimulationRunProvenance::LegacyUnattributed => {
                if analyses
                    .iter()
                    .any(|analysis| analysis.as_ref().provenance.is_some())
                {
                    return Err(
                        "legacy-unattributed run contains prepared analysis provenance".to_owned(),
                    );
                }
            }
            SimulationRunProvenance::LegacyPreparedUnclassified => {
                if analyses.iter().any(|analysis| {
                    analysis
                        .as_ref()
                        .provenance
                        .as_ref()
                        .is_none_or(|provenance| {
                            provenance.source_domain()
                                != AnalysisResultSourceDomain::LegacyUnclassified
                        })
                }) {
                    return Err(
                        "legacy prepared-unclassified run has missing or classified analysis provenance"
                            .to_owned(),
                    );
                }
            }
            SimulationRunProvenance::Prepared(receipt) => {
                receipt.validate_result_prefix(analyses.iter().map(AsRef::as_ref))?;
            }
        }
        Ok(())
    }

    /// Canonical SOA warning/violation devices for an exact project revision.
    pub fn soa_violation_context<W: AsRef<RetainedWaveform>>(
        &self,
        project_revision: rspice_app_types::product::ObjectRevision,
    ) -> Option<(rspice_app_types::product::ContentDigest, Vec<String>)>
    where
        A: AsRef<AnalysisResult<W>>,
    {
        let receipt = self.prepared_receipt()?;
        if receipt.project_revision() != project_revision {
            return None;
        }
        let mut devices = self
            .analyses
            .iter()
            .flat_map(|analysis| match analysis.as_ref().result_payload.as_ref() {
                Some(AnalysisResultPayload::Soa { violations, .. }) => violations
                    .iter()
                    .map(|violation| violation.device_id.clone())
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .collect::<Vec<_>>();
        devices.sort();
        devices.dedup();
        (!devices.is_empty()).then(|| (receipt.source_content_digest(), devices))
    }

    /// Add an analysis result to this run
    pub fn add_analysis<W: AsRef<RetainedWaveform>>(&mut self, mut analysis: A)
    where
        A: AsRef<AnalysisResult<W>> + AsMut<AnalysisResult<W>>,
    {
        let data = analysis.as_mut();
        if data.id == 0
            || self
                .analyses
                .iter()
                .any(|existing| existing.as_ref().id == data.id)
        {
            data.id = self.next_available_analysis_id::<W>();
        }
        if !data.success {
            self.success = false;
        }
        self.analyses.push(analysis);
    }

    pub fn next_available_analysis_id<W: AsRef<RetainedWaveform>>(&self) -> u64
    where
        A: AsRef<AnalysisResult<W>>,
    {
        let mut next_id = self
            .analyses
            .iter()
            .map(|analysis| analysis.as_ref().id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .expect("analysis id space exhausted");
        while self
            .analyses
            .iter()
            .any(|analysis| analysis.as_ref().id == next_id)
        {
            next_id = next_id.checked_add(1).expect("analysis id space exhausted");
        }
        next_id
    }

    /// Find the result produced by one exact prepared analysis instance.
    /// Searching by source remains unambiguous when a run has
    /// multiple analyses of the same kind.
    pub fn find_analysis_by_source_instance<W: AsRef<RetainedWaveform>>(
        &self,
        source_instance_id: AnalysisInstanceId,
    ) -> Option<&A>
    where
        A: AsRef<AnalysisResult<W>>,
    {
        self.analyses.iter().rev().find(|analysis| {
            analysis
                .as_ref()
                .provenance
                .as_ref()
                .is_some_and(|provenance| {
                    provenance.authored_source_instance_id() == source_instance_id
                })
        })
    }

    /// Format timestamp as human-readable time string (e.g., "1:04:01 PM")
    fn format_time(timestamp: f64) -> String {
        use std::time::{Duration, UNIX_EPOCH};
        let _datetime = UNIX_EPOCH + Duration::from_secs_f64(timestamp);
        // Use chrono-free approach for portability
        let secs = timestamp as u64;
        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;
        let (hour_12, am_pm) = if hours == 0 {
            (12, "AM")
        } else if hours < 12 {
            (hours, "AM")
        } else if hours == 12 {
            (12, "PM")
        } else {
            (hours - 12, "PM")
        };
        format!("{}:{:02}:{:02} {}", hour_12, minutes, seconds, am_pm)
    }
}
