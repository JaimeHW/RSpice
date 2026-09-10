//! Single edits must preserve references in every occurrence of a reused master.

use super::*;

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

#[test]
fn editing_a_reused_master_renames_saved_outputs_and_inactive_probes() {
    let (mut fixture, root, child, other) = reused_master_fixture(["X1", "X2"]);
    fixture
        .state
        .activate_history_document(&child, "Edit reused master");
    assert_eq!(fixture.state.workspace.active_schematic_reference(), child);
    fixture
        .state
        .schematic
        .selection
        .components
        .insert(fixture.sources[0]);
    let root_selection = fixture.state.workspace.schematic_buffers[&root.key()].components[0].id;
    for reference in [&root, &other] {
        let source = fixture
            .state
            .workspace
            .schematic_buffers
            .get_mut(&reference.key())
            .unwrap();
        source.with_undo("temporary move", |source| source.components[0].pos.x += 10);
        assert!(source.undo());
        assert!(source.can_redo());
        source.selection.components.insert(source.components[0].id);
    }
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
    let documents = fixture
        .state
        .project_design_history
        .undo
        .last()
        .unwrap()
        .header
        .documents()
        .iter()
        .map(|document| document.reference().key())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        documents,
        BTreeSet::from([root.key(), child.key(), other.key()])
    );
    for (step, name) in ["V9", "V42", "V9"].into_iter().enumerate() {
        let state = &fixture.state;
        assert_eq!(
            state
                .workspace
                .plan_data(fixture.plan)
                .unwrap()
                .saved_outputs[0]
                .source_expression,
            format!("I(/X2/{name})")
        );
        for (document, parents) in [
            (&root, vec!["X1", "X2"]),
            (&child, vec!["X1"]),
            (&other, vec!["X5"]),
        ] {
            let schematic = schematic_for_reference(state, document).unwrap();
            assert!(!schematic.can_redo());
            for (probe, parent) in schematic.probes.iter().zip(parents) {
                assert_eq!(
                    probe.source_expression.as_deref(),
                    Some(format!("I(/{parent}/{name})").as_str())
                );
            }
            assert!(
                state
                    .workspace
                    .open_views
                    .iter()
                    .find(|open| open.reference == *document)
                    .unwrap()
                    .dirty
            );
        }
        assert!(
            state
                .schematic
                .selection
                .components
                .contains(&fixture.sources[0])
        );
        assert!(
            state.workspace.schematic_buffers[&root.key()]
                .selection
                .components
                .contains(&root_selection)
        );
        if step == 0 {
            fixture
                .state
                .activate_history_document(&root, "Inspect affected root");
            assert!(fixture.state.undo_project_design().unwrap().is_some());
            assert_eq!(fixture.state.workspace.active_schematic_reference(), child);
        }
        if step == 1 {
            assert!(fixture.state.redo_project_design().unwrap().is_some());
        }
    }
}

#[test]
fn inactive_document_authority_refuses_the_entire_reference_edit_and_history() {
    for history in [false, true] {
        for failure in ["read only", "pending gesture"] {
            let (mut fixture, root, child, other) = reused_master_fixture(["X1", "X2"]);
            fixture
                .state
                .activate_history_document(&child, "Edit reused master");
            let expected = fixture
                .state
                .schematic
                .components
                .iter()
                .find(|component| component.id == fixture.sources[0])
                .unwrap()
                .clone();
            if history {
                fixture
                    .state
                    .rename_component_transaction(&expected, "V9".to_owned())
                    .unwrap();
            }
            if failure == "read only" {
                fixture
                    .state
                    .workspace
                    .open_views
                    .iter_mut()
                    .find(|open| open.reference == other)
                    .unwrap()
                    .read_only_reference = true;
            } else {
                fixture
                    .state
                    .workspace
                    .schematic_buffers
                    .get_mut(&other.key())
                    .unwrap()
                    .begin_operation("Pending inactive gesture");
            }
            let before = SchematicSnapshot::capture(&fixture.state.schematic);
            let root_before =
                SchematicSnapshot::capture(&fixture.state.workspace.schematic_buffers[&root.key()]);
            let other_before = SchematicSnapshot::capture(
                &fixture.state.workspace.schematic_buffers[&other.key()],
            );
            let outputs = fixture
                .state
                .workspace
                .plan_data(fixture.plan)
                .unwrap()
                .saved_outputs
                .clone();
            let configurations = fixture.state.workspace.configuration_sets.clone();
            let sequence = fixture.state.project_undo_sequence();
            if history {
                assert!(fixture.state.undo_project_design().is_err(), "{failure}");
            } else {
                assert!(
                    fixture
                        .state
                        .rename_component_transaction(&expected, "V9".to_owned())
                        .is_err(),
                    "{failure}"
                );
            }
            assert!(before.is_equal_state(&fixture.state.schematic));
            assert!(
                root_before.is_equal_state(&fixture.state.workspace.schematic_buffers[&root.key()])
            );
            assert!(
                other_before
                    .is_equal_state(&fixture.state.workspace.schematic_buffers[&other.key()])
            );
            assert_eq!(
                fixture
                    .state
                    .workspace
                    .plan_data(fixture.plan)
                    .unwrap()
                    .saved_outputs,
                outputs
            );
            assert_eq!(fixture.state.workspace.configuration_sets, configurations);
            assert_eq!(fixture.state.project_undo_sequence(), sequence);
            assert_eq!(fixture.state.workspace.active_schematic_reference(), child);
            fixture
                .state
                .workspace
                .open_views
                .iter_mut()
                .find(|open| open.reference == other)
                .unwrap()
                .read_only_reference = false;
            fixture
                .state
                .workspace
                .schematic_buffers
                .get_mut(&other.key())
                .unwrap()
                .cancel_operation();
            if history {
                assert!(fixture.state.undo_project_design().unwrap().is_some());
            } else {
                assert!(
                    fixture
                        .state
                        .rename_component_transaction(&expected, "V9".to_owned())
                        .unwrap()
                );
            }
        }
    }
}

#[test]
fn renaming_a_parent_updates_descended_probes_and_their_occurrence() {
    for original in ["X1", "stage1"] {
        let (mut fixture, root, child, other) = reused_master_fixture([original, "X2"]);
        // This root shares the child master, but the edited parent placement
        // belongs to a different root, so its read-only state must not block.
        fixture
            .state
            .workspace
            .open_views
            .iter_mut()
            .find(|open| open.reference == other)
            .unwrap()
            .read_only_reference = true;
        let expected = fixture
            .state
            .schematic
            .components
            .iter()
            .find(|component| component.name == original)
            .unwrap()
            .clone();
        fixture
            .state
            .rename_component_transaction(&expected, "X9".to_owned())
            .unwrap();
        for (step, name) in ["X9", original, "X9"].into_iter().enumerate() {
            let state = &fixture.state;
            for reference in [&root, &child] {
                let source = schematic_for_reference(state, reference).unwrap();
                assert_eq!(
                    source.probes[0].source_expression.as_deref(),
                    Some(format!("I(/{name}/V42)").as_str())
                );
            }
            assert_eq!(
                state
                    .workspace
                    .configuration_sets
                    .find(fixture.configuration)
                    .unwrap()
                    .dut_path(),
                format!("/{name}")
            );
            assert_eq!(
                state
                    .workspace
                    .open_views
                    .iter()
                    .find(|open| open.reference == child)
                    .unwrap()
                    .occurrence
                    .steps[0]
                    .instance_name,
                name
            );
            assert_eq!(
                state.workspace.schematic_buffers[&other.key()].probes[0]
                    .source_expression
                    .as_deref(),
                Some("I(/X5/V42)")
            );
            assert_eq!(
                state
                    .workspace
                    .plan_data(fixture.plan)
                    .unwrap()
                    .saved_outputs[0]
                    .source_expression,
                "I(/X2/V42)"
            );
            if step == 0 {
                assert!(fixture.state.undo_project_design().unwrap().is_some());
            }
            if step == 1 {
                assert!(fixture.state.redo_project_design().unwrap().is_some());
            }
        }
    }
}

#[test]
fn reused_master_property_edits_and_bound_outputs_survive_native_reopen() {
    use crate::workbench::lifecycle::project_lifecycle::{self, DestinationAuthority, SaveScope};

    let (mut fixture, root, child, other) = reused_master_fixture(["X1", "X2"]);
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
    let first_output = fixture
        .state
        .workspace
        .plan_data(fixture.plan)
        .unwrap()
        .saved_outputs[0]
        .id;
    let second_plan = fixture.state.sim_setup.create_plan("Second plan").unwrap();
    let mut output = fixture
        .state
        .workspace
        .plan_data(fixture.plan)
        .unwrap()
        .saved_outputs[0]
        .clone();
    output.id = crate::product::SavedOutputId::new();
    output.name = "First occurrence".to_owned();
    output.source_expression = "I(/X1/V42)".to_owned();
    let second_output = output.id;
    fixture
        .state
        .workspace
        .add_saved_output(second_plan, output)
        .unwrap();
    fixture.state.sim_setup.activate_plan(fixture.plan).unwrap();
    fixture.state.schematic.probes[0].bind_saved_output(second_plan, second_output);
    fixture.state.schematic.probes[1].bind_saved_output(fixture.plan, first_output);
    fixture
        .state
        .workspace
        .schematic_buffers
        .get_mut(&child.key())
        .unwrap()
        .probes[0]
        .bind_saved_output(second_plan, second_output);
    // A buffered document can carry references even without an open tab.
    fixture
        .state
        .workspace
        .open_views
        .retain(|open| open.reference != other);
    fixture
        .state
        .activate_history_document(&child, "Edit reused master");
    let expected = fixture
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == fixture.sources[0])
        .unwrap()
        .clone();
    let mut candidate = expected.clone();
    candidate.name = "V9".to_owned();
    candidate.value = "2".to_owned();
    fixture
        .state
        .edit_component_transaction(&expected, candidate, "Edit source properties")
        .unwrap();
    let path = std::env::temp_dir().join(format!(
        "rspice-reused-rename-{}.rspiceproj",
        uuid::Uuid::new_v4()
    ));
    for (step, name) in ["V9", "V42", "V9"].into_iter().enumerate() {
        project_lifecycle::save_native(
            &mut fixture.state,
            SaveScope::AllDocuments,
            &path,
            if path.exists() {
                DestinationAuthority::Canonical
            } else {
                DestinationAuthority::UserSelected
            },
        )
        .unwrap();
        let loaded = crate::io::load_project_file(&path).unwrap();
        for (reference, parents) in [
            (&root, vec!["X1", "X2"]),
            (&child, vec!["X1"]),
            (&other, vec!["X5"]),
        ] {
            let source = &loaded.workspace.schematic_buffers[&reference.key()];
            for (probe, parent) in source.probes.iter().zip(parents) {
                assert_eq!(
                    probe.source_expression.as_deref(),
                    Some(format!("I(/{parent}/{name})").as_str())
                );
            }
        }
        let source = &loaded.workspace.schematic_buffers[&child.key()];
        let component = source
            .components
            .iter()
            .find(|component| component.id == fixture.sources[0])
            .unwrap();
        assert_eq!(component.name, name);
        assert_eq!(
            component.value,
            if step == 1 {
                expected.value.as_str()
            } else {
                "2"
            }
        );
        assert_eq!(
            source
                .components
                .iter()
                .find(|component| component.id == fixture.dependents[0])
                .unwrap()
                .params,
            format!("vref={name}")
        );
        for (plan, output, parent) in [
            (fixture.plan, first_output, "X2"),
            (second_plan, second_output, "X1"),
        ] {
            let saved = &loaded.workspace.plan_data(plan).unwrap().saved_outputs[0];
            assert_eq!(saved.id, output);
            assert_eq!(saved.source_expression, format!("I(/{parent}/{name})"));
        }
        assert_eq!(source.probes[0].saved_output_id, Some(second_output));
        assert_eq!(source.probes[0].plan_id, Some(second_plan));
        let projection = loaded
            .workspace
            .design_projection(&loaded.libraries, &child, source)
            .unwrap();
        assert_eq!(
            projection.schematic_buffers()[&child.key()]
                .components
                .iter()
                .find(|component| component.id == fixture.sources[0])
                .unwrap()
                .name,
            name
        );
        if step == 0 {
            assert!(fixture.state.undo_project_design().unwrap().is_some());
        }
        if step == 1 {
            assert!(fixture.state.redo_project_design().unwrap().is_some());
        }
    }
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.into_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
}
