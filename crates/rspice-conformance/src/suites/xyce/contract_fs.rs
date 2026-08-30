//! Descriptor-anchored filesystem access for provenance-sensitive contracts.

use super::*;
use std::io::Read as _;

#[derive(Debug)]
pub(super) struct AnchoredDirectory {
    path: PathBuf,
    // Windows retains one no-write/no-delete-sharing handle for every resolved
    // component. Unix uses the terminal handle for descriptor-relative
    // enumeration and openat traversal and retains the chain explicitly.
    guards: Vec<fs::File>,
}

#[derive(Debug)]
pub(super) struct AnchoredFile {
    file: fs::File,
}

#[cfg(not(unix))]
fn open_regular_nofollow(
    path: &Path,
    contract_label: &str,
    record_label: &str,
) -> Result<fs::File, String> {
    let named = fs::symlink_metadata(path)
        .map_err(|error| format!("failed to inspect {contract_label} {record_label}: {error}"))?;
    if named.file_type().is_symlink() || !named.file_type().is_file() {
        return Err(format!(
            "{contract_label} {record_label} must be a regular non-symlink file"
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

        if named.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(format!(
                "{contract_label} {record_label} must not be a reparse point"
            ));
        }
    }

    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt as _;
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ,
        };

        options
            .share_mode(FILE_SHARE_READ)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    let file = options
        .open(path)
        .map_err(|error| format!("failed to open {contract_label} {record_label}: {error}"))?;
    let opened = file.metadata().map_err(|error| {
        format!("failed to inspect opened {contract_label} {record_label}: {error}")
    })?;
    if !opened.file_type().is_file() {
        return Err(format!(
            "{contract_label} {record_label} did not open as a regular file"
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

        if opened.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(format!(
                "{contract_label} {record_label} opened as a reparse point"
            ));
        }
    }
    Ok(file)
}

fn open_directory_nofollow(
    path: &Path,
    contract_label: &str,
    record_label: &str,
) -> Result<fs::File, String> {
    let named = fs::symlink_metadata(path)
        .map_err(|error| format!("failed to inspect {contract_label} {record_label}: {error}"))?;
    if named.file_type().is_symlink() || !named.file_type().is_dir() {
        return Err(format!(
            "{contract_label} {record_label} must be a regular non-symlink directory"
        ));
    }
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;

        options.custom_flags(
            (rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::DIRECTORY).bits() as i32,
        );
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt as _;
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_SHARE_READ,
        };

        options
            .share_mode(FILE_SHARE_READ)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT);
    }
    let file = options
        .open(path)
        .map_err(|error| format!("failed to open {contract_label} {record_label}: {error}"))?;
    let opened = file.metadata().map_err(|error| {
        format!("failed to inspect opened {contract_label} {record_label}: {error}")
    })?;
    if !opened.file_type().is_dir() {
        return Err(format!(
            "{contract_label} {record_label} did not open as a directory"
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;

        if opened.dev() != named.dev() || opened.ino() != named.ino() {
            return Err(format!(
                "{contract_label} {record_label} changed while it was opened"
            ));
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT;

        if opened.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(format!(
                "{contract_label} {record_label} opened as a reparse point"
            ));
        }
    }
    Ok(file)
}

pub(super) fn open_root(path: &Path, contract_label: &str) -> Result<AnchoredDirectory, String> {
    Ok(AnchoredDirectory {
        path: path.to_path_buf(),
        guards: vec![open_directory_nofollow(
            path,
            contract_label,
            "corpus root",
        )?],
    })
}

pub(super) fn member_names(
    directory: &AnchoredDirectory,
    contract_label: &str,
    record_label: &str,
    max_entries: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    #[cfg(unix)]
    {
        let guard = directory
            .guards
            .last()
            .ok_or_else(|| format!("{contract_label} {record_label} lost its directory anchor"))?;
        let entries = rustix::fs::Dir::read_from(guard).map_err(|error| {
            format!("failed to enumerate opened {contract_label} {record_label}: {error}")
        })?;
        for (index, entry) in entries.enumerate() {
            if abort.is_aborted() {
                return Err(format!(
                    "{contract_label} deadline expired while enumerating {record_label}"
                ));
            }
            if index >= max_entries {
                return Err(format!(
                    "{contract_label} {record_label} exceeds its census envelope"
                ));
            }
            let entry = entry.map_err(|error| {
                format!("failed to inspect opened {contract_label} {record_label}: {error}")
            })?;
            let name = entry.file_name().to_str().map_err(|error| {
                format!("{contract_label} {record_label} member name is not UTF-8: {error}")
            })?;
            if matches!(name, "." | "..") {
                continue;
            }
            names.push(name.to_string());
        }
    }
    #[cfg(not(unix))]
    {
        for (index, entry) in fs::read_dir(&directory.path)
            .map_err(|error| {
                format!("failed to enumerate {contract_label} {record_label}: {error}")
            })?
            .enumerate()
        {
            if abort.is_aborted() {
                return Err(format!(
                    "{contract_label} deadline expired while enumerating {record_label}"
                ));
            }
            if index >= max_entries {
                return Err(format!(
                    "{contract_label} {record_label} exceeds its census envelope"
                ));
            }
            let entry = entry.map_err(|error| {
                format!("failed to inspect {contract_label} {record_label}: {error}")
            })?;
            names.push(
                entry
                    .file_name()
                    .to_str()
                    .ok_or_else(|| {
                        format!("{contract_label} {record_label} member name is not UTF-8")
                    })?
                    .to_string(),
            );
        }
    }
    Ok(names)
}

fn exact_member_name(
    directory: &AnchoredDirectory,
    expected: &str,
    contract_label: &str,
    record_label: &str,
    max_entries: usize,
    abort: &dyn AbortSignal,
) -> Result<String, String> {
    let matches = member_names(directory, contract_label, record_label, max_entries, abort)?
        .into_iter()
        .filter(|name| name.eq_ignore_ascii_case(expected))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [name] if name == expected => Ok(name.clone()),
        [] => Err(format!(
            "{contract_label} is missing path component {expected:?}"
        )),
        _ => Err(format!(
            "{contract_label} path component {expected:?} changed case or became ambiguous: {matches:?}"
        )),
    }
}

fn open_directory_member_nofollow(
    directory: &AnchoredDirectory,
    name: &str,
    contract_label: &str,
    record_label: &str,
) -> Result<fs::File, String> {
    #[cfg(unix)]
    {
        let guard = directory
            .guards
            .last()
            .ok_or_else(|| format!("{contract_label} {record_label} lost its directory anchor"))?;
        let fd = rustix::fs::openat(
            guard,
            name,
            rustix::fs::OFlags::RDONLY
                | rustix::fs::OFlags::CLOEXEC
                | rustix::fs::OFlags::NOFOLLOW
                | rustix::fs::OFlags::DIRECTORY,
            rustix::fs::Mode::empty(),
        )
        .map_err(|error| {
            format!("failed to open anchored {contract_label} {record_label}: {error}")
        })?;
        let file: fs::File = fd.into();
        if !file
            .metadata()
            .map_err(|error| {
                format!("failed to inspect opened {contract_label} {record_label}: {error}")
            })?
            .is_dir()
        {
            return Err(format!(
                "{contract_label} {record_label} did not open as a directory"
            ));
        }
        return Ok(file);
    }
    #[cfg(not(unix))]
    open_directory_nofollow(&directory.path.join(name), contract_label, record_label)
}

fn open_file_member_nofollow(
    directory: &AnchoredDirectory,
    name: &str,
    contract_label: &str,
    record_label: &str,
) -> Result<fs::File, String> {
    #[cfg(unix)]
    {
        let guard = directory
            .guards
            .last()
            .ok_or_else(|| format!("{contract_label} {record_label} lost its directory anchor"))?;
        let fd = rustix::fs::openat(
            guard,
            name,
            rustix::fs::OFlags::RDONLY
                | rustix::fs::OFlags::CLOEXEC
                | rustix::fs::OFlags::NONBLOCK
                | rustix::fs::OFlags::NOFOLLOW,
            rustix::fs::Mode::empty(),
        )
        .map_err(|error| {
            format!("failed to open anchored {contract_label} {record_label}: {error}")
        })?;
        let file: fs::File = fd.into();
        if !file
            .metadata()
            .map_err(|error| {
                format!("failed to inspect opened {contract_label} {record_label}: {error}")
            })?
            .is_file()
        {
            return Err(format!(
                "{contract_label} {record_label} did not open as a regular file"
            ));
        }
        return Ok(file);
    }
    #[cfg(not(unix))]
    open_regular_nofollow(&directory.path.join(name), contract_label, record_label)
}

fn require_single_normal_component(
    name: &str,
    contract_label: &str,
    record_label: &str,
) -> Result<(), String> {
    let mut components = Path::new(name).components();
    match (components.next(), components.next()) {
        (Some(std::path::Component::Normal(component)), None)
            if component == std::ffi::OsStr::new(name) =>
        {
            Ok(())
        }
        _ => Err(format!(
            "{contract_label} {record_label} member name must be one normal path component"
        )),
    }
}

pub(super) fn open_file_member(
    directory: &AnchoredDirectory,
    name: &str,
    contract_label: &str,
    record_label: &str,
) -> Result<AnchoredFile, String> {
    require_single_normal_component(name, contract_label, record_label)?;
    Ok(AnchoredFile {
        file: open_file_member_nofollow(directory, name, contract_label, record_label)?,
    })
}

pub(super) fn exact_child_directory(
    mut parent: AnchoredDirectory,
    expected: &str,
    contract_label: &str,
    max_entries: usize,
    abort: &dyn AbortSignal,
) -> Result<AnchoredDirectory, String> {
    let name = exact_member_name(
        &parent,
        expected,
        contract_label,
        "parent directory",
        max_entries,
        abort,
    )?;
    let guard = open_directory_member_nofollow(&parent, &name, contract_label, expected)?;
    parent.path.push(&name);
    parent.guards.push(guard);
    Ok(parent)
}

pub(super) fn exact_child_file(
    parent: &AnchoredDirectory,
    expected: &str,
    contract_label: &str,
    record_label: &str,
    max_entries: usize,
    abort: &dyn AbortSignal,
) -> Result<AnchoredFile, String> {
    let name = exact_member_name(
        parent,
        expected,
        contract_label,
        "parent directory",
        max_entries,
        abort,
    )?;
    Ok(AnchoredFile {
        file: open_file_member_nofollow(parent, &name, contract_label, record_label)?,
    })
}

pub(super) fn optional_exact_child_file(
    parent: &AnchoredDirectory,
    expected: &str,
    contract_label: &str,
    parent_label: &str,
    record_label: &str,
    max_entries: usize,
    abort: &dyn AbortSignal,
) -> Result<Option<AnchoredFile>, String> {
    let matches = member_names(parent, contract_label, parent_label, max_entries, abort)?
        .into_iter()
        .filter(|name| name.eq_ignore_ascii_case(expected))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [name] if name == expected => Ok(Some(AnchoredFile {
            file: open_file_member_nofollow(parent, name, contract_label, record_label)?,
        })),
        [] => Ok(None),
        _ => Err(format!(
            "{contract_label} optional path component {expected:?} changed case or became ambiguous: {matches:?}"
        )),
    }
}

pub(super) fn read_bounded_raw(
    file: AnchoredFile,
    max_bytes: u64,
    contract_label: &str,
    record_label: &str,
    abort: &dyn AbortSignal,
) -> Result<Vec<u8>, String> {
    if abort.is_aborted() {
        return Err(format!(
            "{contract_label} deadline expired before reading {record_label}"
        ));
    }
    let metadata = file.file.metadata().map_err(|error| {
        format!("failed to inspect opened {contract_label} {record_label}: {error}")
    })?;
    if metadata.len() > max_bytes {
        return Err(format!(
            "{contract_label} {record_label} exceeds its {max_bytes}-byte read bound"
        ));
    }
    let capacity = usize::try_from(metadata.len().min(max_bytes)).map_err(|_| {
        format!("{contract_label} {record_label} read bound does not fit this platform")
    })?;
    let read_limit = max_bytes.checked_add(1).ok_or_else(|| {
        format!("{contract_label} {record_label} read bound cannot be incremented safely")
    })?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(capacity).map_err(|error| {
        format!("{contract_label} {record_label} read allocation failed: {error}")
    })?;
    file.file
        .take(read_limit)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("failed to read {contract_label} {record_label}: {error}"))?;
    if abort.is_aborted() {
        return Err(format!(
            "{contract_label} deadline expired while reading {record_label}"
        ));
    }
    if bytes.len() as u64 > max_bytes {
        return Err(format!(
            "{contract_label} {record_label} exceeds its {max_bytes}-byte read bound"
        ));
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    const TEST_LABEL: &str = "anchored contract filesystem test";

    #[test]
    fn exact_traversal_and_bounded_reads_fail_closed() {
        let temporary = tempfile::tempdir().expect("create anchored filesystem fixture");
        let root_path = temporary.path();
        fs::create_dir(root_path.join("Exact")).expect("create exact child directory");
        fs::write(root_path.join("Exact").join("payload.txt"), b"abc")
            .expect("write exact child file");

        let root = open_root(root_path, TEST_LABEL).expect("anchor fixture root");
        let exact = exact_child_directory(root, "Exact", TEST_LABEL, 8, &NoAbort)
            .expect("open exact child directory");
        let file = exact_child_file(&exact, "payload.txt", TEST_LABEL, "payload", 8, &NoAbort)
            .expect("open exact child file");
        assert_eq!(
            read_bounded_raw(file, 3, TEST_LABEL, "payload", &NoAbort)
                .expect("read payload at its exact bound"),
            b"abc"
        );

        let root = open_root(root_path, TEST_LABEL).expect("re-anchor fixture root");
        assert!(exact_child_directory(root, "exact", TEST_LABEL, 8, &NoAbort).is_err());
        let root = open_root(root_path, TEST_LABEL).expect("re-anchor fixture root");
        assert!(exact_child_directory(root, "Missing", TEST_LABEL, 8, &NoAbort).is_err());
        let root = open_root(root_path, TEST_LABEL).expect("re-anchor fixture root");
        assert!(member_names(&root, TEST_LABEL, "root", 0, &NoAbort).is_err());
        let root = open_root(root_path, TEST_LABEL).expect("re-anchor fixture root");
        assert!(member_names(&root, TEST_LABEL, "root", 8, &ImmediateAbort).is_err());

        let root = open_root(root_path, TEST_LABEL).expect("re-anchor fixture root");
        let exact = exact_child_directory(root, "Exact", TEST_LABEL, 8, &NoAbort)
            .expect("re-open exact child directory");
        let oversized = exact_child_file(&exact, "payload.txt", TEST_LABEL, "payload", 8, &NoAbort)
            .expect("re-open exact child file");
        assert!(read_bounded_raw(oversized, 2, TEST_LABEL, "payload", &NoAbort).is_err());
        let aborted = exact_child_file(&exact, "payload.txt", TEST_LABEL, "payload", 8, &NoAbort)
            .expect("re-open exact child file");
        assert!(read_bounded_raw(aborted, 3, TEST_LABEL, "payload", &ImmediateAbort).is_err());
        assert!(
            open_file_member(
                &exact,
                "../Exact/payload.txt",
                TEST_LABEL,
                "traversal payload",
            )
            .is_err()
        );
        let root = open_root(root_path, TEST_LABEL).expect("re-anchor fixture root");
        assert!(
            open_file_member(&root, "Exact/payload.txt", TEST_LABEL, "nested payload",).is_err()
        );
        let unincrementable =
            exact_child_file(&exact, "payload.txt", TEST_LABEL, "payload", 8, &NoAbort)
                .expect("re-open exact child file");
        assert!(
            read_bounded_raw(unincrementable, u64::MAX, TEST_LABEL, "payload", &NoAbort,).is_err(),
            "an unincrementable read bound must fail before Read::take"
        );
    }

    #[cfg(unix)]
    #[test]
    fn unix_symlinks_and_case_collisions_are_rejected() {
        use std::os::unix::fs::symlink;

        let temporary = tempfile::tempdir().expect("create Unix anchored fixture");
        let root_path = temporary.path().join("root");
        fs::create_dir(&root_path).expect("create Unix fixture root");
        fs::create_dir(root_path.join("Target")).expect("create target directory");
        fs::write(root_path.join("Target").join("payload"), b"x").expect("write target payload");
        symlink(root_path.join("Target"), root_path.join("Linked"))
            .expect("create child directory symlink");
        symlink(
            root_path.join("Target").join("payload"),
            root_path.join("payload-link"),
        )
        .expect("create child file symlink");
        let root_link = temporary.path().join("root-link");
        symlink(&root_path, &root_link).expect("create root symlink");

        assert!(open_root(&root_link, TEST_LABEL).is_err());
        let root = open_root(&root_path, TEST_LABEL).expect("anchor Unix fixture");
        assert!(exact_child_directory(root, "Linked", TEST_LABEL, 16, &NoAbort).is_err());
        let root = open_root(&root_path, TEST_LABEL).expect("re-anchor Unix fixture");
        assert!(
            exact_child_file(
                &root,
                "payload-link",
                TEST_LABEL,
                "linked payload",
                16,
                &NoAbort,
            )
            .is_err()
        );

        fs::write(root_path.join("Item"), b"upper").expect("write upper-case member");
        fs::write(root_path.join("item"), b"lower").expect("write lower-case member");
        let root = open_root(&root_path, TEST_LABEL).expect("re-anchor collision fixture");
        assert!(
            exact_child_file(&root, "Item", TEST_LABEL, "item", 16, &NoAbort).is_err(),
            "case-colliding members must be ambiguous even when one spelling is exact"
        );
    }
}
