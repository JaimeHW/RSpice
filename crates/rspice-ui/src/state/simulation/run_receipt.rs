use std::collections::HashSet;

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};

use super::{AnalysisResult, AnalysisResultSourceDomain, AnalysisType};

/// Typed source-check authority captured by immutable run preparation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreparedSourceCheckReceipt {
    SchematicDrc(ContentDigest),
    ManualSourceCheck(ContentDigest),
}

impl PreparedSourceCheckReceipt {
    #[must_use]
    pub const fn digest(self) -> ContentDigest {
        match self {
            Self::SchematicDrc(digest) | Self::ManualSourceCheck(digest) => digest,
        }
    }

    #[must_use]
    pub const fn is_schematic_drc(self) -> bool {
        matches!(self, Self::SchematicDrc(_))
    }

    #[must_use]
    pub const fn is_manual_source_check(self) -> bool {
        matches!(self, Self::ManualSourceCheck(_))
    }
}

/// Authenticated identity and graph position of one ordered prepared task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedRunTaskReceipt {
    instance_id: AnalysisInstanceId,
    source_revision: ObjectRevision,
    dependencies: Vec<AnalysisInstanceId>,
    analysis_kind_tag: u8,
    config_digest: ContentDigest,
}

impl PreparedRunTaskReceipt {
    pub(crate) fn new(
        instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        analysis_kind_tag: u8,
        config_digest: ContentDigest,
    ) -> Result<Self, String> {
        // Canonical analysis tags are a closed execution protocol. Reject an
        // unknown persisted tag instead of allowing it to masquerade as a
        // task produced by this binary.
        if analysis_kind_tag > 25 {
            return Err(format!(
                "prepared task {instance_id} has unknown analysis kind tag {analysis_kind_tag}"
            ));
        }

        let mut unique_dependencies = HashSet::with_capacity(dependencies.len());
        for dependency in &dependencies {
            if *dependency == instance_id {
                return Err(format!(
                    "prepared task {instance_id} cannot depend on itself"
                ));
            }
            if !unique_dependencies.insert(*dependency) {
                return Err(format!(
                    "prepared task {instance_id} repeats dependency {dependency}"
                ));
            }
        }

        Ok(Self {
            instance_id,
            source_revision,
            dependencies,
            analysis_kind_tag,
            config_digest,
        })
    }

    #[must_use]
    pub const fn instance_id(&self) -> AnalysisInstanceId {
        self.instance_id
    }

    #[must_use]
    pub const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    #[must_use]
    pub fn dependencies(&self) -> &[AnalysisInstanceId] {
        &self.dependencies
    }

    #[must_use]
    pub const fn analysis_kind_tag(&self) -> u8 {
        self.analysis_kind_tag
    }

    #[must_use]
    pub const fn config_digest(&self) -> ContentDigest {
        self.config_digest
    }

    /// Result-family identity encoded by the canonical execution tag.
    #[must_use]
    pub const fn result_analysis_type(&self) -> AnalysisType {
        match self.analysis_kind_tag {
            0 => AnalysisType::DcOp,
            1 => AnalysisType::DcSweep,
            2 | 3 => AnalysisType::Ac,
            4 => AnalysisType::Disto,
            5 => AnalysisType::Transient,
            6 => AnalysisType::Noise,
            7 => AnalysisType::Pss,
            8 => AnalysisType::HarmonicBalance,
            9 => AnalysisType::Tf,
            10 => AnalysisType::Sensitivity,
            11 => AnalysisType::PoleZero,
            12 => AnalysisType::Pac,
            13 => AnalysisType::Pnoise,
            14 => AnalysisType::Pxf,
            15 => AnalysisType::Pstb,
            16 => AnalysisType::Stb,
            17 => AnalysisType::MonteCarlo,
            18 => AnalysisType::Parametric,
            19 => AnalysisType::Corner,
            20 => AnalysisType::Reliability,
            21 => AnalysisType::Optimization,
            22 => AnalysisType::Soa,
            23 => AnalysisType::SParameter,
            24 => AnalysisType::Envelope,
            25 => AnalysisType::Fourier,
            _ => unreachable!(),
        }
    }
}

/// Durable authority for a run created from one consumed prepared snapshot.
///
/// The complete task graph remains present even when execution is aborted or
/// fails partway through the queue. Result history is consequently an ordered
/// prefix of `tasks`, never the source from which run classification is
/// inferred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedRunReceipt {
    source_domain: AnalysisResultSourceDomain,
    simulation_plan_id: Option<SimulationPlanId>,
    project_revision: ObjectRevision,
    prepared_snapshot_digest: ContentDigest,
    source_content_digest: ContentDigest,
    source_check_receipt: PreparedSourceCheckReceipt,
    tasks: Vec<PreparedRunTaskReceipt>,
}

impl PreparedRunReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        source_domain: AnalysisResultSourceDomain,
        simulation_plan_id: Option<SimulationPlanId>,
        project_revision: ObjectRevision,
        prepared_snapshot_digest: ContentDigest,
        source_content_digest: ContentDigest,
        source_check_receipt: PreparedSourceCheckReceipt,
        tasks: Vec<PreparedRunTaskReceipt>,
    ) -> Result<Self, String> {
        match (source_domain, simulation_plan_id, source_check_receipt) {
            (
                AnalysisResultSourceDomain::SimulationPlan,
                Some(_),
                PreparedSourceCheckReceipt::SchematicDrc(_),
            )
            | (
                AnalysisResultSourceDomain::ManualDeck,
                None,
                PreparedSourceCheckReceipt::ManualSourceCheck(_),
            ) => {}
            (AnalysisResultSourceDomain::LegacyUnclassified, _, _) => {
                return Err(
                    "current prepared-run receipt cannot use a legacy-unclassified source domain"
                        .to_owned(),
                );
            }
            _ => {
                return Err(
                    "prepared-run source domain, simulation plan, and source-check receipt disagree"
                        .to_owned(),
                );
            }
        }
        if tasks.is_empty() {
            return Err("prepared-run receipt must contain at least one task".to_owned());
        }
        let source_revision = tasks[0].source_revision;
        if tasks
            .iter()
            .any(|task| task.source_revision != source_revision)
        {
            return Err(
                "prepared-run receipt cannot mix task source revisions in one frozen run"
                    .to_owned(),
            );
        }

        let mut prior_tasks = HashSet::with_capacity(tasks.len());
        for task in &tasks {
            if prior_tasks.contains(&task.instance_id) {
                return Err(format!(
                    "prepared-run receipt repeats task instance {}",
                    task.instance_id
                ));
            }
            for dependency in &task.dependencies {
                if !prior_tasks.contains(dependency) {
                    return Err(format!(
                        "prepared task {} dependency {} must appear earlier in receipt order",
                        task.instance_id, dependency
                    ));
                }
            }
            prior_tasks.insert(task.instance_id);
        }

        Ok(Self {
            source_domain,
            simulation_plan_id,
            project_revision,
            prepared_snapshot_digest,
            source_content_digest,
            source_check_receipt,
            tasks,
        })
    }

    #[must_use]
    pub const fn source_domain(&self) -> AnalysisResultSourceDomain {
        self.source_domain
    }

    #[must_use]
    pub const fn simulation_plan_id(&self) -> Option<SimulationPlanId> {
        self.simulation_plan_id
    }

    #[must_use]
    pub const fn project_revision(&self) -> ObjectRevision {
        self.project_revision
    }

    #[must_use]
    pub const fn prepared_snapshot_digest(&self) -> ContentDigest {
        self.prepared_snapshot_digest
    }

    #[must_use]
    pub const fn source_content_digest(&self) -> ContentDigest {
        self.source_content_digest
    }

    #[must_use]
    pub const fn source_check_receipt(&self) -> PreparedSourceCheckReceipt {
        self.source_check_receipt
    }

    #[must_use]
    pub fn tasks(&self) -> &[PreparedRunTaskReceipt] {
        &self.tasks
    }

    pub(crate) fn validate_result_prefix(&self, analyses: &[AnalysisResult]) -> Result<(), String> {
        if analyses.len() > self.tasks.len() {
            return Err(format!(
                "prepared run contains {} results for only {} authenticated tasks",
                analyses.len(),
                self.tasks.len()
            ));
        }

        for (index, (analysis, task)) in analyses.iter().zip(&self.tasks).enumerate() {
            let provenance = analysis.provenance.as_ref().ok_or_else(|| {
                format!(
                    "prepared run result {index} for task {} has no analysis provenance",
                    task.instance_id
                )
            })?;
            if provenance.source_domain() != self.source_domain
                || provenance.source_instance_id() != task.instance_id
                || provenance.source_revision() != task.source_revision
                || provenance.prepared_snapshot_digest() != self.prepared_snapshot_digest
                || provenance.dependency_ids() != task.dependencies
                || analysis.analysis_type != task.result_analysis_type()
            {
                return Err(format!(
                    "prepared run result {index} does not match authenticated task {}",
                    task.instance_id
                ));
            }
        }
        Ok(())
    }
}

/// Authoritative provenance classification retained by a simulation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationRunProvenance {
    /// Result history predates prepared analysis-instance identity entirely.
    LegacyUnattributed,
    /// Historical results carry prepared IDs but predate source-domain and
    /// complete run-receipt persistence.
    LegacyPreparedUnclassified,
    /// Current run authenticated by an exact consumed prepared snapshot.
    Prepared(PreparedRunReceipt),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResultProvenance, SimulationRun};

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn task(
        id: AnalysisInstanceId,
        revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        tag: u8,
        config_byte: u8,
    ) -> PreparedRunTaskReceipt {
        PreparedRunTaskReceipt::new(id, revision, dependencies, tag, digest(config_byte))
            .expect("valid task receipt")
    }

    fn plan_receipt(tasks: Vec<PreparedRunTaskReceipt>) -> PreparedRunReceipt {
        PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            digest(0x31),
            digest(0x32),
            PreparedSourceCheckReceipt::SchematicDrc(digest(0x33)),
            tasks,
        )
        .expect("valid plan receipt")
    }

    fn result(
        id: AnalysisInstanceId,
        revision: ObjectRevision,
        snapshot: ContentDigest,
        dependencies: Vec<AnalysisInstanceId>,
        analysis_type: AnalysisType,
    ) -> AnalysisResult {
        AnalysisResult::new(1, analysis_type, "result").with_provenance(
            AnalysisResultProvenance::new_with_source_domain(
                AnalysisResultSourceDomain::SimulationPlan,
                id,
                revision,
                snapshot,
                dependencies,
            )
            .expect("valid result provenance"),
        )
    }

    #[test]
    fn fresh_run_is_unsealed_and_cannot_validate_as_legacy() {
        let run = SimulationRun::new(1);

        assert!(run.provenance().is_none());
        assert!(run.validate_provenance().unwrap_err().contains("unsealed"));
    }

    #[test]
    fn migration_must_restore_an_explicit_legacy_classification() {
        let mut unattributed = SimulationRun::new(2);
        unattributed.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "legacy"));
        unattributed
            .restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("unattributed legacy history restores explicitly");
        assert!(matches!(
            unattributed.provenance(),
            Some(SimulationRunProvenance::LegacyUnattributed)
        ));

        let mut unclassified = SimulationRun::new(3);
        unclassified.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "legacy prepared").with_provenance(
                AnalysisResultProvenance::new_with_source_domain(
                    AnalysisResultSourceDomain::LegacyUnclassified,
                    AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    digest(0x21),
                    Vec::new(),
                )
                .expect("legacy prepared provenance"),
            ),
        );
        unclassified
            .restore_provenance(SimulationRunProvenance::LegacyPreparedUnclassified)
            .expect("unclassified prepared history restores explicitly");
        assert!(matches!(
            unclassified.provenance(),
            Some(SimulationRunProvenance::LegacyPreparedUnclassified)
        ));
    }

    #[test]
    fn receipt_rejects_mixed_task_revisions() {
        let first = AnalysisInstanceId::new();
        let second = AnalysisInstanceId::new();
        let tasks = vec![
            task(first, ObjectRevision::INITIAL, Vec::new(), 0, 1),
            task(
                second,
                ObjectRevision::new(2).expect("revision two"),
                Vec::new(),
                5,
                2,
            ),
        ];

        let error = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(SimulationPlanId::new()),
            ObjectRevision::INITIAL,
            digest(3),
            digest(4),
            PreparedSourceCheckReceipt::SchematicDrc(digest(5)),
            tasks,
        )
        .expect_err("mixed task revisions must fail");

        assert!(error.contains("mix task source revisions"));
    }

    #[test]
    fn receipt_rejects_source_domain_plan_and_check_mismatches() {
        let make_tasks = || {
            vec![task(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                Vec::new(),
                0,
                9,
            )]
        };
        let plan_id = SimulationPlanId::new();
        let build = |domain, plan, check, tasks| {
            PreparedRunReceipt::new(
                domain,
                plan,
                ObjectRevision::INITIAL,
                digest(6),
                digest(7),
                check,
                tasks,
            )
        };

        assert!(
            build(
                AnalysisResultSourceDomain::SimulationPlan,
                None,
                PreparedSourceCheckReceipt::SchematicDrc(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
        assert!(
            build(
                AnalysisResultSourceDomain::SimulationPlan,
                Some(plan_id),
                PreparedSourceCheckReceipt::ManualSourceCheck(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
        assert!(
            build(
                AnalysisResultSourceDomain::ManualDeck,
                Some(plan_id),
                PreparedSourceCheckReceipt::ManualSourceCheck(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
        assert!(
            build(
                AnalysisResultSourceDomain::ManualDeck,
                None,
                PreparedSourceCheckReceipt::SchematicDrc(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
        assert!(
            build(
                AnalysisResultSourceDomain::LegacyUnclassified,
                None,
                PreparedSourceCheckReceipt::ManualSourceCheck(digest(8)),
                make_tasks(),
            )
            .is_err()
        );
    }

    #[test]
    fn prepared_result_history_accepts_exact_abort_prefix_and_rejects_identity_rewrites() {
        let first = AnalysisInstanceId::new();
        let second = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let receipt = plan_receipt(vec![
            task(first, revision, Vec::new(), 2, 0x41),
            task(second, revision, vec![first], 5, 0x42),
        ]);
        let snapshot = receipt.prepared_snapshot_digest();
        let first_result = result(first, revision, snapshot, Vec::new(), AnalysisType::Ac);
        let second_result = result(
            second,
            revision,
            snapshot,
            vec![first],
            AnalysisType::Transient,
        );

        receipt
            .validate_result_prefix(std::slice::from_ref(&first_result))
            .expect("authenticated abort prefix is valid");
        receipt
            .validate_result_prefix(&[first_result.clone(), second_result.clone()])
            .expect("complete authenticated result sequence is valid");

        assert!(
            receipt
                .validate_result_prefix(&[second_result.clone(), first_result.clone()])
                .is_err(),
            "result reorder must fail"
        );

        let mut changed_type = first_result.clone();
        changed_type.analysis_type = AnalysisType::Transient;
        assert!(receipt.validate_result_prefix(&[changed_type]).is_err());

        let changed_domain = AnalysisResult::new(1, AnalysisType::Ac, "result").with_provenance(
            AnalysisResultProvenance::new_with_source_domain(
                AnalysisResultSourceDomain::ManualDeck,
                first,
                revision,
                snapshot,
                Vec::new(),
            )
            .expect("changed domain provenance"),
        );
        assert!(receipt.validate_result_prefix(&[changed_domain]).is_err());

        let changed_id = result(
            AnalysisInstanceId::new(),
            revision,
            snapshot,
            Vec::new(),
            AnalysisType::Ac,
        );
        assert!(receipt.validate_result_prefix(&[changed_id]).is_err());

        let changed_revision = result(
            first,
            ObjectRevision::new(2).expect("revision two"),
            snapshot,
            Vec::new(),
            AnalysisType::Ac,
        );
        assert!(receipt.validate_result_prefix(&[changed_revision]).is_err());

        let changed_snapshot = result(first, revision, digest(0x99), Vec::new(), AnalysisType::Ac);
        assert!(receipt.validate_result_prefix(&[changed_snapshot]).is_err());

        let changed_dependencies = result(
            second,
            revision,
            snapshot,
            Vec::new(),
            AnalysisType::Transient,
        );
        assert!(
            receipt
                .validate_result_prefix(&[first_result, changed_dependencies])
                .is_err()
        );
    }
}
