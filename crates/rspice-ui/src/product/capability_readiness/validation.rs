//! What a production-evidence fixture must satisfy before it is trusted.
//!
//! Structure is checked exhaustively and up front — every vocabulary entry,
//! policy, stable id, and uniqueness constraint — because the resolver treats
//! a validated fixture as authoritative and does no further checking. A
//! fixture that fails any check is rejected whole rather than partially
//! accepted, so there is no state in which some claims resolve against a
//! fixture the rest of the file has already disqualified.

use super::*;

pub(super) fn validate_production_fixture(
    fixture: &CapabilityFixture,
) -> Result<(), ProductionEvidenceError> {
    if fixture.status != FixtureStatus::ProductionEvidence {
        return Err(ProductionEvidenceError::NonProductionFixture {
            fixture_id: fixture.fixture_id.clone(),
            actual_status: fixture.status,
        });
    }
    if fixture.schema != CAPABILITY_PRODUCTION_EVIDENCE_SCHEMA
        || fixture.schema_version != CAPABILITY_PRODUCTION_EVIDENCE_SCHEMA_VERSION
    {
        return Err(ProductionEvidenceError::InvalidSchema {
            fixture_id: fixture.fixture_id.clone(),
            actual_schema: fixture.schema.clone(),
            actual_version: fixture.schema_version.clone(),
        });
    }
    if fixture.claim_resolution_algorithm.version != CAPABILITY_RESOLVER_ALGORITHM_VERSION {
        return Err(ProductionEvidenceError::UnsupportedAlgorithm {
            fixture_id: fixture.fixture_id.clone(),
            actual_version: fixture.claim_resolution_algorithm.version.clone(),
        });
    }
    validate_exact_vocabulary_and_policy(fixture)?;
    validate_fixture_structure(fixture)
}

pub(super) fn validate_exact_vocabulary_and_policy(
    fixture: &CapabilityFixture,
) -> Result<(), ProductionEvidenceError> {
    let vocabulary = &fixture.vocabulary;
    let algorithm = &fixture.claim_resolution_algorithm;
    let exact_vocabulary = vocabulary.readiness_stages == CapabilityReadinessStage::ALL
        && vocabulary.claim_states
            == [
                ClaimState::Current,
                ClaimState::Stale,
                ClaimState::Unavailable,
                ClaimState::Unknown,
            ]
        && vocabulary.platform_ids
            == [
                PlatformId::Desktop,
                PlatformId::Browser,
                PlatformId::Tablet,
                PlatformId::Phone,
            ]
        && vocabulary.compute_modes == [ComputeMode::Local, ComputeMode::Remote]
        && vocabulary.engine_modes
            == [
                EngineMode::Native,
                EngineMode::Preview,
                EngineMode::Compatibility,
            ]
        && vocabulary.claim_modes == [ClaimMode::Product, ClaimMode::ContractTestVector]
        && vocabulary.subject_kinds
            == [
                SubjectKind::Design,
                SubjectKind::Build,
                SubjectKind::Engine,
                SubjectKind::Model,
                SubjectKind::Platform,
                SubjectKind::ExternalProducer,
                SubjectKind::Connector,
                SubjectKind::Release,
                SubjectKind::SignOff,
            ]
        && vocabulary.evidence_kinds
            == [
                EvidenceKind::DesignContract,
                EvidenceKind::ImplementationConformance,
                EvidenceKind::EngineConformance,
                EvidenceKind::ModelQualification,
                EvidenceKind::PlatformQualification,
                EvidenceKind::ExternalProducerQualification,
                EvidenceKind::ReleaseQualification,
                EvidenceKind::SignOffApproval,
            ]
        && vocabulary.verification_states
            == [
                VerificationState::Verified,
                VerificationState::Unverified,
                VerificationState::Rejected,
                VerificationState::Unknown,
            ]
        && vocabulary.applicability_states
            == [
                ApplicabilityState::Applicable,
                ApplicabilityState::NotApplicable,
                ApplicabilityState::Unknown,
            ]
        && vocabulary.entitlement_grant_states
            == [
                EntitlementGrantState::Granted,
                EntitlementGrantState::Denied,
                EntitlementGrantState::Expired,
                EntitlementGrantState::Revoked,
                EntitlementGrantState::Unknown,
            ]
        && vocabulary.authority_classes
            == [
                AuthorityClass::DesignContract,
                AuthorityClass::ImplementationEvidence,
                AuthorityClass::ReleaseEvidence,
                AuthorityClass::ContractTestVector,
            ]
        && vocabulary.protected_label_families
            == [
                ProtectedLabelFamily::Release,
                ProtectedLabelFamily::Qualified,
                ProtectedLabelFamily::SignOff,
            ]
        && algorithm.precedence
            == [
                ClaimState::Unavailable,
                ClaimState::Unknown,
                ClaimState::Stale,
                ClaimState::Current,
            ];
    if !exact_vocabulary {
        return Err(ProductionEvidenceError::InvalidReadinessVocabulary {
            fixture_id: fixture.fixture_id.clone(),
        });
    }

    let selectors = &algorithm.target_selector_map;
    let exact_selectors = selectors.build_id == "buildIds"
        && selectors.platform_id == "platformIds"
        && selectors.compute_target_id == "computeTargetIds"
        && selectors.engine_id == "engineIds"
        && selectors.model_binding_id == "modelBindingIds"
        && selectors.external_producer_id == "externalProducerIds"
        && selectors.connector_id == "connectorIds"
        && selectors.entitlement_policy_id == "entitlementPolicyIds"
        && selectors.canonical_object_id == "canonicalObjectIds";
    let stages = &algorithm.stage_evidence_kinds;
    let exact_stages = stages.registered.is_empty()
        && stages.design_complete == [EvidenceKind::DesignContract]
        && stages.implementation_ready == [EvidenceKind::ImplementationConformance]
        && stages.engine_ready == [EvidenceKind::EngineConformance]
        && stages.platform_qualified == [EvidenceKind::PlatformQualification]
        && stages.sign_off_eligible == [EvidenceKind::SignOffApproval];
    if !exact_selectors || !exact_stages {
        return invalid_structure(
            fixture,
            "resolver selector or stage-evidence policy differs from version 1.0.0",
        );
    }

    for family in [
        ProtectedLabelFamily::Release,
        ProtectedLabelFamily::Qualified,
        ProtectedLabelFamily::SignOff,
    ] {
        let policy = algorithm.protected_label_policies.get(family);
        let expected_pattern = match family {
            ProtectedLabelFamily::Release => r"\brelease(?:d|[- ]ready)?\b",
            ProtectedLabelFamily::Qualified => r"\bqualif(?:ied|ication)\b",
            ProtectedLabelFamily::SignOff => r"\bsign[- ]?off\b",
        };
        if policy.pattern != expected_pattern
            || policy.product_authority_classes != [AuthorityClass::ReleaseEvidence]
        {
            return invalid_structure(
                fixture,
                format!("protected {family:?} policy is not canonical"),
            );
        }
        for subject in [
            SubjectKind::Design,
            SubjectKind::Build,
            SubjectKind::Engine,
            SubjectKind::Model,
            SubjectKind::Platform,
            SubjectKind::ExternalProducer,
            SubjectKind::Connector,
            SubjectKind::Release,
            SubjectKind::SignOff,
        ] {
            if configured_subject_stage(&policy.minimum_stage_by_subject, subject)
                != Some(canonical_minimum_stage(family, subject))
                || configured_subject_evidence(&policy.required_evidence_kind_by_subject, subject)
                    != Some(canonical_protected_evidence(family, subject))
            {
                return invalid_structure(
                    fixture,
                    format!("protected {family:?} policy is incomplete for {subject:?}"),
                );
            }
        }
    }

    let expected_steps = [
        "parse-protected-labels",
        "resolve-exact-targets",
        "apply-target-and-entitlement-state",
        "enforce-readiness-floor",
        "resolve-required-evidence",
        "verify-source-applicability-currentness",
        "apply-fail-closed-precedence",
        "return-auditable-result",
    ];
    if algorithm.steps.len() != expected_steps.len()
        || algorithm.steps.iter().zip(expected_steps).enumerate().any(
            |(index, (step, expected_id))| {
                usize::from(step.order) != index + 1
                    || step.id != expected_id
                    || step.rule.trim().is_empty()
            },
        )
    {
        return invalid_structure(
            fixture,
            "resolver steps are not the exact ordered 1.0.0 contract",
        );
    }
    Ok(())
}

pub(super) fn configured_subject_stage(
    map: &SubjectStageMap,
    subject: SubjectKind,
) -> Option<CapabilityReadinessStage> {
    match subject {
        SubjectKind::Engine => map.engine,
        SubjectKind::Model => map.model,
        SubjectKind::ExternalProducer => map.external_producer,
        SubjectKind::Platform => map.platform,
        SubjectKind::Connector => map.connector,
        _ => None,
    }
    .or(map.default)
}

pub(super) fn configured_subject_evidence(
    map: &SubjectEvidenceMap,
    subject: SubjectKind,
) -> Option<EvidenceKind> {
    match subject {
        SubjectKind::Engine => map.engine,
        SubjectKind::Model => map.model,
        SubjectKind::ExternalProducer => map.external_producer,
        SubjectKind::Platform => map.platform,
        SubjectKind::Connector => map.connector,
        _ => None,
    }
    .or(map.default)
}

pub(super) fn validate_fixture_structure(
    fixture: &CapabilityFixture,
) -> Result<(), ProductionEvidenceError> {
    if !is_stable_id(&fixture.fixture_id) || fixture.fixture_revision.trim().is_empty() {
        return invalid_structure(fixture, "fixture identity is empty or non-canonical");
    }
    if fixture.as_of.unix_seconds().is_none() {
        return invalid_structure(fixture, "fixture asOf is not a valid RFC 3339 timestamp");
    }
    if fixture.build_identity.artifact_kind != "product-build" {
        return invalid_structure(fixture, "buildIdentity.artifactKind must be product-build");
    }
    for (name, value) in [
        ("buildIdentity.id", fixture.build_identity.id.as_str()),
        (
            "buildIdentity.engineBuildFixtureId",
            fixture.build_identity.engine_build_fixture_id.as_str(),
        ),
    ] {
        ensure_stable_id(fixture, name, value)?;
    }
    for (name, value) in [
        (
            "buildIdentity.packageName",
            fixture.build_identity.package_name.as_str(),
        ),
        (
            "buildIdentity.packageVersion",
            fixture.build_identity.package_version.as_str(),
        ),
        (
            "buildIdentity.fixtureRevision",
            fixture.build_identity.fixture_revision.as_str(),
        ),
        (
            "buildIdentity.productFixtureId",
            fixture.build_identity.product_fixture_id.as_str(),
        ),
        (
            "buildIdentity.productFixtureSchemaVersion",
            fixture
                .build_identity
                .product_fixture_schema_version
                .as_str(),
        ),
        (
            "buildIdentity.canonicalInputRevision",
            fixture.build_identity.canonical_input_revision.as_str(),
        ),
    ] {
        ensure_nonempty(fixture, name, value)?;
    }
    if !fixture.resolution_cases.is_empty() {
        return invalid_structure(
            fixture,
            "production evidence must not contain design or contract-test resolution cases",
        );
    }

    let canonical_ids = fixture.canonical_object_ids.values();
    ensure_unique(fixture, "canonicalObjectIds", canonical_ids.iter().copied())?;
    for id in canonical_ids {
        ensure_stable_id(fixture, "canonicalObjectIds", id.as_str())?;
    }

    for (expected, record) in [
        (PlatformId::Desktop, &fixture.platform_targets.desktop),
        (PlatformId::Browser, &fixture.platform_targets.browser),
        (PlatformId::Tablet, &fixture.platform_targets.tablet),
        (PlatformId::Phone, &fixture.platform_targets.phone),
    ] {
        if record.id != expected {
            return invalid_structure(
                fixture,
                format!(
                    "platform target key {expected} contains record {}",
                    record.id
                ),
            );
        }
        ensure_nonempty(
            fixture,
            "platformTargets.platformTaskContractId",
            &record.platform_task_contract_id,
        )?;
    }

    ensure_unique(
        fixture,
        "computeTargets.id",
        fixture.compute_targets.iter().map(|record| &record.id),
    )?;
    ensure_unique(
        fixture,
        "engines.id",
        fixture.engines.iter().map(|record| &record.id),
    )?;
    ensure_unique(
        fixture,
        "modelQualifications.id",
        fixture.model_qualifications.iter().map(|record| &record.id),
    )?;
    ensure_unique(
        fixture,
        "externalProducers.id",
        fixture.external_producers.iter().map(|record| &record.id),
    )?;
    ensure_unique(
        fixture,
        "connectors.id",
        fixture.connectors.iter().map(|record| &record.id),
    )?;
    ensure_unique(
        fixture,
        "entitlements.id",
        fixture.entitlements.iter().map(|record| &record.id),
    )?;
    ensure_unique(
        fixture,
        "evidence.id",
        fixture.evidence.iter().map(|record| &record.id),
    )?;

    let engine_ids = fixture
        .engines
        .iter()
        .map(|record| record.id.clone())
        .collect::<HashSet<_>>();
    for target in &fixture.compute_targets {
        ensure_stable_id(fixture, "computeTargets.id", target.id.as_str())?;
        ensure_unique(
            fixture,
            "computeTargets.engineIds",
            target.engine_ids.iter(),
        )?;
        for engine_id in &target.engine_ids {
            if !engine_ids.contains(engine_id) {
                return invalid_structure(
                    fixture,
                    format!(
                        "compute target {} references unknown engine {engine_id}",
                        target.id
                    ),
                );
            }
        }
    }
    for engine in &fixture.engines {
        ensure_stable_id(fixture, "engines.id", engine.id.as_str())?;
        ensure_stable_id(fixture, "engines.buildId", engine.build_id.as_str())?;
    }
    for model in &fixture.model_qualifications {
        ensure_stable_id(fixture, "modelQualifications.id", model.id.as_str())?;
        ensure_stable_id(
            fixture,
            "modelQualifications.qualificationEvidenceId",
            &model.qualification_evidence_id,
        )?;
        if model.set_digest.len() != 64
            || !model
                .set_digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return invalid_structure(
                fixture,
                format!("model {} has an invalid set digest", model.id),
            );
        }
    }

    let producer_ids = fixture
        .external_producers
        .iter()
        .map(|record| record.id.clone())
        .collect::<HashSet<_>>();
    let entitlement_ids = fixture
        .entitlements
        .iter()
        .map(|record| record.id.clone())
        .collect::<HashSet<_>>();
    for producer in &fixture.external_producers {
        ensure_stable_id(fixture, "externalProducers.id", producer.id.as_str())?;
        if !matches!(
            producer.external_capability_id.as_str(),
            "photonics" | "em" | "em-ir" | "electrothermal"
        ) {
            return invalid_structure(
                fixture,
                format!(
                    "producer {} has unknown external capability {}",
                    producer.id, producer.external_capability_id
                ),
            );
        }
    }
    for connector in &fixture.connectors {
        ensure_stable_id(fixture, "connectors.id", connector.id.as_str())?;
        if !producer_ids.contains(&connector.producer_id)
            || !entitlement_ids.contains(&connector.entitlement_policy_id)
        {
            return invalid_structure(
                fixture,
                format!(
                    "connector {} has a dangling producer or entitlement",
                    connector.id
                ),
            );
        }
        let Some(producer) = fixture
            .external_producers
            .iter()
            .find(|record| record.id == connector.producer_id)
        else {
            return invalid_structure(
                fixture,
                format!("connector {} references an unknown producer", connector.id),
            );
        };
        if producer.external_capability_id != connector.external_capability_id {
            return invalid_structure(
                fixture,
                format!(
                    "connector {} capability disagrees with its producer",
                    connector.id
                ),
            );
        }
    }
    for entitlement in &fixture.entitlements {
        ensure_stable_id(fixture, "entitlements.id", entitlement.id.as_str())?;
        if timestamp_invalid(entitlement.expires_at.as_ref()) {
            return invalid_structure(
                fixture,
                format!("entitlement {} has an invalid expiry", entitlement.id),
            );
        }
    }

    let compute_ids = fixture
        .compute_targets
        .iter()
        .map(|record| record.id.clone())
        .collect::<HashSet<_>>();
    let model_ids = fixture
        .model_qualifications
        .iter()
        .map(|record| record.id.clone())
        .collect::<HashSet<_>>();
    let connector_ids = fixture
        .connectors
        .iter()
        .map(|record| record.id.clone())
        .collect::<HashSet<_>>();
    let canonical_ids = fixture
        .canonical_object_ids
        .values()
        .into_iter()
        .cloned()
        .collect::<HashSet<_>>();
    for evidence in &fixture.evidence {
        ensure_stable_id(fixture, "evidence.id", evidence.id.as_str())?;
        ensure_nonempty(fixture, "evidence.source.locator", &evidence.source.locator)?;
        ensure_nonempty(
            fixture,
            "evidence.source.revision",
            &evidence.source.revision,
        )?;
        if evidence.source.authority_class == AuthorityClass::ContractTestVector {
            return invalid_structure(
                fixture,
                format!(
                    "production evidence {} uses contract-test-vector authority",
                    evidence.id
                ),
            );
        }
        if evidence.supports_stages.is_empty() {
            return invalid_structure(
                fixture,
                format!("evidence {} supports no stage", evidence.id),
            );
        }
        ensure_unique(
            fixture,
            "evidence.supportsStages",
            evidence.supports_stages.iter(),
        )?;
        ensure_unique(
            fixture,
            "evidence.eligibleLabelFamilies",
            evidence.eligible_label_families.iter(),
        )?;
        let applicability = &evidence.applicability;
        ensure_unique(fixture, "evidence.buildIds", applicability.build_ids.iter())?;
        ensure_unique(
            fixture,
            "evidence.platformIds",
            applicability.platform_ids.iter(),
        )?;
        ensure_unique(
            fixture,
            "evidence.computeTargetIds",
            applicability.compute_target_ids.iter(),
        )?;
        ensure_unique(
            fixture,
            "evidence.engineIds",
            applicability.engine_ids.iter(),
        )?;
        ensure_unique(
            fixture,
            "evidence.modelBindingIds",
            applicability.model_binding_ids.iter(),
        )?;
        ensure_unique(
            fixture,
            "evidence.externalProducerIds",
            applicability.external_producer_ids.iter(),
        )?;
        ensure_unique(
            fixture,
            "evidence.connectorIds",
            applicability.connector_ids.iter(),
        )?;
        ensure_unique(
            fixture,
            "evidence.entitlementPolicyIds",
            applicability.entitlement_policy_ids.iter(),
        )?;
        ensure_unique(
            fixture,
            "evidence.canonicalObjectIds",
            applicability.canonical_object_ids.iter(),
        )?;
        if applicability
            .build_ids
            .iter()
            .any(|id| id != &fixture.build_identity.id)
            || applicability
                .platform_ids
                .iter()
                .any(|id| !vocabulary_platform_contains(fixture, *id))
            || applicability
                .compute_target_ids
                .iter()
                .any(|id| !compute_ids.contains(id))
            || applicability
                .engine_ids
                .iter()
                .any(|id| !engine_ids.contains(id))
            || applicability
                .model_binding_ids
                .iter()
                .any(|id| !model_ids.contains(id))
            || applicability
                .external_producer_ids
                .iter()
                .any(|id| !producer_ids.contains(id))
            || applicability
                .connector_ids
                .iter()
                .any(|id| !connector_ids.contains(id))
            || applicability
                .entitlement_policy_ids
                .iter()
                .any(|id| !entitlement_ids.contains(id))
            || applicability
                .canonical_object_ids
                .iter()
                .any(|id| !canonical_ids.contains(id))
        {
            return invalid_structure(
                fixture,
                format!(
                    "evidence {} contains a dangling applicability selector",
                    evidence.id
                ),
            );
        }
    }
    Ok(())
}

pub(super) fn vocabulary_platform_contains(fixture: &CapabilityFixture, id: PlatformId) -> bool {
    fixture.vocabulary.platform_ids.contains(&id)
}

pub(super) fn ensure_nonempty(
    fixture: &CapabilityFixture,
    field: &str,
    value: &str,
) -> Result<(), ProductionEvidenceError> {
    if value.trim().is_empty() {
        invalid_structure(fixture, format!("{field} must not be empty"))
    } else {
        Ok(())
    }
}

pub(super) fn ensure_stable_id(
    fixture: &CapabilityFixture,
    field: &str,
    value: &str,
) -> Result<(), ProductionEvidenceError> {
    if is_stable_id(value) {
        Ok(())
    } else {
        invalid_structure(
            fixture,
            format!("{field} contains non-canonical ID `{value}`"),
        )
    }
}

pub(super) fn is_stable_id(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphanumeric())
        && bytes.all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'.' | b'_' | b':' | b'@' | b'+' | b'/' | b'-')
        })
}

pub(super) fn ensure_unique<'a, T: Eq + Hash + fmt::Debug + 'a>(
    fixture: &CapabilityFixture,
    field: &str,
    values: impl IntoIterator<Item = &'a T>,
) -> Result<(), ProductionEvidenceError> {
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(value) {
            return invalid_structure(
                fixture,
                format!("{field} contains duplicate value `{value:?}`"),
            );
        }
    }
    Ok(())
}

pub(super) fn invalid_structure<T>(
    fixture: &CapabilityFixture,
    detail: impl Into<String>,
) -> Result<T, ProductionEvidenceError> {
    Err(ProductionEvidenceError::InvalidStructure {
        fixture_id: fixture.fixture_id.clone(),
        detail: detail.into(),
    })
}
