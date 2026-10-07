//! Parser tokens with optional, resumable numeric reads for deferred cards.
use super::*;
use crate::netlist::expr::{
    ExprError, PreparedExpression, PreparedProgress, eval_expression_complex,
    normalize_xyce_expression_result, parse_expression,
};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub(super) struct TokenStream {
    tokens: crate::netlist::lexer::TokenStream,
    bindings: Option<Box<NumericBindings>>,
    require_real_values: bool,
    optional_numeric_failure: Option<Box<str>>,
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

impl TokenStream {
    pub(super) fn new(tokens: Vec<crate::netlist::lexer::Token>) -> Self {
        Self {
            tokens: crate::netlist::lexer::TokenStream::new(tokens),
            bindings: None,
            require_real_values: false,
            optional_numeric_failure: None,
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
            self.numeric_value(eval_expression_complex(spelling, params)?)
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
        let Some(bindings) = &mut self.bindings else {
            return project_numeric(eval_expression_complex(expression, params)?, require_real);
        };
        // A grammar can probe optional operands after one failed read. Do not
        // evaluate later operands ahead of that dependency's random draws.
        if let Some(name) = &bindings.missing {
            return Err(ExprError::UndefinedParam(name.clone()));
        }
        let operand = bindings.operands.entry(key).or_insert_with(|| {
            match parse_expression(expression)
                .and_then(|expr| PreparedExpression::compile(&expr, params))
            {
                Ok(mut program) => {
                    program.begin_evaluation();
                    Operand::Pending(Box::new(program))
                }
                Err(error) => Operand::Complete(Err(error)),
            }
        });
        let program = match operand {
            Operand::Pending(program) => program,
            Operand::Complete(result) => return result.clone(),
        };
        let result =
            match program.resume_with(params, &mut |name| Ok(bindings.values.get(name).copied())) {
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
                Err(error) => Err(error),
            };
        *operand = Operand::Complete(result.clone());
        result
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

impl Deref for TokenStream {
    type Target = crate::netlist::lexer::TokenStream;
    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl DerefMut for TokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}
