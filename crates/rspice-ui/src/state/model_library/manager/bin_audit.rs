//! Durable, content-addressed geometry-bin audit evidence.

use super::*;

const MODEL_BIN_AUDIT_SCHEMA_VERSION: u16 = 1;
const MAX_MODEL_BIN_AUDIT_RECEIPTS: usize = 256;
const MAX_MODEL_BIN_AUDIT_CARDS: usize = 250_000;
const MAX_MODEL_BIN_AUDIT_INSTANCES: usize = 1_000_000;
const MAX_MODEL_BIN_AUDIT_FINDINGS: usize = 250_000;
const MAX_MODEL_BIN_AUDIT_TEXT_BYTES: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ModelBinAuditSelection {
    ExactCard,
    FamilyMatch,
    SharedBoundary,
}

impl From<rspice_core::engine::ModelBinSelectionKind> for ModelBinAuditSelection {
    fn from(value: rspice_core::engine::ModelBinSelectionKind) -> Self {
        match value {
            rspice_core::engine::ModelBinSelectionKind::ExactCard => Self::ExactCard,
            rspice_core::engine::ModelBinSelectionKind::FamilyMatch => Self::FamilyMatch,
            rspice_core::engine::ModelBinSelectionKind::SharedBoundary => Self::SharedBoundary,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelBinAuditAxisRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl From<rspice_core::engine::ModelBinAxisRange> for ModelBinAuditAxisRange {
    fn from(value: rspice_core::engine::ModelBinAxisRange) -> Self {
        Self {
            min: value.min,
            max: value.max,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelBinAuditCard {
    pub model: String,
    pub family: String,
    pub model_type: String,
    pub declaration_order: usize,
    pub length: ModelBinAuditAxisRange,
    pub width: ModelBinAuditAxisRange,
    pub nfin: ModelBinAuditAxisRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelBinAuditInstance {
    pub element: String,
    pub requested_model: String,
    pub selected_model: String,
    pub selection: ModelBinAuditSelection,
    pub match_count: usize,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub nfin: Option<f64>,
    pub multiplier: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelBinAuditFindingKind {
    InvalidCard,
    PositiveAreaOverlap,
    CoverageGap,
    CoverageUnavailable,
    InspectionBlocked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelBinAuditFinding {
    pub kind: ModelBinAuditFindingKind,
    pub family: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub models: Vec<String>,
    pub detail: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<ModelBinAuditAxisRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<ModelBinAuditAxisRange>,
}

impl ModelBinAuditFinding {
    pub fn new(
        kind: ModelBinAuditFindingKind,
        family: impl Into<String>,
        models: Vec<String>,
        detail: impl Into<String>,
        length: Option<ModelBinAuditAxisRange>,
        width: Option<ModelBinAuditAxisRange>,
    ) -> Self {
        Self {
            kind,
            family: family.into(),
            models,
            detail: detail.into(),
            length,
            width,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelBinAuditDraft {
    request_digest: ContentDigest,
    source_digest: Option<ContentDigest>,
    project_id: crate::product::ProjectId,
    project_revision: u64,
    simulation_root: crate::state::CellViewRef,
    reference_process: crate::simulation::dialog::corner::ProcessCorner,
    reference_temperature_celsius: f64,
    cards: Vec<ModelBinAuditCard>,
    instances: Vec<ModelBinAuditInstance>,
    findings: Vec<ModelBinAuditFinding>,
}

impl ModelBinAuditDraft {
    pub fn from_engine(
        request_digest: ContentDigest,
        source_digest: ContentDigest,
        project_id: crate::product::ProjectId,
        project_revision: u64,
        simulation_root: crate::state::CellViewRef,
        reference_process: crate::simulation::dialog::corner::ProcessCorner,
        reference_temperature_celsius: f64,
        inspection: &rspice_core::engine::ModelBinInspection,
        findings: Vec<ModelBinAuditFinding>,
    ) -> Self {
        Self {
            request_digest,
            source_digest: Some(source_digest),
            project_id,
            project_revision,
            simulation_root,
            reference_process,
            reference_temperature_celsius,
            cards: inspection
                .cards
                .iter()
                .map(|card| ModelBinAuditCard {
                    model: card.model.clone(),
                    family: card.family.clone(),
                    model_type: card.model_type.clone(),
                    declaration_order: card.declaration_order,
                    length: card.geometry.length.into(),
                    width: card.geometry.width.into(),
                    nfin: card.geometry.nfin.into(),
                })
                .collect(),
            instances: inspection
                .instances
                .iter()
                .map(|instance| ModelBinAuditInstance {
                    element: instance.element.clone(),
                    requested_model: instance.requested_model.clone(),
                    selected_model: instance.selected_model.clone(),
                    selection: instance.selection.into(),
                    match_count: instance.match_count,
                    length: instance.length,
                    width: instance.width,
                    nfin: instance.nfin,
                    multiplier: instance.multiplier,
                })
                .collect(),
            findings,
        }
    }

    pub fn blocked(
        request_digest: ContentDigest,
        project_id: crate::product::ProjectId,
        project_revision: u64,
        simulation_root: crate::state::CellViewRef,
        reference_process: crate::simulation::dialog::corner::ProcessCorner,
        reference_temperature_celsius: f64,
        diagnostic: impl Into<String>,
    ) -> Self {
        Self {
            request_digest,
            source_digest: None,
            project_id,
            project_revision,
            simulation_root,
            reference_process,
            reference_temperature_celsius,
            cards: Vec::new(),
            instances: Vec::new(),
            findings: vec![ModelBinAuditFinding::new(
                ModelBinAuditFindingKind::InspectionBlocked,
                "",
                Vec::new(),
                diagnostic,
                None,
                None,
            )],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelBinAuditReceipt {
    schema_version: u16,
    sequence: u64,
    request_digest: ContentDigest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_digest: Option<ContentDigest>,
    project_id: crate::product::ProjectId,
    project_revision: u64,
    simulation_root: crate::state::CellViewRef,
    reference_process: crate::simulation::dialog::corner::ProcessCorner,
    reference_temperature_celsius: f64,
    cards: Vec<ModelBinAuditCard>,
    instances: Vec<ModelBinAuditInstance>,
    findings: Vec<ModelBinAuditFinding>,
    semantic_digest: ContentDigest,
}

impl ModelBinAuditReceipt {
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub const fn semantic_digest(&self) -> ContentDigest {
        self.semantic_digest
    }

    pub fn passed(&self) -> bool {
        self.source_digest.is_some() && self.findings.is_empty()
    }

    pub fn to_json_pretty(&self) -> Result<String, String> {
        self.validate()?;
        let mut json = serde_json::to_string_pretty(self)
            .map_err(|error| format!("Could not serialize model-bin audit receipt: {error}"))?;
        json.push('\n');
        Ok(json)
    }

    fn validate(&self) -> Result<(), String> {
        if self.schema_version != MODEL_BIN_AUDIT_SCHEMA_VERSION {
            return Err(format!(
                "Unsupported model-bin audit schema {}; expected {}",
                self.schema_version, MODEL_BIN_AUDIT_SCHEMA_VERSION
            ));
        }
        if self.sequence == 0 || self.project_revision == 0 {
            return Err("Model-bin audit sequence and project revision must be nonzero".to_owned());
        }
        self.simulation_root
            .validate_name_segments()
            .map_err(|error| format!("Invalid model-bin audit root: {error}"))?;
        if !self.reference_temperature_celsius.is_finite() {
            return Err("Model-bin audit temperature must be finite".to_owned());
        }
        check_limit("cards", self.cards.len(), MAX_MODEL_BIN_AUDIT_CARDS)?;
        check_limit(
            "instances",
            self.instances.len(),
            MAX_MODEL_BIN_AUDIT_INSTANCES,
        )?;
        check_limit(
            "findings",
            self.findings.len(),
            MAX_MODEL_BIN_AUDIT_FINDINGS,
        )?;
        let mut declaration_orders = std::collections::BTreeSet::new();
        for card in &self.cards {
            validate_text("card model", &card.model, false)?;
            validate_text("card family", &card.family, false)?;
            validate_text("card model type", &card.model_type, false)?;
            if !declaration_orders.insert(card.declaration_order) {
                return Err(format!(
                    "Model-bin audit contains duplicate card declaration order {}",
                    card.declaration_order
                ));
            }
            validate_axis(card.length)?;
            validate_axis(card.width)?;
            validate_axis(card.nfin)?;
        }
        let mut elements = std::collections::BTreeSet::new();
        for instance in &self.instances {
            validate_text("instance element", &instance.element, false)?;
            validate_text("requested model", &instance.requested_model, false)?;
            validate_text("selected model", &instance.selected_model, false)?;
            if !elements.insert(instance.element.to_ascii_lowercase()) {
                return Err(format!(
                    "Model-bin audit contains duplicate instance element '{}'",
                    instance.element
                ));
            }
            if instance.match_count == 0 {
                return Err("Model-bin audit instance match count must be nonzero".to_owned());
            }
            match instance.selection {
                ModelBinAuditSelection::ExactCard | ModelBinAuditSelection::FamilyMatch
                    if instance.match_count != 1 =>
                {
                    return Err(format!(
                        "Model-bin audit instance '{}' has a single-match selection with match count {}",
                        instance.element, instance.match_count
                    ));
                }
                ModelBinAuditSelection::SharedBoundary if instance.match_count < 2 => {
                    return Err(format!(
                        "Model-bin audit instance '{}' has a shared-boundary selection with match count {}",
                        instance.element, instance.match_count
                    ));
                }
                _ => {}
            }
            for value in [
                instance.length,
                instance.width,
                instance.nfin,
                instance.multiplier,
            ]
            .into_iter()
            .flatten()
            {
                if !value.is_finite() {
                    return Err(
                        "Model-bin audit instance geometry must contain finite values".to_owned(),
                    );
                }
            }
        }
        for finding in &self.findings {
            validate_text("finding family", &finding.family, true)?;
            validate_text("finding detail", &finding.detail, false)?;
            for model in &finding.models {
                validate_text("finding model", model, false)?;
            }
            if let Some(range) = finding.length {
                validate_axis(range)?;
            }
            if let Some(range) = finding.width {
                validate_axis(range)?;
            }
        }
        let inspection_blocked = self
            .findings
            .iter()
            .any(|finding| finding.kind == ModelBinAuditFindingKind::InspectionBlocked);
        if self.source_digest.is_some() && inspection_blocked {
            return Err(
                "Model-bin audit with executable source evidence cannot be inspection-blocked"
                    .to_owned(),
            );
        }
        if self.source_digest.is_none()
            && (!inspection_blocked || !self.cards.is_empty() || !self.instances.is_empty())
        {
            return Err(
                "Model-bin audit without executable source evidence must be a blocked audit with no evaluated cards or instances"
                    .to_owned(),
            );
        }
        let expected = receipt_digest(self)?;
        if self.semantic_digest != expected {
            return Err(format!(
                "Model-bin audit receipt digest mismatch: stored {}, calculated {}",
                self.semantic_digest, expected
            ));
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct ModelBinAuditReceiptMaterial<'a> {
    schema_version: u16,
    sequence: u64,
    request_digest: ContentDigest,
    source_digest: Option<ContentDigest>,
    project_id: crate::product::ProjectId,
    project_revision: u64,
    simulation_root: &'a crate::state::CellViewRef,
    reference_process: crate::simulation::dialog::corner::ProcessCorner,
    reference_temperature_celsius: f64,
    cards: &'a [ModelBinAuditCard],
    instances: &'a [ModelBinAuditInstance],
    findings: &'a [ModelBinAuditFinding],
}

fn receipt_digest(receipt: &ModelBinAuditReceipt) -> Result<ContentDigest, String> {
    let material = ModelBinAuditReceiptMaterial {
        schema_version: receipt.schema_version,
        sequence: receipt.sequence,
        request_digest: receipt.request_digest,
        source_digest: receipt.source_digest,
        project_id: receipt.project_id,
        project_revision: receipt.project_revision,
        simulation_root: &receipt.simulation_root,
        reference_process: receipt.reference_process,
        reference_temperature_celsius: receipt.reference_temperature_celsius,
        cards: &receipt.cards,
        instances: &receipt.instances,
        findings: &receipt.findings,
    };
    let bytes = serde_json::to_vec(&material)
        .map_err(|error| format!("Could not canonicalize model-bin audit receipt: {error}"))?;
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.model-bin-audit-receipt/v1\0");
    hasher.update(bytes);
    Ok(ContentDigest::from_bytes(hasher.finalize().into()))
}

fn validate_axis(range: ModelBinAuditAxisRange) -> Result<(), String> {
    for value in [range.min, range.max].into_iter().flatten() {
        if !value.is_finite() {
            return Err("Model-bin audit range must contain finite values".to_owned());
        }
    }
    if let (Some(min), Some(max)) = (range.min, range.max)
        && min > max
    {
        return Err("Model-bin audit range is reversed".to_owned());
    }
    Ok(())
}

fn validate_text(label: &str, value: &str, allow_empty: bool) -> Result<(), String> {
    if (!allow_empty && value.trim().is_empty()) || value.len() > MAX_MODEL_BIN_AUDIT_TEXT_BYTES {
        return Err(format!("Model-bin audit {label} is empty or too large"));
    }
    Ok(())
}

fn check_limit(label: &str, value: usize, limit: usize) -> Result<(), String> {
    if value > limit {
        Err(format!(
            "Model-bin audit {label} count {value} exceeds limit {limit}"
        ))
    } else {
        Ok(())
    }
}

impl ModelLibraryManager {
    pub fn latest_model_bin_audit_receipt(&self) -> Option<&ModelBinAuditReceipt> {
        self.model_bin_audit_receipts.last()
    }

    pub fn record_model_bin_audit(
        &mut self,
        draft: ModelBinAuditDraft,
    ) -> Result<ModelBinAuditReceipt, String> {
        Self::validate_model_bin_audit_receipt_ledger(&self.model_bin_audit_receipts, None)?;
        check_limit(
            "receipts",
            self.model_bin_audit_receipts.len().saturating_add(1),
            MAX_MODEL_BIN_AUDIT_RECEIPTS,
        )?;
        let sequence = self
            .model_bin_audit_receipts
            .last()
            .map_or(1, |receipt| receipt.sequence.saturating_add(1));
        if sequence == 0 {
            return Err("Model-bin audit receipt sequence is exhausted".to_owned());
        }
        let mut receipt = ModelBinAuditReceipt {
            schema_version: MODEL_BIN_AUDIT_SCHEMA_VERSION,
            sequence,
            request_digest: draft.request_digest,
            source_digest: draft.source_digest,
            project_id: draft.project_id,
            project_revision: draft.project_revision,
            simulation_root: draft.simulation_root,
            reference_process: draft.reference_process,
            reference_temperature_celsius: draft.reference_temperature_celsius,
            cards: draft.cards,
            instances: draft.instances,
            findings: draft.findings,
            semantic_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.semantic_digest = receipt_digest(&receipt)?;
        receipt.validate()?;
        self.model_bin_audit_receipts.push(receipt.clone());
        Ok(receipt)
    }

    pub(crate) fn model_bin_audit_receipts(&self) -> &[ModelBinAuditReceipt] {
        &self.model_bin_audit_receipts
    }

    pub(crate) fn validate_model_bin_audit_receipts(
        &self,
        project_id: crate::product::ProjectId,
    ) -> Result<(), String> {
        Self::validate_model_bin_audit_receipt_ledger(
            &self.model_bin_audit_receipts,
            Some(project_id),
        )
    }

    pub(crate) fn restore_model_bin_audit_receipts(
        &mut self,
        receipts: Vec<ModelBinAuditReceipt>,
        project_id: crate::product::ProjectId,
    ) -> Result<(), String> {
        Self::validate_model_bin_audit_receipt_ledger(&receipts, Some(project_id))?;
        self.model_bin_audit_receipts = receipts;
        Ok(())
    }

    pub(crate) fn validate_model_bin_audit_receipt_ledger(
        receipts: &[ModelBinAuditReceipt],
        project_id: Option<crate::product::ProjectId>,
    ) -> Result<(), String> {
        check_limit("receipts", receipts.len(), MAX_MODEL_BIN_AUDIT_RECEIPTS)?;
        for (index, receipt) in receipts.iter().enumerate() {
            receipt.validate()?;
            let expected_sequence = u64::try_from(index)
                .map_err(|_| "Model-bin audit ledger index overflow".to_owned())?
                .saturating_add(1);
            if receipt.sequence != expected_sequence {
                return Err(format!(
                    "Model-bin audit receipt sequence {} is invalid at ledger position {}; expected {}",
                    receipt.sequence,
                    index + 1,
                    expected_sequence
                ));
            }
            if project_id.is_some_and(|expected| receipt.project_id != expected) {
                return Err(format!(
                    "Model-bin audit receipt {} belongs to project {}, not the current project {}",
                    receipt.sequence,
                    receipt.project_id,
                    project_id.expect("checked above")
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> ModelBinAuditDraft {
        ModelBinAuditDraft {
            request_digest: ContentDigest::from_bytes([1; 32]),
            source_digest: Some(ContentDigest::from_bytes([2; 32])),
            project_id: crate::product::ProjectId::new(),
            project_revision: 1,
            simulation_root: crate::state::CellViewRef::default_top(),
            reference_process: crate::simulation::dialog::corner::ProcessCorner::TT,
            reference_temperature_celsius: 27.0,
            cards: vec![ModelBinAuditCard {
                model: "nch.0".to_owned(),
                family: "nch".to_owned(),
                model_type: "NMOS".to_owned(),
                declaration_order: 0,
                length: ModelBinAuditAxisRange {
                    min: Some(100e-9),
                    max: Some(200e-9),
                },
                width: ModelBinAuditAxisRange {
                    min: Some(200e-9),
                    max: Some(400e-9),
                },
                nfin: ModelBinAuditAxisRange {
                    min: None,
                    max: None,
                },
            }],
            instances: Vec::new(),
            findings: Vec::new(),
        }
    }

    #[test]
    fn receipt_is_content_addressed_and_round_trips() {
        let mut manager = ModelLibraryManager::new();
        let receipt = manager
            .record_model_bin_audit(draft())
            .expect("audit records");
        assert!(receipt.passed());
        let json = receipt.to_json_pretty().expect("receipt exports");
        let restored: ModelBinAuditReceipt = serde_json::from_str(&json).expect("receipt restores");
        restored.validate().expect("restored digest validates");
        assert_eq!(restored.semantic_digest(), receipt.semantic_digest());

        let manager_json = serde_json::to_string(&manager).expect("manager serializes");
        let restored_manager: ModelLibraryManager =
            serde_json::from_str(&manager_json).expect("manager restores");
        let persisted = restored_manager
            .latest_model_bin_audit_receipt()
            .expect("audit ledger persists");
        persisted.validate().expect("persisted receipt validates");
        assert_eq!(persisted.semantic_digest(), receipt.semantic_digest());
    }

    #[test]
    fn tampered_receipt_is_rejected_before_next_append() {
        let mut manager = ModelLibraryManager::new();
        manager
            .record_model_bin_audit(draft())
            .expect("first audit records");
        manager.model_bin_audit_receipts[0].cards[0].model = "tampered".to_owned();
        let before = manager.model_bin_audit_receipts.len();
        let error = manager
            .record_model_bin_audit(draft())
            .expect_err("tampered ledger must fail");
        assert!(error.contains("digest mismatch"), "{error}");
        assert_eq!(manager.model_bin_audit_receipts.len(), before);
    }
}
