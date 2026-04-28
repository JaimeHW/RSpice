//! Waveform Calculator Engine
//!
//! A commercial-grade expression evaluator for simulation results.
//! Supports:
//! - Algebraic operations on waveforms and scalars
//! - Signal processing functions (deriv, integ, clip, etc.)
//! - Measurement functions (rise_time, bandwidth, etc.)
//! - Vector arithmetic handling
//! - Automatic interpolation for mismatched time bases

pub mod ast;
pub mod evaluator;
pub mod functions;
pub mod interpolation;
pub mod parser;

pub use ast::CalculatorExpr;
pub use evaluator::{CalcValue, EvaluationContext, EvaluationError};
pub use interpolation::{
    ExtrapolationMode, InterpolationError, InterpolationMethod, WaveformInterpolator,
    align_waveforms, align_waveforms_union,
};

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
        let signal_lower = signal.to_lowercase();

        // Try exact match first
        if let Some(wf) = self
            .simulation
            .waveforms
            .iter()
            .find(|wf| wf.name.eq_ignore_ascii_case(signal))
        {
            return Some(wf);
        }

        // Try matching as net name inside V() or I()
        self.simulation.waveforms.iter().find(|wf| {
            let wf_net = wf
                .name
                .trim_start_matches("V(")
                .trim_start_matches("I(")
                .trim_end_matches(')')
                .to_lowercase();
            wf_net == signal_lower
        })
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

