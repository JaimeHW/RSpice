use std::cell::RefCell;

/// Evaluation context passed to JIT-compiled functions.
#[repr(C)]
pub struct EvalContext {
    /// Terminal voltages array
    pub voltages: *const f64,
    /// Internal node voltages
    pub internal_voltages: *const f64,
    /// Parameter values
    pub params: *const f64,
    /// Flattened terminal-pair branch current matrix (size = num_terminals * num_terminals)
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
}

thread_local! {
    static NATIVE_RUNTIME_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub(crate) fn clear_native_runtime_error() {
    NATIVE_RUNTIME_ERROR.with(|slot| {
        slot.borrow_mut().take();
    });
}

pub(crate) fn take_native_runtime_error() -> Option<String> {
    NATIVE_RUNTIME_ERROR.with(|slot| slot.borrow_mut().take())
}

fn set_native_runtime_error(message: impl Into<String>) {
    NATIVE_RUNTIME_ERROR.with(|slot| {
        let mut error = slot.borrow_mut();
        if error.is_none() {
            *error = Some(message.into());
        }
    });
}

#[unsafe(export_name = "rspice_native_loop_limit_error")]
pub extern "C" fn rspice_native_loop_limit_error() {
    set_native_runtime_error(
        "native runtime loop iteration limit exceeded; no interpreter fallback",
    );
}

#[unsafe(export_name = "rspice_native_integer_shift_count_error")]
pub extern "C" fn rspice_native_integer_shift_count_error() {
    set_native_runtime_error(
        "native integer shift count outside valid range [0:63]; no interpreter fallback",
    );
}

/// External helper function for table lookup interpolation.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers.
#[unsafe(export_name = "rspice_table_lookup")]
pub unsafe extern "C" fn rspice_table_lookup(
    tables_ptr: *const crate::codegen::LookupTable,
    tables_len: usize,
    table_id: usize,
    input: f64,
) -> f64 {
    if tables_ptr.is_null() || table_id >= tables_len {
        return 0.0;
    }

    let tables = unsafe { std::slice::from_raw_parts(tables_ptr, tables_len) };
    tables[table_id].interpolate(input)
}

/// External helper function for native x64 table lookup interpolation.
///
/// Argument order is chosen for x64 helper-call codegen: scalar input in XMM0,
/// followed by table metadata in integer argument registers.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers.
#[unsafe(export_name = "rspice_table_lookup_native")]
pub unsafe extern "C" fn rspice_table_lookup_native(
    input: f64,
    tables_ptr: *const crate::codegen::LookupTable,
    tables_len: usize,
    table_id: usize,
) -> f64 {
    if tables_ptr.is_null() || table_id >= tables_len {
        return 0.0;
    }

    let tables = unsafe { std::slice::from_raw_parts(tables_ptr, tables_len) };
    tables[table_id].interpolate(input)
}

/// External helper function for native x64 table-model derivatives.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers.
#[unsafe(export_name = "rspice_table_derivative_native")]
pub unsafe extern "C" fn rspice_table_derivative_native(
    input: f64,
    tables_ptr: *const crate::codegen::LookupTable,
    tables_len: usize,
    table_id: usize,
) -> f64 {
    if tables_ptr.is_null() || table_id >= tables_len {
        return 0.0;
    }

    let tables = unsafe { std::slice::from_raw_parts(tables_ptr, tables_len) };
    tables[table_id].derivative(input)
}

/// External helper function for $limit operation.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers.
#[unsafe(export_name = "rspice_limit")]
pub unsafe extern "C" fn rspice_limit(
    state_values: *mut f64,
    state_initialized: *mut u8,
    state_initialized_len: usize,
    state_idx: usize,
    new_value: f64,
    step_limit: f64,
) -> f64 {
    if state_values.is_null() || state_initialized.is_null() || state_idx >= state_initialized_len {
        return new_value;
    }

    let initialized = unsafe { *state_initialized.add(state_idx) != 0 };
    let limited = if initialized {
        let prev_value = unsafe { *state_values.add(state_idx) };
        let delta = new_value - prev_value;
        let limited_delta = delta.clamp(-step_limit, step_limit);
        prev_value + limited_delta
    } else {
        new_value
    };

    unsafe {
        *state_values.add(state_idx) = limited;
        *state_initialized.add(state_idx) = 1;
    }
    limited
}

/// External helper function for idtmod wrapping.
#[unsafe(export_name = "rspice_idtmod_wrap")]
pub extern "C" fn rspice_idtmod_wrap(raw: f64, modulus: f64, offset: f64) -> f64 {
    if modulus > 0.0 {
        let phase = (raw - offset).rem_euclid(modulus);
        offset + phase
    } else {
        raw
    }
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

/// External helper function for Verilog-A left shift.
#[unsafe(export_name = "rspice_shl")]
pub extern "C" fn rspice_shl(left: f64, right: f64) -> f64 {
    ((left as i64) << (right as i64)) as f64
}

/// External helper function for Verilog-A arithmetic right shift.
#[unsafe(export_name = "rspice_shr")]
pub extern "C" fn rspice_shr(left: f64, right: f64) -> f64 {
    ((left as i64) >> (right as i64)) as f64
}

/// External helper function for Verilog-A bitwise and.
#[unsafe(export_name = "rspice_bitand")]
pub extern "C" fn rspice_bitand(left: f64, right: f64) -> f64 {
    ((left as i64) & (right as i64)) as f64
}

/// External helper function for Verilog-A bitwise or.
#[unsafe(export_name = "rspice_bitor")]
pub extern "C" fn rspice_bitor(left: f64, right: f64) -> f64 {
    ((left as i64) | (right as i64)) as f64
}

/// External helper function for Verilog-A bitwise xor.
#[unsafe(export_name = "rspice_bitxor")]
pub extern "C" fn rspice_bitxor(left: f64, right: f64) -> f64 {
    ((left as i64) ^ (right as i64)) as f64
}

/// External helper function for Laplace state-space filter step.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers.
#[unsafe(export_name = "rspice_laplace_step")]
pub unsafe extern "C" fn rspice_laplace_step(
    filters_ptr: *mut crate::laplace::StateSpaceFilter,
    filters_len: usize,
    filter_id: usize,
    input: f64,
    timestep: f64,
) -> f64 {
    if filters_ptr.is_null() || filter_id >= filters_len {
        return input;
    }

    let filters = unsafe { std::slice::from_raw_parts_mut(filters_ptr, filters_len) };

    if timestep <= 0.0 {
        return filters[filter_id].dc_output(input);
    }

    filters[filter_id].step(input, timestep)
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
        set_native_runtime_error(
            "native Laplace helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }

    let ctx = unsafe { &*ctx };
    if ctx.laplace_filters.is_null() {
        set_native_runtime_error(format!(
            "native Laplace helper missing filter storage for filter {filter_id}; no interpreter fallback"
        ));
        return 0.0;
    }
    if filter_id >= ctx.laplace_filters_len {
        set_native_runtime_error(format!(
            "native Laplace helper filter {filter_id} outside filter table length {}; no interpreter fallback",
            ctx.laplace_filters_len
        ));
        return 0.0;
    }

    let filters =
        unsafe { std::slice::from_raw_parts_mut(ctx.laplace_filters, ctx.laplace_filters_len) };
    if ctx.analysis_type == 2 {
        filters[filter_id].step(input, ctx.timestep)
    } else {
        filters[filter_id].dc_output(input)
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
    input: f64,
    ctx: *const EvalContext,
    filter_id: usize,
) -> f64 {
    if ctx.is_null() {
        set_native_runtime_error("native zi helper missing EvalContext; no interpreter fallback");
        return 0.0;
    }

    let ctx = unsafe { &*ctx };
    if ctx.zi_filters.is_null() {
        set_native_runtime_error(format!(
            "native zi helper missing filter storage for filter {filter_id}; no interpreter fallback"
        ));
        return 0.0;
    }
    if filter_id >= ctx.zi_filters_len {
        set_native_runtime_error(format!(
            "native zi helper filter {filter_id} outside filter table length {}; no interpreter fallback",
            ctx.zi_filters_len
        ));
        return 0.0;
    }

    let filters = unsafe { std::slice::from_raw_parts_mut(ctx.zi_filters, ctx.zi_filters_len) };
    filters[filter_id].eval(input, ctx.time, ctx.analysis_type == 2)
}

/// External helper function for native x64 periodic timer events.
///
/// Argument order matches x64 helper-call codegen: scalar start/period in XMM0
/// and XMM1, followed by the context pointer in the platform's integer
/// argument register for the third C ABI argument.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer. Invalid pointers are reported through the native runtime error
/// channel; native mode never dispatches the bytecode interpreter for recovery.
#[unsafe(export_name = "rspice_timer_state_native")]
pub unsafe extern "C" fn rspice_timer_state_native(
    start_time: f64,
    period: f64,
    ctx: *const EvalContext,
) -> f64 {
    if ctx.is_null() {
        set_native_runtime_error(
            "native timer helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }

    let ctx = unsafe { &*ctx };
    let current_time = ctx.time;
    if current_time >= start_time && period > 0.0 {
        let elapsed = current_time - start_time;
        let cycles = (elapsed / period).floor();
        let next_fire = start_time + cycles * period;
        let tolerance = ctx.timestep.max(1e-15);
        if (current_time - next_fire).abs() < tolerance {
            1.0
        } else {
            0.0
        }
    } else {
        0.0
    }
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
    if operands.is_null() {
        set_native_runtime_error(
            "native transition helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }
    if ctx.is_null() {
        set_native_runtime_error(
            "native transition helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 4) };
    let input = operands[0];
    let delay = operands[1];
    let rise_time = operands[2];
    let fall_time = operands[3];
    let ctx = unsafe { &*ctx };

    if ctx.analysis_type != 2 {
        return input;
    }

    if ctx.transition_filters.is_null() {
        set_native_runtime_error(format!(
            "native transition helper missing filter storage for filter {filter_id}; no interpreter fallback"
        ));
        return 0.0;
    }
    if filter_id >= ctx.transition_filters_len {
        set_native_runtime_error(format!(
            "native transition helper filter {filter_id} outside filter table length {}; no interpreter fallback",
            ctx.transition_filters_len
        ));
        return 0.0;
    }

    let filters = unsafe {
        std::slice::from_raw_parts_mut(ctx.transition_filters, ctx.transition_filters_len)
    };
    filters[filter_id].update(
        input,
        ctx.time,
        delay.max(0.0),
        rise_time.max(0.0),
        fall_time.max(0.0),
    )
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
    if operands.is_null() {
        set_native_runtime_error("native slew helper missing operands; no interpreter fallback");
        return 0.0;
    }
    if ctx.is_null() {
        set_native_runtime_error("native slew helper missing EvalContext; no interpreter fallback");
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 3) };
    let input = operands[0];
    let max_pos_slew = operands[1];
    let max_neg_slew = operands[2];
    let ctx = unsafe { &*ctx };

    if ctx.analysis_type != 2 {
        return input;
    }

    if ctx.slew_filters.is_null() {
        set_native_runtime_error(format!(
            "native slew helper missing filter storage for filter {filter_id}; no interpreter fallback"
        ));
        return 0.0;
    }
    if filter_id >= ctx.slew_filters_len {
        set_native_runtime_error(format!(
            "native slew helper filter {filter_id} outside filter table length {}; no interpreter fallback",
            ctx.slew_filters_len
        ));
        return 0.0;
    }

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

    let filters = unsafe { std::slice::from_raw_parts_mut(ctx.slew_filters, ctx.slew_filters_len) };
    filters[filter_id].update(input, ctx.time, max_pos, max_neg)
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
    if operands.is_null() {
        set_native_runtime_error(
            "native absdelay helper missing operands; no interpreter fallback",
        );
        return 0.0;
    }
    if ctx.is_null() {
        set_native_runtime_error(
            "native absdelay helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 2) };
    let input = operands[0];
    let delay_time = operands[1];
    let ctx = unsafe { &*ctx };

    if ctx.analysis_type != 2 {
        return input;
    }

    if ctx.delay_buffers.is_null() {
        set_native_runtime_error(format!(
            "native absdelay helper missing delay-buffer storage for buffer {buffer_id}; no interpreter fallback"
        ));
        return 0.0;
    }
    if buffer_id >= ctx.delay_buffers_len {
        set_native_runtime_error(format!(
            "native absdelay helper buffer {buffer_id} outside delay-buffer table length {}; no interpreter fallback",
            ctx.delay_buffers_len
        ));
        return 0.0;
    }

    let buffers =
        unsafe { std::slice::from_raw_parts_mut(ctx.delay_buffers, ctx.delay_buffers_len) };
    let buffer = &mut buffers[buffer_id];
    buffer.record(ctx.time, input);

    if delay_time <= 0.0 {
        input
    } else {
        buffer.get_delayed(ctx.time, delay_time)
    }
}

/// External helper function for native x64 threshold-crossing detectors.
///
/// `operands` points to two contiguous f64 values emitted by the JIT in VM
/// stack order: expression value and direction. Native mode requires
/// preallocated detector storage and hard-fails through the native runtime
/// error channel when it is missing.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid EvalContext
/// pointer and a valid two-element operand slice. Invalid pointers are reported
/// through the native runtime error channel.
#[unsafe(export_name = "rspice_cross_state_native")]
pub unsafe extern "C" fn rspice_cross_state_native(
    operands: *const f64,
    ctx: *const EvalContext,
    detector_id: usize,
) -> f64 {
    if operands.is_null() {
        set_native_runtime_error("native cross helper missing operands; no interpreter fallback");
        return 0.0;
    }
    if ctx.is_null() {
        set_native_runtime_error(
            "native cross helper missing EvalContext; no interpreter fallback",
        );
        return 0.0;
    }

    let operands = unsafe { std::slice::from_raw_parts(operands, 2) };
    let value = operands[0];
    let direction_raw = operands[1];
    let direction = if direction_raw > 0.5 {
        1
    } else if direction_raw < -0.5 {
        -1
    } else {
        0
    };
    let ctx = unsafe { &*ctx };

    if ctx.cross_detectors.is_null() {
        set_native_runtime_error(format!(
            "native cross helper missing detector storage for detector {detector_id}; no interpreter fallback"
        ));
        return 0.0;
    }
    if detector_id >= ctx.cross_detectors_len {
        set_native_runtime_error(format!(
            "native cross helper detector {detector_id} outside detector table length {}; no interpreter fallback",
            ctx.cross_detectors_len
        ));
        return 0.0;
    }

    let detectors =
        unsafe { std::slice::from_raw_parts_mut(ctx.cross_detectors, ctx.cross_detectors_len) };
    let crossed = detectors[detector_id].update(value, ctx.time, direction);
    if ctx.analysis_type == 2 { crossed } else { 0.0 }
}

/// External helper function for native x64 runtime-indexed variable reads.
///
/// `base_ptr` points at the first element of the array variable run. The helper
/// preserves VM `array_slot` semantics: round the floating index, apply the
/// declared lower bound, and hard-fail through the native runtime error channel
/// when the index is out of range.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid pointer to the
/// first array element and a compile-time-validated element count.
#[unsafe(export_name = "rspice_dynamic_variable_load_native")]
pub unsafe extern "C" fn rspice_dynamic_variable_load_native(
    raw_index: f64,
    base_ptr: *const f64,
    len: usize,
    lower: i64,
) -> f64 {
    if base_ptr.is_null() {
        set_native_runtime_error(
            "native dynamic array read missing variable storage; no interpreter fallback",
        );
        return 0.0;
    }
    if len == 0 {
        set_native_runtime_error(
            "native dynamic array read has zero-length storage; no interpreter fallback",
        );
        return 0.0;
    }
    let Ok(len_i64) = i64::try_from(len) else {
        set_native_runtime_error(
            "native dynamic array read length exceeds native bounds range; no interpreter fallback",
        );
        return 0.0;
    };

    let Some(offset) =
        dynamic_variable_offset(raw_index, len_i64, lower, "native dynamic array read")
    else {
        return 0.0;
    };

    unsafe { *base_ptr.add(offset) }
}

/// External helper function for native x64 runtime-indexed variable writes.
///
/// Returns a pointer to the selected array slot. A null return means the helper
/// recorded a native runtime error and the JIT caller must not store.
///
/// # Safety
/// This function is called from JIT-compiled code with a valid pointer to the
/// first array element and a compile-time-validated element count.
#[unsafe(export_name = "rspice_dynamic_variable_slot_native")]
pub unsafe extern "C" fn rspice_dynamic_variable_slot_native(
    raw_index: f64,
    base_ptr: *mut f64,
    len: usize,
    lower: i64,
) -> *mut f64 {
    if base_ptr.is_null() {
        set_native_runtime_error(
            "native indexed assignment missing variable storage; no interpreter fallback",
        );
        return std::ptr::null_mut();
    }
    if len == 0 {
        set_native_runtime_error(
            "native indexed assignment has zero-length storage; no interpreter fallback",
        );
        return std::ptr::null_mut();
    }
    let Ok(len_i64) = i64::try_from(len) else {
        set_native_runtime_error(
            "native indexed assignment length exceeds native bounds range; no interpreter fallback",
        );
        return std::ptr::null_mut();
    };

    let Some(offset) =
        dynamic_variable_offset(raw_index, len_i64, lower, "native indexed assignment")
    else {
        return std::ptr::null_mut();
    };

    unsafe { base_ptr.add(offset) }
}

fn dynamic_variable_offset(
    raw_index: f64,
    len_i64: i64,
    lower: i64,
    context: &str,
) -> Option<usize> {
    let Some(index) = rounded_i64_without_saturation(raw_index) else {
        set_dynamic_variable_bounds_error(context, raw_index, lower, len_i64);
        return None;
    };

    let Some(offset) = index.checked_sub(lower) else {
        set_dynamic_variable_bounds_error(context, index, lower, len_i64);
        return None;
    };

    if offset < 0 || offset >= len_i64 {
        set_dynamic_variable_bounds_error(context, index, lower, len_i64);
        return None;
    }

    usize::try_from(offset).ok()
}

fn rounded_i64_without_saturation(value: f64) -> Option<i64> {
    const I64_MAX_EXCLUSIVE_AS_F64: f64 = 9_223_372_036_854_775_808.0;

    if !value.is_finite() {
        return None;
    }

    let rounded = value.round();
    if rounded < i64::MIN as f64 || rounded >= I64_MAX_EXCLUSIVE_AS_F64 {
        return None;
    }

    Some(rounded as i64)
}

fn set_dynamic_variable_bounds_error(
    context: &str,
    index: impl std::fmt::Display,
    lower: i64,
    len_i64: i64,
) {
    let upper = lower.saturating_add(len_i64).saturating_sub(1);
    set_native_runtime_error(format!(
        "{context}: array index {index} outside declared bounds [{lower}:{upper}]; no interpreter fallback"
    ));
}

/// External helper function for PushCurrent terminal-pair lookup.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers and lengths.
#[unsafe(export_name = "rspice_current_lookup")]
pub unsafe extern "C" fn rspice_current_lookup(
    branch_currents_ptr: *const f64,
    branch_currents_len: usize,
    _currents_ptr: *const f64,
    _currents_len: usize,
    num_terminals: usize,
    pos: usize,
    neg: usize,
) -> f64 {
    if !branch_currents_ptr.is_null() && pos < num_terminals && neg < num_terminals {
        let idx = pos.saturating_mul(num_terminals).saturating_add(neg);
        if idx < branch_currents_len {
            let value = unsafe { *branch_currents_ptr.add(idx) };
            if value.is_finite() {
                return value;
            }
        }
    }

    f64::NAN
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{
        EvalContext, clear_native_runtime_error, rspice_absdelay_state_native,
        rspice_cross_state_native, rspice_dynamic_variable_load_native,
        rspice_dynamic_variable_slot_native, rspice_laplace_step_native, rspice_slew_state_native,
        rspice_timer_state_native, rspice_transition_state_native, rspice_zi_step_native,
        take_native_runtime_error,
    };
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
        assert_eq!(offset_of!(EvalContext, branch_unknowns), 176);
        assert_eq!(offset_of!(EvalContext, analysis_type), 184);
        assert_eq!(offset_of!(EvalContext, multiplicity), 192);
        assert_eq!(offset_of!(EvalContext, zi_filters), 200);
        assert_eq!(offset_of!(EvalContext, zi_filters_len), 208);
        assert_eq!(offset_of!(EvalContext, transition_filters), 216);
        assert_eq!(offset_of!(EvalContext, transition_filters_len), 224);
        assert_eq!(offset_of!(EvalContext, slew_filters), 232);
        assert_eq!(offset_of!(EvalContext, slew_filters_len), 240);
        assert_eq!(offset_of!(EvalContext, delay_buffers), 248);
        assert_eq!(offset_of!(EvalContext, delay_buffers_len), 256);
        assert_eq!(offset_of!(EvalContext, cross_detectors), 264);
        assert_eq!(offset_of!(EvalContext, cross_detectors_len), 272);
        assert_eq!(size_of::<EvalContext>(), 280);
        assert_eq!(align_of::<EvalContext>(), 8);
    }

    #[test]
    fn laplace_native_helper_records_runtime_error_for_invalid_context() {
        clear_native_runtime_error();

        let value = unsafe { rspice_laplace_step_native(1.25, std::ptr::null(), 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error()
            .expect("invalid native Laplace context must record a runtime error");
        assert!(
            error.contains("Laplace") && error.contains("EvalContext"),
            "error must identify the invalid Laplace context, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
        assert!(
            take_native_runtime_error().is_none(),
            "runtime error retrieval must clear the thread-local slot"
        );
    }

    #[test]
    fn zi_native_helper_records_runtime_error_for_invalid_context() {
        clear_native_runtime_error();

        let value = unsafe { rspice_zi_step_native(1.25, std::ptr::null(), 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error =
            take_native_runtime_error().expect("invalid native zi context must record an error");
        assert!(
            error.contains("zi") && error.contains("EvalContext"),
            "error must identify the invalid zi context, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn timer_native_helper_matches_vm_tolerance_and_reports_invalid_context() {
        clear_native_runtime_error();

        let value = unsafe { rspice_timer_state_native(1.0, 0.5, std::ptr::null()) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error =
            take_native_runtime_error().expect("invalid native timer context must record an error");
        assert!(
            error.contains("timer") && error.contains("EvalContext"),
            "error must identify the invalid timer context, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

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
            time: 1.5,
            timestep: 0.01,
            state_prev: std::ptr::null(),
            state_values: std::ptr::null_mut(),
            state_initialized: std::ptr::null_mut(),
            state_initialized_len: 0,
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
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
        };

        assert_eq!(
            unsafe { rspice_timer_state_native(1.0, 0.5, &ctx) }.to_bits(),
            1.0_f64.to_bits()
        );

        ctx.timestep = 0.001;
        ctx.time = 1.0005;
        assert_eq!(
            unsafe { rspice_timer_state_native(1.0, 0.5, &ctx) }.to_bits(),
            1.0_f64.to_bits(),
            "timer should fire within timestep tolerance"
        );

        ctx.time = 1.0015;
        assert_eq!(
            unsafe { rspice_timer_state_native(1.0, 0.5, &ctx) }.to_bits(),
            0.0_f64.to_bits(),
            "timer should not fire outside timestep tolerance"
        );

        ctx.time = 0.75;
        assert_eq!(
            unsafe { rspice_timer_state_native(1.0, 0.5, &ctx) }.to_bits(),
            0.0_f64.to_bits(),
            "timer should not fire before start time"
        );

        ctx.time = 1.0;
        assert_eq!(
            unsafe { rspice_timer_state_native(1.0, 0.0, &ctx) }.to_bits(),
            0.0_f64.to_bits(),
            "non-positive timer period should never fire"
        );
    }

    #[test]
    fn transition_native_helper_records_runtime_error_for_invalid_pointers() {
        let operands = [1.0, 0.2, 0.4, 0.4];
        clear_native_runtime_error();

        let missing_operands =
            unsafe { rspice_transition_state_native(std::ptr::null(), std::ptr::null(), 0) };

        assert_eq!(missing_operands.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error()
            .expect("invalid native transition operands must record an error");
        assert!(
            error.contains("transition") && error.contains("operands"),
            "error must identify the invalid transition operands, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

        clear_native_runtime_error();
        let missing_ctx =
            unsafe { rspice_transition_state_native(operands.as_ptr(), std::ptr::null(), 0) };

        assert_eq!(missing_ctx.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error()
            .expect("invalid native transition context must record an error");
        assert!(
            error.contains("transition") && error.contains("EvalContext"),
            "error must identify the invalid transition context, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn transition_native_helper_passes_input_through_outside_transient() {
        let operands = [1.25, 0.2, 0.4, 0.4];
        let ctx = empty_eval_context();
        clear_native_runtime_error();

        let value = unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 7) };

        assert_eq!(value.to_bits(), 1.25_f64.to_bits());
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn transition_native_helper_hard_fails_missing_transient_storage() {
        let operands = [1.0, 0.2, 0.4, 0.4];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        clear_native_runtime_error();

        let value = unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing transition storage must hard-fail");
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
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.transition_filters = filters.as_mut_ptr();
        ctx.transition_filters_len = filters.len();
        clear_native_runtime_error();

        ctx.time = 1.0;
        assert_eq!(
            unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );

        ctx.time = 1.4;
        let mid = unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((mid - 0.5).abs() < 1.0e-12, "mid transition: {mid}");

        ctx.time = 1.6;
        let done = unsafe { rspice_transition_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((done - 1.0).abs() < 1.0e-12, "done transition: {done}");
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn slew_native_helper_records_runtime_error_for_invalid_pointers() {
        let operands = [1.0, 2.0, 2.0];
        clear_native_runtime_error();

        let missing_operands =
            unsafe { rspice_slew_state_native(std::ptr::null(), std::ptr::null(), 0) };

        assert_eq!(missing_operands.to_bits(), 0.0_f64.to_bits());
        let error =
            take_native_runtime_error().expect("invalid native slew operands must record an error");
        assert!(
            error.contains("slew") && error.contains("operands"),
            "error must identify the invalid slew operands, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

        clear_native_runtime_error();
        let missing_ctx =
            unsafe { rspice_slew_state_native(operands.as_ptr(), std::ptr::null(), 0) };

        assert_eq!(missing_ctx.to_bits(), 0.0_f64.to_bits());
        let error =
            take_native_runtime_error().expect("invalid native slew context must record an error");
        assert!(
            error.contains("slew") && error.contains("EvalContext"),
            "error must identify the invalid slew context, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn slew_native_helper_passes_input_through_outside_transient() {
        let operands = [1.25, 2.0, 2.0];
        let ctx = empty_eval_context();
        clear_native_runtime_error();

        let value = unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 7) };

        assert_eq!(value.to_bits(), 1.25_f64.to_bits());
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn slew_native_helper_hard_fails_missing_transient_storage() {
        let operands = [1.0, 2.0, 2.0];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        clear_native_runtime_error();

        let value = unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing slew storage must hard-fail");
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
        let operands = [10.0, 2.0, 2.0];
        let mut filters = [SlewFilter::default()];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        ctx.slew_filters = filters.as_mut_ptr();
        ctx.slew_filters_len = filters.len();
        clear_native_runtime_error();

        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );

        ctx.time = 0.5;
        let mid = unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((mid - 1.0).abs() < 1.0e-12, "mid slew: {mid}");

        ctx.time = 1.0;
        let done = unsafe { rspice_slew_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((done - 2.0).abs() < 1.0e-12, "done slew: {done}");
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn absdelay_native_helper_records_runtime_error_for_invalid_pointers() {
        let operands = [1.0, 0.5];
        clear_native_runtime_error();

        let missing_operands =
            unsafe { rspice_absdelay_state_native(std::ptr::null(), std::ptr::null(), 0) };

        assert_eq!(missing_operands.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error()
            .expect("invalid native absdelay operands must record an error");
        assert!(
            error.contains("absdelay") && error.contains("operands"),
            "error must identify the invalid absdelay operands, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

        clear_native_runtime_error();
        let missing_ctx =
            unsafe { rspice_absdelay_state_native(operands.as_ptr(), std::ptr::null(), 0) };

        assert_eq!(missing_ctx.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error()
            .expect("invalid native absdelay context must record an error");
        assert!(
            error.contains("absdelay") && error.contains("EvalContext"),
            "error must identify the invalid absdelay context, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn absdelay_native_helper_passes_input_through_outside_transient() {
        let operands = [1.25, 0.5];
        let ctx = empty_eval_context();
        clear_native_runtime_error();

        let value = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 7) };

        assert_eq!(value.to_bits(), 1.25_f64.to_bits());
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn absdelay_native_helper_hard_fails_missing_transient_storage() {
        let operands = [1.0, 0.5];
        let mut ctx = empty_eval_context();
        ctx.analysis_type = 2;
        clear_native_runtime_error();

        let value = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing absdelay storage must hard-fail");
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
        clear_native_runtime_error();

        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );

        ctx.time = 0.5;
        operands[0] = 1.0;
        assert_eq!(
            unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );

        ctx.time = 1.0;
        operands[0] = 3.0;
        let delayed = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) };
        assert!((delayed - 1.0).abs() < 1.0e-12, "delayed: {delayed}");

        ctx.time = 1.25;
        operands[0] = 5.0;
        let interpolated = unsafe { rspice_absdelay_state_native(operands.as_ptr(), &ctx, 0) };
        assert!(
            (interpolated - 2.0).abs() < 1.0e-12,
            "interpolated delay: {interpolated}"
        );
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn cross_native_helper_records_runtime_error_for_invalid_pointers() {
        let operands = [1.0, 1.0];
        clear_native_runtime_error();

        let missing_operands =
            unsafe { rspice_cross_state_native(std::ptr::null(), std::ptr::null(), 0) };

        assert_eq!(missing_operands.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error()
            .expect("invalid native cross operands must record an error");
        assert!(
            error.contains("cross") && error.contains("operands"),
            "error must identify the invalid cross operands, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

        clear_native_runtime_error();
        let missing_ctx =
            unsafe { rspice_cross_state_native(operands.as_ptr(), std::ptr::null(), 0) };

        assert_eq!(missing_ctx.to_bits(), 0.0_f64.to_bits());
        let error =
            take_native_runtime_error().expect("invalid native cross context must record an error");
        assert!(
            error.contains("cross") && error.contains("EvalContext"),
            "error must identify the invalid cross context, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn cross_native_helper_hard_fails_missing_detector_storage() {
        let operands = [1.0, 1.0];
        let ctx = empty_eval_context();
        clear_native_runtime_error();

        let value = unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) };

        assert_eq!(value.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing cross storage must hard-fail");
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
        let mut operands = [-1.0, 1.0];
        let mut detectors = [CrossDetector::default()];
        let mut ctx = empty_eval_context();
        ctx.cross_detectors = detectors.as_mut_ptr();
        ctx.cross_detectors_len = detectors.len();
        clear_native_runtime_error();

        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits(),
            "non-transient cross evaluation reports zero but still records history"
        );

        ctx.analysis_type = 2;
        ctx.time = 0.5;
        operands[0] = 1.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "rising edge should fire after non-transient history update"
        );

        ctx.time = 1.0;
        operands[0] = 2.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits(),
            "steady positive value should not fire repeatedly"
        );

        detectors[0] = CrossDetector::default();
        std::hint::black_box(&detectors);
        operands = [1.0, -1.0];
        ctx.time = 0.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            0.0_f64.to_bits()
        );

        ctx.time = 0.5;
        operands[0] = -1.0;
        assert_eq!(
            unsafe { rspice_cross_state_native(operands.as_ptr(), &ctx, 0) }.to_bits(),
            1.0_f64.to_bits(),
            "falling edge should obey negative direction"
        );
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn dynamic_variable_helper_loads_rounded_index_and_reports_bounds_errors() {
        let values = [2.0, 4.0, 8.0];
        clear_native_runtime_error();

        let loaded =
            unsafe { rspice_dynamic_variable_load_native(2.49, values.as_ptr(), values.len(), 1) };

        assert_eq!(loaded.to_bits(), 4.0_f64.to_bits());
        assert!(take_native_runtime_error().is_none());

        let out_of_range =
            unsafe { rspice_dynamic_variable_load_native(4.0, values.as_ptr(), values.len(), 1) };

        assert_eq!(out_of_range.to_bits(), 0.0_f64.to_bits());
        let error =
            take_native_runtime_error().expect("out-of-range native array read must hard-fail");
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
    fn dynamic_variable_helper_rejects_unsafe_indexes_without_aliasing_storage() {
        for (name, raw_index, lower) in [
            ("nan", f64::NAN, 0),
            ("infinity", f64::INFINITY, 0),
            ("huge finite", 1.0e300, i64::MAX),
        ] {
            let values = [2.0];
            clear_native_runtime_error();

            let loaded = unsafe {
                rspice_dynamic_variable_load_native(raw_index, values.as_ptr(), 1, lower)
            };

            assert_eq!(loaded.to_bits(), 0.0_f64.to_bits(), "{name}");
            let error = take_native_runtime_error()
                .unwrap_or_else(|| panic!("{name}: unsafe native array read must hard-fail"));
            assert!(
                error.contains("outside declared bounds"),
                "{name}: error must preserve array bounds diagnostic, got: {error}"
            );
            assert!(
                error.contains("no interpreter fallback"),
                "{name}: error must preserve the native hard-fail contract, got: {error}"
            );
        }
    }

    #[test]
    fn dynamic_variable_slot_helper_returns_rounded_slot_and_reports_bounds_errors() {
        let mut values = [2.0, 4.0, 8.0];
        clear_native_runtime_error();

        let slot = unsafe {
            rspice_dynamic_variable_slot_native(2.49, values.as_mut_ptr(), values.len(), 1)
        };

        assert_eq!(slot, unsafe { values.as_mut_ptr().add(1) });
        assert!(take_native_runtime_error().is_none());

        let slot = unsafe {
            rspice_dynamic_variable_slot_native(4.0, values.as_mut_ptr(), values.len(), 1)
        };

        assert!(slot.is_null());
        let error =
            take_native_runtime_error().expect("out-of-range native indexed write must hard-fail");
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
    fn dynamic_variable_slot_helper_rejects_unsafe_indexes_without_aliasing_storage() {
        for (name, raw_index, lower) in [
            ("nan", f64::NAN, 0),
            ("infinity", f64::INFINITY, 0),
            ("huge finite", 1.0e300, i64::MAX),
        ] {
            let mut values = [2.0];
            clear_native_runtime_error();

            let slot = unsafe {
                rspice_dynamic_variable_slot_native(raw_index, values.as_mut_ptr(), 1, lower)
            };

            assert!(slot.is_null(), "{name}");
            assert_eq!(values[0].to_bits(), 2.0_f64.to_bits(), "{name}");
            let error = take_native_runtime_error()
                .unwrap_or_else(|| panic!("{name}: unsafe native indexed write must hard-fail"));
            assert!(
                error.contains("outside declared bounds"),
                "{name}: error must preserve array bounds diagnostic, got: {error}"
            );
            assert!(
                error.contains("no interpreter fallback"),
                "{name}: error must preserve the native hard-fail contract, got: {error}"
            );
        }
    }

    fn empty_eval_context() -> EvalContext {
        EvalContext {
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
        }
    }
}
