//! Project files read with their exact identity: a buffer already in
//! memory, or a stream parsed as it is read.

use crate::file::{finish_decoding, keep_segments_beside};
use crate::{DecodedProject, ProjectFile, ProjectIoError, decode_project_text};
use rspice_design::hierarchy::HierarchySourceFiles;
use rspice_vocabulary::product::ContentDigest;
use serde::Deserialize as _;
use sha2::{Digest as _, Sha256};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

/// Bytes already in memory and their identity. Neither can be replaced
/// after capture, so parsing and canonical admission describe the same
/// snapshot.
#[derive(Debug)]
pub struct ProjectBytes {
    bytes: Vec<u8>,
    digest: ContentDigest,
}

impl ProjectBytes {
    /// Take an already resident buffer without copying it.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, ProjectIoError> {
        let digest = super::digest_bytes(&bytes);
        Ok(Self { bytes, digest })
    }

    pub fn into_parts(self) -> (Vec<u8>, ContentDigest) {
        (self.bytes, self.digest)
    }

    pub(super) fn into_text(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.bytes)
    }

    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    pub fn decode(
        self,
        source_path: Option<&Path>,
        source_files: impl HierarchySourceFiles,
    ) -> Result<DecodedProject, ProjectIoError> {
        let contents =
            std::str::from_utf8(&self.bytes).map_err(|error| ProjectIoError::ParseError {
                damage: crate::FileDamage::Unreadable,
                cause: format!("project is not valid UTF-8: {error}"),
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

/// A project file read once, a buffer at a time: the digest of every byte
/// read, and the project parsed from those same bytes as they passed. The
/// file is never held whole, so its size costs memory only in the project
/// it holds; and the digest and the project cannot describe two versions of
/// the file.
#[derive(Debug)]
pub struct ProjectRead {
    digest: ContentDigest,
    parsed: Result<ProjectFile, ProjectIoError>,
    source_path: Option<PathBuf>,
}

impl ProjectRead {
    /// Read the project `reader` holds: the file at `source_path` when it
    /// has one. A file that does not parse is still read to its end, so its
    /// digest is known, and says what is wrong with it once it is decoded.
    pub fn read(reader: impl Read, source_path: Option<&Path>) -> Result<Self, ProjectIoError> {
        let folders = keep_segments_beside(source_path);
        let mut stream = BufReader::with_capacity(
            64 * 1024,
            HashingReader {
                inner: reader,
                hasher: Sha256::new(),
            },
        );
        let mut parse = || {
            let mut deserializer = serde_json::Deserializer::from_reader(&mut stream);
            ProjectFile::deserialize(&mut deserializer)
                .and_then(|project| deserializer.end().map(|()| project))
        };
        let parsed = match folders {
            Some(folders) => rspice_results::store_reader::reading_segments_from(folders, parse),
            None => parse(),
        };
        let parsed = match parsed {
            Ok(project) => Ok(project),
            Err(error) if error.is_io() => return Err(ProjectIoError::Io(error.to_string())),
            Err(error) => Err(ProjectIoError::unreadable_json(&error)),
        };
        // What the parser left unread is part of the file and its digest.
        io::copy(&mut stream, &mut io::sink())?;
        let digest = ContentDigest::from_bytes(stream.into_inner().hasher.finalize().into());
        Ok(Self {
            digest,
            parsed,
            source_path: source_path.map(Path::to_path_buf),
        })
    }

    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    pub fn decode(
        self,
        source_files: impl HierarchySourceFiles,
    ) -> Result<DecodedProject, ProjectIoError> {
        finish_decoding(self.parsed?, self.source_path.as_deref(), source_files)
    }

    /// Reject changed bytes before migrations or source access, and before
    /// a parse failure of the changed file is reported as damage.
    pub fn decode_if_digest(
        self,
        expected: ContentDigest,
        source_files: impl HierarchySourceFiles,
    ) -> Result<Option<DecodedProject>, ProjectIoError> {
        if self.digest != expected {
            return Ok(None);
        }
        self.decode(source_files).map(Some)
    }
}

/// A reader that hashes what passes through it, under the buffer, so it
/// hashes whole buffers rather than the parser's single bytes.
struct HashingReader<R> {
    inner: R,
    hasher: Sha256,
}

impl<R: Read> Read for HashingReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let read = self.inner.read(buffer)?;
        self.hasher.update(&buffer[..read]);
        Ok(read)
    }
}
