//! The boundary JIT-compiled code calls back across.
//!
//! [`EvalContext`] is the `#[repr(C)]` frame the generated code reads
//! voltages, parameters, and state through; the `rspice_*_native` functions
//! are the `extern "C"` helpers it calls for anything not worth emitting
//! inline — transcendental math and the stateful operators.
//!
//! Errors cannot propagate across this boundary as a `Result`, so helpers
//! publish the first failure into the active [`EvalContext`]. This keeps
//! nested and parallel dispatches isolated without thread-local state.
//!
//! This is an internal contract between this crate's compiler and its own
//! generated code, not a stable ABI. It may change with any release.

use std::cell::UnsafeCell;

use crate::array_index::{checked_array_slot, checked_rounded_i64};
use crate::integer_runtime::{IntegerBinaryOperation, integer_binary, real_to_integer};
use crate::timing_contract::{NormalizedSlewRates, normalize_slew_rates};
use crate::vm::{IntegrationCoefficients, idtmod_wrapped_candidate};

const INTEGER_DESCRIPTOR_KIND_MASK: usize = 0xff;
const INTEGER_DESCRIPTOR_PAYLOAD_SHIFT: u32 = 32;
pub(crate) const INTEGER_CAST_DESCRIPTOR: usize = 0;
const INTEGER_BINARY_DESCRIPTOR_BASE: usize = 1;
const INTEGER_SHIFT_CONST_DESCRIPTOR_BASE: usize = 16;
const INTEGER_BINARY_CONST_DESCRIPTOR_BASE: usize = 32;

fn integration_coefficients(ctx: &EvalContext) -> IntegrationCoefficients {
    IntegrationCoefficients {
        active: ctx.integration_active != 0,
        derivative_scale: ctx.integration_derivative_scale,
        previous_value_scale: ctx.integration_previous_value_scale,
        older_value_scale: ctx.integration_older_value_scale,
        previous_derivative_scale: ctx.integration_previous_derivative_scale,
    }
}

fn event_integer_operand(name: &str, value: f64) -> Result<i32, String> {
    let converted = real_to_integer(value)
        .map_err(|error| format!("{name} integer conversion failed: {error}"))?;
    if f64::from(converted) != value {
        return Err(format!("{name} must evaluate to an integer, got {value}"));
    }
    Ok(converted)
}

pub(crate) fn integer_binary_descriptor(operation: IntegerBinaryOperation) -> usize {
    INTEGER_BINARY_DESCRIPTOR_BASE + integer_operation_code(operation)
}

pub(crate) fn integer_shift_const_descriptor(
    operation: IntegerBinaryOperation,
    count: u8,
) -> usize {
    INTEGER_SHIFT_CONST_DESCRIPTOR_BASE
        + integer_operation_code(operation)
        + (usize::from(count) << 8)
}

pub(crate) fn integer_binary_const_descriptor(
    operation: IntegerBinaryOperation,
    value: i64,
) -> Option<usize> {
    let value = i32::try_from(value).ok()?;
    Some(
        INTEGER_BINARY_CONST_DESCRIPTOR_BASE
            + integer_operation_code(operation)
            + ((value as u32 as usize) << INTEGER_DESCRIPTOR_PAYLOAD_SHIFT),
    )
}

fn integer_operation_code(operation: IntegerBinaryOperation) -> usize {
    match operation {
        IntegerBinaryOperation::Shl => 0,
        IntegerBinaryOperation::Shr => 1,
        IntegerBinaryOperation::BitAnd => 2,
        IntegerBinaryOperation::BitOr => 3,
        IntegerBinaryOperation::BitXor => 4,
    }
}

fn integer_operation_from_code(code: usize) -> Option<IntegerBinaryOperation> {
    match code {
        0 => Some(IntegerBinaryOperation::Shl),
        1 => Some(IntegerBinaryOperation::Shr),
        2 => Some(IntegerBinaryOperation::BitAnd),
        3 => Some(IntegerBinaryOperation::BitOr),
        4 => Some(IntegerBinaryOperation::BitXor),
        _ => None,
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeRuntimeErrorKind {
    NativeJit,
    InvalidNumericResult,
}

pub(crate) struct NativeRuntimeError {
    pub(crate) kind: NativeRuntimeErrorKind,
    pub(crate) message: String,
}

/// Per-dispatch failure state owned by one [`EvalContext`].
///
/// JIT code reads only `failed`; Rust helpers own the diagnostic payload.
/// The status is intentionally not `Sync`: one evaluation frame may be used
/// by only one native dispatch at a time.
#[repr(C)]
pub struct NativeRuntimeStatus {
    failed: UnsafeCell<u8>,
    kind: UnsafeCell<NativeRuntimeErrorKind>,
    message: UnsafeCell<Option<String>>,
}

impl Default for NativeRuntimeStatus {
    fn default() -> Self {
        Self {
            failed: UnsafeCell::new(0),
            kind: UnsafeCell::new(NativeRuntimeErrorKind::NativeJit),
            message: UnsafeCell::new(None),
        }
    }
}

impl NativeRuntimeStatus {
    fn record(&self, kind: NativeRuntimeErrorKind, message: impl Into<String>) {
        // Safety: EvalContext's contract requires exclusive dispatch access.
        // Helpers execute synchronously on that dispatching thread.
        unsafe {
            let slot = &mut *self.message.get();
            if slot.is_none() {
                *self.kind.get() = kind;
                *slot = Some(message.into());
                *self.failed.get() = 1;
            }
        }
    }

    pub(crate) fn take(&self) -> Option<NativeRuntimeError> {
        // Safety: the dispatch contract grants exclusive access while the
        // status is cleared or drained.
        unsafe {
            *self.failed.get() = 0;
            (&mut *self.message.get())
                .take()
                .map(|message| NativeRuntimeError {
                    kind: *self.kind.get(),
                    message,
                })
        }
    }

    pub(crate) fn clear(&self) {
        let _ = self.take();
    }

    pub(crate) fn failed_offset() -> usize {
        std::mem::offset_of!(Self, failed)
    }
}

/// Evaluation context passed to JIT-compiled functions.
#[repr(C)]
pub struct EvalContext {
    /// Terminal voltages array
    pub voltages: *const f64,
    /// Internal node voltages
    pub internal_voltages: *const f64,
    /// Parameter values
    pub params: *const f64,
    /// Flattened current matrix over terminals plus the global reference.
    pub branch_currents: *const f64,
    /// Length of `branch_currents` buffer
    pub branch_currents_len: usize,
    /// Sequentially evaluated contribution currents for current-probe reads
    pub currents: *const f64,
    /// Length of `currents` buffer
    pub currents_len: usize,
    /// Number of terminals in the device
    pub num_terminals: usize,
    /// Whether each external terminal was explicitly connected.
    pub port_connected: *const u8,
    /// Length of `port_connected`.
    pub port_connected_len: usize,
    /// Temperature in Kelvin
    pub temperature: f64,
    /// Simulation time
    pub time: f64,
    /// Time step (for transient)
    pub timestep: f64,
    /// Previous state values (for ddt/idt)
    pub state_prev: *const f64,
    /// Current state values, written by ddt/idt operators (sized by the
    /// device's preallocation scan; null only when no program holds state)
    pub state_values: *mut f64,
    /// Per-state initialized flags used by stateful operators such as $limit.
    pub state_initialized: *mut u8,
    /// Length of `state_initialized`.
    pub state_initialized_len: usize,
    /// Lookup tables pointer (for $table_model)
    /// Points to a slice of LookupTable structs
    pub lookup_tables: *const crate::codegen::LookupTable,
    /// Number of lookup tables
    pub lookup_tables_len: usize,
    /// Laplace state-space filters (mutable for step())
    pub laplace_filters: *mut crate::laplace::StateSpaceFilter,
    /// Number of Laplace filters
    pub laplace_filters_len: usize,
    /// Whether each parameter was explicitly given (one byte per
    /// parameter, same length as `params`)
    pub param_given: *const u8,
    /// Length of `param_given`.
    pub param_given_len: usize,
    /// Branch-current unknown values of potential contributions (sized to
    /// the model's branch unknown count)
    pub branch_unknowns: *const f64,
    /// Analysis type code (0=dc, 1=ac, 2=tran, 3=noise, 4=ic), matching
    /// `VmContext::analysis_type`
    pub analysis_type: u8,
    /// Instance multiplicity ($mfactor)
    pub multiplicity: f64,
    /// Z-domain sampled-data filters (mutable for candidate eval())
    pub zi_filters: *mut crate::zfilter::ZiFilter,
    /// Number of Z-domain filters
    pub zi_filters_len: usize,
    /// Transition filters (mutable for transition(...) evaluation)
    pub transition_filters: *mut crate::vm::TransitionFilter,
    /// Number of transition filters
    pub transition_filters_len: usize,
    /// Slew-rate filters (mutable for slew(...) evaluation)
    pub slew_filters: *mut crate::vm::SlewFilter,
    /// Number of slew-rate filters
    pub slew_filters_len: usize,
    /// Delay buffers (mutable for absdelay(...) evaluation)
    pub delay_buffers: *mut crate::vm::DelayBuffer,
    /// Number of delay buffers
    pub delay_buffers_len: usize,
    /// Cross detectors (mutable for cross(...) evaluation)
    pub cross_detectors: *mut crate::vm::CrossDetector,
    /// Number of cross detectors
    pub cross_detectors_len: usize,
    /// Length of `state_prev`.
    pub state_prev_len: usize,
    /// Length of `state_values`.
    pub state_values_len: usize,
    /// Earliest absolute timer event requested by native evaluation.
    pub timer_event_bound: *mut f64,
    /// Nonzero at the first point of the current analysis.
    pub analysis_initial_step: u8,
    /// Nonzero at the final point of the current analysis.
    pub analysis_final_step: u8,
    /// State values from two accepted points ago.
    pub state_older: *const f64,
    /// Length of `state_older`.
    pub state_older_len: usize,
    /// Candidate derivative/input values for the current point.
    pub state_derivatives: *mut f64,
    /// Length of `state_derivatives`.
    pub state_derivatives_len: usize,
    /// Derivative/input values from the previous accepted point.
    pub state_derivatives_prev: *const f64,
    /// Length of `state_derivatives_prev`.
    pub state_derivatives_prev_len: usize,
    /// Current-value coefficient in the solver's derivative formula.
    pub integration_derivative_scale: f64,
    /// Previous-value coefficient in the solver's derivative formula.
    pub integration_previous_value_scale: f64,
    /// Older-value coefficient in the solver's derivative formula.
    pub integration_older_value_scale: f64,
    /// Previous-derivative coefficient in the solver's derivative formula.
    pub integration_previous_derivative_scale: f64,
    /// Nonzero when transient integration is active.
    pub integration_active: u8,
    /// Per-evaluation convergence flag set when a named limiter changes its
    /// oriented proposal. Required when `limiting_enabled` is nonzero.
    pub limiter_active: *mut u8,
    /// Nonzero only for limited Newton assembly. Probe and small-signal
    /// evaluation bypass limiter history entirely.
    pub limiting_enabled: u8,
    /// Failure state for this dispatch. This must not be shared by concurrent
    /// native calls.
    #[doc(hidden)]
    pub runtime_status: NativeRuntimeStatus,
    /// Per-integration-state marker written by a speculative native
    /// evaluation. These flags are committed by the owning `VmContext`, never
    /// by generated code.
    pub state_candidate_valid: *mut u8,
    /// Length of `state_candidate_valid`.
    pub state_candidate_valid_len: usize,
    /// Per-state older-history lane proposed by the current integration
    /// evaluation and consumed only when the owning VM accepts the point.
    pub state_older_candidate: *mut f64,
    /// Length of `state_older_candidate`.
    pub state_older_candidate_len: usize,
}

impl EvalContext {
    #[cfg(test)]
    pub(crate) fn empty_for_test() -> Self {
        Self {
            voltages: std::ptr::null(),
            internal_voltages: std::ptr::null(),
            params: std::ptr::null(),
            branch_currents: std::ptr::null(),
            branch_currents_len: 0,
            currents: std::ptr::null(),
            currents_len: 0,
            num_terminals: 0,
            port_connected: std::ptr::null(),
            port_connected_len: 0,
            temperature: 0.0,
            time: 0.0,
            timestep: 0.0,
            state_prev: std::ptr::null(),
            state_values: std::ptr::null_mut(),
            state_initialized: std::ptr::null_mut(),
            state_initialized_len: 0,
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
            param_given_len: 0,
            branch_unknowns: std::ptr::null(),
            analysis_type: 0,
            multiplicity: 1.0,
            zi_filters: std::ptr::null_mut(),
            zi_filters_len: 0,
            transition_filters: std::ptr::null_mut(),
            transition_filters_len: 0,
            slew_filters: std::ptr::null_mut(),
            slew_filters_len: 0,
            delay_buffers: std::ptr::null_mut(),
            delay_buffers_len: 0,
            cross_detectors: std::ptr::null_mut(),
            cross_detectors_len: 0,
            state_prev_len: 0,
            state_values_len: 0,
            timer_event_bound: std::ptr::null_mut(),
            analysis_initial_step: 0,
            analysis_final_step: 0,
            state_older: std::ptr::null(),
            state_older_len: 0,
            state_derivatives: std::ptr::null_mut(),
            state_derivatives_len: 0,
            state_derivatives_prev: std::ptr::null(),
            state_derivatives_prev_len: 0,
            integration_derivative_scale: 0.0,
            integration_previous_value_scale: 0.0,
            integration_older_value_scale: 0.0,
            integration_previous_derivative_scale: 0.0,
            integration_active: 0,
            limiter_active: std::ptr::null_mut(),
            limiting_enabled: 0,
            runtime_status: Default::default(),
            state_candidate_valid: std::ptr::null_mut(),
            state_candidate_valid_len: 0,
            state_older_candidate: std::ptr::null_mut(),
            state_older_candidate_len: 0,
        }
    }

    pub(crate) fn clear_runtime_error(&self) {
        self.runtime_status.clear();
    }

    pub(crate) fn take_runtime_error(&self) -> Option<String> {
        self.take_native_runtime_error().map(|error| error.message)
    }

    pub(crate) fn take_native_runtime_error(&self) -> Option<NativeRuntimeError> {
        self.runtime_status.take()
    }

    pub(crate) fn record_runtime_error(&self, message: impl Into<String>) {
        self.runtime_status
            .record(NativeRuntimeErrorKind::NativeJit, message);
    }

    pub(crate) fn record_invalid_numeric_result(&self, message: impl Into<String>) {
        self.runtime_status
            .record(NativeRuntimeErrorKind::InvalidNumericResult, message);
    }
}

fn set_native_context_error(ctx: &EvalContext, message: impl Into<String>) {
    ctx.record_runtime_error(message);
}

fn set_native_context_error_ptr(ctx: *const EvalContext, message: impl Into<String>) {
    // SAFETY: A non-null pointer comes from the active native entry point.
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        set_native_context_error(ctx, message);
    }
}

#[unsafe(export_name = "rspice_native_loop_limit_error")]
pub extern "C" fn rspice_native_loop_limit_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native runtime loop iteration limit exceeded; no interpreter fallback",
    );
}

#[unsafe(export_name = "rspice_native_non_finite_contribution_error")]
pub extern "C" fn rspice_native_non_finite_contribution_error(
    ctx: *const EvalContext,
    contribution: usize,
) {
    // SAFETY: A non-null pointer comes from the active native entry point.
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        ctx.record_invalid_numeric_result(format!(
            "contribution {contribution} evaluated to a non-finite value (NaN or infinity)"
        ));
    }
}

#[unsafe(export_name = "rspice_native_limit_state_values_error")]
pub extern "C" fn rspice_native_limit_state_values_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native limit state missing state storage; no interpreter fallback",
    );
}

#[unsafe(export_name = "rspice_native_limit_state_values_bounds_error")]
pub extern "C" fn rspice_native_limit_state_values_bounds_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native limit state index outside state storage; no interpreter fallback",
    );
}

#[unsafe(export_name = "rspice_native_limit_state_initialized_error")]
pub extern "C" fn rspice_native_limit_state_initialized_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native limit state missing initialization flag storage; no interpreter fallback",
    );
}

#[unsafe(export_name = "rspice_native_limit_state_bounds_error")]
pub extern "C" fn rspice_native_limit_state_bounds_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native limit state index outside initialization flag storage; no interpreter fallback",
    );
}

unsafe fn native_limiter_storage(
    ctx: *const EvalContext,
    state_id: usize,
) -> Option<(*mut f64, *mut u8)> {
    if ctx.is_null() {
        rspice_native_limit_state_values_error(ctx);
        return None;
    }
    let ctx = unsafe { &*ctx };
    if ctx.state_values.is_null() {
        set_native_context_error(
            ctx,
            "native limit state missing state storage; no interpreter fallback",
        );
        return None;
    }
    if state_id >= ctx.state_values_len {
        set_native_context_error(
            ctx,
            "native limit state index outside state storage; no interpreter fallback",
        );
        return None;
    }
    if ctx.state_initialized.is_null() {
        set_native_context_error(
            ctx,
            "native limit state missing initialization flag storage; no interpreter fallback",
        );
        return None;
    }
    if state_id >= ctx.state_initialized_len {
        set_native_context_error(
            ctx,
            "native limit state index outside initialization flag storage; no interpreter fallback",
        );
        return None;
    }
    Some((unsafe { ctx.state_values.add(state_id) }, unsafe {
        ctx.state_initialized.add(state_id)
    }))
}

/// Read the previous Newton value for a named limiter, or use the oriented
/// proposed value before that limiter has produced its first candidate.
///
/// # Safety
/// `ctx` must either be null (which records a native runtime error) or point to
/// a live [`EvalContext`] whose limiter state buffers remain valid for the
/// duration of this call.
#[unsafe(export_name = "rspice_limiter_previous_native")]
pub unsafe extern "C" fn rspice_limiter_previous_native(
    proposed: f64,
    ctx: *const EvalContext,
    state_id: usize,
) -> f64 {
    if ctx.is_null() {
        rspice_native_limit_state_values_error(ctx);
        return 0.0;
    }
    if unsafe { (*ctx).limiting_enabled } == 0 {
        return proposed;
    }

    let Some((state_value, initialized)) = (unsafe { native_limiter_storage(ctx, state_id) })
    else {
        return 0.0;
    };
    if unsafe { *initialized } == 0 {
        proposed
    } else {
        unsafe { *state_value }
    }
}

/// Publish a named limiter candidate as the previous value for the next
/// Newton evaluation and mark whether it differs from its oriented proposal.
/// Probe and small-signal evaluations return the proposal without reading or
/// mutating limiter state.
///
/// # Safety
/// `ctx` must either be null (which records a native runtime error) or point to
/// a live [`EvalContext`] whose limiter state buffers remain valid for the
/// duration of this call.
#[unsafe(export_name = "rspice_limiter_store_native")]
pub unsafe extern "C" fn rspice_limiter_store_native(
    operands: *const f64,
    ctx: *const EvalContext,
    state_id: usize,
) -> f64 {
    if ctx.is_null() {
        rspice_native_limit_state_values_error(ctx);
        return 0.0;
    }
    let context = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            context,
            "native named limiter candidate publish missing operands; no interpreter fallback",
        );
        return 0.0;
    }

    let proposed = unsafe { *operands };
    if context.limiting_enabled == 0 {
        return proposed;
    }

    let limiter_active = context.limiter_active;
    if limiter_active.is_null() {
        set_native_context_error(
            context,
            "native named limiter missing convergence storage; no interpreter fallback",
        );
        return 0.0;
    }
    let Some((state_value, initialized)) = (unsafe { native_limiter_storage(ctx, state_id) })
    else {
        return 0.0;
    };
    let candidate = unsafe { *operands.add(1) };
    unsafe {
        *limiter_active |= u8::from(candidate != proposed);
        *state_value = candidate;
        *initialized = 1;
    }
    candidate
}

#[unsafe(export_name = "rspice_native_current_probe_error")]
pub extern "C" fn rspice_native_current_probe_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native current probe missing terminal-pair current storage; no interpreter fallback",
    );
}

#[unsafe(export_name = "rspice_native_prior_current_error")]
pub extern "C" fn rspice_native_prior_current_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native prior current load missing contribution current storage; no interpreter fallback",
    );
}

#[unsafe(export_name = "rspice_native_param_given_error")]
pub extern "C" fn rspice_native_param_given_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native param_given load missing parameter-given storage; no interpreter fallback",
    );
}

#[unsafe(export_name = "rspice_native_port_connected_error")]
pub extern "C" fn rspice_native_port_connected_error(ctx: *const EvalContext) {
    set_native_context_error_ptr(
        ctx,
        "native port_connected load missing connection-flag storage; no interpreter fallback",
    );
}

/// External helper function for native x64 table lookup interpolation.
///
/// Argument order is chosen for x64 helper-call codegen: scalar input in XMM0,
/// followed by the dispatch context and table ID in integer argument registers.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers.
#[unsafe(export_name = "rspice_table_lookup_native")]
pub unsafe extern "C" fn rspice_table_lookup_native(
    input: f64,
    ctx: *const EvalContext,
    table_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            format!(
                "native table lookup helper missing EvalContext for table {table_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if ctx.lookup_tables.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native table lookup helper missing table storage for table {table_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if table_id >= ctx.lookup_tables_len {
        set_native_context_error(
            ctx,
            format!(
                "native table lookup helper table {table_id} outside table length {}; no interpreter fallback",
                ctx.lookup_tables_len
            ),
        );
        return 0.0;
    }

    let tables = unsafe { std::slice::from_raw_parts(ctx.lookup_tables, ctx.lookup_tables_len) };
    tables[table_id].interpolate(input)
}

/// External helper function for native x64 table-model derivatives.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers.
#[unsafe(export_name = "rspice_table_derivative_native")]
pub unsafe extern "C" fn rspice_table_derivative_native(
    input: f64,
    ctx: *const EvalContext,
    table_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            format!(
                "native table derivative helper missing EvalContext for table {table_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if ctx.lookup_tables.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native table derivative helper missing table storage for table {table_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if table_id >= ctx.lookup_tables_len {
        set_native_context_error(
            ctx,
            format!(
                "native table derivative helper table {table_id} outside table length {}; no interpreter fallback",
                ctx.lookup_tables_len
            ),
        );
        return 0.0;
    }

    let tables = unsafe { std::slice::from_raw_parts(ctx.lookup_tables, ctx.lookup_tables_len) };
    tables[table_id].derivative(input)
}

/// External helper function for idtmod wrapping.
#[unsafe(export_name = "rspice_idtmod_wrap")]
pub extern "C" fn rspice_idtmod_wrap(raw: f64, modulus: f64, offset: f64) -> f64 {
    idtmod_wrapped_candidate(raw, modulus, offset).map_or(f64::NAN, |(wrapped, _)| wrapped)
}

/// External helper function for limited exponential.
#[unsafe(export_name = "rspice_limexp")]
pub extern "C" fn rspice_limexp(x: f64) -> f64 {
    const LIMIT: f64 = 40.0;
    if x > LIMIT {
        let exp_limit = LIMIT.exp();
        exp_limit * (1.0 + x - LIMIT)
    } else if x < -LIMIT {
        (-LIMIT).exp()
    } else {
        x.exp()
    }
}

/// External helper function for CMC-style limited exponential.
#[unsafe(export_name = "rspice_limited_exp")]
pub extern "C" fn rspice_limited_exp(x: f64) -> f64 {
    const LIMIT: f64 = 80.0;
    const LOW_VALUE: f64 = 1.804851387e-35;
    if x > LIMIT {
        LIMIT.exp() * (1.0 + x - LIMIT)
    } else if x < -LIMIT {
        LOW_VALUE
    } else {
        x.exp()
    }
}

/// External helper function for exponential.
#[unsafe(export_name = "rspice_exp")]
pub extern "C" fn rspice_exp(x: f64) -> f64 {
    x.exp()
}

/// External helper function for natural logarithm.
#[unsafe(export_name = "rspice_log")]
pub extern "C" fn rspice_log(x: f64) -> f64 {
    x.ln()
}

/// External helper function for base-10 logarithm.
#[unsafe(export_name = "rspice_log10")]
pub extern "C" fn rspice_log10(x: f64) -> f64 {
    x.log10()
}

/// External helper function for sine.
#[unsafe(export_name = "rspice_sin")]
pub extern "C" fn rspice_sin(x: f64) -> f64 {
    x.sin()
}

/// External helper function for cosine.
#[unsafe(export_name = "rspice_cos")]
pub extern "C" fn rspice_cos(x: f64) -> f64 {
    x.cos()
}

/// External helper function for tangent.
#[unsafe(export_name = "rspice_tan")]
pub extern "C" fn rspice_tan(x: f64) -> f64 {
    x.tan()
}

/// External helper function for hyperbolic sine.
#[unsafe(export_name = "rspice_sinh")]
pub extern "C" fn rspice_sinh(x: f64) -> f64 {
    x.sinh()
}

/// External helper function for hyperbolic cosine.
#[unsafe(export_name = "rspice_cosh")]
pub extern "C" fn rspice_cosh(x: f64) -> f64 {
    x.cosh()
}

/// External helper function for hyperbolic tangent.
#[unsafe(export_name = "rspice_tanh")]
pub extern "C" fn rspice_tanh(x: f64) -> f64 {
    x.tanh()
}

/// External helper function for inverse hyperbolic sine.
#[unsafe(export_name = "rspice_asinh")]
pub extern "C" fn rspice_asinh(x: f64) -> f64 {
    x.asinh()
}

/// External helper function for inverse hyperbolic cosine.
#[unsafe(export_name = "rspice_acosh")]
pub extern "C" fn rspice_acosh(x: f64) -> f64 {
    x.acosh()
}

/// External helper function for inverse hyperbolic tangent.
#[unsafe(export_name = "rspice_atanh")]
pub extern "C" fn rspice_atanh(x: f64) -> f64 {
    x.atanh()
}

/// External helper function for arcsine.
#[unsafe(export_name = "rspice_asin")]
pub extern "C" fn rspice_asin(x: f64) -> f64 {
    x.asin()
}

/// External helper function for arccosine.
#[unsafe(export_name = "rspice_acos")]
pub extern "C" fn rspice_acos(x: f64) -> f64 {
    x.acos()
}

/// External helper function for arctangent.
#[unsafe(export_name = "rspice_atan")]
pub extern "C" fn rspice_atan(x: f64) -> f64 {
    x.atan()
}

/// External helper function for floor.
#[unsafe(export_name = "rspice_floor")]
pub extern "C" fn rspice_floor(x: f64) -> f64 {
    x.floor()
}

/// External helper function for ceiling.
#[unsafe(export_name = "rspice_ceil")]
pub extern "C" fn rspice_ceil(x: f64) -> f64 {
    x.ceil()
}

/// External helper function for power.
#[unsafe(export_name = "rspice_pow")]
pub extern "C" fn rspice_pow(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// External helper function for two-argument arctangent.
#[unsafe(export_name = "rspice_atan2")]
pub extern "C" fn rspice_atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

/// External helper function for Euclidean norm.
#[unsafe(export_name = "rspice_hypot")]
pub extern "C" fn rspice_hypot(left: f64, right: f64) -> f64 {
    left.hypot(right)
}

/// External helper function for Verilog-A remainder.
#[unsafe(export_name = "rspice_mod")]
pub extern "C" fn rspice_mod(left: f64, right: f64) -> f64 {
    left % right
}

/// Evaluate the shared signed-32-bit Verilog-AMS integer contract for native
/// code. The descriptor selects a cast, a two-operand operation, or one of the
/// bounded constant forms used by the native optimizer.
#[unsafe(export_name = "rspice_integer_operation_native")]
pub unsafe extern "C" fn rspice_integer_operation_native(
    operands: *const f64,
    ctx: *const EvalContext,
    descriptor: usize,
) -> f64 {
    let result = (|| {
        if operands.is_null() {
            return Err("native integer operation received null operand storage".to_string());
        }
        let kind = descriptor & INTEGER_DESCRIPTOR_KIND_MASK;
        // SAFETY: generated native call sites pass a validated operand run of
        // the length selected by the descriptor kind.
        let left = unsafe { *operands };
        if kind == INTEGER_CAST_DESCRIPTOR {
            return real_to_integer(left)
                .map(f64::from)
                .map_err(|error| error.to_string());
        }

        if (INTEGER_BINARY_DESCRIPTOR_BASE..INTEGER_BINARY_DESCRIPTOR_BASE + 5).contains(&kind) {
            let operation = integer_operation_from_code(kind - INTEGER_BINARY_DESCRIPTOR_BASE)
                .ok_or_else(|| "native integer descriptor has an invalid operation".to_string())?;
            // SAFETY: binary descriptors require exactly two operands.
            let right = unsafe { *operands.add(1) };
            return integer_binary(operation, left, right).map_err(|error| error.to_string());
        }

        if (INTEGER_SHIFT_CONST_DESCRIPTOR_BASE..INTEGER_SHIFT_CONST_DESCRIPTOR_BASE + 5)
            .contains(&kind)
        {
            let operation = integer_operation_from_code(kind - INTEGER_SHIFT_CONST_DESCRIPTOR_BASE)
                .ok_or_else(|| {
                    "native integer constant-shift descriptor has an invalid operation".to_string()
                })?;
            let count = ((descriptor >> 8) & 0xff) as f64;
            return integer_binary(operation, left, count).map_err(|error| error.to_string());
        }

        if (INTEGER_BINARY_CONST_DESCRIPTOR_BASE..INTEGER_BINARY_CONST_DESCRIPTOR_BASE + 5)
            .contains(&kind)
        {
            let operation =
                integer_operation_from_code(kind - INTEGER_BINARY_CONST_DESCRIPTOR_BASE)
                    .ok_or_else(|| {
                        "native integer constant descriptor has an invalid operation".to_string()
                    })?;
            let right = ((descriptor >> INTEGER_DESCRIPTOR_PAYLOAD_SHIFT) as u32 as i32) as f64;
            return integer_binary(operation, left, right).map_err(|error| error.to_string());
        }

        Err("native integer operation received an invalid descriptor".to_string())
    })();

    match result {
        Ok(value) => value,
        Err(error) => {
            set_native_context_error_ptr(
                ctx,
                format!(
                    "native Verilog-AMS integer operation failed: {error}; no interpreter fallback"
                ),
            );
            0.0
        }
    }
}

/// External helper function for native x64 Laplace state-space filter
/// evaluation.
///
/// Argument order is chosen to stay within register arguments on both Windows
/// x64 and System V: scalar input in XMM0, then context pointer and filter ID
/// in integer argument registers.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer. The context must own a Laplace filter array whose lifetime covers
/// the call.
#[unsafe(export_name = "rspice_laplace_step_native")]
pub unsafe extern "C" fn rspice_laplace_step_native(
    input: f64,
    ctx: *const EvalContext,
    filter_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native Laplace helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }

    let ctx = unsafe { &*ctx };
    if ctx.laplace_filters.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native Laplace helper missing filter storage for filter {filter_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if filter_id >= ctx.laplace_filters_len {
        set_native_context_error(
            ctx,
            format!(
                "native Laplace helper filter {filter_id} outside filter table length {}; no interpreter fallback",
                ctx.laplace_filters_len
            ),
        );
        return 0.0;
    }

    let filters =
        unsafe { std::slice::from_raw_parts_mut(ctx.laplace_filters, ctx.laplace_filters_len) };
    let coefficients = integration_coefficients(ctx);
    let result = if ctx.analysis_type == 2 && coefficients.active {
        filters[filter_id].step_with_integration(input, coefficients)
    } else if ctx.analysis_type == 2 {
        filters[filter_id].dc_candidate(input)
    } else {
        filters[filter_id].dc_output(input)
    };
    match result {
        Ok(value) => value,
        Err(error) => {
            set_native_context_error(
                ctx,
                format!("native Laplace filter {filter_id} evaluation failed: {error}"),
            );
            0.0
        }
    }
}

/// Evaluate the exact input derivative of a native Laplace filter without
/// changing either its accepted history or its in-flight candidate.
///
/// Active transient integration uses the current companion-rule input
/// coefficient. Transient operating-point and non-transient analyses use the
/// DC action, matching the reference VM's `LaplaceStateDerivative` contract.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer. The context must own a Laplace filter array whose lifetime covers
/// the call, and the calling dispatch must have exclusive access to its
/// runtime status.
#[unsafe(export_name = "rspice_laplace_derivative_native")]
pub unsafe extern "C" fn rspice_laplace_derivative_native(
    input_derivative: f64,
    ctx: *const EvalContext,
    filter_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native Laplace derivative helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }

    let ctx = unsafe { &*ctx };
    if ctx.laplace_filters.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native Laplace derivative helper missing filter storage for filter {filter_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if filter_id >= ctx.laplace_filters_len {
        set_native_context_error(
            ctx,
            format!(
                "native Laplace derivative helper filter {filter_id} outside filter table length {}; no interpreter fallback",
                ctx.laplace_filters_len
            ),
        );
        return 0.0;
    }

    let filters = unsafe {
        std::slice::from_raw_parts(ctx.laplace_filters.cast_const(), ctx.laplace_filters_len)
    };
    let filter = &filters[filter_id];
    let coefficients = integration_coefficients(ctx);
    let result = if ctx.analysis_type == 2 && coefficients.active {
        match filter.transient_input_gain(coefficients) {
            Ok(gain) => {
                let result = gain * input_derivative;
                if !result.is_finite() || (result == 0.0 && gain != 0.0 && input_derivative != 0.0)
                {
                    Err("input action is not representable".to_owned())
                } else {
                    Ok(result)
                }
            }
            Err(error) => Err(error.to_string()),
        }
    } else {
        filter
            .dc_output(input_derivative)
            .map_err(|error| error.to_string())
    };

    match result {
        Ok(value) => value,
        Err(error) => {
            ctx.record_invalid_numeric_result(format!(
                "native Laplace derivative {filter_id} evaluation failed: {error}"
            ));
            0.0
        }
    }
}

/// External helper function for native x64 Z-domain sampled-data filter
/// evaluation.
///
/// Argument order matches `rspice_laplace_step_native`, keeping scalar input
/// in XMM0 and the context pointer/filter ID in integer argument registers.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer. The context must own a zi filter array whose lifetime covers the
/// call.
#[unsafe(export_name = "rspice_zi_step_native")]
pub unsafe extern "C" fn rspice_zi_step_native(
    operands: *const f64,
    ctx: *const EvalContext,
    descriptor: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native zi helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }

    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native zi helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }
    let Some(layout) = crate::codegen::ZiRuntimeLayout::from_native_descriptor(descriptor) else {
        set_native_context_error(
            ctx,
            "native zi helper received an invalid runtime layout descriptor; no interpreter fallback",
        );
        return 0.0;
    };
    let Ok(operand_count) = layout.validate_operand_budget() else {
        set_native_context_error(
            ctx,
            "native zi helper rejected an over-budget runtime layout; no interpreter fallback",
        );
        return 0.0;
    };
    let filter_id = layout.filter_id;
    if ctx.zi_filters.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native zi helper missing filter storage for filter {filter_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if filter_id >= ctx.zi_filters_len {
        set_native_context_error(
            ctx,
            format!(
                "native zi helper filter {filter_id} outside filter table length {}; no interpreter fallback",
                ctx.zi_filters_len
            ),
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, operand_count) };
    let filters = unsafe { std::slice::from_raw_parts_mut(ctx.zi_filters, ctx.zi_filters_len) };
    if !filters[filter_id].definition_is_frozen() {
        match layout.freeze_filter(operands) {
            Ok(filter) => filters[filter_id] = filter,
            Err(error) => {
                ctx.record_invalid_numeric_result(format!(
                    "native zi filter {filter_id} definition freeze failed: {error}"
                ));
                return 0.0;
            }
        }
    }
    let input = operands[operands.len() - 2];
    let transition = operands[operands.len() - 1];
    match filters[filter_id].eval_with_transition_constraint(
        input,
        ctx.time,
        ctx.analysis_type == 2,
        transition,
        layout.direct_assignment,
    ) {
        Ok(value) => {
            if ctx.analysis_type == 2 && !ctx.timer_event_bound.is_null() {
                match filters[filter_id].next_event_time(ctx.time) {
                    Ok(event_time) => {
                        let current = unsafe { &mut *ctx.timer_event_bound };
                        *current = current.min(event_time);
                    }
                    Err(error) => {
                        ctx.record_invalid_numeric_result(format!(
                            "native zi filter {filter_id} breakpoint failed: {error}"
                        ));
                        return 0.0;
                    }
                }
            }
            value
        }
        Err(error) => {
            ctx.record_invalid_numeric_result(format!(
                "native zi filter {filter_id} evaluation failed: {error}"
            ));
            0.0
        }
    }
}

/// External helper for native Z-domain sampled-data filter derivative
/// evaluation.
///
/// # Safety
/// When non-null, `ctx` must point to a live [`EvalContext`] whose
/// `zi_filters` allocation contains `zi_filters_len` initialized entries and
/// remains valid for the call. When non-null, `operands` must point to the
/// contiguous `f64` operand sequence described by `descriptor`. No other code
/// may access the selected Zi filter while this function mutates it.
#[unsafe(export_name = "rspice_zi_derivative_native")]
pub unsafe extern "C" fn rspice_zi_derivative_native(
    operands: *const f64,
    ctx: *const EvalContext,
    descriptor: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native zi derivative helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    let Some(layout) = crate::codegen::ZiRuntimeLayout::from_native_descriptor(descriptor) else {
        set_native_context_error(
            ctx,
            "native zi derivative helper received an invalid runtime layout descriptor; no interpreter fallback",
        );
        return 0.0;
    };
    let Ok(operand_count) = layout.validate_operand_budget() else {
        set_native_context_error(
            ctx,
            "native zi derivative helper rejected an over-budget runtime layout; no interpreter fallback",
        );
        return 0.0;
    };
    let filter_id = layout.filter_id;
    if operands.is_null() || ctx.zi_filters.is_null() || filter_id >= ctx.zi_filters_len {
        set_native_context_error(
            ctx,
            format!("native zi derivative helper has invalid operands or filter {filter_id}"),
        );
        return 0.0;
    }
    let operands = unsafe { std::slice::from_raw_parts(operands, operand_count) };
    let filters = unsafe { std::slice::from_raw_parts_mut(ctx.zi_filters, ctx.zi_filters_len) };
    if !filters[filter_id].definition_is_frozen() {
        match layout.freeze_filter(operands) {
            Ok(filter) => filters[filter_id] = filter,
            Err(error) => {
                ctx.record_invalid_numeric_result(format!(
                    "native zi filter {filter_id} definition freeze failed: {error}"
                ));
                return 0.0;
            }
        }
    }
    let derivative = operands[operands.len() - 2];
    let transition = operands[operands.len() - 1];
    match filters[filter_id].eval_derivative_with_constraint(
        derivative,
        ctx.time,
        ctx.analysis_type == 2,
        transition,
        layout.direct_assignment,
    ) {
        Ok(value) => value,
        Err(error) => {
            ctx.record_invalid_numeric_result(format!(
                "native zi filter {filter_id} derivative failed: {error}"
            ));
            0.0
        }
    }
}

fn invalid_native_integration_context(
    ctx: *const EvalContext,
    operator: &str,
    state_id: usize,
    detail: &str,
) -> f64 {
    let message = format!("native {operator} state {state_id} {detail}; no interpreter fallback");
    set_native_context_error_ptr(ctx, message);
    0.0
}

unsafe fn native_state_storage_is_valid(ctx: &EvalContext, state_id: usize) -> bool {
    state_id < ctx.state_values_len
        && state_id < ctx.state_prev_len
        && state_id < ctx.state_older_len
        && state_id < ctx.state_derivatives_len
        && state_id < ctx.state_derivatives_prev_len
        && state_id < ctx.state_initialized_len
        && state_id < ctx.state_candidate_valid_len
        && state_id < ctx.state_older_candidate_len
        && !ctx.state_values.is_null()
        && !ctx.state_prev.is_null()
        && !ctx.state_older.is_null()
        && !ctx.state_derivatives.is_null()
        && !ctx.state_derivatives_prev.is_null()
        && !ctx.state_initialized.is_null()
        && !ctx.state_candidate_valid.is_null()
        && !ctx.state_older_candidate.is_null()
}

/// Native companion evaluation for `ddt`.
///
/// # Safety
/// `operands` points to one f64 and `ctx` points to a live evaluation context.
#[unsafe(export_name = "rspice_ddt_state_native")]
pub unsafe extern "C" fn rspice_ddt_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    state_id: usize,
) -> f64 {
    if operands.is_null() || ctx.is_null() {
        return invalid_native_integration_context(
            ctx,
            "ddt",
            state_id,
            "missing operands or context",
        );
    }
    let ctx = unsafe { &*ctx };
    if !unsafe { native_state_storage_is_valid(ctx, state_id) } {
        return invalid_native_integration_context(
            ctx,
            "ddt",
            state_id,
            "has invalid state storage",
        );
    }
    let value = unsafe { *operands };
    let initialized = unsafe { *ctx.state_initialized.add(state_id) != 0 };
    let previous = if initialized {
        unsafe { *ctx.state_prev.add(state_id) }
    } else {
        value
    };
    let older = if initialized {
        unsafe { *ctx.state_older.add(state_id) }
    } else {
        previous
    };
    let previous_derivative = if initialized {
        unsafe { *ctx.state_derivatives_prev.add(state_id) }
    } else {
        0.0
    };
    let derivative = if ctx.integration_active != 0 {
        value * ctx.integration_derivative_scale
            - previous * ctx.integration_previous_value_scale
            - older * ctx.integration_older_value_scale
            - previous_derivative * ctx.integration_previous_derivative_scale
    } else {
        0.0
    };
    unsafe {
        *ctx.state_values.add(state_id) = value;
        *ctx.state_derivatives.add(state_id) = derivative;
        *ctx.state_older_candidate.add(state_id) = previous;
        *ctx.state_candidate_valid.add(state_id) = 1;
    }
    derivative
}

/// Native companion Jacobian evaluation for `ddt`.
///
/// # Safety
/// `operands` points to one f64 and `ctx` points to a live evaluation context.
#[unsafe(export_name = "rspice_ddt_jacobian_native")]
pub unsafe extern "C" fn rspice_ddt_jacobian_native(
    operands: *const f64,
    ctx: *const EvalContext,
    _state_id: usize,
) -> f64 {
    if operands.is_null() || ctx.is_null() {
        return invalid_native_integration_context(
            ctx,
            "ddt Jacobian",
            0,
            "missing operands or context",
        );
    }
    let ctx = unsafe { &*ctx };
    if ctx.integration_active != 0 {
        (unsafe { *operands }) * ctx.integration_derivative_scale
    } else {
        0.0
    }
}

unsafe fn rspice_integral_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    state_id: usize,
    wrapped: bool,
) -> f64 {
    let operator = if wrapped { "idtmod" } else { "idt" };
    if operands.is_null() || ctx.is_null() {
        return invalid_native_integration_context(
            ctx,
            operator,
            state_id,
            "missing operands or context",
        );
    }
    let ctx = unsafe { &*ctx };
    if !unsafe { native_state_storage_is_valid(ctx, state_id) } {
        return invalid_native_integration_context(
            ctx,
            operator,
            state_id,
            "has invalid state storage",
        );
    }
    let operands = unsafe { std::slice::from_raw_parts(operands, if wrapped { 4 } else { 2 }) };
    let input = operands[0];
    let initial_condition = operands[1];
    let initialized = unsafe { *ctx.state_initialized.add(state_id) != 0 };
    let previous = if initialized {
        unsafe { *ctx.state_prev.add(state_id) }
    } else {
        initial_condition
    };
    let older = if initialized {
        unsafe { *ctx.state_older.add(state_id) }
    } else {
        previous
    };
    let previous_input = if initialized {
        unsafe { *ctx.state_derivatives_prev.add(state_id) }
    } else {
        input
    };
    let raw = if ctx.integration_active != 0 {
        (input
            + previous * ctx.integration_previous_value_scale
            + older * ctx.integration_older_value_scale
            + previous_input * ctx.integration_previous_derivative_scale)
            / ctx.integration_derivative_scale
    } else {
        initial_condition
    };
    let (value, wrap_translation) = if wrapped {
        match idtmod_wrapped_candidate(raw, operands[2], operands[3]) {
            Ok(candidate) => candidate,
            Err(detail) => {
                let detail = format!(
                    "{detail}: raw={raw}, modulus={}, offset={}",
                    operands[2], operands[3]
                );
                return invalid_native_integration_context(ctx, operator, state_id, &detail);
            }
        }
    } else {
        (raw, 0.0)
    };
    let older_candidate = previous - wrap_translation;
    if !older_candidate.is_finite() {
        let detail = format!(
            "common-branch older history is not finite: previous={previous}, translation={wrap_translation}"
        );
        return invalid_native_integration_context(ctx, operator, state_id, &detail);
    }
    unsafe {
        *ctx.state_values.add(state_id) = value;
        *ctx.state_derivatives.add(state_id) = input;
        *ctx.state_older_candidate.add(state_id) = older_candidate;
        *ctx.state_candidate_valid.add(state_id) = 1;
    }
    value
}

/// Native companion evaluation for `idt`.
///
/// # Safety
/// `operands` points to two f64 values and `ctx` points to a live context.
#[unsafe(export_name = "rspice_idt_state_native")]
pub unsafe extern "C" fn rspice_idt_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    state_id: usize,
) -> f64 {
    unsafe { rspice_integral_state_native(operands, ctx, state_id, false) }
}

/// Native companion evaluation for `idtmod`.
///
/// # Safety
/// `operands` points to four f64 values and `ctx` points to a live context.
#[unsafe(export_name = "rspice_idtmod_state_native")]
pub unsafe extern "C" fn rspice_idtmod_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    state_id: usize,
) -> f64 {
    unsafe { rspice_integral_state_native(operands, ctx, state_id, true) }
}

/// Native companion Jacobian evaluation for `idt` and `idtmod`.
///
/// # Safety
/// `operands` points to one f64 and `ctx` points to a live context.
#[unsafe(export_name = "rspice_idt_jacobian_native")]
pub unsafe extern "C" fn rspice_idt_jacobian_native(
    operands: *const f64,
    ctx: *const EvalContext,
    _state_id: usize,
) -> f64 {
    if operands.is_null() || ctx.is_null() {
        return invalid_native_integration_context(
            ctx,
            "idt Jacobian",
            0,
            "missing operands or context",
        );
    }
    let ctx = unsafe { &*ctx };
    if ctx.integration_active != 0 {
        (unsafe { *operands }) / ctx.integration_derivative_scale
    } else {
        0.0
    }
}

/// External helper function for native x64 one-shot and periodic timer events.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer. Invalid pointers are reported through the native runtime error
/// channel; native mode never dispatches the bytecode interpreter for recovery.
#[unsafe(export_name = "rspice_timer_state_native")]
pub unsafe extern "C" fn rspice_timer_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    _timer_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native timer helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native timer helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 4) };
    if ctx.analysis_type != 2 {
        if !matches!(ctx.analysis_type, 0 | 1 | 3 | 4) {
            set_native_context_error(
                ctx,
                format!(
                    "native timer helper received invalid analysis type {}",
                    ctx.analysis_type
                ),
            );
        }
        return 0.0;
    }
    let (result, next_event) = crate::vm::timer_event_evaluation(
        operands[0],
        operands[1],
        operands[2],
        operands[3],
        ctx.time,
        ctx.timestep,
    );
    if let Some(next_event) = next_event
        && !ctx.timer_event_bound.is_null()
    {
        let bound = unsafe { &mut *ctx.timer_event_bound };
        *bound = bound.min(next_event);
    }
    result
}

/// External helper function for native x64 transition filters.
///
/// `operands` points to four contiguous f64 values emitted by the JIT in VM
/// stack order: input, delay, rise time, and fall time. Passing a pointer keeps
/// the x64 ABI simple on Windows, where four scalar arguments would force the
/// context and filter id onto the stack.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer and a valid four-element operand slice. Invalid pointers are
/// reported through the native runtime error channel; native mode never
/// dispatches the bytecode interpreter for recovery.
#[unsafe(export_name = "rspice_transition_state_native")]
pub unsafe extern "C" fn rspice_transition_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    filter_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native transition helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native transition helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 4) };
    let input = operands[0];
    let delay = operands[1];
    let rise_time = operands[2];
    let fall_time = operands[3];
    if let Err(error) =
        crate::vm::TransitionFilter::validate_operands(input, ctx.time, delay, rise_time, fall_time)
    {
        set_native_context_error(ctx, format!("native transition: {error}"));
        return 0.0;
    }
    if matches!(ctx.analysis_type, 1 | 3) {
        return input;
    }
    if !matches!(ctx.analysis_type, 0 | 2 | 4) {
        set_native_context_error(
            ctx,
            format!(
                "native transition helper received invalid analysis type {}",
                ctx.analysis_type
            ),
        );
        return 0.0;
    }

    if ctx.transition_filters.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native transition helper missing filter storage for filter {filter_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if filter_id >= ctx.transition_filters_len {
        set_native_context_error(
            ctx,
            format!(
                "native transition helper filter {filter_id} outside filter table length {}; no interpreter fallback",
                ctx.transition_filters_len
            ),
        );
        return 0.0;
    }

    let filters = unsafe {
        std::slice::from_raw_parts_mut(ctx.transition_filters, ctx.transition_filters_len)
    };
    let result = if ctx.analysis_type == 2 {
        filters[filter_id].eval(input, ctx.time, delay, rise_time, fall_time)
    } else {
        filters[filter_id].eval_operating_point(input, ctx.time, delay, rise_time, fall_time)
    };
    match result {
        Ok(value) => value,
        Err(error) => {
            set_native_context_error(ctx, format!("native transition: {error}"));
            0.0
        }
    }
}

/// Read-only exact local derivative of a native transition candidate.
///
/// `operands` contains input, input derivative, delay, rise time, and fall
/// time. Transient mode recomputes the candidate branch from accepted state;
/// operating-point and small-signal modes return the unity action required by
/// the LRM without touching transition history.
///
/// # Safety
/// Called from verified JIT code with a valid context and five operands.
#[unsafe(export_name = "rspice_transition_derivative_native")]
pub unsafe extern "C" fn rspice_transition_derivative_native(
    operands: *const f64,
    ctx: *const EvalContext,
    filter_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native transition derivative helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native transition derivative helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }
    let operands = unsafe { std::slice::from_raw_parts(operands, 5) };
    if !matches!(ctx.analysis_type, 0..=4) {
        set_native_context_error(
            ctx,
            format!(
                "native transition derivative helper received invalid analysis type {}",
                ctx.analysis_type
            ),
        );
        return 0.0;
    }

    if ctx.analysis_type != 2 {
        match crate::vm::TransitionFilter::validate_operands(
            operands[0],
            ctx.time,
            operands[2],
            operands[3],
            operands[4],
        ) {
            Ok(()) if operands[1].is_finite() => return operands[1],
            Ok(()) => {
                set_native_context_error(
                    ctx,
                    format!(
                        "native transition derivative: input derivative must be finite, got {}",
                        operands[1]
                    ),
                );
                return 0.0;
            }
            Err(error) => {
                set_native_context_error(ctx, format!("native transition derivative: {error}"));
                return 0.0;
            }
        }
    }

    if ctx.transition_filters.is_null() || filter_id >= ctx.transition_filters_len {
        set_native_context_error(
            ctx,
            format!(
                "native transition derivative helper filter {filter_id} outside available filter storage; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    let filters =
        unsafe { std::slice::from_raw_parts(ctx.transition_filters, ctx.transition_filters_len) };
    match filters[filter_id].eval_derivative(
        operands[0],
        operands[1],
        ctx.time,
        operands[2],
        operands[3],
        operands[4],
        ctx.analysis_type,
    ) {
        Ok(value) => value,
        Err(error) => {
            set_native_context_error(ctx, format!("native transition derivative: {error}"));
            0.0
        }
    }
}

/// External helper function for native x64 slew-rate filters.
///
/// `operands` points to three contiguous f64 values emitted by the JIT in VM
/// stack order: input, max positive slew, and max negative slew. Passing a
/// pointer keeps helper-call ABI handling identical to `transition(...)`.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer and a valid three-element operand slice. Invalid pointers are
/// reported through the native runtime error channel; native mode never
/// dispatches the bytecode interpreter for recovery.
#[unsafe(export_name = "rspice_slew_state_native")]
pub unsafe extern "C" fn rspice_slew_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    filter_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native slew helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native slew helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 3) };
    let input = operands[0];
    let max_pos_slew = operands[1];
    let max_neg_slew = operands[2];
    let NormalizedSlewRates::Limited(rates) =
        (match normalize_slew_rates(Some(max_pos_slew), Some(max_neg_slew)) {
            Ok(rates) => rates,
            Err(error) => {
                ctx.record_invalid_numeric_result(format!("slew: {error}"));
                return 0.0;
            }
        })
    else {
        set_native_context_error(ctx, "stateful slew helper encoded passthrough rates");
        return 0.0;
    };

    if ctx.slew_filters.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native slew helper missing filter storage for filter {filter_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if filter_id >= ctx.slew_filters_len {
        set_native_context_error(
            ctx,
            format!(
                "native slew helper filter {filter_id} outside filter table length {}; no interpreter fallback",
                ctx.slew_filters_len
            ),
        );
        return 0.0;
    }

    let filters = unsafe { std::slice::from_raw_parts_mut(ctx.slew_filters, ctx.slew_filters_len) };
    match ctx.analysis_type {
        2 => filters[filter_id].eval(input, ctx.time, rates),
        0 | 4 => filters[filter_id].eval_operating_point(input, ctx.time),
        // Small-signal analyses observe a unity transfer from the first
        // argument and never mutate accepted or candidate slew state.
        1 | 3 => input,
        _ => {
            set_native_context_error(
                ctx,
                format!(
                    "native slew helper received invalid analysis type {}",
                    ctx.analysis_type
                ),
            );
            0.0
        }
    }
}

/// Read-only exact local derivative of a native slew candidate.
///
/// # Safety
/// Called from verified JIT code with a valid context and six operands in
/// source/Jacobian order.
#[unsafe(export_name = "rspice_slew_derivative_native")]
pub unsafe extern "C" fn rspice_slew_derivative_native(
    operands: *const f64,
    ctx: *const EvalContext,
    filter_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native slew derivative helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native slew derivative helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }
    let operands = unsafe { std::slice::from_raw_parts(operands, 6) };
    let NormalizedSlewRates::Limited(rates) =
        (match normalize_slew_rates(Some(operands[2]), Some(operands[4])) {
            Ok(rates) => rates,
            Err(error) => {
                ctx.record_invalid_numeric_result(format!("slew derivative: {error}"));
                return 0.0;
            }
        })
    else {
        set_native_context_error(
            ctx,
            "stateful slew derivative helper encoded passthrough rates",
        );
        return 0.0;
    };
    if ctx.slew_filters.is_null() || filter_id >= ctx.slew_filters_len {
        set_native_context_error(
            ctx,
            format!(
                "native slew derivative helper filter {filter_id} outside available filter storage; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    match ctx.analysis_type {
        2 => {
            let filters =
                unsafe { std::slice::from_raw_parts(ctx.slew_filters, ctx.slew_filters_len) };
            filters[filter_id].eval_derivative(
                operands[0],
                operands[1],
                operands[3],
                operands[5],
                ctx.time,
                rates,
            )
        }
        0 | 1 | 3 | 4 => operands[1],
        analysis_type => {
            set_native_context_error(
                ctx,
                format!(
                    "native slew derivative helper received invalid analysis type {analysis_type}"
                ),
            );
            0.0
        }
    }
}

/// External helper function for native x64 absolute-delay buffers.
///
/// `operands` points to two contiguous f64 values emitted by the JIT in VM
/// stack order: input and delay time. Native mode requires preallocated delay
/// buffer storage in transient analysis and hard-fails rather than dispatching
/// the bytecode interpreter when storage is missing.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer and a valid two-element operand slice. Invalid pointers are reported
/// through the native runtime error channel.
#[unsafe(export_name = "rspice_absdelay_state_native")]
pub unsafe extern "C" fn rspice_absdelay_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    buffer_id: usize,
) -> f64 {
    unsafe { rspice_absdelay_native_impl(operands, ctx, buffer_id, false, false) }
}

#[unsafe(export_name = "rspice_absdelay_state_max_native")]
pub unsafe extern "C" fn rspice_absdelay_state_max_native(
    operands: *const f64,
    ctx: *const EvalContext,
    buffer_id: usize,
) -> f64 {
    unsafe { rspice_absdelay_native_impl(operands, ctx, buffer_id, true, false) }
}

#[unsafe(export_name = "rspice_absdelay_derivative_native")]
pub unsafe extern "C" fn rspice_absdelay_derivative_native(
    operands: *const f64,
    ctx: *const EvalContext,
    buffer_id: usize,
) -> f64 {
    unsafe { rspice_absdelay_native_impl(operands, ctx, buffer_id, false, true) }
}

#[unsafe(export_name = "rspice_absdelay_derivative_max_native")]
pub unsafe extern "C" fn rspice_absdelay_derivative_max_native(
    operands: *const f64,
    ctx: *const EvalContext,
    buffer_id: usize,
) -> f64 {
    unsafe { rspice_absdelay_native_impl(operands, ctx, buffer_id, true, true) }
}

unsafe fn rspice_absdelay_native_impl(
    operands: *const f64,
    ctx: *const EvalContext,
    buffer_id: usize,
    has_max_delay: bool,
    derivative: bool,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native absdelay helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native absdelay helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }

    let operand_count = match (has_max_delay, derivative) {
        (false, false) => 2,
        (true, false) => 3,
        (false, true) => 4,
        (true, true) => 5,
    };
    let operands = unsafe { std::slice::from_raw_parts(operands, operand_count) };
    let input = operands[0];
    let (input_derivative, delay_time, delay_derivative, max_delay) = if derivative {
        (
            operands[1],
            operands[2],
            operands[3],
            if has_max_delay {
                Some(operands[4])
            } else {
                None
            },
        )
    } else {
        (
            0.0,
            operands[1],
            0.0,
            if has_max_delay {
                Some(operands[2])
            } else {
                None
            },
        )
    };
    if ctx.delay_buffers.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native absdelay helper missing delay-buffer storage for buffer {buffer_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if buffer_id >= ctx.delay_buffers_len {
        set_native_context_error(
            ctx,
            format!(
                "native absdelay helper buffer {buffer_id} outside delay-buffer table length {}; no interpreter fallback",
                ctx.delay_buffers_len
            ),
        );
        return 0.0;
    }

    let buffers =
        unsafe { std::slice::from_raw_parts_mut(ctx.delay_buffers, ctx.delay_buffers_len) };
    if ctx.analysis_type != 2 {
        if !matches!(ctx.analysis_type, 0 | 1 | 3 | 4) {
            set_native_context_error(
                ctx,
                format!(
                    "native absdelay helper received invalid analysis type {}",
                    ctx.analysis_type
                ),
            );
            return 0.0;
        }
        let result = if derivative {
            buffers[buffer_id]
                .small_signal_delay(ctx.time, input, delay_time, max_delay)
                .map(|_| input_derivative)
        } else {
            buffers[buffer_id].eval_operating_point(ctx.time, input, delay_time, max_delay)
        };
        return match result {
            Ok(value) => value,
            Err(error) => {
                set_native_context_error(ctx, error);
                0.0
            }
        };
    }
    match buffers[buffer_id].eval_with_coefficients(ctx.time, input, delay_time, max_delay) {
        Ok(evaluation) => {
            if derivative {
                evaluation.delay_coefficient.mul_add(
                    delay_derivative,
                    evaluation.input_coefficient * input_derivative,
                )
            } else {
                evaluation.output
            }
        }
        Err(error) => {
            set_native_context_error(ctx, error);
            0.0
        }
    }
}

/// External helper function for native x64 threshold-crossing detectors.
///
/// `operands` points to five contiguous f64 values emitted by the JIT in VM
/// stack order: expression, direction, time tolerance, expression tolerance,
/// and enable. Native mode requires
/// preallocated detector storage and hard-fails through the native runtime
/// error channel when it is missing.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer and a valid five-element operand slice. Invalid pointers are reported
/// through the native runtime error channel.
#[unsafe(export_name = "rspice_cross_state_native")]
pub unsafe extern "C" fn rspice_cross_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    detector_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native cross helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native cross helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 5) };
    let value = operands[0];
    let direction_raw = operands[1];
    let time_tol = operands[2];
    let expr_tol = operands[3];
    let direction = match event_integer_operand("cross direction", direction_raw) {
        Ok(direction) => direction,
        Err(error) => {
            set_native_context_error(ctx, error);
            return 0.0;
        }
    };
    let enabled = match event_integer_operand("cross enable", operands[4]) {
        Ok(enable) => enable != 0,
        Err(error) => {
            set_native_context_error(ctx, error);
            return 0.0;
        }
    };
    if ctx.cross_detectors.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native cross helper missing detector storage for detector {detector_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if detector_id >= ctx.cross_detectors_len {
        set_native_context_error(
            ctx,
            format!(
                "native cross helper detector {detector_id} outside detector table length {}; no interpreter fallback",
                ctx.cross_detectors_len
            ),
        );
        return 0.0;
    }

    let detectors =
        unsafe { std::slice::from_raw_parts_mut(ctx.cross_detectors, ctx.cross_detectors_len) };
    let crossed = match detectors[detector_id]
        .eval_event(value, ctx.time, direction, time_tol, expr_tol, enabled)
    {
        Ok(crossed) => crossed,
        Err(error) => {
            set_native_context_error(ctx, format!("cross evaluation failed: {error}"));
            return 0.0;
        }
    };
    if ctx.analysis_type == 2 { crossed } else { 0.0 }
}

/// External helper function for native x64 `above(...)` event detectors.
///
/// `operands` contains expression, time tolerance, expression tolerance, and
/// enable. Detector storage is shared with `cross(...)`, with a distinct slot
/// allocated for every expression.
///
/// # Safety
/// Called from JIT code with a valid four-element operand slice and evaluation
/// context. Invalid pointers and state slots are reported through the native
/// runtime error channel.
#[unsafe(export_name = "rspice_above_state_native")]
pub unsafe extern "C" fn rspice_above_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    detector_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native above helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native above helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 4) };
    let enabled = match event_integer_operand("above enable", operands[3]) {
        Ok(enable) => enable != 0,
        Err(error) => {
            set_native_context_error(ctx, error);
            return 0.0;
        }
    };
    if ctx.cross_detectors.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native above helper missing detector storage for detector {detector_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if detector_id >= ctx.cross_detectors_len {
        set_native_context_error(
            ctx,
            format!(
                "native above helper detector {detector_id} outside detector table length {}; no interpreter fallback",
                ctx.cross_detectors_len
            ),
        );
        return 0.0;
    }

    let detectors =
        unsafe { std::slice::from_raw_parts_mut(ctx.cross_detectors, ctx.cross_detectors_len) };
    let result = if matches!(ctx.analysis_type, 0 | 4) {
        detectors[detector_id].eval_above_static(
            operands[0],
            ctx.time,
            operands[1],
            operands[2],
            enabled,
        )
    } else {
        detectors[detector_id].eval_above(operands[0], ctx.time, operands[1], operands[2], enabled)
    };
    match result {
        Ok(result) => result,
        Err(error) => {
            set_native_context_error(ctx, format!("above evaluation failed: {error}"));
            0.0
        }
    }
}

/// External helper for native x64 `last_crossing(...)` history.
///
/// # Safety
/// Called from JIT code with a valid two-element operand slice containing the
/// expression value and direction, plus a valid evaluation context.
#[unsafe(export_name = "rspice_last_crossing_state_native")]
pub unsafe extern "C" fn rspice_last_crossing_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    detector_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_context_error_ptr(
            ctx,
            "native last_crossing helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }
    let ctx = unsafe { &*ctx };
    if operands.is_null() {
        set_native_context_error(
            ctx,
            "native last_crossing helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 2) };
    let direction = match event_integer_operand("last_crossing direction", operands[1]) {
        Ok(direction) => direction,
        Err(error) => {
            set_native_context_error(ctx, error);
            return -1.0;
        }
    };
    if ctx.cross_detectors.is_null() {
        set_native_context_error(
            ctx,
            format!(
                "native last_crossing helper missing detector storage for detector {detector_id}; no interpreter fallback"
            ),
        );
        return 0.0;
    }
    if detector_id >= ctx.cross_detectors_len {
        set_native_context_error(
            ctx,
            format!(
                "native last_crossing helper detector {detector_id} outside detector table length {}; no interpreter fallback",
                ctx.cross_detectors_len
            ),
        );
        return 0.0;
    }

    let detectors =
        unsafe { std::slice::from_raw_parts_mut(ctx.cross_detectors, ctx.cross_detectors_len) };
    let crossing_time =
        match detectors[detector_id].eval_last_crossing(operands[0], ctx.time, direction) {
            Ok(crossing_time) => crossing_time,
            Err(error) => {
                set_native_context_error(ctx, format!("last_crossing evaluation failed: {error}"));
                return -1.0;
            }
        };
    if ctx.analysis_type == 2 {
        crossing_time
    } else {
        -1.0
    }
}

/// Resolve a native x64 runtime-indexed variable slot.
///
/// This resolver is deliberately side-effect free. A null return means the
/// generated caller must publish a dispatch-local diagnostic before returning.
///
/// # Safety
/// A non-null `base_ptr` must point to at least `len` contiguous `f64` values.
#[unsafe(export_name = "rspice_dynamic_variable_slot_native")]
pub unsafe extern "C" fn rspice_dynamic_variable_slot_native(
    raw_index: f64,
    base_ptr: *mut f64,
    len: usize,
    lower: i64,
) -> *mut f64 {
    if base_ptr.is_null() || len == 0 {
        return std::ptr::null_mut();
    }
    let Ok(len_i64) = i64::try_from(len) else {
        return std::ptr::null_mut();
    };

    let Some(offset) = dynamic_variable_offset(raw_index, len_i64, lower) else {
        return std::ptr::null_mut();
    };

    unsafe { base_ptr.add(offset) }
}

/// Publish a runtime-indexed variable failure to the active dispatch.
///
/// The argument layout matches the slot resolver except that the first integer
/// argument is the dispatch context rather than the variable base pointer.
#[unsafe(export_name = "rspice_native_dynamic_variable_error")]
pub extern "C" fn rspice_native_dynamic_variable_error(
    raw_index: f64,
    ctx: *const EvalContext,
    len: usize,
    lower: i64,
) -> f64 {
    let message = if len == 0 {
        "native dynamic variable access has zero-length storage; no interpreter fallback".into()
    } else if let Ok(len_i64) = i64::try_from(len) {
        if let Ok(index) = checked_rounded_i64(raw_index) {
            let offset = index.checked_sub(lower);
            if offset.is_some_and(|offset| offset >= 0 && offset < len_i64) {
                "native dynamic variable access missing variable storage; no interpreter fallback"
                    .into()
            } else {
                dynamic_variable_bounds_error(index, lower, len_i64)
            }
        } else {
            dynamic_variable_bounds_error(raw_index, lower, len_i64)
        }
    } else {
        "native dynamic variable length exceeds native bounds range; no interpreter fallback".into()
    };
    set_native_context_error_ptr(ctx, message);
    0.0
}

fn dynamic_variable_offset(raw_index: f64, len_i64: i64, lower: i64) -> Option<usize> {
    let len = usize::try_from(len_i64).ok()?;
    checked_array_slot(raw_index, 0, len, lower).ok()
}

fn dynamic_variable_bounds_error(
    index: impl std::fmt::Display,
    lower: i64,
    len_i64: i64,
) -> String {
    let upper = lower.saturating_add(len_i64).saturating_sub(1);
    format!(
        "native dynamic variable access: array index {index} outside declared bounds [{lower}:{upper}]; no interpreter fallback"
    )
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{
        EvalContext, INTEGER_CAST_DESCRIPTOR, NativeRuntimeStatus, integer_binary_descriptor,
        rspice_above_state_native, rspice_absdelay_derivative_max_native,
        rspice_absdelay_derivative_native, rspice_absdelay_state_max_native,
        rspice_absdelay_state_native, rspice_cross_state_native, rspice_ddt_state_native,
        rspice_dynamic_variable_slot_native, rspice_idt_state_native,
        rspice_integer_operation_native, rspice_laplace_derivative_native,
        rspice_laplace_step_native, rspice_last_crossing_state_native,
        rspice_limiter_previous_native, rspice_limiter_store_native,
        rspice_native_dynamic_variable_error, rspice_slew_state_native,
        rspice_table_derivative_native, rspice_table_lookup_native, rspice_timer_state_native,
        rspice_transition_derivative_native, rspice_transition_state_native, rspice_zi_step_native,
    };
    use crate::codegen::LookupTable;
    use crate::integer_runtime::IntegerBinaryOperation;
    use crate::vm::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter};
    use std::mem::{align_of, offset_of, size_of};

    #[test]
    fn eval_context_layout_matches_x64_jit_offsets() {
        assert_eq!(offset_of!(EvalContext, voltages), 0);
        assert_eq!(offset_of!(EvalContext, internal_voltages), 8);
        assert_eq!(offset_of!(EvalContext, params), 16);
        assert_eq!(offset_of!(EvalContext, branch_currents), 24);
        assert_eq!(offset_of!(EvalContext, branch_currents_len), 32);
        assert_eq!(offset_of!(EvalContext, currents), 40);
        assert_eq!(offset_of!(EvalContext, currents_len), 48);
        assert_eq!(offset_of!(EvalContext, num_terminals), 56);
        assert_eq!(offset_of!(EvalContext, port_connected), 64);
        assert_eq!(offset_of!(EvalContext, port_connected_len), 72);
        assert_eq!(offset_of!(EvalContext, temperature), 80);
        assert_eq!(offset_of!(EvalContext, time), 88);
        assert_eq!(offset_of!(EvalContext, timestep), 96);
        assert_eq!(offset_of!(EvalContext, state_prev), 104);
        assert_eq!(offset_of!(EvalContext, state_values), 112);
        assert_eq!(offset_of!(EvalContext, state_initialized), 120);
        assert_eq!(offset_of!(EvalContext, state_initialized_len), 128);
        assert_eq!(offset_of!(EvalContext, lookup_tables), 136);
        assert_eq!(offset_of!(EvalContext, lookup_tables_len), 144);
        assert_eq!(offset_of!(EvalContext, laplace_filters), 152);
        assert_eq!(offset_of!(EvalContext, laplace_filters_len), 160);
        assert_eq!(offset_of!(EvalContext, param_given), 168);
        assert_eq!(offset_of!(EvalContext, param_given_len), 176);
        assert_eq!(offset_of!(EvalContext, branch_unknowns), 184);
        assert_eq!(offset_of!(EvalContext, analysis_type), 192);
        assert_eq!(offset_of!(EvalContext, multiplicity), 200);
        assert_eq!(offset_of!(EvalContext, zi_filters), 208);
        assert_eq!(offset_of!(EvalContext, zi_filters_len), 216);
        assert_eq!(offset_of!(EvalContext, transition_filters), 224);
        assert_eq!(offset_of!(EvalContext, transition_filters_len), 232);
        assert_eq!(offset_of!(EvalContext, slew_filters), 240);
        assert_eq!(offset_of!(EvalContext, slew_filters_len), 248);
        assert_eq!(offset_of!(EvalContext, delay_buffers), 256);
        assert_eq!(offset_of!(EvalContext, delay_buffers_len), 264);
        assert_eq!(offset_of!(EvalContext, cross_detectors), 272);
        assert_eq!(offset_of!(EvalContext, cross_detectors_len), 280);
        assert_eq!(offset_of!(EvalContext, state_prev_len), 288);
        assert_eq!(offset_of!(EvalContext, state_values_len), 296);
        assert_eq!(offset_of!(EvalContext, timer_event_bound), 304);
        assert_eq!(offset_of!(EvalContext, analysis_initial_step), 312);
        assert_eq!(offset_of!(EvalContext, analysis_final_step), 313);
        assert_eq!(offset_of!(EvalContext, state_older), 320);
        assert_eq!(offset_of!(EvalContext, state_older_len), 328);
        assert_eq!(offset_of!(EvalContext, state_derivatives), 336);
        assert_eq!(offset_of!(EvalContext, state_derivatives_len), 344);
        assert_eq!(offset_of!(EvalContext, state_derivatives_prev), 352);
        assert_eq!(offset_of!(EvalContext, state_derivatives_prev_len), 360);
        assert_eq!(offset_of!(EvalContext, integration_derivative_scale), 368);
        assert_eq!(
            offset_of!(EvalContext, integration_previous_value_scale),
            376
        );
        assert_eq!(offset_of!(EvalContext, integration_older_value_scale), 384);
        assert_eq!(
            offset_of!(EvalContext, integration_previous_derivative_scale),
            392
        );
        assert_eq!(offset_of!(EvalContext, integration_active), 400);
        assert_eq!(offset_of!(EvalContext, limiter_active), 408);
        assert_eq!(offset_of!(EvalContext, limiting_enabled), 416);
        assert_eq!(offset_of!(EvalContext, runtime_status), 424);
        assert_eq!(offset_of!(EvalContext, state_candidate_valid), 456);
        assert_eq!(offset_of!(EvalContext, state_candidate_valid_len), 464);
        assert_eq!(offset_of!(EvalContext, state_older_candidate), 472);
        assert_eq!(offset_of!(EvalContext, state_older_candidate_len), 480);
        assert_eq!(offset_of!(NativeRuntimeStatus, failed), 0);
        assert_eq!(
            NativeRuntimeStatus::failed_offset(),
            offset_of!(NativeRuntimeStatus, failed)
        );
        assert_eq!(size_of::<EvalContext>(), 488);
        assert_eq!(align_of::<EvalContext>(), 8);
    }

    #[test]
    fn integration_helpers_validate_every_history_buffer_length() {
        let operand = [1.0];

        for missing in [
            "older",
            "derivatives",
            "previous derivatives",
            "candidate status",
            "older candidate",
        ] {
            let previous = [0.0];
            let older = [0.0];
            let previous_derivatives = [0.0];
            let mut values = [0.0];
            let mut derivatives = [0.0];
            let mut initialized = [0_u8];
            let mut candidate_valid = [0_u8];
            let mut older_candidate = [0.0];
            let mut ctx = empty_eval_context();
            ctx.state_prev = previous.as_ptr();
            ctx.state_prev_len = previous.len();
            ctx.state_older = older.as_ptr();
            ctx.state_older_len = older.len();
            ctx.state_values = values.as_mut_ptr();
            ctx.state_values_len = values.len();
            ctx.state_derivatives = derivatives.as_mut_ptr();
            ctx.state_derivatives_len = derivatives.len();
            ctx.state_derivatives_prev = previous_derivatives.as_ptr();
            ctx.state_derivatives_prev_len = previous_derivatives.len();
            ctx.state_initialized = initialized.as_mut_ptr();
            ctx.state_initialized_len = initialized.len();
            ctx.state_candidate_valid = candidate_valid.as_mut_ptr();
            ctx.state_candidate_valid_len = candidate_valid.len();
            ctx.state_older_candidate = older_candidate.as_mut_ptr();
            ctx.state_older_candidate_len = older_candidate.len();
            match missing {
                "older" => ctx.state_older_len = 0,
                "derivatives" => ctx.state_derivatives_len = 0,
                "previous derivatives" => ctx.state_derivatives_prev_len = 0,
                "candidate status" => ctx.state_candidate_valid_len = 0,
                "older candidate" => ctx.state_older_candidate_len = 0,
                _ => unreachable!(),
            }
            ctx.clear_runtime_error();

            let value = unsafe { rspice_ddt_state_native(operand.as_ptr(), &ctx, 0) };

            assert_eq!(value.to_bits(), 0.0_f64.to_bits(), "{missing}");
            let error = ctx
                .take_runtime_error()
                .unwrap_or_else(|| panic!("{missing} length must hard-fail"));
            assert!(
                error.contains("ddt") && error.contains("invalid state storage"),
                "{missing}: unexpected error: {error}"
            );
        }
    }

    #[test]
    fn native_integration_helpers_publish_only_speculative_candidate_status() {
        let previous = [0.0];
        let older = [0.0];
        let previous_derivatives = [0.0];
        let mut values = [0.0];
        let mut derivatives = [0.0];
        let mut initialized = [0_u8];
        let mut candidate_valid = [0_u8];
        let mut older_candidate = [0.0];
        let mut ctx = empty_eval_context();
        ctx.state_prev = previous.as_ptr();
        ctx.state_prev_len = previous.len();
        ctx.state_older = older.as_ptr();
        ctx.state_older_len = older.len();
        ctx.state_values = values.as_mut_ptr();
        ctx.state_values_len = values.len();
        ctx.state_derivatives = derivatives.as_mut_ptr();
        ctx.state_derivatives_len = derivatives.len();
        ctx.state_derivatives_prev = previous_derivatives.as_ptr();
        ctx.state_derivatives_prev_len = previous_derivatives.len();
        ctx.state_initialized = initialized.as_mut_ptr();
        ctx.state_initialized_len = initialized.len();
        ctx.state_candidate_valid = candidate_valid.as_mut_ptr();
        ctx.state_candidate_valid_len = candidate_valid.len();
        ctx.state_older_candidate = older_candidate.as_mut_ptr();
        ctx.state_older_candidate_len = older_candidate.len();
        ctx.integration_active = 1;
        ctx.integration_derivative_scale = 1.0;
        ctx.integration_previous_value_scale = 1.0;

        let first = [2.0, 10.0];
        assert_eq!(
            unsafe { rspice_idt_state_native(first.as_ptr(), &ctx, 0) }.to_bits(),
            12.0_f64.to_bits()
        );
        assert_eq!(initialized[0], 0, "accepted state must remain untouched");
        assert_eq!(candidate_valid[0], 1);
        assert_eq!(older_candidate[0].to_bits(), 10.0_f64.to_bits());

        candidate_valid[0] = 0;
        let rejected_retry = [3.0, 20.0];
        assert_eq!(
            unsafe { rspice_idt_state_native(rejected_retry.as_ptr(), &ctx, 0) }.to_bits(),
            23.0_f64.to_bits()
        );
        assert_eq!(initialized[0], 0);
        assert_eq!(candidate_valid[0], 1);
        assert_eq!(older_candidate[0].to_bits(), 20.0_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn named_limiter_helpers_bypass_state_outside_limited_newton() {
        let mut ctx = empty_eval_context();
        ctx.limiting_enabled = 0;
        ctx.clear_runtime_error();

        assert_eq!(
            unsafe { rspice_limiter_previous_native(3.5, &ctx, usize::MAX) }.to_bits(),
            3.5_f64.to_bits()
        );
        let operands = [3.5, 9.0];
        assert_eq!(
            unsafe { rspice_limiter_store_native(operands.as_ptr(), &ctx, usize::MAX) }.to_bits(),
            3.5_f64.to_bits()
        );
        assert!(
            ctx.take_runtime_error().is_none(),
            "probe and small-signal limiter bypass must not require state storage"
        );
    }

    #[test]
    fn named_limiter_helpers_publish_state_and_convergence() {
        let mut state_values = [0.0_f64];
        let mut state_initialized = [0_u8];
        let mut limiter_active = 0_u8;
        let mut ctx = empty_eval_context();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();
        ctx.limiter_active = &mut limiter_active;
        ctx.limiting_enabled = 1;

        assert_eq!(
            unsafe { rspice_limiter_previous_native(3.5, &ctx, 0) }.to_bits(),
            3.5_f64.to_bits()
        );
        let changed = [3.5, 4.0];
        assert_eq!(
            unsafe { rspice_limiter_store_native(changed.as_ptr(), &ctx, 0) }.to_bits(),
            4.0_f64.to_bits()
        );
        assert_eq!(state_values[0].to_bits(), 4.0_f64.to_bits());
        assert_eq!(state_initialized[0], 1);
        assert_eq!(limiter_active, 1);

        limiter_active = 0;
        let unchanged = [4.0, 4.0];
        assert_eq!(
            unsafe { rspice_limiter_store_native(unchanged.as_ptr(), &ctx, 0) }.to_bits(),
            4.0_f64.to_bits()
        );
        assert_eq!(limiter_active, 0);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn table_native_helpers_record_runtime_error_for_missing_storage() {
        for (name, helper) in [
            (
                "lookup",
                rspice_table_lookup_native
                    as unsafe extern "C" fn(f64, *const EvalContext, usize) -> f64,
            ),
            ("derivative", rspice_table_derivative_native),
        ] {
            let ctx = empty_eval_context();
            ctx.clear_runtime_error();
            let value = unsafe { helper(1.0, &ctx, 0) };
            assert_eq!(value.to_bits(), 0.0_f64.to_bits(), "{name}");
            let error = ctx
                .take_runtime_error()
                .unwrap_or_else(|| panic!("{name} missing table storage must hard-fail"));
            assert!(error.contains("table"), "{name}: {error}");
            assert!(error.contains("table storage"), "{name}: {error}");
            assert!(error.contains("no interpreter fallback"), "{name}: {error}");
        }
    }

    #[test]
    fn table_native_helpers_record_runtime_error_for_out_of_range_table_id() {
        let table = [LookupTable::from_data(vec![0.0, 1.0], vec![2.0, 4.0])];
        for (name, helper) in [
            (
                "lookup",
                rspice_table_lookup_native
                    as unsafe extern "C" fn(f64, *const EvalContext, usize) -> f64,
            ),
            ("derivative", rspice_table_derivative_native),
        ] {
            let mut ctx = empty_eval_context();
            ctx.lookup_tables = table.as_ptr();
            ctx.lookup_tables_len = table.len();
            ctx.clear_runtime_error();
            let value = unsafe { helper(1.0, &ctx, 1) };
            assert_eq!(value.to_bits(), 0.0_f64.to_bits(), "{name}");
            let error = ctx
                .take_runtime_error()
                .unwrap_or_else(|| panic!("{name} out-of-range table id must hard-fail"));
            assert!(error.contains("table"), "{name}: {error}");
            assert!(error.contains("outside table length"), "{name}: {error}");
            assert!(error.contains("no interpreter fallback"), "{name}: {error}");
        }
    }

    #[test]
    fn native_helper_records_failure_in_its_evaluation_context() {
        let ctx = empty_eval_context();
        ctx.clear_runtime_error();

        let value = unsafe { rspice_laplace_step_native(1.25, &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("native helper must publish failure to its dispatch context");
        assert!(error.contains("Laplace") && error.contains("filter storage"));
        assert!(
            ctx.take_runtime_error().is_none(),
            "dispatch error retrieval must clear the context status"
        );
    }

    #[test]
    fn laplace_native_helper_seeds_inactive_transient_equilibrium_on_acceptance() {
        let mut filters = [crate::laplace::StateSpaceFilter::integrator(1.0)
            .expect("first-order low-pass realization")];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.timestep = 0.0;
        ctx.integration_active = 0;
        ctx.laplace_filters = filters.as_mut_ptr();
        ctx.laplace_filters_len = filters.len();

        filters[0].begin_evaluation();
        assert_eq!(unsafe { rspice_laplace_step_native(4.0, &ctx, 0) }, 4.0);
        assert_eq!(filters[0].checkpoint().state, vec![0.0]);

        filters[0].begin_evaluation();
        assert_eq!(unsafe { rspice_laplace_step_native(6.0, &ctx, 0) }, 6.0);
        filters[0].commit();
        assert_eq!(filters[0].checkpoint().state, vec![6.0]);

        ctx.timestep = 0.5;
        ctx.integration_active = 1;
        ctx.integration_derivative_scale = 2.0;
        ctx.integration_previous_value_scale = 2.0;
        filters[0].begin_evaluation();
        let first_step = unsafe { rspice_laplace_step_native(2.0, &ctx, 0) };
        assert!((first_step - (14.0 / 3.0)).abs() <= 1.0e-12);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn laplace_native_derivative_helper_is_exact_read_only_and_checked() {
        let mut filters =
            [
                crate::laplace::StateSpaceFilter::from_transfer_function(&[1.0], &[1.0, 1.0])
                    .expect("valid first-order Laplace filter"),
            ];
        let mut ctx = empty_eval_context();
        ctx.laplace_filters = filters.as_mut_ptr();
        ctx.laplace_filters_len = filters.len();

        let initial = filters[0].checkpoint();
        assert_eq!(
            unsafe { rspice_laplace_derivative_native(2.0, &ctx, 0) }.to_bits(),
            2.0_f64.to_bits(),
            "non-transient derivative uses the DC action"
        );
        assert_eq!(filters[0].checkpoint(), initial);

        ctx.analysis_type = 2;
        ctx.timestep = 0.5;
        ctx.integration_active = 1;
        ctx.integration_derivative_scale = 2.0;
        ctx.integration_previous_value_scale = 2.0;
        let transient = unsafe { rspice_laplace_derivative_native(2.0, &ctx, 0) };
        assert!((transient - (2.0 / 3.0)).abs() <= 1.0e-15);
        assert_eq!(filters[0].checkpoint(), initial);
        assert!(ctx.take_native_runtime_error().is_none());

        let underflow = unsafe { rspice_laplace_derivative_native(f64::from_bits(1), &ctx, 0) };
        assert_eq!(underflow.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_native_runtime_error()
            .expect("unrepresentable derivative must publish a typed runtime error");
        assert_eq!(
            error.kind,
            super::NativeRuntimeErrorKind::InvalidNumericResult
        );
        assert!(error.message.contains("input action is not representable"));
        assert_eq!(filters[0].checkpoint(), initial);
    }

    #[test]
    fn laplace_native_helpers_follow_trapezoidal_and_gear2_coefficients() {
        let mut filters = [crate::laplace::StateSpaceFilter::integrator(1.0)
            .expect("first-order low-pass realization")];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.integration_active = 1;
        ctx.laplace_filters = filters.as_mut_ptr();
        ctx.laplace_filters_len = filters.len();

        ctx.integration_derivative_scale = 4.0;
        ctx.integration_previous_value_scale = 4.0;
        ctx.integration_older_value_scale = 0.0;
        ctx.integration_previous_derivative_scale = 1.0;
        filters[0].begin_evaluation();
        let trap_gain = unsafe { rspice_laplace_derivative_native(1.0, &ctx, 0) };
        assert!((trap_gain - 0.2).abs() <= 8.0 * f64::EPSILON);
        let trap = unsafe { rspice_laplace_step_native(2.0, &ctx, 0) };
        assert!((trap - 0.4).abs() <= 8.0 * f64::EPSILON);
        filters[0].commit();

        ctx.integration_derivative_scale = 3.0;
        ctx.integration_previous_value_scale = 4.0;
        ctx.integration_older_value_scale = -1.0;
        ctx.integration_previous_derivative_scale = 0.0;
        filters[0].begin_evaluation();
        let gear_gain = unsafe { rspice_laplace_derivative_native(1.0, &ctx, 0) };
        assert!((gear_gain - 0.25).abs() <= 8.0 * f64::EPSILON);
        let gear = unsafe { rspice_laplace_step_native(3.0, &ctx, 0) };
        assert!((gear - 1.15).abs() <= 16.0 * f64::EPSILON);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn zi_native_helper_records_runtime_error_for_missing_storage() {
        let ctx = empty_eval_context();
        let operands = [1.0, 1.0, 1.0, 0.0, 1.25, 0.0];
        let descriptor = crate::codegen::ZiRuntimeLayout::unit_coefficients(0)
            .native_descriptor()
            .expect("unit Zi descriptor");

        let value = unsafe { rspice_zi_step_native(operands.as_ptr(), &ctx, descriptor) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing native zi storage must record an error");
        assert!(
            error.contains("zi") && error.contains("filter storage"),
            "error must identify the missing zi storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn timer_native_helper_matches_vm_events() {
        let periodic = [1.0, 0.5, 0.0, 1.0];

        let mut timer_bound = f64::INFINITY;
        let mut ctx = EvalContext {
            voltages: std::ptr::null(),
            internal_voltages: std::ptr::null(),
            params: std::ptr::null(),
            branch_currents: std::ptr::null(),
            branch_currents_len: 0,
            currents: std::ptr::null(),
            currents_len: 0,
            num_terminals: 0,
            port_connected: std::ptr::null(),
            port_connected_len: 0,
            temperature: 0.0,
            time: 0.0,
            timestep: 0.0,
            state_prev: std::ptr::null(),
            state_values: std::ptr::null_mut(),
            state_initialized: std::ptr::null_mut(),
            state_initialized_len: 0,
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
            param_given_len: 0,
            branch_unknowns: std::ptr::null(),
            analysis_type: 2,
            multiplicity: 1.0,
            zi_filters: std::ptr::null_mut(),
            zi_filters_len: 0,
            transition_filters: std::ptr::null_mut(),
            transition_filters_len: 0,
            slew_filters: std::ptr::null_mut(),
            slew_filters_len: 0,
            delay_buffers: std::ptr::null_mut(),
            delay_buffers_len: 0,
            cross_detectors: std::ptr::null_mut(),
            cross_detectors_len: 0,
            state_prev_len: 0,
            state_values_len: 0,
            timer_event_bound: &mut timer_bound,
            analysis_initial_step: 0,
            analysis_final_step: 0,
            state_older: std::ptr::null(),
            state_older_len: 0,
            state_derivatives: std::ptr::null_mut(),
            state_derivatives_len: 0,
            state_derivatives_prev: std::ptr::null(),
            state_derivatives_prev_len: 0,
            integration_derivative_scale: 0.0,
            integration_previous_value_scale: 0.0,
            integration_older_value_scale: 0.0,
            integration_previous_derivative_scale: 0.0,
            integration_active: 0,
            limiter_active: std::ptr::null_mut(),
            limiting_enabled: 0,
            runtime_status: Default::default(),
            state_candidate_valid: std::ptr::null_mut(),
            state_candidate_valid_len: 0,
            state_older_candidate: std::ptr::null_mut(),
            state_older_candidate_len: 0,
        };

        assert_eq!(
            unsafe { rspice_timer_state_native(periodic.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(timer_bound.to_bits(), 1.0_f64.to_bits());

        timer_bound = f64::INFINITY;
        ctx.timestep = 1.0;
        ctx.time = 1.0;
        assert_eq!(
            unsafe { rspice_timer_state_native(periodic.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "periodic timer should fire at its first scheduled event"
        );
        assert_eq!(timer_bound.to_bits(), 1.5_f64.to_bits());

        timer_bound = f64::INFINITY;
        ctx.timestep = 0.5;
        ctx.time = 1.5;
        assert_eq!(
            unsafe { rspice_timer_state_native(periodic.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "periodic timer should fire at every positive period"
        );
        assert_eq!(timer_bound.to_bits(), 2.0_f64.to_bits());

        let one_shot = [1.0, 0.0, 0.0, 1.0];
        timer_bound = f64::INFINITY;
        ctx.time = 1.0;
        ctx.timestep = 1.0;
        assert_eq!(
            unsafe { rspice_timer_state_native(one_shot.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "zero-period timer must fire once"
        );
        assert!(timer_bound.is_infinite());

        let disabled = [1.0, 0.5, 0.0, 0.0];
        timer_bound = f64::INFINITY;
        assert_eq!(
            unsafe { rspice_timer_state_native(disabled.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits(),
            "disabled timer must not fire"
        );
        assert!(timer_bound.is_infinite());
    }

    #[test]
    fn timer_native_helper_is_inactive_outside_transient() {
        let timer = [0.0, 0.0, 0.0, 1.0];
        let mut timer_bound = f64::INFINITY;
        let mut ctx = empty_eval_context();
        ctx.timer_event_bound = &mut timer_bound;

        for analysis_type in [0, 1, 3, 4] {
            ctx.analysis_type = analysis_type;
            timer_bound = f64::INFINITY;
            assert_eq!(
                unsafe { rspice_timer_state_native(timer.as_ptr(), &ctx, 0) }.to_bits(),
                0.0_f64.to_bits(),
                "analysis {analysis_type}"
            );
            assert!(timer_bound.is_infinite());
            assert!(ctx.take_runtime_error().is_none());
        }

        ctx.analysis_type = 2;
        assert_eq!(
            unsafe { rspice_timer_state_native(timer.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits()
        );
    }

    #[test]
    fn zi_native_helper_publishes_exact_absolute_event_time() {
        let operands = [1.0, 1.0, 1.0, 0.9, 2.0, 0.0];
        let layout = crate::codegen::ZiRuntimeLayout::unit_coefficients(0);
        let descriptor = layout.native_descriptor().expect("unit Zi descriptor");
        let mut filters =
            [
                crate::zfilter::ZiFilter::new_with_timing(vec![1.0], vec![1.0], 1.0, 0.9)
                    .expect("valid Zi timing"),
            ];
        let mut timer_bound = f64::INFINITY;
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.time = 0.2;
        ctx.zi_filters = filters.as_mut_ptr();
        ctx.zi_filters_len = filters.len();
        ctx.timer_event_bound = &mut timer_bound;

        let recomposed = ctx.time + (0.9 - ctx.time);
        assert_ne!(
            recomposed.to_bits(),
            0.9_f64.to_bits(),
            "fixture must expose subtraction/addition drift"
        );
        assert_eq!(
            unsafe { rspice_zi_step_native(operands.as_ptr(), &ctx, descriptor) }.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(timer_bound.to_bits(), 0.9_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn transition_native_helper_records_runtime_error_for_invalid_pointers() {
        let ctx = empty_eval_context();
        ctx.clear_runtime_error();

        let missing_operands = unsafe { rspice_transition_state_native(std::ptr::null(), &ctx, 0) };

        assert_eq!(missing_operands.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("invalid native transition operands must record an error");
        assert!(
            error.contains("transition") && error.contains("operands"),
            "error must identify the invalid transition operands, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn transition_native_helper_passes_input_through_in_small_signal_analysis() {
        let operands = [1.25, 0.2, 0.4, 0.4];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 1;
        ctx.clear_runtime_error();

        let value = unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 7) };

        assert_eq!(value.to_bits(), 1.25_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn transition_native_helper_hard_fails_missing_transient_storage() {
        let operands = [1.0, 0.2, 0.4, 0.4];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.clear_runtime_error();

        let value = unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing transition storage must hard-fail");
        assert!(
            error.contains("transition") && error.contains("filter storage"),
            "error must identify missing transition storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn transition_native_helper_uses_vm_transition_filter_state() {
        let operands = [1.0, 0.2, 0.4, 0.4];
        let mut filters = [TransitionFilter::default()];
        filters[0]
            .eval_operating_point(0.0, 0.0, 0.2, 0.4, 0.4)
            .unwrap();
        filters[0].promote_operating_point_candidate();
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.transition_filters = filters.as_mut_ptr();
        ctx.transition_filters_len = filters.len();
        ctx.clear_runtime_error();

        ctx.time = 1.0;
        assert_eq!(
            unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );
        filters[0].commit();

        ctx.time = 1.4;
        let mid = unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((mid - 0.5).abs() < 1.0e-12, "mid transition: {mid}");
        filters[0].commit();

        ctx.time = 1.6;
        let done = unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((done - 1.0).abs() < 1.0e-12, "done transition: {done}");
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn transition_native_helper_rejects_invalid_timing_without_clamping() {
        let operands = [1.0, -0.2, 0.4, 0.4];
        let mut filters = [TransitionFilter::default()];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.transition_filters = filters.as_mut_ptr();
        ctx.transition_filters_len = filters.len();
        ctx.clear_runtime_error();

        assert_eq!(
            unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) },
            0.0
        );
        let error = ctx
            .take_runtime_error()
            .expect("negative transition timing must set the native error channel");
        assert!(
            error.contains("delay") && error.contains("non-negative"),
            "{error}"
        );
    }

    #[test]
    fn transition_native_derivative_is_branch_exact_and_read_only() {
        let mut filters = [TransitionFilter::default()];
        filters[0]
            .eval_operating_point(0.0, 0.0, 0.0, 1.0, 1.0)
            .unwrap();
        filters[0].promote_operating_point_candidate();
        let accepted = filters[0].checkpoint();
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.time = 1.0;
        ctx.transition_filters = filters.as_mut_ptr();
        ctx.transition_filters_len = filters.len();

        let delayed = [1.0, 3.0, 1.0, 2.0, 2.0];
        assert_eq!(
            unsafe { rspice_transition_derivative_native(delayed.as_ptr(), &ctx, 0) },
            0.0
        );
        let ramp = [1.0, 3.0, 0.0, 2.0, 2.0];
        assert_eq!(
            unsafe { rspice_transition_derivative_native(ramp.as_ptr(), &ctx, 0) },
            0.0
        );
        let instantaneous = [1.0, 3.0, 0.0, 0.0, 0.0];
        assert_eq!(
            unsafe { rspice_transition_derivative_native(instantaneous.as_ptr(), &ctx, 0) },
            3.0
        );
        assert_eq!(filters[0].checkpoint(), accepted);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn transition_native_derivative_is_unity_in_small_signal_analysis() {
        let operands = [1.0, 3.0, 5.0, 2.0, 2.0];
        let mut ctx = empty_eval_context();
        for analysis_type in [0, 1, 3, 4] {
            ctx.analysis_type = analysis_type;
            ctx.clear_runtime_error();
            assert_eq!(
                unsafe { rspice_transition_derivative_native(operands.as_ptr(), &ctx, 0) },
                3.0,
                "analysis {analysis_type}"
            );
            assert!(ctx.take_runtime_error().is_none());
        }
    }

    #[test]
    fn slew_native_helper_records_runtime_error_for_invalid_pointers() {
        let ctx = empty_eval_context();
        ctx.clear_runtime_error();

        let missing_operands = unsafe { rspice_slew_state_native(std::ptr::null(), &ctx, 0) };

        assert_eq!(missing_operands.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("invalid native slew operands must record an error");
        assert!(
            error.contains("slew") && error.contains("operands"),
            "error must identify the invalid slew operands, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn slew_native_helper_is_read_only_in_ac_and_noise() {
        let operands = [1.25, 2.0, -2.0];
        for analysis_type in [1, 3] {
            let mut filters = [SlewFilter::default()];
            let mut ctx = empty_eval_context();
            ctx.analysis_type = analysis_type;
            ctx.slew_filters = filters.as_mut_ptr();
            ctx.slew_filters_len = filters.len();
            ctx.clear_runtime_error();

            let value = unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) };

            assert_eq!(value.to_bits(), 1.25_f64.to_bits());
            assert!(filters[0].validate_checkpoint_ready().is_ok());
            assert!(!filters[0].checkpoint().initialized);
            assert!(ctx.take_runtime_error().is_none());
        }
    }

    #[test]
    fn slew_native_helper_hard_fails_missing_transient_storage() {
        let operands = [1.0, 2.0, -2.0];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.clear_runtime_error();

        let value = unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing slew storage must hard-fail");
        assert!(
            error.contains("slew") && error.contains("filter storage"),
            "error must identify missing slew storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn slew_native_helper_uses_vm_slew_filter_state() {
        let mut operands = [0.0, 2.0, -2.0];
        let mut filters = [SlewFilter::default()];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.slew_filters = filters.as_mut_ptr();
        ctx.slew_filters_len = filters.len();
        ctx.clear_runtime_error();

        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) },
            0.0
        );
        filters[0].commit();

        operands[0] = 10.0;
        ctx.time = 0.5;
        let mid = unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((mid - 1.0).abs() < 1.0e-12, "mid slew: {mid}");
        filters[0].commit();

        ctx.time = 1.0;
        let done = unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((done - 2.0).abs() < 1.0e-12, "done slew: {done}");
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn absdelay_native_helper_records_runtime_error_for_invalid_pointers() {
        let ctx = empty_eval_context();
        ctx.clear_runtime_error();

        let missing_operands = unsafe { rspice_absdelay_state_native(std::ptr::null(), &ctx, 0) };

        assert_eq!(missing_operands.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("invalid native absdelay operands must record an error");
        assert!(
            error.contains("absdelay") && error.contains("operands"),
            "error must identify the invalid absdelay operands, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn absdelay_native_helper_passes_input_through_outside_transient() {
        let mut operands = [1.25, 0.5];
        let mut buffers = [DelayBuffer::default()];
        let mut ctx = empty_eval_context();
        ctx.delay_buffers = buffers.as_mut_ptr();
        ctx.delay_buffers_len = buffers.len();
        ctx.clear_runtime_error();

        let value = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 1.25_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());

        buffers[0]
            .commit()
            .expect("accept native fixed-delay definition");
        operands[1] = f64::NAN;
        for analysis_type in [0, 1, 3, 4] {
            ctx.analysis_type = analysis_type;
            let value = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) };
            assert_eq!(
                value.to_bits(),
                1.25_f64.to_bits(),
                "analysis {analysis_type}"
            );
            assert!(ctx.take_runtime_error().is_none());

            let derivative_operands = [1.25, 3.0, f64::NAN, f64::NAN];
            let derivative =
                unsafe { rspice_absdelay_derivative_native(derivative_operands.as_ptr(), &ctx, 0) };
            assert_eq!(derivative.to_bits(), 3.0_f64.to_bits());
            assert!(ctx.take_runtime_error().is_none());
        }
    }

    #[test]
    fn absdelay_native_helper_hard_fails_missing_transient_storage() {
        let operands = [1.0, 0.5];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.clear_runtime_error();

        let value = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing absdelay storage must hard-fail");
        assert!(
            error.contains("absdelay") && error.contains("delay-buffer storage"),
            "error must identify missing absdelay storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn absdelay_native_helper_uses_vm_delay_buffer_state() {
        let mut operands = [0.0, 0.5];
        let mut buffers = [DelayBuffer::default()];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.delay_buffers = buffers.as_mut_ptr();
        ctx.delay_buffers_len = buffers.len();
        ctx.clear_runtime_error();

        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );
        buffers[0].commit().unwrap();

        ctx.time = 0.5;
        operands[0] = 1.0;
        assert_eq!(
            unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );
        buffers[0].commit().unwrap();

        ctx.time = 1.0;
        operands[0] = 3.0;
        let delayed = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((delayed - 1.0).abs() < 1.0e-12, "delayed: {delayed}");
        buffers[0].commit().unwrap();

        ctx.time = 1.25;
        operands[0] = 5.0;
        let interpolated = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) };
        assert!(
            (interpolated - 2.0).abs() < 1.0e-12,
            "interpolated delay: {interpolated}"
        );
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn absdelay_native_max_and_derivative_helpers_use_exact_coefficients() {
        let mut buffers = [DelayBuffer::default()];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.delay_buffers = buffers.as_mut_ptr();
        ctx.delay_buffers_len = buffers.len();

        for (time, input) in [(0.0, 0.0), (1.0, 10.0)] {
            ctx.time = time;
            let operands = [input, 0.5, 2.0];
            unsafe { rspice_absdelay_state_max_native(operands.as_ptr(), &ctx, 0) };
            buffers[0].commit().expect("accept absdelay sample");
        }

        ctx.time = 2.0;
        let operands = [20.0, 2.0, 0.5, 3.0, 2.0];
        let derivative =
            unsafe { rspice_absdelay_derivative_max_native(operands.as_ptr(), &ctx, 0) };
        assert_eq!(derivative, -29.0);
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn cross_native_helper_records_runtime_error_for_invalid_pointers() {
        let ctx = empty_eval_context();
        ctx.clear_runtime_error();

        let missing_operands = unsafe { rspice_cross_state_native(std::ptr::null(), &ctx, 0) };

        assert_eq!(missing_operands.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("invalid native cross operands must record an error");
        assert!(
            error.contains("cross") && error.contains("operands"),
            "error must identify the invalid cross operands, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn cross_native_helper_hard_fails_missing_detector_storage() {
        let operands = [1.0, 1.0, 0.0, 0.0, 1.0];
        let ctx = empty_eval_context();
        ctx.clear_runtime_error();

        let value = unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("missing cross storage must hard-fail");
        assert!(
            error.contains("cross") && error.contains("detector storage"),
            "error must identify missing cross storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn cross_native_helper_uses_vm_detector_state() {
        let mut operands = [-1.0, 1.0, 0.0, 0.0, 1.0];
        let mut detectors = [CrossDetector::default()];
        let mut ctx = empty_eval_context();
        ctx.cross_detectors = detectors.as_mut_ptr();
        ctx.cross_detectors_len = detectors.len();
        ctx.clear_runtime_error();

        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits(),
            "non-transient cross evaluation reports zero but still records history"
        );
        detectors[0].commit();

        ctx.analysis_type = 2;
        ctx.time = 0.5;
        operands[0] = 1.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "rising edge should fire after non-transient history update"
        );
        detectors[0].commit();

        ctx.time = 1.0;
        operands[0] = 2.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits(),
            "steady positive value should not fire repeatedly"
        );

        detectors[0] = CrossDetector::default();
        std::hint::black_box(&detectors);
        operands = [1.0, -1.0, 0.0, 0.0, 1.0];
        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );
        detectors[0].commit();

        ctx.time = 0.5;
        operands[0] = -1.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "falling edge should obey negative direction"
        );
        assert!(ctx.take_runtime_error().is_none());

        detectors[0] = CrossDetector::default();
        operands = [-1.0, 2.0, 0.0, 0.0, 1.0];
        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );
        detectors[0].commit();
        operands[0] = 1.0;
        ctx.time = 1.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits(),
            "invalid integral direction must not alias a rising event"
        );
        assert!(ctx.take_runtime_error().is_none());

        operands[1] = 0.6;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );
        let error = ctx
            .take_runtime_error()
            .expect("non-integer native direction must fail closed");
        assert!(error.contains("must evaluate to an integer"), "{error}");
    }

    #[test]
    fn above_native_helper_uses_stateful_initial_and_rising_events() {
        let mut operands = [1.0, 0.0, 0.0, 1.0];
        let mut detectors = [CrossDetector::default()];
        let mut ctx = empty_eval_context();
        ctx.cross_detectors = detectors.as_mut_ptr();
        ctx.cross_detectors_len = detectors.len();

        assert_eq!(
            unsafe { rspice_above_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "initial positive value must trigger above"
        );
        detectors[0].commit();

        ctx.time = 1.0;
        assert_eq!(
            unsafe { rspice_above_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits(),
            "steady positive values must not retrigger above"
        );
        operands[0] = -1.0;
        assert_eq!(
            unsafe { rspice_above_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );
        detectors[0].commit();

        ctx.time = 2.0;
        operands[0] = 1.0;
        assert_eq!(
            unsafe { rspice_above_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "subsequent rising crossings must trigger above"
        );
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn last_crossing_native_helper_interpolates_accepted_history() {
        let mut operands = [-1.0, 1.0];
        let mut detectors = [CrossDetector::default()];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.cross_detectors = detectors.as_mut_ptr();
        ctx.cross_detectors_len = detectors.len();

        assert_eq!(
            unsafe { rspice_last_crossing_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            (-1.0_f64).to_bits()
        );
        detectors[0].commit();

        ctx.time = 2.0;
        operands[0] = 3.0;
        assert_eq!(
            unsafe { rspice_last_crossing_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.5_f64.to_bits()
        );
        assert_eq!(
            unsafe { rspice_last_crossing_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.5_f64.to_bits(),
            "native Newton reevaluation must be idempotent"
        );
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn dynamic_variable_slot_resolver_returns_rounded_slot_without_side_effects() {
        let mut values = [2.0, 4.0, 8.0];

        let slot = unsafe {
            rspice_dynamic_variable_slot_native(2.49, values.as_mut_ptr(), values.len(), 1)
        };

        assert_eq!(slot, unsafe { values.as_mut_ptr().add(1) });

        let slot = unsafe {
            rspice_dynamic_variable_slot_native(4.0, values.as_mut_ptr(), values.len(), 1)
        };

        assert!(slot.is_null());
    }

    #[test]
    fn dynamic_variable_slot_helper_rejects_unsafe_indexes_without_aliasing_storage() {
        for (name, raw_index, lower) in [
            ("nan", f64::NAN, 0),
            ("infinity", f64::INFINITY, 0),
            ("huge finite", 1.0e300, i64::MAX),
        ] {
            let mut values = [2.0];

            let slot = unsafe {
                rspice_dynamic_variable_slot_native(raw_index, values.as_mut_ptr(), 1, lower)
            };

            assert!(slot.is_null(), "{name}");
            assert_eq!(values[0].to_bits(), 2.0_f64.to_bits(), "{name}");
        }
    }

    #[test]
    fn dynamic_variable_error_helper_reports_into_its_dispatch_context() {
        let ctx = empty_eval_context();
        ctx.clear_runtime_error();

        let value = rspice_native_dynamic_variable_error(4.0, &ctx, 3, 1);

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("dynamic variable failure must publish to its dispatch");
        assert!(
            error.contains("array index 4 outside declared bounds [1:3]"),
            "error must preserve array bounds diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn native_runtime_errors_are_isolated_between_contexts_on_one_thread() {
        let failed_ctx = empty_eval_context();
        let clean_ctx = empty_eval_context();

        let value = rspice_native_dynamic_variable_error(4.0, &failed_ctx, 3, 1);

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        assert!(
            clean_ctx.take_runtime_error().is_none(),
            "a failure must not leak into another evaluation context"
        );
        let error = failed_ctx
            .take_runtime_error()
            .expect("the failing context must retain its own diagnostic");
        assert!(
            error.contains("array index 4 outside declared bounds [1:3]"),
            "unexpected failing-context diagnostic: {error}"
        );
        assert!(
            clean_ctx.take_runtime_error().is_none(),
            "draining the failing context must not alter a clean context"
        );
    }

    #[test]
    fn native_runtime_errors_are_isolated_across_parallel_dispatches() {
        let (dynamic_error, laplace_error, clean_error) = std::thread::scope(|scope| {
            let dynamic = scope.spawn(|| {
                let ctx = empty_eval_context();
                rspice_native_dynamic_variable_error(8.0, &ctx, 8, 0);
                ctx.take_runtime_error()
                    .expect("dynamic-index dispatch must retain its diagnostic")
            });
            let laplace = scope.spawn(|| {
                let ctx = empty_eval_context();
                unsafe { rspice_laplace_step_native(1.0, &ctx, 0) };
                ctx.take_runtime_error()
                    .expect("Laplace dispatch must retain its diagnostic")
            });
            let clean = scope.spawn(|| {
                let ctx = empty_eval_context();
                unsafe { rspice_limiter_previous_native(2.5, &ctx, usize::MAX) };
                ctx.take_runtime_error()
            });
            (
                dynamic.join().expect("dynamic-index dispatch joins"),
                laplace.join().expect("Laplace dispatch joins"),
                clean.join().expect("clean dispatch joins"),
            )
        });

        assert!(
            dynamic_error.contains("array index 8 outside declared bounds [0:7]"),
            "unexpected dynamic-index diagnostic: {dynamic_error}"
        );
        assert!(
            laplace_error.contains("Laplace") && laplace_error.contains("filter storage"),
            "unexpected Laplace diagnostic: {laplace_error}"
        );
        assert!(
            clean_error.is_none(),
            "parallel failures must not contaminate a successful dispatch"
        );
    }

    #[test]
    fn native_runtime_context_recovers_after_error_is_drained() {
        let ctx = empty_eval_context();
        rspice_native_dynamic_variable_error(4.0, &ctx, 3, 1);
        assert!(
            ctx.take_runtime_error().is_some(),
            "fixture must begin with a native runtime failure"
        );

        let value = unsafe { rspice_limiter_previous_native(2.5, &ctx, usize::MAX) };

        assert_eq!(value.to_bits(), 2.5_f64.to_bits());
        assert!(
            ctx.take_runtime_error().is_none(),
            "a successful next dispatch must remain clean after the prior error is drained"
        );
    }

    #[test]
    fn native_integer_helper_matches_signed_32_bit_logical_shift_contract() {
        let ctx = empty_eval_context();
        let operands = [-16.0, 2.0];
        let descriptor = integer_binary_descriptor(IntegerBinaryOperation::Shr);

        let value = unsafe { rspice_integer_operation_native(operands.as_ptr(), &ctx, descriptor) };

        assert_eq!(value.to_bits(), 1_073_741_820.0_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());

        let shifted_out = [-1.0, 32.0];
        let value =
            unsafe { rspice_integer_operation_native(shifted_out.as_ptr(), &ctx, descriptor) };
        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        assert!(ctx.take_runtime_error().is_none());
    }

    #[test]
    fn native_integer_helper_rounds_ties_and_reports_invalid_conversions() {
        let ctx = empty_eval_context();
        let half = [-1.5];
        let value = unsafe {
            rspice_integer_operation_native(half.as_ptr(), &ctx, INTEGER_CAST_DESCRIPTOR)
        };
        assert_eq!(value.to_bits(), (-2.0_f64).to_bits());
        assert!(ctx.take_runtime_error().is_none());

        let invalid = [f64::NAN];
        let value = unsafe {
            rspice_integer_operation_native(invalid.as_ptr(), &ctx, INTEGER_CAST_DESCRIPTOR)
        };
        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = ctx
            .take_runtime_error()
            .expect("invalid integer conversion must publish a native error");
        assert!(error.contains("requires a finite value"), "{error}");
        assert!(error.contains("no interpreter fallback"), "{error}");
    }

    fn empty_eval_context() -> EvalContext {
        EvalContext::empty_for_test()
    }
}
