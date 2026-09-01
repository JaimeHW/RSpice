//! Transactional publication of single-file CLI artifacts.
//!
//! Writers always target a uniquely named file beside the destination. The
//! staged file is flushed and synchronized before an atomic, platform-correct
//! replace publishes it. Until that final operation succeeds, an existing
//! destination remains untouched.

use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use thiserror::Error;

const TEMP_MARKER: &str = ".rspice-tmp-";
const MAX_TEMP_ATTEMPTS: u64 = 128;
static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

/// Failure phase for a transactional artifact write.
#[derive(Debug, Error)]
pub(crate) enum AtomicArtifactError<E>
where
    E: std::error::Error + 'static,
{
    #[error("failed to prepare staged artifact: {0}")]
    Preparation(#[source] io::Error),
    #[error("failed while writing staged artifact: {0}")]
    Write(#[source] E),
    #[error("failed to flush staged artifact: {0}")]
    Flush(#[source] io::Error),
    #[error("failed to atomically commit staged artifact: {0}")]
    Commit(#[source] io::Error),
}

/// Write and durably publish one artifact without exposing partial output.
pub(crate) fn write_atomic<E, F>(destination: &Path, write: F) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut File) -> Result<(), E>,
{
    write_atomic_impl(
        destination,
        write,
        #[cfg(test)]
        None,
    )
}

fn write_atomic_impl<E, F>(
    destination: &Path,
    write: F,
    #[cfg(test)] fault: Option<TestFault>,
) -> Result<(), AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
    F: FnOnce(&mut File) -> Result<(), E>,
{
    reject_symlink_destination(destination).map_err(AtomicArtifactError::Preparation)?;
    let (temporary_path, mut temporary_file) =
        create_staging_file(destination).map_err(AtomicArtifactError::Preparation)?;
    let mut cleanup = StagingCleanup::new(temporary_path.clone());

    if let Err(error) = write(&mut temporary_file) {
        drop(temporary_file);
        cleanup.remove_now();
        return Err(AtomicArtifactError::Write(error));
    }

    #[cfg(test)]
    if let Err(error) = inject_fault(fault, TestFault::Flush) {
        drop(temporary_file);
        cleanup.remove_now();
        return Err(AtomicArtifactError::Flush(error));
    }

    if let Err(error) = temporary_file
        .flush()
        .and_then(|()| temporary_file.sync_all())
    {
        drop(temporary_file);
        cleanup.remove_now();
        return Err(AtomicArtifactError::Flush(error));
    }
    drop(temporary_file);

    #[cfg(test)]
    inject_fault(fault, TestFault::BeforeCommit).map_err(AtomicArtifactError::Commit)?;

    reject_symlink_destination(destination).map_err(AtomicArtifactError::Commit)?;
    if let Err(error) = commit_staging_file(&temporary_path, destination) {
        cleanup.remove_now();
        return Err(AtomicArtifactError::Commit(error));
    }
    cleanup.disarm();

    #[cfg(unix)]
    sync_parent_directory(destination).map_err(AtomicArtifactError::Commit)?;

    Ok(())
}

fn create_staging_file(destination: &Path) -> io::Result<(PathBuf, File)> {
    let file_name = destination.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "output destination must name a file",
        )
    })?;
    let parent = destination_parent(destination);
    let process_id = std::process::id();

    for _ in 0..MAX_TEMP_ATTEMPTS {
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let mut staging_name = OsString::from(".");
        staging_name.push(file_name);
        staging_name.push(format!("{TEMP_MARKER}{process_id}-{id}"));
        let staging_path = parent.join(staging_name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&staging_path)
        {
            Ok(file) => return Ok((staging_path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        format!("could not allocate a unique staging file beside {destination:?}"),
    ))
}

fn destination_parent(destination: &Path) -> &Path {
    match destination.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    }
}

fn reject_symlink_destination(destination: &Path) -> io::Result<()> {
    match std::fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "refusing to replace symlink output destination {}",
                destination.display()
            ),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(not(windows))]
fn commit_staging_file(staging: &Path, destination: &Path) -> io::Result<()> {
    std::fs::rename(staging, destination)
}

#[cfg(windows)]
fn commit_staging_file(staging: &Path, destination: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    fn wide_path(path: &Path) -> io::Result<Vec<u16>> {
        let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
        if wide.contains(&0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "output path contains an embedded NUL",
            ));
        }
        wide.push(0);
        Ok(wide)
    }

    let staging = wide_path(staging)?;
    let destination_wide = wide_path(destination)?;

    // SAFETY: Both path buffers are NUL-terminated and remain alive for the
    // duration of the Win32 call. Staging beside the destination guarantees a
    // same-volume rename, so COPY_ALLOWED is deliberately not requested.
    let succeeded = unsafe {
        MoveFileExW(
            staging.as_ptr(),
            destination_wide.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };

    if succeeded == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(unix)]
fn sync_parent_directory(destination: &Path) -> io::Result<()> {
    File::open(destination_parent(destination))?.sync_all()
}

struct StagingCleanup {
    path: Option<PathBuf>,
}

impl StagingCleanup {
    fn new(path: PathBuf) -> Self {
        Self { path: Some(path) }
    }

    fn remove_now(&mut self) {
        if let Some(path) = self.path.as_ref() {
            match std::fs::remove_file(path) {
                Ok(()) => self.path = None,
                Err(error) if error.kind() == io::ErrorKind::NotFound => self.path = None,
                Err(_) => {}
            }
        }
    }

    fn disarm(&mut self) {
        self.path = None;
    }
}

impl Drop for StagingCleanup {
    fn drop(&mut self) {
        self.remove_now();
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TestFault {
    Flush,
    BeforeCommit,
}

#[cfg(test)]
fn inject_fault(selected: Option<TestFault>, current: TestFault) -> io::Result<()> {
    if selected == Some(current) {
        Err(io::Error::other(format!(
            "injected {} failure",
            match current {
                TestFault::Flush => "flush",
                TestFault::BeforeCommit => "pre-commit",
            }
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let id = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "rspice-atomic-artifact-{}-{id}-{tag}",
                std::process::id()
            ));
            std::fs::create_dir(&path).expect("create unique atomic-artifact test directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn assert_no_staging_files(directory: &Path) {
        let staging_files: Vec<PathBuf> = std::fs::read_dir(directory)
            .expect("read test directory")
            .map(|entry| entry.expect("read directory entry").path())
            .filter(|path| path.to_string_lossy().contains(TEMP_MARKER))
            .collect();
        assert!(
            staging_files.is_empty(),
            "staging files were not cleaned: {staging_files:?}"
        );
    }

    fn assert_old_or_absent(destination: &Path, preexisting: bool) {
        if preexisting {
            assert_eq!(
                std::fs::read(destination).expect("read preserved destination"),
                b"old complete artifact"
            );
        } else {
            assert!(!destination.exists(), "failed write published an artifact");
        }
    }

    #[test]
    fn write_failures_after_header_and_mid_values_preserve_destination() {
        for preexisting in [false, true] {
            for (tag, partial) in [
                ("after-header", b"header\n".as_slice()),
                ("mid-values", b"header\n1,2\n3,".as_slice()),
            ] {
                let directory = TestDirectory::new(tag);
                let destination = directory.path().join("result.csv");
                if preexisting {
                    std::fs::write(&destination, b"old complete artifact")
                        .expect("seed existing destination");
                }

                let error = write_atomic_impl(
                    &destination,
                    |file| -> io::Result<()> {
                        file.write_all(partial)?;
                        Err(io::Error::other("injected serialization failure"))
                    },
                    None,
                )
                .expect_err("injected writer failure must propagate");

                assert!(matches!(error, AtomicArtifactError::Write(_)));
                assert_old_or_absent(&destination, preexisting);
                assert_no_staging_files(directory.path());
            }
        }
    }

    #[test]
    fn flush_and_precommit_failures_preserve_destination() {
        for preexisting in [false, true] {
            for fault in [TestFault::Flush, TestFault::BeforeCommit] {
                let directory = TestDirectory::new(match fault {
                    TestFault::Flush => "flush",
                    TestFault::BeforeCommit => "before-commit",
                });
                let destination = directory.path().join("result.raw");
                if preexisting {
                    std::fs::write(&destination, b"old complete artifact")
                        .expect("seed existing destination");
                }

                let error = write_atomic_impl(
                    &destination,
                    |file| file.write_all(b"new complete artifact"),
                    Some(fault),
                )
                .expect_err("injected publication failure must propagate");

                match fault {
                    TestFault::Flush => {
                        assert!(matches!(error, AtomicArtifactError::Flush(_)));
                    }
                    TestFault::BeforeCommit => {
                        assert!(matches!(error, AtomicArtifactError::Commit(_)));
                    }
                }
                assert_old_or_absent(&destination, preexisting);
                assert_no_staging_files(directory.path());
            }
        }
    }

    #[test]
    fn successful_write_publishes_only_the_complete_artifact() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("success");
            let destination = directory.path().join("result.json");
            if preexisting {
                std::fs::write(&destination, b"old complete artifact")
                    .expect("seed existing destination");
            }

            write_atomic(&destination, |file| {
                file.write_all(b"new complete artifact")
            })
            .expect("atomic artifact write succeeds");

            assert_eq!(
                std::fs::read(&destination).expect("read committed destination"),
                b"new complete artifact"
            );
            assert_no_staging_files(directory.path());
        }
    }

    #[test]
    fn bare_filename_stages_and_syncs_in_the_current_directory() {
        assert_eq!(destination_parent(Path::new("result.csv")), Path::new("."));
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn symlink_destination_is_rejected_without_modifying_its_target() {
        let directory = TestDirectory::new("symlink");
        let target = directory.path().join("target.csv");
        let destination = directory.path().join("result.csv");
        std::fs::write(&target, b"symlink target").expect("write symlink target");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &destination).expect("create file symlink");
        #[cfg(windows)]
        if let Err(error) = std::os::windows::fs::symlink_file(&target, &destination) {
            // Windows returns ERROR_PRIVILEGE_NOT_HELD when Developer Mode is
            // disabled, which is not consistently classified as
            // `PermissionDenied` by every supported toolchain.
            if error.kind() == io::ErrorKind::PermissionDenied || error.raw_os_error() == Some(1314)
            {
                return;
            }
            panic!("create file symlink: {error}");
        }

        let error = write_atomic(&destination, |file| file.write_all(b"replacement"))
            .expect_err("symlink output must be rejected");

        assert!(matches!(error, AtomicArtifactError::Preparation(_)));
        assert_eq!(
            std::fs::read(&target).expect("read symlink target"),
            b"symlink target"
        );
        assert!(destination.is_symlink());
        assert_no_staging_files(directory.path());
    }
}
