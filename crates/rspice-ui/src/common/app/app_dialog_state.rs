//! App Dialog State
//!
//! Modal/dialog payload used by `AppState`. Analysis configuration lives
//! in `SimSetupState`, not here.

use super::{ConfirmationDialogState, ProjectReviewDialogState};

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
    pub(crate) command: crate::workbench::commands::Command,
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
    pub(crate) selected_command: Option<crate::workbench::commands::Command>,
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
    /// Commands run from the palette, newest first — leads the empty-query
    /// list under a RECENT header. Survives close/reopen, capped at five.
    pub(crate) recent: Vec<crate::workbench::commands::Command>,
}

impl CommandPaletteState {
    /// Open fresh (empty query, first row selected, focus requested).
    pub fn open(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
        self.want_focus = true;
    }
}

/// Dialog visibility state

#[derive(Debug, Clone, Default)]
pub struct DialogState {
    /// About dialog
    pub about: bool,
    /// Shortcuts help dialog
    pub shortcuts_help: bool,
    /// Shortcuts help filter text — nobody reads 45 unfiltered rows
    pub shortcuts_filter: String,
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
    /// A saved-file open found a bound, eligible autosave checkpoint; the
    /// restore dialog resolves it before either exact byte snapshot is loaded.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) pending_autosave_restore:
        Option<crate::common::recovery_checkpoint::AutosaveRestoreCandidate>,

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

    /// Preferences dialog open
    pub preferences_open: bool,

    /// Read-only resolved policy review owned by Preferences.
    pub(crate) managed_preference_policy_open: bool,

    /// Transactional shortcut import/export workflows.
    pub(crate) shortcut_portability: super::app_preferences_dialog::shortcut_portability_dialogs::ShortcutPortabilityDialogsState,

    /// Transactional keyboard shortcut editor.
    pub(crate) shortcut_editor: ShortcutEditorState,

    /// Browser-only persist-before-live policy candidate. Native publication
    /// completes in the initiating frame, while browser storage is async.
    pub(crate) shortcut_policy_candidate: Option<crate::workbench::ShortcutPreferences>,

    /// License activation dialog state
    pub license_dialog: LicenseDialogState,

    /// Command palette state
    pub command_palette: CommandPaletteState,

    /// Verilog-A model loading dialog state
    pub veriloga_dialog: crate::panels::VerilogALoadDialogState,

    /// Runtime interaction state for drag, hover, etc.
    pub interaction: super::InteractionState,

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
            || self.new_cell_dialog
            || self.new_view_dialog
            || self.copy_cell_dialog
            || self.rename_cell_dialog
            || self.waveform_calculator_dialog
            || self.preferences_open
            || self.managed_preference_policy_open
            || self.shortcut_portability.application_modal_open()
            || self.shortcut_editor.open
            || self.license_dialog.open
            || self.command_palette.open
            || self.veriloga_dialog.open
            || self.interaction.schematic_delete_confirmation_open
            || self.confirmation_dialog.visible
            || self.project_review_dialog.request.is_some()
            || autosave_restore_open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_blocks_shortcuts(configure: impl FnOnce(&mut DialogState)) {
        let mut dialogs = DialogState::default();
        assert!(!dialogs.application_modal_open());
        configure(&mut dialogs);
        assert!(dialogs.application_modal_open());
    }

    #[test]
    fn every_retained_dialog_owner_blocks_background_shortcuts() {
        assert_blocks_shortcuts(|dialogs| dialogs.about = true);
        assert_blocks_shortcuts(|dialogs| dialogs.shortcuts_help = true);
        assert_blocks_shortcuts(|dialogs| dialogs.new_cell_dialog = true);
        assert_blocks_shortcuts(|dialogs| dialogs.new_view_dialog = true);
        assert_blocks_shortcuts(|dialogs| dialogs.copy_cell_dialog = true);
        assert_blocks_shortcuts(|dialogs| dialogs.rename_cell_dialog = true);
        assert_blocks_shortcuts(|dialogs| dialogs.waveform_calculator_dialog = true);
        assert_blocks_shortcuts(|dialogs| dialogs.preferences_open = true);
        assert_blocks_shortcuts(|dialogs| dialogs.managed_preference_policy_open = true);
        assert_blocks_shortcuts(|dialogs| dialogs.shortcut_portability.open_import());
        assert_blocks_shortcuts(|dialogs| dialogs.shortcut_portability.open_export());
        assert_blocks_shortcuts(|dialogs| dialogs.shortcut_editor.open = true);
        assert_blocks_shortcuts(|dialogs| dialogs.license_dialog.open = true);
        assert_blocks_shortcuts(|dialogs| dialogs.command_palette.open = true);
        assert_blocks_shortcuts(|dialogs| dialogs.veriloga_dialog.open = true);
        assert_blocks_shortcuts(|dialogs| {
            dialogs.interaction.schematic_delete_confirmation_open = true;
        });
        assert_blocks_shortcuts(|dialogs| dialogs.confirmation_dialog.visible = true);
        assert_blocks_shortcuts(|dialogs| dialogs.project_review_dialog.show_close_project());
    }
}
