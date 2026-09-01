//! Reference interpreter for a [`CfgFunction`].
//!
//! The thing every backend is checked against. A generated device is fast and
//! opaque; this is slow and obviously correct, so a disagreement between them
//! is a backend bug by construction rather than an argument about which one to
//! believe.
//!
//! It is generic over the scalar so the same walk serves two purposes: `f64`
//! evaluates the primal, and a dual or complex-step scalar gives derivatives
//! that were computed by a *different* mechanism than the derivative pass —
//! which is the only kind of check on an AD implementation worth having.
//!
//! ## Leaves are computed on demand
//!
//! Constants, parameters, node potentials and the like belong to no block, by
//! design: every block may read them and none owns them. So a read that finds
//! no value asks whether the value is a leaf before reporting it undefined.

use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

use super::cfg::{CfgBinaryOp, CfgFunction, CfgTerminator, CfgUnaryOp, CfgValueKind, is_leaf_kind};
use super::{BlockId, ExprId, ValueId};

/// The arithmetic a CFG needs from its scalar type.
///
/// Predicates go through [`Self::real`] rather than being part of the trait, so
/// a derivative-carrying scalar branches on its value and not on its
/// infinitesimal part — which is what makes a dual number reproduce the primal
/// walk exactly.
pub trait CfgScalar: Copy {
    fn from_f64(value: f64) -> Self;
    fn real(self) -> f64;

    fn neg(self) -> Self;
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn rem(self, rhs: Self) -> Self;
    fn powf(self, rhs: Self) -> Self;
    fn hypot(self, rhs: Self) -> Self;
    /// `self.atan2(rhs)`, with `self` the ordinate and `rhs` the abscissa.
    fn atan2(self, rhs: Self) -> Self;

    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn log10(self) -> Self;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;

    /// `exp` with the argument clamped, matching the runtime's `limexp`.
    fn limexp(self) -> Self {
        if self.real() < LIMEXP_THRESHOLD {
            self.exp()
        } else {
            let scale = Self::from_f64(LIMEXP_THRESHOLD.exp());
            let excess = self.sub(Self::from_f64(LIMEXP_THRESHOLD));
            scale.mul(excess.add(Self::from_f64(1.0)))
        }
    }

    /// The runtime's bounded exponential, clamped at both ends.
    fn limited_exp(self) -> Self {
        if self.real() > LIMEXP_THRESHOLD {
            let scale = Self::from_f64(LIMEXP_MAX);
            let excess = self.sub(Self::from_f64(LIMEXP_THRESHOLD));
            scale.mul(excess.add(Self::from_f64(1.0)))
        } else if self.real() < -LIMEXP_THRESHOLD {
            Self::from_f64(LIMITED_EXP_FLOOR)
        } else {
            self.exp()
        }
    }

    fn limited_exp_derivative(self) -> Self {
        if self.real() > LIMEXP_THRESHOLD {
            Self::from_f64(LIMEXP_MAX)
        } else if self.real() < -LIMEXP_THRESHOLD {
            Self::from_f64(0.0)
        } else {
            self.exp()
        }
    }
}

const LIMEXP_THRESHOLD: f64 = 80.0;
const LIMEXP_MAX: f64 = 5.540_622_384_393_51e34;
const LIMITED_EXP_FLOOR: f64 = 1.804_851_387e-35;

impl CfgScalar for f64 {
    fn from_f64(value: f64) -> Self {
        value
    }
    fn real(self) -> f64 {
        self
    }
    fn neg(self) -> Self {
        -self
    }
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
    fn mul(self, rhs: Self) -> Self {
        self * rhs
    }
    fn div(self, rhs: Self) -> Self {
        self / rhs
    }
    fn rem(self, rhs: Self) -> Self {
        self % rhs
    }
    fn powf(self, rhs: Self) -> Self {
        self.powf(rhs)
    }
    fn hypot(self, rhs: Self) -> Self {
        f64::hypot(self, rhs)
    }
    fn atan2(self, rhs: Self) -> Self {
        f64::atan2(self, rhs)
    }
    fn exp(self) -> Self {
        f64::exp(self)
    }
    fn ln(self) -> Self {
        f64::ln(self)
    }
    fn log10(self) -> Self {
        f64::log10(self)
    }
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
    fn abs(self) -> Self {
        f64::abs(self)
    }
    fn sin(self) -> Self {
        f64::sin(self)
    }
    fn cos(self) -> Self {
        f64::cos(self)
    }
    fn tan(self) -> Self {
        f64::tan(self)
    }
    fn sinh(self) -> Self {
        f64::sinh(self)
    }
    fn cosh(self) -> Self {
        f64::cosh(self)
    }
    fn tanh(self) -> Self {
        f64::tanh(self)
    }
    fn asin(self) -> Self {
        f64::asin(self)
    }
    fn acos(self) -> Self {
        f64::acos(self)
    }
    fn atan(self) -> Self {
        f64::atan(self)
    }
    fn asinh(self) -> Self {
        f64::asinh(self)
    }
    fn acosh(self) -> Self {
        f64::acosh(self)
    }
    fn atanh(self) -> Self {
        f64::atanh(self)
    }
    fn floor(self) -> Self {
        f64::floor(self)
    }
    fn ceil(self) -> Self {
        f64::ceil(self)
    }
}

/// Everything a CFG can read that is not defined inside it.
#[derive(Debug, Clone, Default)]
pub struct CfgEvalInputs<S> {
    /// Parameter values, indexed by `ParamId`.
    pub parameters: Vec<S>,
    /// Which parameters the instance set, indexed by `ParamId`.
    pub parameter_given: Vec<bool>,
    /// External-terminal connection state, indexed by canonical port ordinal.
    pub port_connected: Vec<bool>,
    /// Accepted event-controlled procedural state, in dense generated slot
    /// order.
    pub event_state: Vec<S>,
    /// Node potentials, indexed by `NodeId`.
    pub node_potentials: Vec<S>,
    /// Declared-branch flows, indexed by `BranchId`.
    pub branch_flows: Vec<S>,
    /// Branch-unknown flows, indexed by `BranchUnknownId`.
    pub branch_unknown_flows: Vec<S>,
    pub temperature: S,
    pub thermal_voltage: S,
    pub multiplicity: S,
    pub time: S,
    /// Analysis names that are active, lowercased.
    pub analyses: HashSet<SmolStr>,
    /// `$simparam` overrides, lowercased. Missing names use the fallback the
    /// source supplied.
    pub simparams: HashMap<SmolStr, f64>,
    /// What `ddt` returns. Zero for a static evaluation, which is what the
    /// primal goldens are; the transient companion form is the backend's job,
    /// not the interpreter's.
    pub ddt: S,
    /// The integration rule's `d/dt` coefficient. Zero alongside [`Self::ddt`]
    /// for a static evaluation, so a reactive branch contributes nothing to
    /// either the residual or the Jacobian.
    pub ddt_scale: S,
    /// What `idt` returns, and the `dt` it accumulates by. Zero alongside
    /// [`Self::ddt`] for the same reason: a static evaluation has no step to
    /// integrate over, and the running total is per-instance history the
    /// interpreter does not keep.
    pub idt: S,
    pub idt_scale: S,
    /// Results supplied for stateful event controls, keyed by their canonical
    /// call expression. The reference interpreter has no accepted-history
    /// owner, so omitted sites evaluate false while still evaluating every
    /// operand on the executed path.
    pub event_controls: HashMap<ExprId, S>,
    /// What coarser invalidation stages cached, by slot. Empty for a whole
    /// unsplit function, which reads no slot.
    pub staged: Vec<S>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CfgEvalSnapshot<S> {
    /// Every scalar value that the executed path defined.
    pub values: Vec<Option<S>>,
    /// Every packed derivative the executed path defined, laid out over the
    /// value's shape. Held apart from the scalars rather than in one enum so
    /// reading a scalar stays a `Copy`.
    pub lanes: Vec<Option<Vec<S>>>,
}

impl<S: Copy> CfgEvalSnapshot<S> {
    pub fn value(&self, id: ValueId) -> Option<S> {
        self.values.get(usize::from(id)).copied().flatten()
    }

    /// A packed derivative, in its shape's layout order.
    pub fn lanes(&self, id: ValueId) -> Option<&[S]> {
        self.lanes.get(usize::from(id))?.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgEvalError {
    UndefinedValue(ValueId),
    UnterminatedBlock(BlockId),
    MissingInput(&'static str, usize),
    /// A `ddx` reached evaluation, which means the derivative pass has not run.
    UndifferentiatedDdx(ValueId),
    /// The path did not reach a `Return` within the step budget, which for a
    /// well-formed model means a loop whose condition never falsifies.
    StepLimitExceeded(usize),
    /// A discrete-domain construct reached the analog interpreter.
    ///
    /// This interpreter evaluates one analog body to a number. A process is
    /// not that: it suspends, it reads signals that change while it is
    /// suspended, and it has no single value to return. Running one needs the
    /// event kernel, so meeting one here is a routing bug rather than a model
    /// the interpreter merely does not support, and it says so.
    DigitalConstructInAnalogEvaluation {
        what: &'static str,
    },
}

impl std::fmt::Display for CfgEvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedValue(value) => write!(f, "{value} was read before it was defined"),
            Self::UnterminatedBlock(block) => write!(f, "{block} has no terminator"),
            Self::MissingInput(what, index) => write!(f, "no input supplied for {what} {index}"),
            Self::UndifferentiatedDdx(value) => {
                write!(f, "{value} is a ddx the derivative pass has not resolved")
            }
            Self::StepLimitExceeded(limit) => {
                write!(f, "evaluation did not return within {limit} blocks")
            }
            Self::DigitalConstructInAnalogEvaluation { what } => write!(
                f,
                "{what} is a discrete-domain construct and cannot be evaluated \
                 as part of an analog body"
            ),
        }
    }
}

impl std::error::Error for CfgEvalError {}

/// How many block entries one evaluation may make.
///
/// Generous for any compact model — the deepest run-time loops in the shipped
/// corpus iterate in the tens — and finite so a malformed back edge reports
/// rather than hangs.
pub const DEFAULT_STEP_LIMIT: usize = 1_000_000;

pub fn evaluate<S: CfgScalar>(
    function: &CfgFunction,
    inputs: &CfgEvalInputs<S>,
) -> Result<CfgEvalSnapshot<S>, CfgEvalError> {
    evaluate_with_limit(function, inputs, DEFAULT_STEP_LIMIT)
}

pub fn evaluate_with_limit<S: CfgScalar>(
    function: &CfgFunction,
    inputs: &CfgEvalInputs<S>,
    step_limit: usize,
) -> Result<CfgEvalSnapshot<S>, CfgEvalError> {
    let mut evaluator = Evaluator {
        function,
        inputs,
        values: vec![None; function.values.len()],
        lanes: vec![None; function.values.len()],
    };
    evaluator.run(step_limit)?;
    Ok(CfgEvalSnapshot {
        values: evaluator.values,
        lanes: evaluator.lanes,
    })
}

struct Evaluator<'a, S> {
    function: &'a CfgFunction,
    inputs: &'a CfgEvalInputs<S>,
    values: Vec<Option<S>>,
    lanes: Vec<Option<Vec<S>>>,
}

impl<S: CfgScalar> Evaluator<'_, S> {
    fn run(&mut self, step_limit: usize) -> Result<(), CfgEvalError> {
        // A local copy of the shared reference, so walking the graph does not
        // borrow `self` while the value table is being written.
        let function = self.function;

        // Leaves first, all of them, because they belong to no block.
        //
        // Realising them lazily on first reference is enough to compute the
        // graph, but it leaves a leaf that nothing references — a residual or a
        // Jacobian entry that simplified down to a bare node potential —
        // without a value, while the emitter, which writes every leaf into a
        // prologue, produces one. Two backends that disagree about which values
        // exist disagree about the answer, and `bsimbulk` is where that showed.
        for value in &function.values {
            if !is_leaf_kind(&value.kind) {
                continue;
            }
            if value.value_type.shape().is_some() {
                let lanes = self.compute_lanes(value.id)?;
                self.lanes[usize::from(value.id)] = Some(lanes);
            } else {
                let realised = self.compute(value.id)?;
                self.values[usize::from(value.id)] = Some(realised);
            }
        }

        let mut block = function.entry;
        for _ in 0..step_limit {
            for instruction in &function.block(block).instructions {
                let result = instruction.result;
                if function.value(result).value_type.shape().is_some() {
                    let lanes = self.compute_lanes(result)?;
                    self.lanes[usize::from(result)] = Some(lanes);
                } else {
                    let value = self.compute(result)?;
                    self.values[usize::from(result)] = Some(value);
                }
            }

            match &function.block(block).terminator {
                CfgTerminator::Return => return Ok(()),
                CfgTerminator::Unset => return Err(CfgEvalError::UnterminatedBlock(block)),
                CfgTerminator::Wait { .. } => {
                    return Err(CfgEvalError::DigitalConstructInAnalogEvaluation {
                        what: "a process suspension",
                    });
                }
                CfgTerminator::Jump { target, args } => {
                    self.pass_arguments(*target, args)?;
                    block = *target;
                }
                CfgTerminator::Branch {
                    condition,
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                } => {
                    let taken = self.read(*condition)?.real() != 0.0;
                    let (target, args) = if taken {
                        (*then_target, then_args)
                    } else {
                        (*else_target, else_args)
                    };
                    self.pass_arguments(target, args)?;
                    block = target;
                }
            }
        }
        Err(CfgEvalError::StepLimitExceeded(step_limit))
    }

    /// Bind a successor's parameters to the arguments on the edge.
    ///
    /// Read every argument before writing any parameter: a loop that swaps two
    /// carried variables passes each one's old value, and writing in place
    /// would feed the first write into the second read.
    fn pass_arguments(&mut self, target: BlockId, args: &[ValueId]) -> Result<(), CfgEvalError> {
        let mut incoming = Vec::with_capacity(args.len());
        for arg in args {
            if self.function.value(*arg).value_type.shape().is_some() {
                incoming.push(Carried::Lanes(self.read_lanes(*arg)?));
            } else {
                incoming.push(Carried::Scalar(self.read(*arg)?));
            }
        }
        for (param, value) in self.function.block(target).params.iter().zip(incoming) {
            match value {
                Carried::Scalar(value) => self.values[usize::from(*param)] = Some(value),
                Carried::Lanes(lanes) => self.lanes[usize::from(*param)] = Some(lanes),
            }
        }
        Ok(())
    }

    fn read(&mut self, id: ValueId) -> Result<S, CfgEvalError> {
        if let Some(value) = self.values[usize::from(id)] {
            return Ok(value);
        }
        // Leaves live in no block, so this is where they are realised.
        let value = self.compute(id)?;
        self.values[usize::from(id)] = Some(value);
        Ok(value)
    }

    fn read_lanes(&mut self, id: ValueId) -> Result<Vec<S>, CfgEvalError> {
        if let Some(lanes) = &self.lanes[usize::from(id)] {
            return Ok(lanes.clone());
        }
        let lanes = self.compute_lanes(id)?;
        self.lanes[usize::from(id)] = Some(lanes.clone());
        Ok(lanes)
    }

    /// A packed derivative, in its shape's layout order.
    fn compute_lanes(&mut self, id: ValueId) -> Result<Vec<S>, CfgEvalError> {
        let width = self
            .function
            .lanes_of(id)
            .ok_or(CfgEvalError::UndefinedValue(id))?
            .len();
        let kind = self.function.value(id).kind.clone();
        Ok(match kind {
            CfgValueKind::LaneSplat(constant) => vec![S::from_f64(constant); width],
            CfgValueKind::LaneWiden { input } => {
                let source = self.read_lanes(input)?;
                let from = self.function.lanes_of(input).unwrap_or(&[]).to_vec();
                self.function
                    .lanes_of(id)
                    .unwrap_or(&[])
                    .iter()
                    .map(|lane| match from.iter().position(|held| held == lane) {
                        Some(position) => source[position],
                        None => S::from_f64(0.0),
                    })
                    .collect()
            }
            CfgValueKind::LaneBinary { op, left, right } => {
                let left = self.read_lanes(left)?;
                let right = self.read_lanes(right)?;
                left.into_iter()
                    .zip(right)
                    .map(|(left, right)| apply_binary(op, left, right))
                    .collect()
            }
            CfgValueKind::LaneScalar { op, input, scalar } => {
                let input = self.read_lanes(input)?;
                let scalar = self.read(scalar)?;
                input
                    .into_iter()
                    .map(|lane| apply_binary(op, lane, scalar))
                    .collect()
            }
            // The CFG interpreter is a static/DC oracle. At equilibrium a
            // transition has the exact direct coefficient one, so its packed
            // Jacobian action is the input derivative unchanged.
            CfgValueKind::TransitionDerivative {
                input,
                input_derivative,
                delay,
                rise,
                fall,
                ..
            } => {
                self.read(input)?;
                self.read(delay)?;
                self.read(rise)?;
                self.read(fall)?;
                self.read_lanes(input_derivative)?
            }
            // A merge that no predecessor supplied is a bug in the graph, not a
            // derivative of zero.
            _ => return Err(CfgEvalError::UndefinedValue(id)),
        })
    }

    fn compute(&mut self, id: ValueId) -> Result<S, CfgEvalError> {
        let kind = self.function.value(id).kind.clone();
        Ok(match kind {
            CfgValueKind::RealConstant(value) => S::from_f64(value),
            CfgValueKind::BooleanConstant(value) => S::from_f64(f64::from(u8::from(value))),
            // A parameter that no predecessor supplied a value for is a bug in
            // the graph, not a default of zero.
            CfgValueKind::BlockParameter => return Err(CfgEvalError::UndefinedValue(id)),
            CfgValueKind::Parameter(parameter) => {
                *self.inputs.parameters.get(usize::from(parameter)).ok_or(
                    CfgEvalError::MissingInput("parameter", usize::from(parameter)),
                )?
            }
            CfgValueKind::ParameterGiven(parameter) => {
                let given = self
                    .inputs
                    .parameter_given
                    .get(usize::from(parameter))
                    .copied()
                    .unwrap_or(false);
                S::from_f64(f64::from(u8::from(given)))
            }
            CfgValueKind::PortConnected(port) => {
                let connected = self
                    .inputs
                    .port_connected
                    .get(port as usize)
                    .copied()
                    .unwrap_or(false);
                S::from_f64(f64::from(u8::from(connected)))
            }
            CfgValueKind::EventState(slot) => *self
                .inputs
                .event_state
                .get(slot as usize)
                .ok_or(CfgEvalError::MissingInput("event state", slot as usize))?,
            CfgValueKind::Temperature => self.inputs.temperature,
            CfgValueKind::ThermalVoltage => self.inputs.thermal_voltage,
            CfgValueKind::Multiplicity => self.inputs.multiplicity,
            CfgValueKind::Time => self.inputs.time,
            CfgValueKind::Analysis(name) => {
                S::from_f64(f64::from(u8::from(self.inputs.analyses.contains(&name))))
            }
            CfgValueKind::SimParam { name, fallback } => match self.inputs.simparams.get(&name) {
                Some(value) => S::from_f64(*value),
                None => self.read(fallback)?,
            },
            CfgValueKind::NodePotential(node) => *self
                .inputs
                .node_potentials
                .get(usize::from(node))
                .ok_or(CfgEvalError::MissingInput("node", usize::from(node)))?,
            CfgValueKind::BranchFlow(branch) => *self
                .inputs
                .branch_flows
                .get(usize::from(branch))
                .ok_or(CfgEvalError::MissingInput("branch", usize::from(branch)))?,
            CfgValueKind::BranchUnknownFlow(unknown) => *self
                .inputs
                .branch_unknown_flows
                .get(usize::from(unknown))
                .ok_or(CfgEvalError::MissingInput(
                    "branch unknown",
                    usize::from(unknown),
                ))?,
            // Noise primitives have no large-signal value. Their unit
            // derivative is materialized only by the AD pass.
            CfgValueKind::NoiseProcess(_) => S::from_f64(0.0),
            // Static evaluation: the operator's own value is supplied, and the
            // input is still evaluated because it may have side conditions the
            // path depends on.
            CfgValueKind::Ddt { input, .. } => {
                self.read(input)?;
                self.inputs.ddt
            }
            CfgValueKind::DdtScale => self.inputs.ddt_scale,
            // Same static treatment: the running total is supplied, and both
            // operands are still evaluated because either may carry a side
            // condition the path depends on.
            CfgValueKind::Idt { input, ic, .. } => {
                self.read(input)?;
                self.read(ic)?;
                self.inputs.idt
            }
            CfgValueKind::IdtScale => self.inputs.idt_scale,
            CfgValueKind::Transition {
                input,
                delay,
                rise,
                fall,
                ..
            } => {
                let value = self.read(input)?;
                self.read(delay)?;
                self.read(rise)?;
                self.read(fall)?;
                value
            }
            CfgValueKind::TransitionDerivative {
                input,
                input_derivative,
                delay,
                rise,
                fall,
                ..
            } => {
                self.read(input)?;
                self.read(delay)?;
                self.read(rise)?;
                self.read(fall)?;
                self.read(input_derivative)?
            }
            CfgValueKind::Cross {
                operator,
                input,
                direction,
                time_tol,
                expr_tol,
                enable,
            } => {
                self.read(input)?;
                self.read(direction)?;
                self.read(time_tol)?;
                self.read(expr_tol)?;
                self.read(enable)?;
                self.inputs
                    .event_controls
                    .get(&operator)
                    .copied()
                    .unwrap_or_else(|| S::from_f64(0.0))
            }
            CfgValueKind::Above {
                operator,
                input,
                time_tol,
                expr_tol,
                enable,
            } => {
                self.read(input)?;
                self.read(time_tol)?;
                self.read(expr_tol)?;
                self.read(enable)?;
                self.inputs
                    .event_controls
                    .get(&operator)
                    .copied()
                    .unwrap_or_else(|| S::from_f64(0.0))
            }
            CfgValueKind::Timer {
                operator,
                start,
                period,
                time_tol,
                enable,
            } => {
                self.read(start)?;
                self.read(period)?;
                self.read(time_tol)?;
                self.read(enable)?;
                self.inputs
                    .event_controls
                    .get(&operator)
                    .copied()
                    .unwrap_or_else(|| S::from_f64(0.0))
            }
            CfgValueKind::Staged { slot } => *self
                .inputs
                .staged
                .get(slot as usize)
                .ok_or(CfgEvalError::MissingInput("staged value", slot as usize))?,
            // The proposed value, not the limiter body — this interpreter is the
            // limiting-disabled semantics, which is what the generated device
            // does when `ctx.limiting_enabled()` is false.
            //
            // It is not a simplification for the reference's convenience. A
            // limited evaluation is a function of the bias *and* the previous
            // iterate, so differentiating one differentiates the limiter, and
            // both oracles built on this interpreter exist to check the model's
            // Jacobian. Running the body here would make them measure `dL/dv`
            // and disagree with the `dL/dv := 1` convention the stamp uses —
            // which is the same reason the Phase 0 golden oracle requires
            // limiting off. Limiting is damping applied to a step; the equations
            // are what this evaluates.
            //
            // The limiter body is still lowered, still differentiated, and still
            // emitted: what it produces is the correction lane, and that is
            // checked where it is applied rather than here.
            CfgValueKind::Limit { proposed, .. } | CfgValueKind::LimitPrevious { proposed, .. } => {
                self.read(proposed)?
            }
            // Reading a lane the derivative pass has not created. A `ddx` that
            // survives to evaluation is an un-differentiated function, and the
            // honest answer is that its Jacobian entry does not exist yet.
            CfgValueKind::Ddx { value, .. } => {
                self.read(value)?;
                return Err(CfgEvalError::UndifferentiatedDdx(id));
            }
            CfgValueKind::Unary { op, input } => {
                let input = self.read(input)?;
                apply_unary(op, input)
            }
            CfgValueKind::Binary { op, left, right } => {
                let left = self.read(left)?;
                let right = self.read(right)?;
                apply_binary(op, left, right)
            }
            CfgValueKind::LaneExtract { input, lane } => {
                let position = self
                    .function
                    .lane_position(input, lane)
                    .ok_or(CfgEvalError::UndefinedValue(id))?;
                self.read_lanes(input)?[position]
            }
            // Packed kinds go through `compute_lanes`; reaching here means a
            // value's type disagrees with its kind.
            CfgValueKind::LaneSplat(_)
            | CfgValueKind::LaneWiden { .. }
            | CfgValueKind::LaneBinary { .. }
            | CfgValueKind::LaneScalar { .. } => return Err(CfgEvalError::UndefinedValue(id)),

            // Discrete-domain kinds. `CfgScalar` is a real-arithmetic trait —
            // it has `exp`, `ln`, and a chain rule — and there is no honest
            // mapping from a four-state value onto it. Refusing by name is the
            // whole point: an `x` silently becoming `0.0` here would be a
            // wrong waveform rather than an error.
            CfgValueKind::FourStateConstant(_)
            | CfgValueKind::IntegerConstant(_)
            | CfgValueKind::DigitalSignalRead { .. }
            | CfgValueKind::DigitalRealSignalRead { .. }
            | CfgValueKind::DigitalAnalogPotential { .. }
            | CfgValueKind::DigitalRealArithmetic { .. }
            | CfgValueKind::DigitalRealCompare { .. }
            | CfgValueKind::DigitalRealSelect { .. }
            | CfgValueKind::DigitalRealToBits { .. }
            | CfgValueKind::DigitalBitsToReal { .. }
            | CfgValueKind::DigitalBitwise { .. }
            | CfgValueKind::DigitalBitwiseNot { .. }
            | CfgValueKind::DigitalLogical { .. }
            | CfgValueKind::DigitalLogicalNot { .. }
            | CfgValueKind::DigitalEquality { .. }
            | CfgValueKind::DigitalCaseMatch { .. }
            | CfgValueKind::DigitalRelational { .. }
            | CfgValueKind::DigitalArithmetic { .. }
            | CfgValueKind::DigitalShift { .. }
            | CfgValueKind::DigitalPartSelect { .. }
            | CfgValueKind::DigitalConcat { .. }
            | CfgValueKind::DigitalSelect { .. }
            | CfgValueKind::DigitalBlockingWrite { .. }
            | CfgValueKind::DigitalNonblockingWrite { .. }
            | CfgValueKind::DigitalDriverWrite { .. } => {
                return Err(CfgEvalError::DigitalConstructInAnalogEvaluation {
                    what: "a four-state or integer value",
                });
            }
        })
    }
}

/// What an edge carries into a block parameter.
enum Carried<S> {
    Scalar(S),
    Lanes(Vec<S>),
}

pub(super) fn apply_unary<S: CfgScalar>(op: CfgUnaryOp, input: S) -> S {
    match op {
        CfgUnaryOp::Neg => input.neg(),
        CfgUnaryOp::Not => S::from_f64(f64::from(u8::from(input.real() == 0.0))),
        CfgUnaryOp::Exp => input.exp(),
        CfgUnaryOp::LimExp => input.limexp(),
        CfgUnaryOp::LimitedExp => input.limited_exp(),
        CfgUnaryOp::LimitedExpDerivative => input.limited_exp_derivative(),
        CfgUnaryOp::Ln => input.ln(),
        CfgUnaryOp::Log10 => input.log10(),
        CfgUnaryOp::Sqrt => input.sqrt(),
        CfgUnaryOp::Abs => input.abs(),
        CfgUnaryOp::Sin => input.sin(),
        CfgUnaryOp::Cos => input.cos(),
        CfgUnaryOp::Tan => input.tan(),
        CfgUnaryOp::Sinh => input.sinh(),
        CfgUnaryOp::Cosh => input.cosh(),
        CfgUnaryOp::Tanh => input.tanh(),
        CfgUnaryOp::Asin => input.asin(),
        CfgUnaryOp::Acos => input.acos(),
        CfgUnaryOp::Atan => input.atan(),
        CfgUnaryOp::Asinh => input.asinh(),
        CfgUnaryOp::Acosh => input.acosh(),
        CfgUnaryOp::Atanh => input.atanh(),
        CfgUnaryOp::Floor => input.floor(),
        CfgUnaryOp::Ceil => input.ceil(),
    }
}

pub(super) fn apply_binary<S: CfgScalar>(op: CfgBinaryOp, left: S, right: S) -> S {
    let predicate = |holds: bool| S::from_f64(f64::from(u8::from(holds)));
    match op {
        CfgBinaryOp::Add => left.add(right),
        CfgBinaryOp::Sub => left.sub(right),
        CfgBinaryOp::Mul => left.mul(right),
        CfgBinaryOp::Div => left.div(right),
        CfgBinaryOp::Mod => left.rem(right),
        CfgBinaryOp::Pow => left.powf(right),
        CfgBinaryOp::Hypot => left.hypot(right),
        CfgBinaryOp::Atan2 => left.atan2(right),
        // Selecting rather than computing keeps the derivative on the operand
        // that actually won, which is what a hand-written `fmin` does too.
        CfgBinaryOp::Min => {
            if left.real() <= right.real() {
                left
            } else {
                right
            }
        }
        CfgBinaryOp::Max => {
            if left.real() >= right.real() {
                left
            } else {
                right
            }
        }
        CfgBinaryOp::Eq => predicate(left.real() == right.real()),
        CfgBinaryOp::Ne => predicate(left.real() != right.real()),
        CfgBinaryOp::Lt => predicate(left.real() < right.real()),
        CfgBinaryOp::Le => predicate(left.real() <= right.real()),
        CfgBinaryOp::Gt => predicate(left.real() > right.real()),
        CfgBinaryOp::Ge => predicate(left.real() >= right.real()),
        CfgBinaryOp::And => predicate(left.real() != 0.0 && right.real() != 0.0),
        CfgBinaryOp::Or => predicate(left.real() != 0.0 || right.real() != 0.0),
    }
}
