//! Resolving one capability claim against the evidence that supports it.
//!
//! Resolution is deny-by-default: a claim becomes affirmative only when the
//! required evidence kinds are all present, applicable, unexpired, and bound
//! to an eligible authority. Every negative outcome carries a reason code, so
//! an unmet claim always explains which requirement it failed rather than
//! silently reading as unsupported.

use super::*;

pub(super) fn resolve_capability_claim(
    fixture: Option<&CapabilityFixture>,
    claim: &CapabilityClaim,
    evaluated_at: Timestamp,
) -> ResolvedClaim {
    let input_error = claim_input_error(claim);
    let protected_families = if input_error.is_none() {
        classify_protected_labels(&claim.label)
    } else {
        Vec::new()
    };
    let production_evidence =
        fixture.is_some_and(|fixture| fixture.status == FixtureStatus::ProductionEvidence);
    let requirements = required_evidence(claim, &protected_families, production_evidence);
    let required_evidence_kinds = unique_required_kinds(&requirements);
    let Some(fixture) = fixture else {
        let reason = ResolutionReason {
            state: ClaimState::Unknown,
            code: ReasonCode::EvidenceProviderUnavailable,
            detail: "no production capability evidence provider is configured".into(),
        };
        return resolved_claim(
            claim,
            ResolutionParts {
                as_of: Nullable::none(),
                state: ClaimState::Unknown,
                protected_label_families: protected_families,
                required_evidence_kinds,
                source: Vec::new(),
                reasons: vec![reason],
            },
        );
    };

    let mut reasons = Vec::new();
    let as_of = evaluated_at;
    let as_of_seconds = as_of.unix_seconds();
    if as_of_seconds.is_none() {
        add_reason(
            &mut reasons,
            ClaimState::Unknown,
            ReasonCode::InvalidAsOf,
            as_of.to_string(),
        );
    }
    if let Some(detail) = input_error {
        add_reason(
            &mut reasons,
            ClaimState::Unknown,
            ReasonCode::ClaimInputTooLarge,
            detail,
        );
        return resolved_claim(
            claim,
            ResolutionParts {
                as_of: Nullable::some(as_of),
                state: ClaimState::Unknown,
                protected_label_families: protected_families,
                required_evidence_kinds,
                source: Vec::new(),
                reasons,
            },
        );
    }

    evaluate_claim_contract(claim, &protected_families, &mut reasons);
    enforce_readiness_floors(claim, &protected_families, &mut reasons);
    evaluate_targets(
        fixture,
        claim,
        !protected_families.is_empty(),
        as_of_seconds,
        &mut reasons,
    );

    let bound = claim
        .evidence_binding_ids
        .iter()
        .map(|id| (id, fixture.evidence.iter().find(|record| &record.id == id)))
        .collect::<Vec<_>>();
    for (id, evidence) in &bound {
        if evidence.is_none() {
            add_reason(
                &mut reasons,
                ClaimState::Unknown,
                ReasonCode::EvidenceReferenceUnknown,
                id.to_string(),
            );
        }
    }

    let mut selected = Vec::<EvidenceId>::new();
    for requirement in &requirements {
        let matching = bound
            .iter()
            .filter_map(|(_, evidence)| evidence.filter(|record| record.kind == requirement.kind))
            .collect::<Vec<_>>();
        if matching.is_empty() {
            add_reason(
                &mut reasons,
                ClaimState::Unknown,
                ReasonCode::MissingRequiredEvidence,
                format!(
                    "{}:{}",
                    requirement
                        .family
                        .map_or_else(|| stage_name(claim.asserted_stage), protected_family_name),
                    evidence_kind_name(requirement.kind)
                ),
            );
            continue;
        }
        for evidence in matching {
            if !selected.contains(&evidence.id) {
                selected.push(evidence.id.clone());
            }
            evaluate_evidence(
                fixture,
                claim,
                evidence,
                *requirement,
                as_of_seconds,
                &mut reasons,
            );
        }
    }

    let source = bound
        .into_iter()
        .map(|(requested_id, evidence)| binding_summary(requested_id, evidence, &selected))
        .collect();
    let state = final_state(&reasons);
    resolved_claim(
        claim,
        ResolutionParts {
            as_of: Nullable::some(as_of),
            state,
            protected_label_families: protected_families,
            required_evidence_kinds,
            source,
            reasons,
        },
    )
}

pub(super) fn claim_input_error(claim: &CapabilityClaim) -> Option<String> {
    if claim.label.len() > MAX_CAPABILITY_CLAIM_LABEL_BYTES {
        return Some(format!(
            "label:{}:{}",
            claim.label.len(),
            MAX_CAPABILITY_CLAIM_LABEL_BYTES
        ));
    }
    if claim.evidence_binding_ids.len() > MAX_CAPABILITY_CLAIM_EVIDENCE_BINDINGS {
        return Some(format!(
            "evidenceBindingIds:{}:{}",
            claim.evidence_binding_ids.len(),
            MAX_CAPABILITY_CLAIM_EVIDENCE_BINDINGS
        ));
    }
    let mut identifiers = Vec::with_capacity(claim.evidence_binding_ids.len() + 9);
    if let Some(id) = claim.id.as_ref() {
        identifiers.push(("claimId", id.as_str()));
    }
    identifiers.extend(
        claim
            .evidence_binding_ids
            .iter()
            .map(|id| ("evidenceBindingId", id.as_str())),
    );
    macro_rules! optional_id {
        ($field:ident, $name:literal) => {
            if let Some(id) = claim.targets.$field.as_ref() {
                identifiers.push(($name, id.as_str()));
            }
        };
    }
    optional_id!(build_id, "buildId");
    optional_id!(compute_target_id, "computeTargetId");
    optional_id!(engine_id, "engineId");
    optional_id!(model_binding_id, "modelBindingId");
    optional_id!(external_producer_id, "externalProducerId");
    optional_id!(connector_id, "connectorId");
    optional_id!(entitlement_policy_id, "entitlementPolicyId");
    optional_id!(canonical_object_id, "canonicalObjectId");
    identifiers
        .into_iter()
        .find(|(_, value)| value.len() > MAX_CAPABILITY_CLAIM_IDENTIFIER_BYTES)
        .map(|(name, value)| {
            format!(
                "{name}:{}:{}",
                value.len(),
                MAX_CAPABILITY_CLAIM_IDENTIFIER_BYTES
            )
        })
}

pub(super) struct ResolutionParts {
    as_of: Nullable<Timestamp>,
    state: ClaimState,
    protected_label_families: Vec<ProtectedLabelFamily>,
    required_evidence_kinds: Vec<EvidenceKind>,
    source: Vec<EvidenceBindingSummary>,
    reasons: Vec<ResolutionReason>,
}

pub(super) fn resolved_claim(claim: &CapabilityClaim, parts: ResolutionParts) -> ResolvedClaim {
    let ResolutionParts {
        as_of,
        state,
        protected_label_families,
        required_evidence_kinds,
        source,
        reasons,
    } = parts;
    let reason_codes =
        reasons
            .iter()
            .map(|reason| reason.code)
            .fold(Vec::new(), |mut codes, code| {
                if !codes.contains(&code) {
                    codes.push(code);
                }
                codes
            });
    ResolvedClaim {
        case_id: claim.id.clone(),
        algorithm_version: CAPABILITY_RESOLVER_ALGORITHM_VERSION.into(),
        as_of: as_of.clone(),
        claim_mode: claim.claim_mode,
        label: claim.label.clone(),
        subject_kind: claim.subject_kind,
        asserted_stage: claim.asserted_stage,
        state,
        label_allowed: claim.claim_mode == ClaimMode::Product && state == ClaimState::Current,
        protected_label_families,
        required_evidence_kinds,
        source,
        applicability: ResolvedApplicability {
            targets: claim.targets.clone(),
            exact_target_binding_required: true,
        },
        currentness: ResolvedCurrentness {
            evaluated_at: as_of,
            state,
        },
        reason_codes,
        reasons,
    }
}

pub(super) fn required_evidence(
    claim: &CapabilityClaim,
    protected: &[ProtectedLabelFamily],
    production_evidence: bool,
) -> Vec<EvidenceRequirement> {
    let mut requirements = if production_evidence || protected.is_empty() {
        canonical_stage_evidence(claim.asserted_stage, production_evidence)
            .iter()
            .copied()
            .map(|kind| EvidenceRequirement { family: None, kind })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    if production_evidence
        && claim.targets.model_binding_id.as_ref().is_some()
        && claim
            .asserted_stage
            .satisfies(CapabilityReadinessStage::EngineReady)
        && !requirements
            .iter()
            .any(|requirement| requirement.kind == EvidenceKind::ModelQualification)
    {
        requirements.push(EvidenceRequirement {
            family: None,
            kind: EvidenceKind::ModelQualification,
        });
    }
    if protected.is_empty() {
        return requirements;
    }

    requirements.extend(protected.iter().map(|family| EvidenceRequirement {
        family: Some(*family),
        kind: canonical_protected_evidence(*family, claim.subject_kind),
    }));
    requirements
}

pub(super) fn evaluate_claim_contract(
    claim: &CapabilityClaim,
    protected: &[ProtectedLabelFamily],
    reasons: &mut Vec<ResolutionReason>,
) {
    if claim.claim_mode == ClaimMode::Product {
        if claim.asserted_stage == CapabilityReadinessStage::Registered {
            add_reason(
                reasons,
                ClaimState::Unknown,
                ReasonCode::RegisteredProductClaimForbidden,
                claim.label.clone(),
            );
        }
        match governed_product_label_minimum(claim) {
            Some(minimum) if protected.is_empty() && !claim.asserted_stage.satisfies(minimum) => {
                add_reason(
                    reasons,
                    ClaimState::Unknown,
                    ReasonCode::ReadinessStageInsufficient,
                    format!(
                        "governed-label:{}:{}",
                        stage_name(claim.asserted_stage),
                        stage_name(minimum)
                    ),
                );
            }
            Some(_) => {}
            None => add_reason(
                reasons,
                ClaimState::Unknown,
                ReasonCode::UngovernedProductLabel,
                claim.label.clone(),
            ),
        }
    }

    for (missing, selector) in [
        (claim.targets.build_id.as_ref().is_none(), "buildId"),
        (
            claim.targets.entitlement_policy_id.as_ref().is_none(),
            "entitlementPolicyId",
        ),
        (
            claim.targets.canonical_object_id.as_ref().is_none(),
            "canonicalObjectId",
        ),
    ] {
        if missing {
            add_reason(
                reasons,
                ClaimState::Unknown,
                ReasonCode::RequiredTargetMissing,
                selector.into(),
            );
        }
    }

    let subject_selectors: &[(&str, bool)] = match claim.subject_kind {
        SubjectKind::Engine => &[("engineId", claim.targets.engine_id.as_ref().is_some())],
        SubjectKind::Model => &[(
            "modelBindingId",
            claim.targets.model_binding_id.as_ref().is_some(),
        )],
        SubjectKind::Platform => &[("platformId", claim.targets.platform_id.as_ref().is_some())],
        SubjectKind::ExternalProducer => &[(
            "externalProducerId",
            claim.targets.external_producer_id.as_ref().is_some(),
        )],
        SubjectKind::Connector => &[
            ("connectorId", claim.targets.connector_id.as_ref().is_some()),
            (
                "externalProducerId",
                claim.targets.external_producer_id.as_ref().is_some(),
            ),
        ],
        SubjectKind::SignOff => &[
            ("platformId", claim.targets.platform_id.as_ref().is_some()),
            (
                "computeTargetId",
                claim.targets.compute_target_id.as_ref().is_some(),
            ),
            ("engineId", claim.targets.engine_id.as_ref().is_some()),
            (
                "modelBindingId",
                claim.targets.model_binding_id.as_ref().is_some(),
            ),
        ],
        SubjectKind::Design | SubjectKind::Build | SubjectKind::Release => &[],
    };
    for (selector, present) in subject_selectors {
        if !present {
            add_reason(
                reasons,
                ClaimState::Unknown,
                ReasonCode::RequiredTargetMissing,
                (*selector).into(),
            );
        }
    }

    if claim
        .asserted_stage
        .satisfies(CapabilityReadinessStage::PlatformQualified)
    {
        for (selector, present) in [
            ("platformId", claim.targets.platform_id.as_ref().is_some()),
            (
                "computeTargetId",
                claim.targets.compute_target_id.as_ref().is_some(),
            ),
            ("engineId", claim.targets.engine_id.as_ref().is_some()),
            (
                "modelBindingId",
                claim.targets.model_binding_id.as_ref().is_some(),
            ),
        ] {
            if !present {
                add_reason(
                    reasons,
                    ClaimState::Unknown,
                    ReasonCode::RequiredTargetMissing,
                    selector.into(),
                );
            }
        }
    }
}

pub(super) fn governed_product_label_minimum(
    claim: &CapabilityClaim,
) -> Option<CapabilityReadinessStage> {
    let label = claim.label.trim().to_ascii_lowercase();
    match (claim.subject_kind, label.as_str()) {
        (SubjectKind::Design, "design complete") => Some(CapabilityReadinessStage::DesignComplete),
        (SubjectKind::Build, "implementation ready") => {
            Some(CapabilityReadinessStage::ImplementationReady)
        }
        (SubjectKind::Engine, "engine ready" | "engine qualified")
        | (SubjectKind::Model, "model qualified")
        | (SubjectKind::ExternalProducer, "external producer qualified") => {
            Some(CapabilityReadinessStage::EngineReady)
        }
        (SubjectKind::Platform, "platform qualified")
        | (SubjectKind::Connector, "connector qualified")
        | (SubjectKind::Release, "release ready" | "production release ready" | "released") => {
            Some(CapabilityReadinessStage::PlatformQualified)
        }
        (SubjectKind::SignOff, "sign-off eligible" | "signoff eligible" | "sign off eligible") => {
            Some(CapabilityReadinessStage::SignOffEligible)
        }
        _ => None,
    }
}

pub(super) fn enforce_readiness_floors(
    claim: &CapabilityClaim,
    protected: &[ProtectedLabelFamily],
    reasons: &mut Vec<ResolutionReason>,
) {
    for family in protected {
        let minimum = canonical_minimum_stage(*family, claim.subject_kind);
        if !claim.asserted_stage.satisfies(minimum) {
            add_reason(
                reasons,
                ClaimState::Unknown,
                ReasonCode::ReadinessStageInsufficient,
                format!(
                    "{}:{}:{}",
                    protected_family_name(*family),
                    stage_name(claim.asserted_stage),
                    stage_name(minimum)
                ),
            );
        }
    }
}

pub(super) fn evaluate_targets(
    fixture: &CapabilityFixture,
    claim: &CapabilityClaim,
    is_protected: bool,
    as_of: Option<i64>,
    reasons: &mut Vec<ResolutionReason>,
) {
    if let Some(id) = claim.targets.build_id.as_ref() {
        if fixture.build_identity.id == *id {
            evaluate_record_state(
                fixture.build_identity.state,
                fixture.build_identity.protected_claim_eligible,
                "buildId",
                id,
                is_protected,
                reasons,
            );
        } else {
            unknown_target("buildId", id, reasons);
        }
    }
    if let Some(id) = claim.targets.platform_id.as_ref() {
        let record = fixture.platform_targets.get(*id);
        evaluate_record_state(
            record.state,
            record.protected_claim_eligible,
            "platformId",
            id,
            is_protected,
            reasons,
        );
    }
    if let Some(id) = claim.targets.compute_target_id.as_ref() {
        if let Some(record) = fixture
            .compute_targets
            .iter()
            .find(|record| &record.id == id)
        {
            evaluate_record_state(
                record.state,
                record.protected_claim_eligible,
                "computeTargetId",
                id,
                is_protected,
                reasons,
            );
        } else {
            unknown_target("computeTargetId", id, reasons);
        }
    }
    if let Some(id) = claim.targets.engine_id.as_ref() {
        if let Some(record) = fixture.engines.iter().find(|record| &record.id == id) {
            evaluate_record_state(
                record.state,
                record.protected_claim_eligible,
                "engineId",
                id,
                is_protected,
                reasons,
            );
            if fixture.status == FixtureStatus::ProductionEvidence {
                evaluate_target_verification(record.verification, "engineId", id, reasons);
                if claim.claim_mode == ClaimMode::Product
                    && claim
                        .asserted_stage
                        .satisfies(CapabilityReadinessStage::EngineReady)
                    && record.mode != EngineMode::Native
                {
                    add_reason(
                        reasons,
                        ClaimState::Unavailable,
                        ReasonCode::TargetEngineModeIneligible,
                        format!("engineId:{id}:{:?}", record.mode),
                    );
                }
                if record.mode == EngineMode::Native
                    && record.build_id != fixture.build_identity.engine_build_fixture_id
                {
                    add_reason(
                        reasons,
                        ClaimState::Unavailable,
                        ReasonCode::TargetRelationshipMismatch,
                        format!(
                            "engineId:{id}:buildId:{}:{}",
                            record.build_id, fixture.build_identity.engine_build_fixture_id
                        ),
                    );
                }
                let minimum = match claim.asserted_stage {
                    CapabilityReadinessStage::Registered => CapabilityReadinessStage::Registered,
                    CapabilityReadinessStage::DesignComplete => {
                        CapabilityReadinessStage::DesignComplete
                    }
                    CapabilityReadinessStage::ImplementationReady => {
                        CapabilityReadinessStage::ImplementationReady
                    }
                    CapabilityReadinessStage::EngineReady
                    | CapabilityReadinessStage::PlatformQualified
                    | CapabilityReadinessStage::SignOffEligible => {
                        CapabilityReadinessStage::EngineReady
                    }
                };
                if !record.declared_readiness.satisfies(minimum) {
                    add_reason(
                        reasons,
                        ClaimState::Unknown,
                        ReasonCode::TargetReadinessInsufficient,
                        format!(
                            "engineId:{id}:{}:{}",
                            stage_name(record.declared_readiness),
                            stage_name(minimum)
                        ),
                    );
                }
            }
        } else {
            unknown_target("engineId", id, reasons);
        }
    }
    if let Some(id) = claim.targets.model_binding_id.as_ref() {
        if let Some(record) = fixture
            .model_qualifications
            .iter()
            .find(|record| &record.id == id)
        {
            evaluate_record_state(
                record.state,
                record.protected_claim_eligible,
                "modelBindingId",
                id,
                is_protected,
                reasons,
            );
            if fixture.status == FixtureStatus::ProductionEvidence {
                evaluate_target_verification(record.verification, "modelBindingId", id, reasons);
                let qualification_evidence_id =
                    EvidenceId::from(record.qualification_evidence_id.clone());
                let exact_qualification_is_bound = claim
                    .evidence_binding_ids
                    .contains(&qualification_evidence_id)
                    && fixture.evidence.iter().any(|evidence| {
                        evidence.id == qualification_evidence_id
                            && evidence.kind == EvidenceKind::ModelQualification
                    });
                if !exact_qualification_is_bound {
                    add_reason(
                        reasons,
                        ClaimState::Unknown,
                        ReasonCode::TargetRelationshipMismatch,
                        format!(
                            "modelBindingId:{id}:qualificationEvidenceId:{}",
                            record.qualification_evidence_id
                        ),
                    );
                }
            }
        } else {
            unknown_target("modelBindingId", id, reasons);
        }
    }
    if let Some(id) = claim.targets.external_producer_id.as_ref() {
        if let Some(record) = fixture
            .external_producers
            .iter()
            .find(|record| &record.id == id)
        {
            evaluate_record_state(
                record.state,
                record.protected_claim_eligible,
                "externalProducerId",
                id,
                is_protected,
                reasons,
            );
            if fixture.status == FixtureStatus::ProductionEvidence {
                evaluate_target_verification(
                    record.verification,
                    "externalProducerId",
                    id,
                    reasons,
                );
            }
        } else {
            unknown_target("externalProducerId", id, reasons);
        }
    }
    if let Some(id) = claim.targets.connector_id.as_ref() {
        if let Some(record) = fixture.connectors.iter().find(|record| &record.id == id) {
            evaluate_record_state(
                record.state,
                record.protected_claim_eligible,
                "connectorId",
                id,
                is_protected,
                reasons,
            );
        } else {
            unknown_target("connectorId", id, reasons);
        }
    }
    if let Some(id) = claim.targets.entitlement_policy_id.as_ref() {
        if let Some(record) = fixture.entitlements.iter().find(|record| &record.id == id) {
            evaluate_entitlement(record, as_of, is_protected, reasons);
        } else {
            unknown_target("entitlementPolicyId", id, reasons);
        }
    }
    if let Some(id) = claim.targets.canonical_object_id.as_ref()
        && !fixture.canonical_object_ids.contains(id)
    {
        unknown_target("canonicalObjectId", id, reasons);
    }
    if claim
        .asserted_stage
        .satisfies(CapabilityReadinessStage::PlatformQualified)
        && claim
            .targets
            .canonical_object_id
            .as_ref()
            .is_some_and(|id| id != &fixture.canonical_object_ids.release_candidate)
    {
        add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::TargetRelationshipMismatch,
            "canonicalObjectId:release-candidate-required".into(),
        );
    }

    if let (Some(compute_id), Some(engine_id)) = (
        claim.targets.compute_target_id.as_ref(),
        claim.targets.engine_id.as_ref(),
    ) && let Some(compute) = fixture
        .compute_targets
        .iter()
        .find(|record| &record.id == compute_id)
        && !compute.engine_ids.contains(engine_id)
    {
        add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::TargetRelationshipMismatch,
            format!("computeTargetId:{compute_id}:engineId:{engine_id}"),
        );
    }

    if let Some(connector_id) = claim.targets.connector_id.as_ref()
        && let Some(connector) = fixture
            .connectors
            .iter()
            .find(|record| &record.id == connector_id)
    {
        if claim
            .targets
            .external_producer_id
            .as_ref()
            .is_some_and(|selected| selected != &connector.producer_id)
        {
            add_reason(
                reasons,
                ClaimState::Unavailable,
                ReasonCode::TargetRelationshipMismatch,
                format!("connectorId:{connector_id}:externalProducerId"),
            );
        }
        if claim
            .targets
            .entitlement_policy_id
            .as_ref()
            .is_some_and(|selected| selected != &connector.entitlement_policy_id)
        {
            add_reason(
                reasons,
                ClaimState::Unavailable,
                ReasonCode::TargetRelationshipMismatch,
                format!("connectorId:{connector_id}:entitlementPolicyId"),
            );
        }
        if let Some(producer_id) = claim.targets.external_producer_id.as_ref()
            && let Some(producer) = fixture
                .external_producers
                .iter()
                .find(|record| &record.id == producer_id)
            && producer.external_capability_id != connector.external_capability_id
        {
            add_reason(
                reasons,
                ClaimState::Unavailable,
                ReasonCode::TargetRelationshipMismatch,
                format!("connectorId:{connector_id}:externalCapabilityId"),
            );
        }
    }
}

pub(super) fn evaluate_target_verification(
    verification: VerificationState,
    target_key: &str,
    id: &impl fmt::Display,
    reasons: &mut Vec<ResolutionReason>,
) {
    let (state, code) = match verification {
        VerificationState::Verified => return,
        VerificationState::Rejected => (
            ClaimState::Unavailable,
            ReasonCode::TargetVerificationRejected,
        ),
        VerificationState::Unverified => (
            ClaimState::Unknown,
            ReasonCode::TargetVerificationUnverified,
        ),
        VerificationState::Unknown => (ClaimState::Unknown, ReasonCode::TargetVerificationUnknown),
    };
    add_reason(reasons, state, code, format!("{target_key}:{id}"));
}

pub(super) fn evaluate_entitlement(
    record: &EntitlementRecord,
    as_of: Option<i64>,
    is_protected: bool,
    reasons: &mut Vec<ResolutionReason>,
) {
    match record.grant_state {
        EntitlementGrantState::Denied => add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::EntitlementDenied,
            record.id.to_string(),
        ),
        EntitlementGrantState::Revoked => add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::EntitlementRevoked,
            record.id.to_string(),
        ),
        EntitlementGrantState::Unknown => add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::EntitlementUnknown,
            record.id.to_string(),
        ),
        EntitlementGrantState::Expired => add_reason(
            reasons,
            ClaimState::Stale,
            ReasonCode::EntitlementExpired,
            record.id.to_string(),
        ),
        EntitlementGrantState::Granted => {}
    }
    if timestamp_invalid(record.expires_at.as_ref()) {
        add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::EntitlementTimestampInvalid,
            record.id.to_string(),
        );
    } else if is_expired(record.expires_at.as_ref(), as_of) {
        add_reason(
            reasons,
            ClaimState::Stale,
            ReasonCode::EntitlementExpired,
            record.id.to_string(),
        );
    }
    evaluate_record_state(
        record.state,
        record.protected_claim_eligible,
        "entitlementPolicyId",
        &record.id,
        is_protected,
        reasons,
    );
}

pub(super) fn evaluate_record_state(
    state: ClaimState,
    protected_claim_eligible: bool,
    target_key: &str,
    id: &impl fmt::Display,
    is_protected: bool,
    reasons: &mut Vec<ResolutionReason>,
) {
    let detail = format!("{target_key}:{id}");
    match state {
        ClaimState::Unavailable => add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::TargetUnavailable,
            detail.clone(),
        ),
        ClaimState::Unknown => add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::TargetUnknown,
            detail.clone(),
        ),
        ClaimState::Stale => add_reason(
            reasons,
            ClaimState::Stale,
            ReasonCode::TargetStale,
            detail.clone(),
        ),
        ClaimState::Current => {}
    }
    if is_protected && !protected_claim_eligible {
        add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::TargetProtectedClaimIneligible,
            detail,
        );
    }
}

pub(super) fn unknown_target(
    target_key: &str,
    id: &impl fmt::Display,
    reasons: &mut Vec<ResolutionReason>,
) {
    add_reason(
        reasons,
        ClaimState::Unknown,
        ReasonCode::TargetReferenceUnknown,
        format!("{target_key}:{id}"),
    );
}

pub(super) fn evaluate_evidence(
    fixture: &CapabilityFixture,
    claim: &CapabilityClaim,
    evidence: &EvidenceRecord,
    requirement: EvidenceRequirement,
    as_of: Option<i64>,
    reasons: &mut Vec<ResolutionReason>,
) {
    if !evidence_authority_eligible(
        requirement.kind,
        evidence.source.authority_class,
        claim.claim_mode,
    ) {
        add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::EvidenceAuthorityIneligible,
            evidence.id.to_string(),
        );
    }
    let required_stage = canonical_stage_for_evidence(requirement.kind);
    if !evidence.supports_stages.contains(&required_stage) {
        add_reason(
            reasons,
            ClaimState::Stale,
            ReasonCode::EvidenceStageMismatch,
            format!("{}:{}", evidence.id, stage_name(required_stage)),
        );
    }
    if let Some(family) = requirement.family {
        if !evidence.eligible_label_families.contains(&family) {
            add_reason(
                reasons,
                ClaimState::Unknown,
                ReasonCode::EvidenceLabelFamilyIneligible,
                format!("{}:{}", evidence.id, protected_family_name(family)),
            );
        }
        if claim.claim_mode == ClaimMode::Product {
            let configured = &fixture
                .claim_resolution_algorithm
                .protected_label_policies
                .get(family)
                .product_authority_classes;
            // Product protected labels always require release authority, even
            // if an untrusted fixture attempts to weaken its policy table.
            if evidence.source.authority_class != AuthorityClass::ReleaseEvidence
                || !configured.contains(&evidence.source.authority_class)
            {
                add_reason(
                    reasons,
                    ClaimState::Unknown,
                    ReasonCode::EvidenceAuthorityIneligible,
                    evidence.id.to_string(),
                );
            }
        }
    }

    match evidence.verification {
        VerificationState::Rejected => add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::EvidenceRejected,
            evidence.id.to_string(),
        ),
        VerificationState::Unverified => add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::EvidenceUnverified,
            evidence.id.to_string(),
        ),
        VerificationState::Unknown => add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::EvidenceVerificationUnknown,
            evidence.id.to_string(),
        ),
        VerificationState::Verified => {}
    }
    match evidence.currentness {
        ClaimState::Unavailable => add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::EvidenceUnavailable,
            evidence.id.to_string(),
        ),
        ClaimState::Unknown => add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::EvidenceCurrentnessUnknown,
            evidence.id.to_string(),
        ),
        ClaimState::Stale => add_reason(
            reasons,
            ClaimState::Stale,
            ReasonCode::EvidenceStale,
            evidence.id.to_string(),
        ),
        ClaimState::Current => {}
    }
    match evidence.applicability.status {
        ApplicabilityState::NotApplicable => add_reason(
            reasons,
            ClaimState::Unavailable,
            ReasonCode::EvidenceNotApplicable,
            evidence.id.to_string(),
        ),
        ApplicabilityState::Unknown => add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::EvidenceApplicabilityUnknown,
            evidence.id.to_string(),
        ),
        ApplicabilityState::Applicable => {}
    }
    let issued_at = evidence
        .issued_at
        .as_ref()
        .and_then(Timestamp::unix_seconds);
    let expires_at = evidence
        .expires_at
        .as_ref()
        .and_then(Timestamp::unix_seconds);
    let invalid_interval = issued_at.is_none()
        || timestamp_invalid(evidence.expires_at.as_ref())
        || issued_at
            .zip(expires_at)
            .is_some_and(|(issued, expires)| issued > expires);
    if invalid_interval {
        add_reason(
            reasons,
            ClaimState::Unknown,
            ReasonCode::EvidenceTimestampInvalid,
            evidence.id.to_string(),
        );
    } else if let Some(issued_at) = issued_at {
        if as_of.is_some_and(|evaluated_at| issued_at > evaluated_at) {
            add_reason(
                reasons,
                ClaimState::Unknown,
                ReasonCode::EvidenceNotYetValid,
                evidence.id.to_string(),
            );
        } else if expires_at
            .zip(as_of)
            .is_some_and(|(expires, evaluated_at)| expires <= evaluated_at)
        {
            add_reason(
                reasons,
                ClaimState::Stale,
                ReasonCode::EvidenceExpired,
                evidence.id.to_string(),
            );
        }
    }
    evaluate_exact_target_binding(claim, evidence, reasons);
}

pub(super) fn evidence_authority_eligible(
    kind: EvidenceKind,
    authority: AuthorityClass,
    claim_mode: ClaimMode,
) -> bool {
    if claim_mode == ClaimMode::ContractTestVector {
        return authority == AuthorityClass::ContractTestVector;
    }
    match kind {
        EvidenceKind::DesignContract => matches!(
            authority,
            AuthorityClass::DesignContract | AuthorityClass::ReleaseEvidence
        ),
        EvidenceKind::ImplementationConformance | EvidenceKind::EngineConformance => matches!(
            authority,
            AuthorityClass::ImplementationEvidence | AuthorityClass::ReleaseEvidence
        ),
        EvidenceKind::ModelQualification
        | EvidenceKind::PlatformQualification
        | EvidenceKind::ExternalProducerQualification
        | EvidenceKind::ReleaseQualification
        | EvidenceKind::SignOffApproval => authority == AuthorityClass::ReleaseEvidence,
    }
}

pub(super) fn evaluate_exact_target_binding(
    claim: &CapabilityClaim,
    evidence: &EvidenceRecord,
    reasons: &mut Vec<ResolutionReason>,
) {
    macro_rules! exact_target {
        ($field:ident, $selector:ident, $name:literal) => {
            match claim.targets.$field.as_ref() {
                Some(id) if !evidence.applicability.$selector.contains(id) => {
                    add_reason(
                        reasons,
                        ClaimState::Stale,
                        ReasonCode::EvidenceTargetMismatch,
                        format!("{}:{}:{}", evidence.id, $name, id),
                    );
                }
                None if !evidence.applicability.$selector.is_empty() => {
                    add_reason(
                        reasons,
                        ClaimState::Stale,
                        ReasonCode::EvidenceTargetMismatch,
                        format!("{}:{}:claim-selector-missing", evidence.id, $name),
                    );
                }
                _ => {}
            }
        };
    }
    exact_target!(build_id, build_ids, "buildId");
    exact_target!(platform_id, platform_ids, "platformId");
    exact_target!(compute_target_id, compute_target_ids, "computeTargetId");
    exact_target!(engine_id, engine_ids, "engineId");
    exact_target!(model_binding_id, model_binding_ids, "modelBindingId");
    exact_target!(
        external_producer_id,
        external_producer_ids,
        "externalProducerId"
    );
    exact_target!(connector_id, connector_ids, "connectorId");
    exact_target!(
        entitlement_policy_id,
        entitlement_policy_ids,
        "entitlementPolicyId"
    );
    exact_target!(
        canonical_object_id,
        canonical_object_ids,
        "canonicalObjectId"
    );
}

pub(super) fn binding_summary(
    requested_id: &EvidenceId,
    evidence: Option<&EvidenceRecord>,
    selected: &[EvidenceId],
) -> EvidenceBindingSummary {
    let Some(evidence) = evidence else {
        return EvidenceBindingSummary {
            id: Some(requested_id.clone()),
            kind: None,
            selected: false,
            source: None,
            verification: VerificationState::Unknown,
            currentness: ClaimState::Unknown,
            applicability: ApplicabilityState::Unknown,
            expires_at: Nullable::none(),
        };
    };
    EvidenceBindingSummary {
        id: Some(evidence.id.clone()),
        kind: Some(evidence.kind),
        selected: selected.contains(requested_id),
        source: Some(evidence.source.clone()),
        verification: evidence.verification,
        currentness: evidence.currentness,
        applicability: evidence.applicability.status,
        expires_at: evidence.expires_at.clone(),
    }
}

pub(super) fn add_reason(
    reasons: &mut Vec<ResolutionReason>,
    state: ClaimState,
    code: ReasonCode,
    detail: String,
) {
    if !reasons
        .iter()
        .any(|reason| reason.code == code && reason.detail == detail)
    {
        reasons.push(ResolutionReason {
            state,
            code,
            detail,
        });
    }
}

pub(super) fn final_state(reasons: &[ResolutionReason]) -> ClaimState {
    reasons
        .iter()
        .map(|reason| reason.state)
        .min_by_key(|state| state.precedence_rank())
        .unwrap_or(ClaimState::Current)
}

pub(super) fn unique_required_kinds(requirements: &[EvidenceRequirement]) -> Vec<EvidenceKind> {
    requirements
        .iter()
        .map(|requirement| requirement.kind)
        .fold(Vec::new(), |mut kinds, kind| {
            if !kinds.contains(&kind) {
                kinds.push(kind);
            }
            kinds
        })
}

pub(super) fn canonical_stage_evidence(
    stage: CapabilityReadinessStage,
    cumulative: bool,
) -> &'static [EvidenceKind] {
    if cumulative {
        return match stage {
            CapabilityReadinessStage::Registered => &[],
            CapabilityReadinessStage::DesignComplete => &[EvidenceKind::DesignContract],
            CapabilityReadinessStage::ImplementationReady => &[
                EvidenceKind::DesignContract,
                EvidenceKind::ImplementationConformance,
            ],
            CapabilityReadinessStage::EngineReady => &[
                EvidenceKind::DesignContract,
                EvidenceKind::ImplementationConformance,
                EvidenceKind::EngineConformance,
            ],
            CapabilityReadinessStage::PlatformQualified => &[
                EvidenceKind::DesignContract,
                EvidenceKind::ImplementationConformance,
                EvidenceKind::EngineConformance,
                EvidenceKind::PlatformQualification,
            ],
            CapabilityReadinessStage::SignOffEligible => &[
                EvidenceKind::DesignContract,
                EvidenceKind::ImplementationConformance,
                EvidenceKind::EngineConformance,
                EvidenceKind::PlatformQualification,
                EvidenceKind::SignOffApproval,
            ],
        };
    }
    match stage {
        CapabilityReadinessStage::Registered => &[],
        CapabilityReadinessStage::DesignComplete => &[EvidenceKind::DesignContract],
        CapabilityReadinessStage::ImplementationReady => &[EvidenceKind::ImplementationConformance],
        CapabilityReadinessStage::EngineReady => &[EvidenceKind::EngineConformance],
        CapabilityReadinessStage::PlatformQualified => &[EvidenceKind::PlatformQualification],
        CapabilityReadinessStage::SignOffEligible => &[EvidenceKind::SignOffApproval],
    }
}

pub(super) const fn canonical_stage_for_evidence(kind: EvidenceKind) -> CapabilityReadinessStage {
    match kind {
        EvidenceKind::DesignContract => CapabilityReadinessStage::DesignComplete,
        EvidenceKind::ImplementationConformance => CapabilityReadinessStage::ImplementationReady,
        EvidenceKind::EngineConformance
        | EvidenceKind::ModelQualification
        | EvidenceKind::ExternalProducerQualification => CapabilityReadinessStage::EngineReady,
        EvidenceKind::PlatformQualification | EvidenceKind::ReleaseQualification => {
            CapabilityReadinessStage::PlatformQualified
        }
        EvidenceKind::SignOffApproval => CapabilityReadinessStage::SignOffEligible,
    }
}

pub(super) fn canonical_minimum_stage(
    family: ProtectedLabelFamily,
    subject: SubjectKind,
) -> CapabilityReadinessStage {
    match family {
        ProtectedLabelFamily::Release => CapabilityReadinessStage::PlatformQualified,
        ProtectedLabelFamily::Qualified => match subject {
            SubjectKind::Engine | SubjectKind::Model | SubjectKind::ExternalProducer => {
                CapabilityReadinessStage::EngineReady
            }
            _ => CapabilityReadinessStage::PlatformQualified,
        },
        ProtectedLabelFamily::SignOff => CapabilityReadinessStage::SignOffEligible,
    }
}

pub(super) fn canonical_protected_evidence(
    family: ProtectedLabelFamily,
    subject: SubjectKind,
) -> EvidenceKind {
    match family {
        ProtectedLabelFamily::Release => EvidenceKind::ReleaseQualification,
        ProtectedLabelFamily::Qualified => match subject {
            SubjectKind::Engine => EvidenceKind::EngineConformance,
            SubjectKind::Model => EvidenceKind::ModelQualification,
            SubjectKind::ExternalProducer | SubjectKind::Connector => {
                EvidenceKind::ExternalProducerQualification
            }
            _ => EvidenceKind::PlatformQualification,
        },
        ProtectedLabelFamily::SignOff => EvidenceKind::SignOffApproval,
    }
}

pub(super) fn contains_bounded_ascii(haystack: &str, needle: &str) -> bool {
    haystack.match_indices(needle).any(|(start, _)| {
        let end = start + needle.len();
        let before = haystack[..start].chars().next_back();
        let after = haystack[end..].chars().next();
        before.is_none_or(|character| !is_word_character(character))
            && after.is_none_or(|character| !is_word_character(character))
    })
}

pub(super) const fn is_word_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

pub(super) fn is_expired(expires_at: Option<&Timestamp>, as_of: Option<i64>) -> bool {
    match (expires_at.and_then(Timestamp::unix_seconds), as_of) {
        (Some(expires_at), Some(as_of)) => expires_at <= as_of,
        _ => false,
    }
}

pub(super) fn timestamp_invalid(timestamp: Option<&Timestamp>) -> bool {
    timestamp.is_some_and(|timestamp| timestamp.unix_seconds().is_none())
}

pub(super) fn parse_rfc3339_seconds(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    if bytes.len() < 20 || bytes.get(4) != Some(&b'-') || bytes.get(7) != Some(&b'-') {
        return None;
    }
    if !matches!(bytes.get(10), Some(b'T' | b't'))
        || bytes.get(13) != Some(&b':')
        || bytes.get(16) != Some(&b':')
    {
        return None;
    }
    let year = parse_digits(bytes, 0, 4)? as i32;
    let month = parse_digits(bytes, 5, 2)? as u32;
    let day = parse_digits(bytes, 8, 2)? as u32;
    let hour = parse_digits(bytes, 11, 2)? as u32;
    let minute = parse_digits(bytes, 14, 2)? as u32;
    let second = parse_digits(bytes, 17, 2)? as u32;
    if !(1..=12).contains(&month)
        || day == 0
        || day > days_in_month(year, month)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return None;
    }

    let mut cursor = 19;
    if bytes.get(cursor) == Some(&b'.') {
        cursor += 1;
        let fraction_start = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor += 1;
        }
        if cursor == fraction_start {
            return None;
        }
    }
    let offset_seconds = match bytes.get(cursor) {
        Some(b'Z' | b'z') if cursor + 1 == bytes.len() => 0_i64,
        Some(sign @ (b'+' | b'-')) if cursor + 6 == bytes.len() => {
            if bytes.get(cursor + 3) != Some(&b':') {
                return None;
            }
            let offset_hour = parse_digits(bytes, cursor + 1, 2)? as i64;
            let offset_minute = parse_digits(bytes, cursor + 4, 2)? as i64;
            if offset_hour > 23 || offset_minute > 59 {
                return None;
            }
            let magnitude = offset_hour * 3_600 + offset_minute * 60;
            if *sign == b'+' { magnitude } else { -magnitude }
        }
        _ => return None,
    };
    let local_seconds = days_from_civil(year, month, day)
        .checked_mul(86_400)?
        .checked_add(i64::from(hour) * 3_600 + i64::from(minute) * 60 + i64::from(second))?;
    local_seconds.checked_sub(offset_seconds)
}

pub(super) fn parse_digits(bytes: &[u8], start: usize, length: usize) -> Option<u32> {
    let slice = bytes.get(start..start + length)?;
    if !slice.iter().all(u8::is_ascii_digit) {
        return None;
    }
    slice.iter().try_fold(0_u32, |value, digit| {
        value.checked_mul(10)?.checked_add(u32::from(*digit - b'0'))
    })
}

pub(super) const fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

pub(super) const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Days since 1970-01-01, using Howard Hinnant's civil-date transform.
pub(super) fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let adjusted_year = year - if month <= 2 { 1 } else { 0 };
    let era = if adjusted_year >= 0 {
        adjusted_year
    } else {
        adjusted_year - 399
    } / 400;
    let year_of_era = adjusted_year - era * 400;
    let shifted_month = i64::from(month) + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * shifted_month + 2) / 5 + i64::from(day) - 1;
    let day_of_era = i64::from(year_of_era) * 365 + i64::from(year_of_era) / 4
        - i64::from(year_of_era) / 100
        + day_of_year;
    i64::from(era) * 146_097 + day_of_era - 719_468
}

pub(super) const fn stage_name(stage: CapabilityReadinessStage) -> &'static str {
    match stage {
        CapabilityReadinessStage::Registered => "registered",
        CapabilityReadinessStage::DesignComplete => "design-complete",
        CapabilityReadinessStage::ImplementationReady => "implementation-ready",
        CapabilityReadinessStage::EngineReady => "engine-ready",
        CapabilityReadinessStage::PlatformQualified => "platform-qualified",
        CapabilityReadinessStage::SignOffEligible => "sign-off-eligible",
    }
}

pub(super) const fn protected_family_name(family: ProtectedLabelFamily) -> &'static str {
    match family {
        ProtectedLabelFamily::Release => "release",
        ProtectedLabelFamily::Qualified => "qualified",
        ProtectedLabelFamily::SignOff => "sign-off",
    }
}

pub(super) const fn evidence_kind_name(kind: EvidenceKind) -> &'static str {
    match kind {
        EvidenceKind::DesignContract => "design-contract",
        EvidenceKind::ImplementationConformance => "implementation-conformance",
        EvidenceKind::EngineConformance => "engine-conformance",
        EvidenceKind::ModelQualification => "model-qualification",
        EvidenceKind::PlatformQualification => "platform-qualification",
        EvidenceKind::ExternalProducerQualification => "external-producer-qualification",
        EvidenceKind::ReleaseQualification => "release-qualification",
        EvidenceKind::SignOffApproval => "sign-off-approval",
    }
}
