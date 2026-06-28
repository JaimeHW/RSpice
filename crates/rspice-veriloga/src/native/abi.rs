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

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::EvalContext;
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
        assert_eq!(offset_of!(EvalContext, lookup_tables), 120);
        assert_eq!(offset_of!(EvalContext, lookup_tables_len), 128);
        assert_eq!(offset_of!(EvalContext, laplace_filters), 136);
        assert_eq!(offset_of!(EvalContext, laplace_filters_len), 144);
        assert_eq!(offset_of!(EvalContext, param_given), 152);
        assert_eq!(offset_of!(EvalContext, branch_unknowns), 160);
        assert_eq!(offset_of!(EvalContext, analysis_type), 168);
        assert_eq!(offset_of!(EvalContext, multiplicity), 176);
        assert_eq!(size_of::<EvalContext>(), 184);
        assert_eq!(align_of::<EvalContext>(), 8);
    }
}
