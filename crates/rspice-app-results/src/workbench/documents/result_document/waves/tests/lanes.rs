//! Digital nets in the waveform strip, on real mixed runs.
//!
//! The SAR converter's port-level deck keeps its nets only as event history:
//! the run has analog columns for the converter's nodes and no `D()` column
//! for any of them. Every claim here is made on what the Waves sheet draws
//! for that run, through the frame the reader sees.

use super::*;
use crate::state::{AnalysisResultPayload, SimulationRun};
use egui::epaint::ClippedShape;
use rspice_app_runs::simulation::controller::test_execution::run_manual_batch;
use rspice_results::events::projection::bus_word;
use rspice_results::visualization_document::{BusRadix, VisualizationDocument};
use rspice_results_ui::waves::lanes::{BusView, EventLaneLayout, LaneKey, UserLaneBus, lane_specs};
use rspice_results_ui::waves::logic::LOGIC_ROW_HEIGHT;
use std::sync::OnceLock;

/// The converter's result bits, MSB first.
const BITS: [&str; 8] = ["B7", "B6", "B5", "B4", "B3", "B2", "B1", "B0"];
/// The analog nodes the strip plots beside the lanes.
const ANALOG: [&str; 2] = ["VIN", "TOP"];
pub(super) const FRAME: egui::Vec2 = egui::vec2(1200.0, 760.0);

/// A corpus deck with its netlists inlined and its HDL sources made
/// absolute, so it runs without an imported deck origin, each line passed
/// through `edit`.
fn corpus_deck(design: &str, deck: &str, edit: impl Fn(&str) -> String) -> String {
    // A deck reads a backslash as an escape, so a Windows manifest path goes
    // into the deck with forward slashes.
    let root = format!(
        "{}/../../tests/mixed-corpus/{design}",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/")
    );
    let read = |path: String| std::fs::read_to_string(&path).expect("the corpus deck is readable");
    read(format!("{root}/deck/{deck}"))
        .lines()
        .map(|line| match line.strip_prefix(".include ../net/") {
            Some(file) => read(format!("{root}/net/{file}")),
            None => edit(&line.replace("\"../net/", &format!("\"{root}/net/"))),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Two conversions of the SAR converter (C4, port-level digital), run once
/// through the real controller.
pub(super) fn sar_run() -> &'static SimulationRun {
    static RUN: OnceLock<SimulationRun> = OnceLock::new();
    RUN.get_or_init(|| {
        run_manual_batch(&corpus_deck("c04-sar8", "ports.tran.cir", |line| {
            line.replace(".tran 1n 32u", ".tran 1n 2.4u")
        }))
    })
}

/// A host on `run` with only the `analog` columns on the plot.
pub(super) fn host(run: &SimulationRun, analog: &[&str]) -> ResultsTestHost {
    let mut run = run.clone();
    for analysis in &mut run.analyses {
        for waveform in &mut analysis.waveforms {
            waveform.visible = analog.contains(&waveform.name.as_str());
        }
    }
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state
}

pub(super) fn strip(state: &mut ResultsTestHost) -> Arc<Vec<StripModel>> {
    let presentation = state.model.preferences.result_presentation_policy();
    cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    )
}

/// The committed `(time, code)` points of the digital net `net`.
pub(super) fn history(run: &SimulationRun, net: &str) -> Vec<(f64, u8)> {
    let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
        run.analyses[0].result_payload.as_ref()
    else {
        panic!("the run keeps its event history");
    };
    digital_traces
        .iter()
        .find(|trace| trace.node_name == net)
        .unwrap_or_else(|| panic!("the run records {net}"))
        .points
        .iter()
        .map(|point| (point.time_s, point.value_code))
        .collect()
}

/// The code `net` holds at `t`: its last committed event at or before it.
fn code_at(run: &SimulationRun, net: &str, t: f64) -> Option<u8> {
    history(run, net)
        .iter()
        .rev()
        .find(|(time, _)| *time <= t)
        .map(|(_, code)| *code)
}

/// The SAR strip as reviewed: the analog nodes, SAMP and EOC as logic lanes
/// and the result bits grouped into one `code` bus read in `radix`, in the
/// order the reader arranged them.
pub(super) fn sar_strip(
    radix: BusRadix,
    expanded: bool,
) -> (ResultsTestHost, AnalysisPresentationKey) {
    let mut state = host(sar_run(), &ANALOG);
    let key = strip(&mut state)[0].analysis_key;
    let nets: Vec<String> = ["SAMP", "EOC"]
        .iter()
        .chain(&BITS)
        .map(|net| format!("D({net})"))
        .collect();
    for net in &nets {
        set_event_net_plotted(&mut state.results_parts(), key, net, true);
    }
    state.results.results.session.event_lanes.insert(
        key,
        EventLaneLayout {
            user_buses: vec![UserLaneBus {
                name: "code".to_owned(),
                members: nets[2..].to_vec(),
            }],
            buses: [("code".to_owned(), BusView { radix, expanded })].into(),
            order: vec![
                LaneKey::Signal("D(SAMP)".to_owned()),
                LaneKey::Signal("D(EOC)".to_owned()),
                LaneKey::Bus("code".to_owned()),
            ],
            ..EventLaneLayout::default()
        },
    );
    (state, key)
}

pub(super) fn place_cursors(state: &mut ResultsTestHost, a: Option<f64>, b: Option<f64>) {
    let session = &mut state.results.results.session;
    session.cursor_strip = Some(0);
    session.cursors = crate::ui::plot::CursorPair { a, b };
}

/// One frame of the Waves sheet, as the reader's window lays it out.
pub(super) fn frame(
    ctx: &egui::Context,
    state: &mut ResultsTestHost,
    events: Vec<egui::Event>,
) -> Vec<ClippedShape> {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
        events,
        ..Default::default()
    };
    ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
    })
    .shapes
}

pub(super) fn theme_context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// A vertical stroke the frame painted.
#[derive(Debug, Clone, Copy)]
pub(super) struct Vertical {
    pub(super) x: f32,
    pub(super) top: f32,
    pub(super) bottom: f32,
    pub(super) color: egui::Color32,
}

pub(super) fn verticals(shapes: &[ClippedShape]) -> Vec<Vertical> {
    fn walk(shape: &egui::Shape, out: &mut Vec<Vertical>) {
        match shape {
            egui::Shape::LineSegment { points, stroke }
                if (points[0].x - points[1].x).abs() < 1e-3 =>
            {
                out.push(Vertical {
                    x: points[0].x,
                    top: points[0].y.min(points[1].y),
                    bottom: points[0].y.max(points[1].y),
                    color: stroke.color,
                });
            }
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

/// The cursor lines of one colour, top to bottom: the lanes' first, then
/// each pane's.
pub(super) fn cursor_lines(lines: &[Vertical], color: egui::Color32) -> Vec<Vertical> {
    let mut found: Vec<Vertical> = lines
        .iter()
        .filter(|line| line.color == color && line.bottom - line.top > 8.0)
        .copied()
        .collect();
    found.sort_by(|a, b| a.top.total_cmp(&b.top));
    found
}

/// The row of a logic lane in the reviewed order.
fn row_of(signal: &str) -> usize {
    ["D(SAMP)", "D(EOC)"]
        .iter()
        .position(|candidate| *candidate == signal)
        .expect("a logic lane of the reviewed strip")
}

/// The y range of logic row `index` of the stack the cursor line `lanes`
/// spans: the line runs the stack's full height from its top.
fn lane_row(lanes: Vertical, index: usize) -> (f32, f32) {
    let top = (index as f32).mul_add(LOGIC_ROW_HEIGHT, lanes.top);
    (top, top + LOGIC_ROW_HEIGHT)
}

/// A run whose nets are only event history still puts them on the plot as
/// lanes, and the lanes share the panes' time axis to the pixel: cursor A on
/// SAMP's first fall and cursor B on EOC's last rise each cross the lanes
/// and the pane at one x, and the lanes' edges sit on those lines. Two
/// instants agree, so the two affine maps are one.
#[test]
fn lanes_draw_on_the_panes_time_axis() {
    let run = sar_run();
    let samp = history(run, "SAMP");
    let eoc = history(run, "EOC");
    let (early, late) = (samp[1].0, eoc[eoc.len() - 1].0);
    let (mut state, _) = sar_strip(BusRadix::Hex, false);
    place_cursors(&mut state, Some(early), Some(late));

    let models = strip(&mut state);
    let model = &models[0];
    let labels: Vec<_> = lane_specs(
        model,
        state
            .results
            .results
            .session
            .event_lanes
            .get(&model.analysis_key),
    )
    .iter()
    .map(|spec| spec.label().to_owned())
    .collect();
    assert_eq!(
        labels,
        ["SAMP", "EOC", "code"],
        "the lanes the reader plotted"
    );
    assert!(
        model
            .traces
            .iter()
            .any(|trace| trace.name == "TOP" && trace.visible),
        "a converter node keeps its analog trace beside the lanes"
    );

    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let lines = verticals(&frame(&ctx, &mut state, Vec::new()));
    let tokens = Tokens::get(&ctx);
    for (cursor, color, edge_net, t) in [
        ("A", tokens.color.canvas_cursor_a, "D(SAMP)", early),
        ("B", tokens.color.canvas_cursor_b, "D(EOC)", late),
    ] {
        let crossings = cursor_lines(&lines, color);
        assert!(
            crossings.len() >= 2,
            "cursor {cursor} crosses the lanes and a pane: {crossings:?}"
        );
        let lanes = crossings[0];
        for pane in &crossings[1..] {
            assert!(
                pane.top >= lanes.bottom,
                "the lanes sit above the panes: {crossings:?}"
            );
            assert!(
                (pane.x - lanes.x).abs() <= 0.5,
                "cursor {cursor} at {t:e} s is at x {} on the lanes and {} on a pane",
                lanes.x,
                pane.x
            );
        }
        // The cursors span the whole stack; a stroke inside one row is that
        // lane's own.
        let row = lane_row(lanes, row_of(edge_net));
        let edge = lines
            .iter()
            .filter(|line| line.top >= row.0 && line.bottom <= row.1)
            .min_by(|a, b| (a.x - lanes.x).abs().total_cmp(&(b.x - lanes.x).abs()))
            .unwrap_or_else(|| {
                let in_row: Vec<_> = lines
                    .iter()
                    .filter(|line| line.top >= row.0 - 1.0 && line.bottom <= row.1 + 1.0)
                    .collect();
                panic!("{edge_net} draws its edges in {row:?}: {in_row:?}")
            });
        assert!(
            (edge.x - crossings[1].x).abs() <= 0.5,
            "{edge_net}'s edge at {t:e} s is at x {}, the pane's cursor at {}",
            edge.x,
            crossings[1].x
        );
    }
}

/// The cursor table reads each lane at A and B: a logic lane's level and the
/// bus's word in its radix, spelled as the Events sheet spells a word.
#[test]
fn the_cursor_table_reads_the_lanes_at_both_cursors() {
    let run = sar_run();
    let eoc = history(run, "EOC");
    // Just after each conversion ends, when the word is the converted code.
    let rises: Vec<f64> = eoc
        .windows(2)
        .filter(|pair| pair[1].1 > pair[0].1)
        .map(|pair| pair[1].0)
        .collect();
    assert!(rises.len() >= 2, "the run converts twice: {eoc:?}");
    let (a, b) = (rises[0] + 1e-9, rises[1] + 1e-9);
    let (mut state, _) = sar_strip(BusRadix::Hex, false);
    place_cursors(&mut state, Some(a), Some(b));

    if !state.results.results.session.cursor_tool.is_armed() {
        state.results.results.session.toggle_cursor_tool();
    }
    let painted = super::interaction::painted_texts(&mut state, cursor_table);
    let word = |t: f64| {
        let codes: Vec<Option<u8>> = BITS.iter().map(|net| code_at(run, net, t)).collect();
        bus_word(&codes, BusRadix::Hex).text().to_owned()
    };
    let level = |net: &str, t: f64| {
        bus_word(&[code_at(run, net, t)], BusRadix::Binary)
            .text()
            .to_owned()
    };
    for expected in ["SAMP", "EOC", "code"] {
        assert!(
            painted.iter().any(|text| text == expected),
            "row {expected}: {painted:?}"
        );
    }
    let (word_a, word_b) = (word(a), word(b));
    assert_ne!(word_a, word_b, "the fixture converts two different inputs");
    for expected in [&word_a, &word_b] {
        assert!(
            painted.contains(expected),
            "the code word {expected}: {painted:?}"
        );
    }
    assert_eq!(level("EOC", a), "1", "EOC has risen at A");
}

/// The readout strip under the Waves sheet, where the A/B table is.
fn cursor_table(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let height = readout_strip_height(state);
    readout_strip(ui, state, height);
}

/// Selecting a lane selects its net and the arrow keys then step the
/// cursor from edge to edge of it, landing on the committed event time
/// exactly, not on a sample of the analog grid near it.
#[test]
fn the_edge_step_lands_on_the_next_edge_exactly() {
    let run = sar_run();
    let samp = history(run, "SAMP");
    let (mut state, _) = sar_strip(BusRadix::Hex, false);
    place_cursors(&mut state, Some(samp[1].0 * 0.5), None);
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let lines = verticals(&frame(&ctx, &mut state, Vec::new()));
    let lanes = cursor_lines(&lines, Tokens::get(&ctx).color.canvas_cursor_a)[0];
    // On SAMP's row, clear of its edges and of the cursor.
    let row = lane_row(lanes, row_of("D(SAMP)"));
    let at = egui::pos2(lanes.x + 3.0, (row.0 + row.1) * 0.5);
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    frame(&ctx, &mut state, vec![egui::Event::PointerMoved(at)]);
    frame(&ctx, &mut state, vec![button(true)]);
    frame(&ctx, &mut state, vec![button(false)]);
    let session = &state.results.results.session;
    assert_eq!(
        session
            .selected_trace
            .as_ref()
            .map(|selected| selected.source_name().to_owned()),
        Some("D(SAMP)".to_owned()),
        "selecting the lane selects its net"
    );
    let from = session.cursors.a.expect("cursor A is placed");
    let next = samp
        .iter()
        .map(|(time, _)| *time)
        .find(|time| *time > from)
        .expect("SAMP changes after the cursor");
    frame(
        &ctx,
        &mut state,
        vec![egui::Event::Key {
            key: egui::Key::ArrowRight,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }],
    );
    assert_eq!(
        state.results.results.session.cursors.a,
        Some(next),
        "the step lands on SAMP's next committed edge"
    );
}

/// One frame of the active project result document.
fn document_frame(state: &mut ResultsTestHost, document: crate::product::ResultDocumentId) {
    let ctx = theme_context();
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, FRAME)),
        ..Default::default()
    };
    let mut controller = crate::simulation::SimulationController::new();
    let _ = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            super::super::super::persistent_document::show(
                ui,
                &mut state.results_parts(),
                &mut controller,
                None,
                document,
            );
        });
    });
}

/// A saved view keeps its lanes: which nets are lanes, their order, the
/// reader's bus and that bus's radix and expansion survive writing the
/// document out and reading it back into a fresh session.
#[test]
fn radix_and_expansion_survive_save_and_reopen() {
    let (mut state, key) = sar_strip(BusRadix::Unsigned, true);
    let document_id = {
        let mut parts = state.results_parts();
        super::super::super::create_document::open(&mut parts);
        parts.create_result_document.layout_id =
            rspice_results::document_creation::ResultDocumentLayout::SinglePane
                .id()
                .to_owned();
        super::super::super::create_document::commit(&mut parts).expect("the view is saved")
    };
    // The reader's lanes reach the document over the frames that draw them:
    // the nets' traces, then their kinds, buses and order, then nothing.
    let document = |state: &ResultsTestHost| {
        state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("the project keeps the document")
            .clone()
    };
    for _ in 0..3 {
        document_frame(&mut state, document_id);
    }
    let settled = document(&state).revision();
    document_frame(&mut state, document_id);
    assert_eq!(
        document(&state).revision(),
        settled,
        "an idle frame writes nothing"
    );
    let saved = document(&state);
    assert!(
        saved.user_buses().iter().any(|bus| bus.name == "code"
            && bus.radix == BusRadix::Unsigned
            && bus.expanded
            && bus.nets.len() == BITS.len()),
        "the document holds the reader's bus: {:?}",
        saved.user_buses()
    );

    let json = serde_json::to_string(&saved).expect("the document writes out");
    let reopened: VisualizationDocument = serde_json::from_str(&json).expect("it reads back");
    let mut fresh = host(sar_run(), &ANALOG);
    let reopened_id = fresh
        .model
        .workspace
        .content
        .insert_visualization_document(reopened)
        .expect("the project takes the document back");
    document_frame(&mut fresh, reopened_id);
    document_frame(&mut fresh, reopened_id);
    let layout = fresh
        .results
        .results
        .session
        .event_lanes
        .get(&key)
        .cloned()
        .expect("the reopened pane has its lanes");
    assert_eq!(
        layout.bus_view("code"),
        BusView {
            radix: BusRadix::Unsigned,
            expanded: true
        }
    );
    let models = strip(&mut fresh);
    let model = &models[0];
    let labels: Vec<_> = lane_specs(model, Some(&layout))
        .iter()
        .map(|spec| spec.label().to_owned())
        .collect();
    assert_eq!(
        labels,
        ["SAMP", "EOC", "code"],
        "the lanes come back in order"
    );
}

/// The ladder filter's wreal netlist (C8): its stimulus is a real-valued
/// net, kept as event history.
fn ladder_run() -> SimulationRun {
    run_manual_batch(&corpus_deck("c08-ladder", "wr.tran.cir", |line| {
        line.replace(".tran 0.2u 0.7m", ".tran 0.2u 0.1m")
    }))
}

/// An `E()` net plotted from its row is drawn in the pane of its unit as a
/// step trace: each value held flat to the next event, with a vertical at
/// each event time.
#[test]
fn a_real_net_draws_as_a_step_trace() {
    let run = ladder_run();
    let Some(AnalysisResultPayload::TransientEvents { real_traces, .. }) =
        run.analyses[0].result_payload.as_ref()
    else {
        panic!("the run keeps its event history");
    };
    let real = real_traces.first().expect("the run records a real net");
    let name = format!("E({})", real.node_name);
    let mut state = host(&run, &[]);
    let key = strip(&mut state)[0].analysis_key;
    let unplotted = super::interaction::painted_path_points(&mut state, show);
    set_event_net_plotted(&mut state.results_parts(), key, &name, true);
    let models = strip(&mut state);
    let model = &models[0];
    let lane = model
        .lane_traces
        .iter()
        .find(|lane| lane.source_waveform_name == name)
        .expect("the strip has the lane");
    assert!(lane.visible, "plotting the net puts it on the plot");
    let panes = model.unit_panes();
    let pane = panes
        .iter()
        .find(|pane| pane.unit == model.lane_unit(lane))
        .expect("the lane opens the pane of its unit");
    let steps = rspice_results_ui::waves::lanes::real_step_traces(model, pane);
    assert_eq!(steps.len(), 1, "one step trace for the one real net");
    let step = &steps[0];
    // Every committed change is a vertical of the trace at its exact time.
    for pair in real
        .points
        .windows(2)
        .filter(|pair| pair[0].value != pair[1].value)
    {
        let (t, before, after) = (pair[1].time_s, pair[0].value, pair[1].value);
        let at: Vec<f64> = step
            .x
            .iter()
            .zip(step.y.iter())
            .filter(|(x, _)| **x == t)
            .map(|(_, y)| *y)
            .collect();
        assert_eq!(at, [before, after], "a vertical at {t:e} s");
    }
    assert!(
        super::interaction::painted_path_points(&mut state, show) > unplotted,
        "the pane draws the step trace"
    );
}

/// The picture for review: a strip with the analog panes and the logic and
/// bus lanes of the SAR run.
#[test]
#[ignore = "writes the review picture"]
fn write_the_sar_strip_picture() {
    let run = sar_run();
    let eoc = history(run, "EOC");
    let (mut state, _) = sar_strip(BusRadix::Hex, false);
    place_cursors(&mut state, Some(eoc[eoc.len() - 1].0 + 1e-9), None);
    let canvas = rspice_ui_kit::raster::render(FRAME, |ui, _| show(ui, &mut state.results_parts()));
    std::fs::write(
        "/Users/jameswhitfield/Desktop/RSpice-wip-archive/app-audit/w1-lanes-frame.png",
        canvas.png(canvas.height()),
    )
    .expect("the picture is written");
}

/// The analog nodes the review strip keeps, and which of them it plots.
pub(super) const REVIEW_NODES: [&str; 10] =
    ["VIN", "TOP", "B0", "B1", "B2", "B3", "B4", "B5", "B6", "B7"];
pub(super) const REVIEW_PLOTTED: [&str; 7] = ["VIN", "TOP", "B0", "B2", "B3", "B5", "B7"];

/// The review strip: ten analog nodes, three of them hidden, beside the
/// SAMP and EOC lanes and the `code` bus, with both cursors placed; or,
/// without lanes, its panes alone.
pub(super) fn review_strip(lanes: bool) -> ResultsTestHost {
    let run = sar_run();
    let eoc = history(run, "EOC");
    let samp = history(run, "SAMP");
    let (mut state, _) = if lanes {
        sar_strip(BusRadix::Hex, false)
    } else {
        let mut state = host(run, &ANALOG);
        let key = strip(&mut state)[0].analysis_key;
        (state, key)
    };
    for analysis in &mut state.model.simulation.retained.runs[0].analyses {
        analysis.waveforms.retain(|waveform| {
            REVIEW_NODES.contains(&waveform.name.as_str()) || waveform.name.starts_with("I(")
        });
        for waveform in &mut analysis.waveforms {
            waveform.visible = REVIEW_PLOTTED.contains(&waveform.name.as_str());
        }
    }
    // The analog host projected the strip before its columns were cut down;
    // a new generation makes the projection read the review's own dataset.
    state.model.simulation.view.data_version =
        state.model.simulation.view.data_version.wrapping_add(1);
    place_cursors(
        &mut state,
        Some(samp[1].0),
        Some(eoc[eoc.len() - 1].0 + 1e-9),
    );
    state
}

/// The document well the viewer sits in, filled as the result document
/// fills it before any viewer paints.
pub(super) fn well(ui: &mut Ui) {
    let well = ui.available_rect_before_wrap();
    ui.painter()
        .rect_filled(well, 0.0, Tokens::get(ui.ctx()).color.canvas_bg);
}

/// The pictures for review, written under `RSPICE_PICTURES` when it is set:
/// the strip in the default, light, dark and mismatched themes, and its
/// panes alone with every current hidden.
#[test]
#[ignore = "writes the review pictures"]
fn write_the_review_pictures() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let themed = |mode, canvas_theme| crate::ui::Theme {
        mode,
        canvas_theme,
        ..crate::ui::Theme::default()
    };
    use rspice_ui_kit::{EngineeringCanvasTheme as Canvas, tokens::Mode};
    for (name, theme, lanes) in [
        ("strip-default", crate::ui::Theme::default(), true),
        ("strip-light", themed(Mode::Light, Canvas::Light), true),
        ("strip-dark", themed(Mode::Dark, Canvas::Dark), true),
        (
            "strip-light-ui-dark-canvas",
            themed(Mode::Light, Canvas::Dark),
            true,
        ),
        (
            "strip-dark-ui-light-canvas",
            themed(Mode::Dark, Canvas::Light),
            true,
        ),
        ("pane-all-hidden", crate::ui::Theme::default(), false),
    ] {
        let mut state = review_strip(lanes);
        let canvas = rspice_ui_kit::raster::render_themed(theme, FRAME, |ui, _| {
            well(ui);
            show(ui, &mut state.results_parts());
        });
        std::fs::write(
            format!("{directory}/{name}.png"),
            canvas.png(canvas.height()),
        )
        .expect("the picture is written");
    }
}
