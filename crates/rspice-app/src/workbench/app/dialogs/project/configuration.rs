//! The project's configuration transactions: which configuration runs use,
//! and the edits the Configuration page applies to one.
//!
//! The Configuration page, its navigator and its inspector all commit through
//! here, so each change to the catalog is validated the one way: a
//! configuration that runs are about to use must project, keep its
//! connectivity valid, generate a netlist and seal its sources before the
//! catalog changes.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    ConfigurationCloneScope, ConfigurationSetCatalog, ConfigurationSetDefinition,
    ConfigurationSetError, ConfigurationSetId,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::project::configuration_state::{ConfigurationSelection, plain_refusal};

/// One change to the project's configuration catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum ConfigurationTransaction {
    /// Netlist with `selection` from now on.
    UseForRuns(ConfigurationSelection),
    /// Commit the Configuration page's edits to the configuration they were
    /// made to.
    Apply,
    /// Add `definition` to the project, and use it for runs or leave runs
    /// with what they use now.
    Create {
        definition: Box<ConfigurationSetDefinition>,
        use_for_runs: bool,
    },
    /// Copy every binding of `source` into a new configuration named `name`.
    Clone {
        source: ConfigurationSetId,
        name: String,
        use_for_runs: bool,
    },
    Rename {
        id: ConfigurationSetId,
        name: String,
    },
    Delete(ConfigurationSetId),
}

/// The catalog a transaction produces, the line the log records, and the
/// configuration the page shows afterwards.
struct Prepared {
    candidate: ConfigurationSetCatalog,
    message: String,
    select: Option<ConfigurationSelection>,
}

impl Prepared {
    fn new(candidate: ConfigurationSetCatalog, message: String) -> Self {
        Self {
            candidate,
            message,
            select: None,
        }
    }
}

/// Commit `transaction`, then drop every run preparation the old catalog
/// authorised. The error is the refusal, in words a reader can act on; the
/// catalog is untouched when there is one.
pub(in crate::workbench) fn commit_configuration_transaction(
    app: &mut RSpiceApp,
    transaction: ConfigurationTransaction,
) -> Result<(), String> {
    let applies_draft = transaction == ConfigurationTransaction::Apply;
    let renamed = match &transaction {
        ConfigurationTransaction::Rename { id, name } => Some((*id, name.clone())),
        _ => None,
    };
    let Some(prepared) = prepare(&app.state, transaction)? else {
        if applies_draft {
            app.state.project.configuration.revert();
        }
        return Ok(());
    };
    publish(&mut app.state, prepared.candidate)?;
    let page = &mut app.state.project.configuration;
    if applies_draft {
        page.revert();
    }
    if let Some((id, name)) = renamed
        && let Some(revision) = app
            .state
            .model
            .workspace
            .content
            .configuration_sets
            .find(id)
            .map(|configuration| configuration.revision())
    {
        page.renamed(id, &name, revision);
    }
    if let Some(selection) = prepared.select {
        page.select(selection);
    }
    let message = prepared.message;
    app.invalidate_simulation_preflight();
    app.state
        .record(LogSeverity::Info, LogSource::Studio, message);
    Ok(())
}

/// What `transaction` produces, or `None` when it changes nothing.
fn prepare(
    state: &AppState,
    transaction: ConfigurationTransaction,
) -> Result<Option<Prepared>, String> {
    let current: &ConfigurationSetCatalog = &state.model.workspace.content.configuration_sets;
    match transaction {
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Default) => {
            if current.active_configuration_id().is_none() {
                return Ok(None);
            }
            // The built-in order is what a new project runs with, so
            // returning to it is never refused for what the design holds.
            let mut candidate = current.clone();
            candidate
                .deactivate()
                .map_err(|error| format!("Cannot return to Default: {}", plain_refusal(&error)))?;
            Ok(Some(Prepared::new(
                candidate,
                "Runs now use the Default configuration.".to_owned(),
            )))
        }
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Configuration(id)) => {
            let selected = current
                .find(id)
                .ok_or_else(|| "That configuration was deleted. Pick another one.".to_owned())?;
            if current.active_configuration_id() == Some(id) {
                return Ok(None);
            }
            let name = selected.name().to_owned();
            let mut candidate = current.clone();
            candidate
                .activate(id)
                .map_err(|error| format!("Cannot use {name} for runs: {error}"))?;
            validate_candidate_configuration(state, &candidate)
                .map_err(|error| format!("Cannot use {name} for runs: {error}"))?;
            Ok(Some(Prepared::new(
                candidate,
                format!("Runs now use {name}."),
            )))
        }
        ConfigurationTransaction::Apply => {
            let Some(draft) = state.project.configuration.draft() else {
                return Ok(None);
            };
            let name = current.find(draft.id).map_or_else(
                || draft.definition.name.clone(),
                |committed| committed.name().to_owned(),
            );
            let mut candidate = current.clone();
            match candidate.update(draft.id, draft.base_revision, draft.definition.clone()) {
                Ok(_) => {}
                Err(ConfigurationSetError::NoChanges(_)) => return Ok(None),
                Err(ConfigurationSetError::NotFound(_)) => {
                    return Err(format!(
                        "Cannot apply: {name} was deleted. Revert to drop the edits."
                    ));
                }
                Err(error) => {
                    return Err(format!("Cannot apply {name}: {}", plain_refusal(&error)));
                }
            }
            // Runs read the configuration they use the moment it changes, so
            // only that one has to prove it still netlists.
            if current.active_configuration_id() == Some(draft.id) {
                validate_candidate_configuration(state, &candidate)
                    .map_err(|error| format!("Cannot apply {name}: {error}"))?;
            }
            Ok(Some(Prepared::new(
                candidate,
                format!("Applied the edits to {}.", draft.definition.name),
            )))
        }
        ConfigurationTransaction::Create {
            definition,
            use_for_runs,
        } => {
            let name = definition.name.clone();
            let mut candidate = current.clone();
            let id = candidate
                .create(*definition)
                .map_err(|error| format!("Cannot create {name}: {}", plain_refusal(&error)))?;
            added(
                state,
                current,
                candidate,
                id,
                &name,
                use_for_runs,
                "Created",
            )
        }
        ConfigurationTransaction::Clone {
            source,
            name,
            use_for_runs,
        } => {
            let original = current
                .find(source)
                .ok_or_else(|| "That configuration was deleted. Pick another one.".to_owned())?;
            let mut candidate = current.clone();
            let id = candidate
                .clone_configuration_scoped(
                    source,
                    original.revision(),
                    name.clone(),
                    ConfigurationCloneScope::AllBindings,
                    original.definition().clone(),
                )
                .map_err(|error| {
                    format!(
                        "Cannot clone {}: {}",
                        original.name(),
                        plain_refusal(&error)
                    )
                })?;
            added(state, current, candidate, id, &name, use_for_runs, "Cloned")
        }
        ConfigurationTransaction::Rename { id, name } => {
            let committed = current
                .find(id)
                .ok_or_else(|| "That configuration was deleted. Pick another one.".to_owned())?;
            let old = committed.name().to_owned();
            let mut definition = committed.definition().clone();
            definition.name = name.trim().to_owned();
            let mut candidate = current.clone();
            match candidate.update(id, committed.revision(), definition) {
                Ok(_) => {}
                Err(ConfigurationSetError::NoChanges(_)) => return Ok(None),
                Err(error) => {
                    return Err(format!("Cannot rename {old}: {}", plain_refusal(&error)));
                }
            }
            Ok(Some(Prepared::new(
                candidate,
                format!("Renamed {old} to {}.", name.trim()),
            )))
        }
        ConfigurationTransaction::Delete(id) => {
            let committed = current
                .find(id)
                .ok_or_else(|| "That configuration was deleted already.".to_owned())?;
            let name = committed.name().to_owned();
            if current.active_configuration_id() == Some(id) {
                return Err(delete_refusal(&name));
            }
            let mut candidate = current.clone();
            candidate
                .remove(id, committed.revision())
                .map_err(|error| format!("Cannot delete {name}: {}", plain_refusal(&error)))?;
            Ok(Some(Prepared::new(candidate, format!("Deleted {name}."))))
        }
    }
}

/// Why the configuration used for runs cannot be deleted, and what to do.
pub(in crate::workbench) fn delete_refusal(name: &str) -> String {
    format!(
        "Runs use {name}, so it cannot be deleted. Use another configuration, or Default, for runs first."
    )
}

/// Finish adding configuration `id` to `candidate`: runs use it when asked,
/// and otherwise keep what they used before it was added, whichever that was.
fn added(
    state: &AppState,
    current: &ConfigurationSetCatalog,
    mut candidate: ConfigurationSetCatalog,
    id: ConfigurationSetId,
    name: &str,
    use_for_runs: bool,
    verb: &str,
) -> Result<Option<Prepared>, String> {
    let message = if use_for_runs {
        candidate
            .activate(id)
            .map_err(|error| format!("Cannot use {name} for runs: {}", plain_refusal(&error)))?;
        validate_candidate_configuration(state, &candidate)
            .map_err(|error| format!("Cannot use {name} for runs: {error}"))?;
        format!("{verb} {name}; runs now use it.")
    } else {
        match current.active_configuration_id() {
            Some(previous) => candidate.activate(previous),
            None => candidate.deactivate(),
        }
        .map_err(|error| format!("Cannot add {name}: {}", plain_refusal(&error)))?;
        format!("{verb} {name}.")
    };
    Ok(Some(Prepared {
        candidate,
        message,
        select: Some(ConfigurationSelection::Configuration(id)),
    }))
}

/// Replace the catalog and the project revision as one transaction; the
/// generated netlist's input identity no longer describes the catalog.
fn publish(state: &mut AppState, candidate: ConfigurationSetCatalog) -> Result<(), String> {
    state
        .model
        .workspace
        .content
        .replace_configuration_sets(candidate)
        .map_err(|error| error.to_string())?;
    state
        .model
        .netlist_artifacts
        .current_generation_input_digest = None;
    Ok(())
}

/// Whether runs could execute with `catalog`: its configuration projects,
/// the project's connectivity holds, a netlist generates, and every source
/// it names seals. Each failure says which of the four failed.
pub(in crate::workbench) fn validate_candidate_configuration(
    state: &AppState,
    catalog: &ConfigurationSetCatalog,
) -> Result<(), String> {
    let mut workspace = state.model.workspace.clone();
    *workspace.content.configuration_sets = catalog.clone();
    let projection = workspace
        .configuration_execution_projection(
            &state.model.library_manager,
            &state.model.workspace.content.active_view,
            &state.model.schematic,
        )
        .map_err(|error| match error {
            // Each unbound instance's own sentence names its cell and the
            // views it was searched for; nothing needs to wrap them.
            rspice_design::projection::ConfigurationExecutionPlanError::Unresolved(unbound) => {
                format!("{unbound}.")
            }
            error => error.to_string(),
        })?;
    projection
        .connectivity()
        .validate()
        .map_err(|error| format!("the project connectivity is invalid: {error}"))?;
    let root = projection
        .root_schematic()
        .ok_or_else(|| "its top cell has no schematic open in the project.".to_owned())?;
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        state.model.library_manager.catalog(),
        &projection,
    );
    let generated = rspice_simulation::netlist_gen::generate_netlist_hierarchical(
        root,
        &[],
        &hierarchy,
        &rspice_simulation::netlist_gen::NetlistSourceData::new(
            &crate::simulation::table_route::SourceFiles,
        ),
    );
    if !generated.errors.is_empty() {
        return Err(format!(
            "the netlist does not generate: {}",
            generated.errors.join("; ")
        ));
    }
    let generated = workspace
        .content
        .bind_generated_netlist_provenance(generated.netlist);
    rspice_simulation::netlist_preparation::dependencies::expand_generated_dependencies(
        &generated,
        root.current_file(),
        &state.include_search_chain(),
        state.model.model_library_manager.catalog(),
        state.model.model_library_manager.resolution_records(),
    )
    .map_err(|error| format!("a source it includes cannot be read: {error}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ConfigurationSetId;

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
        let catalog =
            |app: &RSpiceApp| (*app.state.model.workspace.content.configuration_sets).clone();
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
        let refusal =
            commit_configuration_transaction(&mut app, ConfigurationTransaction::Delete(id))
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
}
