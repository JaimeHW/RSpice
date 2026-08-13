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
    /// Reference impedance for a physical reflection trace. Transmission and
    /// mixed-mode loci deliberately carry none and cannot claim Z or VSWR.
    pub reference_impedance_ohm: Option<f64>,
}

impl SParamTrace {
    /// Create an empty, visible trace.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
            visible: true,
            reference_impedance_ohm: None,
        }
    }
}

/// Smith chart viewer state.
#[derive(Debug, Clone)]
pub struct SmithChartState {
    /// S-parameter traces
    pub traces: Vec<SParamTrace>,
}

impl Default for SmithChartState {
    fn default() -> Self {
        Self { traces: Vec::new() }
    }
}

impl SmithChartState {
    /// Drop every trace.
    pub fn clear_traces(&mut self) {
        self.traces.clear();
    }

    /// Append a trace from a run's S-parameter arrays.
    ///
    /// Malformed arrays fail closed. A viewer must never turn truncated or
    /// non-finite retained evidence into a plausible-looking RF locus.
    pub fn load_sparam_data(
        &mut self,
        name: &str,
        frequencies: &[f64],
        s_real: &[f64],
        s_imag: &[f64],
        reference_impedance_ohm: Option<f64>,
    ) -> Result<(), String> {
        if name.trim().is_empty() || name.chars().any(char::is_control) {
            return Err("Smith trace name must be non-empty and printable".to_owned());
        }
        if frequencies.is_empty()
            || frequencies.len() != s_real.len()
            || frequencies.len() != s_imag.len()
        {
            return Err(
                "Smith frequency, real, and imaginary arrays must be non-empty and equal length"
                    .to_owned(),
            );
        }
        if frequencies
            .iter()
            .any(|frequency| !frequency.is_finite() || *frequency <= 0.0)
            || !frequencies.windows(2).all(|pair| pair[0] < pair[1])
        {
            return Err(
                "Smith frequencies must be finite, positive, and strictly increasing".to_owned(),
            );
        }
        if s_real.iter().chain(s_imag).any(|value| !value.is_finite()) {
            return Err("Smith coefficients must be finite".to_owned());
        }
        if reference_impedance_ohm
            .is_some_and(|impedance| !impedance.is_finite() || impedance <= 0.0)
        {
            return Err("Smith reference impedance must be finite and positive".to_owned());
        }

        let mut trace = SParamTrace::new(name);
        trace.reference_impedance_ohm = reference_impedance_ohm;
        trace.points.reserve(frequencies.len());

        for i in 0..frequencies.len() {
            trace.points.push(SParamPoint {
                frequency: frequencies[i],
                s: Complex64::new(s_real[i], s_imag[i]),
            });
        }

        self.traces.push(trace);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_arrays_are_rejected_without_partial_trace() {
        let mut state = SmithChartState::default();
        assert!(
            state
                .load_sparam_data("S11", &[1.0, 2.0], &[0.25], &[0.0, 0.1], Some(75.0))
                .is_err()
        );
        assert!(state.traces.is_empty());
    }

    #[test]
    fn retained_reference_impedance_belongs_to_the_exact_trace() {
        let mut state = SmithChartState::default();
        state
            .load_sparam_data("S22", &[1.0], &[0.2], &[-0.1], Some(75.0))
            .expect("valid reflection trace");
        assert_eq!(state.traces[0].reference_impedance_ohm, Some(75.0));
    }
}
