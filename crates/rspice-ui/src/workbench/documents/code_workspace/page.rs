//! Runtime page and compile/execution state for the Code & Automation workspace.
//!
//! Source bytes and their persisted revisions belong to `ProjectWorkspace`.
//! This module owns only the currently visible page and asynchronous operation
//! receipts that must never survive a process restart.

use crate::simulation::veriloga::{PreparedVerilogARuntime, VerilogASourceOperationToken};

use std::sync::{Arc, Mutex, mpsc};

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
    /// A dependency import is bound to the exact bundle/importer that launched
    /// the asynchronous picker. `None` retains File > Import's root-source
    /// replacement behavior.
    pub import_target: Option<VerilogAImportTarget>,
    /// Selected source document in the model-project navigator.
    pub selected_file: Option<VerilogAFileSelection>,
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
}
