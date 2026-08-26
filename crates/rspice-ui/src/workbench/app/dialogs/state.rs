//! App Dialog State
//!
//! Modal/dialog payload used by `AppState`. Analysis configuration lives
//! in `SimSetupState`, not here.

use crate::workbench::app::{
    ConfigurationSetsDialogState, ConfirmationDialogState, ConnectivityManagerDialogState,
    CreateModelBoundSymbolDialogState, DesignManagementDialogState,
    DesignReviewCommentsDialogState, ProjectReviewDialogState, ProjectRevisionHistoryDialogState,
    SchematicEditAuthority, SelectionBulkEditDialogState, SymbolImportDialogState,
    SymbolParameterFormDialogState,
};

mod reviews;

pub(crate) use reviews::{
    DeletionInstanceResolution, LibraryDeletionReviewState, LibraryDeletionTarget,
    PlanRemovalConsequence, PlanRemovalReview, PlanRemovalTarget, PlanRemovalTone,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ViewOperation {
    #[default]
    FullScreen,
    ResetActiveView,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum FullScreenScope {
    #[default]
    ApplicationWindow,
    ActiveCanvasOnly,
}

impl FullScreenScope {
    pub(crate) const ALL: [Self; 2] = [Self::ApplicationWindow, Self::ActiveCanvasOnly];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::ApplicationWindow => "Application window",
            Self::ActiveCanvasOnly => "Active canvas only",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum FullScreenPanels {
    #[default]
    KeepCurrent,
    HideNavigatorAndInspector,
}

impl FullScreenPanels {
    pub(crate) const ALL: [Self; 2] = [Self::KeepCurrent, Self::HideNavigatorAndInspector];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::KeepCurrent => "Keep current",
            Self::HideNavigatorAndInspector => "Hide navigator and inspector",
        }
    }
}

/// Device-local transaction for the mockup's two explicit View workflows.
///
/// Neither operation owns project data. Full-screen choices are consumed into
/// transient workbench presentation state, while reset-view captures only the
/// workspace identity needed to reject a stale commit after navigation.
#[derive(Debug, Clone)]
pub(crate) struct ViewOperationDialogState {
    pub(crate) open: bool,
    pub(crate) operation: ViewOperation,
    pub(crate) full_screen_scope: FullScreenScope,
    pub(crate) full_screen_panels: FullScreenPanels,
    pub(crate) workspace: crate::workbench::state::Workspace,
}

impl Default for ViewOperationDialogState {
    fn default() -> Self {
        Self {
            open: false,
            operation: ViewOperation::FullScreen,
            full_screen_scope: FullScreenScope::ApplicationWindow,
            full_screen_panels: FullScreenPanels::KeepCurrent,
            workspace: crate::workbench::state::Workspace::Design,
        }
    }
}

impl ViewOperationDialogState {
    pub(crate) fn open_full_screen(&mut self, workspace: crate::workbench::state::Workspace) {
        *self = Self {
            open: true,
            workspace,
            ..Self::default()
        };
    }

    pub(crate) fn open_reset_active_view(&mut self, workspace: crate::workbench::state::Workspace) {
        *self = Self {
            open: true,
            operation: ViewOperation::ResetActiveView,
            workspace,
            ..Self::default()
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ShortcutEditorContext {
    #[default]
    All,
    Global,
    Schematic,
    Simulation,
    Results,
    Verification,
}

impl ShortcutEditorContext {
    pub(crate) const ALL: [Self; 6] = [
        Self::All,
        Self::Global,
        Self::Schematic,
        Self::Simulation,
        Self::Results,
        Self::Verification,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::All => "All contexts",
            Self::Global => "Global",
            Self::Schematic => "Schematic",
            Self::Simulation => "Simulation",
            Self::Results => "Results",
            Self::Verification => "Verification",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ShortcutCaptureTarget {
    pub(crate) command: crate::workbench::commands::vocabulary::Command,
    pub(crate) slot: crate::workbench::ShortcutBindingSlot,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) enum EngineeringTableDialogPage {
    #[default]
    Manager,
    SaveView,
    SavedViews,
    Export,
    RowDetails,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum EngineeringTableExportScope {
    #[default]
    CurrentView,
    SelectedRows,
    CompleteDataset,
}

impl EngineeringTableExportScope {
    pub(crate) const ALL: [Self; 3] =
        [Self::CurrentView, Self::SelectedRows, Self::CompleteDataset];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::CurrentView => "Current view · filters, sort, visible columns",
            Self::SelectedRows => "Selected rows · current visible columns",
            Self::CompleteDataset => "Complete dataset · all logical rows and columns",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum EngineeringTableExportFormat {
    #[default]
    CsvSchema,
    Tsv,
    Xlsx,
    Parquet,
}

impl EngineeringTableExportFormat {
    pub(crate) const ALL: [Self; 4] = [Self::CsvSchema, Self::Tsv, Self::Xlsx, Self::Parquet];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::CsvSchema => "CSV + schema JSON",
            Self::Tsv => "TSV",
            Self::Xlsx => "XLSX · frozen headers and filters",
            Self::Parquet => "Parquet",
        }
    }
}

/// Transactional universal data-grid manager. The active dataset is rebuilt
/// from its owning source at open/commit time; the dialog retains only a view
/// definition and source revision, never a duplicate engineering dataset.
#[derive(Debug, Clone, Default)]
pub(crate) struct EngineeringTableDialogState {
    pub(crate) open: bool,
    pub(crate) page: EngineeringTableDialogPage,
    pub(crate) draft: Option<crate::state::EngineeringTableView>,
    pub(crate) source_revision: u64,
    pub(crate) selected_saved_id: Option<String>,
    pub(crate) save_name: String,
    pub(crate) save_scope: crate::state::EngineeringViewScope,
    pub(crate) save_as_default: bool,
    pub(crate) saved_query: String,
    pub(crate) export_scope: EngineeringTableExportScope,
    pub(crate) export_format: EngineeringTableExportFormat,
    pub(crate) export_headers: bool,
    pub(crate) export_units: bool,
    pub(crate) export_metadata: bool,
    pub(crate) export_hidden_columns: bool,
    pub(crate) selected_row_ids: std::collections::BTreeSet<String>,
    pub(crate) active_cell: Option<(usize, usize)>,
    pub(crate) focus_cell: Option<(usize, usize)>,
    pub(crate) row_details_id: Option<String>,
    pub(crate) saved_edit_id: Option<String>,
    pub(crate) saved_edit_name: String,
    #[cfg(target_arch = "wasm32")]
    pub(crate) saved_import_token: Option<crate::workbench::browser::file_import::TextImportToken>,
    pub(crate) error: Option<String>,
}

impl EngineeringTableDialogState {
    pub(crate) fn open(&mut self, view: crate::state::EngineeringTableView, source_revision: u64) {
        *self = Self {
            open: true,
            draft: Some(view),
            source_revision,
            export_headers: true,
            export_units: true,
            export_metadata: true,
            ..Self::default()
        };
    }

    pub(crate) fn close(&mut self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(token) = self.saved_import_token.take() {
            let _ = crate::workbench::browser::file_import::finish_text_import(token);
        }
        *self = Self::default();
    }
}

/// Retained transaction state for the command-binding editor. Visible
/// controls mutate `draft`; the live profile is replaced only by validated
/// Save.
#[derive(Debug, Clone, Default)]
pub(crate) struct ShortcutEditorState {
    pub(crate) open: bool,
    pub(crate) original: Option<crate::workbench::ShortcutPreferences>,
    pub(crate) draft: Option<crate::workbench::ShortcutPreferences>,
    pub(crate) query: String,
    pub(crate) context: ShortcutEditorContext,
    pub(crate) selected_command: Option<crate::workbench::commands::vocabulary::Command>,
    pub(crate) recording: Option<ShortcutCaptureTarget>,
    pub(crate) capture_strokes: Vec<crate::workbench::ShortcutStroke>,
    pub(crate) capture_last_input_at: Option<f64>,
    pub(crate) dirty: bool,
    /// A browser CAS is in flight. The draft remains isolated and the editor
    /// cannot close or launch a second publication until completion.
    pub(crate) persistence_pending: bool,
    pub(crate) discard_confirmation: bool,
    pub(crate) error_summary: Option<String>,
    pub(crate) repair_receipt: Option<String>,
    pub(crate) focus_error: Option<ShortcutCaptureTarget>,
    pub(crate) body_scroll_offset: f32,
}

impl ShortcutEditorState {
    pub(crate) fn open(&mut self, profile: &crate::workbench::ShortcutPreferences) {
        *self = Self {
            open: true,
            original: Some(profile.clone()),
            draft: Some(profile.clone()),
            ..Self::default()
        };
    }

    pub(crate) fn close_and_discard(&mut self) {
        *self = Self::default();
    }
}

/// Where the license-activation flow stands.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum LicensePhase {
    /// Waiting for a key.
    #[default]
    Entry,
    /// Last verification failed; the message renders under the field.
    Error(String),
    /// Key verified — summary pane with the grant, primary = Activate.
    Verified(crate::services::license::LicenseInfo),
}

/// License activation dialog state.
#[derive(Debug, Clone, Default)]
pub struct LicenseDialogState {
    /// Whether the dialog is showing.
    pub open: bool,
    /// The pasted key text.
    pub text: String,
    /// Current phase of the flow.
    pub phase: LicensePhase,
}

/// Command palette (Ctrl+K) state.
#[derive(Debug, Clone, Default)]
pub struct CommandPaletteState {
    /// Whether the palette is showing.
    pub open: bool,
    /// Filter text.
    pub query: String,
    /// Index into the filtered list.
    pub selected: usize,
    /// Request keyboard focus on the next frame (set when opened).
    pub want_focus: bool,
    /// Optional reviewed scope selected by a task-focused menu entry. This is
    /// consumed when the palette opens; ordinary Ctrl+K starts at All.
    pub(crate) initial_scope: Option<String>,
    /// Commands run from the palette, newest first — leads the empty-query
    /// list under a RECENT header. Survives close/reopen, capped at five.
    pub(crate) recent: Vec<crate::workbench::commands::vocabulary::Command>,
}

/// Retained draft for the mockup-owned project technology transaction.
/// Selection remains isolated until the primary action verifies every pinned
/// source and commits one project metadata revision.
#[derive(Debug, Clone, Default)]
pub(crate) struct TechnologyAttachmentDialogState {
    pub(crate) open: bool,
    pub(crate) selected_library: Option<String>,
    pub(crate) selected_signed_package: Option<String>,
    pub(crate) actor_id: String,
    pub(crate) authority_id: String,
    pub(crate) reason: String,
    pub(crate) migration_reviewed: bool,
    pub(crate) validation_error: Option<String>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) checkpoint_pending: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProjectCheckpointRecoveryState {
    pub(crate) project_id: Option<String>,
    pub(crate) checkpoints:
        Vec<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary>,
    pub(crate) quarantined:
        Vec<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointQuarantine>,
    pub(crate) error: Option<String>,
    pub(crate) initialized: bool,
    #[cfg(target_arch = "wasm32")]
    pub(crate) loading: bool,
}

impl ProjectCheckpointRecoveryState {
    pub(crate) fn invalidate(&mut self) {
        self.initialized = false;
        self.error = None;
    }
}

impl TechnologyAttachmentDialogState {
    pub(crate) fn open(
        &mut self,
        selected_library: Option<String>,
        selected_signed_package: Option<String>,
    ) {
        *self = Self {
            open: true,
            selected_library,
            selected_signed_package,
            actor_id: String::new(),
            authority_id: String::new(),
            reason: String::new(),
            migration_reviewed: false,
            validation_error: None,
            #[cfg(target_arch = "wasm32")]
            checkpoint_pending: false,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

impl CommandPaletteState {
    /// Open fresh (empty query, first row selected, focus requested).
    pub fn open(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
        self.want_focus = true;
        self.initial_scope = None;
    }

    pub(crate) fn open_in_scope(&mut self, scope: &str) {
        self.open();
        self.initial_scope = Some(scope.to_owned());
    }
}

/// Retained, isolated draft for the mockup-owned bus-tap placement
/// transaction. The parsed electrical contract is not published to the
/// schematic until the primary action validates every field.
#[derive(Debug, Clone)]
pub(crate) struct BusTapDialogState {
    pub(crate) open: bool,
    pub(crate) bus: String,
    pub(crate) slice: String,
    pub(crate) orientation: crate::state::BusTapOrientation,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

impl Default for BusTapDialogState {
    fn default() -> Self {
        Self {
            open: false,
            bus: String::new(),
            slice: String::new(),
            orientation: crate::state::BusTapOrientation::Automatic,
            dirty: false,
            discard_confirm: false,
        }
    }
}

impl BusTapDialogState {
    pub(crate) fn open(&mut self) {
        *self = Self {
            open: true,
            bus: "DATA[15:0]".to_owned(),
            slice: "DATA[7:0]".to_owned(),
            orientation: crate::state::BusTapOrientation::Automatic,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    /// Record an input edit without publishing any schematic state. Editing
    /// after the first close attempt returns the footer to its ordinary
    /// Cancel state, matching the mockup workflow lifecycle.
    pub(crate) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
    }

    /// Try to close the isolated draft. An edited draft requires the same
    /// second explicit discard action as the mockup workflow shell.
    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Isolated pre-placement contract for a registry-backed XSPICE device with
/// one or more vector ports. Width edits remain runtime-only until the
/// primary action materializes and validates the exact durable binding.
#[derive(Debug, Clone, Default)]
pub(crate) struct BuiltinXspicePlacementDialogState {
    pub(crate) open: bool,
    pub(crate) stable_id: String,
    pub(crate) display_name: String,
    pub(crate) vector_ports: Vec<crate::state::CatalogXspiceVectorPort>,
    pub(crate) widths: std::collections::BTreeMap<String, usize>,
    pub(crate) design_execution_epoch: u64,
    pub(crate) active_schematic_epoch: u64,
    pub(crate) view_path: String,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
    pub(crate) validation_error: Option<String>,
}

impl BuiltinXspicePlacementDialogState {
    pub(crate) fn open(
        &mut self,
        stable_id: impl Into<String>,
        display_name: impl Into<String>,
        vector_ports: Vec<crate::state::CatalogXspiceVectorPort>,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: impl Into<String>,
    ) {
        let widths = vector_ports
            .iter()
            .map(|port| (port.name.clone(), port.default_width))
            .collect();
        *self = Self {
            open: true,
            stable_id: stable_id.into(),
            display_name: display_name.into(),
            vector_ports,
            widths,
            design_execution_epoch,
            active_schematic_epoch,
            view_path: view_path.into(),
            dirty: false,
            discard_confirm: false,
            validation_error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
        self.validation_error = None;
    }

    pub(crate) fn attempt_close(&mut self) {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
        } else {
            self.close();
        }
    }
}

/// Isolated draft for one snapped, typed net-label placement.
///
/// The canvas click supplies only the anchor and immutable document
/// authority. No durable schematic object exists until the dialog's primary
/// action validates and publishes the complete label.
#[derive(Debug, Clone, Default)]
pub(crate) struct NetLabelPlacementDialogState {
    pub(crate) open: bool,
    pub(crate) name: String,
    pub(crate) anchor: Option<crate::state::Point>,
    pub(crate) authority: Option<SchematicEditAuthority>,
    /// Which label the armed tool is placing. The direction of an off-sheet
    /// connector is part of the draft, so it publishes with the name in one
    /// transaction rather than needing a second edit.
    pub(crate) kind: crate::state::NetLabelKind,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
    pub(crate) validation_error: Option<String>,
}

impl NetLabelPlacementDialogState {
    pub(crate) fn open(
        &mut self,
        anchor: crate::state::Point,
        authority: SchematicEditAuthority,
        kind: crate::state::NetLabelKind,
    ) {
        *self = Self {
            open: true,
            name: String::new(),
            anchor: Some(anchor),
            authority: Some(authority),
            kind,
            dirty: false,
            discard_confirm: false,
            validation_error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
        self.validation_error = None;
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Isolated draft for the View ▸ Hierarchy and annotation visibility
/// transaction.
///
/// The draft is device-local presentation state. Apply publishes all seven
/// controls atomically; Cancel discards them without touching schematic
/// history, topology, project serialization, or retained result data.
/// Governed snap-spacing choices shown by the upgraded schematic toolbar
/// transaction. `Free` disables snapping without replacing the document's
/// retained pitch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum GridSnapSpacingChoice {
    #[default]
    Free,
    Mil25,
    Mil50,
    Metric,
}

impl GridSnapSpacingChoice {
    pub(crate) const ALL: [Self; 4] = [Self::Free, Self::Mil25, Self::Mil50, Self::Metric];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Free => "Free",
            Self::Mil25 => "25 mil",
            Self::Mil50 => "50 mil",
            Self::Metric => "Metric \u{00b7} 0.5 mm",
        }
    }

    pub(crate) const fn pitch(self) -> Option<crate::state::SchematicGridPitch> {
        match self {
            Self::Free => None,
            Self::Mil25 => Some(crate::state::SchematicGridPitch::Mil25),
            Self::Mil50 => Some(crate::state::SchematicGridPitch::Mil50),
            Self::Metric => Some(crate::state::SchematicGridPitch::Metric),
        }
    }

    pub(crate) const fn from_pitch(pitch: crate::state::SchematicGridPitch) -> Self {
        match pitch {
            crate::state::SchematicGridPitch::Mil25 => Self::Mil25,
            crate::state::SchematicGridPitch::Mil50 => Self::Mil50,
            crate::state::SchematicGridPitch::Metric => Self::Metric,
        }
    }
}

/// Complete isolated candidate published by Grid, snap and wire routing.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GridSnapRoutingDraft {
    pub(crate) grid_style: crate::state::GridStyle,
    pub(crate) snap_spacing: GridSnapSpacingChoice,
    pub(crate) snap_engine: crate::state::SnapEngine,
    pub(crate) wire_routing: crate::state::WireRoutingMode,
}

impl Default for GridSnapRoutingDraft {
    fn default() -> Self {
        Self {
            grid_style: crate::state::GridStyle::Dots,
            snap_spacing: GridSnapSpacingChoice::Mil50,
            snap_engine: crate::state::SnapEngine::default(),
            wire_routing: crate::state::WireRoutingMode::HorizontalFirst,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GridSnapRoutingFocusTarget {
    SnapSpacing,
    SnapTargets,
}

/// Application-modal transactional owner for the schematic canvas settings
/// popover. The original and draft are kept separately so Cancel is exact.
#[derive(Debug, Clone, Default)]
pub(crate) struct GridSnapRoutingDialogState {
    pub(crate) open: bool,
    pub(crate) draft: GridSnapRoutingDraft,
    pub(crate) original: GridSnapRoutingDraft,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) validation_error: Option<String>,
    pub(crate) focus_target: Option<GridSnapRoutingFocusTarget>,
}

impl GridSnapRoutingDialogState {
    pub(crate) fn open(&mut self, draft: GridSnapRoutingDraft, authority: SchematicEditAuthority) {
        *self = Self {
            open: true,
            original: draft.clone(),
            draft,
            authority: Some(authority),
            validation_error: None,
            focus_target: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn dirty(&self) -> bool {
        self.open && self.draft != self.original
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SchematicVisibilityDialogState {
    pub(crate) open: bool,
    pub(crate) draft: crate::state::SchematicVisibilityPolicy,
    pub(crate) original: crate::state::SchematicVisibilityPolicy,
}

impl SchematicVisibilityDialogState {
    pub(crate) fn open(&mut self, policy: crate::state::SchematicVisibilityPolicy) {
        *self = Self {
            open: true,
            draft: policy,
            original: policy,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn dirty(&self) -> bool {
        self.open && self.draft != self.original
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DrawingSheetLayersDialogState {
    pub(crate) open: bool,
    pub(crate) draft: crate::state::DrawingSheetLayerVisibility,
}

impl DrawingSheetLayersDialogState {
    pub(crate) fn open(&mut self, layers: crate::state::DrawingSheetLayerVisibility) {
        *self = Self {
            open: true,
            draft: layers,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HierarchyDescendEditMode {
    #[default]
    EditInPlace,
    OpenIsolated,
    ReadOnlyReference,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HierarchyParentContext {
    #[default]
    ShowOneLevel,
    ShowFullHierarchy,
    HideParent,
}

/// Isolated task draft for Design ▸ Descend into selected instance.
///
/// Direct toolbar, inspector, double-click, and Shift+E gestures use the
/// user's hierarchy preference. The menu workflow owns the explicit
/// edit-context choices defined by the upgraded mockup.
#[derive(Debug, Clone, Default)]
pub(crate) struct DescendHierarchyDialogState {
    pub(crate) open: bool,
    pub(crate) instance_name: String,
    pub(crate) reference: Option<crate::state::CellViewRef>,
    pub(crate) edit_mode: HierarchyDescendEditMode,
    pub(crate) parent_context: HierarchyParentContext,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) parent_dirty: bool,
    pub(crate) validation_error: Option<String>,
}

impl DescendHierarchyDialogState {
    pub(crate) fn open(
        &mut self,
        instance_name: String,
        reference: crate::state::CellViewRef,
        authority: SchematicEditAuthority,
        parent_dirty: bool,
        preferred_edit_mode: HierarchyDescendEditMode,
    ) {
        *self = Self {
            open: true,
            instance_name,
            reference: Some(reference),
            edit_mode: preferred_edit_mode,
            parent_context: HierarchyParentContext::ShowOneLevel,
            authority: Some(authority),
            parent_dirty,
            validation_error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// Isolated draft for the mockup-owned Place pin or port transaction.
#[derive(Debug, Clone, Default)]
pub(crate) struct PinPortDialogState {
    pub(crate) open: bool,
    pub(crate) name: String,
    pub(crate) direction_type: crate::state::PortDirectionType,
    pub(crate) discipline: crate::state::PortDiscipline,
    pub(crate) design_execution_epoch: u64,
    pub(crate) active_schematic_epoch: u64,
    pub(crate) topology_version: u64,
    pub(crate) view_path: String,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

impl PinPortDialogState {
    pub(crate) fn open(
        &mut self,
        name: String,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        *self = Self {
            open: true,
            name,
            direction_type: crate::state::PortDirectionType::default(),
            discipline: crate::state::PortDiscipline::default(),
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Isolated draft for the mockup-owned Place text or design note transaction.
#[derive(Debug, Clone, Default)]
pub(crate) struct DesignNoteDialogState {
    pub(crate) open: bool,
    pub(crate) kind: crate::state::DesignNoteKind,
    pub(crate) text: String,
    pub(crate) design_execution_epoch: u64,
    pub(crate) active_schematic_epoch: u64,
    pub(crate) topology_version: u64,
    pub(crate) view_path: String,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

impl DesignNoteDialogState {
    pub(crate) fn open(
        &mut self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        *self = Self {
            open: true,
            kind: crate::state::DesignNoteKind::PlainText,
            text: "Bias network".to_owned(),
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Isolated draft for the mockup-owned Draw documentation shape transaction.
#[derive(Debug, Clone, Default)]
pub(crate) struct DocumentationShapeDialogState {
    pub(crate) open: bool,
    pub(crate) kind: crate::state::DocumentationShapeKind,
    pub(crate) design_execution_epoch: u64,
    pub(crate) active_schematic_epoch: u64,
    pub(crate) topology_version: u64,
    pub(crate) view_path: String,
    pub(crate) expected_shapes: Vec<crate::state::DocumentationShape>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

impl DocumentationShapeDialogState {
    pub(crate) fn open(
        &mut self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
        expected_shapes: Vec<crate::state::DocumentationShape>,
    ) {
        *self = Self {
            open: true,
            kind: crate::state::DocumentationShapeKind::Rectangle,
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            expected_shapes,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Isolated authority and interaction state for the mockup-owned Move
/// selection transaction. The design snapshot remains immutable while the
/// dialog is open and while the tool is armed; pointer/keyboard movement is
/// accumulated as a preview delta and committed only once.
#[derive(Debug, Clone, Default)]
pub(crate) struct MoveSelectionDialogState {
    pub(crate) open: bool,
    pub(crate) armed: bool,
    pub(crate) mode: crate::state::MoveSelectionMode,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) anchor: Option<crate::state::Point>,
    pub(crate) preview_delta: crate::state::Point,
    pub(crate) pointer_drag: bool,
    pub(crate) preview_error: Option<String>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

impl MoveSelectionDialogState {
    pub(crate) fn open(&mut self, authority: SchematicEditAuthority) {
        *self = Self {
            open: true,
            armed: false,
            mode: crate::state::MoveSelectionMode::Connected,
            authority: Some(authority),
            anchor: None,
            preview_delta: crate::state::Point::origin(),
            pointer_drag: false,
            preview_error: None,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn arm(&mut self) {
        self.open = false;
        self.armed = true;
        self.anchor = None;
        self.preview_delta = crate::state::Point::origin();
        self.pointer_drag = false;
        self.preview_error = None;
        self.dirty = false;
        self.discard_confirm = false;
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Isolated authority and interaction state for the mockup-owned Stretch
/// selection transaction. The selected conductor segment or documentation
/// control point is resolved before mutation and remains stable through the
/// live canvas preview and atomic commit.
#[derive(Debug, Clone, Default)]
pub(crate) struct StretchSelectionDialogState {
    pub(crate) open: bool,
    pub(crate) armed: bool,
    pub(crate) policy: crate::state::StretchOrthogonalPolicy,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) target: Option<crate::state::StretchTarget>,
    pub(crate) anchor: Option<crate::state::Point>,
    pub(crate) preview_delta: crate::state::Point,
    pub(crate) pointer_drag: bool,
    pub(crate) preview_error: Option<String>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

impl StretchSelectionDialogState {
    pub(crate) fn open(
        &mut self,
        authority: SchematicEditAuthority,
        target: crate::state::StretchTarget,
    ) {
        *self = Self {
            open: true,
            armed: false,
            policy: crate::state::StretchOrthogonalPolicy::default(),
            authority: Some(authority),
            target: Some(target),
            anchor: None,
            preview_delta: crate::state::Point::origin(),
            pointer_drag: false,
            preview_error: None,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn arm(&mut self) {
        self.open = false;
        self.armed = true;
        self.anchor = None;
        self.preview_delta = crate::state::Point::origin();
        self.pointer_drag = false;
        self.preview_error = None;
        self.dirty = false;
        self.discard_confirm = false;
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = true;
        self.discard_confirm = false;
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Isolated authority, typed draft, and canvas interaction state for the
/// mockup-owned Create object array transaction. The complete replica set is
/// validated as one candidate and the live document remains unchanged until
/// the canvas commit boundary succeeds.
#[derive(Debug, Clone)]
pub(crate) struct ArraySelectionPreviewCache {
    pub(crate) plan: crate::state::SchematicArrayPlan,
    pub(crate) library_revision: u64,
    pub(crate) symbol_revision: u64,
    pub(crate) identity_cursor: u64,
    pub(crate) preview: Result<crate::state::SchematicArrayPreview, String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ArraySelectionDialogState {
    pub(crate) open: bool,
    pub(crate) armed: bool,
    pub(crate) kind: crate::state::SchematicArrayKind,
    pub(crate) count: String,
    pub(crate) naming: String,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) anchor: Option<crate::state::Point>,
    pub(crate) preview_delta: crate::state::Point,
    pub(crate) pointer_drag: bool,
    pub(crate) preview_error: Option<String>,
    pub(crate) preview_cache: Option<ArraySelectionPreviewCache>,
    pub(crate) initial_kind: crate::state::SchematicArrayKind,
    pub(crate) initial_count: String,
    pub(crate) initial_naming: String,
    pub(crate) validation_field_mask: u8,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

impl ArraySelectionDialogState {
    pub(crate) fn open(
        &mut self,
        authority: SchematicEditAuthority,
        count: String,
        naming: String,
    ) {
        let initial_count = count.clone();
        let initial_naming = naming.clone();
        *self = Self {
            open: true,
            armed: false,
            kind: crate::state::SchematicArrayKind::default(),
            count,
            naming,
            authority: Some(authority),
            anchor: None,
            preview_delta: crate::state::Point::origin(),
            pointer_drag: false,
            preview_error: None,
            preview_cache: None,
            initial_kind: crate::state::SchematicArrayKind::default(),
            initial_count,
            initial_naming,
            validation_field_mask: 0,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn arm(&mut self) {
        self.open = false;
        self.armed = true;
        self.anchor = None;
        self.preview_delta = crate::state::Point::origin();
        self.pointer_drag = false;
        self.preview_error = None;
        self.preview_cache = None;
        self.dirty = false;
        self.discard_confirm = false;
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = self.kind != self.initial_kind
            || !equivalent_array_count(&self.count, &self.initial_count)
            || !equivalent_array_naming(&self.naming, &self.initial_naming);
        self.discard_confirm = false;
        self.preview_cache = None;
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

fn equivalent_array_count(left: &str, right: &str) -> bool {
    left == right
        || crate::state::SchematicArrayCount::parse(left)
            .ok()
            .zip(crate::state::SchematicArrayCount::parse(right).ok())
            .is_some_and(|(left, right)| left == right)
}

fn equivalent_array_naming(left: &str, right: &str) -> bool {
    left == right
        || crate::state::SchematicArrayNaming::parse(left)
            .ok()
            .zip(crate::state::SchematicArrayNaming::parse(right).ok())
            .is_some_and(|(left, right)| left == right)
}

/// Isolated authority and authored draft for the mockup-owned Replace
/// instance transaction. The visible Current and Mapping values are resolved
/// facts; Replacement is the only editable field. The selected component's
/// stable identifier is retained so the commit cannot drift to a later
/// selection.
#[derive(Debug, Clone, Default)]
pub(crate) struct ReplaceInstanceDialogState {
    pub(crate) open: bool,
    pub(crate) source_component_id: u64,
    pub(crate) current: String,
    pub(crate) replacement: String,
    pub(crate) mapping: String,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) replacement_authority: Option<crate::state::SchematicReplacementAuthority>,
    pub(crate) initial_target_identity: String,
    pub(crate) initial_target_spec: Option<crate::state::SchematicReplacementTargetSpec>,
    pub(crate) initial_replacement: String,
    pub(crate) preview_error: Option<String>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

pub(crate) struct ReplaceInstanceOpen {
    pub(crate) authority: SchematicEditAuthority,
    pub(crate) replacement_authority: crate::state::SchematicReplacementAuthority,
    pub(crate) source_component_id: u64,
    pub(crate) current: String,
    pub(crate) replacement: String,
    pub(crate) initial_target_identity: String,
    pub(crate) initial_target_spec: crate::state::SchematicReplacementTargetSpec,
    pub(crate) mapping: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CreateHierarchyPortDraft {
    pub(crate) name: String,
    pub(crate) direction: crate::state::PortDirection,
    pub(crate) discipline: crate::state::PortDiscipline,
    pub(crate) source_net: String,
}

/// Isolated draft and complete source authority for Create hierarchy from
/// selection. The live parent, library, and workspace remain untouched until
/// the primary action validates and swaps a complete project candidate.
#[derive(Debug, Clone, Default)]
pub(crate) struct CreateHierarchyDialogState {
    pub(crate) open: bool,
    pub(crate) cell_name: String,
    pub(crate) target_view: crate::state::ViewType,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) plan: Option<crate::state::HierarchyExtractionPlan>,
    pub(crate) ports: Vec<CreateHierarchyPortDraft>,
    pub(crate) library_revision: u64,
    pub(crate) initial_cell_name: String,
    pub(crate) initial_target_view: crate::state::ViewType,
    pub(crate) initial_ports: Vec<CreateHierarchyPortDraft>,
    pub(crate) validation_error: Option<String>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
}

impl CreateHierarchyDialogState {
    pub(crate) fn open(
        &mut self,
        authority: SchematicEditAuthority,
        plan: crate::state::HierarchyExtractionPlan,
        library_revision: u64,
    ) {
        let ports = plan
            .ports
            .iter()
            .map(|port| CreateHierarchyPortDraft {
                name: port.name.clone(),
                direction: port.direction,
                discipline: port.discipline,
                source_net: port.source_net.clone(),
            })
            .collect::<Vec<_>>();
        *self = Self {
            open: true,
            cell_name: "sensor_frontend".to_owned(),
            target_view: crate::state::ViewType::Schematic,
            authority: Some(authority),
            plan: Some(plan),
            ports: ports.clone(),
            library_revision,
            initial_cell_name: "sensor_frontend".to_owned(),
            initial_target_view: crate::state::ViewType::Schematic,
            initial_ports: ports,
            validation_error: None,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = self.cell_name != self.initial_cell_name
            || self.target_view != self.initial_target_view
            || self.ports != self.initial_ports;
        self.discard_confirm = false;
        self.validation_error = None;
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Retained review state for the mockup-owned Check and save transaction.
/// Validation evidence is frozen when the dialog opens and re-derived before
/// publication; the editable revision note never mutates the live document
/// until a canonical save request has been accepted.
#[derive(Debug, Clone, Default)]
pub(crate) struct CheckAndSaveDialogState {
    pub(crate) open: bool,
    pub(crate) revision_note: String,
    pub(crate) revision_note_touched: bool,
    pub(crate) report: Option<
        crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport,
    >,
    pub(crate) validation_error: Option<String>,
    pub(crate) save_receipt: Option<String>,
    pub(crate) saved_with_newer_changes: bool,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_transaction: Option<crate::product::TransactionId>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_revision_id: Option<crate::state::ValidatedSchematicRevisionId>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_view_key: Option<String>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_original_journal: Option<crate::state::ValidatedRevisionJournal>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_expected_journal: Option<crate::state::ValidatedRevisionJournal>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_expected_design_digest: Option<crate::product::ContentDigest>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) pending_original_dirty: bool,
}

impl CheckAndSaveDialogState {
    pub(crate) fn open(
        &mut self,
        report: crate::workbench::app::dialogs::check_and_save_validation::CheckAndSaveValidationReport,
    ) {
        *self = Self {
            open: true,
            report: Some(report),
            ..Self::default()
        };
    }

    pub(crate) fn mark_edited(&mut self) {
        if self.save_receipt.is_some() || self.is_pending() {
            return;
        }
        self.dirty = !self.revision_note.is_empty();
        self.revision_note_touched = true;
        self.discard_confirm = false;
        self.validation_error = None;
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.is_pending() {
            return false;
        }
        if self.save_receipt.is_some() {
            self.close();
            return true;
        }
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }

    pub(crate) fn is_pending(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            self.pending_transaction.is_some()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    }
}

impl ReplaceInstanceDialogState {
    pub(crate) fn open(&mut self, request: ReplaceInstanceOpen) {
        let initial_replacement = request.replacement.clone();
        *self = Self {
            open: true,
            source_component_id: request.source_component_id,
            current: request.current,
            replacement: request.replacement,
            mapping: request.mapping,
            authority: Some(request.authority),
            replacement_authority: Some(request.replacement_authority),
            initial_target_identity: request.initial_target_identity,
            initial_target_spec: Some(request.initial_target_spec),
            initial_replacement,
            preview_error: None,
            dirty: false,
            discard_confirm: false,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = !self
            .replacement
            .trim()
            .eq_ignore_ascii_case(self.initial_replacement.trim());
        self.discard_confirm = false;
        self.preview_error = None;
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

/// Exact, isolated draft for a selected bus. The durable baseline is retained
/// to guard the eventual commit against stale-object overwrite.
#[derive(Debug, Clone)]
pub(crate) struct BusObjectPropertiesDraft {
    pub(crate) original: crate::state::Bus,
    pub(crate) declaration: String,
}

/// Exact, isolated draft for a selected typed bus tap.
#[derive(Debug, Clone)]
pub(crate) struct BusTapObjectPropertiesDraft {
    pub(crate) original: crate::state::BusTap,
    pub(crate) source_bus_id: u64,
    pub(crate) slice: String,
    pub(crate) orientation: crate::state::BusTapOrientation,
}

/// Exact, isolated draft for a selected net label. Coordinates are retained as
/// text until Primary so incomplete or out-of-range edits never partially move
/// the electrical attachment point.
#[derive(Debug, Clone)]
pub(crate) struct NetLabelObjectPropertiesDraft {
    pub(crate) original: crate::state::NetLabel,
    pub(crate) name: String,
    pub(crate) x: String,
    pub(crate) y: String,
}

/// Exact naming authority for a logical net selected through conductor
/// geometry. Unlike a label draft, this has no glyph position to edit: the
/// transaction updates every captured label/port name while retaining IDs.
#[derive(Debug, Clone)]
pub(crate) struct NamedNetObjectPropertiesDraft {
    pub(crate) original: crate::workbench::app::NamedNetTarget,
    pub(crate) name: String,
}

/// Exact isolated draft for one non-electrical schematic documentation object.
#[derive(Debug, Clone)]
pub(crate) struct DesignNoteObjectPropertiesDraft {
    pub(crate) original: crate::state::DesignNote,
    pub(crate) kind: crate::state::DesignNoteKind,
    pub(crate) text: String,
    pub(crate) review_state: Option<crate::state::DesignReviewState>,
}

#[derive(Debug, Clone)]
pub(crate) struct DocumentationShapeObjectPropertiesDraft {
    pub(crate) original: crate::state::DocumentationShape,
    pub(crate) points: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub(crate) enum ObjectPropertiesDraft {
    Bus(BusObjectPropertiesDraft),
    BusTap(BusTapObjectPropertiesDraft),
    NetLabel(NetLabelObjectPropertiesDraft),
    NamedNet(NamedNetObjectPropertiesDraft),
    DesignNote(DesignNoteObjectPropertiesDraft),
    DocumentationShape(DocumentationShapeObjectPropertiesDraft),
}

/// Retained owner for the mockup's generic Object properties transaction.
/// Components continue to use their schema-driven tabbed editor; geometric
/// connectivity objects and logical named nets use this draft because their
/// invariants span stable authority, exact coordinates, and neighboring
/// topology.
#[derive(Debug, Clone, Default)]
pub(crate) struct ObjectPropertiesDialogState {
    pub(crate) open: bool,
    pub(crate) draft: Option<ObjectPropertiesDraft>,
    /// Document generation captured when the isolated editor opened. This
    /// prevents a matching object id in a replacement document from being
    /// mistaken for the original transaction target.
    pub(crate) design_execution_epoch: u64,
    pub(crate) active_schematic_epoch: u64,
    /// Scoped dependency generation captured with the bus/tap draft.
    pub(crate) topology_version: u64,
    /// Exact cell/view identity captured with the draft.
    pub(crate) view_path: String,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
    pub(crate) validation_error: Option<String>,
}

/// Stable schematic object or exact logical-net naming authority captured when
/// Edit ▸ Rename selected object opens.
///
/// Retaining the complete source value is intentional: the primary action can
/// distinguish the original object from a later object that happens to reuse
/// the same numeric ID, and it can reject any concurrent edit without
/// overwriting it.
#[derive(Debug, Clone)]
pub(crate) enum RenameSelectionTarget {
    Component(Box<crate::state::Component>),
    NetLabel(crate::state::NetLabel),
    NamedNet(crate::workbench::app::NamedNetTarget),
    Bus(crate::state::Bus),
}

impl RenameSelectionTarget {
    pub(crate) fn current_name(&self) -> &str {
        match self {
            Self::Component(component) => &component.name,
            Self::NetLabel(label) => &label.name,
            Self::NamedNet(net) => &net.name,
            Self::Bus(bus) => bus
                .declaration
                .as_ref()
                .map_or("", |declaration| declaration.name.as_str()),
        }
    }
}

/// Isolated draft and revision guard for the mockup-owned stable-identity
/// rename workflow. No schematic field is changed until Primary validates and
/// publishes the draft.
#[derive(Debug, Clone, Default)]
pub(crate) struct RenameSelectionDialogState {
    pub(crate) open: bool,
    pub(crate) target: Option<RenameSelectionTarget>,
    pub(crate) draft: String,
    pub(crate) design_execution_epoch: u64,
    pub(crate) active_schematic_epoch: u64,
    pub(crate) topology_version: u64,
    pub(crate) view_path: String,
    pub(crate) validation_error: Option<String>,
}

impl RenameSelectionDialogState {
    pub(crate) fn open(
        &mut self,
        target: RenameSelectionTarget,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        let draft = target.current_name().to_owned();
        *self = Self {
            open: true,
            target: Some(target),
            draft,
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            validation_error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

impl ObjectPropertiesDialogState {
    pub(crate) fn open_bus(
        &mut self,
        bus: &crate::state::Bus,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        *self = Self {
            open: true,
            draft: Some(ObjectPropertiesDraft::Bus(BusObjectPropertiesDraft {
                original: bus.clone(),
                declaration: bus
                    .declaration
                    .as_ref()
                    .map_or_else(String::new, ToString::to_string),
            })),
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            dirty: false,
            discard_confirm: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_bus_tap(
        &mut self,
        tap: &crate::state::BusTap,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        *self = Self {
            open: true,
            draft: Some(ObjectPropertiesDraft::BusTap(BusTapObjectPropertiesDraft {
                original: tap.clone(),
                source_bus_id: tap.bus_id,
                slice: tap.slice.to_string(),
                orientation: tap.orientation,
            })),
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            dirty: false,
            discard_confirm: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_net_label(
        &mut self,
        label: &crate::state::NetLabel,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        *self = Self {
            open: true,
            draft: Some(ObjectPropertiesDraft::NetLabel(
                NetLabelObjectPropertiesDraft {
                    original: label.clone(),
                    name: label.name.clone(),
                    x: label.pos.x.to_string(),
                    y: label.pos.y.to_string(),
                },
            )),
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            dirty: false,
            discard_confirm: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_named_net(
        &mut self,
        target: crate::workbench::app::NamedNetTarget,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        let name = target.name.clone();
        *self = Self {
            open: true,
            draft: Some(ObjectPropertiesDraft::NamedNet(
                NamedNetObjectPropertiesDraft {
                    original: target,
                    name,
                },
            )),
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            dirty: false,
            discard_confirm: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_design_note(
        &mut self,
        note: &crate::state::DesignNote,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        *self = Self {
            open: true,
            draft: Some(ObjectPropertiesDraft::DesignNote(
                DesignNoteObjectPropertiesDraft {
                    original: note.clone(),
                    kind: note.kind,
                    text: note.text.clone(),
                    review_state: note.review.as_ref().map(|review| review.state),
                },
            )),
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            dirty: false,
            discard_confirm: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_documentation_shape(
        &mut self,
        shape: &crate::state::DocumentationShape,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        topology_version: u64,
        view_path: String,
    ) {
        *self = Self {
            open: true,
            draft: Some(ObjectPropertiesDraft::DocumentationShape(
                DocumentationShapeObjectPropertiesDraft {
                    original: shape.clone(),
                    points: shape
                        .geometry
                        .points()
                        .into_iter()
                        .map(|point| (point.x.to_string(), point.y.to_string()))
                        .collect(),
                },
            )),
            design_execution_epoch,
            active_schematic_epoch,
            topology_version,
            view_path,
            dirty: false,
            discard_confirm: false,
            validation_error: None,
        };
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = self
            .draft
            .as_ref()
            .is_some_and(ObjectPropertiesDraft::is_modified);
        self.discard_confirm = false;
        self.validation_error = None;
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn attempt_close(&mut self) -> bool {
        if self.dirty && !self.discard_confirm {
            self.discard_confirm = true;
            false
        } else {
            self.close();
            true
        }
    }
}

impl ObjectPropertiesDraft {
    pub(crate) fn is_modified(&self) -> bool {
        match self {
            Self::Bus(draft) => {
                let text = draft.declaration.trim();
                let candidate = if text.is_empty() {
                    Some(None)
                } else {
                    crate::state::BusDeclaration::parse(text).ok().map(Some)
                };
                candidate.map_or_else(
                    || {
                        draft.declaration
                            != draft
                                .original
                                .declaration
                                .as_ref()
                                .map_or_else(String::new, ToString::to_string)
                    },
                    |candidate| candidate != draft.original.declaration,
                )
            }
            Self::BusTap(draft) => {
                let selector_changed = crate::state::BusSlice::parse(draft.slice.trim())
                    .map_or_else(
                        |_| draft.slice != draft.original.slice.to_string(),
                        |slice| slice != draft.original.slice,
                    );
                draft.source_bus_id != draft.original.bus_id
                    || selector_changed
                    || draft.orientation != draft.original.orientation
            }
            Self::NetLabel(draft) => {
                let candidate = draft
                    .x
                    .trim()
                    .parse::<i32>()
                    .ok()
                    .zip(draft.y.trim().parse::<i32>().ok())
                    .map(|(x, y)| {
                        crate::state::NetLabel::new(
                            draft.original.id,
                            crate::state::Point::new(x, y),
                            draft.name.trim(),
                        )
                    });
                candidate.map_or_else(
                    || {
                        draft.name != draft.original.name
                            || draft.x != draft.original.pos.x.to_string()
                            || draft.y != draft.original.pos.y.to_string()
                    },
                    |candidate| candidate != draft.original,
                )
            }
            Self::NamedNet(draft) => draft.name.trim() != draft.original.name,
            Self::DesignNote(draft) => {
                let mut candidate = draft.original.clone();
                candidate
                    .update(draft.kind, draft.text.clone())
                    .map_or_else(
                        |_| true,
                        |_| {
                            if let Some(review_state) = draft.review_state
                                && candidate.set_review_state(review_state).is_err()
                            {
                                return true;
                            }
                            candidate != draft.original
                        },
                    )
            }
            Self::DocumentationShape(draft) => {
                let points: Option<Vec<_>> = draft
                    .points
                    .iter()
                    .map(|(x, y)| {
                        x.trim()
                            .parse::<i32>()
                            .ok()
                            .zip(y.trim().parse::<i32>().ok())
                            .map(|(x, y)| crate::state::Point::new(x, y))
                    })
                    .collect();
                points
                    .and_then(|points| {
                        crate::state::geometry_from_points(draft.original.kind(), &points).ok()
                    })
                    .is_none_or(|geometry| geometry != draft.original.geometry)
            }
        }
    }
}

/// Dialog visibility state

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HelpCenterPage {
    #[default]
    Help,
    ReleaseNotes,
    MigrationGuide,
    Diagnostics,
    SupportBundle,
    LegalPrivacy,
}

impl HelpCenterPage {
    pub(crate) const ALL: [Self; 6] = [
        Self::Help,
        Self::ReleaseNotes,
        Self::MigrationGuide,
        Self::Diagnostics,
        Self::SupportBundle,
        Self::LegalPrivacy,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Help => "Help Center",
            Self::ReleaseNotes => "Release notes",
            Self::MigrationGuide => "Migration guide",
            Self::Diagnostics => "Diagnostics",
            Self::SupportBundle => "Support bundle",
            Self::LegalPrivacy => "Legal & privacy",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct HelpCenterDialogState {
    pub(crate) open: bool,
    pub(crate) page: HelpCenterPage,
    pub(crate) query: String,
    pub(crate) include_session_log: bool,
    pub(crate) disclosure_reviewed: bool,
}

impl Default for HelpCenterDialogState {
    fn default() -> Self {
        Self {
            open: false,
            page: HelpCenterPage::Help,
            query: String::new(),
            include_session_log: true,
            disclosure_reviewed: false,
        }
    }
}

impl HelpCenterDialogState {
    pub(crate) fn open(&mut self, page: HelpCenterPage) {
        self.open = true;
        self.page = page;
        if page != HelpCenterPage::SupportBundle {
            self.disclosure_reviewed = false;
        }
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.disclosure_reviewed = false;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum WindowSessionPage {
    #[default]
    Windows,
    Documents,
    SessionRestore,
}

impl WindowSessionPage {
    pub(crate) const ALL: [Self; 3] = [Self::Windows, Self::Documents, Self::SessionRestore];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::Documents => "Documents",
            Self::SessionRestore => "Session restore",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WindowWorkflow {
    NewApplicationWindow,
    DetachDocument,
    MoveDocument,
    ReattachDocument,
    ConsolidateWindows,
    MonitorRecovery,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum NewWindowInitialContent {
    #[default]
    EmptyProjectWorkspace,
    CloneCurrentWorkspaceLayout,
    MoveActiveDocument,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum WindowLayoutChoice {
    #[default]
    Engineering,
    Review,
    PlotPresentation,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct WindowSessionDialogState {
    pub(crate) open: bool,
    pub(crate) page: WindowSessionPage,
    pub(crate) workflow: Option<WindowWorkflow>,
    pub(crate) source_window: crate::workbench::ApplicationWindowId,
    pub(crate) source_document: Option<crate::workbench::state::WorkspaceDocumentId>,
    pub(crate) source_label: String,
    pub(crate) initial_content: NewWindowInitialContent,
    pub(crate) destination: Option<crate::workbench::ApplicationWindowId>,
    pub(crate) layout_choice: WindowLayoutChoice,
    pub(crate) restore_on_launch: bool,
    pub(crate) synchronize_chrome: bool,
    pub(crate) recover_all_future_windows: bool,
    pub(crate) reattach_at_end: bool,
    pub(crate) recovery_selected: std::collections::BTreeSet<crate::workbench::ApplicationWindowId>,
    pub(crate) error: Option<String>,
}

impl WindowSessionDialogState {
    pub(crate) fn open(&mut self, page: WindowSessionPage) {
        *self = Self::default();
        self.open = true;
        self.page = page;
    }

    pub(crate) fn open_workflow(&mut self, workflow: WindowWorkflow) {
        *self = Self {
            open: true,
            workflow: Some(workflow),
            restore_on_launch: true,
            synchronize_chrome: true,
            recover_all_future_windows: true,
            ..Self::default()
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
pub struct DialogState {
    /// About dialog
    pub about: bool,
    /// Shortcuts help dialog
    pub shortcuts_help: bool,
    /// Shortcuts help filter text — nobody reads 45 unfiltered rows
    pub shortcuts_filter: String,
    /// Versioned help, migration, diagnostics, support, and notices manager.
    pub(crate) help_center: HelpCenterDialogState,
    /// Device-local window, document, and session presentation manager.
    pub(crate) window_session: WindowSessionDialogState,
    /// Mockup-authored full-screen and reset-view transaction.
    pub(crate) view_operation: ViewOperationDialogState,
    /// New Cell creation dialog
    pub new_cell_dialog: bool,
    /// New Cell name input
    pub new_cell_name: String,
    /// New Cell target library
    pub new_cell_library: String,
    /// New Cell description
    pub new_cell_description: String,
    /// New Cell view types to create
    pub new_cell_create_schematic: bool,
    /// Create symbol view for new cell
    pub new_cell_create_symbol: bool,
    /// Create testbench view for new cell
    pub new_cell_create_testbench: bool,
    /// New Cell validation error message
    pub new_cell_error: Option<String>,
    /// Exact library catalog revision captured when New Cell opened.
    pub(crate) new_cell_library_revision: u64,
    /// New View creation dialog
    pub new_view_dialog: bool,
    /// New View target library
    pub new_view_library: String,
    /// New View target cell
    pub new_view_cell: String,
    /// New View name input
    pub new_view_name: String,
    /// New View type selection
    pub new_view_type: crate::state::ViewType,
    /// New View validation error message
    pub new_view_error: Option<String>,
    /// Exact library catalog revision captured when New View opened.
    pub(crate) new_view_library_revision: u64,
    /// Copy Cell dialog
    pub copy_cell_dialog: bool,
    /// Copy Cell source library
    pub copy_cell_source_library: String,
    /// Copy Cell source cell
    pub copy_cell_source_cell: String,
    /// Copy Cell target library
    pub copy_cell_target_library: String,
    /// Copy Cell new name input
    pub copy_cell_name: String,
    /// Copy Cell validation error message
    pub copy_cell_error: Option<String>,
    /// Exact library catalog revision captured when Copy Cell opened.
    pub(crate) copy_cell_library_revision: u64,
    /// Rename Cell dialog
    pub rename_cell_dialog: bool,
    /// Rename Cell target library
    pub rename_cell_library: String,
    /// Rename Cell current name
    pub rename_cell_current: String,
    /// Rename Cell new name input
    pub rename_cell_name: String,
    /// Rename Cell validation error message
    pub rename_cell_error: Option<String>,
    /// Exact library catalog revision captured when Rename Cell opened.
    pub(crate) rename_cell_library_revision: u64,
    /// Rename View dialog
    pub rename_view_dialog: bool,
    /// Rename View target library
    pub rename_view_library: String,
    /// Rename View target cell
    pub rename_view_cell: String,
    /// Rename View current name
    pub rename_view_current: String,
    /// Rename View new name input
    pub rename_view_name: String,
    /// Rename View validation error message
    pub rename_view_error: Option<String>,
    /// Exact library catalog revision captured when Rename View opened.
    pub(crate) rename_view_library_revision: u64,
    /// New Library dialog
    pub new_library_dialog: bool,
    /// New Library name input
    pub new_library_name: String,
    /// New Library validation error message
    pub new_library_error: Option<String>,
    /// Exact library catalog revision captured when New Library opened.
    pub(crate) new_library_library_revision: u64,
    /// Rename Library dialog
    pub rename_library_dialog: bool,
    /// Rename Library current name
    pub rename_library_current: String,
    /// Rename Library new name input
    pub rename_library_name: String,
    /// Rename Library validation error message
    pub rename_library_error: Option<String>,
    /// Exact library catalog revision captured when Rename Library opened.
    pub(crate) rename_library_library_revision: u64,
    /// Delete Library review
    pub delete_library_dialog: bool,
    /// Delete Library target
    pub delete_library_target: String,
    /// Delete Library blocking reason, retained from the last commit attempt
    pub delete_library_error: Option<String>,
    /// Exact library catalog revision captured when Delete Library opened.
    pub(crate) delete_library_library_revision: u64,
    /// Exact, revision-bound destructive review for deleting one cell or view.
    pub(crate) library_deletion_review: LibraryDeletionReviewState,
    /// Destructive review for removing one record from the analysis plan —
    /// an analysis, a design variable, a saved output or a capture group —
    /// raised only where removal orphans something.
    pub(crate) plan_removal_review: PlanRemovalReview,
    /// A saved-file open found a bound, eligible autosave checkpoint; the
    /// restore dialog resolves it before either exact byte snapshot is loaded.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) pending_autosave_restore:
        Option<crate::workbench::lifecycle::recovery_checkpoint::AutosaveRestoreCandidate>,

    /// Starting position of selection drag (grid coords)
    pub drag_start: Option<(i32, i32)>,
    /// Last drag position for computing delta (grid coords)
    pub last_drag_pos: Option<(i32, i32)>,

    /// DRC results (cached from last run; surfaced by the schematic view)
    pub drc_results: Option<crate::services::drc::DrcResult>,
    /// `topology_version` when the last check ran — canvas markers hide and
    /// the ERC pill reads "stale" once the design changes underneath them.
    pub drc_checked_version: u64,
    /// Position in the severity-ordered finding cycle (F4 / Shift+F4);
    /// None until the first jump after a check.
    pub drc_cycle: Option<usize>,

    /// Waveform calculator dialog open
    pub waveform_calculator_dialog: bool,
    /// The expression and unit diagnostics tool.
    pub expression_diagnostics_dialog: bool,

    /// Preferences dialog open
    pub preferences_open: bool,

    /// Unified Page Setup / Print / Export Active View transaction. Draft
    /// controls and preview state are runtime-only; committed setups live in
    /// `ProjectWorkspace::hardcopy_setups`.
    pub(crate) hardcopy: crate::workbench::app::HardcopyDialogState,

    /// Permanent authored drawing-sheet transaction for schematic and
    /// testbench documents. It never contains printer or export-media state.
    pub(crate) drawing_sheet_setup: crate::workbench::app::DrawingSheetSetupState,

    /// Project and personal defaults for future authored drawing sheets.
    pub(crate) drawing_sheet_defaults: crate::workbench::app::DrawingSheetDefaultsDialogState,

    /// Custom drawing-sheet size library, editor, and portable JSON transfer.
    pub(crate) drawing_sheet_presets: crate::workbench::app::DrawingSheetPresetDialogsState,

    /// Document-wide authored-sheet reconciliation and title-field editing.
    pub(crate) drawing_sheet_support: crate::workbench::app::DrawingSheetSupportState,

    /// Read-only resolved policy review owned by Preferences.
    pub(crate) managed_preference_policy_open: bool,

    /// Device-local workspace layout manager owned by Preferences.
    pub(crate) workspace_layout_manager:
        crate::workbench::app::dialogs::preferences::workspace_layout_manager::WorkspaceLayoutManagerState,

    /// Transactional shortcut import/export workflows.
    pub(crate) shortcut_portability: crate::workbench::app::dialogs::preferences::shortcut_portability_dialogs::ShortcutPortabilityDialogsState,

    /// Transactional keyboard shortcut editor.
    pub(crate) shortcut_editor: ShortcutEditorState,

    /// Browser-only persist-before-live policy candidate. Native publication
    /// completes in the initiating frame, while browser storage is async.
    pub(crate) shortcut_policy_candidate: Option<crate::workbench::ShortcutPreferences>,

    /// License activation dialog state
    pub license_dialog: LicenseDialogState,

    /// Reviewed identity for the project File > New is about to create.
    pub(crate) new_project:
        crate::workbench::app::dialogs::project::new_project::NewProjectDialogState,

    /// Immutable cloud publication transaction.
    pub(crate) publish_web: crate::workbench::app::dialogs::publish_web::PublishWebDialogState,

    /// Live-session lifecycle controls (start review, host roster, guest view).
    pub(crate) live_session:
        crate::workbench::app::dialogs::live_session::LiveSessionDialogState,

    /// Owner-confirmed publication tombstone transaction.
    pub(crate) unpublish_web:
        crate::workbench::app::dialogs::unpublish_web::UnpublishWebDialogState,

    /// Command palette state
    pub command_palette: CommandPaletteState,

    /// Validated configuration transaction for Place bus tap.
    pub(crate) bus_tap: BusTapDialogState,

    /// Exact vector-port widths for a built-in XSPICE placement.
    pub(crate) builtin_xspice_placement: BuiltinXspicePlacementDialogState,

    /// Snapped, typed net-label placement transaction.
    pub(crate) net_label_placement: NetLabelPlacementDialogState,

    /// Device-local hierarchy and annotation visibility transaction.
    pub(crate) schematic_visibility: SchematicVisibilityDialogState,

    /// Device-local construction-layer visibility around the permanent paper
    /// edge. These choices never modify project-owned drawing-sheet setup.
    pub(crate) drawing_sheet_layers: DrawingSheetLayersDialogState,

    /// Live advisory review of printable objects outside the authored drawing
    /// area or overlapping the title block.
    pub(crate) drawing_sheet_overflow_open: bool,

    /// Transactional grid display, exact snapping, and conductor-routing
    /// configuration.
    pub(crate) grid_snap_routing: GridSnapRoutingDialogState,

    /// Explicit menu-owned hierarchy edit-context transaction.
    pub(crate) descend_hierarchy: DescendHierarchyDialogState,

    /// Universal engineering data-grid view, named-view, copy, and export
    /// transaction.
    pub(crate) engineering_table: EngineeringTableDialogState,

    /// Typed interface-port placement transaction.
    pub(crate) pin_port: PinPortDialogState,

    /// Non-electrical typed design-note placement transaction.
    pub(crate) design_note: DesignNoteDialogState,

    /// Non-electrical typed documentation-shape placement transaction.
    pub(crate) documentation_shape: DocumentationShapeDialogState,

    /// Governed Cut, Duplicate, Delete, and Select All schematic selection workflow.
    pub(crate) selection_workflow: crate::workbench::app::SelectionWorkflowDialogState,

    /// Connectivity-aware selected-object movement transaction.
    pub(crate) move_selection: MoveSelectionDialogState,

    /// Anchor-preserving conductor/documentation stretch transaction.
    pub(crate) stretch_selection: StretchSelectionDialogState,

    /// Named linear, rectangular, or radial documentation array transaction.
    pub(crate) array_selection: ArraySelectionDialogState,

    /// Pin-, parameter-, model-, and netlist-compatible instance replacement.
    pub(crate) replace_instance: ReplaceInstanceDialogState,

    /// Multi-document selected-instance extraction transaction.
    pub(crate) create_hierarchy: CreateHierarchyDialogState,

    /// Validated active-document save revision transaction.
    pub(crate) check_and_save: CheckAndSaveDialogState,

    /// Project-owned hierarchy/view configuration manager.
    pub(crate) configuration_sets: ConfigurationSetsDialogState,

    /// Project-owned sheet, variant, annotation, and hierarchy manager.
    pub(crate) design_management: DesignManagementDialogState,

    /// Live connectivity, typed-bus, and reviewed repair manager.
    pub(crate) connectivity_manager: ConnectivityManagerDialogState,

    /// Filtered selection query and atomic active-owner property transaction.
    pub(crate) selection_bulk_edit: SelectionBulkEditDialogState,

    /// Durable anchored schematic review threads and resolution workflow.
    pub(crate) design_review_comments: DesignReviewCommentsDialogState,

    /// Hash-linked validated schematic revisions and semantic audit workflow.
    pub(crate) project_revision_history: ProjectRevisionHistoryDialogState,

    /// Atomic source-, pin-, graphic-, and generated-view symbol transaction.
    pub(crate) create_model_bound_symbol: CreateModelBoundSymbolDialogState,

    /// Validated RSpice/SVG/EDIF symbol import transaction.
    pub(crate) symbol_import: SymbolImportDialogState,

    /// Typed, versioned component parameter-form transaction.
    pub(crate) symbol_parameter_form: SymbolParameterFormDialogState,

    /// Generic typed properties transaction for non-component schematic
    /// objects selected by Edit, Q, double-click, or the context menu.
    pub(crate) object_properties: ObjectPropertiesDialogState,

    /// Stable-identity rename transaction for one component, label, logical
    /// named net, or declared bus selected in the active schematic.
    pub(crate) rename_selection: RenameSelectionDialogState,

    /// Project-owned technology attachment transaction.
    pub(crate) technology_attachment: TechnologyAttachmentDialogState,

    /// Cached, integrity-verified full-project recovery catalog.
    pub(crate) project_checkpoint_recovery: ProjectCheckpointRecoveryState,

    /// Runtime interaction state for drag, hover, etc.
    pub interaction: crate::workbench::app_state::InteractionState,

    /// State for save confirmation modal (unsaved changes warning)
    pub confirmation_dialog: ConfirmationDialogState,

    /// Dedicated engineering review for Revert and Close Project. These
    /// destructive ellipsis actions intentionally do not use the generic
    /// Save/Don't save confirmation grammar.
    pub(crate) project_review_dialog: ProjectReviewDialogState,
}

impl DialogState {
    /// Whether an app-owned modal has exclusive keyboard intent.
    ///
    /// This is deliberately derived from the authoritative workflow states,
    /// rather than from egui's previous-frame modal layer. Shortcut dispatch
    /// runs before dialogs are painted, so retained state is the only value
    /// that is both current and deterministic at that point in the frame.
    pub(crate) fn application_modal_open(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        let autosave_restore_open = self.pending_autosave_restore.is_some();
        #[cfg(target_arch = "wasm32")]
        let autosave_restore_open = false;

        self.about
            || self.shortcuts_help
            || self.help_center.open
            || self.window_session.open
            || self.view_operation.open
            || self.new_cell_dialog
            || self.new_view_dialog
            || self.copy_cell_dialog
            || self.rename_cell_dialog
            || self.rename_view_dialog
            || self.new_library_dialog
            || self.rename_library_dialog
            || self.delete_library_dialog
            || self.library_deletion_review.target.is_some()
            || self.plan_removal_review.target.is_some()
            || self.preferences_open
            || self.hardcopy.open
            || self.drawing_sheet_setup.open
            || self.drawing_sheet_defaults.open
            || self.drawing_sheet_presets.any_open()
            || self.drawing_sheet_support.any_open()
            || self.managed_preference_policy_open
            || self.workspace_layout_manager.open
            || self.shortcut_portability.application_modal_open()
            || self.shortcut_editor.open
            || self.license_dialog.open
            || self.new_project.open
            || self.publish_web.open
            || self.live_session.open
            || self.unpublish_web.open
            || self.command_palette.open
            || self.bus_tap.open
            || self.builtin_xspice_placement.open
            || self.net_label_placement.open
            || self.schematic_visibility.open
            || self.drawing_sheet_layers.open
            || self.drawing_sheet_overflow_open
            || self.grid_snap_routing.open
            || self.engineering_table.open
            || self.pin_port.open
            || self.design_note.open
            || self.documentation_shape.open
            || self.selection_workflow.open
            || self.move_selection.open
            || self.stretch_selection.open
            || self.array_selection.open
            || self.replace_instance.open
            || self.create_hierarchy.open
            || self.check_and_save.open
            || self.connectivity_manager.open
            || self.design_management.open
            || self.selection_bulk_edit.open
            || self.design_review_comments.open
            || self.project_revision_history.open
            || self.create_model_bound_symbol.open
            || self.symbol_import.open
            || self.symbol_parameter_form.open
            || self.configuration_sets.open
            || self.object_properties.open
            || self.rename_selection.open
            || self.technology_attachment.open
            || self.interaction.schematic_delete_confirmation_open
            || self.confirmation_dialog.visible
            || self.project_review_dialog.request.is_some()
            || autosave_restore_open
    }
}

#[cfg(test)]
mod tests;
