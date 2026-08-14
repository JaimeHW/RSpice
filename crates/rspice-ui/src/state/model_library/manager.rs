//! The model library.
//!
//! Owns every model a project can resolve, from every source, and seals the
//! exact set a run executed against. Sealing is by content digest, so a
//! library that changes underneath a completed run is detectable rather
//! than silently assumed identical.

mod project_models;
mod sealing;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[cfg(any(test, target_arch = "wasm32"))]
use std::collections::VecDeque;

use rspice_core::library::SpiceLibraryIndex;

#[cfg(test)]
use super::ModelFileIdentity;
#[cfg(not(target_arch = "wasm32"))]
use super::is_foreign_platform_absolute_path;
use super::{
    DeviceModel, FiniteF64, ModelCorrelationState, ModelDefinitionMetadata, ModelLevel,
    ModelLibrary, ModelQualificationState, ModelSectionQualification, ModelSourceAuthority,
    ModelSourceContent, ModelSourceEdge, ModelSourceEvidenceBinding, ModelSourcePin, ModelType,
    ParameterDataType, ParameterDefinition, ParameterSource, ParameterValue, ProcessCorner,
    ProjectModelDefinition, ProjectModelRevisionDefinition, first_unreachable_source,
    subcircuit_interface_key,
};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
use crate::services::simulation_runner::{CornerModelBinding, CornerProcess};

/// Published result of one atomic project-model definition transaction.
#[derive(Debug, Clone)]
pub struct ProjectModelCommit {
    pub library_name: String,
    pub model_name: String,
    pub before: Option<ModelLibrary>,
    pub after: ModelLibrary,
    /// Definition/source changes invalidate downstream execution; evidence-
    /// only commits do not alter the executable model closure.
    pub affects_execution: bool,
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn normalize_browser_bundle_member_path(path: &str) -> Result<String, String> {
    if path.is_empty() {
        return Err("the relative path is empty".to_owned());
    }
    if path.chars().any(char::is_control) {
        return Err("the relative path contains a control character".to_owned());
    }

    let normalized = path.replace('\\', "/");
    if normalized.starts_with('/') {
        return Err("absolute paths are not allowed".to_owned());
    }

    let mut components = Vec::new();
    for component in normalized.split('/') {
        if component.is_empty() {
            return Err("empty path components are not allowed".to_owned());
        }
        if matches!(component, "." | "..") {
            return Err("'.' and '..' components are not allowed in member identities".to_owned());
        }
        if component.contains(':') {
            return Err("':' is not allowed in a portable member identity".to_owned());
        }
        components.push(component);
    }
    Ok(components.join("/"))
}

#[cfg(any(test, target_arch = "wasm32"))]
fn resolve_browser_bundle_dependency(owner: &str, requested_path: &str) -> Result<String, String> {
    let requested = rspice_core::netlist::normalize_source_path_literal(requested_path)
        .map_err(|error| error.to_string())?;
    if requested.starts_with('/') {
        return Err("absolute paths are not allowed".to_owned());
    }

    let mut components = owner.split('/').collect::<Vec<_>>();
    let owner_file = components
        .pop()
        .ok_or_else(|| "the owning source has no file name".to_owned())?;
    if owner_file.is_empty() {
        return Err("the owning source has no file name".to_owned());
    }

    for component in requested.split('/') {
        match component {
            "" => return Err("empty path components are not allowed".to_owned()),
            "." => {}
            ".." => {
                if components.pop().is_none() {
                    return Err("the dependency escapes the selected source tree".to_owned());
                }
            }
            component if component.contains(':') => {
                return Err("':' is not allowed in a portable dependency path".to_owned());
            }
            component => components.push(component),
        }
    }
    if components.is_empty() {
        return Err("the dependency does not identify a source file".to_owned());
    }
    Ok(components.join("/"))
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_bundle_source_candidate(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "lib" | "model" | "mod" | "spice" | "cir" | "inc" | "scs"
            )
        })
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_bundle_veriloga_member(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "va" | "vams" | "vh"
            )
        })
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_veriloga_include(line: &str) -> Option<String> {
    let remainder = line.trim().strip_prefix("`include")?.trim_start();
    let remainder = remainder.strip_prefix('"')?;
    let end = remainder.find('"')?;
    Some(remainder[..end].to_owned())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_bundle_direct_dependencies(
    owner: &str,
    bytes: &[u8],
) -> Result<Vec<(usize, String)>, String> {
    let source = rspice_core::netlist::decode_source_bytes(bytes)
        .map_err(|error| format!("Uploaded model source '{owner}' cannot be decoded: {error}"))?;
    if browser_bundle_veriloga_member(owner) {
        return Ok(source
            .lines()
            .enumerate()
            .filter_map(|(line, source)| {
                browser_veriloga_include(source).map(|path| (line + 1, path))
            })
            .collect());
    }

    let projection = rspice_core::library::adapt_spectre_model_library(Path::new(owner), &source)
        .map_err(|error| {
        format!(
            "{owner}:{} cannot be imported as an executable model library: {}",
            error.line, error.message
        )
    })?;
    Ok(projection
        .lines()
        .enumerate()
        .filter_map(|(line, source)| {
            rspice_core::netlist::parse_include_directive(source)
                .or_else(|| {
                    rspice_core::netlist::parse_lib_directive(source)
                        .and_then(|(path, section)| section.map(|_| path))
                })
                .or_else(|| {
                    rspice_core::netlist::parse_veriloga_source_directive(source)
                        .map(|include| include.file_path.to_string_lossy().into_owned())
                })
                .map(|path| (line + 1, path))
        })
        .collect())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn infer_browser_bundle_root(
    members: &BTreeMap<String, Vec<u8>>,
    case_folded: &HashMap<String, String>,
) -> Result<Option<String>, String> {
    if members.len() == 1 {
        return Ok(members.keys().next().cloned());
    }
    let candidates = members
        .keys()
        .filter(|path| browser_bundle_source_candidate(path))
        .cloned()
        .collect::<Vec<_>>();
    let mut dependency_targets = HashSet::new();
    for owner in &candidates {
        let Ok(dependencies) = browser_bundle_direct_dependencies(owner, &members[owner]) else {
            continue;
        };
        for (_, requested) in dependencies {
            let Ok(normalized) = resolve_browser_bundle_dependency(owner, &requested) else {
                continue;
            };
            if let Some(target) = case_folded.get(&normalized.to_ascii_lowercase()) {
                dependency_targets.insert(target.clone());
            }
        }
    }
    let mut roots = candidates
        .into_iter()
        .filter(|candidate| !dependency_targets.contains(candidate))
        .collect::<Vec<_>>();
    Ok((roots.len() == 1).then(|| roots.pop().expect("one inferred root")))
}

#[cfg(any(test, target_arch = "wasm32"))]
fn reachable_browser_bundle_members(
    root: &str,
    members: &BTreeMap<String, Vec<u8>>,
    case_folded: &HashMap<String, String>,
) -> Result<HashSet<String>, String> {
    let mut reachable = HashSet::new();
    let mut pending = VecDeque::from([root.to_owned()]);
    while let Some(owner) = pending.pop_front() {
        if !reachable.insert(owner.clone()) {
            continue;
        }
        let bytes = members
            .get(&owner)
            .ok_or_else(|| format!("Selected model source root '{owner}' disappeared"))?;
        for (line, requested) in browser_bundle_direct_dependencies(&owner, bytes)? {
            let normalized =
                resolve_browser_bundle_dependency(&owner, &requested).map_err(|error| {
                    format!("{owner}:{line} has an invalid dependency path '{requested}': {error}")
                })?;
            let target = case_folded
                .get(&normalized.to_ascii_lowercase())
                .ok_or_else(|| {
                    format!(
                        "{owner}:{line} dependency '{requested}' is missing from the selected browser bundle"
                    )
                })?
                .clone();
            pending.push_back(target);
        }
    }
    Ok(reachable)
}

#[cfg(any(test, target_arch = "wasm32"))]
fn browser_bundle_path_ends_with(path: &Path, member: &str) -> bool {
    let path = path
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase();
    let member = member.to_ascii_lowercase();
    path == member
        || path
            .strip_suffix(&member)
            .is_some_and(|prefix| prefix.ends_with('/'))
}

pub const MODEL_RESOLUTION_RECORD_SCHEMA_VERSION: u16 = 1;
pub const MODEL_VALIDATION_RECEIPT_SCHEMA_VERSION: u16 = 1;

/// Consumer namespace governed by one explicit provider decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelConsumerScope {
    PrimitiveModel,
    Subcircuit,
}

impl ModelConsumerScope {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PrimitiveModel => "primitive model",
            Self::Subcircuit => "subcircuit",
        }
    }

    const fn key(self) -> &'static str {
        match self {
            Self::PrimitiveModel => "model",
            Self::Subcircuit => "subckt",
        }
    }
}

/// Exact project-owned decision for a contested executable definition.
///
/// The provider's authenticated source digest makes the decision expire when
/// a source is refreshed, even if the library and definition names are reused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelResolutionRecord {
    pub schema_version: u16,
    pub consumer_scope: ModelConsumerScope,
    pub normalized_name: String,
    pub provider_library: String,
    pub provider_definition: String,
    pub provider_source_digest: ContentDigest,
    pub audit_reason: String,
    pub created_at_unix_ms: u64,
}

impl ModelResolutionRecord {
    fn key(&self) -> String {
        resolution_record_key(self.consumer_scope, &self.normalized_name)
    }

    fn validate(&self) -> Result<(), String> {
        if self.schema_version != MODEL_RESOLUTION_RECORD_SCHEMA_VERSION {
            return Err(format!(
                "model-resolution record for '{}' uses unsupported schema {}",
                self.normalized_name, self.schema_version
            ));
        }
        let normalized = self.normalized_name.trim().to_ascii_lowercase();
        if normalized.is_empty() || normalized != self.normalized_name {
            return Err("model-resolution name must be nonempty canonical lowercase".to_owned());
        }
        if self.provider_definition.to_ascii_lowercase() != self.normalized_name {
            return Err(
                "model-resolution provider definition does not match its canonical name".to_owned(),
            );
        }
        for (field, value, maximum) in [
            (
                "provider library",
                self.provider_library.as_str(),
                512_usize,
            ),
            (
                "provider definition",
                self.provider_definition.as_str(),
                512_usize,
            ),
            ("audit reason", self.audit_reason.as_str(), 2_048_usize),
        ] {
            if value.is_empty()
                || value != value.trim()
                || value.len() > maximum
                || value.chars().any(char::is_control)
            {
                return Err(format!(
                    "model-resolution {field} must be nonempty, trimmed, control-free, and at most {maximum} bytes"
                ));
            }
        }
        if self.created_at_unix_ms == 0 {
            return Err("model-resolution timestamp must be nonzero".to_owned());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModelDefinitionProvider {
    pub library: String,
    pub definition: String,
    pub source_digest: ContentDigest,
}

fn resolution_record_key(scope: ModelConsumerScope, normalized_name: &str) -> String {
    format!("{}:{normalized_name}", scope.key())
}

pub(crate) fn model_library_source_digest(library: &ModelLibrary) -> ContentDigest {
    if let Some(root) = library.root_path.as_deref()
        && let Some(pin) = library.source_closure.iter().find(|pin| pin.path == root)
    {
        return pin.digest;
    }
    let bytes = serde_json::to_value(library)
        .and_then(|canonical| serde_json::to_vec(&canonical))
        .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

/// One ordered model-library binding owned by a simulation plan.
///
/// The name is the project-catalog identity, the digest prevents a refreshed
/// or replaced source from being accepted under an old plan, and the optional
/// corner is the plan's nominal section override. Vector order is executable
/// precedence; it is never reconstructed from the manager's hash map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanModelBinding {
    pub library_name: String,
    pub source_digest: ContentDigest,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_corner: Option<String>,
}

impl SimulationPlanModelBinding {
    fn validate(&self) -> Result<(), String> {
        for (field, value) in [("library name", self.library_name.as_str())]
            .into_iter()
            .chain(
                self.selected_corner
                    .as_deref()
                    .map(|value| ("corner section", value)),
            )
        {
            if value.is_empty() || value != value.trim() || value.chars().any(char::is_control) {
                return Err(format!(
                    "Simulation-plan model {field} must be nonempty, trimmed, and control-free"
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelValidationFindingSeverity {
    Information,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelValidationFinding {
    pub code: String,
    pub severity: ModelValidationFindingSeverity,
    pub message: String,
}

/// Durable evidence that one exact project revision passed the executable
/// model pipeline on one supported platform and engine/schema build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelValidationReceipt {
    pub schema_version: u16,
    pub project_revision: ObjectRevision,
    pub model_execution_plan_digest: ContentDigest,
    pub execution_catalog_digest: ContentDigest,
    /// Number of authenticated ordinary-library source members represented by
    /// `source_closure_digest`. The receipt stores one canonical digest rather
    /// than duplicating every source path and digest into project metadata.
    pub source_count: u64,
    pub source_closure_digest: ContentDigest,
    pub pdk_archive_digest: Option<ContentDigest>,
    pub engine_version: String,
    pub execution_schema_version: u32,
    pub platform: String,
    pub findings: Vec<ModelValidationFinding>,
    pub validated_at_unix_ms: u64,
    pub receipt_digest: ContentDigest,
}

impl ModelValidationReceipt {
    #[allow(clippy::too_many_arguments)]
    fn issue(
        project_revision: ObjectRevision,
        model_execution_plan_digest: ContentDigest,
        execution_catalog_digest: ContentDigest,
        source_count: u64,
        source_closure_digest: ContentDigest,
        pdk_archive_digest: Option<ContentDigest>,
        execution_schema_version: u32,
        findings: Vec<ModelValidationFinding>,
    ) -> Result<Self, String> {
        let engine_version = env!("CARGO_PKG_VERSION").to_owned();
        let platform = model_validation_platform().to_owned();
        let validated_at_unix_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| format!("system clock cannot timestamp model validation: {error}"))?
            .as_millis()
            .try_into()
            .map_err(|_| "model-validation timestamp exceeds the supported range".to_owned())?;
        let receipt_digest = model_validation_receipt_digest(
            project_revision,
            model_execution_plan_digest,
            execution_catalog_digest,
            source_count,
            source_closure_digest,
            pdk_archive_digest,
            &engine_version,
            execution_schema_version,
            &platform,
            &findings,
            validated_at_unix_ms,
        )?;
        let receipt = Self {
            schema_version: MODEL_VALIDATION_RECEIPT_SCHEMA_VERSION,
            project_revision,
            model_execution_plan_digest,
            execution_catalog_digest,
            source_count,
            source_closure_digest,
            pdk_archive_digest,
            engine_version,
            execution_schema_version,
            platform,
            findings,
            validated_at_unix_ms,
            receipt_digest,
        };
        receipt.verify()?;
        Ok(receipt)
    }

    pub fn verify(&self) -> Result<(), String> {
        if self.schema_version != MODEL_VALIDATION_RECEIPT_SCHEMA_VERSION {
            return Err(format!(
                "model-validation receipt uses unsupported schema {}",
                self.schema_version
            ));
        }
        if self.engine_version.trim().is_empty()
            || self.engine_version != self.engine_version.trim()
            || self.engine_version.len() > 128
            || !matches!(
                self.platform.as_str(),
                "desktop-windows" | "desktop-macos" | "desktop-linux" | "browser-wasm32"
            )
            || self.validated_at_unix_ms == 0
        {
            return Err(
                "model-validation receipt has an invalid engine, platform, or timestamp identity"
                    .to_owned(),
            );
        }
        if self.findings.is_empty() || self.findings.len() > 64 {
            return Err(
                "model-validation receipt must retain between 1 and 64 bounded findings".to_owned(),
            );
        }
        for finding in &self.findings {
            for (field, value, maximum) in [
                ("finding code", finding.code.as_str(), 128_usize),
                ("finding message", finding.message.as_str(), 2_048_usize),
            ] {
                if value.is_empty()
                    || value != value.trim()
                    || value.len() > maximum
                    || value.chars().any(char::is_control)
                {
                    return Err(format!(
                        "model-validation {field} must be nonempty, trimmed, control-free, and at most {maximum} bytes"
                    ));
                }
            }
        }
        let expected = model_validation_receipt_digest(
            self.project_revision,
            self.model_execution_plan_digest,
            self.execution_catalog_digest,
            self.source_count,
            self.source_closure_digest,
            self.pdk_archive_digest,
            &self.engine_version,
            self.execution_schema_version,
            &self.platform,
            &self.findings,
            self.validated_at_unix_ms,
        )?;
        if expected != self.receipt_digest {
            return Err("model-validation receipt digest does not match its payload".to_owned());
        }
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn model_validation_receipt_digest(
    project_revision: ObjectRevision,
    model_execution_plan_digest: ContentDigest,
    execution_catalog_digest: ContentDigest,
    source_count: u64,
    source_closure_digest: ContentDigest,
    pdk_archive_digest: Option<ContentDigest>,
    engine_version: &str,
    execution_schema_version: u32,
    platform: &str,
    findings: &[ModelValidationFinding],
    validated_at_unix_ms: u64,
) -> Result<ContentDigest, String> {
    let bytes = serde_json::to_vec(&(
        MODEL_VALIDATION_RECEIPT_SCHEMA_VERSION,
        project_revision,
        model_execution_plan_digest,
        execution_catalog_digest,
        source_count,
        source_closure_digest,
        pdk_archive_digest,
        engine_version,
        execution_schema_version,
        platform,
        findings,
        validated_at_unix_ms,
    ))
    .map_err(|error| format!("Cannot serialize model-validation receipt payload: {error}"))?;
    Ok(ContentDigest::from_bytes(Sha256::digest(bytes).into()))
}

const fn model_validation_platform() -> &'static str {
    if cfg!(target_arch = "wasm32") {
        "browser-wasm32"
    } else if cfg!(target_os = "windows") {
        "desktop-windows"
    } else if cfg!(target_os = "macos") {
        "desktop-macos"
    } else if cfg!(target_os = "linux") {
        "desktop-linux"
    } else {
        "desktop-unsupported"
    }
}

/// One immutable, authenticated model-source snapshot for a simulation run.
/// The exact bytes are intentionally transient and are never serialized into
/// project/session state.
#[derive(Debug, Clone)]
pub struct SealedModelExecutionSources {
    bundle: rspice_core::netlist::SealedSourceBundle,
    sources: Vec<(PathBuf, String)>,
    edges: Vec<rspice_core::netlist::SealedSourceEdge>,
    model_library_source_paths: Vec<PathBuf>,
    libraries: Vec<SealedExecutionLibrary>,
    pdk_process_bindings: Vec<crate::state::pdk_config::SealedPdkModelProcessBinding>,
    pdk_veriloga_artifacts: Vec<crate::state::pdk_config::SealedPdkVerilogAArtifact>,
    pdk_veriloga_bindings: Vec<crate::state::pdk_config::SealedPdkVerilogABinding>,
    pdk_identity: Option<(
        crate::state::pdk_config::PdkTechnologyBinding,
        ContentDigest,
    )>,
    resolution_records: Vec<ModelResolutionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SealedModelLibraryVerilogARoot {
    pub(crate) path: PathBuf,
    pub(crate) netlist_alias: Option<String>,
}

/// Exact model-library bytes and AHDL roots authenticated by one run seal.
/// Signed-PDK artifacts are intentionally excluded; they have their own
/// manifest-governed authority and compiler path.
#[derive(Debug, Clone)]
pub(crate) struct SealedModelLibraryVerilogAAuthority {
    pub(crate) closure_digest: ContentDigest,
    pub(crate) sources: Vec<(PathBuf, String)>,
    pub(crate) roots: Vec<SealedModelLibraryVerilogARoot>,
}

/// Immutable, content-addressed model namespace used by one nominal run.
///
/// This is the semantic boundary shared by preflight, save validation, and
/// prepared-run construction.  It records the exact per-library corner that
/// was selected when sources were sealed and rejects a contested executable
/// namespace before the engine can fall back to first-definition lookup.
#[derive(Debug, Clone)]
pub struct ModelExecutionPlan {
    reference_process: crate::simulation::dialog::corner::ProcessCorner,
    selected_library_corners: Vec<(String, Option<String>)>,
    bindings: Vec<CornerModelBinding>,
    applied_resolutions: Vec<ModelResolutionRecord>,
    digest: ContentDigest,
}

impl ModelExecutionPlan {
    #[must_use]
    pub const fn reference_process(&self) -> crate::simulation::dialog::corner::ProcessCorner {
        self.reference_process
    }

    #[must_use]
    pub fn selected_library_corners(&self) -> &[(String, Option<String>)] {
        &self.selected_library_corners
    }

    #[must_use]
    pub fn bindings(&self) -> &[CornerModelBinding] {
        &self.bindings
    }

    #[must_use]
    pub fn applied_resolutions(&self) -> &[ModelResolutionRecord] {
        &self.applied_resolutions
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    #[must_use]
    pub fn model_cards(&self) -> Vec<String> {
        self.bindings
            .iter()
            .map(|binding| {
                format!(
                    "* RSpice sealed model source: {}\n{}",
                    binding.source_label, binding.materialized_model_cards
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct SealedExecutionLibrary {
    name: String,
    root_path: PathBuf,
    source_digest: ContentDigest,
    corners: Vec<ProcessCorner>,
    selected_corner: Option<String>,
    allows_selected_section_override: bool,
}

/// One corner section materialized out of the sealed bundle, with the identity
/// needed to label it and the domains it covers.
struct MaterializedCornerSection {
    source_label: String,
    section: String,
    materialized_model_cards: String,
}

#[derive(Debug, Clone)]
struct MaterializedPlanBinding {
    binding: CornerModelBinding,
    provider_library: String,
    provider_source_digest: ContentDigest,
    allows_selected_section_override: bool,
}

#[derive(Debug, Clone)]
struct MaterializedDefinition {
    scope: ModelConsumerScope,
    normalized_name: String,
    exact_name: String,
    binding_index: usize,
    name_span: std::ops::Range<usize>,
}

const fn pdk_model_process(process: CornerProcess) -> crate::state::pdk_config::PdkModelProcess {
    match process {
        CornerProcess::TT => crate::state::pdk_config::PdkModelProcess::Tt,
        CornerProcess::SS => crate::state::pdk_config::PdkModelProcess::Ss,
        CornerProcess::FF => crate::state::pdk_config::PdkModelProcess::Ff,
        CornerProcess::SF => crate::state::pdk_config::PdkModelProcess::Sf,
        CornerProcess::FS => crate::state::pdk_config::PdkModelProcess::Fs,
    }
}

const fn simulation_corner_process(
    process: crate::simulation::dialog::corner::ProcessCorner,
) -> CornerProcess {
    match process {
        crate::simulation::dialog::corner::ProcessCorner::TT => CornerProcess::TT,
        crate::simulation::dialog::corner::ProcessCorner::SS => CornerProcess::SS,
        crate::simulation::dialog::corner::ProcessCorner::FF => CornerProcess::FF,
        crate::simulation::dialog::corner::ProcessCorner::SF => CornerProcess::SF,
        crate::simulation::dialog::corner::ProcessCorner::FS => CornerProcess::FS,
    }
}

fn hash_plan_field(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value);
}

fn materialized_definitions(binding_index: usize, cards: &str) -> Vec<MaterializedDefinition> {
    // The editor source map deliberately treats the first line as a SPICE
    // title. Prefix one title so the first actual model card is inspected.
    let prefix = "RSpice materialized provider\n";
    let wrapped = format!("{prefix}{cards}");
    let map = rspice_core::netlist::source_map_for_editor(&wrapped);
    map.model_defs
        .into_iter()
        .filter(|definition| definition.scope.is_none())
        .map(|definition| MaterializedDefinition {
            scope: ModelConsumerScope::PrimitiveModel,
            normalized_name: definition.name.to_ascii_lowercase(),
            exact_name: definition.name,
            binding_index,
            name_span: (definition.span.start - prefix.len())..(definition.span.end - prefix.len()),
        })
        .chain(
            map.subckt_defs
                .into_iter()
                .filter(|definition| definition.scope.is_none())
                .map(|definition| MaterializedDefinition {
                    scope: ModelConsumerScope::Subcircuit,
                    normalized_name: definition.name.to_ascii_lowercase(),
                    exact_name: definition.name,
                    binding_index,
                    name_span: (definition.span.start - prefix.len())
                        ..(definition.span.end - prefix.len()),
                }),
        )
        .collect()
}

/// Canonical project-owned model revisions intentionally carry one top-level
/// base card plus one complete card in each `.lib` section. Selecting such a
/// section must replace the base card, while duplicates in imported or signed
/// sources remain errors. Perform that one narrowly authorized rewrite before
/// resolving conflicts between independent providers.
fn apply_project_section_overrides(bindings: &mut [MaterializedPlanBinding]) -> Result<(), String> {
    let definitions = bindings
        .iter()
        .enumerate()
        .flat_map(|(index, binding)| {
            materialized_definitions(index, &binding.binding.materialized_model_cards)
        })
        .collect::<Vec<_>>();
    let mut groups = BTreeMap::<(ModelConsumerScope, String), Vec<&MaterializedDefinition>>::new();
    for definition in &definitions {
        groups
            .entry((definition.scope, definition.normalized_name.clone()))
            .or_default()
            .push(definition);
    }
    let mut losers = BTreeMap::<usize, Vec<&MaterializedDefinition>>::new();
    let mut unresolved = Vec::new();
    for ((scope, normalized_name), providers) in groups {
        let mut same_source = BTreeMap::<(String, String), Vec<&MaterializedDefinition>>::new();
        for definition in providers {
            let binding = &bindings[definition.binding_index];
            same_source
                .entry((
                    binding.provider_library.clone(),
                    binding.provider_source_digest.to_string(),
                ))
                .or_default()
                .push(definition);
        }
        for ((library, digest), definitions) in same_source {
            if definitions.len() < 2 {
                continue;
            }
            let binding_index = definitions[0].binding_index;
            let authorized = definitions
                .iter()
                .all(|definition| definition.binding_index == binding_index)
                && bindings[binding_index].allows_selected_section_override
                && bindings[binding_index].binding.section.is_some();
            if !authorized {
                unresolved.push(format!(
                    "{} '{}' is repeated inside authenticated provider '{}' at source {}",
                    scope.label(),
                    normalized_name,
                    library,
                    digest
                ));
                continue;
            }
            let winner_span = definitions
                .iter()
                .max_by_key(|definition| definition.name_span.start)
                .map(|definition| definition.name_span.clone())
                .expect("same-source override group is nonempty");
            for definition in definitions {
                if definition.name_span != winner_span {
                    losers
                        .entry(definition.binding_index)
                        .or_default()
                        .push(definition);
                }
            }
        }
    }
    if !unresolved.is_empty() {
        unresolved.truncate(8);
        return Err(format!(
            "Executable model namespace is contested and fails closed: {}. Repair the duplicate source before simulation.",
            unresolved.join("; ")
        ));
    }
    for (binding_index, mut definitions) in losers {
        definitions.sort_by_key(|definition| std::cmp::Reverse(definition.name_span.start));
        for definition in definitions {
            bindings[binding_index].binding.materialized_model_cards =
                mask_materialized_definition(
                    &bindings[binding_index].binding.materialized_model_cards,
                    definition,
                )?;
        }
    }
    Ok(())
}

/// Apply exact project-owned provider decisions to a frozen materialization.
/// Losing definitions are blanked before the engine parses the cards, so the
/// engine consumes one unambiguous namespace rather than relying on include
/// order or first-match lookup.
fn resolve_materialized_definition_namespace(
    mut bindings: Vec<MaterializedPlanBinding>,
    records: &[ModelResolutionRecord],
) -> Result<(Vec<CornerModelBinding>, Vec<ModelResolutionRecord>), String> {
    apply_project_section_overrides(&mut bindings)?;
    let definitions = bindings
        .iter()
        .enumerate()
        .flat_map(|(index, binding)| {
            materialized_definitions(index, &binding.binding.materialized_model_cards)
        })
        .collect::<Vec<_>>();
    let mut groups = BTreeMap::<(ModelConsumerScope, String), Vec<&MaterializedDefinition>>::new();
    for definition in &definitions {
        groups
            .entry((definition.scope, definition.normalized_name.clone()))
            .or_default()
            .push(definition);
    }
    let record_index = records
        .iter()
        .map(|record| {
            (
                (record.consumer_scope, record.normalized_name.clone()),
                record,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut losers = BTreeMap::<usize, Vec<&MaterializedDefinition>>::new();
    let mut applied = Vec::new();
    let mut unresolved = Vec::new();

    for ((scope, normalized_name), providers) in groups {
        if providers.len() < 2 {
            continue;
        }
        let provider_descriptions = providers
            .iter()
            .map(|definition| {
                let binding = &bindings[definition.binding_index];
                format!(
                    "{}/{} at {} (source {})",
                    binding.provider_library,
                    definition.exact_name,
                    binding.binding.source_label,
                    binding.provider_source_digest
                )
            })
            .collect::<Vec<_>>();
        if providers.iter().enumerate().any(|(index, left)| {
            providers.iter().skip(index + 1).any(|right| {
                let left = &bindings[left.binding_index];
                let right = &bindings[right.binding_index];
                left.provider_library == right.provider_library
                    && left.provider_source_digest == right.provider_source_digest
            })
        }) {
            unresolved.push(format!(
                "{} '{}' is repeated inside one authenticated provider: {}",
                scope.label(),
                normalized_name,
                provider_descriptions.join(", ")
            ));
            continue;
        }
        let Some(record) = record_index.get(&(scope, normalized_name.clone())) else {
            unresolved.push(format!(
                "{} '{}' from {}",
                scope.label(),
                normalized_name,
                provider_descriptions.join(", ")
            ));
            continue;
        };
        let winners = providers
            .iter()
            .filter(|definition| {
                let binding = &bindings[definition.binding_index];
                binding.provider_library == record.provider_library
                    && binding.provider_source_digest == record.provider_source_digest
                    && definition.exact_name == record.provider_definition
            })
            .copied()
            .collect::<Vec<_>>();
        if winners.len() != 1 {
            unresolved.push(format!(
                "{} '{}' has a stale provider decision for '{}/{}' at source {}; active providers are {}",
                scope.label(),
                normalized_name,
                record.provider_library,
                record.provider_definition,
                record.provider_source_digest,
                provider_descriptions.join(", ")
            ));
            continue;
        }
        let winner = winners[0];
        for provider in providers {
            if !std::ptr::eq(provider, winner) {
                losers
                    .entry(provider.binding_index)
                    .or_default()
                    .push(provider);
            }
        }
        applied.push((*record).clone());
    }

    if !unresolved.is_empty() {
        unresolved.truncate(8);
        return Err(format!(
            "Executable model namespace is contested and fails closed: {}. Publish an exact source-qualified provider decision or repair the duplicate source before simulation.",
            unresolved.join("; ")
        ));
    }

    for (binding_index, mut definitions) in losers {
        definitions.sort_by_key(|definition| std::cmp::Reverse(definition.name_span.start));
        for definition in definitions {
            bindings[binding_index].binding.materialized_model_cards =
                mask_materialized_definition(
                    &bindings[binding_index].binding.materialized_model_cards,
                    definition,
                )?;
        }
    }
    applied.sort_by_key(|left| left.key());
    applied.dedup_by(|left, right| left.key() == right.key());
    let bindings = bindings
        .into_iter()
        .filter(|binding| !binding.binding.materialized_model_cards.trim().is_empty())
        .map(|binding| {
            binding.binding.validate()?;
            Ok(binding.binding)
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((bindings, applied))
}

fn mask_materialized_definition(
    cards: &str,
    definition: &MaterializedDefinition,
) -> Result<String, String> {
    if definition.name_span.end > cards.len() {
        return Err(format!(
            "cannot apply provider decision for '{}' because its source span is invalid",
            definition.exact_name
        ));
    }
    let bytes = cards.as_bytes();
    let start = bytes[..definition.name_span.start]
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(0, |index| index + 1);
    let mut end = physical_line_end(bytes, start);
    match definition.scope {
        ModelConsumerScope::PrimitiveModel => {
            while end < bytes.len() {
                let next_end = physical_line_end(bytes, end);
                let line = std::str::from_utf8(&bytes[end..next_end]).map_err(|error| {
                    format!("materialized model source is not UTF-8 at continuation: {error}")
                })?;
                if line.trim_start().starts_with('+') {
                    end = next_end;
                } else {
                    break;
                }
            }
        }
        ModelConsumerScope::Subcircuit => {
            let mut cursor = start;
            let mut depth = 0_usize;
            let mut closed = false;
            while cursor < bytes.len() {
                let next = physical_line_end(bytes, cursor);
                let line = std::str::from_utf8(&bytes[cursor..next]).map_err(|error| {
                    format!("materialized subcircuit source is not UTF-8: {error}")
                })?;
                let head = line.split_whitespace().next().unwrap_or("");
                if head.eq_ignore_ascii_case(".subckt") {
                    depth += 1;
                } else if head.eq_ignore_ascii_case(".ends") {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        end = next;
                        closed = true;
                        break;
                    }
                }
                cursor = next;
            }
            if !closed {
                return Err(format!(
                    "cannot apply provider decision because subcircuit '{}' has no matching .ENDS",
                    definition.exact_name
                ));
            }
        }
    }
    let mut masked = bytes.to_vec();
    for byte in &mut masked[start..end] {
        if *byte != b'\n' && *byte != b'\r' {
            *byte = b' ';
        }
    }
    String::from_utf8(masked)
        .map_err(|error| format!("resolved model materialization is not UTF-8: {error}"))
}

fn physical_line_end(bytes: &[u8], start: usize) -> usize {
    bytes[start..]
        .iter()
        .position(|byte| *byte == b'\n')
        .map_or(bytes.len(), |offset| start + offset + 1)
}

impl SealedModelExecutionSources {
    pub(crate) fn model_library_veriloga_authority(
        &self,
    ) -> Result<Option<SealedModelLibraryVerilogAAuthority>, String> {
        let model_paths = self
            .model_library_source_paths
            .iter()
            .map(|path| portable_path_key(path))
            .collect::<HashSet<_>>();
        let mut sources = self
            .sources
            .iter()
            .filter(|(path, _)| model_paths.contains(&portable_path_key(path)))
            .cloned()
            .collect::<Vec<_>>();
        sources.sort_by(|left, right| left.0.cmp(&right.0));

        let mut roots = Vec::<SealedModelLibraryVerilogARoot>::new();
        for (owner, source) in &sources {
            let projected = rspice_core::library::adapt_spectre_model_library(owner, source)
                .map_err(|error| {
                    format!(
                        "Authenticated model source '{}':{} no longer satisfies the Spectre adapter: {}",
                        owner.display(),
                        error.line,
                        error.message
                    )
                })?;
            for line in projected.lines() {
                let Some(include) = rspice_core::netlist::parse_veriloga_source_directive(line)
                else {
                    continue;
                };
                let requested = rspice_core::netlist::normalize_source_path_literal(
                    &include.file_path.to_string_lossy(),
                )
                .map_err(|error| {
                    format!(
                        "Authenticated Verilog-A dependency in '{}' is invalid: {error}",
                        owner.display()
                    )
                })?;
                let matches = self
                    .edges
                    .iter()
                    .filter(|edge| {
                        portable_path_key(&edge.owner) == portable_path_key(owner)
                            && rspice_core::netlist::normalize_source_path_literal(
                                &edge.requested_path,
                            )
                            .is_ok_and(|edge_requested| edge_requested == requested)
                    })
                    .collect::<Vec<_>>();
                let [edge] = matches.as_slice() else {
                    return Err(format!(
                        "Authenticated Verilog-A dependency '{}' in '{}' has {} exact resolution edges; refresh or re-import the model library",
                        requested,
                        owner.display(),
                        matches.len()
                    ));
                };
                if !model_paths.contains(&portable_path_key(&edge.target)) {
                    return Err(format!(
                        "Authenticated Verilog-A dependency '{}' resolves outside the sealed model-library authority",
                        requested
                    ));
                }
                roots.push(SealedModelLibraryVerilogARoot {
                    path: edge.target.clone(),
                    netlist_alias: include.model_name,
                });
            }
        }
        roots.sort_by(|left, right| {
            left.path.cmp(&right.path).then_with(|| {
                left.netlist_alias
                    .as_deref()
                    .unwrap_or_default()
                    .cmp(right.netlist_alias.as_deref().unwrap_or_default())
            })
        });
        roots.dedup();
        for pair in roots.windows(2) {
            if portable_path_key(&pair[0].path) == portable_path_key(&pair[1].path)
                && pair[0].netlist_alias != pair[1].netlist_alias
            {
                return Err(format!(
                    "Verilog-A source '{}' is included with conflicting model aliases",
                    pair[1].path.display()
                ));
            }
        }
        if roots.is_empty() {
            return Ok(None);
        }

        let mut hasher = Sha256::new();
        hasher.update(b"rspice.sealed-model-library-veriloga/v1\0");
        for (path, source) in &sources {
            let path = portable_path_key(path);
            hasher.update((path.len() as u64).to_le_bytes());
            hasher.update(path.as_bytes());
            hasher.update((source.len() as u64).to_le_bytes());
            hasher.update(source.as_bytes());
        }
        for root in &roots {
            let path = portable_path_key(&root.path);
            hasher.update((path.len() as u64).to_le_bytes());
            hasher.update(path.as_bytes());
            if let Some(alias) = &root.netlist_alias {
                hasher.update((alias.len() as u64).to_le_bytes());
                hasher.update(alias.as_bytes());
            } else {
                hasher.update(0_u64.to_le_bytes());
            }
        }
        Ok(Some(SealedModelLibraryVerilogAAuthority {
            closure_digest: ContentDigest::from_bytes(hasher.finalize().into()),
            sources,
            roots,
        }))
    }

    /// Build a source bundle that adds one active root buffer to the exact
    /// authenticated model-library closure.
    ///
    /// Root include edges are accepted only when their portable lexical target
    /// names one retained source exactly. This deliberately does not guess by
    /// basename or consult a host search path: unresolved and ambiguous deck
    /// references fail closed in browser execution.
    pub(crate) fn with_pdk_model_sources(
        mut self,
        pdk: crate::state::pdk_config::SealedPdkModelSources,
    ) -> Result<Self, String> {
        if self.pdk_identity.is_some() {
            return Err("A sealed model snapshot already contains a signed PDK binding".to_owned());
        }

        let mut sources_by_key = self
            .sources
            .iter()
            .map(|(path, source)| (portable_path_key(path), (path.clone(), source.clone())))
            .collect::<BTreeMap<_, _>>();
        for (path, source) in pdk.sources {
            let key = portable_path_key(&path);
            match sources_by_key.entry(key) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert((path, source));
                }
                std::collections::btree_map::Entry::Occupied(entry) if entry.get().1 != source => {
                    return Err(format!(
                        "Signed PDK source '{}' conflicts with an authenticated model-library source",
                        path.display()
                    ));
                }
                std::collections::btree_map::Entry::Occupied(_) => {}
            }
        }

        let mut edges_by_key = self
            .edges
            .iter()
            .map(|edge| {
                (
                    (portable_path_key(&edge.owner), edge.requested_path.clone()),
                    edge.clone(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        for edge in pdk.edges {
            let key = (portable_path_key(&edge.owner), edge.requested_path.clone());
            match edges_by_key.entry(key) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(edge);
                }
                std::collections::btree_map::Entry::Occupied(entry)
                    if portable_path_key(&entry.get().target)
                        != portable_path_key(&edge.target) =>
                {
                    return Err(format!(
                        "Signed PDK and model libraries disagree on dependency '{}' in '{}'",
                        edge.requested_path,
                        edge.owner.display()
                    ));
                }
                std::collections::btree_map::Entry::Occupied(_) => {}
            }
        }

        self.sources = sources_by_key.into_values().collect();
        self.sources.sort_by(|left, right| left.0.cmp(&right.0));
        self.edges = edges_by_key.into_values().collect();
        self.edges.sort_by(|left, right| {
            left.owner
                .cmp(&right.owner)
                .then_with(|| left.requested_path.cmp(&right.requested_path))
                .then_with(|| left.target.cmp(&right.target))
        });
        self.bundle = rspice_core::netlist::SealedSourceBundle::try_new_with_edges(
            self.sources.clone(),
            self.edges.clone(),
        )
        .map_err(|error| format!("Failed to merge signed PDK model sources: {error}"))?;
        self.pdk_process_bindings = pdk.process_bindings;
        self.pdk_veriloga_artifacts = pdk.veriloga_artifacts;
        self.pdk_veriloga_bindings = pdk.veriloga_bindings;
        self.pdk_identity = Some((pdk.binding, pdk.archive_digest));
        Ok(self)
    }

    /// Exact signed package identity participating in this model snapshot.
    #[must_use]
    pub(crate) fn pdk_model_identity(&self) -> Option<(String, ContentDigest)> {
        self.pdk_identity.as_ref().map(|(binding, archive_digest)| {
            (
                format!(
                    "signed-pdk:{}@{}:manifest:{}",
                    binding.package_id, binding.revision, binding.manifest_digest
                ),
                *archive_digest,
            )
        })
    }

    #[must_use]
    pub(crate) fn pdk_veriloga_authority(
        &self,
    ) -> Option<(
        &crate::state::pdk_config::PdkTechnologyBinding,
        ContentDigest,
        &[crate::state::pdk_config::SealedPdkVerilogAArtifact],
        &[crate::state::pdk_config::SealedPdkVerilogABinding],
    )> {
        let (binding, archive_digest) = self.pdk_identity.as_ref()?;
        Some((
            binding,
            *archive_digest,
            &self.pdk_veriloga_artifacts,
            &self.pdk_veriloga_bindings,
        ))
    }

    pub(crate) fn bundle_for_root(
        &self,
        root_path: &Path,
        root_source: &str,
    ) -> Result<rspice_core::netlist::SealedSourceBundle, String> {
        if !super::is_portable_absolute_path(root_path) {
            return Err(format!(
                "Authenticated browser source root must have an absolute portable identity: {}",
                root_path.display()
            ));
        }

        let root_key = portable_path_key(root_path);
        let matching_roots = self
            .sources
            .iter()
            .filter(|(path, _)| portable_path_key(path) == root_key)
            .collect::<Vec<_>>();
        if matching_roots.len() > 1 {
            return Err(format!(
                "Authenticated model sources contain an ambiguous root identity '{}'",
                root_path.display()
            ));
        }

        let mut sources = self.sources.clone();
        let mut edges = self.edges.clone();
        if let Some((accepted_path, accepted_source)) = matching_roots.first().copied() {
            if accepted_source != root_source {
                return Err(format!(
                    "Active source '{}' conflicts with an authenticated model-source member",
                    root_path.display()
                ));
            }
            if accepted_path != root_path {
                for (path, _) in &mut sources {
                    if path == accepted_path {
                        *path = root_path.to_path_buf();
                    }
                }
                for edge in &mut edges {
                    if edge.owner == *accepted_path {
                        edge.owner = root_path.to_path_buf();
                    }
                    if edge.target == *accepted_path {
                        edge.target = root_path.to_path_buf();
                    }
                }
            }
        } else {
            sources.push((root_path.to_path_buf(), root_source.to_owned()));
        }

        let mut root_edge_keys = HashSet::new();
        for requested_path in root_external_source_paths(root_source) {
            let requested_path = rspice_core::netlist::normalize_source_path_literal(
                &requested_path,
            )
            .map_err(|error| {
                format!(
                    "Source '{}' has an invalid external dependency path: {error}",
                    root_path.display()
                )
            })?;
            if !root_edge_keys.insert(requested_path.clone()) {
                continue;
            }
            if edges.iter().any(|edge| {
                portable_path_key(&edge.owner) == root_key && edge.requested_path == requested_path
            }) {
                continue;
            }

            let target_key = portable_dependency_target_key(root_path, &requested_path)?;
            let candidates = self
                .sources
                .iter()
                .filter(|(path, _)| portable_path_key(path) == target_key)
                .map(|(path, _)| path)
                .collect::<Vec<_>>();
            let target = match candidates.as_slice() {
                [target] => (*target).clone(),
                [] => {
                    return Err(format!(
                        "Dependency '{}' referenced by '{}' is not present in the authenticated model-source closure",
                        requested_path,
                        root_path.display()
                    ));
                }
                _ => {
                    return Err(format!(
                        "Dependency '{}' referenced by '{}' has an ambiguous authenticated source identity",
                        requested_path,
                        root_path.display()
                    ));
                }
            };
            edges.push(rspice_core::netlist::SealedSourceEdge {
                owner: root_path.to_path_buf(),
                requested_path,
                target,
            });
        }

        rspice_core::netlist::SealedSourceBundle::try_new_with_edges(sources, edges)
            .map_err(|error| format!("Failed to authorize active source dependencies: {error}"))
    }

    /// Expand an active root through the authenticated model source closure
    /// without consulting a filesystem.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn expand_root_dependencies(
        &self,
        root_path: &Path,
        root_source: &str,
        abort: &dyn rspice_core::abort_signal::AbortSignal,
    ) -> Result<(String, Vec<rspice_core::netlist::ResolvedIncludeDependency>), String> {
        let bundle = self.bundle_for_root(root_path, root_source)?;
        let mut processor = rspice_core::netlist::IncludeProcessor::new_sealed(root_path, bundle);
        let expanded = processor
            .expand_content_with_abort(root_source, root_path, abort)
            .map_err(|error| {
                format!(
                    "Could not expand authenticated dependencies for '{}': {error}",
                    root_path.display()
                )
            })?;
        Ok((expanded, processor.resolved_dependencies().to_vec()))
    }

    /// Freeze the exact model namespace for the nominal/reference run.
    ///
    /// An explicit per-library selection is authoritative for that library.
    /// The reference process supplies the conventional fallback and the signed
    /// PDK process contract. Process sweeps deliberately use
    /// [`Self::corner_model_bindings`] instead, because every sweep point owns
    /// its explicit process independently of the nominal selection.
    pub fn reference_model_execution_plan(
        &self,
        process: crate::simulation::dialog::corner::ProcessCorner,
    ) -> Result<ModelExecutionPlan, String> {
        let corner_process = simulation_corner_process(process);
        let materialized = self.bindings_for_processes(&[corner_process], true)?;
        let (bindings, applied_resolutions) =
            resolve_materialized_definition_namespace(materialized, &self.resolution_records)?;

        let selected_library_corners = self
            .libraries
            .iter()
            .map(|library| {
                let selected = library.selected_corner.clone().or_else(|| {
                    library
                        .corners
                        .iter()
                        .find(|corner| {
                            corner
                                .name
                                .eq_ignore_ascii_case(corner_process.as_keyword())
                        })
                        .map(|corner| corner.name.clone())
                });
                (library.name.clone(), selected)
            })
            .collect::<Vec<_>>();

        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-execution-plan/v2\0");
        hasher.update(corner_process.as_keyword().as_bytes());
        for (library, corner) in &selected_library_corners {
            hash_plan_field(&mut hasher, library.as_bytes());
            hash_plan_field(&mut hasher, corner.as_deref().unwrap_or("").as_bytes());
        }
        for binding in &bindings {
            hash_plan_field(&mut hasher, binding.source_label.as_bytes());
            hash_plan_field(
                &mut hasher,
                binding.section.as_deref().unwrap_or("").as_bytes(),
            );
            hash_plan_field(&mut hasher, binding.materialized_model_cards.as_bytes());
        }
        for resolution in &applied_resolutions {
            let bytes = serde_json::to_vec(resolution).map_err(|error| {
                format!("Cannot digest applied model provider decision: {error}")
            })?;
            hash_plan_field(&mut hasher, &bytes);
        }
        let digest = ContentDigest::from_bytes(hasher.finalize().into());

        Ok(ModelExecutionPlan {
            reference_process: process,
            selected_library_corners,
            bindings,
            applied_resolutions,
            digest,
        })
    }

    /// Materialize the exact model cards for the nominal/reference process.
    pub fn reference_process_model_cards(
        &self,
        process: crate::simulation::dialog::corner::ProcessCorner,
    ) -> Result<Vec<String>, String> {
        self.reference_model_execution_plan(process)
            .map(|plan| plan.model_cards())
    }

    /// Materialize every model section required by a process-corner run from
    /// this same immutable snapshot.
    pub fn corner_model_bindings(
        &self,
        processes: &[CornerProcess],
    ) -> Result<Vec<CornerModelBinding>, String> {
        let mut resolved = Vec::new();
        for process in processes {
            let materialized = self.bindings_for_processes(&[*process], false)?;
            let (bindings, _) =
                resolve_materialized_definition_namespace(materialized, &self.resolution_records)?;
            resolved.extend(bindings);
        }
        Ok(resolved)
    }

    fn bindings_for_processes(
        &self,
        processes: &[CornerProcess],
        honor_nominal_selection: bool,
    ) -> Result<Vec<MaterializedPlanBinding>, String> {
        if self.libraries.is_empty() && self.pdk_process_bindings.is_empty() {
            if let Some(process) = processes
                .iter()
                .find(|process| **process != CornerProcess::TT)
            {
                return Err(format!(
                    "{} requires a PDK model library with an explicit process section",
                    process.as_keyword()
                ));
            }
            return Ok(Vec::new());
        }

        let mut bindings = Vec::new();
        for process in processes {
            for library in &self.libraries {
                let keyword = process.as_keyword();
                let requested_corner = honor_nominal_selection
                    .then_some(library.selected_corner.as_deref())
                    .flatten()
                    .unwrap_or(keyword);
                let corner = library
                    .corners
                    .iter()
                    .find(|corner| corner.name.eq_ignore_ascii_case(requested_corner))
                    .cloned();
                if corner.is_none()
                    && (library.selected_corner.is_some()
                        || *process != CornerProcess::TT
                        || !library.corners.is_empty())
                {
                    return Err(format!(
                        "Model library '{}' does not define selected corner '{}' for the {} reference process",
                        library.name, requested_corner, keyword
                    ));
                }
                match corner.as_ref() {
                    Some(corner) => {
                        for section in self.materialize_library_corner(library, corner)? {
                            let binding = MaterializedPlanBinding {
                                binding: CornerModelBinding {
                                    process: *process,
                                    source_label: section.source_label,
                                    section: Some(section.section),
                                    materialized_model_cards: section.materialized_model_cards,
                                },
                                provider_library: library.name.clone(),
                                provider_source_digest: library.source_digest,
                                allows_selected_section_override: library
                                    .allows_selected_section_override,
                            };
                            binding.binding.validate()?;
                            bindings.push(binding);
                        }
                    }
                    None => {
                        let mut processor = rspice_core::netlist::IncludeProcessor::new_sealed(
                            &library.root_path,
                            self.bundle.clone(),
                        );
                        let materialized_model_cards = processor
                            .process_sealed_root(&library.root_path, None)
                            .map_err(|error| {
                                format!(
                                    "Failed to materialize sealed model library '{}' from '{}': {error}",
                                    library.name,
                                    library.root_path.display()
                                )
                            })?;
                        let binding = MaterializedPlanBinding {
                            binding: CornerModelBinding {
                                process: *process,
                                source_label: library.root_path.display().to_string(),
                                section: None,
                                materialized_model_cards,
                            },
                            provider_library: library.name.clone(),
                            provider_source_digest: library.source_digest,
                            allows_selected_section_override: false,
                        };
                        binding.binding.validate()?;
                        bindings.push(binding);
                    }
                }
            }

            if !self.pdk_process_bindings.is_empty() {
                let pdk_process = pdk_model_process(*process);
                let selected = self
                    .pdk_process_bindings
                    .iter()
                    .filter(|binding| binding.process == pdk_process)
                    .collect::<Vec<_>>();
                if selected.is_empty() {
                    let package = self
                        .pdk_identity
                        .as_ref()
                        .map(|(binding, _)| format!("{} {}", binding.package_id, binding.revision))
                        .unwrap_or_else(|| "signed PDK".to_owned());
                    return Err(format!(
                        "{package} does not supply an explicit {} model-source contract",
                        process.as_keyword()
                    ));
                }
                for source in selected {
                    let mut processor = rspice_core::netlist::IncludeProcessor::new_sealed(
                        &source.root_path,
                        self.bundle.clone(),
                    );
                    let materialized_model_cards = processor
                        .process_sealed_root(&source.root_path, source.section.as_deref())
                        .map_err(|error| {
                            format!(
                                "Failed to materialize signed PDK {} source '{}' from '{}': {error}",
                                process.as_keyword(),
                                source.source_id,
                                source.artifact_path
                            )
                        })?;
                    let package = self
                        .pdk_identity
                        .as_ref()
                        .map(|(binding, digest)| {
                            format!(
                                "{} {} / {} / {} / artifact {} / archive {}",
                                binding.package_id,
                                binding.revision,
                                source.domain.label(),
                                source.source_id,
                                source.artifact_digest,
                                digest
                            )
                        })
                        .unwrap_or_else(|| source.source_id.clone());
                    let binding = MaterializedPlanBinding {
                        binding: CornerModelBinding {
                            process: *process,
                            source_label: package,
                            section: source.section.clone(),
                            materialized_model_cards,
                        },
                        provider_library: format!("signed-pdk:{}", source.source_id),
                        provider_source_digest: source.artifact_digest,
                        allows_selected_section_override: false,
                    };
                    binding.binding.validate()?;
                    bindings.push(binding);
                }
            }
        }
        Ok(bindings)
    }

    fn materialize_library_corner(
        &self,
        library: &SealedExecutionLibrary,
        corner: &ProcessCorner,
    ) -> Result<Vec<MaterializedCornerSection>, String> {
        if let Err(errors) = corner.validate_contract() {
            return Err(format!(
                "Model library '{}' corner '{}' has an invalid section contract: {}",
                library.name,
                corner.name,
                errors.join("; ")
            ));
        }
        let source_path = corner
            .file_path
            .as_deref()
            .unwrap_or(library.root_path.as_path());
        let bindings = corner.effective_section_bindings();
        if bindings.is_empty() {
            return Err(format!(
                "Model library '{}' corner '{}' has no executable section binding",
                library.name, corner.name
            ));
        }

        let mut domains_by_section =
            BTreeMap::<(PathBuf, String), Vec<super::CornerSectionDomain>>::new();
        for binding in bindings {
            domains_by_section
                .entry((source_path.to_path_buf(), binding.section))
                .or_default()
                .push(binding.domain);
        }

        let mut sections = Vec::with_capacity(domains_by_section.len());
        for ((path, section), mut domains) in domains_by_section {
            domains.sort();
            domains.dedup();
            let mut processor =
                rspice_core::netlist::IncludeProcessor::new_sealed(&path, self.bundle.clone());
            let materialized_model_cards = processor
                .process_sealed_root(&path, Some(&section))
                .map_err(|error| {
                    format!(
                        "Failed to materialize {} section '{}' for model library '{}' corner '{}' from '{}': {error}",
                        domains
                            .iter()
                            .map(|domain| domain.label())
                            .collect::<Vec<_>>()
                            .join(" + "),
                        section,
                        library.name,
                        corner.name,
                        path.display()
                    )
                })?;
            sections.push(MaterializedCornerSection {
                source_label: format!(
                    "{} [{}] ({})",
                    path.display(),
                    section,
                    domains
                        .iter()
                        .map(|domain| domain.label())
                        .collect::<Vec<_>>()
                        .join(" + ")
                ),
                section,
                materialized_model_cards,
            });
        }
        Ok(sections)
    }
}

fn root_external_source_paths(source: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut inline_library_depth = 0usize;
    for line in source.lines() {
        let directive = line.split_whitespace().next().unwrap_or_default();
        if directive.eq_ignore_ascii_case(".endl") {
            inline_library_depth = inline_library_depth.saturating_sub(1);
            continue;
        }
        if let Some((path, section)) = rspice_core::netlist::parse_lib_directive(line) {
            if section.is_none() {
                inline_library_depth = inline_library_depth.saturating_add(1);
            } else if inline_library_depth == 0 {
                paths.push(path);
            }
            continue;
        }
        if inline_library_depth == 0
            && let Some(path) = rspice_core::netlist::parse_include_directive(line)
        {
            paths.push(path);
        }
    }
    paths
}

fn portable_dependency_target_key(
    root_path: &Path,
    requested_path: &str,
) -> Result<String, String> {
    let requested = normalize_portable_path_text(requested_path)?;
    if is_portable_absolute_text(&requested) {
        return Ok(portable_text_key(&requested));
    }
    let root = normalize_portable_path_text(&root_path.to_string_lossy())?;
    let mut parent = root.rsplit_once('/').map_or("", |(parent, _)| parent);
    if parent.is_empty() && root.starts_with('/') {
        parent = "/";
    }
    let joined = if parent.is_empty() {
        requested
    } else if parent == "/" {
        format!("/{requested}")
    } else {
        format!("{parent}/{requested}")
    };
    normalize_portable_path_text(&joined).map(|path| portable_text_key(&path))
}

fn portable_path_key(path: &Path) -> String {
    normalize_portable_path_text(&path.to_string_lossy())
        .map(|path| portable_text_key(&path))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

fn portable_text_key(path: &str) -> String {
    let mut key = path.to_owned();
    if is_windows_absolute_text(path) {
        key.make_ascii_lowercase();
    }
    key
}

fn is_portable_absolute_text(path: &str) -> bool {
    path.starts_with('/') || is_windows_absolute_text(path)
}

fn is_windows_absolute_text(path: &str) -> bool {
    let candidate = path
        .strip_prefix("//?/")
        .or_else(|| path.strip_prefix("//./"))
        .unwrap_or(path);
    let candidate = candidate
        .strip_prefix("UNC/")
        .or_else(|| candidate.strip_prefix("unc/"))
        .unwrap_or(candidate);
    let bytes = candidate.as_bytes();
    (bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/')
        || (path.starts_with("//")
            && candidate
                .split('/')
                .filter(|component| !component.is_empty())
                .take(2)
                .count()
                == 2)
}

fn normalize_portable_path_text(path: &str) -> Result<String, String> {
    let mut path = path.trim().replace('\\', "/");
    if path.is_empty() || path.chars().any(char::is_control) {
        return Err("source path is empty or contains a control character".to_owned());
    }
    if let Some(unprefixed) = path.strip_prefix("//?/") {
        path = if let Some(unc) = unprefixed.strip_prefix("UNC/") {
            format!("//{unc}")
        } else {
            unprefixed.to_owned()
        };
    }

    let prefix_len = if path.starts_with("//") {
        2
    } else if path.starts_with('/') {
        1
    } else if is_windows_absolute_text(&path) {
        3
    } else {
        0
    };
    let prefix = &path[..prefix_len];
    let mut components = Vec::new();
    for component in path[prefix_len..].split('/') {
        match component {
            "" | "." => {}
            ".." if components.pop().is_some() => {}
            ".." if prefix_len == 0 => components.push(component),
            ".." => {
                return Err(format!("absolute source path escapes its root: {path}"));
            }
            _ => components.push(component),
        }
    }
    let body = components.join("/");
    Ok(match prefix {
        "//" => format!("//{body}"),
        "/" => format!("/{body}"),
        _ if prefix_len == 3 => format!("{prefix}{body}"),
        _ => body,
    })
}

/// Manager for all model libraries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelLibraryManager {
    /// All libraries
    libraries: HashMap<String, ModelLibrary>,
    /// Currently selected library
    pub selected_library: Option<String>,
    /// Search filter
    pub filter_text: String,
    /// Filter by model type
    pub filter_type: Option<ModelType>,
    /// Durable project decisions for contested executable definitions. The
    /// map key is the canonical `scope:name` identity repeated by each value.
    #[serde(default)]
    resolution_records: BTreeMap<String, ModelResolutionRecord>,
    #[serde(default)]
    validation_receipt: Option<ModelValidationReceipt>,
    /// Index over the shipped model packs, when one was found on disk.
    ///
    /// Held rather than loaded: the packs carry around 199,000 definitions, so
    /// materializing them as `DeviceModel`s would cost far more memory than the
    /// catalogue view needs. Queries stream the on-disk index instead.
    ///
    /// Not serialized. It is a view of what is installed on this machine, so it
    /// is rediscovered on load rather than restored from a project file that may
    /// have been written elsewhere.
    #[serde(skip)]
    spice_packs: Option<Arc<SpiceLibraryIndex>>,
}

/// One definition found in the shipped packs rather than in a loaded library.
///
/// Deliberately not a [`DeviceModel`]: nothing here has been parsed, and
/// presenting an unparsed catalogue row as a loaded model would overstate what
/// the application knows about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackModelHit {
    /// Definition name as written in the source.
    pub name: String,
    /// `model` or `subckt`.
    pub kind: String,
    /// Canonical device class, such as `diode` or `mosfet-n`.
    pub device: String,
    /// Owning pack identifier.
    pub pack: String,
    /// Human-readable pack title.
    pub pack_name: String,
    /// Absolute path to the defining file, when the pack is known.
    pub source: Option<PathBuf>,
    /// 1-based line of the definition.
    pub line: usize,
    /// Whether RSpice has established the right to redistribute the pack.
    pub redistributable: bool,
    /// The individual source file is excluded from redistribution even when
    /// other files in the same pack are shippable.
    pub restricted: bool,
}

impl ModelLibraryManager {
    #[must_use]
    pub fn model_validation_receipt(&self) -> Option<&ModelValidationReceipt> {
        self.validation_receipt.as_ref()
    }

    pub(crate) fn invalidate_model_validation_receipt(&mut self) {
        self.validation_receipt = None;
    }

    pub(crate) fn restore_model_validation_receipt(
        &mut self,
        receipt: Option<ModelValidationReceipt>,
    ) -> Result<(), String> {
        if let Some(receipt) = receipt.as_ref() {
            receipt.verify()?;
        }
        self.validation_receipt = receipt;
        Ok(())
    }

    pub(crate) fn issue_model_validation_receipt(
        &mut self,
        project_revision: ObjectRevision,
        plan_digest: ContentDigest,
        pdk_archive_digest: Option<ContentDigest>,
        execution_schema_version: u32,
        findings: Vec<ModelValidationFinding>,
    ) -> Result<ModelValidationReceipt, String> {
        let (source_count, source_closure_digest) = self.model_validation_source_identity();
        let receipt = ModelValidationReceipt::issue(
            project_revision,
            plan_digest,
            self.execution_catalog_digest(),
            source_count,
            source_closure_digest,
            pdk_archive_digest,
            execution_schema_version,
            findings,
        )?;
        self.validation_receipt = Some(receipt.clone());
        Ok(receipt)
    }

    pub(crate) fn validate_model_validation_receipt(
        &self,
        project_revision: ObjectRevision,
        plan_digest: ContentDigest,
        pdk_archive_digest: Option<ContentDigest>,
        execution_schema_version: u32,
    ) -> Result<&ModelValidationReceipt, String> {
        let receipt = self.validation_receipt.as_ref().ok_or_else(|| {
            "No durable model-validation receipt exists for this project revision.".to_owned()
        })?;
        receipt.verify()?;
        if receipt.project_revision != project_revision {
            return Err(
                "Model-validation receipt is stale after a project revision change.".to_owned(),
            );
        }
        if receipt.model_execution_plan_digest != plan_digest {
            return Err(
                "Model-validation receipt is stale after the execution plan changed.".to_owned(),
            );
        }
        if receipt.execution_catalog_digest != self.execution_catalog_digest() {
            return Err(
                "Model-validation receipt is stale after the model catalog changed.".to_owned(),
            );
        }
        if receipt.pdk_archive_digest != pdk_archive_digest {
            return Err(
                "Model-validation receipt is stale after the signed PDK changed.".to_owned(),
            );
        }
        if receipt.execution_schema_version != execution_schema_version
            || receipt.engine_version != env!("CARGO_PKG_VERSION")
            || receipt.platform != model_validation_platform()
        {
            return Err(
                "Model-validation receipt was produced by a different engine, schema, or platform."
                    .to_owned(),
            );
        }
        let (source_count, source_closure_digest) = self.model_validation_source_identity();
        if receipt.source_count != source_count
            || receipt.source_closure_digest != source_closure_digest
        {
            return Err(
                "Model-validation receipt is stale after source digests changed.".to_owned(),
            );
        }
        Ok(receipt)
    }

    fn model_validation_source_identity(&self) -> (u64, ContentDigest) {
        let mut identities = self
            .libraries_sorted()
            .into_iter()
            .flat_map(|library| {
                library
                    .source_closure
                    .iter()
                    .map(move |source| (library.name.clone(), source.digest.to_string()))
            })
            .collect::<Vec<_>>();
        identities.sort();
        let source_count = identities.len() as u64;
        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-validation-source-closure/v1\0");
        for (library, digest) in identities {
            hash_plan_field(&mut hasher, library.as_bytes());
            hash_plan_field(&mut hasher, digest.as_bytes());
        }
        (
            source_count,
            ContentDigest::from_bytes(hasher.finalize().into()),
        )
    }

    #[must_use]
    pub fn model_resolution_record(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Option<&ModelResolutionRecord> {
        let normalized = definition.trim().to_ascii_lowercase();
        self.resolution_records
            .get(&resolution_record_key(scope, &normalized))
    }

    pub(crate) fn restore_model_resolution_records(
        &mut self,
        records: Vec<ModelResolutionRecord>,
    ) -> Result<(), String> {
        let mut restored = BTreeMap::new();
        for record in records {
            record.validate()?;
            let key = record.key();
            if restored.insert(key.clone(), record).is_some() {
                return Err(format!("model-resolution record '{key}' is repeated"));
            }
        }
        self.resolution_records = restored;
        self.validate_model_resolution_records_against_catalog()
    }

    #[must_use]
    pub(crate) fn owned_model_resolution_records(&self) -> Vec<ModelResolutionRecord> {
        self.resolution_records.values().cloned().collect()
    }

    pub(crate) fn definition_providers(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Vec<ModelDefinitionProvider> {
        let normalized = definition.trim().to_ascii_lowercase();
        let mut providers = Vec::new();
        for library in self.libraries_sorted() {
            let active_sections = library.active_section_names();
            let names = match scope {
                ModelConsumerScope::PrimitiveModel => library
                    .models
                    .values()
                    .map(|model| model.name.as_str())
                    .collect::<Vec<_>>(),
                ModelConsumerScope::Subcircuit => library
                    .subcircuits
                    .values()
                    .filter(|subcircuit| {
                        subcircuit.section.as_deref().is_none_or(|section| {
                            active_sections
                                .iter()
                                .any(|active| active.eq_ignore_ascii_case(section))
                        })
                    })
                    .map(|subcircuit| subcircuit.name.as_str())
                    .collect::<Vec<_>>(),
            };
            for exact_name in names
                .into_iter()
                .filter(|name| name.to_ascii_lowercase() == normalized)
            {
                providers.push(ModelDefinitionProvider {
                    library: library.name.clone(),
                    definition: exact_name.to_owned(),
                    source_digest: model_library_source_digest(library),
                });
            }
        }
        providers.sort_by(|left, right| {
            left.library
                .cmp(&right.library)
                .then_with(|| left.definition.cmp(&right.definition))
                .then_with(|| {
                    left.source_digest
                        .to_string()
                        .cmp(&right.source_digest.to_string())
                })
        });
        providers
    }

    /// Resolve the one provider the flat executable SPICE namespace will use.
    ///
    /// Component properties may retain a library name for provenance, but that
    /// metadata is not an independent namespace selector. Every UI surface must
    /// consult this method so Properties, catalog binding, and the sealed run
    /// plan agree with the same project-global provider decision.
    pub(crate) fn effective_definition_provider(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Result<Option<ModelDefinitionProvider>, String> {
        let providers = self.definition_providers(scope, definition);
        match providers.as_slice() {
            [] => Ok(None),
            [provider] => Ok(Some(provider.clone())),
            _ => {
                let normalized_name = definition.trim().to_ascii_lowercase();
                let Some(record) = self.model_resolution_record(scope, &normalized_name) else {
                    return Err(format!(
                        "{} '{}' has {} executable providers ({}); resolve the project-global provider before binding or editing an instance",
                        scope.label(),
                        definition.trim(),
                        providers.len(),
                        providers
                            .iter()
                            .map(|provider| provider.library.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                };
                let winners = providers
                    .into_iter()
                    .filter(|provider| {
                        provider.library == record.provider_library
                            && provider.definition == record.provider_definition
                            && provider.source_digest == record.provider_source_digest
                    })
                    .collect::<Vec<_>>();
                match winners.as_slice() {
                    [provider] => Ok(Some(provider.clone())),
                    [] => Err(format!(
                        "project-global provider decision for {} '{}' no longer matches an authenticated catalog definition",
                        scope.label(),
                        definition.trim()
                    )),
                    _ => Err(format!(
                        "{} '{}' is repeated inside resolved provider '{}'; repair that source before binding an instance",
                        scope.label(),
                        definition.trim(),
                        record.provider_library
                    )),
                }
            }
        }
    }

    pub fn resolve_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
        provider_library: &str,
        audit_reason: &str,
    ) -> Result<ModelResolutionRecord, String> {
        let normalized_name = definition.trim().to_ascii_lowercase();
        let providers = self.definition_providers(scope, &normalized_name);
        if providers.len() < 2 {
            return Err(format!(
                "{} definition '{}' is not contested by multiple authenticated providers",
                scope.label(),
                definition.trim()
            ));
        }
        let provider = providers
            .iter()
            .find(|provider| provider.library == provider_library)
            .ok_or_else(|| {
                format!(
                    "'{provider_library}' is not an exact provider of contested {} '{}'",
                    scope.label(),
                    normalized_name
                )
            })?;
        if providers.iter().any(|candidate| {
            candidate != provider
                && candidate.library.eq_ignore_ascii_case(&provider.library)
                && candidate.source_digest == provider.source_digest
        }) {
            return Err(format!(
                "{} '{}' is defined more than once inside provider '{}'; repair the source because a provider decision cannot distinguish same-source duplicates",
                scope.label(),
                normalized_name,
                provider.library
            ));
        }
        let created_at_unix_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| format!("system clock cannot timestamp provider decision: {error}"))?
            .as_millis()
            .try_into()
            .map_err(|_| "provider-decision timestamp exceeds the supported range".to_owned())?;
        let record = ModelResolutionRecord {
            schema_version: MODEL_RESOLUTION_RECORD_SCHEMA_VERSION,
            consumer_scope: scope,
            normalized_name,
            provider_library: provider.library.clone(),
            provider_definition: provider.definition.clone(),
            provider_source_digest: provider.source_digest,
            audit_reason: audit_reason.to_owned(),
            created_at_unix_ms,
        };
        record.validate()?;
        self.resolution_records.insert(record.key(), record.clone());
        Ok(record)
    }

    pub fn clear_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> bool {
        let normalized = definition.trim().to_ascii_lowercase();
        self.resolution_records
            .remove(&resolution_record_key(scope, &normalized))
            .is_some()
    }

    fn validate_model_resolution_records_against_catalog(&self) -> Result<(), String> {
        for (key, record) in &self.resolution_records {
            record.validate()?;
            if record.key() != *key {
                return Err(format!(
                    "model-resolution map key '{key}' does not match its record identity '{}'",
                    record.key()
                ));
            }
            let provider = self.get_library(&record.provider_library).ok_or_else(|| {
                format!(
                    "provider decision for {} '{}' is stale because library '{}' was removed",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library
                )
            })?;
            if model_library_source_digest(provider) != record.provider_source_digest {
                return Err(format!(
                    "provider decision for {} '{}' is stale because source '{}' changed digest",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library
                ));
            }
            let exact_provider_exists = self
                .definition_providers(record.consumer_scope, &record.normalized_name)
                .into_iter()
                .any(|candidate| {
                    candidate.library == record.provider_library
                        && candidate.definition == record.provider_definition
                        && candidate.source_digest == record.provider_source_digest
                });
            if !exact_provider_exists {
                return Err(format!(
                    "provider decision for {} '{}' is stale because exact definition '{}/{}' is no longer available",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library,
                    record.provider_definition
                ));
            }
        }
        Ok(())
    }

    /// Convert a parsed card that a `.lib` section owns, recording the section
    /// as execution provenance. Cards at file scope use
    /// [`Self::convert_parsed_model`], which leaves the section unset.
    pub(crate) fn convert_parsed_model_in_section(
        model: &rspice_core::library::ParsedModel,
        file_path: &Path,
        section: Option<&str>,
    ) -> DeviceModel {
        let mut converted = Self::convert_parsed_model(model, file_path);
        converted.section = section.map(str::to_owned);
        converted
    }

    /// Project one parsed subcircuit onto its callable interface. A subcircuit
    /// carries its own source file when it was reached through an include, so
    /// that path wins over the root being scanned.
    fn insert_parsed_subcircuits(
        library: &mut ModelLibrary,
        parsed: &[rspice_core::library::ParsedSubcircuit],
        file_path: &Path,
        section: Option<&str>,
    ) -> Result<(), String> {
        for subcircuit in parsed {
            let interface = Self::convert_parsed_subcircuit(subcircuit, file_path, section);
            let key = subcircuit_interface_key(interface.section.as_deref(), &interface.name);
            if let Some(existing) = library
                .subcircuits
                .keys()
                .find(|existing| existing.eq_ignore_ascii_case(&key))
            {
                return Err(format!(
                    "Subcircuit '{}' is defined more than once in the same library section (first identity '{}')",
                    interface.name, existing
                ));
            }
            library.subcircuits.insert(key, interface);
        }
        Ok(())
    }

    pub(crate) fn convert_parsed_subcircuit(
        subcircuit: &rspice_core::library::ParsedSubcircuit,
        file_path: &Path,
        section: Option<&str>,
    ) -> super::ModelSubcircuitInterface {
        super::ModelSubcircuitInterface {
            name: subcircuit.name.clone(),
            ports: subcircuit.pins.clone(),
            parameter_defaults: subcircuit
                .parameter_defaults
                .iter()
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
            description: subcircuit.description.clone(),
            file_path: Some(
                subcircuit
                    .source_file
                    .as_deref()
                    .unwrap_or(file_path)
                    .to_path_buf(),
            ),
            source_line: subcircuit.source_line,
            section: section.map(str::to_owned),
        }
    }

    /// Stable identity of the persisted model catalogue relevant to source
    /// preparation. Browser filters, selection, shipped-pack indexes, and
    /// audit ledgers are deliberately excluded.
    pub(crate) fn execution_catalog_digest(&self) -> ContentDigest {
        let mut libraries = self.libraries.values().collect::<Vec<_>>();
        libraries.sort_by(|left, right| left.name.cmp(&right.name));
        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-execution-catalog/v4\0");
        for library in libraries {
            // A library owns several `HashMap` fields, so serializing it
            // directly emits their entries in per-instance iteration order and
            // yields a different digest for identical content. Route through
            // `serde_json::Value`, whose objects are key-sorted maps, so the
            // catalogue identity depends only on the content itself. A prepared
            // run compares this digest before dispatch; an order-dependent one
            // expires authorized runs at random.
            let bytes = serde_json::to_value(library)
                .and_then(|canonical| serde_json::to_vec(&canonical))
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        for record in self.resolution_records.values() {
            let bytes = serde_json::to_vec(record)
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }
    /// Create a new manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a library
    pub fn add_library(&mut self, library: ModelLibrary) {
        self.libraries.insert(library.name.clone(), library);
    }

    /// Remove a library
    pub fn remove_library(&mut self, name: &str) -> Option<ModelLibrary> {
        self.libraries.remove(name)
    }

    /// Get a library
    pub fn get_library(&self, name: &str) -> Option<&ModelLibrary> {
        self.libraries.get(name)
    }

    /// Get mutable library
    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut ModelLibrary> {
        self.libraries.get_mut(name)
    }

    /// Select a library
    pub fn select_library(&mut self, name: &str) {
        if self.libraries.contains_key(name) {
            self.selected_library = Some(name.to_string());
        }
    }

    /// Get current library
    pub fn current_library(&self) -> Option<&ModelLibrary> {
        self.selected_library
            .as_ref()
            .and_then(|name| self.libraries.get(name))
    }

    /// Canonical identities of every project-owned model definition admitted
    /// to the executable model closure.
    ///
    /// Prepared simulation receipts retain these typed identities so later
    /// engineering evidence can prove that an exact model revision was present
    /// in the immutable run snapshot instead of trusting a display name or a
    /// user-entered digest.
    pub(crate) fn project_model_definition_identities(
        &self,
    ) -> Result<Vec<(ModelSourceId, String, ObjectRevision, ContentDigest)>, String> {
        let mut identities = Vec::new();
        for library in self
            .libraries
            .values()
            .filter(|library| library.source_authority.is_project_owned())
        {
            let ModelSourceAuthority::ProjectOwned {
                source_id,
                revision: library_revision,
                ..
            } = library.source_authority
            else {
                continue;
            };
            for (model_name, model) in &library.models {
                let metadata = library
                    .model_definition_metadata
                    .get(model_name)
                    .cloned()
                    .ok_or_else(|| {
                        format!(
                            "Project model '{}/{}' has no typed definition metadata",
                            library.name, model_name
                        )
                    })?;
                let definition = ProjectModelRevisionDefinition::new(
                    ProjectModelDefinition::from_device_model(model),
                    metadata,
                );
                let canonical = definition.canonical_source().map_err(|error| {
                    format!(
                        "Project model '{}/{}' cannot be authenticated for execution: {error}",
                        library.name, model_name
                    )
                })?;
                let definition_identity =
                    definition.project_source_identity().map_err(|error| {
                        format!(
                            "Project model '{}/{}' has invalid source identity: {error}",
                            library.name, model_name
                        )
                    })?;
                let revision = definition_identity
                    .as_ref()
                    .map_or(library_revision, |identity| identity.revision);
                let digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
                if let Some(identity) = definition_identity
                    && (identity.source_id != source_id || identity.content_digest != digest)
                {
                    return Err(format!(
                        "Project model '{}/{}' definition identity does not match its retained source",
                        library.name, model_name
                    ));
                }
                identities.push((source_id, model_name.clone(), revision, digest));
            }
        }
        identities.sort_by(|left, right| {
            left.0
                .as_uuid()
                .cmp(&right.0.as_uuid())
                .then_with(|| {
                    left.1
                        .to_ascii_lowercase()
                        .cmp(&right.1.to_ascii_lowercase())
                })
                .then_with(|| left.2.cmp(&right.2))
                .then_with(|| left.3.cmp(&right.3))
        });
        identities.dedup();
        Ok(identities)
    }

    /// Search for models by name
    pub fn search_models(&self, pattern: &str) -> Vec<(&ModelLibrary, &DeviceModel)> {
        let pattern_lower = pattern.to_lowercase();
        let mut results = Vec::new();

        for lib in self.libraries.values() {
            for model in lib.models.values() {
                if model.name.to_lowercase().contains(&pattern_lower)
                    || model.description.to_lowercase().contains(&pattern_lower)
                {
                    if let Some(filter_type) = self.filter_type {
                        if model.model_type == filter_type {
                            results.push((lib, model));
                        }
                    } else {
                        results.push((lib, model));
                    }
                }
            }
        }

        results
    }

    /// Locate the shipped model packs, if this installation has them.
    ///
    /// Absence is normal, not a failure: the built-in library is compiled into
    /// the binary, and the browser build has no filesystem to find packs on.
    pub fn discover_spice_packs(&mut self) {
        self.spice_packs = match SpiceLibraryIndex::discover() {
            Ok(index) => index.map(Arc::new),
            Err(error) => {
                log::warn!("shipped SPICE model packs could not be read: {error}");
                None
            }
        };
    }

    /// The discovered pack index, when one is present.
    pub fn spice_packs(&self) -> Option<&SpiceLibraryIndex> {
        self.spice_packs.as_deref()
    }

    /// Selectable parts across the shipped packs, or zero when none were found.
    ///
    /// The addressable count rather than the raw definition total: two thirds
    /// of the definitions in the catalog are helper cards inside macromodel
    /// bodies, and offering those as parts would be a promise the netlist
    /// cannot keep.
    pub fn pack_definition_count(&self) -> usize {
        self.spice_packs
            .as_ref()
            .map_or(0, |index| index.part_count())
    }

    /// Search the shipped packs for definitions whose name contains `query`.
    ///
    /// Bounded by `limit` because a short query matches tens of thousands of
    /// rows; the catalogue view is a browser, not a dump. An empty query
    /// returns nothing rather than everything, so opening the tab does not
    /// stream a 16 MB index off disk.
    #[cfg(test)]
    pub fn search_pack_models(&self, query: &str, limit: usize) -> Vec<PackModelHit> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }
        let Some(index) = self.spice_packs.as_ref() else {
            return Vec::new();
        };

        let entries = match index.search_parts(trimmed, limit) {
            Ok(entries) => entries,
            Err(error) => {
                log::warn!("shipped SPICE model catalog could not be searched: {error}");
                return Vec::new();
            }
        };

        entries
            .into_iter()
            .map(|entry| {
                let pack = index.pack(&entry.pack);
                PackModelHit {
                    name: entry.name.clone(),
                    kind: entry.kind.clone(),
                    device: entry.device.clone(),
                    pack_name: pack.map_or_else(|| entry.pack.clone(), |p| p.name.clone()),
                    redistributable: pack.is_some_and(|p| p.redistributable),
                    source: entry.source_path(index),
                    line: entry.line,
                    pack: entry.pack,
                    restricted: entry.restricted,
                }
            })
            .collect()
    }

    /// Browse a bounded first page or a canonical device-class projection of
    /// the shipped corpus without loading every catalog row into memory.
    pub fn browse_pack_models(
        &self,
        query: &str,
        pack_filter: Option<&str>,
        device_filters: &[&str],
        offset: usize,
        limit: usize,
    ) -> Result<(usize, Vec<PackModelHit>), String> {
        let Some(index) = self.spice_packs.as_ref() else {
            return Ok((0, Vec::new()));
        };
        let (total, entries) = index
            .query_parts(query, pack_filter, device_filters, offset, limit)
            .map_err(|error| format!("Shipped model catalog could not be read: {error}"))?;

        let hits = entries
            .into_iter()
            .map(|entry| {
                let pack = index.pack(&entry.pack);
                PackModelHit {
                    name: entry.name.clone(),
                    kind: entry.kind.clone(),
                    device: entry.device.clone(),
                    pack_name: pack.map_or_else(|| entry.pack.clone(), |p| p.name.clone()),
                    redistributable: pack.is_some_and(|pack| pack.redistributable),
                    source: entry.source_path(index),
                    line: entry.line,
                    pack: entry.pack,
                    restricted: entry.restricted,
                }
            })
            .collect();
        Ok((total, hits))
    }

    /// Whether a pack's executable entry bytes are available to attach now.
    ///
    /// Browser builds embed discovery metadata only. They must not enable an
    /// attach action whose synthetic catalog path can never be opened.
    #[must_use]
    pub fn spice_pack_entry_available(&self, pack_id: &str) -> bool {
        self.spice_packs.as_ref().is_some_and(|index| {
            index.source_files_available()
                && index
                    .pack(pack_id)
                    .and_then(|pack| pack.entry_path(index.root()))
                    .is_some_and(|entry| entry.is_file())
        })
    }

    /// Load a redistributable pack's declared entry as one authenticated model
    /// library. The caller publishes the resulting manager candidate at the
    /// project transaction boundary.
    pub fn attach_spice_pack(&mut self, pack_id: &str) -> Result<String, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model corpus is not installed on this machine.".to_owned()
        })?;
        let pack = index
            .pack(pack_id)
            .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
        if !pack.redistributable {
            return Err(format!(
                "Model pack '{}' cannot be embedded in a project because its redistribution grant is not established.",
                pack.name
            ));
        }
        let entry = pack.entry_path(index.root()).ok_or_else(|| {
            format!(
                "Model pack '{}' has no declared entry file to attach.",
                pack.name
            )
        })?;
        let library_name = self.load_catalog_source_without_collision(&entry)?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    fn retain_pack_library(&mut self, library_name: &str, pack_id: &str) -> Result<(), String> {
        let library = self.libraries.get_mut(library_name).ok_or_else(|| {
            format!("Attached pack library '{library_name}' disappeared before publication")
        })?;
        let root = library.root_path.as_ref().ok_or_else(|| {
            format!("Attached pack library '{library_name}' has no root identity")
        })?;
        let root_digest = library
            .source_closure
            .iter()
            .find(|pin| pin.path == *root)
            .map(|pin| pin.digest)
            .ok_or_else(|| {
                format!(
                    "Attached pack library '{library_name}' did not retain its root source bytes"
                )
            })?;
        let source_id = match library.source_authority {
            ModelSourceAuthority::RetainedImport { source_id, .. } => source_id,
            _ => ModelSourceId::new(),
        };
        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id,
            digest: root_digest,
        };
        library.pack_id = Some(pack_id.to_owned());
        Ok(())
    }

    /// Explicitly refresh a shipped-pack snapshot from the currently installed
    /// corpus, then immediately return it to retained project authority.
    pub fn refresh_spice_pack(&mut self, pack_id: &str) -> Result<String, String> {
        let entry = {
            let index = self.spice_packs.as_ref().ok_or_else(|| {
                "The shipped model corpus is not installed on this machine.".to_owned()
            })?;
            let pack = index
                .pack(pack_id)
                .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
            pack.entry_path(index.root()).ok_or_else(|| {
                format!(
                    "Model pack '{}' has no declared entry file to refresh.",
                    pack.name
                )
            })?
        };
        let selected_corner = self
            .libraries
            .values()
            .find(|library| library.pack_id.as_deref() == Some(pack_id))
            .and_then(|library| library.selected_corner.clone());
        let library_name = self.load_library_file(&entry, selected_corner.as_deref())?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    /// Load the exact shipped source containing an addressable part. Restricted
    /// files fail closed instead of being silently copied into project data.
    pub fn add_spice_part(&mut self, pack_id: &str, part_name: &str) -> Result<String, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model corpus is not installed on this machine.".to_owned()
        })?;
        let pack = index
            .pack(pack_id)
            .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
        if !pack.redistributable {
            return Err(format!(
                "Part '{part_name}' cannot be copied from '{}' because its redistribution grant is not established.",
                pack.name
            ));
        }
        let matches = index
            .find_part(part_name)
            .map_err(|error| format!("Part '{part_name}' could not be resolved: {error}"))?;
        let entry = matches
            .into_iter()
            .find(|entry| entry.pack == pack_id)
            .ok_or_else(|| {
                format!(
                    "Part '{part_name}' is no longer present in pack '{}'.",
                    pack.name
                )
            })?;
        if entry.restricted {
            return Err(format!(
                "Part '{part_name}' is in a source file that is not licensed for project embedding."
            ));
        }
        let source = entry
            .source_path(index)
            .ok_or_else(|| format!("Part '{part_name}' has no installed source file."))?;
        let library_name = self.load_catalog_source_without_collision(&source)?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    fn load_catalog_source_without_collision(&mut self, path: &Path) -> Result<String, String> {
        let canonical = std::fs::canonicalize(path).map_err(|error| {
            format!(
                "Failed to resolve model source '{}': {error}",
                path.display()
            )
        })?;
        let library_name = canonical
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                format!(
                    "Model source '{}' has no valid file name.",
                    canonical.display()
                )
            })?
            .to_owned();
        if let Some(existing) = self.get_library(&library_name) {
            if existing.root_path.as_deref() == Some(canonical.as_path()) {
                return Ok(library_name);
            }
            return Err(format!(
                "Library name '{library_name}' is already owned by another source; rename or detach it before adding '{}'.",
                canonical.display()
            ));
        }
        self.load_library_file(&canonical, None)
    }

    /// Get libraries sorted by name
    pub fn libraries_sorted(&self) -> Vec<&ModelLibrary> {
        let mut libs: Vec<_> = self.libraries.values().collect();
        libs.sort_by(|a, b| a.name.cmp(&b.name));
        libs
    }

    /// Stable owned snapshot used by guarded multi-library project
    /// transactions. Presentation filters and the shipped-pack index remain
    /// manager state and are intentionally excluded.
    pub(crate) fn library_snapshot(&self) -> Vec<ModelLibrary> {
        self.libraries_sorted().into_iter().cloned().collect()
    }

    /// Replace the complete loaded-library set while preserving presentation
    /// state owned by this manager.
    pub(crate) fn replace_library_snapshot(
        &mut self,
        libraries: Vec<ModelLibrary>,
    ) -> Result<(), String> {
        let expanded = self
            .libraries
            .iter()
            .map(|(name, library)| (name.clone(), library.expanded))
            .collect::<HashMap<_, _>>();
        let mut replacement = HashMap::with_capacity(libraries.len());
        for mut library in libraries {
            if replacement.contains_key(&library.name) {
                return Err(format!(
                    "Model-library snapshot repeats library '{}'",
                    library.name
                ));
            }
            if let Some(retained) = expanded.get(&library.name) {
                library.expanded = *retained;
            }
            replacement.insert(library.name.clone(), library);
        }
        self.libraries = replacement;
        if self
            .selected_library
            .as_ref()
            .is_some_and(|name| !self.libraries.contains_key(name))
        {
            self.selected_library = None;
        }
        Ok(())
    }

    /// Enforce the model-library dialect boundary before any parsed
    /// projection is accepted. `.scs` sources admit the explicit
    /// `simulator lang=spice` interoperability profile and the fail-closed
    /// declarative Spectre model-library subset implemented by the core
    /// adapter. Unsupported native statements are errors, never discarded.
    pub(crate) fn validate_model_source_dialect(path: &Path, source: &str) -> Result<(), String> {
        rspice_core::library::adapt_spectre_model_library(path, source)
            .map(|_| ())
            .map_err(|error| {
                format!(
                    "{}:{} cannot be imported as an executable model library: {}",
                    path.display(),
                    error.line,
                    error.message
                )
            })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn extend_native_veriloga_closure(
        result: &mut rspice_core::library::LibParseResult,
    ) -> Result<(), String> {
        let mut roots = BTreeSet::<PathBuf>::new();
        for resolved in &result.resolved_sources {
            let projected = rspice_core::library::adapt_spectre_model_library(
                &resolved.path,
                &resolved.content,
            )
            .map_err(|error| {
                format!(
                    "{}:{} cannot authenticate AHDL dependencies: {}",
                    resolved.path.display(),
                    error.line,
                    error.message
                )
            })?;
            for line in projected.lines() {
                let Some(include) = rspice_core::netlist::parse_veriloga_source_directive(line)
                else {
                    continue;
                };
                let requested = rspice_core::netlist::normalize_source_path_literal(
                    &include.file_path.to_string_lossy(),
                )
                .map_err(|error| {
                    format!(
                        "{} has an invalid Verilog-A dependency: {error}",
                        resolved.path.display()
                    )
                })?;
                let matches = result
                    .resolved_dependencies
                    .iter()
                    .filter(|dependency| {
                        dependency.owner == resolved.path
                            && rspice_core::netlist::normalize_source_path_literal(
                                &dependency.requested_path,
                            )
                            .is_ok_and(|candidate| candidate == requested)
                    })
                    .collect::<Vec<_>>();
                let [dependency] = matches.as_slice() else {
                    return Err(format!(
                        "{} Verilog-A dependency '{}' has {} resolution edges",
                        resolved.path.display(),
                        requested,
                        matches.len()
                    ));
                };
                roots.insert(dependency.target.clone());
            }
        }

        let limits = rspice_veriloga::SourceProviderLimits {
            max_dependencies: crate::state::MAX_PROJECT_SOURCE_FILES.saturating_add(64),
            max_total_source_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
            max_include_depth: crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
            max_expanded_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
        };
        for root in roots {
            let mut preprocessor = rspice_veriloga::Preprocessor::new();
            preprocessor
                .preprocess_file_with_limits(&root, limits)
                .map_err(|error| {
                    format!(
                        "Could not authenticate Verilog-A closure rooted at '{}': {error}",
                        root.display()
                    )
                })?;
            let documents = preprocessor.take_dependency_documents();
            let provider_paths = documents
                .iter()
                .filter(|document| {
                    document.origin == rspice_veriloga::SourceDocumentOrigin::Provider
                })
                .map(|document| document.logical_path.clone())
                .collect::<HashSet<_>>();
            for document in documents.into_iter().filter(|document| {
                document.origin == rspice_veriloga::SourceDocumentOrigin::Provider
            }) {
                if let Some(existing) = result
                    .resolved_sources
                    .iter()
                    .find(|source| source.path == document.logical_path)
                {
                    if existing.content.as_ref() != document.source.as_str() {
                        return Err(format!(
                            "Verilog-A dependency '{}' changed while its closure was captured",
                            document.logical_path.display()
                        ));
                    }
                    continue;
                }
                let bytes: Arc<[u8]> = Arc::from(document.source.as_bytes());
                let content: Arc<str> = Arc::from(document.source);
                result
                    .resolved_sources
                    .push(rspice_core::library::ResolvedLibSource {
                        path: document.logical_path,
                        bytes,
                        content,
                    });
            }
            for include in preprocessor.take_include_graph() {
                if !provider_paths.contains(&include.included_path) {
                    continue;
                }
                let dependency = rspice_core::library::ResolvedLibDependency {
                    owner: include.including_path,
                    requested_path: include.requested_path,
                    target: include.included_path,
                };
                if let Some(existing) = result.resolved_dependencies.iter().find(|existing| {
                    existing.owner == dependency.owner
                        && existing.requested_path == dependency.requested_path
                }) {
                    if existing.target != dependency.target {
                        return Err(format!(
                            "Verilog-A dependency '{}' in '{}' resolved inconsistently",
                            dependency.requested_path,
                            dependency.owner.display()
                        ));
                    }
                } else {
                    result.resolved_dependencies.push(dependency);
                }
            }
        }
        result
            .resolved_sources
            .sort_by(|left, right| left.path.cmp(&right.path));
        result.resolved_dependencies.sort();
        result.resolved_dependencies.dedup();
        let total_bytes = result
            .resolved_sources
            .iter()
            .try_fold(0usize, |total, source| {
                total.checked_add(source.bytes.len())
            })
            .ok_or_else(|| "Model source closure size overflowed".to_owned())?;
        if result.resolved_sources.len() > crate::state::MAX_PROJECT_SOURCE_FILES
            || total_bytes > crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES
        {
            return Err(format!(
                "Model source closure including Verilog-A dependencies exceeds the project limit ({} files / {} bytes)",
                crate::state::MAX_PROJECT_SOURCE_FILES,
                crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES
            ));
        }
        Ok(())
    }

    /// Total library count
    pub fn library_count(&self) -> usize {
        self.libraries.len()
    }

    /// Total model count across all libraries
    pub fn total_model_count(&self) -> usize {
        self.libraries.values().map(|l| l.model_count()).sum()
    }

    /// Clear all
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.libraries.clear();
        self.resolution_records.clear();
        self.selected_library = None;
    }

    /// Load a library from a .lib file
    ///
    /// Parses the file using the rspice-core library parser and adds models
    /// to a new library entry.
    pub fn load_library_file(
        &mut self,
        path: impl AsRef<std::path::Path>,
        section: Option<&str>,
    ) -> Result<String, String> {
        use rspice_core::library::LibParser;

        let path = std::fs::canonicalize(path.as_ref()).map_err(|error| {
            format!(
                "Failed to resolve model library '{}': {error}",
                path.as_ref().display()
            )
        })?;
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));
        let lib_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unnamed")
            .to_string();

        // The parser captures the exact root-plus-include bytes it consumes.
        // Hash that captured closure so parsing and pinning cannot observe
        // different file versions during an explicit refresh.
        let mut parser = LibParser::new(base_dir);
        let result = parser.parse_file(&path).map_err(|error| {
            format!(
                "Failed to parse model library '{}': {error}",
                path.display()
            )
        })?;
        if !result.is_ok() {
            return Err(format!(
                "Model library '{}' contains parse or dependency errors: {}",
                path.display(),
                result
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }
        #[cfg(not(target_arch = "wasm32"))]
        let result = {
            let mut result = result;
            Self::extend_native_veriloga_closure(&mut result)?;
            result
        };
        let mut source_closure = result
            .resolved_sources
            .iter()
            .map(|source| ModelSourcePin {
                path: source.path.clone(),
                digest: crate::product::ContentDigest::from_bytes(
                    Sha256::digest(source.bytes.as_ref()).into(),
                ),
            })
            .collect::<Vec<_>>();
        source_closure.sort_by(|left, right| left.path.cmp(&right.path));
        let mut source_contents = result
            .resolved_sources
            .iter()
            .map(|source| ModelSourceContent {
                path: source.path.clone(),
                bytes: source.bytes.as_ref().to_vec(),
            })
            .collect::<Vec<_>>();
        source_contents.sort_by(|left, right| left.path.cmp(&right.path));
        for content in &source_contents {
            let source =
                rspice_core::netlist::decode_source_bytes(&content.bytes).map_err(|error| {
                    format!(
                        "Model source '{}' cannot be decoded for dialect validation: {error}",
                        content.path.display()
                    )
                })?;
            Self::validate_model_source_dialect(&content.path, &source)?;
        }
        if source_closure.is_empty() {
            return Err(format!(
                "Model library '{}' produced an empty source dependency closure",
                path.display()
            ));
        }
        let mut source_edges = result
            .resolved_dependencies
            .iter()
            .map(|edge| ModelSourceEdge {
                owner: edge.owner.clone(),
                requested_path: edge.requested_path.clone(),
                target: edge.target.clone(),
            })
            .collect::<Vec<_>>();
        source_edges.sort();
        source_edges.dedup();
        if let Some(unreachable) = first_unreachable_source(&path, &source_closure, &source_edges) {
            return Err(format!(
                "Model library '{}' captured dependency '{}' that is not reachable from its root by authenticated resolution edges",
                path.display(),
                unreachable.display()
            ));
        }

        if let Some(existing) = self.libraries.get(&lib_name)
            && existing.root_path.as_deref() != Some(path.as_path())
        {
            return Err(format!(
                "Cannot load '{}': library name '{}' is already owned by a different model source",
                path.display(),
                lib_name
            ));
        }

        // Build a complete replacement and publish it only after every parse
        // and section check succeeds. A failed refresh never leaves a partly
        // updated model catalog behind.
        let mut library = self
            .libraries
            .get(&lib_name)
            .cloned()
            .unwrap_or_else(|| ModelLibrary::new(&lib_name));
        library.root_path = Some(path.clone());
        library.source_authority = ModelSourceAuthority::External;
        library.source_closure = source_closure;
        library.source_contents = source_contents;
        library.source_edges = source_edges;
        library.models.clear();
        library.top_level_models.clear();
        library.section_models.clear();
        library.subcircuits.clear();
        library.model_definition_metadata.clear();
        library.model_qualification.clear();
        library.model_correlation.clear();
        library.corners.clear();
        library.selected_corner = None;

        for section_name in result.section_names() {
            // Build the corner through its section contract rather than by
            // field assignment: a corner with no section binding materializes
            // to nothing, so a bare name would seal an empty corner.
            let mut corner =
                ProcessCorner::from_composite_section(section_name, path.clone(), false);
            corner.description = format!("Process corner from {lib_name}");
            library.corners.insert(corner.name.clone(), corner);
        }

        let section_names = result.section_names();
        let selected_section = if let Some(section_name) = section {
            Some(section_name.to_owned())
        } else {
            section_names
                .iter()
                .find(|name| name.eq_ignore_ascii_case("tt"))
                .or_else(|| section_names.first())
                .map(|name| (*name).to_owned())
        };

        for model in &result.top_level_models {
            let device_model = Self::convert_parsed_model(model, &path);
            library
                .top_level_models
                .insert(device_model.name.clone(), device_model.clone());
            library
                .models
                .insert(device_model.name.clone(), device_model);
        }
        // Every section's interfaces are retained, not just the selected one:
        // a subcircuit is addressable by section-qualified identity, and a
        // library that declares only `.subckt` definitions is still a library.
        Self::insert_parsed_subcircuits(&mut library, &result.top_level_subcircuits, &path, None)?;
        for lib_section in &result.sections {
            Self::insert_parsed_subcircuits(
                &mut library,
                &lib_section.subcircuits,
                &path,
                Some(&lib_section.name),
            )?;
        }

        for lib_section in &result.sections {
            let section_models = library
                .section_models
                .entry(lib_section.name.clone())
                .or_default();
            for model in &lib_section.models {
                let device_model =
                    Self::convert_parsed_model_in_section(model, &path, Some(&lib_section.name));
                section_models.insert(device_model.name.clone(), device_model);
            }
        }

        if let Some(section_name) = selected_section.as_deref() {
            if let Some(lib_section) = result.get_section(section_name) {
                library.selected_corner = Some(lib_section.name.clone());
                if let Some(corner) = library.corners.get_mut(&lib_section.name) {
                    corner.is_default = true;
                }
            } else {
                return Err(format!(
                    "Section '{}' not found. Available: {:?}",
                    section_name,
                    result.section_names()
                ));
            }
        }
        library.refresh_effective_model_projection();
        if library.top_level_models.is_empty()
            && library.section_models.values().all(HashMap::is_empty)
            && library.subcircuits.is_empty()
        {
            return Err(format!(
                "Model library '{}' contains no supported device models or addressable subcircuits",
                path.display()
            ));
        }

        self.libraries.insert(lib_name.clone(), library);
        Ok(lib_name)
    }

    /// Import one self-contained model source from authenticated bytes.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn load_library_bytes(
        &mut self,
        file_name: &str,
        bytes: Vec<u8>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle(file_name, vec![(file_name.to_owned(), bytes)], section)
    }

    /// Acquire one catalog pack from browser-selected retained bytes.
    ///
    /// The declared entry must be present and must resolve as the bundle's one
    /// dependency root. Only then is the retained library associated with the
    /// catalog identity; an arbitrary upload can never masquerade as a pack.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn load_spice_pack_bundle(
        &mut self,
        pack_id: &str,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<String, String> {
        let (pack_name, entry_name) = {
            let index = self.spice_packs.as_ref().ok_or_else(|| {
                "The embedded model-pack catalog is no longer available.".to_owned()
            })?;
            let pack = index
                .pack(pack_id)
                .ok_or_else(|| format!("Model pack '{pack_id}' is no longer in the catalog."))?;
            if !pack.redistributable {
                return Err(format!(
                    "Model pack '{}' cannot be retained in this project because its redistribution grant is not established.",
                    pack.name
                ));
            }
            let entry_name = normalize_browser_bundle_member_path(
                &pack
                    .entry
                    .as_deref()
                    .ok_or_else(|| {
                        format!("Model pack '{}' has no declared entry file.", pack.name)
                    })?
                    .to_string_lossy(),
            )
            .map_err(|error| {
                format!(
                    "Model pack '{}' has an invalid declared entry file: {error}",
                    pack.name
                )
            })?;
            (pack.name.clone(), entry_name)
        };
        if !files
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(&entry_name))
        {
            return Err(format!(
                "The selected bundle is not '{pack_name}' because it does not contain declared entry '{entry_name}'. Select that entry and its complete source tree."
            ));
        }

        let mut candidate = self.clone();
        let library =
            candidate.load_library_bundle_from_root(&entry_name, &entry_name, files, None)?;
        let imported_root = candidate
            .get_library(&library)
            .ok_or_else(|| "Imported model library disappeared before publication.".to_owned())?;
        if imported_root
            .root_path
            .as_deref()
            .is_none_or(|root| !browser_bundle_path_ends_with(root, &entry_name))
        {
            return Err(format!(
                "The selected '{pack_name}' bundle did not resolve declared entry '{entry_name}' as its one dependency root. Remove unrelated roots and select the complete source tree."
            ));
        }
        candidate
            .get_library_mut(&library)
            .expect("validated imported library remains present")
            .pack_id = Some(pack_id.to_owned());
        *self = candidate;
        Ok(library)
    }

    /// Import a complete browser-selected source tree.
    ///
    /// Every uploaded dependency is retained and every `.include` or external
    /// `.lib` edge is resolved relative to its owning source. Member identities
    /// are normalized portable relative paths, and traversal outside the
    /// selected tree fails closed. Multiple independent roots are joined by an
    /// RSpice-owned synthetic root, so the authenticated closure remains
    /// deterministic and no selected source is silently discarded.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn load_library_bundle(
        &mut self,
        display_name: &str,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle_with_root(display_name, None, files, section)
    }

    /// Import a browser-selected source tree from one explicit executable
    /// entry. Unreachable members are neither decoded nor retained.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn load_library_bundle_from_root(
        &mut self,
        display_name: &str,
        root_member: &str,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle_with_root(display_name, Some(root_member), files, section)
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    fn load_library_bundle_with_root(
        &mut self,
        display_name: &str,
        root_member: Option<&str>,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        use rspice_core::library::{LibParser, ResolvedLibDependency};

        if files.is_empty() {
            return Err("Model source bundle contains no files".to_owned());
        }
        if files.len() > crate::state::MAX_PROJECT_SOURCE_FILES {
            return Err(format!(
                "Model source bundle contains {} files; the limit is {}",
                files.len(),
                crate::state::MAX_PROJECT_SOURCE_FILES
            ));
        }
        let total_bytes = files.iter().try_fold(0usize, |total, (_, bytes)| {
            total
                .checked_add(bytes.len())
                .ok_or_else(|| "Model source bundle size overflowed".to_owned())
        })?;
        if total_bytes > crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES {
            return Err(format!(
                "Model source bundle contains {total_bytes} bytes; the limit is {}",
                crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES
            ));
        }

        let mut members = BTreeMap::<String, Vec<u8>>::new();
        let mut case_folded = HashMap::<String, String>::new();
        for (name, bytes) in files {
            let safe_name = normalize_browser_bundle_member_path(&name).map_err(|error| {
                format!("Model source bundle member '{name}' is invalid: {error}")
            })?;
            if case_folded
                .insert(safe_name.to_ascii_lowercase(), safe_name.clone())
                .is_some()
            {
                return Err(format!(
                    "Model source bundle repeats portable path '{safe_name}' ignoring case"
                ));
            }
            members.insert(safe_name, bytes);
        }

        let requested_root = root_member
            .map(normalize_browser_bundle_member_path)
            .transpose()
            .map_err(|error| format!("Model source bundle root is invalid: {error}"))?;
        let selected_root = if let Some(requested_root) = requested_root {
            case_folded
                .get(&requested_root.to_ascii_lowercase())
                .cloned()
                .ok_or_else(|| {
                    format!("Model source bundle does not contain selected root '{requested_root}'")
                })?
        } else {
            let display_root = normalize_browser_bundle_member_path(display_name)
                .ok()
                .and_then(|display_member| {
                    case_folded
                        .get(&display_member.to_ascii_lowercase())
                        .cloned()
                });
            display_root
                .or(infer_browser_bundle_root(&members, &case_folded)?)
                .ok_or_else(|| {
                "Model source bundle has more than one possible executable root; select the entry file explicitly"
                    .to_owned()
                })?
        };

        let reachable = reachable_browser_bundle_members(&selected_root, &members, &case_folded)?;
        members.retain(|name, _| reachable.contains(name));
        case_folded.retain(|_, name| reachable.contains(name));

        let mut bundle_hasher = Sha256::new();
        for (name, bytes) in &members {
            bundle_hasher.update((name.len() as u64).to_be_bytes());
            bundle_hasher.update(name.as_bytes());
            bundle_hasher.update((bytes.len() as u64).to_be_bytes());
            bundle_hasher.update(bytes);
        }
        let bundle_digest = ContentDigest::from_bytes(bundle_hasher.finalize().into());
        let base = PathBuf::from(format!("/rspice-browser/model-sources/{bundle_digest}"));
        let member_paths = members
            .keys()
            .map(|name| (name.clone(), base.join(name)))
            .collect::<HashMap<_, _>>();
        let mut dependencies = Vec::<ResolvedLibDependency>::new();
        let mut decoded_members = BTreeMap::<String, String>::new();
        let mut veriloga_roots = BTreeSet::<String>::new();
        for (owner_name, bytes) in &members {
            let source = rspice_core::netlist::decode_source_bytes(bytes).map_err(|error| {
                format!("Uploaded model source '{owner_name}' cannot be decoded: {error}")
            })?;
            let dependency_projection =
                rspice_core::library::adapt_spectre_model_library(Path::new(owner_name), &source)
                    .map_err(|error| {
                    format!(
                        "{owner_name}:{} cannot be imported as an executable model library: {}",
                        error.line, error.message
                    )
                })?;
            decoded_members.insert(owner_name.clone(), source.clone());
            let owner = member_paths
                .get(owner_name)
                .expect("every source member has a virtual identity")
                .clone();
            for (line_index, line) in dependency_projection.lines().enumerate() {
                let dependency = rspice_core::netlist::parse_include_directive(line)
                    .map(|path| (path, false))
                    .or_else(|| {
                        rspice_core::netlist::parse_lib_directive(line)
                            .and_then(|(path, section)| section.map(|_| (path, false)))
                    })
                    .or_else(|| {
                        rspice_core::netlist::parse_veriloga_source_directive(line)
                            .map(|include| (include.file_path.to_string_lossy().into_owned(), true))
                    });
                let Some((requested_path, is_veriloga)) = dependency else {
                    continue;
                };
                let normalized = resolve_browser_bundle_dependency(owner_name, &requested_path)
                    .map_err(|error| {
                    format!(
                        "{owner_name}:{} has an invalid dependency path '{requested_path}': {error}",
                        line_index + 1
                    )
                })?;
                let target_name = case_folded
                    .get(&normalized.to_ascii_lowercase())
                    .ok_or_else(|| {
                    format!(
                        "{owner_name}:{} dependency '{requested_path}' is missing from the selected browser bundle",
                        line_index + 1
                    )
                })?;
                let target = member_paths
                    .get(target_name)
                    .expect("case-folded member identity belongs to the virtual bundle");
                if is_veriloga {
                    veriloga_roots.insert(target_name.clone());
                }
                dependencies.push(ResolvedLibDependency {
                    owner: owner.clone(),
                    requested_path,
                    target: target.clone(),
                });
            }
        }

        let virtual_files = decoded_members
            .iter()
            .map(|(path, source)| rspice_veriloga::VirtualSourceFile::new(path, source))
            .collect::<Vec<_>>();
        let veriloga_limits = rspice_veriloga::VirtualCompileLimits {
            max_files: crate::state::MAX_PROJECT_SOURCE_FILES,
            max_path_bytes: crate::state::MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES,
            max_file_bytes: crate::state::MAX_PROJECT_CODE_SOURCE_BYTES,
            max_total_source_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES,
            max_include_depth: crate::state::MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
            max_expanded_bytes: crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2),
            max_module_name_bytes: 128,
        };
        for veriloga_root in &veriloga_roots {
            let bundle = rspice_veriloga::VirtualSourceBundle::new(
                veriloga_root,
                virtual_files.iter().cloned(),
            )
            .map_err(|error| {
                format!("Uploaded Verilog-A bundle rooted at '{veriloga_root}' is invalid: {error}")
            })?;
            let discovery = rspice_veriloga::VerilogACompiler::default()
                .discover_virtual_modules(&bundle, veriloga_limits)
                .map_err(|error| {
                    format!(
                        "Uploaded Verilog-A bundle rooted at '{veriloga_root}' cannot be compiled: {error}"
                    )
                })?;
            for include in discovery.include_graph {
                let Some(owner_name) =
                    case_folded.get(&include.including_path.to_ascii_lowercase())
                else {
                    // Compiler-owned standard headers never become project
                    // model-library artifacts.
                    continue;
                };
                let Some(target_name) =
                    case_folded.get(&include.included_path.to_ascii_lowercase())
                else {
                    continue;
                };
                let owner = member_paths
                    .get(owner_name)
                    .expect("Verilog-A owner belongs to the selected bundle");
                let target = member_paths
                    .get(target_name)
                    .expect("Verilog-A dependency belongs to the selected bundle");
                dependencies.push(ResolvedLibDependency {
                    owner: owner.clone(),
                    requested_path: include.requested_path,
                    target: target.clone(),
                });
            }
        }
        dependencies.sort();
        dependencies.dedup();

        let sources = members
            .iter()
            .map(|(name, bytes)| (member_paths[name].clone(), bytes.clone()))
            .collect::<Vec<_>>();
        let root_name = selected_root;
        let root = member_paths[&root_name].clone();
        let root_bytes = sources
            .iter()
            .find_map(|(path, bytes)| (path == &root).then_some(bytes))
            .expect("the authenticated bundle contains its root");
        let root_digest = ContentDigest::from_bytes(Sha256::digest(root_bytes).into());
        let lib_name = Path::new(&root_name)
            .file_stem()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .unwrap_or("uploaded-models")
            .to_owned();

        let authenticated_dependencies = dependencies.clone();
        let mut parser = LibParser::new(root.parent().unwrap_or(Path::new("/")));
        let result = parser
            .parse_authenticated_closure(root.clone(), sources.clone(), dependencies)
            .map_err(|error| {
                format!("Uploaded model bundle could not be authenticated: {error}")
            })?;
        if !result.is_ok() {
            return Err(format!(
                "Uploaded model bundle contains parse or unresolved dependency errors: {}",
                result
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }

        let mut library = ModelLibrary::new(&lib_name);
        library.root_path = Some(root.clone());
        // Imported bytes the project retained, not a definition the project
        // authored. The distinction is load-bearing: a project-owned library
        // carries a revision and typed definition metadata, and
        // `project_model_definition_identities` fails closed when a
        // project-owned model has none. An import has neither, so recording it
        // as project-owned makes that check demand metadata that cannot exist.
        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id: crate::product::ModelSourceId::new(),
            digest: root_digest,
        };
        library.source_closure = sources
            .iter()
            .map(|(path, bytes)| ModelSourcePin {
                path: path.clone(),
                digest: ContentDigest::from_bytes(Sha256::digest(bytes).into()),
            })
            .collect();
        library
            .source_closure
            .sort_by(|left, right| left.path.cmp(&right.path));
        library.source_contents = sources
            .into_iter()
            .map(|(path, bytes)| ModelSourceContent { path, bytes })
            .collect();
        library
            .source_contents
            .sort_by(|left, right| left.path.cmp(&right.path));
        library.source_edges = authenticated_dependencies
            .iter()
            .map(|dependency| ModelSourceEdge {
                owner: dependency.owner.clone(),
                requested_path: dependency.requested_path.clone(),
                target: dependency.target.clone(),
            })
            .collect();
        library.source_edges.sort();
        library.corners.clear();
        // `ModelLibrary::new` seeds the standard corners and selects "tt".
        // Clearing the catalogue without clearing the selection leaves a
        // library pointing at a corner it no longer defines, which fails
        // projection later with "selected corner does not exist". A section
        // below re-selects when the source actually declares one.
        library.selected_corner = None;
        for section_name in result.section_names() {
            let mut corner =
                ProcessCorner::from_composite_section(section_name, root.clone(), false);
            corner.description = format!("Process corner from {lib_name}");
            library.corners.insert(corner.name.clone(), corner);
        }
        let section_names = result.section_names();
        let selected_section = section.map(str::to_owned).or_else(|| {
            section_names
                .iter()
                .find(|name| name.eq_ignore_ascii_case("tt"))
                .or_else(|| section_names.first())
                .map(|name| (*name).to_owned())
        });
        for model in &result.top_level_models {
            let device_model = Self::convert_parsed_model(model, &root);
            library
                .top_level_models
                .insert(device_model.name.clone(), device_model.clone());
            library
                .models
                .insert(device_model.name.clone(), device_model);
        }
        Self::insert_parsed_subcircuits(&mut library, &result.top_level_subcircuits, &root, None)?;
        for lib_section in &result.sections {
            Self::insert_parsed_subcircuits(
                &mut library,
                &lib_section.subcircuits,
                &root,
                Some(&lib_section.name),
            )?;
        }
        for lib_section in &result.sections {
            let section_models = library
                .section_models
                .entry(lib_section.name.clone())
                .or_default();
            for model in &lib_section.models {
                let device_model =
                    Self::convert_parsed_model_in_section(model, &root, Some(&lib_section.name));
                section_models.insert(device_model.name.clone(), device_model);
            }
        }
        if let Some(section_name) = selected_section.as_deref() {
            let lib_section = result.get_section(section_name).ok_or_else(|| {
                format!(
                    "Section '{section_name}' not found. Available: {:?}",
                    result.section_names()
                )
            })?;
            library.selected_corner = Some(lib_section.name.clone());
            if let Some(corner) = library.corners.get_mut(&lib_section.name) {
                corner.is_default = true;
            }
        }
        library.refresh_effective_model_projection();
        // A macromodel library legitimately declares only `.subckt`
        // definitions, so an empty model map alone is not an empty library.
        if library.top_level_models.is_empty()
            && library.section_models.values().all(HashMap::is_empty)
            && library.subcircuits.is_empty()
        {
            return Err(format!(
                "Model library '{lib_name}' contains no supported device models or addressable subcircuits"
            ));
        }
        if self.libraries.contains_key(&lib_name) {
            return Err(format!(
                "Model library '{lib_name}' already exists; remove it before importing replacement bytes"
            ));
        }
        self.libraries.insert(lib_name.clone(), library);
        Ok(lib_name)
    }

    /// Resolve deterministic, self-contained model cards for a nominal run.
    pub fn reference_process_model_cards(
        &self,
        process: crate::simulation::dialog::corner::ProcessCorner,
    ) -> Result<Vec<String>, String> {
        self.seal_execution_sources()?
            .reference_process_model_cards(process)
    }

    /// Resolve all process-specific sources required by a PVT run.
    #[cfg(test)]
    pub fn corner_model_bindings(
        &self,
        processes: &[CornerProcess],
    ) -> Result<Vec<CornerModelBinding>, String> {
        self.seal_execution_sources()?
            .corner_model_bindings(processes)
    }

    /// Atomically replace the exact external libraries governed by a PDK
    /// configuration.
    ///
    /// Discovery always runs against `next`; cached dialog rows never
    /// authorize application. Every enabled file is canonicalized and parsed
    /// into an isolated manager candidate. Scan, include, parse, or
    /// name-collision failure leaves both this manager and the retained PDK
    /// ownership provenance unchanged.
    pub fn replace_from_pdk_config(
        &mut self,
        previous: Option<&crate::state::pdk_config::PdkConfig>,
        next: &mut crate::state::pdk_config::PdkConfig,
    ) -> Result<usize, Vec<String>> {
        next.discover_model_files();
        if !next.scan_errors.is_empty() {
            return Err(next.scan_errors.clone());
        }

        let mut candidate = self.clone();
        let mut previously_managed = previous
            .into_iter()
            .flat_map(|config| config.managed_model_sources.iter())
            .map(|path| portable_path_key(path))
            .collect::<BTreeSet<_>>();
        if previously_managed.is_empty() {
            previously_managed.extend(
                previous
                    .into_iter()
                    .flat_map(|config| config.discovered_files.iter())
                    .map(|file| portable_path_key(&file.path)),
            );
        }
        let removed = candidate
            .libraries
            .iter()
            .filter(|&(_name, library)| {
                (matches!(library.source_authority, ModelSourceAuthority::External)
                    && library
                        .root_path
                        .as_ref()
                        .is_some_and(|path| previously_managed.contains(&portable_path_key(path))))
            })
            .map(|(name, _library)| name.clone())
            .collect::<Vec<_>>();
        for name in removed {
            candidate.libraries.remove(&name);
        }
        if candidate
            .selected_library
            .as_ref()
            .is_some_and(|name| !candidate.libraries.contains_key(name))
        {
            candidate.selected_library = None;
        }

        let mut loaded = 0;
        let mut errors = Vec::new();
        let mut admitted = Vec::new();
        let mut seen = BTreeSet::new();
        for file in &next.discovered_files {
            if !crate::state::pdk_config::MODEL_FILE_EXTENSIONS.contains(&file.extension.as_str()) {
                continue;
            }
            let canonical = match std::fs::canonicalize(&file.path) {
                Ok(path) => path,
                Err(error) => {
                    errors.push(format!(
                        "{}: failed to resolve discovered model source: {error}",
                        file.path.display()
                    ));
                    continue;
                }
            };
            if !seen.insert(portable_path_key(&canonical)) {
                continue;
            }
            match candidate.load_library_file(&canonical, None) {
                Ok(_) => {
                    admitted.push(canonical);
                    loaded += 1;
                }
                Err(error) => errors.push(format!("{}: {error}", file.path.display())),
            }
        }

        if errors.is_empty() {
            next.managed_model_sources = admitted;
            *self = candidate;
            Ok(loaded)
        } else {
            Err(errors)
        }
    }

    /// Populate with built-in models from the core engine
    ///
    /// Loads the embedded model libraries (diode.lib, mosfet.lib, etc.)
    /// into UI-accessible libraries.
    pub fn load_builtin_models(&mut self) {
        let core_manager = rspice_core::library::LibraryManager::new();

        for model_type in core_manager.available_types() {
            let models = core_manager.models_of_type(model_type);
            if models.is_empty() {
                continue;
            }

            let lib_name = model_type.display_name().to_string();
            let library = self
                .libraries
                .entry(lib_name.clone())
                .or_insert_with(|| ModelLibrary::new(&lib_name));

            for model in models {
                let device_model = DeviceModel {
                    name: model.name.clone(),
                    // Built-in cards are compiled in at file scope; no `.lib`
                    // section owns them.
                    section: None,
                    model_type: Self::convert_core_model_type(model.model_type),
                    spice_type: Some(Self::core_model_type_token(model.model_type).to_owned()),
                    level: ModelLevel::Unknown,
                    spice_level: None,
                    model_version: None,
                    description: model.description.clone().unwrap_or_default(),
                    l_min: model.lmin,
                    l_max: model.lmax,
                    w_min: model.wmin,
                    w_max: model.wmax,
                    vdd: None,
                    vth0: None,
                    file_path: None,
                    parameters: HashMap::new(),
                    string_parameters: HashMap::new(),
                    source_line: None,
                };
                library
                    .models
                    .insert(device_model.name.clone(), device_model);
            }
        }
    }

    /// Convert a parsed model from the core library to UI DeviceModel
    pub(crate) fn convert_parsed_model(
        model: &rspice_core::library::ParsedModel,
        file_path: &std::path::Path,
    ) -> DeviceModel {
        let model_type = Self::convert_core_model_type(model.model_type);

        DeviceModel {
            name: model.name.clone(),
            // This conversion has no section context; a card parsed inside a
            // `.lib` is attributed by the caller that knows the section.
            section: None,
            model_type,
            spice_type: Some(model.spice_type.clone()),
            level: Self::convert_model_level(model.level, &model.spice_type),
            spice_level: model.level,
            model_version: model.version,
            description: model.description.clone().unwrap_or_default(),
            l_min: model.lmin,
            l_max: model.lmax,
            w_min: model.wmin,
            w_max: model.wmax,
            vdd: None,
            vth0: None,
            file_path: Some(
                model
                    .source_file
                    .as_deref()
                    .unwrap_or(file_path)
                    .to_path_buf(),
            ),
            parameters: model.parameters.clone(),
            string_parameters: model.string_params.clone(),
            source_line: model.source_line,
        }
    }

    /// Classify what a model card *claims to be* for browsing/filtering.
    /// The card's type keyword wins over the LEVEL number because several
    /// LEVEL values are overloaded across device families (e.g. 8 is
    /// BSIM3v3 on a MOS card but HICUM/L2 on a BJT card, and 2002 is MVSG
    /// on MOS but DIODE_CMC on a diode). This is not a statement of native
    /// engine support.
    fn convert_model_level(level: Option<u32>, spice_type: &str) -> ModelLevel {
        let type_token = spice_type.trim().to_ascii_uppercase();

        // Family-named cards classify regardless of LEVEL.
        let by_name = match type_token.as_str() {
            t if t.starts_with("BSIMSOI") || t.starts_with("BSIM-SOI") => Some(ModelLevel::BsimSoi),
            t if t.starts_with("BSIMCMG") => Some(ModelLevel::BsimCmg),
            t if t.starts_with("BSIMBULK") => Some(ModelLevel::BsimBulk),
            t if t.starts_with("BSIMIMG") => Some(ModelLevel::BsimImg),
            t if t.starts_with("PSP") => Some(ModelLevel::Psp),
            t if t.starts_with("EKV") => Some(ModelLevel::Ekv),
            t if t.starts_with("HISIM") => Some(ModelLevel::HiSim),
            t if t.starts_with("L_UTSOI") || t.starts_with("LUTSOI") => Some(ModelLevel::LUtsoi),
            "MOSVAR" => Some(ModelLevel::Mosvar),
            t if t.starts_with("MVSG") => Some(ModelLevel::Mvsg),
            t if t.starts_with("VDMOS") || t == "NVDMOS" || t == "PVDMOS" => {
                Some(ModelLevel::Vdmos)
            }
            t if t.starts_with("VBIC") => Some(ModelLevel::Vbic),
            t if t.starts_with("MEXTRAM")
                || t.starts_with("BJT505")
                || t.starts_with("BJTD505") =>
            {
                Some(ModelLevel::Mextram)
            }
            t if t.starts_with("HICUM") => Some(ModelLevel::Hicum),
            t if t.starts_with("ASMHEMT")
                || t.starts_with("ANGELOV")
                || t.starts_with("EPFL_HEMT")
                || t.starts_with("EPFLHEMT") =>
            {
                Some(ModelLevel::Hemt)
            }
            "JUNCAP200" => Some(ModelLevel::Juncap),
            "DIODE_CMC" => Some(ModelLevel::DiodeCmc),
            t if t.starts_with("R2_CMC")
                || t.starts_with("R3_CMC")
                || t == "R2"
                || t == "R3"
                || t == "R2_ET" =>
            {
                Some(ModelLevel::RCmc)
            }
            _ => None,
        };
        if let Some(family) = by_name {
            return family;
        }

        let is_bjt = matches!(type_token.as_str(), "NPN" | "PNP" | "LPNP");
        let is_mos = matches!(type_token.as_str(), "NMOS" | "PMOS");
        let is_diode = matches!(type_token.as_str(), "D" | "DIODE");
        let is_resistor = matches!(type_token.as_str(), "R" | "RES" | "RESISTOR");

        match level {
            Some(4 | 9 | 11 | 12 | 13) if is_bjt => ModelLevel::Vbic,
            Some(8 | 230 | 234) if is_bjt => ModelLevel::Hicum,
            Some(504 | 505) if is_bjt => ModelLevel::Mextram,
            Some(200) if is_diode => ModelLevel::Juncap,
            Some(2002) if is_diode => ModelLevel::DiodeCmc,
            Some(1002 | 1003) if is_resistor => ModelLevel::RCmc,
            Some(1) => ModelLevel::SpiceLevel1,
            Some(3) => ModelLevel::SpiceLevel3,
            Some(8 | 49) if is_mos => ModelLevel::Bsim3v3,
            Some(14 | 54) if is_mos => ModelLevel::Bsim4,
            Some(10 | 55..=57 | 70470) if is_mos => ModelLevel::BsimSoi,
            Some(107 | 108 | 110 | 111) if is_mos => ModelLevel::BsimCmg,
            Some(104) if is_mos => ModelLevel::Psp,
            Some(260 | 301) if is_mos => ModelLevel::Ekv,
            Some(10240) if is_mos => ModelLevel::LUtsoi,
            Some(1000) if is_mos => ModelLevel::Mosvar,
            Some(2002) if is_mos => ModelLevel::Mvsg,
            Some(18) if is_mos => ModelLevel::Vdmos,
            // Preserve the historical level-only classification for cards
            // whose type keyword was not recognized.
            Some(8 | 49) => ModelLevel::Bsim3v3,
            Some(14 | 54) => ModelLevel::Bsim4,
            _ => ModelLevel::Unknown,
        }
    }

    fn core_model_type_token(model_type: rspice_core::library::ModelType) -> &'static str {
        use rspice_core::library::ModelType as CoreType;
        match model_type {
            CoreType::Nmos => "NMOS",
            CoreType::Pmos => "PMOS",
            CoreType::NpnBjt => "NPN",
            CoreType::PnpBjt => "PNP",
            CoreType::Diode => "D",
            CoreType::Resistor => "R",
            CoreType::Capacitor => "C",
            CoreType::Njfet => "NJF",
            CoreType::Pjfet => "PJF",
            CoreType::Other => "OTHER",
        }
    }

    /// Convert core ModelType to UI ModelType
    fn convert_core_model_type(core_type: rspice_core::library::ModelType) -> ModelType {
        use rspice_core::library::ModelType as CoreType;
        match core_type {
            CoreType::Nmos => ModelType::Nmos,
            CoreType::Pmos => ModelType::Pmos,
            CoreType::NpnBjt => ModelType::Npn,
            CoreType::PnpBjt => ModelType::Pnp,
            CoreType::Diode => ModelType::Diode,
            CoreType::Resistor => ModelType::Resistor,
            CoreType::Capacitor => ModelType::Capacitor,
            CoreType::Njfet | CoreType::Pjfet => ModelType::Other,
            CoreType::Other => ModelType::Other,
        }
    }
}

fn validate_project_library_name(name: &str) -> Result<(), String> {
    if name.is_empty() || name.trim() != name || name.len() > 128 {
        return Err(
            "Project model library name must contain 1 to 128 characters without outer whitespace"
                .to_owned(),
        );
    }
    if name
        .chars()
        .any(|character| character.is_control() || matches!(character, '/' | '\\'))
    {
        return Err(format!(
            "Project model library name '{name}' contains an invalid path or control character"
        ));
    }
    Ok(())
}

fn exact_subslice_offsets(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return Vec::new();
    }
    haystack
        .windows(needle.len())
        .enumerate()
        .filter_map(|(offset, candidate)| (candidate == needle).then_some(offset))
        .collect()
}

fn validate_project_owned_retained_closure(
    library: &ModelLibrary,
    root_digest: ContentDigest,
) -> Result<(), String> {
    let root = library.root_path.as_ref().ok_or_else(|| {
        format!(
            "Project-owned model library '{}' has no retained root identity",
            library.name
        )
    })?;
    if library.source_closure.is_empty()
        || library.source_closure.len() != library.source_contents.len()
    {
        return Err(format!(
            "Project-owned model library '{}' has an incomplete retained source closure",
            library.name
        ));
    }
    let mut pins = BTreeMap::new();
    for pin in &library.source_closure {
        if pins.insert(pin.path.clone(), pin.digest).is_some() {
            return Err(format!(
                "Project-owned model library '{}' repeats retained source '{}'",
                library.name,
                pin.path.display()
            ));
        }
    }
    let mut contents = BTreeMap::new();
    for content in &library.source_contents {
        if contents
            .insert(content.path.clone(), &content.bytes)
            .is_some()
        {
            return Err(format!(
                "Project-owned model library '{}' repeats retained bytes for '{}'",
                library.name,
                content.path.display()
            ));
        }
    }
    if pins.keys().ne(contents.keys()) {
        return Err(format!(
            "Project-owned model library '{}' retained pins and bytes do not describe the same closure",
            library.name
        ));
    }
    for (path, expected) in &pins {
        let bytes = contents
            .get(path)
            .expect("pin/content key equality was checked above");
        let actual = ContentDigest::from_bytes(Sha256::digest(bytes).into());
        if actual != *expected {
            return Err(format!(
                "Project-owned model source '{}' fails its retained content digest",
                path.display()
            ));
        }
    }
    if pins.get(root) != Some(&root_digest) {
        return Err(format!(
            "Project-owned model library '{}' root digest is inconsistent with its revision authority",
            library.name
        ));
    }
    if library
        .source_edges
        .iter()
        .any(|edge| !pins.contains_key(&edge.owner) || !pins.contains_key(&edge.target))
        || first_unreachable_source(root, &library.source_closure, &library.source_edges).is_some()
    {
        return Err(format!(
            "Project-owned model library '{}' has an invalid retained include graph",
            library.name
        ));
    }
    Ok(())
}

fn validate_section_qualification_evidence(
    metadata: &ModelDefinitionMetadata,
    qualification: &ModelQualificationState,
    source: &ModelSourceEvidenceBinding,
) -> Result<(), String> {
    for (index, section) in metadata.sections.iter().enumerate() {
        let evidence_digest = match &section.qualification {
            ModelSectionQualification::Qualified {
                evidence_digest: Some(evidence_digest),
            } => evidence_digest,
            ModelSectionQualification::Qualified {
                evidence_digest: None,
            } => {
                return Err(format!(
                    "Model section {:?} is qualified without an evidence digest",
                    section.name
                ));
            }
            ModelSectionQualification::Unqualified
            | ModelSectionQualification::Pending
            | ModelSectionQualification::Failed { .. } => continue,
        };
        let evidence_digest = evidence_digest.parse::<ContentDigest>().map_err(|error| {
            format!(
                "Model section {:?} has an invalid qualification evidence digest: {error}",
                section.name
            )
        })?;
        qualification
            .validate_exact_section_evidence_digest(source, &section.name, evidence_digest)
            .map_err(|error| {
                format!(
                    "Model section {:?} qualification at sections[{index}] is not backed by exact retained evidence: {error}",
                    section.name
                )
            })?;
    }
    Ok(())
}

#[cfg(test)]
fn reconcile_project_model_metadata(
    definition: &ProjectModelDefinition,
    previous: Option<&ModelDefinitionMetadata>,
) -> Result<ModelDefinitionMetadata, String> {
    if previous.is_some_and(|metadata| !metadata.sections.is_empty()) {
        return Err(
            "A sectioned model must be changed through the complete project-model revision transaction"
                .to_owned(),
        );
    }
    reconcile_project_model_revision_metadata(definition, previous)
}

fn reconcile_project_model_revision_metadata(
    definition: &ProjectModelDefinition,
    previous: Option<&ModelDefinitionMetadata>,
) -> Result<ModelDefinitionMetadata, String> {
    let mut metadata = previous.cloned().unwrap_or_default();
    let previous_parameters = metadata.parameters;
    let mut parameters = Vec::with_capacity(
        definition.numeric_parameters.len() + definition.string_parameters.len(),
    );
    for (name, value) in &definition.numeric_parameters {
        let mut parameter = previous_parameters
            .iter()
            .find(|parameter| parameter.name.eq_ignore_ascii_case(name))
            .cloned()
            .unwrap_or_else(|| ParameterDefinition {
                name: name.clone(),
                data_type: ParameterDataType::Numeric,
                value: ParameterValue::Numeric(
                    FiniteF64::new(*value).expect("project definitions reject non-finite values"),
                ),
                unit: None,
                bounds: None,
                source: ParameterSource::Declared {
                    source: "project model source".to_owned(),
                },
                description: format!("Project-owned {name} model parameter"),
            });
        if parameter.data_type != ParameterDataType::Numeric {
            return Err(format!(
                "Model parameter '{name}' cannot change from string to numeric without an explicit schema migration"
            ));
        }
        parameter.name = name.clone();
        parameter.value = ParameterValue::Numeric(
            FiniteF64::new(*value).expect("project definitions reject non-finite values"),
        );
        parameters.push(parameter);
    }
    for (name, value) in &definition.string_parameters {
        let mut parameter = previous_parameters
            .iter()
            .find(|parameter| parameter.name.eq_ignore_ascii_case(name))
            .cloned()
            .unwrap_or_else(|| ParameterDefinition {
                name: name.clone(),
                data_type: ParameterDataType::String,
                value: ParameterValue::String(value.clone()),
                unit: None,
                bounds: None,
                source: ParameterSource::Declared {
                    source: "project model source".to_owned(),
                },
                description: format!("Project-owned {name} model parameter"),
            });
        if parameter.data_type != ParameterDataType::String {
            return Err(format!(
                "Model parameter '{name}' cannot change from numeric to string without an explicit schema migration"
            ));
        }
        parameter.name = name.clone();
        parameter.value = ParameterValue::String(value.clone());
        parameters.push(parameter);
    }
    parameters.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
            .then_with(|| left.name.cmp(&right.name))
    });
    metadata.parameters = parameters;
    metadata
        .validate()
        .map_err(|error| format!("Project model metadata is invalid: {error}"))?;
    Ok(metadata)
}

#[cfg(test)]
fn verify_project_model_round_trip(
    definition: &ProjectModelDefinition,
    parsed: &rspice_core::library::ParsedModel,
) -> Result<(), String> {
    let expected_numeric = definition
        .numeric_parameters
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), *value))
        .collect::<HashMap<_, _>>();
    if parsed.parameters.len() != expected_numeric.len()
        || expected_numeric.iter().any(|(name, value)| {
            parsed
                .parameters
                .get(name)
                .is_none_or(|parsed| parsed.to_bits() != value.to_bits())
        })
    {
        return Err(
            "Project model numeric parameters did not survive canonical source parsing exactly"
                .to_owned(),
        );
    }
    let expected_strings = definition
        .string_parameters
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), value.as_str()))
        .collect::<HashMap<_, _>>();
    if parsed.string_params.len() != expected_strings.len()
        || expected_strings.iter().any(|(name, value)| {
            parsed
                .string_params
                .get(name)
                .is_none_or(|parsed| parsed != value)
        })
    {
        return Err(
            "Project model string parameters did not survive canonical source parsing exactly"
                .to_owned(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests;
