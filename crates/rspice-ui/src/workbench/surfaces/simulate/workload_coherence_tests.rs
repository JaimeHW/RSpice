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
