//! Smith chart rendering, its sheet controls and retained trace readouts.

use egui::Ui;
use num_complex::Complex64;
use rspice_results::{network_matrix::impedance_from_gamma, waveform::SharedWaveformValues};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si_readout};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{SegmentedWidth, chip, segmented, select};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

use super::circles::{ChartCircle, CircleKind, CirclePlane, TwoPortFigures, parse_decibels};
use super::grid::{self, GRID_VALUES, SmithGrid};
use super::state::{ResponseReading, SmithSheetState};
use crate::presentation::{PlotView, angle_readout, frequency_readout, plain_readout, well_hint};
use crate::strip::{self, LegendChip};

/// One trace the sheet lists, with its cached Γ-plane coordinate columns.
///
/// `index` identifies the trace in the host's inventory and keys its drawing
/// cache; the columns hold the reflection coefficient of every retained
/// point, already read through the pane's Γ/Z/Y reading for a response.
pub struct SmithPlotTrace<'a> {
    pub index: usize,
    pub name: &'a str,
    pub frequencies: SharedWaveformValues,
    pub real: SharedWaveformValues,
    pub imaginary: SharedWaveformValues,
    /// The impedance the coefficient is referred to: a port's retained Z₀
    /// for a reflection term, the pane's Z₀ for a response, and none for a
    /// transmission or mixed-mode term, which cannot claim Z or VSWR.
    pub reference_impedance_ohm: Option<f64>,
    /// Whether the trace is drawn; a hidden network term stays in the legend.
    pub visible: bool,
    /// A response the reader added, which the legend offers to remove.
    pub removable: bool,
    /// Decimation-cache key; it changes whenever the columns' meaning does.
    pub cache_key: u64,
}

/// What one frame of the chart draws: the listed traces, the reader's
/// controls, and the two-port figures and circles at the circles' frequency.
#[derive(Clone, Copy)]
pub struct SmithChart<'a> {
    pub traces: &'a [SmithPlotTrace<'a>],
    pub sheet: &'a SmithSheetState,
    pub figures: Option<&'a TwoPortFigures>,
    pub circles: &'a [ChartCircle],
}

/// What one frame of the chart reported back.
pub struct SmithFrame {
    pub plot: plot::PlotResponse,
    /// A legend chip's visibility swatch was clicked.
    pub toggled: Option<usize>,
    /// A legend chip's "Remove" action was chosen.
    pub removed: Option<usize>,
}

/// What the sheet bar offers for the active analysis.
#[derive(Debug, Clone, Copy, Default)]
pub struct SmithBarOffer<'a> {
    /// AC or harmonic-balance responses: the reading, Z₀ and response list.
    pub responses: Option<&'a [String]>,
    /// A two-port S-parameter analysis: the circle controls.
    pub two_port: bool,
    /// Retained two-port noise parameters: the noise-circle control.
    pub noise: bool,
}

/// Draw the Smith sheet's domain controls left-aligned in its bar.
///
/// Returns an expression the reader entered for the host to evaluate and
/// add, so a refusal can name the reason before anything is drawn.
pub fn domain_bar(
    ui: &mut Ui,
    sheet: &mut SmithSheetState,
    offer: SmithBarOffer<'_>,
) -> Option<String> {
    let mut grid = sheet.grid.index();
    if segmented(
        ui,
        "rspice.results.smith.grid",
        &SmithGrid::OPTIONS,
        &mut grid,
        SegmentedWidth::Natural,
    ) {
        sheet.grid = SmithGrid::from_index(grid);
    }
    let mut submitted = None;
    if let Some(retained) = offer.responses {
        ui.add_space(8.0);
        bar_label(ui, "Read as");
        let mut reading = sheet.reading.index();
        if segmented(
            ui,
            "rspice.results.smith.reading",
            &ResponseReading::OPTIONS,
            &mut reading,
            SegmentedWidth::Natural,
        ) {
            sheet.reading = ResponseReading::from_index(reading);
        }
        reference_field(ui, sheet);
        let addable = retained
            .iter()
            .filter(|name| !sheet.responses.contains(name))
            .cloned()
            .collect::<Vec<_>>();
        if !addable.is_empty() {
            let mut options = vec!["+ trace".to_owned()];
            options.extend(addable.iter().cloned());
            if let Some(picked) = select(
                ui,
                "rspice.results.smith.add_trace",
                "Add a retained response to the Smith chart",
                &options[0],
                &options,
                120.0,
            ) && picked > 0
            {
                sheet.responses.push(addable[picked - 1].clone());
            }
        }
        submitted = expression_field(ui, sheet);
    }
    if offer.two_port {
        ui.add_space(8.0);
        let circles = &mut sheet.circles;
        for (label, on, hover) in [
            (
                "STAB",
                &mut circles.stability,
                "Source and load stability circles; ticks face the unstable side",
            ),
            (
                "GA",
                &mut circles.available_gain,
                "Constant available-gain circles on the source plane",
            ),
            (
                "GP",
                &mut circles.power_gain,
                "Constant operating-power-gain circles on the load plane",
            ),
        ] {
            if chip(ui, label, *on).on_hover_text(hover).clicked() {
                *on = !*on;
            }
        }
        if offer.noise
            && chip(ui, "NF", circles.noise)
                .on_hover_text("Constant noise-figure circles on the source plane")
                .clicked()
        {
            circles.noise = !circles.noise;
        }
        if sheet.circles.available_gain || sheet.circles.power_gain {
            decibel_field(ui, "rspice.results.smith.gain_db", "G", true, sheet);
        }
        if offer.noise && sheet.circles.noise {
            decibel_field(ui, "rspice.results.smith.noise_db", "NF", false, sheet);
        }
    }
    submitted
}

fn bar_label(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

/// The pane's Z₀ for responses, edited in place and committed on Enter or
/// focus loss; a value that is not a finite positive resistance is refused.
fn reference_field(ui: &mut Ui, sheet: &mut SmithSheetState) {
    bar_label(ui, "Z\u{2080}");
    let current = format!("{}", sheet.reference_ohm);
    let mut text = sheet.reference_draft.clone().unwrap_or(current);
    let field = ui.add(
        egui::TextEdit::singleline(&mut text)
            .id_salt("rspice.results.smith.reference")
            .desired_width(44.0)
            .font(theme::field_mono()),
    );
    let field = field.on_hover_text("Reference impedance the responses are read against");
    if field.has_focus() {
        sheet.reference_draft = Some(text);
    } else if field.lost_focus() || sheet.reference_draft.is_some() {
        if let Ok(value) =
            rspice_vocabulary::quantity::engineering::parse_engineering_value(text.trim())
            && value.is_finite()
            && value > 0.0
        {
            sheet.reference_ohm = value;
        }
        sheet.reference_draft = None;
    }
    bar_label(ui, "\u{3a9}");
}

/// "+ expr": a calculator expression to read onto the chart.
fn expression_field(ui: &mut Ui, sheet: &mut SmithSheetState) -> Option<String> {
    let Some(mut text) = sheet.expression_draft.take() else {
        if chip(ui, "+ expr", false)
            .on_hover_text("Add a complex expression, such as V(in)/I(Vin) read as Z")
            .clicked()
        {
            sheet.expression_draft = Some(String::new());
            sheet.expression_error = None;
        }
        if let Some(error) = &sheet.expression_error {
            error_label(ui, error);
        }
        return None;
    };
    let field = ui.add(
        egui::TextEdit::singleline(&mut text)
            .id_salt("rspice.results.smith.expression")
            .desired_width(140.0)
            .font(theme::field_mono())
            .hint_text("V(in)/I(Vin)"),
    );
    if text.is_empty() && !field.has_focus() && !field.lost_focus() {
        field.request_focus();
    }
    let escaped = ui.input(|input| input.key_pressed(egui::Key::Escape));
    if field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)) {
        let entered = text.trim().to_owned();
        sheet.expression_error = None;
        return (!entered.is_empty()).then_some(entered);
    }
    if !escaped && !field.lost_focus() {
        sheet.expression_draft = Some(text);
    }
    if let Some(error) = &sheet.expression_error {
        error_label(ui, error);
    }
    None
}

fn error_label(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(message)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.err),
    );
}

/// Gain or noise-figure circle values in dB; empty is the automatic set.
fn decibel_field(
    ui: &mut Ui,
    id: &'static str,
    label: &str,
    gain: bool,
    sheet: &mut SmithSheetState,
) {
    bar_label(ui, label);
    let (values, draft) = if gain {
        (&mut sheet.gain_db, &mut sheet.gain_draft)
    } else {
        (&mut sheet.noise_db, &mut sheet.noise_draft)
    };
    let current = values
        .iter()
        .map(|value| format!("{value}"))
        .collect::<Vec<_>>()
        .join(" ");
    let mut text = draft.clone().unwrap_or(current);
    let field = ui.add(
        egui::TextEdit::singleline(&mut text)
            .id_salt(id)
            .desired_width(72.0)
            .font(theme::field_mono())
            .hint_text("auto"),
    );
    let field = field.on_hover_text(if gain {
        "Gain circle values in dB; empty draws three below the maximum"
    } else {
        "Noise figure circle values in dB; empty draws three above NFmin"
    });
    if field.has_focus() {
        *draft = Some(text);
    } else if field.lost_focus() || draft.is_some() {
        if let Ok(parsed) = parse_decibels(&text) {
            *values = parsed;
        }
        *draft = None;
    }
    bar_label(ui, "dB");
}

/// Inks for one frame of the chart, all resolved from the canvas theme so
/// they stand off the well whatever the interface mode beside it.
#[derive(Clone, Copy)]
struct ChartInks {
    grid: egui::Color32,
    admittance: egui::Color32,
    admittance_label: egui::Color32,
    boundary: egui::Color32,
    label: egui::Color32,
    stability: egui::Color32,
    gain: egui::Color32,
    noise: egui::Color32,
    well: egui::Color32,
}

impl ChartInks {
    fn of(tokens: &Tokens, grid: SmithGrid) -> Self {
        let c = tokens.color;
        // The Smith grid is the chart's coordinate system, not background
        // ruling, so it stands a step above the Cartesian grid lines.
        let ruling = c.canvas_text_faint.gamma_multiply(0.42);
        // Loci take the canvas's trace hues from slot 0, and a two-port lists
        // at most its four coefficients, so the circles and the second grid
        // family take slots 4 to 6, which no locus of a two-port uses. Every
        // trace hue stands 4.5:1 off its canvas, so circle tags read too.
        let second_family = c.traces[6];
        Self {
            grid: ruling,
            // Alone, the admittance grid is the chart's grid. Beside the
            // impedance grid it takes its own ink, as on a printed Z-Y chart.
            admittance: if grid == SmithGrid::ImpedanceAdmittance {
                second_family.gamma_multiply(0.32)
            } else {
                ruling
            },
            admittance_label: if grid == SmithGrid::ImpedanceAdmittance {
                second_family
            } else {
                c.canvas_text_faint
            },
            boundary: c.border_strong,
            label: c.canvas_text_faint,
            gain: c.traces[4],
            stability: c.traces[5],
            noise: c.canvas_text_dim,
            well: c.canvas_bg,
        }
    }
}

/// Grid labels the chart writes, in Γ coordinates with their text.
fn grid_labels(mode: SmithGrid) -> Vec<(Complex64, String, bool, LabelPlace)> {
    let mut labels = Vec::new();
    let both = mode == SmithGrid::ImpedanceAdmittance;
    let value_text = |value: f64| {
        if value.fract() == 0.0 {
            format!("{value:.0}")
        } else {
            format!("{value}")
        }
    };
    if mode.draws_impedance() {
        for value in GRID_VALUES {
            labels.push((
                grid::real_axis_crossing(value, false),
                value_text(value),
                false,
                LabelPlace::AboveAxis,
            ));
            for sign in [1.0, -1.0] {
                labels.push((
                    grid::rim_crossing(sign * value, false),
                    format!(
                        "{}j{}",
                        if sign < 0.0 { "\u{2212}" } else { "" },
                        value_text(value)
                    ),
                    false,
                    LabelPlace::OutsideRim,
                ));
            }
        }
    }
    if mode.draws_admittance() {
        for value in GRID_VALUES {
            labels.push((
                grid::real_axis_crossing(value, true),
                value_text(value),
                true,
                if both {
                    LabelPlace::BelowAxis
                } else {
                    LabelPlace::AboveAxis
                },
            ));
            // On a Z+Y chart the susceptance arcs meet the rim exactly where
            // the opposite-signed reactance arcs do, so only the impedance
            // grid labels the rim.
            if !both {
                for sign in [1.0, -1.0] {
                    labels.push((
                        grid::rim_crossing(sign * value, true),
                        format!(
                            "{}j{}",
                            if sign < 0.0 { "\u{2212}" } else { "" },
                            value_text(value)
                        ),
                        true,
                        LabelPlace::OutsideRim,
                    ));
                }
            }
        }
    }
    labels
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LabelPlace {
    AboveAxis,
    BelowAxis,
    OutsideRim,
}

fn paint_polyline(
    painter: &egui::Painter,
    mapper: &plot::PlotMapper,
    points: &[Complex64],
    stroke: egui::Stroke,
    dashed: bool,
) {
    if points.len() < 2 {
        return;
    }
    let screen = points
        .iter()
        .map(|point| egui::pos2(mapper.x(point.re), mapper.y(point.im)))
        .collect::<Vec<_>>();
    if dashed {
        painter.extend(egui::Shape::dashed_line(&screen, stroke, 6.0, 4.0));
    } else {
        painter.add(egui::Shape::line(screen, stroke));
    }
}

fn paint_grid(
    painter: &egui::Painter,
    mapper: &plot::PlotMapper,
    mode: SmithGrid,
    inks: ChartInks,
) {
    let grid_stroke = egui::Stroke::new(1.0, inks.grid);
    let admittance_stroke = egui::Stroke::new(1.0, inks.admittance);
    let family = |circles: &mut dyn Iterator<Item = grid::GridCircle>, stroke| {
        for circle in circles {
            paint_polyline(
                painter,
                mapper,
                &grid::arc_points(circle.center, circle.radius),
                stroke,
                false,
            );
        }
    };
    if mode.draws_admittance() {
        family(
            &mut GRID_VALUES
                .iter()
                .map(|value| grid::conductance_circle(*value)),
            admittance_stroke,
        );
        family(
            &mut GRID_VALUES
                .iter()
                .flat_map(|value| [*value, -*value])
                .map(grid::susceptance_circle),
            admittance_stroke,
        );
    }
    if mode.draws_impedance() {
        family(
            &mut GRID_VALUES
                .iter()
                .map(|value| grid::resistance_circle(*value)),
            grid_stroke,
        );
        family(
            &mut GRID_VALUES
                .iter()
                .flat_map(|value| [*value, -*value])
                .map(grid::reactance_circle),
            grid_stroke,
        );
    }
    let center = egui::pos2(mapper.x(0.0), mapper.y(0.0));
    let unit = (mapper.x(1.0) - mapper.x(0.0)).abs();
    painter.hline(
        egui::Rangef::new(mapper.x(-1.0), mapper.x(1.0)),
        center.y,
        grid_stroke,
    );
    painter.circle_stroke(center, unit, egui::Stroke::new(1.2, inks.boundary));

    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    for (gamma, text, admittance, place) in grid_labels(mode) {
        let color = if admittance {
            inks.admittance_label
        } else {
            inks.label
        };
        let at = egui::pos2(mapper.x(gamma.re), mapper.y(gamma.im));
        let (position, align) = match place {
            LabelPlace::AboveAxis => (at + egui::vec2(2.0, -1.0), egui::Align2::LEFT_BOTTOM),
            LabelPlace::BelowAxis => (at + egui::vec2(2.0, 1.0), egui::Align2::LEFT_TOP),
            LabelPlace::OutsideRim => {
                let outward = egui::vec2(gamma.re as f32, -gamma.im as f32);
                (at + outward * 11.0, egui::Align2::CENTER_CENTER)
            }
        };
        painter.text(position, align, text, font.clone(), color);
    }
}

/// A circle on the chart: its in-disc arc, a tag at the arc's middle and,
/// for a stability circle, short ticks facing the unstable side.
fn paint_circle(
    painter: &egui::Painter,
    mapper: &plot::PlotMapper,
    circle: &ChartCircle,
    inks: ChartInks,
) {
    let color = match circle.kind {
        CircleKind::Stability { .. } => inks.stability,
        CircleKind::AvailableGain { .. } | CircleKind::PowerGain { .. } => inks.gain,
        CircleKind::NoiseFigure { .. } => inks.noise,
    };
    let center = circle.circle.center;
    let radius = circle.circle.radius;
    let points = grid::arc_points(center, radius);
    if points.len() < 2 {
        return;
    }
    // The source plane is dashed and the load plane solid, so a source
    // circle never reads as a load circle of the same colour.
    let dashed = circle.plane == CirclePlane::Source;
    paint_polyline(
        painter,
        mapper,
        &points,
        egui::Stroke::new(1.4, color),
        dashed,
    );
    let screen = |point: Complex64| egui::pos2(mapper.x(point.re), mapper.y(point.im));
    if let CircleKind::Stability { stable_inside } = circle.kind {
        // Ticks every ~14 px of arc, pointing into the unstable region.
        let unit = f64::from((mapper.x(1.0) - mapper.x(0.0)).abs());
        let stride = ((14.0 / (radius * unit)) / (std::f64::consts::TAU / 256.0))
            .ceil()
            .max(1.0) as usize;
        for point in points.iter().step_by(stride) {
            let outward = (point - center) / radius;
            let toward_unstable = if stable_inside { outward } else { -outward };
            let from = screen(*point);
            let tip =
                from + egui::vec2(toward_unstable.re as f32, -toward_unstable.im as f32) * 5.0;
            painter.line_segment([from, tip], egui::Stroke::new(1.0, color));
        }
    }
    // The tag sits on a patch of the well so grid lines and other circles
    // crossing it do not break the text up.
    let middle = points[points.len() / 2];
    let galley = painter.layout_no_wrap(
        circle.tag(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        color,
    );
    let tag = egui::Align2::LEFT_BOTTOM
        .anchor_size(screen(middle) + egui::vec2(3.0, -3.0), galley.size());
    painter.rect_filled(tag.expand(1.5), 2.0, inks.well);
    painter.galley(tag.min, galley, color);
}

/// Admittance (conductance, susceptance) in siemens from a reflection
/// coefficient referred to `z0`: Y = (1/Z₀)(1 − Γ)/(1 + Γ).
fn admittance_from_gamma(gamma_re: f64, gamma_im: f64, z0: f64) -> Option<(f64, f64)> {
    impedance_from_gamma(-gamma_re, -gamma_im, z0)
        .map(|(resistance, reactance)| (resistance / (z0 * z0), reactance / (z0 * z0)))
}

/// The readout rows for one sample: frequency, Γ, then Z and/or Y as the
/// grid reads, and VSWR, when the trace has a reference impedance.
fn sample_rows(
    trace: &SmithPlotTrace<'_>,
    index: usize,
    grid: SmithGrid,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) -> Vec<crate::presentation::CardRow> {
    let gamma_re = trace.real[index];
    let gamma_im = trace.imaginary[index];
    let frequency = trace.frequencies.get(index).copied().unwrap_or(0.0);
    let mag = gamma_re.hypot(gamma_im);
    let phase = gamma_im.atan2(gamma_re);
    let mut rows = vec![
        (
            "f".to_owned(),
            frequency_readout(quantity_policy, frequency, digits),
        ),
        (
            "Γ".to_owned(),
            format!(
                "{} ∠ {}",
                plain_readout(mag, "", digits),
                angle_readout(quantity_policy, phase, digits)
            ),
        ),
    ];
    let Some(reference) = trace.reference_impedance_ohm else {
        rows.push(("Readout".to_owned(), "complex coefficient only".to_owned()));
        return rows;
    };
    rows.push(("Z₀".to_owned(), fmt_si_readout(reference, "Ω", digits)));
    if grid.draws_impedance() {
        rows.push((
            "Z".to_owned(),
            impedance_from_gamma(gamma_re, gamma_im, reference).map_or_else(
                || "open".to_owned(),
                |(resistance, reactance)| impedance_readout(resistance, reactance, digits),
            ),
        ));
    }
    if grid.draws_admittance() {
        rows.push((
            "Y".to_owned(),
            admittance_from_gamma(gamma_re, gamma_im, reference).map_or_else(
                || "short".to_owned(),
                |(conductance, susceptance)| complex_readout(conductance, susceptance, "S", digits),
            ),
        ));
    }
    rows.push((
        "VSWR".to_owned(),
        if mag < 1.0 {
            plain_readout((1.0 + mag) / (1.0 - mag), "", digits)
        } else {
            "∞".to_owned()
        },
    ));
    rows
}

/// Draw the listed loci, the grid and the circles, and update only the
/// sheet's local view and marker. The host owns source qualification and
/// cache invalidation. Readouts use the reader's `significant_digits`.
pub fn show(
    ui: &mut Ui,
    chart: &SmithChart<'_>,
    view: &mut PlotView,
    pin: &mut Option<(usize, usize)>,
    cache: &mut plot::DecimationCache,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) -> Option<SmithFrame> {
    let SmithChart {
        traces,
        sheet,
        figures,
        circles,
    } = *chart;
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    if traces.is_empty() && figures.is_none() && sheet.responses.is_empty() {
        well_hint(
            ui,
            "No complex traces — run an SP, PSP, HBSP, AC or HB analysis",
        );
        return None;
    }
    let legend: Vec<LegendChip> = traces
        .iter()
        .enumerate()
        .map(|(slot, prepared)| LegendChip {
            name: prepared.name,
            color: c.traces[slot % c.traces.len()],
            on: prepared.visible,
        })
        .collect();
    let mut references = traces
        .iter()
        .filter(|prepared| prepared.visible)
        .filter_map(|prepared| prepared.reference_impedance_ohm)
        .collect::<Vec<_>>();
    references.sort_by(f64::total_cmp);
    references.dedup_by(|left, right| left.to_bits() == right.to_bits());
    let mut subtitle = match references.as_slice() {
        [reference] => format!("Z₀ = {reference} Ω"),
        [] => "Coefficient loci".to_owned(),
        _ => "Per-port retained Z₀".to_owned(),
    };
    if let Some(figures) = figures
        && !circles.is_empty()
    {
        subtitle.push_str(&format!(
            " · circles at {}",
            frequency_readout(quantity_policy, figures.frequency, significant_digits)
        ));
    }
    let removable_from = traces
        .iter()
        .position(|prepared| prepared.removable)
        .unwrap_or(traces.len());
    let header = strip::StripHeader::new("SMITH", &subtitle, &legend)
        .removable_from(removable_from)
        .show(ui);

    let (x0, x1) = view.x.unwrap_or((-1.12, 1.12));
    let (y0, y1) = view.y.unwrap_or((-1.12, 1.12));
    let drawn = traces
        .iter()
        .filter(|prepared| prepared.visible)
        .collect::<Vec<_>>();
    let accessible_detail = format!(
        "{} visible loci with {} retained samples; {} with a reference impedance; {} circles",
        drawn.len(),
        drawn
            .iter()
            .map(|prepared| prepared.real.len())
            .sum::<usize>(),
        drawn
            .iter()
            .filter(|prepared| prepared.reference_impedance_ohm.is_some())
            .count(),
        circles.len()
    );
    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, "Re Γ"),
        XScale::Linear,
        Axis::linear(y0, y1, "Im Γ"),
    )
    .accessible_name("Smith chart")
    .accessible_detail(&accessible_detail)
    .without_x_axis_chrome();
    // The chart is read on its own grid, as a printed Smith chart is: no
    // Cartesian ticks, units or tick gutter around the disc.
    spec.x.ticks.clear();
    spec.y.ticks.clear();
    spec.x.unit.clear();
    spec.y.unit.clear();
    spec.left_margin = 12.0;
    for (slot, prepared) in traces.iter().enumerate() {
        if !prepared.visible {
            continue;
        }
        spec.traces.push(
            // Re Γ is a coordinate, not an ordering: the locus crosses the
            // same abscissa on the way out and on the way back.
            Trace::new(
                &prepared.real,
                &prepared.imaginary,
                c.traces[slot % c.traces.len()],
            )
            .parametric()
            .cache_key(prepared.cache_key),
        );
    }

    let inks = ChartInks::of(&t, sheet.grid);
    let mode = sheet.grid;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        paint_grid(painter, mapper, mode, inks);
        for circle in circles {
            paint_circle(painter, mapper, circle, inks);
        }
    }));

    // Region sized so the INNER plot area is square — the chart's circle
    // grid and the Γ loci must share one scale.
    let avail = ui.available_rect_before_wrap();
    let rect = plot::square_outer_rect(avail, &spec);
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );

    let response = plot::show(&mut plot_ui, &spec, cache, None, None);
    if response.view.any() {
        let change = crate::presentation::square_xy_view_change((x0, x1), (y0, y1), response.view);
        view.apply(&change);
    }
    if traces.is_empty() {
        let hint = if sheet.responses.is_empty() {
            "Add a retained response or an expression such as V(in)/I(Vin)"
        } else {
            "None of the listed responses evaluates on this analysis"
        };
        plot_ui.painter().text(
            response.plot_rect.center_top() + egui::vec2(0.0, 12.0),
            egui::Align2::CENTER_TOP,
            hint,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.canvas_text_dim,
        );
    }

    // Interactive readout: nearest locus point on hover, click to pin a
    // marker. The chart space is the complex-coefficient plane.
    let ranges = ((x0, x1), (y0, y1));
    let mut hovered: Option<(usize, usize)> = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 14.0f32 * 14.0;
        for (slot, prepared) in traces.iter().enumerate() {
            if !prepared.visible {
                continue;
            }
            let (re, im) = (&prepared.real, &prepared.imaginary);
            for i in 0..re.len() {
                if !(re[i].is_finite() && im[i].is_finite()) {
                    continue;
                }
                let pos = crate::presentation::xy_screen_pos(
                    response.plot_rect,
                    (re[i], im[i]),
                    ranges.0,
                    ranges.1,
                );
                let d2 = pos.distance_sq(pointer);
                if d2 < best {
                    best = d2;
                    hovered = Some((slot, i));
                }
            }
        }
    }

    if response.response.clicked() {
        *pin = match hovered {
            Some(hit) if *pin == Some(hit) => None,
            Some(hit) => Some(hit),
            None => None,
        };
    }

    let pinned = pin.filter(|(slot, i)| {
        traces
            .get(*slot)
            .is_some_and(|prepared| prepared.visible && *i < prepared.real.len())
    });
    let painter = plot_ui.painter();
    if let Some((slot, i)) = pinned {
        let prepared = &traces[slot];
        let pos = crate::presentation::xy_screen_pos(
            response.plot_rect,
            (prepared.real[i], prepared.imaginary[i]),
            ranges.0,
            ranges.1,
        );
        let color = c.traces[slot % c.traces.len()];
        painter.circle_filled(pos, 3.0, color);
        painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.8, color));
        painter.text(
            pos + egui::vec2(7.0, -7.0),
            egui::Align2::LEFT_BOTTOM,
            "m1",
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            color,
        );
    }
    // The pinned marker reads out in the side panel; the card is for the
    // point under the pointer, so it never covers the circles for good.
    if let Some((slot, i)) = hovered {
        let prepared = &traces[slot];
        let pos = crate::presentation::xy_screen_pos(
            response.plot_rect,
            (prepared.real[i], prepared.imaginary[i]),
            ranges.0,
            ranges.1,
        );
        painter.circle_stroke(pos, 4.0, egui::Stroke::new(1.5, c.canvas_cursor_a));
        crate::presentation::point_card(
            &plot_ui,
            response.plot_rect,
            pos,
            prepared.name,
            c.traces[slot % c.traces.len()],
            &sample_rows(prepared, i, sheet.grid, quantity_policy, significant_digits),
        );
    }
    Some(SmithFrame {
        plot: response,
        toggled: header.legend_visibility_clicked.or(header.legend_clicked),
        removed: header.legend_removed,
    })
}

/// The marker's trace and sample, and the two-port figures at the circles'
/// frequency, at the reader's `significant_digits`.
pub fn right_panel(
    ui: &mut Ui,
    chart: &SmithChart<'_>,
    pin: Option<(usize, usize)>,
    quantity_policy: &QuantityPresentationPolicy,
    significant_digits: usize,
) {
    let SmithChart {
        traces,
        sheet,
        figures,
        circles,
    } = *chart;
    let grid = sheet.grid;
    let digits = significant_digits;
    section_header(ui, "Smith", None);
    let marked = pin.and_then(|(slot, index)| {
        traces
            .get(slot)
            .filter(|trace| trace.visible && index < trace.real.len())
            .map(|trace| (trace, index))
    });
    let Some(trace) = marked
        .map(|(trace, _)| trace)
        .or_else(|| traces.iter().find(|trace| trace.visible))
    else {
        crate::presentation::panel_note(ui, "Trace metrics appear once a complex trace is drawn.");
        two_port_rows(ui, figures, circles, quantity_policy, digits);
        return;
    };
    let frequencies = &trace.frequencies;
    let maximum_gamma = (0..trace.real.len())
        .map(|index| trace.real[index].hypot(trace.imaginary[index]))
        .filter(|value| value.is_finite())
        .max_by(f64::total_cmp);
    // The chart has no frequency axis, so the sweep states itself; a locus
    // with no reference impedance has no VSWR for the reader to read.
    let mut rows = vec![
        ("Trace", trace.name.to_owned(), true),
        (
            "Sweep",
            match (frequencies.first(), frequencies.last()) {
                (Some(a), Some(b)) => format!(
                    "{} – {}",
                    frequency_readout(quantity_policy, *a, digits),
                    frequency_readout(quantity_policy, *b, digits)
                ),
                _ => "—".to_owned(),
            },
            false,
        ),
    ];
    if let Some(reference) = trace.reference_impedance_ohm {
        rows.push(("Z₀", format!("{reference} Ω"), false));
    }
    if let Some(value) = maximum_gamma {
        rows.push(("|Γ| max", plain_readout(value, "", digits), true));
        if trace.reference_impedance_ohm.is_some() {
            rows.push((
                "Max VSWR",
                if value < 1.0 {
                    format!(
                        "{} : 1",
                        plain_readout((1.0 + value) / (1.0 - value), "", digits)
                    )
                } else {
                    "∞".to_owned()
                },
                false,
            ));
        }
    }
    crate::presentation::stat_table(ui, &rows);
    if let Some((trace, index)) = marked {
        section_header(ui, "Marker m1", None);
        let rows = sample_rows(trace, index, grid, quantity_policy, digits)
            .into_iter()
            .collect::<Vec<_>>();
        // The trace's Z₀ heads the panel already; the hover card keeps it
        // because it stands alone.
        let table = rows
            .iter()
            .filter(|(key, _)| key != "Z₀")
            .map(|(key, value)| (key.as_str(), value.clone(), false))
            .collect::<Vec<_>>();
        crate::presentation::stat_table(ui, &table);
    }
    two_port_rows(ui, figures, circles, quantity_policy, digits);
}

/// K, μ, μ′, the maximum gain, the noise parameters and the stability
/// circles at the circles' frequency.
fn two_port_rows(
    ui: &mut Ui,
    figures: Option<&TwoPortFigures>,
    circles: &[ChartCircle],
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) {
    let Some(figures) = figures else {
        return;
    };
    section_header(ui, "Two-port", None);
    let device = figures.scattering;
    let mut rows = vec![
        (
            "At",
            frequency_readout(quantity_policy, figures.frequency, digits),
            false,
        ),
        ("K", plain_readout(device.rollett_k(), "", digits), false),
        ("μ", plain_readout(device.mu_load(), "", digits), false),
        ("μ′", plain_readout(device.mu_source(), "", digits), false),
        (
            "Stability",
            if device.is_unconditionally_stable() {
                "unconditional".to_owned()
            } else {
                "potentially unstable".to_owned()
            },
            true,
        ),
    ];
    if let Some((label, value)) = figures.maximum_gain_text() {
        rows.push((label, value, false));
    }
    if let Some(noise) = figures.noise {
        rows.push((
            "NFmin",
            format!("{:.3} dB", 10.0 * noise.minimum_noise_factor.log10()),
            false,
        ));
        rows.push((
            "Rn",
            fmt_si_readout(noise.noise_resistance_ohm, "Ω", digits),
            false,
        ));
        rows.push((
            "Γopt",
            format!(
                "{} ∠ {}",
                plain_readout(noise.optimum_source_reflection.norm(), "", digits),
                angle_readout(
                    quantity_policy,
                    noise.optimum_source_reflection.arg(),
                    digits
                )
            ),
            false,
        ));
    }
    let mut circle_rows = Vec::new();
    for circle in circles {
        let CircleKind::Stability { stable_inside } = circle.kind else {
            continue;
        };
        let [centre, radius, side] = match circle.plane {
            CirclePlane::Source => ["Source centre", "Source radius", "Source stable"],
            CirclePlane::Load => ["Load centre", "Load radius", "Load stable"],
        };
        circle_rows.push((
            centre,
            format!(
                "{} ∠ {}",
                plain_readout(circle.circle.center.norm(), "", digits),
                angle_readout(quantity_policy, circle.circle.center.arg(), digits),
            ),
            false,
        ));
        circle_rows.push((
            radius,
            plain_readout(circle.circle.radius, "", digits),
            false,
        ));
        circle_rows.push((
            side,
            if stable_inside { "inside" } else { "outside" }.to_owned(),
            false,
        ));
    }
    rows.extend(circle_rows);
    crate::presentation::stat_table(ui, &rows);
}

/// `R ± jX` at the reader's significant digits: `50.12 Ω + j12.35 Ω`.
fn impedance_readout(resistance: f64, reactance: f64, significant_digits: usize) -> String {
    complex_readout(resistance, reactance, "Ω", significant_digits)
}

fn complex_readout(real: f64, imaginary: f64, unit: &str, significant_digits: usize) -> String {
    format!(
        "{} {} j{}",
        fmt_si_readout(real, unit, significant_digits),
        if imaginary >= 0.0 { "+" } else { "−" },
        fmt_si_readout(imaginary.abs(), unit, significant_digits)
    )
}

#[cfg(test)]
mod tests;
