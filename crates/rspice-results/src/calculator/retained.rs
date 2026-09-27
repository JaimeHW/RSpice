//! Exact retained waveform binding and derived expression output.

mod sample_projection;

use super::complex_functions::dispatch;
use super::evaluator::{EvaluationContext, EvaluationError};
use super::value::{CalcValue, ComplexValue, RealValue, hole};
use crate::interpolation;
use crate::saved_output::ComplexExpressionPolicy;
use crate::waveform::RetainedWaveform;

/// Preserve rectangular evidence when constructing a physical signal input.
pub fn waveform_value(
    waveform: &RetainedWaveform,
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
            .map(|(&real, &imag)| hole(num_complex::Complex64::new(real, imag)))
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

pub fn magnitude_value(
    value: Result<CalcValue, EvaluationError>,
    source: Option<&RetainedWaveform>,
) -> Result<CalcValue, EvaluationError> {
    let value = match value {
        Err(EvaluationError::PhaseUnavailable(_)) if source.is_some() => {
            waveform_value(source.unwrap(), ComplexExpressionPolicy::LegacyMagnitude)
        }
        value => value,
    }?;
    dispatch("mag", vec![value])
}

pub fn reject_unbound_dataset(dataset: Option<&str>) -> Result<(), EvaluationError> {
    if let Some(dataset) = dataset {
        return Err(EvaluationError::IdentifierNotFound(format!(
            "dataset '{dataset}' is not bound in this evaluation context"
        )));
    }
    Ok(())
}

/// Retain expression output in the same rectangular form used by solved AC
/// signals. Magnitude is a display column, never a replacement for components.
pub fn evaluated_waveform(
    value: CalcValue,
    name: &str,
    scalar_axis: Option<&[f64]>,
) -> Result<RetainedWaveform, String> {
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
            Ok(RetainedWaveform::new(name, x, y))
        }
        CalcValue::Real(RealValue::Waveform(x, y)) => Ok(RetainedWaveform::new(name, x, y)),
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
            Ok(RetainedWaveform::new(name, x, magnitude).with_complex_components(name, real, imag))
        }
    }
}

/// Evaluation context backed by one analysis' waveform list — used by the
/// Results workspace, where each strip evaluates expressions against its
/// own analysis instead of the live (active-analysis) waveform set.
pub struct WaveformsContext<'a, W = RetainedWaveform> {
    waveforms: &'a [W],
    complex_policy: ComplexExpressionPolicy,
    projection: Option<sample_projection::SampleProjection<'a>>,
}

impl<'a, W: AsRef<RetainedWaveform>> WaveformsContext<'a, W> {
    /// Wrap an analysis' waveforms.
    pub fn new(waveforms: &'a [W]) -> Self {
        Self::with_policy(waveforms, ComplexExpressionPolicy::Rectangular)
    }

    pub fn with_policy(waveforms: &'a [W], complex_policy: ComplexExpressionPolicy) -> Self {
        Self {
            waveforms,
            complex_policy,
            projection: None,
        }
    }

    /// Bind exact source rows and, when supplied, their authoritative family
    /// coordinate before any arithmetic or stateful calculation occurs.
    pub fn with_sample_projection(
        mut self,
        indices: &'a [usize],
        axis: Option<&'a [f64]>,
    ) -> Result<Self, EvaluationError> {
        self.projection = Some(sample_projection::SampleProjection::new(indices, axis)?);
        if self
            .waveforms
            .first()
            .map(AsRef::as_ref)
            .is_none_or(|waveform| {
                indices
                    .last()
                    .is_some_and(|index| *index >= waveform.x.len())
            })
        {
            return Err(EvaluationError::WaveformMismatch(
                "selected rows exceed the retained analysis axis".to_owned(),
            ));
        }
        Ok(self)
    }

    fn projected<'w>(
        &self,
        waveform: &'w RetainedWaveform,
    ) -> Result<std::borrow::Cow<'w, RetainedWaveform>, EvaluationError> {
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
                if let Some(wf) = self.waveforms.first().map(AsRef::as_ref) {
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
                .map(|wf| self.projected(wf.as_ref()))
                .transpose()?;
            return canonical_ground_value(signal, axis.as_deref())
                .expect("literal canonical ground is always handled");
        }
        match find_waveform(self.waveforms, signal) {
            Some(wf) => waveform_value(self.projected(wf)?.as_ref(), policy),
            None => Err(EvaluationError::IdentifierNotFound(signal.to_string())),
        }
    }
}

/// Find a waveform by signal name with flexible matching: exact name,
/// net name inside `V()`/`I()`, and AC magnitude entries (`|V(out)|`
/// matches `V(out)` so `dB(V(out)/V(in))` works on AC strips).
pub fn find_waveform<'a, W: AsRef<RetainedWaveform>>(
    waveforms: &'a [W],
    signal: &str,
) -> Option<&'a RetainedWaveform> {
    find_literal_in(waveforms, signal).or_else(|| {
        let engine = if let Some(body) = bare_wrapped_signal_name(signal) {
            let engine = rspice_app_types::hierarchy_path::ProbeTarget::engine_alias(body)?;
            // The accessor keeps voltage and current namespaces distinct.
            format!("{}({engine})", &signal.trim_matches('|')[..1])
        } else {
            rspice_app_types::hierarchy_path::ProbeTarget::engine_alias(signal)?
        };
        find_literal_in(waveforms, &engine)
    })
}

fn find_literal_in<'a, W: AsRef<RetainedWaveform>>(
    waveforms: &'a [W],
    signal: &str,
) -> Option<&'a RetainedWaveform> {
    if let Some(wf) = waveforms
        .iter()
        .map(AsRef::as_ref)
        .find(|wf| wf.name.eq_ignore_ascii_case(signal))
    {
        return Some(wf);
    }

    // `|V(out)|` (AC magnitude) matches a request for `V(out)`.
    if let Some(wf) = waveforms
        .iter()
        .map(AsRef::as_ref)
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
        && let Some(wf) = waveforms.iter().map(AsRef::as_ref).find(|wf| {
            bare_wrapped_signal_name(&wf.name).is_none() && wf.name.eq_ignore_ascii_case(node)
        })
    {
        return Some(wf);
    }

    // Bare net name matches inside V() / I() wrappers.
    waveforms.iter().map(AsRef::as_ref).find(|wf| {
        bare_wrapped_signal_name(&wf.name)
            .is_some_and(|wrapped| wrapped.eq_ignore_ascii_case(signal))
    })
}

/// Canonical ground has no solved unknown. Resolve only the literal V(0),
/// against this context's own retained axis. Named aliases need a deck binding;
/// an absent ordinary signal must never be converted into a zero waveform.
pub fn canonical_ground_value(
    signal: &str,
    axis: Option<&RetainedWaveform>,
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

impl<W: AsRef<RetainedWaveform>> EvaluationContext for WaveformsContext<'_, W> {
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
        dispatch("mag", vec![value])
    }

    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        self.get_waveform_with_policy(signal, dataset, self.complex_policy)
    }
}
