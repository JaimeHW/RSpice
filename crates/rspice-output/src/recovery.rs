//! Controlled recovery of staging files left behind by a crashed process.
//!
//! A staging file is removed by the writer that created it — on success, on
//! failure, and on cancellation. Only a process that dies without unwinding
//! (a kill, a power loss, a panic that aborts) can leave one behind, and such
//! a file is invisible in a directory listing on Unix but not on Windows, so
//! it accumulates in a user's result directory.
//!
//! [`recover_stale_artifacts`] is the controlled pass that removes them. It
//! is deliberately conservative:
//!
//! * It examines one directory, without recursion, and stops after
//!   [`MAX_RECOVERY_ENTRIES`] entries so a huge shared directory cannot turn
//!   an export into a directory walk.
//! * It removes a staging file only when the process id embedded in its name
//!   belongs to no live process. A stage owned by this process, by a live
//!   process, or by a process whose liveness the host cannot answer is left
//!   alone and reported instead.
//! * It never removes a predecessor snapshot
//!   ([`PREDECESSOR_MARKER`](crate::PREDECESSOR_MARKER)): an interrupted set
//!   commit can leave one holding the only copy of a published result. Those
//!   are reported so an operator can decide.
//!
//! The caller is expected to log the returned report.

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

use crate::{PREDECESSOR_MARKER, STAGING_MARKER};

/// Largest number of directory entries one recovery pass inspects.
pub const MAX_RECOVERY_ENTRIES: usize = 65_536;

/// What one [`recover_stale_artifacts`] pass did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StagingRecovery {
    /// Staging files of dead processes that were removed.
    pub removed: Vec<PathBuf>,
    /// Staging files left alone because their writer may still be running.
    pub retained_live: Vec<PathBuf>,
    /// Predecessor snapshots, which this pass never removes.
    pub retained_predecessors: Vec<PathBuf>,
    /// Directory entries inspected.
    pub examined: usize,
    /// Whether [`MAX_RECOVERY_ENTRIES`] stopped the pass early.
    pub bounded: bool,
}

impl StagingRecovery {
    /// Whether the pass changed anything.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty()
    }

    /// One line naming what was removed and what was kept, for a log.
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "removed {} stale staging file(s); kept {} owned by a live or unknown process and {} predecessor snapshot(s) after inspecting {} entr{}{}",
            self.removed.len(),
            self.retained_live.len(),
            self.retained_predecessors.len(),
            self.examined,
            if self.examined == 1 { "y" } else { "ies" },
            if self.bounded {
                format!(" (stopped at the {MAX_RECOVERY_ENTRIES}-entry bound)")
            } else {
                String::new()
            }
        )
    }
}

/// Run a controlled recovery pass over one artifact directory.
///
/// A missing directory is not an error: an export creates its destination
/// directory later, and there is nothing to recover in one that does not
/// exist yet.
pub fn recover_stale_artifacts(directory: &Path) -> io::Result<StagingRecovery> {
    let mut report = StagingRecovery::default();
    let entries = match std::fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(report),
        Err(error) => return Err(error),
    };

    for entry in entries {
        if report.examined >= MAX_RECOVERY_ENTRIES {
            report.bounded = true;
            break;
        }
        let entry = entry?;
        report.examined += 1;
        let name = entry.file_name();
        if contains(&name, PREDECESSOR_MARKER) {
            report.retained_predecessors.push(entry.path());
            continue;
        }
        let Some(owner) = staging_owner(&name) else {
            continue;
        };
        if owner == std::process::id() || process_liveness(owner) != Some(false) {
            report.retained_live.push(entry.path());
            continue;
        }
        match std::fs::remove_file(entry.path()) {
            Ok(()) => report.removed.push(entry.path()),
            // A concurrent recovery pass removing the same abandoned file is
            // the expected race, not a failure of this one.
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(report)
}

/// Process id recorded in a staging file name, when the name is one.
///
/// Names are `.<destination><STAGING_MARKER><pid>-<serial>.tmp`.
fn staging_owner(name: &OsStr) -> Option<u32> {
    let name = name.to_str()?;
    if !name.starts_with('.') || !name.ends_with(".tmp") {
        return None;
    }
    let tail = name.split(STAGING_MARKER).nth(1)?;
    tail.split('-').next()?.parse().ok()
}

fn contains(name: &OsStr, marker: &str) -> bool {
    name.as_encoded_bytes()
        .windows(marker.len())
        .any(|window| window == marker.as_bytes())
}

/// `Some(true)` when the process is running, `Some(false)` when it provably
/// is not, and `None` when this host cannot answer.
#[cfg(windows)]
fn process_liveness(process_id: u32) -> Option<bool> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    };

    /// `GetExitCodeProcess` reports this while the process is still running.
    const STILL_ACTIVE: u32 = 259;
    const ERROR_INVALID_PARAMETER: i32 = 87;

    // SAFETY: `OpenProcess` takes no pointers and returns either a handle
    // this function closes or a null handle.
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process_id) };
    if handle.is_null() {
        // Only "no such process" proves the owner is gone; access denied
        // means it exists and belongs to somebody else.
        return match io::Error::last_os_error().raw_os_error() {
            Some(ERROR_INVALID_PARAMETER) => Some(false),
            _ => None,
        };
    }

    let mut exit_code: u32 = 0;
    // SAFETY: `handle` is a live process handle and `exit_code` is a valid
    // writable `u32` for the duration of the call.
    let queried = unsafe { GetExitCodeProcess(handle, &raw mut exit_code) };
    // SAFETY: `handle` came from `OpenProcess` and is closed exactly once.
    unsafe {
        CloseHandle(handle);
    }
    if queried == 0 {
        return None;
    }
    Some(exit_code == STILL_ACTIVE)
}

/// Every Unix answers this with `kill(pid, 0)`: signal 0 is never delivered,
/// and the call reports only whether a process with that id exists. `ESRCH` is
/// the one answer that proves the owner gone; `EPERM` means it exists and
/// belongs to somebody else.
///
/// This used to read `/proc`, which meant only Linux could answer: macOS has no
/// `/proc`, so every host but Linux and Windows reported `None` and recovery
/// kept a killed run's staging files forever. `kill` is POSIX and needs no
/// dependency — the symbol is declared here rather than pulled in with `libc`
/// because a new dependency edits `Cargo.lock`, which is a source-digest input
/// of the Verilog-A built-ins generator and would force a restamp of the whole
/// generated model tree for an unrelated fix.
#[cfg(unix)]
fn process_liveness(process_id: u32) -> Option<bool> {
    /// `kill` failed because the caller may not signal that process, which
    /// means the process exists.
    const EPERM: i32 = 1;
    /// `kill` failed because no such process exists.
    const ESRCH: i32 = 3;

    unsafe extern "C" {
        /// `int kill(pid_t pid, int sig)`. `pid_t` is `i32` on every Unix
        /// target Rust ships.
        fn kill(pid: i32, sig: i32) -> i32;
    }

    // A recorded id that does not fit `pid_t`, or that is zero or negative (a
    // process group rather than a process), was never handed out by this host,
    // and asking `kill` about it would address the wrong thing. Answer `None`
    // so recovery keeps the file instead.
    let pid = i32::try_from(process_id).ok().filter(|pid| *pid > 0)?;
    // SAFETY: `kill` takes two integers and no pointers, and signal 0 is
    // delivered to nothing; the call only reports whether `pid` names an
    // existing process.
    if unsafe { kill(pid, 0) } == 0 {
        return Some(true);
    }
    match io::Error::last_os_error().raw_os_error() {
        Some(ESRCH) => Some(false),
        Some(EPERM) => Some(true),
        _ => None,
    }
}

/// A host that is neither Windows nor Unix — `wasm32-unknown-unknown` is the
/// one this workspace builds — cannot be asked about process ids at all, so
/// recovery keeps every staging file it finds.
#[cfg(not(any(windows, unix)))]
fn process_liveness(_process_id: u32) -> Option<bool> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests_support::TestDirectory;

    /// A process id that cannot be running: id 0 is never a user process on
    /// Windows and is the scheduler on Linux, and neither is openable as a
    /// normal process. Use a high, unallocated id instead.
    fn dead_process_id() -> u32 {
        let mut candidate = 4_000_000;
        while process_liveness(candidate) != Some(false) {
            candidate += 2;
            assert!(candidate < 4_001_000, "no unused process id was found");
        }
        candidate
    }

    fn write(path: &Path, contents: &str) {
        std::fs::write(path, contents).expect("write recovery fixture");
    }

    /// The property every removal in this module rests on: this host can tell
    /// a live process from a reaped one.
    ///
    /// `recover_stale_artifacts` removes a staging file only on `Some(false)`,
    /// so a host that answers `None` reclaims nothing — which is what macOS did
    /// until the `kill(pid, 0)` arm existed, leaving a killed sweep's stages in
    /// the user's result directory for every later run to skip. Stating it here
    /// runs it wherever this crate's unit tests run, rather than only where an
    /// end-to-end kill test does.
    #[cfg(any(windows, unix))]
    #[test]
    fn this_host_proves_a_live_process_alive_and_a_reaped_one_dead() {
        assert_eq!(
            process_liveness(std::process::id()),
            Some(true),
            "this process is running"
        );

        // `--list` makes the test binary print its test names and exit, which
        // is the cheapest child this crate can be sure exists.
        let mut child =
            std::process::Command::new(std::env::current_exe().expect("this test binary's path"))
                .arg("--list")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("spawn the test binary");
        let owner = child.id();
        child.wait().expect("reap the child");

        // `child` is deliberately still alive here: on Windows its open handle
        // is what keeps the id from being recycled under the assertion.
        assert_eq!(
            process_liveness(owner),
            Some(false),
            "a reaped child is provably gone"
        );
        drop(child);
    }

    #[test]
    fn a_dead_writers_stage_is_removed_and_a_live_one_is_kept() {
        let directory = TestDirectory::new("recovery");
        let dead = directory.path().join(format!(
            ".result.csv{STAGING_MARKER}{}-7.tmp",
            dead_process_id()
        ));
        let live = directory.path().join(format!(
            ".result.csv{STAGING_MARKER}{}-8.tmp",
            std::process::id()
        ));
        let snapshot = directory.path().join(format!(
            ".result.csv{PREDECESSOR_MARKER}{}-9.bak",
            dead_process_id()
        ));
        let published = directory.path().join("result.csv");
        write(&dead, "abandoned");
        write(&live, "in flight");
        write(&snapshot, "predecessor");
        write(&published, "published");

        let report = recover_stale_artifacts(directory.path()).expect("run recovery pass");

        assert_eq!(report.removed, vec![dead.clone()]);
        assert_eq!(report.retained_live, vec![live.clone()]);
        assert_eq!(report.retained_predecessors, vec![snapshot.clone()]);
        assert_eq!(report.examined, 4);
        assert!(!report.bounded);
        assert!(!report.is_empty());
        assert!(!dead.exists());
        assert!(live.exists());
        assert_eq!(
            std::fs::read(&snapshot).expect("read retained snapshot"),
            b"predecessor"
        );
        assert_eq!(
            std::fs::read(&published).expect("read published artifact"),
            b"published"
        );
    }

    #[test]
    fn a_directory_without_stages_reports_nothing_removed() {
        let directory = TestDirectory::new("recovery-clean");
        write(&directory.destination(), "published");

        let report = recover_stale_artifacts(directory.path()).expect("run recovery pass");

        assert!(report.is_empty());
        assert_eq!(report.examined, 1);
        assert!(report.summary().contains("removed 0 stale staging file(s)"));
    }

    #[test]
    fn a_missing_directory_is_not_a_recovery_failure() {
        let directory = TestDirectory::new("recovery-missing");
        let absent = directory.path().join("not-created-yet");

        let report = recover_stale_artifacts(&absent).expect("missing directory is not an error");

        assert_eq!(report, StagingRecovery::default());
    }

    #[test]
    fn only_staging_names_are_parsed_for_an_owner() {
        assert_eq!(
            staging_owner(OsStr::new(&format!(".out.csv{STAGING_MARKER}1234-0.tmp"))),
            Some(1234)
        );
        assert_eq!(staging_owner(OsStr::new("out.csv")), None);
        assert_eq!(
            staging_owner(OsStr::new(&format!(".out.csv{STAGING_MARKER}1234-0.bak"))),
            None
        );
        assert_eq!(staging_owner(OsStr::new(".out.csv.other-1234-0.tmp")), None);
    }
}
