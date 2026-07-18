//! Opening the property editor for a schematic component.

use super::AppState;

/// Open the tabbed property editor for `component_id`, populated from the
/// component's registry sheet and current values. Refused on read-only
/// views — the editor is an edit path.
pub(crate) fn open_property_editor(state: &mut AppState, component_id: u64) {
    if state.tabbed_property_dialog.open || state.dialogs.object_properties.open {
        return;
    }
    if state.deny_read_only_edit() {
        return;
    }
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
            crate::properties::ComponentPropertySession::new(
                component.clone(),
                state.design_execution_epoch,
                state.active_schematic_epoch,
                state.workspace.active_view.display_path(),
            ),
        );
    } else {
        log::warn!("No property sheet found for {:?}", component_type);
    }
}

/// Open the canonical properties workflow for the one selected editable
/// schematic object. Component instances retain their schema-driven editor;
/// buses and taps use the topology-aware generic transaction specified by the
/// workbench mockup.
pub(crate) fn open_selected_object_properties(state: &mut AppState) -> bool {
    if state.tabbed_property_dialog.open || state.dialogs.object_properties.open {
        return false;
    }
    if state.deny_read_only_edit() {
        return false;
    }
    if let Some(component_id) = state.schematic.selection.single_component() {
        open_property_editor(state, component_id);
        return state.tabbed_property_dialog.open;
    }
    if let Some(tap_id) = state.schematic.selection.single_bus_tap()
        && let Some(tap) = state.schematic.bus_taps.iter().find(|tap| tap.id == tap_id)
    {
        state.dialogs.object_properties.open_bus_tap(
            tap,
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        return true;
    }
    if let Some(bus_id) = state.schematic.selection.single_bus()
        && let Some(bus) = state.schematic.buses.iter().find(|bus| bus.id == bus_id)
    {
        state.dialogs.object_properties.open_bus(
            bus,
            state.design_execution_epoch,
            state.active_schematic_epoch,
            state.schematic.topology_version(),
            state.workspace.active_view.display_path(),
        );
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Point};

    #[test]
    fn selected_bus_and_tap_open_the_generic_transaction() {
        let mut state = AppState::default();
        let bus = Bus::segment(
            41,
            Point::new(0, 0),
            Point::new(10, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            42,
            &bus,
            Point::new(5, 0),
            Point::new(5, 5),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        state.schematic.buses.push(bus);
        state.schematic.bus_taps.push(tap);

        state.schematic.selection.select_only_bus(41);
        assert!(open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(super::super::ObjectPropertiesDraft::Bus(_))
        ));
        state.dialogs.object_properties.close();

        state.schematic.selection.select_only_bus_tap(42);
        assert!(open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(super::super::ObjectPropertiesDraft::BusTap(_))
        ));
    }

    #[test]
    fn generic_properties_refuses_read_only_schematics() {
        let mut state = AppState::default();
        state
            .schematic
            .buses
            .push(Bus::segment(1, Point::new(0, 0), Point::new(10, 0), None).unwrap());
        state.schematic.selection.select_only_bus(1);
        state.schematic.read_only = true;

        assert!(!open_selected_object_properties(&mut state));
        assert!(!state.dialogs.object_properties.open);
    }

    #[test]
    fn stale_selected_object_ids_fail_closed_without_opening_a_dialog() {
        let mut state = AppState::default();
        state.schematic.selection.select_only_bus(9001);
        assert!(!open_selected_object_properties(&mut state));
        assert!(!state.dialogs.object_properties.open);

        state.schematic.selection.select_only_bus_tap(9002);
        assert!(!open_selected_object_properties(&mut state));
        assert!(!state.dialogs.object_properties.open);
    }

    #[test]
    fn a_retained_property_transaction_cannot_be_overwritten_by_another_owner() {
        let mut state = AppState::default();
        let bus = Bus::segment(
            1,
            Point::new(0, 0),
            Point::new(10, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        state.schematic.buses.push(bus);
        state.schematic.selection.select_only_bus(1);
        assert!(open_selected_object_properties(&mut state));

        state.schematic.selection.select_only_bus(999);
        assert!(!open_selected_object_properties(&mut state));
        assert!(matches!(
            state.dialogs.object_properties.draft,
            Some(super::super::ObjectPropertiesDraft::Bus(ref draft)) if draft.original.id == 1
        ));
    }
}
