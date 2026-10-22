//! Offscreen render of the lanes and a value pane on one time axis: a logic
//! lane, lanes with an unknown and a high-impedance stretch, a HEX bus lane
//! with an unknown word, and an `E()` step sharing a pane with a `V()` trace.
//!
//! Measured, not eyeballed: every edge's ink centroid against the axis
//! mapping, the unknown band and the high-impedance line by pixel, and the
//! pane's plot rectangle against the lanes' mapping.

use super::tests::{NS, fixture};
use super::*;
use rspice_results::events::DigitalEventPointEvidence;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale};
use rspice_ui_kit::raster::{self, Canvas};

const SIZE: egui::Vec2 = egui::vec2(720.0, 320.0);
const LEFT: f32 = 64.0;
const RIGHT: f32 = 14.0;
const END: f64 = 100.0 * NS;
const Y_RANGE: (f64, f64) = (-0.1, 1.1);

struct Render {
    canvas: Canvas,
    lanes_top: f32,
    map: TimeMapping,
    plot_rect: Rect,
    y_range: (f64, f64),
    lane_color: Color32,
    step_color: Color32,
    unknown: Color32,
    error: Color32,
    real: RealLane,
}

/// The §2.4 fixture with `b[1]` unknown from 80 ns, plus `r` (unknown, 0 at
/// 3 ns, unknown at 5 ns, 1 at 8 ns), `z` (high impedance until 50 ns) and
/// a real net `w`.
fn scene() -> (EventLanes, LogicLane, LogicLane, RealLane) {
    let (mut digital, buses) = fixture();
    let b1 = digital
        .iter_mut()
        .find(|trace| trace.node_name == "b[1]")
        .unwrap();
    b1.points.retain(|point| point.time_s < 80.0 * NS);
    b1.points.push(DigitalEventPointEvidence {
        time_s: 80.0 * NS,
        value_code: 2,
    });
    let lanes = EventLanes::from_evidence(&digital, &[], &buses);
    let r = LogicLane::from_events("r", [(0.0, 2), (3.0 * NS, 0), (5.0 * NS, 2), (8.0 * NS, 1)]);
    let z = LogicLane::from_events("z", [(0.0, 12), (50.0 * NS, 1)]);
    let w = RealLane::from_events(
        "w",
        [
            (0.0, 0.0),
            (5.0 * NS, 0.25),
            (9.0 * NS, 0.75),
            (40.0 * NS, 0.5),
            (75.0 * NS, 0.9),
        ],
    );
    (lanes, r, z, w)
}

/// A warning, an error and the end of the run, one of each glyph.
fn hdl_markers() -> Vec<HdlMarker> {
    let marker = |time: f64, severity, text: &str, line| HdlMarker {
        time,
        severity,
        text: text.to_owned(),
        scope: "tb".to_owned(),
        source: Some(SourceLine {
            file: "tb.v".to_owned(),
            line,
        }),
    };
    vec![
        marker(25.0 * NS, MarkerSeverity::Warning, "slow edge", 31),
        marker(65.0 * NS, MarkerSeverity::Error, "count mismatch", 40),
        marker(85.0 * NS, MarkerSeverity::Fatal, "abort", 44),
        marker(95.0 * NS, MarkerSeverity::Finish, "", 52),
    ]
}

fn render() -> Render {
    // An error-level violation on q, clear of the edges measured below.
    let violations = [AssertionSpan {
        start: 42.0 * NS,
        end: 48.0 * NS,
        operand: "D(q)".to_owned(),
        label: "q_settled".to_owned(),
        error: true,
    }];
    let (lanes, r, z, w) = scene();
    let q = lanes.logic("q").unwrap();
    let bus = lanes.bus("b").unwrap();
    let v_x: Vec<f64> = (0..=400).map(|k| f64::from(k) * END / 400.0).collect();
    let v_y: Vec<f64> = v_x
        .iter()
        .map(|t| 0.4f64.mul_add((t / (40.0 * NS) * std::f64::consts::TAU).sin(), 0.5))
        .collect();
    let mut cache = plot::DecimationCache::default();
    let mut measured = None;
    let canvas = raster::render(SIZE, |ui, _| {
        let tokens = Tokens::get(ui.ctx());
        let lane_color = tokens.color.traces[0];
        let step_color = tokens.color.traces[1];
        let rows = [
            LaneRow::Logic {
                lane: q,
                color: lane_color,
            },
            LaneRow::Logic {
                lane: &r,
                color: lane_color,
            },
            LaneRow::Logic {
                lane: &z,
                color: lane_color,
            },
            LaneRow::Bus {
                lane: bus,
                radix: BusRadix::Hex,
                expanded: false,
                members: Vec::new(),
                color: lane_color,
            },
        ];
        let input = LaneStackInput {
            rows: &rows,
            x_range: (0.0, END),
            left_margin: LEFT,
            right_margin: RIGHT,
            selected: &[],
            cursor: Some(55.0 * NS),
            significant_digits: 4,
            violations: &violations,
        };
        show_marker_strip(
            ui,
            &MarkerStripInput {
                hdl: &hdl_markers(),
                assertions: &violations,
                x_range: (0.0, END),
                left_margin: LEFT,
                right_margin: RIGHT,
            },
            &|t| format!("{:.0} ns", t / NS),
        );
        let lanes_top = ui.cursor().top();
        let lanes_rect = Rect::from_min_size(
            pos2(ui.max_rect().left(), lanes_top),
            vec2(ui.available_width(), stack_height(&rows)),
        );
        show_lanes(ui, &input);
        let step = real_step_trace(&w, END, step_color, 7);
        let mut spec = PlotSpec::new(
            Axis::linear(0.0, END, "s"),
            XScale::Linear,
            Axis::linear(Y_RANGE.0, Y_RANGE.1, "V"),
        )
        .without_x_axis_chrome()
        .with_right_margin(RIGHT);
        spec.left_margin = LEFT;
        spec.traces = vec![
            Trace::new(&v_x, &v_y, tokens.color.traces[2]),
            Trace::new(&step.x, &step.y, step_color).shape(&step.shape),
        ];
        let response = plot::show(ui, &spec, &mut cache, None, None);
        measured = Some((
            lanes_top,
            lane_mapping(lanes_rect, &input),
            response.plot_rect,
            response.axes,
            lane_color,
            step_color,
            (tokens.color.warn, tokens.color.err),
        ));
    });
    let (lanes_top, map, plot_rect, axes, lane_color, step_color, (unknown, error)) =
        measured.expect("the pass ran");
    Render {
        canvas,
        lanes_top,
        map,
        plot_rect,
        y_range: axes.1,
        lane_color,
        step_color,
        unknown,
        error,
        real: w,
    }
}

fn channels(color: Color32) -> [f32; 3] {
    let [r, g, b, _] = color.to_array();
    [f32::from(r), f32::from(g), f32::from(b)]
}

/// `color` composited over `background`, as the rasterizer blends.
fn over(color: Color32, background: Color32) -> [f32; 3] {
    let alpha = f32::from(color.a()) / 255.0;
    let [sr, sg, sb] = channels(color);
    let [dr, dg, db] = channels(background);
    [
        dr.mul_add(1.0 - alpha, sr),
        dg.mul_add(1.0 - alpha, sg),
        db.mul_add(1.0 - alpha, sb),
    ]
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).abs())
        .fold(0.0, f32::max)
}

fn pixel(canvas: &Canvas, x: usize, y: usize) -> [f32; 3] {
    channels(canvas.pixel(x, y).expect("inside the canvas"))
}

/// How much of `ink` a pixel carries over `base`, the surface it is drawn
/// on: 0 for bare surface, 1 for full ink, anti-aliased coverage between.
fn coverage(canvas: &Canvas, x: usize, y: usize, ink: Color32, base: [f32; 3]) -> f32 {
    let ink = channels(ink);
    let full = distance(ink, base).max(1.0);
    (1.0 - distance(pixel(canvas, x, y), ink) / full).clamp(0.0, 1.0)
}

/// The ink centroid of row `y` within `half` pixels of `around`, in the
/// pixel-centre convention the mapping uses. The surface is read just left
/// of the window, so a plot well or a grid line under the row cancels out.
fn centroid(canvas: &Canvas, y: f32, around: f32, half: i32, ink: Color32) -> f32 {
    inked_centroid(canvas, y, around, half, ink)
        .unwrap_or_else(|| panic!("no ink near x={around} on row {y}"))
}

/// [`centroid`], or `None` where the row carries no ink there (a dash gap).
fn inked_centroid(canvas: &Canvas, y: f32, around: f32, half: i32, ink: Color32) -> Option<f32> {
    let row = y as usize;
    let centre = around.floor() as i32;
    let base = pixel(canvas, (centre - half - 2) as usize, row);
    let (mut weight, mut moment) = (0.0, 0.0);
    for x in (centre - half)..=(centre + half) {
        let w = coverage(canvas, x as usize, row, ink, base);
        weight += w;
        moment += w * (x as f32 + 0.5);
    }
    (weight > 0.5).then(|| moment / weight)
}

/// The ink a one-pixel horizontal line at `y` leaves in column `x`, summed
/// over the rows it can straddle.
fn line_ink(canvas: &Canvas, x: usize, y: f32, ink: Color32) -> f32 {
    let base = channels(canvas.background());
    let row = y.floor() as usize;
    (row - 1..=row + 1)
        .map(|row| coverage(canvas, x, row, ink, base))
        .sum()
}

fn lane_levels(render: &Render, row: usize) -> LaneLevels {
    let top = render.lanes_top + LOGIC_ROW_HEIGHT * row as f32;
    LaneLevels::of(Rect::from_x_y_ranges(
        render.map.left..=render.map.right,
        top..=top + LOGIC_ROW_HEIGHT,
    ))
}

#[test]
fn lanes_and_the_value_pane_render_on_one_measured_time_axis() {
    let render = render();
    let canvas = &render.canvas;
    let map = render.map;

    // The marker strip sits above the lanes, and q's violation is shaded
    // on q's own lane in the error token.
    assert!(render.lanes_top >= markers::MARKER_STRIP_HEIGHT);
    let shade = over(render.error.gamma_multiply(0.28), canvas.background());
    let shaded = pixel(
        canvas,
        map.x(45.0 * NS) as usize,
        lane_levels(&render, 0).mid() as usize,
    );
    assert!(distance(shaded, shade) <= 2.0, "{shaded:?} vs {shade:?}");

    // The pane's plot rectangle and the lanes' mapping are one axis.
    assert!((render.plot_rect.left() - map.left).abs() <= 0.5);
    assert!((render.plot_rect.right() - map.right).abs() <= 0.5);

    // Every q edge sits on the mapping of its exact instant, to ±0.5 px.
    let (lanes, _, _, _) = scene();
    let q = lanes.logic("q").unwrap();
    let mid = lane_levels(&render, 0).mid();
    for time in &q.times()[1..] {
        let expected = map.x(*time);
        let measured = centroid(canvas, mid, expected, 3, render.lane_color);
        assert!(
            (measured - expected).abs() <= 0.5,
            "edge at {time}: measured x {measured}, mapped {expected}"
        );
    }

    // r is unknown over [5 ns, 8 ns): the band fills between the levels in
    // the warning token, and the same fill marks the bus's unknown word.
    let band = over(render.unknown.gamma_multiply(0.4), canvas.background());
    let r_levels = lane_levels(&render, 1);
    let band_x = map.x(6.5 * NS) as usize;
    let band_pixel = pixel(canvas, band_x, r_levels.mid() as usize);
    assert!(
        distance(band_pixel, band) <= 2.0,
        "{band_pixel:?} vs {band:?}"
    );
    let bus_top = render.lanes_top + 3.0 * LOGIC_ROW_HEIGHT;
    let bus_levels = LaneLevels::of(Rect::from_x_y_ranges(
        map.left..=map.right,
        bus_top..=bus_top + BUS_ROW_HEIGHT,
    ));
    let word_x = (map.x(80.0 * NS) + 5.0) as usize;
    let word_pixel = pixel(canvas, word_x, bus_levels.mid() as usize);
    assert!(
        distance(word_pixel, band) <= 2.0,
        "{word_pixel:?} vs {band:?}"
    );
    // A resolved word is not filled.
    let clear_x = (map.x(50.0 * NS) + 5.0) as usize;
    let clear = pixel(canvas, clear_x, bus_levels.mid() as usize);
    assert!(distance(clear, channels(canvas.background())) <= 2.0);

    // z is high impedance until 50 ns: a dashed line at mid level, with
    // nothing at either level.
    let z_levels = lane_levels(&render, 2);
    let (from, to) = (map.x(5.0 * NS) as usize, map.x(45.0 * NS) as usize);
    let dashes: Vec<f32> = (from..to)
        .map(|x| line_ink(canvas, x, z_levels.mid(), render.lane_color))
        .collect();
    assert!(dashes.iter().any(|ink| *ink > 0.6), "the z line has dashes");
    assert!(dashes.iter().any(|ink| *ink < 0.2), "the z line has gaps");
    for level in [z_levels.high, z_levels.low] {
        assert!(
            (from..to).all(|x| line_ink(canvas, x, level, render.lane_color) < 0.2),
            "nothing at level {level} while z"
        );
    }

    // The E() step rises vertically at exactly 9 ns, on the pane's axis.
    let rect = render.plot_rect;
    let (y0, y1) = render.y_range;
    let y_of = |value: f64| rect.bottom() - ((value - y0) / (y1 - y0)) as f32 * rect.height();
    let step_time = render.real.times()[2];
    // The plot draws this trace in its second dash style, so the rows
    // between the two levels are read where a dash lands on them.
    let mapped = map.x(step_time);
    let rows = (y_of(0.75) + 3.0) as usize..(y_of(0.25) - 3.0) as usize;
    let measured: Vec<f32> = rows
        .filter_map(|row| inked_centroid(canvas, row as f32, mapped, 3, render.step_color))
        .collect();
    assert!(measured.len() >= 5, "the vertical is drawn: {measured:?}");
    for x in measured {
        assert!(
            (x - mapped).abs() <= 0.5,
            "E() step at {step_time}: measured x {x}, mapped {mapped}"
        );
    }
}

/// Writes the render for review. Set `RSPICE_REVIEW_DIR` to choose where.
#[test]
#[ignore = "writes a PNG for review"]
fn write_lane_render_png() {
    let render = render();
    let directory = std::env::var_os("RSPICE_REVIEW_DIR")
        .map_or_else(std::env::temp_dir, std::path::PathBuf::from);
    let path = directory.join("logic-lanes.png");
    let height = render.canvas.height();
    std::fs::write(&path, render.canvas.png(height)).expect("write lane render");
    println!("lane render: {}", path.display());
}
