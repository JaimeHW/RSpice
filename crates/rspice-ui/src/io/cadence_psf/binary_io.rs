use super::CadencePsfError;

pub(super) fn skip_opaque_scalar(cursor: &mut &[u8]) -> Result<(), CadencePsfError> {
    // Unknown scalar kinds in some PSF variants are commonly encoded as aligned
    // 32-bit words. Consume one word so known signals in the same dataset can
    // still be decoded.
    let _ = read_u32(cursor)?;
    Ok(())
}

pub(super) fn parse_string(cursor: &mut &[u8]) -> Result<String, CadencePsfError> {
    let len = read_u32(cursor)? as usize;
    if cursor.len() < len {
        return Err(CadencePsfError::new("string block truncated"));
    }
    let raw = &cursor[..len];
    let value = std::str::from_utf8(raw)
        .map_err(|e| CadencePsfError::new(format!("invalid UTF-8 in PSF string: {}", e)))?
        .to_string();

    let pad = (4 - (len % 4)) % 4;
    if cursor.len() < len + pad {
        return Err(CadencePsfError::new(
            "string padding exceeds remaining bytes",
        ));
    }
    *cursor = &cursor[len + pad..];
    Ok(value)
}

pub(super) fn read_u32(cursor: &mut &[u8]) -> Result<u32, CadencePsfError> {
    if cursor.len() < 4 {
        return Err(CadencePsfError::new(
            "unexpected end of PSF data while reading u32",
        ));
    }
    let (head, tail) = cursor.split_at(4);
    *cursor = tail;
    Ok(u32::from_be_bytes([head[0], head[1], head[2], head[3]]))
}

pub(super) fn read_i32(cursor: &mut &[u8]) -> Result<i32, CadencePsfError> {
    if cursor.len() < 4 {
        return Err(CadencePsfError::new(
            "unexpected end of PSF data while reading i32",
        ));
    }
    let (head, tail) = cursor.split_at(4);
    *cursor = tail;
    Ok(i32::from_be_bytes([head[0], head[1], head[2], head[3]]))
}

pub(super) fn read_u8_padded(cursor: &mut &[u8]) -> Result<u8, CadencePsfError> {
    if cursor.len() < 4 {
        return Err(CadencePsfError::new(
            "unexpected end of PSF data while reading padded u8",
        ));
    }
    let (head, tail) = cursor.split_at(4);
    *cursor = tail;
    Ok(head[0])
}

pub(super) fn read_f64(cursor: &mut &[u8]) -> Result<f64, CadencePsfError> {
    if cursor.len() < 8 {
        return Err(CadencePsfError::new(
            "unexpected end of PSF data while reading f64",
        ));
    }
    let (head, tail) = cursor.split_at(8);
    *cursor = tail;
    Ok(f64::from_be_bytes([
        head[0], head[1], head[2], head[3], head[4], head[5], head[6], head[7],
    ]))
}

pub(super) fn peek_u32(data: &[u8]) -> u32 {
    u32::from_be_bytes([data[0], data[1], data[2], data[3]])
}
