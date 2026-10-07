//! Typed scalar values supplied to source specialization.

use crate::ast::{Expression, FourStateLit, NumberLit, ParamType, ParameterDecl};
use crate::canonical_ir::digital_value::FourStateValue;
use crate::error::{CompileError, CompileResult};
use crate::semantic::DigitalConstants;
use crate::source::Span;

/// An exact scalar override for a Verilog-AMS module parameter.
///
/// Integral values retain their width, signedness and X/Z bits until the
/// declaration's assignment conversion. A real is supplied as binary64.
/// Arrays and strings are not represented by this scalar API.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalarParameterValue {
    /// A signed decimal integer with at least 32 bits.
    Integer(i64),
    /// An explicitly sized four-state integral value.
    Bits { value: FourStateValue, signed: bool },
    /// A finite binary64 real, including either sign of zero.
    Real(f64),
}

impl ScalarParameterValue {
    fn expression(&self, name: &str, span: Span) -> CompileResult<Expression> {
        let refusal = |detail: &str| {
            CompileError::ModuleSelection(format!("parameter override `{name}` {detail}"))
        };
        let (value, signed) = match self {
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

    pub(crate) fn assigned_expression(
        &self,
        parameter: &ParameterDecl,
        time_scale: crate::time_scale::ModuleTimeScale,
    ) -> CompileResult<Expression> {
        if parameter.param_type == ParamType::String || !parameter.dimensions.is_empty() {
            return Err(CompileError::ModuleSelection(format!(
                "parameter `{}` requires an array or string override",
                parameter.name
            )));
        }
        let mut declaration = parameter.clone();
        declaration.default = Some(self.expression(&parameter.name, parameter.span)?);
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
