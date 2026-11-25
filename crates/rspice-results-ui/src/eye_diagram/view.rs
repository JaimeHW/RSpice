//! EYE — density-rendered eye diagram with the compliance mask and the
//! bathtub under it, measured metrics in the right panel.
//!
//! Acquisitions are folded segments of the active transient (built by the
//! eye pipeline). They are rasterized once per data revision into an
//! alpha-accumulated density texture — the classic persistence picture —
//! so each frame costs one textured quad plus the vector mask, instead of
//! restroking a transient's worth of segments.

use egui::Ui;

use super::EyeDiagramState;
use crate::eye_diagram::{BerEstimate, EyeData, EyeMeasurements, EyeTimebaseProvenance};
use crate::fft::view::columns::{Fitting, fitted};
use crate::presentation::{DECIBEL_PLACES, PlotView, fixed_places};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{
    self, Axis, DrawMode, LimitLine, PlotSpec, Trace, XScale, fmt_column_reading,
    fmt_si_significant,
};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::table::{Align, Cell, DataTable};

use crate::presentation::well_hint;
use crate::strip::{self, LegendChip};

/// GPU density image for the EYE viewer, baked once per (data revision,
/// plot size, trace color) — per frame the eye then costs one textured
/// quad instead of restroking every folded acquisition.
#[derive(Clone)]
pub struct EyeTexture {
    revision: u64,
    size: [usize; 2],
    color: egui::Color32,
    /// The data-space window the texture covers, which the draw maps onto
    /// whatever the reader has zoomed to.
    extent_bits: [u64; 4],
    handle: egui::TextureHandle,
}

impl std::fmt::Debug for EyeTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EyeTexture")
            .field("revision", &self.revision)
            .field("size", &self.size)
            .field("extent_bits", &self.extent_bits)
            .finish_non_exhaustive()
    }
}

/// Local plot outcome and whether the retained acquisitions were rasterized.
#[derive(Default)]
pub struct EyeViewResponse {
    pub plot: Option<plot::PlotResponse>,
    pub density_baked: bool,
}

/// What to tell the reader when no eye could be folded.
///
/// A refusal that says only "no usable source" leaves the reader with no
/// move. Each rejection names what the waveform lacked and points at the rate
/// control, which is the remedy in every case.
pub fn unavailable_hint(eye: &EyeDiagramState, source: &str) -> Option<String> {
    eye.timebase_provenance()?.rejection_hint(source)
}

/// What the provenance line says when the stated rate is not the data's own.
const INCOHERENT_NOTE: &str = " · crossings incoherent at this rate";
/// The same fact in the header's own terms, where it carries the warning
/// tone: the tag goes amber and this is the reason behind it.
const INCOHERENT_REASON: &str =
    "crossings incoherent at the set rate — folded as asked, not as the data runs";

/// Is the eye on screen folded at a rate its own crossings disagree with?
fn timebase_is_incoherent(eye: &EyeDiagramState) -> bool {
    matches!(
        eye.timebase_provenance(),
        Some(EyeTimebaseProvenance::Explicit {
            incoherent: true,
            ..
        })
    )
}

/// One line describing what is on screen and where its bit period came from.
fn timebase_summary(eye: &EyeDiagramState, significant_digits: usize) -> String {
    let period = |seconds: f64| fmt_si_significant(seconds, "s", significant_digits);
    let rate = |bits: f64| fmt_si_significant(bits, "b/s", significant_digits);
    match eye.timebase_provenance() {
        Some(EyeTimebaseProvenance::Auto {
            unit_interval,
            edge_count,
            low_confidence,
            ..
        }) => format!(
            "{} · {} · {edge_count} edges",
            if *low_confidence {
                "auto (low confidence)"
            } else {
                "auto"
            },
            period(*unit_interval),
        ),
        Some(EyeTimebaseProvenance::Explicit {
            unit_interval,
            incoherent,
        }) => format!(
            "set · {} · {}{}",
            period(*unit_interval),
            rate(1.0 / unit_interval),
            if *incoherent { INCOHERENT_NOTE } else { "" },
        ),
        Some(EyeTimebaseProvenance::Triggered {
            unit_interval,
            edge_count,
        }) => format!(
            "clock signal · {} · {edge_count} edges",
            period(*unit_interval)
        ),
        Some(
            EyeTimebaseProvenance::AutoRejected(_)
            | EyeTimebaseProvenance::ClockRejected(_)
            | EyeTimebaseProvenance::Unreadable(_),
        )
        | None => rate(eye.data.data_rate),
    }
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// The share of the sheet's height the eye takes when the bathtub stands
/// under it.
const EYE_SHARE: f32 = 0.64;
/// The largest a density texture is on either side, in device pixels: the
/// size every GPU is guaranteed to take.
const MAX_TEXTURE_SIDE: f32 = 2048.0;
/// Positions across the unit interval the bathtub's fitted curve is drawn
/// through.
const BATHTUB_CURVE_POINTS: usize = 201;
/// The least error rate the bathtub's axis reaches.
const BATHTUB_FLOOR: f64 = 1e-18;

/// Render the eye, and the bathtub of its crossings under it.
pub fn show(
    ui: &mut Ui,
    eye: &EyeDiagramState,
    source: &str,
    view: PlotView,
    texture: &mut Option<EyeTexture>,
    cache: &mut plot::DecimationCache,
    significant_digits: usize,
) -> EyeViewResponse {
    let mut out = EyeViewResponse::default();
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    if eye.data.traces.is_empty() {
        let hint = unavailable_hint(eye, source).unwrap_or_else(|| {
            "No eye yet — the eye folds the active transient at the bit period".to_owned()
        });
        well_hint(ui, &hint);
        return out;
    }
    let summary = timebase_summary(eye, significant_digits);
    // Words alone would sit in the same faint grey as the rest of the
    // provenance line, and the reader is being told the picture is not of
    // their signal. The tag carries the tone and the reason behind it.
    let incoherent = timebase_is_incoherent(eye);

    let data = &eye.data;
    let subtitle = format!(
        "{} acquisitions · {} UI · {summary}",
        data.traces.len(),
        data.ui_count,
    );
    strip::StripHeader::new("EYE", &subtitle, &[])
        .incomplete(incoherent.then_some(INCOHERENT_REASON))
        .show(ui);

    let ui_count = f64::from(data.ui_count.max(1));
    let swing = (data.v_high - data.v_low).abs().max(1e-9);
    let auto_y0 = data.v_low - swing * 0.18;
    let auto_y1 = data.v_high + swing * 0.18;
    let (x0, x1) = view.x.unwrap_or((0.0, ui_count));
    let (y0, y1) = view.y.unwrap_or((auto_y0, auto_y1));

    let half_ticks: Vec<f64> = (0..=(ui_count * 2.0) as i64)
        .map(|i| i as f64 * 0.5)
        .collect();
    let x_axis = if view.x.is_some() {
        Axis::linear_with(x0, x1, "UI", 6)
    } else {
        Axis::with_ticks(x0, x1, "UI", &half_ticks)
    };
    let settings = eye.settings();
    let show_mask = settings.show_mask;
    let accessible_detail = if show_mask {
        format!(
            "{} folded acquisitions; compliance mask; {} violations from {} tested samples",
            data.traces.len(),
            settings.mask.violation_count,
            settings.mask.total_samples
        )
    } else {
        format!(
            "{} folded acquisitions; no compliance mask",
            data.traces.len()
        )
    };
    let mut spec = PlotSpec::new(x_axis, XScale::Linear, Axis::linear(y0, y1, "V"))
        .accessible_name("Eye diagram")
        .accessible_detail(&accessible_detail)
        .with_significant_digits(significant_digits);

    let bathtub = &eye.measurements.bathtub;
    let has_bathtub = !bathtub.measured.is_empty();
    let eye_height = if has_bathtub {
        (ui.available_height() * EYE_SHARE).round()
    } else {
        ui.available_height()
    };
    let width = ui.available_width();
    ui.allocate_ui(egui::vec2(width, eye_height), |ui| {
        // Bake (or fetch) the density texture. The bake walks every
        // acquisition once; frames just draw the quad.
        //
        // It is baked over the data's own extent rather than the current
        // view, so panning and zooming re-map the same texture instead of
        // re-walking every acquisition. It is keyed by the data revision,
        // its resolution and its ink, and sized to the plot's device pixels
        // within the side every GPU takes.
        let plot_rect = plot::plot_rect(ui, &spec);
        let pixels = ui.ctx().pixels_per_point();
        let side = |points: f32| (points * pixels).round().clamp(8.0, MAX_TEXTURE_SIDE) as usize;
        let tex_size = [side(plot_rect.width()), side(plot_rect.height())];
        let revision = eye.data_revision();
        let trace_color = c.traces[0];
        let extent = (0.0_f64, ui_count, auto_y0, auto_y1);
        let extent_bits = [
            extent.0.to_bits(),
            extent.1.to_bits(),
            extent.2.to_bits(),
            extent.3.to_bits(),
        ];
        let needs_bake = match texture.as_ref() {
            Some(tex) => {
                tex.revision != revision
                    || tex.size != tex_size
                    || tex.color != trace_color
                    || tex.extent_bits != extent_bits
            }
            None => true,
        };
        if needs_bake {
            out.density_baked = true;
            let image = rasterize_density(
                data,
                extent.0,
                extent.1,
                extent.2,
                extent.3,
                tex_size,
                trace_color,
            );
            let handle =
                ui.ctx()
                    .load_texture("rspice.eye.density", image, egui::TextureOptions::LINEAR);
            *texture = Some(EyeTexture {
                revision,
                size: tex_size,
                color: trace_color,
                extent_bits,
                handle,
            });
        }
        let texture_id = texture.as_ref().map(|tex| tex.handle.id());
        let density_uv = density_uv(extent, (x0, x1), (y0, y1));

        // Underlay: the density quad, then the mask above it, placed on
        // this eye's centre and threshold.
        let mask_points: Vec<(f64, f64)> = if show_mask {
            settings.mask.polygon(eye.mask_frame()).points
        } else {
            Vec::new()
        };
        let mask_fill = {
            let [r, g, b, _] = c.err.to_array();
            egui::Color32::from_rgba_unmultiplied(r, g, b, 26)
        };
        let err = c.err;
        spec.underlay = Some(Box::new(move |painter, mapper| {
            if let Some(texture_id) = texture_id {
                painter.image(texture_id, mapper.rect, density_uv, egui::Color32::WHITE);
            }
            let points: Vec<egui::Pos2> = mask_points
                .iter()
                .map(|&(t_ui, volts)| egui::pos2(mapper.x(t_ui), mapper.y(volts)))
                .collect();
            if points.len() >= 3 {
                // A custom mask need not be convex: its outline is what
                // the test reads, and the fill is only a wash under it.
                painter.add(egui::Shape::convex_polygon(
                    points.clone(),
                    mask_fill,
                    egui::Stroke::NONE,
                ));
                let mut outline = points;
                outline.push(outline[0]);
                painter.extend(egui::Shape::dashed_line(
                    &outline,
                    egui::Stroke::new(1.0, err),
                    5.0,
                    4.0,
                ));
            }
        }));
        out.plot = Some(plot::show(ui, &spec, cache, None, None));
    });

    if has_bathtub {
        show_bathtub(ui, eye, cache, significant_digits);
    }
    out
}

/// The bathtub of the eye's crossings: the error rate against the sampling
/// position across one unit interval from the crossing, the counted points
/// apart from the fitted curve, with the opening read at the error rate the
/// reader chose.
fn show_bathtub(
    ui: &mut Ui,
    eye: &EyeDiagramState,
    cache: &mut plot::DecimationCache,
    significant_digits: usize,
) {
    let c = Tokens::get(ui.ctx()).color;
    let bathtub = &eye.measurements.bathtub;
    let rate = eye.settings().opening_ber;
    let subtitle = bathtub_summary(&eye.measurements, rate, significant_digits);
    strip::StripHeader::new(
        "BATHTUB",
        &subtitle,
        &[
            LegendChip {
                name: "counted",
                color: c.traces[0],
                on: true,
                glyph: strip::LegendGlyph::Dot,
            },
            LegendChip {
                name: "fitted",
                color: c.traces[1],
                on: bathtub.fit.is_some(),
                glyph: strip::LegendGlyph::Line,
            },
        ],
    )
    .show(ui);

    let floor = BATHTUB_FLOOR.min(rate / 1e3);
    let (measured_x, measured_y): (Vec<f64>, Vec<f64>) = bathtub.measured.iter().copied().unzip();
    let (fit_x, fit_y): (Vec<f64>, Vec<f64>) = bathtub
        .fit
        .as_ref()
        .map(|fit| {
            (0..BATHTUB_CURVE_POINTS)
                .map(|step| {
                    #[allow(clippy::cast_precision_loss)]
                    let position = step as f64 / (BATHTUB_CURVE_POINTS - 1) as f64;
                    (position, fit.ber_at(position).clamp(floor, 1.0))
                })
                .unzip()
        })
        .unwrap_or_default();
    let mut spec = PlotSpec::new(
        Axis::linear_with(0.0, 1.0, "UI", 6),
        XScale::Linear,
        ber_axis(floor),
    )
    .with_log_y()
    .with_y_without_unit("BER")
    .accessible_name("Bathtub")
    .with_significant_digits(significant_digits);
    if !fit_x.is_empty() {
        spec.traces.push(Trace::new(&fit_x, &fit_y, c.traces[1]));
    }
    spec.traces.push(
        Trace::new(&measured_x, &measured_y, c.traces[0])
            .draw(DrawMode::Points)
            .show_single_point(),
    );
    spec.limit_lines.push(LimitLine {
        y: rate,
        color: c.text_dim,
        label: format!("{rate:e}"),
    });
    plot::show(ui, &spec, cache, None, None);
}

/// The bathtub's error-rate axis, from `floor` to 1. An error rate is a bare
/// probability, so its decades are powers of ten spelled as the rate field
/// and the reference line spell them (1e-3, 1e-12), not SI prefixes.
fn ber_axis(floor: f64) -> Axis {
    let mut axis = Axis::log_decades(floor, 1.0, "");
    for (value, label) in &mut axis.ticks {
        *label = ber_label(*value);
    }
    axis
}

/// An error rate as a power of ten: "1" for 1, "1e-12" for a decade, and
/// the scientific spelling for a tick between decades.
fn ber_label(value: f64) -> String {
    let power = value.log10().round();
    if (value / 10f64.powf(power) - 1.0).abs() > 1e-9 {
        return format!("{value:e}");
    }
    #[allow(clippy::cast_possible_truncation)]
    let power = power as i32;
    if power == 0 {
        "1".to_owned()
    } else {
        format!("1e{power}")
    }
}

/// What the bathtub reads: the opening at the chosen rate and the jitter the
/// fit separates, or why it draws only what it counted.
fn bathtub_summary(m: &EyeMeasurements, rate: f64, significant_digits: usize) -> String {
    let bathtub = &m.bathtub;
    let Some(fit) = bathtub.fit.as_ref() else {
        return format!(
            "counted only · {} crossings, too few to extrapolate from",
            bathtub.crossing_count
        );
    };
    let seconds = |value: f64| fmt_si_significant(value, "s", significant_digits);
    let opening = fit.opening(rate).map_or_else(
        || "closed".to_owned(),
        |opening| fmt_si_significant(opening, "UI", significant_digits),
    );
    format!(
        "opening {opening} at {rate:e} · RJ {} rms · DJ {} · {} crossings",
        seconds(fit.random_rms()),
        seconds(fit.deterministic()),
        fit.crossing_count,
    )
}

/// Where the reader's window sits inside the baked density texture.
///
/// `extent` is the data window the texture covers, as `(x0, x1, y0, y1)`.
/// Texture row 0 is the *top* of that window, matching how the raster is
/// accumulated, so the vertical fractions run the other way.
fn density_uv(extent: (f64, f64, f64, f64), x: (f64, f64), y: (f64, f64)) -> egui::Rect {
    let (ex0, ex1, ey0, ey1) = extent;
    let x_span = (ex1 - ex0).abs().max(1e-12);
    let y_span = (ey1 - ey0).abs().max(1e-12);
    let u = |value: f64| ((value - ex0) / x_span) as f32;
    let v = |value: f64| ((ey1 - value) / y_span) as f32;
    egui::Rect::from_min_max(egui::pos2(u(x.0), v(y.1)), egui::pos2(u(x.1), v(y.0)))
}

/// Rasterize every acquisition into a density image in the trace colour,
/// its strength at each pixel graded by [`density_strength`].
fn rasterize_density(
    data: &EyeData,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    size: [usize; 2],
    color: egui::Color32,
) -> egui::ColorImage {
    let [width, height] = size;
    let mut coverage = vec![0.0f32; width * height];
    let x_scale = width as f64 / (x1 - x0).max(1e-12);
    let y_span = (y1 - y0).max(1e-12);
    let y_scale = height as f64 / y_span;

    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 1..n {
            let ax = ((trace.time[i - 1] - x0) * x_scale) as f32;
            let ay = ((y1 - trace.amplitude[i - 1]) * y_scale) as f32;
            let bx = ((trace.time[i] - x0) * x_scale) as f32;
            let by = ((y1 - trace.amplitude[i]) * y_scale) as f32;
            if let Some((ax, ay, bx, by)) = clip_line_to_raster(ax, ay, bx, by, width, height) {
                accumulate_line(&mut coverage, width, height, ax, ay, bx, by);
            }
        }
    }

    let densest = coverage.iter().copied().fold(0.0f32, f32::max);
    let [r, g, b, _] = color.to_array();
    let pixels = coverage
        .iter()
        .map(|&hits| {
            let alpha = (density_strength(hits, densest) * 255.0).round() as u8;
            egui::Color32::from_rgba_unmultiplied(r, g, b, alpha)
        })
        .collect();
    egui::ColorImage::new([width, height], pixels)
}

/// The strength, as a fraction of the trace colour, a lone acquisition draws
/// at. Fainter, a trace that occurs once reads as background.
const LONE_TRACE_STRENGTH: f32 = 0.6;

/// How strongly a pixel draws, as a fraction of the trace colour, from the
/// acquisitions crossing it (`hits`, in traces: one trace passing squarely
/// through a pixel leaves one) and the most any pixel holds (`densest`).
///
/// The densest pixel draws at full strength and a lone trace at
/// [`LONE_TRACE_STRENGTH`]. Between them strength rises with the logarithm of
/// the count, as an intensity-graded persistence display does, so an eye of
/// ten thousand acquisitions keeps its gradient instead of saturating. Below
/// one hit the pixel is a trace's anti-aliased edge: the pixel nearest a
/// trace's centre always holds at least half a hit, so it reaches the full
/// lone-trace strength there and fades to nothing away from it.
fn density_strength(hits: f32, densest: f32) -> f32 {
    if hits.is_nan() || hits <= 0.0 {
        return 0.0;
    }
    let edge = (hits * 2.0).min(1.0);
    if densest <= 1.0 {
        // Nothing overlaps, so every trace is the densest thing drawn.
        return edge;
    }
    if hits <= 1.0 {
        return LONE_TRACE_STRENGTH * edge;
    }
    let graded = (hits.ln() / densest.ln()).min(1.0);
    LONE_TRACE_STRENGTH + (1.0 - LONE_TRACE_STRENGTH) * graded
}

fn clip_line_to_raster(
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
    width: usize,
    height: usize,
) -> Option<(f32, f32, f32, f32)> {
    if width == 0 || height == 0 || ![ax, ay, bx, by].iter().all(|value| value.is_finite()) {
        return None;
    }
    // Clip in f64 even though raster coordinates are f32. Deep plot zoom can
    // place segment endpoints billions of pixels off-screen; f32 cannot then
    // preserve the few-pixel difference between the two clipping parameters.
    let (ax64, ay64, bx64, by64) = (ax as f64, ay as f64, bx as f64, by as f64);
    let dx = bx64 - ax64;
    let dy = by64 - ay64;
    let max_x = width.saturating_sub(1) as f64;
    let max_y = height.saturating_sub(1) as f64;
    let mut enter = 0.0f64;
    let mut leave = 1.0f64;
    for (p, q) in [
        (-dx, ax64),
        (dx, max_x - ax64),
        (-dy, ay64),
        (dy, max_y - ay64),
    ] {
        if p.abs() <= f64::EPSILON {
            if q < 0.0 {
                return None;
            }
            continue;
        }
        let ratio = q / p;
        if p < 0.0 {
            enter = enter.max(ratio);
        } else {
            leave = leave.min(ratio);
        }
        if enter > leave {
            return None;
        }
    }
    Some((
        (ax64 + enter * dx) as f32,
        (ay64 + enter * dy) as f32,
        (ax64 + leave * dx) as f32,
        (ay64 + leave * dy) as f32,
    ))
}

/// Anti-aliased DDA: walk the segment one pixel-step at a time, splitting
/// each step's coverage bilinearly across the four nearest pixels.
fn accumulate_line(
    coverage: &mut [f32],
    width: usize,
    height: usize,
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
) {
    let dx = bx - ax;
    let dy = by - ay;
    let steps = dx.abs().max(dy.abs()).ceil().max(1.0) as usize;
    let inv = 1.0 / steps as f32;
    for step in 0..steps {
        let t = step as f32 * inv;
        let x = ax + dx * t;
        let y = ay + dy * t;
        let (xf, yf) = (x.floor(), y.floor());
        let (fx, fy) = (x - xf, y - yf);
        let (xi, yi) = (xf as isize, yf as isize);
        let splat = [
            (xi, yi, (1.0 - fx) * (1.0 - fy)),
            (xi + 1, yi, fx * (1.0 - fy)),
            (xi, yi + 1, (1.0 - fx) * fy),
            (xi + 1, yi + 1, fx * fy),
        ];
        for (px, py, weight) in splat {
            if px >= 0 && py >= 0 && (px as usize) < width && (py as usize) < height {
                coverage[py as usize * width + px as usize] += weight;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Eye metrics + mask verdict.
pub fn right_panel(ui: &mut Ui, eye: &EyeDiagramState, source: &str, significant_digits: usize) {
    section_header(ui, "Eye", None);
    if eye.data.traces.is_empty() {
        crate::presentation::panel_note(
            ui,
            &unavailable_hint(eye, source).unwrap_or_else(|| {
                "Metrics appear once the eye is built from the transient.".to_owned()
            }),
        );
        return;
    }
    let rows = metric_rows(
        &eye.measurements,
        eye.settings().opening_ber,
        significant_digits,
    );
    metrics_table(ui, "rspice.results.eye.metrics", &rows);

    let settings = eye.settings();
    if settings.show_mask {
        section_header(ui, "Mask", None);
        let mask = &settings.mask;
        let rows = [
            (
                "Violations".to_owned(),
                format!("{} of {} samples", mask.violation_count, mask.total_samples),
            ),
            (
                "Clearance".to_owned(),
                or_unmeasured(mask.clearance, |value| {
                    fmt_column_reading(value * 100.0, "%", significant_digits)
                }),
            ),
        ];
        metrics_table(ui, "rspice.results.eye.mask", &rows);
    }
}

/// A two-column table of named figures, flush in the details pane.
fn metrics_table(ui: &mut Ui, id: &str, rows: &[(String, String)]) {
    let columns = fitted(
        ui,
        vec![
            Fitting {
                head: "Measurement".to_owned(),
                tooltip: None,
                align: Align::Left,
                cells: rows.iter().map(|row| (row.0.clone(), false)).collect(),
            },
            Fitting {
                head: "Value".to_owned(),
                tooltip: None,
                align: Align::Right,
                cells: rows.iter().map(|row| (row.1.clone(), true)).collect(),
            },
        ],
    );
    DataTable::new(id, &columns, rows.len()).show(ui, |index| {
        vec![
            Cell::text(rows[index].0.clone()),
            Cell::quantity(rows[index].1.clone()),
        ]
    });
}

/// The eye's figures at the reader's significant digits, each in its unit:
/// the levels and their spread, the opening, the edges, S/N, Q and the error
/// rate it implies, and the jitter, measured and separated by the fit.
fn metric_rows(m: &EyeMeasurements, rate: f64, significant_digits: usize) -> Vec<(String, String)> {
    let value = |value: f64, unit: &str| fmt_column_reading(value, unit, significant_digits);
    let mut rows = vec![(
        "Unit interval".to_owned(),
        format!(
            "{} ({})",
            value(m.unit_interval, "s"),
            value(m.data_rate, "b/s")
        ),
    )];
    let two_level = m.levels.len() == 2;
    for (index, level) in m.levels.iter().enumerate() {
        let name = if two_level {
            ["Zero level", "One level"][index].to_owned()
        } else {
            format!("Level {index}")
        };
        rows.push((name.clone(), value(level.mean, "V")));
        rows.push((format!("{name} σ"), value(level.sigma, "V")));
    }
    rows.push((
        "Amplitude".to_owned(),
        or_unmeasured(m.amplitude, |amplitude| value(amplitude, "V")),
    ));
    if m.openings.len() > 1 {
        for (index, opening) in m.openings.iter().enumerate() {
            rows.push((
                format!("Opening {index} height"),
                value(opening.height, "V"),
            ));
            rows.push((format!("Opening {index} width"), value(opening.width, "UI")));
        }
    } else {
        rows.push(("Eye height".to_owned(), value(m.eye_height, "V")));
        rows.push(("Eye width".to_owned(), value(m.eye_width, "UI")));
        rows.push((
            "Crossing".to_owned(),
            or_unmeasured(m.crossing_percentage, |crossing| {
                value(crossing * 100.0, "%")
            }),
        ));
    }
    rows.push((
        "Rise 20–80".to_owned(),
        or_unmeasured(m.rise_time, |rise| value(rise, "s")),
    ));
    rows.push((
        "Fall 80–20".to_owned(),
        or_unmeasured(m.fall_time, |fall| value(fall, "s")),
    ));
    rows.push((
        "S/N".to_owned(),
        or_unmeasured(m.snr_db, |snr| {
            if snr.is_infinite() {
                "∞".to_owned()
            } else {
                format!("{} dB", fixed_places(snr, DECIBEL_PLACES))
            }
        }),
    ));
    rows.push((
        "Q factor".to_owned(),
        or_unmeasured(m.q_factor, |q| {
            if q.is_infinite() {
                "∞".to_owned()
            } else {
                value(q, "")
            }
        }),
    ));
    rows.push((
        "BER from Q".to_owned(),
        match m.ber {
            BerEstimate::FromQ(ber) => value(ber, ""),
            BerEstimate::NoRandomNoise => "none: not Gaussian".to_owned(),
            BerEstimate::Unmeasured => "—".to_owned(),
        },
    ));
    rows.push(("Jitter p-p".to_owned(), value(m.jitter_pp, "s")));
    rows.push(("Jitter rms".to_owned(), value(m.jitter_rms, "s")));
    match m.bathtub.fit.as_ref() {
        Some(fit) => {
            rows.push(("RJ rms".to_owned(), value(fit.random_rms(), "s")));
            rows.push(("DJ δδ".to_owned(), value(fit.deterministic(), "s")));
            rows.push((
                format!("TJ at {rate:e}"),
                or_unmeasured(fit.total_jitter(rate), |tj| value(tj, "s")),
            ));
            rows.push((
                format!("Opening at {rate:e}"),
                or_unmeasured(fit.opening(rate), |opening| value(opening, "UI")),
            ));
        }
        None => rows.push((
            "RJ and DJ".to_owned(),
            format!("{} crossings, too few to fit", m.bathtub.crossing_count),
        )),
    }
    rows
}

/// An em dash where the data could not support a figure.
///
/// A blank is not a zero and must not print as one: `0 s` of rise time and
/// `0.0 %` of crossing are readings an engineer would act on.
fn or_unmeasured(value: Option<f64>, format: impl FnOnce(f64) -> String) -> String {
    value.map_or_else(|| "—".to_owned(), format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eye_diagram::EyeTrace;

    /// The bathtub labels its decades as powers of ten, as its 1e-12
    /// reference line is labelled, never with SI prefixes.
    #[test]
    fn the_bathtub_axis_reads_powers_of_ten() {
        let labels: Vec<String> = ber_axis(1e-15)
            .ticks
            .into_iter()
            .map(|(_, label)| label)
            .collect();
        let decades: Vec<String> = (-15..0)
            .map(|power| format!("1e{power}"))
            .chain(["1".to_owned()])
            .collect();
        assert_eq!(labels, decades);
        assert_eq!(ber_label(1e-12), format!("{:e}", 1e-12));
        assert_eq!(ber_label(2e-3), "2e-3");
    }

    /// The opening and the edges keep the digits the reader asked for,
    /// zeros included.
    #[test]
    fn the_eye_figures_read_at_significant_digits() {
        let measurements = super::super::EyeMeasurements {
            eye_height: 1.49,
            rise_time: Some(9.6e-12),
            ..Default::default()
        };
        let rows = metric_rows(&measurements, 1e-12, 4);
        let row = |rows: &[(String, String)], name: &str| {
            rows.iter()
                .find(|row| row.0 == name)
                .map(|row| row.1.clone())
                .expect("the row is listed")
        };
        assert_eq!(row(&rows, "Eye height"), "1.490 V");
        assert_eq!(row(&rows, "Rise 20–80"), "9.600 ps");
        assert_eq!(
            row(&metric_rows(&measurements, 1e-12, 3), "Eye height"),
            "1.49 V",
            "three digits keep all three"
        );
    }

    #[test]
    fn density_raster_respects_the_current_viewport() {
        let mut data = EyeData::new(1.0e-9, 2);
        data.add_trace(EyeTrace::new(vec![0.25, 0.75], vec![0.0, 1.0]));

        let visible = rasterize_density(&data, 0.0, 1.0, -0.5, 1.5, [32, 32], egui::Color32::WHITE);
        let outside = rasterize_density(&data, 1.1, 2.0, -0.5, 1.5, [32, 32], egui::Color32::WHITE);

        assert!(visible.pixels.iter().any(|pixel| pixel.a() > 0));
        assert!(outside.pixels.iter().all(|pixel| pixel.a() == 0));
    }

    /// The eye folds at whatever rate the reader states, so the only place
    /// the disagreement can appear is the line that says what was folded.
    /// It has to appear there in words — the picture of an eye folded at a
    /// rate the data does not run at looks like an eye.
    #[test]
    fn a_stated_rate_the_crossings_reject_is_named_in_the_provenance_line() {
        let mut eye = EyeDiagramState::default();
        let mut data = EyeData::new(400e-12, 2);
        data.add_trace(EyeTrace::new(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 0.0]));

        for (incoherent, expected) in [(true, true), (false, false)] {
            eye.load_data_with_timebase(
                data.clone(),
                Some(EyeTimebaseProvenance::Explicit {
                    unit_interval: 400e-12,
                    incoherent,
                }),
            );

            let summary = timebase_summary(&eye, 4);
            assert!(summary.starts_with("set · "), "{summary}");
            assert_eq!(
                summary.contains("crossings incoherent at this rate"),
                expected,
                "{summary}"
            );
            assert_eq!(timebase_is_incoherent(&eye), expected);
        }
    }

    /// A recovered rate is never marked: the estimator refuses rather than
    /// folding at a period the record does not carry, so there is nothing to
    /// warn about and a warning would be noise.
    #[test]
    fn a_recovered_rate_carries_no_incoherence_mark() {
        let mut eye = EyeDiagramState::default();
        let mut data = EyeData::new(1e-9, 2);
        data.add_trace(EyeTrace::new(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 0.0]));
        eye.load_data_with_timebase(
            data,
            Some(EyeTimebaseProvenance::Auto {
                unit_interval: 1e-9,
                edge_count: 40,
                rms_residual_ui: 1e-9,
                low_confidence: false,
            }),
        );

        let summary = timebase_summary(&eye, 4);
        assert!(summary.starts_with("auto · "), "{summary}");
        assert!(!summary.contains("incoherent"), "{summary}");
        assert!(!timebase_is_incoherent(&eye));
    }

    /// The densest pixel is the full trace colour, a trace that occurs once
    /// still reads plainly, and more acquisitions never draw fainter than
    /// fewer.
    #[test]
    fn density_reads_from_a_lone_trace_to_the_densest_pixel() {
        for densest in [2.0, 12.0, 10_000.0] {
            assert_eq!(density_strength(densest, densest), 1.0, "densest {densest}");
            assert!(density_strength(1.0, densest) >= LONE_TRACE_STRENGTH);
            assert!(density_strength(0.5, densest) >= LONE_TRACE_STRENGTH);
            let mut last = 0.0;
            for step in 0..=4_000 {
                let hits = densest * step as f32 / 4_000.0;
                let strength = density_strength(hits, densest);
                assert!(strength >= last, "{hits} of {densest} drew fainter");
                last = strength;
            }
        }
        // A rail ten times rarer than the densest pixel of a large eye is
        // still visibly fainter than it: the gradient survives.
        assert!(density_strength(1_000.0, 10_000.0) < 0.95);

        // In the raster: a lone trace beside a dozen stacked ones.
        let mut data = EyeData::new(1.0e-9, 2);
        data.add_trace(EyeTrace::new(vec![0.0, 2.0], vec![0.25, 0.25]));
        for _ in 0..12 {
            data.add_trace(EyeTrace::new(vec![0.0, 2.0], vec![0.75, 0.75]));
        }
        let image = rasterize_density(&data, 0.0, 2.0, 0.0, 1.0, [64, 64], egui::Color32::WHITE);
        let brightest_in_rows = |rows: std::ops::Range<usize>| {
            rows.flat_map(|y| image.pixels[y * 64..(y + 1) * 64].iter())
                .map(|pixel| pixel.a())
                .max()
                .unwrap_or(0)
        };
        assert_eq!(brightest_in_rows(0..32), 255, "the stacked traces");
        assert!(
            f32::from(brightest_in_rows(32..64)) >= LONE_TRACE_STRENGTH * 255.0 - 0.5,
            "the lone trace drew at {}",
            brightest_in_rows(32..64)
        );
    }

    #[test]
    fn density_raster_clips_far_offscreen_segments_before_walking_pixels() {
        let clipped = clip_line_to_raster(-1.0e9, 16.0, 1.0e9, 16.0, 32, 32)
            .expect("crossing segment remains visible");

        assert_eq!(clipped, (0.0, 16.0, 31.0, 16.0));
        assert!(clip_line_to_raster(-100.0, -50.0, -10.0, -5.0, 32, 32).is_none());
    }

    /// The unzoomed view is the baked extent, so it samples the whole
    /// texture — a drawn eye that is not the picture the raster holds would
    /// be a silent change of what the reader is looking at.
    #[test]
    fn the_unzoomed_view_samples_the_whole_density_texture() {
        let extent = (0.0, 2.0, -0.5, 1.5);
        let uv = density_uv(extent, (0.0, 2.0), (-0.5, 1.5));
        assert!((uv.min.x - 0.0).abs() < 1e-6, "{uv:?}");
        assert!((uv.min.y - 0.0).abs() < 1e-6, "{uv:?}");
        assert!((uv.max.x - 1.0).abs() < 1e-6, "{uv:?}");
        assert!((uv.max.y - 1.0).abs() < 1e-6, "{uv:?}");
    }

    /// Zooming samples the part of the texture the window covers, with row
    /// zero at the top of the data extent — the orientation the raster is
    /// accumulated in.
    #[test]
    fn zooming_samples_the_window_the_reader_asked_for() {
        let extent = (0.0, 2.0, 0.0, 4.0);
        let uv = density_uv(extent, (0.5, 1.5), (1.0, 3.0));
        assert!((uv.min.x - 0.25).abs() < 1e-6, "{uv:?}");
        assert!((uv.max.x - 0.75).abs() < 1e-6, "{uv:?}");
        // y = 3 is a quarter down from the top of the extent; y = 1 is three
        // quarters down.
        assert!((uv.min.y - 0.25).abs() < 1e-6, "{uv:?}");
        assert!((uv.max.y - 0.75).abs() < 1e-6, "{uv:?}");
    }
}
