//! Snap to standard value: each selected resistor, capacitor and inductor
//! takes the nearest member of the project's preferred-number series.
//!
//! The value keeps its unit text, a value that is an expression or a
//! parameter has no nearest member and is left alone, and the whole command
//! is one undo step.

use super::{Model, WorkbenchSession, sync_active_schematic};
use crate::state::{Component, ComponentType};
use rspice_vocabulary::quantity::engineering::parse_engineering_value;

/// Whether a part's value is drawn from a preferred-number series.
pub(crate) const fn takes_standard_value(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor
    )
}

/// The selected parts the command would change, each beside its snapped copy.
fn snaps(model: &Model) -> Vec<(Component, Component)> {
    let series = model.workspace.content.standard_value_series;
    let selection = &model.schematic.session.editor.selection;
    model
        .schematic
        .document()
        .components
        .iter()
        .filter(|part| selection.components.contains(&part.id) && takes_standard_value(part.kind))
        .filter_map(|part| {
            // Judged by the number, not the text: `4700`, `4.7 kΩ` and
            // `10.0k` are members already, and rewriting them as `4.7k`,
            // `4.7kΩ` and `10k` changed no value while the command counted
            // them as snapped.
            let text = part.value.trim();
            let value = parse_engineering_value(text).ok()?;
            let nearest = series.snap(value)?;
            if (nearest - value).abs() <= 1e-9 * value.abs() {
                return None;
            }
            let mut candidate = part.clone();
            candidate.value = series.snap_text(text)?;
            Some((part.clone(), candidate))
        })
        .collect()
}

/// Why Snap to standard value cannot run now, or `None` when it can.
pub(crate) fn snap_block_reason(model: &Model) -> Option<&'static str> {
    if model.schematic_edit_read_only() {
        Some("the active schematic is read-only")
    } else if snaps(model).is_empty() {
        Some("select a resistor, capacitor or inductor whose value is off the series")
    } else {
        None
    }
}

/// Snap to standard value as a command runs it, and say what it did: one path
/// for the Design menu, the palette and the canvas menu.
pub(crate) fn run_snap_to_standard_value(model: &mut Model, wb: &mut WorkbenchSession) {
    if model.schematic_edit_read_only() {
        wb.prompt("The active schematic is read-only.");
        return;
    }
    model.retain_selection_on_active_sheet();
    let edits = snaps(model);
    if edits.is_empty() {
        wb.prompt("The selected values are standard values already.");
        return;
    }
    let series = model.workspace.content.standard_value_series;
    match crate::state::bulk_edit::replace_components(
        &mut model.schematic,
        edits,
        "snap to standard value",
    ) {
        Ok(changed) => {
            sync_active_schematic(model, wb);
            wb.prompt(format!(
                "Snapped {changed} {} to the {} series.",
                if changed == 1 { "value" } else { "values" },
                series.label()
            ));
        }
        Err(error) => wb.prompt(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;
    use crate::workbench::design::test_host::DesignTestHost;
    use rspice_vocabulary::quantity::ESeries;

    /// The selected resistor and capacitor snap to the project's series with
    /// their units kept; a parameter value and an unselected part stay put,
    /// and one undo puts both values back.
    #[test]
    fn selected_values_snap_to_the_project_series_as_one_undo_step() {
        let mut state = DesignTestHost::default();
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
        assert_eq!(snap_block_reason(&state.model), None);

        run_snap_to_standard_value(&mut state.model, &mut state.workbench);

        let values = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .map(|part| part.value.as_str())
            .collect::<Vec<_>>();
        assert_eq!(values, ["4.7kΩ", "100n", "{rload}", "4.6k"]);
        assert!(
            snap_block_reason(&state.model).is_some(),
            "nothing left to snap"
        );

        state.model.schematic.undo();
        assert_eq!(
            state.model.schematic.document().components[0].value,
            "4.6kΩ"
        );
        assert_eq!(state.model.schematic.document().components[1].value, "105n");
    }

    /// Values on the series already, however they are written, keep their
    /// text: with only those selected the command is off, and beside one
    /// real change it counts and rewrites that one alone.
    #[test]
    fn values_already_on_the_series_keep_their_text() {
        let mut state = DesignTestHost::default();
        state.model.workspace.content.standard_value_series = ESeries::E12;
        let part = |id, kind, name: &str, value: &str| {
            Component::new(id, kind, Point::new(id as i32 * 200, 0)).with_name_value(name, value)
        };
        let standard = ["4700", "10.0k", "4.7K", "4.7 kΩ", "1meg"];
        let mut parts = standard
            .iter()
            .enumerate()
            .map(|(index, value)| {
                part(
                    index as u64 + 1,
                    ComponentType::Resistor,
                    &format!("R{}", index + 1),
                    value,
                )
            })
            .collect::<Vec<_>>();
        parts.push(part(6, ComponentType::Capacitor, "C1", "2.2µF"));
        parts.push(part(7, ComponentType::Resistor, "R7", "4.6k"));
        state.model.schematic.document_mut_for_test().components = parts;
        state.model.schematic.init_undo_history();
        let selection = &mut state.model.schematic.session.editor.selection;
        for id in 1..=6 {
            selection.select_component(id);
        }
        assert!(
            snap_block_reason(&state.model).is_some(),
            "nothing selected is off the series"
        );

        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(7);
        assert_eq!(snap_block_reason(&state.model), None);
        run_snap_to_standard_value(&mut state.model, &mut state.workbench);

        let values = state
            .model
            .schematic
            .document()
            .components
            .iter()
            .map(|part| part.value.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            values,
            ["4700", "10.0k", "4.7K", "4.7 kΩ", "1meg", "2.2µF", "4.7k"]
        );
        assert_eq!(
            state.prompt_text(),
            Some("Snapped 1 value to the E12 series.")
        );
    }
}
