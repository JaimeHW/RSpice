//! One projection of the simulation-plan catalog, read by every plan-manager
//! surface.
//!
//! The catalog's facts have owners: the analysis plan owns its instances, the
//! run set's validation owns the declared workload, the workspace payload owns
//! the plan's variables, outputs and specifications, and the run list owns
//! which results reference a plan. A [`PlanCatalogRecord`] reads each fact from
//! its owner exactly once, and the derived quantities below are the only
//! implementations of themselves. That is the point: a workload shown in the
//! table, in a comparison, and in a campaign is one number, so the surfaces
//! cannot disagree about how much work a plan declares.
//!
//! In particular the task count comes from the forecast. Multiplying a point
//! count by an enabled-analysis count is different arithmetic — the run set
//! expands analysis families and their prerequisites — so re-deriving it
//! locally stood a second, wrong answer beside the authoritative one.

use crate::product::{RunId, SimulationPlanId};
use crate::simulation::plan::SimulationPlan;
use crate::simulation::run_set::{
    RunSetForecast, RunSetState, format_bytes, format_duration_ms, validate,
};
use crate::state::SimulationPlanPayload;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::{ReferencePvtPoint, SimulationPlanLineage};

/// Everything the plan-manager surfaces know about one catalog entry.
///
/// Every field has a reader. The manager's table shows the lifecycle and the
/// declared scale of each plan, and its selected-plan aside states the rest —
/// the reference corner, the run set's own forecast, the model closure, the
/// plan-owned record counts, the pinned regression baseline, and the source
/// plan a clone or import came from. A field with no reader was a fact the
/// projection collected and no one could see, so it is not a shape this record
/// is allowed to hold.
#[derive(Debug, Clone)]
pub(super) struct PlanCatalogRecord {
    pub(super) id: SimulationPlanId,
    pub(super) name: String,
    pub(super) revision: u64,
    pub(super) active: bool,
    pub(super) archived: bool,
    /// Total analysis instances, enabled or not.
    pub(super) analyses: usize,
    pub(super) enabled: usize,
    /// The run set's own forecast, or `None` when the declaration does not
    /// validate. Nothing here recomputes a quantity the forecast carries.
    pub(super) forecast: Option<RunSetForecast>,
    pub(super) model_bindings: usize,
    /// The corner and temperature this plan resolves an undeclared axis to.
    /// Never optional: the active setup and every stored plan each carry one,
    /// so a plan whose corner read as unknown was a gap in this projection
    /// rather than a plan without a reference point.
    pub(super) reference_pvt: ReferencePvtPoint,
    pub(super) design_variables: usize,
    pub(super) saved_outputs: usize,
    pub(super) specifications: usize,
    pub(super) regression_baseline: Option<RunId>,
    pub(super) lineage: SimulationPlanLineage,
    /// Runs whose authenticated receipt names this plan.
    pub(super) results: usize,
}

impl PlanCatalogRecord {
    /// Declared PVT points, or `None` when the run set does not validate.
    pub(super) const fn point_count(&self) -> Option<usize> {
        match self.forecast {
            Some(forecast) => Some(forecast.point_count),
            None => None,
        }
    }

    /// Authenticated tasks the run set expands to, or `None` when the run set
    /// does not validate.
    pub(super) const fn task_count(&self) -> Option<usize> {
        match self.forecast {
            Some(forecast) => Some(forecast.task_count),
            None => None,
        }
    }

    /// The declared run set as the table states it.
    ///
    /// One owner, because the manager's filter matches this text and its table
    /// paints it. Two spellings of "invalid" would make a plan findable by a
    /// word the row does not show, or shown a word it cannot be found by.
    pub(super) fn run_set_label(&self) -> String {
        self.point_count()
            .map_or_else(|| "invalid".to_owned(), |count| format!("{count} PVT"))
    }

    pub(super) const fn lifecycle_label(&self) -> &'static str {
        if self.active {
            "active"
        } else if self.archived {
            "archived"
        } else {
            "available"
        }
    }
}

/// The forecast's cost and storage, formatted for display.
///
/// They live on the record rather than at the call site so that a second
/// surface showing a cost cannot invent its own notion of how long a plan
/// takes: both delegate to the run set's own formatters, and both return `None`
/// for a declaration that does not validate rather than a formatted zero.
impl PlanCatalogRecord {
    pub(super) fn estimated_duration(&self) -> Option<String> {
        self.forecast
            .map(|forecast| format_duration_ms(forecast.cost_ms))
    }

    pub(super) fn estimated_storage(&self) -> Option<String> {
        self.forecast
            .map(|forecast| format_bytes(forecast.storage_bytes))
    }
}

/// Project the whole plan catalog in a stable order: the active plan first,
/// then the inactive plans in the order the catalog already keeps them.
///
/// The order must not depend on anything that changes between frames. A
/// selection in this dialog is a row identity, and a table that reorders itself
/// moves the row out from under the pointer.
///
/// One fact comes from a narrower owner than the rest. `plan_data` resolves by
/// identity alone, so it serves the stored payload of any plan, active or not,
/// and a plan with no payload record contributes zero rather than borrowing a
/// neighbour's counts.
pub(super) fn plan_catalog_records(app: &RSpiceApp) -> Vec<PlanCatalogRecord> {
    let result_count = |id: SimulationPlanId| {
        app.state
            .simulation
            .runs
            .iter()
            .filter(|run| {
                run.prepared_receipt()
                    .is_some_and(|receipt| receipt.simulation_plan_id() == Some(id))
            })
            .count()
    };
    let payload = |id: SimulationPlanId| app.state.workspace.plan_data(id);
    let mut records = Vec::with_capacity(app.state.sim_setup.plan_count());
    if let Ok(plan) = app.state.sim_setup.stable_analysis_plan() {
        let id = plan.id();
        let enabled = enabled_instance_count(plan);
        records.push(PlanCatalogRecord {
            id,
            name: app.state.sim_setup.active_plan_name().to_string(),
            revision: plan.revision().get(),
            active: true,
            archived: false,
            analyses: plan.instances().len(),
            enabled,
            forecast: forecast_for(&app.state.sim_setup.run_set, enabled),
            model_bindings: app.state.sim_setup.model_bindings.len(),
            reference_pvt: app.state.sim_setup.reference_pvt,
            design_variables: payload(id).map_or(0, |payload| payload.design_variables.len()),
            saved_outputs: payload(id).map_or(0, |payload| payload.saved_outputs.len()),
            specifications: payload(id).map_or(0, specification_count),
            regression_baseline: payload(id).and_then(|payload| payload.regression_baseline_run),
            lineage: app.state.sim_setup.active_plan_lineage(),
            results: result_count(id),
        });
    }
    records.extend(app.state.sim_setup.inactive_plans().iter().map(|stored| {
        let id = stored.id();
        let enabled = enabled_instance_count(stored.analysis_plan());
        PlanCatalogRecord {
            id,
            name: stored.name().to_string(),
            revision: stored.revision().get(),
            active: false,
            archived: stored.archived(),
            analyses: stored.analysis_plan().instances().len(),
            enabled,
            forecast: forecast_for(stored.run_set(), enabled),
            model_bindings: stored.model_bindings().len(),
            reference_pvt: stored.reference_pvt(),
            design_variables: payload(id).map_or(0, |payload| payload.design_variables.len()),
            saved_outputs: payload(id).map_or(0, |payload| payload.saved_outputs.len()),
            specifications: payload(id).map_or(0, specification_count),
            regression_baseline: payload(id).and_then(|payload| payload.regression_baseline_run),
            lineage: stored.lineage(),
            results: result_count(id),
        }
    }));
    records
}

fn enabled_instance_count(plan: &SimulationPlan) -> usize {
    plan.instances()
        .iter()
        .filter(|instance| instance.enabled())
        .count()
}

/// The run set's forecast, kept only when the declaration validates. A forecast
/// standing beside an error predicts nothing that can be dispatched.
fn forecast_for(run_set: &RunSetState, enabled: usize) -> Option<RunSetForecast> {
    let validation = validate(run_set, enabled);
    validation.errors.is_empty().then_some(validation.forecast)
}

/// Governed specification records, which the scalar projection mirrors one for
/// one. A project predating the governed model is migrated on access, so the
/// scalar entries are the count until that plan has been opened.
fn specification_count(payload: &SimulationPlanPayload) -> usize {
    if payload.specification_definitions.is_empty() {
        payload.specs.len()
    } else {
        payload.specification_definitions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProcessCorner;
    use crate::simulation::run_set::RunSetCompositionMode;

    /// Every plan-manager source file. A later wave adds its own file here, so
    /// the duplication guard covers the module rather than two files of it.
    const PLAN_MANAGER_SOURCES: &[(&str, &str)] = &[
        ("plan_manager.rs", include_str!("../plan_manager.rs")),
        ("kit.rs", include_str!("kit.rs")),
        ("campaign.rs", include_str!("campaign.rs")),
        ("compare.rs", include_str!("compare.rs")),
        ("create.rs", include_str!("create.rs")),
        ("exchange.rs", include_str!("exchange.rs")),
        ("lifecycle.rs", include_str!("lifecycle.rs")),
        ("records.rs", include_str!("records.rs")),
    ];

    /// A second plan retained inactive, so the projection has both kinds of
    /// catalog entry to report on. Returns the active and inactive ids.
    fn app_with_an_inactive_plan() -> (RSpiceApp, SimulationPlanId, SimulationPlanId) {
        let mut app = RSpiceApp::test_instance();
        let first = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();
        let mut setup = app.state.sim_setup.clone();
        let second = setup
            .create_plan("Second characterization")
            .expect("a fresh root plan is created");
        app.state.workspace.migrate_active_plan_data(first);
        app.state.workspace.migrate_inactive_plan_data(second);
        app.state.sim_setup = setup;
        // `create_plan` activates what it creates, so the original plan is now
        // the retained inactive entry.
        (app, second, first)
    }

    #[test]
    fn the_active_plan_reports_every_field_from_its_owner() {
        let mut app = RSpiceApp::test_instance();
        let id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("stable plan")
            .id();
        let mut setup = app.state.sim_setup.clone();
        setup
            .set_reference_pvt(ProcessCorner::FF, -40.0)
            .expect("a physical reference corner");
        app.state.sim_setup = setup;
        app.state.workspace.migrate_active_plan_data(id);

        let records = plan_catalog_records(&app);
        let record = records.first().expect("the active plan is projected");
        let plan = app.state.sim_setup.stable_analysis_plan().unwrap();

        assert_eq!(record.id, id);
        assert!(record.active);
        assert!(!record.archived);
        assert_eq!(record.lifecycle_label(), "active");
        assert_eq!(
            record.name,
            app.state.sim_setup.active_plan_name().as_str(),
            "the name comes from the setup's active plan name"
        );
        assert_eq!(record.revision, plan.revision().get());
        assert_eq!(record.analyses, plan.instances().len());
        assert!(record.analyses > 0, "the fixture plan owns instances");
        assert_eq!(record.enabled, enabled_instance_count(plan));
        assert_eq!(
            record.model_bindings,
            app.state.sim_setup.model_bindings.len()
        );
        assert_eq!(
            record.reference_pvt, app.state.sim_setup.reference_pvt,
            "the active plan's reference corner is the setup's"
        );
        assert_eq!(
            record.reference_pvt.process,
            ProcessCorner::FF,
            "and it tracks a committed change to it"
        );

        let payload = app
            .state
            .workspace
            .plan_data(id)
            .expect("the active plan owns a payload");
        assert_eq!(record.design_variables, payload.design_variables.len());
        assert_eq!(record.saved_outputs, payload.saved_outputs.len());
        assert_eq!(record.specifications, specification_count(payload));
        assert_eq!(record.regression_baseline, payload.regression_baseline_run);
        assert_eq!(
            record.lineage,
            app.state.sim_setup.active_plan_lineage(),
            "a plan reports the lineage its own owner holds"
        );
        assert_eq!(
            record.lineage.source_plan_id(),
            None,
            "the fixture's plan is a root, so it has no source plan"
        );
        assert_eq!(record.results, 0, "the fixture has dispatched nothing");

        let forecast = record.forecast.expect("the default run set validates");
        assert_eq!(
            forecast.enabled_analysis_count, record.enabled,
            "the forecast was validated against this plan's enabled count"
        );
        assert!(record.estimated_duration().is_some());
        assert!(record.estimated_storage().is_some());
    }

    #[test]
    fn an_inactive_plan_reports_its_own_stored_facts() {
        let (app, active_id, inactive_id) = app_with_an_inactive_plan();

        let records = plan_catalog_records(&app);
        assert_eq!(records.len(), 2, "both catalog entries are projected");
        let record = records
            .iter()
            .find(|record| record.id == inactive_id)
            .expect("the retained plan is projected");
        let stored = app
            .state
            .sim_setup
            .inactive_plans()
            .iter()
            .find(|plan| plan.id() == inactive_id)
            .expect("the retained plan is in the catalog");

        assert!(!record.active);
        assert!(!record.archived);
        assert_eq!(record.lifecycle_label(), "available");
        assert_eq!(record.name, stored.name().as_str());
        assert_eq!(record.revision, stored.revision().get());
        assert_eq!(record.analyses, stored.analysis_plan().instances().len());
        assert_eq!(
            record.enabled,
            enabled_instance_count(stored.analysis_plan())
        );
        assert_eq!(record.model_bindings, stored.model_bindings().len());
        assert_eq!(record.lineage, stored.lineage());
        assert_eq!(
            record.reference_pvt,
            stored.reference_pvt(),
            "a stored plan reports the reference corner it holds, not the \
             active plan's and not 'unknown'"
        );
        assert!(
            record.forecast.is_some(),
            "a retained plan's own run set forecasts its own workload"
        );

        // The counts belong to this plan's own payload, never to the active
        // plan's: reading a neighbour's payload would be a lie, so an absent
        // record reports zero.
        let own = app.state.workspace.plan_data(inactive_id);
        assert_eq!(
            record.design_variables,
            own.map_or(0, |payload| payload.design_variables.len())
        );
        assert_eq!(
            record.saved_outputs,
            own.map_or(0, |payload| payload.saved_outputs.len())
        );
        assert_eq!(record.specifications, own.map_or(0, specification_count));
        assert_eq!(
            record.regression_baseline,
            own.and_then(|payload| payload.regression_baseline_run)
        );
        assert_ne!(record.id, active_id);
    }

    /// A plan whose payload is absent reports zero rather than the active
    /// plan's numbers.
    #[test]
    fn a_plan_with_no_payload_record_reports_zero_rather_than_a_neighbours_counts() {
        let (mut app, active_id, inactive_id) = app_with_an_inactive_plan();
        app.state
            .workspace
            .ensure_active_plan_data(active_id)
            .saved_outputs
            .push(crate::state::SavedOutput::new(
                crate::state::SavedOutputKind::RawVoltageOrCurrent,
                "vout".to_owned(),
                "V(out)".to_owned(),
                crate::state::SavedOutputCompatibility::AllCompatibleAnalyses,
                crate::state::SavedOutputPolicy::EveryAcceptedPoint,
                crate::state::SavedOutputPrecision::FullSourcePrecision,
                crate::state::SavedOutputStreaming::StoreOnly,
            )
            .expect("a raw node output is valid"));
        app.state
            .workspace
            .simulation_plan_payloads
            .retain(|record| record.plan_id != inactive_id);

        let records = plan_catalog_records(&app);
        let active = records
            .iter()
            .find(|record| record.id == active_id)
            .expect("the active plan is projected");
        let inactive = records
            .iter()
            .find(|record| record.id == inactive_id)
            .expect("a plan with no payload is still projected");

        assert_eq!(active.saved_outputs, 1);
        assert_eq!(
            inactive.saved_outputs, 0,
            "an absent payload is zero, not the active plan's count"
        );
        assert_eq!(inactive.design_variables, 0);
        assert_eq!(inactive.specifications, 0);
        assert_eq!(inactive.regression_baseline, None);
    }

    /// The corner an inactive plan reports is the one it stores. Reading it off
    /// the active setup would print another plan's fact under this plan's name,
    /// which is the same defect as reporting it unknown — just harder to see.
    #[test]
    fn an_inactive_plans_reference_corner_is_its_own_not_the_active_plans() {
        let (mut app, active_id, inactive_id) = app_with_an_inactive_plan();
        let stored_corner = app
            .state
            .sim_setup
            .inactive_plans()
            .iter()
            .find(|plan| plan.id() == inactive_id)
            .expect("the retained plan is in the catalog")
            .reference_pvt();
        let mut setup = app.state.sim_setup.clone();
        setup
            .set_reference_pvt(ProcessCorner::SS, 125.0)
            .expect("a physical reference corner");
        app.state.sim_setup = setup;

        let records = plan_catalog_records(&app);
        let active = records
            .iter()
            .find(|record| record.id == active_id)
            .expect("the active plan is projected");
        let inactive = records
            .iter()
            .find(|record| record.id == inactive_id)
            .expect("the retained plan is projected");

        assert_eq!(active.reference_pvt.process, ProcessCorner::SS);
        assert_eq!(inactive.reference_pvt, stored_corner);
        assert_ne!(
            inactive.reference_pvt, active.reference_pvt,
            "committing a corner on the active plan must not move the retained \
             plan's"
        );
    }

    #[test]
    fn an_archived_plan_reports_the_archived_lifecycle() {
        let (mut app, _, inactive_id) = app_with_an_inactive_plan();
        let mut setup = app.state.sim_setup.clone();
        setup
            .archive_plan(inactive_id)
            .expect("an inactive plan archives");
        app.state.sim_setup = setup;

        let records = plan_catalog_records(&app);
        let record = records
            .iter()
            .find(|record| record.id == inactive_id)
            .expect("an archived plan stays in the catalog");
        assert!(record.archived);
        assert_eq!(record.lifecycle_label(), "archived");
    }

    #[test]
    fn an_invalid_run_set_carries_no_forecast() {
        let mut app = RSpiceApp::test_instance();
        let mut setup = app.state.sim_setup.clone();
        // Nested composition with a zero maximum depth is a declared run-space
        // error, so there is no workload to forecast.
        setup.run_set.composition.mode = RunSetCompositionMode::Nested;
        setup.run_set.composition.maximum_depth = 0;
        app.state.sim_setup = setup;

        let records = plan_catalog_records(&app);
        let record = records.first().expect("the active plan is projected");
        assert!(
            record.forecast.is_none(),
            "an invalid run set forecasts nothing"
        );
        assert_eq!(record.point_count(), None);
        assert_eq!(record.task_count(), None);
        assert_eq!(record.estimated_duration(), None);
        assert_eq!(record.estimated_storage(), None);
    }

    #[test]
    fn the_order_is_the_active_plan_then_the_catalog_order() {
        let (app, active_id, inactive_id) = app_with_an_inactive_plan();

        let first = plan_catalog_records(&app);
        assert_eq!(first[0].id, active_id, "the active plan leads");
        assert!(first[0].active);
        assert_eq!(first[1].id, inactive_id);
        assert_eq!(
            first[1..]
                .iter()
                .map(|record| record.id)
                .collect::<Vec<_>>(),
            app.state
                .sim_setup
                .inactive_plans()
                .iter()
                .map(|plan| plan.id())
                .collect::<Vec<_>>(),
            "the inactive plans keep the catalog's order"
        );

        // Stable across frames: the projection reads state, never a clock, a
        // hash iteration order, or a frame counter.
        let second = plan_catalog_records(&app);
        assert_eq!(
            first.iter().map(|record| record.id).collect::<Vec<_>>(),
            second.iter().map(|record| record.id).collect::<Vec<_>>()
        );
    }

    #[test]
    fn every_derived_quantity_is_the_forecasts_own_number() {
        let app = RSpiceApp::test_instance();
        let records = plan_catalog_records(&app);
        let record = records.first().expect("the active plan is projected");
        let forecast = record.forecast.expect("the default run set validates");

        assert_eq!(record.task_count(), Some(forecast.task_count));
        assert_eq!(record.point_count(), Some(forecast.point_count));
        assert_eq!(
            record.estimated_duration(),
            Some(format_duration_ms(forecast.cost_ms))
        );
        assert_eq!(
            record.estimated_storage(),
            Some(format_bytes(forecast.storage_bytes))
        );
    }

    /// Opening the plan manager has one owner. The Simulation Studio's title
    /// row used to build its own draft, which meant a second availability rule
    /// beside the command's — and only the command's says truthfully why it
    /// refuses when there is no stable analysis plan.
    #[test]
    fn the_studio_opens_the_plan_manager_through_its_command() {
        let studio = crate::source_guard::production_source(include_str!("../../simulate.rs"));
        assert!(
            !studio.contains("SimulationPlanManagerDraft::new"),
            "simulate.rs builds a plan-manager draft of its own; \
             Command::ManageSimulationPlans owns opening the plan manager, \
             along with its availability check and its refusal message"
        );
        assert!(
            studio.contains("Command::ManageSimulationPlans.execute(app)"),
            "the Simulation Studio's title row no longer routes through \
             Command::ManageSimulationPlans"
        );
        for (name, source) in PLAN_MANAGER_SOURCES {
            let production = crate::source_guard::production_source(source);
            assert!(
                !production.contains("SimulationPlanManagerDraft::new"),
                "{name} builds a plan-manager draft; the command owns that"
            );
        }
    }

    /// The defect this projection exists to close: the campaign table used to
    /// multiply a point count by an enabled-analysis count, which is not the
    /// workload the run set expands to.
    #[test]
    fn nothing_in_the_plan_manager_re_derives_a_workload() {
        for (name, source) in PLAN_MANAGER_SOURCES {
            let production = crate::source_guard::production_source(source);
            for forbidden in ["saturating_mul", "pvt_points"] {
                assert!(
                    !production.contains(forbidden),
                    "{name} contains '{forbidden}': the run set's forecast owns the \
                     workload, so a plan-manager surface reads task_count() rather \
                     than multiplying a point count by an enabled-analysis count"
                );
            }
            if *name != "records.rs" {
                assert!(
                    !production.contains("forecast."),
                    "{name} reads a forecast field directly; records.rs is the one \
                     owner of every quantity derived from a forecast"
                );
            }
        }
    }
}
