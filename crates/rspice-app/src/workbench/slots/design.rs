//! The design feature's dialogs and canvas transactions: placement, selection
//! edits, hierarchy, display options, rule setup and the symbol workflows; the
//! schematic and symbol editors' interaction session; the design panel, the
//! component chooser's filter and the Stimulus Library's reading and drafts.

use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use crate::state::{DesignNoteKind, DesignReviewState, GridStyle, SchematicVisibilityPolicy};
use crate::ui::widgets::DialogChoice;
use crate::workbench::app::actions::workspace::{SchematicRestoreParts, sync_active_schematic};
use crate::workbench::app::dialogs::grid_snap_routing::{self, GridSnapRoutingCommitError};
use crate::workbench::app::dialogs::visibility_options::{self, SchematicVisibilityRecovery};
use crate::workbench::app::{
    ConnectivityManagerDialogState, CreateModelBoundSymbolDialogState,
    DesignReviewCommentsDialogState, FloatingCopyParts, SelectionBulkEditDialogState,
    SymbolImportDialogState, SymbolParameterFormDialogState,
};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::{CanvasGesturePort, DesignPort};
use crate::workbench::state::WorkbenchSession;

/// What the design feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct DesignState {
    /// Validated configuration transaction for Place bus tap.
    pub(crate) bus_tap: crate::workbench::app::dialogs::placement::bus_tap::BusTapDialogState,

    /// Exact vector-port widths for a built-in XSPICE placement.
    pub(crate) builtin_xspice_placement:
        crate::workbench::app::dialogs::placement::builtin_xspice::BuiltinXspicePlacementDialogState,

    /// Snapped, typed net-label placement transaction.
    pub(crate) net_label_placement:
        crate::workbench::app::dialogs::placement::net_label::NetLabelPlacementDialogState,

    /// Display options: device-local schematic visibility and drawing-sheet
    /// layers, neither of which modifies project-owned design data.
    pub(crate) schematic_visibility_dialog:
        crate::workbench::app::dialogs::visibility_options::SchematicVisibilityDialogState,

    /// Transactional grid display, exact snapping, and conductor-routing
    /// configuration.
    pub(crate) grid_snap_routing:
        crate::workbench::app::dialogs::grid_snap_routing::GridSnapRoutingDialogState,

    /// Explicit menu-owned hierarchy edit-context transaction.
    pub(crate) descend_hierarchy:
        crate::workbench::app::dialogs::hierarchy::descend::DescendHierarchyDialogState,

    /// Universal engineering data-grid view, named-view, copy, and export
    /// transaction.
    pub(crate) engineering_table:
        crate::workbench::app::dialogs::engineering_table::EngineeringTableDialogState,

    /// Typed interface-port placement transaction.
    pub(crate) pin_port: crate::workbench::app::dialogs::placement::pin_port::PinPortDialogState,

    /// Non-electrical typed design-note placement transaction.
    pub(crate) design_note:
        crate::workbench::app::dialogs::placement::design_note::DesignNoteDialogState,

    /// Non-electrical typed documentation-shape placement transaction.
    pub(crate) documentation_shape:
        crate::workbench::app::dialogs::placement::documentation_shape::DocumentationShapeDialogState,

    /// Connectivity-aware selected-object movement transaction.
    pub(crate) move_selection:
        crate::workbench::app::dialogs::selection::move_selection::MoveSelectionDialogState,

    /// A Paste or Duplicate following the pointer until a click places it.
    pub(crate) floating_paste:
        Option<crate::workbench::app::dialogs::selection::commands::floating::FloatingPaste>,

    /// Anchor-preserving conductor/documentation stretch transaction.
    pub(crate) stretch_selection:
        crate::workbench::app::dialogs::selection::stretch::StretchSelectionDialogState,

    /// Named linear, rectangular, or radial documentation array transaction.
    pub(crate) array_selection:
        Box<crate::workbench::app::dialogs::selection::array::ArraySelectionDialogState>,

    /// Pin-, parameter-, model-, and netlist-compatible instance replacement.
    pub(crate) replace_instance:
        Box<crate::workbench::app::dialogs::replace_instance::ReplaceInstanceDialogState>,

    /// Adopting a stimulus definition onto a placed source, or saving one from
    /// it. Both directions are one transaction over one instance, so they are
    /// one session with two modes rather than two dialogs that would each have
    /// to state the same instance and the same card.
    pub(crate) stimulus_link:
        crate::workbench::app::dialogs::stimulus_link::StimulusLinkDialogState,

    /// Multi-document selected-instance extraction transaction.
    pub(crate) create_hierarchy:
        Box<crate::workbench::app::dialogs::hierarchy::create::CreateHierarchyDialogState>,

    /// The project's design-rule deck.
    pub(crate) rule_setup: crate::workbench::app::dialogs::rule_setup::RuleSetupDialogState,

    /// Live connectivity, typed-bus, and reviewed repair manager.
    pub(crate) connectivity_manager: Box<ConnectivityManagerDialogState>,

    /// Filtered selection query and atomic active-owner property transaction.
    pub(crate) selection_bulk_edit: Box<SelectionBulkEditDialogState>,

    /// Durable anchored schematic review threads and resolution workflow.
    pub(crate) design_review_comments: DesignReviewCommentsDialogState,

    /// Atomic source-, pin-, graphic-, and generated-view symbol transaction.
    pub(crate) create_model_bound_symbol: CreateModelBoundSymbolDialogState,

    /// Validated RSpice/SVG/EDIF symbol import transaction.
    pub(crate) symbol_import: SymbolImportDialogState,

    /// Typed, versioned component parameter-form transaction.
    pub(crate) symbol_parameter_form: SymbolParameterFormDialogState,

    /// Generic typed properties transaction for non-component schematic
    /// objects selected by Edit, Q, double-click, or the context menu.
    pub(crate) object_properties:
        crate::workbench::app::dialogs::object_properties::ObjectPropertiesDialogState,

    /// Stable-identity rename transaction for one component, label, logical
    /// named net, or declared bus selected in the active schematic.
    pub(crate) rename_selection:
        crate::workbench::app::dialogs::selection::rename::RenameSelectionDialogState,

    /// Find and replace on names, net labels and values.
    pub(crate) find_replace: crate::workbench::app::dialogs::find_replace::FindReplaceDialogState,

    /// One property form over several selected instances.
    pub(crate) shared_properties:
        crate::workbench::app::dialogs::selection::shared_properties::SharedPropertiesDialogState,

    /// Schematic-space cursor position, reported by the canvas each frame the
    /// pointer hovers it; consumed by the status bar.
    pub canvas_hover: Option<(f64, f64)>,
    /// Schematic-space center of the visible canvas, reported each frame the
    /// canvas renders; paste targets fall back to it when the cursor is not
    /// over the canvas (menu-driven paste).
    pub canvas_view_center: Option<(f64, f64)>,
    /// Canvas grid style (dots / lines / off).
    pub grid: GridStyle,
    /// Last non-Off display style retained for backward-compatible session
    /// restoration. The current `G` command cycles dots, lines, and off.
    pub grid_restore_style: GridStyle,
    /// Schematic editor selection classes. This is restored with the UI
    /// session but never written into project-owned schematic data.
    pub schematic_selection_filter: crate::state::SchematicSelectionFilter,
    /// What Copy and Cut last took from a schematic. It belongs to the app,
    /// not to one cell view's editor, so a copy made in one cell view pastes
    /// into another. Never persisted: a session restored later must not
    /// reopen with design objects nobody committed to a drawing.
    pub schematic_clipboard: rspice_design::schematic::clipboard::ClipboardData,
    /// Active and named filters for exact schematic bulk-edit queries.
    pub schematic_bulk_edit_filters:
        crate::workbench::app::dialogs::selection::bulk_filter::SelectionBulkFilterSession,
    /// Device-local schematic display policy. Presentation choices are
    /// restored with the session and are never written to project data.
    pub schematic_visibility: SchematicVisibilityPolicy,
    /// Optional drawing-sheet construction detail shown around the permanent
    /// authored paper boundary. Output inclusion remains independently owned
    /// by Print and Export.
    pub drawing_sheet_layers: crate::state::DrawingSheetLayerVisibility,
    /// Exact runtime snap-target configuration used by the Grid & snap
    /// popover. The document grid pitch remains project-owned.
    pub schematic_snap: crate::state::SnapEngine,
    /// Exact runtime conductor routing mode. This retains the horizontal- or
    /// vertical-first orthogonal preference that the display policy's
    /// user-facing three-way style intentionally abstracts.
    pub schematic_routing_mode: crate::state::WireRoutingMode,
    /// What Duplicate does with a terminal whose net keeps existing outside
    /// the copied set. Restored with the UI session; never project data.
    pub duplicate_external_nets: crate::state::DuplicateExternalNets,
    /// Last visibility-policy presentation snapshot for the active view.
    pub(crate) schematic_visibility_recovery: Option<
        crate::workbench::app::dialogs::visibility_options::SchematicVisibilityRecovery,
    >,
    /// Device-local edge-triggered schematic selection synchronization.
    /// This is deliberately excluded from the session wire: source-map
    /// and retained-result highlights are presentation state, not project
    /// data or a durable preference value.
    pub(crate) schematic_cross_probe: crate::workbench::cross_probe::SchematicCrossProbeRuntime,
    /// Device-local working and named engineering-table views. Project-owned
    /// named views live in `ProjectWorkspace`; this store is the personal
    /// preference authority restored on desktop, web, and tablet hosts.
    pub engineering_table_views: crate::state::engineering_table::EngineeringTableViewStore,
    /// Runtime state for the Schematic-family symbol editor surface.
    pub symbol: crate::workbench::app::actions::workspace::symbol_document::SymbolUiState,

    pub design_panel: crate::workbench::state::DesignPanel,
    /// Which stimulus definition the library workspace is reading, by exact
    /// name. Runtime-only: a definition can be renamed or deleted by another
    /// session of the same project, and restoring a name that no longer
    /// resolves would open the workspace on a definition nobody can see.
    pub selected_stimulus_definition: Option<String>,
    /// What the Stimulus Library instrument is in the middle of editing: one
    /// unapplied draft per definition, each with its own undo history, and the
    /// proof surface's cached evaluation. Runtime-only for the same reason the
    /// selection is, and for one more: a draft is by construction not in the
    /// project document, so restoring one would offer a revision the library
    /// never published.
    pub stimulus_editor: crate::workbench::state::StimulusEditorState,
    /// How the library browser is looking at that library: the scope strip's
    /// slice, and the waveform each row draws. Runtime-only, because both are
    /// readings of the project rather than part of it.
    pub stimulus_browser: crate::workbench::docks::StimulusBrowserState,
    /// The net each hierarchy occurrence lights under "Selected net across
    /// hierarchy", keyed by root cell view and instance path. One master
    /// placed twice has one editor session, so the session cannot hold it.
    pub occurrence_net_highlights: std::collections::BTreeMap<String, String>,
    /// Filter text in the component chooser.
    pub placement_query: String,
    /// Tabbed property dialog state (commercial-grade property editing).
    /// Boxed: the dialog's tabs carry their drafts.
    pub(crate) tabbed_property_dialog: Box<crate::properties::TabbedPropertyDialogState>,
    /// Property registry (component property schemas)
    pub(crate) property_registry: crate::properties::PropertyEditorSchema,
    /// The canvas's interaction state: its drag, hover, last click, context
    /// menu target and keyboard focus. Runtime-only.
    pub(crate) interaction: crate::workbench::app_state::interaction_state::InteractionState,
}

impl DesignState {
    /// Whether a schematic edit is under way: an undo step still open, as
    /// while a drag is held, or a copy floating under the pointer. History
    /// waits for either to land or be cancelled.
    pub(crate) fn schematic_edit_in_progress(&self, model: &Model) -> bool {
        model.schematic.has_pending_operation() || self.floating_paste.is_some()
    }

    /// What a floating Paste or Duplicate reads and writes of this state.
    pub(crate) fn floating_copy_parts(&mut self) -> FloatingCopyParts<'_> {
        FloatingCopyParts {
            floating_paste: &mut self.floating_paste,
            move_selection: &mut self.move_selection,
            stretch_selection: &mut self.stretch_selection,
            array_selection: &mut self.array_selection,
            canvas_hover: self.canvas_hover,
            canvas_view_center: self.canvas_view_center,
            clipboard: &self.schematic_clipboard,
            duplicate_external_nets: self.duplicate_external_nets,
        }
    }

    /// What restoring the active schematic from the workspace reads and
    /// writes of this state.
    pub(crate) fn schematic_restore_parts(&mut self) -> SchematicRestoreParts<'_> {
        SchematicRestoreParts {
            drag: &mut self.interaction.drag,
            snap: &mut self.schematic_snap,
            routing_mode: &mut self.schematic_routing_mode,
            visibility: &mut self.schematic_visibility,
            symbol: &mut self.symbol,
        }
    }

    pub(crate) fn set_grid_style(&mut self, style: GridStyle) {
        if style.visible() {
            self.grid_restore_style = style;
        }
        self.grid = style;
    }

    /// The design frame hook. The pointer and view centre the canvas reports
    /// are the Design canvas's, so while another workspace shows there are
    /// none.
    pub(crate) fn follow_workspace(&mut self, workspace: crate::workbench::state::Workspace) {
        if workspace != crate::workbench::state::Workspace::Design {
            self.canvas_hover = None;
            self.canvas_view_center = None;
        }
    }

    /// Move the snap onto the grid the open schematic's editor snaps to, once
    /// the schematic's grid changed.
    pub(crate) fn follow_schematic_grid(&mut self, model: &crate::workbench::app_state::Model) {
        self.schematic_snap.grid_size = model.schematic.session.editor.snap_engine.grid_size;
    }

    /// Cycle the upgraded toolbar's exact presentation order.
    pub(crate) fn cycle_grid_style(&mut self) -> GridStyle {
        let next = match self.grid {
            GridStyle::Dots => GridStyle::Lines,
            GridStyle::Lines => GridStyle::Off,
            GridStyle::Off => GridStyle::Dots,
        };
        self.set_grid_style(next);
        next
    }

    /// The project's stimulus library went away or went back to its accepted
    /// revision: when the project closes, or its library is reverted.
    ///
    /// The selection may name a definition the library no longer holds, and a
    /// selection that resolves to nothing leaves the workspace on an empty
    /// stage with a name in the dock. The drafts belonged to the revisions now
    /// gone; the close guard has already named every unapplied one, so what is
    /// left is bookkeeping. Keeping one would leave the instrument editing a
    /// record whose saved side no longer exists, and Apply would publish it as
    /// a revision of a definition the library does not hold.
    pub(crate) fn on_project_changed(&mut self) {
        self.selected_stimulus_definition = None;
        self.stimulus_editor.clear();
        self.stimulus_browser.clear();
    }
}

/// Grid, snap and wire routing: what the canvas snaps to and how wires route.
impl DesignState {
    /// Draw Grid, snap and wire routing and carry out what its reader chose.
    pub(crate) fn render_grid_snap_routing_dialog(
        &mut self,
        model: &mut Model,
        workbench: &mut WorkbenchSession,
        ctx: &egui::Context,
    ) {
        if !self.grid_snap_routing.open {
            return;
        }

        let choice =
            grid_snap_routing::show_grid_snap_routing_form(&mut self.grid_snap_routing, model, ctx);
        match choice {
            DialogChoice::Primary => {
                if let Err(error) = self.commit_grid_snap_routing(model, workbench) {
                    self.grid_snap_routing.validation_error = Some(error.message);
                    self.grid_snap_routing.focus_target = error.focus_target;
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.grid_snap_routing.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }

    /// Apply Grid, snap and wire routing: check the whole draft, then set the
    /// grid, the snap and the routing on the slot and the open schematic
    /// together.
    pub(crate) fn commit_grid_snap_routing(
        &mut self,
        model: &mut Model,
        workbench: &mut WorkbenchSession,
    ) -> Result<(), GridSnapRoutingCommitError> {
        let transaction = self.grid_snap_routing.clone();
        if model.schematic.canvas_settings_change_blocked() {
            return Err(GridSnapRoutingCommitError::new(
                "Finish or cancel the active canvas authoring gesture before applying snap or routing settings.",
                None,
            ));
        }
        let authority = transaction.authority.as_ref().ok_or_else(|| {
            GridSnapRoutingCommitError::new("The canvas-settings authority is missing.", None)
        })?;
        authority
            .validate_presentation(&*model, grid_snap_routing::TITLE)
            .map_err(|message| GridSnapRoutingCommitError::new(message, None))?;

        let requested_pitch = transaction.draft.snap_spacing.pitch();
        let pitch_changed = requested_pitch
            .is_some_and(|pitch| pitch != model.schematic.document().document_policy.grid_pitch);
        if pitch_changed && model.schematic_edit_read_only() {
            return Err(GridSnapRoutingCommitError::new(
                "Snap spacing is project-owned and cannot be changed in this read-only view.",
                Some(grid_snap_routing::GridSnapRoutingFocusTarget::SnapSpacing),
            ));
        }
        if requested_pitch.is_some()
            && !grid_snap_routing::any_snap_target_enabled(&transaction.draft.snap_engine)
        {
            return Err(GridSnapRoutingCommitError::new(
                "Enable at least one snap target, or choose Free snap spacing.",
                Some(grid_snap_routing::GridSnapRoutingFocusTarget::SnapTargets),
            ));
        }

        // Finish the complete candidate before mutating any owner. `Free` disables
        // snapping while retaining the governed document pitch for later reuse.
        let mut snap_engine = transaction.draft.snap_engine.clone();
        snap_engine.enabled = requested_pitch.is_some();
        snap_engine.grid_size = requested_pitch
            .unwrap_or(model.schematic.document().document_policy.grid_pitch)
            .canvas_grid_size();
        let routing_mode = grid_snap_routing::canonical_routing_mode(
            transaction.draft.wire_routing,
            model.schematic.session.editor.wire_drawing.routing_mode,
        );
        if !routing_mode.is_orthogonal() && !grid_snap_routing::angled_routing_allowed(model) {
            return Err(GridSnapRoutingCommitError::new(
                grid_snap_routing::ANGLED_ROUTING_REASON,
                None,
            ));
        }

        if let Some(pitch) = requested_pitch.filter(|_| pitch_changed) {
            let changed = model.schematic.change_grid_pitch(pitch);
            if !changed {
                return Err(GridSnapRoutingCommitError::new(
                    "The schematic grid pitch could not be changed.",
                    Some(grid_snap_routing::GridSnapRoutingFocusTarget::SnapSpacing),
                ));
            }
        }

        self.set_grid_style(transaction.draft.grid_style);
        model.schematic.session.editor.snap_engine = snap_engine.clone();
        self.schematic_snap = snap_engine;
        model
            .schematic
            .session
            .editor
            .wire_drawing
            .set_routing_mode(routing_mode);
        model.schematic.session.editor.bus_drawing.routing_mode = routing_mode;
        self.schematic_routing_mode = routing_mode;
        self.schematic_visibility.wire_routing = grid_snap_routing::routing_style(routing_mode);
        if pitch_changed {
            sync_active_schematic(model, workbench);
        }
        self.grid_snap_routing.close();
        workbench.prompt("Grid, snap and routing settings applied.");
        Ok(())
    }
}

/// Display options: the schematic visibility, the drawing-sheet layers and
/// the recovery point a change leaves.
impl DesignState {
    /// Draw Display options and carry out what its reader chose.
    pub(crate) fn render_schematic_visibility_dialog(
        &mut self,
        model: &mut Model,
        workbench: &mut WorkbenchSession,
        ctx: &egui::Context,
    ) {
        if !self.schematic_visibility_dialog.open {
            return;
        }

        let recovery_available = self.visibility_recovery_available(model);
        let choice = visibility_options::show_display_options(
            &mut self.schematic_visibility_dialog,
            recovery_available,
            ctx,
        );
        match choice {
            DialogChoice::Primary => self.apply_display_options(model, workbench),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.schematic_visibility_dialog.close();
            }
            DialogChoice::Secondary => match self.restore_visibility_recovery(model) {
                Ok(()) => {
                    self.schematic_visibility_dialog.close();
                    workbench.prompt("Restored the previous schematic visibility on this device.");
                }
                Err(error) => {
                    workbench.prompt(error);
                }
            },
            DialogChoice::None => {}
        }
    }

    /// Publish both drafts together and close.
    pub(crate) fn apply_display_options(
        &mut self,
        model: &mut Model,
        workbench: &mut WorkbenchSession,
    ) {
        let policy = self.schematic_visibility_dialog.draft;
        if policy != self.schematic_visibility {
            let recovery = self.capture_visibility_recovery(model);
            self.schematic_visibility_recovery = Some(recovery);
        }
        self.publish_visibility_policy(model, policy);
        self.drawing_sheet_layers = self.schematic_visibility_dialog.layers_draft;
        self.schematic_visibility_dialog.close();
        workbench.prompt("Display options applied on this device; the design is unchanged.");
    }

    pub(crate) fn capture_visibility_recovery(&self, model: &Model) -> SchematicVisibilityRecovery {
        SchematicVisibilityRecovery {
            view_path: model.workspace.content.active_view.display_path(),
            policy: self.schematic_visibility,
            routing_mode: self.schematic_routing_mode,
            net_highlight: model.schematic.session.editor.net_highlight.clone(),
            selection: model.schematic.session.editor.selection.clone(),
        }
    }

    fn visibility_recovery_available(&self, model: &Model) -> bool {
        self.schematic_visibility_recovery
            .as_ref()
            .is_some_and(|recovery| {
                recovery.view_path == model.workspace.content.active_view.display_path()
            })
    }

    pub(crate) fn restore_visibility_recovery(&mut self, model: &mut Model) -> Result<(), String> {
        let recovery = self
            .schematic_visibility_recovery
            .clone()
            .ok_or_else(|| "No prior schematic visibility state is available.".to_owned())?;
        if recovery.view_path != model.workspace.content.active_view.display_path() {
            return Err(
                "The active cell/view changed; its prior visibility state was not applied."
                    .to_owned(),
            );
        }
        self.schematic_visibility = recovery.policy;
        self.schematic_routing_mode = recovery.routing_mode;
        model.schematic.session.editor.wire_drawing.routing_mode = recovery.routing_mode;
        model.schematic.session.editor.bus_drawing.routing_mode = recovery.routing_mode;
        model.schematic.session.editor.net_highlight = recovery.net_highlight;
        model.schematic.session.editor.selection = recovery.selection;
        let active_key = model.workspace.content.active_schematic_reference().key();
        if let Some(mut buffer) = model.workspace.schematic_editor_mut(&active_key) {
            buffer.editor.session.editor.selection =
                model.schematic.session.editor.selection.clone();
        }
        self.schematic_visibility_recovery = None;
        Ok(())
    }

    pub(crate) fn publish_visibility_policy(
        &mut self,
        model: &mut Model,
        policy: SchematicVisibilityPolicy,
    ) {
        let current = model.schematic.session.editor.wire_drawing.routing_mode;
        let routing_mode = visibility_options::routing_mode_for(policy.wire_routing, current);
        model.schematic.session.editor.wire_drawing.routing_mode = routing_mode;
        model.schematic.session.editor.bus_drawing.routing_mode = routing_mode;
        self.schematic_routing_mode = routing_mode;
        if !policy.net_highlighting.lights_selected_net() {
            model.schematic.session.editor.net_highlight.clear();
        }
        let selectable_note_ids = model
            .schematic
            .document()
            .design_notes
            .iter()
            .filter(|note| {
                if note.kind != DesignNoteKind::ReviewNote {
                    return true;
                }
                match policy.review_markers {
                    SchematicReviewMarkerVisibility::Hidden => false,
                    SchematicReviewMarkerVisibility::All => true,
                    SchematicReviewMarkerVisibility::OpenAndAssigned => {
                        note.review.as_ref().is_some_and(|review| {
                            review.state == DesignReviewState::Open || review.assignee.is_some()
                        })
                    }
                }
            })
            .map(|note| note.id)
            .collect::<std::collections::HashSet<_>>();
        model
            .schematic
            .session
            .editor
            .selection
            .design_notes
            .retain(|id| selectable_note_ids.contains(id));
        self.schematic_visibility = policy;
    }
}

/// Design's canvas gesture is its drag.
impl CanvasGesturePort for DesignState {
    fn let_go_of_canvas_gesture(&mut self, model: &mut Model) {
        self.interaction.drag.cancel_schematic_drag(model);
    }
}

impl DesignPort for DesignState {
    fn review_markers(&self) -> SchematicReviewMarkerVisibility {
        self.schematic_visibility.review_markers
    }

    fn canvas_hover(&self) -> Option<(f64, f64)> {
        self.canvas_hover
    }

    fn component_property_findings(
        &self,
        model: &Model,
        component: &crate::state::Component,
    ) -> Vec<(String, String)> {
        crate::workbench::app::actions::property_edit::component_property_findings(
            &self.property_registry,
            model,
            component,
        )
    }
}
