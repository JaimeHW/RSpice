//! One typed authority for the host x64 calling convention.
//!
//! Helper signatures mix scalar floating-point and integer/pointer arguments.
//! Win64 assigns registers by argument position while System V uses independent
//! floating-point and integer register sequences, so scattering `cfg`-selected
//! registers through codegen is particularly easy to get subtly wrong.

use super::encoder::Gpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum X64AbiKind {
    #[cfg(windows)]
    Windows,
    #[cfg(not(windows))]
    SystemV,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct X64CallingConvention {
    pub(super) kind: X64AbiKind,
    pub(super) call_shadow_bytes: i32,
    pub(super) caller_saved_xmm_count: usize,
    pub(super) entry_context: Gpr,
    pub(super) entry_variables: Gpr,
    pub(super) entry_kernel_io: Gpr,
    pub(super) saved_context: Gpr,
    pub(super) saved_variables: Gpr,
    /// `(context, filter_id)` for helpers whose first positional argument is
    /// the scalar input in XMM0.
    pub(super) context_filter: [Gpr; 2],
    /// `(operands, context, filter_id)` for helpers receiving an operand slice.
    pub(super) operand_filter: [Gpr; 3],
    /// `(slot_ptr, length, lower_bound)` after the scalar index argument.
    pub(super) dynamic_variable: [Gpr; 3],
}

#[cfg(windows)]
pub(super) const HOST_ABI: X64CallingConvention = X64CallingConvention {
    kind: X64AbiKind::Windows,
    call_shadow_bytes: 32,
    caller_saved_xmm_count: 6,
    entry_context: Gpr::Rcx,
    entry_variables: Gpr::Rdx,
    entry_kernel_io: Gpr::R8,
    saved_context: Gpr::R12,
    saved_variables: Gpr::R13,
    context_filter: [Gpr::Rdx, Gpr::R8],
    operand_filter: [Gpr::Rcx, Gpr::Rdx, Gpr::R8],
    dynamic_variable: [Gpr::Rdx, Gpr::R8, Gpr::R9],
};

#[cfg(not(windows))]
pub(super) const HOST_ABI: X64CallingConvention = X64CallingConvention {
    kind: X64AbiKind::SystemV,
    call_shadow_bytes: 0,
    caller_saved_xmm_count: 16,
    entry_context: Gpr::Rdi,
    entry_variables: Gpr::Rsi,
    entry_kernel_io: Gpr::Rdx,
    saved_context: Gpr::R12,
    saved_variables: Gpr::R13,
    context_filter: [Gpr::Rdi, Gpr::Rsi],
    operand_filter: [Gpr::Rdi, Gpr::Rsi, Gpr::Rdx],
    dynamic_variable: [Gpr::Rdi, Gpr::Rsi, Gpr::Rdx],
};

#[cfg(test)]
mod tests {
    use super::{HOST_ABI, X64AbiKind};

    #[test]
    fn host_descriptor_matches_the_compilation_target() {
        #[cfg(windows)]
        assert_eq!(HOST_ABI.kind, X64AbiKind::Windows);
        #[cfg(not(windows))]
        assert_eq!(HOST_ABI.kind, X64AbiKind::SystemV);
        assert_eq!(HOST_ABI.saved_context, super::Gpr::R12);
        assert_eq!(HOST_ABI.saved_variables, super::Gpr::R13);
    }
}
