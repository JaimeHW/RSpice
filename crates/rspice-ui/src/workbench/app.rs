//! RSpice Application Core
//!
//! The main eframe/egui application. Each frame:
//!
//! 1. [`RSpiceApp::prepare_frame`] applies the active theme (on change),
//!    resolves keyboard shortcuts, and pumps the simulation controller.
//! 2. [`crate::workbench::show`] renders the IDE chrome — menu bar, toolbar,
//!    workspace tabs, contextual side panels, the active center view,
//!    console, status bar, and toasts (see `crate::workbench` for the layout).
//! 3. [`RSpiceApp::render_frame_dialogs`] renders modal dialogs.
//!
//! # State Management
//!
//! Application state is managed in a centralized `AppState` struct:
//! - SchematicState: circuit topology, components, wires
//! - SimulationState: simulation results, waveforms, run history
//! - WorkbenchState: canonical workspace navigation and responsive layout
//! - UiSessionState: theme and document-engine interaction state
//!
//! This follows the commercial EDA pattern where state is:
//! 1. Centralized for consistency
//! 2. Observable for efficient updates
//! 3. Serializable for session recovery

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::veriloga_library::{
    VERILOGA_LIBRARY_NAME, restore_global_veriloga_library, save_global_veriloga_library,
};

const CONTEXT_LONG_PRESS_DURATION_SECONDS: f64 = 0.56;
const CONTEXT_LONG_PRESS_MOVE_TOLERANCE_POINTS: f32 = 9.0;

/// Small, exact frame boundary for mutable inputs that can change through
/// canvas, dock, or modal ownership in the same egui frame.
///
/// Individual workflows still invalidate at their commit boundary where
/// practical. This key is the final authority that prevents an otherwise
/// valid edit path from leaving a prepared run or preflight receipt behind.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FrameSimulationInput {
    topology_version: u64,
    analysis_plan: Option<(
        crate::product::SimulationPlanId,
        crate::product::ObjectRevision,
    )>,
}

impl FrameSimulationInput {
    fn capture(state: &AppState) -> Self {
        Self {
            topology_version: state.schematic.topology_version(),
            analysis_plan: state
                .sim_setup
                .analysis_plan
                .as_ref()
                .map(|plan| (plan.id(), plan.revision())),
        }
    }
}

#[cfg(target_arch = "wasm32")]
const BROWSER_UNLOAD_WARNING: &str = "RSpice has unsaved changes.";

#[cfg(target_arch = "wasm32")]
type BrowserUnloadListener = wasm_bindgen::closure::Closure<dyn FnMut(web_sys::BeforeUnloadEvent)>;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_UNLOAD_DIRTY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static BROWSER_UNLOAD_LISTENER: std::cell::RefCell<Option<BrowserUnloadListener>> =
        const { std::cell::RefCell::new(None) };
}

mod actions;
mod command_palette;
mod dialogs;
#[cfg(target_arch = "wasm32")]
mod pack_persistence;
#[cfg(target_arch = "wasm32")]
mod pdk_persistence;
mod schematic;

pub use dialogs::confirmation_state::{
    ConfirmationAction, ConfirmationDialogState, ConfirmationResponse,
};
pub(crate) use dialogs::confirmation_state::{ProjectReviewDialogState, ProjectReviewRequest};

/// The Model Hub work vocabulary, named where surfaces can reach it.
pub(in crate::workbench) use dialogs::pdk_workflow::ModelHubRequest;

/// The authored purpose strip that opens a workflow dialog's body, named where
/// the surfaces that compose their own dialog bodies can reach it. One painter
/// for that strip is what keeps the info mark, the height and the dashed rule
/// the same on every dialog that opens with one.
pub(in crate::workbench) use dialogs::review_primitives::purpose_line;

/// The authored concept banner, named where the surfaces that compose their own
/// dialog bodies can reach it. One painter for the box, its two tones and its
/// margins.
pub(in crate::workbench) use dialogs::design_management::widgets::concept_banner;

/// Sheet navigation and sheet lifecycle, named where the sheet strip, the
/// command dispatcher and the design manager can all reach the one owner.
pub(in crate::workbench) use actions::sheets;

pub(crate) use dialogs::configuration_sets::{
    ConfigurationSetsDialogState, open_configuration_binding_dialog, open_configuration_sets_dialog,
};

pub(crate) use dialogs::design_management::{
    DesignManagementDialogState, open_design_management_dialog,
};

pub(crate) use dialogs::drawing_sheet_defaults::{
    DrawingSheetDefaultsDialogState, open_drawing_sheet_defaults,
};
pub use dialogs::drawing_sheet_presets::{
    DRAWING_SHEET_PACKAGE_MAX_BYTES, DrawingSheetPackageEncoding, DrawingSheetPackageInspection,
    DrawingSheetPackageVerification, PublishedDrawingSheetPackage,
    drawing_sheet_publisher_public_key, inspect_drawing_sheet_package,
    publish_organization_drawing_sheet_package, verify_published_drawing_sheet_package,
};
pub(crate) use dialogs::drawing_sheet_presets::{
    DrawingSheetPresetDialogsState, capture_personal_preset_into_project,
    open_custom_sheet_size_library,
};
pub(crate) use dialogs::drawing_sheet_setup::{
    DrawingSheetSetupState, drawing_sheet_setup_available, open_drawing_sheet_setup,
    open_drawing_sheet_setup_for_state, open_drawing_sheet_setup_with_preset,
};
pub(crate) use dialogs::drawing_sheet_support::{
    DrawingSheetSupportState, open_sheet_format_manager, open_title_block_fields,
};

pub(crate) use dialogs::connectivity_manager::{
    ConnectivityManagerDialogState, open_connectivity_manager,
};

pub(crate) use dialogs::selection::bulk_edit::{
    SelectionBulkEditDialogState, open_selection_bulk_edit_dialog,
};

pub(crate) use dialogs::design_review_comments::{
    DesignReviewCommentsDialogState, open_design_review_comments,
};

pub(crate) use dialogs::project::new_project::open_new_project_dialog;

pub(crate) use dialogs::project_revision_history::{
    ProjectRevisionHistoryDialogState, open_project_revision_history,
};

pub(crate) use dialogs::source_document::{
    open_source_document_dialog, source_document_dialog_is_available,
};

pub(crate) use dialogs::source_language_tools::{
    language_tools_are_available, open_active_language_tools,
};

pub(crate) use dialogs::create_model_bound_symbol::{
    CreateModelBoundSymbolDialogState, open_create_model_bound_symbol_dialog,
    open_create_subcircuit_bound_symbol_dialog,
};
pub(crate) use dialogs::symbol_definition::{
    open_symbol_import_dialog_for, open_symbol_parameter_form_dialog_for,
};

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) use dialogs::hardcopy::run_worker_request_value as run_hardcopy_worker_request_value;
pub(crate) use dialogs::hardcopy::{HardcopyDialogState, HardcopyWorkflow, open_hardcopy_workflow};
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) use dialogs::pdk_workflow::run_model_import_worker_request_value;

pub(crate) use dialogs::state::{
    ArraySelectionDialogState, ArraySelectionPreviewCache, BuiltinXspicePlacementDialogState,
    BusObjectPropertiesDraft, BusTapDialogState, BusTapObjectPropertiesDraft,
    DescendHierarchyDialogState, DesignNoteDialogState, DesignNoteObjectPropertiesDraft,
    DocumentationShapeDialogState, DocumentationShapeObjectPropertiesDraft,
    EngineeringTableDialogPage, EngineeringTableDialogState, EngineeringTableExportFormat,
    EngineeringTableExportScope, FullScreenPanels, FullScreenScope, GridSnapRoutingDialogState,
    GridSnapRoutingDraft, GridSnapRoutingFocusTarget, GridSnapSpacingChoice, HelpCenterPage,
    HierarchyDescendEditMode, HierarchyParentContext, MoveSelectionDialogState,
    NamedNetObjectPropertiesDraft, NetLabelObjectPropertiesDraft, NetLabelPlacementDialogState,
    NewWindowInitialContent, ObjectPropertiesDraft, PinPortDialogState, RenameSelectionTarget,
    ReplaceInstanceOpen, StretchSelectionDialogState, TechnologyAttachmentDialogState,
    ViewOperation, ViewOperationDialogState, WindowLayoutChoice, WindowSessionPage, WindowWorkflow,
};
pub use dialogs::state::{DialogState, LicensePhase};

pub(crate) use schematic::edit_authority::SchematicEditAuthority;

pub(crate) use dialogs::selection::workflow::{
    SelectionWorkflowDialogState, open_cut_selection_dialog, open_delete_selection_dialog,
    open_duplicate_selection_dialog, open_duplicate_selection_dialog_at, open_select_all_dialog,
};

pub(crate) use schematic::named_net::{
    NamedNetTarget, apply_named_net_rename, selected_named_net_target, validate_named_net_rename,
};

pub(crate) use dialogs::check_and_save::open_check_and_save_dialog;
pub(crate) use dialogs::drawing_sheet_layers::open_drawing_sheet_layers_dialog;
pub(crate) use dialogs::drawing_sheet_overflow::open_drawing_sheet_overflow_review;
pub(crate) use dialogs::engineering_table::open_engineering_table_dialog;
pub(crate) use dialogs::grid_snap_routing::open_grid_snap_routing_dialog;
pub(crate) use dialogs::hierarchy::create::{
    create_hierarchy_available, open_create_hierarchy_dialog,
};
pub(crate) use dialogs::hierarchy::descend::open_descend_hierarchy_dialog;
pub(crate) use dialogs::replace_instance::{
    open_replace_instance_dialog, replace_instance_available,
};
pub(crate) use dialogs::view_operations::{
    open_full_screen_workflow, open_reset_active_view_workflow,
};
pub(crate) use dialogs::window_session::open_window_workflow;

pub(crate) use actions::property_edit::{
    authoritative_component_property_sheet, open_property_editor, open_selected_object_properties,
    selected_object_properties_available,
};

pub(crate) use dialogs::placement::net_label::open_net_label_placement;
pub(crate) use dialogs::selection::array::{
    armed_array_selection_authority, armed_array_selection_plan, cancel_armed_array_selection,
    open_array_selection_dialog,
};
pub(crate) use dialogs::selection::move_selection::{
    armed_move_selection_authority, cancel_armed_move_selection, open_move_selection_dialog,
};
pub(crate) use dialogs::selection::stretch::{
    armed_stretch_selection_authority, cancel_armed_stretch_selection,
    open_stretch_selection_dialog, stretch_delta_for_policy,
};
#[cfg(target_arch = "wasm32")]
pub(crate) use dialogs::symbol_definition::MAX_SYMBOL_DEFINITION_IMPORT_BYTES;
pub(crate) use dialogs::symbol_definition::{
    SymbolImportDialogState, SymbolParameterFormDialogState, open_symbol_import_dialog,
    open_symbol_parameter_form_dialog,
};
pub(crate) use dialogs::visibility_options::open_schematic_visibility_options;

pub(crate) use dialogs::selection::rename::{
    open_selected_object_rename, rename_selection_available,
};

// =============================================================================
// Main Application
// =============================================================================

/// RSpice Application
///
/// The main egui application providing commercial-grade CAD interface.
pub struct RSpiceApp {
    /// Application state
    pub(crate) state: AppState,
    /// First frame flag (for initialization)
    first_frame: bool,
    /// When the autosave timer was armed (first dirty frame) or last fired.
    #[cfg(not(target_arch = "wasm32"))]
    pub(in crate::workbench) autosave_last: Option<std::time::Instant>,
    /// Theme last applied to the egui context (re-applied when the user
    /// changes the workbench theme).
    applied_theme: Option<crate::ui::Theme>,
    /// Window title last pushed to the OS (avoids a viewport command per frame).
    last_window_title: String,
    /// SVG symbol library for component rendering
    pub(crate) symbol_library: Option<crate::schematic::symbols::SymbolLibrary>,
    /// Simulation controller for running analyses
    pub(crate) simulation_controller: crate::simulation::SimulationController,
    /// Verified application-local Python worker service. It is deliberately
    /// outside persisted state and absent from browser builds, which own a Web
    /// Worker backend instead.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) automation_runtime: crate::automation_runtime::NativeAutomationRuntime,
    #[cfg(target_arch = "wasm32")]
    pub(crate) automation_runtime: crate::automation_runtime::BrowserAutomationRuntime,
    /// Project identity currently authorized to own the process/worker. Any
    /// identity change terminates the worker before another event is polled.
    automation_runtime_project_id: crate::product::ProjectId,
    /// Cloud account session boundary (sign-in, entitlements, license leases).
    pub(crate) cloud_account: crate::services::cloud_account::CloudAccountService,
    /// Distributed model packs: the verified catalog, what is installed, and
    /// the store a background install writes through. It is outside persisted
    /// state because it describes this machine, not this project.
    pub(crate) model_hub: crate::services::model_hub::ModelHubService,
    /// Live-session engine: document sync, write leases, and presence over
    /// the relay port the cloud service hands out.
    pub(crate) live_session: crate::workbench::live_session::LiveSessionEngine,
    /// File workflow IO backend (native in production, injectable in tests).
    pub(crate) file_workflow_io:
        Box<dyn crate::workbench::workflows::file_workflow::FileWorkflowIo>,
    /// Export workflow IO backend (native in production, injectable in tests).
    pub(crate) export_workflow_io:
        Box<dyn crate::workbench::workflows::export_workflow::ExportWorkflowIo>,
}

fn configure_platform_input_contract(ctx: &Context) {
    ctx.options_mut(|options| {
        // The mockup registers this gesture document-wide: a 560 ms press
        // opens the contextual touch sheet unless movement exceeds 9 px.
        options.input_options.max_click_duration = CONTEXT_LONG_PRESS_DURATION_SECONDS;
        options.input_options.max_click_dist = CONTEXT_LONG_PRESS_MOVE_TOLERANCE_POINTS;

        // Ctrl+± / Ctrl+0 zoom the schematic rather than the application UI.
        options.zoom_with_keyboard = false;
    });
}

impl RSpiceApp {
    fn terminate_automation_runtime_after_project_change(&mut self) {
        let project_id = self.state.workspace.project.id();
        if project_id == self.automation_runtime_project_id {
            return;
        }
        let _ = self.automation_runtime.terminate();
        self.automation_runtime_project_id = project_id;
        self.state.ui.code_workspace = Default::default();
        self.simulation_controller.clear_prepared_run();
    }

    pub(crate) fn automation_runtime_unavailable_reason(&mut self) -> Option<String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.automation_runtime.poll_discovery();
            self.automation_runtime
                .availability_reason()
                .map(str::to_owned)
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.automation_runtime.availability_reason()
        }
    }

    /// Drop every authorization and presentation artifact derived from a
    /// mutable simulation input. Call this at the same commit boundary as
    /// plan, PVT, solver-option, model, variable, output, or topology edits.
    pub(crate) fn invalidate_simulation_preflight(&mut self) {
        self.state.workbench.preflight.invalidate();
        self.simulation_controller.clear_prepared_run();
    }

    fn invalidate_simulation_preflight_after_frame_edit(&mut self, before: &FrameSimulationInput) {
        if FrameSimulationInput::capture(&self.state) != *before {
            self.invalidate_simulation_preflight();
        }
    }

    pub(crate) fn manual_deck_run_block_reason(&self) -> Option<String> {
        if let Some(reason) = self.state.manual_deck_run_block_reason() {
            return Some(reason);
        }
        let receipt = self.state.ui.netlist.validation.as_ref()?;
        if !self
            .simulation_controller
            .has_retained_manual_authorization(receipt.prepared_snapshot_digest)
        {
            return Some(
                "Validate the exact current netlist before running; its one-shot authorization is no longer retained"
                    .to_owned(),
            );
        }
        None
    }

    #[cfg(all(test, not(target_arch = "wasm32")))]
    pub(crate) fn test_instance() -> Self {
        let state = AppState::default();
        let automation_runtime_project_id = state.workspace.project.id();
        Self {
            state,
            first_frame: false,
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            automation_runtime: crate::automation_runtime::NativeAutomationRuntime::discover(),
            automation_runtime_project_id,
            cloud_account: crate::services::cloud_account::CloudAccountService::unconfigured(),
            // A test opens no real pack store: doing so would make every test
            // read and write this developer's application-data directory.
            model_hub: crate::services::model_hub::ModelHubService::unavailable(
                "This test instance runs without a model-pack store.",
            ),
            live_session: crate::workbench::live_session::LiveSessionEngine::default(),
            file_workflow_io: Box::new(
                crate::workbench::workflows::file_workflow::NativeFileWorkflowIo,
            ),
            export_workflow_io: Box::new(
                crate::workbench::workflows::export_workflow::NativeExportWorkflowIo,
            ),
        }
    }

    /// Create a new application instance
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load persisted state if available
        let mut state: AppState = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            AppState::default()
        };

        let console_open = state
            .ui
            .preferences
            .workspace()
            .map(crate::workbench::WorkspacePreferences::console_on_launch)
            .unwrap_or_default()
            .is_open();
        state.workbench.apply_console_launch_behavior(console_open);
        state.workbench.window_session.normalize_after_restore();
        state.workspace.migrate_owned_netlist_deck_ids();

        state.initialize_shortcut_library_persistence(&cc.egui_ctx);
        state
            .ui
            .set_number_locale(crate::quantity::platform_number_locale());

        // Apply the design-system theme (canvas painters read the active
        // palette directly).
        state.ui.theme.apply(&cc.egui_ctx);

        // Persisted sessions may still carry the legacy seeded
        // "primitives" library (and tabs pointing into it); migrate any
        // drawn content to the user library and drop the placeholders.
        state.migrate_legacy_primitives();

        configure_platform_input_contract(&cc.egui_ctx);
        cc.egui_ctx.set_zoom_factor(1.0);

        // Restore global user Verilog-A library (commercial-style user library).
        restore_global_veriloga_library(&mut state.library_manager, &mut state.workspace);
        state.restore_active_schematic_from_workspace();
        crate::workbench::lifecycle::project_lifecycle::initialize_from_session(&mut state);
        #[cfg(target_arch = "wasm32")]
        initialize_browser_surface_navigation(&mut state, &cc.egui_ctx);
        #[cfg(target_arch = "wasm32")]
        pdk_persistence::initialize_browser_pdk_persistence(&cc.egui_ctx);
        // Started here and taken by a later frame. The store this reads into
        // is opened empty below, because a session that waited for storage to
        // answer would be a blank workspace for as long as it took.
        #[cfg(target_arch = "wasm32")]
        pack_persistence::initialize_browser_pack_restore(&cc.egui_ctx);

        // A license file on disk wins over (or backfills) the session copy.
        #[cfg(not(target_arch = "wasm32"))]
        if state.license.is_none()
            && let Some((key, info)) = crate::services::license::load_stored()
        {
            state.license_key = Some(key);
            state.license = Some(info);
        }

        // Load symbol library
        let symbol_library = match crate::schematic::symbols::SymbolLibrary::load_embedded() {
            Ok(lib) => {
                log::info!(
                    "Loaded {} default SVG component mappings from {} embedded SVG assets",
                    lib.len(),
                    lib.asset_count()
                );
                Some(lib)
            }
            Err(e) => {
                log::warn!("Failed to load SVG symbols, using procedural: {}", e);
                None
            }
        };

        // Log startup
        log::info!("RSpice egui application initialized");

        let automation_runtime_project_id = state.workspace.project.id();
        Self {
            state,
            first_frame: true,
            #[cfg(not(target_arch = "wasm32"))]
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library,
            simulation_controller: crate::simulation::SimulationController::new(),
            #[cfg(not(target_arch = "wasm32"))]
            automation_runtime: crate::automation_runtime::NativeAutomationRuntime::discover(),
            #[cfg(target_arch = "wasm32")]
            automation_runtime: crate::automation_runtime::BrowserAutomationRuntime::discover(),
            automation_runtime_project_id,
            cloud_account: crate::services::cloud_account::CloudAccountService::new(Some(
                cc.egui_ctx.clone(),
            )),
            // Opening sweeps whatever a killed install left behind. A failure
            // here is recorded as a reason and nothing else: a machine that
            // cannot store packs still edits, simulates, and saves.
            model_hub: crate::services::model_hub::ModelHubService::open(),
            live_session: crate::workbench::live_session::LiveSessionEngine::default(),
            file_workflow_io: Box::new(
                crate::workbench::workflows::file_workflow::NativeFileWorkflowIo,
            ),
            export_workflow_io: Box::new(
                crate::workbench::workflows::export_workflow::NativeExportWorkflowIo,
            ),
        }
    }

    fn prepare_frame(&mut self, ctx: &Context) {
        let initializing_frame = self.first_frame;
        let desired_zoom = self.state.ui.preferences.interface_scale();
        if (ctx.zoom_factor() - desired_zoom).abs() > f32::EPSILON {
            ctx.set_zoom_factor(desired_zoom);
        }
        // (Re)apply the theme when it changes — this maps the design tokens
        // onto the egui style and republishes the active palette.
        let system_mode_changed = self.state.ui.theme.mode == crate::ui::Mode::System
            && crate::ui::tokens::Tokens::get(ctx).mode != self.state.ui.theme.mode.effective(ctx);
        if self.first_frame
            || self.applied_theme != Some(self.state.ui.theme)
            || system_mode_changed
        {
            self.state.ui.theme.apply(ctx);
            self.applied_theme = Some(self.state.ui.theme);
            self.first_frame = false;
        }
        // The first egui pass establishes font metrics and persistent panel
        // rectangles. Request the settled pass explicitly; otherwise a WASM
        // deep link can remain provisional until the first user input,
        // leaving painter-owned title text absent while icons are visible.
        if initializing_frame {
            ctx.request_repaint();
        }

        self.terminate_automation_runtime_after_project_change();
        #[cfg(not(target_arch = "wasm32"))]
        self.automation_runtime.poll_discovery();
        // Both source imports record intent and complete from the frame loop.
        // Neither was driven from anywhere, so a requested import stayed
        // pending forever and blocked every later one.
        crate::workbench::documents::code_workspace::poll_veriloga_import(self);
        crate::workbench::documents::code_workspace::poll_automation_source_import(self);
        crate::workbench::documents::code_workspace::poll_automation_workflow(self);
        crate::workbench::documents::code_workspace::settle_pending_python_execute(self);
        crate::workbench::documents::code_workspace::poll_managed_automation_runtime(self);
        let automation = &self.state.ui.code_workspace.automation;
        if automation.pending_runtime_validation.is_some()
            || automation.runtime_execution.is_some()
            || automation.execution.is_active()
        {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }

        #[cfg(target_arch = "wasm32")]
        synchronize_browser_surface_navigation(&mut self.state);
        #[cfg(target_arch = "wasm32")]
        self.state.poll_shortcut_library_persistence();
        #[cfg(target_arch = "wasm32")]
        self.poll_browser_pdk_persistence(ctx);
        #[cfg(target_arch = "wasm32")]
        self.poll_browser_pack_restore(ctx);
        self.handle_shortcuts(ctx);
        #[cfg(target_arch = "wasm32")]
        crate::workbench::browser::file_import::register_text_import_repaint_context(ctx);
        #[cfg(target_arch = "wasm32")]
        crate::workbench::lifecycle::project_lifecycle::poll_browser_binding_restore(
            &mut self.state,
        );
        #[cfg(target_arch = "wasm32")]
        if crate::workbench::workflows::project_workflow::poll_browser_project_import(
            &mut self.state,
        ) {
            self.restore_workspace_after_project_load();
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(event) =
            crate::workbench::workflows::project_workflow::poll_browser_project_save(
                &mut self.state,
            )
        {
            self.handle_save_continuation_event(event);
        }
        #[cfg(target_arch = "wasm32")]
        if crate::workbench::workflows::file_workflow::poll_browser_schematic_import(
            &mut self.state,
        ) {
            self.state.clear_transient_specialized_viewer_data();
        }
        #[cfg(target_arch = "wasm32")]
        crate::workbench::workflows::netlist_workflow::poll_browser_netlist_workflow(
            &mut self.state,
        );
        #[cfg(target_arch = "wasm32")]
        crate::workbench::workflows::result_import_workflow::poll_browser_result_dataset_import(
            &mut self.state,
        );
        self.simulation_controller
            .update(&mut self.state, self.export_workflow_io.as_ref());
        #[cfg(target_arch = "wasm32")]
        crate::workbench::browser::accessibility::publish_workspace_context(
            self.state.workbench.workspace.label(),
            &self.state.workspace.active_view.display_path(),
            self.state.simulation.has_active_execution(),
        );
        if self.state.simulation.has_active_execution() {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
        if matches!(
            self.state.workspace.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        ) {
            self.state
                .workspace
                .set_active_dirty(self.state.schematic.is_dirty);
        }
        self.sync_window_title(ctx);
        self.handle_image_export();
    }

    /// Keep the OS window title (or browser tab title) in sync with the
    /// active document: `cell* — project — RSpice`.
    fn sync_window_title(&mut self, ctx: &Context) {
        let project_has_unsaved_changes =
            crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&self.state);
        let authoring_has_unsaved_changes = self.state.workbench.has_unsaved_authoring_changes();
        let should_warn_before_unload = should_warn_before_browser_unload(
            project_has_unsaved_changes,
            authoring_has_unsaved_changes,
            self.state.workbench.live_write_locks.mirror,
        );
        #[cfg(target_arch = "wasm32")]
        update_browser_before_unload_guard(should_warn_before_unload);
        #[cfg(not(target_arch = "wasm32"))]
        let _ = should_warn_before_unload;

        let dirty = if project_has_unsaved_changes || authoring_has_unsaved_changes {
            "*"
        } else {
            ""
        };
        let view = &self.state.workspace.active_view;
        let title = format!(
            "{}{dirty} — {} — RSpice",
            view.cell,
            self.state.workspace.project.display_name()
        );
        if self.last_window_title != title {
            #[cfg(not(target_arch = "wasm32"))]
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
            #[cfg(target_arch = "wasm32")]
            {
                let _ = ctx;
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    doc.set_title(&title);
                }
            }
            self.last_window_title = title;
        }
    }

    fn render_frame_chrome(&mut self, ui: &mut egui::Ui) {
        crate::workbench::show(ui, self);
    }

    fn capture_application_window_projection(
        &mut self,
        window: crate::workbench::ApplicationWindowId,
    ) {
        let workspace = self.state.workbench.workspace;
        let route = self.state.workbench.current_route();
        let layout = self.state.workbench.current_workspace_layout();
        let full_screen = self.state.workbench.full_screen;
        let active_document =
            crate::workbench::chrome::document_bar::active_document_id(&self.state);
        if let Some(window_state) = self.state.workbench.window_session.state_mut(window) {
            window_state.workspace = workspace;
            window_state.route = route;
            window_state.layout = layout;
            window_state.active_document = active_document;
            window_state.full_screen = full_screen;
        }
    }

    fn project_application_window(
        &mut self,
        window: crate::workbench::ApplicationWindowId,
        fallback_layout: crate::workbench::state::WorkspaceLayoutState,
    ) -> bool {
        let Some(window_state) = self.state.workbench.window_session.state(window).cloned() else {
            return false;
        };
        if self
            .state
            .workbench
            .window_session
            .set_current(window)
            .is_err()
        {
            return false;
        }
        let layout = if window_state.synchronize_chrome_with_primary {
            fallback_layout
        } else {
            window_state.layout
        };
        self.state.workbench.project_application_window(
            window_state.workspace,
            window_state.route,
            layout,
        );
        self.state.workbench.full_screen = window_state.full_screen;
        if let Some(document) = window_state.active_document {
            let _ = crate::workbench::chrome::document_bar::activate_document_by_id(
                &mut self.state,
                &document,
            );
        }
        true
    }

    fn render_secondary_application_windows(&mut self, ctx: &Context) {
        let primary = self.state.workbench.window_session.primary();
        self.capture_application_window_projection(primary);
        let primary_state = self
            .state
            .workbench
            .window_session
            .state(primary)
            .cloned()
            .unwrap_or_default();
        let secondary = self.state.workbench.window_session.secondary_window_ids();
        let mut close_windows = Vec::new();

        for (cascade_index, window) in secondary.into_iter().enumerate() {
            self.state.sync_active_schematic_to_workspace();
            if !self.project_application_window(window, primary_state.layout) {
                continue;
            }
            let Some(presentation) = self.state.workbench.window_session.state(window).cloned()
            else {
                continue;
            };
            let render_plan = presentation.render_plan(cascade_index);
            let mut builder = egui::ViewportBuilder::default()
                .with_title(render_plan.title.clone())
                .with_inner_size(render_plan.inner_size)
                .with_clamp_size_to_monitor_size(true);
            if let Some(position) = render_plan.position {
                builder = builder.with_position(position);
            }

            let title = presentation.title.clone();
            let (viewport_class, viewport_info, reattach_requested) =
                ctx.show_viewport_immediate(window.viewport_id(), builder, |ui, class| {
                    let reattach_requested = if class == egui::ViewportClass::EmbeddedWindow {
                        crate::workbench::show_embedded_secondary(ui, self, &title)
                    } else {
                        crate::workbench::show_secondary(ui, self);
                        false
                    };
                    let viewport_info = ui.ctx().input(|input| input.viewport().clone());
                    (class, viewport_info, reattach_requested)
                });

            self.state.sync_active_schematic_to_workspace();
            self.capture_application_window_projection(window);
            if viewport_class != egui::ViewportClass::EmbeddedWindow
                && let Some(window_state) = self.state.workbench.window_session.state_mut(window)
            {
                window_state.bounds.observe(&viewport_info);
                if render_plan.recovering {
                    window_state.bounds.recovery_pending = false;
                }
            }
            if (viewport_class != egui::ViewportClass::EmbeddedWindow
                && viewport_info.close_requested())
                || reattach_requested
            {
                close_windows.push(window);
            }
            let _ = self.project_application_window(primary, primary_state.layout);
        }

        self.state.sync_active_schematic_to_workspace();
        let _ = self.project_application_window(primary, primary_state.layout);
        for window in close_windows {
            let _ = self.state.workbench.window_session.close_window(window);
        }
    }

    fn render_frame_dialogs(&mut self, ctx: &Context) {
        self.synchronize_design_management_route();
        self.render_confirmation_dialog(ctx);
        dialogs::property_dialog::render_property_dialog(ctx, &mut self.state);
        self.process_pdk_settings_dialog(ctx);
        self.render_preferences_dialog(ctx);
        self.render_license_dialog(ctx);
        self.render_new_project_dialog(ctx);
        self.render_publish_web_dialog(ctx);
        self.render_live_session_dialog(ctx);
        self.render_unpublish_web_dialog(ctx);
        self.render_command_palette(ctx);
        self.render_bus_tap_dialog(ctx);
        self.render_builtin_xspice_placement_dialog(ctx);
        self.render_net_label_dialog(ctx);
        self.render_grid_snap_routing_dialog(ctx);
        self.render_drawing_sheet_layers_dialog(ctx);
        self.render_drawing_sheet_overflow_dialog(ctx);
        self.render_schematic_visibility_dialog(ctx);
        self.render_descend_hierarchy_dialog(ctx);
        self.render_engineering_table_dialog(ctx);
        self.render_pin_port_dialog(ctx);
        self.render_design_note_dialog(ctx);
        self.render_documentation_shape_dialog(ctx);
        self.render_selection_workflow_dialog(ctx);
        self.render_move_selection_dialog(ctx);
        self.render_stretch_selection_dialog(ctx);
        self.render_array_selection_dialog(ctx);
        self.render_replace_instance_dialog(ctx);
        self.render_create_hierarchy_dialog(ctx);
        self.render_check_and_save_dialog(ctx);
        self.render_connectivity_manager_dialog(ctx);
        self.render_design_management_dialog(ctx);
        self.render_selection_bulk_edit_dialog(ctx);
        self.render_design_review_comments_dialog(ctx);
        self.render_project_revision_history_dialog(ctx);
        self.render_source_document_dialog(ctx);
        self.render_source_find_dialog(ctx);
        self.render_source_language_tools_dialog(ctx);
        self.render_create_model_bound_symbol_dialog(ctx);
        self.render_symbol_definition_dialogs(ctx);
        self.render_configuration_sets_dialog(ctx);
        self.render_drawing_sheet_setup_dialog(ctx);
        self.render_drawing_sheet_defaults_dialog(ctx);
        self.render_drawing_sheet_preset_dialogs(ctx);
        self.render_drawing_sheet_support_dialogs(ctx);
        self.render_hardcopy_dialog(ctx);
        self.render_object_properties_dialog(ctx);
        self.render_rename_selection_dialog(ctx);
        self.render_view_operation_dialog(ctx);
        self.render_window_session_dialog(ctx);
        self.render_help_center_dialog(ctx);
        self.render_about_dialog(ctx);
        crate::workbench::workflows::result_import_workflow::show_result_import_dialog(
            ctx,
            &mut self.state,
        );
        // The calculator is a modeless tool, not a modal: it renders beside
        // the dialog stack so the workspace stays live beneath it.
        crate::workbench::tools::calculator_tool::show(ctx, self);
        crate::workbench::tools::expression_diagnostics::show(ctx, self);
        self.render_shortcuts_help_dialog(ctx);
        self.process_model_browser_dialog(ctx);
        self.process_new_cell_dialog(ctx);
        self.process_new_view_dialog(ctx);
        self.process_copy_cell_dialog(ctx);
        self.process_rename_cell_dialog(ctx);
        self.process_library_deletion_review_dialog(ctx);
        self.process_analysis_removal_review_dialog(ctx);
        #[cfg(not(target_arch = "wasm32"))]
        self.process_autosave_restore_dialog(ctx);
        self.process_pending_library_deletions();
        self.process_exit_request(ctx);
        crate::workbench::show_route_overlays(ctx, self);
    }

    /// Keep browser-restored manager routes and the transient egui dialog
    /// owner synchronized. Commands open both in one transaction; this hook
    /// covers deep links, Back/Forward, and restored application sessions.
    fn synchronize_design_management_route(&mut self) {
        let route_active = self.state.workbench.current_route().surface_id()
            == crate::workbench::SurfaceId::DesignManagement;
        if route_active && !self.state.dialogs.design_management.open {
            open_design_management_dialog(&mut self.state);
        } else if !route_active && self.state.dialogs.design_management.open {
            self.state.dialogs.design_management.close_and_discard();
        }
    }
}

/// Close the route-owned Design Management manager without stranding a
/// `?surface=design-management` URL. Ordinary opens traverse back to their
/// exact Design source; direct deep links fail safely to the canonical Design
/// workspace using a replacement history entry.
pub(crate) fn close_design_management_dialog_route(state: &mut AppState) {
    if state.workbench.current_route().surface_id() != crate::workbench::SurfaceId::DesignManagement
    {
        return;
    }
    if state
        .workbench
        .navigate_back(crate::workbench::RouteTransitionSource::User)
        .is_some()
    {
        return;
    }
    let fallback = crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::Design);
    if let Err(error) = state
        .workbench
        .replace_route(fallback, crate::workbench::RouteTransitionSource::User)
    {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Could not close Design Management: {error}"
        )));
    }
}

#[cfg(target_arch = "wasm32")]
fn initialize_browser_surface_navigation(state: &mut AppState, ctx: &Context) {
    use crate::workbench::RouteTransitionSource;
    use crate::workbench::browser::navigation::{
        current_location, install_popstate_listener, restart_history_session,
    };

    match current_location() {
        Ok(location) => {
            if let Some(route) = location.route()
                && let Err(error) = state
                    .workbench
                    .navigate(route, RouteTransitionSource::BrowserPop)
            {
                state.push_user_message(ConsoleMessage::warning(format!(
                    "The requested deep link was not opened: {error}"
                )));
            }
        }
        Err(error) => state.push_user_message(ConsoleMessage::warning(format!(
            "The browser deep link is malformed and was rejected; the address was recovered to the active task: {error}"
        ))),
    }

    // A restored native-style stack is not browser traversal authority. The
    // new process starts with exactly its active route and takes ownership of
    // the current host entry through one canonical replaceState transaction.
    state
        .workbench
        .reset_navigation_history_for_fresh_browser_session();
    let listener_ready = match install_popstate_listener(ctx) {
        Ok(()) => true,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Browser route synchronization is disabled and listener installation will retry automatically: {error}"
            )));
            false
        }
    };
    if listener_ready && let Err(error) = restart_history_session(state.workbench.current_route()) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Browser route synchronization is disabled and will retry automatically; history could not be initialized at the active task: {error}"
        )));
    }
}

#[cfg(target_arch = "wasm32")]
fn synchronize_browser_surface_navigation(state: &mut AppState) {
    use crate::workbench::browser::navigation::{
        ensure_popstate_listener, history_session_ready, poll_popstate, push_route, replace_route,
        traversal_in_flight, traversal_watchdog_expired, traverse_history,
    };
    use crate::workbench::{BrowserHistoryEffect, RouteTransitionSource};

    while let Some(event) = poll_popstate() {
        match event {
            Ok(event) => {
                let route = event.route();
                let availability = crate::workbench::route_availability(route);
                let pending_effects = state.workbench.has_pending_browser_history_effects();
                let external_race = !event.initiated_by_app() && pending_effects;
                let applied = if !availability.can_open() {
                    Err(format!(
                        "surface `{}` is unavailable: {}",
                        route.surface_id(),
                        availability
                            .reason()
                            .unwrap_or("no complete route executor is registered")
                    ))
                } else if external_race {
                    Err(
                        "an external browser traversal raced with newer in-app navigation"
                            .to_owned(),
                    )
                } else if event.initiated_by_app() {
                    (state.workbench.current_route() == route || pending_effects)
                        .then_some(())
                        .ok_or_else(|| {
                            "the in-app traversal did not reach its authenticated browser entry"
                                .to_owned()
                        })
                } else {
                    apply_authenticated_browser_pop(state, event.delta(), route)
                };

                if let Err(error) = applied {
                    state.push_user_message(ConsoleMessage::warning(format!(
                        "Browser history could not restore the requested task and was recovered at a canonical route: {error}"
                    )));
                    if availability.can_open() && !external_race {
                        // The browser entry is authenticated and available;
                        // make it authoritative, then start a bounded session
                        // rather than guessing from stale in-app stacks.
                        if state.workbench.current_route() != route {
                            let _ = state
                                .workbench
                                .navigate(route, RouteTransitionSource::BrowserPop);
                        }
                    }
                    recover_browser_history_at_active_task(state);
                    return;
                }
            }
            Err(error) => {
                state.push_user_message(ConsoleMessage::warning(format!(
                    "An unauthenticated or malformed browser history entry was rejected and canonicalized to the active task: {error}"
                )));
                recover_browser_history_at_active_task(state);
                return;
            }
        }
    }

    if state
        .workbench
        .take_browser_history_effect_queue_overflowed()
    {
        state.push_user_message(ConsoleMessage::warning(
            "Browser synchronization commands exceeded the bounded queue; the address was recovered directly to the active task.",
        ));
        recover_browser_history_at_active_task(state);
        return;
    }

    if traversal_watchdog_expired() {
        state.push_user_message(ConsoleMessage::warning(
            "Browser traversal did not produce an authenticated history event before the watchdog deadline; the address was recovered to the active task.",
        ));
        recover_browser_history_at_active_task(state);
        return;
    }

    if !history_session_ready() {
        state.workbench.clear_browser_history_effects();
        if ensure_popstate_listener().is_err() {
            return;
        }
        let canonical = state.workbench.current_route();
        match crate::workbench::browser::navigation::restart_history_session(canonical) {
            Ok(()) => {
                state
                    .workbench
                    .reset_navigation_history_for_fresh_browser_session();
                state.push_user_message(ConsoleMessage::info(
                    "Browser route synchronization resumed at the active task; browser traversal history was restarted.",
                ));
            }
            Err(_) => return,
        }
    }

    // `history.go` is asynchronous. No later effect may observe or mutate the
    // browser ledger until its exact destination is authenticated by popstate.
    if traversal_in_flight() {
        return;
    }

    while let Some(effect) = state.workbench.take_browser_history_effect() {
        let (result, traversal_started) = match effect {
            BrowserHistoryEffect::Push(route) => (push_route(route).map(|_| ()), false),
            BrowserHistoryEffect::Replace(route) => (replace_route(route).map(|_| ()), false),
            BrowserHistoryEffect::Traverse { delta, destination } => {
                (traverse_history(delta, destination), true)
            }
        };
        if let Err(error) = result {
            rollback_browser_navigation_after_sync_failure(state, error);
            return;
        }
        if traversal_started {
            return;
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn recover_browser_history_at_active_task(state: &mut AppState) {
    use crate::workbench::browser::navigation::{
        ensure_popstate_listener, restart_history_session,
    };

    state.workbench.clear_browser_history_effects();
    let canonical = state.workbench.current_route();
    if let Err(error) = ensure_popstate_listener() {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Browser route synchronization is disabled and listener installation will retry automatically: {error}"
        )));
        return;
    }
    match restart_history_session(canonical) {
        Ok(()) => state
            .workbench
            .reset_navigation_history_for_fresh_browser_session(),
        Err(error) => state.push_user_message(ConsoleMessage::warning(format!(
            "Browser route synchronization is disabled and will retry automatically; the address could not be recovered to the active task: {error}"
        ))),
    }
}

#[cfg(target_arch = "wasm32")]
fn rollback_browser_navigation_after_sync_failure(
    state: &mut AppState,
    error: crate::workbench::browser::navigation::BrowserNavigationError,
) {
    use crate::workbench::RouteTransitionSource;
    use crate::workbench::browser::navigation::{active_browser_route, restart_history_session};

    match active_browser_route() {
        Ok(browser_route) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "The browser address could not be synchronized; the active task was rolled back to the last committed browser route: {error}"
            )));
            if state.workbench.current_route() != browser_route {
                let _ = state
                    .workbench
                    .navigate(browser_route, RouteTransitionSource::BrowserPop);
            }
            state.workbench.clear_browser_history_effects();
            state
                .workbench
                .reset_navigation_history_for_fresh_browser_session();
            if let Err(recovery_error) = restart_history_session(browser_route) {
                state.push_user_message(ConsoleMessage::warning(format!(
                    "Browser route synchronization is disabled and will retry automatically; the history session could not be restarted after rollback: {recovery_error}"
                )));
            }
        }
        Err(_) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "The browser address could not be synchronized and no committed browser session was available for rollback: {error}"
            )));
            recover_browser_history_at_active_task(state);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_authenticated_browser_pop(
    state: &mut AppState,
    delta: i32,
    route: crate::workbench::SurfaceRoute,
) -> Result<(), String> {
    use crate::workbench::RouteTransitionSource;

    if delta < 0 {
        let steps = usize::try_from(delta.unsigned_abs())
            .map_err(|_| "browser back distance is not representable".to_owned())?;
        if steps > state.workbench.back_route_count() {
            return Err("browser back entry is outside the in-app task stack".to_owned());
        }
        state
            .workbench
            .navigate_back_steps(steps, RouteTransitionSource::BrowserPop)
            .ok_or_else(|| "browser back entry is outside the in-app task stack".to_owned())?;
    } else if delta > 0 {
        let steps = usize::try_from(delta)
            .map_err(|_| "browser forward distance is not representable".to_owned())?;
        if steps > state.workbench.forward_route_count() {
            return Err("browser forward entry is outside the in-app task stack".to_owned());
        }
        state
            .workbench
            .navigate_forward_steps(steps, RouteTransitionSource::BrowserPop)
            .ok_or_else(|| "browser forward entry is outside the in-app task stack".to_owned())?;
    }

    if state.workbench.current_route() == route {
        Ok(())
    } else {
        Err("authenticated browser entry and in-app task stack disagree".to_owned())
    }
}

impl eframe::App for RSpiceApp {
    /// Called on each frame
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let simulation_input_before_frame = FrameSimulationInput::capture(&self.state);
        crate::ui::viewport::capture_root_viewport(&ctx, ui.max_rect());
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(enabled) =
                crate::workbench::browser::accessibility::spoken_feedback_override()
            {
                self.state.ui.browser_spoken_feedback = enabled;
            }
            ctx.options_mut(|options| {
                options.screen_reader = self.state.ui.browser_spoken_feedback;
            });
        }
        self.prepare_frame(&ctx);
        self.cloud_account.poll();
        if let Some(error) = self.cloud_account.take_local_error() {
            self.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(error));
        }
        if let Some(url) = self.cloud_account.take_browser_request() {
            #[cfg(not(target_arch = "wasm32"))]
            ctx.open_url(egui::OpenUrl::new_tab(url));
            #[cfg(target_arch = "wasm32")]
            if web_sys::window()
                .and_then(|window| window.location().assign(&url).ok())
                .is_none()
            {
                log::error!("browser sign-in navigation was rejected");
            }
        }
        self.record_publish_receipt();
        self.live_session
            .pump(&mut self.state, &mut self.cloud_account);
        if let Some(text) = self.state.ui.clipboard_text_request.take() {
            ctx.copy_text(text);
        }
        #[cfg(not(target_arch = "wasm32"))]
        self.autosave_tick(&ctx);
        self.render_frame_chrome(ui);
        self.render_secondary_application_windows(&ctx);
        self.refresh_incremental_connectivity_checks();
        self.render_frame_dialogs(&ctx);
        self.invalidate_simulation_preflight_after_frame_edit(&simulation_input_before_frame);
        crate::workbench::synchronize_schematic_cross_probe(&mut self.state);
        self.state.workbench.capture_authoring_recovery();
        // A project replacement can commit from a dialog after prepare_frame.
        // Terminate at the same frame boundary so old authority cannot survive.
        self.terminate_automation_runtime_after_project_change();
    }

    /// Called by eframe when the application is shutting down.
    fn on_exit(&mut self) {
        let _ = self.automation_runtime.terminate();
        log::info!("eframe on_exit — application shutting down");
        #[cfg(target_arch = "wasm32")]
        if let Err(error) = crate::workbench::browser::navigation::uninstall_popstate_listener() {
            log::warn!("Failed to remove browser route listener: {error}");
        }
    }

    /// Save state on exit
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // A live mirror is streamed session data, not device-owned recovery
        // state. Never let eframe autosave, browser local storage, or the
        // global Verilog-A library silently retain it. The previously stored
        // local session stays intact; after the live session ends, normal
        // persistence resumes only if policy allowed the mirror to remain.
        if !should_persist_application_state(&self.state) {
            log::debug!("save: skipped while a live-session mirror is active");
            return;
        }
        log::debug!("save: sync schematic");
        self.state.sync_active_schematic_to_workspace();
        self.state.workbench.capture_authoring_recovery();
        log::debug!("save: veriloga library");
        if let Err(err) =
            save_global_veriloga_library(&self.state.library_manager, &self.state.workspace)
        {
            log::warn!(
                "Failed to persist global Verilog-A library during app save: {}",
                err
            );
        }
        log::debug!("save: serialize app state");
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
        log::debug!("save: done");
    }

    /// Persist the complete recoverable working session on the same cadence
    /// selected in Preferences. Native builds additionally publish their
    /// path-bound exact-byte checkpoint; browser/mobile builds use eframe's
    /// durable application storage as their device-local recovery owner.
    fn auto_save_interval(&self) -> std::time::Duration {
        configured_autosave_interval(self.state.ui.autosave_minutes)
    }
}

impl RSpiceApp {
    fn process_exit_request(&mut self, ctx: &Context) {
        if !self.state.exit_requested {
            return;
        }

        log::info!("application exit requested — closing viewport");
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        self.state.exit_requested = false;
    }

    /// Toggle the console panel between expanded and collapsed.
    pub fn toggle_console(&mut self) {
        self.state.workbench.console_visible = !self.state.workbench.console_visible;
    }
}

fn should_warn_before_browser_unload(
    schematic_dirty: bool,
    workspace_dirty: bool,
    volatile_live_mirror: bool,
) -> bool {
    schematic_dirty || workspace_dirty || volatile_live_mirror
}

fn should_persist_application_state(state: &AppState) -> bool {
    !state.workbench.live_write_locks.mirror
}

fn configured_autosave_interval(minutes: u8) -> std::time::Duration {
    // Deserialization normalizes legacy values to 2/5/10. Keep this boundary
    // defensive so a malformed in-memory integration can never request an
    // every-frame persistence loop.
    std::time::Duration::from_secs(u64::from(minutes.max(1)) * 60)
}

#[cfg(target_arch = "wasm32")]
fn update_browser_before_unload_guard(has_unsaved_changes: bool) {
    use wasm_bindgen::JsCast as _;

    BROWSER_UNLOAD_DIRTY.with(|dirty| dirty.set(has_unsaved_changes));
    BROWSER_UNLOAD_LISTENER.with(|listener| {
        if listener.borrow().is_some() {
            return;
        }

        let Some(window) = web_sys::window() else {
            return;
        };
        let callback = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::BeforeUnloadEvent)>::new(
            |event: web_sys::BeforeUnloadEvent| {
                let dirty = BROWSER_UNLOAD_DIRTY.with(|state| state.get());
                if dirty {
                    event.prevent_default();
                    event.set_return_value(BROWSER_UNLOAD_WARNING);
                }
            },
        );
        if window
            .add_event_listener_with_callback("beforeunload", callback.as_ref().unchecked_ref())
            .is_ok()
        {
            *listener.borrow_mut() = Some(callback);
        }
    });
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProcessCorner;
    use crate::services::drc::{
        DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType,
    };
    use crate::state::{
        CellViewRef, ComponentType, ConfigurationBlackBoxPolicy, ConfigurationModelProfile,
        ConfigurationSetCatalog, ConfigurationSetDefinition, Point, SchematicState,
        UnresolvedBindingPolicy,
    };

    fn runnable_state() -> AppState {
        let mut state = AppState::default();
        // These readiness tests isolate schematic/DRC policy. Project-owned
        // behavioral sources have their own compile-before-run contract.
        state.workspace.project_sources = Default::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        assert!(
            state
                .sim_setup
                .stable_analysis_plan()
                .is_ok_and(|plan| plan.instances().iter().any(|instance| instance.enabled())),
            "default plan should contain a runnable analysis instance"
        );
        state
    }

    fn schematic_readiness(state: &AppState) -> Option<String> {
        crate::workbench::app_state::run_preflight::run_preflight_block_reason(
            &state.sim_setup,
            &state.workspace,
            &state.schematic,
            &state.model_library_manager,
            state.current_blocking_drc_result(),
        )
    }

    fn drc_result(violation_type: DrcViolationType, severity: Option<DrcSeverity>) -> DrcResult {
        let mut result = DrcResult::new();
        let mut violation = DrcViolation::new(
            1,
            violation_type,
            violation_type.description(),
            DrcLocation::Global,
        );
        if let Some(severity) = severity {
            violation = violation.with_severity(severity);
        }
        result.add_violation(violation);
        result.completed = true;
        result
    }

    fn publish_drc_result(state: &mut AppState, result: DrcResult) {
        state
            .publish_active_design_check_result(result)
            .expect("test DRC receipt must publish");
    }

    fn configure_alternate_simulation_root(
        state: &mut AppState,
        root: CellViewRef,
        schematic: SchematicState,
    ) {
        state
            .workspace
            .schematic_buffers
            .insert(root.key(), schematic);
        let mut catalog = ConfigurationSetCatalog::default();
        catalog
            .create(ConfigurationSetDefinition {
                name: "Configured run root".to_owned(),
                root,
                dut_path: "/top".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Local project".to_owned(),
            })
            .expect("valid alternate configuration");
        state
            .workspace
            .replace_configuration_sets(catalog)
            .expect("materialized alternate root publishes");
    }

    #[test]
    fn platform_input_contract_matches_context_gesture_mockup() {
        let ctx = Context::default();

        configure_platform_input_contract(&ctx);

        ctx.options(|options| {
            assert_eq!(options.input_options.max_click_duration, 0.56);
            assert_eq!(options.input_options.max_click_dist, 9.0);
            assert!(!options.zoom_with_keyboard);
        });
    }

    #[test]
    fn frame_topology_edits_drop_preflight_authority_at_one_central_boundary() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.preflight.open = true;
        app.state.workbench.preflight.pending_toast =
            Some(crate::workbench::state::PreflightToast {
                message: "prepared".to_owned(),
                warning: false,
            });
        let before = FrameSimulationInput::capture(&app.state);

        app.state.schematic.bump_topology_version();
        app.invalidate_simulation_preflight_after_frame_edit(&before);

        assert!(!app.state.workbench.preflight.open);
        assert!(app.state.workbench.preflight.report.is_none());
        assert!(app.state.workbench.preflight.pending_toast.is_none());
    }

    #[test]
    fn run_readiness_blocks_current_drc_errors_for_generated_schematic_runs() {
        let mut state = runnable_state();
        publish_drc_result(
            &mut state,
            drc_result(DrcViolationType::MissingGround, None),
        );

        assert!(
            !state.can_run_simulation(),
            "current error-level DRC results must block schematic simulation"
        );
    }

    #[test]
    fn legacy_drc_projection_is_never_execution_authority() {
        let mut state = runnable_state();
        state.dialogs.drc_results = Some(drc_result(DrcViolationType::MissingGround, None));
        state.dialogs.drc_checked_version = state.schematic.topology_version();

        assert!(
            schematic_readiness(&state).is_none(),
            "a presentation-only result without an exact cell/view receipt must not block"
        );
    }

    #[test]
    fn run_readiness_allows_stale_or_warning_only_drc_results() {
        let mut state = runnable_state();
        publish_drc_result(
            &mut state,
            drc_result(DrcViolationType::MissingGround, None),
        );
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(20, 20));
        assert!(
            schematic_readiness(&state).is_none(),
            "stale DRC results should not block after schematic edits"
        );

        publish_drc_result(
            &mut state,
            drc_result(DrcViolationType::UnconnectedPin, Some(DrcSeverity::Warning)),
        );
        assert!(
            schematic_readiness(&state).is_none(),
            "warning-only DRC results should not block simulation"
        );
    }

    #[test]
    fn run_readiness_uses_the_configured_root_not_the_active_editor() {
        let mut state = runnable_state();
        let root = CellViewRef::new("user", "configured", "schematic");
        let mut configured = SchematicState::default();
        configured.add_component(ComponentType::Resistor, Point::new(40, 20));
        configure_alternate_simulation_root(&mut state, root, configured);
        publish_drc_result(
            &mut state,
            drc_result(DrcViolationType::MissingGround, None),
        );

        assert!(
            schematic_readiness(&state).is_none(),
            "an unrelated active-editor DRC receipt must not block the configured root"
        );
    }

    #[test]
    fn run_readiness_rejects_an_empty_configured_root_even_when_the_editor_is_populated() {
        let mut state = runnable_state();
        let root = CellViewRef::new("user", "configured", "schematic");
        configure_alternate_simulation_root(&mut state, root, SchematicState::default());

        let reason = state
            .simulation_run_preflight_block_reason()
            .expect("empty configured root must block");
        assert!(reason.contains("configured simulation root"));
        assert!(reason.contains("user/configured/schematic"));
    }

    #[test]
    fn unreferenced_code_workspace_veriloga_never_blocks_schematic_run_readiness() {
        let mut state = runnable_state();
        state
            .workspace
            .project_sources
            .insert(
                crate::state::ProjectSourceDocument::try_new(
                    "unreferenced.va",
                    crate::state::ProjectSourceLanguage::VerilogA,
                    "module unreferenced; endmodule\n",
                )
                .expect("valid unreferenced source"),
            )
            .expect("unique Code Workspace source");
        assert!(
            state
                .workspace
                .project_sources
                .get(crate::state::ProjectSourceLanguage::VerilogA)
                .is_some()
        );
        state.ui.code_workspace.veriloga.receipt = None;

        assert!(
            schematic_readiness(&state).is_none(),
            "only an executable deck that references project Verilog-A may require its runtime"
        );
    }

    #[test]
    fn run_readiness_demands_technology_only_when_the_plan_requires_it() {
        let mut state = runnable_state();
        assert_eq!(
            state.simulation_run_preflight_block_reason(),
            None,
            "a typical plan runs without an attached project technology"
        );
        assert!(state.can_run_simulation());

        state
            .sim_setup
            .set_reference_pvt(ProcessCorner::SS, 27.0)
            .expect("the reference point is valid");

        let reason = state
            .simulation_run_preflight_block_reason()
            .expect("a non-typical reference section demands an attached technology");
        assert!(
            reason.contains("requires an attached project technology"),
            "{reason}"
        );
        assert!(reason.contains("SS"), "{reason}");
        assert!(!state.can_run_simulation());
    }

    #[test]
    fn run_readiness_rejects_a_technology_binding_without_checkpoint_receipts() {
        let mut state = runnable_state();
        state.provision_test_project_technology_contract();
        // A project copy retains the exact binding and starts a fresh audit
        // history, which is precisely the binding-without-receipts state.
        state.workspace.project = state
            .workspace
            .project
            .fork_copy_at(std::path::PathBuf::from("run_readiness_fixture.rspice"));

        let reason = state
            .simulation_run_preflight_block_reason()
            .expect("an unaudited binding must block");
        assert!(
            reason.contains("predates checkpoint-backed authority receipts"),
            "{reason}"
        );
        assert!(!state.can_run_simulation());
    }

    #[test]
    fn run_readiness_still_enforces_an_attached_technology_contract() {
        let mut state = runnable_state();
        state.provision_test_project_technology_contract();
        assert_eq!(
            state.simulation_run_preflight_block_reason(),
            None,
            "an exact attached contract runs"
        );

        let attached_library = state
            .workspace
            .project
            .technology_binding()
            .expect("the provisioned project owns an exact binding")
            .model_library()
            .to_owned();
        state
            .model_library_manager
            .remove_library(&attached_library);

        let reason = state
            .simulation_run_preflight_block_reason()
            .expect("an attached contract that no longer resolves must block");
        assert!(reason.contains("was removed"), "{reason}");
    }

    #[test]
    fn generated_and_manual_runs_have_separate_drc_readiness() {
        let mut state = runnable_state();
        state.workspace.netlist_source = Some("V1 in 0 1\nR1 in 0 1k\n.end\n".to_string());
        publish_drc_result(
            &mut state,
            drc_result(DrcViolationType::MissingGround, None),
        );

        assert!(
            !state.can_run_simulation(),
            "schematic Simulate must still be blocked by current schematic DRC"
        );
        let manual_reason = state
            .manual_deck_run_block_reason()
            .expect("an unvalidated manual deck must remain blocked");
        assert!(manual_reason.contains("Validate"));
        assert!(
            !manual_reason.contains("DRC"),
            "manual deck readiness must remain independent of schematic DRC"
        );
    }

    #[test]
    fn netlist_manual_run_request_queues_when_engine_is_busy() {
        let mut state = AppState::default();
        let identity = state
            .simulation
            .start_run()
            .execution_identity()
            .expect("current run has execution identity");
        state.simulation.active_execution = Some(identity);
        state.simulation.is_running = false;
        state.request_netlist_manual_deck_run();

        assert_eq!(
            state.simulation.run_intent,
            crate::state::SimulationRunIntent::ManualDeck
        );
        assert!(!state.simulation.trigger_simulation);
        assert!(state.ui.netlist.rerun_queued);

        state.simulation.active_execution = None;
        state.ui.netlist.rerun_queued = false;
        state.request_netlist_manual_deck_run();

        assert!(state.simulation.trigger_simulation);
        assert!(!state.ui.netlist.rerun_queued);
    }

    #[test]
    fn browser_unload_warning_tracks_schematic_or_workspace_dirty_state() {
        assert!(!should_warn_before_browser_unload(false, false, false));
        assert!(should_warn_before_browser_unload(true, false, false));
        assert!(should_warn_before_browser_unload(false, true, false));
        assert!(should_warn_before_browser_unload(true, true, false));
        assert!(should_warn_before_browser_unload(false, false, true));
    }

    #[test]
    fn preference_autosave_cadence_drives_platform_session_persistence() {
        assert_eq!(
            configured_autosave_interval(2),
            std::time::Duration::from_secs(120)
        );
        assert_eq!(
            configured_autosave_interval(5),
            std::time::Duration::from_secs(300)
        );
        assert_eq!(
            configured_autosave_interval(10),
            std::time::Duration::from_secs(600)
        );
        assert_eq!(
            configured_autosave_interval(0),
            std::time::Duration::from_secs(60)
        );
    }

    #[test]
    fn live_mirror_never_enters_application_persistence() {
        let mut state = AppState::default();
        assert!(should_persist_application_state(&state));

        state.workbench.live_write_locks.mirror = true;
        state.workbench.live_write_locks.mirror_save_copy_allowed = true;
        assert!(!should_persist_application_state(&state));

        state.workbench.live_write_locks.mirror_save_copy_allowed = false;
        assert!(!should_persist_application_state(&state));
    }

    #[test]
    fn application_modal_gate_covers_every_app_level_modal_owner() {
        let mut state = AppState::default();
        assert!(!state.application_modal_open());

        state.dialogs.preferences_open = true;
        assert!(state.application_modal_open());
        state.dialogs.preferences_open = false;

        state.sim_setup.palette_open = true;
        assert!(state.application_modal_open());
        state.sim_setup.palette_open = false;

        state.tabbed_property_dialog.open = true;
        assert!(state.application_modal_open());
        state.tabbed_property_dialog.open = false;

        state.pdk_settings_dialog.open = true;
        assert!(state.application_modal_open());
        state.pdk_settings_dialog.open = false;

        state.model_browser_state.open = true;
        assert!(state.application_modal_open());
        state.model_browser_state.open = false;

        state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
        state.ui.netlist.find.open = true;
        assert!(state.application_modal_open());
        state.ui.netlist.find.open = false;

        state.ui.netlist.ownership_dialog.open = true;
        assert!(state.application_modal_open());
        state.ui.netlist.ownership_dialog.open = false;

        state.ui.netlist.comparison_dialog.open = true;
        assert!(state.application_modal_open());
        state.ui.netlist.comparison_dialog.open = false;

        state.ui.netlist.save_dialog.open = true;
        assert!(state.application_modal_open());
        state.ui.netlist.save_dialog.open = false;

        state.ui.netlist.export_dialog.open = true;
        assert!(state.application_modal_open());
        state.ui.netlist.export_dialog.open = false;

        state.workbench.open_project_launcher();
        assert!(state.application_modal_open());
    }

    #[test]
    fn design_management_route_hook_opens_and_dismisses_the_transient_dialog_owner() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workbench
            .replace_route(
                crate::workbench::SurfaceRoute::surface(
                    crate::workbench::SurfaceId::DesignManagement,
                ),
                crate::workbench::RouteTransitionSource::Restore,
            )
            .expect("Design Management route is executable");
        assert!(!app.state.dialogs.design_management.open);

        app.synchronize_design_management_route();
        assert!(app.state.dialogs.design_management.open);

        app.state
            .workbench
            .replace_route(
                crate::workbench::SurfaceRoute::surface(crate::workbench::SurfaceId::Design),
                crate::workbench::RouteTransitionSource::BrowserPop,
            )
            .expect("Design workspace route is executable");
        app.synchronize_design_management_route();
        assert!(!app.state.dialogs.design_management.open);
    }

    #[test]
    fn design_management_close_restores_design_with_canonical_history_effects() {
        use crate::workbench::{
            BrowserHistoryEffect, RouteTransitionSource, SurfaceId, SurfaceRoute,
        };

        let mut state = AppState::default();
        let design = SurfaceRoute::surface(SurfaceId::Design);
        let manager = SurfaceRoute::surface(SurfaceId::DesignManagement);
        state
            .workbench
            .navigate(manager, RouteTransitionSource::User)
            .expect("Design Management route is executable");
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Push(manager))
        );

        close_design_management_dialog_route(&mut state);
        assert_eq!(state.workbench.current_route(), design);
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Traverse {
                delta: -1,
                destination: design,
            })
        );

        state
            .workbench
            .replace_route(manager, RouteTransitionSource::Restore)
            .expect("direct manager deep link restores");
        let _ = state.workbench.take_browser_history_effect();
        close_design_management_dialog_route(&mut state);
        assert_eq!(state.workbench.current_route(), design);
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Replace(design))
        );
    }

    #[test]
    fn application_state_round_trips_through_eframe_ron_storage() {
        #[derive(Default)]
        struct MemoryStorage(std::collections::HashMap<String, String>);

        impl eframe::Storage for MemoryStorage {
            fn get_string(&self, key: &str) -> Option<String> {
                self.0.get(key).cloned()
            }

            fn set_string(&mut self, key: &str, value: String) {
                self.0.insert(key.to_owned(), value);
            }

            fn remove_string(&mut self, key: &str) {
                self.0.remove(key);
            }

            fn flush(&mut self) {}
        }

        let expected = AppState::default();
        let mut storage = MemoryStorage::default();
        eframe::set_value(&mut storage, eframe::APP_KEY, &expected);
        let restored: AppState = eframe::get_value(&storage, eframe::APP_KEY)
            .expect("RSpice must be able to restore a session it just saved");

        assert_eq!(
            restored.workspace.project.id(),
            expected.workspace.project.id()
        );
        assert_eq!(restored.schematic.components, expected.schematic.components);
        assert_eq!(restored.schematic.wires, expected.schematic.wires);
    }
}
