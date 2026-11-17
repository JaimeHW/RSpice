//! Who has a project open, and whether this window may write it.
//!
//! A destination lease serializes one write. The open lease says who has the
//! project open: a small file beside the project whose first byte an open
//! window holds an exclusive lock on. The operating system lets the lock go
//! with the window, however the window ends, so a crashed window leaves no
//! stale lease. The holder's user, machine and opening time follow the locked
//! byte, where another window can read them while the lock is held.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest as _, Sha256};

use super::{
    create_unique_sibling, parent_directory, remove_transaction_temp, resolved_destination,
    try_lock_exclusive, unlock_file,
};

/// Where the holder's record starts: past the locked byte, so a window that
/// does not hold the lock can still read it.
const HOLDER_OFFSET: u64 = 1;
const HOLDER_RECORD_LEN: usize = 512;
const HOLDER_MAGIC: &str = "RSPICE-OPEN-V1";
/// The longest user or machine name a record keeps, in bytes.
const HOLDER_NAME_LEN: usize = 160;

/// Who holds a project's open lease, and since when.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenLeaseHolder {
    pub user: String,
    pub machine: String,
    /// When the holder opened the project, in milliseconds since the epoch.
    pub since_unix_ms: u64,
}

impl OpenLeaseHolder {
    /// The user and machine this process runs as, holding from `since_unix_ms`.
    pub fn this_process(since_unix_ms: u64) -> Self {
        Self {
            user: current_user(),
            machine: current_machine(),
            since_unix_ms,
        }
    }

    fn encode(&self) -> [u8; HOLDER_RECORD_LEN] {
        let text = format!(
            "{HOLDER_MAGIC}\n{}\n{}\n{}\n",
            record_name(&self.user),
            record_name(&self.machine),
            self.since_unix_ms
        );
        let mut record = [0_u8; HOLDER_RECORD_LEN];
        let bytes = text.as_bytes();
        let len = bytes.len().min(HOLDER_RECORD_LEN);
        record[..len].copy_from_slice(&bytes[..len]);
        record
    }

    fn decode(record: &[u8]) -> Option<Self> {
        let end = record
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(record.len());
        let text = std::str::from_utf8(&record[..end]).ok()?;
        let mut lines = text.split('\n');
        if lines.next()? != HOLDER_MAGIC {
            return None;
        }
        let user = lines.next()?.to_owned();
        let machine = lines.next()?.to_owned();
        let since_unix_ms = lines.next()?.parse().ok()?;
        Some(Self {
            user,
            machine,
            since_unix_ms,
        })
    }
}

/// A user or machine name as a record line holds it: no control characters,
/// and no longer than a record keeps.
fn record_name(name: &str) -> String {
    let mut kept = String::new();
    for character in name.chars().filter(|character| !character.is_control()) {
        if kept.len() + character.len_utf8() > HOLDER_NAME_LEN {
            break;
        }
        kept.push(character);
    }
    kept
}

fn current_user() -> String {
    ["USERNAME", "USER", "LOGNAME"]
        .iter()
        .find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()))
        .unwrap_or_default()
}

#[cfg(windows)]
fn current_machine() -> String {
    std::env::var("COMPUTERNAME").unwrap_or_default()
}

#[cfg(unix)]
fn current_machine() -> String {
    rustix::system::uname()
        .nodename()
        .to_string_lossy()
        .into_owned()
}

/// This window's hold on a project: released when dropped, or when the
/// process ends.
pub struct OpenLease {
    path: PathBuf,
    file: File,
}

impl OpenLease {
    /// The lease file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Whether this is the lease of the project at `project`.
    pub fn covers(&self, project: &Path) -> bool {
        open_lease_path(project) == self.path
    }
}

impl std::fmt::Debug for OpenLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenLease")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl Drop for OpenLease {
    fn drop(&mut self) {
        // The lease file stays, as the destination lease's does: removing it
        // after unlocking would race a window taking it.
        let _ = unlock_file(&self.file);
    }
}

/// What asking for a project's open lease found.
#[derive(Debug)]
pub enum OpenLeaseAttempt {
    /// This window holds the lease now.
    Held(OpenLease),
    /// Another window holds it: who, when its record can be read.
    HeldElsewhere(Option<OpenLeaseHolder>),
}

/// Take the open lease of the project at `project` for `holder`, or say who
/// has it.
pub fn acquire_open_lease(
    project: &Path,
    holder: &OpenLeaseHolder,
) -> io::Result<OpenLeaseAttempt> {
    let path = open_lease_path(project);
    let (mut file, writable) = match open_lease_file(&path, true) {
        Ok(file) => (file, true),
        // A lease another user created may be one this user can only read;
        // the lock is still the lock.
        Err(error) if error.kind() == io::ErrorKind::PermissionDenied && path.exists() => {
            (open_lease_file(&path, false)?, false)
        }
        Err(error) => return Err(error),
    };
    if let Err(error) = try_lock_exclusive(&file) {
        if error.kind() != io::ErrorKind::WouldBlock {
            return Err(error);
        }
        return Ok(OpenLeaseAttempt::HeldElsewhere(read_holder(&mut file)));
    }
    if writable {
        let record = holder.encode();
        let written = (|| {
            file.seek(SeekFrom::Start(HOLDER_OFFSET))?;
            file.write_all(&record)?;
            file.flush()
        })();
        if let Err(error) = written {
            let _ = unlock_file(&file);
            return Err(error);
        }
    }
    Ok(OpenLeaseAttempt::Held(OpenLease { path, file }))
}

/// Who holds the open lease of the project at `project`, if anyone does.
pub fn open_lease_holder(project: &Path) -> io::Result<Option<OpenLeaseHolder>> {
    let path = open_lease_path(project);
    let mut file = match open_lease_file(&path, false) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    match try_lock_exclusive(&file) {
        Ok(()) => {
            let _ = unlock_file(&file);
            Ok(None)
        }
        Err(error) if error.kind() == io::ErrorKind::WouldBlock => Ok(read_holder(&mut file)),
        Err(error) => Err(error),
    }
}

fn read_holder(file: &mut File) -> Option<OpenLeaseHolder> {
    let mut record = [0_u8; HOLDER_RECORD_LEN];
    file.seek(SeekFrom::Start(HOLDER_OFFSET)).ok()?;
    let mut read = 0;
    while read < record.len() {
        match file.read(&mut record[read..]) {
            Ok(0) => break,
            Ok(count) => read += count,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => return None,
        }
    }
    OpenLeaseHolder::decode(&record[..read])
}

/// Why this window cannot publish the project at `project`, if it cannot.
#[derive(Debug)]
pub enum WriteRefusal {
    /// The folder takes no new file, so no save can be published there.
    FolderNotWritable(io::Error),
    /// The project file is marked read-only.
    FileReadOnly,
}

/// Whether a save of the project at `project` could be published: a save
/// writes a new file beside it and moves it into place, so the folder must
/// take a new file and the file must not be marked read-only. The probe file
/// is removed at once.
pub fn write_refusal(project: &Path) -> io::Result<Option<WriteRefusal>> {
    let destination = resolved_destination(project);
    match create_unique_sibling(&destination) {
        Ok((probe, file)) => {
            drop(file);
            remove_transaction_temp(&probe)?;
        }
        Err(error) if is_not_writable(&error) => {
            return Ok(Some(WriteRefusal::FolderNotWritable(error)));
        }
        Err(error) => return Err(error),
    }
    match std::fs::metadata(&destination) {
        Ok(metadata) if metadata.permissions().readonly() => Ok(Some(WriteRefusal::FileReadOnly)),
        Ok(_) => Ok(None),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

/// Whether `error` says a place cannot be written, as a read-only share,
/// medium or folder answers.
pub fn is_not_writable(error: &io::Error) -> bool {
    matches!(
        error.kind(),
        io::ErrorKind::PermissionDenied | io::ErrorKind::ReadOnlyFilesystem
    )
}

fn open_lease_path(project: &Path) -> PathBuf {
    let destination = resolved_destination(project);
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-open-lease\0v1\0");
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;
        hasher.update(destination.as_os_str().as_bytes());
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;
        for unit in destination.as_os_str().encode_wide() {
            hasher.update(unit.to_le_bytes());
        }
    }
    let digest: [u8; 32] = hasher.finalize().into();
    let mut encoded = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(encoded, "{byte:02x}");
    }
    parent_directory(&destination).join(format!(".rspice-open-v1-{encoded}.lock"))
}

#[cfg(unix)]
fn open_lease_file(path: &Path, write: bool) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt as _;

    let mut options = OpenOptions::new();
    options
        .read(true)
        .custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
    if write {
        options.write(true).create(true).mode(0o644);
    }
    options.open(path)
}

#[cfg(windows)]
fn open_lease_file(path: &Path, write: bool) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt as _;
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
    };

    let mut options = OpenOptions::new();
    options
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    if write {
        options.write(true).create(true);
    }
    options.open(path)
}

/// A folder no new file can be created in for as long as the guard lives, as
/// a read-only share or medium is: files already in it stay readable.
#[cfg(any(test, feature = "test-support"))]
pub struct UnwritableFolder {
    path: PathBuf,
    #[cfg(windows)]
    original: String,
    #[cfg(unix)]
    original: u32,
}

#[cfg(any(test, feature = "test-support"))]
impl UnwritableFolder {
    pub fn deny_new_files(path: &Path) -> io::Result<Self> {
        #[cfg(windows)]
        {
            let original = super::windows_security_sddl(path)?;
            // Everyone is denied adding a file or folder here, and keeps full
            // access to what is already inside.
            set_windows_dacl(path, "D:(D;;0x6;;;WD)(A;OICI;FA;;;WD)")?;
            Ok(Self {
                path: path.to_path_buf(),
                original,
            })
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let original = std::fs::metadata(path)?.permissions().mode();
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o555))?;
            Ok(Self {
                path: path.to_path_buf(),
                original,
            })
        }
    }

    /// Whether this account can still create files here, as root can.
    pub fn still_writable(&self) -> bool {
        create_unique_sibling(&self.path.join("probe"))
            .and_then(|(probe, file)| {
                drop(file);
                remove_transaction_temp(&probe)
            })
            .is_ok()
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Drop for UnwritableFolder {
    fn drop(&mut self) {
        #[cfg(windows)]
        let _ = set_windows_dacl(&self.path, &self.original);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let _ = std::fs::set_permissions(
                &self.path,
                std::fs::Permissions::from_mode(self.original),
            );
        }
    }
}

#[cfg(all(windows, any(test, feature = "test-support")))]
fn set_windows_dacl(path: &Path, sddl: &str) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt as _;
    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Authorization::{
        ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
    };
    use windows_sys::Win32::Security::{DACL_SECURITY_INFORMATION, SetFileSecurityW};

    let text: Vec<u16> = sddl.encode_utf16().chain(std::iter::once(0)).collect();
    let target: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut descriptor = std::ptr::null_mut();
    let converted = unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            text.as_ptr(),
            SDDL_REVISION_1,
            &mut descriptor,
            std::ptr::null_mut(),
        )
    };
    if converted == 0 {
        return Err(io::Error::last_os_error());
    }
    let written =
        unsafe { SetFileSecurityW(target.as_ptr(), DACL_SECURITY_INFORMATION, descriptor) };
    let result = if written == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    };
    unsafe {
        LocalFree(descriptor.cast());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_that_takes_no_new_file_refuses_a_save() {
        let (folder, project) = project_in("folder");
        let denied = UnwritableFolder::deny_new_files(&folder).unwrap();
        if denied.still_writable() {
            // An account the folder's permissions do not bind, as root.
            return;
        }
        assert!(matches!(
            write_refusal(&project).unwrap(),
            Some(WriteRefusal::FolderNotWritable(_))
        ));
        assert_eq!(std::fs::read(&project).unwrap(), b"{}");
        drop(denied);
        assert!(write_refusal(&project).unwrap().is_none());
        let _ = std::fs::remove_dir_all(folder);
    }

    fn project_in(name: &str) -> (PathBuf, PathBuf) {
        let folder =
            std::env::temp_dir().join(format!("rspice-open-lease-{name}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&folder).unwrap();
        let project = folder.join("amp.rspiceproj");
        std::fs::write(&project, b"{}").unwrap();
        (folder, project)
    }

    fn holder(user: &str) -> OpenLeaseHolder {
        OpenLeaseHolder {
            user: user.to_owned(),
            machine: "BENCH-7".to_owned(),
            since_unix_ms: 1_763_400_000_000,
        }
    }

    #[test]
    fn a_second_window_is_told_who_has_the_project_and_since_when() {
        let (folder, project) = project_in("second");
        let first = acquire_open_lease(&project, &holder("ada")).unwrap();
        assert!(matches!(first, OpenLeaseAttempt::Held(_)));

        let second = acquire_open_lease(&project, &holder("grace")).unwrap();
        let OpenLeaseAttempt::HeldElsewhere(Some(found)) = second else {
            panic!("the second window takes the lease the first holds: {second:?}");
        };
        assert_eq!(found, holder("ada"));
        assert_eq!(open_lease_holder(&project).unwrap(), Some(holder("ada")));

        drop(first);
        assert_eq!(open_lease_holder(&project).unwrap(), None);
        let third = acquire_open_lease(&project, &holder("grace")).unwrap();
        assert!(matches!(third, OpenLeaseAttempt::Held(_)));
        assert_eq!(open_lease_holder(&project).unwrap(), Some(holder("grace")));
        drop(third);
        let _ = std::fs::remove_dir_all(folder);
    }

    #[test]
    fn a_name_with_line_breaks_cannot_forge_the_record() {
        let forged = OpenLeaseHolder {
            user: "ada\n0\nmallory".to_owned(),
            machine: "x".repeat(400),
            since_unix_ms: 7,
        };
        let decoded = OpenLeaseHolder::decode(&forged.encode()).unwrap();
        assert_eq!(decoded.user, "ada0mallory");
        assert_eq!(decoded.machine.len(), HOLDER_NAME_LEN);
        assert_eq!(decoded.since_unix_ms, 7);
    }

    #[test]
    fn a_read_only_file_refuses_a_save_and_a_writable_one_does_not() {
        let (folder, project) = project_in("readonly");
        assert!(write_refusal(&project).unwrap().is_none());
        let mut permissions = std::fs::metadata(&project).unwrap().permissions();
        permissions.set_readonly(true);
        std::fs::set_permissions(&project, permissions.clone()).unwrap();
        assert!(matches!(
            write_refusal(&project).unwrap(),
            Some(WriteRefusal::FileReadOnly)
        ));
        #[allow(clippy::permissions_set_readonly_false)]
        permissions.set_readonly(false);
        std::fs::set_permissions(&project, permissions).unwrap();
        let leftovers = std::fs::read_dir(&folder).unwrap().count();
        assert_eq!(leftovers, 1, "the probe leaves nothing beside the project");
        let _ = std::fs::remove_dir_all(folder);
    }
}
