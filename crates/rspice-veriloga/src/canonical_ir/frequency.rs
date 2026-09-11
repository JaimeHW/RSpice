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

/// Freeze noise metadata and routing gains at their DC primal values, retaining
/// input validation. Existing value IDs stay valid; no transient history is read.
pub(crate) fn freeze_noise_primal(function: &mut CfgFunction) {
    if !function.values.iter().any(|value| {
        matches!(
            value.kind,
            CfgValueKind::Ddt { .. }
                | CfgValueKind::Idt { .. }
                | CfgValueKind::IdtMod { .. }
                | CfgValueKind::IntegralDerivative { .. }
                | CfgValueKind::DdtScale
                | CfgValueKind::IdtScale
        )
    }) {
        return;
    }
    let circular_initials = function
        .values
        .iter()
        .filter_map(|value| {
            if let CfgValueKind::IdtMod { ic, .. } = value.kind {
                Some((value.id, ic))
            } else {
                None
            }
        })
        .collect::<BTreeMap<_, _>>();
    let zero = ValueId::from(function.values.len());
    function.values.push(CfgValue {
        id: zero,
        value_type: CfgValueType::Real,
        kind: CfgValueKind::RealConstant(0.0),
    });
    function.blocks[usize::from(function.entry)]
        .instructions
        .insert(0, CfgInstruction { result: zero });
    for value in &mut function.values {
        value.kind = match value.kind {
            CfgValueKind::Ddt { input, .. } => CfgValueKind::Binary {
                op: CfgBinaryOp::CheckedValue,
                left: input,
                right: zero,
            },
            CfgValueKind::Idt { input, ic, .. } => CfgValueKind::Binary {
                op: CfgBinaryOp::CheckedValue,
                left: input,
                right: ic,
            },
            CfgValueKind::IdtMod {
                input,
                ic,
                modulus,
                offset,
                ..
            } => CfgValueKind::IdtModInitial {
                input,
                ic,
                modulus,
                offset,
            },
            CfgValueKind::DdtScale | CfgValueKind::IdtScale => CfgValueKind::RealConstant(0.0),
            CfgValueKind::IntegralDerivative {
                primal,
                ic_derivative,
                wrap: None,
                ..
            } => CfgValueKind::Binary {
                op: CfgBinaryOp::CheckedValue,
                left: primal,
                right: ic_derivative,
            },
            CfgValueKind::IntegralDerivative {
                primal,
                ic_derivative,
                wrap: Some((modulus, offset, modulus_derivative)),
                ..
            } => {
                let Some(&ic) = circular_initials.get(&primal) else {
                    continue;
                };
                CfgValueKind::IdtModBranchDerivative {
                    primal,
                    ic,
                    modulus,
                    offset,
                    integral_derivative: ic_derivative,
                    modulus_derivative,
                }
            }
            _ => continue,
        };
    }
}

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

#[derive(Clone)]
pub(crate) struct FrequencyCoefficient {
    pub value: ValueId,
    /// A scalar zero/one value, independent of the numerical coefficient.
    /// Only integral terms need this guard for zero-frequency substitution.
    pub active: Option<ValueId>,
}

/// Append the frequency coefficients of `roots`, including the constant term,
/// retaining all existing ids.
/// `primal_count` is the size before AD's scalar `ddx` preparation: source
/// readbacks are frozen coefficients, but the symbolic operations introduced
/// to differentiate those readbacks still carry their integration multipliers.
pub(crate) fn expand(
    function: &mut CfgFunction,
    primal_count: usize,
    roots: &[ValueId],
    control: &dyn PipelineControl,
) -> Result<Vec<BTreeMap<DynamicPower, FrequencyCoefficient>>, FrequencyError> {
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
        let next = if matches!(
            value.kind,
            CfgValueKind::RealConstant(0.0) | CfgValueKind::LaneSplat(0.0)
        ) {
            // A literal zero has no transfer. An operating-point expression
            // that merely evaluates to zero still has its structural support.
            Powers::new()
        } else if index < primal_count || value.value_type == CfgValueType::Boolean {
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

    let mut expanded = powers
        .iter()
        .enumerate()
        .map(|(index, support)| {
            dynamic(support)
                || (index >= primal_count
                    && !support.is_empty()
                    && matches!(
                        source.values[index].kind,
                        CfgValueKind::IntegralDerivative { wrap: Some(_), .. }
                    ))
        })
        .collect::<Vec<_>>();
    // Even a constant-power circular derivative differs from its transient
    // form: AC excludes the initial-condition tangent. Carry that replacement
    // through arithmetic and block arguments that otherwise need no expansion.
    let mut pending = expanded
        .iter()
        .enumerate()
        .filter_map(|(index, &copy)| copy.then_some(index))
        .collect::<Vec<_>>();
    while let Some(index) = pending.pop() {
        for &user in &users[index] {
            if !powers[user].is_empty() && !std::mem::replace(&mut expanded[user], true) {
                pending.push(user);
            }
        }
    }
    let expand_value = |index: usize| expanded[index];
    let mut coefficients = vec![Polynomial::new(); count];
    for (index, support) in powers.iter().enumerate() {
        if support.len() == 1 && support.contains(&DynamicPower::default()) && !expand_value(index)
        {
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
            if expand_value(usize::from(*param)) {
                params.extend(coefficients[usize::from(*param)].values());
            }
        }
        let mut instructions = Vec::new();
        for instruction in &block.instructions {
            instructions.push(instruction.clone());
            let value = source.value(instruction.result);
            if !expand_value(usize::from(value.id)) {
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
                if expand_value(usize::from(param)) {
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
    let activity = if roots
        .iter()
        .any(|root| powers[usize::from(*root)].iter().any(|power| power.idt > 0))
    {
        append_activity(function, &source, primal_count, &powers, control)?
    } else {
        vec![Polynomial::new(); count]
    };
    Ok(roots
        .iter()
        .map(|root| {
            coefficients[usize::from(*root)]
                .iter()
                .map(|(&power, &value)| {
                    (
                        power,
                        FrequencyCoefficient {
                            value,
                            active: (power.idt > 0).then(|| activity[usize::from(*root)][&power]),
                        },
                    )
                })
                .collect()
        })
        .collect())
}

/// Carry structural presence through the polynomial graph. These flags are
/// scalar even for packed tangents: a widen inserts absent lanes, and ordinary
/// scalar CFG simplification can share identical activation across all lanes.
/// Numerical coefficient zero never supplies an activation flag.
fn append_activity(
    function: &mut CfgFunction,
    source: &CfgFunction,
    primal_count: usize,
    powers: &[Powers],
    control: &dyn PipelineControl,
) -> Result<Vec<Polynomial>, FrequencyError> {
    let mut masks = vec![BTreeMap::new(); source.values.len()];
    let mut constants = Vec::new();
    let mut push_constant = |constant| {
        let id = ValueId::from(function.values.len());
        function.values.push(CfgValue {
            id,
            value_type: CfgValueType::Real,
            kind: CfgValueKind::RealConstant(constant),
        });
        constants.push(CfgInstruction { result: id });
        id
    };
    let zero = push_constant(0.0);
    let one = push_constant(1.0);
    let mut tracked = vec![false; source.values.len()];
    for (index, support) in powers.iter().enumerate() {
        let value = &source.values[index];
        // Frequency-independent tangents are coefficients, even when their
        // numerical value happens to be zero. Only routing of dynamic powers
        // can make an integral absent. In particular, do not replay all the
        // model's bias-dependent derivative arithmetic as activation logic.
        tracked[index] = index >= primal_count
            && dynamic(support)
            && matches!(
                value.kind,
                CfgValueKind::BlockParameter
                    | CfgValueKind::IntegralDerivative { .. }
                    | CfgValueKind::Select { .. }
                    | CfgValueKind::SumProductsDiv { .. }
                    | CfgValueKind::LaneSumProductsDiv { .. }
                    | CfgValueKind::Binary {
                        op: CfgBinaryOp::Add
                            | CfgBinaryOp::Sub
                            | CfgBinaryOp::Mul
                            | CfgBinaryOp::Div,
                        ..
                    }
                    | CfgValueKind::LaneBinary { .. }
                    | CfgValueKind::LaneScalar { .. }
                    | CfgValueKind::Unary {
                        op: CfgUnaryOp::Neg,
                        ..
                    }
                    | CfgValueKind::LaneWiden { .. }
                    | CfgValueKind::LaneExtract { .. }
            );
        let lanes = source.lanes_of(value.id).map_or_else(
            || vec![None],
            |lanes| lanes.iter().copied().map(Some).collect(),
        );
        for &power in support {
            for &lane in &lanes {
                let id = if tracked[index] {
                    let id = ValueId::from(function.values.len());
                    function.values.push(CfgValue {
                        id,
                        value_type: CfgValueType::Real,
                        kind: CfgValueKind::BlockParameter,
                    });
                    id
                } else {
                    one
                };
                masks[index].insert((power, lane), id);
            }
        }
    }
    let get = |value: ValueId, power, lane| {
        let lane = if source.value(value).value_type.shape().is_some() {
            lane
        } else {
            None
        };
        masks[usize::from(value)]
            .get(&(power, lane))
            .copied()
            .unwrap_or(zero)
    };
    for block in &source.blocks {
        if control.is_cancelled() {
            return Err(FrequencyError::Cancelled(PipelineCancelled {
                phase: PipelinePhase::DerivativeExtraction,
            }));
        }
        let mut instructions = Vec::new();
        let mut params = Vec::new();
        for param in &block.params {
            if tracked[usize::from(*param)] {
                params.extend(masks[usize::from(*param)].values().copied());
            }
        }
        for instruction in &block.instructions {
            let value = source.value(instruction.result);
            if !tracked[usize::from(value.id)] {
                continue;
            }
            for (&(power, lane), &target) in &masks[usize::from(value.id)] {
                let mut builder = Activity {
                    function,
                    instructions: &mut instructions,
                    zero,
                    one,
                };
                let mut product_sum = |terms: &[(ValueId, ValueId)]| {
                    let mut sum = zero;
                    for &(left, right) in terms {
                        for &a in &powers[usize::from(left)] {
                            for &b in &powers[usize::from(right)] {
                                if a.product(b) == power {
                                    let product = builder.binary(
                                        CfgBinaryOp::Mul,
                                        get(left, a, lane),
                                        get(right, b, lane),
                                    );
                                    sum = builder.binary(CfgBinaryOp::Max, sum, product);
                                }
                            }
                        }
                    }
                    sum
                };
                let result = match &value.kind {
                    CfgValueKind::IntegralDerivative {
                        input_derivative,
                        wrap,
                        ..
                    } => {
                        let input = power.idt.checked_sub(1).map_or(zero, |idt| {
                            get(
                                *input_derivative,
                                DynamicPower {
                                    ddt: power.ddt,
                                    idt,
                                },
                                lane,
                            )
                        });
                        let modulus = wrap
                            .as_ref()
                            .map_or(zero, |(_, _, derivative)| get(*derivative, power, lane));
                        builder.binary(CfgBinaryOp::Max, input, modulus)
                    }
                    CfgValueKind::Select {
                        condition,
                        then_value,
                        else_value,
                    } => {
                        let then_value = get(*then_value, power, lane);
                        let else_value = get(*else_value, power, lane);
                        if then_value == else_value {
                            then_value
                        } else {
                            builder.push(CfgValueKind::Select {
                                condition: *condition,
                                then_value,
                                else_value,
                            })
                        }
                    }
                    CfgValueKind::SumProductsDiv { terms, .. }
                    | CfgValueKind::LaneSumProductsDiv { terms, .. } => product_sum(terms),
                    CfgValueKind::Binary { op, left, right }
                    | CfgValueKind::LaneBinary { op, left, right }
                    | CfgValueKind::LaneScalar {
                        op,
                        input: left,
                        scalar: right,
                    } => match op {
                        CfgBinaryOp::Mul => product_sum(&[(*left, *right)]),
                        CfgBinaryOp::Div => get(*left, power, lane),
                        CfgBinaryOp::Add | CfgBinaryOp::Sub => builder.binary(
                            CfgBinaryOp::Max,
                            get(*left, power, lane),
                            get(*right, power, lane),
                        ),
                        _ => one, // Frequency-independent derivative arithmetic.
                    },
                    CfgValueKind::Unary { input, .. } | CfgValueKind::LaneWiden { input } => {
                        get(*input, power, lane)
                    }
                    CfgValueKind::LaneExtract { input, lane } => get(*input, power, Some(*lane)),
                    _ => unreachable!("tracked activity instruction"),
                };
                // Keep the reserved id for forward edges; the ordinary CFG
                // optimizer removes this exact copy after all blocks exist.
                builder.function.values[usize::from(target)].kind =
                    if let CfgValueKind::RealConstant(value) = builder.function.value(result).kind {
                        CfgValueKind::RealConstant(value)
                    } else {
                        CfgValueKind::Binary {
                            op: CfgBinaryOp::Mul,
                            left: result,
                            right: one,
                        }
                    };
                builder.instructions.push(CfgInstruction { result: target });
            }
        }
        let mut additions = Vec::new();
        for (target, args) in edges(&block.terminator) {
            let mut extra = Vec::new();
            for (&param, &arg) in source.block(target).params.iter().zip(args) {
                if tracked[usize::from(param)] {
                    for &(power, lane) in masks[usize::from(param)].keys() {
                        extra.push(get(arg, power, lane));
                    }
                }
            }
            additions.push(extra);
        }
        let target = &mut function.blocks[usize::from(block.id)];
        target.params.extend(params);
        target.instructions.extend(instructions);
        let mut additions = additions.into_iter();
        match &mut target.terminator {
            CfgTerminator::Jump { args, .. } => args.extend(additions.next().unwrap()),
            CfgTerminator::Branch {
                then_args,
                else_args,
                ..
            } => {
                then_args.extend(additions.next().unwrap());
                else_args.extend(additions.next().unwrap());
            }
            _ => {}
        }
    }
    function.blocks[usize::from(source.entry)]
        .instructions
        .splice(0..0, constants);
    function.validate().map_err(|error| {
        FrequencyError::Unsupported(format!("frequency activation CFG: {error}"))
    })?;
    Ok(masks
        .into_iter()
        .map(|masks| {
            masks
                .into_iter()
                .filter_map(|((power, lane), value)| lane.is_none().then_some((power, value)))
                .collect()
        })
        .collect())
}

struct Activity<'a> {
    function: &'a mut CfgFunction,
    instructions: &'a mut Vec<CfgInstruction>,
    zero: ValueId,
    one: ValueId,
}

impl Activity<'_> {
    fn push(&mut self, kind: CfgValueKind) -> ValueId {
        let id = ValueId::from(self.function.values.len());
        self.function.values.push(CfgValue {
            id,
            value_type: CfgValueType::Real,
            kind,
        });
        self.instructions.push(CfgInstruction { result: id });
        id
    }

    fn binary(&mut self, op: CfgBinaryOp, left: ValueId, right: ValueId) -> ValueId {
        // Every operand is a structural zero/one flag, so these identities
        // cannot erase a numerical NaN or change signed-zero arithmetic.
        let constant = |value| match self.function.value(value).kind {
            CfgValueKind::RealConstant(value) => Some(value),
            _ => None,
        };
        match (op, constant(left), constant(right)) {
            (CfgBinaryOp::Max, Some(1.0), _) | (CfgBinaryOp::Max, _, Some(1.0)) => self.one,
            (CfgBinaryOp::Mul, Some(0.0), _) | (CfgBinaryOp::Mul, _, Some(0.0)) => self.zero,
            (CfgBinaryOp::Max, Some(0.0), _) | (CfgBinaryOp::Mul, Some(1.0), _) => right,
            (CfgBinaryOp::Max, _, Some(0.0)) | (CfgBinaryOp::Mul, _, Some(1.0)) => left,
            _ if left == right => left,
            _ => self.push(CfgValueKind::Binary { op, left, right }),
        }
    }
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
        CfgValueKind::IntegralDerivative {
            input_derivative,
            wrap,
            ..
        } => {
            let mut result: Powers = at(input_derivative)
                .iter()
                .map(|power| power.product(DynamicPower { ddt: 0, idt: 1 }))
                .collect();
            if let Some((_, _, modulus_derivative)) = wrap {
                result.extend(at(modulus_derivative));
            }
            result
        }
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
        self.zero(ty)
    }

    fn zero(&mut self, ty: CfgValueType) -> ValueId {
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
            CfgValueKind::IntegralDerivative {
                primal,
                input_derivative,
                wrap: Some((modulus, offset, modulus_derivative)),
                ..
            } => {
                let ic = match self.function.values[usize::from(*primal)].kind {
                    CfgValueKind::IdtMod { ic, .. } | CfgValueKind::IdtModInitial { ic, .. } => ic,
                    _ => {
                        return Err(FrequencyError::Unsupported(
                            "circular-integrator derivative has no direct primal".into(),
                        ));
                    }
                };
                let integral_derivative = if let Some(idt) = power.idt.checked_sub(1) {
                    self.coefficient(
                        *input_derivative,
                        DynamicPower {
                            ddt: power.ddt,
                            idt,
                        },
                        ty,
                    )
                } else {
                    self.zero(ty)
                };
                let modulus_derivative = self.coefficient(*modulus_derivative, power, ty);
                CfgValueKind::IdtModBranchDerivative {
                    primal: *primal,
                    ic,
                    modulus: *modulus,
                    offset: *offset,
                    integral_derivative,
                    modulus_derivative,
                }
            }
            CfgValueKind::IntegralDerivative {
                input_derivative,
                wrap: None,
                ..
            } => {
                let input_power = DynamicPower {
                    ddt: power.ddt,
                    idt: power.idt - 1,
                };
                let input = self.coefficient(*input_derivative, input_power, ty);
                let one = self.push(
                    CfgValueType::Real,
                    CfgValueKind::RealConstant(1.0),
                    instructions,
                );
                if ty.shape().is_some() {
                    CfgValueKind::LaneScalar {
                        op: CfgBinaryOp::Mul,
                        input,
                        scalar: one,
                    }
                } else {
                    CfgValueKind::Binary {
                        op: CfgBinaryOp::Mul,
                        left: input,
                        right: one,
                    }
                }
            }
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
