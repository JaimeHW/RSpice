//! The release-promotion gate: what a qualified model must carry before it ships.
//!
//! A release candidate is only promotable when every obligation here is
//! discharged — documentation present and content-addressed, licence scope
//! declared, consumer impact assessed, every supported platform shown
//! compatible, and each required approval role recorded.  Promotion then binds
//! the record to one exact suite revision and evidence digest, so a shipped
//! release can never be re-attributed to different evidence after the fact.

use super::*;

/// Content-addressed document used by release declarations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentReference {
    pub id: String,
    pub digest: ContentDigest,
}

impl DocumentReference {
    pub fn try_new(id: impl Into<String>, digest: ContentDigest) -> QualificationResult<Self> {
        let value = Self {
            id: id.into(),
            digest,
        };
        value.validate("document")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> QualificationResult<()> {
        require_text(&format!("{path}.id"), &self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequiredDocumentation {
    ModelDescription,
    ParameterReference,
    QualificationReport,
}

impl RequiredDocumentation {
    pub const REQUIRED: [Self; 3] = [
        Self::ModelDescription,
        Self::ParameterReference,
        Self::QualificationReport,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentationDeclaration {
    pub kind: RequiredDocumentation,
    pub document: DocumentReference,
}

/// Documents declared by a release candidate. Incomplete sets remain useful
/// draft state, but `validate_complete` fails closed.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentationSet {
    pub declarations: Vec<DocumentationDeclaration>,
}

impl DocumentationSet {
    pub fn try_new(mut declarations: Vec<DocumentationDeclaration>) -> QualificationResult<Self> {
        declarations.sort_by_key(|value| value.kind);
        let value = Self { declarations };
        value.validate_structure()?;
        Ok(value)
    }

    fn validate_structure(&self) -> QualificationResult<()> {
        let mut kinds = BTreeSet::new();
        let mut ids = BTreeSet::new();
        for (index, declaration) in self.declarations.iter().enumerate() {
            declaration
                .document
                .validate(&format!("documentation.declarations[{index}].document"))?;
            if !kinds.insert(declaration.kind) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("documentation.declarations[{index}].kind"),
                    "required document kind is duplicated",
                ));
            }
            if !ids.insert(normalized(&declaration.document.id)) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("documentation.declarations[{index}].document.id"),
                    "document ID is duplicated case-insensitively",
                ));
            }
        }
        Ok(())
    }

    pub fn validate_complete(&self) -> QualificationResult<()> {
        self.validate_structure()?;
        let present: BTreeSet<_> = self.declarations.iter().map(|value| value.kind).collect();
        if !RequiredDocumentation::REQUIRED
            .iter()
            .all(|kind| present.contains(kind))
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DocumentationIncomplete,
                "documentation.declarations",
                "model description, parameter reference, and qualification report are required",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LicenseScope {
    FoundryProject,
    OrganizationInternal,
    Redistributable,
}

/// Reviewed license declaration for the promoted model package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseDeclaration {
    pub license_id: String,
    pub expression: String,
    pub scope: LicenseScope,
    pub commercial_use_allowed: bool,
    pub redistribution_allowed: bool,
    pub reviewed: bool,
    pub notice: DocumentReference,
}

impl LicenseDeclaration {
    pub fn validate_for_release(&self) -> QualificationResult<()> {
        require_text("license.license_id", &self.license_id)
            .map_err(|_| license_incomplete("license.license_id", "license ID is required"))?;
        require_text("license.expression", &self.expression).map_err(|_| {
            license_incomplete("license.expression", "license expression is required")
        })?;
        self.notice
            .validate("license.notice")
            .map_err(|_| license_incomplete("license.notice", "license notice is required"))?;
        if !self.reviewed || !self.commercial_use_allowed {
            return Err(license_incomplete(
                "license",
                "a reviewed license permitting commercial use is required",
            ));
        }
        if self.scope == LicenseScope::Redistributable && !self.redistribution_allowed {
            return Err(license_incomplete(
                "license.redistribution_allowed",
                "a redistributable package must explicitly permit redistribution",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConsumerChange {
    NoImpact,
    Compatible,
    Breaking,
}

/// Explicit impact assessment for cells and plans that consume the model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsumerImpactAssessment {
    pub change: ConsumerChange,
    pub summary: String,
    pub affected_consumer_ids: Vec<String>,
    pub migration_plan: Option<DocumentReference>,
    pub reviewed: bool,
}

impl ConsumerImpactAssessment {
    pub fn try_new(
        change: ConsumerChange,
        summary: impl Into<String>,
        mut affected_consumer_ids: Vec<String>,
        migration_plan: Option<DocumentReference>,
        reviewed: bool,
    ) -> QualificationResult<Self> {
        affected_consumer_ids.sort_by_key(|value| normalized(value));
        let value = Self {
            change,
            summary: summary.into(),
            affected_consumer_ids,
            migration_plan,
            reviewed,
        };
        value.validate_for_release()?;
        Ok(value)
    }

    pub fn validate_for_release(&self) -> QualificationResult<()> {
        require_text("consumer_impact.summary", &self.summary).map_err(|_| {
            consumer_incomplete("consumer_impact.summary", "impact summary is required")
        })?;
        for (index, id) in self.affected_consumer_ids.iter().enumerate() {
            require_text(
                &format!("consumer_impact.affected_consumer_ids[{index}]"),
                id,
            )
            .map_err(|_| {
                consumer_incomplete(
                    format!("consumer_impact.affected_consumer_ids[{index}]"),
                    "consumer ID is required",
                )
            })?;
        }
        ensure_unique(
            "consumer_impact.affected_consumer_ids",
            self.affected_consumer_ids.iter().map(String::as_str),
            QualificationErrorCode::DuplicateId,
            "consumer ID",
        )?;
        if let Some(plan) = &self.migration_plan {
            plan.validate("consumer_impact.migration_plan")?;
        }
        if !self.reviewed {
            return Err(consumer_incomplete(
                "consumer_impact.reviewed",
                "consumer impact must be reviewed",
            ));
        }
        if self.change == ConsumerChange::Breaking && self.migration_plan.is_none() {
            return Err(consumer_incomplete(
                "consumer_impact.migration_plan",
                "a breaking change requires a content-addressed migration plan",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompatibilityDisposition {
    Compatible,
    MigrationRequired,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformCompatibilityEvidence {
    pub platform: QualificationPlatform,
    pub disposition: CompatibilityDisposition,
    pub evidence: DocumentReference,
}

/// Runtime and pinned-consumer compatibility declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityAssessment {
    pub platforms: Vec<PlatformCompatibilityEvidence>,
    pub existing_projects: CompatibilityDisposition,
    pub reviewed: bool,
}

impl CompatibilityAssessment {
    pub fn try_new(
        mut platforms: Vec<PlatformCompatibilityEvidence>,
        existing_projects: CompatibilityDisposition,
        reviewed: bool,
    ) -> QualificationResult<Self> {
        platforms.sort_by_key(|value| value.platform);
        let value = Self {
            platforms,
            existing_projects,
            reviewed,
        };
        value.validate_structure()?;
        Ok(value)
    }

    fn validate_structure(&self) -> QualificationResult<()> {
        let mut seen = BTreeSet::new();
        for (index, platform) in self.platforms.iter().enumerate() {
            if !seen.insert(platform.platform) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("compatibility.platforms[{index}].platform"),
                    "compatibility platform is duplicated",
                ));
            }
            platform
                .evidence
                .validate(&format!("compatibility.platforms[{index}].evidence"))?;
        }
        Ok(())
    }

    pub fn validate_for_release(
        &self,
        impact: &ConsumerImpactAssessment,
    ) -> QualificationResult<()> {
        self.validate_structure()?;
        if !self.reviewed {
            return Err(compatibility_incomplete(
                "compatibility.reviewed",
                "compatibility assessment must be reviewed",
            ));
        }
        for required in QualificationPlatform::REQUIRED {
            let result = self
                .platforms
                .iter()
                .find(|value| value.platform == required);
            let Some(result) = result else {
                return Err(compatibility_incomplete(
                    "compatibility.platforms",
                    "Desktop and WebAssembly compatibility evidence are required",
                ));
            };
            if result.disposition == CompatibilityDisposition::Incompatible {
                return Err(compatibility_incomplete(
                    "compatibility.platforms",
                    "an incompatible runtime platform blocks release",
                ));
            }
        }
        if self.platforms.len() != QualificationPlatform::REQUIRED.len() {
            return Err(compatibility_incomplete(
                "compatibility.platforms",
                "exactly one Desktop and one WebAssembly compatibility result are required",
            ));
        }
        let migration_required = self.existing_projects
            == CompatibilityDisposition::MigrationRequired
            || self
                .platforms
                .iter()
                .any(|value| value.disposition == CompatibilityDisposition::MigrationRequired);
        if self.existing_projects == CompatibilityDisposition::Incompatible {
            return Err(compatibility_incomplete(
                "compatibility.existing_projects",
                "incompatible existing projects block release",
            ));
        }
        if migration_required
            && (impact.change != ConsumerChange::Breaking || impact.migration_plan.is_none())
        {
            return Err(compatibility_incomplete(
                "compatibility",
                "migration-required compatibility needs a reviewed breaking-impact migration plan",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PromotionApprovalRole {
    ModelOwner,
    QualificationApprover,
}

impl PromotionApprovalRole {
    pub const REQUIRED: [Self; 2] = [Self::ModelOwner, Self::QualificationApprover];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ApprovalDecision {
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionApproval {
    pub role: PromotionApprovalRole,
    pub approver_id: String,
    pub decision: ApprovalDecision,
    pub decision_revision: ObjectRevision,
}

/// Stable identity of a draft or frozen model release candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseCandidateIdentity {
    pub id: String,
    pub model_id: String,
    pub version: String,
}

impl ReleaseCandidateIdentity {
    fn validate(&self) -> QualificationResult<()> {
        require_text("candidate.identity.id", &self.id)?;
        require_text("candidate.identity.model_id", &self.model_id)?;
        require_text("candidate.identity.version", &self.version)
    }
}

/// Stable identity assigned only after a candidate passes promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelReleaseIdentity {
    pub id: String,
    pub model_id: String,
    pub version: String,
}

impl ModelReleaseIdentity {
    fn validate(&self) -> QualificationResult<()> {
        require_text("release.identity.id", &self.id)?;
        require_text("release.identity.model_id", &self.model_id)?;
        require_text("release.identity.version", &self.version)
    }
}

/// Materialized release gates shown by the Release surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionChecklist {
    pub source_evidence_bound: bool,
    pub qualification_passed: bool,
    pub desktop_passed: bool,
    pub webassembly_passed: bool,
    pub documentation_complete: bool,
    pub license_complete: bool,
    pub consumer_impact_complete: bool,
    pub compatibility_complete: bool,
    pub independent_approval_complete: bool,
}

impl PromotionChecklist {
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.source_evidence_bound
            && self.qualification_passed
            && self.desktop_passed
            && self.webassembly_passed
            && self.documentation_complete
            && self.license_complete
            && self.consumer_impact_complete
            && self.compatibility_complete
            && self.independent_approval_complete
    }
}

/// Versioned candidate state consumed by the existing Release surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelReleaseCandidate {
    pub schema_version: u32,
    pub identity: ReleaseCandidateIdentity,
    pub source: ModelSourceEvidenceBinding,
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub evidence_id: String,
    /// Exact digest of the immutable evidence payload. `None` is retained only
    /// so pre-schema-4 records can be loaded for diagnosis; it never validates
    /// and is never inferred from whatever evidence currently has the same ID.
    #[serde(default)]
    pub evidence_digest: Option<ContentDigest>,
    /// Exact canonical model source retained for semantic and numerical
    /// comparison. Legacy imported candidates may omit it; new GUI promotion
    /// transactions bind it before creating a release.
    #[serde(default)]
    pub definition_source: Vec<u8>,
    /// Exact typed definition contract retained with the candidate. The
    /// canonical SPICE bytes preserve executable behavior; this snapshot also
    /// preserves units, bounds, inheritance, statistics, temperature laws,
    /// and section qualification semantics for governed comparison.
    #[serde(default)]
    pub definition_metadata: Option<ModelDefinitionMetadata>,
    pub documentation: DocumentationSet,
    pub license: Option<LicenseDeclaration>,
    pub consumer_impact: Option<ConsumerImpactAssessment>,
    pub compatibility: Option<CompatibilityAssessment>,
    pub approvals: Vec<PromotionApproval>,
    pub checklist: PromotionChecklist,
}

impl ModelReleaseCandidate {
    pub fn try_new(
        identity: ReleaseCandidateIdentity,
        source: ModelSourceEvidenceBinding,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
        documentation: DocumentationSet,
        license: Option<LicenseDeclaration>,
        consumer_impact: Option<ConsumerImpactAssessment>,
        compatibility: Option<CompatibilityAssessment>,
        mut approvals: Vec<PromotionApproval>,
    ) -> QualificationResult<Self> {
        approvals.sort_by_key(|value| value.role);
        let evidence_digest = evidence.content_digest()?;
        let mut value = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            identity,
            source,
            suite_id: suite.id.clone(),
            suite_revision: suite.revision,
            evidence_id: evidence.id.clone(),
            evidence_digest: Some(evidence_digest),
            definition_source: Vec::new(),
            definition_metadata: None,
            documentation,
            license,
            consumer_impact,
            compatibility,
            approvals,
            checklist: PromotionChecklist {
                source_evidence_bound: false,
                qualification_passed: false,
                desktop_passed: false,
                webassembly_passed: false,
                documentation_complete: false,
                license_complete: false,
                consumer_impact_complete: false,
                compatibility_complete: false,
                independent_approval_complete: false,
            },
        };
        value.checklist = value.derived_checklist(suite, evidence);
        value.validate_bound(suite, evidence)?;
        Ok(value)
    }

    fn approvals_complete(&self) -> bool {
        let mut roles = BTreeSet::new();
        let mut people = BTreeSet::new();
        self.approvals.iter().all(|approval| {
            approval.decision == ApprovalDecision::Approved
                && !approval.approver_id.trim().is_empty()
                && roles.insert(approval.role)
                && people.insert(normalized(&approval.approver_id))
        }) && PromotionApprovalRole::REQUIRED
            .iter()
            .all(|role| roles.contains(role))
            && self.approvals.len() == PromotionApprovalRole::REQUIRED.len()
    }

    fn derived_checklist(
        &self,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> PromotionChecklist {
        let source_evidence_bound = evidence.validate_bound(suite, &self.source).is_ok()
            && normalized(&self.suite_id) == normalized(&suite.id)
            && self.suite_revision == suite.revision
            && normalized(&self.evidence_id) == normalized(&evidence.id)
            && evidence
                .content_digest()
                .is_ok_and(|digest| self.evidence_digest == Some(digest));
        let consumer_impact_complete = self
            .consumer_impact
            .as_ref()
            .is_some_and(|value| value.validate_for_release().is_ok());
        let compatibility_complete = self
            .compatibility
            .as_ref()
            .zip(self.consumer_impact.as_ref())
            .is_some_and(|(compatibility, impact)| {
                compatibility.validate_for_release(impact).is_ok()
            });
        PromotionChecklist {
            source_evidence_bound,
            qualification_passed: source_evidence_bound && evidence.passed,
            desktop_passed: source_evidence_bound
                && evidence.platform_passed(QualificationPlatform::Desktop),
            webassembly_passed: source_evidence_bound
                && evidence.platform_passed(QualificationPlatform::WebAssembly),
            documentation_complete: self.documentation.validate_complete().is_ok(),
            license_complete: self
                .license
                .as_ref()
                .is_some_and(|value| value.validate_for_release().is_ok()),
            consumer_impact_complete,
            compatibility_complete,
            independent_approval_complete: self.approvals_complete(),
        }
    }

    /// Validates references and stored checklist state without requiring the
    /// candidate to be promotable. This preserves honest blocked candidates.
    pub fn validate_bound(
        &self,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> QualificationResult<()> {
        require_schema("candidate.schema_version", self.schema_version)?;
        self.identity.validate()?;
        self.source.validate("candidate.source")?;
        if normalized(&self.identity.model_id) != normalized(&self.source.model_id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "candidate.identity.model_id",
                "candidate model identity does not match its source binding",
            ));
        }
        require_text("candidate.suite_id", &self.suite_id)?;
        require_text("candidate.evidence_id", &self.evidence_id)?;
        if !self.definition_source.is_empty()
            && digest_bytes(&self.definition_source) != self.source.source_digest
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::InputDigestMismatch,
                "candidate.definition_source",
                "retained model definition source does not match the candidate source digest",
            ));
        }
        if let Some(metadata) = &self.definition_metadata {
            metadata.validate().map_err(|error| {
                QualificationValidationError::new(
                    QualificationErrorCode::SourceBindingMismatch,
                    "candidate.definition_metadata",
                    format!("retained typed definition metadata is invalid: {error}"),
                )
            })?;
            let expected_source_id = self
                .source
                .require_project_bound("candidate.source")?
                .to_string();
            let expected_revision = self.source.source_revision.get();
            let expected_digest = self.source.source_digest.to_string();
            for (section_index, section) in metadata.sections.iter().enumerate() {
                for (file_index, file) in section.model_files.iter().enumerate() {
                    if file.source_id != expected_source_id
                        || file.revision != expected_revision
                        || file.content_digest != expected_digest
                    {
                        return Err(QualificationValidationError::new(
                            QualificationErrorCode::SourceBindingMismatch,
                            format!(
                                "candidate.definition_metadata.sections[{section_index}].model_files[{file_index}]"
                            ),
                            "typed definition metadata is not bound to the candidate source identity",
                        ));
                    }
                }
            }
        }
        self.documentation.validate_structure()?;
        if let Some(license) = &self.license {
            license.notice.validate("candidate.license.notice")?;
        }
        if let Some(impact) = &self.consumer_impact {
            for (index, consumer) in impact.affected_consumer_ids.iter().enumerate() {
                require_text(
                    &format!("candidate.consumer_impact.affected_consumer_ids[{index}]"),
                    consumer,
                )?;
            }
            ensure_unique(
                "candidate.consumer_impact.affected_consumer_ids",
                impact.affected_consumer_ids.iter().map(String::as_str),
                QualificationErrorCode::DuplicateId,
                "consumer ID",
            )?;
        }
        if let Some(compatibility) = &self.compatibility {
            compatibility.validate_structure()?;
        }
        validate_approvals(&self.approvals)?;
        evidence.validate_bound(suite, &self.source)?;
        if normalized(&self.suite_id) != normalized(&suite.id)
            || self.suite_revision != suite.revision
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SuiteBindingMismatch,
                "candidate.suite",
                "candidate suite binding does not match",
            ));
        }
        if normalized(&self.evidence_id) != normalized(&evidence.id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "candidate.evidence_id",
                "candidate evidence identity does not match",
            ));
        }
        let evidence_digest = evidence.content_digest()?;
        let bound_evidence_digest = self.evidence_digest.ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                "candidate.evidence_digest",
                "candidate does not retain an evidence content digest",
            )
        })?;
        if bound_evidence_digest != evidence_digest {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "candidate.evidence_digest",
                "candidate evidence content digest does not match the retained evidence payload",
            ));
        }
        let expected = self.derived_checklist(suite, evidence);
        if self.checklist != expected {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::ChecklistMismatch,
                "candidate.checklist",
                "stored promotion checklist does not match the bound declarations and evidence",
            ));
        }
        Ok(())
    }

    /// Fail-closed validation used by the guarded promotion operation.
    pub fn validate_for_promotion(
        &self,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> QualificationResult<()> {
        self.validate_bound(suite, evidence)?;
        self.source.require_project_bound("candidate.source")?;
        if self.definition_source.is_empty() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                "candidate.definition_source",
                "promotion requires the exact canonical model source snapshot",
            ));
        }
        if self.definition_metadata.is_none() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                "candidate.definition_metadata",
                "promotion requires the validated typed model-definition metadata",
            ));
        }
        self.documentation.validate_complete()?;
        self.license
            .as_ref()
            .ok_or_else(|| {
                license_incomplete("candidate.license", "license declaration is required")
            })?
            .validate_for_release()?;
        let impact = self.consumer_impact.as_ref().ok_or_else(|| {
            consumer_incomplete(
                "candidate.consumer_impact",
                "consumer-impact assessment is required",
            )
        })?;
        impact.validate_for_release()?;
        self.compatibility
            .as_ref()
            .ok_or_else(|| {
                compatibility_incomplete(
                    "candidate.compatibility",
                    "compatibility assessment is required",
                )
            })?
            .validate_for_release(impact)?;
        if !self.approvals_complete() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::ApprovalIncomplete,
                "candidate.approvals",
                "distinct model-owner and qualification-approver approvals are required",
            ));
        }
        if !evidence.passed
            || !evidence.platform_passed(QualificationPlatform::Desktop)
            || !evidence.platform_passed(QualificationPlatform::WebAssembly)
            || !self.checklist.is_complete()
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::PromotionBlocked,
                "candidate.checklist",
                "promotion requires fully passing, source-bound Desktop and WebAssembly evidence",
            ));
        }
        Ok(())
    }
}

/// Immutable audit record emitted only by the guarded promotion constructor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelPromotionRecord {
    pub schema_version: u32,
    pub id: String,
    pub candidate_identity: ReleaseCandidateIdentity,
    pub release_identity: ModelReleaseIdentity,
    pub source: ModelSourceEvidenceBinding,
    pub suite_id: String,
    pub suite_revision: ObjectRevision,
    pub evidence_id: String,
    /// Exact evidence payload promoted by this immutable audit record.
    #[serde(default)]
    pub evidence_digest: Option<ContentDigest>,
    pub checklist: PromotionChecklist,
}

/// Immutable model release created together with its promotion record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelRelease {
    pub schema_version: u32,
    pub identity: ModelReleaseIdentity,
    pub candidate_id: String,
    pub source: ModelSourceEvidenceBinding,
    pub evidence_id: String,
    /// Exact evidence payload authorizing this immutable release.
    #[serde(default)]
    pub evidence_digest: Option<ContentDigest>,
    /// Canonical source snapshot copied from the promoted candidate. This is
    /// the immutable comparison authority for later model revisions.
    #[serde(default)]
    pub definition_source: Vec<u8>,
    #[serde(default)]
    pub definition_metadata: Option<ModelDefinitionMetadata>,
    pub promotion_record_id: String,
}

impl ModelPromotionRecord {
    /// Creates a release and its audit record atomically in memory. No record
    /// is returned unless every gate validates against the exact bound source.
    pub fn promote(
        id: impl Into<String>,
        release_identity: ModelReleaseIdentity,
        candidate: &ModelReleaseCandidate,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> QualificationResult<(Self, ModelRelease)> {
        candidate.validate_for_promotion(suite, evidence)?;
        let id = id.into();
        require_text("promotion.id", &id)?;
        release_identity.validate()?;
        if normalized(&release_identity.model_id) != normalized(&candidate.identity.model_id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "release.identity.model_id",
                "release and candidate model identities do not match",
            ));
        }
        if normalized(&release_identity.id) == normalized(&candidate.identity.id) {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::DuplicateId,
                "release.identity.id",
                "release identity must be distinct from the candidate identity",
            ));
        }
        let evidence_digest = evidence.content_digest()?;
        let record = Self {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            id: id.clone(),
            candidate_identity: candidate.identity.clone(),
            release_identity: release_identity.clone(),
            source: candidate.source.clone(),
            suite_id: candidate.suite_id.clone(),
            suite_revision: candidate.suite_revision,
            evidence_id: candidate.evidence_id.clone(),
            evidence_digest: Some(evidence_digest),
            checklist: candidate.checklist.clone(),
        };
        let release = ModelRelease {
            schema_version: MODEL_QUALIFICATION_SCHEMA_VERSION,
            identity: release_identity,
            candidate_id: candidate.identity.id.clone(),
            source: candidate.source.clone(),
            evidence_id: candidate.evidence_id.clone(),
            evidence_digest: Some(evidence_digest),
            definition_source: candidate.definition_source.clone(),
            definition_metadata: candidate.definition_metadata.clone(),
            promotion_record_id: id,
        };
        record.validate_bound(&release, candidate, suite, evidence)?;
        Ok((record, release))
    }

    pub fn validate_bound(
        &self,
        release: &ModelRelease,
        candidate: &ModelReleaseCandidate,
        suite: &QualificationSuite,
        evidence: &QualificationEvidence,
    ) -> QualificationResult<()> {
        require_schema("promotion.schema_version", self.schema_version)?;
        require_schema("release.schema_version", release.schema_version)?;
        require_text("promotion.id", &self.id)?;
        self.release_identity.validate()?;
        release.identity.validate()?;
        candidate.validate_for_promotion(suite, evidence)?;
        let evidence_digest = evidence.content_digest()?;
        let promotion_evidence_digest = self.evidence_digest.ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                "promotion.evidence_digest",
                "promotion record does not retain an evidence content digest",
            )
        })?;
        let release_evidence_digest = release.evidence_digest.ok_or_else(|| {
            QualificationValidationError::new(
                QualificationErrorCode::MissingRequiredValue,
                "release.evidence_digest",
                "release does not retain an evidence content digest",
            )
        })?;
        let exact = self.candidate_identity == candidate.identity
            && self.release_identity == release.identity
            && self.source == candidate.source
            && self.source == release.source
            && normalized(&self.suite_id) == normalized(&suite.id)
            && self.suite_revision == suite.revision
            && normalized(&self.evidence_id) == normalized(&evidence.id)
            && normalized(&release.evidence_id) == normalized(&evidence.id)
            && promotion_evidence_digest == evidence_digest
            && release_evidence_digest == evidence_digest
            && candidate.evidence_digest == Some(evidence_digest)
            && release.definition_source == candidate.definition_source
            && release.definition_metadata == candidate.definition_metadata
            && normalized(&release.candidate_id) == normalized(&candidate.identity.id)
            && normalized(&release.promotion_record_id) == normalized(&self.id)
            && self.checklist == candidate.checklist;
        if !exact {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::SourceBindingMismatch,
                "promotion",
                "promotion record, release, candidate, suite, and evidence are not exactly bound",
            ));
        }
        Ok(())
    }
}
