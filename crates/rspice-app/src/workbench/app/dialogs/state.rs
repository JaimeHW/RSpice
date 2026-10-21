//! App Dialog State
//!
//! Modal/dialog payload used by `AppState`. Analysis configuration lives
//! in `SimSetupState`, not here.

use crate::workbench::app::{
    ConfigurationSetsDialogState, ConfirmationDialogState, ConnectivityManagerDialogState,
    CreateModelBoundSymbolDialogState, DesignManagementDialogState,
    DesignReviewCommentsDialogState, ProjectReviewDialogState, ProjectRevisionHistoryDialogState,
    SelectionBulkEditDialogState, SymbolImportDialogState, SymbolParameterFormDialogState,
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

mod sessions;

pub use sessions::DialogState;
pub(crate) use sessions::{
    DatasetExportPicker, HelpCenterPage, NewWindowInitialContent, RunDetailsDialogState,
    WindowLayoutChoice, WindowSessionPage, WindowWorkflow,
};
