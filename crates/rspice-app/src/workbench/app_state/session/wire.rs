//! What a session keeps of the workbench and of the document engines'
//! session state.
//!
//! [`WorkbenchSession`] and the feature slots hold runtime state beside what a
//! session keeps. The kept part of each is declared here, field for field,
//! with the names, serde attributes and order sessions have always been
//! written with. A saved session is read back by those names, so a field that
//! moves to another owner must not move here: if it did, every session saved
//! before the move would lose it.
//!
//! The workbench is saved every few minutes, so it is written from a borrowed
//! view, [`WorkbenchWireRef`], and read into the owned [`WorkbenchWire`]. The
//! two declare the same fields with the same attributes. The legacy project
//! identity is a hash of the workbench as JSON, so they also write the same
//! JSON: a difference would give every such project a new identity.

use std::collections::{BTreeSet, HashMap, HashSet};

use rspice_results_ui::report::session::ReportAuthoringState;
use serde::{Deserialize, Serialize};

use crate::state::{GridStyle, SchematicVisibilityPolicy, SchematicWireRoutingStyle};
use crate::state::{SimulationPage, VerificationPage};
use crate::ui::Theme;
use crate::workbench::app::dialogs::selection::bulk_filter::SelectionBulkFilterSession;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::model_editor::ModelEditorRecoveryDraft;
use crate::workbench::lifecycle::session::normalize_autosave_minutes;
use crate::workbench::lifecycle::window_session::WindowSessionRegistry;
use crate::workbench::models::workspace_state::{
    ModelsWorkbenchViewState, PdkTechnologyAuthoringRecovery,
};
use crate::workbench::preferences::UserPreferences;
use crate::workbench::project::workspace_state::{LibraryCellviewPage, ProjectPage};
use crate::workbench::slots::{
    CodeState, DesignState, HardcopyState, ModelsState, PlatformState, ProjectState, ResultsState,
    SettingsState, SimulateState,
};
use crate::workbench::state::{
    ConsolePage, DesignPanel, SpecialistToolBrowserState, VerificationSessionState,
    WorkbenchSession, Workspace, WorkspaceDocumentId, WorkspaceDocumentRegistry,
    WorkspaceLayoutState, default_analysis_index, default_console_height, default_inspector_width,
    default_navigator_width,
};
use crate::workbench::{SurfaceNavigation, WorkspacePreset};

/// What a session keeps of the document engines' session state, which the
/// workbench and the design, results, platform and settings slots hold.
/// Responsive layout is persisted by the workbench wire; transient document
/// interaction signals are not.
///
/// The serde name is the one sessions were first written under, so a reader
/// that checks struct names reads them.
#[derive(Serialize, Deserialize)]
#[serde(rename = "UiSessionStateSer")]
pub(in crate::workbench) struct UiSessionWire {
    #[serde(default)]
    theme: Theme,
    /// Legacy on/off flag, still written so older builds keep their
    /// setting; `grid_style` wins when present.
    #[serde(default = "default_true")]
    show_grid: bool,
    #[serde(default)]
    grid_style: Option<GridStyle>,
    #[serde(default)]
    grid_restore_style: Option<GridStyle>,
    #[serde(default)]
    schematic_selection_filter: crate::state::SchematicSelectionFilter,
    #[serde(default)]
    schematic_bulk_edit_filters: SelectionBulkFilterSession,
    #[serde(default)]
    schematic_visibility: SchematicVisibilityPolicy,
    #[serde(default)]
    drawing_sheet_layers: crate::state::DrawingSheetLayerVisibility,
    #[serde(default)]
    schematic_snap: crate::state::SnapEngine,
    #[serde(default)]
    schematic_routing_mode: Option<crate::state::WireRoutingMode>,
    #[serde(default)]
    duplicate_external_nets: crate::state::DuplicateExternalNets,
    #[serde(default)]
    engineering_table_views: crate::state::engineering_table::EngineeringTableViewStore,
    // `theme` and the next three fields are still written here so an older
    // build finds them, but the session no longer owns them: the preferences
    // file is read over this copy at launch
    // (`app_state::session::preferences_file`).
    #[serde(default = "default_autosave_minutes")]
    autosave_minutes: u8,
    #[serde(default)]
    preferences: crate::workbench::preferences::UserPreferences,
    #[serde(default)]
    browser_spoken_feedback: bool,
    #[serde(default)]
    text_locale: crate::workbench::UiTextLocale,
    #[serde(default)]
    result_viewer: crate::workbench::documents::result_document::ResultViewer,
    /// Bit periods the reader pinned eyes to, for results that carry prepared
    /// provenance. Legacy keys name a run-local analysis id that a later
    /// session will never mint again, so they are dropped on save rather than
    /// restored onto whichever result inherits the number.
    #[serde(default)]
    eye_timebase: Vec<(
        rspice_results_ui::eye_diagram::EyeTimebaseKey,
        rspice_results_ui::eye_diagram::EyeTimebase,
    )>,
}

fn default_autosave_minutes() -> u8 {
    crate::services::session::DEFAULT_AUTOSAVE_MINUTES
}

fn default_true() -> bool {
    true
}

impl Default for UiSessionWire {
    /// What a session without this part reads as: a fresh session, every
    /// owner at its default.
    fn default() -> Self {
        Self::capture_owners(
            &WorkbenchSession::default(),
            &PlatformState::default(),
            &SettingsState::default(),
            &DesignState::default(),
            &ResultsState::default(),
            &UserPreferences::default(),
        )
    }
}

/// A test that round-trips the session's own fields captures them from a
/// whole state.
#[cfg(test)]
impl From<&AppState> for UiSessionWire {
    fn from(state: &AppState) -> Self {
        Self::capture(state)
    }
}

/// A test that round-trips the session's own fields restores them onto a
/// default state.
#[cfg(test)]
impl From<UiSessionWire> for AppState {
    fn from(ser: UiSessionWire) -> Self {
        let mut state = AppState::default();
        ser.scatter(&mut state);
        state
    }
}

impl UiSessionWire {
    /// The kept part of the document engines' session, with the reader's
    /// preferences in the place sessions have always written them, though the
    /// model holds them now.
    pub(in crate::workbench) fn capture(state: &AppState) -> Self {
        Self::capture_owners(
            &state.workbench,
            &state.platform,
            &state.settings,
            &state.design,
            &state.results,
            &state.model.preferences,
        )
    }

    fn capture_owners(
        workbench: &WorkbenchSession,
        platform: &PlatformState,
        settings: &SettingsState,
        design: &DesignState,
        results: &ResultsState,
        preferences: &UserPreferences,
    ) -> Self {
        Self {
            theme: workbench.theme,
            show_grid: design.grid.visible(),
            grid_style: Some(design.grid),
            grid_restore_style: Some(if design.grid_restore_style.visible() {
                design.grid_restore_style
            } else if design.grid.visible() {
                design.grid
            } else {
                GridStyle::Dots
            }),
            schematic_selection_filter: design.schematic_selection_filter,
            schematic_bulk_edit_filters: design.schematic_bulk_edit_filters.clone(),
            schematic_visibility: design.schematic_visibility,
            drawing_sheet_layers: design.drawing_sheet_layers,
            schematic_snap: design.schematic_snap.clone(),
            schematic_routing_mode: Some(design.schematic_routing_mode),
            duplicate_external_nets: design.duplicate_external_nets,
            engineering_table_views: design.engineering_table_views.clone(),
            autosave_minutes: normalize_autosave_minutes(platform.autosave_minutes),
            preferences: preferences.clone(),
            browser_spoken_feedback: settings.browser_spoken_feedback,
            text_locale: workbench.text_locale,
            result_viewer: results.results.session.viewer,
            eye_timebase: results
                .results
                .session
                .eye_timebase
                .iter()
                .filter(|(key, _)| key.is_persistable())
                .map(|(key, timebase)| (*key, *timebase))
                .collect(),
        }
    }

    /// Restore what the session kept onto the owners in `state`, and the
    /// preferences it carries onto the model. Everything else the owners
    /// hold keeps what `state` holds.
    pub(in crate::workbench) fn scatter(self, state: &mut AppState) {
        let ser = self;
        let grid = ser.grid_style.unwrap_or(if ser.show_grid {
            GridStyle::Dots
        } else {
            GridStyle::Off
        });
        let grid_restore_style = ser
            .grid_restore_style
            .filter(|style| style.visible())
            .unwrap_or(if grid.visible() {
                grid
            } else {
                GridStyle::Dots
            });
        let mut preferences = ser.preferences;
        preferences.normalize();
        let schematic_routing_mode =
            ser.schematic_routing_mode
                .unwrap_or(match ser.schematic_visibility.wire_routing {
                    SchematicWireRoutingStyle::Orthogonal => {
                        crate::state::WireRoutingMode::HorizontalFirst
                    }
                    SchematicWireRoutingStyle::FortyFiveDegree => {
                        crate::state::WireRoutingMode::FortyFiveDegree
                    }
                    SchematicWireRoutingStyle::FreeAngle => crate::state::WireRoutingMode::Diagonal,
                });
        state.workbench.theme = ser.theme;
        state.design.grid = grid;
        state.design.grid_restore_style = grid_restore_style;
        state.design.schematic_selection_filter = ser.schematic_selection_filter;
        state.design.schematic_bulk_edit_filters = ser.schematic_bulk_edit_filters;
        state.design.schematic_visibility = ser.schematic_visibility;
        state.design.drawing_sheet_layers = ser.drawing_sheet_layers;
        state.design.schematic_snap = ser.schematic_snap;
        state.design.schematic_routing_mode = schematic_routing_mode;
        state.design.duplicate_external_nets = ser.duplicate_external_nets;
        state.design.engineering_table_views = ser.engineering_table_views;
        state.platform.autosave_minutes = normalize_autosave_minutes(ser.autosave_minutes);
        state.settings.browser_spoken_feedback = ser.browser_spoken_feedback;
        state.workbench.text_locale = ser.text_locale;
        state.results.results.session.viewer = ser.result_viewer;
        state.results.results.session.eye_timebase = ser
            .eye_timebase
            .into_iter()
            .filter(|(key, _)| key.is_persistable())
            .collect();
        *state.model.preferences = preferences;
    }
}

/// What a session keeps of [`WorkbenchSession`], as it is read back.
///
/// Each field's attribute says what a session without it restores. A session
/// with no workbench at all restores the defaults of [`WorkbenchSession`] and
/// of the slots that own the rest of what it keeps, which is what
/// [`Self::default`] holds.
#[derive(Deserialize)]
#[serde(rename = "WorkbenchState")]
pub(in crate::workbench) struct WorkbenchWire {
    #[serde(default)]
    workspace: Workspace,
    #[serde(default)]
    window_session: WindowSessionRegistry,
    #[serde(default)]
    navigation_schema_version: u8,
    #[serde(default)]
    layout_schema_version: u8,
    #[serde(default)]
    navigation: SurfaceNavigation,
    #[serde(default = "default_true")]
    navigator_visible: bool,
    #[serde(default = "default_true")]
    inspector_visible: bool,
    #[serde(default)]
    console_visible: bool,
    #[serde(default)]
    console_maximized: bool,
    #[serde(default)]
    focus_mode: bool,
    #[serde(default)]
    split_with_results: bool,
    #[serde(default = "default_navigator_width")]
    navigator_width: f32,
    #[serde(default)]
    navigator_width_custom: bool,
    #[serde(default = "default_inspector_width")]
    inspector_width: f32,
    #[serde(default)]
    inspector_width_custom: bool,
    #[serde(default = "default_console_height")]
    console_height: f32,
    #[serde(default)]
    workspace_layouts: HashMap<Workspace, WorkspaceLayoutState>,
    #[serde(default)]
    workspace_preset: WorkspacePreset,
    #[serde(default)]
    console_page: ConsolePage,
    #[serde(default)]
    design_panel: DesignPanel,
    #[serde(default)]
    documents: WorkspaceDocumentRegistry,
    #[serde(default)]
    netlist_open_documents: HashSet<WorkspaceDocumentId>,
    #[serde(default)]
    project_page: ProjectPage,
    #[serde(default)]
    project_library_filter: String,
    #[serde(default)]
    project_dependency_filter: String,
    #[serde(default)]
    project_dependency_selection: Option<String>,
    #[serde(default)]
    project_checkpoint_selection: Option<String>,
    #[serde(default)]
    library_cellview_page: LibraryCellviewPage,
    #[serde(default)]
    verification_page: VerificationPage,
    #[serde(default)]
    simulation_page: SimulationPage,
    #[serde(default)]
    verification: VerificationSessionState,
    #[serde(default)]
    models_page: crate::state::ModelsPage,
    #[serde(default)]
    models_view: ModelsWorkbenchViewState,
    #[serde(default = "default_analysis_index")]
    active_analysis: usize,
    #[serde(default)]
    active_analysis_instance: Option<crate::product::AnalysisInstanceId>,
    #[serde(default)]
    selected_specification: Option<String>,
    #[serde(default)]
    selected_design_variable: Option<String>,
    #[serde(default)]
    selected_saved_output: Option<String>,
    #[serde(default)]
    selected_capture_group: Option<crate::product::CaptureGroupId>,
    #[serde(default)]
    selected_run_set_dimension: Option<String>,
    #[serde(default)]
    selected_model: Option<String>,
    #[serde(default)]
    netlist_outline_collapsed: BTreeSet<crate::state::OutlineSectionKind>,
    #[serde(default)]
    command_query: String,
    #[serde(default)]
    specialist_tool_browser: SpecialistToolBrowserState,
    #[serde(default)]
    report_authoring: ReportAuthoringState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model_editor_recovery: Option<ModelEditorRecoveryDraft>,
    #[serde(default)]
    pdk_technology_authoring: PdkTechnologyAuthoringRecovery,
}

/// What a session keeps of [`WorkbenchSession`], as it is written: borrowed, so
/// saving the session copies nothing.
///
/// Declared exactly as [`WorkbenchWire`] is. The one difference is the
/// model-editor recovery, held as `Option<&_>` rather than `&Option<_>` so
/// that `Option::is_none` takes it as it is.
#[derive(Serialize)]
#[serde(rename = "WorkbenchState")]
pub(in crate::workbench) struct WorkbenchWireRef<'a> {
    #[serde(default)]
    workspace: &'a Workspace,
    #[serde(default)]
    window_session: &'a WindowSessionRegistry,
    #[serde(default)]
    navigation_schema_version: &'a u8,
    #[serde(default)]
    layout_schema_version: &'a u8,
    #[serde(default)]
    navigation: &'a SurfaceNavigation,
    #[serde(default = "default_true")]
    navigator_visible: &'a bool,
    #[serde(default = "default_true")]
    inspector_visible: &'a bool,
    #[serde(default)]
    console_visible: &'a bool,
    #[serde(default)]
    console_maximized: &'a bool,
    #[serde(default)]
    focus_mode: &'a bool,
    #[serde(default)]
    split_with_results: &'a bool,
    #[serde(default = "default_navigator_width")]
    navigator_width: &'a f32,
    #[serde(default)]
    navigator_width_custom: &'a bool,
    #[serde(default = "default_inspector_width")]
    inspector_width: &'a f32,
    #[serde(default)]
    inspector_width_custom: &'a bool,
    #[serde(default = "default_console_height")]
    console_height: &'a f32,
    #[serde(default)]
    workspace_layouts: &'a HashMap<Workspace, WorkspaceLayoutState>,
    #[serde(default)]
    workspace_preset: &'a WorkspacePreset,
    #[serde(default)]
    console_page: &'a ConsolePage,
    #[serde(default)]
    design_panel: &'a DesignPanel,
    #[serde(default)]
    documents: &'a WorkspaceDocumentRegistry,
    #[serde(default)]
    netlist_open_documents: &'a HashSet<WorkspaceDocumentId>,
    #[serde(default)]
    project_page: &'a ProjectPage,
    #[serde(default)]
    project_library_filter: &'a String,
    #[serde(default)]
    project_dependency_filter: &'a String,
    #[serde(default)]
    project_dependency_selection: &'a Option<String>,
    #[serde(default)]
    project_checkpoint_selection: &'a Option<String>,
    #[serde(default)]
    library_cellview_page: &'a LibraryCellviewPage,
    #[serde(default)]
    verification_page: &'a VerificationPage,
    #[serde(default)]
    simulation_page: &'a SimulationPage,
    #[serde(default)]
    verification: &'a VerificationSessionState,
    #[serde(default)]
    models_page: &'a crate::state::ModelsPage,
    #[serde(default)]
    models_view: &'a ModelsWorkbenchViewState,
    #[serde(default = "default_analysis_index")]
    active_analysis: &'a usize,
    #[serde(default)]
    active_analysis_instance: &'a Option<crate::product::AnalysisInstanceId>,
    #[serde(default)]
    selected_specification: &'a Option<String>,
    #[serde(default)]
    selected_design_variable: &'a Option<String>,
    #[serde(default)]
    selected_saved_output: &'a Option<String>,
    #[serde(default)]
    selected_capture_group: &'a Option<crate::product::CaptureGroupId>,
    #[serde(default)]
    selected_run_set_dimension: &'a Option<String>,
    #[serde(default)]
    selected_model: &'a Option<String>,
    #[serde(default)]
    netlist_outline_collapsed: &'a BTreeSet<crate::state::OutlineSectionKind>,
    #[serde(default)]
    command_query: &'a String,
    #[serde(default)]
    specialist_tool_browser: &'a SpecialistToolBrowserState,
    #[serde(default)]
    report_authoring: &'a ReportAuthoringState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model_editor_recovery: Option<&'a ModelEditorRecoveryDraft>,
    #[serde(default)]
    pdk_technology_authoring: &'a PdkTechnologyAuthoringRecovery,
}

impl<'a> WorkbenchWireRef<'a> {
    /// Borrow what a session keeps of `state`.
    pub(in crate::workbench) fn capture(state: &'a AppState) -> Self {
        Self::capture_parts(
            &state.workbench,
            &state.design,
            &state.code,
            &state.project,
            &state.simulate,
            &state.models,
            &state.hardcopy,
        )
    }

    /// Borrow what a session keeps of the workbench and of the slots that
    /// own the rest of what it keeps. The slots' fields are read by name; the
    /// session round trip of a populated session is what guards them.
    fn capture_parts(
        workbench: &'a WorkbenchSession,
        design: &'a DesignState,
        code: &'a CodeState,
        project: &'a ProjectState,
        simulate: &'a SimulateState,
        models: &'a ModelsState,
        hardcopy: &'a HardcopyState,
    ) -> Self {
        // No `..`: a field added to `WorkbenchSession` does not compile until it
        // is either kept here or named as runtime-only.
        let WorkbenchSession {
            workspace,
            window_session,
            navigation_schema_version,
            layout_schema_version,
            navigation,
            route_diagnostic: _,
            navigator_visible,
            inspector_visible,
            console_visible,
            console_maximized,
            focus_mode,
            split_with_results,
            full_screen: _,
            full_screen_presentation: _,
            coarse_pointer: _,
            navigator_width,
            navigator_width_custom,
            inspector_width,
            inspector_width_custom,
            console_height,
            workspace_layouts,
            workspace_preset,
            console_page,
            console_view: _,
            problems_view: _,
            documents,
            prompt: _,
            navigator_trees: _,
            command_query,
            inline_edit: _,
            last_repeatable_command: _,
            drawer: _,
            specialist_tool_browser,
            notification_center_open: _,
            notification_filter: _,
            notification_unread_only: _,
            window_session_dialog: _,
            command_palette: _,
            confirmation_dialog: _,
            project_review_dialog: _,
            // The UI session wire keeps the theme and the text locale; the
            // rest are runtime-only.
            theme: _,
            number_locale: _,
            text_locale: _,
            toasts: _,
            active_source_editor: _,
            outbox: _,
        } = workbench;
        Self {
            workspace,
            window_session,
            navigation_schema_version,
            layout_schema_version,
            navigation,
            navigator_visible,
            inspector_visible,
            console_visible,
            console_maximized,
            focus_mode,
            split_with_results,
            navigator_width,
            navigator_width_custom,
            inspector_width,
            inspector_width_custom,
            console_height,
            workspace_layouts,
            workspace_preset,
            console_page,
            design_panel: &design.design_panel,
            documents,
            netlist_open_documents: &code.netlist_open_documents,
            project_page: &project.project_page,
            project_library_filter: &project.project_library_filter,
            project_dependency_filter: &project.project_dependency_filter,
            project_dependency_selection: &project.project_dependency_selection,
            project_checkpoint_selection: &project.project_checkpoint_selection,
            library_cellview_page: &project.library_cellview_page,
            verification_page: &simulate.verification_page,
            simulation_page: &simulate.simulation_page,
            verification: &simulate.verification,
            models_page: &models.models_page,
            models_view: &models.models_view,
            active_analysis: &simulate.active_analysis,
            active_analysis_instance: &simulate.active_analysis_instance,
            selected_specification: &simulate.selected_specification,
            selected_design_variable: &simulate.selected_design_variable,
            selected_saved_output: &simulate.selected_saved_output,
            selected_capture_group: &simulate.selected_capture_group,
            selected_run_set_dimension: &simulate.selected_run_set_dimension,
            selected_model: &models.selected_model,
            netlist_outline_collapsed: &code.netlist_outline_collapsed,
            command_query,
            specialist_tool_browser,
            report_authoring: &hardcopy.report_authoring,
            model_editor_recovery: models.model_editor_recovery.as_ref(),
            pdk_technology_authoring: &models.pdk_technology_authoring,
        }
    }

    /// An owned copy of the view.
    fn to_wire(&self) -> WorkbenchWire {
        WorkbenchWire {
            workspace: *self.workspace,
            window_session: self.window_session.clone(),
            navigation_schema_version: *self.navigation_schema_version,
            layout_schema_version: *self.layout_schema_version,
            navigation: self.navigation.clone(),
            navigator_visible: *self.navigator_visible,
            inspector_visible: *self.inspector_visible,
            console_visible: *self.console_visible,
            console_maximized: *self.console_maximized,
            focus_mode: *self.focus_mode,
            split_with_results: *self.split_with_results,
            navigator_width: *self.navigator_width,
            navigator_width_custom: *self.navigator_width_custom,
            inspector_width: *self.inspector_width,
            inspector_width_custom: *self.inspector_width_custom,
            console_height: *self.console_height,
            workspace_layouts: self.workspace_layouts.clone(),
            workspace_preset: *self.workspace_preset,
            console_page: *self.console_page,
            design_panel: *self.design_panel,
            documents: self.documents.clone(),
            netlist_open_documents: self.netlist_open_documents.clone(),
            project_page: *self.project_page,
            project_library_filter: self.project_library_filter.clone(),
            project_dependency_filter: self.project_dependency_filter.clone(),
            project_dependency_selection: self.project_dependency_selection.clone(),
            project_checkpoint_selection: self.project_checkpoint_selection.clone(),
            library_cellview_page: *self.library_cellview_page,
            verification_page: *self.verification_page,
            simulation_page: *self.simulation_page,
            verification: self.verification.clone(),
            models_page: *self.models_page,
            models_view: self.models_view.clone(),
            active_analysis: *self.active_analysis,
            active_analysis_instance: *self.active_analysis_instance,
            selected_specification: self.selected_specification.clone(),
            selected_design_variable: self.selected_design_variable.clone(),
            selected_saved_output: self.selected_saved_output.clone(),
            selected_capture_group: *self.selected_capture_group,
            selected_run_set_dimension: self.selected_run_set_dimension.clone(),
            selected_model: self.selected_model.clone(),
            netlist_outline_collapsed: self.netlist_outline_collapsed.clone(),
            command_query: self.command_query.clone(),
            specialist_tool_browser: self.specialist_tool_browser.clone(),
            report_authoring: self.report_authoring.clone(),
            model_editor_recovery: self.model_editor_recovery.cloned(),
            pdk_technology_authoring: self.pdk_technology_authoring.clone(),
        }
    }
}

impl<'a> From<&'a WorkbenchWire> for WorkbenchWireRef<'a> {
    fn from(wire: &'a WorkbenchWire) -> Self {
        Self {
            workspace: &wire.workspace,
            window_session: &wire.window_session,
            navigation_schema_version: &wire.navigation_schema_version,
            layout_schema_version: &wire.layout_schema_version,
            navigation: &wire.navigation,
            navigator_visible: &wire.navigator_visible,
            inspector_visible: &wire.inspector_visible,
            console_visible: &wire.console_visible,
            console_maximized: &wire.console_maximized,
            focus_mode: &wire.focus_mode,
            split_with_results: &wire.split_with_results,
            navigator_width: &wire.navigator_width,
            navigator_width_custom: &wire.navigator_width_custom,
            inspector_width: &wire.inspector_width,
            inspector_width_custom: &wire.inspector_width_custom,
            console_height: &wire.console_height,
            workspace_layouts: &wire.workspace_layouts,
            workspace_preset: &wire.workspace_preset,
            console_page: &wire.console_page,
            design_panel: &wire.design_panel,
            documents: &wire.documents,
            netlist_open_documents: &wire.netlist_open_documents,
            project_page: &wire.project_page,
            project_library_filter: &wire.project_library_filter,
            project_dependency_filter: &wire.project_dependency_filter,
            project_dependency_selection: &wire.project_dependency_selection,
            project_checkpoint_selection: &wire.project_checkpoint_selection,
            library_cellview_page: &wire.library_cellview_page,
            verification_page: &wire.verification_page,
            simulation_page: &wire.simulation_page,
            verification: &wire.verification,
            models_page: &wire.models_page,
            models_view: &wire.models_view,
            active_analysis: &wire.active_analysis,
            active_analysis_instance: &wire.active_analysis_instance,
            selected_specification: &wire.selected_specification,
            selected_design_variable: &wire.selected_design_variable,
            selected_saved_output: &wire.selected_saved_output,
            selected_capture_group: &wire.selected_capture_group,
            selected_run_set_dimension: &wire.selected_run_set_dimension,
            selected_model: &wire.selected_model,
            netlist_outline_collapsed: &wire.netlist_outline_collapsed,
            command_query: &wire.command_query,
            specialist_tool_browser: &wire.specialist_tool_browser,
            report_authoring: &wire.report_authoring,
            model_editor_recovery: wire.model_editor_recovery.as_ref(),
            pdk_technology_authoring: &wire.pdk_technology_authoring,
        }
    }
}

impl Default for WorkbenchWire {
    /// Built from each owner's default rather than from a whole `AppState`,
    /// which would build the model and the PDK configuration only to drop
    /// them. Each owner is boxed so the frame holds none of them inline.
    fn default() -> Self {
        WorkbenchWireRef::capture_parts(
            &Box::<WorkbenchSession>::default(),
            &Box::<DesignState>::default(),
            &Box::<CodeState>::default(),
            &Box::<ProjectState>::default(),
            &Box::<SimulateState>::default(),
            &Box::<ModelsState>::default(),
            &Box::<HardcopyState>::default(),
        )
        .to_wire()
    }
}

impl WorkbenchWire {
    /// Restore what the session kept onto `state`: the workbench, and the
    /// slots that own the rest of what it keeps.
    ///
    /// Runtime-only fields keep what `state` holds. A restored session
    /// scatters onto the defaults, whose runtime-only fields hold their
    /// types' defaults.
    pub(in crate::workbench) fn scatter(self, state: &mut AppState) {
        // No `..`: a field added to `WorkbenchSession` does not compile until it
        // is either restored here or named as runtime-only.
        let WorkbenchSession {
            workspace,
            window_session,
            navigation_schema_version,
            layout_schema_version,
            navigation,
            route_diagnostic: _,
            navigator_visible,
            inspector_visible,
            console_visible,
            console_maximized,
            focus_mode,
            split_with_results,
            full_screen: _,
            full_screen_presentation: _,
            coarse_pointer: _,
            navigator_width,
            navigator_width_custom,
            inspector_width,
            inspector_width_custom,
            console_height,
            workspace_layouts,
            workspace_preset,
            console_page,
            console_view: _,
            problems_view: _,
            documents,
            prompt: _,
            navigator_trees: _,
            command_query,
            inline_edit: _,
            last_repeatable_command: _,
            drawer: _,
            specialist_tool_browser,
            notification_center_open: _,
            notification_filter: _,
            notification_unread_only: _,
            window_session_dialog: _,
            command_palette: _,
            confirmation_dialog: _,
            project_review_dialog: _,
            // The UI session wire restores the theme and the text locale; the
            // rest are runtime-only.
            theme: _,
            number_locale: _,
            text_locale: _,
            toasts: _,
            active_source_editor: _,
            outbox: _,
        } = &mut *state.workbench;
        *workspace = self.workspace;
        *window_session = self.window_session;
        *navigation_schema_version = self.navigation_schema_version;
        *layout_schema_version = self.layout_schema_version;
        *navigation = self.navigation;
        *navigator_visible = self.navigator_visible;
        *inspector_visible = self.inspector_visible;
        *console_visible = self.console_visible;
        *console_maximized = self.console_maximized;
        *focus_mode = self.focus_mode;
        *split_with_results = self.split_with_results;
        *navigator_width = self.navigator_width;
        *navigator_width_custom = self.navigator_width_custom;
        *inspector_width = self.inspector_width;
        *inspector_width_custom = self.inspector_width_custom;
        *console_height = self.console_height;
        *workspace_layouts = self.workspace_layouts;
        *workspace_preset = self.workspace_preset;
        *console_page = self.console_page;
        state.design.design_panel = self.design_panel;
        *documents = self.documents;
        state.code.netlist_open_documents = self.netlist_open_documents;
        state.project.project_page = self.project_page;
        state.project.project_library_filter = self.project_library_filter;
        state.project.project_dependency_filter = self.project_dependency_filter;
        state.project.project_dependency_selection = self.project_dependency_selection;
        state.project.project_checkpoint_selection = self.project_checkpoint_selection;
        state.project.library_cellview_page = self.library_cellview_page;
        state.simulate.verification_page = self.verification_page;
        state.simulate.simulation_page = self.simulation_page;
        state.simulate.verification = self.verification;
        state.models.models_page = self.models_page;
        state.models.models_view = self.models_view;
        state.simulate.active_analysis = self.active_analysis;
        state.simulate.active_analysis_instance = self.active_analysis_instance;
        state.simulate.selected_specification = self.selected_specification;
        state.simulate.selected_design_variable = self.selected_design_variable;
        state.simulate.selected_saved_output = self.selected_saved_output;
        state.simulate.selected_capture_group = self.selected_capture_group;
        state.simulate.selected_run_set_dimension = self.selected_run_set_dimension;
        state.models.selected_model = self.selected_model;
        state.code.netlist_outline_collapsed = self.netlist_outline_collapsed;
        *command_query = self.command_query;
        *specialist_tool_browser = self.specialist_tool_browser;
        state.hardcopy.report_authoring = self.report_authoring;
        state.models.model_editor_recovery = self.model_editor_recovery;
        state.models.pdk_technology_authoring = self.pdk_technology_authoring;
    }
}

#[cfg(test)]
mod ui_session_tests {
    use super::*;
    use crate::state::{
        SchematicAnnotationVisibility, SchematicBackAnnotationContent, SchematicNetHighlighting,
        SchematicParameterLabelVisibility, SchematicReviewMarkerVisibility,
    };
    use crate::workbench::app::dialogs::selection::bulk_filter::{
        SelectionBulkFilter, SelectionBulkHierarchyScope, SelectionBulkObjectKind,
    };

    #[test]
    fn schematic_selection_filter_round_trips_with_the_ui_session() {
        let mut session = AppState::default();
        session.design.schematic_selection_filter.instances = false;
        session.design.schematic_selection_filter.annotations = false;
        let serialized =
            serde_json::to_value(UiSessionWire::from(&session)).expect("serialize session");
        let wire: UiSessionWire = serde_json::from_value(serialized).expect("deserialize session");
        let restored = AppState::from(wire);

        assert_eq!(
            restored.design.schematic_selection_filter,
            session.design.schematic_selection_filter
        );
    }

    #[test]
    fn ui_text_locale_round_trips_and_legacy_sessions_default_to_english() {
        let mut session = AppState::default();
        session.workbench.text_locale = crate::workbench::UiTextLocale::PseudoExpanded;
        let serialized =
            serde_json::to_value(UiSessionWire::from(&session)).expect("serialize session");
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serialized).expect("deserialize session"),
        );
        assert_eq!(
            restored.workbench.text_locale,
            crate::workbench::UiTextLocale::PseudoExpanded
        );

        let legacy = AppState::from(
            serde_json::from_value::<UiSessionWire>(serde_json::json!({})).expect("legacy session"),
        );
        assert_eq!(
            legacy.workbench.text_locale,
            crate::workbench::UiTextLocale::EnglishUnitedStates
        );
    }

    #[test]
    fn legacy_ui_session_defaults_to_all_selection_classes() {
        let wire: UiSessionWire =
            serde_json::from_value(serde_json::json!({})).expect("legacy session without a filter");
        let restored = AppState::from(wire);

        assert!(
            restored.design.schematic_selection_filter.instances
                && restored.design.schematic_selection_filter.wires
                && restored.design.schematic_selection_filter.labels
                && restored.design.schematic_selection_filter.annotations
        );
    }

    #[test]
    fn schematic_bulk_edit_filters_round_trip_with_the_ui_session() {
        let mut session = AppState::default();
        session.design.schematic_bulk_edit_filters.active = SelectionBulkFilter {
            query: "OPA*".to_owned(),
            object_kind: SelectionBulkObjectKind::InstancesAndParameters,
            hierarchy_scope: SelectionBulkHierarchyScope::CompleteProject,
            model_cell: "OPA189*".to_owned(),
            current_property: "section=tt".to_owned(),
        };
        session
            .design
            .schematic_bulk_edit_filters
            .save_active("TT amplifiers")
            .expect("save filter");

        let serialized =
            serde_json::to_value(UiSessionWire::from(&session)).expect("serialize session");
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.design.schematic_bulk_edit_filters,
            session.design.schematic_bulk_edit_filters
        );
    }

    #[test]
    fn personal_engineering_table_views_round_trip_with_the_ui_session() {
        let mut session = AppState::default();
        let dataset =
            crate::state::engineering_table::active_schematic_dataset(&Default::default());
        let view = crate::state::engineering_table::EngineeringTableView::for_dataset(&dataset);
        session
            .design
            .engineering_table_views
            .save(
                "My device view",
                crate::state::engineering_table::EngineeringViewScope::Personal,
                view,
                true,
                &dataset,
            )
            .expect("valid personal view");

        let serialized =
            serde_json::to_value(UiSessionWire::from(&session)).expect("serialize session");
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.design.engineering_table_views,
            session.design.engineering_table_views
        );
    }

    #[test]
    fn legacy_ui_session_defaults_bulk_edit_filters_without_project_mutation() {
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serde_json::json!({}))
                .expect("legacy session without bulk filters"),
        );
        assert_eq!(
            restored.design.schematic_bulk_edit_filters,
            SelectionBulkFilterSession::default()
        );
    }

    #[test]
    fn schematic_visibility_policy_round_trips_with_the_ui_session() {
        let mut session = AppState::default();
        session.design.schematic_visibility = SchematicVisibilityPolicy {
            annotations: SchematicAnnotationVisibility::ViolationsOnly,
            back_annotation: SchematicBackAnnotationContent::VoltagesCurrentsAndPower,
            parameter_labels: SchematicParameterLabelVisibility::Hidden,
            wire_routing: SchematicWireRoutingStyle::FortyFiveDegree,
            net_highlighting: SchematicNetHighlighting::Off,
            review_markers: SchematicReviewMarkerVisibility::All,
        };

        let serialized =
            serde_json::to_value(UiSessionWire::from(&session)).expect("serialize session");
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.design.schematic_visibility,
            session.design.schematic_visibility
        );
    }

    /// Earlier builds saved a hierarchy-context choice with the policy (C-13,
    /// J-3: the dimmed-parent overlay it governed is gone). A session written
    /// then still loads, and every choice saved beside it survives.
    #[test]
    fn a_session_saved_with_the_retired_hierarchy_choice_still_loads() {
        let legacy = serde_json::json!({
            "schematic_visibility": {
                "hierarchy": "full-visible-hierarchy",
                "annotations": "violations-only",
                "back_annotation": "voltages-only",
                "parameter_labels": "hidden",
                "wire_routing": "forty-five-degree",
                "net_highlighting": "off",
                "review_markers": "all"
            }
        });
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(legacy)
                .expect("a session carrying the retired hierarchy choice loads"),
        );

        assert_eq!(
            restored.design.schematic_visibility,
            SchematicVisibilityPolicy {
                annotations: SchematicAnnotationVisibility::ViolationsOnly,
                back_annotation: SchematicBackAnnotationContent::VoltagesOnly,
                parameter_labels: SchematicParameterLabelVisibility::Hidden,
                wire_routing: SchematicWireRoutingStyle::FortyFiveDegree,
                net_highlighting: SchematicNetHighlighting::Off,
                review_markers: SchematicReviewMarkerVisibility::All,
            }
        );
    }

    /// Before every sheet took its standard's frame, the drawing-sheet layers
    /// held a guide to the printable margins around an inset frame, and the
    /// centring-mark layer was the registration-mark layer. An eframe session
    /// written then still loads, keeping the mark choice.
    #[test]
    fn a_ron_session_saved_with_retired_drawing_sheet_layers_still_loads() {
        let session = AppState::default();
        let today = ron::to_string(&session.design.drawing_sheet_layers).unwrap();
        let written = ron::to_string(&UiSessionWire::from(&session)).unwrap();
        assert!(written.contains(&today), "the session holds its layers");
        let legacy = written.replacen(
            &today,
            "(margins:false,zones:false,title_block:true,registration_marks:false,\
             dimensions_at_detail_zoom:true)",
            1,
        );

        let restored = AppState::from(
            ron::from_str::<UiSessionWire>(&legacy)
                .expect("a session carrying the retired layers loads"),
        );

        assert_eq!(
            restored.design.drawing_sheet_layers,
            crate::state::DrawingSheetLayerVisibility {
                zones: false,
                title_block: true,
                centring_marks: false,
                dimensions_at_detail_zoom: true,
            }
        );
    }

    #[test]
    fn grid_cycle_is_dots_then_lines_then_off_and_round_trips_exactly() {
        let mut session = AppState::default();
        session.design.set_grid_style(GridStyle::Dots);
        assert_eq!(session.design.cycle_grid_style(), GridStyle::Lines);
        assert_eq!(session.design.cycle_grid_style(), GridStyle::Off);
        assert_eq!(session.design.grid, GridStyle::Off);
        assert_eq!(session.design.grid_restore_style, GridStyle::Lines);

        let serialized =
            serde_json::to_value(UiSessionWire::from(&session)).expect("serialize session");
        let mut restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serialized).expect("deserialize session"),
        );

        assert_eq!(restored.design.grid, GridStyle::Off);
        assert_eq!(restored.design.grid_restore_style, GridStyle::Lines);
        assert_eq!(restored.design.cycle_grid_style(), GridStyle::Dots);
        assert_eq!(restored.design.cycle_grid_style(), GridStyle::Lines);
    }

    #[test]
    fn legacy_ui_session_uses_safe_visibility_defaults() {
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serde_json::json!({}))
                .expect("legacy session without visibility policy"),
        );

        assert_eq!(
            restored.design.schematic_visibility,
            SchematicVisibilityPolicy::default()
        );
    }

    #[test]
    fn schematic_snap_and_exact_routing_round_trip_with_the_ui_session() {
        let mut session = AppState::default();
        session.design.schematic_snap.enabled = false;
        session.design.schematic_snap.snap_to_wire_segments = false;
        session.design.schematic_snap.snap_radius = 7;
        session.design.schematic_routing_mode = crate::state::WireRoutingMode::VerticalFirst;

        let serialized =
            serde_json::to_value(UiSessionWire::from(&session)).expect("serialize session");
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.design.schematic_snap,
            session.design.schematic_snap
        );
        assert_eq!(
            restored.design.schematic_routing_mode,
            crate::state::WireRoutingMode::VerticalFirst
        );
    }

    #[test]
    fn duplicate_external_nets_round_trips_with_the_ui_session() {
        let mut session = AppState::default();
        session.design.duplicate_external_nets =
            crate::state::DuplicateExternalNets::PreserveNamedNetAttachment;

        let serialized =
            serde_json::to_value(UiSessionWire::from(&session)).expect("serialize session");
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.design.duplicate_external_nets,
            crate::state::DuplicateExternalNets::PreserveNamedNetAttachment
        );
    }

    #[test]
    fn legacy_ui_session_uses_snap_and_routing_defaults() {
        let restored = AppState::from(
            serde_json::from_value::<UiSessionWire>(serde_json::json!({}))
                .expect("legacy session without editor runtime settings"),
        );

        assert_eq!(
            restored.design.schematic_snap,
            crate::state::SnapEngine::default()
        );
        assert_eq!(
            restored.design.schematic_routing_mode,
            crate::state::WireRoutingMode::HorizontalFirst
        );
        assert_eq!(
            restored.design.duplicate_external_nets,
            crate::state::DuplicateExternalNets::LeaveUnconnected
        );
    }
}

/// Workbench sessions saved and restored through the wire: missing fields,
/// retired blocks and routes, and the layouts and pages a restore keeps.
#[cfg(test)]
mod workbench_session_tests {
    use super::*;
    use crate::workbench::{BrowserHistoryEffect, RouteTransitionSource, SurfaceId, SurfaceRoute};

    fn restore(wire: WorkbenchWire) -> AppState {
        let mut state = AppState::default();
        wire.scatter(&mut state);
        state
    }

    #[test]
    fn workbench_without_a_project_page_defaults_to_overview() {
        let mut encoded =
            serde_json::to_value(WorkbenchWireRef::capture(&AppState::default())).unwrap();
        encoded
            .as_object_mut()
            .expect("workbench is an object")
            .remove("project_page");

        let restored = restore(serde_json::from_value(encoded).unwrap());

        assert_eq!(restored.project.project_page, ProjectPage::Overview);
    }

    /// A session saved while the Visualization Studio existed carries its
    /// `visualization_studio` block and may have quit on the Studio. The block
    /// below is copied from a real session file, with a pane, a selection and a
    /// cache budget added. The session must still load: the block is ignored,
    /// the Studio route is dropped with one diagnostic, and the rest of the
    /// session survives.
    #[test]
    fn session_saved_with_the_deleted_visualization_studio_still_restores() {
        const STUDIO: &str = concat!(
            "visualization_studio:(section:viewers,tool:select,zoom:2.5,",
            "selected_viewer_document:\"viewer-bode\",panes:[(id:42,viewer:Bode,",
            "viewer_document_id:\"viewer-bode\",",
            "dataset_id:\"6f1c2c1e-5b7a-4c1d-9a54-0d3c2f6b8e01\",analysis_sequence:1,",
            "x_link:Some(1),cursor_group:None,page:\"Page 1\",placement:r#below-selected)],",
            "active_pane:Some(42),next_identity:43,revision:7,annotations:[],markers:[],",
            "measurements:[],linked_x_ranges:{1:(0.0,1e-3)},linked_cursor_positions:{},",
            "pane_x_ranges:{},pane_cursor_positions:{},family_policies:{},",
            "report_page_policies:{},comparison_receipts:[],difference_trace_sets:[],",
            "autoscale:r#robust-visible,complex_projection:r#magnitude-phase,",
            "display_lod:r#envelope-preserving,tile_memory_mib:2048,significant_digits:9,",
            "touch_pane:stage)"
        );
        let mut state = AppState::default();
        state.workbench.activate(Workspace::Results);
        let saved =
            ron::to_string(&WorkbenchWireRef::capture(&state)).expect("workbench serializes");
        let saved = saved.replacen(
            "current:\"?view=results\"",
            "current:\"?surface=visualization-studio\"",
            1,
        );
        assert!(
            saved.contains("?surface=visualization-studio"),
            "the session must have quit on the Studio for this to prove anything"
        );
        // The retired engineering profile rides along, as it does in every
        // session saved before it was removed.
        let saved = format!(
            "{},engineering_profile:r#rf-microwave,{STUDIO})",
            saved
                .strip_suffix(')')
                .expect("a workbench session is a RON struct")
        );

        let mut restored = restore(
            ron::from_str(&saved).expect("a session that kept the Studio's state restores"),
        );
        restored.workbench.reconcile_restored_navigation();

        assert_eq!(
            restored.workbench.current_route(),
            SurfaceRoute::surface(SurfaceId::Design)
        );
        assert!(
            restored
                .workbench
                .take_route_diagnostic()
                .is_some_and(|message| message.contains("Malformed routes"))
        );
    }

    #[test]
    fn library_cellview_route_and_page_restore_with_models_workspace_ownership() {
        let mut state = AppState::default();
        state.project.library_cellview_page = LibraryCellviewPage::SymbolForm;
        state
            .workbench
            .navigate(
                SurfaceRoute::surface(SurfaceId::LibraryCellviewManager),
                RouteTransitionSource::User,
            )
            .expect("Library Cellview Manager has a registered executor");

        assert_eq!(state.workbench.workspace, Workspace::Models);
        assert_eq!(
            state.workbench.current_route().surface_id(),
            SurfaceId::LibraryCellviewManager
        );
        assert!(!state.application_modal_open());

        let encoded = serde_json::to_string(&WorkbenchWireRef::capture(&state))
            .expect("workbench session serializes");
        let mut restored =
            restore(serde_json::from_str(&encoded).expect("workbench session restores"));
        restored.workbench.workspace = Workspace::Project;
        restored.workbench.reconcile_restored_navigation();

        assert_eq!(restored.workbench.workspace, Workspace::Models);
        assert_eq!(
            restored.workbench.current_route().surface_id(),
            SurfaceId::LibraryCellviewManager
        );
        assert_eq!(
            restored.project.library_cellview_page,
            LibraryCellviewPage::SymbolForm
        );
    }

    #[test]
    fn legacy_session_without_library_cellview_page_defaults_to_libraries() {
        let state = restore(
            serde_json::from_str(r#"{"workspace":"Models"}"#).expect("legacy workbench restores"),
        );
        assert_eq!(
            state.project.library_cellview_page,
            LibraryCellviewPage::Libraries
        );
    }

    #[test]
    fn legacy_workspace_session_migrates_to_the_matching_primary_route() {
        let mut state = restore(
            serde_json::from_str(r#"{"workspace":"Results"}"#).expect("legacy workbench restores"),
        );
        assert_eq!(
            state.workbench.current_route().surface_id(),
            SurfaceId::Design
        );

        state.workbench.reconcile_restored_navigation();
        assert_eq!(state.workbench.workspace, Workspace::Results);
        assert_eq!(
            state.workbench.current_route().surface_id(),
            SurfaceId::Results
        );
        assert_eq!(
            state.workbench.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Replace(SurfaceRoute::surface(
                SurfaceId::Results
            )))
        );
    }

    /// A session saved while the capability matrix existed carries its
    /// `capability_matrix` block and may have the matrix open or in its history.
    /// It must still load: the stale block is ignored, the dead routes are
    /// dropped with one diagnostic, and the rest of the session survives.
    #[test]
    fn session_saved_with_the_deleted_capability_matrix_still_restores() {
        let mut state = AppState::default();
        state.workbench.activate(Workspace::Results);
        let mut saved =
            serde_json::to_value(WorkbenchWireRef::capture(&state)).expect("workbench serializes");
        saved["engineering_profile"] = serde_json::json!("rf-microwave");
        saved["capability_matrix"] = serde_json::json!({ "section": "workspaces" });
        saved["navigation"] = serde_json::json!({
            "current": "?surface=feature-availability",
            "back": ["?view=results", "?surface=touch-edit-guide"],
            "forward": [],
            "recent": ["?surface=interoperability-matrix"]
        });

        let mut restored = restore(serde_json::from_value(saved).expect("an old session restores"));
        restored.workbench.reconcile_restored_navigation();

        assert_eq!(
            restored.workbench.current_route(),
            SurfaceRoute::surface(SurfaceId::Design)
        );
        assert_eq!(
            restored.workbench.previous_route(),
            Some(SurfaceRoute::surface(SurfaceId::Results))
        );
        assert!(
            restored
                .workbench
                .take_route_diagnostic()
                .is_some_and(|message| message.contains("Malformed routes"))
        );
    }

    #[test]
    fn independent_workspace_layouts_round_trip_with_the_session() {
        let mut state = AppState::default();
        state
            .workbench
            .apply_workspace_preset(WorkspacePreset::Diagnostics);
        state.workbench.activate(Workspace::Results);
        state.workbench.apply_results_review_layout();

        let encoded = serde_json::to_string(&WorkbenchWireRef::capture(&state)).unwrap();
        let restored = restore(serde_json::from_str(&encoded).unwrap());
        assert!(
            restored
                .workbench
                .workspace_layout(Workspace::Design)
                .console_visible
        );
        assert!(
            !restored
                .workbench
                .workspace_layout(Workspace::Results)
                .navigator_visible
        );
        assert_eq!(
            restored
                .workbench
                .workspace_layout(Workspace::Results)
                .inspector_width,
            332.0
        );
    }
}
