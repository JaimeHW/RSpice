//! The coefficient spectrum's strips: the levels of each quantity's lines,
//! their phase, and the waveform they add up to.
//!
//! The strips follow the stack's rule for a sheet the reader never arranged,
//! one strip per unit, written as the view's strip model writes a waveform
//! pane's: a voltage and a current are never read against one axis.

use super::{HarmonicBalanceModel, HarmonicTrace, SpectrumResponse};
use crate::fft::view::phase_axis;
use crate::presentation::{PlotView, frequency_readout, well_hint};
use crate::strip::{self, LegendChip};
use crate::waves::ReadoutPolicy;
use egui::Ui;
use rspice_results::harmonic_spectrum::lines::{carrier, rebuild_waveform};
use rspice_results::harmonic_spectrum::settings::{
    CoefficientQuantity, CoefficientScale, CoefficientSettings,
};
use rspice_results::visualization_document::{SignalForm, Strip, StripAxis, StripMember};
use rspice_ui_kit::plot::{self, Axis, DrawMode, PlotSpec, Trace, XScale, fmt_si_significant};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::sync::Arc;

/// The quantities in the order their strips stand, top to bottom.
const QUANTITIES: [CoefficientQuantity; 3] = [
    CoefficientQuantity::Voltage,
    CoefficientQuantity::Current,
    CoefficientQuantity::Other,
];

/// How many periods of its lowest line the rebuilt waveform spans.
const WAVEFORM_PERIODS: f64 = 2.0;

/// The strips the sheet draws: per quantity present, its levels, then its
/// phase and its waveform when the reader asked for them.
pub(super) fn strips(model: &HarmonicBalanceModel, settings: &CoefficientSettings) -> Vec<Strip> {
    let mut strips = Vec::new();
    for quantity in QUANTITIES {
        let names = model
            .traces
            .iter()
            .filter(|trace| trace.quantity() == quantity)
            .map(|trace| trace.name.clone())
            .collect::<Vec<_>>();
        if names.is_empty() {
            continue;
        }
        let mut push = |form: SignalForm, unit: &str| {
            strips.push(Strip {
                key: format!("{}-{form:?}", quantity.unit()).to_ascii_lowercase(),
                weight: None,
                locked: false,
                axes: vec![StripAxis {
                    unit: unit.to_owned(),
                    members: names
                        .iter()
                        .map(|name| StripMember::Signal {
                            name: name.clone(),
                            form,
                        })
                        .collect(),
                }],
            });
        };
        push(SignalForm::Magnitude, settings.unit(quantity));
        if settings.phase {
            push(SignalForm::Phase, "°");
        }
        if settings.waveform {
            push(SignalForm::Value, quantity.unit());
        }
    }
    strips
}

/// One curve a strip draws: its trace and the arrays it is drawn from.
struct Curve<'a> {
    trace: &'a HarmonicTrace,
    x: Arc<[f64]>,
    y: Arc<[f64]>,
}

fn curves<'a>(
    model: &'a HarmonicBalanceModel,
    strip: &Strip,
    settings: &CoefficientSettings,
) -> (SignalForm, Vec<Curve<'a>>) {
    let mut form = SignalForm::Magnitude;
    let mut out = Vec::new();
    for member in strip.members() {
        let StripMember::Signal {
            name,
            form: member_form,
        } = member
        else {
            continue;
        };
        let Some(trace) = model.traces.iter().find(|trace| &trace.name == name) else {
            continue;
        };
        form = *member_form;
        let (x, y): (Vec<f64>, Vec<f64>) = match member_form {
            SignalForm::Phase => trace
                .lines
                .iter()
                .map(|line| (line.frequency, line.phase_degrees()))
                .unzip(),
            SignalForm::Value => {
                rebuild_waveform(&trace.lines, WAVEFORM_PERIODS).unwrap_or_default()
            }
            _ => {
                let quantity = trace.quantity();
                let carrier = carrier(&trace.lines);
                trace
                    .lines
                    .iter()
                    .map(|line| {
                        (
                            line.frequency,
                            settings.level(line.magnitude(), line.frequency, quantity, carrier),
                        )
                    })
                    .unzip()
            }
        };
        out.push(Curve {
            trace,
            x: x.into(),
            y: y.into(),
        });
    }
    (form, out)
}

/// The finite (lowest, highest) of every curve's ordinate.
fn extent(curves: &[Curve<'_>]) -> Option<(f64, f64)> {
    curves
        .iter()
        .flat_map(|curve| curve.y.iter().copied())
        .filter(|value| value.is_finite())
        .fold(None, |held, value| match held {
            None => Some((value, value)),
            Some((lo, hi)) => Some((f64::min(lo, value), f64::max(hi, value))),
        })
}

/// A curve's identity in the decimation cache: its trace, what it draws and
/// how, so a change of scale never serves the arrays of the last one.
fn curve_key(trace: &HarmonicTrace, form: SignalForm, settings: &CoefficientSettings) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    trace.cache_key.hash(&mut hasher);
    form.hash(&mut hasher);
    if form == SignalForm::Magnitude {
        (settings.scale as u8).hash(&mut hasher);
        (settings.amplitude as u8).hash(&mut hasher);
        settings.reference_ohms.to_bits().hash(&mut hasher);
    }
    hasher.finish()
}

/// Draw every strip, each on the viewport kept under its index.
#[allow(clippy::too_many_arguments)]
pub(super) fn show(
    ui: &mut Ui,
    model: &HarmonicBalanceModel,
    settings: &CoefficientSettings,
    views: &[PlotView],
    quantity_policy: &QuantityPresentationPolicy,
    presentation: ReadoutPolicy,
    cache: &mut plot::DecimationCache,
    pin: &mut Option<f64>,
) -> SpectrumResponse {
    let mut response = SpectrumResponse {
        fit: false,
        plots: Vec::new(),
    };
    let strips = strips(model, settings);
    if strips.is_empty() {
        well_hint(
            ui,
            "No retained complex coefficient spectrum for the selected analysis",
        );
        return response;
    }
    #[allow(clippy::cast_precision_loss)]
    let height = (ui.available_height() / strips.len() as f32).max(96.0);
    let width = ui.available_width();
    for (index, strip) in strips.iter().enumerate() {
        let view = views.get(index).copied().unwrap_or_default();
        let shown = ui
            .allocate_ui(egui::vec2(width, height), |ui| {
                show_strip(
                    ui,
                    model,
                    strip,
                    settings,
                    view,
                    quantity_policy,
                    presentation,
                    cache,
                    // The pinned readout follows the first strip.
                    (index == 0).then_some(&mut *pin),
                )
            })
            .inner;
        response.fit |= shown.0;
        if let Some(plot) = shown.1 {
            response.plots.push((index, plot));
        }
    }
    response
}

#[allow(clippy::too_many_arguments)]
fn show_strip(
    ui: &mut Ui,
    model: &HarmonicBalanceModel,
    strip: &Strip,
    settings: &CoefficientSettings,
    view: PlotView,
    quantity_policy: &QuantityPresentationPolicy,
    presentation: ReadoutPolicy,
    cache: &mut plot::DecimationCache,
    pin: Option<&mut Option<f64>>,
) -> (bool, Option<plot::PlotResponse>) {
    let digits = usize::from(presentation.displayed_significant_digits().get());
    let unit = strip.axes.first().map_or("", |axis| axis.unit.as_str());
    let (form, curves) = curves(model, strip, settings);
    let legend = curves
        .iter()
        .map(|curve| LegendChip {
            name: &curve.trace.name,
            color: curve.trace.color,
            on: true,
            glyph: strip::LegendGlyph::Line,
        })
        .collect::<Vec<_>>();
    let kind = match form {
        SignalForm::Phase => "PHASE",
        SignalForm::Value => "WAVEFORM",
        _ => "SPECTRUM",
    };
    // The legend names the signals and the bar the level, so the strip
    // states nothing more.
    let header = strip::StripHeader::new(kind, "", &legend)
        .id_salt(("rspice.results.harmonic-balance.strip", &strip.key))
        .zoomed(view.is_zoomed())
        .maximizable(false)
        .show(ui);
    let Some((lo, hi)) = extent(&curves) else {
        well_hint(ui, "The retained lines have no finite level to draw");
        return (header.fit_clicked, None);
    };
    let x_max = curves
        .iter()
        .flat_map(|curve| curve.x.last().copied())
        .fold(0.0_f64, f64::max);
    let (x0, x1) = view.x.unwrap_or(match form {
        SignalForm::Value => (0.0, x_max.max(f64::MIN_POSITIVE)),
        _ => (0.0, if x_max > 0.0 { x_max * 1.08 } else { 1.0 }),
    });
    let logarithmic =
        form == SignalForm::Magnitude && settings.scale != CoefficientScale::Magnitude;
    let (y0, y1) = view.y.unwrap_or(match form {
        SignalForm::Phase => (-180.0, 180.0),
        // A line under the floor of a dB strip is drawn down to it: two
        // hundred decibels under the strongest line is beneath any figure.
        _ if logarithmic => ((lo - 10.0).max(hi - 200.0), hi + 10.0),
        SignalForm::Value => super::padded_bounds(lo, hi).unwrap_or((lo - 1.0, hi + 1.0)),
        _ => (lo.min(0.0), if hi > 0.0 { hi * 1.08 } else { 1.0 }),
    });
    if !(x0 < x1 && y0 < y1) {
        well_hint(ui, "The retained lines span no range to draw");
        return (header.fit_clicked, None);
    }
    let x_axis = if form == SignalForm::Value {
        Axis::linear(x0, x1, "s")
    } else {
        let (scale, offset, frequency_unit) = quantity_policy.frequency_axis_transform();
        Axis::linear(x0, x1, "Hz").with_display_transform(scale, offset, frequency_unit)
    };
    let y_axis = if form == SignalForm::Phase {
        phase_axis(y0, y1)
    } else {
        Axis::linear_with(y0, y1, unit, 7)
    };
    let mut spec = PlotSpec::new(x_axis, XScale::Linear, y_axis).accessible_name(match form {
        SignalForm::Phase => "Coefficient spectrum phase",
        SignalForm::Value => "Waveform rebuilt from the coefficient spectrum",
        _ => "Coefficient spectrum levels",
    });
    if form != SignalForm::Value {
        // The lines are discrete, drawn as a spectrum analyser draws them:
        // a stem to each line, from the strip's floor on a decibel scale and
        // from zero on a magnitude or a phase, under the line's dot, with
        // nothing joining one line to the next.
        let floor = if logarithmic { y0 } else { 0.0 };
        let stems = curves
            .iter()
            .map(|curve| {
                (
                    Arc::clone(&curve.x),
                    Arc::clone(&curve.y),
                    curve.trace.color,
                )
            })
            .collect::<Vec<_>>();
        spec.underlay = Some(Box::new(move |painter, mapper| {
            let baseline = mapper.y(floor);
            for (x, y, color) in &stems {
                for (&x, &y) in x.iter().zip(y.iter()) {
                    if y.is_finite() {
                        let tip = if logarithmic { y.max(floor) } else { y };
                        painter.line_segment(
                            [
                                egui::pos2(mapper.x(x), baseline),
                                egui::pos2(mapper.x(x), mapper.y(tip)),
                            ],
                            egui::Stroke::new(1.0, *color),
                        );
                    }
                }
            }
        }));
        if !logarithmic {
            spec.ref_lines.push(plot::RefLine { y: 0.0 });
        }
    }
    for curve in &curves {
        let trace = Trace::new(&curve.x, &curve.y, curve.trace.color)
            .thin()
            .cache_key(curve_key(curve.trace, form, settings));
        spec.traces.push(if form == SignalForm::Value {
            trace
        } else {
            trace.draw(DrawMode::Points)
        });
    }
    let readout = |x: f64| -> Vec<(String, String)> {
        if form == SignalForm::Value {
            let mut rows = vec![("t".to_owned(), fmt_si_significant(x, "s", digits))];
            for curve in &curves {
                rows.push((
                    curve.trace.name.clone(),
                    fmt_si_significant(
                        presentation.cursor_sample(&curve.x, &curve.y, x),
                        unit,
                        digits,
                    ),
                ));
            }
            return rows;
        }
        // Nothing lies between two lines: the cursor reads the nearest.
        let line = curves
            .first()
            .and_then(|curve| super::nearest_line(&curve.x, x).map(|index| curve.x[index]));
        let mut rows = vec![(
            "f".to_owned(),
            frequency_readout(quantity_policy, line.unwrap_or(x), digits),
        )];
        for curve in &curves {
            let level = super::nearest_line(&curve.x, x).map_or(f64::NAN, |index| curve.y[index]);
            rows.push((
                curve.trace.name.clone(),
                fmt_si_significant(level, unit, digits),
            ));
        }
        rows
    };
    let response = plot::show(ui, &spec, cache, None, Some(&readout));
    if let Some(pin) = pin {
        crate::chrome::pinned_readout::follow(ui, &response, spec.x_scale, pin, &readout);
    }
    (header.fit_clicked, Some(response))
}
