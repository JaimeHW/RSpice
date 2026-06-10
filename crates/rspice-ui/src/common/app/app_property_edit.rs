//! Opening the property editor for a schematic component.

use super::AppState;

/// Open the tabbed property editor for `component_id`, populated from the
/// component's registry sheet and current values.
pub(crate) fn open_property_editor(state: &mut AppState, component_id: u64) {
    let Some(component) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
    else {
        return;
    };
    let component_type = component.kind;
    let properties = crate::properties::property_bridge::collect_properties_from_component(
        component,
        &state.property_registry,
    );
    if let Some(sheet) = state.property_registry.get(component_type) {
        state.tabbed_property_dialog.open_for_component(
            component_id,
            &component.name,
            component_type,
            sheet,
            properties,
        );
    } else {
        log::warn!("No property sheet found for {:?}", component_type);
    }
}
