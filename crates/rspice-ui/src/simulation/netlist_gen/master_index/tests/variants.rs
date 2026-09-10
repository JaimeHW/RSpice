//! The frozen hierarchy and generated deck must describe the active variant.

use super::*;
use crate::state::{
    AssemblyVariantDraft, ComponentSubstitution, SchematicObjectKey, VariantInheritance,
    VariantObjectOverride, VariantQualificationPlan, VariantQualificationState,
};

fn fixture(configured: bool) -> (ProjectWorkspace, LibraryManager, SchematicState) {
    let mut libraries = LibraryManager::new();
    let mut user = Library::new("user");
    let mut top_cell = Cell::new("top");
    top_cell.add_view(View::new("schematic", ViewType::Schematic));
    user.add_cell(top_cell);
    libraries.add_library(user);
    let mut work = Library::new("work");
    for name in ["div", "alternate"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        work.add_cell(cell);
    }
    libraries.add_library(work);
    let mut workspace = ProjectWorkspace::default();
    for (name, value) in [("div", "1k"), ("alternate", "7k")] {
        workspace.schematic_buffers.insert(
            CellViewRef::new("work", name, "schematic").key(),
            two_port_master(value),
        );
    }
    let top = top_with_instance(&["a", "b"]);
    workspace
        .schematic_buffers
        .insert(workspace.active_view.key(), top.clone());
    if configured {
        workspace
            .configuration_sets
            .create(ConfigurationSetDefinition {
                name: "Variant testbench".to_owned(),
                root: workspace.active_view.clone(),
                dut_path: "/".to_owned(),
                executable_view_policy: vec!["schematic".to_owned()],
                stop_views: Vec::new(),
                unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "test".to_owned(),
            })
            .unwrap();
    }
    (workspace, libraries, top)
}

fn activate(
    workspace: &mut ProjectWorkspace,
    reference: &CellViewRef,
    component: u64,
    change: VariantObjectOverride,
) {
    let variant = workspace
        .design_management
        .variants_mut()
        .create(AssemblyVariantDraft {
            name: "Qualified variant".to_owned(),
            parent_id: None,
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides: BTreeMap::from([(
                SchematicObjectKey::new(&reference.key(), component).unwrap(),
                change,
            )]),
        })
        .unwrap();
    workspace
        .design_management
        .variants_mut()
        .set_active(variant)
        .unwrap();
}

fn replacement() -> VariantObjectOverride {
    VariantObjectOverride::Substitute {
        replacement: ComponentSubstitution {
            library: "work".to_owned(),
            cell: "alternate".to_owned(),
            view: "schematic".to_owned(),
            value_override: None,
            model_section: None,
            port_equivalence_digest: Some(ContentDigest::from_bytes([9; 32])),
            qualification: VariantQualificationState::Current,
        },
    }
}

fn omission() -> VariantObjectOverride {
    VariantObjectOverride::DoNotPopulate {
        approval_reference: "reviewed omission".to_owned(),
    }
}

#[test]
fn a_variant_replacement_owns_the_plan_and_emitted_master() {
    for configured in [false, true] {
        let (mut workspace, libraries, top) = fixture(configured);
        let active = workspace.active_view.clone();
        let original = workspace
            .design_projection(&libraries, &active, &top)
            .unwrap();
        let instance = top.components.iter().find(|c| c.name == "X1").unwrap().id;
        activate(&mut workspace, &active, instance, replacement());
        let projection = workspace
            .design_projection(&libraries, &active, &top)
            .unwrap();
        assert!(!std::sync::Arc::ptr_eq(&original, &projection));
        let placed = projection.plan().binding(&instance_path("/X1")).unwrap();
        assert_eq!(placed.resolved_reference().cell, "alternate");
        let hierarchy = HierarchySource::from_design_projection(&libraries, &projection);
        let result =
            generate_netlist_hierarchical(projection.root_schematic().unwrap(), &[], &hierarchy);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert_eq!(
            subckt_headers(&result.netlist),
            vec![".subckt alternate a b"]
        );
        assert!(
            result
                .netlist
                .lines()
                .any(|line| line.starts_with('R') && line.ends_with(" 7k"))
        );
        assert!(instance_line(&result.netlist, "X1").ends_with(" alternate"));
        assert_eq!(
            original
                .plan()
                .binding(&instance_path("/X1"))
                .unwrap()
                .resolved_reference()
                .cell,
            "div"
        );
        assert_eq!(
            top.components
                .iter()
                .find(|c| c.id == instance)
                .unwrap()
                .library_cell
                .as_ref()
                .unwrap()
                .cell,
            "div"
        );
        assert!(std::sync::Arc::ptr_eq(
            &projection,
            &workspace
                .design_projection(&libraries, &active, &top)
                .unwrap()
        ));
    }
}

#[test]
fn a_primitive_substitution_inside_a_buffered_master_enters_the_hierarchy() {
    for configured in [false, true] {
        let (mut workspace, libraries, top) = fixture(configured);
        let child = CellViewRef::new("work", "div", "schematic");
        let resistor = workspace.schematic_buffers[&child.key()]
            .components
            .iter()
            .find(|c| c.kind == ComponentType::Resistor)
            .unwrap()
            .id;
        activate(&mut workspace, &child, resistor, replacement());
        let projection = workspace
            .design_projection(&libraries, &workspace.active_view, &top)
            .unwrap();
        let nested = projection
            .plan()
            .binding(&instance_path("/X1/R1"))
            .expect("the replacement introduces a child instance");
        assert_eq!(nested.resolved_reference().cell, "alternate");
        let hierarchy = HierarchySource::from_design_projection(&libraries, &projection);
        let result =
            generate_netlist_hierarchical(projection.root_schematic().unwrap(), &[], &hierarchy);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(
            result
                .netlist
                .lines()
                .any(|line| line.starts_with("XR1 ") && line.ends_with(" alternate")),
            "{}",
            result.netlist
        );
        assert!(
            result
                .netlist
                .lines()
                .any(|line| line.starts_with('R') && line.ends_with(" 7k"))
        );
        assert_eq!(
            workspace.schematic_buffers[&child.key()]
                .components
                .iter()
                .find(|c| c.id == resistor)
                .unwrap()
                .kind,
            ComponentType::Resistor
        );
    }
}

#[test]
fn omitted_instances_do_not_require_or_emit_their_missing_master() {
    for configured in [false, true] {
        let (mut workspace, libraries, mut top) = fixture(configured);
        let active = workspace.active_view.clone();
        let omitted =
            top.add_library_cell_component(Point::new(300, 0), binding("absent", &["a", "b"]));
        activate(&mut workspace, &active, omitted, omission());
        let projection = workspace
            .design_projection(&libraries, &active, &top)
            .expect("an omitted instance cannot block the populated circuit");
        assert!(projection.plan().binding(&instance_path("/X2")).is_none());
        let hierarchy = HierarchySource::from_design_projection(&libraries, &projection);
        let result =
            generate_netlist_hierarchical(projection.root_schematic().unwrap(), &[], &hierarchy);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert_eq!(subckt_headers(&result.netlist), vec![".subckt div a b"]);
        assert!(!result.netlist.lines().any(|line| line.starts_with("X2 ")));
        assert!(top.components.iter().any(|c| c.id == omitted));
    }
}

#[test]
fn a_variant_cannot_silently_remove_the_configured_dut() {
    let (mut workspace, libraries, top) = fixture(true);
    let active = workspace.active_view.clone();
    let configuration = workspace.configuration_sets.active().unwrap().clone();
    let mut definition = configuration.definition().clone();
    definition.dut_path = "/X1".to_owned();
    workspace
        .configuration_sets
        .update(configuration.id(), configuration.revision(), definition)
        .unwrap();
    let original = workspace
        .design_projection(&libraries, &active, &top)
        .unwrap();
    let instance = top.components.iter().find(|c| c.name == "X1").unwrap().id;
    activate(&mut workspace, &active, instance, omission());
    let error = workspace
        .design_projection(&libraries, &active, &top)
        .expect_err("the selected DUT no longer exists in this variant");
    assert!(error.to_string().contains("configured DUT path"), "{error}");
    assert!(original.plan().binding(&instance_path("/X1")).is_some());
    assert!(top.components.iter().any(|c| c.id == instance));
}
