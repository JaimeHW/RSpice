//! Smith chart viewer state: reference impedance and S-parameter traces.
//!
//! This is deliberately only what `result_document::smith` reads. The viewer
//! draws the chart itself and converts Γ to impedance inline, so nothing here
//! computes geometry.
//!
//! What used to live here — chart modes (Z / Y / combined), markers with VSWR
//! and return-loss readout, VSWR circles, constant-R/X circle math, and a
//! hand-rolled complex type with an impedance/admittance pair on top of it —
//! was ~900 lines that no code path reached. Rebuilding any of it belongs with
//! the viewer feature that needs it, against `num_complex`.

use num_complex::Complex64;

/// Default reference impedance in ohms.
pub const Z0_DEFAULT: f64 = 50.0;

/// One S-parameter sample: a frequency and the complex value there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SParamPoint {
    /// Frequency in Hz
    pub frequency: f64,
    /// Complex S-parameter value
    pub s: Complex64,
}

/// A named S-parameter trace drawn on the chart.
#[derive(Debug, Clone)]
pub struct SParamTrace {
    /// Trace name (e.g. "S11", "S22")
    pub name: String,
    /// Data points
    pub points: Vec<SParamPoint>,
    /// Whether the viewer draws this trace and lists it in the legend.
    pub visible: bool,
}

impl SParamTrace {
    /// Create an empty, visible trace.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
            visible: true,
        }
    }
}

/// Smith chart viewer state.
#[derive(Debug, Clone)]
pub struct SmithChartState {
    /// Reference impedance (default 50Ω)
    pub z0: f64,
    /// S-parameter traces
    pub traces: Vec<SParamTrace>,
}

impl Default for SmithChartState {
    fn default() -> Self {
        Self {
            z0: Z0_DEFAULT,
            traces: Vec::new(),
        }
    }
}

impl SmithChartState {
    /// Drop every trace.
    pub fn clear_traces(&mut self) {
        self.traces.clear();
    }

    /// Append a trace from a run's S-parameter arrays.
    ///
    /// The three arrays are zipped to the shortest of them rather than
    /// asserted equal: a truncated result should draw what it has instead of
    /// panicking inside the viewer.
    pub fn load_sparam_data(
        &mut self,
        name: &str,
        frequencies: &[f64],
        s_real: &[f64],
        s_imag: &[f64],
    ) {
        let mut trace = SParamTrace::new(name);
        let n = frequencies.len().min(s_real.len()).min(s_imag.len());
        trace.points.reserve(n);

        for i in 0..n {
            trace.points.push(SParamPoint {
                frequency: frequencies[i],
                s: Complex64::new(s_real[i], s_imag[i]),
            });
        }

        self.traces.push(trace);
    }
}
