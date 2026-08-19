//! Cases the resolver's own path and view-name handling answer for.
//!
//! The design root is implicit, and these hold the consequences: a row never
//! spells it, a configuration written before it became implicit still names
//! the same instances, a pattern is measured against the instances the walk
//! reached, and an instance the grammar cannot name is reported instead of
//! dropped.
//!
//! A configured view is likewise a name and only a name: a stop selects the
//! view it spells, whatever that view's type is called, and a name that lands
//! on a view no source can stand in for descends and says so.

use super::*;
use crate::state::Point;
use crate::state::workspace::tests::{add_schematic_master, instance};

/// A configuration whose ordered views and stops are the caller's, over the
/// bootstrapped root and one instance below it.
fn view_policy_configuration(
    name: &str,
    executable_view_policy: &[&str],
    stop_views: &[&str],
) -> crate::state::ConfigurationSetDefinition {
    crate::state::ConfigurationSetDefinition {
        name: name.to_owned(),
        root: CellViewRef::default_top(),
        dut_path: "/X1".to_owned(),
        executable_view_policy: executable_view_policy
            .iter()
            .map(|view| (*view).to_owned())
            .collect(),
        stop_views: stop_views.iter().map(|view| (*view).to_owned()).collect(),
        unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
        black_box_policy:
            crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
        overrides: Vec::new(),
        model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
        owner: "Analog design".to_owned(),
    }
}

/// Add one source-backed view to a cell that already has a schematic master.
///
/// The name is the caller's and the type is stated separately, which is the
/// whole point: a view called `spice_tt` is a SPICE view whose name is not
/// `spice`.
#[cfg(not(target_arch = "wasm32"))]
fn add_source_view(
    libraries: &mut LibraryManager,
    cell: (&str, &str),
    view_name: &str,
    view_type: ViewType,
    source_path: &std::path::Path,
) {
    let mut view = View::new(view_name, view_type).with_path(source_path.to_path_buf());
    view.metadata
        .insert("netlist.ports".to_owned(), "a,b".to_owned());
    view.metadata
        .insert("netlist.module".to_owned(), cell.1.to_owned());
    libraries
        .get_library_mut(cell.0)
        .expect("the library exists")
        .get_or_create_cell(cell.1)
        .add_view(view);
}

/// A cell whose schematic instantiates one child, so a stop that holds is
/// visible as the absence of that child's row.
fn parent_with_one_child(child_library: &str, child_cell: &str) -> SchematicState {
    let mut schematic = SchematicState::default();
    schematic.add_library_cell_component(Point::new(40, 40), instance(child_library, child_cell));
    schematic
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn custom_stop_view_name_stops_the_hierarchy() {
    let source_path = std::env::temp_dir().join(format!(
        "rspice-custom-stop-view-{}.cir",
        uuid::Uuid::new_v4()
    ));
    std::fs::write(&source_path, ".subckt amp a b\nRload a b 1k\n.ends amp\n")
        .expect("the corner source writes");

    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer")
        .add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "amp",
        parent_with_one_child("work", "bias"),
    );
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "bias",
        SchematicState::default(),
    );
    add_source_view(
        &mut libraries,
        ("work", "amp"),
        "spice_tt",
        ViewType::Spice,
        &source_path,
    );
    workspace
        .configuration_sets
        .create(view_policy_configuration(
            "Typical corner",
            &["spice_tt", "schematic"],
            &["spice_tt"],
        ))
        .expect("a configuration may name its own views");

    let active = workspace.active_view.clone();
    let root = workspace
        .schematic_buffers
        .get(&CellViewRef::default_top().key())
        .expect("top buffer")
        .clone();
    let projection = workspace
        .configuration_execution_projection(&libraries, &active, &root)
        .expect("the corner view resolves");
    let resolution = workspace.resolve_hierarchy(&libraries);

    let stopped = resolution
        .bindings
        .iter()
        .find(|binding| binding.instance_paths == ["/X1"])
        .expect("the DUT row");
    assert_eq!(stopped.reference.view, "spice_tt");
    assert_eq!(stopped.stop_view.as_deref(), Some("spice_tt"));
    assert!(stopped.warnings.is_empty());
    assert!(
        projection
            .plan()
            .and_then(|plan| plan.binding(&InstancePath::parse("/X1").expect("the DUT path")))
            .is_some_and(ConfigurationExecutionBinding::stop_boundary),
        "the named stop is the executable boundary"
    );
    assert!(
        resolution
            .bindings
            .iter()
            .all(|binding| binding.reference.cell != "bias"),
        "a stopped hierarchy has no descent below its boundary"
    );

    let _ = std::fs::remove_file(source_path);
}

#[test]
fn custom_executable_view_name_is_accepted() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer")
        .add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "amp",
        SchematicState::default(),
    );
    let fast = CellViewRef::new("work", "amp", "schematic_fast");
    libraries
        .get_library_mut("work")
        .expect("the library exists")
        .get_or_create_cell("amp")
        .add_view(View::new("schematic_fast", ViewType::Schematic));
    workspace
        .schematic_buffers
        .insert(fast.key(), SchematicState::default());
    workspace
        .configuration_sets
        .create(view_policy_configuration(
            "Fast schematic",
            &["schematic_fast", "schematic"],
            &[],
        ))
        .expect("a custom view name validates");

    let resolution = workspace.resolve_hierarchy(&libraries);

    let dut = resolution
        .bindings
        .iter()
        .find(|binding| binding.instance_paths == ["/X1"])
        .expect("the DUT row");
    assert!(dut.status.is_resolved(), "diagnostic: {:?}", dut.diagnostic);
    assert_eq!(dut.reference.view, "schematic_fast");
    assert_eq!(dut.view_search_order, ["schematic_fast", "schematic"]);
}

#[test]
fn stop_view_matching_a_schematic_warns_and_descends() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer")
        .add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "amp",
        parent_with_one_child("work", "bias"),
    );
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "bias",
        SchematicState::default(),
    );
    workspace
        .configuration_sets
        .create(view_policy_configuration(
            "Schematic stop",
            &["schematic"],
            &["schematic"],
        ))
        .expect("a stop naming a schematic view is well formed");

    let resolution = workspace.resolve_hierarchy(&libraries);

    let dut = resolution
        .bindings
        .iter()
        .find(|binding| binding.instance_paths == ["/X1"])
        .expect("the DUT row");
    assert!(dut.status.is_resolved(), "a warning is not a failure");
    assert!(
        dut.warnings.iter().any(|warning| {
            warning.contains("stop view 'schematic' matched by name")
                && warning.contains("descends into it")
        }),
        "warnings: {:?}",
        dut.warnings
    );
    assert!(
        resolution
            .bindings
            .iter()
            .any(|binding| binding.reference.cell == "bias"),
        "a stop that cannot terminate the hierarchy does not terminate it"
    );
}

#[test]
fn a_configuration_written_before_the_implicit_root_names_the_same_instances() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let top = workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer");
    top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    top.add_library_cell_component(Point::new(80, 20), instance("work", "amp"));
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "amp",
        SchematicState::default(),
    );
    workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Saved before the migration".to_owned(),
            root: CellViewRef::default_top(),
            dut_path: "/top/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![crate::state::ConfigurationSetOverride {
                instance_path: "/top/X2".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: Some("schematic".to_owned()),
                model_section: None,
                eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
            }],
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Analog design".to_owned(),
        })
        .expect("create configuration");

    let resolution = workspace.resolve_hierarchy(&libraries);

    assert!(
        resolution.is_valid(),
        "the legacy spelling names instances that exist: {:?}",
        resolution
            .bindings
            .iter()
            .filter_map(|binding| binding.diagnostic.as_deref())
            .collect::<Vec<_>>()
    );
    let dut = resolution
        .bindings
        .iter()
        .find(|binding| binding.instance_paths == ["/X1"])
        .expect("the DUT is named without the root segment");
    assert_eq!(dut.stop_view, None);
    let overridden = resolution
        .bindings
        .iter()
        .find(|binding| binding.instance_paths == ["/X2"])
        .expect("the override reaches the instance it was authored against");
    assert_eq!(overridden.stop_view.as_deref(), Some("schematic"));
}

#[test]
fn an_override_pattern_is_measured_against_the_instances_the_walk_reached() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    workspace
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: "Empty testbench".to_owned(),
            root: CellViewRef::default_top(),
            // The pre-implicit-root spelling of the design root itself, which
            // the walk reaches and the requirement is therefore met by.
            dut_path: "/top".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: vec![crate::state::ConfigurationSetOverride {
                instance_path: "/top/*".to_owned(),
                executable_views: vec!["schematic".to_owned()],
                stop_view: None,
                model_section: None,
                eligible_platforms: crate::state::ConfigurationPlatform::ALL.to_vec(),
            }],
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Local project".to_owned(),
        })
        .expect("create configuration");

    let resolution = workspace.resolve_hierarchy(&libraries);

    assert!(
        resolution.bindings.iter().any(|binding| {
            binding
                .diagnostic
                .as_deref()
                .is_some_and(|diagnostic| diagnostic.contains("/* matches no instance"))
        }),
        "the empty testbench has nothing for the pattern to select: {:?}",
        resolution
            .bindings
            .iter()
            .map(|binding| binding.diagnostic.as_deref())
            .collect::<Vec<_>>()
    );
    assert!(
        resolution.bindings.iter().all(|binding| {
            binding.instance_paths.iter().all(|path| path != "/top")
                && binding
                    .diagnostic
                    .as_deref()
                    .is_none_or(|diagnostic| !diagnostic.contains("/top"))
        }),
        "the design root is implicit, so no row or diagnostic spells it"
    );
}

#[test]
fn an_instance_the_grammar_cannot_name_is_reported_rather_than_skipped() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let top = workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer");
    top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    top.components.last_mut().expect("placed instance").name = "X 1".to_owned();
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "amp",
        SchematicState::default(),
    );

    let resolution = workspace.resolve_hierarchy(&libraries);

    assert_eq!(resolution.total_instances, 2);
    assert_eq!(resolution.resolved_instances, 1);
    let reported = resolution
        .bindings
        .iter()
        .find(|binding| binding.status == HierarchyBindingStatus::Unresolved)
        .expect("the unnameable instance has a row of its own");
    assert!(
        reported
            .diagnostic
            .as_deref()
            .is_some_and(|diagnostic| diagnostic.contains("X 1")),
        "diagnostic: {:?}",
        reported.diagnostic
    );
}

#[test]
fn no_configuration_still_builds_a_plan() {
    let mut workspace = ProjectWorkspace::default();
    let mut libraries = LibraryManager::default();
    workspace.ensure_library_model(&mut libraries);
    let top = workspace
        .schematic_buffers
        .get_mut(&CellViewRef::default_top().key())
        .expect("top buffer");
    top.add_library_cell_component(Point::new(20, 20), instance("work", "amp"));
    top.add_library_cell_component(Point::new(120, 20), instance("work", "amp"));
    add_schematic_master(
        &mut libraries,
        &mut workspace,
        "work",
        "amp",
        SchematicState::default(),
    );
    assert!(
        workspace.configuration_sets.active().is_none(),
        "this fixture deliberately has no configuration"
    );
    let active = workspace.active_view.clone();
    let root = workspace
        .schematic_buffers
        .get(&active.key())
        .expect("top buffer")
        .clone();

    let projection = workspace
        .configuration_execution_projection(&libraries, &active, &root)
        .expect("an unconfigured workspace projects");
    let plan = projection
        .plan()
        .expect("every resolution produces an execution plan");

    assert_eq!(plan.configuration_id(), None);
    assert_eq!(plan.configuration_revision(), 0);
    let x1 = InstancePath::parse("/X1").expect("a fixture path");
    let x2 = InstancePath::parse("/X2").expect("a fixture path");
    let first = plan.binding(&x1).expect("X1 is planned");
    let second = plan.binding(&x2).expect("X2 is planned");
    assert_eq!(
        first.binding_closure_digest(),
        second.binding_closure_digest(),
        "two occurrences of one cellview with identical subtrees are one master"
    );
    assert_eq!(plan.occurrence_master(&x1), plan.occurrence_master(&x2));
    let key = plan.occurrence_master(&x1).expect("X1 binds a master");
    let record = plan.master(key).expect("the master is recorded");
    assert_eq!(record.name(), "amp");
    assert_eq!(record.occurrences, vec![x1.clone(), x2.clone()]);
    assert_eq!(
        plan.masters().len(),
        1,
        "the root is the deck, not one of its masters"
    );
}
