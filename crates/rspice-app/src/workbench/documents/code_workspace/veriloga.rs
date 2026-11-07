//! Governed in-memory Verilog-A compilation for the Code workspace.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use std::sync::{Arc, Mutex, mpsc};

use rspice_veriloga::{
    RuntimeCompileReport, RuntimeTargetMaturity, RuntimeTargetReadiness, VirtualRuntimeCompilation,
};
use sha2::Digest as _;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    ProjectSourceBundle, ProjectSourceFile, ProjectSourceId, ProjectSourceLanguage,
    ProjectSourceOwner, ProjectSourceRole, ViewType,
};
use crate::workbench::{MessageId, RSpiceApp};
use rspice_app_workbench::workbench::state::WorkbenchSession;

use rspice_app_state::workbench::app_state::model::{
    apply_compiled_interface, diagnostic_capacity_failure, receipt_from_report,
    with_interface_outcome,
};
use rspice_simulation::project_veriloga::VerilogASourceOperationToken;

use super::{
    CodeEditorDiagnostic, CodeEditorSeverity, PendingVerilogACompile, VerilogACompileOutcome,
};
use crate::state::code_diagnostics::CodeDiagnosticCollection;

/// The exact project-owned Verilog-A closure selected by the current editor:
/// the active Verilog-A cell view, the bundle that serves it, which it owns
/// or shares with the views of the file's other modules, and the module that
/// view binds.
#[derive(Debug, Clone)]
pub(crate) struct SelectedVerilogASource {
    pub(super) view: crate::state::CellViewRef,
    pub(super) bundle: ProjectSourceBundle,
    pub(super) selected_module: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct SelectedVerilogAEditorSnapshot {
    selected: SelectedVerilogASource,
    active_path: String,
    selection_was_invalid: bool,
}

impl SelectedVerilogAEditorSnapshot {
    pub(crate) fn bundle_id(&self) -> ProjectSourceId {
        self.selected.bundle().id()
    }

    pub(crate) fn active_path(&self) -> &str {
        &self.active_path
    }

    pub(crate) const fn selection_was_invalid(&self) -> bool {
        self.selection_was_invalid
    }
}

impl SelectedVerilogASource {
    /// The cell view the source is selected for: its module's view, which
    /// for a shared source need not be the bundle's owner.
    pub(crate) fn view(&self) -> &crate::state::CellViewRef {
        &self.view
    }

    pub(crate) fn bundle(&self) -> &ProjectSourceBundle {
        &self.bundle
    }

    pub(crate) fn document(&self) -> &crate::state::ProjectSourceDocument {
        self.bundle.root()
    }

    pub(crate) fn selected_module(&self) -> Option<&str> {
        self.selected_module.as_deref()
    }

    pub(crate) fn token(
        &self,
        project_id: crate::product::ProjectId,
    ) -> VerilogASourceOperationToken {
        VerilogASourceOperationToken::capture(project_id, self.bundle(), self.selected_module())
    }

    pub(crate) fn matches_token(
        &self,
        project_id: crate::product::ProjectId,
        token: VerilogASourceOperationToken,
    ) -> bool {
        self.token(project_id) == token
    }
}

pub(crate) fn selected_veriloga_source(app: &RSpiceApp) -> Result<SelectedVerilogASource, String> {
    let no_view = || "Open a Verilog-A cell view to work on its source.".to_owned();
    let owner = app
        .state
        .model
        .workspace
        .active_source_owner(ProjectSourceLanguage::VerilogA)
        .ok_or_else(no_view)?;
    let ProjectSourceOwner::CellView { reference } = &owner else {
        return Err(no_view());
    };
    // A view that names no module yet is still its author's to edit: a source
    // migrated before it parsed has no module to name until it does.
    let selected_module = match crate::state::workspace::project_veriloga_binding_for_view(
        &app.state.model.workspace,
        &app.state.model.library_manager,
        reference,
    ) {
        Ok(binding) => Some(binding.selected_module().to_owned()),
        Err(_) if view_names_no_module(app, reference) => None,
        Err(error) => return Err(error),
    };
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .ok_or_else(|| {
            format!(
                "{} has no project-owned Verilog-A source bundle.",
                reference.display_path()
            )
        })?
        .clone();
    bundle
        .validate()
        .map_err(|error| format!("The selected Verilog-A source bundle is invalid: {error}"))?;
    Ok(SelectedVerilogASource {
        view: reference.clone(),
        bundle,
        selected_module,
    })
}

/// Whether the Verilog-A view at `reference` exists and states no module.
fn view_names_no_module(app: &RSpiceApp, reference: &crate::state::CellViewRef) -> bool {
    app.state
        .model
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| Some((cell, cell.get_view(&reference.view)?)))
        .is_some_and(|(cell, view)| {
            view.view_type == ViewType::VerilogA
                && crate::state::workspace::metadata_value(
                    [&view.metadata, &cell.metadata],
                    &["veriloga.module", "netlist.module"],
                )
                .is_none()
        })
}

pub(crate) fn selected_veriloga_editor_snapshot(
    app: &RSpiceApp,
) -> Result<SelectedVerilogAEditorSnapshot, String> {
    let selected = selected_veriloga_source(app)?;
    let explicit = app
        .state
        .code
        .code_workspace
        .veriloga
        .selected_file
        .as_ref();
    let selection_was_invalid = explicit.is_some_and(|selection| {
        selection.bundle_id != selected.bundle().id()
            || !selected.bundle().contains_file(&selection.logical_path)
    });
    let active_path = explicit
        .filter(|selection| {
            selection.bundle_id == selected.bundle().id()
                && selected.bundle().contains_file(&selection.logical_path)
        })
        .map(|selection| selection.logical_path.clone())
        .unwrap_or_else(|| selected.document().logical_path().to_owned());
    Ok(SelectedVerilogAEditorSnapshot {
        selected,
        active_path,
        selection_was_invalid,
    })
}

mod import;

#[cfg(test)]
use import::add_bundle_file;
pub use import::poll_veriloga_import;
pub(crate) use import::{
    PickedSource, close_veriloga_import_form, commit_veriloga_source_replacement,
    import_dropped_veriloga_sources, request_veriloga_add_files, request_veriloga_import_locate,
    request_veriloga_source_replacement, writable_libraries,
};
#[cfg(test)]
pub(crate) use import::{complete_veriloga_file_pick, complete_veriloga_files_pick};

pub(crate) fn active_veriloga_file_path(
    app: &RSpiceApp,
    selected: &SelectedVerilogASource,
) -> String {
    app.state
        .code
        .code_workspace
        .veriloga
        .selected_file
        .as_ref()
        .filter(|selection| selection.bundle_id == selected.bundle().id())
        .and_then(|selection| {
            selected
                .bundle()
                .contains_file(&selection.logical_path)
                .then(|| selection.logical_path.clone())
        })
        .unwrap_or_else(|| selected.document().logical_path().to_owned())
}

/// The built-in header the editor well shows in place of `active_path` of
/// `bundle_id`, by its canonical identity and exact text, when one is open.
pub(crate) fn viewed_veriloga_header(
    state: &super::page::VerilogAWorkbenchState,
    bundle_id: ProjectSourceId,
    active_path: &str,
) -> Option<(&'static str, &'static str)> {
    state
        .viewed_header
        .as_ref()
        .filter(|header| {
            header.bundle_id == bundle_id
                && crate::state::project_source_paths_equal(&header.over, active_path)
        })
        .and_then(|header| super::builtin_header(header.identity))
}

/// Record that `bundle_id` changed: compile evidence for it is void, the
/// project has unsaved source, and the cell view that owns the bundle — whose
/// document the bundle is part of — says so on its tab.
///
/// A save clears the tab the way it clears a schematic's: the lifecycle
/// registry fingerprints a cell view's bundle as part of that view's document,
/// so publishing the bundle publishes the view and the registry marks it
/// clean.
pub(crate) fn invalidate_veriloga_evidence(app: &mut RSpiceApp, bundle_id: ProjectSourceId) {
    mark_bundle_unsaved(app, bundle_id);
    app.state.code.code_workspace.veriloga.void_evidence();
}

/// The persisted bundle changed: the project has unsaved source, and so does
/// each cell view the bundle serves, whose tab says so.
fn mark_bundle_unsaved(app: &mut RSpiceApp, bundle_id: ProjectSourceId) {
    let content = &mut app.state.model.workspace.content;
    content.project_sources_dirty = true;
    let views = content
        .project_sources
        .get_bundle(bundle_id)
        .map(|bundle| bundle.cell_views().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    for open in content
        .open_views
        .iter_mut()
        .filter(|open| views.contains(&open.reference))
    {
        open.dirty = true;
    }
}

/// Replace one file's exact UTF-8 bytes only when the same bundle identity is
/// still active. The registry performs the edit transaction on a clone,
/// increments the bundle revision, invalidates validation evidence, and
/// commits only after the complete closure remains valid.
pub(crate) fn replace_selected_veriloga_file(
    app: &mut RSpiceApp,
    expected: &SelectedVerilogASource,
    logical_path: &str,
    contents: String,
) -> Result<bool, String> {
    let current = selected_veriloga_source(app)?;
    let project_id = app.state.model.workspace.content.project.id();
    if !current.matches_token(project_id, expected.token(project_id)) {
        return Err(
            "The active Verilog-A source changed before this edit could be committed.".to_owned(),
        );
    }
    let changed = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(current.bundle().id(), logical_path, contents)
        .map_err(|error| error.to_string())?;
    if changed {
        invalidate_veriloga_evidence(app, current.bundle().id());
    }
    Ok(changed)
}

/// Ask for a compile of the source in front of the reader, as it stands now.
///
/// Compile asks nothing: the closure and its build profile are taken here, at
/// the click, and the frame loop starts exactly that on its next pass. A
/// source still on the compiler's built-in defaults has them written into the
/// bundle as its build profile first, so what compiles is always a profile
/// the project holds.
pub(crate) fn request_veriloga_compile(app: &mut RSpiceApp) -> Result<(), String> {
    if app.state.code.code_workspace.veriloga.pending.is_some() {
        return Err("A Verilog-A compile is already running.".to_owned());
    }
    let mut selected = selected_veriloga_source(app)?;
    if selected.bundle().qualifications().len()
        >= crate::state::MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS
    {
        return Err(
            "This source bundle has reached its limit of saved qualification records. Archive the project as a new revision before compiling again."
                .to_owned(),
        );
    }
    let mut resolved = super::veriloga_profile::resolve_veriloga_build_profile(selected.bundle())?;
    if resolved.legacy_builtin {
        persist_legacy_build_profile(app, &selected, &resolved.profile)?;
        selected = selected_veriloga_source(app)?;
        resolved = super::veriloga_profile::resolve_veriloga_build_profile(selected.bundle())?;
    }
    let entry_modules = &resolved.profile.entry_modules;
    if let Some(module) = selected.selected_module()
        && !entry_modules.is_empty()
        && !entry_modules.iter().any(|entry| entry == module)
    {
        return Err(format!(
            "Selected module '{module}' is not declared by the Verilog-A build profile."
        ));
    }
    app.state.code.code_workspace.veriloga.compile_request = Some(selected);
    Ok(())
}

/// Write `profile` as the build profile of the bundle `expected` names, while
/// that bundle is still exactly what it was: over the profile document it
/// holds, as any edit of a source file is made, or as a new profile document
/// when it holds none yet. Returns whether anything changed.
pub(crate) fn write_veriloga_build_profile(
    app: &mut RSpiceApp,
    expected: &SelectedVerilogASource,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> Result<bool, String> {
    let current = super::veriloga_profile::resolve_veriloga_build_profile(expected.bundle())?;
    match current.logical_path {
        Some(path) => replace_selected_veriloga_file(app, expected, &path, profile.to_toml()?),
        None => {
            let project_id = app.state.model.workspace.content.project.id();
            if !selected_veriloga_source(app)?.matches_token(project_id, expected.token(project_id))
            {
                return Err(
                    "The active Verilog-A source changed before its build profile could be written."
                        .to_owned(),
                );
            }
            add_build_profile_document(app, expected, profile).map(|_| true)
        }
    }
}

fn persist_legacy_build_profile(
    app: &mut RSpiceApp,
    selected: &SelectedVerilogASource,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> Result<(), String> {
    let path = add_build_profile_document(app, selected, profile)?;
    app.state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!("Saved the Verilog-A compiler defaults as project build profile {path}."),
    );
    Ok(())
}

/// Add `profile` to `selected`'s bundle as its build-profile document, at the
/// first free path under `.rspice/`. Returns the path it was written at.
fn add_build_profile_document(
    app: &mut RSpiceApp,
    selected: &SelectedVerilogASource,
    profile: &super::veriloga_profile::VerilogABuildProfile,
) -> Result<String, String> {
    let base = ".rspice/veriloga-build";
    let path = (1_u32..=10_000)
        .map(|index| {
            if index == 1 {
                format!("{base}.toml")
            } else {
                format!("{base}-{index}.toml")
            }
        })
        .find(|path| !selected.bundle().contains_file(path))
        .ok_or_else(|| "Could not allocate a portable Verilog-A build-profile path.".to_owned())?;
    let file =
        ProjectSourceFile::try_new(&path, profile.to_toml()?).map_err(|error| error.to_string())?;
    app.state
        .model
        .workspace
        .content
        .add_project_source_bundle_file_with_role(
            selected.bundle().id(),
            selected.bundle().root().logical_path(),
            file,
            ProjectSourceRole::VerilogABuildProfile,
        )
        .map_err(|error| error.to_string())?;
    invalidate_veriloga_evidence(app, selected.bundle().id());
    Ok(path)
}

/// Compile `selected`, the closure a Compile request took at its click.
/// Native builds use an operating-system thread; browser builds dispatch the
/// same sealed bundle to a dedicated module worker so compilation never stalls
/// egui's event/rendering thread.
fn start_veriloga_compile(
    app: &mut RSpiceApp,
    selected: SelectedVerilogASource,
    repaint: &egui::Context,
) {
    if app.state.code.code_workspace.veriloga.pending.is_some() {
        return;
    }
    let token = selected.token(app.state.model.workspace.content.project.id());
    // A design may instantiate the SPICE cells of the netlist its project's
    // run would build, and keeps the min:typ:max values it selects: a manual
    // deck's as its run expands it, else the configured design's. While that
    // netlist cannot be built the design is compiled against no cells and
    // under the typical values, and a run compiles it again against and under
    // its own. A name nothing in the project defines stays undefined.
    let deck = {
        use crate::simulation::controller::{SimulationController, prepared_run};
        let state = &app.state;
        match state.model.simulation.execution.run_intent {
            crate::state::SimulationRunIntent::ManualDeck => {
                prepared_run::manual_source_inputs(state)
                    .and_then(|manual| {
                        rspice_simulation::execution::preparation::manual_deck_hdl(
                            &prepared_run::run_preparation_inputs(state),
                            manual,
                        )
                    })
                    .ok()
            }
            crate::state::SimulationRunIntent::SimulateRunSet => {
                SimulationController::prepare_design_netlist_for_inspection(&state.model)
                    .ok()
                    .map(|netlist| {
                        use rspice_simulation::project_veriloga::preparation::{
                            deck_delay_select, deck_external_cells,
                        };
                        (deck_external_cells(&netlist), deck_delay_select(&netlist))
                    })
            }
        }
    };
    let delay_select = deck
        .as_ref()
        .map_or_else(Default::default, |(_, delay_select)| *delay_select);
    let cells = deck.map(|(cells, _)| Arc::<[_]>::from(cells));
    let (sender, receiver) = mpsc::channel();
    Arc::make_mut(&mut app.state.code.code_workspace.veriloga.last_failure).clear();
    app.state.code.code_workspace.veriloga.last_failure_token = None;
    app.state.code.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
        token,
        receiver: Arc::new(Mutex::new(receiver)),
        cells: cells.clone(),
        delay_select,
    });

    let wake = repaint.clone();
    if let Err(error) = rspice_simulation::project_veriloga::compile_service::start_compile(
        selected.bundle,
        selected.selected_module,
        cells,
        delay_select,
        sender.clone(),
        move || wake.request_repaint(),
    ) {
        let _ = sender.send(
            rspice_simulation::project_veriloga::compile_service::transport_failure_outcome(error),
        );
        repaint.request_repaint();
    }
}

/// Cancel transient compile work before source/module identity is replaced.
/// Native work is detached exactly as before; browser work is also terminated
/// so abandoned compilation cannot consume CPU or block the next request.
pub(crate) fn cancel_veriloga_compile(app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    rspice_simulation::project_veriloga::compile_service::cancel_compile();
    app.state.code.code_workspace.veriloga.pending = None;
}

/// Start the compile a Compile command asked for.
///
/// The command took the closure at the click; the frame loop starts it here,
/// before any surface draws, with the context the worker wakes when it
/// answers. Its result lands wherever the reader is by then.
pub fn consume_veriloga_compile_request(app: &mut RSpiceApp, repaint: &egui::Context) {
    if let Some(selected) = app
        .state
        .code
        .code_workspace
        .veriloga
        .compile_request
        .take()
    {
        start_veriloga_compile(app, selected, repaint);
    }
}

/// Poll a pending compile and publish it only when its revision and digest
/// still identify the exact visible project source.
///
/// The frame loop runs this every frame, so a result is published whether or
/// not an editor host is on screen when it lands.
pub fn poll_veriloga_compile(app: &mut RSpiceApp) {
    let Some(pending) = app.state.code.code_workspace.veriloga.pending.clone() else {
        return;
    };
    let received = pending
        .receiver
        .lock()
        .ok()
        .and_then(|receiver| match receiver.try_recv() {
            Ok(outcome) => Some(Ok(outcome)),
            Err(mpsc::TryRecvError::Disconnected) => Some(Err(())),
            Err(mpsc::TryRecvError::Empty) => None,
        });
    let Some(received) = received else {
        return;
    };
    cancel_veriloga_compile(app);

    let is_current = selected_veriloga_source(app).is_ok_and(|selected| {
        selected.matches_token(
            app.state.model.workspace.content.project.id(),
            pending.token,
        )
    });
    if !is_current {
        // The editor exposes one active compile receipt. Once an in-flight
        // result is rejected because its source/module contract changed, an
        // older receipt must not remain as apparently current evidence.
        app.state.code.code_workspace.veriloga.receipt = None;
        return;
    }

    match received {
        Ok(VerilogACompileOutcome::Success(report)) => {
            let receipt_bundle = app
                .state
                .model
                .workspace
                .content
                .project_sources
                .get_bundle(pending.token.bundle_id)
                .cloned();
            let receipt = match receipt_from_report(
                pending.token,
                &report,
                receipt_bundle.as_ref(),
                pending.cells.as_deref(),
                pending.delay_select,
            ) {
                Ok(receipt) => receipt,
                Err(error) => {
                    let diagnostic = diagnostic_capacity_failure(pending.token, error);
                    let diagnostics = [diagnostic.clone()];
                    let _ = record_veriloga_qualification(app, pending.token, None, &diagnostics);
                    app.state.code.code_workspace.veriloga.receipt = None;
                    app.state.code.code_workspace.veriloga.last_failure = Arc::new(
                        CodeDiagnosticCollection::try_new(vec![diagnostic]).unwrap_or_default(),
                    );
                    app.state.code.code_workspace.veriloga.last_failure_token = Some(pending.token);
                    app.state.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        "Verilog-A compilation produced more diagnostics than can be kept; the source was not validated.",
                    );
                    announce_compile_failed(
                        &mut app.state.workbench,
                        &app.state.code.code_workspace.veriloga.last_failure,
                    );
                    return;
                }
            };
            if let Err(error) = record_veriloga_qualification(
                app,
                pending.token,
                Some(&report),
                receipt.diagnostics.as_slice(),
            ) {
                app.state.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!(
                        "The compiler succeeded, but its qualification record could not be saved: {error}"
                    ),
                );
                app.state.code.code_workspace.veriloga.receipt = None;
                return;
            }
            let previous_identity = app
                .state
                .model
                .workspace
                .content
                .project_sources
                .get_bundle(pending.token.bundle_id)
                .and_then(ProjectSourceBundle::validated_identity);
            match app
                .state
                .model
                .workspace
                .content
                .project_sources
                .mark_bundle_validated(pending.token.bundle_id)
            {
                Ok(_) => {
                    if previous_identity
                        != app
                            .state
                            .model
                            .workspace
                            .content
                            .project_sources
                            .get_bundle(pending.token.bundle_id)
                            .and_then(ProjectSourceBundle::validated_identity)
                    {
                        app.state.model.workspace.content.project_sources_dirty = true;
                    }
                    // The ports become the interface of the view that selects
                    // the module: for a shared source, that module's cell.
                    let selecting = receipt_bundle.as_ref().and_then(|bundle| {
                        super::veriloga_interface::view_selecting_module(
                            &app.state.model.library_manager,
                            bundle,
                            report.abi.module_name.as_ref(),
                        )
                        .cloned()
                    });
                    let receipt = match &selecting {
                        Some(reference) => {
                            let applied = apply_compiled_interface(
                                &mut app.state.model.library_manager,
                                &app.state.model.workspace.content.schematic_buffers,
                                reference,
                                &report.abi,
                            );
                            if applied.wrote {
                                app.state.model.workspace.content.project_metadata_dirty = true;
                            }
                            if let crate::state::VerilogAInterfaceOutcome::Refused(reason) =
                                &applied.outcome
                            {
                                app.state.record(
                                    LogSeverity::Warning,
                                    LogSource::Studio,
                                    format!(
                                        "The module compiled, but its ports did not become the cell's interface: {reason}."
                                    ),
                                );
                            }
                            let receipt =
                                with_interface_outcome(receipt, reference, applied.outcome);
                            super::veriloga_interface::offer_placement(
                                &mut app.state.code.code_workspace.veriloga,
                                receipt,
                                reference,
                                report.abi.formal_ports.len(),
                            )
                        }
                        None => receipt,
                    };
                    Arc::make_mut(&mut app.state.code.code_workspace.veriloga.last_failure).clear();
                    app.state.code.code_workspace.veriloga.last_failure_token = None;
                    let summary = receipt.diagnostics.summary();
                    app.state.code.code_workspace.veriloga.receipt = Some(receipt);
                    app.state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        format!(
                            "Compiled Verilog-A module {} for every qualified runtime target.",
                            report.abi.module_name
                        ),
                    );
                    announce_compiled(
                        &mut app.state.workbench,
                        &report.abi,
                        summary.total().saturating_sub(summary.errors),
                    );
                }
                Err(error) => {
                    app.state.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!(
                            "The compiler succeeded, but the source could not be marked validated: {error}"
                        ),
                    );
                }
            }
        }
        Ok(VerilogACompileOutcome::Failure(diagnostics)) => {
            if let Err(error) =
                record_veriloga_qualification(app, pending.token, None, &diagnostics)
            {
                app.state.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!(
                        "The failed Verilog-A compile could not be added to the qualification record: {error}"
                    ),
                );
            }
            app.state.code.code_workspace.veriloga.receipt = None;
            let diagnostics = diagnostics
                .into_iter()
                .map(|diagnostic| {
                    let document_id = diagnostic
                        .source_path
                        .clone()
                        .unwrap_or_else(|| pending.token.bundle_id.to_string().into());
                    diagnostic.bind_validation(
                        document_id,
                        pending.token.revision,
                        pending.token.closure_digest.to_string(),
                    )
                })
                .collect();
            app.state.code.code_workspace.veriloga.last_failure = Arc::new(
                CodeDiagnosticCollection::try_new(diagnostics).unwrap_or_else(|error| {
                    CodeDiagnosticCollection::try_new(vec![diagnostic_capacity_failure(
                        pending.token,
                        error,
                    )])
                    .unwrap_or_default()
                }),
            );
            app.state.code.code_workspace.veriloga.last_failure_token = Some(pending.token);
            app.state.record(
                LogSeverity::Error,
                LogSource::Studio,
                "Verilog-A compilation failed. Review the source diagnostics.",
            );
            announce_compile_failed(
                &mut app.state.workbench,
                &app.state.code.code_workspace.veriloga.last_failure,
            );
        }
        Err(()) => {
            app.state.code.code_workspace.veriloga.receipt = None;
            app.state.code.code_workspace.veriloga.last_failure = Arc::new(
                CodeDiagnosticCollection::try_new(vec![
                    CodeEditorDiagnostic::current(
                        "rspice.veriloga.compiler-worker",
                        "VA-WORKER-STOPPED",
                        CodeEditorSeverity::Error,
                        "Compiler worker stopped unexpectedly",
                        "No compiler report was published; the source was not validated.",
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                    .bind_validation(
                        pending.token.bundle_id.to_string(),
                        pending.token.revision,
                        pending.token.closure_digest.to_string(),
                    ),
                ])
                .unwrap_or_default(),
            );
            let worker_diagnostics = app.state.code.code_workspace.veriloga.last_failure.clone();
            if let Err(error) = record_veriloga_qualification(
                app,
                pending.token,
                None,
                worker_diagnostics.as_slice(),
            ) {
                app.state.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!(
                        "The stopped compiler worker could not be added to the qualification record: {error}"
                    ),
                );
            }
            app.state.code.code_workspace.veriloga.last_failure_token = Some(pending.token);
            app.state.record(
                LogSeverity::Error,
                LogSource::Studio,
                "The Verilog-A compiler worker stopped before publishing a report.",
            );
            announce_compile_failed(
                &mut app.state.workbench,
                &app.state.code.code_workspace.veriloga.last_failure,
            );
        }
    }
}

/// Say on the status bar what a passing compile made, where a screen reader
/// hears it too: "Compiled opamp_bhv: 5 ports, 5 parameters".
fn announce_compiled(
    workbench: &mut WorkbenchSession,
    abi: &rspice_veriloga::RuntimeAbiSummary,
    advisories: usize,
) {
    let messages = workbench.messages();
    let counted = |count: usize, one: MessageId, many: MessageId| {
        messages.format(
            if count == 1 { one } else { many },
            &[("count", &count.to_string())],
        )
    };
    let ports = counted(
        abi.formal_ports.len(),
        MessageId::VerilogAPortCountSingular,
        MessageId::VerilogAPortCount,
    );
    let parameters = counted(
        abi.parameters.len(),
        MessageId::VerilogAParameterCountSingular,
        MessageId::VerilogAParameterCount,
    );
    let mut arguments = vec![
        ("module", abi.module_name.to_string()),
        ("ports", ports),
        ("parameters", parameters),
    ];
    let template = if advisories == 0 {
        MessageId::VerilogACompiledAnnouncement
    } else {
        arguments.push((
            "advisories",
            counted(
                advisories,
                MessageId::VerilogAAdvisorySingular,
                MessageId::VerilogAAdvisories,
            ),
        ));
        MessageId::VerilogACompiledWithAdvisoriesAnnouncement
    };
    let arguments = arguments
        .iter()
        .map(|(name, value)| (*name, value.as_str()))
        .collect::<Vec<_>>();
    let announcement = messages.format(template, &arguments);
    workbench.prompt(announcement);
}

/// Say on the status bar that the compile was refused, and how many errors
/// refused it: "Compile failed: 2 errors".
fn announce_compile_failed(workbench: &mut WorkbenchSession, failure: &CodeDiagnosticCollection) {
    let messages = workbench.messages();
    let errors = failure.summary().errors.max(1);
    let errors = messages.format(
        if errors == 1 {
            MessageId::VerilogAErrorSingular
        } else {
            MessageId::VerilogAErrors
        },
        &[("count", &errors.to_string())],
    );
    let announcement = messages.format(
        MessageId::VerilogACompileFailedAnnouncement,
        &[("errors", &errors)],
    );
    workbench.prompt(announcement);
}

fn record_veriloga_qualification(
    app: &mut RSpiceApp,
    token: VerilogASourceOperationToken,
    report: Option<&RuntimeCompileReport>,
    diagnostics: &[CodeEditorDiagnostic],
) -> Result<u64, String> {
    let selected = selected_veriloga_source(app)?;
    if !selected.matches_token(app.state.model.workspace.content.project.id(), token) {
        return Err("qualification result no longer matches the current source bundle".to_owned());
    }
    let resolved = super::veriloga_profile::resolve_veriloga_build_profile(selected.bundle())?;
    let selected_module = report
        .map(|report| report.abi.module_name.to_string())
        .or_else(|| selected.selected_module().map(str::to_owned))
        .or_else(|| resolved.profile.entry_modules.first().cloned())
        .unwrap_or_else(|| {
            super::veriloga_profile::module_name_from_path(selected.bundle().root().logical_path())
        });
    let report_bytes = match report {
        Some(report) => serde_json::to_vec(report),
        None => serde_json::to_vec(diagnostics),
    }
    .map_err(|error| format!("could not encode the qualification report: {error}"))?;
    let report_digest =
        crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(&report_bytes).into());
    let targets = report
        .map(|report| {
            report
                .targets
                .all()
                .iter()
                .map(|target| crate::state::ProjectSourceQualificationTarget {
                    name: target.target.label().to_owned(),
                    readiness: runtime_readiness_label(target.readiness).to_owned(),
                    maturity: runtime_maturity_label(target.maturity).to_owned(),
                    detail: target.detail.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    let checks = report
        .map(|report| {
            report
                .specialist
                .checks
                .iter()
                .map(|check| crate::state::ProjectSourceQualificationCheck {
                    name: specialist_check_label(check.kind).to_owned(),
                    disposition: specialist_disposition_label(check.disposition).to_owned(),
                    findings: check.findings,
                    detail: check.detail.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    let error_count = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == CodeEditorSeverity::Error)
        .count();
    let warning_count = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == CodeEditorSeverity::Warning)
        .count();
    let recorded_at_unix_ms = crate::time_compat::unix_epoch()
        .as_millis()
        .try_into()
        .map_err(|_| "qualification timestamp exceeds the persisted range".to_owned())?;
    let record = crate::state::ProjectSourceQualificationAttempt {
        attempt_id: uuid::Uuid::new_v4(),
        recorded_at_unix_ms,
        source_revision: token.revision,
        source_closure_digest: token.closure_digest,
        profile_digest: resolved.digest,
        package_name: resolved.profile.package.name,
        package_version: resolved.profile.package.version,
        selected_module,
        compiler_version: env!("CARGO_PKG_VERSION").to_owned(),
        disposition: if report.is_some() {
            crate::state::ProjectSourceQualificationDisposition::Passed
        } else {
            crate::state::ProjectSourceQualificationDisposition::Failed
        },
        report_digest,
        targets,
        checks,
        error_count,
        warning_count,
    };
    let sequence = app
        .state
        .model
        .workspace
        .content
        .append_project_source_qualification(token.bundle_id, record)
        .map_err(|error| error.to_string())?;
    // The qualification history is persisted with the bundle, so recording
    // an attempt is a change to the owning cell view's document.
    mark_bundle_unsaved(app, token.bundle_id);
    Ok(sequence)
}

const fn runtime_readiness_label(readiness: RuntimeTargetReadiness) -> &'static str {
    match readiness {
        RuntimeTargetReadiness::Available => "available",
        RuntimeTargetReadiness::Unavailable => "unavailable",
        RuntimeTargetReadiness::Rejected => "rejected",
    }
}

const fn runtime_maturity_label(maturity: RuntimeTargetMaturity) -> &'static str {
    match maturity {
        RuntimeTargetMaturity::Production => "production",
        RuntimeTargetMaturity::Preview => "preview",
        RuntimeTargetMaturity::QualificationOnly => "qualification-only",
    }
}

const fn specialist_check_label(check: rspice_veriloga::SpecialistCheckKind) -> &'static str {
    match check {
        rspice_veriloga::SpecialistCheckKind::HiddenState => "hidden-state",
        rspice_veriloga::SpecialistCheckKind::Discontinuity => "discontinuity",
        rspice_veriloga::SpecialistCheckKind::UnitsAndRanges => "units-and-ranges",
        rspice_veriloga::SpecialistCheckKind::Convergence => "convergence",
        rspice_veriloga::SpecialistCheckKind::Portability => "portability",
    }
}

const fn specialist_disposition_label(
    disposition: rspice_veriloga::SpecialistCheckDisposition,
) -> &'static str {
    match disposition {
        rspice_veriloga::SpecialistCheckDisposition::Evaluated => "evaluated",
        rspice_veriloga::SpecialistCheckDisposition::Findings => "findings",
        rspice_veriloga::SpecialistCheckDisposition::EvidenceOnly => "evidence-only",
        rspice_veriloga::SpecialistCheckDisposition::NotEvaluated => "not-evaluated",
    }
}

#[cfg(test)]
fn compile_selected_source(selected: &SelectedVerilogASource) -> VerilogACompileOutcome {
    rspice_simulation::project_veriloga::compile_service::compile_source(
        selected.bundle(),
        selected.selected_module(),
        None,
        Default::default(),
    )
}

pub(crate) fn compile_project_bundle_virtual_for_provenance(
    bundle: &ProjectSourceBundle,
    selected_module: &str,
) -> Result<VirtualRuntimeCompilation, String> {
    rspice_simulation::project_veriloga::compile_project_virtual_runtime(
        bundle,
        selected_module,
        None,
        Default::default(),
    )
    .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, CellViewRef, Library, ProjectSourceDocument, View};
    use rspice_app_state::workbench::app_state::model::compile_project_bundle_receipt;

    #[test]
    fn editor_and_execution_share_the_project_macro_expansion_contract() {
        let limits = rspice_simulation::project_veriloga::project_virtual_compile_limits();
        assert_eq!(
            limits.max_expanded_bytes,
            crate::state::MAX_PROJECT_SOURCE_BUNDLE_BYTES.saturating_mul(2)
        );
    }

    /// A `behavioral/<cell>/veriloga` view binding `module`, owning a
    /// one-file bundle at `root` that holds `source`.
    fn add_module_view(
        app: &mut RSpiceApp,
        cell_name: &str,
        root: &str,
        module: &str,
        source: &str,
    ) -> CellViewRef {
        if app
            .state
            .model
            .library_manager
            .get_library("behavioral")
            .is_none()
        {
            app.state
                .model
                .library_manager
                .add_library(Library::new("behavioral"));
        }
        let reference = CellViewRef::new("behavioral", cell_name, "veriloga");
        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), module.to_owned());
        view.metadata
            .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
        let library = app
            .state
            .model
            .library_manager
            .get_library_mut("behavioral")
            .unwrap();
        let mut cell = Cell::new(cell_name);
        cell.add_view(view);
        library.add_cell(cell);
        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(reference.clone()),
            ProjectSourceLanguage::VerilogA,
            root,
            source,
            [],
            [],
        )
        .unwrap();
        app.state
            .model
            .workspace
            .content
            .project_sources
            .insert_bundle(bundle)
            .unwrap();
        reference
    }

    /// A test app whose open Verilog-A view binds `module` over exactly
    /// `source`.
    fn module_source_with(module: &str, source: &str) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        let reference = add_module_view(&mut app, "legacy", "legacy.va", module, source);
        app.state
            .model
            .workspace
            .open_view(reference, ViewType::VerilogA);
        app
    }

    fn portless_module() -> RSpiceApp {
        module_source_with("legacy", "module legacy; endmodule\n")
    }

    /// A compile of a source still on the compiler's built-in defaults writes
    /// them into the bundle as its build profile, and compiles the closure
    /// holding it: the request is taken after the write, not before.
    #[test]
    fn a_compile_request_writes_legacy_defaults_as_the_build_profile_it_compiles() {
        let mut app = portless_module();

        request_veriloga_compile(&mut app).unwrap();

        let request = app
            .state
            .code
            .code_workspace
            .veriloga
            .compile_request
            .clone()
            .expect("the click took the closure");
        let bundle = app
            .state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(request.bundle().id())
            .unwrap();
        let profile_path = bundle
            .paths_for_role(ProjectSourceRole::VerilogABuildProfile)
            .next()
            .unwrap();
        assert_eq!(profile_path, ".rspice/veriloga-build.toml");
        assert!(!bundle.root().content().contains(profile_path));
        assert!(app.state.model.workspace.content.project_sources_dirty);
        assert_eq!(request.bundle().revision(), bundle.revision());
        assert_eq!(request.bundle().closure_digest(), bundle.closure_digest());
        assert!(request.bundle().contains_file(profile_path));
    }

    /// A build profile written over the one a bundle holds is an ordinary
    /// edit of that document: the revision moves, the compile evidence goes,
    /// and the profile reads back as written.
    #[test]
    fn a_build_profile_is_written_over_the_document_the_bundle_holds() {
        let mut app = portless_module();
        request_veriloga_compile(&mut app).unwrap();
        app.state.code.code_workspace.veriloga.compile_request = None;
        let selected = selected_veriloga_source(&app).unwrap();
        let revision = selected.bundle().revision();
        let mut profile =
            super::super::veriloga_profile::resolve_veriloga_build_profile(selected.bundle())
                .unwrap()
                .profile;
        profile.targets.generated_rust = true;

        assert!(write_veriloga_build_profile(&mut app, &selected, &profile).unwrap());

        let after = selected_veriloga_source(&app).unwrap();
        assert!(after.bundle().revision() > revision);
        assert_eq!(
            after
                .bundle()
                .paths_for_role(ProjectSourceRole::VerilogABuildProfile)
                .count(),
            1,
            "written over, not beside"
        );
        let resolved =
            super::super::veriloga_profile::resolve_veriloga_build_profile(after.bundle()).unwrap();
        assert_eq!(resolved.profile, profile);
        let stale = write_veriloga_build_profile(&mut app, &selected, &profile)
            .expect_err("a write is bound to the closure it was made against");
        assert!(stale.contains("changed"), "{stale}");
    }

    #[test]
    fn qualification_history_is_append_only_without_changing_source_identity() {
        let mut app = portless_module();
        request_veriloga_compile(&mut app).unwrap();
        let selected = selected_veriloga_source(&app).unwrap();
        let token = selected.token(app.state.model.workspace.content.project.id());
        let revision = selected.bundle().revision();
        let closure_digest = selected.bundle().closure_digest();
        let VerilogACompileOutcome::Success(report) = compile_selected_source(&selected) else {
            panic!("legacy source must compile")
        };
        let receipt = receipt_from_report(
            token,
            &report,
            Some(selected.bundle()),
            None,
            Default::default(),
        )
        .unwrap();

        let first = record_veriloga_qualification(
            &mut app,
            token,
            Some(&report),
            receipt.diagnostics.as_slice(),
        )
        .unwrap();
        let second = record_veriloga_qualification(
            &mut app,
            token,
            Some(&report),
            receipt.diagnostics.as_slice(),
        )
        .unwrap();

        let bundle = app
            .state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(token.bundle_id)
            .unwrap();
        assert_eq!((first, second), (1, 2));
        assert_eq!(bundle.qualifications().len(), 2);
        assert_eq!(bundle.revision(), revision);
        assert_eq!(bundle.closure_digest(), closure_digest);
        bundle.validate().unwrap();
    }

    fn add_cell_bundle(app: &mut RSpiceApp, cell_name: &str) -> CellViewRef {
        add_module_view(
            app,
            cell_name,
            "shared.va",
            "shared_model",
            "module shared_model(p, n); inout p, n; electrical p, n; endmodule\n",
        )
    }

    /// Every Verilog-A source is a cell view's. A project-wide bundle still in
    /// a registry is never the source in front of the reader: with no
    /// Verilog-A view open there is none, and with one open it is that view's.
    #[test]
    fn only_the_open_veriloga_view_selects_a_source() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .model
            .workspace
            .content
            .project_sources
            .insert(
                ProjectSourceDocument::try_new(
                    "legacy.va",
                    ProjectSourceLanguage::VerilogA,
                    "module legacy; endmodule\n",
                )
                .unwrap(),
            )
            .unwrap();

        let error =
            selected_veriloga_source(&app).expect_err("a project-wide bundle is not a selection");
        assert!(error.contains("Open a Verilog-A cell view"), "{error}");

        let reference = add_cell_bundle(&mut app, "owned");
        app.state
            .model
            .workspace
            .open_view(reference.clone(), ViewType::VerilogA);
        let selected = selected_veriloga_source(&app).unwrap();
        assert_eq!(
            selected.bundle().owner(),
            &ProjectSourceOwner::cell_view(reference)
        );
        assert_eq!(selected.selected_module(), Some("shared_model"));
    }

    /// A file dropped while a Verilog-A view is open is a file to import: it
    /// goes to the import form and the open view's source is left as it was.
    #[test]
    fn a_dropped_source_goes_to_the_import_form_and_never_replaces_the_open_view() {
        let mut app = portless_module();
        let sources_before = app.state.model.workspace.content.project_sources.clone();
        let source = "module dropped(p, n); inout p, n; electrical p, n; endmodule\n";

        import_dropped_veriloga_sources(
            &app.state.model,
            &mut app.state.code.code_workspace.veriloga,
            vec![PickedSource {
                name: "dropped.va".to_owned(),
                contents: source.to_owned(),
                folder: None,
            }],
        )
        .expect("drop import");

        assert_eq!(
            app.state.model.workspace.content.project_sources,
            sources_before
        );
        let form = app
            .state
            .code
            .code_workspace
            .veriloga
            .import_form
            .as_ref()
            .expect("the drop opened the import form");
        assert_eq!(form.file_name, "dropped.va");
        assert!(
            app.state
                .code
                .code_workspace
                .veriloga
                .replace_review
                .is_none()
        );
    }

    /// "Replace source from file…" replaces nothing when the file arrives: it
    /// opens the review, and only committing that review rewrites the file,
    /// whose previous text stays in the revision history.
    #[test]
    fn a_source_replacement_waits_for_its_confirmation() {
        let mut app = portless_module();
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let root = selected.document().logical_path().to_owned();
        let original = selected.document().content().to_owned();
        let replacement = "module legacy(p, n); inout p, n; electrical p, n; endmodule\n";

        request_veriloga_source_replacement(&mut app, &root).unwrap();
        assert!(app.state.code.code_workspace.veriloga.import_requested);
        complete_veriloga_file_pick(&mut app, "next.va", replacement);

        let review = app
            .state
            .code
            .code_workspace
            .veriloga
            .replace_review
            .clone()
            .expect("the picked file waits for confirmation");
        assert_eq!(review.target.logical_path, root);
        assert!(review.restorable);
        let content = |app: &RSpiceApp| {
            app.state
                .model
                .workspace
                .content
                .project_sources
                .get_bundle(bundle_id)
                .unwrap()
                .root()
                .content()
                .to_owned()
        };
        assert_eq!(
            content(&app),
            original,
            "nothing is replaced before confirming"
        );
        assert!(app.state.code.code_workspace.veriloga.import_form.is_none());

        commit_veriloga_source_replacement(&mut app).unwrap();

        assert_eq!(content(&app), replacement);
        let bundle = app
            .state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(
            bundle
                .revision_history()
                .last()
                .and_then(|snapshot| snapshot.file_content(&root)),
            Some(original.as_str()),
            "the replaced text stays restorable"
        );
    }

    /// With no Verilog-A view in front, an imported file is read into the
    /// import form, which names its modules; nothing is created until the form
    /// is committed, and there is no project-wide bundle to fill any more.
    #[test]
    fn a_root_import_with_no_veriloga_view_reads_the_file_into_the_import_form() {
        let mut app = RSpiceApp::test_instance();
        let sources_before = app.state.model.workspace.content.project_sources.clone();
        let source = "module actual_model(p, n); inout p, n; electrical p, n; endmodule\n";

        app.state.code.code_workspace.veriloga.import_requested = true;
        complete_veriloga_file_pick(&mut app, "behavioral/presentation_name.va", source);

        let form = app
            .state
            .code
            .code_workspace
            .veriloga
            .import_form
            .as_ref()
            .expect("the file is read into the import form");
        assert_eq!(form.file_name, "behavioral/presentation_name.va");
        assert_eq!(
            form.closure.modules,
            Ok(vec![crate::state::workspace::DeclaredModule {
                name: "actual_model".to_owned(),
                ports: vec!["p".to_owned(), "n".to_owned()],
            }])
        );
        assert_eq!(
            app.state.model.workspace.content.project_sources,
            sources_before
        );
        assert!(!app.state.code.code_workspace.veriloga.import_requested);
    }

    #[test]
    fn a_source_replacement_rejects_a_bundle_changed_while_the_picker_is_open() {
        let mut app = portless_module();
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let root = selected.document().logical_path().to_owned();

        request_veriloga_source_replacement(&mut app, &root).unwrap();
        app.state
            .model
            .workspace
            .content
            .replace_project_source_bundle_file(
                bundle_id,
                &root,
                "module retained_after_picker_open; endmodule\n".to_owned(),
            )
            .unwrap();
        complete_veriloga_file_pick(
            &mut app,
            "stale_picker_result.va",
            "module stale_picker_result; endmodule\n",
        );

        let bundle = app
            .state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        assert_eq!(
            bundle.root().content(),
            "module retained_after_picker_open; endmodule\n"
        );
        assert!(
            app.state
                .code
                .code_workspace
                .veriloga
                .replace_target
                .is_none()
        );
        assert!(
            app.state
                .code
                .code_workspace
                .veriloga
                .replace_review
                .is_none(),
            "a stale replacement is discarded, not offered"
        );
    }

    #[test]
    fn root_edit_invalidates_bundle_validation_and_compile_evidence() {
        let mut app = RSpiceApp::test_instance();
        let reference = add_cell_bundle(&mut app, "editable");
        app.state
            .model
            .workspace
            .open_view(reference, ViewType::VerilogA);
        let selected = selected_veriloga_source(&app).unwrap();
        app.state
            .model
            .workspace
            .content
            .project_sources
            .mark_bundle_validated(selected.bundle().id())
            .unwrap();
        let receipt = compile_project_bundle_receipt(
            app.state.model.workspace.content.project.id(),
            selected.bundle(),
            selected.selected_module(),
        )
        .unwrap();
        app.state.code.code_workspace.veriloga.receipt = Some(receipt);
        let before = selected.token(app.state.model.workspace.content.project.id());

        assert!(
            replace_selected_veriloga_file(
                &mut app,
                &selected,
                "shared.va",
                "module shared_model(p, n); inout p, n; electrical p, n; real changed; endmodule\n"
                    .to_owned(),
            )
            .unwrap()
        );

        let edited = selected_veriloga_source(&app).unwrap();
        let after = edited.token(app.state.model.workspace.content.project.id());
        assert_eq!(after.bundle_id, before.bundle_id);
        assert!(after.revision > before.revision);
        assert_ne!(after.closure_digest, before.closure_digest);
        assert!(!edited.bundle().validation_is_current());
        assert!(app.state.code.code_workspace.veriloga.receipt.is_none());
        assert!(app.state.model.workspace.content.project_sources_dirty);
    }

    #[test]
    fn a_multifile_bundle_compiles_with_its_views_module() {
        let mut app = module_source_with(
            "legacy",
            "module legacy(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
        );
        let selected = selected_veriloga_source(&app).unwrap();
        let bundle_id = selected.bundle().id();
        let root_path = selected.document().logical_path().to_owned();
        add_bundle_file(
            &mut app,
            bundle_id,
            &root_path,
            "support/constants.vams".to_owned(),
            "`define SCALE 1\n".to_owned(),
        )
        .unwrap();

        let selected = selected_veriloga_source(&app).unwrap();
        assert_eq!(selected.selected_module(), Some("legacy"));
        match compile_selected_source(&selected) {
            VerilogACompileOutcome::Success(_) => {}
            VerilogACompileOutcome::Failure(diagnostics) => {
                panic!("multi-file compile failed: {diagnostics:#?}")
            }
        }
    }

    #[test]
    fn a_compile_carrying_warnings_publishes_its_receipt_and_never_reads_as_failed() {
        let mut app = module_source_with(
            "legacy",
            "module legacy(p, n);\n  inout p, n; electrical p, n; parameter string label = \"x\";\n  analog begin\n    $display(\"%s\", label);\n    I(p, n) <+ V(p, n);\n  end\nendmodule\n",
        );

        let selected = selected_veriloga_source(&app).unwrap();
        let token = selected.token(app.state.model.workspace.content.project.id());
        let VerilogACompileOutcome::Success(report) = compile_selected_source(&selected) else {
            panic!("a discarded system task must not fail the compile")
        };
        assert_eq!(report.diagnostics.len(), 1);

        let receipt = receipt_from_report(
            token,
            &report,
            Some(selected.bundle()),
            None,
            Default::default(),
        )
        .unwrap();
        let warning = receipt
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code() == "VA-SEM-NO-EFFECT-SYSTEM-TASK")
            .expect("the compiler's own warning reaches the editor with its code");
        assert_eq!(warning.severity, CodeEditorSeverity::Warning);
        assert!(warning.message.contains("$display"));
        let summary = receipt.diagnostics.summary();
        assert_eq!(summary.errors, 0);
        assert!(summary.warnings >= 1);

        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (**receipt.compilation.report()).clone(),
            )))
            .unwrap();
        app.state.code.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.compilation.token(),
            receiver: Arc::new(Mutex::new(receiver)),
            cells: None,
            delay_select: Default::default(),
        });

        poll_veriloga_compile(&mut app);

        let published = app
            .state
            .code
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .expect("a pass carrying advisories still publishes a receipt");
        assert_eq!(published.diagnostics.summary().errors, 0);
        assert!(
            published
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code() == "VA-SEM-NO-EFFECT-SYSTEM-TASK")
        );
        assert!(
            app.state
                .code
                .code_workspace
                .veriloga
                .last_failure
                .is_empty()
        );
        assert!(
            app.state
                .code
                .code_workspace
                .veriloga
                .last_failure_token
                .is_none()
        );
    }

    #[test]
    fn an_unsupported_generate_region_reaches_the_editor_with_its_compiler_code() {
        // `generate` and `genvar` are elaborated (IEEE 1364-2005 section 12.4),
        // so what still carries this code is an `endgenerate` with no region
        // open. What the test is about is unchanged: a parse refusal reaches
        // the editor with its compiler code, severity, and line.
        let app = module_source_with(
            "legacy",
            "module legacy(p, n);\n  inout p, n; electrical p, n;\n  endgenerate\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
        );

        let selected = selected_veriloga_source(&app).unwrap();
        let VerilogACompileOutcome::Failure(diagnostics) = compile_selected_source(&selected)
        else {
            panic!("a region terminator with nothing to terminate")
        };

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code(), "VA-PARSE-UNSUPPORTED-GENERATE");
        assert_eq!(diagnostics[0].severity, CodeEditorSeverity::Error);
        assert!(diagnostics[0].message.contains("no open generate region"));
        assert_eq!(diagnostics[0].line, Some(3));
    }

    /// A variable read before it is assigned keeps its value from the last
    /// accepted time point. The compile succeeds, and Problems lists one
    /// warning for the module naming the variable and the line of that first
    /// read.
    #[test]
    fn a_retained_variable_reaches_problems_as_a_warning_naming_its_first_read() {
        let mut app = module_source_with(
            "counter",
            "module counter(p, n);\n  inout p, n; electrical p, n;\n  real count;\n  analog begin\n    count = count + 1;\n    I(p, n) <+ count * V(p, n);\n  end\nendmodule\n",
        );

        let selected = selected_veriloga_source(&app).unwrap();
        let token = selected.token(app.state.model.workspace.content.project.id());
        let VerilogACompileOutcome::Success(report) = compile_selected_source(&selected) else {
            panic!("a retained variable is legal Verilog-A and must not fail the compile")
        };
        let receipt = receipt_from_report(
            token,
            &report,
            Some(selected.bundle()),
            None,
            Default::default(),
        )
        .unwrap();
        let retained: Vec<_> = receipt
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code() == "VA-SEM-RETAINED-VARIABLE")
            .collect();
        assert_eq!(retained.len(), 1, "one warning per module");
        assert_eq!(retained[0].severity, CodeEditorSeverity::Warning);
        assert_eq!(
            &*retained[0].message,
            "Module `counter` has 1 variable read before it is assigned, so it keeps its value \
             from the last accepted time point: `count` (line 5)"
        );

        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (**receipt.compilation.report()).clone(),
            )))
            .unwrap();
        app.state.code.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.compilation.token(),
            receiver: Arc::new(Mutex::new(receiver)),
            cells: None,
            delay_select: Default::default(),
        });
        poll_veriloga_compile(&mut app);

        // Problems lists every diagnostic of the published receipt.
        let published = app
            .state
            .code
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .expect("a pass carrying a warning still publishes a receipt");
        let warning = published
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code() == "VA-SEM-RETAINED-VARIABLE")
            .expect("the published receipt carries the retained variable");
        assert!(
            warning.message.contains("`count` (line 5)"),
            "{}",
            warning.message
        );
    }

    #[test]
    fn completed_async_compile_cannot_cross_publish_between_cell_views() {
        let mut app = RSpiceApp::test_instance();
        let first = add_cell_bundle(&mut app, "first");
        let second = add_cell_bundle(&mut app, "second");
        app.state
            .model
            .workspace
            .open_view(first, ViewType::VerilogA);
        let first_selected = selected_veriloga_source(&app).unwrap();
        let receipt = compile_project_bundle_receipt(
            app.state.model.workspace.content.project.id(),
            first_selected.bundle(),
            first_selected.selected_module(),
        )
        .unwrap();
        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (**receipt.compilation.report()).clone(),
            )))
            .unwrap();
        app.state.code.code_workspace.veriloga.receipt = None;
        app.state.code.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.compilation.token(),
            receiver: Arc::new(Mutex::new(receiver)),
            cells: None,
            delay_select: Default::default(),
        });
        app.state
            .model
            .workspace
            .open_view(second, ViewType::VerilogA);

        poll_veriloga_compile(&mut app);

        assert!(app.state.code.code_workspace.veriloga.pending.is_none());
        assert!(app.state.code.code_workspace.veriloga.receipt.is_none());
        let second_selected = selected_veriloga_source(&app).unwrap();
        assert!(!second_selected.matches_token(
            app.state.model.workspace.content.project.id(),
            receipt.compilation.token()
        ));
    }

    #[test]
    fn completed_async_compile_cannot_publish_after_module_contract_changes() {
        let mut app = RSpiceApp::test_instance();
        let reference = add_cell_bundle(&mut app, "module_change");
        app.state
            .model
            .workspace
            .open_view(reference.clone(), ViewType::VerilogA);
        let selected = selected_veriloga_source(&app).unwrap();
        let receipt = compile_project_bundle_receipt(
            app.state.model.workspace.content.project.id(),
            selected.bundle(),
            selected.selected_module(),
        )
        .unwrap();
        let (sender, receiver) = mpsc::channel();
        sender
            .send(VerilogACompileOutcome::Success(Box::new(
                (**receipt.compilation.report()).clone(),
            )))
            .unwrap();
        app.state.code.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: receipt.compilation.token(),
            receiver: Arc::new(Mutex::new(receiver)),
            cells: None,
            delay_select: Default::default(),
        });
        app.state
            .model
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .and_then(|cell| cell.get_view_mut(&reference.view))
            .unwrap()
            .metadata
            .insert("veriloga.module".to_owned(), "different_model".to_owned());

        poll_veriloga_compile(&mut app);

        assert!(app.state.code.code_workspace.veriloga.pending.is_none());
        assert!(app.state.code.code_workspace.veriloga.receipt.is_none());
        let changed = selected_veriloga_source(&app).unwrap();
        assert!(!changed.matches_token(
            app.state.model.workspace.content.project.id(),
            receipt.compilation.token()
        ));
        assert!(!changed.bundle().validation_is_current());
    }

    /// The published receipt names the min:typ:max selection its compile
    /// kept, as it names the cells: a receipt compiled under min serves a
    /// deck selecting min, and is stale for one keeping the typical values.
    #[test]
    fn the_published_receipt_names_the_selection_it_was_compiled_under() {
        use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;
        let mut app = module_source_with(
            "held",
            "module held(p, n);\n  inout p, n; electrical p, n;\n  parameter real g = 1.0e-3:2.0e-3:3.0e-3;\n  analog I(p, n) <+ g * V(p, n);\nendmodule\n",
        );
        let selected = selected_veriloga_source(&app).unwrap();
        let outcome = rspice_simulation::project_veriloga::compile_service::compile_source(
            selected.bundle(),
            selected.selected_module(),
            None,
            DigitalDelaySelect::Min,
        );
        assert!(matches!(outcome, VerilogACompileOutcome::Success(_)));
        let (sender, receiver) = mpsc::channel();
        sender.send(outcome).unwrap();
        app.state.code.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
            token: selected.token(app.state.model.workspace.content.project.id()),
            receiver: Arc::new(Mutex::new(receiver)),
            cells: None,
            delay_select: DigitalDelaySelect::Min,
        });

        poll_veriloga_compile(&mut app);

        let published = &app
            .state
            .code
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .expect("the compile is published")
            .compilation;
        assert_eq!(published.delay_select(), DigitalDelaySelect::Min);
        assert!(published.builds_for(&[], DigitalDelaySelect::Min));
        assert!(!published.builds_for(&[], DigitalDelaySelect::Typ));
    }
}
