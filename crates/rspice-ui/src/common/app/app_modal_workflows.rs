use egui::Context;

use super::{AppState, RSpiceApp, apply_component_property_edits};

fn apply_properties_dialog_result(
    state: &mut AppState,
    result: crate::properties::dialog::PropertiesDialogResult,
) -> bool {
    match result {
        crate::properties::dialog::PropertiesDialogResult::Apply(id, props) => {
            apply_component_property_edits(state, id, props)
        }
        crate::properties::dialog::PropertiesDialogResult::Cancel => false,
        crate::properties::dialog::PropertiesDialogResult::None => false,
    }
}

impl RSpiceApp {
    pub(super) fn process_component_properties_dialog(&mut self, ctx: &Context) {
        let result = crate::properties::dialog::render_properties_dialog(
            ctx,
            &mut self.state.property_editor,
        );
        let _ = apply_properties_dialog_result(&mut self.state, result);
    }

    pub(super) fn process_model_browser_dialog(&mut self, ctx: &Context) {
        let _ = crate::properties::model_browser::render_model_browser(
            ctx,
            &mut self.state.model_browser_state,
            &self.state.model_library_manager,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::dialog::EditedProperties;
    use crate::state::{Component, ComponentType, Point};

    fn make_state_with_component() -> AppState {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(1, ComponentType::Resistor, Point::new(100, 100))
                .with_name_value("R1", "1k"),
        );
        state
    }

    #[test]
    fn test_apply_properties_dialog_result_applies_edits() {
        let mut state = make_state_with_component();

        let changed = apply_properties_dialog_result(
            &mut state,
            crate::properties::dialog::PropertiesDialogResult::Apply(
                1,
                EditedProperties {
                    name: "R2".to_string(),
                    value: "2k".to_string(),
                    model: String::new(),
                    parameters: Vec::new(),
                },
            ),
        );

        assert!(changed);
        let component = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == 1)
            .unwrap();
        assert_eq!(component.name, "R2");
        assert_eq!(component.value, "2k");
    }

    #[test]
    fn test_apply_properties_dialog_result_cancel_is_noop() {
        let mut state = make_state_with_component();
        let changed = apply_properties_dialog_result(
            &mut state,
            crate::properties::dialog::PropertiesDialogResult::Cancel,
        );
        assert!(!changed);
        assert_eq!(state.schematic.components[0].name, "R1");
    }

    #[test]
    fn test_apply_properties_dialog_result_none_is_noop() {
        let mut state = make_state_with_component();
        let changed = apply_properties_dialog_result(
            &mut state,
            crate::properties::dialog::PropertiesDialogResult::None,
        );
        assert!(!changed);
        assert_eq!(state.schematic.components[0].value, "1k");
    }
}
