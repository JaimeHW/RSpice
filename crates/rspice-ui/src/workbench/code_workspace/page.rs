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

/// Exact project-owned Verilog-A closure captured when compilation starts.
///
/// Unlike automation's single-document token, this identity includes the
/// stable bundle owner and the digest of every file in its sealed dependency
/// closure. A result can therefore never cross-publish between two cell views
/// that happen to contain identical root text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerilogASourceOperationToken {
    pub project_id: crate::product::ProjectId,
    pub bundle_id: crate::state::ProjectSourceId,
    pub revision: u64,
    pub closure_digest: crate::product::ContentDigest,
    /// Exact explicit module selection requested by a cell-view contract.
    /// `None` preserves the Code Workspace's compiler-selected module mode.
    pub requested_module_digest: Option<crate::product::ContentDigest>,
}

#[derive(Debug)]
pub struct PendingVerilogACompile {
    pub token: VerilogASourceOperationToken,
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
    netlist_alias: String,
    model_json: String,
    canonical_ir_json: String,
}

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
        if receipt.token.project_id != project_id
            || receipt.token.bundle_id != bundle.id()
            || receipt.token.revision != bundle.revision().get()
            || receipt.token.closure_digest != bundle.closure_digest()
            || receipt
                .token
                .requested_module_digest
                .is_some_and(|expected| {
                    expected != veriloga_selected_module_digest(&receipt.module_name)
                })
        {
            return Err(
                "The retained Verilog-A runtime does not identify the exact current project source"
                    .to_owned(),
            );
        }
        let source_key = crate::state::project_veriloga_bundle_source_key(
            project_id,
            bundle,
            &receipt.module_name,
        )
        .map_err(|error| error.to_string())?;
        let netlist_alias = netlist_alias.into();
        let model_json = serde_json::to_string(&receipt.report.model)
            .map_err(|error| format!("Could not serialize compiled Verilog-A model: {error}"))?;
        let canonical_ir_json = serde_json::to_string(&receipt.report.canonical_ir)
            .map_err(|error| format!("Could not serialize canonical Verilog-A IR: {error}"))?;
        let artifact_digest = runtime_artifact_digest(
            &source_key,
            bundle.closure_digest(),
            &receipt.module_name,
            &netlist_alias,
            &model_json,
            &canonical_ir_json,
        );
        let runtime = Self {
            source_key,
            source_digest: bundle.closure_digest(),
            artifact_digest,
            module_name: receipt.module_name.clone(),
            netlist_alias,
            model_json,
            canonical_ir_json,
        };
        runtime.validate()?;
        Ok(runtime)
    }

    fn try_from_virtual_compilation(
        source_key: String,
        source_digest: crate::product::ContentDigest,
        netlist_alias: String,
        compilation: &rspice_veriloga::VirtualRuntimeCompilation,
    ) -> Result<Self, String> {
        compilation
            .validate_integrity()
            .map_err(|error| format!("Compiled Verilog-A bundle is invalid: {error}"))?;
        let model_json = serde_json::to_string(&compilation.runtime.model)
            .map_err(|error| format!("Could not serialize compiled Verilog-A model: {error}"))?;
        let canonical_ir_json = serde_json::to_string(&compilation.runtime.canonical_ir)
            .map_err(|error| format!("Could not serialize canonical Verilog-A IR: {error}"))?;
        let artifact_digest = runtime_artifact_digest(
            &source_key,
            source_digest,
            &compilation.selected_module,
            &netlist_alias,
            &model_json,
            &canonical_ir_json,
        );
        let runtime = Self {
            source_key,
            source_digest,
            artifact_digest,
            module_name: compilation.selected_module.clone(),
            netlist_alias,
            model_json,
            canonical_ir_json,
        };
        runtime.validate()?;
        Ok(runtime)
    }

    pub fn install(&self) -> Result<(), String> {
        rspice_core::register_project_veriloga_runtimes_for_session([self.registration()?])
    }

    fn registration(&self) -> Result<rspice_core::ProjectVerilogARuntimeRegistration, String> {
        self.validate()?;
        let model: rspice_veriloga::CompiledModel = serde_json::from_str(&self.model_json)
            .map_err(|error| format!("Compiled Verilog-A model payload is invalid: {error}"))?;
        let canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
            serde_json::from_str(&self.canonical_ir_json)
                .map_err(|error| format!("Canonical Verilog-A IR payload is invalid: {error}"))?;
        Ok(rspice_core::ProjectVerilogARuntimeRegistration {
            source_key: self.source_key.clone().into(),
            aliases: vec![self.netlist_alias.clone()],
            model,
            canonical_ir,
        })
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
        if !valid_veriloga_netlist_identifier(&self.netlist_alias) {
            return Err("Verilog-A runtime has an invalid netlist alias".to_owned());
        }
        let expected = runtime_artifact_digest(
            &self.source_key,
            self.source_digest,
            &self.module_name,
            &self.netlist_alias,
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

    pub fn netlist_alias(&self) -> &str {
        &self.netlist_alias
    }

    pub fn terminal_names(&self) -> Result<Vec<String>, String> {
        self.validate()?;
        let model: rspice_veriloga::CompiledModel = serde_json::from_str(&self.model_json)
            .map_err(|error| format!("Compiled Verilog-A model payload is invalid: {error}"))?;
        Ok(model
            .terminal_names
            .iter()
            .map(ToString::to_string)
            .collect())
    }
}

pub(crate) fn veriloga_selected_module_digest(module_name: &str) -> crate::product::ContentDigest {
    let mut hasher = Sha256::new();
    let domain = b"rspice.veriloga-selected-module/v1";
    hasher.update((domain.len() as u64).to_be_bytes());
    hasher.update(domain);
    hasher.update((module_name.len() as u64).to_be_bytes());
    hasher.update(module_name.as_bytes());
    crate::product::ContentDigest::from_bytes(hasher.finalize().into())
}

/// Canonically ordered set of every project-owned Verilog-A runtime required
/// by one immutable executable deck. The set rejects case-folded key/alias
/// collisions before worker transfer so model selection cannot depend on
/// discovery order.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PreparedVerilogARuntimeSet {
    runtimes: Vec<PreparedVerilogARuntime>,
}

impl PreparedVerilogARuntimeSet {
    pub fn try_new(mut runtimes: Vec<PreparedVerilogARuntime>) -> Result<Self, String> {
        for runtime in &runtimes {
            runtime.validate()?;
        }
        runtimes.sort_by(|left, right| {
            left.source_key
                .to_ascii_lowercase()
                .cmp(&right.source_key.to_ascii_lowercase())
                .then_with(|| {
                    left.netlist_alias
                        .to_ascii_lowercase()
                        .cmp(&right.netlist_alias.to_ascii_lowercase())
                })
        });
        for pair in runtimes.windows(2) {
            if pair[0].source_key.eq_ignore_ascii_case(&pair[1].source_key) {
                return Err(format!(
                    "Verilog-A runtime source key '{}' is duplicated",
                    pair[1].source_key
                ));
            }
        }
        let mut aliases = std::collections::HashMap::<String, crate::product::ContentDigest>::new();
        for runtime in &runtimes {
            let alias = runtime.netlist_alias.to_ascii_uppercase();
            if let Some(existing) = aliases.insert(alias, runtime.artifact_digest)
                && existing != runtime.artifact_digest
            {
                return Err(format!(
                    "Verilog-A netlist alias '{}' identifies different prepared artifacts",
                    runtime.netlist_alias
                ));
            }
        }
        Ok(Self { runtimes })
    }

    pub fn validate(&self) -> Result<(), String> {
        Self::try_new(self.runtimes.clone()).and_then(|canonical| {
            if canonical == *self {
                Ok(())
            } else {
                Err("Verilog-A runtime set is not in canonical order".to_owned())
            }
        })
    }

    pub fn is_empty(&self) -> bool {
        self.runtimes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.runtimes.len()
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &PreparedVerilogARuntime> {
        self.runtimes.iter()
    }

    pub fn install(&self) -> Result<(), String> {
        self.validate()?;
        let registrations = self
            .runtimes
            .iter()
            .map(PreparedVerilogARuntime::registration)
            .collect::<Result<Vec<_>, _>>()?;
        rspice_core::register_project_veriloga_runtimes_for_session(registrations)
    }
}

pub(crate) fn compile_project_source_bundle_runtime(
    project_id: crate::product::ProjectId,
    bundle: &crate::state::ProjectSourceBundle,
    module_name: &str,
) -> Result<PreparedVerilogARuntime, String> {
    if bundle.language() != crate::state::ProjectSourceLanguage::VerilogA {
        return Err(format!(
            "Project source bundle {} is {}, not Verilog-A",
            bundle.id(),
            bundle.language()
        ));
    }
    let source_key =
        crate::state::project_veriloga_bundle_source_key(project_id, bundle, module_name)
            .map_err(|error| error.to_string())?;
    let netlist_alias = crate::state::project_veriloga_bundle_alias(bundle, module_name)
        .map_err(|error| error.to_string())?;
    let files =
        std::iter::once(rspice_veriloga::VirtualSourceFile::new(
            bundle.root().logical_path(),
            bundle.root().content(),
        ))
        .chain(bundle.files().iter().map(|file| {
            rspice_veriloga::VirtualSourceFile::new(file.logical_path(), file.content())
        }));
    let virtual_bundle =
        rspice_veriloga::VirtualSourceBundle::new(bundle.root().logical_path(), files)
            .map_err(|error| format!("Project Verilog-A bundle is invalid: {error}"))?;
    let limits = super::veriloga::project_virtual_compile_limits();
    let compilation = rspice_veriloga::VerilogACompiler::default()
        .compile_virtual_runtime(&virtual_bundle, module_name, limits)
        .map_err(|error| {
            format!(
                "Could not compile Verilog-A module '{module_name}' from project bundle {}: {error}",
                bundle.id()
            )
        })?;
    PreparedVerilogARuntime::try_from_virtual_compilation(
        source_key,
        bundle.closure_digest(),
        netlist_alias,
        &compilation,
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
    netlist_alias: &str,
    model_json: &str,
    canonical_ir_json: &str,
) -> crate::product::ContentDigest {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice.project-veriloga-runtime/v2\0");
    for bytes in [
        source_key.as_bytes(),
        source_digest.as_bytes(),
        module_name.as_bytes(),
        netlist_alias.as_bytes(),
        model_json.as_bytes(),
        canonical_ir_json.as_bytes(),
    ] {
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    crate::product::ContentDigest::from_bytes(hasher.finalize().into())
}

fn valid_veriloga_netlist_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
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
    /// A dependency import is bound to the exact bundle/importer that launched
    /// the asynchronous picker. `None` retains File > Import's root-source
    /// replacement behavior.
    pub import_target: Option<VerilogAImportTarget>,
    /// Selected source document in the model-project navigator.
    pub selected_file: Option<VerilogAFileSelection>,
    /// Inline add/rename transaction currently being authored.
    pub file_editor: Option<VerilogAFileEditorState>,
    /// Explicit module selection for multi-file Code Workspace bundles. Cell
    /// views continue to use their governed `veriloga.module` metadata.
    pub selected_module: String,
    pub pending: Option<PendingVerilogACompile>,
    pub receipt: Option<VerilogACompileReceipt>,
    pub last_failure: Vec<CodeEditorDiagnostic>,
    pub last_failure_token: Option<VerilogASourceOperationToken>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAImportTarget {
    pub bundle_id: crate::state::ProjectSourceId,
    pub importer_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAFileSelection {
    pub bundle_id: crate::state::ProjectSourceId,
    pub logical_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerilogAFileEditorKind {
    Add { importer_path: String },
    Rename { current_path: String },
    Delete { current_path: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAFileEditorState {
    pub bundle_id: crate::state::ProjectSourceId,
    pub kind: VerilogAFileEditorKind,
    pub logical_path: String,
    pub error: Option<String>,
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
}
