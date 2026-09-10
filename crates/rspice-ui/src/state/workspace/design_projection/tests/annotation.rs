//! Legacy annotation receipts must project names and references together.

use super::*;
use crate::state::{
    AnnotationObject, AnnotationPosition, ProtectedReferencePolicy, RenumberOrder, RenumberRequest,
    RenumberScope, SchematicObjectKey,
};

fn retain_annotation(
    workspace: &mut ProjectWorkspace,
    reference: &CellViewRef,
    schematic: &SchematicState,
    ids: &[u64],
) {
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: ids
            .iter()
            .map(|id| {
                let component = schematic
                    .components
                    .iter()
                    .find(|component| component.id == *id)
                    .unwrap();
                AnnotationObject {
                    object: SchematicObjectKey::new(&reference.key(), *id).unwrap(),
                    current_reference: component.name.clone(),
                    device_family: component.kind.spice_prefix().to_owned(),
                    sheet_id: None,
                    hierarchy_path: "/".to_owned(),
                    position: AnnotationPosition {
                        x: component.pos.x.into(),
                        y: component.pos.y.into(),
                    },
                    connectivity_order: Some(*id),
                    locked: false,
                    external: false,
                    imported: false,
                }
            })
            .collect(),
    };
    let annotation = workspace.design_management.annotation_mut();
    let preview = annotation.preview_renumbering(&request).unwrap();
    annotation.commit_renumbering(&preview, &request).unwrap();
}

#[test]
fn unapplied_annotation_projects_controlled_source_references_with_names() {
    let (mut workspace, libraries, reference, mut active) = workspace_with_two_cell_views();
    let voltage = active.add_component(ComponentType::VoltageSource, Point::new(400, 0));
    set_name(&mut active, voltage, "V42");
    let controlled = active.add_component(ComponentType::Cccs, Point::new(500, 0));
    active
        .components
        .iter_mut()
        .find(|component| component.id == controlled)
        .unwrap()
        .params = "vref=V42 gain=2".to_owned();
    retain_annotation(&mut workspace, &reference, &active, &[voltage]);
    let original = crate::state::SchematicSnapshot::capture(&active);
    let catalog = workspace.design_management.clone();
    let projection = projection_of(&workspace, &libraries, &reference, &active);
    let projected = projection.root_schematic().unwrap();
    assert_eq!(
        projected
            .components
            .iter()
            .find(|component| component.id == voltage)
            .unwrap()
            .name,
        "V1"
    );
    let controlled = projected
        .components
        .iter()
        .find(|component| component.id == controlled)
        .unwrap();
    let parameters = crate::state::parse_replacement_parameters_strict(&controlled.params).unwrap();
    assert_eq!(parameters["vref"], "V1");
    assert_eq!(parameters["gain"], "2");
    assert!(original.is_equal_state(&active));
    assert_eq!(workspace.design_management, catalog);
    assert!(Arc::ptr_eq(
        &projection,
        &projection_of(&workspace, &libraries, &reference, &active)
    ));
    let repeated = projection_of(&workspace, &libraries, &reference, projected);
    assert_eq!(
        repeated.root_schematic().unwrap().components,
        projected.components
    );
}

#[test]
fn projection_resolves_swapped_windings_from_the_original_names() {
    let (mut workspace, libraries, reference, mut active) = workspace_with_two_cell_views();
    let first = active.add_component(ComponentType::Inductor, Point::new(600, 0));
    let second = active.add_component(ComponentType::Inductor, Point::new(400, 0));
    set_name(&mut active, first, "L1");
    set_name(&mut active, second, "L2");
    active
        .components
        .iter_mut()
        .find(|component| component.id == first)
        .unwrap()
        .params = "coupled_to=L2 coupling_factor=-0.5".to_owned();
    let coupling = active.add_component(ComponentType::CoupledInductor, Point::new(800, 0));
    active
        .components
        .iter_mut()
        .find(|component| component.id == coupling)
        .unwrap()
        .params = "inductors='l1, L2'".to_owned();
    retain_annotation(&mut workspace, &reference, &active, &[first, second]);
    let original = crate::state::SchematicSnapshot::capture(&active);
    let projection = projection_of(&workspace, &libraries, &reference, &active);
    let components = &projection.root_schematic().unwrap().components;
    let first = components
        .iter()
        .find(|component| component.id == first)
        .unwrap();
    assert_eq!(first.name, "L2");
    let parameters = crate::state::parse_replacement_parameters_strict(&first.params).unwrap();
    assert_eq!(parameters["coupled_to"], "L1");
    assert_eq!(parameters["coupling_factor"], "-0.5");
    assert_eq!(
        components
            .iter()
            .find(|component| component.id == second)
            .unwrap()
            .name,
        "L1"
    );
    let coupling = components
        .iter()
        .find(|component| component.id == coupling)
        .unwrap();
    assert_eq!(
        crate::state::parse_params_string(&coupling.params)["inductors"],
        "L2, L1"
    );
    assert!(original.is_equal_state(&active));
}

#[test]
fn a_failed_annotation_projection_leaves_the_source_and_cached_projection_intact() {
    for malformed in [false, true] {
        let (mut workspace, libraries, reference, mut active) = workspace_with_two_cell_views();
        let voltage = active.add_component(ComponentType::VoltageSource, Point::new(400, 0));
        set_name(&mut active, voltage, "V42");
        retain_annotation(&mut workspace, &reference, &active, &[voltage]);
        let valid = active.clone();
        let cached = projection_of(&workspace, &libraries, &reference, &valid);
        if malformed {
            let dependent = active.add_component(ComponentType::Cccs, Point::new(500, 0));
            active
                .components
                .iter_mut()
                .find(|component| component.id == dependent)
                .unwrap()
                .params = "vref='unfinished".to_owned();
        } else {
            let collision = active.add_component(ComponentType::VoltageSource, Point::new(500, 0));
            set_name(&mut active, collision, "V1");
        }
        let original = crate::state::SchematicSnapshot::capture(&active);
        let catalog = workspace.design_management.clone();
        let error = workspace
            .design_projection(&libraries, &reference, &active)
            .unwrap_err();
        assert!(error.to_string().contains("reference annotation"));
        assert!(error.to_string().contains(&reference.key()));
        assert!(original.is_equal_state(&active));
        assert_eq!(workspace.design_management, catalog);
        assert!(Arc::ptr_eq(
            &cached,
            &projection_of(&workspace, &libraries, &reference, &valid)
        ));
    }
}

#[test]
fn omitted_variant_components_do_not_participate_in_annotation_references() {
    use crate::state::{
        AssemblyVariantDraft, VariantInheritance, VariantObjectOverride, VariantQualificationPlan,
    };
    let (mut workspace, libraries, reference, mut active) = workspace_with_two_cell_views();
    let voltage = active.add_component(ComponentType::VoltageSource, Point::new(400, 0));
    set_name(&mut active, voltage, "V42");
    let omitted = active.add_component(ComponentType::Cccs, Point::new(500, 0));
    active
        .components
        .iter_mut()
        .find(|component| component.id == omitted)
        .unwrap()
        .params = "vref='unfinished".to_owned();
    retain_annotation(&mut workspace, &reference, &active, &[voltage]);
    let variant = workspace
        .design_management
        .variants_mut()
        .create(AssemblyVariantDraft {
            name: "Without controlled source".to_owned(),
            parent_id: None,
            inheritance: VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: VariantQualificationPlan::InvalidateAffectedTests,
            overrides: BTreeMap::from([(
                SchematicObjectKey::new(&reference.key(), omitted).unwrap(),
                VariantObjectOverride::DoNotPopulate {
                    approval_reference: "reviewed-omission".to_owned(),
                },
            )]),
        })
        .unwrap();
    workspace
        .design_management
        .variants_mut()
        .set_active(variant)
        .unwrap();
    let projection = projection_of(&workspace, &libraries, &reference, &active);
    let components = &projection.root_schematic().unwrap().components;
    assert!(!components.iter().any(|component| component.id == omitted));
    assert_eq!(
        components
            .iter()
            .find(|component| component.id == voltage)
            .unwrap()
            .name,
        "V1"
    );
    assert!(
        active
            .components
            .iter()
            .any(|component| component.id == omitted)
    );
}
