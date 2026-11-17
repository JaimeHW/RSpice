//! Runtime state of the source documents: their dialogs and the Verilog-A
//! compile and import operations.
//!
//! Source bytes and their persisted revisions belong to `ProjectWorkspace`.
//! This module owns only transient dialog state and asynchronous operation
//! receipts that must never survive a process restart.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use crate::{
    product::{ContentDigest, ProjectId},
    state::{
        ProjectSourceId, ProjectSourceLanguage, VerilogACompileReceipt, VerilogAPlacementHandoff,
    },
};
use rspice_simulation::project_veriloga::VerilogASourceOperationToken;
#[cfg(test)]
use rspice_simulation::veriloga::PreparedVerilogARuntime;

use std::sync::{Arc, Mutex, mpsc};

use super::super::CodeSourceFileAction;

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

#[derive(Debug)]
pub struct PendingVerilogACompile {
    pub token: VerilogASourceOperationToken,
    pub receiver: Arc<Mutex<mpsc::Receiver<VerilogACompileOutcome>>>,
    /// The cells the design is compiled against, which its receipt names.
    pub cells: Option<Arc<[rspice_veriloga::ExternalCell]>>,
    /// The min:typ:max selection the design is compiled under, which its
    /// receipt names.
    pub delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
    pub started: web_time::Instant,
    pub cancellation: CompileCancellation,
}

impl PendingVerilogACompile {
    pub fn new(
        token: VerilogASourceOperationToken,
        receiver: mpsc::Receiver<VerilogACompileOutcome>,
        cells: Option<Arc<[rspice_veriloga::ExternalCell]>>,
        delay_select: rspice_veriloga::canonical_ir::digital::DigitalDelaySelect,
    ) -> Self {
        Self {
            token,
            receiver: Arc::new(Mutex::new(receiver)),
            cells,
            delay_select,
            started: web_time::Instant::now(),
            cancellation: CompileCancellation::default(),
        }
    }
}

impl Clone for PendingVerilogACompile {
    fn clone(&self) -> Self {
        Self {
            token: self.token,
            receiver: Arc::clone(&self.receiver),
            cells: self.cells.clone(),
            delay_select: self.delay_select,
            started: self.started,
            cancellation: self.cancellation.clone(),
        }
    }
}

pub use rspice_simulation::project_veriloga::compile_service::{
    CompileCancellation, VerilogACompileOutcome,
};

#[derive(Debug, Clone, Default)]
pub struct VerilogAWorkbenchState {
    pub import_requested: bool,
    /// A Compile the reader asked for: the source closure, build profile
    /// included, exactly as it stood at the click. The frame loop starts it
    /// on its next pass, where it has the egui context the worker wakes; a
    /// result for a closure that has moved on since is stale like any other.
    pub compile_request: Option<super::SelectedVerilogASource>,
    /// Add files is bound to the exact bundle revision and the file in front
    /// that launched the asynchronous picker.
    pub import_target: Option<VerilogAImportTarget>,
    /// "Replace source from file…" is bound to the exact bundle and file the
    /// editor showed when the picker opened. With neither target the picked
    /// file is File > Import's, and becomes cell views of its own.
    pub replace_target: Option<VerilogAReplaceTarget>,
    /// File > Import's form: a read file that becomes cell views of its own.
    pub import_form: Option<VerilogAImportFormState>,
    /// Further roots of the same pick or drop, each opening the form in turn
    /// once the one before it is committed or cancelled.
    pub import_queue: Vec<VerilogAImportFormState>,
    /// A picked file waiting for the confirmation that it replaces the file
    /// it was picked for. Nothing is replaced until it is confirmed.
    pub replace_review: Option<VerilogAReplaceReview>,
    /// Selected source document in the model-project navigator.
    pub selected_file: Option<VerilogAFileSelection>,
    /// A built-in standard header the editor well shows read-only in place of
    /// the selected file. Choosing a file of the bundle returns to editing.
    pub(crate) viewed_header: Option<VerilogAHeaderView>,
    pub pending: Option<PendingVerilogACompile>,
    pub receipt: Option<VerilogACompileReceipt>,
    pub last_failure: Arc<crate::state::code_diagnostics::CodeDiagnosticCollection>,
    pub last_failure_token: Option<VerilogASourceOperationToken>,
    /// How long the last finished compile took, and of which source.
    pub last_compile_duration: Option<(VerilogASourceOperationToken, std::time::Duration)>,
    /// A compile's result was thrown away because the source moved on while
    /// it ran. Stands until the next compile starts.
    pub compile_discarded: bool,
    /// The status bar has yet to say so.
    pub compile_discard_unannounced: bool,
    /// Where the open view's cell symbol stands against its interface.
    pub symbol_standing: super::veriloga_interface::VerilogASymbolStanding,
    /// Devices New Verilog-A device… created whose module the reader has not
    /// yet compiled into something placeable.
    pub placement_handoffs: Vec<VerilogAPlacementHandoff>,
    /// The check of the source while it is written, between compiles.
    pub live_check: VerilogALiveCheck,
}

/// The source is checked, short of generating code, once its writer pauses:
/// what the check finds is shown as the source's findings until it changes
/// again or is compiled. The browser build has no thread to check on.
#[derive(Debug, Clone, Default)]
pub struct VerilogALiveCheck {
    /// The source as it was last seen changed, and the frame time then.
    #[cfg(not(target_arch = "wasm32"))]
    pub edited: Option<(VerilogASourceOperationToken, f64)>,
    /// A check on its way, of the source its token names.
    #[cfg(not(target_arch = "wasm32"))]
    pub running: Option<RunningVerilogACheck>,
    /// What the last check of the source its token names found.
    pub findings: Option<(
        VerilogASourceOperationToken,
        Arc<crate::state::code_diagnostics::CodeDiagnosticCollection>,
    )>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct RunningVerilogACheck {
    pub token: VerilogASourceOperationToken,
    pub receiver: Arc<Mutex<mpsc::Receiver<Vec<super::CodeEditorDiagnostic>>>>,
    /// Stops the check; dropped with the last clone of this check (the
    /// project closed, or the evidence voided), it stops the check too.
    pub cancellation: CompileCancellation,
}

impl VerilogAWorkbenchState {
    /// Drop what the last compile proved, and stop a compile still running:
    /// the source either was of is not the source any more. The stopped work
    /// never holds the next compile up, and the reader is told it is gone.
    pub fn void_evidence(&mut self) {
        if let Some(pending) = self.pending.take() {
            pending.cancellation.cancel();
            self.note_compile_discarded();
        }
        self.receipt = None;
        Arc::make_mut(&mut self.last_failure).clear();
        self.last_failure_token = None;
        self.last_compile_duration = None;
        self.live_check = VerilogALiveCheck::default();
    }

    pub(crate) fn note_compile_discarded(&mut self) {
        self.compile_discarded = true;
        self.compile_discard_unannounced = true;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAImportTarget {
    pub project_id: crate::product::ProjectId,
    pub bundle_id: crate::state::ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: crate::product::ContentDigest,
    pub importer_path: String,
}

/// The file "Replace source from file…" was asked for, captured before the
/// picker opens: the exact bundle revision and the logical path the editor
/// was showing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAReplaceTarget {
    pub project_id: ProjectId,
    pub view: crate::state::CellViewRef,
    pub bundle_id: ProjectSourceId,
    pub bundle_revision: u64,
    pub closure_digest: ContentDigest,
    pub logical_path: String,
}

/// A picked file and the file it would replace, waiting for confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAReplaceReview {
    pub target: VerilogAReplaceTarget,
    pub file_name: String,
    pub contents: String,
    /// Whether the text being replaced stays in the bundle's revision
    /// history, where the source history restores it.
    pub restorable: bool,
}

/// A Verilog-A/AMS root file read for File > Import, with every file its
/// includes reach, that becomes cell views: one per chosen top module, in
/// `library`, all sharing one bundle of the files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAImportFormState {
    pub project_id: ProjectId,
    /// The root file, by the path it takes in the bundle.
    pub file_name: String,
    /// Every file the import holds, root included, by the path it takes in
    /// the bundle: what was picked or dropped, what was read from the root's
    /// folder, and what Locate supplied.
    pub files: std::collections::BTreeMap<String, String>,
    /// The folder the root was read from, where its includes are looked for.
    pub folder: Option<std::path::PathBuf>,
    /// What the root's includes reach, and the top modules it declares.
    pub closure: crate::state::workspace::ImportClosure,
    /// The library the cells go to. Only a writable library is offered.
    pub library: String,
    /// The top modules that get a cell, by name.
    pub chosen: Vec<String>,
    pub error: Option<String>,
    /// The missing file a Locate pick is for, while one is open.
    pub locating: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAFileSelection {
    pub bundle_id: crate::state::ProjectSourceId,
    pub logical_path: String,
}

/// A built-in header open read-only in the Verilog-A editor well.
///
/// It stands in for one file of one bundle: the file selected when it was
/// opened. A different bundle in front of the reader, or a different file
/// selected by anything, and the well shows that file again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerilogAHeaderView {
    pub(crate) bundle_id: crate::state::ProjectSourceId,
    pub(crate) over: String,
    /// The header's canonical identity, `@rspice/stdlib/disciplines.vams`.
    pub(crate) identity: &'static str,
}

#[derive(Debug, Clone, Default)]
pub struct CodeWorkspaceRuntimeState {
    /// Whether the source-document lifecycle dialog is showing. The
    /// transactions below stay independent of it: a file operation opened from
    /// the navigator brings the dialog up around its own review section, and
    /// closing the dialog abandons the draft rather than committing it.
    pub source_document_dialog: bool,
    pub source_file_dialog: Option<CodeSourceFileDialogState>,
    pub source_history: Option<CodeSourceHistoryState>,
    pub source_search: Option<CodeSourceSearchState>,
    pub language_tools: Option<super::CodeLanguageToolsState>,
    /// Format, Rename and Replace, each one step of the bundle it changed.
    pub(crate) source_edit_history: super::source_edit_history::SourceEditHistory,
    /// Language index retained against the exact closure it was built from,
    /// so the source navigators can run every frame without re-parsing the
    /// bundle on each one.
    pub(crate) source_index_cache: Option<super::SourceIndexCache>,
    /// Where each open source document's caret and selection were last
    /// reported, keyed by the bundle and the logical path inside it. Two cell
    /// views may each own a `model.va`; keying by language and path made them
    /// overwrite one another's place.
    pub source_carets: std::collections::BTreeMap<(ProjectSourceId, String), usize>,
    pub source_selections: std::collections::BTreeMap<(ProjectSourceId, String), (usize, usize)>,
    pub veriloga: VerilogAWorkbenchState,
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
    pub include_resolutions:
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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_app_state::workbench::app_state::model::compile_project_bundle_receipt;
    // These cover the receipt -> runtime adapter, so they stay with the
    // receipt. What they assert about the runtime itself now comes from
    // the engine layer.
    use rspice_simulation::netlist_preparation::{
        append_project_veriloga_directive, project_veriloga_directive,
    };
    use rspice_simulation::veriloga::PreparedVerilogARuntimeSet;

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
            crate::state::ProjectSourceOwner::cell_view(crate::state::CellViewRef::new(
                "behavioral",
                module_name,
                "veriloga",
            )),
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
            compile_project_bundle_receipt(project_id, &bundle, Some(module_name)).unwrap();
        receipt
            .compilation
            .prepare_runtime_with_alias(project_id, &bundle, alias)
            .unwrap()
    }

    #[test]
    fn an_edited_bundle_keeps_stable_identity_for_receipt_and_runtime() {
        let project_id = crate::product::ProjectId::new();
        let owner = crate::state::ProjectSourceOwner::cell_view(crate::state::CellViewRef::new(
            "behavioral",
            "stable",
            "veriloga",
        ));
        let mut registry = crate::state::ProjectSourceRegistry::try_from_bundles([
            crate::state::ProjectSourceBundle::try_new(
                owner.clone(),
                crate::state::ProjectSourceLanguage::VerilogA,
                "stable.va",
                "module stable(p, n); inout p, n; electrical p, n; endmodule\n",
                [],
                [],
            )
            .unwrap(),
        ])
        .unwrap();
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
        let receipt = compile_project_bundle_receipt(project_id, bundle, None)
            .expect("edited current bundle compiles");

        let runtime = receipt.compilation.prepare_runtime(project_id, bundle);

        assert!(runtime.is_ok());
        assert_eq!(receipt.compilation.token().bundle_id, stable_id);
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
        // payload is private to `rspice_simulation::veriloga` now, and a persisted
        // artifact edited on disk is the case that actually matters.
        // Another base64 letter first: still an artifact's spelling, but not
        // the bytes the digest covers.
        let mut document: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
        let artifact = document["artifact"].as_str().unwrap().to_owned();
        let first = if artifact.starts_with('A') { 'B' } else { 'A' };
        document["artifact"] = serde_json::Value::String(format!("{first}{}", &artifact[1..]));
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
            .device_runtimes()
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
