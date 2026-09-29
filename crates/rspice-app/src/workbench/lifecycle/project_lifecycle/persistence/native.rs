//! Native filesystem adapter for project bindings and exact publication.

use super::PersistenceBinding;
use crate::io::ProjectSnapshot;
use crate::io::durable_file::CompareExchangeError;
use crate::product::ContentDigest;
use rspice_project::ProjectIoError;
use rspice_project::persistence::native::NativeProjectStorage;
use rspice_project::persistence::{
    NativeBindingReceipt, PersistenceError, ProjectBytes, digest_bytes,
};
use std::path::{Path, PathBuf};

pub(crate) struct NativeStorage;

fn native_storage_error(error: CompareExchangeError) -> PersistenceError {
    match error {
        CompareExchangeError::PublicationUncertain {
            message,
            recovery_paths,
        } => PersistenceError::PublicationUncertain {
            message,
            recovery_paths,
        },
        CompareExchangeError::LeaseBusy(path) => PersistenceError::LeaseBusy(path),
        CompareExchangeError::Io(error) => PersistenceError::Io(error),
        // Publication maps this to ExternalChange at its call site. Retain
        // the existing diagnostic if another storage operation reports it.
        error @ CompareExchangeError::Conflict { .. } => {
            PersistenceError::Platform(error.to_string())
        }
    }
}

impl NativeProjectStorage for NativeStorage {
    type ExpectedContent = crate::io::durable_file::ExpectedContent;

    fn normalize_path(&self, path: &Path) -> Result<PathBuf, PersistenceError> {
        normalize_native_path(path)
    }

    fn reconcile_publication(&self, path: &Path) -> Result<(), PersistenceError> {
        crate::io::durable_file::reconcile_publication(path).map_err(native_storage_error)
    }

    fn read_project(&self, path: &Path) -> Result<ProjectBytes, ProjectIoError> {
        crate::io::project_io::read_project_bytes(path)
    }

    fn accepted_content(&self, digest: ContentDigest) -> Self::ExpectedContent {
        crate::io::durable_file::ExpectedContent::Digest(*digest.as_bytes())
    }

    fn observe_destination(
        &self,
        path: &Path,
    ) -> Result<crate::io::durable_file::ExpectedContent, PersistenceError> {
        crate::io::durable_file::observe_expected_content(path).map_err(native_storage_error)
    }

    fn publish(
        &self,
        path: &Path,
        expected: crate::io::durable_file::ExpectedContent,
        bytes: &[u8],
    ) -> Result<ContentDigest, PersistenceError> {
        match crate::io::durable_file::compare_exchange_bytes(path, expected, bytes) {
            Ok(()) => Ok(digest_bytes(bytes)),
            Err(CompareExchangeError::Conflict { .. }) => Err(PersistenceError::ExternalChange),
            Err(error) => Err(native_storage_error(error)),
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
            let left = std::fs::metadata(left_path).map_err(PersistenceError::Io)?;
            let right = std::fs::metadata(right_path).map_err(PersistenceError::Io)?;

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
    let (project, binding) = PersistenceBinding::open(
        &NativeStorage,
        path,
        crate::state::workspace::WorkspaceSourceFiles,
    )?;
    Ok((ProjectSnapshot::from_decoded(project), binding))
}

pub(crate) fn restore_native_binding(
    path: &Path,
    session_project_id: &str,
    receipt: &NativeBindingReceipt,
) -> Result<(ProjectSnapshot, PersistenceBinding), PersistenceError> {
    let (project, binding) = PersistenceBinding::restore(
        &NativeStorage,
        path,
        session_project_id,
        receipt,
        crate::state::workspace::WorkspaceSourceFiles,
    )?;
    Ok((ProjectSnapshot::from_decoded(project), binding))
}

pub(crate) fn normalize_native_path(path: &Path) -> Result<PathBuf, PersistenceError> {
    if path.exists() {
        return std::fs::canonicalize(path).map_err(PersistenceError::Io);
    }
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let parent = std::fs::canonicalize(parent).map_err(PersistenceError::Io)?;
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

    let file = std::fs::File::open(path).map_err(PersistenceError::Io)?;
    let mut information = unsafe { std::mem::zeroed::<BY_HANDLE_FILE_INFORMATION>() };
    let succeeded =
        unsafe { GetFileInformationByHandle(file.as_raw_handle() as *mut _, &mut information) };
    if succeeded == 0 {
        return Err(PersistenceError::Io(std::io::Error::last_os_error()));
    }
    let file_index = ((information.nFileIndexHigh as u64) << 32) | information.nFileIndexLow as u64;
    Ok((information.dwVolumeSerialNumber, file_index))
}

#[cfg(test)]
mod tests;
