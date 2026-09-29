//! Deterministic stored ZIP32 encoding for engineering-data packages.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoredZipError {
    EntryCount { count: usize },
    EntryNameLength { length: usize },
    EntryLength { length: usize },
    ArchiveLength { length: usize },
}

impl std::fmt::Display for StoredZipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::EntryCount { .. } => "CI evidence package has an invalid entry count",
            Self::EntryNameLength { .. } => "CI evidence package entry name is too long",
            Self::EntryLength { .. } => "CI evidence package entry is too large",
            Self::ArchiveLength { .. } => "CI evidence package exceeds ZIP32 limits",
        })
    }
}

impl std::error::Error for StoredZipError {}

pub fn deterministic_stored_zip(entries: &[(&str, &[u8])]) -> Result<Vec<u8>, StoredZipError> {
    if entries.is_empty() || entries.len() > u16::MAX as usize {
        return Err(StoredZipError::EntryCount {
            count: entries.len(),
        });
    }
    let mut archive = Vec::new();
    let mut directory = Vec::new();
    for (name, contents) in entries {
        let name = name.as_bytes();
        let name_len = u16::try_from(name.len())
            .map_err(|_| StoredZipError::EntryNameLength { length: name.len() })?;
        let content_len =
            u32::try_from(contents.len()).map_err(|_| StoredZipError::EntryLength {
                length: contents.len(),
            })?;
        let offset = u32::try_from(archive.len()).map_err(|_| StoredZipError::ArchiveLength {
            length: archive.len(),
        })?;
        let crc = crc32(contents);

        push_u32(&mut archive, 0x0403_4b50);
        push_u16(&mut archive, 20);
        push_u16(&mut archive, 0x0800);
        push_u16(&mut archive, 0);
        push_u16(&mut archive, 0);
        push_u16(&mut archive, 0x0021);
        push_u32(&mut archive, crc);
        push_u32(&mut archive, content_len);
        push_u32(&mut archive, content_len);
        push_u16(&mut archive, name_len);
        push_u16(&mut archive, 0);
        archive.extend_from_slice(name);
        archive.extend_from_slice(contents);

        push_u32(&mut directory, 0x0201_4b50);
        push_u16(&mut directory, 20);
        push_u16(&mut directory, 20);
        push_u16(&mut directory, 0x0800);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0x0021);
        push_u32(&mut directory, crc);
        push_u32(&mut directory, content_len);
        push_u32(&mut directory, content_len);
        push_u16(&mut directory, name_len);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0);
        push_u16(&mut directory, 0);
        push_u32(&mut directory, 0);
        push_u32(&mut directory, offset);
        directory.extend_from_slice(name);
    }
    let directory_offset =
        u32::try_from(archive.len()).map_err(|_| StoredZipError::ArchiveLength {
            length: archive.len(),
        })?;
    let directory_len =
        u32::try_from(directory.len()).map_err(|_| StoredZipError::ArchiveLength {
            length: directory.len(),
        })?;
    archive.extend_from_slice(&directory);
    push_u32(&mut archive, 0x0605_4b50);
    push_u16(&mut archive, 0);
    push_u16(&mut archive, 0);
    push_u16(&mut archive, entries.len() as u16);
    push_u16(&mut archive, entries.len() as u16);
    push_u32(&mut archive, directory_len);
    push_u32(&mut archive, directory_offset);
    push_u16(&mut archive, 0);
    Ok(archive)
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & (0_u32.wrapping_sub(crc & 1)));
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zip32_refusals_retain_the_failed_count_and_name_length() {
        let error = deterministic_stored_zip(&[]).unwrap_err();
        assert_eq!(error, StoredZipError::EntryCount { count: 0 });
        assert_eq!(
            error.to_string(),
            "CI evidence package has an invalid entry count"
        );
        let name = "x".repeat(usize::from(u16::MAX) + 1);
        let error = deterministic_stored_zip(&[(&name, &[])]).unwrap_err();
        assert_eq!(
            error,
            StoredZipError::EntryNameLength { length: name.len() }
        );
        assert_eq!(
            error.to_string(),
            "CI evidence package entry name is too long"
        );
    }
}
