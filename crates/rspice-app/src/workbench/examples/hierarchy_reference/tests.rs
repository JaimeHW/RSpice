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

use rspice_app_services::io::project_io::{load_project_text, serialize_project_file};
use rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot;
use rspice_design::hierarchy::HierarchySource;
use rspice_simulation::netlist_gen::generate_netlist_hierarchical;

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
    project.file.validate().expect("the snapshot validates");

    let text = serialize_project_file(&project).expect("the reference project serializes");
    let restored = load_project_text(&text, None).expect("the serialized project loads");
    restored
        .file
        .validate()
        .expect("the restored project validates");
    assert!(
        restored.file.simulation_results_warning.is_none(),
        "{:?}",
        restored.file.simulation_results_warning
    );

    for view in [
        &reference.top,
        &reference.amp_schematic,
        &reference.user_filter,
        &reference.vendor_filter,
    ] {
        assert!(
            restored
                .file
                .workspace
                .schematic_buffers
                .contains_key(&view.key()),
            "{} lost its schematic buffer",
            view.display_path()
        );
    }
    assert_eq!(
        restored
            .file
            .libraries
            .get_library(&reference.amp_symbol.library)
            .and_then(|library| library.get_cell(&reference.amp_symbol.cell))
            .and_then(|cell| cell.get_view(&reference.amp_symbol.view))
            .map(|view| view.view_type),
        Some(ViewType::Symbol)
    );

    let catalog = restored
        .file
        .workspace
        .design_management
        .sheet_catalog(&reference.amp_schematic.key())
        .expect("the amp sheet catalog is retained");
    assert_eq!(
        catalog
            .sheets()
            .iter()
            .map(rspice_app_state::state::DesignSheet::id)
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
        .file
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
            .file
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
        .model
        .workspace
        .configuration_execution_projection(
            &state.model.library_manager,
            &state.model.workspace.content.active_view,
            &state.model.schematic,
        )
        .expect("the active configuration resolves into an execution plan");
    let plan = projection.plan();

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

    let hierarchy = HierarchySource::from_execution_projection(
        state.model.library_manager.catalog(),
        &projection,
    );
    let result = generate_netlist_hierarchical(
        projection
            .root_schematic()
            .expect("the projection carries the root schematic"),
        &[],
        &hierarchy,
        &rspice_simulation::netlist_gen::NetlistSourceData::new(
            &rspice_app_state::simulation::table_route::SourceFiles,
        ),
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
