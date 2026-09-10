//! Annotation publication, dependent references and shared project history.

use super::*;
use crate::product::{ObjectRevision, SimulationPlanId};
use crate::state::{
    AnnotationObject, AnnotationPosition, ConfigurationBlackBoxPolicy, ConfigurationModelProfile,
    ConfigurationSetDefinition, ConfigurationSetId, Point, ProtectedReferencePolicy, RenumberOrder,
    RenumberRequest, RenumberScope, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
    SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SchematicObjectKey,
    SchematicProbe, UnresolvedBindingPolicy,
};

struct Fixture {
    state: AppState,
    candidate: DesignManagementCatalog,
    sources: Vec<u64>,
    dependents: Vec<u64>,
    plan: SimulationPlanId,
    configuration: ConfigurationSetId,
}

fn fixture(names: &[&str]) -> Fixture {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let owner = state.workspace.active_schematic_reference();
    let plan = state.sim_setup.stable_analysis_plan().unwrap().id();
    let mut sources = Vec::new();
    let mut dependents = Vec::new();
    let mut objects = Vec::new();
    for (index, name) in names.iter().enumerate() {
        let x = (names.len() - index) as i32 * 100;
        let id = state
            .schematic
            .add_component(ComponentType::VoltageSource, Point::new(x, 0));
        state
            .schematic
            .components
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .name = (*name).to_owned();
        sources.push(id);
        let dependent = state
            .schematic
            .add_component(ComponentType::Cccs, Point::new(x, 100));
        state
            .schematic
            .components
            .iter_mut()
            .find(|c| c.id == dependent)
            .unwrap()
            .params = format!("vref={name}");
        dependents.push(dependent);
        let output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            format!("current {index}"),
            format!("I({name})"),
            SavedOutputCompatibility::OpTranAc,
            SavedOutputPolicy::EveryAcceptedPoint,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .unwrap();
        let mut probe = SchematicProbe::new(
            1000 + index as u64,
            Point::new(x, 200),
            format!("I({name})"),
            Some(format!("I({name})")),
        )
        .unwrap();
        probe.bind_saved_output(plan, output.id);
        state.schematic.probes.push(probe);
        state.workspace.add_saved_output(plan, output).unwrap();
        objects.push(AnnotationObject {
            object: SchematicObjectKey::new(&owner.key(), id).unwrap(),
            current_reference: (*name).to_owned(),
            device_family: "V".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition { x: x.into(), y: 0 },
            connectivity_order: Some(index as u64),
            locked: false,
            external: false,
            imported: false,
        });
    }
    let configuration = state
        .workspace
        .configuration_sets
        .create(ConfigurationSetDefinition {
            name: "Annotation test".to_owned(),
            root: owner,
            dut_path: format!("/{}", names[0]),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            overrides: Vec::new(),
            owner: "test".to_owned(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        })
        .unwrap();
    state.schematic.clear_undo_history();
    state.schematic.is_dirty = false;
    state.sync_active_schematic_to_workspace();
    let candidate = annotation_candidate(&state, objects);
    Fixture {
        state,
        candidate,
        sources,
        dependents,
        plan,
        configuration,
    }
}

fn annotation_candidate(
    state: &AppState,
    objects: Vec<AnnotationObject>,
) -> DesignManagementCatalog {
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects,
    };
    let mut candidate = state.workspace.design_management.clone();
    let preview = candidate
        .annotation()
        .preview_renumbering(&request)
        .unwrap();
    candidate
        .annotation_mut()
        .commit_renumbering(&preview, &request)
        .unwrap();
    candidate
}

fn publish(fixture: &mut Fixture) {
    let state = &mut fixture.state;
    let before = state.workspace.design_management.clone();
    let transaction = state
        .prepare_design_management_schematic_transaction(&fixture.candidate)
        .unwrap();
    let committed_revision = state
        .workspace
        .replace_design_management(fixture.candidate.clone())
        .unwrap();
    state.apply_design_management_schematic_transaction(&transaction);
    state.record_design_management_transaction(DesignManagementHistoryEntry {
        description: "renumber schematic references".to_owned(),
        owner: state.workspace.active_schematic_reference(),
        before,
        after: state.workspace.design_management.clone(),
        before_schematics: transaction.before,
        after_schematics: transaction.after,
        references: transaction.references,
        committed_revision,
    });
}

fn assert_references(fixture: &Fixture, names: &[&str]) {
    let state = &fixture.state;
    for (index, name) in names.iter().enumerate() {
        assert_eq!(
            state
                .schematic
                .components
                .iter()
                .find(|c| c.id == fixture.sources[index])
                .unwrap()
                .name,
            *name
        );
        assert_eq!(
            state
                .schematic
                .components
                .iter()
                .find(|c| c.id == fixture.dependents[index])
                .unwrap()
                .params,
            format!("vref={name}")
        );
        assert_eq!(
            state.schematic.probes[index].source_expression.as_deref(),
            Some(format!("I({name})").as_str())
        );
        let output = &state
            .workspace
            .plan_data(fixture.plan)
            .unwrap()
            .saved_outputs[index];
        assert_eq!(output.source_expression, format!("I({name})"));
        assert_eq!(
            state.schematic.probes[index].saved_output_id,
            Some(output.id)
        );
    }
    assert_eq!(
        state
            .workspace
            .configuration_sets
            .find(fixture.configuration)
            .unwrap()
            .dut_path(),
        format!("/{}", names[0])
    );
}

#[test]
fn annotation_publish_and_history_update_the_scoped_schematic_atomically() {
    let mut fixture = fixture(&["V42"]);
    publish(&mut fixture);
    assert_references(&fixture, &["V1"]);
    assert!(fixture.state.undo_project_design().unwrap().is_some());
    assert_references(&fixture, &["V42"]);
    assert!(fixture.state.redo_project_design().unwrap().is_some());
    assert_references(&fixture, &["V1"]);
    assert_eq!(
        fixture
            .state
            .workspace
            .configuration_sets
            .find(fixture.configuration)
            .unwrap()
            .revision(),
        4
    );
    assert_eq!(
        fixture
            .state
            .workspace
            .plan_data(fixture.plan)
            .unwrap()
            .saved_outputs[0]
            .revision,
        ObjectRevision::new(4).unwrap()
    );
    assert!(!fixture.state.schematic.can_undo());
}

#[test]
fn annotation_swaps_preserve_every_reference_in_both_history_directions() {
    let mut fixture = fixture(&["V1", "V2"]);
    publish(&mut fixture);
    assert_references(&fixture, &["V2", "V1"]);
    fixture.state.undo_project_design().unwrap();
    assert_references(&fixture, &["V1", "V2"]);
    fixture.state.redo_project_design().unwrap();
    assert_references(&fixture, &["V2", "V1"]);
}

#[test]
fn annotation_preparation_refuses_before_any_owner_or_history_changes() {
    for failure in ["malformed reference", "output revision", "read only"] {
        let mut fixture = fixture(&["V42"]);
        match failure {
            "malformed reference" => {
                fixture
                    .state
                    .schematic
                    .components
                    .iter_mut()
                    .find(|c| c.id == fixture.dependents[0])
                    .unwrap()
                    .params = "vref='V42".to_owned()
            }
            "output revision" => {
                fixture
                    .state
                    .workspace
                    .plan_data_mut(fixture.plan)
                    .unwrap()
                    .saved_outputs[0]
                    .revision = ObjectRevision::new(u64::MAX).unwrap()
            }
            "read only" => fixture.state.workspace.open_views[0].read_only_reference = true,
            _ => unreachable!(),
        }
        let before = SchematicSnapshot::capture(&fixture.state.schematic);
        let configurations = fixture.state.workspace.configuration_sets.clone();
        let outputs = fixture
            .state
            .workspace
            .plan_data(fixture.plan)
            .unwrap()
            .saved_outputs
            .clone();
        let revision = fixture.state.workspace.project.revision();
        assert!(
            fixture
                .state
                .prepare_design_management_schematic_transaction(&fixture.candidate)
                .is_err(),
            "{failure}"
        );
        assert!(before.is_equal_state(&fixture.state.schematic));
        assert_eq!(fixture.state.workspace.configuration_sets, configurations);
        assert_eq!(
            fixture
                .state
                .workspace
                .plan_data(fixture.plan)
                .unwrap()
                .saved_outputs,
            outputs
        );
        assert_eq!(fixture.state.workspace.project.revision(), revision);
        assert!(fixture.state.project_undo_sequence().is_none());
    }
}

#[test]
fn annotation_history_refusal_does_not_partly_restore_names_or_catalogs() {
    let mut fixture = fixture(&["V42"]);
    publish(&mut fixture);
    fixture
        .state
        .workspace
        .plan_data_mut(fixture.plan)
        .unwrap()
        .saved_outputs[0]
        .revision = ObjectRevision::new(u64::MAX).unwrap();
    let catalog = fixture.state.workspace.design_management.clone();
    let revision = fixture.state.workspace.project.revision();
    assert!(fixture.state.undo_project_design().is_err());
    assert_references(&fixture, &["V1"]);
    assert_eq!(fixture.state.workspace.design_management, catalog);
    assert_eq!(fixture.state.workspace.project.revision(), revision);
    assert!(fixture.state.project_undo_sequence().is_some());
}

#[test]
fn annotation_updates_reused_masters_and_probes_in_independent_document_roots() {
    for original_names in [["X1", "X2"], ["stage1", "stage2"]] {
        verify_reused_master_annotation(original_names);
    }
}

fn verify_reused_master_annotation(original_names: [&str; 2]) {
    use crate::state::{LibraryCellInstance, OpenCellView, View, ViewType};
    let mut fixture = fixture(&["V42"]);
    let state = &mut fixture.state;
    state
        .workspace
        .ensure_library_model(&mut state.library_manager);
    let root = state.workspace.active_schematic_reference();
    let child_ref = CellViewRef::new(&root.library, "child", "schematic");
    let other_ref = CellViewRef::new(&root.library, "other", "schematic");
    for reference in [&child_ref, &other_ref] {
        state
            .library_manager
            .get_library_mut(&reference.library)
            .unwrap()
            .get_or_create_cell(&reference.cell)
            .add_view(View::new("schematic", ViewType::Schematic));
        state
            .workspace
            .open_views
            .push(OpenCellView::new(reference.clone(), ViewType::Schematic));
    }
    let open = state
        .workspace
        .open_views
        .iter_mut()
        .find(|open| open.reference == child_ref)
        .unwrap();
    open.occurrence = crate::state::workspace::DocumentOccurrence::rooted(root.clone());
    open.occurrence
        .descend(original_names[0].to_owned(), child_ref.clone());
    let mut child = std::mem::take(&mut state.schematic);
    child.probes = vec![
        SchematicProbe::new(
            1000,
            Point::origin(),
            format!("I(/{}/V42)", original_names[0]),
            Some(format!("I(/{}/V42)", original_names[0])),
        )
        .unwrap(),
    ];
    state
        .workspace
        .schematic_buffers
        .insert(child_ref.key(), child);
    for (index, name) in original_names.iter().enumerate() {
        let id = state.schematic.add_library_cell_component(
            Point::new((2 - index as i32) * 100, 0),
            LibraryCellInstance::new(&root.library, "child", "schematic"),
        );
        state
            .schematic
            .components
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap()
            .name = (*name).to_owned();
        state.schematic.probes.push(
            SchematicProbe::new(
                2000 + index as u64,
                Point::origin(),
                format!("I(/{name}/V42)"),
                Some(format!("I(/{name}/V42)")),
            )
            .unwrap(),
        );
    }
    let mut other = SchematicState::default();
    let id = other.add_library_cell_component(
        Point::origin(),
        LibraryCellInstance::new(&root.library, "child", "schematic"),
    );
    other
        .components
        .iter_mut()
        .find(|c| c.id == id)
        .unwrap()
        .name = "X5".to_owned();
    other.probes.push(
        SchematicProbe::new(
            3000,
            Point::origin(),
            "I(/X5/V42)",
            Some("I(/X5/V42)".to_owned()),
        )
        .unwrap(),
    );
    state
        .workspace
        .schematic_buffers
        .insert(other_ref.key(), other);
    state
        .workspace
        .plan_data_mut(fixture.plan)
        .unwrap()
        .saved_outputs[0]
        .source_expression = format!("I(/{}/V42)", original_names[1]);
    let configuration = state
        .workspace
        .configuration_sets
        .find(fixture.configuration)
        .unwrap();
    let mut definition = configuration.definition().clone();
    definition.dut_path = format!("/{}/V42", original_names[0]);
    state
        .workspace
        .configuration_sets
        .update(fixture.configuration, configuration.revision(), definition)
        .unwrap();
    state.schematic.clear_undo_history();
    state.sync_active_schematic_to_workspace();
    let mut objects = vec![AnnotationObject {
        object: SchematicObjectKey::new(&child_ref.key(), fixture.sources[0]).unwrap(),
        current_reference: "V42".to_owned(),
        device_family: "V".to_owned(),
        sheet_id: None,
        hierarchy_path: format!("/{}", original_names[0]),
        position: AnnotationPosition { x: 0, y: 0 },
        connectivity_order: Some(1),
        locked: false,
        external: false,
        imported: false,
    }];
    objects.extend(
        state
            .schematic
            .components
            .iter()
            .map(|component| AnnotationObject {
                object: SchematicObjectKey::new(&root.key(), component.id).unwrap(),
                current_reference: component.name.clone(),
                device_family: "X".to_owned(),
                sheet_id: None,
                hierarchy_path: "/top".to_owned(),
                position: AnnotationPosition {
                    x: component.pos.x.into(),
                    y: 0,
                },
                connectivity_order: Some(component.id),
                locked: false,
                external: false,
                imported: false,
            }),
    );
    fixture.candidate = annotation_candidate(state, objects);
    let active_configuration = state.workspace.configuration_sets.active_configuration_id();
    publish(&mut fixture);
    let assert_sources = |fixture: &Fixture, name: &str| {
        let state = &fixture.state;
        let parents = if name == "V1" {
            ["X2", "X1"]
        } else {
            original_names
        };
        let child = &state.workspace.schematic_buffers[&child_ref.key()];
        assert_eq!(
            child
                .components
                .iter()
                .find(|c| c.id == fixture.sources[0])
                .unwrap()
                .name,
            name
        );
        assert_eq!(
            child
                .components
                .iter()
                .find(|c| c.id == fixture.dependents[0])
                .unwrap()
                .params,
            format!("vref={name}")
        );
        assert_eq!(
            child.probes[0].source_expression.as_deref(),
            Some(format!("I(/{}/{name})", parents[0]).as_str())
        );
        for (index, parent) in parents.iter().enumerate() {
            assert_eq!(
                state.schematic.probes[index].source_expression.as_deref(),
                Some(format!("I(/{parent}/{name})").as_str())
            );
        }
        assert_eq!(
            state.workspace.schematic_buffers[&other_ref.key()].probes[0]
                .source_expression
                .as_deref(),
            Some(format!("I(/X5/{name})").as_str())
        );
        assert_eq!(
            state
                .workspace
                .plan_data(fixture.plan)
                .unwrap()
                .saved_outputs[0]
                .source_expression,
            format!("I(/{}/{name})", parents[1])
        );
        assert_eq!(
            state
                .workspace
                .configuration_sets
                .find(fixture.configuration)
                .unwrap()
                .dut_path(),
            format!("/{}/{name}", parents[0])
        );
        let occurrence = &state
            .workspace
            .open_views
            .iter()
            .find(|open| open.reference == child_ref)
            .unwrap()
            .occurrence;
        assert_eq!(occurrence.steps[0].instance_name, parents[0]);
        assert_eq!(occurrence.terminal_master(), &child_ref);
        assert_eq!(
            state.workspace.configuration_sets.active_configuration_id(),
            active_configuration
        );
    };
    assert_sources(&fixture, "V1");
    fixture.state.undo_project_design().unwrap();
    assert_sources(&fixture, "V42");
    fixture.state.redo_project_design().unwrap();
    assert_sources(&fixture, "V1");
}
