//! Cases the resolver's own path handling answers for.
//!
//! The design root is implicit, and these hold the consequences: a row never
//! spells it, a configuration written before it became implicit still names
//! the same instances, a pattern is measured against the instances the walk
//! reached, and an instance the grammar cannot name is reported instead of
//! dropped.

use super::*;
use crate::state::Point;
use crate::state::workspace::tests::{add_schematic_master, instance};

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
