use super::*;

pub(super) fn canonical_fixture() -> CapabilityFixture {
    let canonical_object_ids = CanonicalObjectIds {
        project: "project-demo180".into(),
        design: "design-demo180-inverter".into(),
        testbench: "testbench-demo180-inverter".into(),
        simulation_plan: "simulation-plan-demo180-inverter".into(),
        run_set: "run-set-demo180-inverter".into(),
        job: "job-demo180-inverter".into(),
        run: "run-demo180-inverter".into(),
        nominal_dataset: "dataset-demo180-nominal".into(),
        pvt_dataset: "dataset-demo180-pvt".into(),
        result_document: "result-document-demo180".into(),
        verification_evidence: "verification-evidence-demo180".into(),
        release_candidate: "release-candidate-demo180".into(),
        model_binding: "model-binding-demo180".into(),
        review_comment: "review-comment-demo180".into(),
        approval: "approval-demo180".into(),
    };
    let build_id = BuildId::from("build-rspice-design-fixture-0.1.0");
    let native_engine_id = EngineId::from("engine-rspice-native-fixture-0.1.0+91e7c2a");
    let preview_engine_id = EngineId::from("engine-rspice-preview-fixture-0.1.0+91e7c2a");
    let model_binding_id = ModelBindingId::from("model-binding-demo180-2.3.1");
    let default_entitlement_id = EntitlementPolicyId::from("entitlement-standard");

    let baseline_targets = ClaimTargets {
        build_id: Nullable::some(build_id.clone()),
        platform_id: Nullable::none(),
        compute_target_id: Nullable::none(),
        engine_id: Nullable::none(),
        model_binding_id: Nullable::none(),
        external_producer_id: Nullable::none(),
        connector_id: Nullable::none(),
        entitlement_policy_id: Nullable::some(default_entitlement_id.clone()),
        canonical_object_id: Nullable::some(canonical_object_ids.design.clone()),
    };
    let release_targets = ClaimTargets {
        build_id: Nullable::some(build_id.clone()),
        platform_id: Nullable::some(PlatformId::Desktop),
        compute_target_id: Nullable::some(ComputeTargetId::from("local-desktop-12")),
        engine_id: Nullable::some(native_engine_id.clone()),
        model_binding_id: Nullable::some(model_binding_id.clone()),
        external_producer_id: Nullable::none(),
        connector_id: Nullable::none(),
        entitlement_policy_id: Nullable::some(default_entitlement_id.clone()),
        canonical_object_id: Nullable::some(canonical_object_ids.release_candidate.clone()),
    };

    let mut evidence = vec![
        evidence_record(
            "design-contract-current",
            EvidenceKind::DesignContract,
            AuthorityClass::DesignContract,
            &baseline_targets,
            VerificationState::Verified,
            ClaimState::Current,
            ApplicabilityState::Applicable,
            vec![],
        ),
        evidence_record(
            "design-contract-stale",
            EvidenceKind::DesignContract,
            AuthorityClass::DesignContract,
            &baseline_targets,
            VerificationState::Verified,
            ClaimState::Stale,
            ApplicabilityState::Applicable,
            vec![],
        ),
        evidence_record(
            "design-contract-not-applicable",
            EvidenceKind::DesignContract,
            AuthorityClass::DesignContract,
            &baseline_targets,
            VerificationState::Verified,
            ClaimState::Current,
            ApplicabilityState::NotApplicable,
            vec![],
        ),
        evidence_record(
            "design-contract-unverified",
            EvidenceKind::DesignContract,
            AuthorityClass::DesignContract,
            &baseline_targets,
            VerificationState::Unverified,
            ClaimState::Current,
            ApplicabilityState::Applicable,
            vec![],
        ),
        evidence_record(
            "release-qualification-current",
            EvidenceKind::ReleaseQualification,
            AuthorityClass::ReleaseEvidence,
            &release_targets,
            VerificationState::Verified,
            ClaimState::Current,
            ApplicabilityState::Applicable,
            vec![ProtectedLabelFamily::Release],
        ),
        evidence_record(
            "qualified-contract-vector-current",
            EvidenceKind::EngineConformance,
            AuthorityClass::ContractTestVector,
            &engine_targets(&baseline_targets, &native_engine_id),
            VerificationState::Verified,
            ClaimState::Current,
            ApplicabilityState::Applicable,
            vec![ProtectedLabelFamily::Qualified],
        ),
        evidence_record(
            "qualified-contract-vector-stale",
            EvidenceKind::EngineConformance,
            AuthorityClass::ContractTestVector,
            &engine_targets(&baseline_targets, &native_engine_id),
            VerificationState::Verified,
            ClaimState::Stale,
            ApplicabilityState::Applicable,
            vec![ProtectedLabelFamily::Qualified],
        ),
    ];

    let mut unknown_build_targets = baseline_targets.clone();
    unknown_build_targets.build_id = Nullable::some(BuildId::from("unknown-build"));
    evidence.push(evidence_record(
        "design-contract-unknown-build",
        EvidenceKind::DesignContract,
        AuthorityClass::DesignContract,
        &unknown_build_targets,
        VerificationState::Verified,
        ClaimState::Current,
        ApplicabilityState::Applicable,
        vec![],
    ));

    let mut expired_entitlement_targets = baseline_targets.clone();
    expired_entitlement_targets.entitlement_policy_id =
        Nullable::some(EntitlementPolicyId::from("entitlement-expired"));
    evidence.push(evidence_record(
        "design-contract-expired-entitlement",
        EvidenceKind::DesignContract,
        AuthorityClass::DesignContract,
        &expired_entitlement_targets,
        VerificationState::Verified,
        ClaimState::Current,
        ApplicabilityState::Applicable,
        vec![],
    ));

    let mut denied_entitlement_targets = baseline_targets.clone();
    denied_entitlement_targets.entitlement_policy_id =
        Nullable::some(EntitlementPolicyId::from("entitlement-denied"));
    evidence.push(evidence_record(
        "design-contract-denied-entitlement",
        EvidenceKind::DesignContract,
        AuthorityClass::DesignContract,
        &denied_entitlement_targets,
        VerificationState::Verified,
        ClaimState::Current,
        ApplicabilityState::Applicable,
        vec![],
    ));

    let mut ineligible_release_targets = release_targets.clone();
    ineligible_release_targets.platform_id = Nullable::some(PlatformId::Browser);
    evidence.push(evidence_record(
        "release-qualification-ineligible-target",
        EvidenceKind::ReleaseQualification,
        AuthorityClass::ReleaseEvidence,
        &ineligible_release_targets,
        VerificationState::Verified,
        ClaimState::Current,
        ApplicabilityState::Applicable,
        vec![ProtectedLabelFamily::Release],
    ));

    let resolution_cases = vec![
        resolution_case(
            "current-design-contract",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::DesignComplete,
            baseline_targets.clone(),
            vec!["design-contract-current"],
            ClaimState::Current,
            true,
            vec![],
        ),
        resolution_case(
            "stale-design-contract",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::DesignComplete,
            baseline_targets.clone(),
            vec!["design-contract-stale"],
            ClaimState::Stale,
            false,
            vec![ReasonCode::EvidenceStale],
        ),
        resolution_case(
            "unavailable-design-contract",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::DesignComplete,
            baseline_targets.clone(),
            vec!["design-contract-not-applicable"],
            ClaimState::Unavailable,
            false,
            vec![ReasonCode::EvidenceNotApplicable],
        ),
        resolution_case(
            "unverified-design-contract",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::DesignComplete,
            baseline_targets.clone(),
            vec!["design-contract-unverified"],
            ClaimState::Unknown,
            false,
            vec![ReasonCode::EvidenceUnverified],
        ),
        resolution_case(
            "missing-design-evidence",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::DesignComplete,
            baseline_targets.clone(),
            vec![],
            ClaimState::Unknown,
            false,
            vec![ReasonCode::MissingRequiredEvidence],
        ),
        resolution_case(
            "unknown-build-target",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::DesignComplete,
            unknown_build_targets,
            vec!["design-contract-unknown-build"],
            ClaimState::Unknown,
            false,
            vec![ReasonCode::TargetReferenceUnknown],
        ),
        resolution_case(
            "expired-entitlement",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::DesignComplete,
            expired_entitlement_targets,
            vec!["design-contract-expired-entitlement"],
            ClaimState::Stale,
            false,
            vec![ReasonCode::EntitlementExpired],
        ),
        resolution_case(
            "denied-entitlement",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::DesignComplete,
            denied_entitlement_targets,
            vec!["design-contract-denied-entitlement"],
            ClaimState::Unavailable,
            false,
            vec![ReasonCode::EntitlementDenied],
        ),
        resolution_case(
            "release-label-without-evidence",
            ClaimMode::Product,
            "Production release ready",
            SubjectKind::Release,
            CapabilityReadinessStage::PlatformQualified,
            release_base_targets(&baseline_targets, &canonical_object_ids.release_candidate),
            vec![],
            ClaimState::Unknown,
            false,
            vec![ReasonCode::MissingRequiredEvidence],
        ),
        resolution_case(
            "current-release-contract",
            ClaimMode::Product,
            "Production release ready",
            SubjectKind::Release,
            CapabilityReadinessStage::PlatformQualified,
            release_targets.clone(),
            vec!["release-qualification-current"],
            ClaimState::Current,
            true,
            vec![],
        ),
        resolution_case(
            "release-ineligible-target",
            ClaimMode::Product,
            "Production release ready",
            SubjectKind::Release,
            CapabilityReadinessStage::PlatformQualified,
            ineligible_release_targets,
            vec!["release-qualification-ineligible-target"],
            ClaimState::Unavailable,
            false,
            vec![ReasonCode::TargetProtectedClaimIneligible],
        ),
        resolution_case(
            "registered-product-claim",
            ClaimMode::Product,
            "Design complete",
            SubjectKind::Design,
            CapabilityReadinessStage::Registered,
            baseline_targets.clone(),
            vec![],
            ClaimState::Unknown,
            false,
            vec![ReasonCode::RegisteredProductClaimForbidden],
        ),
        resolution_case(
            "positive-qualified-contract-test-vector",
            ClaimMode::ContractTestVector,
            "Engine qualified",
            SubjectKind::Engine,
            CapabilityReadinessStage::EngineReady,
            engine_targets(&baseline_targets, &native_engine_id),
            vec!["qualified-contract-vector-current"],
            ClaimState::Current,
            false,
            vec![],
        ),
        resolution_case(
            "stale-qualified-contract-test-vector",
            ClaimMode::ContractTestVector,
            "Engine qualified",
            SubjectKind::Engine,
            CapabilityReadinessStage::EngineReady,
            engine_targets(&baseline_targets, &native_engine_id),
            vec!["qualified-contract-vector-stale"],
            ClaimState::Stale,
            false,
            vec![ReasonCode::EvidenceStale],
        ),
    ];

    CapabilityFixture {
        schema: "https://rspice.dev/schemas/capability-design-fixture/1.0.0".into(),
        schema_version: "1.0.0".into(),
        fixture_id: "rspice-capability-design-fixture".into(),
        fixture_revision: "tracked-rust-fixture-r1".into(),
        status: FixtureStatus::DesignFixture,
        as_of: Timestamp::from("2026-07-13T12:00:00Z"),
        boundary: CapabilityBoundary {
            purpose: "Deterministic capability resolver contract vectors.".into(),
            does_not_claim:
                "This design fixture does not prove production readiness or release eligibility."
                    .into(),
            currentness_meaning: "Current within the deterministic fixture timestamp.".into(),
            test_vector_meaning: "Contract vectors exercise policy without authorizing labels."
                .into(),
            unknown_policy: "Unknown fails closed.".into(),
            stale_policy: "Stale fails closed.".into(),
            unavailable_policy: "Unavailable fails closed.".into(),
        },
        authority: FixtureAuthority {
            plan_sections: vec!["capability-readiness".into()],
            canonical_fixture: ContractAuthority {
                path: "crates/rspice-ui/src/product/capability_readiness_test_fixture.rs".into(),
                id: "tracked-capability-test-fixture".into(),
                schema_version: "1.0.0".into(),
            },
            commercial_scope: ContractAuthority {
                path: "product/commercial-scope".into(),
                id: "commercial-scope-contract".into(),
                schema_version: "1.0.0".into(),
            },
            platform_task: ContractAuthority {
                path: "product/platform-task".into(),
                id: "platform-task-contract".into(),
                schema_version: "1.0.0".into(),
            },
            product_manifest_path: "product/product-manifest".into(),
            resolver_path: "product/capability-readiness-resolver".into(),
        },
        vocabulary: canonical_vocabulary(),
        build_identity: BuildIdentity {
            id: build_id,
            package_name: "rspice-ui".into(),
            package_version: "0.1.0".into(),
            artifact_kind: "gui-design-mockup".into(),
            fixture_revision: "tracked-rust-fixture-r1".into(),
            product_fixture_id: "rspice-product-fixture".into(),
            product_fixture_schema_version: "1.0.0".into(),
            canonical_input_revision: "tracked-rust-fixture-r1".into(),
            engine_build_fixture_id: "engine-build-rspice-fixture-0.1.0+91e7c2a".into(),
            state: ClaimState::Current,
            protected_claim_eligible: true,
            qualification_boundary: "Design identity only; production requires signed evidence."
                .into(),
        },
        canonical_object_ids,
        platform_targets: PlatformTargets {
            desktop: platform_record(PlatformId::Desktop, ClaimState::Current, true),
            browser: platform_record(PlatformId::Browser, ClaimState::Current, false),
            tablet: platform_record(PlatformId::Tablet, ClaimState::Unknown, false),
            phone: platform_record(PlatformId::Phone, ClaimState::Unavailable, false),
        },
        compute_targets: vec![ComputeTarget {
            id: "local-desktop-12".into(),
            mode: ComputeMode::Local,
            state: ClaimState::Current,
            engine_ids: vec![native_engine_id.clone()],
            protected_claim_eligible: true,
            boundary: "Local desktop fixture target.".into(),
        }],
        engines: vec![
            EngineRecord {
                id: native_engine_id,
                mode: EngineMode::Native,
                build_id: "engine-build-rspice-fixture-0.1.0+91e7c2a".into(),
                state: ClaimState::Current,
                declared_readiness: CapabilityReadinessStage::Registered,
                verification: VerificationState::Unverified,
                protected_claim_eligible: true,
                boundary: "Native engine requires independent production verification.".into(),
            },
            EngineRecord {
                id: preview_engine_id,
                mode: EngineMode::Preview,
                build_id: "engine-build-rspice-preview-fixture-0.1.0+91e7c2a".into(),
                state: ClaimState::Current,
                declared_readiness: CapabilityReadinessStage::Registered,
                verification: VerificationState::Unverified,
                protected_claim_eligible: false,
                boundary: "Preview engines cannot support protected product claims.".into(),
            },
        ],
        model_qualifications: vec![ModelQualification {
            id: model_binding_id,
            pdk_lock: "demo180@2.3.1".into(),
            set_digest: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
            qualification_evidence_id: "design-model-qualification".into(),
            state: ClaimState::Current,
            verification: VerificationState::Unverified,
            protected_claim_eligible: true,
            boundary: "Model binding requires independently signed qualification evidence.".into(),
        }],
        external_producers: vec![
            ExternalProducer {
                id: "external-producer-electrothermal".into(),
                external_capability_id: "electrothermal".into(),
                state: ClaimState::Current,
                verification: VerificationState::Unverified,
                protected_claim_eligible: true,
                boundary: "External producer requires independent qualification.".into(),
            },
            ExternalProducer {
                id: "external-producer-photonics".into(),
                external_capability_id: "photonics".into(),
                state: ClaimState::Current,
                verification: VerificationState::Unverified,
                protected_claim_eligible: true,
                boundary: "External producer requires independent qualification.".into(),
            },
        ],
        connectors: vec![ConnectorRecord {
            id: "connector-electrothermal".into(),
            external_capability_id: "electrothermal".into(),
            producer_id: "external-producer-electrothermal".into(),
            entitlement_policy_id: default_entitlement_id,
            state: ClaimState::Current,
            protected_claim_eligible: true,
            boundary: "Connector relationship is exact and entitlement governed.".into(),
        }],
        entitlements: vec![
            entitlement_record("entitlement-standard", EntitlementGrantState::Granted),
            entitlement_record("entitlement-expired", EntitlementGrantState::Expired),
            entitlement_record("entitlement-denied", EntitlementGrantState::Denied),
        ],
        claim_resolution_algorithm: canonical_algorithm(),
        evidence,
        resolution_cases,
    }
}

fn canonical_vocabulary() -> CapabilityVocabulary {
    CapabilityVocabulary {
        readiness_stages: CapabilityReadinessStage::ALL.into(),
        claim_states: vec![
            ClaimState::Current,
            ClaimState::Stale,
            ClaimState::Unavailable,
            ClaimState::Unknown,
        ],
        platform_ids: vec![
            PlatformId::Desktop,
            PlatformId::Browser,
            PlatformId::Tablet,
            PlatformId::Phone,
        ],
        compute_modes: vec![ComputeMode::Local, ComputeMode::Remote],
        engine_modes: vec![
            EngineMode::Native,
            EngineMode::Preview,
            EngineMode::Compatibility,
        ],
        claim_modes: vec![ClaimMode::Product, ClaimMode::ContractTestVector],
        subject_kinds: vec![
            SubjectKind::Design,
            SubjectKind::Build,
            SubjectKind::Engine,
            SubjectKind::Model,
            SubjectKind::Platform,
            SubjectKind::ExternalProducer,
            SubjectKind::Connector,
            SubjectKind::Release,
            SubjectKind::SignOff,
        ],
        evidence_kinds: vec![
            EvidenceKind::DesignContract,
            EvidenceKind::ImplementationConformance,
            EvidenceKind::EngineConformance,
            EvidenceKind::ModelQualification,
            EvidenceKind::PlatformQualification,
            EvidenceKind::ExternalProducerQualification,
            EvidenceKind::ReleaseQualification,
            EvidenceKind::SignOffApproval,
        ],
        verification_states: vec![
            VerificationState::Verified,
            VerificationState::Unverified,
            VerificationState::Rejected,
            VerificationState::Unknown,
        ],
        applicability_states: vec![
            ApplicabilityState::Applicable,
            ApplicabilityState::NotApplicable,
            ApplicabilityState::Unknown,
        ],
        entitlement_grant_states: vec![
            EntitlementGrantState::Granted,
            EntitlementGrantState::Denied,
            EntitlementGrantState::Expired,
            EntitlementGrantState::Revoked,
            EntitlementGrantState::Unknown,
        ],
        authority_classes: vec![
            AuthorityClass::DesignContract,
            AuthorityClass::ImplementationEvidence,
            AuthorityClass::ReleaseEvidence,
            AuthorityClass::ContractTestVector,
        ],
        protected_label_families: vec![
            ProtectedLabelFamily::Release,
            ProtectedLabelFamily::Qualified,
            ProtectedLabelFamily::SignOff,
        ],
    }
}

fn canonical_algorithm() -> ClaimResolutionAlgorithm {
    ClaimResolutionAlgorithm {
        version: CAPABILITY_RESOLVER_ALGORITHM_VERSION.into(),
        precedence: vec![
            ClaimState::Unavailable,
            ClaimState::Unknown,
            ClaimState::Stale,
            ClaimState::Current,
        ],
        target_selector_map: TargetSelectorMap {
            build_id: "buildIds".into(),
            platform_id: "platformIds".into(),
            compute_target_id: "computeTargetIds".into(),
            engine_id: "engineIds".into(),
            model_binding_id: "modelBindingIds".into(),
            external_producer_id: "externalProducerIds".into(),
            connector_id: "connectorIds".into(),
            entitlement_policy_id: "entitlementPolicyIds".into(),
            canonical_object_id: "canonicalObjectIds".into(),
        },
        stage_evidence_kinds: StageEvidenceKinds {
            registered: vec![],
            design_complete: vec![EvidenceKind::DesignContract],
            implementation_ready: vec![EvidenceKind::ImplementationConformance],
            engine_ready: vec![EvidenceKind::EngineConformance],
            platform_qualified: vec![EvidenceKind::PlatformQualification],
            sign_off_eligible: vec![EvidenceKind::SignOffApproval],
        },
        protected_label_policies: ProtectedLabelPolicies {
            release: ProtectedLabelPolicy {
                pattern: r"\brelease(?:d|[- ]ready)?\b".into(),
                minimum_stage_by_subject: uniform_stage(
                    CapabilityReadinessStage::PlatformQualified,
                ),
                required_evidence_kind_by_subject: uniform_evidence(
                    EvidenceKind::ReleaseQualification,
                ),
                product_authority_classes: vec![AuthorityClass::ReleaseEvidence],
            },
            qualified: ProtectedLabelPolicy {
                pattern: r"\bqualif(?:ied|ication)\b".into(),
                minimum_stage_by_subject: SubjectStageMap {
                    engine: Some(CapabilityReadinessStage::EngineReady),
                    model: Some(CapabilityReadinessStage::EngineReady),
                    external_producer: Some(CapabilityReadinessStage::EngineReady),
                    platform: Some(CapabilityReadinessStage::PlatformQualified),
                    connector: Some(CapabilityReadinessStage::PlatformQualified),
                    default: Some(CapabilityReadinessStage::PlatformQualified),
                },
                required_evidence_kind_by_subject: SubjectEvidenceMap {
                    engine: Some(EvidenceKind::EngineConformance),
                    model: Some(EvidenceKind::ModelQualification),
                    external_producer: Some(EvidenceKind::ExternalProducerQualification),
                    platform: Some(EvidenceKind::PlatformQualification),
                    connector: Some(EvidenceKind::ExternalProducerQualification),
                    default: Some(EvidenceKind::PlatformQualification),
                },
                product_authority_classes: vec![AuthorityClass::ReleaseEvidence],
            },
            sign_off: ProtectedLabelPolicy {
                pattern: r"\bsign[- ]?off\b".into(),
                minimum_stage_by_subject: uniform_stage(CapabilityReadinessStage::SignOffEligible),
                required_evidence_kind_by_subject: uniform_evidence(EvidenceKind::SignOffApproval),
                product_authority_classes: vec![AuthorityClass::ReleaseEvidence],
            },
        },
        steps: [
            "parse-protected-labels",
            "resolve-exact-targets",
            "apply-target-and-entitlement-state",
            "enforce-readiness-floor",
            "resolve-required-evidence",
            "verify-source-applicability-currentness",
            "apply-fail-closed-precedence",
            "return-auditable-result",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, id)| AlgorithmStep {
            order: u8::try_from(index + 1).expect("eight resolver steps fit in u8"),
            id: id.into(),
            rule: format!("Execute the canonical {id} rule."),
        })
        .collect(),
    }
}

fn uniform_stage(stage: CapabilityReadinessStage) -> SubjectStageMap {
    SubjectStageMap {
        engine: None,
        model: None,
        external_producer: None,
        platform: None,
        connector: None,
        default: Some(stage),
    }
}

fn uniform_evidence(kind: EvidenceKind) -> SubjectEvidenceMap {
    SubjectEvidenceMap {
        engine: None,
        model: None,
        external_producer: None,
        platform: None,
        connector: None,
        default: Some(kind),
    }
}

fn platform_record(
    id: PlatformId,
    state: ClaimState,
    protected_claim_eligible: bool,
) -> PlatformRecord {
    PlatformRecord {
        id,
        platform_task_contract_id: format!("platform-task-{id}"),
        state,
        qualification: "Design fixture only".into(),
        protected_claim_eligible,
        boundary: "Production qualification requires independent evidence.".into(),
    }
}

fn entitlement_record(id: &str, grant_state: EntitlementGrantState) -> EntitlementRecord {
    EntitlementRecord {
        id: id.into(),
        state: ClaimState::Current,
        grant_state,
        expires_at: Nullable::some(Timestamp::from("2027-01-01T00:00:00Z")),
        protected_claim_eligible: true,
        boundary: "Deterministic entitlement contract vector.".into(),
    }
}

fn engine_targets(base: &ClaimTargets, engine_id: &EngineId) -> ClaimTargets {
    let mut targets = base.clone();
    targets.engine_id = Nullable::some(engine_id.clone());
    targets
}

fn release_base_targets(
    base: &ClaimTargets,
    release_candidate_id: &CanonicalObjectId,
) -> ClaimTargets {
    let mut targets = base.clone();
    targets.canonical_object_id = Nullable::some(release_candidate_id.clone());
    targets
}

#[allow(clippy::too_many_arguments)]
fn evidence_record(
    id: &str,
    kind: EvidenceKind,
    authority_class: AuthorityClass,
    targets: &ClaimTargets,
    verification: VerificationState,
    currentness: ClaimState,
    status: ApplicabilityState,
    eligible_label_families: Vec<ProtectedLabelFamily>,
) -> EvidenceRecord {
    EvidenceRecord {
        id: id.into(),
        kind,
        source: EvidenceSource {
            authority_class,
            locator: format!("fixture://capability-readiness/{id}"),
            revision: "tracked-r1".into(),
        },
        verification,
        currentness,
        issued_at: Nullable::some(Timestamp::from("2026-07-01T00:00:00Z")),
        expires_at: Nullable::some(Timestamp::from("2026-08-13T10:00:00Z")),
        applicability: EvidenceApplicability {
            status,
            build_ids: targets.build_id.as_ref().cloned().into_iter().collect(),
            platform_ids: targets.platform_id.as_ref().copied().into_iter().collect(),
            compute_target_ids: targets
                .compute_target_id
                .as_ref()
                .cloned()
                .into_iter()
                .collect(),
            engine_ids: targets.engine_id.as_ref().cloned().into_iter().collect(),
            model_binding_ids: targets
                .model_binding_id
                .as_ref()
                .cloned()
                .into_iter()
                .collect(),
            external_producer_ids: targets
                .external_producer_id
                .as_ref()
                .cloned()
                .into_iter()
                .collect(),
            connector_ids: targets.connector_id.as_ref().cloned().into_iter().collect(),
            entitlement_policy_ids: targets
                .entitlement_policy_id
                .as_ref()
                .cloned()
                .into_iter()
                .collect(),
            canonical_object_ids: targets
                .canonical_object_id
                .as_ref()
                .cloned()
                .into_iter()
                .collect(),
        },
        supports_stages: vec![canonical_stage_for_evidence(kind)],
        eligible_label_families,
        boundary: "Deterministic resolver evidence; never a production trust source.".into(),
    }
}

#[allow(clippy::too_many_arguments)]
fn resolution_case(
    id: &str,
    claim_mode: ClaimMode,
    label: &str,
    subject_kind: SubjectKind,
    asserted_stage: CapabilityReadinessStage,
    targets: ClaimTargets,
    evidence_binding_ids: Vec<&str>,
    state: ClaimState,
    label_allowed: bool,
    required_reason_codes: Vec<ReasonCode>,
) -> ResolutionCase {
    ResolutionCase {
        id: id.into(),
        claim_mode,
        label: label.into(),
        subject_kind,
        asserted_stage,
        targets,
        evidence_binding_ids: evidence_binding_ids
            .into_iter()
            .map(EvidenceId::from)
            .collect(),
        expected: ExpectedResolution {
            state,
            label_allowed,
            required_reason_codes,
        },
    }
}
