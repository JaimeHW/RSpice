//! What the reference project has to be true of before any wave builds on it.
//!
//! Two things are pinned here: the project the builder produces survives the
//! real save/load boundary, and it netlists through the same projection the
//! run path uses. Master names are read out of the execution plan rather than
//! spelled out, because naming is the resolver's decision and a test that
//! spells it out pins the wrong thing. What *is* spelled out is the shape the
//! reference exists to hold: one cell instantiated twice is one definition,
//! and one cell name in two libraries is two.

use super::*;

use crate::io::project_io::{load_project_text, serialize_project_file};
use crate::simulation::netlist_gen::{HierarchySource, generate_netlist_hierarchical};
use crate::workbench::lifecycle::project_lifecycle::snapshot;

/// The four instance paths `top` executes, in placement order.
fn instance_paths() -> Vec<InstancePath> {
    AMP_INSTANCES
        .into_iter()
        .chain(FILTER_INSTANCES)
        .map(HierarchyReference::instance_path)
        .collect()
}

#[test]
fn the_reference_project_round_trips_through_the_persisted_format() {
    let reference = build();
    let project = snapshot(&reference.state).expect("the reference project snapshots");
    project.validate().expect("the snapshot validates");

    let text = serialize_project_file(&project).expect("the reference project serializes");
    let restored = load_project_text(&text, None).expect("the serialized project loads");
    restored.validate().expect("the restored project validates");
    assert!(
        restored.simulation_results_warning.is_none(),
        "{:?}",
        restored.simulation_results_warning
    );

    for view in [
        &reference.top,
        &reference.amp_schematic,
        &reference.user_filter,
        &reference.vendor_filter,
    ] {
        assert!(
            restored
                .workspace
                .schematic_buffers
                .contains_key(&view.key()),
            "{} lost its schematic buffer",
            view.display_path()
        );
    }
    assert_eq!(
        restored
            .libraries
            .get_library(&reference.amp_symbol.library)
            .and_then(|library| library.get_cell(&reference.amp_symbol.cell))
            .and_then(|cell| cell.get_view(&reference.amp_symbol.view))
            .map(|view| view.view_type),
        Some(ViewType::Symbol)
    );

    let catalog = restored
        .workspace
        .design_management
        .sheet_catalog(&reference.amp_schematic.key())
        .expect("the amp sheet catalog is retained");
    assert_eq!(
        catalog
            .sheets()
            .iter()
            .map(crate::state::DesignSheet::id)
            .collect::<Vec<_>>(),
        reference.amp_sheets.to_vec()
    );
    assert_eq!(
        catalog
            .cross_sheet_ports()
            .iter()
            .map(|port| port.definition().net_name.as_str())
            .collect::<Vec<_>>(),
        vec![CROSS_SHEET_NET]
    );

    let configuration = restored
        .workspace
        .configuration_sets
        .active()
        .expect("the reference configuration stays active");
    assert_eq!(configuration.id(), reference.configuration);
    assert_eq!(
        configuration.dut_path(),
        HierarchyReference::instance_path(AMP_INSTANCES[0]).to_string()
    );
    assert_eq!(
        restored
            .workspace
            .plan_data(reference.plan)
            .map(|payload| payload
                .saved_outputs
                .iter()
                .map(|output| output.source_expression.as_str())
                .collect::<Vec<_>>()),
        Some(vec![SAVED_OUTPUT_EXPRESSION])
    );
}

#[test]
fn the_reference_project_netlists_through_the_configured_execution_projection() {
    let reference = build();
    let state = &reference.state;
    let projection = state
        .workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.active_view,
            &state.schematic,
        )
        .expect("the active configuration resolves into an execution plan");
    let plan = projection
        .plan()
        .expect("an active configuration produces an execution plan");

    let masters = instance_paths()
        .into_iter()
        .map(|path| {
            let master = plan
                .occurrence_master(&path)
                .and_then(|key| plan.master(key))
                .unwrap_or_else(|| panic!("{path} resolves to an emitted master"))
                .name()
                .to_owned();
            (path, master)
        })
        .collect::<Vec<_>>();
    let distinct = masters
        .iter()
        .map(|(_, master)| master.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        distinct.len(),
        3,
        "amp's two occurrences share one definition and filter's two do not: {masters:?}"
    );
    assert_eq!(
        masters[0].1, masters[1].1,
        "two instances of one cellview that resolve identically are one master"
    );
    assert_ne!(
        masters[2].1, masters[3].1,
        "one cell name in two libraries is two masters"
    );

    let hierarchy = HierarchySource::from_execution_projection(&state.library_manager, &projection);
    let result = generate_netlist_hierarchical(
        projection
            .root_schematic()
            .expect("the projection carries the root schematic"),
        &[],
        &hierarchy,
    );
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(
        result.warnings.is_empty(),
        "warnings: {:?}",
        result.warnings
    );

    for (path, master) in &masters {
        let instance = path
            .segments()
            .last()
            .expect("an instance path names at least one instance");
        assert!(
            result
                .netlist
                .lines()
                .any(|line| line.trim() == format!(".subckt {master} {}", ports_of(instance))),
            "{path} has no definition for {master}:\n{}",
            result.netlist
        );
        assert!(
            result
                .netlist
                .lines()
                .any(|line| line.starts_with(&format!("{instance} ")) && line.ends_with(master)),
            "{path} has no instance line binding {master}:\n{}",
            result.netlist
        );
    }
    assert_eq!(
        result
            .netlist
            .lines()
            .filter(|line| line
                .trim_start()
                .to_ascii_lowercase()
                .starts_with(".subckt "))
            .count(),
        3,
        "each master is declared exactly once:\n{}",
        result.netlist
    );
    assert_eq!(
        result
            .emission_map
            .iter()
            .map(|row| (row.occurrence.to_string(), row.master.clone()))
            .collect::<Vec<_>>()
            .len(),
        masters.len(),
        "the emission map covers every occurrence: {:?}",
        result.emission_map
    );

    assert!(
        result
            .netlist
            .lines()
            .any(|line| line.split_whitespace().nth(2) == Some(CROSS_SHEET_NET)),
        "the materialized cross-sheet net is absent:\n{}",
        result.netlist
    );
    assert!(
        result
            .netlist
            .lines()
            .any(|line| line.contains(GLOBAL_SUPPLY_NET)),
        "the global supply label is absent:\n{}",
        result.netlist
    );
}

/// The `.subckt` port list one instance's master carries.
fn ports_of(instance: &str) -> String {
    if AMP_INSTANCES.contains(&instance) {
        AMP_PORTS.join(" ")
    } else if FILTER_INSTANCES.contains(&instance) {
        FILTER_PORTS.join(" ")
    } else {
        panic!("{instance} is not one of the reference instances")
    }
}

#[test]
fn the_reference_libraries_hold_the_cross_library_name_collision() {
    let reference = build();
    let libraries = &reference.state.library_manager;

    for view in [&reference.user_filter, &reference.vendor_filter] {
        assert_eq!(
            libraries
                .get_library(&view.library)
                .and_then(|library| library.get_cell(&view.cell))
                .and_then(|cell| cell.get_view(&view.view))
                .map(|view| view.view_type),
            Some(ViewType::Schematic),
            "{} is missing",
            view.display_path()
        );
    }
    assert_ne!(
        reference.user_filter.library,
        reference.vendor_filter.library
    );
    assert_eq!(reference.user_filter.cell, reference.vendor_filter.cell);
    assert!(
        !libraries
            .get_library(VENDOR_LIBRARY)
            .expect("the vendor library exists")
            .read_only,
        "the vendor library is writable in this reference"
    );

    let amp_symbol = libraries
        .get_library(&reference.amp_symbol.library)
        .and_then(|library| library.get_cell(&reference.amp_symbol.cell))
        .and_then(|cell| cell.get_view(&reference.amp_symbol.view))
        .expect("the authored amp symbol view exists");
    let document = crate::state::SymbolDocument::load_from_view(amp_symbol)
        .expect("the authored symbol document loads");
    assert_eq!(
        document
            .pins
            .iter()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        AMP_PORTS.to_vec()
    );
    assert_eq!(
        document
            .text_runs()
            .map(|(text, _)| text)
            .collect::<Vec<_>>(),
        vec![AMP_CELL.to_ascii_uppercase().as_str()]
    );
}

#[test]
fn the_reference_top_carries_the_typed_bus_and_its_scalar_tap() {
    let reference = build();
    let top = reference
        .state
        .workspace
        .schematic_buffers
        .get(&reference.top.key())
        .expect("the testbench buffer is saved");

    let bus = top.buses.first().expect("the testbench declares one bus");
    let declaration = bus
        .declaration
        .as_ref()
        .expect("the bus carries a typed declaration");
    assert_eq!(declaration.name, DATA_BUS);
    assert_eq!(declaration.width(), 4);
    let tap = top.bus_taps.first().expect("the bus has one tap");
    assert_eq!(tap.bus_id, bus.id);
    assert_eq!(tap.slice.to_string(), DATA_BUS_MEMBER);
    assert!(
        top.net_labels
            .iter()
            .any(|label| label.name == DATA_BUS_MEMBER),
        "the tapped net is unlabelled"
    );
    assert!(
        top.net_labels
            .iter()
            .any(|label| label.name == GLOBAL_SUPPLY_NET),
        "the global supply label is absent"
    );
    assert!(
        top.net_labels
            .iter()
            .any(|label| label.name == TOP_OUTPUT_NET),
        "the labelled output net is absent"
    );
}
