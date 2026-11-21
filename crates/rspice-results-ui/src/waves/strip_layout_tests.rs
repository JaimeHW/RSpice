//! What the strip menus and drags make of a transient's waveform stack.
use super::*;
use crate::derived::DerivedSeries;
use crate::waves::ProjectionOptions;
use rspice_results::{
    analysis_result::AnalysisResult, analysis_type::AnalysisType,
    executed_deck::ExecutedDeckArchive, waveform::RetainedTrace,
};
use rspice_ui_kit::tokens::Tokens;
use std::collections::HashSet;

/// A transient over 1 ms whose columns are `columns`, each a name, a unit
/// and the values it holds over three samples.
fn model(columns: &[(&str, &str, [f64; 3])]) -> StripModel {
    let grid = vec![0.0, 0.5e-3, 1.0e-3];
    let mut run = rspice_results::run::SimulationRun::new(
        1,
        0.0,
        rspice_results::run::ExecutionTarget::LocalDesktop,
    );
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran", 0.0).with_waveforms(
            columns
                .iter()
                .map(|(name, unit, values)| {
                    RetainedTrace::new(*name, grid.clone(), values.to_vec(), "#fff")
                        .with_unit(*unit)
                })
                .collect(),
        ),
    );
    let mut models = crate::waves::build_models(
        &[&run],
        &ExecutedDeckArchive::default(),
        &mut DerivedSeries::default(),
        &Tokens::default(),
        ProjectionOptions {
            phase_continuous: false,
            complex_display: Default::default(),
            ac: Default::default(),
            selection: None,
            hidden_family_traces: &HashSet::new(),
        },
        |_| None,
    );
    crate::waves::extent::resolve_x_ranges(&mut models);
    models.remove(0)
}

/// A 5 V rail and the 2 mV ripple on it, both volts.
fn rail_and_ripple() -> StripModel {
    model(&[
        ("V(rail)", "V", [5.0, 5.0, 5.0]),
        ("V(ripple)", "V", [0.0, 2.0e-3, -2.0e-3]),
    ])
}

fn trace(model: &StripModel, name: &str) -> Curve {
    Curve::Trace(
        model
            .traces
            .iter()
            .position(|trace| trace.name == name)
            .expect("the trace"),
    )
}

/// The trace names each strip draws, axis by axis.
fn drawn(model: &StripModel) -> Vec<Vec<Vec<String>>> {
    let name = |index: &usize| model.traces[*index].name.clone();
    model
        .unit_panes()
        .iter()
        .map(|pane| {
            std::iter::once(pane.traces.iter().map(name).collect())
                .chain(
                    pane.secondary
                        .iter()
                        .map(|axis| axis.traces.iter().map(name).collect()),
                )
                .collect()
        })
        .collect()
}

#[test]
fn one_drag_takes_a_ripple_off_its_rail_onto_a_strip_of_its_own() {
    let mut model = rail_and_ripple();
    assert_eq!(drawn(&model), [[["V(rail)", "V(ripple)"]]]);

    let ripple = trace(&model, "V(ripple)");
    model.arrangement = move_to_gap(&model, ripple, 1).expect("a gap takes the ripple");

    assert_eq!(drawn(&model), [[["V(rail)"]], [["V(ripple)"]]]);
    let panes = model.unit_panes();
    assert_ne!(
        panes[0].slot, panes[1].slot,
        "the two strips keep their zoom apart"
    );
}

#[test]
fn a_ripple_given_an_axis_of_its_own_stays_on_its_rails_strip() {
    let mut model = rail_and_ripple();
    let ripple = trace(&model, "V(ripple)");
    model.arrangement = move_to_new_axis(&model, ripple).expect("a second volts axis");

    assert_eq!(drawn(&model), [[vec!["V(rail)"], vec!["V(ripple)"]]]);
    let panes = model.unit_panes();
    assert_eq!(panes[0].secondary[0].unit, "V");
    assert_ne!(
        panes[0].slot, panes[0].secondary[0].slot,
        "each volts axis keeps its own range"
    );
    let back = move_to_axis(&model, ripple, 0).expect("back onto the rail's axis");
    model.arrangement = back;
    assert_eq!(drawn(&model), [[["V(rail)", "V(ripple)"]]]);
}

#[test]
fn a_current_dropped_on_a_voltage_strip_reads_against_an_axis_of_its_own() {
    let mut model = model(&[
        ("V(out)", "V", [0.0, 1.0, 2.0]),
        ("I(load)", "A", [0.0, 1.0e-3, 2.0e-3]),
    ]);
    assert_eq!(drawn(&model), [[["V(out)"]], [["I(load)"]]]);
    let current = trace(&model, "I(load)");

    model.arrangement = move_to_strip(&model, current, 0).expect("the volts strip takes amps");

    assert_eq!(drawn(&model), [[vec!["V(out)"], vec!["I(load)"]]]);
    let panes = model.unit_panes();
    assert_eq!(panes[0].axis, "V");
    assert_eq!(panes[0].secondary[0].unit, "A");
    assert_ne!(panes[0].slot, panes[0].secondary[0].slot);
}

/// Five columns of five units.
const FIVE_UNITS: [(&str, &str, [f64; 3]); 5] = [
    ("V(a)", "V", [0.0; 3]),
    ("I(b)", "A", [0.0; 3]),
    ("P(c)", "W", [0.0; 3]),
    ("R(d)", "\u{3a9}", [0.0; 3]),
    ("T(e)", "K", [0.0; 3]),
];

#[test]
fn a_strip_refuses_a_fifth_unit_and_says_why() {
    let stack = model(&FIVE_UNITS);
    assert_eq!(stack.unit_panes().len(), 5);

    let reason = combine_all_strips(&stack).expect_err("five units on one strip");
    assert!(reason.contains("four Y axes"), "{reason}");
    assert!(reason.contains("K curve"), "{reason}");

    let mut four = model(&FIVE_UNITS);
    four.arrangement = combine_first_four(&stack);
    assert_eq!(four.unit_panes().len(), 2);
    let fifth = trace(&four, "T(e)");
    let reason = move_to_strip(&four, fifth, 0).expect_err("a fifth axis");
    assert!(reason.contains("four Y axes"), "{reason}");
}

/// The first four strips merged into one, the fifth left alone.
fn combine_first_four(model: &StripModel) -> Vec<Strip> {
    let mut strips = layout(model);
    let rest = strips.split_off(1);
    for strip in rest.iter().take(3) {
        for axis in &strip.axes {
            strips[0].axes.push(axis.clone());
        }
    }
    strips.extend(rest.into_iter().skip(3));
    strips
}

#[test]
fn a_pinned_range_stays_with_its_strip_when_a_strip_above_it_goes() {
    let mut model = model(&[
        ("V(a)", "V", [0.0; 3]),
        ("V(b)", "V", [0.0; 3]),
        ("V(c)", "V", [0.0; 3]),
    ]);
    model.arrangement = split_all_strips(&model).expect("three strips");
    let slot_of_c = model.unit_panes()[2].slot;

    let b = model
        .traces
        .iter()
        .position(|trace| trace.name == "V(b)")
        .expect("V(b)");
    model.traces[b].visible = false;

    let panes = model.unit_panes();
    assert_eq!(panes.len(), 2, "a strip with nothing to draw takes no room");
    assert_eq!(
        panes[1].slot, slot_of_c,
        "the third strip's range is kept by what it is, not where it stands"
    );
}

#[test]
fn split_and_merge_undo_each_other() {
    let mut model = rail_and_ripple();
    model.arrangement = split_strip(&model, 0).expect("two curves split");
    assert_eq!(drawn(&model), [[["V(rail)"]], [["V(ripple)"]]]);
    assert!(split_strip(&model, 0).is_err(), "one curve does not split");

    model.arrangement = merge_with_strip_above(&model, 1).expect("the ripple joins the rail");
    assert_eq!(drawn(&model), [[["V(rail)", "V(ripple)"]]]);
    assert!(merge_with_strip_above(&model, 0).is_err());

    model.arrangement = strip_by_unit(&model).expect("back to a strip per unit");
    assert!(model.arrangement.is_empty());
    assert!(strip_by_unit(&model).is_err());
}

#[test]
fn a_copy_draws_the_curve_on_both_strips() {
    let mut model = rail_and_ripple();
    let ripple = trace(&model, "V(ripple)");
    model.arrangement = copy_to_new_strip(&model, ripple).expect("a copy");
    assert_eq!(
        drawn(&model),
        [vec![vec!["V(rail)", "V(ripple)"]], vec![vec!["V(ripple)"]]]
    );
}

#[test]
fn a_hidden_curve_comes_back_to_the_strip_that_listed_it() {
    let mut model = model(&[
        ("V(a)", "V", [0.0; 3]),
        ("V(b)", "V", [0.0; 3]),
        ("V(c)", "V", [0.0; 3]),
    ]);
    let c = trace(&model, "V(c)");
    model.arrangement = move_to_new_strip(&model, c).expect("V(c) apart");
    let Curve::Trace(index) = c else {
        unreachable!()
    };
    model.traces[index].visible = false;
    // Another edit while V(c) is hidden writes the strips down again.
    let a = trace(&model, "V(a)");
    model.arrangement = move_to_new_strip(&model, a).expect("V(a) apart");
    model.traces[index].visible = true;
    assert_eq!(drawn(&model), [[["V(b)"]], [["V(a)"]], [["V(c)"]]]);
}

#[test]
fn a_locked_strip_keeps_its_height() {
    let mut model = rail_and_ripple();
    model.arrangement = split_strip(&model, 0).expect("two strips");
    model.arrangement = resize(&model, 0, (3.0, 1.0)).expect("a divider drag");
    let weights: Vec<f32> = model.unit_panes().iter().map(|pane| pane.weight).collect();
    assert!(
        (weights[0] - 1.5).abs() < 1e-6 && (weights[1] - 0.5).abs() < 1e-6,
        "{weights:?}"
    );

    model.arrangement = set_locked(&model, 0, true).expect("lock");
    assert!(resize(&model, 0, (1.0, 1.0)).is_err());
    model.arrangement = equalise(&model).expect("the unlocked strip evens out");
    let weights: Vec<f32> = model.unit_panes().iter().map(|pane| pane.weight).collect();
    assert!(
        (weights[0] - 1.5).abs() < 1e-6 && (weights[1] - 1.0).abs() < 1e-6,
        "{weights:?}"
    );
}

#[test]
fn a_header_drag_reorders_the_strips() {
    let mut model = model(&[("V(a)", "V", [0.0; 3]), ("I(b)", "A", [0.0; 3])]);
    model.arrangement = reorder(&model, 1, 0).expect("amps above volts");
    assert_eq!(drawn(&model), [[["I(b)"]], [["V(a)"]]]);
}
