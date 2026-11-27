//! Analog-to-digital and digital-to-analog views on the SAR converter's run
//! (C4, port-level digital): a `digital()` lane of its analog clock, its
//! `code` bus as an `analog()` staircase against the capacitor DAC, and both
//! conversions offered from the menus of the frame the reader sees.

use super::lanes::{ANALOG, BITS, frame, history, host, sar_run, sar_strip, strip, theme_context};
use super::*;
use crate::state::ComplexExpressionPolicy;
use egui::epaint::ClippedShape;
use rspice_results::result_presentation::ExprTrace;
use rspice_results::visualization_document::BusRadix;
use rspice_results_ui::waves::lanes::lane_specs;

const NS: f64 = 1e-9;
const CLOCK: &str = "digital(V(CLK), 0.9)";
/// The clock lane's name in the gutter and the cursor table.
const CLOCK_LANE: &str = "V(CLK) ≥ 0.9";

pub(super) fn add_expression(
    state: &mut ResultsTestHost,
    key: AnalysisPresentationKey,
    text: &str,
) {
    state
        .results
        .results
        .session
        .analysis_exprs
        .entry(key)
        .or_default()
        .push(ExprTrace {
            text: text.to_owned(),
            complex_policy: ComplexExpressionPolicy::Rectangular,
            visible: true,
        });
}

pub(super) fn expressions(state: &ResultsTestHost, key: AnalysisPresentationKey) -> Vec<String> {
    state
        .results
        .results
        .session
        .analysis_exprs
        .get(&key)
        .map(|exprs| exprs.iter().map(|expr| expr.text.clone()).collect())
        .unwrap_or_default()
}

/// The clock source is `pulse(0 1.8 0 1n 1n 49n 100n)`: it crosses 0.9 V
/// halfway up each 1 ns rise and halfway down each 1 ns fall, at
/// 0.5 + 100k ns and 50.5 + 100k ns. The lane's edges sit there, between
/// the solver's samples at the ramp's corners, not on them.
#[test]
fn a_digital_expression_is_a_logic_lane_with_its_edges_on_the_threshold() {
    let mut state = host(sar_run(), &ANALOG);
    let key = strip(&mut state)[0].analysis_key;
    add_expression(&mut state, key, CLOCK);
    let models = strip(&mut state);
    let model = &models[0];
    let lanes = model.event_lanes.as_deref().expect("the strip has lanes");
    let lane = lanes
        .logic(CLOCK_LANE)
        .expect("the expression is a logic lane");
    let levels: String = lane.levels().iter().map(|level| level.symbol()).collect();
    assert!(levels.starts_with("0101"), "{levels}");
    let end = 2.4e-6;
    let mut expected = Vec::new();
    for k in 0..24 {
        let period = f64::from(k) * 100.0 * NS;
        expected.push(period + 0.5 * NS);
        expected.push(period + 50.5 * NS);
    }
    expected.retain(|t| *t < end);
    // The lane starts low at the first sample, then changes at each crossing.
    assert_eq!(lane.times().len(), expected.len() + 1);
    let worst = lane.times()[1..]
        .iter()
        .zip(&expected)
        .map(|(t, want)| (t - want).abs())
        .fold(0.0, f64::max);
    assert!(worst < 1e-15, "an edge is {worst} s off its crossing");

    // It is a row of the lane stack, and no value trace of a pane.
    let labels: Vec<String> = lane_specs(model, None)
        .iter()
        .map(|spec| spec.label().to_owned())
        .collect();
    assert_eq!(labels, [CLOCK_LANE]);
    let resolved = super::super::expressions::resolve_strip_exprs(
        &mut state.results_parts(),
        model,
        &Tokens::default(),
    );
    assert!(resolved.is_empty(), "a digital() lane is drawn as no curve");

    // The frame draws the row: the panes move down by one logic row.
    let ctx = theme_context();
    let shapes = frame(&ctx, &mut state, Vec::new());
    let shapes = if shapes.is_empty() {
        frame(&ctx, &mut state, Vec::new())
    } else {
        shapes
    };
    assert!(
        texts(&shapes).iter().any(|(text, _)| text == CLOCK_LANE),
        "the gutter names the lane"
    );
}

/// The code bus as a staircase is the DAC's input code: at every settled
/// trial it equals the netlist's own code probe, `bcode`, which reads the
/// DAC's bit nodes at half the rail (net/sar_analog.cir). The top plate
/// during conversion is `-vin + code·vref/256` with vref = 1 V and vin the
/// input sampled when samp fell, so `V(TOP) + vin` follows the staircase
/// over 256 at every trial.
#[test]
fn the_code_bus_staircase_follows_the_dac() {
    let run = sar_run();
    for radix in [BusRadix::Unsigned, BusRadix::Signed] {
        let (mut state, key) = sar_strip(radix, false);
        let models = strip(&mut state);
        let lanes = state.results.results.session.event_lanes.get(&key).cloned();
        let outputs = super::super::evaluate_expression_reading(
            &state.model.simulation,
            (models[0].analysis_index, None),
            "analog(D(code))",
            None,
            ComplexExpressionPolicy::Rectangular,
            lanes.as_ref(),
            &super::super::Places::default(),
        )
        .expect("the code bus reads as a staircase");
        let x = outputs[0].waveform.data.x.read_whole().unwrap();
        let y = outputs[0].waveform.data.y.read_whole().unwrap();
        let held = |t: f64| {
            let index = x.partition_point(|time| *time <= t);
            y[index - 1]
        };
        // The staircase equals the bus word at every event.
        let mut events: Vec<f64> = BITS
            .iter()
            .flat_map(|net| history(run, net).into_iter().map(|(t, _)| t))
            .collect();
        events.sort_by(f64::total_cmp);
        events.dedup();
        for t in &events {
            let codes: Vec<Option<u8>> = BITS
                .iter()
                .map(|net| {
                    history(run, net)
                        .iter()
                        .rev()
                        .find(|(time, _)| time <= t)
                        .map(|(_, code)| *code)
                })
                .collect();
            let word = rspice_results::events::projection::word_unsigned(&codes);
            let want = word.map_or(f64::NAN, |word| {
                if radix == BusRadix::Signed && word >= 128 {
                    word as f64 - 256.0
                } else {
                    word as f64
                }
            });
            let got = held(*t);
            assert!(
                got == want || (got.is_nan() && want.is_nan()),
                "at {t}: {got} vs {want}"
            );
        }
        if radix == BusRadix::Signed {
            assert!(
                y.iter().any(|value| *value < 0.0),
                "a signed bus reads negative"
            );
            continue;
        }
        // During conversion each trial settles within the clock period:
        // read 90 ns after every clock edge while samp is low.
        let column = |name: &str| {
            let waveform = run.analyses[0]
                .waveforms
                .iter()
                .find(|waveform| waveform.name == name)
                .unwrap_or_else(|| panic!("the run has {name}"));
            (
                waveform.x.read_whole().unwrap().to_vec(),
                waveform.y.read_whole().unwrap().to_vec(),
            )
        };
        let (tx, ty) = column("TOP");
        let (vx, vy) = column("VIN");
        let at = |xs: &[f64], ys: &[f64], t: f64| {
            let i = xs.partition_point(|time| *time <= t).clamp(1, xs.len() - 1);
            let f = (t - xs[i - 1]) / (xs[i] - xs[i - 1]);
            ys[i - 1] + f * (ys[i] - ys[i - 1])
        };
        // The top plate floats when samp falls, holding the charge of the vin
        // it sampled; the deck steps vin to the next conversion's input half
        // way through this one (net/vin_codes.cir), so the DAC is read
        // against the sampled vin, not the live one.
        let samp = history(run, "SAMP");
        let sampled_at = |t: f64| match samp.iter().rev().find(|(time, _)| *time <= t) {
            Some((time, 0)) => Some(*time),
            _ => None,
        };
        let (cx, cy) = column("CODE");
        let mut worst: f64 = 0.0;
        let mut compared = 0;
        for k in 0..24 {
            let t = f64::from(k).mul_add(100.0 * NS, 90.5 * NS);
            let Some(fell) = sampled_at(t) else {
                continue;
            };
            if held(t).is_nan() {
                continue;
            }
            let dac = at(&tx, &ty, t) + at(&vx, &vy, fell);
            let code = at(&cx, &cy, t);
            assert!(
                (code - held(t)).abs() < 1e-9,
                "at {t}: the code probe reads {code}, the staircase {}",
                held(t)
            );
            worst = worst.max((dac - held(t) / 256.0).abs());
            compared += 1;
        }
        assert!(compared >= 16, "{compared} settled trials");
        assert!(worst < 1e-6, "the DAC is {worst} V off code/256");
    }
}

/// Every run of text a frame painted, with where it was laid out.
pub(super) fn texts(shapes: &[ClippedShape]) -> Vec<(String, egui::Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                text.galley.rect.translate(text.pos.to_vec2()),
            )),
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

fn press(at: egui::Pos2, button: egui::PointerButton, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: at,
        button,
        pressed,
        modifiers: egui::Modifiers::default(),
    }
}

/// Click `button` on `at` across the frames egui needs to see it.
pub(super) fn click(
    ctx: &egui::Context,
    state: &mut ResultsTestHost,
    at: egui::Pos2,
    button: egui::PointerButton,
) {
    frame(ctx, state, vec![egui::Event::PointerMoved(at)]);
    frame(ctx, state, vec![press(at, button, true)]);
    frame(ctx, state, vec![press(at, button, false)]);
}

/// Where the frame painted `wanted`, after the frames it takes to settle.
pub(super) fn find(ctx: &egui::Context, state: &mut ResultsTestHost, wanted: &str) -> egui::Pos2 {
    let shapes = frame(ctx, state, Vec::new());
    texts(&shapes)
        .into_iter()
        .rev()
        .find(|(text, _)| text == wanted)
        .map(|(_, rect)| rect.center())
        .unwrap_or_else(|| panic!("the frame paints {wanted:?}"))
}

/// The `code` bus row's menu offers Analog, which adds its staircase to the
/// strip as `analog(D(code))`.
#[test]
fn a_lanes_menu_draws_it_as_a_staircase() {
    let (mut state, key) = sar_strip(BusRadix::Hex, false);
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let row = find(&ctx, &mut state, "code");
    click(&ctx, &mut state, row, egui::PointerButton::Secondary);
    let item = find(&ctx, &mut state, "Analog");
    click(&ctx, &mut state, item, egui::PointerButton::Primary);
    assert_eq!(expressions(&state, key), ["analog(D(code))"]);
}

/// A trace's menu offers Digital, with its threshold starting at the middle
/// of the trace's range; adding it puts a `digital()` lane on the strip.
#[test]
fn a_traces_menu_reads_it_as_logic() {
    let mut state = host(sar_run(), &["CLK"]);
    let key = strip(&mut state)[0].analysis_key;
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let chip = find(&ctx, &mut state, "CLK");
    click(&ctx, &mut state, chip, egui::PointerButton::Secondary);
    let digital = find(&ctx, &mut state, "Digital");
    frame(&ctx, &mut state, vec![egui::Event::PointerMoved(digital)]);
    click(&ctx, &mut state, digital, egui::PointerButton::Primary);
    let add = find(&ctx, &mut state, "Add digital lane");
    click(&ctx, &mut state, add, egui::PointerButton::Primary);
    assert_eq!(expressions(&state, key), [CLOCK]);
    let models = strip(&mut state);
    assert!(
        models[0]
            .event_lanes
            .as_deref()
            .is_some_and(|lanes| lanes.logic(CLOCK_LANE).is_some()),
        "the strip draws the new lane"
    );
}

/// The staircase is the bus word's integer value, a plain number: it opens a
/// pane of its own instead of borrowing the volts axis of VIN and TOP or
/// being flagged as a value with no unit, and the `digital()` lane beside it
/// is drawn on no pane.
#[test]
fn the_staircase_has_a_pane_of_its_own() {
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, "analog(D(code))");
    add_expression(&mut state, key, CLOCK);
    let ctx = theme_context();
    // The first frame evaluates the expressions, the second routes them with
    // what evaluation found.
    frame(&ctx, &mut state, Vec::new());
    frame(&ctx, &mut state, Vec::new());
    let models = strip(&mut state);
    let panes: Vec<(String, bool, usize, Vec<usize>)> = models[0]
        .unit_panes()
        .iter()
        .map(|pane| {
            (
                pane.axis_unit().to_owned(),
                pane.unlabeled,
                pane.traces.len(),
                pane.expressions.clone(),
            )
        })
        .collect();
    assert!(
        panes.contains(&("V".to_owned(), false, 2, vec![])),
        "VIN and TOP keep the volts pane to themselves: {panes:?}"
    );
    let staircase: Vec<_> = panes
        .iter()
        .filter(|(.., expressions)| !expressions.is_empty())
        .collect();
    assert_eq!(
        staircase,
        [&(String::new(), false, 0, vec![0])],
        "only the staircase is on a pane, and it is a plain number's"
    );
}
