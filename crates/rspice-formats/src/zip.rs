//! Deterministic stored ZIP32 encoding for engineering-data packages.

pub fn deterministic_stored_zip(entries: &[(&str, &[u8])]) -> Result<Vec<u8>, String> {
    if entries.is_empty() || entries.len() > u16::MAX as usize {
        return Err("CI evidence package has an invalid entry count".to_owned());
    }
    let mut archive = Vec::new();
    let mut directory = Vec::new();
    for (name, contents) in entries {
        let name = name.as_bytes();
        let name_len = u16::try_from(name.len())
            .map_err(|_| "CI evidence package entry name is too long".to_owned())?;
        let content_len = u32::try_from(contents.len())
            .map_err(|_| "CI evidence package entry is too large".to_owned())?;
        let offset = u32::try_from(archive.len())
            .map_err(|_| "CI evidence package exceeds ZIP32 limits".to_owned())?;
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
    let directory_offset = u32::try_from(archive.len())
        .map_err(|_| "CI evidence package exceeds ZIP32 limits".to_owned())?;
    let directory_len = u32::try_from(directory.len())
        .map_err(|_| "CI evidence package exceeds ZIP32 limits".to_owned())?;
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
