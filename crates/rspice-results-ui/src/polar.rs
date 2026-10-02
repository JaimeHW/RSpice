//! POLAR — one retained complex response on the polar plane: concentric
//! radius rings, 30° spokes, the swept locus with its direction of travel,
//! and the two shared frequency cursors read as A │ B │ Δ.
//!
//! Nothing here is derived that the result does not carry. Mixed-mode terms
//! are offered only when the engine retained mixed-mode traces — a plain SP
//! result lists its physical terms and nothing else — and the quantity list
//! is built from the retained port count rather than from a fixed menu.
//!
//! The canvas is painted directly instead of going through the Cartesian
//! plot engine: a polar chart has no Re/Im tick chrome to label, and in dB
//! mode the drawing plane is a radial warp of the complex plane, so Cartesian
//! tick numbers beside it would name coordinates that are not the data's.

use std::sync::Arc;

use egui::{Sense, Ui, WidgetInfo, WidgetType};

use crate::waveform::WaveformData;
use rspice_results::{analysis_type::AnalysisType, family_metadata::AnalysisResultFamilyMetadata};
type AnalysisResult = rspice_results::analysis_result::AnalysisResult<WaveformData>;
use rspice_ui_kit::plot::fmt_si;
use rspice_ui_kit::theme::{self, FontWeight, paint_focus_ring};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{SegmentedWidth, chip, section_header, segmented, select};

use crate::presentation::well_hint;
use crate::strip::{self, LegendChip};

/// How the radius axis is ruled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PolarRadius {
    /// Linear magnitude: the locus is the complex plane itself.
    #[default]
    Magnitude,
    /// 20·log₁₀ magnitude over a 50 dB window ending at the ceiling.
    Decibels,
}

impl PolarRadius {
    const OPTIONS: [&'static str; 2] = ["|·|", "dB"];

    const fn index(self) -> usize {
        match self {
            Self::Magnitude => 0,
            Self::Decibels => 1,
        }
    }

    const fn from_index(index: usize) -> Self {
        if index == 1 {
            Self::Decibels
        } else {
            Self::Magnitude
        }
    }
}

/// The dB window the radius spans, in decibels. Five 10 dB rings.
const DECIBEL_SPAN: f64 = 50.0;

/// Session controls for the polar sheet.
///
/// The selected quantity is held by its retained waveform name, never by
/// ordinal: a re-run that retains one term fewer would otherwise silently
/// move the reader onto a different network term under the same control.
#[derive(Debug, Clone, Default)]
pub struct PolarSheetState {
    pub quantity: Option<String>,
    pub radius: PolarRadius,
    pub decade_marks: bool,
    pub normalize: bool,
}

/// One quantity the sheet can draw, resolved from the retained result.
#[derive(Debug, Clone)]
pub struct PolarQuantity {
    /// Retained waveform name — the selection key.
    name: String,
    /// What the bar and the pane head call it.
    label: String,
    waveform_index: usize,
    /// Mixed-mode terms first in mode order, then physical, then by port
    /// pair. The mode rank keeps `Sdd` before `Sdc` before `Scd` before
    /// `Scc`, which is the order a differential network is read in.
    order: (u8, u8, usize, usize),
    /// Retained per-port reference impedance, for a physical reflection term.
    reference_impedance_ohm: Option<f64>,
    /// The producer's unit for the linear magnitude, when it stated one.
    unit: String,
}

/// Whether a retained waveform carries a complex series this sheet can draw.
///
/// Deliberately O(1): the deep finite/ordering check is
/// [`AnalysisResult::validate_retained_evidence`], which the availability gate
/// resolves through the workspace memo once per dataset generation. Repeating
/// it here would walk every sample of every trace on every frame.
fn complex_is_plottable(waveform: &WaveformData) -> bool {
    waveform.complex.as_ref().is_some_and(|complex| {
        !complex.real.is_empty()
            && complex.real.len() == complex.imag.len()
            && complex.real.len() == waveform.x.len()
    })
}

/// The mixed-mode prefix of an S-parameter trace name, as the engine spelled
/// it. `None` is a physical term.
fn mixed_mode_prefix(name: &str) -> Option<(char, char)> {
    let core = name.trim().trim_matches('|');
    let suffix = core.strip_prefix('S').or_else(|| core.strip_prefix('s'))?;
    let mut characters = suffix.chars();
    let output = characters.next()?.to_ascii_lowercase();
    let input = characters.next()?.to_ascii_lowercase();
    matches!(
        (output, input),
        ('d', 'd') | ('d', 'c') | ('c', 'd') | ('c', 'c')
    )
    .then_some((output, input))
}

/// Where a mixed-mode term sits in the reading order of a differential
/// network: the differential terms, then each conversion, then the common
/// ones. A physical term has no mode and ranks first among its own kind.
fn mixed_mode_rank(name: &str) -> u8 {
    match mixed_mode_prefix(name) {
        Some(('d', 'd')) | None => 0,
        Some(('d', 'c')) => 1,
        Some(('c', 'd')) => 2,
        _ => 3,
    }
}

/// `Sdd21 · D2 ← D1`, `S21 · Port 2 ← Port 1`, `S11 · Port 1`.
fn network_term_label(name: &str, output_port: usize, input_port: usize) -> String {
    let term = name.trim().trim_matches('|');
    match mixed_mode_prefix(name) {
        Some((output_mode, input_mode)) => {
            let output_mode = output_mode.to_ascii_uppercase();
            let input_mode = input_mode.to_ascii_uppercase();
            if output_port == input_port && output_mode == input_mode {
                format!("{term} \u{b7} {output_mode}{output_port}")
            } else {
                format!(
                    "{term} \u{b7} {output_mode}{output_port} \u{2190} {input_mode}{input_port}"
                )
            }
        }
        None if output_port == input_port => format!("{term} \u{b7} Port {output_port}"),
        None => format!("{term} \u{b7} Port {output_port} \u{2190} Port {input_port}"),
    }
}

/// Every complex quantity the active analysis retained, in bar order.
pub fn quantities(analysis: &AnalysisResult) -> Vec<PolarQuantity> {
    if !analysis.success {
        return Vec::new();
    }
    let network = match (&analysis.analysis_type, analysis.family_metadata.as_ref()) {
        (
            AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp,
            Some(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm,
                ..
            }),
        ) => Some(reference_impedances_ohm.clone()),
        _ => None,
    };
    let mut resolved = Vec::new();
    for (waveform_index, waveform) in analysis.waveforms.iter().enumerate() {
        if !complex_is_plottable(waveform) {
            continue;
        }
        let complex = waveform
            .complex
            .as_ref()
            .expect("plottability checked the complex components");
        match network.as_ref() {
            Some(reference_impedances_ohm) => {
                let Some(identity) =
                    rspice_results::network_matrix::trace_identity(&complex.source_name)
                        .or_else(|| rspice_results::network_matrix::trace_identity(&waveform.name))
                else {
                    continue;
                };
                // Mixed-mode terms are indexed over half as many logical
                // ports as the run declares physical ones, exactly as the
                // Smith sheet reads them.
                let port_count = if identity.physical_ports {
                    reference_impedances_ohm.len()
                } else if reference_impedances_ohm.len().is_multiple_of(2) {
                    reference_impedances_ohm.len() / 2
                } else {
                    continue;
                };
                if identity.output_port > port_count || identity.input_port > port_count {
                    continue;
                }
                let term = if complex.source_name.trim().is_empty() {
                    waveform.name.as_str()
                } else {
                    complex.source_name.as_str()
                };
                resolved.push(PolarQuantity {
                    name: waveform.name.clone(),
                    label: network_term_label(term, identity.output_port, identity.input_port),
                    waveform_index,
                    order: (
                        u8::from(identity.physical_ports),
                        mixed_mode_rank(term),
                        identity.input_port,
                        identity.output_port,
                    ),
                    reference_impedance_ohm: (identity.physical_ports
                        && identity.output_port == identity.input_port)
                        .then(|| reference_impedances_ohm[identity.output_port - 1]),
                    unit: waveform.unit.clone().unwrap_or_default(),
                });
            }
            None => {
                if !matches!(
                    analysis.analysis_type,
                    AnalysisType::Ac | AnalysisType::HarmonicBalance
                ) {
                    continue;
                }
                let label = if complex.source_name.trim().is_empty() {
                    waveform.name.clone()
                } else {
                    complex.source_name.clone()
                };
                resolved.push(PolarQuantity {
                    name: waveform.name.clone(),
                    label,
                    waveform_index,
                    order: (0, 0, 0, waveform_index),
                    reference_impedance_ohm: None,
                    unit: waveform.unit.clone().unwrap_or_default(),
                });
            }
        }
    }
    resolved.sort_by(|left, right| {
        left.order
            .cmp(&right.order)
            .then_with(|| left.name.cmp(&right.name))
    });
    resolved
}

/// The quantity the reader has selected, or the first the result carries.
fn selected_index(quantities: &[PolarQuantity], sheet: &PolarSheetState) -> usize {
    sheet
        .quantity
        .as_deref()
        .and_then(|name| quantities.iter().position(|quantity| quantity.name == name))
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// the drawing model
// ---------------------------------------------------------------------------

/// One frame's worth of drawing evidence, holding the retained sample arrays
/// by handle so the analysis borrow ends before any control is edited.
pub struct PolarLocus {
    label: String,
    unit: String,
    reference_impedance_ohm: Option<f64>,
    x: rspice_results::waveform::SharedWaveformValues,
    real: rspice_results::waveform::SharedWaveformValues,
    imag: rspice_results::waveform::SharedWaveformValues,
}

impl PolarLocus {
    /// Retained swept-axis samples shared with the host cursor pair.
    pub fn frequencies(&self) -> &[f64] {
        &self.x
    }

    fn len(&self) -> usize {
        self.x.len()
    }

    fn magnitude(&self, index: usize) -> f64 {
        self.real[index].hypot(self.imag[index])
    }

    pub fn is_finite(&self, index: usize) -> bool {
        self.x[index].is_finite() && self.real[index].is_finite() && self.imag[index].is_finite()
    }

    /// The largest finite magnitude on the locus.
    fn peak_magnitude(&self) -> f64 {
        (0..self.len())
            .filter(|index| self.is_finite(*index))
            .map(|index| self.magnitude(index))
            .fold(0.0_f64, f64::max)
    }

    /// Sample nearest a frequency, by absolute distance on the swept axis.
    pub fn nearest_to_frequency(&self, frequency: f64) -> Option<usize> {
        (0..self.len())
            .filter(|index| self.is_finite(*index))
            .min_by(|left, right| {
                (self.x[*left] - frequency)
                    .abs()
                    .total_cmp(&(self.x[*right] - frequency).abs())
            })
    }
}

/// The radius ruling: what the outer ring stands for and how a magnitude maps
/// onto 0…1 of the drawn radius.
#[derive(Debug, Clone, Copy)]
struct RadiusRule {
    mode: PolarRadius,
    /// Magnitude at the outer ring (linear mode).
    ceiling: f64,
    /// Decibels at the outer ring (dB mode).
    ceiling_db: f64,
}

impl RadiusRule {
    fn resolve(mode: PolarRadius, peak: f64, normalize: bool) -> Self {
        let peak = if peak.is_finite() && peak > 0.0 {
            peak
        } else {
            1.0
        };
        match mode {
            PolarRadius::Magnitude => {
                let ceiling = if normalize {
                    peak
                } else {
                    // A 1-2-5 step above the peak, so the five rings land on
                    // numbers a reader can hold.
                    let decade = 10.0_f64.powf(peak.log10().floor());
                    let steps = [1.0, 2.0, 5.0, 10.0];
                    steps
                        .iter()
                        .map(|step| step * decade)
                        .find(|candidate| *candidate >= peak)
                        .unwrap_or(peak)
                };
                Self {
                    mode,
                    ceiling,
                    ceiling_db: 0.0,
                }
            }
            PolarRadius::Decibels => {
                let peak_db = 20.0 * peak.log10();
                let ceiling_db = if normalize {
                    0.0
                } else {
                    // `ceil` on a small negative quotient yields -0.0, which
                    // prints as "-0 dB" on the ruling.
                    let rounded = (peak_db / 10.0).ceil() * 10.0;
                    if rounded == 0.0 { 0.0 } else { rounded }
                };
                Self {
                    mode,
                    ceiling: peak,
                    ceiling_db,
                }
            }
        }
    }

    /// Fraction of the drawn radius a magnitude lands at, clamped to the ring
    /// set. `None` is a magnitude the ruling cannot place at all.
    fn fraction(self, magnitude: f64) -> Option<f64> {
        if !magnitude.is_finite() {
            return None;
        }
        match self.mode {
            PolarRadius::Magnitude => {
                (self.ceiling > 0.0).then(|| (magnitude / self.ceiling).clamp(0.0, 1.0))
            }
            PolarRadius::Decibels => {
                if magnitude <= 0.0 {
                    return Some(0.0);
                }
                let decibels = 20.0 * magnitude.log10();
                Some(((decibels - (self.ceiling_db - DECIBEL_SPAN)) / DECIBEL_SPAN).clamp(0.0, 1.0))
            }
        }
    }

    /// What the ring at `fraction` of the radius stands for.
    fn ring_label(self, fraction: f64) -> String {
        match self.mode {
            PolarRadius::Magnitude => fmt_si(self.ceiling * fraction, "", 2),
            PolarRadius::Decibels => {
                format!("{:.0}", self.ceiling_db - DECIBEL_SPAN * (1.0 - fraction))
            }
        }
    }

    /// The head-note sentence stating the ruling.
    fn head_note(self, unit: &str, normalized: bool) -> String {
        let normalized = if normalized {
            " \u{b7} normalized to the locus maximum"
        } else {
            ""
        };
        match self.mode {
            PolarRadius::Magnitude => format!(
                "radius linear \u{b7} outer {}{normalized}",
                fmt_si(self.ceiling, unit, 3)
            ),
            PolarRadius::Decibels => format!(
                "radius dB \u{b7} {:.0} \u{2026} {:.0} dB{normalized}",
                self.ceiling_db,
                self.ceiling_db - DECIBEL_SPAN
            ),
        }
    }

    /// Whether the unit circle is worth emphasizing: it must be inside the
    /// ring set and not so far inside that the emphasis is a dot.
    fn emphasizes_unit_circle(self) -> bool {
        self.mode == PolarRadius::Magnitude && self.ceiling >= 1.0 && self.ceiling <= 12.5
    }
}

// ---------------------------------------------------------------------------
// the sheet bar
// ---------------------------------------------------------------------------

/// The domain controls this sheet owns, drawn left-aligned in the sheet bar.
pub fn domain_bar(
    ui: &mut Ui,
    analysis: Option<&AnalysisResult>,
    sheet: &mut PolarSheetState,
) -> bool {
    let Some(analysis) = analysis else {
        return false;
    };
    let quantities = quantities(analysis);
    if quantities.is_empty() {
        return false;
    }
    let selected = selected_index(&quantities, sheet);
    let options = quantities
        .iter()
        .map(|quantity| quantity.label.clone())
        .collect::<Vec<_>>();
    let identity = quantities[selected].label.clone();
    let reference = quantities[selected].reference_impedance_ohm;

    if let Some(picked) = select(
        ui,
        "rspice.results.polar.quantity",
        "Polar quantity",
        &options[selected],
        &options,
        216.0,
    ) {
        sheet.quantity = quantities.get(picked).map(|q| q.name.clone());
    }

    let mut radius = sheet.radius.index();
    if segmented(
        ui,
        "rspice.results.polar.radius",
        &PolarRadius::OPTIONS,
        &mut radius,
        SegmentedWidth::Natural,
    ) {
        sheet.radius = PolarRadius::from_index(radius);
    }

    let decade_marks = sheet.decade_marks;
    let response = chip(ui, "DEC", decade_marks).on_hover_text(if decade_marks {
        "Decade frequency marks on the locus"
    } else {
        "Decade frequency marks off"
    });
    if response.clicked() {
        sheet.decade_marks = !decade_marks;
    }

    let normalize = sheet.normalize;
    let response = chip(ui, "NORM", normalize).on_hover_text(if normalize {
        "Radius normalized to this locus' own maximum"
    } else {
        "Radius ruled in retained units"
    });
    if response.clicked() {
        sheet.normalize = !normalize;
    }

    let t = Tokens::get(ui.ctx());
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(identity)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    // The ohm sign is absent from the bundled mono face, so the retained
    // reference impedance is stated in the sans pane head rather than here.
    if let Some(reference) = reference {
        ui.label(
            egui::RichText::new(format!("Z\u{2080} {reference} \u{3a9}"))
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    }
    true
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Cursor A and B as sample indices on the active locus.
#[derive(Debug, Clone, Copy, Default)]
pub struct CursorSamples {
    pub a: Option<usize>,
    pub b: Option<usize>,
}

/// Render the polar sheet.
pub fn show(
    ui: &mut Ui,
    locus: Option<&PolarLocus>,
    sheet: &PolarSheetState,
    samples: CursorSamples,
    policy: &rspice_app_types::quantity::QuantityPresentationPolicy,
) -> Option<CanvasResponse> {
    let Some(locus) = locus else {
        well_hint(ui, ABSENT_STATE);
        return None;
    };

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let rule = RadiusRule::resolve(sheet.radius, locus.peak_magnitude(), sheet.normalize);

    // The legend chips are painted in the mono face, which has no ohm sign,
    // so the retained reference impedance rides the sans subtitle instead.
    let legend = [
        LegendChip {
            name: &locus.label,
            color: c.traces[0],
            on: true,
        },
        LegendChip {
            name: "A \u{b7} B frequency markers",
            color: c.accent,
            on: true,
        },
    ];
    let mut subtitle = rule.head_note(&locus.unit, sheet.normalize);
    if let Some(reference) = locus.reference_impedance_ohm {
        subtitle.push_str(&format!(" \u{b7} Z\u{2080} {reference} \u{3a9}"));
    }
    strip::StripHeader::new("POLAR", &subtitle, &legend).show(ui);

    Some(paint_canvas(ui, &t, locus, rule, samples, sheet, policy))
}

/// What the reader is told when the run carries no complex response at all.
const ABSENT_STATE: &str = "No complex response in this run — run an AC, SP, or HB analysis";

/// Resolve a selected locus from the host-qualified analysis without copying its sample arrays.
pub fn resolve_locus(analysis: &AnalysisResult, sheet: &PolarSheetState) -> Option<PolarLocus> {
    let quantities = quantities(analysis);
    if quantities.is_empty() {
        return None;
    }
    let quantity = quantities
        .get(selected_index(&quantities, sheet))
        .or_else(|| quantities.first())?;
    let waveform = analysis.waveforms.get(quantity.waveform_index)?;
    let complex = waveform.complex.as_ref()?;
    Some(PolarLocus {
        label: quantity.label.clone(),
        unit: quantity.unit.clone(),
        reference_impedance_ohm: quantity.reference_impedance_ohm,
        x: Arc::clone(&waveform.x),
        real: Arc::clone(&complex.real),
        imag: Arc::clone(&complex.imag),
    })
}

/// What one frame of the canvas reported back.
pub struct CanvasResponse {
    pub response: egui::Response,
    /// Sample nearest the pointer, when it is over the canvas.
    pub hovered: Option<usize>,
}

fn paint_canvas(
    ui: &mut Ui,
    t: &Tokens,
    locus: &PolarLocus,
    rule: RadiusRule,
    samples: CursorSamples,
    sheet: &PolarSheetState,
    policy: &rspice_app_types::quantity::QuantityPresentationPolicy,
) -> CanvasResponse {
    let c = &t.color;
    let area = ui.available_rect_before_wrap();
    let (rect, response) = ui.allocate_exact_size(area.size(), Sense::click_and_drag());
    let gestures = "Polar locus. Click the locus to snap the nearer cursor; \
         left and right arrows nudge cursor A, with Shift for B and Ctrl for one sample; \
         Escape clears both.";
    let label = format!("{} \u{b7} {}", locus.label, gestures);
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Image, ui.is_enabled(), &label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::GraphicsDocument);
        node.set_label(label.clone());
    });
    paint_focus_ring(ui, &response, rect);
    if !ui.is_rect_visible(rect) {
        return CanvasResponse {
            response,
            hovered: None,
        };
    }

    // Room for the outermost spoke labels on all four sides.
    const SPOKE_LABEL_MARGIN: f32 = 26.0;
    let side = rect.width().min(rect.height()) - 2.0 * SPOKE_LABEL_MARGIN;
    if side <= 16.0 {
        return CanvasResponse {
            response,
            hovered: None,
        };
    }
    let centre = rect.center();
    let radius = side * 0.5;
    let painter = ui.painter().with_clip_rect(rect);
    let grid = egui::Stroke::new(1.0, c.canvas_grid);

    // Stack radial labels beside the 90° spoke. Along the horizontal spoke,
    // long SI labels overlapped one another in smaller panes. Thin the labels
    // if even their line heights cannot fit, while retaining every grid ring.
    let label_stride = ((tokens::FS_0 + 4.0) / (radius / 5.0)).ceil().max(1.0) as i32;
    for step in 1..=5 {
        let fraction = f64::from(step) / 5.0;
        let emphasized = rule.emphasizes_unit_circle()
            && (rule.ceiling * fraction - 1.0).abs() < rule.ceiling * 1.0e-9;
        painter.circle_stroke(
            centre,
            radius * fraction as f32,
            if emphasized {
                egui::Stroke::new(1.4, c.border_strong)
            } else {
                grid
            },
        );
        if (5 - step) % label_stride == 0 {
            painter.text(
                egui::pos2(centre.x + 5.0, centre.y - radius * fraction as f32 + 3.0),
                egui::Align2::LEFT_TOP,
                rule.ring_label(fraction),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_faint,
            );
        }
    }
    // The unit circle when it falls between rings rather than on one.
    if rule.emphasizes_unit_circle() {
        let fraction = 1.0 / rule.ceiling;
        painter.circle_stroke(
            centre,
            radius * fraction as f32,
            egui::Stroke::new(1.4, c.border_strong),
        );
    }

    // 30° spokes, labelled 0 … ±180.
    for step in 0..12 {
        let degrees = f64::from(step) * 30.0;
        let angle = degrees.to_radians();
        let (sin, cos) = angle.sin_cos();
        let tip = egui::pos2(
            centre.x + radius * cos as f32,
            centre.y - radius * sin as f32,
        );
        painter.line_segment([centre, tip], grid);
        let signed = if degrees > 180.0 {
            degrees - 360.0
        } else {
            degrees
        };
        let label_at = egui::pos2(
            centre.x + (radius + 12.0) * cos as f32,
            centre.y - (radius + 12.0) * sin as f32,
        );
        painter.text(
            label_at,
            egui::Align2::CENTER_CENTER,
            format!("{signed:.0}\u{b0}"),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }

    // The locus.
    let place = |index: usize| -> Option<egui::Pos2> {
        if !locus.is_finite(index) {
            return None;
        }
        let magnitude = locus.magnitude(index);
        let fraction = rule.fraction(magnitude)?;
        let (real, imag) = (locus.real[index], locus.imag[index]);
        let (unit_real, unit_imag) = if magnitude > 0.0 {
            (real / magnitude, imag / magnitude)
        } else {
            (0.0, 0.0)
        };
        Some(egui::pos2(
            centre.x + radius * (fraction * unit_real) as f32,
            centre.y - radius * (fraction * unit_imag) as f32,
        ))
    };

    let mut points = Vec::with_capacity(locus.len());
    for index in 0..locus.len() {
        match place(index) {
            Some(point) => points.push(point),
            None => {
                if points.len() >= 2 {
                    painter.add(egui::Shape::line(
                        std::mem::take(&mut points),
                        egui::Stroke::new(1.4, c.traces[0]),
                    ));
                } else {
                    points.clear();
                }
            }
        }
    }
    if points.len() >= 2 {
        painter.add(egui::Shape::line(
            points,
            egui::Stroke::new(1.4, c.traces[0]),
        ));
    }

    let first = (0..locus.len()).find_map(place);
    let last = (0..locus.len()).rev().find_map(place);
    if let Some(start) = first {
        painter.circle_filled(start, 3.5, c.traces[0]);
    }
    if let Some(stop) = last {
        painter.circle_stroke(stop, 4.0, egui::Stroke::new(1.5, c.traces[0]));
    }
    paint_direction_arrow(&painter, locus, &place, c.traces[0]);

    if sheet.decade_marks {
        paint_decade_marks(&painter, t, locus, &place, policy);
    }

    // Cursor flags.
    for (label, sample, color) in [("A", samples.a, c.traces[1]), ("B", samples.b, c.accent)] {
        let Some(point) = sample.and_then(&place) else {
            continue;
        };
        painter.circle_filled(point, 3.5, color);
        painter.circle_stroke(point, 6.0, egui::Stroke::new(1.2, color));
        painter.text(
            egui::pos2(
                point.x + 8.0,
                point.y + if label == "A" { -8.0 } else { 8.0 },
            ),
            if label == "A" {
                egui::Align2::LEFT_BOTTOM
            } else {
                egui::Align2::LEFT_TOP
            },
            label,
            theme::mono(tokens::FS_0, FontWeight::SemiBold),
            color,
        );
    }

    // Nearest sample under the pointer, and its card.
    let mut hovered = None;
    if let Some(pointer) = response.hover_pos()
        && rect.contains(pointer)
    {
        let mut best = 16.0_f32 * 16.0;
        for index in 0..locus.len() {
            let Some(point) = place(index) else { continue };
            let distance = point.distance_sq(pointer);
            if distance < best {
                best = distance;
                hovered = Some(index);
            }
        }
    }
    if let Some(index) = hovered
        && let Some(point) = place(index)
    {
        painter.circle_stroke(point, 5.0, egui::Stroke::new(1.5, c.accent));
        crate::presentation::point_card(
            ui,
            rect,
            point,
            &locus.label,
            c.traces[0],
            &sample_card_rows(locus, index, policy),
        );
    }

    CanvasResponse { response, hovered }
}

fn sample_card_rows(
    locus: &PolarLocus,
    index: usize,
    policy: &rspice_app_types::quantity::QuantityPresentationPolicy,
) -> Vec<crate::presentation::CardRow> {
    let magnitude = locus.magnitude(index);
    vec![
        ("f".to_owned(), policy.format_frequency(locus.x[index], 2)),
        ("|\u{b7}|".to_owned(), fmt_si(magnitude, &locus.unit, 3)),
        (
            // Not "∠": that sign is in none of the bundled faces.
            "phase".to_owned(),
            policy.format_angle(locus.imag[index].atan2(locus.real[index]), 1),
        ),
    ]
}

/// One arrowhead at the midpoint of the locus, pointing the way the sweep
/// travels. Without it a closed locus does not say which end is the start.
fn paint_direction_arrow(
    painter: &egui::Painter,
    locus: &PolarLocus,
    place: &impl Fn(usize) -> Option<egui::Pos2>,
    color: egui::Color32,
) {
    let middle = locus.len() / 2;
    let (Some(from), Some(to)) = (place(middle.saturating_sub(1)), place(middle)) else {
        return;
    };
    let direction = to - from;
    let length = direction.length();
    if length < 1.0e-3 {
        return;
    }
    let forward = direction / length;
    let side = egui::vec2(-forward.y, forward.x);
    painter.add(egui::Shape::convex_polygon(
        vec![
            to + forward * 5.0,
            to - forward * 3.0 + side * 3.5,
            to - forward * 3.0 - side * 3.5,
        ],
        color,
        egui::Stroke::NONE,
    ));
}

/// A dot and a label at every decade boundary the sweep crosses.
fn paint_decade_marks(
    painter: &egui::Painter,
    t: &Tokens,
    locus: &PolarLocus,
    place: &impl Fn(usize) -> Option<egui::Pos2>,
    policy: &rspice_app_types::quantity::QuantityPresentationPolicy,
) {
    let finite = (0..locus.len()).filter(|index| locus.is_finite(*index));
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
    for index in finite {
        low = low.min(locus.x[index]);
        high = high.max(locus.x[index]);
    }
    if !(low.is_finite() && high.is_finite() && low > 0.0 && high > low) {
        return;
    }
    let mut decade = low.log10().ceil() as i32;
    while (decade as f64) <= high.log10() {
        let frequency = 10.0_f64.powi(decade);
        decade += 1;
        let Some(index) = locus.nearest_to_frequency(frequency) else {
            continue;
        };
        // Only a decade the sweep actually reached, not the nearest sample to
        // one it never got to.
        if (locus.x[index] - frequency).abs() > frequency * 0.05 {
            continue;
        }
        let Some(point) = place(index) else { continue };
        painter.circle_filled(point, 2.5, t.color.text_dim);
        painter.text(
            egui::pos2(point.x + 5.0, point.y + 2.0),
            egui::Align2::LEFT_TOP,
            policy.format_frequency(frequency, 0),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// One register row: what it measures, then A, B and Δ.
type RegisterRow = (&'static str, String, String, String);

/// The A │ B │ Δ register.
pub fn right_panel(
    ui: &mut Ui,
    locus: Option<&PolarLocus>,
    samples: CursorSamples,
    policy: &rspice_app_types::quantity::QuantityPresentationPolicy,
) {
    section_header(ui, "Cursors", None);
    let Some(locus) = locus else {
        crate::presentation::panel_note(
            ui,
            "Cursor values appear once a complex response is loaded.",
        );
        return;
    };
    let rows = register_rows(locus, samples, policy);
    register_table(ui, &rows);
    crate::presentation::panel_note(
        ui,
        "Phase is the principal value; the Δ column unwraps it before differencing. \
         Group delay is the A\u{2192}B secant −Δφ/2πΔf, not an instantaneous derivative.",
    );
}

fn register_rows(
    locus: &PolarLocus,
    samples: CursorSamples,
    policy: &rspice_app_types::quantity::QuantityPresentationPolicy,
) -> Vec<RegisterRow> {
    let dash = || "\u{2014}".to_owned();
    let at = |sample: Option<usize>, read: &dyn Fn(usize) -> String| sample.map_or_else(dash, read);
    let frequency = |index: usize| policy.format_frequency(locus.x[index], 2);
    let magnitude = |index: usize| fmt_si(locus.magnitude(index), &locus.unit, 2);
    let decibels = |index: usize| {
        let value = locus.magnitude(index);
        if value > 0.0 {
            format!("{:.2}", 20.0 * value.log10())
        } else {
            "\u{2212}\u{221e}".to_owned()
        }
    };
    let phase = |index: usize| policy.format_angle(locus.imag[index].atan2(locus.real[index]), 1);
    let real = |index: usize| fmt_si(locus.real[index], "", 2);
    let imaginary = |index: usize| fmt_si(locus.imag[index], "", 2);

    let pair = samples.a.zip(samples.b);
    let decades = pair.map_or_else(dash, |(a, b)| {
        let (from, to) = (locus.x[a], locus.x[b]);
        if from > 0.0 && to > 0.0 {
            format!("{:+.2} dec", (to / from).log10())
        } else {
            dash()
        }
    });
    let magnitude_delta = pair.map_or_else(dash, |(a, b)| {
        fmt_si(locus.magnitude(b) - locus.magnitude(a), &locus.unit, 2)
    });
    let decibel_delta = pair.map_or_else(dash, |(a, b)| {
        let (first, second) = (locus.magnitude(a), locus.magnitude(b));
        if first > 0.0 && second > 0.0 {
            format!("{:+.2}", 20.0 * (second / first).log10())
        } else {
            dash()
        }
    });
    let phase_delta = pair.map_or_else(dash, |(a, b)| {
        policy.format_angle(unwrapped_phase_delta(locus, a, b), 1)
    });
    let group_delay = pair.map_or_else(dash, |(a, b)| {
        group_delay_seconds(locus, a, b).map_or_else(dash, |value| fmt_si(value, "s", 2))
    });

    vec![
        (
            "Frequency",
            at(samples.a, &frequency),
            at(samples.b, &frequency),
            decades,
        ),
        (
            "|\u{b7}|",
            at(samples.a, &magnitude),
            at(samples.b, &magnitude),
            magnitude_delta,
        ),
        (
            "|\u{b7}| dB",
            at(samples.a, &decibels),
            at(samples.b, &decibels),
            decibel_delta,
        ),
        (
            "Phase",
            at(samples.a, &phase),
            at(samples.b, &phase),
            phase_delta,
        ),
        (
            "Re",
            at(samples.a, &real),
            at(samples.b, &real),
            pair.map_or_else(dash, |(a, b)| fmt_si(locus.real[b] - locus.real[a], "", 2)),
        ),
        (
            "Im",
            at(samples.a, &imaginary),
            at(samples.b, &imaginary),
            pair.map_or_else(dash, |(a, b)| fmt_si(locus.imag[b] - locus.imag[a], "", 2)),
        ),
        ("Group delay", dash(), dash(), group_delay),
    ]
}

/// Phase from A to B, unwrapped along the retained samples.
///
/// The principal value alone is a lie across a wrap: a locus that turned
/// −370° reads as −10°, and the group delay computed from it is off by a
/// factor the reader cannot see. The samples between the cursors are what
/// says how many turns happened, so they are what is walked.
fn unwrapped_phase_delta(locus: &PolarLocus, a: usize, b: usize) -> f64 {
    let (low, high, sign) = if a <= b { (a, b, 1.0) } else { (b, a, -1.0) };
    let mut total = 0.0;
    let mut previous = locus.imag[low].atan2(locus.real[low]);
    for index in (low + 1)..=high {
        if !locus.is_finite(index) {
            continue;
        }
        let current = locus.imag[index].atan2(locus.real[index]);
        let mut step = current - previous;
        while step > std::f64::consts::PI {
            step -= std::f64::consts::TAU;
        }
        while step < -std::f64::consts::PI {
            step += std::f64::consts::TAU;
        }
        total += step;
        previous = current;
    }
    sign * total
}

/// −Δφ / 2πΔf across the two cursors, in seconds.
fn group_delay_seconds(locus: &PolarLocus, a: usize, b: usize) -> Option<f64> {
    let span = locus.x[b] - locus.x[a];
    if !span.is_finite() || span == 0.0 {
        return None;
    }
    let delta = unwrapped_phase_delta(locus, a, b);
    let value = -delta / (std::f64::consts::TAU * span);
    value.is_finite().then_some(value)
}

/// The four-column register: measure, A, B, Δ.
fn register_table(ui: &mut Ui, rows: &[RegisterRow]) {
    let t = Tokens::get(ui.ctx());
    let c = &t.color;
    let width = ui.available_width();
    let inner = (width - 24.0).max(1.0);
    // Three exact values and their name in a 292 pt panel: the name column
    // gets what is left after the numbers, not the other way round.
    let name_width = inner * 0.26;
    let value_width = ((inner - name_width) / 3.0).max(1.0);

    let header = ["", "A", "B", "\u{394}"];
    let (header_rect, _) = ui.allocate_exact_size(egui::vec2(width, 20.0), Sense::hover());
    if ui.is_rect_visible(header_rect) {
        let painter = ui.painter();
        for (column, label) in header.iter().enumerate().skip(1) {
            let left = header_rect.left() + 12.0 + name_width + value_width * (column - 1) as f32;
            painter.text(
                egui::pos2(left + value_width - 2.0, header_rect.center().y),
                egui::Align2::RIGHT_CENTER,
                *label,
                // Sans: the Δ head is not in the bundled mono face.
                theme::sans(tokens::FS_0, FontWeight::SemiBold),
                c.text_faint,
            );
        }
        painter.hline(
            header_rect.x_range(),
            header_rect.bottom() - 0.5,
            egui::Stroke::new(1.0, c.border),
        );
    }

    for (index, (name, a, b, delta)) in rows.iter().enumerate() {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(width, 25.0), Sense::hover());
        response.widget_info(|| {
            WidgetInfo::labeled(
                WidgetType::Label,
                ui.is_enabled(),
                format!("{name}: A {a}, B {b}, delta {delta}"),
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Row);
        });
        if !ui.is_rect_visible(rect) {
            continue;
        }
        let painter = ui.painter();
        let name_rect = egui::Rect::from_min_max(
            egui::pos2(rect.left() + 12.0, rect.top()),
            egui::pos2(rect.left() + 12.0 + name_width, rect.bottom()),
        );
        painter.with_clip_rect(name_rect).text(
            egui::pos2(name_rect.left(), rect.center().y),
            egui::Align2::LEFT_CENTER,
            *name,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            c.text_dim,
        );
        for (column, value) in [a, b, delta].into_iter().enumerate() {
            let left = name_rect.right() + value_width * column as f32;
            let cell = egui::Rect::from_min_max(
                egui::pos2(left, rect.top()),
                egui::pos2(left + value_width - 2.0, rect.bottom()),
            );
            painter.with_clip_rect(cell).text(
                egui::pos2(cell.right(), rect.center().y),
                egui::Align2::RIGHT_CENTER,
                value,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text,
            );
        }
        if index + 1 < rows.len() {
            painter.hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A four-port S-parameter result carrying both the mixed-mode terms the
    /// engine retained and the physical ones.
    fn four_port_analysis() -> AnalysisResult {
        let frequency: Vec<f64> = (0..64)
            .map(|index| 1.0e6 * 10.0_f64.powf(index as f64 / 21.0))
            .collect();
        let mut waveforms = Vec::new();
        for (name, gain) in [
            ("Sdd21", 2.0),
            ("Sdd11", 0.3),
            ("Sdc21", 0.01),
            ("S11", 0.25),
            ("S21", 0.5),
        ] {
            let (real, imaginary): (Vec<f64>, Vec<f64>) = frequency
                .iter()
                .map(|f| {
                    let phase = -(f / 1.0e9) * std::f64::consts::PI;
                    let magnitude = gain / (1.0 + (f / 2.4e9).powi(2)).sqrt();
                    (magnitude * phase.cos(), magnitude * phase.sin())
                })
                .unzip();
            let magnitude = real
                .iter()
                .zip(imaginary.iter())
                .map(|(re, im)| re.hypot(*im))
                .collect::<Vec<_>>();
            waveforms.push(
                WaveformData::new(format!("|{name}|"), frequency.clone(), magnitude, "#00aaff")
                    .with_complex_components(name, real, imaginary),
            );
        }
        AnalysisResult::new(1, AnalysisType::SParameter, "SP", 0.0)
            .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
                noise_reference_temperature_kelvin: None,
                reference_impedances_ohm: vec![50.0, 50.0, 50.0, 50.0],
            })
            .with_waveforms(waveforms)
    }

    fn two_port_analysis() -> AnalysisResult {
        let mut analysis = four_port_analysis();
        analysis
            .waveforms
            .retain(|waveform| !waveform.name.contains("dd") && !waveform.name.contains("dc"));
        analysis.family_metadata = Some(AnalysisResultFamilyMetadata::SParameter {
            noise_reference_temperature_kelvin: None,
            reference_impedances_ohm: vec![50.0, 50.0],
        });
        analysis
    }

    /// The quantity list is built from what the result retained, never from a
    /// fixed menu: a two-port SP has no mixed-mode terms to offer and must not
    /// grow them, and a four-port lists its retained ones first.
    #[test]
    fn the_quantity_list_is_exactly_what_the_result_retained() {
        let four = quantities(&four_port_analysis());
        let labels = four.iter().map(|q| q.label.as_str()).collect::<Vec<_>>();
        assert_eq!(
            labels,
            [
                "Sdd11 \u{b7} D1",
                "Sdd21 \u{b7} D2 \u{2190} D1",
                "Sdc21 \u{b7} D2 \u{2190} C1",
                "S11 \u{b7} Port 1",
                "S21 \u{b7} Port 2 \u{2190} Port 1",
            ],
            "mixed-mode terms come first and physical terms keep their port names"
        );

        let two = quantities(&two_port_analysis());
        assert_eq!(
            two.iter().map(|q| q.label.as_str()).collect::<Vec<_>>(),
            ["S11 \u{b7} Port 1", "S21 \u{b7} Port 2 \u{2190} Port 1"],
            "a two-port run must not be offered mixed-mode quantities"
        );
    }

    /// Only a physical reflection term has a retained reference impedance, so
    /// only it may print one.
    #[test]
    fn only_a_physical_reflection_term_carries_a_reference_impedance() {
        let resolved = quantities(&four_port_analysis());
        let by_label = |needle: &str| {
            resolved
                .iter()
                .find(|q| q.label.starts_with(needle))
                .expect("the fixture retains this term")
        };
        assert_eq!(by_label("S11").reference_impedance_ohm, Some(50.0));
        assert_eq!(by_label("S21").reference_impedance_ohm, None);
        assert_eq!(by_label("Sdd11").reference_impedance_ohm, None);
    }

    /// An AC result offers its complex responses by their source names, and
    /// nothing about ports is asserted for them.
    #[test]
    fn an_ac_result_offers_its_complex_responses() {
        let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            WaveformData::new("|V(out)|", vec![1.0, 2.0], vec![1.0, 0.5], "#00aaff")
                .with_complex_components("V(out)", vec![1.0, 0.5], vec![0.0, -0.25]),
            WaveformData::new("phase(V(out))", vec![1.0, 2.0], vec![0.0, -26.0], "#f80"),
        ]);
        let resolved = quantities(&analysis);
        assert_eq!(
            resolved.len(),
            1,
            "the real-valued phase trace is not a locus"
        );
        assert_eq!(resolved[0].label, "V(out)");
        assert_eq!(resolved[0].reference_impedance_ohm, None);
    }

    /// A transient result carries no complex response at all.
    #[test]
    fn a_run_without_a_complex_response_offers_no_quantity() {
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0e-9], vec![0.0, 1.0], "#00aaff"),
            ]);
        assert!(quantities(&analysis).is_empty());
        assert_eq!(
            ABSENT_STATE,
            "No complex response in this run — run an AC, SP, or HB analysis"
        );
    }

    /// The dB ruling is five 10 dB rings ending at a 10 dB step above the
    /// peak, and a magnitude below the floor clamps onto the centre rather
    /// than escaping the canvas as a negative radius.
    #[test]
    fn the_decibel_ruling_is_a_fifty_decibel_window_that_clamps_at_both_ends() {
        let rule = RadiusRule::resolve(PolarRadius::Decibels, 0.5, false);
        assert!((rule.ceiling_db - 0.0).abs() < 1.0e-12, "{rule:?}");
        assert_eq!(rule.fraction(1.0), Some(1.0));
        assert_eq!(rule.fraction(0.0), Some(0.0));
        assert_eq!(rule.fraction(1.0e-9), Some(0.0));
        let half = rule.fraction(10.0_f64.powf(-25.0 / 20.0)).expect("finite");
        assert!((half - 0.5).abs() < 1.0e-9, "{half}");
        assert_eq!(
            rule.head_note("", false),
            "radius dB \u{b7} 0 \u{2026} -50 dB"
        );
    }

    /// The linear ruling lands the outer ring on a 1-2-5 number, and
    /// normalizing puts the locus maximum exactly on it.
    #[test]
    fn the_linear_ruling_rounds_out_to_a_readable_ring() {
        let rule = RadiusRule::resolve(PolarRadius::Magnitude, 1.7, false);
        assert!((rule.ceiling - 2.0).abs() < 1.0e-12, "{rule:?}");
        assert!(rule.emphasizes_unit_circle());

        let normalized = RadiusRule::resolve(PolarRadius::Magnitude, 1.7, true);
        assert!((normalized.ceiling - 1.7).abs() < 1.0e-12);
        assert_eq!(normalized.fraction(1.7), Some(1.0));

        // A hundred-fold transfer keeps its ruling but stops claiming a unit
        // circle nobody could see.
        let large = RadiusRule::resolve(PolarRadius::Magnitude, 100.08, false);
        assert!((large.ceiling - 200.0).abs() < 1.0e-12, "{large:?}");
        assert!(!large.emphasizes_unit_circle());
    }

    fn ramp_locus(turns: f64, points: usize) -> PolarLocus {
        let x: Vec<f64> = (0..points)
            .map(|index| 1.0e9 + index as f64 * 1.0e6)
            .collect();
        let (real, imag): (Vec<f64>, Vec<f64>) = (0..points)
            .map(|index| {
                let phase = -std::f64::consts::TAU * turns * index as f64 / (points - 1) as f64;
                (phase.cos(), phase.sin())
            })
            .unzip();
        PolarLocus {
            label: "S21 \u{b7} Port 2 \u{2190} Port 1".to_owned(),
            unit: String::new(),
            reference_impedance_ohm: None,
            x: Arc::new(x),
            real: Arc::new(real),
            imag: Arc::new(imag),
        }
    }

    /// A locus that turned more than half a revolution between the cursors
    /// has a phase difference the principal value cannot state, and the group
    /// delay computed from the principal value is wrong by whole turns.
    #[test]
    fn the_phase_delta_unwraps_before_it_is_differenced() {
        let locus = ramp_locus(3.0, 601);
        let last = locus.len() - 1;
        let delta = unwrapped_phase_delta(&locus, 0, last);
        assert!(
            (delta + 3.0 * std::f64::consts::TAU).abs() < 1.0e-6,
            "three turns of lag read as {delta} rad"
        );

        // −Δφ / 2πΔf over 600 MHz of sweep.
        let delay = group_delay_seconds(&locus, 0, last).expect("finite delay");
        let expected = 3.0 / 600.0e6;
        assert!((delay - expected).abs() < 1.0e-15, "{delay} vs {expected}");

        // Reading the cursors the other way round flips the sign of both.
        assert!(
            (unwrapped_phase_delta(&locus, last, 0) - 3.0 * std::f64::consts::TAU).abs() < 1e-6
        );
    }

    /// Cursor placement is by frequency, so a re-run that changes the sample
    /// count still lands the cursor on the nearest retained sample.
    #[test]
    fn a_cursor_resolves_to_the_nearest_retained_sample() {
        let locus = ramp_locus(1.0, 11);
        assert_eq!(locus.nearest_to_frequency(1.0e9), Some(0));
        assert_eq!(locus.nearest_to_frequency(1.0044e9), Some(4));
        assert_eq!(locus.nearest_to_frequency(9.9e9), Some(10));
    }

    /// Every register row states A, B and Δ, and says so with a dash rather
    /// than a zero when a cursor is not placed.
    #[test]
    fn the_register_states_a_dash_for_a_cursor_that_is_not_placed() {
        let locus = ramp_locus(0.25, 101);
        let policy = rspice_app_types::quantity::QuantityPresentationPolicy::default();
        let rows = register_rows(
            &locus,
            CursorSamples {
                a: Some(0),
                b: None,
            },
            &policy,
        );
        assert_eq!(
            rows.iter().map(|row| row.0).collect::<Vec<_>>(),
            [
                "Frequency",
                "|\u{b7}|",
                "|\u{b7}| dB",
                "Phase",
                "Re",
                "Im",
                "Group delay",
            ]
        );
        assert!(rows.iter().all(|row| row.2 == "\u{2014}"));
        assert!(rows.iter().all(|row| row.3 == "\u{2014}"));
        assert_eq!(rows[2].1, "0.00", "a unit-magnitude sample is 0 dB");
    }

    /// The Δ frequency column is stated in decades, which is what a swept
    /// logarithmic response is read in.
    #[test]
    fn the_frequency_delta_is_stated_in_decades() {
        let locus = ramp_locus(0.5, 3);
        let policy = rspice_app_types::quantity::QuantityPresentationPolicy::default();
        let rows = register_rows(
            &locus,
            CursorSamples {
                a: Some(0),
                b: Some(2),
            },
            &policy,
        );
        let expected = format!("{:+.2} dec", (locus.x[2] / locus.x[0]).log10());
        assert_eq!(rows[0].3, expected);
    }
}
