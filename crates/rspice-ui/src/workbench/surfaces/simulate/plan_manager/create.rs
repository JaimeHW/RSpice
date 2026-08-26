//! Creating a fresh root plan.
//!
//! One of the five routes the plan-manager shell dispatches to. The rendering
//! here cannot reach the application: it takes the draft and the catalog
//! projection and reports an action, and [`commit_create_plan`] is the only
//! thing in this file handed the whole application. That split is the point —
//! a route's render path has no way to mutate the run controller or the
//! workspace, so whole-application access lives in one named function per
//! route instead of being spread across five render paths.
//!
//! # Why there is no starting-point selector
//!
//! The authored dialog opens with one: organization template, blank plan, clone
//! active plan setup, import resolved manifest. None of the four survives
//! contact with this product.
//!
//! *Organization template* has no owner at all. RSpice has no organization
//! policy engine and no org-managed template library, so the option would name a
//! source that does not exist and the "selected template digest" the authored
//! table cites would be a digest of nothing.
//!
//! *Clone active plan setup* and *import resolved manifest* each already have
//! their own door, their own draft and their own transaction: `Clone…` on the
//! selected row opens `ClonePlanDraft`, and `Import…` sits in the toolbar beside
//! `New plan…`. A selector here would be a third route onto two transactions
//! that are already reachable, which is the clutter this product's bar rules
//! out — and routing through `commit_clone_plan` would cost expressiveness
//! rather than add it: its `copy_analyses_options` drives `copy_analyses` and
//! `copy_advanced_options` from one boolean, so "take the solver options but not
//! the analyses" — which the three flags below state directly — cannot be said
//! through the clone draft at all.
//!
//! *Blank plan* is what is left, and a selector with one option is not a
//! control. It is also no longer what this route does: a plan created here
//! carries a chosen reference point and up to three inherited configuration
//! domains, so Create makes a *configured* root plan and the authored
//! blank-or-clone dichotomy does not describe either end of it.
//!
//! So Create owns one transaction, Clone and Import keep theirs, and
//! [`NewSimulationPlanDraft`] grows no field for a choice that has no fourth
//! destination.

use super::*;

use crate::product::ProcessCorner;
use crate::simulation::run_set::ReferencePoint;
use crate::workbench::app::concept_banner;
use crate::workbench::state::NewSimulationPlanDraft;

const CREATE_EYEBROW: &str = "SIMULATION · NEW PLAN · FRESH ROOT IDENTITY";
const CREATE_TITLE: &str = "New simulation plan";
const CREATE_PRIMARY: &str = "Create plan";

/// What this transaction is, stated once above the form.
///
/// The authored banner also excludes approvals and release evidence. This
/// product has neither, and an exclusion is only a fact about something the
/// product could otherwise have copied — so the sentence names the one thing
/// creation genuinely does not copy and stops there.
const CREATE_BANNER: &str = "Creation produces a new stable plan identity and an \
     initial revision. It does not copy result datasets.";

/// What the transaction does, in the order it does it.
///
/// Every clause is `SimSetupState::create_plan` followed by this route's own
/// commit: the current plan is pushed into the catalog exactly as it stands, a
/// fresh plan identity is minted and activated, and the configuration chosen
/// below is installed on it.
const CREATE_DESCRIPTION: &str = "The active plan is retained in the catalog exactly as it stands, and a new plan with its own stable identity is created and opened in its place.";

/// The corners offered, in the order [`ProcessCorner`] declares them.
///
/// All five, not the three `ProcessCorner::speed_corners` returns: the skewed
/// corners are as much a reference point as the speed ones, and a plan
/// characterised at SF cannot say so through a control that omits it.
const PLAN_PROCESS_CORNERS: [ProcessCorner; 5] = [
    ProcessCorner::TT,
    ProcessCorner::SS,
    ProcessCorner::FF,
    ProcessCorner::SF,
    ProcessCorner::FS,
];

/// The coldest reference temperature the catalog accepts.
///
/// `SimSetupState::set_reference_pvt` refuses anything at or below absolute
/// zero, so the control stops just above it. The bound is on the control rather
/// than only in the refusal because a spinner that can be dragged to a value the
/// transaction is certain to reject states a range the product does not have.
const COLDEST_REFERENCE_CELSIUS: f64 = -273.14;

/// Width of the corner select and of the temperature field beside it.
///
/// Both are fixed and narrow. Their longest values are known — a two-letter
/// corner name, and a temperature written to one decimal with its unit — so
/// giving either the whole value column would spend width that the setting row's
/// wrapped detail can use instead.
const CORNER_SELECT_WIDTH: f32 = 78.0;
const TEMPERATURE_FIELD_WIDTH: f32 = 108.0;

/// Below this the two typed fields stack instead of standing side by side.
///
/// A `workflow_setting_row` gives its control half of the track it is handed, so
/// in a two-column form each field's control has a quarter of the dialog. At 640
/// that is 160 points, which still holds a corner select beside a temperature;
/// under it the pair stacks and each field gets the full width back.
const FORM_TWO_COLUMN_WIDTH: f32 = 640.0;

/// Gap between the inputs and the statement of what they produce.
const GROUP_GAP: f32 = 6.0;

/// The name is unique across the catalog, which is the one thing the catalog
/// checks about it before anything else.
const NAME_DETAIL: &str = "Unique across the catalog, compared without regard to case. Results reference the identity below, never this name.";

/// What the reference point is for, from its two owners: the run set resolves an
/// undeclared axis to it, and `set_reference_pvt` keeps the solver's `TEMP`
/// option and the operating point's temperature equal to it.
const PVT_DETAIL: &str = "The corner and temperature an undeclared run-set axis resolves to. The temperature is also this plan's solver TEMP option.";

/// What the three flags do, and what declining one means.
///
/// The retired plan keeps everything: `create_plan` pushes it into the catalog
/// with its own closure, options and policy before it installs the new plan, so
/// inheriting is a copy and never a move.
const INHERIT_DETAIL: &str = "Copied from the plan being retired, which keeps its own either way. Whatever is left unticked opens at this product's default.";

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let model_bindings = records
        .iter()
        .find(|record| record.active)
        .map(|record| record.model_bindings);
    let mut action = None;
    let choice = Dialog::new(CREATE_EYEBROW, CREATE_TITLE, CREATE_PRIMARY)
        .description(CREATE_DESCRIPTION)
        .size(DialogSize::SimulationWorkflow)
        .ghost("Cancel")
        // A name the catalog already holds is refused by the catalog, and the
        // refusal is reported below. It is deliberately not re-decided here: the
        // comparison `ensure_plan_name_available` makes is its own, and a second
        // spelling of it in this file is a second thing that can disagree with
        // the transaction it is supposed to predict.
        .primary_enabled(!draft.name.trim().is_empty())
        .show(ctx, |ui| {
            concept_banner(ui, CREATE_BANNER, false);
            new_plan_group(ui, draft, model_bindings);
            ui.add_space(GROUP_GAP);
            created_plan_statement(ui, &draft.new_plan);
            workflow_validation_message(ui, draft.validation_error.as_deref());
        });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ApplyCreate),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// Everything the reader decides: the plan's name, its reference point, and
/// which of three configuration domains it opens with.
///
/// Setting rows rather than property rows, because each of the three is a
/// control whose consequence needs a sentence and the property-row family has
/// nowhere to put one — its label column is a name, not an explanation. The
/// statement below is property rows for the opposite reason: nothing there is
/// decided, so nothing there needs explaining.
/// The two typed fields stand side by side, as the authored form grid does, and
/// the three choices run across the row under them. Stacked they were three
/// full-width rows for two short values and three checkboxes, which is the
/// height the outcome table below now has.
///
/// Below [`FORM_TWO_COLUMN_WIDTH`] the pair stacks: a `workflow_setting_row`
/// hands its control the right-hand half of whatever track it is given, and half
/// of half a narrow dialog is not a field.
fn new_plan_group(
    ui: &mut Ui,
    draft: &mut SimulationPlanManagerDraft,
    model_bindings: Option<usize>,
) {
    kit::section_head(ui, "New plan", None);
    let name = |ui: &mut Ui, name: &mut String| {
        workflow_setting_row(ui, "Plan name", NAME_DETAIL, |ui| {
            mono_input(ui, "Plan name", name, ui.available_width());
        });
    };
    let point = |ui: &mut Ui, point: &mut ReferencePoint| {
        workflow_setting_row(ui, "Reference PVT point", PVT_DETAIL, |ui| {
            reference_point_controls(ui, point);
        });
    };
    if ui.available_width() >= FORM_TWO_COLUMN_WIDTH {
        let (plan_name, reference_pvt) = (&mut draft.name, &mut draft.new_plan.reference_pvt);
        kit::equal_columns(ui, 2, |ui, index| {
            if index == 0 {
                name(ui, plan_name);
            } else {
                point(ui, reference_pvt);
            }
        });
    } else {
        name(ui, &mut draft.name);
        point(ui, &mut draft.new_plan.reference_pvt);
    }
    workflow_setting_row(ui, "Taken from the active plan", INHERIT_DETAIL, |ui| {
        inheritance_controls(ui, &mut draft.new_plan, model_bindings);
    });
}

/// The corner beside its temperature, which is how the workbench chrome's own
/// PVT control reads the same fact.
///
/// The temperature edits the stored `f64` directly rather than through a typed
/// draft string. That is not a shortcut: the two would have to be kept equal,
/// and the transaction reads the number — so a control that owns the number
/// cannot come to disagree with what is committed.
fn reference_point_controls(ui: &mut Ui, point: &mut ReferencePoint) {
    let options = PLAN_PROCESS_CORNERS
        .iter()
        .map(|corner| corner.short_name().to_owned())
        .collect::<Vec<_>>();
    ui.horizontal(|ui| {
        if let Some(index) = select(
            ui,
            "simulation.plan-manager.create.reference-corner",
            "Reference process corner",
            point.process.short_name(),
            &options,
            CORNER_SELECT_WIDTH,
        ) {
            point.process = PLAN_PROCESS_CORNERS[index];
        }
        let height = Tokens::get(ui.ctx()).metrics.ctl_h;
        let response = ui.add_sized(
            vec2(TEMPERATURE_FIELD_WIDTH, height),
            egui::DragValue::new(&mut point.temperature_celsius)
                .range(COLDEST_REFERENCE_CELSIUS..=f64::MAX)
                .speed(1.0)
                .fixed_decimals(1)
                .suffix(" °C"),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::DragValue,
                ui.is_enabled(),
                "Reference temperature in degrees Celsius",
            )
        });
    });
}

/// The three configuration domains a new plan can open with.
///
/// Only the closure carries a figure, because it is the only one of the three
/// the catalog projection measures. Printing a count beside the other two would
/// mean deriving one here, and a number this file invented would stand beside
/// two the projection owns.
fn inheritance_controls(
    ui: &mut Ui,
    new_plan: &mut NewSimulationPlanDraft,
    model_bindings: Option<usize>,
) {
    ui.vertical(|ui| {
        workflow_switch(
            ui,
            &model_closure_label(model_bindings),
            &mut new_plan.inherit_model_closure,
        );
        workflow_switch(ui, "Solver options", &mut new_plan.inherit_solver_options);
        workflow_switch(
            ui,
            "Result retention policy",
            &mut new_plan.inherit_save_policy,
        );
    });
}

/// The closure offered, sized from the active plan's own record.
///
/// With no active record projected there is no size to state, so the label names
/// the domain and stops there rather than printing a zero that would read as a
/// measured empty closure.
fn model_closure_label(model_bindings: Option<usize>) -> String {
    model_bindings.map_or_else(
        || "Ordered model closure".to_owned(),
        |count| {
            format!(
                "Ordered model closure · {count} binding{}",
                plural_suffix(count)
            )
        },
    )
}

/// The one object this transaction creates, in the authored four columns:
/// what is created, what it came from, what travelled with it, and what did not.
///
/// It replaces two property groups that answered the same question in two
/// headings. The authored table is one row because the transaction produces one
/// object, and stating it as a row lets the copied and excluded halves stand
/// beside each other where a reader compares them.
///
/// `Copied` is computed from the three checkboxes above, so the row is a
/// statement about the transaction as it is currently configured rather than a
/// fixed sentence beside controls that change it.
///
/// The authored table also excludes waivers, approvals and release evidence.
/// This product has no approval system, no waivers and no release evidence, and
/// an exclusion is only a fact about something the product could otherwise have
/// copied.
fn created_plan_statement(ui: &mut Ui, new_plan: &NewSimulationPlanDraft) {
    let cells = created_plan_cells(new_plan);
    let columns = created_plan_columns(ui, &cells);
    // Measured, not authored: the copied cell is built from the reader's own
    // choices, so its width is not knowable where a constant would be written —
    // and this route promises that nothing it paints is elided.
    if ui.available_width() >= kit::table_minimum_width(&columns) {
        let announced = cells
            .iter()
            .map(|(heading, value)| format!("{heading}: {value}"))
            .collect::<Vec<_>>()
            .join(" · ");
        kit::read_only_records_table(
            ui,
            "simulation.plan-manager.create.outcome",
            &columns,
            &[TableRow {
                selected: false,
                announced,
            }],
            |ui, _, column| {
                let t = Tokens::get(ui.ctx());
                kit::cell_prose(ui, &cells[column].1, t.color.text);
            },
        );
        return;
    }
    // Too narrow for four tracks. The same four facts as rows, where each value
    // has the whole width to itself instead of a quarter of it.
    kit::section_head(ui, "Created by this transaction", None);
    for (heading, value) in &cells {
        property_row(ui, heading, value);
    }
}

/// The four cells, each from the transaction that produces it.
///
/// `SimulationPlan::new` mints a fresh identity at `ObjectRevision::INITIAL` and
/// seeds one enabled transient instance — it is not an empty plan, which is the
/// cell most easily got wrong here. `create_plan` retires the active plan into
/// the catalog and installs a default lineage, so the source is that plan and the
/// new one is a root with no source of its own.
///
/// The excluded cell names the three things this transaction never carries
/// across whatever is ticked: a run's authenticated receipt names the plan it was
/// dispatched from and nothing here rewrites one, `migrate_inactive_plan_data`
/// seeds a default payload rather than copying, and `create_plan` installs
/// `RunSetState::reference_only`.
fn created_plan_cells(new_plan: &NewSimulationPlanDraft) -> [(&'static str, String); 4] {
    let copied = [
        (new_plan.inherit_model_closure, "model closure"),
        (new_plan.inherit_solver_options, "solver options"),
        (new_plan.inherit_save_policy, "retention policy"),
    ]
    .into_iter()
    .filter_map(|(taken, name)| taken.then_some(name))
    .collect::<Vec<_>>();
    [
        (
            "Created object",
            "plan identity · revision 1 · 1 transient".to_owned(),
        ),
        ("Source", "the retired active plan".to_owned()),
        (
            "Copied",
            if copied.is_empty() {
                "nothing · catalog defaults".to_owned()
            } else {
                copied.join(" · ")
            },
        ),
        (
            "Excluded",
            "results · plan-owned records · run-set axes".to_owned(),
        ),
    ]
}

/// One track per cell, each wide enough for its heading and its value.
///
/// The last is elastic so the solved widths fill the table exactly; the rest are
/// their content's width and no more, which is what keeps the four inside a
/// 760-point dialog.
fn created_plan_columns(ui: &Ui, cells: &[(&'static str, String); 4]) -> Vec<TableColumn> {
    cells
        .iter()
        .enumerate()
        .map(|(index, (heading, value))| {
            let width = kit::prose_column_width(ui, heading, value);
            TableColumn {
                heading,
                track: if index + 1 == cells.len() {
                    ColumnTrack::Elastic(width)
                } else {
                    ColumnTrack::Fixed(width)
                },
            }
        })
        .collect()
}

/// Create a plan and activate it, moving the setup and the workspace payload
/// together or not at all.
///
/// The four configuration domains the new plan owns come from `new_plan` rather
/// than staying at whatever [`SimSetupState::create_plan`] mints, because a plan
/// created at defaults is a plan whose corner, model closure, solver options and
/// retention all have to be found and changed afterwards.
///
/// [`SimSetupState::create_plan`]: crate::workbench::app_state::SimSetupState::create_plan
///
/// Inheritance reads the setup that is active *here*, at commit time. That is the
/// only point at which "the active plan" is a definite thing: the manager stays
/// open across frames, and the plan under it can be switched in between.
pub(super) fn commit_create_plan(
    app: &mut RSpiceApp,
    name: &str,
    new_plan: &NewSimulationPlanDraft,
) -> Result<(SimulationPlanId, String), String> {
    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    // Read before the transaction, because `create_plan` retires the active plan
    // into the catalog and installs an empty closure and default options in its
    // place. After it returns, the plan being inherited from is no longer active.
    let inherited = InheritedPlanConfiguration {
        model_bindings: setup.model_bindings.clone(),
        options: setup.options.clone(),
        save_policy: setup.save_policy,
    };
    let id = setup.create_plan(name).map_err(|error| error.to_string())?;
    apply_new_plan_configuration(&mut setup, new_plan, inherited)?;
    workspace.migrate_inactive_plan_data(id);
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
    Ok((
        id,
        format!("Created and activated fresh root plan '{name}' with identity {id}."),
    ))
}

/// What the plan that was active before the transaction owned, captured so the
/// three inheritance flags have something to inherit from.
///
/// It is a struct rather than three parameters because it is one snapshot taken
/// at one instant. Three separate arguments could be read at three different
/// points, and the point they are read at is the whole correctness question here:
/// `create_plan` retires the active plan and installs an empty closure, default
/// options and a default policy in its place.
struct InheritedPlanConfiguration {
    model_bindings: Vec<crate::state::model_library::SimulationPlanModelBinding>,
    options: crate::simulation::dialog::SimulationOptions,
    save_policy: crate::workbench::app_state::SimulationSavePolicy,
}

/// Install the created plan's own configuration on `setup`.
///
/// Ordering is load-bearing twice over. Inherited options are installed before
/// the reference point, because `set_reference_pvt` is the one owner of keeping
/// the solver's `TEMP` option, its editor draft and the operating-point
/// temperature equal to the plan's reference temperature — the other way round,
/// an inherited options block would overwrite all three with the temperature of
/// the plan it came from. And the options editor's draft is rebuilt from the
/// options actually installed, because `create_plan` built it from the defaults it
/// minted and an inherited block replaces those.
///
/// The catalog is revalidated at the end. `create_plan` validated the catalog it
/// committed, and every assignment here happens after it returned, so what this
/// installs is checked rather than trusted.
fn apply_new_plan_configuration(
    setup: &mut crate::workbench::app_state::SimSetupState,
    new_plan: &NewSimulationPlanDraft,
    inherited: InheritedPlanConfiguration,
) -> Result<(), String> {
    if new_plan.inherit_solver_options {
        setup.options = inherited.options;
        setup.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&setup.options);
    }
    if new_plan.inherit_model_closure {
        setup.model_bindings = inherited.model_bindings;
    }
    if new_plan.inherit_save_policy {
        setup.save_policy = inherited.save_policy;
    }
    setup.set_reference_pvt(
        new_plan.reference_pvt.process,
        new_plan.reference_pvt.temperature_celsius,
    )?;
    setup
        .validate_plan_catalog()
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::dialog::SimulationOptions;
    use crate::state::model_library::SimulationPlanModelBinding;
    use crate::state::{
        AnalysisResultSourceDomain, PreparedRunReceipt, PreparedRunTaskReceipt,
        PreparedSourceCheckReceipt, SimulationRun,
    };
    use crate::workbench::app_state::SimulationSavePolicy;

    /// A reference point no default produces, so a plan carrying it carries it
    /// because this route committed it.
    const CHOSEN_POINT: ReferencePoint = ReferencePoint {
        process: ProcessCorner::SF,
        temperature_celsius: -40.0,
    };

    /// An active plan whose four configuration domains are all distinguishable
    /// from the defaults `create_plan` mints, so inheriting one is observable.
    fn app_with_a_distinctive_active_plan() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let mut setup = app.state.sim_setup.clone();
        setup
            .set_reference_pvt(ProcessCorner::FF, 85.0)
            .expect("a physical reference corner");
        setup.options.reltol = 3.5e-6;
        setup.save_policy.retained_dataset_limit = 42;
        setup.model_bindings.push(SimulationPlanModelBinding {
            library_name: "foundry-models".to_owned(),
            source_digest: ContentDigest::from_bytes([3; 32]),
            selected_corner: Some("TT".to_owned()),
        });
        app.state.sim_setup = setup;
        assert_ne!(
            app.state.sim_setup.options.reltol,
            SimulationOptions::default().reltol,
            "the fixture's options must differ from the default, or inheriting \
             them proves nothing"
        );
        assert_ne!(
            app.state.sim_setup.save_policy.retained_dataset_limit,
            SimulationSavePolicy::default().retained_dataset_limit,
            "the fixture's policy must differ from the default"
        );
        app
    }

    /// One prepared run whose authenticated receipt names `id`.
    ///
    /// The receipt is what makes a result a reference to a plan: the projection
    /// counts a run for a plan when the receipt's plan identity matches, so a
    /// fixture without one would let the exclusion assertion below pass on an
    /// empty set.
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

    fn draft_with(
        inherit_model: bool,
        inherit_options: bool,
        inherit_policy: bool,
    ) -> NewSimulationPlanDraft {
        NewSimulationPlanDraft {
            reference_pvt: CHOSEN_POINT,
            inherit_model_closure: inherit_model,
            inherit_solver_options: inherit_options,
            inherit_save_policy: inherit_policy,
        }
    }

    fn record_for(app: &RSpiceApp, id: SimulationPlanId) -> PlanCatalogRecord {
        plan_catalog_records(app)
            .into_iter()
            .find(|record| record.id == id)
            .unwrap_or_else(|| panic!("plan {id} is projected"))
    }

    /// The transaction's own claims: a new stable identity, an initial revision,
    /// a root lineage, and the analysis `SimulationPlan::new` seeds.
    ///
    /// The revision matters because it is the one row of the minted group a
    /// reader would otherwise have to take on trust, and the instance count
    /// matters because "a fresh plan is empty" is the plausible wrong answer —
    /// `SimulationPlan::new` seeds one enabled transient instance and only
    /// `SimulationPlan::empty` does not.
    #[test]
    fn creating_a_plan_mints_a_fresh_identity_at_the_initial_revision() {
        let mut app = app_with_a_distinctive_active_plan();
        let retired = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();

        let (created, _) = commit_create_plan(
            &mut app,
            "Skew characterization",
            &draft_with(false, false, false),
        )
        .expect("the create transaction commits");

        assert_ne!(created, retired, "the new plan reused the retired identity");
        let record = record_for(&app, created);
        assert!(
            record.active,
            "the created plan is the active editable plan"
        );
        assert!(!record.archived);
        assert_eq!(
            record.revision,
            ObjectRevision::INITIAL.get(),
            "a created plan opens at the initial revision"
        );
        assert_eq!(
            record.lineage.source_plan_id(),
            None,
            "a created plan is a root, so the surface may state it has no source"
        );
        assert_eq!(
            (record.analyses, record.enabled),
            (1, 1),
            "a fresh plan carries one enabled transient instance, which is what \
             the minted group states"
        );
        assert!(
            record.forecast.is_some(),
            "the reference-only run set the transaction installs must validate"
        );
        // The retired plan is still in the catalog, unchanged and not archived.
        let retained = record_for(&app, retired);
        assert!(!retained.active && !retained.archived);
        assert_eq!(retained.revision, 1);
    }

    /// Every one of the four inputs reaches the plan as the catalog stores it —
    /// not merely the live editor state it is installed into.
    ///
    /// A second create retires the first plan into `inactive_plans`, and the
    /// stored record is then read back through `export_plan`. That is the
    /// strongest form of the claim: what survives being retired is what the plan
    /// actually owns, and options and retention have no accessor on the stored
    /// record other than this one.
    #[test]
    fn each_of_the_four_inputs_reaches_the_stored_plan() {
        let mut app = app_with_a_distinctive_active_plan();
        let inherited_options = app.state.sim_setup.options.clone();
        let inherited_policy = app.state.sim_setup.save_policy;
        let inherited_bindings = app.state.sim_setup.model_bindings.clone();

        let (created, _) = commit_create_plan(
            &mut app,
            "Stored characterization",
            &draft_with(true, true, true),
        )
        .expect("the create transaction commits");
        commit_create_plan(
            &mut app,
            "Successor plan",
            &NewSimulationPlanDraft::default(),
        )
        .expect("a second create retires the first plan into the catalog");

        let stored = app
            .state
            .sim_setup
            .export_plan(created)
            .expect("the retired plan is in the catalog");
        assert_eq!(
            stored.reference_pvt, CHOSEN_POINT,
            "the chosen reference point is not what the plan stores"
        );
        assert_eq!(
            stored.options.reltol, inherited_options.reltol,
            "the inherited solver options are not what the plan stores"
        );
        assert_eq!(
            stored.options.temp, CHOSEN_POINT.temperature_celsius,
            "the reference temperature owns the solver TEMP option, so it has to \
             survive both the inherited options block and being retired"
        );
        assert_eq!(
            stored.save_policy.retained_dataset_limit, inherited_policy.retained_dataset_limit,
            "the inherited retention policy is not what the plan stores"
        );
        assert_eq!(
            stored
                .model_bindings
                .iter()
                .map(|binding| binding.library_name.as_str())
                .collect::<Vec<_>>(),
            inherited_bindings
                .iter()
                .map(|binding| binding.library_name.as_str())
                .collect::<Vec<_>>(),
            "the inherited model closure is not what the plan stores"
        );
    }

    /// Each flag off produces the product default and on produces the active
    /// plan's value — one flag at a time, so no flag can be passing on another's
    /// effect.
    #[test]
    fn each_inheritance_flag_is_independently_observable() {
        for (model, options, policy) in [
            (true, false, false),
            (false, true, false),
            (false, false, true),
        ] {
            let mut app = app_with_a_distinctive_active_plan();
            let source = app.state.sim_setup.clone();
            commit_create_plan(
                &mut app,
                "Selective plan",
                &draft_with(model, options, policy),
            )
            .expect("the create transaction commits");
            let setup = &app.state.sim_setup;

            if model {
                assert_eq!(
                    setup.model_bindings.len(),
                    source.model_bindings.len(),
                    "the closure flag alone did not carry the closure"
                );
            } else {
                assert!(
                    setup.model_bindings.is_empty(),
                    "a plan that declines the closure declares an explicitly empty one"
                );
            }

            if options {
                assert_eq!(setup.options.reltol, source.options.reltol);
            } else {
                assert_eq!(
                    setup.options.reltol,
                    SimulationOptions::default().reltol,
                    "a plan that declines the options did not open at the default"
                );
            }
            // Whatever was decided about the options, the chosen temperature is
            // the plan's: it owns TEMP either way.
            assert_eq!(setup.options.temp, CHOSEN_POINT.temperature_celsius);

            if policy {
                assert_eq!(
                    setup.save_policy.retained_dataset_limit,
                    source.save_policy.retained_dataset_limit
                );
            } else {
                assert_eq!(
                    setup.save_policy.retained_dataset_limit,
                    SimulationSavePolicy::default().retained_dataset_limit,
                    "a plan that declines the policy did not open at the default"
                );
            }
        }
    }

    /// The exclusion the surface states: no result reference is copied, and the
    /// plan being retired keeps every one it had.
    ///
    /// Also the two exclusions beside it — the plan-owned records and the
    /// declared run-set axes — which are the other two rows of the group.
    #[test]
    fn creating_a_plan_copies_no_result_reference_or_plan_owned_record() {
        let mut app = app_with_a_distinctive_active_plan();
        let retired = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();
        app.state
            .workspace
            .ensure_active_plan_data(retired)
            .saved_outputs
            .push(retained_output());
        app.state.simulation.runs.push(run_referencing(retired));
        let before = record_for(&app, retired);
        assert_eq!(
            (before.results, before.saved_outputs),
            (1, 1),
            "the fixture must give the retired plan something to keep"
        );

        let (created, _) = commit_create_plan(
            &mut app,
            "Uncontaminated plan",
            &draft_with(true, true, true),
        )
        .expect("the create transaction commits");

        let created = record_for(&app, created);
        assert_eq!(
            created.results, 0,
            "a created plan must reference no result: nothing in this \
             transaction rewrites a run receipt"
        );
        assert_eq!(
            (
                created.design_variables,
                created.saved_outputs,
                created.specifications
            ),
            (0, 0, 0),
            "a created plan's variables, outputs and specifications start empty"
        );
        assert_eq!(
            created.model_bindings, 1,
            "the closure was inherited, so the empty payload above is the \
             transaction's own boundary and not a plan that inherited nothing"
        );
        assert_eq!(
            created.point_count(),
            Some(1),
            "a created plan declares the reference point only, not the axes of \
             the plan it replaced"
        );

        let after = record_for(&app, retired);
        assert_eq!(
            (after.results, after.saved_outputs),
            (before.results, before.saved_outputs),
            "creating a plan took something away from the plan it retired"
        );
    }

    /// One plan-owned record for the retired plan to keep.
    fn retained_output() -> crate::state::SavedOutput {
        crate::state::SavedOutput::new(
            crate::state::SavedOutputKind::RawVoltageOrCurrent,
            "vout".to_owned(),
            "V(out)".to_owned(),
            crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
            crate::state::SavedOutputPolicy::EveryAcceptedPoint,
            crate::state::SavedOutputPrecision::FullSourcePrecision,
            crate::state::SavedOutputStreaming::StoreOnly,
        )
        .expect("a raw node output is valid")
    }

    /// The label carries the closure's size when the projection measures one,
    /// and names the domain without a figure when it does not.
    #[test]
    fn the_closure_label_states_a_size_only_when_one_was_measured() {
        assert_eq!(
            model_closure_label(Some(1)),
            "Ordered model closure · 1 binding"
        );
        assert_eq!(
            model_closure_label(Some(3)),
            "Ordered model closure · 3 bindings"
        );
        assert_eq!(model_closure_label(None), "Ordered model closure");
    }

    /// Every corner the enum declares is offered, and each is offered once.
    ///
    /// The list is authored here because `ProcessCorner` publishes only its three
    /// speed corners as a set, and a control that dropped a skewed corner would
    /// leave a reference point this product supports unreachable from this route.
    #[test]
    fn every_process_corner_is_offered_exactly_once() {
        let mut names = PLAN_PROCESS_CORNERS
            .iter()
            .map(|corner| corner.short_name())
            .collect::<Vec<_>>();
        names.sort_unstable();
        let unique = names.len();
        names.dedup();
        assert_eq!(names.len(), unique, "a corner is offered twice");
        assert!(
            PLAN_PROCESS_CORNERS.contains(&ProcessCorner::default()),
            "the default reference corner is not offered"
        );
        for corner in ProcessCorner::speed_corners() {
            assert!(
                PLAN_PROCESS_CORNERS.contains(&corner),
                "{} is a speed corner and is not offered",
                corner.short_name()
            );
        }
    }

    /// The temperature control cannot produce a value the transaction refuses.
    #[test]
    fn the_coldest_offered_temperature_is_one_the_catalog_accepts() {
        let mut setup = RSpiceApp::test_instance().state.sim_setup.clone();
        assert!(
            setup
                .set_reference_pvt(ProcessCorner::TT, COLDEST_REFERENCE_CELSIUS)
                .is_ok(),
            "the control's floor is a temperature set_reference_pvt refuses"
        );
        assert!(
            setup.set_reference_pvt(ProcessCorner::TT, -273.15).is_err(),
            "absolute zero must still be refused, or this floor guards nothing"
        );
    }

    /// Every fact this route paints lands on screen *whole* at every gated
    /// viewport.
    ///
    /// The shell's route fit gate proves nothing is clipped. It cannot see a
    /// truncated value: a label elided to `Corner char…` sits entirely inside its
    /// clip rect and passes. Elision is how a property row keeps its geometry, so
    /// nothing here may be long enough to need it — which makes the absence of an
    /// ellipsis in the paint the exact statement of that.
    ///
    /// Every string this checks is this route's own: its eyebrow, title,
    /// description and button labels, its setting rows, its controls and its two
    /// statement groups. There is no list of expected labels, deliberately —
    /// rewording a row is covered without editing this test.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_stated_fact_paints_whole_at_every_gated_viewport() {
        for (name, screen) in [
            ("desktop", egui::Vec2::new(1024.0, 640.0)),
            ("edge-to-edge", egui::Vec2::new(820.0, 640.0)),
            ("portrait", egui::Vec2::new(560.0, 900.0)),
        ] {
            let elided = elided_text_at(screen);
            assert!(
                elided.is_empty(),
                "at {name} the create route painted elided values: {elided:?}"
            );
        }
    }

    /// Every string the real dialog painted at one viewport that had to be
    /// shortened to fit the box it was painted in.
    #[cfg(not(target_arch = "wasm32"))]
    fn elided_text_at(screen: egui::Vec2) -> Vec<String> {
        let mut app = RSpiceApp::test_instance();
        let active = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();
        let name = app.state.sim_setup.active_plan_name().to_string();
        let mut draft = SimulationPlanManagerDraft::new(active, name);
        draft.mode = SimulationPlanManagerMode::Create;
        draft.name = "New simulation plan".to_owned();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        // The dialog owns the draft for a frame and the shell re-arms it, so each
        // pass is handed its own copy and the three passes render one state
        // rather than accumulating it.
        let mut pass = || {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
                    ..Default::default()
                },
                |ctx| plan_manager_dialog(ctx, &mut app, draft.clone()),
            )
        };
        let _ = pass();
        let _ = pass();
        let output = pass();
        output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(text) => Some(text.galley.job.text.clone()),
                _ => None,
            })
            .filter(|painted| painted.contains('\u{2026}'))
            .collect()
    }
}
