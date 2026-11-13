//! Analog-to-digital and digital-to-analog conversion: `digital()` reads an
//! analog signal as logic levels, `analog()` draws a bus word as a staircase.
//!
//! # `digital(w, vth[, vtl[, tx]])`
//!
//! The semantics are Virtuoso Visualization and Analysis XL's A/D conversion
//! (User Guide 6.1.6, Measurements > Analog To Digital, and the calculator's
//! `a2d`, OCEAN `awvAnalog2Digital`):
//!
//! - With one threshold, a value equal to or greater than it is 1 and a value
//!   below it is 0.
//! - With a high and a low threshold, a value equal to or greater than the
//!   high one is 1 and a value equal to or less than the low one is 0. Between
//!   them the signal keeps the level it had, which is the hysteresis, for at
//!   most the time to X; once it has stayed between them longer than that it
//!   is X. Omitted, the time to X is unlimited, so the band only holds; zero
//!   makes the whole band X.
//! - A signal that starts between the thresholds has no level to keep and is
//!   X until it reaches one of them.
//!
//! An edge is dated where the linearly interpolated signal meets the
//! threshold, never at the next retained sample, and the X that a time limit
//! produces is dated at the instant the limit ran out.
//!
//! The result is a step waveform: two samples at every edge instant, one with
//! the level before and one with the level after, so it is horizontal
//! between edges and vertical at them. 0 and 1 are the levels and X is a hole.
//! The waveform stack reads a `digital()` trace as a logic lane.
//!
//! # `analog(bus)`
//!
//! A bus or a logic net, read from its exact event history, as a staircase of
//! its word's integer value: unsigned, or two's complement in the bus's own
//! width when its radix is signed. A word with an unknown or high-impedance
//! bit denotes no integer and is a hole.

use super::super::ast::CalculatorExpr;
use super::super::evaluator::EvaluationError;
use super::super::value::RealValue;
use super::{forward_domain, scalar_arg, series_arg};

/// Whether an expression is a `digital()` conversion as a whole, which the
/// waveform stack draws as a logic lane rather than as a value trace.
pub fn is_logic_expression(expr: &CalculatorExpr) -> bool {
    matches!(expr, CalculatorExpr::FunctionCall { name, .. } if name.eq_ignore_ascii_case("digital"))
}

/// One level of a converted signal.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Level {
    Low,
    High,
    Unknown,
}

impl Level {
    const fn value(self) -> f64 {
        match self {
            Self::Low => 0.0,
            Self::High => 1.0,
            Self::Unknown => f64::NAN,
        }
    }
}

/// Where a value lies against the thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Region {
    Low,
    Band,
    High,
}

/// How `digital()` reads a value.
#[derive(Debug, Clone, Copy)]
struct Thresholds {
    high: f64,
    /// Equal to `high` for a single threshold, which then has no band.
    low: f64,
    single: bool,
    /// How long the signal may stay in the band before it is X.
    time_to_x: f64,
}

impl Thresholds {
    fn region(&self, y: f64) -> Region {
        if self.single {
            if y >= self.high {
                Region::High
            } else {
                Region::Low
            }
        } else if y >= self.high {
            Region::High
        } else if y <= self.low {
            Region::Low
        } else {
            Region::Band
        }
    }

    /// The instants strictly inside the segment `(x0, y0)`–`(x1, y1)` at
    /// which the linear segment crosses a threshold, in time order.
    fn crossings(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<f64> {
        let levels: &[f64] = if self.single {
            &[self.high][..]
        } else if y1 >= y0 {
            &[self.low, self.high][..]
        } else {
            &[self.high, self.low][..]
        };
        let (lo, hi) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        levels
            .iter()
            .filter(|&&level| lo < level && level < hi)
            .map(|&level| {
                let fraction = (level - y0) / (y1 - y0);
                (x0 + fraction * (x1 - x0)).clamp(x0, x1)
            })
            .collect()
    }
}

/// The level changes of a converted signal, in time order, with the level
/// from each.
struct Conversion {
    thresholds: Thresholds,
    changes: Vec<(f64, Level)>,
    level: Level,
    /// When the signal entered the band holding a level, while it still may.
    band_since: Option<f64>,
}

impl Conversion {
    /// Hold `level` from `t`. A change at the instant of the last one
    /// replaces it, so a signal that passes several thresholds at once
    /// changes once, to where it settled.
    fn push(&mut self, t: f64, level: Level) {
        if self.changes.last().is_some_and(|last| last.0 == t) {
            self.changes.pop();
        }
        if self.changes.last().is_none_or(|last| last.1 != level) {
            self.changes.push((t, level));
        }
        self.level = level;
    }

    /// The signal has stayed in the band until `t`: once that is longer than
    /// the time to X, the held level gives way to X where the limit ran out.
    fn expire(&mut self, t: f64) {
        if let Some(since) = self.band_since {
            let limit = since + self.thresholds.time_to_x;
            if t > limit {
                self.push(limit, Level::Unknown);
                self.band_since = None;
            }
        }
    }

    fn enter(&mut self, t: f64, region: Region) {
        self.expire(t);
        match region {
            Region::High | Region::Low => {
                let level = if region == Region::High {
                    Level::High
                } else {
                    Level::Low
                };
                if self.level != level {
                    self.push(t, level);
                }
                self.band_since = None;
            }
            Region::Band => {
                if self.level != Level::Unknown && self.band_since.is_none() {
                    self.band_since = Some(t);
                }
            }
        }
    }

    /// Start, or restart after a hole, at a finite sample.
    fn start(&mut self, t: f64, y: f64) {
        let level = match self.thresholds.region(y) {
            Region::High => Level::High,
            Region::Low => Level::Low,
            Region::Band => Level::Unknown,
        };
        self.band_since = None;
        self.push(t, level);
    }
}

/// `digital(w, vth[, vtl[, tx]])` — see the module documentation.
pub(super) fn digital(args: Vec<RealValue>) -> Result<RealValue, EvaluationError> {
    const NAME: &str = "digital";
    super::check_arg_range(NAME, &args, 2, 4)?;
    let (x, y) = series_arg(NAME, &args[0])?;
    forward_domain(NAME, x)?;
    let high = scalar_arg(NAME, "threshold", &args[1])?;
    let low = args
        .get(2)
        .map(|value| scalar_arg(NAME, "low threshold", value))
        .transpose()?;
    let time_to_x = args
        .get(3)
        .map(|value| scalar_arg(NAME, "time to X", value))
        .transpose()?;
    if !high.is_finite() || low.is_some_and(|low| !low.is_finite()) {
        return Err(EvaluationError::MathError(
            "digital needs finite thresholds".to_owned(),
        ));
    }
    if let Some(low) = low
        && low >= high
    {
        return Err(EvaluationError::MathError(format!(
            "digital needs its low threshold below its high one, but {low} is not below {high}"
        )));
    }
    if let Some(time_to_x) = time_to_x
        && (time_to_x.is_nan() || time_to_x < 0.0)
    {
        return Err(EvaluationError::MathError(format!(
            "digital needs a time to X of zero or more, not {time_to_x}"
        )));
    }
    let thresholds = Thresholds {
        high,
        low: low.unwrap_or(high),
        single: low.is_none(),
        time_to_x: time_to_x.unwrap_or(f64::INFINITY),
    };
    let (times, levels) = convert(thresholds, x, y)?;
    let end = *x.last().expect("series_arg refuses an empty waveform");
    Ok(step_waveform(&times, &levels, end))
}

/// The level changes of `y` over `x` read through `thresholds`.
fn convert(
    thresholds: Thresholds,
    x: &[f64],
    y: &[f64],
) -> Result<(Vec<f64>, Vec<Level>), EvaluationError> {
    let mut conversion = Conversion {
        thresholds,
        changes: Vec::new(),
        level: Level::Unknown,
        band_since: None,
    };
    let mut previous: Option<(f64, f64)> = None;
    for (&xi, &yi) in x.iter().zip(y) {
        if !yi.is_finite() {
            // A hole: nothing is known from this sample to the next value.
            if previous.is_some() || conversion.changes.is_empty() {
                conversion.expire(xi);
                conversion.band_since = None;
                conversion.push(xi, Level::Unknown);
            }
            previous = None;
            continue;
        }
        let Some((x0, y0)) = previous else {
            conversion.start(xi, yi);
            previous = Some((xi, yi));
            continue;
        };
        if xi == x0 {
            // A vertical step: the value jumps at this instant.
            conversion.enter(xi, thresholds.region(yi));
        } else {
            // Each piece between crossings lies in one region, which its
            // midpoint names; it starts at its first instant. A sample that
            // sits exactly on a threshold is classified by where the signal
            // goes next, which is where its level changes.
            let mut cuts = vec![x0];
            cuts.extend(thresholds.crossings(x0, y0, xi, yi));
            cuts.push(xi);
            for piece in cuts.windows(2) {
                let (from, to) = (piece[0], piece[1]);
                if to > from {
                    let middle = 0.5 * (from + to);
                    let value = y0 + (yi - y0) * ((middle - x0) / (xi - x0));
                    conversion.enter(from, thresholds.region(value));
                }
            }
        }
        previous = Some((xi, yi));
    }
    if let Some(&end) = x.last() {
        conversion.expire(end);
    }
    if conversion.changes.is_empty() {
        return Err(EvaluationError::MathError(
            "digital needs a waveform with a finite sample".to_owned(),
        ));
    }
    Ok(conversion.changes.into_iter().unzip())
}

/// Levels held from each change instant, drawn horizontal then vertical,
/// through `end`.
fn step_waveform(times: &[f64], levels: &[Level], end: f64) -> RealValue {
    let mut x = Vec::with_capacity(times.len() * 2 + 1);
    let mut y = Vec::with_capacity(times.len() * 2 + 1);
    for (index, (&t, level)) in times.iter().zip(levels).enumerate() {
        if index > 0 {
            x.push(t);
            y.push(levels[index - 1].value());
        }
        x.push(t);
        y.push(level.value());
    }
    if let (Some(&last), Some(level)) = (times.last(), levels.last())
        && end > last
    {
        x.push(end);
        y.push(level.value());
    }
    RealValue::create_waveform(x, y)
}

/// A bus or logic net's exact event history, as `analog()` reads it.
#[derive(Debug, Clone, PartialEq)]
pub struct EventWord {
    /// Word-change instants, non-decreasing.
    pub times: Vec<f64>,
    /// The bits held from each instant, MSB first; `None` for a bit that is
    /// unknown or high impedance.
    pub words: Vec<Vec<Option<bool>>>,
    /// Whether the word reads as a two's complement integer.
    pub signed: bool,
    /// The last instant of the analysis, through which the last word holds.
    pub end: f64,
}

/// The integer a word denotes, or `None` when a bit is not 0 or 1.
fn word_value(bits: &[Option<bool>], signed: bool) -> Option<f64> {
    let mut value = 0.0_f64;
    for bit in bits {
        value = value.mul_add(2.0, f64::from(u8::from((*bit)?)));
    }
    if signed && bits.first().copied().flatten() == Some(true) {
        value -= 2_f64.powi(i32::try_from(bits.len()).unwrap_or(i32::MAX));
    }
    Some(value)
}

/// `analog(bus)` — the word's integer value as a staircase through the
/// analysis' end, with a hole where a word denotes no integer.
pub fn analog_staircase(word: &EventWord) -> Result<RealValue, EvaluationError> {
    if word.times.is_empty() || word.times.len() != word.words.len() {
        return Err(EvaluationError::MathError(
            "analog needs a bus or logic net with events".to_owned(),
        ));
    }
    let mut x = Vec::with_capacity(word.times.len() * 2 + 1);
    let mut y = Vec::with_capacity(word.times.len() * 2 + 1);
    let mut held: Option<f64> = None;
    for (&t, bits) in word.times.iter().zip(&word.words) {
        let value = word_value(bits, word.signed).unwrap_or(f64::NAN);
        if let Some(previous) = held {
            x.push(t);
            y.push(previous);
        }
        x.push(t);
        y.push(value);
        held = Some(value);
    }
    let last = *word.times.last().expect("checked non-empty");
    if word.end > last {
        x.push(word.end);
        y.push(held.unwrap_or(f64::NAN));
    }
    Ok(RealValue::create_waveform(x, y))
}

/// `analog(w)` of a value that is already a waveform reads as itself.
pub(super) fn analog(args: Vec<RealValue>) -> Result<RealValue, EvaluationError> {
    super::check_arg_count("analog", &args, 1)?;
    let mut args = args;
    match args.pop().expect("one argument") {
        RealValue::Scalar(_) => Err(EvaluationError::TypeMismatch(
            "analog draws a bus, a logic net or a waveform, not a scalar".to_owned(),
        )),
        waveform => Ok(waveform),
    }
}

#[cfg(test)]
mod tests;
