//! Quantity kinds (`tolerance.toml` `kind`), evaluated identically on every
//! producer's [`Observation`]: RSpice, a checked-in oracle table, or the live
//! held-digital O5 run. Nothing here knows which producer it is reading, which
//! is the point — a measure that treated the two sides differently would be
//! comparing the measure, not the simulators.
//!
//! Units: noise densities are power densities (V²/Hz); a producer that writes
//! an amplitude density is squared on ingestion ([`super::oracle`]).

use std::collections::BTreeMap;

use num_complex::Complex64;

use super::case::{Observation, canonical_signal};
use super::design::MeasureSpec;
use super::events::Trace;

/// What one measure produced.
#[derive(Debug, Clone, PartialEq)]
pub enum Measured {
    Values(Vec<f64>),
    Complex(Vec<Complex64>),
    Roots {
        poles: Vec<Complex64>,
        zeros: Vec<Complex64>,
    },
    Events(BTreeMap<String, Trace>),
}

/// Why a measure could not be taken.
#[derive(Debug, Clone, PartialEq)]
pub enum MeasureError {
    /// A declared signal, net or scalar is absent from the observation: a
    /// silent omission when the producer is RSpice.
    Missing(String),
    /// The quantity cannot be evaluated on this observation (empty window, no
    /// crossing, malformed spec).
    Invalid(String),
}

impl std::fmt::Display for MeasureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing(what) => write!(f, "absent: {what}"),
            Self::Invalid(what) => write!(f, "{what}"),
        }
    }
}

type MeasureResult<T> = Result<T, MeasureError>;

fn invalid<T>(message: impl Into<String>) -> MeasureResult<T> {
    Err(MeasureError::Invalid(message.into()))
}

/// Evaluate `spec` on `observation` for `variant`.
pub fn evaluate(
    spec: &MeasureSpec,
    variant: &str,
    observation: &Observation,
) -> MeasureResult<Measured> {
    let scale = spec.scale.unwrap_or(1.0);
    let scaled = |values: Vec<f64>| {
        Measured::Values(values.into_iter().map(|value| value * scale).collect())
    };
    match spec.kind.as_str() {
        "op" => {
            let mut values = Vec::new();
            for signal in signal_list(spec, variant)? {
                let series = real_series(observation, &signal)?;
                let value = *series
                    .last()
                    .ok_or_else(|| MeasureError::Missing(format!("{signal} has no value")))?;
                values.push(value);
            }
            Ok(scaled(values))
        }
        "held_at" | "interp_at" => {
            let signal = single_signal(spec, variant)?;
            let series = real_series(observation, &signal)?;
            let times = sample_times(spec)?;
            let mut values = Vec::with_capacity(times.len());
            for time in times {
                values.push(if spec.kind == "held_at" {
                    held_at(&observation.axis, &series, time)?
                } else {
                    interp_at(&observation.axis, &series, time)?
                });
            }
            Ok(scaled(values))
        }
        "mean" | "min" | "max" => {
            let signal = single_signal(spec, variant)?;
            let series = real_series(observation, &signal)?;
            let window = window(spec)?;
            let value = match spec.kind.as_str() {
                "mean" => pwl_mean(&observation.axis, &series, window)?,
                "min" => window_extreme(&observation.axis, &series, window, f64::min)?,
                _ => window_extreme(&observation.axis, &series, window, f64::max)?,
            };
            Ok(scaled(vec![value]))
        }
        "cross" if spec.nets.is_some() => {
            // The n-th digital edge of an event net: 0→1 for rise, 1→0 for fall.
            let nets = net_list(spec, variant).unwrap_or_default();
            let net = nets
                .first()
                .ok_or(MeasureError::Invalid("cross needs one net".into()))?;
            let rising = edge_is_rising(spec)?;
            let window = window_or_all(spec);
            let (from, to) = if rising { ("0", "1") } else { ("1", "0") };
            let edges: Vec<f64> = event_trace(observation, net)?
                .windows(2)
                .filter(|pair| pair[0].1 == from && pair[1].1 == to)
                .map(|pair| pair[1].0)
                .filter(|time| in_window(*time, window))
                .collect();
            let n = spec.n.unwrap_or(1);
            let time = edges
                .get(n.saturating_sub(1))
                .copied()
                .ok_or_else(|| MeasureError::Invalid(format!("{net}: fewer than {n} edges")))?;
            Ok(Measured::Values(vec![time]))
        }
        "cross" => {
            let signal = single_signal(spec, variant)?;
            let series = real_series(observation, &signal)?;
            let level = spec
                .level
                .ok_or(MeasureError::Invalid("cross needs level".into()))?;
            let rising = edge_is_rising(spec)?;
            let crossings = crossings(
                &observation.axis,
                &series,
                level,
                rising,
                window_or_all(spec),
            );
            let n = spec.n.unwrap_or(1);
            let time = crossings.get(n.saturating_sub(1)).copied().ok_or_else(|| {
                MeasureError::Invalid(format!("{signal}: fewer than {n} crossings of {level}"))
            })?;
            Ok(Measured::Values(vec![time]))
        }
        "period" => {
            let window = window_or_all(spec);
            let edges = if let Some(nets) = net_list(spec, variant) {
                let net = nets
                    .first()
                    .ok_or(MeasureError::Invalid("period needs one net".into()))?;
                let trace = event_trace(observation, net)?;
                trace
                    .windows(2)
                    .filter(|pair| pair[0].1 != "1" && pair[1].1 == "1")
                    .map(|pair| pair[1].0)
                    .filter(|time| in_window(*time, window))
                    .collect::<Vec<_>>()
            } else {
                let signal = single_signal(spec, variant)?;
                let series = real_series(observation, &signal)?;
                let level = spec
                    .level
                    .ok_or(MeasureError::Invalid("period needs level".into()))?;
                crossings(&observation.axis, &series, level, true, window)
            };
            if edges.len() < 2 {
                return invalid(format!(
                    "period: {} rising edges in the window",
                    edges.len()
                ));
            }
            let period = (edges[edges.len() - 1] - edges[0]) / (edges.len() - 1) as f64;
            Ok(Measured::Values(vec![period]))
        }
        "settle" => {
            let signal = single_signal(spec, variant)?;
            let series = real_series(observation, &signal)?;
            let target = spec
                .target
                .ok_or(MeasureError::Invalid("settle needs target".into()))?;
            let band = spec
                .band
                .ok_or(MeasureError::Invalid("settle needs band".into()))?;
            let window = window_or_all(spec);
            Ok(Measured::Values(vec![settle_time(
                &observation.axis,
                &series,
                target,
                band,
                window,
            )?]))
        }
        "dc_curve" | "noise" => {
            let signal = if spec.kind == "noise" {
                spec.signal
                    .as_ref()
                    .and_then(|signal| signal.for_variant(variant))
                    .unwrap_or_else(|| "onoise".to_owned())
            } else {
                single_signal(spec, variant)?
            };
            let series = real_series(observation, &signal)?;
            let points = axis_points(spec)?;
            let log = spec.kind == "noise";
            let values = points
                .iter()
                .map(|point| axis_interp(&observation.axis, &series, *point, log))
                .collect::<MeasureResult<Vec<_>>>()?;
            Ok(scaled(values))
        }
        "ac" => {
            let signal = single_signal(spec, variant)?;
            let series = complex_series(observation, &signal)?;
            let points = axis_points(spec)?;
            let values = points
                .iter()
                .map(|point| complex_interp(&observation.axis, &series, *point))
                .collect::<MeasureResult<Vec<_>>>()?;
            Ok(Measured::Complex(
                values.into_iter().map(|value| value * scale).collect(),
            ))
        }
        "tf" => scalar_list(observation, &selected(spec, &["gain", "zin", "zout"])),
        "stb" => {
            let names = selected(spec, &["pm", "gm", "ugf"]);
            let Some(series) = observation.complex.get("loopgain") else {
                return scalar_list(observation, &names);
            };
            let margins = stb_margins(&observation.axis, series)?;
            Ok(Measured::Values(
                names
                    .iter()
                    .map(|name| match *name {
                        "pm" => margins[0],
                        "gm" => margins[1],
                        _ => margins[2],
                    })
                    .collect(),
            ))
        }
        "sens" => {
            let params = spec
                .params
                .clone()
                .ok_or(MeasureError::Invalid("sens needs params".into()))?;
            let names: Vec<String> = params
                .iter()
                .map(|param| param.to_ascii_lowercase())
                .collect();
            let refs: Vec<&str> = names.iter().map(String::as_str).collect();
            scalar_list(observation, &refs)
        }
        "pz" => {
            let smax = spec.smax.unwrap_or(f64::INFINITY);
            let keep = |roots: &[[f64; 2]]| {
                let mut roots: Vec<Complex64> = roots
                    .iter()
                    .map(|[re, im]| Complex64::new(*re, *im))
                    .filter(|root| root.norm() < smax)
                    .collect();
                roots.sort_by(|a, b| a.norm().total_cmp(&b.norm()).then(a.im.total_cmp(&b.im)));
                roots
            };
            Ok(Measured::Roots {
                poles: keep(&observation.poles),
                zeros: keep(&observation.zeros),
            })
        }
        "events" => {
            let nets =
                net_list(spec, variant).ok_or(MeasureError::Invalid("events needs nets".into()))?;
            let window = window_or_all(spec);
            let mut traces = BTreeMap::new();
            for net in nets {
                let trace = event_trace(observation, &net)?;
                let kept: Trace = trace
                    .iter()
                    .filter(|(time, _)| in_window(*time, window) && !excluded(spec, *time))
                    .cloned()
                    .collect();
                traces.insert(net, kept);
            }
            Ok(Measured::Events(traces))
        }
        "codes" => {
            let nets =
                net_list(spec, variant).ok_or(MeasureError::Invalid("codes needs nets".into()))?;
            let strobe = spec
                .strobe
                .as_ref()
                .ok_or(MeasureError::Invalid("codes needs strobe".into()))?;
            let strobe_trace = event_trace(observation, &strobe.to_ascii_lowercase())?;
            let window = window_or_all(spec);
            let traces = nets
                .iter()
                .map(|net| event_trace(observation, net))
                .collect::<MeasureResult<Vec<_>>>()?;
            let mut codes = Vec::new();
            for pair in strobe_trace.windows(2) {
                if !(pair[0].1 != "1" && pair[1].1 == "1") {
                    continue;
                }
                let time = pair[1].0;
                if !in_window(time, window) || excluded(spec, time) {
                    continue;
                }
                let mut code = 0u64;
                for trace in &traces {
                    // Sampled just before the strobe edge: the value held at it.
                    let value = trace
                        .iter()
                        .take_while(|(at, _)| *at < time)
                        .last()
                        .map(|(_, value)| value.as_str())
                        .unwrap_or("x");
                    let bit = match value {
                        "0" => 0,
                        "1" => 1,
                        other => return invalid(format!("codes: net value {other} at {time:e}")),
                    };
                    code = (code << 1) | bit;
                }
                codes.push(code as f64);
            }
            Ok(Measured::Values(codes))
        }
        other => invalid(format!(
            "measure kind {other} is not evaluable on one observation"
        )),
    }
}

fn signal_list(spec: &MeasureSpec, variant: &str) -> MeasureResult<Vec<String>> {
    if let Some(signals) = &spec.signals {
        return Ok(signals.clone());
    }
    Ok(vec![single_signal(spec, variant)?])
}

fn single_signal(spec: &MeasureSpec, variant: &str) -> MeasureResult<String> {
    spec.signal
        .as_ref()
        .and_then(|signal| signal.for_variant(variant))
        .ok_or_else(|| {
            MeasureError::Invalid(format!(
                "{} needs a signal for variant {variant}",
                spec.kind
            ))
        })
}

fn net_list(spec: &MeasureSpec, variant: &str) -> Option<Vec<String>> {
    spec.nets
        .as_ref()
        .and_then(|nets| nets.for_variant(variant))
        .map(|nets| {
            nets.into_iter()
                .map(|net| net.to_ascii_lowercase())
                .collect()
        })
}

/// A signal expression: terms `v(a)`, `i(src)` or `node`, joined by `+`/`-`.
pub fn parse_expression(expression: &str) -> Vec<(f64, String)> {
    let mut terms = Vec::new();
    let mut sign = 1.0;
    let mut depth = 0usize;
    let mut current = String::new();
    for character in expression.chars() {
        match character {
            '(' => {
                depth += 1;
                current.push(character);
            }
            ')' => {
                depth = depth.saturating_sub(1);
                current.push(character);
            }
            '+' | '-' if depth == 0 && !current.trim().is_empty() => {
                terms.push((sign, canonical_signal(current.trim())));
                current.clear();
                sign = if character == '-' { -1.0 } else { 1.0 };
            }
            '-' if depth == 0 => sign = -sign,
            '+' if depth == 0 => {}
            _ => current.push(character),
        }
    }
    if !current.trim().is_empty() {
        terms.push((sign, canonical_signal(current.trim())));
    }
    terms
}

/// The expression's real series, summed term by term.
pub fn real_series(observation: &Observation, expression: &str) -> MeasureResult<Vec<f64>> {
    let mut total: Option<Vec<f64>> = None;
    for (sign, name) in expression_terms(expression) {
        let series = observation
            .real
            .get(&name)
            .ok_or_else(|| MeasureError::Missing(missing(observation, &name)))?;
        total = Some(match total {
            None => series.iter().map(|value| sign * value).collect(),
            Some(mut sum) => {
                if sum.len() != series.len() {
                    return invalid(format!("{name}: series length differs within {expression}"));
                }
                for (accumulated, value) in sum.iter_mut().zip(series) {
                    *accumulated += sign * value;
                }
                sum
            }
        });
    }
    total.ok_or_else(|| MeasureError::Invalid(format!("empty signal expression {expression:?}")))
}

fn complex_series(observation: &Observation, expression: &str) -> MeasureResult<Vec<Complex64>> {
    let mut total: Option<Vec<Complex64>> = None;
    for (sign, name) in expression_terms(expression) {
        let series = observation
            .complex
            .get(&name)
            .ok_or_else(|| MeasureError::Missing(missing(observation, &name)))?;
        let series: Vec<Complex64> = series
            .iter()
            .map(|[re, im]| Complex64::new(*re, *im) * sign)
            .collect();
        total = Some(match total {
            None => series,
            Some(sum) => {
                if sum.len() != series.len() {
                    return invalid(format!("{name}: series length differs within {expression}"));
                }
                sum.into_iter().zip(series).map(|(a, b)| a + b).collect()
            }
        });
    }
    total.ok_or_else(|| MeasureError::Invalid(format!("empty signal expression {expression:?}")))
}

fn expression_terms(expression: &str) -> Vec<(f64, String)> {
    // `onoise` and similar named outputs are not node voltages.
    let trimmed = expression.trim().to_ascii_lowercase();
    if matches!(trimmed.as_str(), "onoise" | "inoise") {
        return vec![(1.0, trimmed)];
    }
    parse_expression(expression)
}

fn missing(observation: &Observation, name: &str) -> String {
    let mut available = observation.available.clone();
    if available.is_empty() {
        available = observation
            .real
            .keys()
            .chain(observation.complex.keys())
            .cloned()
            .collect();
    }
    let listed = available
        .iter()
        .take(12)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{name} (producer delivered {} signals: {listed}{})",
        available.len(),
        if available.len() > 12 { ", …" } else { "" }
    )
}

fn event_trace<'a>(observation: &'a Observation, net: &str) -> MeasureResult<&'a Trace> {
    observation.events.get(net).ok_or_else(|| {
        let available: Vec<_> = observation.events.keys().take(12).cloned().collect();
        MeasureError::Missing(format!(
            "event net {net} (producer delivered {} nets: {})",
            observation.events.len(),
            available.join(", ")
        ))
    })
}

/// The quantity's `params` (a subset of `all`), or all of them.
fn selected<'a>(spec: &MeasureSpec, all: &[&'a str]) -> Vec<&'a str> {
    match &spec.params {
        Some(params) => all
            .iter()
            .filter(|name| params.iter().any(|param| param.eq_ignore_ascii_case(name)))
            .copied()
            .collect(),
        None => all.to_vec(),
    }
}

/// Phase margin (degrees), gain margin (dB) and unity-gain frequency from a
/// loop-gain series, by one rule for every producer: the series is oriented
/// so its low-frequency phase is near 0° (a producer reporting −T is
/// negated), the phase is unwrapped from there, the unity-gain frequency is
/// the first fall of |T| through 1 (log-interpolated), PM = 180° + ∠T there,
/// and GM = −|T|dB where the unwrapped phase first reaches −180°.
pub fn stb_margins(axis: &[f64], loopgain: &[[f64; 2]]) -> MeasureResult<[f64; 3]> {
    if axis.len() != loopgain.len() || axis.len() < 2 {
        return invalid("loop gain does not match its frequency axis");
    }
    let mut series: Vec<Complex64> = loopgain
        .iter()
        .map(|[re, im]| Complex64::new(*re, *im))
        .collect();
    if series[0].re < 0.0 {
        for value in &mut series {
            *value = -*value;
        }
    }
    let mut phase = Vec::with_capacity(series.len());
    let mut previous = series[0].arg().to_degrees();
    phase.push(previous);
    for value in &series[1..] {
        let mut next = value.arg().to_degrees();
        while next - previous > 180.0 {
            next -= 360.0;
        }
        while next - previous < -180.0 {
            next += 360.0;
        }
        phase.push(next);
        previous = next;
    }
    let magnitude_db: Vec<f64> = series
        .iter()
        .map(|value| 20.0 * value.norm().log10())
        .collect();
    let interpolate = |values: &[f64], index: usize, fraction: f64| {
        values[index] + fraction * (values[index + 1] - values[index])
    };
    let log_axis: Vec<f64> = axis.iter().map(|frequency| frequency.log10()).collect();
    let unity = (0..axis.len() - 1)
        .find(|index| magnitude_db[*index] >= 0.0 && magnitude_db[index + 1] < 0.0);
    let Some(index) = unity else {
        return invalid("the loop gain never falls through 0 dB in the sweep");
    };
    let fraction = magnitude_db[index] / (magnitude_db[index] - magnitude_db[index + 1]);
    let ugf = 10f64.powf(interpolate(&log_axis, index, fraction));
    let pm = 180.0 + interpolate(&phase, index, fraction);
    let gm = match (0..axis.len() - 1)
        .find(|index| phase[*index] > -180.0 && phase[index + 1] <= -180.0)
    {
        Some(index) => {
            let fraction = (phase[index] + 180.0) / (phase[index] - phase[index + 1]);
            -interpolate(&magnitude_db, index, fraction)
        }
        None => f64::INFINITY,
    };
    Ok([pm, gm, ugf])
}

fn scalar_list(observation: &Observation, names: &[&str]) -> MeasureResult<Measured> {
    names
        .iter()
        .map(|name| {
            observation
                .scalars
                .get(*name)
                .copied()
                .ok_or_else(|| MeasureError::Missing(format!("scalar {name}")))
        })
        .collect::<MeasureResult<Vec<_>>>()
        .map(Measured::Values)
}

fn sample_times(spec: &MeasureSpec) -> MeasureResult<Vec<f64>> {
    let times = match (&spec.times, &spec.at) {
        (Some(grid), _) => grid.times(),
        (None, Some(at)) => at.clone(),
        (None, None) => return invalid(format!("{} needs times or at", spec.kind)),
    };
    Ok(times
        .into_iter()
        .filter(|time| !excluded(spec, *time))
        .collect())
}

fn axis_points(spec: &MeasureSpec) -> MeasureResult<Vec<f64>> {
    match (&spec.points, &spec.times) {
        (Some(points), _) => Ok(points.clone()),
        (None, Some(grid)) => Ok(grid.times()),
        (None, None) => invalid(format!("{} needs points", spec.kind)),
    }
}

fn window(spec: &MeasureSpec) -> MeasureResult<(f64, f64)> {
    spec.window
        .map(|[start, stop]| (start, stop))
        .ok_or_else(|| MeasureError::Invalid(format!("{} needs a window", spec.kind)))
}

fn window_or_all(spec: &MeasureSpec) -> (f64, f64) {
    spec.window
        .map(|[start, stop]| (start, stop))
        .unwrap_or((f64::NEG_INFINITY, f64::INFINITY))
}

fn in_window(time: f64, (start, stop): (f64, f64)) -> bool {
    time >= start && time <= stop
}

fn excluded(spec: &MeasureSpec, time: f64) -> bool {
    spec.exclude
        .iter()
        .any(|exclusion| time >= exclusion.window[0] && time <= exclusion.window[1])
}

fn edge_is_rising(spec: &MeasureSpec) -> MeasureResult<bool> {
    match spec.edge.as_deref().unwrap_or("rise") {
        "rise" => Ok(true),
        "fall" => Ok(false),
        other => invalid(format!("edge must be rise or fall, not {other}")),
    }
}

fn check_axis(axis: &[f64], series: &[f64]) -> MeasureResult<()> {
    if axis.is_empty() || axis.len() != series.len() {
        return invalid(format!(
            "axis has {} points but the series has {}",
            axis.len(),
            series.len()
        ));
    }
    Ok(())
}

/// The last accepted sample at or before `time`.
pub fn held_at(axis: &[f64], series: &[f64], time: f64) -> MeasureResult<f64> {
    check_axis(axis, series)?;
    let index = axis.partition_point(|at| *at <= time);
    if index == 0 {
        return invalid(format!("held_at {time:e} precedes the first sample"));
    }
    Ok(series[index - 1])
}

/// Linear interpolation at `time` (smooth nodes only).
pub fn interp_at(axis: &[f64], series: &[f64], time: f64) -> MeasureResult<f64> {
    axis_interp(axis, series, time, false)
}

fn axis_interp(axis: &[f64], series: &[f64], point: f64, log: bool) -> MeasureResult<f64> {
    check_axis(axis, series)?;
    let (low, high) = bracket(axis, point)?;
    if low == high {
        return Ok(series[low]);
    }
    let (x0, x1, x) = if log && axis[low] > 0.0 && axis[high] > 0.0 && point > 0.0 {
        (axis[low].ln(), axis[high].ln(), point.ln())
    } else {
        (axis[low], axis[high], point)
    };
    let fraction = (x - x0) / (x1 - x0);
    Ok(series[low] + fraction * (series[high] - series[low]))
}

fn complex_interp(axis: &[f64], series: &[Complex64], point: f64) -> MeasureResult<Complex64> {
    if axis.is_empty() || axis.len() != series.len() {
        return invalid("complex series does not match its axis");
    }
    let (low, high) = bracket(axis, point)?;
    if low == high {
        return Ok(series[low]);
    }
    let (x0, x1, x) = if axis[low] > 0.0 && point > 0.0 {
        (axis[low].ln(), axis[high].ln(), point.ln())
    } else {
        (axis[low], axis[high], point)
    };
    let fraction = (x - x0) / (x1 - x0);
    Ok(series[low] + (series[high] - series[low]) * fraction)
}

/// Two axis values name the same point when they agree to a relative 1e-9.
/// A sweep card and a tolerance table spell one value two ways: the deck's
/// `25u` is 25 × 1e-6, one ulp below the table's `2.5e-5`, so exact equality
/// would put a window ending at the stop time outside the run.
fn same_point(value: f64, point: f64) -> bool {
    (value - point).abs() <= 1e-9 * point.abs().max(1e-30)
}

/// Indices bracketing `point`; equal when `point` is a grid value
/// ([`same_point`]) — the usual case, since both sides run the same sweep card.
fn bracket(axis: &[f64], point: f64) -> MeasureResult<(usize, usize)> {
    if let Some(index) = axis.iter().position(|value| same_point(*value, point)) {
        return Ok((index, index));
    }
    let ascending = axis.first() <= axis.last();
    let index = if ascending {
        axis.partition_point(|value| *value < point)
    } else {
        axis.partition_point(|value| *value > point)
    };
    if index == 0 || index >= axis.len() {
        return invalid(format!("point {point:e} lies outside the producer's axis"));
    }
    Ok((index - 1, index))
}

/// Time-weighted piecewise-linear mean over `[start, stop]`.
pub fn pwl_mean(axis: &[f64], series: &[f64], (start, stop): (f64, f64)) -> MeasureResult<f64> {
    if series.len() == 1 && axis.len() <= 1 {
        // An operating point is constant in time.
        return Ok(series[0]);
    }
    check_axis(axis, series)?;
    let (first, last) = (axis[0], axis[axis.len() - 1]);
    // A window end that is the run's end spelled another way is the run's end.
    let start = if same_point(first, start) { first } else { start };
    let stop = if same_point(last, stop) { last } else { stop };
    if stop.partial_cmp(&start) != Some(std::cmp::Ordering::Greater) || start < first || stop > last
    {
        return invalid(format!(
            "mean window [{start:e}, {stop:e}] is not inside the run"
        ));
    }
    let value_at = |time: f64| axis_interp(axis, series, time, false);
    let mut area = 0.0;
    let mut previous_time = start;
    let mut previous_value = value_at(start)?;
    for (index, time) in axis.iter().enumerate() {
        if *time <= start {
            continue;
        }
        if *time >= stop {
            break;
        }
        area += 0.5 * (previous_value + series[index]) * (time - previous_time);
        previous_time = *time;
        previous_value = series[index];
    }
    let last = value_at(stop)?;
    area += 0.5 * (previous_value + last) * (stop - previous_time);
    Ok(area / (stop - start))
}

fn window_extreme(
    axis: &[f64],
    series: &[f64],
    (start, stop): (f64, f64),
    pick: fn(f64, f64) -> f64,
) -> MeasureResult<f64> {
    if series.len() == 1 && axis.len() <= 1 {
        return Ok(series[0]);
    }
    check_axis(axis, series)?;
    let mut extreme = pick(
        axis_interp(axis, series, start, false)?,
        axis_interp(axis, series, stop, false)?,
    );
    for (time, value) in axis.iter().zip(series) {
        if *time > start && *time < stop {
            extreme = pick(extreme, *value);
        }
    }
    Ok(extreme)
}

/// Linearly interpolated crossing times of `level` in the window.
pub fn crossings(
    axis: &[f64],
    series: &[f64],
    level: f64,
    rising: bool,
    window: (f64, f64),
) -> Vec<f64> {
    let mut times = Vec::new();
    for index in 1..axis.len().min(series.len()) {
        let (a, b) = (series[index - 1] - level, series[index] - level);
        let crossed = if rising {
            a < 0.0 && b >= 0.0
        } else {
            a > 0.0 && b <= 0.0
        };
        if !crossed {
            continue;
        }
        let fraction = a / (a - b);
        let time = axis[index - 1] + fraction * (axis[index] - axis[index - 1]);
        if in_window(time, window) {
            times.push(time);
        }
    }
    times
}

/// First time after which `|x − target| ≤ band` holds to the window's end.
pub fn settle_time(
    axis: &[f64],
    series: &[f64],
    target: f64,
    band: f64,
    window: (f64, f64),
) -> MeasureResult<f64> {
    check_axis(axis, series)?;
    let indices: Vec<usize> = (0..axis.len())
        .filter(|index| in_window(axis[*index], window))
        .collect();
    let Some(&last) = indices.last() else {
        return invalid("settle window holds no samples");
    };
    if (series[last] - target).abs() > band {
        return invalid(format!(
            "never settles: the window ends {:e} from the target, band {band:e}",
            series[last] - target
        ));
    }
    let mut settled = axis[indices[0]];
    for pair in indices.windows(2) {
        let (i, j) = (pair[0], pair[1]);
        let outside_i = (series[i] - target).abs() > band;
        let outside_j = (series[j] - target).abs() > band;
        if outside_i && !outside_j {
            // Entered the band between i and j: interpolate the boundary.
            let boundary = if series[i] > target {
                target + band
            } else {
                target - band
            };
            let fraction = (boundary - series[i]) / (series[j] - series[i]);
            settled = axis[i] + fraction * (axis[j] - axis[i]);
        } else if outside_j {
            settled = axis[j];
        }
    }
    Ok(settled)
}

/// Evaluate a scalar statistic.
pub fn statistic(name: &str, values: &[f64]) -> MeasureResult<f64> {
    if values.len() < 2 {
        return invalid(format!(
            "{name} needs at least two values, got {}",
            values.len()
        ));
    }
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let var = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (n - 1.0);
    match name {
        "mean" => Ok(mean),
        "var" => Ok(var),
        "std" => Ok(var.sqrt()),
        other => invalid(format!("unknown statistic {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suites::mixed::design::{PerVariant, TimeGrid};

    fn ramp() -> Observation {
        let mut observation = Observation {
            kind: "tran".into(),
            axis: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            ..Default::default()
        };
        observation
            .real
            .insert("v(a)".into(), vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        observation
            .real
            .insert("v(b)".into(), vec![1.0, 1.0, 1.0, 1.0, 1.0]);
        observation
    }

    fn spec(kind: &str) -> MeasureSpec {
        MeasureSpec {
            kind: kind.into(),
            signal: Some(PerVariant::All("v(a)".into())),
            ..Default::default()
        }
    }

    #[test]
    fn held_and_interpolated_samples_differ_between_grid_points() {
        let observation = ramp();
        let mut held = spec("held_at");
        held.at = Some(vec![1.5, 4.0]);
        assert_eq!(
            evaluate(&held, "x", &observation).unwrap(),
            Measured::Values(vec![1.0, 4.0])
        );
        let mut interp = spec("interp_at");
        interp.at = Some(vec![1.5]);
        interp.scale = Some(2.0);
        assert_eq!(
            evaluate(&interp, "x", &observation).unwrap(),
            Measured::Values(vec![3.0])
        );
        held.at = Some(vec![-1.0]);
        assert!(evaluate(&held, "x", &observation).is_err());
    }

    #[test]
    fn expressions_difference_signals_and_missing_ones_are_named() {
        let observation = ramp();
        let mut op = spec("op");
        op.signal = Some(PerVariant::All("v(a)-v(b)".into()));
        assert_eq!(
            evaluate(&op, "x", &observation).unwrap(),
            Measured::Values(vec![3.0])
        );
        op.signal = Some(PerVariant::All("v(missing)".into()));
        assert!(matches!(
            evaluate(&op, "x", &observation),
            Err(MeasureError::Missing(_))
        ));
        assert_eq!(
            parse_expression("V(xmod.outintp) - v(xmod.outintn)"),
            vec![
                (1.0, "v(xmod.outintp)".into()),
                (-1.0, "v(xmod.outintn)".into())
            ]
        );
    }

    #[test]
    fn window_mean_crossing_settle_and_period_on_a_ramp() {
        let observation = ramp();
        let mut mean = spec("mean");
        mean.window = Some([1.0, 3.0]);
        assert_eq!(
            evaluate(&mean, "x", &observation).unwrap(),
            Measured::Values(vec![2.0])
        );
        let mut cross = spec("cross");
        cross.level = Some(2.5);
        assert_eq!(
            evaluate(&cross, "x", &observation).unwrap(),
            Measured::Values(vec![2.5])
        );
        let mut settle = spec("settle");
        settle.target = Some(4.0);
        settle.band = Some(0.5);
        assert_eq!(
            evaluate(&settle, "x", &observation).unwrap(),
            Measured::Values(vec![3.5])
        );
        let wave: Vec<f64> = (0..41)
            .map(|k| (k as f64 * 0.25 * std::f64::consts::PI).sin())
            .collect();
        let mut sine = Observation {
            axis: (0..41).map(|k| k as f64 * 0.125).collect(),
            ..Default::default()
        };
        sine.real.insert("v(a)".into(), wave);
        let mut period = spec("period");
        period.level = Some(0.0);
        let Measured::Values(value) = evaluate(&period, "x", &sine).unwrap() else {
            panic!()
        };
        assert!((value[0] - 1.0).abs() < 1e-9, "{value:?}");
    }

    #[test]
    fn a_window_ending_at_the_stop_time_spelled_otherwise_is_inside_the_run() {
        // `.tran 0.1n 25u` stops at 25 × 1e-6, one ulp below the 2.5e-5 a
        // tolerance table writes for the same instant.
        let stop = 25.0 * 1e-6;
        assert!(stop < 2.5e-5);
        let axis = vec![0.0, 1e-5, 2e-5, stop];
        let series = vec![0.0, 1.0, 2.0, 2.5];
        let mean = pwl_mean(&axis, &series, (2e-5, 2.5e-5)).unwrap();
        assert!((mean - 2.25).abs() < 1e-9, "{mean}");
        // A window that genuinely runs past the stop time is still refused.
        assert!(pwl_mean(&axis, &series, (2e-5, 2.6e-5)).is_err());
        assert!(pwl_mean(&axis, &series, (-1e-6, 1e-5)).is_err());
    }

    #[test]
    fn loop_margins_follow_one_rule_for_either_sign_convention() {
        // T = 100 / (1 + j f/10)^2: |T| = 1 near 99.5 Hz, where each pole
        // contributes −84.26° of phase.
        let axis: Vec<f64> = (0..=60).map(|k| 10f64.powf(k as f64 / 10.0)).collect();
        let series: Vec<[f64; 2]> = axis
            .iter()
            .map(|f| {
                let pole = Complex64::new(1.0, f / 10.0);
                let t = Complex64::new(100.0, 0.0) / (pole * pole);
                [t.re, t.im]
            })
            .collect();
        let [pm, gm, ugf] = stb_margins(&axis, &series).unwrap();
        assert!((ugf - 99.5).abs() < 2.0, "{ugf}");
        assert!(
            (pm - (180.0 - 2.0 * (ugf / 10.0).atan().to_degrees())).abs() < 0.5,
            "{pm}"
        );
        assert!(gm.is_infinite(), "two poles never reach −180°: {gm}");
        let negated: Vec<[f64; 2]> = series.iter().map(|[re, im]| [-re, -im]).collect();
        assert_eq!(stb_margins(&axis, &negated).unwrap(), [pm, gm, ugf]);
    }

    #[test]
    fn statistics_use_the_unbiased_variance() {
        assert_eq!(statistic("mean", &[1.0, 3.0]).unwrap(), 2.0);
        assert_eq!(statistic("var", &[1.0, 3.0]).unwrap(), 2.0);
        assert!(statistic("var", &[1.0]).is_err());
    }

    #[test]
    fn codes_sample_nets_before_each_strobe_rise() {
        let mut observation = Observation::default();
        observation.events.insert(
            "clk".into(),
            vec![
                (0.0, "0".into()),
                (1.0, "1".into()),
                (2.0, "0".into()),
                (3.0, "1".into()),
            ],
        );
        observation
            .events
            .insert("b1".into(), vec![(0.0, "1".into()), (2.5, "0".into())]);
        observation
            .events
            .insert("b0".into(), vec![(0.0, "0".into()), (1.0, "1".into())]);
        let codes = MeasureSpec {
            kind: "codes".into(),
            nets: Some(PerVariant::All(vec!["b1".into(), "b0".into()])),
            strobe: Some("clk".into()),
            ..Default::default()
        };
        // At t=1 b0 changes on the strobe itself, so the held value is 0.
        assert_eq!(
            evaluate(&codes, "x", &observation).unwrap(),
            Measured::Values(vec![2.0, 1.0])
        );
        let mut times = spec("held_at");
        times.times = Some(TimeGrid {
            start: 0.5,
            step: 1.0,
            count: 3,
        });
        assert_eq!(
            evaluate(&times, "x", &ramp()).unwrap(),
            Measured::Values(vec![0.0, 1.0, 2.0])
        );
    }
}
