//! Separate each accepted integral coordinate from its instantaneous input.
//! Private scalar leaves preserve compiler postorder without exposing solver
//! state as authored node names or rounding a tiny derivative against history.

use super::*;
use crate::expr::AcceptedSdtState;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Input {
    Node(usize),
    Branch(usize),
    Integral(usize),
}

#[derive(Debug, Clone)]
struct Equation {
    ast: Expr,
    program: CompiledExpr,
    inputs: Vec<Input>,
}

impl Equation {
    fn new(ast: Expr, inputs: &HashMap<String, Input>) -> Self {
        let program = compile(&ast);
        let mut bindings = vec![Input::Node(0); program.node_map.len()];
        for (name, &index) in &program.node_map {
            bindings[index] = inputs[name];
        }
        Self {
            ast,
            program,
            inputs: bindings,
        }
    }

    fn evaluate(
        &self,
        values: impl Fn(Input) -> (Value, Derivative),
        environment: BehavioralEnvironment,
    ) -> Option<(Value, Derivative)> {
        let (values, directions): (Vec<_>, Vec<_>) =
            self.inputs.iter().copied().map(values).unzip();
        compiled_expression_node_direction(
            &self.ast,
            &self.program,
            &values,
            &directions,
            environment,
        )
    }
}

#[derive(Debug, Clone)]
pub(super) struct IntegralEquations {
    rates: Vec<Equation>,
    output: Equation,
}

struct Lower<'a> {
    original: &'a CompiledExpr,
    inputs: HashMap<String, Input>,
    names: HashMap<Input, String>,
    rates: Vec<Equation>,
}

impl Lower<'_> {
    fn scalar(&mut self, input: Input) -> Expr {
        let next = self.names.len();
        let name = self
            .names
            .entry(input)
            .or_insert_with(|| format!("\0integral_input_{next}"))
            .clone();
        self.inputs.insert(name.clone(), input);
        Expr::NodeVoltage(name)
    }

    fn lower(&mut self, expression: &Expr) -> Expr {
        match expression {
            Expr::NodeVoltage(name) => self.scalar(Input::Node(self.original.node_map[name])),
            Expr::BranchCurrent(name) => self.scalar(Input::Branch(self.original.branch_map[name])),
            Expr::Unary { op, operand } => Expr::Unary {
                op: *op,
                operand: Box::new(self.lower(operand)),
            },
            Expr::Binary { op, left, right } => Expr::Binary {
                op: *op,
                left: Box::new(self.lower(left)),
                right: Box::new(self.lower(right)),
            },
            Expr::Function { func, args } => {
                let mut args = args.iter().map(|arg| self.lower(arg)).collect::<Vec<_>>();
                if *func == Function::Sdt {
                    let index = self.rates.len();
                    self.rates.push(Equation::new(
                        args.pop().expect("SDT has one input"),
                        &self.inputs,
                    ));
                    self.scalar(Input::Integral(index))
                } else {
                    Expr::Function { func: *func, args }
                }
            }
            Expr::LookupTable { input, table } => Expr::LookupTable {
                input: Box::new(self.lower(input)),
                table: table.clone(),
            },
            _ => expression.clone(),
        }
    }
}

impl IntegralEquations {
    pub(super) fn new(ast: &Expr, program: &CompiledExpr) -> Option<Self> {
        if program.sdt_count == 0 {
            return None;
        }
        let mut lower = Lower {
            original: program,
            inputs: HashMap::new(),
            names: HashMap::new(),
            rates: Vec::new(),
        };
        let output = lower.lower(ast);
        assert_eq!(lower.rates.len(), program.sdt_count);
        Some(Self {
            output: Equation::new(output, &lower.inputs),
            rates: lower.rates,
        })
    }

    /// Differentiate the actual trapezoidal VM update while holding accepted
    /// history fixed. Nested states consume their inner trial value/derivative.
    pub(super) fn transient_partial(
        &self,
        nodes: &[Value],
        branches: &[Value],
        history: &[AcceptedSdtState],
        environment: BehavioralEnvironment,
        target: DerivativeTarget<'_>,
    ) -> Option<Value> {
        let physical = |input| match input {
            Input::Node(i) => (
                nodes[i],
                Value::from(matches!(target, DerivativeTarget::Node(j) if j == i)).into(),
            ),
            Input::Branch(i) => (
                branches[i],
                Value::from(matches!(target, DerivativeTarget::Branch(j) if j == i)).into(),
            ),
            Input::Integral(_) => unreachable!(),
        };
        let mut integrals: Vec<(Value, Derivative)> = Vec::with_capacity(self.rates.len());
        for (rate, accepted) in self.rates.iter().zip(history) {
            let (input, direction) = rate.evaluate(
                |binding| match binding {
                    Input::Integral(i) => integrals[i],
                    _ => physical(binding),
                },
                environment,
            )?;
            let dt = if environment.time == 0.0 {
                0.0
            } else {
                (environment.time - accepted.time).max(0.0)
            };
            integrals.push((
                accepted.integral + 0.5 * (accepted.input + input) * dt,
                direction * 0.5 * dt,
            ));
        }
        let (_, derivative) = self.output.evaluate(
            |binding| match binding {
                Input::Integral(i) => integrals[i],
                _ => physical(binding),
            },
            environment,
        )?;
        let derivative =
            normalize_expression_boundary(derivative.binary64(), environment.expression_dialect);
        derivative.is_finite().then_some(derivative)
    }
}

macro_rules! integral_source {
    ($kind:ty) => {
        impl $kind {
            fn append_integral_rate_directions(
                &self,
                solution: &[Value],
                direction: &[Value],
                integrals: &[Value],
                time: Value,
                output: &mut Vec<Value>,
            ) -> Result<(), String> {
                let Some(equations) = &self.integral_equations else { return Ok(()); };
                let environment = BehavioralEnvironment {
                    time, frequency: self.frequency, temperature: self.temperature,
                    gmin: self.gmin, expression_dialect: self.expression_dialect,
                    logarithm_domain: LogarithmDomain::Ieee,
                };
                for (index, rate) in equations.rates.iter().enumerate() {
                    let values = |input| {
                        let binding = match input {
                            Input::Integral(i) => return (integrals[i], 0.0.into()),
                            Input::Node(i) => self.node_bindings[i],
                            Input::Branch(i) => self.branch_bindings[i],
                        };
                        binding.map_or((0.0, 0.0.into()), |i| (solution[i], direction[i].into()))
                    };
                    let (value, derivative) = rate.evaluate(values, environment).ok_or_else(|| format!(
                        "behavioral source '{}' SDT {index} has no analytic input derivative", self.name))?;
                    let derivative = derivative.binary64();
                    if !value.is_finite() || !derivative.is_finite() {
                        return Err(format!("behavioral source '{}' SDT {index} has a non-finite input or noise derivative", self.name));
                    }
                    output.push(derivative);
                }
                Ok(())
            }

            pub(super) fn integral_partial(
                &self,
                time: Value,
                target: DerivativeTarget<'_>,
            ) -> Option<Value> {
                self.integral_equations.as_ref()?.transient_partial(
                    &self.node_values,
                    &self.branch_values,
                    &self.vm.accepted_sdt_history(self.program.sdt_count),
                    BehavioralEnvironment {
                        time,
                        frequency: self.frequency,
                        temperature: self.temperature,
                        gmin: self.gmin,
                        expression_dialect: self.expression_dialect,
                        logarithm_domain: LogarithmDomain::Ieee,
                    },
                    target,
                )
            }
        }
    };
}
integral_source!(BehavioralVoltageSource);
integral_source!(BehavioralCurrentSource);

impl BehavioralSources {
    /// Instantaneous perturbations of dz/dt with every integral held fixed.
    /// State-to-state propagation belongs to the period map, not this injection.
    pub(crate) fn integral_rate_directions(
        &self,
        solution: &[Value],
        direction: &[Value],
        integrals: &[Value],
        time: Value,
    ) -> Result<Vec<Value>, String> {
        if integrals.len() != self.integral_count() || solution.len() != direction.len() {
            return Err("behavioral integral noise direction has inconsistent dimensions".into());
        }
        let mut output = Vec::with_capacity(integrals.len());
        for source in &self.voltage_sources {
            source.append_integral_rate_directions(
                solution,
                direction,
                &integrals[output.len()..],
                time,
                &mut output,
            )?;
        }
        for source in &self.current_sources {
            source.append_integral_rate_directions(
                solution,
                direction,
                &integrals[output.len()..],
                time,
                &mut output,
            )?;
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integral_tangents_preserve_tiny_steps_nested_states_and_branch_controls() {
        let mut source = BehavioralVoltageSource::new(
            "b1".into(),
            2,
            0,
            0,
            "sdt(v(in)*v(in)) + 3*sdt(sdt(i(vsense))) + 5*sdt(i(vsense))",
        )
        .unwrap();
        source
            .bind_references(|_| Some(1), |_| BehavioralBranchResolution::Branch(1))
            .unwrap();
        let history = vec![
            AcceptedSdtState {
                time: 1.0,
                input: 1.0,
                integral: 1e10
            };
            4
        ];
        source.vm.restore_sdt_history(&history);
        let time = 1.0 + 1e-12;
        let dt = time - 1.0;
        source.linearize_expression(&[2.0, 3.0], time).unwrap();
        assert!((source.node_partials[0] / (2.0 * dt) - 1.0).abs() < 1e-14);
        let expected = 3.0 * (0.5 * dt).powi(2) + 5.0 * 0.5 * dt;
        assert!((source.branch_partials[0] / expected - 1.0).abs() < 1e-14);
        assert_eq!(source.vm.accepted_sdt_history(4), history);

        // At the history origin, the input may change but the stored integral
        // cannot. Both source kinds must therefore have an exact zero tangent.
        source.linearize_expression(&[2.0, 3.0], 1.0).unwrap();
        assert_eq!(source.node_partials, [0.0]);
        assert_eq!(source.branch_partials, [0.0]);
        let mut current =
            BehavioralCurrentSource::new("b2".into(), 2, 0, "sdt(v(in)*v(in))").unwrap();
        current
            .bind_references(|_| Some(1), |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        current.vm.restore_sdt_history(&history[..1]);
        current.linearize_expression(&[2.0], time).unwrap();
        assert!((current.node_partials[0] / (2.0 * dt) - 1.0).abs() < 1e-14);

        let sources = BehavioralSources {
            voltage_sources: vec![source],
            current_sources: vec![current],
        };
        let rates = sources
            .integral_rate_directions(&[2.0, 3.0], &[0.5, 0.25], &[1e10; 5], 1.0)
            .unwrap();
        // An outer SDT sees its inner coordinate held fixed; only its inner
        // equation receives the direct branch-current impulse.
        assert_eq!(rates, [2.0, 0.25, 0.0, 0.25, 2.0]);
    }
}
