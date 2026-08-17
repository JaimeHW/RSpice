//! Typed workbench commands and their single dispatch boundary.
//!
//! The workbench never paints a visible action without routing it here.  A
//! command is omitted from a menu when its behavior is not implemented; the
//! UI does not advertise speculative or placeholder capability.

use crate::schematic::view::SchematicSymbolContext;
use crate::state::{ComponentType, Tool};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::menu_bar::{FileMenuAction, dispatch_file_menu_action};
use std::cell::RefCell;

use super::state::{
    ProjectLauncherFilter, SpecificationEvidenceFilter, VerificationPage, WorkbenchState, Workspace,
};

pub(crate) mod code_context;
mod registry;
pub(crate) mod vocabulary;

const COMMAND_ZOOM_FACTOR: f64 = 1.2;

fn stop_simulation_enabled(simulation: &crate::state::SimulationState) -> bool {
    simulation.can_request_abort_active_run()
        && crate::simulation::execution::execution_target_supports_cancellation()
}

fn model_library_rescan_diagnostic(app: &RSpiceApp) -> (String, bool) {
    let discovered_files = app.state.pdk_config.discovered_files.len();
    let scan_errors = &app.state.pdk_config.scan_errors;
    let pack_definitions = app.state.model_library_manager.pack_definition_count();
    let summary = format!(
        "Model library rescan found {discovered_files} configured model file(s) and \
         {pack_definitions} shipped pack part(s)"
    );
    if scan_errors.is_empty() {
        (summary, false)
    } else {
        (
            format!(
                "{summary}; {} configured path error(s): {}",
                scan_errors.len(),
                scan_errors.join("; ")
            ),
            true,
        )
    }
}

fn selection_layout_command(
    command: Command,
) -> Option<crate::schematic::view::selection_layout::SelectionLayoutCommand> {
    use crate::schematic::view::selection_layout::SelectionLayoutCommand as Layout;
    Some(match command {
        Command::AlignSelectionLeft => Layout::AlignLeft,
        Command::AlignSelectionCenter => Layout::AlignCenter,
        Command::AlignSelectionRight => Layout::AlignRight,
        Command::AlignSelectionTop => Layout::AlignTop,
        Command::AlignSelectionMiddle => Layout::AlignMiddle,
        Command::AlignSelectionBottom => Layout::AlignBottom,
        Command::DistributeSelectionHorizontal => Layout::DistributeHorizontal,
        Command::DistributeSelectionVertical => Layout::DistributeVertical,
        _ => return None,
    })
}

/// Commands whose design-workspace execution can mutate the active schematic
/// or a project-owned schematic contract. Presentation and inspection
/// commands deliberately remain available in read-only contexts.
pub(crate) const fn command_edits_schematic(command: Command) -> bool {
    matches!(
        command,
        Command::Undo
            | Command::Redo
            | Command::Cut
            | Command::Paste
            | Command::Duplicate
            | Command::Delete
            | Command::RenameSelection
            | Command::PlaceInstance
            | Command::PlaceWire
            | Command::PlaceBus
            | Command::PlaceBusTap
            | Command::PlaceJunction
            | Command::PlaceLabel
            | Command::PlacePin
            | Command::PlaceText
            | Command::PlaceShape
            | Command::MoveSelection
            | Command::StretchSelection
            | Command::ArraySelection
            | Command::ReplaceInstance
            | Command::CreateHierarchy
            | Command::DesignManagement
            | Command::CheckAndSave
            | Command::Place(_)
            | Command::RotateSelection
            | Command::MirrorSelectionHorizontal
            | Command::MirrorSelectionVertical
            | Command::AlignSelectionLeft
            | Command::AlignSelectionCenter
            | Command::AlignSelectionRight
            | Command::AlignSelectionTop
            | Command::AlignSelectionMiddle
            | Command::AlignSelectionBottom
            | Command::DistributeSelectionHorizontal
            | Command::DistributeSelectionVertical
    )
}

/// Resolve the catalog's single selection only when it is an exact,
/// authenticated project-owned revision accepted by the model editor.
///
/// Command enablement is deliberately fail-closed. A stale catalog selection,
/// mismatched retained bytes, or a partial source closure must never expose an
/// editor command that can only fail after navigation.
fn selected_project_model_for_editor(app: &RSpiceApp) -> Result<(&str, &str), &'static str> {
    if !app.state.project_lifecycle.project_open {
        return Err("open a project before editing a device model");
    }
    let library_name = app
        .state
        .model_library_manager
        .selected_library
        .as_deref()
        .ok_or("select one model in Model & library catalog")?;
    let model_name = app
        .state
        .workbench
        .selected_model
        .as_deref()
        .ok_or("select one model in Model & library catalog")?;
    let library = app
        .state
        .model_library_manager
        .get_library(library_name)
        .ok_or("the selected model library no longer exists")?;
    match library.source_authority {
        crate::state::model_library::ModelSourceAuthority::ProjectOwned { .. } => {}
        crate::state::model_library::ModelSourceAuthority::BuiltIn => {
            return Err("the selected model is built-in; create an editable project copy first");
        }
        crate::state::model_library::ModelSourceAuthority::External => {
            return Err("the selected model is external; create an editable project copy first");
        }
        crate::state::model_library::ModelSourceAuthority::RetainedImport { .. } => {
            return Err(
                "the selected model is a read-only retained import; create an editable project copy first",
            );
        }
    }
    crate::workbench::documents::model_editor::resolve_project_model_for_editor(
        &app.state.model_library_manager,
        library_name,
        model_name,
    )
    .map_err(
        |_| "the selected project model retained source or typed definition is inconsistent",
    )?;
    Ok((library_name, model_name))
}

/// Resolve one selected external or built-in model that can become a new
/// project-owned revision. Mutation authority and editor lifecycle checks live
/// here so every command surface exposes the same fail-closed availability.
fn selected_model_for_project_copy(app: &RSpiceApp) -> Result<(&str, &str), &'static str> {
    if !app.state.project_lifecycle.project_open {
        return Err("open a project before creating an editable model copy");
    }
    let library_name = app
        .state
        .model_library_manager
        .selected_library
        .as_deref()
        .ok_or("select one external or built-in model in Model & library catalog")?;
    let model_name = app
        .state
        .workbench
        .selected_model
        .as_deref()
        .ok_or("select one external or built-in model in Model & library catalog")?;
    let library = app
        .state
        .model_library_manager
        .get_library(library_name)
        .ok_or("the selected model library no longer exists")?;
    if !library.models.contains_key(model_name) {
        return Err("the selected model no longer exists in its library");
    }
    if library.source_authority.is_project_owned() {
        return Err("the selected model is already an editable project copy");
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Err("the project is open read-only");
    }
    if app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .is_some()
    {
        return Err("a model qualification run is active");
    }
    if app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .is_some_and(|draft| draft.is_dirty())
    {
        return Err("save or discard the open model candidate first");
    }
    Ok((library_name, model_name))
}

/// Model-editor toolbar commands open governed review surfaces before they
/// perform an operation. This state is deliberately UI-local: the persisted
/// model draft remains the sole source of engineering truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModelEditorWorkflow {
    SaveRevision,
    ValidateCandidate,
    RunQualification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModelEditorWorkflowRequest {
    pub(crate) workflow: ModelEditorWorkflow,
    pub(crate) library_name: String,
    pub(crate) model_name: String,
    pub(crate) prepared: bool,
    pub(crate) error: Option<String>,
}

thread_local! {
    static MODEL_EDITOR_WORKFLOW: RefCell<Option<ModelEditorWorkflowRequest>> = const {
        RefCell::new(None)
    };
}

pub(crate) fn request_model_editor_workflow(
    app: &RSpiceApp,
    workflow: ModelEditorWorkflow,
) -> bool {
    let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
        return false;
    };
    MODEL_EDITOR_WORKFLOW.with(|state| {
        *state.borrow_mut() = Some(ModelEditorWorkflowRequest {
            workflow,
            library_name: draft.library_name.clone(),
            model_name: draft.model_name.clone(),
            prepared: false,
            error: None,
        });
    });
    true
}

pub(crate) fn active_model_editor_workflow(app: &RSpiceApp) -> Option<ModelEditorWorkflowRequest> {
    let identity = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .map(|draft| (draft.library_name.as_str(), draft.model_name.as_str()));
    MODEL_EDITOR_WORKFLOW.with(|state| {
        let mut state = state.borrow_mut();
        let matches_open_candidate = state.as_ref().is_some_and(|request| {
            identity.is_some_and(|(library, model)| {
                request.library_name.eq_ignore_ascii_case(library)
                    && request.model_name.eq_ignore_ascii_case(model)
            })
        });
        if matches_open_candidate {
            state.clone()
        } else {
            *state = None;
            None
        }
    })
}

pub(crate) fn prepare_model_editor_workflow() {
    MODEL_EDITOR_WORKFLOW.with(|state| {
        if let Some(request) = state.borrow_mut().as_mut() {
            request.prepared = true;
        }
    });
}

pub(crate) fn set_model_editor_workflow_error(error: impl Into<String>) {
    MODEL_EDITOR_WORKFLOW.with(|state| {
        if let Some(request) = state.borrow_mut().as_mut() {
            request.error = Some(error.into());
        }
    });
}

pub(crate) fn close_model_editor_workflow() {
    MODEL_EDITOR_WORKFLOW.with(|state| {
        *state.borrow_mut() = None;
    });
}

pub use registry::{CommandAvailability, ShortcutContext, ShortcutKind};

impl Command {
    pub fn is_enabled(self, app: &RSpiceApp) -> bool {
        let state = &app.state;
        if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(state)
            && self.blocked_by_project_operation()
        {
            return false;
        }
        if self.requires_open_project() && !state.project_lifecycle.project_open {
            return false;
        }
        match self {
            Self::OpenWorkspace(workspace) => {
                workspace_available(state.project_lifecycle.project_open, workspace)
            }
            Self::Save => {
                state.project_lifecycle.project_open
                    || state.schematic.current_file.is_some()
                    || state.browser_schematic_save_name.is_some()
            }
            Self::SaveAs => state.project_lifecycle.project_open,
            Self::SaveAll => {
                state.project_lifecycle.project_open
                    && crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state)
            }
            Self::RevertActiveDocument => {
                state.project_lifecycle.accepted().is_some()
                    && crate::workbench::lifecycle::project_lifecycle::active_document_is_dirty(state)
                    && !state.simulation.has_active_execution()
            }
            Self::CloseActiveDocument => {
                crate::workbench::lifecycle::project_lifecycle::can_close_active_document(state)
            }
            Self::CloseProject => state.project_lifecycle.project_open,
            Self::OpenNetlist => {
                !state.simulation.has_active_execution()
            }
            Self::ImportNetlist => {
                state.project_lifecycle.project_open
                    && !state.workbench.safe_mode.project_read_only()
                    && !state.simulation.has_active_execution()
            }
            Self::PublishToWeb => {
                matches!(
                    app.cloud_account.availability(),
                    crate::services::cloud_account::CloudAccountAvailability::Native
                ) && app
                    .cloud_account
                    .snapshot()
                    .cloud_feature_enabled(crate::services::cloud_account::CLOUD_PUBLISHING_FEATURE)
            }
            // Hosting needs an open project, but the same dialog joins by
            // code from any state — so the command never requires one.
            Self::LiveSession => {
                matches!(
                    app.cloud_account.availability(),
                    crate::services::cloud_account::CloudAccountAvailability::Native
                        | crate::services::cloud_account::CloudAccountAvailability::Browser
                ) && app.cloud_account.snapshot().cloud_feature_enabled(
                    crate::services::cloud_account::LIVE_COLLABORATION_FEATURE,
                )
            }
            Self::PageSetup => crate::workbench::app::drawing_sheet_setup_available(app),
            Self::SheetFormatManager => {
                crate::workbench::app::drawing_sheet_setup_available(app)
                    && !state.schematic_edit_read_only()
                    && !state.dialogs.drawing_sheet_support.manager.open
                    && state
                        .workspace
                        .design_management
                        .sheet_catalog(&state.workspace.active_key())
                        .is_some_and(|catalog| catalog.active().is_some())
            }
            Self::CustomSheetSizes => {
                crate::workbench::app::drawing_sheet_setup_available(app)
                    && !state.dialogs.drawing_sheet_presets.any_open()
            }
            Self::ExportActiveView => {
                crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available(state)
            }
            Self::PrintHardcopy => {
                cfg!(any(target_os = "windows", target_arch = "wasm32"))
                    && crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available(
                        state,
                    )
            }
            Self::Undo => {
                if state.workbench.safe_mode.project_read_only() {
                    false
                } else if (netlist_page_is_visible(state)
                    && crate::workbench::documents::netlist_document::can_undo_netlist_edit(state))
                    || state.can_undo_project_design()
                {
                    true
                } else if active_symbol_editor(app) {
                    state.can_undo_active_symbol_document()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && state.schematic.can_undo()
                }
            }
            Self::Redo => {
                if state.workbench.safe_mode.project_read_only() {
                    false
                } else if (netlist_page_is_visible(state)
                    && crate::workbench::documents::netlist_document::can_redo_netlist_edit(state))
                    || state.can_redo_project_design()
                {
                    true
                } else if active_symbol_editor(app) {
                    state.can_redo_active_symbol_document()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && state.schematic.can_redo()
                }
            }
            Self::Cut => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only()
                        && !state.ui.symbol.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && schematic_selection_has_live_object(&state.schematic)
                }
            }
            Self::Delete => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only()
                        && !state.ui.symbol.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && schematic_selection_has_deletable_object(&state.schematic)
                }
            }
            Self::Duplicate => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only()
                        && !state.ui.symbol.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && schematic_selection_has_duplicable_object(&state.schematic)
                }
            }
            Self::Copy => {
                if state.workbench.workspace == Workspace::Results {
                    state.ui.results.cursors.a.is_some()
                } else if active_symbol_editor(app) {
                    !state.ui.symbol.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && schematic_selection_has_live_object(&state.schematic)
                }
            }
            Self::Paste => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only() && !state.ui.symbol.clipboard.is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic_edit_read_only()
                        && !state.schematic.clipboard.is_empty()
                }
            }
            Self::SelectAll => active_symbol_editor(app) || active_schematic_editor(app),
            Self::RenameSelection => {
                active_schematic_editor(app)
                    && crate::workbench::app::rename_selection_available(state)
            }
            Self::RotateSelection
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.schematic.selection.components.iter().any(|id| {
                        state
                            .schematic
                            .components
                            .iter()
                            .any(|component| component.id == *id)
                    })
            }
            Self::AlignSelectionLeft
            | Self::AlignSelectionCenter
            | Self::AlignSelectionRight
            | Self::AlignSelectionTop
            | Self::AlignSelectionMiddle
            | Self::AlignSelectionBottom
            | Self::DistributeSelectionHorizontal
            | Self::DistributeSelectionVertical => {
                active_schematic_editor(app)
                    && selection_layout_command(self).is_some_and(|command| {
                        crate::schematic::view::selection_layout::selection_layout_availability(
                            state, command,
                        )
                        .is_ok()
                    })
            }
            Self::MoveSelection => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.schematic.has_live_movable_selection()
            }
            Self::StretchSelection => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.schematic.default_stretch_target().is_some()
            }
            Self::ArraySelection => {
                active_schematic_editor(app)
                    && !state.schematic_edit_read_only()
                    && state.schematic.validate_array_source_selection().is_ok()
            }
            Self::ReplaceInstance => {
                active_schematic_editor(app)
                    && crate::workbench::app::replace_instance_available(state)
            }
            Self::CreateHierarchy => {
                active_schematic_editor(app)
                    && crate::workbench::app::create_hierarchy_available(state)
            }
            Self::ConnectivityManager => {
                active_schematic_editor(app) && state.project_lifecycle.project_open
            }
            Self::DesignManagement => {
                active_schematic_editor(app)
                    && state.project_lifecycle.project_open
                    && !state.schematic_edit_read_only()
            }
            Self::SelectionBulkEdit => {
                active_schematic_editor(app) && state.project_lifecycle.project_open
            }
            Self::ConfigurationSets => state.project_lifecycle.project_open,
            Self::ReviewComments => {
                active_schematic_editor(app) && state.project_lifecycle.project_open
            }
            Self::RevisionHistory => state.project_lifecycle.project_open,
            Self::ObjectProperties => {
                if active_symbol_editor(app) {
                    let selection = state.ui.symbol.effective_selection();
                    selection.pins.len() + selection.shapes.len() == 1
                } else {
                    active_schematic_editor(app)
                        && crate::workbench::app::selected_object_properties_available(state)
                }
            }
            Self::ZoomFit => {
                active_symbol_editor(app)
                    || active_schematic_editor(app)
                    || (state.workbench.workspace == Workspace::Results
                        && crate::workbench::documents::result_document::fit_gesture_available(state))
            }
            Self::FitSchematicContent | Self::DrawingSheetLayers => active_schematic_editor(app),
            // Magnifying a result viewport needs the sheet's own extents, so
            // it is offered on the unit-pane stack and nowhere it could not
            // be carried out.
            Self::ZoomIn | Self::ZoomOut => {
                active_symbol_editor(app)
                    || active_schematic_editor(app)
                    || (state.workbench.workspace == Workspace::Results
                        && crate::workbench::documents::result_document::zoom_gesture_available(state))
            }
            Self::ZoomOneToOne => active_symbol_editor(app) || active_schematic_editor(app),
            Self::ResetActiveView => reset_active_view_available(state.workbench.workspace),
            Self::EngineeringTableView => active_schematic_editor(app),
            Self::NewApplicationWindow
            | Self::DetachDocument
            | Self::MoveDocumentToWindow
            | Self::ReattachDocument
            | Self::ConsolidateWindows
            | Self::MonitorRecovery
                if state.application_modal_open() =>
            {
                false
            }
            Self::NewApplicationWindow => true,
            Self::DetachDocument => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.active)
            }
            Self::MoveDocumentToWindow => {
                state.workbench.window_session.windows().count() > 1
                    && crate::workbench::chrome::document_bar::document_descriptors(state)
                        .iter()
                        .any(|document| document.open && document.active)
            }
            Self::ReattachDocument => {
                !state.workbench.window_session.current().is_primary()
                    && crate::workbench::chrome::document_bar::document_descriptors(state)
                        .iter()
                        .any(|document| document.open && document.active)
            }
            Self::ConsolidateWindows | Self::MonitorRecovery => {
                !state
                    .workbench
                    .window_session
                    .secondary_window_ids()
                    .is_empty()
            }
            Self::ToggleResultsSplit => {
                state.project_lifecycle.project_open
                    && state.workbench.supports_results_split()
                    && state.simulation.has_retained_result_dataset()
            }
            Self::PreviousDocument | Self::NextDocument => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .filter(|document| document.open)
                    .count()
                    > 1
            }
            Self::CloseOtherDocuments => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.closable && !document.active)
            }
            Self::CloseAllDocuments => {
                crate::workbench::chrome::document_bar::document_descriptors(state)
                    .iter()
                    .any(|document| document.open && document.closable)
            }
            Self::CycleGrid => active_symbol_editor(app) || active_schematic_editor(app),
            Self::GridSnapRouting => active_schematic_editor(app),
            Self::VisibilityOptions => active_schematic_editor(app),
            Self::SelectTool => active_symbol_editor(app) || active_schematic_editor(app),
            Self::PlaceWire
            | Self::PlaceBus
            | Self::PlaceBusTap
            | Self::PlaceJunction
            | Self::PlaceProbe
            | Self::PlacePin
            | Self::PlaceText
            | Self::PlaceShape => {
                active_schematic_editor(app) && !state.schematic_edit_read_only()
            }
            Self::PlaceInstance | Self::PlaceLabel | Self::Place(_) => {
                active_schematic_editor(app) && !state.schematic_edit_read_only()
            }
            Self::SymbolPinTool
            | Self::SymbolPolylineTool
            | Self::SymbolRectangleTool
            | Self::SymbolCircleTool
            | Self::SymbolArcTool
            | Self::SymbolPolygonTool
            | Self::SymbolTextTool => active_symbol_editor(app) && !state.active_view_read_only(),
            Self::SymbolRotatePin | Self::SymbolMirrorPin => {
                active_symbol_editor(app)
                    && !state.active_view_read_only()
                    && state.ui.symbol.effective_selection().pins.len() == 1
                    && state.ui.symbol.effective_selection().shapes.is_empty()
                    && state.ui.symbol.effective_selection().attributes.is_empty()
                    && state.ui.symbol.effective_selection().texts.is_empty()
            }
            Self::SymbolSave => active_symbol_editor(app) && !state.active_view_read_only(),
            Self::AscendHierarchy => {
                active_schematic_editor(app) && state.workspace.hierarchy_stack.len() > 1
            }
            Self::DescendHierarchy | Self::DescendHierarchyDirect => {
                active_schematic_editor(app) && state.selected_hierarchy_master().is_some()
            }
            // A symbol cellview checks its pin contract. The editor's own
            // keyboard route always did this; the command must agree, or the
            // toolbar shows a dead control beside a working shortcut.
            Self::RunChecks => active_schematic_editor(app) || active_symbol_editor(app),
            Self::CheckAndSave => {
                active_schematic_editor(app) && !state.schematic_edit_read_only()
            }
            Self::ClearChecks if active_symbol_editor(app) => state.dialogs.drc_results.is_some(),
            Self::ClearChecks => {
                state.dialogs.drc_results.is_some()
                    || state.active_design_check_status().last_receipt().is_some()
            }
            Self::NextViolation | Self::PreviousViolation if active_symbol_editor(app) => state
                .dialogs
                .drc_results
                .as_ref()
                .is_some_and(|result| !result.violations().is_empty()),
            Self::NextViolation | Self::PreviousViolation => state
                .active_design_check_status()
                .current_receipt()
                .is_some_and(|receipt| !receipt.result.violations().is_empty()),
            Self::RunSimulation => {
                if state.workbench.workspace == Workspace::Netlist {
                    app.manual_deck_run_block_reason().is_none()
                } else {
                    !state.simulation.has_active_execution()
                        && !state.simulation.trigger_simulation
                }
            }
            // The manager edits the plan catalog through the active plan's
            // stable identity. A project whose plan never migrated to that
            // identity has nothing for it to select, so the command reports
            // unavailable rather than opening an empty dialog.
            Self::ManageSimulationPlans => state.sim_setup.stable_analysis_plan().is_ok(),
            // Each of these opens a window that only the netlist page draws.
            // Availability therefore has to name the page, not just the
            // workspace: from Verilog-A or Automation they would set a dialog
            // open with nothing on screen, and it would then appear
            // unprompted the moment the user came back.
            // Find now exists on all three code pages: the netlist page has
            // its own deck find, and the two language pages search their
            // bundle. It was restricted to the netlist page only while the
            // bundle search had no surface.
            Self::FindCodeDocument => state.workbench.workspace == Workspace::Netlist,
            Self::ValidateCodeDocument => {
                netlist_page_is_visible(state)
                    && state.ui.netlist.active_document
                        != crate::workbench::documents::netlist_document::ActiveNetlistDocument::GeneratedDiff
                    && !state.simulation.netlist_content.trim().is_empty()
            }
            // Unlike its neighbours this one belongs to the two language
            // pages, not the netlist page: it manages a project source
            // bundle, which the deck is not.
            Self::ManageSourceDocument => {
                state.workbench.workspace == Workspace::Netlist
                    && crate::workbench::app::source_document_dialog_is_available(state)
            }
            Self::SourceLanguageTools => {
                state.workbench.workspace == Workspace::Netlist
                    && crate::workbench::app::language_tools_are_available(state)
            }
            // This gate was written for the compile command and never
            // attached, so the button stayed enabled with no bundle, no
            // selected source, an empty root, a compile already running, or a
            // read-only project -- and then the surface refused the request it
            // had just been handed.
            Self::CompileVerilogA => code_context::resolve_veriloga_compile(app).is_some(),
            Self::CompareGeneratedRevisions => {
                netlist_page_is_visible(state)
                    && !state.ui.netlist.generated_history.is_empty()
                    && state.ui.netlist.generated_document.is_some()
            }
            Self::StopSimulation => stop_simulation_enabled(&state.simulation),
            Self::ClearResults => {
                state.simulation.has_results()
                    && !state.simulation.has_active_execution()
            }
            Self::ImportResultDataset => {
                !state.workbench.safe_mode.project_read_only()
                    && !state.simulation.has_active_execution()
            }
            // Waves is the workspace's default sheet, and its empty state is
            // the workspace's landing, so this route stays actionable before a
            // dataset exists. Every other viewer is only actionable when the
            // active retained dataset satisfies the same compatibility
            // contract used by the in-workspace viewer tabs.
            Self::ResultViewer(crate::workbench::ResultViewer::Waves) => true,
            Self::ResultViewer(viewer) => {
                crate::workbench::documents::result_document::viewer_is_available(state, viewer)
            }
            Self::ToggleLinkedCursors => {
                state.workbench.workspace == Workspace::Results && state.simulation.has_results()
            }
            Self::DatasetManifestBrowser => {
                state.project_lifecycle.project_open && !state.simulation.runs.is_empty()
            }
            Self::CreateResultDocument => {
                state.project_lifecycle.project_open && state.simulation.has_results()
            }
            Self::CompareResultDatasets => {
                crate::workbench::documents::visualization_studio::results_comparison_available(
                    state,
                )
            }
            Self::ModelCreateProjectCopy => selected_model_for_project_copy(app).is_ok(),
            Self::ModelEditor | Self::ModelCorrelation => {
                selected_project_model_for_editor(app).is_ok()
            }
            Self::ModelSaveRevision => {
                state
                    .workbench
                    .model_editor
                    .draft
                    .as_ref()
                    .is_some_and(|draft| {
                        draft.is_dirty()
                            && !state.workbench.safe_mode.project_read_only()
                            && state.project_lifecycle.project_open
                            && state
                                .workbench
                                .model_editor
                                .qualification_execution
                                .is_none()
                    })
            }
            Self::ModelValidate => state.workbench.model_editor.draft.is_some(),
            Self::ModelRunQualificationTests => state
                .workbench
                .model_editor
                .draft
                .as_ref()
                .is_some_and(|draft| {
                    state.project_lifecycle.project_open
                        && !state.workbench.safe_mode.project_read_only()
                        && !draft.definition_is_dirty()
                        && draft
                            .qualification
                            .validate_for_model(&draft.model_name)
                            .is_ok()
                        && crate::state::model_library::ModelSourceEvidenceBinding::try_new_project_bound(
                            &draft.model_name,
                            draft.source_id,
                            draft.base_source_digest,
                            draft.base_source_revision,
                        )
                        .is_ok_and(|source| {
                            draft
                                .qualification
                                .exact_suites_for_source(&source)
                                .is_ok_and(|suites| !suites.is_empty())
                        })
                        && state.workbench.model_editor.validation.is_some()
                        && state
                            .workbench
                            .model_editor
                            .qualification_execution
                            .is_none()
                }),
            Self::ModelCompareRelease => {
                state
                    .workbench
                    .model_editor
                    .draft
                    .as_ref()
                    .is_some_and(|draft| {
                        !draft.definition_is_dirty()
                            && !draft.qualification.candidates.is_empty()
                            && !draft.qualification.releases.is_empty()
                            && draft
                                .qualification
                                .validate_for_model(&draft.model_name)
                                .is_ok()
                            && state
                                .workbench
                                .model_editor
                                .qualification_execution
                                .is_none()
                    })
            }
            Self::VisualizationStudio => state.project_lifecycle.project_open,
            Self::ReportAuthoring => super::surfaces::report_authoring::can_open(state),
            Self::SaveReportDocument => {
                super::surfaces::report_authoring::can_save_document(state)
            }
            Self::AddReportPage => super::surfaces::report_authoring::can_add_page(state),
            Self::ReportPageProperties => {
                super::surfaces::report_authoring::can_edit_page_properties(state)
            }
            Self::AddVisualizationPane | Self::ExportVisualizationDocument => {
                state.workbench.current_route().surface_id()
                    == super::SurfaceId::VisualizationStudio
                    && state.project_lifecycle.project_open
                    && state.simulation.has_results()
            }
            Self::VisualizationTraceManager
            | Self::VisualizationCursorManager
            | Self::ReviewNotes
            | Self::MeasurementLibrary
            | Self::FamilySlicing => {
                matches!(
                    state.workbench.current_route().surface_id(),
                    super::SurfaceId::Results | super::SurfaceId::VisualizationStudio
                ) && state.project_lifecycle.project_open
                    && state.simulation.has_results()
            }
            Self::VisualizationDocumentProperties => {
                let surface = state.workbench.current_route().surface_id();
                matches!(
                    surface,
                    super::SurfaceId::Results | super::SurfaceId::VisualizationStudio
                ) && state.project_lifecycle.project_open
                    && (surface == super::SurfaceId::VisualizationStudio
                        || state.simulation.has_results())
            }
            Self::ExpressionDiagnostics => state.simulation.has_results(),
            Self::ExportWaveformsCsv => state.simulation.has_results(),
            Self::VerificationPage(page) if !page.is_operational() => false,
            Self::ClearConsole => {
                !state.log_buffer.is_empty() || !state.script_console.history.is_empty()
            }
            _ => true,
        }
    }

    pub(crate) fn execute_with_feedback(self, app: &mut RSpiceApp, ctx: &egui::Context) {
        let available = self.availability(app) == CommandAvailability::Available;
        self.execute(app);
        if available && self == Self::RescanModelLibraries {
            let (message, has_errors) = model_library_rescan_diagnostic(app);
            if has_errors {
                app.state.ui.toasts.warn_with_title(
                    ctx,
                    "Model library rescan completed with warnings",
                    message,
                );
            } else {
                app.state
                    .ui
                    .toasts
                    .success(ctx, "Model libraries rescanned", message);
            }
        }
    }

    pub fn execute(self, app: &mut RSpiceApp) {
        if crate::workbench::lifecycle::project_lifecycle::operation_in_progress(&app.state)
            && self.blocked_by_project_operation()
        {
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Wait for the current project operation to finish before starting another.",
                ));
            return;
        }
        if self.requires_open_project() && !app.state.project_lifecycle.project_open {
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Open a project before using this command.",
                ));
            return;
        }
        if active_schematic_editor(app)
            && command_edits_schematic(self)
            && app.state.schematic_edit_read_only()
        {
            app.state.deny_read_only_edit();
            return;
        }
        match self {
            Self::OpenWorkspace(workspace) => {
                if workspace_available(app.state.project_lifecycle.project_open, workspace) {
                    activate_workspace(app, workspace);
                }
            }
            Self::ProjectLauncher => app.state.workbench.open_project_launcher(),
            Self::RecentProjects => open_recent_projects(&mut app.state.workbench),
            Self::NewProject => file_action(app, FileMenuAction::NewProject),
            Self::OpenProject => file_action(app, FileMenuAction::OpenProject),
            // File Save always persists the project, including incomplete or
            // unvalidated owned netlist work. Publishing a standalone source
            // file is an explicit Netlist-toolbar operation and must never
            // replace project persistence just because that editor is active.
            Self::Save => file_action(app, FileMenuAction::Save),
            Self::SaveAs => file_action(app, FileMenuAction::SaveProjectAs),
            Self::SaveAll => file_action(app, FileMenuAction::SaveAll),
            Self::RevertActiveDocument => file_action(app, FileMenuAction::RevertActiveDocument),
            Self::CloseActiveDocument => file_action(app, FileMenuAction::CloseActiveDocument),
            Self::CloseProject => file_action(app, FileMenuAction::CloseProject),
            Self::NewCell => open_new_cell_dialog(app),
            Self::OpenDocument => file_action(app, FileMenuAction::Open),
            Self::OpenNetlist => file_action(app, FileMenuAction::OpenNetlist),
            Self::ImportNetlist => file_action(app, FileMenuAction::ImportNetlist),
            Self::ImportVerilogA => file_action(app, FileMenuAction::ImportVerilogA),
            Self::ImportResultDataset => {
                crate::workbench::workflows::result_import_workflow::import_result_dataset(
                    &mut app.state,
                );
            }
            Self::ExportSchematicSvg => file_action(app, FileMenuAction::ExportSvg),
            Self::ExportWaveformsCsv => file_action(app, FileMenuAction::ExportCsvWaveforms),
            Self::LiveSession => app.open_live_session_dialog(),
            Self::PublishToWeb => app.open_publish_web_dialog(),
            Self::ExportPublicationSnapshot => {
                file_action(app, FileMenuAction::ExportPublicationSnapshot)
            }
            Self::ExportNetlist(format) => {
                if netlist_page_is_visible(&app.state) {
                    app.state.ui.netlist.export_dialog.open = true;
                    app.state.ui.netlist.export_dialog.format = format;
                    app.state.ui.netlist.export_dialog.error = None;
                } else {
                    crate::workbench::menu_bar::action_export_netlist_with_io(
                        &mut app.state,
                        format,
                        app.export_workflow_io.as_ref(),
                    );
                }
            }
            Self::PageSetup => {
                if !crate::workbench::app::open_drawing_sheet_setup_for_state(&mut app.state) {
                    app.state.push_user_message(
                        crate::diagnostics::ConsoleMessage::warning(
                            "Page setup is available only for an active schematic or testbench drawing sheet.",
                        ),
                    );
                }
            }
            Self::SheetFormatManager => {
                if let Err(error) = crate::workbench::app::open_sheet_format_manager(&mut app.state)
                {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
                }
            }
            Self::CustomSheetSizes => {
                if !crate::workbench::app::open_custom_sheet_size_library(&mut app.state) {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                            "Custom sheet sizes is already open.",
                        ));
                }
            }
            Self::PrintHardcopy => crate::workbench::app::open_hardcopy_workflow(
                app,
                crate::workbench::app::HardcopyWorkflow::Print,
            ),
            Self::ExportActiveView => crate::workbench::app::open_hardcopy_workflow(
                app,
                crate::workbench::app::HardcopyWorkflow::Export,
            ),
            Self::Exit => file_action(app, FileMenuAction::Exit),
            Self::Undo => app.action_edit_undo(),
            Self::Redo => app.action_edit_redo(),
            Self::Cut => {
                if active_symbol_editor(app) {
                    app.delete_selected_symbol_item(true);
                } else {
                    crate::workbench::app::open_cut_selection_dialog(&mut app.state);
                }
            }
            Self::Copy => {
                if app.state.workbench.workspace == Workspace::Results {
                    if let Some(text) =
                        crate::workbench::documents::result_document::copy_cursor_text(
                            &mut app.state,
                        )
                    {
                        app.state.ui.clipboard_text_request = Some(text);
                    }
                } else if active_symbol_editor(app) {
                    app.copy_selected_symbol_shape();
                } else {
                    crate::schematic::view::sheet_visibility::retain_selection_on_active_sheet(
                        &mut app.state,
                    );
                    app.state.copy_active_schematic_selection();
                }
            }
            Self::Paste => {
                if active_symbol_editor(app) {
                    app.paste_symbol_shape();
                } else {
                    let anchor = app.state.schematic_paste_anchor();
                    if !app.state.schematic.paste_at(anchor) {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                                "Paste could not be completed at the current canvas target",
                            ));
                    }
                }
            }
            Self::Duplicate => {
                if active_symbol_editor(app) {
                    app.copy_selected_symbol_shape();
                    app.paste_symbol_shape();
                } else {
                    crate::workbench::app::open_duplicate_selection_dialog(&mut app.state);
                }
            }
            Self::Delete => {
                if active_symbol_editor(app) {
                    app.delete_selected_symbol_item(false);
                } else {
                    crate::workbench::app::open_delete_selection_dialog(&mut app.state);
                }
            }
            Self::SelectAll => {
                if active_symbol_editor(app) {
                    app.select_all_symbol_items();
                } else {
                    crate::workbench::app::open_select_all_dialog(&mut app.state);
                }
            }
            Self::RenameSelection => {
                crate::workbench::app::open_selected_object_rename(&mut app.state);
            }
            Self::ObjectProperties => {
                if active_symbol_editor(app) {
                    app.state.workbench.inspector_visible = true;
                    app.state.workbench.drawer = Some(super::state::Drawer::Inspector);
                } else {
                    crate::workbench::app::open_selected_object_properties(&mut app.state);
                }
            }
            Self::FindInDesign => {
                if netlist_page_is_visible(&app.state) {
                    app.state.ui.netlist.find.open = true;
                } else {
                    activate_workspace(app, Workspace::Design);
                    app.state.workbench.navigator_visible = true;
                    app.state.workbench.drawer = Some(super::state::Drawer::Navigator);
                    app.state.workbench.focus_navigator_search = true;
                }
            }
            Self::Preferences => {
                let route = super::SurfaceRoute::surface(super::SurfaceId::Preferences);
                if let Err(error) = app
                    .state
                    .workbench
                    .navigate(route, super::RouteTransitionSource::User)
                {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                            error.to_string(),
                        ));
                }
            }
            Self::ZoomIn => {
                if app.state.workbench.workspace == Workspace::Results {
                    crate::workbench::documents::result_document::request_view_gesture(
                        &mut app.state,
                        crate::workbench::documents::result_document::ViewGesture::ZoomIn,
                    );
                } else if active_symbol_editor(app) {
                    app.state.ui.symbol.zoom =
                        (app.state.ui.symbol.zoom * COMMAND_ZOOM_FACTOR as f32).min(16.0);
                } else {
                    app.state.schematic.zoom =
                        (app.state.schematic.zoom * COMMAND_ZOOM_FACTOR).min(8.0);
                }
            }
            Self::ZoomOut => {
                if app.state.workbench.workspace == Workspace::Results {
                    crate::workbench::documents::result_document::request_view_gesture(
                        &mut app.state,
                        crate::workbench::documents::result_document::ViewGesture::ZoomOut,
                    );
                } else if active_symbol_editor(app) {
                    app.state.ui.symbol.zoom =
                        (app.state.ui.symbol.zoom / COMMAND_ZOOM_FACTOR as f32).max(0.1);
                } else {
                    app.state.schematic.zoom =
                        (app.state.schematic.zoom / COMMAND_ZOOM_FACTOR).max(0.25);
                }
            }
            Self::ZoomFit => {
                if app.state.workbench.workspace == Workspace::Results {
                    // The waveform sheets key their viewports by analysis, so
                    // fitting them is not a write to the plot-ordinal store.
                    crate::workbench::documents::result_document::request_view_gesture(
                        &mut app.state,
                        crate::workbench::documents::result_document::ViewGesture::Fit,
                    );
                } else if active_symbol_editor(app) {
                    app.state.ui.symbol.needs_fit = true;
                } else {
                    app.state.schematic.needs_drawing_sheet_fit = true;
                    app.state.schematic.needs_fit = false;
                }
            }
            Self::FitSchematicContent => {
                app.state.schematic.needs_fit = true;
                app.state.schematic.needs_drawing_sheet_fit = false;
            }
            Self::ZoomOneToOne => {
                if active_symbol_editor(app) {
                    app.state.ui.symbol.zoom = 1.0;
                } else {
                    app.state.schematic.zoom = 1.0;
                }
            }
            Self::CycleGrid => cycle_canvas_grid(app),
            Self::GridSnapRouting => {
                crate::workbench::app::open_grid_snap_routing_dialog(&mut app.state);
            }
            Self::DrawingSheetLayers => {
                crate::workbench::app::open_drawing_sheet_layers_dialog(&mut app.state);
            }
            Self::VisibilityOptions => {
                crate::workbench::app::open_schematic_visibility_options(&mut app.state);
            }
            Self::ToggleFullScreen => {
                if app.state.workbench.full_screen_presentation {
                    crate::workbench::exit_full_screen_presentation(app);
                } else {
                    crate::workbench::app::open_full_screen_workflow(&mut app.state);
                }
            }
            Self::ResetActiveView => {
                if reset_active_view_available(app.state.workbench.workspace) {
                    crate::workbench::app::open_reset_active_view_workflow(&mut app.state);
                }
            }
            Self::EngineeringTableView => {
                crate::workbench::app::open_engineering_table_dialog(&mut app.state);
            }
            Self::ToggleNavigator => {
                if app.state.workbench.navigator_visible {
                    app.state.workbench.dismiss_navigator();
                } else {
                    app.state.workbench.navigator_visible = true;
                }
            }
            Self::ToggleInspector => {
                if app.state.workbench.inspector_visible {
                    app.state.workbench.dismiss_inspector();
                } else {
                    app.state.workbench.inspector_visible = true;
                }
            }
            Self::ToggleConsole => {
                if app.state.workbench.focus_mode {
                    app.state.workbench.focus_mode = false;
                    app.state.workbench.console_visible = true;
                } else {
                    app.state.workbench.console_visible = !app.state.workbench.console_visible;
                }
                if !app.state.workbench.console_visible {
                    app.state.workbench.console_maximized = false;
                }
            }
            Self::ToggleResultsSplit => {
                let enabled = !app.state.workbench.split_with_results;
                if enabled {
                    if let Some(run_index) = app.state.simulation.newest_retained_result_run_index()
                    {
                        // The split starts on the newest materialized dataset,
                        // but remains the one canonical Results selection:
                        // there is no copied run, waveform, or document owner.
                        app.state.simulation.select_run(run_index);
                        app.state.workbench.split_with_results = true;
                    }
                } else {
                    app.state.workbench.split_with_results = false;
                }
            }
            Self::OpenProblems => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_visible = true;
                app.state.workbench.console_maximized = false;
                app.state.workbench.console_page = super::state::ConsolePage::Problems;
            }
            Self::OpenConsole => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_visible = true;
                app.state.workbench.console_maximized = false;
            }
            Self::ToggleConsoleMaximized => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_maximized = !app.state.workbench.console_maximized;
                app.state.workbench.console_visible = true;
            }
            Self::ClearConsole => {
                app.state.clear_primary_log();
                app.state.script_console.history.clear();
            }
            Self::ToggleFocusMode => {
                app.state.workbench.focus_mode = !app.state.workbench.focus_mode;
                if app.state.workbench.focus_mode {
                    app.state.workbench.close_drawer();
                }
            }
            Self::ResetLayout => app.state.workbench.reset_layout(),
            Self::NewApplicationWindow => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::NewApplicationWindow,
            ),
            Self::DetachDocument => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::DetachDocument,
            ),
            Self::MoveDocumentToWindow => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::MoveDocument,
            ),
            Self::ReattachDocument => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::ReattachDocument,
            ),
            Self::ConsolidateWindows => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::ConsolidateWindows,
            ),
            Self::MonitorRecovery => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::MonitorRecovery,
            ),
            Self::PreviousDocument => {
                crate::workbench::chrome::document_bar::cycle_document(&mut app.state, true);
            }
            Self::NextDocument => {
                crate::workbench::chrome::document_bar::cycle_document(&mut app.state, false);
            }
            Self::CloseOtherDocuments => {
                let count =
                    crate::workbench::chrome::document_bar::close_other_documents(&mut app.state);
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                        "Closed {count} other document presentation(s); project data was retained"
                    )));
            }
            Self::CloseAllDocuments => {
                let count =
                    crate::workbench::chrome::document_bar::close_all_documents(&mut app.state);
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                        "Closed {count} document presentation(s); pinned project data was retained"
                    )));
            }
            Self::WorkspaceLayouts => {
                let preset = app
                    .state
                    .ui
                    .preferences
                    .workspace()
                    .map(|workspace| workspace.preset())
                    .unwrap_or_default();
                app.state.dialogs.workspace_layout_manager.open(preset);
            }
            Self::WindowManager => app
                .state
                .dialogs
                .window_session
                .open(crate::workbench::app::WindowSessionPage::Windows),
            Self::PreviousWorkspace => app.state.workbench.cycle_workspace(true),
            Self::NextWorkspace => app.state.workbench.cycle_workspace(false),
            Self::SelectTool => {
                if active_symbol_editor(app) {
                    app.state.ui.symbol.tool = crate::workbench::SymbolTool::Select;
                } else {
                    set_tool(app, Tool::Select);
                }
            }
            Self::PlaceInstance => {
                activate_workspace(app, Workspace::Design);
                app.state.workbench.navigator_visible = true;
                app.state.workbench.drawer = Some(super::state::Drawer::Navigator);
                app.state.workbench.design_panel = super::state::DesignPanel::ComponentShelf;
                app.state.workbench.focus_placement_search = true;
            }
            Self::PlaceWire => set_tool(app, Tool::Wire),
            Self::PlaceBus => set_tool(app, Tool::Bus),
            Self::PlaceBusTap => {
                activate_workspace(app, Workspace::Design);
                app.state.dialogs.bus_tap.open();
            }
            Self::PlaceJunction => set_tool(app, Tool::Junction),
            Self::PlaceLabel => set_tool(app, Tool::Label),
            Self::PlaceProbe => set_tool(app, Tool::Probe),
            Self::PlacePin => {
                activate_workspace(app, Workspace::Design);
                let name = app.state.schematic.suggested_port_name("BIAS_EN");
                let design_execution_epoch = app.state.design_execution_epoch;
                let active_schematic_epoch = app.state.active_schematic_epoch;
                let topology_version = app.state.schematic.topology_version();
                let view_path = app.state.workspace.active_view.display_path();
                app.state.dialogs.pin_port.open(
                    name,
                    design_execution_epoch,
                    active_schematic_epoch,
                    topology_version,
                    view_path,
                );
            }
            Self::PlaceText => {
                activate_workspace(app, Workspace::Design);
                let design_execution_epoch = app.state.design_execution_epoch;
                let active_schematic_epoch = app.state.active_schematic_epoch;
                let topology_version = app.state.schematic.topology_version();
                let view_path = app.state.workspace.active_view.display_path();
                app.state.dialogs.design_note.open(
                    design_execution_epoch,
                    active_schematic_epoch,
                    topology_version,
                    view_path,
                );
            }
            Self::PlaceShape => {
                activate_workspace(app, Workspace::Design);
                let design_execution_epoch = app.state.design_execution_epoch;
                let active_schematic_epoch = app.state.active_schematic_epoch;
                let topology_version = app.state.schematic.topology_version();
                let view_path = app.state.workspace.active_view.display_path();
                let expected_shapes = app.state.schematic.documentation_shapes.clone();
                app.state.dialogs.documentation_shape.open(
                    design_execution_epoch,
                    active_schematic_epoch,
                    topology_version,
                    view_path,
                    expected_shapes,
                );
            }
            Self::MoveSelection => {
                crate::workbench::app::open_move_selection_dialog(&mut app.state);
            }
            Self::StretchSelection => {
                crate::workbench::app::open_stretch_selection_dialog(&mut app.state);
            }
            Self::ArraySelection => {
                crate::workbench::app::open_array_selection_dialog(&mut app.state);
            }
            Self::ReplaceInstance => {
                crate::workbench::app::open_replace_instance_dialog(&mut app.state);
            }
            Self::CreateHierarchy => {
                crate::workbench::app::open_create_hierarchy_dialog(&mut app.state);
            }
            Self::ConnectivityManager => {
                crate::workbench::app::open_connectivity_manager(&mut app.state);
            }
            Self::DesignManagement => {
                let route = super::SurfaceRoute::surface(super::SurfaceId::DesignManagement);
                match app
                    .state
                    .workbench
                    .navigate(route, super::RouteTransitionSource::User)
                {
                    Ok(_) => crate::workbench::app::open_design_management_dialog(&mut app.state),
                    Err(error) => {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                                error.to_string(),
                            ))
                    }
                }
            }
            Self::SelectionBulkEdit => {
                crate::workbench::app::open_selection_bulk_edit_dialog(&mut app.state);
            }
            Self::ConfigurationSets => {
                crate::workbench::app::open_configuration_sets_dialog(&mut app.state);
            }
            Self::ReviewComments => {
                crate::workbench::app::open_design_review_comments(&mut app.state);
            }
            Self::RevisionHistory => {
                crate::workbench::app::open_project_revision_history(&mut app.state);
            }
            Self::SymbolPinTool => {
                app.state.ui.symbol.tool = super::SymbolTool::PlacePin;
                let next = app
                    .state
                    .load_active_symbol_document()
                    .ok()
                    .and_then(|document| {
                        document
                            .pins
                            .iter()
                            .find(|pin| pin.position.is_none())
                            .map(|pin| pin.name.clone())
                    });
                if let Some(pin) = next {
                    app.state.ui.symbol.select_pin(pin);
                } else {
                    app.state.ui.symbol.clear_selection();
                }
            }
            Self::SymbolPolylineTool => {
                app.state.ui.symbol.tool = super::SymbolTool::Line;
                app.state.ui.symbol.pending_polyline.clear();
            }
            Self::SymbolRectangleTool => {
                app.state.ui.symbol.tool = super::SymbolTool::Rectangle;
                app.state.ui.symbol.shape_start = None;
            }
            Self::SymbolCircleTool => {
                app.state.ui.symbol.tool = super::SymbolTool::Circle;
                app.state.ui.symbol.shape_start = None;
            }
            Self::SymbolArcTool => {
                app.state.ui.symbol.tool = super::SymbolTool::Arc;
                app.state.ui.symbol.shape_start = None;
            }
            Self::SymbolPolygonTool => {
                app.state.ui.symbol.tool = super::SymbolTool::Polygon;
                app.state.ui.symbol.pending_polyline.clear();
            }
            Self::SymbolTextTool => app.state.ui.symbol.tool = super::SymbolTool::Text,
            Self::SymbolRotatePin => {
                crate::schematic::symbol_editor::rotate_selected_pin(&mut app.state)
            }
            Self::SymbolMirrorPin => {
                crate::schematic::symbol_editor::mirror_selected_pin(&mut app.state)
            }
            Self::SymbolSave => {
                app.state.ui.symbol.save_dialog_open = true;
                app.state.ui.symbol.save_error = None;
            }
            Self::Place(ComponentType::Port) => Self::PlacePin.execute(app),
            Self::Place(kind) => set_tool(app, Tool::Place(kind)),
            Self::RotateSelection => {
                let symbol_context = SchematicSymbolContext::from_state(&app.state);
                crate::schematic::view::sheet_visibility::retain_selection_on_active_sheet(
                    &mut app.state,
                );
                crate::schematic::view::sheet_visibility::with_hidden_wire_topology_preserved(
                    &mut app.state,
                    |schematic| {
                        schematic.rotate_selection_resolved(|component| {
                            symbol_context.terminal_points(component)
                        })
                    },
                );
            }
            Self::MirrorSelectionHorizontal => {
                let symbol_context = SchematicSymbolContext::from_state(&app.state);
                crate::schematic::view::sheet_visibility::retain_selection_on_active_sheet(
                    &mut app.state,
                );
                crate::schematic::view::sheet_visibility::with_hidden_wire_topology_preserved(
                    &mut app.state,
                    |schematic| {
                        schematic.mirror_selection_h_resolved(|component| {
                            symbol_context.terminal_points(component)
                        })
                    },
                );
            }
            Self::MirrorSelectionVertical => {
                let symbol_context = SchematicSymbolContext::from_state(&app.state);
                crate::schematic::view::sheet_visibility::retain_selection_on_active_sheet(
                    &mut app.state,
                );
                crate::schematic::view::sheet_visibility::with_hidden_wire_topology_preserved(
                    &mut app.state,
                    |schematic| {
                        schematic.mirror_selection_v_resolved(|component| {
                            symbol_context.terminal_points(component)
                        })
                    },
                );
            }
            Self::AlignSelectionLeft
            | Self::AlignSelectionCenter
            | Self::AlignSelectionRight
            | Self::AlignSelectionTop
            | Self::AlignSelectionMiddle
            | Self::AlignSelectionBottom
            | Self::DistributeSelectionHorizontal
            | Self::DistributeSelectionVertical => {
                let command =
                    selection_layout_command(self).expect("selection layout command is mapped");
                let symbol_context = SchematicSymbolContext::from_state(&app.state);
                match crate::schematic::view::selection_layout::apply_selection_layout(
                    &mut app.state,
                    &symbol_context,
                    command,
                ) {
                    Ok(true) => {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                                "{} completed as one undoable transaction.",
                                command.label()
                            )))
                    }
                    Ok(false) => {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                                "{} produced no geometry change.",
                                command.label()
                            )))
                    }
                    Err(error) => {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                                format!("{} was not applied: {error}.", command.label()),
                            ))
                    }
                }
            }
            Self::Cancel => {
                if app.state.dialogs.descend_hierarchy.open {
                    app.state.dialogs.descend_hierarchy.close();
                } else if app.state.dialogs.move_selection.armed {
                    crate::workbench::app::cancel_armed_move_selection(&mut app.state);
                } else if app.state.dialogs.stretch_selection.armed {
                    crate::workbench::app::cancel_armed_stretch_selection(&mut app.state);
                } else if app.state.dialogs.array_selection.armed {
                    crate::workbench::app::cancel_armed_array_selection(&mut app.state);
                } else if app.state.workbench.drawer.is_some() {
                    app.state.workbench.close_drawer();
                } else if app.state.dialogs.object_properties.open {
                    app.state.dialogs.object_properties.attempt_close();
                } else if app.state.tabbed_property_dialog.open {
                    app.state.tabbed_property_dialog.attempt_close();
                } else if app.state.workbench.workspace == Workspace::Results
                    && app.state.ui.results.cursors.any()
                {
                    app.state.ui.results.clear_cursors();
                } else {
                    app.state.schematic.cancel_interaction_step();
                }
            }
            Self::AscendHierarchy => {
                app.state.ascend_workspace_level();
            }
            Self::DescendHierarchy => {
                crate::workbench::app::open_descend_hierarchy_dialog(&mut app.state);
            }
            Self::DescendHierarchyDirect => {
                app.state.open_selected_instance_master();
            }
            Self::RunChecks if active_symbol_editor(app) => {
                app.state.run_active_symbol_pin_checks();
            }
            Self::RunChecks => crate::workbench::menu_bar::run_design_rule_check(&mut app.state),
            Self::CheckAndSave => {
                crate::workbench::app::open_check_and_save_dialog(&mut app.state);
            }
            Self::ClearChecks => {
                if !active_symbol_editor(app) {
                    app.state.clear_active_design_check();
                }
                app.state.dialogs.drc_results = None;
                app.state.dialogs.drc_checked_version = 0;
                app.state.dialogs.drc_cycle = None;
            }
            Self::NextViolation => {
                crate::schematic::view::violations::cycle_violation(&mut app.state, 1);
            }
            Self::PreviousViolation => {
                crate::schematic::view::violations::cycle_violation(&mut app.state, -1);
            }
            Self::RunSimulation => {
                if app.state.workbench.workspace == Workspace::Netlist {
                    if app.manual_deck_run_block_reason().is_none() {
                        app.state.request_netlist_manual_deck_run();
                    }
                } else {
                    app.state.workbench.preflight.request_run_and_queue();
                }
            }
            Self::StopSimulation => {
                if stop_simulation_enabled(&app.state.simulation) {
                    if let Err(error) = app.state.simulation.request_abort_active_run() {
                        app.state
                            .push_sim_message(crate::diagnostics::ConsoleMessage::warning(error));
                    }
                } else if app.state.simulation.has_active_execution() {
                    let reason = match Self::StopSimulation.availability(app) {
                        CommandAvailability::Disabled(reason) => reason,
                        CommandAvailability::Available | CommandAvailability::Hidden => {
                            "the active simulation execution cannot accept cancellation"
                        }
                    };
                    app.state
                        .push_sim_message(crate::diagnostics::ConsoleMessage::warning(format!(
                            "Stop request ignored: {reason}; the active run was left intact"
                        )));
                }
            }
            Self::JobsManager => crate::workbench::tools::jobs_manager::open(app),
            Self::PreflightChecks => super::preflight::run(app),
            // Sole owner of "open the plan manager". The toolbar plan chip,
            // the palette and the Simulation Studio title row all route here
            // instead of building the draft themselves.
            Self::ManageSimulationPlans => {
                match app.state.sim_setup.stable_analysis_plan().map(|plan| plan.id()) {
                    Ok(plan_id) => {
                        let name = app.state.sim_setup.active_plan_name().clone();
                        app.state.workbench.simulation_workflow = Some(
                            super::state::SimulationWorkflowDialog::PlanManager(
                                super::state::SimulationPlanManagerDraft::new(
                                    plan_id,
                                    name.as_str(),
                                ),
                            ),
                        );
                    }
                    Err(error) => app
                        .state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(error)),
                }
            }
            Self::SimulationOptions => {
                // The workspace owns a Solver & convergence page that edits the
                // same options and more; send the command there rather than to
                // a second, smaller editor of the same state.
                crate::workbench::menu_bar::open_simulation_options(&mut app.state);
                app.state.workbench.simulation_page =
                    crate::workbench::state::SimulationPage::Solver;
                activate_workspace(app, Workspace::Simulate);
            }
            Self::GenerateNetlist => {
                crate::workbench::menu_bar::action_view_netlist(&mut app.state);
                activate_workspace(app, Workspace::Netlist);
            }
            Self::FindCodeDocument => {
                if let Err(error) =
                    crate::workbench::documents::code_workspace::open_active_source_search(app)
                {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
                }
            }
            Self::ValidateCodeDocument => {
                crate::workbench::workflows::netlist_workflow::validate_visible_netlist_source(app);
            }
            Self::ManageSourceDocument => {
                if let Err(error) =
                    crate::workbench::app::open_source_document_dialog(&mut app.state)
                {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
                }
            }
            Self::SourceLanguageTools => {
                if let Err(error) = crate::workbench::app::open_active_language_tools(app) {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
                }
            }
            Self::CompareGeneratedRevisions => {
                app.state.ui.netlist.comparison_dialog.open = true;
                app.state
                    .ui
                    .netlist
                    .comparison_dialog
                    .selected_history_index = app
                    .state
                    .ui
                    .netlist
                    .generated_history
                    .len()
                    .saturating_sub(1);
            }
            Self::ClearResults => {
                if app.state.simulation.has_active_execution() {
                    app.state
                        .push_sim_message(crate::diagnostics::ConsoleMessage::warning(
                        "Result history cannot be cleared while a simulation execution owns a run"
                            .to_owned(),
                    ));
                } else {
                    app.state.clear_simulation_results();
                }
            }
            Self::ToggleLinkedCursors => app.state.ui.results.toggle_linked_cursors(),
            Self::DatasetManifestBrowser => {
                crate::workbench::documents::result_document::open_dataset_browser(app);
            }
            Self::CreateResultDocument => {
                crate::workbench::documents::result_document::open_create_document(app);
            }
            Self::WaveformCalculator => app.state.dialogs.waveform_calculator_dialog = true,
            Self::ExpressionDiagnostics => {
                app.state.dialogs.expression_diagnostics_dialog = true;
            }
            Self::CompareResultDatasets => {
                crate::workbench::documents::visualization_studio::open_results_comparison(app);
            }
            Self::ResultViewer(viewer) => {
                if viewer == crate::workbench::ResultViewer::Waves
                    || crate::workbench::documents::result_document::viewer_is_available(
                        &app.state, viewer,
                    )
                {
                    app.state.ui.results.viewer = viewer;
                    activate_workspace(app, Workspace::Results);
                } else {
                    let reason =
                        crate::workbench::documents::result_document::viewer_unavailability_reason(
                            &app.state, viewer,
                        )
                        .unwrap_or("the active dataset is incompatible with this viewer");
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                            "{} cannot be opened: {reason}.",
                            viewer.label()
                        )));
                }
            }
            Self::EditSpecifications => {
                crate::workbench::documents::result_document::open_specification_editor(
                    &mut app.state,
                );
            }
            Self::VerificationPage(VerificationPage::Drc) => {
                app.state.push_user_message(
                    crate::diagnostics::ConsoleMessage::warning(
                        "Physical DRC is unavailable until a retained layout source, qualified rule deck, and immutable marker database are integrated.",
                    ),
                );
            }
            Self::VerificationPage(page) => {
                activate_workspace(app, Workspace::Verify);
                app.state.workbench.verification_page = page;
            }
            Self::ProjectPage(page) => {
                activate_workspace(app, Workspace::Project);
                app.state.workbench.project_page = page;
            }
            Self::ModelsPage(page) => {
                activate_workspace(app, Workspace::Models);
                app.state.workbench.models_page = page;
            }
            Self::SimulationPage(page) => {
                activate_workspace(app, Workspace::Simulate);
                app.state.workbench.simulation_page = page;
            }
            Self::ModelBrowser => app.state.model_browser_state.open = true,
            Self::ModelCreateProjectCopy => match selected_model_for_project_copy(app) {
                Ok((library_name, model_name)) => {
                    let library_name = library_name.to_owned();
                    let model_name = model_name.to_owned();
                    match crate::workbench::documents::model_editor::create_editable_project_copy_and_open(
                        app,
                        &library_name,
                        &model_name,
                    ) {
                        Ok((project_library, project_model)) => app.state.push_user_message(
                            crate::diagnostics::ConsoleMessage::info(format!(
                                "Created editable project model '{project_library}/{project_model}'."
                            )),
                        ),
                        Err(error) => app.state.push_user_message(
                            crate::diagnostics::ConsoleMessage::warning(format!(
                                "Cannot create editable project model: {error}"
                            )),
                        ),
                    }
                }
                Err(reason) => {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                            "Cannot create editable project model: {reason}."
                        )))
                }
            },
            Self::ModelEditor => match selected_project_model_for_editor(app) {
                Ok((library_name, model_name)) => {
                    let library_name = library_name.to_owned();
                    let model_name = model_name.to_owned();
                    if let Err(error) =
                        crate::workbench::documents::model_editor::open_project_model(
                            app,
                            &library_name,
                            &model_name,
                        )
                    {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                                format!("Cannot open device model editor: {error}"),
                            ));
                    }
                }
                Err(reason) => {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                            "Cannot open device model editor: {reason}."
                        )))
                }
            },
            Self::ModelCorrelation => match selected_project_model_for_editor(app) {
                Ok(_) => {
                    if let Err(error) = app.state.workbench.navigate(
                        super::SurfaceRoute::surface(super::SurfaceId::ModelCorrelation),
                        super::RouteTransitionSource::User,
                    ) {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                                format!("Cannot open measurement correlation: {error}"),
                            ));
                    }
                }
                Err(reason) => {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                            "Cannot open measurement correlation: {reason}."
                        )))
                }
            },
            Self::ModelSaveRevision => {
                if !request_model_editor_workflow(app, ModelEditorWorkflow::SaveRevision) {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                            "Model revision cannot be reviewed: no project-owned candidate is open",
                        ));
                }
            }
            Self::ModelValidate => {
                if !request_model_editor_workflow(app, ModelEditorWorkflow::ValidateCandidate) {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                        "Model validation cannot be reviewed: no project-owned candidate is open",
                    ));
                }
            }
            Self::ModelRunQualificationTests => {
                if !request_model_editor_workflow(app, ModelEditorWorkflow::RunQualification) {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                            "Qualification cannot be reviewed: no project-owned candidate is open",
                        ));
                }
            }
            Self::ModelCompareRelease => {
                app.state.workbench.model_editor.comparison_open = true;
            }
            Self::PdkSettings => app
                .state
                .pdk_settings_dialog
                .open(app.state.pdk_config.clone()),
            Self::RescanModelLibraries => {
                app.state.pdk_config.discover_model_files();
                app.state.model_library_manager.discover_spice_packs();
                let (message, has_errors) = model_library_rescan_diagnostic(app);
                if has_errors {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
                } else {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::info(message));
                }
            }
            Self::CompileVerilogA => {
                activate_workspace(app, Workspace::Netlist);
                app.state.ui.code_workspace.page =
                    crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA;
                // Navigating to the page is not compiling. The surface owns
                // the compile transaction because opening its review dialog
                // needs an egui context, so the request is left for it to
                // consume on the next frame.
                app.state.ui.code_workspace.veriloga.compile_requested = true;
            }
            Self::AutomationConsole => {
                activate_workspace(app, Workspace::Netlist);
                app.state.ui.code_workspace.page =
                    crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
            }
            Self::CommandPalette => app.state.dialogs.command_palette.open(),
            Self::KeyboardShortcuts => app.state.dialogs.shortcuts_help = true,
            Self::AccountOrganization => super::account_organization::open(app),
            Self::License => app.open_license_dialog(),
            Self::DesignSpecialistWorkspaces | Self::SpecialistToolBrowser => {
                crate::workbench::tools::specialist_tool_browser::open(app);
            }
            Self::VisualizationStudio => {
                crate::workbench::documents::visualization_studio::open(app)
            }
            Self::ReportAuthoring => super::surfaces::report_authoring::open(app),
            Self::SaveReportDocument => super::surfaces::report_authoring::save_document(app),
            Self::AddReportPage => super::surfaces::report_authoring::open_add_page(app),
            Self::ReportPageProperties => {
                super::surfaces::report_authoring::open_page_properties(app);
            }
            Self::AddVisualizationPane => {
                crate::workbench::documents::visualization_studio::open_add_pane(app)
            }
            Self::VisualizationTraceManager => {
                open_studio_tool(app, "Trace and family manager");
                crate::workbench::documents::visualization_studio::open_trace_manager(app);
            }
            Self::VisualizationCursorManager => {
                open_studio_tool(app, "Cursor groups and links");
                crate::workbench::documents::visualization_studio::open_cursor_manager(app);
            }
            Self::ReviewNotes => {
                open_studio_tool(app, "Review notes");
                crate::workbench::documents::visualization_studio::open_annotation_editor(app);
            }
            Self::MeasurementLibrary => {
                open_studio_tool(app, "Measurement library");
                crate::workbench::documents::visualization_studio::open_measurement_editor(app);
            }
            Self::FamilySlicing => {
                open_studio_tool(app, "Family slicing and pivot");
                crate::workbench::documents::visualization_studio::open_family_slicing(app);
            }
            Self::VisualizationDocumentProperties => {
                open_studio_tool(app, "Plot document properties");
                crate::workbench::documents::visualization_studio::open_document_properties(app);
            }
            Self::ExportVisualizationDocument => {
                crate::workbench::documents::visualization_studio::export_document(app);
            }
            Self::FeatureAvailability => {
                let route = super::SurfaceRoute::surface(super::SurfaceId::FeatureAvailability);
                if let Err(error) = app
                    .state
                    .workbench
                    .navigate(route, super::RouteTransitionSource::User)
                {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                            error.to_string(),
                        ));
                }
            }
            Self::InteroperabilityMatrix => {
                let route = super::SurfaceRoute::capability_workflow(
                    super::CapabilityWorkflowId::InteroperabilityMatrix,
                );
                if let Err(error) = app
                    .state
                    .workbench
                    .navigate(route, super::RouteTransitionSource::User)
                {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(
                            error.to_string(),
                        ));
                }
            }
            Self::HelpCenter => app
                .state
                .dialogs
                .help_center
                .open(crate::workbench::app::HelpCenterPage::Help),
            Self::ReleaseNotes => app
                .state
                .dialogs
                .help_center
                .open(crate::workbench::app::HelpCenterPage::ReleaseNotes),
            Self::MigrationGuide => app
                .state
                .dialogs
                .help_center
                .open(crate::workbench::app::HelpCenterPage::MigrationGuide),
            Self::SystemDiagnostics => app
                .state
                .dialogs
                .help_center
                .open(crate::workbench::app::HelpCenterPage::Diagnostics),
            Self::SupportBundle => app
                .state
                .dialogs
                .help_center
                .open(crate::workbench::app::HelpCenterPage::SupportBundle),
            Self::LegalPrivacy => app
                .state
                .dialogs
                .help_center
                .open(crate::workbench::app::HelpCenterPage::LegalPrivacy),
            Self::About => app.state.dialogs.about = true,
        }
    }
}

/// Whether the netlist document — not merely the workspace that hosts it — is
/// the thing on screen.
fn netlist_page_is_visible(state: &crate::workbench::AppState) -> bool {
    state.workbench.workspace == Workspace::Netlist
        && state.ui.code_workspace.page
            == crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
}

fn activate_workspace(app: &mut RSpiceApp, workspace: Workspace) {
    app.state.workbench.activate(workspace);
}

const fn workspace_available(project_open: bool, workspace: Workspace) -> bool {
    project_open || matches!(workspace, Workspace::Project)
}

/// Open a Visualization Studio tool, and say so when that means leaving the
/// result view behind.
///
/// These tools operate across visualization documents, so the Studio is
/// genuinely their home — but a menu item that silently swaps the surface
/// under the reader is a surprise. Announcing the move in the console leaves
/// a record of what happened and why the plot went away.
fn open_studio_tool(app: &mut RSpiceApp, tool: &str) {
    if app.state.workbench.current_route().surface_id() != super::SurfaceId::Results {
        return;
    }
    crate::workbench::documents::visualization_studio::open(app);
    if app.state.workbench.current_route().surface_id() == super::SurfaceId::VisualizationStudio {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
                "{tool} opened in Visualization Studio, which owns tools that span visualization documents. The result view is one Back away."
            )));
    }
}

fn active_symbol_editor(app: &RSpiceApp) -> bool {
    matches!(
        app.state.workbench.workspace,
        Workspace::Design | Workspace::Models
    ) && app.state.workspace.active_view_type() == crate::state::ViewType::Symbol
}

/// Mockup-owned `G` command: cycle only the canvas presentation. Snap pitch
/// and target classes remain independent in Grid, snap and wire routing.
fn cycle_canvas_grid(app: &mut RSpiceApp) {
    if active_symbol_editor(app) {
        let enabled = !app.state.ui.symbol.show_grid;
        app.state.ui.symbol.show_grid = enabled;
        return;
    }

    app.state.ui.cycle_grid_style();
}

fn active_schematic_editor(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && matches!(
            app.state.workspace.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
}

fn schematic_selection_has_live_object(schematic: &crate::state::SchematicState) -> bool {
    let selection = &schematic.selection;
    schematic
        .components
        .iter()
        .any(|component| selection.has_component(component.id))
        || schematic
            .wires
            .iter()
            .any(|wire| selection.has_wire(wire.id))
        || schematic
            .junctions
            .iter()
            .any(|junction| selection.has_junction(junction.pos))
        || schematic
            .net_labels
            .iter()
            .any(|label| selection.has_net_label(label.id))
        || schematic.buses.iter().any(|bus| selection.has_bus(bus.id))
        || schematic
            .bus_taps
            .iter()
            .any(|tap| selection.has_bus_tap(tap.id))
        || schematic
            .design_notes
            .iter()
            .any(|note| selection.has_design_note(note.id))
        || schematic
            .documentation_shapes
            .iter()
            .any(|shape| selection.has_documentation_shape(shape.id))
        || schematic
            .probes
            .iter()
            .any(|probe| selection.has_probe(probe.id))
}

/// Return whether Delete can resolve at least one selected identity to a live
/// complete object. Segment and vertex handles are edit subobjects rather than
/// independently persisted conductors, so the destructive review promotes a
/// live handle to its owning wire before commit.
fn schematic_selection_has_deletable_object(schematic: &crate::state::SchematicState) -> bool {
    schematic_selection_has_live_object(schematic)
        || schematic.wires.iter().any(|wire| {
            schematic.selection.wire_segments.iter().any(|selected| {
                selected.wire_id == wire.id && selected.segment_index < wire.segment_count()
            }) || schematic.selection.wire_vertices.iter().any(|selected| {
                selected.wire_id == wire.id && selected.vertex_index < wire.vertex_count()
            })
        })
}

fn schematic_selection_has_duplicable_object(schematic: &crate::state::SchematicState) -> bool {
    let selection = &schematic.selection;
    selection.wire_segments.is_empty()
        && selection.wire_vertices.is_empty()
        && (schematic
            .components
            .iter()
            .any(|component| selection.has_component(component.id))
            || schematic
                .wires
                .iter()
                .any(|wire| selection.has_wire(wire.id))
            || schematic
                .net_labels
                .iter()
                .any(|label| selection.has_net_label(label.id))
            || schematic.buses.iter().any(|bus| selection.has_bus(bus.id))
            || schematic
                .bus_taps
                .iter()
                .any(|tap| selection.has_bus_tap(tap.id))
            || schematic
                .design_notes
                .iter()
                .any(|note| selection.has_design_note(note.id))
            || schematic
                .documentation_shapes
                .iter()
                .any(|shape| selection.has_documentation_shape(shape.id))
            || schematic
                .probes
                .iter()
                .any(|probe| selection.has_probe(probe.id)))
}

fn set_tool(app: &mut RSpiceApp, tool: Tool) {
    activate_workspace(app, Workspace::Design);
    if app.state.dialogs.move_selection.armed && tool != Tool::MoveSelection {
        app.state.dialogs.move_selection.close();
    }
    if app.state.dialogs.stretch_selection.armed && tool != Tool::StretchSelection {
        app.state.dialogs.stretch_selection.close();
    }
    if app.state.dialogs.array_selection.armed && tool != Tool::ArraySelection {
        app.state.dialogs.array_selection.close();
    }
    app.state.schematic.arm_tool(tool);
}

fn open_recent_projects(workbench: &mut WorkbenchState) {
    workbench.open_project_launcher();
    workbench.project_launcher_filter = ProjectLauncherFilter::Recent;
}

const fn reset_active_view_available(_workspace: Workspace) -> bool {
    true
}

pub(crate) fn reset_active_view(app: &mut RSpiceApp) {
    app.state.workbench.navigator_query.clear();
    match app.state.workbench.workspace {
        Workspace::Project => {
            app.state.workbench.command_query.clear();
        }
        Workspace::Design => {
            if app.state.workspace.active_view_type() == crate::state::ViewType::Symbol {
                app.state.ui.symbol.zoom = 1.0;
                app.state.ui.symbol.pan = (0.0, 0.0);
                app.state.ui.symbol.needs_fit = true;
                app.state.ui.symbol.clear_selection();
                app.state.ui.symbol.marquee_start = None;
                app.state.ui.symbol.marquee_current = None;
            } else {
                app.state.schematic.zoom = 1.0;
                app.state.schematic.pan = (0.0, 0.0);
                app.state.schematic.needs_fit = true;
                app.state.schematic.center_request = None;
                app.state.schematic.selection.clear();
                app.state.schematic.selection_rect.cancel();
                app.state.schematic.net_highlight.clear();
            }
        }
        Workspace::Simulate => {
            // The studio's view state is what narrows its two registries.
            // Everything else on these pages is the plan itself, which a
            // view reset has no business touching.
            app.state.workbench.saved_output_filter.clear();
            app.state.workbench.specification_filter.clear();
            app.state.workbench.specification_evidence_filter = SpecificationEvidenceFilter::All;
        }
        Workspace::Results => {
            let viewer = app.state.ui.results.viewer;
            app.state
                .ui
                .results
                .views
                .retain(|(candidate, _, _), _| *candidate != viewer);
            app.state.ui.results.clear_cursors();
            app.state.ui.results.rf_pin.remove(&viewer);
            if viewer == crate::workbench::ResultViewer::Waves {
                app.state.ui.results.hidden_strips.clear();
                app.state.ui.results.maximized_strip = None;
            }
            app.state
                .workbench
                .visualization_studio
                .reset_transient_view();
        }
        Workspace::Verify => {
            app.state.workbench.selected_specification = None;
        }
        Workspace::Models => {
            app.state.workbench.selected_model = None;
        }
        Workspace::Netlist => {
            app.state.ui.netlist.cursor_line = 0;
            app.state.ui.netlist.completion_open = false;
            app.state.ui.netlist.completion_index = 0;
        }
    }
}

fn open_new_cell_dialog(app: &mut RSpiceApp) {
    let selected = app
        .state
        .library_manager
        .selected_library
        .as_ref()
        .and_then(|name| {
            app.state
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        });
    let target_library = selected.or_else(|| {
        app.state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| !library.read_only)
            .map(|library| library.name.clone())
    });
    let library_revision = app.state.library_manager.revision();

    let dialogs = &mut app.state.dialogs;
    dialogs.new_cell_library = target_library.unwrap_or_default();
    dialogs.new_cell_name.clear();
    dialogs.new_cell_description.clear();
    dialogs.new_cell_create_schematic = true;
    dialogs.new_cell_create_symbol = false;
    dialogs.new_cell_create_testbench = false;
    dialogs.new_cell_error = None;
    dialogs.new_cell_library_revision = library_revision;
    dialogs.new_cell_dialog = true;
}

fn file_action(app: &mut RSpiceApp, action: FileMenuAction) {
    dispatch_file_menu_action(
        &mut app.state,
        action,
        app.file_workflow_io.as_ref(),
        app.export_workflow_io.as_ref(),
    );
}

#[cfg(test)]
mod tests;
