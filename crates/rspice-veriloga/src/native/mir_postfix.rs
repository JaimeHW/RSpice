//! A postfix-plan walker generic over the scalar, so the shipped route can be
//! evaluated in a precision the shipped route does not have.
//!
//! # Why this exists
//!
//! [`super::cfg_mir_census`] measures the two routes' agreement against a
//! reference computed in [`DoubleDouble`](super::double_double::DoubleDouble).
//! The CFG route already has a door for that — its plan is a lowering of a
//! `CfgFunction`, and [`crate::canonical_ir::evaluate_cfg`] walks one with any
//! [`CfgScalar`]. The shipped route has none: its plan entries are flat
//! [`NativeProgram`] streams that only a machine-code backend consumes, and
//! there is no interpreter for them anywhere in the estate.
//!
//! So one is written here, for the census alone. It is generic over the scalar
//! for the same reason the CFG interpreter is: `f64` reproduces what the
//! compiled program does — which is how this walker is *checked*, entry by
//! entry, against the machine code on every module the census runs — and
//! `DoubleDouble` gives the reference the same walk could not otherwise have.
//!
//! # Where the semantics come from
//!
//! [`crate::native::x64::codegen`], operation by operation, because that is
//! what the census actually executes. Three of its rules are worth naming
//! because a plausible reading of the operation name gets them wrong:
//!
//! * **Truthiness is "not exactly zero", NaN included.** The backend clears the
//!   sign bit and tests the remaining payload, so every NaN is true. `IfElse`,
//!   `Logical` and `LogicalConst` all read it.
//! * **`LoadCurrent` reads `branch_currents` and `LoadPriorCurrent` reads
//!   `currents`.** The names are the other way round from the context fields
//!   they address.
//! * **`min` and `max` select rather than blend, and prefer the left operand.**
//!   `MINSD`/`MAXSD` return the right operand when either is NaN, and the
//!   backend then patches the left one back in when the right is the NaN or
//!   when both are zero.
//!
//! # What it refuses, and why refusing is the right answer
//!
//! Every operation that reaches into live analog-operator storage — a Laplace
//! or Z-domain filter, a transition, a slew, a delay buffer, a crossing
//! detector, a `$limit` history, a lookup table — is a call into the runtime
//! whose result is a property of storage this census does not stand up. The
//! integrators are the exception and not an inconsistency: with
//! `integration_active` clear the runtime's own helpers return an exact
//! constant, which is reproduced here rather than guessed.
//!
//! A refusal is named by operation and reported, never silently substituted.
//! An entry the walker refuses simply has no double-double reference, and the
//! census says which operation cost it one.

use std::collections::BTreeSet;

use crate::array_index::checked_array_slot;
use crate::canonical_ir::CfgScalar;
use crate::integer_runtime::{integer_binary, real_to_integer};
use crate::jit::assignment::NativeAssignment;
use crate::jit::expr::{
    BinaryMathOp, CompareOp, ExtremumOp, LogicalOp, NativeOp, NativeProgram, UnaryMathOp,
    VoltageNode, native_op_name, runtime_integer_operation,
};

/// The backend's own thermal-voltage coefficient, `k/q`.
const THERMAL_VOLTAGE_PER_K: f64 = 1.380_649e-23 / 1.602_176_634e-19;

/// The runtime loop bound both native backends encode.
const MAX_RUNTIME_LOOP_ITERATIONS: usize = 100_000;

/// Why a program produced no value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PostfixRefusal {
    /// An operation this walker does not implement. Named, and counted per
    /// module by the census.
    Operation(&'static str),
    /// The stream did not leave exactly one value on the stack, or an operation
    /// found fewer operands than it needs. A finding about the plan rather than
    /// about this walker.
    Malformed(&'static str),
    /// A load addressed storage the operating point does not have, or an
    /// integer conversion the runtime refuses. The compiled route reports a
    /// runtime error at the same point and the census discards the comparison.
    RuntimeError(&'static str),
}

impl PostfixRefusal {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Operation(name) | Self::Malformed(name) | Self::RuntimeError(name) => name,
        }
    }
}

/// Every `f64` the compiled route would read, at one operating point.
///
/// Borrowed from the arrays the census hands the compiled plans through
/// [`EvalContext`](crate::native::EvalContext), so "seeded exactly from the
/// `f64` point" is a property of the storage rather than of a conversion.
pub(super) struct MirPoint<'a> {
    pub(super) parameters: &'a [f64],
    pub(super) parameter_given: &'a [u8],
    pub(super) port_connected: &'a [u8],
    pub(super) terminal_voltages: &'a [f64],
    pub(super) internal_voltages: &'a [f64],
    /// Branch unknowns in *runtime* order, which is what a postfix load names.
    pub(super) branch_unknowns: &'a [f64],
    /// `EvalContext::currents`, which `LoadPriorCurrent` reads.
    pub(super) currents: &'a [f64],
    /// `EvalContext::branch_currents`, which `LoadCurrent` reads.
    pub(super) branch_currents: &'a [f64],
    pub(super) temperature: f64,
    pub(super) time: f64,
    pub(super) multiplicity: f64,
    pub(super) analysis: u8,
    pub(super) initial_step: bool,
    pub(super) final_step: bool,
}

impl MirPoint<'_> {
    /// The value `Analysis(id)` yields, from
    /// [`x64::codegen`](crate::native::x64)'s `emit_analysis_check`.
    fn analysis_value(&self, id: u8) -> f64 {
        let active = match id {
            7 => self.initial_step,
            8 => self.final_step,
            5 => matches!(self.analysis, 0 | 4),
            6 => matches!(self.analysis, 1 | 3),
            0..=4 => self.analysis == id,
            _ => false,
        };
        f64::from(u8::from(active))
    }

    fn node_voltage(&self, node: VoltageNode) -> Result<f64, PostfixRefusal> {
        match node {
            VoltageNode::Ground => Ok(0.0),
            VoltageNode::Terminal(index) => self
                .terminal_voltages
                .get(index)
                .copied()
                .ok_or(PostfixRefusal::RuntimeError("LoadVoltage")),
            VoltageNode::Internal(index) => self
                .internal_voltages
                .get(index)
                .copied()
                .ok_or(PostfixRefusal::RuntimeError("LoadInternalVoltage")),
        }
    }
}

/// One walk of a plan's postfix programs in the scalar `S`.
///
/// The variable array is the walk's own: the shipped route reads slots its
/// assignment pass wrote, so a reference that read the *`f64`* slots would be
/// measuring the entry program's rounding and calling it the route's. Both
/// passes are re-walked in `S`, which is the same cone the CFG route inlines.
pub(super) struct PostfixWalk<'a, S: CfgScalar> {
    point: &'a MirPoint<'a>,
    variables: Vec<S>,
    prelude: Vec<S>,
    refused: BTreeSet<&'static str>,
}

impl<'a, S: CfgScalar> PostfixWalk<'a, S> {
    pub(super) fn new(point: &'a MirPoint<'a>, variables: usize, prelude: usize) -> Self {
        Self {
            point,
            variables: vec![S::from_f64(0.0); variables],
            prelude: vec![S::from_f64(0.0); prelude],
            refused: BTreeSet::new(),
        }
    }

    /// Every operation name this walk could not evaluate, in name order.
    pub(super) fn refusals(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.refused.iter().copied()
    }

    /// Run one assignment pass into the variable array.
    ///
    /// A refusal inside an assignment is recorded and the assignment is
    /// skipped, leaving its slot at whatever it held: the walk continues so
    /// that the entries which do not depend on that slot still get a
    /// reference, and the census names the refusal either way.
    pub(super) fn fill_variables(&mut self, assignments: &[NativeAssignment]) {
        for assignment in assignments {
            self.run_assignment(assignment);
        }
    }

    fn run_assignment(&mut self, assignment: &NativeAssignment) {
        match assignment {
            NativeAssignment::Direct { var_index, program } => {
                if let Ok(value) = self.run(program)
                    && let Some(slot) = self.variables.get_mut(*var_index)
                {
                    *slot = value;
                }
            }
            NativeAssignment::Indexed {
                base,
                len,
                lower,
                index,
                value,
            } => {
                let (Ok(index), Ok(value)) = (self.run(index), self.run(value)) else {
                    return;
                };
                if let Ok(slot) = checked_array_slot(index.real(), *base, *len, *lower)
                    && let Some(target) = self.variables.get_mut(slot)
                {
                    *target = value;
                }
            }
            NativeAssignment::Loop { condition, body } => {
                let mut iterations = 0_usize;
                while let Ok(active) = self.run(condition) {
                    if active.real() == 0.0 || iterations >= MAX_RUNTIME_LOOP_ITERATIONS {
                        break;
                    }
                    for statement in body {
                        self.run_assignment(statement);
                    }
                    iterations += 1;
                }
            }
        }
    }

    /// Evaluate one postfix program to its single result.
    pub(super) fn run(&mut self, program: &NativeProgram) -> Result<S, PostfixRefusal> {
        let mut stack: Vec<S> = Vec::with_capacity(program.max_stack_depth().max(1));
        for op in program.ops() {
            match self.step(&mut stack, *op) {
                Ok(()) => {}
                Err(refusal) => {
                    self.refused.insert(refusal.name());
                    return Err(refusal);
                }
            }
        }
        if stack.len() == 1 {
            Ok(stack[0])
        } else {
            let refusal = PostfixRefusal::Malformed("program-result");
            self.refused.insert(refusal.name());
            Err(refusal)
        }
    }

    #[allow(clippy::too_many_lines)]
    fn step(&mut self, stack: &mut Vec<S>, op: NativeOp) -> Result<(), PostfixRefusal> {
        let name = native_op_name(&op);
        match op {
            NativeOp::Const(value) => stack.push(S::from_f64(value)),
            NativeOp::LoadParam(index) => {
                stack.push(S::from_f64(self.read(self.point.parameters, index, name)?));
            }
            NativeOp::LoadParamGiven(index) => {
                let flag = self.read_flag(self.point.parameter_given, index, name)?;
                stack.push(S::from_f64(flag));
            }
            NativeOp::LoadPortConnected(index) => {
                let flag = self.read_flag(self.point.port_connected, index, name)?;
                stack.push(S::from_f64(flag));
            }
            NativeOp::LoadVoltage { pos, neg } => {
                let pos = self.point.node_voltage(pos)?;
                let neg = self.point.node_voltage(neg)?;
                stack.push(S::from_f64(pos).sub(S::from_f64(neg)));
            }
            NativeOp::LoadCurrent(index) => {
                stack.push(S::from_f64(self.read(
                    self.point.branch_currents,
                    index,
                    name,
                )?));
            }
            NativeOp::LoadPriorCurrent(index) => {
                stack.push(S::from_f64(self.read(self.point.currents, index, name)?));
            }
            NativeOp::LoadInternalVoltage(index) => {
                let value = self.read(self.point.internal_voltages, index, name)?;
                stack.push(S::from_f64(value));
            }
            NativeOp::LoadVariable(index) => {
                let value = *self
                    .variables
                    .get(index)
                    .ok_or(PostfixRefusal::RuntimeError(name))?;
                stack.push(value);
            }
            NativeOp::LoadVariableDyn { base, len, lower } => {
                let raw = Self::top(stack, name)?;
                let slot = checked_array_slot(raw.real(), base, len, lower)
                    .map_err(|_| PostfixRefusal::RuntimeError(name))?;
                let value = *self
                    .variables
                    .get(slot)
                    .ok_or(PostfixRefusal::RuntimeError(name))?;
                *stack.last_mut().ok_or(PostfixRefusal::Malformed(name))? = value;
            }
            NativeOp::LoadBranchUnknown(index) => {
                let value = self.read(self.point.branch_unknowns, index, name)?;
                stack.push(S::from_f64(value));
            }
            NativeOp::LoadTemperature => stack.push(S::from_f64(self.point.temperature)),
            NativeOp::LoadThermalVoltage => stack.push(
                S::from_f64(self.point.temperature).mul(S::from_f64(THERMAL_VOLTAGE_PER_K)),
            ),
            NativeOp::LoadTime => stack.push(S::from_f64(self.point.time)),
            NativeOp::Analysis(id) => stack.push(S::from_f64(self.point.analysis_value(id))),
            NativeOp::LoadMfactor => stack.push(S::from_f64(self.point.multiplicity)),
            NativeOp::LoadPreludeSlot(index) => {
                let value = *self
                    .prelude
                    .get(index)
                    .ok_or(PostfixRefusal::RuntimeError(name))?;
                stack.push(value);
            }
            NativeOp::StorePreludeSlot(index) => {
                let value = Self::top(stack, name)?;
                *self
                    .prelude
                    .get_mut(index)
                    .ok_or(PostfixRefusal::RuntimeError(name))? = value;
            }
            NativeOp::Add => Self::binary(stack, name, S::add)?,
            NativeOp::Sub => Self::binary(stack, name, S::sub)?,
            NativeOp::Mul => Self::binary(stack, name, S::mul)?,
            NativeOp::Div => Self::binary(stack, name, S::div)?,
            NativeOp::AddConst(value) => {
                Self::unary(stack, name, |x| x.add(S::from_f64(value)))?;
            }
            NativeOp::SubConst(value) => {
                Self::unary(stack, name, |x| x.sub(S::from_f64(value)))?;
            }
            NativeOp::MulConst(value) => {
                Self::unary(stack, name, |x| x.mul(S::from_f64(value)))?;
            }
            NativeOp::DivConst(value) => {
                Self::unary(stack, name, |x| x.div(S::from_f64(value)))?;
            }
            NativeOp::SubFromConst(value) => {
                Self::unary(stack, name, |x| S::from_f64(value).sub(x))?;
            }
            NativeOp::DivFromConst(value) => {
                Self::unary(stack, name, |x| S::from_f64(value).div(x))?;
            }
            NativeOp::Neg => Self::unary(stack, name, S::neg)?,
            NativeOp::Abs => Self::unary(stack, name, S::abs)?,
            NativeOp::Square => Self::unary(stack, name, |x| x.mul(x))?,
            NativeOp::Sqrt => Self::unary(stack, name, S::sqrt)?,
            NativeOp::Compare(op) => {
                Self::binary(stack, name, |left, right| {
                    S::from_f64(f64::from(u8::from(compare(op, left.real(), right.real()))))
                })?;
            }
            NativeOp::CompareConst(op, value) => {
                Self::unary(stack, name, |x| {
                    S::from_f64(f64::from(u8::from(compare(op, x.real(), value))))
                })?;
            }
            NativeOp::Logical(LogicalOp::Not) => {
                Self::unary(stack, name, |x| S::from_f64(f64::from(u8::from(!truthy(x)))))?;
            }
            NativeOp::Logical(op) => {
                Self::binary(stack, name, |left, right| {
                    let value = match op {
                        LogicalOp::And => truthy(left) && truthy(right),
                        _ => truthy(left) || truthy(right),
                    };
                    S::from_f64(f64::from(u8::from(value)))
                })?;
            }
            NativeOp::LogicalConst(op, rhs) => {
                Self::unary(stack, name, |x| {
                    let value = match (op, rhs) {
                        (LogicalOp::And, true) | (LogicalOp::Or, false) => truthy(x),
                        (LogicalOp::And, false) => false,
                        _ => true,
                    };
                    S::from_f64(f64::from(u8::from(value)))
                })?;
            }
            NativeOp::IfElse => {
                if stack.len() < 3 {
                    return Err(PostfixRefusal::Malformed(name));
                }
                let otherwise = stack.pop().expect("depth checked");
                let then = stack.pop().expect("depth checked");
                let condition = stack.pop().expect("depth checked");
                stack.push(if truthy(condition) { then } else { otherwise });
            }
            NativeOp::Extremum(op) => {
                Self::binary(stack, name, |left, right| extremum(op, left, right))?;
            }
            NativeOp::ExtremumConst(op, value) => {
                Self::unary(stack, name, |x| extremum(op, x, S::from_f64(value)))?;
            }
            NativeOp::ExtremumConstLhs(op, value) => {
                Self::unary(stack, name, |x| extremum(op, S::from_f64(value), x))?;
            }
            NativeOp::UnaryMath(op) => Self::unary(stack, name, |x| unary_math(op, x))?,
            NativeOp::BinaryMath(op) => {
                Self::binary(stack, name, |left, right| binary_math(op, left, right))?;
            }
            NativeOp::IntegerCast => {
                let value = Self::top(stack, name)?;
                let integer =
                    real_to_integer(value.real()).map_err(|_| PostfixRefusal::RuntimeError(name))?;
                *stack.last_mut().ok_or(PostfixRefusal::Malformed(name))? =
                    S::from_f64(f64::from(integer));
            }
            NativeOp::IntegerBinary(op) => {
                let (left, right) = Self::two(stack, name)?;
                let value = integer_binary(runtime_integer_operation(op), left.real(), right.real())
                    .map_err(|_| PostfixRefusal::RuntimeError(name))?;
                stack.push(S::from_f64(value));
            }
            NativeOp::IntegerShiftConst(op, count) => {
                let value = Self::top(stack, name)?;
                let shifted = integer_binary(
                    runtime_integer_operation(op),
                    value.real(),
                    f64::from(count),
                )
                .map_err(|_| PostfixRefusal::RuntimeError(name))?;
                *stack.last_mut().ok_or(PostfixRefusal::Malformed(name))? = S::from_f64(shifted);
            }
            NativeOp::IntegerBinaryConst(op, literal) => {
                let value = Self::top(stack, name)?;
                let literal = i32::try_from(literal)
                    .map_err(|_| PostfixRefusal::RuntimeError(name))?;
                let combined = integer_binary(
                    runtime_integer_operation(op),
                    value.real(),
                    f64::from(literal),
                )
                .map_err(|_| PostfixRefusal::RuntimeError(name))?;
                *stack.last_mut().ok_or(PostfixRefusal::Malformed(name))? = S::from_f64(combined);
            }
            // With `integration_active` clear — which is what a static
            // operating point is — the runtime's own helpers return these
            // exactly. `ddt` and its Jacobian answer zero; `idt` answers its
            // initial condition and drops the integrand; `idtmod` would have
            // to wrap that condition, which needs the modulus rules the
            // runtime keeps, so it is refused rather than approximated.
            NativeOp::DdtState(_) => Self::unary(stack, name, |_| S::from_f64(0.0))?,
            NativeOp::DdtJacobian | NativeOp::IdtJacobian => {
                Self::unary(stack, name, |_| S::from_f64(0.0))?;
            }
            NativeOp::IdtState(_) => Self::binary(stack, name, |_, condition| condition)?,
            // Both noise magnitudes are the constant zero outside a noise
            // analysis, which is what the backend emits rather than a call.
            NativeOp::WhiteNoise => Self::unary(stack, name, |_| S::from_f64(0.0))?,
            NativeOp::FlickerNoise => Self::binary(stack, name, |_, _| S::from_f64(0.0))?,
            _ => return Err(PostfixRefusal::Operation(name)),
        }
        Ok(())
    }

    fn read(&self, values: &[f64], index: usize, name: &'static str) -> Result<f64, PostfixRefusal> {
        values
            .get(index)
            .copied()
            .ok_or(PostfixRefusal::RuntimeError(name))
    }

    fn read_flag(
        &self,
        flags: &[u8],
        index: usize,
        name: &'static str,
    ) -> Result<f64, PostfixRefusal> {
        flags
            .get(index)
            .map(|flag| f64::from(u8::from(*flag != 0)))
            .ok_or(PostfixRefusal::RuntimeError(name))
    }

    fn top(stack: &[S], name: &'static str) -> Result<S, PostfixRefusal> {
        stack
            .last()
            .copied()
            .ok_or(PostfixRefusal::Malformed(name))
    }

    fn two(stack: &mut Vec<S>, name: &'static str) -> Result<(S, S), PostfixRefusal> {
        if stack.len() < 2 {
            return Err(PostfixRefusal::Malformed(name));
        }
        let right = stack.pop().expect("depth checked");
        let left = stack.pop().expect("depth checked");
        Ok((left, right))
    }

    fn unary(
        stack: &mut [S],
        name: &'static str,
        apply: impl FnOnce(S) -> S,
    ) -> Result<(), PostfixRefusal> {
        let slot = stack.last_mut().ok_or(PostfixRefusal::Malformed(name))?;
        *slot = apply(*slot);
        Ok(())
    }

    fn binary(
        stack: &mut Vec<S>,
        name: &'static str,
        apply: impl FnOnce(S, S) -> S,
    ) -> Result<(), PostfixRefusal> {
        let (left, right) = Self::two(stack, name)?;
        stack.push(apply(left, right));
        Ok(())
    }
}

/// Verilog-A truthiness: anything but an exact zero, NaN included.
fn truthy<S: CfgScalar>(value: S) -> bool {
    value.real() != 0.0
}

fn compare(op: CompareOp, left: f64, right: f64) -> bool {
    match op {
        CompareOp::Gt => left > right,
        CompareOp::Lt => left < right,
        CompareOp::Ge => left >= right,
        CompareOp::Le => left <= right,
        CompareOp::Eq => left == right,
        CompareOp::Ne => left != right,
    }
}

/// `min`/`max` as the backend selects them.
///
/// `MINSD`/`MAXSD` return the *right* operand whenever either is a NaN, and
/// the emitter then puts the left one back when the right was the NaN or when
/// both operands are zero. Two consequences the ordinary reading misses: a NaN
/// on the left wins, and the result is always one of the two operands rather
/// than a blend — which is why a masked selection commits no rounding at all.
fn extremum<S: CfgScalar>(op: ExtremumOp, left: S, right: S) -> S {
    let (a, b) = (left.real(), right.real());
    if a.is_nan() {
        return right;
    }
    if b.is_nan() {
        return left;
    }
    if a == 0.0 && b == 0.0 {
        return left;
    }
    let takes_left = match op {
        ExtremumOp::Min => a < b,
        ExtremumOp::Max => a > b,
    };
    if takes_left { left } else { right }
}

fn unary_math<S: CfgScalar>(op: UnaryMathOp, value: S) -> S {
    match op {
        UnaryMathOp::Exp => value.exp(),
        UnaryMathOp::Log => value.ln(),
        UnaryMathOp::Log10 => value.log10(),
        UnaryMathOp::Sin => value.sin(),
        UnaryMathOp::Cos => value.cos(),
        UnaryMathOp::Tan => value.tan(),
        UnaryMathOp::Sinh => value.sinh(),
        UnaryMathOp::Cosh => value.cosh(),
        UnaryMathOp::Tanh => value.tanh(),
        UnaryMathOp::Asinh => value.asinh(),
        UnaryMathOp::Acosh => value.acosh(),
        UnaryMathOp::Atanh => value.atanh(),
        UnaryMathOp::Asin => value.asin(),
        UnaryMathOp::Acos => value.acos(),
        UnaryMathOp::Atan => value.atan(),
        UnaryMathOp::Floor => value.floor(),
        UnaryMathOp::Ceil => value.ceil(),
        UnaryMathOp::Limexp => value.limexp(),
        UnaryMathOp::LimitedExp => value.limited_exp(),
    }
}

fn binary_math<S: CfgScalar>(op: BinaryMathOp, left: S, right: S) -> S {
    match op {
        BinaryMathOp::Pow => left.powf(right),
        BinaryMathOp::Atan2 => left.atan2(right),
        BinaryMathOp::Hypot => left.hypot(right),
        BinaryMathOp::Mod => left.rem(right),
    }
}

#[cfg(test)]
mod tests {
    use super::{MirPoint, PostfixRefusal, PostfixWalk};
    use crate::jit::expr::{
        CompareOp, ExtremumOp, LogicalOp, NativeOp, NativeProgram, UnaryMathOp, VoltageNode,
    };
    use crate::native::double_double::DoubleDouble;

    const PARAMETERS: [f64; 3] = [2.5, -0.75, 1.0e-9];
    const TERMINALS: [f64; 2] = [0.7, 0.1];
    const INTERNALS: [f64; 1] = [0.31];

    fn point() -> MirPoint<'static> {
        MirPoint {
            parameters: &PARAMETERS,
            parameter_given: &[1, 0, 1],
            port_connected: &[1, 1],
            terminal_voltages: &TERMINALS,
            internal_voltages: &INTERNALS,
            branch_unknowns: &[1.5e-3],
            currents: &[3.0e-6],
            branch_currents: &[7.0e-6],
            temperature: 300.15,
            time: 1.0e-9,
            multiplicity: 1.0,
            analysis: 2,
            initial_step: true,
            final_step: false,
        }
    }

    fn program(ops: Vec<NativeOp>, depth: usize) -> NativeProgram {
        NativeProgram::from_ops_for_test(ops, depth, Vec::new(), Vec::new())
    }

    fn run_f64(ops: Vec<NativeOp>, depth: usize) -> Result<f64, PostfixRefusal> {
        let point = point();
        let mut walk: PostfixWalk<'_, f64> = PostfixWalk::new(&point, 4, 2);
        walk.run(&program(ops, depth))
    }

    /// The walker computes what the operating point holds, through every load
    /// the corpus reaches.
    #[test]
    fn every_load_reads_the_storage_the_backend_addresses() {
        assert_eq!(run_f64(vec![NativeOp::LoadParam(0)], 1), Ok(2.5));
        assert_eq!(run_f64(vec![NativeOp::LoadParamGiven(1)], 1), Ok(0.0));
        assert_eq!(run_f64(vec![NativeOp::LoadPortConnected(1)], 1), Ok(1.0));
        assert_eq!(
            run_f64(
                vec![NativeOp::LoadVoltage {
                    pos: VoltageNode::Terminal(0),
                    neg: VoltageNode::Terminal(1),
                }],
                1
            ),
            Ok(0.6)
        );
        assert_eq!(
            run_f64(
                vec![NativeOp::LoadVoltage {
                    pos: VoltageNode::Internal(0),
                    neg: VoltageNode::Ground,
                }],
                1
            ),
            Ok(0.31)
        );
        // The two current loads address the fields whose names are the other
        // way round from theirs.
        assert_eq!(run_f64(vec![NativeOp::LoadCurrent(0)], 1), Ok(7.0e-6));
        assert_eq!(run_f64(vec![NativeOp::LoadPriorCurrent(0)], 1), Ok(3.0e-6));
        assert_eq!(run_f64(vec![NativeOp::LoadBranchUnknown(0)], 1), Ok(1.5e-3));
        assert_eq!(run_f64(vec![NativeOp::LoadTemperature], 1), Ok(300.15));
        assert_eq!(run_f64(vec![NativeOp::LoadTime], 1), Ok(1.0e-9));
        assert_eq!(run_f64(vec![NativeOp::LoadMfactor], 1), Ok(1.0));
        assert_eq!(
            run_f64(vec![NativeOp::LoadThermalVoltage], 1),
            Ok(300.15 * (1.380_649e-23 / 1.602_176_634e-19))
        );
        // `tran` is analysis 2, `static` is 5, `smallsignal` is 6, and the
        // initial step is 7.
        assert_eq!(run_f64(vec![NativeOp::Analysis(2)], 1), Ok(1.0));
        assert_eq!(run_f64(vec![NativeOp::Analysis(0)], 1), Ok(0.0));
        assert_eq!(run_f64(vec![NativeOp::Analysis(5)], 1), Ok(0.0));
        assert_eq!(run_f64(vec![NativeOp::Analysis(7)], 1), Ok(1.0));
        assert_eq!(run_f64(vec![NativeOp::Analysis(8)], 1), Ok(0.0));
        assert_eq!(run_f64(vec![NativeOp::Analysis(99)], 1), Ok(0.0));
        // An index the point does not have is the runtime error the compiled
        // route reports there, not a zero the walk invented.
        assert_eq!(
            run_f64(vec![NativeOp::LoadParam(9)], 1),
            Err(PostfixRefusal::RuntimeError("LoadParam"))
        );
    }

    /// Truthiness is "not exactly zero", and every operation that reads a
    /// condition reads it that way — NaN included.
    #[test]
    fn a_nan_condition_is_true() {
        let select = |condition: f64| {
            run_f64(
                vec![
                    NativeOp::Const(condition),
                    NativeOp::Const(11.0),
                    NativeOp::Const(22.0),
                    NativeOp::IfElse,
                ],
                3,
            )
        };
        assert_eq!(select(1.0), Ok(11.0));
        assert_eq!(select(0.0), Ok(22.0));
        assert_eq!(select(-0.0), Ok(22.0));
        assert_eq!(select(f64::NAN), Ok(11.0));
        assert_eq!(select(-3.0), Ok(11.0));

        assert_eq!(
            run_f64(
                vec![
                    NativeOp::Const(f64::NAN),
                    NativeOp::Logical(LogicalOp::Not)
                ],
                1
            ),
            Ok(0.0)
        );
        assert_eq!(
            run_f64(
                vec![
                    NativeOp::Const(f64::NAN),
                    NativeOp::Const(0.0),
                    NativeOp::Logical(LogicalOp::And),
                ],
                2
            ),
            Ok(0.0)
        );
        assert_eq!(
            run_f64(
                vec![
                    NativeOp::Const(f64::NAN),
                    NativeOp::LogicalConst(LogicalOp::And, true),
                ],
                1
            ),
            Ok(1.0)
        );
    }

    /// Comparisons are ordered, except `!=`, which is the one the backend
    /// makes true on an unordered pair.
    #[test]
    fn the_comparisons_are_ordered_except_the_inequality() {
        let compare = |op, left: f64, right: f64| {
            run_f64(
                vec![
                    NativeOp::Const(left),
                    NativeOp::Const(right),
                    NativeOp::Compare(op),
                ],
                2,
            )
        };
        for op in [
            CompareOp::Gt,
            CompareOp::Lt,
            CompareOp::Ge,
            CompareOp::Le,
            CompareOp::Eq,
        ] {
            assert_eq!(compare(op, f64::NAN, 1.0), Ok(0.0), "{op:?}");
            assert_eq!(compare(op, 1.0, f64::NAN), Ok(0.0), "{op:?}");
        }
        assert_eq!(compare(CompareOp::Ne, f64::NAN, 1.0), Ok(1.0));
        assert_eq!(compare(CompareOp::Gt, 2.0, 1.0), Ok(1.0));
        assert_eq!(compare(CompareOp::Le, 2.0, 2.0), Ok(1.0));
        assert_eq!(
            run_f64(
                vec![
                    NativeOp::Const(f64::NAN),
                    NativeOp::CompareConst(CompareOp::Lt, 1.0)
                ],
                1
            ),
            Ok(0.0)
        );
    }

    /// `min` and `max` select an operand rather than blending two, and the
    /// left one wins a NaN and a signed zero.
    #[test]
    fn the_extrema_select_the_operand_the_backend_selects() {
        let extremum = |op, left: f64, right: f64| {
            run_f64(
                vec![
                    NativeOp::Const(left),
                    NativeOp::Const(right),
                    NativeOp::Extremum(op),
                ],
                2,
            )
        };
        assert_eq!(extremum(ExtremumOp::Max, 3.0, 1.0), Ok(3.0));
        assert_eq!(extremum(ExtremumOp::Min, 3.0, 1.0), Ok(1.0));
        // A NaN on either side loses to the number.
        assert_eq!(extremum(ExtremumOp::Max, f64::NAN, 1.0), Ok(1.0));
        assert_eq!(extremum(ExtremumOp::Max, 1.0, f64::NAN), Ok(1.0));
        // Two zeros return the left one, sign and all.
        assert!(
            extremum(ExtremumOp::Min, 0.0, -0.0)
                .expect("a value")
                .is_sign_positive()
        );
        assert_eq!(
            run_f64(
                vec![
                    NativeOp::Const(5.0),
                    NativeOp::ExtremumConstLhs(ExtremumOp::Min, 2.0)
                ],
                1
            ),
            Ok(2.0)
        );
    }

    /// The reversed-literal operations put the literal on the left, which is
    /// the whole reason they exist as separate operations.
    #[test]
    fn the_reversed_literal_operations_do_not_commute() {
        assert_eq!(
            run_f64(vec![NativeOp::Const(4.0), NativeOp::SubFromConst(10.0)], 1),
            Ok(6.0)
        );
        assert_eq!(
            run_f64(vec![NativeOp::Const(4.0), NativeOp::SubConst(10.0)], 1),
            Ok(-6.0)
        );
        assert_eq!(
            run_f64(vec![NativeOp::Const(4.0), NativeOp::DivFromConst(10.0)], 1),
            Ok(2.5)
        );
        assert_eq!(
            run_f64(vec![NativeOp::Const(4.0), NativeOp::DivConst(10.0)], 1),
            Ok(0.4)
        );
    }

    /// The integrators are exact constants at a static operating point, which
    /// is what the runtime's own helpers return with `integration_active`
    /// clear — reproduced rather than guessed.
    #[test]
    fn the_integrators_answer_what_the_runtime_answers_when_integration_is_off() {
        assert_eq!(
            run_f64(vec![NativeOp::Const(9.0), NativeOp::DdtState(0)], 1),
            Ok(0.0)
        );
        assert_eq!(
            run_f64(vec![NativeOp::Const(9.0), NativeOp::DdtJacobian], 1),
            Ok(0.0)
        );
        assert_eq!(
            run_f64(vec![NativeOp::Const(9.0), NativeOp::IdtJacobian], 1),
            Ok(0.0)
        );
        // `idt` returns its initial condition and drops the integrand.
        assert_eq!(
            run_f64(
                vec![
                    NativeOp::Const(9.0),
                    NativeOp::Const(4.5),
                    NativeOp::IdtState(0)
                ],
                2
            ),
            Ok(4.5)
        );
        assert_eq!(
            run_f64(vec![NativeOp::Const(9.0), NativeOp::WhiteNoise], 1),
            Ok(0.0)
        );
    }

    /// An operation the walker does not implement is named rather than
    /// approximated, and the name reaches the caller.
    #[test]
    fn an_unimplemented_operation_is_refused_by_name() {
        let point = point();
        let mut walk: PostfixWalk<'_, f64> = PostfixWalk::new(&point, 4, 2);
        let refused = walk.run(&program(
            vec![NativeOp::Const(1.0), NativeOp::LaplaceState(0)],
            1,
        ));
        assert_eq!(refused, Err(PostfixRefusal::Operation("LaplaceState")));
        assert_eq!(walk.refusals().collect::<Vec<_>>(), vec!["LaplaceState"]);
    }

    /// The prelude slot is a store whose value continues, which is the only
    /// operation in the vocabulary that exists for a side effect.
    #[test]
    fn a_prelude_store_publishes_and_yields() {
        let point = point();
        let mut walk: PostfixWalk<'_, f64> = PostfixWalk::new(&point, 4, 2);
        let stored = walk.run(&program(
            vec![NativeOp::Const(6.25), NativeOp::StorePreludeSlot(1)],
            1,
        ));
        assert_eq!(stored, Ok(6.25), "the store is an identity on its operand");
        assert_eq!(walk.run(&program(vec![NativeOp::LoadPreludeSlot(1)], 1)), Ok(6.25));
    }

    /// The assignment pass fills the variable array the entries then read,
    /// including through a dynamic index.
    #[test]
    fn the_assignment_pass_fills_what_the_entries_read() {
        use crate::jit::assignment::NativeAssignment;
        let point = point();
        let mut walk: PostfixWalk<'_, f64> = PostfixWalk::new(&point, 4, 2);
        walk.fill_variables(&[
            NativeAssignment::Direct {
                var_index: 0,
                program: program(vec![NativeOp::LoadParam(0), NativeOp::MulConst(4.0)], 1),
            },
            NativeAssignment::Direct {
                var_index: 1,
                program: program(vec![NativeOp::LoadVariable(0), NativeOp::AddConst(1.0)], 1),
            },
            NativeAssignment::Indexed {
                base: 2,
                len: 2,
                lower: 0,
                index: program(vec![NativeOp::Const(1.0)], 1),
                value: program(vec![NativeOp::Const(-8.0)], 1),
            },
        ]);
        assert_eq!(walk.run(&program(vec![NativeOp::LoadVariable(0)], 1)), Ok(10.0));
        assert_eq!(walk.run(&program(vec![NativeOp::LoadVariable(1)], 1)), Ok(11.0));
        assert_eq!(walk.run(&program(vec![NativeOp::LoadVariable(3)], 1)), Ok(-8.0));
        assert_eq!(
            walk.run(&program(
                vec![
                    NativeOp::Const(1.0),
                    NativeOp::LoadVariableDyn {
                        base: 2,
                        len: 2,
                        lower: 0
                    }
                ],
                1
            )),
            Ok(-8.0)
        );
    }

    /// The same program walked in `f64` and in double-double agrees where the
    /// arithmetic is exact and differs by the `f64` rounding where it is not.
    ///
    /// This is the walker's whole purpose stated as a test: one stream, two
    /// precisions, and the difference is a measurement.
    #[test]
    fn the_two_precisions_differ_by_exactly_the_f64_rounding() {
        let ops = vec![
            NativeOp::Const(1.0),
            NativeOp::Const(3.0),
            NativeOp::Div,
            NativeOp::Const(3.0),
            NativeOp::Mul,
            NativeOp::SubConst(1.0),
        ];
        let point = point();
        let mut narrow: PostfixWalk<'_, f64> = PostfixWalk::new(&point, 0, 0);
        let mut wide: PostfixWalk<'_, DoubleDouble> = PostfixWalk::new(&point, 0, 0);
        let narrow = narrow.run(&program(ops.clone(), 2)).expect("f64 result");
        let wide = wide.run(&program(ops, 2)).expect("double-double result");
        // `(1/3)*3 - 1` is exactly zero in `f64` — the two roundings cancel —
        // and the double-double walk says the same, because it is the *same*
        // rounded third that was multiplied back.
        assert_eq!(narrow, 0.0);
        assert!(wide.to_f64().abs() < 1.0e-16, "{:e}", wide.to_f64());

        // Where the two do differ, the difference is the f64 rounding: a sum
        // that drops its small term.
        let dropping = vec![
            NativeOp::Const(1.0),
            NativeOp::Const(1.0e-20),
            NativeOp::Add,
        ];
        let mut narrow: PostfixWalk<'_, f64> = PostfixWalk::new(&point, 0, 0);
        let mut wide: PostfixWalk<'_, DoubleDouble> = PostfixWalk::new(&point, 0, 0);
        let narrow = narrow.run(&program(dropping.clone(), 2)).expect("f64");
        let wide = wide.run(&program(dropping, 2)).expect("double-double");
        assert_eq!(narrow, 1.0);
        assert_eq!(wide.relative_distance_to(narrow), 1.0e-20);
    }

    /// Every unary library function reaches the scalar's own rule, so a walk
    /// in double-double takes the double-double transcendental rather than
    /// the `f64` one lifted.
    #[test]
    fn the_library_functions_reach_the_scalars_own_rule() {
        let point = point();
        for (op, argument) in [
            (UnaryMathOp::Exp, 0.7),
            (UnaryMathOp::Log, 3.3),
            (UnaryMathOp::Log10, 3.3),
            (UnaryMathOp::Sin, 0.7),
            (UnaryMathOp::Cos, 0.7),
            (UnaryMathOp::Tan, 0.7),
            (UnaryMathOp::Sinh, 0.7),
            (UnaryMathOp::Cosh, 0.7),
            (UnaryMathOp::Tanh, 0.7),
            (UnaryMathOp::Asinh, 0.7),
            (UnaryMathOp::Acosh, 3.3),
            (UnaryMathOp::Atanh, 0.7),
            (UnaryMathOp::Asin, 0.7),
            (UnaryMathOp::Acos, 0.7),
            (UnaryMathOp::Atan, 0.7),
            (UnaryMathOp::Floor, 3.3),
            (UnaryMathOp::Ceil, 3.3),
            (UnaryMathOp::Limexp, 0.7),
            (UnaryMathOp::LimitedExp, 0.7),
        ] {
            let ops = vec![NativeOp::Const(argument), NativeOp::UnaryMath(op)];
            let mut narrow: PostfixWalk<'_, f64> = PostfixWalk::new(&point, 0, 0);
            let mut wide: PostfixWalk<'_, DoubleDouble> = PostfixWalk::new(&point, 0, 0);
            let narrow = narrow.run(&program(ops.clone(), 1)).expect("f64");
            let wide = wide.run(&program(ops, 1)).expect("double-double");
            assert!(
                wide.relative_distance_to(narrow) < 4.0 * f64::EPSILON,
                "{op:?}({argument}): {narrow:e} vs {:e}",
                wide.to_f64()
            );
        }
    }
}
