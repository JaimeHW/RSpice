//! References added between history operations retain current scope and atomic publication.

use super::*;

fn add_output(
    state: &mut AppState,
    plan: SimulationPlanId,
    expression: &str,
) -> crate::product::SavedOutputId {
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "New reference",
        expression,
        SavedOutputCompatibility::OpTranAc,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap();
    let id = output.id;
    state.workspace.add_saved_output(plan, output).unwrap();
    id
}

fn rename_source(fixture: &mut Fixture, child: &CellViewRef) {
    fixture
        .state
        .activate_history_document(child, "Edit reused master");
    let expected = fixture
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == fixture.sources[0])
        .unwrap()
        .clone();
    fixture
        .state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
}

fn cross_history(state: &mut AppState, forward: bool) -> Result<Option<String>, String> {
    if forward {
        state.redo_project_design()
    } else {
        state.undo_project_design()
    }
}

fn sequence(state: &AppState, forward: bool) -> Option<u64> {
    if forward {
        state.project_redo_sequence()
    } else {
        state.project_undo_sequence()
    }
}

#[test]
fn bulk_annotation_revision_exhaustion_preserves_source_and_history() {
    let mut fixture = fixture(&["V42"]);
    let configuration = fixture
        .state
        .workspace
        .configuration_sets
        .find(fixture.configuration)
        .unwrap();
    let mut definition = configuration.definition().clone();
    definition.dut_path = "/".to_owned();
    fixture
        .state
        .workspace
        .configuration_sets
        .update(fixture.configuration, configuration.revision(), definition)
        .unwrap();
    let mut project = serde_json::to_value(&fixture.state.workspace.project).unwrap();
    project["revision"] = serde_json::json!(u64::MAX - 1);
    fixture.state.workspace.project = serde_json::from_value(project).unwrap();
    publish(&mut fixture);
    assert_eq!(
        fixture.state.workspace.project.revision(),
        ObjectRevision::new(u64::MAX).unwrap()
    );
    let snapshot = SchematicSnapshot::capture(&fixture.state.schematic);
    let catalog = fixture.state.workspace.design_management.clone();
    let configurations = fixture.state.workspace.configuration_sets.clone();
    let outputs = fixture
        .state
        .workspace
        .plan_data(fixture.plan)
        .unwrap()
        .saved_outputs
        .clone();
    let active = fixture.state.workspace.active_schematic_reference();
    let sequence = fixture.state.project_undo_sequence().unwrap();

    assert!(fixture.state.undo_project_design().is_err());
    assert_eq!(fixture.state.project_undo_sequence(), Some(sequence));
    assert_eq!(fixture.state.project_redo_sequence(), None);
    assert_eq!(fixture.state.workspace.active_schematic_reference(), active);
    assert!(snapshot.is_equal_state(&fixture.state.schematic));
    assert_eq!(fixture.state.workspace.design_management, catalog);
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
}

#[test]
fn new_probe_owners_refuse_atomically_before_history_navigation_and_can_retry() {
    for forward in [false, true] {
        for read_only in [false, true] {
            let (mut fixture, root, child, other) = reused_master_fixture(["X1", "X2"]);
            fixture
                .state
                .workspace
                .schematic_buffers
                .get_mut(&other.key())
                .unwrap()
                .probes
                .clear();
            rename_source(&mut fixture, &child);
            if forward {
                assert!(fixture.state.undo_project_design().unwrap().is_some());
            }
            let from = if forward { "V42" } else { "V9" };
            let to = if forward { "V9" } else { "V42" };
            let other_source = fixture
                .state
                .workspace
                .schematic_buffers
                .get_mut(&other.key())
                .unwrap();
            other_source.probes.push(
                SchematicProbe::new(
                    4000,
                    Point::new(23, 57),
                    format!("I(/X5/{from})"),
                    Some(format!("I(/X5/{from})")),
                )
                .unwrap(),
            );
            if read_only {
                fixture
                    .state
                    .workspace
                    .open_views
                    .iter_mut()
                    .find(|open| open.reference == other)
                    .unwrap()
                    .read_only_reference = true;
            } else {
                other_source.begin_operation("Pending new consumer gesture");
            }
            fixture
                .state
                .activate_history_document(&root, "Inspect root before history");
            let state = &mut fixture.state;
            let snapshots = [&root, &child, &other].map(|reference| {
                SchematicSnapshot::capture(schematic_for_reference(state, reference).unwrap())
            });
            let outputs = state
                .workspace
                .plan_data(fixture.plan)
                .unwrap()
                .saved_outputs
                .clone();
            let configurations = state.workspace.configuration_sets.clone();
            let before_sequence = sequence(state, forward);
            assert!(cross_history(state, forward).is_err());
            assert_eq!(state.workspace.active_schematic_reference(), root);
            assert_eq!(sequence(state, forward), before_sequence);
            for (reference, snapshot) in [&root, &child, &other].into_iter().zip(&snapshots) {
                assert!(
                    snapshot.is_equal_state(schematic_for_reference(state, reference).unwrap())
                );
            }
            assert_eq!(
                state
                    .workspace
                    .plan_data(fixture.plan)
                    .unwrap()
                    .saved_outputs,
                outputs
            );
            assert_eq!(state.workspace.configuration_sets, configurations);
            if read_only {
                state
                    .workspace
                    .open_views
                    .iter_mut()
                    .find(|open| open.reference == other)
                    .unwrap()
                    .read_only_reference = false;
            } else {
                state
                    .workspace
                    .schematic_buffers
                    .get_mut(&other.key())
                    .unwrap()
                    .cancel_operation();
            }
            assert!(cross_history(state, forward).unwrap().is_some());
            let retained = if forward {
                &state.project_design_history.undo
            } else {
                &state.project_design_history.redo
            };
            assert!(
                retained
                    .last()
                    .unwrap()
                    .header
                    .documents()
                    .iter()
                    .any(|document| document.reference() == &other)
            );
            let probe = &state.workspace.schematic_buffers[&other.key()].probes[0];
            assert_eq!(probe.id, 4000);
            assert_eq!(probe.position, Point::new(23, 57));
            assert_eq!(
                probe.source_expression.as_deref(),
                Some(format!("I(/X5/{to})").as_str())
            );
            assert_eq!(probe.reference, format!("I(/X5/{to})"));
            assert!(cross_history(state, !forward).unwrap().is_some());
            assert_eq!(
                state.workspace.schematic_buffers[&other.key()].probes[0]
                    .source_expression
                    .as_deref(),
                Some(format!("I(/X5/{from})").as_str())
            );
        }
    }
}

#[test]
fn new_output_revision_exhaustion_keeps_history_and_navigation_intact() {
    for forward in [false, true] {
        let (mut fixture, root, child, _) = reused_master_fixture(["X1", "X2"]);
        rename_source(&mut fixture, &child);
        if forward {
            assert!(fixture.state.undo_project_design().unwrap().is_some());
        }
        let from = if forward { "V42" } else { "V9" };
        let to = if forward { "V9" } else { "V42" };
        let state = &mut fixture.state;
        let id = add_output(state, fixture.plan, &format!("I(/X1/{from})"));
        let output = state
            .workspace
            .plan_data_mut(fixture.plan)
            .unwrap()
            .saved_outputs
            .iter_mut()
            .find(|output| output.id == id)
            .unwrap();
        let revision = output.revision;
        output.revision = ObjectRevision::new(u64::MAX).unwrap();
        state.activate_history_document(&root, "Inspect root before history");
        let outputs = state
            .workspace
            .plan_data(fixture.plan)
            .unwrap()
            .saved_outputs
            .clone();
        let snapshot = SchematicSnapshot::capture(&state.schematic);
        let before_sequence = sequence(state, forward);
        assert!(cross_history(state, forward).is_err());
        assert_eq!(state.workspace.active_schematic_reference(), root);
        assert_eq!(sequence(state, forward), before_sequence);
        assert!(snapshot.is_equal_state(&state.schematic));
        assert_eq!(
            state
                .workspace
                .plan_data(fixture.plan)
                .unwrap()
                .saved_outputs,
            outputs
        );
        state
            .workspace
            .plan_data_mut(fixture.plan)
            .unwrap()
            .saved_outputs
            .iter_mut()
            .find(|output| output.id == id)
            .unwrap()
            .revision = revision;
        assert!(cross_history(state, forward).unwrap().is_some());
        let output = state
            .workspace
            .plan_data(fixture.plan)
            .unwrap()
            .saved_outputs
            .iter()
            .find(|output| output.id == id)
            .unwrap();
        assert_eq!(output.source_expression, format!("I(/X1/{to})"));
        assert_eq!(output.revision, revision.next().unwrap());
    }
}

#[test]
fn bulk_annotation_history_includes_new_plan_outputs_in_both_directions() {
    let mut fixture = fixture(&["V42"]);
    let catalog = &mut fixture.state.workspace.configuration_sets;
    let configuration = catalog.find(fixture.configuration).unwrap();
    let mut definition = configuration.definition().clone();
    definition.dut_path = "/".to_owned();
    catalog
        .update(fixture.configuration, configuration.revision(), definition)
        .unwrap();
    publish(&mut fixture);
    let first = add_output(&mut fixture.state, fixture.plan, "I(V1)");
    assert!(fixture.state.undo_project_design().unwrap().is_some());
    let output = fixture
        .state
        .workspace
        .plan_data(fixture.plan)
        .unwrap()
        .saved_outputs
        .iter()
        .find(|output| output.id == first)
        .unwrap();
    assert_eq!(output.source_expression, "I(V42)");
    let second_plan = fixture.state.sim_setup.create_plan("Later plan").unwrap();
    let second = add_output(&mut fixture.state, second_plan, "I(V42)");
    fixture.state.sim_setup.activate_plan(fixture.plan).unwrap();
    assert!(fixture.state.redo_project_design().unwrap().is_some());
    for (plan, id) in [(fixture.plan, first), (second_plan, second)] {
        let output = fixture
            .state
            .workspace
            .plan_data(plan)
            .unwrap()
            .saved_outputs
            .iter()
            .find(|output| output.id == id)
            .unwrap();
        assert_eq!(output.source_expression, "I(V1)");
    }
}

#[test]
fn history_resolves_current_configuration_roots_before_rewriting_shared_output_text() {
    use crate::state::{LibraryCellInstance, View, ViewType};

    for bulk in [false, true] {
        let (mut fixture, root, child, other) = reused_master_fixture(["X1", "X2"]);
        let target_name = if bulk { "V1" } else { "V9" };
        let independent = CellViewRef::new(&root.library, "independent", "schematic");
        let state = &mut fixture.state;
        state
            .library_manager
            .get_library_mut(&root.library)
            .unwrap()
            .get_or_create_cell(&independent.cell)
            .add_view(View::new("schematic", ViewType::Schematic));
        let mut independent_source = SchematicState::default();
        let id = independent_source.add_component(ComponentType::VoltageSource, Point::origin());
        independent_source
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .unwrap()
            .name = target_name.to_owned();
        state
            .workspace
            .schematic_buffers
            .insert(independent.key(), independent_source);
        let mut other_source = SchematicState::default();
        let id = other_source.add_library_cell_component(
            Point::origin(),
            LibraryCellInstance::new(&root.library, &independent.cell, "schematic"),
        );
        other_source
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .unwrap()
            .name = "X2".to_owned();
        state
            .workspace
            .schematic_buffers
            .insert(other.key(), other_source);
        let mut definition = state
            .workspace
            .configuration_sets
            .find(fixture.configuration)
            .unwrap()
            .definition()
            .clone();
        definition.root = other.clone();
        definition.name = "Independent testbench".to_owned();
        definition.dut_path = "/".to_owned();
        let other_configuration = state
            .workspace
            .configuration_sets
            .create(definition)
            .unwrap();
        state
            .workspace
            .configuration_sets
            .activate(fixture.configuration)
            .unwrap();
        if bulk {
            fixture.candidate = annotation_candidate(
                state,
                vec![AnnotationObject {
                    object: SchematicObjectKey::new(&child.key(), fixture.sources[0]).unwrap(),
                    current_reference: "V42".to_owned(),
                    device_family: "V".to_owned(),
                    sheet_id: None,
                    hierarchy_path: "/X1".to_owned(),
                    position: AnnotationPosition { x: 0, y: 0 },
                    connectivity_order: None,
                    locked: false,
                    external: false,
                    imported: false,
                }],
            );
            publish(&mut fixture);
        } else {
            rename_source(&mut fixture, &child);
        }
        let state = &mut fixture.state;
        state
            .workspace
            .configuration_sets
            .activate(other_configuration)
            .unwrap();
        state.activate_history_document(&other, "Inspect independent testbench");
        let expected_output = state
            .workspace
            .plan_data(fixture.plan)
            .unwrap()
            .saved_outputs[0]
            .clone();
        assert_eq!(
            expected_output.source_expression,
            format!("I(/X2/{target_name})")
        );
        for (forward, child_name) in [(false, "V42"), (true, target_name)] {
            assert!(cross_history(state, forward).unwrap().is_some());
            assert_eq!(
                state.workspace.configuration_sets.active_configuration_id(),
                Some(other_configuration)
            );
            assert_eq!(
                state
                    .workspace
                    .plan_data(fixture.plan)
                    .unwrap()
                    .saved_outputs[0],
                expected_output
            );
            assert_eq!(
                state.workspace.schematic_buffers[&independent.key()].components[0].name,
                target_name
            );
            assert_eq!(
                schematic_for_reference(state, &child)
                    .unwrap()
                    .components
                    .iter()
                    .find(|component| component.id == fixture.sources[0])
                    .unwrap()
                    .name,
                child_name
            );
        }
    }
}

#[test]
fn new_saved_outputs_follow_both_directions_of_a_past_component_rename() {
    let (mut fixture, _, child, _) = reused_master_fixture(["X1", "X2"]);
    let state = &mut fixture.state;
    state.activate_history_document(&child, "Edit reused master");
    let expected = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == fixture.sources[0])
        .unwrap()
        .clone();
    state
        .rename_component_transaction(&expected, "V9".to_owned())
        .unwrap();
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "Added after rename",
        "I(/X1/V9)",
        SavedOutputCompatibility::OpTranAc,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap();
    let output_id = output.id;
    state
        .workspace
        .add_saved_output(fixture.plan, output)
        .unwrap();
    assert!(state.undo_project_design().unwrap().is_some());
    assert_eq!(
        state
            .workspace
            .plan_data(fixture.plan)
            .unwrap()
            .saved_outputs
            .iter()
            .find(|output| output.id == output_id)
            .unwrap()
            .source_expression,
        "I(/X1/V42)"
    );
    let second = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "Added after undo",
        "I(/X2/V42)",
        SavedOutputCompatibility::OpTranAc,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap();
    let second_id = second.id;
    state
        .workspace
        .add_saved_output(fixture.plan, second)
        .unwrap();
    assert!(state.redo_project_design().unwrap().is_some());
    let outputs = &state
        .workspace
        .plan_data(fixture.plan)
        .unwrap()
        .saved_outputs;
    for (id, path) in [(output_id, "I(/X1/V9)"), (second_id, "I(/X2/V9)")] {
        assert_eq!(
            outputs
                .iter()
                .find(|output| output.id == id)
                .unwrap()
                .source_expression,
            path
        );
    }
}
