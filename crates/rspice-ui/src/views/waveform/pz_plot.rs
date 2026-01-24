//! Pole-Zero Plot Viewer
//!
//! Commercial-grade pole-zero visualization for stability and frequency response analysis.
//! Features:
//!
//! - Complex plane visualization
//! - Poles (×) and zeros (○) display
//! - Unit circle for discrete-time systems
//! - Stability region highlighting
//! - Interactive cursor with value readout
//! - Quality factor (Q) from pole locations
//! - Natural frequency and damping ratio
//! - S-domain and Z-domain support

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// =============================================================================
// Complex Number (reuse from smith_chart or local)
// =============================================================================

/// Complex number for pole-zero representation
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn from_polar(magnitude: f64, angle_rad: f64) -> Self {
        Self {
            re: magnitude * angle_rad.cos(),
            im: magnitude * angle_rad.sin(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn phase_deg(&self) -> f64 {
        self.phase() * 180.0 / PI
    }

    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Check if this is a real number (imag ≈ 0)
    pub fn is_real(&self) -> bool {
        self.im.abs() < 1e-12
    }

    /// Check if point is inside unit circle |z| < 1
    pub fn is_inside_unit_circle(&self) -> bool {
        self.magnitude() < 1.0
    }

    /// Check if point is in left half-plane (re < 0)
    pub fn is_in_lhp(&self) -> bool {
        self.re < 0.0
    }

    /// Check if point is in right half-plane (re > 0)
    pub fn is_in_rhp(&self) -> bool {
        self.re > 0.0
    }

    /// Distance to another point
    pub fn distance_to(&self, other: &Complex) -> f64 {
        let dr = self.re - other.re;
        let di = self.im - other.im;
        (dr * dr + di * di).sqrt()
    }
}

// =============================================================================
// Domain Types
// =============================================================================

/// Analysis domain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Domain {
    /// Continuous-time (s-domain)
    #[default]
    SDomain,
    /// Discrete-time (z-domain)
    ZDomain,
}

impl Domain {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::SDomain => "s-domain (Continuous)",
            Self::ZDomain => "z-domain (Discrete)",
        }
    }

    pub fn all() -> &'static [Domain] {
        &[Domain::SDomain, Domain::ZDomain]
    }

    /// Get stability criterion description
    pub fn stability_criterion(&self) -> &'static str {
        match self {
            Self::SDomain => "Stable: All poles in left half-plane (Re < 0)",
            Self::ZDomain => "Stable: All poles inside unit circle (|z| < 1)",
        }
    }
}

// =============================================================================
// Pole and Zero
// =============================================================================

/// Pole type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoleType {
    /// Real pole (no imaginary part)
    Real,
    /// Complex conjugate pair
    ComplexPair,
    /// Single complex pole (rare, usually pairs)
    Complex,
    /// Pole at origin
    Origin,
    /// Pole at infinity
    Infinity,
}

impl PoleType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Real => "Real",
            Self::ComplexPair => "Complex Pair",
            Self::Complex => "Complex",
            Self::Origin => "Origin",
            Self::Infinity => "Infinity",
        }
    }
}

/// A pole in the complex plane
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pole {
    /// Location in complex plane
    pub location: Complex,
    /// Multiplicity (order)
    pub multiplicity: u32,
    /// Pole type
    pub pole_type: PoleType,
    /// Label for display
    pub label: Option<String>,
}

impl Pole {
    pub fn new(re: f64, im: f64) -> Self {
        let location = Complex::new(re, im);
        let pole_type = if re.abs() < 1e-12 && im.abs() < 1e-12 {
            PoleType::Origin
        } else if im.abs() < 1e-12 {
            PoleType::Real
        } else {
            PoleType::Complex
        };

        Self {
            location,
            multiplicity: 1,
            pole_type,
            label: None,
        }
    }

    pub fn with_multiplicity(mut self, m: u32) -> Self {
        self.multiplicity = m;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Check if this pole is stable in s-domain
    pub fn is_stable_s(&self) -> bool {
        self.location.re < 0.0 || (self.location.re.abs() < 1e-12 && self.location.im.abs() < 1e-12)
    }

    /// Check if this pole is stable in z-domain
    pub fn is_stable_z(&self) -> bool {
        self.location.magnitude() < 1.0 + 1e-10
    }

    /// Check if marginally stable (on boundary)
    pub fn is_marginal_s(&self) -> bool {
        self.location.re.abs() < 1e-10 && self.location.im.abs() > 1e-10
    }

    /// Check if marginally stable in z-domain
    pub fn is_marginal_z(&self) -> bool {
        (self.location.magnitude() - 1.0).abs() < 1e-10
    }

    /// Get natural frequency (rad/s) for this pole
    pub fn natural_frequency(&self) -> f64 {
        self.location.magnitude()
    }

    /// Get damping ratio (ζ) for this pole
    pub fn damping_ratio(&self) -> f64 {
        let wn = self.natural_frequency();
        if wn < 1e-12 {
            return 1.0;
        }
        -self.location.re / wn
    }

    /// Get quality factor (Q) for this pole
    pub fn quality_factor(&self) -> f64 {
        let zeta = self.damping_ratio();
        if zeta < 1e-12 {
            return f64::INFINITY;
        }
        if zeta > 0.5 - 1e-10 {
            return 0.5 / zeta;
        }
        1.0 / (2.0 * zeta)
    }

    /// Get damped natural frequency (rad/s)
    pub fn damped_frequency(&self) -> f64 {
        self.location.im.abs()
    }

    /// Get time constant (s) for real pole
    pub fn time_constant(&self) -> Option<f64> {
        if self.location.re.abs() < 1e-12 {
            None
        } else {
            Some(-1.0 / self.location.re)
        }
    }

    /// Get settling time (2% criterion)
    pub fn settling_time(&self) -> Option<f64> {
        if self.location.re >= 0.0 {
            None // Unstable
        } else {
            Some(-4.0 / self.location.re)
        }
    }
}

/// A zero in the complex plane
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zero {
    /// Location in complex plane
    pub location: Complex,
    /// Multiplicity (order)
    pub multiplicity: u32,
    /// Label for display
    pub label: Option<String>,
}

impl Zero {
    pub fn new(re: f64, im: f64) -> Self {
        Self {
            location: Complex::new(re, im),
            multiplicity: 1,
            label: None,
        }
    }

    pub fn with_multiplicity(mut self, m: u32) -> Self {
        self.multiplicity = m;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Check if this is a right half-plane zero (non-minimum phase)
    pub fn is_rhp(&self) -> bool {
        self.location.re > 0.0
    }

    /// Check if this zero is on the real axis
    pub fn is_real(&self) -> bool {
        self.location.im.abs() < 1e-12
    }
}

// =============================================================================
// Pole-Zero Data
// =============================================================================

/// Complete pole-zero data for a system
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PoleZeroData {
    /// System name/identifier
    pub name: String,

    /// Analysis domain
    pub domain: Domain,

    /// List of poles
    pub poles: Vec<Pole>,

    /// List of zeros
    pub zeros: Vec<Zero>,

    /// DC gain (optional)
    pub dc_gain: Option<f64>,

    /// Sampling frequency for z-domain (Hz)
    pub sampling_freq: Option<f64>,
}

impl PoleZeroData {
    pub fn new(name: &str, domain: Domain) -> Self {
        Self {
            name: name.to_string(),
            domain,
            poles: Vec::new(),
            zeros: Vec::new(),
            dc_gain: None,
            sampling_freq: None,
        }
    }

    /// Add a pole
    pub fn add_pole(&mut self, pole: Pole) {
        self.poles.push(pole);
    }

    /// Add a complex conjugate pole pair
    pub fn add_pole_pair(&mut self, re: f64, im: f64) {
        let mut p1 = Pole::new(re, im);
        p1.pole_type = PoleType::ComplexPair;
        let mut p2 = Pole::new(re, -im);
        p2.pole_type = PoleType::ComplexPair;
        self.poles.push(p1);
        self.poles.push(p2);
    }

    /// Add a zero
    pub fn add_zero(&mut self, zero: Zero) {
        self.zeros.push(zero);
    }

    /// Add a complex conjugate zero pair
    pub fn add_zero_pair(&mut self, re: f64, im: f64) {
        self.zeros.push(Zero::new(re, im));
        self.zeros.push(Zero::new(re, -im));
    }

    /// Check if system is stable
    pub fn is_stable(&self) -> bool {
        match self.domain {
            Domain::SDomain => self.poles.iter().all(|p| p.is_stable_s()),
            Domain::ZDomain => self.poles.iter().all(|p| p.is_stable_z()),
        }
    }

    /// Check if marginally stable
    pub fn is_marginally_stable(&self) -> bool {
        match self.domain {
            Domain::SDomain => {
                self.poles.iter().any(|p| p.is_marginal_s())
                    && !self.poles.iter().any(|p| p.location.re > 1e-10)
            }
            Domain::ZDomain => {
                self.poles.iter().any(|p| p.is_marginal_z())
                    && !self
                        .poles
                        .iter()
                        .any(|p| p.location.magnitude() > 1.0 + 1e-10)
            }
        }
    }

    /// Get number of unstable poles
    pub fn unstable_pole_count(&self) -> usize {
        match self.domain {
            Domain::SDomain => self.poles.iter().filter(|p| !p.is_stable_s()).count(),
            Domain::ZDomain => self.poles.iter().filter(|p| !p.is_stable_z()).count(),
        }
    }

    /// Get number of RHP zeros (non-minimum phase)
    pub fn rhp_zero_count(&self) -> usize {
        self.zeros.iter().filter(|z| z.is_rhp()).count()
    }

    /// Check if minimum phase (no RHP zeros)
    pub fn is_minimum_phase(&self) -> bool {
        self.rhp_zero_count() == 0
    }

    /// Get dominant pole (closest to imaginary axis in s-domain)
    pub fn dominant_pole(&self) -> Option<&Pole> {
        match self.domain {
            Domain::SDomain => self
                .poles
                .iter()
                .filter(|p| p.is_stable_s())
                .min_by(|a, b| {
                    a.location
                        .re
                        .abs()
                        .partial_cmp(&b.location.re.abs())
                        .unwrap()
                }),
            Domain::ZDomain => self
                .poles
                .iter()
                .filter(|p| p.is_stable_z())
                .max_by(|a, b| {
                    a.location
                        .magnitude()
                        .partial_cmp(&b.location.magnitude())
                        .unwrap()
                }),
        }
    }

    /// Get system order
    pub fn order(&self) -> usize {
        self.poles.iter().map(|p| p.multiplicity as usize).sum()
    }

    /// Get bounds for plotting
    pub fn get_bounds(&self) -> PlotBounds {
        let all_points: Vec<&Complex> = self
            .poles
            .iter()
            .map(|p| &p.location)
            .chain(self.zeros.iter().map(|z| &z.location))
            .collect();

        if all_points.is_empty() {
            return PlotBounds::default();
        }

        let mut min_re = f64::MAX;
        let mut max_re = f64::MIN;
        let mut min_im = f64::MAX;
        let mut max_im = f64::MIN;

        for p in &all_points {
            min_re = min_re.min(p.re);
            max_re = max_re.max(p.re);
            min_im = min_im.min(p.im);
            max_im = max_im.max(p.im);
        }

        // Add some padding
        let re_range = (max_re - min_re).max(1.0);
        let im_range = (max_im - min_im).max(1.0);
        let padding = (re_range.max(im_range)) * 0.2;

        PlotBounds {
            min_re: min_re - padding,
            max_re: max_re + padding,
            min_im: min_im - padding,
            max_im: max_im + padding,
        }
    }
}

/// Plot bounds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlotBounds {
    pub min_re: f64,
    pub max_re: f64,
    pub min_im: f64,
    pub max_im: f64,
}

impl Default for PlotBounds {
    fn default() -> Self {
        Self {
            min_re: -2.0,
            max_re: 2.0,
            min_im: -2.0,
            max_im: 2.0,
        }
    }
}

impl PlotBounds {
    /// Check if bounds include the origin
    pub fn includes_origin(&self) -> bool {
        self.min_re <= 0.0 && self.max_re >= 0.0 && self.min_im <= 0.0 && self.max_im >= 0.0
    }

    /// Get center
    pub fn center(&self) -> (f64, f64) {
        (
            (self.min_re + self.max_re) / 2.0,
            (self.min_im + self.max_im) / 2.0,
        )
    }

    /// Get span
    pub fn span(&self) -> (f64, f64) {
        (self.max_re - self.min_re, self.max_im - self.min_im)
    }
}

// =============================================================================
// PZ Plot Configuration
// =============================================================================

/// Pole-zero plot configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PzPlotConfig {
    /// Show coordinate grid
    pub show_grid: bool,

    /// Show unit circle (for z-domain)
    pub show_unit_circle: bool,

    /// Show stability region
    pub show_stability_region: bool,

    /// Pole marker size
    pub pole_size: f32,

    /// Zero marker size
    pub zero_size: f32,

    /// Pole color
    pub pole_color: String,

    /// Zero color
    pub zero_color: String,

    /// Unstable pole color
    pub unstable_color: String,

    /// Background color
    pub background_color: String,

    /// Grid color
    pub grid_color: String,

    /// Show pole/zero labels
    pub show_labels: bool,

    /// Show numerical values on hover
    pub show_values: bool,
}

impl Default for PzPlotConfig {
    fn default() -> Self {
        Self {
            show_grid: true,
            show_unit_circle: true,
            show_stability_region: true,
            pole_size: 10.0,
            zero_size: 10.0,
            pole_color: "#ff5722".to_string(),
            zero_color: "#4CAF50".to_string(),
            unstable_color: "#f44336".to_string(),
            background_color: "#1a1a2e".to_string(),
            grid_color: "#333".to_string(),
            show_labels: true,
            show_values: true,
        }
    }
}

// =============================================================================
// PZ Plot State
// =============================================================================

/// Complete pole-zero plot state
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PzPlotState {
    /// Plot configuration
    pub config: PzPlotConfig,

    /// Pole-zero data
    pub data: PoleZeroData,

    /// Current view bounds
    pub bounds: PlotBounds,

    /// Selected pole index
    pub selected_pole: Option<usize>,

    /// Selected zero index
    pub selected_zero: Option<usize>,

    /// Cursor position
    pub cursor: Option<Complex>,
}

impl PzPlotState {
    pub fn new(data: PoleZeroData) -> Self {
        let bounds = data.get_bounds();
        Self {
            config: PzPlotConfig::default(),
            data,
            bounds,
            selected_pole: None,
            selected_zero: None,
            cursor: None,
        }
    }

    /// Get stability summary
    pub fn stability_summary(&self) -> StabilitySummary {
        StabilitySummary {
            is_stable: self.data.is_stable(),
            is_marginally_stable: self.data.is_marginally_stable(),
            unstable_poles: self.data.unstable_pole_count(),
            is_minimum_phase: self.data.is_minimum_phase(),
            rhp_zeros: self.data.rhp_zero_count(),
            order: self.data.order(),
        }
    }

    /// Get selected pole info
    pub fn selected_pole_info(&self) -> Option<PoleInfo> {
        let idx = self.selected_pole?;
        let pole = self.data.poles.get(idx)?;

        Some(PoleInfo {
            location: pole.location,
            natural_frequency: pole.natural_frequency(),
            damping_ratio: pole.damping_ratio(),
            quality_factor: pole.quality_factor(),
            damped_frequency: pole.damped_frequency(),
            time_constant: pole.time_constant(),
            settling_time: pole.settling_time(),
            is_stable: match self.data.domain {
                Domain::SDomain => pole.is_stable_s(),
                Domain::ZDomain => pole.is_stable_z(),
            },
        })
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, re: f64, im: f64) {
        self.cursor = Some(Complex::new(re, im));
    }

    /// Clear selections
    pub fn clear_selection(&mut self) {
        self.selected_pole = None;
        self.selected_zero = None;
    }

    /// Auto-fit bounds to data
    pub fn auto_fit(&mut self) {
        self.bounds = self.data.get_bounds();
    }
}

/// Stability summary
#[derive(Debug, Clone)]
pub struct StabilitySummary {
    pub is_stable: bool,
    pub is_marginally_stable: bool,
    pub unstable_poles: usize,
    pub is_minimum_phase: bool,
    pub rhp_zeros: usize,
    pub order: usize,
}

/// Pole information for display
#[derive(Debug, Clone)]
pub struct PoleInfo {
    pub location: Complex,
    pub natural_frequency: f64,
    pub damping_ratio: f64,
    pub quality_factor: f64,
    pub damped_frequency: f64,
    pub time_constant: Option<f64>,
    pub settling_time: Option<f64>,
    pub is_stable: bool,
}

// =============================================================================
// Formatting Helpers
// =============================================================================

/// Format complex number for display
pub fn format_complex(c: &Complex) -> String {
    let sign = if c.im >= 0.0 { "+" } else { "-" };
    format!("{:.4} {} j{:.4}", c.re, sign, c.im.abs())
}

/// Format frequency for display
pub fn format_frequency(freq_rad: f64) -> String {
    let freq_hz = freq_rad / (2.0 * PI);
    if freq_hz.abs() >= 1e9 {
        format!("{:.3} GHz", freq_hz / 1e9)
    } else if freq_hz.abs() >= 1e6 {
        format!("{:.3} MHz", freq_hz / 1e6)
    } else if freq_hz.abs() >= 1e3 {
        format!("{:.3} kHz", freq_hz / 1e3)
    } else {
        format!("{:.3} Hz", freq_hz)
    }
}

/// Format time for display
pub fn format_time(t: f64) -> String {
    let abs_t = t.abs();
    if abs_t >= 1.0 {
        format!("{:.3} s", t)
    } else if abs_t >= 1e-3 {
        format!("{:.3} ms", t * 1e3)
    } else if abs_t >= 1e-6 {
        format!("{:.3} µs", t * 1e6)
    } else if abs_t >= 1e-9 {
        format!("{:.3} ns", t * 1e9)
    } else {
        format!("{:.3} ps", t * 1e12)
    }
}

// =============================================================================
// Generation Helpers
// =============================================================================

/// Generate unit circle points for z-domain
pub fn unit_circle_points(num_points: usize) -> Vec<(f64, f64)> {
    (0..=num_points)
        .map(|i| {
            let angle = 2.0 * PI * i as f64 / num_points as f64;
            (angle.cos(), angle.sin())
        })
        .collect()
}

/// Generate imaginary axis line
pub fn imaginary_axis_points(min_im: f64, max_im: f64) -> Vec<(f64, f64)> {
    vec![(0.0, min_im), (0.0, max_im)]
}

/// Generate real axis line
pub fn real_axis_points(min_re: f64, max_re: f64) -> Vec<(f64, f64)> {
    vec![(min_re, 0.0), (max_re, 0.0)]
}

// =============================================================================
// Pole-Zero Plot UI Component
// =============================================================================

/// PZ plot viewer component properties
#[derive(Props, Clone, PartialEq)]
pub struct PzPlotViewerProps {
    /// Plot state
    pub state: PzPlotState,

    /// Canvas width
    #[props(default = 600)]
    pub width: u32,

    /// Canvas height
    #[props(default = 600)]
    pub height: u32,

    /// Show info panel
    #[props(default = true)]
    pub show_info: bool,
}

/// Pole-zero plot viewer component
#[component]
pub fn PzPlotViewer(props: PzPlotViewerProps) -> Element {
    let margins = PlotMargins {
        top: 40.0,
        bottom: 50.0,
        left: 60.0,
        right: if props.show_info { 220.0 } else { 40.0 },
    };

    let plot_width = (props.width as f64) - margins.left - margins.right;
    let plot_height = (props.height as f64) - margins.top - margins.bottom;

    let bounds = &props.state.bounds;
    let x_range = bounds.max_re - bounds.min_re;
    let y_range = bounds.max_im - bounds.min_im;

    // Coordinate transform functions
    let to_screen_x =
        |re: f64| -> f64 { margins.left + ((re - bounds.min_re) / x_range) * plot_width };
    let to_screen_y =
        |im: f64| -> f64 { margins.top + ((bounds.max_im - im) / y_range) * plot_height };

    let summary = props.state.stability_summary();

    rsx! {
        div {
            class: "pz-plot-container",
            style: "display: flex; background: {props.state.config.background_color}; border-radius: 8px; padding: 10px;",

            svg {
                width: "{props.width}",
                height: "{props.height}",
                view_box: "0 0 {props.width} {props.height}",

                // Background
                rect {
                    x: "0",
                    y: "0",
                    width: "{props.width}",
                    height: "{props.height}",
                    fill: "{props.state.config.background_color}",
                }

                // Plot area
                rect {
                    x: "{margins.left}",
                    y: "{margins.top}",
                    width: "{plot_width}",
                    height: "{plot_height}",
                    fill: "#0d0d1a",
                    stroke: "#333",
                }

                // Title
                text {
                    x: "{(props.width as f64) / 2.0}",
                    y: "20",
                    fill: "#fff",
                    "font-size": "14",
                    "text-anchor": "middle",
                    "font-weight": "bold",
                    "Pole-Zero Plot ({props.state.data.domain.display_name()})"
                }

                // Stability region (shading for stable area)
                if props.state.config.show_stability_region {
                    { render_stability_region(&props.state.data.domain, &bounds, margins.left, margins.top, plot_width, plot_height) }
                }

                // Grid
                if props.state.config.show_grid {
                    { render_pz_grid(&bounds, margins.left, margins.top, plot_width, plot_height, &props.state.config.grid_color) }
                }

                // Unit circle (for z-domain)
                if props.state.config.show_unit_circle && props.state.data.domain == Domain::ZDomain {
                    { render_unit_circle(to_screen_x(0.0), to_screen_y(0.0), plot_width.min(plot_height) / 2.0 * (2.0 / x_range).min(2.0 / y_range)) }
                }

                // Imaginary axis
                if bounds.min_re <= 0.0 && bounds.max_re >= 0.0 {
                    line {
                        x1: "{to_screen_x(0.0)}",
                        y1: "{margins.top}",
                        x2: "{to_screen_x(0.0)}",
                        y2: "{margins.top + plot_height}",
                        stroke: "#666",
                        "stroke-width": "1",
                        "stroke-dasharray": "4,2",
                    }
                }

                // Real axis
                if bounds.min_im <= 0.0 && bounds.max_im >= 0.0 {
                    line {
                        x1: "{margins.left}",
                        y1: "{to_screen_y(0.0)}",
                        x2: "{margins.left + plot_width}",
                        y2: "{to_screen_y(0.0)}",
                        stroke: "#666",
                        "stroke-width": "1",
                        "stroke-dasharray": "4,2",
                    }
                }

                // Poles (X markers)
                for (i, pole) in props.state.data.poles.iter().enumerate() {
                    { render_pole(pole, i, &props.state, to_screen_x(pole.location.re), to_screen_y(pole.location.im)) }
                }

                // Zeros (O markers)
                for (i, zero) in props.state.data.zeros.iter().enumerate() {
                    { render_zero(zero, i, &props.state, to_screen_x(zero.location.re), to_screen_y(zero.location.im)) }
                }

                // Axis labels
                { render_pz_axis_labels(&bounds, margins.left, margins.top, plot_width, plot_height) }
            }

            // Info panel
            if props.show_info {
                { render_pz_info_panel(&props.state, &summary) }
            }
        }
    }
}

/// Plot margins
struct PlotMargins {
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

/// Render stability region
fn render_stability_region(
    domain: &Domain,
    bounds: &PlotBounds,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) -> Element {
    match domain {
        Domain::SDomain => {
            // Stable region is left half-plane
            let x_ratio = if bounds.max_re > 0.0 && bounds.min_re < 0.0 {
                (-bounds.min_re) / (bounds.max_re - bounds.min_re)
            } else if bounds.max_re <= 0.0 {
                1.0
            } else {
                0.0
            };
            let stable_width = w * x_ratio;

            rsx! {
                rect {
                    x: "{x}",
                    y: "{y}",
                    width: "{stable_width}",
                    height: "{h}",
                    fill: "rgba(76, 175, 80, 0.1)",
                }
            }
        }
        Domain::ZDomain => {
            // Stable region is inside unit circle (handled by unit circle rendering)
            rsx! {}
        }
    }
}

/// Render grid
fn render_pz_grid(bounds: &PlotBounds, x: f64, y: f64, w: f64, h: f64, color: &str) -> Element {
    let x_range = bounds.max_re - bounds.min_re;
    let y_range = bounds.max_im - bounds.min_im;
    let x_step = nice_step(x_range / 5.0);
    let y_step = nice_step(y_range / 5.0);

    rsx! {
        g { class: "grid",
            // Vertical lines
            {
                let mut lines = Vec::new();
                let mut v = (bounds.min_re / x_step).ceil() * x_step;
                while v <= bounds.max_re {
                    let sx = x + ((v - bounds.min_re) / x_range) * w;
                    lines.push((sx, v));
                    v += x_step;
                }
                rsx! {
                    for (sx, _) in lines {
                        line {
                            x1: "{sx}",
                            y1: "{y}",
                            x2: "{sx}",
                            y2: "{y + h}",
                            stroke: "{color}",
                            "stroke-width": "0.5",
                            opacity: "0.5",
                        }
                    }
                }
            }
            // Horizontal lines
            {
                let mut lines = Vec::new();
                let mut v = (bounds.min_im / y_step).ceil() * y_step;
                while v <= bounds.max_im {
                    let sy = y + ((bounds.max_im - v) / y_range) * h;
                    lines.push((sy, v));
                    v += y_step;
                }
                rsx! {
                    for (sy, _) in lines {
                        line {
                            x1: "{x}",
                            y1: "{sy}",
                            x2: "{x + w}",
                            y2: "{sy}",
                            stroke: "{color}",
                            "stroke-width": "0.5",
                            opacity: "0.5",
                        }
                    }
                }
            }
        }
    }
}

/// Render unit circle for z-domain
fn render_unit_circle(cx: f64, cy: f64, radius: f64) -> Element {
    rsx! {
        circle {
            cx: "{cx}",
            cy: "{cy}",
            r: "{radius}",
            fill: "rgba(76, 175, 80, 0.1)",
            stroke: "#4CAF50",
            "stroke-width": "1",
            "stroke-dasharray": "4,2",
        }
    }
}

/// Render a pole marker (X)
fn render_pole(pole: &Pole, idx: usize, state: &PzPlotState, x: f64, y: f64) -> Element {
    let size = state.config.pole_size as f64;
    let is_selected = state.selected_pole == Some(idx);
    let is_stable = match state.data.domain {
        Domain::SDomain => pole.is_stable_s(),
        Domain::ZDomain => pole.is_stable_z(),
    };
    let color = if is_stable {
        &state.config.pole_color
    } else {
        &state.config.unstable_color
    };
    let stroke_width = if is_selected { 3.0 } else { 2.0 };

    rsx! {
        g { class: "pole",
            // X marker
            line {
                x1: "{x - size/2.0}",
                y1: "{y - size/2.0}",
                x2: "{x + size/2.0}",
                y2: "{y + size/2.0}",
                stroke: "{color}",
                "stroke-width": "{stroke_width}",
            }
            line {
                x1: "{x - size/2.0}",
                y1: "{y + size/2.0}",
                x2: "{x + size/2.0}",
                y2: "{y - size/2.0}",
                stroke: "{color}",
                "stroke-width": "{stroke_width}",
            }
            // Multiplicity indicator
            if pole.multiplicity > 1 {
                text {
                    x: "{x + size}",
                    y: "{y - size}",
                    fill: "{color}",
                    "font-size": "10",
                    "{pole.multiplicity}"
                }
            }
            // Label
            if state.config.show_labels {
                if let Some(label) = &pole.label {
                    text {
                        x: "{x + size}",
                        y: "{y}",
                        fill: "#fff",
                        "font-size": "11",
                        "{label}"
                    }
                }
            }
        }
    }
}

/// Render a zero marker (O)
fn render_zero(zero: &Zero, idx: usize, state: &PzPlotState, x: f64, y: f64) -> Element {
    let size = state.config.zero_size as f64;
    let is_selected = state.selected_zero == Some(idx);
    let is_rhp = zero.is_rhp();
    let color = if is_rhp {
        "#ff9800" // Warning color for RHP zeros
    } else {
        &state.config.zero_color
    };
    let stroke_width = if is_selected { 3.0 } else { 2.0 };

    rsx! {
        g { class: "zero",
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "{size/2.0}",
                fill: "none",
                stroke: "{color}",
                "stroke-width": "{stroke_width}",
            }
            // Multiplicity indicator
            if zero.multiplicity > 1 {
                text {
                    x: "{x + size}",
                    y: "{y - size}",
                    fill: "{color}",
                    "font-size": "10",
                    "{zero.multiplicity}"
                }
            }
            // Label
            if state.config.show_labels {
                if let Some(label) = &zero.label {
                    text {
                        x: "{x + size}",
                        y: "{y}",
                        fill: "#fff",
                        "font-size": "11",
                        "{label}"
                    }
                }
            }
        }
    }
}

/// Render axis labels
fn render_pz_axis_labels(bounds: &PlotBounds, x: f64, y: f64, w: f64, h: f64) -> Element {
    rsx! {
        g { class: "axis-labels",
            // X-axis label
            text {
                x: "{x + w/2.0}",
                y: "{y + h + 35.0}",
                fill: "#888",
                "font-size": "12",
                "text-anchor": "middle",
                "Real (σ)"
            }
            // Y-axis label
            text {
                x: "15",
                y: "{y + h/2.0}",
                fill: "#888",
                "font-size": "12",
                "text-anchor": "middle",
                transform: "rotate(-90, 15, {y + h/2.0})",
                "Imaginary (jω)"
            }
            // Axis tick labels
            text {
                x: "{x}",
                y: "{y + h + 15.0}",
                fill: "#888",
                "font-size": "10",
                "text-anchor": "middle",
                "{bounds.min_re:.2}"
            }
            text {
                x: "{x + w}",
                y: "{y + h + 15.0}",
                fill: "#888",
                "font-size": "10",
                "text-anchor": "middle",
                "{bounds.max_re:.2}"
            }
        }
    }
}

/// Render info panel
fn render_pz_info_panel(state: &PzPlotState, summary: &StabilitySummary) -> Element {
    let stability_color = if summary.is_stable {
        "#4CAF50"
    } else if summary.is_marginally_stable {
        "#ff9800"
    } else {
        "#f44336"
    };

    let stability_text = if summary.is_stable {
        "STABLE"
    } else if summary.is_marginally_stable {
        "MARGINALLY STABLE"
    } else {
        "UNSTABLE"
    };

    rsx! {
        div {
            class: "pz-info-panel",
            style: "margin-left: 20px; color: #fff; font-family: monospace; font-size: 12px; min-width: 200px;",

            h3 {
                style: "margin: 0 0 10px 0; color: #ff5722;",
                "Pole-Zero Analysis"
            }

            // Stability status
            div { style: "margin-bottom: 10px; padding: 8px; background: #222; border-radius: 4px; border-left: 3px solid {stability_color};",
                div { style: "color: {stability_color}; font-weight: bold;", "{stability_text}" }
                div { style: "font-size: 10px; color: #888; margin-top: 4px;",
                    "{state.data.domain.stability_criterion()}"
                }
            }

            // System info
            div { style: "margin-bottom: 10px;",
                div { style: "color: #888;", "System" }
                div { "Order: {summary.order}" }
                div { "Poles: {state.data.poles.len()}" }
                div { "Zeros: {state.data.zeros.len()}" }
                if summary.unstable_poles > 0 {
                    div { style: "color: #f44336;", "Unstable poles: {summary.unstable_poles}" }
                }
                if summary.rhp_zeros > 0 {
                    div { style: "color: #ff9800;", "RHP zeros: {summary.rhp_zeros} (non-minimum phase)" }
                }
            }

            // Selected pole info
            if let Some(info) = state.selected_pole_info() {
                div { style: "margin-bottom: 10px; padding: 8px; background: #333; border-radius: 4px;",
                    div { style: "color: #ff5722;", "Selected Pole" }
                    div { "Location: {format_complex(&info.location)}" }
                    div { "ωₙ: {format_frequency(info.natural_frequency)}" }
                    div { "ζ: {info.damping_ratio:.4}" }
                    div { "Q: {info.quality_factor:.2}" }
                    if let Some(tc) = info.time_constant {
                        div { "τ: {format_time(tc)}" }
                    }
                    if let Some(ts) = info.settling_time {
                        div { "Tₛ (2%): {format_time(ts)}" }
                    }
                }
            }

            // Legend
            div { style: "margin-top: 10px; padding-top: 10px; border-top: 1px solid #333;",
                div { style: "color: #888; margin-bottom: 5px;", "Legend" }
                div { style: "display: flex; align-items: center; margin: 3px 0;",
                    span { style: "color: #ff5722; font-size: 14px; margin-right: 8px;", "×" }
                    span { "Pole" }
                }
                div { style: "display: flex; align-items: center; margin: 3px 0;",
                    span { style: "color: #4CAF50; font-size: 14px; margin-right: 8px;", "○" }
                    span { "Zero" }
                }
            }
        }
    }
}

/// Calculate nice step size for axis
fn nice_step(rough_step: f64) -> f64 {
    let exponent = rough_step.log10().floor();
    let fraction = rough_step / (10.0_f64).powf(exponent);
    let nice_fraction = if fraction < 1.5 {
        1.0
    } else if fraction < 3.0 {
        2.0
    } else if fraction < 7.0 {
        5.0
    } else {
        10.0
    };
    nice_fraction * (10.0_f64).powf(exponent)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Complex Number Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_complex_new() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.re, 3.0);
        assert_eq!(c.im, 4.0);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert!((c.magnitude() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_complex_phase_deg() {
        let c = Complex::new(1.0, 1.0);
        assert!((c.phase_deg() - 45.0).abs() < 0.1);
    }

    #[test]
    fn test_complex_from_polar() {
        let c = Complex::from_polar(1.0, 0.0);
        assert!((c.re - 1.0).abs() < 0.001);
        assert!(c.im.abs() < 0.001);
    }

    #[test]
    fn test_complex_is_real() {
        assert!(Complex::new(1.0, 0.0).is_real());
        assert!(!Complex::new(1.0, 0.1).is_real());
    }

    #[test]
    fn test_complex_is_inside_unit_circle() {
        assert!(Complex::new(0.5, 0.5).is_inside_unit_circle());
        assert!(!Complex::new(1.0, 0.0).is_inside_unit_circle());
        assert!(!Complex::new(0.0, 1.0).is_inside_unit_circle());
    }

    #[test]
    fn test_complex_lhp_rhp() {
        assert!(Complex::new(-1.0, 0.0).is_in_lhp());
        assert!(!Complex::new(1.0, 0.0).is_in_lhp());
        assert!(Complex::new(1.0, 0.0).is_in_rhp());
    }

    #[test]
    fn test_complex_distance() {
        let a = Complex::new(0.0, 0.0);
        let b = Complex::new(3.0, 4.0);
        assert!((a.distance_to(&b) - 5.0).abs() < 0.001);
    }

    // -------------------------------------------------------------------------
    // Domain Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_domain_all() {
        assert_eq!(Domain::all().len(), 2);
    }

    #[test]
    fn test_domain_display_names() {
        assert!(Domain::SDomain.display_name().contains("Continuous"));
        assert!(Domain::ZDomain.display_name().contains("Discrete"));
    }

    // -------------------------------------------------------------------------
    // Pole Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pole_new_real() {
        let p = Pole::new(-1.0, 0.0);
        assert_eq!(p.pole_type, PoleType::Real);
    }

    #[test]
    fn test_pole_new_origin() {
        let p = Pole::new(0.0, 0.0);
        assert_eq!(p.pole_type, PoleType::Origin);
    }

    #[test]
    fn test_pole_new_complex() {
        let p = Pole::new(-1.0, 1.0);
        assert_eq!(p.pole_type, PoleType::Complex);
    }

    #[test]
    fn test_pole_with_multiplicity() {
        let p = Pole::new(-1.0, 0.0).with_multiplicity(2);
        assert_eq!(p.multiplicity, 2);
    }

    #[test]
    fn test_pole_with_label() {
        let p = Pole::new(-1.0, 0.0).with_label("p1");
        assert_eq!(p.label, Some("p1".to_string()));
    }

    #[test]
    fn test_pole_is_stable_s() {
        assert!(Pole::new(-1.0, 0.0).is_stable_s());
        assert!(Pole::new(-1.0, 1.0).is_stable_s());
        assert!(!Pole::new(1.0, 0.0).is_stable_s());
    }

    #[test]
    fn test_pole_is_stable_z() {
        assert!(Pole::new(0.5, 0.5).is_stable_z());
        assert!(!Pole::new(0.9, 0.9).is_stable_z()); // |z| > 1
    }

    #[test]
    fn test_pole_natural_frequency() {
        let p = Pole::new(-1.0, 1.0);
        let wn = p.natural_frequency();
        assert!((wn - 2.0_f64.sqrt()).abs() < 0.001);
    }

    #[test]
    fn test_pole_damping_ratio() {
        let p = Pole::new(-1.0, 1.0);
        let zeta = p.damping_ratio();
        let expected = 1.0 / 2.0_f64.sqrt();
        assert!((zeta - expected).abs() < 0.01);
    }

    #[test]
    fn test_pole_quality_factor() {
        let p = Pole::new(-0.1, 1.0); // High Q
        let q = p.quality_factor();
        assert!(q > 1.0);
    }

    #[test]
    fn test_pole_time_constant() {
        let p = Pole::new(-10.0, 0.0);
        let tau = p.time_constant().unwrap();
        assert!((tau - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_pole_settling_time() {
        let p = Pole::new(-10.0, 0.0);
        let ts = p.settling_time().unwrap();
        assert!((ts - 0.4).abs() < 0.001); // 4/|σ|
    }

    // -------------------------------------------------------------------------
    // Zero Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_zero_new() {
        let z = Zero::new(-1.0, 0.0);
        assert_eq!(z.location.re, -1.0);
        assert_eq!(z.multiplicity, 1);
    }

    #[test]
    fn test_zero_is_rhp() {
        assert!(Zero::new(1.0, 0.0).is_rhp());
        assert!(!Zero::new(-1.0, 0.0).is_rhp());
    }

    #[test]
    fn test_zero_is_real() {
        assert!(Zero::new(-1.0, 0.0).is_real());
        assert!(!Zero::new(-1.0, 1.0).is_real());
    }

    // -------------------------------------------------------------------------
    // PoleZeroData Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pz_data_new() {
        let data = PoleZeroData::new("Test", Domain::SDomain);
        assert_eq!(data.name, "Test");
        assert!(data.poles.is_empty());
    }

    #[test]
    fn test_pz_data_add_pole() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(-1.0, 0.0));
        assert_eq!(data.poles.len(), 1);
    }

    #[test]
    fn test_pz_data_add_pole_pair() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole_pair(-1.0, 1.0);
        assert_eq!(data.poles.len(), 2);
        assert_eq!(data.poles[0].location.im, 1.0);
        assert_eq!(data.poles[1].location.im, -1.0);
    }

    #[test]
    fn test_pz_data_is_stable_s() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(-1.0, 0.0));
        data.add_pole(Pole::new(-2.0, 1.0));
        assert!(data.is_stable());
    }

    #[test]
    fn test_pz_data_is_unstable_s() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(1.0, 0.0)); // RHP pole
        assert!(!data.is_stable());
    }

    #[test]
    fn test_pz_data_is_stable_z() {
        let mut data = PoleZeroData::new("Test", Domain::ZDomain);
        data.add_pole(Pole::new(0.5, 0.0));
        assert!(data.is_stable());
    }

    #[test]
    fn test_pz_data_is_unstable_z() {
        let mut data = PoleZeroData::new("Test", Domain::ZDomain);
        data.add_pole(Pole::new(1.5, 0.0)); // Outside unit circle
        assert!(!data.is_stable());
    }

    #[test]
    fn test_pz_data_unstable_pole_count() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(-1.0, 0.0));
        data.add_pole(Pole::new(1.0, 0.0));
        data.add_pole(Pole::new(2.0, 0.0));
        assert_eq!(data.unstable_pole_count(), 2);
    }

    #[test]
    fn test_pz_data_rhp_zero_count() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_zero(Zero::new(-1.0, 0.0));
        data.add_zero(Zero::new(1.0, 0.0));
        assert_eq!(data.rhp_zero_count(), 1);
    }

    #[test]
    fn test_pz_data_is_minimum_phase() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_zero(Zero::new(-1.0, 0.0));
        assert!(data.is_minimum_phase());

        data.add_zero(Zero::new(1.0, 0.0));
        assert!(!data.is_minimum_phase());
    }

    #[test]
    fn test_pz_data_order() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(-1.0, 0.0).with_multiplicity(2));
        data.add_pole(Pole::new(-2.0, 0.0));
        assert_eq!(data.order(), 3);
    }

    #[test]
    fn test_pz_data_get_bounds() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(-2.0, 1.0));
        data.add_pole(Pole::new(-1.0, -1.0));
        let bounds = data.get_bounds();
        assert!(bounds.min_re < -2.0);
        assert!(bounds.max_re > -1.0);
    }

    #[test]
    fn test_pz_data_dominant_pole() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(-0.1, 0.0));
        data.add_pole(Pole::new(-10.0, 0.0));
        let dominant = data.dominant_pole().unwrap();
        assert!((dominant.location.re - (-0.1)).abs() < 0.01);
    }

    // -------------------------------------------------------------------------
    // PlotBounds Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_plot_bounds_default() {
        let bounds = PlotBounds::default();
        assert!(bounds.includes_origin());
    }

    #[test]
    fn test_plot_bounds_center() {
        let bounds = PlotBounds {
            min_re: -2.0,
            max_re: 2.0,
            min_im: -1.0,
            max_im: 1.0,
        };
        let (cx, cy) = bounds.center();
        assert_eq!(cx, 0.0);
        assert_eq!(cy, 0.0);
    }

    #[test]
    fn test_plot_bounds_span() {
        let bounds = PlotBounds {
            min_re: -2.0,
            max_re: 2.0,
            min_im: -1.0,
            max_im: 1.0,
        };
        let (sx, sy) = bounds.span();
        assert_eq!(sx, 4.0);
        assert_eq!(sy, 2.0);
    }

    // -------------------------------------------------------------------------
    // Config Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pz_config_default() {
        let config = PzPlotConfig::default();
        assert!(config.show_grid);
        assert!(config.show_unit_circle);
    }

    // -------------------------------------------------------------------------
    // State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pz_state_new() {
        let data = PoleZeroData::new("Test", Domain::SDomain);
        let state = PzPlotState::new(data);
        assert!(state.selected_pole.is_none());
    }

    #[test]
    fn test_pz_state_stability_summary() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(-1.0, 0.0));
        let state = PzPlotState::new(data);
        let summary = state.stability_summary();
        assert!(summary.is_stable);
        assert_eq!(summary.order, 1);
    }

    #[test]
    fn test_pz_state_set_cursor() {
        let data = PoleZeroData::new("Test", Domain::SDomain);
        let mut state = PzPlotState::new(data);
        state.set_cursor(1.0, 2.0);
        assert!(state.cursor.is_some());
        assert_eq!(state.cursor.unwrap().re, 1.0);
    }

    #[test]
    fn test_pz_state_clear_selection() {
        let data = PoleZeroData::new("Test", Domain::SDomain);
        let mut state = PzPlotState::new(data);
        state.selected_pole = Some(0);
        state.clear_selection();
        assert!(state.selected_pole.is_none());
    }

    // -------------------------------------------------------------------------
    // Formatting Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_complex() {
        let c = Complex::new(1.5, -2.3);
        let s = format_complex(&c);
        assert!(s.contains("1.5"));
        assert!(s.contains("-"));
        assert!(s.contains("j"));
    }

    #[test]
    fn test_format_frequency() {
        let s = format_frequency(2.0 * PI * 1e9);
        assert!(s.contains("GHz"));
    }

    #[test]
    fn test_format_time() {
        let s = format_time(1e-6);
        assert!(s.contains("µs"));
    }

    // -------------------------------------------------------------------------
    // Generation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_unit_circle_points() {
        let points = unit_circle_points(100);
        assert_eq!(points.len(), 101);
        // First and last should be same (complete circle)
        assert!((points[0].0 - points[100].0).abs() < 0.01);
    }

    #[test]
    fn test_imaginary_axis_points() {
        let points = imaginary_axis_points(-2.0, 2.0);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].0, 0.0);
    }

    #[test]
    fn test_real_axis_points() {
        let points = real_axis_points(-2.0, 2.0);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].1, 0.0);
    }

    // -------------------------------------------------------------------------
    // Serialization Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_complex_serialize() {
        let c = Complex::new(1.0, 2.0);
        let json = serde_json::to_string(&c).unwrap();
        assert!(json.contains("1.0"));
    }

    #[test]
    fn test_pz_data_roundtrip() {
        let mut data = PoleZeroData::new("Test", Domain::SDomain);
        data.add_pole(Pole::new(-1.0, 1.0));
        data.add_zero(Zero::new(-2.0, 0.0));

        let json = serde_json::to_string(&data).unwrap();
        let parsed: PoleZeroData = serde_json::from_str(&json).unwrap();
        assert_eq!(data.name, parsed.name);
        assert_eq!(data.poles.len(), parsed.poles.len());
    }

    #[test]
    fn test_pz_state_roundtrip() {
        let data = PoleZeroData::new("Test", Domain::SDomain);
        let state = PzPlotState::new(data);
        let json = serde_json::to_string(&state).unwrap();
        let parsed: PzPlotState = serde_json::from_str(&json).unwrap();
        assert_eq!(state.data.name, parsed.data.name);
    }
}
