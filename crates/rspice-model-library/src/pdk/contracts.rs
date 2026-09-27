//! Portable signed-PDK archive, process, physical-technology and callback descriptors.
//!
//! These retained declarations do not grant installation or execution authority.

use rspice_app_types::product::ContentDigest;
use serde::{Deserialize, Serialize};

pub const PDK_TECHNOLOGY_ARCHIVE_SCHEMA_VERSION: u32 = 1;
pub const PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION: u32 = 5;
pub const MINIMUM_PDK_TECHNOLOGY_MANIFEST_SCHEMA_VERSION: u32 = 1;
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
pub struct PdkTechnologyBinding {
    pub package_id: String,
    pub revision: String,
    pub manifest_digest: ContentDigest,
}
