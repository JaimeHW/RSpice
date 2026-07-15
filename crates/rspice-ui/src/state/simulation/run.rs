use super::*;
use crate::product::{AnalysisInstanceId, DatasetId, RunId};

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
pub struct SimulationRun {
    /// Stable product identity. This survives project moves, history reorder,
    /// and save/reload; it is the authoritative reference for selections and
    /// dataset provenance.
    pub run_id: RunId,
    /// Stable identity of the result dataset assembled by this run. The
    /// dataset contains the run's analyses and becomes the immutable unit
    /// addressed by result documents, comparisons, and overlays after
    /// completion.
    pub dataset_id: DatasetId,
    /// Human-facing run sequence (monotonically increasing within a project).
    /// It is a label/order value, not persistent object identity.
    pub id: u64,
    /// Human-readable label with timestamp (e.g., "Run 3 (1:04:01 PM)")
    pub label: String,
    /// Unix timestamp when run started
    pub timestamp: f64,
    /// All analysis results from this run
    pub analyses: Vec<AnalysisResult>,
    /// Durable run-level authority. `None` is a transient, unsealed run and
    /// must never be persisted as a legacy classification.
    provenance: Option<SimulationRunProvenance>,
    /// Total simulation time in seconds
    pub elapsed_time: f64,
    /// Whether all analyses in this run succeeded
    pub success: bool,
}

impl SimulationRun {
    /// Create a new simulation run with auto-generated ID and timestamp
    pub fn new(run_number: u64) -> Self {
        let timestamp = Self::current_timestamp();
        let time_str = Self::format_time(timestamp);
        Self {
            run_id: RunId::new(),
            dataset_id: DatasetId::new(),
            id: run_number,
            label: format!("Run {} ({})", run_number, time_str),
            timestamp,
            analyses: Vec::new(),
            provenance: None,
            elapsed_time: 0.0,
            success: true,
        }
    }

    pub(crate) fn new_prepared(run_number: u64, receipt: PreparedRunReceipt) -> Self {
        let mut run = Self::new(run_number);
        run.provenance = Some(SimulationRunProvenance::Prepared(receipt));
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

    /// Restore an explicit persisted classification without inferring it from
    /// the current result vector. Reserved for versioned project migration.
    pub(crate) fn restore_provenance(
        &mut self,
        provenance: SimulationRunProvenance,
    ) -> Result<(), String> {
        if self.provenance.is_some() {
            return Err(format!(
                "simulation run {} already has authoritative provenance",
                self.id
            ));
        }
        Self::validate_provenance_against_results(&provenance, &self.analyses)?;
        self.provenance = Some(provenance);
        Ok(())
    }

    /// Validate that retained results are an exact ordered prefix of the
    /// authoritative receipt. Aborted and partial runs may omit the remaining
    /// suffix; they may not rewrite or reorder completed task identity.
    pub fn validate_provenance(&self) -> Result<(), String> {
        let provenance = self.provenance.as_ref().ok_or_else(|| {
            format!(
                "simulation run {} is unsealed and has no authoritative provenance",
                self.id
            )
        })?;
        Self::validate_provenance_against_results(provenance, &self.analyses)
    }

    fn validate_provenance_against_results(
        provenance: &SimulationRunProvenance,
        analyses: &[AnalysisResult],
    ) -> Result<(), String> {
        match provenance {
            SimulationRunProvenance::LegacyUnattributed => {
                if analyses
                    .iter()
                    .any(|analysis| analysis.provenance.is_some())
                {
                    return Err(
                        "legacy-unattributed run contains prepared analysis provenance".to_owned(),
                    );
                }
            }
            SimulationRunProvenance::LegacyPreparedUnclassified => {
                if analyses.iter().any(|analysis| {
                    analysis.provenance.as_ref().is_none_or(|provenance| {
                        provenance.source_domain() != AnalysisResultSourceDomain::LegacyUnclassified
                    })
                }) {
                    return Err(
                        "legacy prepared-unclassified run has missing or classified analysis provenance"
                            .to_owned(),
                    );
                }
            }
            SimulationRunProvenance::Prepared(receipt) => {
                receipt.validate_result_prefix(analyses)?;
            }
        }
        Ok(())
    }

    /// Add an analysis result to this run
    pub fn add_analysis(&mut self, mut analysis: AnalysisResult) {
        if analysis.id == 0
            || self
                .analyses
                .iter()
                .any(|existing| existing.id == analysis.id)
        {
            analysis.id = self.next_available_analysis_id();
        }
        if !analysis.success {
            self.success = false;
        }
        self.analyses.push(analysis);
    }

    fn next_available_analysis_id(&self) -> u64 {
        let mut next_id = self
            .analyses
            .iter()
            .map(|analysis| analysis.id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .expect("analysis id space exhausted");
        while self.analyses.iter().any(|analysis| analysis.id == next_id) {
            next_id = next_id.checked_add(1).expect("analysis id space exhausted");
        }
        next_id
    }

    /// Set the total elapsed time for this run
    pub fn set_elapsed_time(&mut self, elapsed: f64) {
        self.elapsed_time = elapsed;
    }

    /// Get count of successful analyses
    pub fn successful_analyses(&self) -> usize {
        self.analyses.iter().filter(|a| a.success).count()
    }

    /// Find analysis by type (returns first match)
    pub fn find_analysis(&self, analysis_type: AnalysisType) -> Option<&AnalysisResult> {
        self.analyses
            .iter()
            .find(|a| a.analysis_type == analysis_type)
    }

    /// Find the result produced by one exact prepared analysis instance.
    /// Unlike [`Self::find_analysis`], this remains unambiguous when a run has
    /// multiple analyses of the same kind.
    pub fn find_analysis_by_source_instance(
        &self,
        source_instance_id: AnalysisInstanceId,
    ) -> Option<&AnalysisResult> {
        self.analyses.iter().find(|analysis| {
            analysis
                .provenance
                .as_ref()
                .is_some_and(|provenance| provenance.source_instance_id() == source_instance_id)
        })
    }

    /// Get current timestamp as Unix epoch seconds
    fn current_timestamp() -> f64 {
        crate::common::time_compat::unix_epoch().as_secs_f64()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ContentDigest, ObjectRevision};

    #[test]
    fn add_analysis_assigns_unique_ids_when_converters_reuse_placeholder() {
        let mut run = SimulationRun::new(7);
        let stable_id = run.run_id;
        let dataset_id = run.dataset_id;

        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC"));

        let ids: Vec<_> = run.analyses.iter().map(|analysis| analysis.id).collect();
        assert_eq!(ids, vec![1, 2]);
        assert_eq!(run.run_id, stable_id);
        assert_eq!(run.dataset_id, dataset_id);
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
}
