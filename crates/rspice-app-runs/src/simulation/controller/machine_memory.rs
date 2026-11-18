//! The memory of the machine a run is prepared on, as its operating system
//! reports it.

use rspice_simulation::execution::preparation::MachineMemory;

/// The memory installed and free on this machine, or `None` where the host
/// cannot say: a run then keeps the counts this build ships.
pub(crate) fn machine_memory() -> Option<MachineMemory> {
    query()
}

#[cfg(windows)]
fn query() -> Option<MachineMemory> {
    use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    let mut status = MEMORYSTATUSEX {
        dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
        dwMemoryLoad: 0,
        ullTotalPhys: 0,
        ullAvailPhys: 0,
        ullTotalPageFile: 0,
        ullAvailPageFile: 0,
        ullTotalVirtual: 0,
        ullAvailVirtual: 0,
        ullAvailExtendedVirtual: 0,
    };
    // SAFETY: `status` is a live, writable MEMORYSTATUSEX whose `dwLength`
    // names its size, as the call requires.
    let answered = unsafe { GlobalMemoryStatusEx(&mut status) } != 0;
    answered.then_some(MachineMemory {
        installed_bytes: status.ullTotalPhys,
        free_bytes: status.ullAvailPhys,
    })
}

#[cfg(target_os = "linux")]
fn query() -> Option<MachineMemory> {
    let meminfo = std::fs::read_to_string("/proc/meminfo").ok()?;
    let kilobytes = |field: &str| {
        meminfo.lines().find_map(|line| {
            let value = line.strip_prefix(field)?.strip_prefix(':')?;
            value.trim().strip_suffix("kB")?.trim().parse::<u64>().ok()
        })
    };
    Some(MachineMemory {
        installed_bytes: kilobytes("MemTotal")? * 1024,
        free_bytes: kilobytes("MemAvailable")? * 1024,
    })
}

#[cfg(target_os = "macos")]
fn query() -> Option<MachineMemory> {
    let mut installed: u64 = 0;
    let mut size = std::mem::size_of::<u64>();
    // SAFETY: the name is NUL-terminated and `installed` is a writable u64
    // whose size `size` states, as `hw.memsize` answers.
    let answered = unsafe {
        libc::sysctlbyname(
            c"hw.memsize".as_ptr(),
            (&raw mut installed).cast(),
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    } == 0;
    if !answered {
        return None;
    }
    // SAFETY: an all-zero vm_statistics64 is a valid value of the plain
    // counters it holds.
    let mut statistics: libc::vm_statistics64 = unsafe { std::mem::zeroed() };
    let mut count = libc::HOST_VM_INFO64_COUNT;
    // SAFETY: `statistics` is writable and `count` states its size in
    // integers, as the HOST_VM_INFO64 flavour answers.
    #[allow(deprecated)]
    let answered = unsafe {
        libc::host_statistics64(
            libc::mach_host_self(),
            libc::HOST_VM_INFO64,
            (&raw mut statistics).cast(),
            &mut count,
        )
    } == libc::KERN_SUCCESS;
    // SAFETY: sysconf reads a constant of the running system.
    let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if !answered || page <= 0 {
        return None;
    }
    // Free pages and pages the system would reclaim first, as the system's
    // own memory pressure reading counts what is available.
    let pages = u64::from(statistics.free_count) + u64::from(statistics.inactive_count);
    Some(MachineMemory {
        installed_bytes: installed,
        free_bytes: pages * page as u64,
    })
}

#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
fn query() -> Option<MachineMemory> {
    None
}

#[cfg(all(test, any(windows, target_os = "linux", target_os = "macos")))]
mod tests {
    /// The desktop systems say how much memory there is, and never more
    /// free than installed.
    #[test]
    fn a_desktop_system_says_its_memory() {
        let memory = super::machine_memory().expect("the system says its memory");
        assert!(memory.installed_bytes >= 256 * 1024 * 1024, "{memory:?}");
        assert!(memory.free_bytes <= memory.installed_bytes, "{memory:?}");
    }
}
