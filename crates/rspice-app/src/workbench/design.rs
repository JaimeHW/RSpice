//! The design feature: the schematic and symbol canvases, the stimulus
//! library, and the dialogs that edit what they draw.
//!
//! What it has not registered yet, the shell's legacy tables still serve.

use egui::Context;

use crate::workbench::app::dialogs::design_review_comments::ReviewComments;
use crate::workbench::app::dialogs::engineering_table::EngineeringTable;
use crate::workbench::app::dialogs::hierarchy::descend;
use crate::workbench::app::dialogs::selection::{array, bulk_edit, rename, shared_properties};
use crate::workbench::app::dialogs::{
    find_replace, object_properties, placement, rule_setup, symbol_definition,
};
use crate::workbench::framework::feature::{Cx, Feature, FeatureRegistry, NoHandles};
use crate::workbench::slots::DesignState;
use crate::workbench::state::WorkbenchSession;

/// The design feature.
pub(crate) struct DesignFeature;

impl Feature for DesignFeature {
    type State = DesignState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;

    fn register(_registry: &mut FeatureRegistry<Self>) {}
}

/// The dialogs the shell's dialog order draws, each over the parts of the
/// design slot it owns.
impl DesignFeature {
    /// Place bus tap.
    pub(in crate::workbench) fn render_bus_tap_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::bus_tap::render_bus_tap_dialog(&mut cx.state.bus_tap, cx.model, ctx);
    }

    /// Net label, or the off-sheet connector form.
    pub(in crate::workbench) fn render_net_label_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::net_label::render_net_label_dialog(
            &mut cx.state.net_label_placement,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Create pins.
    pub(in crate::workbench) fn render_pin_port_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::pin_port::render_pin_port_dialog(&mut cx.state.pin_port, cx.model, ctx);
    }

    /// Place text or design note.
    pub(in crate::workbench) fn render_design_note_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        placement::design_note::render_design_note_dialog(&mut cx.state.design_note, cx.model, ctx);
    }

    /// Draw documentation shape.
    pub(in crate::workbench) fn render_documentation_shape_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        placement::documentation_shape::render_documentation_shape_dialog(
            &mut cx.state.documentation_shape,
            cx.model,
            ctx,
        );
    }

    /// Create array.
    pub(in crate::workbench) fn render_array_selection_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        array::render_array_selection_dialog(&mut cx.state.array_selection, cx.model, ctx);
    }

    /// Object properties.
    pub(in crate::workbench) fn render_object_properties_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        object_properties::render_object_properties_dialog(
            &mut cx.state.object_properties,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Rename.
    pub(in crate::workbench) fn render_rename_selection_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        rename::render_rename_selection_dialog(
            &mut cx.state.rename_selection,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Find and replace.
    pub(in crate::workbench) fn render_find_replace_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let edit_in_progress = cx.state.schematic_edit_in_progress(cx.model);
        find_replace::render_find_replace_dialog(
            &mut cx.state.find_replace,
            edit_in_progress,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Properties, for the instances selected together.
    pub(in crate::workbench) fn render_shared_properties_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        shared_properties::render_shared_properties_dialog(
            &mut cx.state.shared_properties,
            &cx.state.property_registry,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Grid, snap and wire routing.
    pub(in crate::workbench) fn render_grid_snap_routing_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        cx.state
            .render_grid_snap_routing_dialog(cx.model, cx.wb, ctx);
    }

    /// Display options.
    pub(in crate::workbench) fn render_schematic_visibility_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        cx.state
            .render_schematic_visibility_dialog(cx.model, cx.wb, ctx);
    }

    /// Descend into hierarchy. A checked choice goes to the shell, which
    /// answers it straight after the dialog's step draws.
    pub(in crate::workbench) fn render_descend_hierarchy_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        descend::render_descend_hierarchy_dialog(
            &mut cx.state.descend_hierarchy,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Import symbol definition, then the symbol's parameter form.
    pub(in crate::workbench) fn render_symbol_definition_dialogs(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        symbol_definition::render_symbol_definition_dialogs(
            &mut cx.state.symbol_import,
            &mut cx.state.symbol_parameter_form,
            cx.model,
            ctx,
        );
    }

    /// Design review comments, signed with the name the cloud gives the
    /// reviewer.
    pub(in crate::workbench) fn render_design_review_comments_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        ReviewComments {
            dialog: &mut cx.state.design_review_comments,
            design_note: &mut cx.state.design_note,
            model: cx.model,
            workbench: cx.wb,
            cloud: cx.peers.cloud(),
            session: cx.peers.cloud_session(),
        }
        .render_design_review_comments_dialog(ctx);
    }

    /// Design rule setup. A new deck changes what blocks a run, so an applied
    /// one drops the run prepared under the old deck with its preflight.
    pub(in crate::workbench) fn render_rule_setup_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        let review_markers = cx.state.schematic_visibility.review_markers;
        if rule_setup::render_rule_setup_dialog(
            &mut cx.state.rule_setup,
            review_markers,
            cx.model,
            cx.runs,
            ctx,
        ) {
            rspice_app_runs::workbench::preflight::invalidate(
                &mut cx.runs.preflight,
                cx.handles.run_controller,
            );
        }
    }

    /// Bulk edit, over its draft and the saved filters.
    pub(in crate::workbench) fn render_selection_bulk_edit_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        bulk_edit::render_selection_bulk_edit_dialog(
            &mut cx.state.selection_bulk_edit,
            &mut cx.state.schematic_bulk_edit_filters,
            cx.model,
            cx.wb,
            ctx,
        );
    }

    /// Engineering table view, over its draft and the views saved on this
    /// device.
    pub(in crate::workbench) fn render_engineering_table_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        EngineeringTable {
            dialog: &mut cx.state.engineering_table,
            views: &mut cx.state.engineering_table_views,
            model: cx.model,
            platform: cx.platform,
            workbench: cx.wb,
        }
        .render_engineering_table_dialog(ctx);
    }
}
