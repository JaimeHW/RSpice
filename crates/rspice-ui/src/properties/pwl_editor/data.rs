use crate::properties::{format_engineering_value, parse_engineering_value};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A single time-value point in a PWL waveform.
///
/// Represents one vertex of the piecewise linear function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PwlPoint {
    /// Time coordinate in seconds
    pub time: f64,
    /// Value (voltage or current) at this time
    pub value: f64,
}

impl PwlPoint {
    /// Create a new PWL point.
    pub fn new(time: f64, value: f64) -> Self {
        Self { time, value }
    }

    /// Create origin point (0, 0).
    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Format time with engineering notation.
    pub fn time_string(&self) -> String {
        format!("{} s", format_engineering_value(self.time))
    }

    /// Format value with engineering notation and unit.
    pub fn value_string(&self, unit: &str) -> String {
        format!("{} {}", format_engineering_value(self.value), unit)
    }

    /// Validate that time is non-negative.
    pub fn validate(&self) -> Result<(), PwlValidationError> {
        if self.time < 0.0 {
            return Err(PwlValidationError::NegativeTime(self.time));
        }
        if !self.time.is_finite() {
            return Err(PwlValidationError::InvalidTime(self.time));
        }
        if !self.value.is_finite() {
            return Err(PwlValidationError::InvalidValue(self.value));
        }
        Ok(())
    }
}

impl Default for PwlPoint {
    fn default() -> Self {
        Self::origin()
    }
}

/// Errors that can occur during PWL data validation.
#[derive(Debug, Clone, PartialEq)]
pub enum PwlValidationError {
    /// Time value is negative.
    NegativeTime(f64),
    /// Time value is not finite (NaN or Inf).
    InvalidTime(f64),
    /// Value is not finite (NaN or Inf).
    InvalidValue(f64),
    /// Time values are not monotonically increasing.
    NonMonotonicTime { index: usize, prev: f64, curr: f64 },
    /// Duplicate time value detected.
    DuplicateTime { index: usize, time: f64 },
    /// Parse error for time string.
    TimeParseError { index: usize, text: String },
    /// Parse error for value string.
    ValueParseError { index: usize, text: String },
    /// Empty data when at least one point is required.
    EmptyData,
}

impl fmt::Display for PwlValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeTime(t) => write!(f, "Time cannot be negative: {}", t),
            Self::InvalidTime(t) => write!(f, "Invalid time value: {}", t),
            Self::InvalidValue(v) => write!(f, "Invalid value: {}", v),
            Self::NonMonotonicTime { index, prev, curr } => {
                write!(
                    f,
                    "Time must be strictly increasing: point {} has t={} after t={}",
                    index + 1,
                    curr,
                    prev
                )
            }
            Self::DuplicateTime { index, time } => {
                write!(f, "Duplicate time at point {}: t={}", index + 1, time)
            }
            Self::TimeParseError { index, text } => {
                write!(f, "Cannot parse time at point {}: '{}'", index + 1, text)
            }
            Self::ValueParseError { index, text } => {
                write!(f, "Cannot parse value at point {}: '{}'", index + 1, text)
            }
            Self::EmptyData => write!(f, "PWL data cannot be empty"),
        }
    }
}

impl std::error::Error for PwlValidationError {}

/// Collection of PWL points with parsing, validation, and serialization.
///
/// Maintains points in time-sorted order and validates monotonicity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PwlData {
    /// Time-value points (sorted by time).
    points: Vec<PwlPoint>,
    /// Whether to repeat the waveform.
    pub repeat: bool,
    /// Time delay before waveform starts.
    pub delay: f64,
}

impl PwlData {
    /// Create empty PWL data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create PWL data with initial points.
    pub fn with_points(points: Vec<PwlPoint>) -> Self {
        let mut data = Self {
            points,
            repeat: false,
            delay: 0.0,
        };
        data.sort_by_time();
        data
    }

    /// Parse PWL data from space-separated string format.
    ///
    /// Format: "t1 v1 t2 v2 t3 v3 ..."
    ///
    /// Supports engineering notation (e.g., "0 0 1n 1 2n 0")
    pub fn parse(s: &str) -> Result<Self, PwlValidationError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(PwlValidationError::EmptyData);
        }

        let tokens: Vec<&str> = s.split_whitespace().collect();

        if !crate::utils::numeric::is_multiple_of(tokens.len(), 2) {
            return Err(PwlValidationError::ValueParseError {
                index: tokens.len() / 2,
                text: "Odd number of values - expected time-value pairs".to_string(),
            });
        }

        let mut points = Vec::with_capacity(tokens.len() / 2);

        for (i, chunk) in tokens.chunks(2).enumerate() {
            let time = parse_engineering_value(chunk[0]).map_err(|_| {
                PwlValidationError::TimeParseError {
                    index: i,
                    text: chunk[0].to_string(),
                }
            })?;

            let value = parse_engineering_value(chunk[1]).map_err(|_| {
                PwlValidationError::ValueParseError {
                    index: i,
                    text: chunk[1].to_string(),
                }
            })?;

            points.push(PwlPoint::new(time, value));
        }

        // Authored PWL rows are ordered data. Do not sort malformed input into
        // validity: the editor must report a descending or duplicate time at
        // the row where it was authored.
        let data = Self::with_ordered_points(points);
        data.validate()?;
        Ok(data)
    }

    /// Serialize to space-separated string format.
    ///
    /// Uses a shortest round-trip decimal representation. Compact engineering
    /// formatting is useful for presentation, but scaling and rounding it can
    /// change an untouched `f64` when the text is parsed again.
    pub fn serialize(&self) -> String {
        self.points
            .iter()
            .map(|p| {
                format!(
                    "{} {}",
                    format_spice_number_lossless(p.time),
                    format_spice_number_lossless(p.value)
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Construct PWL data without changing the authored row order.
    pub(super) fn with_ordered_points(points: Vec<PwlPoint>) -> Self {
        Self {
            points,
            repeat: false,
            delay: 0.0,
        }
    }

    /// Validate the PWL data.
    ///
    /// Checks:
    /// - All times are non-negative and finite
    /// - All values are finite
    /// - Times are strictly monotonically increasing
    pub fn validate(&self) -> Result<(), PwlValidationError> {
        if self.points.is_empty() {
            return Err(PwlValidationError::EmptyData);
        }
        for point in &self.points {
            point.validate()?;
        }

        for i in 1..self.points.len() {
            let prev = self.points[i - 1].time;
            let curr = self.points[i].time;

            if curr < prev {
                return Err(PwlValidationError::NonMonotonicTime {
                    index: i,
                    prev,
                    curr,
                });
            }
            if (curr - prev).abs() < 1e-18 {
                return Err(PwlValidationError::DuplicateTime {
                    index: i,
                    time: curr,
                });
            }
        }

        Ok(())
    }

    /// Get the number of points.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get points slice.
    pub fn points(&self) -> &[PwlPoint] {
        &self.points
    }

    /// Get mutable points slice.
    pub fn points_mut(&mut self) -> &mut Vec<PwlPoint> {
        &mut self.points
    }

    /// Add a new point (will be sorted by time).
    pub fn add_point(&mut self, point: PwlPoint) {
        self.points.push(point);
        self.sort_by_time();
    }

    /// Insert a point at the given index.
    pub fn insert_point(&mut self, index: usize, point: PwlPoint) {
        if index <= self.points.len() {
            self.points.insert(index, point);
        }
    }

    /// Remove a point at the given index.
    pub fn remove_point(&mut self, index: usize) -> Option<PwlPoint> {
        if index < self.points.len() {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    /// Update a point at the given index.
    pub fn update_point(&mut self, index: usize, point: PwlPoint) {
        if index < self.points.len() {
            self.points[index] = point;
        }
    }

    /// Sort points by time.
    pub fn sort_by_time(&mut self) {
        self.points.sort_by(|a, b| {
            a.time
                .partial_cmp(&b.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Clear all points.
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Get time range (min, max).
    pub fn time_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let min = self.points.first().map(|p| p.time).unwrap_or(0.0);
        let max = self.points.last().map(|p| p.time).unwrap_or(0.0);
        Some((min, max))
    }

    /// Get value range (min, max).
    pub fn value_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let min = self
            .points
            .iter()
            .map(|p| p.value)
            .fold(f64::INFINITY, f64::min);
        let max = self
            .points
            .iter()
            .map(|p| p.value)
            .fold(f64::NEG_INFINITY, f64::max);
        Some((min, max))
    }

    /// Interpolate value at a given time.
    pub fn interpolate(&self, t: f64) -> Option<f64> {
        if self.points.is_empty() {
            return None;
        }

        let t = t - self.delay;
        if t < 0.0 {
            return Some(self.points.first()?.value);
        }

        let t = if self.repeat {
            if let Some((_, max)) = self.time_range() {
                if max > 0.0 { t % max } else { t }
            } else {
                t
            }
        } else {
            t
        };

        if t <= self.points[0].time {
            return Some(self.points[0].value);
        }

        if t >= self.points.last()?.time {
            return Some(self.points.last()?.value);
        }

        for i in 1..self.points.len() {
            if t <= self.points[i].time {
                let p0 = &self.points[i - 1];
                let p1 = &self.points[i];
                let dt = p1.time - p0.time;
                if dt.abs() < 1e-18 {
                    return Some(p0.value);
                }
                let alpha = (t - p0.time) / dt;
                return Some(p0.value + alpha * (p1.value - p0.value));
            }
        }

        Some(self.points.last()?.value)
    }

    /// Generate standard pulse waveform.
    pub fn pulse(v_low: f64, v_high: f64, period: f64, duty: f64, rise: f64, fall: f64) -> Self {
        let pw = period * duty;
        let points = vec![
            PwlPoint::new(0.0, v_low),
            PwlPoint::new(rise, v_high),
            PwlPoint::new(rise + pw, v_high),
            PwlPoint::new(rise + pw + fall, v_low),
            PwlPoint::new(period, v_low),
        ];
        Self {
            points,
            repeat: true,
            delay: 0.0,
        }
    }

    /// Generate ramp waveform.
    pub fn ramp(v_start: f64, v_end: f64, t_rise: f64) -> Self {
        Self::with_points(vec![
            PwlPoint::new(0.0, v_start),
            PwlPoint::new(t_rise, v_end),
        ])
    }
}

/// Format a value with engineering notation for SPICE compatibility.
pub(super) fn format_engineering_for_spice(value: f64) -> String {
    let abs_value = value.abs();

    if abs_value == 0.0 {
        return "0".to_string();
    }

    let (scaled, suffix) = if abs_value >= 1e12 {
        (value / 1e12, "T")
    } else if abs_value >= 1e9 {
        (value / 1e9, "G")
    } else if abs_value >= 1e6 {
        (value / 1e6, "Meg")
    } else if abs_value >= 1e3 {
        (value / 1e3, "k")
    } else if abs_value >= 1.0 {
        (value, "")
    } else if abs_value >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "u")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_value >= 1e-12 {
        (value * 1e12, "p")
    } else if abs_value >= 1e-15 {
        (value * 1e15, "f")
    } else {
        (value * 1e18, "a")
    };

    let eps = 1e-9;
    let is_int = (scaled.round() - scaled).abs() < eps;

    if is_int {
        format!("{:.0}{}", scaled.round(), suffix)
    } else {
        let formatted = format!("{:.6}", scaled);
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
        format!("{}{}", trimmed, suffix)
    }
}

/// Format one finite SPICE number without losing any `f64` information.
///
/// Rust's float display uses the shortest decimal that round-trips to the
/// same binary value. Scientific notation is accepted by the SPICE quantity
/// parser, so no engineering-prefix rescaling is necessary here.
pub(super) fn format_spice_number_lossless(value: f64) -> String {
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_round_trips_high_precision_points_bit_exact() {
        let time = f64::from_bits(0x3ff0_0000_0000_0001);
        let value = f64::from_bits(0x3fd5_5555_5555_5555);
        let data = PwlData::with_points(vec![PwlPoint::origin(), PwlPoint::new(time, value)]);

        let reparsed = PwlData::parse(&data.serialize()).unwrap();

        assert_eq!(reparsed.points()[1].time.to_bits(), time.to_bits());
        assert_eq!(reparsed.points()[1].value.to_bits(), value.to_bits());
    }
}
