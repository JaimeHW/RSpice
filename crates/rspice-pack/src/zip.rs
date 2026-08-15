use std::collections::BTreeSet;

use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec_with_limit};

use crate::{
    error::{ArchiveFeature, PackError},
    limits::Limits,
    path::validate_path,
};

const LOCAL_HEADER_SIGNATURE: u32 = 0x0403_4b50;
const CENTRAL_HEADER_SIGNATURE: u32 = 0x0201_4b50;
const EOCD_SIGNATURE: u32 = 0x0605_4b50;
const ZIP64_LOCATOR_SIGNATURE: u32 = 0x0706_4b50;

const LOCAL_HEADER_BYTES: usize = 30;
const CENTRAL_HEADER_BYTES: usize = 46;
const EOCD_BYTES: usize = 22;
const ZIP64_LOCATOR_BYTES: usize = 20;

/// Version made by / needed to extract: 2.0, host system 0 (MS-DOS/FAT), the
/// oldest version that defines DEFLATE. Keeping the host byte at zero is what
/// makes external attributes of zero the only consistent value.
const VERSION: u16 = 20;
const METHOD_STORE: u16 = 0;
const METHOD_DEFLATE: u16 = 8;
/// 1980-01-01 00:00, the earliest instant the MS-DOS field can express.
const DOS_TIME: u16 = 0x0000;
const DOS_DATE: u16 = 0x0021;
const DEFLATE_LEVEL: u8 = 6;
const ZIP32_SENTINEL_16: u16 = 0xFFFF;
const ZIP32_SENTINEL_32: u32 = 0xFFFF_FFFF;

/// Bytes searched backwards for the end-of-central-directory record beyond the
/// record itself. The writer emits no archive comment, so a conforming archive
/// is found on the first probe; the window exists so a truncated or padded
/// archive is refused after bounded work rather than a full scan.
const EOCD_SCAN_WINDOW: usize = 1024;

const GENERAL_PURPOSE_ENCRYPTED: u16 = 1 << 0;
const GENERAL_PURPOSE_DATA_DESCRIPTOR: u16 = 1 << 3;
const GENERAL_PURPOSE_STRONG_ENCRYPTION: u16 = 1 << 6;
const GENERAL_PURPOSE_MASKED_HEADERS: u16 = 1 << 13;

/// One expanded archive entry, in central-directory order.
pub(crate) struct ArchiveEntry {
    pub(crate) name: String,
    pub(crate) data: Vec<u8>,
}

/// Writes the deterministic pack container.
///
/// Entries are written in the order given — callers own the fixed ordering
/// rule — with every field that a general ZIP writer would vary (timestamps,
/// versions, extra fields, attributes, comments, compression level) pinned, so
/// the same inputs against the same build produce byte-identical archives.
pub(crate) fn write_archive(entries: &[(&str, &[u8])]) -> Result<Vec<u8>, PackError> {
    let overflow = || PackError::MalformedArchive("archive exceeds the addressable 32-bit layout");
    let mut archive = Vec::new();
    let mut directory = Vec::new();
    for (name, data) in entries {
        validate_path(name)?;
        let compressed = compress_to_vec(data, DEFLATE_LEVEL);
        let checksum = crc32(data);
        let offset = u32::try_from(archive.len()).map_err(|_| overflow())?;
        let compressed_length = u32::try_from(compressed.len()).map_err(|_| overflow())?;
        let expanded_length = u32::try_from(data.len()).map_err(|_| overflow())?;
        let name_length = u16::try_from(name.len()).map_err(|_| overflow())?;

        archive.extend_from_slice(&LOCAL_HEADER_SIGNATURE.to_le_bytes());
        archive.extend_from_slice(&VERSION.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&METHOD_DEFLATE.to_le_bytes());
        archive.extend_from_slice(&DOS_TIME.to_le_bytes());
        archive.extend_from_slice(&DOS_DATE.to_le_bytes());
        archive.extend_from_slice(&checksum.to_le_bytes());
        archive.extend_from_slice(&compressed_length.to_le_bytes());
        archive.extend_from_slice(&expanded_length.to_le_bytes());
        archive.extend_from_slice(&name_length.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(name.as_bytes());
        archive.extend_from_slice(&compressed);

        directory.extend_from_slice(&CENTRAL_HEADER_SIGNATURE.to_le_bytes());
        directory.extend_from_slice(&VERSION.to_le_bytes());
        directory.extend_from_slice(&VERSION.to_le_bytes());
        directory.extend_from_slice(&0_u16.to_le_bytes());
        directory.extend_from_slice(&METHOD_DEFLATE.to_le_bytes());
        directory.extend_from_slice(&DOS_TIME.to_le_bytes());
        directory.extend_from_slice(&DOS_DATE.to_le_bytes());
        directory.extend_from_slice(&checksum.to_le_bytes());
        directory.extend_from_slice(&compressed_length.to_le_bytes());
        directory.extend_from_slice(&expanded_length.to_le_bytes());
        directory.extend_from_slice(&name_length.to_le_bytes());
        directory.extend_from_slice(&0_u16.to_le_bytes());
        directory.extend_from_slice(&0_u16.to_le_bytes());
        directory.extend_from_slice(&0_u16.to_le_bytes());
        directory.extend_from_slice(&0_u16.to_le_bytes());
        directory.extend_from_slice(&0_u32.to_le_bytes());
        directory.extend_from_slice(&offset.to_le_bytes());
        directory.extend_from_slice(name.as_bytes());
    }

    let count = u16::try_from(entries.len()).map_err(|_| overflow())?;
    let directory_offset = u32::try_from(archive.len()).map_err(|_| overflow())?;
    let directory_length = u32::try_from(directory.len()).map_err(|_| overflow())?;
    archive.extend_from_slice(&directory);
    archive.extend_from_slice(&EOCD_SIGNATURE.to_le_bytes());
    archive.extend_from_slice(&0_u16.to_le_bytes());
    archive.extend_from_slice(&0_u16.to_le_bytes());
    archive.extend_from_slice(&count.to_le_bytes());
    archive.extend_from_slice(&count.to_le_bytes());
    archive.extend_from_slice(&directory_length.to_le_bytes());
    archive.extend_from_slice(&directory_offset.to_le_bytes());
    archive.extend_from_slice(&0_u16.to_le_bytes());
    Ok(archive)
}

/// A central-directory record, already checked against the format subset.
struct CentralRecord {
    name: String,
    method: u16,
    checksum: u32,
    compressed_length: usize,
    expanded_length: usize,
    local_offset: usize,
}

/// Reads the pack container, refusing anything outside the reviewed subset.
///
/// Every limit is enforced before a caller sees a byte: the compressed cap on
/// entry, the entry-count cap from the directory header, and the expanded cap
/// from the declared sizes — which are then re-proved against the actual
/// inflated length and CRC, so a lying header buys nothing.
pub(crate) fn read_archive(
    archive: &[u8],
    limits: &Limits,
) -> Result<Vec<ArchiveEntry>, PackError> {
    if archive.len() > limits.max_archive_bytes {
        return Err(PackError::ArchiveTooLarge {
            limit: limits.max_archive_bytes,
            actual: archive.len(),
        });
    }
    let eocd = locate_eocd(archive)?;
    if read_u16(archive, eocd + 20)? != 0 {
        return Err(PackError::UnsupportedArchiveFeature(
            ArchiveFeature::ArchiveComment,
        ));
    }
    if eocd >= ZIP64_LOCATOR_BYTES
        && read_u32(archive, eocd - ZIP64_LOCATOR_BYTES)? == ZIP64_LOCATOR_SIGNATURE
    {
        return Err(PackError::UnsupportedArchiveFeature(ArchiveFeature::Zip64));
    }
    if read_u16(archive, eocd + 4)? != 0 || read_u16(archive, eocd + 6)? != 0 {
        return Err(PackError::UnsupportedArchiveFeature(
            ArchiveFeature::MultipleDisks,
        ));
    }
    let disk_entries = read_u16(archive, eocd + 8)?;
    let total_entries = read_u16(archive, eocd + 10)?;
    if disk_entries == ZIP32_SENTINEL_16 || total_entries == ZIP32_SENTINEL_16 {
        return Err(PackError::UnsupportedArchiveFeature(ArchiveFeature::Zip64));
    }
    if disk_entries != total_entries {
        return Err(PackError::MalformedArchive(
            "the end-of-central-directory record disagrees with itself about the entry count",
        ));
    }
    let count = usize::from(total_entries);
    if count > limits.max_files {
        return Err(PackError::TooManyFiles {
            limit: limits.max_files,
            actual: count,
        });
    }
    let directory_length = read_u32(archive, eocd + 12)?;
    let directory_offset = read_u32(archive, eocd + 16)?;
    if directory_length == ZIP32_SENTINEL_32 || directory_offset == ZIP32_SENTINEL_32 {
        return Err(PackError::UnsupportedArchiveFeature(ArchiveFeature::Zip64));
    }
    let directory_start = directory_offset as usize;
    let directory_end = directory_start
        .checked_add(directory_length as usize)
        .ok_or(PackError::MalformedArchive("central directory overflows"))?;
    if directory_end != eocd {
        return Err(PackError::MalformedArchive(
            "the central directory does not end at the end-of-central-directory record",
        ));
    }

    let records = read_central_directory(archive, directory_start, directory_end, count)?;
    let declared: u64 = records
        .iter()
        .map(|record| record.expanded_length as u64)
        .sum();
    if declared > limits.max_expanded_bytes as u64 {
        return Err(PackError::ExpandedTooLarge {
            limit: limits.max_expanded_bytes,
        });
    }

    let mut entries = Vec::with_capacity(records.len());
    let mut consumed = 0_usize;
    for record in records {
        let data = read_entry_data(archive, &record, directory_start, &mut consumed)?;
        entries.push(ArchiveEntry {
            name: record.name,
            data,
        });
    }
    // Local records tile the archive exactly, so there is nowhere to park
    // bytes that no entry accounts for and no signature covers.
    if consumed != directory_start {
        return Err(PackError::MalformedArchive(
            "the archive carries bytes that belong to no entry",
        ));
    }
    Ok(entries)
}

fn locate_eocd(archive: &[u8]) -> Result<usize, PackError> {
    let last = archive
        .len()
        .checked_sub(EOCD_BYTES)
        .ok_or(PackError::MalformedArchive(
            "archive is shorter than an end-of-central-directory record",
        ))?;
    let first = last.saturating_sub(EOCD_SCAN_WINDOW);
    for offset in (first..=last).rev() {
        if read_u32(archive, offset)? != EOCD_SIGNATURE {
            continue;
        }
        let comment = usize::from(read_u16(archive, offset + 20)?);
        if offset + EOCD_BYTES + comment == archive.len() {
            return Ok(offset);
        }
    }
    Err(PackError::MalformedArchive(
        "no end-of-central-directory record",
    ))
}

fn read_central_directory(
    archive: &[u8],
    start: usize,
    end: usize,
    count: usize,
) -> Result<Vec<CentralRecord>, PackError> {
    let truncated = || PackError::MalformedArchive("the central directory is truncated");
    let mut records = Vec::with_capacity(count.min(1024));
    let mut names = BTreeSet::new();
    let mut cursor = start;
    for _ in 0..count {
        let header_end = cursor
            .checked_add(CENTRAL_HEADER_BYTES)
            .ok_or_else(truncated)?;
        if header_end > end {
            return Err(truncated());
        }
        if read_u32(archive, cursor)? != CENTRAL_HEADER_SIGNATURE {
            return Err(PackError::MalformedArchive(
                "a central directory record is missing its signature",
            ));
        }
        check_general_purpose_flags(read_u16(archive, cursor + 8)?)?;
        let method = read_method(read_u16(archive, cursor + 10)?)?;
        let checksum = read_u32(archive, cursor + 16)?;
        let compressed_length = read_size(read_u32(archive, cursor + 20)?)?;
        let expanded_length = read_size(read_u32(archive, cursor + 24)?)?;
        let name_length = usize::from(read_u16(archive, cursor + 28)?);
        if read_u16(archive, cursor + 30)? != 0 {
            return Err(PackError::UnsupportedArchiveFeature(
                ArchiveFeature::ExtraField,
            ));
        }
        if read_u16(archive, cursor + 32)? != 0 {
            return Err(PackError::UnsupportedArchiveFeature(
                ArchiveFeature::EntryComment,
            ));
        }
        if read_u16(archive, cursor + 34)? != 0 {
            return Err(PackError::UnsupportedArchiveFeature(
                ArchiveFeature::MultipleDisks,
            ));
        }
        // The high half of the external attributes carries Unix mode bits,
        // including the symbolic-link type an external extractor would honour.
        // The reviewed writer emits zero, so anything else is a refusal rather
        // than a value to interpret.
        if read_u32(archive, cursor + 38)? != 0 {
            return Err(PackError::UnsupportedArchiveFeature(
                ArchiveFeature::ExternalAttributes,
            ));
        }
        let local_offset = read_size(read_u32(archive, cursor + 42)?)?;
        let name_end = header_end.checked_add(name_length).ok_or_else(truncated)?;
        if name_end > end {
            return Err(truncated());
        }
        let name = read_name(archive, header_end, name_length)?;
        if !names.insert(name.clone()) {
            return Err(PackError::DuplicateEntry(name));
        }
        records.push(CentralRecord {
            name,
            method,
            checksum,
            compressed_length,
            expanded_length,
            local_offset,
        });
        cursor = name_end;
    }
    if cursor != end {
        return Err(PackError::MalformedArchive(
            "the central directory holds trailing bytes",
        ));
    }
    Ok(records)
}

fn read_entry_data(
    archive: &[u8],
    record: &CentralRecord,
    directory_start: usize,
    consumed: &mut usize,
) -> Result<Vec<u8>, PackError> {
    let inconsistent = || {
        PackError::MalformedArchive("a local header disagrees with its central directory record")
    };
    if record.local_offset != *consumed {
        return Err(PackError::MalformedArchive(
            "local records overlap, leave a gap, or are out of central directory order",
        ));
    }
    let header_end = record
        .local_offset
        .checked_add(LOCAL_HEADER_BYTES)
        .ok_or_else(inconsistent)?;
    if header_end > directory_start {
        return Err(inconsistent());
    }
    if read_u32(archive, record.local_offset)? != LOCAL_HEADER_SIGNATURE {
        return Err(PackError::MalformedArchive(
            "a local record is missing its signature",
        ));
    }
    check_general_purpose_flags(read_u16(archive, record.local_offset + 6)?)?;
    if read_method(read_u16(archive, record.local_offset + 8)?)? != record.method
        || read_u32(archive, record.local_offset + 14)? != record.checksum
        || read_size(read_u32(archive, record.local_offset + 18)?)? != record.compressed_length
        || read_size(read_u32(archive, record.local_offset + 22)?)? != record.expanded_length
    {
        return Err(inconsistent());
    }
    let name_length = usize::from(read_u16(archive, record.local_offset + 26)?);
    if read_u16(archive, record.local_offset + 28)? != 0 {
        return Err(PackError::UnsupportedArchiveFeature(
            ArchiveFeature::ExtraField,
        ));
    }
    if name_length != record.name.len()
        || read_name(archive, header_end, name_length)? != record.name
    {
        return Err(inconsistent());
    }
    let data_start = header_end
        .checked_add(name_length)
        .ok_or_else(inconsistent)?;
    let data_end = data_start
        .checked_add(record.compressed_length)
        .ok_or_else(inconsistent)?;
    if data_end > directory_start {
        return Err(PackError::MalformedArchive(
            "entry data runs into the central directory",
        ));
    }
    let data = archive.get(data_start..data_end).ok_or_else(inconsistent)?;
    *consumed = data_end;

    let expanded = match record.method {
        METHOD_STORE => {
            if record.compressed_length != record.expanded_length {
                return Err(PackError::MalformedArchive(
                    "a stored entry declares different compressed and expanded sizes",
                ));
            }
            data.to_vec()
        }
        _ => decompress_to_vec_with_limit(data, record.expanded_length).map_err(|_| {
            PackError::MalformedArchive(
                "a deflate stream is invalid or expands past its declared size",
            )
        })?,
    };
    if expanded.len() != record.expanded_length {
        return Err(PackError::MalformedArchive(
            "an entry expands to a different size than the central directory declares",
        ));
    }
    if crc32(&expanded) != record.checksum {
        return Err(PackError::MalformedArchive("an entry fails its checksum"));
    }
    Ok(expanded)
}

fn check_general_purpose_flags(flags: u16) -> Result<(), PackError> {
    let feature = if flags & GENERAL_PURPOSE_ENCRYPTED != 0 {
        ArchiveFeature::Encryption
    } else if flags & GENERAL_PURPOSE_DATA_DESCRIPTOR != 0 {
        ArchiveFeature::DataDescriptor
    } else if flags & GENERAL_PURPOSE_STRONG_ENCRYPTION != 0 {
        ArchiveFeature::StrongEncryption
    } else if flags & GENERAL_PURPOSE_MASKED_HEADERS != 0 {
        ArchiveFeature::MaskedLocalHeaders
    } else if flags != 0 {
        ArchiveFeature::GeneralPurposeFlags
    } else {
        return Ok(());
    };
    Err(PackError::UnsupportedArchiveFeature(feature))
}

fn read_method(method: u16) -> Result<u16, PackError> {
    if method == METHOD_STORE || method == METHOD_DEFLATE {
        Ok(method)
    } else {
        Err(PackError::UnsupportedArchiveFeature(
            ArchiveFeature::CompressionMethod(method),
        ))
    }
}

fn read_size(size: u32) -> Result<usize, PackError> {
    if size == ZIP32_SENTINEL_32 {
        return Err(PackError::UnsupportedArchiveFeature(ArchiveFeature::Zip64));
    }
    usize::try_from(size).map_err(|_| {
        PackError::MalformedArchive("an entry size exceeds this platform's addressing")
    })
}

fn read_name(archive: &[u8], offset: usize, length: usize) -> Result<String, PackError> {
    let raw = slice_at(archive, offset, length)?;
    let name = std::str::from_utf8(raw)
        .map_err(|_| PackError::IllegalPath(String::from_utf8_lossy(raw).into_owned()))?;
    validate_path(name)?;
    Ok(name.to_owned())
}

fn slice_at(archive: &[u8], offset: usize, length: usize) -> Result<&[u8], PackError> {
    let end = offset
        .checked_add(length)
        .ok_or(PackError::MalformedArchive("a record offset overflows"))?;
    archive.get(offset..end).ok_or(PackError::MalformedArchive(
        "a record extends past the end of the archive",
    ))
}

fn read_u16(archive: &[u8], offset: usize) -> Result<u16, PackError> {
    let field: [u8; 2] = slice_at(archive, offset, 2)?
        .try_into()
        .map_err(|_| PackError::MalformedArchive("a record field is truncated"))?;
    Ok(u16::from_le_bytes(field))
}

fn read_u32(archive: &[u8], offset: usize) -> Result<u32, PackError> {
    let field: [u8; 4] = slice_at(archive, offset, 4)?
        .try_into()
        .map_err(|_| PackError::MalformedArchive("a record field is truncated"))?;
    Ok(u32::from_le_bytes(field))
}

const fn crc_table() -> [u32; 256] {
    let mut table = [0_u32; 256];
    let mut index = 0_usize;
    while index < 256 {
        let mut value = index as u32;
        let mut bit = 0;
        while bit < 8 {
            value = if value & 1 == 1 {
                0xEDB8_8320 ^ (value >> 1)
            } else {
                value >> 1
            };
            bit += 1;
        }
        table[index] = value;
        index += 1;
    }
    table
}

static CRC_TABLE: [u32; 256] = crc_table();

pub(crate) fn crc32(bytes: &[u8]) -> u32 {
    let mut checksum = 0xFFFF_FFFF_u32;
    for byte in bytes {
        let index = ((checksum ^ u32::from(*byte)) & 0xFF) as usize;
        checksum = CRC_TABLE[index] ^ (checksum >> 8);
    }
    checksum ^ 0xFFFF_FFFF
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Rewrites one little-endian field inside an archive.
    pub(crate) fn patch_u16(archive: &mut [u8], offset: usize, value: u16) {
        archive[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    /// Offset of the end-of-central-directory record in an archive this crate
    /// wrote, which never carries a comment.
    pub(crate) fn eocd_offset(archive: &[u8]) -> usize {
        archive.len() - EOCD_BYTES
    }

    /// Offset of the first central directory record.
    pub(crate) fn central_offset(archive: &[u8]) -> usize {
        let eocd = eocd_offset(archive);
        read_u32(archive, eocd + 16).expect("test archive has a directory offset") as usize
    }

    /// Byte offset of the compression method field in a local record.
    const LOCAL_METHOD: usize = 8;
    /// Byte offset of the general-purpose flag field in a central record.
    pub(crate) const CENTRAL_FLAGS: usize = 8;
    /// Byte offset of the compression method field in a central record.
    pub(crate) const CENTRAL_METHOD: usize = 10;

    #[test]
    fn checksums_match_the_published_ieee_vector() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn archives_round_trip_and_are_deterministic() {
        let entries: [(&str, &[u8]); 3] = [
            ("manifest.json", b"{}"),
            ("signature.ed25519", &[7_u8; 64]),
            ("models/a.lib", b"* a model\n.model d d\n"),
        ];
        let first = write_archive(&entries).expect("writes");
        let second = write_archive(&entries).expect("writes");
        assert_eq!(first, second);
        let read = read_archive(&first, &Limits::default()).expect("reads");
        assert_eq!(read.len(), 3);
        assert_eq!(read[0].name, "manifest.json");
        assert_eq!(read[2].data, b"* a model\n.model d d\n");
    }

    #[test]
    fn stored_entries_are_accepted() {
        let payload = b"stored bytes".as_slice();
        let archive = build_stored_archive("models/a.lib", payload);
        let read = read_archive(&archive, &Limits::default()).expect("reads");
        assert_eq!(read.len(), 1);
        assert_eq!(read[0].name, "models/a.lib");
        assert_eq!(read[0].data, payload);
    }

    /// Hand-builds a one-entry archive with compression method 0.
    pub(crate) fn build_stored_archive(name: &str, data: &[u8]) -> Vec<u8> {
        let checksum = crc32(data);
        let length = data.len() as u32;
        let name_length = name.len() as u16;
        let mut archive = Vec::new();
        archive.extend_from_slice(&LOCAL_HEADER_SIGNATURE.to_le_bytes());
        archive.extend_from_slice(&VERSION.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&METHOD_STORE.to_le_bytes());
        archive.extend_from_slice(&DOS_TIME.to_le_bytes());
        archive.extend_from_slice(&DOS_DATE.to_le_bytes());
        archive.extend_from_slice(&checksum.to_le_bytes());
        archive.extend_from_slice(&length.to_le_bytes());
        archive.extend_from_slice(&length.to_le_bytes());
        archive.extend_from_slice(&name_length.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(name.as_bytes());
        archive.extend_from_slice(data);
        let directory_offset = archive.len() as u32;
        archive.extend_from_slice(&CENTRAL_HEADER_SIGNATURE.to_le_bytes());
        archive.extend_from_slice(&VERSION.to_le_bytes());
        archive.extend_from_slice(&VERSION.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&METHOD_STORE.to_le_bytes());
        archive.extend_from_slice(&DOS_TIME.to_le_bytes());
        archive.extend_from_slice(&DOS_DATE.to_le_bytes());
        archive.extend_from_slice(&checksum.to_le_bytes());
        archive.extend_from_slice(&length.to_le_bytes());
        archive.extend_from_slice(&length.to_le_bytes());
        archive.extend_from_slice(&name_length.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&0_u32.to_le_bytes());
        archive.extend_from_slice(&0_u32.to_le_bytes());
        archive.extend_from_slice(name.as_bytes());
        let directory_length = archive.len() as u32 - directory_offset;
        archive.extend_from_slice(&EOCD_SIGNATURE.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive.extend_from_slice(&1_u16.to_le_bytes());
        archive.extend_from_slice(&1_u16.to_le_bytes());
        archive.extend_from_slice(&directory_length.to_le_bytes());
        archive.extend_from_slice(&directory_offset.to_le_bytes());
        archive.extend_from_slice(&0_u16.to_le_bytes());
        archive
    }

    #[test]
    fn a_truncated_archive_is_refused() {
        let archive = write_archive(&[("a.lib", b"x")]).expect("writes");
        for length in [0, 1, 21, archive.len() - 1] {
            assert!(read_archive(&archive[..length], &Limits::default()).is_err());
        }
    }

    #[test]
    fn duplicate_entry_names_are_refused() {
        let archive = write_archive(&[("a.lib", b"x"), ("a.lib", b"y")]).expect("writes");
        assert!(matches!(
            read_archive(&archive, &Limits::default()),
            Err(PackError::DuplicateEntry(name)) if name == "a.lib"
        ));
    }

    #[test]
    fn the_compressed_cap_is_enforced_before_parsing() {
        let archive = write_archive(&[("a.lib", b"x")]).expect("writes");
        let limits = Limits {
            max_archive_bytes: 8,
            ..Limits::default()
        };
        assert!(matches!(
            read_archive(&archive, &limits),
            Err(PackError::ArchiveTooLarge { limit: 8, .. })
        ));
    }

    #[test]
    fn the_entry_count_cap_is_enforced() {
        let archive = write_archive(&[("a.lib", b"x"), ("b.lib", b"y")]).expect("writes");
        let limits = Limits {
            max_files: 1,
            ..Limits::default()
        };
        assert!(matches!(
            read_archive(&archive, &limits),
            Err(PackError::TooManyFiles {
                limit: 1,
                actual: 2
            })
        ));
    }

    #[test]
    fn the_expanded_cap_is_enforced_from_declared_sizes() {
        let archive = write_archive(&[("a.lib", &[b'x'; 4096])]).expect("writes");
        let limits = Limits {
            max_expanded_bytes: 16,
            ..Limits::default()
        };
        assert!(matches!(
            read_archive(&archive, &limits),
            Err(PackError::ExpandedTooLarge { limit: 16 })
        ));
    }

    #[test]
    fn the_data_descriptor_flag_is_refused() {
        let mut archive = write_archive(&[("a.lib", b"x")]).expect("writes");
        let central = central_offset(&archive);
        patch_u16(
            &mut archive,
            central + CENTRAL_FLAGS,
            GENERAL_PURPOSE_DATA_DESCRIPTOR,
        );
        assert!(matches!(
            read_archive(&archive, &Limits::default()),
            Err(PackError::UnsupportedArchiveFeature(
                ArchiveFeature::DataDescriptor
            ))
        ));
    }

    #[test]
    fn entry_encryption_is_refused() {
        let mut archive = write_archive(&[("a.lib", b"x")]).expect("writes");
        let central = central_offset(&archive);
        patch_u16(
            &mut archive,
            central + CENTRAL_FLAGS,
            GENERAL_PURPOSE_ENCRYPTED,
        );
        assert!(matches!(
            read_archive(&archive, &Limits::default()),
            Err(PackError::UnsupportedArchiveFeature(
                ArchiveFeature::Encryption
            ))
        ));
    }

    #[test]
    fn unsupported_compression_methods_are_refused() {
        let mut archive = write_archive(&[("a.lib", b"x")]).expect("writes");
        let central = central_offset(&archive);
        patch_u16(&mut archive, central + CENTRAL_METHOD, 12);
        assert!(matches!(
            read_archive(&archive, &Limits::default()),
            Err(PackError::UnsupportedArchiveFeature(
                ArchiveFeature::CompressionMethod(12)
            ))
        ));
    }

    #[test]
    fn a_local_header_that_disagrees_with_its_record_is_refused() {
        let mut archive = write_archive(&[("a.lib", b"x")]).expect("writes");
        patch_u16(&mut archive, LOCAL_METHOD, METHOD_STORE);
        assert!(matches!(
            read_archive(&archive, &Limits::default()),
            Err(PackError::MalformedArchive(_))
        ));
    }

    #[test]
    fn zip64_sentinels_are_refused() {
        let mut archive = write_archive(&[("a.lib", b"x")]).expect("writes");
        let eocd = eocd_offset(&archive);
        patch_u16(&mut archive, eocd + 10, ZIP32_SENTINEL_16);
        assert!(matches!(
            read_archive(&archive, &Limits::default()),
            Err(PackError::UnsupportedArchiveFeature(ArchiveFeature::Zip64))
        ));
    }

    #[test]
    fn archive_comments_are_refused() {
        let mut archive = write_archive(&[("a.lib", b"x")]).expect("writes");
        let eocd = eocd_offset(&archive);
        patch_u16(&mut archive, eocd + 20, 3);
        archive.extend_from_slice(b"pad");
        assert!(matches!(
            read_archive(&archive, &Limits::default()),
            Err(PackError::UnsupportedArchiveFeature(
                ArchiveFeature::ArchiveComment
            ))
        ));
    }

    #[test]
    fn unaccounted_bytes_between_records_are_refused() {
        let archive = write_archive(&[("a.lib", b"x"), ("b.lib", b"y")]).expect("writes");
        let central = central_offset(&archive);
        let mut padded = Vec::with_capacity(archive.len() + 4);
        // Push the central directory and its record offsets four bytes later,
        // leaving filler no entry accounts for.
        padded.extend_from_slice(&archive[..central]);
        padded.extend_from_slice(b"junk");
        padded.extend_from_slice(&archive[central..]);
        let eocd = padded.len() - EOCD_BYTES;
        let shifted = (central + 4) as u32;
        padded[eocd + 16..eocd + 20].copy_from_slice(&shifted.to_le_bytes());
        assert!(matches!(
            read_archive(&padded, &Limits::default()),
            Err(PackError::MalformedArchive(_))
        ));
    }

    #[test]
    fn traversal_entry_names_are_refused() {
        for name in ["../escape.lib", "a//b.lib", "a\\b.lib", "/abs.lib"] {
            assert!(matches!(
                write_archive(&[(name, b"x")]),
                Err(PackError::IllegalPath(_))
            ));
        }
    }

    #[test]
    fn a_corrupted_deflate_stream_fails_its_checksum_or_inflation() {
        let mut archive = write_archive(&[("a.lib", &[b'a'; 512])]).expect("writes");
        let payload = LOCAL_HEADER_BYTES + "a.lib".len() + 4;
        assert!(payload < central_offset(&archive));
        archive[payload] ^= 0xFF;
        assert!(matches!(
            read_archive(&archive, &Limits::default()),
            Err(PackError::MalformedArchive(_))
        ));
    }
}
