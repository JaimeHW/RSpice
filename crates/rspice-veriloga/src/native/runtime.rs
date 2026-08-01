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

use super::{JitError, JitResult};
use std::ptr::{self, NonNull};

// PSP104 currently emits roughly 28 MiB. Retain bounded virtual-memory use
// while leaving headroom for larger commercial compact models and future
// lowering growth.
const MAX_EXECUTABLE_IMAGE_BYTES: usize = 256 * 1024 * 1024;

pub(crate) struct ExecutableMemory {
    ptr: NonNull<u8>,
    len: usize,
    allocation_ptr: NonNull<u8>,
    #[cfg_attr(windows, allow(dead_code))]
    allocation_len: usize,
    #[cfg(windows)]
    windows_function_table:
        Option<Box<[windows_sys::Win32::System::Diagnostics::Debug::IMAGE_RUNTIME_FUNCTION_ENTRY]>>,
}

#[cfg(windows)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct WindowsX64RuntimeFunction {
    pub(crate) begin_address: u32,
    pub(crate) end_address: u32,
    pub(crate) unwind_info_address: u32,
}

impl ExecutableMemory {
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
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

    #[cfg(test)]
    fn base_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    #[cfg(all(test, windows))]
    fn allocation_ptr(&self) -> *const u8 {
        self.allocation_ptr.as_ptr()
    }

    #[cfg(all(test, windows))]
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

#[cfg(windows)]
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

        Ok(Self {
            ptr: NonNull::new(ptr.cast::<u8>()).expect("VirtualAlloc returned a checked pointer"),
            len: bytes.len(),
            allocation_ptr: NonNull::new(allocation_ptr.cast::<u8>())
                .expect("VirtualAlloc reserve returned a checked pointer"),
            allocation_len,
            windows_function_table: None,
        })
    }
}

#[cfg(unix)]
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

#[cfg(all(unix, not(target_arch = "x86_64")))]
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
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Diagnostics::Debug::RtlDeleteFunctionTable;
            use windows_sys::Win32::System::Memory::{MEM_RELEASE, VirtualFree};

            if let Some(table) = self.windows_function_table.take() {
                let _ = RtlDeleteFunctionTable(table.as_ptr());
            }
            VirtualFree(self.allocation_ptr.as_ptr().cast(), 0, MEM_RELEASE);
        }

        #[cfg(unix)]
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
