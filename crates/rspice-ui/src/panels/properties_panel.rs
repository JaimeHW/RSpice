//! Property dialog host.
//!
//! The floating tabbed property dialog (schema-driven editing via the
//! `PropertyRegistry`). Inline inspection lives in the workbench inspector
//! (`crate::workbench::panels::schematic`).

use crate::common::app::AppState;
use crate::properties::{TabbedDialogResult, render_tabbed_property_dialog};

/// Render the floating tabbed property dialog window
/// Call this from the main app update loop
pub fn render_property_dialog(ctx: &egui::Context, state: &mut AppState) -> TabbedDialogResult {
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let number_locale = state.ui.number_locale;
    let result = render_tabbed_property_dialog(
        ctx,
        &mut state.tabbed_property_dialog,
        &state.property_registry,
        &state.model_library_manager,
        quantity_policy,
        number_locale,
        state.schematic.document_policy.property_commit,
    );

    // Handle dialog result - apply changes back to component
    if matches!(result, TabbedDialogResult::Applied)
        && let Some(comp_id) = state.tabbed_property_dialog.component_id
    {
        let committed = state.tabbed_property_dialog.take_prepared_commit();
        let committed_names: Vec<String> = committed.keys().cloned().collect();
        // The bridge serializes the full component property map. Merge the
        // validated delta onto the last committed baseline so a rejected
        // partial field cannot erase an unrelated existing parameter.
        let mut values = state.tabbed_property_dialog.original_values.clone();
        values.extend(committed);
        let before = crate::state::SchematicSnapshot::capture(&state.schematic);

        // Find the component and update its properties
        if let Some(component) = state
            .schematic
            .components
            .iter_mut()
            .find(|c| c.id == comp_id)
        {
            crate::properties::property_bridge::apply_properties_to_component(
                component,
                &values,
                &state.property_registry,
            );
            state.schematic.is_dirty = true;
            state.schematic.bump_topology_version();
            state.schematic.commit_undo_from(before, "edit properties");
        }

        // OK closes (clear everything); Apply keeps editing with the
        // committed values as the new baseline.
        if state.tabbed_property_dialog.open {
            state
                .tabbed_property_dialog
                .mark_fields_applied(committed_names);
        } else {
            state.tabbed_property_dialog.clear_after_apply();
        }
    }

    result
}
