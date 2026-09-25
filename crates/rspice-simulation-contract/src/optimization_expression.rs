//! Authored optimization expression spelling shared by draft and executor.

pub fn validate_optimization_expression(expression: &str) -> Result<(), String> {
    if expression.trim().is_empty() {
        return Err("Optimization expression must not be empty".into());
    }
    if expression
        .chars()
        .any(|ch| ch.is_control() || matches!(ch, '{' | '}'))
    {
        return Err(
            "Enter one optimization expression without enclosing braces or line breaks".into(),
        );
    }
    Ok(())
}
