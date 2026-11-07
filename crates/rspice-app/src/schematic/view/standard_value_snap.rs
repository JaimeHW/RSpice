//! Snap to standard value: each selected resistor, capacitor and inductor
//! takes the nearest member of the project's preferred-number series.
//!
//! The value keeps its unit text, a value that is an expression or a
//! parameter has no nearest member and is left alone, and the whole command
//! is one undo step.

use super::AppState;
use super::sheet_visibility::retain_selection_on_active_sheet;
use crate::state::{Component, ComponentType};

/// Whether a part's value is drawn from a preferred-number series.
pub(crate) const fn takes_standard_value(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor
    )
}

/// The selected parts the command would change, each beside its snapped copy.
fn snaps(state: &AppState) -> Vec<(Component, Component)> {
    let series = state.model.workspace.content.standard_value_series;
    let selection = &state.model.schematic.session.editor.selection;
    state
        .model
        .schematic
        .document()
        .components
        .iter()
        .filter(|part| selection.components.contains(&part.id) && takes_standard_value(part.kind))
        .filter_map(|part| {
            let snapped = series.snap_text(part.value.trim())?;
            (snapped != part.value).then(|| {
                let mut candidate = part.clone();
                candidate.value = snapped;
                (part.clone(), candidate)
            })
        })
        .collect()
}

/// Why Snap to standard value cannot run now, or `None` when it can.
pub(crate) fn snap_block_reason(state: &AppState) -> Option<&'static str> {
    if state.schematic_edit_read_only() {
        Some("the active schematic is read-only")
    } else if snaps(state).is_empty() {
        Some("select a resistor, capacitor or inductor whose value is off the series")
    } else {
        None
    }
}

/// Snap to standard value as a command runs it, and say what it did: one path
/// for the Design menu, the palette and the canvas menu.
pub(crate) fn run_snap_to_standard_value(state: &mut AppState) {
    if state.schematic_edit_read_only() {
        state.prompt("The active schematic is read-only.");
        return;
    }
    retain_selection_on_active_sheet(state);
    let edits = snaps(state);
    if edits.is_empty() {
        state.prompt("The selected values are standard values already.");
        return;
    }
    let series = state.model.workspace.content.standard_value_series;
    match crate::state::bulk_edit::replace_components(
        &mut state.model.schematic,
        edits,
        "snap to standard value",
    ) {
        Ok(changed) => {
            state.sync_active_schematic_to_workspace();
            state.prompt(format!(
                "Snapped {changed} {} to the {} series.",
                if changed == 1 { "value" } else { "values" },
                series.label()
            ));
        }
        Err(error) => state.prompt(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;
    use rspice_vocabulary::quantity::ESeries;

    /// The selected resistor and capacitor snap to the project's series with
    /// their units kept; a parameter value and an unselected part stay put,
    /// and one undo puts both values back.
    #[test]
    fn selected_values_snap_to_the_project_series_as_one_undo_step() {
        let mut state = AppState::default();
        state.model.workspace.content.standard_value_series = ESeries::E12;
        let part = |id, kind, name: &str, value: &str| {
            Component::new(id, kind, Point::new(id as i32 * 200, 0)).with_name_value(name, value)
        };
        state.model.schematic.document_mut_for_test().components = vec![
            part(1, ComponentType::Resistor, "R1", "4.6kΩ"),
            part(2, ComponentType::Capacitor, "C1", "105n"),
            part(3, ComponentType::Resistor, "R2", "{rload}"),
            part(4, ComponentType::Resistor, "R3", "4.6k"),
        ];
        state.model.schematic.init_undo_history();
        let selection = &mut state.model.schematic.session.editor.selection;
        for id in [1, 2, 3] {
            selection.select_component(id);
        }
        assert_eq!(snap_block_reason(&state), None);

        run_snap_to_standard_value(&mut state);

        let values = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .map(|part| part.value.as_str())
            .collect::<Vec<_>>();
        assert_eq!(values, ["4.7kΩ", "100n", "{rload}", "4.6k"]);
        assert!(snap_block_reason(&state).is_some(), "nothing left to snap");

        state.model.schematic.undo();
        assert_eq!(
            state.model.schematic.document().components[0].value,
            "4.6kΩ"
        );
        assert_eq!(state.model.schematic.document().components[1].value, "105n");
    }
}
