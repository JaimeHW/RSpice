//! The lock file that says who is editing one file of a library folder.

use serde::{Deserialize, Serialize};

use super::format::{EncodedFile, LibraryFileError, encode_library_file};

/// Who holds one file of a library folder for editing: written beside the
/// file as `<name>.lock` by the first edit, only when no lock is there, and
/// removed when the file has no unsaved changes or the project closes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LibraryFileLock {
    pub user: String,
    pub computer: String,
    /// The holding process's identifier on that computer.
    pub process: u32,
    /// The project the edit is made in, by its file's path or identity.
    pub project: String,
    /// When the lock was taken, in milliseconds since the epoch.
    pub since_unix_ms: u64,
}

impl LibraryFileLock {
    /// Who holds it, as a refusal names them: "jsmith on LAB-PC3".
    pub fn holder(&self) -> String {
        format!("{} on {}", self.user, self.computer)
    }

    /// Whether this lock was taken on `computer`, the only place a dead
    /// holder can be recognised and its lock taken over.
    pub fn is_on(&self, computer: &str) -> bool {
        self.computer.eq_ignore_ascii_case(computer)
    }

    pub fn encode(&self) -> Result<EncodedFile, LibraryFileError> {
        encode_library_file(self)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, LibraryFileError> {
        serde_json::from_slice(bytes).map_err(|error| LibraryFileError::Damaged(error.to_string()))
    }
}
