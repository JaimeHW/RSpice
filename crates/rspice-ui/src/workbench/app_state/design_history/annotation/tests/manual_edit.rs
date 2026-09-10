//! Manual annotation authority across projection, compensation and native persistence.

use super::*;

fn executable_fixture() -> Fixture {
    let mut fixture = fixture(&["V42"]);
    let catalog = &mut fixture.state.workspace.configuration_sets;
    let configuration = catalog.find(fixture.configuration).unwrap();
    let mut definition = configuration.definition().clone();
    // The voltage source is a primitive; the executable DUT is the root.
    definition.dut_path = "/".to_owned();
    catalog
        .update(fixture.configuration, configuration.revision(), definition)
        .unwrap();
    publish(&mut fixture);
    fixture
}

fn rename(fixture: &mut Fixture, name: &str) -> Result<bool, String> {
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
        .rename_component_transaction(&expected, name.to_owned())
}

fn assert_projection(fixture: &Fixture, name: &str) {
    assert_component_references(fixture, &[name]);
    let state = &fixture.state;
    let projection = state
        .workspace
        .design_projection(
            &state.library_manager,
            &state.workspace.active_schematic_reference(),
            &state.schematic,
        )
        .unwrap();
    let projected = projection.root_schematic().unwrap();
    assert_eq!(
        projected
            .components
            .iter()
            .find(|c| c.id == fixture.sources[0])
            .unwrap()
            .name,
        name
    );
    assert_eq!(
        projected
            .components
            .iter()
            .find(|c| c.id == fixture.dependents[0])
            .unwrap()
            .params,
        format!("vref={name}")
    );
}

#[test]
fn manual_rename_after_annotation_survives_projection_and_history() {
    let mut fixture = executable_fixture();
    let retained = fixture
        .state
        .workspace
        .design_management
        .annotation()
        .journal()[0]
        .clone();
    // Returning deliberately to the pre-annotation name must also be durable.
    rename(&mut fixture, "V42").unwrap();
    for (step, name) in ["V42", "V1", "V42"].into_iter().enumerate() {
        assert_projection(&fixture, name);
        assert_eq!(
            fixture
                .state
                .workspace
                .design_management
                .annotation()
                .journal()[0],
            retained
        );
        if step == 0 {
            assert!(fixture.state.undo_project_design().unwrap().is_some());
        }
        if step == 1 {
            assert!(fixture.state.redo_project_design().unwrap().is_some());
        }
    }
}

#[test]
fn annotation_and_repeated_manual_edits_share_a_reversible_history_chain() {
    let mut fixture = executable_fixture();
    rename(&mut fixture, "V42").unwrap();
    rename(&mut fixture, "V8").unwrap();
    let mut revision = fixture.state.workspace.project.revision();
    for name in ["V42", "V1", "V42"] {
        assert!(
            fixture.state.undo_project_design().unwrap().is_some(),
            "undo {name}"
        );
        assert_projection(&fixture, name);
        assert!(fixture.state.workspace.project.revision() > revision);
        revision = fixture.state.workspace.project.revision();
    }
    for name in ["V1", "V42", "V8"] {
        assert!(
            fixture.state.redo_project_design().unwrap().is_some(),
            "redo {name}"
        );
        assert_projection(&fixture, name);
        assert!(fixture.state.workspace.project.revision() > revision);
        revision = fixture.state.workspace.project.revision();
    }
}

#[test]
fn manual_annotation_refusal_is_atomic_at_commit_and_history_boundaries() {
    for history in [false, true] {
        for failure in [
            "project revision",
            "catalog revision",
            "output revision",
            "annotation authority",
        ] {
            let mut fixture = executable_fixture();
            if history {
                rename(&mut fixture, "V42").unwrap();
            }
            let state = &mut fixture.state;
            match failure {
                "project revision" => {
                    let mut wire = serde_json::to_value(&state.workspace.project).unwrap();
                    wire["revision"] = serde_json::json!(u64::MAX);
                    state.workspace.project = serde_json::from_value(wire).unwrap();
                }
                "catalog revision" => {
                    let mut wire =
                        serde_json::to_value(&state.workspace.design_management).unwrap();
                    wire["revision"] = serde_json::json!(u64::MAX);
                    state.workspace.design_management = serde_json::from_value(wire).unwrap();
                }
                "output revision" => {
                    state
                        .workspace
                        .plan_data_mut(fixture.plan)
                        .unwrap()
                        .saved_outputs[0]
                        .revision = ObjectRevision::new(u64::MAX).unwrap()
                }
                "annotation authority" => {
                    let object = SchematicObjectKey::new(
                        &state.workspace.active_schematic_reference().key(),
                        fixture.sources[0],
                    )
                    .unwrap();
                    state
                        .workspace
                        .design_management
                        .annotation_mut()
                        .commit_manual_reference_edit(
                            object,
                            if history { "V42" } else { "V1" },
                            "V7",
                        )
                        .unwrap();
                }
                _ => unreachable!(),
            }
            let before = SchematicSnapshot::capture(&state.schematic);
            let catalog = state.workspace.design_management.clone();
            let configurations = state.workspace.configuration_sets.clone();
            let outputs = state
                .workspace
                .plan_data(fixture.plan)
                .unwrap()
                .saved_outputs
                .clone();
            let revision = state.workspace.project.revision();
            let undo_len = state.project_design_history.undo.len();
            if history {
                assert!(!state.can_undo_project_design(), "{failure}");
                assert!(
                    !matches!(state.undo_project_design(), Ok(Some(_))),
                    "{failure}"
                );
            } else {
                assert!(rename(&mut fixture, "V42").is_err(), "{failure}");
            }
            let state = &fixture.state;
            assert!(before.is_equal_state(&state.schematic), "{failure}");
            assert_eq!(state.workspace.design_management, catalog);
            assert_eq!(state.workspace.configuration_sets, configurations);
            assert_eq!(
                state
                    .workspace
                    .plan_data(fixture.plan)
                    .unwrap()
                    .saved_outputs,
                outputs
            );
            assert_eq!(state.workspace.project.revision(), revision);
            assert_eq!(state.project_design_history.undo.len(), undo_len);
        }
    }
}

#[test]
fn manual_annotation_names_and_references_survive_native_save_and_reopen() {
    use crate::workbench::lifecycle::project_lifecycle::{self, DestinationAuthority, SaveScope};
    let mut fixture = executable_fixture();
    rename(&mut fixture, "V42").unwrap();
    let path = std::env::temp_dir().join(format!(
        "rspice-manual-annotation-{}.rspiceproj",
        uuid::Uuid::new_v4()
    ));
    for (step, name) in ["V42", "V1", "V42"].into_iter().enumerate() {
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
        let reference = loaded.workspace.active_schematic_reference();
        let source = &loaded.workspace.schematic_buffers[&reference.key()];
        let projection = loaded
            .workspace
            .design_projection(&loaded.libraries, &reference, source)
            .unwrap();
        for schematic in [source, projection.root_schematic().unwrap()] {
            assert_eq!(
                schematic
                    .components
                    .iter()
                    .find(|c| c.id == fixture.sources[0])
                    .unwrap()
                    .name,
                name
            );
            assert_eq!(
                schematic
                    .components
                    .iter()
                    .find(|c| c.id == fixture.dependents[0])
                    .unwrap()
                    .params,
                format!("vref={name}")
            );
            assert_eq!(
                schematic.probes[0].source_expression.as_deref(),
                Some(format!("I({name})").as_str())
            );
        }
        let outputs = &loaded
            .workspace
            .plan_data(fixture.plan)
            .unwrap()
            .saved_outputs;
        assert_eq!(outputs[0].source_expression, format!("I({name})"));
        assert_eq!(source.probes[0].saved_output_id, Some(outputs[0].id));
        assert_eq!(
            loaded.workspace.design_management.annotation(),
            fixture.state.workspace.design_management.annotation()
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

#[test]
fn unrelated_descriptor_changes_cannot_reanchor_annotation_history() {
    let mut fixture = executable_fixture();
    rename(&mut fixture, "V42").unwrap();
    let mut published = fixture.state.workspace.project.clone();
    published.set_path(std::path::PathBuf::from("first-save.rspiceproj"));
    published
        .rename("Unrelated project edit".to_owned())
        .unwrap();
    fixture
        .state
        .retain_annotation_history_after_save_descriptor(&published);
    fixture.state.workspace.project = published;
    assert!(!fixture.state.can_undo_project_design());
    assert!(fixture.state.undo_project_design().unwrap().is_none());
    assert_projection(&fixture, "V42");
}
