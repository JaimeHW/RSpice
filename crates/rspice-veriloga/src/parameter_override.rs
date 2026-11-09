//! Typed scalar values supplied to source specialization.

use crate::ast::{
    ArrayLiteralElement, ArrayLiteralExpr, Expression, FourStateLit, NumberLit, ParamType,
    ParameterDecl, StringLit,
};
use crate::canonical_ir::digital_value::FourStateValue;
use crate::error::{CompileError, CompileResult};
use crate::semantic::DigitalConstants;
use crate::source::Span;

/// An exact scalar override for a Verilog-AMS module parameter.
///
/// Integral values retain their width, signedness and X/Z bits until the
/// declaration's assignment conversion. A real is supplied as binary64. A
/// string reaches only a `string` parameter, and a whole array only a
/// parameter array.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ScalarParameterValue {
    /// A signed decimal integer with at least 32 bits.
    Integer(i64),
    /// An explicitly sized four-state integral value.
    Bits { value: FourStateValue, signed: bool },
    /// A finite binary64 real, including either sign of zero.
    Real(f64),
    /// The value of a `string` parameter, without quotes or escapes.
    String(smol_str::SmolStr),
    /// Every element of a parameter array, in declaration order.
    RealArray(Vec<f64>),
}

impl ScalarParameterValue {
    fn expression(&self, name: &str, span: Span) -> CompileResult<Expression> {
        let refusal = |detail: &str| {
            CompileError::ModuleSelection(format!("parameter override `{name}` {detail}"))
        };
        let (value, signed) = match self {
            Self::String(value) => {
                return Ok(Expression::StringLit(StringLit {
                    value: value.clone(),
                    span,
                }));
            }
            Self::Integer(value) => (
                FourStateValue::from_integer(
                    crate::numeric_literal::unsized_integer_width(*value),
                    i128::from(*value),
                ),
                true,
            ),
            Self::Bits { value, signed } => {
                if value.width() == 0 || value.width() > crate::semantic::MAX_DIGITAL_VECTOR_WIDTH {
                    return Err(refusal("exceeds the supported packed width"));
                }
                (value.clone(), *signed)
            }
            Self::RealArray(_) => {
                return Err(refusal("is an array and takes no single value"));
            }
            Self::Real(value) => {
                if !value.is_finite() {
                    return Err(refusal("must be finite"));
                }
                return Ok(Expression::Number(NumberLit {
                    value: *value,
                    raw: format!("{value:e}").into(),
                    span,
                }));
            }
        };
        let raw = format!(
            "{}'{}b{}",
            value.width(),
            if signed { "s" } else { "" },
            value.spelling()
        );
        let literal =
            crate::four_state::decode(&raw).map_err(|error| refusal(&error.to_string()))?;
        Ok(Expression::Digital(crate::ast::DigitalExpr::FourState(
            FourStateLit {
                value: literal,
                span,
            },
        )))
    }

    /// `'{...}` for a parameter array, element by element as its type reads
    /// them: an integer array takes whole numbers only. A wrong element count
    /// is the replacement check's to refuse.
    fn array_expression(parameter: &ParameterDecl, values: &[f64]) -> CompileResult<Expression> {
        let refuse = |detail: String| {
            CompileError::ModuleSelection(format!(
                "parameter override `{}` {detail}",
                parameter.name
            ))
        };
        if parameter.dimensions.is_empty() {
            return Err(refuse("is not an array and takes one value".into()));
        }
        let integer = parameter.param_type == ParamType::Integer;
        let span = parameter.span;
        let mut elements = Vec::with_capacity(values.len());
        for &value in values {
            if !value.is_finite() {
                return Err(refuse("must have finite elements".into()));
            }
            if integer && value.fract() != 0.0 {
                return Err(refuse(format!(
                    "is an integer array and cannot take {value}"
                )));
            }
            let raw = if integer {
                format!("{value:.0}")
            } else {
                format!("{value:e}")
            };
            elements.push(ArrayLiteralElement::Value(Expression::Number(NumberLit {
                value,
                raw: raw.into(),
                span,
            })));
        }
        Ok(Expression::ArrayLiteral(ArrayLiteralExpr {
            elements,
            assignment_pattern: true,
            span,
        }))
    }

    pub(crate) fn assigned_expression(
        &self,
        parameter: &ParameterDecl,
        time_scale: crate::time_scale::ModuleTimeScale,
    ) -> CompileResult<Expression> {
        if let Self::RealArray(values) = self {
            return Self::array_expression(parameter, values);
        }
        if !parameter.dimensions.is_empty() {
            return Err(CompileError::ModuleSelection(format!(
                "parameter `{}` requires an array override",
                parameter.name
            )));
        }
        // A string is not a number, and a number is not a string.
        let is_string = matches!(self, Self::String(_));
        if (parameter.param_type == ParamType::String) != is_string {
            return Err(CompileError::ModuleSelection(format!(
                "parameter `{}` {}",
                parameter.name,
                if is_string {
                    "is not a string parameter and cannot take a string value"
                } else {
                    "is a string parameter and requires a string value"
                }
            )));
        }
        let expression = self.expression(&parameter.name, parameter.span)?;
        if is_string {
            return Ok(expression);
        }
        if parameter.packed_range.is_some() || parameter.signedness.is_some() {
            // Bounds may name another simultaneously overridden parameter.
            // Convert after the complete effective source scope is installed.
            return Ok(expression);
        }
        let mut declaration = parameter.clone();
        declaration.default = Some(expression);
        crate::canonical_ir::digital_lower::parameter_override_literal(
            &declaration,
            &DigitalConstants::default(),
            time_scale,
        )
        .map_err(|message| {
            CompileError::ModuleSelection(format!(
                "parameter override `{}`: {message}",
                parameter.name
            ))
        })
    }
}

/// Stable typed source assignments; independent of JSON floating-point spelling.
pub(crate) fn specialization_identity(
    values: &[(smol_str::SmolStr, ScalarParameterValue)],
) -> [u8; 32] {
    if values.is_empty() {
        return [0; 32];
    }
    let mut hash = blake3::Hasher::new();
    hash.update(b"rspice.source-specialization\0");
    for (name, value) in values {
        hash.update(&(name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
        match value {
            ScalarParameterValue::Real(value) => {
                hash.update(&[0]);
                hash.update(&value.to_bits().to_le_bytes());
            }
            ScalarParameterValue::Integer(value) => {
                hash.update(&[1]);
                hash.update(&value.to_le_bytes());
            }
            ScalarParameterValue::Bits { value, signed } => {
                hash.update(&[2, u8::from(*signed)]);
                hash.update(&value.width().to_le_bytes());
                hash.update(value.spelling().as_bytes());
            }
            ScalarParameterValue::String(value) => {
                hash.update(&[3]);
                hash.update(&(value.len() as u64).to_le_bytes());
                hash.update(value.as_bytes());
            }
            ScalarParameterValue::RealArray(values) => {
                hash.update(&[4]);
                hash.update(&(values.len() as u64).to_le_bytes());
                for value in values {
                    hash.update(&value.to_bits().to_le_bytes());
                }
            }
        }
    }
    *hash.finalize().as_bytes()
}
