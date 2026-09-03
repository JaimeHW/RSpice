//! Versioned linear-memory ABI for generated model modules.
//!
//! All pointers are wasm32 byte offsets rather than Rust pointers. Every
//! variable-size region travels with an explicit element count. The worker
//! validates the frame before invocation and generated entries repeat the
//! magic/version/size guard before reading any capability.

use std::mem::{offset_of, size_of};

use super::WASM_JIT_ABI_VERSION;
#[cfg(any(target_arch = "wasm32", test))]
use crate::codegen::ZiPolynomialLayout;
use crate::codegen::ZiRuntimeLayout;

pub const WASM_JIT_FRAME_MAGIC: u32 = 0x5253_574a; // "RSWJ"
pub const WASM_JIT_STATUS_OK: i32 = 0;
pub const WASM_JIT_STATUS_ABI_MISMATCH: i32 = -1;
pub const WASM_JIT_STATUS_RUNTIME_ERROR: i32 = -2;
/// Fixed start of the authenticated, frame-relative variable-arity operand
/// region. The stable header remains independently addressable at its original
/// offsets.
pub const WASM_JIT_SLICE_OPERANDS_OFFSET: u32 = 168;
/// Browser helper resource limit. This carries 1,020 coefficient values or 510
/// complex-root tuples in addition to Zi's four fixed runtime operands.
pub const WASM_JIT_MAX_SLICE_OPERANDS: usize = crate::zfilter::MAX_ZI_RUNTIME_OPERANDS;

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
    /// Per-evaluation slot array a CFG prelude publishes its shared values
    /// into, and every entry built against that prelude reads. Zero-length for
    /// a plan with no prelude, which is every plan the shipped route builds.
    pub prelude_slots_ptr: u32,
    pub prelude_slots_len: u32,
}

/// Complete primary-module dispatch allocation. Generated modules receive a
/// pointer to `frame` and address variable-arity helper operands immediately
/// after the stable header. Keeping the storage inline makes the capability
/// frame-relative and allocation-free; no secondary module supplies a pointer.
#[repr(C, align(8))]
pub(crate) struct WasmJitDispatchFrame {
    pub frame: WasmJitEvalFrame,
    pub slice_operands: [f64; WASM_JIT_MAX_SLICE_OPERANDS],
}

impl WasmJitDispatchFrame {
    #[cfg(any(target_arch = "wasm32", test))]
    pub fn new(mut frame: WasmJitEvalFrame) -> Self {
        frame.byte_len = WASM_JIT_MAX_EVAL_FRAME_BYTES;
        Self {
            frame,
            slice_operands: [0.0; WASM_JIT_MAX_SLICE_OPERANDS],
        }
    }
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
            prelude_slots_ptr: 0,
            prelude_slots_len: 0,
        }
    }
}

pub const WASM_JIT_EVAL_FRAME_BYTES: u32 = size_of::<WasmJitEvalFrame>() as u32;
pub const WASM_JIT_MAX_EVAL_FRAME_BYTES: u32 = size_of::<WasmJitDispatchFrame>() as u32;
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
pub const FRAME_PRELUDE_SLOTS_PTR_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, prelude_slots_ptr) as u64;
pub const FRAME_PRELUDE_SLOTS_LEN_OFFSET: u64 =
    offset_of!(WasmJitEvalFrame, prelude_slots_len) as u64;

const _: () = {
    assert!(WASM_JIT_MAX_SLICE_OPERANDS == crate::zfilter::MAX_ZI_RUNTIME_OPERANDS);
    assert!(WASM_JIT_EVAL_FRAME_BYTES == 168);
    assert!(WASM_JIT_SLICE_OPERANDS_OFFSET == WASM_JIT_EVAL_FRAME_BYTES);
    assert!(offset_of!(WasmJitDispatchFrame, slice_operands) == 168);
    assert!(WASM_JIT_MAX_EVAL_FRAME_BYTES == 8_360);
    assert!(FRAME_RESULT_OFFSET == 16);
    assert!(FRAME_SESSION_TOKEN_OFFSET == 28);
    assert!(FRAME_PARAMETERS_PTR_OFFSET == 32);
    assert!(FRAME_ANALYSIS_MASK_OFFSET == 104);
    assert!(FRAME_SESSION_GENERATION_OFFSET == 108);
    assert!(FRAME_PROGRAM_ACTIVE_PTR_OFFSET == 112);
    assert!(FRAME_JACOBIANS_PTR_OFFSET == 120);
    assert!(FRAME_TEMPERATURE_OFFSET == 128);
    assert!(FRAME_M_FACTOR_OFFSET == 152);
    assert!(FRAME_PRELUDE_SLOTS_PTR_OFFSET == 160);
    assert!(FRAME_PRELUDE_SLOTS_LEN_OFFSET == 164);
};

const ZI_LAYOUT_LENGTH_MASK: usize = (1 << 14) - 1;

/// Browser-specific 31-bit Zi descriptor. Unlike the native descriptor, this
/// never passes through a machine-word-sized value and is therefore identical
/// on wasm32 and 64-bit hosts. The filter slot travels separately in `aux0`.
pub(crate) fn encode_zi_layout_descriptor(layout: ZiRuntimeLayout) -> Option<i32> {
    layout.validate_operand_budget().ok()?;
    let numerator_len = layout.numerator.definition_len();
    let denominator_len = layout.denominator.definition_len();
    if numerator_len > ZI_LAYOUT_LENGTH_MASK || denominator_len > ZI_LAYOUT_LENGTH_MASK {
        return None;
    }
    let mut packed = u32::try_from(numerator_len).ok()?;
    packed |= u32::from(layout.numerator.is_roots()) << 14;
    packed |= u32::try_from(denominator_len).ok()? << 15;
    packed |= u32::from(layout.denominator.is_roots()) << 29;
    packed |= u32::from(layout.direct_assignment) << 30;
    i32::try_from(packed).ok()
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn decode_zi_layout_descriptor(
    filter_id: usize,
    packed: i32,
) -> Option<ZiRuntimeLayout> {
    let packed = u32::try_from(packed).ok()?;
    if packed >> 31 != 0 {
        return None;
    }
    let numerator_len = usize::try_from(packed & ((1 << 14) - 1)).ok()?;
    let denominator_len = usize::try_from((packed >> 15) & ((1 << 14) - 1)).ok()?;
    let layout = ZiRuntimeLayout {
        filter_id,
        numerator: if (packed >> 14) & 1 == 0 {
            ZiPolynomialLayout::Coefficients { len: numerator_len }
        } else {
            ZiPolynomialLayout::Roots { len: numerator_len }
        },
        denominator: if (packed >> 29) & 1 == 0 {
            ZiPolynomialLayout::Coefficients {
                len: denominator_len,
            }
        } else {
            ZiPolynomialLayout::Roots {
                len: denominator_len,
            }
        },
        direct_assignment: (packed >> 30) & 1 != 0,
    };
    layout.validate_operand_budget().ok()?;
    Some(layout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_frame_is_versioned_and_layout_is_pinned() {
        let frame = WasmJitEvalFrame::default();
        assert_eq!(frame.magic, WASM_JIT_FRAME_MAGIC);
        assert_eq!(frame.abi_version, WASM_JIT_ABI_VERSION);
        assert_eq!(frame.byte_len, 168);
        assert_eq!(size_of::<WasmJitEvalFrame>(), 168);
        assert_eq!(FRAME_VARIABLES_PTR_OFFSET, 96);
        assert_eq!(FRAME_VARIABLES_LEN_OFFSET, 100);
        assert_eq!(FRAME_PROGRAM_ACTIVE_LEN_OFFSET, 116);
        assert_eq!(FRAME_JACOBIANS_LEN_OFFSET, 124);
        assert_eq!(WASM_JIT_SLICE_OPERANDS_OFFSET, 168);
        assert_eq!(WASM_JIT_MAX_EVAL_FRAME_BYTES, 8_360);
        assert_eq!(FRAME_PRELUDE_SLOTS_PTR_OFFSET, 160);
        assert_eq!(FRAME_PRELUDE_SLOTS_LEN_OFFSET, 164);
        let dispatch = WasmJitDispatchFrame::new(frame);
        assert_eq!(dispatch.frame.byte_len, WASM_JIT_MAX_EVAL_FRAME_BYTES);
        assert_eq!(dispatch.slice_operands.len(), WASM_JIT_MAX_SLICE_OPERANDS);
    }

    #[test]
    fn browser_zi_descriptor_is_machine_word_independent() {
        let layout = ZiRuntimeLayout {
            filter_id: 7,
            numerator: ZiPolynomialLayout::Roots { len: 31 },
            denominator: ZiPolynomialLayout::Coefficients { len: 19 },
            direct_assignment: true,
        };
        let packed = encode_zi_layout_descriptor(layout).expect("representable browser layout");
        assert_eq!(decode_zi_layout_descriptor(7, packed), Some(layout));
        assert!(decode_zi_layout_descriptor(7, -1).is_none());

        let over_budget = ZiRuntimeLayout {
            numerator: ZiPolynomialLayout::Coefficients { len: 1_020 },
            denominator: ZiPolynomialLayout::Coefficients { len: 1 },
            ..layout
        };
        assert!(encode_zi_layout_descriptor(over_budget).is_none());
    }
}
