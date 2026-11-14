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

/// The staircase is drawn in a pane of its own, and that pane names it: an
/// entry under the volts pane's header row, with its swatch, as each of
/// TOP and VIN has in the volts pane.
#[test]
fn an_expression_on_a_pane_of_its_own_is_named_in_it() {
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, "analog(D(code))");
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let volts = text(&output, "45 hidden").expect("the volts pane's header row");
    let entry = text(&output, "analog(D(code))").expect("the staircase is named");
    assert!(
        entry.top() > volts.bottom(),
        "the staircase's entry is in its own pane, under the volts pane's row: \
         entry {entry:?}, volts row {volts:?}"
    );
    // The strip header's chip has a swatch of the same name; the entry's is
    // the one on its row.
    let swatch = controls(&output)
        .into_iter()
        .filter(|(_, label, _)| label.as_deref() == Some("Toggle analog(D(code)) visibility"))
        .filter_map(|(_, _, rect)| rect)
        .min_by(|a, b| {
            (a.center().y - entry.center().y)
                .abs()
                .total_cmp(&(b.center().y - entry.center().y).abs())
        })
        .expect("the entry has its swatch");
    assert!(
        (swatch.center().y - entry.center().y).abs() < 4.0 && swatch.right() <= entry.left(),
        "the entry's swatch leads it on its row: swatch {swatch:?}, entry {entry:?}"
    );
}

/// An expression on a pane of signals hides from its entry's swatch as a
/// signal does, then waits behind the pane's hidden control, and ticking it
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
            text(&output, "45 hidden")
                .is_some_and(|row| (entry.center().y - row.center().y).abs() < 4.0)
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
    let hidden = text(&output, "46 hidden").expect("the pane counts the hidden expression");
    click(
        &ctx,
        &mut state,
        hidden.center(),
        egui::PointerButton::Primary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    let item = text(&menu, DOUBLE).expect("the hidden control lists the expression");
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

/// Where the first run of text that reads `wanted` exactly was painted: on
/// the strip, the strip header's.
fn first(output: &FullOutput, wanted: &str) -> Option<egui::Rect> {
    texts(&output.shapes)
        .into_iter()
        .find(|(text, _)| text == wanted)
        .map(|(_, rect)| rect)
}

/// An expression's chip in the strip header offers the conversion a trace's
/// menu offers, ahead of Remove: Digital, which reads the expression as
/// typed.
#[test]
fn an_expression_chip_offers_digital() {
    const DOUBLE: &str = "V(TOP)*2";
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, DOUBLE);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let chip = first(&output, DOUBLE).expect("the strip header names the expression");
    click(
        &ctx,
        &mut state,
        chip.center(),
        egui::PointerButton::Secondary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    assert!(
        text(&menu, "Remove expression").is_some(),
        "the chip still offers Remove"
    );
    let digital = text(&menu, "Digital").expect("the chip's menu offers Digital");
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
}

/// A `digital()` expression's chip offers what a logic lane's menu offers:
/// Analog, which draws it as a staircase on a plain number's pane.
#[test]
fn a_logic_expression_chip_offers_analog() {
    const CLOCK: &str = "digital(V(CLK), 0.9)";
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, CLOCK);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    let chip = first(&output, CLOCK).expect("the strip header names the expression");
    click(
        &ctx,
        &mut state,
        chip.center(),
        egui::PointerButton::Secondary,
    );
    let menu = run(&ctx, &mut state, Vec::new());
    assert!(
        text(&menu, "Digital").is_none(),
        "a logic expression is not read as logic again"
    );
    let analog = text(&menu, "Analog").expect("the chip's menu offers Analog");
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
    // Both names are cut to the strip header's 24 characters.
    let name = rspice_results_ui::waves::header::elide(&staircase, 24);
    let output = settled(&ctx, &mut state);
    assert_eq!(
        texts(&output.shapes)
            .iter()
            .filter(|(text, _)| text == &name)
            .count(),
        2,
        "the staircase is drawn and named in its pane as in the strip header"
    );
}

/// Both axes' log toggles are one chip naming its axis, lit while the axis
/// is logarithmic: the pane's reads "log Y" in its header, the shared X
/// axis's "log X" in the strip header, which owns that axis, clear of
/// cursor A's flag with A on
/// the plot's left edge, with lanes beside the panes and without.
#[test]
fn the_y_log_toggle_names_its_axis_and_the_x_toggle_clears_cursor_a() {
    for lanes in [true, false] {
        let mut state = if lanes {
            sar_strip(BusRadix::Unsigned, false).0
        } else {
            super::lanes::host(super::lanes::sar_run(), &super::lanes::ANALOG)
        };
        super::lanes::place_cursors(&mut state, Some(0.0), None);
        let ctx = theme_context();
        let output = settled(&ctx, &mut state);
        // The volts pane's Y toggle and the strip's X toggle; the currents'
        // pane draws nothing and offers no toggle.
        let count = |word: &str| {
            texts(&output.shapes)
                .iter()
                .filter(|(text, _)| text == word)
                .count()
        };
        assert_eq!(
            (count("log Y"), count("log X")),
            (1, 1),
            "the Y toggle and the X toggle (lanes: {lanes})"
        );
        assert!(
            texts(&output.shapes)
                .iter()
                .all(|(text, _)| text != "LIN" && text != "LOG"),
            "no toggle states its scale in capitals (lanes: {lanes})"
        );
        let toggle = control(&output, "Logarithmic X axis");
        let flag = texts(&output.shapes)
            .into_iter()
            .filter(|(text, _)| text == "A")
            .map(|(_, rect)| rect)
            .max_by(|a, b| a.top().total_cmp(&b.top()))
            .expect("cursor A's flag on the time axis");
        assert!(
            toggle.bottom() < flag.top() && !toggle.expand(4.0).intersects(flag.expand(3.0)),
            "the X toggle clears cursor A's flag (lanes: {lanes}): toggle {toggle:?}, flag letter \
             {flag:?}"
        );
    }
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
    let row = text(&output, "45 hidden").expect("the volts pane's header row");
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

/// The C4 strip's currents, which the run keeps but does not plot and
/// nobody has plotted, open no pane: the strip draws its volts and its
/// staircase, and the currents wait in the signal browser. Once the reader
/// has hidden one, the currents' pane is its header alone, the row that
/// offers them, with no autoscale and no log toggle for an axis the pane
/// does not draw, its hidden control in one column with the volts pane's.
#[test]
fn a_pane_with_nothing_plotted_offers_no_axis_controls() {
    let (mut state, _) = sar_strip(BusRadix::Unsigned, false);
    let ctx = theme_context();
    let output = settled(&ctx, &mut state);
    assert!(
        text(&output, "62 hidden").is_none() && text(&output, "45 hidden").is_some(),
        "no pane is drawn for currents nobody plotted"
    );
    let (analysis, waveform) = {
        let models = super::lanes::strip(&mut state);
        let current = models[0]
            .traces
            .iter()
            .find(|trace| trace.source_waveform_name.starts_with("I("))
            .expect("the run keeps its currents");
        (models[0].analysis_index(), current.waveform_index)
    };
    toggle_visibility(&mut state.results_parts(), analysis, waveform);
    toggle_visibility(&mut state.results_parts(), analysis, waveform);
    let output = settled(&ctx, &mut state);
    let currents = text(&output, "62 hidden").expect("the currents' row offers them");
    let volts = text(&output, "45 hidden").expect("the volts pane's row");
    assert!(
        (currents.right() - volts.right()).abs() < 1.0,
        "the hidden controls stand in one column: volts ends at {}, currents at {}",
        volts.right(),
        currents.right()
    );
    let on_row = |row: egui::Rect, name: &str| {
        controls(&output)
            .into_iter()
            .filter(|(_, label, rect)| {
                label.as_deref() == Some(name)
                    && rect.is_some_and(|rect| (rect.center().y - row.center().y).abs() < 6.0)
            })
            .count()
    };
    for name in ["log Y", "Fit Y to the traces in view"] {
        assert_eq!(on_row(volts, name), 1, "the volts pane offers {name:?}");
        assert_eq!(
            on_row(currents, name),
            0,
            "the currents' row offers {name:?}"
        );
    }
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
            .any(|(text, _)| text.starts_with("No visible traces")),
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
/// `analog(D(code))` entry reads 128, beside the lane's 128.
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
    let volts = text(&output, "45 hidden").expect("the volts pane's header row");
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
        reads(&entry, "128") && !reads(&entry, "0"),
        "the entry reads after the event: {entry:?}"
    );
    let lane = row("code", false);
    assert!(
        reads(&lane, "128"),
        "the lane reads after the event: {lane:?}"
    );
}

/// The pictures for review, written under `RSPICE_PICTURES` when it is set:
/// the staircase named in its own pane beside the DAC, and an expression of
/// volts named in the volts pane with cursor A's reading, then hidden there.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_pane_legend_pictures() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
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
    let eoc = super::lanes::history(super::lanes::sar_run(), "EOC");
    let (mut state, key) = sar_strip(BusRadix::Unsigned, false);
    add_expression(&mut state, key, "V(TOP)*2");
    add_expression(&mut state, key, "analog(D(code))");
    super::lanes::place_cursors(&mut state, Some(eoc[0].0 + 1e-9), None);
    write("pane-legend-expressions", &mut state);
    state
        .results
        .results
        .session
        .analysis_exprs
        .get_mut(&key)
        .expect("the strip has expressions")[0]
        .visible = false;
    state
        .results
        .results
        .session
        .sync_expression_projection(key, 0);
    write("pane-legend-expression-hidden", &mut state);
}
