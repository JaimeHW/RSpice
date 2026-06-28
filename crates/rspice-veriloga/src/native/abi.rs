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

/// External helper function for $limit operation.
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
        new_value
    } else {
        unsafe { *state_prev.add(state_idx) }
    };

    if prev_value == 0.0 && new_value != 0.0 {
        return new_value;
    }

    let delta = new_value - prev_value;
    let limited_delta = delta.clamp(-step_limit, step_limit);
    prev_value + limited_delta
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
