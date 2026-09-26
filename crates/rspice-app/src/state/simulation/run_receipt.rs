//! Integration tests for retained run receipts and application history.

use super::*;
use crate::product::{
    AnalysisInstanceId, ContentDigest, ModelSourceId, ObjectRevision, SimulationPlanId,
};
use crate::state::CellViewRef;

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
        PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: digest(0x31),
            source_content_digest: digest(0x32),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x33)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks,
        })
        .expect("valid plan receipt")
    }

    #[test]
    fn prepared_receipt_retains_typed_project_model_identity() {
        let source_id = ModelSourceId::new();
        let identity = PreparedModelSourceIdentity::new(
            source_id,
            "precision_nmos",
            ObjectRevision::INITIAL,
            digest(0x44),
            PreparedModelQualification::Released,
        )
        .expect("valid project model identity");
        let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: digest(0x31),
            source_content_digest: digest(0x32),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x33)),
            project_model_sources: vec![identity],
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![task(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                Vec::new(),
                0,
                0x45,
            )],
        })
        .expect("model-bound prepared receipt");

        let retained = &receipt.project_model_sources()[0];
        assert_eq!(retained.source_id(), source_id);
        assert_eq!(retained.model_name(), "precision_nmos");
        assert_eq!(retained.revision(), ObjectRevision::INITIAL);
        assert_eq!(retained.content_digest(), digest(0x44));
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

    fn amp_reference() -> CellViewRef {
        CellViewRef::new("user", "amp", "schematic")
    }

    #[test]
    fn a_hierarchy_map_row_must_agree_with_the_engine_scope_it_claims() {
        let row = HierarchyMapRow::new("/X1/X2", "amp_1", "X1.X2", amp_reference())
            .expect("a derived engine prefix is accepted");
        assert_eq!(row.occurrence(), "/X1/X2");
        assert_eq!(row.master(), "amp_1");
        assert_eq!(row.engine_prefix(), "X1.X2");
        assert_eq!(row.master_reference(), &amp_reference());

        assert!(
            HierarchyMapRow::new("/X1/X2", "amp_1", "X1.X9", amp_reference())
                .unwrap_err()
                .contains("rather than 'X1.X2'")
        );
        assert!(
            HierarchyMapRow::new("/X1", "", "X1", amp_reference())
                .unwrap_err()
                .contains("no master name")
        );
        assert!(
            HierarchyMapRow::new("/", "amp_1", "", amp_reference())
                .unwrap_err()
                .contains("design root")
        );
        assert!(
            HierarchyMapRow::new("/X1/", "amp_1", "", amp_reference())
                .unwrap_err()
                .contains("not an instance path")
        );
    }

    #[test]
    fn a_sealed_hierarchy_map_names_each_occurrence_once() {
        let receipt = plan_receipt(vec![task(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            0,
            0x51,
        )]);
        assert!(
            receipt.hierarchy_map().is_empty(),
            "a receipt seals no map until one is carried onto it"
        );

        let sealed = receipt
            .clone()
            .with_hierarchy_map(vec![
                HierarchyMapRow::new("/X1", "amp_1", "X1", amp_reference())
                    .expect("outer occurrence"),
                HierarchyMapRow::new("/X1/X2", "amp_2", "X1.X2", amp_reference())
                    .expect("inner occurrence"),
            ])
            .expect("distinct occurrences seal");
        assert_eq!(
            sealed
                .hierarchy_map()
                .iter()
                .map(HierarchyMapRow::engine_prefix)
                .collect::<Vec<_>>(),
            vec!["X1", "X1.X2"]
        );

        // Two rows for one instance would make the reverse map ambiguous, and
        // the case fold is what decides that they are one instance.
        let error = receipt
            .with_hierarchy_map(vec![
                HierarchyMapRow::new("/X1", "amp_1", "X1", amp_reference()).expect("first row"),
                HierarchyMapRow::new("/x1", "amp_2", "X1", amp_reference()).expect("second row"),
            ])
            .expect_err("one occurrence cannot name two masters");
        assert!(
            error.contains("repeats hierarchy map occurrence"),
            "{error}"
        );
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

        let error = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: digest(3),
            source_content_digest: digest(4),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(5)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks,
        })
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
            PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
                source_domain: domain,
                simulation_plan_id: plan,
                project_revision: ObjectRevision::INITIAL,
                prepared_snapshot_digest: digest(6),
                source_content_digest: digest(7),
                source_check_receipt: check,
                project_model_sources: Vec::new(),
                specifications: Vec::new(),
                specification_policy:
                    rspice_results::specification::PreparedSpecificationPolicy::default(),
                tasks,
            })
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
            .validate_result_prefix(
                (std::slice::from_ref(&first_result))
                    .iter()
                    .map(|analysis| &analysis.data),
            )
            .expect("authenticated abort prefix is valid");
        receipt
            .validate_result_prefix(
                ([first_result.clone(), second_result.clone()])
                    .iter()
                    .map(|analysis| &analysis.data),
            )
            .expect("complete authenticated result sequence is valid");

        assert!(
            receipt
                .validate_result_prefix(
                    ([second_result.clone(), first_result.clone()])
                        .iter()
                        .map(|analysis| &analysis.data)
                )
                .is_err(),
            "result reorder must fail"
        );

        let mut changed_type = first_result.clone();
        changed_type.analysis_type = AnalysisType::Transient;
        assert!(
            receipt
                .validate_result_prefix(([changed_type]).iter().map(|analysis| &analysis.data))
                .is_err()
        );

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
        assert!(
            receipt
                .validate_result_prefix(([changed_domain]).iter().map(|analysis| &analysis.data))
                .is_err()
        );

        let changed_id = result(
            AnalysisInstanceId::new(),
            revision,
            snapshot,
            Vec::new(),
            AnalysisType::Ac,
        );
        assert!(
            receipt
                .validate_result_prefix(([changed_id]).iter().map(|analysis| &analysis.data))
                .is_err()
        );

        let changed_revision = result(
            first,
            ObjectRevision::new(2).expect("revision two"),
            snapshot,
            Vec::new(),
            AnalysisType::Ac,
        );
        assert!(
            receipt
                .validate_result_prefix(([changed_revision]).iter().map(|analysis| &analysis.data))
                .is_err()
        );

        let changed_snapshot = result(first, revision, digest(0x99), Vec::new(), AnalysisType::Ac);
        assert!(
            receipt
                .validate_result_prefix(([changed_snapshot]).iter().map(|analysis| &analysis.data))
                .is_err()
        );

        let changed_dependencies = result(
            second,
            revision,
            snapshot,
            Vec::new(),
            AnalysisType::Transient,
        );
        assert!(
            receipt
                .validate_result_prefix(
                    ([first_result, changed_dependencies])
                        .iter()
                        .map(|analysis| &analysis.data)
                )
                .is_err()
        );
    }
}
