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

#[cfg(test)]
mod tests {
    use super::apply_component_property_edits;
    use crate::common::app::AppState;
    use crate::properties::dialog::EditedProperties;
    use crate::state::{Component, ComponentType, Point};

    fn resistor(id: u64, name: &str, value: &str) -> Component {
        Component::new(id, ComponentType::Resistor, Point::new(100, 100))
            .with_name_value(name, value)
    }

    #[test]
    fn test_apply_component_property_edits_creates_undo_and_restores_on_undo() {
        let mut state = AppState::default();
        state.schematic.components.push(resistor(1, "R1", "1k"));

        let changed = apply_component_property_edits(
            &mut state,
            1,
            EditedProperties {
                name: "R2".to_string(),
                value: "2k".to_string(),
                model: String::new(),
                parameters: Vec::new(),
            },
        );
        assert!(changed, "property edit should create an undo checkpoint");
        assert!(state.schematic.is_dirty);
        assert!(state.schematic.can_undo());
        assert_eq!(
            state.schematic.undo_description(),
            Some("Edit properties for R1")
        );

        let updated = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == 1)
            .expect("component should remain present");
        assert_eq!(updated.name, "R2");
        assert_eq!(updated.value, "2k");

        assert!(
            state.schematic.undo(),
            "undo should restore original properties"
        );
        let restored = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == 1)
            .expect("component should still be present after undo");
        assert_eq!(restored.name, "R1");
        assert_eq!(restored.value, "1k");
    }

    #[test]
    fn test_apply_component_property_edits_noop_when_values_unchanged() {
        let mut state = AppState::default();
        state.schematic.components.push(resistor(1, "R1", "1k"));

        let changed = apply_component_property_edits(
            &mut state,
            1,
            EditedProperties {
                name: "R1".to_string(),
                value: "1k".to_string(),
                model: String::new(),
                parameters: Vec::new(),
            },
        );
        assert!(!changed, "no-op edits should not create undo entries");
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn test_apply_component_property_edits_returns_false_when_component_missing() {
        let mut state = AppState::default();
        let changed = apply_component_property_edits(
            &mut state,
            42,
            EditedProperties {
                name: "R2".to_string(),
                value: "2k".to_string(),
                model: String::new(),
                parameters: Vec::new(),
            },
        );
        assert!(!changed);
        assert!(!state.schematic.is_dirty);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn test_apply_component_property_edits_uses_id_when_original_name_is_empty() {
        let mut state = AppState::default();
        state.schematic.components.push(resistor(7, "", "1k"));

        let changed = apply_component_property_edits(
            &mut state,
            7,
            EditedProperties {
                name: "R7".to_string(),
                value: "2k".to_string(),
                model: String::new(),
                parameters: Vec::new(),
            },
        );
        assert!(changed);
        assert_eq!(
            state.schematic.undo_description(),
            Some("Edit properties for component 7")
        );
    }

    #[test]
    fn test_apply_component_property_edits_bumps_topology_version() {
        let mut state = AppState::default();
        state.schematic.components.push(resistor(1, "R1", "1k"));
        let version_before = state.schematic.topology_version();

        let changed = apply_component_property_edits(
            &mut state,
            1,
            EditedProperties {
                name: "R2".to_string(),
                value: "2k".to_string(),
                model: String::new(),
                parameters: Vec::new(),
            },
        );

        assert!(changed);
        assert!(
            state.schematic.topology_version() > version_before,
            "topology version should increase after mutation"
        );
    }
}
