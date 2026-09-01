//! Dialog state for help, publishing, license, and collaborative session workflows.

use super::*;

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
    /// Whether this dialog owns the canonical `?surface=help-center` route.
    pub(crate) route_owned: bool,
}

impl Default for HelpCenterDialogState {
    fn default() -> Self {
        Self {
            open: false,
            page: HelpCenterPage::Help,
            query: String::new(),
            include_session_log: true,
            disclosure_reviewed: false,
            route_owned: false,
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

    pub(crate) fn open_routed(&mut self, page: HelpCenterPage) {
        self.open(page);
        self.route_owned = true;
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.disclosure_reviewed = false;
        self.route_owned = false;
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
    /// Why a removal the plan will not perform was refused before it was
    /// staged. The review above asks; this one only states.
    pub(crate) plan_removal_refusal: PlanRemovalRefusal,
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
            || self.plan_removal_refusal.analysis.is_some()
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
