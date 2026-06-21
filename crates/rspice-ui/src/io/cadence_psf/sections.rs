use std::collections::HashMap;

use super::binary_io::{parse_string, peek_u32, read_f64, read_i32, read_u32};
use super::toc::TocEntry;
use super::types::{
    CadencePsfError, CadencePsfValue, DataType, SignalRef, TraceDef, TypeDecl, TypeKind,
};

pub(super) fn parse_header(
    file: &[u8],
    entry: TocEntry,
) -> Result<HashMap<String, CadencePsfValue>, CadencePsfError> {
    if entry.start + 8 > file.len() {
        return Err(CadencePsfError::new("header section truncated"));
    }

    let end_of_section = peek_u32(&file[entry.start + 4..entry.start + 8]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 8 {
        return Err(CadencePsfError::new("invalid header section end offset"));
    }

    let mut cursor = &file[entry.start + 8..end_of_section];
    let mut header = HashMap::new();
    while cursor.len() > 4 {
        if consume_zero_word_padding(&mut cursor) {
            break;
        }
        let (name, value) = parse_named_value(&mut cursor)?;
        header.insert(name, value);
    }
    reject_non_zero_trailing_bytes("header", cursor, &[])?;
    Ok(header)
}

pub(super) fn parse_types(
    file: &[u8],
    entry: TocEntry,
) -> Result<HashMap<u32, TypeDecl>, CadencePsfError> {
    if entry.start + 16 > file.len() {
        return Err(CadencePsfError::new("type section truncated"));
    }
    let block_type = peek_u32(&file[entry.start + 8..entry.start + 12]);
    if block_type != 22 {
        return Err(CadencePsfError::new(format!(
            "type section expected block 22, got {}",
            block_type
        )));
    }
    let end_of_section = peek_u32(&file[entry.start + 12..entry.start + 16]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 16 {
        return Err(CadencePsfError::new("invalid type section end offset"));
    }

    let mut cursor = &file[entry.start + 16..end_of_section];
    let mut types = HashMap::new();
    while cursor.len() > 4 {
        if consume_zero_word_padding(&mut cursor) {
            break;
        }
        parse_type_decl(&mut cursor, &mut types)?;
    }
    // Type payloads may end with a trailing struct-terminator word.
    reject_non_zero_trailing_bytes("type", cursor, &[18])?;

    Ok(types)
}

pub(super) fn parse_type_decl(
    cursor: &mut &[u8],
    types: &mut HashMap<u32, TypeDecl>,
) -> Result<u32, CadencePsfError> {
    let block = read_u32(cursor)?;
    if block != 16 {
        return Err(CadencePsfError::new(format!(
            "type item expected block 16, got {}",
            block
        )));
    }

    let type_id = read_u32(cursor)?;
    let name = parse_string(cursor)?;
    let array_type_raw = read_u32(cursor)?;
    let data_type = DataType::from_u32(read_u32(cursor)?);
    let kind = match data_type {
        DataType::Struct => {
            let mut members = Vec::new();
            while cursor.len() > 4 {
                if peek_u32(cursor) == 18 {
                    let _ = read_u32(cursor)?;
                    break;
                }
                let member_id = parse_type_decl(cursor, types)?;
                members.push(member_id);
            }
            TypeKind::Struct { members }
        }
        DataType::Array => TypeKind::Array {
            element_type_raw: array_type_raw,
        },
        other => TypeKind::Primitive(other),
    };

    skip_properties(cursor)?;
    types.insert(type_id, TypeDecl { name, kind });
    Ok(type_id)
}

pub(super) fn parse_sweeps(
    file: &[u8],
    entry: TocEntry,
) -> Result<Vec<SignalRef>, CadencePsfError> {
    if entry.start + 8 > file.len() {
        return Err(CadencePsfError::new("sweep section truncated"));
    }
    let end_of_section = peek_u32(&file[entry.start + 4..entry.start + 8]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 8 {
        return Err(CadencePsfError::new("invalid sweep section end offset"));
    }

    let mut cursor = &file[entry.start + 8..end_of_section];
    let mut sweeps = Vec::new();
    while cursor.len() > 4 {
        if consume_zero_word_padding(&mut cursor) {
            break;
        }
        let block = read_u32(&mut cursor)?;
        if block != 16 {
            return Err(CadencePsfError::new(format!(
                "sweep signal expected block 16, got {}",
                block
            )));
        }
        sweeps.push(parse_signal_ref(&mut cursor)?);
    }
    reject_non_zero_trailing_bytes("sweep", cursor, &[])?;
    Ok(sweeps)
}

pub(super) fn parse_traces(file: &[u8], entry: TocEntry) -> Result<Vec<TraceDef>, CadencePsfError> {
    if entry.start + 16 > file.len() {
        return Err(CadencePsfError::new("trace section truncated"));
    }
    let block_type = peek_u32(&file[entry.start + 8..entry.start + 12]);
    if block_type != 22 {
        return Err(CadencePsfError::new(format!(
            "trace section expected block 22, got {}",
            block_type
        )));
    }
    let end_of_section = peek_u32(&file[entry.start + 12..entry.start + 16]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 16 {
        return Err(CadencePsfError::new("invalid trace section end offset"));
    }

    let mut cursor = &file[entry.start + 16..end_of_section];
    let mut traces = Vec::new();
    while cursor.len() > 4 {
        if consume_zero_word_padding(&mut cursor) {
            break;
        }
        let block = read_u32(&mut cursor)?;
        match block {
            16 => traces.push(TraceDef::Signal(parse_signal_ref(&mut cursor)?)),
            17 => traces.push(TraceDef::Group(parse_group_signals(&mut cursor)?)),
            other => {
                return Err(CadencePsfError::new(format!(
                    "unexpected trace block type {}",
                    other
                )));
            }
        }
    }
    reject_non_zero_trailing_bytes("trace", cursor, &[])?;

    Ok(traces)
}
pub(super) fn parse_group_signals(cursor: &mut &[u8]) -> Result<Vec<SignalRef>, CadencePsfError> {
    let _group_id = read_u32(cursor)?;
    let _group_name = parse_string(cursor)?;
    let count = read_u32(cursor)? as usize;
    validate_declared_count_fits_remaining("trace group", count, 16, cursor.len())?;

    let mut signals = Vec::new();
    signals.try_reserve(count).map_err(|_| {
        CadencePsfError::new(format!(
            "trace group declares too many signals ({count}) to allocate"
        ))
    })?;
    for _ in 0..count {
        let block = read_u32(cursor)?;
        if block != 16 {
            return Err(CadencePsfError::new(format!(
                "trace group expected signal block 16, got {}",
                block
            )));
        }
        signals.push(parse_signal_ref(cursor)?);
    }
    Ok(signals)
}

pub(super) fn parse_signal_ref(cursor: &mut &[u8]) -> Result<SignalRef, CadencePsfError> {
    let id = read_u32(cursor)?;
    let name = parse_string(cursor)?;
    let type_id = read_u32(cursor)?;
    skip_properties(cursor)?;
    Ok(SignalRef { id, name, type_id })
}

pub(super) fn skip_properties(cursor: &mut &[u8]) -> Result<(), CadencePsfError> {
    while cursor.len() > 4 {
        let block = peek_u32(cursor);
        if !(33..=35).contains(&block) {
            break;
        }
        let _ = parse_named_value(cursor)?;
    }
    Ok(())
}

pub(super) fn parse_named_value(
    cursor: &mut &[u8],
) -> Result<(String, CadencePsfValue), CadencePsfError> {
    let block = read_u32(cursor)?;
    let name = parse_string(cursor)?;
    let value = match block {
        33 => CadencePsfValue::Text(parse_string(cursor)?),
        34 => CadencePsfValue::Int(read_i32(cursor)? as i64),
        35 => CadencePsfValue::Real(read_f64(cursor)?),
        other => {
            return Err(CadencePsfError::new(format!(
                "unexpected named-value block {}",
                other
            )));
        }
    };
    Ok((name, value))
}

pub(super) fn flatten_traces(traces: &[TraceDef]) -> Vec<SignalRef> {
    let mut flat = Vec::new();
    for trace in traces {
        match trace {
            TraceDef::Signal(signal) => flat.push(signal.clone()),
            TraceDef::Group(signals) => flat.extend(signals.iter().cloned()),
        }
    }
    flat
}

pub(super) fn header_usize(
    header: &HashMap<String, CadencePsfValue>,
    key: &str,
) -> Result<usize, CadencePsfError> {
    match header.get(key) {
        Some(CadencePsfValue::Int(v)) => usize::try_from(*v).map_err(|_| {
            CadencePsfError::new(format!(
                "header value '{}' must be a non-negative integer count",
                key
            ))
        }),
        Some(CadencePsfValue::Real(v)) => {
            if !v.is_finite() {
                return Err(CadencePsfError::new(format!(
                    "header value '{}' must be finite",
                    key
                )));
            }
            if *v < 0.0 {
                return Err(CadencePsfError::new(format!(
                    "header value '{}' must be a non-negative integer count",
                    key
                )));
            }
            if v.fract() != 0.0 {
                return Err(CadencePsfError::new(format!(
                    "header value '{}' must be an integer count",
                    key
                )));
            }
            if *v > usize::MAX as f64 {
                return Err(CadencePsfError::new(format!(
                    "header value '{}' exceeds supported range",
                    key
                )));
            }
            Ok(*v as usize)
        }
        Some(_) => Err(CadencePsfError::new(format!(
            "header value '{}' has non-numeric type",
            key
        ))),
        None => Err(CadencePsfError::new(format!(
            "missing required header key '{}'",
            key
        ))),
    }
}

pub(super) fn parse_zero_pad(mut cursor: &[u8]) -> Result<&[u8], CadencePsfError> {
    let len = read_u32(&mut cursor)? as usize;
    if cursor.len() < len {
        return Err(CadencePsfError::new("zero-pad block truncated"));
    }
    let (pad, tail) = cursor.split_at(len);
    if pad.iter().any(|byte| *byte != 0) {
        return Err(CadencePsfError::new(
            "zero-pad block contains non-zero bytes",
        ));
    }
    Ok(tail)
}

pub(super) fn consume_zero_word_padding(cursor: &mut &[u8]) -> bool {
    if cursor.len() < 4 || peek_u32(cursor) != 0 {
        return false;
    }
    if cursor[4..].iter().all(|byte| *byte == 0) {
        *cursor = &[];
        return true;
    }
    false
}

pub(super) fn reject_non_zero_trailing_bytes(
    section_name: &str,
    trailing: &[u8],
    allowed_words: &[u32],
) -> Result<(), CadencePsfError> {
    if trailing.is_empty() || trailing.iter().all(|byte| *byte == 0) {
        return Ok(());
    }
    if trailing.len() == 4 && allowed_words.contains(&peek_u32(trailing)) {
        return Ok(());
    }

    Err(CadencePsfError::new(format!(
        "{} section has unexpected non-zero trailing bytes ({})",
        section_name,
        trailing.len()
    )))
}

pub(super) fn validate_declared_count_fits_remaining(
    label: &str,
    declared: usize,
    min_bytes_per_item: usize,
    remaining_bytes: usize,
) -> Result<(), CadencePsfError> {
    if min_bytes_per_item == 0 {
        return Ok(());
    }
    let max_from_remaining = remaining_bytes / min_bytes_per_item;
    if declared > max_from_remaining {
        return Err(CadencePsfError::new(format!(
            "{label} declares {declared} item(s), but remaining payload ({remaining_bytes} bytes) can hold at most {max_from_remaining}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_u32(bytes: &mut Vec<u8>, value: u32) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn push_string(bytes: &mut Vec<u8>, value: &str) {
        push_u32(bytes, value.len() as u32);
        bytes.extend_from_slice(value.as_bytes());
        let pad = (4 - (value.len() % 4)) % 4;
        bytes.extend(std::iter::repeat_n(0, pad));
    }

    #[test]
    fn trace_group_count_is_bounded_by_remaining_bytes() {
        let mut bytes = Vec::new();
        push_u32(&mut bytes, 7);
        push_string(&mut bytes, "group");
        push_u32(&mut bytes, 2);
        let mut cursor = bytes.as_slice();

        let err =
            parse_group_signals(&mut cursor).expect_err("oversized trace group must be rejected");

        let message = err.to_string();
        assert!(
            message.contains("trace group")
                && message.contains("declares 2")
                && message.contains("remaining"),
            "unexpected error: {message}"
        );
    }
}
