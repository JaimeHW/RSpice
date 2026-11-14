//! The text of a plot's header, in the encoding its writer chose.
//!
//! ngspice, Xyce and RSpice write the header as ASCII. LTspice XVII and
//! LTspice 24 write it as UTF-16LE, usually without a byte-order mark, and an
//! ASCII `Values:` section after such a header is UTF-16LE too. A binary
//! section starts at the byte right after the `Binary:` line's terminator in
//! either encoding: one byte off there and every value is wrong.

use std::borrow::Cow;

use super::RawParseError;

/// How a plot's text is encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TextEncoding {
    /// ASCII or UTF-8, with or without a byte-order mark.
    Utf8,
    /// UTF-16 little-endian, with or without a byte-order mark: LTspice.
    Utf16Le,
}

/// A plot's header decoded to text, and where its data section starts.
#[derive(Debug)]
pub(super) struct HeaderText {
    /// One line per header line, each ending in `\n`; the last is the
    /// `Binary:` or `Values:` line.
    pub(super) text: String,
    /// Byte offset of the data section in the plot's bytes.
    pub(super) data_offset: usize,
    pub(super) encoding: TextEncoding,
}

const UTF16LE_BOM: [u8; 2] = [0xFF, 0xFE];
const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Whether `bytes` start as UTF-16LE text: a byte-order mark, or an ASCII
/// character whose high byte is zero. No ASCII header holds a NUL byte.
pub(super) fn looks_utf16le(bytes: &[u8]) -> bool {
    bytes.starts_with(&UTF16LE_BOM) || matches!(bytes, [first, 0, ..] if *first != 0)
}

/// Decode the header that opens `bytes`, through its `Binary:` or `Values:`
/// line, in the encoding its first bytes declare.
pub(super) fn decode_header(bytes: &[u8]) -> Result<HeaderText, RawParseError> {
    let (encoding, mut offset) = if bytes.starts_with(&UTF16LE_BOM) {
        (TextEncoding::Utf16Le, UTF16LE_BOM.len())
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        return Err(RawParseError::UnsupportedFormat(
            "the header is UTF-16 big-endian; rawfile headers are ASCII, UTF-8 or \
             UTF-16 little-endian (LTspice)"
                .to_string(),
        ));
    } else if bytes.starts_with(&UTF8_BOM) {
        (TextEncoding::Utf8, UTF8_BOM.len())
    } else if looks_utf16le(bytes) {
        (TextEncoding::Utf16Le, 0)
    } else {
        (TextEncoding::Utf8, 0)
    };
    let mut text = String::new();
    let mut line_number = 0usize;
    while offset < bytes.len() {
        let rest = bytes.get(offset..).unwrap_or_default();
        line_number += 1;
        let (line, consumed) = match encoding {
            TextEncoding::Utf8 => utf8_line(rest, line_number)?,
            TextEncoding::Utf16Le => utf16le_line(rest, line_number)?,
        };
        offset += consumed;
        let line = line.trim_end_matches(['\n', '\r']);
        text.try_reserve(line.len().saturating_add(1))
            .map_err(|error| {
                RawParseError::DataError(format!("unable to retain the raw header: {error}"))
            })?;
        text.push_str(line);
        text.push('\n');
        if matches!(line.trim(), "Binary:" | "Values:") {
            return Ok(HeaderText {
                text,
                data_offset: offset,
                encoding,
            });
        }
    }
    Err(RawParseError::InvalidHeader(format!(
        "the header ends after line {line_number} without a 'Binary:' or 'Values:' line, \
         so the file has no data section"
    )))
}

/// The UTF-8 line that opens `rest`, with its terminator, and its length.
fn utf8_line(rest: &[u8], line_number: usize) -> Result<(Cow<'_, str>, usize), RawParseError> {
    let end = rest
        .iter()
        .position(|byte| *byte == b'\n')
        .map_or(rest.len(), |index| index + 1);
    let line = std::str::from_utf8(rest.get(..end).unwrap_or(rest)).map_err(|error| {
        RawParseError::InvalidHeader(format!(
            "line {line_number} is not UTF-8 text ({error}); a rawfile header is ASCII, \
             UTF-8 or UTF-16 little-endian"
        ))
    })?;
    Ok((Cow::Borrowed(line), end))
}

/// The UTF-16LE line that opens `rest`, with its terminator, and its length.
fn utf16le_line(rest: &[u8], line_number: usize) -> Result<(Cow<'_, str>, usize), RawParseError> {
    let mut units = Vec::new();
    for pair in rest.chunks_exact(2) {
        let unit = u16::from_le_bytes([pair[0], pair[1]]);
        units.push(unit);
        if unit == u16::from(b'\n') {
            break;
        }
    }
    let consumed = units.len() * 2;
    if units.last() != Some(&u16::from(b'\n')) && consumed != rest.len() {
        return Err(RawParseError::InvalidHeader(format!(
            "line {line_number} ends in half a UTF-16 character; the file is truncated"
        )));
    }
    let line = String::from_utf16(&units).map_err(|_| {
        RawParseError::InvalidHeader(format!(
            "line {line_number} is not UTF-16 text: it holds an unpaired surrogate"
        ))
    })?;
    Ok((Cow::Owned(line), consumed))
}

/// The ASCII text a UTF-16LE run spells, up to its first character outside
/// ASCII. A `Values:` section is ASCII, so this is all of it that matters.
pub(super) fn utf16le_ascii(bytes: &[u8], max_units: usize) -> Vec<u8> {
    bytes
        .chunks_exact(2)
        .take(max_units)
        .map_while(|pair| (pair[1] == 0 && pair[0] != 0 && pair[0].is_ascii()).then_some(pair[0]))
        .collect()
}

/// Whether `bytes` are nothing but UTF-16LE whitespace.
pub(super) fn is_utf16le_blank(bytes: &[u8]) -> bool {
    bytes.len().is_multiple_of(2)
        && bytes
            .chunks_exact(2)
            .all(|pair| pair[1] == 0 && pair[0].is_ascii_whitespace())
}
