//! Restore pending annotation as one project-wide reference transition.

use super::*;
use crate::state::{
    AnnotationObject, AnnotationPosition, ComponentType, ConfigurationBlackBoxPolicy,
    ConfigurationModelProfile, ConfigurationSetDefinition, LibraryCellInstance, Point,
    ProtectedReferencePolicy, RenumberOrder, RenumberRequest, RenumberScope, SavedOutput,
    SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming, SchematicObjectKey, SchematicProbe, SchematicState,
    UnresolvedBindingPolicy,
};
use std::collections::BTreeMap;

#[derive(Clone)]
struct PendingAnnotation {
    project: ProjectFile,
    root: CellViewRef,
    child: CellViewRef,
    parent: u64,
    voltage: u64,
    dependent: u64,
    outputs: [(
        crate::product::SimulationPlanId,
        crate::product::SavedOutputId,
    ); 2],
}

fn pending_annotation() -> PendingAnnotation {
    let mut project = project_with_execution_context();
    let root = project.workspace.active_schematic_reference();
    let child = CellViewRef::new("user", "annotated_child", "schematic");
    let mut cell = Cell::new(&child.cell);
    cell.add_view(View::new("schematic", ViewType::Schematic));
    project
        .libraries
        .get_library_mut("user")
        .unwrap()
        .add_cell(cell);
    let mut master = SchematicState::default();
    let voltage = master.add_component(ComponentType::VoltageSource, Point::new(100, 0));
    master
        .components
        .iter_mut()
        .find(|component| component.id == voltage)
        .unwrap()
        .name = "V42".to_owned();
    let dependent = master.add_component(ComponentType::Cccs, Point::new(200, 0));
    master
        .components
        .iter_mut()
        .find(|component| component.id == dependent)
        .unwrap()
        .params = "vref=V42".to_owned();
    project
        .workspace
        .schematic_buffers
        .insert(child.key(), master);
    let top = project
        .workspace
        .schematic_buffers
        .get_mut(&root.key())
        .unwrap();
    let parent = top.add_library_cell_component(
        Point::new(100, 0),
        LibraryCellInstance::new("user", &child.cell, "schematic"),
    );
    top.components
        .iter_mut()
        .find(|component| component.id == parent)
        .unwrap()
        .name = "X42".to_owned();
    let other = top.add_library_cell_component(
        Point::new(200, 0),
        LibraryCellInstance::new("user", &child.cell, "schematic"),
    );
    top.components
        .iter_mut()
        .find(|component| component.id == other)
        .unwrap()
        .name = "X7".to_owned();
    project
        .workspace
        .configuration_sets
        .create(ConfigurationSetDefinition {
            name: "Pending annotation".to_owned(),
            root: root.clone(),
            dut_path: "/X42".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            overrides: Vec::new(),
            owner: "test".to_owned(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        })
        .unwrap();
    let setup = &mut project.execution_context.as_mut().unwrap().simulation_plan;
    let first_plan = setup.stable_analysis_plan().unwrap().id();
    let second_plan = setup.create_plan("Other occurrence").unwrap();
    setup.activate_plan(first_plan).unwrap();
    let outputs =
        [(first_plan, "I(/X42/V42)"), (second_plan, "I(/X7/V42)")].map(|(plan, expression)| {
            let output = SavedOutput::new(
                SavedOutputKind::RawVoltageOrCurrent,
                "Source current",
                expression,
                SavedOutputCompatibility::OpTranAc,
                SavedOutputPolicy::EveryAcceptedPoint,
                SavedOutputPrecision::FullSourcePrecision,
                SavedOutputStreaming::StoreOnly,
            )
            .unwrap();
            let id = output.id;
            project.workspace.add_saved_output(plan, output).unwrap();
            (plan, id)
        });
    for (reference, id) in [(&root, 1000), (&child, 1001)] {
        let mut probe = SchematicProbe::new(
            id,
            Point::new(50, 50),
            "Source current",
            Some("I(/X42/V42)".to_owned()),
        )
        .unwrap();
        probe.bind_saved_output(first_plan, outputs[0].1);
        project
            .workspace
            .schematic_buffers
            .get_mut(&reference.key())
            .unwrap()
            .probes
            .push(probe);
    }
    project
        .workspace
        .descend_into("X42".to_owned(), child.clone(), ViewType::Schematic);
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: [
            (&root, parent, "X42", "X", "/"),
            (&child, voltage, "V42", "V", "/X42"),
        ]
        .into_iter()
        .map(|(reference, id, name, family, path)| AnnotationObject {
            object: SchematicObjectKey::new(&reference.key(), id).unwrap(),
            current_reference: name.to_owned(),
            device_family: family.to_owned(),
            sheet_id: None,
            hierarchy_path: path.to_owned(),
            position: AnnotationPosition { x: 0, y: 0 },
            connectivity_order: Some(id),
            locked: false,
            external: false,
            imported: false,
        })
        .collect(),
    };
    let annotation = project.workspace.design_management.annotation_mut();
    let preview = annotation.preview_renumbering(&request).unwrap();
    annotation.commit_renumbering(&preview, &request).unwrap();
    PendingAnnotation {
        project,
        root,
        child,
        parent,
        voltage,
        dependent,
        outputs,
    }
}

#[test]
fn reopening_pending_annotation_aligns_hierarchy_outputs_and_bound_probes_once() {
    let fixture = pending_annotation();
    let bytes = serialize_project_file(&fixture.project).unwrap();
    let loaded = load_project_text(&bytes, None).unwrap();
    let workspace = &loaded.workspace;
    let projection = workspace
        .design_projection(
            &loaded.libraries,
            &fixture.child,
            &workspace.schematic_buffers[&fixture.child.key()],
        )
        .unwrap();
    assert!(
        projection
            .plan()
            .binding(&InstancePath::parse("/X1").unwrap())
            .is_some()
    );
    assert!(
        projection
            .plan()
            .binding(&InstancePath::parse("/X42").unwrap())
            .is_none()
    );
    assert_eq!(
        workspace.configuration_sets.active().unwrap().dut_path(),
        "/X1"
    );
    assert_eq!(workspace.occurrence_path().to_string(), "/X1");
    for (reference, component_id, name) in [
        (&fixture.root, fixture.parent, "X1"),
        (&fixture.child, fixture.voltage, "V1"),
    ] {
        let schematic = &workspace.schematic_buffers[&reference.key()];
        assert_eq!(
            schematic
                .components
                .iter()
                .find(|component| component.id == component_id)
                .unwrap()
                .name,
            name
        );
        assert_eq!(
            schematic.probes[0].source_expression.as_deref(),
            Some("I(/X1/V1)")
        );
        assert_eq!(schematic.probes[0].reference, "Source current");
        assert_eq!(
            schematic.probes[0].saved_output_id,
            Some(fixture.outputs[0].1)
        );
    }
    let dependent = workspace.schematic_buffers[&fixture.child.key()]
        .components
        .iter()
        .find(|component| component.id == fixture.dependent)
        .unwrap();
    assert_eq!(
        crate::state::parse_params_string(&dependent.params)["vref"],
        "V1"
    );
    for ((plan, id), expression) in fixture.outputs.into_iter().zip(["I(/X1/V1)", "I(/X7/V1)"]) {
        let output = workspace
            .plan_data(plan)
            .unwrap()
            .saved_outputs
            .iter()
            .find(|output| output.id == id)
            .unwrap();
        assert_eq!(output.source_expression, expression);
    }
    assert_eq!(
        workspace.design_management,
        fixture.project.workspace.design_management
    );
    let again = load_project_text(&serialize_project_file(&loaded).unwrap(), None).unwrap();
    assert_eq!(
        again.workspace.configuration_sets,
        workspace.configuration_sets
    );
    assert_eq!(
        again.workspace.simulation_plan_payloads,
        workspace.simulation_plan_payloads
    );
    assert!(again.workspace_migration_warning.is_none());
}

fn damage_pending_annotation(fixture: &mut PendingAnnotation, failure: &str) {
    let master = fixture
        .project
        .workspace
        .schematic_buffers
        .get_mut(&fixture.child.key())
        .unwrap();
    match failure {
        "unrecorded name" => {
            master
                .components
                .iter_mut()
                .find(|component| component.id == fixture.voltage)
                .unwrap()
                .name = "V99".to_owned()
        }
        "collision" => {
            let id = master.add_component(ComponentType::VoltageSource, Point::new(300, 0));
            master
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .unwrap()
                .name = "V1".to_owned();
        }
        "malformed reference" => {
            master
                .components
                .iter_mut()
                .find(|component| component.id == fixture.dependent)
                .unwrap()
                .params = "vref='unfinished".to_owned()
        }
        "output revision" => {
            fixture
                .project
                .workspace
                .plan_data_mut(fixture.outputs[0].0)
                .unwrap()
                .saved_outputs[0]
                .revision = crate::product::ObjectRevision::new(u64::MAX).unwrap()
        }
        _ => unreachable!(),
    }
}

#[test]
fn annotation_restoration_refuses_before_publishing_any_project_owner() {
    for failure in [
        "unrecorded name",
        "collision",
        "malformed reference",
        "output revision",
    ] {
        let mut fixture = pending_annotation();
        damage_pending_annotation(&mut fixture, failure);
        let before = serde_json::to_value(&fixture.project.workspace).unwrap();
        let dirty = fixture.project.workspace.project_metadata_dirty;
        let snapshots: BTreeMap<_, _> = fixture
            .project
            .workspace
            .schematic_buffers
            .iter()
            .map(|(key, source)| {
                (
                    key.clone(),
                    crate::state::SchematicSnapshot::capture(source),
                )
            })
            .collect();
        let error = fixture
            .project
            .workspace
            .restore_pending_annotation(&fixture.project.libraries)
            .unwrap_err();
        assert!(!error.is_empty());
        assert_eq!(
            serde_json::to_value(&fixture.project.workspace).unwrap(),
            before,
            "{failure}"
        );
        assert_eq!(fixture.project.workspace.project_metadata_dirty, dirty);
        for (key, snapshot) in snapshots {
            assert!(snapshot.is_equal_state(&fixture.project.workspace.schematic_buffers[&key]));
        }
        let bytes = serialize_project_file(&fixture.project).unwrap();
        assert!(
            matches!(load_project_text(&bytes, None), Err(ProjectIoError::InvalidData(message)) if message.contains("reference annotation restoration failed"))
        );
    }
}

#[test]
fn restoration_follows_the_complete_recorded_name_lineage() {
    let mut fixture = pending_annotation();
    fixture
        .project
        .workspace
        .design_management
        .annotation_mut()
        .commit_manual_reference_edit(
            SchematicObjectKey::new(&fixture.child.key(), fixture.voltage).unwrap(),
            "V1",
            "V9",
        )
        .unwrap()
        .unwrap();
    let catalog = fixture.project.workspace.design_management.clone();
    let loaded =
        load_project_text(&serialize_project_file(&fixture.project).unwrap(), None).unwrap();
    let child = &loaded.workspace.schematic_buffers[&fixture.child.key()];
    assert_eq!(
        child
            .components
            .iter()
            .find(|component| component.id == fixture.voltage)
            .unwrap()
            .name,
        "V9"
    );
    let dependent = child
        .components
        .iter()
        .find(|component| component.id == fixture.dependent)
        .unwrap();
    assert_eq!(
        crate::state::parse_params_string(&dependent.params)["vref"],
        "V9"
    );
    for ((plan, id), expression) in fixture.outputs.into_iter().zip(["I(/X1/V9)", "I(/X7/V9)"]) {
        let output = loaded
            .workspace
            .plan_data(plan)
            .unwrap()
            .saved_outputs
            .iter()
            .find(|output| output.id == id)
            .unwrap();
        assert_eq!(output.source_expression, expression);
    }
    assert_eq!(loaded.workspace.design_management, catalog);
    assert!(
        loaded
            .workspace_migration_warning
            .as_deref()
            .is_some_and(|warning| warning.contains("Applied approved reference annotation"))
    );
}

fn annotation_session(project: &ProjectFile) -> AppState {
    let (sim_setup, model_library_manager, _) = project
        .execution_context
        .clone()
        .unwrap()
        .into_state(project.workspace.project.id())
        .unwrap();
    AppState {
        schematic: project
            .workspace
            .active_context_schematic()
            .cloned()
            .unwrap_or_default(),
        workspace: project.workspace.clone(),
        library_manager: project.libraries.clone(),
        sim_setup,
        model_library_manager,
        ..Default::default()
    }
}

fn restore_annotation_session(state: &AppState, ron: bool) -> AppState {
    if ron {
        ron::from_str(&ron::to_string(state).unwrap()).unwrap()
    } else {
        serde_json::from_str(&serde_json::to_string(state).unwrap()).unwrap()
    }
}

fn assert_restored_annotation(state: &AppState, fixture: &PendingAnnotation) {
    let expected =
        load_project_text(&serialize_project_file(&fixture.project).unwrap(), None).unwrap();
    for reference in [&fixture.root, &fixture.child] {
        let restored = &state.workspace.schematic_buffers[&reference.key()];
        let canonical = &expected.workspace.schematic_buffers[&reference.key()];
        assert_eq!(restored.components, canonical.components);
        assert_eq!(restored.probes, canonical.probes);
    }
    assert_eq!(
        state.workspace.configuration_sets,
        expected.workspace.configuration_sets
    );
    assert_eq!(
        state.workspace.simulation_plan_payloads,
        expected.workspace.simulation_plan_payloads
    );
    assert_eq!(state.workspace.active_view, expected.workspace.active_view);
    assert_eq!(
        serde_json::to_value(&state.workspace.open_views).unwrap(),
        serde_json::to_value(&expected.workspace.open_views).unwrap()
    );
    if let Some(active) = state.workspace.active_context_schematic() {
        assert_eq!(state.schematic.components, active.components);
    }
    assert_eq!(
        state.workspace.design_management,
        fixture.project.workspace.design_management
    );
    assert!(
        state
            .workspace
            .design_projection(
                &state.library_manager,
                &fixture.child,
                &state.workspace.schematic_buffers[&fixture.child.key()],
            )
            .is_ok()
    );
}

#[test]
fn session_restores_pending_annotation_and_every_reference_once() {
    for ron in [false, true] {
        for schematic_active in [false, true] {
            let mut fixture = pending_annotation();
            if !schematic_active {
                let reference = CellViewRef::new("user", &fixture.child.cell, "layout");
                fixture
                    .project
                    .libraries
                    .get_library_mut("user")
                    .unwrap()
                    .get_cell_mut(&fixture.child.cell)
                    .unwrap()
                    .add_view(View::new("layout", ViewType::Layout));
                fixture
                    .project
                    .workspace
                    .open_view(reference, ViewType::Layout);
            }
            let state = annotation_session(&fixture.project);
            let original = serde_json::to_value(&state.workspace).unwrap();
            let restored = restore_annotation_session(&state, ron);
            assert_restored_annotation(&restored, &fixture);
            assert_eq!(serde_json::to_value(&state.workspace).unwrap(), original);
            assert!(restored.log_buffer.entries().any(|entry| {
                entry
                    .message
                    .contains("Applied approved reference annotation")
            }));
            let repeated = restore_annotation_session(&restored, ron);
            assert_restored_annotation(&repeated, &fixture);
            assert!(!repeated.log_buffer.entries().any(|entry| {
                entry
                    .message
                    .contains("Applied approved reference annotation")
            }));
        }
    }
}

#[test]
fn session_retains_unsaved_schematic_flags_across_document_switches() {
    for ron in [false, true] {
        let mut fixture = pending_annotation();
        fixture
            .project
            .workspace
            .restore_pending_annotation(&fixture.project.libraries)
            .unwrap();
        let state = annotation_session(&fixture.project);
        let mut restored = restore_annotation_session(&state, ron);
        for reference in [&fixture.root, &fixture.child] {
            assert!(restored.workspace.schematic_buffers[&reference.key()].is_dirty);
            assert!(
                restored
                    .workspace
                    .open_views
                    .iter()
                    .any(|open| open.reference == *reference && open.dirty)
            );
        }
        assert!(restored.schematic.is_dirty);
        restored
            .workspace
            .activate_view(fixture.root.clone(), ViewType::Schematic);
        restored.schematic = restored
            .workspace
            .active_context_schematic()
            .unwrap()
            .clone();
        restored.sync_active_schematic_to_workspace();
        let repeated = restore_annotation_session(&restored, ron);
        assert!(repeated.schematic.is_dirty);
        assert!(repeated.workspace.open_views.iter().all(|open| open.dirty));
    }
}

#[test]
fn failed_session_annotation_preserves_documents_blocks_execution_and_retries_after_repair() {
    for ron in [false, true] {
        for configured in [false, true] {
            for failure in [
                "unrecorded name",
                "collision",
                "malformed reference",
                "output revision",
            ] {
                let mut fixture = pending_annotation();
                if !configured {
                    fixture.project.workspace.configuration_sets = Default::default();
                }
                let mut broken = fixture.clone();
                damage_pending_annotation(&mut broken, failure);
                let source = annotation_session(&broken.project);
                let mut restored = restore_annotation_session(&source, ron);
                for reference in [&fixture.root, &fixture.child] {
                    assert_eq!(
                        restored.workspace.schematic_buffers[&reference.key()].components,
                        source.workspace.schematic_buffers[&reference.key()].components
                    );
                    assert_eq!(
                        restored.workspace.schematic_buffers[&reference.key()].probes,
                        source.workspace.schematic_buffers[&reference.key()].probes
                    );
                }
                assert_eq!(
                    restored.workspace.configuration_sets,
                    source.workspace.configuration_sets
                );
                assert_eq!(
                    restored.workspace.simulation_plan_payloads,
                    source.workspace.simulation_plan_payloads
                );
                assert!(
                    serde_json::to_value(&restored.workspace)
                        .unwrap()
                        .get("annotation_restoration_error")
                        .is_none()
                );
                for blocked in [&restored, &restore_annotation_session(&restored, ron)] {
                    let error = blocked
                        .workspace
                        .design_projection(
                            &blocked.library_manager,
                            &fixture.child,
                            &blocked.schematic,
                        )
                        .unwrap_err();
                    assert!(
                        error.to_string().contains("reference annotation"),
                        "{failure}: {error}"
                    );
                    assert!(
                        blocked
                            .workspace
                            .design_projection_key(
                                &blocked.library_manager,
                                &fixture.child,
                                &blocked.schematic,
                            )
                            .is_none()
                    );
                    assert!(
                        blocked
                            .log_buffer
                            .entries()
                            .any(|entry| entry.message.contains("reference annotation"))
                    );
                }
                // Repair the invalid owner, but keep active and inactive edit
                // transactions pending: neither may be overwritten by recovery.
                restored.schematic.begin_operation("Repair reference owner");
                restored.schematic.components.clone_from(
                    &fixture.project.workspace.schematic_buffers[&fixture.child.key()].components,
                );
                restored
                    .workspace
                    .plan_data_mut(fixture.outputs[0].0)
                    .unwrap()
                    .saved_outputs[0]
                    .revision = fixture
                    .project
                    .workspace
                    .plan_data(fixture.outputs[0].0)
                    .unwrap()
                    .saved_outputs[0]
                    .revision;
                restored.sync_active_schematic_to_workspace();
                assert!(restored.workspace.annotation_restoration_error().is_some());
                restored.schematic.end_operation();
                restored
                    .workspace
                    .schematic_buffers
                    .get_mut(&fixture.root.key())
                    .unwrap()
                    .begin_operation("Other unfinished edit");
                restored.sync_active_schematic_to_workspace();
                assert!(restored.workspace.annotation_restoration_error().is_some());
                restored
                    .workspace
                    .schematic_buffers
                    .get_mut(&fixture.root.key())
                    .unwrap()
                    .end_operation();
                // Checking a repaired document uses the normal sync/retry path.
                // DRC findings are independent of the restored reference closure.
                let _ = restored.run_active_design_checks();
                assert!(
                    restored.workspace.annotation_restoration_error().is_none(),
                    "{failure}"
                );
                assert_restored_annotation(&restored, &fixture);
                assert_restored_annotation(&restore_annotation_session(&restored, ron), &fixture);
            }
        }
    }
}
