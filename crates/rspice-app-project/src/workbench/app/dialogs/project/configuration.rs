//! The project's configuration transactions: which configuration runs use,
//! and the edits the Configuration page applies to one.
//!
//! The Configuration page, its navigator and its inspector all commit through
//! here, so each change to the catalog is validated the one way: a
//! configuration that runs are about to use must project, keep its
//! connectivity valid, generate a netlist and seal its sources before the
//! catalog changes. The projection is taken on the frame; the rest runs off
//! it, and the change lands on the first frame after the check answers.

use std::sync::{Arc, Mutex, PoisonError};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{
    ConfigurationCloneScope, ConfigurationSetCatalog, ConfigurationSetDefinition,
    ConfigurationSetError, ConfigurationSetId,
};
use crate::workbench::app_state::Model;
use crate::workbench::project::configuration_state::{
    ConfigurationPageState, ConfigurationSelection, plain_refusal,
};
use rspice_app_runs::simulation::SimulationController;
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_simulation::netlist_preparation::dependencies;

/// One change to the project's configuration catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigurationTransaction {
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

/// What a committed transaction did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Committed {
    /// The catalog changed, or the transaction changed nothing.
    Done,
    /// It hands runs a configuration, which is being checked off the frame:
    /// it lands, or is refused, when the check answers.
    Checking,
}

/// The catalog a transaction produces, the line the log records and the
/// step Undo names it by, the configuration the page shows afterwards,
/// whether it carries the page's edits, which then stop being edits, and,
/// when runs are to use what it produces, the words a refused check starts
/// with.
#[derive(Debug, Clone)]
struct Prepared {
    candidate: ConfigurationSetCatalog,
    message: String,
    step: String,
    select: Option<ConfigurationSelection>,
    applies_draft: bool,
    renamed: Option<(ConfigurationSetId, String)>,
    check: Option<String>,
}

impl Prepared {
    fn new(candidate: ConfigurationSetCatalog, message: String) -> Self {
        Self {
            candidate,
            message,
            step: String::new(),
            select: None,
            applies_draft: false,
            renamed: None,
            check: None,
        }
    }
}

/// Commit `transaction` over what it reads and writes, the Configuration
/// page, the model and the run tier a committed change voids, then drop every
/// run preparation the old catalog authorised. The error is the refusal, in
/// words a reader can act on; the catalog is untouched when there is one.
///
/// A transaction that hands runs a configuration lands only once the check
/// that runs could execute with it answers, which
/// [`finish_configuration_check`] takes on a later frame.
pub fn commit_configuration(
    page: &mut ConfigurationPageState,
    model: &mut Model,
    runs: &mut RunsState,
    run_controller: &mut SimulationController,
    transaction: ConfigurationTransaction,
) -> Result<Committed, String> {
    if page.is_checking() {
        return Err(
            "A configuration is still being checked. Wait for the check to finish.".to_owned(),
        );
    }
    let applies_draft = transaction == ConfigurationTransaction::Apply;
    let renamed = match &transaction {
        ConfigurationTransaction::Rename { id, name } => Some((*id, name.clone())),
        _ => None,
    };
    let step = step_description(page, model, &transaction);
    let Some(mut prepared) = prepare(page, model, transaction)? else {
        if applies_draft {
            page.revert();
        }
        return Ok(Committed::Done);
    };
    prepared.applies_draft |= applies_draft;
    prepared.renamed = renamed;
    prepared.step = step;
    let Some(refused) = prepared.check.clone() else {
        land(page, model, runs, run_controller, prepared)?;
        return Ok(Committed::Done);
    };
    let input =
        check_input(model, &prepared.candidate).map_err(|error| format!("{refused}: {error}"))?;
    let base = (*model.workspace.content.configuration_sets).clone();
    let check = ConfigurationCheck::start(page, prepared, base, input);
    page.check = Some(check);
    Ok(Committed::Checking)
}

/// Land the transaction waiting on its check once the check has answered:
/// the answer, once, or `None` while the check runs or when nothing waits on
/// one. `ctx` is asked for a frame when the answer comes in.
pub fn finish_configuration_check(
    page: &mut ConfigurationPageState,
    model: &mut Model,
    runs: &mut RunsState,
    run_controller: &mut SimulationController,
    ctx: &egui::Context,
) -> Option<Result<(), String>> {
    let answer = page.check.as_ref()?.answer(ctx)?;
    let ConfigurationCheck { prepared, base, .. } = page.check.take()?;
    let refused = prepared.check.clone().unwrap_or_default();
    let current: &ConfigurationSetCatalog = &model.workspace.content.configuration_sets;
    if *current != base {
        return Some(Err(format!(
            "{refused}: the configurations changed while it was checked. Try again."
        )));
    }
    Some(
        answer
            .map_err(|error| format!("{refused}: {error}"))
            .and_then(|()| land(page, model, runs, run_controller, prepared)),
    )
}

/// Publish what `prepared` produces and carry out what it leaves the page to
/// do, then drop every run preparation the old catalog authorised.
fn land(
    page: &mut ConfigurationPageState,
    model: &mut Model,
    runs: &mut RunsState,
    run_controller: &mut SimulationController,
    prepared: Prepared,
) -> Result<(), String> {
    publish(model, prepared.candidate, prepared.step)?;
    if prepared.applies_draft {
        page.revert();
    }
    if let Some((id, name)) = prepared.renamed
        && let Some(revision) = model
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
    // The next two calls are what RSpiceApp::invalidate_simulation_preflight does.
    runs.preflight.invalidate();
    run_controller.clear_prepared_run();
    model.record(LogSeverity::Info, LogSource::Studio, prepared.message);
    Ok(())
}

/// A transaction that hands runs a configuration, waiting on the check that
/// runs could execute with it. The check runs off the frame, so the page
/// goes on painting while it does.
#[derive(Debug, Clone)]
pub struct ConfigurationCheck {
    prepared: Prepared,
    /// The catalog the transaction was made against. It lands on that one
    /// only, so a change made meanwhile is never overwritten.
    base: ConfigurationSetCatalog,
    slot: Arc<Mutex<CheckSlot>>,
}

/// Where the check leaves its answer, and the frame waiting on it.
#[derive(Debug, Default)]
struct CheckSlot {
    answer: Option<Result<(), String>>,
    waiting: Option<egui::Context>,
}

impl ConfigurationCheck {
    fn start(
        page: &ConfigurationPageState,
        prepared: Prepared,
        base: ConfigurationSetCatalog,
        input: CheckInput,
    ) -> Self {
        let slot = Arc::new(Mutex::new(CheckSlot::default()));
        let answered = Arc::clone(&slot);
        #[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
        let hold = page.check_hold.clone();
        #[cfg(not(all(any(test, feature = "test-support"), not(target_arch = "wasm32"))))]
        let _ = page;
        rspice_app_services::workbench::platform::spawn_or_inline(move || {
            #[cfg(all(any(test, feature = "test-support"), not(target_arch = "wasm32")))]
            if let Some(hold) = hold {
                hold.wait();
            }
            let answer = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| check(input)))
                .unwrap_or_else(|_| Err("the check stopped unexpectedly.".to_owned()));
            let waiting = {
                let mut slot = answered.lock().unwrap_or_else(PoisonError::into_inner);
                slot.answer = Some(answer);
                slot.waiting.take()
            };
            if let Some(ctx) = waiting {
                ctx.request_repaint();
            }
        });
        Self {
            prepared,
            base,
            slot,
        }
    }

    /// The answer, once, if it is in; otherwise `ctx` is asked for a frame
    /// when it is.
    fn answer(&self, ctx: &egui::Context) -> Option<Result<(), String>> {
        let mut slot = self.slot.lock().unwrap_or_else(PoisonError::into_inner);
        let answer = slot.answer.take();
        if answer.is_none() {
            slot.waiting = Some(ctx.clone());
        }
        answer
    }
}

/// What `transaction` produces, or `None` when it changes nothing.
fn prepare(
    page: &ConfigurationPageState,
    model: &Model,
    transaction: ConfigurationTransaction,
) -> Result<Option<Prepared>, String> {
    let current: &ConfigurationSetCatalog = &model.workspace.content.configuration_sets;
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
            // The page's edits to it go with it: the reader pressed Use for
            // runs over tables that show them, so runs use what was shown.
            let draft = page
                .draft()
                .filter(|draft| draft.id == id && page.is_dirty(current));
            if current.active_configuration_id() == Some(id) && draft.is_none() {
                return Ok(None);
            }
            let name = selected.name().to_owned();
            let mut candidate = current.clone();
            if let Some(draft) = draft {
                match candidate.update(id, draft.base_revision, draft.definition.clone()) {
                    Ok(_) | Err(ConfigurationSetError::NoChanges(_)) => {}
                    Err(error) => {
                        return Err(format!(
                            "Cannot use {name} for runs: {}",
                            plain_refusal(&error)
                        ));
                    }
                }
            }
            candidate
                .activate(id)
                .map_err(|error| format!("Cannot use {name} for runs: {error}"))?;
            let mut prepared = Prepared::new(
                candidate,
                if draft.is_some() {
                    format!("Applied the edits to {name}; runs now use it.")
                } else {
                    format!("Runs now use {name}.")
                },
            );
            prepared.applies_draft = draft.is_some();
            prepared.check = Some(format!("Cannot use {name} for runs"));
            Ok(Some(prepared))
        }
        ConfigurationTransaction::Apply => {
            let Some(draft) = page.draft() else {
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
            let mut prepared = Prepared::new(
                candidate,
                format!("Applied the edits to {}.", draft.definition.name),
            );
            // Runs read the configuration they use the moment it changes, so
            // only that one has to prove it still netlists.
            if current.active_configuration_id() == Some(draft.id) {
                prepared.check = Some(format!("Cannot apply {name}"));
            }
            Ok(Some(prepared))
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
            added(current, candidate, id, &name, use_for_runs, "Created")
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
            added(current, candidate, id, &name, use_for_runs, "Cloned")
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
pub fn delete_refusal(name: &str) -> String {
    format!(
        "Runs use {name}, so it cannot be deleted. Use another configuration, or Default, for runs first."
    )
}

/// Finish adding configuration `id` to `candidate`: runs use it when asked,
/// and otherwise keep what they used before it was added, whichever that was.
fn added(
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
        format!("{verb} {name}; runs now use it.")
    } else {
        match current.active_configuration_id() {
            Some(previous) => candidate.activate(previous),
            None => candidate.deactivate(),
        }
        .map_err(|error| format!("Cannot add {name}: {}", plain_refusal(&error)))?;
        format!("{verb} {name}.")
    };
    let mut prepared = Prepared::new(candidate, message);
    prepared.select = Some(ConfigurationSelection::Configuration(id));
    prepared.check = use_for_runs.then(|| format!("Cannot use {name} for runs"));
    Ok(Some(prepared))
}

/// Replace the catalog and the project revision as one transaction, one
/// step of Undo named by what it did; the generated netlist's input identity
/// no longer describes the catalog.
fn publish(
    model: &mut Model,
    candidate: ConfigurationSetCatalog,
    step: String,
) -> Result<(), String> {
    model.publish_configuration_sets(candidate, step)
}

/// What `transaction` does, as Undo and Redo name it.
fn step_description(
    page: &ConfigurationPageState,
    model: &Model,
    transaction: &ConfigurationTransaction,
) -> String {
    let catalog: &ConfigurationSetCatalog = &model.workspace.content.configuration_sets;
    let name = |id: ConfigurationSetId| {
        catalog
            .find(id)
            .map_or_else(String::new, |configuration| configuration.name().to_owned())
    };
    match transaction {
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Default) => {
            "use Default for runs".to_owned()
        }
        ConfigurationTransaction::UseForRuns(ConfigurationSelection::Configuration(id)) => {
            format!("use {} for runs", name(*id))
        }
        ConfigurationTransaction::Apply => format!(
            "apply edits to {}",
            page.draft()
                .map_or_else(String::new, |draft| name(draft.id))
        ),
        ConfigurationTransaction::Create { definition, .. } => {
            format!("create configuration {}", definition.name.trim())
        }
        ConfigurationTransaction::Clone {
            source, name: copy, ..
        } => {
            format!("clone {} as {}", name(*source), copy.trim())
        }
        ConfigurationTransaction::Rename { id, name: renamed } => {
            format!("rename {} to {}", name(*id), renamed.trim())
        }
        ConfigurationTransaction::Delete(id) => format!("delete configuration {}", name(*id)),
    }
}

/// What the check of a catalog reads, taken on the frame and owned, so the
/// rest of the check can run off it: the design projected with the catalog
/// in place of the project's own, the libraries it binds, where its includes
/// are searched for and, in the browser, the sources sealed for it.
struct CheckInput {
    projection: rspice_design::projection::ConfigurationExecutionProjection,
    libraries: rspice_design::library::LibraryCatalog,
    include_search: rspice_simulation::netlist_preparation::IncludeSearchChain,
    #[cfg(target_arch = "wasm32")]
    sealed: rspice_simulation::model_sources::SealedModelExecutionSources,
}

/// Project the design with `catalog` in place of the project's own. The
/// project is read where it stands; nothing of it is copied but the
/// libraries the netlist binds, and its own projection is not displaced.
fn check_input(model: &Model, catalog: &ConfigurationSetCatalog) -> Result<CheckInput, String> {
    let workspace = &model.workspace;
    let candidate = rspice_design::stamped::Stamped::new(catalog.clone());
    let projection = workspace
        .inspect_candidate_design_projection(
            &candidate,
            &model.library_manager,
            &workspace.content.active_view,
            &model.schematic,
            &workspace.candidate_projection_cache(),
        )
        .and_then(|projection| projection.into_execution())
        .map_err(|error| match error {
            // Each unbound instance's own sentence names its cell and the
            // views it was searched for; nothing needs to wrap them.
            rspice_design::projection::ConfigurationExecutionPlanError::Unresolved(unbound) => {
                format!("{unbound}.")
            }
            error => error.to_string(),
        })?;
    Ok(CheckInput {
        projection,
        libraries: model.library_manager.catalog().clone(),
        include_search: model.include_search_chain(),
        #[cfg(target_arch = "wasm32")]
        sealed: rspice_simulation::model_sources::seal_catalog_execution_sources(
            model.model_library_manager.catalog(),
            model.model_library_manager.resolution_records(),
        )
        .map_err(|error| format!("a source it includes cannot be read: {error}"))?,
    })
}

/// Whether runs could execute with the projected design: the project's
/// connectivity holds, a netlist generates, and every source it includes
/// reads. Each failure says which of the three failed.
fn check(input: CheckInput) -> Result<(), String> {
    let projection = &input.projection;
    projection
        .connectivity()
        .validate()
        .map_err(|error| format!("the project connectivity is invalid: {error}"))?;
    let root = projection
        .root_schematic()
        .ok_or_else(|| "its top cell has no schematic open in the project.".to_owned())?;
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        &input.libraries,
        projection,
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
    dependencies::expand_generated_dependencies_with_sealed_sources(
        &generated.netlist,
        root.current_file(),
        &input.include_search,
        {
            #[cfg(target_arch = "wasm32")]
            {
                Some(&input.sealed)
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                None
            }
        },
    )
    .map_err(|error| format!("a source it includes cannot be read: {error}"))?;
    Ok(())
}
