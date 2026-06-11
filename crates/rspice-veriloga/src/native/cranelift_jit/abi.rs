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
    /// Sequentially evaluated branch currents for fallback semantics
    pub currents: *const f64,
    /// Length of `currents` buffer
    pub currents_len: usize,
    /// Number of terminals in the device
    pub num_terminals: usize,
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
}

/// External helper function for table lookup interpolation.
/// Called from JIT code to perform table interpolation.
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

    // Safety: caller guarantees valid pointer and bounds.
    let tables = unsafe { std::slice::from_raw_parts(tables_ptr, tables_len) };
    tables[table_id].interpolate(input)
}

/// External helper function for $limit operation.
/// Bounds value change per iteration for convergence control.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers.
#[unsafe(export_name = "rspice_limit")]
pub unsafe extern "C" fn rspice_limit(
    state_prev: *const f64,
    state_idx: usize,
    new_value: f64,
    step_limit: f64,
) -> f64 {
    let prev_value = if state_prev.is_null() {
        new_value // First iteration: use new value
    } else {
        // Safety: caller guarantees valid pointer.
        unsafe { *state_prev.add(state_idx) }
    };

    // If prev is 0 and this is effectively first iteration, use new_value.
    if prev_value == 0.0 && new_value != 0.0 {
        return new_value;
    }

    let delta = new_value - prev_value;
    let limited_delta = delta.clamp(-step_limit, step_limit);
    prev_value + limited_delta
}

/// External helper function for limited exponential.
/// Uses linear extrapolation beyond the limit to prevent overflow
/// while maintaining C0 and C1 continuity.
///
/// # Safety
/// This function is called from JIT-compiled code.
#[unsafe(export_name = "rspice_limexp")]
pub extern "C" fn rspice_limexp(x: f64) -> f64 {
    const LIMIT: f64 = 40.0; // exp(40) ~= 2.4e17
    if x > LIMIT {
        let exp_limit = LIMIT.exp();
        // Linear extrapolation: f(x) = f(limit) + f'(limit) * (x - limit).
        // For exp, f'(x) = exp(x), so f'(limit) = exp(limit).
        exp_limit * (1.0 + x - LIMIT)
    } else if x < -LIMIT {
        // For very negative values, return essentially 0.
        (-LIMIT).exp()
    } else {
        x.exp()
    }
}

/// External helper function for Laplace state-space filter step.
/// Called from JIT code to advance filter state using Backward Euler integration.
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
    // Null pointer or out-of-bounds check.
    if filters_ptr.is_null() || filter_id >= filters_len {
        // DC passthrough: return input unchanged for safety.
        return input;
    }

    // Safety: caller guarantees valid pointer and bounds.
    let filters = unsafe { std::slice::from_raw_parts_mut(filters_ptr, filters_len) };

    // Zero timestep means DC analysis - return DC gain * input.
    if timestep <= 0.0 {
        return filters[filter_id].dc_output(input);
    }

    // Step the filter forward in time.
    filters[filter_id].step(input, timestep)
}

/// External helper function for PushCurrent terminal-pair lookup.
///
/// # Safety
/// This function is called from JIT-compiled code with valid pointers and lengths.
#[unsafe(export_name = "rspice_current_lookup")]
pub unsafe extern "C" fn rspice_current_lookup(
    branch_currents_ptr: *const f64,
    branch_currents_len: usize,
    currents_ptr: *const f64,
    currents_len: usize,
    num_terminals: usize,
    pos: usize,
    neg: usize,
) -> f64 {
    if !branch_currents_ptr.is_null() && pos < num_terminals && neg < num_terminals {
        let idx = pos.saturating_mul(num_terminals).saturating_add(neg);
        if idx < branch_currents_len {
            // Safety: caller guarantees valid pointer and bounds.
            let value = unsafe { *branch_currents_ptr.add(idx) };
            if value.is_finite() {
                return value;
            }
        }
    }

    if !currents_ptr.is_null() && currents_len > 0 {
        // Safety: caller guarantees valid pointer and bounds.
        return unsafe { *currents_ptr };
    }

    0.0
}
