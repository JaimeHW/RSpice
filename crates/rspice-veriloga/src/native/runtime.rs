//! Allocating and protecting the pages emitted code runs from.
//!
//! [`ExecutableMemory`] wraps the platform calls — `VirtualAlloc`/
//! `VirtualProtect` on Windows, `mmap`/`mprotect` elsewhere — behind one owned
//! allocation that unmaps on drop.
//!
//! Pages are written while writable and only then flipped to executable, never
//! mapped both at once, and the instruction cache is flushed before the memory
//! is called. This module is the whole of the crate's raw-pointer and page-
//! permission surface by design: keeping it here is what lets the rest of the
//! native backend be reviewed as ordinary code.
//!
//! ## Floating-point environment
//!
//! Generated code inherits the calling thread's floating-point control word
//! (`MXCSR` on x64, `FPCR` on AArch64) and never writes it. Round-to-nearest
//! with denormals enabled is assumed, which is what every supported host
//! establishes at thread creation and what Rust itself relies on.
//!
//! This is deliberately not enforced. A third-party library that enables
//! flush-to-zero on a simulation thread would change generated code's denormal
//! behaviour -- but it would change the bytecode interpreter's and the
//! ahead-of-time compiled models' behaviour identically, because they are
//! ordinary compiled `f64` arithmetic on the same thread. Setting the control
//! word here would therefore make the backends *disagree* rather than agree.
//! Any such policy has to be a process-wide decision, not one this module
//! makes on its own.

#[cfg(all(target_arch = "aarch64", unix))]
use super::aarch64::unwind::A64UnwindFunction;
#[cfg(all(target_arch = "aarch64", windows))]
use super::aarch64::unwind::WindowsA64RuntimeFunction;
use super::{JitError, JitResult};
use std::ptr::{self, NonNull};
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
use std::sync::{Arc, Mutex, OnceLock};

// PSP104 currently emits roughly 28 MiB. Retain bounded virtual-memory use
// while leaving headroom for larger commercial compact models and future
// lowering growth.
const MAX_EXECUTABLE_IMAGE_BYTES: usize = 256 * 1024 * 1024;

/// Virtual capacity of the single process-wide Apple-Silicon `MAP_JIT` region.
///
/// Apple permits one JIT mapping in a hardened process. The arena is virtual
/// address space; physical pages are committed only as generated models are
/// published. Released page ranges are returned to the free list and advised
/// away before reuse.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const MACOS_JIT_ARENA_BYTES: usize = 4 * 1024 * 1024 * 1024;

pub(crate) struct ExecutableMemory {
    ptr: NonNull<u8>,
    len: usize,
    #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
    allocation_ptr: NonNull<u8>,
    #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
    #[cfg_attr(windows, allow(dead_code))]
    allocation_len: usize,
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    _macos_allocation: MacosJitAllocation,
    #[cfg(all(unix, target_arch = "aarch64"))]
    unix_aarch64_unwind: Option<UnixA64UnwindRegistration>,
    #[cfg(all(windows, target_arch = "x86_64"))]
    windows_function_table:
        Option<Box<[windows_sys::Win32::System::Diagnostics::Debug::IMAGE_RUNTIME_FUNCTION_ENTRY]>>,
    #[cfg(all(windows, target_arch = "aarch64"))]
    windows_function_table: Option<
        Box<[windows_sys::Win32::System::Diagnostics::Debug::IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY]>,
    >,
}

#[cfg(all(test, feature = "native", target_arch = "aarch64"))]
mod aarch64_tests {
    use super::{ExecutableMemory, MAX_EXECUTABLE_IMAGE_BYTES, sync_instruction_cache};
    use crate::native::JitError;
    use crate::native::aarch64::encoder::{A64Encoder, XReg};
    use std::sync::Arc;
    #[cfg(unix)]
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[cfg(unix)]
    static EXPECTED_JIT_START: AtomicUsize = AtomicUsize::new(0);
    #[cfg(unix)]
    static EXPECTED_JIT_END: AtomicUsize = AtomicUsize::new(0);

    #[cfg(unix)]
    #[repr(C)]
    struct DwarfEhBases {
        text_base: usize,
        data_base: usize,
        function: usize,
    }

    #[cfg(unix)]
    unsafe extern "C" {
        fn _Unwind_Find_FDE(
            pc: *const std::ffi::c_void,
            bases: *mut DwarfEhBases,
        ) -> *const std::ffi::c_void;
        fn _Unwind_Backtrace(
            callback: unsafe extern "C" fn(
                context: *mut std::ffi::c_void,
                argument: *mut std::ffi::c_void,
            ) -> i32,
            argument: *mut std::ffi::c_void,
        ) -> i32;
        fn _Unwind_GetIP(context: *mut std::ffi::c_void) -> usize;
    }

    #[cfg(unix)]
    unsafe extern "C" fn find_jit_frame(
        context: *mut std::ffi::c_void,
        found: *mut std::ffi::c_void,
    ) -> i32 {
        let ip = unsafe { _Unwind_GetIP(context) };
        let start = EXPECTED_JIT_START.load(Ordering::Relaxed);
        let end = EXPECTED_JIT_END.load(Ordering::Relaxed);
        if start <= ip && ip < end {
            unsafe {
                *found.cast::<u8>() = 1;
            }
        }
        0 // _URC_NO_REASON
    }

    #[cfg(unix)]
    extern "C" fn capture_jit_frame() -> u64 {
        let mut found = 0_u8;
        unsafe {
            let _ = _Unwind_Backtrace(find_jit_frame, (&mut found as *mut u8).cast());
        }
        u64::from(found)
    }

    fn return_constant(value: u16) -> Vec<u8> {
        let mut encoder = A64Encoder::new();
        encoder
            .movz_x(XReg::X0, value, 0)
            .expect("encode return value");
        encoder.ret();
        encoder.into_bytes()
    }

    fn call_u64(memory: &ExecutableMemory) -> u64 {
        let entry: extern "C" fn() -> u64 = unsafe {
            std::mem::transmute(memory.ptr_at(0).expect("entry pointer inside allocation"))
        };
        entry()
    }

    #[test]
    fn executable_memory_rejects_empty_and_oversized_images() {
        let empty = match ExecutableMemory::allocate(&[]) {
            Ok(_) => panic!("empty image should fail"),
            Err(error) => error,
        };
        assert!(matches!(empty, JitError::ExecutableMemory { .. }));

        let oversized = super::validate_image_len(MAX_EXECUTABLE_IMAGE_BYTES + 1)
            .expect_err("oversized image should fail");
        assert!(oversized.to_string().contains("safety limit"));
    }

    #[test]
    fn executable_memory_owns_source_bytes_and_executes_published_code() {
        let mut bytes = return_constant(42);
        let memory = ExecutableMemory::allocate(&bytes).expect("publish A64 function");
        bytes.fill(0);

        assert_eq!(call_u64(&memory), 42);
        assert_eq!(memory.len(), 8);
        assert!(memory.ptr_at(memory.len()).is_err());
    }

    #[test]
    fn executable_memory_reuses_released_arena_ranges() {
        for iteration in 0..2048_u16 {
            let expected = iteration ^ 0x5a5a;
            let memory = ExecutableMemory::allocate(&return_constant(expected))
                .expect("publish repeated A64 function");
            assert_eq!(call_u64(&memory), u64::from(expected));
        }
    }

    #[test]
    fn executable_memory_supports_parallel_publication_and_execution() {
        let workers = (0..8_u16)
            .map(|worker| {
                std::thread::spawn(move || {
                    for iteration in 0..128_u16 {
                        let expected = worker.wrapping_mul(257).wrapping_add(iteration);
                        let memory = Arc::new(
                            ExecutableMemory::allocate(&return_constant(expected))
                                .expect("publish concurrent A64 function"),
                        );
                        let shared = Arc::clone(&memory);
                        assert_eq!(call_u64(&shared), u64::from(expected));
                    }
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().expect("A64 JIT worker completes");
        }
    }

    #[test]
    fn instruction_cache_sync_accepts_published_code() {
        let memory = ExecutableMemory::allocate(&return_constant(42))
            .expect("publish A64 function for cache sync");
        sync_instruction_cache(
            memory.ptr_at(0).expect("published entry pointer"),
            memory.len(),
        )
        .expect("synchronize A64 instruction cache");
    }

    #[cfg(unix)]
    #[test]
    fn registered_dwarf_frame_covers_generated_nonleaf_entry() {
        use crate::native::aarch64::unwind::analyze_function;
        use crate::native::model::CodeOffset;

        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        encoder
            .stp_x_pre(XReg::X19, XReg::X20, XReg::Sp, -16)
            .unwrap();
        encoder
            .stp_x_pre(XReg::X29, XReg::X30, XReg::Sp, -16)
            .unwrap();
        encoder.add_x_imm(XReg::X29, XReg::Sp, 0).unwrap();
        encoder.nop();
        encoder
            .ldp_x_post(XReg::X29, XReg::X30, XReg::Sp, 16)
            .unwrap();
        encoder
            .ldp_x_post(XReg::X19, XReg::X20, XReg::Sp, 16)
            .unwrap();
        encoder.ret();
        let bytes = encoder.into_bytes();
        let function = analyze_function(CodeOffset::new(0), &bytes, "unwind sentinel")
            .expect("derive sentinel unwind shape");
        let memory = ExecutableMemory::allocate_with_aarch64_unwind(&bytes, &[function])
            .expect("publish registered sentinel");
        let pc = memory.ptr_at(16).expect("sentinel body address");
        let mut bases = DwarfEhBases {
            text_base: 0,
            data_base: 0,
            function: 0,
        };
        let fde = unsafe { _Unwind_Find_FDE(pc.cast(), &mut bases) };
        assert!(!fde.is_null(), "libunwind must find the generated FDE");
        assert_eq!(bases.function, memory.ptr_at(0).unwrap() as usize);
    }

    #[cfg(unix)]
    #[test]
    fn registered_dwarf_cfi_unwinds_through_generated_helper_frame() {
        use crate::native::aarch64::unwind::analyze_function;
        use crate::native::model::CodeOffset;

        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        encoder
            .stp_x_pre(XReg::X19, XReg::X20, XReg::Sp, -16)
            .unwrap();
        encoder
            .stp_x_pre(XReg::X29, XReg::X30, XReg::Sp, -16)
            .unwrap();
        encoder.add_x_imm(XReg::X29, XReg::Sp, 0).unwrap();
        encoder
            .mov_u64(XReg::X16, capture_jit_frame as *const () as usize as u64)
            .unwrap();
        encoder.blr(XReg::X16);
        encoder
            .ldp_x_post(XReg::X29, XReg::X30, XReg::Sp, 16)
            .unwrap();
        encoder
            .ldp_x_post(XReg::X19, XReg::X20, XReg::Sp, 16)
            .unwrap();
        encoder.ret();
        let bytes = encoder.into_bytes();
        let function = analyze_function(CodeOffset::new(0), &bytes, "backtrace sentinel")
            .expect("derive backtrace unwind shape");
        let memory = ExecutableMemory::allocate_with_aarch64_unwind(&bytes, &[function])
            .expect("publish backtrace sentinel");
        EXPECTED_JIT_START.store(memory.ptr_at(0).unwrap() as usize, Ordering::Relaxed);
        EXPECTED_JIT_END.store(
            memory.ptr_at(memory.len() - 1).unwrap() as usize + 1,
            Ordering::Relaxed,
        );
        let entry: extern "C" fn() -> u64 =
            unsafe { std::mem::transmute(memory.ptr_at(0).expect("backtrace sentinel entry")) };
        assert_eq!(entry(), 1, "system unwinder must traverse the JIT frame");
    }

    #[cfg(windows)]
    #[test]
    fn windows_registers_generated_arm64_full_xdata() {
        use crate::native::aarch64::unwind::{analyze_function, append_windows_unwind_data};
        use crate::native::model::CodeOffset;
        use windows_sys::Win32::System::Diagnostics::Debug::RtlLookupFunctionEntry;

        let mut encoder = A64Encoder::new();
        encoder.bti_c();
        encoder
            .stp_x_pre(XReg::X19, XReg::X20, XReg::Sp, -16)
            .unwrap();
        encoder
            .stp_x_pre(XReg::X29, XReg::X30, XReg::Sp, -16)
            .unwrap();
        encoder.add_x_imm(XReg::X29, XReg::Sp, 0).unwrap();
        encoder.nop();
        encoder
            .ldp_x_post(XReg::X29, XReg::X30, XReg::Sp, 16)
            .unwrap();
        encoder
            .ldp_x_post(XReg::X19, XReg::X20, XReg::Sp, 16)
            .unwrap();
        encoder.ret();
        let mut image = encoder.into_bytes();
        let function = analyze_function(CodeOffset::new(0), &image, "Windows unwind sentinel")
            .expect("derive Windows unwind shape");
        let functions = append_windows_unwind_data(&mut image, &[function])
            .expect("append Windows ARM64 xdata");
        let memory = ExecutableMemory::allocate_with_aarch64_unwind(&image, &functions)
            .expect("register Windows ARM64 function table");

        let mut image_base = 0_usize;
        let control_pc = memory.ptr_at(16).unwrap() as usize;
        let runtime_function =
            unsafe { RtlLookupFunctionEntry(control_pc, &mut image_base, std::ptr::null_mut()) };
        assert!(!runtime_function.is_null());
        assert_eq!(image_base, memory.ptr_at(0).unwrap() as usize);
        assert_eq!(unsafe { (*runtime_function).BeginAddress }, 0);
        assert_eq!(
            unsafe { (*runtime_function).Anonymous.UnwindData },
            functions[0].unwind_data
        );
        let header = unsafe {
            std::ptr::read_unaligned(
                memory
                    .ptr_at(functions[0].unwind_data as usize)
                    .unwrap()
                    .cast::<u32>(),
            )
        };
        assert_eq!(header & 0x0003_ffff, (function.len() / 4) as u32);
    }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FreeRange {
    offset: usize,
    len: usize,
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
struct MacosJitArena {
    mapping_ptr: NonNull<u8>,
    mapping_len: usize,
    executable_ptr: NonNull<u8>,
    executable_len: usize,
    page_size: usize,
    free_ranges: Mutex<Vec<FreeRange>>,
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
struct MacosJitAllocation {
    arena: Arc<MacosJitArena>,
    offset: usize,
    reserved_len: usize,
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
static MACOS_JIT_ARENA: OnceLock<Result<Arc<MacosJitArena>, JitError>> = OnceLock::new();

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
unsafe extern "C" {
    fn rspice_jit_publish(
        arena_start: *mut std::ffi::c_void,
        arena_end: *mut std::ffi::c_void,
        destination: *mut std::ffi::c_void,
        source: *const std::ffi::c_void,
        length: usize,
    ) -> std::ffi::c_int;
    fn rspice_clear_instruction_cache(start: *mut std::ffi::c_void, length: usize);
}

#[cfg(all(unix, target_arch = "aarch64", not(target_os = "macos")))]
unsafe extern "C" {
    fn rspice_clear_instruction_cache(start: *mut std::ffi::c_void, length: usize);
}

#[cfg(all(windows, target_arch = "x86_64"))]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WindowsX64RuntimeFunction {
    pub(crate) begin_address: u32,
    pub(crate) end_address: u32,
    pub(crate) unwind_info_address: u32,
}

#[cfg(all(unix, target_arch = "aarch64"))]
struct UnixA64UnwindRegistration {
    eh_frame: super::aarch64::unwind::A64EhFrame,
}

#[cfg(all(unix, target_arch = "aarch64"))]
unsafe extern "C" {
    fn __register_frame(begin: *const std::ffi::c_void);
    fn __deregister_frame(begin: *const std::ffi::c_void);
}

#[cfg(all(unix, target_arch = "aarch64"))]
impl UnixA64UnwindRegistration {
    fn register(code_base: *const u8, functions: &[A64UnwindFunction]) -> JitResult<Self> {
        let eh_frame = super::aarch64::unwind::encode_eh_frame(code_base, functions)?;
        #[cfg(target_os = "macos")]
        for offset in &eh_frame.fde_offsets {
            let fde = unsafe { eh_frame.words.as_ptr().cast::<u8>().add(*offset) };
            unsafe {
                __register_frame(fde.cast());
            }
        }
        #[cfg(not(target_os = "macos"))]
        unsafe {
            __register_frame(eh_frame.words.as_ptr().cast());
        }
        Ok(Self { eh_frame })
    }
}

#[cfg(all(unix, target_arch = "aarch64"))]
impl Drop for UnixA64UnwindRegistration {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        for offset in self.eh_frame.fde_offsets.iter().rev() {
            let fde = unsafe { self.eh_frame.words.as_ptr().cast::<u8>().add(*offset) };
            unsafe {
                __deregister_frame(fde.cast());
            }
        }
        #[cfg(not(target_os = "macos"))]
        unsafe {
            __deregister_frame(self.eh_frame.words.as_ptr().cast());
        }
    }
}

impl ExecutableMemory {
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    #[cfg(all(test, feature = "native", target_arch = "x86_64"))]
    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The published image as bytes, for digesting a compiled model.
    ///
    /// Gated like [`Self::is_empty`] above it: its one caller is
    /// `NativeModel::image_bytes`, which the identity and cost censuses read
    /// and which is x86-64 only, so an AArch64 test build denies this as dead
    /// code.
    #[cfg(all(test, feature = "native", target_arch = "x86_64"))]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        // Safety: the allocation is `len` bytes long and lives as long as
        // `self`; publication has already made it read-execute, and reading
        // executable memory is defined.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    pub(crate) fn ptr_at(&self, offset: usize) -> JitResult<*const u8> {
        if offset >= self.len {
            return Err(JitError::ExecutableMemory {
                detail: format!(
                    "entry offset {offset} outside executable image length {}",
                    self.len
                )
                .into(),
            });
        }
        Ok(unsafe { self.ptr.as_ptr().add(offset) as *const u8 })
    }

    #[cfg(all(test, target_arch = "x86_64", any(windows, target_os = "linux")))]
    fn base_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    #[cfg(all(test, windows, target_arch = "x86_64"))]
    fn allocation_ptr(&self) -> *const u8 {
        self.allocation_ptr.as_ptr()
    }

    #[cfg(all(test, windows, target_arch = "x86_64"))]
    fn allocation_len(&self) -> usize {
        self.allocation_len
    }
}

fn validate_image_size(bytes: &[u8]) -> JitResult<()> {
    if bytes.is_empty() {
        return Err(JitError::ExecutableMemory {
            detail: "cannot allocate empty executable image".into(),
        });
    }
    validate_image_len(bytes.len())
}

fn validate_image_len(image_len: usize) -> JitResult<()> {
    if image_len > MAX_EXECUTABLE_IMAGE_BYTES {
        return Err(JitError::ExecutableMemory {
            detail: format!(
                "executable image length {} exceeds the {}-byte safety limit",
                image_len, MAX_EXECUTABLE_IMAGE_BYTES
            )
            .into(),
        });
    }
    Ok(())
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
fn guarded_allocation_layout(image_len: usize, page_size: usize) -> JitResult<(usize, usize)> {
    if page_size == 0 || !page_size.is_power_of_two() {
        return Err(JitError::ExecutableMemory {
            detail: format!("invalid operating-system page size {page_size}").into(),
        });
    }
    let rounded_image_len = image_len
        .checked_add(page_size - 1)
        .map(|len| len & !(page_size - 1))
        .ok_or_else(|| JitError::ExecutableMemory {
            detail: "executable image page rounding overflow".into(),
        })?;
    let allocation_len = rounded_image_len
        .checked_add(
            page_size
                .checked_mul(2)
                .ok_or_else(|| JitError::ExecutableMemory {
                    detail: "executable guard-page size overflow".into(),
                })?,
        )
        .ok_or_else(|| JitError::ExecutableMemory {
            detail: "guarded executable allocation size overflow".into(),
        })?;
    Ok((rounded_image_len, allocation_len))
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
impl MacosJitArena {
    fn shared() -> JitResult<Arc<Self>> {
        MACOS_JIT_ARENA
            .get_or_init(|| Self::allocate().map(Arc::new))
            .clone()
    }

    fn allocate() -> JitResult<Self> {
        use libc::{
            _SC_PAGESIZE, MAP_ANON, MAP_FAILED, MAP_JIT, MAP_PRIVATE, PROT_EXEC, PROT_READ,
            PROT_WRITE, mmap, sysconf,
        };

        let page_size = unsafe { sysconf(_SC_PAGESIZE) };
        if page_size <= 0 {
            return Err(JitError::ExecutableMemory {
                detail: std::io::Error::last_os_error().to_string().into(),
            });
        }
        let page_size = usize::try_from(page_size).map_err(|_| JitError::ExecutableMemory {
            detail: "operating-system page size does not fit usize".into(),
        })?;
        if !page_size.is_power_of_two() {
            return Err(JitError::ExecutableMemory {
                detail: format!("invalid operating-system page size {page_size}").into(),
            });
        }
        let executable = unsafe {
            mmap(
                ptr::null_mut(),
                MACOS_JIT_ARENA_BYTES,
                PROT_READ | PROT_WRITE | PROT_EXEC,
                MAP_PRIVATE | MAP_ANON | MAP_JIT,
                -1,
                0,
            )
        };
        if executable == MAP_FAILED {
            return Err(JitError::ExecutableMemory {
                detail: format!(
                    "mmap(MAP_JIT) failed for the Apple-Silicon JIT arena: {}",
                    std::io::Error::last_os_error()
                )
                .into(),
            });
        }

        Ok(Self {
            mapping_ptr: NonNull::new(executable.cast::<u8>()).expect("mmap result was checked"),
            mapping_len: MACOS_JIT_ARENA_BYTES,
            executable_ptr: NonNull::new(executable.cast::<u8>())
                .expect("fixed MAP_JIT result was checked"),
            executable_len: MACOS_JIT_ARENA_BYTES,
            page_size,
            free_ranges: Mutex::new(vec![FreeRange {
                offset: 0,
                len: MACOS_JIT_ARENA_BYTES,
            }]),
        })
    }

    fn publish(self: &Arc<Self>, bytes: &[u8]) -> JitResult<MacosJitAllocation> {
        validate_image_size(bytes)?;
        let reserved_len = bytes
            .len()
            .checked_add(self.page_size - 1)
            .map(|len| len & !(self.page_size - 1))
            .ok_or_else(|| JitError::ExecutableMemory {
                detail: "Apple-Silicon JIT allocation rounding overflow".into(),
            })?;
        let offset = self.reserve(reserved_len)?;
        let destination = unsafe { self.executable_ptr.as_ptr().add(offset) };
        let arena_start = self.executable_ptr.as_ptr();
        let arena_end = unsafe { arena_start.add(self.executable_len) };
        let result = unsafe {
            rspice_jit_publish(
                arena_start.cast(),
                arena_end.cast(),
                destination.cast(),
                bytes.as_ptr().cast(),
                bytes.len(),
            )
        };
        if result != 0 {
            self.release(offset, reserved_len);
            return Err(JitError::ExecutableMemory {
                detail: format!(
                    "pthread_jit_write_with_callback_np rejected an authenticated JIT publication with status {result}"
                )
                .into(),
            });
        }
        if let Err(error) = sync_instruction_cache(destination, bytes.len()) {
            self.release(offset, reserved_len);
            return Err(error);
        }

        Ok(MacosJitAllocation {
            arena: Arc::clone(self),
            offset,
            reserved_len,
        })
    }

    fn reserve(&self, len: usize) -> JitResult<usize> {
        let mut free_ranges = self
            .free_ranges
            .lock()
            .map_err(|_| JitError::ExecutableMemory {
                detail: "Apple-Silicon JIT arena allocator lock is poisoned".into(),
            })?;
        let Some(index) = free_ranges.iter().position(|range| range.len >= len) else {
            let free_bytes = free_ranges.iter().map(|range| range.len).sum::<usize>();
            return Err(JitError::ExecutableMemory {
                detail: format!(
                    "Apple-Silicon JIT arena exhausted: requested {len} bytes, {free_bytes} bytes remain free in {} disjoint range(s)",
                    free_ranges.len()
                )
                .into(),
            });
        };
        let offset = free_ranges[index].offset;
        if free_ranges[index].len == len {
            free_ranges.remove(index);
        } else {
            free_ranges[index].offset += len;
            free_ranges[index].len -= len;
        }
        Ok(offset)
    }

    fn release(&self, offset: usize, len: usize) {
        use libc::{MADV_DONTNEED, madvise};

        if offset
            .checked_add(len)
            .is_none_or(|end| end > self.executable_len)
        {
            return;
        }
        let ptr = unsafe { self.executable_ptr.as_ptr().add(offset) };
        unsafe {
            let _ = madvise(ptr.cast(), len, MADV_DONTNEED);
        }

        let Ok(mut free_ranges) = self.free_ranges.lock() else {
            return;
        };
        free_ranges.push(FreeRange { offset, len });
        free_ranges.sort_unstable_by_key(|range| range.offset);
        let mut merged: Vec<FreeRange> = Vec::with_capacity(free_ranges.len());
        for range in free_ranges.drain(..) {
            if let Some(previous) = merged.last_mut()
                && previous.offset.checked_add(previous.len) == Some(range.offset)
            {
                previous.len += range.len;
            } else {
                merged.push(range);
            }
        }
        *free_ranges = merged;
    }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
impl Drop for MacosJitAllocation {
    fn drop(&mut self) {
        self.arena.release(self.offset, self.reserved_len);
    }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
impl Drop for MacosJitArena {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.mapping_ptr.as_ptr().cast(), self.mapping_len);
        }
    }
}

// Safety: allocation and reclamation are serialized by `free_ranges`; the
// mapping base and length are immutable for the arena's lifetime.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
unsafe impl Send for MacosJitArena {}
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
unsafe impl Sync for MacosJitArena {}

#[cfg(all(windows, target_arch = "x86_64"))]
impl ExecutableMemory {
    pub(crate) fn allocate_with_windows_unwind(
        bytes: &[u8],
        functions: &[WindowsX64RuntimeFunction],
    ) -> JitResult<Self> {
        use windows_sys::Win32::System::Diagnostics::Debug::{
            IMAGE_RUNTIME_FUNCTION_ENTRY, IMAGE_RUNTIME_FUNCTION_ENTRY_0, RtlAddFunctionTable,
        };

        let mut memory = Self::allocate(bytes)?;
        if functions.is_empty() {
            return Ok(memory);
        }
        let entry_count =
            u32::try_from(functions.len()).map_err(|_| JitError::ExecutableMemory {
                detail: "Windows x64 runtime-function table length exceeds u32".into(),
            })?;
        let table = functions
            .iter()
            .map(|function| IMAGE_RUNTIME_FUNCTION_ENTRY {
                BeginAddress: function.begin_address,
                EndAddress: function.end_address,
                Anonymous: IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
                    UnwindInfoAddress: function.unwind_info_address,
                },
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let registered = unsafe {
            RtlAddFunctionTable(
                table.as_ptr(),
                entry_count,
                memory.ptr.as_ptr() as usize as u64,
            )
        };
        if !registered {
            return Err(JitError::ExecutableMemory {
                detail: "RtlAddFunctionTable rejected generated Windows x64 unwind metadata".into(),
            });
        }
        memory.windows_function_table = Some(table);
        Ok(memory)
    }

    #[allow(dead_code)]
    pub(crate) fn allocate(bytes: &[u8]) -> JitResult<Self> {
        allocate_windows(bytes)
    }
}

#[cfg(all(windows, target_arch = "aarch64"))]
impl ExecutableMemory {
    pub(crate) fn allocate_with_aarch64_unwind(
        bytes: &[u8],
        functions: &[WindowsA64RuntimeFunction],
    ) -> JitResult<Self> {
        use windows_sys::Win32::System::Diagnostics::Debug::{
            IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0,
            RtlAddFunctionTable,
        };

        let mut memory = Self::allocate(bytes)?;
        if functions.is_empty() {
            return Ok(memory);
        }
        let entry_count =
            u32::try_from(functions.len()).map_err(|_| JitError::ExecutableMemory {
                detail: "Windows ARM64 runtime-function table length exceeds u32".into(),
            })?;
        let table = functions
            .iter()
            .map(|function| IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
                BeginAddress: function.begin_address,
                Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_0 {
                    UnwindData: function.unwind_data,
                },
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let registered = unsafe {
            RtlAddFunctionTable(table.as_ptr(), entry_count, memory.ptr.as_ptr() as usize)
        };
        if !registered {
            return Err(JitError::ExecutableMemory {
                detail: "RtlAddFunctionTable rejected generated Windows ARM64 unwind metadata"
                    .into(),
            });
        }
        memory.windows_function_table = Some(table);
        Ok(memory)
    }

    #[allow(dead_code)]
    pub(crate) fn allocate(bytes: &[u8]) -> JitResult<Self> {
        allocate_windows(bytes)
    }
}

#[cfg(windows)]
fn allocate_windows(bytes: &[u8]) -> JitResult<ExecutableMemory> {
    use windows_sys::Win32::Foundation::GetLastError;
    use windows_sys::Win32::System::Memory::{
        MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_NOACCESS, PAGE_READWRITE,
        VirtualAlloc, VirtualFree, VirtualProtect,
    };
    use windows_sys::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};

    validate_image_size(bytes)?;
    let mut system_info = std::mem::MaybeUninit::<SYSTEM_INFO>::zeroed();
    unsafe {
        GetSystemInfo(system_info.as_mut_ptr());
    }
    let page_size = unsafe { system_info.assume_init().dwPageSize as usize };
    let (committed_len, allocation_len) = guarded_allocation_layout(bytes.len(), page_size)?;

    let allocation_ptr =
        unsafe { VirtualAlloc(ptr::null_mut(), allocation_len, MEM_RESERVE, PAGE_NOACCESS) };
    if allocation_ptr.is_null() {
        return Err(JitError::ExecutableMemory {
            detail: format!("VirtualAlloc reserve failed with error {}", unsafe {
                GetLastError()
            })
            .into(),
        });
    }
    let ptr = unsafe { allocation_ptr.cast::<u8>().add(page_size).cast() };
    let committed_ptr = unsafe { VirtualAlloc(ptr, committed_len, MEM_COMMIT, PAGE_READWRITE) };
    if committed_ptr.is_null() {
        let error = unsafe { GetLastError() };
        unsafe {
            VirtualFree(allocation_ptr, 0, MEM_RELEASE);
        }
        return Err(JitError::ExecutableMemory {
            detail: format!("VirtualAlloc commit failed with error {error}").into(),
        });
    }

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.cast::<u8>(), bytes.len());
    }

    let mut old_protect = PAGE_READWRITE;
    let protect_ok =
        unsafe { VirtualProtect(ptr, committed_len, PAGE_EXECUTE_READ, &mut old_protect) };
    if protect_ok == 0 {
        let error = unsafe { GetLastError() };
        unsafe {
            VirtualFree(allocation_ptr, 0, MEM_RELEASE);
        }
        return Err(JitError::ExecutableMemory {
            detail: format!("VirtualProtect failed with error {error}").into(),
        });
    }

    if let Err(error) = sync_instruction_cache(ptr.cast::<u8>(), bytes.len()) {
        unsafe {
            VirtualFree(allocation_ptr, 0, MEM_RELEASE);
        }
        return Err(error);
    }

    Ok(ExecutableMemory {
        ptr: NonNull::new(ptr.cast::<u8>()).expect("VirtualAlloc returned a checked pointer"),
        len: bytes.len(),
        allocation_ptr: NonNull::new(allocation_ptr.cast::<u8>())
            .expect("VirtualAlloc reserve returned a checked pointer"),
        allocation_len,
        windows_function_table: None,
    })
}

#[cfg(all(unix, target_arch = "aarch64"))]
impl ExecutableMemory {
    pub(crate) fn allocate_with_aarch64_unwind(
        bytes: &[u8],
        functions: &[A64UnwindFunction],
    ) -> JitResult<Self> {
        let mut memory = Self::allocate(bytes)?;
        memory.unix_aarch64_unwind = Some(UnixA64UnwindRegistration::register(
            memory.ptr.as_ptr(),
            functions,
        )?);
        Ok(memory)
    }
}

#[cfg(all(unix, not(all(target_os = "macos", target_arch = "aarch64"))))]
impl ExecutableMemory {
    #[allow(dead_code)]
    pub(crate) fn allocate(bytes: &[u8]) -> JitResult<Self> {
        use libc::{
            _SC_PAGESIZE, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_EXEC, PROT_NONE, PROT_READ,
            PROT_WRITE, mmap, mprotect, munmap, sysconf,
        };

        validate_image_size(bytes)?;
        let page_size = unsafe { sysconf(_SC_PAGESIZE) };
        if page_size <= 0 {
            return Err(JitError::ExecutableMemory {
                detail: std::io::Error::last_os_error().to_string().into(),
            });
        }
        let page_size = usize::try_from(page_size).map_err(|_| JitError::ExecutableMemory {
            detail: "operating-system page size does not fit usize".into(),
        })?;
        let (mapped_image_len, allocation_len) = guarded_allocation_layout(bytes.len(), page_size)?;

        let allocation_ptr = unsafe {
            mmap(
                ptr::null_mut(),
                allocation_len,
                PROT_NONE,
                MAP_PRIVATE | MAP_ANON,
                -1,
                0,
            )
        };
        if allocation_ptr == MAP_FAILED {
            return Err(JitError::ExecutableMemory {
                detail: std::io::Error::last_os_error().to_string().into(),
            });
        }
        let ptr = unsafe { allocation_ptr.cast::<u8>().add(page_size).cast() };
        if unsafe { mprotect(ptr, mapped_image_len, PROT_READ | PROT_WRITE) } != 0 {
            let error = std::io::Error::last_os_error();
            unsafe {
                munmap(allocation_ptr, allocation_len);
            }
            return Err(JitError::ExecutableMemory {
                detail: error.to_string().into(),
            });
        }

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.cast::<u8>(), bytes.len());
        }

        if unsafe { mprotect(ptr, mapped_image_len, PROT_READ | PROT_EXEC) } != 0 {
            let error = std::io::Error::last_os_error();
            unsafe {
                munmap(allocation_ptr, allocation_len);
            }
            return Err(JitError::ExecutableMemory {
                detail: error.to_string().into(),
            });
        }

        if let Err(error) = sync_instruction_cache(ptr.cast::<u8>(), bytes.len()) {
            unsafe {
                munmap(allocation_ptr, allocation_len);
            }
            return Err(error);
        }

        Ok(Self {
            ptr: NonNull::new(ptr.cast::<u8>()).expect("mmap returned a checked pointer"),
            len: bytes.len(),
            allocation_ptr: NonNull::new(allocation_ptr.cast::<u8>())
                .expect("mmap returned a checked pointer"),
            allocation_len,
            #[cfg(target_arch = "aarch64")]
            unix_aarch64_unwind: None,
        })
    }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
impl ExecutableMemory {
    #[allow(dead_code)]
    pub(crate) fn allocate(bytes: &[u8]) -> JitResult<Self> {
        let arena = MacosJitArena::shared()?;
        let allocation = arena.publish(bytes)?;
        let ptr = unsafe { arena.executable_ptr.as_ptr().add(allocation.offset) };
        Ok(Self {
            ptr: NonNull::new(ptr).expect("arena allocation pointer remains non-null"),
            len: bytes.len(),
            _macos_allocation: allocation,
            unix_aarch64_unwind: None,
        })
    }
}

#[cfg(not(any(windows, unix)))]
impl ExecutableMemory {
    #[allow(dead_code)]
    pub(crate) fn allocate(_bytes: &[u8]) -> JitResult<Self> {
        Err(JitError::UnsupportedTarget {
            target: std::env::consts::OS.into(),
            reason: "native executable memory allocation is not implemented for this OS".into(),
        })
    }
}

#[cfg(windows)]
#[allow(dead_code)]
fn sync_instruction_cache(ptr: *const u8, len: usize) -> JitResult<()> {
    use windows_sys::Win32::Foundation::GetLastError;
    use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    let process = unsafe { GetCurrentProcess() };
    let ok = unsafe { FlushInstructionCache(process, ptr.cast(), len) };
    if ok == 0 {
        return Err(JitError::ExecutableMemory {
            detail: format!("FlushInstructionCache failed with error {}", unsafe {
                GetLastError()
            })
            .into(),
        });
    }

    Ok(())
}

#[cfg(all(unix, target_arch = "x86_64"))]
#[allow(dead_code)]
fn sync_instruction_cache(_ptr: *const u8, _len: usize) -> JitResult<()> {
    // x86_64 has coherent instruction and data caches for this generated-code path.
    Ok(())
}

#[cfg(all(unix, target_arch = "aarch64"))]
#[allow(dead_code)]
fn sync_instruction_cache(ptr: *const u8, len: usize) -> JitResult<()> {
    if ptr.is_null() || len == 0 {
        return Err(JitError::ExecutableMemory {
            detail: "instruction-cache synchronization requires a nonempty range".into(),
        });
    }
    unsafe {
        rspice_clear_instruction_cache(ptr.cast_mut().cast(), len);
    }
    Ok(())
}

#[cfg(all(unix, not(any(target_arch = "x86_64", target_arch = "aarch64"))))]
#[allow(dead_code)]
fn sync_instruction_cache(_ptr: *const u8, _len: usize) -> JitResult<()> {
    Err(JitError::ExecutableMemory {
        detail: format!(
            "instruction cache flush is not implemented for unix/{}",
            std::env::consts::ARCH
        )
        .into(),
    })
}

impl Drop for ExecutableMemory {
    fn drop(&mut self) {
        #[cfg(all(unix, target_arch = "aarch64"))]
        drop(self.unix_aarch64_unwind.take());

        #[cfg(all(windows, target_arch = "x86_64"))]
        unsafe {
            use windows_sys::Win32::System::Diagnostics::Debug::RtlDeleteFunctionTable;
            use windows_sys::Win32::System::Memory::{MEM_RELEASE, VirtualFree};

            if let Some(table) = self.windows_function_table.take() {
                let _ = RtlDeleteFunctionTable(table.as_ptr());
            }
            VirtualFree(self.allocation_ptr.as_ptr().cast(), 0, MEM_RELEASE);
        }

        #[cfg(all(windows, target_arch = "aarch64"))]
        unsafe {
            use windows_sys::Win32::System::Diagnostics::Debug::RtlDeleteFunctionTable;
            use windows_sys::Win32::System::Memory::{MEM_RELEASE, VirtualFree};

            if let Some(table) = self.windows_function_table.take() {
                let _ = RtlDeleteFunctionTable(table.as_ptr());
            }
            VirtualFree(self.allocation_ptr.as_ptr().cast(), 0, MEM_RELEASE);
        }

        #[cfg(all(unix, not(all(target_os = "macos", target_arch = "aarch64"))))]
        unsafe {
            libc::munmap(self.allocation_ptr.as_ptr().cast(), self.allocation_len);
        }
    }
}

// Safety: allocation publishes only immutable RX pages. `ptr_at` exposes
// const pointers, mutation is impossible after construction, and Drop needs
// exclusive ownership before unmapping.
unsafe impl Send for ExecutableMemory {}
unsafe impl Sync for ExecutableMemory {}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{
        ExecutableMemory, MAX_EXECUTABLE_IMAGE_BYTES, sync_instruction_cache, validate_image_len,
    };
    use crate::native::JitError;
    use crate::native::x64::encoder::X64Encoder;

    #[test]
    fn executable_memory_rejects_empty_image() {
        let error = match ExecutableMemory::allocate(&[]) {
            Ok(_) => panic!("empty image should fail"),
            Err(error) => error,
        };

        match error {
            JitError::ExecutableMemory { detail } => {
                assert_eq!(detail, "cannot allocate empty executable image");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn executable_memory_rejects_oversized_image() {
        let error = validate_image_len(MAX_EXECUTABLE_IMAGE_BYTES + 1)
            .expect_err("oversized image should fail");
        assert!(error.to_string().contains("safety limit"));
    }

    #[test]
    fn executable_memory_limit_covers_large_shipped_models() {
        validate_image_len(32 * 1024 * 1024)
            .expect("large shipped compact-model image must fit the safety limit");
    }

    #[test]
    fn executable_memory_runs_mov_eax_imm32_ret() {
        let mut enc = X64Encoder::new();
        enc.mov_eax_imm32(42);
        enc.ret();
        let memory =
            ExecutableMemory::allocate(&enc.into_bytes()).expect("allocate executable memory");
        assert!(!memory.is_empty());

        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn() -> u32 = unsafe { std::mem::transmute(entry) };
        assert_eq!(f(), 42);
    }

    #[test]
    fn executable_memory_owns_source_bytes_after_allocation() {
        let mut enc = X64Encoder::new();
        enc.mov_eax_imm32(42);
        enc.ret();
        let mut bytes = enc.into_bytes();
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate executable memory");

        bytes.fill(0xcc);

        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn() -> u32 = unsafe { std::mem::transmute(entry) };
        assert_eq!(f(), 42);
    }

    #[test]
    fn executable_memory_repeated_allocate_call_drop_stress() {
        for iteration in 0..2048u32 {
            let expected = iteration ^ 0x5a5a;
            let mut enc = X64Encoder::new();
            enc.mov_eax_imm32(expected);
            enc.ret();
            let memory =
                ExecutableMemory::allocate(&enc.into_bytes()).expect("allocate executable memory");

            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn() -> u32 = unsafe { std::mem::transmute(entry) };
            assert_eq!(f(), expected);
        }
    }

    #[test]
    fn instruction_cache_sync_accepts_executable_memory() {
        let mut enc = X64Encoder::new();
        enc.mov_eax_imm32(42);
        enc.ret();
        let memory =
            ExecutableMemory::allocate(&enc.into_bytes()).expect("allocate executable memory");

        let ptr = memory.ptr_at(0).expect("entry point inside image");
        sync_instruction_cache(ptr, memory.len())
            .expect("sync instruction cache for executable memory");
    }

    #[cfg(windows)]
    #[test]
    fn executable_memory_is_read_execute_and_not_writable() {
        use std::mem::{MaybeUninit, size_of};
        use windows_sys::Win32::System::Memory::{
            MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READ, VirtualQuery,
        };

        let mut enc = X64Encoder::new();
        enc.mov_eax_imm32(42);
        enc.ret();
        let memory =
            ExecutableMemory::allocate(&enc.into_bytes()).expect("allocate executable memory");
        let mut info = MaybeUninit::<MEMORY_BASIC_INFORMATION>::zeroed();
        let bytes = unsafe {
            VirtualQuery(
                memory.base_ptr().cast(),
                info.as_mut_ptr(),
                size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        assert_eq!(bytes, size_of::<MEMORY_BASIC_INFORMATION>());
        let info = unsafe { info.assume_init() };
        assert_eq!(
            info.Protect & 0xff,
            PAGE_EXECUTE_READ,
            "published native pages must be RX, never writable"
        );
    }

    #[cfg(windows)]
    #[test]
    fn executable_memory_has_no_access_guard_pages() {
        use std::mem::{MaybeUninit, size_of};
        use windows_sys::Win32::System::Memory::{
            MEM_RESERVE, MEMORY_BASIC_INFORMATION, PAGE_NOACCESS, VirtualQuery,
        };

        let memory = ExecutableMemory::allocate(&[0xC3]).expect("allocate guarded image");
        let mut leading = MaybeUninit::<MEMORY_BASIC_INFORMATION>::zeroed();
        let leading_bytes = unsafe {
            VirtualQuery(
                memory.allocation_ptr().cast(),
                leading.as_mut_ptr(),
                size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        assert_eq!(leading_bytes, size_of::<MEMORY_BASIC_INFORMATION>());
        let leading = unsafe { leading.assume_init() };
        assert_eq!(leading.State, MEM_RESERVE);
        assert_eq!(leading.AllocationProtect & 0xff, PAGE_NOACCESS);

        let trailing_ptr = unsafe { memory.allocation_ptr().add(memory.allocation_len() - 1) };
        let mut trailing = MaybeUninit::<MEMORY_BASIC_INFORMATION>::zeroed();
        let trailing_bytes = unsafe {
            VirtualQuery(
                trailing_ptr.cast(),
                trailing.as_mut_ptr(),
                size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        assert_eq!(trailing_bytes, size_of::<MEMORY_BASIC_INFORMATION>());
        let trailing = unsafe { trailing.assume_init() };
        assert_eq!(trailing.State, MEM_RESERVE);
        assert_eq!(trailing.AllocationProtect & 0xff, PAGE_NOACCESS);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn executable_memory_is_read_execute_and_not_writable() {
        let mut enc = X64Encoder::new();
        enc.mov_eax_imm32(42);
        enc.ret();
        let memory =
            ExecutableMemory::allocate(&enc.into_bytes()).expect("allocate executable memory");
        let address = memory.base_ptr() as usize;
        let maps = std::fs::read_to_string("/proc/self/maps").expect("read process mappings");
        let permissions = maps
            .lines()
            .find_map(|line| {
                let mut fields = line.split_whitespace();
                let range = fields.next()?;
                let permissions = fields.next()?;
                let (start, end) = range.split_once('-')?;
                let start = usize::from_str_radix(start, 16).ok()?;
                let end = usize::from_str_radix(end, 16).ok()?;
                (start <= address && address < end).then_some(permissions)
            })
            .expect("find executable-memory mapping");
        assert!(
            permissions.starts_with("r-x"),
            "published native pages must be RX, never writable; got {permissions}"
        );
    }
}
