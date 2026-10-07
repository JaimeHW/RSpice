//! Parser tokens with optional, resumable numeric reads for deferred cards.
use super::*;
use crate::netlist::expr::{
    ExprError, ExpressionEvaluationError, PreparedExpression, PreparedProgress,
    eval_expression_complex_with_abort, normalize_xyce_expression_result,
    parse_expression_with_abort,
};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone)]
pub(super) struct TokenStream<'a> {
    tokens: crate::netlist::lexer::TokenStream,
    bindings: Option<Box<NumericBindings>>,
    require_real_values: bool,
    optional_numeric_failure: Option<Box<str>>,
    abort: &'a dyn AbortSignal,
}

impl std::fmt::Debug for TokenStream<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TokenStream")
            .field("tokens", &self.tokens)
            .field("bindings", &self.bindings)
            .field("require_real_values", &self.require_real_values)
            .field("optional_numeric_failure", &self.optional_numeric_failure)
            .finish_non_exhaustive()
    }
}

/// Numeric grammar probes can discard an expression error or retry on a clone.
/// Retain cancellation independently until the enclosing parse boundary, and
/// never poll the caller again after its first abort.
pub(super) struct NumericParseAbort<'a> {
    signal: &'a dyn AbortSignal,
    cancelled: AtomicBool,
}

impl<'a> NumericParseAbort<'a> {
    pub(super) fn new(signal: &'a dyn AbortSignal) -> Self {
        Self {
            signal,
            cancelled: AtomicBool::new(false),
        }
    }

    pub(super) fn finish<T, E: Into<ParseWithAbortError>>(
        &self,
        result: Result<T, E>,
    ) -> Result<T, ParseWithAbortError> {
        if self.cancelled.load(Ordering::Relaxed) {
            Err(ParseWithAbortError::Aborted)
        } else {
            result.map_err(Into::into)
        }
    }
}

impl AbortSignal for NumericParseAbort<'_> {
    fn is_aborted(&self) -> bool {
        if self.cancelled.load(Ordering::Relaxed) {
            return true;
        }
        if self.signal.is_aborted() {
            self.cancelled.store(true, Ordering::Relaxed);
            return true;
        }
        false
    }
}

fn numeric_expression_error(error: ExpressionEvaluationError) -> ExprError {
    match error {
        ExpressionEvaluationError::Expression(error) => error,
        // Only the grammar sees this sentinel. NumericParseAbort::finish
        // restores the typed abort even when a grammar probe ignores it.
        ExpressionEvaluationError::Aborted => {
            ExprError::InvalidArgument("numeric parsing cancelled".into())
        }
    }
}

#[derive(Debug, Clone, Default)]
struct NumericBindings {
    operands: HashMap<usize, Operand>,
    values: HashMap<String, crate::ComplexValue>,
    missing: Option<String>,
}

#[derive(Debug, Clone)]
enum Operand {
    Pending(Box<PreparedExpression>),
    Complete(Result<Value, ExprError>),
}

impl TokenStream<'_> {
    pub(super) fn new(tokens: Vec<crate::netlist::lexer::Token>) -> Self {
        Self {
            tokens: crate::netlist::lexer::TokenStream::new(tokens),
            bindings: None,
            require_real_values: false,
            optional_numeric_failure: None,
            abort: &NoAbort,
        }
    }

    pub(super) fn with_abort(self, abort: &dyn AbortSignal) -> TokenStream<'_> {
        TokenStream {
            tokens: self.tokens,
            bindings: self.bindings,
            require_real_values: self.require_real_values,
            optional_numeric_failure: self.optional_numeric_failure,
            abort,
        }
    }

    pub(super) fn require_real_numeric_values(&mut self) {
        self.require_real_values = true;
        self.optional_numeric_failure = None;
    }

    pub(super) fn has_optional_numeric_failure(&self) -> bool {
        self.optional_numeric_failure.is_some()
    }

    pub(super) fn remember_optional_numeric_failure(&mut self, error: ParseError) {
        let message = match error {
            ParseError::InvalidValue(message) => message,
            error => error.to_string(),
        };
        self.optional_numeric_failure = Some(message.into_boxed_str());
    }

    pub(super) fn take_optional_numeric_failure(&mut self) -> Option<ParseError> {
        self.optional_numeric_failure
            .take()
            .map(|message| ParseError::InvalidValue(message.into()))
    }

    pub(super) fn requires_real_values(&self) -> bool {
        self.require_real_values
    }

    pub(super) fn numeric_value(&self, value: crate::ComplexValue) -> Result<Value, ExprError> {
        project_numeric(value, self.require_real_values)
    }

    pub(super) fn numeric_literal(
        &self,
        spelling: &str,
        value: Value,
        params: &ParamContext,
    ) -> Result<Value, ExprError> {
        // Check the lexer's value before dialect normalization could turn
        // literal overflow into a finite expression sentinel. A numeric `j`
        // suffix must also survive the lexer's real-valued token payload.
        self.numeric_value(value.into())?;
        if self.require_real_values && spelling.bytes().any(|byte| matches!(byte, b'j' | b'J')) {
            self.numeric_value(
                eval_expression_complex_with_abort(spelling, params, self.abort)
                    .map_err(numeric_expression_error)?,
            )
        } else {
            Ok(value)
        }
    }

    pub(super) fn begin_numeric_binding(&mut self) {
        self.bindings = Some(Box::default());
    }

    pub(super) fn binding_numeric_values(&self) -> bool {
        self.bindings.is_some()
    }

    pub(super) fn missing_numeric_parameter(&self) -> Option<&str> {
        self.bindings.as_ref()?.missing.as_deref()
    }

    pub(super) fn resume_numeric_binding(
        &mut self,
        checkpoint: usize,
        name: String,
        value: crate::ComplexValue,
    ) {
        let bindings = self.bindings.as_mut().expect("binding mode");
        bindings.values.insert(name, value);
        bindings.missing = None;
        self.tokens.restore_checkpoint(checkpoint);
    }

    /// Read before advancing the token: its authored byte position identifies
    /// the operand across grammar retries, including nested MC analysis cards.
    pub(super) fn numeric_expression(
        &mut self,
        expression: &str,
        params: &ParamContext,
    ) -> Result<Value, ExprError> {
        let key = self.peek().span.start;
        let require_real = self.require_real_values;
        let abort = self.abort;
        let Some(bindings) = &mut self.bindings else {
            return project_numeric(
                eval_expression_complex_with_abort(expression, params, abort)
                    .map_err(numeric_expression_error)?,
                require_real,
            );
        };
        // A grammar can probe optional operands after one failed read. Do not
        // evaluate later operands ahead of that dependency's random draws.
        if let Some(name) = &bindings.missing {
            return Err(ExprError::UndefinedParam(name.clone()));
        }
        let operand = bindings.operands.entry(key).or_insert_with(|| {
            match parse_expression_with_abort(expression, abort)
                .map_err(ExpressionEvaluationError::from)
                .and_then(|expr| PreparedExpression::compile_with_abort(&expr, params, abort))
            {
                Ok(mut program) => {
                    program.begin_evaluation();
                    Operand::Pending(Box::new(program))
                }
                Err(error) => Operand::Complete(Err(numeric_expression_error(error))),
            }
        });
        let program = match operand {
            Operand::Pending(program) => program,
            Operand::Complete(result) => return result.clone(),
        };
        let result = match program.resume_with_abort(
            params,
            &mut |name| Ok(bindings.values.get(name).copied()),
            abort,
        ) {
            Ok(PreparedProgress::MissingParameter(name)) => {
                bindings.missing = Some(name.clone());
                return Err(ExprError::UndefinedParam(name));
            }
            Ok(PreparedProgress::Complete(value)) => {
                let value = if params.expression_dialect() == ExpressionDialect::Xyce {
                    normalize_xyce_expression_result(value)
                } else {
                    value
                };
                project_numeric(value, require_real)
            }
            Err(error) => Err(numeric_expression_error(error)),
        };
        *operand = Operand::Complete(result.clone());
        result
    }

    pub(super) fn numeric_expression_capturing_direction(
        &mut self,
        expression: &str,
        params: &ParamContext,
        direction: Option<&mut Result<Derivative, ExprError>>,
    ) -> Result<Value, ExprError> {
        let Some(direction) = direction else {
            return self.numeric_expression(expression, params);
        };
        self.numeric_value(self.evaluate_complex_capturing_direction(
            expression,
            params,
            Some(direction),
        )?)
    }

    pub(super) fn evaluate_expression(
        &self,
        expression: &str,
        params: &ParamContext,
    ) -> Result<Value, ExprError> {
        self.evaluate_value_capturing_direction(expression, params, None)
    }

    pub(super) fn evaluate_value_capturing_direction(
        &self,
        expression: &str,
        params: &ParamContext,
        direction: Option<&mut Result<Derivative, ExprError>>,
    ) -> Result<Value, ExprError> {
        self.evaluate_complex_capturing_direction(expression, params, direction)
            .map(|value| value.re)
    }

    fn evaluate_complex_capturing_direction(
        &self,
        expression: &str,
        params: &ParamContext,
        direction: Option<&mut Result<Derivative, ExprError>>,
    ) -> Result<crate::ComplexValue, ExprError> {
        let Some(direction) = direction else {
            return eval_expression_complex_with_abort(expression, params, self.abort)
                .map_err(numeric_expression_error);
        };
        let (value, tangent) = params
            .evaluate_parameter_binding_with_abort(expression, self.abort)
            .map_err(numeric_expression_error)?;
        *direction = tangent
            .unwrap_or_else(|| Ok(crate::netlist::expr::ComplexDirection::zero()))
            .map(|tangent| tangent.re);
        Ok(value)
    }

    pub(super) fn prepare_behavioral_expression(
        &self,
        expression: &str,
        params: &ParamContext,
    ) -> Result<String, String> {
        crate::netlist::expr::prepare_behavioral_expression_with_abort(
            expression, params, self.abort,
        )
        .map_err(|error| match error {
            crate::netlist::expr::BehavioralPreparationError::Semantic(error) => error,
            crate::netlist::expr::BehavioralPreparationError::Aborted => {
                "numeric parsing cancelled".into()
            }
        })
    }

    pub(super) fn prepare_numeric_expression(
        &self,
        expression: &str,
        params: &ParamContext,
    ) -> Result<PreparedExpression, ExprError> {
        parse_expression_with_abort(expression, self.abort)
            .map_err(ExpressionEvaluationError::from)
            .and_then(|expression| {
                PreparedExpression::compile_with_abort(&expression, params, self.abort)
            })
            .map_err(numeric_expression_error)
    }
}

fn project_numeric(value: crate::ComplexValue, require_real: bool) -> Result<Value, ExprError> {
    if require_real {
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(ExprError::InvalidArgument(
                "numeric field is nonfinite".into(),
            ));
        }
        if !crate::netlist::expr::is_real(value) {
            return Err(ExprError::InvalidArgument(
                "numeric field requires a real value; use real(), imag() or mag() explicitly"
                    .into(),
            ));
        }
    }
    Ok(value.re)
}

impl Deref for TokenStream<'_> {
    type Target = crate::netlist::lexer::TokenStream;
    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl DerefMut for TokenStream<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}
