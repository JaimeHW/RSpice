//! Formatting utilities for SI prefixes and engineering notation.
//!
//! Provides consistent formatting for time, voltage, and frequency values
//! throughout the application.

/// Format time value with appropriate SI prefix.
///
/// Automatically selects the best unit (ps, ns, µs, ms, s) based on magnitude.
pub fn format_time(t: f64) -> String {
    let abs_t = t.abs();
    if abs_t < 1e-9 {
        format!("{:.2}ps", t * 1e12)
    } else if abs_t < 1e-6 {
        format!("{:.2}ns", t * 1e9)
    } else if abs_t < 1e-3 {
        format!("{:.2}µs", t * 1e6)
    } else if abs_t < 1.0 {
        format!("{:.2}ms", t * 1e3)
    } else {
        format!("{:.2}s", t)
    }
}

/// Format voltage value with appropriate SI prefix.
///
/// Automatically selects the best unit (nV, µV, mV, V, kV) based on magnitude.
pub fn format_voltage(v: f64) -> String {
    let abs_v = v.abs();
    if abs_v >= 1e3 {
        format!("{:.3}kV", v / 1e3)
    } else if abs_v >= 1.0 {
        format!("{:.3}V", v)
    } else if abs_v >= 1e-3 {
        format!("{:.3}mV", v * 1e3)
    } else if abs_v >= 1e-6 {
        format!("{:.3}µV", v * 1e6)
    } else {
        format!("{:.3}nV", v * 1e9)
    }
}

/// Format frequency value with appropriate SI prefix.
///
/// Automatically selects the best unit (mHz, Hz, kHz, MHz, GHz) based on magnitude.
pub fn format_frequency(f: f64) -> String {
    if f >= 1e9 {
        format!("{:.2}GHz", f / 1e9)
    } else if f >= 1e6 {
        format!("{:.2}MHz", f / 1e6)
    } else if f >= 1e3 {
        format!("{:.2}kHz", f / 1e3)
    } else if f >= 1.0 {
        format!("{:.2}Hz", f)
    } else if f >= 1e-3 {
        format!("{:.2}mHz", f * 1e3)
    } else {
        format!("{:.4}Hz", f)
    }
}

/// Parse a hex color string (#RRGGBB) to RGBA floats [0.0, 1.0].
///
/// Returns white [1.0, 1.0, 1.0, 1.0] if parsing fails.
pub fn parse_hex_color(hex: &str) -> [f32; 4] {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255) as f32 / 255.0;
        [r, g, b, 1.0]
    } else {
        [1.0, 1.0, 1.0, 1.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(1.0), "1.00s");
        assert_eq!(format_time(0.001), "1.00ms");
        assert_eq!(format_time(0.000001), "1.00µs");
        assert_eq!(format_time(0.000000001), "1.00ns");
    }

    #[test]
    fn test_format_voltage() {
        assert_eq!(format_voltage(1.0), "1.000V");
        assert_eq!(format_voltage(0.001), "1.000mV");
        assert_eq!(format_voltage(1000.0), "1.000kV");
    }

    #[test]
    fn test_format_frequency() {
        assert_eq!(format_frequency(1.0), "1.00Hz");
        assert_eq!(format_frequency(1000.0), "1.00kHz");
        assert_eq!(format_frequency(1e6), "1.00MHz");
    }

    #[test]
    fn test_parse_hex_color() {
        let red = parse_hex_color("#ff0000");
        assert!((red[0] - 1.0).abs() < 0.01);
        assert!(red[1].abs() < 0.01);
        assert!(red[2].abs() < 0.01);
    }
}
