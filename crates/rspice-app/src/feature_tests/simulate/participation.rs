//! The Run Set's participation over the whole application: a refusal names
//! the points it drops the way the table does, on the parity fixture.

/// A refusal names the dropped points the way the point table names them.
///
/// It used to print their keys — `dimension-cload-value-001+…` — which is
/// the studio's internal identity for a row the operator is looking at on
/// the same page under a different name. A refusal has to name the object
/// it is about in the spelling the reader can find.
#[test]
fn a_refusal_names_dropped_points_the_way_the_table_does() {
    use crate::simulation::plan::AnalysisKind;
    use rspice_simulation_contract::run_set::RunSetDimensionKind;

    use super::page_runset_parity::{app_with, scope_to_points};
    use crate::workbench::surfaces::simulate::participation::reconcile_selections;
    use rspice_simulation_contract::run_set;

    let mut app = app_with(&[AnalysisKind::OperatingPoint]);
    for dimension in &mut app.state.model.sim_setup.run_set.dimensions {
        dimension.enabled = matches!(
            dimension.kind,
            RunSetDimensionKind::Temperature | RunSetDimensionKind::ProcessSection
        );
    }
    let keys = scope_to_points(&mut app.state, AnalysisKind::OperatingPoint, &[0]);
    let key = keys.first().expect("one point is selected").clone();
    let label = run_set::point_key_label(&app.state.model.sim_setup.run_set, &key);
    assert_ne!(label, key, "the fixture's point has a name of its own");

    // Switch one axis off. Every key the selection holds names a point of
    // the two-axis space and none of them survives, while every value it
    // was built from is still declared — which is exactly the edit whose
    // refusal was unreadable.
    for dimension in &mut app.state.model.sim_setup.run_set.dimensions {
        if dimension.kind == RunSetDimensionKind::ProcessSection {
            dimension.enabled = false;
        }
    }

    let Err(refusal) = reconcile_selections(&app.state.model) else {
        panic!("dropping every point an instance is scoped to is a refusal");
    };
    assert!(
        refusal.contains(&label),
        "the refusal names the point as {label:?}; it said {refusal:?}"
    );
    assert!(
        !refusal.contains(&key),
        "the refusal must not print the point key {key:?}: {refusal:?}"
    );
}
