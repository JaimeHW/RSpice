//! A plot keeps the picture it drew while nothing it shows changes; one too
//! costly for a frame is drawn off the frame, and one asked for and then
//! superseded is never shown.

use super::*;
use crate::plot::{Axis, DecimationWork};

const COLOR: Color32 = Color32::from_rgb(90, 170, 250);

fn sine(samples: usize) -> (Vec<f64>, Vec<f64>) {
    let x: Vec<f64> = (0..samples)
        .map(|index| index as f64 * 10.0 / samples as f64)
        .collect();
    let y = x.iter().map(|x| (x * 3.0).sin()).collect();
    (x, y)
}

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::Theme::default().apply(&ctx);
    ctx
}

/// One frame of a plot `size` big of `curves` copies of a curve over
/// `0..=x1`, each with an identity of its own, and what the cache did for it.
fn frame(
    ctx: &egui::Context,
    cache: &mut DecimationCache,
    curve: (&WaveformValues, &WaveformValues),
    curves: usize,
    x1: f64,
    size: egui::Vec2,
) -> (DecimationWork, Vec<egui::epaint::ClippedShape>) {
    cache.ensure_version(1);
    let before = cache.work();
    let (x, y) = (
        curve.0.in_memory().unwrap_or_default(),
        curve.1.in_memory().unwrap_or_default(),
    );
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            ..Default::default()
        },
        |ui| {
            let mut spec = PlotSpec::new(
                Axis::linear(0.0, x1, "s"),
                XScale::Linear,
                Axis::linear(-1.5, 1.5, "V"),
            );
            for index in 0..curves {
                spec.traces.push(
                    Trace::new(x, y, COLOR)
                        .cache_key(index as u64)
                        .shared(curve.0, curve.1),
                );
            }
            super::super::render::show(ui, &spec, cache, None, None);
        },
    );
    (cache.work().since(before), output.shapes)
}

fn visit(shape: &Shape, each: &mut impl FnMut(&Shape)) {
    if let Shape::Vec(shapes) = shape {
        for shape in shapes {
            visit(shape, each);
        }
    } else {
        each(shape);
    }
}

/// The strokes of the curves' colour a frame painted.
fn curve_strokes(shapes: &[egui::epaint::ClippedShape]) -> usize {
    let mut strokes = 0;
    for clipped in shapes {
        visit(&clipped.shape, &mut |shape| {
            if let Shape::Path(path) = shape
                && path.stroke.color == egui::epaint::ColorMode::Solid(COLOR)
            {
                strokes += 1;
            }
        });
    }
    strokes
}

/// The images a frame painted from textures of their own.
fn images(shapes: &[egui::epaint::ClippedShape]) -> usize {
    let mut images = 0;
    for clipped in shapes {
        visit(&clipped.shape, &mut |shape| {
            if let Shape::Mesh(mesh) = shape
                && mesh.texture_id != egui::TextureId::default()
            {
                images += 1;
            }
        });
    }
    images
}

fn says_updating(shapes: &[egui::epaint::ClippedShape]) -> bool {
    let mut says = false;
    for clipped in shapes {
        visit(&clipped.shape, &mut |shape| {
            if let Shape::Text(text) = shape {
                says |= text.galley.job.text.starts_with("Updating");
            }
        });
    }
    says
}

#[test]
fn an_unchanged_frame_reads_no_envelope_and_strokes_nothing() {
    let ctx = context();
    let mut cache = DecimationCache::default();
    let (x, y) = sine(5_000);
    let curve = (&WaveformValues::from(x), &WaveformValues::from(y));
    let size = egui::vec2(900.0, 400.0);
    let (first, drawn) = frame(&ctx, &mut cache, curve, 20, 10.0, size);
    assert_eq!(
        (first.rebuilds, first.strokes),
        (20, 20),
        "the first frame reduces and strokes every curve"
    );
    for _ in 0..3 {
        let (again, kept) = frame(&ctx, &mut cache, curve, 20, 10.0, size);
        assert_eq!(
            (again.hits, again.rebuilds, again.strokes),
            (0, 0, 0),
            "an unchanged frame read an envelope or stroked a curve"
        );
        assert_eq!(
            curve_strokes(&kept),
            curve_strokes(&drawn),
            "the kept picture is the one drawn"
        );
    }
    let (zoomed, _) = frame(&ctx, &mut cache, curve, 20, 5.0, size);
    assert_eq!(zoomed.strokes, 20, "a new window draws the picture again");
}

#[test]
fn a_picture_of_more_curve_than_the_plot_has_pixels_is_kept_as_pixels() {
    let ctx = context();
    let mut cache = DecimationCache::default();
    let (x, y) = sine(1_000);
    let curve = (&WaveformValues::from(x), &WaveformValues::from(y));
    // A small plot of a hundred curves holds more points than pixels.
    let size = egui::vec2(240.0, 120.0);
    let (first, drawn) = frame(&ctx, &mut cache, curve, 100, 10.0, size);
    assert_eq!(first.strokes, 100);
    assert_eq!(curve_strokes(&drawn), 0, "no curve is stroked as a shape");
    assert_eq!(images(&drawn), 1, "the curves are one image");
    let (again, kept) = frame(&ctx, &mut cache, curve, 100, 10.0, size);
    assert_eq!((again.hits, again.rebuilds, again.strokes), (0, 0, 0));
    assert_eq!(images(&kept), 1);
}

#[test]
fn a_picture_costing_more_than_a_frame_is_drawn_off_it() {
    let ctx = context();
    let mut cache = DecimationCache::default();
    let (x, y) = sine(40_000);
    let curve = (&WaveformValues::from(x), &WaveformValues::from(y));
    let size = egui::vec2(900.0, 400.0);
    let (asked, shown) = frame(&ctx, &mut cache, curve, 300, 10.0, size);
    assert_eq!(
        (asked.rebuilds, asked.strokes),
        (0, 0),
        "the frame that asks for the picture draws none of it"
    );
    assert!(
        says_updating(&shown),
        "the plot says its curves are updating"
    );
    let started = std::time::Instant::now();
    let arrived = loop {
        let (work, shown) = frame(&ctx, &mut cache, curve, 300, 10.0, size);
        if !says_updating(&shown) {
            break work;
        }
        assert!(
            started.elapsed() < std::time::Duration::from_secs(60),
            "the picture never arrived"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    };
    assert_eq!(
        arrived.strokes, 300,
        "the arriving picture holds every curve"
    );
    let (again, _) = frame(&ctx, &mut cache, curve, 300, 10.0, size);
    assert_eq!((again.rebuilds, again.strokes), (0, 0));
}

fn nothing_drawn() -> Built {
    Built {
        drawing: Drawing::Shapes(Vec::new()),
        complete: true,
        kept: Vec::new(),
        curves: 0,
    }
}

#[test]
fn a_superseded_picture_is_called_off_and_never_shown() {
    let geometry = Geometry {
        plot_rect: Rect::from_min_size(Pos2::ZERO, egui::vec2(100.0, 50.0)),
        pixels_per_point: 1.0,
        x: (0.0, 1.0, XScale::Linear),
        y: (0.0, 1.0, XScale::Linear),
        columns: 128,
        rows: 64,
        background: Color32::BLACK,
    };
    let mut pictures = Pictures::default();
    let (first_cancel, first) = pictures.request(1, geometry);
    let (second_cancel, second) = pictures.request(2, geometry);
    assert!(
        first_cancel.load(Ordering::Relaxed),
        "asking for another picture calls the first off"
    );
    assert!(!second_cancel.load(Ordering::Relaxed));
    *first.lock().expect("the first picture's slot") = Some(nothing_drawn());
    assert!(
        pictures.arrived().is_none(),
        "the superseded picture never arrives"
    );
    *second.lock().expect("the second picture's slot") = Some(nothing_drawn());
    let (key, _, _) = pictures
        .arrived()
        .expect("the picture asked for last arrives");
    assert_eq!(key, 2);
    assert!(pictures.arrived().is_none(), "and arrives once");
}
