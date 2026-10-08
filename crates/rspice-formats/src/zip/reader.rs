//! Validate source member identities before selecting any ZIP payload.
use std::collections::HashSet;
use std::io::Cursor;
use zip::ZipArchive;
use zip::result::ZipError;

#[derive(Debug)]
pub(crate) enum ArchiveReadError {
    Zip(ZipError),
    MemberCount { members: usize, limit: usize },
    DuplicateMember(String),
}

/// The ZIP backend indexes members by their raw name and silently replaces
/// earlier entries with that name. Inspect the original central directory too,
/// before callers choose a manifest or result array from the resulting index.
///
/// ZIP64 uses the same fixed central header and name/extra/comment lengths.
/// The backend supplies its resolved directory offset, including any prefix.
/// The backend first admits the advertised count before reserving metadata.
pub(crate) fn open_unique_archive(
    bytes: &[u8],
    max_members: usize,
) -> Result<ZipArchive<Cursor<&[u8]>>, ArchiveReadError> {
    let archive = ZipArchive::with_config(
        zip::read::Config {
            max_files: Some(max_members),
            ..Default::default()
        },
        Cursor::new(bytes),
    )
    .map_err(|error| match error {
        ZipError::FileCountLimit { files, limit } => ArchiveReadError::MemberCount {
            members: files,
            limit,
        },
        error => ArchiveReadError::Zip(error),
    })?;
    if archive.len() > max_members {
        return Err(ArchiveReadError::MemberCount {
            members: archive.len(),
            limit: max_members,
        });
    }
    let invalid = || {
        ArchiveReadError::Zip(ZipError::InvalidArchive(
            "invalid source central directory".into(),
        ))
    };
    let mut position = usize::try_from(archive.central_directory_start()).map_err(|_| invalid())?;
    let mut names = HashSet::new();
    let mut count = 0usize;
    while bytes
        .get(position..)
        .is_some_and(|remaining| remaining.starts_with(b"PK\x01\x02"))
    {
        count = count.checked_add(1).ok_or_else(invalid)?;
        if count > max_members {
            return Err(ArchiveReadError::MemberCount {
                members: count,
                limit: max_members,
            });
        }
        let header_end = position.checked_add(46).ok_or_else(invalid)?;
        let header = bytes.get(position..header_end).ok_or_else(invalid)?;
        let length = |offset| usize::from(u16::from_le_bytes([header[offset], header[offset + 1]]));
        let name_end = header_end.checked_add(length(28)).ok_or_else(invalid)?;
        let name = bytes.get(header_end..name_end).ok_or_else(invalid)?;
        if !names.insert(name) {
            return Err(ArchiveReadError::DuplicateMember(
                String::from_utf8_lossy(name).into_owned(),
            ));
        }
        position = name_end
            .checked_add(length(30) + length(32))
            .ok_or_else(invalid)?;
        if position > bytes.len() {
            return Err(invalid());
        }
    }
    if count != archive.len() {
        return Err(ArchiveReadError::Zip(ZipError::InvalidArchive(
            "source central directory and member index disagree".into(),
        )));
    }
    Ok(archive)
}

#[cfg(test)]
mod tests;
