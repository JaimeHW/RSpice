//! Waveform Calculator Engine
//!
//! A commercial-grade expression evaluator for simulation results.
//! Supports:
//! - Algebraic operations on waveforms and scalars
//! - Signal processing functions (deriv, integ, clip, etc.)
//! - Measurement functions (rise_time, bandwidth, etc.)
//! - Vector arithmetic handling
//! - Automatic interpolation for mismatched time bases

pub(crate) use rspice_results::calculator::spice_parser as parser;
pub(crate) use rspice_results::calculator::{ast, evaluator, functions, value};

use rspice_results::calculator::retained::{
    canonical_ground_value, find_waveform, reject_unbound_dataset,
};
pub(crate) use rspice_results::calculator::retained::{magnitude_value, waveform_value};
use rspice_results::waveform::RetainedWaveform;

pub use evaluator::{CalcValue, EvaluationContext, EvaluationError};
pub use value::{ComplexValue, RealValue};

/// Evaluation context over the application's decorated retained waveforms.
pub type WaveformsContext<'a> =
    rspice_results::calculator::retained::WaveformsContext<'a, WaveformData>;

/// Attach application presentation to an exact derived waveform.
pub(crate) fn evaluated_waveform(
    value: CalcValue,
    name: &str,
    scalar_axis: Option<&[f64]>,
) -> Result<WaveformData, String> {
    Ok(WaveformData {
        data: rspice_results::calculator::retained::evaluated_waveform(value, name, scalar_axis)?,
        color: "#f5b700".to_owned(),
        visible: true,
        display_cache: None,
    })
}

// =============================================================================
// Simulation Context Adapter
// =============================================================================
//
// Bridges SimulationState waveforms to the calculator EvaluationContext trait.
// This allows expressions like "V(out) * 2" to resolve V(out) from simulation.

use crate::state::{ComplexExpressionPolicy, SimulationState, WaveformData};

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
    complex_policy: ComplexExpressionPolicy,
}

impl<'a> SimulationContext<'a> {
    /// Create a new evaluation context from simulation state
    pub fn new(simulation: &'a SimulationState) -> Self {
        Self {
            simulation,
            complex_policy: ComplexExpressionPolicy::Rectangular,
        }
    }

    /// Find a waveform by signal name with flexible matching
    ///
    /// Supports several naming conventions:
    /// - Exact match: "V(out)" matches "V(out)"
    /// - Wrapped match: "out" matches "V(out)" or "I(out)"
    /// - Case-insensitive matching
    fn find_waveform(&self, signal: &str) -> Option<&RetainedWaveform> {
        find_waveform(&self.simulation.waveforms, signal)
    }
}

impl<'a> EvaluationContext for SimulationContext<'a> {
    fn get_magnitude(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        magnitude_value(
            self.get_waveform(signal, dataset),
            self.find_waveform(signal),
        )
    }

    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        reject_unbound_dataset(dataset)?;

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

        if let Some(value) =
            canonical_ground_value(signal, self.simulation.waveforms.first().map(AsRef::as_ref))
        {
            return value;
        }

        // Find the waveform by signal name
        match self.find_waveform(signal) {
            Some(wf) => waveform_value(wf, self.complex_policy),
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

    #[test]
    fn complex_missing_phase_allows_only_explicit_magnitude_projections() {
        let simulation = SimulationState {
            waveforms: vec![waveform("|V(out)|", 2.0)],
            ..Default::default()
        };
        let live = SimulationContext::new(&simulation);
        let retained = WaveformsContext::new(&simulation.waveforms);
        for context in [&live as &dyn EvaluationContext, &retained] {
            assert!(matches!(
                context.get_waveform("V(out)", None),
                Err(EvaluationError::PhaseUnavailable(_))
            ));
            assert_eq!(
                context.get_magnitude("V(out)", None).unwrap(),
                expected(2.0)
            );
            assert!(context.get_magnitude("V(out)", Some("other-run")).is_err());
        }
        for text in [
            "mag(V(out))",
            "abs(V(out))",
            "magnitude(V(out))",
            "dB(V(out))",
        ] {
            assert!(
                evaluator::evaluate(&parser::try_parse(text).unwrap(), &retained).is_ok(),
                "{text}"
            );
        }
        for text in [
            "V(out)*2",
            "real(V(out))",
            "phase(V(out))",
            "mag(V(out)-V(out))",
        ] {
            assert!(
                evaluator::evaluate(&parser::try_parse(text).unwrap(), &retained).is_err(),
                "{text}"
            );
        }
    }

    #[test]
    fn complex_components_must_match_the_retained_axis() {
        let mut wave =
            waveform("|V(out)|", 1.0).with_complex_components("V(out)", vec![1.0; 2], vec![0.0; 2]);
        wave.complex.as_mut().unwrap().imag = vec![0.0].into();
        assert!(waveform_value(&wave, ComplexExpressionPolicy::Rectangular).is_err());
    }

    #[test]
    fn complex_and_real_arithmetic_never_turn_missing_samples_into_numbers() {
        let wave = WaveformData::new("V(out)", vec![0.0, 1.0], vec![f64::NAN, 2.0], "#fff");
        let waves = [wave];
        let context = WaveformsContext::new(&waves);
        for text in ["V(out)^0", "complex(V(out),0)^0"] {
            let value = evaluator::evaluate(&parser::try_parse(text).unwrap(), &context).unwrap();
            match value {
                CalcValue::Real(RealValue::Waveform(_, values)) => assert!(values[0].is_nan()),
                CalcValue::Complex(ComplexValue::Waveform(_, values)) => {
                    assert!(values[0].re.is_nan() && values[0].im.is_nan())
                }
                _ => panic!("expected a waveform"),
            }
        }
    }

    fn waveform(name: &str, value: f64) -> WaveformData {
        WaveformData::new(name, vec![0.0, 1.0], vec![value, value], "#ffffff")
    }

    fn expected(value: f64) -> CalcValue {
        CalcValue::create_waveform(vec![0.0, 1.0], vec![value, value])
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
        let context =
            WaveformsContext::with_policy(&waveforms, ComplexExpressionPolicy::LegacyMagnitude);

        assert_eq!(context.get_waveform("out", None).unwrap(), expected(3.75));
        assert_eq!(context.get_waveform("vdd", None).unwrap(), expected(5.0));
    }

    #[test]
    fn both_contexts_resolve_only_canonical_voltage_ground_on_their_own_axis() {
        let simulation = SimulationState {
            waveforms: vec![waveform("V(00)", 2.0), waveform("I(0)", 3.0)],
            ..SimulationState::default()
        };
        let live = SimulationContext::new(&simulation);
        let retained = WaveformsContext::new(&simulation.waveforms);
        for context in [&live as &dyn EvaluationContext, &retained] {
            assert_eq!(context.get_waveform("V(0)", None).unwrap(), expected(0.0));
            assert_eq!(context.get_waveform("V(00)", None).unwrap(), expected(2.0));
            assert_eq!(context.get_waveform("I(0)", None).unwrap(), expected(3.0));
            assert!(context.get_waveform("V(missing)", None).is_err());
            assert!(context.get_waveform("V(GND)", None).is_err());
        }
        assert!(
            WaveformsContext::new(&[])
                .get_waveform("V(0)", None)
                .is_err()
        );
    }

    #[test]
    fn live_and_retained_calculators_bind_scopes_without_crossing_quantity_namespaces() {
        let simulation = SimulationState {
            waveforms: vec![
                waveform("V(X1.out)", 2.0),
                waveform("I(X1.out)", 3.0),
                waveform("V(X1:out)", 4.0),
                waveform("V(out)", 5.0),
                waveform("X2.out", 6.0),
                waveform("I(X2.out)", 7.0),
            ],
            ..SimulationState::default()
        };
        let live = SimulationContext::new(&simulation);
        let retained = WaveformsContext::new(&simulation.waveforms);
        for context in [&live as &dyn EvaluationContext, &retained] {
            for (signal, value) in [
                ("V(/X1/out)", 2.0),
                ("|V(/X1/out)|", 2.0),
                ("V(X1:out)", 4.0),
                ("I(/top/X1/out)", 3.0),
                ("I(X1:out)", 3.0),
                ("/X1/out", 2.0),
                ("V(/out)", 5.0),
                ("V(/X2/out)", 6.0),
                ("I(/X2/out)", 7.0),
            ] {
                assert_eq!(context.get_waveform(signal, None).unwrap(), expected(value));
            }
            for signal in ["V(/missing/out)", "I(/out)", "V(//out)", "V(I(X2.out))"] {
                assert!(context.get_waveform(signal, None).is_err(), "{signal}");
            }
        }
    }
}
#[test]
fn complex_physical_difference_uses_retained_phase_in_both_contexts() {
    let waves = vec![
        WaveformData::new("|V(a)|", vec![1.0, 2.0], vec![1.0; 2], "#fff")
            .with_unit("V")
            .with_complex_components("V(a)", vec![1.0; 2], vec![0.0; 2]),
        WaveformData::new("|V(b)|", vec![1.0, 2.0], vec![1.0; 2], "#fff")
            .with_unit("V")
            .with_complex_components("V(b)", vec![-1.0; 2], vec![0.0; 2]),
    ];
    let simulation = SimulationState {
        waveforms: waves,
        ..Default::default()
    };
    let expression = parser::try_parse("abs(V(a)-V(b))").unwrap();
    let expected = CalcValue::create_waveform(vec![1.0, 2.0], vec![2.0; 2]);
    assert_eq!(
        evaluator::evaluate(&expression, &SimulationContext::new(&simulation)).unwrap(),
        expected
    );
    assert_eq!(
        evaluator::evaluate(&expression, &WaveformsContext::new(&simulation.waveforms)).unwrap(),
        expected
    );
}
