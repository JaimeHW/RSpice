//! What the trace manager calls each row it lists.
//!
//! A trace's label is the reader's to write, and until they write it the label
//! is the engine's spelling of the vector derivation provisioned the trace
//! from. A bus bit reaches the engine as `data#3`, a name no drawing can
//! contain, so a row is shown in the notation its bus was drawn in for exactly
//! as long as the label is still the one derivation wrote — and in the reader's
//! own words from the moment they rename it. Neither the draft nor the document
//! is rewritten: the stored label is the key an edit matches on.

use std::collections::BTreeSet;

use super::super::active_project_visualization_document_id;
use crate::simulation::netlist_gen::bus_notations;
use crate::workbench::app_state::AppState;

/// The label each row of the visibility draft paints, in draft order.
pub(super) fn row_labels(state: &AppState) -> Vec<String> {
    let notations = bus_notations(&state.workspace, &state.schematic);
    let derived = derived_labels(state);
    state
        .workbench
        .visualization_studio
        .draft_trace_visibility
        .iter()
        .map(|(label, _)| shown_label(label, derived.contains(label), &notations.display(label)))
        .collect()
}

/// One row's name.
///
/// `label_is_derived` states that the trace still carries the label derivation
/// wrote for it, which is the only label a display surface may re-spell.
fn shown_label(label: &str, label_is_derived: bool, design_spelling: &str) -> String {
    if label_is_derived {
        design_spelling.to_owned()
    } else {
        label.to_owned()
    }
}

/// Labels on the active pane that are still exactly the vector each trace was
/// provisioned from.
fn derived_labels(state: &AppState) -> BTreeSet<String> {
    let Some(document_id) = active_project_visualization_document_id(state) else {
        return BTreeSet::new();
    };
    let active_pane = state.workbench.visualization_studio.active_pane;
    state
        .workspace
        .visualization_document(document_id)
        .map(|document| {
            document
                .traces()
                .iter()
                .filter(|trace| active_pane == Some(trace.pane_id.get()))
                .filter(|trace| trace.source_signal() == Some(trace.label.as_str()))
                .map(|trace| trace.label.clone())
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Bus, BusDeclaration, Point};

    /// A design whose only declaration is `DATA[7:0]`, drawn once.
    fn design_declaring_data() -> AppState {
        let mut state = AppState::default();
        let declaration = BusDeclaration::parse("DATA[7:0]").expect("a bus declaration");
        state.schematic.buses.push(
            Bus::segment(1, Point::new(0, 0), Point::new(100, 0), Some(declaration))
                .expect("a drawn bus"),
        );
        state
    }

    #[test]
    fn a_default_labelled_bit_is_shown_in_the_notation_its_bus_was_drawn_in() {
        let state = design_declaring_data();
        let notations = bus_notations(&state.workspace, &state.schematic);
        let label = "x1.data#3";

        assert_eq!(
            shown_label(label, true, &notations.display(label)),
            "x1.data[3]"
        );
    }

    /// The same name, typed by the reader rather than written by derivation,
    /// reaches the row exactly as they spelled it.
    #[test]
    fn a_renamed_trace_is_shown_in_the_words_the_reader_typed() {
        let state = design_declaring_data();
        let notations = bus_notations(&state.workspace, &state.schematic);
        let label = "x1.data#3";

        assert_eq!(
            shown_label(label, false, &notations.display(label)),
            "x1.data#3"
        );
    }
}
