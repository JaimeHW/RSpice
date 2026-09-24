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
mod complex_functions;
mod complex_ops;
pub(crate) mod evaluator;
pub(crate) mod functions;
pub(crate) mod interpolation;
pub(crate) mod parser;
mod sample_projection;
mod value;

pub use evaluator::{CalcValue, EvaluationContext, EvaluationError};
pub use value::{ComplexValue, RealValue};

/// Preserve rectangular evidence when constructing a physical signal input.
pub(crate) fn waveform_value(
    waveform: &WaveformData,
    policy: ComplexExpressionPolicy,
) -> Result<CalcValue, EvaluationError> {
    let invalid = |error: interpolation::InterpolationError| {
        EvaluationError::WaveformMismatch(error.to_string())
    };
    interpolation::validate_samples(&waveform.x, &waveform.y).map_err(invalid)?;
    if let Some(complex) = &waveform.complex
        && !policy.is_legacy()
    {
        interpolation::validate_samples(&waveform.x, &complex.real).map_err(invalid)?;
        interpolation::validate_samples(&waveform.x, &complex.imag).map_err(invalid)?;
        let y = complex
            .real
            .iter()
            .zip(complex.imag.iter())
            .map(|(&real, &imag)| value::hole(num_complex::Complex64::new(real, imag)))
            .collect();
        Ok(CalcValue::Complex(ComplexValue::Waveform(
            waveform.x.to_vec(),
            y,
        )))
    } else {
        if !policy.is_legacy() && waveform.name.starts_with('|') && waveform.name.ends_with('|') {
            return Err(EvaluationError::PhaseUnavailable(waveform.name.clone()));
        }
        Ok(CalcValue::create_waveform(
            waveform.x.to_vec(),
            waveform.y.to_vec(),
        ))
    }
}

pub(crate) fn magnitude_value(
    value: Result<CalcValue, EvaluationError>,
    source: Option<&WaveformData>,
) -> Result<CalcValue, EvaluationError> {
    let value = match value {
        Err(EvaluationError::PhaseUnavailable(_)) if source.is_some() => {
            waveform_value(source.unwrap(), ComplexExpressionPolicy::LegacyMagnitude)
        }
        value => value,
    }?;
    complex_functions::dispatch("mag", vec![value])
}

fn reject_unbound_dataset(dataset: Option<&str>) -> Result<(), EvaluationError> {
    if let Some(dataset) = dataset {
        return Err(EvaluationError::IdentifierNotFound(format!(
            "dataset '{dataset}' is not bound in this evaluation context"
        )));
    }
    Ok(())
}

/// Retain expression output in the same rectangular form used by solved AC
/// signals. Magnitude is a display column, never a replacement for components.
pub(crate) fn evaluated_waveform(
    value: CalcValue,
    name: &str,
    scalar_axis: Option<&[f64]>,
) -> Result<WaveformData, String> {
    let axis = || {
        scalar_axis
            .filter(|axis| !axis.is_empty())
            .map(<[f64]>::to_vec)
            .ok_or_else(|| "scalar expression has no retained axis".to_owned())
    };
    match value {
        CalcValue::Real(RealValue::Scalar(value)) => {
            let x = axis()?;
            let y = vec![value; x.len()];
            Ok(WaveformData::new(name, x, y, "#f5b700"))
        }
        CalcValue::Real(RealValue::Waveform(x, y)) => Ok(WaveformData::new(name, x, y, "#f5b700")),
        CalcValue::Complex(value) => {
            let (x, y) = match value {
                ComplexValue::Scalar(value) => {
                    let x = axis()?;
                    let y = vec![value; x.len()];
                    (x, y)
                }
                ComplexValue::Waveform(x, y) => (x, y),
            };
            let magnitude: Vec<_> = y.iter().map(|value| value.re.hypot(value.im)).collect();
            let (real, imag): (Vec<_>, Vec<_>) =
                y.into_iter().map(|value| (value.re, value.im)).unzip();
            Ok(WaveformData::new(name, x, magnitude, "#f5b700")
                .with_complex_components(name, real, imag))
        }
    }
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
    fn find_waveform(&self, signal: &str) -> Option<&WaveformData> {
        find_in(&self.simulation.waveforms, signal)
    }
}

/// Evaluation context backed by one analysis' waveform list — used by the
/// Results workspace, where each strip evaluates expressions against its
/// own analysis instead of the live (active-analysis) waveform set.
pub struct WaveformsContext<'a> {
    waveforms: &'a [WaveformData],
    complex_policy: ComplexExpressionPolicy,
    projection: Option<sample_projection::SampleProjection<'a>>,
}

impl<'a> WaveformsContext<'a> {
    /// Wrap an analysis' waveforms.
    pub fn new(waveforms: &'a [WaveformData]) -> Self {
        Self::with_policy(waveforms, ComplexExpressionPolicy::Rectangular)
    }

    pub fn with_policy(
        waveforms: &'a [WaveformData],
        complex_policy: ComplexExpressionPolicy,
    ) -> Self {
        Self {
            waveforms,
            complex_policy,
            projection: None,
        }
    }

    /// Bind exact source rows and, when supplied, their authoritative family
    /// coordinate before any arithmetic or stateful calculation occurs.
    pub(crate) fn with_sample_projection(
        mut self,
        indices: &'a [usize],
        axis: Option<&'a [f64]>,
    ) -> Result<Self, EvaluationError> {
        self.projection = Some(sample_projection::SampleProjection::new(indices, axis)?);
        if self.waveforms.first().is_none_or(|waveform| {
            indices
                .last()
                .is_some_and(|index| *index >= waveform.x.len())
        }) {
            return Err(EvaluationError::WaveformMismatch(
                "selected rows exceed the retained analysis axis".to_owned(),
            ));
        }
        Ok(self)
    }

    fn projected<'w>(
        &self,
        waveform: &'w WaveformData,
    ) -> Result<std::borrow::Cow<'w, WaveformData>, EvaluationError> {
        self.projection.map_or_else(
            || Ok(std::borrow::Cow::Borrowed(waveform)),
            |projection| projection.apply(waveform).map(std::borrow::Cow::Owned),
        )
    }

    fn get_waveform_with_policy(
        &self,
        signal: &str,
        dataset: Option<&str>,
        policy: ComplexExpressionPolicy,
    ) -> Result<CalcValue, EvaluationError> {
        reject_unbound_dataset(dataset)?;
        match signal.to_uppercase().as_str() {
            "TIME" | "T" | "FREQ" | "FREQUENCY" => {
                if let Some(wf) = self.waveforms.first() {
                    let wf = self.projected(wf)?;
                    let x = wf.x.to_vec();
                    let y = x.clone();
                    return Ok(CalcValue::create_waveform(x, y));
                }
                return Err(EvaluationError::IdentifierNotFound(format!(
                    "No waveforms available for {signal}"
                )));
            }
            _ => {}
        }
        if signal.eq_ignore_ascii_case("V(0)") {
            let axis = self
                .waveforms
                .first()
                .map(|wf| self.projected(wf))
                .transpose()?;
            return canonical_ground_value(signal, axis.as_deref())
                .expect("literal canonical ground is always handled");
        }
        match find_in(self.waveforms, signal) {
            Some(wf) => waveform_value(self.projected(wf)?.as_ref(), policy),
            None => Err(EvaluationError::IdentifierNotFound(signal.to_string())),
        }
    }
}

/// Find a waveform by signal name with flexible matching: exact name,
/// net name inside `V()`/`I()`, and AC magnitude entries (`|V(out)|`
/// matches `V(out)` so `dB(V(out)/V(in))` works on AC strips).
fn find_in<'a>(waveforms: &'a [WaveformData], signal: &str) -> Option<&'a WaveformData> {
    find_literal_in(waveforms, signal).or_else(|| {
        let engine = if let Some(body) = bare_wrapped_signal_name(signal) {
            let engine = crate::state::ProbeTarget::engine_alias(body)?;
            // The accessor keeps voltage and current namespaces distinct.
            format!("{}({engine})", &signal.trim_matches('|')[..1])
        } else {
            crate::state::ProbeTarget::engine_alias(signal)?
        };
        find_literal_in(waveforms, &engine)
    })
}

fn find_literal_in<'a>(waveforms: &'a [WaveformData], signal: &str) -> Option<&'a WaveformData> {
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

    // Transient producers retain bare node names. A voltage accessor may
    // read one; a current accessor must never resolve to that node voltage.
    if signal
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
        && let Some(node) = bare_wrapped_signal_name(signal)
        && let Some(wf) = waveforms.iter().find(|wf| {
            bare_wrapped_signal_name(&wf.name).is_none() && wf.name.eq_ignore_ascii_case(node)
        })
    {
        return Some(wf);
    }

    // Bare net name matches inside V() / I() wrappers.
    waveforms.iter().find(|wf| {
        bare_wrapped_signal_name(&wf.name)
            .is_some_and(|wrapped| wrapped.eq_ignore_ascii_case(signal))
    })
}

/// Canonical ground has no solved unknown. Resolve only the literal V(0),
/// against this context's own retained axis. Named aliases need a deck binding;
/// an absent ordinary signal must never be converted into a zero waveform.
pub(crate) fn canonical_ground_value(
    signal: &str,
    axis: Option<&WaveformData>,
) -> Option<Result<CalcValue, EvaluationError>> {
    if !signal.eq_ignore_ascii_case("V(0)") {
        return None;
    }
    Some(
        axis.filter(|wave| !wave.x.is_empty())
            .map(|wave| CalcValue::create_waveform(wave.x.to_vec(), vec![0.0; wave.x.len()]))
            .ok_or_else(|| {
                EvaluationError::IdentifierNotFound("V(0) has no retained analysis axis".to_owned())
            }),
    )
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
    fn get_magnitude(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        let value = match self.get_waveform(signal, dataset) {
            Err(EvaluationError::PhaseUnavailable(_)) => self.get_waveform_with_policy(
                signal,
                dataset,
                ComplexExpressionPolicy::LegacyMagnitude,
            ),
            value => value,
        }?;
        complex_functions::dispatch("mag", vec![value])
    }

    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        self.get_waveform_with_policy(signal, dataset, self.complex_policy)
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

        if let Some(value) = canonical_ground_value(signal, self.simulation.waveforms.first()) {
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
