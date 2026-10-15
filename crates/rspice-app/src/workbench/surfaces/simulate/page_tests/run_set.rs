//! What the PVT, sweeps & variation route promises.
//!
//! The page owns three facts — the axes it declares, the forecast they resolve
//! to, and the rule that combines them — and it owes the reader the refusals
//! and advisories that come out of validating them. These cases read the text
//! the route paints; `page_raster` reads where it lands.

use rspice_simulation_contract::run_set::{RunSetAction, RunSetDimensionKind, dispatch};

use super::{render, render_with};
use crate::workbench::state::SimulationPage;

/// The run-space page states its declaration, its cost and its composition.
///
/// Each of these is a fact the page is the owner of: the axes it declares, the
/// forecast they resolve to, and the rule that combines them. The forecast is
/// the toolbar's own line — points, tasks, duration and storage, beside the
/// working revision they belong to — and a page that drew the strip without it
/// would be showing a space with no stated cost.
#[test]
fn the_run_space_page_states_every_axis_and_what_they_resolve_to() {
    let rendered = render_with(SimulationPage::RunSet, 1200.0, |app| {
        for dimension in &mut app.state.sim_setup.run_set.dimensions {
            dimension.enabled = matches!(
                dimension.kind,
                RunSetDimensionKind::ProcessSection
                    | RunSetDimensionKind::Supply
                    | RunSetDimensionKind::Temperature
            );
        }
    });

    for fragment in [
        "Run space",
        "Process section",
        "Supply voltage",
        "Temperature",
        "cartesian composition",
        "Task rate",
        "Resolved point table",
        "Execution budgets",
        "Composition",
    ] {
        assert!(
            rendered.contains(fragment),
            "the run-space page omitted {fragment:?}:\n{rendered}"
        );
    }
    // Once, on the toolbar, against the revision it is a forecast of. The
    // strip used to close on a tile that stated the same five numbers, three
    // of which the two tables below already carried in their heads.
    assert!(
        rendered.contains("27 points"),
        "the toolbar must state the resolved point count:\n{rendered}"
    );
    assert_eq!(
        rendered.matches("27 points").count(),
        1,
        "the forecast has one owner:\n{rendered}"
    );
    let line = rendered
        .lines()
        .find(|line| line.starts_with("working revision"))
        .expect("the toolbar states the working revision");
    assert!(
        line.contains("27 points") && line.contains("task"),
        "the toolbar's forecast line is the revision, the points and the queue \
         in one statement: {line:?}"
    );
}

/// A refusal is shown against the declaration that caused it, and the invalid
/// value stays on screen. Dropping it would leave the user with an error about
/// a value they could no longer see.
#[test]
fn an_unresolvable_run_space_shows_its_refusal_and_keeps_the_invalid_value() {
    let rendered = render_with(SimulationPage::RunSet, 1200.0, |app| {
        app.state.sim_setup.run_set.dimensions[2].enabled = true;
        let id = app.state.sim_setup.run_set.dimensions[2].id.clone();
        dispatch(
            &mut app.state.sim_setup.run_set,
            RunSetAction::SetValues {
                id,
                text: "-40\nwarm\n125".to_owned(),
            },
            1,
        );
    });

    assert!(
        rendered.contains("Temperature \u{b7} warm is not a temperature"),
        "the refusal must lead with the axis it is about:\n{rendered}"
    );
    assert!(
        !rendered.contains("RUNSET-"),
        "a stable code belongs to the receipt and the log, not to the page:\n{rendered}"
    );
    assert!(
        rendered.contains("warm"),
        "the invalid value must stay visible:\n{rendered}"
    );
    assert!(
        rendered.contains("Run-set preview blocked"),
        "the page must say that nothing was dispatched:\n{rendered}"
    );
}

/// An advisory states what the space is doing. Our own identifier for it is not
/// a code a reader can look up, and the band head cannot count rows the reader
/// is already looking at.
#[test]
fn the_advisory_band_states_the_advice_without_a_code_or_a_count() {
    let rendered = render(SimulationPage::RunSet, 1200.0);

    assert!(
        rendered.contains("No process axis is enabled"),
        "the advisory itself must reach the page:\n{rendered}"
    );
    assert!(
        rendered.contains("Run-set advisories\n"),
        "the band is named once, with nothing after it:\n{rendered}"
    );
    assert!(
        !rendered.contains("RUNSET-"),
        "a stable code belongs to the receipt and the log, not to the page:\n{rendered}"
    );
    assert!(
        !rendered.contains("notes"),
        "the rows underneath are the count:\n{rendered}"
    );
}

/// The composition card names the mode and what that mode costs this plan. It
/// does not restate the mode's own definition, describe the manifest format, or
/// explain the invalid-value policy the dropdown beside it already states.
#[test]
fn the_composition_card_restates_no_rule_its_own_controls_already_name() {
    let rendered = render(SimulationPage::RunSet, 1200.0);

    assert!(
        rendered.contains("Cartesian \u{b7} every combination"),
        "the mode is still named, by the control that chooses it:\n{rendered}"
    );
    for lecture in [
        "Full cross product",
        "Point identity",
        "run manifest",
        "Nothing is corrected, rounded or dropped",
    ] {
        assert!(
            !rendered.contains(lecture),
            "{lecture:?} explains the implementation of a control on this page:\n{rendered}"
        );
    }
    // Statistical variation has one owner, and it is the run-space strip: the
    // strip carries the control that adds the analysis, so a second statement
    // of the same state in this card could only send the reader back up to it.
    assert_eq!(
        rendered.matches("Monte Carlo").count(),
        1,
        "statistical variation is stated twice:\n{rendered}"
    );
    assert!(
        !rendered.contains("Statistical variation"),
        "the composition card restates what the run-space strip owns:\n{rendered}"
    );
    // Nothing is excluded in the fixture, and a row whose whole content is
    // that nothing happened does not render.
    assert!(
        !rendered.contains("Excluded points"),
        "an exclusion row with no exclusions to report:\n{rendered}"
    );
}

/// A run space that does exclude points still says so, in the card that owns
/// the exclusion rule.
///
/// The row above is deleted in its normal state and not deleted outright, so
/// this is the half that has to keep working: the pair of them is the content
/// rule rather than one assertion that the string is gone.
#[test]
fn the_composition_card_reports_exclusions_when_the_run_space_has_them() {
    let rendered = render_with(SimulationPage::RunSet, 1200.0, |app| {
        for dimension in &mut app.state.sim_setup.run_set.dimensions {
            dimension.enabled = matches!(dimension.kind, RunSetDimensionKind::ProcessSection);
        }
        app.state
            .sim_setup
            .run_set
            .composition
            .excluded_points
            .insert("process-section=SS".to_owned());
    });

    assert!(
        rendered.contains("Excluded points"),
        "an exclusion the composition is not applying must be stated:\n{rendered}"
    );
}

/// The dimension editor states what this axis is, not what its kind means.
///
/// The axis card above the editor already names the kind beside its value
/// type, and the Source authority field over the deleted row names the thing
/// the axis actually binds to. The row between them spelled out a definition
/// that is the same sentence for every plan.
#[test]
fn the_dimension_editor_does_not_define_the_kind_the_axis_card_names() {
    let rendered = render(SimulationPage::RunSet, 1200.0);

    assert!(
        rendered.contains("Selected dimension \u{b7} Process section"),
        "the editor still names the axis it is editing:\n{rendered}"
    );
    assert!(
        rendered.contains("model-library:process-sections"),
        "the authority the axis binds to is still stated:\n{rendered}"
    );
    assert!(
        !rendered.contains("Binds to"),
        "the editor defines the axis kind instead of stating this axis:\n{rendered}"
    );
}

/// The strip, the forecast and the point table read one declaration.
///
/// Before the run set they read three: the axis cards read the corner dialog's
/// fixed buffers, the forecast read `num_corners`, and the table read
/// `corner_names`. Disabling an axis is the cheapest way to prove they move
/// together.
#[test]
fn disabling_an_axis_moves_the_forecast_and_the_point_table_together() {
    let rendered = render_with(SimulationPage::RunSet, 1200.0, |app| {
        for dimension in &mut app.state.sim_setup.run_set.dimensions {
            dimension.enabled = matches!(
                dimension.kind,
                RunSetDimensionKind::ProcessSection
                    | RunSetDimensionKind::Supply
                    | RunSetDimensionKind::Temperature
            );
        }
        let id = app.state.sim_setup.run_set.dimensions[1].id.clone();
        dispatch(
            &mut app.state.sim_setup.run_set,
            RunSetAction::SetEnabled { id, enabled: false },
            1,
        );
    });

    assert!(
        rendered.contains('9'),
        "3 process × 3 temperature is nine points:\n{rendered}"
    );
    assert!(
        rendered.contains("9 points"),
        "the point table must report the same nine:\n{rendered}"
    );
}
