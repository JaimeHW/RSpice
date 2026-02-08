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
    align_waveforms, align_waveforms_union, ExtrapolationMode, InterpolationError,
    InterpolationMethod, WaveformInterpolator,
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
        let x: Vec<f64> = wf.x.iter().map(|v| *v as f64).collect();
        let y: Vec<f64> = wf.y.iter().map(|v| *v as f64).collect();
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
                    let x: Vec<f64> = wf.x.iter().map(|v| *v as f64).collect();
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
                    let x: Vec<f64> = wf.x.iter().map(|v| *v as f64).collect();
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
    use crate::state::WaveformData;
    use rspice_core::Value;

    fn create_test_simulation_state() -> SimulationState {
        let mut state = SimulationState::default();

        // Add some test waveforms
        state.waveforms.push(WaveformData::new(
            "V(out)",
            vec![Value::from(0.0), Value::from(1.0), Value::from(2.0)],
            vec![Value::from(0.0), Value::from(0.5), Value::from(1.0)],
            "#ff0000",
        ));

        state.waveforms.push(WaveformData::new(
            "V(in)",
            vec![Value::from(0.0), Value::from(1.0), Value::from(2.0)],
            vec![Value::from(1.0), Value::from(1.0), Value::from(1.0)],
            "#00ff00",
        ));

        state.waveforms.push(WaveformData::new(
            "I(R1)",
            vec![Value::from(0.0), Value::from(1.0), Value::from(2.0)],
            vec![Value::from(0.001), Value::from(0.002), Value::from(0.003)],
            "#0000ff",
        ));

        state
    }

    #[test]
    fn test_simulation_context_exact_match() {
        let sim = create_test_simulation_state();
        let ctx = SimulationContext::new(&sim);

        let result = ctx.get_waveform("V(out)", None);
        assert!(result.is_ok(), "Should find V(out) by exact match");

        let result = ctx.get_waveform("I(R1)", None);
        assert!(result.is_ok(), "Should find I(R1) by exact match");
    }

    #[test]
    fn test_simulation_context_net_name_match() {
        let sim = create_test_simulation_state();
        let ctx = SimulationContext::new(&sim);

        // Looking for "out" should match "V(out)"
        let result = ctx.get_waveform("out", None);
        assert!(result.is_ok(), "Should find 'out' matching V(out)");

        // Looking for "in" should match "V(in)"
        let result = ctx.get_waveform("in", None);
        assert!(result.is_ok(), "Should find 'in' matching V(in)");
    }

    #[test]
    fn test_simulation_context_case_insensitive() {
        let sim = create_test_simulation_state();
        let ctx = SimulationContext::new(&sim);

        let result = ctx.get_waveform("v(OUT)", None);
        assert!(result.is_ok(), "Should find V(out) case-insensitively");

        let result = ctx.get_waveform("OUT", None);
        assert!(result.is_ok(), "Should find 'out' case-insensitively");
    }

    #[test]
    fn test_simulation_context_not_found() {
        let sim = create_test_simulation_state();
        let ctx = SimulationContext::new(&sim);

        let result = ctx.get_waveform("V(nonexistent)", None);
        assert!(
            result.is_err(),
            "Should return error for nonexistent signal"
        );

        match result {
            Err(EvaluationError::IdentifierNotFound(name)) => {
                assert_eq!(name, "V(nonexistent)");
            }
            _ => panic!("Expected IdentifierNotFound error"),
        }
    }

    #[test]
    fn test_simulation_context_time_constant() {
        let sim = create_test_simulation_state();
        let ctx = SimulationContext::new(&sim);

        let result = ctx.get_waveform("TIME", None);
        assert!(
            result.is_ok(),
            "Should find TIME constant if waveforms exist"
        );

        // Also test lowercase
        let result = ctx.get_waveform("time", None);
        assert!(
            result.is_ok(),
            "Should find time constant case-insensitively"
        );
    }

    #[test]
    fn test_simulation_context_empty_state() {
        let sim = SimulationState::default();
        let ctx = SimulationContext::new(&sim);

        let result = ctx.get_waveform("V(out)", None);
        assert!(result.is_err(), "Should return error when no waveforms");

        let result = ctx.get_waveform("TIME", None);
        assert!(result.is_err(), "TIME should fail when no waveforms");
    }

    #[test]
    fn test_simulation_context_waveform_conversion() {
        let sim = create_test_simulation_state();
        let ctx = SimulationContext::new(&sim);

        let result = ctx.get_waveform("V(out)", None).unwrap();

        match result {
            CalcValue::Waveform(x, y) => {
                assert_eq!(x.len(), 3, "X should have 3 points");
                assert_eq!(y.len(), 3, "Y should have 3 points");
                assert_eq!(x[0], 0.0);
                assert_eq!(x[2], 2.0);
                assert_eq!(y[0], 0.0);
                assert_eq!(y[2], 1.0);
            }
            _ => panic!("Expected Waveform result"),
        }
    }
}
