//! One typed authority for the host AArch64 calling convention.
//!
//! Darwin, ELF AAPCS64, and Windows ARM64 agree on RSpice's public two-pointer
//! entry shape and on the nonvolatile registers used to cache it. Platform
//! distinctions remain explicit so helper-call lowering and unwind metadata
//! cannot accidentally treat the ABIs as interchangeable later.

use super::encoder::{DReg, XReg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum A64AbiKind {
    #[cfg(target_os = "macos")]
    Darwin,
    #[cfg(all(unix, not(target_os = "macos")))]
    ElfAapcs64,
    #[cfg(windows)]
    Windows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct A64CallingConvention {
    pub(super) kind: A64AbiKind,
    pub(super) stack_alignment: usize,
    pub(super) entry_context: XReg,
    pub(super) entry_variables: XReg,
    pub(super) entry_kernel_io: XReg,
    pub(super) saved_context: XReg,
    pub(super) saved_variables: XReg,
    pub(super) saved_kernel_io: XReg,
    pub(super) frame_pointer: XReg,
    pub(super) link_register: XReg,
    pub(super) indirect_call_scratch: XReg,
    pub(super) result: DReg,
    /// Logical SSA registers map only to volatile SIMD registers. D8-D15 are
    /// nonvolatile and deliberately excluded, so leaf arithmetic needs no SIMD
    /// save area on any supported host ABI.
    pub(super) logical_f64_registers: [DReg; 21],
}

const COMMON: A64CallingConvention = A64CallingConvention {
    #[cfg(target_os = "macos")]
    kind: A64AbiKind::Darwin,
    #[cfg(all(unix, not(target_os = "macos")))]
    kind: A64AbiKind::ElfAapcs64,
    #[cfg(windows)]
    kind: A64AbiKind::Windows,
    stack_alignment: 16,
    entry_context: XReg::X0,
    entry_variables: XReg::X1,
    entry_kernel_io: XReg::X2,
    saved_context: XReg::X19,
    saved_variables: XReg::X20,
    saved_kernel_io: XReg::X21,
    frame_pointer: XReg::X29,
    link_register: XReg::X30,
    indirect_call_scratch: XReg::X16,
    result: DReg::D0,
    logical_f64_registers: [
        DReg::D0,
        DReg::D1,
        DReg::D2,
        DReg::D3,
        DReg::D4,
        DReg::D5,
        DReg::D6,
        DReg::D7,
        DReg::D16,
        DReg::D17,
        DReg::D18,
        DReg::D19,
        DReg::D20,
        DReg::D21,
        DReg::D22,
        DReg::D23,
        DReg::D24,
        DReg::D25,
        DReg::D26,
        DReg::D27,
        DReg::D28,
    ],
};

pub(super) const HOST_ABI: A64CallingConvention = COMMON;

#[cfg(test)]
mod tests {
    use super::{A64AbiKind, HOST_ABI};
    use crate::native::aarch64::encoder::{DReg, XReg};

    #[test]
    fn host_descriptor_matches_target_and_excludes_reserved_registers() {
        #[cfg(target_os = "macos")]
        assert_eq!(HOST_ABI.kind, A64AbiKind::Darwin);
        #[cfg(all(unix, not(target_os = "macos")))]
        assert_eq!(HOST_ABI.kind, A64AbiKind::ElfAapcs64);
        #[cfg(windows)]
        assert_eq!(HOST_ABI.kind, A64AbiKind::Windows);

        assert_eq!(HOST_ABI.stack_alignment, 16);
        assert_eq!(HOST_ABI.entry_context, XReg::X0);
        assert_eq!(HOST_ABI.entry_variables, XReg::X1);
        assert_eq!(HOST_ABI.saved_context, XReg::X19);
        assert_eq!(HOST_ABI.saved_variables, XReg::X20);
        assert_eq!(HOST_ABI.frame_pointer, XReg::X29);
        assert_eq!(HOST_ABI.link_register, XReg::X30);
        assert_eq!(HOST_ABI.indirect_call_scratch, XReg::X16);
        assert_eq!(HOST_ABI.result, DReg::D0);
        assert!(!HOST_ABI.logical_f64_registers.contains(&DReg::D8));
        assert_eq!(HOST_ABI.logical_f64_registers[8], DReg::D16);
    }
}
