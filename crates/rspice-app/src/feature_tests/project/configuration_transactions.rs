//! The configuration transaction carried out on the whole application: apply,
//! refuse, delete, clone and rename against the runs the simulation
//! controller would start.

use crate::state::{ConfigurationSetDefinition, ConfigurationSetId};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::project::configuration::{
    ConfigurationTransaction, commit_configuration, delete_refusal,
};
use crate::workbench::project::configuration_state::ConfigurationSelection;

fn commit_configuration_transaction(
    app: &mut RSpiceApp,
    transaction: ConfigurationTransaction,
) -> Result<(), String> {
    commit_configuration(
        &mut app.state.project.configuration,
        &mut app.state.model,
        &mut app.state.runs,
        &mut app.simulation_controller,
        transaction,
    )
}

#[test]
fn using_a_deleted_configuration_for_runs_is_refused_and_changes_nothing() {
    let mut app = RSpiceApp::test_instance();
    let before_catalog = app.state.model.workspace.content.configuration_sets.clone();
    let before_revision = app.state.model.workspace.content.project.revision();

    let error = commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Configuration(
            ConfigurationSetId::new(),
        )),
    )
    .expect_err("a missing configuration is refused");

    assert!(error.contains("was deleted"), "{error}");
    assert_eq!(
        app.state.model.workspace.content.configuration_sets,
        before_catalog
    );
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        before_revision
    );
}

fn reference_app() -> (RSpiceApp, ConfigurationSetId) {
    let mut app = RSpiceApp::test_instance();
    let fixture = crate::workbench::examples::hierarchy_reference::build();
    app.state = fixture.state;
    (app, fixture.configuration)
}

fn edit_views(app: &mut RSpiceApp, id: ConfigurationSetId, views: &[&str], stops: &[&str]) {
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            definition.executable_view_policy =
                views.iter().map(|view| (*view).to_owned()).collect();
            definition.stop_views = stops.iter().map(|view| (*view).to_owned()).collect();
        });
}

#[test]
fn apply_commits_the_edits_against_the_revision_they_started_from() {
    let (mut app, id) = reference_app();
    let catalog = |app: &RSpiceApp| (*app.state.model.workspace.content.configuration_sets).clone();
    let revision = catalog(&app).find(id).unwrap().revision();
    edit_views(&mut app, id, &["schematic", "spice"], &[]);

    commit_configuration_transaction(&mut app, ConfigurationTransaction::Apply).unwrap();

    let committed = catalog(&app);
    let configuration = committed.find(id).unwrap();
    assert_eq!(configuration.revision(), revision + 1);
    assert_eq!(
        configuration.executable_view_policy(),
        ["schematic", "spice"]
    );
    assert!(app.state.project.configuration.draft().is_none());

    // Edits started before someone else's change are refused, not merged.
    edit_views(&mut app, id, &["schematic"], &[]);
    let mut elsewhere = committed.clone();
    let mut definition = configuration.definition().clone();
    definition.stop_views = vec!["spice".to_owned()];
    elsewhere.update(id, revision + 1, definition).unwrap();
    app.state
        .model
        .workspace
        .content
        .replace_configuration_sets(elsewhere.clone())
        .unwrap();
    let project_revision = app.state.model.workspace.content.project.revision();

    let refusal = commit_configuration_transaction(&mut app, ConfigurationTransaction::Apply)
        .expect_err("a stale draft is refused");
    assert!(
        refusal.contains("changed while you were editing"),
        "{refusal}"
    );
    assert_eq!(catalog(&app), elsewhere);
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        project_revision
    );
    assert!(
        app.state.project.configuration.draft().is_some(),
        "the edits survive"
    );
}

#[test]
fn a_refused_apply_leaves_the_project_untouched() {
    let (mut app, id) = reference_app();
    let before = app.state.model.workspace.content.configuration_sets.clone();
    let project_revision = app.state.model.workspace.content.project.revision();
    edit_views(&mut app, id, &["schematic"], &["extracted"]);

    let refusal = commit_configuration_transaction(&mut app, ConfigurationTransaction::Apply)
        .expect_err("a stop view outside the view list is refused");

    assert_eq!(
        refusal,
        concat!(
            "Cannot apply Reference hierarchy: Stop list: extracted is not in its view list. ",
            "Add it to the view list or remove the stop."
        )
    );
    assert_eq!(app.state.model.workspace.content.configuration_sets, before);
    assert_eq!(
        app.state.model.workspace.content.project.revision(),
        project_revision
    );
    assert!(app.state.project.configuration.is_dirty(&before));
}

/// An apply that leaves an instance without a view names the cell and
/// the views it was searched for, in words, not the netlister's terms.
#[test]
fn an_apply_that_unbinds_an_instance_names_its_cell_and_views() {
    let (mut app, id) = reference_app();
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            crate::workbench::project::configuration_state::set_instance_view(
                definition,
                "/X1",
                Some("spice"),
            );
        });

    let refusal = commit_configuration_transaction(&mut app, ConfigurationTransaction::Apply)
        .expect_err("an instance with no view is refused");

    assert_eq!(
        refusal,
        "Cannot apply Reference hierarchy: user/amp has no spice view."
    );
    assert_eq!(
        app.state.model.workspace.content.configuration_sets,
        catalog
    );
}

#[test]
fn default_returns_runs_to_the_built_in_order_and_back() {
    let (mut app, id) = reference_app();
    let revision = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .find(id)
        .unwrap()
        .revision();
    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Default),
    )
    .unwrap();
    let catalog = &app.state.model.workspace.content.configuration_sets;
    assert_eq!(catalog.active_configuration_id(), None);
    assert_eq!(catalog.find(id).unwrap().revision(), revision);
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .simulation_root_reference(),
        crate::state::CellViewRef::new(
            &app.state.model.workspace.content.project.root_library,
            &app.state.model.workspace.content.project.top_cell,
            "schematic",
        )
    );

    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Configuration(id)),
    )
    .unwrap();
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .configuration_sets
            .active_configuration_id(),
        Some(id)
    );
}

fn definition_from(
    app: &RSpiceApp,
    id: ConfigurationSetId,
    name: &str,
) -> ConfigurationSetDefinition {
    let mut definition = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .find(id)
        .unwrap()
        .definition()
        .clone();
    definition.name = name.to_owned();
    definition.overrides.clear();
    definition
}

fn active(app: &RSpiceApp) -> Option<ConfigurationSetId> {
    app.state
        .model
        .workspace
        .content
        .configuration_sets
        .active_configuration_id()
}

#[test]
fn the_configuration_used_for_runs_cannot_be_deleted() {
    let (mut app, id) = reference_app();
    let before = app.state.model.workspace.content.configuration_sets.clone();
    let refusal = commit_configuration_transaction(&mut app, ConfigurationTransaction::Delete(id))
        .expect_err("deleting what runs use is refused");
    assert_eq!(refusal, delete_refusal("Reference hierarchy"));
    assert!(refusal.contains("Use another configuration, or Default, for runs first"));
    assert_eq!(app.state.model.workspace.content.configuration_sets, before);

    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Default),
    )
    .unwrap();
    commit_configuration_transaction(&mut app, ConfigurationTransaction::Delete(id)).unwrap();
    assert!(
        app.state
            .model
            .workspace
            .content
            .configuration_sets
            .find(id)
            .is_none()
    );
}

#[test]
fn a_new_configuration_leaves_runs_alone_unless_asked() {
    let (mut app, id) = reference_app();
    let definition = definition_from(&app, id, "Characterization");
    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::Create {
            definition: Box::new(definition.clone()),
            use_for_runs: false,
        },
    )
    .unwrap();
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let created = catalog
        .configurations()
        .iter()
        .find(|configuration| configuration.name() == "Characterization")
        .unwrap()
        .id();
    assert_eq!(active(&app), Some(id));
    assert_eq!(
        app.state.project.configuration.selection(&catalog),
        ConfigurationSelection::Configuration(created),
        "the page shows what was created"
    );

    // From Default, a configuration added without being used for runs
    // leaves runs on Default.
    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Default),
    )
    .unwrap();
    let mut second = definition.clone();
    second.name = "Lab".to_owned();
    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::Create {
            definition: Box::new(second),
            use_for_runs: false,
        },
    )
    .unwrap();
    assert_eq!(active(&app), None);

    let mut third = definition;
    third.name = "Signoff".to_owned();
    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::Create {
            definition: Box::new(third),
            use_for_runs: true,
        },
    )
    .unwrap();
    let signoff = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .configurations()
        .iter()
        .find(|configuration| configuration.name() == "Signoff")
        .unwrap()
        .id();
    assert_eq!(active(&app), Some(signoff));
}

#[test]
fn a_clone_copies_every_binding_and_can_take_over_runs() {
    let (mut app, id) = reference_app();
    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::Clone {
            source: id,
            name: "Reference copy".to_owned(),
            use_for_runs: true,
        },
    )
    .unwrap();
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let copy = catalog
        .configurations()
        .iter()
        .find(|configuration| configuration.name() == "Reference copy")
        .unwrap();
    assert_eq!(copy.overrides(), catalog.find(id).unwrap().overrides());
    assert_eq!(active(&app), Some(copy.id()));
}

#[test]
fn renaming_keeps_unapplied_edits_applicable() {
    let (mut app, id) = reference_app();
    edit_views(&mut app, id, &["schematic", "spice"], &[]);
    commit_configuration_transaction(
        &mut app,
        ConfigurationTransaction::Rename {
            id,
            name: "Renamed".to_owned(),
        },
    )
    .unwrap();
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    assert_eq!(catalog.find(id).unwrap().name(), "Renamed");
    assert!(app.state.project.configuration.is_dirty(&catalog));
    commit_configuration_transaction(&mut app, ConfigurationTransaction::Apply).unwrap();
    let configuration = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .find(id)
        .unwrap()
        .clone();
    assert_eq!(configuration.name(), "Renamed");
    assert_eq!(
        configuration.executable_view_policy(),
        ["schematic", "spice"]
    );
}
