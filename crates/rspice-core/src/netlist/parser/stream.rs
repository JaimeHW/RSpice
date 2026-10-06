//! Parser tokens with optional, resumable numeric reads for deferred cards.
use super::*;
use crate::netlist::expr::{
    ExprError, PreparedExpression, PreparedProgress, normalize_xyce_expression_result,
    parse_expression,
};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub(super) struct TokenStream {
    tokens: crate::netlist::lexer::TokenStream,
    bindings: Option<Box<NumericBindings>>,
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
        original: &Self,
        name: String,
        value: crate::ComplexValue,
    ) {
        let bindings = self.bindings.as_mut().expect("binding mode");
        bindings.values.insert(name, value);
        bindings.missing = None;
        self.tokens = original.tokens.clone();
    }

    /// Read before advancing the token: its authored byte position identifies
    /// the operand across grammar retries, including nested MC analysis cards.
    pub(super) fn numeric_expression(
        &mut self,
        expression: &str,
        params: &ParamContext,
    ) -> Result<Value, ExprError> {
        let key = self.peek().span.start;
        let Some(bindings) = &mut self.bindings else {
            return eval_expression(expression, params);
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
                    Ok(if params.expression_dialect() == ExpressionDialect::Xyce {
                        normalize_xyce_expression_result(value).re
                    } else {
                        value.re
                    })
                }
                Err(error) => Err(error),
            };
        *operand = Operand::Complete(result.clone());
        result
    }
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
