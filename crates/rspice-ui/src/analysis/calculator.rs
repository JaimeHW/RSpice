//! Waveform Calculator Engine
//!
//! A commercial-grade expression evaluator for simulation results.
//! Supports:
//! - Algebraic operations on waveforms and scalars
//! - Signal processing functions (deriv, integ, clip, etc.)
//! - Measurement functions (rise_time, bandwidth, etc.)
//! - Vector arithmetic handling
//! - Automatic interpolation for mismatched time bases

pub(crate) mod ast;
pub(crate) mod evaluator;
pub(crate) mod functions;
pub(crate) mod interpolation;
pub(crate) mod parser;

pub use evaluator::{CalcValue, EvaluationContext, EvaluationError};

// =============================================================================
// Simulation Context Adapter
// =============================================================================
//
// Bridges SimulationState waveforms to the calculator EvaluationContext trait.
// This allows expressions like "V(out) * 2" to resolve V(out) from simulation.

use crate::state::{SimulationState, WaveformData};

/// Evaluation context backed by simulation results.
///
/// Implements the `EvaluationContext` trait to provide waveform data
/// from `SimulationState` to the calculator evaluator.
///
/// # Example Usage
///
/// ```ignore
/// let ctx = SimulationContext::new(&app_state.simulation);
/// let result = evaluator::evaluate(&parsed_expr, &ctx)?;
/// ```
pub struct SimulationContext<'a> {
    /// Reference to simulation state containing waveforms
    simulation: &'a SimulationState,
}

impl<'a> SimulationContext<'a> {
    /// Create a new evaluation context from simulation state
    pub fn new(simulation: &'a SimulationState) -> Self {
        Self { simulation }
    }

    /// Convert WaveformData to CalcValue format
    fn waveform_to_calc_value(wf: &WaveformData) -> CalcValue {
        // Convert x/y vectors from Value (f64) to Vec<f64>
        let x: Vec<f64> = wf.x.to_vec();
        let y: Vec<f64> = wf.y.to_vec();
        CalcValue::create_waveform(x, y)
    }

    /// Find a waveform by signal name with flexible matching
    ///
    /// Supports several naming conventions:
    /// - Exact match: "V(out)" matches "V(out)"
    /// - Wrapped match: "out" matches "V(out)" or "I(out)"
    /// - Case-insensitive matching
    fn find_waveform(&self, signal: &str) -> Option<&WaveformData> {
        find_in(&self.simulation.waveforms, signal)
    }
}

/// Evaluation context backed by one analysis' waveform list — used by the
/// Results workspace, where each strip evaluates expressions against its
/// own analysis instead of the live (active-analysis) waveform set.
pub struct WaveformsContext<'a> {
    waveforms: &'a [WaveformData],
}

impl<'a> WaveformsContext<'a> {
    /// Wrap an analysis' waveforms.
    pub fn new(waveforms: &'a [WaveformData]) -> Self {
        Self { waveforms }
    }
}

/// Find a waveform by signal name with flexible matching: exact name,
/// net name inside `V()`/`I()`, and AC magnitude entries (`|V(out)|`
/// matches `V(out)` so `dB(V(out)/V(in))` works on AC strips).
fn find_in<'a>(waveforms: &'a [WaveformData], signal: &str) -> Option<&'a WaveformData> {
    if let Some(wf) = waveforms
        .iter()
        .find(|wf| wf.name.eq_ignore_ascii_case(signal))
    {
        return Some(wf);
    }

    // `|V(out)|` (AC magnitude) matches a request for `V(out)`.
    if let Some(wf) = waveforms
        .iter()
        .find(|wf| wf.name.trim_matches('|').eq_ignore_ascii_case(signal))
    {
        return Some(wf);
    }

    // Bare net name matches inside V() / I() wrappers.
    waveforms.iter().find(|wf| {
        bare_wrapped_signal_name(&wf.name)
            .is_some_and(|wrapped| wrapped.eq_ignore_ascii_case(signal))
    })
}

/// Return the body of a voltage/current wrapper without assuming the producer
/// used uppercase `V`/`I`. AC magnitude traces retain the same signal spelling
/// inside a symmetric pair of bars.
fn bare_wrapped_signal_name(name: &str) -> Option<&str> {
    let name = name
        .strip_prefix('|')
        .and_then(|inner| inner.strip_suffix('|'))
        .unwrap_or(name);
    let prefix = name.get(..2)?;
    if !prefix.eq_ignore_ascii_case("V(") && !prefix.eq_ignore_ascii_case("I(") {
        return None;
    }
    name.strip_suffix(')')?.get(2..)
}

impl<'a> EvaluationContext for WaveformsContext<'a> {
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        let _ = dataset;
        match signal.to_uppercase().as_str() {
            "TIME" | "T" | "FREQ" | "FREQUENCY" => {
                if let Some(wf) = self.waveforms.first() {
                    let x: Vec<f64> = wf.x.to_vec();
                    let y = x.clone();
                    return Ok(CalcValue::create_waveform(x, y));
                }
                return Err(EvaluationError::IdentifierNotFound(format!(
                    "No waveforms available for {signal}"
                )));
            }
            _ => {}
        }
        match find_in(self.waveforms, signal) {
            Some(wf) => Ok(SimulationContext::waveform_to_calc_value(wf)),
            None => Err(EvaluationError::IdentifierNotFound(signal.to_string())),
        }
    }
}

impl<'a> EvaluationContext for SimulationContext<'a> {
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        // Handle dataset selection (for multi-run results)
        // For now, we use the active waveforms in simulation state
        // Future: support dataset like "run1:V(out)" or selecting from runs
        let _ = dataset; // Reserved for future multi-run support

        // Handle special constants
        match signal.to_uppercase().as_str() {
            "TIME" | "T" => {
                // Look for any transient waveform and return its X axis as time
                if let Some(wf) = self.simulation.waveforms.first() {
                    let x: Vec<f64> = wf.x.to_vec();
                    let y = x.clone(); // TIME returns x as both x and y
                    return Ok(CalcValue::create_waveform(x, y));
                }
                return Err(EvaluationError::IdentifierNotFound(
                    "No waveforms available for TIME constant".to_string(),
                ));
            }
            "FREQ" | "FREQUENCY" => {
                // Look for AC waveform and return its X axis as frequency
                if let Some(wf) = self.simulation.waveforms.first() {
                    let x: Vec<f64> = wf.x.to_vec();
                    let y = x.clone();
                    return Ok(CalcValue::create_waveform(x, y));
                }
                return Err(EvaluationError::IdentifierNotFound(
                    "No waveforms available for FREQ constant".to_string(),
                ));
            }
            _ => {}
        }

        // Find the waveform by signal name
        match self.find_waveform(signal) {
            Some(wf) => Ok(Self::waveform_to_calc_value(wf)),
            None => Err(EvaluationError::IdentifierNotFound(signal.to_string())),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn waveform(name: &str, value: f64) -> WaveformData {
        WaveformData::new(name, vec![0.0, 1.0], vec![value, value], "#ffffff")
    }

    fn expected(value: f64) -> CalcValue {
        CalcValue::Waveform(vec![0.0, 1.0], vec![value, value])
    }

    #[test]
    fn live_context_resolves_case_insensitive_wrappers_from_bare_names() {
        let simulation = SimulationState {
            waveforms: vec![waveform("v(OUT)", 1.25), waveform("i(VdD)", 2.5)],
            ..SimulationState::default()
        };
        let context = SimulationContext::new(&simulation);

        assert_eq!(context.get_waveform("oUt", None).unwrap(), expected(1.25));
        assert_eq!(context.get_waveform("VDD", None).unwrap(), expected(2.5));
    }

    #[test]
    fn per_analysis_context_resolves_case_insensitive_wrappers_from_bare_names() {
        let waveforms = vec![waveform("|v(OUT)|", 3.75), waveform("i(VdD)", 5.0)];
        let context = WaveformsContext::new(&waveforms);

        assert_eq!(context.get_waveform("out", None).unwrap(), expected(3.75));
        assert_eq!(context.get_waveform("vdd", None).unwrap(), expected(5.0));
    }
}
