//! Bounded project byte snapshots and decoding against their exact identity.

use crate::{DecodedProject, MAX_PROJECT_FILE_BYTES, ProjectIoError, decode_project_text};
use rspice_app_types::product::ContentDigest;
use rspice_design::hierarchy::HierarchySourceFiles;
use sha2::{Digest as _, Sha256};
use std::{io::Read, path::Path};

/// Bytes and identity from one bounded read. Neither can be replaced after
/// capture, so parsing and canonical admission describe the same snapshot.
#[derive(Debug)]
pub struct ProjectBytes {
    bytes: Vec<u8>,
    digest: ContentDigest,
}

impl ProjectBytes {
    pub fn read(mut reader: impl Read, advertised: u64) -> Result<Self, ProjectIoError> {
        if advertised > MAX_PROJECT_FILE_BYTES {
            return Err(ProjectIoError::InvalidData(format!(
                "project is {advertised} bytes; the supported maximum is {MAX_PROJECT_FILE_BYTES} bytes"
            )));
        }
        let mut bytes = Vec::with_capacity(advertised.min(8 * 1024 * 1024) as usize);
        let mut hasher = Sha256::new();
        let mut buffer = [0_u8; 64 * 1024];
        let mut total = 0_u64;
        loop {
            let read = reader.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            total = total.saturating_add(read as u64);
            if total > MAX_PROJECT_FILE_BYTES {
                return Err(ProjectIoError::InvalidData(format!(
                    "project grew beyond the supported {MAX_PROJECT_FILE_BYTES} byte maximum while it was being read"
                )));
            }
            hasher.update(&buffer[..read]);
            bytes.extend_from_slice(&buffer[..read]);
        }
        Ok(Self {
            bytes,
            digest: ContentDigest::from_bytes(hasher.finalize().into()),
        })
    }

    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    pub fn decode(
        self,
        source_path: Option<&Path>,
        source_files: impl HierarchySourceFiles,
    ) -> Result<DecodedProject, ProjectIoError> {
        let contents = std::str::from_utf8(&self.bytes).map_err(|error| {
            ProjectIoError::ParseError(format!("project is not valid UTF-8: {error}"))
        })?;
        decode_project_text(contents, source_path, source_files)
    }

    /// Reject changed bytes before UTF-8, JSON, migrations or source access.
    pub fn decode_if_digest(
        self,
        expected: ContentDigest,
        source_path: Option<&Path>,
        source_files: impl HierarchySourceFiles,
    ) -> Result<Option<DecodedProject>, ProjectIoError> {
        if self.digest != expected {
            return Ok(None);
        }
        self.decode(source_path, source_files).map(Some)
    }
}

#[cfg(test)]
mod tests;
