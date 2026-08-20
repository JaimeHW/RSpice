//! Runtime page and compile/execution state for the Code & Automation workspace.
//!
//! Source bytes and their persisted revisions belong to `ProjectWorkspace`.
//! This module owns only the currently visible page and asynchronous operation
//! receipts that must never survive a process restart.

use crate::simulation::veriloga::{PreparedVerilogARuntime, VerilogASourceOperationToken};
use crate::{
    product::{ContentDigest, ProjectId},
    state::{ProjectSourceId, ProjectSourceLanguage},
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};

use super::super::{CodeSourceFileAction, CodeWorkspacePage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeSourceFileDialogState {
    pub action: CodeSourceFileAction,
    pub project_id: ProjectId,
    pub bundle_id: ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: ContentDigest,
    pub language: ProjectSourceLanguage,
    pub source_path: String,
    pub importer_path: String,
    pub proposed_path: String,
    pub error: Option<String>,
}

/// Transaction used only when a project does not yet own the selected Code
/// Workspace source bundle. It captures the project identity at open time and
/// carries portable logical paths rather than giving starter filenames any
/// semantic authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeSourceWorkspaceDialogState {
    pub project_id: ProjectId,
    pub language: ProjectSourceLanguage,
    pub root_path: String,
    pub build_profile_path: String,
    pub run_plan_path: String,
    pub environment_lock_path: String,
    pub permission_manifest_path: String,
    pub netlist_source_path: String,
    pub error: Option<String>,
}

/// Revision-bound source history and comparison session. Restoring a retained
/// revision is a whole-bundle graph operation (files, roles, and dependency
/// edges), so the dialog captures the exact current closure it was opened on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeSourceHistoryState {
    pub project_id: ProjectId,
    pub bundle_id: ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: ContentDigest,
    pub language: ProjectSourceLanguage,
    pub source_path: String,
    pub selected_revision: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeSourceImportState {
    pub project_id: ProjectId,
    pub bundle_id: ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: ContentDigest,
    pub language: ProjectSourceLanguage,
    pub importer_path: String,
    pub picker_started: bool,
}

/// Revision-bound, bundle-wide source search/replace session. Search options
/// are transient UI state; the captured identity prevents results or a
/// replacement preview from being applied to a newer source closure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeSourceSearchState {
    pub project_id: ProjectId,
    pub bundle_id: ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: ContentDigest,
    pub generated_reference_revision: u64,
    pub generated_reference_digest: ContentDigest,
    pub language: ProjectSourceLanguage,
    pub active_path: String,
    /// Character range captured from the exact active editor revision when
    /// the search opened. It remains immutable for this revision-bound search
    /// and is discarded if the bundle changes.
    pub selection_char_range: Option<(usize, usize)>,
    pub scope: CodeSourceSearchScope,
    pub query: String,
    pub replacement: String,
    pub match_case: bool,
    pub whole_symbol: bool,
    pub regular_expression: bool,
    pub include_comments: bool,
    pub include_generated_references: bool,
    pub confirm_replace: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CodeSourceSearchScope {
    Selection,
    CurrentDocument,
    OpenDocuments,
    #[default]
    ActiveLanguageProject,
}

/// Exact source identity captured when a background operation starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceOperationToken {
    pub project_id: crate::product::ProjectId,
    pub revision: u64,
    pub content_digest: crate::product::ContentDigest,
}

#[derive(Debug)]
pub struct PendingVerilogACompile {
    pub token: VerilogASourceOperationToken,
    pub receiver: Arc<Mutex<mpsc::Receiver<VerilogACompileOutcome>>>,
}

/// Immutable, identity-bound review of one Verilog-A build transaction. All
/// editable policy remains in the project-owned build-profile document; this
/// snapshot exists so the user can verify exactly what will execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogACompileDialogState {
    pub project_id: crate::product::ProjectId,
    pub bundle_id: crate::state::ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: crate::product::ContentDigest,
    pub profile_digest: crate::product::ContentDigest,
    pub profile_path: String,
    pub package_name: String,
    pub package_version: String,
    pub selected_module: String,
    pub compile_order: Vec<String>,
    pub include_paths: Vec<String>,
    pub definitions: Vec<(String, String)>,
    pub undefinitions: Vec<String>,
    pub generated_rust: bool,
    pub native_x64_jit: bool,
    pub reject_fallback: bool,
    pub checks: Vec<(String, bool)>,
    pub cell_bindings: Vec<(String, String)>,
    pub qualification_attempts: usize,
    pub recent_qualifications: Vec<VerilogAQualificationHistoryRow>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAQualificationHistoryRow {
    pub sequence: u64,
    pub recorded_at_unix_ms: u64,
    pub disposition: crate::state::ProjectSourceQualificationDisposition,
    pub selected_module: String,
    pub report_digest: crate::product::ContentDigest,
}

impl Clone for PendingVerilogACompile {
    fn clone(&self) -> Self {
        Self {
            token: self.token,
            receiver: Arc::clone(&self.receiver),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum VerilogACompileOutcome {
    Success(Box<rspice_veriloga::RuntimeCompileReport>),
    Failure(Vec<CodeEditorDiagnostic>),
}

/// Retained metadata from the latest compile of the exact current source.
#[derive(Debug, Clone)]
pub struct VerilogACompileReceipt {
    pub token: VerilogASourceOperationToken,
    pub module_name: String,
    pub analog_ports: usize,
    pub noise_sources: usize,
    pub state_variables: usize,
    pub bytecode_available: bool,
    pub native_jit: TargetQualification,
    pub wasm_interpreter: TargetQualification,
    pub generated_rust: TargetQualification,
    pub diagnostics: Arc<super::CodeDiagnosticCollection>,
    /// The exact runtime artifacts advertised by this receipt. Keeping the
    /// report alive prevents the Compile action from degrading into a metadata
    /// preview after publication to the engine's session registry.
    pub report: Arc<rspice_veriloga::RuntimeCompileReport>,
}

/// Unpack an editor compile receipt into the identity facts the engine layer
/// validates. The receipt also carries `CodeEditorDiagnostic`, so it stays
/// here rather than travelling down to `simulation::veriloga` with the
/// runtime it produces.
impl PreparedVerilogARuntime {
    pub fn try_from_current_bundle_receipt(
        project_id: crate::product::ProjectId,
        bundle: &crate::state::ProjectSourceBundle,
        receipt: &VerilogACompileReceipt,
    ) -> Result<Self, String> {
        Self::try_from_current_bundle_receipt_with_alias(
            project_id,
            bundle,
            receipt,
            receipt.module_name.clone(),
        )
    }

    pub fn try_from_current_bundle_receipt_with_alias(
        project_id: crate::product::ProjectId,
        bundle: &crate::state::ProjectSourceBundle,
        receipt: &VerilogACompileReceipt,
        netlist_alias: impl Into<String>,
    ) -> Result<Self, String> {
        Self::try_new(
            project_id,
            bundle,
            &receipt.token,
            &receipt.module_name,
            &receipt.report,
            netlist_alias,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetQualification {
    Available,
    Preview,
    QualificationOnly,
    Unsupported(String),
    Failed(String),
}

#[derive(Debug, Clone, Default)]
pub struct VerilogAWorkbenchState {
    pub import_requested: bool,
    /// One-shot command-palette/menu request. The surface consumes this only
    /// after it owns an egui context, then the ordinary revision-bound compile
    /// transaction becomes authoritative.
    pub compile_requested: bool,
    pub compile_dialog: Option<VerilogACompileDialogState>,
    /// A dependency import is bound to the exact bundle/importer that launched
    /// the asynchronous picker. `None` retains File > Import's root-source
    /// replacement behavior.
    pub import_target: Option<VerilogAImportTarget>,
    pub root_import_target: Option<VerilogARootImportTarget>,
    /// Selected source document in the model-project navigator.
    pub selected_file: Option<VerilogAFileSelection>,
    /// Explicit module selection for multi-file Code Workspace bundles. Cell
    /// views continue to use their governed `veriloga.module` metadata.
    pub selected_module: String,
    pub pending: Option<PendingVerilogACompile>,
    pub receipt: Option<VerilogACompileReceipt>,
    pub last_failure: Arc<super::CodeDiagnosticCollection>,
    pub last_failure_token: Option<VerilogASourceOperationToken>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAImportTarget {
    pub project_id: crate::product::ProjectId,
    pub bundle_id: crate::state::ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: crate::product::ContentDigest,
    pub importer_path: String,
}

/// Root-source replacement/creation authority captured before an asynchronous
/// native or browser picker opens. `None` bundle identity means the exact
/// project had no Code Workspace Verilog-A bundle at request time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerilogARootImportTarget {
    pub project_id: crate::product::ProjectId,
    pub bundle_identity: Option<(
        crate::state::ProjectSourceId,
        u64,
        crate::product::ContentDigest,
    )>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAFileSelection {
    pub bundle_id: crate::state::ProjectSourceId,
    pub logical_path: String,
}

#[derive(Debug, Clone, Default)]
pub struct AutomationWorkbenchState {
    /// Selected logical path in the project-owned Automation bundle. `None`
    /// resolves to the bundle root; no demonstration filename is identity.
    pub selected_file: Option<String>,
    pub receipt: Option<AutomationValidationReceipt>,
    pub manifest: Option<crate::automation_workflow::AutomationWorkspaceManifest>,
    /// Exact immutable request snapshot accepted by the worker protocol. It
    /// is cleared on any source, role, permission, environment, or project
    /// revision change and is never reconstructed from UI labels.
    pub runtime_snapshot: Option<rspice_automation_protocol::SourceSnapshot>,
    /// Native worker request that is authoritatively compiling the exact
    /// current closure. A static editor parse never creates a validation
    /// receipt for Python.
    pub pending_runtime_validation: Option<PendingAutomationRuntimeValidation>,
    /// A user Run/Dry Run action that first needed authoritative managed-
    /// Python validation. The exact action is resumed only if that same source
    /// token is accepted; source edits clear it through the ordinary runtime
    /// invalidation path.
    pub deferred_runtime_launch: Option<AutomationRuntimeLaunchMode>,
    /// Authenticated capability-broker session for one exact Python launch.
    /// Handles in this ledger are host-issued, request-scoped identities; they
    /// are never inferred from script variables or UI labels.
    pub runtime_execution: Option<PendingAutomationRuntimeExecution>,
    pub diagnostics: Arc<super::CodeDiagnosticCollection>,
    pub diagnostic_token: Option<SourceOperationToken>,
    pub execution: AutomationExecutionState,
    pub cancel_requested: bool,
    pub debug: AutomationDebugState,
    pub artifacts: AutomationArtifactStore,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AutomationValidationReceipt {
    pub token: SourceOperationToken,
}

#[derive(Debug, Clone)]
pub struct PendingAutomationRuntimeValidation {
    pub request_id: u64,
    pub bundle_id: crate::state::ProjectSourceId,
    pub token: SourceOperationToken,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomationRuntimeLaunchMode {
    Run,
    DryRun,
    Debug,
}

impl AutomationRuntimeLaunchMode {
    pub const fn is_dry_run(self) -> bool {
        matches!(self, Self::DryRun)
    }

    pub const fn is_debug(self) -> bool {
        matches!(self, Self::Debug)
    }
}

#[derive(Debug, Clone)]
pub struct PendingAutomationRuntimeExecution {
    pub request_id: u64,
    pub bundle_id: crate::state::ProjectSourceId,
    pub token: SourceOperationToken,
    pub mode: AutomationRuntimeLaunchMode,
    pub session_id: Option<uuid::Uuid>,
    pub last_call_id: u64,
    pub project_handle: Option<uuid::Uuid>,
    pub plan_handle: Option<uuid::Uuid>,
    pub preview_handle: Option<uuid::Uuid>,
    pub run_handle: Option<uuid::Uuid>,
    pub simulation_run_id: Option<crate::product::RunId>,
    pub pending_execute_call_id: Option<u64>,
    pub requirements_evaluated: bool,
    pub comparison_completed: bool,
    pub artifacts_exported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutomationDebugPhase {
    #[default]
    Inactive,
    Paused,
    Running,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutomationBreakpointKind {
    #[default]
    Stop,
    Conditional,
    Logpoint,
    HitCount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomationBreakpoint {
    pub breakpoint_id: uuid::Uuid,
    pub document_id: uuid::Uuid,
    pub source_path: String,
    pub line: usize,
    pub enabled: bool,
    pub kind: AutomationBreakpointKind,
    pub expression: String,
    pub hit_count: u64,
    pub observed_hits: u64,
}

impl AutomationBreakpoint {
    pub fn stop(document_id: uuid::Uuid, source_path: String, line: usize) -> Self {
        Self {
            breakpoint_id: uuid::Uuid::new_v4(),
            document_id,
            source_path,
            line,
            enabled: true,
            kind: AutomationBreakpointKind::Stop,
            expression: String::new(),
            hit_count: 0,
            observed_hits: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutomationExceptionPolicy {
    All,
    #[default]
    Uncaught,
    Never,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomationDebugFrame {
    pub frame_id: u64,
    pub document_id: uuid::Uuid,
    pub name: String,
    pub source_path: String,
    pub line: usize,
    pub locals_reference: u64,
    pub globals_reference: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomationDebugValue {
    pub name: String,
    pub type_name: String,
    pub display_value: String,
    pub redacted: bool,
    pub variables_reference: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutomationDebugVariableScope {
    Locals,
    Globals,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomationWatch {
    pub expression: String,
    pub value: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomationStructuredOutput {
    pub sequence: u64,
    pub level: Arc<str>,
    pub event: Arc<str>,
    pub detail: Arc<str>,
}

pub const MAX_AUTOMATION_STRUCTURED_LOG_RECORDS: usize = 5_000_000;
pub const MAX_AUTOMATION_ARTIFACT_RECORDS: usize = 100_000;
const AUTOMATION_LOG_CHUNK_RECORDS: usize = 4_096;

#[derive(Debug, Clone, Default)]
pub struct AutomationStructuredOutputStore {
    chunks: Vec<Vec<AutomationStructuredOutput>>,
    interned_labels: std::collections::HashMap<Arc<str>, Arc<str>>,
    len: usize,
    next_sequence: u64,
}

impl AutomationStructuredOutputStore {
    pub fn try_push(&mut self, level: &str, event: &str, detail: &str) -> Result<u64, String> {
        validate_collection_count(
            "Structured Automation log",
            self.len.saturating_add(1),
            MAX_AUTOMATION_STRUCTURED_LOG_RECORDS,
        )?;
        self.next_sequence = self.next_sequence.saturating_add(1).max(1);
        let level = intern_automation_label(&mut self.interned_labels, level);
        let event = intern_automation_label(&mut self.interned_labels, event);
        if self
            .chunks
            .last()
            .is_none_or(|chunk| chunk.len() == AUTOMATION_LOG_CHUNK_RECORDS)
        {
            self.chunks
                .push(Vec::with_capacity(AUTOMATION_LOG_CHUNK_RECORDS));
        }
        let chunk = self.chunks.last_mut().ok_or_else(|| {
            "Structured Automation log storage could not allocate a chunk.".to_owned()
        })?;
        chunk.push(AutomationStructuredOutput {
            sequence: self.next_sequence,
            level,
            event,
            detail: Arc::from(detail),
        });
        self.len += 1;
        Ok(self.next_sequence)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &AutomationStructuredOutput> {
        self.chunks.iter().flat_map(|chunk| chunk.iter())
    }
}

fn intern_automation_label(
    pool: &mut std::collections::HashMap<Arc<str>, Arc<str>>,
    value: &str,
) -> Arc<str> {
    if let Some(existing) = pool.get(value) {
        return Arc::clone(existing);
    }
    let value: Arc<str> = Arc::from(value);
    pool.insert(Arc::clone(&value), Arc::clone(&value));
    value
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomationArtifactRecord {
    sequence: u64,
    kind: crate::automation_workflow::ArtifactKind,
    digest: ContentDigest,
    bytes: Arc<[u8]>,
}

impl AutomationArtifactRecord {
    #[cfg(test)]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub const fn kind(&self) -> crate::automation_workflow::ArtifactKind {
        self.kind
    }

    pub const fn file_name(&self) -> &'static str {
        self.kind.file_name()
    }

    pub const fn media_type(&self) -> &'static str {
        self.kind.media_type()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, Default)]
pub struct AutomationArtifactStore {
    records: Vec<AutomationArtifactRecord>,
    blobs: HashMap<ContentDigest, Arc<[u8]>>,
    latest_by_kind: HashMap<crate::automation_workflow::ArtifactKind, usize>,
}

impl AutomationArtifactStore {
    pub fn try_new(
        records: Vec<crate::automation_workflow::RenderedArtifact>,
    ) -> Result<Self, String> {
        let mut store = Self::default();
        store.records.reserve(records.len());
        for record in records {
            store.try_push(record)?;
        }
        Ok(store)
    }

    pub fn try_push(
        &mut self,
        artifact: crate::automation_workflow::RenderedArtifact,
    ) -> Result<u64, String> {
        validate_collection_count(
            "Automation artifact collection",
            self.records.len().saturating_add(1),
            MAX_AUTOMATION_ARTIFACT_RECORDS,
        )?;
        let digest = artifact.digest();
        let bytes = if let Some(existing) = self.blobs.get(&digest) {
            if existing.as_ref() != artifact.bytes() {
                return Err(format!(
                    "Automation artifact digest collision for {digest}; no record was retained."
                ));
            }
            Arc::clone(existing)
        } else {
            let bytes = artifact.shared_bytes();
            self.blobs.insert(digest, Arc::clone(&bytes));
            bytes
        };
        let sequence = u64::try_from(self.records.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| "Automation artifact sequence space is exhausted.".to_owned())?;
        let kind = artifact.kind();
        self.records.push(AutomationArtifactRecord {
            sequence,
            kind,
            digest,
            bytes,
        });
        self.latest_by_kind.insert(kind, self.records.len() - 1);
        Ok(sequence)
    }

    pub fn get(
        &self,
        kind: crate::automation_workflow::ArtifactKind,
    ) -> Option<&AutomationArtifactRecord> {
        self.latest_by_kind
            .get(&kind)
            .and_then(|index| self.records.get(*index))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, AutomationArtifactRecord> {
        self.records.iter()
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.blobs.clear();
        self.latest_by_kind.clear();
    }
}

fn validate_collection_count(label: &str, count: usize, maximum: usize) -> Result<(), String> {
    if count > maximum {
        return Err(format!(
            "{label} contains {count} records; the supported maximum is {maximum}."
        ));
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct AutomationDebugState {
    pub phase: AutomationDebugPhase,
    pub pause_requested: bool,
    pub current_line: Option<usize>,
    pub selected_frame_id: Option<u64>,
    pub breakpoints: Vec<AutomationBreakpoint>,
    pub call_stack: Vec<AutomationDebugFrame>,
    pub locals: Vec<AutomationDebugValue>,
    pub globals: Vec<AutomationDebugValue>,
    pub watches: Vec<AutomationWatch>,
    pub watch_input: String,
    pub exception_policy: AutomationExceptionPolicy,
    pub output: AutomationStructuredOutputStore,
    pub last_error: Option<String>,
    pub pending_stack_request_id: Option<u64>,
    pub pending_variable_requests: std::collections::BTreeMap<u64, AutomationDebugVariableScope>,
    pub pending_evaluation_requests: std::collections::BTreeMap<u64, String>,
}

impl Default for AutomationDebugState {
    fn default() -> Self {
        Self {
            phase: AutomationDebugPhase::Inactive,
            pause_requested: false,
            current_line: None,
            selected_frame_id: None,
            breakpoints: Vec::new(),
            call_stack: Vec::new(),
            locals: Vec::new(),
            globals: Vec::new(),
            watches: Vec::new(),
            watch_input: String::new(),
            exception_policy: AutomationExceptionPolicy::Uncaught,
            output: AutomationStructuredOutputStore::default(),
            last_error: None,
            pending_stack_request_id: None,
            pending_variable_requests: std::collections::BTreeMap::new(),
            pending_evaluation_requests: std::collections::BTreeMap::new(),
        }
    }
}

/// Complete immutable authority captured immediately before an Automation run
/// is dispatched. Evidence publication is refused if any live identity or
/// governed configuration diverges from this snapshot while execution is in
/// flight.
#[derive(Debug, Clone)]
pub struct AutomationDispatchSnapshot {
    pub project_id: crate::product::ProjectId,
    pub project_revision: crate::product::ObjectRevision,
    pub plan_id: crate::product::SimulationPlanId,
    pub plan_revision: crate::product::ObjectRevision,
    /// Digest of the exact immutable prepared-run snapshot authorized for
    /// this workflow invocation. This authenticates PVT, solver options,
    /// model bindings, source bytes, target capabilities, task order, and
    /// every analysis configuration consumed by execution.
    pub prepared_snapshot_digest: crate::product::ContentDigest,
    /// Explicit source identity retained alongside the whole-snapshot digest
    /// so run correlation fails closed if either receipt field diverges.
    pub source_content_digest: crate::product::ContentDigest,
    pub plan_name: String,
    pub plan_payload: crate::state::SimulationPlanPayload,
    pub project_sources: crate::state::ProjectSourceRegistry,
    pub baseline_run: Arc<crate::state::SimulationRun>,
    pub baseline_digest: crate::product::ContentDigest,
}

#[derive(Debug, Clone, Default)]
pub enum AutomationExecutionState {
    #[default]
    Idle,
    PythonBroker {
        token: SourceOperationToken,
        dry_run: bool,
    },
    PythonComplete {
        dry_run: bool,
    },
    AwaitingDispatch {
        token: SourceOperationToken,
        plan: crate::automation_workflow::AutomationPlan,
        prior_run_ids: Vec<crate::product::RunId>,
        snapshot: AutomationDispatchSnapshot,
    },
    Running {
        token: SourceOperationToken,
        plan: crate::automation_workflow::AutomationPlan,
        run_id: crate::product::RunId,
        snapshot: AutomationDispatchSnapshot,
    },
    Complete {
        passed: bool,
        run_id: crate::product::RunId,
    },
    Cancelled,
    Failed,
}

impl AutomationExecutionState {
    pub const fn is_active(&self) -> bool {
        matches!(
            self,
            Self::PythonBroker { .. } | Self::AwaitingDispatch { .. } | Self::Running { .. }
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct CodeWorkspaceRuntimeState {
    pub page: CodeWorkspacePage,
    /// Whether the source-document lifecycle dialog is showing. The
    /// transactions below stay independent of it: a file operation opened from
    /// the navigator brings the dialog up around its own review section, and
    /// closing the dialog abandons the draft rather than committing it.
    pub(crate) source_document_dialog: bool,
    pub source_workspace_dialog: Option<CodeSourceWorkspaceDialogState>,
    pub source_file_dialog: Option<CodeSourceFileDialogState>,
    pub source_history: Option<CodeSourceHistoryState>,
    pub source_import: Option<CodeSourceImportState>,
    pub source_search: Option<CodeSourceSearchState>,
    pub language_tools: Option<super::CodeLanguageToolsState>,
    /// Language index retained against the exact closure it was built from,
    /// so the source navigators can run every frame without re-parsing the
    /// bundle on each one.
    pub(crate) source_index_cache: Option<super::SourceIndexCache>,
    pub source_carets: std::collections::BTreeMap<(ProjectSourceLanguage, String), usize>,
    pub source_selections:
        std::collections::BTreeMap<(ProjectSourceLanguage, String), (usize, usize)>,
    pub veriloga: VerilogAWorkbenchState,
    pub automation: AutomationWorkbenchState,
    /// Directory typed into the include-search-path row of the source
    /// lifecycle dialog, before it is added to the project's ordered chain.
    pub(crate) include_search_draft: String,
    /// Selected entry of that ordered chain. Order is the setting, so one
    /// selection owns move-up, move-down and remove rather than every row
    /// repeating three controls.
    pub(crate) include_search_selected: Option<usize>,
    /// Where each of the Netlist page's retained includes resolved, keyed by
    /// the locator its own card wrote.
    ///
    /// Evidence about one exact resolution rather than a durable property of
    /// the deck: written when the closure is resolved against the host, and
    /// deliberately not persisted, because a project restored elsewhere has
    /// not walked that chain.
    pub(crate) include_resolutions:
        std::collections::BTreeMap<String, rspice_core::netlist::IncludeResolution>,
    /// The `.lib` sections each retained include's exact bytes declare, keyed
    /// by the same locator and held against the digest they were read from.
    ///
    /// Derived from the retained bytes rather than from the host file, and
    /// deliberately not persisted, for the same reason as the resolutions
    /// above. It is a cache because enumerating a foundry corner file costs a
    /// library parse and the navigator asks what every row offers on each
    /// frame; the digest is what makes a stale entry unobservable.
    pub(crate) include_lib_sections: std::collections::BTreeMap<
        String,
        (
            crate::product::ContentDigest,
            std::sync::Arc<[rspice_core::library::LibSectionSummary]>,
        ),
    >,
}

use super::CodeEditorDiagnostic;

#[cfg(test)]
mod tests {
    use super::*;
    // These cover the receipt -> runtime adapter, so they stay with the
    // receipt. What they assert about the runtime itself now comes from
    // the engine layer.
    use crate::simulation::veriloga::{
        PreparedVerilogARuntimeSet, append_project_veriloga_directive, project_veriloga_directive,
    };

    fn compiled_runtime(file_name: &str, module_name: &str) -> PreparedVerilogARuntime {
        compiled_runtime_with_alias(file_name, module_name, module_name)
    }

    fn compiled_runtime_with_alias(
        file_name: &str,
        module_name: &str,
        alias: &str,
    ) -> PreparedVerilogARuntime {
        let project_id = crate::product::ProjectId::new();
        let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::code_workspace(
                crate::state::ProjectSourceLanguage::VerilogA,
            ),
            crate::state::ProjectSourceLanguage::VerilogA,
            file_name,
            format!(
                "module {module_name}(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
            ),
            [],
            [],
        )
        .unwrap();
        let receipt =
            super::super::compile_project_bundle_receipt(project_id, &bundle, Some(module_name))
                .unwrap();
        PreparedVerilogARuntime::try_from_current_bundle_receipt_with_alias(
            project_id, &bundle, &receipt, alias,
        )
        .unwrap()
    }

    #[test]
    fn edited_singleton_bundle_keeps_stable_identity_for_receipt_and_runtime() {
        let project_id = crate::product::ProjectId::new();
        let mut registry = crate::state::ProjectSourceRegistry::try_from_documents([
            crate::state::ProjectSourceDocument::try_new(
                "stable.va",
                crate::state::ProjectSourceLanguage::VerilogA,
                "module stable(p, n); inout p, n; electrical p, n; endmodule\n",
            )
            .unwrap(),
        ])
        .unwrap();
        let owner = crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::VerilogA,
        );
        let stable_id = registry.bundle_for_owner(&owner).unwrap().id();
        registry
            .replace_bundle_file_content(
                stable_id,
                "stable.va",
                "module stable(p, n); inout p, n; electrical p, n; real revision_two; endmodule\n"
                    .to_owned(),
            )
            .unwrap();
        let bundle = registry.bundle_for_owner(&owner).unwrap();
        assert_eq!(bundle.id(), stable_id);
        let receipt = super::super::compile_project_bundle_receipt(project_id, bundle, None)
            .expect("edited current bundle compiles");

        let runtime =
            PreparedVerilogARuntime::try_from_current_bundle_receipt(project_id, bundle, &receipt);

        assert!(runtime.is_ok());
        assert_eq!(receipt.token.bundle_id, stable_id);
    }

    #[test]
    fn project_runtime_directive_is_inserted_before_end_once() {
        let mut deck = "R1 in 0 1k\n.end\n".to_owned();
        append_project_veriloga_directive(&mut deck, "__rspice_project__/p/d/model.va", "owned");
        append_project_veriloga_directive(&mut deck, "__rspice_project__/p/d/model.va", "owned");

        assert_eq!(
            deck,
            "R1 in 0 1k\n.veriloga \"__rspice_project__/p/d/model.va\" owned\n.end\n"
        );
    }

    #[test]
    fn runtime_serialization_preserves_exact_artifacts_and_rejects_tampering() {
        let runtime = compiled_runtime("model.va", "owned");
        let encoded = serde_json::to_vec(&runtime).unwrap();
        let restored: PreparedVerilogARuntime = serde_json::from_slice(&encoded).unwrap();

        assert_eq!(restored, runtime);
        assert!(restored.validate().is_ok());

        // Tamper through the serialized form rather than the field. The
        // payload is private to `simulation::veriloga` now, and a persisted
        // artifact edited on disk is the case that actually matters.
        let mut document: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
        let model_json = document["model_json"].as_str().unwrap().to_owned();
        document["model_json"] = serde_json::Value::String(format!("{model_json} "));
        let tampered: PreparedVerilogARuntime = serde_json::from_value(document).unwrap();
        assert!(tampered.validate().is_err());
    }

    #[test]
    fn directive_alias_is_the_compiled_module_not_the_file_stem() {
        let runtime = compiled_runtime("unrelated_file_name.va", "owned");
        assert_eq!(
            project_veriloga_directive(runtime.source_key(), runtime.module_name()),
            format!(".veriloga \"{}\" owned", runtime.source_key())
        );
    }

    #[test]
    fn runtime_set_is_canonical_and_rejects_key_or_alias_collisions() {
        let first = compiled_runtime_with_alias("first.va", "shared", "rspice_va_first");
        let second = compiled_runtime_with_alias("second.va", "shared", "rspice_va_second");
        let set = PreparedVerilogARuntimeSet::try_new(vec![second.clone(), first.clone()]).unwrap();
        let keys = set
            .iter()
            .map(PreparedVerilogARuntime::source_key)
            .collect::<Vec<_>>();
        assert!(
            keys.windows(2)
                .all(|pair| { pair[0].to_ascii_lowercase() < pair[1].to_ascii_lowercase() })
        );
        assert!(set.validate().is_ok());

        assert!(PreparedVerilogARuntimeSet::try_new(vec![first.clone(), first]).is_err());
        let alias_collision = compiled_runtime_with_alias("third.va", "other", "rspice_va_second");
        assert!(PreparedVerilogARuntimeSet::try_new(vec![second, alias_collision]).is_err());
    }

    #[test]
    fn runtime_set_round_trip_preserves_every_artifact_and_alias() {
        let runtimes = PreparedVerilogARuntimeSet::try_new(vec![
            compiled_runtime_with_alias("one.va", "one", "rspice_va_one"),
            compiled_runtime_with_alias("two.va", "two", "rspice_va_two"),
        ])
        .unwrap();
        let encoded = serde_json::to_vec(&runtimes).unwrap();
        let restored: PreparedVerilogARuntimeSet = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(restored, runtimes);
        assert!(restored.validate().is_ok());
    }

    #[test]
    fn structured_output_store_chunks_without_discarding_early_records() {
        let mut output = AutomationStructuredOutputStore::default();
        for index in 0..=AUTOMATION_LOG_CHUNK_RECORDS {
            output
                .try_push("info", "event", &index.to_string())
                .unwrap();
        }
        assert_eq!(output.len(), AUTOMATION_LOG_CHUNK_RECORDS + 1);
        assert_eq!(output.iter().next().unwrap().sequence, 1);
        assert_eq!(
            output.iter().next_back().unwrap().sequence,
            (AUTOMATION_LOG_CHUNK_RECORDS + 1) as u64
        );
        let first = output.iter().next().unwrap();
        let last = output.iter().next_back().unwrap();
        assert!(Arc::ptr_eq(&first.level, &last.level));
        assert!(Arc::ptr_eq(&first.event, &last.event));
    }

    #[test]
    fn automation_scale_limits_accept_exact_maxima_and_reject_one_more() {
        assert!(
            validate_collection_count(
                "Structured Automation log",
                MAX_AUTOMATION_STRUCTURED_LOG_RECORDS,
                MAX_AUTOMATION_STRUCTURED_LOG_RECORDS,
            )
            .is_ok()
        );
        assert!(
            validate_collection_count(
                "Structured Automation log",
                MAX_AUTOMATION_STRUCTURED_LOG_RECORDS + 1,
                MAX_AUTOMATION_STRUCTURED_LOG_RECORDS,
            )
            .is_err()
        );
        assert!(
            validate_collection_count(
                "Automation artifact collection",
                MAX_AUTOMATION_ARTIFACT_RECORDS,
                MAX_AUTOMATION_ARTIFACT_RECORDS,
            )
            .is_ok()
        );
        assert!(
            validate_collection_count(
                "Automation artifact collection",
                MAX_AUTOMATION_ARTIFACT_RECORDS + 1,
                MAX_AUTOMATION_ARTIFACT_RECORDS,
            )
            .is_err()
        );
    }

    #[test]
    fn artifact_store_accepts_exact_limit_with_content_addressed_payloads() {
        let artifact = crate::automation_workflow::RenderedArtifact::new(
            crate::automation_workflow::ArtifactKind::JunitXml,
            b"shared artifact payload".to_vec(),
        );
        let mut store = AutomationArtifactStore::default();
        for _ in 0..MAX_AUTOMATION_ARTIFACT_RECORDS {
            store.try_push(artifact.clone()).unwrap();
        }

        assert_eq!(store.iter().count(), MAX_AUTOMATION_ARTIFACT_RECORDS);
        assert_eq!(store.blobs.len(), 1);
        assert_eq!(
            store
                .get(crate::automation_workflow::ArtifactKind::JunitXml)
                .unwrap()
                .sequence(),
            MAX_AUTOMATION_ARTIFACT_RECORDS as u64
        );
        assert!(Arc::ptr_eq(
            &store.records.first().unwrap().bytes,
            &store.records.last().unwrap().bytes
        ));
        assert!(store.try_push(artifact).is_err());
    }
}
