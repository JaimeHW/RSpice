//! Signals of other runs an expression names with `@`.
//!
//! `V(out) - V(out)@baseline` is the difference between a result and the run
//! it is compared with. The other run's signal is read onto the evaluated
//! result's own X axis by the comparison's monotone interpolation, straight in
//! X or, on a sweep spaced by a constant ratio, straight in log X, and nothing
//! is extrapolated. It is refused where the difference would mean nothing:
//! another X quantity, another unit, or no X the two runs share.

use super::ast::CalculatorExpr;
use super::evaluator::EvaluationError;
use super::retained::{DatasetSource, find_waveform};
use crate::number_format::fmt_si_significant;
use crate::waveform::RetainedWaveform;
use crate::waveform_comparison::{AxisWarp, check_resampling_axis, resample_onto};

/// The name `@` reads the pinned baseline by.
pub const BASELINE: &str = "baseline";

/// A retained run an expression can read a signal of: its analysis that
/// matches the one evaluated.
pub struct NamedRun<'a, W> {
    /// The names `@` reads it by, matched without regard to case: `Run 3`,
    /// and [`BASELINE`] for the pinned one.
    pub names: Vec<String>,
    pub waveforms: &'a [W],
    pub coordinate_unit: Option<&'a str>,
}

/// The runs an expression evaluated over one result can name.
pub struct RunDatasets<'a, W> {
    /// The result evaluated: its signals and the unit of its X axis.
    pub own: &'a [W],
    pub coordinate_unit: Option<&'a str>,
    pub runs: Vec<NamedRun<'a, W>>,
}

fn quantity(unit: Option<&str>) -> String {
    match unit {
        Some(unit) if !unit.is_empty() => unit.to_owned(),
        _ => "an unstated unit".to_owned(),
    }
}

/// Every sample of `values`: an expression is evaluated over every sample of its operands at
/// once, so a stored operand is read whole, and one larger than the loaded-results budget is
/// refused.
fn read(values: &crate::waveform::WaveformValues, what: &str) -> Result<Vec<f64>, EvaluationError> {
    values
        .read_whole()
        .map(std::borrow::Cow::into_owned)
        .map_err(|failure| EvaluationError::WaveformMismatch(format!("{what}: {failure}")))
}

/// Each run `expr` names with `@`, once, with the first signal it reads
/// there: `(signal, run)`.
#[must_use]
pub fn named_runs(expr: &CalculatorExpr) -> Vec<(String, String)> {
    let mut named: Vec<(String, String)> = Vec::new();
    let mut pending = vec![expr];
    while let Some(expr) = pending.pop() {
        match expr {
            CalculatorExpr::WaveformRef {
                signal,
                dataset: Some(dataset),
            } => {
                if !named
                    .iter()
                    .any(|(_, run)| run.eq_ignore_ascii_case(dataset))
                {
                    named.push((signal.clone(), dataset.clone()));
                }
            }
            CalculatorExpr::BinaryOp { left, right, .. } => {
                pending.extend([right.as_ref(), left.as_ref()]);
            }
            CalculatorExpr::UnaryOp { operand, .. } => pending.push(operand),
            CalculatorExpr::FunctionCall { args, .. } => pending.extend(args.iter().rev()),
            CalculatorExpr::WaveformRef { dataset: None, .. }
            | CalculatorExpr::Number(_)
            | CalculatorExpr::Constant(_)
            | CalculatorExpr::Keyword(_) => {}
        }
    }
    named
}

/// Whether `expr` is a difference from another run: a subtraction one side
/// of which reads a signal of another run, as `V(out) - V(out)@baseline`.
#[must_use]
pub fn is_run_difference(expr: &CalculatorExpr) -> bool {
    matches!(
        expr,
        CalculatorExpr::BinaryOp { op: super::ast::BinaryOp::Sub, left, right }
            if !named_runs(left).is_empty() || !named_runs(right).is_empty()
    )
}

impl<W: AsRef<RetainedWaveform>> RunDatasets<'_, W> {
    fn run(&self, dataset: &str) -> Option<&NamedRun<'_, W>> {
        self.runs.iter().find(|run| {
            run.names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(dataset))
        })
    }

    /// How `signal@dataset` is read onto this result's axis, as a trace
    /// states it: the run by its first name, and whether it is interpolated
    /// straight in X or in log X. `None` when it cannot be read.
    #[must_use]
    pub fn reading(&self, signal: &str, dataset: &str) -> Option<String> {
        let run = self.run(dataset)?;
        let source = find_waveform(run.waveforms, signal)?;
        let x = read(&source.x, signal).ok()?;
        Some(format!(
            "{}, {}",
            run.names.first()?,
            AxisWarp::of(&x).words()
        ))
    }
}

impl<W: AsRef<RetainedWaveform>> DatasetSource for RunDatasets<'_, W> {
    fn waveform_on(
        &self,
        signal: &str,
        dataset: &str,
        axis: &[f64],
    ) -> Result<RetainedWaveform, EvaluationError> {
        let named = format!("{signal}@{dataset}");
        let run = self.run(dataset).ok_or_else(|| {
            EvaluationError::IdentifierNotFound(if dataset.eq_ignore_ascii_case(BASELINE) {
                format!(
                    "{named}: no baseline is pinned here; compare this result with another \
                         run first, or name a run, as in {signal}@\"Run 2\""
                )
            } else {
                format!("{named}: no run named {dataset} has this analysis")
            })
        })?;
        if run.coordinate_unit != self.coordinate_unit {
            return Err(EvaluationError::WaveformMismatch(format!(
                "{named} sweeps X in {} and this result sweeps it in {}, so the two cannot be \
                 read on one axis",
                quantity(run.coordinate_unit),
                quantity(self.coordinate_unit),
            )));
        }
        let source = find_waveform(run.waveforms, signal).ok_or_else(|| {
            EvaluationError::IdentifierNotFound(format!("{named}: {dataset} holds no {signal}"))
        })?;
        if let Some(own) = find_waveform(self.own, signal)
            && own.unit != source.unit
        {
            return Err(EvaluationError::WaveformMismatch(format!(
                "{signal} is in {} here and in {} in {dataset}",
                quantity(own.unit.as_deref()),
                quantity(source.unit.as_deref()),
            )));
        }
        let x = read(&source.x, &named)?;
        check_resampling_axis(&named, &x).map_err(EvaluationError::WaveformMismatch)?;
        let (first, last) = (x[0], x[x.len() - 1]);
        if !axis.iter().any(|value| (first..=last).contains(value)) {
            let here = axis.iter().copied().filter(|value| value.is_finite()).fold(
                None,
                |range: Option<(f64, f64)>, value| {
                    Some(range.map_or((value, value), |(low, high)| {
                        (low.min(value), high.max(value))
                    }))
                },
            );
            let unit = self.coordinate_unit.unwrap_or("");
            let span = |(low, high): (f64, f64)| {
                format!(
                    "{} to {}",
                    fmt_si_significant(low, unit, 4),
                    fmt_si_significant(high, unit, 4)
                )
            };
            return Err(EvaluationError::WaveformMismatch(format!(
                "{named} covers {}, which no X of this result ({}) falls in",
                span((first, last)),
                here.map_or_else(|| "no samples".to_owned(), span),
            )));
        }
        let warp = AxisWarp::of(&x);
        let resample = |values: &crate::waveform::WaveformValues| {
            read(values, &named).map(|values| resample_onto(&x, &values, axis, warp))
        };
        let mut waveform =
            RetainedWaveform::new(named.as_str(), axis.to_vec(), resample(&source.y)?);
        waveform.unit.clone_from(&source.unit);
        if let Some(complex) = &source.complex {
            waveform = waveform.with_complex_components(
                named.as_str(),
                resample(&complex.real)?,
                resample(&complex.imag)?,
            );
        }
        Ok(waveform)
    }
}

#[cfg(test)]
mod tests;
