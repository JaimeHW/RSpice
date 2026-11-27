//! POLAR — retained complex responses on the polar plane: concentric
//! radius rings, 30° spokes, the swept loci with their direction of travel,
//! and the two shared frequency cursors read as A │ B │ Δ on the primary
//! response. Up to four more responses overlay it, each in its own palette
//! ink, with the cursors marked on them at the same frequencies.
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

use egui::{Sense, Ui, WidgetInfo, WidgetType};

use rspice_results::renderability::{
    PolarQuantitySource, polar_quantity_source, polar_reference_impedances,
};
use rspice_results::waveform::RetainedTrace;
type AnalysisResult = rspice_results::analysis_result::AnalysisResult<RetainedTrace>;
use crate::presentation::{angle_readout, frequency_readout};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::{fmt_column_reading, fmt_si_significant, fmt_signed_reading};
use rspice_ui_kit::theme::{self, FontWeight, paint_focus_ring};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{SegmentedWidth, chip, segmented, select};

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
    /// Responses drawn over the primary one, by retained waveform name.
    pub overlays: Vec<String>,
    pub radius: PolarRadius,
    pub decade_marks: bool,
    pub normalize: bool,
}

/// The most responses that overlay the primary one; each has its own ink.
pub const MAX_OVERLAYS: usize = 4;

/// The palette inks the overlays draw in, in order: the canvas's trace hues
/// after the primary's. Each canvas theme's hues stand apart from one another
/// and from both cursor inks, so none needs skipping.
fn overlay_inks(palette: &rspice_ui_kit::palette::Palette) -> &[egui::Color32] {
    &palette.traces[1..=MAX_OVERLAYS]
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
    let network = polar_reference_impedances(analysis);
    // Without port impedances only an AC or HB response is a polar quantity,
    // so no other analysis's columns are walked for one: the tab strip asks
    // on every frame.
    if network.is_none()
        && !matches!(
            analysis.analysis_type,
            rspice_results::analysis_type::AnalysisType::Ac
                | rspice_results::analysis_type::AnalysisType::HarmonicBalance
        )
    {
        return Vec::new();
    }
    let mut resolved = Vec::new();
    for (waveform_index, waveform) in analysis.waveforms.iter().enumerate() {
        let Some(source) = polar_quantity_source(analysis.analysis_type, network, waveform) else {
            continue;
        };
        let complex = waveform
            .complex
            .as_ref()
            .expect("plottability checked the complex components");
        match source {
            PolarQuantitySource::NetworkTerm {
                identity,
                reference_impedances_ohm,
            } => {
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
            PolarQuantitySource::Response => {
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
    /// Retained waveform name — the selection key.
    name: String,
    label: String,
    unit: String,
    reference_impedance_ohm: Option<f64>,
    /// The analysis kind the strip header names, `AC`, `SP`.
    kind_tag: String,
    x: rspice_results::waveform::HeldColumn,
    real: rspice_results::waveform::HeldColumn,
    imag: rspice_results::waveform::HeldColumn,
}

impl PolarLocus {
    /// The retained waveform the locus draws.
    pub fn name(&self) -> &str {
        &self.name
    }

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
    /// The radius is ruled against the locus maximum rather than in
    /// retained units.
    normalized: bool,
    /// Magnitude at the outer ring (linear mode).
    ceiling: f64,
    /// Decibels at the outer ring (dB mode).
    ceiling_db: f64,
    /// Rings drawn out to the outer one, evenly spaced.
    rings: u32,
}

/// The dB ruling's rings: one every 10 dB across the window.
const DECIBEL_RINGS: u32 = 5;

/// The ruling the sheet draws `locus` and its overlays on. Every locus fits;
/// normalized, each is scaled to its own maximum and the primary's sets the
/// ring. A passive port's reflection coefficient rules to the unit circle.
fn ruling(locus: &PolarLocus, overlays: &[PolarLocus], sheet: &PolarSheetState) -> RadiusRule {
    let peak = if sheet.normalize {
        locus.peak_magnitude()
    } else {
        overlays
            .iter()
            .map(PolarLocus::peak_magnitude)
            .fold(locus.peak_magnitude(), f64::max)
    };
    let rule = RadiusRule::resolve(sheet.radius, peak, sheet.normalize);
    if locus.reference_impedance_ohm.is_some() {
        rule.within_unit_circle(peak)
    } else {
        rule
    }
}

/// The radius the sheet rules `analysis` to, as the Inspector states it:
/// `0 … 1`, `−50 … 0 dB`, and "normalized" when each locus is scaled to its
/// own maximum. A polar chart has no X or Y interval of its own.
pub fn radius_summary(analysis: &AnalysisResult, sheet: &PolarSheetState) -> Option<String> {
    let locus = resolve_locus(analysis, sheet)?;
    let overlays = resolve_overlays(analysis, sheet);
    Some(ruling(&locus, &overlays, sheet).summary(&locus.unit))
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
                let (ceiling, rings) = if normalize {
                    (peak, 5)
                } else {
                    rspice_results::plot_axis::polar_rings(peak)
                };
                Self {
                    mode,
                    normalized: normalize,
                    ceiling,
                    ceiling_db: 0.0,
                    rings,
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
                    normalized: normalize,
                    ceiling: peak,
                    ceiling_db,
                    rings: DECIBEL_RINGS,
                }
            }
        }
    }

    /// A reflection coefficient no larger than one rules to the unit circle:
    /// the whole disc a passive port can reach, so the reader sees how far
    /// inside it the locus stays rather than a ring fitted to this sweep.
    fn within_unit_circle(self, peak: f64) -> Self {
        if self.mode == PolarRadius::Magnitude
            && !self.normalized
            && peak.is_finite()
            && peak <= 1.0
        {
            Self {
                ceiling: 1.0,
                rings: 5,
                ..self
            }
        } else {
            self
        }
    }

    /// The radius interval the ruling spans, as the Inspector states it.
    fn summary(self, unit: &str) -> String {
        let span = match self.mode {
            PolarRadius::Magnitude => {
                format!("0 \u{2026} {}", fmt_si_significant(self.ceiling, unit, 3))
            }
            PolarRadius::Decibels => format!(
                "{} \u{2026} {}",
                fmt_si_significant((self.ceiling_db - DECIBEL_SPAN).round(), "", 6),
                fmt_si_significant(self.ceiling_db.round(), "dB", 6)
            ),
        };
        if self.normalized {
            format!("{span} \u{b7} normalized")
        } else {
            span
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
                // Normalized, the locus maximum is 0 dB at the outer ring.
                let magnitude = if self.normalized {
                    magnitude / self.ceiling
                } else {
                    magnitude
                };
                let decibels = 20.0 * magnitude.log10();
                Some(((decibels - (self.ceiling_db - DECIBEL_SPAN)) / DECIBEL_SPAN).clamp(0.0, 1.0))
            }
        }
    }

    /// What the ring at `fraction` of the radius stands for.
    fn ring_label(self, fraction: f64) -> String {
        match self.mode {
            // The head note states the unit once, so a ring is a plain
            // number with no prefix of its own: 0.4, never 400 m beside 1.
            PolarRadius::Magnitude => fmt_si_significant(self.ceiling * fraction, "", 3),
            PolarRadius::Decibels => fmt_si_significant(
                (self.ceiling_db - DECIBEL_SPAN * (1.0 - fraction)).round(),
                "",
                6,
            ),
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
                fmt_si_significant(self.ceiling, unit, 3)
            ),
            PolarRadius::Decibels => format!(
                "radius dB \u{b7} {} \u{2026} {}{normalized}",
                fmt_si_significant(self.ceiling_db.round(), "", 6),
                fmt_si_significant((self.ceiling_db - DECIBEL_SPAN).round(), "dB", 6)
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
        let primary = &quantities[picked].name;
        sheet.overlays.retain(|name| name != primary);
    }
    if sheet.overlays.len() < MAX_OVERLAYS {
        let addable = quantities
            .iter()
            .enumerate()
            .filter(|(index, quantity)| {
                *index != selected && !sheet.overlays.contains(&quantity.name)
            })
            .map(|(_, quantity)| quantity)
            .collect::<Vec<_>>();
        if !addable.is_empty() {
            let mut overlay_options = vec!["+ overlay".to_owned()];
            overlay_options.extend(addable.iter().map(|quantity| quantity.label.clone()));
            if let Some(picked) = select(
                ui,
                "rspice.results.polar.overlay",
                "Overlay another response",
                &overlay_options[0],
                &overlay_options,
                112.0,
            ) && picked > 0
            {
                sheet.overlays.push(addable[picked - 1].name.clone());
            }
        }
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
    let response = chip(ui, "Decades", decade_marks).on_hover_text(if decade_marks {
        "Decade frequency marks on the locus"
    } else {
        "Decade frequency marks off"
    });
    if response.clicked() {
        sheet.decade_marks = !decade_marks;
    }

    let normalize = sheet.normalize;
    let response = chip(ui, "Normalize", normalize).on_hover_text(if normalize {
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

/// Render the polar sheet: the primary locus, which carries the cursors,
/// and its overlays.
pub fn show(
    ui: &mut Ui,
    locus: Option<&PolarLocus>,
    overlays: &[PolarLocus],
    sheet: &PolarSheetState,
    samples: CursorSamples,
    policy: &rspice_vocabulary::quantity::QuantityPresentationPolicy,
    significant_digits: usize,
) -> Option<CanvasResponse> {
    let Some(locus) = locus else {
        well_hint(ui, ABSENT_STATE);
        return None;
    };

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let rule = ruling(locus, overlays, sheet);

    // The legend chips are painted in the mono face, which has no ohm sign,
    // so the retained reference impedance rides the sans subtitle instead.
    // The curves first, the primary then its overlays, and the markers drawn
    // on them after every curve they mark.
    let mut legend = vec![LegendChip {
        name: &locus.label,
        color: c.traces[0],
        on: true,
        glyph: strip::LegendGlyph::Line,
    }];
    let inks = overlay_inks(&c);
    let overlays = &overlays[..overlays.len().min(inks.len())];
    for (overlay, ink) in overlays.iter().zip(inks) {
        legend.push(LegendChip {
            name: &overlay.label,
            color: *ink,
            on: true,
            glyph: strip::LegendGlyph::Line,
        });
    }
    legend.push(LegendChip {
        name: "A \u{b7} B frequency markers",
        color: c.canvas_cursor_a,
        on: true,
        glyph: strip::LegendGlyph::Dot,
    });
    let mut subtitle = rule.head_note(&locus.unit, sheet.normalize);
    if let Some(reference) = locus.reference_impedance_ohm {
        subtitle.push_str(&format!(" \u{b7} Z\u{2080} {reference} \u{3a9}"));
    }
    let header = strip::StripHeader::new(&locus.kind_tag, &subtitle, &legend)
        .removable(1..1 + overlays.len())
        .show(ui);

    let loci = Loci {
        primary: locus,
        overlays,
    };
    let mut response = paint_canvas(
        ui,
        &t,
        loci,
        rule,
        samples,
        sheet,
        Readings {
            policy,
            significant_digits,
        },
    );
    response.removed_overlay = header
        .legend_removed
        .and_then(|chip| chip.checked_sub(1))
        .filter(|overlay| *overlay < overlays.len());
    Some(response)
}

/// Resolve the overlay loci the sheet lists, skipping any the analysis no
/// longer retains and the primary itself.
pub fn resolve_overlays(analysis: &AnalysisResult, sheet: &PolarSheetState) -> Vec<PolarLocus> {
    let quantities = quantities(analysis);
    let primary = quantities
        .get(selected_index(&quantities, sheet))
        .map(|quantity| quantity.name.clone());
    sheet
        .overlays
        .iter()
        .filter(|name| Some(*name) != primary.as_ref())
        .filter_map(|name| {
            let quantity = quantities.iter().find(|quantity| quantity.name == *name)?;
            locus_of(analysis, quantity)
        })
        .take(MAX_OVERLAYS)
        .collect()
}

/// What the reader is told when the run carries no complex response at all.
const ABSENT_STATE: &str = "No complex response in this run — run an AC, SP, or HB analysis";

/// A network opens on its transmission terms, as ADS opens S21 and S12 on a
/// polar plot and leaves S11 and S22 to the Smith chart: until the reader
/// chooses a locus or an overlay, the first transmission term is the locus
/// and the others overlay it.
pub fn seed_network_defaults(analysis: &AnalysisResult, sheet: &mut PolarSheetState) {
    if sheet.quantity.is_some() || !sheet.overlays.is_empty() {
        return;
    }
    let transmissions = quantities(analysis)
        .into_iter()
        .filter(|quantity| {
            rspice_results::network_matrix::trace_identity(&quantity.name)
                .is_some_and(|identity| identity.output_port != identity.input_port)
        })
        .map(|quantity| quantity.name)
        .collect::<Vec<_>>();
    let Some((first, rest)) = transmissions.split_first() else {
        return;
    };
    sheet.quantity = Some(first.clone());
    sheet.overlays = rest.iter().take(MAX_OVERLAYS).cloned().collect();
}

/// Resolve a selected locus from the host-qualified analysis without copying its sample arrays.
pub fn resolve_locus(analysis: &AnalysisResult, sheet: &PolarSheetState) -> Option<PolarLocus> {
    let quantities = quantities(analysis);
    if quantities.is_empty() {
        return None;
    }
    let quantity = quantities
        .get(selected_index(&quantities, sheet))
        .or_else(|| quantities.first())?;
    locus_of(analysis, quantity)
}

/// The locus of `quantity`, once its samples have arrived: a stored sweep is read off the frame,
/// and is not drawn until it is.
fn locus_of(analysis: &AnalysisResult, quantity: &PolarQuantity) -> Option<PolarLocus> {
    let waveform = analysis.waveforms.get(quantity.waveform_index)?;
    let complex = waveform.complex.as_ref()?;
    Some(PolarLocus {
        name: quantity.name.clone(),
        label: quantity.label.clone(),
        unit: quantity.unit.clone(),
        reference_impedance_ohm: quantity.reference_impedance_ohm,
        kind_tag: analysis.analysis_type.short_label().to_uppercase(),
        x: waveform.x.fetch_whole().ready()?,
        real: complex.real.fetch_whole().ready()?,
        imag: complex.imag.fetch_whole().ready()?,
    })
}

/// What one frame of the canvas reported back.
pub struct CanvasResponse {
    pub response: egui::Response,
    /// Primary-locus sample nearest the pointer, when it is the nearest
    /// sample of any locus over the canvas.
    pub hovered: Option<usize>,
    /// An overlay whose legend chip's "Remove" was chosen.
    pub removed_overlay: Option<usize>,
}

/// The primary locus and the responses overlaid on it.
#[derive(Clone, Copy)]
struct Loci<'a> {
    primary: &'a PolarLocus,
    overlays: &'a [PolarLocus],
}

/// How the canvas states a value: the reader's units and digits.
#[derive(Clone, Copy)]
struct Readings<'a> {
    policy: &'a rspice_vocabulary::quantity::QuantityPresentationPolicy,
    significant_digits: usize,
}

fn paint_canvas(
    ui: &mut Ui,
    t: &Tokens,
    loci: Loci<'_>,
    rule: RadiusRule,
    samples: CursorSamples,
    sheet: &PolarSheetState,
    readings: Readings<'_>,
) -> CanvasResponse {
    let Loci {
        primary: locus,
        overlays,
    } = loci;
    let Readings {
        policy,
        significant_digits,
    } = readings;
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
            removed_overlay: None,
        };
    }

    // Room for the outermost spoke labels on all four sides.
    const SPOKE_LABEL_MARGIN: f32 = 26.0;
    let side = rect.width().min(rect.height()) - 2.0 * SPOKE_LABEL_MARGIN;
    if side <= 16.0 {
        return CanvasResponse {
            response,
            hovered: None,
            removed_overlay: None,
        };
    }
    let centre = rect.center();
    let radius = side * 0.5;
    let painter = ui.painter().with_clip_rect(rect);
    let grid = egui::Stroke::new(1.0, c.canvas_grid);

    // Stack radial labels beside the 90° spoke. Along the horizontal spoke,
    // long SI labels overlapped one another in smaller panes. Thin the labels
    // if even their line heights cannot fit, while retaining every grid ring.
    let rings = rule.rings.max(1);
    let label_stride = ((tokens::FS_0 + 4.0) / (radius / rings as f32))
        .ceil()
        .max(1.0) as u32;
    for step in 1..=rings {
        let fraction = f64::from(step) / f64::from(rings);
        let emphasized = rule.emphasizes_unit_circle()
            && (rule.ceiling * fraction - 1.0).abs() < rule.ceiling * 1.0e-9;
        painter.circle_stroke(
            centre,
            radius * fraction as f32,
            if emphasized {
                egui::Stroke::new(1.4, c.canvas_frame)
            } else {
                grid
            },
        );
        if (rings - step).is_multiple_of(label_stride) {
            painter.text(
                egui::pos2(centre.x + 5.0, centre.y - radius * fraction as f32 + 3.0),
                egui::Align2::LEFT_TOP,
                rule.ring_label(fraction),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.canvas_text_faint,
            );
        }
    }
    // The unit circle when it falls between rings rather than on one.
    if rule.emphasizes_unit_circle() {
        let fraction = 1.0 / rule.ceiling;
        painter.circle_stroke(
            centre,
            radius * fraction as f32,
            egui::Stroke::new(1.4, c.canvas_frame),
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
            format!("{}\u{b0}", fmt_si_significant(signed.round(), "", 6)),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.canvas_text_faint,
        );
    }

    // The overlays under the primary locus, each in its own ink. Normalized,
    // each is scaled so its own maximum lands where the primary's does.
    let primary_peak = locus.peak_magnitude();
    let overlay_places = overlays
        .iter()
        .map(|overlay| {
            let scale = if sheet.normalize && overlay.peak_magnitude() > 0.0 {
                primary_peak / overlay.peak_magnitude()
            } else {
                1.0
            };
            placer(overlay, rule, centre, radius, scale)
        })
        .collect::<Vec<_>>();
    let inks = overlay_inks(c);
    for ((overlay, place), ink) in overlays.iter().zip(&overlay_places).zip(inks) {
        paint_locus(&painter, overlay, place, *ink);
    }
    let place = placer(locus, rule, centre, radius, 1.0);
    paint_locus(&painter, locus, &place, c.traces[0]);

    if sheet.decade_marks {
        paint_decade_marks(&painter, t, locus, &place, centre, policy);
    }

    // Cursor flags, in the colours the shared cursor pair has on every plot.
    // An overlay marks the same frequencies with a ring and no flag.
    for (sample, color) in [
        (samples.a, c.canvas_cursor_a),
        (samples.b, c.canvas_cursor_b),
    ] {
        let Some(frequency) = sample.and_then(|index| locus.x.get(index).copied()) else {
            continue;
        };
        for (overlay, place) in overlays.iter().zip(&overlay_places) {
            if let Some(point) = overlay.nearest_to_frequency(frequency).and_then(place) {
                painter.circle_stroke(point, 4.0, egui::Stroke::new(1.2, color));
            }
        }
    }
    for (label, sample, color) in [
        ("A", samples.a, c.canvas_cursor_a),
        ("B", samples.b, c.canvas_cursor_b),
    ] {
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

    // Nearest sample of any locus under the pointer, and its card. Only a
    // primary sample is a cursor target.
    let mut nearest: Option<(Option<usize>, usize, egui::Pos2)> = None;
    if let Some(pointer) = response.hover_pos()
        && rect.contains(pointer)
    {
        let mut best = 16.0_f32 * 16.0;
        let candidates =
            std::iter::once((None, locus, &place as &dyn Fn(usize) -> Option<egui::Pos2>)).chain(
                overlays.iter().zip(&overlay_places).enumerate().map(
                    |(overlay, (locus, place))| {
                        (
                            Some(overlay),
                            locus,
                            place as &dyn Fn(usize) -> Option<egui::Pos2>,
                        )
                    },
                ),
            );
        for (which, candidate, place) in candidates {
            for index in 0..candidate.len() {
                let Some(point) = place(index) else { continue };
                let distance = point.distance_sq(pointer);
                if distance < best {
                    best = distance;
                    nearest = Some((which, index, point));
                }
            }
        }
    }
    let mut hovered = None;
    if let Some((which, index, point)) = nearest {
        let (hit, color) = match which {
            None => {
                hovered = Some(index);
                (locus, c.traces[0])
            }
            Some(overlay) => (&overlays[overlay], inks[overlay]),
        };
        painter.circle_stroke(point, 5.0, egui::Stroke::new(1.5, c.canvas_cursor_a));
        crate::presentation::point_card(
            ui,
            rect,
            point,
            &hit.label,
            color,
            &sample_card_rows(hit, index, policy, significant_digits),
        );
    }

    CanvasResponse {
        response,
        hovered,
        removed_overlay: None,
    }
}

/// Where each sample of `locus` lands on the canvas, its magnitude scaled by
/// `scale` before the radius ruling reads it.
fn placer<'a>(
    locus: &'a PolarLocus,
    rule: RadiusRule,
    centre: egui::Pos2,
    radius: f32,
    scale: f64,
) -> impl Fn(usize) -> Option<egui::Pos2> + 'a {
    move |index: usize| {
        if !locus.is_finite(index) {
            return None;
        }
        let magnitude = locus.magnitude(index);
        let fraction = rule.fraction(magnitude * scale)?;
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
    }
}

/// One locus: its line, broken across samples the ruling cannot place, a
/// filled start, a hollow stop and the arrow of travel.
fn paint_locus(
    painter: &egui::Painter,
    locus: &PolarLocus,
    place: &impl Fn(usize) -> Option<egui::Pos2>,
    color: egui::Color32,
) {
    let mut points = Vec::with_capacity(locus.len());
    for index in 0..locus.len() {
        match place(index) {
            Some(point) => points.push(point),
            None => {
                if points.len() >= 2 {
                    painter.add(egui::Shape::line(
                        std::mem::take(&mut points),
                        egui::Stroke::new(1.4, color),
                    ));
                } else {
                    points.clear();
                }
            }
        }
    }
    if points.len() >= 2 {
        painter.add(egui::Shape::line(points, egui::Stroke::new(1.4, color)));
    }
    let first = (0..locus.len()).find_map(place);
    let last = (0..locus.len()).rev().find_map(place);
    if let Some(start) = first {
        painter.circle_filled(start, 3.5, color);
    }
    if let Some(stop) = last {
        painter.circle_stroke(stop, 4.0, egui::Stroke::new(1.5, color));
    }
    paint_direction_arrow(painter, locus, place, color);
}

fn sample_card_rows(
    locus: &PolarLocus,
    index: usize,
    policy: &rspice_vocabulary::quantity::QuantityPresentationPolicy,
    significant_digits: usize,
) -> Vec<crate::presentation::CardRow> {
    let magnitude = locus.magnitude(index);
    vec![
        (
            "f".to_owned(),
            frequency_readout(policy, locus.x[index], significant_digits),
        ),
        (
            "|\u{b7}|".to_owned(),
            fmt_column_reading(magnitude, &locus.unit, significant_digits),
        ),
        (
            // Not "∠": that sign is in none of the bundled faces.
            "phase".to_owned(),
            angle_readout(
                policy,
                locus.imag[index].atan2(locus.real[index]),
                significant_digits,
            ),
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
    centre: egui::Pos2,
    policy: &rspice_vocabulary::quantity::QuantityPresentationPolicy,
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
        painter.circle_filled(point, 2.5, t.color.canvas_text_dim);
        // The label sits on the far side of the mark from the chart's
        // centre, off the locus the mark is on, and never cuts a curve. A
        // mark at the centre has no far side; its label sits right and below.
        let offset = point - centre;
        let outward = if offset.length() > 12.0 {
            offset.normalized()
        } else {
            egui::vec2(0.92, 0.39)
        };
        let side = |component: f32| {
            if component > 0.38 {
                egui::Align::Min
            } else if component < -0.38 {
                egui::Align::Max
            } else {
                egui::Align::Center
            }
        };
        painter.text(
            point + outward * 7.0,
            egui::Align2([side(outward.x), side(outward.y)]),
            frequency_readout(policy, frequency, 3),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.canvas_text_dim,
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
    policy: &rspice_vocabulary::quantity::QuantityPresentationPolicy,
    significant_digits: usize,
) {
    section_header(ui, "Cursors", None);
    let Some(locus) = locus else {
        crate::presentation::panel_note(
            ui,
            "Cursor values appear once a complex response is loaded.",
        );
        return;
    };
    let rows = register_rows(locus, samples, policy, significant_digits);
    register_table(ui, &rows);
}

/// What a register row's own convention is, where it has one. A sentence
/// about two of the rows belongs on those rows, not under the table.
fn row_hover(name: &str) -> Option<&'static str> {
    match name {
        "Phase" => Some("Principal value; the Δ column unwraps it before differencing."),
        "Group delay" => Some(
            "At A and B, −dφ/2πdf from the neighbouring samples; Δ is the A→B secant −Δφ/2πΔf.",
        ),
        _ => None,
    }
}

fn register_rows(
    locus: &PolarLocus,
    samples: CursorSamples,
    policy: &rspice_vocabulary::quantity::QuantityPresentationPolicy,
    digits: usize,
) -> Vec<RegisterRow> {
    let dash = || "\u{2014}".to_owned();
    let at = |sample: Option<usize>, read: &dyn Fn(usize) -> String| sample.map_or_else(dash, read);
    let frequency = |index: usize| frequency_readout(policy, locus.x[index], digits);
    let magnitude = |index: usize| fmt_column_reading(locus.magnitude(index), &locus.unit, digits);
    let decibels = |index: usize| {
        let value = locus.magnitude(index);
        if value > 0.0 {
            crate::presentation::fixed_places(
                20.0 * value.log10(),
                crate::presentation::DECIBEL_PLACES,
            )
        } else {
            "\u{2212}\u{221e}".to_owned()
        }
    };
    let phase =
        |index: usize| angle_readout(policy, locus.imag[index].atan2(locus.real[index]), digits);
    let delay = |index: usize| {
        group_delay_at(locus, index)
            .map_or_else(dash, |value| fmt_column_reading(value, "s", digits))
    };
    let real = |index: usize| fmt_column_reading(locus.real[index], "", digits);
    let imaginary = |index: usize| fmt_column_reading(locus.imag[index], "", digits);

    let pair = samples.a.zip(samples.b);
    let decades = pair.map_or_else(dash, |(a, b)| {
        let (from, to) = (locus.x[a], locus.x[b]);
        if from > 0.0 && to > 0.0 {
            fmt_signed_reading((to / from).log10(), "dec", digits)
        } else {
            dash()
        }
    });
    let magnitude_delta = pair.map_or_else(dash, |(a, b)| {
        fmt_column_reading(locus.magnitude(b) - locus.magnitude(a), &locus.unit, digits)
    });
    let decibel_delta = pair.map_or_else(dash, |(a, b)| {
        let (first, second) = (locus.magnitude(a), locus.magnitude(b));
        if first > 0.0 && second > 0.0 {
            rspice_ui_kit::plot::format::signed_fixed_places(
                20.0 * (second / first).log10(),
                crate::presentation::DECIBEL_PLACES,
            )
        } else {
            dash()
        }
    });
    let phase_delta = pair.map_or_else(dash, |(a, b)| {
        angle_readout(policy, unwrapped_phase_delta(locus, a, b), digits)
    });
    let group_delay = pair.map_or_else(dash, |(a, b)| {
        group_delay_seconds(locus, a, b)
            .map_or_else(dash, |value| fmt_column_reading(value, "s", digits))
    });

    vec![
        (
            "Frequency",
            at(samples.a, &frequency),
            at(samples.b, &frequency),
            decades,
        ),
        (
            "Magnitude",
            at(samples.a, &magnitude),
            at(samples.b, &magnitude),
            magnitude_delta,
        ),
        (
            "Magnitude dB",
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
            pair.map_or_else(dash, |(a, b)| {
                fmt_column_reading(locus.real[b] - locus.real[a], "", digits)
            }),
        ),
        (
            "Im",
            at(samples.a, &imaginary),
            at(samples.b, &imaginary),
            pair.map_or_else(dash, |(a, b)| {
                fmt_column_reading(locus.imag[b] - locus.imag[a], "", digits)
            }),
        ),
        (
            "Group delay",
            at(samples.a, &delay),
            at(samples.b, &delay),
            group_delay,
        ),
    ]
}

/// −dφ/2πdf at one sample, from its finite neighbours: the central
/// difference inside the sweep, one-sided at either end.
fn group_delay_at(locus: &PolarLocus, index: usize) -> Option<f64> {
    let before = (0..index)
        .rev()
        .find(|sample| locus.is_finite(*sample))
        .unwrap_or(index);
    let after = (index + 1..locus.len())
        .find(|sample| locus.is_finite(*sample))
        .unwrap_or(index);
    group_delay_seconds(locus, before, after)
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

/// The register's inset from the panel edges.
const REGISTER_EDGE: f32 = 12.0;
/// The air between two register columns.
const REGISTER_GAP: f32 = 10.0;

/// The four-column register: measure, A, B, Δ. Each value column is as wide
/// as its widest reading, a gap from the next and right-aligned; the name
/// column takes what is left and wraps a name that will not fit on a line.
fn register_table(ui: &mut Ui, rows: &[RegisterRow]) {
    let t = Tokens::get(ui.ctx());
    let c = &t.color;
    let width = ui.available_width();
    let inner = (width - 2.0 * REGISTER_EDGE).max(1.0);
    let value_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    // Sans: the Δ head is not in the bundled mono face.
    let head_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let name_font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let header = ["A", "B", "\u{394}"];

    let (name_width, columns) = {
        let painter = ui.painter();
        let measure = |text: &str, font: &egui::FontId| {
            painter
                .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::PLACEHOLDER)
                .size()
                .x
        };
        let mut columns = header.map(|label| measure(label, &head_font));
        for (_, a, b, delta) in rows {
            for (column, value) in [a, b, delta].into_iter().enumerate() {
                columns[column] = columns[column].max(measure(value, &value_font));
            }
        }
        // A name keeps at least its longest word; past that the readings
        // give way, each by the same share, and clip.
        let longest_word = rows
            .iter()
            .flat_map(|row| row.0.split(' '))
            .map(|word| measure(word, &name_font))
            .fold(0.0_f32, f32::max);
        let readings: f32 = columns.iter().sum();
        let name_width = (inner - readings - REGISTER_GAP * 3.0).max(longest_word);
        let room = (inner - name_width - REGISTER_GAP * 3.0).max(0.0);
        if readings > room && readings > 0.0 {
            for column in &mut columns {
                *column *= room / readings;
            }
        }
        (name_width, columns)
    };
    // Each value column's right edge, from the register's left inset.
    let mut rights = [0.0_f32; 3];
    let mut edge = REGISTER_EDGE + name_width;
    for (right, column) in rights.iter_mut().zip(columns) {
        edge += REGISTER_GAP + column;
        *right = edge;
    }

    let (header_rect, _) = ui.allocate_exact_size(egui::vec2(width, 20.0), Sense::hover());
    if ui.is_rect_visible(header_rect) {
        let painter = ui.painter();
        for (right, label) in rights.iter().zip(header) {
            painter.text(
                egui::pos2(header_rect.left() + right, header_rect.center().y),
                egui::Align2::RIGHT_CENTER,
                label,
                head_font.clone(),
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
        let name_galley = ui.painter().layout(
            (*name).to_owned(),
            name_font.clone(),
            c.text_dim,
            name_width,
        );
        let height = (name_galley.size().y + 8.0).max(25.0);
        let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), Sense::hover());
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
        if let Some(hover) = row_hover(name) {
            response.on_hover_text(hover);
        }
        if !ui.is_rect_visible(rect) {
            continue;
        }
        let painter = ui.painter();
        painter.galley(
            egui::pos2(
                rect.left() + REGISTER_EDGE,
                rect.center().y - name_galley.size().y * 0.5,
            ),
            name_galley,
            c.text_dim,
        );
        for ((right, column), value) in rights.iter().zip(columns).zip([a, b, delta]) {
            let cell = egui::Rect::from_min_max(
                egui::pos2(rect.left() + right - column, rect.top()),
                egui::pos2(rect.left() + right, rect.bottom()),
            );
            painter.with_clip_rect(cell).text(
                egui::pos2(cell.right(), rect.center().y),
                egui::Align2::RIGHT_CENTER,
                value,
                value_font.clone(),
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
    use rspice_results::{
        analysis_type::AnalysisType, family_metadata::AnalysisResultFamilyMetadata,
    };

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
                RetainedTrace::new(format!("|{name}|"), frequency.clone(), magnitude, "#00aaff")
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
            RetainedTrace::new("|V(out)|", vec![1.0, 2.0], vec![1.0, 0.5], "#00aaff")
                .with_complex_components("V(out)", vec![1.0, 0.5], vec![0.0, -0.25]),
            RetainedTrace::new("phase(V(out))", vec![1.0, 2.0], vec![0.0, -26.0], "#f80"),
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
                RetainedTrace::new("V(out)", vec![0.0, 1.0e-9], vec![0.0, 1.0], "#00aaff"),
            ]);
        assert!(quantities(&analysis).is_empty());
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
            "radius dB \u{b7} 0 \u{2026} \u{2212}50 dB"
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
        assert!((large.ceiling - 120.0).abs() < 1.0e-9, "{large:?}");
        assert_eq!(large.rings, 6);
        assert!(!large.emphasizes_unit_circle());
    }

    /// A locus peaking at six is ruled in six rings of one, not out to ten,
    /// and a passive port's reflection coefficient is ruled to the unit
    /// circle however far inside it this sweep stays.
    #[test]
    fn the_outer_ring_fits_the_locus_and_a_reflection_rules_to_the_unit_circle() {
        let fitted = RadiusRule::resolve(PolarRadius::Magnitude, 6.0, false);
        assert!((fitted.ceiling - 6.0).abs() < 1.0e-9, "{fitted:?}");
        assert_eq!(fitted.rings, 6);
        assert_eq!(fitted.summary("V"), "0 \u{2026} 6 V");

        let reflection = RadiusRule::resolve(PolarRadius::Magnitude, 0.62, false);
        let reflection = reflection.within_unit_circle(0.62);
        assert!((reflection.ceiling - 1.0).abs() < 1.0e-12, "{reflection:?}");
        assert_eq!(reflection.rings, 5);
        assert!(reflection.emphasizes_unit_circle());

        // An active port that reflects more than it receives is ruled to
        // its own locus, with the unit circle still marked inside it.
        let active =
            RadiusRule::resolve(PolarRadius::Magnitude, 1.3, false).within_unit_circle(1.3);
        assert!((active.ceiling - 1.5).abs() < 1.0e-9, "{active:?}");
        assert!(active.emphasizes_unit_circle());
    }

    /// Group delay at each cursor is the local phase slope, so a constant
    /// delay reads the same at A, at B and across them.
    #[test]
    fn group_delay_is_read_at_each_cursor_from_the_local_phase_slope() {
        let locus = ramp_locus(3.0, 601);
        let expected = 3.0 / 600.0e6;
        for index in [0, 300, 600] {
            let delay = group_delay_at(&locus, index).expect("finite delay");
            assert!((delay - expected).abs() < 1.0e-15, "{index}: {delay}");
        }
        let policy = rspice_vocabulary::quantity::QuantityPresentationPolicy::default();
        let rows = register_rows(
            &locus,
            CursorSamples {
                a: Some(100),
                b: Some(500),
            },
            &policy,
            4,
        );
        let group = rows.iter().find(|row| row.0 == "Group delay").expect("row");
        assert_eq!(group.1, fmt_column_reading(expected, "s", 4));
        assert_eq!(group.2, group.1);
        assert_eq!(group.3, group.1);
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
            name: "S21".to_owned(),
            label: "S21 \u{b7} Port 2 \u{2190} Port 1".to_owned(),
            unit: String::new(),
            reference_impedance_ohm: None,
            kind_tag: "SP".to_owned(),
            x: x.into(),
            real: real.into(),
            imag: imag.into(),
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
        let policy = rspice_vocabulary::quantity::QuantityPresentationPolicy::default();
        let rows = register_rows(
            &locus,
            CursorSamples {
                a: Some(0),
                b: None,
            },
            &policy,
            4,
        );
        assert_eq!(
            rows.iter().map(|row| row.0).collect::<Vec<_>>(),
            [
                "Frequency",
                "Magnitude",
                "Magnitude dB",
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
        let policy = rspice_vocabulary::quantity::QuantityPresentationPolicy::default();
        let rows = register_rows(
            &locus,
            CursorSamples {
                a: Some(0),
                b: Some(2),
            },
            &policy,
            4,
        );
        let expected = fmt_signed_reading((locus.x[2] / locus.x[0]).log10(), "dec", 4);
        assert_eq!(rows[0].3, expected);
    }

    /// An overlay is held by name: the primary is never its own overlay, a
    /// response the run no longer retains is skipped, and four at most draw.
    #[test]
    fn overlays_resolve_by_name_and_skip_what_the_run_lacks() {
        let analysis = four_port_analysis();
        let names = quantities(&analysis)
            .into_iter()
            .map(|quantity| quantity.name)
            .collect::<Vec<_>>();
        let mut sheet = PolarSheetState {
            quantity: Some(names[0].clone()),
            ..PolarSheetState::default()
        };
        sheet.overlays = vec![
            names[0].clone(),
            "S99".to_owned(),
            names[1].clone(),
            names[2].clone(),
        ];
        let resolved = resolve_overlays(&analysis, &sheet)
            .iter()
            .map(|locus| locus.name().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(resolved, vec![names[1].clone(), names[2].clone()]);
        sheet.overlays = names[1..].to_vec();
        assert_eq!(resolve_overlays(&analysis, &sheet).len(), MAX_OVERLAYS);
    }

    /// Normalized in dB, each locus' maximum sits at 0 dB on the outer ring.
    #[test]
    fn a_normalized_decibel_ruling_puts_the_maximum_on_the_rim() {
        let rule = RadiusRule::resolve(PolarRadius::Decibels, 2.0, true);
        assert_eq!(rule.fraction(2.0), Some(1.0));
        let half = rule.fraction(2.0 * 10f64.powf(-25.0 / 20.0)).unwrap();
        assert!((half - 0.5).abs() < 1e-12, "{half}");
    }
}
