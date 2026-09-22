//! Shooting admission and accepted expression coordinates for capacitance laws.
use super::*;
use crate::device::behavioral::integrals::{
    BehavioralFqPoint, IntegralBindings, PeriodicExpressionSample, PrescribedIntegralRate,
};
use crate::device::behavioral::periodicity;
use crate::expr::{AcceptedSdtState, Function};

impl SolutionDependentCapacitor {
    fn integral_bindings(&self, time: Value, state_start: usize) -> IntegralBindings<'_> {
        IntegralBindings {
            name: &self.name,
            nodes: &self.node_bindings,
            branches: &self.branch_bindings,
            state_start,
            environment: BehavioralEnvironment {
                time,
                frequency: self.frequency,
                temperature: self.temperature,
                gmin: self.gmin,
                expression_dialect: self.expression_dialect,
                logarithm_domain: LogarithmDomain::Guarded,
            },
        }
    }

    pub(crate) fn integral_names(&self) -> impl Iterator<Item = String> + '_ {
        (0..self.program.sdt_count).map(|index| format!("C:{}:sdt:{index}", self.name))
    }

    pub(crate) fn append_prescribed_integral_rates<'a>(
        &'a self,
        plans: &mut Vec<PrescribedIntegralRate<'a>>,
    ) -> Result<(), String> {
        if let Some(equations) = &self.integral_equations {
            equations.append_prescribed_rates("C", self.integral_bindings(0.0, 0), plans)?;
        }
        Ok(())
    }

    pub(crate) fn sample_periodic_capacitance(
        &mut self,
        point: BehavioralFqPoint<'_>,
        state_start: usize,
    ) -> Result<PeriodicExpressionSample, String> {
        if let Some(equations) = &self.integral_equations {
            equations.periodic_output(point, self.integral_bindings(point.time, state_start))
        } else {
            let sample = self.linearize(point.inputs, point.time);
            Ok(PeriodicExpressionSample {
                value: sample.value,
                partials: sample.partials,
            })
        }
    }

    pub(crate) fn stamp_periodic_integrals(
        &self,
        point: BehavioralFqPoint<'_>,
        state_start: usize,
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        if let Some(equations) = &self.integral_equations {
            equations.stamp_periodic_integrals(
                point,
                self.integral_bindings(point.time, state_start),
                f,
                q,
            )?;
        }
        Ok(())
    }

    pub(crate) fn periodicity_context(&self) -> Context<'_> {
        Context::transient(&[], &[], 0.0)
            .with_temperature(self.temperature)
            .with_frequency(self.frequency)
            .with_gmin(self.gmin)
            .with_expression_dialect(self.expression_dialect)
    }

    pub(crate) fn periodic_expression(&self) -> &Expr {
        &self.ast
    }

    pub(crate) fn has_periodic_shooting_equation(&self, period: Value, autonomous: bool) -> bool {
        periodicity::carrier_frequency_context(
            crate::device::behavioral::expression_depends_on_frequency(&self.ast),
            self.frequency,
            self.expression_dialect,
        ) && periodicity::time_increment_with_state(
            &self.ast,
            period,
            &self.periodicity_context(),
            autonomous,
            true,
        ) == Some(0.0)
    }

    pub(crate) fn max_authored_tone_cycles(&self, period: Value) -> Value {
        let context = self.periodicity_context();
        periodicity::max_authored_tone_cycles(&self.ast, period, &context)
            .max(periodicity::finite_fourier_degree(&self.ast, period, &context).unwrap_or(0.0))
    }

    pub(crate) fn minimum_pss_interval(&self) -> Option<Value> {
        periodicity::minimum_pss_interval(&self.ast, &self.periodicity_context(), true)
    }

    pub(crate) fn minimum_periodic_collocation_interval(&self) -> Option<Value> {
        periodicity::minimum_pss_interval(&self.ast, &self.periodicity_context(), false)
    }

    pub(crate) fn has_periodic_response_context(&self) -> bool {
        !crate::device::behavioral::expression_depends_on_frequency(&self.ast)
    }

    pub(crate) fn needs_time_resolution(&self, period: Value) -> bool {
        periodicity::needs_time_features(&self.ast, period, &self.periodicity_context())
    }

    pub(crate) fn validate_periodic_integral_rates(&self) -> Result<(), String> {
        fn check(expr: &Expr, context: &Context<'_>) -> Option<Value> {
            match expr {
                Expr::Function { func, args } => {
                    if *func == Function::Sdt
                        && let [input] = args.as_slice()
                        && let Some(value) = crate::expr::constant_value(input, context)
                        && value != 0.0
                    {
                        return Some(value);
                    }
                    args.iter().find_map(|arg| check(arg, context))
                }
                Expr::Unary { operand, .. } => check(operand, context),
                Expr::Binary { left, right, .. } => {
                    check(left, context).or_else(|| check(right, context))
                }
                Expr::LookupTable { input, .. } => check(input, context),
                _ => None,
            }
        }
        if let Some(value) = check(&self.ast, &self.periodicity_context()) {
            return Err(format!(
                "capacitor '{}' SDT has constant nonzero input {value:e}; its integral cannot be periodic",
                self.name
            ));
        }
        Ok(())
    }

    pub(crate) fn accepted_integrals(&self) -> impl Iterator<Item = Value> + '_ {
        self.vm.accepted_integrals(self.program.sdt_count)
    }

    pub(crate) fn reset_integrals(&mut self, values: &[Value]) -> Result<(), String> {
        if values.len() != self.program.sdt_count || values.iter().any(|v| !v.is_finite()) {
            return Err(format!(
                "capacitor '{}' has invalid SDT shooting coordinates",
                self.name
            ));
        }
        self.vm.restore_sdt_history(
            &values
                .iter()
                .map(|&integral| AcceptedSdtState {
                    integral,
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
        );
        Ok(())
    }

    pub(crate) fn rebase_accepted_history(&mut self, time: Value) {
        let mut history = self.accepted_sdt_history();
        for state in &mut history {
            state.time = time;
        }
        self.restore_sdt_history(&history);
    }
}
