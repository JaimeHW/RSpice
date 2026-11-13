//! Retained traces for the Smith chart presentation and the reader's
//! controls for the sheet.

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
    /// A complex response of an AC or harmonic-balance analysis rather than
    /// a network coefficient. Its samples are the response itself; the pane
    /// reads them as Γ, Z or Y against the pane's own Z₀.
    pub response: bool,
}

impl SParamTrace {
    /// Create an empty, visible trace.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
            visible: true,
            reference_impedance_ohm: None,
            response: false,
        }
    }
}

/// How a complex response that is not a network coefficient lands on the
/// chart.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ResponseReading {
    /// The response is a reflection coefficient and is drawn as it is.
    #[default]
    Reflection,
    /// The response is an impedance in ohms: Γ = (Z − Z₀)/(Z + Z₀).
    Impedance,
    /// The response is an admittance in siemens: Γ = (1 − Y·Z₀)/(1 + Y·Z₀).
    Admittance,
}

impl ResponseReading {
    /// Segmented-control labels, in [`Self::index`] order.
    pub const OPTIONS: [&'static str; 3] = ["Γ", "Z", "Y"];

    pub const fn index(self) -> usize {
        match self {
            Self::Reflection => 0,
            Self::Impedance => 1,
            Self::Admittance => 2,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Impedance,
            2 => Self::Admittance,
            _ => Self::Reflection,
        }
    }

    /// The reflection coefficient a response sample stands for against
    /// `reference_ohm`. A response at the chart's singular point (Z = −Z₀)
    /// is not finite and is not drawn.
    pub fn reflection(self, value: Complex64, reference_ohm: f64) -> Complex64 {
        match self {
            Self::Reflection => value,
            Self::Impedance => (value - reference_ohm) / (value + reference_ohm),
            Self::Admittance => {
                let normalized = value * reference_ohm;
                (1.0 - normalized) / (1.0 + normalized)
            }
        }
    }
}

/// Which Smith-chart circles a two-port S-parameter sheet draws.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SmithCircleSet {
    /// Source- and load-plane stability circles.
    pub stability: bool,
    /// Source-plane constant available-gain circles.
    pub available_gain: bool,
    /// Load-plane constant operating-power-gain circles.
    pub power_gain: bool,
    /// Source-plane constant noise-figure circles; offered only when the run
    /// retained the two-port noise parameters.
    pub noise: bool,
}

impl SmithCircleSet {
    pub const fn any(self) -> bool {
        self.stability || self.available_gain || self.power_gain || self.noise
    }
}

/// The reader's controls for one Smith sheet.
#[derive(Debug, Clone)]
pub struct SmithSheetState {
    pub grid: super::grid::SmithGrid,
    /// How an AC or harmonic-balance response is read onto the chart.
    pub reading: ResponseReading,
    /// The pane's reference impedance for responses, in ohms.
    pub reference_ohm: f64,
    /// The Z₀ field's text while the reader is editing it.
    pub reference_draft: Option<String>,
    /// The responses an AC or harmonic-balance sheet draws, in order, as
    /// calculator expressions. A retained trace name is the simplest one;
    /// `V(in)/I(Vin)` is an input impedance.
    pub responses: Vec<String>,
    /// The expression field's text while it is open.
    pub expression_draft: Option<String>,
    /// Why the last expression the reader entered was refused.
    pub expression_error: Option<String>,
    /// Network terms the reader switched off, by retained name.
    pub hidden: std::collections::BTreeSet<String>,
    pub circles: SmithCircleSet,
    /// Gain-circle values in dB; empty means the three whole decibels at
    /// and below the maximum gain.
    pub gain_db: Vec<f64>,
    /// Noise-circle values in dB; empty means three half-decibel steps from
    /// the first half decibel above NFmin.
    pub noise_db: Vec<f64>,
    /// The gain or noise value field's text while it is being edited.
    pub gain_draft: Option<String>,
    pub noise_draft: Option<String>,
    /// The two-port the sheet resolved for the analysis it last drew, keyed
    /// by that analysis' cache owner, so a frame does not re-resolve four
    /// retained columns to place its circles.
    pub two_port_cache: Option<(
        u64,
        Option<std::sync::Arc<rspice_results::network_matrix::two_port::TwoPortSweep>>,
    )>,
}

impl Default for SmithSheetState {
    fn default() -> Self {
        Self {
            grid: super::grid::SmithGrid::default(),
            reading: ResponseReading::default(),
            reference_ohm: 50.0,
            reference_draft: None,
            responses: Vec::new(),
            expression_draft: None,
            expression_error: None,
            hidden: std::collections::BTreeSet::new(),
            circles: SmithCircleSet::default(),
            gain_db: Vec::new(),
            noise_db: Vec::new(),
            gain_draft: None,
            noise_draft: None,
            two_port_cache: None,
        }
    }
}

/// Smith chart viewer state.
#[derive(Debug, Clone, Default)]
pub struct SmithChartState {
    /// S-parameter traces
    pub traces: Vec<SParamTrace>,
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
        self.load(
            name,
            frequencies,
            s_real,
            s_imag,
            reference_impedance_ohm,
            false,
        )
    }

    /// Append a complex AC or harmonic-balance response, which the pane
    /// reads against its own Z₀. Fails closed exactly as
    /// [`Self::load_sparam_data`] does.
    pub fn load_response_data(
        &mut self,
        name: &str,
        frequencies: &[f64],
        real: &[f64],
        imag: &[f64],
    ) -> Result<(), String> {
        self.load(name, frequencies, real, imag, None, true)
    }

    fn load(
        &mut self,
        name: &str,
        frequencies: &[f64],
        s_real: &[f64],
        s_imag: &[f64],
        reference_impedance_ohm: Option<f64>,
        response: bool,
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
        trace.response = response;
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

    /// A response read as Z or Y lands where that impedance belongs on a
    /// chart of the pane's Z₀, and reads back as the same impedance.
    #[test]
    fn a_response_read_as_an_impedance_lands_at_its_reflection() {
        let reference = 50.0;
        let at = |reading: ResponseReading, value: Complex64| reading.reflection(value, reference);
        assert_eq!(
            at(ResponseReading::Impedance, Complex64::new(50.0, 0.0)),
            Complex64::new(0.0, 0.0)
        );
        assert!((at(ResponseReading::Impedance, Complex64::new(150.0, 0.0)) - 0.5).norm() < 1e-15);
        assert!((at(ResponseReading::Admittance, Complex64::new(0.02, 0.0))).norm() < 1e-15);
        let impedance = Complex64::new(23.0, -41.0);
        for (reading, value) in [
            (ResponseReading::Impedance, impedance),
            (ResponseReading::Admittance, 1.0 / impedance),
        ] {
            let gamma = at(reading, value);
            let (resistance, reactance) =
                rspice_results::network_matrix::impedance_from_gamma(gamma.re, gamma.im, reference)
                    .expect("finite impedance");
            assert!((resistance - 23.0).abs() < 1e-12 && (reactance + 41.0).abs() < 1e-12);
        }
        let coefficient = Complex64::new(0.3, -0.2);
        assert_eq!(at(ResponseReading::Reflection, coefficient), coefficient);
    }

    #[test]
    fn a_response_trace_carries_no_retained_reference() {
        let mut state = SmithChartState::default();
        state
            .load_response_data("V(in)", &[1.0, 2.0], &[0.2, 0.3], &[-0.1, 0.0])
            .expect("valid response");
        assert!(state.traces[0].response);
        assert_eq!(state.traces[0].reference_impedance_ohm, None);
    }
}
