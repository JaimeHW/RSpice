//! Prescribed smooth forcing admitted by the physical-event descriptor.
//! This is a structural regularity contract, not an inference from equal
//! sampled values. Stateful, switched and circuit-dependent equations need
//! their corresponding event/state owner before this contract can expand.

use super::*;
use crate::expr::{TimeDerivatives, constant_over_time, constant_value};

fn smooth(expr: &Expr, context: &Context<'_>) -> bool {
    if constant_over_time(expr) {
        return true;
    }
    match expr {
        Expr::Time => true,
        Expr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => smooth(operand, context),
        Expr::Binary { op, left, right } => match op {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
                smooth(left, context) && smooth(right, context)
            }
            BinaryOp::Div => {
                smooth(left, context)
                    && constant_value(right, context)
                        .is_some_and(|value| value.is_finite() && value != 0.0)
            }
            BinaryOp::Pow => {
                smooth(left, context)
                    && constant_value(right, context).is_some_and(|value| {
                        value.is_finite() && value >= 0.0 && value.fract() == 0.0
                    })
            }
            _ => false,
        },
        Expr::Function { func, args } => {
            matches!(
                func,
                Function::Exp
                    | Function::Sin
                    | Function::Cos
                    | Function::Atan
                    | Function::Sinh
                    | Function::Cosh
                    | Function::Asinh
                    | Function::Sqr
            ) && args.len() == 1
                && smooth(&args[0], context)
        }
        _ => false,
    }
}

fn prepared<'a>(
    ast: &Expr,
    prescribed: Option<(&'a CompiledExpr, Context<'a>)>,
) -> Option<(&'a CompiledExpr, Context<'a>)> {
    let (program, context) = prescribed?;
    let context = context.with_frequency(0.0).with_ieee_logarithm();
    (TimeDerivatives::supports(program) && smooth(ast, &context)).then_some((program, context))
}

impl BehavioralVoltageSource {
    pub(crate) fn physical_time_program(&self) -> Option<(&CompiledExpr, Context<'_>)> {
        prepared(&self.ast, self.prescribed_time_program())
    }

    pub(crate) fn physical_time_is_stationary(&self) -> bool {
        self.physical_time_program().is_some() && constant_over_time(&self.ast)
    }
}

impl BehavioralCurrentSource {
    pub(crate) fn physical_time_program(&self) -> Option<(&CompiledExpr, Context<'_>)> {
        prepared(&self.ast, self.prescribed_time_program())
    }

    pub(crate) fn physical_time_is_stationary(&self) -> bool {
        self.physical_time_program().is_some() && constant_over_time(&self.ast)
    }
}

impl BehavioralSources {
    pub(crate) fn has_smooth_physical_time_equations(&self) -> bool {
        self.voltage_sources
            .iter()
            .all(|source| source.physical_time_program().is_some())
            && self
                .current_sources
                .iter()
                .all(|source| source.physical_time_program().is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    #[test]
    fn physical_time_contract_distinguishes_smooth_forcing_from_missing_event_owners() {
        for expression in [
            ".7+1u*sin(2*pi*1e9*time)",
            "time^2/3",
            "exp(-time)",
            "atan(time)",
        ] {
            let source = BehavioralVoltageSource::new("b".into(), 1, 0, 1, expression).unwrap();
            let (program, mut context) = source.physical_time_program().unwrap();
            context.time = 0.37;
            assert!(
                TimeDerivatives::new(program, 2, 100_000, &NoAbort)
                    .unwrap()
                    .evaluate(&context, &NoAbort)
                    .is_ok()
            );
            assert!(!source.physical_time_is_stationary());
        }
        for expression in [
            "v(n)",
            "i(v1)",
            "sdt(1)",
            "if(time>1,1,0)",
            "abs(time-1)",
            "1/(time-1)",
            "sqrt(time)",
            "time^0.5",
        ] {
            let voltage = BehavioralVoltageSource::new("b".into(), 1, 0, 1, expression).unwrap();
            let current = BehavioralCurrentSource::new("b".into(), 1, 0, expression).unwrap();
            assert!(voltage.physical_time_program().is_none(), "{expression}");
            assert!(current.physical_time_program().is_none(), "{expression}");
        }
        let source = BehavioralCurrentSource::new("b".into(), 1, 0, "sin(1)+temper/1000").unwrap();
        assert!(source.physical_time_is_stationary());
    }
}
