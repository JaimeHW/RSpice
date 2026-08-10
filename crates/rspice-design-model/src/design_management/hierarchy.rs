//! Hierarchy-management policy, audit evidence, and lifecycle receipts.

use super::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EditInPlaceDepth {
    #[default]
    CurrentAndParentContext,
    CurrentOnly,
    TwoParentLevels,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MissingHierarchyViewPolicy {
    #[default]
    BlockNetlist,
    UseDeclaredFallbackOrder,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchyBlackBoxPolicy {
    #[default]
    RequireSignedBoundaryContract,
    AllowProjectAbstract,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchyCyclePolicy {
    #[default]
    BlockSaveAndIdentifyPath,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyManagementSettings {
    pub edit_in_place_depth: EditInPlaceDepth,
    pub missing_view: MissingHierarchyViewPolicy,
    pub black_box: HierarchyBlackBoxPolicy,
    pub cycle_detection: HierarchyCyclePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HierarchyAuditConfiguration {
    ActiveProject,
    ConfigurationSet {
        id: ConfigurationSetId,
        revision: u64,
        semantic_digest: ContentDigest,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchyViewChecks {
    #[default]
    AllDeclaredFallbacks,
    SelectedHierarchy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProtectedBoundaryChecks {
    #[default]
    ValidateSignaturesAndPins,
    PinsOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyAuditSubject {
    pub instance_path: String,
    pub cell_name: String,
    pub design_view: String,
    pub declared_fallbacks: Vec<String>,
    pub resolved_simulation_view: Option<String>,
    pub fallback_used: Option<String>,
    pub child_instance_paths: Vec<String>,
    pub protected_boundary_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedBoundaryEvidence {
    pub boundary_id: String,
    pub signature_valid: bool,
    pub pins_match: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyAuditRequest {
    pub configuration: HierarchyAuditConfiguration,
    pub view_checks: HierarchyViewChecks,
    pub protected_boundaries: ProtectedBoundaryChecks,
    pub subjects: Vec<HierarchyAuditSubject>,
    pub boundary_evidence: Vec<ProtectedBoundaryEvidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HierarchyAuditFindingKind {
    UnresolvedView,
    UndeclaredFallback,
    MissingChild,
    HierarchyCycle,
    MissingProtectedBoundaryEvidence,
    InvalidProtectedBoundarySignature,
    ProtectedBoundaryPinMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyAuditFinding {
    pub kind: HierarchyAuditFindingKind,
    pub instance_path: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HierarchyAuditReceipt {
    pub(super) id: HierarchyAuditReceiptId,
    pub(super) sequence: u64,
    pub(super) request_digest: ContentDigest,
    pub(super) resolved_subjects: usize,
    pub(super) findings: Vec<HierarchyAuditFinding>,
    pub(super) semantic_digest: ContentDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesignManagementOwnershipReceipt {
    pub catalog_revision: u64,
    pub affected_sheet_catalogs: usize,
    pub remapped_variant_objects: usize,
    pub remapped_annotation_objects: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignManagementCopyReceipt {
    pub catalog_revision: u64,
    pub copied_sheet_catalogs: usize,
    pub sheet_identity_map: BTreeMap<SheetId, SheetId>,
    pub port_identity_map: BTreeMap<CrossSheetPortId, CrossSheetPortId>,
}

impl HierarchyAuditReceipt {
    #[must_use]
    pub const fn id(&self) -> HierarchyAuditReceiptId {
        self.id
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn request_digest(&self) -> ContentDigest {
        self.request_digest
    }

    #[must_use]
    pub const fn resolved_subjects(&self) -> usize {
        self.resolved_subjects
    }

    #[must_use]
    pub fn findings(&self) -> &[HierarchyAuditFinding] {
        &self.findings
    }

    #[must_use]
    pub fn passed(&self) -> bool {
        self.findings.is_empty()
    }

    #[must_use]
    pub const fn semantic_digest(&self) -> ContentDigest {
        self.semantic_digest
    }

    pub(super) fn validate(&self) -> Result<(), DesignManagementError> {
        require_non_nil(self.id.as_uuid(), "hierarchy audit receipt")?;
        require_nonzero_revision(
            self.sequence,
            "hierarchy audit receipt",
            self.id.to_string(),
        )?;
        require_limit(
            "hierarchy audit findings",
            self.findings.len(),
            MAX_HIERARCHY_AUDIT_FINDINGS,
        )?;
        for finding in &self.findings {
            validate_path("hierarchy audit finding path", &finding.instance_path)?;
            validate_value("hierarchy audit finding detail", &finding.detail, false)?;
        }
        require_digest(
            self.semantic_digest,
            digest(
                "rspice-hierarchy-audit-receipt-semantic/v1",
                &HierarchyAuditReceiptMaterial::from(self),
            )?,
            "hierarchy audit receipt",
            self.id.to_string(),
        )
    }
}

#[derive(Serialize)]
pub(super) struct HierarchyAuditReceiptMaterial<'a> {
    id: HierarchyAuditReceiptId,
    sequence: u64,
    request_digest: ContentDigest,
    resolved_subjects: usize,
    findings: &'a [HierarchyAuditFinding],
}

impl<'a> From<&'a HierarchyAuditReceipt> for HierarchyAuditReceiptMaterial<'a> {
    fn from(receipt: &'a HierarchyAuditReceipt) -> Self {
        Self {
            id: receipt.id,
            sequence: receipt.sequence,
            request_digest: receipt.request_digest,
            resolved_subjects: receipt.resolved_subjects,
            findings: &receipt.findings,
        }
    }
}
