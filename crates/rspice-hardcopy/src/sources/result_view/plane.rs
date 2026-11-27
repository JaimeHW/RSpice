//! The complex-plane pages: Smith, polar and Nyquist.
//!
//! A plane is read by its distances and angles, so both axes share one
//! scale. The window the sheet opens on is widened about its centre along
//! the axis with room to spare until it fills the plot area, as the sheet's
//! `equal_scale_ranges` fills its rect, rather than letterboxing a square.

use super::*;
use rspice_results::number_format::{fmt_column_reading, fmt_si_significant};
use rspice_results::nyquist::{
    LOOP_GAIN_LOCUS_LABEL, NyquistData, NyquistMargin, loop_gain_traces,
};
use rspice_results::smith::{ResponseReading, SmithGrid};

/// The plot area every plane fills, in micrometres.
const AREA: (i64, i64) = (
    PLOT_WIDTH_UM - 2 * PLOT_INSET_UM,
    PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM,
);

/// The normalised values a Smith chart rules and labels, as the sheet does.
const SMITH_GRID: [f64; 5] = [0.2, 0.5, 1.0, 2.0, 5.0];

/// The half-width of the window the Smith sheet opens on.
const SMITH_REACH: f64 = 1.12;

/// What the Nyquist sheet calls a loop gain's negative-frequency branch.
const MIRROR_LABEL: &str = "L(\u{2212}j\u{3c9})";

/// Digits a printed margin is read to, as the sheet's readouts default.
const READOUT_DIGITS: usize = 4;

/// `x` and `y`, the one with room to spare widened about its centre, so the
/// window fills the plot area on one scale.
fn equal_scale(x: (f64, f64), y: (f64, f64)) -> ((f64, f64), (f64, f64)) {
    let (width, height) = (AREA.0 as f64, AREA.1 as f64);
    let per = ((x.1 - x.0) / width).max((y.1 - y.0) / height);
    if !(per.is_finite() && per > 0.0) {
        return (x, y);
    }
    let about = |(low, high): (f64, f64), length: f64| {
        let centre = (low + high) * 0.5;
        let half = per * length * 0.5;
        (centre - half, centre + half)
    };
    (about(x, width), about(y, height))
}

/// One plane's window, mapped onto the plot area on one scale.
struct Plane {
    x: (f64, f64),
    y: (f64, f64),
}

impl Plane {
    fn new(x: (f64, f64), y: (f64, f64)) -> Self {
        let (x, y) = equal_scale(x, y);
        Self { x, y }
    }

    fn frame(&self) -> PlotFrame {
        PlotFrame {
            x_minimum: self.x.0,
            x_maximum: self.x.1,
            y_minimum: self.y.0,
            y_maximum: self.y.1,
            x_span: self.x.1 - self.x.0,
            y_span: self.y.1 - self.y.0,
            plot_width: AREA.0,
            plot_height: AREA.1,
        }
    }

    fn paths(&self, points: &[(f64, f64)]) -> Result<Vec<Vec<SemanticPoint>>, HardcopySourceError> {
        clipped_plot_paths(
            points, self.x.0, self.x.1, self.y.0, self.y.1, AREA.0, AREA.1,
        )
    }

    /// Where `(re, im)` falls on the page, if it is inside the window.
    fn point(&self, (re, im): (f64, f64)) -> Option<SemanticPoint> {
        ((self.x.0..=self.x.1).contains(&re) && (self.y.0..=self.y.1).contains(&im))
            .then(|| {
                map_plot_point(
                    (re, im),
                    self.x.0,
                    self.y.0,
                    self.x.1 - self.x.0,
                    self.y.1 - self.y.0,
                    AREA.0,
                    AREA.1,
                )
                .ok()
            })
            .flatten()
    }

    fn guide(
        &self,
        points: &[(f64, f64)],
        major: bool,
    ) -> Result<SemanticPlotGuide, HardcopySourceError> {
        Ok(SemanticPlotGuide {
            paths: self.paths(points)?,
            major,
            second: false,
        })
    }

    /// A small ring round `at`, marking a point the plane is read at.
    fn spot(&self, at: (f64, f64)) -> Option<SemanticPlotGuide> {
        let centre = self.point(at)?;
        let ring = (0..=16)
            .map(|step| {
                let angle = std::f64::consts::TAU * f64::from(step) / 16.0;
                SemanticPoint::new(
                    centre.x_um + (700.0 * angle.cos()).round() as i64,
                    centre.y_um + (700.0 * angle.sin()).round() as i64,
                )
            })
            .collect();
        Some(SemanticPlotGuide {
            paths: vec![ring],
            major: true,
            second: false,
        })
    }

    /// A caption beside the page point of `at`, offset by `(dx, dy)`
    /// micrometres; none for a point outside the window.
    fn label(
        &self,
        at: (f64, f64),
        (dx, dy): (i64, i64),
        text: String,
        anchor: SemanticCaptionAnchor,
    ) -> Option<SemanticPlotCaption> {
        let point = self.point(at)?;
        Some(SemanticPlotCaption {
            text,
            position: SemanticPoint::new(point.x_um + dx, point.y_um + dy),
            anchor,
        })
    }
}

/// The points of the circle about `centre` of `radius`, closed.
fn circle(centre: (f64, f64), radius: f64, steps: u32) -> Vec<(f64, f64)> {
    (0..=steps)
        .map(|step| {
            let angle = std::f64::consts::TAU * f64::from(step) / f64::from(steps);
            (
                centre.0 + radius * angle.cos(),
                centre.1 + radius * angle.sin(),
            )
        })
        .collect()
}

/// The part of the circle about `centre` of `radius` inside the unit disc.
fn arc_in_unit_disc(centre: (f64, f64), radius: f64) -> Vec<(f64, f64)> {
    let distance = centre.0.hypot(centre.1);
    if distance + radius <= 1.0 + 1.0e-9 {
        return circle(centre, radius, 256);
    }
    if distance >= 1.0 + radius || radius >= distance + 1.0 {
        return Vec::new();
    }
    let toward_origin = (-centre.1).atan2(-centre.0);
    let half = ((distance * distance + radius * radius - 1.0) / (2.0 * radius * distance))
        .clamp(-1.0, 1.0)
        .acos();
    let steps = ((half / std::f64::consts::PI) * 256.0).ceil().max(12.0) as u32;
    (0..=steps)
        .map(|step| {
            let angle = toward_origin - half + 2.0 * half * f64::from(step) / f64::from(steps);
            (
                centre.0 + radius * angle.cos(),
                centre.1 + radius * angle.sin(),
            )
        })
        .collect()
}

/// A grid value as the Smith sheet labels it: `1`, not `1.0`.
fn grid_value(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

/// One complex locus a plane draws, and what it is called.
struct Locus {
    identity: String,
    label: String,
    points: Vec<(f64, f64)>,
    companion: bool,
    /// Its place in the sheet's trace colours, where the sheet counts it
    /// among loci it does not draw.
    palette_slot: Option<u16>,
}

/// The plane's page: its loci over its rulings.
fn plane_plot(
    viewer: ResultViewer,
    plane: &Plane,
    loci: &[Locus],
    axis_ticks: Vec<SemanticAxisTick>,
    captions: Vec<SemanticPlotCaption>,
    guides: Vec<SemanticPlotGuide>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let traces = loci
        .iter()
        .enumerate()
        .map(|(index, locus)| {
            Ok(SemanticPlotTrace {
                trace_id: stable_quick_trace_id(viewer, index, &locus.identity),
                label: locus.label.clone(),
                paths: plane.paths(&locus.points)?,
                source_samples: locus
                    .points
                    .iter()
                    .map(|(re, im)| (re.to_bits(), im.to_bits()))
                    .collect(),
                companion: locus.companion,
                palette_slot: locus.palette_slot,
                key: None,
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    Ok(SemanticPlot {
        viewer,
        page_id: stable_page_id("Results"),
        pane_id: 0,
        x_scale: AxisScale::Linear,
        y_scale: AxisScale::Linear,
        axis_ticks,
        traces,
        cursors: Vec::new(),
        markers: Vec::new(),
        annotations: Vec::new(),
        captions,
        frames: Vec::new(),
        guides,
    })
}

/// A plane's ticks, the captions that name its axes and the lines that
/// rule it.
struct PlaneAxes {
    ticks: Vec<SemanticAxisTick>,
    captions: Vec<SemanticPlotCaption>,
    guides: Vec<SemanticPlotGuide>,
}

/// The plane's own axes: Re along the bottom, Im up the left, ruled on the
/// one scale both share, and the two lines through the origin.
fn plane_axes(plane: &Plane) -> Result<PlaneAxes, HardcopySourceError> {
    let (ticks, mut captions) = plot_axes(AxisScale::Linear, AxisScale::Linear, &plane.frame())?;
    // A word and a signed value a millimetre apart read as one ("Re−3"),
    // so the caption stands as far off the frame as a value's sign is wide
    // again.
    captions.push(SemanticPlotCaption {
        text: "Re".to_owned(),
        position: SemanticPoint::new(
            PLOT_INSET_UM - 2_500,
            PLOT_HEIGHT_UM - PLOT_INSET_UM + 2_600,
        ),
        anchor: SemanticCaptionAnchor::End,
    });
    captions.push(SemanticPlotCaption {
        text: "Im".to_owned(),
        position: SemanticPoint::new(PLOT_INSET_UM - 1_000, PLOT_INSET_UM - 1_200),
        anchor: SemanticCaptionAnchor::End,
    });
    let guides = vec![
        plane.guide(&[(plane.x.0, 0.0), (plane.x.1, 0.0)], false)?,
        plane.guide(&[(0.0, plane.y.0), (0.0, plane.y.1)], false)?,
    ];
    Ok(PlaneAxes {
        ticks,
        captions,
        guides,
    })
}

/// The points of one complex series, each column asked of the reader
/// service whole, as a Nyquist locus is: a stored sweep is read on the
/// service's thread under its one budget, never a block at a time here.
fn complex_points(
    name: &str,
    real: &WaveformValues,
    imaginary: &WaveformValues,
) -> Result<Vec<(f64, f64)>, HardcopySourceError> {
    let (real, imaginary) = (whole(name, real)?, whole(name, imaginary)?);
    Ok(real
        .iter()
        .copied()
        .zip(imaginary.iter().copied())
        .collect())
}

/// The complex loci a polar or Smith sheet draws from one analysis: each
/// shown trace the `renderability` rule names a quantity of the polar
/// sheet, a network term or a complex response.
fn polar_loci<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<Vec<Locus>, HardcopySourceError> {
    let network = network_impedances(active.analysis);
    active
        .analysis
        .waveforms
        .iter()
        .filter(|waveform| (active.is_visible)(waveform))
        .map(AsRef::as_ref)
        .filter(|waveform| is_polar_quantity(active.analysis.analysis_type, network, waveform))
        .filter_map(|waveform| {
            let complex = waveform.complex.as_ref()?;
            let points = complex_points(&waveform.name, &complex.real, &complex.imag);
            Some(points.map(|points| Locus {
                identity: format!(
                    "{}:{}:{}:{}:complex",
                    active.dataset_id, active.run_id, active.analysis.id, waveform.name
                ),
                label: waveform.name.clone(),
                points,
                companion: false,
                palette_slot: None,
            }))
        })
        .collect()
}

/// The per-port reference impedances of a network analysis, which make its
/// complex traces network terms: `renderability::polar_reference_impedances`
/// over any retained waveform.
fn network_impedances<W>(analysis: &AnalysisResult<W>) -> Option<&[f64]> {
    match (&analysis.analysis_type, analysis.family_metadata.as_ref()) {
        (
            AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp,
            Some(AnalysisResultFamilyMetadata::SParameter {
                reference_impedances_ohm,
                ..
            }),
        ) => Some(reference_impedances_ohm),
        _ => None,
    }
}

/// Whether one retained waveform is a quantity the polar sheet draws: a
/// network term at a port pair the run declares, or a complex AC or
/// harmonic-balance response. `renderability::polar_quantity_source`, over
/// any retained waveform.
fn is_polar_quantity(
    analysis_type: AnalysisType,
    network: Option<&[f64]>,
    waveform: &RetainedWaveform,
) -> bool {
    let Some(complex) = waveform.complex.as_ref() else {
        return false;
    };
    if complex.real.is_empty()
        || complex.real.len() != complex.imag.len()
        || complex.real.len() != waveform.x.len()
    {
        return false;
    }
    match network {
        Some(impedances) => {
            let Some(identity) =
                rspice_results::network_matrix::trace_identity(&complex.source_name)
                    .or_else(|| rspice_results::network_matrix::trace_identity(&waveform.name))
            else {
                return false;
            };
            let ports = if identity.physical_ports {
                impedances.len()
            } else if impedances.len().is_multiple_of(2) {
                impedances.len() / 2
            } else {
                return false;
            };
            identity.output_port <= ports && identity.input_port <= ports
        }
        None => matches!(
            analysis_type,
            AnalysisType::Ac | AnalysisType::HarmonicBalance
        ),
    }
}

/// The loci a Smith sheet draws, and the reference impedances they are read
/// against.
struct SmithLoci {
    loci: Vec<Locus>,
    references: Vec<f64>,
    /// Whether the loci are responses read through the sheet's reading,
    /// rather than a network's own coefficients.
    responses: bool,
}

/// What the Smith sheet draws from one analysis. A network draws its terms
/// as the chart opens them, its reflection terms on and its transmission
/// terms off, each switched as the reader switched it; each keeps the colour
/// of its place among all the terms the sheet lists. Any other analysis
/// draws the responses the reader listed, in order, each read through the
/// sheet's Γ, Z or Y reading against its Z₀.
fn smith_loci<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
    sheet: &QuickSmithSettings,
) -> Result<SmithLoci, HardcopySourceError> {
    let analysis = active.analysis;
    let mut loci = Vec::new();
    let mut references = Vec::new();
    if let Some(impedances) = network_impedances(analysis) {
        let mut waveforms = analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&RetainedWaveform>>();
        waveforms.sort_by(|left, right| left.name.cmp(&right.name));
        let mut listed = 0_u16;
        for waveform in waveforms {
            let Some(identity) = rspice_results::network_matrix::trace_identity(&waveform.name)
            else {
                continue;
            };
            let ports = if identity.physical_ports {
                impedances.len()
            } else {
                impedances.len() / 2
            };
            let Some(complex) = waveform.complex.as_ref() else {
                continue;
            };
            if identity.output_port > ports
                || identity.input_port > ports
                || complex.real.is_empty()
                || complex.real.len() != complex.imag.len()
                || complex.real.len() != waveform.x.len()
            {
                continue;
            }
            // The chart names a coefficient, S11, not the magnitude column
            // |S11| it is retained beside.
            let name =
                if rspice_results::network_matrix::trace_identity(&complex.source_name).is_some() {
                    &complex.source_name
                } else {
                    &waveform.name
                };
            let slot = listed;
            listed = listed.saturating_add(1);
            let reflection = identity.output_port == identity.input_port;
            if reflection == sheet.toggled.iter().any(|toggled| toggled == name) {
                continue;
            }
            if reflection && identity.physical_ports {
                references.extend(
                    identity
                        .output_port
                        .checked_sub(1)
                        .and_then(|port| impedances.get(port)),
                );
            }
            loci.push(Locus {
                identity: format!(
                    "{}:{}:{}:{}:complex",
                    active.dataset_id, active.run_id, analysis.id, waveform.name
                ),
                label: name.clone(),
                points: complex_points(name, &complex.real, &complex.imag)?,
                companion: false,
                palette_slot: Some(slot),
            });
        }
        return Ok(SmithLoci {
            loci,
            references,
            responses: false,
        });
    }
    for text in &sheet.responses {
        let retained = analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .find(|waveform| {
                waveform.name == *text && is_polar_quantity(analysis.analysis_type, None, waveform)
            });
        let parts = match retained.and_then(|waveform| waveform.complex.as_ref()) {
            Some(complex) => complex_points(text, &complex.real, &complex.imag)?,
            None => {
                match rspice_results::calculator::retained::complex_locus(&analysis.waveforms, text)
                {
                    Ok((_, real, imaginary)) => real.into_iter().zip(imaginary).collect(),
                    Err(_) => continue,
                }
            }
        };
        // A response at the chart's singular point has no reflection, and
        // the locus breaks there.
        let points = parts
            .into_iter()
            .map(|sample| {
                let gamma = sheet.reading.reflection_of(sample, sheet.reference_ohm);
                if gamma.0.is_finite() && gamma.1.is_finite() {
                    gamma
                } else {
                    (0.0, f64::NAN)
                }
            })
            .collect();
        references.push(sheet.reference_ohm);
        loci.push(Locus {
            identity: format!(
                "{}:{}:{}:{text}:response",
                active.dataset_id, active.run_id, analysis.id
            ),
            label: text.clone(),
            points,
            companion: false,
            palette_slot: None,
        });
    }
    Ok(SmithLoci {
        loci,
        references,
        responses: true,
    })
}

/// What the sheet's header says the chart is read against, and, for
/// responses, how they were read onto it.
fn smith_subtitle(drawn: &SmithLoci, reading: ResponseReading) -> String {
    let mut references = drawn.references.clone();
    references.sort_by(f64::total_cmp);
    references.dedup_by(|left, right| left.to_bits() == right.to_bits());
    let mut subtitle = match references.as_slice() {
        [reference] => format!("Z\u{2080} = {reference} \u{3a9}"),
        [] => "Coefficient loci".to_owned(),
        _ => "Per-port retained Z\u{2080}".to_owned(),
    };
    if drawn.responses {
        subtitle.push_str(" \u{b7} responses read as ");
        subtitle.push_str(ResponseReading::OPTIONS[reading.index()]);
    }
    subtitle
}

/// One family of the Smith chart's rulings: constant resistance and
/// reactance, or, reflected through the centre, constant conductance and
/// susceptance. A family drawn second, over the first, takes its own ink,
/// labels the real axis below it and leaves the rim to the first.
fn smith_family(
    plane: &Plane,
    admittance: bool,
    second: bool,
    guides: &mut Vec<SemanticPlotGuide>,
    captions: &mut Vec<SemanticPlotCaption>,
) -> Result<(), HardcopySourceError> {
    let flip = if admittance { -1.0 } else { 1.0 };
    let mut rule = |points: &[(f64, f64)]| -> Result<(), HardcopySourceError> {
        let mut guide = plane.guide(points, false)?;
        guide.second = second;
        guides.push(guide);
        Ok(())
    };
    for value in SMITH_GRID {
        rule(&circle(
            (flip * value / (value + 1.0), 0.0),
            1.0 / (value + 1.0),
            256,
        ))?;
        captions.extend(plane.label(
            (flip * (value - 1.0) / (value + 1.0), 0.0),
            if second { (-400, 2_600) } else { (-400, -700) },
            grid_value(value),
            SemanticCaptionAnchor::End,
        ));
        for sign in [1.0, -1.0] {
            let x = sign * value;
            rule(&arc_in_unit_disc((flip, flip / x), 1.0 / value))?;
            if second {
                // Its arcs meet the rim where the first family's
                // opposite-signed arcs do, which label it already.
                continue;
            }
            // Γ = (jx − 1)/(jx + 1): where the arc meets the rim.
            let denominator = 1.0 + x * x;
            let rim = (
                flip * (x * x - 1.0) / denominator,
                flip * 2.0 * x / denominator,
            );
            let outward = (rim.0 * 2_200.0, -rim.1 * 2_200.0);
            captions.extend(plane.label(
                rim,
                (outward.0.round() as i64, outward.1.round() as i64 + 700),
                format!(
                    "{}j{}",
                    if sign < 0.0 { "\u{2212}" } else { "" },
                    grid_value(value)
                ),
                if rim.0 < -0.1 {
                    SemanticCaptionAnchor::End
                } else {
                    SemanticCaptionAnchor::Start
                },
            ));
        }
    }
    Ok(())
}

/// The Smith page as the sheet stands: the unit circle and the grid it
/// rules, impedance, admittance or both, labelled as it labels them; the
/// loci it draws, a response read through its reading; and, where its
/// header states them, the Z₀ the chart is read against and the reading.
pub(super) fn quick_smith_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
    sheet: &QuickSmithSettings,
) -> Result<SemanticPlot, HardcopySourceError> {
    let drawn = smith_loci(active, sheet)?;
    if drawn.loci.is_empty() {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "the Smith chart has no complex coefficient to draw".to_owned(),
        ));
    }
    let plane = Plane::new((-SMITH_REACH, SMITH_REACH), (-SMITH_REACH, SMITH_REACH));
    let mut guides = vec![
        plane.guide(&circle((0.0, 0.0), 1.0, 256), true)?,
        plane.guide(&[(-1.0, 0.0), (1.0, 0.0)], false)?,
    ];
    let mut captions = vec![SemanticPlotCaption::new(
        smith_subtitle(&drawn, sheet.reading),
        SemanticPoint::new(PLOT_INSET_UM + 2_000, PLOT_INSET_UM + 4_000),
    )];
    let both = sheet.grid == SmithGrid::ImpedanceAdmittance;
    if sheet.grid.draws_admittance() {
        smith_family(&plane, true, both, &mut guides, &mut captions)?;
    }
    if sheet.grid.draws_impedance() {
        smith_family(&plane, false, false, &mut guides, &mut captions)?;
    }
    plane_plot(
        ResultViewer::Smith,
        &plane,
        &drawn.loci,
        Vec::new(),
        captions,
        guides,
    )
}

/// The rings a polar chart is ruled in, as the sheet rules them.
const POLAR_RINGS: u32 = 5;

/// The polar page: each complex quantity on its plane, with rings at the
/// magnitudes its axis rules and a spoke every 30°.
pub(super) fn quick_polar_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let loci = polar_loci(active)?;
    let reach = loci
        .iter()
        .flat_map(|locus| &locus.points)
        .map(|(re, im)| re.hypot(*im))
        .filter(|magnitude| magnitude.is_finite())
        .fold(0.0_f64, f64::max);
    if loci.is_empty() || reach <= 0.0 {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "the polar chart has no complex quantity to draw".to_owned(),
        ));
    }
    // The outer ring is the sheet's: the first 1-2-5 number at or past
    // the largest magnitude, divided into the sheet's five rings.
    let outer = rspice_results::plot_axis::polar_ring_ceiling(reach);
    let plane = Plane::new((-outer * 1.05, outer * 1.05), (-outer * 1.05, outer * 1.05));
    let mut guides = Vec::new();
    let mut captions = Vec::new();
    for ring in 1..=POLAR_RINGS {
        let radius = outer * f64::from(ring) / f64::from(POLAR_RINGS);
        guides.push(plane.guide(&circle((0.0, 0.0), radius, 256), ring == POLAR_RINGS)?);
        // Up the 90° spoke, inside each ring, as the sheet writes them:
        // clear of the angles round the rim.
        captions.extend(plane.label(
            (0.0, radius),
            (300, 2_600),
            fmt_si_significant(radius, "", 3),
            SemanticCaptionAnchor::Start,
        ));
    }
    for degrees in (0..360).step_by(30) {
        let (sin, cos) = f64::from(degrees).to_radians().sin_cos();
        guides.push(plane.guide(&[(0.0, 0.0), (outer * cos, outer * sin)], false)?);
        let rim = (outer * cos, outer * sin);
        captions.extend(plane.label(
            rim,
            (
                (cos * 1_800.0).round() as i64,
                (-sin * 1_800.0).round() as i64 + 700,
            ),
            // Signed about 0°, as the sheet reads a phase: −150° rather
            // than 210°.
            format!(
                "{}\u{b0}",
                rspice_results::number_format::tick_label(f64::from(if degrees > 180 {
                    degrees - 360
                } else {
                    degrees
                }))
            ),
            if cos < -0.1 {
                SemanticCaptionAnchor::End
            } else {
                SemanticCaptionAnchor::Start
            },
        ));
    }
    plane_plot(
        ResultViewer::Polar,
        &plane,
        &loci,
        Vec::new(),
        captions,
        guides,
    )
}

/// What the Nyquist sheet draws from one analysis: a stability run's loop
/// gain, kept as two real traces, on the interpolant its margins are read
/// from; otherwise each retained complex response, which the print still
/// draws where the sheet asks for a stability run.
struct NyquistLoci {
    loci: Vec<Locus>,
    gain_margin: Option<NyquistMargin>,
    phase_margin: Option<NyquistMargin>,
    loop_gain: bool,
}

fn nyquist_loci<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<NyquistLoci, HardcopySourceError> {
    let analysis = active.analysis;
    let identity = |name: &str| {
        format!(
            "{}:{}:{}:{}:nyquist",
            active.dataset_id, active.run_id, analysis.id, name
        )
    };
    let with_mirror = |locus: &NyquistData, label: &str, mirror: &str, loci: &mut Vec<Locus>| {
        let points: Vec<(f64, f64)> = locus.points.iter().map(|p| (p.real, p.imag)).collect();
        loci.push(Locus {
            identity: identity(label),
            label: label.to_owned(),
            points: points.clone(),
            companion: false,
            palette_slot: None,
        });
        loci.push(Locus {
            identity: identity(mirror),
            label: mirror.to_owned(),
            points: points.iter().map(|(re, im)| (*re, -im)).collect(),
            companion: true,
            palette_slot: None,
        });
    };
    if let Some((real, imaginary)) = loop_gain_traces(analysis.analysis_type, &analysis.waveforms) {
        let (real, imaginary) = (
            analysis.waveforms[real].as_ref(),
            analysis.waveforms[imaginary].as_ref(),
        );
        let (frequency, re) = whole_pair(real)?;
        let im = whole(&imaginary.name, &imaginary.y)?;
        let locus = NyquistData::from_arrays(LOOP_GAIN_LOCUS_LABEL, &frequency, &re, &im);
        let mut loci = Vec::new();
        with_mirror(
            &locus.drawn_locus(),
            LOOP_GAIN_LOCUS_LABEL,
            MIRROR_LABEL,
            &mut loci,
        );
        return Ok(NyquistLoci {
            loci,
            gain_margin: locus.gain_margin(),
            phase_margin: locus.phase_margin(),
            loop_gain: true,
        });
    }
    let mut loci = Vec::new();
    for waveform in analysis
        .waveforms
        .iter()
        .filter(|waveform| (active.is_visible)(waveform))
        .map(AsRef::as_ref)
    {
        let Some(complex) = waveform.complex.as_ref() else {
            continue;
        };
        let frequency = whole(&waveform.name, &waveform.x)?;
        let re = whole(&waveform.name, &complex.real)?;
        let im = whole(&waveform.name, &complex.imag)?;
        let locus = NyquistData::from_arrays(&waveform.name, &frequency, &re, &im);
        let mirror = format!("{} (\u{2212}\u{3c9})", waveform.name);
        with_mirror(&locus, &waveform.name, &mirror, &mut loci);
    }
    Ok(NyquistLoci {
        loci,
        gain_margin: None,
        phase_margin: None,
        loop_gain: false,
    })
}

/// The window the Nyquist sheet opens on: the critical point, the unit
/// circle and the gain-margin crossing, with room past −1.
fn critical_window(gain_margin: Option<NyquistMargin>) -> ((f64, f64), (f64, f64)) {
    let (mut left, mut right) = (-2.0_f64, 1.0_f64);
    if let Some(margin) = gain_margin {
        let crossing = -1.0 / margin.value;
        if crossing.is_finite() {
            left = left.min(crossing);
            right = right.max(crossing);
        }
    }
    let half = (right - left) * 0.5 * 1.1;
    let centre = (left + right) * 0.5;
    ((centre - half, centre + half), (-half, half))
}

/// A window round every finite point of `loci`, the critical point and the
/// unit circle.
fn whole_window(loci: &[Locus]) -> ((f64, f64), (f64, f64)) {
    let (mut left, mut right, mut reach) = (-1.0_f64, 1.0_f64, 1.0_f64);
    for (re, im) in loci.iter().flat_map(|locus| &locus.points) {
        if re.is_finite() && im.is_finite() {
            left = left.min(*re);
            right = right.max(*re);
            reach = reach.max(im.abs());
        }
    }
    let half = ((right - left) * 0.5).max(reach) * 1.05;
    let centre = (left + right) * 0.5;
    ((centre - half, centre + half), (-half, half))
}

/// The Nyquist page: what the sheet draws, from the same source rule. A
/// loop gain is framed on its critical point, with the unit circle, its
/// negative-frequency branch dashed, and each margin where it is read.
pub(super) fn quick_nyquist_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
    overlay: &RetainedQuickViewOverlay,
) -> Result<SemanticPlot, HardcopySourceError> {
    let nyquist = nyquist_loci(active)?;
    if nyquist.loci.is_empty() {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "the Nyquist plot has no loop gain or complex response to draw".to_owned(),
        ));
    }
    let (x, y) = if nyquist.loop_gain {
        critical_window(nyquist.gain_margin)
    } else {
        whole_window(&nyquist.loci)
    };
    let (x, y) = (
        overlay.window_x().unwrap_or(x),
        overlay.window_y().unwrap_or(y),
    );
    let plane = Plane::new(x, y);
    let PlaneAxes {
        ticks,
        mut captions,
        mut guides,
    } = plane_axes(&plane)?;
    guides.push(plane.guide(&circle((0.0, 0.0), 1.0, 256), false)?);
    guides.extend(plane.spot((-1.0, 0.0)));
    captions.extend(plane.label(
        (-1.0, 0.0),
        (-900, -900),
        "\u{2212}1 + j0".to_owned(),
        SemanticCaptionAnchor::End,
    ));
    if let Some(margin) = nyquist.gain_margin {
        let crossing = -1.0 / margin.value;
        if crossing.is_finite() {
            guides.extend(plane.spot((crossing, 0.0)));
            captions.extend(plane.label(
                (crossing, 0.0),
                (900, 2_600),
                // Decibels read to two places, as the sheet tags the margin.
                format!(
                    "GM {}",
                    fmt_column_reading(20.0 * margin.value.log10(), "dB", READOUT_DIGITS)
                ),
                SemanticCaptionAnchor::Start,
            ));
        }
    }
    if let Some(margin) = nyquist.phase_margin {
        let (sin, cos) = margin.phase_deg.to_radians().sin_cos();
        guides.extend(plane.spot((cos, sin)));
        captions.extend(plane.label(
            (cos, sin),
            (900, -900),
            format!(
                "PM {}",
                // An angle reads with its degree sign on the number.
                fmt_si_significant(margin.value, "", READOUT_DIGITS) + "\u{b0}"
            ),
            SemanticCaptionAnchor::Start,
        ));
    }
    plane_plot(
        ResultViewer::Nyquist,
        &plane,
        &nyquist.loci,
        ticks,
        captions,
        guides,
    )
}

#[cfg(test)]
mod tests;
