//! Capability-gated execution of callbacks retained in signed PDK archives.
//!
//! The guest receives no WASI, clock, entropy, filesystem, environment, or
//! network imports. Its entire authority is the intersection of the signed
//! callback contract and the fixed `rspice` ABI implemented below. Execution
//! is deterministic, fuel-metered, memory-bounded, and tied to exact package,
//! artifact, input, output, and target identities in a verifiable receipt.

use std::collections::{BTreeMap, BTreeSet};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use wasmi::{
    Caller, Config, EnforcedLimits, Engine, Extern, ExternType, Linker, Module, Store, StoreLimits,
    StoreLimitsBuilder, ValType,
};

use crate::product::{ContentDigest, ObjectRevision, ProjectId, SimulationPlanId};

use super::technology_package::{
    MAX_PDK_ARTIFACT_BYTES, MAX_PDK_CALLBACK_ARTIFACT_BYTES, MAX_PDK_TOTAL_ARTIFACT_BYTES,
    PDK_CALLBACK_ABI_VERSION, PdkCallbackCapability, PdkCallbackContract, PdkExecutionTarget,
    PdkTechnologyArtifact, PdkTechnologyBinding, PdkTechnologyError, SignedPdkTechnologyArchive,
    ValidatedPdkTechnologyPackage, current_execution_target,
};

pub const PDK_CALLBACK_EXECUTION_RECEIPT_SCHEMA_VERSION: u32 = 1;
pub const PROJECT_PDK_CALLBACK_RECEIPT_SCHEMA_VERSION: u32 = 1;
pub const MAX_PROJECT_PDK_CALLBACK_RECEIPTS: usize = 4_096;
pub const PDK_CALLBACK_FUEL_LIMIT: u64 = 10_000_000;
pub const PDK_CALLBACK_MEMORY_LIMIT_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_PDK_CALLBACK_PROJECT_PARAMETERS: usize = 1_024;
pub const MAX_PDK_CALLBACK_PARAMETER_KEY_BYTES: usize = 256;
pub const MAX_PDK_CALLBACK_PARAMETER_VALUE_BYTES: usize = 16 * 1024;
pub const MAX_PDK_CALLBACK_METADATA_ENTRIES: usize = 256;
pub const MAX_PDK_CALLBACK_METADATA_KEY_BYTES: usize = 256;
pub const MAX_PDK_CALLBACK_METADATA_VALUE_BYTES: usize = 64 * 1024;
pub const MAX_PDK_CALLBACK_METADATA_TOTAL_BYTES: usize = 256 * 1024;

const HOST_ABI_MODULE: &str = "rspice";
const GUEST_MEMORY_EXPORT: &str = "memory";
const HOST_ERROR_INVALID_ARGUMENT: i32 = -1;
const HOST_ERROR_NOT_FOUND: i32 = -2;
const HOST_ERROR_BUFFER_TOO_SMALL: i32 = -3;
const HOST_ERROR_FORBIDDEN: i32 = -4;
const HOST_ERROR_LIMIT: i32 = -5;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkCallbackExecutionInput {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub project_parameters: BTreeMap<String, String>,
}

impl PdkCallbackExecutionInput {
    pub fn validate(&self) -> Result<(), PdkCallbackError> {
        if self.project_parameters.len() > MAX_PDK_CALLBACK_PROJECT_PARAMETERS {
            return Err(PdkCallbackError::InvalidInput(format!(
                "project parameter count exceeds {MAX_PDK_CALLBACK_PROJECT_PARAMETERS}"
            )));
        }
        for (key, value) in &self.project_parameters {
            validate_host_key(
                "project parameter key",
                key,
                MAX_PDK_CALLBACK_PARAMETER_KEY_BYTES,
            )?;
            validate_host_text(
                "project parameter value",
                value,
                MAX_PDK_CALLBACK_PARAMETER_VALUE_BYTES,
            )?;
        }
        Ok(())
    }

    pub fn content_digest(&self) -> Result<ContentDigest, PdkCallbackError> {
        self.validate()?;
        let bytes = serde_json::to_vec(self)
            .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkCallbackExecutionReceipt {
    pub schema_version: u32,
    pub package_binding: PdkTechnologyBinding,
    pub archive_digest: ContentDigest,
    pub callback_id: String,
    pub callback_artifact_path: String,
    pub callback_artifact_digest: ContentDigest,
    pub abi_version: u32,
    pub execution_target: PdkExecutionTarget,
    pub input_digest: ContentDigest,
    pub output_digest: ContentDigest,
    pub fuel_limit: u64,
    pub fuel_consumed: u64,
    pub derived_metadata: BTreeMap<String, String>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct CallbackReceiptPayload<'a> {
    schema_version: u32,
    package_binding: &'a PdkTechnologyBinding,
    archive_digest: ContentDigest,
    callback_id: &'a str,
    callback_artifact_path: &'a str,
    callback_artifact_digest: ContentDigest,
    abi_version: u32,
    execution_target: PdkExecutionTarget,
    input_digest: ContentDigest,
    output_digest: ContentDigest,
    fuel_limit: u64,
    fuel_consumed: u64,
    derived_metadata: &'a BTreeMap<String, String>,
}

impl PdkCallbackExecutionReceipt {
    pub fn validate(&self) -> Result<(), PdkCallbackError> {
        if self.schema_version != PDK_CALLBACK_EXECUTION_RECEIPT_SCHEMA_VERSION {
            return Err(PdkCallbackError::InvalidReceipt(format!(
                "unsupported receipt schema {}",
                self.schema_version
            )));
        }
        validate_host_key("callback id", &self.callback_id, 256)?;
        validate_host_key(
            "callback artifact path",
            &self.callback_artifact_path,
            1_024,
        )?;
        if self.abi_version != PDK_CALLBACK_ABI_VERSION {
            return Err(PdkCallbackError::InvalidReceipt(format!(
                "callback ABI {} is not supported",
                self.abi_version
            )));
        }
        if self.fuel_limit != PDK_CALLBACK_FUEL_LIMIT || self.fuel_consumed > self.fuel_limit {
            return Err(PdkCallbackError::InvalidReceipt(
                "fuel identity is outside the callback execution contract".to_owned(),
            ));
        }
        validate_metadata(&self.derived_metadata)?;
        if digest_metadata(&self.derived_metadata)? != self.output_digest {
            return Err(PdkCallbackError::InvalidReceipt(
                "derived metadata digest does not match its payload".to_owned(),
            ));
        }
        if self.calculate_digest()? != self.receipt_digest {
            return Err(PdkCallbackError::InvalidReceipt(
                "receipt digest does not match its payload".to_owned(),
            ));
        }
        Ok(())
    }

    fn calculate_digest(&self) -> Result<ContentDigest, PdkCallbackError> {
        let payload = CallbackReceiptPayload {
            schema_version: self.schema_version,
            package_binding: &self.package_binding,
            archive_digest: self.archive_digest,
            callback_id: &self.callback_id,
            callback_artifact_path: &self.callback_artifact_path,
            callback_artifact_digest: self.callback_artifact_digest,
            abi_version: self.abi_version,
            execution_target: self.execution_target,
            input_digest: self.input_digest,
            output_digest: self.output_digest,
            fuel_limit: self.fuel_limit,
            fuel_consumed: self.fuel_consumed,
            derived_metadata: &self.derived_metadata,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

/// Project-owned evidence for one exact signed callback invocation.
///
/// The embedded execution receipt proves the sandbox/package boundary. This
/// outer receipt additionally binds the canonical input payload, active plan,
/// project revision transaction, operator identity, and append-only project
/// ledger position so derived metadata cannot be mistaken for ambient or
/// administrator-active state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectPdkCallbackReceipt {
    pub schema_version: u32,
    pub sequence: u64,
    pub project_id: ProjectId,
    pub from_project_revision: ObjectRevision,
    pub to_project_revision: ObjectRevision,
    pub plan_id: SimulationPlanId,
    pub plan_revision: ObjectRevision,
    pub actor_id: String,
    pub authority_id: String,
    pub reason: String,
    pub input: PdkCallbackExecutionInput,
    pub execution: PdkCallbackExecutionReceipt,
    pub previous_receipt_digest: Option<ContentDigest>,
    pub receipt_digest: ContentDigest,
}

#[derive(Serialize)]
struct ProjectPdkCallbackReceiptPayload<'a> {
    schema_version: u32,
    sequence: u64,
    project_id: ProjectId,
    from_project_revision: ObjectRevision,
    to_project_revision: ObjectRevision,
    plan_id: SimulationPlanId,
    plan_revision: ObjectRevision,
    actor_id: &'a str,
    authority_id: &'a str,
    reason: &'a str,
    input: &'a PdkCallbackExecutionInput,
    execution: &'a PdkCallbackExecutionReceipt,
    previous_receipt_digest: Option<ContentDigest>,
}

impl ProjectPdkCallbackReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn issue(
        sequence: u64,
        project_id: ProjectId,
        from_project_revision: ObjectRevision,
        to_project_revision: ObjectRevision,
        plan_id: SimulationPlanId,
        plan_revision: ObjectRevision,
        actor_id: String,
        authority_id: String,
        reason: String,
        input: PdkCallbackExecutionInput,
        execution: PdkCallbackExecutionReceipt,
        previous_receipt_digest: Option<ContentDigest>,
    ) -> Result<Self, PdkCallbackError> {
        let mut receipt = Self {
            schema_version: PROJECT_PDK_CALLBACK_RECEIPT_SCHEMA_VERSION,
            sequence,
            project_id,
            from_project_revision,
            to_project_revision,
            plan_id,
            plan_revision,
            actor_id,
            authority_id,
            reason,
            input,
            execution,
            previous_receipt_digest,
            receipt_digest: ContentDigest::from_bytes([0; 32]),
        };
        receipt.receipt_digest = receipt.calculate_digest()?;
        receipt.validate()?;
        Ok(receipt)
    }

    pub fn validate(&self) -> Result<(), PdkCallbackError> {
        if self.schema_version != PROJECT_PDK_CALLBACK_RECEIPT_SCHEMA_VERSION {
            return Err(PdkCallbackError::InvalidReceipt(format!(
                "unsupported project callback receipt schema {}",
                self.schema_version
            )));
        }
        if self.sequence == 0 {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback receipt sequence is zero".to_owned(),
            ));
        }
        if self.project_id.as_uuid().is_nil() {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback receipt has a nil project identity".to_owned(),
            ));
        }
        if self.from_project_revision.next().ok() != Some(self.to_project_revision) {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback receipt does not advance exactly one project revision".to_owned(),
            ));
        }
        validate_receipt_text("actor ID", &self.actor_id, 256)?;
        validate_receipt_text("authority ID", &self.authority_id, 256)?;
        validate_receipt_text("reason", &self.reason, 2_048)?;
        self.input.validate()?;
        self.execution.validate()?;
        if self.input.content_digest()? != self.execution.input_digest {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback input payload does not match the sandbox input digest".to_owned(),
            ));
        }
        if self.calculate_digest()? != self.receipt_digest {
            return Err(PdkCallbackError::InvalidReceipt(
                "project callback receipt digest does not match its payload".to_owned(),
            ));
        }
        Ok(())
    }

    fn calculate_digest(&self) -> Result<ContentDigest, PdkCallbackError> {
        let payload = ProjectPdkCallbackReceiptPayload {
            schema_version: self.schema_version,
            sequence: self.sequence,
            project_id: self.project_id,
            from_project_revision: self.from_project_revision,
            to_project_revision: self.to_project_revision,
            plan_id: self.plan_id,
            plan_revision: self.plan_revision,
            actor_id: &self.actor_id,
            authority_id: &self.authority_id,
            reason: &self.reason,
            input: &self.input,
            execution: &self.execution,
            previous_receipt_digest: self.previous_receipt_digest,
        };
        let bytes = serde_json::to_vec(&payload)
            .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
        Ok(content_digest(&bytes))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PdkCallbackError {
    #[error(transparent)]
    Technology(#[from] PdkTechnologyError),
    #[error("project PDK callback transaction failed: {0}")]
    ProjectTransaction(String),
    #[error("invalid PDK callback input: {0}")]
    InvalidInput(String),
    #[error("signed PDK callback '{0}' is not declared")]
    CallbackNotFound(String),
    #[error("signed PDK callback module is invalid: {0}")]
    InvalidModule(String),
    #[error("signed PDK callback capability violation: {0}")]
    CapabilityViolation(String),
    #[error("signed PDK callback could not be instantiated: {0}")]
    Instantiation(String),
    #[error("signed PDK callback execution failed: {0}")]
    Execution(String),
    #[error("signed PDK callback returned status {0}")]
    GuestStatus(i32),
    #[error("signed PDK callback host contract was violated: {0}")]
    HostViolation(String),
    #[error("invalid PDK callback receipt: {0}")]
    InvalidReceipt(String),
    #[error("PDK callback serialization failed: {0}")]
    Serialization(String),
}

fn validate_receipt_text(field: &str, value: &str, maximum: usize) -> Result<(), PdkCallbackError> {
    if value.is_empty() || value != value.trim() {
        return Err(PdkCallbackError::InvalidReceipt(format!(
            "project callback {field} must be nonempty and trimmed"
        )));
    }
    if value.len() > maximum || value.chars().any(char::is_control) {
        return Err(PdkCallbackError::InvalidReceipt(format!(
            "project callback {field} exceeds {maximum} bytes or contains control characters"
        )));
    }
    Ok(())
}

#[derive(Debug)]
struct SealedCallback {
    contract: PdkCallbackContract,
    artifact_digest: ContentDigest,
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct CallbackHostState {
    limits: StoreLimits,
    capabilities: BTreeSet<PdkCallbackCapability>,
    project_parameters: BTreeMap<String, String>,
    package_files: BTreeMap<String, Vec<u8>>,
    metadata: BTreeMap<String, String>,
    metadata_bytes: usize,
    fault: Option<String>,
}

pub(super) fn validate_signed_callbacks(
    archive: &SignedPdkTechnologyArchive,
    package: &ValidatedPdkTechnologyPackage,
) -> Result<(), String> {
    let package_files = seal_archive_files(archive, package).map_err(|error| error.to_string())?;
    for contract in &package.manifest().callbacks {
        let callback =
            seal_callback(package, contract, &package_files).map_err(|error| error.to_string())?;
        validate_module_contract(&callback.contract, &callback.bytes)
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub(super) fn execute_signed_callback(
    archive: &SignedPdkTechnologyArchive,
    package: &ValidatedPdkTechnologyPackage,
    callback_id: &str,
    input: &PdkCallbackExecutionInput,
) -> Result<PdkCallbackExecutionReceipt, PdkCallbackError> {
    input.validate()?;
    let archive_bytes = serde_json::to_vec(archive)
        .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
    if content_digest(&archive_bytes) != package.archive_digest() {
        return Err(PdkCallbackError::Technology(
            PdkTechnologyError::NotRuntimeValidated(
                "callback archive bytes no longer match the trusted archive digest".to_owned(),
            ),
        ));
    }
    let contract = package
        .manifest()
        .callbacks
        .iter()
        .find(|contract| contract.callback_id.eq_ignore_ascii_case(callback_id))
        .ok_or_else(|| PdkCallbackError::CallbackNotFound(callback_id.to_owned()))?;
    let package_files = seal_archive_files(archive, package)?;
    let callback = seal_callback(package, contract, &package_files)?;
    let (metadata, fuel_consumed) = execute_module(&callback, input, package_files)?;
    let output_digest = digest_metadata(&metadata)?;
    let mut receipt = PdkCallbackExecutionReceipt {
        schema_version: PDK_CALLBACK_EXECUTION_RECEIPT_SCHEMA_VERSION,
        package_binding: package.binding(),
        archive_digest: package.archive_digest(),
        callback_id: callback.contract.callback_id,
        callback_artifact_path: callback.contract.artifact_path,
        callback_artifact_digest: callback.artifact_digest,
        abi_version: callback.contract.abi_version,
        execution_target: current_execution_target(),
        input_digest: input.content_digest()?,
        output_digest,
        fuel_limit: PDK_CALLBACK_FUEL_LIMIT,
        fuel_consumed,
        derived_metadata: metadata,
        receipt_digest: ContentDigest::from_bytes([0; 32]),
    };
    receipt.receipt_digest = receipt.calculate_digest()?;
    receipt.validate()?;
    Ok(receipt)
}

fn seal_archive_files(
    archive: &SignedPdkTechnologyArchive,
    package: &ValidatedPdkTechnologyPackage,
) -> Result<BTreeMap<String, Vec<u8>>, PdkCallbackError> {
    let archive_bytes = serde_json::to_vec(archive)
        .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
    if content_digest(&archive_bytes) != package.archive_digest() {
        return Err(PdkTechnologyError::NotRuntimeValidated(
            "callback archive bytes do not match the trusted archive identity".to_owned(),
        )
        .into());
    }
    let declared = package
        .manifest()
        .artifacts
        .iter()
        .map(|artifact| (artifact.path.to_ascii_lowercase(), artifact))
        .collect::<BTreeMap<_, _>>();
    let mut files = BTreeMap::new();
    let mut total = 0usize;
    for file in &archive.files {
        let key = file.path.to_ascii_lowercase();
        let artifact = declared
            .get(&key)
            .ok_or_else(|| PdkTechnologyError::UndeclaredArtifact(file.path.clone()))?;
        let bytes = STANDARD.decode(&file.content_base64).map_err(|error| {
            PdkTechnologyError::InvalidBase64 {
                field: format!("files[{}].content_base64", file.path),
                detail: error.to_string(),
            }
        })?;
        if bytes.len() > MAX_PDK_ARTIFACT_BYTES {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "artifact '{}' exceeds {MAX_PDK_ARTIFACT_BYTES} decoded bytes",
                file.path
            ))
            .into());
        }
        total = total.checked_add(bytes.len()).ok_or_else(|| {
            PdkTechnologyError::LimitExceeded("callback package byte count overflow".to_owned())
        })?;
        if total > MAX_PDK_TOTAL_ARTIFACT_BYTES {
            return Err(PdkTechnologyError::LimitExceeded(format!(
                "callback package exceeds {MAX_PDK_TOTAL_ARTIFACT_BYTES} decoded bytes"
            ))
            .into());
        }
        verify_artifact_bytes(artifact, &bytes, package)?;
        if files.insert(key, bytes).is_some() {
            return Err(PdkTechnologyError::Duplicate(format!(
                "callback package repeats path '{}'",
                file.path
            ))
            .into());
        }
    }
    if files.len() != declared.len() {
        let missing = declared
            .keys()
            .find(|path| !files.contains_key(*path))
            .expect("different map lengths imply a missing declared artifact");
        return Err(PdkTechnologyError::MissingArtifact(missing.clone()).into());
    }
    Ok(files)
}

fn verify_artifact_bytes(
    artifact: &PdkTechnologyArtifact,
    bytes: &[u8],
    package: &ValidatedPdkTechnologyPackage,
) -> Result<(), PdkCallbackError> {
    if u64::try_from(bytes.len()).ok() != Some(artifact.size_bytes) {
        return Err(PdkTechnologyError::ArtifactSizeMismatch {
            path: artifact.path.clone(),
            declared: artifact.size_bytes,
            actual: bytes.len(),
        }
        .into());
    }
    let actual = content_digest(bytes);
    if actual != artifact.sha256 || package.artifact_digests().get(&artifact.path) != Some(&actual)
    {
        return Err(PdkTechnologyError::ArtifactDigestMismatch {
            path: artifact.path.clone(),
            declared: artifact.sha256,
            actual,
        }
        .into());
    }
    Ok(())
}

fn seal_callback(
    package: &ValidatedPdkTechnologyPackage,
    contract: &PdkCallbackContract,
    package_files: &BTreeMap<String, Vec<u8>>,
) -> Result<SealedCallback, PdkCallbackError> {
    let artifact = package
        .manifest()
        .artifacts
        .iter()
        .find(|artifact| artifact.path.eq_ignore_ascii_case(&contract.artifact_path))
        .ok_or_else(|| PdkTechnologyError::MissingArtifact(contract.artifact_path.clone()))?;
    let bytes = package_files
        .get(&contract.artifact_path.to_ascii_lowercase())
        .ok_or_else(|| PdkTechnologyError::MissingArtifact(contract.artifact_path.clone()))?;
    if bytes.len() > MAX_PDK_CALLBACK_ARTIFACT_BYTES {
        return Err(PdkTechnologyError::LimitExceeded(format!(
            "callback '{}' exceeds {MAX_PDK_CALLBACK_ARTIFACT_BYTES} bytes",
            contract.callback_id
        ))
        .into());
    }
    verify_artifact_bytes(artifact, bytes, package)?;
    Ok(SealedCallback {
        contract: contract.clone(),
        artifact_digest: artifact.sha256,
        bytes: bytes.clone(),
    })
}

fn callback_engine() -> Engine {
    let mut config = Config::default();
    config.consume_fuel(true);
    config.enforced_limits(EnforcedLimits::strict());
    Engine::new(&config)
}

fn validate_module_contract(
    contract: &PdkCallbackContract,
    bytes: &[u8],
) -> Result<(), PdkCallbackError> {
    if contract.abi_version != PDK_CALLBACK_ABI_VERSION {
        return Err(PdkCallbackError::InvalidModule(format!(
            "callback '{}' declares unsupported ABI {}",
            contract.callback_id, contract.abi_version
        )));
    }
    let engine = callback_engine();
    let module = Module::new(&engine, bytes)
        .map_err(|error| PdkCallbackError::InvalidModule(error.to_string()))?;
    let capabilities = contract
        .capabilities
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let mut imports = BTreeSet::new();
    for import in module.imports() {
        let identity = (import.module().to_owned(), import.name().to_owned());
        if !imports.insert(identity) {
            return Err(PdkCallbackError::InvalidModule(format!(
                "duplicate import {}/{}",
                import.module(),
                import.name()
            )));
        }
        let (required, params, results) = host_import_contract(import.module(), import.name())
            .ok_or_else(|| {
                PdkCallbackError::CapabilityViolation(format!(
                    "import {}/{} is outside the RSpice callback ABI",
                    import.module(),
                    import.name()
                ))
            })?;
        if !capabilities.contains(&required) {
            return Err(PdkCallbackError::CapabilityViolation(format!(
                "import {}/{} requires signed capability {:?}",
                import.module(),
                import.name(),
                required
            )));
        }
        require_function_type(import.ty(), params, results, import.name())?;
    }
    match module.get_export(GUEST_MEMORY_EXPORT) {
        Some(ExternType::Memory(_)) => {}
        Some(_) => {
            return Err(PdkCallbackError::InvalidModule(
                "export 'memory' is not a linear memory".to_owned(),
            ));
        }
        None => {
            return Err(PdkCallbackError::InvalidModule(
                "required linear-memory export 'memory' is absent".to_owned(),
            ));
        }
    }
    let entrypoint = module.get_export(&contract.entrypoint).ok_or_else(|| {
        PdkCallbackError::InvalidModule(format!(
            "entrypoint export '{}' is absent",
            contract.entrypoint
        ))
    })?;
    require_function_type(&entrypoint, &[], &[ValType::I32], &contract.entrypoint)
}

fn require_function_type(
    ty: &ExternType,
    params: &[ValType],
    results: &[ValType],
    name: &str,
) -> Result<(), PdkCallbackError> {
    let ExternType::Func(ty) = ty else {
        return Err(PdkCallbackError::InvalidModule(format!(
            "'{name}' is not a function"
        )));
    };
    if ty.params() != params || ty.results() != results {
        return Err(PdkCallbackError::InvalidModule(format!(
            "function '{name}' has an incompatible ABI signature"
        )));
    }
    Ok(())
}

fn host_import_contract(
    module: &str,
    name: &str,
) -> Option<(
    PdkCallbackCapability,
    &'static [ValType],
    &'static [ValType],
)> {
    const I32_2: &[ValType] = &[ValType::I32, ValType::I32];
    const I32_4: &[ValType] = &[ValType::I32, ValType::I32, ValType::I32, ValType::I32];
    const I32_RESULT: &[ValType] = &[ValType::I32];
    if module != HOST_ABI_MODULE {
        return None;
    }
    match name {
        "project_parameter_len" => Some((
            PdkCallbackCapability::ReadProjectParameters,
            I32_2,
            I32_RESULT,
        )),
        "project_parameter_read" => Some((
            PdkCallbackCapability::ReadProjectParameters,
            I32_4,
            I32_RESULT,
        )),
        "package_file_len" => Some((PdkCallbackCapability::ReadPackage, I32_2, I32_RESULT)),
        "package_file_read" => Some((PdkCallbackCapability::ReadPackage, I32_4, I32_RESULT)),
        "emit_metadata" => Some((
            PdkCallbackCapability::WriteDerivedMetadata,
            I32_4,
            I32_RESULT,
        )),
        _ => None,
    }
}

fn execute_module(
    callback: &SealedCallback,
    input: &PdkCallbackExecutionInput,
    package_files: BTreeMap<String, Vec<u8>>,
) -> Result<(BTreeMap<String, String>, u64), PdkCallbackError> {
    validate_module_contract(&callback.contract, &callback.bytes)?;
    let engine = callback_engine();
    let module = Module::new(&engine, callback.bytes.as_slice())
        .map_err(|error| PdkCallbackError::InvalidModule(error.to_string()))?;
    let limits = StoreLimitsBuilder::new()
        .memory_size(PDK_CALLBACK_MEMORY_LIMIT_BYTES)
        .table_elements(1_024)
        .instances(1)
        .tables(1)
        .memories(1)
        .trap_on_grow_failure(true)
        .build();
    let mut store = Store::new(
        &engine,
        CallbackHostState {
            limits,
            capabilities: callback.contract.capabilities.iter().copied().collect(),
            project_parameters: input.project_parameters.clone(),
            package_files,
            metadata: BTreeMap::new(),
            metadata_bytes: 0,
            fault: None,
        },
    );
    store.limiter(|state| &mut state.limits);
    store
        .set_fuel(PDK_CALLBACK_FUEL_LIMIT)
        .map_err(|error| PdkCallbackError::Execution(error.to_string()))?;
    let mut linker = Linker::new(&engine);
    define_host_abi(&mut linker)?;
    let instance = linker
        .instantiate_and_start(&mut store, &module)
        .map_err(|error| PdkCallbackError::Instantiation(error.to_string()))?;
    if let Some(fault) = store.data().fault.clone() {
        return Err(PdkCallbackError::HostViolation(fault));
    }
    let entrypoint = instance
        .get_typed_func::<(), i32>(&store, &callback.contract.entrypoint)
        .map_err(|error| PdkCallbackError::InvalidModule(error.to_string()))?;
    let status = entrypoint
        .call(&mut store, ())
        .map_err(|error| PdkCallbackError::Execution(error.to_string()))?;
    if let Some(fault) = store.data().fault.clone() {
        return Err(PdkCallbackError::HostViolation(fault));
    }
    if status != 0 {
        return Err(PdkCallbackError::GuestStatus(status));
    }
    let remaining = store
        .get_fuel()
        .map_err(|error| PdkCallbackError::Execution(error.to_string()))?;
    let consumed = PDK_CALLBACK_FUEL_LIMIT.saturating_sub(remaining);
    let metadata = store.data().metadata.clone();
    validate_metadata(&metadata)?;
    Ok((metadata, consumed))
}

fn define_host_abi(linker: &mut Linker<CallbackHostState>) -> Result<(), PdkCallbackError> {
    linker
        .func_wrap(
            HOST_ABI_MODULE,
            "project_parameter_len",
            |mut caller: Caller<'_, CallbackHostState>, key_ptr: i32, key_len: i32| -> i32 {
                if !caller
                    .data()
                    .capabilities
                    .contains(&PdkCallbackCapability::ReadProjectParameters)
                {
                    return HOST_ERROR_FORBIDDEN;
                }
                let Some(key) = read_guest_utf8(
                    &mut caller,
                    key_ptr,
                    key_len,
                    MAX_PDK_CALLBACK_PARAMETER_KEY_BYTES,
                ) else {
                    return HOST_ERROR_INVALID_ARGUMENT;
                };
                caller
                    .data()
                    .project_parameters
                    .get(&key)
                    .and_then(|value| i32::try_from(value.len()).ok())
                    .unwrap_or(HOST_ERROR_NOT_FOUND)
            },
        )
        .map_err(|error| PdkCallbackError::Instantiation(error.to_string()))?;
    linker
        .func_wrap(
            HOST_ABI_MODULE,
            "project_parameter_read",
            |mut caller: Caller<'_, CallbackHostState>,
             key_ptr: i32,
             key_len: i32,
             dst_ptr: i32,
             dst_capacity: i32|
             -> i32 {
                if !caller
                    .data()
                    .capabilities
                    .contains(&PdkCallbackCapability::ReadProjectParameters)
                {
                    return HOST_ERROR_FORBIDDEN;
                }
                let Some(key) = read_guest_utf8(
                    &mut caller,
                    key_ptr,
                    key_len,
                    MAX_PDK_CALLBACK_PARAMETER_KEY_BYTES,
                ) else {
                    return HOST_ERROR_INVALID_ARGUMENT;
                };
                let Some(value) = caller.data().project_parameters.get(&key).cloned() else {
                    return HOST_ERROR_NOT_FOUND;
                };
                write_guest_bytes(&mut caller, dst_ptr, dst_capacity, value.as_bytes())
            },
        )
        .map_err(|error| PdkCallbackError::Instantiation(error.to_string()))?;
    linker
        .func_wrap(
            HOST_ABI_MODULE,
            "package_file_len",
            |mut caller: Caller<'_, CallbackHostState>, path_ptr: i32, path_len: i32| -> i32 {
                if !caller
                    .data()
                    .capabilities
                    .contains(&PdkCallbackCapability::ReadPackage)
                {
                    return HOST_ERROR_FORBIDDEN;
                }
                let Some(path) = read_guest_utf8(&mut caller, path_ptr, path_len, 1_024) else {
                    return HOST_ERROR_INVALID_ARGUMENT;
                };
                caller
                    .data()
                    .package_files
                    .get(&path.to_ascii_lowercase())
                    .and_then(|value| i32::try_from(value.len()).ok())
                    .unwrap_or(HOST_ERROR_NOT_FOUND)
            },
        )
        .map_err(|error| PdkCallbackError::Instantiation(error.to_string()))?;
    linker
        .func_wrap(
            HOST_ABI_MODULE,
            "package_file_read",
            |mut caller: Caller<'_, CallbackHostState>,
             path_ptr: i32,
             path_len: i32,
             dst_ptr: i32,
             dst_capacity: i32|
             -> i32 {
                if !caller
                    .data()
                    .capabilities
                    .contains(&PdkCallbackCapability::ReadPackage)
                {
                    return HOST_ERROR_FORBIDDEN;
                }
                let Some(path) = read_guest_utf8(&mut caller, path_ptr, path_len, 1_024) else {
                    return HOST_ERROR_INVALID_ARGUMENT;
                };
                let Some(value) = caller
                    .data()
                    .package_files
                    .get(&path.to_ascii_lowercase())
                    .cloned()
                else {
                    return HOST_ERROR_NOT_FOUND;
                };
                write_guest_bytes(&mut caller, dst_ptr, dst_capacity, &value)
            },
        )
        .map_err(|error| PdkCallbackError::Instantiation(error.to_string()))?;
    linker
        .func_wrap(
            HOST_ABI_MODULE,
            "emit_metadata",
            |mut caller: Caller<'_, CallbackHostState>,
             key_ptr: i32,
             key_len: i32,
             value_ptr: i32,
             value_len: i32|
             -> i32 {
                if !caller
                    .data()
                    .capabilities
                    .contains(&PdkCallbackCapability::WriteDerivedMetadata)
                {
                    return HOST_ERROR_FORBIDDEN;
                }
                let Some(key) = read_guest_utf8(
                    &mut caller,
                    key_ptr,
                    key_len,
                    MAX_PDK_CALLBACK_METADATA_KEY_BYTES,
                ) else {
                    return HOST_ERROR_INVALID_ARGUMENT;
                };
                let Some(value) = read_guest_utf8(
                    &mut caller,
                    value_ptr,
                    value_len,
                    MAX_PDK_CALLBACK_METADATA_VALUE_BYTES,
                ) else {
                    return HOST_ERROR_INVALID_ARGUMENT;
                };
                if validate_host_key(
                    "derived metadata key",
                    &key,
                    MAX_PDK_CALLBACK_METADATA_KEY_BYTES,
                )
                .is_err()
                    || validate_host_text(
                        "derived metadata value",
                        &value,
                        MAX_PDK_CALLBACK_METADATA_VALUE_BYTES,
                    )
                    .is_err()
                {
                    set_host_fault(
                        &mut caller,
                        "callback emitted invalid derived metadata".to_owned(),
                    );
                    return HOST_ERROR_INVALID_ARGUMENT;
                }
                if caller.data().metadata.contains_key(&key) {
                    set_host_fault(
                        &mut caller,
                        format!("callback emitted duplicate metadata key '{key}'"),
                    );
                    return HOST_ERROR_INVALID_ARGUMENT;
                }
                let added = key.len().saturating_add(value.len());
                if caller.data().metadata.len() >= MAX_PDK_CALLBACK_METADATA_ENTRIES
                    || caller.data().metadata_bytes.saturating_add(added)
                        > MAX_PDK_CALLBACK_METADATA_TOTAL_BYTES
                {
                    set_host_fault(
                        &mut caller,
                        "callback derived metadata exceeds configured limits".to_owned(),
                    );
                    return HOST_ERROR_LIMIT;
                }
                let state = caller.data_mut();
                state.metadata_bytes += added;
                state.metadata.insert(key, value);
                0
            },
        )
        .map_err(|error| PdkCallbackError::Instantiation(error.to_string()))?;
    Ok(())
}

fn read_guest_utf8(
    caller: &mut Caller<'_, CallbackHostState>,
    pointer: i32,
    length: i32,
    maximum: usize,
) -> Option<String> {
    let Ok(offset) = usize::try_from(pointer) else {
        set_host_fault(
            caller,
            "guest supplied a negative source pointer".to_owned(),
        );
        return None;
    };
    let Ok(length) = usize::try_from(length) else {
        set_host_fault(caller, "guest supplied a negative source length".to_owned());
        return None;
    };
    if length > maximum {
        set_host_fault(caller, format!("guest read exceeds {maximum} bytes"));
        return None;
    }
    let Some(memory) = caller
        .get_export(GUEST_MEMORY_EXPORT)
        .and_then(Extern::into_memory)
    else {
        set_host_fault(caller, "guest memory export disappeared".to_owned());
        return None;
    };
    let mut bytes = vec![0; length];
    if let Err(error) = memory.read(&*caller, offset, &mut bytes) {
        set_host_fault(caller, format!("guest memory read failed: {error}"));
        return None;
    }
    match String::from_utf8(bytes) {
        Ok(value) => Some(value),
        Err(error) => {
            set_host_fault(caller, format!("guest supplied non-UTF-8 text: {error}"));
            None
        }
    }
}

fn write_guest_bytes(
    caller: &mut Caller<'_, CallbackHostState>,
    pointer: i32,
    capacity: i32,
    value: &[u8],
) -> i32 {
    let Ok(offset) = usize::try_from(pointer) else {
        set_host_fault(
            caller,
            "guest supplied a negative destination pointer".to_owned(),
        );
        return HOST_ERROR_INVALID_ARGUMENT;
    };
    let Ok(capacity) = usize::try_from(capacity) else {
        set_host_fault(
            caller,
            "guest supplied a negative destination capacity".to_owned(),
        );
        return HOST_ERROR_INVALID_ARGUMENT;
    };
    if capacity < value.len() {
        return HOST_ERROR_BUFFER_TOO_SMALL;
    }
    let Some(memory) = caller
        .get_export(GUEST_MEMORY_EXPORT)
        .and_then(Extern::into_memory)
    else {
        set_host_fault(caller, "guest memory export disappeared".to_owned());
        return HOST_ERROR_INVALID_ARGUMENT;
    };
    if let Err(error) = memory.write(&mut *caller, offset, value) {
        set_host_fault(caller, format!("guest memory write failed: {error}"));
        return HOST_ERROR_INVALID_ARGUMENT;
    }
    i32::try_from(value.len()).unwrap_or(HOST_ERROR_LIMIT)
}

fn set_host_fault(caller: &mut Caller<'_, CallbackHostState>, detail: String) {
    if caller.data().fault.is_none() {
        caller.data_mut().fault = Some(detail);
    }
}

fn validate_metadata(metadata: &BTreeMap<String, String>) -> Result<(), PdkCallbackError> {
    if metadata.len() > MAX_PDK_CALLBACK_METADATA_ENTRIES {
        return Err(PdkCallbackError::InvalidReceipt(format!(
            "derived metadata contains more than {MAX_PDK_CALLBACK_METADATA_ENTRIES} entries"
        )));
    }
    let mut total = 0usize;
    for (key, value) in metadata {
        validate_host_key(
            "derived metadata key",
            key,
            MAX_PDK_CALLBACK_METADATA_KEY_BYTES,
        )?;
        validate_host_text(
            "derived metadata value",
            value,
            MAX_PDK_CALLBACK_METADATA_VALUE_BYTES,
        )?;
        total = total.saturating_add(key.len()).saturating_add(value.len());
    }
    if total > MAX_PDK_CALLBACK_METADATA_TOTAL_BYTES {
        return Err(PdkCallbackError::InvalidReceipt(format!(
            "derived metadata exceeds {MAX_PDK_CALLBACK_METADATA_TOTAL_BYTES} bytes"
        )));
    }
    Ok(())
}

fn validate_host_key(field: &str, value: &str, maximum: usize) -> Result<(), PdkCallbackError> {
    validate_host_text(field, value, maximum)?;
    if !value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(byte, b'_' | b'-' | b'.' | b':' | b'/' | b'[' | b']' | b'@')
    }) {
        return Err(PdkCallbackError::InvalidInput(format!(
            "{field} contains unsupported characters"
        )));
    }
    Ok(())
}

fn validate_host_text(field: &str, value: &str, maximum: usize) -> Result<(), PdkCallbackError> {
    if value.is_empty() || value.len() > maximum || value.chars().any(char::is_control) {
        return Err(PdkCallbackError::InvalidInput(format!(
            "{field} must contain 1..={maximum} non-control UTF-8 bytes"
        )));
    }
    Ok(())
}

fn digest_metadata(metadata: &BTreeMap<String, String>) -> Result<ContentDigest, PdkCallbackError> {
    validate_metadata(metadata)?;
    let bytes = serde_json::to_vec(metadata)
        .map_err(|error| PdkCallbackError::Serialization(error.to_string()))?;
    Ok(content_digest(&bytes))
}

fn content_digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::pdk_config::{
        PdkAdministrativeAuthority, PdkPublisherTrustStore, PdkTechnologyRegistry,
    };
    use ed25519_dalek::{Signer as _, SigningKey};

    fn callback_archive(
        wat_source: &str,
        capabilities: Vec<PdkCallbackCapability>,
    ) -> (Vec<u8>, PdkPublisherTrustStore, PdkAdministrativeAuthority) {
        let (bytes, trust, authority) = super::super::technology_package::tests::fixture_archive();
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: super::super::technology_package::PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        let callback_bytes = wat::parse_str(wat_source).unwrap();
        let callback = &mut manifest.callbacks[0];
        callback.capabilities = capabilities;
        callback.abi_version = PDK_CALLBACK_ABI_VERSION;
        callback.entrypoint = "derive".to_owned();
        let artifact = manifest
            .artifacts
            .iter_mut()
            .find(|artifact| artifact.path == callback.artifact_path)
            .unwrap();
        artifact.size_bytes = u64::try_from(callback_bytes.len()).unwrap();
        artifact.sha256 = content_digest(&callback_bytes);
        let file = archive
            .files
            .iter_mut()
            .find(|file| file.path == callback.artifact_path)
            .unwrap();
        file.content_base64 = STANDARD.encode(callback_bytes);
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(
            SigningKey::from_bytes(&[0x42; 32])
                .sign(&manifest_bytes)
                .to_bytes(),
        );
        (serde_json::to_vec(&archive).unwrap(), trust, authority)
    }

    fn install(
        bytes: &[u8],
        trust: &PdkPublisherTrustStore,
        authority: &PdkAdministrativeAuthority,
    ) -> (PdkTechnologyRegistry, PdkTechnologyBinding, ContentDigest) {
        let mut registry = PdkTechnologyRegistry::default();
        registry
            .install_archive_bytes(bytes, trust, authority, "Install callback fixture")
            .unwrap();
        let package = registry.validated_packages()[0].clone();
        (registry, package.binding(), package.archive_digest())
    }

    #[test]
    fn signed_callback_executes_with_capabilities_and_verifiable_provenance() {
        let wat = r#"(module
            (import "rspice" "project_parameter_len" (func $len (param i32 i32) (result i32)))
            (import "rspice" "project_parameter_read" (func $read (param i32 i32 i32 i32) (result i32)))
            (import "rspice" "emit_metadata" (func $emit (param i32 i32 i32 i32) (result i32)))
            (memory (export "memory") 1 2)
            (data (i32.const 0) "width")
            (data (i32.const 16) "derived.width")
            (func (export "derive") (result i32)
                (local $length i32)
                i32.const 0
                i32.const 5
                call $len
                local.tee $length
                i32.const 0
                i32.lt_s
                if
                    i32.const 10
                    return
                end
                i32.const 0
                i32.const 5
                i32.const 64
                i32.const 64
                call $read
                i32.const 0
                i32.lt_s
                if
                    i32.const 11
                    return
                end
                i32.const 16
                i32.const 13
                i32.const 64
                local.get $length
                call $emit))"#;
        let (bytes, trust, authority) = callback_archive(
            wat,
            vec![
                PdkCallbackCapability::ReadProjectParameters,
                PdkCallbackCapability::WriteDerivedMetadata,
            ],
        );
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let input = PdkCallbackExecutionInput {
            project_parameters: BTreeMap::from([("width".to_owned(), "2.5u".to_owned())]),
        };
        let receipt = registry
            .execute_callback_for_binding(&binding, archive_digest, "DERIVE-DEVICE", &input)
            .unwrap();
        receipt.validate().unwrap();
        assert_eq!(
            receipt.derived_metadata.get("derived.width"),
            Some(&"2.5u".to_owned())
        );
        assert_eq!(receipt.package_binding, binding);
        assert_eq!(receipt.archive_digest, archive_digest);
        assert!(receipt.fuel_consumed > 0);

        let repeated = registry
            .execute_callback_for_binding(&binding, archive_digest, "derive-device", &input)
            .unwrap();
        assert_eq!(receipt, repeated);
    }

    #[test]
    fn callback_import_without_signed_capability_is_rejected_at_install() {
        let wat = r#"(module
            (import "rspice" "project_parameter_len" (func (param i32 i32) (result i32)))
            (memory (export "memory") 1 1)
            (func (export "derive") (result i32) i32.const 0))"#;
        let (bytes, trust, authority) = callback_archive(wat, Vec::new());
        let mut registry = PdkTechnologyRegistry::default();
        let error = registry
            .install_archive_bytes(&bytes, &trust, &authority, "Reject unauthorized import")
            .unwrap_err();
        assert!(error.to_string().contains("requires signed capability"));
        assert!(registry.archives().is_empty());
    }

    #[test]
    fn callback_reads_only_exact_signed_package_bytes() {
        let wat = r#"(module
            (import "rspice" "package_file_len" (func $len (param i32 i32) (result i32)))
            (import "rspice" "package_file_read" (func $read (param i32 i32 i32 i32) (result i32)))
            (import "rspice" "emit_metadata" (func $emit (param i32 i32 i32 i32) (result i32)))
            (memory (export "memory") 1 2)
            (data (i32.const 0) "models/demo.lib")
            (data (i32.const 32) "model.prefix")
            (func (export "derive") (result i32)
                i32.const 0
                i32.const 15
                call $len
                i32.const 0
                i32.lt_s
                if
                    i32.const 20
                    return
                end
                i32.const 0
                i32.const 15
                i32.const 128
                i32.const 1024
                call $read
                i32.const 0
                i32.lt_s
                if
                    i32.const 21
                    return
                end
                i32.const 32
                i32.const 12
                i32.const 128
                i32.const 4
                call $emit))"#;
        let (bytes, trust, authority) = callback_archive(
            wat,
            vec![
                PdkCallbackCapability::ReadPackage,
                PdkCallbackCapability::WriteDerivedMetadata,
            ],
        );
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let receipt = registry
            .execute_callback_for_binding(
                &binding,
                archive_digest,
                "derive-device",
                &PdkCallbackExecutionInput::default(),
            )
            .unwrap();
        assert_eq!(
            receipt.derived_metadata.get("model.prefix"),
            Some(&".lib".to_owned())
        );
        receipt.validate().unwrap();
    }

    #[test]
    fn executable_callback_contract_rejects_schema_downgrade() {
        let (bytes, trust, authority) = callback_archive(
            r#"(module
                (memory (export "memory") 1 1)
                (func (export "derive") (result i32) i32.const 0))"#,
            Vec::new(),
        );
        let signing_key = SigningKey::from_bytes(&[0x42; 32]);
        let mut archive: SignedPdkTechnologyArchive = serde_json::from_slice(&bytes).unwrap();
        let mut manifest: super::super::technology_package::PdkTechnologyManifest =
            serde_json::from_slice(&STANDARD.decode(&archive.manifest_base64).unwrap()).unwrap();
        manifest.schema_version = 2;
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        archive.manifest_base64 = STANDARD.encode(&manifest_bytes);
        archive.signature_base64 = STANDARD.encode(signing_key.sign(&manifest_bytes).to_bytes());
        let bytes = serde_json::to_vec(&archive).unwrap();
        let mut registry = PdkTechnologyRegistry::default();
        let error = registry
            .install_archive_bytes(
                &bytes,
                &trust,
                &authority,
                "Reject callback schema downgrade",
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("executable callbacks require manifest schema 3")
        );
    }

    #[test]
    fn callback_fuel_and_memory_limits_fail_closed() {
        let infinite = r#"(module
            (memory (export "memory") 1 1)
            (func (export "derive") (result i32)
                (loop $again br $again)
                i32.const 0))"#;
        let (bytes, trust, authority) = callback_archive(infinite, Vec::new());
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let error = registry
            .execute_callback_for_binding(
                &binding,
                archive_digest,
                "derive-device",
                &PdkCallbackExecutionInput::default(),
            )
            .unwrap_err();
        assert!(matches!(error, PdkCallbackError::Execution(_)));

        let oversized = r#"(module
            (memory (export "memory") 200 200)
            (func (export "derive") (result i32) i32.const 0))"#;
        let (bytes, trust, authority) = callback_archive(oversized, Vec::new());
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let error = registry
            .execute_callback_for_binding(
                &binding,
                archive_digest,
                "derive-device",
                &PdkCallbackExecutionInput::default(),
            )
            .unwrap_err();
        assert!(matches!(error, PdkCallbackError::Instantiation(_)));
    }

    #[test]
    fn callback_host_rejects_invalid_memory_even_when_guest_ignores_status() {
        let wat = r#"(module
            (import "rspice" "emit_metadata" (func $emit (param i32 i32 i32 i32) (result i32)))
            (memory (export "memory") 1 1)
            (func (export "derive") (result i32)
                i32.const 65530
                i32.const 32
                i32.const 0
                i32.const 1
                call $emit
                drop
                i32.const 0))"#;
        let (bytes, trust, authority) =
            callback_archive(wat, vec![PdkCallbackCapability::WriteDerivedMetadata]);
        let (registry, binding, archive_digest) = install(&bytes, &trust, &authority);
        let error = registry
            .execute_callback_for_binding(
                &binding,
                archive_digest,
                "derive-device",
                &PdkCallbackExecutionInput::default(),
            )
            .unwrap_err();
        assert!(matches!(error, PdkCallbackError::HostViolation(_)));
    }
}
