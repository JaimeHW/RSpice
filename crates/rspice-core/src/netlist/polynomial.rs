//! Bounded polynomial lowering shared by controlled sources and expressions.

use crate::abort_signal::AbortSignal;
use crate::resource::{MAX_EXPRESSION_TREE_DEPTH, MAX_POLYNOMIAL_EXPANSION_BYTES};

#[derive(Clone, Copy)]
pub(super) enum PolynomialOrdering {
    /// SPICE2: x, y, x*x, x*y, y*y, ...
    Symmetric,
    /// Xyce expressions: x, y, x*x, x*y, y*x, y*y, ...
    Ordered,
}

#[derive(Debug, thiserror::Error)]
pub(super) enum PolynomialExpansionError {
    #[error("POLY expansion was cancelled")]
    Aborted,
    #[error("POLY requires at least one controlling expression")]
    MissingVariables,
    #[error("POLY expansion exceeds the expression stack safety limit")]
    Depth,
    #[error("POLY expansion exceeds the {MAX_POLYNOMIAL_EXPANSION_BYTES}-byte limit")]
    Bytes,
}

/// Emit only the monomials with supplied coefficients. Never enumerate an
/// entire combinatorial degree or recurse once per controlling variable.
pub(super) fn expand_polynomial(
    variables: &[String],
    coefficients: &[String],
    ordering: PolynomialOrdering,
    abort: &dyn AbortSignal,
) -> Result<String, PolynomialExpansionError> {
    if variables.is_empty() {
        return Err(PolynomialExpansionError::MissingVariables);
    }
    let mut result = String::new();
    // Digits identify one monomial's variable factors. Saturate the degree
    // once no nonzero coefficient can fit the retained expression tree.
    let mut digits = Vec::new();
    let mut degree_too_large = false;
    for (index, coefficient) in coefficients.iter().enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(PolynomialExpansionError::Aborted);
        }
        if index != 0 && !degree_too_large {
            if let Some(slot) = digits
                .iter()
                .rposition(|&digit| digit + 1 < variables.len())
            {
                digits[slot] += 1;
                let fill = match ordering {
                    PolynomialOrdering::Symmetric => digits[slot],
                    PolynomialOrdering::Ordered => 0,
                };
                digits[slot + 1..].fill(fill);
            } else if digits.len() + 2 >= MAX_EXPRESSION_TREE_DEPTH {
                degree_too_large = true;
            } else {
                digits.fill(0);
                digits.push(0);
            }
        }
        if coefficient.parse::<f64>().is_ok_and(|value| value == 0.0) {
            continue;
        }
        if degree_too_large {
            return Err(PolynomialExpansionError::Depth);
        }
        let bytes = digits.iter().fold(
            result
                .len()
                .saturating_add(coefficient.len())
                .saturating_add(5),
            |bytes, &digit| {
                bytes
                    .saturating_add(variables[digit].len())
                    .saturating_add(3)
            },
        );
        if bytes > MAX_POLYNOMIAL_EXPANSION_BYTES {
            return Err(PolynomialExpansionError::Bytes);
        }
        if !result.is_empty() {
            result.push_str(" + ");
        }
        result.push('(');
        result.push_str(coefficient);
        result.push(')');
        for &digit in &digits {
            result.push_str("*(");
            result.push_str(&variables[digit]);
            result.push(')');
        }
    }
    if result.is_empty() {
        result.push('0');
    }
    Ok(result)
}
