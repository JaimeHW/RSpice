//! Complex frequency-domain evaluation of differentiated Verilog-A bytecode.
//!
//! The ordinary VM deliberately stores real scalars because DC and transient
//! Newton systems are real.  AC and noise linearization are different: an
//! analog-operator derivative is a complex transfer action.  Keeping the two
//! components together is required for nested operators; evaluating separate
//! scalar "real" and "imaginary" passes loses the cross products in, for
//! example, two cascaded Laplace filters.

use super::{VmContext, VmError, idtmod_wrapped_value};
use crate::array_index::{ArrayIndexError, checked_array_slot, saturated_array_upper};
use crate::codegen::assignment_liveness::{AssignmentEffects, assignment_step_is_live};
use crate::codegen::{AssignmentStep, BytecodeProgram, Instruction, ZiRuntimeLayout};
use crate::complex_arithmetic::FrequencyValue;
use crate::integer_runtime::{IntegerBinaryOperation, integer_binary};
use crate::timing_contract::{NormalizedSlewRates, normalize_slew_rates};
use num_complex::Complex64;
use rspice_veriloga_runtime::arithmetic::IdtModOrigin;
use std::collections::BTreeMap;

const MAX_RUNTIME_LOOP_ITERATIONS: usize = 1_000_000;

struct SmallSignalScratch<V> {
    variables: Vec<V>,
    stack: Vec<V>,
    laplace_responses: Vec<Option<Complex64>>,
    zi_responses: Vec<Option<Complex64>>,
}

impl<V> SmallSignalScratch<V> {
    fn retained_bytes(&self) -> usize {
        self.variables
            .capacity()
            .saturating_mul(std::mem::size_of::<V>())
            .saturating_add(
                self.stack
                    .capacity()
                    .saturating_mul(std::mem::size_of::<V>()),
            )
            .saturating_add(
                self.laplace_responses
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Option<Complex64>>()),
            )
            .saturating_add(
                self.zi_responses
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Option<Complex64>>()),
            )
    }
}

thread_local! {
    // Shared by sequential device evaluations on this thread, without keeping
    // a second variable image in every circuit instance. Only capacities are
    // reused; no operating-point value or frequency response survives a call.
    static SMALL_SIGNAL_SCRATCH: std::cell::RefCell<SmallSignalScratch<Complex64>> = Default::default();
}

impl<V> Default for SmallSignalScratch<V> {
    fn default() -> Self {
        Self {
            variables: Vec::new(),
            stack: Vec::new(),
            laplace_responses: Vec::new(),
            zi_responses: Vec::new(),
        }
    }
}
trait FrequencyScalar: Copy + std::fmt::Debug + std::ops::Neg<Output = Self> {
    const TRACK_RANGE: bool;
    fn new(real: f64, imaginary: f64) -> Self;
    fn binary64(self) -> Complex64;
    fn is_zero(self) -> bool;
    fn circular_derivative(
        self,
        origin: &IdtModOrigin,
        phase: f64,
        modulus: f64,
        offset: f64,
        modulus_derivative: Self,
        range_lost: &mut bool,
    ) -> Result<Self, VmError>;
    fn is_real(self) -> bool;
    fn is_finite(self) -> bool;
    fn add(self, other: Self, range_lost: &mut bool) -> Self;
    fn subtract(self, other: Self, range_lost: &mut bool) -> Self;
    fn multiply(self, other: Self, range_lost: &mut bool) -> Self;
    fn divide(self, other: Self, range_lost: &mut bool) -> Self;
    fn scale(self, scale: f64, range_lost: &mut bool) -> Self;
    fn sum_products_div(
        pairs: &[[Self; 2]],
        divisor: Self,
        range_lost: &mut bool,
    ) -> Result<Self, VmError>;

    fn from_complex(value: Complex64) -> Self {
        Self::new(value.re, value.im)
    }
    fn take_scratch() -> SmallSignalScratch<Self> {
        SmallSignalScratch::default()
    }
    fn recycle_scratch(_scratch: SmallSignalScratch<Self>) {}
}

impl FrequencyScalar for Complex64 {
    const TRACK_RANGE: bool = true;
    fn is_zero(self) -> bool {
        self.re == 0.0 && self.im == 0.0
    }
    fn circular_derivative(
        self,
        origin: &IdtModOrigin,
        phase: f64,
        modulus: f64,
        offset: f64,
        modulus_derivative: Self,
        range_lost: &mut bool,
    ) -> Result<Self, VmError> {
        let value = FrequencyValue::from_complex(self)
            .circular_derivative(
                origin,
                phase,
                modulus,
                offset,
                FrequencyValue::from_complex(modulus_derivative),
            )
            .map_err(|detail| VmError::InvalidNumericResult(detail.into()))?;
        let result = value.binary64();
        if !value.has_regular_components() || !result.is_finite() {
            *range_lost = true;
        }
        Ok(result)
    }
    fn sum_products_div(
        pairs: &[[Self; 2]],
        divisor: Self,
        range_lost: &mut bool,
    ) -> Result<Self, VmError> {
        let value = crate::complex_arithmetic::sum_complex_products_div(pairs, divisor);
        let real_product = pairs
            .iter()
            .any(|[a, b]| (a.re != 0.0 && b.re != 0.0) || (a.im != 0.0 && b.im != 0.0));
        let imaginary_product = pairs
            .iter()
            .any(|[a, b]| (a.re != 0.0 && b.im != 0.0) || (a.im != 0.0 && b.re != 0.0));
        let real_zero =
            !(real_product && divisor.re != 0.0 || imaginary_product && divisor.im != 0.0);
        let imaginary_zero =
            !(imaginary_product && divisor.re != 0.0 || real_product && divisor.im != 0.0);
        if !(value.re.is_normal() || (value.re == 0.0 && real_zero))
            || !(value.im.is_normal() || (value.im == 0.0 && imaginary_zero))
        {
            *range_lost = true;
        }
        Ok(value)
    }
    #[inline]
    fn new(real: f64, imaginary: f64) -> Self {
        Self::new(real, imaginary)
    }
    #[inline]
    fn binary64(self) -> Self {
        self
    }
    #[inline]
    fn is_real(self) -> bool {
        self.im == 0.0
    }
    #[inline]
    fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
    #[inline]
    fn add(self, other: Self, range_lost: &mut bool) -> Self {
        let value = self + other;
        if !value.is_finite() {
            *range_lost = true;
        }
        value
    }
    #[inline]
    fn subtract(self, other: Self, range_lost: &mut bool) -> Self {
        self.add(-other, range_lost)
    }
    #[inline]
    fn multiply(self, other: Self, range_lost: &mut bool) -> Self {
        let value = crate::complex_arithmetic::multiply_complex(self, other);
        if !FrequencyValue::regular_result(value, self, other) {
            *range_lost = true;
        }
        value
    }
    #[inline]
    fn divide(self, other: Self, range_lost: &mut bool) -> Self {
        let value = crate::complex_arithmetic::divide_complex(self, other);
        if !FrequencyValue::regular_result(value, self, other) {
            *range_lost = true;
        }
        value
    }
    #[inline]
    fn scale(self, scale: f64, range_lost: &mut bool) -> Self {
        let value = self * scale;
        if !((value.re.is_normal() || (value.re == 0.0 && (self.re == 0.0 || scale == 0.0)))
            && (value.im.is_normal() || (value.im == 0.0 && (self.im == 0.0 || scale == 0.0))))
        {
            *range_lost = true;
        }
        value
    }
    #[inline]
    fn take_scratch() -> SmallSignalScratch<Self> {
        SMALL_SIGNAL_SCRATCH
            .try_with(|cache| {
                cache
                    .try_borrow_mut()
                    .map(|mut cache| std::mem::take(&mut *cache))
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
    #[inline]
    fn recycle_scratch(scratch: SmallSignalScratch<Self>) {
        if scratch.retained_bytes() <= 1024 * 1024 {
            let _ = SMALL_SIGNAL_SCRATCH.try_with(|cache| {
                if let Ok(mut cache) = cache.try_borrow_mut() {
                    if scratch.retained_bytes() >= cache.retained_bytes() {
                        *cache = scratch;
                    }
                }
            });
        }
    }
}

impl FrequencyScalar for FrequencyValue {
    const TRACK_RANGE: bool = false;
    fn is_zero(self) -> bool {
        self.is_zero()
    }
    fn circular_derivative(
        self,
        origin: &IdtModOrigin,
        phase: f64,
        modulus: f64,
        offset: f64,
        modulus_derivative: Self,
        _range_lost: &mut bool,
    ) -> Result<Self, VmError> {
        self.circular_derivative(origin, phase, modulus, offset, modulus_derivative)
            .map_err(|detail| VmError::InvalidNumericResult(detail.into()))
    }
    fn sum_products_div(
        pairs: &[[Self; 2]],
        divisor: Self,
        _range_lost: &mut bool,
    ) -> Result<Self, VmError> {
        Self::sum_products_div(pairs, divisor)
            .map_err(|error| VmError::InvalidNumericResult(format!("complex quotient: {error:?}")))
    }
    fn new(real: f64, imaginary: f64) -> Self {
        Self::new(real, imaginary)
    }
    fn binary64(self) -> Complex64 {
        self.binary64()
    }
    fn is_real(self) -> bool {
        self.is_real()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn add(self, other: Self, _range_lost: &mut bool) -> Self {
        self + other
    }
    fn subtract(self, other: Self, _range_lost: &mut bool) -> Self {
        self - other
    }
    fn multiply(self, other: Self, _range_lost: &mut bool) -> Self {
        self.multiply(other)
    }
    fn divide(self, other: Self, _range_lost: &mut bool) -> Self {
        self.divide(other)
    }
    fn scale(self, scale: f64, _range_lost: &mut bool) -> Self {
        self * scale
    }
}

/// Read-only AC/noise evaluation with an ordinary binary64 path and a cold
/// retry when an intermediate needs an exponent outside binary64's range.
/// Both paths use the same dispatcher and replay from the same immutable seed.
pub(crate) struct SmallSignalVm<'a> {
    ordinary: SmallSignalEngine<'a, Complex64>,
    wide: Option<Box<SmallSignalEngine<'a, FrequencyValue>>>,
    seed: &'a [f64],
    assignments: Option<&'a [AssignmentStep]>,
    assignment_liveness: Option<&'a [bool]>,
}

impl<'a> SmallSignalVm<'a> {
    #[cfg(test)]
    fn new(context: &'a VmContext, frequency_hz: f64) -> Result<Self, VmError> {
        Self::with_variable_seed(context, frequency_hz, &context.variables)
    }

    #[inline]
    pub(crate) fn with_variable_seed(
        context: &'a VmContext,
        frequency_hz: f64,
        seed: &'a [f64],
    ) -> Result<Self, VmError> {
        Ok(Self {
            ordinary: SmallSignalEngine::with_variable_seed(context, frequency_hz, seed)?,
            wide: None,
            seed,
            assignments: None,
            assignment_liveness: None,
        })
    }

    #[inline]
    pub(crate) fn execute_assignments(
        &mut self,
        steps: &'a [AssignmentStep],
    ) -> Result<(), VmError> {
        self.execute_live_assignments(steps, None)
    }

    /// Replay only variables required by the caller's frequency-domain outputs.
    /// The same selection is retained if extended-range recovery replays it.
    pub(crate) fn execute_live_assignments(
        &mut self,
        steps: &'a [AssignmentStep],
        live: Option<&'a [bool]>,
    ) -> Result<(), VmError> {
        if self.wide.is_some() || self.assignments.is_some() {
            // A second assignment stream starts from the first stream's final
            // image. Promote before it, so a later retry cannot omit that state.
            self.promote()?;
            return self
                .wide
                .as_mut()
                .expect("promoted engine")
                .execute_assignment_steps(steps, live);
        }
        self.assignments = Some(steps);
        self.assignment_liveness = live;
        let result = self.ordinary.execute_assignment_steps(steps, live);
        if self.ordinary.range_lost {
            self.promote()
        } else {
            result
        }
    }

    #[cfg(test)]
    fn execute(&mut self, program: &BytecodeProgram) -> Result<Complex64, VmError> {
        self.execute_scaled(program, 1.0)
    }

    #[inline]
    pub(crate) fn execute_scaled(
        &mut self,
        program: &BytecodeProgram,
        scale: f64,
    ) -> Result<Complex64, VmError> {
        if let Some(wide) = &mut self.wide {
            return wide.execute_scaled(program, scale);
        }
        let result = self.ordinary.execute_scaled(program, scale);
        if !self.ordinary.range_lost {
            return result;
        }
        self.promote()?;
        self.wide
            .as_mut()
            .expect("promoted engine")
            .execute_scaled(program, scale)
    }

    #[cold]
    #[inline(never)]
    fn promote(&mut self) -> Result<(), VmError> {
        if self.wide.is_none() {
            let mut wide = Box::new(SmallSignalEngine::with_variable_seed(
                self.ordinary.context,
                self.ordinary.frequency_hz,
                self.seed,
            )?);
            if let Some(steps) = self.assignments {
                wide.execute_assignment_steps(steps, self.assignment_liveness)?;
            }
            self.wide = Some(wide);
        }
        Ok(())
    }
}

/// Read-only complex evaluator used only for frequency-domain Jacobians.
///
/// A private complex variable image replays the assignment stream so
/// forward-mode derivative shadows retain the phase introduced by dynamic
/// operators.  The real runtime context has already completed its normal
/// native/VM/WASM operating-point evaluation and is never mutated here.
struct SmallSignalEngine<'a, V: FrequencyScalar> {
    context: &'a VmContext,
    variables: Vec<V>,
    stack: Vec<V>,
    // A VM borrows one immutable operating-point context at one frequency.
    // Cache transfer responses across assignment and Jacobian programs, while
    // keeping each instruction's input/action outside the cache.
    laplace_responses: Vec<Option<Complex64>>,
    zi_responses: Vec<Option<Complex64>>,
    frequency_hz: f64,
    omega: f64,
    range_lost: bool,
    integral_origins: BTreeMap<usize, IdtModOrigin>,
}

impl<V: FrequencyScalar> Drop for SmallSignalEngine<'_, V> {
    #[inline]
    fn drop(&mut self) {
        self.variables.clear();
        self.stack.clear();
        self.laplace_responses.clear();
        self.zi_responses.clear();
        let scratch = SmallSignalScratch {
            variables: std::mem::take(&mut self.variables),
            stack: std::mem::take(&mut self.stack),
            laplace_responses: std::mem::take(&mut self.laplace_responses),
            zi_responses: std::mem::take(&mut self.zi_responses),
        };
        V::recycle_scratch(scratch);
    }
}
impl<'a, V: FrequencyScalar> SmallSignalEngine<'a, V> {
    #[cfg(test)]
    pub(crate) fn new(context: &'a VmContext, frequency_hz: f64) -> Result<Self, VmError> {
        Self::with_variable_seed(context, frequency_hz, &context.variables)
    }

    #[inline]
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
        let mut scratch = V::take_scratch();
        scratch.variables.extend(
            variable_seed
                .iter()
                .copied()
                .map(|value| V::new(value, 0.0)),
        );
        Ok(Self {
            context,
            variables: scratch.variables,
            stack: scratch.stack,
            laplace_responses: scratch.laplace_responses,
            zi_responses: scratch.zi_responses,
            frequency_hz,
            omega,
            range_lost: false,
            integral_origins: BTreeMap::new(),
        })
    }

    pub(crate) fn execute_scaled(
        &mut self,
        program: &BytecodeProgram,
        scale: f64,
    ) -> Result<Complex64, VmError> {
        let value = self.execute_value(program)?;
        let result = self.scale_value(value, scale).binary64();
        if !result.re.is_finite() || !result.im.is_finite() {
            return Err(VmError::InvalidNumericResult(format!(
                "small-signal bytecode produced non-finite result {}+j{}",
                result.re, result.im
            )));
        }
        Ok(result)
    }

    fn execute_value(&mut self, program: &BytecodeProgram) -> Result<V, VmError> {
        self.stack.clear();
        let mut pc = 0;
        while let Some(instruction) = program.instructions.get(pc) {
            pc += 1;
            let skip = match instruction {
                Instruction::JumpIfFalse(skip) => {
                    self.check_range()?;
                    if self.pop_real("conditional jump")? == 0.0 {
                        *skip
                    } else {
                        0
                    }
                }
                Instruction::Jump(skip) => *skip,
                _ => {
                    self.execute_instruction(instruction)?;
                    continue;
                }
            };
            pc = pc
                .checked_add(skip)
                .filter(|end| *end <= program.instructions.len())
                .ok_or(VmError::InvalidInstruction(
                    "conditional jump is outside bytecode",
                ))?;
        }
        // Arithmetic only updates a sticky flag. Check once at each program
        // boundary (and before conditional jumps), instead of returning an
        // extra Result from every numerical operation. No assignment can
        // publish a value from a program that lost range.
        self.check_range()?;
        let result = self
            .stack
            .pop()
            .ok_or(VmError::StackUnderflow("No small-signal result on stack"))?;
        if !self.stack.is_empty() {
            return Err(VmError::InvalidInstruction(
                "small-signal bytecode left extra values on the stack",
            ));
        }
        if !result.is_finite() {
            return Err(VmError::InvalidNumericResult(format!(
                "small-signal bytecode produced non-finite intermediate {result:?}"
            )));
        }
        Ok(result)
    }

    fn execute_assignment_steps(
        &mut self,
        steps: &[AssignmentStep],
        live: Option<&[bool]>,
    ) -> Result<(), VmError> {
        if live.is_some_and(<[bool]>::is_empty) {
            return Ok(());
        }
        for step in steps {
            if live.is_some_and(|live| {
                !assignment_step_is_live(step, live, AssignmentEffects::SkipTasks)
            }) {
                continue;
            }
            match step {
                // Linearization replays numerical assignments, not task effects.
                AssignmentStep::Task(_) | AssignmentStep::Initialization { .. } => {}
                AssignmentStep::Assign(assignment) => {
                    let value = self.execute_value(&assignment.program)?;
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
                    let index_value = self.execute_value(index)?;
                    let raw = self.real_value(index_value, "array index")?;
                    let slot = Self::array_slot(raw, *base, *len, *lower)?;
                    let value = self.execute_value(value)?;
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
                        let condition_value = self.execute_value(condition)?;
                        let active = self.real_value(condition_value, "runtime-loop condition")?;
                        if active == 0.0 {
                            break;
                        }
                        self.execute_assignment_steps(body, live)?;
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
    fn pop(&mut self, operation: &'static str) -> Result<V, VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow(operation))
    }

    fn real_value(&self, value: V, label: &str) -> Result<f64, VmError> {
        if !value.is_real() {
            return Err(VmError::InvalidNumericResult(format!(
                "{label} is not a real operating-point value during small-signal evaluation: {value:?}"
            )));
        }
        let real = value.binary64().re;
        // Preserve ordinary IEEE operands, including an authored `inf` in a
        // comparison. Only a finite extended value that cannot be represented
        // as an operating-point scalar needs this conversion error.
        if !V::TRACK_RANGE && value.is_finite() && !real.is_finite() {
            return Err(VmError::InvalidNumericResult(format!(
                "{label} is outside the finite operating-point range"
            )));
        }
        Ok(real)
    }

    fn pop_real(&mut self, operation: &'static str) -> Result<f64, VmError> {
        let value = self.pop(operation)?;
        self.real_value(value, operation)
    }

    fn unary(&mut self, operation: &'static str, f: impl FnOnce(V) -> V) -> Result<(), VmError> {
        let value = self.pop(operation)?;
        self.stack.push(f(value));
        Ok(())
    }

    fn binary(
        &mut self,
        operation: &'static str,
        f: impl FnOnce(V, V, &mut bool) -> V,
    ) -> Result<(), VmError> {
        let right = self.pop(operation)?;
        let left = self.pop(operation)?;
        let result = f(left, right, &mut self.range_lost);
        self.stack.push(result);
        Ok(())
    }

    #[inline]
    fn check_range(&self) -> Result<(), VmError> {
        if V::TRACK_RANGE && self.range_lost {
            return Err(VmError::InvalidNumericResult(
                "small-signal arithmetic requires exponent-range recovery".into(),
            ));
        }
        Ok(())
    }

    #[inline]
    fn multiply_values(&mut self, left: V, right: V) -> V {
        V::multiply(left, right, &mut self.range_lost)
    }

    #[inline]
    fn scale_value(&mut self, value: V, scale: f64) -> V {
        if scale == 1.0 {
            return value;
        }
        V::scale(value, scale, &mut self.range_lost)
    }

    fn unary_real(
        &mut self,
        operation: &'static str,
        f: impl FnOnce(f64) -> f64,
    ) -> Result<(), VmError> {
        let value = self.pop_real(operation)?;
        self.stack.push(V::new(f(value), 0.0));
        Ok(())
    }

    fn binary_real(
        &mut self,
        operation: &'static str,
        f: impl FnOnce(f64, f64) -> f64,
    ) -> Result<(), VmError> {
        let right = self.pop_real(operation)?;
        let left = self.pop_real(operation)?;
        self.stack.push(V::new(f(left, right), 0.0));
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
        self.stack.push(V::new(value, 0.0));
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
            if index != operands.len() - 2 && !operand.is_real() {
                return Err(VmError::InvalidNumericResult(format!(
                    "Zi definition/timing operand {index} is complex during small-signal evaluation"
                )));
            }
        }
        let action = operands[operands.len() - 2];
        let transition = self.real_value(operands[operands.len() - 1], "Zi transition time")?;
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
        let map_error = |error| {
            VmError::InvalidNumericResult(format!("zi filter {}: {error}", layout.filter_id))
        };
        let response = if derivative {
            cached_filter_response(
                &mut self.zi_responses,
                self.context.zi_filters.len(),
                layout.filter_id,
                || {
                    filter
                        .frequency_response_rectangular(self.frequency_hz)
                        .map_err(map_error)
                },
            )
            .map(V::from_complex)?
        } else {
            V::new(filter.dc_gain().map_err(map_error)?, 0.0)
        };
        self.stack.truncate(start);
        let result = self.multiply_values(response, action);
        self.stack.push(result);

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
            self.multiply_values(
                V::from_complex(Complex64::from_polar(1.0, phase)),
                input_derivative.expect("derivative operand was decoded"),
            )
        } else {
            V::new(input_real, 0.0)
        };
        self.stack.push(result);
        Ok(())
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), VmError> {
        match instruction {
            Instruction::JumpIfFalse(_) | Instruction::Jump(_) => {
                return Err(VmError::InvalidInstruction(
                    "conditional jump requires a bytecode program",
                ));
            }
            Instruction::PushConst(value) => self.stack.push(V::new(*value, 0.0)),
            Instruction::PushParam(index) => {
                let value = self
                    .context
                    .parameters
                    .get(*index)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing parameter slot"))?;
                self.stack.push(V::new(value, 0.0));
            }
            Instruction::PushParamGiven(index) => {
                let value = self
                    .context
                    .param_given
                    .get(*index)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing parameter-given slot"))?;
                self.stack.push(V::new(f64::from(value != 0), 0.0));
            }
            Instruction::PushBranchCurrent(index) => {
                let value = self
                    .context
                    .branch_current_values
                    .get(*index)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing branch-current slot"))?;
                self.stack.push(V::new(value, 0.0));
            }
            Instruction::PushVoltage(pos, neg) => {
                self.stack
                    .push(V::new(self.context.try_voltage(*pos, *neg)?, 0.0));
            }
            Instruction::PushCurrent(pos, neg) => {
                self.stack
                    .push(V::new(self.context.try_current(*pos, *neg)?, 0.0));
            }
            Instruction::PushInternalVoltage(index) => {
                let value = self
                    .context
                    .internal_voltages
                    .get(*index)
                    .copied()
                    .ok_or(VmError::InvalidInstruction("missing internal-voltage slot"))?;
                self.stack.push(V::new(value, 0.0));
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
                self.stack.push(V::new(self.context.temperature, 0.0));
            }
            Instruction::PushVt => {
                self.stack.push(V::new(self.context.vt(), 0.0));
            }
            Instruction::PushTime => {
                self.stack.push(V::new(self.context.time, 0.0));
            }
            Instruction::PushSimParamValue(parameter) => {
                self.stack
                    .push(V::new(self.context.simparam(*parameter)?, 0.0));
            }
            Instruction::PushSimParamPresent(parameter) => {
                self.stack.push(V::new(
                    f64::from(
                        self.context
                            .simulation_parameters
                            .get_parameter(*parameter)
                            .is_some(),
                    ),
                    0.0,
                ));
            }
            Instruction::PushMfactor => {
                self.stack.push(V::new(self.context.multiplicity, 0.0));
            }
            Instruction::PushPortConnected(terminal) => self.stack.push(V::new(
                f64::from(self.context.port_connected(*terminal)),
                0.0,
            )),
            Instruction::ZiState(layout) => self.execute_zi(*layout, false)?,
            Instruction::ZiStateDerivative(layout) => self.execute_zi(*layout, true)?,

            Instruction::IntegerArithmetic(op) => self.integer_binary(
                IntegerBinaryOperation::Arithmetic(*op),
                "integer arithmetic",
            )?,
            Instruction::CheckedValue => {
                let derivative = self.pop("ddx")?;
                let primal = self.pop("ddx")?;
                if !primal.is_finite() {
                    return Err(VmError::InvalidNumericResult(
                        "ddx operand is not finite".into(),
                    ));
                }
                if !derivative.is_finite() {
                    return Err(VmError::InvalidNumericResult(
                        "ddx derivative is not finite".into(),
                    ));
                }
                self.stack.push(derivative);
            }
            Instruction::Add => self.binary("Add", V::add)?,
            Instruction::Sub => self.binary("Sub", V::subtract)?,
            Instruction::Mul => self.binary("Mul", V::multiply)?,
            Instruction::Div => self.binary("Div", V::divide)?,
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
            Instruction::Min => self.binary_real("Min", rspice_veriloga_runtime::rspice_min)?,
            Instruction::Max => self.binary_real("Max", rspice_veriloga_runtime::rspice_max)?,
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
            Instruction::Hypot => self.binary_real("Hypot", f64::hypot)?,
            Instruction::SumProductsDiv(terms) => {
                let count = terms.checked_mul(2).and_then(|n| n.checked_add(1)).ok_or(
                    VmError::InvalidInstruction("sum-products quotient count overflow"),
                )?;
                let start = self
                    .stack
                    .len()
                    .checked_sub(count)
                    .ok_or(VmError::StackUnderflow("sum-products quotient"))?;
                let divisor = self.stack[start + count - 1];
                let (pairs, remainder) = self.stack[start..start + count - 1].as_chunks::<2>();
                debug_assert!(remainder.is_empty());
                let result = V::sum_products_div(pairs, divisor, &mut self.range_lost)?;
                self.stack.truncate(start);
                self.stack.push(result);
            }
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
                self.stack.push(V::new(0.0, 0.0));
            }
            Instruction::IdtState(_) => {
                let initial = self.pop_real("IdtState initial condition")?;
                let _input = self.pop_real("IdtState input")?;
                self.stack.push(V::new(initial, 0.0));
            }
            Instruction::IdtModState(slot) => {
                let offset = self.pop_real("IdtModState offset")?;
                let modulus = self.pop_real("IdtModState modulus")?;
                let initial = self.pop_real("IdtModState initial condition")?;
                let _input = self.pop_real("IdtModState input")?;
                let wrapped = idtmod_wrapped_value(initial, modulus, offset).map_err(
                    |detail| {
                        VmError::InvalidNumericResult(format!(
                            "idtmod small-signal operating point {detail}: initial={initial}, modulus={modulus}, offset={offset}"
                        ))
                    },
                )?;
                let origin = IdtModOrigin::ZERO
                    .rebased(initial, wrapped)
                    .map_err(|detail| VmError::InvalidNumericResult(detail.into()))?;
                self.integral_origins.insert(*slot, origin);
                self.stack.push(V::new(wrapped, 0.0));
            }
            Instruction::DdtJacobian => {
                let input = self.pop("DdtJacobian")?;
                let result = self.multiply_values(V::new(0.0, self.omega), input);
                self.stack.push(result);
            }
            Instruction::IdtJacobian => {
                let input = self.pop("IdtJacobian")?;
                if self.omega == 0.0 {
                    return Err(VmError::InvalidNumericResult(
                        "idt/idtmod small-signal transfer is singular at zero frequency".into(),
                    ));
                }
                let result = V::divide(input, V::new(0.0, self.omega), &mut self.range_lost);
                self.stack.push(result);
            }
            Instruction::IdtDerivativeState(slot) | Instruction::IdtModDerivativeState(slot) => {
                let wrapped = matches!(instruction, Instruction::IdtModDerivativeState(_));
                let modulus_derivative = if wrapped {
                    self.pop("integral modulus derivative")?
                } else {
                    V::new(0.0, 0.0)
                };
                let _ic_derivative = self.pop("integral initial-condition derivative")?;
                let input = self.pop("integral input derivative")?;
                let offset = if wrapped {
                    self.pop_real("integral offset")?
                } else {
                    0.0
                };
                let modulus = if wrapped {
                    self.pop_real("integral modulus")?
                } else {
                    1.0
                };
                let primal = self.pop_real("integral primal")?;
                let mut result = if input.is_zero() {
                    input
                } else {
                    if self.omega == 0.0 {
                        return Err(VmError::InvalidNumericResult(
                            "idt/idtmod small-signal transfer is singular at zero frequency".into(),
                        ));
                    }
                    V::divide(input, V::new(0.0, self.omega), &mut self.range_lost)
                };
                self.check_range()?;
                if wrapped {
                    let origin =
                        self.integral_origins
                            .get(slot)
                            .ok_or(VmError::InvalidInstruction(
                                "circular integral derivative has no operating-point origin",
                            ))?;
                    result = result.circular_derivative(
                        origin,
                        primal,
                        modulus,
                        offset,
                        modulus_derivative,
                        &mut self.range_lost,
                    )?;
                }
                self.stack.push(result);
            }
            Instruction::TableDerivative(table_id) => {
                let input = self.pop_real("TableDerivative")?;
                let table = self
                    .context
                    .lookup_tables
                    .get(*table_id)
                    .ok_or(VmError::InvalidInstruction("missing lookup table"))?;
                self.stack.push(V::new(table.derivative(input), 0.0));
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
                self.stack.push(V::new(table.interpolate(input), 0.0));
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
                self.stack.push(V::new(input, 0.0));
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
                self.stack.push(V::new(input, 0.0));
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
                let gain = filter.small_signal_input_gain(self.context.time);
                let result = self.scale_value(derivative, gain);
                self.stack.push(result);
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
                self.stack.push(V::new(0.0, 0.0));
            }
            Instruction::LastCrossingState(_) => {
                let direction = self.pop_real("last_crossing direction")?;
                let input = self.pop_real("last_crossing input")?;
                // Freeze the operator at its static value, retaining the same
                // operand checks as transient and generated noise evaluation.
                rspice_veriloga_runtime::evaluate_generated_last_crossing(
                    rspice_veriloga_runtime::GeneratedCrossState::INITIAL,
                    input,
                    0.0,
                    direction,
                )
                .map_err(|error| {
                    VmError::InvalidNumericResult(format!(
                        "last_crossing evaluation failed: {error}"
                    ))
                })?;
                self.stack.push(V::new(-1.0, 0.0));
            }
            Instruction::WhiteNoise => {
                let _power = self.pop_real("white_noise power")?;
                self.stack.push(V::new(0.0, 0.0));
            }
            Instruction::FlickerNoise => {
                let _exponent = self.pop_real("flicker_noise exponent")?;
                let _power = self.pop_real("flicker_noise power")?;
                self.stack.push(V::new(0.0, 0.0));
            }
            Instruction::Analysis(kind) => {
                let bit = 1_u32.checked_shl(u32::from(*kind)).unwrap_or(0);
                let active = self.context.analysis_query_mask() & bit != 0;
                self.stack.push(V::new(f64::from(active), 0.0));
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
                self.stack.push(V::new(0.0, 0.0));
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
                self.stack.push(V::new(0.0, 0.0));
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
                let result = self.scale_value(input, gain);
                self.stack.push(result);
            }
            Instruction::LaplaceStateDerivative(filter_id) => {
                let input = self.pop("LaplaceStateDerivative")?;
                let filter = self
                    .context
                    .laplace_filters
                    .get(*filter_id)
                    .ok_or(VmError::InvalidInstruction("missing laplace filter"))?;
                let response = cached_filter_response(
                    &mut self.laplace_responses,
                    self.context.laplace_filters.len(),
                    *filter_id,
                    || {
                        filter
                            .frequency_response_rectangular(self.frequency_hz)
                            .map_err(|error| {
                                VmError::InvalidNumericResult(format!(
                                    "Laplace filter {filter_id}: {error}"
                                ))
                            })
                    },
                )?;
                let result = self.multiply_values(V::from_complex(response), input);
                self.stack.push(result);
            }
        }
        Ok(())
    }
}

/// The caller has resolved `filter_id` in the immutable context before lookup.
fn cached_filter_response(
    cache: &mut Vec<Option<Complex64>>,
    filter_count: usize,
    filter_id: usize,
    evaluate: impl FnOnce() -> Result<(f64, f64), VmError>,
) -> Result<Complex64, VmError> {
    if let Some(Some(response)) = cache.get(filter_id) {
        return Ok(*response);
    }
    let (real, imaginary) = evaluate()?;
    let response = Complex64::new(real, imaginary);
    cache.resize(filter_count, None);
    cache[filter_id] = Some(response);
    Ok(response)
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
    fn last_crossing_checks_frozen_operands_without_history() {
        let context = ac_context();
        let program = |input, direction| BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(input),
                Instruction::PushConst(direction),
                Instruction::LastCrossingState(0),
            ],
        };
        for frequency in [0.0, 1.0, 1e9] {
            for input in [-f64::MAX, 0.0, f64::MAX] {
                for direction in [-1.0, 0.0, 1.0] {
                    let mut vm = SmallSignalVm::new(&context, frequency).unwrap();
                    assert_eq!(
                        vm.execute(&program(input, direction)).unwrap(),
                        Complex64::new(-1.0, 0.0)
                    );
                }
            }
            for (input, direction) in [
                (1.0, -2.0),
                (1.0, 2.0),
                (1.0, 0.5),
                (1.0, f64::NAN),
                (1.0, f64::INFINITY),
                (1.0, f64::NEG_INFINITY),
                (f64::NAN, 0.0),
                (f64::INFINITY, 0.0),
                (f64::NEG_INFINITY, 0.0),
            ] {
                let mut vm = SmallSignalVm::new(&context, frequency).unwrap();
                let error = vm.execute(&program(input, direction)).expect_err(
                    "invalid last_crossing operands must not become a finite static sentinel",
                );
                assert!(error.to_string().contains("last_crossing"), "{error}");
            }
        }
        assert!(context.cross_detectors.is_empty());
    }

    #[test]
    fn selective_replay_retains_runtime_loop_and_dynamic_array_dependencies() {
        use crate::codegen::AssignmentProgram;
        use crate::codegen::assignment_liveness::{
            mark_program_variable_reads, propagate_live_assignment_slots,
        };

        let program = |instructions| BytecodeProgram { instructions };
        let assign = |var_index, instructions| {
            AssignmentStep::Assign(AssignmentProgram {
                var_index,
                program: program(instructions),
            })
        };
        let dead_pole = program(vec![Instruction::PushConst(1.0), Instruction::IdtJacobian]);
        let steps = [
            assign(0, vec![Instruction::PushConst(0.0)]),
            AssignmentStep::Loop {
                condition: program(vec![
                    Instruction::PushVariable(0),
                    Instruction::PushConst(2.0),
                    Instruction::Lt,
                ]),
                body: vec![
                    AssignmentStep::AssignIndexed {
                        base: 3,
                        len: 2,
                        lower: 0,
                        index: program(vec![Instruction::PushVariable(0)]),
                        value: program(vec![
                            Instruction::PushVariable(0),
                            Instruction::PushConst(1.0),
                            Instruction::Add,
                        ]),
                    },
                    assign(
                        0,
                        vec![
                            Instruction::PushVariable(0),
                            Instruction::PushConst(1.0),
                            Instruction::Add,
                        ],
                    ),
                ],
            },
            assign(
                5,
                vec![
                    Instruction::PushConst(1.0),
                    Instruction::PushVariableDyn {
                        base: 3,
                        len: 2,
                        lower: 0,
                    },
                ],
            ),
            // Neither a dead assignment nor its enclosing loop condition may
            // introduce a pole into the requested output.
            AssignmentStep::Loop {
                condition: dead_pole.clone(),
                body: vec![AssignmentStep::Assign(AssignmentProgram {
                    var_index: 2,
                    program: dead_pole,
                })],
            },
        ];
        let output = program(vec![Instruction::PushVariable(5)]);
        let mut live = vec![false; 6];
        mark_program_variable_reads(&output, &mut live);
        propagate_live_assignment_slots(&steps, &mut live, AssignmentEffects::SkipTasks);
        let mut context = ac_context();
        context.variables = vec![0.0; 6];
        let mut vm = SmallSignalVm::new(&context, 0.0).unwrap();
        vm.execute_live_assignments(&steps, Some(&live)).unwrap();
        assert_eq!(vm.execute(&output).unwrap(), Complex64::new(2.0, 0.0));
        assert_eq!(context.variables, [0.0; 6]);
    }

    #[test]
    fn range_recovery_preserves_the_assignment_selection_and_original_seed() {
        use crate::codegen::AssignmentProgram;
        let mut context = ac_context();
        context.variables = vec![2.0, 0.0, 0.0];
        let product = BytecodeProgram {
            instructions: vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(1e200),
                Instruction::Mul,
                Instruction::PushConst(1e200),
                Instruction::Mul,
            ],
        };
        let steps = [
            AssignmentStep::Assign(AssignmentProgram {
                var_index: 0,
                program: BytecodeProgram {
                    instructions: vec![
                        Instruction::PushVariable(0),
                        Instruction::PushConst(1.0),
                        Instruction::Add,
                    ],
                },
            }),
            AssignmentStep::Assign(AssignmentProgram {
                var_index: 1,
                program: product.clone(),
            }),
            AssignmentStep::Assign(AssignmentProgram {
                var_index: 2,
                program: BytecodeProgram {
                    instructions: vec![Instruction::PushConst(1.0), Instruction::IdtJacobian],
                },
            }),
        ];
        for assign_product in [false, true] {
            let live = [true, assign_product, false];
            let mut vm = SmallSignalVm::new(&context, 0.0).unwrap();
            vm.execute_live_assignments(&steps, Some(&live)).unwrap();
            let output = if assign_product {
                BytecodeProgram {
                    instructions: vec![Instruction::PushVariable(1)],
                }
            } else {
                product.clone()
            };
            let result = vm.execute_scaled(&output, 1e-200).unwrap();
            assert!((result.re / 1e200 - 3.0).abs() < 1e-14);
            assert_eq!(result.im, 0.0);
            assert_eq!(context.variables, [2.0, 0.0, 0.0]);
        }
    }

    #[test]
    fn real_comparisons_accept_authored_infinity_before_and_after_recovery() {
        let context = ac_context();
        for recover_first in [false, true] {
            let mut instructions = vec![Instruction::PushConst(f64::INFINITY)];
            if recover_first {
                // This operation invokes range recovery, which must retain
                // the authored infinity's comparison semantics on replay.
                instructions.extend([Instruction::PushConst(1.0), Instruction::Add]);
            }
            instructions.extend([Instruction::PushConst(2.0), Instruction::Gt]);
            let mut vm = SmallSignalVm::new(&context, 1.0).unwrap();
            assert_eq!(
                vm.execute(&BytecodeProgram { instructions }).unwrap(),
                Complex64::new(1.0, 0.0)
            );
        }
    }

    #[test]
    fn range_recovery_replays_assignments_from_the_original_seed() {
        use crate::codegen::AssignmentProgram;

        let mut context = ac_context();
        context.variables = vec![2.0, 0.0];
        let increment = AssignmentStep::Assign(AssignmentProgram {
            var_index: 0,
            program: BytecodeProgram {
                instructions: vec![
                    Instruction::PushVariable(0),
                    Instruction::PushConst(1.0),
                    Instruction::Add,
                ],
            },
        });
        for gain in [1e-200, 1e200] {
            let product = BytecodeProgram {
                instructions: vec![
                    Instruction::PushVariable(0),
                    Instruction::PushConst(gain),
                    Instruction::Mul,
                    Instruction::DdtJacobian,
                ],
            };
            let assignments = [
                increment.clone(),
                AssignmentStep::Assign(AssignmentProgram {
                    var_index: 1,
                    program: product.clone(),
                }),
            ];
            let next_assignments = [increment.clone()];
            // Exercise recovery both during assignments and in a subsequent
            // Jacobian program, after ordinary assignments already completed.
            for assign_product in [false, true] {
                let mut vm = SmallSignalVm::new(&context, gain / std::f64::consts::TAU).unwrap();
                vm.execute_assignments(&assignments[..if assign_product { 2 } else { 1 }])
                    .unwrap();
                let program = if assign_product {
                    BytecodeProgram {
                        instructions: vec![Instruction::PushVariable(1)],
                    }
                } else {
                    product.clone()
                };
                let result = vm.execute_scaled(&program, 1.0 / gain).unwrap();
                assert_eq!(result.re, 0.0);
                assert!((result.im / gain - 3.0).abs() <= 8.0 * f64::EPSILON);
                vm.execute_assignments(&next_assignments).unwrap();
                let value = vm
                    .execute(&BytecodeProgram {
                        instructions: vec![Instruction::PushVariable(0)],
                    })
                    .unwrap();
                assert_eq!(value, Complex64::new(4.0, 0.0));
                assert_eq!(context.variables, [2.0, 0.0]);
            }
        }
    }

    #[test]
    fn complex_division_preserves_range_and_live_stack_values() {
        let context = ac_context();
        let mut vm = SmallSignalEngine::<FrequencyValue>::new(&context, 1.0).unwrap();
        for scale in [f64::from_bits(1), 1e-200, 1.0, 1e200, f64::MAX] {
            for (left, right, expected) in [
                (
                    Complex64::new(scale, 0.0),
                    Complex64::new(scale, 0.0),
                    Complex64::new(1.0, 0.0),
                ),
                (
                    Complex64::new(scale, scale),
                    Complex64::new(scale, -scale),
                    Complex64::new(0.0, 1.0),
                ),
                (
                    Complex64::new(scale, scale),
                    Complex64::new(scale, scale),
                    Complex64::new(1.0, 0.0),
                ),
                (
                    Complex64::new(scale, scale),
                    Complex64::new(0.0, scale),
                    Complex64::new(1.0, -1.0),
                ),
            ] {
                vm.stack = vec![
                    FrequencyValue::new(7.0, -3.0),
                    FrequencyValue::from_complex(left),
                    FrequencyValue::from_complex(right),
                ];
                vm.execute_instruction(&Instruction::Div).unwrap();
                assert_eq!(
                    vm.stack.iter().map(|v| v.binary64()).collect::<Vec<_>>(),
                    [Complex64::new(7.0, -3.0), expected]
                );
            }
        }
        for frequency in [1e-200, 1e200] {
            let mut vm = SmallSignalVm::new(&context, frequency).unwrap();
            let program = BytecodeProgram {
                instructions: vec![Instruction::PushConst(2.0), Instruction::IdtJacobian],
            };
            let value = vm.execute(&program).unwrap();
            assert_eq!(value.re, 0.0);
            assert_eq!(value.im, -2.0 / (std::f64::consts::TAU * frequency));
        }
    }

    #[test]
    fn quotient_sum_preserves_complex_products_and_division() {
        let context = ac_context();
        let mut vm = SmallSignalEngine::<Complex64>::new(&context, 1.0).unwrap();
        for divisor in [Complex64::new(4.0, 0.0), Complex64::new(4.0, 4.0)] {
            vm.stack = vec![
                Complex64::new(7.0, 0.0),
                Complex64::new(1.6e308, 0.0),
                Complex64::new(1.0, 1.0),
                Complex64::new(1.6e308, 0.0),
                Complex64::new(1.0, 1.0),
                divisor,
            ];
            vm.execute_instruction(&Instruction::SumProductsDiv(2))
                .unwrap();
            assert_eq!(vm.stack[0], Complex64::new(7.0, 0.0));
            assert_eq!(vm.stack.len(), 2);
            let expected_im = if divisor.im == 0.0 { 8e307 } else { 0.0 };
            assert!((vm.stack[1].re / 8e307 - 1.0).abs() < 1e-14);
            assert_eq!(vm.stack[1].im, expected_im);
        }
        vm.stack.clear();
        assert!(
            vm.execute_instruction(&Instruction::SumProductsDiv(usize::MAX))
                .is_err()
        );
        assert!(
            vm.execute_instruction(&Instruction::SumProductsDiv(2))
                .is_err()
        );
    }

    #[test]
    fn quotient_replay_retains_wide_components_and_cancellation() {
        let context = ac_context();
        for complex_divisor in [false, true] {
            let mut instructions = Vec::new();
            for (a, b) in [(1e200, 1e200), (1e-200, 1e-200), (-1e200, 1e200)] {
                instructions.extend([
                    Instruction::PushConst(a),
                    Instruction::PushConst(b),
                    Instruction::Mul,
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(1.0),
                    Instruction::DdtJacobian,
                    Instruction::Add,
                ]);
            }
            instructions.extend([
                Instruction::PushConst(1e-200),
                Instruction::PushConst(1e-200),
                Instruction::Mul,
            ]);
            if complex_divisor {
                instructions.extend([
                    Instruction::PushConst(1e-200),
                    Instruction::PushConst(1e-200),
                    Instruction::Mul,
                    Instruction::DdtJacobian,
                    Instruction::Add,
                ]);
            }
            instructions.push(Instruction::SumProductsDiv(3));
            let mut vm = SmallSignalVm::new(&context, 1.0 / std::f64::consts::TAU).unwrap();
            let value = vm.execute(&BytecodeProgram { instructions }).unwrap();
            assert_eq!(
                value,
                Complex64::new(1.0, if complex_divisor { 0.0 } else { 1.0 })
            );
            assert!(
                vm.wide.is_some(),
                "The regression must exercise range replay"
            );
        }
    }

    #[test]
    fn conditional_integer_conversion_is_lazy_in_small_signal_execution() {
        let context = ac_context();
        let mut program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(0.0),
                Instruction::JumpIfFalse(4),
                Instruction::PushConst(3e9),
                Instruction::PushConst(0.0),
                Instruction::BitOr,
                Instruction::Jump(1),
                Instruction::PushConst(7.0),
            ],
        };
        // The false branch must neither convert the invalid integer nor leave
        // values from the untaken arm on the expression stack.
        let mut vm = SmallSignalVm::new(&context, 1.0).unwrap();
        assert_eq!(vm.execute(&program).unwrap(), Complex64::new(7.0, 0.0));
        program.instructions[0] = Instruction::PushConst(1.0);
        assert!(vm.execute(&program).is_err());
        program.instructions[1] = Instruction::Jump(usize::MAX);
        assert!(vm.execute(&program).is_err());
    }

    #[test]
    fn cached_filter_responses_keep_site_kinds_actions_and_frequencies_distinct() {
        use crate::codegen::ZiPolynomialLayout;
        let mut context = ac_context();
        context.laplace_filters = vec![StateSpaceFilter::lowpass_first_order(1.0).unwrap()];
        context.zi_filters =
            vec![crate::zfilter::ZiFilter::new(vec![1.0, 1.0], vec![1.0], 1.0).unwrap()];
        let layout = ZiRuntimeLayout {
            filter_id: 0,
            numerator: ZiPolynomialLayout::Coefficients { len: 2 },
            denominator: ZiPolynomialLayout::Coefficients { len: 1 },
            direct_assignment: false,
        };
        for analysis in [1, 3] {
            context.analysis_type = analysis;
            for frequency in [0.25, 0.75] {
                let mut vm = SmallSignalVm::new(&context, frequency).unwrap();
                for action in [2.0, 3.0, -4.0] {
                    let laplace = BytecodeProgram {
                        instructions: vec![
                            Instruction::PushConst(action),
                            Instruction::LaplaceStateDerivative(0),
                        ],
                    };
                    let response = vm.execute(&laplace).unwrap();
                    let expected = Complex64::new(
                        action / (1.0 + frequency * frequency),
                        -action * frequency / (1.0 + frequency * frequency),
                    );
                    assert!((response - expected).norm() <= 8.0 * f64::EPSILON * action.abs());
                    let mut zi = BytecodeProgram {
                        instructions: vec![
                            Instruction::PushConst(1.0),
                            Instruction::PushConst(1.0),
                            Instruction::PushConst(1.0),
                            Instruction::PushConst(1.0),
                            Instruction::PushConst(0.0),
                            Instruction::PushConst(action),
                            Instruction::PushConst(0.0),
                            Instruction::ZiStateDerivative(layout),
                        ],
                    };
                    let expected =
                        Complex64::new(action, if frequency == 0.25 { -action } else { action });
                    assert!(
                        (vm.execute(&zi).unwrap() - expected).norm()
                            <= 8.0 * f64::EPSILON * action.abs()
                    );
                    *zi.instructions.last_mut().unwrap() = Instruction::ZiState(layout);
                    assert_eq!(vm.execute(&zi).unwrap(), Complex64::new(2.0 * action, 0.0));
                    let dc = BytecodeProgram {
                        instructions: vec![
                            Instruction::PushConst(action),
                            Instruction::LaplaceState(0),
                        ],
                    };
                    assert_eq!(vm.execute(&dc).unwrap(), Complex64::new(action, 0.0));
                }
            }
        }
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
    fn complex_products_recover_finite_cross_products_in_bytecode_and_filters() {
        let mut context = ac_context();
        let frequency = 1.0 / std::f64::consts::TAU;
        let tiny = f64::from_bits(1);
        for (left, right, expected) in [
            ([1.6e308, 8e307], [1.2, 0.3], [1.68e308, 1.44e308]),
            ([tiny, tiny], [0.5, 0.5], [0.0, tiny]),
        ] {
            let program = BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(left[0]),
                    Instruction::PushConst(left[1]),
                    Instruction::DdtJacobian,
                    Instruction::Add,
                    Instruction::PushConst(right[0]),
                    Instruction::PushConst(right[1]),
                    Instruction::DdtJacobian,
                    Instruction::Add,
                    Instruction::Mul,
                ],
            };
            let result = SmallSignalVm::new(&context, frequency)
                .unwrap()
                .execute(&program)
                .expect("both final product components are representable");
            for (actual, expected) in [result.re, result.im].into_iter().zip(expected) {
                if expected == 0.0 || expected == tiny {
                    assert_eq!(actual, expected);
                } else {
                    assert!((actual / expected - 1.0).abs() <= 4.0 * f64::EPSILON);
                }
            }
        }

        context.laplace_filters = vec![
            StateSpaceFilter::new(vec![vec![-2.0]], vec![1.6e308], vec![2.5], 0.0).unwrap(),
            StateSpaceFilter::new(vec![vec![-4.0]], vec![5.1], vec![1.0], 0.0).unwrap(),
        ];
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushConst(1.0),
                Instruction::LaplaceStateDerivative(0),
                Instruction::LaplaceStateDerivative(1),
            ],
        };
        let result = SmallSignalVm::new(&context, frequency)
            .unwrap()
            .execute(&program)
            .expect("the cascade has finite rectangular components");
        assert!((result.re / 1.68e308 - 1.0).abs() <= 8.0 * f64::EPSILON);
        assert!((result.im / -1.44e308 - 1.0).abs() <= 8.0 * f64::EPSILON);
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
