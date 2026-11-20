//! The structural walk a MATLAB v5 file passes before the parser reads it.
//!
//! The parser inflates a compressed variable to the end of its stream with no
//! bound, multiplies a matrix's dimensions in `i32` unchecked, and stops without
//! a word at the first variable it cannot read whole. This walk refuses each of
//! those first, and allocates nothing in proportion to any size a file declares:
//!
//! - every element, and every part of a matrix, fits inside what holds it, so a
//!   file cut short is refused rather than read without its last variable;
//! - a compressed variable's declared size counts against the import's
//!   expanded-byte limit before anything past its tag is inflated, and its
//!   stream must expand to exactly that size;
//! - a numeric matrix's dimensions are not negative and multiply without
//!   overflowing `i32`, the type the parser multiplies them in.

use super::MatlabReadFailure;
use std::io::Read;

const MI_INT32: u32 = 5;
const MI_MATRIX: u32 = 14;
const MI_COMPRESSED: u32 = 15;
/// Array classes from 6 (double) on are full numeric arrays, the only ones whose
/// dimensions the parser multiplies. Cell, struct, object, char and sparse
/// arrays (1 to 5) are skipped, so a large sparse matrix is never refused.
const FIRST_FULL_NUMERIC_CLASS: u32 = 6;

#[derive(Clone, Copy)]
enum Endian {
    Little,
    Big,
}

impl Endian {
    fn u32(self, bytes: [u8; 4]) -> u32 {
        match self {
            Self::Little => u32::from_le_bytes(bytes),
            Self::Big => u32::from_be_bytes(bytes),
        }
    }

    fn i32(self, bytes: [u8; 4]) -> i32 {
        match self {
            Self::Little => i32::from_le_bytes(bytes),
            Self::Big => i32::from_be_bytes(bytes),
        }
    }
}

/// One element tag in either of the format's two layouts.
struct Tag {
    data_type: u32,
    /// Bytes the tag itself occupies: 8 in the long layout, 4 in the small one,
    /// whose data sits in the tag's second word.
    tag_bytes: u64,
    data_bytes: u64,
    padding: u64,
}

impl Tag {
    fn read(endian: Endian, source: &mut impl Read) -> Result<Self, MatlabReadFailure> {
        let first = endian.u32(read_word(source)?);
        if first & 0xFFFF_0000 == 0 {
            let data_bytes = u64::from(endian.u32(read_word(source)?));
            Ok(Self {
                data_type: first,
                tag_bytes: 8,
                data_bytes,
                padding: data_bytes.next_multiple_of(8) - data_bytes,
            })
        } else {
            let data_bytes = u64::from(first >> 16);
            if data_bytes > 4 {
                return Err(MatlabReadFailure::Malformed);
            }
            Ok(Self {
                data_type: first & 0xFFFF,
                tag_bytes: 4,
                data_bytes,
                padding: 4 - data_bytes,
            })
        }
    }

    /// The whole element: tag, data and the padding that aligns the next one.
    fn padded_bytes(&self) -> u64 {
        self.tag_bytes + self.data_bytes + self.padding
    }
}

// Only a compressed stream can end before a read is done: every other source
// is a slice whose size was checked before it was read.
fn read_word(source: &mut impl Read) -> Result<[u8; 4], MatlabReadFailure> {
    let mut word = [0; 4];
    source
        .read_exact(&mut word)
        .map_err(|_| MatlabReadFailure::CompressedSize)?;
    Ok(word)
}

fn skip(source: &mut impl Read, bytes: u64) -> Result<(), MatlabReadFailure> {
    let skipped = std::io::copy(&mut source.take(bytes), &mut std::io::sink())
        .map_err(|_| MatlabReadFailure::CompressedSize)?;
    if skipped == bytes {
        Ok(())
    } else {
        Err(MatlabReadFailure::CompressedSize)
    }
}

/// Walk the file's elements; `max_expanded_bytes` bounds what its compressed
/// variables may expand to, together.
pub(super) fn check(bytes: &[u8], max_expanded_bytes: u64) -> Result<(), MatlabReadFailure> {
    let header = crate::matlab::HEADER_BYTES;
    // A header the parser cannot read is its own error to report.
    let endian = match bytes.get(header - 2..header) {
        Some(b"IM") => Endian::Little,
        Some(b"MI") => Endian::Big,
        _ => return Ok(()),
    };
    let mut expanded = 0_u64;
    let mut rest = &bytes[header..];
    let mut variable = 0_usize;
    while !rest.is_empty() {
        variable += 1;
        let available = rest.len() as u64;
        // Either tag layout takes eight bytes with its data word.
        if available < 8 {
            return Err(truncated(variable));
        }
        let mut cursor = rest;
        let tag = Tag::read(endian, &mut cursor)?;
        let data_end = tag.tag_bytes + tag.data_bytes;
        if data_end > available {
            return Err(truncated(variable));
        }
        let payload = &cursor[..tag.data_bytes as usize];
        let padding = match tag.data_type {
            MI_MATRIX => {
                check_matrix(endian, &mut &payload[..], tag.data_bytes)?;
                tag.padding
            }
            // Compressed variables are written unaligned.
            MI_COMPRESSED => {
                check_compressed(endian, payload, &mut expanded, max_expanded_bytes)?;
                0
            }
            _ => tag.padding,
        };
        // Padding the file ends inside holds no data; the parser stops there.
        rest = &rest[(data_end + padding).min(available) as usize..];
    }
    Ok(())
}

fn truncated(variable: usize) -> MatlabReadFailure {
    MatlabReadFailure::Truncated { variable }
}

/// Walk the parts of one matrix of `bytes` bytes. Each part must fit inside the
/// matrix; a numeric matrix's dimensions must multiply as the parser will.
fn check_matrix(
    endian: Endian,
    source: &mut impl Read,
    bytes: u64,
) -> Result<(), MatlabReadFailure> {
    let mut consumed = 0_u64;
    let mut class = None;
    let mut part = 0_usize;
    // A part needs at least one tag word and its data word; a shorter tail is
    // slack the parser never reads.
    while bytes - consumed >= 8 {
        part += 1;
        let tag = Tag::read(endian, source)?;
        if tag.padded_bytes() > bytes - consumed {
            return Err(MatlabReadFailure::Malformed);
        }
        let numeric = class.is_some_and(|class| class >= FIRST_FULL_NUMERIC_CLASS);
        match part {
            1 if tag.data_bytes == 8 => {
                let flags = endian.u32(read_word(source)?);
                class = Some(flags & 0xFF);
                skip(source, 4 + tag.padding)?;
            }
            2 if numeric && tag.data_type == MI_INT32 && tag.data_bytes % 4 == 0 => {
                check_dimensions(endian, source, tag.data_bytes / 4)?;
                skip(source, tag.padding)?;
            }
            _ => skip(source, tag.data_bytes + tag.padding)?,
        }
        consumed += tag.padded_bytes();
    }
    skip(source, bytes - consumed)
}

fn check_dimensions(
    endian: Endian,
    source: &mut impl Read,
    count: u64,
) -> Result<(), MatlabReadFailure> {
    let mut product = Some(1_i32);
    let mut dimensions = Vec::new();
    for _ in 0..count {
        let dimension = endian.i32(read_word(source)?);
        if dimension < 0 {
            return Err(MatlabReadFailure::NegativeDimension { dimension });
        }
        // Kept for the message only while it stays short.
        if dimensions.len() < 8 {
            dimensions.push(dimension);
        }
        product = product.and_then(|product| product.checked_mul(dimension));
    }
    if product.is_none() {
        return Err(MatlabReadFailure::DimensionOverflow { dimensions });
    }
    Ok(())
}

/// Check one compressed variable. Its declared size is charged to `expanded`
/// and refused past `limit` before anything after its tag is inflated; the
/// stream must then expand to exactly that size.
fn check_compressed(
    endian: Endian,
    payload: &[u8],
    expanded: &mut u64,
    limit: u64,
) -> Result<(), MatlabReadFailure> {
    let mut stream = flate2::read::ZlibDecoder::new(payload);
    let tag = Tag::read(endian, &mut stream)?;
    let declared = tag.tag_bytes + tag.data_bytes;
    *expanded = expanded.saturating_add(declared);
    if *expanded > limit {
        return Err(MatlabReadFailure::ExpandedByteLimit {
            expanded: *expanded,
            limit,
        });
    }
    let mut inner = (&mut stream).take(tag.data_bytes);
    if tag.data_type == MI_MATRIX {
        check_matrix(endian, &mut inner, tag.data_bytes)?;
    } else {
        skip(&mut inner, tag.data_bytes)?;
    }
    // The parser keeps everything the stream holds, so a stream that runs on
    // past its declared element and that element's alignment is the bomb this
    // walk exists to refuse.
    let trailing = std::io::copy(&mut stream.take(8), &mut std::io::sink())
        .map_err(|_| MatlabReadFailure::CompressedSize)?;
    if trailing == 8 {
        return Err(MatlabReadFailure::CompressedSize);
    }
    Ok(())
}
