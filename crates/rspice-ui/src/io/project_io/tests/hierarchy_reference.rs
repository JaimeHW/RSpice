//! The frozen hierarchical reference project, as one released schema wrote it.
//!
//! `hierarchy_reference_legacy.rspiceproj` is the exact output of
//! `serialize_project_file` for the project
//! `workbench::examples::hierarchy_reference::build` produces, captured once
//! and never regenerated. The live builder answers for what the application
//! can author today; these bytes answer for what it must still be able to
//! open, which is a different question the moment a schema moves.
//!
//! Every later schema change extends this file's suite rather than replacing
//! the fixture: a migration lane adds the cases that pin how its own change
//! reads these bytes, and the assertions below stay as the floor.

use super::*;

/// The frozen bytes. Never rewrite them — a regenerated fixture proves only
/// that today's writer agrees with today's reader.
const FROZEN_PROJECT: &str = include_str!("hierarchy_reference_legacy.rspiceproj");

const USER_LIBRARY: &str = "user";
const VENDOR_LIBRARY: &str = "vendor";
const AMP_VIEW_KEY: &str = "user/amp/schematic";
const FILTER_CELL: &str = "filter";
const CROSS_SHEET_NET: &str = "mid";
const DUT_PATH: &str = "/top/X1";
const SAVED_OUTPUT_EXPRESSION: &str = "V(out)";

#[test]
fn the_frozen_hierarchical_reference_still_loads_and_validates() {
    let project = load_project_text(FROZEN_PROJECT, None).expect("the frozen reference loads");
    project.validate().expect("the frozen reference validates");
    assert!(
        project.simulation_results_warning.is_none(),
        "{:?}",
        project.simulation_results_warning
    );

    for view in [
        "user/top/schematic",
        AMP_VIEW_KEY,
        "user/filter/schematic",
        "vendor/filter/schematic",
    ] {
        assert!(
            project.workspace.schematic_buffers.contains_key(view),
            "{view} lost its schematic buffer"
        );
    }
    for library in [USER_LIBRARY, VENDOR_LIBRARY] {
        assert!(
            project
                .libraries
                .get_library(library)
                .and_then(|library| library.get_cell(FILTER_CELL))
                .is_some(),
            "{library} lost its '{FILTER_CELL}' cell"
        );
    }
    assert_eq!(
        project
            .libraries
            .get_library(USER_LIBRARY)
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .map(|view| view.view_type),
        Some(ViewType::Symbol)
    );
}

#[test]
fn the_frozen_hierarchical_reference_keeps_its_sheets_configuration_and_outputs() {
    let project = load_project_text(FROZEN_PROJECT, None).expect("the frozen reference loads");

    let catalog = project
        .workspace
        .design_management
        .sheet_catalog(AMP_VIEW_KEY)
        .expect("the amp sheet catalog is retained");
    assert_eq!(catalog.sheets().len(), 2);
    assert_eq!(
        catalog
            .cross_sheet_ports()
            .iter()
            .map(|port| port.definition().net_name.as_str())
            .collect::<Vec<_>>(),
        vec![CROSS_SHEET_NET]
    );

    let configuration = project
        .workspace
        .configuration_sets
        .active()
        .expect("the reference configuration is still active");
    assert_eq!(configuration.dut_path(), DUT_PATH);
    assert_eq!(configuration.root().key(), "user/top/schematic");

    let plan = project
        .execution_context
        .as_ref()
        .and_then(|context| context.simulation_plan.analysis_plan.as_ref())
        .expect("the frozen reference carries a stable analysis plan")
        .id();
    assert_eq!(
        project.workspace.plan_data(plan).map(|payload| payload
            .saved_outputs
            .iter()
            .map(|output| output.source_expression.as_str())
            .collect::<Vec<_>>()),
        Some(vec![SAVED_OUTPUT_EXPRESSION])
    );
}

#[test]
fn the_frozen_hierarchical_reference_rewrites_to_a_project_that_loads_again() {
    let project = load_project_text(FROZEN_PROJECT, None).expect("the frozen reference loads");
    let rewritten = serialize_project_file(&project).expect("the loaded reference re-serializes");
    load_project_text(&rewritten, None)
        .expect("the re-serialized reference loads")
        .validate()
        .expect("the re-serialized reference validates");
}
