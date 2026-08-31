//! The stack machine that executes compiled bytecode programs.
//!
//! [`super::Vm`] walks a [`BytecodeProgram`] against a
//! [`super::VmContext`], pushing and popping f64 operands. Analog
//! operators appear as single instructions that consult or update the
//! context's state slots, so the executor itself holds no memory between
//! programs — all of it lives in the context.
//!
//! This is the reference runtime: whatever the JIT and the generated Rust
//! backend produce is expected to agree with it numerically.

use super::context::INTEGRATION_CANDIDATE_VALID;
use super::{VmContext, VmError};
use crate::array_index::{ArrayIndexError, checked_array_slot, saturated_array_upper};
use crate::codegen::{BytecodeProgram, Instruction};
use crate::integer_runtime::{IntegerBinaryOperation, integer_binary, real_to_integer};
use crate::timing_contract::{NormalizedSlewRates, normalize_slew_rates};

fn event_integer_operand(name: &str, value: f64) -> Result<i32, VmError> {
    let converted = real_to_integer(value).map_err(|error| {
        VmError::InvalidNumericResult(format!("{name} integer conversion failed: {error}"))
    })?;
    if f64::from(converted) != value {
        return Err(VmError::InvalidNumericResult(format!(
            "{name} must evaluate to an integer, got {value}"
        )));
    }
    Ok(converted)
}

fn limited_exp(value: f64) -> f64 {
    const LIMIT: f64 = 80.0;
    const LOW_VALUE: f64 = 1.804851387e-35;
    if value > LIMIT {
        LIMIT.exp() * (1.0 + value - LIMIT)
    } else if value < -LIMIT {
        LOW_VALUE
    } else {
        value.exp()
    }
}

/// Stack-based virtual machine for bytecode execution.
pub struct Vm<'a> {
    /// Execution context
    pub context: &'a mut VmContext,
    /// Evaluation stack
    pub stack: Vec<f64>,
}

/// Evaluate a Zi value directly from its canonical operand slice. Browser-WASM
/// helpers use this entry rather than copying a variable-length definition into
/// the VM's heap-backed stack.
pub(crate) fn execute_zi_state(
    context: &mut VmContext,
    layout: crate::codegen::ZiRuntimeLayout,
    operands: &[f64],
) -> Result<f64, VmError> {
    let operand_count = layout.validate_operand_budget().map_err(|error| {
        VmError::InvalidNumericResult(format!("Zi runtime layout rejected: {error}"))
    })?;
    if operands.len() != operand_count {
        return Err(VmError::InvalidInstruction("invalid zi operand count"));
    }
    let input = operands[operands.len() - 2];
    let transition = operands[operands.len() - 1];
    let filter_id = layout.filter_id;
    let filter = context
        .zi_filters
        .get_mut(filter_id)
        .ok_or(VmError::InvalidInstruction("missing zi filter"))?;
    if !filter.definition_is_frozen() {
        *filter = layout.freeze_filter(operands).map_err(|error| {
            VmError::InvalidNumericResult(format!(
                "zi filter {filter_id} definition freeze failed: {error}"
            ))
        })?;
    }
    let time = context.time;
    let transient = context.analysis_type == 2;
    context
        .zi_filters
        .get_mut(filter_id)
        .ok_or(VmError::InvalidInstruction("missing zi filter"))?
        .eval_with_transition_constraint(
            input,
            time,
            transient,
            transition,
            layout.direct_assignment,
        )
        .map_err(|error| VmError::InvalidNumericResult(format!("zi filter {filter_id}: {error}")))
}

/// Read-only Zi derivative counterpart to [`execute_zi_state`].
pub(crate) fn execute_zi_state_derivative(
    context: &mut VmContext,
    layout: crate::codegen::ZiRuntimeLayout,
    operands: &[f64],
) -> Result<f64, VmError> {
    let operand_count = layout.validate_operand_budget().map_err(|error| {
        VmError::InvalidNumericResult(format!("Zi runtime layout rejected: {error}"))
    })?;
    if operands.len() != operand_count {
        return Err(VmError::InvalidInstruction(
            "invalid zi derivative operand count",
        ));
    }
    let derivative = operands[operands.len() - 2];
    let transition = operands[operands.len() - 1];
    let filter_id = layout.filter_id;
    let filter = context
        .zi_filters
        .get_mut(filter_id)
        .ok_or(VmError::InvalidInstruction("missing zi filter"))?;
    if !filter.definition_is_frozen() {
        *filter = layout.freeze_filter(operands).map_err(|error| {
            VmError::InvalidNumericResult(format!(
                "zi filter {filter_id} definition freeze failed: {error}"
            ))
        })?;
    }
    let time = context.time;
    let transient = context.analysis_type == 2;
    context
        .zi_filters
        .get(filter_id)
        .ok_or(VmError::InvalidInstruction("missing zi filter"))?
        .eval_derivative_with_constraint(
            derivative,
            time,
            transient,
            transition,
            layout.direct_assignment,
        )
        .map_err(|error| VmError::InvalidNumericResult(format!("zi filter {filter_id}: {error}")))
}

impl<'a> Vm<'a> {
    /// Create a new VM with the given context.
    pub fn new(context: &'a mut VmContext) -> Self {
        Self {
            context,
            stack: Vec::with_capacity(32),
        }
    }

    /// Execute a bytecode program and return the result.
    pub fn execute(&mut self, program: &BytecodeProgram) -> Result<f64, VmError> {
        self.stack.clear();

        for instruction in &program.instructions {
            self.execute_instruction(instruction)?;
        }

        self.stack
            .pop()
            .ok_or(VmError::StackUnderflow("No result on stack"))
    }

    /// Resolve a runtime array index (declared-bounds space) to a variable
    /// slot of the contiguous element run at `base`
    pub fn array_slot(raw: f64, base: usize, len: usize, lower: i64) -> Result<usize, VmError> {
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

    /// Execute a single instruction.
    #[inline]
    pub(crate) fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        match instruction {
            Instruction::PushConst(v) => {
                self.stack.push(*v);
            }
            Instruction::PushParam(idx) => {
                let v = self
                    .context
                    .parameters
                    .get(*idx)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing parameter slot"))?;
                self.stack.push(v);
            }
            Instruction::PushParamGiven(idx) => {
                let given = self
                    .context
                    .param_given
                    .get(*idx)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing parameter-given slot"))?;
                let v = if given != 0 { 1.0 } else { 0.0 };
                self.stack.push(v);
            }
            Instruction::PushVoltage(pos, neg) => {
                let v = self.context.try_voltage(*pos, *neg)?;
                self.stack.push(v);
            }
            Instruction::PushCurrent(pos, neg) => {
                let v = self.context.try_current(*pos, *neg)?;
                self.stack.push(v);
            }
            Instruction::PushBranchCurrent(k) => {
                let v = self
                    .context
                    .branch_current_values
                    .get(*k)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing branch-current slot"))?;
                self.stack.push(v);
            }
            Instruction::PushInternalVoltage(idx) => {
                let v = self
                    .context
                    .internal_voltages
                    .get(*idx)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing internal-voltage slot"))?;
                self.stack.push(v);
            }
            Instruction::PushVariable(idx) => {
                let v = self
                    .context
                    .variables
                    .get(*idx)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing variable slot"))?;
                self.stack.push(v);
            }
            Instruction::PushVariableDyn { base, len, lower } => {
                let raw = self
                    .stack
                    .pop()
                    .ok_or(VmError::StackUnderflow("PushVariableDyn"))?;
                let slot = Self::array_slot(raw, *base, *len, *lower)?;
                let v = self
                    .context
                    .variables
                    .get(slot)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing dynamic variable slot"))?;
                self.stack.push(v);
            }
            Instruction::PushTime => {
                self.stack.push(self.context.time);
            }
            Instruction::PushMfactor => {
                self.stack.push(self.context.multiplicity);
            }
            Instruction::PushPortConnected(terminal) => {
                self.stack.push(if self.context.port_connected(*terminal) {
                    1.0
                } else {
                    0.0
                });
            }
            Instruction::ZiState(layout) => {
                let operand_count = layout.validate_operand_budget().map_err(|error| {
                    VmError::InvalidNumericResult(format!("Zi runtime layout rejected: {error}"))
                })?;
                if self.stack.len() < operand_count {
                    return Err(VmError::StackUnderflow("ZiState"));
                }
                let start = self.stack.len() - operand_count;
                let output = execute_zi_state(self.context, *layout, &self.stack[start..])?;
                self.stack.truncate(start);
                self.stack.push(output);
            }
            Instruction::ZiStateDerivative(layout) => {
                let operand_count = layout.validate_operand_budget().map_err(|error| {
                    VmError::InvalidNumericResult(format!("Zi runtime layout rejected: {error}"))
                })?;
                if self.stack.len() < operand_count {
                    return Err(VmError::StackUnderflow("ZiStateDerivative"));
                }
                let start = self.stack.len() - operand_count;
                let output =
                    execute_zi_state_derivative(self.context, *layout, &self.stack[start..])?;
                self.stack.truncate(start);
                self.stack.push(output);
            }
            Instruction::PushTemperature => {
                self.stack.push(self.context.temperature);
            }
            Instruction::PushVt => {
                self.stack.push(self.context.vt());
            }

            // Binary operations
            Instruction::Add => self.binary_op(|a, b| a + b)?,
            Instruction::Sub => self.binary_op(|a, b| a - b)?,
            Instruction::Mul => self.binary_op(|a, b| a * b)?,
            Instruction::Div => self.binary_op(|a, b| a / b)?,
            Instruction::Pow => self.binary_op(|a, b| a.powf(b))?,
            Instruction::Mod => self.binary_op(|a, b| a % b)?,
            Instruction::Shl => self.integer_binary_op(IntegerBinaryOperation::Shl)?,
            Instruction::Shr => self.integer_binary_op(IntegerBinaryOperation::Shr)?,
            Instruction::BitAnd => self.integer_binary_op(IntegerBinaryOperation::BitAnd)?,
            Instruction::BitOr => self.integer_binary_op(IntegerBinaryOperation::BitOr)?,
            Instruction::BitXor => self.integer_binary_op(IntegerBinaryOperation::BitXor)?,

            // Unary operations
            Instruction::Neg => self.unary_op(|a| -a)?,
            Instruction::Abs => self.unary_op(|a| a.abs())?,
            Instruction::Sqrt => self.unary_op(|a| a.sqrt())?,
            Instruction::Exp => self.unary_op(|a| a.exp())?,
            Instruction::Log => self.unary_op(|a| a.ln())?,
            Instruction::Log10 => self.unary_op(|a| a.log10())?,
            Instruction::Sin => self.unary_op(|a| a.sin())?,
            Instruction::Cos => self.unary_op(|a| a.cos())?,
            Instruction::Tan => self.unary_op(|a| a.tan())?,
            Instruction::Sinh => self.unary_op(|a| a.sinh())?,
            Instruction::Cosh => self.unary_op(|a| a.cosh())?,
            Instruction::Tanh => self.unary_op(|a| a.tanh())?,

            // Two-argument functions
            Instruction::Min => self.binary_op(|a, b| a.min(b))?,
            Instruction::Max => self.binary_op(|a, b| a.max(b))?,

            // Limited exponential for convergence
            // Uses linear extrapolation beyond the limit to prevent overflow
            // while maintaining C0 and C1 continuity
            Instruction::Limexp => self.unary_op(|a| {
                const LIMIT: f64 = 40.0; // exp(40) ~= 2.4e17
                if a > LIMIT {
                    let exp_limit = LIMIT.exp();
                    // Linear extrapolation: f(x) = f(limit) + f'(limit) * (x - limit)
                    // For exp, f'(x) = exp(x), so f'(limit) = exp(limit)
                    exp_limit * (1.0 + a - LIMIT)
                } else if a < -LIMIT {
                    // For very negative values, return essentially 0
                    (-LIMIT).exp()
                } else {
                    a.exp()
                }
            })?,
            Instruction::LimitedExp => self.unary_op(limited_exp)?,

            // Inverse trigonometric functions
            Instruction::Asin => self.unary_op(|a| a.asin())?,
            Instruction::Acos => self.unary_op(|a| a.acos())?,
            Instruction::Atan => self.unary_op(|a| a.atan())?,
            Instruction::Asinh => self.unary_op(|a| a.asinh())?,
            Instruction::Acosh => self.unary_op(|a| a.acosh())?,
            Instruction::Atanh => self.unary_op(|a| a.atanh())?,
            Instruction::Atan2 => self.binary_op(|y, x| y.atan2(x))?,

            // Rounding functions
            Instruction::Floor => self.unary_op(|a| a.floor())?,
            Instruction::Ceil => self.unary_op(|a| a.ceil())?,

            // Power function (2-argument: base^exponent)
            Instruction::FnPow => self.binary_op(|base, exp| base.powf(exp))?,

            // Conditional: if cond != 0, use then_val, else else_val
            Instruction::IfElse => {
                let else_val = self.pop()?;
                let then_val = self.pop()?;
                let cond = self.pop()?;
                let result = if cond != 0.0 { then_val } else { else_val };
                self.stack.push(result);
            }

            // Comparison operations (return 1.0 for true, 0.0 for false)
            Instruction::Gt => self.binary_op(|a, b| if a > b { 1.0 } else { 0.0 })?,
            Instruction::Lt => self.binary_op(|a, b| if a < b { 1.0 } else { 0.0 })?,
            Instruction::Ge => self.binary_op(|a, b| if a >= b { 1.0 } else { 0.0 })?,
            Instruction::Le => self.binary_op(|a, b| if a <= b { 1.0 } else { 0.0 })?,
            Instruction::Eq => self.binary_op(|a, b| if a == b { 1.0 } else { 0.0 })?,
            Instruction::Ne => self.binary_op(|a, b| if a != b { 1.0 } else { 0.0 })?,

            // Logical operations
            Instruction::And => {
                self.binary_op(|a, b| if a != 0.0 && b != 0.0 { 1.0 } else { 0.0 })?
            }
            Instruction::Or => {
                self.binary_op(|a, b| if a != 0.0 || b != 0.0 { 1.0 } else { 0.0 })?
            }
            Instruction::Not => self.unary_op(|a| if a == 0.0 { 1.0 } else { 0.0 })?,

            // State-based ddt using the transient solver's companion rule.
            Instruction::DdtState(idx) => {
                let current_value = self.pop()?;
                if self.context.state_values.len() <= *idx {
                    self.context.allocate_states(*idx + 1);
                }
                let initialized = self.context.state_initialized[*idx];
                let prev_value = self
                    .context
                    .state_values_prev
                    .get(*idx)
                    .copied()
                    .filter(|_| initialized)
                    .unwrap_or(current_value);
                let older_value = self
                    .context
                    .state_values_older
                    .get(*idx)
                    .copied()
                    .filter(|_| initialized)
                    .unwrap_or(prev_value);
                let previous_derivative = self
                    .context
                    .state_derivatives_prev
                    .get(*idx)
                    .copied()
                    .filter(|_| initialized)
                    .unwrap_or(0.0);

                self.context.state_values[*idx] = current_value;
                let coefficients = self.context.integration_coefficients();
                let derivative = if coefficients.active {
                    coefficients.derivative_scale * current_value
                        - coefficients.previous_value_scale * prev_value
                        - coefficients.older_value_scale * older_value
                        - coefficients.previous_derivative_scale * previous_derivative
                } else {
                    0.0
                };
                self.context.state_derivatives[*idx] = derivative;
                self.context.state_older_candidate[*idx] = prev_value;
                self.context.state_candidate_valid[*idx] = INTEGRATION_CANDIDATE_VALID;
                self.stack.push(derivative);
            }

            // State-based idt, algebraically inverted from the same companion
            // derivative rule used by ddt.
            Instruction::IdtState(idx) => {
                let ic = self.pop()?;
                let current_value = self.pop()?;
                if self.context.state_values.len() <= *idx {
                    self.context.allocate_states(*idx + 1);
                }
                let initialized = self.context.state_initialized[*idx];
                let prev_integral = self.context.state_values_prev[*idx];
                let prev_integral = if initialized { prev_integral } else { ic };
                let older_integral = if initialized {
                    self.context.state_values_older[*idx]
                } else {
                    prev_integral
                };
                let previous_input = if initialized {
                    self.context.state_derivatives_prev[*idx]
                } else {
                    current_value
                };
                let coefficients = self.context.integration_coefficients();
                let new_integral = if coefficients.active {
                    (current_value
                        + coefficients.previous_value_scale * prev_integral
                        + coefficients.older_value_scale * older_integral
                        + coefficients.previous_derivative_scale * previous_input)
                        / coefficients.derivative_scale
                } else {
                    ic
                };
                self.context.state_values[*idx] = new_integral;
                self.context.state_derivatives[*idx] = current_value;
                self.context.state_older_candidate[*idx] = prev_integral;
                self.context.state_candidate_valid[*idx] = INTEGRATION_CANDIDATE_VALID;

                self.stack.push(new_integral);
            }

            // Wrapped integration: idtmod(expr, ic, modulus, offset)
            // Stack: [expr, ic, modulus, offset]; the integral folds into
            // [offset, offset + modulus)
            Instruction::IdtModState(idx) => {
                let offset = self.pop()?;
                let modulus = self.pop()?;
                let ic = self.pop()?;
                let current_value = self.pop()?;
                if self.context.state_values.len() <= *idx {
                    self.context.allocate_states(*idx + 1);
                }
                let initialized = self.context.state_initialized[*idx];
                let prev = if initialized {
                    self.context.state_values_prev[*idx]
                } else {
                    ic
                };
                let older = if initialized {
                    self.context.state_values_older[*idx]
                } else {
                    prev
                };
                let previous_input = if initialized {
                    self.context.state_derivatives_prev[*idx]
                } else {
                    current_value
                };
                let coefficients = self.context.integration_coefficients();
                let raw = if coefficients.active {
                    (current_value
                        + coefficients.previous_value_scale * prev
                        + coefficients.older_value_scale * older
                        + coefficients.previous_derivative_scale * previous_input)
                        / coefficients.derivative_scale
                } else {
                    ic
                };

                let (wrapped, rebase) = super::idtmod_wrapped_candidate(raw, modulus, offset)
                    .map_err(|detail| {
                        VmError::InvalidNumericResult(format!(
                            "idtmod state {idx} {detail}: raw={raw}, modulus={modulus}, offset={offset}"
                        ))
                    })?;
                let older_candidate = prev - rebase;
                if !older_candidate.is_finite() {
                    return Err(VmError::InvalidNumericResult(format!(
                        "idtmod state {idx} common-branch older history is not finite: previous={prev}, translation={rebase}"
                    )));
                }

                self.context.state_values[*idx] = wrapped;
                self.context.state_derivatives[*idx] = current_value;
                self.context.state_older_candidate[*idx] = older_candidate;
                self.context.state_candidate_valid[*idx] = INTEGRATION_CANDIDATE_VALID;

                self.stack.push(wrapped);
            }

            // Companion Jacobian factor for ddt: a / dt (0 at DC)
            Instruction::DdtJacobian => {
                let coefficients = self.context.integration_coefficients();
                self.unary_op(|a| {
                    if coefficients.active {
                        a * coefficients.derivative_scale
                    } else {
                        0.0
                    }
                })?
            }

            // Companion Jacobian factor for idt: a * dt (0 at DC)
            Instruction::IdtJacobian => {
                let coefficients = self.context.integration_coefficients();
                self.unary_op(|a| {
                    if coefficients.active {
                        a / coefficients.derivative_scale
                    } else {
                        0.0
                    }
                })?
            }

            // Slope of a lookup table at the input point
            Instruction::TableDerivative(table_id) => {
                let input = self.pop()?;
                let table = self
                    .context
                    .lookup_tables
                    .get(*table_id)
                    .ok_or(VmError::InvalidInstruction("missing lookup table"))?;
                let result = table.derivative(input);
                self.stack.push(result);
            }

            // $limit function: bounds value change per Newton iteration
            // Stack: [new_value, step_limit] -> [limited_value]
            // Tracks the previous *iteration* value in the state slot (the
            // old implementation read the never-written previous-timestep
            // array, clamping against zero forever).
            Instruction::LimitState(idx) => {
                let step_limit = self.pop()?;
                let new_value = self.pop()?;

                if self.context.state_values.len() <= *idx
                    || self.context.state_values_prev.len() <= *idx
                    || self.context.state_values_older.len() <= *idx
                    || self.context.state_derivatives.len() <= *idx
                    || self.context.state_derivatives_prev.len() <= *idx
                    || self.context.state_initialized.len() <= *idx
                    || self.context.state_candidate_valid.len() <= *idx
                {
                    self.context.allocate_states(*idx + 1);
                }

                let limited_value = if self.context.state_initialized[*idx] {
                    let prev_value = self.context.state_values[*idx];
                    let delta = new_value - prev_value;
                    let limited_delta = delta.clamp(-step_limit, step_limit);
                    prev_value + limited_delta
                } else {
                    new_value
                };

                self.context.state_values[*idx] = limited_value;
                self.context.state_initialized[*idx] = true;
                self.stack.push(limited_value);
            }

            Instruction::CanonicalLimitState(_) => {
                return Err(VmError::InvalidInstruction(
                    "canonical-only named limiter metadata is non-executable; no interpreter fallback",
                ));
            }

            // TableLookup: linear interpolation in lookup table
            // Stack: [input_value] -> [interpolated_value]
            // Uses context.lookup_tables for table storage
            Instruction::TableLookup(table_id) => {
                let input = self.pop()?;
                let table = self
                    .context
                    .lookup_tables
                    .get(*table_id)
                    .ok_or(VmError::InvalidInstruction("missing lookup table"))?;
                let result = table.interpolate(input);
                self.stack.push(result);
            }

            // AbsDelayState: transport delay with circular buffer
            // Stack: [expr_value, delay_time] -> [delayed_value]
            // Uses context.delay_buffers for storage
            Instruction::AbsDelayState(buffer_id) => {
                let delay_time = self.pop()?;
                let current_value = self.pop()?;
                let current_time = self.context.time;
                let is_transient = self.context.analysis_type == 2;

                let result = if !is_transient {
                    current_value
                } else {
                    self.context
                        .delay_buffers
                        .get_mut(*buffer_id)
                        .ok_or_else(|| {
                            VmError::InvalidRuntimeConfiguration(format!(
                                "absdelay buffer {buffer_id} is not preallocated"
                            ))
                        })?
                        .eval(current_time, current_value, delay_time, None)
                        .map_err(VmError::InvalidRuntimeConfiguration)?
                };

                self.stack.push(result);
            }
            Instruction::AbsDelayStateMax(buffer_id) => {
                let max_delay = self.pop()?;
                let delay_time = self.pop()?;
                let current_value = self.pop()?;
                let result = if self.context.analysis_type == 2 {
                    self.context
                        .delay_buffers
                        .get_mut(*buffer_id)
                        .ok_or_else(|| {
                            VmError::InvalidRuntimeConfiguration(format!(
                                "absdelay buffer {buffer_id} is not preallocated"
                            ))
                        })?
                        .eval(
                            self.context.time,
                            current_value,
                            delay_time,
                            Some(max_delay),
                        )
                        .map_err(VmError::InvalidRuntimeConfiguration)?
                } else {
                    current_value
                };
                self.stack.push(result);
            }
            Instruction::AbsDelayStateDerivative(buffer_id)
            | Instruction::AbsDelayStateDerivativeMax(buffer_id) => {
                let max_delay = if matches!(instruction, Instruction::AbsDelayStateDerivativeMax(_))
                {
                    Some(self.pop()?)
                } else {
                    None
                };
                let delay_derivative = self.pop()?;
                let delay_time = self.pop()?;
                let input_derivative = self.pop()?;
                let input = self.pop()?;
                let result = if self.context.analysis_type == 2 {
                    let evaluation = self
                        .context
                        .delay_buffers
                        .get_mut(*buffer_id)
                        .ok_or_else(|| {
                            VmError::InvalidRuntimeConfiguration(format!(
                                "absdelay buffer {buffer_id} is not preallocated"
                            ))
                        })?
                        .eval_with_coefficients(self.context.time, input, delay_time, max_delay)
                        .map_err(VmError::InvalidRuntimeConfiguration)?;
                    evaluation.delay_coefficient.mul_add(
                        delay_derivative,
                        evaluation.input_coefficient * input_derivative,
                    )
                } else {
                    input_derivative
                };
                self.stack.push(result);
            }

            // TransitionState: piecewise-linear signal smoothing
            // Stack: [expr, delay, rise_time, fall_time] -> [filtered]
            Instruction::TransitionState(filter_id) => {
                let fall_time = self.pop()?;
                let rise_time = self.pop()?;
                let delay = self.pop()?;
                let input = self.pop()?;
                let time = self.context.time;
                if self.context.transition_filters.len() <= *filter_id {
                    self.context
                        .transition_filters
                        .resize_with(*filter_id + 1, Default::default);
                }
                let filter = &mut self.context.transition_filters[*filter_id];
                let result = match self.context.analysis_type {
                    2 => filter.eval(input, time, delay, rise_time, fall_time),
                    // DC and explicit initial-condition analysis pass the
                    // input through while retaining the final Newton
                    // candidate as the direct-transient seed.
                    0 | 4 => filter.eval_operating_point(input, time, delay, rise_time, fall_time),
                    // AC/noise use the LRM's approximate unity small-signal
                    // transfer without perturbing accepted time-domain state.
                    1 | 3 => super::filters::TransitionFilter::validate_operands(
                        input, time, delay, rise_time, fall_time,
                    )
                    .map(|()| input),
                    analysis_type => Err(format!(
                        "transition received invalid analysis type {analysis_type}"
                    )),
                }
                .map_err(|error| VmError::InvalidNumericResult(format!("transition: {error}")))?;

                self.stack.push(result);
            }
            Instruction::TransitionStateDerivative(filter_id) => {
                let fall_time = self.pop()?;
                let rise_time = self.pop()?;
                let delay = self.pop()?;
                let input_derivative = self.pop()?;
                let input = self.pop()?;
                let filter = self.context.transition_filters.get(*filter_id).ok_or(
                    VmError::InvalidInstruction("missing transition derivative filter"),
                )?;
                let result = filter
                    .eval_derivative(
                        input,
                        input_derivative,
                        self.context.time,
                        delay,
                        rise_time,
                        fall_time,
                        self.context.analysis_type,
                    )
                    .map_err(|error| {
                        VmError::InvalidNumericResult(format!("transition derivative: {error}"))
                    })?;
                self.stack.push(result);
            }

            // SlewState: slew rate limiting
            // Stack: [expr, max_pos_slew, max_neg_slew] -> [limited]
            Instruction::SlewState(filter_id) => {
                let max_neg_slew = self.pop()?;
                let max_pos_slew = self.pop()?;
                let input = self.pop()?;
                let time = self.context.time;

                let NormalizedSlewRates::Limited(rates) =
                    normalize_slew_rates(Some(max_pos_slew), Some(max_neg_slew))
                        .map_err(|error| VmError::InvalidNumericResult(format!("slew: {error}")))?
                else {
                    return Err(VmError::InvalidInstruction(
                        "stateful slew instruction encoded passthrough rates",
                    ));
                };
                if self.context.slew_filters.len() <= *filter_id {
                    self.context
                        .slew_filters
                        .resize_with(*filter_id + 1, Default::default);
                }
                let analysis_type = self.context.analysis_type;
                let filter = &mut self.context.slew_filters[*filter_id];
                let result = match analysis_type {
                    2 => filter.eval(input, time, rates),
                    // DC and explicit initial-condition analysis establish
                    // the seed promoted when transient integration starts.
                    0 | 4 => filter.eval_operating_point(input, time),
                    // AC/noise are read-only small-signal evaluations. They
                    // must not publish an OP candidate or perturb checkpoint
                    // readiness/later transient startup.
                    1 | 3 => input,
                    _ => {
                        return Err(VmError::InvalidRuntimeConfiguration(format!(
                            "slew received invalid analysis type {analysis_type}"
                        )));
                    }
                };

                self.stack.push(result);
            }
            Instruction::SlewStateDerivative(filter_id) => {
                let max_neg_slew_derivative = self.pop()?;
                let max_neg_slew = self.pop()?;
                let max_pos_slew_derivative = self.pop()?;
                let max_pos_slew = self.pop()?;
                let input_derivative = self.pop()?;
                let input = self.pop()?;
                let NormalizedSlewRates::Limited(rates) =
                    normalize_slew_rates(Some(max_pos_slew), Some(max_neg_slew)).map_err(
                        |error| VmError::InvalidNumericResult(format!("slew derivative: {error}")),
                    )?
                else {
                    return Err(VmError::InvalidInstruction(
                        "stateful slew derivative encoded passthrough rates",
                    ));
                };
                let filter = self
                    .context
                    .slew_filters
                    .get(*filter_id)
                    .ok_or(VmError::InvalidInstruction("missing slew filter"))?;
                let result = match self.context.analysis_type {
                    2 => filter.eval_derivative(
                        input,
                        input_derivative,
                        max_pos_slew_derivative,
                        max_neg_slew_derivative,
                        self.context.time,
                        rates,
                    ),
                    0 | 1 | 3 | 4 => input_derivative,
                    analysis_type => {
                        return Err(VmError::InvalidRuntimeConfiguration(format!(
                            "slew derivative received invalid analysis type {analysis_type}"
                        )));
                    }
                };
                self.stack.push(result);
            }

            // CrossState: threshold crossing detection
            // Stack: [expr, direction, time_tol, expr_tol, enable] -> [0 or 1]
            Instruction::CrossState(detector_id) => {
                let enable = self.pop()?;
                let expr_tol = self.pop()?;
                let time_tol = self.pop()?;
                let direction = self.pop()?;
                let value = self.pop()?;
                let time = self.context.time;
                let direction = event_integer_operand("cross direction", direction)?;
                let enabled = event_integer_operand("cross enable", enable)? != 0;
                let is_transient = self.context.analysis_type == 2;

                if self.context.cross_detectors.len() <= *detector_id {
                    self.context
                        .cross_detectors
                        .resize_with(*detector_id + 1, Default::default);
                }
                let detector = &mut self.context.cross_detectors[*detector_id];
                let crossed = detector
                    .eval_event(value, time, direction, time_tol, expr_tol, enabled)
                    .map_err(|error| {
                        VmError::InvalidNumericResult(format!("cross evaluation failed: {error}"))
                    })?;

                // Cross events only trigger in transient analysis.
                let result = if is_transient { crossed } else { 0.0 };

                self.stack.push(result);
            }

            // LastCrossingState: interpolated time of the last zero crossing
            // Stack: [expr, direction] -> [time or -1]
            Instruction::LastCrossingState(detector_id) => {
                let direction = self.pop()?;
                let value = self.pop()?;
                let direction = event_integer_operand("last_crossing direction", direction)?;
                if self.context.cross_detectors.len() <= *detector_id {
                    self.context
                        .cross_detectors
                        .resize_with(*detector_id + 1, Default::default);
                }
                let detector = &mut self.context.cross_detectors[*detector_id];
                let crossing_time = detector
                    .eval_last_crossing(value, self.context.time, direction)
                    .map_err(|error| {
                        VmError::InvalidNumericResult(format!(
                            "last_crossing evaluation failed: {error}"
                        ))
                    })?;
                self.stack.push(if self.context.analysis_type == 2 {
                    crossing_time
                } else {
                    -1.0
                });
            }

            // WhiteNoise: white noise source
            // Stack: [power] -> [0]
            // In time domain, noise returns 0
            Instruction::WhiteNoise => {
                let _power = self.pop()?;
                // Noise contributes to AC noise analysis, not time domain
                self.stack.push(0.0);
            }

            // FlickerNoise: 1/f flicker noise
            // Stack: [power, exponent] -> [0]
            Instruction::FlickerNoise => {
                let _exponent = self.pop()?;
                let _power = self.pop()?;
                // Noise contributes to AC noise analysis, not time domain
                self.stack.push(0.0);
            }

            // Analysis: check current analysis type
            // Stack: [] -> [0 or 1]
            // analysis_type encoding: 0=dc, 1=ac, 2=tran, 3=noise, 4=ic
            Instruction::Analysis(analysis_str_id) => {
                let current_type = self.context.analysis_type;
                let result = match analysis_str_id {
                    0 => {
                        // "dc" check
                        if current_type == 0 { 1.0 } else { 0.0 }
                    }
                    1 => {
                        // "ac" check
                        if current_type == 1 { 1.0 } else { 0.0 }
                    }
                    2 => {
                        // "tran" check
                        if current_type == 2 { 1.0 } else { 0.0 }
                    }
                    3 => {
                        // "noise" check
                        if current_type == 3 { 1.0 } else { 0.0 }
                    }
                    4 => {
                        // "ic" check
                        if current_type == 4 { 1.0 } else { 0.0 }
                    }
                    5 => {
                        // "static": any equilibrium analysis (DC or IC)
                        if current_type == 0 || current_type == 4 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    6 => {
                        // "smallsig": frequency-domain small-signal analyses
                        if current_type == 1 || current_type == 3 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    7 => f64::from(self.context.analysis_initial_step),
                    8 => f64::from(self.context.analysis_final_step),
                    _ => 0.0, // Unknown analysis type
                };
                self.stack.push(result);
            }

            // AboveState: initial-positive and rising crossing event detection
            // Stack: [expr, time_tol, expr_tol, enable] -> [0 or 1]
            Instruction::AboveState(detector_id) => {
                let enable = self.pop()?;
                let expr_tol = self.pop()?;
                let time_tol = self.pop()?;
                let value = self.pop()?;
                if self.context.cross_detectors.len() <= *detector_id {
                    self.context
                        .cross_detectors
                        .resize_with(*detector_id + 1, Default::default);
                }
                let detector = &mut self.context.cross_detectors[*detector_id];
                let enabled = event_integer_operand("above enable", enable)? != 0;
                let result = if matches!(self.context.analysis_type, 0 | 4) {
                    detector.eval_above_static(
                        value,
                        self.context.time,
                        time_tol,
                        expr_tol,
                        enabled,
                    )
                } else {
                    detector.eval_above(value, self.context.time, time_tol, expr_tol, enabled)
                }
                .map_err(|error| {
                    VmError::InvalidNumericResult(format!("above evaluation failed: {error}"))
                })?;
                self.stack.push(result);
            }

            // TimerState: one-shot or periodic timer event
            // Stack: [start_time, period, time_tol, enable] -> [0 or 1]
            Instruction::TimerState(_timer_id) => {
                let enable = self.pop()?;
                let time_tol = self.pop()?;
                let period = self.pop()?;
                let start_time = self.pop()?;
                let (result, next_event) = crate::vm::timer_event_evaluation(
                    start_time,
                    period,
                    time_tol,
                    enable,
                    self.context.time,
                    self.context.timestep(),
                );
                if let Some(next_event) = next_event {
                    self.context.request_timer_event(next_event);
                }
                self.stack.push(result);
            }

            // LaplaceState: Laplace transfer function (both ZP and ND forms)
            // Stack: [input] -> [filtered_output]
            Instruction::LaplaceState(filter_id) => {
                let input = self.pop()?;
                let result = if self.context.analysis_type == 2 {
                    let coefficients = self.context.integration_coefficients();
                    if let Some(filter) = self.context.laplace_filters.get_mut(*filter_id) {
                        let result = if coefficients.active {
                            filter.step_with_integration(input, coefficients)
                        } else {
                            // The transient operating-point pass has no
                            // integration formula. Solve an equilibrium
                            // candidate that acceptance can seed as history.
                            filter.dc_candidate(input)
                        };
                        result.map_err(|error| {
                            VmError::InvalidNumericResult(format!(
                                "Laplace filter {filter_id}: {error}"
                            ))
                        })?
                    } else {
                        return Err(VmError::InvalidInstruction("missing laplace filter"));
                    }
                } else {
                    // DC and others (s=0)
                    if let Some(filter) = self.context.laplace_filters.get(*filter_id) {
                        filter.dc_output(input).map_err(|error| {
                            VmError::InvalidNumericResult(format!(
                                "Laplace filter {filter_id}: {error}"
                            ))
                        })?
                    } else {
                        return Err(VmError::InvalidInstruction("missing laplace filter"));
                    }
                };
                self.stack.push(result);
            }
            // Read-only Laplace Jacobian action. Active transient integration
            // uses the coefficient of the current companion-rule input;
            // equilibrium and all other analyses use the filter's DC action.
            Instruction::LaplaceStateDerivative(filter_id) => {
                let input_derivative = self.pop()?;
                let coefficients = self.context.integration_coefficients();
                let filter = self
                    .context
                    .laplace_filters
                    .get(*filter_id)
                    .ok_or(VmError::InvalidInstruction("missing laplace filter"))?;
                let result = if self.context.analysis_type == 2 && coefficients.active {
                    let gain = filter.transient_input_gain(coefficients).map_err(|error| {
                        VmError::InvalidNumericResult(format!(
                            "Laplace derivative {filter_id}: {error}"
                        ))
                    })?;
                    let result = gain * input_derivative;
                    if !result.is_finite()
                        || (result == 0.0 && gain != 0.0 && input_derivative != 0.0)
                    {
                        return Err(VmError::InvalidNumericResult(format!(
                            "Laplace derivative {filter_id}: input action is not representable"
                        )));
                    }
                    result
                } else {
                    filter.dc_output(input_derivative).map_err(|error| {
                        VmError::InvalidNumericResult(format!(
                            "Laplace derivative {filter_id}: {error}"
                        ))
                    })?
                };
                self.stack.push(result);
            }
        }
        Ok(())
    }

    /// Pop a value from the stack.
    #[inline]
    fn pop(&mut self) -> Result<f64, VmError> {
        self.stack
            .pop()
            .ok_or(VmError::StackUnderflow("Stack underflow"))
    }

    /// Apply a unary operation.
    #[inline]
    fn unary_op<F>(&mut self, f: F) -> Result<(), VmError>
    where
        F: FnOnce(f64) -> f64,
    {
        let a = self.pop()?;
        self.stack.push(f(a));
        Ok(())
    }

    /// Apply a binary operation.
    #[inline]
    fn binary_op<F>(&mut self, f: F) -> Result<(), VmError>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.stack.push(f(a, b));
        Ok(())
    }

    #[inline]
    fn integer_binary_op(&mut self, operation: IntegerBinaryOperation) -> Result<(), VmError> {
        let right = self.pop()?;
        let left = self.pop()?;
        let value = integer_binary(operation, left, right).map_err(|error| {
            VmError::InvalidNumericResult(format!("Verilog-AMS integer operation failed: {error}"))
        })?;
        self.stack.push(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::IntegrationCoefficients;

    fn execute(instructions: Vec<Instruction>) -> Result<f64, VmError> {
        let mut context = VmContext::default();
        let mut vm = Vm::new(&mut context);
        vm.execute(&BytecodeProgram { instructions })
    }

    fn execute_with_context(
        context: &mut VmContext,
        instructions: Vec<Instruction>,
    ) -> Result<f64, VmError> {
        let mut vm = Vm::new(context);
        vm.execute(&BytecodeProgram { instructions })
    }

    #[test]
    fn integer_instructions_follow_the_shared_32_bit_contract() {
        assert_eq!(
            execute(vec![
                Instruction::PushConst(-16.0),
                Instruction::PushConst(2.0),
                Instruction::Shr,
            ]),
            Ok(1_073_741_820.0)
        );
        assert_eq!(
            execute(vec![
                Instruction::PushConst(1_073_741_824.0),
                Instruction::PushConst(1.0),
                Instruction::Shl,
            ]),
            Ok(f64::from(i32::MIN))
        );
        assert_eq!(
            execute(vec![
                Instruction::PushConst(5.5),
                Instruction::PushConst(3.0),
                Instruction::BitAnd,
            ]),
            Ok(2.0)
        );
    }

    #[test]
    fn integer_instructions_fail_closed_without_panicking() {
        for (left, right) in [
            (f64::NAN, 1.0),
            (f64::INFINITY, 1.0),
            (1.0, f64::NAN),
            (1.0, f64::from(i32::MAX) + 0.5),
        ] {
            let error = execute(vec![
                Instruction::PushConst(left),
                Instruction::PushConst(right),
                Instruction::Shl,
            ])
            .expect_err("invalid integer operation must fail");
            assert!(matches!(error, VmError::InvalidNumericResult(_)));
        }
    }

    #[test]
    fn event_integer_operands_are_exact_and_invalid_directions_are_inactive() {
        let cross = |value, direction, enable| {
            vec![
                Instruction::PushConst(value),
                Instruction::PushConst(direction),
                Instruction::PushConst(0.0),
                Instruction::PushConst(0.0),
                Instruction::PushConst(enable),
                Instruction::CrossState(0),
            ]
        };
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.time = 0.0;
        assert_eq!(
            execute_with_context(&mut context, cross(-1.0, 2.0, 1.0)),
            Ok(0.0)
        );
        context.cross_detectors[0].commit();
        context.time = 1.0;
        assert_eq!(
            execute_with_context(&mut context, cross(1.0, 2.0, 1.0)),
            Ok(0.0),
            "a direction other than -1, 0, or +1 generates no event"
        );

        for (direction, enable) in [(0.6, 1.0), (1.0, 0.5), (f64::NAN, 1.0)] {
            let error = execute_with_context(&mut context, cross(1.0, direction, enable))
                .expect_err("non-integer event operands must fail closed");
            assert!(matches!(error, VmError::InvalidNumericResult(_)));
        }
    }

    #[test]
    fn inverse_hyperbolic_instructions_use_stable_library_operations() {
        let tiny = 4.076_064_268_724_245e-15;
        let asinh = execute(vec![Instruction::PushConst(tiny), Instruction::Asinh])
            .expect("asinh instruction");
        let atanh = execute(vec![Instruction::PushConst(tiny), Instruction::Atanh])
            .expect("atanh instruction");
        let acosh_input = 1.0 + f64::EPSILON;
        let acosh = execute(vec![
            Instruction::PushConst(acosh_input),
            Instruction::Acosh,
        ])
        .expect("acosh instruction");

        assert_eq!(asinh.to_bits(), tiny.asinh().to_bits());
        assert_eq!(atanh.to_bits(), tiny.atanh().to_bits());
        assert_eq!(acosh.to_bits(), acosh_input.acosh().to_bits());
        assert_ne!(
            asinh.to_bits(),
            (tiny + (tiny * tiny + 1.0).sqrt()).ln().to_bits(),
            "the former expanded formula loses low-order input information"
        );
    }

    #[test]
    fn missing_parameter_slot_is_a_vm_error() {
        let err = execute(vec![Instruction::PushParam(0)])
            .expect_err("missing parameter slot must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn missing_param_given_slot_is_a_vm_error() {
        let err = execute(vec![Instruction::PushParamGiven(0)])
            .expect_err("missing $param_given slot must not evaluate as false");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn missing_variable_slot_is_a_vm_error() {
        let err = execute(vec![Instruction::PushVariable(0)])
            .expect_err("missing variable slot must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn missing_internal_voltage_slot_is_a_vm_error() {
        let err = execute(vec![Instruction::PushInternalVoltage(0)])
            .expect_err("missing internal voltage slot must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn terminal_voltage_to_ground_evaluates_from_context() {
        let mut context = VmContext::new(1);
        context.voltages[0] = 3.25;

        let value =
            execute_with_context(&mut context, vec![Instruction::PushVoltage(0, usize::MAX)])
                .expect("terminal-to-ground voltage should evaluate");

        assert_eq!(value, 3.25);
    }

    #[test]
    fn missing_internal_node_voltage_in_push_voltage_is_a_vm_error() {
        let mut context = VmContext::new(1);

        let err = execute_with_context(&mut context, vec![Instruction::PushVoltage(1, usize::MAX)])
            .expect_err("missing internal-node voltage must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn missing_branch_current_slot_is_a_vm_error() {
        let err = execute(vec![Instruction::PushBranchCurrent(0)])
            .expect_err("missing branch current slot must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn terminal_pair_current_uses_exact_pair_and_orientation() {
        let mut context = VmContext::new(2);
        context.currents.push(123.0);
        context.set_branch_current(0, 1, 4.25);

        let forward = execute_with_context(&mut context, vec![Instruction::PushCurrent(0, 1)])
            .expect("exact terminal-pair current should evaluate");
        assert_eq!(forward, 4.25);

        let reverse = execute_with_context(&mut context, vec![Instruction::PushCurrent(1, 0)])
            .expect("reverse terminal-pair current should evaluate");
        assert_eq!(reverse, -4.25);
    }

    #[test]
    fn terminal_to_ground_current_uses_exact_pair_and_orientation() {
        let mut context = VmContext::new(2);
        context.set_branch_current(0, usize::MAX, 3.5);

        let forward =
            execute_with_context(&mut context, vec![Instruction::PushCurrent(0, usize::MAX)])
                .expect("terminal-to-ground current should evaluate");
        assert_eq!(forward, 3.5);

        let reverse =
            execute_with_context(&mut context, vec![Instruction::PushCurrent(usize::MAX, 0)])
                .expect("ground-to-terminal current should evaluate");
        assert_eq!(reverse, -3.5);
    }

    #[test]
    fn missing_terminal_pair_current_is_a_vm_error() {
        let mut context = VmContext::new(2);
        context.currents.push(123.0);

        let err = execute_with_context(&mut context, vec![Instruction::PushCurrent(0, 1)])
            .expect_err("missing terminal-pair current must not alias first current");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn missing_dynamic_variable_slot_is_a_vm_error() {
        let err = execute(vec![
            Instruction::PushConst(1.0),
            Instruction::PushVariableDyn {
                base: 0,
                len: 1,
                lower: 1,
            },
        ])
        .expect_err("missing dynamic variable slot must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn dynamic_array_index_rejects_nonfinite_and_unrepresentable_values() {
        for raw in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let error = Vm::array_slot(raw, 0, 1, 0)
                .expect_err("non-finite array indices must fail closed");
            assert!(matches!(error, VmError::InvalidNumericResult(_)));
        }

        let error = Vm::array_slot(9_223_372_036_854_775_808.0, 0, 1, 0)
            .expect_err("unrepresentable rounded array indices must fail closed");
        assert!(matches!(error, VmError::InvalidNumericResult(_)));
    }

    #[test]
    fn dynamic_array_index_rejects_malformed_layout_arithmetic() {
        assert!(matches!(
            Vm::array_slot(0.0, 0, 0, 0),
            Err(VmError::InvalidInstruction(_))
        ));
        assert!(matches!(
            Vm::array_slot(1.0, usize::MAX, 2, 0),
            Err(VmError::InvalidInstruction(_))
        ));
    }

    #[test]
    fn missing_lookup_table_is_a_vm_error() {
        let err = execute(vec![
            Instruction::PushConst(0.5),
            Instruction::TableLookup(0),
        ])
        .expect_err("missing lookup table must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn missing_table_derivative_is_a_vm_error() {
        let err = execute(vec![
            Instruction::PushConst(0.5),
            Instruction::TableDerivative(0),
        ])
        .expect_err("missing table derivative must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn missing_zi_filter_is_a_vm_error() {
        let err = execute(vec![
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(0.0),
            Instruction::ZiState(crate::codegen::ZiRuntimeLayout::unit_coefficients(0)),
        ])
        .expect_err("missing zi filter must not evaluate as zero");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn zi_runtime_rejects_an_over_budget_layout_before_stack_arithmetic() {
        let layout = crate::codegen::ZiRuntimeLayout {
            filter_id: 0,
            numerator: crate::codegen::ZiPolynomialLayout::Coefficients { len: 1020 },
            denominator: crate::codegen::ZiPolynomialLayout::Coefficients { len: 1 },
            direct_assignment: false,
        };
        let error = execute(vec![Instruction::ZiState(layout)])
            .expect_err("a tampered over-budget Zi instruction must fail closed");

        assert!(matches!(error, VmError::InvalidNumericResult(_)));
        assert!(error.to_string().contains("platform-uniform maximum 1024"));
    }

    #[test]
    fn missing_laplace_filter_is_a_vm_error() {
        let err = execute(vec![
            Instruction::PushConst(1.0),
            Instruction::LaplaceState(0),
        ])
        .expect_err("missing laplace filter must not pass the input through");

        assert!(
            matches!(err, VmError::InvalidInstruction(_)),
            "expected invalid instruction error, got {err:?}"
        );
    }

    #[test]
    fn omitted_maxdelay_freezes_the_first_accepted_positive_delay() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.allocate_delay_buffers(1);
        let program = |input, delay| {
            vec![
                Instruction::PushConst(input),
                Instruction::PushConst(delay),
                Instruction::AbsDelayState(0),
            ]
        };

        for (time, input) in [(0.0, 1.0), (1.0, 2.0)] {
            context.time = time;
            context.begin_stateful_evaluation();
            execute_with_context(&mut context, program(input, 0.25))
                .expect("positive-delay transient candidate");
            context
                .advance_state()
                .expect("accept positive-delay transient candidate");
        }

        context.time = 2.0;
        context.begin_stateful_evaluation();
        assert_eq!(
            execute_with_context(&mut context, program(3.0, 2.0)).expect("frozen-delay evaluation"),
            2.75,
            "without maxdelay, the first accepted td must remain frozen"
        );
    }

    #[test]
    fn absdelay_requires_preallocated_transient_storage() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        let error = execute_with_context(
            &mut context,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.25),
                Instruction::AbsDelayState(0),
            ],
        )
        .expect_err("malformed bytecode must not allocate absdelay state while evaluating");
        assert!(matches!(error, VmError::InvalidRuntimeConfiguration(_)));
        assert!(error.to_string().contains("not preallocated"));
    }

    #[test]
    fn absdelay_maxdelay_derivative_uses_exact_interpolation_coefficients() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.allocate_delay_buffers(1);
        let primal = |input| {
            vec![
                Instruction::PushConst(input),
                Instruction::PushConst(0.5),
                Instruction::PushConst(2.0),
                Instruction::AbsDelayStateMax(0),
            ]
        };
        for (time, input) in [(0.0, 0.0), (1.0, 10.0)] {
            context.time = time;
            context.begin_stateful_evaluation();
            execute_with_context(&mut context, primal(input)).expect("absdelay history sample");
            context
                .advance_state()
                .expect("accept absdelay history sample");
        }

        context.time = 2.0;
        context.begin_stateful_evaluation();
        let derivative = execute_with_context(
            &mut context,
            vec![
                Instruction::PushConst(20.0),
                Instruction::PushConst(2.0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(3.0),
                Instruction::PushConst(2.0),
                Instruction::AbsDelayStateDerivativeMax(0),
            ],
        )
        .expect("exact absdelay derivative");
        assert_eq!(derivative, -29.0);

        context.begin_stateful_evaluation();
        let clamped = execute_with_context(
            &mut context,
            vec![
                Instruction::PushConst(20.0),
                Instruction::PushConst(2.0),
                Instruction::PushConst(4.0),
                Instruction::PushConst(3.0),
                Instruction::PushConst(2.0),
                Instruction::AbsDelayStateDerivativeMax(0),
            ],
        )
        .expect("clamped absdelay derivative");
        assert_eq!(clamped, 0.0, "clamped td has zero derivative coefficient");
    }

    #[test]
    fn singular_laplace_evaluations_are_typed_vm_errors() {
        let mut dc_context = VmContext::default();
        dc_context.laplace_filters.push(
            crate::laplace::StateSpaceFilter::from_transfer_function(&[1.0], &[1.0, 0.0])
                .expect("ideal integrator is valid in transient"),
        );
        let error = execute_with_context(
            &mut dc_context,
            vec![Instruction::PushConst(1.0), Instruction::LaplaceState(0)],
        )
        .expect_err("integrator DC equilibrium is singular");
        assert!(matches!(error, VmError::InvalidNumericResult(_)));
        assert!(error.to_string().contains("DC equilibrium"));

        let mut transient_context = VmContext::default();
        transient_context.analysis_type = 2;
        transient_context.try_set_timestep(1.0).unwrap();
        transient_context.laplace_filters.push(
            crate::laplace::StateSpaceFilter::new(vec![vec![1.0]], vec![1.0], vec![1.0], 0.0)
                .expect("well-formed state-space filter"),
        );
        let error = execute_with_context(
            &mut transient_context,
            vec![Instruction::PushConst(1.0), Instruction::LaplaceState(0)],
        )
        .expect_err("singular transient state solve must fail");
        assert!(matches!(error, VmError::InvalidNumericResult(_)));
        assert!(error.to_string().contains("transient"));
    }

    #[test]
    fn transient_laplace_inactive_integration_seeds_the_first_step_on_acceptance() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.try_set_timestep(0.0).unwrap();
        context.laplace_filters.push(
            crate::laplace::StateSpaceFilter::integrator(1.0)
                .expect("first-order low-pass realization"),
        );
        let program = |input| vec![Instruction::PushConst(input), Instruction::LaplaceState(0)];

        context.begin_stateful_evaluation();
        assert_eq!(
            execute_with_context(&mut context, program(4.0)).expect("t=0 DC candidate"),
            4.0
        );
        assert_eq!(context.laplace_filters[0].checkpoint().state, vec![0.0]);

        context.begin_stateful_evaluation();
        assert_eq!(
            execute_with_context(&mut context, program(6.0)).expect("replacement t=0 DC candidate"),
            6.0
        );
        context
            .advance_state()
            .expect("accept the final operating-point candidate");
        assert_eq!(context.laplace_filters[0].checkpoint().state, vec![6.0]);

        context.set_timestep(0.5);
        context.begin_stateful_evaluation();
        let first_step = execute_with_context(&mut context, program(2.0))
            .expect("first positive transient step");
        assert!((first_step - (14.0 / 3.0)).abs() <= 1.0e-12);
    }

    #[test]
    fn laplace_derivative_matches_transient_primal_finite_difference_and_is_read_only() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.try_set_timestep(0.5).unwrap();
        let mut filter = crate::laplace::StateSpaceFilter::integrator(1.0)
            .expect("first-order low-pass realization");
        filter
            .set_initial_state(&[0.25])
            .expect("matching accepted state");
        context.laplace_filters.push(filter);

        let derivative = execute_with_context(
            &mut context,
            vec![
                Instruction::PushConst(1.0),
                Instruction::LaplaceStateDerivative(0),
            ],
        )
        .expect("read-only transient derivative");
        assert!((derivative - 1.0 / 3.0).abs() <= 8.0 * f64::EPSILON);
        assert_eq!(context.laplace_filters[0].checkpoint().state, vec![0.25]);

        let base = context.laplace_filters[0].clone();
        let epsilon = 1.0e-6;
        let mut primal = |input| {
            context.laplace_filters[0] = base.clone();
            execute_with_context(
                &mut context,
                vec![Instruction::PushConst(input), Instruction::LaplaceState(0)],
            )
            .expect("finite transient primal")
        };
        let upper = primal(0.75 + epsilon);
        let lower = primal(0.75 - epsilon);
        let finite_difference = (upper - lower) / (2.0 * epsilon);
        assert!((finite_difference - derivative).abs() <= 1.0e-9);

        context.laplace_filters[0] = base;
        let primal_candidate = execute_with_context(
            &mut context,
            vec![Instruction::PushConst(0.75), Instruction::LaplaceState(0)],
        )
        .expect("publish transient primal candidate");
        assert!((primal_candidate - 5.0 / 12.0).abs() <= 1.0e-12);
        execute_with_context(
            &mut context,
            vec![
                Instruction::PushConst(1.0),
                Instruction::LaplaceStateDerivative(0),
            ],
        )
        .expect("derivative must preserve in-flight primal candidate");
        context
            .advance_state()
            .expect("accept the preserved primal candidate");
        assert!((context.laplace_filters[0].checkpoint().state[0] - 5.0 / 12.0).abs() <= 1.0e-12);
    }

    #[test]
    fn changing_companion_rule_invalidates_a_laplace_candidate() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context
            .try_set_integration_coefficients(IntegrationCoefficients::backward_euler(0.5))
            .expect("valid Backward Euler rule");
        let mut filter = crate::laplace::StateSpaceFilter::integrator(1.0)
            .expect("first-order low-pass realization");
        filter
            .set_initial_state(&[0.25])
            .expect("matching accepted state");
        context.laplace_filters.push(filter);

        context.begin_stateful_evaluation();
        execute_with_context(
            &mut context,
            vec![Instruction::PushConst(1.0), Instruction::LaplaceState(0)],
        )
        .expect("finite Backward Euler candidate");

        context
            .try_set_integration_coefficients(IntegrationCoefficients {
                active: true,
                derivative_scale: 4.0,
                previous_value_scale: 4.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 1.0,
            })
            .expect("valid trapezoidal rule");
        context
            .advance_state()
            .expect("coefficient change discarded the stale candidate");
        assert_eq!(context.laplace_filters[0].checkpoint().state, vec![0.25]);
    }

    #[test]
    fn transition_derivative_vm_is_branch_exact_and_preserves_primal_candidate() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context.allocate_transition_filters(1);
        context.transition_filters[0]
            .eval_operating_point(1.0, 0.0, 0.0, 1.0, 1.0)
            .unwrap();
        context.transition_filters[0].promote_operating_point_candidate();
        context.time = 1.0;
        let accepted = context.transition_filters[0].checkpoint();

        let direct_derivative = || {
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(3.0),
                Instruction::PushConst(0.0),
                Instruction::PushConst(0.0),
                Instruction::PushConst(0.0),
                Instruction::TransitionStateDerivative(0),
            ]
        };
        for _ in 0..2 {
            assert_eq!(
                execute_with_context(&mut context, direct_derivative()),
                Ok(3.0)
            );
            assert_eq!(context.transition_filters[0].checkpoint(), accepted);
        }

        context.begin_stateful_evaluation();
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(0.0),
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(2.0),
                    Instruction::TransitionState(0),
                ],
            ),
            Ok(1.0)
        );
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(3.0),
                    Instruction::PushConst(0.0),
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(2.0),
                    Instruction::TransitionStateDerivative(0),
                ],
            ),
            Ok(0.0)
        );
        context
            .advance_state()
            .expect("accept primal candidate after read-only derivative");
        let committed = context.transition_filters[0].checkpoint();
        assert_eq!(committed.input, 2.0);
        assert_eq!(committed.output, 1.0);
        assert!(committed.active.is_some());
    }

    #[test]
    fn laplace_derivative_uses_dc_action_without_active_transient_integration() {
        let mut context = VmContext::default();
        context.analysis_type = 2;
        context
            .try_set_integration_coefficients(IntegrationCoefficients::inactive())
            .unwrap();
        context.laplace_filters.push(
            crate::laplace::StateSpaceFilter::integrator(1.0)
                .expect("first-order low-pass realization"),
        );

        let derivative = execute_with_context(
            &mut context,
            vec![
                Instruction::PushConst(2.0),
                Instruction::LaplaceStateDerivative(0),
            ],
        )
        .expect("transient operating-point derivative uses DC action");
        assert_eq!(derivative, 2.0);
        assert_eq!(context.laplace_filters[0].checkpoint().state, vec![0.0]);
    }

    #[test]
    fn integration_initialization_is_published_only_when_the_candidate_is_accepted() {
        let mut context = VmContext::with_states(0, 3);
        context
            .try_set_integration_coefficients(IntegrationCoefficients::backward_euler(0.5))
            .unwrap();

        context.begin_stateful_evaluation();
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![Instruction::PushConst(4.0), Instruction::DdtState(0)],
            )
            .unwrap(),
            0.0
        );
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(10.0),
                    Instruction::IdtState(1),
                ],
            )
            .unwrap(),
            11.0
        );
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(0.0),
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.0),
                    Instruction::IdtModState(2),
                ],
            )
            .unwrap(),
            0.0
        );
        assert_eq!(context.state_initialized, vec![false; 3]);
        assert_eq!(context.state_candidate_valid, vec![1; 3]);
        assert!(
            context
                .accepted_checkpoint()
                .expect_err("a speculative integration candidate must block checkpoint capture")
                .to_string()
                .contains("in-flight Newton candidate")
        );

        context.begin_stateful_evaluation();
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![Instruction::PushConst(7.0), Instruction::DdtState(0)],
            )
            .unwrap(),
            0.0
        );
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![
                    Instruction::PushConst(4.0),
                    Instruction::PushConst(20.0),
                    Instruction::IdtState(1),
                ],
            )
            .unwrap(),
            22.0
        );
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.25),
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(0.0),
                    Instruction::IdtModState(2),
                ],
            )
            .unwrap(),
            0.75
        );
        assert_eq!(context.state_initialized, vec![false; 3]);

        context.advance_state().unwrap();
        assert_eq!(context.state_initialized, vec![true; 3]);
        assert_eq!(context.state_candidate_valid, vec![2; 3]);
        assert_eq!(context.state_values_prev, vec![7.0, 22.0, 0.75]);
    }

    #[test]
    fn acceptance_leaves_unexecuted_integration_slots_unchanged() {
        let mut context = VmContext::with_states(0, 2);
        context
            .try_set_integration_coefficients(IntegrationCoefficients::backward_euler(1.0))
            .unwrap();
        context.state_values_prev = vec![1.0, 2.0];
        context.state_values_older = vec![0.5, 1.5];
        context.state_derivatives_prev = vec![0.25, 0.75];
        context.state_initialized = vec![true, true];
        context.state_candidate_valid = vec![2, 2];

        context.begin_stateful_evaluation();
        execute_with_context(
            &mut context,
            vec![Instruction::PushConst(3.0), Instruction::DdtState(0)],
        )
        .unwrap();
        context.advance_state().unwrap();

        assert_eq!(context.state_values_prev, vec![3.0, 2.0]);
        assert_eq!(context.state_values_older, vec![1.0, 1.5]);
        assert_eq!(context.state_derivatives_prev[1], 0.75);
    }

    #[test]
    fn skipped_retry_discards_a_failed_nonfinite_integration_candidate() {
        let mut context = VmContext::with_states(0, 1);
        context
            .try_set_integration_coefficients(IntegrationCoefficients::backward_euler(1.0))
            .unwrap();
        context.state_values[0] = 4.0;
        context.state_values_prev[0] = 4.0;
        context.state_values_older[0] = 3.0;
        context.state_derivatives[0] = 2.0;
        context.state_derivatives_prev[0] = 2.0;
        context.state_initialized[0] = true;

        context.begin_stateful_evaluation();
        assert!(
            execute_with_context(
                &mut context,
                vec![Instruction::PushConst(f64::NAN), Instruction::DdtState(0)],
            )
            .unwrap()
            .is_nan()
        );
        assert!(context.validate_advance_state().is_err());

        context.begin_stateful_evaluation();
        context
            .advance_state()
            .expect("a clean retry that skips the operator must accept");
        assert_eq!(context.state_values[0].to_bits(), 4.0_f64.to_bits());
        assert_eq!(context.state_derivatives[0].to_bits(), 2.0_f64.to_bits());
        assert_eq!(context.state_values_prev[0].to_bits(), 4.0_f64.to_bits());
        assert_eq!(context.state_values_older[0].to_bits(), 3.0_f64.to_bits());
        assert!(context.accepted_checkpoint().is_ok());
    }

    #[test]
    fn nonfinite_operating_point_candidate_is_not_promoted_to_transient_history() {
        let mut context = VmContext::with_states(0, 1);

        assert_eq!(
            execute_with_context(
                &mut context,
                vec![Instruction::PushConst(f64::NAN), Instruction::DdtState(0)],
            )
            .unwrap()
            .to_bits(),
            0.0_f64.to_bits()
        );
        assert!(!context.state_initialized[0]);

        context.set_integration_coefficients(IntegrationCoefficients::backward_euler(1.0));

        assert!(!context.state_initialized[0]);
        assert_eq!(context.state_values[0].to_bits(), 0.0_f64.to_bits());
        assert_eq!(context.state_values_prev[0].to_bits(), 0.0_f64.to_bits());
        assert_eq!(context.state_values_older[0].to_bits(), 0.0_f64.to_bits());
        assert_eq!(context.state_derivatives[0].to_bits(), 0.0_f64.to_bits());
        assert_eq!(context.state_candidate_valid, vec![2]);
    }

    #[test]
    fn dynamic_limiter_allocation_keeps_every_state_pool_structurally_aligned() {
        let mut context = VmContext::default();
        assert_eq!(
            execute_with_context(
                &mut context,
                vec![
                    Instruction::PushConst(5.0),
                    Instruction::PushConst(1.0),
                    Instruction::LimitState(2),
                ],
            )
            .unwrap()
            .to_bits(),
            5.0_f64.to_bits()
        );

        assert_eq!(context.state_values.len(), 3);
        assert_eq!(context.state_values_prev.len(), 3);
        assert_eq!(context.state_values_older.len(), 3);
        assert_eq!(context.state_derivatives.len(), 3);
        assert_eq!(context.state_derivatives_prev.len(), 3);
        assert_eq!(context.state_initialized.len(), 3);
        assert_eq!(context.state_candidate_valid, vec![0; 3]);
        context.advance_state().unwrap();
        assert_eq!(context.state_values[2].to_bits(), 5.0_f64.to_bits());
    }
}
