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

    let index = raw_index.round() as i64;
    let offset = index - lower;
    if offset < 0 || offset >= len_i64 {
        let upper = lower.saturating_add(len_i64).saturating_sub(1);
        set_native_runtime_error(format!(
            "native dynamic array read: array index {index} outside declared bounds [{lower}:{upper}]; no interpreter fallback"
        ));
        return 0.0;
    }

    unsafe { *base_ptr.add(offset as usize) }
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
        EvalContext, clear_native_runtime_error, rspice_dynamic_variable_load_native,
        rspice_laplace_step_native, rspice_zi_step_native, take_native_runtime_error,
    };
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
        assert_eq!(size_of::<EvalContext>(), 216);
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
}
