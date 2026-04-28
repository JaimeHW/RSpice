use super::AppState;

pub(super) fn apply_component_property_edits(
    state: &mut AppState,
    id: u64,
    props: crate::properties::dialog::EditedProperties,
) -> bool {
    let (old_name, changed) = match state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
    {
        Some(component) => (
            component.name.clone(),
            component.name != props.name || component.value != props.value,
        ),
        None => return false,
    };
    if !changed {
        return false;
    }

    let description = if old_name.is_empty() {
        format!("Edit properties for component {}", id)
    } else {
        format!("Edit properties for {}", old_name)
    };
    let new_name = props.name;
    let new_value = props.value;

    state.schematic.with_undo(description, move |schematic| {
        if let Some(component) = schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
        {
            component.name = new_name;
            component.value = new_value;
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        }
    })
}
