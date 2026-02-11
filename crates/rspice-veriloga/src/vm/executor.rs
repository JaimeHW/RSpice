use super::{VmContext, VmError};
use crate::codegen::{BytecodeProgram, Instruction};

/// Stack-based virtual machine for bytecode execution.
pub struct Vm<'a> {
    /// Execution context
    pub context: &'a mut VmContext,
    /// Evaluation stack
    pub stack: Vec<f64>,
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

    /// Execute a single instruction.
    #[inline]
    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        match instruction {
            Instruction::PushConst(v) => {
                self.stack.push(*v);
            }
            Instruction::PushParam(idx) => {
                let v = self.context.parameters.get(*idx).copied().unwrap_or(0.0);
                self.stack.push(v);
            }
            Instruction::PushVoltage(pos, neg) => {
                self.stack.push(self.context.voltage(*pos, *neg));
            }
            Instruction::PushCurrent(pos, neg) => {
                self.stack.push(self.context.current(*pos, *neg));
            }
            Instruction::PushInternalVoltage(idx) => {
                self.stack.push(self.context.internal_voltage(*idx));
            }
            Instruction::PushVariable(idx) => {
                let v = self.context.variables.get(*idx).copied().unwrap_or(0.0);
                self.stack.push(v);
            }
            Instruction::PushTime => {
                self.stack.push(self.context.time);
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

            // Inverse trigonometric functions
            Instruction::Asin => self.unary_op(|a| a.asin())?,
            Instruction::Acos => self.unary_op(|a| a.acos())?,
            Instruction::Atan => self.unary_op(|a| a.atan())?,
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
                let result = if cond.abs() > 1e-15 {
                    then_val
                } else {
                    else_val
                };
                self.stack.push(result);
            }

            // Comparison operations (return 1.0 for true, 0.0 for false)
            Instruction::Gt => self.binary_op(|a, b| if a > b { 1.0 } else { 0.0 })?,
            Instruction::Lt => self.binary_op(|a, b| if a < b { 1.0 } else { 0.0 })?,
            Instruction::Ge => self.binary_op(|a, b| if a >= b { 1.0 } else { 0.0 })?,
            Instruction::Le => self.binary_op(|a, b| if a <= b { 1.0 } else { 0.0 })?,
            Instruction::Eq => {
                self.binary_op(|a, b| if (a - b).abs() < 1e-15 { 1.0 } else { 0.0 })?
            }
            Instruction::Ne => {
                self.binary_op(|a, b| if (a - b).abs() >= 1e-15 { 1.0 } else { 0.0 })?
            }

            // Logical operations
            Instruction::And => self.binary_op(|a, b| {
                if a.abs() > 1e-15 && b.abs() > 1e-15 {
                    1.0
                } else {
                    0.0
                }
            })?,
            Instruction::Or => self.binary_op(|a, b| {
                if a.abs() > 1e-15 || b.abs() > 1e-15 {
                    1.0
                } else {
                    0.0
                }
            })?,
            Instruction::Not => self.unary_op(|a| if a.abs() < 1e-15 { 1.0 } else { 0.0 })?,

            // State-based ddt: (current_expr - prev_state) / dt
            // The expression value is on the stack, we save it and compute derivative
            Instruction::DdtState(idx) => {
                let current_value = self.pop()?;
                let prev_value = self
                    .context
                    .state_values_prev
                    .get(*idx)
                    .copied()
                    .unwrap_or(current_value);

                // Save current value for next timestep
                if *idx < self.context.state_values.len() {
                    // Note: We can't mutate context here since it's borrowed
                    // The device layer should handle state updates
                }

                // Compute derivative: (current - prev) / dt
                // For DC analysis (dt=0), return 0
                let dt = self.context.timestep;
                let derivative = if dt.abs() > 1e-20 {
                    (current_value - prev_value) / dt
                } else {
                    0.0 // DC analysis: ddt = 0
                };
                self.stack.push(derivative);
            }

            // State-based idt: prev_state + expr * dt
            // Accumulates the integral over time
            Instruction::IdtState(idx) => {
                let current_value = self.pop()?;
                let prev_integral = self
                    .context
                    .state_values_prev
                    .get(*idx)
                    .copied()
                    .unwrap_or(0.0);

                let dt = self.context.timestep;
                // Forward Euler: integral += expr * dt
                let new_integral = if dt.abs() > 1e-20 {
                    prev_integral + current_value * dt
                } else {
                    prev_integral // DC: use accumulated value
                };
                self.stack.push(new_integral);
            }

            // $limit function: bounds value change per Newton iteration
            // Stack: [new_value, step_limit] -> [limited_value]
            // Uses state to track previous iteration value
            Instruction::LimitState(idx) => {
                let step_limit = self.pop()?;
                let new_value = self.pop()?;

                // Get previous value from state (or use new_value if first iteration)
                let prev_value = self
                    .context
                    .state_values_prev
                    .get(*idx)
                    .copied()
                    .unwrap_or(new_value);

                // Compute the change from previous iteration
                let delta = new_value - prev_value;

                // Clamp the delta to within [-step_limit, +step_limit]
                let limited_delta = if delta > step_limit {
                    step_limit
                } else if delta < -step_limit {
                    -step_limit
                } else {
                    delta
                };

                // Compute limited value
                let limited_value = prev_value + limited_delta;
                self.stack.push(limited_value);
            }

            // TableLookup: linear interpolation in lookup table
            // Stack: [input_value] -> [interpolated_value]
            // Uses context.lookup_tables for table storage
            Instruction::TableLookup(table_id) => {
                let input = self.pop()?;

                // Get the lookup table from context
                let result = if let Some(table) = self.context.lookup_tables.get(*table_id) {
                    // Use the commercial-grade LookupTable interpolation
                    table.interpolate(input)
                } else {
                    // Table not found - return 0 with warning
                    // In production, this should be caught at compile time
                    0.0
                };

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

                if is_transient {
                    self.context.delay_buffers[*buffer_id].record(current_time, current_value);
                }

                let result = if !is_transient || delay_time <= 0.0 {
                    current_value
                } else {
                    self.context.delay_buffers[*buffer_id].get_delayed(current_time, delay_time)
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
                    filter.update(
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
                    filter.update(input, time, max_pos, max_neg)
                };

                self.stack.push(result);
            }

            // CrossState: threshold crossing detection
            // Stack: [expr, direction] -> [0 or 1]
            Instruction::CrossState(detector_id) => {
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
                let crossed = detector.update(value, time, direction);

                // Cross events only trigger in transient analysis.
                let result = if is_transient { crossed } else { 0.0 };

                self.stack.push(result);
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
            // analysis_type encoding: 0=dc, 1=ac, 2=tran, 3=noise
            Instruction::Analysis(analysis_str_id) => {
                let current_type = self.context.analysis_type;
                let result = match analysis_str_id {
                    0 => {
                        // "dc" check
                        if current_type == 0 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    1 => {
                        // "ac" check
                        if current_type == 1 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    2 => {
                        // "tran" check
                        if current_type == 2 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    3 => {
                        // "noise" check
                        if current_type == 3 {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    _ => 0.0, // Unknown analysis type
                };
                self.stack.push(result);
            }

            // AboveState: level crossing event detection
            // Stack: [expr, threshold] -> [0 or 1]
            Instruction::AboveState(_detector_id) => {
                let threshold = self.pop()?;
                let value = self.pop()?;
                // For DC analysis, simply check if value > threshold
                // In transient, this would generate events on crossing
                let result = if value > threshold { 1.0 } else { 0.0 };
                self.stack.push(result);
            }

            // TimerState: periodic timer event
            // Stack: [start_time, period] -> [0 or 1]
            Instruction::TimerState(_timer_id) => {
                let period = self.pop()?;
                let start_time = self.pop()?;
                let current_time = self.context.time;

                // Timer fires when: time >= start_time AND
                // (time - start_time) is at a multiple of period
                let result = if current_time >= start_time && period > 0.0 {
                    let elapsed = current_time - start_time;
                    let cycles = (elapsed / period).floor();
                    let next_fire = start_time + cycles * period;
                    // Check if current time is at (or very close to) a fire point
                    let tolerance = self.context.timestep.max(1e-15);
                    if (current_time - next_fire).abs() < tolerance {
                        1.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                self.stack.push(result);
            }

            // LaplaceState: Laplace transfer function (both ZP and ND forms)
            // Stack: [input] -> [filtered_output]
            Instruction::LaplaceState(filter_id) => {
                let input = self.pop()?;
                let result = if self.context.analysis_type == 2 {
                    // Transient analysis
                    if let Some(filter) = self.context.laplace_filters.get_mut(*filter_id) {
                        filter.step(input, self.context.timestep)
                    } else {
                        input
                    }
                } else {
                    // DC and others (s=0)
                    if let Some(filter) = self.context.laplace_filters.get(*filter_id) {
                        filter.dc_output(input)
                    } else {
                        input
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
