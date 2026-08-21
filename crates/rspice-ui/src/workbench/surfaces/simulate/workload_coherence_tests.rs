//! Every surface that promises a queue promises the same queue.
//!
//! Five surfaces state how much work the plan is: the Run Set forecast tile,
//! the resolved point table's status line, the preview receipt, the preflight
//! Execution cell, and the task-rate table. Before this lane they stated it in
//! two currencies — four counted tasks and one of those also priced them —
//! and the pricing was an inline multiplication rather than a shared one.
//!
//! Two things can go wrong and only one of them is obvious. The obvious one is
//! a duration that disagrees with its own task count. The other is a task-rate
//! table whose rows do not add up to the number the budget is checked against:
//! a table that shows its working is the more convincing of two disagreeing
//! numbers, and it would be the wrong one. These tests pin both against the
//! single owner, and pin that the per-analysis rows honour participation
//! rather than pricing every analysis at the full matrix.

use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::simulation::run_set::{self, AnalysisRunAt};
use crate::workbench::RSpiceApp;

use super::page_runset::{exact_plan_task_count, plan_run_set_validation};
use super::page_runset_parity_tests::{
    app_with, drive_pss_from_the_fixture_supply, enable_only_the_temperature_axis, instance_of,
};
use super::workload::{PlanWorkload, modelled_duration};

#[test]
fn the_task_rate_rows_add_up_to_the_number_the_budget_is_checked_against() {
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    let points = enable_only_the_temperature_axis(&mut app.state);
    assert!(points > 1, "the fixture space must actually multiply");

    let workload = PlanWorkload::resolve(&app).expect("the fixture workload prices");
    let summed: usize = workload.rows.iter().map(|row| row.tasks()).sum();
    let scalar = exact_plan_task_count(&app)
        .expect("the page can forecast this workload")
        .expect("a reference-only Run Set has an exact count");

    assert_eq!(
        summed, scalar,
        "the task-rate rows and the plan's scalar count are one fold read twice"
    );
    assert_eq!(
        scalar,
        plan_run_set_validation(&app).forecast.task_count,
        "the forecast the tile and the point table read is the same count"
    );
}

#[test]
fn a_row_prices_the_rate_times_the_participation() {
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    let points = enable_only_the_temperature_axis(&mut app.state);

    let workload = PlanWorkload::resolve(&app).expect("the fixture workload prices");
    let pss = workload
        .rows
        .iter()
        .find(|row| row.kind == AnalysisKind::Pss)
        .expect("the fixture plan holds a PSS instance");

    // A PSS retaining harmonics costs its steady state plus a spectrum, and
    // the fixture leaves the harmonic count at its default. That is the one
    // rate in the fixture that is not one, so it is what proves the table
    // multiplies two factors rather than restating a point count.
    assert_eq!(
        pss.tasks_per_point, 2,
        "a retained spectrum is a second task"
    );
    assert_eq!(
        pss.points, points,
        "an unnarrowed analysis visits every point"
    );
    assert_eq!(pss.tasks(), 2 * points);
    assert!(
        pss.rate_note.is_some(),
        "a rate above one has to say why, or the cell is unaccountable"
    );
}

#[test]
fn narrowing_one_analysis_shrinks_its_row_and_the_plan_duration() {
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    let points = enable_only_the_temperature_axis(&mut app.state);
    assert!(points > 1, "the fixture space must actually multiply");

    // The default temperature axis is -40/25/125 and the default reference is
    // 27 °C, so nothing sits on the reference and a nominal participation
    // would refuse. Move the reference onto a declared value: this test is
    // about narrowing, and the refusal is pinned separately below.
    app.state.sim_setup.reference_pvt.temperature_celsius = 25.0;

    let before = PlanWorkload::resolve(&app).expect("the fixture workload prices");
    let before_tasks = before
        .total_tasks()
        .expect("the fixture is within capacity");
    let before_ms = run_set::modelled_cost_ms(before_tasks, before.cost_per_task_ms);

    // Narrow the operating point to the reference condition alone. This is the
    // whole reason participation exists, and a task-rate table that priced
    // every analysis at the full matrix would not move at all here.
    let op = instance_of(&app.state, AnalysisKind::OperatingPoint);
    app.state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .set_run_at(op, AnalysisRunAt::NominalPoint)
        .expect("the operating point narrows to the nominal condition");

    let after = PlanWorkload::resolve(&app).expect("the narrowed workload prices");
    let op_row = after
        .rows
        .iter()
        .find(|row| row.id == op)
        .expect("the narrowed instance still has a row");
    assert_eq!(
        op_row.points, 1,
        "a nominal-only analysis visits exactly the reference point"
    );
    assert_eq!(op_row.at_cell(after.matrix_points), "nominal");

    let after_tasks = after.total_tasks().expect("the fixture is within capacity");
    assert_eq!(
        after_tasks,
        before_tasks - (points - 1),
        "narrowing to one point drops exactly the points it stopped visiting"
    );
    let after_ms = run_set::modelled_cost_ms(after_tasks, after.cost_per_task_ms);
    assert!(
        after_ms < before_ms,
        "a shorter queue is a shorter run: {after_ms} ms must be under {before_ms} ms"
    );
    assert_eq!(
        exact_plan_task_count(&app)
            .expect("the page still forecasts")
            .expect("still exact"),
        after_tasks,
        "the scalar count honours the narrowing its own table reported"
    );
}

#[test]
fn a_participation_that_refuses_is_priced_at_the_whole_matrix() {
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    let points = enable_only_the_temperature_axis(&mut app.state);

    // The default axis declares -40/25/125 and the default reference is 27 °C,
    // so a nominal-only instance names a point the space does not contain.
    let op = instance_of(&app.state, AnalysisKind::OperatingPoint);
    app.state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .set_run_at(op, AnalysisRunAt::NominalPoint)
        .expect("the plan stores the participation it cannot yet resolve");

    let workload = PlanWorkload::resolve(&app).expect("an unresolved row still prices");
    let op_row = workload
        .rows
        .iter()
        .find(|row| row.id == op)
        .expect("the instance still has a row");

    assert!(op_row.unresolved, "the row has to say it did not resolve");
    assert_eq!(
        op_row.points, points,
        "an unresolved participation is priced at the whole space, never at zero: \
         a budget that silently shrinks is the failure worth refusing over"
    );
    assert_eq!(op_row.at_cell(workload.matrix_points), "unresolved");
}

#[test]
fn a_refused_composition_is_not_priced_at_all() {
    // A temperature sweep owns its own point expansion — in both axis modes,
    // since inheriting the run-set axis authors the numbers once but still
    // walks them here — and so does an enabled global Run Set. Two expansions
    // over one run is refused by `validate_for_plan` and refused again by the
    // prepared snapshot before it mints a task.
    //
    // So there is no queue. The card must say so rather than multiply a rate by
    // the matrix: that number would be arithmetically consistent with every
    // other number on the page and describe a run that cannot start, which is
    // the most expensive way to be wrong.
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Temperature]);
    enable_only_the_temperature_axis(&mut app.state);

    let validation = plan_run_set_validation(&app);
    assert!(
        validation
            .errors
            .iter()
            .any(|error| error.id == "RUNSET-ANALYSIS-COMPOSITION"),
        "the fixture must actually be the refused composition"
    );

    // The prepared expansion agrees, and mints nothing.
    let frozen = app.state.clone();
    assert!(
        app.simulation_controller
            .prepare_run_set_for_preflight(&frozen)
            .is_err(),
        "the expansion refuses a plan with two point authorities"
    );

    assert!(
        super::workload::composition_refusals(&validation)
            .next()
            .is_some(),
        "the task-rate card reads that refusal and states it instead of a total"
    );
}

#[test]
fn every_surface_prices_the_same_queue_in_the_same_currency() {
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Pss]);
    drive_pss_from_the_fixture_supply(&mut app.state);
    enable_only_the_temperature_axis(&mut app.state);

    let workload = PlanWorkload::resolve(&app).expect("the fixture workload prices");
    let tasks = workload
        .total_tasks()
        .expect("the fixture is within capacity");
    let validation = plan_run_set_validation(&app);
    let forecast = validation.forecast;

    // The forecast tile's "Cost" row and the preview receipt both read
    // `forecast.cost_ms`; the point table's status line and the preflight
    // Execution cell both call `modelled_duration`; the task-rate card sums
    // its rows. All three have to be one sentence about one queue.
    let tile = run_set::format_duration_ms(forecast.cost_ms);
    let strip = modelled_duration(&app.state, forecast.task_count);
    let card = workload.total_duration().expect("the fixture prices");

    assert_eq!(forecast.task_count, tasks, "one queue");
    assert_eq!(tile, strip, "the forecast tile and the strips agree");
    assert_eq!(tile, card, "the forecast tile and the task-rate card agree");

    // And the rows themselves add up in milliseconds, not just in tasks — a
    // per-row duration rounded independently could sum to a different total
    // than the total the tile shows.
    let summed_ms: u64 = workload
        .rows
        .iter()
        .map(|row| run_set::modelled_cost_ms(row.tasks(), workload.cost_per_task_ms))
        .sum();
    assert_eq!(
        summed_ms, forecast.cost_ms,
        "the per-row durations are the total duration, split"
    );
}

#[test]
fn a_plan_with_no_axes_prices_every_analysis_once() {
    // With no declared space there is no participation to honour, and every
    // enabled analysis costs its rate exactly once. This is the case a table
    // built on point counts would divide by zero on.
    let app = app_with(&[AnalysisKind::OperatingPoint]);
    let workload = PlanWorkload::resolve(&app).expect("an axis-free plan prices");

    assert_eq!(workload.matrix_points, 1);
    for row in &workload.rows {
        assert_eq!(row.points, 1, "no axes means one visit");
        assert!(
            matches!(row.run_at, AnalysisRunAt::AllPoints),
            "with nothing to narrow against, no row may claim a narrowing"
        );
    }
    assert_eq!(
        workload.total_tasks().expect("within capacity"),
        exact_plan_task_count(&app)
            .expect("the page forecasts")
            .expect("exact"),
    );
}

#[test]
fn an_invalid_workload_refuses_rather_than_pricing_the_plan_short() {
    let mut app = app_with(&[AnalysisKind::OperatingPoint, AnalysisKind::Temperature]);
    // A temperature sweep declares its own points, and an unparseable one has
    // no number of them. Pricing it at one task would understate the queue by
    // however many temperatures it actually holds, which is precisely the
    // budget an operator would approve by mistake.
    let temperature = instance_of(&app.state, AnalysisKind::Temperature);
    app.state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .edit(temperature, |draft| {
            let AnalysisDraft::Temperature(draft) = draft else {
                panic!("a temperature instance owns a temperature draft");
            };
            // Set before the initialization flag so `ensure_initialized` does
            // not reset the draft out from under the edit.
            draft.initialized = true;
            draft.specific_temps = "not a temperature".to_owned();
        })
        .expect("the temperature list edits");

    let refusal = PlanWorkload::resolve(&app);
    assert!(
        refusal.is_err(),
        "an unreadable workload is a refusal, never a row priced at one"
    );
    assert!(
        exact_plan_task_count(&app).is_err(),
        "the scalar count refuses wherever its table refuses"
    );
}

/// A retracing DC sweep solves roughly twice the points and still costs one
/// task, because the retrace happens *inside* the task rather than minting a
/// second one.
///
/// This is worth pinning precisely because the "2x" is real and tempting. The
/// rate column is tasks per point, and `exact_plan_task_count` — which is what
/// a dispatch is authorized against — is the sum of the rows. Doubling the rate
/// to reflect solver effort would make the plan claim a queue with two DC tasks
/// in it when the prepared expansion mints one, which is the same class of
/// disagreement these tests exist to stop, in the opposite direction: not a
/// count that understates the queue, but one that invents work the queue never
/// contains.
///
/// The extra effort is real and is simply not expressed in this currency. The
/// duration model prices every task at one flat per-task budget, so a 10-point
/// AC and a 10,000-point AC cost the same here too; a retracing sweep is not a
/// special case of that limitation.
#[test]
fn a_retracing_dc_sweep_is_still_one_task_per_point() {
    let mut app = app_with(&[AnalysisKind::DcSweep]);
    let dc = instance_of(&app.state, AnalysisKind::DcSweep);

    let rate_of = |app: &RSpiceApp| {
        let workload = PlanWorkload::resolve(app).expect("the DC fixture prices");
        let row = workload
            .rows
            .iter()
            .find(|row| row.id == dc)
            .expect("the DC instance owns a row");
        (row.tasks_per_point, row.tasks(), workload.total_tasks())
    };

    let (one_way_rate, one_way_tasks, one_way_total) = rate_of(&app);
    assert_eq!(
        one_way_rate, 1,
        "an ordinary DC sweep is one task per point"
    );

    app.state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("stable plan")
        .edit(dc, |draft| {
            let AnalysisDraft::DcSweep(draft) = draft else {
                panic!("a DC instance owns a DC draft");
            };
            draft.hysteresis = true;
        })
        .expect("retracing is a valid edit");

    let (retrace_rate, retrace_tasks, retrace_total) = rate_of(&app);
    assert_eq!(
        retrace_rate, one_way_rate,
        "the retrace is solved inside the one task, so the rate is unchanged"
    );
    assert_eq!(retrace_tasks, one_way_tasks);
    let retrace_total = retrace_total.expect("within capacity");
    assert_eq!(retrace_total, one_way_total.expect("within capacity"));

    // And the scalar the budget is checked against agrees, which is the whole
    // point: the rows and the authorized queue stay one number.
    assert_eq!(
        retrace_total,
        exact_plan_task_count(&app)
            .expect("the page forecasts")
            .expect("exact"),
    );
}
