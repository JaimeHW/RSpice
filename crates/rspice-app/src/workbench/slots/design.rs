//! The design feature's dialogs and canvas transactions: placement, selection
//! edits, hierarchy, display options, rule setup and the symbol workflows.

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
        crate::workbench::app::dialogs::selection::array::ArraySelectionDialogState,

    /// Pin-, parameter-, model-, and netlist-compatible instance replacement.
    pub(crate) replace_instance:
        crate::workbench::app::dialogs::replace_instance::ReplaceInstanceDialogState,

    /// Adopting a stimulus definition onto a placed source, or saving one from
    /// it. Both directions are one transaction over one instance, so they are
    /// one session with two modes rather than two dialogs that would each have
    /// to state the same instance and the same card.
    pub(crate) stimulus_link:
        crate::workbench::app::dialogs::stimulus_link::StimulusLinkDialogState,

    /// Multi-document selected-instance extraction transaction.
    pub(crate) create_hierarchy:
        crate::workbench::app::dialogs::hierarchy::create::CreateHierarchyDialogState,

    /// The project's design-rule deck.
    pub(crate) rule_setup: crate::workbench::app::dialogs::rule_setup::RuleSetupDialogState,

    /// Live connectivity, typed-bus, and reviewed repair manager.
    pub(crate) connectivity_manager: ConnectivityManagerDialogState,

    /// Filtered selection query and atomic active-owner property transaction.
    pub(crate) selection_bulk_edit: SelectionBulkEditDialogState,

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
}
