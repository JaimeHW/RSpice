//! Native command history and persistence acceptance for component renames.
//! Assertions cross the schematic, configuration, plan, and durable probe owners.

use crate::product::{ObjectRevision, SimulationPlanId};
use crate::state::{
    Component, ComponentType, ConfigurationBlackBoxPolicy, ConfigurationModelProfile,
    ConfigurationSetDefinition, ConfigurationSetId, Point, SavedOutput, SavedOutputCompatibility,
    SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SchematicProbe,
    SchematicSnapshot, UnresolvedBindingPolicy,
};
use crate::workbench::app::RSpiceApp;

pub(crate) fn fixture() -> (RSpiceApp, Component, ConfigurationSetId, SimulationPlanId) {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .add_component(ComponentType::VoltageSource, Point::origin());
    let expected = app.state.model.schematic.document().components[0].clone();
    let source = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "current",
        "I(V1)",
        SavedOutputCompatibility::OpTranAc,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap();
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .unwrap()
        .id();
    app.state
        .model
        .workspace
        .content
        .add_saved_output(plan, source.clone())
        .unwrap();
    let mut probe =
        SchematicProbe::new(100, Point::new(80, 20), "I(V1)", Some("I(V1)".to_owned())).unwrap();
    probe.bind_saved_output(plan, source.id);
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .probes
        .push(probe);
    let root = app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference();
    let id = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .create(ConfigurationSetDefinition {
            name: "Rename test".to_owned(),
            root,
            dut_path: "/V1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            overrides: Vec::new(),
            owner: "test".to_owned(),
            unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            cell_bindings: Vec::new(),
            model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        })
        .unwrap();
    app.state.model.schematic.session.is_dirty = false;
    app.state.model.workspace.content.project_metadata_dirty = false;
    (app, expected, id, plan)
}

#[test]
fn preparation_failure_never_publishes_any_owner_or_history() {
    for refusal in [
        "configuration revision",
        "output revision",
        "read only",
        "stale target",
        "duplicate",
    ] {
        let (mut app, expected, configuration, plan) = fixture();
        match refusal {
            "configuration revision" => {
                let mut json =
                    serde_json::to_value(&app.state.model.workspace.content.configuration_sets)
                        .unwrap();
                json["configurations"][0]["revision"] = serde_json::json!(u64::MAX);
                app.state.model.workspace.content.configuration_sets =
                    serde_json::from_value(json).unwrap();
            }
            "output revision" => {
                app.state
                    .model
                    .workspace
                    .content
                    .plan_data_mut(plan)
                    .unwrap()
                    .saved_outputs[0]
                    .revision = ObjectRevision::new(u64::MAX).unwrap()
            }
            "read only" => app.state.model.schematic.session.read_only = true,
            "stale target" => {
                app.state.model.schematic.document_mut_for_test().components[0].value =
                    "8".to_owned()
            }
            "duplicate" => {
                let id = app
                    .state
                    .model
                    .schematic
                    .add_component(ComponentType::VoltageSource, Point::new(100, 0));
                app.state
                    .model
                    .schematic
                    .document_mut_for_test()
                    .components
                    .iter_mut()
                    .find(|component| component.id == id)
                    .unwrap()
                    .name = "V9".to_owned();
            }
            _ => unreachable!(),
        }
        let before = SchematicSnapshot::capture(app.state.model.schematic.document());
        let catalog = app.state.model.workspace.content.configuration_sets.clone();
        let payloads = app
            .state
            .model
            .workspace
            .content
            .simulation_plan_payloads
            .clone();
        let dirty = app.state.model.schematic.session.is_dirty;
        let epoch = app.state.model.design_execution_epoch;
        let mut candidate = expected.clone();
        candidate.name = "V9".to_owned();
        candidate.value = "12".to_owned();
        candidate.params = "ac=1".to_owned();
        assert!(
            app.state
                .model
                .edit_component_transaction(&expected, candidate, "edit properties")
                .is_err(),
            "{refusal}"
        );
        assert!(
            before.is_equal_document(app.state.model.schematic.document()),
            "{refusal}"
        );
        assert_eq!(
            catalog,
            app.state.model.workspace.content.configuration_sets
        );
        assert_eq!(
            payloads,
            app.state.model.workspace.content.simulation_plan_payloads
        );
        assert_eq!(dirty, app.state.model.schematic.session.is_dirty);
        assert!(!app.state.model.workspace.content.project_metadata_dirty);
        assert_eq!(epoch, app.state.model.design_execution_epoch);
        assert!(app.state.model.project_undo_sequence().is_none());
        assert_eq!(
            app.state
                .model
                .workspace
                .content
                .configuration_sets
                .find(configuration)
                .unwrap()
                .dut_path(),
            "/V1"
        );
    }
}

#[test]
fn an_unchanged_name_preserves_authored_reference_spelling_and_history() {
    let (mut app, expected, _, _) = fixture();
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Cccs, Point::new(100, 0));
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .unwrap()
        .params = "vref=v1".to_owned();
    let before = SchematicSnapshot::capture(app.state.model.schematic.document());
    let epoch = app.state.model.design_execution_epoch;
    assert!(
        !app.state
            .model
            .rename_component_transaction(&expected, expected.name.clone())
            .unwrap()
    );
    assert!(before.is_equal_document(app.state.model.schematic.document()));
    assert_eq!(app.state.model.design_execution_epoch, epoch);
    assert!(app.state.model.project_undo_sequence().is_none());
    assert!(!app.state.model.workspace.content.project_metadata_dirty);
}
