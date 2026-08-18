//! Opening, renaming, archiving and restoring a plan.
//!
//! One of the five routes the plan-manager shell dispatches to. Two of its
//! modes render — renaming and confirming an archive — while opening and
//! restoring commit straight from the browse surface without a dialog of their
//! own.
//!
//! Both surfaces state only what the plan catalog itself enforces. The authored
//! reference names two facts this product has no owner for, and neither is
//! ported nor replaced by a zero. One is a deprecated-name alias policy: there
//! is no alias field on a plan anywhere in the catalog, so the control would
//! offer a choice with no effect. The other is a count of name-based script
//! references, which would report the result of an audit nothing in RSpice
//! performs — and a zero there would assert that the audit ran and found none.
//!
//! The four commits below differ in how much they are allowed to touch, and the
//! difference is deliberate. Opening and renaming invalidate the simulation
//! preflight, so they are handed the application. Archiving and restoring only
//! move a flag in the plan catalog, so they are handed the catalog and could not
//! reach the run controller if they tried.

use super::*;

use crate::workbench::design_system::property_row_input;

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    if draft.mode == SimulationPlanManagerMode::ConfirmArchive {
        return confirm_archive(ctx, draft, records);
    }
    rename(ctx, draft, records)
}

/// Exactly what [`SimSetupState::rename_plan`] writes, and nothing more.
///
/// It assigns the plan's name and returns. The stored analysis plan that carries
/// the identity and the revision is never reached, and a run receipt names a
/// plan by that identity rather than by its name.
///
/// [`SimSetupState::rename_plan`]: crate::workbench::app_state::SimSetupState::rename_plan
const RENAME_DESCRIPTION: &str = "Renaming writes the plan's display name and nothing else. Its stable identity, its revision, and every run receipt that references it are left exactly as they are.";

/// The two ways the catalog refuses a rename, which are the two `rename_plan`
/// returns before it assigns anything: a name another plan already holds under
/// the same case-folded key, and a plan whose analysis instances are executing.
///
/// One note, not two. They were a box each, and the two boxes made the same
/// claim — that the catalog decides, not this dialog — twice over a surface
/// whose whole content is three rows and two fields.
const RENAME_BOUNDARIES: &[(&str, &str)] = &[(
    "Refused by the catalog itself",
    "A name another plan already holds is rejected, compared without regard to \
     case, and a plan whose analysis instances are executing keeps its name \
     until they finish. Either refusal is reported here and nothing is written.",
)];

fn rename(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let selected = records
        .iter()
        .find(|record| record.id == draft.selected_plan_id);
    // A rename to the name the plan already has is not a rename. The catalog
    // would accept it — its uniqueness check excepts the plan being renamed —
    // and report success for a transaction that changed nothing.
    let renames = selected.is_some_and(|record| {
        let proposed = draft.name.trim();
        !proposed.is_empty() && proposed != record.name
    });
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · RENAME PLAN · IDENTITY PRESERVED",
        "Rename simulation plan",
        "Rename plan",
    )
    .description(RENAME_DESCRIPTION)
    .size(DialogSize::SimulationWorkflow)
    .ghost("Cancel")
    .primary_enabled(renames)
    .show(ctx, |ui| {
        if let Some(selected) = selected {
            rename_name_group(ui, &mut draft.name, selected);
            rename_preserved_group(ui, selected);
        }
        kit::note_grid(ui, RENAME_BOUNDARIES);
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ApplyRename),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// The name being replaced, over the name replacing it.
///
/// Both are property rows rather than a read-only field beside a labelled input,
/// so the two names land in one column and the edit reads as a substitution.
///
/// The head names the plan rather than the field. "Name" closed by the trailing
/// lifecycle word reads as "name available" on a surface whose whole subject is
/// whether a name is available, which is the one sentence this dialog must not
/// accidentally say.
fn rename_name_group(ui: &mut Ui, name: &mut String, selected: &PlanCatalogRecord) {
    kit::section_head(
        ui,
        "Plan to rename",
        Some(HeadStatus {
            label: selected.lifecycle_label(),
            tone: lifecycle_tone(selected),
        }),
    );
    property_row(ui, "Current name", &selected.name);
    property_row_input(ui, "New unique name", name, name.trim().is_empty());
}

/// What the rename leaves alone.
///
/// The revision is stated because it is the fact a reader is most likely to
/// doubt: renaming an object is exactly the kind of edit that bumps a revision
/// elsewhere, and here it does not.
fn rename_preserved_group(ui: &mut Ui, selected: &PlanCatalogRecord) {
    let t = Tokens::get(ui.ctx());
    kit::section_head(ui, "Preserved by this rename", None);
    property_row_status(
        ui,
        "Stable plan identity",
        &selected.id.to_string(),
        t.color.ok,
        StatusMark::Success,
    );
    property_row_status(
        ui,
        "Plan revision",
        &selected.revision.to_string(),
        t.color.ok,
        StatusMark::Success,
    );
    result_reference_row(ui, selected);
}

/// What archiving does, from the stored plan's own documented meaning: a
/// recoverable retirement that retains the plan's complete configuration and all
/// of its result references.
const ARCHIVE_DESCRIPTION: &str = "Archiving retires a plan from the working catalog and deletes nothing. The plan keeps its complete configuration and every result reference, and Restore returns it.";

/// The transaction's boundary, in one note.
///
/// The refusal is the stronger half and belongs in these words: `archive_plan`
/// rejects the active plan before it looks at anything else, so the guarantee
/// holds however the transaction is reached rather than depending on the browse
/// surface having disabled a button.
///
/// The reversibility half was a second box, and it repeated the row group above
/// it: that group counts what archiving retains, item by item, from the
/// catalog's own numbers. A box promising the same thing in prose adds a claim
/// and no fact.
const ARCHIVE_BOUNDARIES: &[(&str, &str)] = &[(
    "Refused by the catalog itself",
    "The plan catalog rejects archiving the active plan, or a plan whose \
     analysis instances are executing — a disabled button is not what enforces \
     it. Open another plan first. Restore reverses an archive.",
)];

fn confirm_archive(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let selected = records
        .iter()
        .find(|record| record.id == draft.selected_plan_id);
    let mut action = None;
    let choice = Dialog::new(
        "SIMULATION · ARCHIVE PLAN · RECOVERABLE RETIREMENT",
        "Archive simulation plan",
        "Archive plan",
    )
    .description(ARCHIVE_DESCRIPTION)
    .size(DialogSize::Transaction)
    .ghost("Cancel")
    // Two of the catalog's three refusals are knowable from the projection, so
    // the action is not offered for a plan it would certainly reject. The
    // catalog remains the authority: the third refusal has no field here, and
    // every one of them is reported through the validation message below.
    .primary_enabled(selected.is_some_and(|record| !record.active && !record.archived))
    .show(ctx, |ui| {
        if let Some(selected) = selected {
            archive_subject_group(ui, selected);
            archive_retained_group(ui, selected);
        }
        kit::note_grid(ui, ARCHIVE_BOUNDARIES);
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ConfirmArchive),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// Which plan is being retired, named and identified.
fn archive_subject_group(ui: &mut Ui, selected: &PlanCatalogRecord) {
    kit::section_head(
        ui,
        "Plan to archive",
        Some(HeadStatus {
            label: selected.lifecycle_label(),
            tone: lifecycle_tone(selected),
        }),
    );
    property_row(ui, "Name", &selected.name);
    property_row(ui, "Stable identity", &selected.id.to_string());
}

/// What survives the retirement, counted rather than promised.
///
/// `archive_plan` moves one boolean on the stored plan and reaches nothing else
/// on it, so every count here reads the same after the transaction as before it.
/// Stating them as the catalog's own numbers is what makes "complete
/// configuration" a checkable claim instead of reassurance.
///
/// The counts are grouped exactly as the browse aside groups them, and for the
/// same reason: the labels carry the nouns and the values carry only figures, so
/// no row has to pluralize a word to stay true.
fn archive_retained_group(ui: &mut Ui, selected: &PlanCatalogRecord) {
    let t = Tokens::get(ui.ctx());
    kit::section_head(ui, "Retained while archived", None);
    property_row_status(
        ui,
        "Analyses, model bindings",
        &format!("{} · {}", selected.analyses, selected.model_bindings),
        t.color.ok,
        StatusMark::Success,
    );
    property_row_status(
        ui,
        "Variables, outputs, specifications",
        &format!(
            "{} · {} · {}",
            selected.design_variables, selected.saved_outputs, selected.specifications
        ),
        t.color.ok,
        StatusMark::Success,
    );
    result_reference_row(ui, selected);
}

/// The runs that reference this plan.
///
/// One painter for both transactions, because both make the same promise about
/// the same number: a run's authenticated receipt names a plan by its stable
/// identity, so neither renaming nor archiving can detach a result from the plan
/// that produced it.
///
/// With no runs yet there is nothing to promise. A "0 references retained" row
/// would dress an empty set as a guarantee, so the empty case says what is
/// actually true instead.
fn result_reference_row(ui: &mut Ui, selected: &PlanCatalogRecord) {
    let t = Tokens::get(ui.ctx());
    match selected.results {
        0 => property_row(ui, "Result references", "no run references this plan yet"),
        count => property_row_status(
            ui,
            "Result references",
            &format!(
                "{count} run receipt{} · matched by identity",
                plan_plural_suffix(count)
            ),
            t.color.ok,
            StatusMark::Success,
        ),
    };
}

/// Open a plan: the setup and the plan-owned workspace payload move together,
/// on a copy installed only once the whole switch validates.
pub(super) fn commit_activate_plan(
    app: &mut RSpiceApp,
    id: SimulationPlanId,
) -> Result<String, String> {
    if app.state.sim_setup.stable_analysis_plan()?.id() == id {
        return Ok(format!(
            "'{}' remains the active editable plan.",
            app.state.sim_setup.active_plan_name()
        ));
    }
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    workspace.migrate_inactive_plan_data(id);
    setup.activate_plan(id).map_err(|error| error.to_string())?;
    workspace.sync_legacy_specs_projection(id);
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    let name = setup.active_plan_name().to_string();
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.invalidate_simulation_preflight();
    Ok(format!(
        "Opened '{name}' atomically with its complete plan-owned configuration."
    ))
}

/// Rename a plan, and report the two facts the dialog promised were preserved.
///
/// Both are read back off the committed catalog rather than echoed from the
/// draft, so the message states what the transaction left behind instead of what
/// it was asked for.
pub(super) fn commit_rename_plan(
    app: &mut RSpiceApp,
    id: SimulationPlanId,
    name: &str,
) -> Result<String, String> {
    let mut setup = app.state.sim_setup.clone();
    setup
        .rename_plan(id, name)
        .map_err(|error| error.to_string())?;
    setup
        .validate_plan_catalog()
        .map_err(|error| error.to_string())?;
    let message = simulation_plan_catalog_entry(&setup, id).map_or_else(
        || format!("Renamed simulation plan {id}; its identity and revision were preserved."),
        |(name, revision, _, _)| {
            format!(
                "Renamed to '{name}'. Identity {id} and revision {} are unchanged, and every run receipt still resolves to this plan.",
                revision.get()
            )
        },
    );
    app.state.sim_setup = setup;
    app.invalidate_simulation_preflight();
    Ok(message)
}

/// Archiving and restoring reach no further than the plan catalog.
///
/// They take the catalog rather than the whole application, unlike the two
/// commits above: neither invalidates the preflight nor touches the run
/// controller, so neither has any business being handed a value it could mutate
/// those through.
///
/// The refusal on the active plan is the catalog's, taken before it looks for
/// the plan at all. Nothing here re-checks it, because a second copy of the rule
/// is a second thing that can be wrong.
pub(super) fn commit_archive_plan(
    setup: &mut crate::workbench::app_state::SimSetupState,
    id: SimulationPlanId,
) -> Result<String, String> {
    let mut candidate = setup.clone();
    candidate
        .archive_plan(id)
        .map_err(|error| error.to_string())?;
    let name = catalog_entry_name(&candidate, id);
    *setup = candidate;
    Ok(format!(
        "Archived '{name}'. Its complete configuration and every result reference are retained, and Restore returns it to the working catalog."
    ))
}

pub(super) fn commit_restore_plan(
    setup: &mut crate::workbench::app_state::SimSetupState,
    id: SimulationPlanId,
) -> Result<String, String> {
    let mut candidate = setup.clone();
    candidate
        .restore_plan(id)
        .map_err(|error| error.to_string())?;
    let name = catalog_entry_name(&candidate, id);
    *setup = candidate;
    Ok(format!(
        "Restored '{name}' to the working catalog with its configuration and result references intact."
    ))
}

/// The catalog's own name for one plan, falling back to its identity.
///
/// Read from the committed catalog, so an archive or a restore names what it
/// actually moved. The fallback is unreachable after a transaction the catalog
/// accepted — it found the plan to move it — and is a formatted identity rather
/// than a guess at a name.
fn catalog_entry_name(
    setup: &crate::workbench::app_state::SimSetupState,
    id: SimulationPlanId,
) -> String {
    simulation_plan_catalog_entry(setup, id).map_or_else(|| id.to_string(), |(name, ..)| name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::state::{
        AnalysisResultSourceDomain, PreparedRunReceipt, PreparedRunTaskReceipt,
        PreparedSourceCheckReceipt, SimulationRun,
    };
    use crate::workbench::app_state::sim_setup::plan_catalog::SimulationPlanCatalogError;

    /// One prepared run whose authenticated receipt names `id`.
    ///
    /// The receipt is what makes a result a reference to a plan: the catalog
    /// projection counts a run for a plan when the receipt's simulation plan
    /// identity matches, so a fixture without one would let every assertion
    /// below about result references pass on an empty set.
    fn run_referencing(id: SimulationPlanId) -> SimulationRun {
        let revision = ObjectRevision::INITIAL;
        let digest = |byte: u8| ContentDigest::from_bytes([byte; 32]);
        let receipt = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(id),
            revision,
            digest(1),
            digest(2),
            PreparedSourceCheckReceipt::SchematicDrc(digest(3)),
            vec![
                PreparedRunTaskReceipt::new(
                    AnalysisInstanceId::new(),
                    revision,
                    Vec::new(),
                    5,
                    digest(4),
                )
                .expect("a valid prepared task receipt"),
            ],
        )
        .expect("a valid prepared run receipt");
        SimulationRun::new_prepared(1, receipt)
    }

    /// An active plan, one retained inactive plan, and a run referencing the
    /// retained one. Returns the active and the retained identities.
    ///
    /// `create_plan` activates what it creates, so the fixture's original plan
    /// becomes the retained entry — which is the one the archive and restore
    /// transactions can act on.
    fn app_with_a_referenced_plan() -> (RSpiceApp, SimulationPlanId, SimulationPlanId) {
        let mut app = RSpiceApp::test_instance();
        let retained = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();
        let mut setup = app.state.sim_setup.clone();
        let active = setup
            .create_plan("Corner characterization")
            .expect("a fresh root plan is created");
        app.state.workspace.migrate_active_plan_data(retained);
        app.state.workspace.migrate_inactive_plan_data(active);
        app.state.sim_setup = setup;
        app.state.simulation.runs.push(run_referencing(retained));
        (app, active, retained)
    }

    fn record_for(app: &RSpiceApp, id: SimulationPlanId) -> PlanCatalogRecord {
        plan_catalog_records(app)
            .into_iter()
            .find(|record| record.id == id)
            .unwrap_or_else(|| panic!("plan {id} is projected"))
    }

    /// The rename dialog's whole claim: the name moves and nothing else does.
    #[test]
    fn a_rename_preserves_the_identity_the_revision_and_the_result_references() {
        let (mut app, _, retained) = app_with_a_referenced_plan();
        let before = record_for(&app, retained);
        assert_eq!(
            before.results, 1,
            "the fixture's run must reference the plan, or this proves nothing"
        );

        commit_rename_plan(&mut app, retained, "  Retained sweep · release  ")
            .expect("a retained plan renames");

        let after = record_for(&app, retained);
        assert_eq!(
            after.name, "Retained sweep · release",
            "the name is trimmed and written"
        );
        assert_eq!(after.id, before.id, "the rename moved the stable identity");
        assert_eq!(
            after.revision, before.revision,
            "the rename moved the revision"
        );
        assert_eq!(
            after.results, before.results,
            "a result stopped resolving to the renamed plan"
        );
        assert_eq!(
            app.state.simulation.runs[0]
                .prepared_receipt()
                .and_then(PreparedRunReceipt::simulation_plan_id),
            Some(retained),
            "the receipt names the plan by identity, so the rename cannot touch it"
        );
    }

    /// Archiving retains the plan's complete configuration and all of its result
    /// references, which is what the stored record's own documentation promises.
    #[test]
    fn archiving_retains_the_configuration_and_every_result_reference() {
        let (mut app, _, retained) = app_with_a_referenced_plan();
        let before = record_for(&app, retained);
        assert!(!before.archived);

        commit_archive_plan(&mut app.state.sim_setup, retained).expect("an inactive plan archives");

        let after = record_for(&app, retained);
        assert!(after.archived, "the plan is not archived");
        assert_eq!(after.name, before.name);
        assert_eq!(after.id, before.id);
        assert_eq!(after.revision, before.revision);
        assert_eq!(after.analyses, before.analyses);
        assert_eq!(after.enabled, before.enabled);
        assert_eq!(after.model_bindings, before.model_bindings);
        assert_eq!(after.reference_pvt, before.reference_pvt);
        assert_eq!(after.forecast, before.forecast);
        assert_eq!(after.design_variables, before.design_variables);
        assert_eq!(after.saved_outputs, before.saved_outputs);
        assert_eq!(after.specifications, before.specifications);
        assert_eq!(after.regression_baseline, before.regression_baseline);
        assert_eq!(after.lineage, before.lineage);
        assert_eq!(
            after.results, before.results,
            "archiving detached a result from the plan that produced it"
        );
    }

    /// The refusal the archive surface claims is stronger than a disabled
    /// button: it is the catalog's, and it is typed.
    #[test]
    fn archiving_the_active_plan_is_refused_by_the_catalog() {
        let (mut app, active, _) = app_with_a_referenced_plan();

        assert_eq!(
            app.state.sim_setup.clone().archive_plan(active),
            Err(SimulationPlanCatalogError::ActivePlanCannotBeArchived(
                active
            )),
            "the catalog does not refuse the active plan on its own account"
        );

        let error = commit_archive_plan(&mut app.state.sim_setup, active)
            .expect_err("archiving the active plan is refused");
        assert!(
            error.to_ascii_lowercase().contains("active"),
            "the refusal does not say the plan is active: {error}"
        );
        let record = record_for(&app, active);
        assert!(
            record.active && !record.archived,
            "the refused archive still moved the plan"
        );
    }

    /// Restore is the reverse of archive, down to every fact the projection
    /// carries.
    #[test]
    fn restoring_reverses_an_archive() {
        let (mut app, _, retained) = app_with_a_referenced_plan();
        let before = record_for(&app, retained);

        commit_archive_plan(&mut app.state.sim_setup, retained).expect("an inactive plan archives");
        commit_restore_plan(&mut app.state.sim_setup, retained).expect("an archived plan restores");

        let after = record_for(&app, retained);
        assert!(!after.archived, "the restore left the plan archived");
        assert_eq!(after.lifecycle_label(), before.lifecycle_label());
        assert_eq!(after.name, before.name);
        assert_eq!(after.id, before.id);
        assert_eq!(after.revision, before.revision);
        assert_eq!(after.analyses, before.analyses);
        assert_eq!(after.model_bindings, before.model_bindings);
        assert_eq!(after.results, before.results);
        assert_eq!(
            app.state.sim_setup.restore_plan(retained),
            Err(SimulationPlanCatalogError::PlanNotArchived(retained)),
            "a restored plan is no longer archived, so restoring it again is refused"
        );
    }
}
