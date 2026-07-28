//! Tests for suite lifecycle and evidence safety.
//!
//! Every case is about not accepting evidence that is not there: a stale,
//! tampered, or incomplete platform run rolls back, a failed disposition
//! blocks promotion until an exact rerun passes, and retirement is explicit
//! rather than implied by a newer result.

use super::*;

fn digest(byte: u8) -> ContentDigest {
    ContentDigest::from_bytes([byte; 32])
}

fn model_source(byte: u8) -> Vec<u8> {
    format!("* retained qualification model\n.model demo180_nch NMOS (VTO=0.{byte})\n").into_bytes()
}

fn source(byte: u8) -> ModelSourceEvidenceBinding {
    ModelSourceEvidenceBinding::try_new_project_bound(
        "demo180_nch",
        "11111111-1111-4111-8111-111111111111".parse().unwrap(),
        digest_bytes(&model_source(byte)),
        ObjectRevision::new(7).unwrap(),
    )
    .unwrap()
}

fn vector(id: &str, name: &str, byte: u8) -> QualificationVector {
    vector_for_source(id, name, byte, 7)
}

fn vector_for_source(
    id: &str,
    name: &str,
    input_variant: u8,
    source_variant: u8,
) -> QualificationVector {
    let retained_model = model_source(source_variant);
    let input = format!(
            "qualification {input_variant}\n{}V1 output 0 DC 1\nM1 output output 0 0 demo180_nch\n.end\n",
            String::from_utf8(retained_model.clone()).unwrap()
        )
        .into_bytes();
    QualificationVector::try_new_source_bound(
        id,
        name,
        source(source_variant),
        retained_model,
        input,
        QualificationAnalysis::DcOperatingPoint,
        vec![
            QualificationOutputDefinition::try_new(
                "drain_current",
                QualificationProbe::NodeVoltage {
                    node: "output".into(),
                },
                QualificationSample::OperatingPoint,
            )
            .unwrap(),
        ],
        vec![QualificationReference::try_new("drain_current", 1.0, 0.01, 0.005).unwrap()],
    )
    .unwrap()
}

fn suite() -> QualificationSuite {
    QualificationSuite::try_new(
        "dc-iv",
        "DC IV",
        ObjectRevision::new(3).unwrap(),
        vec![
            vector("dc-002", "Output sweep", 2),
            vector("dc-001", "Transfer sweep", 1),
        ],
    )
    .unwrap()
}

fn platform_outcome(platform: QualificationPlatform, passes: bool) -> PlatformQualificationOutcome {
    let error = if passes { 0.001 } else { 0.02 };
    PlatformQualificationOutcome::try_new(
        platform,
        vec![
            ReferenceErrorEvidence::try_new("drain_current", 1.0, 1.0 + error, 0.01, 0.005)
                .unwrap(),
        ],
    )
    .unwrap()
}

fn executable_vector(
    id: &str,
    name: &str,
    testbench: &str,
    analysis: QualificationAnalysis,
    outputs: Vec<QualificationOutputDefinition>,
    references: Vec<QualificationReference>,
) -> QualificationVector {
    let retained_model = model_source(7);
    let input = format!(
        "qualification {id}\n{}{testbench}",
        String::from_utf8(retained_model.clone()).unwrap()
    )
    .into_bytes();
    QualificationVector::try_new_source_bound(
        id,
        name,
        source(7),
        retained_model,
        input,
        analysis,
        outputs,
        references,
    )
    .unwrap()
}

fn evidence(
    suite: &QualificationSuite,
    source: &ModelSourceEvidenceBinding,
    passes: bool,
) -> QualificationEvidence {
    let outcomes = suite
        .vectors
        .iter()
        .map(|vector| {
            QualificationVectorOutcome::try_new(
                vector.id.clone(),
                vector.input_digest,
                vec![
                    platform_outcome(QualificationPlatform::WebAssembly, passes),
                    platform_outcome(QualificationPlatform::Desktop, true),
                ],
            )
            .unwrap()
        })
        .collect();
    QualificationEvidence::try_new(
        "evidence-7",
        source.clone(),
        suite.id.clone(),
        suite.revision,
        outcomes,
    )
    .unwrap()
}

fn imported_platform_run(
    run: &QualificationPlatformRun,
    platform: QualificationPlatform,
    suite: &QualificationSuite,
    source: &ModelSourceEvidenceBinding,
) -> QualificationPlatformRun {
    let outcomes = run
        .vector_outcomes
        .iter()
        .map(|value| {
            let mut value = value.clone();
            value.outcome.platform = platform;
            value
        })
        .collect();
    QualificationPlatformRun::try_new(platform, source.clone(), suite, outcomes).unwrap()
}

fn failed_platform_run(
    suite: &QualificationSuite,
    source: &ModelSourceEvidenceBinding,
    platform: QualificationPlatform,
) -> QualificationPlatformRun {
    let outcomes = suite
        .vectors
        .iter()
        .map(|vector| QualificationPlatformVectorOutcome {
            vector_id: vector.id.clone(),
            input_digest: vector.input_digest,
            outcome: PlatformQualificationOutcome::try_failed(
                platform,
                QualificationExecutionFailure::try_new(
                    QualificationFailureStage::Measurement,
                    "reference-mismatch",
                    "retained test failure",
                )
                .unwrap(),
            )
            .unwrap(),
        })
        .collect();
    QualificationPlatformRun::try_new(platform, source.clone(), suite, outcomes).unwrap()
}

fn document(id: &str, byte: u8) -> DocumentReference {
    DocumentReference::try_new(id, digest(byte)).unwrap()
}

fn documents() -> DocumentationSet {
    DocumentationSet::try_new(vec![
        DocumentationDeclaration {
            kind: RequiredDocumentation::QualificationReport,
            document: document("qualification-report", 13),
        },
        DocumentationDeclaration {
            kind: RequiredDocumentation::ModelDescription,
            document: document("model-description", 11),
        },
        DocumentationDeclaration {
            kind: RequiredDocumentation::ParameterReference,
            document: document("parameter-reference", 12),
        },
    ])
    .unwrap()
}

fn license() -> LicenseDeclaration {
    LicenseDeclaration {
        license_id: "foundry-project-42".into(),
        expression: "LicenseRef-Demo180-Project".into(),
        scope: LicenseScope::FoundryProject,
        commercial_use_allowed: true,
        redistribution_allowed: false,
        reviewed: true,
        notice: document("license-notice", 20),
    }
}

fn impact() -> ConsumerImpactAssessment {
    ConsumerImpactAssessment::try_new(
        ConsumerChange::Compatible,
        "Parameter-only correction within the declared model domain.",
        vec!["plan-4".into(), "cell-18".into()],
        None,
        true,
    )
    .unwrap()
}

fn compatibility() -> CompatibilityAssessment {
    CompatibilityAssessment::try_new(
        vec![
            PlatformCompatibilityEvidence {
                platform: QualificationPlatform::WebAssembly,
                disposition: CompatibilityDisposition::Compatible,
                evidence: document("wasm-compat", 22),
            },
            PlatformCompatibilityEvidence {
                platform: QualificationPlatform::Desktop,
                disposition: CompatibilityDisposition::Compatible,
                evidence: document("desktop-compat", 21),
            },
        ],
        CompatibilityDisposition::Compatible,
        true,
    )
    .unwrap()
}

fn approvals() -> Vec<PromotionApproval> {
    vec![
        PromotionApproval {
            role: PromotionApprovalRole::QualificationApprover,
            approver_id: "m.chen".into(),
            decision: ApprovalDecision::Approved,
            decision_revision: ObjectRevision::new(9).unwrap(),
        },
        PromotionApproval {
            role: PromotionApprovalRole::ModelOwner,
            approver_id: "a.singh".into(),
            decision: ApprovalDecision::Approved,
            decision_revision: ObjectRevision::new(8).unwrap(),
        },
    ]
}

fn candidate(
    suite: &QualificationSuite,
    evidence: &QualificationEvidence,
    source: &ModelSourceEvidenceBinding,
) -> ModelReleaseCandidate {
    candidate_with_snapshot(
        ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "demo180-nch-candidate-18".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.1".into(),
            },
            source.clone(),
            suite,
            evidence,
            documents(),
            Some(license()),
            Some(impact()),
            Some(compatibility()),
            approvals(),
        )
        .unwrap(),
    )
}

fn candidate_with_snapshot(mut candidate: ModelReleaseCandidate) -> ModelReleaseCandidate {
    candidate.definition_source = model_source(7);
    candidate.definition_metadata = Some(ModelDefinitionMetadata::default());
    candidate
}

#[test]
fn non_negative_finite_rejects_negative_nan_and_infinity() {
    for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let error = NonNegativeFinite::new(invalid).unwrap_err();
        assert_eq!(error.code, QualificationErrorCode::InvalidNumber);
    }
    assert_eq!(
        NonNegativeFinite::new(-0.0).unwrap(),
        NonNegativeFinite::new(0.0).unwrap()
    );
    assert!(ReferenceTolerance::try_new("gain", -0.1, 0.1).is_err());
    assert!(ReferenceErrorEvidence::try_new("gain", f64::NAN, 0.0, 0.1, 0.1).is_err());
    assert!(serde_json::from_str::<NonNegativeFinite>("-0.1").is_err());
}

#[test]
fn suites_are_deterministic_and_reject_case_insensitive_duplicates() {
    let suite = suite();
    assert_eq!(suite.vectors[0].id, "dc-001");
    let duplicate = QualificationSuite::try_new(
        "dc",
        "DC",
        ObjectRevision::INITIAL,
        vec![vector("Case-A", "First", 1), vector("case-a", "Second", 2)],
    )
    .unwrap_err();
    assert_eq!(duplicate.code, QualificationErrorCode::DuplicateId);

    let duplicate_name = QualificationSuite::try_new(
        "dc",
        "DC",
        ObjectRevision::INITIAL,
        vec![vector("a", "Transfer", 1), vector("b", "TRANSFER", 2)],
    )
    .unwrap_err();
    assert_eq!(duplicate_name.code, QualificationErrorCode::DuplicateName);
}

#[test]
fn inconsistent_reference_platform_vector_and_suite_states_fail_closed() {
    let suite = suite();
    let source = source(7);
    let mut first_evidence = evidence(&suite, &source, true);
    first_evidence.vector_outcomes[0].platforms[0].references[0].passed = false;
    let error = first_evidence.validate_bound(&suite, &source).unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::InconsistentResult);

    let mut second_evidence = evidence(&suite, &source, true);
    second_evidence.vector_outcomes[0].platforms[0].passed = false;
    assert_eq!(
        second_evidence
            .validate_bound(&suite, &source)
            .unwrap_err()
            .code,
        QualificationErrorCode::InconsistentResult
    );

    let mut third_evidence = evidence(&suite, &source, true);
    third_evidence.passed = false;
    assert_eq!(
        third_evidence
            .validate_bound(&suite, &source)
            .unwrap_err()
            .code,
        QualificationErrorCode::InconsistentResult
    );
}

#[test]
fn evidence_requires_exact_source_suite_vector_platform_and_tolerance_binding() {
    let suite = suite();
    let source_binding = source(7);
    let mut first_evidence = evidence(&suite, &source_binding, true);
    assert!(
        first_evidence
            .validate_bound(&suite, &source_binding)
            .is_ok()
    );

    let other_source = source(8);
    assert_eq!(
        first_evidence
            .validate_bound(&suite, &other_source)
            .unwrap_err()
            .code,
        QualificationErrorCode::SourceBindingMismatch
    );

    first_evidence.vector_outcomes.pop();
    first_evidence.passed = first_evidence
        .vector_outcomes
        .iter()
        .all(|value| value.passed);
    assert_eq!(
        first_evidence
            .validate_bound(&suite, &source_binding)
            .unwrap_err()
            .code,
        QualificationErrorCode::EvidenceCoverageMismatch
    );

    let mut second_evidence = evidence(&suite, &source_binding, true);
    second_evidence.vector_outcomes[0].platforms[0].references[0].absolute_tolerance =
        NonNegativeFinite::new(99.0).unwrap();
    assert_eq!(
        second_evidence
            .validate_bound(&suite, &source_binding)
            .unwrap_err()
            .code,
        QualificationErrorCode::EvidenceCoverageMismatch
    );
}

#[test]
fn failed_wasm_evidence_is_valid_but_not_promotion_eligible() {
    let suite = suite();
    let source = source(7);
    let evidence = evidence(&suite, &source, false);
    assert!(evidence.validate_bound(&suite, &source).is_ok());
    assert!(!evidence.passed);
    assert!(evidence.platform_passed(QualificationPlatform::Desktop));
    assert!(!evidence.platform_passed(QualificationPlatform::WebAssembly));

    let candidate = candidate(&suite, &evidence, &source);
    let error = candidate
        .validate_for_promotion(&suite, &evidence)
        .unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::PromotionBlocked);
    assert!(!candidate.checklist.webassembly_passed);
}

#[test]
fn missing_required_document_license_impact_or_compatibility_blocks_promotion() {
    let suite = suite();
    let source = source(7);
    let evidence = evidence(&suite, &source, true);

    let incomplete_docs = DocumentationSet::try_new(vec![DocumentationDeclaration {
        kind: RequiredDocumentation::ModelDescription,
        document: document("description", 1),
    }])
    .unwrap();
    let candidate = candidate_with_snapshot(
        ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-docs".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.docs".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            incomplete_docs,
            Some(license()),
            Some(impact()),
            Some(compatibility()),
            approvals(),
        )
        .unwrap(),
    );
    assert_eq!(
        candidate
            .validate_for_promotion(&suite, &evidence)
            .unwrap_err()
            .code,
        QualificationErrorCode::DocumentationIncomplete
    );

    let candidate = candidate_with_snapshot(
        ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-missing".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.missing".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            documents(),
            None,
            None,
            None,
            approvals(),
        )
        .unwrap(),
    );
    assert_eq!(
        candidate
            .validate_for_promotion(&suite, &evidence)
            .unwrap_err()
            .code,
        QualificationErrorCode::LicenseIncomplete
    );

    let candidate = candidate_with_snapshot(
        ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-impact".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.impact".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            documents(),
            Some(license()),
            None,
            Some(compatibility()),
            approvals(),
        )
        .unwrap(),
    );
    assert_eq!(
        candidate
            .validate_for_promotion(&suite, &evidence)
            .unwrap_err()
            .code,
        QualificationErrorCode::ConsumerImpactIncomplete
    );

    let candidate = candidate_with_snapshot(
        ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-compatibility".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.compatibility".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            documents(),
            Some(license()),
            Some(impact()),
            None,
            approvals(),
        )
        .unwrap(),
    );
    assert_eq!(
        candidate
            .validate_for_promotion(&suite, &evidence)
            .unwrap_err()
            .code,
        QualificationErrorCode::CompatibilityIncomplete
    );
}

#[test]
fn license_and_breaking_compatibility_rules_fail_closed() {
    let mut invalid_license = license();
    invalid_license.commercial_use_allowed = false;
    assert_eq!(
        invalid_license.validate_for_release().unwrap_err().code,
        QualificationErrorCode::LicenseIncomplete
    );

    let breaking_impact = ConsumerImpactAssessment {
        change: ConsumerChange::Breaking,
        summary: "Pin meaning changed".into(),
        affected_consumer_ids: vec!["cell-1".into()],
        migration_plan: None,
        reviewed: true,
    };
    assert_eq!(
        breaking_impact.validate_for_release().unwrap_err().code,
        QualificationErrorCode::ConsumerImpactIncomplete
    );

    let mut compatibility = compatibility();
    compatibility.platforms[0].disposition = CompatibilityDisposition::Incompatible;
    assert_eq!(
        compatibility
            .validate_for_release(&impact())
            .unwrap_err()
            .code,
        QualificationErrorCode::CompatibilityIncomplete
    );
}

#[test]
fn approvals_are_independent_and_checklist_cannot_be_forged() {
    let suite = suite();
    let source = source(7);
    let evidence = evidence(&suite, &source, true);
    let mut same_person = approvals();
    same_person[1].approver_id = "M.CHEN".into();
    assert_eq!(
        ModelReleaseCandidate::try_new(
            ReleaseCandidateIdentity {
                id: "candidate-approval".into(),
                model_id: "demo180_nch".into(),
                version: "4.8.3-rc.approval".into(),
            },
            source.clone(),
            &suite,
            &evidence,
            documents(),
            Some(license()),
            Some(impact()),
            Some(compatibility()),
            same_person,
        )
        .unwrap_err()
        .code,
        QualificationErrorCode::DuplicateId
    );

    let mut candidate = candidate(&suite, &evidence, &source);
    candidate.checklist.webassembly_passed = false;
    assert_eq!(
        candidate
            .validate_bound(&suite, &evidence)
            .unwrap_err()
            .code,
        QualificationErrorCode::ChecklistMismatch
    );
}

#[test]
fn guarded_promotion_creates_exactly_bound_release_and_record() {
    let suite = suite();
    let source = source(7);
    let evidence = evidence(&suite, &source, true);
    let candidate = candidate(&suite, &evidence, &source);
    let (record, release) = ModelPromotionRecord::promote(
        "promotion-19",
        ModelReleaseIdentity {
            id: "demo180-nch-release-4.8.3".into(),
            model_id: "demo180_nch".into(),
            version: "4.8.3".into(),
        },
        &candidate,
        &suite,
        &evidence,
    )
    .unwrap();
    let evidence_digest = evidence.content_digest().unwrap();
    assert!(record.checklist.is_complete());
    assert_eq!(release.promotion_record_id, record.id);
    assert_eq!(candidate.evidence_digest, Some(evidence_digest));
    assert_eq!(record.evidence_digest, Some(evidence_digest));
    assert_eq!(release.evidence_digest, Some(evidence_digest));
    assert!(
        record
            .validate_bound(&release, &candidate, &suite, &evidence)
            .is_ok()
    );

    let json = serde_json::to_string(&record).unwrap();
    let restored: ModelPromotionRecord = serde_json::from_str(&json).unwrap();
    assert_eq!(restored, record);
}

#[test]
fn governed_records_reject_missing_or_mismatched_evidence_digests() {
    let suite = suite();
    let source = source(7);
    let evidence = evidence(&suite, &source, true);
    let candidate = candidate(&suite, &evidence, &source);
    let mut serialized = serde_json::to_value(&candidate).unwrap();
    serialized
        .as_object_mut()
        .unwrap()
        .remove("evidence_digest");
    let legacy: ModelReleaseCandidate = serde_json::from_value(serialized).unwrap();
    assert_eq!(legacy.evidence_digest, None);
    assert_eq!(
        legacy.validate_bound(&suite, &evidence).unwrap_err().code,
        QualificationErrorCode::MissingRequiredValue
    );

    let (mut promotion, release) = ModelPromotionRecord::promote(
        "promotion-digest",
        ModelReleaseIdentity {
            id: "release-digest".into(),
            model_id: "demo180_nch".into(),
            version: "4.8.3".into(),
        },
        &candidate,
        &suite,
        &evidence,
    )
    .unwrap();
    promotion.evidence_digest = Some(digest(99));
    assert_eq!(
        promotion
            .validate_bound(&release, &candidate, &suite, &evidence)
            .unwrap_err()
            .code,
        QualificationErrorCode::SourceBindingMismatch
    );
}

#[test]
fn promotion_rejects_mismatched_source_and_failed_evidence() {
    let suite = suite();
    let source = source(7);
    let passing = evidence(&suite, &source, true);
    let candidate = candidate(&suite, &passing, &source);
    let failed = evidence(&suite, &source, false);
    let error = ModelPromotionRecord::promote(
        "promotion-blocked",
        ModelReleaseIdentity {
            id: "release-blocked".into(),
            model_id: "demo180_nch".into(),
            version: "4.8.3".into(),
        },
        &candidate,
        &suite,
        &failed,
    )
    .unwrap_err();
    assert!(matches!(
        error.code,
        QualificationErrorCode::SourceBindingMismatch
            | QualificationErrorCode::ChecklistMismatch
            | QualificationErrorCode::PromotionBlocked
    ));
}

#[test]
fn state_orders_records_and_rejects_case_insensitive_suite_names() {
    let first = suite();
    let second = QualificationSuite::try_new(
        "ac",
        "AC",
        ObjectRevision::INITIAL,
        vec![vector("ac-1", "Gain", 4)],
    )
    .unwrap();
    let state = ModelQualificationState::try_new(
        vec![first.clone(), second],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(state.suites[0].id, "ac");

    let mut duplicate = first;
    duplicate.id = "other".into();
    duplicate.name = "dc iv".into();
    assert_eq!(
        ModelQualificationState::try_new(
            vec![suite(), duplicate],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap_err()
        .code,
        QualificationErrorCode::DuplicateName
    );
}

#[test]
fn suite_only_state_is_rejected_under_the_wrong_model_key() {
    let state = ModelQualificationState::try_new(
        vec![suite()],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    state.validate_for_model("demo180_nch").unwrap();
    assert_eq!(
        state
            .validate_for_model("different_model")
            .unwrap_err()
            .code,
        QualificationErrorCode::SourceBindingMismatch
    );
}

#[test]
fn complete_state_round_trips_with_deterministic_ordering() {
    let suite = suite();
    let source = source(7);
    let evidence = evidence(&suite, &source, true);
    let candidate = candidate(&suite, &evidence, &source);
    let (promotion, release) = ModelPromotionRecord::promote(
        "promotion-19",
        ModelReleaseIdentity {
            id: "release-4.8.3".into(),
            model_id: "demo180_nch".into(),
            version: "4.8.3".into(),
        },
        &candidate,
        &suite,
        &evidence,
    )
    .unwrap();
    let state = ModelQualificationState::try_new(
        vec![suite],
        Vec::new(),
        vec![evidence],
        vec![candidate],
        vec![release],
        vec![promotion],
    )
    .unwrap();
    let first = serde_json::to_string(&state).unwrap();
    let restored: ModelQualificationState = serde_json::from_str(&first).unwrap();
    restored.validate().unwrap();
    restored.validate_for_model("demo180_nch").unwrap();
    assert_eq!(
        restored
            .validate_for_model("different_model")
            .expect_err("cross-model evidence must fail")
            .code,
        QualificationErrorCode::SourceBindingMismatch
    );
    assert_eq!(serde_json::to_string(&restored).unwrap(), first);
    assert_eq!(restored, state);
}

#[test]
fn qualified_section_digest_resolves_only_exact_passing_evidence() {
    let suite = suite();
    let bound_source = source(7);
    let base_evidence = evidence(&suite, &bound_source, true);
    let evidence_digest = base_evidence.content_digest().unwrap();
    let state = ModelQualificationState::try_new(
        vec![suite],
        Vec::new(),
        vec![base_evidence],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    state
        .validate_exact_evidence_digest(&bound_source, evidence_digest)
        .unwrap();
    assert_eq!(
        state
            .validate_exact_evidence_digest(&source(8), evidence_digest)
            .unwrap_err()
            .code,
        QualificationErrorCode::SourceBindingMismatch
    );
    assert_eq!(
        state
            .validate_exact_evidence_digest(&bound_source, digest(99))
            .unwrap_err()
            .code,
        QualificationErrorCode::MissingRequiredValue
    );

    let complete_source = b"* base\n.model demo180_nch NMOS (VTO=0.7)\n.lib TT\n.model demo180_nch NMOS (VTO=0.8)\n.endl TT\n".to_vec();
    let section_source = b".model demo180_nch NMOS (VTO=0.8)\n".to_vec();
    let section_binding = ModelSourceEvidenceBinding::try_new_project_bound(
        "demo180_nch",
        "11111111-1111-4111-8111-111111111111".parse().unwrap(),
        digest_bytes(&complete_source),
        ObjectRevision::new(7).unwrap(),
    )
    .unwrap();
    let mut executable = b"RSpice model qualification\n".to_vec();
    executable.extend_from_slice(&section_source);
    executable.extend_from_slice(b"V1 output 0 DC 1\nM1 output output 0 0 demo180_nch\n.end\n");
    let section_vector = QualificationVector::try_new_source_section_bound(
        "tt-vector",
        "TT operating point",
        section_binding.clone(),
        complete_source,
        Some("TT".to_owned()),
        section_source,
        executable,
        QualificationAnalysis::DcOperatingPoint,
        vec![
            QualificationOutputDefinition::try_new(
                "drain_current",
                QualificationProbe::NodeVoltage {
                    node: "output".into(),
                },
                QualificationSample::OperatingPoint,
            )
            .unwrap(),
        ],
        vec![QualificationReference::try_new("drain_current", 1.0, 0.01, 0.005).unwrap()],
    )
    .unwrap();
    let section_suite = QualificationSuite::try_new(
        "tt-suite",
        "TT qualification",
        ObjectRevision::INITIAL,
        vec![section_vector],
    )
    .unwrap();
    let section_evidence = evidence(&section_suite, &section_binding, true);
    let section_digest = section_evidence.content_digest().unwrap();
    let section_state = ModelQualificationState::try_new(
        vec![section_suite],
        Vec::new(),
        vec![section_evidence],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    section_state
        .validate_exact_section_evidence_digest(&section_binding, "TT", section_digest)
        .unwrap();
    assert_eq!(
        section_state
            .validate_exact_section_evidence_digest(&section_binding, "FF", section_digest,)
            .unwrap_err()
            .code,
        QualificationErrorCode::EvidenceCoverageMismatch
    );
}

#[test]
fn source_revision_reconciliation_prunes_stale_work_but_keeps_release_lineage() {
    let promoted_suite = suite();
    let promoted_source = source(7);
    let promoted_evidence = evidence(&promoted_suite, &promoted_source, true);
    let promoted_candidate = candidate(&promoted_suite, &promoted_evidence, &promoted_source);
    let (promotion, release) = ModelPromotionRecord::promote(
        "promotion-retained",
        ModelReleaseIdentity {
            id: "release-retained".into(),
            model_id: "demo180_nch".into(),
            version: "4.8.3".into(),
        },
        &promoted_candidate,
        &promoted_suite,
        &promoted_evidence,
    )
    .unwrap();

    let stale_suite = QualificationSuite::try_new(
        "stale-suite",
        "Stale suite",
        ObjectRevision::INITIAL,
        vec![vector_for_source("stale-vector", "Stale vector", 3, 8)],
    )
    .unwrap();
    let mut stale_evidence = evidence(&stale_suite, &source(8), true);
    stale_evidence.id = "stale-evidence".into();
    stale_evidence.validate_internal().unwrap();
    let state = ModelQualificationState::try_new(
        vec![promoted_suite.clone(), stale_suite],
        Vec::new(),
        vec![promoted_evidence.clone(), stale_evidence],
        vec![promoted_candidate.clone()],
        vec![release.clone()],
        vec![promotion.clone()],
    )
    .unwrap();

    let current = source(9);
    let reconciled = state.reconcile_after_source_revision(&current).unwrap();
    assert_eq!(reconciled.suites, vec![promoted_suite]);
    assert_eq!(reconciled.evidence, vec![promoted_evidence]);
    assert_eq!(reconciled.candidates, vec![promoted_candidate]);
    assert_eq!(reconciled.releases, vec![release]);
    assert_eq!(reconciled.promotions, vec![promotion]);
    assert!(
        reconciled
            .exact_suites_for_source(&current)
            .unwrap()
            .is_empty()
    );
    reconciled.validate().unwrap();
}

#[test]
fn retained_input_digest_tampering_is_rejected() {
    let mut suite = suite();
    suite.vectors[0].executable_input.push(b' ');
    let error = suite.validate().unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::InputDigestMismatch);
}

#[test]
fn executable_vectors_require_the_exact_bound_candidate_source() {
    let mut wrong_digest = vector("digest", "Digest", 1);
    wrong_digest.model_source = model_source(8);
    assert_eq!(
        wrong_digest.validate("vector").unwrap_err().code,
        QualificationErrorCode::SourceBindingMismatch
    );

    let mut not_embedded = vector("embedded", "Embedded", 2);
    let retained = not_embedded.model_source.clone();
    let executable = String::from_utf8(not_embedded.executable_input.clone())
        .unwrap()
        .replace(&String::from_utf8(retained).unwrap(), "");
    not_embedded.executable_input = executable.into_bytes();
    not_embedded.input_digest = digest_bytes(&not_embedded.executable_input);
    assert_eq!(
        not_embedded.validate("vector").unwrap_err().code,
        QualificationErrorCode::SourceBindingMismatch
    );

    let legacy = ModelSourceEvidenceBinding::try_new(
        "demo180_nch",
        digest_bytes(&model_source(7)),
        ObjectRevision::new(7).unwrap(),
    )
    .unwrap();
    assert_eq!(
        validate_execution_contract(&suite(), &legacy)
            .unwrap_err()
            .code,
        QualificationErrorCode::SourceBindingMismatch
    );
}

#[test]
fn suites_cannot_mix_source_revisions() {
    let error = QualificationSuite::try_new(
        "mixed",
        "Mixed",
        ObjectRevision::INITIAL,
        vec![
            vector_for_source("one", "One", 1, 7),
            vector_for_source("two", "Two", 2, 8),
        ],
    )
    .unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::SourceBindingMismatch);
}

#[test]
fn native_execution_measures_outputs_and_applies_declared_tolerances() {
    let source = source(7);
    let passing_suite = suite();
    let run = QualificationExecutionService::execute_current_platform(
        &passing_suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    assert_eq!(run.platform, QualificationPlatform::Desktop);
    assert!(run.passed);
    assert_eq!(run.vector_outcomes.len(), passing_suite.vectors.len());
    let reference = &run.vector_outcomes[0].outcome.references[0];
    assert_eq!(reference.expected_value, FiniteValue::new(1.0).unwrap());
    assert_eq!(reference.observed_value, FiniteValue::new(1.0).unwrap());

    let mut failing_suite = passing_suite.clone();
    for vector in &mut failing_suite.vectors {
        vector.references[0].expected = FiniteValue::new(2.0).unwrap();
    }
    let run = QualificationExecutionService::execute_current_platform(
        &failing_suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    assert!(!run.passed);
    assert!(
        run.vector_outcomes
            .iter()
            .all(|value| !value.outcome.passed)
    );
}

#[test]
fn ac_cv_noise_and_transient_vectors_execute_real_solver_results() {
    let ac_vector = executable_vector(
        "ac-cv",
        "AC and effective capacitance",
        "V1 output 0 DC 0 AC 1\nM1 output output 0 0 demo180_nch\n.end\n",
        QualificationAnalysis::AcSweep {
            frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
        },
        vec![
            QualificationOutputDefinition::try_new(
                "capacitance",
                QualificationProbe::AcEffectiveCapacitance {
                    branch: "V1".to_owned(),
                    excitation_magnitude: FiniteValue::new(1.0).unwrap(),
                },
                QualificationSample::FirstFrequencyPoint,
            )
            .unwrap(),
            QualificationOutputDefinition::try_new(
                "voltage-magnitude",
                QualificationProbe::AcNodeVoltageMagnitude {
                    node: "output".to_owned(),
                },
                QualificationSample::FirstFrequencyPoint,
            )
            .unwrap(),
        ],
        vec![
            QualificationReference::try_new("capacitance", 0.0, 1.0, 0.0).unwrap(),
            QualificationReference::try_new("voltage-magnitude", 1.0, 1.0e-10, 1.0e-10).unwrap(),
        ],
    );
    let noise_vector = executable_vector(
        "noise",
        "Output noise",
        "V1 input 0 DC 0 AC 1\nR1 input output 1k\nR2 output 0 1k\nM1 output input 0 0 demo180_nch\n.end\n",
        QualificationAnalysis::Noise {
            output_node: "output".to_owned(),
            output_reference: None,
            input_source: "V1".to_owned(),
            frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
            temperature_kelvin: FiniteValue::new(300.15).unwrap(),
        },
        vec![
            QualificationOutputDefinition::try_new(
                "output-noise-density",
                QualificationProbe::NoiseOutputDensity,
                QualificationSample::FirstFrequencyPoint,
            )
            .unwrap(),
        ],
        vec![QualificationReference::try_new("output-noise-density", 0.0, 1.0, 0.0).unwrap()],
    );
    let transient_vector = executable_vector(
        "transient",
        "Transient voltage",
        "V1 output 0 DC 1\nM1 output output 0 0 demo180_nch\n.end\n",
        QualificationAnalysis::Transient {
            stop_time: FiniteValue::new(1.0e-6).unwrap(),
            max_step: FiniteValue::new(1.0e-7).unwrap(),
        },
        vec![
            QualificationOutputDefinition::try_new(
                "output-voltage",
                QualificationProbe::TransientNodeVoltage {
                    node: "output".to_owned(),
                },
                QualificationSample::LastTimePoint,
            )
            .unwrap(),
        ],
        vec![QualificationReference::try_new("output-voltage", 1.0, 1.0e-8, 1.0e-8).unwrap()],
    );
    let suite = QualificationSuite::try_new(
        "advanced",
        "Advanced qualification",
        ObjectRevision::INITIAL,
        vec![ac_vector, noise_vector, transient_vector],
    )
    .unwrap();

    let run = QualificationExecutionService::execute_current_platform(
        &suite,
        &source(7),
        &rspice_core::NoAbort,
    )
    .unwrap();
    assert!(run.passed, "{run:#?}");
    for outcome in &run.vector_outcomes {
        assert!(outcome.outcome.failure.is_none(), "{outcome:#?}");
        assert!(
            outcome
                .outcome
                .references
                .iter()
                .all(|reference| reference.observed_value.get().is_finite()),
            "{outcome:#?}"
        );
    }
    let ac = run
        .vector_outcomes
        .iter()
        .find(|outcome| outcome.vector_id == "ac-cv")
        .unwrap();
    let voltage = ac
        .outcome
        .references
        .iter()
        .find(|reference| reference.quantity == "voltage-magnitude")
        .unwrap();
    assert!((voltage.observed_value.get() - 1.0).abs() <= 1.0e-10);
}

#[test]
fn advanced_analysis_axes_and_probe_domains_fail_closed() {
    let mut vector = executable_vector(
        "axis",
        "Frequency axis",
        "V1 output 0 DC 0 AC 1\nM1 output output 0 0 demo180_nch\n.end\n",
        QualificationAnalysis::AcSweep {
            frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
        },
        vec![
            QualificationOutputDefinition::try_new(
                "frequency",
                QualificationProbe::FrequencyValue,
                QualificationSample::FirstFrequencyPoint,
            )
            .unwrap(),
        ],
        vec![QualificationReference::try_new("frequency", 1.0e3, 0.0, 0.0).unwrap()],
    );
    vector.analysis = QualificationAnalysis::AcSweep {
        frequencies: vec![
            FiniteValue::new(1.0e3).unwrap(),
            FiniteValue::new(1.0e3).unwrap(),
        ],
    };
    let error = QualificationSuite::try_new(
        "invalid-axis",
        "Invalid frequency axis",
        ObjectRevision::INITIAL,
        vec![vector.clone()],
    )
    .unwrap_err();
    assert_eq!(
        error.code,
        QualificationErrorCode::InvalidExecutionDefinition
    );

    vector.analysis = QualificationAnalysis::AcSweep {
        frequencies: vec![FiniteValue::new(1.0e3).unwrap()],
    };
    vector.outputs[0].probe = QualificationProbe::NoiseOutputDensity;
    let error = QualificationSuite::try_new(
        "invalid-probe",
        "Invalid probe domain",
        ObjectRevision::INITIAL,
        vec![vector],
    )
    .unwrap_err();
    assert_eq!(
        error.code,
        QualificationErrorCode::InvalidExecutionDefinition
    );
}

#[test]
fn execution_failures_are_retained_as_failing_platform_outcomes() {
    let source = source(7);
    let mut suite = suite();
    suite.vectors[0].outputs[0].probe = QualificationProbe::NodeVoltage {
        node: "missing-node".into(),
    };
    let run = QualificationExecutionService::execute_current_platform(
        &suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let failed = run
        .vector_outcomes
        .iter()
        .find(|value| value.vector_id == "dc-001")
        .unwrap();
    assert!(!failed.outcome.passed);
    assert_eq!(
        failed.outcome.failure.as_ref().unwrap().stage,
        QualificationFailureStage::Measurement
    );
    assert!(failed.outcome.references.is_empty());
    assert!(run.validate_bound(&suite, &source).is_ok());
}

#[test]
fn missing_runtime_parity_cannot_be_promoted_to_evidence() {
    let source = source(7);
    let suite = suite();
    let desktop = QualificationExecutionService::execute_current_platform(
        &suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let error = QualificationExecutionService::assemble_evidence(
        "evidence",
        &suite,
        &source,
        vec![desktop],
    )
    .unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::EvidenceCoverageMismatch);
}

#[test]
fn cancellation_publishes_no_partial_platform_run() {
    let result = QualificationExecutionService::execute_current_platform(
        &suite(),
        &source(7),
        &rspice_core::abort_signal::ImmediateAbort,
    );
    assert!(matches!(
        result,
        Err(QualificationExecutionError::Cancelled)
    ));
}

#[test]
fn cooperative_session_runs_one_vector_per_step_and_cancels_atomically() {
    let source = source(7);
    let suite = suite();
    let mut session = QualificationExecutionSession::try_new(&suite, &source).unwrap();
    assert_eq!(session.progress().completed_vectors, 0);
    let step = session.step(&rspice_core::NoAbort).unwrap();
    let QualificationExecutionStep::InProgress(progress) = step else {
        panic!("a two-vector suite must not publish a run after one step");
    };
    assert_eq!(progress.completed_vectors, 1);
    assert!(!session.is_finished());
    session.cancel();
    assert!(session.is_cancelled());
    assert!(matches!(
        session.step(&rspice_core::NoAbort),
        Err(QualificationExecutionError::Cancelled)
    ));
}

#[test]
fn cooperative_session_publishes_only_the_terminal_validated_run() {
    let source = source(7);
    let suite = suite();
    let mut session = QualificationExecutionSession::try_new(&suite, &source).unwrap();
    assert!(matches!(
        session.step(&rspice_core::NoAbort).unwrap(),
        QualificationExecutionStep::InProgress(_)
    ));
    let terminal = session.step(&rspice_core::NoAbort).unwrap();
    let QualificationExecutionStep::Complete { progress, run } = terminal else {
        panic!("the final vector must publish the complete platform run");
    };
    assert_eq!(progress.completed_vectors, suite.vectors.len());
    run.validate_bound(&suite, &source).unwrap();
    assert!(session.is_finished());
    assert!(matches!(
        session.step(&rspice_core::NoAbort),
        Err(QualificationExecutionError::SessionFinished)
    ));
}

#[test]
fn assembled_evidence_is_deterministic_and_requires_real_run_records() {
    let source = source(7);
    let suite = suite();
    let desktop = QualificationExecutionService::execute_current_platform(
        &suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let wasm_outcomes = desktop
        .vector_outcomes
        .iter()
        .map(|value| {
            let mut value = value.clone();
            value.outcome.platform = QualificationPlatform::WebAssembly;
            value
        })
        .collect();
    // Imported platform runs use the same validator as locally-produced
    // runs. Production code cannot relabel `execute_current_platform`.
    let wasm = QualificationPlatformRun::try_new(
        QualificationPlatform::WebAssembly,
        source.clone(),
        &suite,
        wasm_outcomes,
    )
    .unwrap();
    let first = QualificationExecutionService::assemble_evidence(
        "evidence",
        &suite,
        &source,
        vec![desktop.clone(), wasm.clone()],
    )
    .unwrap();
    let second = QualificationExecutionService::assemble_evidence(
        "evidence",
        &suite,
        &source,
        vec![wasm, desktop],
    )
    .unwrap();
    assert_eq!(first, second);
    assert_eq!(
        serde_json::to_string(&first).unwrap(),
        serde_json::to_string(&second).unwrap()
    );
}

#[test]
fn schema_one_inputs_are_not_migrated_by_inventing_execution_contracts() {
    let legacy = serde_json::json!({
        "schema_version": 1,
        "id": "legacy",
        "name": "Legacy",
        "revision": 1,
        "vectors": [{
            "id": "legacy-vector",
            "name": "Legacy vector",
            "input_digest": digest(1).to_string(),
            "tolerances": [{"quantity": "out", "absolute": 0.1, "relative": 0.1}]
        }]
    });
    assert!(serde_json::from_value::<QualificationSuite>(legacy).is_err());
}

#[test]
fn empty_legacy_state_migrates_without_inventing_evidence() {
    for version in [2, 3] {
        let mut serialized = serde_json::to_value(ModelQualificationState::default()).unwrap();
        serialized["schema_version"] = serde_json::json!(version);
        if version == 2 {
            serialized.as_object_mut().unwrap().remove("platform_runs");
        }
        let restored: ModelQualificationState = serde_json::from_value(serialized).unwrap();
        assert_eq!(restored.schema_version, MODEL_QUALIFICATION_SCHEMA_VERSION);
        assert!(restored.platform_runs.is_empty());
        restored.validate().unwrap();
    }
}

#[test]
fn populated_schema_two_state_is_loaded_for_diagnosis_but_not_upgraded() {
    let mut serialized = serde_json::to_value(
        ModelQualificationState::try_new(
            vec![suite()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap(),
    )
    .unwrap();
    serialized["schema_version"] = serde_json::json!(2);
    let restored: ModelQualificationState = serde_json::from_value(serialized).unwrap();
    assert_eq!(restored.schema_version, 2);
    assert_eq!(
        restored.validate().unwrap_err().code,
        QualificationErrorCode::UnsupportedSchema
    );
}

#[test]
fn populated_schema_three_state_is_not_upgraded_by_inventing_evidence_digests() {
    let suite = suite();
    let source = source(7);
    let evidence = evidence(&suite, &source, true);
    let candidate = candidate(&suite, &evidence, &source);
    let mut serialized = serde_json::to_value(
        ModelQualificationState::try_new(
            vec![suite],
            Vec::new(),
            vec![evidence],
            vec![candidate],
            Vec::new(),
            Vec::new(),
        )
        .unwrap(),
    )
    .unwrap();
    serialized["schema_version"] = serde_json::json!(3);
    serialized["candidates"][0]
        .as_object_mut()
        .unwrap()
        .remove("evidence_digest");

    let restored: ModelQualificationState = serde_json::from_value(serialized).unwrap();
    assert_eq!(restored.schema_version, 3);
    assert_eq!(restored.candidates[0].evidence_digest, None);
    assert_eq!(
        restored.validate().unwrap_err().code,
        QualificationErrorCode::UnsupportedSchema
    );
}

#[test]
fn persisted_qualification_contracts_reject_unknown_fields() {
    let mut state = serde_json::to_value(ModelQualificationState::default()).unwrap();
    state["future_field"] = serde_json::json!(true);
    assert!(serde_json::from_value::<ModelQualificationState>(state).is_err());

    let mut suite_value = serde_json::to_value(suite()).unwrap();
    suite_value["vectors"][0]["future_field"] = serde_json::json!(true);
    assert!(serde_json::from_value::<QualificationSuite>(suite_value).is_err());
}

#[test]
fn evidence_upsert_is_append_only_idempotent_and_atomic() {
    let suite = suite();
    let source = source(7);
    let original = evidence(&suite, &source, true);
    let release_candidate = candidate(&suite, &original, &source);
    let mut state = ModelQualificationState::try_new(
        vec![suite.clone()],
        Vec::new(),
        vec![original.clone()],
        vec![release_candidate],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let before = state.clone();
    let mut invalid = original.clone();
    invalid.vector_outcomes.pop();
    assert_eq!(
        state.upsert_evidence_atomically(invalid).unwrap_err().code,
        QualificationErrorCode::ImmutableRecord
    );
    assert_eq!(state, before);

    let replacement = evidence(&suite, &source, false);
    assert_eq!(
        state
            .upsert_evidence_atomically(replacement)
            .unwrap_err()
            .code,
        QualificationErrorCode::ImmutableRecord
    );
    assert_eq!(state, before);
    state.upsert_evidence_atomically(original).unwrap();
    assert_eq!(state, before);
    assert!(state.evidence[0].passed);
    state.validate().unwrap();
}

#[test]
fn atomic_promotion_commits_release_and_record_or_rolls_back_duplicates() {
    let suite = suite();
    let source = source(7);
    let evidence = evidence(&suite, &source, true);
    let candidate = candidate(&suite, &evidence, &source);
    let candidate_id = candidate.identity.id.clone();
    let mut state = ModelQualificationState::try_new(
        vec![suite],
        Vec::new(),
        vec![evidence],
        vec![candidate],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let release_identity = ModelReleaseIdentity {
        id: "release-atomic".into(),
        model_id: "demo180_nch".into(),
        version: "4.8.3".into(),
    };
    state
        .promote_candidate_atomically("promotion-atomic", release_identity.clone(), &candidate_id)
        .unwrap();
    assert_eq!(state.releases.len(), 1);
    assert_eq!(state.promotions.len(), 1);
    state.validate().unwrap();

    let committed = state.clone();
    let repeated_candidate = state.promote_candidate_atomically(
        "promotion-second",
        ModelReleaseIdentity {
            id: "release-second".into(),
            model_id: "demo180_nch".into(),
            version: "4.8.4".into(),
        },
        &candidate_id,
    );
    assert_eq!(
        repeated_candidate.unwrap_err().code,
        QualificationErrorCode::DuplicateId
    );
    assert_eq!(state, committed);

    let duplicate_promotion = state.promote_candidate_atomically(
        "promotion-atomic",
        ModelReleaseIdentity {
            id: "release-other".into(),
            model_id: "demo180_nch".into(),
            version: "4.8.4".into(),
        },
        &candidate_id,
    );
    assert_eq!(
        duplicate_promotion.unwrap_err().code,
        QualificationErrorCode::DuplicateId
    );
    assert_eq!(state, committed);

    let duplicate_release =
        state.promote_candidate_atomically("promotion-other", release_identity, &candidate_id);
    assert_eq!(
        duplicate_release.unwrap_err().code,
        QualificationErrorCode::DuplicateId
    );
    assert_eq!(state, committed);
}

#[test]
fn platform_runs_persist_and_exact_pair_assembles_evidence_atomically() {
    let suite = suite();
    let source = source(7);
    let desktop = QualificationExecutionService::execute_current_platform(
        &suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let wasm = imported_platform_run(
        &desktop,
        QualificationPlatform::WebAssembly,
        &suite,
        &source,
    );
    let state = ModelQualificationState::try_new(
        vec![suite.clone()],
        vec![wasm, desktop],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(
        state.platform_runs[0].platform,
        QualificationPlatform::Desktop
    );
    let json = serde_json::to_string(&state).unwrap();
    let mut restored: ModelQualificationState = serde_json::from_str(&json).unwrap();
    restored.validate().unwrap();
    assert_eq!(restored, state);
    let [desktop, wasm] = restored
        .exact_platform_run_pair(&suite.id, &source)
        .unwrap();
    assert_eq!(desktop.platform, QualificationPlatform::Desktop);
    assert_eq!(wasm.platform, QualificationPlatform::WebAssembly);

    let assembled = restored
        .assemble_and_upsert_evidence_atomically("assembled", &suite.id, &source)
        .unwrap();
    assert!(assembled.passed);
    assert_eq!(restored.evidence, vec![assembled]);
    restored.validate().unwrap();
}

#[test]
fn stale_tampered_or_incomplete_platform_runs_roll_back() {
    let suite = suite();
    let source = source(7);
    let desktop = QualificationExecutionService::execute_current_platform(
        &suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let mut state = ModelQualificationState::try_new(
        vec![suite.clone()],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let empty = state.clone();

    let mut stale = desktop.clone();
    stale.suite_revision = stale.suite_revision.next().unwrap();
    assert_eq!(
        state
            .upsert_platform_run_atomically(stale)
            .unwrap_err()
            .code,
        QualificationErrorCode::SuiteBindingMismatch
    );
    assert_eq!(state, empty);

    let mut tampered = desktop.clone();
    tampered.vector_outcomes[0].input_digest = digest(99);
    assert_eq!(
        state
            .upsert_platform_run_atomically(tampered)
            .unwrap_err()
            .code,
        QualificationErrorCode::EvidenceCoverageMismatch
    );
    assert_eq!(state, empty);

    state
        .upsert_platform_run_atomically(desktop.clone())
        .unwrap();
    let desktop_only = state.clone();
    assert_eq!(
        state
            .assemble_and_upsert_evidence_atomically("incomplete", &suite.id, &source)
            .unwrap_err()
            .code,
        QualificationErrorCode::EvidenceCoverageMismatch
    );
    assert_eq!(state, desktop_only);

    let duplicate = ModelQualificationState::try_new(
        vec![suite],
        vec![desktop.clone(), desktop],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap_err();
    assert_eq!(duplicate.code, QualificationErrorCode::DuplicateId);
}

#[test]
fn suite_and_vector_lifecycle_is_atomic_revisioned_and_evidence_safe() {
    let suite = suite();
    let source = source(7);
    let desktop = QualificationExecutionService::execute_current_platform(
        &suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let mut state = ModelQualificationState::try_new(
        vec![suite.clone()],
        vec![desktop],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    assert_eq!(
        state
            .qualification_vector(&suite.id, "dc-001")
            .unwrap()
            .name,
        "Transfer sweep"
    );

    let replacement = vector_for_source("dc-001", "Transfer sweep revised", 9, 7);
    state
        .replace_vector_atomically(&suite.id, "dc-001", replacement)
        .unwrap();
    assert_eq!(
        state.qualification_suite(&suite.id).unwrap().revision,
        suite.revision.next().unwrap()
    );
    assert_eq!(
        state
            .qualification_vector(&suite.id, "dc-001")
            .unwrap()
            .name,
        "Transfer sweep revised"
    );
    assert!(state.platform_runs.is_empty());

    state.delete_vector_atomically(&suite.id, "dc-002").unwrap();
    assert_eq!(
        state.qualification_suite(&suite.id).unwrap().vectors.len(),
        1
    );
    let retained = state.clone();
    let error = state
        .delete_vector_atomically(&suite.id, "dc-001")
        .unwrap_err();
    assert_eq!(
        error.code,
        QualificationErrorCode::InvalidExecutionDefinition
    );
    assert_eq!(state, retained);

    state.delete_suite_atomically(&suite.id).unwrap();
    assert!(state.suites.is_empty());

    let evidence = evidence(&suite, &source, true);
    let release_candidate = candidate(&suite, &evidence, &source);
    let mut immutable = ModelQualificationState::try_new(
        vec![suite.clone()],
        Vec::new(),
        vec![evidence],
        vec![release_candidate],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let retained = immutable.clone();
    let error = immutable
        .replace_vector_atomically(
            &suite.id,
            "dc-001",
            vector_for_source("dc-001", "Forbidden edit", 9, 7),
        )
        .unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::ImmutableRecord);
    assert_eq!(immutable, retained);
    assert_eq!(
        immutable
            .delete_suite_atomically(&suite.id)
            .unwrap_err()
            .code,
        QualificationErrorCode::ImmutableRecord
    );
    assert_eq!(immutable, retained);
}

#[test]
fn failed_disposition_blocks_promotion_until_exact_cross_platform_rerun_passes() {
    let suite = suite();
    let source = source(7);
    let passing_evidence = evidence(&suite, &source, true);
    let release_candidate = candidate(&suite, &passing_evidence, &source);
    let candidate_id = release_candidate.identity.id.clone();
    let failed = failed_platform_run(&suite, &source, QualificationPlatform::Desktop);
    let mut state = ModelQualificationState::try_new(
        vec![suite.clone()],
        vec![failed],
        vec![passing_evidence],
        vec![release_candidate],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    state
        .record_vector_disposition_atomically(
            "disp-failed-dc-001",
            &suite.id,
            "dc-001",
            &source,
            QualificationVectorDispositionCause::Failed,
            QualificationVectorRequiredAction::Rerun,
            "Reference mismatch requires an exact parity rerun",
        )
        .unwrap();
    let retained = state.clone();
    let error = state
        .promote_candidate_atomically(
            "promotion-blocked",
            ModelReleaseIdentity {
                id: "release-blocked".to_owned(),
                model_id: source.model_id.clone(),
                version: "4.8.3".to_owned(),
            },
            &candidate_id,
        )
        .unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::PromotionBlocked);
    assert_eq!(state, retained);
    assert_eq!(
        state
            .resolve_vector_disposition_by_rerun_atomically("disp-failed-dc-001")
            .unwrap_err()
            .code,
        QualificationErrorCode::DispositionInvalid
    );
    assert_eq!(state, retained);

    let desktop = QualificationExecutionService::execute_current_platform(
        &suite,
        &source,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let wasm = imported_platform_run(
        &desktop,
        QualificationPlatform::WebAssembly,
        &suite,
        &source,
    );
    state.upsert_platform_run_atomically(desktop).unwrap();
    state.upsert_platform_run_atomically(wasm).unwrap();
    state
        .resolve_vector_disposition_by_rerun_atomically("disp-failed-dc-001")
        .unwrap();
    assert!(!state.vector_dispositions[0].is_open());
    state
        .promote_candidate_atomically(
            "promotion-after-rerun",
            ModelReleaseIdentity {
                id: "release-after-rerun".to_owned(),
                model_id: source.model_id.clone(),
                version: "4.8.3".to_owned(),
            },
            &candidate_id,
        )
        .unwrap();
    assert_eq!(state.promotions.len(), 1);
}

#[test]
fn stale_dispositions_cannot_be_waived_by_rerun_and_retirement_is_explicit() {
    let suite = suite();
    let current_source = source(8);
    let mut state = ModelQualificationState::try_new(
        vec![suite.clone()],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let empty = state.clone();
    let error = state
        .record_vector_disposition_atomically(
            "stale-rerun",
            &suite.id,
            "dc-001",
            &current_source,
            QualificationVectorDispositionCause::Stale,
            QualificationVectorRequiredAction::Rerun,
            "Source revision changed",
        )
        .unwrap_err();
    assert_eq!(error.code, QualificationErrorCode::DispositionInvalid);
    assert_eq!(state, empty);

    state
        .record_vector_disposition_atomically(
            "stale-retire",
            &suite.id,
            "dc-001",
            &current_source,
            QualificationVectorDispositionCause::Stale,
            QualificationVectorRequiredAction::Retire,
            "The old source-bound vector is outside the current candidate",
        )
        .unwrap();
    assert!(state.vector_dispositions[0].is_open());
    state.delete_suite_atomically(&suite.id).unwrap();
    assert_eq!(
        state.vector_dispositions[0].resolution,
        Some(QualificationVectorDispositionResolution::Retired)
    );
    state.validate().unwrap();
}
