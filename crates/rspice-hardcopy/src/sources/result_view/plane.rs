//! The complex-plane pages: Smith, polar and Nyquist.
//!
//! A plane is read by its distances and angles, so both axes share one
//! scale. The window the sheet opens on is widened about its centre along
//! the axis with room to spare until it fills the plot area, as the sheet's
//! `equal_scale_ranges` fills its rect, rather than letterboxing a square.

use super::*;
use rspice_results::number_format::fmt_si_significant;
use rspice_results::nyquist::{
    LOOP_GAIN_LOCUS_LABEL, NyquistData, NyquistMargin, loop_gain_traces,
};

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

/// The plane's own axes: Re along the bottom, Im up the left, ruled on the
/// one scale both share, and the two lines through the origin.
fn plane_axes(
    plane: &Plane,
) -> Result<
    (
        Vec<SemanticAxisTick>,
        Vec<SemanticPlotCaption>,
        Vec<SemanticPlotGuide>,
    ),
    HardcopySourceError,
> {
    let (ticks, mut captions) = plot_axes(AxisScale::Linear, AxisScale::Linear, &plane.frame())?;
    captions.push(SemanticPlotCaption {
        text: "Re".to_owned(),
        position: SemanticPoint::new(
            PLOT_INSET_UM - 1_000,
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
    Ok((ticks, captions, guides))
}

/// The complex loci a polar or Smith sheet draws from one analysis: each
/// shown trace the `renderability` rule names a quantity of the polar
/// sheet, a network term or a complex response.
fn polar_loci<W: AsRef<RetainedWaveform>>(active: &RetainedQuickViewSource<'_, W>) -> Vec<Locus> {
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
            Some(Locus {
                identity: format!(
                    "{}:{}:{}:{}:complex",
                    active.dataset_id, active.run_id, active.analysis.id, waveform.name
                ),
                label: waveform.name.clone(),
                points: complex
                    .real
                    .samples(..)
                    .zip(complex.imag.samples(..))
                    .collect(),
                companion: false,
            })
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

/// The Smith page: the unit circle, the constant-resistance circles and
/// constant-reactance arcs the sheet rules, labelled as it labels them, and
/// each coefficient drawn as the sheet's Γ reading draws it.
pub(super) fn quick_smith_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let loci = polar_loci(active);
    if loci.is_empty() {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "the Smith chart has no complex coefficient to draw".to_owned(),
        ));
    }
    let plane = Plane::new((-SMITH_REACH, SMITH_REACH), (-SMITH_REACH, SMITH_REACH));
    let mut guides = vec![
        plane.guide(&circle((0.0, 0.0), 1.0, 256), true)?,
        plane.guide(&[(-1.0, 0.0), (1.0, 0.0)], false)?,
    ];
    let mut captions = Vec::new();
    for value in SMITH_GRID {
        guides.push(plane.guide(
            &circle((value / (value + 1.0), 0.0), 1.0 / (value + 1.0), 256),
            false,
        )?);
        captions.extend(plane.label(
            ((value - 1.0) / (value + 1.0), 0.0),
            (-400, -700),
            grid_value(value),
            SemanticCaptionAnchor::End,
        ));
        for sign in [1.0, -1.0] {
            let x = sign * value;
            guides.push(plane.guide(&arc_in_unit_disc((1.0, 1.0 / x), 1.0 / value), false)?);
            // Γ = (jx − 1)/(jx + 1): where the arc meets the rim.
            let denominator = 1.0 + x * x;
            let rim = ((x * x - 1.0) / denominator, 2.0 * x / denominator);
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
    plane_plot(
        ResultViewer::Smith,
        &plane,
        &loci,
        Vec::new(),
        captions,
        guides,
    )
}

/// The polar page: each complex quantity on its plane, with rings at the
/// magnitudes its axis rules and a spoke every 30°.
pub(super) fn quick_polar_plot<W: AsRef<RetainedWaveform>>(
    active: &RetainedQuickViewSource<'_, W>,
) -> Result<SemanticPlot, HardcopySourceError> {
    let loci = polar_loci(active);
    let reach = loci
        .iter()
        .flat_map(|locus| &locus.points)
        .map(|(re, im)| re.hypot(*im))
        .filter(|magnitude| magnitude.is_finite())
        .fold(0.0_f64, f64::max);
    if loci.is_empty() || !(reach > 0.0) {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "the polar chart has no complex quantity to draw".to_owned(),
        ));
    }
    // The outermost ring is the first the axis rules at or past the
    // largest magnitude, so every locus sits inside its rim.
    let step = rspice_results::plot_axis::linear_ticks(0.0, reach, 4).step;
    let outer = if step > 0.0 {
        (reach / step).ceil() * step
    } else {
        reach
    };
    let plane = Plane::new((-outer * 1.05, outer * 1.05), (-outer * 1.05, outer * 1.05));
    let mut guides = Vec::new();
    let mut captions = Vec::new();
    let ruled = rspice_results::plot_axis::linear_ticks_at(0.0, outer * (1.0 + 1.0e-9), step);
    for (index, (radius, label)) in ruled.ticks.iter().enumerate() {
        if *radius <= 0.0 {
            continue;
        }
        let rim = index + 1 == ruled.ticks.len();
        guides.push(plane.guide(&circle((0.0, 0.0), *radius, 256), rim)?);
        captions.extend(plane.label(
            (*radius, 0.0),
            (300, -500),
            label.clone(),
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
            format!("{degrees}\u{b0}"),
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
        });
        loci.push(Locus {
            identity: identity(mirror),
            label: mirror.to_owned(),
            points: points.iter().map(|(re, im)| (*re, -im)).collect(),
            companion: true,
        });
    };
    if let Some((real, imaginary)) = loop_gain_traces(analysis.analysis_type, &analysis.waveforms) {
        let (real, imaginary) = (
            analysis.waveforms[real].as_ref(),
            analysis.waveforms[imaginary].as_ref(),
        );
        let (frequency, re) = whole_pair(real)?;
        let im = imaginary.y.read_whole().map_err(|failure| {
            HardcopySourceError::InvalidVisualizationSource(format!(
                "{} could not be read: {failure}",
                imaginary.name
            ))
        })?;
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
        let frequency = waveform.x.samples(..).collect::<Vec<_>>();
        let re = complex.real.samples(..).collect::<Vec<_>>();
        let im = complex.imag.samples(..).collect::<Vec<_>>();
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
    let (ticks, mut captions, mut guides) = plane_axes(&plane)?;
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
                format!(
                    "GM {}",
                    fmt_si_significant(20.0 * margin.value.log10(), "dB", READOUT_DIGITS)
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
