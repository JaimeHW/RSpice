//! The pane legend on the SAR converter's run (C4), judged through the
//! frame the reader sees: every control it draws says what it is, and every
//! curve a pane draws is named in that pane, an expression's as a signal's.

use super::conversion::{add_expression, expressions, texts};
use super::lanes::{FRAME, sar_strip, theme_context};
use super::*;
use egui::FullOutput;
use egui::accesskit::Role;
use rspice_results::visualization_document::BusRadix;

/// One frame of the Waves sheet, as the reader's window lays it out.
fn run(ctx: &egui::Context, state: &mut ResultsTestHost, events: Vec<egui::Event>) -> FullOutput {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
        events,
        ..Default::default()
    };
    ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
    })
}

/// A settled frame, the pointer off the strip so no hover readout covers
/// it: evaluation, routing and sizing take a pass each.
fn settled(ctx: &egui::Context, state: &mut ResultsTestHost) -> FullOutput {
    ctx.enable_accesskit();
    run(ctx, state, vec![egui::Event::PointerGone]);
    run(ctx, state, Vec::new());
    run(ctx, state, Vec::new())
}

/// Every control the frame publishes, by its role, its name and where it is.
fn controls(output: &FullOutput) -> Vec<(Role, Option<String>, Option<egui::Rect>)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .map(|update| {
            update
                .nodes
                .iter()
                .map(|(_, node)| {
                    let rect = node.bounds().map(|bounds| {
                        egui::Rect::from_min_max(
                            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                        )
                    });
                    (node.role(), node.label().map(str::to_owned), rect)
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Where the control named `name` is.
fn control(output: &FullOutput, name: &str) -> egui::Rect {
    controls(output)
        .into_iter()
        .find_map(|(_, label, rect)| (label.as_deref() == Some(name)).then_some(rect).flatten())
        .unwrap_or_else(|| panic!("the frame publishes {name:?}"))
}

/// Where the last run of text that starts with `wanted` was painted.
fn text(output: &FullOutput, wanted: &str) -> Option<egui::Rect> {
    texts(&output.shapes)
        .into_iter()
        .rev()
        .find(|(text, _)| text.starts_with(wanted))
        .map(|(_, rect)| rect)
}

/// The volts pane's header row: the topmost place its first signal is named.
fn volts_row(output: &FullOutput) -> Option<egui::Rect> {
    texts(&output.shapes)
        .into_iter()
        .filter(|(text, _)| text.starts_with("VIN"))
        .map(|(_, rect)| rect)
        .min_by(|a, b| a.top().total_cmp(&b.top()))
}

/// Where the last run of text that reads `wanted` exactly was painted.
fn exact(output: &FullOutput, wanted: &str) -> Option<egui::Rect> {
    texts(&output.shapes)
        .into_iter()
        .rev()
        .find(|(text, _)| text == wanted)
        .map(|(_, rect)| rect)
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
    run(ctx, state, vec![egui::Event::PointerMoved(at)]);
    run(ctx, state, vec![press(at, button, true)]);
    run(ctx, state, vec![press(at, button, false)]);
}

/// The `analog()` staircase is a plain number, and its pane names no unit:
/// its axis goes unmarked while the volts axis states its unit, and no
/// control is drawn with nothing on it.
#[test]
fn a_plain_numbers_pane_draws_no_blank_control() {
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, "analog(D(code))");
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let controls = controls(&output);
    let blank: Vec<_> = controls
        .iter()
        .filter(|(role, label, _)| {
            *role == Role::Button && label.as_deref().is_none_or(|label| label.trim().is_empty())
        })
        .collect();
    assert!(
        text(&output, "V").is_some(),
        "the volts pane's axis keeps its unit"
    );
    assert!(blank.is_empty(), "a button says nothing: {blank:?}");
}

/// An expression on a pane of signals hides from its entry's swatch as a
/// signal does, then waits in the pane's trace list, and ticking it
/// there draws it again.
#[test]
fn an_expression_hides_and_shows_from_its_pane_legend() {
    const DOUBLE: &str = "V(TOP)*2";
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, DOUBLE);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    assert!(
        text(&output, DOUBLE).is_some_and(|entry| {
            volts_row(&output).is_some_and(|row| (entry.center().y - row.center().y).abs() < 4.0)
        }),
        "the volts expression is named on the volts pane's row"
    );
    let swatch = control(&output, &format!("Toggle {DOUBLE} visibility"));
    click(
        &ctx,
        &mut state,
        swatch.center(),
        egui::PointerButton::Primary,
    );
    let visible =
        |state: &ResultsTestHost| state.results.results.session.analysis_exprs[&key][0].visible;
    assert!(!visible(&state), "the swatch hides the expression");
    let output = settled(&ctx, &mut state);
    let hidden = text(&output, "1 hidden").expect("the pane counts the hidden expression");
    click(
        &ctx,
        &mut state,
        hidden.center(),
        egui::PointerButton::Primary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    let item = text(&menu, DOUBLE).expect("the trace list lists the expression");
    click(
        &ctx,
        &mut state,
        item.center(),
        egui::PointerButton::Primary,
    );
    assert!(visible(&state), "ticking it draws it again");
    assert_eq!(expressions(&state, key), [DOUBLE], "nothing else changed");
}

/// An expression's entry offers what a signal's does: Digital, which reads
/// the expression as typed, and Remove, which takes it off the strip.
#[test]
fn an_expressions_entry_converts_and_removes_it() {
    const DOUBLE: &str = "V(TOP)*2";
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, DOUBLE);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let entry = text(&output, DOUBLE).expect("the expression is named");
    click(
        &ctx,
        &mut state,
        entry.center(),
        egui::PointerButton::Secondary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    let digital = text(&menu, "Digital").expect("the entry's menu offers Digital");
    run(
        &ctx,
        &mut state,
        vec![egui::Event::PointerMoved(digital.center())],
    );
    click(
        &ctx,
        &mut state,
        digital.center(),
        egui::PointerButton::Primary,
    );
    let form = run(&ctx, &mut state, Vec::new());
    let add = text(&form, "Add digital lane").expect("Digital opens its threshold form");
    click(&ctx, &mut state, add.center(), egui::PointerButton::Primary);
    let added = expressions(&state, key);
    assert!(
        added.len() == 2 && added[1].starts_with(&format!("digital({DOUBLE}, ")),
        "Digital reads the expression as typed: {added:?}"
    );

    let output = settled(&ctx, &mut state);
    // The digital lane's name in the gutter starts with the expression too.
    let entry = exact(&output, DOUBLE).expect("the expression is still named");
    click(
        &ctx,
        &mut state,
        entry.center(),
        egui::PointerButton::Secondary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    let remove = text(&menu, "Remove expression").expect("the entry's menu offers Remove");
    click(
        &ctx,
        &mut state,
        remove.center(),
        egui::PointerButton::Primary,
    );
    settled(&ctx, &mut state);
    assert_eq!(
        expressions(&state, key),
        [added[1].clone()],
        "Remove takes the expression off the strip and leaves its conversion"
    );
}

/// An expression is named once, as a signal is: in the legend of the pane
/// that draws it. The strip header names the analysis and offers to add an
/// expression, and names no curve.
#[test]
fn an_expression_is_named_once_in_its_panes_legend() {
    const DOUBLE: &str = "V(TOP)*2";
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, DOUBLE);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let named: Vec<egui::Rect> = texts(&output.shapes)
        .into_iter()
        .filter(|(text, _)| text == DOUBLE)
        .map(|(_, rect)| rect)
        .collect();
    let row = volts_row(&output).expect("the volts pane's header row");
    assert_eq!(named.len(), 1, "the expression is named once: {named:?}");
    assert!(
        (named[0].center().y - row.center().y).abs() < 2.0,
        "it is named on the volts pane's row"
    );
    assert!(
        text(&output, "+ Expression").is_some(),
        "the strip header still offers to add one"
    );
}

/// An expression that fails to evaluate draws nothing and opens no pane, so
/// the top pane's legend names it, says it failed and gives the evaluator's
/// reason on hover; its menu removes it.
#[test]
fn an_expression_that_fails_is_named_and_removable() {
    const BROKEN: &str = "V(NOWHERE)*2";
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, BROKEN);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let failed = format!("{BROKEN} \u{b7} failed");
    let entry = exact(&output, &failed).expect("the top pane names the failed expression");
    let row = volts_row(&output).expect("the volts pane's header row");
    assert!(
        (entry.center().y - row.center().y).abs() < 2.0,
        "the top pane's row names it"
    );
    click(
        &ctx,
        &mut state,
        entry.center(),
        egui::PointerButton::Secondary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    let remove = text(&menu, "Remove expression").expect("its menu offers Remove");
    click(
        &ctx,
        &mut state,
        remove.center(),
        egui::PointerButton::Primary,
    );
    assert!(
        expressions(&state, key).is_empty(),
        "Remove takes it off the strip"
    );
    let output = settled(&ctx, &mut state);
    assert!(
        exact(&output, &failed).is_none(),
        "and its entry goes with it"
    );
}

/// A `digital()` expression is named once, as the lane it draws, and the
/// lane's menu offers what the expression's entry would: Analog, which draws
/// it as a staircase on a plain number's pane, and Remove. It is no net, so
/// the menu offers no probe of one.
#[test]
fn a_logic_expressions_lane_converts_and_removes_it() {
    const CLOCK: &str = "digital(V(CLK), 0.9)";
    const LANE: &str = "V(CLK) \u{2265} 0.9";
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, CLOCK);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    assert!(
        exact(&output, CLOCK).is_none(),
        "no legend names the lane's expression again"
    );
    let lane = exact(&output, LANE).expect("the lane is named in the gutter");
    click(
        &ctx,
        &mut state,
        lane.center(),
        egui::PointerButton::Secondary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    assert!(
        text(&menu, "Digital").is_none(),
        "a logic expression is not read as logic again"
    );
    assert!(
        text(&menu, "Select net").is_none(),
        "an expression's lane is no net to probe"
    );
    assert!(
        text(&menu, "Remove expression").is_some(),
        "the lane's menu removes its expression"
    );
    let analog = text(&menu, "Analog").expect("the lane's menu offers Analog");
    click(
        &ctx,
        &mut state,
        analog.center(),
        egui::PointerButton::Primary,
    );
    let staircase = format!("analog({CLOCK})");
    assert_eq!(
        expressions(&state, key),
        [CLOCK.to_owned(), staircase.clone()]
    );
    let output = settled(&ctx, &mut state);
    assert_eq!(
        texts(&output.shapes)
            .iter()
            .filter(|(text, _)| text.starts_with("analog("))
            .count(),
        1,
        "the staircase is drawn and named once, in its pane"
    );
    let lane = exact(&output, LANE).expect("the lane is still named");
    click(
        &ctx,
        &mut state,
        lane.center(),
        egui::PointerButton::Secondary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    let remove = text(&menu, "Remove expression").expect("the lane's menu offers Remove");
    click(
        &ctx,
        &mut state,
        remove.center(),
        egui::PointerButton::Primary,
    );
    assert_eq!(
        expressions(&state, key),
        [staircase],
        "Remove takes the lane's expression off the strip and leaves its staircase"
    );
}

/// Pointing into a pane reads each expression it draws in the pane's unit,
/// as each of its signals reads: `V(TOP)*2` in volts, never a bare prefix.
#[test]
fn the_hover_readout_reads_an_expression_in_its_panes_unit() {
    const DOUBLE: &str = "V(TOP)*2";
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, DOUBLE);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let row = volts_row(&output).expect("the volts pane's header row");
    let at = egui::pos2(600.0, row.bottom() + 150.0);
    run(&ctx, &mut state, vec![egui::Event::PointerMoved(at)]);
    run(&ctx, &mut state, Vec::new());
    let output = run(&ctx, &mut state, Vec::new());
    let painted = texts(&output.shapes);
    let label = painted
        .iter()
        .filter(|(text, rect)| text == DOUBLE && rect.top() > row.bottom())
        .map(|(_, rect)| *rect)
        .next_back()
        .expect("the hover readout names the expression");
    let value = painted
        .iter()
        .find(|(_, rect)| {
            (rect.center().y - label.center().y).abs() < 2.0 && rect.left() > label.right()
        })
        .map(|(text, _)| text.clone())
        .expect("the hover readout reads the expression");
    assert!(value.ends_with('V'), "the reading is in volts: {value:?}");
}

/// A pane whose plotted traces hold no finite value has nothing to draw
/// either: it is its header, naming them, and no pane-sized well says to
/// enable a trace the reader has enabled.
#[test]
fn a_pane_whose_traces_hold_no_value_is_its_header_alone() {
    let time = vec![0.0, 1.0e-3, 2.0e-3, 3.0e-3];
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(out)", time.clone(), vec![0.0, 1.0, 0.5, 0.25], "#fff"),
            RetainedTrace::new("I(R1)", time, vec![f64::NAN; 4], "#fff"),
        ]),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let painted = texts(&output.shapes);
    assert!(
        !painted
            .iter()
            .any(|(text, _)| text == rspice_results_ui::waves::stack::NO_VISIBLE_TRACE),
        "no well asks for a trace that is enabled"
    );
    let header = exact(&output, "I(R1)").expect("the currents' row names its trace");
    let axis = text(&output, "TIME").expect("the shared axis labels itself");
    assert!(
        axis.top() - header.bottom() < rspice_results_ui::waves::header::WAVE_PANE_HEADER_HEIGHT,
        "the axis follows the row: row ends at {}, axis starts at {}",
        header.bottom(),
        axis.top()
    );
}

/// At an event's instant a staircase reads the value after the event, as
/// its lane does and as its curve is drawn from there on: with cursor A on
/// the second sampling edge, where the code steps from 0 to 128, the
/// `analog(D(code))` entry reads 128.0 at the reader's four digits, beside
/// the lane's 128.
#[test]
fn an_event_instant_reads_the_value_after_the_event() {
    const STAIRCASE: &str = "analog(D(code))";
    let samp = super::lanes::history(super::lanes::sar_run(), "SAMP");
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, STAIRCASE);
    super::lanes::place_cursors(&mut state, Some(samp[1].0), None);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let painted = texts(&output.shapes);
    let volts = volts_row(&output).expect("the volts pane's header row");
    // The texts of the last row that names `name` (below the volts pane's
    // header for a pane entry), from its name rightwards.
    let row = |name: &str, in_pane: bool| {
        let label = painted
            .iter()
            .rev()
            .find(|(text, rect)| {
                text.starts_with(name) && (!in_pane || rect.top() > volts.bottom())
            })
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("{name} is named: {painted:?}"));
        painted
            .iter()
            .filter(|(_, rect)| {
                (rect.center().y - label.center().y).abs() < 4.0 && rect.left() >= label.left()
            })
            .map(|(text, _)| text.clone())
            .collect::<Vec<_>>()
    };
    let reads = |row: &[String], value: &str| {
        row.iter()
            .any(|text| text == value || text.ends_with(&format!(" {value}")))
    };
    let entry = row(STAIRCASE, true);
    assert!(
        reads(&entry, "128.0") && !reads(&entry, "0"),
        "the entry reads after the event: {entry:?}"
    );
    let lane = row("code", false);
    assert!(
        reads(&lane, "128"),
        "the lane reads after the event: {lane:?}"
    );
}
