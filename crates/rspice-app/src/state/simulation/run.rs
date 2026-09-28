//! Application run ownership with a live monotonic clock and provisional display.

use super::*;
#[cfg(test)]
use crate::product::AnalysisInstanceId;
use rspice_results::run::SimulationRun as RetainedRun;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct SimulationRun {
    pub data: RetainedRun<AnalysisResult>,
    elapsed_started_at: Option<crate::time_compat::Instant>,
}

impl Deref for SimulationRun {
    type Target = RetainedRun<AnalysisResult>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for SimulationRun {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl AsRef<RetainedRun<AnalysisResult>> for SimulationRun {
    fn as_ref(&self) -> &RetainedRun<AnalysisResult> {
        &self.data
    }
}

impl AsMut<RetainedRun<AnalysisResult>> for SimulationRun {
    fn as_mut(&mut self) -> &mut RetainedRun<AnalysisResult> {
        &mut self.data
    }
}

impl SimulationRun {
    pub(crate) fn from_restored(data: RetainedRun<AnalysisResult>) -> Self {
        Self {
            data,
            elapsed_started_at: None,
        }
    }

    /// Create current run evidence and start its live monotonic clock.
    pub fn new(run_number: u64) -> Self {
        Self {
            data: RetainedRun::new(
                run_number,
                Self::current_timestamp(),
                ExecutionTarget::current(),
            ),
            elapsed_started_at: Some(crate::time_compat::Instant::now()),
        }
    }

    pub(crate) fn new_prepared(run_number: u64, receipt: PreparedRunReceipt) -> Self {
        Self {
            data: RetainedRun::new_prepared(
                run_number,
                Self::current_timestamp(),
                ExecutionTarget::current(),
                receipt,
            ),
            elapsed_started_at: Some(crate::time_compat::Instant::now()),
        }
    }

    /// Seal retained judgments and duration before releasing the live clock.
    pub(crate) fn finish_lifecycle(
        &mut self,
        terminal: SimulationRunLifecycle,
    ) -> Result<(), String> {
        let was_terminal = self.lifecycle.is_terminal();
        let started = self.elapsed_started_at;
        let id = self.id;
        self.data.finish_lifecycle(terminal, || {
            started
                .ok_or_else(|| format!("simulation run {id} has no live timing evidence"))
                .map(|started| started.elapsed())
        })?;
        if !was_terminal {
            self.elapsed_started_at = None;
        }
        Ok(())
    }

    pub(crate) fn restore_lifecycle(
        &mut self,
        lifecycle: SimulationRunLifecycle,
        elapsed: f64,
    ) -> Result<(), String> {
        self.data.restore_lifecycle(lifecycle, elapsed)?;
        self.elapsed_started_at = None;
        Ok(())
    }

    #[must_use]
    pub fn prepared_receipt(&self) -> Option<&PreparedRunReceipt> {
        self.data.prepared_receipt()
    }

    #[must_use]
    pub fn execution_identity(&self) -> Option<SimulationExecutionIdentity> {
        self.data.execution_identity()
    }

    /// Publish or refresh the single provisional result for an in-flight
    /// prepared task. This does not change run success: the task has not
    /// reached a terminal outcome yet.
    pub(crate) fn upsert_live_analysis(
        &mut self,
        mut analysis: AnalysisResult,
    ) -> Result<(), String> {
        if !analysis.is_live_partial() {
            return Err("only a live partial analysis can use provisional retention".to_owned());
        }
        let instance = analysis
            .provenance()
            .map(AnalysisResultProvenance::source_instance_id)
            .ok_or_else(|| "live partial analysis has no prepared-task provenance".to_owned())?;
        if let Some(index) = self.analyses.iter().position(|existing| {
            existing
                .provenance()
                .is_some_and(|provenance| provenance.source_instance_id() == instance)
        }) {
            if !self.analyses[index].is_live_partial() {
                return Err(format!(
                    "prepared task {instance} already has a terminal retained result"
                ));
            }
            analysis.id = self.analyses[index].id;
            self.analyses[index] = analysis;
        } else {
            if analysis.id == 0
                || self
                    .analyses
                    .iter()
                    .any(|existing| existing.id == analysis.id)
            {
                analysis.id = self.next_available_analysis_id();
            }
            self.analyses.push(analysis);
        }
        self.validate_provenance()
    }

    /// Replace the provisional prefix for this exact task, or append the
    /// terminal result when no live presentation was published.
    pub(crate) fn replace_live_or_add_analysis(&mut self, mut analysis: AnalysisResult) {
        let matching_live = analysis.provenance().and_then(|provenance| {
            let instance = provenance.source_instance_id();
            self.analyses.iter().position(|existing| {
                existing.is_live_partial()
                    && existing
                        .provenance()
                        .is_some_and(|provenance| provenance.source_instance_id() == instance)
            })
        });
        if let Some(index) = matching_live {
            analysis.id = self.analyses[index].id;
            if !analysis.success {
                self.success = false;
            }
            self.analyses[index] = analysis;
        } else {
            self.add_analysis(analysis);
        }
    }

    #[cfg(test)]
    pub fn set_elapsed_time(&mut self, elapsed: f64) {
        self.elapsed_time = elapsed;
    }

    #[cfg(test)]
    pub fn find_analysis(&self, analysis_type: AnalysisType) -> Option<&AnalysisResult> {
        self.analyses
            .iter()
            .find(|a| a.analysis_type == analysis_type)
    }

    fn current_timestamp() -> f64 {
        crate::time_compat::unix_epoch().as_secs_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision, SimulationPlanId};

    fn prepared_spec_run(spec: crate::state::SpecEntry) -> (SimulationRun, AnalysisInstanceId) {
        let task_id = AnalysisInstanceId::new();
        let snapshot = ContentDigest::from_bytes([0x71; 32]);
        let task = PreparedRunTaskReceipt::new(
            task_id,
            ObjectRevision::INITIAL,
            Vec::new(),
            2,
            ContentDigest::from_bytes([0x72; 32]),
        )
        .expect("task receipt");
        let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: snapshot,
            source_content_digest: ContentDigest::from_bytes([0x73; 32]),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(
                ContentDigest::from_bytes([0x74; 32]),
            ),
            project_model_sources: Vec::new(),
            specifications: vec![PreparedSpecification::new(spec).expect("prepared spec")],
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![task],
        })
        .expect("prepared receipt");
        (SimulationRun::new_prepared(1, receipt), task_id)
    }

    #[test]
    fn add_analysis_assigns_unique_ids_when_converters_reuse_placeholder() {
        let mut run = SimulationRun::new(7);
        let stable_id = run.run_id;
        let dataset_id = run.dataset_id;
        let job_id = run.job_id;

        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC"));

        let ids: Vec<_> = run.analyses.iter().map(|analysis| analysis.id).collect();
        assert_eq!(ids, vec![1, 2]);
        assert_eq!(run.run_id, stable_id);
        assert_eq!(run.dataset_id, dataset_id);
        assert!(job_id.is_some());
        assert_eq!(run.job_id, job_id);
        assert_eq!(run.execution_target, Some(ExecutionTarget::current()));
        assert_eq!(run.lifecycle, SimulationRunLifecycle::Preparing);
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn run_duration_is_independent_of_wall_clock_adjustments() {
        for wall_shift in [-86_400.0, 86_400.0] {
            let observed = crate::time_compat::Instant::now();
            let mut run = SimulationRun::new(9);
            run.timestamp += wall_shift;
            run.mark_running().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(2));
            run.finish_lifecycle(SimulationRunLifecycle::Completed)
                .unwrap();
            assert!(
                run.elapsed_time >= 0.001,
                "wall clock jump erased elapsed time"
            );
            assert!(
                run.elapsed_time <= observed.elapsed().as_secs_f64(),
                "wall clock jump was counted as simulation time"
            );
        }
    }

    #[test]
    fn terminal_run_duration_is_immutable_on_duplicate_completion() {
        for terminal in [
            SimulationRunLifecycle::Completed,
            SimulationRunLifecycle::Failed,
            SimulationRunLifecycle::Aborted,
            SimulationRunLifecycle::Interrupted,
        ] {
            let mut run = SimulationRun::new(9);
            run.mark_running().unwrap();
            run.finish_lifecycle(terminal).unwrap();
            let sealed = run.elapsed_time.to_bits();
            run.timestamp -= 86_400.0;
            run.finish_lifecycle(terminal).unwrap();
            assert_eq!(run.elapsed_time.to_bits(), sealed, "{terminal:?}");
            assert_eq!(run.lifecycle, terminal);
            assert!(run.mark_running().is_err());
            assert_eq!(run.elapsed_time.to_bits(), sealed);
        }
    }

    #[test]
    fn invalid_restored_timing_does_not_mutate_the_live_run() {
        let mut run = SimulationRun::new(9);
        run.mark_running().unwrap();
        let started = run.elapsed_started_at;
        for invalid in [-0.1, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(
                run.restore_lifecycle(SimulationRunLifecycle::Completed, invalid)
                    .is_err()
            );
            assert_eq!(run.lifecycle, SimulationRunLifecycle::Running);
            assert_eq!(run.elapsed_time, 0.0);
            assert_eq!(run.elapsed_started_at, started);
        }
        run.restore_lifecycle(SimulationRunLifecycle::Running, 12.5)
            .unwrap();
        assert_eq!(run.lifecycle, SimulationRunLifecycle::Interrupted);
        assert_eq!(run.elapsed_time, 12.5);
        assert!(run.elapsed_started_at.is_none());
        run.finish_lifecycle(SimulationRunLifecycle::Interrupted)
            .unwrap();
        assert_eq!(run.elapsed_time, 12.5);
    }

    #[test]
    fn current_run_lifecycle_is_monotonic_and_seals_duration() {
        let mut run = SimulationRun::new(9);
        run.elapsed_started_at =
            Some(crate::time_compat::Instant::now() - std::time::Duration::from_millis(1));

        run.mark_running().expect("engine accepts the prepared run");
        run.mark_cancelling().expect("cancel request is retained");
        run.finish_lifecycle(SimulationRunLifecycle::Aborted)
            .expect("controller seals abort acknowledgement");

        assert_eq!(run.lifecycle, SimulationRunLifecycle::Aborted);
        assert!(run.elapsed_time > 0.0);
        assert!(
            run.mark_running().is_err(),
            "terminal lifecycle cannot be reopened"
        );
    }

    #[test]
    fn terminal_run_seals_verdict_against_frozen_specification() {
        let (mut run, task_id) = prepared_spec_run(crate::state::SpecEntry {
            measurement: "gain".to_owned(),
            expression: "param='gain'".to_owned(),
            min: Some(10.0),
            max: None,
            unit: "dB".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        });
        let snapshot = run
            .prepared_receipt()
            .expect("prepared receipt")
            .prepared_snapshot_digest();
        let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 9.5)])
            .with_provenance(
                AnalysisResultProvenance::new(
                    task_id,
                    ObjectRevision::INITIAL,
                    snapshot,
                    Vec::new(),
                )
                .expect("provenance"),
            );
        run.add_analysis(analysis);
        run.mark_running().expect("running");
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .expect("terminal seal");

        let verdicts = run.specification_verdicts().expect("sealed verdicts");
        assert_eq!(verdicts.len(), 1);
        assert_eq!(
            verdicts[0].status(),
            SpecificationVerdictStatus::BoundFailure
        );
        assert_eq!(verdicts[0].worst_value(), Some(9.5));
        assert_eq!(verdicts[0].signed_margin(), Some(-0.5));
        assert_eq!(verdicts[0].source_instance_id(), Some(task_id));
    }

    #[test]
    fn legacy_unknown_cannot_be_relabelled_without_execution_evidence() {
        let mut run = SimulationRun::new(10);
        run.lifecycle = SimulationRunLifecycle::LegacyUnknown;

        assert!(run.mark_running().is_err());
        assert!(
            run.finish_lifecycle(SimulationRunLifecycle::Completed)
                .is_err()
        );
        assert_eq!(run.lifecycle, SimulationRunLifecycle::LegacyUnknown);
    }

    #[test]
    fn same_kind_results_remain_attributable_to_exact_source_instances() {
        let first_id = AnalysisInstanceId::new();
        let second_id = AnalysisInstanceId::new();
        let snapshot = ContentDigest::from_bytes([0x5a; 32]);
        let mut run = SimulationRun::new(8);

        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC low band").with_provenance(
                AnalysisResultProvenance::new(
                    first_id,
                    ObjectRevision::INITIAL,
                    snapshot,
                    Vec::new(),
                )
                .expect("first provenance is valid"),
            ),
        );
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "AC high band").with_provenance(
                AnalysisResultProvenance::new(
                    second_id,
                    ObjectRevision::INITIAL,
                    snapshot,
                    vec![first_id],
                )
                .expect("second provenance is valid"),
            ),
        );

        assert_eq!(
            run.find_analysis(AnalysisType::Ac).unwrap().label,
            "AC low band"
        );
        assert_eq!(
            run.find_analysis_by_source_instance(first_id)
                .unwrap()
                .label,
            "AC low band"
        );
        assert_eq!(
            run.find_analysis_by_source_instance(second_id)
                .unwrap()
                .label,
            "AC high band"
        );
        assert_eq!(
            run.find_analysis_by_source_instance(second_id)
                .unwrap()
                .provenance
                .as_ref()
                .unwrap()
                .dependency_ids(),
            &[first_id]
        );
    }

    #[test]
    fn authored_source_selection_returns_the_final_expanded_point() {
        let authored = AnalysisInstanceId::new();
        let first_point = AnalysisInstanceId::new();
        let final_point = AnalysisInstanceId::new();
        let snapshot = ContentDigest::from_bytes([0x5b; 32]);
        let mut run = SimulationRun::new(9);
        for (label, execution_id) in [("OP point 1/2", first_point), ("OP point 2/2", final_point)]
        {
            run.add_analysis(
                AnalysisResult::new(1, AnalysisType::DcOp, label).with_provenance(
                    AnalysisResultProvenance::new_with_authored_source_domain(
                        AnalysisResultSourceDomain::SimulationPlan,
                        execution_id,
                        authored,
                        ObjectRevision::INITIAL,
                        snapshot,
                        Vec::new(),
                    )
                    .expect("expanded provenance is valid"),
                ),
            );
        }

        let selected = run
            .find_analysis_by_source_instance(authored)
            .expect("authored OP selection");
        assert_eq!(selected.label, "OP point 2/2");
        assert_eq!(
            selected.provenance.as_ref().unwrap().source_instance_id(),
            final_point
        );
    }
}
