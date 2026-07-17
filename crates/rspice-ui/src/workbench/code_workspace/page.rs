//! Runtime page and compile/execution state for the Code & Automation workspace.
//!
//! Source bytes and their persisted revisions belong to `ProjectWorkspace`.
//! This module owns only the currently visible page and asynchronous operation
//! receipts that must never survive a process restart.

use std::sync::{Arc, Mutex, mpsc};

use sha2::{Digest as _, Sha256};

/// Mockup-defined page set. Reports are intentionally not a visible fourth tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeWorkspacePage {
    #[default]
    Netlist,
    VerilogA,
    Automation,
}

impl CodeWorkspacePage {
    pub const ALL: [Self; 3] = [Self::Netlist, Self::VerilogA, Self::Automation];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Netlist => "Netlist",
            Self::VerilogA => "Verilog-A",
            Self::Automation => "Automation",
        }
    }
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
    pub token: SourceOperationToken,
    pub receiver: Arc<Mutex<mpsc::Receiver<VerilogACompileOutcome>>>,
}

impl Clone for PendingVerilogACompile {
    fn clone(&self) -> Self {
        Self {
            token: self.token,
            receiver: Arc::clone(&self.receiver),
        }
    }
}

#[derive(Debug)]
pub enum VerilogACompileOutcome {
    Success(Box<rspice_veriloga::RuntimeCompileReport>),
    Failure(Vec<CodeEditorDiagnostic>),
}

/// Retained metadata from the latest compile of the exact current source.
#[derive(Debug, Clone)]
pub struct VerilogACompileReceipt {
    pub token: SourceOperationToken,
    pub module_name: String,
    pub analog_ports: usize,
    pub noise_sources: usize,
    pub state_variables: usize,
    pub semantic_ir_digest: String,
    pub bytecode_available: bool,
    pub native_jit: TargetQualification,
    pub wasm_interpreter: TargetQualification,
    pub generated_rust: TargetQualification,
    pub diagnostics: Vec<CodeEditorDiagnostic>,
    /// The exact runtime artifacts advertised by this receipt. Keeping the
    /// report alive prevents the Compile action from degrading into a metadata
    /// preview after publication to the engine's session registry.
    pub report: Arc<rspice_veriloga::RuntimeCompileReport>,
}

/// Immutable, worker-transferable Verilog-A runtime bound to one exact
/// project-owned source identity. The virtual source key is content addressed
/// and project scoped, so neither another open project nor an ambient file can
/// satisfy the directive accidentally.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PreparedVerilogARuntime {
    source_key: String,
    source_digest: crate::product::ContentDigest,
    artifact_digest: crate::product::ContentDigest,
    module_name: String,
    model_json: String,
    canonical_ir_json: String,
}

impl PreparedVerilogARuntime {
    pub fn try_from_current_receipt(
        project_id: crate::product::ProjectId,
        document: &crate::state::ProjectSourceDocument,
        receipt: &VerilogACompileReceipt,
    ) -> Result<Self, String> {
        if receipt.token.project_id != project_id
            || receipt.token.revision != document.revision().get()
            || receipt.token.content_digest != document.content_digest()
        {
            return Err(
                "The retained Verilog-A runtime does not identify the exact current project source"
                    .to_owned(),
            );
        }
        let source_key = project_veriloga_source_key(project_id, document);
        let model_json = serde_json::to_string(&receipt.report.model)
            .map_err(|error| format!("Could not serialize compiled Verilog-A model: {error}"))?;
        let canonical_ir_json = serde_json::to_string(&receipt.report.canonical_ir)
            .map_err(|error| format!("Could not serialize canonical Verilog-A IR: {error}"))?;
        let artifact_digest = runtime_artifact_digest(
            &source_key,
            document.content_digest(),
            &receipt.module_name,
            &model_json,
            &canonical_ir_json,
        );
        let runtime = Self {
            source_key,
            source_digest: document.content_digest(),
            artifact_digest,
            module_name: receipt.module_name.clone(),
            model_json,
            canonical_ir_json,
        };
        runtime.validate()?;
        Ok(runtime)
    }

    pub fn install(&self) -> Result<(), String> {
        self.validate()?;
        let model: rspice_veriloga::CompiledModel = serde_json::from_str(&self.model_json)
            .map_err(|error| format!("Compiled Verilog-A model payload is invalid: {error}"))?;
        let canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
            serde_json::from_str(&self.canonical_ir_json)
                .map_err(|error| format!("Canonical Verilog-A IR payload is invalid: {error}"))?;
        rspice_core::register_project_veriloga_runtime_for_session(
            &self.source_key,
            model,
            canonical_ir,
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.source_key.starts_with("__rspice_project__/")
            || self.source_key.contains('\\')
            || self.source_key.chars().any(char::is_control)
        {
            return Err("Verilog-A runtime has an invalid project virtual source key".to_owned());
        }
        if self.module_name.trim().is_empty() || self.module_name.chars().any(char::is_control) {
            return Err("Verilog-A runtime has an invalid module identity".to_owned());
        }
        let expected = runtime_artifact_digest(
            &self.source_key,
            self.source_digest,
            &self.module_name,
            &self.model_json,
            &self.canonical_ir_json,
        );
        if expected != self.artifact_digest {
            return Err("Verilog-A runtime artifact digest does not match its payload".to_owned());
        }
        let model: rspice_veriloga::CompiledModel = serde_json::from_str(&self.model_json)
            .map_err(|error| format!("Compiled Verilog-A model payload is invalid: {error}"))?;
        let canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
            serde_json::from_str(&self.canonical_ir_json)
                .map_err(|error| format!("Canonical Verilog-A IR payload is invalid: {error}"))?;
        if model.name.as_str() != self.module_name {
            return Err(format!(
                "Verilog-A runtime module '{}' does not match compiled model '{}'",
                self.module_name, model.name
            ));
        }
        if canonical_ir.hir.module_name.as_str() != self.module_name {
            return Err(format!(
                "Verilog-A canonical IR module '{}' does not match runtime module '{}'",
                canonical_ir.hir.module_name, self.module_name
            ));
        }
        Ok(())
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub const fn source_digest(&self) -> crate::product::ContentDigest {
        self.source_digest
    }

    pub const fn artifact_digest(&self) -> crate::product::ContentDigest {
        self.artifact_digest
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }
}

/// Deterministic virtual source identity shared by displayed generated decks,
/// prepared execution, worker transfer, and runtime-cache registration.
pub fn project_veriloga_source_key(
    project_id: crate::product::ProjectId,
    document: &crate::state::ProjectSourceDocument,
) -> String {
    format!(
        "__rspice_project__/{project_id}/{}/{}",
        document.content_digest(),
        document.file_name()
    )
}

/// Insert one project Verilog-A directive before the terminal `.end` card.
/// The exact same helper is used by the retained generated artifact and the
/// immutable prepared-run source, preventing display/execution drift.
pub fn project_veriloga_directive(source_key: &str, module_name: &str) -> String {
    format!(".veriloga \"{source_key}\" {module_name}")
}

pub fn append_project_veriloga_directive(source: &mut String, source_key: &str, module_name: &str) {
    let directive = project_veriloga_directive(source_key, module_name);
    if source
        .lines()
        .any(|line| line.trim().eq_ignore_ascii_case(&directive))
    {
        return;
    }
    let end = source
        .lines()
        .enumerate()
        .find_map(|(index, line)| line.trim().eq_ignore_ascii_case(".end").then_some(index));
    let retained_trailing_newline = source.ends_with('\n');
    let mut lines = source.lines().map(str::to_owned).collect::<Vec<_>>();
    lines.insert(end.unwrap_or(lines.len()), directive);
    *source = lines.join("\n");
    if retained_trailing_newline || !source.is_empty() {
        source.push('\n');
    }
}

fn runtime_artifact_digest(
    source_key: &str,
    source_digest: crate::product::ContentDigest,
    module_name: &str,
    model_json: &str,
    canonical_ir_json: &str,
) -> crate::product::ContentDigest {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.project-veriloga-runtime/v1\0");
    for bytes in [
        source_key.as_bytes(),
        source_digest.as_bytes(),
        module_name.as_bytes(),
        model_json.as_bytes(),
        canonical_ir_json.as_bytes(),
    ] {
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    crate::product::ContentDigest::from_bytes(hasher.finalize().into())
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
    pub import_in_progress: bool,
    pub pending: Option<PendingVerilogACompile>,
    pub receipt: Option<VerilogACompileReceipt>,
    pub last_failure: Vec<CodeEditorDiagnostic>,
    pub last_failure_token: Option<SourceOperationToken>,
}

#[derive(Debug, Clone, Default)]
pub struct AutomationWorkbenchState {
    pub receipt: Option<AutomationValidationReceipt>,
    pub diagnostics: Vec<CodeEditorDiagnostic>,
    pub diagnostic_token: Option<SourceOperationToken>,
    pub execution: AutomationExecutionState,
    pub artifacts: Vec<crate::automation_workflow::RenderedArtifact>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AutomationValidationReceipt {
    pub token: SourceOperationToken,
    pub plan: crate::automation_workflow::AutomationPlan,
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
        run_id: crate::product::RunId,
        passed: bool,
    },
    Failed,
}

impl AutomationExecutionState {
    pub const fn is_active(&self) -> bool {
        matches!(self, Self::AwaitingDispatch { .. } | Self::Running { .. })
    }

    pub const fn label(&self) -> &'static str {
        match self {
            Self::Idle => "validation required",
            Self::AwaitingDispatch { .. } => "preflight accepted",
            Self::Running { .. } => "workflow running",
            Self::Complete { passed: true, .. } => "release gates pass",
            Self::Complete { passed: false, .. } => "release gates failed",
            Self::Failed => "workflow failed",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CodeWorkspaceRuntimeState {
    pub page: CodeWorkspacePage,
    pub veriloga: VerilogAWorkbenchState,
    pub automation: AutomationWorkbenchState,
}

impl CodeWorkspaceRuntimeState {
    pub fn application_modal_open(&self) -> bool {
        false
    }
}

use super::CodeEditorDiagnostic;

#[cfg(test)]
mod tests {
    use super::*;

    fn compiled_runtime(file_name: &str, module_name: &str) -> PreparedVerilogARuntime {
        let project_id = crate::product::ProjectId::new();
        let document = crate::state::ProjectSourceDocument::try_new(
            file_name,
            crate::state::ProjectSourceLanguage::VerilogA,
            format!(
                "module {module_name}(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
            ),
        )
        .unwrap();
        let receipt = super::super::compile_project_source_receipt(project_id, &document).unwrap();
        PreparedVerilogARuntime::try_from_current_receipt(project_id, &document, &receipt).unwrap()
    }

    #[test]
    fn project_runtime_keys_are_content_and_project_scoped() {
        let document = crate::state::ProjectSourceDocument::try_new(
            "model.va",
            crate::state::ProjectSourceLanguage::VerilogA,
            "module model; endmodule\n",
        )
        .unwrap();
        let first = project_veriloga_source_key(crate::product::ProjectId::new(), &document);
        let second = project_veriloga_source_key(crate::product::ProjectId::new(), &document);

        assert_ne!(first, second);
        assert!(first.starts_with("__rspice_project__/"));
        assert!(first.ends_with("/model.va"));
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

        let mut tampered = restored;
        tampered.model_json.push(' ');
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
}
