//! The canvas's transient voltage annotations read through the whole application: the
//! values shown are the transient's at the results' cursor A.

use super::app_state_of;
use crate::schematic::view::scene::tests::text;
use crate::schematic::view::scene::*;
use crate::schematic::view::transient_annotation_status;
use crate::state::SchematicAnnotationVisibility;
use crate::state::{AnalysisResult, AnalysisType};
use crate::workbench::app_state::AppState;

/// [`state_with_operating_point`](crate::schematic::view::scene::tests::state_with_operating_point) made a whole application.
fn state_with_operating_point() -> AppState {
    app_state_of(crate::schematic::view::scene::tests::state_with_operating_point())
}

/// Transient voltages badge the nets and the selected device as the DC
/// operating point does, from the run's transient read at results cursor
/// A, linearly between time points, and at the run's last time point
/// while A is off the run; never from the DC operating point beside it.
#[test]
fn transient_voltages_annotate_the_transient_at_cursor_a() {
    let mut state = state_with_operating_point();
    state.model.simulation.retained.runs[0].add_analysis(
        AnalysisResult::new(2, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            crate::state::RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0e-3, 2.0e-3],
                vec![0.0, 1.0, 3.0],
                "#ffbd2e",
            ),
            crate::state::RetainedTrace::new(
                "I(vbias)",
                vec![0.0, 1.0e-3, 2.0e-3],
                vec![0.0, -1.0e-3, -3.0e-3],
                "#4fc1ff",
            ),
        ]),
    );
    state.design.schematic_visibility.annotations =
        SchematicAnnotationVisibility::TransientVoltages;
    let read = |state: &AppState| {
        operating_point_annotations(&state.canvas_facts())
            .iter()
            .map(text)
            .collect::<Vec<_>>()
    };

    assert_eq!(read(&state), ["out 3 V", "I(vbias) = -3 mA"]);
    assert_eq!(
        transient_annotation_status(&state.model, state.results.results.session.cursors.a),
        "Transient voltages at 2 ms, the end of the run. Place cursor A to choose another instant."
    );

    state.results.results.session.cursors.a = Some(1.5e-3);
    assert_eq!(read(&state), ["out 2 V", "I(vbias) = -2 mA"]);
    assert_eq!(
        transient_annotation_status(&state.model, state.results.results.session.cursors.a),
        "Transient voltages at 1.5 ms, cursor A."
    );

    state.results.results.session.cursors.a = Some(5.0);
    assert_eq!(read(&state), ["out 3 V", "I(vbias) = -3 mA"]);

    state.model.simulation.retained.runs[0].analyses.truncate(1);
    assert!(read(&state).is_empty());
    assert_eq!(
        transient_annotation_status(&state.model, state.results.results.session.cursors.a),
        "No transient result is selected: run a transient analysis to annotate its voltages."
    );
}
