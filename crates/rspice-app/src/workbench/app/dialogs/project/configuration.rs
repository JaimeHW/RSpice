//! The project's configuration transactions: which configuration runs use,
//! and the edits the Configuration page applies to one.
//!
//! The Configuration page, its navigator and its inspector all commit through
//! here, so each change to the catalog is validated the one way: a
//! configuration that runs are about to use must project, keep its
//! connectivity valid, generate a netlist and seal its sources before the
//! catalog changes.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{ConfigurationSetCatalog, ConfigurationSetError};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::project::configuration_state::{
    ConfigurationSelection, built_in, plain_refusal,
};

/// One change to the project's configuration catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum ConfigurationTransaction {
    /// Netlist with `selection` from now on.
    UseForRuns(ConfigurationSelection),
    /// Commit the Configuration page's edits to the configuration they were
    /// made to.
    Apply,
}

/// Commit `transaction`, then drop every run preparation the old catalog
/// authorised. The error is the refusal, in words a reader can act on; the
/// catalog is untouched when there is one.
pub(in crate::workbench) fn commit_configuration_transaction(
    app: &mut RSpiceApp,
    transaction: ConfigurationTransaction,
) -> Result<(), String> {
    let applies_draft = transaction == ConfigurationTransaction::Apply;
    let Some((candidate, message)) = prepare(&app.state, transaction)? else {
        if applies_draft {
            app.state.project.configuration.revert();
        }
        return Ok(());
    };
    publish(&mut app.state, candidate)?;
    if applies_draft {
        app.state.project.configuration.revert();
    }
    app.invalidate_simulation_preflight();
    app.state
        .record(LogSeverity::Info, LogSource::Studio, message);
    Ok(())
}

/// The catalog `transaction` produces and the line the log records, or
/// `None` when it changes nothing.
fn prepare(
    state: &AppState,
    transaction: ConfigurationTransaction,
) -> Result<Option<(ConfigurationSetCatalog, String)>, String> {
    let current: &ConfigurationSetCatalog = &state.model.workspace.content.configuration_sets;
    match transaction {
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Default) => {
            if current.active_configuration_id().is_none() {
                return Ok(None);
            }
            // The built-in order is what a new project runs with, so
            // returning to it is never refused for what the design holds.
            let mut candidate = current.clone();
            built_in::deactivate(&mut candidate)
                .map_err(|error| format!("Cannot return to Default: {error}"))?;
            Ok(Some((
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
            Ok(Some((candidate, format!("Runs now use {name}."))))
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
            Ok(Some((
                candidate,
                format!("Applied the edits to {}.", draft.definition.name),
            )))
        }
    }
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
        .map_err(|error| error.to_string())?;
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
}
