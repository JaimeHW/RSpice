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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u32_reads_big_endian_and_advances_cursor() {
        let mut cursor: &[u8] = &[0x12, 0x34, 0x56, 0x78, 0x9A];
        let value = read_u32(&mut cursor).expect("u32 read should succeed");
        assert_eq!(value, 0x12345678);
        assert_eq!(cursor, &[0x9A]);
    }

    #[test]
    fn test_read_i32_reads_negative_big_endian_value() {
        let mut cursor: &[u8] = &[0xFF, 0xFF, 0xFF, 0x80];
        let value = read_i32(&mut cursor).expect("i32 read should succeed");
        assert_eq!(value, -128);
        assert!(cursor.is_empty());
    }

    #[test]
    fn test_read_u8_padded_consumes_four_bytes() {
        let mut cursor: &[u8] = &[0xAB, 0xCD, 0xEF, 0x01, 0xFF];
        let value = read_u8_padded(&mut cursor).expect("padded u8 read should succeed");
        assert_eq!(value, 0xAB);
        assert_eq!(cursor, &[0xFF]);
    }

    #[test]
    fn test_read_f64_reads_value_and_advances_cursor() {
        let source = 3.25_f64.to_be_bytes();
        let mut payload = source.to_vec();
        payload.push(0xFE);
        let mut cursor: &[u8] = &payload;
        let value = read_f64(&mut cursor).expect("f64 read should succeed");
        assert!((value - 3.25).abs() < 1e-12);
        assert_eq!(cursor, &[0xFE]);
    }

    #[test]
    fn test_parse_string_respects_padding_alignment() {
        let mut payload = Vec::new();
        payload.extend_from_slice(&(3u32.to_be_bytes()));
        payload.extend_from_slice(b"abc");
        payload.push(0x00); // 4-byte alignment padding
        payload.extend_from_slice(&[0xAA, 0xBB]);
        let mut cursor: &[u8] = &payload;
        let parsed = parse_string(&mut cursor).expect("string parse should succeed");
        assert_eq!(parsed, "abc");
        assert_eq!(cursor, &[0xAA, 0xBB]);
    }

    #[test]
    fn test_parse_string_rejects_truncated_payload() {
        let mut cursor: &[u8] = &[0x00, 0x00, 0x00, 0x04, b'a', b'b', b'c'];
        let err = parse_string(&mut cursor).expect_err("truncated string must fail");
        assert!(err.to_string().contains("truncated"));
    }

    #[test]
    fn test_skip_opaque_scalar_consumes_one_word() {
        let mut cursor: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF, 0x11];
        skip_opaque_scalar(&mut cursor).expect("opaque scalar skip should succeed");
        assert_eq!(cursor, &[0x11]);
    }

    #[test]
    fn test_peek_u32_reads_without_advancing_cursor() {
        let data = [0x01, 0x23, 0x45, 0x67, 0x89];
        assert_eq!(peek_u32(&data[..4]), 0x01234567);
    }
}
