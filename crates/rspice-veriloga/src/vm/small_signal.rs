//! Complex frequency-domain evaluation of differentiated Verilog-A bytecode.
//!
//! The ordinary VM deliberately stores real scalars because DC and transient
//! Newton systems are real.  AC and noise linearization are different: an
//! analog-operator derivative is a complex transfer action.  Keeping the two
//! components together is required for nested operators; evaluating separate
//! scalar "real" and "imaginary" passes loses the cross products in, for
//! example, two cascaded Laplace filters.

use super::{VmContext, VmError, idtmod_wrapped_candidate};
use crate::array_index::{ArrayIndexError, checked_array_slot, saturated_array_upper};
use crate::codegen::{AssignmentStep, BytecodeProgram, Instruction, ZiRuntimeLayout};
use crate::integer_runtime::{IntegerBinaryOperation, integer_binary};
use crate::timing_contract::{NormalizedSlewRates, normalize_slew_rates};
use num_complex::Complex64;

const MAX_RUNTIME_LOOP_ITERATIONS: usize = 1_000_000;

/// Read-only complex evaluator used only for frequency-domain Jacobians.
///
/// A private complex variable image replays the assignment stream so
/// forward-mode derivative shadows retain the phase introduced by dynamic
/// operators.  The real runtime context has already completed its normal
/// native/VM/WASM operating-point evaluation and is never mutated here.
pub(crate) struct SmallSignalVm<'a> {
    context: &'a VmContext,
    variables: Vec<Complex64>,
    stack: Vec<Complex64>,
    frequency_hz: f64,
    omega: f64,
}

impl<'a> SmallSignalVm<'a> {
    #[cfg(test)]
    pub(crate) fn new(context: &'a VmContext, frequency_hz: f64) -> Result<Self, VmError> {
        Self::with_variable_seed(context, frequency_hz, &context.variables)
    }

    pub(crate) fn with_variable_seed(
        context: &'a VmContext,
        frequency_hz: f64,
        variable_seed: &[f64],
    ) -> Result<Self, VmError> {
        if !frequency_hz.is_finite() || frequency_hz < 0.0 {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "small-signal frequency must be finite and nonnegative, got {frequency_hz}"
            )));
        }
        if !matches!(context.analysis_type, 1 | 3) {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "complex small-signal evaluation requires AC or noise analysis, got analysis type {}",
                context.analysis_type
            )));
        }
        let omega = std::f64::consts::TAU * frequency_hz;
        if !omega.is_finite() {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "small-signal angular frequency overflows at {frequency_hz} Hz"
            )));
        }
        Ok(Self {
            context,
            variables: variable_seed
                .iter()
                .copied()
                .map(|value| Complex64::new(value, 0.0))
                .collect(),
            stack: Vec::with_capacity(32),
            frequency_hz,
            omega,
        })
    }

    pub(crate) fn execute_assignments(&mut self, steps: &[AssignmentStep]) -> Result<(), VmError> {
        self.execute_assignment_steps(steps)
    }

    pub(crate) fn execute(&mut self, program: &BytecodeProgram) -> Result<Complex64, VmError> {
        self.stack.clear();
        for instruction in &program.instructions {
            self.execute_instruction(instruction)?;
        }
        let result = self
            .stack
            .pop()
            .ok_or(VmError::StackUnderflow("No small-signal result on stack"))?;
        if !self.stack.is_empty() {
            return Err(VmError::InvalidInstruction(
                "small-signal bytecode left extra values on the stack",
            ));
        }
        if !result.re.is_finite() || !result.im.is_finite() {
            return Err(VmError::InvalidNumericResult(format!(
                "small-signal bytecode produced non-finite result {}+j{}",
                result.re, result.im
            )));
        }
        Ok(result)
    }

    fn execute_assignment_steps(&mut self, steps: &[AssignmentStep]) -> Result<(), VmError> {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    let value = self.execute(&assignment.program)?;
                    let slot = self.variables.get_mut(assignment.var_index).ok_or(
                        VmError::InvalidInstruction(
                            "small-signal assignment target is outside variable storage",
                        ),
                    )?;
                    *slot = value;
                }
                AssignmentStep::AssignIndexed {
                    base,
                    len,
                    lower,
                    index,
                    value,
                } => {
                    let index_value = self.execute(index)?;
                    let raw = self.real_value(index_value, "array index")?;
                    let slot = Self::array_slot(raw, *base, *len, *lower)?;
                    let value = self.execute(value)?;
                    let target = self.variables.get_mut(slot).ok_or(
                        VmError::InvalidInstruction(
                            "small-signal indexed assignment target is outside variable storage",
                        ),
                    )?;
                    *target = value;
                }
                AssignmentStep::Loop { condition, body } => {
                    let mut iterations = 0usize;
                    loop {
                        let condition_value = self.execute(condition)?;
                        let active = self.real_value(condition_value, "runtime-loop condition")?;
                        if active == 0.0 {
                            break;
                        }
                        self.execute_assignment_steps(body)?;
                        iterations += 1;
                        if iterations >= MAX_RUNTIME_LOOP_ITERATIONS {
                            return Err(VmError::InvalidInstruction(
                                "small-signal runtime loop iteration limit exceeded",
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn array_slot(raw: f64, base: usize, len: usize, lower: i64) -> Result<usize, VmError> {
        checked_array_slot(raw, base, len, lower).map_err(|error| match error {
            ArrayIndexError::NonFinite { raw } => VmError::InvalidNumericResult(format!(
                "runtime array index must be finite, got {raw}"
            )),
            ArrayIndexError::RoundedOutOfRange { raw } => VmError::InvalidNumericResult(format!(
                "runtime array index {raw} rounds outside the signed 64-bit index range"
            )),
            ArrayIndexError::OutOfBounds { index } => VmError::IndexOutOfBounds {
                index,
                lower,
                upper: saturated_array_upper(lower, len),
            },
            ArrayIndexError::Empty => {
                VmError::InvalidInstruction("zero-length dynamic variable range")
            }
            ArrayIndexError::SlotOverflow => {
                VmError::InvalidInstruction("dynamic variable slot arithmetic overflow")
            }
        })
    }

    #[inline]
    fn pop(&mut self, operation: &'static str) -> Result<Complex64, VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow(operation))
    }

    fn real_value(&self, value: Complex64, label: &str) -> Result<f64, VmError> {
        if value.im != 0.0 {
            return Err(VmError::InvalidNumericResult(format!(
                "{label} is not a real operating-point value during small-signal evaluation: {}+j{}",
                value.re, value.im
            )));
        }
        Ok(value.re)
    }

    fn pop_real(&mut self, operation: &'static str) -> Result<f64, VmError> {
        let value = self.pop(operation)?;
        self.real_value(value, operation)
    }

    fn unary(
        &mut self,
        operation: &'static str,
        f: impl FnOnce(Complex64) -> Complex64,
    ) -> Result<(), VmError> {
        let value = self.pop(operation)?;
        self.stack.push(f(value));
        Ok(())
    }

    fn binary(
        &mut self,
        operation: &'static str,
        f: impl FnOnce(Complex64, Complex64) -> Complex64,
    ) -> Result<(), VmError> {
        let right = self.pop(operation)?;
        let left = self.pop(operation)?;
        self.stack.push(f(left, right));
        Ok(())
    }

    fn unary_real(
        &mut self,
        operation: &'static str,
        f: impl FnOnce(f64) -> f64,
    ) -> Result<(), VmError> {
        let value = self.pop_real(operation)?;
        self.stack.push(Complex64::new(f(value), 0.0));
        Ok(())
    }

    fn binary_real(
        &mut self,
        operation: &'static str,
        f: impl FnOnce(f64, f64) -> f64,
    ) -> Result<(), VmError> {
        let right = self.pop_real(operation)?;
        let left = self.pop_real(operation)?;
        self.stack.push(Complex64::new(f(left, right), 0.0));
        Ok(())
    }

    fn integer_binary(
        &mut self,
        operation: IntegerBinaryOperation,
        label: &'static str,
    ) -> Result<(), VmError> {
        let right = self.pop_real(label)?;
        let left = self.pop_real(label)?;
        let value = integer_binary(operation, left, right)
            .map_err(|error| VmError::InvalidNumericResult(format!("{label} failed: {error}")))?;
        self.stack.push(Complex64::new(value, 0.0));
        Ok(())
    }

    fn execute_zi(&mut self, layout: ZiRuntimeLayout, derivative: bool) -> Result<(), VmError> {
        let operand_count = layout.validate_operand_budget().map_err(|error| {
            VmError::InvalidNumericResult(format!("Zi runtime layout rejected: {error}"))
        })?;
        if self.stack.len() < operand_count {
            return Err(VmError::StackUnderflow(if derivative {
                "ZiStateDerivative"
            } else {
                "ZiState"
            }));
        }
        let start = self.stack.len() - operand_count;
        let operands = &self.stack[start..];
        for (index, operand) in operands.iter().enumerate() {
            if index != operands.len() - 2 && operand.im != 0.0 {
                return Err(VmError::InvalidNumericResult(format!(
                    "Zi definition/timing operand {index} is complex during small-signal evaluation"
                )));
            }
        }
        let action = operands[operands.len() - 2];
        let transition = operands[operands.len() - 1].re;
        if !transition.is_finite() || transition < 0.0 {
            return Err(VmError::InvalidNumericResult(format!(
                "Zi transition time must be finite and nonnegative, got {transition}"
            )));
        }
        let filter = self
            .context
            .zi_filters
            .get(layout.filter_id)
            .ok_or(VmError::InvalidInstruction("missing zi filter"))?;
        if !filter.definition_is_frozen() {
            return Err(VmError::InvalidRuntimeConfiguration(format!(
                "Zi filter {} was not frozen by the operating-point evaluation",
                layout.filter_id
            )));
        }
        let (real, imag) = if derivative {
            filter.frequency_response_rectangular(self.frequency_hz)
        } else {
            filter.dc_gain().map(|gain| (gain, 0.0))
        }
        .map_err(|error| {
            VmError::InvalidNumericResult(format!("zi filter {}: {error}", layout.filter_id))
        })?;
        self.stack.truncate(start);
        self.stack.push(Complex64::new(real, imag) * action);
        Ok(())
    }

    fn execute_absdelay(
        &mut self,
        buffer_id: usize,
        with_maximum: bool,
        derivative: bool,
    ) -> Result<(), VmError> {
        let max_delay = if with_maximum {
            Some(self.pop_real("absdelay maxdelay")?)
        } else {
            None
        };
        let _delay_derivative = derivative
            .then(|| self.pop("absdelay delay derivative"))
            .transpose()?;
        let delay = self.pop_real("absdelay delay")?;
        let input_derivative = derivative
            .then(|| self.pop("absdelay input derivative"))
            .transpose()?;
        let input = self.pop("absdelay input")?;
        let input_real = self.real_value(input, "absdelay operating-point input")?;
        let buffer = self.context.delay_buffers.get(buffer_id).ok_or_else(|| {
            VmError::InvalidRuntimeConfiguration(format!(
                "absdelay buffer {buffer_id} is not preallocated"
            ))
        })?;
        let effective_delay = buffer
            .small_signal_delay(self.context.time, input_real, delay, max_delay)
            .map_err(|error| VmError::InvalidNumericResult(format!("absdelay: {error}")))?;
        let result = if derivative {
            let phase = -self.omega * effective_delay;
            if !phase.is_finite() {
                return Err(VmError::InvalidNumericResult(format!(
                    "absdelay phase overflows at {} Hz and delay {effective_delay}",
                    self.frequency_hz
                )));
            }
            Complex64::from_polar(1.0, phase)
                * input_derivative.expect("derivative operand was decoded")
        } else {
            Complex64::new(input_real, 0.0)
        };
        self.stack.push(result);
        Ok(())
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        match instruction {
            Instruction::PushConst(value) => self.stack.push(Complex64::new(*value, 0.0)),
            Instruction::PushParam(index) => {
                let value = self
                    .context
                    .parameters
                    .get(*index)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing parameter slot"))?;
                self.stack.push(Complex64::new(value, 0.0));
            }
            Instruction::PushParamGiven(index) => {
                let value = self
                    .context
                    .param_given
                    .get(*index)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing parameter-given slot"))?;
                self.stack.push(Complex64::new(f64::from(value != 0), 0.0));
            }
            Instruction::PushBranchCurrent(index) => {
                let value = self
                    .context
                    .branch_current_values
                    .get(*index)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing branch-current slot"))?;
                self.stack.push(Complex64::new(value, 0.0));
            }
            Instruction::PushVoltage(pos, neg) => {
                self.stack
                    .push(Complex64::new(self.context.try_voltage(*pos, *neg)?, 0.0));
            }
            Instruction::PushCurrent(pos, neg) => {
                self.stack
                    .push(Complex64::new(self.context.try_current(*pos, *neg)?, 0.0));
            }
            Instruction::PushInternalVoltage(index) => {
                let value = self
                    .context
                    .internal_voltages
                    .get(*index)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing internal-voltage slot"))?;
                self.stack.push(Complex64::new(value, 0.0));
            }
            Instruction::PushVariable(index) => {
                let value =
                    self.variables
                        .get(*index)
                        .copied()
                        .ok_or(VmError::InvalidInstruction(
                            "missing small-signal variable slot",
                        ))?;
                self.stack.push(value);
            }
            Instruction::PushVariableDyn { base, len, lower } => {
                let raw = self.pop_real("PushVariableDyn")?;
                let slot = Self::array_slot(raw, *base, *len, *lower)?;
                let value =
                    self.variables
                        .get(slot)
                        .copied()
                        .ok_or(VmError::InvalidInstruction(
                            "missing dynamic small-signal variable slot",
                        ))?;
                self.stack.push(value);
            }
            Instruction::PushTemperature => {
                self.stack
                    .push(Complex64::new(self.context.temperature, 0.0));
            }
            Instruction::PushVt => {
                self.stack.push(Complex64::new(self.context.vt(), 0.0));
            }
            Instruction::PushTime => {
                self.stack.push(Complex64::new(self.context.time, 0.0));
            }
            Instruction::PushMfactor => {
                self.stack
                    .push(Complex64::new(self.context.multiplicity, 0.0));
            }
            Instruction::PushPortConnected(terminal) => self.stack.push(Complex64::new(
                f64::from(self.context.port_connected(*terminal)),
                0.0,
            )),
            Instruction::ZiState(layout) => self.execute_zi(*layout, false)?,
            Instruction::ZiStateDerivative(layout) => self.execute_zi(*layout, true)?,

            Instruction::Add => self.binary("Add", |left, right| left + right)?,
            Instruction::Sub => self.binary("Sub", |left, right| left - right)?,
            Instruction::Mul => self.binary("Mul", |left, right| left * right)?,
            Instruction::Div => self.binary("Div", |left, right| left / right)?,
            Instruction::Pow | Instruction::FnPow => self.binary_real("Pow", f64::powf)?,
            Instruction::Mod => self.binary_real("Mod", |left, right| left % right)?,
            Instruction::Shl => self.integer_binary(IntegerBinaryOperation::Shl, "left shift")?,
            Instruction::Shr => self.integer_binary(IntegerBinaryOperation::Shr, "right shift")?,
            Instruction::BitAnd => {
                self.integer_binary(IntegerBinaryOperation::BitAnd, "bitwise and")?
            }
            Instruction::BitOr => {
                self.integer_binary(IntegerBinaryOperation::BitOr, "bitwise or")?
            }
            Instruction::BitXor => {
                self.integer_binary(IntegerBinaryOperation::BitXor, "bitwise xor")?
            }
            Instruction::Neg => self.unary("Neg", |value| -value)?,
            Instruction::Abs => self.unary_real("Abs", f64::abs)?,
            Instruction::Sqrt => self.unary_real("Sqrt", f64::sqrt)?,
            Instruction::Exp => self.unary_real("Exp", f64::exp)?,
            Instruction::Log => self.unary_real("Log", f64::ln)?,
            Instruction::Log10 => self.unary_real("Log10", f64::log10)?,
            Instruction::Sin => self.unary_real("Sin", f64::sin)?,
            Instruction::Cos => self.unary_real("Cos", f64::cos)?,
            Instruction::Tan => self.unary_real("Tan", f64::tan)?,
            Instruction::Sinh => self.unary_real("Sinh", f64::sinh)?,
            Instruction::Cosh => self.unary_real("Cosh", f64::cosh)?,
            Instruction::Tanh => self.unary_real("Tanh", f64::tanh)?,
            Instruction::Min => self.binary_real("Min", f64::min)?,
            Instruction::Max => self.binary_real("Max", f64::max)?,
            Instruction::Limexp => {
                self.unary_real("Limexp", rspice_veriloga_runtime::rspice_limexp)?
            }
            Instruction::LimitedExp => {
                self.unary_real("LimitedExp", rspice_veriloga_runtime::rspice_limited_exp)?
            }
            Instruction::Asin => self.unary_real("Asin", f64::asin)?,
            Instruction::Acos => self.unary_real("Acos", f64::acos)?,
            Instruction::Atan => self.unary_real("Atan", f64::atan)?,
            Instruction::Asinh => self.unary_real("Asinh", f64::asinh)?,
            Instruction::Acosh => self.unary_real("Acosh", f64::acosh)?,
            Instruction::Atanh => self.unary_real("Atanh", f64::atanh)?,
            Instruction::Atan2 => self.binary_real("Atan2", |left, right| left.atan2(right))?,
            Instruction::Floor => self.unary_real("Floor", f64::floor)?,
            Instruction::Ceil => self.unary_real("Ceil", f64::ceil)?,
            Instruction::Gt => self.binary_real("Gt", |left, right| f64::from(left > right))?,
            Instruction::Lt => self.binary_real("Lt", |left, right| f64::from(left < right))?,
            Instruction::Ge => self.binary_real("Ge", |left, right| f64::from(left >= right))?,
            Instruction::Le => self.binary_real("Le", |left, right| f64::from(left <= right))?,
            Instruction::Eq => self.binary_real("Eq", |left, right| f64::from(left == right))?,
            Instruction::Ne => self.binary_real("Ne", |left, right| f64::from(left != right))?,
            Instruction::And => {
                self.binary_real("And", |left, right| f64::from(left != 0.0 && right != 0.0))?
            }
            Instruction::Or => {
                self.binary_real("Or", |left, right| f64::from(left != 0.0 || right != 0.0))?
            }
            Instruction::Not => self.unary_real("Not", |value| f64::from(value == 0.0))?,
            Instruction::IfElse => {
                let else_value = self.pop("IfElse")?;
                let then_value = self.pop("IfElse")?;
                let condition = self.pop_real("IfElse condition")?;
                self.stack.push(if condition != 0.0 {
                    then_value
                } else {
                    else_value
                });
            }

            // Primal state operators are operating-point values inside a
            // differentiated expression.  Their derivative counterparts below
            // carry the actual complex perturbation.
            Instruction::DdtState(_) => {
                let _input = self.pop_real("DdtState")?;
                self.stack.push(Complex64::new(0.0, 0.0));
            }
            Instruction::IdtState(_) => {
                let initial = self.pop_real("IdtState initial condition")?;
                let _input = self.pop_real("IdtState input")?;
                self.stack.push(Complex64::new(initial, 0.0));
            }
            Instruction::IdtModState(_) => {
                let offset = self.pop_real("IdtModState offset")?;
                let modulus = self.pop_real("IdtModState modulus")?;
                let initial = self.pop_real("IdtModState initial condition")?;
                let _input = self.pop_real("IdtModState input")?;
                let (wrapped, _) = idtmod_wrapped_candidate(initial, modulus, offset).map_err(
                    |detail| {
                        VmError::InvalidNumericResult(format!(
                            "idtmod small-signal operating point {detail}: initial={initial}, modulus={modulus}, offset={offset}"
                        ))
                    },
                )?;
                self.stack.push(Complex64::new(wrapped, 0.0));
            }
            Instruction::DdtJacobian => {
                let input = self.pop("DdtJacobian")?;
                self.stack.push(Complex64::new(0.0, self.omega) * input);
            }
            Instruction::IdtJacobian => {
                let input = self.pop("IdtJacobian")?;
                if self.omega == 0.0 {
                    return Err(VmError::InvalidNumericResult(
                        "idt/idtmod small-signal transfer is singular at zero frequency".into(),
                    ));
                }
                self.stack.push(input / Complex64::new(0.0, self.omega));
            }
            Instruction::TableDerivative(table_id) => {
                let input = self.pop_real("TableDerivative")?;
                let table = self
                    .context
                    .lookup_tables
                    .get(*table_id)
                    .ok_or(VmError::InvalidInstruction("missing lookup table"))?;
                self.stack
                    .push(Complex64::new(table.derivative(input), 0.0));
            }
            Instruction::LimitState(_) => {
                let _step = self.pop_real("LimitState step")?;
                let input = self.pop("LimitState input")?;
                self.stack.push(input);
            }
            Instruction::CanonicalLimitState(_) => {
                // Canonical named limiting is inactive in small-signal
                // analysis, so its metadata opcode is the identity here.
            }
            Instruction::TableLookup(table_id) => {
                let input = self.pop_real("TableLookup")?;
                let table = self
                    .context
                    .lookup_tables
                    .get(*table_id)
                    .ok_or(VmError::InvalidInstruction("missing lookup table"))?;
                self.stack
                    .push(Complex64::new(table.interpolate(input), 0.0));
            }
            Instruction::AbsDelayState(buffer_id) => {
                self.execute_absdelay(*buffer_id, false, false)?
            }
            Instruction::AbsDelayStateMax(buffer_id) => {
                self.execute_absdelay(*buffer_id, true, false)?
            }
            Instruction::AbsDelayStateDerivative(buffer_id) => {
                self.execute_absdelay(*buffer_id, false, true)?
            }
            Instruction::AbsDelayStateDerivativeMax(buffer_id) => {
                self.execute_absdelay(*buffer_id, true, true)?
            }
            Instruction::TransitionState(_) => {
                let fall = self.pop_real("transition fall time")?;
                let rise = self.pop_real("transition rise time")?;
                let delay = self.pop_real("transition delay")?;
                let input = self.pop_real("transition input")?;
                super::filters::TransitionFilter::validate_operands(
                    input,
                    self.context.time,
                    delay,
                    rise,
                    fall,
                )
                .map_err(|error| VmError::InvalidNumericResult(format!("transition: {error}")))?;
                self.stack.push(Complex64::new(input, 0.0));
            }
            Instruction::TransitionStateDerivative(_) => {
                let fall = self.pop_real("transition derivative fall time")?;
                let rise = self.pop_real("transition derivative rise time")?;
                let delay = self.pop_real("transition derivative delay")?;
                let derivative = self.pop("transition input derivative")?;
                let input = self.pop_real("transition operating-point input")?;
                super::filters::TransitionFilter::validate_operands(
                    input,
                    self.context.time,
                    delay,
                    rise,
                    fall,
                )
                .map_err(|error| {
                    VmError::InvalidNumericResult(format!("transition derivative: {error}"))
                })?;
                self.stack.push(derivative);
            }
            Instruction::SlewState(_) => {
                let negative = self.pop_real("slew negative rate")?;
                let positive = self.pop_real("slew positive rate")?;
                let input = self.pop_real("slew input")?;
                let NormalizedSlewRates::Limited(_) =
                    normalize_slew_rates(Some(positive), Some(negative))
                        .map_err(|error| VmError::InvalidNumericResult(format!("slew: {error}")))?
                else {
                    return Err(VmError::InvalidInstruction(
                        "stateful slew instruction encoded passthrough rates",
                    ));
                };
                self.stack.push(Complex64::new(input, 0.0));
            }
            Instruction::SlewStateDerivative(filter_id) => {
                let _negative_derivative = self.pop("slew negative-rate derivative")?;
                let negative = self.pop_real("slew negative rate")?;
                let _positive_derivative = self.pop("slew positive-rate derivative")?;
                let positive = self.pop_real("slew positive rate")?;
                let derivative = self.pop("slew input derivative")?;
                let _input = self.pop_real("slew operating-point input")?;
                let NormalizedSlewRates::Limited(_) =
                    normalize_slew_rates(Some(positive), Some(negative)).map_err(|error| {
                        VmError::InvalidNumericResult(format!("slew derivative: {error}"))
                    })?
                else {
                    return Err(VmError::InvalidInstruction(
                        "stateful slew derivative encoded passthrough rates",
                    ));
                };
                let filter = self.context.slew_filters.get(*filter_id).ok_or(
                    VmError::InvalidInstruction("missing slew filter during small-signal replay"),
                )?;
                self.stack
                    .push(derivative * filter.small_signal_input_gain(self.context.time));
            }
            Instruction::CrossState(_) => {
                for label in [
                    "cross enable",
                    "cross expr_tol",
                    "cross time_tol",
                    "cross direction",
                    "cross input",
                ] {
                    let _ = self.pop_real(label)?;
                }
                self.stack.push(Complex64::new(0.0, 0.0));
            }
            Instruction::LastCrossingState(_) => {
                let _direction = self.pop_real("last_crossing direction")?;
                let _input = self.pop_real("last_crossing input")?;
                self.stack.push(Complex64::new(-1.0, 0.0));
            }
            Instruction::WhiteNoise => {
                let _power = self.pop_real("white_noise power")?;
                self.stack.push(Complex64::new(0.0, 0.0));
            }
            Instruction::FlickerNoise => {
                let _exponent = self.pop_real("flicker_noise exponent")?;
                let _power = self.pop_real("flicker_noise power")?;
                self.stack.push(Complex64::new(0.0, 0.0));
            }
            Instruction::Analysis(kind) => {
                let current = self.context.analysis_type;
                let active = match kind {
                    0 => current == 0,
                    1 => current == 1,
                    2 => current == 2,
                    3 => current == 3,
                    4 => current == 4,
                    5 => matches!(current, 0 | 4),
                    6 => matches!(current, 1 | 3),
                    7 => self.context.analysis_initial_step,
                    8 => self.context.analysis_final_step,
                    _ => false,
                };
                self.stack.push(Complex64::new(f64::from(active), 0.0));
            }
            Instruction::AboveState(_) => {
                for label in [
                    "above enable",
                    "above expr_tol",
                    "above time_tol",
                    "above input",
                ] {
                    let _ = self.pop_real(label)?;
                }
                self.stack.push(Complex64::new(0.0, 0.0));
            }
            Instruction::TimerState(_) => {
                for label in [
                    "timer enable",
                    "timer time_tol",
                    "timer period",
                    "timer start",
                ] {
                    let _ = self.pop_real(label)?;
                }
                self.stack.push(Complex64::new(0.0, 0.0));
            }
            Instruction::LaplaceState(filter_id) => {
                let input = self.pop("LaplaceState")?;
                let filter = self
                    .context
                    .laplace_filters
                    .get(*filter_id)
                    .ok_or(VmError::InvalidInstruction("missing laplace filter"))?;
                let gain = filter.dc_output(1.0).map_err(|error| {
                    VmError::InvalidNumericResult(format!("Laplace filter {filter_id}: {error}"))
                })?;
                self.stack.push(input * gain);
            }
            Instruction::LaplaceStateDerivative(filter_id) => {
                let input = self.pop("LaplaceStateDerivative")?;
                let filter = self
                    .context
                    .laplace_filters
                    .get(*filter_id)
                    .ok_or(VmError::InvalidInstruction("missing laplace filter"))?;
                let (real, imag) = filter
                    .frequency_response_rectangular(self.frequency_hz)
                    .map_err(|error| {
                        VmError::InvalidNumericResult(format!(
                            "Laplace filter {filter_id}: {error}"
                        ))
                    })?;
                self.stack.push(Complex64::new(real, imag) * input);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::Instruction;
    use crate::laplace::StateSpaceFilter;
    use crate::timing_contract::SlewRateMagnitudes;

    fn ac_context() -> VmContext {
        let mut context = VmContext::new(0);
        context.analysis_type = 1;
        context
    }

    #[test]
    fn nested_laplace_actions_preserve_complex_cross_products() {
        let mut context = ac_context();
        context.laplace_filters = vec![
            StateSpaceFilter::lowpass_first_order(1.0).unwrap(),
            StateSpaceFilter::lowpass_first_order(1.0).unwrap(),
        ];
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(1.0),
                Instruction::LaplaceStateDerivative(0),
                Instruction::LaplaceStateDerivative(1),
            ],
        };
        let result = SmallSignalVm::new(&context, 1.0)
            .unwrap()
            .execute(&program)
            .unwrap();
        assert!((result.re - 0.0).abs() <= 1.0e-14, "{result:?}");
        assert!((result.im + 0.5).abs() <= 1.0e-14, "{result:?}");
    }

    #[test]
    fn ddt_and_idt_have_exact_frequency_domain_actions() {
        let context = ac_context();
        let frequency = 7.0;
        let omega = std::f64::consts::TAU * frequency;
        let ddt = BytecodeProgram {
            instructions: vec![Instruction::PushConst(2.0), Instruction::DdtJacobian],
        };
        let idt = BytecodeProgram {
            instructions: vec![Instruction::PushConst(2.0), Instruction::IdtJacobian],
        };
        let mut vm = SmallSignalVm::new(&context, frequency).unwrap();
        assert_eq!(vm.execute(&ddt).unwrap(), Complex64::new(0.0, 2.0 * omega));
        let integral = vm.execute(&idt).unwrap();
        assert_eq!(integral.re, 0.0);
        assert!((integral.im + 2.0 / omega).abs() <= 1.0e-15);
    }

    #[test]
    fn absdelay_applies_transport_phase_without_delay_modulation_term() {
        let mut context = ac_context();
        context.allocate_delay_buffers(1);
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(3.0),
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.25),
                Instruction::PushConst(99.0),
                Instruction::AbsDelayStateDerivative(0),
            ],
        };
        let result = SmallSignalVm::new(&context, 1.0)
            .unwrap()
            .execute(&program)
            .unwrap();
        assert!((result.re - 0.0).abs() <= 1.0e-14, "{result:?}");
        assert!((result.im + 1.0).abs() <= 1.0e-14, "{result:?}");
    }

    #[test]
    fn retained_active_slew_has_zero_small_signal_input_gain() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(5.0),
                Instruction::PushConst(3.0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(0.0),
                Instruction::PushConst(-0.5),
                Instruction::PushConst(0.0),
                Instruction::SlewStateDerivative(0),
            ],
        };

        let mut fresh = ac_context();
        fresh.allocate_slew_filters(1);
        let fresh_result = SmallSignalVm::new(&fresh, 1.0)
            .unwrap()
            .execute(&program)
            .unwrap();
        assert_eq!(fresh_result, Complex64::new(3.0, 0.0));

        let mut retained = ac_context();
        retained.allocate_slew_filters(1);
        retained.slew_filters[0].eval_operating_point(0.0, 0.0);
        retained.slew_filters[0].promote_operating_point_candidate();
        retained.slew_filters[0].eval(
            2.0,
            1.0,
            SlewRateMagnitudes {
                rise: 0.5,
                fall: 0.5,
            },
        );
        retained.slew_filters[0].commit();
        retained.time = 1.0;
        assert_eq!(retained.slew_filters[0].next_corner_time(1.0), Some(4.0));
        let retained_result = SmallSignalVm::new(&retained, 1.0)
            .unwrap()
            .execute(&program)
            .unwrap();
        assert_eq!(retained_result, Complex64::new(0.0, 0.0));
    }
}
