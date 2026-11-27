//! Plot gestures driven through egui's own input path: raw events into a
//! context frame, with the caller applying each frame's view change as a
//! host does, and the resulting window asserted.

use super::*;
use crate::plot::{Axis, Trace, XScale, fmt_si_significant};

const SIZE: egui::Vec2 = egui::vec2(480.0, 300.0);
const X0: (f64, f64) = (0.0, 1.0);
const Y0: (f64, f64) = (-1.0, 1.0);

/// The modifiers egui-winit and the browser both report for a held Ctrl on
/// Windows and Linux: Ctrl, which is also the command key there.
const CTRL: egui::Modifiers = egui::Modifiers {
    alt: false,
    ctrl: true,
    shift: false,
    mac_cmd: false,
    command: true,
};

/// What one frame of the plot reported.
struct Frame {
    cursor_drag: Option<(CursorId, f64)>,
    plot_rect: Rect,
    id: egui::Id,
    shapes: Vec<egui::epaint::ClippedShape>,
}

/// A host of one plot: it owns the window and applies every change the plot
/// reports, frame by frame, as a results sheet does.
struct Host {
    ctx: egui::Context,
    cache: DecimationCache,
    x: (f64, f64),
    y: (f64, f64),
    time: f64,
    cursors: Option<CursorPair>,
    horizontal: [Option<f64>; 2],
    both_axes: bool,
    trace: (Vec<f64>, Vec<f64>),
}

impl Host {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        let x: Vec<f64> = (0..=200).map(|index| f64::from(index) / 200.0).collect();
        let y = x.iter().map(|x| (6.0 * x).sin()).collect();
        Self {
            ctx,
            cache: DecimationCache::default(),
            x: X0,
            y: Y0,
            time: 0.0,
            cursors: None,
            horizontal: [None; 2],
            both_axes: false,
            trace: (x, y),
        }
    }

    fn frame_with(&mut self, events: Vec<egui::Event>, modifiers: egui::Modifiers) -> Frame {
        self.time += 1.0 / 60.0;
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SIZE)),
            time: Some(self.time),
            modifiers,
            events,
            ..Default::default()
        };
        let mut spec = PlotSpec::new(
            Axis::linear(self.x.0, self.x.1, "s"),
            XScale::Linear,
            Axis::linear(self.y.0, self.y.1, "V"),
        );
        let trace = Trace::new(&self.trace.0, &self.trace.1, Color32::WHITE);
        spec.traces.push(trace);
        spec.horizontal_cursors = self.horizontal;
        if self.both_axes {
            spec = spec.pinch_zooms_both_axes();
        }
        let cursors = self.cursors;
        let mut reported = None;
        let output = self.ctx.run_ui(input, |ui| {
            let out = show(ui, &spec, &mut self.cache, cursors.as_ref(), None);
            reported = Some((out.view, out.cursor_drag, out.plot_rect, out.response.id));
            // A widget below the plot, for the focus to move to if the
            // plot let it.
            let _ = ui.button("next");
        });
        let (view, cursor_drag, plot_rect, id) = reported.expect("the plot was drawn");
        if view.reset {
            (self.x, self.y) = (X0, Y0);
        }
        self.x = view.x.unwrap_or(self.x);
        self.y = view.y.unwrap_or(self.y);
        Frame {
            cursor_drag,
            plot_rect,
            id,
            shapes: output.shapes,
        }
    }

    fn frame(&mut self, events: Vec<egui::Event>) -> Frame {
        self.frame_with(events, egui::Modifiers::NONE)
    }

    /// Let egui's wheel smoothing hand over everything it holds.
    fn settle(&mut self, modifiers: egui::Modifiers) {
        for _ in 0..60 {
            self.frame_with(Vec::new(), modifiers);
        }
    }

    fn plot_rect(&mut self) -> Rect {
        self.frame(Vec::new()).plot_rect
    }

    fn data_x(&self, plot: Rect, px: f32) -> f64 {
        self.x.0 + (self.x.1 - self.x.0) * f64::from((px - plot.left()) / plot.width())
    }

    fn data_y(&self, plot: Rect, py: f32) -> f64 {
        self.y.0 + (self.y.1 - self.y.0) * f64::from((plot.bottom() - py) / plot.height())
    }
}

fn wheel(lines: f32, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::MouseWheel {
        unit: egui::MouseWheelUnit::Line,
        delta: egui::vec2(0.0, lines),
        phase: egui::TouchPhase::Move,
        modifiers,
    }
}

fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}

fn button(at: Pos2, pressed: bool, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers,
    }
}

fn span(range: (f64, f64)) -> f64 {
    range.1 - range.0
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1.0e-9 * a.abs().max(b.abs()).max(1.0)
}

/// Drag the primary button from `from` to `to`, with the threshold crossed
/// in a frame of its own first, as the hand does.
fn drag(host: &mut Host, from: Pos2, to: Pos2, modifiers: egui::Modifiers) {
    host.frame_with(vec![egui::Event::PointerMoved(from)], modifiers);
    host.frame_with(
        vec![
            egui::Event::PointerMoved(from),
            button(from, true, modifiers),
        ],
        modifiers,
    );
    let step = from + (to - from) * 0.25;
    host.frame_with(vec![egui::Event::PointerMoved(step)], modifiers);
    host.frame_with(vec![egui::Event::PointerMoved(to)], modifiers);
    host.frame_with(
        vec![egui::Event::PointerMoved(to), button(to, false, modifiers)],
        modifiers,
    );
}

/// A Ctrl+wheel arrives as egui produces it: a wheel event carrying Ctrl,
/// which egui turns into its zoom factor and reports no scroll for. It zooms
/// Y about the pointer at the rate the wheel zooms X, and leaves X alone.
#[test]
fn ctrl_wheel_zooms_y_about_the_pointer_and_leaves_x_alone() {
    let mut host = Host::new();
    let plot = host.plot_rect();
    let at = plot.left_top() + egui::vec2(plot.width() * 0.3, plot.height() * 0.25);
    host.frame(vec![egui::Event::PointerMoved(at)]);
    let level = host.data_y(plot, at.y);

    host.frame_with(vec![wheel(1.0, CTRL)], CTRL);
    host.settle(CTRL);

    assert_eq!(host.x, X0, "Ctrl+wheel moved X");
    // One line of wheel is egui's 40 points, which the plain wheel turns
    // into a span of exp(-0.002 × 40).
    let expected = span(Y0) * (-0.002f64 * 40.0).exp();
    assert!(
        (span(host.y) - expected).abs() < 1.0e-3 * expected,
        "Y span {} after one line of Ctrl+wheel, expected {expected}",
        span(host.y)
    );
    assert!(
        (host.data_y(plot, at.y) - level).abs() < 1.0e-6,
        "the level under the pointer moved"
    );
}

/// With Shift as well, Ctrl+wheel zooms both axes by the same factor.
#[test]
fn ctrl_shift_wheel_zooms_both_axes() {
    let mut host = Host::new();
    let plot = host.plot_rect();
    let both = egui::Modifiers {
        shift: true,
        ..CTRL
    };
    host.frame(vec![egui::Event::PointerMoved(plot.center())]);
    host.frame_with(vec![wheel(1.0, both)], both);
    host.settle(both);

    let factor = (-0.002f64 * 40.0).exp();
    assert!((span(host.x) - span(X0) * factor).abs() < 1.0e-3);
    assert!((span(host.y) - span(Y0) * factor).abs() < 2.0e-3);
}

/// The plain wheel keeps zooming X at the pointer, and Shift+wheel pans X
/// with the span kept, the content following the wheel.
#[test]
fn the_wheel_zooms_x_and_shift_wheel_pans_it() {
    let mut host = Host::new();
    let plot = host.plot_rect();
    host.frame(vec![egui::Event::PointerMoved(plot.center())]);
    host.frame(vec![wheel(1.0, egui::Modifiers::NONE)]);
    host.settle(egui::Modifiers::NONE);
    assert!(span(host.x) < span(X0) * 0.95, "the wheel did not zoom X");
    assert_eq!(host.y, Y0, "the wheel moved Y");

    let before = host.x;
    host.frame_with(
        vec![wheel(-1.0, egui::Modifiers::SHIFT)],
        egui::Modifiers::SHIFT,
    );
    host.settle(egui::Modifiers::SHIFT);
    assert!(
        close(span(host.x), span(before)),
        "Shift+wheel changed the span"
    );
    // Wheel down is 40 points of content moving left: the window moves
    // right by that share of the plot.
    let moved = (host.x.0 - before.0) / span(before);
    assert!(
        (moved - 40.0 / f64::from(plot.width())).abs() < 1.0e-3,
        "Shift+wheel moved the window by {moved} of its span"
    );
    assert_eq!(host.y, Y0);
}

/// A trackpad pinch arrives as a zoom event. On a waveform it zooms X about
/// the pointer; on a chart that asks for it, both axes.
#[test]
fn a_pinch_zooms_x_on_a_waveform_and_both_axes_where_asked() {
    let mut host = Host::new();
    let plot = host.plot_rect();
    let at = plot.left_top() + egui::vec2(plot.width() * 0.75, plot.height() * 0.5);
    host.frame(vec![egui::Event::PointerMoved(at)]);
    let instant = host.data_x(plot, at.x);
    host.frame(vec![egui::Event::Zoom(1.25)]);
    assert!(close(span(host.x), span(X0) / 1.25), "X span {:?}", host.x);
    assert!(
        close(host.data_x(plot, at.x), instant),
        "the instant under the pinch moved"
    );
    assert_eq!(host.y, Y0, "a waveform pinch moved Y");

    let mut chart = Host::new();
    chart.both_axes = true;
    chart.frame(vec![egui::Event::PointerMoved(at)]);
    chart.frame(vec![egui::Event::Zoom(1.25)]);
    assert!(close(span(chart.x), span(X0) / 1.25));
    assert!(close(span(chart.y), span(Y0) / 1.25));
}

/// Over the Y gutter the wheel zooms that axis, and a drag there pans it
/// alone.
#[test]
fn the_y_gutter_zooms_and_pans_its_axis() {
    let mut host = Host::new();
    let plot = host.plot_rect();
    let gutter = pos2(plot.left() - 20.0, plot.center().y);
    host.frame(vec![egui::Event::PointerMoved(gutter)]);
    host.frame(vec![wheel(1.0, egui::Modifiers::NONE)]);
    host.settle(egui::Modifiers::NONE);
    assert_eq!(host.x, X0, "the gutter wheel moved X");
    assert!(
        span(host.y) < span(Y0) * 0.95,
        "the gutter wheel did not zoom Y"
    );

    let before = host.y;
    drag(
        &mut host,
        gutter,
        gutter + egui::vec2(30.0, 60.0),
        egui::Modifiers::NONE,
    );
    assert_eq!(host.x, X0, "a gutter drag panned X");
    assert!(
        close(span(host.y), span(before)),
        "a gutter drag changed the Y span"
    );
    assert!(
        host.y.0 > before.0,
        "a downward gutter drag did not raise the window"
    );
}

/// A box zoom reads its band's shape: flat zooms X, tall zooms Y, anything
/// else both. A one-axis zoom asked for first, as the X key's command asks
/// for it, makes the next band zoom that axis alone.
#[test]
fn a_zoom_band_picks_its_axes_by_shape_or_by_the_armed_axis() {
    let band = |size: egui::Vec2, arm: Option<OneAxisZoom>| {
        let mut host = Host::new();
        let plot = host.plot_rect();
        let from = plot.left_top() + egui::vec2(40.0, 40.0);
        host.frame(vec![egui::Event::PointerMoved(from)]);
        if let Some(arm) = arm {
            request_one_axis_zoom(&host.ctx, arm);
            host.frame(Vec::new());
        }
        drag(&mut host, from, from + size, egui::Modifiers::SHIFT);
        (host.x != X0, host.y != Y0)
    };
    assert_eq!(
        band(egui::vec2(160.0, 6.0), None),
        (true, false),
        "a flat band"
    );
    assert_eq!(
        band(egui::vec2(6.0, 120.0), None),
        (false, true),
        "a tall band"
    );
    assert_eq!(band(egui::vec2(160.0, 120.0), None), (true, true), "a box");
    assert_eq!(
        band(egui::vec2(160.0, 120.0), Some(OneAxisZoom::X)),
        (true, false),
        "a box after X"
    );
    assert_eq!(
        band(egui::vec2(160.0, 120.0), Some(OneAxisZoom::Y)),
        (false, true),
        "a box after Y"
    );
}

/// The plot answers no letter for a one-axis zoom: the workspace binds the
/// keys to its commands, which ask for it. Asked for after the pointer has
/// left for a menu, the zoom is armed on the plot the pointer last stood
/// over, and the plot says so; asked for again, it is disarmed.
#[test]
fn a_one_axis_zoom_is_asked_for_not_keyed_and_lands_on_the_last_plot_pointed_at() {
    let mut host = Host::new();
    let plot = host.plot_rect();
    host.frame(vec![egui::Event::PointerMoved(plot.center())]);
    let frame = host.frame(vec![key(egui::Key::X, egui::Modifiers::NONE)]);
    assert!(
        !texts(&frame.shapes).iter().any(|text| text == "zoom X"),
        "the bare X key armed the plot"
    );
    host.frame(vec![egui::Event::PointerGone]);
    request_one_axis_zoom(&host.ctx, OneAxisZoom::X);
    let frame = host.frame(Vec::new());
    assert!(
        texts(&frame.shapes).iter().any(|text| text == "zoom X"),
        "the zoom asked for did not arm the plot last pointed at"
    );
    request_one_axis_zoom(&host.ctx, OneAxisZoom::X);
    let frame = host.frame(Vec::new());
    assert!(
        !texts(&frame.shapes).iter().any(|text| text == "zoom X"),
        "asked for again, the zoom stayed armed"
    );
}

/// With the plot focused, each key changes the window as its mouse twin
/// does: an arrow as a drag of a tenth of the plot, plus and minus as a zoom
/// about the centre, Ctrl with them on Y, and X then Enter zooms the band
/// between the cursors.
#[test]
fn the_keys_of_a_focused_plot_pan_and_zoom_as_the_pointer_does() {
    let focused = || {
        let mut host = Host::new();
        let frame = host.frame(Vec::new());
        host.ctx.memory_mut(|memory| memory.request_focus(frame.id));
        host.frame(Vec::new());
        host.frame(Vec::new());
        (host, frame.plot_rect)
    };

    let (mut keys, plot) = focused();
    keys.frame(vec![key(egui::Key::ArrowRight, egui::Modifiers::NONE)]);
    let mut pointer = Host::new();
    let from = plot.center();
    drag(
        &mut pointer,
        from,
        from - egui::vec2(plot.width() * 0.1, 0.0),
        egui::Modifiers::NONE,
    );
    assert!(
        // The drag is measured in f32 screen points.
        (keys.x.0 - pointer.x.0).abs() < 1.0e-6 && (keys.x.1 - pointer.x.1).abs() < 1.0e-6,
        "Right panned to {:?}, a tenth-of-the-plot drag to {:?}",
        keys.x,
        pointer.x
    );
    assert!(close(keys.x.0, 0.1), "Right panned to {:?}", keys.x);
    assert_eq!(keys.y, Y0);
    keys.frame(vec![key(egui::Key::ArrowUp, egui::Modifiers::NONE)]);
    assert!(close(keys.y.0, Y0.0 + 0.2) && close(keys.y.1, Y0.1 + 0.2));

    let (mut keys, _) = focused();
    keys.frame(vec![key(egui::Key::Plus, egui::Modifiers::NONE)]);
    assert!(
        close(keys.x.0, 0.1) && close(keys.x.1, 0.9),
        "plus zoomed to {:?}",
        keys.x
    );
    assert_eq!(keys.y, Y0);
    keys.frame(vec![key(egui::Key::Minus, CTRL)]);
    assert!(
        close(keys.y.0, -1.25) && close(keys.y.1, 1.25),
        "Ctrl+minus: {:?}",
        keys.y
    );
    assert!(close(keys.x.0, 0.1) && close(keys.x.1, 0.9));

    let (mut keys, _) = focused();
    keys.cursors = Some(CursorPair {
        a: Some(0.6),
        b: Some(0.2),
    });
    request_one_axis_zoom(&keys.ctx, OneAxisZoom::X);
    keys.frame(Vec::new());
    keys.frame(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
    assert_eq!(keys.x, (0.2, 0.6), "X then Enter zoomed to {:?}", keys.x);
    assert_eq!(keys.y, Y0);

    // The arrows stay the plot's: egui does not also move the focus.
    let (mut keys, _) = focused();
    let frame = keys.frame(vec![key(egui::Key::ArrowDown, egui::Modifiers::NONE)]);
    keys.frame(vec![key(egui::Key::ArrowLeft, egui::Modifiers::NONE)]);
    assert!(keys.ctx.memory(|memory| memory.has_focus(frame.id)));
}

/// A cursor line is a handle along its whole height: a drag that starts on
/// it moves the cursor and does not pan.
#[test]
fn a_cursor_line_drags_from_anywhere_along_it() {
    let mut host = Host::new();
    host.cursors = Some(CursorPair {
        a: Some(0.5),
        b: None,
    });
    let plot = host.plot_rect();
    let on_line = pos2(plot.center().x + 2.0, plot.bottom() - 20.0);
    host.frame(vec![egui::Event::PointerMoved(on_line)]);
    host.frame(vec![
        egui::Event::PointerMoved(on_line),
        button(on_line, true, egui::Modifiers::NONE),
    ]);
    host.frame(vec![egui::Event::PointerMoved(
        on_line + egui::vec2(20.0, 0.0),
    )]);
    let moved = host.frame(vec![egui::Event::PointerMoved(
        on_line + egui::vec2(48.0, 5.0),
    )]);
    let (id, x) = moved.cursor_drag.expect("the line drag moved no cursor");
    assert_eq!(id, CursorId::A);
    let expected = 0.5 + 50.0 / f64::from(plot.width());
    assert!(
        (x - expected).abs() < 1.0e-6,
        "cursor A dragged to {x}, expected {expected}"
    );
    assert_eq!(
        (host.x, host.y),
        (X0, Y0),
        "the cursor drag panned the plot"
    );
}

/// A cursor outside the window stands as a chevron on the edge it left by,
/// and a click on it centres the window on the cursor at the same width.
#[test]
fn a_cursor_outside_the_window_is_a_chevron_that_brings_it_back() {
    let mut host = Host::new();
    host.cursors = Some(CursorPair {
        a: Some(0.25),
        b: Some(2.0),
    });
    let frame = host.frame(Vec::new());
    let plot = frame.plot_rect;
    let ink = Tokens::get(&host.ctx).color.canvas_cursor_b;
    let chevron = frame
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            Shape::Rect(rect) if rect.fill == ink && rect.rect.right() >= plot.right() - 1.0 => {
                Some(rect.rect)
            }
            _ => None,
        })
        .expect("no chevron at the right edge for cursor B");
    host.frame(vec![egui::Event::PointerMoved(chevron.center())]);
    host.frame(vec![
        button(chevron.center(), true, egui::Modifiers::NONE),
        button(chevron.center(), false, egui::Modifiers::NONE),
    ]);
    assert!(
        close(host.x.0, 1.5) && close(host.x.1, 2.5),
        "the window is {:?}",
        host.x
    );
}

fn painted(shapes: &[egui::epaint::ClippedShape]) -> Vec<Shape> {
    fn flatten(shape: &Shape, out: &mut Vec<Shape>) {
        match shape {
            Shape::Vec(inner) => inner.iter().for_each(|shape| flatten(shape, out)),
            shape => out.push(shape.clone()),
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        flatten(&clipped.shape, &mut out);
    }
    out
}

/// A sample with a hole either side is still data: it draws as a dot.
#[test]
fn an_isolated_sample_draws_as_a_dot() {
    let mut host = Host::new();
    host.trace = (
        vec![0.0, 0.25, 0.5, 0.75, 1.0],
        vec![f64::NAN, 0.5, f64::NAN, -0.5, f64::NAN],
    );
    let frame = host.frame(Vec::new());
    let dots = painted(&frame.shapes)
        .into_iter()
        .filter(|shape| matches!(shape, Shape::Circle(circle) if circle.fill == Color32::WHITE))
        .count();
    assert_eq!(dots, 2, "each isolated sample draws one dot");
}

fn texts(shapes: &[egui::epaint::ClippedShape]) -> Vec<String> {
    painted(shapes)
        .into_iter()
        .filter_map(|shape| match shape {
            Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect()
}

/// A horizontal cursor's level, and the difference between two, are read in
/// the axis' unit through the one formatter.
#[test]
fn horizontal_cursor_labels_carry_the_unit() {
    let mut host = Host::new();
    host.horizontal = [Some(0.3), Some(-0.45)];
    let frame = host.frame(Vec::new());
    let texts = texts(&frame.shapes);
    let h1 = format!("H1 {}", fmt_si_significant(0.3, "V", 4));
    let h2 = format!("H2 {}", fmt_si_significant(-0.45, "V", 4));
    let delta = format!("\u{0394}Y {}", fmt_si_signed(-0.75, "V", 4));
    for label in [&h1, &h2, &delta] {
        assert!(texts.contains(label), "{label:?} is not among {texts:?}");
    }
    assert!(h1.ends_with("mV") && delta.ends_with("mV"), "{h1} {delta}");
}
