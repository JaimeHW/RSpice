use std::collections::{HashMap, HashSet};

use thiserror::Error;

use super::opt::{
    THERMAL_VOLTAGE_PER_K, limexp_derivative, limexp_value, limited_exp_derivative,
    limited_exp_value, real_truth_value,
};
use super::{
    DerivativeLane, EquationId, IrDiagnostic, OptBinaryOp, OptModel, OptUnaryOp, OptValueKind,
    ValueId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OptEvalInputs {
    pub parameters: Vec<f64>,
    pub node_potentials: Vec<f64>,
    pub branch_flows: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptEvalValue {
    Real(f64),
    Boolean(bool),
}

impl OptEvalValue {
    pub fn real(self) -> Option<f64> {
        match self {
            Self::Real(value) => Some(value),
            Self::Boolean(_) => None,
        }
    }

    pub fn boolean(self) -> Option<bool> {
        match self {
            Self::Real(_) => None,
            Self::Boolean(value) => Some(value),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Real(_) => "real",
            Self::Boolean(_) => "boolean",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptEvaluatedDerivative {
    pub lane: DerivativeLane,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptEvalSnapshot {
    values: Vec<OptEvalValue>,
    derivatives: Vec<Vec<OptEvaluatedDerivative>>,
}

impl OptEvalSnapshot {
    pub fn value(&self, value: ValueId) -> Option<OptEvalValue> {
        self.values.get(usize::from(value)).copied()
    }

    pub fn real(&self, value: ValueId) -> Option<f64> {
        self.value(value).and_then(OptEvalValue::real)
    }

    pub fn boolean(&self, value: ValueId) -> Option<bool> {
        self.value(value).and_then(OptEvalValue::boolean)
    }

    pub fn derivative(&self, value: ValueId, lane: DerivativeLane) -> Option<f64> {
        self.derivatives
            .get(usize::from(value))?
            .iter()
            .find(|derivative| derivative.lane == lane)
            .map(|derivative| derivative.value)
    }
}

#[derive(Debug, Error)]
pub enum OptEvalError {
    #[error("OptIR validation failed")]
    Validation(#[source] OptValidationError),
    #[error("OptIR input '{kind}' index {index} is outside input length {len}")]
    MissingInput {
        kind: &'static str,
        index: u32,
        len: usize,
    },
    #[error("OptIR value {value} expected {expected}, found {found}")]
    TypeMismatch {
        value: ValueId,
        expected: &'static str,
        found: &'static str,
    },
    #[error("OptIR equation value {equation} is not evaluable by the scalar reference evaluator")]
    UnsupportedEquationValue { equation: EquationId },
    #[error("OptIR runtime loop {loop_id} is missing")]
    MissingRuntimeLoop { loop_id: u32 },
    #[error("OptIR runtime loop {loop_id} exceeded iteration limit")]
    RuntimeLoopLimit { loop_id: u32 },
    #[error("OptIR value {value} has a cyclic scalar evaluation dependency")]
    CyclicValue { value: ValueId },
}

#[derive(Debug, Error)]
#[error("{diagnostics:?}")]
pub struct OptValidationError {
    pub diagnostics: Vec<IrDiagnostic>,
}

impl OptModel {
    pub fn evaluate(&self, inputs: &OptEvalInputs) -> Result<OptEvalSnapshot, OptEvalError> {
        evaluate_opt_model(self, inputs)
    }
}

pub fn evaluate_opt_model(
    model: &OptModel,
    inputs: &OptEvalInputs,
) -> Result<OptEvalSnapshot, OptEvalError> {
    model
        .validate()
        .map_err(|diagnostics| OptEvalError::Validation(OptValidationError { diagnostics }))?;

    let mut evaluator = OptEvaluator::new(model, inputs);
    for value in &model.values {
        evaluator.evaluate_value_id(value.id)?;
    }
    let values = evaluator.finish();

    let mut derivatives = vec![Vec::new(); values.len()];
    for value in &model.values {
        let slot = &mut derivatives[usize::from(value.id)];
        slot.reserve(value.derivatives.len());

        for derivative in &value.derivatives {
            let derivative_value = real_value(&values, derivative.value)?;
            slot.push(OptEvaluatedDerivative {
                lane: derivative.lane,
                value: derivative_value,
            });
        }
    }

    Ok(OptEvalSnapshot {
        values,
        derivatives,
    })
}

struct OptEvaluator<'a> {
    model: &'a OptModel,
    inputs: &'a OptEvalInputs,
    values: Vec<Option<OptEvalValue>>,
    visiting: Vec<bool>,
    loop_indices: HashMap<u32, f64>,
    runtime_loop_values: HashMap<(u32, u32), OptEvalValue>,
    runtime_loop_derivatives: HashMap<(u32, u32, DerivativeLane), f64>,
}

impl<'a> OptEvaluator<'a> {
    fn new(model: &'a OptModel, inputs: &'a OptEvalInputs) -> Self {
        Self {
            model,
            inputs,
            values: vec![None; model.values.len()],
            visiting: vec![false; model.values.len()],
            loop_indices: HashMap::new(),
            runtime_loop_values: HashMap::new(),
            runtime_loop_derivatives: HashMap::new(),
        }
    }

    fn finish(self) -> Vec<OptEvalValue> {
        self.values
            .into_iter()
            .map(|value| value.expect("all OptIR values were evaluated"))
            .collect()
    }

    fn evaluate_value_id(&mut self, value: ValueId) -> Result<OptEvalValue, OptEvalError> {
        let index = usize::from(value);
        if let Some(evaluated) = self.values[index] {
            return Ok(evaluated);
        }
        if self.visiting[index] {
            return Err(OptEvalError::CyclicValue { value });
        }

        self.visiting[index] = true;
        let kind = self.model.values[index].kind.clone();
        let evaluated = self.evaluate_value_kind(value, &kind);
        self.visiting[index] = false;
        let evaluated = evaluated?;
        self.values[index] = Some(evaluated);
        Ok(evaluated)
    }

    fn evaluate_value_kind(
        &mut self,
        value: ValueId,
        kind: &OptValueKind,
    ) -> Result<OptEvalValue, OptEvalError> {
        match kind {
            OptValueKind::RealConstant(constant) => Ok(OptEvalValue::Real(*constant)),
            OptValueKind::BooleanConstant(constant) => Ok(OptEvalValue::Boolean(*constant)),
            OptValueKind::Parameter { parameter } => Ok(OptEvalValue::Real(input_at(
                "parameter",
                &self.inputs.parameters,
                parameter.index(),
            )?)),
            OptValueKind::ParamGiven { .. } => Ok(OptEvalValue::Real(0.0)),
            OptValueKind::Temperature => Ok(OptEvalValue::Real(300.15)),
            OptValueKind::ThermalVoltage => Ok(OptEvalValue::Real(300.15 * THERMAL_VOLTAGE_PER_K)),
            OptValueKind::Multiplicity => Ok(OptEvalValue::Real(1.0)),
            OptValueKind::Time => Ok(OptEvalValue::Real(0.0)),
            OptValueKind::Analysis { .. } => Ok(OptEvalValue::Real(0.0)),
            OptValueKind::Ddx {
                value,
                pos_node,
                neg_node,
            } => Ok(OptEvalValue::Real(
                self.evaluate_ddx_projection(*value, *pos_node, *neg_node)?,
            )),
            OptValueKind::Ddt { .. } | OptValueKind::DdtScale => Ok(OptEvalValue::Real(0.0)),
            OptValueKind::NodePotential { node } => Ok(OptEvalValue::Real(input_at(
                "node_potential",
                &self.inputs.node_potentials,
                node.index(),
            )?)),
            OptValueKind::BranchFlow { branch } => Ok(OptEvalValue::Real(input_at(
                "branch_flow",
                &self.inputs.branch_flows,
                branch.index(),
            )?)),
            OptValueKind::BranchUnknownFlow { branch_unknown } => Ok(OptEvalValue::Real(input_at(
                "branch_flow",
                &self.inputs.branch_flows,
                branch_unknown.index(),
            )?)),
            OptValueKind::LoopIndex { loop_id } => Ok(OptEvalValue::Real(
                self.loop_indices.get(loop_id).copied().unwrap_or(0.0),
            )),
            OptValueKind::CountedSum {
                loop_id,
                count,
                initial,
                term,
            } => self.evaluate_counted_sum(*loop_id, *count, *initial, *term),
            OptValueKind::RuntimeLoopVariable { loop_id, slot } => self
                .runtime_loop_values
                .get(&(*loop_id, *slot))
                .copied()
                .ok_or(OptEvalError::MissingRuntimeLoop { loop_id: *loop_id }),
            OptValueKind::RuntimeLoopVariableDerivative {
                loop_id,
                slot,
                lane,
            } => Ok(OptEvalValue::Real(
                self.runtime_loop_derivatives
                    .get(&(*loop_id, *slot, *lane))
                    .copied()
                    .unwrap_or(0.0),
            )),
            OptValueKind::RuntimeLoopResult { loop_id, .. }
            | OptValueKind::RuntimeLoopResultDerivative { loop_id, .. } => {
                self.evaluate_runtime_loop(*loop_id)?;
                self.values[usize::from(value)]
                    .ok_or(OptEvalError::MissingRuntimeLoop { loop_id: *loop_id })
            }
            OptValueKind::Unary { op, input } => self.evaluate_unary(value, *op, *input),
            OptValueKind::Binary { op, left, right } => {
                self.evaluate_binary(value, *op, *left, *right)
            }
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => {
                let condition = self.boolean_value(*condition)?;
                if condition {
                    self.value_at(*then_value)
                } else {
                    self.value_at(*else_value)
                }
            }
            OptValueKind::EquationValue { equation } => {
                Err(OptEvalError::UnsupportedEquationValue {
                    equation: *equation,
                })
            }
        }
    }

    fn evaluate_counted_sum(
        &mut self,
        loop_id: u32,
        count: ValueId,
        initial: ValueId,
        term: ValueId,
    ) -> Result<OptEvalValue, OptEvalError> {
        let count = self.real_value(count)?;
        let mut accumulator = self.real_value(initial)?;
        let previous = self.loop_indices.get(&loop_id).copied();
        let mut index = 0_i64;
        while (index as f64) < count {
            self.loop_indices.insert(loop_id, index as f64);
            self.clear_loop_dependent_values();
            accumulator += self.real_value(term)?;
            index += 1;
        }
        match previous {
            Some(previous) => {
                self.loop_indices.insert(loop_id, previous);
            }
            None => {
                self.loop_indices.remove(&loop_id);
            }
        }
        self.clear_loop_dependent_values();
        Ok(OptEvalValue::Real(accumulator))
    }

    fn evaluate_runtime_loop(&mut self, loop_id: u32) -> Result<(), OptEvalError> {
        let runtime_loop = self
            .model
            .runtime_loops
            .iter()
            .find(|runtime_loop| runtime_loop.loop_id == loop_id)
            .cloned()
            .ok_or(OptEvalError::MissingRuntimeLoop { loop_id })?;
        if runtime_loop
            .variables
            .iter()
            .all(|variable| self.values[usize::from(variable.result)].is_some())
        {
            return Ok(());
        }

        let previous_values = self.runtime_loop_values.clone();
        let previous_derivatives = self.runtime_loop_derivatives.clone();
        let derivative_lanes = self.runtime_loop_derivative_lanes(loop_id);

        let mut values = Vec::with_capacity(runtime_loop.variables.len());
        let mut derivatives = Vec::with_capacity(runtime_loop.variables.len());
        for variable in &runtime_loop.variables {
            values.push(self.value_at(variable.initial)?);
            let mut derivative_values = HashMap::new();
            for lane in &derivative_lanes {
                derivative_values
                    .insert(*lane, self.derivative_real_value(variable.initial, *lane)?);
            }
            derivatives.push(derivative_values);
        }
        self.install_runtime_loop_state(loop_id, &values, &derivatives);

        let mut guard = 0usize;
        while self.truth_value(runtime_loop.condition)? {
            guard += 1;
            if guard > 1_000_000 {
                self.runtime_loop_values = previous_values;
                self.runtime_loop_derivatives = previous_derivatives;
                return Err(OptEvalError::RuntimeLoopLimit { loop_id });
            }

            let mut next_values = Vec::with_capacity(runtime_loop.assignments.len());
            let mut next_derivatives = Vec::with_capacity(runtime_loop.assignments.len());
            for assignment in &runtime_loop.assignments {
                self.clear_loop_dependent_values();
                let value = self.value_at(assignment.value)?;
                let mut derivative_values = HashMap::new();
                for lane in &derivative_lanes {
                    derivative_values
                        .insert(*lane, self.derivative_real_value(assignment.value, *lane)?);
                }
                next_values.push((assignment.slot, value));
                next_derivatives.push((assignment.slot, derivative_values));
            }

            for (slot, value) in next_values {
                if let Some(target) = values.get_mut(slot as usize) {
                    *target = value;
                }
            }
            for (slot, derivative_values) in next_derivatives {
                if let Some(target) = derivatives.get_mut(slot as usize) {
                    *target = derivative_values;
                }
            }
            self.install_runtime_loop_state(loop_id, &values, &derivatives);
            self.clear_loop_dependent_values();
        }

        for (slot, variable) in runtime_loop.variables.iter().enumerate() {
            self.values[usize::from(variable.result)] = values.get(slot).copied();
        }
        for value in &self.model.values {
            if let OptValueKind::RuntimeLoopResultDerivative {
                loop_id: derivative_loop,
                slot,
                lane,
            } = value.kind
                && derivative_loop == loop_id
            {
                let derivative = derivatives
                    .get(slot as usize)
                    .and_then(|values| values.get(&lane))
                    .copied()
                    .unwrap_or(0.0);
                self.values[usize::from(value.id)] = Some(OptEvalValue::Real(derivative));
            }
        }

        self.runtime_loop_values = previous_values;
        self.runtime_loop_derivatives = previous_derivatives;
        self.clear_loop_dependent_values();
        Ok(())
    }

    fn runtime_loop_derivative_lanes(&self, loop_id: u32) -> Vec<DerivativeLane> {
        let mut lanes = HashSet::new();
        for value in &self.model.values {
            if let OptValueKind::RuntimeLoopResultDerivative {
                loop_id: derivative_loop,
                lane,
                ..
            } = value.kind
                && derivative_loop == loop_id
            {
                lanes.insert(lane);
            }
        }
        lanes.into_iter().collect()
    }

    fn install_runtime_loop_state(
        &mut self,
        loop_id: u32,
        values: &[OptEvalValue],
        derivatives: &[HashMap<DerivativeLane, f64>],
    ) {
        for (slot, value) in values.iter().copied().enumerate() {
            let slot = u32::try_from(slot).expect("runtime loop slot exceeds u32::MAX");
            self.runtime_loop_values.insert((loop_id, slot), value);
        }
        for (slot, derivative_values) in derivatives.iter().enumerate() {
            let slot = u32::try_from(slot).expect("runtime loop slot exceeds u32::MAX");
            for (lane, value) in derivative_values {
                self.runtime_loop_derivatives
                    .insert((loop_id, slot, *lane), *value);
            }
        }
    }

    fn clear_loop_dependent_values(&mut self) {
        let mut memo = vec![None; self.model.values.len()];
        for value in &self.model.values {
            if self.value_depends_on_loop_index(value.id, &mut memo) {
                self.values[usize::from(value.id)] = None;
            }
        }
    }

    fn value_depends_on_loop_index(&self, value: ValueId, memo: &mut [Option<bool>]) -> bool {
        let index = usize::from(value);
        if let Some(depends) = memo[index] {
            return depends;
        }
        let depends = match self.model.values[index].kind {
            OptValueKind::LoopIndex { .. }
            | OptValueKind::RuntimeLoopVariable { .. }
            | OptValueKind::RuntimeLoopVariableDerivative { .. } => true,
            OptValueKind::Unary { input, .. } => self.value_depends_on_loop_index(input, memo),
            OptValueKind::Binary { left, right, .. } => {
                self.value_depends_on_loop_index(left, memo)
                    || self.value_depends_on_loop_index(right, memo)
            }
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => {
                self.value_depends_on_loop_index(condition, memo)
                    || self.value_depends_on_loop_index(then_value, memo)
                    || self.value_depends_on_loop_index(else_value, memo)
            }
            OptValueKind::CountedSum {
                count,
                initial,
                term,
                ..
            } => {
                self.value_depends_on_loop_index(count, memo)
                    || self.value_depends_on_loop_index(initial, memo)
                    || self.value_depends_on_loop_index(term, memo)
            }
            OptValueKind::Ddx { value, .. } | OptValueKind::Ddt { input: value, .. } => {
                self.value_depends_on_loop_index(value, memo)
            }
            OptValueKind::RealConstant(_)
            | OptValueKind::BooleanConstant(_)
            | OptValueKind::Parameter { .. }
            | OptValueKind::ParamGiven { .. }
            | OptValueKind::Temperature
            | OptValueKind::ThermalVoltage
            | OptValueKind::Multiplicity
            | OptValueKind::Time
            | OptValueKind::Analysis { .. }
            | OptValueKind::DdtScale
            | OptValueKind::NodePotential { .. }
            | OptValueKind::BranchFlow { .. }
            | OptValueKind::BranchUnknownFlow { .. }
            | OptValueKind::RuntimeLoopResult { .. }
            | OptValueKind::RuntimeLoopResultDerivative { .. }
            | OptValueKind::EquationValue { .. } => false,
        };
        memo[index] = Some(depends);
        depends
    }

    fn evaluate_unary(
        &mut self,
        owner: ValueId,
        op: OptUnaryOp,
        input: ValueId,
    ) -> Result<OptEvalValue, OptEvalError> {
        let result = (|| match op {
            OptUnaryOp::Pos => Ok(OptEvalValue::Real(self.real_value(input)?)),
            OptUnaryOp::Neg => Ok(OptEvalValue::Real(-self.real_value(input)?)),
            OptUnaryOp::Not => Ok(OptEvalValue::Boolean(!self.truth_value(input)?)),
            OptUnaryOp::Exp => Ok(OptEvalValue::Real(self.real_value(input)?.exp())),
            OptUnaryOp::LimExp => Ok(OptEvalValue::Real(limexp_value(self.real_value(input)?))),
            OptUnaryOp::LimExpDerivative => Ok(OptEvalValue::Real(limexp_derivative(
                self.real_value(input)?,
            ))),
            OptUnaryOp::LimitedExp => Ok(OptEvalValue::Real(limited_exp_value(
                self.real_value(input)?,
            ))),
            OptUnaryOp::LimitedExpDerivative => Ok(OptEvalValue::Real(limited_exp_derivative(
                self.real_value(input)?,
            ))),
            OptUnaryOp::Ln => Ok(OptEvalValue::Real(self.real_value(input)?.ln())),
            OptUnaryOp::Sqrt => Ok(OptEvalValue::Real(self.real_value(input)?.sqrt())),
            OptUnaryOp::Abs => Ok(OptEvalValue::Real(self.real_value(input)?.abs())),
            OptUnaryOp::Sin => Ok(OptEvalValue::Real(self.real_value(input)?.sin())),
            OptUnaryOp::Cos => Ok(OptEvalValue::Real(self.real_value(input)?.cos())),
            OptUnaryOp::Tan => Ok(OptEvalValue::Real(self.real_value(input)?.tan())),
            OptUnaryOp::Sinh => Ok(OptEvalValue::Real(self.real_value(input)?.sinh())),
            OptUnaryOp::Cosh => Ok(OptEvalValue::Real(self.real_value(input)?.cosh())),
            OptUnaryOp::Tanh => Ok(OptEvalValue::Real(self.real_value(input)?.tanh())),
            OptUnaryOp::Atan => Ok(OptEvalValue::Real(self.real_value(input)?.atan())),
            OptUnaryOp::Asinh => Ok(OptEvalValue::Real(self.real_value(input)?.asinh())),
            OptUnaryOp::Floor => Ok(OptEvalValue::Real(self.real_value(input)?.floor())),
            OptUnaryOp::Ceil => Ok(OptEvalValue::Real(self.real_value(input)?.ceil())),
        })();

        result.map_err(|error| remap_type_mismatch(owner, error))
    }

    fn evaluate_binary(
        &mut self,
        owner: ValueId,
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    ) -> Result<OptEvalValue, OptEvalError> {
        let result = (|| match op {
            OptBinaryOp::Add => Ok(OptEvalValue::Real(
                self.real_value(left)? + self.real_value(right)?,
            )),
            OptBinaryOp::Sub => Ok(OptEvalValue::Real(
                self.real_value(left)? - self.real_value(right)?,
            )),
            OptBinaryOp::Mul => Ok(OptEvalValue::Real(
                self.real_value(left)? * self.real_value(right)?,
            )),
            OptBinaryOp::Div => Ok(OptEvalValue::Real(
                self.real_value(left)? / self.real_value(right)?,
            )),
            OptBinaryOp::Pow => Ok(OptEvalValue::Real(
                self.real_value(left)?.powf(self.real_value(right)?),
            )),
            OptBinaryOp::Eq => Ok(OptEvalValue::Boolean(
                self.value_at(left)? == self.value_at(right)?,
            )),
            OptBinaryOp::Ne => Ok(OptEvalValue::Boolean(
                self.value_at(left)? != self.value_at(right)?,
            )),
            OptBinaryOp::Lt => Ok(OptEvalValue::Boolean(
                self.real_value(left)? < self.real_value(right)?,
            )),
            OptBinaryOp::Le => Ok(OptEvalValue::Boolean(
                self.real_value(left)? <= self.real_value(right)?,
            )),
            OptBinaryOp::Gt => Ok(OptEvalValue::Boolean(
                self.real_value(left)? > self.real_value(right)?,
            )),
            OptBinaryOp::Ge => Ok(OptEvalValue::Boolean(
                self.real_value(left)? >= self.real_value(right)?,
            )),
            OptBinaryOp::And => Ok(OptEvalValue::Boolean(
                self.truth_value(left)? && self.truth_value(right)?,
            )),
            OptBinaryOp::Or => Ok(OptEvalValue::Boolean(
                self.truth_value(left)? || self.truth_value(right)?,
            )),
        })();

        result.map_err(|error| remap_type_mismatch(owner, error))
    }

    fn evaluate_ddx_projection(
        &mut self,
        value: ValueId,
        pos_node: Option<crate::canonical_ir::NodeId>,
        neg_node: Option<crate::canonical_ir::NodeId>,
    ) -> Result<f64, OptEvalError> {
        let pos = match pos_node {
            Some(node) => self.derivative_real_value(value, DerivativeLane::node(node))?,
            None => 0.0,
        };
        if let Some(node) = neg_node {
            let neg = self.derivative_real_value(value, DerivativeLane::node(node))?;
            Ok(0.5 * (pos - neg))
        } else {
            Ok(pos)
        }
    }

    fn derivative_real_value(
        &mut self,
        value: ValueId,
        lane: DerivativeLane,
    ) -> Result<f64, OptEvalError> {
        let Some(derivative_value) = self.model.values[usize::from(value)]
            .derivatives
            .iter()
            .find(|derivative| derivative.lane == lane)
            .map(|derivative| derivative.value)
        else {
            return Ok(0.0);
        };
        let evaluated = self.evaluate_value_id(derivative_value)?;
        evaluated.real().ok_or(OptEvalError::TypeMismatch {
            value: derivative_value,
            expected: "real",
            found: evaluated.label(),
        })
    }

    fn value_at(&mut self, value: ValueId) -> Result<OptEvalValue, OptEvalError> {
        self.evaluate_value_id(value)
    }

    fn real_value(&mut self, value: ValueId) -> Result<f64, OptEvalError> {
        let evaluated = self.value_at(value)?;
        evaluated.real().ok_or(OptEvalError::TypeMismatch {
            value,
            expected: "real",
            found: evaluated.label(),
        })
    }

    fn boolean_value(&mut self, value: ValueId) -> Result<bool, OptEvalError> {
        let evaluated = self.value_at(value)?;
        evaluated.boolean().ok_or(OptEvalError::TypeMismatch {
            value,
            expected: "boolean",
            found: evaluated.label(),
        })
    }

    fn truth_value(&mut self, value: ValueId) -> Result<bool, OptEvalError> {
        match self.value_at(value)? {
            OptEvalValue::Real(value) => Ok(real_truth_value(value)),
            OptEvalValue::Boolean(value) => Ok(value),
        }
    }
}

fn remap_type_mismatch(owner: ValueId, error: OptEvalError) -> OptEvalError {
    match error {
        OptEvalError::TypeMismatch {
            expected, found, ..
        } => OptEvalError::TypeMismatch {
            value: owner,
            expected,
            found,
        },
        other => other,
    }
}

fn input_at(kind: &'static str, inputs: &[f64], index: u32) -> Result<f64, OptEvalError> {
    inputs
        .get(index as usize)
        .copied()
        .ok_or(OptEvalError::MissingInput {
            kind,
            index,
            len: inputs.len(),
        })
}

fn value_at(values: &[OptEvalValue], value: ValueId) -> OptEvalValue {
    values[usize::from(value)]
}

fn real_value(values: &[OptEvalValue], value: ValueId) -> Result<f64, OptEvalError> {
    let evaluated = value_at(values, value);
    evaluated.real().ok_or(OptEvalError::TypeMismatch {
        value,
        expected: "real",
        found: evaluated.label(),
    })
}
