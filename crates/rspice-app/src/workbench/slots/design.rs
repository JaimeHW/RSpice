//! The design feature's dialogs and canvas transactions: placement, selection
//! edits, hierarchy, display options, rule setup and the symbol workflows; the
//! schematic and symbol editors' interaction session; the design panel, the
//! component chooser's filter and the Stimulus Library's reading and drafts.

use crate::state::{GridStyle, SchematicVisibilityPolicy};
use crate::workbench::app::{
    ConnectivityManagerDialogState, CreateModelBoundSymbolDialogState,
    DesignReviewCommentsDialogState, SelectionBulkEditDialogState, SymbolImportDialogState,
    SymbolParameterFormDialogState,
};

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
    /// The sheet-membership and generated-symbol failures last recorded.
    ///
    /// The drawing is synced every frame, so a failure that stands is
    /// recorded when it first appears or changes, not sixty times a second.
    pub recorded_sync_failures: crate::workbench::app::actions::workspace::SyncFailures,
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
}

impl DesignState {
    pub(crate) fn set_grid_style(&mut self, style: GridStyle) {
        if style.visible() {
            self.grid_restore_style = style;
        }
        self.grid = style;
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
}
