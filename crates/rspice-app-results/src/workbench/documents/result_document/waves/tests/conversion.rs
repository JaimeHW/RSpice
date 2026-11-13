//! Analog-to-digital and digital-to-analog views on the SAR converter's run
//! (C4, port-level digital): a `digital()` lane of its analog clock, its
//! `code` bus as an `analog()` staircase against the capacitor DAC, and both
//! conversions offered from the menus of the frame the reader sees.

use super::lanes::{
    ANALOG, BITS, FRAME, frame, history, host, sar_run, sar_strip, strip, theme_context,
};
use super::*;
use crate::state::ComplexExpressionPolicy;
use egui::epaint::ClippedShape;
use rspice_results::result_presentation::ExprTrace;
use rspice_results::visualization_document::{BusRadix, BusValueName};
use rspice_results_ui::waves::lanes::lane_specs;

const NS: f64 = 1e-9;
const CLOCK: &str = "digital(V(CLK), 0.9)";
/// The clock lane's name in the gutter and the cursor table.
const CLOCK_LANE: &str = "V(CLK) ≥ 0.9";

fn add_expression(state: &mut ResultsTestHost, key: AnalysisPresentationKey, text: &str) {
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

fn expressions(state: &ResultsTestHost, key: AnalysisPresentationKey) -> Vec<String> {
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
        texts(&shapes)
            .iter()
            .any(|(text, _)| text.starts_with("digital(")),
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
            models[0].analysis_index,
            "analog(D(code))",
            None,
            ComplexExpressionPolicy::Rectangular,
            lanes.as_ref(),
        )
        .expect("the code bus reads as a staircase");
        let x = outputs[0].waveform.data.x.as_slice();
        let y = outputs[0].waveform.data.y.as_slice();
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
            (waveform.x.to_vec(), waveform.y.to_vec())
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
fn texts(shapes: &[ClippedShape]) -> Vec<(String, egui::Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
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
fn click(
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
fn find(ctx: &egui::Context, state: &mut ResultsTestHost, wanted: &str) -> egui::Pos2 {
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

/// The pictures for review, written under `RSPICE_PICTURES` when it is set:
/// the analog clock with its `digital()` lane, the `code` bus as a staircase
/// beside the DAC's top plate, the bus with named words, and the octal and
/// ASCII radixes.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_conversion_pictures() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let run = sar_run();
    let eoc = history(run, "EOC");
    let samp = history(run, "SAMP");
    // The first conversion ends where eoc first rises.
    let done = eoc
        .iter()
        .find(|(_, code)| *code == 1)
        .expect("the first conversion ends")
        .0;
    let write = |name: &str, state: &mut ResultsTestHost| {
        let canvas = rspice_ui_kit::raster::render(FRAME, |ui, _| {
            super::lanes::well(ui);
            show(ui, &mut state.results_parts());
        });
        std::fs::write(
            format!("{directory}/{name}.png"),
            canvas.png(canvas.height()),
        )
        .expect("the picture is written");
    };

    // An analog clock and its digital() lane, with a hysteresis band.
    let mut state = host(run, &["CLK"]);
    let key = strip(&mut state)[0].analysis_key;
    add_expression(&mut state, key, CLOCK);
    add_expression(&mut state, key, "digital(V(CLK), 1.2, 0.6, 0.2n)");
    super::lanes::place_cursors(&mut state, Some(150.5 * NS), Some(250.5 * NS));
    write("digital-clock", &mut state);
    // One rising edge up close, cursors on the two thresholds: the
    // hysteresis lane holds 0 for its time to X after V(CLK) enters the band
    // at 0.6 V, reads X, and goes high where V(CLK) crosses 1.2 V.
    set_shared_x_view(
        &mut state.results.results.session,
        key,
        1,
        Some((99.9 * NS, 101.1 * NS)),
    );
    super::lanes::place_cursors(
        &mut state,
        Some((100.0 + 1.0 / 3.0) * NS),
        Some((100.0 + 2.0 / 3.0) * NS),
    );
    write("digital-clock-edge", &mut state);

    // The code bus as a staircase beside the DAC's top plate.
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, "analog(D(code))");
    super::lanes::place_cursors(
        &mut state,
        Some(samp[1].0),
        Some(eoc[eoc.len() - 1].0 + 1e-9),
    );
    write("analog-code-dac", &mut state);

    // A bus with named words.
    let (mut state, key) = sar_strip(BusRadix::Hex, false);
    let result: Vec<Option<u8>> = BITS
        .iter()
        .map(|net| {
            history(run, net)
                .iter()
                .rev()
                .find(|(time, _)| *time <= done)
                .map(|(_, code)| *code)
        })
        .collect();
    let result = rspice_results::events::projection::word_unsigned(&result)
        .expect("the first conversion ends on a resolved code");
    named(&mut state, key, result);
    super::lanes::place_cursors(&mut state, Some(done + 1e-9), None);
    write("value-names", &mut state);

    // The octal and ASCII radixes.
    for (name, radix) in [
        ("radix-octal", BusRadix::Octal),
        ("radix-ascii", BusRadix::Ascii),
    ] {
        let (mut state, _) = sar_strip(radix, false);
        super::lanes::place_cursors(&mut state, Some(done + 1e-9), None);
        write(name, &mut state);
    }
}

/// Names for the code words a conversion starts from and tries first, and
/// the one it ends on (`result`).
fn named(state: &mut ResultsTestHost, key: AnalysisPresentationKey, result: u64) {
    let mut names: Vec<BusValueName> =
        [(0x00, "ZERO"), (0x01, "LSB"), (0x40, "QTR"), (0x80, "MID")]
            .into_iter()
            .map(|(value, name)| BusValueName {
                value,
                name: name.to_owned(),
            })
            .collect();
    if !names.iter().any(|name| name.value == result) {
        names.push(BusValueName {
            value: result,
            name: "RESULT".to_owned(),
        });
        names.sort_by_key(|name| name.value);
    }
    let layout = state
        .results
        .results
        .session
        .event_lanes
        .get_mut(&key)
        .expect("the strip has lanes");
    layout.value_names.insert("code".to_owned(), names);
}
