//! DC mismatch summaries and ranked contributors retain their distinct meanings.
use super::*;
use crate::analysis::dcmatch::DcMatchResult;
use crate::execution::result_document::{ResultScalar, ScalarValue};
use crate::netlist::expr::ExprError;

#[derive(Clone, Copy)]
pub(super) enum DcMatchColumn {
    Nominal,
    Total,
    Mismatch,
    Process,
    Quoted,
    ParameterSigma,
    Sensitivity,
    Contribution,
    Share,
}

impl DcMatchColumn {
    pub(super) fn is_scalar(self) -> bool {
        matches!(
            self,
            Self::Nominal | Self::Total | Self::Mismatch | Self::Process | Self::Quoted
        )
    }

    pub(super) fn sample(self, result: &DcMatchResult, row: usize) -> Option<ComplexValue> {
        Some(
            match self {
                Self::Nominal => result.nominal_value,
                Self::Total => result.sigma_total,
                Self::Mismatch => result.sigma_mismatch,
                Self::Process => result.sigma_process,
                Self::Quoted => result.quoted_sigma(),
                Self::ParameterSigma => result.contributors.get(row)?.sigma_parameter,
                Self::Sensitivity => result.contributors.get(row)?.sensitivity,
                Self::Contribution => result.contributors.get(row)?.contribution,
                Self::Share => result.contributors.get(row)?.share,
            }
            .into(),
        )
    }
}

pub(super) fn select<'a>(dataset: &'a ControlNamedDataset, name: &str) -> Option<Selected<'a>> {
    let ControlAnalysisResult::DcMatch(result) = &dataset.result else {
        return None;
    };
    use DcMatchColumn::*;
    let signal = name.to_ascii_lowercase();
    let column = match signal.as_str() {
        "nominal_value" => Nominal,
        "sigma_total" => Total,
        "sigma_mismatch" => Mismatch,
        "sigma_process" => Process,
        "quoted_sigma" => Quoted,
        "sigma_parameter" => ParameterSigma,
        "sensitivity" => Sensitivity,
        "contribution" => Contribution,
        "share" => Share,
        _ => return None,
    };
    let unit = match column {
        // Contributors can describe parameters with different physical units.
        ParameterSigma | Sensitivity => SignalUnit::Unspecified,
        Share => SignalUnit::Dimensionless,
        _ => result.output_unit(),
    };
    Some(Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal,
        },
        column: Column::DcMatch(column),
        unit,
        noise_probe: None,
    })
}

pub(in crate::engine::control) fn resolve_scalar(
    circuit: &ControlCircuit,
    name: &str,
) -> Result<Option<ComplexValue>, ExprError> {
    let Ok((dataset, raw)) = circuit.qualified(name, 0) else {
        return Ok(None);
    };
    let Some(selected) = select(dataset, raw) else {
        return Ok(None);
    };
    if !selected.is_scalar() && dataset.length() != 1 {
        return Err(ExprError::InvalidArgument(format!(
            "DC mismatch vector '{name}' requires exactly one contributor to be a scalar"
        )));
    }
    selected
        .sample(0)
        .filter(|v| v.re.is_finite() && v.im.is_finite())
        .map(Some)
        .ok_or_else(|| {
            ExprError::InvalidArgument(format!(
                "DC mismatch scalar '{name}' is nonfinite or unavailable"
            ))
        })
}

/// Summary expressions are evaluated once, even when report filtering retains
/// no contributors. Mixing them with contributor vectors uses normal broadcast.
pub(super) fn printed_expression(
    resolver: &mut Resolver<'_>,
    expression: &Expr,
    label: &str,
    position: usize,
) -> Result<Option<ControlScalar>, ControlExecutionError> {
    if !resolver
        .circuit
        .datasets
        .iter()
        .any(|dataset| matches!(dataset.result, ControlAnalysisResult::DcMatch(_)))
    {
        return Ok(None);
    }
    let mut bound = expression.clone();
    let mut inputs = Vec::new();
    let Ok(unit) = resolver.bind(&mut bound, &mut inputs) else {
        return Ok(None);
    };
    if inputs.is_empty() || !inputs.iter().all(Selected::is_scalar) {
        return Ok(None);
    }
    resolver.charge(2usize.saturating_add(inputs.len().saturating_mul(2)))?;
    let first = &inputs[0];
    let scalar = if matches!(expression, Expr::Param(_)) {
        let ControlAnalysisResult::DcMatch(result) = &first.dataset.result else {
            unreachable!()
        };
        let scalar = crate::execution::AnalysisResultDocument::dc_match_scalars(result)
            .map_err(|error| command_error(resolver.line, error.to_string()))?
            .into_iter()
            .find(|scalar| scalar.name() == first.id.signal)
            .ok_or_else(|| {
                command_error(
                    resolver.line,
                    "DC mismatch scalar lost its document identity",
                )
            })?;
        ResultScalar::new(scalar.name(), label, Some(unit), scalar.value().clone())
    } else {
        let mut context = ParamContext::new();
        for (index, input) in inputs.iter().enumerate() {
            check_abort(resolver.abort, resolver.line)?;
            let value = input
                .sample(0)
                .ok_or_else(|| unavailable(resolver.line, input.dataset, &input.id.signal))?;
            context.set_complex(&format!("\0V{index}"), value);
        }
        let value = evaluate_complex_with_functions_and_abort(
            &bound,
            &context,
            &mut |_| Ok(None),
            &mut |_, _| Ok(None),
            resolver.abort,
        )
        .map_err(|error| ControlError::evaluation(resolver.line, error))?;
        let value = if value.im == 0.0 {
            ScalarValue::Real {
                value: Some(value.re),
            }
        } else {
            ScalarValue::Complex {
                value: Some(value.into()),
            }
        };
        ResultScalar::new(label, label, Some(unit), value)
    }
    .map_err(|error| command_error(resolver.line, error.to_string()))?;
    Ok(Some(ControlScalar {
        position,
        dataset: first.dataset.name.clone(),
        scalar,
    }))
}
