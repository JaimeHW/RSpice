//! Forward temperature option bindings, owned by the parser's lexical scopes.

use super::*;
use crate::netlist::expr::{PreparedExpression, parse_expression};

#[derive(Clone, Copy, Debug)]
pub(in super::super) enum TemperatureOption {
    Temp,
    Tnom,
}

impl TemperatureOption {
    fn name(self) -> &'static str {
        match self {
            Self::Temp => "TEMP",
            Self::Tnom => "TNOM",
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Temp => 0,
            Self::Tnom => 1,
        }
    }

    fn install(self, options: &mut SimulationOptions, value: Value) {
        match self {
            Self::Temp => options.temp = Some(value),
            Self::Tnom => options.tnom = Some(value),
        }
    }
}

#[derive(Debug)]
struct PendingTemperatureOption {
    option: TemperatureOption,
    ordinal: usize,
    expression: PreparedExpression,
    bound_values: HashMap<String, crate::ComplexValue>,
    sign: Value,
    origin: NetlistSourceLocation,
}

#[derive(Debug, Default)]
pub(in super::super) struct TemperatureOptionPlan {
    ordinal: usize,
    last: [usize; 2],
    scopes: Vec<Vec<PendingTemperatureOption>>,
}

pub(in super::super) struct TemperatureOptionSink<'a> {
    pub(in super::super) plan: &'a mut TemperatureOptionPlan,
    pub(in super::super) depth: usize,
    pub(in super::super) origin: &'a NetlistSourceLocation,
}

impl TemperatureOptionSink<'_> {
    pub(in super::super) fn parse(
        &mut self,
        option: TemperatureOption,
        stream: &mut TokenStream,
        line: usize,
        params: &ParamContext,
        options: &mut SimulationOptions,
    ) -> Result<(), ParseError> {
        skip_commas(stream);
        let sign_width = usize::from(matches!(
            stream.peek().kind,
            TokenKind::Plus | TokenKind::Minus
        ));
        let expression = match &stream.peek_n(sign_width).kind {
            TokenKind::Expression(expression) => Some(expression.clone()),
            TokenKind::Ident(name)
                if params.get(name).is_none()
                    && parse_boolean_literal(name).is_none()
                    && crate::netlist::lexer::parse_spice_value(name).is_err() =>
            {
                Some(name.clone())
            }
            _ => None,
        };
        let deferred = if let Some(expression) =
            expression.filter(|expression| needs_forward_probe(expression, params))
        {
            // A failed evaluation can already have sampled a random function.
            // Probe on an isolated stream; immediate values are evaluated on
            // the live stream exactly once. Forward expressions draw only when
            // their owning scope closes, in their authored order.
            match eval_expression(&expression, &params.isolated_random_clone()) {
                Err(crate::netlist::expr::ExprError::UndefinedParam(_)) => {
                    let parsed = parse_expression(&expression)
                        .map_err(|error| ParseError::InvalidValue(error.to_string()))?;
                    let prepared = PreparedExpression::compile(&parsed, params)
                        .map_err(|error| ParseError::InvalidValue(error.to_string()))?;
                    let mut bound_values = HashMap::new();
                    prepared.visit_runtime_parameters(|name| {
                        if let Some(value) = params.get_complex(name) {
                            bound_values.insert(name.to_owned(), value);
                        }
                    });
                    let sign = if matches!(stream.peek().kind, TokenKind::Minus) {
                        -1.0
                    } else {
                        1.0
                    };
                    for _ in 0..=sign_width {
                        stream.advance();
                    }
                    Some((prepared, bound_values, sign))
                }
                // Preserve the ordinary evaluator's lazy branches and errors.
                _ => None,
            }
        } else {
            None
        };
        self.plan.ordinal += 1;
        self.plan.last[option.index()] = self.plan.ordinal;
        if let Some((expression, bound_values, sign)) = deferred {
            self.plan
                .scopes
                .resize_with(self.plan.scopes.len().max(self.depth + 1), Vec::new);
            self.plan.scopes[self.depth].push(PendingTemperatureOption {
                option,
                ordinal: self.plan.ordinal,
                expression,
                bound_values,
                sign,
                origin: self.origin.clone(),
            });
        } else {
            let value = expect_value(stream, line, params)?;
            option.install(options, parse_celsius_option(option.name(), value, line)?);
        }
        Ok(())
    }
}

impl TemperatureOptionPlan {
    pub(in super::super) fn resolve_scope(
        &mut self,
        depth: usize,
        params: &ParamContext,
        options: &mut SimulationOptions,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let Some(scope) = self.scopes.get_mut(depth) else {
            return Ok(());
        };
        for pending in scope.iter_mut() {
            ensure_parse_not_aborted(abort)?;
            let value = pending
                .expression
                .evaluate_with(params, &mut |name| {
                    Ok(pending.bound_values.get(name).copied())
                })
                .map_err(|error| pending.error(error.to_string()))?;
            let value = if params.expression_dialect() == ExpressionDialect::Xyce {
                crate::netlist::expr::normalize_xyce_expression_result(value)
            } else {
                value
            }
            .re * pending.sign;
            let value = parse_celsius_option(pending.option.name(), value, pending.origin.line)
                .map_err(|error| pending.error(error.to_string()))?;
            // A later literal or a child scope can supersede this assignment.
            // Still validate and evaluate every active authored assignment.
            if self.last[pending.option.index()] == pending.ordinal {
                pending.option.install(options, value);
            }
        }
        scope.clear();
        Ok(())
    }
}

impl PendingTemperatureOption {
    fn error(&self, message: String) -> ParseError {
        ParseError::Syntax {
            line: self.origin.line,
            message: format!("{}: {}: {message}", self.origin, self.option.name()),
        }
    }
}

/// Known scalar bindings and builtin functions cannot introduce a missing name.
/// Keep their common path free of full parameter-environment snapshots. User
/// functions and retained bindings are conservatively probed because their
/// bodies can hide both forward references and statistical calls.
fn needs_forward_probe(expression: &str, params: &ParamContext) -> bool {
    use crate::netlist::expr::Expr;
    let Ok(parsed) = parse_expression(expression) else {
        return false;
    };
    let mut pending = vec![&parsed];
    while let Some(node) = pending.pop() {
        match node {
            Expr::Param(name)
                if params.get_complex(name).is_none()
                    || params.get_parameter_expression(name).is_some()
                    || params.get_global_expression(name).is_some() =>
            {
                return true;
            }
            Expr::FnCall { name, .. } if params.get_function(name).is_some() => return true,
            Expr::FnCall { args, .. } => pending.extend(args.iter()),
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            _ => {}
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_resolution_stops_at_the_first_cancellation_poll() {
        let mut plan = TemperatureOptionPlan::default();
        let mut params = ParamContext::new();
        let mut options = SimulationOptions::default();
        let origin = NetlistSourceLocation::in_memory(2);
        for _ in 0..3 {
            let mut stream = TokenStream::new(tokenize("{ambient}").unwrap());
            TemperatureOptionSink {
                plan: &mut plan,
                depth: 1,
                origin: &origin,
            }
            .parse(
                TemperatureOption::Temp,
                &mut stream,
                2,
                &params,
                &mut options,
            )
            .unwrap();
        }
        params.set("ambient", 85.0);
        let abort = crate::abort_signal::CountingAbort::new(1);
        assert!(matches!(
            plan.resolve_scope(1, &params, &mut options, &abort),
            Err(ParseWithAbortError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
        assert_eq!(options.temp, None);
    }
}
