//! Frequency coefficients of an already differentiated analog CFG.
//!
//! Primal values are held at the operating point. The derivative graph is a
//! polynomial in the two integration multipliers, D and I. Keeping its real
//! coefficients in the original blocks preserves branches and loop-carried
//! tangents, and lets the ordinary optimizer share their arithmetic with the
//! transient Jacobian. A frequency sweep only substitutes D = jω, I = 1/jω;
//! it neither repeats the primal body nor touches operator histories.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::ValueId;
use super::cfg::{
    CfgBinaryOp, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp, CfgValue, CfgValueKind,
    CfgValueType,
};
use crate::metrics::{PipelineCancelled, PipelineControl, PipelinePhase};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DynamicPower {
    pub ddt: u32,
    pub idt: u32,
}

impl DynamicPower {
    fn product(self, other: Self) -> Self {
        Self {
            ddt: self.ddt + other.ddt,
            idt: self.idt + other.idt,
        }
    }
}

#[derive(Debug)]
pub(crate) enum FrequencyError {
    Cancelled(PipelineCancelled),
    Unsupported(String),
}

type Polynomial = BTreeMap<DynamicPower, ValueId>;
type Powers = BTreeSet<DynamicPower>;

/// Append the dynamic coefficients of `roots`, retaining all existing ids.
/// `primal_count` is the size before AD's scalar `ddx` preparation: source
/// readbacks are frozen coefficients, but the symbolic operations introduced
/// to differentiate those readbacks still carry their integration multipliers.
pub(crate) fn expand(
    function: &mut CfgFunction,
    primal_count: usize,
    roots: &[ValueId],
    control: &dyn PipelineControl,
) -> Result<Vec<Polynomial>, FrequencyError> {
    let source = function.clone();
    let count = source.values.len();
    let mut inputs = vec![Vec::new(); count];
    for value in &source.values {
        if usize::from(value.id) >= primal_count && value.value_type != CfgValueType::Boolean {
            inputs[usize::from(value.id)] = value.kind.operands();
        }
    }
    for block in &source.blocks {
        for (target, args) in edges(&block.terminator) {
            for (&param, &arg) in source.block(target).params.iter().zip(args) {
                if usize::from(param) >= primal_count
                    && source.value(param).value_type != CfgValueType::Boolean
                {
                    inputs[usize::from(param)].push(arg);
                }
            }
        }
    }
    let mut needed = vec![false; count];
    let mut pending = roots.to_vec();
    while let Some(value) = pending.pop() {
        if !std::mem::replace(&mut needed[usize::from(value)], true) {
            pending.extend(&inputs[usize::from(value)]);
        }
    }
    let mut users = vec![Vec::new(); count];
    let mut queue = VecDeque::new();
    let mut queued = needed.clone();
    let mut powers = vec![Powers::new(); count];
    for (index, operands) in inputs
        .iter()
        .enumerate()
        .filter(|(index, _)| needed[*index])
    {
        queue.push_back(index);
        for operand in operands {
            users[usize::from(*operand)].push(index);
        }
    }
    let mut iterations = 0usize;
    while let Some(index) = queue.pop_front() {
        queued[index] = false;
        iterations += 1;
        if iterations.is_multiple_of(1024) && control.is_cancelled() {
            return Err(FrequencyError::Cancelled(PipelineCancelled {
                phase: PipelinePhase::DerivativeExtraction,
            }));
        }
        let value = &source.values[index];
        let next = if index < primal_count || value.value_type == CfgValueType::Boolean {
            Powers::from([DynamicPower::default()])
        } else {
            value_powers(&value.kind, &inputs[index], &powers)?
        };
        if next
            .iter()
            .any(|power| u64::from(power.ddt) + u64::from(power.idt) > count as u64)
        {
            return Err(FrequencyError::Unsupported("an integration multiplier recurs through a loop; its frequency response has no finite coefficient expansion".into()));
        }
        let old_len = powers[index].len();
        powers[index].extend(next);
        if old_len != powers[index].len() {
            for &user in &users[index] {
                if !std::mem::replace(&mut queued[user], true) {
                    queue.push_back(user);
                }
            }
        }
    }

    let mut coefficients = vec![Polynomial::new(); count];
    for (index, support) in powers.iter().enumerate() {
        if support.len() == 1 && support.contains(&DynamicPower::default()) {
            coefficients[index].insert(DynamicPower::default(), ValueId::from(index));
        } else {
            for &power in support {
                let id = ValueId::from(function.values.len());
                let kind = if matches!(
                    source.values[index].kind,
                    CfgValueKind::DdtScale | CfgValueKind::IdtScale
                ) {
                    CfgValueKind::RealConstant(1.0)
                } else {
                    CfgValueKind::BlockParameter
                };
                function.values.push(CfgValue {
                    id,
                    value_type: source.values[index].value_type,
                    kind,
                });
                coefficients[index].insert(power, id);
            }
        }
    }
    let mut builder = Expansion {
        function,
        coefficients: &coefficients,
        constants: BTreeMap::new(),
        constants_emitted: Vec::new(),
    };
    for block in &source.blocks {
        if control.is_cancelled() {
            return Err(FrequencyError::Cancelled(PipelineCancelled {
                phase: PipelinePhase::DerivativeExtraction,
            }));
        }
        let mut params = block.params.clone();
        for param in &block.params {
            if dynamic(&powers[usize::from(*param)]) {
                params.extend(coefficients[usize::from(*param)].values());
            }
        }
        let mut instructions = Vec::new();
        for instruction in &block.instructions {
            instructions.push(instruction.clone());
            let value = source.value(instruction.result);
            if !dynamic(&powers[usize::from(value.id)]) {
                continue;
            }
            for (&power, &target) in &coefficients[usize::from(value.id)] {
                builder.emit_coefficient(value, power, target, &mut instructions)?;
            }
        }
        let mut terminator = block.terminator.clone();
        let mut extend_edge = |target, args: &mut Vec<ValueId>| {
            let original = args.clone();
            for (&param, arg) in source.block(target).params.iter().zip(original) {
                if dynamic(&powers[usize::from(param)]) {
                    for &power in coefficients[usize::from(param)].keys() {
                        args.push(builder.coefficient(arg, power, source.value(param).value_type));
                    }
                }
            }
        };
        match &mut terminator {
            CfgTerminator::Jump { target, args } => extend_edge(*target, args),
            CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } => {
                extend_edge(*then_target, then_args);
                extend_edge(*else_target, else_args);
            }
            _ => {}
        }
        let target = &mut builder.function.blocks[usize::from(block.id)];
        target.params = params;
        target.instructions = instructions;
        target.terminator = terminator;
    }
    builder.function.blocks[usize::from(source.entry)]
        .instructions
        .splice(0..0, builder.constants_emitted);
    builder.function.validate().map_err(|error| {
        FrequencyError::Unsupported(format!("frequency coefficient CFG: {error}"))
    })?;
    Ok(roots
        .iter()
        .map(|root| {
            coefficients[usize::from(*root)]
                .iter()
                .filter(|(power, _)| **power != DynamicPower::default())
                .map(|(&power, &value)| (power, value))
                .collect()
        })
        .collect())
}

fn edges(terminator: &CfgTerminator) -> Vec<(super::BlockId, &[ValueId])> {
    match terminator {
        CfgTerminator::Jump { target, args } => vec![(*target, args)],
        CfgTerminator::Branch {
            then_target,
            then_args,
            else_target,
            else_args,
            ..
        } => vec![(*then_target, then_args), (*else_target, else_args)],
        _ => Vec::new(),
    }
}

fn dynamic(powers: &Powers) -> bool {
    powers.iter().any(|power| *power != DynamicPower::default())
}

fn value_powers(
    kind: &CfgValueKind,
    inputs: &[ValueId],
    powers: &[Powers],
) -> Result<Powers, FrequencyError> {
    let at = |value: &ValueId| &powers[usize::from(*value)];
    Ok(match kind {
        CfgValueKind::DdtScale => Powers::from([DynamicPower { ddt: 1, idt: 0 }]),
        CfgValueKind::IdtScale => Powers::from([DynamicPower { ddt: 0, idt: 1 }]),
        CfgValueKind::BlockParameter => inputs
            .iter()
            .flat_map(|input| at(input).iter().copied())
            .collect(),
        CfgValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            independent_powers(kind, &[*condition], powers)?;
            at(then_value).union(at(else_value)).copied().collect()
        }
        CfgValueKind::SumProductsDiv { terms, divisor }
        | CfgValueKind::LaneSumProductsDiv { terms, divisor } => {
            independent_powers(kind, &[*divisor], powers)?;
            if at(divisor).is_empty() {
                Powers::new()
            } else {
                terms
                    .iter()
                    .flat_map(|(a, b)| {
                        at(a)
                            .iter()
                            .flat_map(|a| at(b).iter().map(move |b| a.product(*b)))
                    })
                    .collect()
            }
        }
        CfgValueKind::Binary { op, left, right }
        | CfgValueKind::LaneBinary { op, left, right }
        | CfgValueKind::LaneScalar {
            op,
            input: left,
            scalar: right,
        } => match op {
            CfgBinaryOp::Add | CfgBinaryOp::Sub => at(left).union(at(right)).copied().collect(),
            CfgBinaryOp::Mul => at(left)
                .iter()
                .flat_map(|left| at(right).iter().map(|right| left.product(*right)))
                .collect(),
            CfgBinaryOp::Div if !dynamic(at(right)) => {
                if at(right).is_empty() {
                    Powers::new()
                } else {
                    at(left).clone()
                }
            }
            _ => independent_powers(kind, inputs, powers)?,
        },
        CfgValueKind::Unary {
            op: CfgUnaryOp::Neg,
            input,
        }
        | CfgValueKind::LaneWiden { input }
        | CfgValueKind::LaneExtract { input, .. } => at(input).clone(),
        _ => independent_powers(kind, inputs, powers)?,
    })
}

fn independent_powers(
    kind: &CfgValueKind,
    inputs: &[ValueId],
    powers: &[Powers],
) -> Result<Powers, FrequencyError> {
    if inputs
        .iter()
        .any(|input| dynamic(&powers[usize::from(*input)]))
    {
        return Err(FrequencyError::Unsupported(format!(
            "non-polynomial integration multiplier in {kind:?}"
        )));
    }
    Ok(
        if inputs
            .iter()
            .any(|input| powers[usize::from(*input)].is_empty())
        {
            Powers::new()
        } else {
            Powers::from([DynamicPower::default()])
        },
    )
}

struct Expansion<'a> {
    function: &'a mut CfgFunction,
    coefficients: &'a [Polynomial],
    constants: BTreeMap<Option<super::ShapeId>, ValueId>,
    constants_emitted: Vec<CfgInstruction>,
}

impl Expansion<'_> {
    fn coefficient(&mut self, value: ValueId, power: DynamicPower, ty: CfgValueType) -> ValueId {
        if let Some(value) = self.coefficients[usize::from(value)].get(&power) {
            return *value;
        }
        if let Some(value) = self.constants.get(&ty.shape()) {
            return *value;
        }
        let kind = if ty.shape().is_some() {
            CfgValueKind::LaneSplat(0.0)
        } else {
            CfgValueKind::RealConstant(0.0)
        };
        let id = ValueId::from(self.function.values.len());
        self.function.values.push(CfgValue {
            id,
            value_type: ty,
            kind,
        });
        self.constants.insert(ty.shape(), id);
        self.constants_emitted.push(CfgInstruction { result: id });
        id
    }

    fn push(
        &mut self,
        ty: CfgValueType,
        kind: CfgValueKind,
        instructions: &mut Vec<CfgInstruction>,
    ) -> ValueId {
        let id = ValueId::from(self.function.values.len());
        self.function.values.push(CfgValue {
            id,
            value_type: ty,
            kind,
        });
        instructions.push(CfgInstruction { result: id });
        id
    }

    fn emit_coefficient(
        &mut self,
        value: &CfgValue,
        power: DynamicPower,
        target: ValueId,
        instructions: &mut Vec<CfgInstruction>,
    ) -> Result<(), FrequencyError> {
        let ty = value.value_type;
        let kind = match &value.kind {
            CfgValueKind::DdtScale | CfgValueKind::IdtScale => CfgValueKind::RealConstant(1.0),
            CfgValueKind::Select {
                condition,
                then_value,
                else_value,
            } => CfgValueKind::Select {
                condition: self.coefficient(
                    *condition,
                    DynamicPower::default(),
                    CfgValueType::Boolean,
                ),
                then_value: self.coefficient(*then_value, power, ty),
                else_value: self.coefficient(*else_value, power, ty),
            },
            CfgValueKind::SumProductsDiv { terms, divisor }
            | CfgValueKind::LaneSumProductsDiv { terms, divisor } => {
                let mut products = Vec::new();
                for &(left, right) in terms {
                    for (&a_power, &a) in &self.coefficients[usize::from(left)] {
                        for (&b_power, &b) in &self.coefficients[usize::from(right)] {
                            if a_power.product(b_power) == power {
                                products.push((a, b));
                            }
                        }
                    }
                }
                let divisor =
                    self.coefficient(*divisor, DynamicPower::default(), CfgValueType::Real);
                if ty.shape().is_some() {
                    CfgValueKind::LaneSumProductsDiv {
                        terms: products,
                        divisor,
                    }
                } else {
                    CfgValueKind::SumProductsDiv {
                        terms: products,
                        divisor,
                    }
                }
            }
            CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left,
                right,
            }
            | CfgValueKind::LaneBinary {
                op: CfgBinaryOp::Mul,
                left,
                right,
            }
            | CfgValueKind::LaneScalar {
                op: CfgBinaryOp::Mul,
                input: left,
                scalar: right,
            } => {
                let mut sum = None;
                for (&a_power, &a) in &self.coefficients[usize::from(*left)] {
                    for (&b_power, &b) in &self.coefficients[usize::from(*right)] {
                        if a_power.product(b_power) != power {
                            continue;
                        }
                        let mut term = value.kind.clone();
                        // A repeated operand can have different powers on its
                        // two occurrences, so map positions, not operand ids.
                        let mut operands = [a, b].into_iter();
                        term.map_operands(|_| operands.next().expect("binary operand"));
                        let term = self.push(ty, term, instructions);
                        sum = Some(match sum {
                            None => term,
                            Some(left) => self.push(
                                ty,
                                if ty.shape().is_some() {
                                    CfgValueKind::LaneBinary {
                                        op: CfgBinaryOp::Add,
                                        left,
                                        right: term,
                                    }
                                } else {
                                    CfgValueKind::Binary {
                                        op: CfgBinaryOp::Add,
                                        left,
                                        right: term,
                                    }
                                },
                                instructions,
                            ),
                        });
                    }
                }
                let input = sum.expect("support was computed from products");
                self.function.value(input).kind.clone()
            }
            _ => {
                let mut kind = value.kind.clone();
                let division = matches!(
                    kind,
                    CfgValueKind::Binary {
                        op: CfgBinaryOp::Div,
                        ..
                    } | CfgValueKind::LaneScalar {
                        op: CfgBinaryOp::Div,
                        ..
                    }
                );
                let mut position = 0;
                kind.map_operands(|operand| {
                    let power = if division && position == 1 {
                        DynamicPower::default()
                    } else {
                        power
                    };
                    position += 1;
                    self.coefficient(operand, power, self.function.value(operand).value_type)
                });
                kind
            }
        };
        self.function.values[usize::from(target)].kind = kind;
        instructions.push(CfgInstruction { result: target });
        Ok(())
    }
}
