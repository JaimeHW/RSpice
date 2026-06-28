use super::{JitError, JitResult};
use std::ptr;

pub struct ExecutableMemory {
    ptr: *mut u8,
    len: usize,
}

impl ExecutableMemory {
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr as *const u8
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[cfg(windows)]
impl ExecutableMemory {
    pub fn allocate(bytes: &[u8]) -> JitResult<Self> {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::System::Memory::{
            VirtualAlloc, VirtualFree, VirtualProtect, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE,
            PAGE_EXECUTE_READ, PAGE_READWRITE,
        };

        if bytes.is_empty() {
            return Err(JitError::ExecutableMemory {
                detail: "cannot allocate empty executable image".into(),
            });
        }

        let ptr = unsafe {
            VirtualAlloc(
                ptr::null_mut(),
                bytes.len(),
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };
        if ptr.is_null() {
            return Err(JitError::ExecutableMemory {
                detail: format!("VirtualAlloc failed with error {}", unsafe {
                    GetLastError()
                })
                .into(),
            });
        }

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.cast::<u8>(), bytes.len());
        }

        let mut old_protect = PAGE_READWRITE;
        let protect_ok =
            unsafe { VirtualProtect(ptr, bytes.len(), PAGE_EXECUTE_READ, &mut old_protect) };
        if protect_ok == 0 {
            let error = unsafe { GetLastError() };
            unsafe {
                VirtualFree(ptr, 0, MEM_RELEASE);
            }
            return Err(JitError::ExecutableMemory {
                detail: format!("VirtualProtect failed with error {error}").into(),
            });
        }

        if let Err(error) = sync_instruction_cache(ptr.cast::<u8>(), bytes.len()) {
            unsafe {
                VirtualFree(ptr, 0, MEM_RELEASE);
            }
            return Err(error);
        }

        Ok(Self {
            ptr: ptr.cast::<u8>(),
            len: bytes.len(),
        })
    }
}

#[cfg(unix)]
impl ExecutableMemory {
    pub fn allocate(bytes: &[u8]) -> JitResult<Self> {
        use libc::{
            mmap, mprotect, munmap, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_EXEC, PROT_READ,
            PROT_WRITE,
        };

        if bytes.is_empty() {
            return Err(JitError::ExecutableMemory {
                detail: "cannot allocate empty executable image".into(),
            });
        }

        let ptr = unsafe {
            mmap(
                ptr::null_mut(),
                bytes.len(),
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANON,
                -1,
                0,
            )
        };
        if ptr == MAP_FAILED {
            return Err(JitError::ExecutableMemory {
                detail: std::io::Error::last_os_error().to_string().into(),
            });
        }

        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.cast::<u8>(), bytes.len());
        }

        if unsafe { mprotect(ptr, bytes.len(), PROT_READ | PROT_EXEC) } != 0 {
            let error = std::io::Error::last_os_error();
            unsafe {
                munmap(ptr, bytes.len());
            }
            return Err(JitError::ExecutableMemory {
                detail: error.to_string().into(),
            });
        }

        if let Err(error) = sync_instruction_cache(ptr.cast::<u8>(), bytes.len()) {
            unsafe {
                munmap(ptr, bytes.len());
            }
            return Err(error);
        }

        Ok(Self {
            ptr: ptr.cast::<u8>(),
            len: bytes.len(),
        })
    }
}

#[cfg(not(any(windows, unix)))]
impl ExecutableMemory {
    pub fn allocate(_bytes: &[u8]) -> JitResult<Self> {
        Err(JitError::UnsupportedTarget {
            target: std::env::consts::OS.into(),
            reason: "native executable memory allocation is not implemented for this OS".into(),
        })
    }
}

#[cfg(windows)]
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
fn sync_instruction_cache(_ptr: *const u8, _len: usize) -> JitResult<()> {
    // x86_64 has coherent instruction and data caches for this generated-code path.
    Ok(())
}

#[cfg(all(unix, not(target_arch = "x86_64")))]
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
        if self.ptr.is_null() || self.len == 0 {
            return;
        }

        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::System::Memory::{VirtualFree, MEM_RELEASE};

            VirtualFree(self.ptr.cast(), 0, MEM_RELEASE);
        }

        #[cfg(unix)]
        unsafe {
            libc::munmap(self.ptr.cast(), self.len);
        }
    }
}

unsafe impl Send for ExecutableMemory {}
unsafe impl Sync for ExecutableMemory {}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{sync_instruction_cache, ExecutableMemory};
    use crate::native::x64::encoder::X64Encoder;
    use crate::native::JitError;

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
    fn executable_memory_runs_mov_eax_imm32_ret() {
        let mut enc = X64Encoder::new();
        enc.mov_eax_imm32(42);
        enc.ret();
        let memory =
            ExecutableMemory::allocate(&enc.into_bytes()).expect("allocate executable memory");
        assert!(!memory.is_empty());

        let f: extern "C" fn() -> u32 = unsafe { std::mem::transmute(memory.as_ptr()) };
        assert_eq!(f(), 42);
    }

    #[test]
    fn instruction_cache_sync_accepts_executable_memory() {
        let mut enc = X64Encoder::new();
        enc.mov_eax_imm32(42);
        enc.ret();
        let memory =
            ExecutableMemory::allocate(&enc.into_bytes()).expect("allocate executable memory");

        sync_instruction_cache(memory.as_ptr(), memory.len())
            .expect("sync instruction cache for executable memory");
    }
}
