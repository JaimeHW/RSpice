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
/// ZIP64 footer counts are admitted before the backend reserves metadata.
pub(crate) fn open_unique_archive(
    bytes: &[u8],
    max_members: usize,
) -> Result<ZipArchive<Cursor<&[u8]>>, ArchiveReadError> {
    admit_zip64_member_counts(bytes, max_members)?;
    let archive = ZipArchive::new(Cursor::new(bytes)).map_err(ArchiveReadError::Zip)?;
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

const ZIP64_END_SIGNATURE: &[u8] = b"PK\x06\x06";
const ZIP64_END_BYTES: usize = 56;
const CENTRAL_HEADER_BYTES: u64 = 46;

/// The backend reserves its member index from the footer's declared count
/// before decoding a single entry. A ZIP32 footer declares at most 65,535, but
/// the backend accepts any ZIP64 count whose directory, at 46 bytes per entry,
/// fits before the record: a 64 MiB source could reserve hundreds of MiB.
/// Refuse every ZIP64 end record the backend could accept with a count above
/// the limit, not only the last, because the backend falls back to earlier
/// footers when a later one fails to decode.
fn admit_zip64_member_counts(bytes: &[u8], max_members: usize) -> Result<(), ArchiveReadError> {
    let limit = u64::try_from(max_members).unwrap_or(u64::MAX);
    for (position, record) in bytes.windows(ZIP64_END_BYTES).enumerate() {
        if !record.starts_with(ZIP64_END_SIGNATURE) {
            continue;
        }
        let field = |offset: usize| u64::from_le_bytes(std::array::from_fn(|i| record[offset + i]));
        let members = field(32);
        let directory_end = members
            .saturating_mul(CENTRAL_HEADER_BYTES)
            .saturating_add(field(48));
        if members > limit && u64::try_from(position).is_ok_and(|start| directory_end <= start) {
            return Err(ArchiveReadError::MemberCount {
                members: usize::try_from(members).unwrap_or(usize::MAX),
                limit: max_members,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
