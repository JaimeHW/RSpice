//! Versioned linear-memory ABI for generated model modules.
//!
//! All pointers are wasm32 byte offsets rather than Rust pointers. Every
//! variable-size region travels with an explicit element count. The worker
//! validates the frame before invocation and generated entries repeat the
//! magic/version/size guard before reading any capability.

use std::mem::{offset_of, size_of};

use super::WASM_JIT_ABI_VERSION;

pub const WASM_JIT_FRAME_MAGIC: u32 = 0x5253_574a; // "RSWJ"
pub const WASM_JIT_STATUS_OK: i32 = 0;
pub const WASM_JIT_STATUS_ABI_MISMATCH: i32 = -1;
pub const WASM_JIT_STATUS_RUNTIME_ERROR: i32 = -2;

/// Stable header shared by Rust and every secondary WebAssembly module.
///
/// This type is never passed across the boundary as a native pointer. Its
/// layout defines byte offsets into the primary module's linear memory.
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct WasmJitEvalFrame {
    pub magic: u32,
    pub abi_version: u32,
    pub byte_len: u32,
    pub flags: u32,
    pub result: f64,
    pub error_status: i32,
    /// Opaque active runtime-session token. Zero forbids stateful helpers.
    pub session_token: u32,
    pub parameters_ptr: u32,
    pub parameters_len: u32,
    pub parameter_given_ptr: u32,
    pub parameter_given_len: u32,
    pub port_connected_ptr: u32,
    pub port_connected_len: u32,
    pub terminal_voltages_ptr: u32,
    pub terminal_voltages_len: u32,
    pub internal_voltages_ptr: u32,
    pub internal_voltages_len: u32,
    pub currents_ptr: u32,
    pub currents_len: u32,
    pub prior_currents_ptr: u32,
    pub prior_currents_len: u32,
    pub branch_unknowns_ptr: u32,
    pub branch_unknowns_len: u32,
    pub variables_ptr: u32,
    pub variables_len: u32,
    pub analysis_mask: u32,
    /// Generation paired with `session_token` to reject stale dispatches.
    pub session_generation: u32,
    /// Per-stamp instance activation, one byte each. Read only by the fused
    /// kernels, which skip an inactive stamp exactly as the native drivers do.
    pub program_active_ptr: u32,
    pub program_active_len: u32,
    /// Flattened Jacobian output array written by the fused stamp kernel, in
    /// stamp-major then entry order.
    pub jacobians_ptr: u32,
    pub jacobians_len: u32,
    pub temperature: f64,
    pub thermal_voltage: f64,
    pub time: f64,
    pub m_factor: f64,
}

impl Default for WasmJitEvalFrame {
    fn default() -> Self {
        Self {
            magic: WASM_JIT_FRAME_MAGIC,
            abi_version: WASM_JIT_ABI_VERSION,
            byte_len: WASM_JIT_EVAL_FRAME_BYTES,
            flags: 0,
            result: 0.0,
            error_status: WASM_JIT_STATUS_OK,
            session_token: 0,
            parameters_ptr: 0,
            parameters_len: 0,
            parameter_given_ptr: 0,
            parameter_given_len: 0,
            port_connected_ptr: 0,
            port_connected_len: 0,
            terminal_voltages_ptr: 0,
            terminal_voltages_len: 0,
            internal_voltages_ptr: 0,
            internal_voltages_len: 0,
            currents_ptr: 0,
            currents_len: 0,
            prior_currents_ptr: 0,
            prior_currents_len: 0,
            branch_unknowns_ptr: 0,
            branch_unknowns_len: 0,
            variables_ptr: 0,
            variables_len: 0,
            analysis_mask: 0,
            session_generation: 0,
            program_active_ptr: 0,
            program_active_len: 0,
            jacobians_ptr: 0,
            jacobians_len: 0,
            temperature: 0.0,
            thermal_voltage: 0.0,
            time: 0.0,
            m_factor: 1.0,
        }
    }
}

pub const WASM_JIT_EVAL_FRAME_BYTES: u32 = size_of::<WasmJitEvalFrame>() as u32;
pub const FRAME_MAGIC_OFFSET: u64 = offset_of!(WasmJitEvalFrame, magic) as u64;
pub const FRAME_ABI_VERSION_OFFSET: u64 = offset_of!(WasmJitEvalFrame, abi_version) as u64;
pub const FRAME_BYTE_LEN_OFFSET: u64 = offset_of!(WasmJitEvalFrame, byte_len) as u64;
pub const FRAME_RESULT_OFFSET: u64 = offset_of!(WasmJitEvalFrame, result) as u64;
pub const FRAME_ERROR_STATUS_OFFSET: u64 = offset_of!(WasmJitEvalFrame, error_status) as u64;
pub const FRAME_SESSION_TOKEN_OFFSET: u64 = offset_of!(WasmJitEvalFrame, session_token) as u64;
pub const FRAME_PARAMETERS_PTR_OFFSET: u64 = offset_of!(WasmJitEvalFrame, parameters_ptr) as u64;
pub const FRAME_PARAMETERS_LEN_OFFSET: u64 = offset_of!(WasmJitEvalFrame, parameters_len) as u64;
pub const FRAME_PARAMETER_GIVEN_PTR_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, parameter_given_ptr) as u64;
pub const FRAME_PARAMETER_GIVEN_LEN_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, parameter_given_len) as u64;
pub const FRAME_PORT_CONNECTED_PTR_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, port_connected_ptr) as u64;
pub const FRAME_PORT_CONNECTED_LEN_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, port_connected_len) as u64;
pub const FRAME_TERMINAL_VOLTAGES_PTR_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, terminal_voltages_ptr) as u64;
pub const FRAME_TERMINAL_VOLTAGES_LEN_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, terminal_voltages_len) as u64;
pub const FRAME_INTERNAL_VOLTAGES_PTR_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, internal_voltages_ptr) as u64;
pub const FRAME_INTERNAL_VOLTAGES_LEN_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, internal_voltages_len) as u64;
pub const FRAME_CURRENTS_PTR_OFFSET: u64 = offset_of!(WasmJitEvalFrame, currents_ptr) as u64;
pub const FRAME_CURRENTS_LEN_OFFSET: u64 = offset_of!(WasmJitEvalFrame, currents_len) as u64;
pub const FRAME_PRIOR_CURRENTS_PTR_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, prior_currents_ptr) as u64;
pub const FRAME_PRIOR_CURRENTS_LEN_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, prior_currents_len) as u64;
pub const FRAME_BRANCH_UNKNOWNS_PTR_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, branch_unknowns_ptr) as u64;
pub const FRAME_BRANCH_UNKNOWNS_LEN_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, branch_unknowns_len) as u64;
pub const FRAME_VARIABLES_PTR_OFFSET: u64 = offset_of!(WasmJitEvalFrame, variables_ptr) as u64;
pub const FRAME_VARIABLES_LEN_OFFSET: u64 = offset_of!(WasmJitEvalFrame, variables_len) as u64;
pub const FRAME_ANALYSIS_MASK_OFFSET: u64 = offset_of!(WasmJitEvalFrame, analysis_mask) as u64;
pub const FRAME_SESSION_GENERATION_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, session_generation) as u64;
pub const FRAME_PROGRAM_ACTIVE_PTR_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, program_active_ptr) as u64;
pub const FRAME_PROGRAM_ACTIVE_LEN_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, program_active_len) as u64;
pub const FRAME_JACOBIANS_PTR_OFFSET: u64 = offset_of!(WasmJitEvalFrame, jacobians_ptr) as u64;
pub const FRAME_JACOBIANS_LEN_OFFSET: u64 = offset_of!(WasmJitEvalFrame, jacobians_len) as u64;
pub const FRAME_TEMPERATURE_OFFSET: u64 = offset_of!(WasmJitEvalFrame, temperature) as u64;
pub const FRAME_THERMAL_VOLTAGE_OFFSET: u64 = offset_of!(WasmJitEvalFrame, thermal_voltage) as u64;
pub const FRAME_TIME_OFFSET: u64 = offset_of!(WasmJitEvalFrame, time) as u64;
pub const FRAME_M_FACTOR_OFFSET: u64 = offset_of!(WasmJitEvalFrame, m_factor) as u64;

const _: () = {
    assert!(WASM_JIT_EVAL_FRAME_BYTES == 160);
    assert!(FRAME_RESULT_OFFSET == 16);
    assert!(FRAME_SESSION_TOKEN_OFFSET == 28);
    assert!(FRAME_PARAMETERS_PTR_OFFSET == 32);
    assert!(FRAME_ANALYSIS_MASK_OFFSET == 104);
    assert!(FRAME_SESSION_GENERATION_OFFSET == 108);
    assert!(FRAME_PROGRAM_ACTIVE_PTR_OFFSET == 112);
    assert!(FRAME_JACOBIANS_PTR_OFFSET == 120);
    assert!(FRAME_TEMPERATURE_OFFSET == 128);
    assert!(FRAME_M_FACTOR_OFFSET == 152);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_frame_is_versioned_and_layout_is_pinned() {
        let frame = WasmJitEvalFrame::default();
        assert_eq!(frame.magic, WASM_JIT_FRAME_MAGIC);
        assert_eq!(frame.abi_version, WASM_JIT_ABI_VERSION);
        assert_eq!(frame.byte_len, 160);
        assert_eq!(size_of::<WasmJitEvalFrame>(), 160);
        assert_eq!(FRAME_VARIABLES_PTR_OFFSET, 96);
        assert_eq!(FRAME_VARIABLES_LEN_OFFSET, 100);
        assert_eq!(FRAME_PROGRAM_ACTIVE_LEN_OFFSET, 116);
        assert_eq!(FRAME_JACOBIANS_LEN_OFFSET, 124);
    }
}
