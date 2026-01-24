//! Smith Chart Viewer
//!
//! Commercial-grade Smith chart visualization for RF/microwave circuit analysis.
//! Features:
//!
//! - Impedance (Z) and Admittance (Y) modes
//! - Normalized and denormalized views
//! - S-parameter overlay
//! - VSWR circles
//! - Q circles
//! - Constant resistance/conductance circles
//! - Constant reactance/susceptance arcs
//! - Marker and cursor support
//! - Export to various formats

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// =============================================================================
// Smith Chart Types
// =============================================================================

/// Smith chart display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SmithChartMode {
    /// Impedance (Z) chart
    #[default]
    Impedance,
    /// Admittance (Y) chart
    Admittance,
    /// Combined Z-Y overlay
    Combined,
}

impl SmithChartMode {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Impedance => "Impedance (Z)",
            Self::Admittance => "Admittance (Y)",
            Self::Combined => "Combined (Z+Y)",
        }
    }

    pub fn all() -> &'static [SmithChartMode] {
        &[
            SmithChartMode::Impedance,
            SmithChartMode::Admittance,
            SmithChartMode::Combined,
        ]
    }
}

/// Complex number representation
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

    /// Add two complex numbers
    pub fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }

    /// Subtract two complex numbers
    pub fn sub(&self, other: &Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }

    /// Multiply two complex numbers
    pub fn mul(&self, other: &Complex) -> Complex {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    /// Divide by another complex number
    pub fn div(&self, other: &Complex) -> Option<Complex> {
        let denom = other.re * other.re + other.im * other.im;
        if denom.abs() < 1e-30 {
            return None;
        }
        Some(Complex::new(
            (self.re * other.re + self.im * other.im) / denom,
            (self.im * other.re - self.re * other.im) / denom,
        ))
    }

    /// Scale by real number
    pub fn scale(&self, s: f64) -> Complex {
        Complex::new(self.re * s, self.im * s)
    }
}

/// Impedance value (complex)
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Impedance {
    /// Resistance (real part) in ohms
    pub r: f64,
    /// Reactance (imaginary part) in ohms
    pub x: f64,
}

impl Impedance {
    pub fn new(r: f64, x: f64) -> Self {
        Self { r, x }
    }

    /// Convert to complex
    pub fn to_complex(&self) -> Complex {
        Complex::new(self.r, self.x)
    }

    /// Create from complex
    pub fn from_complex(c: Complex) -> Self {
        Self { r: c.re, x: c.im }
    }

    /// Normalize to reference impedance
    pub fn normalize(&self, z0: f64) -> Self {
        Self {
            r: self.r / z0,
            x: self.x / z0,
        }
    }

    /// Denormalize from reference impedance
    pub fn denormalize(&self, z0: f64) -> Self {
        Self {
            r: self.r * z0,
            x: self.x * z0,
        }
    }

    /// Calculate reflection coefficient (Gamma)
    pub fn to_gamma(&self, z0: f64) -> Complex {
        let z_norm = self.normalize(z0);
        let z = Complex::new(z_norm.r, z_norm.x);
        let one = Complex::new(1.0, 0.0);

        // Gamma = (Z - 1) / (Z + 1)
        let num = z.sub(&one);
        let den = z.add(&one);
        num.div(&den).unwrap_or(Complex::new(0.0, 0.0))
    }

    /// Calculate VSWR from reflection coefficient magnitude
    pub fn vswr(&self, z0: f64) -> f64 {
        let gamma = self.to_gamma(z0).magnitude();
        if gamma >= 1.0 {
            f64::INFINITY
        } else {
            (1.0 + gamma) / (1.0 - gamma)
        }
    }

    /// Convert to admittance
    pub fn to_admittance(&self) -> Admittance {
        let mag_sq = self.r * self.r + self.x * self.x;
        if mag_sq < 1e-30 {
            Admittance::new(f64::INFINITY, 0.0)
        } else {
            Admittance::new(self.r / mag_sq, -self.x / mag_sq)
        }
    }

    /// Magnitude
    pub fn magnitude(&self) -> f64 {
        (self.r * self.r + self.x * self.x).sqrt()
    }

    /// Phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.x.atan2(self.r) * 180.0 / PI
    }
}

/// Admittance value (complex)
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Admittance {
    /// Conductance (real part) in siemens
    pub g: f64,
    /// Susceptance (imaginary part) in siemens
    pub b: f64,
}

impl Admittance {
    pub fn new(g: f64, b: f64) -> Self {
        Self { g, b }
    }

    /// Convert to complex
    pub fn to_complex(&self) -> Complex {
        Complex::new(self.g, self.b)
    }

    /// Convert to impedance
    pub fn to_impedance(&self) -> Impedance {
        let mag_sq = self.g * self.g + self.b * self.b;
        if mag_sq < 1e-30 {
            Impedance::new(f64::INFINITY, 0.0)
        } else {
            Impedance::new(self.g / mag_sq, -self.b / mag_sq)
        }
    }

    /// Normalize to reference admittance
    pub fn normalize(&self, y0: f64) -> Self {
        Self {
            g: self.g / y0,
            b: self.b / y0,
        }
    }
}

/// S-parameter data point
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SParameter {
    /// Frequency in Hz
    pub frequency: f64,
    /// Complex S-parameter value
    pub value: Complex,
}

impl SParameter {
    pub fn new(frequency: f64, re: f64, im: f64) -> Self {
        Self {
            frequency,
            value: Complex::new(re, im),
        }
    }

    /// Convert to impedance given Z0
    pub fn to_impedance(&self, z0: f64) -> Impedance {
        gamma_to_impedance(&self.value, z0)
    }

    /// Get magnitude in dB
    pub fn magnitude_db(&self) -> f64 {
        20.0 * self.value.magnitude().log10()
    }

    /// Get phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.value.phase_deg()
    }

    /// Get VSWR
    pub fn vswr(&self) -> f64 {
        let gamma = self.value.magnitude();
        if gamma >= 1.0 {
            f64::INFINITY
        } else {
            (1.0 + gamma) / (1.0 - gamma)
        }
    }

    /// Get return loss in dB
    pub fn return_loss_db(&self) -> f64 {
        -self.magnitude_db()
    }
}

/// Smith chart marker
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmithMarker {
    /// Marker ID
    pub id: usize,
    /// Marker label
    pub label: String,
    /// Index into trace data
    pub data_index: usize,
    /// Color
    pub color: String,
    /// Visible
    pub visible: bool,
}

impl SmithMarker {
    pub fn new(id: usize, data_index: usize) -> Self {
        Self {
            id,
            label: format!("M{}", id + 1),
            data_index,
            color: "#ff5722".to_string(),
            visible: true,
        }
    }
}

/// Smith chart trace (S-parameter data series)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmithTrace {
    /// Trace name (e.g., "S11", "S22")
    pub name: String,
    /// Data points
    pub data: Vec<SParameter>,
    /// Color
    pub color: String,
    /// Line width
    pub line_width: f32,
    /// Visible
    pub visible: bool,
    /// Markers on this trace
    pub markers: Vec<SmithMarker>,
}

impl SmithTrace {
    pub fn new(name: &str, data: Vec<SParameter>) -> Self {
        Self {
            name: name.to_string(),
            data,
            color: "#2196F3".to_string(),
            line_width: 2.0,
            visible: true,
            markers: Vec::new(),
        }
    }

    /// Add a marker at the given index
    pub fn add_marker(&mut self, index: usize) {
        if index < self.data.len() {
            let id = self.markers.len();
            self.markers.push(SmithMarker::new(id, index));
        }
    }

    /// Get frequency range
    pub fn frequency_range(&self) -> Option<(f64, f64)> {
        if self.data.is_empty() {
            return None;
        }
        let min = self
            .data
            .iter()
            .map(|s| s.frequency)
            .fold(f64::MAX, f64::min);
        let max = self
            .data
            .iter()
            .map(|s| s.frequency)
            .fold(f64::MIN, f64::max);
        Some((min, max))
    }
}

// =============================================================================
// Coordinate Conversion
// =============================================================================

/// Convert reflection coefficient to Smith chart coordinates
/// Returns (x, y) in range [-1, 1]
pub fn gamma_to_smith(gamma: &Complex) -> (f64, f64) {
    (gamma.re, gamma.im)
}

/// Convert impedance to reflection coefficient
pub fn impedance_to_gamma(z: &Impedance, z0: f64) -> Complex {
    z.to_gamma(z0)
}

/// Convert reflection coefficient to impedance
pub fn gamma_to_impedance(gamma: &Complex, z0: f64) -> Impedance {
    // Z = Z0 * (1 + Gamma) / (1 - Gamma)
    let one = Complex::new(1.0, 0.0);
    let num = one.add(gamma);
    let den = one.sub(gamma);

    if let Some(ratio) = num.div(&den) {
        Impedance::new(ratio.re * z0, ratio.im * z0)
    } else {
        Impedance::new(f64::INFINITY, 0.0)
    }
}

/// Convert admittance to reflection coefficient
pub fn admittance_to_gamma(y: &Admittance, y0: f64) -> Complex {
    let z = y.to_impedance();
    z.to_gamma(1.0 / y0)
}

// =============================================================================
// Circle/Arc Generation
// =============================================================================

/// Generate points for a constant resistance circle
pub fn constant_r_circle(r_norm: f64, num_points: usize) -> Vec<(f64, f64)> {
    if r_norm < 0.0 {
        return Vec::new();
    }

    // Center: (r/(r+1), 0)
    // Radius: 1/(r+1)
    let center_x = r_norm / (r_norm + 1.0);
    let radius = 1.0 / (r_norm + 1.0);

    let mut points = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let angle = 2.0 * PI * i as f64 / (num_points - 1) as f64;
        let x = center_x + radius * angle.cos();
        let y = radius * angle.sin();

        // Only include points inside unit circle
        if x * x + y * y <= 1.001 {
            points.push((x, y));
        }
    }
    points
}

/// Generate points for a constant reactance arc
pub fn constant_x_arc(x_norm: f64, num_points: usize) -> Vec<(f64, f64)> {
    if x_norm.abs() < 1e-10 {
        // X = 0 is a straight line
        return vec![(-1.0, 0.0), (1.0, 0.0)];
    }

    // Center: (1, 1/x)
    // Radius: 1/|x|
    let center_x = 1.0;
    let center_y = 1.0 / x_norm;
    let radius = 1.0 / x_norm.abs();

    let mut points = Vec::new();
    for i in 0..num_points {
        let angle = PI * i as f64 / (num_points - 1) as f64;
        let angle = if x_norm > 0.0 {
            -PI / 2.0 - angle
        } else {
            PI / 2.0 - angle
        };

        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();

        // Only include points inside unit circle
        if x * x + y * y <= 1.001 && x >= -1.0 && x <= 1.0 {
            points.push((x, y));
        }
    }
    points
}

/// Generate points for a constant conductance circle (admittance chart)
pub fn constant_g_circle(g_norm: f64, num_points: usize) -> Vec<(f64, f64)> {
    // Mirror of constant R circle across the origin
    let r_points = constant_r_circle(g_norm, num_points);
    r_points.iter().map(|(x, y)| (-x, -y)).collect()
}

/// Generate points for a constant susceptance arc (admittance chart)
pub fn constant_b_arc(b_norm: f64, num_points: usize) -> Vec<(f64, f64)> {
    // Mirror of constant X arc
    let x_points = constant_x_arc(b_norm, num_points);
    x_points.iter().map(|(x, y)| (-x, -y)).collect()
}

/// Generate VSWR circle
pub fn vswr_circle(vswr: f64, num_points: usize) -> Vec<(f64, f64)> {
    if vswr < 1.0 {
        return Vec::new();
    }

    let gamma = (vswr - 1.0) / (vswr + 1.0);
    let mut points = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let angle = 2.0 * PI * i as f64 / (num_points - 1) as f64;
        points.push((gamma * angle.cos(), gamma * angle.sin()));
    }
    points
}

/// Generate Q contour
pub fn q_circle(q: f64, num_points: usize) -> Vec<(f64, f64)> {
    // Q = |X|/R contour
    // This is more complex - we sample impedances with constant Q
    let mut points = Vec::new();

    for i in 0..num_points {
        let angle = PI * i as f64 / (num_points - 1) as f64 - PI / 2.0;
        let r = 0.01 + 100.0 * (i as f64 / (num_points - 1) as f64);
        let x = q * r;

        let z_pos = Impedance::new(r, x);
        let z_neg = Impedance::new(r, -x);

        let g_pos = z_pos.to_gamma(1.0);
        let g_neg = z_neg.to_gamma(1.0);

        if g_pos.magnitude() <= 1.001 {
            points.push((g_pos.re, g_pos.im));
        }
        if g_neg.magnitude() <= 1.001 {
            points.push((g_neg.re, g_neg.im));
        }
    }
    points
}

// =============================================================================
// Smith Chart State
// =============================================================================

/// Smith chart viewer configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmithChartConfig {
    /// Display mode
    pub mode: SmithChartMode,

    /// Reference impedance (Z0)
    pub z0: f64,

    /// Show resistance circles
    pub show_r_circles: bool,

    /// Show reactance arcs
    pub show_x_arcs: bool,

    /// Show conductance circles (admittance mode)
    pub show_g_circles: bool,

    /// Show susceptance arcs (admittance mode)
    pub show_b_arcs: bool,

    /// Show VSWR circles
    pub show_vswr_circles: bool,

    /// VSWR values to display
    pub vswr_values: Vec<f64>,

    /// Show Q contours
    pub show_q_circles: bool,

    /// Q values to display
    pub q_values: Vec<f64>,

    /// Standard R values for circles
    pub r_values: Vec<f64>,

    /// Standard X values for arcs
    pub x_values: Vec<f64>,

    /// Grid color
    pub grid_color: String,

    /// Background color
    pub background_color: String,
}

impl Default for SmithChartConfig {
    fn default() -> Self {
        Self {
            mode: SmithChartMode::Impedance,
            z0: 50.0,
            show_r_circles: true,
            show_x_arcs: true,
            show_g_circles: false,
            show_b_arcs: false,
            show_vswr_circles: false,
            vswr_values: vec![1.5, 2.0, 3.0],
            show_q_circles: false,
            q_values: vec![1.0, 2.0, 5.0],
            r_values: vec![0.0, 0.2, 0.5, 1.0, 2.0, 5.0],
            x_values: vec![-2.0, -1.0, -0.5, -0.2, 0.2, 0.5, 1.0, 2.0],
            grid_color: "#444".to_string(),
            background_color: "#1a1a2e".to_string(),
        }
    }
}

/// Complete Smith chart state
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SmithChartState {
    /// Configuration
    pub config: SmithChartConfig,

    /// Traces
    pub traces: Vec<SmithTrace>,

    /// Active trace index
    pub active_trace: Option<usize>,

    /// Cursor position (normalized)
    pub cursor: Option<(f64, f64)>,

    /// Cursor impedance
    pub cursor_impedance: Option<Impedance>,
}

impl SmithChartState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a trace
    pub fn add_trace(&mut self, trace: SmithTrace) {
        self.traces.push(trace);
        if self.active_trace.is_none() {
            self.active_trace = Some(0);
        }
    }

    /// Get active trace
    pub fn active_trace(&self) -> Option<&SmithTrace> {
        self.active_trace.and_then(|i| self.traces.get(i))
    }

    /// Update cursor position
    pub fn set_cursor(&mut self, x: f64, y: f64) {
        if x * x + y * y <= 1.0 {
            self.cursor = Some((x, y));
            let gamma = Complex::new(x, y);
            self.cursor_impedance = Some(gamma_to_impedance(&gamma, self.config.z0));
        } else {
            self.cursor = None;
            self.cursor_impedance = None;
        }
    }

    /// Clear cursor
    pub fn clear_cursor(&mut self) {
        self.cursor = None;
        self.cursor_impedance = None;
    }

    /// Get marker data for display
    pub fn get_marker_info(&self, trace_idx: usize, marker_idx: usize) -> Option<MarkerInfo> {
        let trace = self.traces.get(trace_idx)?;
        let marker = trace.markers.get(marker_idx)?;
        let data = trace.data.get(marker.data_index)?;

        let impedance = data.to_impedance(self.config.z0);

        Some(MarkerInfo {
            label: marker.label.clone(),
            frequency: data.frequency,
            impedance,
            gamma: data.value,
            vswr: data.vswr(),
            return_loss_db: data.return_loss_db(),
        })
    }
}

/// Marker information for display
#[derive(Debug, Clone)]
pub struct MarkerInfo {
    pub label: String,
    pub frequency: f64,
    pub impedance: Impedance,
    pub gamma: Complex,
    pub vswr: f64,
    pub return_loss_db: f64,
}

// =============================================================================
// Formatting Helpers
// =============================================================================

/// Format impedance for display
pub fn format_impedance(z: &Impedance) -> String {
    let sign = if z.x >= 0.0 { "+" } else { "-" };
    format!("{:.2} {} j{:.2} Ω", z.r, sign, z.x.abs())
}

/// Format frequency for display
pub fn format_frequency(freq: f64) -> String {
    if freq >= 1e9 {
        format!("{:.3} GHz", freq / 1e9)
    } else if freq >= 1e6 {
        format!("{:.3} MHz", freq / 1e6)
    } else if freq >= 1e3 {
        format!("{:.3} kHz", freq / 1e3)
    } else {
        format!("{:.3} Hz", freq)
    }
}

/// Format VSWR for display
pub fn format_vswr(vswr: f64) -> String {
    if vswr >= 100.0 || !vswr.is_finite() {
        "∞:1".to_string()
    } else {
        format!("{:.2}:1", vswr)
    }
}

// =============================================================================
// Smith Chart UI Component
// =============================================================================

/// Smith chart viewer component properties
#[derive(Props, Clone, PartialEq)]
pub struct SmithChartViewerProps {
    /// Smith chart state
    pub state: SmithChartState,

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

/// Smith chart viewer component
#[component]
pub fn SmithChartViewer(props: SmithChartViewerProps) -> Element {
    let center_x = props.width as f64 / 2.0;
    let center_y = props.height as f64 / 2.0;
    let radius = (props.width.min(props.height) as f64 / 2.0) - 40.0;

    rsx! {
        div {
            class: "smith-chart-container",
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

                // Title
                text {
                    x: "{center_x}",
                    y: "20",
                    fill: "#fff",
                    "font-size": "14",
                    "text-anchor": "middle",
                    "font-weight": "bold",
                    "Smith Chart (Z₀ = {props.state.config.z0} Ω)"
                }

                // Unit circle (outer boundary)
                circle {
                    cx: "{center_x}",
                    cy: "{center_y}",
                    r: "{radius}",
                    fill: "#0d0d1a",
                    stroke: "#666",
                    "stroke-width": "2",
                }

                // Resistance circles
                if props.state.config.show_r_circles {
                    { render_r_circles(&props.state.config.r_values, center_x, center_y, radius, &props.state.config.grid_color) }
                }

                // Reactance arcs
                if props.state.config.show_x_arcs {
                    { render_x_arcs(&props.state.config.x_values, center_x, center_y, radius, &props.state.config.grid_color) }
                }

                // VSWR circles
                if props.state.config.show_vswr_circles {
                    for vswr_val in &props.state.config.vswr_values {
                        { render_vswr_circle(*vswr_val, center_x, center_y, radius) }
                    }
                }

                // Real axis
                line {
                    x1: "{center_x - radius}",
                    y1: "{center_y}",
                    x2: "{center_x + radius}",
                    y2: "{center_y}",
                    stroke: "{props.state.config.grid_color}",
                    "stroke-width": "1",
                }

                // Center point (matched load)
                circle {
                    cx: "{center_x}",
                    cy: "{center_y}",
                    r: "3",
                    fill: "#4CAF50",
                }

                // S-parameter traces
                for trace in &props.state.traces {
                    if trace.visible {
                        { render_smith_trace(trace, center_x, center_y, radius) }
                    }
                }

                // Markers
                for trace in &props.state.traces {
                    for marker in &trace.markers {
                        if marker.visible {
                            if let Some(data) = trace.data.get(marker.data_index) {
                                { render_smith_marker(data, marker, center_x, center_y, radius) }
                            }
                        }
                    }
                }

                // Cursor
                if let Some((cx_pos, cy_pos)) = props.state.cursor {
                    { render_cursor(cx_pos, cy_pos, center_x, center_y, radius) }
                }

                // Axis labels
                { render_axis_labels(center_x, center_y, radius) }
            }

            // Info panel
            if props.show_info {
                { render_info_panel(&props.state) }
            }
        }
    }
}

/// Render resistance circles
fn render_r_circles(r_values: &[f64], cx: f64, cy: f64, radius: f64, color: &str) -> Element {
    rsx! {
        g { class: "r-circles",
            for r in r_values {
                { render_single_r_circle(*r, cx, cy, radius, color) }
            }
        }
    }
}

fn render_single_r_circle(r: f64, cx: f64, cy: f64, radius: f64, color: &str) -> Element {
    let points = constant_r_circle(r, 100);
    if points.is_empty() {
        return rsx! {};
    }

    let path_data = points_to_svg_path(&points, cx, cy, radius);

    rsx! {
        path {
            d: "{path_data}",
            stroke: "{color}",
            "stroke-width": "0.5",
            fill: "none",
            opacity: "0.6",
        }
    }
}

/// Render reactance arcs
fn render_x_arcs(x_values: &[f64], cx: f64, cy: f64, radius: f64, color: &str) -> Element {
    rsx! {
        g { class: "x-arcs",
            for x in x_values {
                { render_single_x_arc(*x, cx, cy, radius, color) }
            }
        }
    }
}

fn render_single_x_arc(x: f64, cx: f64, cy: f64, radius: f64, color: &str) -> Element {
    let points = constant_x_arc(x, 100);
    if points.is_empty() {
        return rsx! {};
    }

    let path_data = points_to_svg_path(&points, cx, cy, radius);

    rsx! {
        path {
            d: "{path_data}",
            stroke: "{color}",
            "stroke-width": "0.5",
            fill: "none",
            opacity: "0.6",
        }
    }
}

/// Render VSWR circle
fn render_vswr_circle(vswr: f64, cx: f64, cy: f64, radius: f64) -> Element {
    let gamma = (vswr - 1.0) / (vswr + 1.0);
    let r = gamma * radius;

    rsx! {
        circle {
            cx: "{cx}",
            cy: "{cy}",
            r: "{r}",
            stroke: "#ff9800",
            "stroke-width": "1",
            "stroke-dasharray": "4,2",
            fill: "none",
            opacity: "0.7",
        }
    }
}

/// Render S-parameter trace
fn render_smith_trace(trace: &SmithTrace, cx: f64, cy: f64, radius: f64) -> Element {
    if trace.data.is_empty() {
        return rsx! {};
    }

    let points: Vec<(f64, f64)> = trace
        .data
        .iter()
        .map(|s| {
            let (gx, gy) = gamma_to_smith(&s.value);
            (cx + gx * radius, cy - gy * radius) // Flip y for SVG
        })
        .collect();

    let path_data = svg_path_from_coords(&points);

    rsx! {
        path {
            d: "{path_data}",
            stroke: "{trace.color}",
            "stroke-width": "{trace.line_width}",
            fill: "none",
        }
        // Start marker
        if let Some((x, y)) = points.first() {
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "4",
                fill: "{trace.color}",
            }
        }
        // End marker
        if let Some((x, y)) = points.last() {
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "4",
                fill: "{trace.color}",
                stroke: "#fff",
                "stroke-width": "1",
            }
        }
    }
}

/// Render marker on chart
fn render_smith_marker(
    data: &SParameter,
    marker: &SmithMarker,
    cx: f64,
    cy: f64,
    radius: f64,
) -> Element {
    let (gx, gy) = gamma_to_smith(&data.value);
    let x = cx + gx * radius;
    let y = cy - gy * radius;

    rsx! {
        g { class: "marker",
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "6",
                fill: "{marker.color}",
                stroke: "#fff",
                "stroke-width": "2",
            }
            text {
                x: "{x + 10.0}",
                y: "{y - 10.0}",
                fill: "#fff",
                "font-size": "12",
                "font-weight": "bold",
                "{marker.label}"
            }
        }
    }
}

/// Render cursor
fn render_cursor(gx: f64, gy: f64, cx: f64, cy: f64, radius: f64) -> Element {
    let x = cx + gx * radius;
    let y = cy - gy * radius;

    rsx! {
        g { class: "cursor",
            // Crosshair
            line {
                x1: "{x - 10.0}",
                y1: "{y}",
                x2: "{x + 10.0}",
                y2: "{y}",
                stroke: "#fff",
                "stroke-width": "1",
            }
            line {
                x1: "{x}",
                y1: "{y - 10.0}",
                x2: "{x}",
                y2: "{y + 10.0}",
                stroke: "#fff",
                "stroke-width": "1",
            }
            circle {
                cx: "{x}",
                cy: "{y}",
                r: "4",
                fill: "none",
                stroke: "#fff",
                "stroke-width": "1",
            }
        }
    }
}

/// Render axis labels
fn render_axis_labels(cx: f64, cy: f64, radius: f64) -> Element {
    rsx! {
        g { class: "axis-labels",
            // Short circuit (left)
            text {
                x: "{cx - radius - 15.0}",
                y: "{cy + 4.0}",
                fill: "#888",
                "font-size": "11",
                "text-anchor": "end",
                "0"
            }
            // Open circuit (right)
            text {
                x: "{cx + radius + 15.0}",
                y: "{cy + 4.0}",
                fill: "#888",
                "font-size": "11",
                "text-anchor": "start",
                "∞"
            }
            // +j (top)
            text {
                x: "{cx}",
                y: "{cy - radius - 8.0}",
                fill: "#888",
                "font-size": "11",
                "text-anchor": "middle",
                "+j"
            }
            // -j (bottom)
            text {
                x: "{cx}",
                y: "{cy + radius + 16.0}",
                fill: "#888",
                "font-size": "11",
                "text-anchor": "middle",
                "-j"
            }
        }
    }
}

/// Render info panel
fn render_info_panel(state: &SmithChartState) -> Element {
    rsx! {
        div {
            class: "smith-info-panel",
            style: "margin-left: 20px; color: #fff; font-family: monospace; font-size: 12px; min-width: 180px;",

            h3 {
                style: "margin: 0 0 10px 0; color: #2196F3;",
                "Smith Chart"
            }

            div { style: "margin-bottom: 10px;",
                div { style: "color: #888;", "Reference" }
                div { "Z₀ = {state.config.z0} Ω" }
                div { "Mode: {state.config.mode.display_name()}" }
            }

            // Cursor info
            if let Some(z) = &state.cursor_impedance {
                div { style: "margin-bottom: 10px; padding: 8px; background: #333; border-radius: 4px;",
                    div { style: "color: #4CAF50;", "Cursor" }
                    div { "Z = {format_impedance(z)}" }
                    div { "VSWR = {format_vswr(z.vswr(state.config.z0))}" }
                }
            }

            // Trace info
            for (i, trace) in state.traces.iter().enumerate() {
                div { style: "margin-bottom: 8px;",
                    div { style: "color: {trace.color};", "{trace.name}" }
                    if let Some((f_min, f_max)) = trace.frequency_range() {
                        div { style: "font-size: 10px; color: #888;",
                            "{format_frequency(f_min)} - {format_frequency(f_max)}"
                        }
                    }
                }
            }
        }
    }
}

/// Convert normalized points to SVG path
fn points_to_svg_path(points: &[(f64, f64)], cx: f64, cy: f64, radius: f64) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = String::with_capacity(points.len() * 20);
    for (i, (x, y)) in points.iter().enumerate() {
        let sx = cx + x * radius;
        let sy = cy - y * radius; // Flip y for SVG
        if i == 0 {
            path.push_str(&format!("M {:.1} {:.1}", sx, sy));
        } else {
            path.push_str(&format!(" L {:.1} {:.1}", sx, sy));
        }
    }
    path
}

/// Convert coordinate pairs to SVG path
fn svg_path_from_coords(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = String::with_capacity(points.len() * 20);
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            path.push_str(&format!("M {:.1} {:.1}", x, y));
        } else {
            path.push_str(&format!(" L {:.1} {:.1}", x, y));
        }
    }
    path
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
    fn test_complex_phase() {
        let c = Complex::new(1.0, 1.0);
        assert!((c.phase() - PI / 4.0).abs() < 0.001);
    }

    #[test]
    fn test_complex_phase_deg() {
        let c = Complex::new(0.0, 1.0);
        assert!((c.phase_deg() - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_complex_from_polar() {
        let c = Complex::from_polar(1.0, PI / 2.0);
        assert!(c.re.abs() < 0.001);
        assert!((c.im - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_complex_conjugate() {
        let c = Complex::new(3.0, 4.0);
        let conj = c.conjugate();
        assert_eq!(conj.re, 3.0);
        assert_eq!(conj.im, -4.0);
    }

    #[test]
    fn test_complex_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.add(&b);
        assert_eq!(c.re, 4.0);
        assert_eq!(c.im, 6.0);
    }

    #[test]
    fn test_complex_sub() {
        let a = Complex::new(3.0, 4.0);
        let b = Complex::new(1.0, 2.0);
        let c = a.sub(&b);
        assert_eq!(c.re, 2.0);
        assert_eq!(c.im, 2.0);
    }

    #[test]
    fn test_complex_mul() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.mul(&b);
        // (1+2j)(3+4j) = 3+4j+6j+8j^2 = 3+10j-8 = -5+10j
        assert_eq!(c.re, -5.0);
        assert_eq!(c.im, 10.0);
    }

    #[test]
    fn test_complex_div() {
        let a = Complex::new(4.0, 2.0);
        let b = Complex::new(2.0, 0.0);
        let c = a.div(&b).unwrap();
        assert_eq!(c.re, 2.0);
        assert_eq!(c.im, 1.0);
    }

    #[test]
    fn test_complex_div_by_zero() {
        let a = Complex::new(1.0, 0.0);
        let b = Complex::new(0.0, 0.0);
        assert!(a.div(&b).is_none());
    }

    // -------------------------------------------------------------------------
    // Impedance Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_impedance_new() {
        let z = Impedance::new(50.0, 25.0);
        assert_eq!(z.r, 50.0);
        assert_eq!(z.x, 25.0);
    }

    #[test]
    fn test_impedance_normalize() {
        let z = Impedance::new(100.0, 50.0);
        let z_norm = z.normalize(50.0);
        assert_eq!(z_norm.r, 2.0);
        assert_eq!(z_norm.x, 1.0);
    }

    #[test]
    fn test_impedance_denormalize() {
        let z = Impedance::new(2.0, 1.0);
        let z_denorm = z.denormalize(50.0);
        assert_eq!(z_denorm.r, 100.0);
        assert_eq!(z_denorm.x, 50.0);
    }

    #[test]
    fn test_impedance_to_gamma_matched() {
        let z = Impedance::new(50.0, 0.0);
        let gamma = z.to_gamma(50.0);
        assert!(gamma.magnitude() < 0.001); // Matched load has Gamma = 0
    }

    #[test]
    fn test_impedance_to_gamma_short() {
        let z = Impedance::new(0.0, 0.0);
        let gamma = z.to_gamma(50.0);
        assert!((gamma.re + 1.0).abs() < 0.001); // Short has Gamma = -1
        assert!(gamma.im.abs() < 0.001);
    }

    #[test]
    fn test_impedance_to_gamma_open() {
        let z = Impedance::new(1e10, 0.0); // Very high impedance
        let gamma = z.to_gamma(50.0);
        assert!((gamma.re - 1.0).abs() < 0.01); // Open has Gamma ≈ 1
    }

    #[test]
    fn test_impedance_vswr() {
        let z = Impedance::new(100.0, 0.0);
        let vswr = z.vswr(50.0);
        assert!((vswr - 2.0).abs() < 0.01); // VSWR = 2 for 100 ohm load
    }

    #[test]
    fn test_impedance_to_admittance() {
        let z = Impedance::new(50.0, 0.0);
        let y = z.to_admittance();
        assert!((y.g - 0.02).abs() < 0.001); // 1/50 = 0.02 S
    }

    // -------------------------------------------------------------------------
    // Admittance Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_admittance_to_impedance() {
        let y = Admittance::new(0.02, 0.0);
        let z = y.to_impedance();
        assert!((z.r - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_admittance_normalize() {
        let y = Admittance::new(0.02, 0.01);
        let y_norm = y.normalize(0.02);
        assert_eq!(y_norm.g, 1.0);
        assert_eq!(y_norm.b, 0.5);
    }

    // -------------------------------------------------------------------------
    // S-Parameter Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_s_parameter_new() {
        let s = SParameter::new(1e9, 0.5, 0.5);
        assert_eq!(s.frequency, 1e9);
        assert_eq!(s.value.re, 0.5);
    }

    #[test]
    fn test_s_parameter_magnitude_db() {
        let s = SParameter::new(1e9, 0.1, 0.0);
        let db = s.magnitude_db();
        assert!((db - (-20.0)).abs() < 0.1); // |0.1| = -20 dB
    }

    #[test]
    fn test_s_parameter_vswr() {
        let s = SParameter::new(1e9, 0.5, 0.0);
        let vswr = s.vswr();
        assert!((vswr - 3.0).abs() < 0.01); // |Γ|=0.5 -> VSWR=3
    }

    #[test]
    fn test_s_parameter_return_loss() {
        let s = SParameter::new(1e9, 0.1, 0.0);
        let rl = s.return_loss_db();
        assert!((rl - 20.0).abs() < 0.1);
    }

    // -------------------------------------------------------------------------
    // Coordinate Conversion Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_gamma_to_impedance_center() {
        let gamma = Complex::new(0.0, 0.0);
        let z = gamma_to_impedance(&gamma, 50.0);
        assert!((z.r - 50.0).abs() < 0.1);
        assert!(z.x.abs() < 0.1);
    }

    #[test]
    fn test_gamma_to_impedance_short() {
        let gamma = Complex::new(-1.0, 0.0);
        let z = gamma_to_impedance(&gamma, 50.0);
        assert!(z.r.abs() < 0.1);
    }

    #[test]
    fn test_impedance_gamma_roundtrip() {
        let z_orig = Impedance::new(75.0, 25.0);
        let gamma = z_orig.to_gamma(50.0);
        let z_back = gamma_to_impedance(&gamma, 50.0);
        assert!((z_orig.r - z_back.r).abs() < 0.1);
        assert!((z_orig.x - z_back.x).abs() < 0.1);
    }

    // -------------------------------------------------------------------------
    // Circle Generation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_constant_r_circle_r0() {
        let points = constant_r_circle(0.0, 100);
        assert!(!points.is_empty());
        // R=0 circle passes through Gamma=-1
        let has_minus_one = points.iter().any(|(x, _)| (*x + 1.0).abs() < 0.1);
        assert!(has_minus_one);
    }

    #[test]
    fn test_constant_r_circle_r1() {
        let points = constant_r_circle(1.0, 100);
        assert!(!points.is_empty());
        // R=1 circle passes through Gamma=0
        let has_center = points.iter().any(|(x, y)| x.abs() < 0.1 && y.abs() < 0.1);
        assert!(has_center);
    }

    #[test]
    fn test_constant_x_arc_positive() {
        let points = constant_x_arc(1.0, 50);
        assert!(!points.is_empty());
        // X=1 arc should be in upper half
        let all_upper = points.iter().all(|(_, y)| *y >= -0.1);
        assert!(all_upper);
    }

    #[test]
    fn test_constant_x_arc_negative() {
        let points = constant_x_arc(-1.0, 50);
        assert!(!points.is_empty());
        // X=-1 arc should be in lower half
        let all_lower = points.iter().all(|(_, y)| *y <= 0.1);
        assert!(all_lower);
    }

    #[test]
    fn test_vswr_circle() {
        let points = vswr_circle(2.0, 50);
        assert!(!points.is_empty());
        // VSWR=2 means |Gamma|=1/3
        let expected_radius = 1.0 / 3.0;
        for (x, y) in &points {
            let r = (x * x + y * y).sqrt();
            assert!((r - expected_radius).abs() < 0.01);
        }
    }

    #[test]
    fn test_vswr_circle_invalid() {
        let points = vswr_circle(0.5, 50); // Invalid VSWR < 1
        assert!(points.is_empty());
    }

    // -------------------------------------------------------------------------
    // State Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_smith_chart_config_default() {
        let config = SmithChartConfig::default();
        assert_eq!(config.z0, 50.0);
        assert!(config.show_r_circles);
    }

    #[test]
    fn test_smith_chart_state_new() {
        let state = SmithChartState::new();
        assert!(state.traces.is_empty());
        assert_eq!(state.config.z0, 50.0);
    }

    #[test]
    fn test_smith_chart_add_trace() {
        let mut state = SmithChartState::new();
        let trace = SmithTrace::new("S11", vec![]);
        state.add_trace(trace);
        assert_eq!(state.traces.len(), 1);
        assert_eq!(state.active_trace, Some(0));
    }

    #[test]
    fn test_smith_chart_set_cursor_inside() {
        let mut state = SmithChartState::new();
        state.set_cursor(0.5, 0.5);
        assert!(state.cursor.is_some());
        assert!(state.cursor_impedance.is_some());
    }

    #[test]
    fn test_smith_chart_set_cursor_outside() {
        let mut state = SmithChartState::new();
        state.set_cursor(1.5, 0.0); // Outside unit circle
        assert!(state.cursor.is_none());
    }

    #[test]
    fn test_smith_trace_add_marker() {
        let s = SParameter::new(1e9, 0.5, 0.0);
        let mut trace = SmithTrace::new("S11", vec![s]);
        trace.add_marker(0);
        assert_eq!(trace.markers.len(), 1);
    }

    #[test]
    fn test_smith_trace_frequency_range() {
        let s1 = SParameter::new(1e9, 0.5, 0.0);
        let s2 = SParameter::new(2e9, 0.3, 0.0);
        let trace = SmithTrace::new("S11", vec![s1, s2]);
        let (min, max) = trace.frequency_range().unwrap();
        assert_eq!(min, 1e9);
        assert_eq!(max, 2e9);
    }

    #[test]
    fn test_smith_chart_mode_all() {
        assert_eq!(SmithChartMode::all().len(), 3);
    }

    // -------------------------------------------------------------------------
    // Formatting Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_impedance() {
        let z = Impedance::new(50.0, 25.0);
        let s = format_impedance(&z);
        assert!(s.contains("50"));
        assert!(s.contains("25"));
        assert!(s.contains("Ω"));
    }

    #[test]
    fn test_format_impedance_negative_x() {
        let z = Impedance::new(50.0, -25.0);
        let s = format_impedance(&z);
        assert!(s.contains("-"));
    }

    #[test]
    fn test_format_frequency_ghz() {
        let s = format_frequency(2.4e9);
        assert!(s.contains("GHz"));
    }

    #[test]
    fn test_format_frequency_mhz() {
        let s = format_frequency(433e6);
        assert!(s.contains("MHz"));
    }

    #[test]
    fn test_format_frequency_khz() {
        let s = format_frequency(100e3);
        assert!(s.contains("kHz"));
    }

    #[test]
    fn test_format_vswr() {
        let s = format_vswr(2.0);
        assert!(s.contains("2.00:1"));
    }

    #[test]
    fn test_format_vswr_infinity() {
        let s = format_vswr(f64::INFINITY);
        assert!(s.contains("∞"));
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
    fn test_smith_config_roundtrip() {
        let config = SmithChartConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: SmithChartConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn test_smith_state_roundtrip() {
        let state = SmithChartState::new();
        let json = serde_json::to_string(&state).unwrap();
        let parsed: SmithChartState = serde_json::from_str(&json).unwrap();
        assert_eq!(state, parsed);
    }
}
