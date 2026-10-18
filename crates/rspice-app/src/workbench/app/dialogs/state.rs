//! App Dialog State
//!
//! Modal/dialog payload used by `AppState`. Analysis configuration lives
//! in `SimSetupState`, not here.

use rspice_schematic_editor::object_properties::{
    BusObjectPropertiesDraft, BusTapObjectPropertiesDraft, DesignNoteObjectPropertiesDraft,
    DocumentationShapeObjectPropertiesDraft, NamedNetObjectPropertiesDraft,
    NetLabelObjectPropertiesDraft, ObjectPropertiesDraft,
};

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
    PlanRemovalConsequence, PlanRemovalRefusal, PlanRemovalReview, PlanRemovalTarget,
    PlanRemovalTone,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ShortcutEditorContext {
    #[default]
    All,
    Global,
    Schematic,
    Simulation,
    Results,
    Verification,
    SourceEditor,
}

impl ShortcutEditorContext {
    pub(crate) const ALL: [Self; 7] = [
        Self::All,
        Self::Global,
        Self::Schematic,
        Self::Simulation,
        Self::Results,
        Self::Verification,
        Self::SourceEditor,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::All => "All contexts",
            Self::Global => "Global",
            Self::Schematic => "Schematic",
            Self::Simulation => "Simulation",
            Self::Results => "Results",
            Self::Verification => "Verification",
            Self::SourceEditor => "Source editor",
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
    /// Whether the palette owns the canonical `?surface=command-palette`
    /// route. Contextual test and embedded hosts may still render the palette
    /// without changing application navigation.
    pub(crate) route_owned: bool,
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

    pub(crate) fn open_routed(&mut self) {
        self.open();
        self.route_owned = true;
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
        self.route_owned = false;
    }

    pub(crate) fn open_in_scope(&mut self, scope: &str) {
        self.open();
        self.initial_scope = Some(scope.to_owned());
    }
}

/// Retained, isolated draft for the bus-tap placement transaction. The parsed
/// electrical contract is not published to the schematic until the primary
/// action validates every field.
#[derive(Debug, Clone, Default)]
pub(crate) struct BusTapDialogState {
    pub(crate) open: bool,
    pub(crate) fields: rspice_schematic_editor::bus_tap_placement::BusTapFields,
    pub(crate) source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
}

impl BusTapDialogState {
    /// `bus` is the declaration the sheet leaves no doubt about, or empty.
    /// Nothing else is prefilled: a sample written into a field is a tap the
    /// reader never asked for.
    pub(crate) fn open(
        &mut self,
        source: rspice_schematic_editor::requests::EditorRequestSource,
        bus: String,
    ) {
        *self = Self {
            open: true,
            fields: rspice_schematic_editor::bus_tap_placement::BusTapFields {
                bus,
                ..Default::default()
            },
            source: Some(source),
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
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
    pub(crate) source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
    pub(crate) validation_error: Option<String>,
}

impl BuiltinXspicePlacementDialogState {
    pub(crate) fn open(
        &mut self,
        stable_id: impl Into<String>,
        display_name: impl Into<String>,
        vector_ports: Vec<crate::state::CatalogXspiceVectorPort>,
        source: rspice_schematic_editor::requests::EditorRequestSource,
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
            source: Some(source),
            validation_error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
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
    /// Where on the screen the entry sits, taken from the click that opened
    /// it. Kept, so the card does not follow the pointer once it is up.
    pub(crate) screen_anchor: Option<egui::Pos2>,
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
            screen_anchor: None,
            validation_error: None,
        };
    }

    /// The entry's place on the screen, fixed the first frame it is drawn.
    pub(crate) fn screen_anchor_or(&mut self, pointer: egui::Pos2) -> egui::Pos2 {
        *self.screen_anchor.get_or_insert(pointer)
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

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

    pub(crate) fn label(self) -> String {
        self.pitch().map_or_else(
            || "Free".to_owned(),
            crate::schematic::view::drawing_sheet::grid_pitch_label,
        )
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

/// Isolated draft for the View ▸ Schematic visibility transaction.
///
/// The draft is device-local presentation state. Apply publishes all six
/// controls atomically; Cancel discards them without touching schematic
/// history, topology, project serialization, or retained result data.
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
    /// Edit the instance's symbol master on the parent, through the
    /// instance's transform.
    EditSymbolInPlace,
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
            authority: Some(authority),
            parent_dirty,
            validation_error: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// Draft for New Verilog-A device….
#[derive(Debug, Clone, Default)]
pub(crate) struct NewVerilogADeviceFormState {
    pub(crate) open: bool,
    pub(crate) library: String,
    pub(crate) name: String,
    /// The ports the starter module declares, as typed: `inp:in d[7:0] vss`.
    pub(crate) ports: String,
    /// Why the last commit was refused. Any edit clears it.
    pub(crate) error: Option<String>,
    /// The schematic the form was opened from: where the device is offered
    /// for placement once its module compiles with a port.
    pub(crate) origin: Option<crate::state::CellViewRef>,
}

/// Draft for Create pins.
///
/// The contract fields outlive a close, as they do in Virtuoso: a reader who
/// has just placed four logic inputs is far more likely to place a fifth than
/// to want the form's defaults back. The names and the document authority do
/// not — those belong to one batch, in one document.
#[derive(Debug, Clone, Default)]
pub(crate) struct PinPortDialogState {
    pub(crate) open: bool,
    pub(crate) fields: rspice_schematic_editor::pin_placement::PinPlacementFields,
    /// The document this batch was named for.
    pub(crate) authority: Option<crate::state::PlacementAuthority>,
    /// The last name armed this session, which the next open suggests from.
    pub(crate) last_name: String,
}

impl PinPortDialogState {
    pub(crate) fn open(&mut self, names: String, authority: crate::state::PlacementAuthority) {
        self.open = true;
        self.fields.names = names;
        self.authority = Some(authority);
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
        self.fields.names.clear();
        self.authority = None;
    }
}

/// Isolated draft for the Place text or design note transaction.
///
/// The text starts empty. The sample the form used to open holding was
/// written into the field as a value, so an untouched form placed a note that
/// said it.
#[derive(Debug, Clone, Default)]
pub(crate) struct DesignNoteDialogState {
    pub(crate) open: bool,
    pub(crate) kind: crate::state::DesignNoteKind,
    pub(crate) text: String,
    pub(crate) source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
}

impl DesignNoteDialogState {
    pub(crate) fn open(&mut self, source: rspice_schematic_editor::requests::EditorRequestSource) {
        *self = Self {
            open: true,
            kind: crate::state::DesignNoteKind::PlainText,
            text: String::new(),
            source: Some(source),
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// Isolated draft for the Draw documentation shape transaction.
#[derive(Debug, Clone, Default)]
pub(crate) struct DocumentationShapeDialogState {
    pub(crate) open: bool,
    pub(crate) kind: crate::state::DocumentationShapeKind,
    pub(crate) source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
    pub(crate) expected_shapes: Vec<crate::state::DocumentationShape>,
}

impl DocumentationShapeDialogState {
    pub(crate) fn open(
        &mut self,
        source: rspice_schematic_editor::requests::EditorRequestSource,
        expected_shapes: Vec<crate::state::DocumentationShape>,
    ) {
        *self = Self {
            open: true,
            kind: crate::state::DocumentationShapeKind::Rectangle,
            source: Some(source),
            expected_shapes,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// Authority and interaction state of the armed Move tool. The design
/// snapshot remains immutable while the tool is armed; pointer and keyboard
/// movement is accumulated as a preview delta and committed only once.
///
/// `mode` outlives one command on purpose: it is a tool option on the canvas
/// strip, and a tool option a reader set two commands ago is still the one
/// they meant.
#[derive(Debug, Clone, Default)]
pub(crate) struct MoveSelectionDialogState {
    pub(crate) armed: bool,
    pub(crate) mode: crate::state::MoveSelectionMode,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) canvas: rspice_schematic_editor::session::transform::TransformCanvasSession,
    pub(crate) generation: u64,
}

impl MoveSelectionDialogState {
    pub(crate) fn arm(&mut self, authority: SchematicEditAuthority) {
        *self = Self {
            armed: true,
            mode: self.mode,
            authority: Some(authority),
            canvas: Default::default(),
            generation: self.generation.wrapping_add(1),
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self {
            generation: self.generation.wrapping_add(1),
            mode: self.mode,
            ..Self::default()
        };
    }
}

/// A Paste or Duplicate floating under the pointer until a click places it.
///
/// The drawing does not change while it floats: the copy is held here, bound
/// to the cell view it was asked for in and turned as R and M asked, and the
/// click pastes it there as one undo step.
#[derive(Debug, Clone)]
pub(crate) struct FloatingPaste {
    pub(crate) objects: rspice_design::schematic::clipboard::ClipboardData,
    /// Where the copy's middle sat when the pointer took hold of it.
    pub(crate) start: crate::state::Point,
    /// The grid point the pointer took hold at; until the pointer is over
    /// the canvas, nothing has, and the copy jumps to it when it arrives.
    pub(crate) grab: Option<crate::state::Point>,
    /// How far the pointer has carried the copy since.
    pub(crate) delta: crate::state::Point,
    /// The cell view and sheet the copy was bound to.
    pub(crate) view_path: String,
    pub(crate) sheet: Option<crate::state::SheetId>,
    pub(crate) origin: FloatingPasteOrigin,
    /// The drawing the placement would leave, for the position, content and
    /// orientation it was worked out for.
    pub(crate) preview: Option<FloatingPastePreview>,
}

impl FloatingPaste {
    /// Where the copy's middle sits now, on the document grid.
    pub(crate) fn at(&self) -> crate::state::Point {
        self.start + self.delta
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FloatingPastePreview {
    pub(crate) at: crate::state::Point,
    pub(crate) content_version: u64,
    /// What the placement would change, taken once when the preview is
    /// worked out rather than searched for on every repaint.
    pub(crate) diff: rspice_schematic_editor::view::preview::TransformCandidateDiff,
    pub(crate) result: Result<bool, String>,
}

/// What floated, for the line the placement answers with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FloatingPasteOrigin {
    Paste,
    Duplicate {
        count: usize,
        kept: usize,
        unresolved_nets: bool,
    },
}

/// Authority and interaction state of the armed Stretch tool. The selected
/// conductor segment or documentation control point is resolved before
/// mutation and remains stable through the live canvas preview and the
/// atomic commit.
#[derive(Debug, Clone, Default)]
pub(crate) struct StretchSelectionDialogState {
    pub(crate) armed: bool,
    pub(crate) policy: crate::state::StretchOrthogonalPolicy,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) canvas: rspice_schematic_editor::session::stretch::StretchCanvasSession,
    pub(crate) generation: u64,
}

impl StretchSelectionDialogState {
    pub(crate) fn arm(
        &mut self,
        authority: SchematicEditAuthority,
        target: crate::state::StretchTarget,
        policy: crate::state::StretchOrthogonalPolicy,
    ) {
        *self = Self {
            armed: true,
            policy,
            authority: Some(authority),
            canvas: rspice_schematic_editor::session::stretch::StretchCanvasSession {
                target: Some(target),
                gesture: Default::default(),
            },
            generation: self.generation.wrapping_add(1),
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self {
            generation: self.generation.wrapping_add(1),
            ..Self::default()
        };
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
    pub(crate) canvas: rspice_schematic_editor::session::transform::TransformCanvasSession,
    pub(crate) generation: u64,
    pub(crate) preview_cache: Option<ArraySelectionPreviewCache>,
    pub(crate) initial_kind: crate::state::SchematicArrayKind,
    pub(crate) initial_count: String,
    pub(crate) initial_naming: String,
    pub(crate) dirty: bool,
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
            canvas: Default::default(),
            generation: self.generation.wrapping_add(1),
            preview_cache: None,
            initial_kind: crate::state::SchematicArrayKind::default(),
            initial_count,
            initial_naming,
            dirty: false,
        };
    }

    pub(crate) fn arm(&mut self) {
        self.open = false;
        self.armed = true;
        self.canvas = Default::default();
        self.generation = self.generation.wrapping_add(1);
        self.preview_cache = None;
        self.dirty = false;
    }

    pub(crate) fn close(&mut self) {
        *self = Self {
            generation: self.generation.wrapping_add(1),
            ..Self::default()
        };
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = self.kind != self.initial_kind
            || !equivalent_array_count(&self.count, &self.initial_count)
            || !equivalent_array_naming(&self.naming, &self.initial_naming);
        self.preview_cache = None;
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
    /// The validated revision was published; the dialog now only reviews it.
    pub(crate) saved: bool,
    /// Edits arrived while the browser save was in flight, so the saved
    /// snapshot does not hold them.
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
        if self.saved || self.is_pending() {
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
        if self.saved {
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

/// Retained owner for the mockup's generic Object properties transaction.
/// Components continue to use their schema-driven tabbed editor; geometric
/// connectivity objects and logical named nets use this draft because their
/// invariants span stable authority, exact coordinates, and neighboring
/// topology.
#[derive(Debug, Clone, Default)]
pub(crate) struct ObjectPropertiesDialogState {
    pub(crate) open: bool,
    pub(crate) draft: Option<ObjectPropertiesDraft>,
    /// Canonical source captured with the isolated draft.
    pub(crate) source: Option<rspice_schematic_editor::requests::EditorRequestSource>,
    /// Actual tab identity; a symbol tab can resolve to its paired schematic.
    pub(crate) view_path: String,
    pub(crate) dirty: bool,
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
        source: rspice_schematic_editor::requests::EditorRequestSource,
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
            source: Some(source),
            view_path,
            dirty: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_bus_tap(
        &mut self,
        tap: &crate::state::BusTap,
        source: rspice_schematic_editor::requests::EditorRequestSource,
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
            source: Some(source),
            view_path,
            dirty: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_net_label(
        &mut self,
        label: &crate::state::NetLabel,
        source: rspice_schematic_editor::requests::EditorRequestSource,
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
            source: Some(source),
            view_path,
            dirty: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_named_net(
        &mut self,
        target: crate::workbench::app::NamedNetTarget,
        source: rspice_schematic_editor::requests::EditorRequestSource,
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
            source: Some(source),
            view_path,
            dirty: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_design_note(
        &mut self,
        note: &crate::state::DesignNote,
        source: rspice_schematic_editor::requests::EditorRequestSource,
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
            source: Some(source),
            view_path,
            dirty: false,
            validation_error: None,
        };
    }

    pub(crate) fn open_documentation_shape(
        &mut self,
        shape: &crate::state::DocumentationShape,
        source: rspice_schematic_editor::requests::EditorRequestSource,
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
            source: Some(source),
            view_path,
            dirty: false,
            validation_error: None,
        };
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = self
            .draft
            .as_ref()
            .is_some_and(ObjectPropertiesDraft::is_modified);
        self.validation_error = None;
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

mod sessions;

pub use sessions::DialogState;
pub(crate) use sessions::{
    DatasetExportPicker, HelpCenterPage, NewWindowInitialContent, RunDetailsDialogState,
    WindowLayoutChoice, WindowSessionPage, WindowWorkflow,
};
