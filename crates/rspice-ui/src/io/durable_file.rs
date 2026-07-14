//! Crash-consistent native file publication.
//!
//! A successful return is the durability boundary used by primary project,
//! schematic, and recovery files: bytes are written to a unique sibling,
//! synchronized, atomically published, and the containing directory is then
//! synchronized on platforms that require it.

#![cfg(not(target_arch = "wasm32"))]

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use uuid::Uuid;

const UNIQUE_TEMP_ATTEMPTS: usize = 16;

/// Publish a file without exposing partially serialized bytes at `path`.
///
/// The callback may stream arbitrarily large payloads into the temporary file.
/// If it or any durability step fails, the previous target remains in place
/// whenever publication has not already completed, and callers receive an
/// error so they can retain dirty state and recovery evidence.
pub(crate) fn atomic_write_with<E>(
    path: &Path,
    write: impl FnOnce(&mut File) -> Result<(), E>,
) -> Result<(), E>
where
    E: From<io::Error>,
{
    let parent = parent_directory(path);
    std::fs::create_dir_all(parent).map_err(E::from)?;
    let predecessor_permissions = std::fs::metadata(path)
        .ok()
        .map(|metadata| metadata.permissions());

    let (temp, mut file) = create_unique_sibling(path).map_err(E::from)?;

    let staged = (|| {
        write(&mut file)?;
        file.flush().map_err(E::from)?;
        if let Some(permissions) = predecessor_permissions {
            file.set_permissions(permissions).map_err(E::from)?;
        }
        file.sync_all().map_err(E::from)?;
        Ok(())
    })();
    drop(file);

    if let Err(error) = staged {
        remove_unpublished_temp(&temp);
        return Err(error);
    }

    if let Err(error) = atomic_replace(&temp, path).map_err(E::from) {
        remove_unpublished_temp(&temp);
        return Err(error);
    }

    sync_parent_directory(parent).map_err(|error| {
        E::from(io::Error::new(
            error.kind(),
            format!(
                "'{}' was atomically published, but its containing directory could not be synchronized; publication may already have occurred and durability is uncertain: {error}",
                path.display()
            ),
        ))
    })
}

pub(crate) fn atomic_write_bytes(path: &Path, bytes: &[u8]) -> io::Result<()> {
    atomic_write_with(path, |file| file.write_all(bytes))
}

/// Copy `source` to `target` through the same durable publication boundary.
/// An existing target is never truncated in place.
pub(crate) fn atomic_copy(source: &Path, target: &Path) -> io::Result<()> {
    let mut source = File::open(source)?;
    atomic_write_with(target, |target| {
        std::io::copy(&mut source, target)?;
        Ok(())
    })
}

/// Persist a successful metadata-only transaction such as recovery deletion.
pub(crate) fn sync_parent_of(path: &Path) -> io::Result<()> {
    sync_parent_directory(parent_directory(path))
}

fn parent_directory(path: &Path) -> &Path {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn unique_sibling(path: &Path) -> PathBuf {
    let mut value = path.as_os_str().to_owned();
    value.push(format!(".{}.tmp", Uuid::new_v4()));
    PathBuf::from(value)
}

fn create_unique_sibling(path: &Path) -> io::Result<(PathBuf, File)> {
    for _ in 0..UNIQUE_TEMP_ATTEMPTS {
        let candidate = unique_sibling(path);
        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&candidate)
        {
            Ok(file) => return Ok((candidate, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        format!(
            "could not allocate a unique sibling transaction for '{}' after {UNIQUE_TEMP_ATTEMPTS} attempts",
            path.display()
        ),
    ))
}

fn remove_unpublished_temp(path: &Path) {
    if std::fs::remove_file(path).is_ok() {
        let _ = sync_parent_of(path);
    }
}

#[cfg(windows)]
fn atomic_replace(from: &Path, to: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt as _;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_WRITE_THROUGH, MoveFileExW, REPLACEFILE_WRITE_THROUGH, ReplaceFileW,
    };

    let target_exists = to.exists();
    let from = from
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let to = to
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let replaced = if target_exists {
        // ReplaceFile preserves the destination's ACL, ownership-related
        // metadata, compression/encryption attributes, and creation identity.
        unsafe {
            ReplaceFileW(
                to.as_ptr(),
                from.as_ptr(),
                std::ptr::null(),
                REPLACEFILE_WRITE_THROUGH,
                std::ptr::null(),
                std::ptr::null(),
            )
        }
    } else {
        // Do not pass REPLACE_EXISTING for a create. If another actor publishes
        // this name after our existence check, fail instead of overwriting it.
        unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), MOVEFILE_WRITE_THROUGH) }
    };
    if replaced == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(not(windows))]
fn atomic_replace(from: &Path, to: &Path) -> io::Result<()> {
    std::fs::rename(from, to)
}

#[cfg(unix)]
fn sync_parent_directory(parent: &Path) -> io::Result<()> {
    File::open(parent)?.sync_all()
}

#[cfg(not(unix))]
fn sync_parent_directory(_parent: &Path) -> io::Result<()> {
    // MOVEFILE_WRITE_THROUGH supplies the Windows publication boundary. Other
    // supported non-Unix targets currently use the same native path.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_staging_preserves_target_and_removes_unique_temp() {
        let root = unique_temp_dir("failed-stage");
        let target = root.join("state.json");
        std::fs::write(&target, b"durable predecessor").expect("write predecessor");

        let result = atomic_write_with::<io::Error>(&target, |file| {
            file.write_all(b"partial successor")?;
            Err(io::Error::other("injected serializer failure"))
        });

        assert!(result.is_err());
        assert_eq!(
            std::fs::read(&target).expect("read predecessor"),
            b"durable predecessor"
        );
        let entries = std::fs::read_dir(&root)
            .expect("read fixture")
            .map(|entry| entry.expect("entry").path())
            .collect::<Vec<_>>();
        assert_eq!(entries, vec![target]);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn successful_publication_replaces_target_without_fixed_temp_names() {
        let root = unique_temp_dir("success");
        let target = root.join("state.json");
        std::fs::write(&target, b"old").expect("write predecessor");

        atomic_write_bytes(&target, b"new durable bytes").expect("publish successor");

        assert_eq!(
            std::fs::read(&target).expect("read successor"),
            b"new durable bytes"
        );
        assert_eq!(std::fs::read_dir(&root).expect("read fixture").count(), 1);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn replacement_preserves_existing_unix_mode() {
        use std::os::unix::fs::PermissionsExt as _;

        let root = unique_temp_dir("permissions");
        let target = root.join("state.json");
        std::fs::write(&target, b"old").expect("write predecessor");
        std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o640))
            .expect("set predecessor mode");

        atomic_write_bytes(&target, b"new").expect("publish successor");

        assert_eq!(
            std::fs::metadata(&target)
                .expect("read metadata")
                .permissions()
                .mode()
                & 0o777,
            0o640
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-durable-file-{label}-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }
}
