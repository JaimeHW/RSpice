//! Preserve primitive support through measurement references. Interval
//! operators must not mistake a sampled regular part for the full primitive.
use super::*;

fn constant_condition(
    expression: &NetExpr,
    names: &HashMap<String, usize>,
    params: &crate::netlist::ParamContext,
) -> Option<ComplexValue> {
    let constant = match expression {
        NetExpr::Number(_) | NetExpr::ComplexNumber(_) => true,
        NetExpr::Param(name) => {
            !names.contains_key(&name.to_ascii_uppercase())
                && !matches!(
                    name.to_ascii_uppercase().as_str(),
                    "TIME" | "FREQ" | "FREQUENCY" | "HERTZ"
                )
                && params.get_complex(name).is_some()
        }
        NetExpr::UnaryOp { operand, .. } => constant_condition(operand, names, params).is_some(),
        NetExpr::BinOp { left, right, .. } => {
            constant_condition(left, names, params).is_some()
                && constant_condition(right, names, params).is_some()
        }
        // Do not run statistical, file-backed or user functions while deriving
        // dependencies. Their evaluation belongs to the actual live row.
        _ => false,
    };
    constant
        .then(|| crate::netlist::expr::evaluate_complex_raw(expression, params).ok())
        .flatten()
        .filter(|value| value.re.is_finite() && value.im.is_finite())
}

fn prune(
    expression: &NetExpr,
    names: &HashMap<String, usize>,
    params: &crate::netlist::ParamContext,
    abort: &dyn AbortSignal,
) -> Result<NetExpr, EquationMeasurementEvaluationError> {
    if abort.is_aborted() {
        return Err(EquationMeasurementEvaluationError::Aborted);
    }
    if let NetExpr::FnCall { name, args } = expression
        && name.eq_ignore_ascii_case("IF")
        && !params.has_function(name)
        && let [test, on_true, on_false] = args.as_slice()
        && let Some(condition) = constant_condition(test, names, params)
    {
        return prune(
            if condition.re != 0.0 || condition.im != 0.0 {
                on_true
            } else {
                on_false
            },
            names,
            params,
            abort,
        );
    }
    Ok(match expression {
        NetExpr::UnaryOp { op, operand } => NetExpr::UnaryOp {
            op: *op,
            operand: Box::new(prune(operand, names, params, abort)?),
        },
        NetExpr::BinOp { op, left, right } => NetExpr::BinOp {
            op: *op,
            left: Box::new(prune(left, names, params, abort)?),
            right: Box::new(prune(right, names, params, abort)?),
        },
        NetExpr::FnCall { name, args } => NetExpr::FnCall {
            name: name.clone(),
            args: args
                .iter()
                .map(|arg| prune(arg, names, params, abort))
                .collect::<Result<_, _>>()?,
        },
        _ => expression.clone(),
    })
}

pub(in super::super) fn bind(
    programs: &mut [LiveMeasureProgram<'_>],
    dependencies: &[Vec<String>],
    names: &HashMap<String, usize>,
    params: &crate::netlist::ParamContext,
    abort: &dyn AbortSignal,
) -> Result<(), EquationMeasurementEvaluationError> {
    let mut has_primitives = false;
    for (index, program) in programs.iter_mut().enumerate() {
        if abort.is_aborted() {
            return Err(EquationMeasurementEvaluationError::Aborted);
        }
        let mut owns_primitives = false;
        if let Some(observation) = &program.observation {
            for cursor in &observation.cursors {
                if abort.is_aborted() {
                    return Err(EquationMeasurementEvaluationError::Aborted);
                }
                owns_primitives |= !cursor.term.trace.derivatives.is_empty();
            }
        }
        if owns_primitives {
            program.primitive_sources.try_reserve(1).map_err(|_| {
                EquationMeasurementEvaluationError::Detail(
                    "cannot allocate current primitive dependencies".into(),
                )
            })?;
            program.primitive_sources.push(index);
            has_primitives = true;
        }
    }
    if !has_primitives {
        return Ok(());
    }
    let mut dependencies = dependencies.to_vec();
    for (program, dependency) in programs.iter().zip(&mut dependencies) {
        if abort.is_aborted() {
            return Err(EquationMeasurementEvaluationError::Aborted);
        }
        if program.failure.is_some() || program.state.is_none() {
            continue;
        }
        if let MeasureType::Equation { expression, .. } | MeasureType::Param { expression } =
            &program.statement.measure_type
            && matches!(
                expression.kind,
                crate::netlist::measure::MeasureExpressionKind::Expression
            )
        {
            let parsed = crate::netlist::expr::parse_expression(&expression.text)
                .map_err(|error| EquationMeasurementEvaluationError::Detail(error.to_string()))?;
            let prepared =
                LivePreparedExpression::compile(&prune(&parsed, names, params, abort)?, params)
                    .map_err(EquationMeasurementEvaluationError::Detail)?;
            *dependency = prepared
                .parameters
                .values()
                .map(|parameter| parameter.canonical_measure.clone())
                .collect();
        }
    }
    loop {
        let mut changed = false;
        for (index, dependency) in dependencies.iter().enumerate() {
            if abort.is_aborted() {
                return Err(EquationMeasurementEvaluationError::Aborted);
            }
            for name in dependency {
                let Some(&source) = names.get(name) else {
                    continue;
                };
                let missing = || {
                    EquationMeasurementEvaluationError::Detail(
                        "invalid current primitive dependency index".into(),
                    )
                };
                let count = programs
                    .get(source)
                    .ok_or_else(missing)?
                    .primitive_sources
                    .len();
                for position in 0..count {
                    if abort.is_aborted() {
                        return Err(EquationMeasurementEvaluationError::Aborted);
                    }
                    let primitive = programs
                        .get(source)
                        .and_then(|program| program.primitive_sources.get(position))
                        .copied()
                        .ok_or_else(missing)?;
                    let destination = &mut programs
                        .get_mut(index)
                        .ok_or_else(missing)?
                        .primitive_sources;
                    if !destination.contains(&primitive) {
                        destination.try_reserve(1).map_err(|_| {
                            EquationMeasurementEvaluationError::Detail(
                                "cannot allocate current primitive dependencies".into(),
                            )
                        })?;
                        destination.push(primitive);
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            return Ok(());
        }
    }
}
