//! Signed, content-addressed PDK technology-package administration.
//!
//! A configured model search path is not a technology package.  This module
//! keeps those concepts separate and gives the PDK administration surface a
//! real authority boundary:
//!
//! - the publisher signature covers the exact manifest bytes;
//! - every packaged artifact is size- and digest-verified;
//! - layer, purpose, stream-map, connectivity, callback, and platform
//!   contracts are validated before installation;
//! - installed revisions are immutable;
//! - activation and rollback are append-only, hash-chained transactions; and
//! - deserialized packages are never executable until they are revalidated
//!   against the current trust store.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

mod trust_verification;

use crate::product::ContentDigest;

pub const PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION: u32 = 1;
pub const PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION: u32 = 5;
const MINIMUM_PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION: u32 = 1;
pub const MAX_PDK_ARCHIVE_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_PDK_MANIFEST_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_PDK_ARTIFACTS: usize = 4_096;
pub const MAX_PDK_ARTIFACT_BYTES: usize = 32 * 1024 * 1024;
pub const MAX_PDK_TOTAL_ARTIFACT_BYTES: usize = 48 * 1024 * 1024;
pub const MAX_PDK_LAYERS: usize = 2_048;
pub const MAX_PDK_LAYER_ALIASES: usize = 16_384;
pub const MAX_PDK_STREAM_MAP_ENTRIES: usize = 16_384;
pub const MAX_PDK_CONNECTIVITY_EDGES: usize = 8_192;
pub const MAX_PDK_VIA_DEFINITIONS: usize = 8_192;
pub const MAX_PDK_RECOGNITION_CONTRACTS: usize = 4_096;
pub const MAX_PDK_EXTRACTION_CONTRACTS: usize = 4_096;
pub const MAX_PDK_QUALIFICATION_VECTORS: usize = 16_384;
pub const MAX_PDK_RECOGNITION_TERMINALS: usize = 128;
pub const MAX_PDK_AUDIT_RECEIPTS: usize = 16_384;
pub const MAX_PDK_TRUST_AUDIT_RECEIPTS: usize = 16_384;
pub const MAX_PDK_PUBLISHER_KEYS: usize = 4_096;
pub const MAX_PDK_MODEL_PROCESS_CONTRACTS: usize = 5;
pub const MAX_PDK_MODEL_SECTION_SOURCES: usize = 1_024;
pub const MAX_PDK_VERILOGA_SOURCE_CONTRACTS: usize = 1_024;
pub const MAX_PDK_SYMBOL_DEFINITIONS: usize = 4_096;
pub const MAX_PDK_CALLBACK_CONTRACTS: usize = 256;
pub const MAX_PDK_CALLBACK_ARTIFACT_BYTES: usize = 4 * 1024 * 1024;
pub const PDK_CALLBACK_ABI_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedPdkTechnologyArchive {
    pub schema_version: u32,
    pub manifest_base64: String,
    pub signature_base64: String,
    pub files: Vec<PdkTechnologyArchiveFile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyArchiveFile {
    pub path: String,
    pub content_base64: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyManifest {
    pub schema_version: u32,
    pub package_id: String,
    pub technology_name: String,
    pub revision: String,
    pub publisher_id: String,
    pub signing_key_id: String,
    pub license_spdx: String,
    pub process_node_nm: u32,
    pub database_unit_meters: f64,
    pub stack_name: String,
    pub compatibility: PdkTechnologyCompatibility,
    /// Exact SPICE model roots and process-section selections supplied by
    /// this package. Model artifacts are executable only through this typed
    /// contract; an artifact label alone never grants simulator authority.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub model_sources: Vec<PdkModelProcessContract>,
    /// Exact Verilog-A roots compiled from signed, package-contained source
    /// closures. An artifact typed as Verilog-A source is never executable
    /// unless one of these contracts selects its module and netlist alias.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub veriloga_sources: Vec<PdkVerilogASourceContract>,
    /// Read-only symbol, pin, netlist, and typed parameter-form contracts
    /// supplied by this exact signed technology revision.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub symbol_definitions: Vec<crate::state::ModelBoundSymbolDefinition>,
    pub layers: Vec<PdkTechnologyLayer>,
    /// Portable alternate names for exact layer-purpose identities. Aliases
    /// are normalized case-insensitively and never replace the canonical
    /// layer or purpose stored in layout data.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layer_aliases: Vec<PdkLayerAlias>,
    pub stream_map: Vec<PdkStreamMapEntry>,
    pub connectivity: Vec<PdkConnectivityEdge>,
    /// Manufacturable via-generator limits for connectivity transitions.
    /// Legacy manifests may provide connectivity without generator geometry;
    /// consumers must not infer dimensions from a bare connectivity edge.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vias: Vec<PdkViaDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recognition: Vec<PdkRecognitionContract>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extraction: Vec<PdkExtractionContract>,
    pub callbacks: Vec<PdkCallbackContract>,
    pub artifacts: Vec<PdkTechnologyArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyCompatibility {
    pub minimum_engine_version: String,
    pub minimum_viewer_version: String,
    pub targets: Vec<PdkExecutionTarget>,
}

/// Process identity understood by the RSpice run-set and corner engines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PdkModelProcess {
    Tt,
    Ss,
    Ff,
    Sf,
    Fs,
}

impl PdkModelProcess {
    pub const ALL: [Self; 5] = [Self::Tt, Self::Ss, Self::Ff, Self::Sf, Self::Fs];

    #[must_use]
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Tt => "TT",
            Self::Ss => "SS",
            Self::Ff => "FF",
            Self::Sf => "SF",
            Self::Fs => "FS",
        }
    }
}

/// Functional model domain supplied by one selected package source.
///
/// A composite source is the conventional foundry `.lib TT` contract. The
/// remaining domains permit foundries to split MOS, bipolar, passive,
/// macro-model, statistical, and aging cards without making execution guess
/// which source is authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkModelDomain {
    Composite,
    Mos,
    Bjt,
    Passives,
    MacroModels,
    StatisticalGlobal,
    StatisticalLocal,
    Aging,
}

impl PdkModelDomain {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Composite => "Composite",
            Self::Mos => "MOS",
            Self::Bjt => "BJT",
            Self::Passives => "Passives",
            Self::MacroModels => "Macro models",
            Self::StatisticalGlobal => "Statistical (global)",
            Self::StatisticalLocal => "Statistical (local)",
            Self::Aging => "Aging",
        }
    }
}

/// One exact artifact/section that supplies a process-domain model source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkModelSectionSource {
    pub source_id: String,
    pub domain: PdkModelDomain,
    pub artifact_path: String,
    /// `None` deliberately selects the complete source. `Some` selects the
    /// named inline `.lib` section through the authenticated sealed resolver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}

/// Complete executable model-source contract for one process point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkModelProcessContract {
    pub process: PdkModelProcess,
    pub sources: Vec<PdkModelSectionSource>,
    pub required_domains: Vec<PdkModelDomain>,
}

/// One executable Verilog-A runtime selected from the signed package.
///
/// The root and every recursively included document must be artifacts typed as
/// [`PdkTechnologyArtifactKind::VerilogASource`] in the same archive. The
/// compiler is never permitted to consult an ambient file system or network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkVerilogASourceContract {
    pub source_id: String,
    pub root_artifact_path: String,
    pub module_name: String,
    pub netlist_alias: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkExecutionTarget {
    Desktop,
    WebAssembly,
    Mobile,
}

#[cfg(target_arch = "wasm32")]
#[must_use]
pub const fn current_execution_target() -> PdkExecutionTarget {
    PdkExecutionTarget::WebAssembly
}

#[cfg(all(
    not(target_arch = "wasm32"),
    any(target_os = "android", target_os = "ios")
))]
#[must_use]
pub const fn current_execution_target() -> PdkExecutionTarget {
    PdkExecutionTarget::Mobile
}

#[cfg(all(
    not(target_arch = "wasm32"),
    not(any(target_os = "android", target_os = "ios"))
))]
#[must_use]
pub const fn current_execution_target() -> PdkExecutionTarget {
    PdkExecutionTarget::Desktop
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyLayer {
    pub name: String,
    pub order: u16,
    pub kind: PdkLayerKind,
    pub purposes: Vec<String>,
    pub role: String,
    pub display_rgba: [u8; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkLayerKind {
    Substrate,
    Well,
    Active,
    Poly,
    Metal,
    Via,
    Cut,
    Marker,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkStreamMapEntry {
    pub layer: String,
    pub purpose: String,
    pub stream_layer: u16,
    pub stream_datatype: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkLayerAlias {
    pub alias: String,
    pub layer: String,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkConnectivityEdge {
    pub from_layer: String,
    pub through_layer: String,
    pub to_layer: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkViaDefinition {
    pub via_id: String,
    pub lower_layer: String,
    pub cut_layer: String,
    pub upper_layer: String,
    pub cut_width_meters: f64,
    pub cut_height_meters: f64,
    pub lower_enclosure_meters: f64,
    pub upper_enclosure_meters: f64,
    pub maximum_rows: u16,
    pub maximum_columns: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_rms_current_per_cut_amperes: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkLayerPurposeRef {
    pub layer: String,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkRecognitionTerminal {
    pub terminal_name: String,
    pub layer: String,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkRecognitionQualificationVector {
    pub vector_id: String,
    pub layout_artifact_path: String,
    pub expected_instance_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkRecognitionContract {
    pub contract_id: String,
    pub device_class: String,
    pub rule_artifact_path: String,
    pub terminals: Vec<PdkRecognitionTerminal>,
    pub qualification_vectors: Vec<PdkRecognitionQualificationVector>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkExtractionQuantity {
    Resistance,
    Capacitance,
    CouplingCapacitance,
    Inductance,
    DeviceParameter,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkExtractionQualificationVector {
    pub vector_id: String,
    pub layout_artifact_path: String,
    pub reference_artifact_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkExtractionContract {
    pub contract_id: String,
    pub rule_artifact_path: String,
    pub quantities: Vec<PdkExtractionQuantity>,
    pub layer_purposes: Vec<PdkLayerPurposeRef>,
    pub qualification_vectors: Vec<PdkExtractionQualificationVector>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkCallbackContract {
    pub callback_id: String,
    pub artifact_path: String,
    /// Version of the capability-oriented `rspice` host ABI imported by this
    /// module. Version 1 callbacks export one `() -> i32` entrypoint and a
    /// linear memory named `memory`.
    #[serde(default)]
    pub abi_version: u32,
    /// Exact exported function invoked by the sandbox.
    #[serde(default)]
    pub entrypoint: String,
    pub capabilities: Vec<PdkCallbackCapability>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkCallbackCapability {
    ReadPackage,
    ReadProjectParameters,
    WriteDerivedMetadata,
    Network,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyArtifact {
    pub path: String,
    pub kind: PdkTechnologyArtifactKind,
    pub size_bytes: u64,
    pub sha256: ContentDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkTechnologyArtifactKind {
    Model,
    VerilogASource,
    RuleDeck,
    DisplayResource,
    StreamMap,
    RecognitionMap,
    ExtractionRule,
    QualificationVector,
    QualificationReference,
    Callback,
    Documentation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrustedPdkPublisherKey {
    pub publisher_id: String,
    pub key_id: String,
    pub verifying_key: [u8; 32],
    #[serde(default)]
    pub revoked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkTrustAuditAction {
    Provision,
    Revoke,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTrustAuditReceipt {
    pub sequence: u64,
    pub action: PdkTrustAuditAction,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub publisher_id: String,
    pub key_id: String,
    pub key_fingerprint: ContentDigest,
    pub before_revoked: Option<bool>,
    pub after_revoked: bool,
    pub previous_receipt_digest: Option<ContentDigest>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct PdkTrustAuditPayload<'a> {
    sequence: u64,
    action: PdkTrustAuditAction,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    publisher_id: &'a str,
    key_id: &'a str,
    key_fingerprint: ContentDigest,
    before_revoked: Option<bool>,
    after_revoked: bool,
    previous_receipt_digest: Option<ContentDigest>,
}

impl PdkTrustAuditReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, PdkTechnologyError> {
        let payload = PdkTrustAuditPayload {
            sequence: self.sequence,
            action: self.action,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            publisher_id: &self.publisher_id,
            key_id: &self.key_id,
            key_fingerprint: self.key_fingerprint,
            before_revoked: self.before_revoked,
            after_revoked: self.after_revoked,
            previous_receipt_digest: self.previous_receipt_digest,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkPublisherTrustStore {
    pub keys: Vec<TrustedPdkPublisherKey>,
    #[serde(default)]
    audit: Vec<PdkTrustAuditReceipt>,
}

impl PdkPublisherTrustStore {
    pub fn validate(&self) -> Result<(), PdkTechnologyError> {
        if self.keys.len() > MAX_PDK_PUBLISHER_KEYS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "publisher trust store exceeds {MAX_PDK_PUBLISHER_KEYS} keys"
            )));
        }
        let mut identities = BTreeSet::new();
        for (index, key) in self.keys.iter().enumerate() {
            validate_identifier(
                &format!("trust_store.keys[{index}].publisher_id"),
                &key.publisher_id,
            )?;
            validate_identifier(&format!("trust_store.keys[{index}].key_id"), &key.key_id)?;
            VerifyingKey::from_bytes(&key.verifying_key).map_err(|error| {
                PdkTechnologyError::InvalidTrustStore(format!(
                    "trust_store.keys[{index}].verifying_key is invalid: {error}"
                ))
            })?;
            let identity = (
                key.publisher_id.to_ascii_lowercase(),
                key.key_id.to_ascii_lowercase(),
            );
            if !identities.insert(identity) {
                return Err(PdkTechnologyError::InvalidTrustStore(format!(
                    "trust_store.keys[{index}] repeats a case-insensitive publisher/key identity"
                )));
            }
        }
        self.validate_audit_chain()
    }

    #[must_use]
    pub fn audit(&self) -> &[PdkTrustAuditReceipt] {
        &self.audit
    }

    pub fn provision_key(
        &mut self,
        key: TrustedPdkPublisherKey,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTrustAuditReceipt, PdkTechnologyError> {
        self.validate()?;
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        if key.revoked {
            return Err(PdkTechnologyError::InvalidTrustStore(
                "a newly provisioned key cannot begin revoked".to_owned(),
            ));
        }
        let mut candidate = self.clone();
        if candidate.keys.iter().any(|existing| {
            existing
                .publisher_id
                .eq_ignore_ascii_case(&key.publisher_id)
                && existing.key_id.eq_ignore_ascii_case(&key.key_id)
        }) {
            return Err(PdkTechnologyError::ImmutableTrustKey(format!(
                "{}/{} is already provisioned",
                key.publisher_id, key.key_id
            )));
        }
        candidate.keys.push(key.clone());
        candidate.keys.sort_by(|left, right| {
            (
                left.publisher_id.to_ascii_lowercase(),
                left.key_id.to_ascii_lowercase(),
            )
                .cmp(&(
                    right.publisher_id.to_ascii_lowercase(),
                    right.key_id.to_ascii_lowercase(),
                ))
        });
        let receipt = candidate.append_trust_receipt(
            PdkTrustAuditAction::Provision,
            &key,
            None,
            false,
            authority,
            reason,
        )?;
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    pub fn revoke_key(
        &mut self,
        publisher_id: &str,
        key_id: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTrustAuditReceipt, PdkTechnologyError> {
        self.validate()?;
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        let mut candidate = self.clone();
        let key = candidate
            .keys
            .iter_mut()
            .find(|key| {
                key.publisher_id.eq_ignore_ascii_case(publisher_id)
                    && key.key_id.eq_ignore_ascii_case(key_id)
            })
            .ok_or_else(|| PdkTechnologyError::UntrustedPublisher {
                publisher_id: publisher_id.to_owned(),
                key_id: key_id.to_owned(),
            })?;
        if key.revoked {
            return Err(PdkTechnologyError::ImmutableTrustKey(format!(
                "{}/{} is already revoked",
                key.publisher_id, key.key_id
            )));
        }
        key.revoked = true;
        let key = key.clone();
        let receipt = candidate.append_trust_receipt(
            PdkTrustAuditAction::Revoke,
            &key,
            Some(false),
            true,
            authority,
            reason,
        )?;
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    fn append_trust_receipt(
        &mut self,
        action: PdkTrustAuditAction,
        key: &TrustedPdkPublisherKey,
        before_revoked: Option<bool>,
        after_revoked: bool,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTrustAuditReceipt, PdkTechnologyError> {
        if self.audit.len() >= MAX_PDK_TRUST_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "publisher trust audit receipts exceed {MAX_PDK_TRUST_AUDIT_RECEIPTS}"
            )));
        }
        let mut receipt = PdkTrustAuditReceipt {
            sequence: u64::try_from(self.audit.len())
                .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?
                .checked_add(1)
                .ok_or_else(|| {
                    PdkTechnologyError::LimitExceeded(
                        "publisher trust audit sequence is exhausted".to_owned(),
                    )
                })?,
            action,
            actor_id: authority.actor_id.clone(),
            authority_id: authority.authority_id.clone(),
            reason: reason.to_owned(),
            publisher_id: key.publisher_id.clone(),
            key_id: key.key_id.clone(),
            key_fingerprint: content_digest(&key.verifying_key),
            before_revoked,
            after_revoked,
            previous_receipt_digest: self.audit.last().map(|receipt| receipt.receipt_digest),
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;
        self.audit.push(receipt.clone());
        Ok(receipt)
    }

    fn validate_audit_chain(&self) -> Result<(), PdkTechnologyError> {
        if self.audit.len() > MAX_PDK_TRUST_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                "receipt count exceeds {MAX_PDK_TRUST_AUDIT_RECEIPTS}"
            )));
        }
        let mut previous = None;
        let mut audited = BTreeMap::<(String, String), (ContentDigest, bool)>::new();
        for (index, receipt) in self.audit.iter().enumerate() {
            let expected_sequence = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| {
                    PdkTechnologyError::TrustAuditCorrupted(
                        "receipt sequence is exhausted".to_owned(),
                    )
                })?;
            if receipt.sequence != expected_sequence
                || receipt.previous_receipt_digest != previous
                || receipt.calculate_digest()? != receipt.receipt_digest
            {
                return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                    "receipt #{} has invalid sequence or digest linkage",
                    receipt.sequence
                )));
            }
            validate_identifier("trust_audit.publisher_id", &receipt.publisher_id)?;
            validate_identifier("trust_audit.key_id", &receipt.key_id)?;
            validate_text("trust_audit.reason", &receipt.reason, 1_024)?;
            PdkAdministrativeAuthority {
                actor_id: receipt.actor_id.clone(),
                authority_id: receipt.authority_id.clone(),
            }
            .validate()?;
            let identity = (
                receipt.publisher_id.to_ascii_lowercase(),
                receipt.key_id.to_ascii_lowercase(),
            );
            match receipt.action {
                PdkTrustAuditAction::Provision => {
                    if receipt.before_revoked.is_some()
                        || receipt.after_revoked
                        || audited
                            .insert(identity, (receipt.key_fingerprint, false))
                            .is_some()
                    {
                        return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                            "receipt #{} is not a valid immutable provision transition",
                            receipt.sequence
                        )));
                    }
                }
                PdkTrustAuditAction::Revoke => {
                    if receipt.before_revoked != Some(false) || !receipt.after_revoked {
                        return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                            "receipt #{} is not a valid revocation transition",
                            receipt.sequence
                        )));
                    }
                    if let Some((fingerprint, revoked)) = audited.get_mut(&identity) {
                        if *fingerprint != receipt.key_fingerprint || *revoked {
                            return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                                "receipt #{} revokes a different or already-revoked key",
                                receipt.sequence
                            )));
                        }
                        *revoked = true;
                    } else {
                        // A first receipt may revoke a key provisioned by a
                        // legacy configuration predating trust audit support.
                        audited.insert(identity, (receipt.key_fingerprint, true));
                    }
                }
            }
            previous = Some(receipt.receipt_digest);
        }
        for ((publisher_id, key_id), (fingerprint, revoked)) in audited {
            let key = self
                .keys
                .iter()
                .find(|key| {
                    key.publisher_id.eq_ignore_ascii_case(&publisher_id)
                        && key.key_id.eq_ignore_ascii_case(&key_id)
                })
                .ok_or_else(|| {
                    PdkTechnologyError::TrustAuditCorrupted(format!(
                        "audited key {publisher_id}/{key_id} is absent"
                    ))
                })?;
            if content_digest(&key.verifying_key) != fingerprint || key.revoked != revoked {
                return Err(PdkTechnologyError::TrustAuditCorrupted(format!(
                    "audited key {publisher_id}/{key_id} does not match its final receipt state"
                )));
            }
        }
        Ok(())
    }

    fn resolve(
        &self,
        publisher_id: &str,
        key_id: &str,
    ) -> Result<&TrustedPdkPublisherKey, PdkTechnologyError> {
        self.validate()?;
        let key = self
            .keys
            .iter()
            .find(|candidate| {
                candidate.publisher_id.eq_ignore_ascii_case(publisher_id)
                    && candidate.key_id.eq_ignore_ascii_case(key_id)
            })
            .ok_or_else(|| PdkTechnologyError::UntrustedPublisher {
                publisher_id: publisher_id.to_owned(),
                key_id: key_id.to_owned(),
            })?;
        if key.revoked {
            return Err(PdkTechnologyError::RevokedPublisherKey {
                publisher_id: publisher_id.to_owned(),
                key_id: key_id.to_owned(),
            });
        }
        Ok(key)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedPdkTechnologyPackage {
    manifest: PdkTechnologyManifest,
    manifest_digest: ContentDigest,
    archive_digest: ContentDigest,
    artifact_digests: BTreeMap<String, ContentDigest>,
    symbol_definitions: Vec<crate::state::ModelBoundSymbolDefinition>,
}

impl ValidatedPdkTechnologyPackage {
    #[must_use]
    pub fn manifest(&self) -> &PdkTechnologyManifest {
        &self.manifest
    }

    #[must_use]
    pub const fn manifest_digest(&self) -> ContentDigest {
        self.manifest_digest
    }

    #[must_use]
    pub const fn archive_digest(&self) -> ContentDigest {
        self.archive_digest
    }

    #[must_use]
    pub fn artifact_digests(&self) -> &BTreeMap<String, ContentDigest> {
        &self.artifact_digests
    }

    /// Signed technology symbols materialized against this archive's exact
    /// content-addressed model-source paths.
    #[must_use]
    pub fn symbol_definitions(&self) -> &[crate::state::ModelBoundSymbolDefinition] {
        &self.symbol_definitions
    }

    pub fn runtime_compatibility(&self) -> Result<(), String> {
        validate_runtime_compatibility(&self.manifest).map_err(|error| error.to_string())
    }

    #[must_use]
    pub fn binding(&self) -> PdkTechnologyBinding {
        PdkTechnologyBinding {
            package_id: self.manifest.package_id.clone(),
            revision: self.manifest.revision.clone(),
            manifest_digest: self.manifest_digest,
        }
    }
}

/// Runtime-only, exact source closure produced from one currently trusted
/// signed package. Every path is a content-addressed virtual identity; no
/// worker or browser execution path reopens a host file.
#[derive(Debug, Clone)]
pub(crate) struct SealedPdkModelSources {
    pub(crate) binding: PdkTechnologyBinding,
    pub(crate) archive_digest: ContentDigest,
    pub(crate) sources: Vec<(PathBuf, String)>,
    pub(crate) edges: Vec<rspice_core::netlist::SealedSourceEdge>,
    pub(crate) process_bindings: Vec<SealedPdkModelProcessBinding>,
    pub(crate) veriloga_artifacts: Vec<SealedPdkVerilogAArtifact>,
    pub(crate) veriloga_bindings: Vec<SealedPdkVerilogABinding>,
}

/// One source/section selected by a typed process contract inside a sealed
/// PDK model-source closure.
#[derive(Debug, Clone)]
pub(crate) struct SealedPdkModelProcessBinding {
    pub(crate) process: PdkModelProcess,
    pub(crate) source_id: String,
    pub(crate) domain: PdkModelDomain,
    pub(crate) root_path: PathBuf,
    pub(crate) artifact_path: String,
    pub(crate) artifact_digest: ContentDigest,
    pub(crate) section: Option<String>,
}

/// Exact UTF-8 Verilog-A artifact retained from one authenticated archive.
#[derive(Debug, Clone)]
pub(crate) struct SealedPdkVerilogAArtifact {
    pub(crate) path: String,
    pub(crate) source: String,
    pub(crate) digest: ContentDigest,
}

/// One signed manifest contract whose dependency closure compiled while the
/// package was validated. Execution recompiles these exact retained bytes and
/// checks the resulting runtime at the ordinary prepared-run boundary.
#[derive(Debug, Clone)]
pub(crate) struct SealedPdkVerilogABinding {
    pub(crate) source_id: String,
    pub(crate) root_artifact_path: String,
    pub(crate) root_artifact_digest: ContentDigest,
    pub(crate) module_name: String,
    pub(crate) netlist_alias: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyBinding {
    pub package_id: String,
    pub revision: String,
    pub manifest_digest: ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkAdministrativeAuthority {
    pub actor_id: String,
    pub authority_id: String,
}

impl PdkAdministrativeAuthority {
    pub fn validate(&self) -> Result<(), PdkTechnologyError> {
        validate_identifier("authority.actor_id", &self.actor_id)?;
        validate_identifier("authority.authority_id", &self.authority_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PdkTechnologyAuditAction {
    Install,
    Activate,
    Rollback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyAuditReceipt {
    pub sequence: u64,
    pub action: PdkTechnologyAuditAction,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub target: PdkTechnologyBinding,
    pub archive_digest: ContentDigest,
    pub before_active: Option<PdkTechnologyBinding>,
    pub after_active: Option<PdkTechnologyBinding>,
    pub previous_receipt_digest: Option<ContentDigest>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct PdkTechnologyAuditPayload<'a> {
    sequence: u64,
    action: PdkTechnologyAuditAction,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    target: &'a PdkTechnologyBinding,
    archive_digest: ContentDigest,
    before_active: &'a Option<PdkTechnologyBinding>,
    after_active: &'a Option<PdkTechnologyBinding>,
    previous_receipt_digest: Option<ContentDigest>,
}

impl PdkTechnologyAuditReceipt {
    fn calculate_digest(&self) -> Result<ContentDigest, PdkTechnologyError> {
        let payload = PdkTechnologyAuditPayload {
            sequence: self.sequence,
            action: self.action,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            target: &self.target,
            archive_digest: self.archive_digest,
            before_active: &self.before_active,
            after_active: &self.after_active,
            previous_receipt_digest: self.previous_receipt_digest,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

/// Persisted signed archives and administrative history.  `validated_packages`
/// is intentionally runtime-only: loading serialized state cannot restore
/// trust.  Call `revalidate_installed` with the current trust store before any
/// package can be activated or consumed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkTechnologyRegistry {
    #[serde(default)]
    archives: Vec<SignedPdkTechnologyArchive>,
    #[serde(default)]
    active: Option<PdkTechnologyBinding>,
    #[serde(default)]
    audit: Vec<PdkTechnologyAuditReceipt>,
    #[serde(skip)]
    validated_packages: Vec<ValidatedPdkTechnologyPackage>,
    #[serde(skip)]
    validation_errors: Vec<String>,
}

impl PdkTechnologyRegistry {
    #[must_use]
    pub fn archives(&self) -> &[SignedPdkTechnologyArchive] {
        &self.archives
    }

    /// Resolve the immutable archive behind an exact currently validated
    /// package. This is used by the unsigned authoring export to retain source
    /// artifact bytes without granting the draft executable authority.
    #[must_use]
    pub fn archive_for_package(
        &self,
        package: &ValidatedPdkTechnologyPackage,
    ) -> Option<&SignedPdkTechnologyArchive> {
        self.archives.iter().find(|archive| {
            serde_json::to_vec(archive)
                .ok()
                .is_some_and(|bytes| content_digest(&bytes) == package.archive_digest)
        })
    }

    /// Remove the signed archive payloads from a persistence clone while
    /// retaining its bindings and audit history. Browser persistence stores
    /// these potentially large immutable payloads in a content-addressed
    /// object store instead of embedding them in one configuration record.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(super) fn take_archives_for_browser_persistence(
        &mut self,
    ) -> Vec<SignedPdkTechnologyArchive> {
        self.validated_packages.clear();
        self.validation_errors.clear();
        std::mem::take(&mut self.archives)
    }

    /// Reattach exact signed archive payloads loaded from the browser object
    /// store. This deliberately restores no runtime trust; the caller must
    /// run `revalidate_installed` against the current publisher trust store.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(super) fn restore_archives_from_browser_persistence(
        &mut self,
        archives: Vec<SignedPdkTechnologyArchive>,
    ) -> Result<(), PdkTechnologyError> {
        if !self.archives.is_empty() {
            return Err(PdkTechnologyError::AuditCorrupted(
                "browser PDK metadata unexpectedly contains embedded archives".to_owned(),
            ));
        }
        if archives.len() > MAX_PDK_ARTIFACTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology package registry is limited to {MAX_PDK_ARTIFACTS} installed revisions"
            )));
        }
        self.archives = archives;
        self.validated_packages.clear();
        self.validation_errors.clear();
        Ok(())
    }

    #[must_use]
    pub fn active_binding(&self) -> Option<&PdkTechnologyBinding> {
        self.active.as_ref()
    }

    #[must_use]
    pub fn audit(&self) -> &[PdkTechnologyAuditReceipt] {
        &self.audit
    }

    #[must_use]
    pub fn validated_packages(&self) -> &[ValidatedPdkTechnologyPackage] {
        &self.validated_packages
    }

    #[must_use]
    pub fn validation_errors(&self) -> &[String] {
        &self.validation_errors
    }

    #[must_use]
    pub fn active_package(&self) -> Option<&ValidatedPdkTechnologyPackage> {
        let active = self.active.as_ref()?;
        self.validated_packages.iter().find(|package| {
            package
                .manifest
                .package_id
                .eq_ignore_ascii_case(&active.package_id)
                && package.manifest.revision == active.revision
                && package.manifest_digest == active.manifest_digest
        })
    }

    /// Seal the exact signed model-source closure named by a project pin.
    ///
    /// The project binding, manifest digest, archive digest, decoded artifact
    /// bytes, package-relative dependency graph, and process-section contract
    /// are all checked again before runtime source authority is returned.
    /// This method never falls back to the administratively active revision.
    pub(crate) fn seal_model_sources_for_binding(
        &self,
        binding: &PdkTechnologyBinding,
        expected_archive_digest: ContentDigest,
    ) -> Result<SealedPdkModelSources, PdkTechnologyError> {
        self.validate_audit_chain()?;
        if !self.validation_errors.is_empty() {
            return Err(PdkTechnologyError::NotRuntimeValidated(
                self.validation_errors.join("; "),
            ));
        }
        let package = self
            .validated_packages
            .iter()
            .find(|package| package.binding() == *binding)
            .ok_or_else(|| {
                PdkTechnologyError::NotRuntimeValidated(format!(
                    "{} {} does not resolve to its exact currently trusted manifest",
                    binding.package_id, binding.revision
                ))
            })?;
        if package.archive_digest != expected_archive_digest {
            return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                "{} {} resolves to archive {}, not the project-pinned archive {}",
                binding.package_id,
                binding.revision,
                package.archive_digest,
                expected_archive_digest
            )));
        }
        package.runtime_compatibility().map_err(|detail| {
            PdkTechnologyError::IncompatibleRuntime(format!(
                "{} {}: {detail}",
                binding.package_id, binding.revision
            ))
        })?;

        let matching_archives = self
            .archives
            .iter()
            .filter(|archive| {
                archive_identity(archive).is_ok_and(|candidate| candidate == *binding)
                    && serde_json::to_vec(*archive)
                        .map(|bytes| content_digest(&bytes) == expected_archive_digest)
                        .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        let archive = match matching_archives.as_slice() {
            [archive] => *archive,
            [] => {
                return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                    "{} {} has no exact installed archive for project execution",
                    binding.package_id, binding.revision
                )));
            }
            _ => {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "{} {} resolves to more than one identical archive",
                    binding.package_id, binding.revision
                )));
            }
        };
        seal_pdk_model_sources(archive, package)
    }

    /// Execute one callback from the exact currently trusted archive selected
    /// by a project binding. Administrative activation is deliberately not an
    /// authority source for callback execution.
    pub fn execute_callback_for_binding(
        &self,
        binding: &PdkTechnologyBinding,
        expected_archive_digest: ContentDigest,
        callback_id: &str,
        input: &super::technology_callback::PdkCallbackExecutionInput,
    ) -> Result<
        super::technology_callback::PdkCallbackExecutionReceipt,
        super::technology_callback::PdkCallbackError,
    > {
        self.validate_audit_chain()?;
        if !self.validation_errors.is_empty() {
            return Err(
                PdkTechnologyError::NotRuntimeValidated(self.validation_errors.join("; ")).into(),
            );
        }
        let package = self
            .validated_packages
            .iter()
            .find(|package| package.binding() == *binding)
            .ok_or_else(|| {
                PdkTechnologyError::NotRuntimeValidated(format!(
                    "{} {} does not resolve to its exact currently trusted manifest",
                    binding.package_id, binding.revision
                ))
            })?;
        if package.archive_digest != expected_archive_digest {
            return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                "{} {} resolves to archive {}, not the project-pinned archive {}",
                binding.package_id,
                binding.revision,
                package.archive_digest,
                expected_archive_digest
            ))
            .into());
        }
        package.runtime_compatibility().map_err(|detail| {
            PdkTechnologyError::IncompatibleRuntime(format!(
                "{} {}: {detail}",
                binding.package_id, binding.revision
            ))
        })?;
        let matching_archives = self
            .archives
            .iter()
            .filter(|archive| {
                archive_identity(archive).is_ok_and(|candidate| candidate == *binding)
                    && serde_json::to_vec(*archive)
                        .map(|bytes| content_digest(&bytes) == expected_archive_digest)
                        .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        let archive = match matching_archives.as_slice() {
            [archive] => *archive,
            [] => {
                return Err(PdkTechnologyError::NotRuntimeValidated(format!(
                    "{} {} has no exact installed archive for callback execution",
                    binding.package_id, binding.revision
                ))
                .into());
            }
            _ => {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "{} {} resolves to more than one identical callback archive",
                    binding.package_id, binding.revision
                ))
                .into());
            }
        };
        super::technology_callback::execute_signed_callback(archive, package, callback_id, input)
    }

    #[must_use]
    pub fn runtime_ready(&self) -> bool {
        self.validation_errors.is_empty()
            && (self.active.is_none() || self.active_package().is_some())
    }

    pub fn install_archive_bytes(
        &mut self,
        bytes: &[u8],
        trust_store: &PdkPublisherTrustStore,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        self.validate_audit_chain()?;
        let (archive, package) = validate_archive_bytes(bytes, trust_store)?;
        let binding = package.binding();
        if self.archives.iter().any(|installed| {
            archive_identity(installed).is_ok_and(|candidate| {
                candidate
                    .package_id
                    .eq_ignore_ascii_case(&binding.package_id)
                    && candidate.revision == binding.revision
            })
        }) {
            return Err(PdkTechnologyError::ImmutableRevision(format!(
                "{} {} is already installed; signed revisions are immutable",
                binding.package_id, binding.revision
            )));
        }
        if self.archives.len() >= MAX_PDK_ARTIFACTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology package registry is limited to {MAX_PDK_ARTIFACTS} installed revisions"
            )));
        }

        let receipt = self.next_receipt(
            PdkTechnologyAuditAction::Install,
            authority,
            reason,
            binding,
            package.archive_digest,
            self.active.clone(),
            self.active.clone(),
        )?;
        self.archives.push(archive);
        self.validated_packages.push(package);
        self.sort_packages();
        self.audit.push(receipt.clone());
        Ok(receipt)
    }

    /// Rebuild all runtime trust from the exact persisted archives.  Failure
    /// publishes neither a partial validated catalog nor executable authority.
    pub fn revalidate_installed(
        &mut self,
        trust_store: &PdkPublisherTrustStore,
    ) -> Result<usize, Vec<String>> {
        if let Err(error) = trust_store.validate() {
            self.validated_packages.clear();
            self.validation_errors = vec![error.to_string()];
            return Err(self.validation_errors.clone());
        }
        if let Err(error) = self.validate_audit_chain() {
            self.validated_packages.clear();
            self.validation_errors = vec![error.to_string()];
            return Err(self.validation_errors.clone());
        }
        let mut packages = Vec::with_capacity(self.archives.len());
        let mut errors = Vec::new();
        for (index, archive) in self.archives.iter().enumerate() {
            match validate_archive(archive, trust_store) {
                Ok(package) => packages.push(package),
                Err(error) => errors.push(format!("archive[{index}]: {error}")),
            }
        }
        if errors.is_empty() {
            for (index, receipt) in self.audit.iter().enumerate() {
                let Some(package) = packages
                    .iter()
                    .find(|package| package.binding() == receipt.target)
                else {
                    errors.push(format!(
                        "receipt[{index}] references package {} {} that is not installed with its exact signed manifest",
                        receipt.target.package_id, receipt.target.revision
                    ));
                    continue;
                };
                if receipt.archive_digest != package.archive_digest {
                    errors.push(format!(
                        "receipt[{index}] archive digest does not match package {} {}",
                        receipt.target.package_id, receipt.target.revision
                    ));
                }
            }
        }
        if errors.is_empty() {
            packages.sort_by(package_order);
            if let Some(active) = &self.active {
                match packages
                    .iter()
                    .find(|package| package.binding() == *active)
                {
                    None => errors.push(format!(
                        "active binding {} {} does not resolve to an exact currently trusted package",
                        active.package_id, active.revision
                    )),
                    Some(package) => {
                        if let Err(error) = validate_runtime_compatibility(&package.manifest) {
                            errors.push(format!(
                                "active binding {} {} is incompatible: {error}",
                                active.package_id, active.revision
                            ));
                        }
                    }
                }
            }
        }
        if errors.is_empty() {
            self.validated_packages = packages;
            self.validation_errors.clear();
            Ok(self.validated_packages.len())
        } else {
            self.validated_packages.clear();
            self.validation_errors = errors.clone();
            Err(errors)
        }
    }

    pub fn activate(
        &mut self,
        package_id: &str,
        revision: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        self.activate_as(
            PdkTechnologyAuditAction::Activate,
            package_id,
            revision,
            authority,
            reason,
        )
    }

    pub fn rollback_to(
        &mut self,
        package_id: &str,
        revision: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        let appeared_before = self.audit.iter().any(|receipt| {
            receipt.after_active.as_ref().is_some_and(|binding| {
                binding.package_id.eq_ignore_ascii_case(package_id) && binding.revision == revision
            })
        });
        if !appeared_before {
            return Err(PdkTechnologyError::InvalidTransition(format!(
                "{package_id} {revision} has never been an active trusted binding"
            )));
        }
        self.activate_as(
            PdkTechnologyAuditAction::Rollback,
            package_id,
            revision,
            authority,
            reason,
        )
    }

    pub fn validate_audit_chain(&self) -> Result<(), PdkTechnologyError> {
        if self.audit.len() > MAX_PDK_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology audit exceeds {MAX_PDK_AUDIT_RECEIPTS} receipts"
            )));
        }
        let mut previous = None;
        let mut expected_active = None;
        for (index, receipt) in self.audit.iter().enumerate() {
            let expected_sequence = u64::try_from(index).map_err(|_| {
                PdkTechnologyError::LimitExceeded("audit index overflow".to_owned())
            })? + 1;
            if receipt.sequence != expected_sequence {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "receipt[{index}] sequence is {}, expected {expected_sequence}",
                    receipt.sequence
                )));
            }
            if receipt.previous_receipt_digest != previous {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "receipt[{index}] does not bind the exact previous receipt"
                )));
            }
            validate_identifier(&format!("audit[{index}].actor_id"), &receipt.actor_id)?;
            validate_identifier(
                &format!("audit[{index}].authority_id"),
                &receipt.authority_id,
            )?;
            validate_text(&format!("audit[{index}].reason"), &receipt.reason, 1_024)?;
            validate_identifier(
                &format!("audit[{index}].target.package_id"),
                &receipt.target.package_id,
            )?;
            validate_version(
                &format!("audit[{index}].target.revision"),
                &receipt.target.revision,
            )?;
            if receipt.calculate_digest()? != receipt.receipt_digest {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "receipt[{index}] content digest does not match its payload"
                )));
            }
            if receipt.before_active != expected_active {
                return Err(PdkTechnologyError::AuditCorrupted(format!(
                    "receipt[{index}] before_active does not match the preceding transaction"
                )));
            }
            match receipt.action {
                PdkTechnologyAuditAction::Install => {
                    if receipt.after_active != receipt.before_active {
                        return Err(PdkTechnologyError::AuditCorrupted(format!(
                            "receipt[{index}] install transaction changes the active binding"
                        )));
                    }
                }
                PdkTechnologyAuditAction::Activate | PdkTechnologyAuditAction::Rollback => {
                    if receipt.after_active.as_ref() != Some(&receipt.target) {
                        return Err(PdkTechnologyError::AuditCorrupted(format!(
                            "receipt[{index}] activation target and resulting binding differ"
                        )));
                    }
                    if receipt.after_active == receipt.before_active {
                        return Err(PdkTechnologyError::AuditCorrupted(format!(
                            "receipt[{index}] records a no-op activation transition"
                        )));
                    }
                }
            }
            expected_active = receipt.after_active.clone();
            previous = Some(receipt.receipt_digest);
        }
        if self.active != expected_active {
            return Err(PdkTechnologyError::AuditCorrupted(
                "registry active binding does not match the final audit transaction".to_owned(),
            ));
        }
        Ok(())
    }

    fn activate_as(
        &mut self,
        action: PdkTechnologyAuditAction,
        package_id: &str,
        revision: &str,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        authority.validate()?;
        validate_text("reason", reason, 1_024)?;
        self.validate_audit_chain()?;
        let package = self
            .validated_packages
            .iter()
            .find(|candidate| {
                candidate
                    .manifest
                    .package_id
                    .eq_ignore_ascii_case(package_id)
                    && candidate.manifest.revision == revision
            })
            .cloned()
            .ok_or_else(|| {
                PdkTechnologyError::NotRuntimeValidated(format!(
                    "{package_id} {revision} is not present in the current trusted runtime catalog"
                ))
            })?;
        let after = package.binding();
        validate_runtime_compatibility(&package.manifest)?;
        if self.active.as_ref() == Some(&after) {
            return Err(PdkTechnologyError::InvalidTransition(format!(
                "{} {} is already active",
                after.package_id, after.revision
            )));
        }
        let before = self.active.clone();
        let receipt = self.next_receipt(
            action,
            authority,
            reason,
            after.clone(),
            package.archive_digest,
            before,
            Some(after.clone()),
        )?;
        self.active = Some(after);
        self.audit.push(receipt.clone());
        Ok(receipt)
    }

    fn next_receipt(
        &self,
        action: PdkTechnologyAuditAction,
        authority: &PdkAdministrativeAuthority,
        reason: &str,
        target: PdkTechnologyBinding,
        archive_digest: ContentDigest,
        before_active: Option<PdkTechnologyBinding>,
        after_active: Option<PdkTechnologyBinding>,
    ) -> Result<PdkTechnologyAuditReceipt, PdkTechnologyError> {
        if self.audit.len() >= MAX_PDK_AUDIT_RECEIPTS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "technology audit is limited to {MAX_PDK_AUDIT_RECEIPTS} receipts"
            )));
        }
        let sequence = u64::try_from(self.audit.len())
            .map_err(|_| PdkTechnologyError::LimitExceeded("audit index overflow".to_owned()))?
            + 1;
        let mut receipt = PdkTechnologyAuditReceipt {
            sequence,
            action,
            actor_id: authority.actor_id.clone(),
            authority_id: authority.authority_id.clone(),
            reason: reason.to_owned(),
            target,
            archive_digest,
            before_active,
            after_active,
            previous_receipt_digest: self.audit.last().map(|receipt| receipt.receipt_digest),
            receipt_digest: content_digest(&[]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;
        Ok(receipt)
    }

    fn sort_packages(&mut self) {
        self.archives.sort_by(|left, right| {
            let left = archive_identity(left).ok();
            let right = archive_identity(right).ok();
            left.as_ref()
                .map(|binding| {
                    (
                        binding.package_id.to_ascii_lowercase(),
                        binding.revision.clone(),
                    )
                })
                .cmp(&right.as_ref().map(|binding| {
                    (
                        binding.package_id.to_ascii_lowercase(),
                        binding.revision.clone(),
                    )
                }))
        });
        self.validated_packages.sort_by(package_order);
    }
}

pub fn validate_archive_bytes(
    bytes: &[u8],
    trust_store: &PdkPublisherTrustStore,
) -> Result<(SignedPdkTechnologyArchive, ValidatedPdkTechnologyPackage), PdkTechnologyError> {
    if bytes.len() > MAX_PDK_ARCHIVE_BYTES {
        return Err(PdkTechnologyError::ArchiveTooLarge {
            actual: bytes.len(),
            maximum: MAX_PDK_ARCHIVE_BYTES,
        });
    }
    let archive: SignedPdkTechnologyArchive = serde_json::from_slice(bytes)
        .map_err(|error| PdkTechnologyError::ArchiveParse(error.to_string()))?;
    let package = validate_archive(&archive, trust_store)?;
    Ok((archive, package))
}

pub fn validate_archive(
    archive: &SignedPdkTechnologyArchive,
    trust_store: &PdkPublisherTrustStore,
) -> Result<ValidatedPdkTechnologyPackage, PdkTechnologyError> {
    if archive.schema_version != PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION {
        return Err(PdkTechnologyError::UnsupportedSchema {
            object: "archive",
            actual: archive.schema_version,
            supported: PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION,
        });
    }
    if archive.files.len() > MAX_PDK_ARTIFACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "archive has {} files; maximum is {MAX_PDK_ARTIFACTS}",
            archive.files.len()
        )));
    }
    let manifest_bytes = decode_bounded(
        "manifest_base64",
        &archive.manifest_base64,
        MAX_PDK_MANIFEST_BYTES,
    )?;
    let manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| PdkTechnologyError::ManifestParse(error.to_string()))?;
    validate_manifest(&manifest)?;

    let signature_bytes = decode_bounded("signature_base64", &archive.signature_base64, 64)?;
    trust_store.verify_publisher_signature(
        &manifest.publisher_id,
        &manifest.signing_key_id,
        &manifest_bytes,
        &signature_bytes,
    )?;

    let mut actual_files = BTreeMap::<String, (usize, ContentDigest)>::new();
    let mut total = 0usize;
    for (index, file) in archive.files.iter().enumerate() {
        validate_package_path(&format!("files[{index}].path"), &file.path)?;
        let normalized = file.path.to_ascii_lowercase();
        if actual_files.contains_key(&normalized) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "archive repeats case-insensitive path '{}'",
                file.path
            )));
        }
        let bytes = decode_bounded(
            &format!("files[{index}].content_base64"),
            &file.content_base64,
            MAX_PDK_ARTIFACT_BYTES,
        )?;
        total = total.checked_add(bytes.len()).ok_or_else(|| {
            PdkTechnologyError::LimitExceeded("archive byte count overflow".to_owned())
        })?;
        if total > MAX_PDK_TOTAL_ARTIFACT_BYTES {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "decoded artifact bytes exceed {MAX_PDK_TOTAL_ARTIFACT_BYTES}"
            )));
        }
        actual_files.insert(normalized, (bytes.len(), content_digest(&bytes)));
    }

    let mut artifact_digests = BTreeMap::new();
    for (index, artifact) in manifest.artifacts.iter().enumerate() {
        let key = artifact.path.to_ascii_lowercase();
        let Some((actual_size, actual_digest)) = actual_files.remove(&key) else {
            return Err(PdkTechnologyError::MissingArtifact(artifact.path.clone()));
        };
        let declared_size = usize::try_from(artifact.size_bytes).map_err(|_| {
            PdkTechnologyError::InvalidField(format!(
                "artifacts[{index}].size_bytes cannot be represented on this platform"
            ))
        })?;
        if declared_size != actual_size {
            return Err(PdkTechnologyError::ArtifactSizeMismatch {
                path: artifact.path.clone(),
                declared: artifact.size_bytes,
                actual: actual_size,
            });
        }
        if artifact.sha256 != actual_digest {
            return Err(PdkTechnologyError::ArtifactDigestMismatch {
                path: artifact.path.clone(),
                declared: artifact.sha256,
                actual: actual_digest,
            });
        }
        artifact_digests.insert(artifact.path.clone(), actual_digest);
    }
    if let Some((extra, _)) = actual_files.first_key_value() {
        return Err(PdkTechnologyError::UndeclaredArtifact(extra.clone()));
    }

    let archive_digest = content_digest(
        &serde_json::to_vec(archive)
            .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?,
    );
    let symbol_definitions = materialize_signed_symbol_definitions(&manifest, archive_digest)?;
    let package = ValidatedPdkTechnologyPackage {
        manifest,
        manifest_digest: content_digest(&manifest_bytes),
        // JSON envelope whitespace is intentionally not part of package
        // identity. The signature binds the exact manifest bytes and each
        // manifest digest binds exact decoded artifact bytes. This digest
        // binds the complete normalized envelope identically before and after
        // persistence.
        archive_digest,
        artifact_digests,
        symbol_definitions,
    };
    super::technology_callback::validate_signed_callbacks(archive, &package)
        .map_err(PdkTechnologyError::CallbackValidation)?;
    // Executable model contracts are part of package validation, not a
    // deferred simulation-time best effort. This proves section existence,
    // package-relative dependency closure, source encoding, and reachability
    // using the exact decoded artifact bytes covered by the signed manifest.
    let _ = seal_pdk_model_sources(archive, &package)?;
    Ok(package)
}

fn materialize_signed_symbol_definitions(
    manifest: &PdkTechnologyManifest,
    archive_digest: ContentDigest,
) -> Result<Vec<crate::state::ModelBoundSymbolDefinition>, PdkTechnologyError> {
    let virtual_root = signed_model_virtual_root(&archive_digest.to_string());
    let mut definitions = Vec::with_capacity(manifest.symbol_definitions.len());
    for (index, signed) in manifest.symbol_definitions.iter().enumerate() {
        let mut definition = signed.clone();
        let crate::state::SymbolSourceContract::Model { model, .. } = &mut definition.source else {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] is not model-bound"
            )));
        };
        let package_path = model.source_path.as_deref().ok_or_else(|| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] has no model source path"
            ))
        })?;
        let source_path = virtual_root
            .join(package_path_to_host_path(package_path))
            .to_string_lossy()
            .into_owned();
        model.source_path = Some(source_path.clone());
        definition
            .netlist
            .model
            .as_mut()
            .ok_or_else(|| {
                PdkTechnologyError::InvalidField(format!(
                    "manifest.symbol_definitions[{index}] has no executable model binding"
                ))
            })?
            .source_path = Some(source_path);
        definition.validate().map_err(|error| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] is invalid after signed source materialization: {error}"
            ))
        })?;
        definitions.push(definition);
    }
    definitions.sort_by(|left, right| {
        left.identity
            .cell
            .to_ascii_lowercase()
            .cmp(&right.identity.cell.to_ascii_lowercase())
    });
    Ok(definitions)
}

fn seal_pdk_model_sources(
    archive: &SignedPdkTechnologyArchive,
    package: &ValidatedPdkTechnologyPackage,
) -> Result<SealedPdkModelSources, PdkTechnologyError> {
    let binding = package.binding();
    let archive_bytes = serde_json::to_vec(archive)
        .map_err(|error| PdkTechnologyError::Serialization(error.to_string()))?;
    if content_digest(&archive_bytes) != package.archive_digest {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "{} {} archive bytes no longer match the validated archive digest",
            binding.package_id, binding.revision
        )));
    }

    let model_artifacts = package
        .manifest
        .artifacts
        .iter()
        .filter(|artifact| artifact.kind == PdkTechnologyArtifactKind::Model)
        .map(|artifact| (artifact.path.to_ascii_lowercase(), artifact))
        .collect::<BTreeMap<_, _>>();
    let (veriloga_artifacts, veriloga_bindings) = seal_pdk_veriloga_sources(archive, package)?;
    if model_artifacts.is_empty() {
        return Ok(SealedPdkModelSources {
            binding,
            archive_digest: package.archive_digest,
            sources: Vec::new(),
            edges: Vec::new(),
            process_bindings: Vec::new(),
            veriloga_artifacts,
            veriloga_bindings,
        });
    }

    let archive_files = archive
        .files
        .iter()
        .map(|file| (file.path.to_ascii_lowercase(), file))
        .collect::<BTreeMap<_, _>>();
    let virtual_root = signed_model_virtual_root(&package.archive_digest.to_string());
    let mut virtual_paths = BTreeMap::<String, PathBuf>::new();
    let mut sources = Vec::<(PathBuf, String)>::with_capacity(model_artifacts.len());
    for (key, artifact) in &model_artifacts {
        let file = archive_files
            .get(key)
            .ok_or_else(|| PdkTechnologyError::MissingArtifact(artifact.path.clone()))?;
        let bytes = decode_bounded(
            &format!("files[{}].content_base64", file.path),
            &file.content_base64,
            MAX_PDK_ARTIFACT_BYTES,
        )?;
        let actual_digest = content_digest(&bytes);
        if actual_digest != artifact.sha256
            || package.artifact_digests.get(&artifact.path) != Some(&actual_digest)
        {
            return Err(PdkTechnologyError::ArtifactDigestMismatch {
                path: artifact.path.clone(),
                declared: artifact.sha256,
                actual: actual_digest,
            });
        }
        let actual_size = bytes.len();
        if u64::try_from(actual_size).ok() != Some(artifact.size_bytes) {
            return Err(PdkTechnologyError::ArtifactSizeMismatch {
                path: artifact.path.clone(),
                declared: artifact.size_bytes,
                actual: actual_size,
            });
        }
        let source = rspice_core::netlist::decode_source_bytes(&bytes).map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "model artifact '{}' cannot be decoded with the supported source policy: {error}",
                artifact.path
            ))
        })?;
        let path = virtual_root.join(package_path_to_host_path(&artifact.path));
        virtual_paths.insert(key.clone(), path.clone());
        sources.push((path, source));
    }
    sources.sort_by(|left, right| left.0.cmp(&right.0));

    let source_by_path = sources
        .iter()
        .map(|(path, source)| (path.clone(), source.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut edge_keys = BTreeSet::<(PathBuf, String, PathBuf)>::new();
    for (artifact_key, owner_path) in &virtual_paths {
        let source = source_by_path
            .get(owner_path)
            .expect("every virtual model path has decoded source");
        let owner_artifact = model_artifacts
            .get(artifact_key)
            .expect("every virtual model path names an artifact");
        for requested_path in pdk_external_source_paths(source) {
            let target_artifact_path =
                resolve_package_dependency(&owner_artifact.path, &requested_path)?;
            let target_key = target_artifact_path.to_ascii_lowercase();
            let target = virtual_paths.get(&target_key).ok_or_else(|| {
                PdkTechnologyError::ModelMaterialization(format!(
                    "model artifact '{}' references '{}' which is not a signed model artifact in this package",
                    owner_artifact.path, requested_path
                ))
            })?;
            edge_keys.insert((owner_path.clone(), requested_path, target.clone()));
        }
    }
    let edges = edge_keys
        .into_iter()
        .map(
            |(owner, requested_path, target)| rspice_core::netlist::SealedSourceEdge {
                owner,
                requested_path,
                target,
            },
        )
        .collect::<Vec<_>>();
    let bundle = rspice_core::netlist::SealedSourceBundle::try_new_with_edges(
        sources.clone(),
        edges.clone(),
    )
    .map_err(|error| {
        PdkTechnologyError::ModelMaterialization(format!(
            "signed model-source bundle is invalid: {error}"
        ))
    })?;

    let mut process_bindings = Vec::new();
    let mut reachable_artifacts = BTreeSet::<String>::new();
    for contract in &package.manifest.model_sources {
        for source in &contract.sources {
            let artifact_key = source.artifact_path.to_ascii_lowercase();
            let artifact = model_artifacts.get(&artifact_key).ok_or_else(|| {
                PdkTechnologyError::InvalidReference(format!(
                    "model source '{}' references missing model artifact '{}'",
                    source.source_id, source.artifact_path
                ))
            })?;
            let root_path = virtual_paths
                .get(&artifact_key)
                .expect("validated model artifact has a virtual path")
                .clone();
            let mut processor =
                rspice_core::netlist::IncludeProcessor::new_sealed(&root_path, bundle.clone());
            let materialized = processor
                .process_sealed_root(&root_path, source.section.as_deref())
                .map_err(|error| {
                    PdkTechnologyError::ModelMaterialization(format!(
                        "{} {} process {} source '{}' from '{}' could not be materialized: {error}",
                        binding.package_id,
                        binding.revision,
                        contract.process.keyword(),
                        source.source_id,
                        source.artifact_path
                    ))
                })?;
            if materialized.trim().is_empty() {
                return Err(PdkTechnologyError::ModelMaterialization(format!(
                    "{} {} process {} source '{}' materializes no executable model cards",
                    binding.package_id,
                    binding.revision,
                    contract.process.keyword(),
                    source.source_id
                )));
            }
            reachable_artifacts.insert(artifact_key);
            for dependency in processor.resolved_dependencies() {
                let dependency_key = virtual_paths
                    .iter()
                    .find_map(|(key, path)| {
                        (path == dependency.resolved_path()).then(|| key.clone())
                    })
                    .ok_or_else(|| {
                        PdkTechnologyError::ModelMaterialization(format!(
                            "resolved dependency '{}' escaped the signed package closure",
                            dependency.resolved_path().display()
                        ))
                    })?;
                reachable_artifacts.insert(dependency_key);
            }
            process_bindings.push(SealedPdkModelProcessBinding {
                process: contract.process,
                source_id: source.source_id.clone(),
                domain: source.domain,
                root_path,
                artifact_path: artifact.path.clone(),
                artifact_digest: artifact.sha256,
                section: source.section.clone(),
            });
        }
    }
    process_bindings.sort_by(|left, right| {
        left.process
            .cmp(&right.process)
            .then_with(|| left.domain.cmp(&right.domain))
            .then_with(|| {
                left.source_id
                    .to_ascii_lowercase()
                    .cmp(&right.source_id.to_ascii_lowercase())
            })
            .then_with(|| left.source_id.cmp(&right.source_id))
    });
    let unreachable = model_artifacts
        .keys()
        .filter(|path| !reachable_artifacts.contains(*path))
        .cloned()
        .collect::<Vec<_>>();
    if !unreachable.is_empty() {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "signed model artifacts are unreachable from every declared process contract: {}",
            unreachable.join(", ")
        )));
    }

    Ok(SealedPdkModelSources {
        binding,
        archive_digest: package.archive_digest,
        sources,
        edges,
        process_bindings,
        veriloga_artifacts,
        veriloga_bindings,
    })
}

fn seal_pdk_veriloga_sources(
    archive: &SignedPdkTechnologyArchive,
    package: &ValidatedPdkTechnologyPackage,
) -> Result<
    (
        Vec<SealedPdkVerilogAArtifact>,
        Vec<SealedPdkVerilogABinding>,
    ),
    PdkTechnologyError,
> {
    let declared = package
        .manifest
        .artifacts
        .iter()
        .filter(|artifact| artifact.kind == PdkTechnologyArtifactKind::VerilogASource)
        .map(|artifact| (artifact.path.to_ascii_lowercase(), artifact))
        .collect::<BTreeMap<_, _>>();
    if declared.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }

    let archive_files = archive
        .files
        .iter()
        .map(|file| (file.path.to_ascii_lowercase(), file))
        .collect::<BTreeMap<_, _>>();
    let mut artifacts = Vec::with_capacity(declared.len());
    for (key, artifact) in &declared {
        let file = archive_files
            .get(key)
            .ok_or_else(|| PdkTechnologyError::MissingArtifact(artifact.path.clone()))?;
        let bytes = decode_bounded(
            &format!("files[{}].content_base64", file.path),
            &file.content_base64,
            MAX_PDK_ARTIFACT_BYTES,
        )?;
        let actual_digest = content_digest(&bytes);
        if actual_digest != artifact.sha256
            || package.artifact_digests.get(&artifact.path) != Some(&actual_digest)
        {
            return Err(PdkTechnologyError::ArtifactDigestMismatch {
                path: artifact.path.clone(),
                declared: artifact.sha256,
                actual: actual_digest,
            });
        }
        if u64::try_from(bytes.len()).ok() != Some(artifact.size_bytes) {
            return Err(PdkTechnologyError::ArtifactSizeMismatch {
                path: artifact.path.clone(),
                declared: artifact.size_bytes,
                actual: bytes.len(),
            });
        }
        let source = rspice_core::netlist::decode_source_bytes(&bytes).map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "Verilog-A artifact '{}' cannot be decoded with the supported source policy: {error}",
                artifact.path
            ))
        })?;
        artifacts.push(SealedPdkVerilogAArtifact {
            path: artifact.path.clone(),
            source,
            digest: actual_digest,
        });
    }
    artifacts.sort_by(|left, right| {
        left.path
            .to_ascii_lowercase()
            .cmp(&right.path.to_ascii_lowercase())
            .then_with(|| left.path.cmp(&right.path))
    });

    let virtual_files = artifacts
        .iter()
        .map(|artifact| rspice_veriloga::VirtualSourceFile::new(&artifact.path, &artifact.source))
        .collect::<Vec<_>>();
    let limits = rspice_veriloga::VirtualCompileLimits {
        max_files: MAX_PDK_ARTIFACTS,
        max_path_bytes: 1_024,
        max_file_bytes: MAX_PDK_ARTIFACT_BYTES,
        max_total_source_bytes: MAX_PDK_TOTAL_ARTIFACT_BYTES,
        max_include_depth: 64,
        max_expanded_bytes: MAX_PDK_TOTAL_ARTIFACT_BYTES.saturating_mul(2),
        max_module_name_bytes: 128,
    };
    let mut reachable = BTreeSet::<String>::new();
    let mut bindings = Vec::with_capacity(package.manifest.veriloga_sources.len());
    for contract in &package.manifest.veriloga_sources {
        let root_key = contract.root_artifact_path.to_ascii_lowercase();
        let root = declared.get(&root_key).ok_or_else(|| {
            PdkTechnologyError::InvalidReference(format!(
                "Verilog-A source '{}' references missing artifact '{}'",
                contract.source_id, contract.root_artifact_path
            ))
        })?;
        let bundle = rspice_veriloga::VirtualSourceBundle::new(
            &contract.root_artifact_path,
            virtual_files.clone(),
        )
        .map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "signed Verilog-A source bundle for '{}' is invalid: {error}",
                contract.source_id
            ))
        })?;
        let compilation = rspice_veriloga::VerilogACompiler::default()
            .compile_virtual_runtime(&bundle, &contract.module_name, limits)
            .map_err(|error| {
                PdkTechnologyError::ModelMaterialization(format!(
                    "signed Verilog-A source '{}' module '{}' could not be compiled: {error}",
                    contract.source_id, contract.module_name
                ))
            })?;
        compilation.validate_integrity().map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "compiled Verilog-A source '{}' failed its integrity check: {error}",
                contract.source_id
            ))
        })?;
        for dependency in &compilation.dependency_closure {
            let key = dependency.logical_path.to_ascii_lowercase();
            if declared.contains_key(&key) {
                reachable.insert(key);
            }
        }
        bindings.push(SealedPdkVerilogABinding {
            source_id: contract.source_id.clone(),
            root_artifact_path: root.path.clone(),
            root_artifact_digest: root.sha256,
            module_name: contract.module_name.clone(),
            netlist_alias: contract.netlist_alias.clone(),
        });
    }
    bindings.sort_by(|left, right| {
        left.source_id
            .to_ascii_lowercase()
            .cmp(&right.source_id.to_ascii_lowercase())
            .then_with(|| left.source_id.cmp(&right.source_id))
    });
    let unreachable = declared
        .keys()
        .filter(|path| !reachable.contains(*path))
        .cloned()
        .collect::<Vec<_>>();
    if !unreachable.is_empty() {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "signed Verilog-A artifacts are unreachable from every declared runtime contract: {}",
            unreachable.join(", ")
        )));
    }
    Ok((artifacts, bindings))
}

fn package_path_to_host_path(path: &str) -> PathBuf {
    path.split('/').collect()
}

fn signed_model_virtual_root(identity: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(format!(r"C:\rspice-pdk\model-sources\{identity}"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        PathBuf::from(format!("/rspice-pdk/model-sources/{identity}"))
    }
}

fn pdk_external_source_paths(source: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in source.lines() {
        if let Some((path, section)) = rspice_core::netlist::parse_lib_directive(line) {
            if section.is_some() {
                paths.push(path);
            }
            continue;
        }
        if let Some(path) = rspice_core::netlist::parse_include_directive(line) {
            paths.push(path);
        }
    }
    paths
}

fn resolve_package_dependency(
    owner_artifact_path: &str,
    requested_path: &str,
) -> Result<String, PdkTechnologyError> {
    let requested =
        rspice_core::netlist::normalize_source_path_literal(requested_path).map_err(|error| {
            PdkTechnologyError::ModelMaterialization(format!(
                "model artifact '{owner_artifact_path}' contains invalid dependency path '{requested_path}': {error}"
            ))
        })?;
    let requested = requested.replace('\\', "/");
    if requested.starts_with('/')
        || requested.starts_with("//")
        || requested
            .as_bytes()
            .get(1)
            .is_some_and(|separator| *separator == b':')
    {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "model artifact '{owner_artifact_path}' requests external absolute dependency '{requested_path}'"
        )));
    }

    let mut components = owner_artifact_path
        .split('/')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    components.pop();
    for component in requested.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                if components.pop().is_none() {
                    return Err(PdkTechnologyError::ModelMaterialization(format!(
                        "model artifact '{owner_artifact_path}' dependency '{requested_path}' escapes the signed package root"
                    )));
                }
            }
            component => components.push(component.to_owned()),
        }
    }
    if components.is_empty() {
        return Err(PdkTechnologyError::ModelMaterialization(format!(
            "model artifact '{owner_artifact_path}' dependency '{requested_path}' has no package target"
        )));
    }
    let resolved = components.join("/");
    validate_package_path("model dependency target", &resolved)?;
    Ok(resolved)
}

pub(super) fn validate_manifest(
    manifest: &PdkTechnologyManifest,
) -> Result<(), PdkTechnologyError> {
    if !(MINIMUM_PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION..=PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION)
        .contains(&manifest.schema_version)
    {
        return Err(PdkTechnologyError::UnsupportedSchema {
            object: "manifest",
            actual: manifest.schema_version,
            supported: PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION,
        });
    }
    validate_identifier("manifest.package_id", &manifest.package_id)?;
    validate_text("manifest.technology_name", &manifest.technology_name, 256)?;
    validate_version("manifest.revision", &manifest.revision)?;
    validate_identifier("manifest.publisher_id", &manifest.publisher_id)?;
    validate_identifier("manifest.signing_key_id", &manifest.signing_key_id)?;
    validate_text("manifest.license_spdx", &manifest.license_spdx, 128)?;
    validate_text("manifest.stack_name", &manifest.stack_name, 256)?;
    if manifest.process_node_nm == 0 || manifest.process_node_nm > 1_000_000 {
        return Err(PdkTechnologyError::InvalidField(
            "manifest.process_node_nm must be in 1..=1000000".to_owned(),
        ));
    }
    if !manifest.database_unit_meters.is_finite()
        || !(1.0e-12..=1.0e-3).contains(&manifest.database_unit_meters)
    {
        return Err(PdkTechnologyError::InvalidField(
            "manifest.database_unit_meters must be finite and in 1e-12..=1e-3".to_owned(),
        ));
    }
    validate_version(
        "manifest.compatibility.minimum_engine_version",
        &manifest.compatibility.minimum_engine_version,
    )?;
    validate_version(
        "manifest.compatibility.minimum_viewer_version",
        &manifest.compatibility.minimum_viewer_version,
    )?;
    let targets = manifest
        .compatibility
        .targets
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if targets.len() != manifest.compatibility.targets.len() {
        return Err(PdkTechnologyError::Duplicate(
            "manifest.compatibility.targets contains duplicates".to_owned(),
        ));
    }
    if targets.is_empty() {
        return Err(PdkTechnologyError::InvalidField(
            "manifest.compatibility.targets must declare at least one permitted execution target"
                .to_owned(),
        ));
    }

    if manifest.layers.is_empty() || manifest.layers.len() > MAX_PDK_LAYERS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.layers must contain 1..={MAX_PDK_LAYERS} entries"
        )));
    }
    let mut layers = BTreeMap::<String, BTreeSet<String>>::new();
    let mut layer_kinds = BTreeMap::<String, PdkLayerKind>::new();
    let mut orders = BTreeSet::new();
    for (index, layer) in manifest.layers.iter().enumerate() {
        validate_identifier(&format!("manifest.layers[{index}].name"), &layer.name)?;
        validate_text(&format!("manifest.layers[{index}].role"), &layer.role, 256)?;
        if !orders.insert(layer.order) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.layers[{index}].order {} is repeated",
                layer.order
            )));
        }
        if layer.purposes.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.layers[{index}].purposes is empty"
            )));
        }
        let key = layer.name.to_ascii_lowercase();
        if layers.contains_key(&key) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.layers repeats case-insensitive layer '{}'",
                layer.name
            )));
        }
        let mut purposes = BTreeSet::new();
        for (purpose_index, purpose) in layer.purposes.iter().enumerate() {
            validate_identifier(
                &format!("manifest.layers[{index}].purposes[{purpose_index}]"),
                purpose,
            )?;
            if !purposes.insert(purpose.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.layers[{index}] repeats purpose '{purpose}'"
                )));
            }
        }
        layer_kinds.insert(key.clone(), layer.kind);
        layers.insert(key, purposes);
    }

    if manifest.schema_version < 5
        && (!manifest.layer_aliases.is_empty() || !manifest.vias.is_empty())
    {
        return Err(PdkTechnologyError::InvalidField(
            "layer aliases and via definitions require manifest schema 5 or newer".to_owned(),
        ));
    }
    if manifest.layer_aliases.len() > MAX_PDK_LAYER_ALIASES {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.layer_aliases exceeds {MAX_PDK_LAYER_ALIASES} entries"
        )));
    }
    let mut layer_aliases = BTreeSet::new();
    for (index, alias) in manifest.layer_aliases.iter().enumerate() {
        validate_identifier(
            &format!("manifest.layer_aliases[{index}].alias"),
            &alias.alias,
        )?;
        validate_layer_purpose_reference(
            &format!("manifest.layer_aliases[{index}]"),
            &PdkLayerPurposeRef {
                layer: alias.layer.clone(),
                purpose: alias.purpose.clone(),
            },
            &layers,
        )?;
        let identity = alias.alias.to_ascii_lowercase();
        if layers.contains_key(&identity) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.layer_aliases[{index}] alias '{}' collides with a canonical layer name",
                alias.alias
            )));
        }
        if !layer_aliases.insert(identity) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.layer_aliases repeats case-insensitive alias '{}'",
                alias.alias
            )));
        }
    }

    if manifest.stream_map.is_empty() || manifest.stream_map.len() > MAX_PDK_STREAM_MAP_ENTRIES {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.stream_map must contain 1..={MAX_PDK_STREAM_MAP_ENTRIES} entries"
        )));
    }
    let mut logical_mappings = BTreeSet::new();
    let mut stream_mappings = BTreeSet::new();
    for (index, mapping) in manifest.stream_map.iter().enumerate() {
        let layer = mapping.layer.to_ascii_lowercase();
        let purpose = mapping.purpose.to_ascii_lowercase();
        let Some(purposes) = layers.get(&layer) else {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.stream_map[{index}].layer '{}' is not declared",
                mapping.layer
            )));
        };
        if !purposes.contains(&purpose) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.stream_map[{index}] references undeclared purpose '{}:{}'",
                mapping.layer, mapping.purpose
            )));
        }
        if !logical_mappings.insert((layer, purpose)) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.stream_map[{index}] repeats a logical layer/purpose"
            )));
        }
        if !stream_mappings.insert((mapping.stream_layer, mapping.stream_datatype)) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.stream_map[{index}] repeats stream pair {}/{}",
                mapping.stream_layer, mapping.stream_datatype
            )));
        }
    }
    for (layer, purposes) in &layers {
        for purpose in purposes {
            if !logical_mappings.contains(&(layer.clone(), purpose.clone())) {
                return Err(PdkTechnologyError::MissingMapping(format!(
                    "{layer}:{purpose}"
                )));
            }
        }
    }

    if manifest.connectivity.len() > MAX_PDK_CONNECTIVITY_EDGES {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.connectivity exceeds {MAX_PDK_CONNECTIVITY_EDGES} edges"
        )));
    }
    let mut edges = BTreeSet::new();
    for (index, edge) in manifest.connectivity.iter().enumerate() {
        let from = edge.from_layer.to_ascii_lowercase();
        let through = edge.through_layer.to_ascii_lowercase();
        let to = edge.to_layer.to_ascii_lowercase();
        for (field, value) in [
            ("from_layer", &from),
            ("through_layer", &through),
            ("to_layer", &to),
        ] {
            if !layers.contains_key(value) {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.connectivity[{index}].{field} '{value}' is not declared"
                )));
            }
        }
        if from == through || through == to || from == to {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.connectivity[{index}] must reference three distinct layers"
            )));
        }
        if !edges.insert((from, through, to)) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.connectivity[{index}] repeats an edge"
            )));
        }
    }

    if manifest.vias.len() > MAX_PDK_VIA_DEFINITIONS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.vias exceeds {MAX_PDK_VIA_DEFINITIONS} entries"
        )));
    }
    let mut via_ids = BTreeSet::new();
    let mut via_transitions = BTreeSet::new();
    for (index, via) in manifest.vias.iter().enumerate() {
        validate_identifier(&format!("manifest.vias[{index}].via_id"), &via.via_id)?;
        if !via_ids.insert(via.via_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.vias repeats case-insensitive via ID '{}'",
                via.via_id
            )));
        }
        let lower = via.lower_layer.to_ascii_lowercase();
        let cut = via.cut_layer.to_ascii_lowercase();
        let upper = via.upper_layer.to_ascii_lowercase();
        for (field, value) in [
            ("lower_layer", &lower),
            ("cut_layer", &cut),
            ("upper_layer", &upper),
        ] {
            if !layers.contains_key(value) {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.vias[{index}].{field} '{value}' is not declared"
                )));
            }
        }
        if lower == cut || cut == upper || lower == upper {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.vias[{index}] must reference three distinct layers"
            )));
        }
        if !matches!(
            layer_kinds.get(&cut),
            Some(PdkLayerKind::Via | PdkLayerKind::Cut)
        ) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.vias[{index}].cut_layer '{}' is not typed as a via or cut layer",
                via.cut_layer
            )));
        }
        if !edges.contains(&(lower.clone(), cut.clone(), upper.clone())) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.vias[{index}] has no matching connectivity edge '{} -> {} -> {}'",
                via.lower_layer, via.cut_layer, via.upper_layer
            )));
        }
        if !via_transitions.insert((lower, cut, upper)) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.vias[{index}] repeats a layer transition"
            )));
        }
        for (field, value) in [
            ("cut_width_meters", via.cut_width_meters),
            ("cut_height_meters", via.cut_height_meters),
            ("lower_enclosure_meters", via.lower_enclosure_meters),
            ("upper_enclosure_meters", via.upper_enclosure_meters),
        ] {
            if !value.is_finite() || !(1.0e-12..=1.0e-3).contains(&value) {
                return Err(PdkTechnologyError::InvalidField(format!(
                    "manifest.vias[{index}].{field} must be finite and in 1e-12..=1e-3"
                )));
            }
        }
        if via.maximum_rows == 0 || via.maximum_columns == 0 {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.vias[{index}] maximum rows and columns must be nonzero"
            )));
        }
        if via
            .maximum_rms_current_per_cut_amperes
            .is_some_and(|value| !value.is_finite() || value <= 0.0)
        {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.vias[{index}].maximum_rms_current_per_cut_amperes must be finite and positive"
            )));
        }
    }

    if manifest.artifacts.is_empty() || manifest.artifacts.len() > MAX_PDK_ARTIFACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.artifacts must contain 1..={MAX_PDK_ARTIFACTS} entries"
        )));
    }
    let mut artifact_paths = BTreeMap::new();
    for (index, artifact) in manifest.artifacts.iter().enumerate() {
        validate_package_path(&format!("manifest.artifacts[{index}].path"), &artifact.path)?;
        if artifact.size_bytes > u64::try_from(MAX_PDK_ARTIFACT_BYTES).unwrap_or(u64::MAX) {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.artifacts[{index}] exceeds {MAX_PDK_ARTIFACT_BYTES} bytes"
            )));
        }
        if artifact_paths
            .insert(
                artifact.path.to_ascii_lowercase(),
                (artifact.path.as_str(), artifact.kind),
            )
            .is_some()
        {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.artifacts repeats case-insensitive path '{}'",
                artifact.path
            )));
        }
    }

    if manifest.model_sources.len() > PdkModelProcess::ALL.len() {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.model_sources exceeds {MAX_PDK_MODEL_PROCESS_CONTRACTS} process contracts"
        )));
    }
    let model_artifact_count = artifact_paths
        .values()
        .filter(|(_, kind)| *kind == PdkTechnologyArtifactKind::Model)
        .count();
    if model_artifact_count == 0 && !manifest.model_sources.is_empty() {
        return Err(PdkTechnologyError::InvalidReference(
            "manifest.model_sources declares executable roots but the package has no model artifacts"
                .to_owned(),
        ));
    }
    if model_artifact_count > 0 && manifest.model_sources.is_empty() {
        return Err(PdkTechnologyError::InvalidReference(
            "model artifacts require typed manifest.model_sources process contracts".to_owned(),
        ));
    }
    let mut process_contracts = BTreeSet::new();
    let mut total_model_sources = 0usize;
    for (process_index, contract) in manifest.model_sources.iter().enumerate() {
        if !process_contracts.insert(contract.process) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.model_sources repeats {}",
                contract.process.keyword()
            )));
        }
        if contract.sources.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.model_sources[{process_index}].sources is empty"
            )));
        }
        total_model_sources = total_model_sources
            .checked_add(contract.sources.len())
            .ok_or_else(|| {
                PdkTechnologyError::LimitExceeded("model section source count overflow".to_owned())
            })?;
        if total_model_sources > MAX_PDK_MODEL_SECTION_SOURCES {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.model_sources contains more than {MAX_PDK_MODEL_SECTION_SOURCES} section sources"
            )));
        }
        let mut source_ids = BTreeSet::new();
        let mut source_selections = BTreeSet::new();
        let mut supplied_domains = BTreeSet::new();
        for (source_index, source) in contract.sources.iter().enumerate() {
            validate_identifier(
                &format!(
                    "manifest.model_sources[{process_index}].sources[{source_index}].source_id"
                ),
                &source.source_id,
            )?;
            if !source_ids.insert(source.source_id.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.model_sources[{process_index}] repeats source ID '{}'",
                    source.source_id
                )));
            }
            validate_package_path(
                &format!(
                    "manifest.model_sources[{process_index}].sources[{source_index}].artifact_path"
                ),
                &source.artifact_path,
            )?;
            let Some((_, kind)) = artifact_paths.get(&source.artifact_path.to_ascii_lowercase())
            else {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.model_sources[{process_index}] source '{}' references missing artifact '{}'",
                    source.source_id, source.artifact_path
                )));
            };
            if *kind != PdkTechnologyArtifactKind::Model {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.model_sources[{process_index}] source '{}' references an artifact not typed as model",
                    source.source_id
                )));
            }
            if let Some(section) = source.section.as_deref() {
                validate_model_section_name(
                    &format!(
                        "manifest.model_sources[{process_index}].sources[{source_index}].section"
                    ),
                    section,
                )?;
            }
            let selection = (
                source.artifact_path.to_ascii_lowercase(),
                source.section.as_deref().map(str::to_ascii_lowercase),
            );
            if !source_selections.insert(selection) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.model_sources[{process_index}] repeats artifact/section selection '{}{}'",
                    source.artifact_path,
                    source
                        .section
                        .as_deref()
                        .map(|section| format!(" [{section}]"))
                        .unwrap_or_default()
                )));
            }
            supplied_domains.insert(source.domain);
        }
        if contract.required_domains.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.model_sources[{process_index}].required_domains is empty"
            )));
        }
        let mut required_domains = BTreeSet::new();
        for domain in &contract.required_domains {
            if !required_domains.insert(*domain) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.model_sources[{process_index}] repeats required domain {domain:?}"
                )));
            }
            if !supplied_domains.contains(domain) {
                return Err(PdkTechnologyError::InvalidReference(format!(
                    "manifest.model_sources[{process_index}] requires {domain:?} but supplies no matching source"
                )));
            }
        }
    }
    if model_artifact_count > 0 && !process_contracts.contains(&PdkModelProcess::Tt) {
        return Err(PdkTechnologyError::InvalidReference(
            "model-source packages must explicitly supply the TT reference process".to_owned(),
        ));
    }

    if manifest.schema_version < 4 && !manifest.symbol_definitions.is_empty() {
        return Err(PdkTechnologyError::InvalidField(
            "signed symbol definitions require manifest schema 4 or newer".to_owned(),
        ));
    }
    if manifest.symbol_definitions.len() > MAX_PDK_SYMBOL_DEFINITIONS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.symbol_definitions exceeds {MAX_PDK_SYMBOL_DEFINITIONS} definitions"
        )));
    }
    let model_artifact_paths = artifact_paths
        .iter()
        .filter_map(|(path, (_, kind))| {
            (*kind == PdkTechnologyArtifactKind::Model).then_some(path.as_str())
        })
        .collect::<BTreeSet<_>>();
    let mut symbol_providers_by_artifact = BTreeMap::<String, BTreeSet<String>>::new();
    for contract in &manifest.model_sources {
        for source in &contract.sources {
            symbol_providers_by_artifact
                .entry(source.artifact_path.to_ascii_lowercase())
                .or_default()
                .insert(format!("signed-pdk:{}", source.source_id));
        }
    }
    let mut symbol_identities = BTreeSet::new();
    for (index, definition) in manifest.symbol_definitions.iter().enumerate() {
        if !definition
            .identity
            .library
            .eq_ignore_ascii_case(&manifest.package_id)
        {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] identity library must equal package ID '{}'",
                manifest.package_id
            )));
        }
        let identity = (
            definition.identity.library.to_ascii_lowercase(),
            definition.identity.cell.to_ascii_lowercase(),
        );
        if !symbol_identities.insert(identity) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.symbol_definitions repeats case-insensitive identity '{}/{}'",
                definition.identity.library, definition.identity.cell
            )));
        }
        let crate::state::SymbolSourceContract::Model { model, .. } = &definition.source else {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] must use an executable model source contract"
            )));
        };
        if definition.netlist.model.as_ref() != Some(model) {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] source and netlist model identities differ"
            )));
        }
        if model.implementation_view != crate::state::SymbolImplementationView::Spice {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] must bind a signed SPICE model source; use veriloga_sources for Verilog-A authority"
            )));
        }
        let source_path = model.source_path.as_deref().ok_or_else(|| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] has no model source path"
            ))
        })?;
        validate_package_path(
            &format!("manifest.symbol_definitions[{index}].source_path"),
            source_path,
        )?;
        if !model_artifact_paths.contains(source_path.to_ascii_lowercase().as_str()) {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.symbol_definitions[{index}] source '{}' is not a model artifact in this package",
                source_path
            )));
        }
        let providers = symbol_providers_by_artifact
            .get(&source_path.to_ascii_lowercase())
            .ok_or_else(|| {
                PdkTechnologyError::InvalidReference(format!(
                    "manifest.symbol_definitions[{index}] source '{}' is not reachable from a model-source contract",
                    source_path
                ))
            })?;
        if !providers
            .iter()
            .any(|provider| provider.eq_ignore_ascii_case(&model.library))
        {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.symbol_definitions[{index}] model library '{}' must name one signed provider for '{}': {}",
                model.library,
                source_path,
                providers.iter().cloned().collect::<Vec<_>>().join(", ")
            )));
        }
        if model.revision.as_deref() != Some(manifest.revision.as_str()) {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] model revision must equal signed package revision '{}'",
                manifest.revision
            )));
        }
        let mut executable = definition.clone();
        let virtual_source = signed_model_virtual_root("manifest-validation")
            .join(package_path_to_host_path(source_path))
            .to_string_lossy()
            .into_owned();
        let crate::state::SymbolSourceContract::Model { model, .. } = &mut executable.source else {
            unreachable!("model source was required above")
        };
        model.source_path = Some(virtual_source.clone());
        executable
            .netlist
            .model
            .as_mut()
            .expect("matching netlist model was required above")
            .source_path = Some(virtual_source);
        executable.validate().map_err(|error| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.symbol_definitions[{index}] is invalid: {error}"
            ))
        })?;
    }

    let veriloga_artifact_count = artifact_paths
        .values()
        .filter(|(_, kind)| *kind == PdkTechnologyArtifactKind::VerilogASource)
        .count();
    if manifest.schema_version < 2
        && (!manifest.veriloga_sources.is_empty() || veriloga_artifact_count > 0)
    {
        return Err(PdkTechnologyError::InvalidField(
            "signed Verilog-A artifacts require manifest schema 2 or newer".to_owned(),
        ));
    }
    if manifest.veriloga_sources.len() > MAX_PDK_VERILOGA_SOURCE_CONTRACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.veriloga_sources exceeds {MAX_PDK_VERILOGA_SOURCE_CONTRACTS} contracts"
        )));
    }
    if veriloga_artifact_count == 0 && !manifest.veriloga_sources.is_empty() {
        return Err(PdkTechnologyError::InvalidReference(
            "manifest.veriloga_sources declares executable roots but the package has no Verilog-A source artifacts"
                .to_owned(),
        ));
    }
    if veriloga_artifact_count > 0 && manifest.veriloga_sources.is_empty() {
        return Err(PdkTechnologyError::InvalidReference(
            "Verilog-A source artifacts require typed manifest.veriloga_sources contracts"
                .to_owned(),
        ));
    }
    let mut veriloga_source_ids = BTreeSet::new();
    let mut veriloga_aliases = BTreeSet::new();
    let mut veriloga_selections = BTreeSet::new();
    for (index, contract) in manifest.veriloga_sources.iter().enumerate() {
        validate_identifier(
            &format!("manifest.veriloga_sources[{index}].source_id"),
            &contract.source_id,
        )?;
        if !veriloga_source_ids.insert(contract.source_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.veriloga_sources repeats source ID '{}'",
                contract.source_id
            )));
        }
        validate_package_path(
            &format!("manifest.veriloga_sources[{index}].root_artifact_path"),
            &contract.root_artifact_path,
        )?;
        let Some((_, kind)) = artifact_paths.get(&contract.root_artifact_path.to_ascii_lowercase())
        else {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.veriloga_sources[{index}] references missing artifact '{}'",
                contract.root_artifact_path
            )));
        };
        if *kind != PdkTechnologyArtifactKind::VerilogASource {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.veriloga_sources[{index}] root '{}' is not typed as Verilog-A source",
                contract.root_artifact_path
            )));
        }
        validate_veriloga_identifier(
            &format!("manifest.veriloga_sources[{index}].module_name"),
            &contract.module_name,
        )?;
        validate_veriloga_identifier(
            &format!("manifest.veriloga_sources[{index}].netlist_alias"),
            &contract.netlist_alias,
        )?;
        if !veriloga_aliases.insert(contract.netlist_alias.to_ascii_uppercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.veriloga_sources repeats case-insensitive netlist alias '{}'",
                contract.netlist_alias
            )));
        }
        let selection = (
            contract.root_artifact_path.to_ascii_lowercase(),
            contract.module_name.to_ascii_lowercase(),
        );
        if !veriloga_selections.insert(selection) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.veriloga_sources repeats root/module selection '{}' / '{}'",
                contract.root_artifact_path, contract.module_name
            )));
        }
    }

    if manifest.recognition.len() > MAX_PDK_RECOGNITION_CONTRACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.recognition exceeds {MAX_PDK_RECOGNITION_CONTRACTS} contracts"
        )));
    }
    if manifest.extraction.len() > MAX_PDK_EXTRACTION_CONTRACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.extraction exceeds {MAX_PDK_EXTRACTION_CONTRACTS} contracts"
        )));
    }
    let mut recognition_ids = BTreeSet::new();
    let mut extraction_ids = BTreeSet::new();
    let mut vector_ids = BTreeSet::new();
    let mut referenced_special_artifacts = BTreeSet::new();
    let mut vector_count = 0usize;
    for (index, contract) in manifest.recognition.iter().enumerate() {
        validate_identifier(
            &format!("manifest.recognition[{index}].contract_id"),
            &contract.contract_id,
        )?;
        validate_identifier(
            &format!("manifest.recognition[{index}].device_class"),
            &contract.device_class,
        )?;
        if !recognition_ids.insert(contract.contract_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.recognition repeats contract '{}'",
                contract.contract_id
            )));
        }
        require_special_artifact(
            &format!("manifest.recognition[{index}].rule_artifact_path"),
            &contract.rule_artifact_path,
            PdkTechnologyArtifactKind::RecognitionMap,
            &artifact_paths,
            &mut referenced_special_artifacts,
        )?;
        if contract.terminals.is_empty() || contract.terminals.len() > MAX_PDK_RECOGNITION_TERMINALS
        {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.recognition[{index}].terminals must contain 1..={MAX_PDK_RECOGNITION_TERMINALS} entries"
            )));
        }
        let mut terminals = BTreeSet::new();
        for (terminal_index, terminal) in contract.terminals.iter().enumerate() {
            validate_identifier(
                &format!("manifest.recognition[{index}].terminals[{terminal_index}].terminal_name"),
                &terminal.terminal_name,
            )?;
            if !terminals.insert(terminal.terminal_name.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.recognition[{index}] repeats terminal '{}'",
                    terminal.terminal_name
                )));
            }
            validate_layer_purpose_reference(
                &format!("manifest.recognition[{index}].terminals[{terminal_index}]"),
                &PdkLayerPurposeRef {
                    layer: terminal.layer.clone(),
                    purpose: terminal.purpose.clone(),
                },
                &layers,
            )?;
        }
        if contract.qualification_vectors.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.recognition[{index}].qualification_vectors is empty"
            )));
        }
        vector_count = vector_count
            .checked_add(contract.qualification_vectors.len())
            .ok_or_else(|| {
                PdkTechnologyError::LimitExceeded("qualification vector count overflow".to_owned())
            })?;
        for (vector_index, vector) in contract.qualification_vectors.iter().enumerate() {
            validate_identifier(
                &format!(
                    "manifest.recognition[{index}].qualification_vectors[{vector_index}].vector_id"
                ),
                &vector.vector_id,
            )?;
            if !vector_ids.insert(vector.vector_id.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest qualification vectors repeat '{}'",
                    vector.vector_id
                )));
            }
            if vector.expected_instance_count > 1_000_000 {
                return Err(PdkTechnologyError::LimitExceeded(format!(
                    "manifest.recognition[{index}].qualification_vectors[{vector_index}].expected_instance_count exceeds 1000000"
                )));
            }
            require_special_artifact(
                &format!(
                    "manifest.recognition[{index}].qualification_vectors[{vector_index}].layout_artifact_path"
                ),
                &vector.layout_artifact_path,
                PdkTechnologyArtifactKind::QualificationVector,
                &artifact_paths,
                &mut referenced_special_artifacts,
            )?;
        }
    }
    for (index, contract) in manifest.extraction.iter().enumerate() {
        validate_identifier(
            &format!("manifest.extraction[{index}].contract_id"),
            &contract.contract_id,
        )?;
        if !extraction_ids.insert(contract.contract_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.extraction repeats contract '{}'",
                contract.contract_id
            )));
        }
        require_special_artifact(
            &format!("manifest.extraction[{index}].rule_artifact_path"),
            &contract.rule_artifact_path,
            PdkTechnologyArtifactKind::ExtractionRule,
            &artifact_paths,
            &mut referenced_special_artifacts,
        )?;
        if contract.quantities.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.extraction[{index}].quantities is empty"
            )));
        }
        let quantities = contract.quantities.iter().copied().collect::<BTreeSet<_>>();
        if quantities.len() != contract.quantities.len() {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.extraction[{index}].quantities contains duplicates"
            )));
        }
        if contract.layer_purposes.is_empty() || contract.layer_purposes.len() > MAX_PDK_LAYERS {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.extraction[{index}].layer_purposes must contain 1..={MAX_PDK_LAYERS} entries"
            )));
        }
        let mut layer_purposes = BTreeSet::new();
        for (reference_index, reference) in contract.layer_purposes.iter().enumerate() {
            validate_layer_purpose_reference(
                &format!("manifest.extraction[{index}].layer_purposes[{reference_index}]"),
                reference,
                &layers,
            )?;
            if !layer_purposes.insert((
                reference.layer.to_ascii_lowercase(),
                reference.purpose.to_ascii_lowercase(),
            )) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest.extraction[{index}] repeats layer/purpose '{}:{}'",
                    reference.layer, reference.purpose
                )));
            }
        }
        if contract.qualification_vectors.is_empty() {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.extraction[{index}].qualification_vectors is empty"
            )));
        }
        vector_count = vector_count
            .checked_add(contract.qualification_vectors.len())
            .ok_or_else(|| {
                PdkTechnologyError::LimitExceeded("qualification vector count overflow".to_owned())
            })?;
        for (vector_index, vector) in contract.qualification_vectors.iter().enumerate() {
            validate_identifier(
                &format!(
                    "manifest.extraction[{index}].qualification_vectors[{vector_index}].vector_id"
                ),
                &vector.vector_id,
            )?;
            if !vector_ids.insert(vector.vector_id.to_ascii_lowercase()) {
                return Err(PdkTechnologyError::Duplicate(format!(
                    "manifest qualification vectors repeat '{}'",
                    vector.vector_id
                )));
            }
            require_special_artifact(
                &format!(
                    "manifest.extraction[{index}].qualification_vectors[{vector_index}].layout_artifact_path"
                ),
                &vector.layout_artifact_path,
                PdkTechnologyArtifactKind::QualificationVector,
                &artifact_paths,
                &mut referenced_special_artifacts,
            )?;
            require_special_artifact(
                &format!(
                    "manifest.extraction[{index}].qualification_vectors[{vector_index}].reference_artifact_path"
                ),
                &vector.reference_artifact_path,
                PdkTechnologyArtifactKind::QualificationReference,
                &artifact_paths,
                &mut referenced_special_artifacts,
            )?;
        }
    }
    if vector_count > MAX_PDK_QUALIFICATION_VECTORS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest recognition and extraction contracts exceed {MAX_PDK_QUALIFICATION_VECTORS} qualification vectors"
        )));
    }
    for (path, (_, kind)) in &artifact_paths {
        if matches!(
            kind,
            PdkTechnologyArtifactKind::RecognitionMap
                | PdkTechnologyArtifactKind::ExtractionRule
                | PdkTechnologyArtifactKind::QualificationVector
                | PdkTechnologyArtifactKind::QualificationReference
        ) && !referenced_special_artifacts.contains(path)
        {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "specialized artifact '{path}' is not owned by a recognition or extraction contract"
            )));
        }
    }

    let mut callback_ids = BTreeSet::new();
    if manifest.callbacks.len() > MAX_PDK_CALLBACK_CONTRACTS {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "manifest.callbacks exceeds {MAX_PDK_CALLBACK_CONTRACTS} contracts"
        )));
    }
    if !manifest.callbacks.is_empty() && manifest.schema_version < 3 {
        return Err(PdkTechnologyError::InvalidField(
            "executable callbacks require manifest schema 3 or newer".to_owned(),
        ));
    }
    for (index, callback) in manifest.callbacks.iter().enumerate() {
        validate_identifier(
            &format!("manifest.callbacks[{index}].callback_id"),
            &callback.callback_id,
        )?;
        validate_package_path(
            &format!("manifest.callbacks[{index}].artifact_path"),
            &callback.artifact_path,
        )?;
        if callback.abi_version != PDK_CALLBACK_ABI_VERSION {
            return Err(PdkTechnologyError::InvalidField(format!(
                "manifest.callbacks[{index}].abi_version is {}; supported ABI is {PDK_CALLBACK_ABI_VERSION}",
                callback.abi_version
            )));
        }
        validate_identifier(
            &format!("manifest.callbacks[{index}].entrypoint"),
            &callback.entrypoint,
        )?;
        if !callback_ids.insert(callback.callback_id.to_ascii_lowercase()) {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.callbacks repeats callback '{}'",
                callback.callback_id
            )));
        }
        let Some((_, kind)) = artifact_paths.get(&callback.artifact_path.to_ascii_lowercase())
        else {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.callbacks[{index}] references missing artifact '{}'",
                callback.artifact_path
            )));
        };
        if *kind != PdkTechnologyArtifactKind::Callback {
            return Err(PdkTechnologyError::InvalidReference(format!(
                "manifest.callbacks[{index}] artifact is not typed as callback"
            )));
        }
        let artifact = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.path.eq_ignore_ascii_case(&callback.artifact_path))
            .expect("callback artifact path was resolved above");
        if usize::try_from(artifact.size_bytes)
            .ok()
            .is_none_or(|size| size > MAX_PDK_CALLBACK_ARTIFACT_BYTES)
        {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "manifest.callbacks[{index}] artifact '{}' exceeds {MAX_PDK_CALLBACK_ARTIFACT_BYTES} bytes",
                callback.artifact_path
            )));
        }
        let capabilities = callback
            .capabilities
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if capabilities.len() != callback.capabilities.len() {
            return Err(PdkTechnologyError::Duplicate(format!(
                "manifest.callbacks[{index}].capabilities contains duplicates"
            )));
        }
        if capabilities.contains(&PdkCallbackCapability::Network) {
            return Err(PdkTechnologyError::ForbiddenCapability(format!(
                "manifest.callbacks[{index}] requests network access"
            )));
        }
    }
    Ok(())
}

fn validate_layer_purpose_reference(
    path: &str,
    reference: &PdkLayerPurposeRef,
    layers: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), PdkTechnologyError> {
    validate_identifier(&format!("{path}.layer"), &reference.layer)?;
    validate_identifier(&format!("{path}.purpose"), &reference.purpose)?;
    let Some(purposes) = layers.get(&reference.layer.to_ascii_lowercase()) else {
        return Err(PdkTechnologyError::InvalidReference(format!(
            "{path}.layer '{}' is not declared",
            reference.layer
        )));
    };
    if !purposes.contains(&reference.purpose.to_ascii_lowercase()) {
        return Err(PdkTechnologyError::InvalidReference(format!(
            "{path} references undeclared purpose '{}:{}'",
            reference.layer, reference.purpose
        )));
    }
    Ok(())
}

fn require_special_artifact(
    path: &str,
    artifact_path: &str,
    required_kind: PdkTechnologyArtifactKind,
    artifacts: &BTreeMap<String, (&str, PdkTechnologyArtifactKind)>,
    referenced: &mut BTreeSet<String>,
) -> Result<(), PdkTechnologyError> {
    validate_package_path(path, artifact_path)?;
    let identity = artifact_path.to_ascii_lowercase();
    let Some((_, actual_kind)) = artifacts.get(&identity) else {
        return Err(PdkTechnologyError::InvalidReference(format!(
            "{path} references missing artifact '{artifact_path}'"
        )));
    };
    if *actual_kind != required_kind {
        return Err(PdkTechnologyError::InvalidReference(format!(
            "{path} artifact '{artifact_path}' has type {actual_kind:?}, expected {required_kind:?}"
        )));
    }
    if !referenced.insert(identity) {
        return Err(PdkTechnologyError::Duplicate(format!(
            "specialized artifact '{artifact_path}' is referenced more than once"
        )));
    }
    Ok(())
}

fn archive_identity(
    archive: &SignedPdkTechnologyArchive,
) -> Result<PdkTechnologyBinding, PdkTechnologyError> {
    let bytes = decode_bounded(
        "manifest_base64",
        &archive.manifest_base64,
        MAX_PDK_MANIFEST_BYTES,
    )?;
    let manifest: PdkTechnologyManifest = serde_json::from_slice(&bytes)
        .map_err(|error| PdkTechnologyError::ManifestParse(error.to_string()))?;
    Ok(PdkTechnologyBinding {
        package_id: manifest.package_id,
        revision: manifest.revision,
        manifest_digest: content_digest(&bytes),
    })
}

fn package_order(
    left: &ValidatedPdkTechnologyPackage,
    right: &ValidatedPdkTechnologyPackage,
) -> std::cmp::Ordering {
    (
        left.manifest.package_id.to_ascii_lowercase(),
        left.manifest.revision.as_str(),
    )
        .cmp(&(
            right.manifest.package_id.to_ascii_lowercase(),
            right.manifest.revision.as_str(),
        ))
}

fn decode_bounded(field: &str, value: &str, maximum: usize) -> Result<Vec<u8>, PdkTechnologyError> {
    let approximate = value.len().saturating_mul(3) / 4;
    if approximate > maximum.saturating_add(3) {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "{field} exceeds {maximum} decoded bytes"
        )));
    }
    let bytes = STANDARD
        .decode(value)
        .map_err(|error| PdkTechnologyError::InvalidBase64 {
            field: field.to_owned(),
            detail: error.to_string(),
        })?;
    if bytes.len() > maximum {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "{field} contains {} decoded bytes; maximum is {maximum}",
            bytes.len()
        )));
    }
    Ok(bytes)
}

fn content_digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn validate_identifier(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 128)?;
    if !value.bytes().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'-' | b'_' | b'.' | b':' | b'@')
    }) {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} must use lowercase ASCII identifier characters"
        )));
    }
    Ok(())
}

fn validate_version(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 64)?;
    semver::Version::parse(value).map(|_| ()).map_err(|error| {
        PdkTechnologyError::InvalidField(format!("{path} must be a semantic version: {error}"))
    })
}

fn validate_runtime_compatibility(
    manifest: &PdkTechnologyManifest,
) -> Result<(), PdkTechnologyError> {
    let current_target = current_execution_target();
    if !manifest.compatibility.targets.contains(&current_target) {
        return Err(PdkTechnologyError::IncompatibleRuntime(format!(
            "{} {} does not permit the current {current_target:?} execution target",
            manifest.package_id, manifest.revision
        )));
    }
    let current = semver::Version::parse(env!("CARGO_PKG_VERSION")).map_err(|error| {
        PdkTechnologyError::IncompatibleRuntime(format!("RSpice build version is invalid: {error}"))
    })?;
    for (component, minimum) in [
        ("engine", &manifest.compatibility.minimum_engine_version),
        ("viewer", &manifest.compatibility.minimum_viewer_version),
    ] {
        let minimum = semver::Version::parse(minimum).map_err(|error| {
            PdkTechnologyError::InvalidField(format!(
                "manifest.compatibility.minimum_{component}_version is invalid: {error}"
            ))
        })?;
        if current < minimum {
            return Err(PdkTechnologyError::IncompatibleRuntime(format!(
                "{} {} requires {component} {minimum} or newer; this build is {current}",
                manifest.package_id, manifest.revision
            )));
        }
    }
    Ok(())
}

fn validate_text(path: &str, value: &str, maximum: usize) -> Result<(), PdkTechnologyError> {
    if value.trim() != value || value.is_empty() || value.len() > maximum {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} must contain 1..={maximum} bytes without surrounding whitespace"
        )));
    }
    if value.chars().any(char::is_control) {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} contains a control character"
        )));
    }
    Ok(())
}

fn validate_package_path(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 1_024)?;
    if value.starts_with('/')
        || value.starts_with('\\')
        || value.contains('\\')
        || value.contains(':')
        || value
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} must be a normalized relative forward-slash path"
        )));
    }
    Ok(())
}

fn validate_model_section_name(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 256)?;
    if value.chars().any(|character| {
        character.is_whitespace() || character == '"' || character == '\'' || character.is_control()
    }) {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} contains whitespace, a quote, or a control character"
        )));
    }
    Ok(())
}

fn validate_veriloga_identifier(path: &str, value: &str) -> Result<(), PdkTechnologyError> {
    validate_text(path, value, 128)?;
    let mut characters = value.chars();
    if !characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        || !characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(PdkTechnologyError::InvalidField(format!(
            "{path} must be a portable Verilog-A/SPICE identifier"
        )));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PdkTechnologyError {
    #[error("technology archive is {actual} bytes; maximum is {maximum}")]
    ArchiveTooLarge { actual: usize, maximum: usize },
    #[error("technology archive JSON is invalid: {0}")]
    ArchiveParse(String),
    #[error("technology manifest JSON is invalid: {0}")]
    ManifestParse(String),
    #[error("unsupported {object} schema {actual}; newest supported schema is {supported}")]
    UnsupportedSchema {
        object: &'static str,
        actual: u32,
        supported: u32,
    },
    #[error("invalid field: {0}")]
    InvalidField(String),
    #[error("invalid reference: {0}")]
    InvalidReference(String),
    #[error("duplicate identity: {0}")]
    Duplicate(String),
    #[error("missing stream mapping for {0}")]
    MissingMapping(String),
    #[error("limit exceeded: {0}")]
    LimitExceeded(String),
    #[error("{field} is not valid base64: {detail}")]
    InvalidBase64 { field: String, detail: String },
    #[error("signature contains {actual} bytes; exactly 64 are required")]
    InvalidSignatureLength { actual: usize },
    #[error("publisher '{publisher_id}' key '{key_id}' is not trusted")]
    UntrustedPublisher {
        publisher_id: String,
        key_id: String,
    },
    #[error("publisher '{publisher_id}' key '{key_id}' is revoked")]
    RevokedPublisherKey {
        publisher_id: String,
        key_id: String,
    },
    #[error("publisher signature verification failed for '{publisher_id}' key '{key_id}'")]
    InvalidSignature {
        publisher_id: String,
        key_id: String,
    },
    #[error("invalid publisher trust store: {0}")]
    InvalidTrustStore(String),
    #[error("immutable publisher trust-key conflict: {0}")]
    ImmutableTrustKey(String),
    #[error("publisher trust audit chain is corrupted: {0}")]
    TrustAuditCorrupted(String),
    #[error("manifest declares missing artifact '{0}'")]
    MissingArtifact(String),
    #[error("archive contains undeclared artifact '{0}'")]
    UndeclaredArtifact(String),
    #[error("artifact '{path}' declares {declared} bytes but contains {actual}")]
    ArtifactSizeMismatch {
        path: String,
        declared: u64,
        actual: usize,
    },
    #[error("artifact '{path}' digest mismatch: declared {declared}, actual {actual}")]
    ArtifactDigestMismatch {
        path: String,
        declared: ContentDigest,
        actual: ContentDigest,
    },
    #[error("signed PDK model-source materialization failed: {0}")]
    ModelMaterialization(String),
    #[error("forbidden callback capability: {0}")]
    ForbiddenCapability(String),
    #[error("signed PDK callback validation failed: {0}")]
    CallbackValidation(String),
    #[error("immutable technology revision conflict: {0}")]
    ImmutableRevision(String),
    #[error("technology package is not runtime-validated: {0}")]
    NotRuntimeValidated(String),
    #[error("invalid technology transition: {0}")]
    InvalidTransition(String),
    #[error("incompatible technology runtime: {0}")]
    IncompatibleRuntime(String),
    #[error("technology audit chain is corrupted: {0}")]
    AuditCorrupted(String),
    #[error("technology administration serialization failed: {0}")]
    Serialization(String),
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use ed25519_dalek::{Signer as _, SigningKey};

    pub(crate) fn fixture_archive() -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority)
    {
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let model_bytes = br#".lib TT
.model nmos_demo nmos level=1 vto=0.55
.endl TT
.lib SS
.model nmos_demo nmos level=1 vto=0.60
.endl SS
.lib FF
.model nmos_demo nmos level=1 vto=0.50
.endl FF
.lib SF
.model nmos_demo nmos level=1 vto=0.58
.endl SF
.lib FS
.model nmos_demo nmos level=1 vto=0.52
.endl FS
"#
        .to_vec();
        let callback_bytes = wat::parse_str(
            r#"(module
                (memory (export "memory") 1 2)
                (func (export "derive") (result i32)
                    i32.const 0))"#,
        )
        .unwrap();
        let manifest = PdkTechnologyManifest {
            schema_version: PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION,
            package_id: "demo180".to_owned(),
            technology_name: "Demo 180 nm".to_owned(),
            revision: "2.3.1".to_owned(),
            publisher_id: "rspice-foundry-demo".to_owned(),
            signing_key_id: "ceremony-01".to_owned(),
            license_spdx: "LicenseRef-RSpice-Demo-PDK".to_owned(),
            process_node_nm: 180,
            database_unit_meters: 1.0e-9,
            stack_name: "1P2M".to_owned(),
            compatibility: PdkTechnologyCompatibility {
                minimum_engine_version: "0.1.0".to_owned(),
                minimum_viewer_version: "0.1.0".to_owned(),
                targets: vec![
                    PdkExecutionTarget::Desktop,
                    PdkExecutionTarget::WebAssembly,
                    PdkExecutionTarget::Mobile,
                ],
            },
            model_sources: PdkModelProcess::ALL
                .into_iter()
                .map(|process| PdkModelProcessContract {
                    process,
                    sources: vec![PdkModelSectionSource {
                        source_id: format!(
                            "demo-models-{}",
                            process.keyword().to_ascii_lowercase()
                        ),
                        domain: PdkModelDomain::Composite,
                        artifact_path: "models/demo.lib".to_owned(),
                        section: Some(process.keyword().to_owned()),
                    }],
                    required_domains: vec![PdkModelDomain::Composite],
                })
                .collect(),
            veriloga_sources: Vec::new(),
            symbol_definitions: Vec::new(),
            layers: vec![
                PdkTechnologyLayer {
                    name: "active".to_owned(),
                    order: 0,
                    kind: PdkLayerKind::Active,
                    purposes: vec!["drawing".to_owned()],
                    role: "diffusion".to_owned(),
                    display_rgba: [64, 160, 96, 255],
                },
                PdkTechnologyLayer {
                    name: "cont".to_owned(),
                    order: 1,
                    kind: PdkLayerKind::Cut,
                    purposes: vec!["drawing".to_owned()],
                    role: "active to metal1".to_owned(),
                    display_rgba: [192, 192, 192, 255],
                },
                PdkTechnologyLayer {
                    name: "metal1".to_owned(),
                    order: 2,
                    kind: PdkLayerKind::Metal,
                    purposes: vec!["drawing".to_owned(), "pin".to_owned()],
                    role: "routing".to_owned(),
                    display_rgba: [64, 144, 208, 255],
                },
            ],
            layer_aliases: Vec::new(),
            stream_map: vec![
                PdkStreamMapEntry {
                    layer: "active".to_owned(),
                    purpose: "drawing".to_owned(),
                    stream_layer: 1,
                    stream_datatype: 0,
                },
                PdkStreamMapEntry {
                    layer: "cont".to_owned(),
                    purpose: "drawing".to_owned(),
                    stream_layer: 2,
                    stream_datatype: 0,
                },
                PdkStreamMapEntry {
                    layer: "metal1".to_owned(),
                    purpose: "drawing".to_owned(),
                    stream_layer: 3,
                    stream_datatype: 0,
                },
                PdkStreamMapEntry {
                    layer: "metal1".to_owned(),
                    purpose: "pin".to_owned(),
                    stream_layer: 3,
                    stream_datatype: 1,
                },
            ],
            connectivity: vec![PdkConnectivityEdge {
                from_layer: "active".to_owned(),
                through_layer: "cont".to_owned(),
                to_layer: "metal1".to_owned(),
            }],
            vias: Vec::new(),
            recognition: Vec::new(),
            extraction: Vec::new(),
            callbacks: vec![PdkCallbackContract {
                callback_id: "derive-device".to_owned(),
                artifact_path: "callbacks/derive.wasm".to_owned(),
                abi_version: PDK_CALLBACK_ABI_VERSION,
                entrypoint: "derive".to_owned(),
                capabilities: vec![
                    PdkCallbackCapability::ReadPackage,
                    PdkCallbackCapability::WriteDerivedMetadata,
                ],
            }],
            artifacts: vec![
                PdkTechnologyArtifact {
                    path: "models/demo.lib".to_owned(),
                    kind: PdkTechnologyArtifactKind::Model,
                    size_bytes: u64::try_from(model_bytes.len()).unwrap(),
                    sha256: content_digest(&model_bytes),
                },
                PdkTechnologyArtifact {
                    path: "callbacks/derive.wasm".to_owned(),
                    kind: PdkTechnologyArtifactKind::Callback,
                    size_bytes: u64::try_from(callback_bytes.len()).unwrap(),
                    sha256: content_digest(&callback_bytes),
                },
            ],
        };
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        let archive = SignedPdkTechnologyArchive {
            schema_version: PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION,
            manifest_base64: STANDARD.encode(&manifest_bytes),
            signature_base64: STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes()),
            files: vec![
                PdkTechnologyArchiveFile {
                    path: "models/demo.lib".to_owned(),
                    content_base64: STANDARD.encode(model_bytes),
                },
                PdkTechnologyArchiveFile {
                    path: "callbacks/derive.wasm".to_owned(),
                    content_base64: STANDARD.encode(callback_bytes),
                },
            ],
        };
        (
            serde_json::to_vec(&archive).unwrap(),
            PdkPublisherTrustStore {
                keys: vec![TrustedPdkPublisherKey {
                    publisher_id: "rspice-foundry-demo".to_owned(),
                    key_id: "ceremony-01".to_owned(),
                    verifying_key: signing_key.verifying_key().to_bytes(),
                    revoked: false,
                }],
                audit: Vec::new(),
            },
            PdkAdministrativeAuthority {
                actor_id: "cad-admin@example.com".to_owned(),
                authority_id: "role:pdk-administrator".to_owned(),
            },
        )
    }

    pub(crate) fn fixture_archive_with_veriloga()
    -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
        let (bytes, trust, authority) = fixture_archive();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let root = br#"`include "parts/resistance.vams"
module pdk_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = `PDK_RESISTANCE;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#
        .to_vec();
        let dependency = b"`define PDK_RESISTANCE 250.0\n".to_vec();
        for (path, content) in [
            ("veriloga/pdk_resistor.va", root.as_slice()),
            ("veriloga/parts/resistance.vams", dependency.as_slice()),
        ] {
            manifest.artifacts.push(PdkTechnologyArtifact {
                path: path.to_owned(),
                kind: PdkTechnologyArtifactKind::VerilogASource,
                size_bytes: u64::try_from(content.len()).unwrap(),
                sha256: content_digest(content),
            });
            archive.files.push(PdkTechnologyArchiveFile {
                path: path.to_owned(),
                content_base64: STANDARD.encode(content),
            });
        }
        manifest.veriloga_sources = vec![PdkVerilogASourceContract {
            source_id: "pdk-resistor-runtime".to_owned(),
            root_artifact_path: "veriloga/pdk_resistor.va".to_owned(),
            module_name: "pdk_resistor".to_owned(),
            netlist_alias: "pdk_resistor_model".to_owned(),
        }];
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        (serde_json::to_vec(&archive).unwrap(), trust, authority)
    }

    #[test]
    fn schema_five_layer_aliases_and_via_definitions_are_typed_and_cross_validated() {
        let (bytes, _, _) = fixture_archive();
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        manifest.layer_aliases.push(PdkLayerAlias {
            alias: "m1_drawing".to_owned(),
            layer: "metal1".to_owned(),
            purpose: "drawing".to_owned(),
        });
        manifest.vias.push(PdkViaDefinition {
            via_id: "cont_active_m1".to_owned(),
            lower_layer: "active".to_owned(),
            cut_layer: "cont".to_owned(),
            upper_layer: "metal1".to_owned(),
            cut_width_meters: 1.6e-7,
            cut_height_meters: 1.6e-7,
            lower_enclosure_meters: 5.0e-8,
            upper_enclosure_meters: 5.0e-8,
            maximum_rows: 8,
            maximum_columns: 8,
            maximum_rms_current_per_cut_amperes: Some(8.0e-3),
        });
        validate_manifest(&manifest).expect("schema-five physical contracts validate");

        let mut invalid_alias = manifest.clone();
        invalid_alias.layer_aliases[0].alias = "metal1".to_owned();
        assert!(matches!(
            validate_manifest(&invalid_alias),
            Err(PdkTechnologyError::Duplicate(_))
        ));

        let mut invalid_via = manifest;
        invalid_via.vias[0].cut_layer = "active".to_owned();
        assert!(matches!(
            validate_manifest(&invalid_via),
            Err(PdkTechnologyError::InvalidField(_)) | Err(PdkTechnologyError::InvalidReference(_))
        ));
    }

    pub(crate) fn fixture_signed_symbol(
        manifest: &PdkTechnologyManifest,
    ) -> crate::state::ModelBoundSymbolDefinition {
        let mut model =
            crate::state::SymbolModelReference::new("signed-pdk:demo-models-tt", "nmos_demo")
                .with_source_path("models/demo.lib");
        model.section = Some("TT".to_owned());
        model.revision = Some(manifest.revision.clone());
        let pins = [
            ("D", crate::state::SymbolPinSide::Right),
            ("G", crate::state::SymbolPinSide::Left),
            ("S", crate::state::SymbolPinSide::Right),
            ("B", crate::state::SymbolPinSide::Bottom),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (name, side))| {
            crate::state::SymbolPinDefinition::new(
                name,
                crate::state::SymbolElectricalType::Analog,
                crate::state::PortDirection::InOut,
                side,
                index + 1,
            )
        })
        .collect::<Vec<_>>();
        let ports = pins
            .iter()
            .map(crate::state::SymbolPinDefinition::port_spec)
            .collect();
        crate::state::ModelBoundSymbolDefinition::new(
            crate::state::SymbolIdentity::new(
                &manifest.package_id,
                "nmos_demo",
                1,
                "signed-pdk:demo180/nmos_demo",
            ),
            crate::state::SymbolSourceContract::model(model.clone(), ports),
            pins,
            crate::state::SymbolGraphicTemplate::RectangularIc,
            crate::state::SymbolParameterForm {
                revision: 1,
                sections: Vec::new(),
            },
            crate::state::SymbolNetlistBinding {
                device_prefix: "M".to_owned(),
                model: Some(model),
                template: "M{name} {nodes} {model} {params}".to_owned(),
                parameter_order: Vec::new(),
            },
            crate::state::GeneratedSymbolViews::default(),
        )
    }

    pub(crate) fn fixture_archive_with_symbols()
    -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
        let (bytes, trust, authority) = fixture_archive();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        manifest.symbol_definitions = vec![fixture_signed_symbol(&manifest)];
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        (serde_json::to_vec(&archive).unwrap(), trust, authority)
    }

    #[test]
    fn signed_symbols_materialize_exact_archive_bound_sources() {
        let (bytes, trust, _) = fixture_archive_with_symbols();
        let (_, package) = validate_archive_bytes(&bytes, &trust).expect("signed symbol validates");
        assert_eq!(package.manifest().symbol_definitions.len(), 1);
        assert_eq!(
            package.manifest().symbol_definitions[0]
                .netlist
                .model
                .as_ref()
                .and_then(|model| model.source_path.as_deref()),
            Some("models/demo.lib")
        );

        let definition = package
            .symbol_definitions()
            .first()
            .expect("runtime symbol");
        definition
            .validate()
            .expect("materialized symbol validates");
        let source = definition
            .netlist
            .model
            .as_ref()
            .and_then(|model| model.source_path.as_deref())
            .expect("materialized source");
        assert_eq!(
            PathBuf::from(source),
            signed_model_virtual_root(&package.archive_digest().to_string())
                .join(package_path_to_host_path("models/demo.lib"))
        );
    }

    #[test]
    fn signed_symbols_reject_provider_or_artifact_authority_mismatch() {
        let (bytes, trust, _) = fixture_archive_with_symbols();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let definition = manifest.symbol_definitions.first_mut().unwrap();
        let crate::state::SymbolSourceContract::Model { model, .. } = &mut definition.source else {
            unreachable!()
        };
        model.library = "signed-pdk:unrelated-provider".to_owned();
        definition.netlist.model = Some(model.clone());
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());

        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidReference(_))
        ));
    }

    #[test]
    fn signed_model_sources_materialize_exact_process_sections_and_archive_identity() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(
                &bytes,
                &trust,
                &authority,
                "Install executable model-source fixture",
            )
            .expect("signed package installs");
        let package = registry.validated_packages()[0].clone();
        let sealed = registry
            .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
            .expect("exact project-bound model sources seal");
        assert_eq!(sealed.sources.len(), 1);
        assert_eq!(sealed.process_bindings.len(), 5);
        assert_eq!(sealed.binding, package.binding());
        assert_eq!(sealed.archive_digest, package.archive_digest());

        let combined = crate::state::model_library::ModelLibraryManager::new()
            .seal_execution_sources()
            .expect("empty ordinary model catalog seals")
            .with_pdk_model_sources(sealed)
            .expect("signed PDK closure merges");
        let tt = combined
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("TT materializes");
        assert_eq!(tt.len(), 1);
        assert!(tt[0].contains("vto=0.55"));
        assert!(!tt[0].contains("vto=0.60"));
        assert!(!tt[0].to_ascii_lowercase().contains(".lib "));

        let corner_bindings = combined
            .corner_model_bindings(&[
                crate::services::simulation_runner::CornerProcess::SS,
                crate::services::simulation_runner::CornerProcess::FF,
            ])
            .expect("explicit signed corner sections materialize");
        assert_eq!(corner_bindings.len(), 2);
        assert!(
            corner_bindings[0]
                .materialized_model_cards
                .contains("vto=0.60")
        );
        assert!(
            corner_bindings[1]
                .materialized_model_cards
                .contains("vto=0.50")
        );
        let (identity, digest) = combined
            .pdk_model_identity()
            .expect("prepared model snapshot binds signed package");
        assert!(identity.contains("demo180@2.3.1"));
        assert_eq!(digest, package.archive_digest());
    }

    #[test]
    fn signed_veriloga_closure_compiles_and_retains_exact_archive_authority() {
        let (bytes, trust, authority) = fixture_archive_with_veriloga();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(
                &bytes,
                &trust,
                &authority,
                "Install signed Verilog-A fixture",
            )
            .expect("signed Verilog-A package installs");
        let package = registry.validated_packages()[0].clone();
        let sealed = registry
            .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
            .expect("exact signed Verilog-A closure seals");
        assert_eq!(sealed.veriloga_artifacts.len(), 2);
        assert_eq!(sealed.veriloga_bindings.len(), 1);
        let combined = crate::state::model_library::ModelLibraryManager::new()
            .seal_execution_sources()
            .unwrap()
            .with_pdk_model_sources(sealed)
            .unwrap();
        let (binding, archive_digest, artifacts, bindings) = combined
            .pdk_veriloga_authority()
            .expect("signed runtime authority retained");
        assert_eq!(binding, &package.binding());
        assert_eq!(archive_digest, package.archive_digest());
        let runtime = crate::simulation::veriloga::compile_signed_pdk_source_runtime(
            binding,
            archive_digest,
            artifacts,
            &bindings[0],
        )
        .expect("retained signed source recompiles");
        assert!(runtime.source_key().starts_with("__rspice_pdk__/"));
        assert_eq!(runtime.source_digest(), package.archive_digest());
        assert_eq!(runtime.module_name(), "pdk_resistor");
        assert_eq!(runtime.netlist_alias(), "pdk_resistor_model");
        assert_eq!(runtime.terminal_names().unwrap(), ["p", "n"]);
        let encoded = serde_json::to_vec(&runtime).unwrap();
        let restored: crate::simulation::veriloga::PreparedVerilogARuntime =
            serde_json::from_slice(&encoded).unwrap();
        restored.validate().expect("worker payload revalidates");
        assert_eq!(restored, runtime);
    }

    #[test]
    fn signed_veriloga_rejects_tampered_or_untyped_dependency_bytes() {
        let (bytes, trust, _) = fixture_archive_with_veriloga();
        let mut tampered: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        tampered
            .files
            .iter_mut()
            .find(|file| file.path.ends_with("resistance.vams"))
            .unwrap()
            .content_base64 = STANDARD.encode(b"`define PDK_RESISTANCE 251.0\n");
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&tampered).unwrap(), &trust),
            Err(PdkTechnologyError::ArtifactDigestMismatch { .. })
        ));

        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut untyped: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&untyped.manifest_base64).unwrap()).unwrap();
        manifest
            .artifacts
            .iter_mut()
            .find(|artifact| artifact.path.ends_with("resistance.vams"))
            .unwrap()
            .kind = PdkTechnologyArtifactKind::Documentation;
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        untyped.manifest_base64 = STANDARD.encode(&manifest_bytes);
        untyped.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&untyped).unwrap(), &trust),
            Err(PdkTechnologyError::ModelMaterialization(_))
        ));
    }

    #[test]
    fn signed_veriloga_manifest_rejects_alias_collisions_unreachable_sources_and_schema_downgrade()
    {
        let (bytes, trust, _) = fixture_archive_with_veriloga();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);

        let mut collision: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&collision.manifest_base64).unwrap()).unwrap();
        let mut duplicate = manifest.veriloga_sources[0].clone();
        duplicate.source_id = "second-runtime".to_owned();
        duplicate.module_name = "second_module".to_owned();
        duplicate.netlist_alias = "PDK_RESISTOR_MODEL".to_owned();
        manifest.veriloga_sources.push(duplicate);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        collision.manifest_base64 = STANDARD.encode(&manifest_bytes);
        collision.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&collision).unwrap(), &trust),
            Err(PdkTechnologyError::Duplicate(_))
        ));

        let mut unreachable: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&unreachable.manifest_base64).unwrap())
                .unwrap();
        let orphan = b"module orphan(p); inout p; electrical p; analog I(p) <+ 0.0; endmodule\n";
        manifest.artifacts.push(PdkTechnologyArtifact {
            path: "veriloga/orphan.va".to_owned(),
            kind: PdkTechnologyArtifactKind::VerilogASource,
            size_bytes: u64::try_from(orphan.len()).unwrap(),
            sha256: content_digest(orphan),
        });
        unreachable.files.push(PdkTechnologyArchiveFile {
            path: "veriloga/orphan.va".to_owned(),
            content_base64: STANDARD.encode(orphan),
        });
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        unreachable.manifest_base64 = STANDARD.encode(&manifest_bytes);
        unreachable.signature_base64 =
            STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&unreachable).unwrap(), &trust),
            Err(PdkTechnologyError::ModelMaterialization(_))
        ));

        let mut downgraded: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&downgraded.manifest_base64).unwrap()).unwrap();
        manifest.schema_version = 1;
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        downgraded.manifest_base64 = STANDARD.encode(&manifest_bytes);
        downgraded.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&downgraded).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidField(_))
        ));
    }

    #[test]
    fn signed_model_sections_close_package_relative_dependencies_in_memory() {
        let (bytes, trust, authority) = fixture_archive();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let root = STANDARD.decode(&archive.files[0].content_base64).unwrap();
        let root = String::from_utf8(root)
            .unwrap()
            .replacen(".lib TT\n", ".lib TT\n.include \"parts/common.inc\"\n", 1)
            .into_bytes();
        archive.files[0].content_base64 = STANDARD.encode(&root);
        manifest.artifacts[0].size_bytes = u64::try_from(root.len()).unwrap();
        manifest.artifacts[0].sha256 = content_digest(&root);

        let dependency = b".model pdk_helper d is=1e-14\n".to_vec();
        manifest.artifacts.push(PdkTechnologyArtifact {
            path: "models/parts/common.inc".to_owned(),
            kind: PdkTechnologyArtifactKind::Model,
            size_bytes: u64::try_from(dependency.len()).unwrap(),
            sha256: content_digest(&dependency),
        });
        archive.files.push(PdkTechnologyArchiveFile {
            path: "models/parts/common.inc".to_owned(),
            content_base64: STANDARD.encode(&dependency),
        });
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        let signed = serde_json::to_vec(&archive).unwrap();

        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(
                &signed,
                &trust,
                &authority,
                "Install dependency-closed package",
            )
            .expect("package-relative dependency validates");
        let package = registry.validated_packages()[0].clone();
        let sealed = registry
            .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
            .expect("dependency closure seals");
        assert_eq!(sealed.sources.len(), 2);
        assert_eq!(sealed.edges.len(), 1);
        let combined = crate::state::model_library::ModelLibraryManager::new()
            .seal_execution_sources()
            .unwrap()
            .with_pdk_model_sources(sealed)
            .unwrap();
        let cards = combined
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .unwrap();
        assert!(cards[0].contains(".model pdk_helper d is=1e-14"));
        assert!(!cards[0].contains(".include"));
        assert!(!cards[0].contains("/rspice-pdk/"));
    }

    #[test]
    fn signed_model_contract_rejects_external_dependencies_and_missing_reference_process() {
        let (bytes, trust, _) = fixture_archive();
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();

        let external = b".include \"../../outside.lib\"\n.model nmos_demo nmos level=1".to_vec();
        archive.files[0].content_base64 = STANDARD.encode(&external);
        manifest.artifacts[0].size_bytes = u64::try_from(external.len()).unwrap();
        manifest.artifacts[0].sha256 = content_digest(&external);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::ModelMaterialization(_))
        ));

        let (bytes, trust, _) = fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        manifest
            .model_sources
            .retain(|contract| contract.process != PdkModelProcess::Tt);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidReference(_))
        ));
    }

    #[test]
    fn project_model_sealing_rejects_post_validation_archive_mutation() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install exact package")
            .expect("install");
        let package = registry.validated_packages()[0].clone();
        registry.archives[0].files[0].content_base64 =
            STANDARD.encode(b".model attacker nmos level=1");
        assert!(matches!(
            registry.seal_model_sources_for_binding(&package.binding(), package.archive_digest()),
            Err(PdkTechnologyError::NotRuntimeValidated(_))
        ));
    }

    #[test]
    fn signed_archive_verifies_every_exact_artifact_and_contract() {
        let (bytes, trust, _) = fixture_archive();
        let (_, package) = validate_archive_bytes(&bytes, &trust).expect("archive validates");

        assert_eq!(package.manifest().package_id, "demo180");
        assert_eq!(package.manifest().layers.len(), 3);
        assert_eq!(package.artifact_digests().len(), 2);
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            package.archive_digest(),
            content_digest(&serde_json::to_vec(&archive).unwrap())
        );
    }

    #[test]
    fn recognition_and_extraction_contracts_are_typed_complete_and_source_bound() {
        let (bytes, trust, _) = fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
        let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();

        let specialized = [
            (
                "recognition/nmos.json",
                PdkTechnologyArtifactKind::RecognitionMap,
                br#"{"device":"nmos"}"#.as_slice(),
            ),
            (
                "extraction/rc.json",
                PdkTechnologyArtifactKind::ExtractionRule,
                br#"{"quantities":["r","c"]}"#.as_slice(),
            ),
            (
                "qualification/nmos-layout.json",
                PdkTechnologyArtifactKind::QualificationVector,
                br#"{"shapes":[]}"#.as_slice(),
            ),
            (
                "qualification/rc-layout.json",
                PdkTechnologyArtifactKind::QualificationVector,
                br#"{"wires":[]}"#.as_slice(),
            ),
            (
                "qualification/rc-reference.json",
                PdkTechnologyArtifactKind::QualificationReference,
                br#"{"r":10.0,"c":1e-15}"#.as_slice(),
            ),
        ];
        for (path, kind, content) in specialized {
            manifest.artifacts.push(PdkTechnologyArtifact {
                path: path.to_owned(),
                kind,
                size_bytes: u64::try_from(content.len()).unwrap(),
                sha256: content_digest(content),
            });
            archive.files.push(PdkTechnologyArchiveFile {
                path: path.to_owned(),
                content_base64: STANDARD.encode(content),
            });
        }
        manifest.recognition = vec![PdkRecognitionContract {
            contract_id: "recognize-nmos".to_owned(),
            device_class: "nmos".to_owned(),
            rule_artifact_path: "recognition/nmos.json".to_owned(),
            terminals: vec![
                PdkRecognitionTerminal {
                    terminal_name: "source".to_owned(),
                    layer: "active".to_owned(),
                    purpose: "drawing".to_owned(),
                },
                PdkRecognitionTerminal {
                    terminal_name: "drain".to_owned(),
                    layer: "active".to_owned(),
                    purpose: "drawing".to_owned(),
                },
            ],
            qualification_vectors: vec![PdkRecognitionQualificationVector {
                vector_id: "recognize-nmos-positive".to_owned(),
                layout_artifact_path: "qualification/nmos-layout.json".to_owned(),
                expected_instance_count: 1,
            }],
        }];
        manifest.extraction = vec![PdkExtractionContract {
            contract_id: "extract-metal-rc".to_owned(),
            rule_artifact_path: "extraction/rc.json".to_owned(),
            quantities: vec![
                PdkExtractionQuantity::Resistance,
                PdkExtractionQuantity::Capacitance,
            ],
            layer_purposes: vec![PdkLayerPurposeRef {
                layer: "metal1".to_owned(),
                purpose: "drawing".to_owned(),
            }],
            qualification_vectors: vec![PdkExtractionQualificationVector {
                vector_id: "extract-metal-rc-reference".to_owned(),
                layout_artifact_path: "qualification/rc-layout.json".to_owned(),
                reference_artifact_path: "qualification/rc-reference.json".to_owned(),
            }],
        }];

        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        let signed = serde_json::to_vec(&archive).unwrap();
        let (_, package) =
            validate_archive_bytes(&signed, &trust).expect("typed contracts validate");
        assert_eq!(package.manifest().recognition.len(), 1);
        assert_eq!(package.manifest().extraction.len(), 1);

        let mut invalid = manifest.clone();
        invalid.recognition[0].terminals[0].purpose = "undeclared".to_owned();
        let invalid_bytes = serde_json::to_vec(&invalid).unwrap();
        archive.manifest_base64 = STANDARD.encode(&invalid_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&invalid_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidReference(_))
        ));

        let mut duplicate = manifest;
        duplicate.extraction[0].qualification_vectors[0].layout_artifact_path =
            "qualification/nmos-layout.json".to_owned();
        let duplicate_bytes = serde_json::to_vec(&duplicate).unwrap();
        archive.manifest_base64 = STANDARD.encode(&duplicate_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&duplicate_bytes).to_bytes());
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&archive).unwrap(), &trust),
            Err(PdkTechnologyError::Duplicate(_))
        ));
    }

    #[test]
    fn project_pin_resolves_only_the_exact_currently_trusted_archive() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install for project pin")
            .expect("install");
        let package = registry
            .validated_packages()
            .first()
            .expect("installed package validates");
        let pin =
            crate::state::workspace::ProjectSignedTechnologyPin::from_validated_package(package)
                .expect("project pin");
        pin.validate_registry(&registry)
            .expect("exact trusted archive resolves");

        let json = serde_json::to_string(&pin).expect("pin serializes");
        let restored: crate::state::workspace::ProjectSignedTechnologyPin =
            serde_json::from_str(&json).expect("pin deserializes");
        assert_eq!(restored, pin);

        let mut revoked = trust;
        revoked.keys[0].revoked = true;
        registry
            .revalidate_installed(&revoked)
            .expect_err("revocation invalidates runtime packages");
        assert!(matches!(
            pin.validate_registry(&registry),
            Err(crate::state::workspace::TechnologyBindingError::SignedPackageUnavailable { .. })
        ));
    }

    #[test]
    fn publisher_key_provision_and_revocation_are_immutable_and_hash_chained() {
        let (bytes, fixture_trust, authority) = fixture_archive();
        let key = fixture_trust.keys[0].clone();
        let mut trust = PdkPublisherTrustStore::default();
        let provision = trust
            .provision_key(key.clone(), &authority, "Approve foundry ceremony key")
            .expect("provision");
        assert_eq!(provision.action, PdkTrustAuditAction::Provision);
        assert!(validate_archive_bytes(&bytes, &trust).is_ok());

        let before_duplicate = trust.clone();
        assert!(matches!(
            trust.provision_key(key.clone(), &authority, "Duplicate"),
            Err(PdkTechnologyError::ImmutableTrustKey(_))
        ));
        assert_eq!(trust, before_duplicate);

        let revoke = trust
            .revoke_key(
                &key.publisher_id,
                &key.key_id,
                &authority,
                "Publisher key retired",
            )
            .expect("revoke");
        assert_eq!(revoke.action, PdkTrustAuditAction::Revoke);
        assert_eq!(
            revoke.previous_receipt_digest,
            Some(provision.receipt_digest)
        );
        assert!(matches!(
            validate_archive_bytes(&bytes, &trust),
            Err(PdkTechnologyError::RevokedPublisherKey { .. })
        ));
        let before_second_revoke = trust.clone();
        assert!(matches!(
            trust.revoke_key(&key.publisher_id, &key.key_id, &authority, "Revoke again"),
            Err(PdkTechnologyError::ImmutableTrustKey(_))
        ));
        assert_eq!(trust, before_second_revoke);

        let json = serde_json::to_string(&trust).expect("serialize governed trust");
        let restored: PdkPublisherTrustStore =
            serde_json::from_str(&json).expect("deserialize governed trust");
        restored.validate().expect("restored audit validates");

        let mut tampered = restored;
        tampered.audit[0].reason = "altered".to_owned();
        assert!(matches!(
            tampered.validate(),
            Err(PdkTechnologyError::TrustAuditCorrupted(_))
        ));
    }

    #[test]
    fn tampered_artifact_signature_and_unknown_key_fail_closed() {
        let (bytes, trust, _) = fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        archive.files[0].content_base64 = STANDARD.encode(b"tampered");
        let tampered = serde_json::to_vec(&archive).unwrap();
        assert!(matches!(
            validate_archive_bytes(&tampered, &trust),
            Err(PdkTechnologyError::ArtifactSizeMismatch { .. })
                | Err(PdkTechnologyError::ArtifactDigestMismatch { .. })
        ));

        let mut signature_tampered: SignedPdkTechnologyArchive =
            serde_json::from_slice(&bytes).unwrap();
        signature_tampered.signature_base64 = STANDARD.encode([0_u8; 64]);
        assert!(matches!(
            validate_archive_bytes(&serde_json::to_vec(&signature_tampered).unwrap(), &trust),
            Err(PdkTechnologyError::InvalidSignature { .. })
        ));

        assert!(matches!(
            validate_archive_bytes(&bytes, &PdkPublisherTrustStore::default()),
            Err(PdkTechnologyError::UntrustedPublisher { .. })
        ));
    }

    #[test]
    fn network_callbacks_and_incomplete_stream_maps_are_rejected() {
        let (bytes, trust, _) = fixture_archive();
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
        let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();
        manifest.callbacks[0]
            .capabilities
            .push(PdkCallbackCapability::Network);
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PdkTechnologyError::ForbiddenCapability(_))
        ));

        manifest.callbacks[0].capabilities.pop();
        manifest.stream_map.pop();
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PdkTechnologyError::MissingMapping(_))
        ));

        let mut revoked = trust;
        revoked.keys[0].revoked = true;
        assert!(matches!(
            validate_archive_bytes(&bytes, &revoked),
            Err(PdkTechnologyError::RevokedPublisherKey { .. })
        ));
    }

    #[test]
    fn target_declarations_are_nonempty_and_activation_is_platform_scoped() {
        let (bytes, trust, authority) = fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
        let mut manifest: PdkTechnologyManifest = serde_json::from_slice(&manifest_bytes).unwrap();
        manifest.compatibility.targets.clear();
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PdkTechnologyError::InvalidField(_))
        ));

        manifest.compatibility.targets = vec![match current_execution_target() {
            PdkExecutionTarget::Desktop => PdkExecutionTarget::WebAssembly,
            PdkExecutionTarget::WebAssembly | PdkExecutionTarget::Mobile => {
                PdkExecutionTarget::Desktop
            }
        }];
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        let restricted_bytes = serde_json::to_vec(&archive).unwrap();

        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(
                &restricted_bytes,
                &trust,
                &authority,
                "Install platform-restricted package",
            )
            .expect("a platform-restricted package can be inspected");
        assert!(matches!(
            registry.activate(
                "demo180",
                "2.3.1",
                &authority,
                "Attempt incompatible activation"
            ),
            Err(PdkTechnologyError::IncompatibleRuntime(_))
        ));
        assert!(registry.active_binding().is_none());
    }

    #[test]
    fn install_activation_and_rollback_are_hash_chained_and_revalidated() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        let install = registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install reviewed package")
            .expect("install");
        let activate = registry
            .activate(
                "demo180",
                "2.3.1",
                &authority,
                "Activate for new project bindings",
            )
            .expect("activate");
        assert_eq!(
            activate.previous_receipt_digest,
            Some(install.receipt_digest)
        );
        registry.validate_audit_chain().expect("audit chain");
        assert!(registry.active_package().is_some());

        let json = serde_json::to_string(&registry).unwrap();
        let mut restored: PdkTechnologyRegistry = serde_json::from_str(&json).unwrap();
        assert!(restored.active_package().is_none());
        assert!(!restored.runtime_ready());
        restored
            .revalidate_installed(&trust)
            .expect("revalidate persisted archive");
        assert!(restored.active_package().is_some());

        // The same active target is not a rollback. A future revision must be
        // activated before returning to this retained prior binding.
        assert!(matches!(
            restored.rollback_to("demo180", "2.3.1", &authority, "No intervening revision"),
            Err(PdkTechnologyError::InvalidTransition(_))
        ));
    }

    #[test]
    fn immutable_revision_and_tampered_audit_fail_before_mutation() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install")
            .unwrap();
        let before = registry.clone();
        assert!(matches!(
            registry.install_archive_bytes(&bytes, &trust, &authority, "Install again"),
            Err(PdkTechnologyError::ImmutableRevision(_))
        ));
        assert_eq!(registry, before);

        registry.audit[0].reason = "altered".to_owned();
        let tampered = registry.clone();
        assert!(matches!(
            registry.activate("demo180", "2.3.1", &authority, "Activate"),
            Err(PdkTechnologyError::AuditCorrupted(_))
        ));
        assert_eq!(registry, tampered);
    }

    #[test]
    fn recomputed_hashes_cannot_disguise_impossible_transitions_or_wrong_archives() {
        let (bytes, trust, authority) = fixture_archive();
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install")
            .unwrap();

        let target = registry.audit[0].target.clone();
        registry.audit[0].after_active = Some(target.clone());
        registry.audit[0].receipt_digest = registry.audit[0].calculate_digest().unwrap();
        registry.active = Some(target);
        assert!(matches!(
            registry.validate_audit_chain(),
            Err(PdkTechnologyError::AuditCorrupted(_))
        ));

        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(&bytes, &trust, &authority, "Install")
            .unwrap();
        registry.audit[0].archive_digest = content_digest(b"wrong archive");
        registry.audit[0].receipt_digest = registry.audit[0].calculate_digest().unwrap();
        let errors = registry
            .revalidate_installed(&trust)
            .expect_err("receipt must bind the installed archive");
        assert!(
            errors
                .iter()
                .any(|error| error.contains("archive digest does not match"))
        );
    }

    #[test]
    fn unknown_manifest_fields_and_unsafe_paths_are_rejected() {
        let (bytes, trust, _) = fixture_archive();
        let archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let manifest_bytes = STANDARD.decode(&archive.manifest_base64).unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&manifest_bytes).unwrap();
        value["unknown"] = serde_json::json!(true);
        assert!(
            serde_json::from_value::<PdkTechnologyManifest>(value)
                .unwrap_err()
                .to_string()
                .contains("unknown field")
        );
        assert!(matches!(
            validate_package_path("artifact", "../secret"),
            Err(PdkTechnologyError::InvalidField(_))
        ));
        assert!(trust.validate().is_ok());
    }
}
