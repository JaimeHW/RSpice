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

use super::{VmContext, VmError};
use crate::codegen::{BytecodeProgram, Instruction};

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
        let index = raw.round() as i64;
        let offset = index - lower;
        if offset < 0 || offset >= len as i64 {
            return Err(VmError::IndexOutOfBounds {
                index,
                lower,
                upper: lower + len as i64 - 1,
            });
        }
        Ok(base + offset as usize)
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
            Instruction::Shl => self.binary_op(|a, b| ((a as i64) << (b as i64)) as f64)?,
            Instruction::Shr => self.binary_op(|a, b| ((a as i64) >> (b as i64)) as f64)?,
            Instruction::BitAnd => self.binary_op(|a, b| ((a as i64) & (b as i64)) as f64)?,
            Instruction::BitOr => self.binary_op(|a, b| ((a as i64) | (b as i64)) as f64)?,
            Instruction::BitXor => self.binary_op(|a, b| ((a as i64) ^ (b as i64)) as f64)?,

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
                let coefficients = self.context.integration;
                let derivative = if coefficients.active {
                    coefficients.derivative_scale * current_value
                        - coefficients.previous_value_scale * prev_value
                        - coefficients.older_value_scale * older_value
                        - coefficients.previous_derivative_scale * previous_derivative
                } else {
                    0.0
                };
                self.context.state_derivatives[*idx] = derivative;
                self.context.state_initialized[*idx] = true;
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
                let coefficients = self.context.integration;
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
                self.context.state_initialized[*idx] = true;

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
                let coefficients = self.context.integration;
                let raw = if coefficients.active {
                    (current_value
                        + coefficients.previous_value_scale * prev
                        + coefficients.older_value_scale * older
                        + coefficients.previous_derivative_scale * previous_input)
                        / coefficients.derivative_scale
                } else {
                    ic
                };

                // Fold into [offset, offset + modulus)
                let wrapped = if modulus > 0.0 {
                    let phase = (raw - offset).rem_euclid(modulus);
                    offset + phase
                } else {
                    raw
                };

                self.context.state_values[*idx] = wrapped;
                self.context.state_derivatives[*idx] = current_value;
                self.context.state_initialized[*idx] = true;

                self.stack.push(wrapped);
            }

            // Companion Jacobian factor for ddt: a / dt (0 at DC)
            Instruction::DdtJacobian => {
                let coefficients = self.context.integration;
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
                let coefficients = self.context.integration;
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

                if self.context.state_values.len() <= *idx {
                    self.context.state_values.resize(*idx + 1, 0.0);
                }
                if self.context.state_initialized.len() <= *idx {
                    self.context.state_initialized.resize(*idx + 1, false);
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

                if self.context.delay_buffers.len() <= *buffer_id {
                    self.context
                        .delay_buffers
                        .resize_with(*buffer_id + 1, Default::default);
                }

                let result = if !is_transient || delay_time <= 0.0 {
                    current_value
                } else {
                    self.context.delay_buffers[*buffer_id].eval(
                        current_time,
                        current_value,
                        delay_time,
                    )
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
                let is_transient = self.context.analysis_type == 2;

                let result = if !is_transient {
                    input
                } else {
                    if self.context.transition_filters.len() <= *filter_id {
                        self.context
                            .transition_filters
                            .resize_with(*filter_id + 1, Default::default);
                    }
                    let filter = &mut self.context.transition_filters[*filter_id];
                    filter.eval(
                        input,
                        time,
                        delay.max(0.0),
                        rise_time.max(0.0),
                        fall_time.max(0.0),
                    )
                };

                self.stack.push(result);
            }

            // SlewState: slew rate limiting
            // Stack: [expr, max_pos_slew, max_neg_slew] -> [limited]
            Instruction::SlewState(filter_id) => {
                let max_neg_slew = self.pop()?;
                let max_pos_slew = self.pop()?;
                let input = self.pop()?;
                let time = self.context.time;
                let is_transient = self.context.analysis_type == 2;

                let result = if !is_transient {
                    input
                } else {
                    if self.context.slew_filters.len() <= *filter_id {
                        self.context
                            .slew_filters
                            .resize_with(*filter_id + 1, Default::default);
                    }
                    let filter = &mut self.context.slew_filters[*filter_id];
                    let max_pos = if max_pos_slew.is_finite() && max_pos_slew > 0.0 {
                        max_pos_slew
                    } else {
                        f64::INFINITY
                    };
                    let max_neg = if max_neg_slew.is_finite() && max_neg_slew > 0.0 {
                        max_neg_slew
                    } else {
                        f64::INFINITY
                    };
                    filter.eval(input, time, max_pos, max_neg)
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
                let direction = if direction > 0.5 {
                    1
                } else if direction < -0.5 {
                    -1
                } else {
                    0
                };
                let is_transient = self.context.analysis_type == 2;

                if self.context.cross_detectors.len() <= *detector_id {
                    self.context
                        .cross_detectors
                        .resize_with(*detector_id + 1, Default::default);
                }
                let detector = &mut self.context.cross_detectors[*detector_id];
                let crossed =
                    detector.eval_event(value, time, direction, time_tol, expr_tol, enable != 0.0);

                // Cross events only trigger in transient analysis.
                let result = if is_transient { crossed } else { 0.0 };

                self.stack.push(result);
            }

            // LastCrossingState: interpolated time of the last zero crossing
            // Stack: [expr, direction] -> [time or -1]
            Instruction::LastCrossingState(detector_id) => {
                let direction = self.pop()?;
                let value = self.pop()?;
                let direction = if direction > 0.5 {
                    1
                } else if direction < -0.5 {
                    -1
                } else {
                    0
                };
                if self.context.cross_detectors.len() <= *detector_id {
                    self.context
                        .cross_detectors
                        .resize_with(*detector_id + 1, Default::default);
                }
                let detector = &mut self.context.cross_detectors[*detector_id];
                let crossing_time =
                    detector.eval_last_crossing(value, self.context.time, direction);
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
                let result = detector.eval_above(
                    value,
                    self.context.time,
                    time_tol,
                    expr_tol,
                    enable != 0.0,
                );
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
                    self.context.timestep,
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
                    // Transient analysis
                    if let Some(filter) = self.context.laplace_filters.get_mut(*filter_id) {
                        filter.step(input, self.context.timestep).map_err(|error| {
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
        transient_context.timestep = 1.0;
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
}
