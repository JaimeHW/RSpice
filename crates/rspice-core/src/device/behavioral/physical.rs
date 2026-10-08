//! Smooth physical-event equations and their analytic derivatives.
//! Voltage sources currently prescribe time; current sources can also depend
//! on nodal coordinates. Memory and branch-current impulses require their
//! own state/descriptor owner, independently of a finite sample at one bias.

use super::*;
use crate::abort_signal::AbortSignal;
use crate::expr::{TimeDerivativeError, TimeDerivatives, constant_over_time, constant_value};
use crate::resource::{ResourceKind, ResourceLimitError};

fn smooth(expr: &Expr, context: &Context<'_>, nodal: bool) -> bool {
    if constant_over_time(expr) {
        return true;
    }
    match expr {
        Expr::Time => true,
        Expr::NodeVoltage(_) => nodal,
        Expr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => smooth(operand, context, nodal),
        Expr::Binary { op, left, right } => match op {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
                smooth(left, context, nodal) && smooth(right, context, nodal)
            }
            BinaryOp::Div => {
                smooth(left, context, nodal)
                    && constant_value(right, context)
                        .is_some_and(|value| value.is_finite() && value != 0.0)
            }
            BinaryOp::Pow => {
                smooth(left, context, nodal)
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
                && smooth(&args[0], context, nodal)
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
    (TimeDerivatives::supports(program) && smooth(ast, &context, false))
        .then_some((program, context))
}

impl BehavioralVoltageSource {
    pub(crate) fn physical_time_program(&self) -> Option<(&CompiledExpr, Context<'_>)> {
        prepared(&self.ast, self.prescribed_time_program())
    }

    pub(crate) fn physical_time_is_stationary(&self) -> bool {
        self.physical_time_program().is_some() && constant_over_time(&self.ast)
    }
}

/// Current and physical derivatives at fixed event coordinates. These are
/// constitutive values, not a Newton affine RHS or accepted device history.
pub(crate) struct PhysicalCurrentSample {
    pub value: Value,
    pub time_partial: Value,
    pub partials: Vec<(usize, Value)>,
}

impl BehavioralCurrentSource {
    pub(crate) fn has_smooth_physical_current_equation(&self) -> bool {
        if self.program.node_map.is_empty() {
            return self.physical_time_program().is_some();
        }
        self.program.sdt_count == 0
            && self.program.branch_map.is_empty()
            && smooth(
                &self.ast,
                &self
                    .periodicity_context()
                    .with_frequency(0.0)
                    .with_ieee_logarithm(),
                true,
            )
    }

    /// Read-only physical sampling. Numerical-domain failures remain NaNs so
    /// the event Newton owner can backtrack; binding, quota and cancellation
    /// failures are returned before any trial can be accepted.
    pub(crate) fn physical_current_sample(
        &self,
        solution: &[Value],
        time: Value,
        max_values: usize,
        abort: &dyn AbortSignal,
    ) -> Result<PhysicalCurrentSample, TimeDerivativeError> {
        if abort.is_aborted() {
            return Err(TimeDerivativeError::Aborted);
        }
        if !self.has_smooth_physical_current_equation()
            || self.node_bindings.len() != self.program.node_map.len()
            || self
                .node_bindings
                .iter()
                .flatten()
                .any(|&index| index >= solution.len())
        {
            return Err(TimeDerivativeError::Unsupported);
        }
        // Inputs, output partials, VM stack and recursive analytic workspace.
        // Include scaled derivative arithmetic before any scratch allocation.
        let words = self
            .program
            .instructions
            .len()
            .saturating_mul(32)
            .saturating_add(self.node_bindings.len().saturating_mul(8))
            .saturating_add(32 * 1024);
        ResourceLimitError::ensure(ResourceKind::ResultValues, words, max_values)?;
        let mut nodes = Vec::with_capacity(self.node_bindings.len());
        for binding in &self.node_bindings {
            if abort.is_aborted() {
                return Err(TimeDerivativeError::Aborted);
            }
            nodes.push(binding.map_or(0.0, |index| solution[index]));
        }
        let context = Context::transient(&nodes, &[], time)
            .with_temperature(self.temperature)
            .with_frequency(0.0)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect)
            .with_ieee_logarithm();
        let value = Vm::new().execute(&self.program, &context);
        let environment = BehavioralEnvironment {
            time,
            frequency: 0.0,
            temperature: self.temperature,
            gmin: self.gmin,
            expression_dialect: self.expression_dialect,
            logarithm_domain: LogarithmDomain::Ieee,
        };
        let derivative = |target| {
            analytic_expression_partial(&self.ast, &self.program, &nodes, &[], environment, target)
                .unwrap_or(Value::NAN)
        };
        let mut partials = Vec::with_capacity(nodes.len());
        for (local, binding) in self.node_bindings.iter().enumerate() {
            if abort.is_aborted() {
                return Err(TimeDerivativeError::Aborted);
            }
            if let Some(column) = binding {
                partials.push((*column, derivative(DerivativeTarget::PhysicalNode(local))));
            }
        }
        let time_partial = derivative(DerivativeTarget::Time);
        if abort.is_aborted() {
            return Err(TimeDerivativeError::Aborted);
        }
        Ok(PhysicalCurrentSample {
            value,
            time_partial,
            partials,
        })
    }
    pub(crate) fn physical_time_program(&self) -> Option<(&CompiledExpr, Context<'_>)> {
        prepared(&self.ast, self.prescribed_time_program())
    }

    pub(crate) fn physical_time_is_stationary(&self) -> bool {
        self.physical_time_program().is_some() && constant_over_time(&self.ast)
    }
}

impl BehavioralSources {
    pub(crate) fn has_smooth_physical_equations(&self) -> bool {
        self.voltage_sources
            .iter()
            .all(|source| source.physical_time_program().is_some())
            && self
                .current_sources
                .iter()
                .all(BehavioralCurrentSource::has_smooth_physical_current_equation)
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

    fn nodal_source(expression: &str, dialect: ExpressionDialect) -> BehavioralCurrentSource {
        let mut source = BehavioralCurrentSource::new("b".into(), 1, 0, expression).unwrap();
        source
            .bind_references(
                |name| match name.to_ascii_lowercase().as_str() {
                    "a" => Some(1),
                    "b" => Some(2),
                    _ => None,
                },
                |_| BehavioralBranchResolution::MissingDevice,
            )
            .unwrap();
        source.set_expression_dialect(dialect);
        source
    }

    #[test]
    fn physical_nodal_current_partials_are_physical_at_zero_and_hold_inputs_fixed_in_time() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            // Xyce's Newton slope for x^1 at zero is regularized to zero;
            // the physical event Jacobian must retain the actual unit slope.
            let source = nodal_source("v(a)^1 + 2^-1*v(b)^3*(1+time^2)", dialect);
            assert!(source.has_smooth_physical_current_equation());
            let sample = source
                .physical_current_sample(&[0.0, -2.0], 0.5, 100_000, &NoAbort)
                .unwrap();
            assert_eq!(sample.value, -5.0);
            assert_eq!(sample.time_partial, -4.0);
            let partial = |column| {
                sample
                    .partials
                    .iter()
                    .filter(|(c, _)| *c == column)
                    .map(|(_, p)| *p)
                    .sum::<Value>()
            };
            assert_eq!(partial(0), 1.0);
            assert_eq!(partial(1), 7.5);
            assert!(source.physical_time_program().is_none());
            for expression in [
                "abs(v(a))",
                "if(v(a)>0,1,0)",
                "sdt(v(a))",
                "1/v(a)",
                "v(a)^.5",
            ] {
                assert!(
                    !nodal_source(expression, dialect).has_smooth_physical_current_equation(),
                    "{expression}"
                );
            }
        }
    }

    #[test]
    fn physical_nodal_current_sampling_is_bounded_cancellable_and_read_only() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct StopAt {
            count: AtomicUsize,
            stop: usize,
        }
        impl AbortSignal for StopAt {
            fn is_aborted(&self) -> bool {
                self.count.fetch_add(1, Ordering::SeqCst) + 1 >= self.stop
            }
        }
        let source = nodal_source("v(a)*v(b)+time", ExpressionDialect::Ngspice);
        assert!(
            matches!(source.physical_current_sample(&[2.0, 3.0], 0.5, 100, &NoAbort),
            Err(TimeDerivativeError::Resource(e)) if e.limit == 100 && e.requested > e.limit)
        );
        assert!(matches!(
            source.physical_current_sample(&[2.0], 0.5, 100_000, &NoAbort),
            Err(TimeDerivativeError::Unsupported)
        ));
        for stop in [1, 3, 5] {
            let abort = StopAt {
                count: AtomicUsize::new(0),
                stop,
            };
            assert!(matches!(
                source.physical_current_sample(&[2.0, 3.0], 0.5, 100_000, &abort),
                Err(TimeDerivativeError::Aborted)
            ));
            assert_eq!(abort.count.load(Ordering::SeqCst), stop);
        }
        let sample = source
            .physical_current_sample(&[2.0, 3.0], 0.5, 100_000, &NoAbort)
            .unwrap();
        assert_eq!(sample.value, 6.5);
        assert_eq!(sample.time_partial, 1.0);
        // Event sampling must not alter the source's ordinary Newton/history scratch.
        assert!(source.node_values.iter().all(|value| *value == 0.0));
        assert!(source.node_partials.iter().all(|value| *value == 0.0));
    }
}
