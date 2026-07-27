//! Public entry points for parsing and evaluating expressions.
//!
//! `parse_expression` yields an AST that can be evaluated repeatedly, which
//! is what callers inside the Newton loop want. The `eval_*` helpers parse
//! and evaluate in one call for one-shot use. Each comes in a real-valued
//! form and a `_complex` form; the real form takes the real component.

use super::*;

//=============================================================================
// Public API
//=============================================================================

/// Parse a SPICE expression string into an AST
pub fn parse_expression(input: &str) -> Result<Expr, ExprError> {
    let mut parser = ExprParser::new(input);
    parser.parse()
}

/// Parse and evaluate a SPICE expression with the given context
pub fn eval_expression(input: &str, ctx: &ParamContext) -> Result<Value, ExprError> {
    let expr = parse_expression(input)?;
    evaluate(&expr, ctx)
}

/// Parse and evaluate a SPICE expression, preserving complex values.
pub fn eval_expression_complex(input: &str, ctx: &ParamContext) -> Result<ComplexValue, ExprError> {
    let expr = parse_expression(input)?;
    evaluate_complex(&expr, ctx)
}

/// Evaluate a simple expression without parameters
pub fn eval_simple(input: &str) -> Result<Value, ExprError> {
    eval_expression(input, &ParamContext::new())
}

/// Evaluate a simple expression without parameters, preserving complex values.
pub fn eval_simple_complex(input: &str) -> Result<ComplexValue, ExprError> {
    eval_expression_complex(input, &ParamContext::new())
}
