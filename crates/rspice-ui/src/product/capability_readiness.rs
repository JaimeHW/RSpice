//! Fail-closed capability-readiness contracts and resolver.
//!
//! This module deliberately does not inspect compiled Rust modules, registered
//! routes, simulator APIs, or visible controls. Those are not capability
//! evidence. A customer-facing claim is resolved only from an explicit evidence
//! bundle whose source, target applicability, verification, currentness, and
//! authority all satisfy the contract below.
//!
//! The mockup capability fixture is consumed only by this module's tests. The
//! production resolver defaults to no evidence and therefore no allowed claims.

use std::{collections::HashSet, fmt, hash::Hash};

use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};

pub const CAPABILITY_RESOLVER_ALGORITHM_VERSION: &str = "1.0.0";
pub const CAPABILITY_PRODUCTION_EVIDENCE_SCHEMA: &str =
    "https://rspice.dev/schemas/capability-production-evidence/1.0.0";
pub const CAPABILITY_PRODUCTION_EVIDENCE_SCHEMA_VERSION: &str = "1.0.0";
pub const MAX_CAPABILITY_EVIDENCE_PAYLOAD_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_CAPABILITY_CLAIM_LABEL_BYTES: usize = 512;
pub const MAX_CAPABILITY_CLAIM_EVIDENCE_BINDINGS: usize = 256;
pub const MAX_CAPABILITY_CLAIM_IDENTIFIER_BYTES: usize = 256;

macro_rules! typed_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[must_use]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

typed_id!(BuildId);
typed_id!(ComputeTargetId);
typed_id!(EngineId);
typed_id!(EngineBuildId);
typed_id!(ModelBindingId);
typed_id!(ExternalProducerId);
typed_id!(ExternalCapabilityId);
typed_id!(ConnectorId);
typed_id!(EntitlementPolicyId);
typed_id!(CanonicalObjectId);
typed_id!(EvidenceId);
typed_id!(ClaimId);

/// A required JSON field whose value may explicitly be `null`.
///
/// Unlike an `Option<T>` field, this wrapper makes omission a deserialization
/// error while still representing the fixture's explicit null selectors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Nullable<T>(pub Option<T>);

impl<T> Nullable<T> {
    #[must_use]
    pub const fn none() -> Self {
        Self(None)
    }

    #[must_use]
    pub const fn some(value: T) -> Self {
        Self(Some(value))
    }

    #[must_use]
    pub const fn as_ref(&self) -> Option<&T> {
        self.0.as_ref()
    }
}

fn deserialize_nullable<'de, D, T>(deserializer: D) -> Result<Nullable<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Nullable)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(String);

impl Timestamp {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn unix_seconds(&self) -> Option<i64> {
        parse_rfc3339_seconds(self.as_str())
    }
}

impl From<&str> for Timestamp {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FixtureStatus {
    /// Deterministic design and contract-test data; never a production provider.
    DesignFixture,
    /// An independently supplied evidence bundle eligible for production use.
    ProductionEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimState {
    Current,
    Stale,
    Unavailable,
    Unknown,
}

impl ClaimState {
    /// Lower ranks are more fail-closed and win aggregate resolution.
    const fn precedence_rank(self) -> u8 {
        match self {
            Self::Unavailable => 0,
            Self::Unknown => 1,
            Self::Stale => 2,
            Self::Current => 3,
        }
    }
}

/// Capability readiness has its own exact six-stage vocabulary. It must not be
/// conflated with the existing product lifecycle `ReadinessStage`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityReadinessStage {
    Registered,
    DesignComplete,
    ImplementationReady,
    EngineReady,
    PlatformQualified,
    SignOffEligible,
}

impl CapabilityReadinessStage {
    pub const ALL: [Self; 6] = [
        Self::Registered,
        Self::DesignComplete,
        Self::ImplementationReady,
        Self::EngineReady,
        Self::PlatformQualified,
        Self::SignOffEligible,
    ];

    #[must_use]
    pub const fn satisfies(self, minimum: Self) -> bool {
        self as u8 >= minimum as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimMode {
    Product,
    ContractTestVector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SubjectKind {
    Design,
    Build,
    Engine,
    Model,
    Platform,
    ExternalProducer,
    Connector,
    Release,
    SignOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceKind {
    DesignContract,
    ImplementationConformance,
    EngineConformance,
    ModelQualification,
    PlatformQualification,
    ExternalProducerQualification,
    ReleaseQualification,
    SignOffApproval,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationState {
    Verified,
    Unverified,
    Rejected,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicabilityState {
    Applicable,
    NotApplicable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntitlementGrantState {
    Granted,
    Denied,
    Expired,
    Revoked,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthorityClass {
    DesignContract,
    ImplementationEvidence,
    ReleaseEvidence,
    ContractTestVector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProtectedLabelFamily {
    Release,
    Qualified,
    SignOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformId {
    Desktop,
    Browser,
    Tablet,
    Phone,
}

impl fmt::Display for PlatformId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Desktop => "desktop",
            Self::Browser => "browser",
            Self::Tablet => "tablet",
            Self::Phone => "phone",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComputeMode {
    Local,
    Remote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineMode {
    Native,
    Preview,
    Compatibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapabilityBoundary {
    pub purpose: String,
    pub does_not_claim: String,
    pub currentness_meaning: String,
    pub test_vector_meaning: String,
    pub unknown_policy: String,
    pub stale_policy: String,
    pub unavailable_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContractAuthority {
    pub path: String,
    pub id: String,
    pub schema_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FixtureAuthority {
    pub plan_sections: Vec<String>,
    pub canonical_fixture: ContractAuthority,
    pub commercial_scope: ContractAuthority,
    pub platform_task: ContractAuthority,
    pub product_manifest_path: String,
    pub resolver_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapabilityVocabulary {
    pub readiness_stages: Vec<CapabilityReadinessStage>,
    pub claim_states: Vec<ClaimState>,
    pub platform_ids: Vec<PlatformId>,
    pub compute_modes: Vec<ComputeMode>,
    pub engine_modes: Vec<EngineMode>,
    pub claim_modes: Vec<ClaimMode>,
    pub subject_kinds: Vec<SubjectKind>,
    pub evidence_kinds: Vec<EvidenceKind>,
    pub verification_states: Vec<VerificationState>,
    pub applicability_states: Vec<ApplicabilityState>,
    pub entitlement_grant_states: Vec<EntitlementGrantState>,
    pub authority_classes: Vec<AuthorityClass>,
    pub protected_label_families: Vec<ProtectedLabelFamily>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BuildIdentity {
    pub id: BuildId,
    pub package_name: String,
    pub package_version: String,
    pub artifact_kind: String,
    pub fixture_revision: String,
    pub product_fixture_id: String,
    pub product_fixture_schema_version: String,
    pub canonical_input_revision: String,
    pub engine_build_fixture_id: EngineBuildId,
    pub state: ClaimState,
    pub protected_claim_eligible: bool,
    pub qualification_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CanonicalObjectIds {
    pub project: CanonicalObjectId,
    pub design: CanonicalObjectId,
    pub testbench: CanonicalObjectId,
    pub simulation_plan: CanonicalObjectId,
    pub run_set: CanonicalObjectId,
    pub job: CanonicalObjectId,
    pub run: CanonicalObjectId,
    pub nominal_dataset: CanonicalObjectId,
    pub pvt_dataset: CanonicalObjectId,
    pub result_document: CanonicalObjectId,
    pub verification_evidence: CanonicalObjectId,
    pub release_candidate: CanonicalObjectId,
    pub model_binding: CanonicalObjectId,
    pub review_comment: CanonicalObjectId,
    pub approval: CanonicalObjectId,
}

impl CanonicalObjectIds {
    fn values(&self) -> [&CanonicalObjectId; 15] {
        [
            &self.project,
            &self.design,
            &self.testbench,
            &self.simulation_plan,
            &self.run_set,
            &self.job,
            &self.run,
            &self.nominal_dataset,
            &self.pvt_dataset,
            &self.result_document,
            &self.verification_evidence,
            &self.release_candidate,
            &self.model_binding,
            &self.review_comment,
            &self.approval,
        ]
    }

    fn contains(&self, id: &CanonicalObjectId) -> bool {
        self.values().contains(&id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformRecord {
    pub id: PlatformId,
    pub platform_task_contract_id: String,
    pub state: ClaimState,
    pub qualification: String,
    pub protected_claim_eligible: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformTargets {
    pub desktop: PlatformRecord,
    pub browser: PlatformRecord,
    pub tablet: PlatformRecord,
    pub phone: PlatformRecord,
}

impl PlatformTargets {
    fn get(&self, id: PlatformId) -> &PlatformRecord {
        match id {
            PlatformId::Desktop => &self.desktop,
            PlatformId::Browser => &self.browser,
            PlatformId::Tablet => &self.tablet,
            PlatformId::Phone => &self.phone,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ComputeTarget {
    pub id: ComputeTargetId,
    pub mode: ComputeMode,
    pub state: ClaimState,
    pub engine_ids: Vec<EngineId>,
    pub protected_claim_eligible: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EngineRecord {
    pub id: EngineId,
    pub mode: EngineMode,
    pub build_id: EngineBuildId,
    pub state: ClaimState,
    pub declared_readiness: CapabilityReadinessStage,
    pub verification: VerificationState,
    pub protected_claim_eligible: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModelQualification {
    pub id: ModelBindingId,
    pub pdk_lock: String,
    pub set_digest: String,
    pub qualification_evidence_id: String,
    pub state: ClaimState,
    pub verification: VerificationState,
    pub protected_claim_eligible: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExternalProducer {
    pub id: ExternalProducerId,
    pub external_capability_id: ExternalCapabilityId,
    pub state: ClaimState,
    pub verification: VerificationState,
    pub protected_claim_eligible: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConnectorRecord {
    pub id: ConnectorId,
    pub external_capability_id: ExternalCapabilityId,
    pub producer_id: ExternalProducerId,
    pub entitlement_policy_id: EntitlementPolicyId,
    pub state: ClaimState,
    pub protected_claim_eligible: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EntitlementRecord {
    pub id: EntitlementPolicyId,
    pub state: ClaimState,
    pub grant_state: EntitlementGrantState,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub expires_at: Nullable<Timestamp>,
    pub protected_claim_eligible: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TargetSelectorMap {
    pub build_id: String,
    pub platform_id: String,
    pub compute_target_id: String,
    pub engine_id: String,
    pub model_binding_id: String,
    pub external_producer_id: String,
    pub connector_id: String,
    pub entitlement_policy_id: String,
    pub canonical_object_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct StageEvidenceKinds {
    pub registered: Vec<EvidenceKind>,
    pub design_complete: Vec<EvidenceKind>,
    pub implementation_ready: Vec<EvidenceKind>,
    pub engine_ready: Vec<EvidenceKind>,
    pub platform_qualified: Vec<EvidenceKind>,
    pub sign_off_eligible: Vec<EvidenceKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SubjectStageMap {
    pub engine: Option<CapabilityReadinessStage>,
    pub model: Option<CapabilityReadinessStage>,
    pub external_producer: Option<CapabilityReadinessStage>,
    pub platform: Option<CapabilityReadinessStage>,
    pub connector: Option<CapabilityReadinessStage>,
    pub default: Option<CapabilityReadinessStage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SubjectEvidenceMap {
    pub engine: Option<EvidenceKind>,
    pub model: Option<EvidenceKind>,
    pub external_producer: Option<EvidenceKind>,
    pub platform: Option<EvidenceKind>,
    pub connector: Option<EvidenceKind>,
    pub default: Option<EvidenceKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProtectedLabelPolicy {
    pub pattern: String,
    pub minimum_stage_by_subject: SubjectStageMap,
    pub required_evidence_kind_by_subject: SubjectEvidenceMap,
    pub product_authority_classes: Vec<AuthorityClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ProtectedLabelPolicies {
    pub release: ProtectedLabelPolicy,
    pub qualified: ProtectedLabelPolicy,
    pub sign_off: ProtectedLabelPolicy,
}

impl ProtectedLabelPolicies {
    fn get(&self, family: ProtectedLabelFamily) -> &ProtectedLabelPolicy {
        match family {
            ProtectedLabelFamily::Release => &self.release,
            ProtectedLabelFamily::Qualified => &self.qualified,
            ProtectedLabelFamily::SignOff => &self.sign_off,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AlgorithmStep {
    pub order: u8,
    pub id: String,
    pub rule: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClaimResolutionAlgorithm {
    pub version: String,
    pub precedence: Vec<ClaimState>,
    pub target_selector_map: TargetSelectorMap,
    pub stage_evidence_kinds: StageEvidenceKinds,
    pub protected_label_policies: ProtectedLabelPolicies,
    pub steps: Vec<AlgorithmStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvidenceSource {
    pub authority_class: AuthorityClass,
    pub locator: String,
    pub revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvidenceApplicability {
    pub status: ApplicabilityState,
    pub build_ids: Vec<BuildId>,
    pub platform_ids: Vec<PlatformId>,
    pub compute_target_ids: Vec<ComputeTargetId>,
    pub engine_ids: Vec<EngineId>,
    pub model_binding_ids: Vec<ModelBindingId>,
    pub external_producer_ids: Vec<ExternalProducerId>,
    pub connector_ids: Vec<ConnectorId>,
    pub entitlement_policy_ids: Vec<EntitlementPolicyId>,
    pub canonical_object_ids: Vec<CanonicalObjectId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvidenceRecord {
    pub id: EvidenceId,
    pub kind: EvidenceKind,
    pub source: EvidenceSource,
    pub verification: VerificationState,
    pub currentness: ClaimState,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub issued_at: Nullable<Timestamp>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub expires_at: Nullable<Timestamp>,
    pub applicability: EvidenceApplicability,
    pub supports_stages: Vec<CapabilityReadinessStage>,
    pub eligible_label_families: Vec<ProtectedLabelFamily>,
    pub boundary: String,
}

/// Exact nine-selector claim target tuple. Every field must be present in
/// serialized input, though any selector may explicitly be null.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClaimTargets {
    #[serde(deserialize_with = "deserialize_nullable")]
    pub build_id: Nullable<BuildId>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub platform_id: Nullable<PlatformId>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub compute_target_id: Nullable<ComputeTargetId>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub engine_id: Nullable<EngineId>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub model_binding_id: Nullable<ModelBindingId>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub external_producer_id: Nullable<ExternalProducerId>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub connector_id: Nullable<ConnectorId>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub entitlement_policy_id: Nullable<EntitlementPolicyId>,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub canonical_object_id: Nullable<CanonicalObjectId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapabilityClaim {
    pub id: Option<ClaimId>,
    pub claim_mode: ClaimMode,
    pub label: String,
    pub subject_kind: SubjectKind,
    pub asserted_stage: CapabilityReadinessStage,
    pub targets: ClaimTargets,
    pub evidence_binding_ids: Vec<EvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpectedResolution {
    pub state: ClaimState,
    pub label_allowed: bool,
    pub required_reason_codes: Vec<ReasonCode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResolutionCase {
    pub id: ClaimId,
    pub claim_mode: ClaimMode,
    pub label: String,
    pub subject_kind: SubjectKind,
    pub asserted_stage: CapabilityReadinessStage,
    pub targets: ClaimTargets,
    pub evidence_binding_ids: Vec<EvidenceId>,
    pub expected: ExpectedResolution,
}

impl ResolutionCase {
    #[must_use]
    pub fn claim(&self) -> CapabilityClaim {
        CapabilityClaim {
            id: Some(self.id.clone()),
            claim_mode: self.claim_mode,
            label: self.label.clone(),
            subject_kind: self.subject_kind,
            asserted_stage: self.asserted_stage,
            targets: self.targets.clone(),
            evidence_binding_ids: self.evidence_binding_ids.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CapabilityFixture {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub schema_version: String,
    pub fixture_id: String,
    pub fixture_revision: String,
    pub status: FixtureStatus,
    pub as_of: Timestamp,
    pub boundary: CapabilityBoundary,
    pub authority: FixtureAuthority,
    pub vocabulary: CapabilityVocabulary,
    pub build_identity: BuildIdentity,
    pub canonical_object_ids: CanonicalObjectIds,
    pub platform_targets: PlatformTargets,
    pub compute_targets: Vec<ComputeTarget>,
    pub engines: Vec<EngineRecord>,
    pub model_qualifications: Vec<ModelQualification>,
    pub external_producers: Vec<ExternalProducer>,
    pub connectors: Vec<ConnectorRecord>,
    pub entitlements: Vec<EntitlementRecord>,
    pub claim_resolution_algorithm: ClaimResolutionAlgorithm,
    pub evidence: Vec<EvidenceRecord>,
    pub resolution_cases: Vec<ResolutionCase>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReasonCode {
    EvidenceProviderUnavailable,
    InvalidAsOf,
    ResolverPolicyMissing,
    ClaimInputTooLarge,
    UngovernedProductLabel,
    RegisteredProductClaimForbidden,
    RequiredTargetMissing,
    ReadinessStageInsufficient,
    TargetReferenceUnknown,
    TargetRelationshipMismatch,
    TargetUnavailable,
    TargetUnknown,
    TargetStale,
    TargetReadinessInsufficient,
    TargetEngineModeIneligible,
    TargetVerificationRejected,
    TargetVerificationUnverified,
    TargetVerificationUnknown,
    TargetProtectedClaimIneligible,
    EntitlementDenied,
    EntitlementRevoked,
    EntitlementUnknown,
    EntitlementExpired,
    EntitlementTimestampInvalid,
    EvidenceReferenceUnknown,
    MissingRequiredEvidence,
    EvidenceStageMismatch,
    EvidenceLabelFamilyIneligible,
    EvidenceAuthorityIneligible,
    EvidenceRejected,
    EvidenceUnverified,
    EvidenceVerificationUnknown,
    EvidenceUnavailable,
    EvidenceCurrentnessUnknown,
    EvidenceStale,
    EvidenceNotApplicable,
    EvidenceApplicabilityUnknown,
    EvidenceExpired,
    EvidenceNotYetValid,
    EvidenceTimestampInvalid,
    EvidenceTargetMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolutionReason {
    pub state: ClaimState,
    pub code: ReasonCode,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvidenceBindingSummary {
    pub id: Option<EvidenceId>,
    pub kind: Option<EvidenceKind>,
    pub selected: bool,
    pub source: Option<EvidenceSource>,
    pub verification: VerificationState,
    pub currentness: ClaimState,
    pub applicability: ApplicabilityState,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub expires_at: Nullable<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResolvedApplicability {
    pub targets: ClaimTargets,
    pub exact_target_binding_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResolvedCurrentness {
    #[serde(deserialize_with = "deserialize_nullable")]
    pub evaluated_at: Nullable<Timestamp>,
    pub state: ClaimState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResolvedClaim {
    pub case_id: Option<ClaimId>,
    pub algorithm_version: String,
    #[serde(deserialize_with = "deserialize_nullable")]
    pub as_of: Nullable<Timestamp>,
    pub claim_mode: ClaimMode,
    pub label: String,
    pub subject_kind: SubjectKind,
    pub asserted_stage: CapabilityReadinessStage,
    pub state: ClaimState,
    pub label_allowed: bool,
    pub protected_label_families: Vec<ProtectedLabelFamily>,
    pub required_evidence_kinds: Vec<EvidenceKind>,
    pub source: Vec<EvidenceBindingSummary>,
    pub applicability: ResolvedApplicability,
    pub currentness: ResolvedCurrentness,
    pub reason_codes: Vec<ReasonCode>,
    pub reasons: Vec<ResolutionReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductionEvidenceError {
    PayloadTooLarge {
        actual_bytes: usize,
        maximum_bytes: usize,
    },
    InvalidTrustedPublicKey,
    InvalidSignature,
    MalformedPayload(String),
    NonProductionFixture {
        fixture_id: String,
        actual_status: FixtureStatus,
    },
    InvalidSchema {
        fixture_id: String,
        actual_schema: String,
        actual_version: String,
    },
    UnsupportedAlgorithm {
        fixture_id: String,
        actual_version: String,
    },
    InvalidReadinessVocabulary {
        fixture_id: String,
    },
    InvalidStructure {
        fixture_id: String,
        detail: String,
    },
}

impl fmt::Display for ProductionEvidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayloadTooLarge {
                actual_bytes,
                maximum_bytes,
            } => write!(
                formatter,
                "capability evidence payload is {actual_bytes} bytes; maximum is {maximum_bytes}"
            ),
            Self::InvalidTrustedPublicKey => {
                formatter.write_str("capability evidence trusted public key is invalid")
            }
            Self::InvalidSignature => {
                formatter.write_str("capability evidence signature verification failed")
            }
            Self::MalformedPayload(detail) => {
                write!(
                    formatter,
                    "capability evidence payload is malformed: {detail}"
                )
            }
            Self::NonProductionFixture {
                fixture_id,
                actual_status,
            } => write!(
                formatter,
                "capability fixture {fixture_id} is {actual_status:?}, not production evidence"
            ),
            Self::InvalidSchema {
                fixture_id,
                actual_schema,
                actual_version,
            } => write!(
                formatter,
                "capability fixture {fixture_id} declares unsupported schema {actual_schema} version {actual_version}"
            ),
            Self::UnsupportedAlgorithm {
                fixture_id,
                actual_version,
            } => write!(
                formatter,
                "capability fixture {fixture_id} requires unsupported resolver {actual_version}"
            ),
            Self::InvalidReadinessVocabulary { fixture_id } => write!(
                formatter,
                "capability fixture {fixture_id} does not declare the exact fail-closed vocabulary"
            ),
            Self::InvalidStructure { fixture_id, detail } => {
                write!(
                    formatter,
                    "capability fixture {fixture_id} is structurally invalid: {detail}"
                )
            }
        }
    }
}

impl std::error::Error for ProductionEvidenceError {}

/// An owned production bundle whose exact serialized bytes were authenticated
/// by a caller-supplied Ed25519 trust anchor and passed every structural gate.
/// Fields are private so raw or self-declared fixtures cannot be bound to the
/// production resolver.
#[derive(Debug)]
pub struct VerifiedProductionEvidence {
    fixture: CapabilityFixture,
    signed_payload: Box<[u8]>,
    signature: [u8; 64],
    signer_public_key: [u8; 32],
}

impl VerifiedProductionEvidence {
    pub fn verify(
        exact_payload: &[u8],
        signature_bytes: &[u8; 64],
        trusted_public_key: &[u8; 32],
    ) -> Result<Self, ProductionEvidenceError> {
        if exact_payload.len() > MAX_CAPABILITY_EVIDENCE_PAYLOAD_BYTES {
            return Err(ProductionEvidenceError::PayloadTooLarge {
                actual_bytes: exact_payload.len(),
                maximum_bytes: MAX_CAPABILITY_EVIDENCE_PAYLOAD_BYTES,
            });
        }
        let verifying_key = VerifyingKey::from_bytes(trusted_public_key)
            .map_err(|_| ProductionEvidenceError::InvalidTrustedPublicKey)?;
        verifying_key
            .verify_strict(exact_payload, &Signature::from_bytes(signature_bytes))
            .map_err(|_| ProductionEvidenceError::InvalidSignature)?;
        let fixture = serde_json::from_slice::<CapabilityFixture>(exact_payload)
            .map_err(|error| ProductionEvidenceError::MalformedPayload(error.to_string()))?;
        validate_production_fixture(&fixture)?;
        Ok(Self {
            fixture,
            signed_payload: exact_payload.into(),
            signature: *signature_bytes,
            signer_public_key: *trusted_public_key,
        })
    }

    #[must_use]
    pub fn fixture_id(&self) -> &str {
        &self.fixture.fixture_id
    }

    #[must_use]
    pub fn exact_signed_payload(&self) -> &[u8] {
        &self.signed_payload
    }

    #[must_use]
    pub const fn signature_bytes(&self) -> &[u8; 64] {
        &self.signature
    }

    #[must_use]
    pub const fn signer_public_key(&self) -> &[u8; 32] {
        &self.signer_public_key
    }
}

fn validate_production_fixture(fixture: &CapabilityFixture) -> Result<(), ProductionEvidenceError> {
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

fn validate_exact_vocabulary_and_policy(
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

fn configured_subject_stage(
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

fn configured_subject_evidence(
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

fn validate_fixture_structure(fixture: &CapabilityFixture) -> Result<(), ProductionEvidenceError> {
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

fn vocabulary_platform_contains(fixture: &CapabilityFixture, id: PlatformId) -> bool {
    fixture.vocabulary.platform_ids.contains(&id)
}

fn ensure_nonempty(
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

fn ensure_stable_id(
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

fn is_stable_id(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphanumeric())
        && bytes.all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'.' | b'_' | b':' | b'@' | b'+' | b'/' | b'-')
        })
}

fn ensure_unique<'a, T: Eq + Hash + fmt::Debug + 'a>(
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

fn invalid_structure<T>(
    fixture: &CapabilityFixture,
    detail: impl Into<String>,
) -> Result<T, ProductionEvidenceError> {
    Err(ProductionEvidenceError::InvalidStructure {
        fixture_id: fixture.fixture_id.clone(),
        detail: detail.into(),
    })
}

/// Immutable resolver. `Default` is the production-safe state: no provider,
/// no records, and therefore no affirmative claims.
#[derive(Debug, Clone, Copy, Default)]
pub struct CapabilityResolver<'a> {
    fixture: Option<&'a CapabilityFixture>,
}

impl<'a> CapabilityResolver<'a> {
    /// Bind only a privately verified production bundle. Raw fixtures have no
    /// API path into this constructor.
    #[must_use]
    pub const fn with_production_evidence(evidence: &'a VerifiedProductionEvidence) -> Self {
        Self {
            fixture: Some(&evidence.fixture),
        }
    }

    /// Resolve without mutation or inference from runtime/UI presence.
    ///
    /// `evaluated_at` must come from the caller's trusted current-time source.
    /// Requiring it on every call prevents a signed bundle from replaying its
    /// own historical `asOf` timestamp after evidence has expired.
    #[must_use]
    pub fn resolve_at(&self, claim: &CapabilityClaim, evaluated_at: Timestamp) -> ResolvedClaim {
        resolve_capability_claim(self.fixture, claim, evaluated_at)
    }

    /// Deterministic design-fixture convenience used only by contract tests.
    /// Production code has no no-clock resolution API.
    #[cfg(test)]
    fn resolve(&self, claim: &CapabilityClaim) -> ResolvedClaim {
        self.resolve_at(
            claim,
            self.fixture.map_or_else(
                || Timestamp::from("1970-01-01T00:00:00Z"),
                |fixture| fixture.as_of.clone(),
            ),
        )
    }

    #[cfg(test)]
    fn for_design_fixture(fixture: &'a CapabilityFixture) -> Self {
        debug_assert_eq!(fixture.status, FixtureStatus::DesignFixture);
        Self {
            fixture: Some(fixture),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct EvidenceRequirement {
    family: Option<ProtectedLabelFamily>,
    kind: EvidenceKind,
}

#[must_use]
pub fn classify_protected_labels(label: &str) -> Vec<ProtectedLabelFamily> {
    let lowercase = label.to_ascii_lowercase();
    let mut families = Vec::with_capacity(3);
    if ["release", "released", "release-ready", "release ready"]
        .iter()
        .any(|pattern| contains_bounded_ascii(&lowercase, pattern))
    {
        families.push(ProtectedLabelFamily::Release);
    }
    if ["qualified", "qualification"]
        .iter()
        .any(|pattern| contains_bounded_ascii(&lowercase, pattern))
    {
        families.push(ProtectedLabelFamily::Qualified);
    }
    if ["signoff", "sign-off", "sign off"]
        .iter()
        .any(|pattern| contains_bounded_ascii(&lowercase, pattern))
    {
        families.push(ProtectedLabelFamily::SignOff);
    }
    families
}

fn resolve_capability_claim(
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

fn claim_input_error(claim: &CapabilityClaim) -> Option<String> {
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

struct ResolutionParts {
    as_of: Nullable<Timestamp>,
    state: ClaimState,
    protected_label_families: Vec<ProtectedLabelFamily>,
    required_evidence_kinds: Vec<EvidenceKind>,
    source: Vec<EvidenceBindingSummary>,
    reasons: Vec<ResolutionReason>,
}

fn resolved_claim(claim: &CapabilityClaim, parts: ResolutionParts) -> ResolvedClaim {
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

fn required_evidence(
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

fn evaluate_claim_contract(
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

fn governed_product_label_minimum(claim: &CapabilityClaim) -> Option<CapabilityReadinessStage> {
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

fn enforce_readiness_floors(
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

fn evaluate_targets(
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

fn evaluate_target_verification(
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

fn evaluate_entitlement(
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

fn evaluate_record_state(
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

fn unknown_target(target_key: &str, id: &impl fmt::Display, reasons: &mut Vec<ResolutionReason>) {
    add_reason(
        reasons,
        ClaimState::Unknown,
        ReasonCode::TargetReferenceUnknown,
        format!("{target_key}:{id}"),
    );
}

fn evaluate_evidence(
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

fn evidence_authority_eligible(
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

fn evaluate_exact_target_binding(
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

fn binding_summary(
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

fn add_reason(
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

fn final_state(reasons: &[ResolutionReason]) -> ClaimState {
    reasons
        .iter()
        .map(|reason| reason.state)
        .min_by_key(|state| state.precedence_rank())
        .unwrap_or(ClaimState::Current)
}

fn unique_required_kinds(requirements: &[EvidenceRequirement]) -> Vec<EvidenceKind> {
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

fn canonical_stage_evidence(
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

const fn canonical_stage_for_evidence(kind: EvidenceKind) -> CapabilityReadinessStage {
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

fn canonical_minimum_stage(
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

fn canonical_protected_evidence(
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

fn contains_bounded_ascii(haystack: &str, needle: &str) -> bool {
    haystack.match_indices(needle).any(|(start, _)| {
        let end = start + needle.len();
        let before = haystack[..start].chars().next_back();
        let after = haystack[end..].chars().next();
        before.is_none_or(|character| !is_word_character(character))
            && after.is_none_or(|character| !is_word_character(character))
    })
}

const fn is_word_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

fn is_expired(expires_at: Option<&Timestamp>, as_of: Option<i64>) -> bool {
    match (expires_at.and_then(Timestamp::unix_seconds), as_of) {
        (Some(expires_at), Some(as_of)) => expires_at <= as_of,
        _ => false,
    }
}

fn timestamp_invalid(timestamp: Option<&Timestamp>) -> bool {
    timestamp.is_some_and(|timestamp| timestamp.unix_seconds().is_none())
}

fn parse_rfc3339_seconds(value: &str) -> Option<i64> {
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

fn parse_digits(bytes: &[u8], start: usize, length: usize) -> Option<u32> {
    let slice = bytes.get(start..start + length)?;
    if !slice.iter().all(u8::is_ascii_digit) {
        return None;
    }
    slice.iter().try_fold(0_u32, |value, digit| {
        value.checked_mul(10)?.checked_add(u32::from(*digit - b'0'))
    })
}

const fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Days since 1970-01-01, using Howard Hinnant's civil-date transform.
fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
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

const fn stage_name(stage: CapabilityReadinessStage) -> &'static str {
    match stage {
        CapabilityReadinessStage::Registered => "registered",
        CapabilityReadinessStage::DesignComplete => "design-complete",
        CapabilityReadinessStage::ImplementationReady => "implementation-ready",
        CapabilityReadinessStage::EngineReady => "engine-ready",
        CapabilityReadinessStage::PlatformQualified => "platform-qualified",
        CapabilityReadinessStage::SignOffEligible => "sign-off-eligible",
    }
}

const fn protected_family_name(family: ProtectedLabelFamily) -> &'static str {
    match family {
        ProtectedLabelFamily::Release => "release",
        ProtectedLabelFamily::Qualified => "qualified",
        ProtectedLabelFamily::SignOff => "sign-off",
    }
}

const fn evidence_kind_name(kind: EvidenceKind) -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    const CANONICAL_FIXTURE_JSON: &str = include_str!(
        "../../../../mockups/rspice-workbench-host/implementation/capability-readiness-fixture.json"
    );

    fn canonical_fixture() -> CapabilityFixture {
        serde_json::from_str(CANONICAL_FIXTURE_JSON)
            .expect("the canonical design fixture must match the closed Rust contract")
    }

    fn canonical_case<'a>(fixture: &'a CapabilityFixture, id: &str) -> &'a ResolutionCase {
        fixture
            .resolution_cases
            .iter()
            .find(|case| case.id.as_str() == id)
            .unwrap_or_else(|| panic!("missing canonical case {id}"))
    }

    fn applicability_for(targets: &ClaimTargets) -> EvidenceApplicability {
        EvidenceApplicability {
            status: ApplicabilityState::Applicable,
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
        }
    }

    fn signing_key() -> SigningKey {
        SigningKey::from_bytes(&[0x5a; 32])
    }

    fn signed_fixture_bytes(fixture: &CapabilityFixture) -> (Vec<u8>, [u8; 64], [u8; 32]) {
        let payload = serde_json::to_vec(fixture).expect("fixture serialization must succeed");
        let signing_key = signing_key();
        let signature = signing_key.sign(&payload).to_bytes();
        let public_key = signing_key.verifying_key().to_bytes();
        (payload, signature, public_key)
    }

    fn verify_fixture(
        fixture: &CapabilityFixture,
    ) -> Result<VerifiedProductionEvidence, ProductionEvidenceError> {
        let (payload, signature, public_key) = signed_fixture_bytes(fixture);
        VerifiedProductionEvidence::verify(&payload, &signature, &public_key)
    }

    fn production_base() -> (CapabilityFixture, CapabilityClaim) {
        let mut fixture = canonical_fixture();
        let claim = canonical_case(&fixture, "release-label-without-evidence").claim();
        fixture.schema = CAPABILITY_PRODUCTION_EVIDENCE_SCHEMA.into();
        fixture.schema_version = CAPABILITY_PRODUCTION_EVIDENCE_SCHEMA_VERSION.into();
        fixture.status = FixtureStatus::ProductionEvidence;
        fixture.fixture_id = "independently-authorized-production-evidence".into();
        fixture.build_identity.artifact_kind = "product-build".into();
        fixture.evidence.clear();
        fixture.resolution_cases.clear();
        (fixture, claim)
    }

    fn install_evidence(
        fixture: &mut CapabilityFixture,
        claim: &mut CapabilityClaim,
        kind: EvidenceKind,
        authority_class: AuthorityClass,
        eligible_label_families: Vec<ProtectedLabelFamily>,
    ) {
        fixture.evidence.clear();
        claim.evidence_binding_ids.clear();
        append_evidence(
            fixture,
            claim,
            "verified-current-exact-evidence",
            kind,
            authority_class,
            eligible_label_families,
        );
    }

    fn append_evidence(
        fixture: &mut CapabilityFixture,
        claim: &mut CapabilityClaim,
        id: &str,
        kind: EvidenceKind,
        authority_class: AuthorityClass,
        eligible_label_families: Vec<ProtectedLabelFamily>,
    ) {
        let evidence_id = EvidenceId::from(id);
        fixture.evidence.push(EvidenceRecord {
            id: evidence_id.clone(),
            kind,
            source: EvidenceSource {
                authority_class,
                locator: "evidence://independent-authority/exact-target".into(),
                revision: "verified-r1".into(),
            },
            verification: VerificationState::Verified,
            currentness: ClaimState::Current,
            issued_at: Nullable::some(Timestamp::from("2026-07-13T10:00:00Z")),
            expires_at: Nullable::some(Timestamp::from("2026-08-13T10:00:00Z")),
            applicability: applicability_for(&claim.targets),
            supports_stages: vec![canonical_stage_for_evidence(kind)],
            eligible_label_families,
            boundary: "Independently verified evidence for the exact target tuple only.".into(),
        });
        claim.evidence_binding_ids.push(evidence_id);
    }

    fn positive_release_bundle() -> (CapabilityFixture, CapabilityClaim) {
        let (mut fixture, mut claim) = production_base();
        claim.targets.platform_id = Nullable::some(PlatformId::Desktop);
        claim.targets.compute_target_id = Nullable::some(ComputeTargetId::from("local-desktop-12"));
        claim.targets.engine_id =
            Nullable::some(EngineId::from("engine-rspice-native-fixture-0.1.0+91e7c2a"));
        claim.targets.model_binding_id =
            Nullable::some(ModelBindingId::from("model-binding-demo180-2.3.1"));
        let native_engine = fixture
            .engines
            .iter_mut()
            .find(|engine| engine.mode == EngineMode::Native)
            .expect("production test fixture has a native engine");
        native_engine.verification = VerificationState::Verified;
        native_engine.declared_readiness = CapabilityReadinessStage::EngineReady;
        native_engine.build_id = fixture.build_identity.engine_build_fixture_id.clone();
        let model = fixture
            .model_qualifications
            .first_mut()
            .expect("production test fixture has a model binding");
        model.verification = VerificationState::Verified;
        model.qualification_evidence_id = "verified-model-qualification".into();
        fixture.evidence.clear();
        claim.evidence_binding_ids.clear();
        append_evidence(
            &mut fixture,
            &mut claim,
            "verified-release-qualification",
            EvidenceKind::ReleaseQualification,
            AuthorityClass::ReleaseEvidence,
            vec![ProtectedLabelFamily::Release],
        );
        append_evidence(
            &mut fixture,
            &mut claim,
            "verified-design-contract",
            EvidenceKind::DesignContract,
            AuthorityClass::DesignContract,
            Vec::new(),
        );
        append_evidence(
            &mut fixture,
            &mut claim,
            "verified-implementation-conformance",
            EvidenceKind::ImplementationConformance,
            AuthorityClass::ImplementationEvidence,
            Vec::new(),
        );
        append_evidence(
            &mut fixture,
            &mut claim,
            "verified-engine-conformance",
            EvidenceKind::EngineConformance,
            AuthorityClass::ImplementationEvidence,
            Vec::new(),
        );
        append_evidence(
            &mut fixture,
            &mut claim,
            "verified-platform-qualification",
            EvidenceKind::PlatformQualification,
            AuthorityClass::ReleaseEvidence,
            Vec::new(),
        );
        append_evidence(
            &mut fixture,
            &mut claim,
            "verified-model-qualification",
            EvidenceKind::ModelQualification,
            AuthorityClass::ReleaseEvidence,
            Vec::new(),
        );
        (fixture, claim)
    }

    fn selected_evidence_mut<'a>(
        fixture: &'a mut CapabilityFixture,
        claim: &CapabilityClaim,
    ) -> &'a mut EvidenceRecord {
        let selected_id = &claim.evidence_binding_ids[0];
        fixture
            .evidence
            .iter_mut()
            .find(|record| &record.id == selected_id)
            .expect("the test bundle must contain its selected evidence")
    }

    fn assert_invalid_structure(error: ProductionEvidenceError) {
        assert!(
            matches!(error, ProductionEvidenceError::InvalidStructure { .. }),
            "expected structural rejection, got {error:?}"
        );
    }

    #[test]
    fn canonical_fixture_deserializes_with_exact_six_stage_vocabulary() {
        let fixture = canonical_fixture();
        assert_eq!(fixture.status, FixtureStatus::DesignFixture);
        assert_eq!(
            fixture.vocabulary.readiness_stages,
            CapabilityReadinessStage::ALL
        );
        assert_eq!(fixture.resolution_cases.len(), 14);
        assert_eq!(
            fixture.claim_resolution_algorithm.precedence,
            [
                ClaimState::Unavailable,
                ClaimState::Unknown,
                ClaimState::Stale,
                ClaimState::Current,
            ]
        );
        assert_eq!(fixture.build_identity.artifact_kind, "gui-design-mockup");
        assert!(fixture.boundary.does_not_claim.contains("does not prove"));
    }

    #[test]
    fn canonical_fourteen_cases_match_every_expected_branch() {
        let fixture = canonical_fixture();
        let resolver = CapabilityResolver::for_design_fixture(&fixture);
        let first = fixture
            .resolution_cases
            .iter()
            .map(|case| resolver.resolve(&case.claim()))
            .collect::<Vec<_>>();
        let second = fixture
            .resolution_cases
            .iter()
            .map(|case| resolver.resolve(&case.claim()))
            .collect::<Vec<_>>();
        assert_eq!(first, second, "resolution must be deterministic");

        for (case, resolved) in fixture.resolution_cases.iter().zip(&first) {
            assert_eq!(resolved.case_id.as_ref(), Some(&case.id));
            assert_eq!(resolved.claim_mode, case.claim_mode, "{}", case.id);
            assert_eq!(resolved.state, case.expected.state, "{}", case.id);
            assert_eq!(
                resolved.label_allowed,
                case.claim_mode == ClaimMode::Product && case.expected.label_allowed,
                "{}",
                case.id
            );
            for reason in &case.expected.required_reason_codes {
                assert!(
                    resolved.reason_codes.contains(reason),
                    "{} is missing {reason:?}; actual={:?}",
                    case.id,
                    resolved.reason_codes
                );
            }
            assert!(resolved.applicability.exact_target_binding_required);
            assert_eq!(resolved.currentness.state, resolved.state);
        }

        let states = first
            .iter()
            .map(|result| result.state)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(
            states,
            [
                ClaimState::Current,
                ClaimState::Stale,
                ClaimState::Unavailable,
                ClaimState::Unknown,
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn protected_words_are_derived_from_customer_copy_with_word_boundaries() {
        assert_eq!(
            classify_protected_labels("Production release ready"),
            [ProtectedLabelFamily::Release]
        );
        assert_eq!(
            classify_protected_labels("Platform qualified"),
            [ProtectedLabelFamily::Qualified]
        );
        assert_eq!(
            classify_protected_labels("Sign-off eligible"),
            [ProtectedLabelFamily::SignOff]
        );
        assert_eq!(
            classify_protected_labels("Release qualification and sign off"),
            [
                ProtectedLabelFamily::Release,
                ProtectedLabelFamily::Qualified,
                ProtectedLabelFamily::SignOff,
            ]
        );
        assert!(classify_protected_labels("Design complete").is_empty());
        assert!(classify_protected_labels("prereleasecandidate").is_empty());
        assert!(classify_protected_labels("unqualifiedly").is_empty());
    }

    #[test]
    fn product_protected_cases_never_accept_contract_test_authority() {
        let fixture = canonical_fixture();
        let resolver = CapabilityResolver::for_design_fixture(&fixture);
        let vector = canonical_case(&fixture, "positive-qualified-contract-test-vector");
        let positive = resolver.resolve(&vector.claim());
        assert_eq!(positive.state, ClaimState::Current);
        assert_eq!(positive.claim_mode, ClaimMode::ContractTestVector);
        assert!(!positive.label_allowed);

        let mut product_copy = vector.claim();
        product_copy.claim_mode = ClaimMode::Product;
        let rejected = resolver.resolve(&product_copy);
        assert_eq!(rejected.state, ClaimState::Unknown);
        assert!(!rejected.label_allowed);
        assert!(
            rejected
                .reason_codes
                .contains(&ReasonCode::EvidenceAuthorityIneligible)
        );
    }

    #[test]
    fn production_default_has_no_provider_and_all_claims_fail_closed() {
        let fixture = canonical_fixture();
        let claim = canonical_case(&fixture, "current-design-contract").claim();
        let resolved = CapabilityResolver::default().resolve(&claim);
        assert_eq!(resolved.state, ClaimState::Unknown);
        assert!(!resolved.label_allowed);
        assert_eq!(resolved.as_of, Nullable::none());
        assert_eq!(
            resolved.reason_codes,
            [ReasonCode::EvidenceProviderUnavailable]
        );
        assert!(resolved.source.is_empty());
    }

    #[test]
    fn signed_design_fixture_cannot_cross_the_production_boundary() {
        let fixture = canonical_fixture();
        let error = verify_fixture(&fixture)
            .expect_err("design fixtures must never become production providers");
        assert_eq!(
            error,
            ProductionEvidenceError::NonProductionFixture {
                fixture_id: fixture.fixture_id.clone(),
                actual_status: FixtureStatus::DesignFixture,
            }
        );
    }

    #[test]
    fn exact_verified_current_applicable_release_authority_is_the_positive_product_path() {
        let (fixture, claim) = positive_release_bundle();
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let resolver = CapabilityResolver::with_production_evidence(&verified);
        let resolved = resolver.resolve(&claim);
        assert_eq!(resolved.state, ClaimState::Current);
        assert!(resolved.label_allowed);
        assert_eq!(
            resolved.required_evidence_kinds,
            [
                EvidenceKind::DesignContract,
                EvidenceKind::ImplementationConformance,
                EvidenceKind::EngineConformance,
                EvidenceKind::PlatformQualification,
                EvidenceKind::ModelQualification,
                EvidenceKind::ReleaseQualification,
            ]
        );
        assert_eq!(resolved.source.len(), 6);
        assert!(resolved.source.iter().all(|source| source.selected));
        assert_eq!(
            resolved.source[0]
                .source
                .as_ref()
                .map(|source| source.authority_class),
            Some(AuthorityClass::ReleaseEvidence)
        );
        assert!(resolved.reason_codes.is_empty());
    }

    #[test]
    fn verified_bundle_retains_the_exact_authenticated_payload_and_trust_anchor() {
        let (fixture, _) = positive_release_bundle();
        let (payload, signature, public_key) = signed_fixture_bytes(&fixture);
        let verified = VerifiedProductionEvidence::verify(&payload, &signature, &public_key)
            .expect("valid signature and production fixture");
        assert_eq!(verified.fixture_id(), fixture.fixture_id);
        assert_eq!(verified.exact_signed_payload(), payload);
        assert_eq!(verified.signature_bytes(), &signature);
        assert_eq!(verified.signer_public_key(), &public_key);
    }

    #[test]
    fn signature_verification_rejects_wrong_key_signature_tampering_and_payload_tampering() {
        let (fixture, _) = positive_release_bundle();
        let (payload, signature, public_key) = signed_fixture_bytes(&fixture);

        let wrong_key = SigningKey::from_bytes(&[0x6b; 32])
            .verifying_key()
            .to_bytes();
        assert_eq!(
            VerifiedProductionEvidence::verify(&payload, &signature, &wrong_key).unwrap_err(),
            ProductionEvidenceError::InvalidSignature
        );

        let mut bad_signature = signature;
        bad_signature[7] ^= 0x80;
        assert_eq!(
            VerifiedProductionEvidence::verify(&payload, &bad_signature, &public_key).unwrap_err(),
            ProductionEvidenceError::InvalidSignature
        );

        let mut tampered_payload = payload;
        let last = tampered_payload.len() - 1;
        tampered_payload[last] ^= 0x01;
        assert_eq!(
            VerifiedProductionEvidence::verify(&tampered_payload, &signature, &public_key)
                .unwrap_err(),
            ProductionEvidenceError::InvalidSignature
        );
    }

    #[test]
    fn authenticated_malformed_and_oversize_payloads_fail_closed() {
        let signing_key = signing_key();
        let malformed = b"{";
        let signature = signing_key.sign(malformed).to_bytes();
        let public_key = signing_key.verifying_key().to_bytes();
        assert!(matches!(
            VerifiedProductionEvidence::verify(malformed, &signature, &public_key),
            Err(ProductionEvidenceError::MalformedPayload(_))
        ));

        let oversize = vec![b' '; MAX_CAPABILITY_EVIDENCE_PAYLOAD_BYTES + 1];
        assert_eq!(
            VerifiedProductionEvidence::verify(&oversize, &[0; 64], &public_key).unwrap_err(),
            ProductionEvidenceError::PayloadTooLarge {
                actual_bytes: MAX_CAPABILITY_EVIDENCE_PAYLOAD_BYTES + 1,
                maximum_bytes: MAX_CAPABILITY_EVIDENCE_PAYLOAD_BYTES,
            }
        );
    }

    #[test]
    fn production_bundle_requires_exact_schema_and_canonical_policy() {
        let (mut fixture, _) = positive_release_bundle();
        fixture.schema = "https://attacker.invalid/permissive.json".into();
        assert!(matches!(
            verify_fixture(&fixture),
            Err(ProductionEvidenceError::InvalidSchema { .. })
        ));

        let (mut fixture, _) = positive_release_bundle();
        fixture
            .vocabulary
            .readiness_stages
            .swap(0, CapabilityReadinessStage::ALL.len() - 1);
        assert!(matches!(
            verify_fixture(&fixture),
            Err(ProductionEvidenceError::InvalidReadinessVocabulary { .. })
        ));

        let (mut fixture, _) = positive_release_bundle();
        fixture.claim_resolution_algorithm.steps[0].rule.clear();
        assert_invalid_structure(verify_fixture(&fixture).unwrap_err());
    }

    #[test]
    fn production_bundle_rejects_duplicate_dangling_and_test_vector_records() {
        let (mut fixture, _) = positive_release_bundle();
        fixture.evidence.push(fixture.evidence[0].clone());
        assert_invalid_structure(verify_fixture(&fixture).unwrap_err());

        let (mut fixture, _) = positive_release_bundle();
        fixture.evidence[0].applicability.build_ids = vec![BuildId::from("unknown-build")];
        assert_invalid_structure(verify_fixture(&fixture).unwrap_err());

        let (mut fixture, _) = positive_release_bundle();
        fixture.connectors[0].producer_id = ExternalProducerId::from("unknown-producer");
        assert_invalid_structure(verify_fixture(&fixture).unwrap_err());

        let (mut fixture, _) = positive_release_bundle();
        fixture.evidence[0].source.authority_class = AuthorityClass::ContractTestVector;
        assert_invalid_structure(verify_fixture(&fixture).unwrap_err());

        let (mut fixture, _) = positive_release_bundle();
        let design_fixture = canonical_fixture();
        fixture
            .resolution_cases
            .push(design_fixture.resolution_cases[0].clone());
        assert_invalid_structure(verify_fixture(&fixture).unwrap_err());
    }

    #[test]
    fn no_affirmative_protected_product_claim_without_every_release_gate() {
        type Mutation = fn(&mut CapabilityFixture, &mut CapabilityClaim);
        let vectors: &[(Mutation, ClaimState, ReasonCode)] = &[
            (
                |_, claim| claim.evidence_binding_ids.clear(),
                ClaimState::Unknown,
                ReasonCode::MissingRequiredEvidence,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim).source.authority_class =
                        AuthorityClass::ImplementationEvidence;
                },
                ClaimState::Unknown,
                ReasonCode::EvidenceAuthorityIneligible,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim).verification =
                        VerificationState::Unverified;
                },
                ClaimState::Unknown,
                ReasonCode::EvidenceUnverified,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim).currentness = ClaimState::Stale;
                },
                ClaimState::Stale,
                ReasonCode::EvidenceStale,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim).applicability.status =
                        ApplicabilityState::NotApplicable;
                },
                ClaimState::Unavailable,
                ReasonCode::EvidenceNotApplicable,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim)
                        .applicability
                        .build_ids
                        .clear();
                },
                ClaimState::Stale,
                ReasonCode::EvidenceTargetMismatch,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim)
                        .applicability
                        .platform_ids
                        .clear();
                },
                ClaimState::Stale,
                ReasonCode::EvidenceTargetMismatch,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim).expires_at =
                        Nullable::some(Timestamp::from("2026-07-13T11:00:00Z"));
                },
                ClaimState::Stale,
                ReasonCode::EvidenceExpired,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim).expires_at =
                        Nullable::some(Timestamp::from("not-a-time"));
                },
                ClaimState::Unknown,
                ReasonCode::EvidenceTimestampInvalid,
            ),
            (
                |fixture, claim| {
                    selected_evidence_mut(fixture, claim).issued_at = Nullable::none();
                },
                ClaimState::Unknown,
                ReasonCode::EvidenceTimestampInvalid,
            ),
        ];

        for (mutate, expected_state, expected_reason) in vectors {
            let (mut fixture, mut claim) = positive_release_bundle();
            mutate(&mut fixture, &mut claim);
            let verified = verify_fixture(&fixture).expect("structurally valid mutation");
            let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
            assert_eq!(result.state, *expected_state, "{expected_reason:?}");
            assert!(!result.label_allowed, "{expected_reason:?}");
            assert!(result.reason_codes.contains(expected_reason));
        }
    }

    #[test]
    fn unavailable_has_priority_over_unknown_stale_and_current() {
        let (mut fixture, claim) = positive_release_bundle();
        let evidence = fixture
            .evidence
            .iter_mut()
            .find(|evidence| evidence.id == claim.evidence_binding_ids[0])
            .unwrap();
        evidence.verification = VerificationState::Unknown;
        evidence.currentness = ClaimState::Stale;
        evidence.applicability.status = ApplicabilityState::NotApplicable;
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unavailable);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::EvidenceVerificationUnknown)
        );
        assert!(result.reason_codes.contains(&ReasonCode::EvidenceStale));
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::EvidenceNotApplicable)
        );
    }

    #[test]
    fn invalid_or_expired_time_fails_closed_without_using_wall_clock_time() {
        let (fixture, claim) = positive_release_bundle();
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let resolver = CapabilityResolver::with_production_evidence(&verified);

        let invalid = resolver.resolve_at(&claim, Timestamp::from("not-a-time"));
        assert_eq!(invalid.state, ClaimState::Unknown);
        assert!(invalid.reason_codes.contains(&ReasonCode::InvalidAsOf));

        let expired = resolver.resolve_at(&claim, Timestamp::from("2026-08-13T10:00:00Z"));
        assert_eq!(expired.state, ClaimState::Stale);
        assert!(expired.reason_codes.contains(&ReasonCode::EvidenceExpired));

        let before_expiry = resolver.resolve_at(&claim, Timestamp::from("2026-08-13T09:59:59Z"));
        assert_eq!(before_expiry.state, ClaimState::Current);
    }

    #[test]
    fn future_issued_and_inverted_evidence_intervals_fail_closed() {
        let (mut fixture, claim) = positive_release_bundle();
        selected_evidence_mut(&mut fixture, &claim).issued_at =
            Nullable::some(Timestamp::from("2026-07-13T13:00:00Z"));
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unknown);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::EvidenceNotYetValid)
        );

        let (mut fixture, claim) = positive_release_bundle();
        let evidence = selected_evidence_mut(&mut fixture, &claim);
        evidence.issued_at = Nullable::some(Timestamp::from("2026-08-14T10:00:00Z"));
        evidence.expires_at = Nullable::some(Timestamp::from("2026-08-13T10:00:00Z"));
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unknown);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::EvidenceTimestampInvalid)
        );
        assert!(
            !result
                .reason_codes
                .contains(&ReasonCode::EvidenceNotYetValid)
        );
    }

    #[test]
    fn only_governed_non_registered_product_claims_can_render_customer_labels() {
        let (fixture, mut claim) = positive_release_bundle();
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let resolver = CapabilityResolver::with_production_evidence(&verified);

        claim.label = "Commercially available".into();
        let ungoverned = resolver.resolve(&claim);
        assert_eq!(ungoverned.state, ClaimState::Unknown);
        assert!(!ungoverned.label_allowed);
        assert!(
            ungoverned
                .reason_codes
                .contains(&ReasonCode::UngovernedProductLabel)
        );

        claim.label = "Design complete".into();
        claim.subject_kind = SubjectKind::Design;
        claim.asserted_stage = CapabilityReadinessStage::Registered;
        claim.evidence_binding_ids.clear();
        let registered = resolver.resolve(&claim);
        assert_eq!(registered.state, ClaimState::Unknown);
        assert!(!registered.label_allowed);
        assert!(
            registered
                .reason_codes
                .contains(&ReasonCode::RegisteredProductClaimForbidden)
        );
    }

    #[test]
    fn governed_product_labels_cannot_be_backed_by_a_lower_readiness_stage() {
        let (mut fixture, mut claim) = production_base();
        claim.subject_kind = SubjectKind::Build;
        claim.label = "Implementation ready".into();
        claim.asserted_stage = CapabilityReadinessStage::DesignComplete;
        install_evidence(
            &mut fixture,
            &mut claim,
            EvidenceKind::DesignContract,
            AuthorityClass::DesignContract,
            vec![],
        );
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unknown);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::ReadinessStageInsufficient)
        );
        assert!(!result.label_allowed);
    }

    #[test]
    fn baseline_and_subject_selectors_are_mandatory() {
        type TargetMutation = fn(&mut ClaimTargets);
        let baseline: &[(&str, TargetMutation)] = &[
            ("buildId", |targets| targets.build_id = Nullable::none()),
            ("entitlementPolicyId", |targets| {
                targets.entitlement_policy_id = Nullable::none();
            }),
            ("canonicalObjectId", |targets| {
                targets.canonical_object_id = Nullable::none();
            }),
        ];
        for (name, mutate) in baseline {
            let (fixture, mut claim) = positive_release_bundle();
            mutate(&mut claim.targets);
            let verified = verify_fixture(&fixture).expect("valid signed production evidence");
            let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
            assert_eq!(result.state, ClaimState::Unknown, "{name}");
            assert!(!result.label_allowed, "{name}");
            assert!(
                result
                    .reason_codes
                    .contains(&ReasonCode::RequiredTargetMissing),
                "{name}: {:?}",
                result.reason_codes
            );
        }

        let subjects = [
            (
                SubjectKind::Engine,
                "Engine ready",
                CapabilityReadinessStage::EngineReady,
            ),
            (
                SubjectKind::Model,
                "Model qualified",
                CapabilityReadinessStage::EngineReady,
            ),
            (
                SubjectKind::Platform,
                "Platform qualified",
                CapabilityReadinessStage::PlatformQualified,
            ),
            (
                SubjectKind::ExternalProducer,
                "External producer qualified",
                CapabilityReadinessStage::EngineReady,
            ),
            (
                SubjectKind::Connector,
                "Connector qualified",
                CapabilityReadinessStage::PlatformQualified,
            ),
            (
                SubjectKind::SignOff,
                "Sign-off eligible",
                CapabilityReadinessStage::SignOffEligible,
            ),
        ];
        for (subject, label, stage) in subjects {
            let (fixture, mut claim) = positive_release_bundle();
            claim.subject_kind = subject;
            claim.label = label.into();
            claim.asserted_stage = stage;
            match subject {
                SubjectKind::Engine => claim.targets.engine_id = Nullable::none(),
                SubjectKind::Model => claim.targets.model_binding_id = Nullable::none(),
                SubjectKind::Platform => claim.targets.platform_id = Nullable::none(),
                SubjectKind::ExternalProducer => {
                    claim.targets.external_producer_id = Nullable::none();
                }
                SubjectKind::Connector => claim.targets.connector_id = Nullable::none(),
                SubjectKind::SignOff => claim.targets.compute_target_id = Nullable::none(),
                SubjectKind::Design | SubjectKind::Build | SubjectKind::Release => {}
            }
            let verified = verify_fixture(&fixture).expect("valid signed production evidence");
            let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
            assert!(
                result
                    .reason_codes
                    .contains(&ReasonCode::RequiredTargetMissing),
                "{subject:?}: {:?}",
                result.reason_codes
            );
            assert!(!result.label_allowed, "{subject:?}");
        }
    }

    #[test]
    fn selected_target_tuples_must_match_fixture_relationships() {
        let (mut fixture, mut claim) = production_base();
        claim.subject_kind = SubjectKind::Engine;
        claim.label = "Engine ready".into();
        claim.asserted_stage = CapabilityReadinessStage::EngineReady;
        claim.targets.compute_target_id = Nullable::some(ComputeTargetId::from("local-desktop-12"));
        claim.targets.engine_id = Nullable::some(EngineId::from(
            "engine-rspice-preview-fixture-0.1.0+91e7c2a",
        ));
        let preview = fixture
            .engines
            .iter_mut()
            .find(|engine| engine.id == *claim.targets.engine_id.as_ref().unwrap())
            .unwrap();
        preview.verification = VerificationState::Verified;
        preview.declared_readiness = CapabilityReadinessStage::EngineReady;
        install_evidence(
            &mut fixture,
            &mut claim,
            EvidenceKind::EngineConformance,
            AuthorityClass::ImplementationEvidence,
            vec![],
        );
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unavailable);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::TargetRelationshipMismatch)
        );

        let (mut fixture, mut claim) = production_base();
        claim.subject_kind = SubjectKind::Connector;
        claim.label = "Connector qualified".into();
        claim.asserted_stage = CapabilityReadinessStage::PlatformQualified;
        claim.targets.connector_id = Nullable::some(ConnectorId::from("connector-electrothermal"));
        claim.targets.external_producer_id =
            Nullable::some(ExternalProducerId::from("external-producer-photonics"));
        install_evidence(
            &mut fixture,
            &mut claim,
            EvidenceKind::ExternalProducerQualification,
            AuthorityClass::ReleaseEvidence,
            vec![ProtectedLabelFamily::Qualified],
        );
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unavailable);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::TargetRelationshipMismatch)
        );
    }

    #[test]
    fn evidence_kind_authority_matrix_is_exact_for_product_and_test_vectors() {
        let cases = [
            (
                EvidenceKind::DesignContract,
                vec![
                    AuthorityClass::DesignContract,
                    AuthorityClass::ReleaseEvidence,
                ],
            ),
            (
                EvidenceKind::ImplementationConformance,
                vec![
                    AuthorityClass::ImplementationEvidence,
                    AuthorityClass::ReleaseEvidence,
                ],
            ),
            (
                EvidenceKind::EngineConformance,
                vec![
                    AuthorityClass::ImplementationEvidence,
                    AuthorityClass::ReleaseEvidence,
                ],
            ),
            (
                EvidenceKind::ModelQualification,
                vec![AuthorityClass::ReleaseEvidence],
            ),
            (
                EvidenceKind::PlatformQualification,
                vec![AuthorityClass::ReleaseEvidence],
            ),
            (
                EvidenceKind::ExternalProducerQualification,
                vec![AuthorityClass::ReleaseEvidence],
            ),
            (
                EvidenceKind::ReleaseQualification,
                vec![AuthorityClass::ReleaseEvidence],
            ),
            (
                EvidenceKind::SignOffApproval,
                vec![AuthorityClass::ReleaseEvidence],
            ),
        ];
        let authorities = [
            AuthorityClass::DesignContract,
            AuthorityClass::ImplementationEvidence,
            AuthorityClass::ReleaseEvidence,
            AuthorityClass::ContractTestVector,
        ];
        for (kind, eligible) in cases {
            for authority in authorities {
                assert_eq!(
                    evidence_authority_eligible(kind, authority, ClaimMode::Product),
                    eligible.contains(&authority),
                    "product {kind:?}/{authority:?}"
                );
                assert_eq!(
                    evidence_authority_eligible(kind, authority, ClaimMode::ContractTestVector),
                    authority == AuthorityClass::ContractTestVector,
                    "test vector {kind:?}/{authority:?}"
                );
            }
        }
    }

    #[test]
    fn unprotected_product_evidence_still_requires_kind_appropriate_authority() {
        let (mut fixture, mut claim) = production_base();
        claim.subject_kind = SubjectKind::Engine;
        claim.label = "Engine ready".into();
        claim.asserted_stage = CapabilityReadinessStage::EngineReady;
        claim.targets.engine_id =
            Nullable::some(EngineId::from("engine-rspice-native-fixture-0.1.0+91e7c2a"));
        let engine = fixture
            .engines
            .iter_mut()
            .find(|engine| engine.id == *claim.targets.engine_id.as_ref().unwrap())
            .unwrap();
        engine.verification = VerificationState::Verified;
        engine.declared_readiness = CapabilityReadinessStage::EngineReady;
        install_evidence(
            &mut fixture,
            &mut claim,
            EvidenceKind::EngineConformance,
            AuthorityClass::DesignContract,
            vec![],
        );
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unknown);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::EvidenceAuthorityIneligible)
        );
        assert!(!result.label_allowed);
    }

    #[test]
    fn selected_engine_requires_verified_state_and_declared_readiness() {
        let verification_cases = [
            (
                VerificationState::Unverified,
                ClaimState::Unknown,
                ReasonCode::TargetVerificationUnverified,
            ),
            (
                VerificationState::Unknown,
                ClaimState::Unknown,
                ReasonCode::TargetVerificationUnknown,
            ),
            (
                VerificationState::Rejected,
                ClaimState::Unavailable,
                ReasonCode::TargetVerificationRejected,
            ),
        ];
        for (verification, expected_state, expected_reason) in verification_cases {
            let (mut fixture, mut claim) = production_base();
            claim.subject_kind = SubjectKind::Engine;
            claim.label = "Engine ready".into();
            claim.asserted_stage = CapabilityReadinessStage::EngineReady;
            claim.targets.engine_id =
                Nullable::some(EngineId::from("engine-rspice-native-fixture-0.1.0+91e7c2a"));
            let engine = fixture
                .engines
                .iter_mut()
                .find(|engine| engine.id == *claim.targets.engine_id.as_ref().unwrap())
                .unwrap();
            engine.verification = verification;
            engine.declared_readiness = CapabilityReadinessStage::EngineReady;
            install_evidence(
                &mut fixture,
                &mut claim,
                EvidenceKind::EngineConformance,
                AuthorityClass::ImplementationEvidence,
                vec![],
            );
            let verified = verify_fixture(&fixture).expect("valid signed production evidence");
            let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
            assert_eq!(result.state, expected_state, "{verification:?}");
            assert!(result.reason_codes.contains(&expected_reason));
            assert!(!result.label_allowed);
        }

        let (mut fixture, mut claim) = production_base();
        claim.subject_kind = SubjectKind::Engine;
        claim.label = "Engine ready".into();
        claim.asserted_stage = CapabilityReadinessStage::EngineReady;
        claim.targets.engine_id =
            Nullable::some(EngineId::from("engine-rspice-native-fixture-0.1.0+91e7c2a"));
        let engine = fixture
            .engines
            .iter_mut()
            .find(|engine| engine.id == *claim.targets.engine_id.as_ref().unwrap())
            .unwrap();
        engine.verification = VerificationState::Verified;
        engine.declared_readiness = CapabilityReadinessStage::Registered;
        install_evidence(
            &mut fixture,
            &mut claim,
            EvidenceKind::EngineConformance,
            AuthorityClass::ImplementationEvidence,
            vec![],
        );
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unknown);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::TargetReadinessInsufficient)
        );
    }

    #[test]
    fn selected_model_and_external_producer_must_be_verified() {
        type TargetSelection = fn(&mut ClaimTargets);
        let cases: &[(SubjectKind, &str, EvidenceKind, TargetSelection)] = &[
            (
                SubjectKind::Model,
                "Model qualified",
                EvidenceKind::ModelQualification,
                |targets| {
                    targets.model_binding_id =
                        Nullable::some(ModelBindingId::from("model-binding-demo180-2.3.1"));
                },
            ),
            (
                SubjectKind::ExternalProducer,
                "External producer qualified",
                EvidenceKind::ExternalProducerQualification,
                |targets| {
                    targets.external_producer_id = Nullable::some(ExternalProducerId::from(
                        "external-producer-electrothermal",
                    ));
                },
            ),
        ];
        for (subject, label, evidence_kind, select_target) in cases {
            let (mut fixture, mut claim) = production_base();
            claim.subject_kind = *subject;
            claim.label = (*label).into();
            claim.asserted_stage = CapabilityReadinessStage::EngineReady;
            select_target(&mut claim.targets);
            install_evidence(
                &mut fixture,
                &mut claim,
                *evidence_kind,
                AuthorityClass::ReleaseEvidence,
                vec![ProtectedLabelFamily::Qualified],
            );
            let verified = verify_fixture(&fixture).expect("valid signed production evidence");
            let result = CapabilityResolver::with_production_evidence(&verified).resolve(&claim);
            assert_eq!(result.state, ClaimState::Unknown, "{subject:?}");
            assert!(
                result
                    .reason_codes
                    .contains(&ReasonCode::TargetVerificationUnverified),
                "{subject:?}: {:?}",
                result.reason_codes
            );
            assert!(!result.label_allowed, "{subject:?}");
        }
    }

    #[test]
    fn unknown_evidence_is_retained_as_an_auditable_source_binding() {
        let fixture = canonical_fixture();
        let mut claim = canonical_case(&fixture, "current-design-contract").claim();
        claim.evidence_binding_ids = vec![EvidenceId::from("missing-evidence")];
        let result = CapabilityResolver::for_design_fixture(&fixture).resolve(&claim);
        assert_eq!(result.state, ClaimState::Unknown);
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::EvidenceReferenceUnknown)
        );
        assert!(
            result
                .reason_codes
                .contains(&ReasonCode::MissingRequiredEvidence)
        );
        assert_eq!(result.source.len(), 1);
        assert_eq!(
            result.source[0].id,
            Some(EvidenceId::from("missing-evidence"))
        );
        assert_eq!(result.source[0].source, None);
        assert_eq!(result.source[0].currentness, ClaimState::Unknown);
    }

    #[test]
    fn production_resolution_requires_the_cumulative_stage_evidence_chain() {
        for missing in [
            EvidenceKind::DesignContract,
            EvidenceKind::ImplementationConformance,
            EvidenceKind::EngineConformance,
            EvidenceKind::PlatformQualification,
            EvidenceKind::ModelQualification,
        ] {
            let (mut fixture, mut claim) = positive_release_bundle();
            let removed_ids = fixture
                .evidence
                .iter()
                .filter(|evidence| evidence.kind == missing)
                .map(|evidence| evidence.id.clone())
                .collect::<Vec<_>>();
            fixture.evidence.retain(|evidence| evidence.kind != missing);
            claim
                .evidence_binding_ids
                .retain(|id| !removed_ids.contains(id));
            let verified = verify_fixture(&fixture).expect("signed bundle remains structural");
            let resolved = CapabilityResolver::with_production_evidence(&verified)
                .resolve_at(&claim, fixture.as_of.clone());
            assert_eq!(resolved.state, ClaimState::Unknown, "{missing:?}");
            assert!(!resolved.label_allowed, "{missing:?}");
            assert!(
                resolved
                    .reason_codes
                    .contains(&ReasonCode::MissingRequiredEvidence),
                "{missing:?}: {:?}",
                resolved.reason_codes
            );
        }
    }

    #[test]
    fn asserted_signoff_stage_cannot_reuse_platform_evidence() {
        let (fixture, mut claim) = positive_release_bundle();
        claim.subject_kind = SubjectKind::Platform;
        claim.label = "Platform qualified".into();
        claim.asserted_stage = CapabilityReadinessStage::SignOffEligible;
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let resolved = CapabilityResolver::with_production_evidence(&verified)
            .resolve_at(&claim, fixture.as_of.clone());
        assert_eq!(resolved.state, ClaimState::Unknown);
        assert!(!resolved.label_allowed);
        assert!(
            resolved
                .required_evidence_kinds
                .contains(&EvidenceKind::SignOffApproval)
        );
        assert!(
            resolved
                .reason_codes
                .contains(&ReasonCode::MissingRequiredEvidence)
        );
    }

    #[test]
    fn platform_and_release_stages_require_the_exact_execution_tuple() {
        type TargetMutation = fn(&mut ClaimTargets);
        for (selector, clear) in [
            (
                "platformId",
                (|targets: &mut ClaimTargets| targets.platform_id = Nullable::none())
                    as TargetMutation,
            ),
            ("computeTargetId", |targets: &mut ClaimTargets| {
                targets.compute_target_id = Nullable::none();
            }),
            ("engineId", |targets: &mut ClaimTargets| {
                targets.engine_id = Nullable::none();
            }),
            ("modelBindingId", |targets: &mut ClaimTargets| {
                targets.model_binding_id = Nullable::none();
            }),
        ] {
            let (fixture, mut claim) = positive_release_bundle();
            clear(&mut claim.targets);
            let verified = verify_fixture(&fixture).expect("valid signed production evidence");
            let resolved = CapabilityResolver::with_production_evidence(&verified)
                .resolve_at(&claim, fixture.as_of.clone());
            assert_eq!(resolved.state, ClaimState::Unknown, "{selector}");
            assert!(!resolved.label_allowed, "{selector}");
            assert!(
                resolved
                    .reason_codes
                    .contains(&ReasonCode::RequiredTargetMissing),
                "{selector}: {:?}",
                resolved.reason_codes
            );
        }
    }

    #[test]
    fn preview_or_compatibility_engine_mode_cannot_be_promoted_by_signed_flags() {
        for mode in [EngineMode::Preview, EngineMode::Compatibility] {
            let (mut fixture, claim) = positive_release_bundle();
            let engine_id = claim.targets.engine_id.as_ref().unwrap();
            let engine = fixture
                .engines
                .iter_mut()
                .find(|engine| &engine.id == engine_id)
                .unwrap();
            engine.mode = mode;
            engine.state = ClaimState::Current;
            engine.verification = VerificationState::Verified;
            engine.protected_claim_eligible = true;
            engine.declared_readiness = CapabilityReadinessStage::SignOffEligible;
            let verified = verify_fixture(&fixture).expect("valid signed production evidence");
            let resolved = CapabilityResolver::with_production_evidence(&verified)
                .resolve_at(&claim, fixture.as_of.clone());
            assert_eq!(resolved.state, ClaimState::Unavailable, "{mode:?}");
            assert!(!resolved.label_allowed, "{mode:?}");
            assert!(
                resolved
                    .reason_codes
                    .contains(&ReasonCode::TargetEngineModeIneligible),
                "{mode:?}: {:?}",
                resolved.reason_codes
            );
        }
    }

    #[test]
    fn model_binding_must_name_the_exact_bound_qualification_record() {
        let (mut fixture, claim) = positive_release_bundle();
        fixture.model_qualifications[0].qualification_evidence_id =
            "different-model-qualification".into();
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let resolved = CapabilityResolver::with_production_evidence(&verified)
            .resolve_at(&claim, fixture.as_of.clone());
        assert_eq!(resolved.state, ClaimState::Unknown);
        assert!(!resolved.label_allowed);
        assert!(
            resolved
                .reason_codes
                .contains(&ReasonCode::TargetRelationshipMismatch)
        );
    }

    #[test]
    fn oversized_claim_inputs_fail_before_evidence_scans() {
        let (fixture, mut claim) = positive_release_bundle();
        claim.label = "x".repeat(MAX_CAPABILITY_CLAIM_LABEL_BYTES + 1);
        let verified = verify_fixture(&fixture).expect("valid signed production evidence");
        let resolved = CapabilityResolver::with_production_evidence(&verified)
            .resolve_at(&claim, fixture.as_of.clone());
        assert_eq!(resolved.state, ClaimState::Unknown);
        assert!(!resolved.label_allowed);
        assert_eq!(resolved.reason_codes, [ReasonCode::ClaimInputTooLarge]);
        assert!(resolved.source.is_empty());
    }

    #[test]
    fn serde_contracts_are_closed_and_target_nulls_must_be_explicit() {
        let mut root: serde_json::Value = serde_json::from_str(CANONICAL_FIXTURE_JSON).unwrap();
        root.as_object_mut()
            .unwrap()
            .insert("unexpected".into(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<CapabilityFixture>(root).is_err());

        let fixture = canonical_fixture();
        let case = canonical_case(&fixture, "current-design-contract");
        let mut targets = serde_json::to_value(&case.targets).unwrap();
        targets.as_object_mut().unwrap().remove("platformId");
        assert!(serde_json::from_value::<ClaimTargets>(targets).is_err());

        assert!(serde_json::from_str::<ClaimState>("\"optimistic\"").is_err());
        assert!(serde_json::from_str::<CapabilityReadinessStage>("\"tested\"").is_err());

        let vector = canonical_case(&fixture, "positive-qualified-contract-test-vector");
        let resolved = CapabilityResolver::for_design_fixture(&fixture).resolve(&vector.claim());
        let serialized = serde_json::to_value(resolved).unwrap();
        assert_eq!(serialized["claimMode"], "contract-test-vector");
        assert_eq!(serialized["labelAllowed"], false);
    }

    #[test]
    fn resolver_never_mutates_fixture_or_claim_inputs() {
        let fixture = canonical_fixture();
        let claim = canonical_case(&fixture, "stale-design-contract").claim();
        let fixture_before = fixture.clone();
        let claim_before = claim.clone();
        let _ = CapabilityResolver::for_design_fixture(&fixture).resolve(&claim);
        assert_eq!(fixture, fixture_before);
        assert_eq!(claim, claim_before);
    }
}
