//! Preflight describes the same populated circuit as execution.

use super::*;
use crate::workbench::examples::hierarchy_reference::omit_missing_instance;

#[test]
fn omitted_instances_do_not_reappear_as_preflight_hierarchy_blockers() {
    let mut state = crate::workbench::examples::hierarchy_reference::build().state;
    let baseline = collect_report(&state);
    assert!(
        baseline
            .blockers
            .iter()
            .all(|issue| issue.check != "Hierarchy binding")
    );
    omit_missing_instance(&mut state);
    let authored = serde_json::to_value(&state.model.schematic).unwrap();
    assert!(
        state
            .model
            .workspace
            .configuration_execution_projection(
                &state.model.library_manager,
                &state.model.workspace.content.active_view,
                &state.model.schematic,
            )
            .is_ok()
    );
    let report = collect_report(&state);
    assert!(
        report
            .blockers
            .iter()
            .all(|issue| issue.check != "Hierarchy binding"),
        "{:?}",
        report.blockers
    );
    assert!(
        report
            .advisories
            .iter()
            .all(|issue| !format!("{issue:?}").contains("omitted_master"))
    );
    assert_eq!(
        serde_json::to_value(&state.model.schematic).unwrap(),
        authored
    );
    assert!(
        state
            .model
            .workspace
            .resolve_hierarchy_with_active(
                &state.model.library_manager,
                &state.model.workspace.content.active_view,
                &state.model.schematic,
            )
            .bindings
            .iter()
            .any(|binding| binding.reference.cell == "omitted_master"
                && !binding.status.is_resolved()),
        "authored dependency inspection still sees the retained object"
    );
}
