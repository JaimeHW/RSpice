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

/// Evaluate a simple expression without parameters
pub fn eval_simple(input: &str) -> Result<Value, ExprError> {
    eval_expression(input, &ParamContext::new())
}
