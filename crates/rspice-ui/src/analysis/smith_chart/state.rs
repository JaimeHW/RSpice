//! Smith Chart State Management
//!
//! Viewer state for the Smith chart including display mode, reference impedance,
//! data traces, and marker state.

use super::complex::Complex;
use super::impedance::{Admittance, Impedance, Z0_DEFAULT};

// =============================================================================
// Chart Mode
// =============================================================================

/// Smith chart display mode
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SmithChartMode {
    /// Impedance (Z) chart - standard
    #[default]
    Impedance,
    /// Admittance (Y) chart - rotated 180°
    Admittance,
    /// Combined Z and Y overlaid
    Combined,
}

impl SmithChartMode {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Impedance => "Impedance (Z)",
            Self::Admittance => "Admittance (Y)",
            Self::Combined => "Combined (Z+Y)",
        }
    }

    /// Get all modes
    pub fn all() -> &'static [SmithChartMode] {
        &[Self::Impedance, Self::Admittance, Self::Combined]
    }
}

// =============================================================================
// S-Parameter Data
// =============================================================================

/// S-parameter data point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SParamPoint {
    /// Frequency in Hz
    pub frequency: f64,
    /// Complex S-parameter value
    pub s: Complex,
}

impl SParamPoint {
    /// Create new S-parameter point
    pub fn new(frequency: f64, s: Complex) -> Self {
        Self { frequency, s }
    }

    /// Convert to impedance given Z0
    pub fn to_impedance(&self, z0: f64) -> Impedance {
        Impedance::from_gamma(self.s, z0)
    }

    /// Get magnitude in dB
    pub fn magnitude_db(&self) -> f64 {
        let mag = self.s.magnitude();
        if mag < 1e-15 {
            -300.0
        } else {
            20.0 * mag.log10()
        }
    }

    /// Get phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.s.phase_deg()
    }
}

/// S-parameter trace for Smith chart overlay
#[derive(Debug, Clone)]
pub struct SParamTrace {
    /// Trace name (e.g., "S11", "S22")
    pub name: String,
    /// Data points
    pub points: Vec<SParamPoint>,
    /// Display color (egui Color32 as u32)
    pub color: u32,
    /// Visibility
    pub visible: bool,
}

impl SParamTrace {
    /// Create new S-parameter trace
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
            color: 0xFF3B82F6, // Blue
            visible: true,
        }
    }

    /// Add data point
    pub fn add_point(&mut self, frequency: f64, s: Complex) {
        self.points.push(SParamPoint::new(frequency, s));
    }

    /// Get frequency range
    pub fn frequency_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for point in &self.points {
            min = min.min(point.frequency);
            max = max.max(point.frequency);
        }
        Some((min, max))
    }

    /// Find point nearest to frequency
    pub fn find_by_frequency(&self, freq: f64) -> Option<&SParamPoint> {
        self.points.iter().min_by(|a, b| {
            (a.frequency - freq)
                .abs()
                .partial_cmp(&(b.frequency - freq).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

// =============================================================================
// Marker State
// =============================================================================

/// Marker on Smith chart
#[derive(Debug, Clone, Copy, Default)]
pub struct SmithChartMarker {
    /// Marker is active
    pub active: bool,
    /// Frequency in Hz (for S-param traces)
    pub frequency: f64,
    /// X coordinate on Smith chart (-1 to 1)
    pub x: f64,
    /// Y coordinate on Smith chart (-1 to 1)
    pub y: f64,
}

impl SmithChartMarker {
    /// Get impedance at marker position
    pub fn impedance(&self, z0: f64) -> Impedance {
        let gamma = Complex::new(self.x, self.y);
        Impedance::from_gamma(gamma, z0)
    }

    /// Get admittance at marker position
    pub fn admittance(&self, z0: f64) -> Admittance {
        self.impedance(z0).to_admittance()
    }

    /// Get VSWR at marker position
    pub fn vswr(&self) -> f64 {
        let gamma_mag = Complex::new(self.x, self.y).magnitude();
        if gamma_mag >= 1.0 {
            f64::INFINITY
        } else {
            (1.0 + gamma_mag) / (1.0 - gamma_mag)
        }
    }

    /// Get return loss in dB
    pub fn return_loss_db(&self) -> f64 {
        let gamma_mag = Complex::new(self.x, self.y).magnitude();
        if gamma_mag < 1e-10 {
            f64::INFINITY
        } else {
            -20.0 * gamma_mag.log10()
        }
    }
}

// =============================================================================
// VSWR Circle
// =============================================================================

/// VSWR circle overlay
#[derive(Debug, Clone, Copy)]
pub struct VswrCircle {
    /// VSWR value
    pub vswr: f64,
    /// Display this circle
    pub visible: bool,
}

impl VswrCircle {
    /// Create new VSWR circle
    pub fn new(vswr: f64) -> Self {
        Self {
            vswr,
            visible: true,
        }
    }

    /// Calculate radius on Smith chart
    /// VSWR = (1 + |Γ|) / (1 - |Γ|)
    /// |Γ| = (VSWR - 1) / (VSWR + 1)
    pub fn radius(&self) -> f64 {
        if self.vswr <= 1.0 {
            0.0
        } else {
            (self.vswr - 1.0) / (self.vswr + 1.0)
        }
    }
}

// =============================================================================
// Smith Chart State
// =============================================================================

/// Complete Smith chart viewer state
#[derive(Debug, Clone)]
pub struct SmithChartState {
    /// Display mode (Z, Y, or combined)
    pub mode: SmithChartMode,
    /// Reference impedance (default 50Ω)
    pub z0: f64,
    /// S-parameter traces
    pub traces: Vec<SParamTrace>,
    /// Marker state
    pub marker: SmithChartMarker,
    /// VSWR circles to display
    pub vswr_circles: Vec<VswrCircle>,
    /// Show constant R circles
    pub show_r_circles: bool,
    /// Show constant X circles
    pub show_x_circles: bool,
    /// Show grid
    pub show_grid: bool,
    /// Show unit circle
    pub show_unit_circle: bool,
    /// Normalized impedance display
    pub normalized: bool,
}

impl Default for SmithChartState {
    fn default() -> Self {
        Self {
            mode: SmithChartMode::Impedance,
            z0: Z0_DEFAULT,
            traces: Vec::new(),
            marker: SmithChartMarker::default(),
            vswr_circles: vec![
                VswrCircle::new(1.5),
                VswrCircle::new(2.0),
                VswrCircle::new(3.0),
            ],
            show_r_circles: true,
            show_x_circles: true,
            show_grid: true,
            show_unit_circle: true,
            normalized: true,
        }
    }
}

impl SmithChartState {
    /// Create new state with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set reference impedance
    pub fn set_z0(&mut self, z0: f64) {
        if z0 > 0.0 {
            self.z0 = z0;
        }
    }

    /// Add S-parameter trace
    pub fn add_trace(&mut self, trace: SParamTrace) {
        self.traces.push(trace);
    }

    /// Clear all traces
    pub fn clear_traces(&mut self) {
        self.traces.clear();
    }

    /// Load from S-parameter simulation data
    pub fn load_sparam_data(
        &mut self,
        name: &str,
        frequencies: &[f64],
        s_real: &[f64],
        s_imag: &[f64],
    ) {
        let mut trace = SParamTrace::new(name);
        let n = frequencies.len().min(s_real.len()).min(s_imag.len());

        for i in 0..n {
            trace.add_point(frequencies[i], Complex::new(s_real[i], s_imag[i]));
        }

        self.traces.push(trace);
    }

    /// Get visible VSWR circles
    pub fn visible_vswr_circles(&self) -> Vec<&VswrCircle> {
        self.vswr_circles.iter().filter(|c| c.visible).collect()
    }

    /// Place marker at screen coordinates
    pub fn place_marker(&mut self, x: f64, y: f64) {
        // Clamp to unit circle
        let r = (x * x + y * y).sqrt();
        if r > 1.0 {
            self.marker.x = x / r;
            self.marker.y = y / r;
        } else {
            self.marker.x = x;
            self.marker.y = y;
        }
        self.marker.active = true;
    }

    /// Clear marker
    pub fn clear_marker(&mut self) {
        self.marker.active = false;
    }

    /// Get marker impedance string for display
    pub fn marker_impedance_string(&self) -> String {
        if !self.marker.active {
            return String::new();
        }

        let z = self.marker.impedance(self.z0);
        if self.normalized {
            let zn = z.normalize(self.z0);
            format!("Z = {:.3}+j{:.3}", zn.r, zn.x)
        } else {
            format!("{}", z)
        }
    }

    /// Get marker readout for display
    pub fn marker_readout(&self) -> MarkerReadout {
        if !self.marker.active {
            return MarkerReadout::default();
        }

        let z = self.marker.impedance(self.z0);
        let y = z.to_admittance();
        let zn = z.normalize(self.z0);

        MarkerReadout {
            impedance: z,
            impedance_normalized: zn,
            admittance: y,
            vswr: self.marker.vswr(),
            return_loss_db: self.marker.return_loss_db(),
            gamma: Complex::new(self.marker.x, self.marker.y),
        }
    }
}

/// Marker readout data for display
#[derive(Debug, Clone, Default)]
pub struct MarkerReadout {
    /// Actual impedance
    pub impedance: Impedance,
    /// Normalized impedance
    pub impedance_normalized: Impedance,
    /// Admittance
    pub admittance: Admittance,
    /// VSWR
    pub vswr: f64,
    /// Return loss in dB
    pub return_loss_db: f64,
    /// Reflection coefficient
    pub gamma: Complex,
}

// =============================================================================
// Smith Chart Circle Calculations
// =============================================================================

/// Calculate center and radius for constant-R circle on Smith chart
///
/// For normalized r, the circle has:
/// - Center at (r/(r+1), 0)
/// - Radius = 1/(r+1)
pub fn constant_r_circle(r_normalized: f64) -> (f64, f64, f64) {
    if r_normalized < 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let center_x = r_normalized / (r_normalized + 1.0);
    let center_y = 0.0;
    let radius = 1.0 / (r_normalized + 1.0);
    (center_x, center_y, radius)
}

/// Calculate center and radius for constant-X circle on Smith chart
///
/// For normalized x, the circle has:
/// - Center at (1, 1/x) for x > 0
/// - Center at (1, 1/x) for x < 0
/// - Radius = 1/|x|
pub fn constant_x_circle(x_normalized: f64) -> Option<(f64, f64, f64)> {
    if x_normalized.abs() < 1e-10 {
        return None; // X=0 is a line, not a circle
    }
    let center_x = 1.0;
    let center_y = 1.0 / x_normalized;
    let radius = 1.0 / x_normalized.abs();
    Some((center_x, center_y, radius))
}

/// Standard R values for grid circles
pub fn standard_r_values() -> &'static [f64] {
    &[0.0, 0.2, 0.5, 1.0, 2.0, 5.0]
}

/// Standard X values for grid circles
pub fn standard_x_values() -> &'static [f64] {
    &[-5.0, -2.0, -1.0, -0.5, -0.2, 0.2, 0.5, 1.0, 2.0, 5.0]
}

// =============================================================================
// Tests
// =============================================================================
