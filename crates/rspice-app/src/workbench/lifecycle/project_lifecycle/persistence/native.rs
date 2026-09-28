//! Native filesystem adapter for project bindings and exact publication.

use super::PersistenceBinding;
use crate::io::ProjectSnapshot;
use crate::product::ContentDigest;
use rspice_project::persistence::native::NativeProjectStorage;
use rspice_project::persistence::{NativeBindingReceipt, PersistenceError, digest_bytes};
use std::path::{Path, PathBuf};

pub(crate) struct NativeStorage;

impl NativeProjectStorage for NativeStorage {
    type ExpectedContent = crate::io::durable_file::ExpectedContent;

    fn normalize_path(&self, path: &Path) -> Result<PathBuf, PersistenceError> {
        normalize_native_path(path)
    }

    fn accepted_content(&self, digest: ContentDigest) -> Self::ExpectedContent {
        crate::io::durable_file::ExpectedContent::Digest(*digest.as_bytes())
    }

    fn observe_destination(
        &self,
        path: &Path,
    ) -> Result<crate::io::durable_file::ExpectedContent, PersistenceError> {
        crate::io::durable_file::observe_expected_content(path)
            .map_err(|error| PersistenceError::Platform(error.to_string()))
    }

    fn publish(
        &self,
        path: &Path,
        expected: crate::io::durable_file::ExpectedContent,
        bytes: &[u8],
    ) -> Result<ContentDigest, PersistenceError> {
        match crate::io::durable_file::compare_exchange_bytes(path, expected, bytes) {
            Ok(()) => Ok(digest_bytes(bytes)),
            Err(crate::io::durable_file::CompareExchangeError::Conflict { .. }) => {
                Err(PersistenceError::ExternalChange)
            }
            Err(crate::io::durable_file::CompareExchangeError::Io(error)) => {
                Err(PersistenceError::Platform(error.to_string()))
            }
            Err(error) => Err(PersistenceError::Platform(error.to_string())),
        }
    }

    fn same_file(&self, left: &Path, right: &Path) -> Result<bool, PersistenceError> {
        // A missing endpoint cannot currently alias an existing filesystem
        // object. In particular, a deleted canonical source must not prevent the
        // user from recovering work to an independently selected destination.
        if !left.exists() || !right.exists() {
            return Ok(false);
        }

        #[cfg(windows)]
        {
            Ok(windows_file_identity(left)? == windows_file_identity(right)?)
        }

        #[cfg(not(windows))]
        {
            let left_path = left;
            let right_path = right;
            let left = std::fs::metadata(left_path)
                .map_err(|error| PersistenceError::Platform(error.to_string()))?;
            let right = std::fs::metadata(right_path)
                .map_err(|error| PersistenceError::Platform(error.to_string()))?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt as _;
                Ok(left.dev() == right.dev() && left.ino() == right.ino())
            }
            #[cfg(not(unix))]
            {
                let _ = (left, right);
                Ok(normalize_native_path(left_path)? == normalize_native_path(right_path)?)
            }
        }
    }
}

pub(crate) fn read_native_binding(
    path: &Path,
) -> Result<(ProjectSnapshot, PersistenceBinding), PersistenceError> {
    let canonical_path = normalize_native_path(path)?;
    crate::io::durable_file::reconcile_publication(&canonical_path)
        .map_err(|error| PersistenceError::Platform(error.to_string()))?;
    let (project, digest) = crate::io::project_io::load_project_file_with_digest(&canonical_path)?;
    Ok((
        project,
        PersistenceBinding {
            canonical_path,
            accepted_digest: digest,
        },
    ))
}

pub(crate) fn restore_native_binding(
    path: &Path,
    session_project_id: &str,
    receipt: &NativeBindingReceipt,
) -> Result<(ProjectSnapshot, PersistenceBinding), PersistenceError> {
    receipt.validate_session_project(session_project_id)?;
    let canonical_path = normalize_native_path(path)?;
    receipt.validate_canonical_path(&canonical_path)?;
    crate::io::durable_file::reconcile_publication(&canonical_path)
        .map_err(|error| PersistenceError::Platform(error.to_string()))?;
    let Some(project) = crate::io::project_io::load_project_file_with_expected_digest(
        &canonical_path,
        receipt.accepted_digest,
    )?
    else {
        return Err(PersistenceError::ExternalChange);
    };
    receipt.validate_loaded_project(&project.file)?;
    Ok((
        project,
        PersistenceBinding {
            canonical_path,
            accepted_digest: receipt.accepted_digest,
        },
    ))
}

pub(crate) fn normalize_native_path(path: &Path) -> Result<PathBuf, PersistenceError> {
    if path.exists() {
        return std::fs::canonicalize(path)
            .map_err(|error| PersistenceError::Platform(error.to_string()));
    }
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let parent = std::fs::canonicalize(parent)
        .map_err(|error| PersistenceError::Platform(error.to_string()))?;
    let name = path.file_name().ok_or_else(|| {
        PersistenceError::Platform(format!("'{}' has no project filename", path.display()))
    })?;
    Ok(parent.join(name))
}

#[cfg(windows)]
fn windows_file_identity(path: &Path) -> Result<(u32, u64), PersistenceError> {
    use std::os::windows::io::AsRawHandle as _;
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
    };

    let file =
        std::fs::File::open(path).map_err(|error| PersistenceError::Platform(error.to_string()))?;
    let mut information = unsafe { std::mem::zeroed::<BY_HANDLE_FILE_INFORMATION>() };
    let succeeded =
        unsafe { GetFileInformationByHandle(file.as_raw_handle() as *mut _, &mut information) };
    if succeeded == 0 {
        return Err(PersistenceError::Platform(
            std::io::Error::last_os_error().to_string(),
        ));
    }
    let file_index = ((information.nFileIndexHigh as u64) << 32) | information.nFileIndexLow as u64;
    Ok((information.dwVolumeSerialNumber, file_index))
}
