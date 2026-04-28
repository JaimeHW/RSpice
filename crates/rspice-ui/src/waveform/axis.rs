//! Axis Rendering and SI Prefix Formatting
//!
//! Commercial-grade axis rendering with automatic SI prefix scaling,
//! smart tick placement, and professional label formatting.
//!
//! # Features
//!
//! - Automatic SI prefix selection (fs, ps, ns, µs, ms, s for time)
//! - Smart tick intervals (1-2-5 sequence)
//! - Decimal precision management
//! - Grid line generation

// =============================================================================
// SI Prefix Formatting
// =============================================================================

/// SI prefix data: (threshold, scale_factor, prefix_string)
/// MUST be sorted by threshold from smallest to largest for reverse iteration to work
const SI_PREFIXES: [(f64, f64, &str); 17] = [
    (1e-24, 1e24, "y"), // yocto
    (1e-21, 1e21, "z"), // zepto
    (1e-18, 1e18, "a"), // atto
    (1e-15, 1e15, "f"), // femto
    (1e-12, 1e12, "p"), // pico
    (1e-9, 1e9, "n"),   // nano
    (1e-6, 1e6, "µ"),   // micro
    (1e-3, 1e3, "m"),   // milli
    (1e0, 1e0, ""),     // (base)
    (1e3, 1e-3, "k"),   // kilo
    (1e6, 1e-6, "M"),   // mega
    (1e9, 1e-9, "G"),   // giga
    (1e12, 1e-12, "T"), // tera
    (1e15, 1e-15, "P"), // peta
    (1e18, 1e-18, "E"), // exa
    (1e21, 1e-21, "Z"), // zetta
    (1e24, 1e-24, "Y"), // yotta
];

/// Get SI prefix and scale factor for a value
///
/// Returns (scale_factor, prefix_string) where:
/// - scale_factor: multiply the value by this to get the display value
/// - prefix_string: the SI prefix to append to the unit
///
/// # Example
/// ```ignore
/// let (scale, prefix) = si_prefix_for_value(0.000001);
/// // scale = 1e6, prefix = "µ"
/// // 0.000001 * 1e6 = 1.0 µ
/// ```
pub fn si_prefix_for_value(value: f64) -> (f64, &'static str) {
    let abs_value = value.abs();

    if abs_value == 0.0 {
        return (1.0, "");
    }

    // Find the appropriate SI prefix
    // We want the largest prefix where abs_value >= threshold
    for &(threshold, scale, prefix) in SI_PREFIXES.iter().rev() {
        if abs_value >= threshold {
            return (scale, prefix);
        }
    }

    // Fallback to base
    (1.0, "")
}

/// Get SI prefix for a range of values
///
/// Determines the best prefix for displaying a range, using the
/// magnitude of the larger bound.
pub fn si_prefix_for_range(min: f64, max: f64) -> (f64, &'static str) {
    let abs_max = max.abs().max(min.abs());
    si_prefix_for_value(abs_max)
}

/// Format a value with automatic SI prefix
///
/// # Arguments
/// * `value` - The value to format
/// * `unit` - The base unit (e.g., "s" for seconds, "V" for volts)
/// * `precision` - Number of decimal places
pub fn format_with_si_prefix(value: f64, unit: &str, precision: usize) -> String {
    let (scale, prefix) = si_prefix_for_value(value);
    let scaled = value * scale;
    format!("{:.prec$} {}{}", scaled, prefix, unit, prec = precision)
}

/// Format a value for axis labels (compact form)
///
/// This uses a compact format suitable for axis tick labels,
/// with minimal decimal places and no unit suffix.
pub fn format_axis_value(value: f64, scale: f64, precision: usize) -> String {
    let scaled = value * scale;
    if precision == 0 {
        format!("{:.0}", scaled)
    } else {
        format!("{:.prec$}", scaled, prec = precision)
    }
}

// =============================================================================
// Tick Generation
// =============================================================================

/// Standard tick intervals following the 1-2-5 sequence
///
/// This sequence is used by professional instruments and plotting software
/// because it provides "nice" round numbers at any scale.
const TICK_SEQUENCE: [f64; 3] = [1.0, 2.0, 5.0];

/// Tick specification for an axis
#[derive(Debug, Clone, PartialEq)]
pub struct TickSpec {
    /// Major tick positions (in data coordinates)
    pub major_ticks: Vec<f64>,
    /// Minor tick positions (between major ticks)
    pub minor_ticks: Vec<f64>,
    /// The interval between major ticks
    pub major_interval: f64,
    /// SI scale factor for labels
    pub scale: f64,
    /// SI prefix string
    pub prefix: &'static str,
    /// Suggested decimal precision for labels
    pub precision: usize,
}

impl Default for TickSpec {
    fn default() -> Self {
        Self {
            major_ticks: Vec::new(),
            minor_ticks: Vec::new(),
            major_interval: 1.0,
            scale: 1.0,
            prefix: "",
            precision: 0,
        }
    }
}

/// Calculate optimal tick marks for an axis
///
/// This implements the commercial-standard tick placement algorithm:
/// 1. Choose interval from 1-2-5 sequence that gives ~5-10 major ticks
/// 2. Round start/end to interval boundaries
/// 3. Generate major and minor ticks
///
/// # Arguments
/// * `min` - Minimum data value
/// * `max` - Maximum data value
/// * `target_major_ticks` - Desired number of major tick marks (typically 5-10)
pub fn calculate_ticks(min: f64, max: f64, target_major_ticks: usize) -> TickSpec {
    if min >= max || !min.is_finite() || !max.is_finite() {
        return TickSpec::default();
    }

    let range = max - min;
    let target_interval = range / target_major_ticks.max(1) as f64;

    // Find the magnitude (power of 10)
    let magnitude = 10.0_f64.powf(target_interval.log10().floor());

    // Find the best interval from 1-2-5 sequence
    let mut best_interval = magnitude;
    let mut best_diff = f64::MAX;

    for &multiplier in &TICK_SEQUENCE {
        let interval = magnitude * multiplier;
        let diff = (interval - target_interval).abs();
        if diff < best_diff {
            best_diff = diff;
            best_interval = interval;
        }

        // Also try 10x multiplier (next decade)
        let interval_10x = magnitude * multiplier * 10.0;
        let diff_10x = (interval_10x - target_interval).abs();
        if diff_10x < best_diff {
            best_diff = diff_10x;
            best_interval = interval_10x;
        }
    }

    // Round min/max to interval boundaries
    let tick_start = (min / best_interval).floor() * best_interval;
    let tick_end = (max / best_interval).ceil() * best_interval;

    // Generate major ticks
    let mut major_ticks = Vec::new();
    let mut tick = tick_start;
    let epsilon = best_interval * 1e-9; // For floating point comparison

    // Safety limit: prevent runaway memory allocation from extreme zoom
    const MAX_MAJOR_TICKS: usize = 50;

    while tick <= tick_end + epsilon && major_ticks.len() < MAX_MAJOR_TICKS {
        if tick >= min - epsilon && tick <= max + epsilon {
            major_ticks.push(tick);
        }
        tick += best_interval;
    }

    // Generate minor ticks (4 subdivisions between major ticks)
    let minor_interval = best_interval / 5.0;
    let mut minor_ticks = Vec::new();

    // Safety limit: prevent runaway memory allocation from extreme zoom
    const MAX_MINOR_TICKS: usize = 250;

    tick = tick_start;
    while tick <= tick_end + epsilon && minor_ticks.len() < MAX_MINOR_TICKS {
        for i in 1..5 {
            if minor_ticks.len() >= MAX_MINOR_TICKS {
                break;
            }
            let minor = tick + i as f64 * minor_interval;
            if minor > min - epsilon && minor < max + epsilon {
                // Don't add minor tick at major tick position
                let at_major = major_ticks.iter().any(|&m| (m - minor).abs() < epsilon);
                if !at_major {
                    minor_ticks.push(minor);
                }
            }
        }
        tick += best_interval;
    }

    // Determine SI prefix and precision based on the absolute values
    let (scale, prefix) = si_prefix_for_range(min, max);

    // Calculate precision needed: enough to distinguish between ticks
    let scaled_interval = best_interval * scale;
    let precision = calculate_precision(scaled_interval);

    TickSpec {
        major_ticks,
        minor_ticks,
        major_interval: best_interval,
        scale,
        prefix,
        precision,
    }
}

/// Calculate decimal precision needed for a given interval
fn calculate_precision(interval: f64) -> usize {
    if interval >= 1.0 {
        0
    } else if interval >= 0.1 {
        1
    } else if interval >= 0.01 {
        2
    } else if interval >= 0.001 {
        3
    } else {
        4
    }
}

// =============================================================================
// Axis Labels
// =============================================================================

/// Generate formatted labels for tick positions
pub fn generate_tick_labels(ticks: &[f64], scale: f64, precision: usize) -> Vec<String> {
    ticks
        .iter()
        .map(|&tick| format_axis_value(tick, scale, precision))
        .collect()
}

/// Get unit string with SI prefix for axis title
pub fn format_axis_unit(base_unit: &str, prefix: &str) -> String {
    format!("{}{}", prefix, base_unit)
}

// =============================================================================
// Grid Lines
// =============================================================================

/// Grid line type for rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridLineType {
    /// Major grid line (darker, at major tick positions)
    Major,
    /// Minor grid line (lighter, at minor tick positions)
    Minor,
}

/// Grid line specification
#[derive(Debug, Clone)]
pub struct GridLine {
    /// Position in data coordinates
    pub position: f64,
    /// Type of grid line
    pub line_type: GridLineType,
}

/// Generate grid lines for an axis
pub fn generate_grid_lines(ticks: &TickSpec) -> Vec<GridLine> {
    let mut lines = Vec::with_capacity(ticks.major_ticks.len() + ticks.minor_ticks.len());

    for &pos in &ticks.major_ticks {
        lines.push(GridLine {
            position: pos,
            line_type: GridLineType::Major,
        });
    }

    for &pos in &ticks.minor_ticks {
        lines.push(GridLine {
            position: pos,
            line_type: GridLineType::Minor,
        });
    }

    // Sort by position for consistent rendering order
    lines.sort_by(|a, b| {
        a.position
            .partial_cmp(&b.position)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    lines
}

// =============================================================================
// Time Formatting
// =============================================================================

/// Format a time value for display (specialized for waveform X-axis)
pub fn format_time(seconds: f64) -> String {
    format_with_si_prefix(seconds, "s", 3)
}

/// Format a time delta for cursor readouts
pub fn format_time_delta(seconds: f64) -> String {
    let (scale, prefix) = si_prefix_for_value(seconds);
    let scaled = seconds * scale;
    format!("Δt = {:.3} {}s", scaled, prefix)
}

/// Format a frequency value
pub fn format_frequency(hertz: f64) -> String {
    format_with_si_prefix(hertz, "Hz", 3)
}

/// Calculate frequency from period
pub fn period_to_frequency(period: f64) -> f64 {
    if period > 0.0 { 1.0 / period } else { 0.0 }
}

// =============================================================================
// Voltage/Current Formatting
// =============================================================================

/// Format a voltage value for display
pub fn format_voltage(volts: f64) -> String {
    format_with_si_prefix(volts, "V", 3)
}

/// Format a voltage delta for cursor readouts
pub fn format_voltage_delta(volts: f64) -> String {
    let (scale, prefix) = si_prefix_for_value(volts);
    let scaled = volts * scale;
    format!("Δv = {:.3} {}V", scaled, prefix)
}

/// Format a current value for display
pub fn format_current(amps: f64) -> String {
    format_with_si_prefix(amps, "A", 3)
}

// =============================================================================
// Tests
// =============================================================================

