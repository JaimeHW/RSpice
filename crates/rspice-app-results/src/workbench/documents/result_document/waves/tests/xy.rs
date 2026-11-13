//! A unit pane's curves plotted against another signal, in a strip of their
//! own after the strip drawn against time: the pane's menu makes it, its one
//! horizontal axis is the signal, its header narrows the span of time it
//! draws, its cursor points are the time cursors, and the project and saved
//! views keep it.

use super::conversion::{click, find, texts};
use super::lanes::{frame, theme_context};
use super::*;
use egui::epaint::ClippedShape;
use rspice_results::visualization_document::Abscissa;
use rspice_results_ui::session::XyStrip;
use std::f64::consts::PI;

const OMEGA: f64 = 2.0 * PI * 1.0e3;
const PHI: f64 = PI / 3.0;

/// A 1 kHz transient over one period with V(b) leading V(a) by 60°.
fn lissajous_host() -> ResultsTestHost {
    let time: Vec<f64> = (0..=400).map(|step| f64::from(step) * 2.5e-6).collect();
    let a: Vec<f64> = time.iter().map(|t| (OMEGA * t).sin()).collect();
    let b: Vec<f64> = time.iter().map(|t| (OMEGA * t + PHI).sin()).collect();
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(a)", time.clone(), a, "#fff").with_unit("V"),
            RetainedTrace::new("V(b)", time, b, "#fff").with_unit("V"),
        ]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy provenance");
    run.restore_lifecycle(crate::state::SimulationRunLifecycle::LegacyUnknown, 0.0)
        .expect("legacy lifecycle");
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state
}

fn signal(name: &str) -> Abscissa {
    Abscissa::Signal {
        name: name.to_owned(),
    }
}

/// Plot the first strip's `unit` curves against `abscissa`, as the pane's
/// menu does.
fn open_strip(state: &mut ResultsTestHost, unit: &str, abscissa: Abscissa) {
    let analysis = super::lanes::strip(state)[0].analysis_key;
    state
        .results
        .results
        .session
        .xy_strips
        .push(XyStrip::new(analysis, unit, abscissa));
}

fn strips(state: &ResultsTestHost) -> &[XyStrip] {
    &state.results.results.session.xy_strips
}

/// The filled triangles a frame painted in `color`: the arrowheads.
fn arrowheads(shapes: &[ClippedShape], color: egui::Color32) -> Vec<Vec<egui::Pos2>> {
    fn walk(shape: &egui::Shape, color: egui::Color32, out: &mut Vec<Vec<egui::Pos2>>) {
        match shape {
            egui::Shape::Path(path)
                if path.closed && path.points.len() == 3 && path.fill == color =>
            {
                out.push(path.points.clone());
            }
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, color, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, color, &mut out);
    }
    out
}

/// The open lines a frame painted 1 px wide in a translucent shade: the
/// hairlines of a crowded family.
fn hairlines(shapes: &[ClippedShape]) -> usize {
    fn walk(shape: &egui::Shape) -> usize {
        match shape {
            egui::Shape::Path(path)
                if !path.closed
                    && path.stroke.width == 1.0
                    && matches!(
                        path.stroke.color,
                        egui::epaint::ColorMode::Solid(color) if color.a() > 0 && color.a() < 255
                    ) =>
            {
                1
            }
            egui::Shape::Vec(inner) => inner.iter().map(walk).sum(),
            _ => 0,
        }
    }
    shapes.iter().map(|clipped| walk(&clipped.shape)).sum()
}

/// The filled circles a frame painted in `color`, by centre.
fn dots(shapes: &[ClippedShape], color: egui::Color32) -> Vec<egui::Pos2> {
    fn walk(shape: &egui::Shape, color: egui::Color32, out: &mut Vec<egui::Pos2>) {
        match shape {
            egui::Shape::Circle(circle) if circle.fill == color || circle.stroke.color == color => {
                out.push(circle.center);
            }
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, color, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, color, &mut out);
    }
    out
}

/// Where the frame painted the volts pane's unit tag: the topmost "V".
fn volts_unit_tag(ctx: &egui::Context, state: &mut ResultsTestHost) -> egui::Pos2 {
    texts(&frame(ctx, state, Vec::new()))
        .into_iter()
        .filter(|(text, _)| text == "V")
        .min_by(|(_, a), (_, b)| a.top().total_cmp(&b.top()))
        .map(|(_, rect)| rect.center())
        .expect("the volts pane's header names its unit")
}

fn painted_texts(ctx: &egui::Context, state: &mut ResultsTestHost) -> Vec<(String, egui::Rect)> {
    frame(ctx, state, Vec::new());
    texts(&frame(ctx, state, Vec::new()))
}

/// Drag the pointer from `from` to `to` across the frames egui needs.
fn drag(ctx: &egui::Context, state: &mut ResultsTestHost, from: egui::Pos2, to: egui::Pos2) {
    let button = |at: egui::Pos2, pressed: bool| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::default(),
    };
    frame(ctx, state, vec![egui::Event::PointerMoved(from)]);
    frame(ctx, state, vec![button(from, true)]);
    for step in 1..=8 {
        let at = from + (to - from) * (step as f32 / 8.0);
        frame(ctx, state, vec![egui::Event::PointerMoved(at)]);
    }
    frame(ctx, state, vec![button(to, false)]);
}

/// `text` read as an axis tick: a bare number, its minus the typographic
/// one.
fn tick(text: &str) -> Option<f64> {
    text.replace('\u{2212}', "-").parse().ok()
}

/// The top of the strip whose header reads `subtitle`.
fn strip_top(painted: &[(String, egui::Rect)], subtitle: &str) -> f32 {
    painted
        .iter()
        .find(|(text, _)| text == subtitle)
        .map(|(_, rect)| rect.top())
        .unwrap_or_else(|| panic!("a strip header reads {subtitle:?}: {painted:?}"))
}

/// The time strip's pane header offers the strip's signals; choosing V(a)
/// adds a strip after it, headed "vs V(a)", whose X axis says how the two
/// were paired. That axis label chooses again, choosing what is already
/// shown makes nothing twice, and the strip's close button takes it away.
#[test]
fn the_panes_menu_makes_a_strip_against_a_signal_and_its_x_axis_chooses_again() {
    let mut state = lissajous_host();
    let ctx = theme_context();
    let painted = painted_texts(&ctx, &mut state);
    assert!(
        !painted.iter().any(|(text, _)| text.starts_with("vs ")),
        "a result drawn against time alone has no strip against a signal: {painted:?}"
    );

    let unit = volts_unit_tag(&ctx, &mut state);
    click(&ctx, &mut state, unit, egui::PointerButton::Secondary);
    assert!(
        texts(&frame(&ctx, &mut state, Vec::new()))
            .iter()
            .any(|(text, _)| text == "Plot the V curves against"),
        "the header's context menu offers what the curves are plotted against"
    );
    let item = find(&ctx, &mut state, "V(a)");
    click(&ctx, &mut state, item, egui::PointerButton::Primary);
    let analysis = super::lanes::strip(&mut state)[0].analysis_key;
    assert_eq!(
        strips(&state),
        [XyStrip::new(analysis, "V", signal("V(a)"))]
    );
    let painted = painted_texts(&ctx, &mut state);
    let top = strip_top(&painted, "vs V(a)");
    let x_label = painted
        .iter()
        .find(|(text, rect)| text.starts_with("V(a) \u{b7} at each t sample") && rect.top() > top)
        .map(|(_, rect)| rect.center())
        .unwrap_or_else(|| panic!("the X axis names the abscissa and the pairing: {painted:?}"));

    // The pane's menu again, on V(a): the strip already shows it.
    let unit = volts_unit_tag(&ctx, &mut state);
    click(&ctx, &mut state, unit, egui::PointerButton::Secondary);
    let item = find(&ctx, &mut state, "V(a)");
    click(&ctx, &mut state, item, egui::PointerButton::Primary);
    assert_eq!(
        strips(&state).len(),
        1,
        "a plot shown once is not made twice"
    );

    // The X axis label chooses what this strip is plotted against.
    click(&ctx, &mut state, x_label, egui::PointerButton::Primary);
    let item = find(&ctx, &mut state, "V(b)");
    click(&ctx, &mut state, item, egui::PointerButton::Primary);
    assert_eq!(
        strips(&state),
        [XyStrip::new(analysis, "V", signal("V(b)"))]
    );

    // The strip header's close button, its one action, takes it away. Both
    // header rows end on one rule, where the pane header's log chip ends 9
    // px past its text; the button is 22 px wide.
    let painted = painted_texts(&ctx, &mut state);
    let top = strip_top(&painted, "vs V(b)");
    let log = painted
        .iter()
        .find(|(text, rect)| text == "log" && rect.top() > top)
        .map(|(_, rect)| *rect)
        .expect("the strip's pane header has its log chip");
    let close = egui::pos2(log.right() + 9.0 - 11.0, top + 7.0);
    click(&ctx, &mut state, close, egui::PointerButton::Primary);
    assert!(strips(&state).is_empty());
}

/// A strip against a signal has exactly one horizontal axis, the signal's:
/// one row of ticks under its plot and one axis label, and no time ruler and
/// no overview under it.
#[test]
fn a_strip_against_a_signal_draws_one_horizontal_axis_and_no_ruler() {
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    let ctx = theme_context();
    let painted = painted_texts(&ctx, &mut state);
    let top = strip_top(&painted, "vs V(a)");
    let below: Vec<&(String, egui::Rect)> = painted
        .iter()
        .filter(|(_, rect)| rect.top() > top)
        .collect();
    // The ruler's own words stand under the time strip alone.
    for word in ["FULL", "LIN"] {
        assert_eq!(
            painted.iter().filter(|(text, _)| text == word).count(),
            1,
            "{word}: {painted:?}"
        );
        assert!(
            !below.iter().any(|(text, _)| text == word),
            "no ruler under the strip: {below:?}"
        );
    }
    assert!(!below.iter().any(|(text, _)| text.starts_with("TIME")));
    // Ticks in one row are a horizontal axis's; the Y axis' stand one above
    // another.
    let mut rows: std::collections::BTreeMap<i32, usize> = std::collections::BTreeMap::new();
    for (_, rect) in below.iter().filter(|(text, _)| tick(text).is_some()) {
        *rows.entry(rect.center().y.round() as i32).or_default() += 1;
    }
    let axes: Vec<_> = rows.iter().filter(|(_, count)| **count >= 3).collect();
    assert_eq!(axes.len(), 1, "one row of horizontal ticks: {below:?}");
    assert_eq!(
        below
            .iter()
            .filter(|(text, _)| text.starts_with("V(a) \u{b7}"))
            .count(),
        1,
        "one horizontal axis label: {below:?}"
    );
}

/// Making a strip against a signal leaves the strip drawn against time
/// with every curve, its legend and its time axis, and closing it leaves
/// the frame exactly as it was before.
#[test]
fn making_or_closing_a_strip_against_a_signal_leaves_the_time_strip_as_it_was() {
    let mut state = lissajous_host();
    let ctx = theme_context();
    let before = painted_texts(&ctx, &mut state);
    let before_shapes = frame(&ctx, &mut state, Vec::new());
    // What reads the time strip, but for the ticks of its value axes, which
    // follow its height: its header, legend, ruler and time ticks.
    let time_strip = |painted: &[(String, egui::Rect)], bottom: f32| {
        let mut read: Vec<String> = painted
            .iter()
            .filter(|(text, rect)| rect.bottom() <= bottom && tick(text).is_none())
            .map(|(text, _)| text.clone())
            .collect();
        read.sort();
        read
    };
    let whole = time_strip(&before, f32::INFINITY);

    open_strip(&mut state, "V", signal("V(a)"));
    let during = painted_texts(&ctx, &mut state);
    let top = strip_top(&during, "vs V(a)");
    assert_eq!(time_strip(&during, top), whole);
    assert!(
        during
            .iter()
            .filter(|(_, rect)| rect.bottom() <= top)
            .all(|(text, _)| !text.starts_with("vs ")),
        "the time strip says nothing of the strip after it"
    );

    state.results.results.session.xy_strips.clear();
    frame(&ctx, &mut state, Vec::new());
    let after = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        format!("{after:?}"),
        format!("{before_shapes:?}"),
        "closing the strip leaves the frame as it was"
    );
}

/// The full-width rectangles a frame filled with `color`.
fn filled_rects(shapes: &[ClippedShape], color: egui::Color32) -> Vec<egui::Rect> {
    fn walk(shape: &egui::Shape, color: egui::Color32, out: &mut Vec<egui::Rect>) {
        match shape {
            egui::Shape::Rect(rect) if rect.fill == color => out.push(rect.rect),
            egui::Shape::Vec(inner) => inner.iter().for_each(|shape| walk(shape, color, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, color, &mut out);
    }
    out.retain(|rect| rect.width() >= 0.9 * super::lanes::FRAME.x);
    out
}

/// Between the time strip and the strip after it lies a gap of canvas, then
/// a hairline in border ink on top of the next strip's header: the time
/// strip's overview track ends its own strip, and the next header plainly
/// starts another.
#[test]
fn strips_stand_apart_by_a_gap_of_canvas_and_a_rule() {
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let shapes = frame(&ctx, &mut state, Vec::new());
    let top = strip_top(&texts(&shapes), "vs V(a)");
    let color = rspice_ui_kit::tokens::Tokens::get(&ctx).color;
    let rule = filled_rects(&shapes, color.border)
        .into_iter()
        .find(|rect| {
            rect.height() == 1.0
                && rect.bottom() <= top
                && rect.bottom() >= top - rspice_results_ui::strip::HEADER_HEIGHT
        })
        .expect("a rule in border ink stands on top of the next strip's header");
    let gap = filled_rects(&shapes, color.canvas_bg)
        .into_iter()
        .find(|rect| rect.bottom() == rule.top())
        .expect("a gap of canvas lies over the rule");
    assert_eq!(gap.height(), 8.0, "the gap is 8 px of canvas: {gap:?}");
}

/// The header's span field narrows the curve to the time it names: typed
/// "250u" for its end, the strip draws the first quarter period alone, where
/// V(a) never goes negative.
#[test]
fn the_span_field_clips_the_curve_to_the_time_it_names() {
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    let ctx = theme_context();
    let negative_ticks = |painted: &[(String, egui::Rect)], top: f32| {
        painted
            .iter()
            .filter(|(text, rect)| rect.top() > top && tick(text).is_some_and(|value| value < 0.0))
            .count()
    };
    let painted = painted_texts(&ctx, &mut state);
    let top = strip_top(&painted, "vs V(a)");
    assert!(
        negative_ticks(&painted, top) > 0,
        "a whole period spans ±1 V"
    );

    let row = header_row(&painted, top);
    let end = span_end(&row)
        .map(|(_, rect)| rect.center())
        .unwrap_or_else(|| panic!("the header shows the record's end: {row:?}"));
    type_into(&ctx, &mut state, end, "250u");
    assert_eq!(strips(&state)[0].sweep_range, Some((0.0, 250.0e-6)));
    let painted = painted_texts(&ctx, &mut state);
    assert_eq!(negative_ticks(&painted, top), 0, "{painted:?}");
    let row = header_row(&painted, top);
    assert!(
        span_end(&row).is_some_and(|(text, _)| text.starts_with("250")),
        "the field reads the span it draws: {row:?}"
    );
}

/// The texts of the header row of the strip whose top is `top`, left to
/// right.
fn header_row(painted: &[(String, egui::Rect)], top: f32) -> Vec<(String, egui::Rect)> {
    let mut row: Vec<(String, egui::Rect)> = painted
        .iter()
        .filter(|(_, rect)| (rect.center().y - (top + 7.0)).abs() < 10.0)
        .cloned()
        .collect();
    row.sort_by(|a, b| a.1.left().total_cmp(&b.1.left()));
    row
}

/// The span field's end in a header row: the last text before the "follow"
/// toggle.
fn span_end(row: &[(String, egui::Rect)]) -> Option<&(String, egui::Rect)> {
    let follow = row.iter().position(|(text, _)| text == "follow")?;
    row[..follow].last()
}

/// Click the text box at `at`, replace what it holds with `text` and press
/// Enter.
fn type_into(ctx: &egui::Context, state: &mut ResultsTestHost, at: egui::Pos2, text: &str) {
    click(ctx, state, at, egui::PointerButton::Primary);
    let key = |key: egui::Key, modifiers: egui::Modifiers| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    };
    frame(
        ctx,
        state,
        vec![key(egui::Key::A, egui::Modifiers::COMMAND)],
    );
    frame(ctx, state, vec![egui::Event::Text(text.to_owned())]);
    frame(
        ctx,
        state,
        vec![key(egui::Key::Enter, egui::Modifiers::NONE)],
    );
}

/// Following, the strip draws the window the time strip shows: zoomed to the
/// first quarter period, where V(a) never goes negative, the strip draws that
/// quarter and its field reads it. Typing a span stops following, and the
/// strip draws the span typed.
#[test]
fn a_following_strip_draws_the_window_the_time_strip_shows() {
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    let ctx = theme_context();
    let negative_ticks = |painted: &[(String, egui::Rect)], top: f32| {
        painted
            .iter()
            .filter(|(text, rect)| rect.top() > top && tick(text).is_some_and(|value| value < 0.0))
            .count()
    };
    let painted = painted_texts(&ctx, &mut state);
    let top = strip_top(&painted, "vs V(a)");
    let row = header_row(&painted, top);
    let follow = row
        .iter()
        .find(|(text, _)| text == "follow")
        .map(|(_, rect)| rect.center())
        .unwrap_or_else(|| panic!("the header offers to follow: {row:?}"));
    assert!(
        !strips(&state)[0].follow_view,
        "a new strip does not follow"
    );
    click(&ctx, &mut state, follow, egui::PointerButton::Primary);
    assert!(strips(&state)[0].follow_view);

    let (analysis, pane_count) = {
        let models = super::lanes::strip(&mut state);
        (models[0].analysis_key, models[0].unit_panes().len())
    };
    rspice_results_ui::waves::stack::set_shared_x_view(
        &mut state.results.results.session,
        analysis,
        pane_count,
        Some((0.0, 250.0e-6)),
    );
    let painted = painted_texts(&ctx, &mut state);
    assert_eq!(negative_ticks(&painted, top), 0, "{painted:?}");
    let row = header_row(&painted, top);
    assert!(
        span_end(&row).is_some_and(|(text, _)| text.starts_with("250")),
        "the field reads the window it follows: {row:?}"
    );
    assert_eq!(
        strips(&state)[0].sweep_range,
        None,
        "following keeps no span of its own"
    );

    // Typed, the span is the strip's own again and it follows no longer.
    let end = span_end(&row).map(|(_, rect)| rect.center()).unwrap();
    type_into(&ctx, &mut state, end, "500u");
    assert!(
        !strips(&state)[0].follow_view,
        "typing a span stops following"
    );
    assert_eq!(strips(&state)[0].sweep_range, Some((0.0, 500.0e-6)));
    let painted = painted_texts(&ctx, &mut state);
    assert!(
        negative_ticks(&painted, top) > 0,
        "half a period spans ±1 V"
    );
}

/// Cursor A's point is taken hold of and dragged along the curve to where
/// B stands, half a period on: A moves to that instant. Moving A in time, as
/// the time strip does, moves the point along the curve to the closed form.
#[test]
fn dragging_a_cursor_point_moves_a_in_time_and_moving_a_moves_the_point() {
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    super::lanes::place_cursors(&mut state, Some(0.25e-3), Some(0.5e-3));
    let ctx = theme_context();
    let tokens = crate::ui::tokens::Tokens::get(&ctx);
    let top = strip_top(&painted_texts(&ctx, &mut state), "vs V(a)");
    let shapes = frame(&ctx, &mut state, Vec::new());
    let on_figure = |color| {
        dots(&shapes, color)
            .into_iter()
            .filter(|point| point.y > top)
            .collect::<Vec<_>>()
    };
    let a = on_figure(tokens.color.canvas_cursor_a);
    let b = on_figure(tokens.color.canvas_cursor_b);
    assert_eq!(
        (a.len(), b.len()),
        (1, 1),
        "one point per cursor on the figure"
    );
    drag(&ctx, &mut state, a[0], b[0]);
    // The sample nearest B's point is the one at half a period.
    assert_eq!(
        state.results.results.session.cursors.a,
        Some(f64::from(200) * 2.5e-6)
    );
    assert_eq!(state.results.results.session.cursors.b, Some(0.5e-3));

    // A moved in time to an eighth of a period stands at sin(π/4) against
    // sin(π/4 + π/3).
    state.results.results.session.cursors.a = Some(0.125e-3);
    let painted: Vec<String> = painted_texts(&ctx, &mut state)
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    assert!(
        painted
            .iter()
            .any(|text| text.starts_with("A \u{b7} t 125 ")
                && text.ends_with("s \u{b7} V(a) 707 mV \u{b7} 966 mV")),
        "{painted:?}"
    );
}

/// A Lissajous figure carries sparse arrowheads in its own colour, all on
/// the plot, and its reverse traversal points them the other way.
#[test]
fn a_lissajous_figure_carries_arrows_the_way_time_runs() {
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let shapes = frame(&ctx, &mut state, Vec::new());
    let color = super::lanes::strip(&mut state)[0]
        .traces
        .iter()
        .find(|trace| trace.name == "V(b)")
        .expect("V(b) is plotted")
        .color;
    let arrows = arrowheads(&shapes, color);
    assert!(
        (3..=24).contains(&arrows.len()),
        "an ellipse a few hundred points round carries a few arrows: {}",
        arrows.len()
    );
    // V(b) leads V(a), so the figure is travelled clockwise, as x = sin t,
    // y = cos t is: from (0, 0.87) at t = 0 both rise. On a screen whose Y
    // grows downward a clockwise turn has a positive cross product, and the
    // tip of every arrow turns about the figure's centre that way.
    let centre = arrows
        .iter()
        .flatten()
        .fold(egui::Vec2::ZERO, |sum, point| sum + point.to_vec2())
        / (arrows.len() * 3) as f32;
    for arrow in &arrows {
        let base = (arrow[1].to_vec2() + arrow[2].to_vec2()) * 0.5;
        let forward = arrow[0].to_vec2() - base;
        let radius = base - centre;
        let turn = radius.x * forward.y - radius.y * forward.x;
        assert!(turn > 0.0, "an arrow at {base:?} turns the wrong way");
    }
}

/// A family on a strip against a signal is never folded into a band: past
/// 64 members every loop is a faint 1 px hairline and none carries arrows
/// until the reader points at one, which is drawn over the others at full
/// weight with its arrows. At 50 every member is stroked and carries its
/// own.
#[test]
fn a_large_family_against_a_signal_is_drawn_as_faint_hairlines() {
    let draw = |members: usize, pointer: Option<egui::Pos2>| {
        let mut state = super::frame_cost::family_of_signals(members, 200, &["V(in)", "V(out)"]);
        // The workspace picks the viewer a corner family is read in.
        crate::workbench::documents::result_document::prepare_viewer_state(
            &mut state.results_parts(),
        );
        open_strip(&mut state, "V", signal("V(in)"));
        let color = super::lanes::strip(&mut state)[0]
            .traces
            .iter()
            .find(|trace| trace.source_waveform_name == "V(out)")
            .expect("the family plots V(out)")
            .color;
        let ctx = theme_context();
        let before = state.results.results.session.cache.work();
        let strokes = hairlines(&frame(&ctx, &mut state, Vec::new()));
        let stroked = state
            .results
            .results
            .session
            .cache
            .work()
            .since(before)
            .rebuilds;
        let events = pointer.map_or_else(Vec::new, |at| vec![egui::Event::PointerMoved(at)]);
        frame(&ctx, &mut state, events);
        let shapes = frame(&ctx, &mut state, Vec::new());
        let painted = texts(&shapes);
        (
            arrowheads(&shapes, color).len(),
            (strokes, stroked),
            painted,
        )
    };
    let (arrows, (hairlines, _), painted) = draw(100, None);
    assert!(
        hairlines >= 100,
        "each of a hundred loops is a faint hairline: {hairlines}"
    );
    assert_eq!(
        arrows, 0,
        "no member carries arrows while none is pointed at"
    );
    // Point where the loops crowd: each member is a circle of radius about
    // 1 V about the origin, so at V(in) = 1 V, V(out) = 0, read off the
    // strip's own ticks.
    let top = strip_top(&painted, "vs V(in)");
    let ticks: Vec<(f64, egui::Pos2)> = painted
        .iter()
        .filter(|(_, rect)| rect.top() > top)
        .filter_map(|(text, rect)| Some((tick(text)?, rect.center())))
        .collect();
    let bottom = ticks
        .iter()
        .map(|(_, at)| at.y)
        .fold(f32::NEG_INFINITY, f32::max);
    let on_x = |value: f64| {
        ticks
            .iter()
            .find(|(held, at)| *held == value && (at.y - bottom).abs() < 1.0)
            .map(|(_, at)| at.x)
            .unwrap_or_else(|| panic!("the X axis ticks {value}: {ticks:?}"))
    };
    let zero_y = ticks
        .iter()
        .find(|(held, at)| *held == 0.0 && (at.y - bottom).abs() >= 1.0)
        .map(|(_, at)| at.y)
        .expect("the Y axis ticks 0");
    let centre = egui::pos2(on_x(0.0) + 2.0 * (on_x(0.5) - on_x(0.0)), zero_y);
    let (arrows, ..) = draw(100, Some(centre));
    assert!(arrows > 0, "the member pointed at carries its arrows");
    let (arrows, (hairlines, stroked), _) = draw(50, None);
    // The strip drawn against time may paint a faint line of its own; fifty
    // loops are no crowd, and none of them is one.
    assert!(hairlines < 50, "fifty loops are no crowd: {hairlines}");
    assert!(stroked >= 50, "fifty loops are each stroked: {stroked}");
    assert!(arrows >= 50, "each of fifty loops carries arrows: {arrows}");
}

/// Cursor A at a quarter period stands where sin(ωt) = 1: its tag reads the
/// instant, V(a) = 1 V and V(b) = sin(π/2 + π/3) = 500 mV, the closed form.
#[test]
fn a_cursor_on_the_figure_reads_the_instant_and_both_coordinates() {
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    super::lanes::place_cursors(&mut state, Some(0.25e-3), Some(0.5e-3));
    let ctx = theme_context();
    let painted: Vec<String> = painted_texts(&ctx, &mut state)
        .into_iter()
        .map(|(text, _)| text)
        .collect();
    assert!(
        painted
            .iter()
            .any(|text| text.starts_with("A \u{b7} t 250 ")
                && text.ends_with("s \u{b7} V(a) 1 V \u{b7} 500 mV")),
        "{painted:?}"
    );
    // At half a period sin(ωt) is 1.2e-16 V, which the ±1.08 V axis
    // resolves as 0 V, and sin(π + π/3) = −√3/2 reads −866 mV, both to the
    // axis' millivolt.
    assert!(
        painted
            .iter()
            .any(|text| text.starts_with("B \u{b7} t 500 ")
                && text.ends_with("s \u{b7} V(a) 0 V \u{b7} -866 mV")),
        "{painted:?}"
    );
}

/// The quick view's strips are the project's: saved with the result
/// presentation, span and all, and drawn again when the project is
/// reopened.
#[test]
fn the_project_keeps_a_strip_against_a_signal_and_its_span() {
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    state.results.results.session.xy_strips[0].sweep_range = Some((0.25e-3, 0.75e-3));
    let analysis = super::lanes::strip(&mut state)[0].analysis_key;
    let saved = state
        .results
        .results
        .project_presentation(&state.model.simulation);
    assert_eq!(
        saved.xy_plots,
        [rspice_results::result_presentation::StripXyPlot {
            analysis,
            unit: "V".to_owned(),
            abscissa: signal("V(a)"),
            sweep_range: Some(
                rspice_results::visualization_document::AxisRange::new(0.25e-3, 0.75e-3).unwrap()
            ),
            follow_sweep_view: false,
        }]
    );
    state.results.results.session.xy_strips.clear();
    crate::workbench::documents::result_document::restore_presentation(
        &mut state.results_parts(),
        serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap(),
    );
    let mut expected = XyStrip::new(analysis, "V", signal("V(a)"));
    expected.sweep_range = Some((0.25e-3, 0.75e-3));
    assert_eq!(strips(&state), [expected]);
}

/// A saved view keeps its strip against a signal, the strip's span and
/// whether it follows the time strip, draws them when shown, keeps a change
/// made inside it, and hands the quick view its own strip back when it is
/// left.
#[test]
fn a_saved_view_keeps_its_strip_against_a_signal_and_the_span() {
    use super::super::super::persistent_document::{
        capture_pane_presentation, project_pane_presentation, projection,
    };
    use super::expression_units::{document_frame, save_single_pane_view};
    let mut state = lissajous_host();
    open_strip(&mut state, "V", signal("V(a)"));
    state.results.results.session.xy_strips[0].sweep_range = Some((0.0, 0.5e-3));
    let document_id = save_single_pane_view(&mut state);
    let kept = |state: &ResultsTestHost| {
        state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("the project keeps the document")
            .xy_plots()
            .iter()
            .map(|plot| {
                (
                    plot.unit.clone(),
                    plot.abscissa.text().to_owned(),
                    plot.sweep_range.map(|range| (range.minimum, range.maximum)),
                    plot.follow_sweep_view,
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        kept(&state),
        [(
            "V".to_owned(),
            "V(a)".to_owned(),
            Some((0.0, 0.5e-3)),
            false
        )]
    );
    document_frame(&mut state, document_id);

    // Inside the view's frame, between its projection and its capture, the
    // reader plots against V(b) instead, over the second half period, and
    // has the strip follow the time strip's window.
    let pane = projection(state.results_reads(), document_id)
        .expect("the document projects")
        .pages[0]
        .panes[0]
        .clone();
    let viewer = crate::workbench::ResultViewer::from_viewer_document_id(&pane.viewer_id)
        .expect("the pane names a Results viewer");
    project_pane_presentation(&mut state.results_parts(), &pane, viewer)
        .expect("the pane projects onto its strip");
    {
        let strip = &mut state.results.results.session.xy_strips[0];
        strip.abscissa = signal("V(b)");
        strip.sweep_range = Some((0.5e-3, 1.0e-3));
        strip.follow_view = true;
    }
    capture_pane_presentation(&mut state.results_parts(), &pane, viewer);
    assert_eq!(
        kept(&state),
        [(
            "V".to_owned(),
            "V(b)".to_owned(),
            Some((0.5e-3, 1.0e-3)),
            true
        )]
    );
    let reloaded: rspice_results::visualization_document::VisualizationDocument =
        serde_json::from_str(
            &serde_json::to_string(
                state
                    .model
                    .workspace
                    .content
                    .visualization_document(document_id)
                    .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(reloaded.xy_plots()[0].abscissa, signal("V(b)"));
    assert!(reloaded.xy_plots()[0].follow_sweep_view);

    state.results.results.session.leave_persistent_document();
    let held = |state: &ResultsTestHost| {
        strips(state)
            .iter()
            .map(|strip| {
                (
                    strip.abscissa.text().to_owned(),
                    strip.sweep_range,
                    strip.follow_view,
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        held(&state),
        [("V(a)".to_owned(), Some((0.0, 0.5e-3)), false)],
        "the quick view has its strip back"
    );
    // Shown again, the view draws its own.
    document_frame(&mut state, document_id);
    assert_eq!(
        held(&state),
        [("V(b)".to_owned(), Some((0.5e-3, 1.0e-3)), true)]
    );
    // The strip closed inside the view, the view forgets it.
    project_pane_presentation(&mut state.results_parts(), &pane, viewer)
        .expect("the pane projects onto its strip");
    state.results.results.session.xy_strips.clear();
    capture_pane_presentation(&mut state.results_parts(), &pane, viewer);
    assert!(kept(&state).is_empty());
}

/// A host on a run the real controller solved, its first analysis active.
fn solved_host(deck: &str) -> ResultsTestHost {
    let run = rspice_app_runs::simulation::controller::test_execution::run_manual_batch(deck);
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state
}

fn trace<'a>(model: &'a StripModel, name: &str) -> &'a rspice_results_ui::waves::StripTrace {
    model
        .traces
        .iter()
        .find(|trace| trace.source_waveform_name == name)
        .unwrap_or_else(|| panic!("the strip plots {name}"))
}

/// An NMOS output family, Id against Vds for three gate voltages, from a
/// nested DC sweep: each member's curve is that member's drain current
/// against its own drain voltage, sample for sample, and in saturation it
/// is the level-1 square law.
#[test]
fn a_mosfet_family_plots_each_members_drain_current_against_its_drain_voltage() {
    const KP: f64 = 100.0e-6;
    const W_OVER_L: f64 = 10.0;
    const VTO: f64 = 0.7;
    const LAMBDA: f64 = 0.02;
    let mut state = solved_host(
        "nmos output family\n\
         VD d 0 DC 0\n\
         VG g 0 DC 1\n\
         M1 d g 0 0 nch W=10u L=1u\n\
         .model nch nmos (level=1 vto=0.7 kp=100u lambda=0.02)\n\
         .dc VD 0 3 0.05 VG 1.5 2.5 0.5\n\
         .end\n",
    );
    state.results.results.session.viewer =
        rspice_results::result_presentation::ResultViewer::DcSweep;
    let models = super::lanes::strip(&mut state);
    let model = &models[0];
    let members = [("1.5", 1), ("2", 2), ("2.5", 3)];
    for (vgs, point) in members {
        let suffix = format!(" [VG={vgs}; point {point}]");
        let id = trace(model, &format!("I(VD){suffix}"));
        let vd = trace(model, &format!("V(D){suffix}"));
        let curve = rspice_results_ui::waves::xy::resample(
            (vd.x.as_slice(), vd.y.as_slice()),
            (id.x.as_slice(), id.y.as_slice()),
            crate::ui::plot::SampleInterpolation::Linear,
        )
        .expect("one sweep pairs");
        assert!(!curve.interpolated);
        assert_eq!(curve.x.len(), 61, "Vds from 0 to 3 V in 50 mV steps");
        for (k, (&x, &y)) in curve.x.iter().zip(&curve.y).enumerate() {
            assert_eq!(x.to_bits(), vd.y[k].to_bits());
            assert_eq!(y.to_bits(), id.y[k].to_bits());
            assert!((x - 0.05 * k as f64).abs() < 1.0e-9, "Vds[{k}] = {x}");
        }
        // Saturation at Vds = 3 V: Id = kp/2 · W/L · (Vgs − Vto)² · (1 + λ·Vds).
        let gate: f64 = vgs.parse().unwrap();
        let expected = 0.5 * KP * W_OVER_L * (gate - VTO).powi(2) * (1.0 + LAMBDA * 3.0);
        let solved = curve.y[60].abs();
        assert!(
            (solved - expected).abs() <= 1.0e-6 * expected,
            "Vgs = {gate}: Id(3 V) = {solved}, the square law gives {expected}"
        );
    }

    // The amperes curves plotted against the first member's drain voltage
    // draw one curve per member. Each member's Vds only rises, so the axis
    // says which way it travels and no member carries arrows.
    let colors: Vec<egui::Color32> = members
        .iter()
        .map(|(vgs, point)| trace(model, &format!("I(VD) [VG={vgs}; point {point}]")).color)
        .collect();
    drop(models);
    open_strip(&mut state, "A", signal("V(D) [VG=1.5; point 1]"));
    let ctx = theme_context();
    frame(&ctx, &mut state, Vec::new());
    let shapes = frame(&ctx, &mut state, Vec::new());
    for color in colors {
        assert!(
            arrowheads(&shapes, color).is_empty(),
            "a member drawn along a rising sweep carries no arrows"
        );
    }
    // Every member reads against the drain voltage of its own point, so the
    // strip names the quantity alone, never the column it was chosen from.
    let painted = painted_texts(&ctx, &mut state);
    assert!(
        painted.iter().any(|(text, _)| text == "vs V(D)"),
        "the strip header reads \"vs V(D)\": {painted:?}"
    );
    assert!(
        painted
            .iter()
            .any(|(text, _)| text.starts_with("V(D) \u{b7} at each ")),
        "the X axis caption reads \"V(D) \u{b7} at each … sample\": {painted:?}"
    );
    assert!(
        !painted.iter().any(|(text, _)| text.starts_with("vs V(D) [")
            || text.starts_with("V(D) [VG=1.5; point 1] \u{b7}")),
        "neither names the member column it was chosen from: {painted:?}"
    );
}

/// Vout against Vin from a slow transient ramp through a resistor into a
/// diode is the circuit's DC transfer curve: at every DC sweep point inside
/// the ramp the two agree within the transient's tolerance, reltol · |V| +
/// vntol.
#[test]
fn a_transient_ramp_drawn_against_its_input_is_the_dc_transfer_curve() {
    let circuit = "VIN in 0 PWL(0 0 1m 2)\n\
                   R1 in out 1k\n\
                   D1 out 0 dmod\n\
                   .model dmod d (is=1e-14 n=1)\n";
    let mut transient = solved_host(&format!("diode transfer\n{circuit}.tran 1u 1m\n.end\n"));
    let dc = solved_host(&format!(
        "diode transfer\n{}.dc VIN 0 2 0.01\n.end\n",
        circuit.replace("PWL(0 0 1m 2)", "DC 0")
    ));
    let dc_run = &dc.model.simulation.retained.runs[0].analyses[0];
    let dc_out = dc_run
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "V(OUT)")
        .expect("the sweep keeps V(OUT)");
    let models = super::lanes::strip(&mut transient);
    // The transient names its node columns bare; the DC sweep wraps them.
    let vin = trace(&models[0], "IN");
    let vout = trace(&models[0], "OUT");
    let curve = rspice_results_ui::waves::xy::resample(
        (vin.x.as_slice(), vin.y.as_slice()),
        (vout.x.as_slice(), vout.y.as_slice()),
        crate::ui::plot::SampleInterpolation::Linear,
    )
    .expect("one transient grid pairs");
    let mut compared = 0;
    for (&v_in, &v_out) in dc_out.x.iter().zip(dc_out.y.iter()) {
        if !(0.05..=1.95).contains(&v_in) {
            continue;
        }
        let drawn = rspice_results::sampling::sample_at_with(
            &curve.x,
            &curve.y,
            v_in,
            crate::ui::plot::SampleInterpolation::Linear,
        );
        let tolerance = 1.0e-3 * v_out.abs() + 1.0e-6;
        assert!(
            (drawn - v_out).abs() <= tolerance,
            "at Vin = {v_in}: the transient draws {drawn}, the DC sweep {v_out}"
        );
        compared += 1;
    }
    assert!(compared >= 180, "{compared} sweep points compared");

    // Drawn, the curve carries no arrows: Vin only rises along the ramp, so
    // the axis already says which way the transient travels.
    let color = vout.color;
    drop(models);
    open_strip(&mut transient, "V", signal("IN"));
    let ctx = theme_context();
    frame(&ctx, &mut transient, Vec::new());
    assert!(arrowheads(&frame(&ctx, &mut transient, Vec::new()), color).is_empty());
}

/// Every text a frame painted, with the face and the ink of its first run.
fn styled_texts(shapes: &[ClippedShape]) -> Vec<(String, egui::Rect, egui::FontId, egui::Color32)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect, egui::FontId, egui::Color32)>) {
        match shape {
            egui::Shape::Text(text) => {
                if let Some(section) = text.galley.job.sections.first() {
                    out.push((
                        text.galley.job.text.clone(),
                        egui::Rect::from_min_size(text.pos, text.galley.size()),
                        section.format.font_id.clone(),
                        section.format.color,
                    ));
                }
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

/// The name of the quantity before the span field, "t" under a transient,
/// "VD" under a DC sweep of VD and "TEMP" under a temperature sweep, and the
/// dash between the ends, are the header's own words: the subtitle's face,
/// size and ink.
#[test]
fn the_span_fields_quantity_reads_in_the_headers_words() {
    let dc = |deck: &str| {
        let mut state = solved_host(deck);
        state.results.results.session.viewer =
            rspice_results::result_presentation::ResultViewer::DcSweep;
        state
    };
    let cases = [
        (lissajous_host(), "V", "V(a)", "t"),
        (
            dc("source sweep\nVD d 0 DC 0\nR1 d 0 1k\n.dc VD 0 3 0.1\n.end\n"),
            "A",
            "V(D)",
            "VD",
        ),
        (
            dc("temperature sweep\nV1 a 0 DC 1\nR1 a 0 1k\n.dc TEMP -40 125 5\n.end\n"),
            "A",
            "V(A)",
            "TEMP",
        ),
    ];
    for (mut state, unit, abscissa, quantity) in cases {
        open_strip(&mut state, unit, signal(abscissa));
        let ctx = theme_context();
        frame(&ctx, &mut state, Vec::new());
        let painted = styled_texts(&frame(&ctx, &mut state, Vec::new()));
        let subtitle = format!("vs {abscissa}");
        let (_, header, font, ink) = painted
            .iter()
            .find(|(text, ..)| *text == subtitle)
            .unwrap_or_else(|| panic!("a strip header reads {subtitle:?}: {painted:?}"));
        for word in [quantity, "\u{2013}"] {
            let (_, _, word_font, word_ink) = painted
                .iter()
                .find(|(text, rect, ..)| {
                    text == word && (rect.center().y - header.center().y).abs() < 4.0
                })
                .unwrap_or_else(|| panic!("the header row reads {word:?}: {painted:?}"));
            assert_eq!(
                (word_font, word_ink),
                (font, ink),
                "{word:?} reads in the subtitle's face, size and ink"
            );
        }
    }
}
