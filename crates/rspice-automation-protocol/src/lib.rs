//! Versioned, bounded protocol shared by native and browser Automation workers.
//!
//! The UI and worker exchange immutable source snapshots and typed events. No
//! request carries host paths or ambient authority; access to project, result,
//! artifact, file, network, process, environment, or clipboard resources is
//! represented by opaque broker capabilities.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

pub const PROTOCOL_VERSION: ProtocolVersion = ProtocolVersion { major: 1, minor: 4 };
pub const MAX_DOCUMENTS: usize = 10_000;
pub const MAX_SOURCE_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_PATH_BYTES: usize = 1_024;
pub const MAX_TEXT_FIELD_BYTES: usize = 1024 * 1024;
pub const MAX_BREAKPOINTS: usize = 100_000;
pub const MAX_CAPABILITIES: usize = 4_096;
pub const MAX_WATCH_EXPRESSION_BYTES: usize = 16_384;
/// Maximum encoded native request/event frame. This covers the complete
/// bounded source closure plus protocol overhead without permitting an
/// attacker-controlled allocation.
pub const MAX_ENVELOPE_BYTES: usize = MAX_SOURCE_BYTES + 8 * 1024 * 1024;

/// Length-prefixed native transport. The payload is UTF-8 JSON so a worker
/// crash can be diagnosed with standard tooling; framing and domain
/// validation, rather than EOF or newline conventions, define boundaries.
/// Browser workers use the same serde representation through structured clone.
pub mod native_codec {
    use std::io::{self, Read, Write};

    use serde::{Serialize, de::DeserializeOwned};

    use super::MAX_ENVELOPE_BYTES;

    pub fn write_frame<W, T>(writer: &mut W, value: &T) -> Result<(), CodecError>
    where
        W: Write,
        T: Serialize,
    {
        let payload = serde_json::to_vec(value).map_err(CodecError::Encode)?;
        if payload.len() > MAX_ENVELOPE_BYTES {
            return Err(CodecError::FrameTooLarge {
                actual: payload.len(),
                maximum: MAX_ENVELOPE_BYTES,
            });
        }
        let length = u32::try_from(payload.len()).map_err(|_| CodecError::FrameTooLarge {
            actual: payload.len(),
            maximum: MAX_ENVELOPE_BYTES,
        })?;
        writer.write_all(&length.to_be_bytes())?;
        writer.write_all(&payload)?;
        writer.flush()?;
        Ok(())
    }

    pub fn read_frame<R, T>(reader: &mut R) -> Result<Option<T>, CodecError>
    where
        R: Read,
        T: DeserializeOwned,
    {
        let mut length = [0_u8; 4];
        match read_header(reader, &mut length)? {
            HeaderRead::Eof => return Ok(None),
            HeaderRead::Complete => {}
        }
        let length = u32::from_be_bytes(length) as usize;
        if length == 0 {
            return Err(CodecError::EmptyFrame);
        }
        if length > MAX_ENVELOPE_BYTES {
            return Err(CodecError::FrameTooLarge {
                actual: length,
                maximum: MAX_ENVELOPE_BYTES,
            });
        }
        let mut payload = vec![0_u8; length];
        reader.read_exact(&mut payload).map_err(|error| {
            if error.kind() == io::ErrorKind::UnexpectedEof {
                CodecError::TruncatedFrame { expected: length }
            } else {
                CodecError::Io(error)
            }
        })?;
        serde_json::from_slice(&payload)
            .map(Some)
            .map_err(CodecError::Decode)
    }

    enum HeaderRead {
        Eof,
        Complete,
    }

    fn read_header<R: Read>(
        reader: &mut R,
        header: &mut [u8; 4],
    ) -> Result<HeaderRead, CodecError> {
        let mut filled = 0;
        while filled < header.len() {
            match reader.read(&mut header[filled..]) {
                Ok(0) if filled == 0 => return Ok(HeaderRead::Eof),
                Ok(0) => return Err(CodecError::TruncatedHeader { actual: filled }),
                Ok(count) => filled += count,
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Err(error) => return Err(CodecError::Io(error)),
            }
        }
        Ok(HeaderRead::Complete)
    }

    #[derive(Debug, thiserror::Error)]
    pub enum CodecError {
        #[error("native Automation transport I/O failed: {0}")]
        Io(#[from] io::Error),
        #[error("could not encode Automation protocol envelope: {0}")]
        Encode(serde_json::Error),
        #[error("could not decode Automation protocol envelope: {0}")]
        Decode(serde_json::Error),
        #[error("Automation protocol frame is empty")]
        EmptyFrame,
        #[error("Automation protocol header ended after {actual} of 4 bytes")]
        TruncatedHeader { actual: usize },
        #[error("Automation protocol frame ended before its declared {expected} bytes")]
        TruncatedFrame { expected: usize },
        #[error("Automation protocol frame has {actual} bytes; maximum is {maximum}")]
        FrameTooLarge { actual: usize, maximum: usize },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

impl ProtocolVersion {
    pub fn require_compatible(self) -> Result<(), ProtocolError> {
        if self.major != PROTOCOL_VERSION.major || self.minor > PROTOCOL_VERSION.minor {
            return Err(ProtocolError::IncompatibleVersion {
                received_major: self.major,
                received_minor: self.minor,
                supported_major: PROTOCOL_VERSION.major,
                supported_minor: PROTOCOL_VERSION.minor,
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Digest(pub [u8; 32]);

impl Digest {
    pub fn validate(self, field: &'static str) -> Result<(), ProtocolError> {
        if self.0.iter().all(|byte| *byte == 0) {
            return Err(ProtocolError::ZeroDigest { field });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimePlatform {
    NativeWindows,
    NativeMacOs,
    NativeLinux,
    BrowserWasm,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeIdentity {
    pub managed: bool,
    pub platform: RuntimePlatform,
    pub architecture: String,
    pub runtime_build: String,
    pub runtime_digest: Digest,
    pub python_version: String,
    pub python_abi: String,
    pub rspice_api_version: String,
    pub protocol: ProtocolVersion,
}

impl RuntimeIdentity {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        self.protocol.require_compatible()?;
        self.runtime_digest.validate("runtime_digest")?;
        for (field, value) in [
            ("architecture", self.architecture.as_str()),
            ("runtime_build", self.runtime_build.as_str()),
            ("python_version", self.python_version.as_str()),
            ("python_abi", self.python_abi.as_str()),
            ("rspice_api_version", self.rspice_api_version.as_str()),
        ] {
            validate_text(field, value, 256, false)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentRole {
    PythonEntry,
    PythonModule,
    RunPlan,
    EnvironmentLock,
    PermissionManifest,
    Resource,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceDocument {
    pub document_id: Uuid,
    pub logical_path: String,
    pub revision: u64,
    pub role: DocumentRole,
    pub read_only: bool,
    pub source: String,
}

impl SourceDocument {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        validate_uuid("document_id", self.document_id)?;
        validate_revision("document revision", self.revision)?;
        validate_logical_path(&self.logical_path)?;
        if self.source.len() > MAX_TEXT_FIELD_BYTES * 16 {
            return Err(ProtocolError::LimitExceeded {
                field: "document source",
                actual: self.source.len(),
                limit: MAX_TEXT_FIELD_BYTES * 16,
            });
        }
        if self.source.as_bytes().contains(&0) {
            return Err(ProtocolError::NulByte {
                field: "document source",
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityGrant {
    pub capability: CapabilityKind,
    /// Opaque capability-broker scope, never a native host path supplied by Python.
    pub scope: String,
    pub token: Uuid,
}

impl CapabilityGrant {
    fn validate(&self) -> Result<(), ProtocolError> {
        validate_uuid("capability token", self.token)?;
        validate_text("capability scope", &self.scope, 4_096, false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityKind {
    ProjectRead,
    ProjectWrite,
    SimulationExecute,
    ResultRead,
    ArtifactWrite,
    ExternalFileRead,
    ExternalFileWrite,
    Network,
    ProcessSpawn,
    EnvironmentRead,
    ClipboardRead,
    ClipboardWrite,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSnapshot {
    pub project_id: Uuid,
    pub workspace_id: Uuid,
    pub workspace_revision: u64,
    pub closure_digest: Digest,
    pub environment_digest: Digest,
    pub permission_digest: Digest,
    pub entry_document_id: Uuid,
    pub selected_run_plan_document_id: Option<Uuid>,
    pub python_requirement: String,
    pub api_requirement: String,
    pub browser_runtime_requirement: Option<String>,
    pub documents: Vec<SourceDocument>,
    pub capabilities: Vec<CapabilityGrant>,
}

impl SourceSnapshot {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        validate_uuid("project_id", self.project_id)?;
        validate_uuid("workspace_id", self.workspace_id)?;
        validate_uuid("entry_document_id", self.entry_document_id)?;
        if let Some(id) = self.selected_run_plan_document_id {
            validate_uuid("selected_run_plan_document_id", id)?;
        }
        validate_revision("workspace revision", self.workspace_revision)?;
        self.closure_digest.validate("closure_digest")?;
        self.environment_digest.validate("environment_digest")?;
        self.permission_digest.validate("permission_digest")?;
        validate_text("python requirement", &self.python_requirement, 256, false)?;
        validate_text("API requirement", &self.api_requirement, 256, false)?;
        if let Some(requirement) = &self.browser_runtime_requirement {
            validate_text("browser runtime requirement", requirement, 256, false)?;
        }
        if self.documents.is_empty() || self.documents.len() > MAX_DOCUMENTS {
            return Err(ProtocolError::LimitExceeded {
                field: "documents",
                actual: self.documents.len(),
                limit: MAX_DOCUMENTS,
            });
        }
        if self.capabilities.len() > MAX_CAPABILITIES {
            return Err(ProtocolError::LimitExceeded {
                field: "capabilities",
                actual: self.capabilities.len(),
                limit: MAX_CAPABILITIES,
            });
        }

        let mut ids = BTreeSet::new();
        let mut paths = BTreeSet::new();
        let mut total_bytes = 0usize;
        let mut entry_count = 0usize;
        let mut lock_count = 0usize;
        let mut permission_count = 0usize;
        let mut selected_run_plan_found = self.selected_run_plan_document_id.is_none();
        for document in &self.documents {
            document.validate()?;
            if !ids.insert(document.document_id) {
                return Err(ProtocolError::DuplicateIdentity {
                    field: "document_id",
                    value: document.document_id,
                });
            }
            let key = portable_path_key(&document.logical_path);
            if !paths.insert(key) {
                return Err(ProtocolError::DuplicatePath {
                    path: document.logical_path.clone(),
                });
            }
            total_bytes = total_bytes.checked_add(document.source.len()).ok_or(
                ProtocolError::LimitExceeded {
                    field: "source bytes",
                    actual: usize::MAX,
                    limit: MAX_SOURCE_BYTES,
                },
            )?;
            entry_count += usize::from(document.role == DocumentRole::PythonEntry);
            lock_count += usize::from(document.role == DocumentRole::EnvironmentLock);
            permission_count += usize::from(document.role == DocumentRole::PermissionManifest);
            if document.document_id == self.entry_document_id
                && document.role != DocumentRole::PythonEntry
            {
                return Err(ProtocolError::RoleMismatch {
                    field: "entry_document_id",
                    expected: DocumentRole::PythonEntry,
                });
            }
            if Some(document.document_id) == self.selected_run_plan_document_id {
                if document.role != DocumentRole::RunPlan {
                    return Err(ProtocolError::RoleMismatch {
                        field: "selected_run_plan_document_id",
                        expected: DocumentRole::RunPlan,
                    });
                }
                selected_run_plan_found = true;
            }
        }
        if total_bytes > MAX_SOURCE_BYTES {
            return Err(ProtocolError::LimitExceeded {
                field: "source bytes",
                actual: total_bytes,
                limit: MAX_SOURCE_BYTES,
            });
        }
        if !ids.contains(&self.entry_document_id) {
            return Err(ProtocolError::MissingIdentity {
                field: "entry_document_id",
                value: self.entry_document_id,
            });
        }
        if let Some(value) = self
            .selected_run_plan_document_id
            .filter(|_| !selected_run_plan_found)
        {
            return Err(ProtocolError::MissingIdentity {
                field: "selected_run_plan_document_id",
                value,
            });
        }
        if entry_count != 1 || lock_count != 1 || permission_count != 1 {
            return Err(ProtocolError::InvalidRoleCardinality {
                entries: entry_count,
                environment_locks: lock_count,
                permission_manifests: permission_count,
            });
        }
        let mut tokens = BTreeSet::new();
        for capability in &self.capabilities {
            capability.validate()?;
            if !tokens.insert(capability.token) {
                return Err(ProtocolError::DuplicateIdentity {
                    field: "capability token",
                    value: capability.token,
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LaunchMode {
    Validate,
    DryRun,
    Run,
    Debug,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceLimits {
    pub wall_time_ms: u64,
    pub cpu_time_ms: u64,
    pub memory_bytes: u64,
    pub output_bytes: u64,
    pub artifact_bytes: u64,
    pub max_tasks: u32,
    pub max_stack_depth: u32,
}

impl ResourceLimits {
    pub fn validate(self) -> Result<(), ProtocolError> {
        for (field, value) in [
            ("wall_time_ms", self.wall_time_ms),
            ("cpu_time_ms", self.cpu_time_ms),
            ("memory_bytes", self.memory_bytes),
            ("output_bytes", self.output_bytes),
            ("artifact_bytes", self.artifact_bytes),
            ("max_tasks", u64::from(self.max_tasks)),
            ("max_stack_depth", u64::from(self.max_stack_depth)),
        ] {
            if value == 0 {
                return Err(ProtocolError::ZeroLimit { field });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum BreakpointKind {
    Stop,
    Conditional {
        expression: String,
    },
    Logpoint {
        template: String,
    },
    HitCount {
        count: u64,
        condition: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Breakpoint {
    pub breakpoint_id: Uuid,
    pub document_id: Uuid,
    pub line: u64,
    pub column: u64,
    pub enabled: bool,
    pub kind: BreakpointKind,
}

impl Breakpoint {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        validate_uuid("breakpoint_id", self.breakpoint_id)?;
        validate_uuid("breakpoint document_id", self.document_id)?;
        if self.line == 0 || self.column == 0 {
            return Err(ProtocolError::InvalidPosition {
                line: self.line,
                column: self.column,
            });
        }
        match &self.kind {
            BreakpointKind::Stop => {}
            BreakpointKind::Conditional { expression } => {
                validate_text("breakpoint expression", expression, 16_384, false)?;
            }
            BreakpointKind::Logpoint { template } => {
                validate_text("logpoint template", template, 65_536, false)?;
            }
            BreakpointKind::HitCount { count, condition } => {
                if *count == 0 {
                    return Err(ProtocolError::ZeroLimit {
                        field: "breakpoint hit count",
                    });
                }
                if let Some(condition) = condition {
                    validate_text("breakpoint condition", condition, 16_384, false)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExceptionPolicy {
    All,
    Uncaught,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DebugControl {
    Continue,
    Pause,
    StepIn,
    StepOver,
    StepOut,
    Restart,
    Stop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "kebab-case", deny_unknown_fields)]
pub enum RuntimeRequest {
    Probe,
    Launch {
        mode: LaunchMode,
        snapshot: Box<SourceSnapshot>,
        limits: ResourceLimits,
        breakpoints: Vec<Breakpoint>,
        exception_policy: ExceptionPolicy,
    },
    SetBreakpoints {
        session_id: Uuid,
        breakpoints: Vec<Breakpoint>,
    },
    DebugControl {
        session_id: Uuid,
        control: DebugControl,
    },
    StackTrace {
        session_id: Uuid,
        start: u64,
        count: u64,
    },
    Variables {
        session_id: Uuid,
        reference: u64,
        start: u64,
        count: u64,
    },
    Evaluate {
        session_id: Uuid,
        frame_id: u64,
        expression: String,
    },
    HostResponse {
        session_id: Uuid,
        call_id: u64,
        response: HostResponse,
    },
    Cancel {
        session_id: Uuid,
    },
    Shutdown,
}

impl RuntimeRequest {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        match self {
            Self::Probe | Self::Shutdown => Ok(()),
            Self::Launch {
                snapshot,
                limits,
                breakpoints,
                ..
            } => {
                snapshot.validate()?;
                limits.validate()?;
                validate_breakpoints(breakpoints, &snapshot.documents)
            }
            Self::SetBreakpoints {
                session_id,
                breakpoints,
            } => {
                validate_uuid("session_id", *session_id)?;
                if breakpoints.len() > MAX_BREAKPOINTS {
                    return Err(ProtocolError::LimitExceeded {
                        field: "breakpoints",
                        actual: breakpoints.len(),
                        limit: MAX_BREAKPOINTS,
                    });
                }
                breakpoints.iter().try_for_each(Breakpoint::validate)
            }
            Self::DebugControl { session_id, .. }
            | Self::StackTrace { session_id, .. }
            | Self::Variables { session_id, .. }
            | Self::Evaluate { session_id, .. }
            | Self::HostResponse { session_id, .. }
            | Self::Cancel { session_id } => {
                validate_uuid("session_id", *session_id)?;
                if let Self::Evaluate { expression, .. } = self {
                    validate_text(
                        "watch expression",
                        expression,
                        MAX_WATCH_EXPRESSION_BYTES,
                        false,
                    )?;
                }
                if let Self::HostResponse {
                    call_id, response, ..
                } = self
                {
                    if *call_id == 0 {
                        return Err(ProtocolError::ZeroReference { field: "call_id" });
                    }
                    response.validate()?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestEnvelope {
    pub protocol: ProtocolVersion,
    pub request_id: u64,
    pub request: RuntimeRequest,
}

impl RequestEnvelope {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        self.protocol.require_compatible()?;
        if self.request_id == 0 {
            return Err(ProtocolError::ZeroRequestId);
        }
        self.request.validate()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeState {
    Starting,
    Ready,
    Validating,
    Running,
    Paused,
    Cancelling,
    Cancelled,
    Completed,
    Failed,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourcePosition {
    pub line: u64,
    pub column: u64,
    pub byte_offset: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRange {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeDiagnostic {
    pub diagnostic_id: Uuid,
    pub document_id: Option<Uuid>,
    pub document_revision: Option<u64>,
    pub severity: DiagnosticSeverity,
    pub source: String,
    pub code: String,
    pub message: String,
    pub range: Option<SourceRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StackFrame {
    pub frame_id: u64,
    pub name: String,
    pub document_id: Uuid,
    pub range: SourceRange,
    pub locals_reference: u64,
    pub globals_reference: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Variable {
    pub name: String,
    pub type_name: String,
    pub display_value: String,
    pub variables_reference: u64,
    pub redacted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StopReason {
    Breakpoint,
    Step,
    Exception,
    Pause,
    Entry,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "kebab-case", deny_unknown_fields)]
pub enum RuntimeEvent {
    Hello {
        identity: RuntimeIdentity,
    },
    State {
        state: RuntimeState,
        detail: String,
    },
    Diagnostic {
        diagnostic: RuntimeDiagnostic,
    },
    Output {
        channel: String,
        category: String,
        text: String,
    },
    Stopped {
        reason: StopReason,
        description: String,
        frame_id: Option<u64>,
    },
    Stack {
        frames: Vec<StackFrame>,
        total: u64,
    },
    Variables {
        values: Vec<Variable>,
        total: u64,
    },
    Evaluated {
        expression: String,
        result: Variable,
    },
    Progress {
        operation: String,
        completed: u64,
        total: Option<u64>,
    },
    PermissionDenied {
        capability: CapabilityKind,
        scope: String,
        operation: String,
    },
    HostCall {
        call: HostCall,
    },
    ResultPublished {
        handle: Uuid,
        kind: String,
    },
    ArtifactPublished {
        handle: Uuid,
        kind: String,
        bytes: u64,
    },
    Terminated {
        exit_code: Option<i32>,
        reason: String,
    },
    WorkerFailed {
        code: String,
        message: String,
        recoverable: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventEnvelope {
    pub protocol: ProtocolVersion,
    pub request_id: Option<u64>,
    pub session_id: Option<Uuid>,
    pub sequence: u64,
    pub event: RuntimeEvent,
}

impl EventEnvelope {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        self.protocol.require_compatible()?;
        if self.sequence == 0 {
            return Err(ProtocolError::ZeroSequence);
        }
        if self.request_id == Some(0) {
            return Err(ProtocolError::ZeroRequestId);
        }
        if let Some(session_id) = self.session_id {
            validate_uuid("session_id", session_id)?;
        }
        match &self.event {
            RuntimeEvent::Hello { identity } => identity.validate(),
            RuntimeEvent::State { detail, .. } => {
                validate_text("state detail", detail, 65_536, true)
            }
            RuntimeEvent::Diagnostic { diagnostic } => validate_diagnostic(diagnostic),
            RuntimeEvent::Output {
                channel,
                category,
                text,
            } => {
                validate_text("output channel", channel, 128, false)?;
                validate_text("output category", category, 128, false)?;
                validate_text("output text", text, MAX_TEXT_FIELD_BYTES, true)
            }
            RuntimeEvent::Stopped {
                description,
                frame_id,
                ..
            } => {
                validate_text("stop description", description, 65_536, true)?;
                if frame_id == &Some(0) {
                    return Err(ProtocolError::ZeroReference { field: "frame_id" });
                }
                Ok(())
            }
            RuntimeEvent::Stack { frames, .. } => frames.iter().try_for_each(validate_frame),
            RuntimeEvent::Variables { values, .. } => values.iter().try_for_each(validate_variable),
            RuntimeEvent::Evaluated { expression, result } => {
                validate_text("evaluated expression", expression, 16_384, false)?;
                validate_variable(result)
            }
            RuntimeEvent::Progress { operation, .. } => {
                validate_text("progress operation", operation, 1_024, false)
            }
            RuntimeEvent::PermissionDenied {
                scope, operation, ..
            } => {
                validate_text("permission scope", scope, 4_096, false)?;
                validate_text("permission operation", operation, 1_024, false)
            }
            RuntimeEvent::HostCall { call } => call.validate(),
            RuntimeEvent::ResultPublished { handle, kind }
            | RuntimeEvent::ArtifactPublished { handle, kind, .. } => {
                validate_uuid("published handle", *handle)?;
                validate_text("published kind", kind, 256, false)
            }
            RuntimeEvent::Terminated { reason, .. } => {
                validate_text("termination reason", reason, 65_536, true)
            }
            RuntimeEvent::WorkerFailed { code, message, .. } => {
                validate_text("worker failure code", code, 256, false)?;
                validate_text("worker failure message", message, 65_536, false)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostCall {
    pub call_id: u64,
    pub capability: CapabilityKind,
    pub capability_token: Uuid,
    pub operation: HostOperation,
}

impl HostCall {
    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.call_id == 0 {
            return Err(ProtocolError::ZeroReference { field: "call_id" });
        }
        validate_uuid("host-call capability token", self.capability_token)?;
        let expected = self.operation.required_capability();
        if self.capability != expected {
            return Err(ProtocolError::CapabilityMismatch {
                expected,
                actual: self.capability,
            });
        }
        self.operation.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HostOperation {
    OpenProject {
        selector: String,
    },
    LoadRunPlan {
        project_handle: Uuid,
        document_id: Uuid,
    },
    ValidateRunPlan {
        plan_handle: Uuid,
        target: String,
        fail_closed: bool,
    },
    ExecuteRunPlan {
        preview_handle: Uuid,
    },
    EvaluateRequirements {
        run_handle: Uuid,
        profile: String,
    },
    CompareRun {
        run_handle: Uuid,
        baseline: String,
        waveforms: bool,
    },
    ExportRun {
        run_handle: Uuid,
        formats: Vec<String>,
    },
    ReadEnvironment {
        name: String,
    },
}

impl HostOperation {
    pub const fn required_capability(&self) -> CapabilityKind {
        match self {
            Self::OpenProject { .. } | Self::LoadRunPlan { .. } | Self::ValidateRunPlan { .. } => {
                CapabilityKind::ProjectRead
            }
            Self::ExecuteRunPlan { .. } => CapabilityKind::SimulationExecute,
            Self::EvaluateRequirements { .. } | Self::CompareRun { .. } => {
                CapabilityKind::ResultRead
            }
            Self::ExportRun { .. } => CapabilityKind::ArtifactWrite,
            Self::ReadEnvironment { .. } => CapabilityKind::EnvironmentRead,
        }
    }

    fn validate(&self) -> Result<(), ProtocolError> {
        match self {
            Self::OpenProject { selector } => {
                validate_text("project selector", selector, 4_096, false)
            }
            Self::LoadRunPlan {
                project_handle,
                document_id,
            } => {
                validate_uuid("project handle", *project_handle)?;
                validate_uuid("run-plan document ID", *document_id)
            }
            Self::ValidateRunPlan {
                plan_handle,
                target,
                ..
            } => {
                validate_uuid("run-plan handle", *plan_handle)?;
                validate_text("run target", target, 1_024, false)
            }
            Self::ExecuteRunPlan { preview_handle } => {
                validate_uuid("run preview handle", *preview_handle)
            }
            Self::EvaluateRequirements {
                run_handle,
                profile,
            } => {
                validate_uuid("run handle", *run_handle)?;
                validate_text("requirements profile", profile, 1_024, false)
            }
            Self::CompareRun {
                run_handle,
                baseline,
                ..
            } => {
                validate_uuid("run handle", *run_handle)?;
                validate_text("comparison baseline", baseline, 4_096, false)
            }
            Self::ExportRun {
                run_handle,
                formats,
            } => {
                validate_uuid("run handle", *run_handle)?;
                if formats.is_empty() || formats.len() > 256 {
                    return Err(ProtocolError::LimitExceeded {
                        field: "artifact formats",
                        actual: formats.len(),
                        limit: 256,
                    });
                }
                let mut unique = BTreeSet::new();
                for format in formats {
                    validate_text("artifact format", format, 256, false)?;
                    if !unique.insert(format) {
                        return Err(ProtocolError::DuplicateText {
                            field: "artifact format",
                            value: format.clone(),
                        });
                    }
                }
                Ok(())
            }
            Self::ReadEnvironment { name } => {
                validate_text("environment variable name", name, 256, false)?;
                if !portable_environment_name(name) {
                    return Err(ProtocolError::InvalidEnvironmentName {
                        value: name.clone(),
                    });
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HostResponse {
    Success {
        handle: Option<Uuid>,
        detail: String,
        /// Optional bounded textual result returned by read-only broker calls.
        /// Existing handle-only operations omit this field on the wire.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<String>,
    },
    Failure {
        code: String,
        message: String,
        permission_denied: bool,
    },
}

impl HostResponse {
    fn validate(&self) -> Result<(), ProtocolError> {
        match self {
            Self::Success {
                handle,
                detail,
                value,
            } => {
                if let Some(handle) = handle {
                    validate_uuid("host response handle", *handle)?;
                }
                validate_text("host response detail", detail, 65_536, true)?;
                if let Some(value) = value {
                    validate_text("host response value", value, 65_536, true)?;
                }
                Ok(())
            }
            Self::Failure { code, message, .. } => {
                validate_text("host response failure code", code, 256, false)?;
                validate_text("host response failure message", message, 65_536, false)
            }
        }
    }
}

fn validate_breakpoints(
    breakpoints: &[Breakpoint],
    documents: &[SourceDocument],
) -> Result<(), ProtocolError> {
    if breakpoints.len() > MAX_BREAKPOINTS {
        return Err(ProtocolError::LimitExceeded {
            field: "breakpoints",
            actual: breakpoints.len(),
            limit: MAX_BREAKPOINTS,
        });
    }
    let documents_by_id = documents
        .iter()
        .map(|document| (document.document_id, document))
        .collect::<BTreeMap<_, _>>();
    let mut breakpoint_ids = BTreeSet::new();
    for breakpoint in breakpoints {
        breakpoint.validate()?;
        if !breakpoint_ids.insert(breakpoint.breakpoint_id) {
            return Err(ProtocolError::DuplicateIdentity {
                field: "breakpoint_id",
                value: breakpoint.breakpoint_id,
            });
        }
        let Some(document) = documents_by_id.get(&breakpoint.document_id) else {
            return Err(ProtocolError::MissingIdentity {
                field: "breakpoint document_id",
                value: breakpoint.document_id,
            });
        };
        if !matches!(
            document.role,
            DocumentRole::PythonEntry | DocumentRole::PythonModule
        ) {
            return Err(ProtocolError::BreakpointDocumentIsNotPython {
                value: breakpoint.document_id,
            });
        }
        let line = breakpoint.line as usize;
        let Some(source_line) = document.source.lines().nth(line.saturating_sub(1)) else {
            return Err(ProtocolError::BreakpointOutsideSource {
                document_id: breakpoint.document_id,
                line: breakpoint.line,
                column: breakpoint.column,
            });
        };
        if breakpoint.column as usize > source_line.chars().count().saturating_add(1) {
            return Err(ProtocolError::BreakpointOutsideSource {
                document_id: breakpoint.document_id,
                line: breakpoint.line,
                column: breakpoint.column,
            });
        }
    }
    Ok(())
}

fn validate_diagnostic(diagnostic: &RuntimeDiagnostic) -> Result<(), ProtocolError> {
    validate_uuid("diagnostic_id", diagnostic.diagnostic_id)?;
    if let Some(id) = diagnostic.document_id {
        validate_uuid("diagnostic document_id", id)?;
    }
    if let Some(revision) = diagnostic.document_revision {
        validate_revision("diagnostic document revision", revision)?;
    }
    validate_text("diagnostic source", &diagnostic.source, 256, false)?;
    validate_text("diagnostic code", &diagnostic.code, 256, false)?;
    validate_text("diagnostic message", &diagnostic.message, 65_536, false)?;
    if let Some(range) = diagnostic.range {
        validate_range(range)?;
    }
    Ok(())
}

fn validate_frame(frame: &StackFrame) -> Result<(), ProtocolError> {
    if frame.frame_id == 0 {
        return Err(ProtocolError::ZeroReference { field: "frame_id" });
    }
    validate_uuid("frame document_id", frame.document_id)?;
    validate_text("frame name", &frame.name, 4_096, false)?;
    if frame.locals_reference == 0 || frame.globals_reference == 0 {
        return Err(ProtocolError::ZeroReference {
            field: "frame variable reference",
        });
    }
    validate_range(frame.range)
}

fn validate_variable(variable: &Variable) -> Result<(), ProtocolError> {
    validate_text("variable name", &variable.name, 4_096, false)?;
    validate_text("variable type", &variable.type_name, 4_096, true)?;
    validate_text(
        "variable display value",
        &variable.display_value,
        65_536,
        true,
    )
}

fn validate_range(range: SourceRange) -> Result<(), ProtocolError> {
    for position in [range.start, range.end] {
        if position.line == 0 || position.column == 0 {
            return Err(ProtocolError::InvalidPosition {
                line: position.line,
                column: position.column,
            });
        }
    }
    if (range.end.byte_offset, range.end.line, range.end.column)
        < (
            range.start.byte_offset,
            range.start.line,
            range.start.column,
        )
    {
        return Err(ProtocolError::ReversedRange);
    }
    Ok(())
}

fn validate_uuid(field: &'static str, value: Uuid) -> Result<(), ProtocolError> {
    if value.is_nil() {
        return Err(ProtocolError::NilIdentity { field });
    }
    Ok(())
}

fn validate_revision(field: &'static str, revision: u64) -> Result<(), ProtocolError> {
    if revision == 0 {
        return Err(ProtocolError::ZeroRevision { field });
    }
    Ok(())
}

fn validate_text(
    field: &'static str,
    value: &str,
    limit: usize,
    allow_empty: bool,
) -> Result<(), ProtocolError> {
    if (!allow_empty && value.trim().is_empty()) || value.as_bytes().contains(&0) {
        return Err(ProtocolError::InvalidText { field });
    }
    if value.len() > limit {
        return Err(ProtocolError::LimitExceeded {
            field,
            actual: value.len(),
            limit,
        });
    }
    Ok(())
}

fn portable_environment_name(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|character| character == '_' || character.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

/// Locale-independent Unicode caseless identity for portable project paths.
///
/// This deliberately matches the UI source graph and both managed Python
/// adapters: NFC normalization surrounds upper-then-lower expansion so case
/// aliases such as `Straße`/`STRASSE` and canonically equivalent accents
/// cannot enter a worker snapshot as distinct documents.
fn portable_path_key(path: &str) -> String {
    path.nfc()
        .flat_map(char::to_uppercase)
        .flat_map(char::to_lowercase)
        .collect::<String>()
        .nfc()
        .collect()
}

fn validate_logical_path(path: &str) -> Result<(), ProtocolError> {
    if path.is_empty()
        || path.len() > MAX_PATH_BYTES
        || path != path.trim()
        || path.starts_with('/')
        || path.starts_with('\\')
        || path.contains('\\')
        || path.as_bytes().contains(&0)
        || path.split('/').any(|component| {
            component.is_empty() || matches!(component, "." | "..") || component != component.trim()
        })
    {
        return Err(ProtocolError::InvalidLogicalPath {
            path: path.to_owned(),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProtocolError {
    #[error(
        "Automation protocol {received_major}.{received_minor} is incompatible with supported {supported_major}.{supported_minor}"
    )]
    IncompatibleVersion {
        received_major: u16,
        received_minor: u16,
        supported_major: u16,
        supported_minor: u16,
    },
    #[error("{field} must not be the nil identity")]
    NilIdentity { field: &'static str },
    #[error("{field} must not be an all-zero digest")]
    ZeroDigest { field: &'static str },
    #[error("{field} revision must be non-zero")]
    ZeroRevision { field: &'static str },
    #[error("{field} limit must be non-zero")]
    ZeroLimit { field: &'static str },
    #[error("request_id must be non-zero")]
    ZeroRequestId,
    #[error("event sequence must be non-zero")]
    ZeroSequence,
    #[error("{field} reference must be non-zero")]
    ZeroReference { field: &'static str },
    #[error("{field} contains empty, NUL, or otherwise invalid text")]
    InvalidText { field: &'static str },
    #[error("{field} contains a NUL byte")]
    NulByte { field: &'static str },
    #[error("{field} has {actual} entries/bytes; limit is {limit}")]
    LimitExceeded {
        field: &'static str,
        actual: usize,
        limit: usize,
    },
    #[error("logical path '{path}' is not portable and normalized")]
    InvalidLogicalPath { path: String },
    #[error("environment variable name '{value}' is not portable")]
    InvalidEnvironmentName { value: String },
    #[error("{field} repeats identity {value}")]
    DuplicateIdentity { field: &'static str, value: Uuid },
    #[error("{field} repeats value '{value}'")]
    DuplicateText { field: &'static str, value: String },
    #[error("host operation requires capability {expected:?}, not {actual:?}")]
    CapabilityMismatch {
        expected: CapabilityKind,
        actual: CapabilityKind,
    },
    #[error("source closure repeats logical path '{path}' case-insensitively")]
    DuplicatePath { path: String },
    #[error("{field} references missing identity {value}")]
    MissingIdentity { field: &'static str, value: Uuid },
    #[error("breakpoint document {value} is not a Python source document")]
    BreakpointDocumentIsNotPython { value: Uuid },
    #[error("breakpoint {line}:{column} is outside Python document {document_id}")]
    BreakpointOutsideSource {
        document_id: Uuid,
        line: u64,
        column: u64,
    },
    #[error("{field} does not reference a document with role {expected:?}")]
    RoleMismatch {
        field: &'static str,
        expected: DocumentRole,
    },
    #[error(
        "source closure requires one entry, environment lock, and permission manifest; found {entries}, {environment_locks}, and {permission_manifests}"
    )]
    InvalidRoleCardinality {
        entries: usize,
        environment_locks: usize,
        permission_manifests: usize,
    },
    #[error("invalid source position {line}:{column}")]
    InvalidPosition { line: u64, column: u64 },
    #[error("source range end precedes its start")]
    ReversedRange,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u128) -> Uuid {
        Uuid::from_u128(value)
    }

    fn document(value: u128, path: &str, role: DocumentRole, source: &str) -> SourceDocument {
        SourceDocument {
            document_id: id(value),
            logical_path: path.to_owned(),
            revision: 1,
            role,
            read_only: matches!(role, DocumentRole::EnvironmentLock),
            source: source.to_owned(),
        }
    }

    fn snapshot() -> SourceSnapshot {
        SourceSnapshot {
            project_id: id(1),
            workspace_id: id(2),
            workspace_revision: 7,
            closure_digest: Digest([1; 32]),
            environment_digest: Digest([2; 32]),
            permission_digest: Digest([3; 32]),
            entry_document_id: id(3),
            selected_run_plan_document_id: Some(id(4)),
            python_requirement: ">=3.14,<3.15".to_owned(),
            api_requirement: ">=1.0,<2.0".to_owned(),
            browser_runtime_requirement: Some("=0.26.4".to_owned()),
            documents: vec![
                document(
                    3,
                    "flows/nightly.py",
                    DocumentRole::PythonEntry,
                    "print('ok')\n",
                ),
                document(
                    4,
                    "plans/release.data",
                    DocumentRole::RunPlan,
                    "schema: v1\n",
                ),
                document(
                    5,
                    "environment/pinned.toml",
                    DocumentRole::EnvironmentLock,
                    "format = 'lock/v1'\n",
                ),
                document(
                    6,
                    "security/policy.toml",
                    DocumentRole::PermissionManifest,
                    "network = 'deny'\n",
                ),
                document(
                    7,
                    "flows/helpers.py",
                    DocumentRole::PythonModule,
                    "def helper(): return 1\n",
                ),
            ],
            capabilities: vec![CapabilityGrant {
                capability: CapabilityKind::ProjectRead,
                scope: "project-snapshot".to_owned(),
                token: id(20),
            }],
        }
    }

    #[test]
    fn arbitrary_paths_and_extra_modules_validate_and_round_trip() {
        let snapshot = snapshot();
        snapshot.validate().unwrap();
        let request = RequestEnvelope {
            protocol: PROTOCOL_VERSION,
            request_id: 1,
            request: RuntimeRequest::Launch {
                mode: LaunchMode::Debug,
                snapshot: Box::new(snapshot),
                limits: ResourceLimits {
                    wall_time_ms: 60_000,
                    cpu_time_ms: 30_000,
                    memory_bytes: 512 * 1024 * 1024,
                    output_bytes: 16 * 1024 * 1024,
                    artifact_bytes: 128 * 1024 * 1024,
                    max_tasks: 100,
                    max_stack_depth: 1_000,
                },
                breakpoints: vec![Breakpoint {
                    breakpoint_id: id(30),
                    document_id: id(3),
                    line: 1,
                    column: 1,
                    enabled: true,
                    kind: BreakpointKind::Conditional {
                        expression: "enabled".to_owned(),
                    },
                }],
                exception_policy: ExceptionPolicy::Uncaught,
            },
        };
        request.validate().unwrap();
        let encoded = serde_json::to_vec(&request).unwrap();
        let restored: RequestEnvelope = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(restored, request);
        restored.validate().unwrap();
    }

    #[test]
    fn source_identity_and_role_corruption_fail_closed() {
        let mut candidate = snapshot();
        candidate.documents[1].logical_path = "FLOWS/NIGHTLY.PY".to_owned();
        assert!(matches!(
            candidate.validate(),
            Err(ProtocolError::DuplicatePath { .. })
        ));

        for alias in ["FLOWS/STRASSE.PY", "FLOWS/CAFE\u{301}.PY"] {
            let mut candidate = snapshot();
            candidate.documents[0].logical_path = match alias {
                "FLOWS/STRASSE.PY" => "flows/Straße.py".to_owned(),
                _ => "flows/café.py".to_owned(),
            };
            candidate.documents[1].logical_path = alias.to_owned();
            assert!(matches!(
                candidate.validate(),
                Err(ProtocolError::DuplicatePath { .. })
            ));
        }

        let mut candidate = snapshot();
        candidate.documents[0].role = DocumentRole::PythonModule;
        assert!(matches!(
            candidate.validate(),
            Err(ProtocolError::RoleMismatch { .. })
                | Err(ProtocolError::InvalidRoleCardinality { .. })
        ));

        let mut candidate = snapshot();
        candidate.closure_digest = Digest([0; 32]);
        assert!(matches!(
            candidate.validate(),
            Err(ProtocolError::ZeroDigest { .. })
        ));
    }

    #[test]
    fn breakpoint_and_envelope_bounds_fail_closed() {
        let breakpoint = Breakpoint {
            breakpoint_id: id(30),
            document_id: id(3),
            line: 0,
            column: 1,
            enabled: true,
            kind: BreakpointKind::HitCount {
                count: 0,
                condition: None,
            },
        };
        assert!(matches!(
            breakpoint.validate(),
            Err(ProtocolError::InvalidPosition { .. })
        ));
        let request = RequestEnvelope {
            protocol: PROTOCOL_VERSION,
            request_id: 0,
            request: RuntimeRequest::Probe,
        };
        assert_eq!(request.validate(), Err(ProtocolError::ZeroRequestId));
    }

    #[test]
    fn watch_expression_bounds_are_shared_and_fail_closed() {
        let request = |expression: String| RuntimeRequest::Evaluate {
            session_id: id(1),
            frame_id: 1,
            expression,
        };
        request("value".repeat(MAX_WATCH_EXPRESSION_BYTES / 5))
            .validate()
            .unwrap();
        assert!(matches!(
            request(" ".to_owned()).validate(),
            Err(ProtocolError::InvalidText {
                field: "watch expression"
            })
        ));
        assert!(matches!(
            request("value\0suffix".to_owned()).validate(),
            Err(ProtocolError::InvalidText {
                field: "watch expression"
            })
        ));
        assert!(matches!(
            request("x".repeat(MAX_WATCH_EXPRESSION_BYTES + 1)).validate(),
            Err(ProtocolError::LimitExceeded {
                field: "watch expression",
                limit: MAX_WATCH_EXPRESSION_BYTES,
                ..
            })
        ));
    }

    #[test]
    fn launch_breakpoints_are_python_source_bound_and_in_range() {
        let mut candidate = snapshot();
        let valid = Breakpoint {
            breakpoint_id: id(31),
            document_id: id(7),
            line: 1,
            column: 5,
            enabled: true,
            kind: BreakpointKind::Stop,
        };
        validate_breakpoints(std::slice::from_ref(&valid), &candidate.documents).unwrap();

        let mut outside = valid.clone();
        outside.line = 2;
        assert!(matches!(
            validate_breakpoints(&[outside], &candidate.documents),
            Err(ProtocolError::BreakpointOutsideSource { .. })
        ));

        let mut non_python = valid;
        non_python.document_id = id(4);
        assert!(matches!(
            validate_breakpoints(&[non_python], &candidate.documents),
            Err(ProtocolError::BreakpointDocumentIsNotPython { .. })
        ));

        candidate.documents[4].source = "μ = 1\n".to_owned();
        let unicode_end = Breakpoint {
            breakpoint_id: id(32),
            document_id: id(7),
            line: 1,
            column: 7,
            enabled: true,
            kind: BreakpointKind::Stop,
        };
        assert!(matches!(
            validate_breakpoints(&[unicode_end], &candidate.documents),
            Err(ProtocolError::BreakpointOutsideSource { .. })
        ));
    }

    #[test]
    fn native_frames_are_bounded_and_round_trip_without_newline_assumptions() {
        let request = RequestEnvelope {
            protocol: PROTOCOL_VERSION,
            request_id: 9,
            request: RuntimeRequest::Probe,
        };
        let mut bytes = Vec::new();
        native_codec::write_frame(&mut bytes, &request).unwrap();
        assert_ne!(bytes[0..4], [0, 0, 0, 0]);
        let mut cursor = std::io::Cursor::new(bytes);
        let restored: RequestEnvelope = native_codec::read_frame(&mut cursor).unwrap().unwrap();
        assert_eq!(restored, request);
        assert!(
            native_codec::read_frame::<_, RequestEnvelope>(&mut cursor)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn native_frames_reject_truncation_and_oversized_headers_before_allocation() {
        let truncated_header = [0_u8, 0, 0];
        assert!(matches!(
            native_codec::read_frame::<_, RequestEnvelope>(&mut truncated_header.as_slice()),
            Err(native_codec::CodecError::TruncatedHeader { actual: 3 })
        ));

        let declared = u32::try_from(MAX_ENVELOPE_BYTES + 1).unwrap().to_be_bytes();
        assert!(matches!(
            native_codec::read_frame::<_, RequestEnvelope>(&mut declared.as_slice()),
            Err(native_codec::CodecError::FrameTooLarge { .. })
        ));

        let mut truncated_payload = vec![0_u8, 0, 0, 8];
        truncated_payload.extend_from_slice(b"{}");
        assert!(matches!(
            native_codec::read_frame::<_, RequestEnvelope>(&mut truncated_payload.as_slice()),
            Err(native_codec::CodecError::TruncatedFrame { expected: 8 })
        ));
    }

    #[test]
    fn host_calls_are_typed_and_capability_bound() {
        let call = HostCall {
            call_id: 1,
            capability: CapabilityKind::SimulationExecute,
            capability_token: id(80),
            operation: HostOperation::ExecuteRunPlan {
                preview_handle: id(81),
            },
        };
        call.validate().unwrap();

        let mut wrong = call;
        wrong.capability = CapabilityKind::ProjectRead;
        assert!(matches!(
            wrong.validate(),
            Err(ProtocolError::CapabilityMismatch { .. })
        ));

        let environment = HostCall {
            call_id: 2,
            capability: CapabilityKind::EnvironmentRead,
            capability_token: id(82),
            operation: HostOperation::ReadEnvironment {
                name: "RSPICE_LICENSE_TOKEN".to_owned(),
            },
        };
        environment.validate().unwrap();
        let mut invalid_environment = environment;
        invalid_environment.operation = HostOperation::ReadEnvironment {
            name: "NOT-PORTABLE".to_owned(),
        };
        assert!(matches!(
            invalid_environment.validate(),
            Err(ProtocolError::InvalidEnvironmentName { .. })
        ));

        HostResponse::Success {
            handle: None,
            detail: "environment value returned".to_owned(),
            value: Some("secret".to_owned()),
        }
        .validate()
        .unwrap();
    }

    #[test]
    fn operations_for_unimplemented_ambient_capabilities_fail_at_the_protocol_boundary() {
        for capability in [
            CapabilityKind::ProjectWrite,
            CapabilityKind::ExternalFileRead,
            CapabilityKind::ExternalFileWrite,
            CapabilityKind::Network,
            CapabilityKind::ProcessSpawn,
            CapabilityKind::ClipboardRead,
            CapabilityKind::ClipboardWrite,
        ] {
            let call = HostCall {
                call_id: 1,
                capability,
                capability_token: id(90),
                operation: HostOperation::OpenProject {
                    selector: ".".to_owned(),
                },
            };
            assert!(matches!(
                call.validate(),
                Err(ProtocolError::CapabilityMismatch {
                    expected: CapabilityKind::ProjectRead,
                    actual,
                }) if actual == capability
            ));
        }
    }
}
