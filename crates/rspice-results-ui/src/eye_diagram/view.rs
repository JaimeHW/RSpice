//! EYE — density-rendered eye diagram with the compliance mask, measured
//! metrics in the right panel.
//!
//! Acquisitions are folded segments of the active transient (built by the
//! eye pipeline). They are rasterized once per data revision into an
//! alpha-accumulated density texture — the classic persistence picture —
//! so each frame costs one textured quad plus the vector mask, instead of
//! restroking a transient's worth of segments.

use egui::Ui;

use super::EyeDiagramState;
use crate::eye_diagram::{
    EyeData, EyeRateEditor, EyeTimebase, EyeTimebaseProvenance, parse_eye_timebase,
};
use crate::presentation::PlotView;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, XScale, fmt_si};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{chip, section_header};

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

// ---------------------------------------------------------------------------
// timebase control
// ---------------------------------------------------------------------------

/// EYE docbar controls: the compliance mask, and what the eye folds at.
pub fn inline_actions(
    ui: &mut Ui,
    eye: &mut EyeDiagramState,
    timebase: Option<EyeTimebase>,
) -> Option<EyeTimebase> {
    let mut requested = None;
    let mask_on = eye.show_mask;
    let mask_response = chip(ui, "mask", mask_on).on_hover_text(if mask_on {
        "Compliance mask on — tested against the loaded acquisitions"
    } else {
        "Compliance mask off — click to test the loaded acquisitions against it"
    });
    mask_response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Button, true, mask_on, "mask")
    });
    if mask_response.clicked() {
        // Toggling runs the test now, against what is loaded now. Latching
        // the verdict at load time is how an untested mask reads as a pass.
        eye.set_show_mask(!mask_on);
    }

    let Some(timebase) = timebase else {
        return requested;
    };
    let editing = eye.rate_editor.is_some();
    let label = match timebase {
        EyeTimebase::Auto => "auto".to_owned(),
        EyeTimebase::Explicit { unit_interval } => fmt_si(unit_interval, "s", 2),
    };
    let rate_response = chip(ui, &label, editing).on_hover_text(match timebase {
        EyeTimebase::Auto => "Bit period recovered from the waveform — click to set it",
        EyeTimebase::Explicit { .. } => "Bit period set by you — click to change it",
    });
    rate_response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Button, true, editing, label.as_str())
    });
    if rate_response.clicked() {
        eye.rate_editor = if editing {
            None
        } else {
            Some(EyeRateEditor {
                text: match timebase {
                    EyeTimebase::Auto => String::new(),
                    EyeTimebase::Explicit { unit_interval } => fmt_si(unit_interval, "s", 3),
                },
                error: None,
                needs_focus: true,
            })
        };
    }

    // Take the editor for the frame so commit and cancel can replace it.
    let mut editor = eye.rate_editor.take();
    let mut commit = None;
    if let Some(open) = editor.as_mut() {
        let field = ui.add(
            egui::TextEdit::singleline(&mut open.text)
                .desired_width(96.0)
                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                .hint_text("2.5G · 400p"),
        );
        if open.needs_focus {
            field.request_focus();
            open.needs_focus = false;
        }
        let escaped = field.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Escape));
        if field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            commit = Some(open.text.clone());
        }
        if escaped {
            editor = None;
        }
        if let Some(open) = editor.as_ref()
            && let Some(message) = open.error.as_deref()
        {
            ui.label(
                egui::RichText::new(message)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.err),
            );
        }
    }
    if let Some(text) = commit {
        match parse_eye_timebase(&text) {
            Ok(unit_interval) => {
                requested = Some(EyeTimebase::Explicit { unit_interval });
                editor = None;
            }
            Err(message) => {
                editor = Some(EyeRateEditor {
                    text,
                    error: Some(message),
                    needs_focus: false,
                });
            }
        }
    }
    eye.rate_editor = editor;

    if matches!(timebase, EyeTimebase::Explicit { .. })
        && chip(ui, "auto", false)
            .on_hover_text("Go back to recovering the bit period from the waveform")
            .clicked()
    {
        requested = Some(EyeTimebase::Auto);
        eye.rate_editor = None;
    }
    requested
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
fn timebase_summary(eye: &EyeDiagramState) -> String {
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
            fmt_si(*unit_interval, "s", 3),
        ),
        Some(EyeTimebaseProvenance::Explicit {
            unit_interval,
            incoherent,
        }) => format!(
            "set · {} · {}{}",
            fmt_si(*unit_interval, "s", 3),
            fmt_si(1.0 / unit_interval, "b/s", 2),
            if *incoherent { INCOHERENT_NOTE } else { "" },
        ),
        Some(EyeTimebaseProvenance::AutoRejected(_)) | None => fmt_si(eye.data.data_rate, "b/s", 1),
    }
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the eye.
pub fn show(
    ui: &mut Ui,
    eye: &EyeDiagramState,
    source: &str,
    view: PlotView,
    texture: &mut Option<EyeTexture>,
    cache: &mut plot::DecimationCache,
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
    let summary = timebase_summary(eye);
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
    let legend = [LegendChip {
        name: "density",
        color: c.traces[0],
        on: true,
    }];
    strip::StripHeader::new("EYE", &subtitle, &legend)
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
    let accessible_detail = if eye.mask.enabled {
        format!(
            "{} folded acquisitions; compliance mask {}; {} violations from {} tested samples",
            data.traces.len(),
            if eye.show_mask { "visible" } else { "hidden" },
            eye.mask.violation_count,
            eye.mask.total_samples
        )
    } else {
        format!(
            "{} folded acquisitions; no compliance mask",
            data.traces.len()
        )
    };
    let mut spec = PlotSpec::new(x_axis, XScale::Linear, Axis::linear(y0, y1, "V"))
        .accessible_name("Eye diagram")
        .accessible_detail(&accessible_detail);

    // Bake (or fetch) the density texture. The bake walks every acquisition
    // once; frames just draw the quad.
    //
    // It is baked over the data's own extent rather than the current view, so
    // panning and zooming re-map the same texture instead of re-walking every
    // acquisition. Before, a drag rebaked on every frame of the gesture — the
    // one moment where the cost is most visible. The texture is therefore
    // keyed by the data revision, its resolution and its ink, and the draw
    // below maps the reader's window onto it.
    let plot_rect = plot::plot_rect(ui, &spec);
    let tex_size = [
        (plot_rect.width().round() as usize).max(8),
        (plot_rect.height().round() as usize).max(8),
    ];
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

    // Underlay: the density quad, then the mask above it.
    let show_mask = eye.show_mask && eye.mask.enabled && !eye.mask.inner.points.is_empty();
    let mask_fill = {
        let [r, g, b, _] = c.err.to_array();
        egui::Color32::from_rgba_unmultiplied(r, g, b, 26)
    };
    // The absolute mask mapped into display coordinates (unit intervals,
    // volts): time is scaled by the UI ratio so the mask keeps its per-UI
    // geometry at any data rate, voltage lands on the axis as stored.
    let mask_points: Vec<(f64, f64)> = eye.mask.inner_in_ui_volts().points;
    let err = c.err;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        if let Some(texture_id) = texture_id {
            painter.image(texture_id, mapper.rect, density_uv, egui::Color32::WHITE);
        }
        if show_mask {
            let points: Vec<egui::Pos2> = mask_points
                .iter()
                .map(|&(t_ui, volts)| egui::pos2(mapper.x(t_ui), mapper.y(volts)))
                .collect();
            if points.len() >= 3 {
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
        }
    }));

    let response = plot::show(ui, &spec, cache, None, None);
    out.plot = Some(response);
    out
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

/// Rasterize every acquisition into an alpha-accumulated density image —
/// the same visual as stroking each at ~18/255 alpha, computed once.
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

    let [r, g, b, _] = color.to_array();
    let pixels = coverage
        .iter()
        .map(|&cov| {
            let alpha = (cov * 18.0).min(255.0) as u8;
            egui::Color32::from_rgba_unmultiplied(r, g, b, alpha)
        })
        .collect();
    egui::ColorImage::new([width, height], pixels)
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
pub fn right_panel(ui: &mut Ui, eye: &EyeDiagramState, source: &str) {
    if eye.data.traces.is_empty() {
        section_header(ui, "Eye", None);
        crate::presentation::panel_note(
            ui,
            &unavailable_hint(eye, source).unwrap_or_else(|| {
                "Metrics appear once the eye is built from the transient.".to_owned()
            }),
        );
        return;
    }

    let summary = timebase_summary(eye);
    let m = &eye.measurements;

    section_header(ui, "Eye", None);
    let rows = [
        ("Unit interval", fmt_si(m.unit_interval, "s", 3), true),
        ("Data rate", fmt_si(m.data_rate, "b/s", 2), false),
        ("Eye height", fmt_si(m.eye_height, "V", 0), true),
        ("Eye width", format!("{:.2} UI", m.eye_width), true),
        ("Jitter rms", fmt_si(m.jitter_rms, "s", 1), false),
        ("Jitter p-p", fmt_si(m.jitter_pp, "s", 1), false),
        (
            "Crossing",
            or_unmeasured(m.crossing_percentage, |value| {
                format!("{:.1} %", value * 100.0)
            }),
            false,
        ),
        (
            "Rise 20–80",
            or_unmeasured(m.rise_time, |value| fmt_si(value, "s", 0)),
            false,
        ),
        (
            "Fall 80–20",
            or_unmeasured(m.fall_time, |value| fmt_si(value, "s", 0)),
            false,
        ),
        (
            "Q factor",
            or_unmeasured(m.q_factor, |value| {
                if value.is_infinite() {
                    "∞".to_owned()
                } else {
                    format!("{value:.1}")
                }
            }),
            false,
        ),
    ];
    crate::presentation::stat_table(ui, &rows);
    crate::presentation::panel_note(ui, &format!("Folded at {summary}."));

    if eye.mask.enabled {
        section_header(ui, "Mask", None);
        let mask = &eye.mask;
        let rows = [
            ("Template", mask.name.clone(), false),
            (
                "Violations",
                format!("{} / {}", mask.violation_count, mask.total_samples),
                true,
            ),
            (
                "Margin",
                or_unmeasured(mask.margin, |value| format!("{:+.0} %", value * 100.0)),
                true,
            ),
            (
                "Pass rate",
                or_unmeasured(mask.pass_rate(), |value| format!("{:.3} %", value * 100.0)),
                false,
            ),
        ];
        crate::presentation::stat_table(ui, &rows);
        // The mask is stored in absolute units at its authoring rate; when
        // the on-screen eye runs at a different rate, the mask time is
        // scaled by the UI ratio (see `EyeMask::inner_in_ui_volts`) — say so.
        let reference = mask.reference_data_rate;
        let current = eye.data.data_rate;
        if reference > 0.0 && (reference - current).abs() > reference * 1e-6 {
            crate::presentation::panel_note(
                ui,
                &format!(
                    "Mask authored at {} — time scaled to the current UI.",
                    fmt_si(reference, "b/s", 1)
                ),
            );
        }
    }
    crate::presentation::panel_note(
        ui,
        "Acquisitions folded at the configured bit period; thresholds 20/50/80 %.",
    );
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

            let summary = timebase_summary(&eye);
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

        let summary = timebase_summary(&eye);
        assert!(summary.starts_with("auto · "), "{summary}");
        assert!(!summary.contains("incoherent"), "{summary}");
        assert!(!timebase_is_incoherent(&eye));
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
